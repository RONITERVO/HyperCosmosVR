//! Documentary visual stage builders for all 25 HyperCosmosVR experiences (`01..=25`),
//! packing live `DocumentaryFocusState` telemetry cleanly into GPU hero/sprite buffers
//! with strict OLED `#000000` void preservation and zero corner photobombing.

use cosmos_spatial::{EntityKind, LodTier, VisibleEntity};
use glam::{Quat, Vec3};

use crate::types::{HeroVolumeInstance, SpriteInstance};
use crate::universe_scene::{DocumentaryFocusState, SceneBuildInput, experience_accent_linear, hsl_linear};

/// Encodes active zone (`u16`), feature (`u8`), `scene_index + 1` (`u8`), narration cue,
/// active weight, and domain submode into `HeroVolumeInstance::kind_tags`.
pub fn pack_doc_tags(
    hero_kind: u32,
    vid: u32,
    base_flags: u32,
    submode: u32,
    doc: &DocumentaryFocusState,
) -> [u32; 4] {
    let z_u16 = doc.active_zone_index as u32 & 0xFFFF;
    let f_u8 = (doc.active_feature_index as u32) & 0xFF;
    let sc_u8 = ((doc.scene_index + 1).clamp(0, 254) as u32) & 0xFF;
    let packed_z = z_u16 | (f_u8 << 16) | (sc_u8 << 24);

    let cue_u8 = (doc.narration_cue_progress.clamp(0.0, 1.0) * 255.0).round() as u32;
    let wt_u8 = (doc.narration_active_weight.clamp(0.0, 1.0) * 255.0).round() as u32;
    let packed_w =
        (base_flags & 0xFF) | ((cue_u8 & 0xFF) << 8) | ((wt_u8 & 0xFF) << 16) | ((submode & 0xFF) << 24);

    [hero_kind, vid, packed_z, packed_w]
}

/// Resolves the deterministic shader sub-scene variant ID (`vid`) from explicit
/// `doc.active_zone_index` overrides (for catalog entries lacking per-zone `scene_index`),
/// falling back to `doc.scene_index` when provided.
pub fn doc_stage_variant_id(experience_id: u8, doc: &DocumentaryFocusState) -> u32 {
    let z = doc.active_zone_index;
    let sc = if doc.scene_index >= 0 {
        Some(doc.scene_index as u32)
    } else {
        None
    };
    match experience_id {
        13 => match z {
            2 => 10, // H2O + Benzene Molecules (07c)
            3 => 11, // Hydrogen Electron Orbitals (07)
            6 => 12, // QCD Quarks & Gluon Flux Bag (07b)
            _ => 10 + (sc.unwrap_or(if z != u16::MAX { z as u32 } else { 0 }) % 5),
        },
        14 => match z {
            8 => 22,  // Binary BH Inspiral & Quadrupole Gravitational Waves (08b)
            11 => 23, // Planck Spin-Foam & Calabi-Yau String Loops (08)
            _ => 20 + (sc.unwrap_or(if z != u16::MAX { z as u32 } else { 0 }) % 4),
        },
        15 => 10 + (sc.unwrap_or(if z != u16::MAX { z as u32 } else { 0 }) % 4),
        16 => match z {
            0 | u16::MAX => 23, // 3D Phylogenetic Tree of Life (18)
            4 => 20,            // B-DNA Double Helix + Cell Membrane (18b)
            _ => 20 + (sc.unwrap_or(z as u32) % 5),
        },
        17 => 30 + (sc.unwrap_or(if z != u16::MAX { z as u32 } else { 0 }) % 5),
        11 => 110 + (sc.unwrap_or(if z != u16::MAX { z as u32 } else { 0 }) % 4),
        12 => 120 + (sc.unwrap_or(if z != u16::MAX { z as u32 } else { 0 }) % 5),
        18 => 180 + (sc.unwrap_or(if z != u16::MAX { z as u32 } else { 0 }) % 7),
        19 => match z {
            0 | u16::MAX => 190, // Kinetic Involute Gears (20)
            16 => 194,           // Etched Silicon Die + Transformer Attention Web (20b)
            _ => 190 + (sc.unwrap_or(z as u32) % 7),
        },
        20 => match z {
            0 | u16::MAX => 200, // Multicolor Bifurcating Future Probability Arcs (21)
            16 => 202,           // Rotating O'Neill Cylinder + Dyson Mirror Swarm (21b)
            _ => 200 + (sc.unwrap_or(z as u32) % 4),
        },
        21 => match z {
            12 => 212, // Cracked Europa Ice Crust + Hydrothermal Cryo-Plumes (22b)
            _ => 210,  // Goldilocks Habitable Ring (22)
        },
        22 => match z {
            12 => 221, // Prime 2,3,5,7,11 Arecibo Cipher Rings + Pulsar Rays (10b)
            _ => 220,  // Parabolic SETI Dish + Laser Beacon (10)
        },
        23 => 230,
        24 => match z {
            24 => 241, // Self-Valuing Ethical Balance Scales (24b)
            _ => 240,  // Cosmic Astrolabe & Planetary Shield Arc + Scales (24)
        },
        25 => 250,
        _ => experience_id as u32,
    }
}

