use std::path::PathBuf;

use cosmos_manifest::{
    contrast_ink, midi_for_x, midi_to_hz, spanish_color_name, timbre_index_for_y, ExperienceKind,
    Locale, ManifestError, MetricMode, UnitKind, UniverseCatalog, Waveform, AUDIO_STRETCH_BACKEND,
    MAGIC_CAT,
};

fn baked_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("assets")
        .join("baked")
}

fn open_real_catalog(with_audio: bool) -> UniverseCatalog {
    let dir = baked_dir();
    let hcvb = dir.join("universe_catalog.hcvb");
    let hcvp = dir.join("universe_audio.hcvp");
    UniverseCatalog::open(&hcvb, with_audio.then_some(hcvp.as_path()))
        .expect("real baked universe catalog should open and verify cleanly")
}

#[test]
fn real_mmap_catalog_and_audio_pack_verify_all_18898_assets() {
    let cat = open_real_catalog(true);
    assert_eq!(AUDIO_STRETCH_BACKEND, "wsola 0.3.0");
    assert_eq!(cat.header().magic, MAGIC_CAT);
    assert!(cat.has_audio_pack());

    assert_eq!(cat.experiences().len(), 25);
    assert_eq!(cat.all_zones().len(), 376);
    assert_eq!(cat.all_features().len(), 2456);
    assert_eq!(cat.all_key_entries().len(), 6415);
    assert_eq!(cat.audio_assets().len(), 18898);
    assert_eq!(cat.all_cues().len(), 148045);
    assert_eq!(cat.body_layout().len(), 11);
    assert_eq!(cat.all_landmarks().len(), 31);
    assert_eq!(cat.music_notes().len(), 12);
    assert_eq!(cat.music_timbres().len(), 4);

    let (mut es_n, mut en_n, mut fi_n) = (0usize, 0usize, 0usize);
    for asset in cat.audio_assets() {
        match asset.locale_enum().expect("valid locale enum") {
            Locale::EsEs => es_n += 1,
            Locale::EnUs => en_n += 1,
            Locale::FiFi => fi_n += 1,
        }
        let mp3 = cat.mp3_slice(asset).expect("every asset has valid MP3 slice");
        assert_eq!(mp3.len(), asset.mp3_len as usize);
        assert!(mp3.len() >= 16);

        let text = cat.asset_text(asset);
        assert!(!text.is_empty());
        let cues = cat.asset_cues(asset);
        assert!(!cues.is_empty());
        for cue in cues {
            let slice = cat.cue_slice(asset, cue);
            assert_eq!(
                slice.chars().count(),
                (cue.end_scalar - cue.start_scalar) as usize
            );
        }
        let p0 = cat.cue_progress_at_ms(asset, 0);
        assert_eq!(p0.active_cue_index, Some(0));
        let pend = cat.cue_progress_at_ms(asset, asset.duration_ms.saturating_add(100));
        assert_eq!(pend.revealed_text, text);
    }

    assert_eq!(es_n, 6068);
    assert_eq!(en_n, 6415);
    assert_eq!(fi_n, 6415);
}

#[test]
fn real_catalog_matches_v1_json_metadata_and_crc32() {
    let cat = open_real_catalog(false);
    assert!(!cat.has_audio_pack());

    let json_path = baked_dir().join("universe_catalog.v1.json");
    let raw = std::fs::read_to_string(&json_path).expect("read v1.json");
    let v: serde_json::Value = serde_json::from_str(&raw).expect("parse v1.json");

    assert_eq!(v["audio_stretch_backend"], AUDIO_STRETCH_BACKEND);
    assert_eq!(v["counts"]["experiences"], 25);
    assert_eq!(v["counts"]["zones"], 376);
    assert_eq!(v["counts"]["features"], 2456);
    assert_eq!(v["counts"]["audio_assets"], 18898);
    assert_eq!(v["counts"]["cues"], 148045);

    let expected_cat_crc = format!("0x{:08x}", cat.header().catalog_crc32);
    let expected_aud_crc = format!("0x{:08x}", cat.header().audio_crc32);
    assert_eq!(v["binary_artifacts"]["hcvb_crc32"], expected_cat_crc);
    assert_eq!(v["binary_artifacts"]["hcvp_crc32"], expected_aud_crc);
}

