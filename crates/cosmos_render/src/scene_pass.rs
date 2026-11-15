use std::num::NonZeroU32;

use bytemuck::cast_slice;
use wgpu::util::DeviceExt;

use crate::pipeline::{DEPTH_FORMAT, HDR_SCENE_FORMAT, PipelineBank, STEREO_MULTIVIEW_MASK};
use crate::types::{
    AudioReactiveUniforms, FarShellUniforms, FrameUniforms, HERO_CUBE_INDICES, HERO_CUBE_VERTICES,
    HeroVolumeInstance, RenderSceneFrame, SimDispatchUniforms, SimParticle, SpriteBatchUniforms,
    SpriteInstance,
};

const DEFAULT_PARTICLE_CAP: usize = 65_536;
const DEFAULT_HERO_CAP: usize = 512;
const DEFAULT_SPRITE_CAP: usize = 32_768;
const FIELD_GRID_DIM: u32 = 256;

/// Owns the `Rgba16Float` linear HDR scene target, Reversed-Z depth buffer, compute particle &
/// 2D ping-pong PDE buffers, and executes Compute + FarShell + Hero + Sprite + Overlay passes.
pub struct ScenePass {
    pub width: u32,
    pub height: u32,
    pub eye_count: u32,

    hdr_tex: wgpu::Texture,
    pub hdr_srv_view: wgpu::TextureView,
    hdr_rtv_view: wgpu::TextureView,
    depth_tex: wgpu::Texture,
    depth_rtv_view: wgpu::TextureView,

    pub u_frame_buf: wgpu::Buffer,
    pub u_audio_buf: wgpu::Buffer,
    pub shared_frame_bg: wgpu::BindGroup,

    u_sim_buf: wgpu::Buffer,
    u_field_sim_buf: wgpu::Buffer,
    particle_buf: wgpu::Buffer,
    particle_cap: usize,
    compute_bg: wgpu::BindGroup,
    field_compute_bg: wgpu::BindGroup,

    _field_tex_a: wgpu::Texture,
    _field_tex_b: wgpu::Texture,
    field_srv_a: wgpu::TextureView,
    field_srv_b: wgpu::TextureView,
    field_bg_ab: wgpu::BindGroup,
    field_bg_ba: wgpu::BindGroup,
    field_parity: bool,

    u_far_buf: wgpu::Buffer,
    far_bg: wgpu::BindGroup,

    cube_vb: wgpu::Buffer,
    cube_ib: wgpu::Buffer,
    hero_surface_buf: wgpu::Buffer,
    hero_surface_cap: usize,
    hero_surface_bg_a: wgpu::BindGroup,
    hero_surface_bg_b: wgpu::BindGroup,
    hero_volume_buf: wgpu::Buffer,
    hero_volume_cap: usize,
    hero_volume_bg_a: wgpu::BindGroup,
    hero_volume_bg_b: wgpu::BindGroup,

    u_particle_batch_buf: wgpu::Buffer,
    u_sprite_batch_buf: wgpu::Buffer,
    sprite_buf: wgpu::Buffer,
    sprite_cap: usize,
    particle_sprite_bg: wgpu::BindGroup,
    explicit_sprite_bg: wgpu::BindGroup,
}

