use cosmos_math::ScalePose64;
use cosmos_spatial::{CosmosSpatialIndex, EntityKind, LodTier, VisibleEntity};
use glam::{DQuat, DVec3, Quat, Vec3};

use crate::doc_stages::{emit_celestial_or_landmark, emit_doc_stage_hero};
use crate::scene_pass::seed_particles_for_mode;
use crate::types::{
    AudioReactiveUniforms, DiegeticUiInput, FarShellUniforms, FrameUniforms, HeroVolumeInstance,
    PostSettings, RenderCamera, RenderSceneFrame, SimDispatchUniforms, SpriteBatchUniforms,
    SpriteInstance,
};

const DEFAULT_PARTICLE_COUNT: u32 = 32_768;

/// Live documentary zone, feature, cue, and domain telemetry fed from runtime/narration into scene assembly.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DocumentaryFocusState {
    pub active_zone_index: u16,       // u16::MAX = overview
    pub active_feature_index: u16,    // u16::MAX = zone macro
    pub scene_index: i32,             // ZoneRecord::scene_index or -1
    pub narration_cue_progress: f32,  // [0..1] SyncVoice live progress
    pub narration_active_weight: f32, // [0..1] active narration glow
    pub zone_accent_rgb: [f32; 3],    // linear HDR active zone accent
    pub telemetry: [f32; 4],          // domain params (01: [H,S,L,mat]; 02: [midi,timbre,freq,amp]; 03..25: [range0,range1,phys0,phys1])
    pub cue_tags_mask: u8,            // 6-bit semantic narration cue mask packed into base_flags[2..8]
    pub submode_override: u8,         // optional narration submode override (used when caller passes submode == 0)
    pub emphasis_boost: f32,          // [0..1] spotlight weight modulation from SegmentVisualHint
}

impl Default for DocumentaryFocusState {
    fn default() -> Self {
        Self {
            active_zone_index: u16::MAX,
            active_feature_index: u16::MAX,
            scene_index: -1,
            narration_cue_progress: 0.0,
            narration_active_weight: 0.0,
            zone_accent_rgb: [0.0; 3],
            telemetry: [0.0; 4],
            cue_tags_mask: 0,
            submode_override: 0,
            emphasis_boost: 1.0,
        }
    }
}

/// Input state for assembling one frame of the 25-experience continuous cosmos.
#[derive(Clone, Debug)]
pub struct SceneBuildInput {
    pub cam_pose: ScalePose64,
    pub camera: RenderCamera,
    pub viewport_width: u32,
    pub viewport_height: u32,
    pub time_sec: f32,
    pub dt_sec: f32,
    pub scale_speed: f32,
    pub active_experience_id: u8,
    pub active_region_phase: f32,
    pub focused_entity_id: Option<u64>,
    /// `[0..1]` Deep Earth crust/mantle/core cutaway reveal (auto-activated inside `03` below `u <= 6.72`).
    pub earth_cutaway_blend: f32,
    pub fovea_rad: f32,
    pub particle_budget: u32,
    pub audio: AudioReactiveUniforms,
    pub diegetic: DiegeticUiInput,
    pub post: PostSettings,
    pub documentary: DocumentaryFocusState,
}

impl SceneBuildInput {
    pub fn desktop_default(
        cam_pose: ScalePose64,
        camera: RenderCamera,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            cam_pose,
            camera,
            viewport_width: width.max(1),
            viewport_height: height.max(1),
            time_sec: 0.0,
            dt_sec: 1.0 / 90.0,
            scale_speed: 0.0,
            active_experience_id: 3,
            active_region_phase: 0.0,
            focused_entity_id: None,
            earth_cutaway_blend: 0.0,
            fovea_rad: 0.34,
            particle_budget: DEFAULT_PARTICLE_COUNT,
            audio: AudioReactiveUniforms::default(),
            diegetic: DiegeticUiInput::default(),
            post: PostSettings::default(),
            documentary: DocumentaryFocusState::default(),
        }
    }
}

/// Translates `CosmosSpatialIndex` visible multiscale results + audio/diegetic state into
/// GPU buffers and compute dispatch parameters for `CosmosRenderer`.
#[derive(Debug, Default)]
pub struct UniverseSceneBuilder {
    frame_index: u32,
    last_compute_mode: Option<u32>,
}

