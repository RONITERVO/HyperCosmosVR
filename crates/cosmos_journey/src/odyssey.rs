use std::collections::HashMap;

use cosmos_manifest::{Locale, NONE_ASSET, UniverseCatalog};
use cosmos_math::{ScalePose64, SectorId};
use cosmos_narration::TargetResolver;
use cosmos_nav::COLD_START_SCALE_EXP;
use cosmos_spatial::{CosmosSpatialIndex, all_experience_placements, hsl_to_prism_local, music_filament_local};
use glam::{DQuat, DVec3};

use crate::director::{localized_experience_title, orient_toward};
use crate::plugin::StoryJourneyPlugin;
use crate::schema::{
    BeatMotionProfile, BeatTargetSelector, BeatTransition, CURRENT_SCHEMA_VERSION, JourneyBeat,
    JourneyChapter, JourneyError, LocalizedTitle, MotionProfileKind, SerializedPose64,
    StoryJourneyManifest, TransitionKind,
};

struct OdysseyActSpec {
    act_index: u8,
    experiences: &'static [u8],
}

/// Canonical 5-act topological ordering across all 25 experiences respecting `continuity_edges()`.
const ODYSSEY_ACTS: &[OdysseyActSpec] = &[
    OdysseyActSpec {
        act_index: 1,
        experiences: &[3, 4],
    },
    OdysseyActSpec {
        act_index: 2,
        experiences: &[5, 21, 22, 6, 7, 8, 9, 10],
    },
    OdysseyActSpec {
        act_index: 3,
        experiences: &[13, 14],
    },
    OdysseyActSpec {
        act_index: 4,
        experiences: &[15, 16, 17, 1, 2],
    },
    OdysseyActSpec {
        act_index: 5,
        experiences: &[18, 19, 20, 11, 12, 23, 24, 25],
    },
];

/// Full-catalog built-in Story-Journey covering every zone (`376`), feature (`2,456`),
/// App `01` prism color (`130` keys), App `02` music filament (`48` MIDI × `4` timbres),
/// and 100% of baked catalog audio assets (`18,898`) across one seamless 5-act loop.
pub struct GrandCosmicOdysseyPlugin;

impl StoryJourneyPlugin for GrandCosmicOdysseyPlugin {
    fn id(&self) -> &str {
        "grand_cosmic_odyssey"
    }

    fn title(&self, locale: Locale) -> &str {
        match locale {
            Locale::EsEs => "Gran Odisea Cósmica Completa",
            Locale::EnUs => "Grand Cosmic Odyssey (Full Catalog)",
            Locale::FiFi => "Suuri Kosminen Odysseia (Koko Luettelo)",
        }
    }

    fn description(&self, locale: Locale) -> &str {
        match locale {
            Locale::EsEs => {
                "Viaje continuo por las 25 experiencias, 376 zonas, 2.456 rasgos, espectro cromático y arpa cimática."
            }
            Locale::EnUs => {
                "Continuous 5-act voyage across all 25 experiences, 376 zones, 2,456 features, Colors prism, and Music harp."
            }
            Locale::FiFi => {
                "Jatkuva 5-näytöksinen matka kaikkiin 25 kokemukseen, 376 vyöhykkeeseen, 2 456 kohteeseen, väreihin ja musiikkiin."
            }
        }
    }

