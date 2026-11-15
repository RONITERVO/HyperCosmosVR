//! Execute the shipping analytic cutaway on a tiny compute pipeline. This
//! checks geometry against known rays without recompiling the renderer or
//! duplicating the intersection implementation in Rust.
use glam::Vec3;
use wgpu::util::DeviceExt;

#[test]
fn solar_plasma_proxies_enclose_finite_caps_under_oblique_and_near_perspective() {
    use glam::DVec3;
    let mut cases = vec![
        (DVec3::Z * 3.0, DVec3::NEG_X * 0.015, DVec3::X * 0.015, 0.04),
        (DVec3::Z, DVec3::NEG_Z * 0.1, DVec3::Z * 0.1, 0.02),
        (DVec3::Z * 0.005, DVec3::NEG_X * 0.1, DVec3::X * 0.1, 0.02),
        (DVec3::Z * 0.2, DVec3::NEG_Z * 0.1, DVec3::Z, 0.02),
        // A broad cap very close to the eye used to combine incompatible
        // lateral/depth extrema, making the proxy four times too wide.
        (
            DVec3::Z * 0.045,
            DVec3::NEG_X * 0.015,
            DVec3::X * 0.015,
            0.04,
        ),
        (
            DVec3::Z * 0.04001,
            DVec3::NEG_X * 0.015,
            DVec3::X * 0.015,
            0.04,
        ),
        // End-on and nearly side-on caps exercise zero/repeated tangent roots.
        (DVec3::Z * 0.101, DVec3::NEG_Z * 0.1, DVec3::Z * 0.1, 0.02),
        (
            DVec3::Z * 0.045,
            DVec3::new(-0.015, 0.0, -1e-6),
            DVec3::new(0.015, 0.0, 1e-6),
            0.04,
        ),
    ];
    let mut seed = 937_u32;
    let mut random = || {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        f64::from(seed >> 8) / f64::from(1_u32 << 24)
    };
    for _ in 0..512 {
        let center = DVec3::new(random() - 0.5, random() - 0.5, random() - 0.5);
        let axis = DVec3::new(random() - 0.5, random() - 0.5, random() - 0.5).normalize();
        let view = DVec3::new(random() - 0.5, random() - 0.5, random() - 0.5).normalize();
        let half = 0.002 + random() * 0.6;
        cases.push((
            center + view * (0.001 + random() * 5.0),
            center - axis * half,
            center + axis * half,
            0.0001 + random() * 0.08,
        ));
    }
    let inputs: Vec<[f32; 4]> = cases
        .iter()
        .flat_map(|(ro, a, b, bound)| {
            [
                ro.as_vec3().extend(*bound as f32).to_array(),
                a.as_vec3().extend(0.0).to_array(),
                b.as_vec3().extend(0.0).to_array(),
            ]
        })
        .collect();
    let source = concat!(
        include_str!("../../../shaders/common_oled.wgsl"),
        "\n",
        include_str!("../../../shaders/doc_common.wgsl"),
        "\n",
        include_str!("../../../shaders/hero_doc_sun.wgsl"),
        "\n",
        include_str!("../../../shaders/hero_solar_interior.wgsl"),
        "\n",
        include_str!("../../../shaders/solar_plasma.wgsl"),
        "\n",
        "@group(2) @binding(0) var<storage,read> probes:array<vec4<f32>>;\n",
        "@group(2) @binding(1) var<storage,read_write> results:array<vec4<f32>>;\n",
        "@compute @workgroup_size(1) fn probe(@builtin(global_invocation_id) id:vec3<u32>) {\n",
        "let ro=probes[id.x*3u]; let p=solar_plasma_proxy(ro.xyz,probes[id.x*3u+1u].xyz,probes[id.x*3u+2u].xyz,ro.w);\n",
        "results[id.x*4u]=p.limits; results[id.x*4u+1u]=vec4<f32>(p.center,select(0.0,1.0,p.full_screen));\n",
        "results[id.x*4u+2u]=vec4<f32>(p.along,0.0); results[id.x*4u+3u]=vec4<f32>(p.side,0.0); }\n"
    );
    let output = run_solar_probe(source, "probe", &inputs, cases.len(), 4);
    assert_eq!(
        output[2 * 4 + 1][3],
        1.0,
        "a cylinder crossing the eye plane needs full-viewport coverage"
    );
    assert_eq!(
        output[3 * 4 + 1][3],
        1.0,
        "an end-on cylinder surrounding the observer needs full-viewport coverage"
    );
    assert!(
        output[0][1] - output[0][0] < 0.032,
        "side-on CME segments must not retain the old 0.11R padded length"
    );
    assert!(
        output[4 * 4][3] - output[4 * 4][2] < 0.18,
        "near cap must use its actual tangent rays, not the old 0.72R independent-extrema width"
    );
    let mut finite = 0;
    for (index, (ro, a, b, bound)) in cases.iter().enumerate() {
        let data = &output[index * 4..index * 4 + 4];
        assert!(
            data.iter().flatten().all(|v| v.is_finite()),
            "case{index}: non-finite proxy"
        );
        if data[1][3] > 0.5 {
            continue;
        }
        finite += 1;
        let center = Vec3::from_slice(&data[1]).as_dvec3();
        let along = Vec3::from_slice(&data[2]).as_dvec3();
        let side = Vec3::from_slice(&data[3]).as_dvec3();
        let view = (ro - center).normalize();
        assert!(along.dot(side).abs() < 1e-5 && along.dot(view).abs() < 1e-5);
        let axis = (b - a).normalize();
        let radial_x = axis
            .cross(if axis.y.abs() < 0.9 {
                DVec3::Y
            } else {
                DVec3::X
            })
            .normalize();
        let radial_y = axis.cross(radial_x);
        // Independently project actual cap/cylinder points along rays through
        // the eye onto the proxy's plane. No interval-bound formula is copied.
        for axial in [0.0, 0.25, 0.5, 0.75, 1.0] {
            for angle_index in 0..128 {
                let angle = f64::from(angle_index) * std::f64::consts::TAU / 128.0;
                let point =
                    a.lerp(*b, axial) + (radial_x * angle.cos() + radial_y * angle.sin()) * bound;
                let ray = point - ro;
                let plane_t = (center - ro).dot(view) / ray.dot(view);
                assert!(
                    plane_t > 0.0,
                    "case{index}: finite proxy cannot contain a point behind the eye"
                );
                let projected = ro + ray * plane_t - center;
                let xy = [projected.dot(along), projected.dot(side)];
                for dimension in 0..2 {
                    let low = f64::from(data[0][dimension * 2]);
                    let high = f64::from(data[0][dimension * 2 + 1]);
                    let epsilon = 1e-5 * (1.0 + low.abs().max(high.abs()));
                    assert!(
                        xy[dimension] >= low - epsilon && xy[dimension] <= high + epsilon,
                        "case{index} axial{axial} angle{angle_index}: projected{} outside[{low},{high}]",
                        xy[dimension]
                    );
                }
            }
        }
    }
    assert!(
        finite > 400,
        "random coverage must include hundreds of finite oblique/near bounds"
    );
}

