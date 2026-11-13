use std::path::PathBuf;

use cosmos_app::assets::UniverseAssets;
use cosmos_app::engine::UniverseRuntime;
use cosmos_app::headless::run_headless_render;
use cosmos_audio::AudioOutputMode;
use cosmos_manifest::Locale;
use cosmos_math::DEFAULT_DESKTOP_NEAR_RENDER_UNITS;
use cosmos_narration::FsmStateKind;
use cosmos_nav::{DesktopFlightInput, COLD_START_SCALE_EXP};
use cosmos_render::RenderCamera;
use glam::{DVec3, Vec3};

#[test]
fn cold_start_earth_orbit_narration_and_locale_speed_controls() {
    let assets = UniverseAssets::load().expect("locate and load universe assets");
    let mut rt = UniverseRuntime::new(assets, AudioOutputMode::Headless).expect("init runtime");

    // 1. Cold start lands directly in Earth Low Orbit (u = +6.6) with zero 2D UI.
    assert!((rt.flight.pose.scale_exp - COLD_START_SCALE_EXP).abs() < 1e-6);

    let cam = RenderCamera::mono(
        rt.flight.pose.orient.as_quat(),
        72.0_f32.to_radians(),
        16.0 / 9.0,
        DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
    );

    // 2. Stationary gaze settles 0 -> 200 -> 900ms into active SyncVoice narration + live-ink glyphs.
    let mut pkt = rt.step_desktop_frame(
        &DesktopFlightInput::default(),
        Vec3::NEG_Z,
        true, // summon Wrist Resonance Ring
        false,
        false,
        &cam,
        0.016,
        1280,
        720,
    );
    for _ in 0..70 {
        pkt = rt.step_desktop_frame(
            &DesktopFlightInput::default(),
            Vec3::NEG_Z,
            true,
            false,
            false,
            &cam,
            0.016,
            1280,
            720,
        );
    }
    assert_eq!(pkt.active_experience_id, 3);
    assert!(
        matches!(pkt.fsm_state, FsmStateKind::Narrating | FsmStateKind::Complete),
        "expected Narrating or Complete after >900ms stationary gaze, got {:?}",
        pkt.fsm_state
    );
    assert!(!pkt.typo.glyphs.is_empty(), "expected 3D MSDF glyphs active");
    assert!(
        pkt.scene.sprites.len() >= 12,
        "expected 12 Wrist Resonance Ring pearls emitted as luminous sprites"
    );

    // 3. Playback speed bead snap (1.0x -> 1.5x) & Locale switch (EN -> FI).
    let r1 = rt.step_playback_rate(2);
    assert!((r1 - 1.5).abs() < 1e-4);
    assert_eq!(rt.locale, Locale::EnUs);
    rt.toggle_locale();
    assert_eq!(rt.locale, Locale::FiFi);

    // 4. Flight away cancels narration cleanly, Home flight returns smoothly toward Earth.
    let fly = DesktopFlightInput {
        move_axes: DVec3::new(0.0, 0.0, 1.0),
        scroll_delta: 4.0,
        ..Default::default()
    };
    let moved = rt.step_desktop_frame(&fly, Vec3::NEG_Z, false, false, false, &cam, 0.05, 1280, 720);
    assert_eq!(moved.fsm_state, FsmStateKind::Moving);
    rt.trigger_home();
    assert!(rt.flight.is_home_flight_active());
}

#[test]
fn headless_rtx5070_renders_all_signature_experiences_and_benchmark() {
    let out_dir: PathBuf = std::env::temp_dir().join("hypercosmos_app_test_renders");
    let _ = std::fs::remove_dir_all(&out_dir);

    let (shots, bench) =
        run_headless_render(&out_dir, 960, 540, true).expect("headless RTX 5070 render sweep");
    assert!(
        shots.len() >= 25,
        "expected >= 25 verification captures covering all 25 experiences, got {}",
        shots.len()
    );
    let mut covered_experiences = std::collections::BTreeSet::new();
    for s in &shots {
        covered_experiences.insert(s.experience_id);
        assert!(s.path.is_file(), "missing rendered PNG {}", s.path.display());
        assert!(
            s.glyph_count > 10,
            "shot '{}' missing 3D MSDF typography (glyphs={})",
            s.filename,
            s.glyph_count
        );
        assert!(
            s.tether_count >= 13,
            "shot '{}' missing diegetic Wrist Ring or active focal narration tether (tethers={})",
            s.filename,
            s.tether_count
        );
        assert!(
            s.non_black_pixels > 5_000,
            "shot '{}' appears blank (non_black={})",
            s.filename,
            s.non_black_pixels
        );
        assert!(
            s.pure_black_pixels > 500,
            "shot '{}' failed pure #000000 OLED void preservation (pure_black={})",
            s.filename,
            s.pure_black_pixels
        );
    }
    for exp_id in 1_u8..=25 {
        assert!(
            covered_experiences.contains(&exp_id),
            "headless showcase sweep missed experience {exp_id:02}: covered={covered_experiences:?}"
        );
    }
    let b = bench.expect("benchmark sweep returned");
    assert_eq!(b.frame_count, 120);
    assert!(b.mean_ms > 0.0 && b.mean_ms < 50.0);

    let _ = std::fs::remove_dir_all(&out_dir);
}
