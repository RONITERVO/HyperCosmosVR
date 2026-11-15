use std::collections::HashMap;
use std::num::{NonZeroU32, NonZeroU64};
use std::sync::Mutex;

use crate::RenderError;
use crate::types::{
    AudioReactiveUniforms, FarShellUniforms, FrameUniforms, HeroVertex, HeroVolumeInstance,
    PostPassUniforms, SimDispatchUniforms, SimParticle, SpriteBatchUniforms, SpriteInstance,
};

const COMMON_OLED_WGSL: &str = include_str!("../../../shaders/common_oled.wgsl");
const COMPUTE_UNIVERSE_WGSL: &str = include_str!("../../../shaders/compute_universe.wgsl");
const FAR_SHELL_WGSL: &str = include_str!("../../../shaders/far_shell.wgsl");
const DOC_COMMON_WGSL: &str = include_str!("../../../shaders/doc_common.wgsl");
const HERO_DOC_SUN_WGSL: &str = include_str!("../../../shaders/hero_doc_sun.wgsl");
const HERO_SOLAR_INTERIOR_WGSL: &str = include_str!("../../../shaders/hero_solar_interior.wgsl");
const HERO_SOLAR_CORONA_WGSL: &str = include_str!("../../../shaders/hero_solar_corona.wgsl");
const SOLAR_PLASMA_WGSL: &str = include_str!("../../../shaders/solar_plasma.wgsl");
const SOLAR_HERO_WGSL: &str = include_str!("../../../shaders/solar_hero.wgsl");
const HERO_DOC_COSMOS_WGSL: &str = include_str!("../../../shaders/hero_doc_cosmos.wgsl");
const HERO_DOC_MATTER_MIND_WGSL: &str = include_str!("../../../shaders/hero_doc_matter_mind.wgsl");
const HERO_DOC_CIV_HORIZON_WGSL: &str =
    include_str!("../../../shaders/hero_doc_civilization_horizon.wgsl");
const HERO_VOLUMETRIC_WGSL: &str = include_str!("../../../shaders/hero_volumetric.wgsl");
const INSTANCED_SPRITES_WGSL: &str = include_str!("../../../shaders/instanced_sprites.wgsl");
const POST_BLOOM_AGX_WGSL: &str = include_str!("../../../shaders/post_bloom_agx.wgsl");

const BLUE_NOISE_PNG: &[u8] = include_bytes!("../../../assets/textures/blue_noise_128.png");

pub const HDR_SCENE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;
pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;
pub const STEREO_MULTIVIEW_MASK: Option<NonZeroU32> = NonZeroU32::new(0b11);

fn concat_with_prelude(pass_src: &str) -> String {
    let mut out = String::with_capacity(COMMON_OLED_WGSL.len() + pass_src.len() + 2);
    out.push_str(COMMON_OLED_WGSL);
    out.push('\n');
    out.push_str(pass_src);
    out
}

fn concat_hero_shaders() -> String {
    let parts = [
        COMMON_OLED_WGSL,
        DOC_COMMON_WGSL,
        HERO_DOC_COSMOS_WGSL,
        HERO_DOC_MATTER_MIND_WGSL,
        HERO_DOC_CIV_HORIZON_WGSL,
        HERO_VOLUMETRIC_WGSL,
    ];
    let total_len: usize = parts.iter().map(|s| s.len() + 1).sum();
    let mut out = String::with_capacity(total_len);
    for part in parts {
        out.push_str(part);
        out.push('\n');
    }
    out
}

fn concat_solar_plasma_shaders() -> String {
    [
        COMMON_OLED_WGSL,
        DOC_COMMON_WGSL,
        HERO_DOC_SUN_WGSL,
        HERO_SOLAR_INTERIOR_WGSL,
        SOLAR_PLASMA_WGSL,
    ]
    .join("\n")
}

fn concat_solar_hero_shaders() -> String {
    [COMMON_OLED_WGSL, DOC_COMMON_WGSL, HERO_DOC_SUN_WGSL,
        HERO_SOLAR_INTERIOR_WGSL, HERO_SOLAR_CORONA_WGSL, SOLAR_HERO_WGSL].join("\n")
}

