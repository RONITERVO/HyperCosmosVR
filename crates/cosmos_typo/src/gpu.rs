use std::fs;
use std::num::NonZeroU32;
use std::path::Path;

use bytemuck::{Pod, Zeroable};

use crate::atlas::MsdfAtlas;
use crate::error::TypoError;

/// Size in bytes of `@group(0) @binding(0)` `FrameUniforms` (`SHADER_CONTRACTS.md` §2).
pub const FRAME_UNIFORMS_BYTES: u64 = 432;
/// Size in bytes of `@group(0) @binding(1)` `AudioReactiveUniforms` (`SHADER_CONTRACTS.md` §2).
pub const AUDIO_REACTIVE_UNIFORMS_BYTES: u64 = 64;

/// `@group(1) @binding(0)` uniforms for `msdf_text.wgsl` (`32` bytes).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Pod, Zeroable)]
pub struct TextPassUniforms {
    /// `x`: `global_alpha [0..1]`, `y`: `edge_softness_scale` (default `1.0`),
    /// `z`: `nib_glow_gain` (default `1.35`), `w`: `depth_bias_scale` (default `1.0`).
    pub params_a: [f32; 4],
    /// `rgb`: linear HDR live-ink nib accent (`[0,0,0]` auto-uses `u_frame.region_accent`),
    /// `w`: `halo_scale` (default `1.0`).
    pub nib_color: [f32; 4],
}

impl Default for TextPassUniforms {
    fn default() -> Self {
        Self {
            params_a: [1.0, 1.0, 1.35, 1.0],
            nib_color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

/// `@group(1) @binding(1)` per-glyph quad instance (`112` bytes = `7 x vec4<f32>`).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Pod, Zeroable)]
pub struct GlyphInstance {
    /// `xyz`: camera-relative world center (meters), `w`: anti-z-fighting pull toward eye (meters).
    pub pos_center: [f32; 4],
    /// `xyz`: world half-right vector (meters), `w`: atlas MSDF pixel range (e.g. `8.0`).
    pub right_vec: [f32; 4],
    /// `xyz`: world half-up vector (meters), `w`: halo SDF thickness (e.g. `0.22`).
    pub up_vec: [f32; 4],
    /// `u_left, v_top, u_right, v_bottom` (GPU top-down convention: `v=0` top, `v=1` bottom).
    pub uv_rect: [f32; 4],
    /// `rgb`: linear HDR fill color, `a`: opacity `[0..1]`.
    pub color_hdr: [f32; 4],
    /// `rgb`: linear HDR obsidian outline (e.g. `[0.002, 0.003, 0.006]`), `a`: halo opacity `[0..1]`.
    pub halo_color: [f32; 4],
    /// `x`: `glyph_t0 [0..1]`, `y`: `glyph_t1 [0..1]`,
    /// `z`: `reveal_cursor` (`>=1.5` static, `<-0.5` hidden, `0..1.2` SyncVoice sweep),
    /// `w`: mode (`0=spatial anchor, 1=live-ink active, 2=Spanish ghost ribbon, 3=remembered muted, 4=dwell-focused`).
    pub ink_params: [f32; 4],
}

/// `@group(1) @binding(2)` glowing 3D leader filament connecting narration block to anchor (`48` bytes).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Pod, Zeroable)]
pub struct TetherInstance {
    /// `xyz`: start world position (camera-relative meters), `w`: half_width (meters).
    pub p0_width: [f32; 4],
    /// `xyz`: end world position (camera-relative meters), `w`: opacity `[0..1]`.
    pub p1_alpha: [f32; 4],
    /// `rgb`: linear HDR filament color, `w`: end_taper `[0..1]`.
    pub color_hdr: [f32; 4],
}

const _: () = assert!(std::mem::size_of::<TextPassUniforms>() == 32);
const _: () = assert!(std::mem::size_of::<GlyphInstance>() == 112);
const _: () = assert!(std::mem::size_of::<TetherInstance>() == 48);

/// Pipeline target configuration for [`MsdfTextRenderer`].
#[derive(Clone, Copy, Debug)]
pub struct MsdfPipelineConfig {
    pub color_format: wgpu::TextureFormat,
    pub depth_format: Option<wgpu::TextureFormat>,
    pub enable_stereo_multiview: bool,
    pub sample_count: u32,
}

impl Default for MsdfPipelineConfig {
    fn default() -> Self {
        Self {
            color_format: wgpu::TextureFormat::Rgba16Float,
            depth_format: Some(wgpu::TextureFormat::Depth32Float),
            enable_stereo_multiview: true,
            sample_count: 1,
        }
    }
}

/// Creates a compatible shared `@group(0)` bind group layout (`FrameUniforms` 432B + `AudioReactiveUniforms` 64B)
/// matching `shaders/common_oled.wgsl` and `SHADER_CONTRACTS.md` §2.
pub fn create_shared_group0_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
    device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some("cosmos_shared_group0_layout"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT | wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU32::new(FRAME_UNIFORMS_BYTES as u32).map(Into::into),
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT | wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: NonZeroU32::new(AUDIO_REACTIVE_UNIFORMS_BYTES as u32)
                        .map(Into::into),
                },
                count: None,
            },
        ],
    })
}