    fn build_manifest(
        &self,
        catalog: &UniverseCatalog,
        spatial: &CosmosSpatialIndex,
    ) -> Result<StoryJourneyManifest, JourneyError> {
        let mut chapters = Vec::new();
        let color_rgb_table = build_color_key_rgb_table(catalog);

        for act in ODYSSEY_ACTS {
            let act_idx = act.act_index;
            for &exp_id in act.experiences {
                let beats = match exp_id {
                    1 => build_colors_chapter_beats(catalog, &color_rgb_table),
                    2 => build_music_chapter_beats(),
                    _ => build_catalog_experience_beats(catalog, spatial, exp_id),
                };
                if !beats.is_empty() {
                    chapters.push(JourneyChapter {
                        id: format!("act{act_idx}_exp{exp_id:02}"),
                        act_index: act_idx,
                        title: localized_experience_title(catalog, exp_id),
                        beats,
                    });
                }
            }
        }

        // Guarantee 100% audio asset coverage across all 1..=25 catalog key_entries.
        ensure_full_audio_coverage(catalog, spatial, &mut chapters);

        // Append graceful Ouroboros return beat back to Cold-Start Earth Low Orbit.
        let earth_home_pose = cold_start_earth_pose();
        chapters.push(JourneyChapter {
            id: "act5_ouroboros_earth_return".to_owned(),
            act_index: 5,
            title: LocalizedTitle::new(
                "Regreso Ouroboros a la Órbita Terrestre",
                "Ouroboros Return to Earth Orbit",
                "Ouroboros-paluu Maan kiertoradalle",
            ),
            beats: vec![JourneyBeat {
                beat_id: "ouroboros_earth_home".to_owned(),
                pose: SerializedPose64::from_pose64(&earth_home_pose),
                look_at_pose: Some(SerializedPose64::from_pose64(&ScalePose64::new(
                    SectorId::EARTH_ORBIT,
                    DVec3::ZERO,
                    COLD_START_SCALE_EXP,
                    DQuat::IDENTITY,
                ))),
                target: BeatTargetSelector::CatalogZone {
                    experience_id: 3,
                    zone_index: 0,
                    feature_index: None,
                    is_macro_zoomed_out: true,
                },
                motion: BeatMotionProfile {
                    kind: MotionProfileKind::OrbitalParallax,
                    yaw_deg_per_sec: 4.5,
                    max_sentences_per_vantage: 2,
                    min_dwell_sec: 2.8,
                    max_dwell_sec: 12.0,
                    ..Default::default()
                },
                transition_in: BeatTransition {
                    kind: TransitionKind::LogScaleHyperDive,
                    duration_sec: 3.4,
                    narration_lead_sec: 0.45,
                },
                caption_hint: Some("Ouroboros · Tierra".to_owned()),
            }],
        });

        let manifest = StoryJourneyManifest {
            schema_version: CURRENT_SCHEMA_VERSION,
            id: self.id().to_owned(),
            title: LocalizedTitle::new(
                self.title(Locale::EsEs),
                self.title(Locale::EnUs),
                self.title(Locale::FiFi),
            ),
            subtitle: LocalizedTitle::new(
                self.description(Locale::EsEs),
                self.description(Locale::EnUs),
                self.description(Locale::FiFi),
            ),
            author: "HyperCosmosVR".to_owned(),
            loop_journey: true,
            chapters,
        };
        manifest.validate(Some(catalog))?;
        Ok(manifest)
    }
}

/// Curated compact 5-act highlight Story-Journey (~22 beats across key scales) for quick showcases.
pub struct CosmicHighlightsPlugin;

impl StoryJourneyPlugin for CosmicHighlightsPlugin {
    fn id(&self) -> &str {
        "cosmic_highlights_tour"
    }

    fn title(&self, locale: Locale) -> &str {
        match locale {
            Locale::EsEs => "Lo Mejor del Hipercosmos",
            Locale::EnUs => "HyperCosmos Highlights Tour",
            Locale::FiFi => "HyperCosmos-kohokohdat",
        }
    }

    fn description(&self, locale: Locale) -> &str {
        match locale {
            Locale::EsEs => "Recorrido ágil de 5 actos desde la Aurora terrestre hasta la escala de Planck y el Horizonte Final.",
            Locale::EnUs => "Crisp 5-act showcase gliding from Earth Aurora through Galaxies, Planck foam, Mind, Colors, Music, and the Final Lotus.",
            Locale::FiFi => "Tiivis 5-näytöksinen esittely Maan revontulista Planckin vaahtoon ja lopulliseen horisonttiin.",
        }
    }

