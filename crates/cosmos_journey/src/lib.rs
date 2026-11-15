//! Modular Story-Journey plugin runtime, 5-act full-catalog Cosmic Odyssey generator,
//! authored observations and optional orbit choreography, elastic thumbstick override with
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
    fn moving_profile_sentence_hops_and_recorder_rdp_roundtrip() {
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
        let moving_beat = manifest.flatten_beats().iter().position(|(_, _, beat)| {
            beat.motion.kind != MotionProfileKind::StaticHold
        }).expect("remaining Highlights retain moving profiles");
        let mut director = JourneyDirector::from_manifest(manifest, start).unwrap();
        director.start(&spatial.tree, &start, moving_beat);

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
    fn earth_observation_preserves_pose_through_head_turns_and_sentence_boundaries() {
        let catalog = load_real_catalog();
        let spatial = build_default_universe_index();
        let manifest = CosmicHighlightsPlugin.build_manifest(&catalog, &spatial).unwrap();
        let first = &manifest.chapters[0].beats[0];
        let start = first.pose.to_pose64(&first.beat_id).unwrap();
        let mut director = JourneyDirector::from_manifest(manifest, start).unwrap();
        director.start(&spatial.tree, &start, 0);
        for _ in 0..45 {
            director.step(&spatial.tree, 0.05, &ManualOverrideInput::default(), Vec3::ZERO, Quat::IDENTITY, None, None, false);
        }
        let observation = *director.ghost_pose();
        director.notify_beat_narration_finished();
        for frame in 0..300 {
            if frame % 20 == 0 {
                director.notify_translation_sentence_finished(&spatial.tree);
            }
            let head_turn = Quat::from_rotation_y(frame as f32 * 0.01);
            director.step(&spatial.tree, 0.05, &ManualOverrideInput::default(), Vec3::new(0.08, 0.0, 0.0), head_turn, None, None, false);
            assert_eq!(*director.ghost_pose(), observation, "static observation moved at frame {frame}");
            assert_eq!(director.active_beat_index(), 0, "narration skipped the visual observation interval");
        }
    }

    #[test]
    fn earth_highlights_paths_clear_the_surface_and_frame_the_nightside_aurora() {
        let catalog = load_real_catalog();
        let spatial = build_default_universe_index();
        let manifest = CosmicHighlightsPlugin.build_manifest(&catalog, &spatial).unwrap();
        let earth = &manifest.chapters[0].beats;
        let radius = 6.371e6 / 10.0_f64.powf(6.8);
        let required = [(0, None), (6, Some(0)), (5, None), (0, Some(0)), (0, Some(1)), (2, Some(0))];
        for (beat, (zone, feature)) in earth.iter().zip(required) {
            assert_eq!(beat.target, BeatTargetSelector::CatalogZone {
                experience_id: 3,
                zone_index: zone,
                feature_index: feature,
                is_macro_zoomed_out: feature.is_none(),
            });
            assert!(beat.motion.min_dwell_sec >= 18.0);
            assert_eq!(beat.motion.kind, MotionProfileKind::StaticHold);
        }
        for pair in earth.windows(2) {
            let from = pair[0].pose.to_pose64(&pair[0].beat_id).unwrap();
            let to = pair[1].pose.to_pose64(&pair[1].beat_id).unwrap();
            let spline = build_transition_spline(&spatial.tree, &from, &to, pair[1].transition_in.kind).unwrap();
            for sample in 0..=256 {
                let pose = spline.evaluate_without_segment_easing(&spatial.tree, f64::from(sample) / 256.0).unwrap();
                assert!(pose.local_pos.length() > radius * 1.09, "{} crosses Earth at sample {sample}", pair[1].beat_id);
            }
        }
        let wind = earth[3].pose.to_pose64(&earth[3].beat_id).unwrap();
        let sun = DVec3::new(0.62, 0.35, -0.64).normalize();
        assert!(wind.local_pos.dot(sun) > radius * 13.5, "incoming-wind observation must be sunward of the bow shock");
        assert!(wind.local_pos.length() < radius * 24.0, "wind encounter must be closer than the global overview");
        assert_ne!(earth[3].pose, earth[4].pose, "wind encounter and global field overview need distinct viewpoints");
        let close = &earth[5];
        let observer = close.pose.to_pose64(&close.beat_id).unwrap();
        let target = close.look_at_pose.as_ref().unwrap().to_pose64(&close.beat_id).unwrap();
        assert!(target.local_pos.normalize().dot(sun) < -0.5, "auroral target must lie on the nightside");
        assert!((1.015..1.04).contains(&(target.local_pos.length() / radius)), "look at the emitting atmosphere, not Earth's center");
        let to_earth = -observer.local_pos.normalize();
        let forward = observer.orient * DVec3::NEG_Z;
        assert!(forward.angle_between(to_earth).to_degrees() > 50.0, "close observation must look across the horizon");
        let up = observer.orient * DVec3::Y;
        assert!(up.dot(observer.local_pos.normalize()) > 0.8, "southern horizon must put space above and Earth below");
        let global = earth[4].pose.to_pose64(&earth[4].beat_id).unwrap();
        assert!(global.local_pos.length() > radius * 30.0, "global field view must frame the physical bow shock and magnetotail");
    }

    #[test]
    fn solar_observations_preserve_landmarks_and_clear_the_photosphere() {
        let catalog = load_real_catalog();
        let spatial = build_default_universe_index();
        let manifest = CosmicHighlightsPlugin.build_manifest(&catalog, &spatial).unwrap();
        let solar: Vec<_> = manifest.flatten_beats().into_iter()
            .map(|(_, _, beat)| beat)
            .filter(|beat| matches!(beat.target, BeatTargetSelector::CatalogZone {
                experience_id: 4, zone_index: 0, ..
            })).collect();
        assert_eq!(solar.len(), 6);
        let radius = cosmos_spatial::SUN_RADIUS_M * 10.0_f64.powf(-cosmos_spatial::SOLAR_SECTOR_SCALE_EXP);
        for (index, beat) in solar.iter().enumerate() {
            let pose = beat.pose.to_pose64(&beat.beat_id).unwrap();
            assert_eq!(pose.sector_id, SectorId::SOLAR_SYSTEM);
            assert!(pose.local_pos.length() > radius * 1.05, "{} enters the Sun", beat.beat_id);
            assert_eq!(beat.motion.kind, MotionProfileKind::StaticHold);
            assert_eq!(beat.target, BeatTargetSelector::CatalogZone {
                experience_id: 4, zone_index: 0,
                feature_index: index.checked_sub(1).map(|i| i as u16),
                is_macro_zoomed_out: index == 0,
            });
            if index > 0 {
                assert_eq!(beat.transition_in.kind, TransitionKind::SolarOrbitArc);
            }
        }
        for pair in solar.windows(2) {
            let from = pair[0].pose.to_pose64(&pair[0].beat_id).unwrap();
            let to = pair[1].pose.to_pose64(&pair[1].beat_id).unwrap();
            let spline = build_transition_spline(&spatial.tree, &from, &to, TransitionKind::SolarOrbitArc).unwrap();
            for sample in 0..=512 {
                let pose = spline.evaluate_without_segment_easing(&spatial.tree, f64::from(sample) / 512.0).unwrap();
                assert!(pose.local_pos.length() > radius * 1.054,
                    "{} crosses the close observing floor at {sample}", pair[1].beat_id);
            }
        }
        let latitude = 20.0_f64.to_radians();
        let center = DVec3::new(0.0, latitude.sin(), latitude.cos());
        let north = DVec3::new(0.0, latitude.cos(), -latitude.sin());
        let overview = solar[0].pose.to_pose64("whole solar environment").unwrap();
        let eye_r = overview.local_pos / radius;
        let to_view = overview.orient.conjugate();
        // An independent enclosing box contains family1's complete evolving
        // prominence/sheath, including its 3.5-sigma material support. The
        // current 128x65 curve samples at 0/16/24/32/40s span E[-.54,.54],
        // N[-.12,.15], C[.998,2.744] before support. Test generous corners,
        // not only a single apex or the centered photosphere's bounding sphere.
        for aspect in [3400.0 / 3468.0, 1.0, 16.0 / 9.0] {
            let tan_y = 36.0_f64.to_radians().tan();
            let tan_x = tan_y * aspect;
            for east in [-0.65, 0.65] {
                for poleward in [-0.25, 0.25] {
                    for outward in [0.94, 2.90] {
                        let point = DVec3::X * east + north * poleward + center * outward;
                        let local = to_view * (point - eye_r);
                        assert!(local.z < 0.0);
                        assert!((local.x / -local.z / tan_x).abs() < 0.90
                            && (local.y / -local.z / tan_y).abs() < 0.90,
                            "whole-Sun observation must retain eruption context with a margin at aspect {aspect}");
                    }
                }
            }
            let forward = overview.orient * DVec3::NEG_Z;
            let disc_half_angle = (1.02 / eye_r.length()).asin();
            let disc_offset = forward.angle_between(-eye_r);
            assert!(disc_offset + disc_half_angle < tan_x.atan().min(tan_y.atan()) - 3.0_f64.to_radians(),
                "entire photosphere and chromospheric rim must fit with a margin");
            assert!(disc_half_angle > 10.0_f64.to_radians(),
                "whole-Sun context must still show a substantial disc");
        }
        let core = solar[1].pose.to_pose64("core").unwrap();
        assert!(core.local_pos.x > 0.0 && core.local_pos.dot(north) > 0.0 && core.local_pos.dot(center) > 0.0,
            "interior observer must face the real C/E/N cut opening");
        let granules = solar[2].pose.to_pose64("granules").unwrap();
        let forward = granules.orient * DVec3::NEG_Z;
        let look_angle = forward.angle_between(-granules.local_pos.normalize()).to_degrees();
        let horizon_angle = (radius / granules.local_pos.length()).asin().to_degrees();
        assert!((30.0..55.0).contains(&look_angle) && horizon_angle - look_angle < 36.0,
            "granulation needs inspectable foreground cells and a horizon inside the upper field: {look_angle}/{horizon_angle}");
        assert!((granules.orient * DVec3::Y).dot(granules.local_pos.normalize()) > 0.5,
            "radial up must put the photosphere below the observer");
        for beat in &solar[3..=4] {
            assert!(beat.motion.min_dwell_sec >= 60.0, "a full coronal event must fit before autoplay advances");
        }
        let roots = [-0.055, 0.055].map(|east| (center + DVec3::X * east).normalize() * radius);
        let active = solar[3].pose.to_pose64("active region").unwrap();
        let root_separation = (roots[0] - active.local_pos).angle_between(roots[1] - active.local_pos).to_degrees();
        assert!(root_separation > 25.0,
            "the bipole must remain visibly separated, not collapse into an edge-on tower: {root_separation}");
        for index in [3, 4] {
            let observer = solar[index].pose.to_pose64("coronal observation").unwrap();
            let forward = observer.orient * DVec3::NEG_Z;
            for point in [roots[0], roots[1], center * radius * 1.18] {
                let angle = forward.angle_between(point - observer.local_pos).to_degrees();
                assert!(angle < 33.0,
                    "{} must contain both roots and the ordinary arcade inside the 72-degree field: {angle}", solar[index].beat_id);
            }
        }
        let eruption = solar[4].pose.to_pose64("eruption").unwrap();
        let raised_gaze = eruption.orient * DQuat::from_rotation_x(30.0_f64.to_radians()) * DVec3::NEG_Z;
        let late_apex = center * radius * 2.60 - eruption.local_pos;
        assert!(raised_gaze.angle_between(late_apex).to_degrees() < 25.0,
            "the late eruption must be reachable by a comfortable voluntary upward head turn");
    }

    #[test]
    fn highlights_earth_to_solar_cut_has_no_intermediate_camera_frame() {
        let catalog = load_real_catalog();
        let spatial = build_default_universe_index();
        let manifest = CosmicHighlightsPlugin.build_manifest(&catalog, &spatial).unwrap();
        let flat = manifest.flatten_beats();
        let solar_index = flat.iter().position(|(_, _, beat)| beat.target.experience_id() == Some(4)).unwrap();
        let source = flat[solar_index - 1].2.pose.to_pose64("source").unwrap();
        let destination = flat[solar_index].2.pose.to_pose64("destination").unwrap();
        let target = flat[solar_index].2.target.clone();
        assert_eq!(source.sector_id, SectorId::EARTH_ORBIT);
        assert_eq!(destination.sector_id, SectorId::SOLAR_SYSTEM);
        assert_eq!(flat[solar_index].2.transition_in.kind, TransitionKind::InstantBookmark);
        assert_eq!(flat.iter().filter(|(_, _, b)| b.transition_in.kind == TransitionKind::InstantBookmark).count(), 1,
            "only the authored Earth-to-Solar boundary changes transition policy");
        let step = |director: &mut JourneyDirector, dt| {
            director.step(&spatial.tree, dt, &ManualOverrideInput::default(), Vec3::ZERO, Quat::IDENTITY, None, None, false)
        };
        // A previous 50ms implementation exposed several mixed-frame poses at
        // 120Hz; even a zero-duration update must now arrive in a single frame.
        for dt in [0.0, 0.000001, 0.0005, 1.0 / 120.0, 1.0 / 30.0] {
            let mut director = JourneyDirector::from_manifest(manifest.clone(), source).unwrap();
            director.start(&spatial.tree, &source, solar_index);
            assert_eq!(*director.ghost_pose(), source);
            let arrival = step(&mut director, dt);
            assert_eq!(arrival.ghost_pose, Some(destination), "intermediate cut pose at dt={dt}");
            let camera = arrival.desired_camera_pose.unwrap();
            assert_eq!(camera.sector_id, destination.sector_id);
            assert_eq!(camera.local_pos, destination.local_pos);
            assert_eq!(camera.scale_exp, destination.scale_exp);
            assert!(camera.orient.dot(destination.orient).abs() > 1.0 - 1e-12);
            assert_eq!(arrival.trigger_target, Some(target.clone()), "camera and narration must arrive together");
            assert!(step(&mut director, 0.0).trigger_target.is_none(), "arrival narration must trigger only once");
        }
        // A paused chapter seek must not silently resume or cut the view.
        let mut director = JourneyDirector::from_manifest(manifest.clone(), source).unwrap();
        director.start(&spatial.tree, &source, solar_index - 1);
        director.toggle_play_pause(&spatial.tree, &source);
        assert!(director.next_beat(&spatial.tree, &source));
        let paused = step(&mut director, 1.0);
        assert_eq!(paused.ghost_pose, Some(source));
        assert!(paused.trigger_target.is_none());
        director.toggle_play_pause(&spatial.tree, &source);
        let resumed = step(&mut director, 0.0);
        assert_eq!(resumed.ghost_pose, Some(destination));
        assert_eq!(resumed.trigger_target, Some(target.clone()));

        // Normal autoplay keeps the final physical Earth pose until the same
        // update that supplies the Solar target; no reanchored travel is shown.
        let mut director = JourneyDirector::from_manifest(manifest, source).unwrap();
        director.start(&spatial.tree, &source, solar_index - 1);
        step(&mut director, 14.0);
        director.notify_beat_narration_finished();
        let boundary = step(&mut director, 24.0);
        assert!(boundary.beat_changed);
        assert_eq!(boundary.ghost_pose, Some(source));
        let arrival = step(&mut director, 0.0);
        assert_eq!(arrival.ghost_pose, Some(destination));
        assert_eq!(arrival.trigger_target, Some(target));
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
        assert_eq!(manifest.total_beats(), 48);
        let first = &manifest.chapters[0].beats[0];
        let pose = first.pose.to_pose64(&first.beat_id).unwrap();
        assert_eq!(pose.local_pos, cosmos_nav::EARTH_ORBIT_HOME_LOCAL_POS);
        let generated = CosmicHighlightsPlugin
            .build_manifest(&catalog, &spatial)
            .unwrap();
        // Compare the same JSON representation: the parser may round the final
        // binary digit of a generated f64 camera coordinate on deserialization.
        let generated_json = StoryJourneyManifest::from_json_str(&generated.to_json_pretty().unwrap()).unwrap();
        assert_eq!(generated_json.chapters[0], manifest.chapters[0], "shipped Earth chapter drifted from its generator");
        let earth = ScalePose64::new(SectorId::EARTH_ORBIT, DVec3::ZERO, 6.8, DQuat::IDENTITY);
        let relative = pose.project_entity(&spatial.tree, &earth, 6.371e6).unwrap();
        let half_angle = (relative.rel_scale as f64 / relative.distance_render_units)
            .asin()
            .to_degrees();
        assert!(
            (15.0..25.0).contains(&half_angle),
            "overview must fit a 72-degree vertical FOV with margin: {half_angle}"
        );
    }
}