/// Blends active documentary zone accent RGB smoothly onto fallback secondary accent.
fn active_zone_color(doc: &DocumentaryFocusState, fallback: Vec3) -> Vec3 {
    let za = Vec3::from(doc.zone_accent_rgb);
    if za.length_squared() > 1e-4 {
        fallback.lerp(za, 0.72)
    } else {
        fallback
    }
}

/// Normalizes App 01 `[H_deg, S, L, mat]` & App 02 `[midi_48..95, timbre_0..3, freq, amp]`
/// into shader-ready `[0..1]` ranges inside `phys_b` (`kind == 5u / 6u`).
fn normalized_colors_telemetry(doc: &DocumentaryFocusState) -> ([f32; 4], u32) {
    let raw_h = doc.telemetry[0];
    let h_01 = if raw_h > 1.01 {
        (raw_h / 360.0).rem_euclid(1.0)
    } else {
        raw_h.clamp(0.0, 1.0)
    };
    let s = doc.telemetry[1].clamp(0.0, 1.0);
    let l = doc.telemetry[2].clamp(0.0, 1.0);
    let mat = (doc.telemetry[3].round().clamp(0.0, 3.0)) as u32;
    let has_hsl = s > 0.01 || l > 0.01 || mat > 0 || doc.narration_active_weight > 0.02;
    ([h_01, if has_hsl { s } else { 0.85 }, if has_hsl { l } else { 0.55 }, 1.0], if has_hsl { mat.max(1) } else { 0 })
}

fn normalized_music_telemetry(doc: &DocumentaryFocusState, synth_b: [f32; 4]) -> ([f32; 4], u32) {
    let raw_midi = doc.telemetry[0];
    let pitch_01 = if raw_midi > 1.01 {
        ((raw_midi - 48.0) / 47.0).clamp(0.0, 1.0)
    } else if raw_midi > 0.0 {
        raw_midi.clamp(0.0, 1.0)
    } else {
        synth_b[0].clamp(0.0, 1.0)
    };
    let timbre = if doc.telemetry[1] >= 0.5 || doc.telemetry[2] > 1.0 {
        doc.telemetry[1].round().clamp(0.0, 3.0) as u32
    } else {
        (synth_b[1].round().clamp(0.0, 3.0)) as u32
    };
    let amp = doc.telemetry[3].max(synth_b[2]).clamp(0.0, 1.0);
    ([pitch_01, timbre as f32, amp, 1.0], timbre)
}