    fn build_manifest(
        &self,
        catalog: &UniverseCatalog,
        spatial: &CosmosSpatialIndex,
    ) -> Result<StoryJourneyManifest, JourneyError> {
        let selections: &[(u8, &[usize])] = &[
            (3, &[0, 6, 13]),
            (4, &[0, 4, 7]),
            (5, &[0, 1]),
            (21, &[0]),
            (22, &[0]),
            (7, &[0]),
            (9, &[0]),
            (13, &[0, 3]),
            (14, &[0, 4]),
            (16, &[0]),
            (17, &[0]),
            (18, &[0]),
            (20, &[0]),
            (25, &[0]),
        ];

        let mut chapters = Vec::new();
        for &(exp_id, zone_picks) in selections {
            let zones = catalog.zones(exp_id);
            let mut beats = Vec::new();
            let mut prev_u = COLD_START_SCALE_EXP;
            for &z_idx in zone_picks {
                if let Some(zone) = zones.get(z_idx) {
                    let beat = make_zone_beat(catalog, spatial, zone, prev_u);
                    prev_u = beat.pose.scale_exp;
                    beats.push(beat);
                    if !catalog.features(zone).is_empty() {
                        beats.push(make_feature_beat(spatial, zone, 0, prev_u));
                    }
                }
            }
            if !beats.is_empty() {
                chapters.push(JourneyChapter {
                    id: format!("highlights_exp_{exp_id:02}"),
                    act_index: 1,
                    title: localized_experience_title(catalog, exp_id),
                    beats,
                });
            }
        }

        // Include signature Colors & Music stops.
        chapters.push(JourneyChapter {
            id: "highlights_color_music".to_owned(),
            act_index: 4,
            title: LocalizedTitle::new(
                "01–02 · Prisma Cromático y Arpa Armónica",
                "01–02 · Chromatic Prism & Harmonic Harp",
                "01–02 · Väriprisma ja Harmoninen Harppu",
            ),
            beats: vec![
                make_color_beat(0, [255, 72, 48], 24.0, 0.92, 0.56),
                make_music_beat(69, 2),
            ],
        });

        let manifest = StoryJourneyManifest {
            schema_version: CURRENT_SCHEMA_VERSION,
            id: self.id().to_owned(),
            title: LocalizedTitle::new(
                self.title(Locale::EsEs),
                self.title(Locale::EnUs),
                self.title(Locale::FiFi),
            ),
            subtitle: LocalizedTitle::new(
                self.description(Locale::EsEs),
                self.description(Locale::EnUs),
                self.description(Locale::FiFi),
            ),
            author: "HyperCosmosVR".to_owned(),
            loop_journey: true,
            chapters,
        };
        manifest.validate(Some(catalog))?;
        Ok(manifest)
    }
}

fn cold_start_earth_pose() -> ScalePose64 {
    let cam_pos = DVec3::new(0.22, 0.73, 1.01);
    ScalePose64::new(
        SectorId::EARTH_ORBIT,
        cam_pos,
        COLD_START_SCALE_EXP,
        orient_toward(cam_pos, DVec3::ZERO),
    )
}

fn entity_to_pose(e: &cosmos_spatial::SpatialEntity) -> ScalePose64 {
    ScalePose64::new(e.sector_id, e.local_pos, e.scale_exp, DQuat::IDENTITY)
}

fn experience_anchor_pose(spatial: &CosmosSpatialIndex, exp_id: u8) -> ScalePose64 {
    if let Some(ent) = spatial.find_experience_anchor(exp_id) {
        return entity_to_pose(ent);
    }
    if let Some(pl) = all_experience_placements().into_iter().find(|p| p.id.0 == exp_id) {
        return ScalePose64::new(
            pl.sector_id,
            pl.anchor_local,
            pl.focus_scale_exp,
            DQuat::IDENTITY,
        );
    }
    cold_start_earth_pose()
}

fn build_catalog_experience_beats(
    catalog: &UniverseCatalog,
    spatial: &CosmosSpatialIndex,
    exp_id: u8,
) -> Vec<JourneyBeat> {
    let zones = catalog.zones(exp_id);
    let mut beats = Vec::with_capacity(zones.len() * 4);
    let mut prev_u = all_experience_placements()
        .into_iter()
        .find(|p| p.id.0 == exp_id)
        .map_or(0.0, |p| p.focus_scale_exp);

    for zone in zones {
        let z_beat = make_zone_beat(catalog, spatial, zone, prev_u);
        prev_u = z_beat.pose.scale_exp;
        beats.push(z_beat);

        let feats = catalog.features(zone);
        for (f_idx, _) in feats.iter().enumerate() {
            let f_beat = make_feature_beat(spatial, zone, f_idx as u16, prev_u);
            prev_u = f_beat.pose.scale_exp;
            beats.push(f_beat);
        }
    }
    beats
}

