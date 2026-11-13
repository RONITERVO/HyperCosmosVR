use crate::pipeline::{HDR_SCENE_FORMAT, PipelineBank, STEREO_MULTIVIEW_MASK};
use crate::scene_pass::ScenePass;
use crate::types::{OutputEncodeMode, PostPassUniforms, PostSettings};

const BLOOM_MIPS: usize = 5;

struct BloomMipTarget {
    srv_view: wgpu::TextureView,
    rtv_view: wgpu::TextureView,
    width: u32,
    height: u32,
}

/// Tight-threshold Dual-Kawase bloom chain + optical dispersion + AgX OLED tonemapping +
/// TPDF 128x128 blue-noise dithering pass.
pub struct PostPass {
    width: u32,
    height: u32,
    eye_count: u32,

    _bloom_a: wgpu::Texture,
    _bloom_b: wgpu::Texture,
    a_mips: Vec<BloomMipTarget>,
    b_mips: Vec<BloomMipTarget>,

    u_prefilter_buf: wgpu::Buffer,
    prefilter_bg: wgpu::BindGroup,

    _down_bufs: Vec<wgpu::Buffer>,
    down_bgs: Vec<wgpu::BindGroup>,

    _up_bufs: Vec<wgpu::Buffer>,
    up_bgs: Vec<wgpu::BindGroup>,

    u_final_buf: wgpu::Buffer,
    final_bg: wgpu::BindGroup,
}

impl PostPass {
    pub fn new(device: &wgpu::Device, pipelines: &PipelineBank, scene: &ScenePass) -> Self {
        Self::build(device, pipelines, scene)
    }

    pub fn sync_to_scene(
        &mut self,
        device: &wgpu::Device,
        pipelines: &PipelineBank,
        scene: &ScenePass,
        resized: bool,
    ) {
        if resized
            || self.width != scene.width
            || self.height != scene.height
            || self.eye_count != scene.eye_count
        {
            *self = Self::build(device, pipelines, scene);
        }
    }