impl UniverseSceneBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build(
        &mut self,
        index: &CosmosSpatialIndex,
        input: &SceneBuildInput,
    ) -> RenderSceneFrame {
        self.frame_index = self.frame_index.wrapping_add(1);
        let u_cam = input.cam_pose.scale_exp as f32;
        let accent = experience_accent_linear(input.active_experience_id);

        let vp0 = input.camera.eyes[0].view_proj.to_cols_array_2d();
        let vp1 = input.camera.eyes[1].view_proj.to_cols_array_2d();
        let ivp0 = input.camera.eyes[0].inv_view_proj.to_cols_array_2d();
        let ivp1 = input.camera.eyes[1].inv_view_proj.to_cols_array_2d();
        let e0 = input.camera.eyes[0].eye_offset;
        let e1 = input.camera.eyes[1].eye_offset;
        let stereo_flag = if input.camera.is_stereo { 1.0 } else { 0.0 };

        let focus_dir = input.diegetic.focus_ray.normalize_or(input.camera.cam_forward);
        let frame = FrameUniforms {
            view_proj: [vp0, vp1],
            inv_view_proj: [ivp0, ivp1],
            eye_offset: [
                [e0.x, e0.y, e0.z, stereo_flag],
                [e1.x, e1.y, e1.z, stereo_flag],
            ],
            cam_forward: [
                input.camera.cam_forward.x,
                input.camera.cam_forward.y,
                input.camera.cam_forward.z,
                input.camera.fov_y_rad,
            ],
            cam_right: [
                input.camera.cam_right.x,
                input.camera.cam_right.y,
                input.camera.cam_right.z,
                input.camera.aspect,
            ],
            cam_up: [
                input.camera.cam_up.x,
                input.camera.cam_up.y,
                input.camera.cam_up.z,
                input.camera.near_plane,
            ],
            scale_params: [u_cam, input.scale_speed, input.post.exposure, input.time_sec],
            post_params: [
                input.post.bloom_thresh.max(1.05),
                input.post.bloom_gain.max(0.0),
                input.post.dispersion.max(0.0),
                input.post.dither_amp.max(0.0),
            ],
            viewport_params: [
                input.viewport_width as f32,
                input.viewport_height as f32,
                1.0 / input.viewport_width.max(1) as f32,
                1.0 / input.viewport_height.max(1) as f32,
            ],
            stereo_params: [
                if input.camera.is_stereo { 2.0 } else { 1.0 },
                input.fovea_rad.max(0.22),
                (self.frame_index % 128) as f32,
                input.active_region_phase,
            ],
            focus_ray: [
                focus_dir.x,
                focus_dir.y,
                focus_dir.z,
                input.diegetic.dwell_progress.clamp(0.0, 1.0),
            ],
            region_accent: [accent.x, accent.y, accent.z, input.active_experience_id as f32],
        };

        let far = build_far_shell_uniforms(u_cam, input.active_experience_id, &input.documentary);

        let compute_mode = select_compute_mode(u_cam, input.active_experience_id, &input.documentary);
        let particle_count = input.particle_budget.max(256);
        let reseed_compute_particles = if self.last_compute_mode != Some(compute_mode) {
            self.last_compute_mode = Some(compute_mode);
            Some((compute_mode, particle_count))
        } else {
            None
        };

        let anchor_sector = index.best_anchor_sector_for_scale(input.cam_pose.scale_exp);
        let anchor_rel = input
            .cam_pose
            .project_entity(
                &index.tree,
                &ScalePose64::new(anchor_sector, DVec3::ZERO, input.cam_pose.scale_exp, DQuat::IDENTITY),
                10.0f64.powf(input.cam_pose.scale_exp),
            )
            .map(|r| r.pos)
            .unwrap_or(Vec3::new(0.0, 0.0, -3.0));

        let particle_field_radius = if input.active_experience_id == 3 {
            3.0f32
        } else {
            4.2f32
        };
        let compute = SimDispatchUniforms {
            params_a: [
                input.dt_sec.clamp(0.0005, 0.033),
                input.time_sec,
                0.94,
                1.0,
            ],
            params_b: [particle_field_radius, 2.2, 0.9, 6.0 + input.audio.levels_a[2] * 4.0],
            attractor_a: [0.0, 0.0, 0.0, 1.15],
            attractor_b: [1.8, 0.35, -1.4, 0.65],
            counts: [compute_mode, particle_count, 256, 256],
        };

        let dispatch_field_grid = match (compute_mode, input.active_experience_id) {
            (_, 15) => Some(0),
            // Exp 16 Z09 multicellular / Z10 development: Gray-Scott morphogen (Turing) patterning
            (_, 16) if matches!(input.documentary.active_zone_index, 9 | 10) => Some(0),
            (_, 2) | (8, _) => Some(1),
            (_, 3) => Some(2),
            _ => None,
        };

        let particle_kind = match compute_mode {
            1 | 2 | 7 | 14 => 1.0, // Flow/Filament/Bus splats
            _ => 0.0,              // Luminance blackbody/quantum sprites
        };
        let particle_batch = SpriteBatchUniforms {
            sector_origin_scale: [anchor_rel.x, anchor_rel.y, anchor_rel.z, 1.0],
            batch_params: [particle_kind, 0.68, 0.44, input.time_sec],
        };

        // Spatial octree query (use left eye / mono frustum)
        let mut heroes_surface = Vec::with_capacity(16);
        let mut heroes_volume = Vec::with_capacity(32);
        let mut sprites = Vec::with_capacity(512);

        let min_angular = 0.05_f32.to_radians();
        let mut active_zone_shell: Option<(Vec3, f32, [f32; 4], f32)> = None;
        if let Ok(query) = index.query_visible(
            &input.cam_pose,
            input.camera.eyes[0].view_proj,
            min_angular,
            Some(4096),
        ) {
            for vis in &query.visible {
                if vis.kind == EntityKind::ZoneShell
                    && vis.experience_id == input.active_experience_id
                    && vis.zone_index == input.documentary.active_zone_index
                {
                    let z_pos = vis.pose.pos;
                    let z_dist = z_pos.length();
                    let z_fwd = z_pos.normalize_or_zero().dot(input.camera.cam_forward);
                    if z_pos.is_finite() && (0.25..=6.0).contains(&z_dist) && z_fwd > 0.65 {
                        let z_rot = vis.pose.orient;
                        let z_dwell = if input.focused_entity_id == Some(vis.id) {
                            input.diegetic.dwell_progress.clamp(0.0, 1.0)
                        } else {
                            input.diegetic.dwell_progress
                        };
                        active_zone_shell = Some((
                            z_pos,
                            vis.pose.rel_scale.clamp(0.65, 2.8),
                            [z_rot.x, z_rot.y, z_rot.z, z_rot.w],
                            z_dwell,
                        ));
                    }
                }
                emit_visible_entity(
                    vis,
                    input,
                    &mut heroes_surface,
                    &mut heroes_volume,
                    &mut sprites,
                );
            }
        }

        // Ensure hero prism / harp / Earth / stage are richly populated even when looking near/inside anchor or at a ZoneShell
        if heroes_surface.is_empty() && heroes_volume.is_empty() {
            if let Some((zs_pos, zs_scale, zs_rot, zs_dwell)) = active_zone_shell {
                emit_doc_stage_hero(
                    input.active_experience_id,
                    zs_pos,
                    zs_scale,
                    zs_rot,
                    Vec3::new(0.62, 0.45, -0.64).normalize(),
                    zs_dwell,
                    input,
                    &mut heroes_surface,
                    &mut heroes_volume,
                    &mut sprites,
                );
            } else {
                emit_fallback_hero(
                    input.active_experience_id,
                    u_cam,
                    input,
                    &mut heroes_surface,
                    &mut heroes_volume,
                    &mut sprites,
                );
            }
        }

        emit_diegetic_ui(&input.diegetic, input.time_sec, accent, &mut sprites);

        RenderSceneFrame {
            frame,
            audio: input.audio,
            far,
            compute,
            reseed_compute_particles,
            dispatch_field_grid,
            particle_batch,
            particle_draw_count: particle_count,
            heroes_surface,
            heroes_volume,
            sprite_batch: SpriteBatchUniforms {
                sector_origin_scale: [0.0, 0.0, 0.0, 1.0],
                batch_params: [0.0, 1.4, 1.0, input.time_sec],
            },
            sprites,
            post: input.post,
        }
    }
}

