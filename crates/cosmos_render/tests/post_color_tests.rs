//! Exercise the shipping final fragment shader without compiling the hero pipeline.
use cosmos_render::{FrameUniforms, GpuContext, OutputEncodeMode, PostPassUniforms};
use wgpu::util::DeviceExt;

#[test]
fn final_post_preserves_agx_transfer_oled_black_and_linear_hdr() {
    let gpu = cosmos_render::init_for_dx12_luid(None).expect("DX12 post-color device");
    let shader = gpu
        .device
        .create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shipping OLED final post shader"),
            source: wgpu::ShaderSource::Wgsl(
                concat!(
                    include_str!("../../../shaders/common_oled.wgsl"),
                    "\n",
                    include_str!("../../../shaders/post_bloom_agx.wgsl")
                )
                .into(),
            ),
        });
    let hardware = FinalPost::new(&gpu, &shader, wgpu::TextureFormat::Rgba8UnormSrgb);
    let manual = FinalPost::new(&gpu, &shader, wgpu::TextureFormat::Rgba8Unorm);
    let hdr = FinalPost::new(&gpu, &shader, wgpu::TextureFormat::Rgba16Float);

    // Independent neutral-axis reference points, evaluated from Filament's
    // seventh-order AgX NONE fit, its 2.2 EOTF, then the IEC sRGB OETF:
    // https://github.com/google/filament/blob/main/filament/src/ToneMapper.cpp
    // AgxToneMapper::operator(), agxDefaultContrastApprox; checked 2026-11-15.
    // Our sixth-order approximation differs by <=2 codes here. A 3-code budget
    // includes that fit error, half-float scene storage, and 8-bit quantization.
    // These fixed values deliberately do not reimplement the shipping polynomial.
    // Missing the EOTF makes 18% gray ~187 instead of ~128 and fails decisively.
    for (radiance, expected) in [
        (0.01, 17_u8),
        (0.04, 61),
        (0.18, 128),
        (1.0, 201),
        (4.0, 237),
    ] {
        let scene = [radiance; 3];
        let a = hardware.render(&gpu, scene, 0.0, 1.0);
        let b = manual.render(&gpu, scene, 0.0, 1.0);
        for channel in 0..3 {
            assert!(
                a[channel].abs_diff(expected) <= 3 && b[channel].abs_diff(expected) <= 3,
                "AgX reference for {radiance}: hardware={a:?}, manual={b:?}, expected {expected} +/-3"
            );
            assert!(
                a[channel].abs_diff(b[channel]) <= 1,
                "output transfer paths disagree: {a:?} / {b:?}"
            );
        }
        assert!(
            a[..3].iter().max().unwrap() - a[..3].iter().min().unwrap() <= 1,
            "neutral cast: {a:?}"
        );
        assert_eq!(a[3], 255);
        assert_eq!(b[3], 255);
    }

    // Colored patches catch channel-order errors that a neutral ramp cannot.
    for (scene, brightest, darkest) in [
        ([1.0, 0.18, 0.04], 0, 2),
        ([0.04, 1.0, 0.18], 1, 0),
        ([0.18, 0.04, 1.0], 2, 1),
    ] {
        let a = hardware.render(&gpu, scene, 0.0, 1.0);
        let b = manual.render(&gpu, scene, 0.0, 1.0);
        for channel in 0..3 {
            assert!(
                a[channel].abs_diff(b[channel]) <= 1,
                "colored transfer mismatch: {scene:?} -> {a:?} / {b:?}"
            );
        }
        assert!(
            a[brightest] > a[darkest] + 40,
            "lost channel identity: {scene:?} -> {a:?}"
        );
    }

    // White noise texel + enabled dither would lift black without the early gate.
    for scene in [[0.0; 3], [0.0002; 3]] {
        assert_eq!(hardware.render(&gpu, scene, 1.0, 1.0), [0, 0, 0, 255]);
        assert_eq!(manual.render(&gpu, scene, 1.0, 1.0), [0, 0, 0, 255]);
        assert_eq!(
            hdr.render(&gpu, scene, 1.0, 1.0),
            half_bytes([0, 0, 0, 0x3c00])
        );
    }

    // Exactly representable binary16 RGB values: HDR must bypass AgX and sRGB,
    // preserve radiance above 1, and still apply the authored exposure once.
    assert_eq!(
        hdr.render(&gpu, [0.25, 0.5, 4.0], 0.0, 1.0),
        half_bytes([0x3400, 0x3800, 0x4400, 0x3c00])
    );
    assert_eq!(
        hdr.render(&gpu, [0.25, 0.5, 4.0], 0.0, 2.0),
        half_bytes([0x3800, 0x3c00, 0x4800, 0x3c00])
    );
}

fn half_bytes(values: [u16; 4]) -> Vec<u8> {
    values.into_iter().flat_map(u16::to_le_bytes).collect()
}

