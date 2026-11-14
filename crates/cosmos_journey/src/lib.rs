//! Modular Story-Journey plugin runtime, 5-act full-catalog Cosmic Odyssey generator,
//! sentence-paced anti-boredom micro-choreography, elastic thumbstick override with
//! non-occluding peripheral exit pearls, and in-game 3D waypoint/zoom recording.

pub mod director;
pub mod elastic;
pub mod odyssey;
pub mod plugin;
pub mod recorder;
pub mod schema;

pub use director::{
    build_transition_spline, localized_experience_title, orient_toward, JourneyBookmark,
    JourneyDirector, JourneyStepOutput,
};
pub use elastic::{
    ElasticOverrideConfig, ElasticOverrideState, JourneyPlayState, ManualOverrideInput,
    PeripheralChoice, PeripheralPromptVisual,
};
pub use odyssey::{CosmicHighlightsPlugin, GrandCosmicOdysseyPlugin};
pub use plugin::{JourneyRegistry, JourneySummary, JsonFileJourneyPlugin, StoryJourneyPlugin};
pub use recorder::{
    ExportedJourneyPaths, JourneyRecorder, RecordedSample, RecordedWaypoint, rdp_simplify_samples,
};
pub use schema::{
    BeatMotionProfile, BeatTargetSelector, BeatTransition, CURRENT_SCHEMA_VERSION, JourneyBeat,
    JourneyChapter, JourneyError, LocalizedTitle, MotionProfileKind, SHARE_URI_PREFIX,
    SerializedPose64, StoryJourneyManifest, TransitionKind, sector_from_name, sector_to_name,
};

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use cosmos_manifest::{Locale, NONE_ASSET, UniverseCatalog};
    use cosmos_math::{ScalePose64, SectorId, SectorTree};
    use cosmos_narration::TargetResolver;
    use cosmos_spatial::build_default_universe_index;
    use glam::{DQuat, DVec3, Quat, Vec3};

    use super::*;

    fn load_real_catalog() -> UniverseCatalog {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("assets")
            .join("baked");
        let hcvb = root.join("universe_catalog.hcvb");
        let hcvp = root.join("universe_audio.hcvp");
        UniverseCatalog::open(&hcvb, hcvp.exists().then_some(hcvp.as_path()))
            .expect("baked catalog assets")
    }

    #[test]
    fn grand_odyssey_covers_100_percent_of_catalog_audio_assets_and_validates() {
        let catalog = load_real_catalog();
        let spatial = build_default_universe_index();
        let manifest = GrandCosmicOdysseyPlugin
            .build_manifest(&catalog, &spatial)
            .expect("odyssey manifest");

        manifest.validate(Some(&catalog)).expect("manifest validates");

        let mut seen_exps = [false; 26];
        for (_, _, beat) in manifest.flatten_beats() {
            assert!(
                beat.motion.max_sentences_per_vantage <= 2,
                "odyssey beat {} exceeded 2-sentence vantage budget",
                beat.beat_id
            );
            let exp_id = match beat.target {
                BeatTargetSelector::CatalogZone { experience_id, .. }
                | BeatTargetSelector::LexiconKey { experience_id, .. } => Some(experience_id),
                BeatTargetSelector::ChromaticKey { .. } | BeatTargetSelector::ChromaticRgb { .. } => {
                    Some(1)
                }
                BeatTargetSelector::MusicFilament { .. } => Some(2),
                BeatTargetSelector::None => None,
            };
            if let Some(id) = exp_id
                && (1..=25).contains(&id)
            {
                seen_exps[id as usize] = true;
            }
        }
        for (exp_id, &seen) in seen_exps.iter().enumerate().skip(1) {
            assert!(
                seen,
                "odyssey did not visit experience {exp_id:02}"
            );
        }

        let mut covered = vec![false; catalog.audio_assets().len()];
        let resolver = TargetResolver::default();
        let mut mark = |idx_opt: Option<u32>| {
            if let Some(idx) = idx_opt {
                covered[idx as usize] = true;
            }
        };

        for (_, _, beat) in manifest.flatten_beats() {
            if let Some(focus) = beat.target.to_target_focus() {
                let plan = resolver.resolve(&catalog, &focus).expect("target resolves");
                for seg in &plan.segments {
                    mark(seg.assets.es);
                    mark(seg.assets.en);
                    mark(seg.assets.fi);
                    for tok in &seg.assets.extra_tokens {
                        mark(tok.es);
                        mark(tok.en);
                        mark(tok.fi);
                    }
                }
            }
        }

        let covered_count = covered.iter().filter(|&&c| c).count();
        let total = catalog.audio_assets().len();
        assert_eq!(total, 18_898);
        assert_eq!(
            covered_count, total,
            "uncovered audio assets remain: {covered_count}/{total}"
        );
    }

    #[test]
    fn share_uri_roundtrip_and_tamper_detection() {
        let catalog = load_real_catalog();
        let spatial = build_default_universe_index();
        let manifest = CosmicHighlightsPlugin
            .build_manifest(&catalog, &spatial)
            .expect("highlights manifest");

        let uri = manifest.encode_share_uri().expect("encode");
        assert!(uri.starts_with(SHARE_URI_PREFIX));
        let decoded = StoryJourneyManifest::decode_share_uri(&uri).expect("decode");
        assert_eq!(decoded.id, manifest.id);
        assert_eq!(decoded.total_beats(), manifest.total_beats());
        for ((_, _, a), (_, _, b)) in manifest
            .flatten_beats()
            .into_iter()
            .zip(decoded.flatten_beats())
        {
            assert_eq!(a.beat_id, b.beat_id);
            assert_eq!(a.target, b.target);
            assert!((a.pose.scale_exp - b.pose.scale_exp).abs() < 1e-12);
            for i in 0..4 {
                assert!((a.pose.orient_xyzw[i] - b.pose.orient_xyzw[i]).abs() < 1e-12);
            }
        }

        // Corrupt the last payload character -> checksum or decode must reject cleanly.
        let mut tampered = uri.clone();
        let last = tampered.pop().unwrap();
        tampered.push(if last == 'A' { 'B' } else { 'A' });
        assert!(StoryJourneyManifest::decode_share_uri(&tampered).is_err());
    }

    #[test]
    fn elastic_override_keeps_head_turn_invariant_and_spawns_non_foveal_exit_pearls() {
        let mut state = ElasticOverrideState::default();
        let mut play_state = JourneyPlayState::Playing;
        let ghost = ScalePose64::new(
            SectorId::EARTH_ORBIT,
            DVec3::new(0.22, 0.73, 1.01),
            6.6,
            DQuat::IDENTITY,
        );

        // Head turn does NOT feed ManualOverrideInput -> offset stays identically zero.
        state.step(&ManualOverrideInput::default(), 0.5, &mut play_state);
        assert!(state.is_zero());
        assert_eq!(play_state, JourneyPlayState::Playing);

        // Small thumbstick poke moves off rail then magnetically returns when released.
        state.step(
            &ManualOverrideInput {
                move_local: Vec3::new(0.5, 0.0, 0.0),
                ..Default::default()
            },
            0.25,
            &mut play_state,
        );
        assert!(state.offset_local_ru.x > 0.1);
        assert_eq!(play_state, JourneyPlayState::Playing);
        let displaced_x = state.offset_local_ru.x;
        state.step(&ManualOverrideInput::default(), 0.4, &mut play_state);
        assert!(state.offset_local_ru.x < displaced_x);

        // Prolonged strong override triggers PausedByLeash & twin peripheral pearls >20 deg off-axis.
        for _ in 0..25 {
            state.step(
                &ManualOverrideInput {
                    move_local: Vec3::new(1.0, 0.0, 0.0),
                    scale_delta_du: 1.0,
                    ..Default::default()
                },
                0.1,
                &mut play_state,
            );
        }
        assert_eq!(play_state, JourneyPlayState::PausedByLeash);

        let (vis_opt, _) = state.evaluate_peripheral_prompt(
            play_state,
            Vec3::ZERO,
            Quat::IDENTITY,
            Vec3::new(0.5, 0.0, -2.0),
            None,
            None,
            false,
        );
        let vis = vis_opt.expect("peripheral prompt visible");
        let angle_follow = (-Vec3::Z).angle_between(vis.follow_world.normalize()).to_degrees();
        let angle_exit = (-Vec3::Z).angle_between(vis.exit_world.normalize()).to_degrees();
        assert!(angle_follow > 20.0, "FOLLOW entered foveal cone: {angle_follow}");
        assert!(angle_exit > 20.0, "EXIT entered foveal cone: {angle_exit}");

        let tree = SectorTree::default();
        let _composed = state.compose_camera_pose(&tree, &ghost);
    }

    #[test]
    fn sentence_budget_never_parks_camera_stationary_and_recorder_rdp_roundtrips() {
        let catalog = load_real_catalog();
        let spatial = build_default_universe_index();
        let manifest = CosmicHighlightsPlugin
            .build_manifest(&catalog, &spatial)
            .expect("highlights");

        for (_, _, beat) in manifest.flatten_beats() {
            assert!(
                beat.motion.max_sentences_per_vantage <= 2,
                "beat {} exceeded 2-sentence vantage budget",
                beat.beat_id
            );
        }

        let start = ScalePose64::new(
            SectorId::EARTH_ORBIT,
            DVec3::new(0.22, 0.73, 1.01),
            6.6,
            DQuat::IDENTITY,
        );
        let mut director = JourneyDirector::from_manifest(manifest, start).unwrap();
        director.start(&spatial.tree, &start, 0);

        // Advance past spline transition into Dwelling and verify camera never stalls across sentences.
        for _ in 0..80 {
            director.step(
                &spatial.tree,
                0.05,
                &ManualOverrideInput::default(),
                Vec3::ZERO,
                Quat::IDENTITY,
                None,
                None,
                false,
            );
        }
        let p_a = director.ghost_pose().local_pos;
        director.notify_translation_sentence_finished(&spatial.tree);
        director.notify_translation_sentence_finished(&spatial.tree);
        for _ in 0..10 {
            director.step(
                &spatial.tree,
                0.05,
                &ManualOverrideInput::default(),
                Vec3::ZERO,
                Quat::IDENTITY,
                None,
                None,
                false,
            );
        }
        let p_b = director.ghost_pose().local_pos;
        assert!(p_a.distance(p_b) > 1e-3, "camera stalled after 2 sentences");

        // Recorder live trace + RDP decimation + manifest serialization roundtrip.
        let mut rec = JourneyRecorder::new();
        rec.record_manual_waypoint(&start, BeatTargetSelector::None, Some("alpha".into()));
        assert!(rec.toggle_live_trace(&spatial.tree));
        for i in 1..=10 {
            let p = ScalePose64::new(
                SectorId::EARTH_ORBIT,
                DVec3::new(0.22 + i as f64 * 0.15, 0.73, 1.01),
                6.6 + i as f64 * 0.12,
                DQuat::IDENTITY,
            );
            rec.step_live_trace(0.2, &p, BeatTargetSelector::None);
        }
        assert!(!rec.toggle_live_trace(&spatial.tree));
        assert!(rec.waypoint_count() >= 3);
        let recorded = rec
            .build_manifest(
                "unit_test_rec",
                LocalizedTitle::unified("Rec"),
                "tester",
            )
            .unwrap();
        let json = recorded.to_json_pretty().unwrap();
        let reparsed = StoryJourneyManifest::from_json_str(&json).unwrap();
        assert_eq!(reparsed.id, "unit_test_rec");
        let _ = NONE_ASSET;
        let _ = Locale::EnUs;
    }

    #[test]
    fn shipped_highlights_json_asset_loads_and_validates_against_catalog() {
        let catalog = load_real_catalog();
        let spatial = build_default_universe_index();
        let journeys_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("..")
            .join("assets")
            .join("journeys");
        let mut reg = JourneyRegistry::with_builtins();
        let loaded_count = reg
            .discover_in_directory(&journeys_dir)
            .expect("scan assets/journeys");
        assert!(loaded_count >= 1);
        let manifest = reg
            .resolve(
                &journeys_dir.join("cosmic_highlights_tour.hcj.json").to_string_lossy(),
                &catalog,
                &spatial,
            )
            .expect("shipped json validates");
        assert_eq!(manifest.id, "cosmic_highlights_tour");
        assert_eq!(manifest.total_beats(), 44);
    }
}