/// Returns seeded particle buffer for callers initializing compute state outside `UniverseSceneBuilder`.
pub fn initial_particles_for_mode(mode: u32, count: u32) -> Vec<crate::types::SimParticle> {
    seed_particles_for_mode(mode, count, 4.2)
}

fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

pub fn select_compute_mode(
    u_cam: f32,
    experience_id: u8,
    doc: &DocumentaryFocusState,
) -> u32 {
    match experience_id {
        1 => 9,
        2 => 8,
        3 => 11,
        4 => 10,
        5 | 6 => 0,
        7 => 1,
        8 => 2,
        9 => match doc.active_zone_index {
            4..=7 => 13,
            8..=10 => 5,
            _ => 2,
        },
        10 => match doc.active_zone_index {
            0..=4 => 13,
            5 => 3,
            6..=8 => 5,
            _ => {
                if u_cam >= 26.8 {
                    5
                } else {
                    13
                }
            }
        },
        21 => match doc.active_zone_index {
            6..=10 => 12,
            19 => 1,
            22 | 23 => 15,
            _ => 13,
        },
        11 | 12 => 13,
        22 => match doc.active_zone_index {
            2 | 8..=11 | 13..=14 | 26 => 14,
            19..=22 | 25 | 27 => 15,
            _ => 13,
        },
        13 => match doc.active_zone_index {
            2 | 3 => 4,    // molecules, atoms -> Electron Spherical-Harmonic Orbitals
            4..=6 => 3,    // nucleus, nucleons, quarks -> QCD Quark-Gluon Color Flux
            1 | 7 => 12,   // matter, elementary -> calm structured lattice
            0 | 8..=11 => 5, // residual, forces, fields, vacuum, frontier -> Quantum Vacuum Fluctuations
            _ => {
                if u_cam < -12.5 {
                    3
                } else {
                    4
                }
            }
        },
        14 => match doc.active_zone_index {
            1..=5 => 13, // everyday, special, causality, spacetime, equivalence -> calm reference dust
            6..=9 => 16, // curvature, blackholes, waves, cosmos -> relativistic geodesic lensing flow
            _ => 5,      // frontier, horizons, planck, quantumgravity, emergent, origin -> Planck Spin-Foam
        },
        15 => match doc.active_zone_index {
            9 => 6,  // selforganization -> local-rule flock / reaction swarm
            _ => 12, // emergent convection vortex + Gray-Scott PDE
        },
        16 => match doc.active_zone_index {
            15..=18 => 7, // nervous, brains, perception, consciousness -> neural connectome firing
            _ => 6,       // cells, metabolism, heredity, evolution, organisms -> cellular BZ shell
        },
        17 => 7,
        18..=20 => 14,
        23..=25 => 15,
        _ => {
            if u_cam < -21.0 {
                5
            } else if u_cam < -12.5 {
                3
            } else if u_cam < -9.0 {
                4
            } else if u_cam < -4.0 {
                6
            } else if u_cam > 23.0 {
                2
            } else if u_cam > 18.0 {
                0
            } else {
                9
            }
        }
    }
}

