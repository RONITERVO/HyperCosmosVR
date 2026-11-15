//! Measure energy and LOD continuity in the shipping photosphere shader.
//! This deliberately does not reimplement its noise, granules or calibration.
use glam::Vec3;
use wgpu::util::DeviceExt;

const SAMPLE_COUNT: usize = 16_384;
const PHYSICAL_TIMES: [f32; 3] = [0.0, 373.0, 2000.0];
const FEATURES: [u32; 2] = [255, 1];
const FOOTPRINTS: [f32; 15] = [
    0.0, 0.06, 0.1, 0.18, 0.24, 0.4, 0.6, 0.9, 1.15, 0.059, 0.061, 0.149, 0.151, 0.239, 0.241,
];

fn quiet_directions() -> Vec<[f32; 4]> {
    // Uniform spherical sampling, independent of the shader's lattice/hash.
    // Exclude a generous26-degree cap around the shared active-region frame:
    // its spots and faculae are intentional spatial contrast, not quiet-Sun LOD.
    let latitude = 20.0_f64.to_radians();
    let region = [0.0, latitude.sin(), latitude.cos()];
    let mut seed = 0x5739_481d_u32;
    let mut random = || {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (f64::from(seed) + 0.5) / 4_294_967_296.0
    };
    let mut directions = Vec::with_capacity(SAMPLE_COUNT);
    while directions.len() < SAMPLE_COUNT {
        let y = random() * 2.0 - 1.0;
        let azimuth = random() * std::f64::consts::TAU;
        let radius = (1.0 - y * y).sqrt();
        let n = [radius * azimuth.cos(), y, radius * azimuth.sin()];
        if n[1] * region[1] + n[2] * region[2] > 0.9 {
            continue;
        }
        directions.push([n[0] as f32, n[1] as f32, n[2] as f32, 0.0]);
    }
    directions
}

fn probe_surface(directions: &[[f32; 4]], cases: &[[f32; 4]]) -> Vec<[f32; 4]> {
    let source = concat!(
        include_str!("../../../shaders/common_oled.wgsl"),
        "\n",
        include_str!("../../../shaders/doc_common.wgsl"),
        "\n",
        include_str!("../../../shaders/hero_doc_sun.wgsl"),
        r#"
@group(2) @binding(0) var<storage,read> directions:array<vec4<f32>>;
@group(2) @binding(1) var<storage,read> cases:array<vec4<f32>>;
@group(2) @binding(2) var<storage,read_write> results:array<vec4<f32>>;
@compute @workgroup_size(64)
fn probe_surface(@builtin(global_invocation_id) id:vec3<u32>) {
    if (id.x>=arrayLength(&directions) || id.y>=arrayLength(&cases)) { return; }
    let n=normalize(directions[id.x].xyz);
    let params=cases[id.y];
    var inst:HeroVolumeInstance;
    inst.kind_tags=vec4<u32>(3u,1400u,u32(params.z)<<16u,0u);
    let footprint=params.y*900000.0/DOC_SUN_RADIUS_M;
    // n is also the viewing direction, fixing mu=1 and isolating surface LOD.
    // The requested times are physical reference seconds, not application time.
    let radiance=shade_doc_sun_surface(n*0.84,n,n,inst,
        params.x/DOC_SUN_TIME_SCALE,footprint);
    results[id.y*arrayLength(&directions)+id.x]=vec4<f32>(radiance,1.0);
}
"#
    );
    run_surface_probe(source, "probe_surface", directions, cases, 1)
}

fn run_surface_probe(
    source: &str,
    entry: &str,
    directions: &[[f32; 4]],
    cases: &[[f32; 4]],
    rows_per_probe: usize,
) -> Vec<[f32; 4]> {
    let gpu = cosmos_render::init_for_dx12_luid(None).expect("solar surface probe device");
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(entry),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some(entry),
            layout: None,
            module: &shader,
            entry_point: Some(entry),
            compilation_options: Default::default(),
            cache: None,
        });
    let inputs: Vec<_> = [directions, cases]
        .into_iter()
        .map(|data| {
            gpu.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: None,
                    contents: bytemuck::cast_slice(data),
                    usage: wgpu::BufferUsages::STORAGE,
                })
        })
        .collect();
    let size = (directions.len() * cases.len() * rows_per_probe * 16) as u64;
    let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("solar radiance samples"),
        size,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let readback = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let empty: Vec<_> = (0..2)
        .map(|index| {
            gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &pipeline.get_bind_group_layout(index),
                entries: &[],
            })
        })
        .collect();
    let bindings = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(2),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: inputs[0].as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: inputs[1].as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: output.as_entire_binding(),
            },
        ],
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        for (index, group) in empty.iter().enumerate() {
            pass.set_bind_group(index as u32, group, &[]);
        }
        pass.set_bind_group(2, &bindings, &[]);
        pass.dispatch_workgroups(directions.len().div_ceil(64) as u32, cases.len() as u32, 1);
    }
    encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, size);
    gpu.queue.submit(Some(encoder.finish()));
    let (tx, rx) = std::sync::mpsc::channel();
    readback
        .slice(..)
        .map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .unwrap();
    rx.recv().unwrap().unwrap();
    let bytes = readback.slice(..).get_mapped_range().unwrap();
    bytemuck::cast_slice(&bytes).to_vec()
}