/// Compiles `common_oled.wgsl` prepended verbatim before `msdf_text.wgsl` into one `wgpu::ShaderModule`.
pub fn compile_msdf_shader_module(
    device: &wgpu::Device,
    common_oled_wgsl: &str,
    msdf_text_wgsl: &str,
) -> wgpu::ShaderModule {
    let combined = format!("{common_oled_wgsl}\n{msdf_text_wgsl}");
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("cosmos_msdf_text_shader"),
        source: wgpu::ShaderSource::Wgsl(combined.into()),
    })
}

/// GPU uploader, MSDF atlas texture (`Rgba8Unorm`), and Mono + Stereo Multiview pipelines.
pub struct MsdfTextRenderer {
    group1_layout: wgpu::BindGroupLayout,
    text_bind_group: wgpu::BindGroup,
    uniform_buf: wgpu::Buffer,
    glyph_buf: wgpu::Buffer,
    tether_buf: wgpu::Buffer,
    glyph_capacity: usize,
    tether_capacity: usize,
    glyph_count: u32,
    tether_count: u32,
    _atlas_texture: wgpu::Texture,
    atlas_view: wgpu::TextureView,
    atlas_sampler: wgpu::Sampler,
    tether_mono_pipeline: wgpu::RenderPipeline,
    glyph_mono_pipeline: wgpu::RenderPipeline,
    tether_stereo_pipeline: Option<wgpu::RenderPipeline>,
    glyph_stereo_pipeline: Option<wgpu::RenderPipeline>,
}

impl MsdfTextRenderer {
    /// Loads `common_oled.wgsl` and `msdf_text.wgsl` from `shaders_dir` and constructs GPU pipelines + atlas texture.
    pub fn from_shaders_dir(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        atlas: &MsdfAtlas,
        shaders_dir: impl AsRef<Path>,
        shared_group0_layout: &wgpu::BindGroupLayout,
        config: MsdfPipelineConfig,
    ) -> Result<Self, TypoError> {
        let dir = shaders_dir.as_ref();
        let common_path = dir.join("common_oled.wgsl");
        let msdf_path = dir.join("msdf_text.wgsl");

        let common_src = fs::read_to_string(&common_path).map_err(|e| TypoError::ShaderLoad {
            path: common_path.display().to_string(),
            reason: e.to_string(),
        })?;
        let msdf_src = fs::read_to_string(&msdf_path).map_err(|e| TypoError::ShaderLoad {
            path: msdf_path.display().to_string(),
            reason: e.to_string(),
        })?;

        Ok(Self::from_wgsl_sources(
            device,
            queue,
            atlas,
            &common_src,
            &msdf_src,
            shared_group0_layout,
            config,
        ))
    }