pub fn build_far_shell_uniforms(
    u_cam: f32,
    experience_id: u8,
    doc: &DocumentaryFocusState,
) -> FarShellUniforms {
    let mw_fade_in = smoothstep(5.0, 9.5, u_cam);
    let mw_fade_out = 1.0 - smoothstep(21.2, 23.6, u_cam);
    let mut galaxy_plane = (mw_fade_in * mw_fade_out).clamp(0.0, 1.0);

    // Keep Cosmic Web (u = 25.35) completely free of warm CMB haze, separate CMB (09) from Inflation (10),
    // and fade Inflation out cleanly above u = 27.4 so Final Horizon Lotus (u ~ 28.5) floats in >=70% #000000 void.
    let mut cmb_alpha = (smoothstep(25.8, 26.28, u_cam) * (1.0 - smoothstep(26.9, 27.3, u_cam))).clamp(0.0, 1.0);
    let mut inflation_alpha =
        (smoothstep(26.70, 27.05, u_cam) * (1.0 - smoothstep(27.20, 27.42, u_cam))).clamp(0.0, 1.0);
    let lotus_boost = if matches!(experience_id, 11 | 12 | 23 | 24 | 25) && u_cam > 26.0 {
        0.42
    } else {
        0.0
    };
    let lotus_alpha = (smoothstep(27.1, 28.0, u_cam)).max(lotus_boost);

    let mut quantum_void_alpha = 1.0 - smoothstep(-22.0, -14.0, u_cam);
    let cosmic_web_sky_alpha =
        (smoothstep(23.5, 24.8, u_cam) * (1.0 - smoothstep(25.95, 26.4, u_cam))).clamp(0.0, 1.0);

    let north = Vec3::new(0.18, 0.96, 0.21).normalize();
    let sun = Vec3::new(0.62, 0.45, -0.64).normalize();
    let mut star_density = if u_cam < -20.0 {
        0.0
    } else if u_cam > 26.5 {
        0.25
    } else {
        1.0
    };

    if doc.active_zone_index != u16::MAX {
        match experience_id {
            9 => {
                if doc.active_zone_index < 8 {
                    cmb_alpha = 0.0;
                } else {
                    cmb_alpha = cmb_alpha.max(0.62);
                }
                if doc.active_zone_index >= 7 {
                    star_density = 0.0;
                }
            }
            10 => {
                star_density = 0.0;
                cmb_alpha = if doc.active_zone_index <= 1 {
                    cmb_alpha.max(0.65)
                } else {
                    0.0
                };
                inflation_alpha = if doc.active_zone_index >= 7 {
                    inflation_alpha.max(0.55)
                } else {
                    0.0
                };
            }
            13 => {
                galaxy_plane = 0.0;
                star_density = 0.0;
                quantum_void_alpha = 0.0;
            }
            14 => {
                galaxy_plane = 0.0;
                star_density = 0.0;
                if doc.active_zone_index == 11 {
                    quantum_void_alpha *= 0.35;
                } else {
                    quantum_void_alpha *= 0.22;
                }
            }
            21 => {
                if matches!(doc.active_zone_index, 6..=10) {
                    galaxy_plane = 0.0;
                    star_density *= 0.25;
                }
            }
            22 => {
                if matches!(doc.active_zone_index, 19 | 20 | 27) {
                    galaxy_plane *= 0.35;
                    star_density *= 0.65;
                }
            }
            _ => {}
        }
    }

    FarShellUniforms {
        regime_weights: [galaxy_plane, cmb_alpha, inflation_alpha, lotus_alpha],
        galaxy_north: [north.x, north.y, north.z, star_density],
        solar_dir: [sun.x, sun.y, sun.z, if u_cam > 6.5 && u_cam < 13.5 { 0.45 } else { 0.1 }],
        extra_weights: [quantum_void_alpha, cosmic_web_sky_alpha, 1.0, 0.0],
    }
}