#[test]
fn solar_corona_quadrature_matches_dense_independent_columns() {
    let latitude = 20.0_f32.to_radians();
    let c = Vec3::new(0.0, latitude.sin(), latitude.cos());
    let n = Vec3::new(0.0, latitude.cos(), -latitude.sin());
    let mut cases = Vec::new();
    for impact in [
        0.0, 0.05, 0.3, 0.7, 0.98, 0.9999, 1.0001, 1.005, 1.02, 1.1, 1.5, 2.8,
    ] {
        cases.push((
            format!("active disk/limb impact{impact}"),
            c * 3.0 + Vec3::X * impact,
            -c,
            false,
        ));
        cases.push((
            format!("quiet disk/limb impact{impact}"),
            Vec3::Z * 3.0 + Vec3::X * impact,
            Vec3::NEG_Z,
            false,
        ));
    }
    cases.extend([
        ("close tangent active".into(), c * 1.01, Vec3::X, false),
        (
            "close inward active".into(),
            c * 1.055,
            (Vec3::X * 0.15 - c).normalize(),
            false,
        ),
        (
            "close outward active".into(),
            c * 1.055,
            (Vec3::X * 0.15 + c).normalize(),
            false,
        ),
        (
            "interior cutaway crossing".into(),
            (c + Vec3::X * 0.45 + n * 0.25).normalize() * 2.4,
            -(c + Vec3::X * 0.45 + n * 0.25).normalize(),
            true,
        ),
        (
            "inside opening outward".into(),
            (c + Vec3::X + n * 0.2).normalize() * 0.8,
            (c + Vec3::X + n * 0.2).normalize(),
            true,
        ),
    ]);
    let input: Vec<[f32; 4]> = cases
        .iter()
        .flat_map(|(_, ro, rd, cut)| {
            [
                ro.extend(if *cut { 1.0 } else { 0.0 }).to_array(),
                rd.extend(0.0).to_array(),
            ]
        })
        .collect();
    let source = concat!(
        include_str!("../../../shaders/common_oled.wgsl"),
        "\n",
        include_str!("../../../shaders/doc_common.wgsl"),
        "\n",
        include_str!("../../../shaders/hero_doc_sun.wgsl"),
        "\n",
        include_str!("../../../shaders/hero_solar_interior.wgsl"),
        "\n",
        include_str!("../../../shaders/hero_solar_corona.wgsl"),
        "\n",
        "@group(2) @binding(0) var<storage,read> probes:array<vec4<f32>>;\n",
        "@group(2) @binding(1) var<storage,read_write> results:array<vec4<f32>>;\n",
        "@compute @workgroup_size(1) fn probe_columns(@builtin(global_invocation_id) id:vec3<u32>) {\n",
        "let o=probes[id.x*2u]; let ro=o.xyz; let rd=normalize(probes[id.x*2u+1u].xyz);\n",
        "let outer=intersect_sphere(ro,rd,3.0); let start=max(outer.x,0.0); var stop=outer.y;\n",
        "let inner=intersect_sphere(ro,rd,1.0);\n",
        "if(o.w>0.5) {stop=min(stop,solar_cutaway_hit(ro,rd,1.0).t);}\n",
        "else if(inner.y>0.0 && inner.x>0.0) {stop=min(stop,inner.x);}\n",
        "let actual=solar_corona_columns(ro,rd,start,stop);\n",
        "var reference=vec3<f32>(0.0); var compensation=vec3<f32>(0.0);\n",
        // Uniform midpoint integration has no closest-point transform, Gauss
        // nodes or production quadrature weights. Split only the hollow volume
        // to avoid a reference error from sampling the solid interior boundary.
        "for(var side=0u;side<2u;side+=1u) {var a=start;var b=stop;\n",
        "if(inner.y>inner.x) {if(side==0u) {b=min(b,inner.x);} else {a=max(a,inner.y);}}\n",
        "else if(side==1u) {continue;} if(b<=a) {continue;}\n",
        "let dt=(b-a)/8192.0; for(var i=0u;i<8192u;i+=1u) {\n",
        "let p=ro+rd*(a+(f32(i)+0.5)*dt); let y=solar_corona_components(p)*dt-compensation;\n",
        "let next=reference+y; compensation=(next-reference)-y; reference=next; }}\n",
        "results[id.x*2u]=vec4<f32>(actual.value,0.0); results[id.x*2u+1u]=vec4<f32>(reference,0.0); }\n"
    );
    let output = run_solar_probe(source, "probe_columns", &input, cases.len(), 2);
    let mut failures = Vec::new();
    let mut worst = 0.0_f32;
    for (index, (label, _, _, _)) in cases.iter().enumerate() {
        let actual = output[index * 2];
        let reference = output[index * 2 + 1];
        assert!(actual.iter().chain(reference.iter()).all(|v| v.is_finite()));
        for component in 0..3 {
            let error = (actual[component] - reference[component]).abs();
            let relative = error / reference[component].abs().max(1e-7);
            worst = worst.max(relative);
            if error > 1e-7_f32.max(reference[component].abs() * 0.005) {
                failures.push(format!(
                    "{label} component{component}: actual{} reference{} relative{relative:.6}",
                    actual[component], reference[component]
                ));
            }
        }
    }
    eprintln!("solar corona worst component relative error={worst:.6}");
    assert!(
        failures.is_empty(),
        "solar corona integration exceeds0.5% tolerance:\n{}",
        failures.join("\n")
    );
}