impl ScenePass {
    pub fn new(
        device: &wgpu::Device,
        pipelines: &PipelineBank,
        width: u32,
        height: u32,
        is_stereo: bool,
    ) -> Self {
        let width = width.max(1);
        let height = height.max(1);
        let eye_count = if is_stereo { 2 } else { 1 };

        let (hdr_tex, hdr_srv_view, hdr_rtv_view, depth_tex, depth_rtv_view) =
            create_scene_targets(device, width, height, eye_count);

        let u_frame_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("u_frame"),
            contents: bytemuck::bytes_of(&FrameUniforms::default()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let u_audio_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("u_audio"),
            contents: bytemuck::bytes_of(&AudioReactiveUniforms::default()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let shared_frame_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shared_frame_bg (@group(0))"),
            layout: &pipelines.shared_frame_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: u_frame_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: u_audio_buf.as_entire_binding(),
                },
            ],
        });

        let u_sim_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("u_sim"),
            contents: bytemuck::bytes_of(&SimDispatchUniforms::default()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let u_field_sim_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("u_field_sim"),
            contents: bytemuck::bytes_of(&SimDispatchUniforms::default()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let particle_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sim_particles"),
            size: (DEFAULT_PARTICLE_CAP * std::mem::size_of::<SimParticle>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let compute_bg = make_compute_bg(device, pipelines, &u_sim_buf, &particle_buf);
        let field_compute_bg = make_compute_bg(device, pipelines, &u_field_sim_buf, &particle_buf);

        let (field_tex_a, field_srv_a, field_uav_a) = create_field_texture(device, "field_tex_a");
        let (field_tex_b, field_srv_b, field_uav_b) = create_field_texture(device, "field_tex_b");
        let field_bg_ab =
            make_field_bg(device, pipelines, &field_srv_a, &field_uav_b, "field_bg_ab");
        let field_bg_ba =
            make_field_bg(device, pipelines, &field_srv_b, &field_uav_a, "field_bg_ba");

        let u_far_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("u_far"),
            contents: bytemuck::bytes_of(&FarShellUniforms::default()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let far_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("far_bg"),
            layout: &pipelines.far_bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: u_far_buf.as_entire_binding(),
            }],
        });

        let cube_vb = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("hero_cube_vb"),
            contents: cast_slice(&HERO_CUBE_VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let cube_ib = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("hero_cube_ib"),
            contents: cast_slice(&HERO_CUBE_INDICES),
            usage: wgpu::BufferUsages::INDEX,
        });

        let hero_surface_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("hero_surface_buf"),
            size: (DEFAULT_HERO_CAP * std::mem::size_of::<HeroVolumeInstance>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let hero_volume_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("hero_volume_buf"),
            size: (DEFAULT_HERO_CAP * std::mem::size_of::<HeroVolumeInstance>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let hero_surface_bg_a = make_hero_bg(
            device,
            pipelines,
            &hero_surface_buf,
            &field_srv_a,
            "hero_surface_bg_a",
        );
        let hero_surface_bg_b = make_hero_bg(
            device,
            pipelines,
            &hero_surface_buf,
            &field_srv_b,
            "hero_surface_bg_b",
        );
        let hero_volume_bg_a = make_hero_bg(
            device,
            pipelines,
            &hero_volume_buf,
            &field_srv_a,
            "hero_volume_bg_a",
        );
        let hero_volume_bg_b = make_hero_bg(
            device,
            pipelines,
            &hero_volume_buf,
            &field_srv_b,
            "hero_volume_bg_b",
        );

        let u_particle_batch_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("u_particle_batch"),
            contents: bytemuck::bytes_of(&SpriteBatchUniforms::default()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let u_sprite_batch_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("u_sprite_batch"),
            contents: bytemuck::bytes_of(&SpriteBatchUniforms::default()),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let sprite_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("sprite_buf"),
            size: (DEFAULT_SPRITE_CAP * std::mem::size_of::<SpriteInstance>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let particle_sprite_bg = make_sprite_bg(
            device,
            pipelines,
            &u_particle_batch_buf,
            &particle_buf,
            &sprite_buf,
            "particle_sprite_bg",
        );
        let explicit_sprite_bg = make_sprite_bg(
            device,
            pipelines,
            &u_sprite_batch_buf,
            &particle_buf,
            &sprite_buf,
            "explicit_sprite_bg",
        );

        Self {
            width,
            height,
            eye_count,
            hdr_tex,
            hdr_srv_view,
            hdr_rtv_view,
            depth_tex,
            depth_rtv_view,
            u_frame_buf,
            u_audio_buf,
            shared_frame_bg,
            u_sim_buf,
            u_field_sim_buf,
            particle_buf,
            particle_cap: DEFAULT_PARTICLE_CAP,
            compute_bg,
            field_compute_bg,
            _field_tex_a: field_tex_a,
            _field_tex_b: field_tex_b,
            field_srv_a,
            field_srv_b,
            field_bg_ab,
            field_bg_ba,
            field_parity: false,
            u_far_buf,
            far_bg,
            cube_vb,
            cube_ib,
            hero_surface_buf,
            hero_surface_cap: DEFAULT_HERO_CAP,
            hero_surface_bg_a,
            hero_surface_bg_b,
            hero_volume_buf,
            hero_volume_cap: DEFAULT_HERO_CAP,
            hero_volume_bg_a,
            hero_volume_bg_b,
            u_particle_batch_buf,
            u_sprite_batch_buf,
            sprite_buf,
            sprite_cap: DEFAULT_SPRITE_CAP,
            particle_sprite_bg,
            explicit_sprite_bg,
        }
    }

    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
        is_stereo: bool,
    ) -> bool {
        let width = width.max(1);
        let height = height.max(1);
        let eye_count = if is_stereo { 2 } else { 1 };
        if self.width == width && self.height == height && self.eye_count == eye_count {
            return false;
        }
        self.width = width;
        self.height = height;
        self.eye_count = eye_count;
        let (hdr_tex, hdr_srv_view, hdr_rtv_view, depth_tex, depth_rtv_view) =
            create_scene_targets(device, width, height, eye_count);
        self.hdr_tex = hdr_tex;
        self.hdr_srv_view = hdr_srv_view;
        self.hdr_rtv_view = hdr_rtv_view;
        self.depth_tex = depth_tex;
        self.depth_rtv_view = depth_rtv_view;
        true
    }

    fn ensure_capacities(
        &mut self,
        device: &wgpu::Device,
        pipelines: &PipelineBank,
        scene: &RenderSceneFrame,
    ) {
        let mut rebuilt_particles = false;
        let needed_particles = scene.particle_draw_count.max(scene.compute.counts[1]) as usize;
        if needed_particles > self.particle_cap {
            self.particle_cap = needed_particles.next_power_of_two();
            self.particle_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("sim_particles"),
                size: (self.particle_cap * std::mem::size_of::<SimParticle>()) as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.compute_bg =
                make_compute_bg(device, pipelines, &self.u_sim_buf, &self.particle_buf);
            self.field_compute_bg =
                make_compute_bg(device, pipelines, &self.u_field_sim_buf, &self.particle_buf);
            rebuilt_particles = true;
        }

        let mut rebuilt_sprites = false;
        if scene.sprites.len() > self.sprite_cap {
            self.sprite_cap = scene.sprites.len().next_power_of_two();
            self.sprite_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("sprite_buf"),
                size: (self.sprite_cap * std::mem::size_of::<SpriteInstance>()) as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            rebuilt_sprites = true;
        }

        if rebuilt_particles || rebuilt_sprites {
            self.particle_sprite_bg = make_sprite_bg(
                device,
                pipelines,
                &self.u_particle_batch_buf,
                &self.particle_buf,
                &self.sprite_buf,
                "particle_sprite_bg",
            );
            self.explicit_sprite_bg = make_sprite_bg(
                device,
                pipelines,
                &self.u_sprite_batch_buf,
                &self.particle_buf,
                &self.sprite_buf,
                "explicit_sprite_bg",
            );
        }

        if scene.heroes_surface.len() > self.hero_surface_cap {
            self.hero_surface_cap = scene.heroes_surface.len().next_power_of_two();
            self.hero_surface_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("hero_surface_buf"),
                size: (self.hero_surface_cap * std::mem::size_of::<HeroVolumeInstance>()) as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.hero_surface_bg_a = make_hero_bg(
                device,
                pipelines,
                &self.hero_surface_buf,
                &self.field_srv_a,
                "hero_surface_bg_a",
            );
            self.hero_surface_bg_b = make_hero_bg(
                device,
                pipelines,
                &self.hero_surface_buf,
                &self.field_srv_b,
                "hero_surface_bg_b",
            );
        }
        if scene.heroes_volume.len() > self.hero_volume_cap {
            self.hero_volume_cap = scene.heroes_volume.len().next_power_of_two();
            self.hero_volume_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("hero_volume_buf"),
                size: (self.hero_volume_cap * std::mem::size_of::<HeroVolumeInstance>()) as u64,
                usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            self.hero_volume_bg_a = make_hero_bg(
                device,
                pipelines,
                &self.hero_volume_buf,
                &self.field_srv_a,
                "hero_volume_bg_a",
            );
            self.hero_volume_bg_b = make_hero_bg(
                device,
                pipelines,
                &self.hero_volume_buf,
                &self.field_srv_b,
                "hero_volume_bg_b",
            );
        }
    }

    /// Uploads frame uniforms/buffers, dispatches compute particles/fields, and renders
    /// FarShell -> HeroSurface -> HeroVolume -> Particles/Sprites -> `draw_overlay_in_hdr`
    /// inside a single `Rgba16Float` HDR render pass (with `#000000` void clear).
    pub fn execute<F>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        pipelines: &PipelineBank,
        scene: &RenderSceneFrame,
        mut draw_overlay_in_hdr: F,
    ) where
        F: FnMut(&mut wgpu::RenderPass<'_>, bool),
    {
        self.ensure_capacities(device, pipelines, scene);

        queue.write_buffer(&self.u_frame_buf, 0, bytemuck::bytes_of(&scene.frame));
        queue.write_buffer(&self.u_audio_buf, 0, bytemuck::bytes_of(&scene.audio));
        queue.write_buffer(&self.u_far_buf, 0, bytemuck::bytes_of(&scene.far));
        queue.write_buffer(&self.u_sim_buf, 0, bytemuck::bytes_of(&scene.compute));
        if let Some(field_mode) = scene.dispatch_field_grid {
            let mut field_sim = scene.compute;
            field_sim.counts[0] = field_mode;
            queue.write_buffer(&self.u_field_sim_buf, 0, bytemuck::bytes_of(&field_sim));
        }
        queue.write_buffer(
            &self.u_particle_batch_buf,
            0,
            bytemuck::bytes_of(&scene.particle_batch),
        );
        queue.write_buffer(
            &self.u_sprite_batch_buf,
            0,
            bytemuck::bytes_of(&scene.sprite_batch),
        );

        if let Some((mode, count)) = scene.reseed_compute_particles
            && count > 0
        {
            let seeded = seed_particles_for_mode(mode, count, scene.compute.params_b[0].max(0.5));
            queue.write_buffer(&self.particle_buf, 0, cast_slice(&seeded));
        }

        if !scene.heroes_surface.is_empty() {
            queue.write_buffer(
                &self.hero_surface_buf,
                0,
                cast_slice(&scene.heroes_surface),
            );
        }
        if !scene.heroes_volume.is_empty() {
            queue.write_buffer(&self.hero_volume_buf, 0, cast_slice(&scene.heroes_volume));
        }
        if !scene.sprites.is_empty() {
            queue.write_buffer(&self.sprite_buf, 0, cast_slice(&scene.sprites));
        }

        // 1) Compute particle advection & optional 2D PDE grid step
        if scene.compute.counts[1] > 0 || scene.dispatch_field_grid.is_some() {
            let mut cpass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("cosmos_compute_pass"),
                timestamp_writes: None,
            });
            if scene.compute.counts[1] > 0 {
                cpass.set_pipeline(&pipelines.advect_pipeline);
                cpass.set_bind_group(0, &self.shared_frame_bg, &[]);
                cpass.set_bind_group(1, &self.compute_bg, &[]);
                let wg = scene.compute.counts[1].div_ceil(256);
                cpass.dispatch_workgroups(wg, 1, 1);
            }
            if scene.dispatch_field_grid.is_some() {
                cpass.set_pipeline(&pipelines.field_pipeline);
                cpass.set_bind_group(0, &self.shared_frame_bg, &[]);
                cpass.set_bind_group(1, &self.field_compute_bg, &[]);
                let bg = if self.field_parity {
                    &self.field_bg_ba
                } else {
                    &self.field_bg_ab
                };
                cpass.set_bind_group(2, bg, &[]);
                let wg = FIELD_GRID_DIM.div_ceil(8);
                cpass.dispatch_workgroups(wg, wg, 1);
                self.field_parity = !self.field_parity;
            }
        }

        // 2) HDR Linear Scene Render Pass (#000000 void floor clear + Reversed-Z 0.0 clear)
        let is_stereo = self.eye_count > 1;
        let multiview_mask: Option<NonZeroU32> = if is_stereo {
            STEREO_MULTIVIEW_MASK
        } else {
            None
        };

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some(if is_stereo {
                    "hdr_scene_stereo"
                } else {
                    "hdr_scene_mono"
                }),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.hdr_rtv_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_rtv_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.0), // Reversed-Z infinite far = 0.0
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask,
            });

            // 2a. Far Shell Sky Dome
            rpass.set_pipeline(if is_stereo {
                &pipelines.far_stereo
            } else {
                &pipelines.far_mono
            });
            rpass.set_bind_group(0, &self.shared_frame_bg, &[]);
            rpass.set_bind_group(1, &self.far_bg, &[]);
            rpass.draw(0..3, 0..1);

            // 2b. Opaque / Depth-Writing Hero Surfaces (Earth cutaway, planets, Kerr black core)
            let hero_surface_bg = if self.field_parity {
                &self.hero_surface_bg_b
            } else {
                &self.hero_surface_bg_a
            };
            if !scene.heroes_surface.is_empty() {
                rpass.set_pipeline(if is_stereo {
                    &pipelines.hero_surface_stereo
                } else {
                    &pipelines.hero_surface_mono
                });
                rpass.set_bind_group(0, &self.shared_frame_bg, &[]);
                rpass.set_bind_group(1, hero_surface_bg, &[]);
                rpass.set_vertex_buffer(0, self.cube_vb.slice(..));
                rpass.set_index_buffer(self.cube_ib.slice(..), wgpu::IndexFormat::Uint16);
                rpass.draw_indexed(0..36, 0, 0..scene.heroes_surface.len() as u32);
            }

            // 2c. Translucent Volumetric Raymarched Heroes (Auroras, Cassini rings, Nebulae, Kerr Lensing, HSL Prism, Cymatic Harp, Lotus)
            let hero_volume_bg = if self.field_parity {
                &self.hero_volume_bg_b
            } else {
                &self.hero_volume_bg_a
            };
            if !scene.heroes_volume.is_empty() {
                rpass.set_pipeline(if is_stereo {
                    &pipelines.hero_volume_stereo
                } else {
                    &pipelines.hero_volume_mono
                });
                rpass.set_bind_group(0, &self.shared_frame_bg, &[]);
                rpass.set_bind_group(1, hero_volume_bg, &[]);
                rpass.set_vertex_buffer(0, self.cube_vb.slice(..));
                rpass.set_index_buffer(self.cube_ib.slice(..), wgpu::IndexFormat::Uint16);
                rpass.draw_indexed(0..36, 0, 0..scene.heroes_volume.len() as u32);
            }

            // 2d. Compute-Advected Particles
            if scene.particle_draw_count > 0 {
                rpass.set_pipeline(if is_stereo {
                    &pipelines.particle_stereo
                } else {
                    &pipelines.particle_mono
                });
                rpass.set_bind_group(0, &self.shared_frame_bg, &[]);
                rpass.set_bind_group(1, &self.particle_sprite_bg, &[]);
                rpass.draw(0..6, 0..scene.particle_draw_count);
            }

            // 2e. Explicit World-Space & Diegetic Wrist/Dwell Sprites
            if !scene.sprites.is_empty() {
                rpass.set_pipeline(if is_stereo {
                    &pipelines.sprite_stereo
                } else {
                    &pipelines.sprite_mono
                });
                rpass.set_bind_group(0, &self.shared_frame_bg, &[]);
                rpass.set_bind_group(1, &self.explicit_sprite_bg, &[]);
                rpass.draw(0..6, 0..scene.sprites.len() as u32);
            }

            // 2f. 3D MSDF Typography & Tethers (`cosmos_typo`) rendered directly in HDR before Bloom + AgX
            draw_overlay_in_hdr(&mut rpass, is_stereo);
        }
    }
}