fn create_shader(device: &wgpu::Device, label: &str, src: &str) -> wgpu::ShaderModule {
    let full = concat_with_prelude(src);
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some(label),
        source: wgpu::ShaderSource::Wgsl(full.into()),
    })
}

/// Shared `@group(0)` layout (`u_frame` + `u_audio`), pass layouts, compiled compute & render
/// pipelines, and shared 128x128 blue-noise texture.
pub struct PipelineBank {
    pub shared_frame_bgl: wgpu::BindGroupLayout,
    pub compute_bgl: wgpu::BindGroupLayout,
    pub field_grid_bgl: wgpu::BindGroupLayout,
    pub far_bgl: wgpu::BindGroupLayout,
    pub hero_bgl: wgpu::BindGroupLayout,
    pub sprite_bgl: wgpu::BindGroupLayout,
    pub post_bgl: wgpu::BindGroupLayout,

    pub advect_pipeline: wgpu::ComputePipeline,
    pub field_pipeline: wgpu::ComputePipeline,

    pub far_mono: wgpu::RenderPipeline,
    pub far_stereo: wgpu::RenderPipeline,

    pub hero_surface_mono: wgpu::RenderPipeline,
    pub hero_surface_stereo: wgpu::RenderPipeline,
    pub hero_volume_mono: wgpu::RenderPipeline,
    pub hero_volume_stereo: wgpu::RenderPipeline,
    pub solar_surface_mono: wgpu::RenderPipeline,
    pub solar_surface_stereo: wgpu::RenderPipeline,
    pub solar_corona_mono: wgpu::RenderPipeline,
    pub solar_corona_stereo: wgpu::RenderPipeline,
    pub solar_plasma_mono: wgpu::RenderPipeline,
    pub solar_plasma_stereo: wgpu::RenderPipeline,
    pub solar_loop_emission_mono: wgpu::RenderPipeline,
    pub solar_loop_emission_stereo: wgpu::RenderPipeline,
    pub solar_absorption_mono: wgpu::RenderPipeline,
    pub solar_absorption_stereo: wgpu::RenderPipeline,

    pub particle_mono: wgpu::RenderPipeline,
    pub particle_stereo: wgpu::RenderPipeline,
    pub sprite_mono: wgpu::RenderPipeline,
    pub sprite_stereo: wgpu::RenderPipeline,

    pub post_shader: wgpu::ShaderModule,
    pub post_pipeline_layout: wgpu::PipelineLayout,
    pub prefilter_mono: wgpu::RenderPipeline,
    pub prefilter_stereo: wgpu::RenderPipeline,
    pub kawase_down_mono: wgpu::RenderPipeline,
    pub kawase_down_stereo: wgpu::RenderPipeline,
    pub kawase_up_mono: wgpu::RenderPipeline,
    pub kawase_up_stereo: wgpu::RenderPipeline,
    final_pipelines: Mutex<HashMap<(wgpu::TextureFormat, bool), wgpu::RenderPipeline>>,

    pub blue_noise_tex: wgpu::Texture,
    pub blue_noise_view: wgpu::TextureView,
    pub blue_noise_repeat_samp: wgpu::Sampler,
    pub earth_maps_tex: wgpu::Texture,
    pub earth_maps_view: wgpu::TextureView,
    pub earth_maps_samp: wgpu::Sampler,
    pub solar_noise_tex: wgpu::Texture,
    pub solar_noise_view: wgpu::TextureView,
    pub solar_noise_sampler: wgpu::Sampler,
    pub linear_clamp_samp: wgpu::Sampler,
    pub point_clamp_samp: wgpu::Sampler,
}