    /// Constructs GPU pipelines and uploads the `Rgba8Unorm` (linear, non-sRGB) MSDF atlas texture.
    pub fn from_wgsl_sources(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        atlas: &MsdfAtlas,
        common_oled_wgsl: &str,
        msdf_text_wgsl: &str,
        shared_group0_layout: &wgpu::BindGroupLayout,
        config: MsdfPipelineConfig,
    ) -> Self {
        let shader = compile_msdf_shader_module(device, common_oled_wgsl, msdf_text_wgsl);

        let group1_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cosmos_typo_group1_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU32::new(
                            std::mem::size_of::<TextPassUniforms>() as u32,
                        )
                        .map(Into::into),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU32::new(
                            std::mem::size_of::<GlyphInstance>() as u32,
                        )
                        .map(Into::into),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU32::new(
                            std::mem::size_of::<TetherInstance>() as u32,
                        )
                        .map(Into::into),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cosmos_typo_pipeline_layout"),
            bind_group_layouts: &[Some(shared_group0_layout), Some(&group1_layout)],
            immediate_size: 0,
        });

        let atlas_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("cosmos_msdf_atlas_rgba8unorm"),
            size: wgpu::Extent3d {
                width: atlas.width(),
                height: atlas.height(),
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &atlas_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            atlas.rgba8_pixels(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(atlas.width() * 4),
                rows_per_image: Some(atlas.height()),
            },
            wgpu::Extent3d {
                width: atlas.width(),
                height: atlas.height(),
                depth_or_array_layers: 1,
            },
        );

        let atlas_view = atlas_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let atlas_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("cosmos_msdf_linear_sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("cosmos_typo_uniforms"),
            size: std::mem::size_of::<TextPassUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let default_u = TextPassUniforms::default();
        queue.write_buffer(&uniform_buf, 0, bytemuck::bytes_of(&default_u));

        let glyph_capacity = 256usize;
        let tether_capacity = 64usize;
        let glyph_buf = create_instance_buffer::<GlyphInstance>(
            device,
            "cosmos_typo_glyph_instances",
            glyph_capacity,
        );
        let tether_buf = create_instance_buffer::<TetherInstance>(
            device,
            "cosmos_typo_tether_instances",
            tether_capacity,
        );

        let text_bind_group = create_group1_bind_group(
            device,
            &group1_layout,
            &uniform_buf,
            &glyph_buf,
            &tether_buf,
            &atlas_view,
            &atlas_sampler,
        );

        let tether_mono_pipeline = build_render_pipeline(
            device,
            &pipeline_layout,
            &shader,
            "vs_tether_main",
            "fs_tether_main",
            "cosmos_tether_mono_pipeline",
            config,
            None,
        );
        let glyph_mono_pipeline = build_render_pipeline(
            device,
            &pipeline_layout,
            &shader,
            "vs_glyph_main",
            "fs_glyph_main",
            "cosmos_glyph_mono_pipeline",
            config,
            None,
        );

        let (tether_stereo_pipeline, glyph_stereo_pipeline) =
            if config.enable_stereo_multiview && device.features().contains(wgpu::Features::MULTIVIEW)
            {
                let mask = NonZeroU32::new(0b11);
                (
                    Some(build_render_pipeline(
                        device,
                        &pipeline_layout,
                        &shader,
                        "vs_tether_stereo",
                        "fs_tether_main",
                        "cosmos_tether_stereo_pipeline",
                        config,
                        mask,
                    )),
                    Some(build_render_pipeline(
                        device,
                        &pipeline_layout,
                        &shader,
                        "vs_glyph_stereo",
                        "fs_glyph_main",
                        "cosmos_glyph_stereo_pipeline",
                        config,
                        mask,
                    )),
                )
            } else {
                (None, None)
            };

        Self {
            group1_layout,
            text_bind_group,
            uniform_buf,
            glyph_buf,
            tether_buf,
            glyph_capacity,
            tether_capacity,
            glyph_count: 0,
            tether_count: 0,
            _atlas_texture: atlas_texture,
            atlas_view,
            atlas_sampler,
            tether_mono_pipeline,
            glyph_mono_pipeline,
            tether_stereo_pipeline,
            glyph_stereo_pipeline,
        }
    }

    #[inline]
    pub fn group1_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.group1_layout
    }

    #[inline]
    pub const fn glyph_count(&self) -> u32 {
        self.glyph_count
    }

    #[inline]
    pub const fn tether_count(&self) -> u32 {
        self.tether_count
    }

    #[inline]
    pub const fn has_stereo_pipelines(&self) -> bool {
        self.glyph_stereo_pipeline.is_some() && self.tether_stereo_pipeline.is_some()
    }

    /// Uploads `TextPassUniforms`, `GlyphInstance[]`, and `TetherInstance[]` for the current frame,
    /// growing storage buffers seamlessly when instance counts exceed capacity.
    pub fn upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        uniforms: &TextPassUniforms,
        glyphs: &[GlyphInstance],
        tethers: &[TetherInstance],
    ) {
        queue.write_buffer(&self.uniform_buf, 0, bytemuck::bytes_of(uniforms));

        let mut rebind = false;
        if glyphs.len() > self.glyph_capacity {
            self.glyph_capacity = glyphs.len().next_power_of_two();
            self.glyph_buf = create_instance_buffer::<GlyphInstance>(
                device,
                "cosmos_typo_glyph_instances",
                self.glyph_capacity,
            );
            rebind = true;
        }
        if tethers.len() > self.tether_capacity {
            self.tether_capacity = tethers.len().next_power_of_two();
            self.tether_buf = create_instance_buffer::<TetherInstance>(
                device,
                "cosmos_typo_tether_instances",
                self.tether_capacity,
            );
            rebind = true;
        }

        if rebind {
            self.text_bind_group = create_group1_bind_group(
                device,
                &self.group1_layout,
                &self.uniform_buf,
                &self.glyph_buf,
                &self.tether_buf,
                &self.atlas_view,
                &self.atlas_sampler,
            );
        }

        if !glyphs.is_empty() {
            queue.write_buffer(&self.glyph_buf, 0, bytemuck::cast_slice(glyphs));
        }
        if !tethers.is_empty() {
            queue.write_buffer(&self.tether_buf, 0, bytemuck::cast_slice(tethers));
        }

        self.glyph_count = glyphs.len() as u32;
        self.tether_count = tethers.len() as u32;
    }

    /// Draws glowing leader filaments then 3D MSDF glyphs inside an active `Rgba16Float` HDR render pass.
    ///
    /// Pass `shared_group0 = Some(bg)` if `@group(0)` is not already bound on `pass`.
    pub fn render(
        &self,
        pass: &mut wgpu::RenderPass<'_>,
        shared_group0: Option<&wgpu::BindGroup>,
        is_stereo: bool,
    ) {
        if self.tether_count == 0 && self.glyph_count == 0 {
            return;
        }
        if let Some(bg0) = shared_group0 {
            pass.set_bind_group(0, bg0, &[]);
        }
        pass.set_bind_group(1, &self.text_bind_group, &[]);

        if self.tether_count > 0 {
            let pipeline = if is_stereo {
                self.tether_stereo_pipeline
                    .as_ref()
                    .unwrap_or(&self.tether_mono_pipeline)
            } else {
                &self.tether_mono_pipeline
            };
            pass.set_pipeline(pipeline);
            pass.draw(0..6, 0..self.tether_count);
        }

        if self.glyph_count > 0 {
            let pipeline = if is_stereo {
                self.glyph_stereo_pipeline
                    .as_ref()
                    .unwrap_or(&self.glyph_mono_pipeline)
            } else {
                &self.glyph_mono_pipeline
            };
            pass.set_pipeline(pipeline);
            pass.draw(0..6, 0..self.glyph_count);
        }
    }
}