fn run_solar_probe(
    source: &str,
    entry: &str,
    input: &[[f32; 4]],
    cases: usize,
    rows_per_case: usize,
) -> Vec<[f32; 4]> {
    let gpu = cosmos_render::init_for_dx12_luid(None).expect("solar analytic probe device");
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
    let input_buffer = gpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(input),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let size = (cases * rows_per_case * 16) as u64;
    let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
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
        .map(|i| {
            gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &pipeline.get_bind_group_layout(i),
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
                resource: input_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output.as_entire_binding(),
            },
        ],
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        for (i, group) in empty.iter().enumerate() {
            pass.set_bind_group(i as u32, group, &[]);
        }
        pass.set_bind_group(2, &bindings, &[]);
        pass.dispatch_workgroups(cases as u32, 1, 1);
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

#[test]
fn solar_strand_integrals_remain_finite_and_continuous_when_viewed_along_the_axis() {
    use glam::DVec3;
    let mut cases = Vec::new();
    let a = DVec3::ZERO;
    let b = DVec3::Z * 0.1;
    // This covers the discontinuity at the old denominator floor, finite cap
    // clipping, subpixel filtering, reversed rays and a truly perpendicular ray.
    for epsilon in [0.0, 1e-7, 1e-6, 1e-4, 0.001, 0.01] {
        let direction = DVec3::new(epsilon, 0.0, -1.0).normalize();
        cases.push((
            format!("near-axis epsilon={epsilon}"),
            DVec3::new(0.0002 - epsilon * 2.95, 0.0, 3.0),
            direction,
            0.0002,
            0.0,
            4.0,
        ));
    }
    cases.extend([
        (
            "perpendicular".into(),
            DVec3::new(1.0, 0.0002, 0.05),
            -DVec3::X,
            0.0002,
            0.0,
            2.0,
        ),
        (
            "reversed axial ray".into(),
            DVec3::new(0.0002, 0.0, -3.0),
            DVec3::Z,
            0.0002,
            0.0,
            4.0,
        ),
        (
            "segment behind eye".into(),
            DVec3::new(0.0002, 0.0, 3.0),
            DVec3::Z,
            0.0002,
            0.0,
            4.0,
        ),
        (
            "outside axial caps".into(),
            DVec3::new(1.0, 0.0, 0.20),
            -DVec3::X,
            0.0002,
            0.0,
            2.0,
        ),
        (
            "half obscured axial ray".into(),
            DVec3::new(0.0002, 0.0, 3.0),
            -DVec3::Z,
            0.0002,
            0.0,
            2.95,
        ),
        (
            "distant subpixel axial ray".into(),
            DVec3::new(0.0002, 0.0, 3.0),
            -DVec3::Z,
            0.0002,
            0.001,
            4.0,
        ),
    ]);
    let mut input = Vec::new();
    let mut expected = Vec::new();
    for (_, origin, direction, sigma0, pixel_scale, limit) in &cases {
        input.extend([
            origin.as_vec3().extend(*sigma0 as f32).to_array(),
            direction.as_vec3().extend(*pixel_scale as f32).to_array(),
            a.as_vec3().extend(*limit as f32).to_array(),
            b.as_vec3().extend(0.0).to_array(),
        ]);
        // Independent numerical quadrature in world space. No ray/axis closest
        // point, epsilon denominator, erf or shader interval algebra is copied.
        let pixel_depth = ((a + b) * 0.5 - origin).dot(*direction).clamp(0.0, *limit);
        let sigma = (sigma0 * sigma0 + (pixel_depth * pixel_scale).powi(2) / 12.0).sqrt();
        let dt = limit / 200_000.0;
        let mut value = 0.0;
        for index in 0..200_000 {
            let p = origin + direction * ((f64::from(index) + 0.5) * dt);
            if (0.0..=0.1).contains(&p.z) {
                value += (-(p.x * p.x + p.y * p.y) / (2.0 * sigma * sigma)).exp() * dt;
            }
        }
        expected.push(value / ((2.0 * std::f64::consts::PI).sqrt() * sigma) * sigma0 / sigma);
    }
    let source = concat!(
        include_str!("../../../shaders/common_oled.wgsl"),
        "\n",
        include_str!("../../../shaders/doc_common.wgsl"),
        "\n",
        include_str!("../../../shaders/hero_doc_sun.wgsl"),
        "\n",
        include_str!("../../../shaders/hero_solar_interior.wgsl"),
        "\n",
        include_str!("../../../shaders/solar_plasma.wgsl"),
        "\n",
        "@group(2) @binding(0) var<storage,read> probes: array<vec4<f32>>;\n",
        "@group(2) @binding(1) var<storage,read_write> result: array<vec4<f32>>;\n",
        "@compute @workgroup_size(1) fn integral_probe(@builtin(global_invocation_id) id: vec3<u32>) {\n",
        "let o=probes[id.x*4u]; let d=probes[id.x*4u+1u]; let a=probes[id.x*4u+2u]; let b=probes[id.x*4u+3u];\n",
        "let v=solar_plasma_integral(o.xyz,d.xyz,a.xyz,b.xyz,o.w,d.w,a.w);\n",
        "result[id.x]=vec4<f32>(v.column,v.depth,v.along,0.0); }\n"
    );
    let gpu = cosmos_render::init_for_dx12_luid(None).expect("solar strand test device");
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shipping solar strand integral"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("solar strand integral regression"),
            layout: None,
            module: &shader,
            entry_point: Some("integral_probe"),
            compilation_options: Default::default(),
            cache: None,
        });
    let buffer = gpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("solar strand probes"),
            contents: bytemuck::cast_slice(&input),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let size = cases.len() as u64 * 16;
    let output = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
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
                resource: buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
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
        pass.dispatch_workgroups(cases.len() as u32, 1, 1);
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
    let rows: &[[f32; 4]] = bytemuck::cast_slice(&bytes);
    for ((case, expected), actual) in cases.iter().zip(expected).zip(rows) {
        assert!(
            actual.iter().all(|value| value.is_finite()),
            "{}: non-finite {actual:?}",
            case.0
        );
        assert!(
            (f64::from(actual[0]) - expected).abs() < expected * 0.005 + 1e-5,
            "{}: GPU column {} versus numerical integral {expected}",
            case.0,
            actual[0]
        );
        assert!((0.0..=case.5 as f32).contains(&actual[1]));
        assert!((0.0..=1.0).contains(&actual[2]));
    }
}