impl PipelineBank {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Result<Self, RenderError> {
        let shared_frame_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shared_frame_bgl (@group(0))"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT | wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(
                            std::mem::size_of::<FrameUniforms>() as u64,
                        ),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT | wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(
                            std::mem::size_of::<AudioReactiveUniforms>() as u64,
                        ),
                    },
                    count: None,
                },
            ],
        });

        let compute_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("compute_bgl (@group(1))"),
            entries: &[
                uniform_entry(0, wgpu::ShaderStages::COMPUTE, std::mem::size_of::<SimDispatchUniforms>()),
                storage_entry(1, wgpu::ShaderStages::COMPUTE, false, std::mem::size_of::<SimParticle>()),
            ],
        });

        let field_grid_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("field_grid_bgl (@group(2))"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: HDR_SCENE_FORMAT,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });

        let far_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("far_bgl (@group(1))"),
            entries: &[uniform_entry(
                0,
                wgpu::ShaderStages::VERTEX_FRAGMENT,
                std::mem::size_of::<FarShellUniforms>(),
            )],
        });

        let hero_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("hero_bgl (@group(1))"),
            entries: &[
                storage_entry(
                    0,
                    wgpu::ShaderStages::VERTEX_FRAGMENT,
                    true,
                    std::mem::size_of::<HeroVolumeInstance>(),
                ),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
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
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 6,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 7,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D3,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 8,
                    visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let sprite_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("sprite_bgl (@group(1))"),
            entries: &[
                uniform_entry(
                    0,
                    wgpu::ShaderStages::VERTEX_FRAGMENT,
                    std::mem::size_of::<SpriteBatchUniforms>(),
                ),
                storage_entry(
                    1,
                    wgpu::ShaderStages::VERTEX,
                    true,
                    std::mem::size_of::<SimParticle>(),
                ),
                storage_entry(
                    2,
                    wgpu::ShaderStages::VERTEX,
                    true,
                    std::mem::size_of::<SpriteInstance>(),
                ),
            ],
        });

        let post_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("post_bgl (@group(1))"),
            entries: &[
                uniform_entry(
                    0,
                    wgpu::ShaderStages::VERTEX_FRAGMENT,
                    std::mem::size_of::<PostPassUniforms>(),
                ),
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2Array,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
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
                wgpu::BindGroupLayoutEntry {
                    binding: 5,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
            ],
        });

        // --- Shaders & Layouts ---
        let compute_shader = create_shader(device, "compute_universe.wgsl", COMPUTE_UNIVERSE_WGSL);
        let advect_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("advect_layout"),
            bind_group_layouts: &[Some(&shared_frame_bgl), Some(&compute_bgl)],
            immediate_size: 0,
        });
        let advect_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("cs_advect_particles"),
            layout: Some(&advect_layout),
            module: &compute_shader,
            entry_point: Some("cs_advect_particles"),
            compilation_options: Default::default(),
            cache: None,
        });

        let field_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("field_layout"),
            bind_group_layouts: &[
                Some(&shared_frame_bgl),
                Some(&compute_bgl),
                Some(&field_grid_bgl),
            ],
            immediate_size: 0,
        });
        let field_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("cs_field_grid"),
            layout: Some(&field_layout),
            module: &compute_shader,
            entry_point: Some("cs_field_grid"),
            compilation_options: Default::default(),
            cache: None,
        });

        // Far Shell
        let far_shader = create_shader(device, "far_shell.wgsl", FAR_SHELL_WGSL);
        let far_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("far_layout"),
            bind_group_layouts: &[Some(&shared_frame_bgl), Some(&far_bgl)],
            immediate_size: 0,
        });
        let far_mono = build_far_pipeline(device, &far_layout, &far_shader, "vs_main", None);
        let far_stereo = build_far_pipeline(
            device,
            &far_layout,
            &far_shader,
            "vs_stereo",
            STEREO_MULTIVIEW_MASK,
        );

        // Hero Volumetric & Analytical Surfaces
        let hero_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("hero_volumetric_modular.wgsl"),
            source: wgpu::ShaderSource::Wgsl(concat_hero_shaders().into()),
        });
        let hero_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("hero_layout"),
            bind_group_layouts: &[Some(&shared_frame_bgl), Some(&hero_bgl)],
            immediate_size: 0,
        });
        let solar_plasma_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("solar_plasma_modular.wgsl"),
            source: wgpu::ShaderSource::Wgsl(concat_solar_plasma_shaders().into()),
        });
        let solar_hero_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("solar_hero_modular.wgsl"),
            source: wgpu::ShaderSource::Wgsl(concat_solar_hero_shaders().into()),
        });
        // DXC compilation dominates cold start. Independent shader families
        // compile concurrently, while each mono/stereo pair stays sequential
        // so wgpu's DX12 fragment cache avoids duplicate expensive compiles.
        let ((hero_surface_mono, hero_surface_stereo),
            (hero_volume_mono, hero_volume_stereo),
            (solar_plasma_mono, solar_plasma_stereo, solar_loop_emission_mono,
                solar_loop_emission_stereo, solar_absorption_mono, solar_absorption_stereo),
            (solar_surface_mono, solar_surface_stereo, solar_corona_mono, solar_corona_stereo)) =
        std::thread::scope(|scope| {
            let surfaces = scope.spawn(|| {
                let hero_surface_mono = build_hero_pipeline(
                    device,
                    &hero_layout,
                    &hero_shader,
                    "vs_main",
                    "fs_hero_surface",
                    true,
                    wgpu::BlendState::REPLACE,
                    None,
                );
                let hero_surface_stereo = build_hero_pipeline(
                    device,
                    &hero_layout,
                    &hero_shader,
                    "vs_stereo",
                    "fs_hero_surface",
                    true,
                    wgpu::BlendState::REPLACE,
                    STEREO_MULTIVIEW_MASK,
                );
                (hero_surface_mono, hero_surface_stereo)
            });
            let volumes = scope.spawn(|| {
                let hero_volume_mono = build_hero_pipeline(
                    device,
                    &hero_layout,
                    &hero_shader,
                    "vs_main",
                    "fs_hero_volume",
                    false,
                    wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING,
                    None,
                );
                let hero_volume_stereo = build_hero_pipeline(
                    device,
                    &hero_layout,
                    &hero_shader,
                    "vs_stereo",
                    "fs_hero_volume",
                    false,
                    wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING,
                    STEREO_MULTIVIEW_MASK,
                );
                (hero_volume_mono, hero_volume_stereo)
            });

            // Resolved field-aligned solar strands share the hero storage binding,
            // but generate their six proxy vertices directly from the segment ID.
            let plasma = scope.spawn(|| {
                let solar_plasma_mono = build_solar_plasma_pipeline(
                    device,
                    &hero_layout,
                    &solar_plasma_shader,
                    "vs_solar_plasma",
                    "fs_solar_plasma",
                    None,
                    None,
                );
                let solar_plasma_stereo = build_solar_plasma_pipeline(
                    device,
                    &hero_layout,
                    &solar_plasma_shader,
                    "vs_solar_plasma_stereo",
                    "fs_solar_plasma",
                    STEREO_MULTIVIEW_MASK,
                    None,
                );
                let solar_loop_emission_mono = build_solar_plasma_pipeline(
                    device, &hero_layout, &solar_plasma_shader,
                    "vs_solar_plasma", "fs_solar_plasma", None, Some(0),
                );
                let solar_loop_emission_stereo = build_solar_plasma_pipeline(
                    device, &hero_layout, &solar_plasma_shader,
                    "vs_solar_plasma_stereo", "fs_solar_plasma", STEREO_MULTIVIEW_MASK, Some(0),
                );
                let solar_absorption_mono = build_solar_plasma_pipeline(
                    device, &hero_layout, &solar_plasma_shader,
                    "vs_solar_absorption", "fs_solar_absorption", None, None,
                );
                let solar_absorption_stereo = build_solar_plasma_pipeline(
                    device, &hero_layout, &solar_plasma_shader,
                    "vs_solar_absorption_stereo", "fs_solar_absorption", STEREO_MULTIVIEW_MASK, None,
                );
                (solar_plasma_mono, solar_plasma_stereo, solar_loop_emission_mono,
                    solar_loop_emission_stereo, solar_absorption_mono, solar_absorption_stereo)
            });
            let solar = scope.spawn(|| {
                let surface_mono = build_hero_pipeline(device, &hero_layout, &solar_hero_shader,
                    "vs_solar_hero", "fs_solar_surface", true, wgpu::BlendState::REPLACE, None);
                let surface_stereo = build_hero_pipeline(device, &hero_layout, &solar_hero_shader,
                    "vs_solar_hero_stereo", "fs_solar_surface", true, wgpu::BlendState::REPLACE, STEREO_MULTIVIEW_MASK);
                let corona_mono = build_hero_pipeline(device, &hero_layout, &solar_hero_shader,
                    "vs_solar_hero", "fs_solar_corona", false, wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING, None);
                let corona_stereo = build_hero_pipeline(device, &hero_layout, &solar_hero_shader,
                    "vs_solar_hero_stereo", "fs_solar_corona", false, wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING, STEREO_MULTIVIEW_MASK);
                (surface_mono, surface_stereo, corona_mono, corona_stereo)
            });
            (surfaces.join().expect("surface pipeline initialization failed"),
                volumes.join().expect("volume pipeline initialization failed"),
                plasma.join().expect("solar plasma pipeline initialization failed"),
                solar.join().expect("dedicated solar pipeline initialization failed"))
        });

        // Instanced Sprites & Advected Particles
        let sprite_shader = create_shader(device, "instanced_sprites.wgsl", INSTANCED_SPRITES_WGSL);
        let sprite_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("sprite_layout"),
            bind_group_layouts: &[Some(&shared_frame_bgl), Some(&sprite_bgl)],
            immediate_size: 0,
        });
        let additive_hdr = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
        };
        let particle_mono = build_sprite_pipeline(
            device,
            &sprite_layout,
            &sprite_shader,
            "vs_particle_main",
            additive_hdr,
            None,
        );
        let particle_stereo = build_sprite_pipeline(
            device,
            &sprite_layout,
            &sprite_shader,
            "vs_particle_stereo",
            additive_hdr,
            STEREO_MULTIVIEW_MASK,
        );
        let sprite_mono = build_sprite_pipeline(
            device,
            &sprite_layout,
            &sprite_shader,
            "vs_sprite_main",
            wgpu::BlendState::ALPHA_BLENDING,
            None,
        );
        let sprite_stereo = build_sprite_pipeline(
            device,
            &sprite_layout,
            &sprite_shader,
            "vs_sprite_stereo",
            wgpu::BlendState::ALPHA_BLENDING,
            STEREO_MULTIVIEW_MASK,
        );

        // Post Bloom + AgX
        let post_shader = create_shader(device, "post_bloom_agx.wgsl", POST_BLOOM_AGX_WGSL);
        let post_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("post_layout"),
            bind_group_layouts: &[Some(&shared_frame_bgl), Some(&post_bgl)],
            immediate_size: 0,
        });

        let kawase_add = wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::Constant,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent::OVER,
        };
        let prefilter_mono = build_post_pipeline(
            device,
            &post_pipeline_layout,
            &post_shader,
            "vs_main",
            "fs_bloom_prefilter",
            HDR_SCENE_FORMAT,
            wgpu::BlendState::REPLACE,
            None,
        );
        let prefilter_stereo = build_post_pipeline(
            device,
            &post_pipeline_layout,
            &post_shader,
            "vs_stereo",
            "fs_bloom_prefilter",
            HDR_SCENE_FORMAT,
            wgpu::BlendState::REPLACE,
            STEREO_MULTIVIEW_MASK,
        );
        let kawase_down_mono = build_post_pipeline(
            device,
            &post_pipeline_layout,
            &post_shader,
            "vs_main",
            "fs_kawase_down",
            HDR_SCENE_FORMAT,
            wgpu::BlendState::REPLACE,
            None,
        );
        let kawase_down_stereo = build_post_pipeline(
            device,
            &post_pipeline_layout,
            &post_shader,
            "vs_stereo",
            "fs_kawase_down",
            HDR_SCENE_FORMAT,
            wgpu::BlendState::REPLACE,
            STEREO_MULTIVIEW_MASK,
        );
        let kawase_up_mono = build_post_pipeline(
            device,
            &post_pipeline_layout,
            &post_shader,
            "vs_main",
            "fs_kawase_up",
            HDR_SCENE_FORMAT,
            kawase_add,
            None,
        );
        let kawase_up_stereo = build_post_pipeline(
            device,
            &post_pipeline_layout,
            &post_shader,
            "vs_stereo",
            "fs_kawase_up",
            HDR_SCENE_FORMAT,
            kawase_add,
            STEREO_MULTIVIEW_MASK,
        );

        // Blue Noise 128x128 Texture
        let bn_img = image::load_from_memory_with_format(BLUE_NOISE_PNG, image::ImageFormat::Png)
            .map_err(|e| RenderError::PipelineInit(format!("blue_noise_128.png decode: {e}")))?
            .to_rgba8();
        let (bn_w, bn_h) = bn_img.dimensions();
        let blue_noise_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("blue_noise_128"),
            size: wgpu::Extent3d {
                width: bn_w,
                height: bn_h,
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
                texture: &blue_noise_tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            bn_img.as_raw(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bn_w * 4),
                rows_per_image: Some(bn_h),
            },
            wgpu::Extent3d {
                width: bn_w,
                height: bn_h,
                depth_or_array_layers: 1,
            },
        );
        let blue_noise_view = blue_noise_tex.create_view(&wgpu::TextureViewDescriptor::default());

        let blue_noise_repeat_samp = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("blue_noise_repeat"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });
        let linear_clamp_samp = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("linear_clamp"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Linear,
            ..Default::default()
        });
        let point_clamp_samp = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("point_clamp"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let (earth_maps_tex, earth_maps_view, earth_maps_samp) =
            crate::earth_textures::upload(device, queue)?;
        let (solar_noise_tex, solar_noise_view, solar_noise_sampler) =
            crate::solar_noise::create_lattice(device, queue);
        Ok(Self {
            shared_frame_bgl,
            compute_bgl,
            field_grid_bgl,
            far_bgl,
            hero_bgl,
            sprite_bgl,
            post_bgl,
            advect_pipeline,
            field_pipeline,
            far_mono,
            far_stereo,
            hero_surface_mono,
            hero_surface_stereo,
            hero_volume_mono,
            hero_volume_stereo,
            solar_surface_mono,
            solar_surface_stereo,
            solar_corona_mono,
            solar_corona_stereo,
            solar_plasma_mono,
            solar_plasma_stereo,
            solar_loop_emission_mono,
            solar_loop_emission_stereo,
            solar_absorption_mono,
            solar_absorption_stereo,
            particle_mono,
            particle_stereo,
            sprite_mono,
            sprite_stereo,
            post_shader,
            post_pipeline_layout,
            prefilter_mono,
            prefilter_stereo,
            kawase_down_mono,
            kawase_down_stereo,
            kawase_up_mono,
            kawase_up_stereo,
            final_pipelines: Mutex::new(HashMap::new()),
            blue_noise_tex,
            blue_noise_view,
            blue_noise_repeat_samp,
            earth_maps_tex,
            earth_maps_view,
            earth_maps_samp,
            solar_noise_tex,
            solar_noise_view,
            solar_noise_sampler,
            linear_clamp_samp,
            point_clamp_samp,
        })
    }

    /// Returns (or compiles & caches) the final AgX OLED composite pipeline matching `target_format` & `is_stereo`.
    pub fn final_pipeline(
        &self,
        device: &wgpu::Device,
        target_format: wgpu::TextureFormat,
        is_stereo: bool,
    ) -> wgpu::RenderPipeline {
        let mut cache = self.final_pipelines.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(p) = cache.get(&(target_format, is_stereo)) {
            return p.clone();
        }
        let vs = if is_stereo { "vs_stereo" } else { "vs_main" };
        let mv = if is_stereo {
            STEREO_MULTIVIEW_MASK
        } else {
            None
        };
        let p = build_post_pipeline(
            device,
            &self.post_pipeline_layout,
            &self.post_shader,
            vs,
            "fs_final_oled_agx",
            target_format,
            wgpu::BlendState::REPLACE,
            mv,
        );
        cache.insert((target_format, is_stereo), p.clone());
        p
    }
}