fn make_zone_beat(
    catalog: &UniverseCatalog,
    spatial: &CosmosSpatialIndex,
    zone: &cosmos_manifest::ZoneRecord,
    prev_scale_u: f64,
) -> JourneyBeat {
    let exp_id = zone.app_num;
    let z_idx = zone.zone_idx as u16;
    let subject_pose = spatial
        .find_zone_entity(exp_id, z_idx)
        .map(entity_to_pose)
        .unwrap_or_else(|| experience_anchor_pose(spatial, exp_id));

    let view_dir = DVec3::new(
        ((z_idx as f64) * 0.65).sin() * 0.45 + 0.22,
        0.34,
        0.88,
    )
    .normalize();

    let cam_pose = if exp_id == 3 && z_idx == 0 {
        cold_start_earth_pose()
    } else {
        spatial
            .find_zone_entity(exp_id, z_idx)
            .and_then(|e| spatial.navigation_pose_for_entity(e, view_dir))
            .map(|(pose, _)| pose)
            .unwrap_or_else(|| framing_pose_for_subject(&subject_pose, 2.35))
    };

    let delta_u = cam_pose.scale_exp - prev_scale_u;
    let (motion_kind, trans_kind) = if delta_u < -0.5 {
        (MotionProfileKind::ScaleDiveCruise, TransitionKind::LogScaleHyperDive)
    } else if delta_u > 0.5 {
        (MotionProfileKind::HelicalAscent, TransitionKind::ContinuousOrbitBridge)
    } else {
        (MotionProfileKind::OrbitalParallax, TransitionKind::CatmullRomScaleSpline)
    };

    let zone_slug = catalog.str_slice(zone.id);
    JourneyBeat {
        beat_id: format!("exp{exp_id:02}_z{z_idx:03}_{zone_slug}"),
        pose: SerializedPose64::from_pose64(&cam_pose),
        look_at_pose: Some(SerializedPose64::from_pose64(&subject_pose)),
        target: BeatTargetSelector::CatalogZone {
            experience_id: exp_id,
            zone_index: z_idx,
            feature_index: None,
            is_macro_zoomed_out: true,
        },
        motion: BeatMotionProfile {
            kind: motion_kind,
            yaw_deg_per_sec: 5.5,
            pitch_wave_deg: 1.8,
            scale_breathing_du: 0.08,
            orbit_radius_factor: 0.35,
            max_sentences_per_vantage: 2,
            min_dwell_sec: 2.4,
            max_dwell_sec: 16.0,
        },
        transition_in: BeatTransition {
            kind: trans_kind,
            duration_sec: if delta_u.abs() > 2.5 { 2.8 } else { 1.9 },
            narration_lead_sec: 0.35,
        },
        caption_hint: None,
    }
}

