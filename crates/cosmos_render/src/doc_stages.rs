//! Documentary visual stage builders for all 25 HyperCosmosVR experiences (`01..=25`),
//! packing live `DocumentaryFocusState` telemetry cleanly into GPU hero/sprite buffers
//! with strict OLED `#000000` void preservation and zero corner photobombing.

use cosmos_spatial::{EntityKind, LodTier, VisibleEntity};
use glam::{Quat, Vec3};

use crate::types::{HeroVolumeInstance, SpriteInstance};
use crate::universe_scene::{DocumentaryFocusState, SceneBuildInput, experience_accent_linear, hsl_linear};

/// Encodes active zone (`u16`), feature (`u8`), `scene_index + 1` (`u8`), narration cue,
/// active weight, semantic cue bitmask (`bits 2..7` of `kind_tags[3]`), and domain submode
/// into `HeroVolumeInstance::kind_tags`.
pub fn pack_doc_tags(
    hero_kind: u32,
    vid: u32,
    base_flags: u32,
    submode: u32,
    doc: &DocumentaryFocusState,
) -> [u32; 4] {
    let z_u16 = doc.active_zone_index as u32 & 0xFFFF;
    let f_u8 = (doc.active_feature_index as u32) & 0xFF;
    let sc_u8 = if doc.scene_index >= 0 {
        ((doc.scene_index + 1).clamp(1, 254) as u32) & 0xFF
    } else if doc.active_zone_index != u16::MAX {
        ((doc.active_zone_index.min(253) as u32) + 1) & 0xFF
    } else {
        0
    };
    let packed_z = z_u16 | (f_u8 << 16) | (sc_u8 << 24);

    let cue_u8 = (doc.narration_cue_progress.clamp(0.0, 1.0) * 255.0).round() as u32;
    let effective_wt = if doc.narration_active_weight > 0.0 {
        (doc.narration_active_weight * (0.70 + 0.30 * doc.emphasis_boost.clamp(0.0, 1.0)))
            .clamp(0.0, 1.0)
    } else {
        0.0
    };
    let wt_u8 = (effective_wt * 255.0).round() as u32;
    let flags_byte = (base_flags & 0x03) | (((doc.cue_tags_mask as u32) & 0x3F) << 2);
    let effective_submode = if submode != 0 {
        submode
    } else {
        doc.submode_override as u32
    };
    let packed_w = (flags_byte & 0xFF)
        | ((cue_u8 & 0xFF) << 8)
        | ((wt_u8 & 0xFF) << 16)
        | ((effective_submode & 0xFF) << 24);

    [hero_kind, vid, packed_z, packed_w]
}

/// Zero-Conflict Shader ID Registry: canonical zone variant ID (`1100..=3599`) for
/// `(experience_id 1..=25, zone_index 0..=99)`: `1000 + experience_id * 100 + zone_index`.
#[inline]
pub const fn canonical_zone_vid(experience_id: u8, zone_index: u16) -> u32 {
    let clamped_zone = if zone_index > 99 { 99 } else { zone_index as u32 };
    1000 + (experience_id as u32) * 100 + clamped_zone
}

/// Zero-Conflict Shader ID Registry: canonical companion/overlay geometry variant ID (`3600..=3869`)
/// matching the Pre-Allocated Companion Block table in `docs/CONTRIBUTOR_VISUAL_ROADMAP.md`.
#[inline]
pub const fn canonical_companion_vid(experience_id: u8, slot: u8) -> u32 {
    const fn clamp_u32(v: u32, max_v: u32) -> u32 {
        if v > max_v { max_v } else { v }
    }
    let s = slot as u32;
    match experience_id {
        1 => 3600 + clamp_u32(s, 9),
        2 => 3610 + clamp_u32(s, 9),
        3 => 3620 + clamp_u32(s, 9),
        4 => 3640 + clamp_u32(s, 11),
        5 => 3660 + clamp_u32(s, 11),
        6 => 3680 + clamp_u32(s, 8),
        7 => 3700 + clamp_u32(s, 9),
        8..=10 => 3710 + ((experience_id - 8) as u32) * 3 + clamp_u32(s, 2),
        11..=25 => 3720 + ((experience_id - 11) as u32) * 10 + clamp_u32(s, 9),
        _ => 3600 + clamp_u32(s, 9),
    }
}

/// Returns `true` when `(experience_id, zone_index)` has been upgraded to its bespoke `[TIER-A]`
/// canonical shader ID (`1100..=3599`) and `doc_feat(inst)` sub-staging.
#[inline]
pub const fn is_zone_migrated_tier_a(experience_id: u8, zone_index: u16) -> bool {
    matches!(
        (experience_id, zone_index),
        (3, 0..=13)
            | (4, 0..=11)
            | (5, 0..=12)
            | (6, 0..=8)
            | (7, 0..=9)
            | (8, 0..=10)
            | (9, 0..=10)
            | (10, 0..=8)
            | (13, 0..=11)
            | (14, 0..=14)
            | (21, 0..=24)
            | (22, 0..=27)
    )
}