#[test]
fn app_01_rgb_color_solver_hits_lexicon_over_full_rgb_cube() {
    let cat = open_real_catalog(false);
    let exp1 = cat.experience(1).unwrap();
    assert_eq!(exp1.kind_enum(), Some(ExperienceKind::ColorSpectrum));
    assert_eq!(exp1.key_count, 130);

    assert_eq!(spanish_color_name(0, 0, 0), "negro");
    assert_eq!(contrast_ink(0, 0, 0), [255, 255, 255]);
    assert_eq!(spanish_color_name(255, 255, 255), "blanco");
    assert_eq!(contrast_ink(255, 255, 255), [10, 10, 14]);
    assert_eq!(spanish_color_name(128, 128, 128), "gris");
    assert_eq!(spanish_color_name(90, 8, 12), "granate");
    assert_eq!(spanish_color_name(110, 68, 28), "marrón");
    assert_eq!(spanish_color_name(245, 232, 188), "ámbar pálido");
    assert_eq!(spanish_color_name(228, 215, 180), "beige");
    assert_eq!(spanish_color_name(240, 196, 20), "dorado");
    assert_eq!(spanish_color_name(80, 90, 20), "verde oliva");
    assert_eq!(spanish_color_name(15, 34, 82), "azul marino");

    // Exhaustive sampled RGB cube (52^3 = 140,608 samples) must always hit an active ES/EN/FI triple.
    for r in (0..=255).step_by(5) {
        for g in (0..=255).step_by(5) {
            for b in (0..=255).step_by(5) {
                let solved = cat.solve_color(r, g, b);
                assert!(
                    solved.triple.is_present(),
                    "RGB ({r},{g},{b}) -> {:?} missing in App 01 lexicon",
                    solved.name
                );
            }
        }
    }
}

#[test]
fn app_02_music_synth_solver_and_lexicon() {
    let cat = open_real_catalog(false);
    let exp2 = cat.experience(2).unwrap();
    assert_eq!(exp2.kind_enum(), Some(ExperienceKind::MusicSynth));
    assert_eq!(exp2.key_count, 20);

    assert_eq!(midi_for_x(0.0), 48);
    assert_eq!(midi_for_x(1.0), 95);
    assert!((midi_to_hz(69) - 440.0).abs() < 1e-12);
    assert_eq!(timbre_index_for_y(0.1), 0);
    assert_eq!(timbre_index_for_y(0.3), 1);
    assert_eq!(timbre_index_for_y(0.6), 2);
    assert_eq!(timbre_index_for_y(0.9), 3);

    let touch = cat.solve_music_touch(21.0 / 48.0 + 0.005, 0.10);
    assert_eq!(touch.midi, 69); // A4
    assert_eq!(touch.octave, 4);
    assert_eq!(cat.str_slice(touch.note.symbol), "A");
    assert_eq!(
        cat.asset_text(cat.audio_asset(touch.note.latin.es_asset).unwrap()),
        "la"
    );
    assert_eq!(touch.timbre.waveform_enum(), Some(Waveform::Sine));
    assert_eq!(
        cat.asset_text(cat.audio_asset(touch.timbre.label.es_asset).unwrap()),
        "cristal"
    );
    assert!(touch.octave_triple.is_present());
    assert_eq!(
        cat.asset_text(cat.audio_asset(touch.octave_triple.es_asset).unwrap()),
        "octava 4"
    );
}