    fn build(device: &wgpu::Device, pipelines: &PipelineBank, scene: &ScenePass) -> Self {
        let width = scene.width.max(1);
        let height = scene.height.max(1);
        let eye_count = scene.eye_count.max(1);

        let base_w = (width / 2).max(1);
        let base_h = (height / 2).max(1);

        let (bloom_a, a_mips) =
            create_bloom_chain(device, base_w, base_h, eye_count, "bloom_chain_a");
        let (bloom_b, b_mips) =
            create_bloom_chain(device, base_w, base_h, eye_count, "bloom_chain_b");

        let zero_uniform = PostPassUniforms {
            texel_scale: [1.0 / width as f32, 1.0 / height as f32, 1.0, 0.0],
            grade_params: [0.0, 0.0, 0.25, 1e-5],
        };

        let u_prefilter_buf = make_post_ubuf(device, "u_post_prefilter", zero_uniform);
        // Bind unused b_mips[0] in t_bloom slot so a_mips[0] is never simultaneously RESOURCE and COLOR_TARGET.
        let prefilter_bg = make_post_bg(
            device,
            pipelines,
            &u_prefilter_buf,
            &scene.hdr_srv_view,
            &b_mips[0].srv_view,
            "post_prefilter_bg",
        );

        let mut down_bufs = Vec::with_capacity(BLOOM_MIPS - 1);
        let mut down_bgs = Vec::with_capacity(BLOOM_MIPS - 1);
        for k in 0..(BLOOM_MIPS - 1) {
            let src = if k % 2 == 0 {
                &a_mips[k]
            } else {
                &b_mips[k]
            };
            let u = PostPassUniforms {
                texel_scale: [1.0 / src.width as f32, 1.0 / src.height as f32, 0.85, 0.0],
                grade_params: [0.0, 0.0, 0.25, 1e-5],
            };
            let buf = make_post_ubuf(device, &format!("u_post_down_{k}"), u);
            let bg = make_post_bg(
                device,
                pipelines,
                &buf,
                &scene.hdr_srv_view,
                &src.srv_view,
                &format!("post_down_bg_{k}"),
            );
            down_bufs.push(buf);
            down_bgs.push(bg);
        }

        let mut up_bufs = Vec::with_capacity(BLOOM_MIPS - 1);
        let mut up_bgs = Vec::with_capacity(BLOOM_MIPS - 1);
        for k in (0..(BLOOM_MIPS - 1)).rev() {
            let src_mip = k + 1;
            let src = if src_mip % 2 == 0 {
                &a_mips[src_mip]
            } else {
                &b_mips[src_mip]
            };
            let u = PostPassUniforms {
                texel_scale: [1.0 / src.width as f32, 1.0 / src.height as f32, 0.85, 0.0],
                grade_params: [0.0, 0.0, 0.25, 1e-5],
            };
            let buf = make_post_ubuf(device, &format!("u_post_up_{k}"), u);
            let bg = make_post_bg(
                device,
                pipelines,
                &buf,
                &scene.hdr_srv_view,
                &src.srv_view,
                &format!("post_up_bg_{k}"),
            );
            up_bufs.push(buf);
            up_bgs.push(bg);
        }

        let u_final_buf = make_post_ubuf(device, "u_post_final", zero_uniform);
        let final_bg = make_post_bg(
            device,
            pipelines,
            &u_final_buf,
            &scene.hdr_srv_view,
            &a_mips[0].srv_view,
            "post_final_bg",
        );

        Self {
            width,
            height,
            eye_count,
            _bloom_a: bloom_a,
            _bloom_b: bloom_b,
            a_mips,
            b_mips,
            u_prefilter_buf,
            prefilter_bg,
            _down_bufs: down_bufs,
            down_bgs,
            _up_bufs: up_bufs,
            up_bgs,
            u_final_buf,
            final_bg,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn execute(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        pipelines: &PipelineBank,
        scene: &ScenePass,
        target_view: &wgpu::TextureView,
        target_format: wgpu::TextureFormat,
        post: PostSettings,
    ) {
        let is_stereo = self.eye_count > 1;
        let mv = if is_stereo {
            STEREO_MULTIVIEW_MASK
        } else {
            None
        };

        // Write prefilter uniforms (update soft knee & void floor eps dynamically)
        let prefilter_u = PostPassUniforms {
            texel_scale: [1.0 / self.width as f32, 1.0 / self.height as f32, 1.0, 0.0],
            grade_params: [
                0.0,
                post.vignette_strength,
                post.bloom_soft_knee,
                post.void_floor_eps,
            ],
        };
        queue.write_buffer(&self.u_prefilter_buf, 0, bytemuck::bytes_of(&prefilter_u));

        // 1) Bloom Prefilter -> bloom_a[0]
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bloom_prefilter"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.a_mips[0].rtv_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: mv,
            });
            rpass.set_pipeline(if is_stereo {
                &pipelines.prefilter_stereo
            } else {
                &pipelines.prefilter_mono
            });
            rpass.set_bind_group(0, &scene.shared_frame_bg, &[]);
            rpass.set_bind_group(1, &self.prefilter_bg, &[]);
            rpass.draw(0..3, 0..1);
        }

