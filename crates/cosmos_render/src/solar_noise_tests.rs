//! Verify the shipping texture interpolation against independent, exactly
//! representable corner values instead of duplicating its GPU hash function.
use super::*;
use wgpu::util::DeviceExt;

fn corner(x: i32, y: i32, z: i32, channel: usize) -> usize {
    let [x, y, z] = [x, y, z].map(|v| v.rem_euclid(DIM as i32) as u32);
    match channel {
        0 => ((3 * x + 5 * y + 7 * z) ^ (x >> 2)) as usize & 7,
        1 => ((7 * x + 3 * y + 5 * z) ^ (z >> 1)) as usize & 7,
        _ => ((5 * x + 7 * y + 3 * z) ^ (y >> 3)) as usize & 7,
    }
}

fn reference(p: [f32; 4]) -> [f64; 3] {
    let cell = [p[0], p[1], p[2]].map(|v| v.floor() as i32);
    let weight = std::array::from_fn::<_, 3, _>(|axis| {
        let t = f64::from(p[axis]) - f64::from(cell[axis]);
        6.0 * t.powi(5) - 15.0 * t.powi(4) + 10.0 * t.powi(3)
    });
    std::array::from_fn(|channel| {
        let mut sum = 0.0;
        for z in 0..2 {
            for y in 0..2 {
                for x in 0..2 {
                    let w: f64 = [x, y, z]
                        .into_iter()
                        .enumerate()
                        .map(|(axis, bit)| {
                            if bit == 0 {
                                1.0 - weight[axis]
                            } else {
                                weight[axis]
                            }
                        })
                        .product();
                    sum += w * corner(cell[0] + x, cell[1] + y, cell[2] + z, channel) as f64 / 8.0;
                }
            }
        }
        sum * 2.0 - 1.0
    })
}

fn interpolation_error_bound(p: [f32; 4], channel: usize) -> f64 {
    let cell = [p[0], p[1], p[2]].map(|v| v.floor() as i32);
    // A trilinear partial derivative is a convex combination of the four
    // parallel edge slopes. Allow one 8-bit hardware weight step per axis,
    // plus float coordinate arithmetic, rather than an empirical image gate.
    let mut bound = 0.0001;
    for axis in 0..3 {
        let mut largest_edge: f64 = 0.0;
        for other in 0..4 {
            let mut a = cell;
            a[(axis + 1) % 3] += other & 1;
            a[(axis + 2) % 3] += other >> 1;
            let mut b = a;
            b[axis] += 1;
            let difference = (corner(a[0], a[1], a[2], channel) as f64
                - corner(b[0], b[1], b[2], channel) as f64)
                .abs()
                / 4.0;
            largest_edge = largest_edge.max(difference);
        }
        bound += largest_edge / 256.0;
    }
    bound
}