#[test]
fn app_04_sun_fix_macro_facts_and_evidence_class_rules() {
    let cat = open_real_catalog(false);

    // Context-sensitive lexicon disambiguation across apps (same ES key -> distinct EN translations).
    let core_triple = cat.find_key_triple(4, "núcleo").unwrap();
    let nuc_triple = cat.find_key_triple(16, "núcleo").unwrap();
    assert_eq!(
        cat.asset_text(cat.audio_asset(core_triple.en_asset).unwrap()),
        "core"
    );
    assert_eq!(
        cat.asset_text(cat.audio_asset(nuc_triple.en_asset).unwrap()),
        "nucleus"
    );

    // App 03 unit_kind verification.
    let z03 = cat.zones(3);
    assert_eq!(z03.len(), 14);
    assert_eq!(z03[0].metric_mode_enum(), Some(MetricMode::Material));
    assert_eq!(z03[0].unit_kind_enum(), Some(UnitKind::Altitude));
    assert_eq!(z03.last().unwrap().unit_kind_enum(), Some(UnitKind::Depth));

    // App 04 Sun canonical metric fix + Earth continuity + BODY_LAYOUT.
    let z04 = cat.zones(4);
    assert_eq!(z04.len(), 12);
    let sun = &z04[0];
    assert_eq!(cat.str_slice(sun.id), "sun");
    assert_eq!(sun.metric_mode_enum(), Some(MetricMode::SolarCenter));
    assert_eq!(
        cat.str_slice(sun.raw_metric),
        "Centro del sistema solar · estrella · fuente de luz y energía"
    );
    let sun_tokens = cat.metric_tokens(sun);
    assert_eq!(sun_tokens.len(), 3);
    let sun_es: Vec<&str> = sun_tokens
        .iter()
        .map(|t| cat.asset_text(cat.audio_asset(t.triple.es_asset).unwrap()))
        .collect();
    assert_eq!(
        sun_es,
        ["Centro del sistema solar", "estrella", "fuente de luz y energía"]
    );

    let earth = &z04[3];
    assert_eq!(cat.str_slice(earth.id), "earth");
    assert_eq!(earth.metric_mode_enum(), Some(MetricMode::EarthContinuity));
    assert_eq!(cat.metric_tokens(earth).len(), 2);
    assert_eq!(cat.str_slice(earth.period), "365,25 días");

    // Landmarks for Apps 05..07.
    assert_eq!(cat.landmarks(5).count(), 12);
    assert_eq!(cat.landmarks(6).count(), 9);
    assert_eq!(cat.landmarks(7).count(), 10);

    // Check ALL 376 zones macro_fact + evidence_class + footers across 03..25.
    let mut footer_count = 0usize;
    for app_num in 3..=25u8 {
        for z in cat.zones(app_num) {
            assert!(z.macro_fact.is_present(), "missing macro_fact in app {app_num}");
            let fact_en = cat.asset_text(cat.audio_asset(z.macro_fact.en_asset).unwrap());
            assert!(!fact_en.is_empty());

            if app_num >= 11 {
                assert!(z.evidence_class.is_present());
                let expected_default = if app_num == 11 || app_num >= 16 { 1 } else { 0 };
                assert_eq!(z.speak_evidence_default, expected_default);
            } else {
                assert!(!z.evidence_class.is_present());
                assert_eq!(z.speak_evidence_default, 0);
            }

            if app_num >= 21 {
                assert!(!cat.str_slice(z.footer).is_empty(), "missing footer in app {app_num}");
                footer_count += 1;
            }
        }
    }
    assert_eq!(footer_count, 129);
}