fn create_scene_targets(
    device: &wgpu::Device,
    width: u32,
    height: u32,
    eye_count: u32,
) -> (
    wgpu::Texture,
    wgpu::TextureView,
    wgpu::TextureView,
    wgpu::Texture,
    wgpu::TextureView,
) {
    let size = wgpu::Extent3d {
        width,
        height,
        depth_or_array_layers: eye_count,
    };
    let hdr_tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("hdr_scene_tex"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: HDR_SCENE_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let hdr_srv_view = hdr_tex.create_view(&wgpu::TextureViewDescriptor {
        label: Some("hdr_srv_d2array"),
        format: Some(HDR_SCENE_FORMAT),
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        base_array_layer: 0,
        array_layer_count: Some(eye_count),
        ..Default::default()
    });
    let rtv_dim = if eye_count > 1 {
        wgpu::TextureViewDimension::D2Array
    } else {
        wgpu::TextureViewDimension::D2
    };
    let hdr_rtv_view = hdr_tex.create_view(&wgpu::TextureViewDescriptor {
        label: Some("hdr_rtv"),
        format: Some(HDR_SCENE_FORMAT),
        dimension: Some(rtv_dim),
        base_array_layer: 0,
        array_layer_count: Some(eye_count),
        ..Default::default()
    });

    let depth_tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("depth_reversed_z"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let depth_rtv_view = depth_tex.create_view(&wgpu::TextureViewDescriptor {
        label: Some("depth_rtv"),
        format: Some(DEPTH_FORMAT),
        dimension: Some(rtv_dim),
        base_array_layer: 0,
        array_layer_count: Some(eye_count),
        ..Default::default()
    });

    (hdr_tex, hdr_srv_view, hdr_rtv_view, depth_tex, depth_rtv_view)
}

fn create_field_texture(
    device: &wgpu::Device,
    label: &str,
) -> (wgpu::Texture, wgpu::TextureView, wgpu::TextureView) {
    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some(label),
        size: wgpu::Extent3d {
            width: FIELD_GRID_DIM,
            height: FIELD_GRID_DIM,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: HDR_SCENE_FORMAT,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let srv = tex.create_view(&wgpu::TextureViewDescriptor::default());
    let uav = tex.create_view(&wgpu::TextureViewDescriptor::default());
    (tex, srv, uav)
}

fn make_compute_bg(
    device: &wgpu::Device,
    pipelines: &PipelineBank,
    u_sim_buf: &wgpu::Buffer,
    particle_buf: &wgpu::Buffer,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("compute_bg"),
        layout: &pipelines.compute_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: u_sim_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: particle_buf.as_entire_binding(),
            },
        ],
    })
}