fn make_feature_beat(
    spatial: &CosmosSpatialIndex,
    zone: &cosmos_manifest::ZoneRecord,
    feature_idx: u16,
    prev_scale_u: f64,
) -> JourneyBeat {
    let exp_id = zone.app_num;
    let z_idx = zone.zone_idx as u16;
    let subject_pose = spatial
        .find_feature_entity(exp_id, z_idx, feature_idx)
        .or_else(|| spatial.find_zone_entity(exp_id, z_idx))
        .map(entity_to_pose)
        .unwrap_or_else(|| experience_anchor_pose(spatial, exp_id));

    let view_dir = DVec3::new(((feature_idx as f64) * 1.1).cos() * 0.48, 0.28, 0.84).normalize();

    let cam_pose = spatial
        .find_feature_entity(exp_id, z_idx, feature_idx)
        .and_then(|e| spatial.navigation_pose_for_entity(e, view_dir))
        .map(|(pose, _)| pose)
        .unwrap_or_else(|| framing_pose_for_subject(&subject_pose, 1.45));

    let delta_u = cam_pose.scale_exp - prev_scale_u;
    JourneyBeat {
        beat_id: format!("exp{exp_id:02}_z{z_idx:03}_f{feature_idx:03}"),
        pose: SerializedPose64::from_pose64(&cam_pose),
        look_at_pose: Some(SerializedPose64::from_pose64(&subject_pose)),
        target: BeatTargetSelector::CatalogZone {
            experience_id: exp_id,
            zone_index: z_idx,
            feature_index: Some(feature_idx),
            is_macro_zoomed_out: false,
        },
        motion: BeatMotionProfile {
            kind: if feature_idx.is_multiple_of(2) {
                MotionProfileKind::OrbitalParallax
            } else {
                MotionProfileKind::FigureEightShowcase
            },
            yaw_deg_per_sec: 6.2,
            pitch_wave_deg: 2.1,
            scale_breathing_du: 0.06,
            orbit_radius_factor: 0.38,
            max_sentences_per_vantage: 2,
            min_dwell_sec: 2.0,
            max_dwell_sec: 12.0,
        },
        transition_in: BeatTransition {
            kind: if delta_u.abs() > 1.5 {
                TransitionKind::LogScaleHyperDive
            } else {
                TransitionKind::CatmullRomScaleSpline
            },
            duration_sec: 1.45,
            narration_lead_sec: 0.30,
        },
        caption_hint: None,
    }
}

fn framing_pose_for_subject(subject: &ScalePose64, offset_ru: f64) -> ScalePose64 {
    let dir = DVec3::new(0.26, 0.34, 0.90).normalize();
    let cam_pos = subject.local_pos + dir * offset_ru;
    ScalePose64::new(
        subject.sector_id,
        cam_pos,
        subject.scale_exp - 0.15,
        orient_toward(cam_pos, subject.local_pos),
    )
}

#[derive(Copy, Clone)]
struct ColorCandidate {
    rgb: [u8; 3],
    h: f64,
    s: f64,
    l: f64,
    score: f64,
}

fn build_color_key_rgb_table(catalog: &UniverseCatalog) -> Vec<([u8; 3], f64, f64, f64)> {
    let entries = catalog.app_key_entries(1);
    let mut best: Vec<Option<ColorCandidate>> = vec![None; entries.len()];
    let mut name_to_indices: HashMap<&str, Vec<usize>> = HashMap::new();
    for (idx, e) in entries.iter().enumerate() {
        name_to_indices
            .entry(catalog.str_slice(e.key))
            .or_default()
            .push(idx);
    }

    // Coarse dense sweep over RGB space + HSL perceptual space.
    for r in (0..=255).step_by(17) {
        for g in (0..=255).step_by(17) {
            for b in (0..=255).step_by(17) {
                let rgb = [r as u8, g as u8, b as u8];
                let solved = catalog.solve_color(rgb[0], rgb[1], rgb[2]);
                if let Some(idxs) = name_to_indices.get(solved.name.as_str()) {
                    let score = solved.hsl.s * 0.4 + 1.0 - (solved.hsl.l - 0.5).abs();
                    for &idx in idxs {
                        if best[idx].is_none_or(|cur| score > cur.score) {
                            best[idx] = Some(ColorCandidate {
                                rgb,
                                h: solved.hsl.h,
                                s: solved.hsl.s,
                                l: solved.hsl.l,
                                score,
                            });
                        }
                    }
                }
            }
        }
    }

    // Fine chromatic HSL sweep for any narrow gamut color key not matched on the 16^3 RGB cube.
    for idx in 0..entries.len() {
        if best[idx].is_some() {
            continue;
        }
        let target_key = catalog.str_slice(entries[idx].key);
        'outer: for h_step in 0..72 {
            let h = (h_step as f64) * 5.0;
            for s in [0.0, 18.0, 42.0, 68.0, 92.0] {
                for l in [8.0, 20.0, 35.0, 50.0, 65.0, 80.0, 93.0] {
                    let rgb = cosmos_narration::hsl_to_srgb_u8(h, s, l);
                    let solved = catalog.solve_color(rgb[0], rgb[1], rgb[2]);
                    if solved.name == target_key {
                        best[idx] = Some(ColorCandidate {
                            rgb,
                            h: solved.hsl.h,
                            s: solved.hsl.s,
                            l: solved.hsl.l,
                            score: 1.0,
                        });
                        break 'outer;
                    }
                }
            }
        }
    }

    best.into_iter()
        .enumerate()
        .map(|(idx, slot)| {
            slot.map(|c| (c.rgb, c.h, c.s, c.l))
                .unwrap_or_else(|| {
                    let h = ((idx as f64) * 360.0) / (entries.len().max(1) as f64);
                    let rgb = cosmos_narration::hsl_to_srgb_u8(h, 75.0, 52.0);
                    (rgb, h, 0.75, 0.52)
                })
        })
        .collect()
}