#[test]
#[ignore = "requires a GPU; run explicitly with --ignored on the rendering test host"]
fn solar_cached_noise_matches_independent_lattice_and_periodic_seams() {
    let gpu = crate::init_for_dx12_luid(None).expect("solar noise GPU");
    let (texture, view, sampler) = create_lattice(&gpu.device, &gpu.queue);
    let mut positions: Vec<[f32; 4]> = vec![
        [17.0, 43.0, 91.0, 0.0],
        [0.0, 0.0, 0.0, 0.0],
        [127.0, 127.0, 127.0, 0.0],
        [-1.0, -2.0, -3.0, 0.0],
    ];
    // These triplets exercise both sides of each periodic seam and the center
    // itself; the following copy also checks all three axes shifted by a period.
    for axis in 0..3 {
        for seam in [-128.0, 0.0, 128.0] {
            for offset in [-0.001, 0.0, 0.001] {
                let mut p = [3.25, -2.75, 19.5, 0.0];
                p[axis] = seam + offset;
                positions.push(p);
            }
        }
    }
    let mut seed = 5739_u32;
    for _ in 0..512 {
        let p = std::array::from_fn(|axis| {
            if axis == 3 {
                return 0.0;
            }
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            (seed >> 8) as f32 / (1_u32 << 24) as f32 * 256.0 - 128.0
        });
        positions.push(p);
    }
    let period_start = positions.len();
    positions.extend(
        positions
            .clone()
            .into_iter()
            .map(|p| [p[0] + 128.0, p[1] + 128.0, p[2] + 128.0, 0.0]),
    );
    let source = [
        include_str!("../../../shaders/common_oled.wgsl"),
        include_str!("../../../shaders/doc_common.wgsl"),
        r#"
@group(2) @binding(0) var<storage,read> probes:array<vec4<f32>>;
@group(2) @binding(1) var<storage,read_write> results:array<vec4<f32>>;
@compute @workgroup_size(1) fn sample_lattice(@builtin(global_invocation_id) id:vec3<u32>) {
    let p=probes[id.x].xyz;
    results[id.x*2u]=vec4<f32>(solar_cached_noise(p),1.0);
    let cell=vec3<i32>(floor(p)) & vec3<i32>(127);
    let expected=hash33(vec3<f32>(cell));
    results[id.x*2u+1u]=vec4<f32>(abs(textureLoad(solar_noise_tex,cell,0).xyz-expected),1.0);
}
"#,
    ]
    .join("\n");
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("solar cached noise numerical probe"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let pipeline = gpu
        .device
        .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &shader,
            entry_point: Some("sample_lattice"),
            compilation_options: Default::default(),
            cache: None,
        });
    let empty = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[],
    });
    let noise_bg = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(1),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 7,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 8,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });
    let input = gpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&positions),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let size = (positions.len() * 2 * 16) as u64;
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
    let probe_bg = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &pipeline.get_bind_group_layout(2),
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: input.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: output.as_entire_binding(),
            },
        ],
    });
    let run = || {
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_compute_pass(&Default::default());
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &empty, &[]);
            pass.set_bind_group(1, &noise_bg, &[]);
            pass.set_bind_group(2, &probe_bg, &[]);
            pass.dispatch_workgroups(positions.len() as u32, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, size);
        gpu.queue.submit(Some(encoder.finish()));
        let (tx, rx) = std::sync::mpsc::channel();
        readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |r| tx.send(r).unwrap());
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();
        rx.recv().unwrap().unwrap();
        let values: Vec<[f32; 4]> =
            bytemuck::cast_slice(&readback.slice(..).get_mapped_range().unwrap()).to_vec();
        readback.unmap();
        values
    };
    let initialized = run();
    assert!(
        initialized
            .iter()
            .skip(1)
            .step_by(2)
            .all(|v| v[..3].iter().all(|e| *e <= 0.0005)),
        "GPU initialization must match hash corners within half-float quantization"
    );
    // Upload an unrelated known lattice. All values are exact IEEE binary16
    // eighths, separating interpolation errors from hash and quantization errors.
    let half_eighths = [
        0_u16, 0x3000, 0x3400, 0x3600, 0x3800, 0x3900, 0x3a00, 0x3b00,
    ];
    let mut texels = Vec::with_capacity((DIM * DIM * DIM * 4) as usize);
    for z in 0..DIM as i32 {
        for y in 0..DIM as i32 {
            for x in 0..DIM as i32 {
                for channel in 0..3 {
                    texels.push(half_eighths[corner(x, y, z, channel)]);
                }
                texels.push(0x3c00);
            }
        }
    }
    gpu.queue.write_texture(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        bytemuck::cast_slice(&texels),
        wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(DIM * 8),
            rows_per_image: Some(DIM),
        },
        wgpu::Extent3d {
            width: DIM,
            height: DIM,
            depth_or_array_layers: DIM,
        },
    );
    let observed = run();
    let mut worst: f64 = 0.0;
    for (i, &p) in positions.iter().enumerate() {
        for (channel, expected) in reference(p).into_iter().enumerate() {
            let error = (f64::from(observed[i * 2][channel]) - expected).abs();
            worst = worst.max(error);
            let allowed = interpolation_error_bound(p, channel);
            assert!(
                error < allowed,
                "probe{i} {p:?} channel{channel}: error={error}, bound={allowed}"
            );
            if i < 4 {
                assert!(error < 1e-6, "integer corner samples must be exact");
            }
        }
    }
    for i in 0..period_start {
        for (channel, value) in observed[i * 2][..3].iter().enumerate() {
            assert!(
                (*value - observed[(i + period_start) * 2][channel]).abs() < 0.0003,
                "period mismatch at {i}, channel{channel}"
            );
        }
    }
    for axis in 0..3 {
        for seam in 0..3 {
            let center = 4 + axis * 9 + seam * 3 + 1;
            for (channel, value) in observed[(center - 1) * 2][..3].iter().enumerate() {
                assert!(
                    (*value - observed[(center + 1) * 2][channel]).abs() < 0.0003,
                    "periodic seam must remain continuous"
                );
            }
        }
    }
    eprintln!(
        "solar cached-noise max absolute interpolation error={worst:.8}, {} samples",
        positions.len()
    );
}