/// Maps `cosmos_spatial` solar bodies (`SolarSystem` `exp 4` / `CelestialBody`) cleanly
/// onto their analytical surface/ring shader variants.
pub fn emit_celestial_or_landmark(
    vis: &VisibleEntity,
    input: &SceneBuildInput,
    sun_dir: Vec3,
    dwell: f32,
    heroes_surface: &mut Vec<HeroVolumeInstance>,
    heroes_volume: &mut Vec<HeroVolumeInstance>,
    sprites: &mut Vec<SpriteInstance>,
) {
    let pos = vis.pose.pos;
    let dist = pos.length().max(1e-4);
    let accent = experience_accent_linear(vis.experience_id);
    let sun = [sun_dir.x, sun_dir.y, sun_dir.z, 1.0];
    let rot = vis.pose.orient;
    let rot_arr = [rot.x, rot.y, rot.z, rot.w];

    if vis.kind == EntityKind::CelestialBody {
        // Prevent Solar System bodies (which share EARTH_ORBIT / lower shell pad)
        // from leaking surface heroes or fallback sprites into Deep Earth (03),
        // Culture (18), Technology (19), or any non-Solar experience.
        if input.active_experience_id != 4 || vis.experience_id != 4 {
            return;
        }
        if matches!(vis.lod, LodTier::Hero | LodTier::Impostor | LodTier::MacroCloud) {
            // 12 Solar System zones (0:Sun, 1:Mercury, 2:Venus, 3:Earth, 4:Mars,
            // 5:Asteroid Belt, 6:Jupiter, 7:Saturn, 8:Uranus, 9:Neptune, 10:Kuiper Belt, 11:Pluto)
            let body_idx = vis.zone_index as u32;
            let max_r = (dist * 0.42).clamp(0.04, 10.0);
            let floor_r = match body_idx {
                0 => (dist * 0.014).clamp(0.04, 0.14),
                6 | 7 => (dist * 0.0072).clamp(0.032, 0.088),
                _ => (dist * 0.0048).clamp(0.022, 0.056),
            };
            let r_b = vis.pose.rel_scale.max(floor_r).clamp(0.022, max_r);
            match body_idx {
                0 => {
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b],
                        rot_quat: rot_arr,
                        color_a: [1.05, 0.68, 0.34, 0.92],
                        color_b: [1.45, 0.48, 0.10, 1.0],
                        phys_a: [input.time_sec * 0.04, 0.0, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(1, 1, 0, 0, &input.documentary),
                    });
                    heroes_volume.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b * 1.30],
                        rot_quat: rot_arr,
                        color_a: [1.25, 0.62, 0.22, 0.88],
                        color_b: [1.45, 0.42, 0.10, 0.85],
                        phys_a: [input.time_sec * 0.04, 0.0, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(3, 4, 0, 0, &input.documentary),
                    });
                    return;
                }
                3 => {
                    // Earth inside Solar System
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b],
                        rot_quat: rot_arr,
                        color_a: [1.0, 1.0, 1.0, 1.15],
                        color_b: [0.16, 0.58, 1.45, 1.0],
                        phys_a: [input.time_sec * 0.05, 0.409, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(0, 0, 0, 0, &input.documentary),
                    });
                    return;
                }
                5 | 10 => {
                    // Asteroid Belt (5: Ceres + rocky debris ring) & Kuiper Belt (10: icy dwarf + icy ring)
                    let is_kuiper = body_idx == 10;
                    let body_r = (r_b * 0.44).clamp(0.03, 1.2);
                    let ring_tilt = Quat::from_euler(glam::EulerRot::XYZ, 0.35, 0.15, 0.08);
                    let rq = [ring_tilt.x, ring_tilt.y, ring_tilt.z, ring_tilt.w];
                    let (vid, ca, cb, r_col_a, r_col_b, shard_col) = if is_kuiper {
                        (
                            10,
                            [0.72, 0.84, 0.95, 1.10],
                            [0.22, 1.25, 1.65, 1.0],
                            [0.45, 0.85, 1.15, 0.62],
                            [0.62, 0.96, 1.25, 0.62],
                            [0.52, 0.88, 1.22, 0.90],
                        )
                    } else {
                        (
                            3,
                            [0.54, 0.49, 0.43, 1.05],
                            [0.30, 0.27, 0.23, 1.0],
                            [0.76, 0.62, 0.46, 0.58],
                            [0.54, 0.46, 0.36, 0.58],
                            [0.78, 0.64, 0.48, 0.92],
                        )
                    };
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, body_r],
                        rot_quat: rot_arr,
                        color_a: ca,
                        color_b: cb,
                        phys_a: [input.time_sec * 0.04, 0.12, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(1, vid, 0, 0, &input.documentary),
                    });
                    heroes_volume.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, body_r * 2.35],
                        rot_quat: rq,
                        color_a: r_col_a,
                        color_b: r_col_b,
                        phys_a: [0.0, 0.35, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(2, 1, 0, 0, &input.documentary),
                    });
                    let spread = r_b.clamp(0.08, 1.6);
                    for j in 0..7 {
                        let a = (j as f32 / 7.0) * std::f32::consts::TAU + (vis.id % 17) as f32 * 0.37;
                        let rad = spread * (0.42 + 0.14 * ((j % 3) as f32));
                        let off = Vec3::new(a.cos() * rad, ((j as f32 * 1.9).sin()) * rad * 0.18, a.sin() * rad);
                        let rock_sz = (spread * (0.045 + 0.015 * ((j % 3) as f32))).clamp(0.006, 0.065);
                        sprites.push(SpriteInstance {
                            pos_size: [pos.x + off.x, pos.y + off.y, pos.z + off.z, rock_sz],
                            axis_aspect: [0.0, 1.0, 0.0, 1.0],
                            color_alpha: shard_col,
                            tags_f: [2.0, ((vis.id + j as u64) % 64) as f32 * 0.09, if is_kuiper { 0.5 } else { 0.0 }, 0.0],
                        });
                    }
                    return;
                }
                7 => {
                    // Saturn + Cassini/Encke Ring Plane
                    let ring_tilt = Quat::from_euler(glam::EulerRot::XYZ, 0.47, 0.2, 0.15);
                    let rq = [ring_tilt.x, ring_tilt.y, ring_tilt.z, ring_tilt.w];
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b],
                        rot_quat: rq,
                        color_a: [0.88, 0.75, 0.50, 1.1],
                        color_b: [0.65, 0.58, 0.44, 1.0],
                        phys_a: [0.0, 0.47, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(1, 2, 0, 0, &input.documentary),
                    });
                    heroes_volume.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b * 2.25],
                        rot_quat: rq,
                        color_a: [0.95, 0.84, 0.62, 1.15],
                        color_b: [0.48, 0.56, 0.78, 1.0],
                        phys_a: [0.0, 0.47, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(2, 2, 0, 0, &input.documentary),
                    });
                    return;
                }
                8 => {
                    // Uranus + Delicate Ring
                    let tilt_u = Quat::from_euler(glam::EulerRot::XYZ, 1.71, 0.1, 0.0);
                    let uq = [tilt_u.x, tilt_u.y, tilt_u.z, tilt_u.w];
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b],
                        rot_quat: uq,
                        color_a: [0.42, 0.85, 0.94, 1.1],
                        color_b: [0.58, 0.96, 1.05, 1.0],
                        phys_a: [0.0, 1.71, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(1, 7, 0, 0, &input.documentary),
                    });
                    heroes_volume.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b * 1.85],
                        rot_quat: uq,
                        color_a: [0.48, 0.85, 0.95, 0.75],
                        color_b: [0.65, 0.95, 1.1, 0.75],
                        phys_a: [0.0, 1.71, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(2, 1, 0, 0, &input.documentary),
                    });
                    return;
                }
                _ => {
                    let vid = match body_idx {
                        1 => 3,  // Mercury
                        2 => 4,  // Venus
                        4 => 5,  // Mars
                        6 => 6,  // Jupiter
                        9 => 8,  // Neptune
                        11 => 9, // Pluto
                        _ => 10, // Europa / icy moons
                    };
                    let (ca, cb) = match vid {
                        3 => ([0.52, 0.48, 0.44, 1.05], [0.28, 0.26, 0.24, 1.0]),
                        4 => ([0.95, 0.78, 0.45, 1.08], [0.62, 0.42, 0.20, 1.0]),
                        5 => ([0.88, 0.36, 0.16, 1.10], [0.48, 0.18, 0.08, 1.0]),
                        6 => ([0.88, 0.72, 0.54, 1.12], [0.78, 0.44, 0.24, 1.0]),
                        8 => ([0.16, 0.42, 1.05, 1.15], [0.35, 0.72, 1.35, 1.0]),
                        9 => ([0.78, 0.62, 0.50, 1.05], [0.94, 0.88, 0.78, 1.0]),
                        _ => ([0.72, 0.84, 0.95, 1.10], [0.22, 1.25, 1.65, 1.0]),
                    };
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b],
                        rot_quat: rot_arr,
                        color_a: ca,
                        color_b: cb,
                        phys_a: [input.time_sec * 0.03, 0.1, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(1, vid, 0, 0, &input.documentary),
                    });
                    return;
                }
            }
        }
    }

    if vis.kind == EntityKind::GalacticLandmark {
        if vis.experience_id != input.active_experience_id
            && matches!(input.active_experience_id, 1..=4 | 11..=25)
        {
            return;
        }
        if vis.lod == LodTier::Hero
            && (vis.experience_id == input.active_experience_id
                || matches!(input.active_experience_id, 5..=9))
        {
            // Promote iconic landmarks (Heliopause=20, Orion/Eagle Pillars=21, Local Bubble=22, Virgo/M87)
            let vid = match (vis.experience_id, vis.zone_index % 3) {
                (5, 0) => 20,
                (5, 1) => 21,
                (5, _) => 22,
                (6, _) => 6,
                (7, _) => 7,
                (8, _) => 8,
                _ => vis.experience_id as u32,
            };
            let max_r = (dist * 0.40).clamp(0.05, 16.0);
            let r_l = vis.pose.rel_scale.clamp(0.05, max_r);
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_l],
                rot_quat: rot_arr,
                color_a: [accent.x, accent.y, accent.z, 1.08],
                color_b: [0.35, 0.95, 1.55, 0.92],
                phys_a: [input.time_sec * 0.15, 0.0, 0.8, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(3, vid, 0, 0, &input.documentary),
            });
            return;
        }
    }

    // Crisp pinpoint sprite fallback for distant celestial bodies & landmarks
    let is_solar_belt =
        vis.kind == EntityKind::CelestialBody && vis.experience_id == 4 && matches!(vis.zone_index, 5 | 10);
    let kind_f = if is_solar_belt {
        2.0 // Rocky / icy belt shard sprite
    } else {
        match vis.experience_id {
            4 => 5.0,               // Analytic shaded micro-planet / icy moon
            7 | 8 | 17 | 20 => 1.0, // Directional cosmic/neural filament
            _ => 0.0,               // Blackbody star / beacon gem
        }
    };
    let phase = (vis.id % 64) as f32 * 0.07;
    let sub_p = if is_solar_belt {
        if vis.zone_index == 10 { 0.5 } else { 0.0 }
    } else {
        match vis.experience_id {
            4 => 0.48,  // shaded moon/planet sprite
            20 => 0.88, // hex Dyson glint
            21 => 0.45,
            22 => 0.85,
            _ => 0.1,
        }
    };
    let dist_s = pos.length().max(1e-4);
    if dist_s < 0.65 && !is_solar_belt && vis.experience_id != 4 {
        return;
    }
    let max_ang = if is_solar_belt {
        (dist_s * 0.015).min(0.055)
    } else if vis.experience_id == 4 {
        (dist_s * 0.035).min(0.22)
    } else {
        (dist_s * 0.011).min(0.048)
    };
    let r_spr = vis.pose.rel_scale.clamp(0.004, max_ang);
    let fade = if vis.experience_id == 4 {
        1.0
    } else {
        ((dist_s - 0.65) / 0.85).clamp(0.0, 1.0)
    };
    sprites.push(SpriteInstance {
        pos_size: [pos.x, pos.y, pos.z, r_spr],
        axis_aspect: if kind_f == 1.0 {
            [0.0, 1.0, 0.0, 1.40]
        } else {
            [0.0, 1.0, 0.0, 1.0]
        },
        color_alpha: [accent.x * 1.15, accent.y * 1.15, accent.z * 1.15, 0.72 * fade],
        tags_f: [kind_f, phase, sub_p, 0.0],
    });
}