fn uniform_entry(
    binding: u32,
    visibility: wgpu::ShaderStages,
    bytes: usize,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: NonZeroU64::new(bytes as u64),
        },
        count: None,
    }
}

fn storage_entry(
    binding: u32,
    visibility: wgpu::ShaderStages,
    read_only: bool,
    min_bytes: usize,
) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Storage { read_only },
            has_dynamic_offset: false,
            min_binding_size: NonZeroU64::new(min_bytes as u64),
        },
        count: None,
    }
}

fn build_far_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    vs_entry: &str,
    multiview_mask: Option<NonZeroU32>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(if multiview_mask.is_some() {
            "far_shell_stereo"
        } else {
            "far_shell_mono"
        }),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some(vs_entry),
            compilation_options: Default::default(),
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: None,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: Some(false),
            depth_compare: Some(wgpu::CompareFunction::GreaterEqual),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: Default::default(),
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: HDR_SCENE_FORMAT,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask,
        cache: None,
    })
}

#[allow(clippy::too_many_arguments)]
fn build_hero_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    vs_entry: &str,
    fs_entry: &str,
    depth_write_enabled: bool,
    blend: wgpu::BlendState,
    multiview_mask: Option<NonZeroU32>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(fs_entry),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some(vs_entry),
            compilation_options: Default::default(),
            buffers: &[Some(wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<HeroVertex>() as u64,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &[wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                }],
            })],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Front),
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: Some(depth_write_enabled),
            depth_compare: Some(wgpu::CompareFunction::GreaterEqual),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: Default::default(),
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(fs_entry),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: HDR_SCENE_FORMAT,
                blend: Some(blend),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask,
        cache: None,
    })
}