        // 2) Dual-Kawase Downsample alternating bloom_a[0,2,4] <-> bloom_b[1,3]
        let down_pipe = if is_stereo {
            &pipelines.kawase_down_stereo
        } else {
            &pipelines.kawase_down_mono
        };
        for k in 0..(BLOOM_MIPS - 1) {
            let dst = if (k + 1) % 2 == 0 {
                &self.a_mips[k + 1]
            } else {
                &self.b_mips[k + 1]
            };
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bloom_kawase_down"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &dst.rtv_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: mv,
            });
            rpass.set_pipeline(down_pipe);
            rpass.set_bind_group(0, &scene.shared_frame_bg, &[]);
            rpass.set_bind_group(1, &self.down_bgs[k], &[]);
            rpass.draw(0..3, 0..1);
        }

        // 3) Dual-Kawase Additive Upsample back into bloom_a[0] (multi-octave Gaussian glow, zero void lift)
        let up_pipe = if is_stereo {
            &pipelines.kawase_up_stereo
        } else {
            &pipelines.kawase_up_mono
        };
        for (idx, k) in (0..(BLOOM_MIPS - 1)).rev().enumerate() {
            let dst = if k % 2 == 0 {
                &self.a_mips[k]
            } else {
                &self.b_mips[k]
            };
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("bloom_kawase_up"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &dst.rtv_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: mv,
            });
            rpass.set_pipeline(up_pipe);
            rpass.set_blend_constant(wgpu::Color {
                r: 0.36,
                g: 0.36,
                b: 0.36,
                a: 1.0,
            });
            rpass.set_bind_group(0, &scene.shared_frame_bg, &[]);
            rpass.set_bind_group(1, &self.up_bgs[idx], &[]);
            rpass.draw(0..3, 0..1);
        }

        // 4) Final Composite + Edge Chromatic Dispersion + AgX OLED Tonemap + TPDF Blue-Noise Dither
        let encode_mode = OutputEncodeMode::from_texture_format(target_format).as_shader_f32();
        let final_u = PostPassUniforms {
            texel_scale: [1.0 / self.width as f32, 1.0 / self.height as f32, 1.0, 0.0],
            grade_params: [
                encode_mode,
                post.vignette_strength,
                post.bloom_soft_knee,
                post.void_floor_eps,
            ],
        };
        queue.write_buffer(&self.u_final_buf, 0, bytemuck::bytes_of(&final_u));

        let final_pipe = pipelines.final_pipeline(device, target_format, is_stereo);
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("final_oled_agx"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: mv,
        });
        rpass.set_pipeline(&final_pipe);
        rpass.set_bind_group(0, &scene.shared_frame_bg, &[]);
        rpass.set_bind_group(1, &self.final_bg, &[]);
        rpass.draw(0..3, 0..1);
    }
}

fn create_bloom_chain(
    device: &wgpu::Device,
    base_w: u32,
    base_h: u32,
    eye_count: u32,
    label: &str,
) -> (wgpu::Texture, Vec<BloomMipTarget>) {
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: base_w,
            height: base_h,
            depth_or_array_layers: eye_count,
        },
        mip_level_count: BLOOM_MIPS as u32,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: HDR_SCENE_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    let rtv_dim = if eye_count > 1 {
        wgpu::TextureViewDimension::D2Array
    } else {
        wgpu::TextureViewDimension::D2
    };

    let mut mips = Vec::with_capacity(BLOOM_MIPS);
    for mip in 0..BLOOM_MIPS {
        let w = (base_w >> mip).max(1);
        let h = (base_h >> mip).max(1);
        let srv_view = tex.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("{label}_srv_m{mip}")),
            format: Some(HDR_SCENE_FORMAT),
            dimension: Some(wgpu::TextureViewDimension::D2Array),
            base_mip_level: mip as u32,
            mip_level_count: Some(1),
            base_array_layer: 0,
            array_layer_count: Some(eye_count),
            ..Default::default()
        });
        let rtv_view = tex.create_view(&wgpu::TextureViewDescriptor {
            label: Some(&format!("{label}_rtv_m{mip}")),
            format: Some(HDR_SCENE_FORMAT),
            dimension: Some(rtv_dim),
            base_mip_level: mip as u32,
            mip_level_count: Some(1),
            base_array_layer: 0,
            array_layer_count: Some(eye_count),
            ..Default::default()
        });
        mips.push(BloomMipTarget {
            srv_view,
            rtv_view,
            width: w,
            height: h,
        });
    }
    (tex, mips)
}

fn make_post_ubuf(device: &wgpu::Device, label: &str, val: PostPassUniforms) -> wgpu::Buffer {
    use wgpu::util::DeviceExt;
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some(label),
        contents: bytemuck::bytes_of(&val),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    })
}

fn make_post_bg(
    device: &wgpu::Device,
    pipelines: &PipelineBank,
    u_post_buf: &wgpu::Buffer,
    hdr_srv: &wgpu::TextureView,
    bloom_src_srv: &wgpu::TextureView,
    label: &str,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout: &pipelines.post_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: u_post_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(hdr_srv),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(bloom_src_srv),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(&pipelines.blue_noise_view),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::Sampler(&pipelines.linear_clamp_samp),
            },
            wgpu::BindGroupEntry {
                binding: 5,
                resource: wgpu::BindingResource::Sampler(&pipelines.point_clamp_samp),
            },
        ],
    })
}