fn make_field_bg(
    device: &wgpu::Device,
    pipelines: &PipelineBank,
    field_in: &wgpu::TextureView,
    field_out: &wgpu::TextureView,
    label: &str,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout: &pipelines.field_grid_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(field_in),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(field_out),
            },
        ],
    })
}

fn make_hero_bg(
    device: &wgpu::Device,
    pipelines: &PipelineBank,
    hero_buf: &wgpu::Buffer,
    field_grid_view: &wgpu::TextureView,
    label: &str,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout: &pipelines.hero_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: hero_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&pipelines.blue_noise_view),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&pipelines.blue_noise_repeat_samp),
            },
            wgpu::BindGroupEntry {
                binding: 3,
                resource: wgpu::BindingResource::TextureView(field_grid_view),
            },
            wgpu::BindGroupEntry {
                binding: 4,
                resource: wgpu::BindingResource::Sampler(&pipelines.blue_noise_repeat_samp),
            },
            wgpu::BindGroupEntry {
                binding: 5,
                resource: wgpu::BindingResource::TextureView(&pipelines.earth_maps_view),
            },
            wgpu::BindGroupEntry {
                binding: 6,
                resource: wgpu::BindingResource::Sampler(&pipelines.earth_maps_samp),
            },
        ],
    })
}