#[test]
fn solar_cutaway_uses_true_retained_geometry_and_rejects_sphere_misses() {
    let gpu = cosmos_render::init_for_dx12_luid(None).expect("solar intersection test device");
    let shader = gpu.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("shipping solar cutaway geometry"),
        source: wgpu::ShaderSource::Wgsl(concat!(
            include_str!("../../../shaders/common_oled.wgsl"), "\n",
            include_str!("../../../shaders/doc_common.wgsl"), "\n",
            include_str!("../../../shaders/hero_solar_interior.wgsl"), "\n",
            "struct GeometryRay { origin: vec4<f32>, direction: vec4<f32> };\n",
            "@group(2) @binding(0) var<storage, read> rays: array<GeometryRay>;\n",
            "@group(2) @binding(1) var<storage, read_write> results: array<vec4<f32>>;\n",
            "@compute @workgroup_size(1) fn test_geometry(@builtin(global_invocation_id) id: vec3<u32>) {\n",
            "let ray = rays[id.x];\n",
            "let hit = solar_cutaway_hit(ray.origin.xyz, ray.direction.xyz, ray.origin.w);\n",
            "results[id.x * 2u] = vec4<f32>(hit.t, f32(hit.region), hit.normal.xy);\n",
            "results[id.x * 2u + 1u] = vec4<f32>(hit.normal.z, 0.0, 0.0, 0.0);\n",
            "}\n"
        ).into()),
    });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("solar analytic geometry regression"),
            layout: None,
            module: &shader,
            entry_point: Some("test_geometry"),
            compilation_options: Default::default(),
            cache: None,
        });
    let angle = 20.0_f32.to_radians();
    let east = Vec3::X;
    let north = Vec3::new(0.0, angle.cos(), -angle.sin());
    let center = Vec3::new(0.0, angle.sin(), angle.cos());
    let frame = |p: Vec3| east * p.x + north * p.y + center * p.z;
    let diagonal = Vec3::ONE.normalize();
    // origin/direction are expressed in E/N/C; each expected distance follows
    // directly from a known sphere radius or an axis-aligned cut-plane crossing.
    let cases = [
        (
            "miss outer and core",
            Vec3::new(2.0, 0.0, 3.0),
            Vec3::NEG_Z,
            None,
        ),
        (
            "near limb miss",
            Vec3::new(1.01, 0.0, 3.0),
            Vec3::NEG_Z,
            None,
        ),
        ("looking outward", diagonal * 3.0, diagonal, None),
        (
            "retained photosphere",
            -Vec3::X * 3.0,
            Vec3::X,
            Some((2.0, 0, -east)),
        ),
        (
            "opening reveals core",
            diagonal * 3.0,
            -diagonal,
            Some((2.75, 1, frame(diagonal))),
        ),
        (
            "core miss reaches radiative wall",
            Vec3::new(0.6, 0.3, 3.0),
            Vec3::NEG_Z,
            Some((3.0, 2, center)),
        ),
        (
            "convection wall",
            Vec3::new(0.8, 0.3, 3.0),
            Vec3::NEG_Z,
            Some((3.0, 3, center)),
        ),
        (
            "inside opening",
            Vec3::new(0.6, 0.3, 0.3),
            Vec3::NEG_Z,
            Some((0.3, 2, center)),
        ),
        (
            "inside core",
            Vec3::ZERO,
            diagonal,
            Some((0.25, 1, frame(diagonal))),
        ),
        (
            "inside retained hemisphere",
            -Vec3::X * 0.6,
            Vec3::NEG_X,
            Some((0.4, 0, -east)),
        ),
    ];
    let mut rays = Vec::<[f32; 4]>::new();
    let mut expected = Vec::new();
    // Both actual surface and coronal proxy conventions must use the same
    // geometry, not a hidden hardcoded unit sphere or fixed near distance.
    for radius in [1.0, 0.84, 1.0 / 3.0] {
        for (label, origin, direction, hit) in cases {
            rays.push((frame(origin) * radius).extend(radius).to_array());
            rays.push(frame(direction).extend(0.0).to_array());
            expected.push((label, radius, hit));
        }
    }
    let ray_buffer = gpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("known solar rays"),
            contents: bytemuck::cast_slice(&rays),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let byte_count = expected.len() as u64 * 32;
    let result_buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("solar intersection results"),
        size: byte_count,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
        mapped_at_creation: false,
    });
    let readback = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("solar geometry readback"),
        size: byte_count,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let empty_groups: Vec<_> = (0..2)
        .map(|group| {
            gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("unused shipping shader bindings"),
                layout: &pipeline.get_bind_group_layout(group),
                entries: &[],
            })
        })
        .collect();
    let test_group = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("solar geometry test buffers"),
        layout: &pipeline.get_bind_group_layout(2),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: ray_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: result_buffer.as_entire_binding(),
            },
        ],
    });
    let mut encoder = gpu.device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        for (index, group) in empty_groups.iter().enumerate() {
            pass.set_bind_group(index as u32, group, &[]);
        }
        pass.set_bind_group(2, &test_group, &[]);
        pass.dispatch_workgroups(expected.len() as u32, 1, 1);
    }
    encoder.copy_buffer_to_buffer(&result_buffer, 0, &readback, 0, byte_count);
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
    let rows: &[[f32; 4]] = bytemuck::cast_slice(&bytes);
    for (index, (label, radius, hit)) in expected.iter().enumerate() {
        let row = rows[index * 2];
        assert!(
            row.iter().all(|value| value.is_finite()),
            "{label}: non-finite hit {row:?}"
        );
        if let Some((distance, region, normal)) = hit {
            assert!(
                (row[0] - distance * radius).abs() < 2e-5,
                "{label} at R={radius}: wrong depth {row:?}"
            );
            assert_eq!(row[1] as u32, *region, "{label}: wrong material region");
            let actual_normal = Vec3::new(row[2], row[3], rows[index * 2 + 1][0]);
            assert!(
                actual_normal.distance(*normal) < 2e-5,
                "{label}: wrong surface normal {actual_normal:?}"
            );
        } else {
            assert!(
                row[0] > 1e20,
                "{label} at R={radius}: sphere-miss sentinel became a hit {row:?}"
            );
        }
    }
}