fn build_colors_chapter_beats(
    catalog: &UniverseCatalog,
    color_table: &[([u8; 3], f64, f64, f64)],
) -> Vec<JourneyBeat> {
    let entries = catalog.app_key_entries(1);
    let mut order: Vec<usize> = (0..entries.len()).collect();
    order.sort_by(|&a, &b| {
        let (_, ha, _, la) = color_table[a];
        let (_, hb, _, lb) = color_table[b];
        ha.total_cmp(&hb).then_with(|| la.total_cmp(&lb))
    });

    order
        .into_iter()
        .map(|k_idx| {
            let (rgb, h, s, l) = color_table[k_idx];
            make_color_beat(k_idx as u16, rgb, h, s, l)
        })
        .collect()
}

fn make_color_beat(key_index: u16, rgb: [u8; 3], h_deg: f64, s: f64, l: f64) -> JourneyBeat {
    let prism_pt = hsl_to_prism_local(h_deg, s, l);
    let outward = DVec3::new(prism_pt.x, prism_pt.y * 0.4 + 0.15, prism_pt.z)
        .normalize_or_zero();
    let dir = if outward.length_squared() < 1e-4 {
        DVec3::Z
    } else {
        outward
    };
    let cam_pos = prism_pt + dir * 1.15;
    let cam_pose = ScalePose64::new(
        SectorId::COLORS_PRISM,
        cam_pos,
        -0.4,
        orient_toward(cam_pos, prism_pt),
    );
    let look_at = ScalePose64::new(SectorId::COLORS_PRISM, prism_pt, -0.4, DQuat::IDENTITY);
    JourneyBeat {
        beat_id: format!("exp01_color_key_{key_index:03}"),
        pose: SerializedPose64::from_pose64(&cam_pose),
        look_at_pose: Some(SerializedPose64::from_pose64(&look_at)),
        target: BeatTargetSelector::ChromaticKey { key_index, rgb },
        motion: BeatMotionProfile {
            kind: MotionProfileKind::OrbitalParallax,
            yaw_deg_per_sec: 7.0,
            pitch_wave_deg: 2.0,
            scale_breathing_du: 0.04,
            orbit_radius_factor: 0.40,
            max_sentences_per_vantage: 1,
            min_dwell_sec: 1.5,
            max_dwell_sec: 6.0,
        },
        transition_in: BeatTransition {
            kind: TransitionKind::ContinuousOrbitBridge,
            duration_sec: 0.95,
            narration_lead_sec: 0.25,
        },
        caption_hint: None,
    }
}

fn build_music_chapter_beats() -> Vec<JourneyBeat> {
    (48u8..=95u8)
        .enumerate()
        .map(|(i, midi)| {
            let timbre_idx = ((i / 12) + (i % 4)) as u8 % 4;
            make_music_beat(midi, timbre_idx)
        })
        .collect()
}