fn emit_visible_entity(
    vis: &VisibleEntity,
    input: &SceneBuildInput,
    heroes_surface: &mut Vec<HeroVolumeInstance>,
    heroes_volume: &mut Vec<HeroVolumeInstance>,
    sprites: &mut Vec<SpriteInstance>,
) {
    let pos = vis.pose.pos;
    if !pos.is_finite() || pos.length_squared() < 1e-8 {
        return;
    }
    let rot = vis.pose.orient;
    let rot_arr = [rot.x, rot.y, rot.z, rot.w];
    let accent = experience_accent_linear(vis.experience_id);
    let is_focused = input.focused_entity_id == Some(vis.id);
    let dwell = if is_focused {
        input.diegetic.dwell_progress.clamp(0.0, 1.0)
    } else {
        0.0
    };
    let sun_dir = Vec3::new(0.62, 0.45, -0.64).normalize();

    match vis.kind {
        EntityKind::ExperienceAnchor => {
            let max_r = vis.pose.rel_scale.clamp(0.45, 24.0);
            let dist = pos.length();
            let forward_cos = pos.normalize_or_zero().dot(input.camera.cam_forward);
            let isolated_active = matches!(input.active_experience_id, 1 | 2 | 11 | 15..=25);
            // Do not allow sibling/coaxial or peripheral corner-clipping experience shells to photobomb.
            let coaxial_other = vis.experience_id != input.active_experience_id
                && (vis.lod != LodTier::Hero
                    || isolated_active
                    || dist <= max_r * 1.85
                    || forward_cos < 0.85);
            if vis.lod == LodTier::Hero && !coaxial_other {
                // When zoomed deeper than Planck anchor scale (e.g. u = -35 vs anchor -30), keep Exp 14 centered in view.
                let (hero_pos, hero_scale) =
                    if vis.experience_id == 14 && input.active_experience_id == 14 && dist > 40.0 {
                        (input.camera.cam_forward * 2.95, 1.45)
                    } else {
                        (pos, vis.pose.rel_scale)
                    };
                emit_doc_stage_hero(
                    vis.experience_id,
                    hero_pos,
                    hero_scale,
                    rot_arr,
                    sun_dir,
                    dwell,
                    input,
                    heroes_surface,
                    heroes_volume,
                    sprites,
                );
            } else if !isolated_active && dist >= 0.75 {
                let r_sprite = (vis.pose.rel_scale * 0.08).clamp(0.004, (dist * 0.012).min(0.055));
                let fade = smoothstep(0.75, 1.60, dist);
                sprites.push(SpriteInstance {
                    pos_size: [pos.x, pos.y, pos.z, r_sprite],
                    axis_aspect: [0.0, 1.0, 0.0, 1.0],
                    color_alpha: [accent.x * 1.15, accent.y * 1.15, accent.z * 1.15, 0.62 * fade],
                    tags_f: [0.0, vis.experience_id as f32 * 0.3, 0.0, 0.0],
                });
            }
        }
        EntityKind::ChromaticSample => {
            let dist = pos.length().max(1e-4);
            if dist < 0.35 {
                return;
            }
            // Crisp chromatic dispersion sparkle beads around Experience 01 Equilateral Prism
            let hue = (vis.feature_index as f32 * 30.0) / 360.0;
            let rgb = hsl_linear(hue, 0.94, 0.58);
            let r_sprite = vis.pose.rel_scale.clamp(0.0035, (dist * 0.011).min(0.022));
            let fade = smoothstep(0.35, 0.90, dist);
            sprites.push(SpriteInstance {
                pos_size: [pos.x, pos.y, pos.z, r_sprite],
                axis_aspect: [0.0, 1.0, 0.0, 1.0],
                color_alpha: [rgb.x * 1.25, rgb.y * 1.25, rgb.z * 1.25, 0.78 * fade],
                tags_f: [0.0, hue, 0.0, 0.0],
            });
        }
        EntityKind::MusicalFilament => {
            let dist = pos.length().max(1e-4);
            if dist < 0.35 {
                return;
            }
            // Delicate harmonic shimmer node alongside Experience 02 48-String Cymatic Harp
            let timbre = (vis.zone_index % 4) as f32;
            let hue = ((vis.feature_index % 12) as f32) / 12.0;
            let rgb = hsl_linear(hue, 0.95, 0.6);
            let r_sprite = vis.pose.rel_scale.clamp(0.003, (dist * 0.010).min(0.016));
            let fade = smoothstep(0.35, 0.90, dist);
            sprites.push(SpriteInstance {
                pos_size: [pos.x, pos.y, pos.z, r_sprite],
                axis_aspect: [0.0, 1.0, 0.0, 1.45],
                color_alpha: [rgb.x * 1.15, rgb.y * 1.15, rgb.z * 1.15, 0.60 * fade],
                tags_f: [1.0, hue, timbre, 0.0],
            });
        }
        EntityKind::CelestialBody | EntityKind::GalacticLandmark => {
            emit_celestial_or_landmark(
                vis,
                input,
                sun_dir,
                dwell,
                heroes_surface,
                heroes_volume,
                sprites,
            );
        }
        _ => {
            let dist = pos.length().max(1e-4);
            if dist < 0.65 {
                return;
            }
            let kind_f = match vis.experience_id {
                7 | 8 | 17 | 20 | 24 => 1.0,
                _ => 0.0,
            };
            let max_ang = if kind_f > 0.5 {
                (dist * 0.013).min(0.042)
            } else {
                (dist * 0.011).min(0.048)
            };
            let r_sprite = vis.pose.rel_scale.clamp(0.004, max_ang);
            let aspect = if kind_f > 0.5 { 1.35 } else { 1.0 };
            let fade = smoothstep(0.65, 1.45, dist);
            sprites.push(SpriteInstance {
                pos_size: [pos.x, pos.y, pos.z, r_sprite],
                axis_aspect: [0.0, 1.0, 0.0, aspect],
                color_alpha: [accent.x * 1.05, accent.y * 1.05, accent.z * 1.05, 0.58 * fade],
                tags_f: [kind_f, (vis.id % 32) as f32 * 0.2, 0.0, 0.0],
            });
        }
    }
}