/// Dispatches canonical `vid` (`1100..=3599`) for `[TIER-A]` migrated zones while preserving
/// legacy `doc_stage_variant_id` compatibility for unmigrated fallback zones.
#[inline]
pub fn resolve_zone_shader_vid(experience_id: u8, doc: &DocumentaryFocusState) -> u32 {
    if is_zone_migrated_tier_a(experience_id, doc.active_zone_index) {
        canonical_zone_vid(experience_id, doc.active_zone_index)
    } else {
        doc_stage_variant_id(experience_id, doc)
    }
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
        3 => 0,
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

#[allow(clippy::too_many_arguments)]
fn push_solar_companion(
    heroes_surface: &mut Vec<HeroVolumeInstance>,
    pos: Vec3,
    radius: f32,
    rot_arr: [f32; 4],
    color_a: [f32; 4],
    color_b: [f32; 4],
    phys_a: [f32; 4],
    sun: [f32; 4],
    comp_vid: u32,
    submode: u32,
    doc: &DocumentaryFocusState,
) {
    heroes_surface.push(HeroVolumeInstance {
        pos_radius: [pos.x, pos.y, pos.z, radius],
        rot_quat: rot_arr,
        color_a,
        color_b,
        phys_a,
        phys_b: sun,
        kind_tags: pack_doc_tags(1, comp_vid, 0, submode, doc),
    });
}

/// Maps `cosmos_spatial` solar bodies (`SolarSystem` `exp 4` / `CelestialBody`) cleanly
/// onto their analytical surface/ring shader variants and `[TIER-A]` `1400..=1411` + `3640..=3651` stages.
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
    let stage_right = input.camera.cam_right.normalize_or(Vec3::X);
    let stage_up = input.camera.cam_up.normalize_or(Vec3::Y);

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
            let is_active_body = input.documentary.active_zone_index == vis.zone_index;
            let body_doc = if is_active_body {
                input.documentary
            } else {
                DocumentaryFocusState::default()
            };
            let canon_vid = canonical_zone_vid(4, vis.zone_index);
            let comp_vid = canonical_companion_vid(4, (vis.zone_index as u8).min(11));

            let max_r = (dist * 0.42).clamp(0.04, 10.0);
            let floor_r = match body_idx {
                0 => (dist * 0.014).clamp(0.04, 0.14),
                6 | 7 => (dist * 0.0072).clamp(0.032, 0.088),
                _ => (dist * 0.0048).clamp(0.022, 0.056),
            };
            let r_b = vis.pose.rel_scale.max(floor_r).clamp(0.022, max_r);
            let max_comp_r = (dist * 0.16).clamp(0.02, 0.42);
            match body_idx {
                0 => {
                    let surf_vid = if is_active_body { canon_vid } else { 1 };
                    let vol_vid = if is_active_body { canon_vid } else { 4 };
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b],
                        rot_quat: rot_arr,
                        color_a: [1.05, 0.68, 0.34, 0.92],
                        color_b: [1.45, 0.48, 0.10, 1.0],
                        phys_a: [input.time_sec * 0.04, 0.0, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(1, surf_vid, 0, 0, &body_doc),
                    });
                    heroes_volume.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b * 1.30],
                        rot_quat: rot_arr,
                        color_a: [1.25, 0.62, 0.22, 0.88],
                        color_b: [1.45, 0.42, 0.10, 0.85],
                        phys_a: [input.time_sec * 0.04, 0.0, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(3, vol_vid, 0, 0, &body_doc),
                    });
                    return;
                }
                3 => {
                    // Earth inside Solar System (`1403` + orbiting Moon & dipolar magnetosphere `3643`)
                    let surf_vid = if is_active_body { canon_vid } else { 0 };
                    let earth_submode = if is_active_body { 2 } else { 0 };
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b],
                        rot_quat: rot_arr,
                        color_a: [1.0, 1.0, 1.0, 1.15],
                        color_b: [0.16, 0.58, 1.45, 1.0],
                        phys_a: [input.time_sec * 0.05, 0.409, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(0, surf_vid, 0, earth_submode, &body_doc),
                    });
                    if is_active_body {
                        let shell_mul = 1.22_f32;
                        heroes_volume.push(HeroVolumeInstance {
                            pos_radius: [pos.x, pos.y, pos.z, r_b * shell_mul],
                            rot_quat: rot_arr,
                            color_a: [0.15, 2.2, 0.9, 1.2],
                            color_b: [1.1, 0.24, 2.0, 1.0],
                            phys_a: [input.time_sec * 0.03, 0.84 / shell_mul, 0.0, dwell],
                            phys_b: sun,
                            kind_tags: pack_doc_tags(0, canon_vid, 0, 3, &body_doc),
                        });
                        let m_ang = input.time_sec * 0.18;
                        let moon_pos = pos
                            - stage_right * (r_b * (1.85 + 0.10 * m_ang.cos()))
                            + stage_up * (r_b * (0.68 + 0.06 * m_ang.sin()));
                        push_solar_companion(
                            heroes_surface,
                            moon_pos,
                            (r_b * 0.24).clamp(0.015, max_comp_r),
                            rot_arr,
                            [0.72, 0.72, 0.74, 1.05],
                            [0.36, 0.36, 0.38, 1.0],
                            [input.time_sec * 0.02, 0.0, 0.0, dwell],
                            sun,
                            comp_vid,
                            0,
                            &body_doc,
                        );
                    }
                    return;
                }
                5 | 10 => {
                    // Asteroid Belt (5: Ceres + Vesta/Psyche `3645` + Kirkwood disk) & Kuiper Belt (10: frosted KBO + Haumea/Makemake/Comet `3650`)
                    let is_kuiper = body_idx == 10;
                    let body_r = (r_b * 0.44).clamp(0.03, 1.2);
                    let ring_tilt = Quat::from_euler(glam::EulerRot::XYZ, 0.35, 0.15, 0.08);
                    let rq = [ring_tilt.x, ring_tilt.y, ring_tilt.z, ring_tilt.w];
                    let legacy_vid = if is_kuiper { 10 } else { 3 };
                    let surf_vid = if is_active_body { canon_vid } else { legacy_vid };
                    let ring_vid = if is_active_body { canon_vid } else { 1 };
                    let (ca, cb, r_col_a, r_col_b, shard_col) = if is_kuiper {
                        (
                            [0.72, 0.84, 0.95, 1.10],
                            [0.52, 0.26, 0.18, 1.0],
                            [0.45, 0.85, 1.15, 0.62],
                            [0.62, 0.96, 1.25, 0.62],
                            [0.52, 0.88, 1.22, 0.90],
                        )
                    } else {
                        (
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
                        kind_tags: pack_doc_tags(1, surf_vid, 0, body_idx, &body_doc),
                    });
                    heroes_volume.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, body_r * 2.35],
                        rot_quat: rq,
                        color_a: r_col_a,
                        color_b: r_col_b,
                        phys_a: [0.0, 0.35, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(2, ring_vid, 0, body_idx, &body_doc),
                    });
                    if is_active_body {
                        let feat_u = body_doc.active_feature_index.min(4) as u32;
                        let (comp_mul, comp_ca, comp_cb) = if is_kuiper {
                            let c = match feat_u {
                                1 => ([0.92, 0.94, 0.98, 1.12], [0.76, 0.88, 0.98, 1.0]), // Haumea ellipsoid frost
                                2 => ([0.88, 0.44, 0.24, 1.14], [0.96, 0.72, 0.48, 1.0]), // Makemake warm methane-tholin
                                3 => ([0.36, 0.96, 1.38, 1.16], [1.18, 0.88, 0.42, 1.0]), // 2:3 orbital resonance node
                                4 => ([0.42, 0.56, 0.72, 1.10], [0.36, 1.15, 1.65, 1.0]), // active comet nucleus
                                _ => ([0.86, 0.88, 0.92, 1.10], [0.42, 0.78, 1.15, 1.0]), // Pluto/Eris bright dwarf
                            };
                            let mul = match feat_u {
                                1 => 0.42,
                                2 => 0.36,
                                _ => 0.38,
                            };
                            (mul, c.0, c.1)
                        } else {
                            (
                                0.38,
                                [0.62, 0.58, 0.54, 1.08],
                                [0.85, 0.88, 0.94, 1.0],
                            )
                        };
                        let comp_r = (body_r * comp_mul).clamp(0.014, max_comp_r);
                        let comp_pos = pos - stage_right * (body_r * 1.48) + stage_up * (body_r * 0.46);
                        push_solar_companion(
                            heroes_surface,
                            comp_pos,
                            comp_r,
                            rot_arr,
                            comp_ca,
                            comp_cb,
                            [input.time_sec * 0.08, 0.25, 0.0, dwell],
                            sun,
                            comp_vid,
                            feat_u,
                            &body_doc,
                        );
                        if is_kuiper && body_doc.active_feature_index == 4 {
                            heroes_volume.push(HeroVolumeInstance {
                                pos_radius: [comp_pos.x, comp_pos.y, comp_pos.z, comp_r * 2.6],
                                rot_quat: rot_arr,
                                color_a: [0.32, 0.92, 1.65, 1.10],
                                color_b: [1.45, 0.96, 0.42, 0.92],
                                phys_a: [input.time_sec * 0.08, 0.0, 0.0, dwell],
                                phys_b: sun,
                                kind_tags: pack_doc_tags(3, comp_vid, 0, 4, &body_doc),
                            });
                        }
                    }
                    let spread = r_b.clamp(0.08, 1.6);
                    let is_family_cluster =
                        is_active_body && !is_kuiper && body_doc.active_feature_index == 3;
                    for j in 0..7 {
                        let a = if is_family_cluster {
                            0.42 + (j as f32 - 3.0) * 0.14
                        } else {
                            (j as f32 / 7.0) * std::f32::consts::TAU
                                + (vis.id % 17) as f32 * 0.37
                        };
                        let rad = if is_family_cluster {
                            spread * (0.58 + 0.04 * ((j % 3) as f32))
                        } else {
                            spread * (0.42 + 0.14 * ((j % 3) as f32))
                        };
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
                    // Saturn + Cassini/B-ring + Titan/Enceladus companions (`1407` / `3647`)
                    let ring_tilt = Quat::from_euler(glam::EulerRot::XYZ, 0.47, 0.2, 0.15);
                    let rq = [ring_tilt.x, ring_tilt.y, ring_tilt.z, ring_tilt.w];
                    let surf_vid = if is_active_body { canon_vid } else { 2 };
                    let ring_vid = if is_active_body { canon_vid } else { 2 };
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b],
                        rot_quat: rq,
                        color_a: [0.88, 0.75, 0.50, 1.1],
                        color_b: [0.65, 0.58, 0.44, 1.0],
                        phys_a: [input.time_sec * 0.04, 0.47, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(1, surf_vid, 0, 0, &body_doc),
                    });
                    heroes_volume.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b * 2.25],
                        rot_quat: rq,
                        color_a: [0.95, 0.84, 0.62, 1.15],
                        color_b: [0.48, 0.56, 0.78, 1.0],
                        phys_a: [input.time_sec * 0.04, 0.47, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(2, ring_vid, 0, 0, &body_doc),
                    });
                    if is_active_body {
                        let titan_pos = pos - stage_right * (r_b * 2.42) + stage_up * (r_b * 0.78);
                        push_solar_companion(
                            heroes_surface,
                            titan_pos,
                            (r_b * 0.16).clamp(0.014, max_comp_r),
                            rot_arr,
                            [0.95, 0.60, 0.22, 1.10],
                            [1.15, 0.76, 0.28, 1.0],
                            [input.time_sec * 0.02, 0.0, 0.0, dwell],
                            sun,
                            comp_vid,
                            0,
                            &body_doc,
                        );
                        let enc_r = (r_b * 0.11).clamp(0.011, max_comp_r);
                        let enc_pos = pos + stage_right * (r_b * 2.36) - stage_up * (r_b * 0.52);
                        push_solar_companion(
                            heroes_surface,
                            enc_pos,
                            enc_r,
                            rot_arr,
                            [0.86, 0.94, 1.02, 1.12],
                            [0.32, 0.92, 1.35, 1.0],
                            [input.time_sec * 0.03, 0.0, 0.0, dwell],
                            sun,
                            comp_vid,
                            1,
                            &body_doc,
                        );
                        if body_doc.active_feature_index == 4 {
                            heroes_volume.push(HeroVolumeInstance {
                                pos_radius: [enc_pos.x, enc_pos.y, enc_pos.z, (enc_r * 2.15).clamp(0.02, max_comp_r * 1.4)],
                                rot_quat: rot_arr,
                                color_a: [0.38, 1.15, 1.65, 1.10],
                                color_b: [0.78, 0.95, 1.15, 0.95],
                                phys_a: [input.time_sec * 0.25, 0.0, 0.0, dwell],
                                phys_b: sun,
                                kind_tags: pack_doc_tags(3, comp_vid, 0, 4, &body_doc),
                            });
                        }
                    }
                    return;
                }
                8 => {
                    // Uranus + dark charcoal epsilon rings + Miranda companion (`1408` / `3648`)
                    let tilt_u = Quat::from_euler(glam::EulerRot::XYZ, 1.71, 0.1, 0.0);
                    let uq = [tilt_u.x, tilt_u.y, tilt_u.z, tilt_u.w];
                    let surf_vid = if is_active_body { canon_vid } else { 7 };
                    let ring_vid = if is_active_body { canon_vid } else { 1 };
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b],
                        rot_quat: uq,
                        color_a: [0.42, 0.85, 0.94, 1.1],
                        color_b: [0.58, 0.96, 1.05, 1.0],
                        phys_a: [input.time_sec * 0.03, 1.71, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(1, surf_vid, 0, body_idx, &body_doc),
                    });
                    heroes_volume.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b * 1.85],
                        rot_quat: uq,
                        color_a: [0.22, 0.25, 0.28, 0.60],
                        color_b: [0.34, 0.38, 0.42, 0.60],
                        phys_a: [input.time_sec * 0.03, 1.71, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(2, ring_vid, 0, body_idx, &body_doc),
                    });
                    if is_active_body {
                        let miranda_pos = pos + stage_right * (r_b * 1.56) + stage_up * (r_b * 0.56);
                        push_solar_companion(
                            heroes_surface,
                            miranda_pos,
                            (r_b * 0.18).clamp(0.012, max_comp_r),
                            rot_arr,
                            [0.64, 0.68, 0.72, 1.08],
                            [0.36, 0.42, 0.48, 1.0],
                            [input.time_sec * 0.04, 0.0, 0.0, dwell],
                            sun,
                            comp_vid,
                            0,
                            &body_doc,
                        );
                    }
                    return;
                }
                _ => {
                    let legacy_vid = match body_idx {
                        1 => 3,  // Mercury
                        2 => 4,  // Venus
                        4 => 5,  // Mars
                        6 => 6,  // Jupiter
                        9 => 8,  // Neptune
                        11 => 9, // Pluto
                        _ => 10, // Europa / icy moons
                    };
                    let surf_vid = if is_active_body { canon_vid } else { legacy_vid };
                    let (ca, cb) = match legacy_vid {
                        3 => ([0.52, 0.48, 0.44, 1.05], [0.28, 0.26, 0.24, 1.0]),
                        4 => ([0.95, 0.78, 0.45, 1.08], [0.62, 0.42, 0.20, 1.0]),
                        5 => ([0.88, 0.36, 0.16, 1.10], [0.48, 0.18, 0.08, 1.0]),
                        6 => ([0.88, 0.72, 0.54, 1.12], [0.78, 0.44, 0.24, 1.0]),
                        8 => ([0.16, 0.42, 1.05, 1.15], [0.35, 0.72, 1.35, 1.0]),
                        9 => ([0.78, 0.62, 0.50, 1.05], [0.94, 0.88, 0.78, 1.0]),
                        _ => ([0.72, 0.84, 0.95, 1.10], [0.22, 1.25, 1.65, 1.0]),
                    };
                    let spin_angle = if body_idx == 2 {
                        // Venus rotates retrograde (~243 Earth days backwards)
                        -input.time_sec * 0.03
                    } else {
                        input.time_sec * 0.03
                    };
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [pos.x, pos.y, pos.z, r_b],
                        rot_quat: rot_arr,
                        color_a: ca,
                        color_b: cb,
                        phys_a: [spin_angle, 0.1, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(1, surf_vid, 0, body_idx, &body_doc),
                    });
                    if is_active_body {
                        match body_idx {
                            4 => {
                                // Phobos & Deimos around Mars (`3644`)
                                push_solar_companion(
                                    heroes_surface,
                                    pos - stage_right * (r_b * 1.48) + stage_up * (r_b * 0.42),
                                    (r_b * 0.16).clamp(0.012, max_comp_r),
                                    rot_arr,
                                    [0.52, 0.38, 0.30, 1.06],
                                    [0.32, 0.22, 0.16, 1.0],
                                    [input.time_sec * 0.06, 0.0, 0.0, dwell],
                                    sun,
                                    comp_vid,
                                    0,
                                    &body_doc,
                                );
                                push_solar_companion(
                                    heroes_surface,
                                    pos + stage_right * (r_b * 1.82) - stage_up * (r_b * 0.44),
                                    (r_b * 0.12).clamp(0.010, max_comp_r),
                                    rot_arr,
                                    [0.48, 0.36, 0.28, 1.04],
                                    [0.28, 0.20, 0.15, 1.0],
                                    [input.time_sec * 0.04, 0.0, 0.0, dwell],
                                    sun,
                                    comp_vid,
                                    1,
                                    &body_doc,
                                );
                            }
                            6 => {
                                // Galilean quartet (`3646`): Europa (submode=1 -> cracked-ice cutaway) + Io (submode=0) + Ganymede/Callisto sprites
                                push_solar_companion(
                                    heroes_surface,
                                    pos - stage_right * (r_b * 1.62) + stage_up * (r_b * 0.28),
                                    (r_b * 0.22).clamp(0.015, max_comp_r),
                                    rot_arr,
                                    [0.72, 0.84, 0.95, 1.12],
                                    [0.22, 1.25, 1.65, 1.0],
                                    [input.time_sec * 0.03, 0.0, 0.0, dwell],
                                    sun,
                                    comp_vid,
                                    1,
                                    &body_doc,
                                );
                                push_solar_companion(
                                    heroes_surface,
                                    pos + stage_right * (r_b * 1.44) - stage_up * (r_b * 0.24),
                                    (r_b * 0.20).clamp(0.014, max_comp_r),
                                    rot_arr,
                                    [0.98, 0.82, 0.28, 1.12],
                                    [0.95, 0.42, 0.12, 1.0],
                                    [input.time_sec * 0.05, 0.0, 0.0, dwell],
                                    sun,
                                    comp_vid,
                                    0,
                                    &body_doc,
                                );
                                for (idx_m, (rx, ry, col_m)) in [
                                    (-2.05_f32, -0.34_f32, [0.76, 0.72, 0.66, 0.95]),
                                    (2.12_f32, 0.36_f32, [0.56, 0.54, 0.52, 0.92]),
                                ]
                                .into_iter()
                                .enumerate()
                                {
                                    let mp = pos + stage_right * (r_b * rx) + stage_up * (r_b * ry);
                                    sprites.push(SpriteInstance {
                                        pos_size: [mp.x, mp.y, mp.z, (r_b * 0.14).clamp(0.010, 0.065)],
                                        axis_aspect: [0.0, 1.0, 0.0, 1.0],
                                        color_alpha: col_m,
                                        tags_f: [5.0, (idx_m as f32) * 0.25, 0.48, 0.0],
                                    });
                                }
                            }
                            9 => {
                                // Retrograde cantaloupe Triton (`3649`) orbiting Neptune
                                push_solar_companion(
                                    heroes_surface,
                                    pos - stage_right * (r_b * 1.56) + stage_up * (r_b * 0.48),
                                    (r_b * 0.22).clamp(0.014, max_comp_r),
                                    rot_arr,
                                    [0.82, 0.76, 0.74, 1.08],
                                    [0.45, 0.65, 0.88, 1.0],
                                    [-input.time_sec * 0.05, 0.0, 0.0, dwell],
                                    sun,
                                    comp_vid,
                                    0,
                                    &body_doc,
                                );
                            }
                            11 => {
                                // Tidally locked Charon (`3651`) + 4 outer tiny moonlets (Nix, Hydra, Kerberos, Styx)
                                push_solar_companion(
                                    heroes_surface,
                                    pos + stage_right * (r_b * 1.62) + stage_up * (r_b * 0.28),
                                    (r_b * 0.51).clamp(0.016, max_comp_r),
                                    rot_arr,
                                    [0.58, 0.58, 0.62, 1.06],
                                    [0.54, 0.22, 0.14, 1.0],
                                    [input.time_sec * 0.03, 0.0, 0.0, dwell],
                                    sun,
                                    comp_vid,
                                    0,
                                    &body_doc,
                                );
                                for m_i in 0..4u32 {
                                    let ang = (m_i as f32) * 1.45 + 0.55;
                                    let rad = r_b * (2.15 + 0.24 * (m_i as f32));
                                    let mp = pos
                                        + stage_right * (ang.cos() * rad)
                                        + stage_up * (ang.sin() * rad * 0.48);
                                    sprites.push(SpriteInstance {
                                        pos_size: [mp.x, mp.y, mp.z, (r_b * 0.09).clamp(0.008, 0.048)],
                                        axis_aspect: [stage_right.x, stage_right.y, stage_right.z, 1.35],
                                        color_alpha: [0.84, 0.88, 0.94, 0.95],
                                        tags_f: [5.0, (m_i as f32) * 0.18, 0.45, 0.0],
                                    });
                                }
                            }
                            _ => {}
                        }
                    }
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
        let is_active_landmark = matches!(vis.experience_id, 5..=7)
            && vis.experience_id == input.active_experience_id
            && input.documentary.active_zone_index == vis.zone_index;
        if (vis.lod == LodTier::Hero && (vis.experience_id == input.active_experience_id || matches!(input.active_experience_id, 5..=9)))
            || (is_active_landmark && matches!(vis.lod, LodTier::Hero | LodTier::Impostor | LodTier::MacroCloud))
        {
            let max_r = (dist * 0.40).clamp(0.05, 16.0);
            let floor_r = if is_active_landmark {
                let min_floor = 0.08_f32.min(max_r);
                (dist * 0.24).clamp(min_floor, max_r)
            } else {
                0.05_f32.min(max_r)
            };
            let r_l = vis.pose.rel_scale.max(floor_r).clamp(0.05_f32.min(max_r), max_r);
            if matches!(vis.experience_id, 5..=7) {
                let max_z = match vis.experience_id {
                    5 => 12,
                    6 => 8,
                    _ => 9,
                };
                let max_comp_z = match vis.experience_id {
                    5 => 11,
                    6 => 8,
                    _ => 9,
                };
                let z_idx = vis.zone_index.min(max_z);
                let lm_doc = if is_active_landmark {
                    input.documentary
                } else {
                    DocumentaryFocusState::default()
                };
                let canon_vid = canonical_zone_vid(vis.experience_id, z_idx);
                let comp_vid = canonical_companion_vid(vis.experience_id, (z_idx as u8).min(max_comp_z as u8));
                let base_sub = if vis.experience_id == 5 && z_idx == 12 {
                    32
                } else {
                    z_idx as u32
                };
                let primary_vid = if is_active_landmark || (vis.experience_id == 5 && z_idx == 12) {
                    canon_vid
                } else {
                    comp_vid
                };
                heroes_volume.push(HeroVolumeInstance {
                    pos_radius: [pos.x, pos.y, pos.z, r_l],
                    rot_quat: rot_arr,
                    color_a: [accent.x, accent.y, accent.z, 1.08],
                    color_b: [0.35, 0.95, 1.55, 0.92],
                    phys_a: [input.time_sec * 0.15, 0.0, 0.8, dwell],
                    phys_b: sun,
                    kind_tags: pack_doc_tags(3, primary_vid, 0, base_sub, &lm_doc),
                });
                if is_active_landmark && z_idx <= max_comp_z {
                    let comp_max_r = (dist * 0.16).clamp(0.04, 4.0);
                    let comp_r = (r_l * 0.42).clamp(0.04_f32.min(comp_max_r), comp_max_r);
                    let comp_pos = pos + stage_right * (r_l * 1.15) + stage_up * (r_l * 0.22);
                    let comp_sub = lm_doc.active_feature_index.min(4) as u32;
                    heroes_volume.push(HeroVolumeInstance {
                        pos_radius: [comp_pos.x, comp_pos.y, comp_pos.z, comp_r],
                        rot_quat: rot_arr,
                        color_a: [accent.x * 1.06, accent.y * 1.06, accent.z * 1.06, 1.12],
                        color_b: [0.42, 1.05, 1.62, 0.95],
                        phys_a: [input.time_sec * 0.15, 0.0, 0.8, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(3, comp_vid, 0, comp_sub, &lm_doc),
                    });
                }
                return;
            }

            let vid = vis.experience_id as u32;
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
    let r_spr = vis.pose.rel_scale.clamp(0.004, max_ang.max(0.004));
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
    let default_doc = DocumentaryFocusState::default();
    let doc = if input.active_experience_id == 0 || experience_id == input.active_experience_id {
        &input.documentary
    } else {
        &default_doc
    };
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
            // Atmospheric & surface zones (`Z00..=Z06`, including `Z06 Surface` at `u = 6.05`) never open the cutaway prematurely.
            let is_surface_or_atmo_zone = doc.active_zone_index <= 6;
            let cutaway = if !is_surface_or_atmo_zone
                && (input.earth_cutaway_blend > 0.25
                    || (input.active_experience_id == 3 && (5.2..=6.45).contains(&u_cam))
                    || (7..=13).contains(&doc.active_zone_index))
            {
                1u32
            } else {
                0u32
            };
            let earth_vid = resolve_zone_shader_vid(3, doc);
            let shell_submode = if earth_vid == canonical_zone_vid(3, 5) {
                2u32
            } else if cutaway != 0 {
                1u32
            } else {
                3u32
            };
            let shell_mul = match doc.active_zone_index {
                0 => 1.22_f32,
                1 => 1.18_f32,
                _ => 1.14_f32,
            };
            let earth_limb_in_shell = 0.84 / shell_mul;
            let surface_submode = if cutaway != 0 { 1u32 } else { 2u32 };
            heroes_surface.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, earth_solid_r],
                rot_quat,
                color_a: [1.0, 1.0, 1.0, 1.15],
                color_b: [0.16, 0.58, 1.45, 1.0],
                phys_a: [input.time_sec * 0.03, 0.409, 0.0, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(0, earth_vid, cutaway, surface_submode, doc),
            });
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, earth_solid_r * shell_mul],
                rot_quat,
                color_a: [0.15, 2.2, 0.9, 1.2],
                color_b: [1.1, 0.24, 2.0, 1.0],
                phys_a: [input.time_sec * 0.03, earth_limb_in_shell, 0.0, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(0, earth_vid, 0, shell_submode, doc),
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
        // EXP 05: MILKY WAY / SGR A* KERR BLACK HOLE (`1500..=1512`)
        // --------------------------------------------------------------------
        5 => {
            let max_r = (pos.length() * 0.46).clamp(0.04, 24.0);
            let r_bh = rel_scale.clamp(0.04, max_r);
            let stage_vid = if input.active_experience_id == 5 {
                resolve_zone_shader_vid(5, doc)
            } else {
                5
            };
            let stage_sub = if doc.active_zone_index == 12 {
                32u32
            } else if doc.active_zone_index <= 12 {
                doc.active_zone_index as u32
            } else {
                0u32
            };
            heroes_surface.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_bh],
                rot_quat,
                color_a: [0.0, 0.0, 0.0, 1.0],
                color_b: [0.0, 0.0, 0.0, 1.0],
                phys_a: [0.95, 0.0, 0.95, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(4, stage_vid, 2, stage_sub, doc),
            });
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_bh],
                rot_quat,
                color_a: [1.25, 1.0, 0.88, 1.25],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 1.0],
                phys_a: [0.95, 0.25, 0.95, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(4, stage_vid, 2, stage_sub, doc),
            });
        }

        // --------------------------------------------------------------------
        // EXP 12: FUTURE OF UNIVERSE — Primary Documentary Stage (120..=124)
        //         + Flanking Hawking BH Companion on Black-Hole Era Zones
        // --------------------------------------------------------------------
        12 => {
            let max_r = (pos.length() * 0.42).clamp(0.04, 24.0);
            let r_g = (rel_scale * 0.88).clamp(0.04, max_r);
            let vid = resolve_zone_shader_vid(12, doc);
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_g],
                rot_quat: face_quat,
                color_a: [accent.x, accent.y, accent.z, 1.2],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 0.95],
                phys_a: [input.time_sec * 0.2, 0.0, 0.8, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(9, vid, 0, 0, doc),
            });
            if matches!(doc.active_zone_index, 5..=7) {
                let bh_p = pos + stage_right * (r_g * 0.84);
                let r_bh = (r_g * 0.24).clamp(0.04, 4.0);
                let bh_vid = canonical_companion_vid(12, 0);
                heroes_surface.push(HeroVolumeInstance {
                    pos_radius: [bh_p.x, bh_p.y, bh_p.z, r_bh],
                    rot_quat,
                    color_a: [0.0, 0.0, 0.0, 1.0],
                    color_b: [0.0, 0.0, 0.0, 1.0],
                    phys_a: [0.95, 0.0, 0.95, dwell],
                    phys_b: sun,
                    kind_tags: pack_doc_tags(4, bh_vid, 2, doc.active_zone_index as u32, doc),
                });
                heroes_volume.push(HeroVolumeInstance {
                    pos_radius: [bh_p.x, bh_p.y, bh_p.z, r_bh],
                    rot_quat,
                    color_a: [1.15, 0.95, 0.88, 1.15],
                    color_b: [zone_c.x, zone_c.y, zone_c.z, 0.92],
                    phys_a: [0.95, 0.25, 0.95, dwell],
                    phys_b: sun,
                    kind_tags: pack_doc_tags(4, bh_vid, 2, doc.active_zone_index as u32, doc),
                });
            }
        }

        // --------------------------------------------------------------------
        // EXP 06..10: COSMOLOGICAL MACRO (LOCAL GROUP, LANIAKEA, WEB, OBSERVABLE, EARLY)
        // --------------------------------------------------------------------
        6..=10 => {
            let max_r = (pos.length() * 0.42).clamp(0.04, 24.0);
            let r_g = rel_scale.clamp(0.04, max_r);
            let stage_vid = resolve_zone_shader_vid(experience_id, doc);
            let submode = if doc.active_zone_index != u16::MAX {
                (doc.active_zone_index as u32).min(31)
            } else {
                0
            };
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_g],
                rot_quat,
                color_a: [accent.x, accent.y, accent.z, 1.12],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 0.92],
                phys_a: [input.time_sec * 0.2, 0.0, 0.8, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(3, stage_vid, 0, submode, doc),
            });
        }

        // --------------------------------------------------------------------
        // EXP 13 & 14: MATTER/ENERGY & SPACETIME (TIER-A Act III)
        // --------------------------------------------------------------------
        13 | 14 => {
            let max_r = (pos.length() * 0.42).clamp(0.04, 6.0);
            let r_q = (rel_scale * 0.86).clamp(0.04, max_r);
            let vid = resolve_zone_shader_vid(experience_id, doc);
            let base_flags = match (experience_id, doc.active_zone_index) {
                (13, 5) | (14, 2 | 5 | 7 | 8 | 10 | 13) => 0x02,
                _ => 0,
            };
            let submode = match (experience_id, doc.active_zone_index) {
                (13, 6) | (14, 11) => 8,
                _ => (doc.active_feature_index.min(7) as u32) + 1,
            };
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_q],
                rot_quat: face_quat,
                color_a: [accent.x, accent.y, accent.z, 1.2],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 1.0],
                phys_a: [input.time_sec * 0.4, 0.0, 1.0, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(7, vid, base_flags, submode, doc),
            });

            // Flanking companion surface sphere on non-showcase zones so canonical
            // showcase screenshots remain unoccluded while exercising 3740..=3759.
            if doc.active_zone_index != u16::MAX {
                let comp_slot = if experience_id == 13 {
                    match doc.active_zone_index {
                        0 => Some(0u8),
                        1 => Some(1),
                        4 => Some(4),
                        5 => Some(5),
                        7 => Some(7),
                        8 | 9 => Some(8),
                        10 | 11 => Some(9),
                        _ => None, // showcase zones 2, 3, 6
                    }
                } else {
                    match doc.active_zone_index {
                        0 => Some(0u8),
                        1 => Some(1),
                        2 | 3 => Some(2),
                        4 => Some(3),
                        5 => Some(4),
                        6 => Some(5),
                        7 => Some(6),
                        9 => Some(8),
                        10 | 12..=14 => Some(9),
                        _ => None, // showcase zones 8, 11
                    }
                };
                if let Some(slot) = comp_slot {
                    let comp_p = pos + stage_right * (r_q * 0.62) + stage_up * (r_q * 0.16);
                    let r_comp = (r_q * 0.13).clamp(0.025, 0.72);
                    let comp_vid = canonical_companion_vid(experience_id, slot);
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [comp_p.x, comp_p.y, comp_p.z, r_comp],
                        rot_quat,
                        color_a: [accent.x, accent.y, accent.z, 1.0],
                        color_b: [zone_c.x, zone_c.y, zone_c.z, 1.0],
                        phys_a: [0.65, 0.0, 0.8, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(1, comp_vid, base_flags, submode, doc),
                    });
                }
            }
        }

        // --------------------------------------------------------------------
        // EXP 15, 16, 17: COMPLEXITY, LIFE & SCULPTED 3D ANATOMICAL MIND
        // --------------------------------------------------------------------
        15..=17 => {
            let max_r = (pos.length() * 0.42).clamp(0.04, 6.0);
            let r_b = (rel_scale * 0.86).clamp(0.04, max_r);
            let vid = resolve_zone_shader_vid(experience_id, doc);
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
            let vid = resolve_zone_shader_vid(experience_id, doc);
            let submode = if doc.active_zone_index != u16::MAX {
                (doc.active_zone_index as u32).min(31)
            } else {
                0
            };
            heroes_volume.push(HeroVolumeInstance {
                pos_radius: [pos.x, pos.y, pos.z, r_g],
                rot_quat: face_quat,
                color_a: [accent.x, accent.y, accent.z, 1.2],
                color_b: [zone_c.x, zone_c.y, zone_c.z, 0.95],
                phys_a: [input.time_sec * 0.2, 0.0, 0.8, dwell],
                phys_b: sun,
                kind_tags: pack_doc_tags(9, vid, 0, submode, doc),
            });

            if experience_id == 21 {
                if doc.active_zone_index == u16::MAX {
                    // Legacy overview fallback: flanking Habitable Exoplanet + Cracked Ice Moon
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
                } else if matches!(doc.active_zone_index, 4 | 13 | 14 | 15) {
                    // Canonical Tier-A Exp 21 companion planet sphere (`vid=3820`) for planetary/biosignature zones only
                    let comp_sub = match doc.active_zone_index {
                        4 => 0,
                        13 => 1,
                        14 => 2,
                        _ => 3,
                    };
                    let comp_p = pos + stage_right * (r_g * 0.62);
                    let (col_a, col_b) = match comp_sub {
                        3 => ([0.62, 0.24, 0.14, 1.15], [1.25, 0.52, 0.22, 1.0]),
                        2 => ([0.18, 0.78, 0.56, 1.15], [0.28, 1.18, 1.48, 1.0]),
                        1 => ([0.34, 0.56, 0.88, 1.15], [0.55, 0.96, 1.38, 1.0]),
                        _ => ([0.22, 0.78, 0.48, 1.15], [0.35, 0.95, 1.35, 1.0]),
                    };
                    heroes_surface.push(HeroVolumeInstance {
                        pos_radius: [comp_p.x, comp_p.y, comp_p.z, (r_g * 0.13).clamp(0.03, 1.8)],
                        rot_quat: face_quat,
                        color_a: col_a,
                        color_b: col_b,
                        phys_a: [input.time_sec * 0.05, 0.0, 0.0, dwell],
                        phys_b: sun,
                        kind_tags: pack_doc_tags(1, canonical_companion_vid(21, 0), 0, comp_sub, doc),
                    });
                }
            } else if experience_id == 22
                && matches!(doc.active_zone_index, 4 | 5 | 19 | 21 | 22 | 25 | 26 | 27)
            {
                let comp_slot: u8 = match doc.active_zone_index {
                    4 => 0,
                    5 => 1,
                    19 => 2,
                    21 | 22 => 3,
                    25 | 26 => 4,
                    _ => 5,
                };
                let comp_vid = canonical_companion_vid(22, comp_slot);
                let comp_p = pos + stage_right * (r_g * 0.62);
                let (col_a, col_b, temp) = match comp_slot {
                    0 => ([0.18, 0.72, 0.92, 1.12], [0.28, 1.18, 1.48, 1.0], 5600.0),
                    1 => ([0.96, 0.76, 0.28, 1.15], [0.26, 1.12, 1.44, 1.0], 5800.0),
                    2 => ([0.62, 0.84, 1.12, 1.12], [1.22, 0.88, 0.34, 1.0], 6200.0),
                    3 => ([0.22, 0.82, 0.98, 1.15], [0.46, 0.92, 1.46, 1.0], 5400.0),
                    4 => ([0.88, 0.74, 0.24, 1.15], [0.22, 1.18, 0.86, 1.0], 5500.0),
                    _ => ([0.92, 0.72, 0.32, 1.15], [0.24, 1.14, 1.46, 1.0], 5700.0),
                };
                heroes_surface.push(HeroVolumeInstance {
                    pos_radius: [comp_p.x, comp_p.y, comp_p.z, (r_g * 0.12).clamp(0.03, 1.7)],
                    rot_quat: face_quat,
                    color_a: col_a,
                    color_b: col_b,
                    phys_a: [input.time_sec * 0.05, 0.0, 0.0, dwell],
                    phys_b: [sun[0], sun[1], sun[2], temp],
                    kind_tags: pack_doc_tags(1, comp_vid, 0, comp_slot as u32, doc),
                });
            }
        }
    }
}
