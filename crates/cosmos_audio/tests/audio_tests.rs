use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use cosmos_audio::{
    decode_mp3_bytes, snap_playback_rate, stretch_and_resample_to_48k, AudioEngine,
    AudioOutputMode, FeedbackChimeSynth, SpatialNarrationRenderer, AUDIO_BLOCK_FRAMES,
    OUTPUT_SAMPLE_RATE, SUPPORTED_PLAYBACK_RATES,
};
use cosmos_manifest::{Locale, UniverseCatalog};
use glam::Vec3;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn open_catalog() -> Arc<UniverseCatalog> {
    let dir = workspace_root().join("assets").join("baked");
    let hcvb = dir.join("universe_catalog.hcvb");
    let hcvp = dir.join("universe_audio.hcvp");
    Arc::new(UniverseCatalog::open(&hcvb, Some(hcvp.as_path())).expect("open baked catalog"))
}

fn hrir_path() -> PathBuf {
    workspace_root().join("assets").join("hrir").join("irc_1040_c.bin")
}

#[test]
fn real_mp3_decode_wsola_rates_and_rubato_resample_produce_valid_pcm() {
    let cat = open_catalog();
    let triple = cat
        .find_key_triple(4, "Tierra")
        .expect("Earth key triple present");
    let asset_idx = triple.es_asset;
    let asset = cat.audio_asset(asset_idx).unwrap();
    let mp3 = cat.mp3_slice(asset).unwrap();

    let decoded = decode_mp3_bytes(mp3, asset.duration_ms).expect("decode MP3 bytes");
    assert_eq!(decoded.sample_rate, 24_000);
    assert!(!decoded.samples_mono.is_empty());

    let pcm_1x = stretch_and_resample_to_48k(&decoded, 1.0).expect("1.0x bypass + resample");
    let pcm_175x = stretch_and_resample_to_48k(&decoded, 1.75).expect("1.75x wsola + resample");
    let pcm_3x = stretch_and_resample_to_48k(&decoded, 3.0).expect("3.0x wsola + resample");

    // Expect near 2x frame expansion at 48 kHz for 1.0x, and scaled shorter lengths at 1.75x / 3.0x.
    let expected_1x = decoded.samples_mono.len() * 2;
    assert!(
        (pcm_1x.len() as isize - expected_1x as isize).abs() < 64,
        "1.0x 24k->48k expected ~{expected_1x}, got {}",
        pcm_1x.len()
    );
    assert!(pcm_175x.len() < pcm_1x.len());
    assert!(pcm_3x.len() < pcm_175x.len());

    let ratio_3x = pcm_1x.len() as f32 / pcm_3x.len().max(1) as f32;
    assert!(
        (2.4..=3.6).contains(&ratio_3x),
        "3.0x compressed ratio unexpected: {ratio_3x}"
    );

    for buf in [&pcm_1x, &pcm_175x, &pcm_3x] {
        let peak = buf.iter().copied().map(f32::abs).fold(0.0_f32, f32::max);
        assert!(peak > 0.01 && peak <= 1.25, "unexpected peak amplitude: {peak}");
        assert!(buf.iter().all(|s| s.is_finite()));
    }

    assert_eq!(SUPPORTED_PLAYBACK_RATES.len(), 9);
    assert_eq!(snap_playback_rate(1.78), 1.75);
}