struct FinalPost {
    pipeline: wgpu::RenderPipeline,
    frame: wgpu::Buffer,
    frame_bg: wgpu::BindGroup,
    post_bg: wgpu::BindGroup,
    source_rtv: wgpu::TextureView,
    bloom_rtv: wgpu::TextureView,
    output: wgpu::Texture,
    output_rtv: wgpu::TextureView,
    bytes_per_pixel: usize,
}

impl FinalPost {
    fn new(gpu: &GpuContext, shader: &wgpu::ShaderModule, format: wgpu::TextureFormat) -> Self {
        let device = &gpu.device;
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("isolated production fs_final_oled_agx"),
            layout: None,
            vertex: wgpu::VertexState {
                module: shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: shader,
                entry_point: Some("fs_final_oled_agx"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });
        let uniform = |label, bytes: &[u8]| {
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: bytes,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            })
        };
        let frame = uniform(
            "post test frame",
            bytemuck::bytes_of(&FrameUniforms::default()),
        );
        let post = uniform(
            "post test grade",
            bytemuck::bytes_of(&PostPassUniforms {
                texel_scale: [1.0, 1.0, 1.0, 0.0],
                grade_params: [
                    OutputEncodeMode::from_texture_format(format).as_shader_f32(),
                    0.0,
                    0.25,
                    1e-5,
                ],
            }),
        );
        // Auto layouts use only resources reachable from the production entry
        // point: unused audio and reserved point sampler do not need stand-ins.
        let frame_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("post test frame"),
            layout: &pipeline.get_bind_group_layout(0),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: frame.as_entire_binding(),
            }],
        });
        let source = pixel_texture(device, wgpu::TextureFormat::Rgba16Float);
        let bloom = pixel_texture(device, wgpu::TextureFormat::Rgba16Float);
        let source_srv = source.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        let bloom_srv = bloom.create_view(&wgpu::TextureViewDescriptor {
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            ..Default::default()
        });
        let noise = pixel_texture(device, wgpu::TextureFormat::Rgba8Unorm);
        gpu.queue.write_texture(
            noise.as_image_copy(),
            &[255; 4],
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4),
                rows_per_image: Some(1),
            },
            wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        let noise_srv = noise.create_view(&Default::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            min_filter: wgpu::FilterMode::Linear,
            mag_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });
        let post_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("post test textures"),
            layout: &pipeline.get_bind_group_layout(1),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: post.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&source_srv),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&bloom_srv),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&noise_srv),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });
        let output = pixel_texture(device, format);
        Self {
            pipeline,
            frame,
            frame_bg,
            post_bg,
            source_rtv: source.create_view(&Default::default()),
            bloom_rtv: bloom.create_view(&Default::default()),
            output_rtv: output.create_view(&Default::default()),
            output,
            bytes_per_pixel: if format == wgpu::TextureFormat::Rgba16Float {
                8
            } else {
                4
            },
        }
    }

    fn render(&self, gpu: &GpuContext, rgb: [f64; 3], dither: f32, exposure: f32) -> Vec<u8> {
        let frame = FrameUniforms {
            scale_params: [0.0, 0.0, exposure, 0.0],
            post_params: [1.05, 0.0, 0.0, dither],
            viewport_params: [1.0; 4],
            ..Default::default()
        };
        gpu.queue
            .write_buffer(&self.frame, 0, bytemuck::bytes_of(&frame));
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        for (view, clear) in [
            (
                &self.source_rtv,
                wgpu::Color {
                    r: rgb[0],
                    g: rgb[1],
                    b: rgb[2],
                    a: 1.0,
                },
            ),
            (&self.bloom_rtv, wgpu::Color::BLACK),
        ] {
            drop(encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            }));
        }
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.output_rtv,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::RED),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.frame_bg, &[]);
            pass.set_bind_group(1, &self.post_bg, &[]);
            pass.draw(0..3, 0..1);
        }
        let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("post pixel readback"),
            size: 256,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        encoder.copy_texture_to_buffer(
            self.output.as_image_copy(),
            wgpu::TexelCopyBufferInfo {
                buffer: &buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(256),
                    rows_per_image: Some(1),
                },
            },
            wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
        );
        gpu.queue.submit(Some(encoder.finish()));
        let slice = buffer.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |result| {
            tx.send(result).unwrap();
        });
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .unwrap();
        rx.recv().unwrap().unwrap();
        let mapped = slice.get_mapped_range().unwrap();
        let pixel = mapped[..self.bytes_per_pixel].to_vec();
        drop(mapped);
        buffer.unmap();
        pixel
    }
}

fn pixel_texture(device: &wgpu::Device, format: wgpu::TextureFormat) -> wgpu::Texture {
    device.create_texture(&wgpu::TextureDescriptor {
        label: Some("post test pixel"),
        size: wgpu::Extent3d {
            width: 1,
            height: 1,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    })
}