fn create_instance_buffer<T>(
    device: &wgpu::Device,
    label: &str,
    capacity: usize,
) -> wgpu::Buffer {
    let cap = capacity.max(1);
    device.create_buffer(&wgpu::BufferDescriptor {
        label: Some(label),
        size: (cap * std::mem::size_of::<T>()) as u64,
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    })
}

fn create_group1_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    uniform_buf: &wgpu::Buffer,
    glyph_buf: &wgpu::Buffer,
    tether_buf: &wgpu::Buffer,
    atlas_view: &wgpu::TextureView,
    atlas_sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("cosmos_typo_group1_bind_group"),
        layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: glyph_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: tether_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(atlas_view),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::Sampler(atlas_sampler),
            },
        ],
    })
}

#[allow(clippy::too_many_arguments)]
fn build_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    vs_entry: &str,
    fs_entry: &str,
    label: &str,
    config: MsdfPipelineConfig,
    multiview_mask: Option<NonZeroU32>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some(vs_entry),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            unclipped_depth: false,
            polygon_mode: wgpu::PolygonMode::Fill,
            conservative: false,
        },
        depth_stencil: config.depth_format.map(|format| wgpu::DepthStencilState {
            format,
            depth_write_enabled: Some(false),
            depth_compare: Some(wgpu::CompareFunction::GreaterEqual),
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: config.sample_count.max(1),
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(fs_entry),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: config.color_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask,
        cache: None,
    })
}