fn luminance(rgb: &[f32; 4]) -> f64 {
    f64::from(rgb[0]) * 0.2126 + f64::from(rgb[1]) * 0.7152 + f64::from(rgb[2]) * 0.0722
}

#[test]
fn solar_penumbral_filter_tables_preserve_shipping_filament_ensemble() {
    const ANGLES: usize = 8192;
    const ROWS: usize = 2;
    let angles: Vec<_> = (0..ANGLES)
        .map(|index| {
            [
                ((index as f64 + 0.5) * std::f64::consts::TAU / ANGLES as f64) as f32,
                0.0,
                0.0,
                0.0,
            ]
        })
        .collect();
    let mut cases = Vec::new();
    for spot in 0..2 {
        for time in PHYSICAL_TIMES {
            // All24 existing0.05-band table knots AND every midpoint. Checking
            // only stored knots would miss biased interpolation between them.
            for radial_index in 0..47 {
                cases.push([radial_index as f32 * 0.025, time, spot as f32, 0.0]);
            }
        }
    }
    let source = concat!(
        include_str!("../../../shaders/common_oled.wgsl"),
        "\n",
        include_str!("../../../shaders/doc_common.wgsl"),
        "\n",
        include_str!("../../../shaders/hero_doc_sun.wgsl"),
        r#"
@group(2) @binding(0) var<storage,read> directions:array<vec4<f32>>;
@group(2) @binding(1) var<storage,read> cases:array<vec4<f32>>;
@group(2) @binding(2) var<storage,read_write> results:array<vec4<f32>>;
@compute @workgroup_size(64)
fn probe_penumbral_ensemble(@builtin(global_invocation_id) id:vec3<u32>) {
    if (id.x>=arrayLength(&directions) || id.y>=arrayLength(&cases)) { return; }
    let angle=directions[id.x].x;
    let params=cases[id.y];
    let offset=(id.y*arrayLength(&directions)+id.x)*2u;
    // Keep radial filtering identical. Only the angular resolution changes,
    // so the actual shipping fallback table is compared with its own ensemble.
    let resolved=doc_sun_penumbral_fibrils(angle,params.x,params.y,u32(params.z),0.0,0.0);
    let filtered=doc_sun_penumbral_fibrils(angle,params.x,params.y,u32(params.z),3.0*TAU/310.0,0.0);
    results[offset]=vec4<f32>(resolved,1.0);
    results[offset+1u]=vec4<f32>(filtered,1.0);
}
"#
    );
    let output = run_surface_probe(source, "probe_penumbral_ensemble", &angles, &cases, ROWS);
    let mut failures = Vec::new();
    let mut max_temperature_bias = 0.0_f64;
    let mut max_outer_coverage_error = 0.0_f64;
    for (case, probes) in cases.iter().zip(output.as_chunks::<{ ANGLES * ROWS }>().0) {
        let mut resolved = [0.0_f64; 3];
        let mut filtered = [0.0_f64; 3];
        for rows in probes.as_chunks::<ROWS>().0 {
            assert!(
                rows.iter()
                    .flatten()
                    .all(|v| v.is_finite() && (0.0..=1.0).contains(v)),
                "penumbral case{case:?}: invalid helper field"
            );
            for channel in 0..3 {
                resolved[channel] += f64::from(rows[0][channel]) / ANGLES as f64;
                filtered[channel] += f64::from(rows[1][channel]) / ANGLES as f64;
            }
        }
        // These are the existing shipping penumbral temperature coefficients;
        // no noise, morphology, hash or filtering formula is copied into Rust.
        let temperature_bias =
            580.0 * (filtered[0] - resolved[0]) + 180.0 * (filtered[1] - resolved[1]);
        let coverage_error = (filtered[2] - resolved[2]).abs();
        max_temperature_bias = max_temperature_bias.max(temperature_bias.abs());
        // Coverage starts participating in the outer material at0.52. Include
        // every sample from0.5 onward conservatively, without excluding failures.
        if case[0] >= 0.5 {
            max_outer_coverage_error = max_outer_coverage_error.max(coverage_error);
        }
        // Machine-readable measured means also provide calibration evidence
        // if a new filament population requires updated tables.
        println!(
            "PENUMBRA_MEAN spot={} physical_time={} radial={:.3} resolved={resolved:?} filtered={filtered:?} temperature_bias_K={temperature_bias:.6} coverage_error={coverage_error:.6}",
            case[2], case[1], case[0]
        );
        if temperature_bias.abs() > 20.0 || (case[0] >= 0.5 && coverage_error > 0.02) {
            failures.push(format!(
                "spot{} time{} radial{:.3}: mean temperature bias{temperature_bias:.3}K (20K budget), coverage error{coverage_error:.6} (outer budget0.02)",
                case[2],case[1],case[0]
            ));
        }
    }
    println!(
        "penumbral ensemble: {}cases × {ANGLES}angles, maximum temperature bias{max_temperature_bias:.4}K, outer coverage error{max_outer_coverage_error:.6}",
        cases.len()
    );
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn solar_surface_is_continuous_across_actual_granule_winner_changes() {
    const PROBES: usize = 2048;
    const ROWS: usize = 6;
    let directions = quiet_directions();
    let directions = &directions[..PROBES];
    let cases: Vec<_> = FEATURES
        .into_iter()
        .flat_map(|feature| PHYSICAL_TIMES.map(|time| [time, 0.0, feature as f32, 0.0]))
        .collect();
    let source = concat!(
        include_str!("../../../shaders/common_oled.wgsl"),
        "\n",
        include_str!("../../../shaders/doc_common.wgsl"),
        "\n",
        include_str!("../../../shaders/hero_doc_sun.wgsl"),
        r#"
@group(2) @binding(0) var<storage,read> directions:array<vec4<f32>>;
@group(2) @binding(1) var<storage,read> cases:array<vec4<f32>>;
@group(2) @binding(2) var<storage,read_write> results:array<vec4<f32>>;
@compute @workgroup_size(64)
fn probe_boundaries(@builtin(global_invocation_id) id:vec3<u32>) {
    if (id.x>=arrayLength(&directions) || id.y>=arrayLength(&cases)) { return; }
    let params=cases[id.y];
    let cell_radius=900000.0/DOC_SUN_RADIUS_M;
    // Preserve the exact application-to-physical f32 clock used by shading.
    let time=params.x/DOC_SUN_TIME_SCALE;
    let physical_time=doc_sun_clock(time);
    // Equal normalization to shade_doc_sun_surface's actual p_surf path.
    var left=normalize(directions[id.x].xyz);
    let axis=select(vec3<f32>(1.0,0.0,0.0),vec3<f32>(0.0,1.0,0.0),abs(left.x)>0.9);
    let tangent=normalize(cross(left,axis));
    var right=normalize(left+tangent*cell_radius*1.25);
    var left_seed=doc_sun_granule(normalize(left),physical_time,0.0).seed;
    var right_seed=doc_sun_granule(normalize(right),physical_time,0.0).seed;
    let found=left_seed!=right_seed;
    let offset=(id.y*arrayLength(&directions)+id.x)*6u;
    for (var row=0u;row<6u;row+=1u) { results[offset+row]=vec4<f32>(0.0); }
    if (!found) { return; }
    var iterations=0u;
    for (var step=0u;step<40u;step+=1u) {
        // Normalizing the vector sum bisects the actual short spherical arc.
        // A third winner still leaves a valid unequal-seed bracket.
        if (length(normalize(right)-normalize(left))<=1e-7) { break; }
        let middle=normalize(left+right);
        if (all(middle==left) || all(middle==right)) { break; }
        let middle_seed=doc_sun_granule(normalize(middle),physical_time,0.0).seed;
        if (middle_seed==left_seed) { left=middle; left_seed=middle_seed; }
        else { right=middle; right_seed=middle_seed; }
        iterations+=1u;
    }
    let ln=normalize(left);
    let rn=normalize(right);
    // Re-query the actual endpoints; a stale or assumed boundary cannot pass.
    let lg=doc_sun_granule(ln,physical_time,0.0);
    let rg=doc_sun_granule(rn,physical_time,0.0);
    var inst:HeroVolumeInstance;
    inst.kind_tags=vec4<u32>(3u,1400u,u32(params.z)<<16u,0u);
    let a=shade_doc_sun_surface(left,ln,ln,inst,time,0.0);
    let b=shade_doc_sun_surface(right,rn,rn,inst,time,0.0);
    let quiet=shade_doc_sun_surface(left,ln,ln,inst,time,cell_radius*1.15);
    results[offset]=vec4<f32>(a,1.0);
    results[offset+1u]=vec4<f32>(b,1.0);
    results[offset+2u]=vec4<f32>(quiet,1.0);
    results[offset+3u]=vec4<f32>(ln,lg.seed);
    results[offset+4u]=vec4<f32>(rn,rg.seed);
    results[offset+5u]=vec4<f32>(lg.edge,rg.edge,f32(iterations),length(rn-ln));
}
"#
    );
    let output = run_surface_probe(source, "probe_boundaries", directions, &cases, ROWS);
    assert_eq!(output.len(), PROBES * cases.len() * ROWS);
    for (case, records) in cases.iter().zip(output.as_chunks::<{ PROBES * ROWS }>().0) {
        let mut actual_boundaries = 0;
        let mut largest_jump = 0.0_f64;
        let mut largest_step = 0.0_f32;
        let mut failures = Vec::new();
        for (index, rows) in records.as_chunks::<ROWS>().0.iter().enumerate() {
            assert!(
                rows.iter().flatten().all(|v| v.is_finite()),
                "case{case:?} probe{index}: nonfinite boundary result"
            );
            if rows[0][3] == 0.0 {
                continue;
            }
            actual_boundaries += 1;
            assert_ne!(
                rows[3][3], rows[4][3],
                "case{case:?} probe{index}: endpoints must have different shipping winners"
            );
            let step = Vec3::from_slice(&rows[3]).distance(Vec3::from_slice(&rows[4]));
            largest_step = largest_step.max(step);
            // One ulp in each normalized f32 component can prevent another
            // sphere midpoint. Four machine epsilons cover that float limit;
            // the usual stop is1e-7R, under0.008% of the900km cell scale.
            assert!(
                step > 0.0 && step <= 4.0 * f32::EPSILON,
                "case{case:?} probe{index}: unresolved spherical bracket{step}R"
            );
            assert!(rows[..3].iter().flat_map(|v| &v[..3]).all(|v| *v >= 0.0));
            let reference = luminance(&rows[2]);
            assert!(reference > 0.0);
            let jump = (luminance(&rows[0]) - luminance(&rows[1])).abs() / reference;
            largest_jump = largest_jump.max(jump);
            // At this tiny spatial step the continuous120km fine texture can
            // change slightly. Permit0.5% of quiet radiance, not a finite cell
            // jump, with a1% per-channel guard against compensating RGB changes.
            let color_jump = (0..3)
                .map(|channel| {
                    f64::from((rows[0][channel] - rows[1][channel]).abs())
                        / f64::from(rows[2][channel]).max(1e-6)
                })
                .fold(0.0_f64, f64::max);
            if (jump > 0.005 || color_jump > 0.01) && failures.len() < 8 {
                failures.push(format!("probe{index} step{step} seeds{:?}/{:?} edge{:?}: luminance jump{:.5}% RGBmax{:.5}%", rows[3][3], rows[4][3], &rows[5][..2], jump*100.0, color_jump*100.0));
            }
        }
        assert!(
            actual_boundaries >= PROBES / 3,
            "case{case:?}: only{actual_boundaries} actual winner changes; insufficient coverage"
        );
        println!(
            "granule boundary feature{} physical_time{}: {actual_boundaries} actual changes, maximum step{largest_step}R, maximum radiance jump{:.5}%",
            case[2],
            case[0],
            largest_jump * 100.0
        );
        assert!(
            failures.is_empty(),
            "case{case:?}: finite material jumps:\n{}",
            failures.join("\n")
        );
    }
}

#[test]
fn solar_surface_preserves_ensemble_radiance_and_continuity_across_detail_lod() {
    let directions = quiet_directions();
    let mut cases = Vec::new();
    for feature in FEATURES {
        for time in PHYSICAL_TIMES {
            for footprint in FOOTPRINTS {
                cases.push([time, footprint, feature as f32, 0.0]);
            }
        }
    }
    let output = probe_surface(&directions, &cases);
    assert_eq!(output.len(), SAMPLE_COUNT * cases.len());
    let mut energy_failures = Vec::new();
    let mean = |samples: &[[f32; 4]]| {
        samples.iter().fold([0.0_f64; 3], |mut sum, rgb| {
            for channel in 0..3 {
                sum[channel] += f64::from(rgb[channel]) / samples.len() as f64;
            }
            sum
        })
    };
    for (group_index, group) in output
        .as_chunks::<{ SAMPLE_COUNT * FOOTPRINTS.len() }>()
        .0
        .iter()
        .enumerate()
    {
        let feature = FEATURES[group_index / PHYSICAL_TIMES.len()];
        let time = PHYSICAL_TIMES[group_index % PHYSICAL_TIMES.len()];
        let samples = group.as_chunks::<SAMPLE_COUNT>().0;
        let unresolved = mean(&samples[8]);
        let reference = unresolved[0] * 0.2126 + unresolved[1] * 0.7152 + unresolved[2] * 0.0722;
        assert!(reference.is_finite() && reference > 0.0);
        for (index, sample) in samples.iter().enumerate() {
            assert!(
                sample.iter().flatten().all(|v| v.is_finite() && *v >= 0.0),
                "feature{feature} time{time} footprint{}: invalid radiance",
                FOOTPRINTS[index]
            );
            let rgb_mean = mean(sample);
            let energy = rgb_mean[0] * 0.2126 + rgb_mean[1] * 0.7152 + rgb_mean[2] * 0.0722;
            let drift = (energy / reference - 1.0).abs();
            println!(
                "feature{feature} physical_time{time} footprint{:.3}: RGBmean{rgb_mean:?} luminance_drift{:.3}%",
                FOOTPRINTS[index],
                drift * 100.0
            );
            // Independent finite sampling and non-calibrated authored plasma
            // justify a few percent, not an exact hand-tuned pixel expectation.
            if drift >= 0.035 {
                energy_failures.push(format!(
                    "feature{feature} time{time} footprint{}: ensemble energy drift{:.3}% exceeds3.5%",
                    FOOTPRINTS[index], drift * 100.0
                ));
            }
            // The contrast transform preserves per-sample chromaticity but
            // changes its ensemble weighting slightly. Allow that small effect
            // while still rejecting a hidden red/blue LOD balance change.
            for channel in 0..3 {
                let color_drift = (rgb_mean[channel] / unresolved[channel] - 1.0).abs();
                if color_drift >= 0.05 {
                    energy_failures.push(format!(
                        "feature{feature} time{time} footprint{} channel{channel}: mean drift{:.3}% exceeds5%",
                        FOOTPRINTS[index], color_drift * 100.0
                    ));
                }
            }
        }
        let resolved_mean = mean(&samples[0]);
        let resolved_energy =
            resolved_mean[0] * 0.2126 + resolved_mean[1] * 0.7152 + resolved_mean[2] * 0.0722;
        let variance = samples[0]
            .iter()
            .map(|rgb| (luminance(rgb) - resolved_energy).powi(2))
            .sum::<f64>()
            / SAMPLE_COUNT as f64;
        assert!(
            variance.sqrt() / resolved_energy > 0.10,
            "feature{feature} time{time}: energy continuity must not pass by erasing all resolved granules"
        );
        let unresolved_spread = samples[8]
            .iter()
            .map(|rgb| (luminance(rgb) - reference).abs())
            .fold(0.0, f64::max);
        assert!(
            unresolved_spread / reference < 1e-4,
            "feature{feature} time{time}: unresolved quiet-Sun detail must converge to its mean"
        );
        for [left, right] in [[9, 10], [11, 12], [13, 14]] {
            let signed = samples[left]
                .iter()
                .zip(&samples[right])
                .map(|(a, b)| luminance(b) - luminance(a))
                .collect::<Vec<_>>();
            let jump = signed.iter().sum::<f64>().abs() / SAMPLE_COUNT as f64 / reference;
            let local_change =
                signed.iter().map(|v| v.abs()).sum::<f64>() / SAMPLE_COUNT as f64 / reference;
            assert!(
                jump < 0.005 && local_change < 0.01,
                "feature{feature} time{time} adjacent footprints{}→{}: mean jump{:.3}%, mean absolute change{:.3}%",
                FOOTPRINTS[left],
                FOOTPRINTS[right],
                jump * 100.0,
                local_change * 100.0
            );
        }
    }
    // Validate the angular sampling itself, without inspecting shader output.
    assert!(
        directions
            .iter()
            .all(|d| (Vec3::from_slice(d).length() - 1.0).abs() < 1e-6)
    );
    assert!(energy_failures.is_empty(), "{}", energy_failures.join("\n"));
}