#[test]
fn hrtf_spatial_and_zone_synth_ducking_and_sync_clock_headless() {
    let cat = open_catalog();
    let hrir = hrir_path();
    assert!(hrir.exists(), "missing HRIR asset {}", hrir.display());

    let mut engine = AudioEngine::new(Some(hrir.as_path()), AudioOutputMode::Headless)
        .expect("start headless HRTF audio engine");
    assert!(engine.is_headless());
    assert!(engine.uses_hrtf());

    // Verify binaural ear dominance on the real IRCAM HRIR sphere (steady-state speech band).
    let mut binaural = SpatialNarrationRenderer::from_hrir_file(&hrir).unwrap();
    let tone: [f32; AUDIO_BLOCK_FRAMES] = std::array::from_fn(|i| {
        let t = i as f32 * std::f32::consts::TAU / OUTPUT_SAMPLE_RATE as f32;
        ((450.0 * t).sin() + (1400.0 * t).sin() + (2600.0 * t).sin()) * 0.15
    });
    let mut scratch = [0.0_f32; AUDIO_BLOCK_FRAMES * 2];
    let mut left_out = [0.0_f32; AUDIO_BLOCK_FRAMES * 2];
    let mut right_out = [0.0_f32; AUDIO_BLOCK_FRAMES * 2];
    let left_dir = Vec3::new(-1.0, 0.0, -0.3);
    let right_dir = Vec3::new(1.0, 0.0, -0.3);
    binaural.mix_block(&tone, left_dir, 1.0, &mut scratch);
    binaural.mix_block(&tone, left_dir, 1.0, &mut left_out);
    scratch.fill(0.0);
    binaural.mix_block(&tone, right_dir, 1.0, &mut scratch);
    binaural.mix_block(&tone, right_dir, 1.0, &mut right_out);
    let l_left_e: f32 = left_out.as_chunks::<2>().0.iter().map(|f| f[0] * f[0]).sum();
    let r_left_e: f32 = left_out.as_chunks::<2>().0.iter().map(|f| f[1] * f[1]).sum();
    let l_right_e: f32 = right_out.as_chunks::<2>().0.iter().map(|f| f[0] * f[0]).sum();
    let r_right_e: f32 = right_out.as_chunks::<2>().0.iter().map(|f| f[1] * f[1]).sum();
    assert!(
        l_left_e > r_left_e * 1.25 && r_right_e > l_right_e * 1.25,
        "HRTF should match OpenXR ears (-X -> L, +X -> R): left=({l_left_e},{r_left_e}) right=({l_right_e},{r_right_e})"
    );

    // Set active zone ambient drone and measure unducked baseline RMS.
    let earth = &cat.zones(4)[3];
    engine.set_zone_sound(Some(earth.sound), earth.scene_index);
    engine.set_dwell_progress(0.85);
    engine.snap_playback_rate_bead(1.75, true);

    let mut frame_buf = vec![0.0_f32; 480 * 2]; // 10 ms WASAPI-shaped frames!
    for _ in 0..30 {
        assert!(engine.render_headless_frames(&mut frame_buf));
    }
    let unducked_uniforms = engine.reactive_uniforms();
    assert!(unducked_uniforms.levels_a[1] > 0.005, "ambient_rms should be non-zero");

    // Trigger narration + prefetch neighbor assets, render several blocks, verify ducking & SyncVoice clock.
    let triple = earth.macro_fact;
    engine.prefetch_neighbors(&cat, &[earth.name.es_asset, earth.macro_fact.en_asset]);
    let played_asset = engine
        .play_triple(&cat, triple, Locale::EsEs, Vec3::new(-0.35, 0.1, -0.9))
        .expect("play triple");

    for _ in 0..25 {
        engine.render_headless_frames(&mut frame_buf);
        assert!(frame_buf.iter().all(|s| s.is_finite()));
    }
    let peak = frame_buf.iter().copied().map(f32::abs).fold(0.0_f32, f32::max);
    assert!(peak > 0.01 && peak <= 1.0);

    let speaking_uniforms = engine.reactive_uniforms();
    assert!(speaking_uniforms.levels_a[0] > 0.001, "voice_rms should be active");
    assert!(
        speaking_uniforms.levels_a[1] < unducked_uniforms.levels_a[1] * 0.65,
        "ambient drone should duck smoothly during speech ({} -> {})",
        unducked_uniforms.levels_a[1],
        speaking_uniforms.levels_a[1]
    );
    assert!((speaking_uniforms.levels_b[3] - 1.75).abs() < 0.01);
    assert!(speaking_uniforms.synth_a[0] > 100.0);
    assert!(speaking_uniforms.synth_b[3] >= 0.8);

    let sync = engine.poll_sync_clock(&cat).expect("active sync clock");
    assert_eq!(sync.asset_idx, played_asset);
    assert!(sync.is_speaking);
    assert!(!sync.is_finished);
    assert!(sync.source_ms > 0);
    assert!(!sync.cue_progress.revealed_text.is_empty());

    // Fade-out cancellation over ~120 ms when moving away.
    engine.stop_narration();
    for _ in 0..18 {
        engine.render_headless_frames(&mut frame_buf);
    }
    let after_stop = engine.playback_clock();
    assert!(!after_stop.is_speaking);
    assert!(after_stop.is_finished);

    // Regression: restarting the SAME stopped/finished asset_idx must immediately clear stale
    // `is_finished == true` in both `playback_clock()` and `poll_sync_clock()` before mixer drain.
    let gen_before = engine.play_generation();
    engine
        .play_asset(&cat, played_asset, Vec3::new(0.2, 0.0, -0.95))
        .expect("restart same asset");
    let gen_restart = engine.play_generation();
    assert!(gen_restart > gen_before);
    let pending_sync = engine
        .poll_sync_clock(&cat)
        .expect("pending sync clock on restart");
    assert_eq!(pending_sync.asset_idx, played_asset);
    assert!(pending_sync.is_speaking);
    assert!(!pending_sync.is_finished);
    assert_eq!(pending_sync.source_ms, 0);

    // Rapid same-frame stop -> immediate restart must never emit stale `is_finished == true`
    // and the newer generation must survive the queued StopNarration.
    engine.stop_narration();
    engine
        .play_asset(&cat, played_asset, Vec3::new(-0.1, 0.0, -0.99))
        .expect("rapid stop->restart same asset");
    assert!(engine.play_generation() > gen_restart);
    let rapid_sync = engine
        .poll_sync_clock(&cat)
        .expect("sync clock immediately after rapid stop->restart");
    assert_eq!(rapid_sync.asset_idx, played_asset);
    assert!(rapid_sync.is_speaking);
    assert!(!rapid_sync.is_finished);

    for _ in 0..10 {
        engine.render_headless_frames(&mut frame_buf);
    }
    let live_restart = engine.poll_sync_clock(&cat).expect("live restarted sync clock");
    assert_eq!(live_restart.asset_idx, played_asset);
    assert!(live_restart.is_speaking);
    assert!(!live_restart.is_finished);
    assert!(live_restart.source_ms > 0);

    // Non-blocking background thread safety smoke test for prefetch cache readiness.
    std::thread::sleep(Duration::from_millis(20));
}