fn build_solar_plasma_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    vs_entry: &str,
    fs_entry: &str,
    multiview_mask: Option<NonZeroU32>,
    family: Option<u32>,
) -> wgpu::RenderPipeline {
    let constants = family.map(|family| [("SOLAR_PLASMA_FAMILY", f64::from(family))]);
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(vs_entry),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some(vs_entry),
            compilation_options: Default::default(),
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: None,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: Some(false),
            depth_compare: Some(wgpu::CompareFunction::GreaterEqual),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: Default::default(),
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(fs_entry),
            compilation_options: wgpu::PipelineCompilationOptions {
                constants: constants.as_ref().map_or(&[], |constants| constants.as_slice()),
                ..Default::default()
            },
            targets: &[Some(wgpu::ColorTargetState {
                format: HDR_SCENE_FORMAT,
                blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask,
        cache: None,
    })
}

fn build_sprite_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    vs_entry: &str,
    blend: wgpu::BlendState,
    multiview_mask: Option<NonZeroU32>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(vs_entry),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some(vs_entry),
            compilation_options: Default::default(),
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: None,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: DEPTH_FORMAT,
            depth_write_enabled: Some(false),
            depth_compare: Some(wgpu::CompareFunction::GreaterEqual),
            stencil: Default::default(),
            bias: Default::default(),
        }),
        multisample: Default::default(),
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: HDR_SCENE_FORMAT,
                blend: Some(blend),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask,
        cache: None,
    })
}