#[test]
fn corrupted_or_truncated_buffers_return_typed_errors_without_panicking() {
    assert!(matches!(
        UniverseCatalog::from_bytes(&[], None),
        Err(ManifestError::TruncatedHeader { .. })
    ));

    let valid_hcvb = std::fs::read(baked_dir().join("universe_catalog.hcvb")).unwrap();

    let mut bad_magic = valid_hcvb.clone();
    bad_magic[0] = b'X';
    assert!(matches!(
        UniverseCatalog::from_bytes(&bad_magic, None),
        Err(ManifestError::BadCatalogMagic)
    ));

    let mut bad_ver = valid_hcvb.clone();
    bad_ver[8..12].copy_from_slice(&99_u32.to_le_bytes());
    assert!(matches!(
        UniverseCatalog::from_bytes(&bad_ver, None),
        Err(ManifestError::UnsupportedVersion(99))
    ));

    let mut bad_crc = valid_hcvb.clone();
    let last = bad_crc.len() - 1;
    bad_crc[last] ^= 0xFF;
    assert!(matches!(
        UniverseCatalog::from_bytes(&bad_crc, None),
        Err(ManifestError::CatalogCrcMismatch { .. })
    ));

    // Recompute valid catalog CRC over corrupted table offset to prove bounds checks hold post-CRC.
    let mut bad_sec = valid_hcvb;
    // experiences_off sits at byte offset 32..36.
    bad_sec[32..36].copy_from_slice(&0x7FFF_FFFF_u32.to_le_bytes());
    // Recompute header crc at bytes 16..20? No, experiences_off is inside header (0..128) AND
    // strings/tables bounds are checked on header offsets cleanly!
    let body_crc = crc32fast::hash(&bad_sec[128..]);
    bad_sec[16..20].copy_from_slice(&body_crc.to_le_bytes());
    assert!(matches!(
        UniverseCatalog::from_bytes(&bad_sec, None),
        Err(ManifestError::SectionOutOfBounds {
            name: "experiences",
            ..
        })
    ));

    let pristine = std::fs::read(baked_dir().join("universe_catalog.hcvb")).unwrap();
    assert!(matches!(
        UniverseCatalog::from_bytes(&pristine, Some(&[1, 2, 3])),
        Err(ManifestError::AudioPackSizeMismatch { .. })
    ));

    // Patch header.audio_pack_size (offset 24..32) to 16 bytes so audio header/CRC errors can be tested without 1.1 GB copy.
    let mut small_hdr = pristine;
    small_hdr[24..32].copy_from_slice(&16_u64.to_le_bytes());

    assert!(matches!(
        UniverseCatalog::from_bytes(&small_hdr, Some(&[0_u8; 16])),
        Err(ManifestError::BadAudioMagic)
    ));

    let mut ver_err_aud = [0_u8; 16];
    ver_err_aud[..8].copy_from_slice(b"HCVRAUD1");
    ver_err_aud[8..12].copy_from_slice(&7_u32.to_le_bytes());
    assert!(matches!(
        UniverseCatalog::from_bytes(&small_hdr, Some(&ver_err_aud)),
        Err(ManifestError::UnsupportedAudioVersion(7))
    ));

    let mut crc_err_aud = [0_u8; 16];
    crc_err_aud[..8].copy_from_slice(b"HCVRAUD1");
    crc_err_aud[8..12].copy_from_slice(&1_u32.to_le_bytes());
    assert!(matches!(
        UniverseCatalog::from_bytes(&small_hdr, Some(&crc_err_aud)),
        Err(ManifestError::AudioCrcMismatch { .. })
    ));
}

#[test]
fn zone_at_progress_and_feature_at_cover_all_experiences() {
    let cat = open_real_catalog(false);
    for app_num in 3..=25u8 {
        for step in 0..=20 {
            let p = step as f32 / 20.0;
            let z = cat
                .zone_at_progress(app_num, p)
                .expect("every progress in [0,1] maps to a zone");
            assert_eq!(z.app_num, app_num);
            let (f_idx, feat) = cat
                .feature_at(z, 0.5)
                .expect("every zone has at least one feature");
            assert!(f_idx < z.feature_count as usize);
            assert!(feat.name.is_present());
            assert!(feat.fact.is_present());
            if z.has_sound == 1 {
                assert!(z.sound.waveform_enum().is_some());
                assert!(z.sound.base_hz > 0.0);
            }
        }
    }
}