#[test]
fn app_02_polyphonic_16_voices_and_all_four_timbres_render_cleanly() {
    let mut engine = AudioEngine::new(None, AudioOutputMode::Headless).expect("headless fallback");
    assert!(!engine.uses_hrtf());

    // Trigger 18 simultaneous notes spanning all 4 timbres to test 16-voice polyphony + stealing.
    for i in 0..18_u64 {
        let midi = 48 + ((i * 2) as u8 % 48);
        let timbre = (i as usize) % 4;
        let pan = -0.8 + (i as f32 / 17.0) * 1.6;
        engine.music_note_on(i + 1, midi, timbre, pan);
    }

    let mut out = vec![0.0_f32; 2048 * 2];
    engine.render_headless_frames(&mut out);
    assert!(out.iter().all(|s| s.is_finite()));
    let peak = out.iter().copied().map(f32::abs).fold(0.0_f32, f32::max);
    assert!(peak > 0.05 && peak <= 1.0, "poly synth peak={peak}");

    let u = engine.reactive_uniforms();
    assert!(u.synth_b[2] > 0.01, "music_energy should be non-zero");

    for i in 0..18_u64 {
        engine.music_note_off(i + 1);
    }
    for _ in 0..40 {
        engine.render_headless_frames(&mut out);
    }
    let u_after = engine.reactive_uniforms();
    assert!(u_after.synth_b[2] < u.synth_b[2] * 0.25, "notes should decay cleanly on release");
}

#[test]
fn dwell_chime_locks_once_without_double_trigger() {
    let mut chimes = FeedbackChimeSynth::new();
    let mut block = [0.0_f32; AUDIO_BLOCK_FRAMES * 2];

    chimes.set_dwell_progress(0.50);
    chimes.set_dwell_progress(0.99);
    chimes.mix_block(AUDIO_BLOCK_FRAMES, &mut block);
    let rms_lock_burst: f32 =
        (block.iter().map(|s| s * s).sum::<f32>() / block.len() as f32).sqrt();

    // Let lock envelope decay well below steady-state dwell shimmer.
    for _ in 0..80 {
        block.fill(0.0);
        chimes.mix_block(AUDIO_BLOCK_FRAMES, &mut block);
    }

    // Advancing dwell from 0.99 -> 1.00 must NOT re-trigger `lock_env = 1.0`.
    chimes.set_dwell_progress(1.0);
    block.fill(0.0);
    chimes.mix_block(AUDIO_BLOCK_FRAMES, &mut block);
    let rms_after_1_0: f32 =
        (block.iter().map(|s| s * s).sum::<f32>() / block.len() as f32).sqrt();

    assert!(
        rms_lock_burst > 0.04 && rms_after_1_0 < rms_lock_burst * 0.65,
        "dwell lock chime must trigger once at >=0.99, not double-trigger at 1.0 (burst={rms_lock_burst}, after={rms_after_1_0})"
    );
}

#[test]
fn auto_output_mode_never_panics_regardless_of_hardware() {
    let hrir = hrir_path();
    let res = AudioEngine::new(Some(hrir.as_path()), AudioOutputMode::Auto);
    assert!(res.is_ok(), "Auto output mode should open WASAPI or fall back cleanly: {res:?}");
}