#[allow(clippy::too_many_arguments)]
fn build_post_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    vs_entry: &str,
    fs_entry: &str,
    target_format: wgpu::TextureFormat,
    blend: wgpu::BlendState,
    multiview_mask: Option<NonZeroU32>,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(fs_entry),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some(vs_entry),
            compilation_options: Default::default(),
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            cull_mode: None,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: Default::default(),
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(fs_entry),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: target_format,
                blend: Some(blend),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview_mask,
        cache: None,
    })
}

#[cfg(test)]
mod shader_validation {
    use super::*;

    /// Catch WGSL parser/type errors before creating a GPU device or compiling
    /// the large DX12 pipelines. This also runs on CI machines without a GPU.
    #[test]
    fn every_shader_parses_and_validates_without_a_gpu() {
        let mut sources = vec![
            ("documentary hero", concat_hero_shaders()),
            ("solar plasma", concat_solar_plasma_shaders()),
            ("solar hero", concat_solar_hero_shaders()),
            ("solar noise lattice", crate::solar_noise::shader_source()),
        ];
        for (label, source) in [
            ("compute", COMPUTE_UNIVERSE_WGSL),
            ("sky", FAR_SHELL_WGSL),
            ("sprites", INSTANCED_SPRITES_WGSL),
            ("post", POST_BLOOM_AGX_WGSL),
        ] {
            sources.push((label, concat_with_prelude(source)));
        }
        for (label, source) in sources {
            let module = wgpu::naga::front::wgsl::parse_str(&source)
                .unwrap_or_else(|err| panic!("{label}: {}", err.emit_to_string(&source)));
            wgpu::naga::valid::Validator::new(
                wgpu::naga::valid::ValidationFlags::all(),
                wgpu::naga::valid::Capabilities::all(),
            )
            .validate(&module)
            .unwrap_or_else(|err| panic!("{label}: {}", err.emit_to_string(&source)));
        }
    }
}

#[cfg(test)]
#[path = "solar_blend_tests.rs"]
mod solar_blend_tests;