/// Emits multi-part documentary hero + prop assemblies across all 25 experiences (`01..=25`).
#[allow(clippy::too_many_arguments)]
pub fn emit_doc_stage_hero(
    experience_id: u8,
    pos: Vec3,
    rel_scale: f32,
    rot_quat: [f32; 4],
    sun_dir: Vec3,
    dwell: f32,
    input: &SceneBuildInput,
    heroes_surface: &mut Vec<HeroVolumeInstance>,
    heroes_volume: &mut Vec<HeroVolumeInstance>,
    sprites: &mut Vec<SpriteInstance>,
) {
    let doc = &input.documentary;
    let accent = experience_accent_linear(experience_id);
    let zone_c = active_zone_color(doc, accent);
    let u_cam = input.cam_pose.scale_exp as f32;
    let sun = [sun_dir.x, sun_dir.y, sun_dir.z, 1.0];

    let view_fwd = pos.normalize_or(input.camera.cam_forward);
    let stage_right = view_fwd.cross(Vec3::Y).normalize_or(Vec3::X);
    let stage_up = stage_right.cross(view_fwd).normalize_or(Vec3::Y);

    // Zero-roll horizontal viewer-facing basis (+Z toward camera XZ, +X along stage_right, +Y world up)
    // so raymarched sculptural dioramas never collapse edge-on when approached from side azimuths.
    let fwd_h = Vec3::new(-pos.x, 0.0, -pos.z).normalize_or(-view_fwd);
    let right_h = Vec3::Y.cross(fwd_h).normalize_or(stage_right);
    let up_h = fwd_h.cross(right_h).normalize_or(Vec3::Y);
    let face_q = Quat::from_mat3(&glam::Mat3::from_cols(right_h, up_h, fwd_h));
    let face_quat = [face_q.x, face_q.y, face_q.z, face_q.w];

    match experience_id {
        // --------------------------------------------------------------------
        // EXP 01: COLORS — Dispersion Prism + Interactive 3D HSL Bicone + 3 PBR Spheres
        // --------------------------------------------------------------------
        1 => {
            let max_r = (pos.length() * 0.42).clamp(0.04, 6.0);
            let r_p = (rel_scale * 0.72).clamp(0.04, max_r);
            let (hsl_phys, mat_submode) = normalized_colors_telemetry(doc);

            // Primary prism/cones/quantum/tetrahedral shader hero slightly left
            let left_p = pos - stage_right * (r_p * 0.42);
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [left_p.x, left_p.y, left_p.z, r_p * 0.68],
                rot_quat: face_quat,
                color_a: [1.2, 1.2, 1.2, 1.3],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 1.0],
                phys_a: [input.time_sec, 0.0, input.audio.synth_b[0], dwell],
                phys_b: hsl_phys,
                kind_tags: pack_doc_tags(5, 1, 0, mat_submode, doc),
            });

            // Sculpted 3D HSL Bicone + Live Interactive Telemetry Lock Ring slightly right
            let right_p = pos + stage_right * (r_p * 0.46);
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [right_p.x, right_p.y, right_p.z, r_p * 0.62],
                rot_quat: face_quat,
                color_a: [1.15, 1.15, 1.15, 1.25],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 1.0],
                phys_a: [input.time_sec, 0.0, 0.85, dwell],
                phys_b: hsl_phys,
                kind_tags: pack_doc_tags(5, 10, 0, mat_submode.max(1), doc),
            });

            // Three PBR Material Target Spheres cleanly arrayed below in the void (Gold, Iridescent Glass, Matte Clay)
            for mat_idx in 1u32..=3u32 {
                let x_off = (mat_idx as f32 - 2.0) * (r_p * 0.34);
                let mp = pos + stage_right * x_off - stage_up * (r_p * 0.56);
                let base_c = match mat_idx {
                    1 => [0.95, 0.72, 0.26, 1.15],
                    2 => [0.52, 0.92, 1.25, 1.15],
                    _ => [0.85, 0.38, 0.22, 1.10],
                };
                let studio_sun_v = Vec3::new(0.45, 0.62, 0.64).normalize();
                let studio_sun = [studio_sun_v.x, studio_sun_v.y, studio_sun_v.z, 1.0];
                heroes_surface.push(HeroVolumeInstance {
                    pos_radius: [mp.x, mp.y, mp.z, r_p * 0.14],
                    rot_quat: face_quat,
                    color_a: base_c,
                    color_b: [zone_c.x, zone_c.y, zone_c.z, 1.0],
                    phys_a: [input.time_sec * 0.2, 0.0, 0.0, if mat_submode == mat_idx { 0.8 } else { 0.0 }],
                    phys_b: studio_sun,
                    kind_tags: pack_doc_tags(1, 12, 0, mat_idx, doc),
                });
            }
        }

        // --------------------------------------------------------------------
        // EXP 02: MUSIC — Chladni Nodal Plate + 48-String Plucked Harp + 4 Timbre Chambers
        // --------------------------------------------------------------------
        2 => {
            let max_r = (pos.length() * 0.42).clamp(0.04, 6.0);
            let r_h = (rel_scale * 0.68).clamp(0.04, max_r);
            let (music_phys, active_timbre) = normalized_music_telemetry(doc, input.audio.synth_b);

            // Central Chladni Plate + 48-String Chromatic Harp
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_h * 0.76],
                rot_quat: face_quat,
                color_a: [accent.x, accent.y, accent.z, 1.28],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 1.0],
                phys_a: [input.time_sec, 0.0, 0.7 + input.audio.synth_b[2], dwell],
                phys_b: music_phys,
                kind_tags: pack_doc_tags(6, 1, 0, active_timbre, doc),
            });

            // 4 Architectural Timbre Chambers flanking the main harp (Cristal, Aire, Calido, Profundo)
            for ch in 0u32..4u32 {
                let vid = 11 + ch;
                let angle = (ch as f32 - 1.5) * 0.42;
                let side = if ch < 2 { -1.0 } else { 1.0 };
                let ring_y = if ch == 0 || ch == 3 { 0.34 } else { -0.30 };
                let cp = pos + stage_right * (side * r_h * 0.78 + angle * r_h * 0.12) + stage_up * (ring_y * r_h);
                let chamber_col = hsl_linear(0.55 - ch as f32 * 0.12, 0.90, 0.56);
                let is_sel = active_timbre == ch;
                heroes_volume.push(HeroVolumeInstance {
                    pos_radius: [cp.x, cp.y, cp.z, r_h * 0.28],
                    rot_quat: face_quat,
                    color_a: [chamber_col.x, chamber_col.y, chamber_col.z, if is_sel { 1.35 } else { 0.85 }],
                    color_b: [zone_c.x, zone_c.y, zone_c.z, 1.0],
                    phys_a: [input.time_sec, 0.0, if is_sel { 1.0 } else { 0.4 }, if is_sel { dwell.max(0.5) } else { 0.0 }],
                    phys_b: music_phys,
                    kind_tags: pack_doc_tags(6, vid, 0, ch, doc),
                });
            }
        }

        // --------------------------------------------------------------------
        // EXP 03: DEEP EARTH — 14-Zone Mantle/Core Cutaway + Auroral Magnetosphere + Orbital Ticks
        // --------------------------------------------------------------------
        3 => {
            if u_cam < 1.2 || (input.active_experience_id != 3 && u_cam < 5.2) {
                return;
            }
            let earth_solid_r = (6.371e6_f64 * 10.0_f64.powf(-input.cam_pose.scale_exp) / 0.84)
                .clamp(0.18_f64, ((rel_scale * 1.2) as f64).max(0.25)) as f32;
            let cutaway = if input.earth_cutaway_blend > 0.25
                || (input.active_experience_id == 3 && (5.2..=6.45).contains(&u_cam))
                || (doc.active_zone_index >= 7 && doc.active_zone_index <= 13)
            {
                1u32
            } else {
                0u32
            };
            heroes_surface.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, earth_solid_r],
                rot_quat,
                color_a: [1.0, 1.0, 1.0, 1.15],
                color_b: [0.16, 0.58, 1.45, 1.0],
                phys_a: [input.time_sec * 0.03, 0.409, 0.0, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(0, 0, cutaway, 0, doc),
            });
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, earth_solid_r * 1.14],
                rot_quat,
                color_a: [0.15, 2.2, 0.9, 1.2],
                color_b: [1.1, 0.24, 2.0, 1.0],
                phys_a: [input.time_sec * 0.03, 0.0, 0.0, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(0, 0, 0, 0, doc),
            });
            // Orbiting Moon impostor tucked safely into upper-left void (zero Thermosphere label overlap)
            let m_ang = input.time_sec * 0.14;
            let moon_p = pos
                - stage_right * (earth_solid_r * (2.15 + 0.12 * m_ang.cos()))
                + stage_up * (earth_solid_r * 1.15)
                + view_fwd * (earth_solid_r * 0.35 * m_ang.sin());
            sprites.push(SpriteInstance {
                pos_size: [moon_p.x, moon_p.y, moon_p.z, (earth_solid_r * 0.20).clamp(0.03, 0.42)],
                axis_aspect: [0.0, 1.0, 0.0, 1.0],
                color_alpha: [0.76, 0.78, 0.82, 0.95],
                tags_f: [5.0, 0.25, 0.48, 0.0],
            });
        }

        // --------------------------------------------------------------------
        // EXP 04: SOLAR SYSTEM — Analytical Hairline Keplerian Orbital Plane
        // --------------------------------------------------------------------
        4 => {
            if input.active_experience_id != 4 {
                let sun_r = (rel_scale * 0.18).clamp(0.24, 10.0);
                heroes_surface.push(HeroVolumeInstance {
                    pos_radius: [pos.x, pos.y, pos.z, sun_r],
                    rot_quat,
                    color_a: [1.05, 0.68, 0.34, 0.92],
                    color_b: [1.45, 0.48, 0.10, 1.0],
                    phys_a: [input.time_sec * 0.04, 0.0, 0.0, dwell],
                    phys_b: sun,
                    kind_tags: pack_doc_tags(1, 1, 0, 0, doc),
                });
                heroes_volume.push(HeroVolumeInstance {
                    pos_radius: [pos.x, pos.y, pos.z, sun_r * 1.30],
                    rot_quat,
                    color_a: [1.25, 0.62, 0.22, 0.88],
                    color_b: [1.45, 0.42, 0.10, 0.85],
                    phys_a: [input.time_sec * 0.04, 0.0, 0.0, dwell],
                    phys_b: sun,
                    kind_tags: pack_doc_tags(3, 4, 0, 0, doc),
                });
            }
            // Analytic ecliptic Keplerian orbital hairlines locked strictly to SOLAR_SYSTEM canonical scale (u=11.2)
            let sector_unit_render = 10.0_f64.powf(11.2 - input.cam_pose.scale_exp) as f32;
            let max_orbit_r = 11.2 * sector_unit_render;
            let disk_r = max_orbit_r.clamp(1.2, 48.0);
            let track_scale = (max_orbit_r / disk_r).max(1e-4);
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, disk_r],
                rot_quat,
                color_a: [0.36, 0.78, 1.15, 0.68],
                color_b: [1.18, 0.88, 0.44, 0.72],
                phys_a: [input.time_sec * 0.04, 0.0, track_scale, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(2, 4, 0, 0, doc),
            });
        }

        // --------------------------------------------------------------------
        // EXP 05 & 12: BLACK HOLES / MILKY WAY BAR / DYING DWARF
        // --------------------------------------------------------------------
        5 | 12 => {
            let max_r = (pos.length() * 0.46).clamp(0.04, 24.0);
            let r_bh = rel_scale.clamp(0.04, max_r);
            heroes_surface.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_bh],
                rot_quat,
                color_a: [0.0, 0.0, 0.0, 1.0],
                color_b: [0.0, 0.0, 0.0, 1.0],
                phys_a: [0.95, 0.0, 0.95, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(4, experience_id as u32, 2, 0, doc),
            });
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_bh],
                rot_quat,
                color_a: [1.25, 1.0, 0.88, 1.25],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 1.0],
                phys_a: [0.95, 0.25, 0.95, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(4, experience_id as u32, 2, 0, doc),
            });
            if experience_id == 12 {
                // Cooling Red/Black Dwarf + Future Arrow beside Hawking BH
                let dp = pos + stage_right * (r_bh * 1.15);
                heroes_surface.push(HeroVolumeInstance {
                    pos_radius: [dp.x, dp.y, dp.z, r_bh * 0.26],
                    rot_quat,
                    color_a: [0.95, 0.25, 0.08, 1.1],
                    color_b: [1.35, 0.42, 0.12, 0.9],
                    phys_a: [input.time_sec * 0.1, 0.0, 0.0, dwell],
                    phys_b: sun,
                    kind_tags: pack_doc_tags(1, 12, 0, 0, doc),
                });
            }
        }

        // --------------------------------------------------------------------
        // EXP 06..10: COSMOLOGICAL MACRO (LOCAL GROUP, LANIAKEA, WEB, OBSERVABLE, EARLY)
        // --------------------------------------------------------------------
        6..=10 => {
            let max_r = (pos.length() * 0.42).clamp(0.04, 24.0);
            let r_g = rel_scale.clamp(0.04, max_r);
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_g],
                rot_quat,
                color_a: [accent.x, accent.y, accent.z, 1.12],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 0.92],
                phys_a: [input.time_sec * 0.2, 0.0, 0.8, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(3, experience_id as u32, 0, 0, doc),
            });
        }

        // --------------------------------------------------------------------
        // EXP 13 & 14: MATTER/ENERGY & SPACETIME
        // --------------------------------------------------------------------
        13 | 14 => {
            let max_r = (pos.length() * 0.42).clamp(0.04, 6.0);
            let r_q = (rel_scale * 0.86).clamp(0.04, max_r);
            let vid = doc_stage_variant_id(experience_id, doc);
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_q],
                rot_quat: face_quat,
                color_a: [accent.x, accent.y, accent.z, 1.2],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 1.0],
                phys_a: [input.time_sec * 0.4, 0.0, 1.0, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(7, vid, 0, 0, doc),
            });
        }

        // --------------------------------------------------------------------
        // EXP 15, 16, 17: COMPLEXITY, LIFE & SCULPTED 3D ANATOMICAL MIND
        // --------------------------------------------------------------------
        15..=17 => {
            let max_r = (pos.length() * 0.42).clamp(0.04, 6.0);
            let r_b = (rel_scale * 0.86).clamp(0.04, max_r);
            let vid = doc_stage_variant_id(experience_id, doc);
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_b],
                rot_quat: face_quat,
                color_a: [accent.x, accent.y, accent.z, 1.22],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 1.0],
                phys_a: [input.time_sec * 0.3, 0.0, 0.8, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(8, vid, 0, 0, doc),
            });
        }

        // --------------------------------------------------------------------
        // EXP 11 & 18..25: EVIDENCE, CULTURE, TECH, FUTURES, ASTROBIO, CONTACT,
        //                  PERSPECTIVES, MEANING & FINAL HORIZON LOTUS
        // --------------------------------------------------------------------
        _ => {
            let max_r = (pos.length() * 0.42).clamp(0.04, 24.0);
            let r_g = (rel_scale * 0.88).clamp(0.04, max_r);
            let vid = doc_stage_variant_id(experience_id, doc);
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_g],
                rot_quat: face_quat,
                color_a: [accent.x, accent.y, accent.z, 1.2],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 0.95],
                phys_a: [input.time_sec * 0.2, 0.0, 0.8, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(9, vid, 0, 0, doc),
            });

            if experience_id == 21 && doc.active_zone_index != 12 {
                // Orbiting Habitable Exoplanet + Cracked Ice Ocean World tucked cleanly beside Goldilocks Ring
                let exo_p = pos + stage_right * (r_g * 0.56);
                heroes_surface.push(HeroVolumeInstance {
                    pos_radius: [exo_p.x, exo_p.y, exo_p.z, (r_g * 0.13).clamp(0.03, 2.0)],
                    rot_quat: face_quat,
                    color_a: [0.22, 0.78, 0.48, 1.15],
                    color_b: [0.35, 0.95, 1.35, 1.0],
                    phys_a: [input.time_sec * 0.05, 0.0, 0.0, dwell],
                    phys_b: sun,
                    kind_tags: pack_doc_tags(1, 11, 0, 0, doc),
                });
                let eur_p = pos - stage_right * (r_g * 0.56);
                heroes_surface.push(HeroVolumeInstance {
                    pos_radius: [eur_p.x, eur_p.y, eur_p.z, (r_g * 0.11).clamp(0.03, 1.6)],
                    rot_quat: face_quat,
                    color_a: [0.72, 0.84, 0.94, 1.1],
                    color_b: [0.22, 1.25, 1.65, 1.0],
                    phys_a: [input.time_sec * 0.05, 0.0, 0.0, dwell],
                    phys_b: sun,
                    kind_tags: pack_doc_tags(1, 10, 0, 0, doc),
                });
            }
        }
    }
}