fn make_music_beat(midi: u8, timbre_idx: u8) -> JourneyBeat {
    let fil_pt = music_filament_local(midi, timbre_idx);
    let cam_pos = fil_pt + DVec3::new(0.24, 0.36, 1.05);
    let cam_pose = ScalePose64::new(
        SectorId::MUSIC_HARP,
        cam_pos,
        0.1,
        orient_toward(cam_pos, fil_pt),
    );
    let look_at = ScalePose64::new(SectorId::MUSIC_HARP, fil_pt, 0.1, DQuat::IDENTITY);
    JourneyBeat {
        beat_id: format!("exp02_midi_{midi}_t{timbre_idx}"),
        pose: SerializedPose64::from_pose64(&cam_pose),
        look_at_pose: Some(SerializedPose64::from_pose64(&look_at)),
        target: BeatTargetSelector::MusicFilament { midi, timbre_idx },
        motion: BeatMotionProfile {
            kind: MotionProfileKind::FigureEightShowcase,
            yaw_deg_per_sec: 6.5,
            pitch_wave_deg: 1.8,
            scale_breathing_du: 0.04,
            orbit_radius_factor: 0.35,
            max_sentences_per_vantage: 1,
            min_dwell_sec: 1.7,
            max_dwell_sec: 7.0,
        },
        transition_in: BeatTransition {
            kind: TransitionKind::ContinuousOrbitBridge,
            duration_sec: 0.9,
            narration_lead_sec: 0.20,
        },
        caption_hint: None,
    }
}

fn ensure_full_audio_coverage(
    catalog: &UniverseCatalog,
    spatial: &CosmosSpatialIndex,
    chapters: &mut [JourneyChapter],
) {
    let mut covered = vec![false; catalog.audio_assets().len()];
    let resolver = TargetResolver::default();

    let mut mark_asset = |id_opt: Option<u32>| {
        if let Some(id) = id_opt
            && let Some(slot) = covered.get_mut(id as usize)
        {
            *slot = true;
        }
    };

    for ch in chapters.iter() {
        for beat in &ch.beats {
            if let Some(focus) = beat.target.to_target_focus()
                && let Ok(plan) = resolver.resolve(catalog, &focus)
            {
                for seg in &plan.segments {
                    mark_asset(seg.assets.es);
                    mark_asset(seg.assets.en);
                    mark_asset(seg.assets.fi);
                    for tok in &seg.assets.extra_tokens {
                        mark_asset(tok.es);
                        mark_asset(tok.en);
                        mark_asset(tok.fi);
                    }
                }
            }
        }
    }

    // Any key entry whose ES/EN/FI audio clip was not yet triggered receives a smooth LexiconKey beat.
    for exp_id in 1u8..=25u8 {
        let entries = catalog.app_key_entries(exp_id);
        let mut extra_beats = Vec::new();
        for (k_idx, entry) in entries.iter().enumerate() {
            let needs_visit = [entry.triple.es_asset, entry.triple.en_asset, entry.triple.fi_asset]
                .into_iter()
                .any(|id| id != NONE_ASSET && !covered[id as usize]);
            if needs_visit {
                for id in [entry.triple.es_asset, entry.triple.en_asset, entry.triple.fi_asset] {
                    if id != NONE_ASSET && let Some(s) = covered.get_mut(id as usize) {
                        *s = true;
                    }
                }
                let subject_anchor = experience_anchor_pose(spatial, exp_id);
                let anchor = framing_pose_for_subject(&subject_anchor, 1.8);
                extra_beats.push(JourneyBeat {
                    beat_id: format!("exp{exp_id:02}_lexicon_{k_idx:04}"),
                    pose: SerializedPose64::from_pose64(&anchor),
                    look_at_pose: Some(SerializedPose64::from_pose64(&subject_anchor)),
                    target: BeatTargetSelector::LexiconKey {
                        experience_id: exp_id,
                        key_index: k_idx as u16,
                    },
                    motion: BeatMotionProfile {
                        kind: MotionProfileKind::OrbitalParallax,
                        max_sentences_per_vantage: 1,
                        min_dwell_sec: 1.5,
                        max_dwell_sec: 6.0,
                        ..Default::default()
                    },
                    transition_in: BeatTransition {
                        duration_sec: 0.95,
                        narration_lead_sec: 0.25,
                        ..Default::default()
                    },
                    caption_hint: None,
                });
            }
        }
        if !extra_beats.is_empty()
            && let Some(ch) = chapters
                .iter_mut()
                .find(|c| c.id.ends_with(&format!("exp{exp_id:02}")))
        {
            ch.beats.extend(extra_beats);
        }
    }
}