fn emit_fallback_hero(
    experience_id: u8,
    _u_cam: f32,
    input: &SceneBuildInput,
    heroes_surface: &mut Vec<HeroVolumeInstance>,
    heroes_volume: &mut Vec<HeroVolumeInstance>,
    sprites: &mut Vec<SpriteInstance>,
) {
    let center = input.camera.cam_forward * 3.2;
    let rot = Quat::IDENTITY;
    emit_doc_stage_hero(
        experience_id,
        center,
        1.5,
        [rot.x, rot.y, rot.z, rot.w],
        Vec3::new(0.62, 0.45, -0.64).normalize(),
        input.diegetic.dwell_progress,
        input,
        heroes_surface,
        heroes_volume,
        sprites,
    );
}

fn emit_diegetic_ui(
    diegetic: &DiegeticUiInput,
    time_sec: f32,
    accent: Vec3,
    sprites: &mut Vec<SpriteInstance>,
) {
    if diegetic.show_focus_shimmer && diegetic.dwell_progress > 0.06 {
        let dir = diegetic.focus_ray.normalize_or(Vec3::NEG_Z);
        let p = dir * 2.2;
        let radius = 0.045 + 0.03 * diegetic.dwell_progress;
        sprites.push(SpriteInstance {
            pos_size: [p.x, p.y, p.z, radius],
            axis_aspect: [0.0, 1.0, 0.0, 1.0],
            color_alpha: [
                accent.x * (0.9 + diegetic.dwell_progress),
                accent.y * (0.9 + diegetic.dwell_progress),
                accent.z * (0.9 + diegetic.dwell_progress),
                0.72 * diegetic.dwell_progress,
            ],
            tags_f: [4.0, time_sec * 0.4, 1.0, 0.0],
        });
    }

    if let Some(wrist) = diegetic.wrist_pos {
        // 9 speed beads: [1.0..=3.0] orbiting wrist ring (solid pearl = inactive, crisp jewel halo = active)
        let ring_r = 0.052;
        let active_idx = (((diegetic.playback_rate - 1.0) * 4.0).round() as i32).clamp(0, 8) as usize;
        for idx in 0..9 {
            let frac = idx as f32 / 9.0;
            let ang = frac * std::f32::consts::TAU;
            let local = Vec3::new(ang.cos() * ring_r, 0.025, ang.sin() * ring_r);
            let p = wrist + diegetic.wrist_orient * local;
            let is_active = idx == active_idx;
            let col = if is_active {
                Vec3::new(2.1, 1.55, 0.52)
            } else {
                accent * 0.62
            };
            sprites.push(SpriteInstance {
                pos_size: [p.x, p.y, p.z, if is_active { 0.0072 } else { 0.0046 }],
                axis_aspect: [0.0, 1.0, 0.0, 1.0],
                color_alpha: [col.x, col.y, col.z, if is_active { 0.98 } else { 0.78 }],
                tags_f: [4.0, frac, if is_active { 1.0 } else { 0.0 }, 0.0],
            });
        }
        // Subtle locale triad pearl anchored on inner ring crest (0=ES amber-coral, 1=EN cyan, 2=FI mint)
        let locale_col = match diegetic.locale_idx % 3 {
            0 => Vec3::new(1.85, 0.85, 0.35),
            1 => Vec3::new(0.35, 1.35, 1.95),
            _ => Vec3::new(0.45, 1.75, 1.05),
        };
        let locale_pos = wrist + diegetic.wrist_orient * Vec3::new(0.0, 0.038, -ring_r * 0.68);
        sprites.push(SpriteInstance {
            pos_size: [locale_pos.x, locale_pos.y, locale_pos.z, 0.0052],
            axis_aspect: [0.0, 1.0, 0.0, 1.0],
            color_alpha: [locale_col.x, locale_col.y, locale_col.z, 0.92],
            tags_f: [4.0, diegetic.locale_idx as f32 * 0.33, 0.0, 0.0],
        });
    }
}

