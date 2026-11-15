//! Small GPU test of the production strand fragment entries and blend pipeline.
//! Controlled crossing rays isolate compositing from the authored curve layout.
use super::*;
use wgpu::util::DeviceExt;

#[test]
#[ignore = "requires a GPU; run explicitly with --ignored on the rendering test host"]
fn solar_absorption_and_emission_are_independent_of_strand_submission_order() {
    let gpu = crate::init_for_dx12_luid(None).expect("solar blend GPU");
    let source = concat_solar_plasma_shaders()
        + r#"
@vertex fn vs_blend_probe(@builtin(vertex_index) v:u32,
    @builtin(instance_index) i:u32) -> SolarPlasmaVertex {
    var out: SolarPlasmaVertex;
    let p = array<vec2<f32>,3>(vec2<f32>(-1,-1),vec2<f32>(3,-1),vec2<f32>(-1,3));
    out.clip=vec4<f32>(p[v],0.5,1.0);
    out.local_position=vec3<f32>(0,0,1.1);
    // Two overlapping finite strands at different true depths and columns.
    out.a=vec3<f32>(-0.08,f32(i)*0.008,1.1+f32(i)*0.04);
    out.b=vec3<f32>(0.08,f32(i)*0.008,1.1+f32(i)*0.04);
    out.eye=vec3<f32>(0,0,3);
    // Sample two cool condensations at their centers: this isolates strong
    // physical opacity from later changes to the optically thin CME material.
    let seed=0.2+f32(i)*0.6;
    let delay=solar_plasma_hash(7.0).y*7.0;
    let rain_time=fract((u_frame.scale_params.w-30.0-delay)*0.085+seed);
    let rain_u=0.5*pow(rain_time,1.65);
    out.data=vec4<f32>(0.02,0.5-rain_u,0.0,seed);
    out.tags=vec4<u32>(0u,0u,i,0u);
    return out;
}
"#;
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shipping solar blend probes"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });
    let frame_layout = gpu
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[uniform_entry(
                0,
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                std::mem::size_of::<FrameUniforms>(),
            )],
        });
    let hero_layout = gpu
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                storage_entry(
                    0,
                    wgpu::ShaderStages::VERTEX_FRAGMENT,
                    true,
                    std::mem::size_of::<HeroVolumeInstance>(),
                ),
                wgpu::BindGroupLayoutEntry {
                    binding: 7,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D3,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 8,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });
    let layout = gpu
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[Some(&frame_layout), Some(&hero_layout)],
            immediate_size: 0,
        });
    let absorb = build_solar_plasma_pipeline(
        &gpu.device,
        &layout,
        &shader,
        "vs_blend_probe",
        "fs_solar_absorption",
        None,
        None,
    );
    let emit = build_solar_plasma_pipeline(
        &gpu.device,
        &layout,
        &shader,
        "vs_blend_probe",
        "fs_solar_plasma",
        None,
        None,
    );
    let emit_loop = build_solar_plasma_pipeline(
        &gpu.device,
        &layout,
        &shader,
        "vs_blend_probe",
        "fs_solar_plasma",
        None,
        Some(0),
    );
    let projection =
        cosmos_math::reversed_z_infinite_perspective_rh(std::f32::consts::FRAC_PI_2, 1.0, 0.02)
            .to_cols_array_2d();
    let frame = FrameUniforms {
        view_proj: [projection; 2],
        scale_params: [0.0, 0.0, 1.0, 44.0],
        viewport_params: [4096.0, 4096.0, 1.0 / 4096.0, 1.0 / 4096.0],
        ..Default::default()
    };
    let hero = HeroVolumeInstance {
        pos_radius: [0.0, 0.0, -3.0, 3.0],
        rot_quat: [0.0, 0.0, 0.0, 1.0],
        phys_a: [0.0, 1.0 / 3.0, 0.0, 0.0],
        kind_tags: [3, 1400, 3 << 16, 0],
        ..bytemuck::Zeroable::zeroed()
    };
    let frame_buffer = gpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&frame),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
    let hero_buffer = gpu
        .device
        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&hero),
            usage: wgpu::BufferUsages::STORAGE,
        });
    let frame_bg = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &frame_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: frame_buffer.as_entire_binding(),
        }],
    });
    let (_noise, noise_view, noise_sampler) =
        crate::solar_noise::create_lattice(&gpu.device, &gpu.queue);
    let hero_bg = gpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &hero_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: hero_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 7,
                resource: wgpu::BindingResource::TextureView(&noise_view),
            },
            wgpu::BindGroupEntry {
                binding: 8,
                resource: wgpu::BindingResource::Sampler(&noise_sampler),
            },
        ],
    });
    let size = wgpu::Extent3d {
        width: 1,
        height: 1,
        depth_or_array_layers: 1,
    };
    let target = gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: HDR_SCENE_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let color = target.create_view(&Default::default());
    let depth = gpu
        .device
        .create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
        .create_view(&Default::default());
    let readback = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 256,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let render = |bg: [f64; 3], do_absorb: bool, do_emit: bool, reverse: bool, fixed: bool| {
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &color,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: bg[0],
                            g: bg[1],
                            b: bg[2],
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                ..Default::default()
            });
            pass.set_bind_group(0, &frame_bg, &[]);
            pass.set_bind_group(1, &hero_bg, &[]);
            let emission_pipeline = if fixed { &emit_loop } else { &emit };
            for (enabled, pipeline) in [(do_absorb, &absorb), (do_emit, emission_pipeline)] {
                if !enabled {
                    continue;
                }
                pass.set_pipeline(pipeline);
                for id in if reverse { [1, 0] } else { [0, 1] } {
                    pass.draw(0..3, id..id + 1);
                }
            }
        }
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &target,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(256),
                    rows_per_image: Some(1),
                },
            },
            size,
        );
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
        let rgb = std::array::from_fn::<_, 3, _>(|channel| {
            let h = u16::from_le_bytes([bytes[channel * 2], bytes[channel * 2 + 1]]);
            assert_eq!(h & 0x8000, 0, "negative HDR sample");
            let exponent = (h >> 10) & 31;
            assert_ne!(exponent, 31, "non-finite HDR sample");
            let mantissa = f32::from(h & 1023);
            if exponent == 0 {
                mantissa * 2.0_f32.powi(-24)
            } else {
                (1.0 + mantissa / 1024.0) * 2.0_f32.powi(i32::from(exponent) - 15)
            }
        });
        drop(bytes);
        readback.unmap();
        rgb
    };
    let background = [0.9, 0.6, 0.3];
    let attenuated = render(background, true, false, false, false);
    let emission = render([0.0; 3], false, true, false, false);
    let combined = render(background, true, true, false, false);
    let reversed = render(background, true, true, true, false);
    assert!(
        attenuated[0] < 0.85,
        "fixture must contain visible cool absorption: {attenuated:?}"
    );
    assert!(
        emission[0] > 0.001,
        "fixture must contain luminous crossing strands: {emission:?}"
    );
    for channel in 0..3 {
        assert!(
            (combined[channel] - reversed[channel]).abs() < 0.0015,
            "submission order changes overlapping plasma: {combined:?} vs {reversed:?}"
        );
        assert!(
            (combined[channel] - attenuated[channel] - emission[channel]).abs() < 0.0015,
            "emission must add after absorption without attenuating earlier strands"
        );
    }
    // Specialization must preserve the real fragment's heating/rain output,
    // not merely compile. Compare its actual HDR result through the same rays,
    // columns, uniforms and blending across the event's distinct phases.
    for time in [0.0, 14.0, 30.0, 44.0, 59.9] {
        let mut sample = frame;
        sample.scale_params[3] = time;
        gpu.queue
            .write_buffer(&frame_buffer, 0, bytemuck::bytes_of(&sample));
        let reference = render(background, true, true, false, false);
        let specialized = render(background, true, true, false, true);
        for channel in 0..3 {
            assert!(
                (reference[channel] - specialized[channel]).abs()
                    <= (reference[channel].abs() * 0.001).max(2e-6),
                "family specialization changed time{time} channel{channel}: {reference:?} vs {specialized:?}"
            );
        }
    }
}