fn make_sprite_bg(
    device: &wgpu::Device,
    pipelines: &PipelineBank,
    u_batch_buf: &wgpu::Buffer,
    particle_buf: &wgpu::Buffer,
    sprite_buf: &wgpu::Buffer,
    label: &str,
) -> wgpu::BindGroup {
    device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some(label),
        layout: &pipelines.sprite_bgl,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: u_batch_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: particle_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: sprite_buf.as_entire_binding(),
            },
        ],
    })
}

/// Deterministically initializes GPU particles in sector-local coordinates for compute modes `0..=16`.
pub fn seed_particles_for_mode(mode: u32, count: u32, radius_scale: f32) -> Vec<SimParticle> {
    let mut out = Vec::with_capacity(count as usize);
    for i in 0..count {
        let mut s = i.wrapping_mul(747_796_405).wrapping_add(2_891_336_453);
        let mut next_u = || {
            s = s.wrapping_mul(747_796_405).wrapping_add(2_891_336_453);
            let word = ((s >> ((s >> 28).wrapping_add(4))) ^ s).wrapping_mul(277_803_737);
            let r = (word >> 22) ^ word;
            (r & 0x00FF_FFFF) as f32 / 16_777_215.0
        };

        let u0 = next_u();
        let u1 = next_u();
        let u2 = next_u();
        let theta = u0 * std::f32::consts::TAU;
        let phi = (u1 * 2.0 - 1.0).acos();
        let radial = u2.cbrt() * radius_scale;

        let pos = match mode {
            // Flat spiral galaxy / cymatic / Keplerian ring / Hyper-Lotus / Relativistic Geodesic modes
            0 | 8 | 10 | 15 | 16 => {
                let r_disk = (0.06 + 0.94 * u1.sqrt()) * radius_scale;
                let arm = (i % 4) as f32 * (std::f32::consts::FRAC_PI_2);
                let ang = if matches!(mode, 10 | 15 | 16) {
                    theta
                } else {
                    theta * 0.25 + arm + (r_disk / radius_scale) * 3.8
                };
                let y = (u2 - 0.5) * radius_scale * 0.10 * (1.0 - 0.65 * (r_disk / radius_scale));
                [r_disk * ang.cos(), y, r_disk * ang.sin()]
            }
            // Geodynamo circum-planetary shell (03) kept tightly hugged inside magnetosphere
            11 => {
                let geo_r = radius_scale * (0.28 + 0.36 * u2.cbrt());
                [
                    geo_r * phi.sin() * theta.cos(),
                    geo_r * phi.cos(),
                    geo_r * phi.sin() * theta.sin(),
                ]
            }
            // Spherical shell / protocell / electron orbital / redshift stream
            4 | 6 | 13 => [
                radial * phi.sin() * theta.cos(),
                radial * phi.cos(),
                radial * phi.sin() * theta.sin(),
            ],
            // Orthogonal bus grid for mode 14
            14 => [
                ((u0 * 6.0).floor() - 2.5) * radius_scale * 0.25,
                ((u1 * 6.0).floor() - 2.5) * radius_scale * 0.25,
                ((u2 * 6.0).floor() - 2.5) * radius_scale * 0.25,
            ],
            // Cosmic web / Planck / Neural connectome / Chromatic / Convection volumes
            _ => [
                (u0 * 2.0 - 1.0) * radius_scale,
                (u1 * 2.0 - 1.0) * radius_scale,
                (u2 * 2.0 - 1.0) * radius_scale,
            ],
        };

        let n_orb = 1 + (i % 4);
        let l_orb = i % n_orb;
        let sub_id = (l_orb << 8) | n_orb;
        let base_radius = match mode {
            0 | 1 => 0.0011 * radius_scale,
            2 => 0.0009 * radius_scale,
            3..=5 => 0.0018 * radius_scale,
            _ => 0.0013 * radius_scale,
        };

        out.push(SimParticle {
            pos_age: [pos[0], pos[1], pos[2], next_u()],
            vel_mass: [0.0, 0.0, 0.0, 1.0],
            color_size: [1.0, 1.0, 1.0, base_radius.max(0.0022)],
            tags: [mode, sub_id, i.wrapping_mul(2654435761), 0],
        });
    }
    out
}