/// Linear HDR palette signature mapped uniquely across all 25 experiences (`01..=25`).
pub fn experience_accent_linear(experience_id: u8) -> Vec3 {
    match experience_id {
        1 => Vec3::new(1.35, 0.45, 1.45),  // Colors prism magenta-violet
        2 => Vec3::new(0.45, 1.25, 1.55),  // Music harmonic cyan
        3 => Vec3::new(0.24, 0.78, 1.38),  // Deep Earth sapphire-aurora
        4 => Vec3::new(1.45, 0.92, 0.34),  // Solar System golden amber
        5 => Vec3::new(0.55, 0.85, 1.45),  // Milky Way stellar blue
        6 => Vec3::new(0.75, 0.72, 1.38),  // Local Group periwinkle
        7 => Vec3::new(1.28, 0.78, 0.42),  // Laniakea Great Attractor gold
        8 => Vec3::new(0.85, 0.58, 1.65),  // Cosmic Web ultraviolet
        9 => Vec3::new(1.15, 0.75, 0.52),  // Observable Horizon warm copper
        10 => Vec3::new(1.55, 0.58, 0.35), // Early Universe primordial fire
        11 => Vec3::new(0.48, 1.32, 1.15), // Cosmic Evidence teal-emerald
        12 => Vec3::new(0.58, 0.42, 1.35), // Future of Universe deep indigo
        13 => Vec3::new(0.28, 1.35, 1.48), // Matter & Energy quark cyan
        14 => Vec3::new(1.22, 0.48, 1.62), // Spacetime curved violet
        15 => Vec3::new(0.28, 1.42, 0.92), // Complexity reaction-diffusion mint
        16 => Vec3::new(0.32, 1.38, 0.65), // Life biosphere bioluminescent green
        17 => Vec3::new(1.35, 0.65, 1.45), // Mind connectome cathedral orchid
        18 => Vec3::new(1.48, 1.05, 0.42), // Culture golden nocturnal grid
        19 => Vec3::new(0.42, 1.18, 1.68), // Technology cybernetic azure
        20 => Vec3::new(1.12, 1.32, 0.52), // Futures solar-sage
        21 => Vec3::new(0.38, 1.45, 1.12), // Cosmic Life exo-chlorophyll
        22 => Vec3::new(1.55, 0.82, 0.48), // Cosmic Contact beacon amber
        23 => Vec3::new(0.95, 0.85, 1.55), // Perspectives lotus silver-blue
        24 => Vec3::new(1.42, 0.72, 1.28), // Cosmic Meaning lotus rose
        _ => Vec3::new(1.45, 1.25, 0.88),  // Final Horizon incandescent pearl
    }
}

pub(crate) fn hsl_linear(h_01: f32, s: f32, l: f32) -> Vec3 {
    let hue = h_01.rem_euclid(1.0);
    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;
    let hue2rgb = |mut t: f32| {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            p + (q - p) * 6.0 * t
        } else if t < 0.5 {
            q
        } else if t < 2.0 / 3.0 {
            p + (q - p) * (2.0 / 3.0 - t) * 6.0
        } else {
            p
        }
    };
    Vec3::new(
        hue2rgb(hue + 1.0 / 3.0).powf(2.2),
        hue2rgb(hue).powf(2.2),
        hue2rgb(hue - 1.0 / 3.0).powf(2.2),
    )
}
