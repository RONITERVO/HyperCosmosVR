use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Instant;

use bytemuck::{Pod, Zeroable};
use cosmos_manifest::UniverseCatalog;
use cosmos_math::{camera_view_matrix, reversed_z_infinite_perspective_rh};
use cosmos_typo::{
    ANGULAR_DIST_MAX_M, ANGULAR_DIST_MIN_M, AmbientCandidateAngular, AmbientLabelInput,
    AngularRect, BilingualNarrationSpec, BilingualPhase, CACHE_BIN_FILENAME, CACHE_JSON_FILENAME,
    CameraBasis, DEFAULT_MAX_AMBIENT_LABELS, DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    DEFAULT_VERGENCE_MAX_M, DEFAULT_VERGENCE_MIN_M, FontRole, GlyphInkMode, GlyphInstance,
    MsdfAtlas, MsdfBakeConfig, MsdfPipelineConfig, MsdfTextRenderer, TetherInstance,
    TextPassUniforms, TypoLayoutEngine, apparent_world_height, build_frame_typography,
    build_frame_typography_with_blockers, create_shared_group0_layout,
    cue_reveal_cursor_for_asset, scale_band_alpha, solve_ambient_deoverlap,
};
use glam::{Quat, Vec3};

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root should resolve")
}

fn load_or_bake_workspace_atlas() -> &'static MsdfAtlas {
    static ATLAS: OnceLock<MsdfAtlas> = OnceLock::new();
    ATLAS.get_or_init(|| {
        let root = workspace_root();
        let fonts_dir = root.join("assets/fonts");
        let cache_dir = root.join("assets/baked/msdf_cache");
        MsdfAtlas::load_or_bake(fonts_dir, cache_dir, &MsdfBakeConfig::default())
            .expect("dual-font MSDF atlas should bake or load cleanly")
    })
}

#[test]
fn pod_struct_sizes_match_shader_contracts() {
    assert_eq!(std::mem::size_of::<TextPassUniforms>(), 32);
    assert_eq!(std::mem::size_of::<GlyphInstance>(), 112);
    assert_eq!(std::mem::size_of::<TetherInstance>(), 48);

    let g = GlyphInstance::zeroed();
    let bytes = bytemuck::bytes_of(&g);
    assert_eq!(bytes.len(), 112);
    let t = TetherInstance::zeroed();
    assert_eq!(bytemuck::bytes_of(&t).len(), 48);
}

#[test]
fn dual_font_msdf_bake_coverage_and_fast_cache_load() {
    let root = workspace_root();
    let cache_dir = root.join("assets/baked/msdf_cache");
    let atlas = load_or_bake_workspace_atlas();

    assert_eq!(atlas.width(), 2048);
    assert_eq!(atlas.height(), 2048);
    assert_eq!(atlas.rgba8_pixels().len(), 2048 * 2048 * 4);
    assert!(root.join("assets/baked/msdf_cache").join(CACHE_BIN_FILENAME).is_file());
    assert!(root.join("assets/baked/msdf_cache").join(CACHE_JSON_FILENAME).is_file());

    // Required ASCII + Spanish + Finnish + cosmic/math coverage on both font roles.
    let ascii: String = (0x20u8..=0x7Eu8).map(|b| b as char).collect();
    let spanish = "áéíóúüñÁÉÍÓÚÜÑ¿¡";
    let finnish = "äöåÄÖÅ";
    let symbols = "Ω♯⁻²³⁵°·→—×";

    for role in FontRole::ALL {
        assert!(atlas.covers_string(role, &ascii), "{role:?} missing ASCII");
        assert!(atlas.covers_string(role, spanish), "{role:?} missing Spanish");
        assert!(atlas.covers_string(role, finnish), "{role:?} missing Finnish");
        assert!(atlas.covers_string(role, symbols), "{role:?} missing cosmic/math symbols");
    }

    // Verify 100% coverage of real strings from all 25 experiences in universe_catalog.hcvb!
    let hcvb_path = root.join("assets/baked/universe_catalog.hcvb");
    let catalog = UniverseCatalog::open(&hcvb_path, None).expect("catalog should open");
    for asset in catalog.audio_assets() {
        let text = catalog.asset_text(asset);
        assert!(
            atlas.covers_string(FontRole::DisplayScript, text),
            "DisplayScript missing char in catalog asset text: {text:?}"
        );
        assert!(
            atlas.covers_string(FontRole::UiSans, text),
            "UiSans missing char in catalog asset text: {text:?}"
        );
    }

    // Verify cold-ish cache load speed (< 30 ms) and rebuild resilience on missing dir.
    let _ = MsdfAtlas::load_from_cache(&cache_dir, Some(atlas.fonts_fingerprint()))
        .expect("warmup load");
    let t0 = Instant::now();
    let reloaded = MsdfAtlas::load_from_cache(&cache_dir, Some(atlas.fonts_fingerprint()))
        .expect("cached MSDF binary should load cleanly");
    let elapsed_ms = t0.elapsed().as_secs_f64() * 1000.0;
    println!("MSDF binary cache load time: {elapsed_ms:.2} ms");
    assert!(
        elapsed_ms < 30.0,
        "MSDF cache load took {elapsed_ms:.2} ms (limit < 30 ms)"
    );
    assert_eq!(reloaded.glyph_count(), atlas.glyph_count());

    // Missing cache rebuilds cleanly into a temp directory.
    let tmp_cache = std::env::temp_dir().join("hypercosmos_typo_rebuild_test");
    let _ = std::fs::remove_dir_all(&tmp_cache);
    let rebuilt = MsdfAtlas::load_or_bake(
        root.join("assets/fonts"),
        &tmp_cache,
        &MsdfBakeConfig::default(),
    )
    .expect("missing cache should rebuild cleanly");
    assert_eq!(rebuilt.glyph_count(), atlas.glyph_count());
    let _ = std::fs::remove_dir_all(&tmp_cache);
}

#[test]
fn upright_glyph_uv_and_msdf_tile_orientation() {
    let atlas = load_or_bake_workspace_atlas();

    for role in FontRole::ALL {
        for ch in ['T', 'L', 'A', 'Á', 'ñ', '¿', '¡', 'Ω'] {
            let g = atlas.glyph(role, ch).expect("glyph must exist");
            assert!(g.has_outline, "glyph '{ch}' ({role:?}) must have outline");
            assert!(g.uv_rect[0] < g.uv_rect[2], "u_left < u_right for '{ch}'");
            assert!(g.uv_rect[1] < g.uv_rect[3], "v_top < v_bottom for '{ch}'");
            assert!(
                g.plane_bounds[0] < g.plane_bounds[2] && g.plane_bounds[1] < g.plane_bounds[3],
                "invalid plane_bounds for '{ch}'"
            );
        }
    }

    // Verify upright orientation in UiSans:
    // 'T': top crossbar has solid ink across left/center/right; lower stem has ink ONLY at center!
    let t_top_left = atlas.sample_tile_median(FontRole::UiSans, 'T', 0.22, 0.16).unwrap();
    let t_top_mid = atlas.sample_tile_median(FontRole::UiSans, 'T', 0.50, 0.16).unwrap();
    let t_top_right = atlas.sample_tile_median(FontRole::UiSans, 'T', 0.78, 0.16).unwrap();
    let t_bot_left = atlas.sample_tile_median(FontRole::UiSans, 'T', 0.22, 0.80).unwrap();
    let t_bot_mid = atlas.sample_tile_median(FontRole::UiSans, 'T', 0.50, 0.80).unwrap();
    let t_bot_right = atlas.sample_tile_median(FontRole::UiSans, 'T', 0.78, 0.80).unwrap();
    assert!(
        t_top_left > 0.5 && t_top_mid > 0.5 && t_top_right > 0.5,
        "'T' top crossbar should be filled: {t_top_left}, {t_top_mid}, {t_top_right}"
    );
    assert!(
        t_bot_mid > 0.5 && t_bot_left < 0.5 && t_bot_right < 0.5,
        "'T' bottom should only have center stem: {t_bot_left}, {t_bot_mid}, {t_bot_right}"
    );

    // 'L': top is only left stem; bottom has wide foot extending right!
    let l_top_right = atlas.sample_tile_median(FontRole::UiSans, 'L', 0.78, 0.22).unwrap();
    let l_bot_right = atlas.sample_tile_median(FontRole::UiSans, 'L', 0.78, 0.84).unwrap();
    assert!(
        l_top_right < 0.5 && l_bot_right > 0.5,
        "'L' must be upright (top_right={l_top_right}, bot_right={l_bot_right})"
    );

    // 'Á': top band contains acute accent ink while lower-right corner is empty/outside.
    let g_a_acute = atlas.glyph(FontRole::UiSans, 'Á').unwrap();
    let [rx, ry, rw, rh] = g_a_acute.pixel_rect;
    let mut top_ink_y = Vec::new();
    for dy in 0..rh / 3 {
        for dx in 0..rw {
            let m = atlas
                .sample_tile_median(
                    FontRole::UiSans,
                    'Á',
                    (dx as f32 + 0.5) / rw as f32,
                    (dy as f32 + 0.5) / rh as f32,
                )
                .unwrap();
            if m > 0.5 {
                top_ink_y.push(dy);
            }
        }
    }
    assert!(
        !top_ink_y.is_empty(),
        "'Á' must have acute accent ink in top third of tile (rect={rx},{ry},{rw}x{rh})"
    );

    // Emit a compact upright debug preview image in target/ for visual sanity inspection.
    let preview_dir = workspace_root().join("target");
    let _ = std::fs::create_dir_all(&preview_dir);
    let sample_chars = ['T', 'L', 'A', 'Á', 'ñ', '¿', 'Ω'];
    let cell = 96u32;
    let mut preview = image::GrayImage::new(cell * sample_chars.len() as u32, cell);
    for (ci, &ch) in sample_chars.iter().enumerate() {
        for py in 0..cell {
            for px in 0..cell {
                let m = atlas
                    .sample_tile_median(
                        FontRole::UiSans,
                        ch,
                        (px as f32 + 0.5) / cell as f32,
                        (py as f32 + 0.5) / cell as f32,
                    )
                    .unwrap_or(0.0);
                let v = ((m - 0.5) * 8.0 + 0.5).clamp(0.0, 1.0);
                preview.put_pixel(ci as u32 * cell + px, py, image::Luma([(v * 255.0) as u8]));
            }
        }
    }
    preview
        .save(preview_dir.join("msdf_upright_preview.png"))
        .expect("debug preview PNG should write cleanly");
}

#[test]
fn angular_sizing_and_deoverlap_zero_overlaps_on_64_crowded_labels() {
    let atlas = load_or_bake_workspace_atlas();

    // Apparent angular sizing clamp contract: clamp(dist, 0.85, 18.0) * 2 * tan(angle / 2)
    let ang = 2.0_f32.to_radians();
    let h_near = apparent_world_height(0.2, ang);
    let h_min = apparent_world_height(ANGULAR_DIST_MIN_M, ang);
    let h_mid = apparent_world_height(5.0, ang);
    let h_far = apparent_world_height(500.0, ang);
    let h_max = apparent_world_height(ANGULAR_DIST_MAX_M, ang);
    assert!((h_near - h_min).abs() < 1e-6);
    assert!((h_far - h_max).abs() < 1e-6);
    assert!(h_mid > h_min && h_mid < h_max);

    // Direct 64-label crowded angular solver test
    let mut cands = Vec::with_capacity(64);
    for i in 0..64u64 {
        let col = (i % 8) as f32;
        let row = (i / 8) as f32;
        cands.push(AmbientCandidateAngular {
            id: i + 1,
            salience: (64 - i) as f32,
            rect: AngularRect {
                az_center: (col - 3.5) * 0.8_f32.to_radians(),
                el_center: (row - 3.5) * 0.4_f32.to_radians(),
                half_az: 1.6_f32.to_radians(),
                half_el: 0.55_f32.to_radians(),
            },
        });
    }
    let placed = solve_ambient_deoverlap(
        &cands,
        &[],
        DEFAULT_MAX_AMBIENT_LABELS,
        DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    );
    assert!(!placed.is_empty());
    assert!(placed.len() <= DEFAULT_MAX_AMBIENT_LABELS);
    for (i, a) in placed.iter().enumerate() {
        assert!(a.nudge_el_rad.abs() <= DEFAULT_MAX_VERTICAL_NUDGE_RAD + 1e-5);
        for b in &placed[i + 1..] {
            assert!(
                !a.placed_rect.overlaps(&b.placed_rect),
                "direct de-overlap collision between {} and {}",
                a.id,
                b.id
            );
        }
    }

    // Full 3D scene build with 64 crowded labels + active bilingual narration card
    let cam = CameraBasis::default();
    let mut ambient = Vec::with_capacity(64);
    for i in 0..64u64 {
        let az = (((i % 8) as f32) - 3.5) * 1.1_f32.to_radians();
        let el = (((i / 8) as f32) - 3.5) * 0.8_f32.to_radians();
        let dist = 4.0 + (i % 5) as f32 * 0.7;
        let pos = Vec3::new(az.sin() * dist, el.sin() * dist, -az.cos() * dist);
        ambient.push(AmbientLabelInput::landmark(
            100 + i,
            format!("Región Cósmica {i:02}"),
            pos,
            100.0 - i as f32,
            0.0,
            20.0,
        ));
    }

    let narration = BilingualNarrationSpec {
        anchor_world_pos: Vec3::new(0.0, 0.2, -65.0), // far anchor -> pulls into 1.6..2.8m vergence
        eyebrow: Some("ESPAÑOL → ENGLISH · OBSERVACIÓN DIRECTA".into()),
        remembered_lines: vec!["Sistema Solar · 9,58 UA".into()],
        active_heading: Some("División de Cassini".into()),
        spanish_text: "La brecha oscura esculpida por resonancias gravitacionales.".into(),
        translation_text: Some("The dark gap sculpted by gravitational resonances.".into()),
        footer_text: Some("Horizonte de observación continua".into()),
        phase: BilingualPhase::TranslationOverGhost { trans_reveal: 0.55 },
        ..Default::default()
    };

    let frame = build_frame_typography(
        atlas,
        &cam,
        &ambient,
        Some(&narration),
        DEFAULT_MAX_AMBIENT_LABELS,
        DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    );

    assert!(frame.placed_ambient.len() <= DEFAULT_MAX_AMBIENT_LABELS);
    let narr_rect = frame.active_narration_rect.expect("narration rect must exist");
    let vergence = frame.active_narration_distance_m.expect("vergence dist must exist");
    assert!(
        (DEFAULT_VERGENCE_MIN_M..=DEFAULT_VERGENCE_MAX_M).contains(&vergence),
        "narration vergence {vergence} outside [1.6, 2.8]m"
    );

    for (i, a) in frame.placed_ambient.iter().enumerate() {
        assert!(!a.rect.overlaps(&narr_rect), "ambient {} overlapped active narration", a.id);
        for b in &frame.placed_ambient[i + 1..] {
            assert!(
                !a.rect.overlaps(&b.rect),
                "ambient {} overlapped ambient {}",
                a.id,
                b.id
            );
        }
    }

    // Verify mode 3 (remembered), mode 2 (ghost Spanish at 16% effective alpha), and mode 1 (active sweep)
    let has_remembered = frame
        .glyphs
        .iter()
        .any(|g| (g.ink_params[3] - GlyphInkMode::RememberedMuted as u8 as f32).abs() < 0.01);
    let ghost_glyph = frame
        .glyphs
        .iter()
        .find(|g| (g.ink_params[3] - GlyphInkMode::SpanishGhostRibbon as u8 as f32).abs() < 0.01)
        .expect("ghost Spanish glyphs must be emitted in TranslationOverGhost phase");
    assert!(
        (ghost_glyph.color_hdr[3] * 0.72 - 0.16).abs() < 0.01,
        "ghost Spanish visual opacity should equal 16% (got {})",
        ghost_glyph.color_hdr[3] * 0.72
    );
    let has_active_live_ink = frame.glyphs.iter().any(|g| {
        (g.ink_params[3] - GlyphInkMode::LiveInkActive as u8 as f32).abs() < 0.01
            && (g.ink_params[2] - 0.55).abs() < 1e-4
    });
    assert!(has_remembered);
    assert!(has_active_live_ink);
    assert!(!frame.tethers.is_empty(), "tether filament back to anchor must be emitted");

    // Verify temporal fade tracker + scale-band ramp
    let mut engine = TypoLayoutEngine::default();
    let f1 = engine.build_frame(atlas, &cam, &ambient[..4], None, 0.016);
    let f2 = engine.build_frame(atlas, &cam, &ambient[..4], None, 0.25);
    assert!(f2.placed_ambient[0].alpha > f1.placed_ambient[0].alpha);
    assert_eq!(scale_band_alpha(25.0, 0.0, 20.0, 0.5), 0.0);
    assert!((scale_band_alpha(10.0, 0.0, 20.0, 0.5) - 1.0).abs() < 1e-5);
}

#[test]
fn narration_offset_shelf_blocker_exclusion_and_deduplication() {
    let atlas = load_or_bake_workspace_atlas();
    let cam = CameraBasis {
        fov_y_rad: 68.0_f32.to_radians(),
        aspect: 1600.0 / 900.0,
        ..Default::default()
    };

    let hero_blocker = AngularRect::from_deg(0.0, 0.0, 10.5, 10.0);
    let wrist_ring_blocker = AngularRect::from_deg(-23.0, -16.0, 10.5, 10.5);
    let blockers = [hero_blocker, wrist_ring_blocker];

    let narration = BilingualNarrationSpec {
        anchor_world_pos: Vec3::new(0.0, 0.0, -3.5),
        eyebrow: Some("ES → ENGLISH · SISTEMA SOLAR · 1.50x".into()),
        remembered_lines: vec!["Tierra · Orbita terrestre".into()],
        active_heading: Some("Anillos de Saturno".into()),
        spanish_text: "Cinturones de hielo cristalino orbitando en silencio.".into(),
        translation_text: Some("Belts of crystalline ice orbiting in silence.".into()),
        phase: BilingualPhase::TranslationOverGhost { trans_reveal: 0.65 },
        ..Default::default()
    };

    // Duplicate & paired landmark labels + one directly inside Wrist Ring blocker
    let ambient = vec![
        AmbientLabelInput::landmark(
            1,
            "Vecindario estelar · Stellar neighborhood",
            Vec3::new(1.4, 1.0, -4.0),
            12.0,
            0.0,
            20.0,
        ),
        AmbientLabelInput::landmark(
            2,
            "  vecindario  estelar · stellar neighborhood ",
            Vec3::new(1.5, 1.05, -4.0),
            9.0,
            0.0,
            20.0,
        ),
        AmbientLabelInput::landmark(
            3,
            "Vecindario estelar",
            Vec3::new(1.6, 1.1, -4.0),
            6.0,
            0.0,
            20.0,
        ),
        AmbientLabelInput::landmark(
            4,
            "Sirio · Sirius",
            Vec3::new(-1.8, -1.2, -4.0), // inside Wrist Ring blocker
            10.0,
            0.0,
            20.0,
        ),
        AmbientLabelInput::landmark(
            5,
            "Saturno · Saturn",
            Vec3::new(1.5, 0.65, -4.0),
            11.0,
            0.0,
            20.0,
        ),
    ];

    let frame = build_frame_typography_with_blockers(
        atlas,
        &cam,
        &ambient,
        Some(&narration),
        &blockers,
        DEFAULT_MAX_AMBIENT_LABELS,
        DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    );

    let narr_rect = frame.active_narration_rect.expect("narration rect must exist");
    assert!(
        !narr_rect.overlaps(&hero_blocker),
        "narration block ({narr_rect:?}) overlapped central hero blocker ({hero_blocker:?})"
    );
    assert!(
        !narr_rect.overlaps(&wrist_ring_blocker),
        "narration block ({narr_rect:?}) overlapped Wrist Ring blocker ({wrist_ring_blocker:?})"
    );
    assert!(
        narr_rect.el_center.abs().to_degrees() >= 10.0
            || narr_rect.az_center.abs().to_degrees() >= 15.0,
        "narration card must steer off center onto offset spatial shelf, got {:?}",
        (narr_rect.az_center.to_degrees(), narr_rect.el_center.to_degrees())
    );

    // Only ID 1 should survive from the three "Vecindario estelar" variants; ID 4 inside Wrist Ring is excluded!
    let placed_ids: Vec<u64> = frame.placed_ambient.iter().map(|p| p.id).collect();
    assert!(placed_ids.contains(&1), "highest-salience unique label must survive");
    assert!(!placed_ids.contains(&2), "exact duplicate label must be deduplicated");
    assert!(!placed_ids.contains(&3), "unpaired duplicate prefix label must be deduplicated");
    assert!(!placed_ids.contains(&4), "label overlapping Wrist Ring blocker must be excluded");

    for a in &frame.placed_ambient {
        assert!(!a.rect.overlaps(&hero_blocker));
        assert!(!a.rect.overlaps(&wrist_ring_blocker));
        assert!(!a.rect.overlaps(&narr_rect));
    }
}

#[test]
fn syncvoice_cue_cursor_progresses_zero_to_one() {
    let root = workspace_root();
    let catalog = UniverseCatalog::open(root.join("assets/baked/universe_catalog.hcvb"), None)
        .expect("catalog should open");

    let asset = catalog
        .audio_assets()
        .iter()
        .find(|a| catalog.asset_cues(a).len() >= 4 && a.duration_ms > 500)
        .expect("catalog should contain multi-cue SyncVoice asset");

    let c0 = cue_reveal_cursor_for_asset(&catalog, asset, 0);
    let c_mid = cue_reveal_cursor_for_asset(&catalog, asset, asset.duration_ms / 2);
    let c_end = cue_reveal_cursor_for_asset(&catalog, asset, asset.duration_ms + 50);

    assert!((0.0..=0.25).contains(&c0), "expected start near 0.0, got {c0}");
    assert!(
        c_mid > c0 && c_mid < c_end,
        "expected monotonic progression {c0} < {c_mid} < {c_end}"
    );
    assert!((c_end - 1.0).abs() < 1e-5, "expected end == 1.0, got {c_end}");

    // Fine-grained monotonic check across 20 steps
    let mut prev = 0.0f32;
    for step in 0..=20 {
        let t_ms = (asset.duration_ms * step) / 20;
        let cur = cue_reveal_cursor_for_asset(&catalog, asset, t_ms);
        assert!(cur >= prev - 1e-6 && cur <= 1.0 + 1e-6);
        prev = cur;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct FrameUniformsTest {
    view_proj: [[[f32; 4]; 4]; 2],
    inv_view_proj: [[[f32; 4]; 4]; 2],
    eye_offset: [[f32; 4]; 2],
    cam_forward: [f32; 4],
    cam_right: [f32; 4],
    cam_up: [f32; 4],
    scale_params: [f32; 4],
    post_params: [f32; 4],
    viewport_params: [f32; 4],
    stereo_params: [f32; 4],
    focus_ray: [f32; 4],
    region_accent: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct AudioUniformsTest {
    data: [[f32; 4]; 4],
}

fn find_dxcompiler_dll() -> Option<String> {
    let sdk_path = r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\dxcompiler.dll";
    if Path::new(sdk_path).is_file() {
        return Some(sdk_path.to_string());
    }
    None
}

fn f16_bits_to_f32(bits: u16) -> f32 {
    let sign = ((bits >> 15) & 1) as u32;
    let exp = ((bits >> 10) & 0x1F) as u32;
    let frac = (bits & 0x03FF) as u32;
    let f_bits = if exp == 0 {
        if frac == 0 {
            sign << 31
        } else {
            let mut e = 127 - 14;
            let mut m = frac;
            while (m & 0x0400) == 0 {
                m <<= 1;
                e -= 1;
            }
            m &= 0x03FF;
            (sign << 31) | (e << 23) | (m << 13)
        }
    } else if exp == 31 {
        (sign << 31) | (0xFF << 23) | (frac << 13)
    } else {
        (sign << 31) | ((exp + (127 - 15)) << 23) | (frac << 13)
    };
    f32::from_bits(f_bits)
}

#[test]
fn gpu_mono_and_stereo_multiview_pipeline_render_and_upright_check() {
    let root = workspace_root();
    let atlas = load_or_bake_workspace_atlas();

    let dx12_compiler = match find_dxcompiler_dll() {
        Some(dxc_path) => wgpu::Dx12Compiler::DynamicDxc { dxc_path },
        None => wgpu::Dx12Compiler::default_dynamic_dxc(),
    };

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::DX12,
        backend_options: wgpu::BackendOptions {
            dx12: wgpu::Dx12BackendOptions {
                shader_compiler: dx12_compiler,
                ..Default::default()
            },
            ..Default::default()
        },
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });

    let adapters: Vec<wgpu::Adapter> =
        pollster::block_on(instance.enumerate_adapters(wgpu::Backends::DX12));
    let adapter = adapters
        .into_iter()
        .find(|a| {
            let info = a.get_info();
            info.vendor == 0x10DE && info.device_type == wgpu::DeviceType::DiscreteGpu
        })
        .expect("RTX 5070 discrete DX12 adapter must be present");

    assert!(
        adapter.features().contains(wgpu::Features::MULTIVIEW),
        "DynamicDxc DX12 adapter must support MULTIVIEW"
    );

    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("cosmos_typo_test_device"),
        required_features: wgpu::Features::MULTIVIEW,
        required_limits: wgpu::Limits {
            max_multiview_view_count: 2,
            ..wgpu::Limits::default()
        },
        experimental_features: Default::default(),
        memory_hints: Default::default(),
        trace: Default::default(),
    }))
    .expect("device creation should succeed");

    let group0_layout = create_shared_group0_layout(&device);
    let mut renderer = MsdfTextRenderer::from_shaders_dir(
        &device,
        &queue,
        atlas,
        root.join("shaders"),
        &group0_layout,
        MsdfPipelineConfig::default(),
    )
    .expect("shaders should compile cleanly");
    assert!(renderer.has_stereo_pipelines());

    let w = 256u32;
    let h = 256u32;
    let proj = reversed_z_infinite_perspective_rh(60.0_f32.to_radians(), 1.0, 0.04);
    let view = camera_view_matrix(Quat::IDENTITY);
    let vp = proj * view;
    let inv_vp = vp.inverse();

    let frame_u = FrameUniformsTest {
        view_proj: [vp.to_cols_array_2d(), vp.to_cols_array_2d()],
        inv_view_proj: [inv_vp.to_cols_array_2d(), inv_vp.to_cols_array_2d()],
        eye_offset: [[-0.032, 0.0, 0.0, 1.0], [0.032, 0.0, 0.0, 1.0]],
        cam_forward: [0.0, 0.0, -1.0, 60.0_f32.to_radians()],
        cam_right: [1.0, 0.0, 0.0, 1.0],
        cam_up: [0.0, 1.0, 0.0, 0.04],
        scale_params: [6.6, 0.0, 1.0, 1.0],
        post_params: [1.05, 0.5, 0.0, 1.0],
        viewport_params: [w as f32, h as f32, 1.0 / w as f32, 1.0 / h as f32],
        stereo_params: [2.0, 0.3, 0.0, 0.0],
        focus_ray: [0.0, 0.0, -1.0, 1.0],
        region_accent: [0.35, 0.85, 1.25, 3.0],
    };
    let audio_u = AudioUniformsTest {
        data: [
            [0.5, 0.2, 0.1, 0.1],
            [0.5, 0.0, 0.1, 1.0],
            [0.0; 4],
            [0.0; 4],
        ],
    };

    let frame_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("test_frame_u"),
        size: std::mem::size_of::<FrameUniformsTest>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let audio_buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("test_audio_u"),
        size: std::mem::size_of::<AudioUniformsTest>() as u64,
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    queue.write_buffer(&frame_buf, 0, bytemuck::bytes_of(&frame_u));
    queue.write_buffer(&audio_buf, 0, bytemuck::bytes_of(&audio_u));

    let group0_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("test_group0_bg"),
        layout: &group0_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: frame_buf.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: audio_buf.as_entire_binding(),
            },
        ],
    });

    // Place a single large upright 'T' centered directly in front of camera so we can verify
    // GPU-rendered top crossbar vs bottom stem directly from the framebuffer!
    let g_t = atlas.glyph(FontRole::UiSans, 'T').unwrap();
    let glyph_t = GlyphInstance {
        pos_center: [0.0, 0.0, -2.0, 0.0],
        right_vec: [0.55, 0.0, 0.0, g_t.px_range],
        up_vec: [0.0, 0.65, 0.0, 0.22],
        uv_rect: g_t.uv_rect,
        color_hdr: [1.0, 1.0, 1.0, 1.0],
        halo_color: [0.002, 0.003, 0.006, 0.9],
        ink_params: [0.0, 1.0, 2.0, 0.0],
    };
    let tether = TetherInstance {
        p0_width: [-0.6, -0.75, -2.0, 0.01],
        p1_alpha: [0.6, -0.75, -2.0, 0.9],
        color_hdr: [0.4, 0.9, 1.3, 0.5],
    };
    renderer.upload(
        &device,
        &queue,
        &TextPassUniforms::default(),
        &[glyph_t],
        &[tether],
    );

    // Mono render pass
    let mono_color = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("mono_color"),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba16Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let mono_depth = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("mono_depth"),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    // Stereo multiview render pass (2-layer D2Array)
    let stereo_color = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("stereo_color"),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 2,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba16Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let stereo_depth = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("stereo_depth"),
        size: wgpu::Extent3d {
            width: w,
            height: h,
            depth_or_array_layers: 2,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });

    let mono_c_view = mono_color.create_view(&wgpu::TextureViewDescriptor::default());
    let mono_d_view = mono_depth.create_view(&wgpu::TextureViewDescriptor::default());
    let stereo_c_view = stereo_color.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        array_layer_count: Some(2),
        ..Default::default()
    });
    let stereo_d_view = stereo_depth.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        array_layer_count: Some(2),
        ..Default::default()
    });

    let row_bytes = w * 8; // Rgba16Float = 8 bytes/px, 256 * 8 = 2048 (multiple of 256)
    let layer_bytes = (row_bytes * h) as u64;
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("typo_readback"),
        size: layer_bytes * 3, // mono + stereo layer 0 + stereo layer 1
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    // World text, full near-depth occlusion, then foreground controls against that same occluder.
    for (depth, foreground) in [(0.0, false), (1.0, false), (1.0, true)] {
        renderer.set_foreground_start(foreground.then_some((0, 0)));
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("typo_test_encoder"),
        });

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("mono_typo_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &mono_c_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &mono_d_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(depth), // Reversed-Z far = 0.0
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            renderer.render(&mut pass, Some(&group0_bg), false);
        }

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("stereo_typo_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &stereo_c_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &stereo_d_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(depth),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: std::num::NonZeroU32::new(0b11),
            });
            renderer.render(&mut pass, Some(&group0_bg), true);
        }

        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &mono_color,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(row_bytes),
                    rows_per_image: Some(h),
                },
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &stereo_color,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: layer_bytes,
                    bytes_per_row: Some(row_bytes),
                    rows_per_image: Some(h),
                },
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 2,
            },
        );

        queue.submit(Some(encoder.finish()));

        let slice = readback.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| {
            let _ = tx.send(r);
        });
        let _ = device.poll(wgpu::PollType::wait_indefinitely());
        rx.recv().unwrap().expect("readback map should succeed");

        let mapped = slice
            .get_mapped_range()
            .expect("mapped range should succeed");
        let count_bright = |layer_idx: usize, y_range: std::ops::Range<u32>| -> usize {
            let base = layer_idx * layer_bytes as usize;
            let mut count = 0usize;
            for y in y_range {
                for x in 0..w {
                    let off = base + ((y * w + x) * 8) as usize;
                    let r = f16_bits_to_f32(u16::from_le_bytes([mapped[off], mapped[off + 1]]));
                    if r > 0.5 {
                        count += 1;
                    }
                }
            }
            count
        };

        // In GPU render: top half of 'T' (y in 40..120) has wide horizontal crossbar;
        // lower half of glyph (y in 136..200) has narrow vertical stem -> top has significantly more pixels!
        let mono_top = count_bright(0, 40..120);
        let mono_bot = count_bright(0, 136..200);
        let stereo_l_top = count_bright(1, 40..120);
        let stereo_l_bot = count_bright(1, 136..200);
        let stereo_r_top = count_bright(2, 40..120);
        let stereo_r_bot = count_bright(2, 136..200);

        println!(
            "GPU MSDF 'T' pixels -> mono(top={mono_top}, bot={mono_bot}), stereoL(top={stereo_l_top}, bot={stereo_l_bot}), stereoR(top={stereo_r_top}, bot={stereo_r_bot})"
        );
        if depth == 1.0 && !foreground {
            assert_eq!(
                [
                    mono_top,
                    mono_bot,
                    stereo_l_top,
                    stereo_l_bot,
                    stereo_r_top,
                    stereo_r_bot
                ],
                [0; 6]
            );
        } else {
            assert!(mono_top > 100 && mono_bot > 30 && mono_top > mono_bot + 30);
            assert!(stereo_l_top > 100 && stereo_l_bot > 30 && stereo_l_top > stereo_l_bot + 30);
            assert!(stereo_r_top > 100 && stereo_r_bot > 30 && stereo_r_top > stereo_r_bot + 30);
        }

        drop(mapped);
        readback.unmap();
    }
}

#[test]
fn narration_and_ambient_lock_zero_screen_roll_on_rolled_camera() {
    let atlas = load_or_bake_workspace_atlas();

    // Pitch camera down -24 deg AND roll clockwise +22 deg while world horizon_up stays Vec3::Y.
    let pitch = (-24.0_f32).to_radians();
    let roll = 22.0_f32.to_radians();
    let fwd = Vec3::new(0.0, pitch.sin(), -pitch.cos()).normalize();
    let base_up = Vec3::new(0.0, pitch.cos(), pitch.sin()).normalize();
    let base_right = fwd.cross(base_up).normalize();
    let cam_right = (base_right * roll.cos() - base_up * roll.sin()).normalize();
    let cam_up = (base_right * roll.sin() + base_up * roll.cos()).normalize();

    let cam = CameraBasis {
        eye_center: Vec3::ZERO,
        forward: fwd,
        right: cam_right,
        up: cam_up,
        fov_y_rad: 65.0_f32.to_radians(),
        aspect: 16.0 / 9.0,
        u_cam: 0.0,
        horizon_up: Vec3::Y,
    };

    let hero_blocker = AngularRect::from_deg(0.0, 0.0, 12.0, 11.5);
    let narration = BilingualNarrationSpec {
        anchor_world_pos: fwd * 3.5,
        eyebrow: Some("ES → ENGLISH · TEST".into()),
        active_heading: Some("Átomo de hidrógeno".into()),
        spanish_text: "Órbita cuántica suspendida en silencio profundo.".into(),
        translation_text: Some("Quantum orbital suspended in deep silence.".into()),
        phase: BilingualPhase::TranslationOverGhost { trans_reveal: 0.8 },
        arc_curve: true,
        ..Default::default()
    };

    let off_axis_dir = (fwd * 0.90 + cam_right * 0.30 + cam_up * 0.22).normalize();
    let ambient = vec![AmbientLabelInput::landmark(
        7,
        "Emoción · Emotion",
        off_axis_dir * 3.2,
        12.0,
        -5.0,
        25.0,
    )];

    let frame = build_frame_typography_with_blockers(
        atlas,
        &cam,
        &ambient,
        Some(&narration),
        &[hero_blocker],
        DEFAULT_MAX_AMBIENT_LABELS,
        DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    );

    assert!(!frame.glyphs.is_empty(), "must emit glyphs");
    assert!(
        !frame.placed_ambient.is_empty(),
        "off-axis upper-right ambient label should be placed"
    );
    for (idx, g) in frame.glyphs.iter().enumerate() {
        let r = Vec3::new(g.right_vec[0], g.right_vec[1], g.right_vec[2]).normalize();
        let u = Vec3::new(g.up_vec[0], g.up_vec[1], g.up_vec[2]).normalize();
        assert!(
            r.dot(cam_up).abs() < 1e-4,
            "glyph {idx} horizontal vector tilted vs camera.up: dot={}",
            r.dot(cam_up)
        );
        assert!(
            u.dot(cam_up) > 0.9999 && u.dot(fwd).abs() < 1e-4,
            "glyph {idx} vertical vector rolled away from viewer screen up: dot_up={}, dot_fwd={}",
            u.dot(cam_up),
            u.dot(fwd)
        );
    }

    // Verify ambient label baseline ('E' and 'n' both rest on baseline y=0) has zero net screen-Y tilt.
    let amb = &frame.placed_ambient[0];
    let g_first = &frame.glyphs[amb.glyph_start];
    let g_last = &frame.glyphs[amb.glyph_end - 1];
    let b0 = Vec3::from_slice(&g_first.pos_center[0..3]) - Vec3::from_slice(&g_first.up_vec[0..3]);
    let b1 = Vec3::from_slice(&g_last.pos_center[0..3]) - Vec3::from_slice(&g_last.up_vec[0..3]);
    let y0 = b0.dot(cam_up) / b0.dot(fwd);
    let y1 = b1.dot(cam_up) / b1.dot(fwd);
    let x0 = b0.dot(cam_right) / b0.dot(fwd);
    let x1 = b1.dot(cam_right) / b1.dot(fwd);
    let slope_deg = (y1 - y0).atan2((x1 - x0).abs()).to_degrees().abs();
    assert!(
        slope_deg < 0.05,
        "ambient label exhibited {slope_deg:.3} deg screen tilt"
    );
}

#[test]
fn display_script_block_keeps_generous_vertical_gap_above_and_below() {
    let atlas = load_or_bake_workspace_atlas();
    let cam = CameraBasis::default();

    let eyebrow = "ES → ENGLISH · PRISM";
    let heading = "Átomo gótico y rápido";
    let spanish = "Ágilmente las galaxias espirales brillan bajo luces árticas y profundas";
    let translation = "Gracefully the spiral galaxies shimmer beneath deep arctic skies";
    let footer = "DWELL 1.2s · VR";

    let spec = BilingualNarrationSpec {
        anchor_world_pos: Vec3::new(0.0, 0.0, -2.5),
        eyebrow: Some(eyebrow.into()),
        active_heading: Some(heading.into()),
        spanish_text: spanish.into(),
        translation_text: Some(translation.into()),
        footer_text: Some(footer.into()),
        phase: BilingualPhase::TranslationOverGhost { trans_reveal: 1.0 },
        arc_curve: false,
        ..Default::default()
    };

    let shaped_heading = atlas.shape_text(FontRole::UiSans, heading, Some(spec.max_line_width_em / 0.82));
    let mut shaped_spanish = atlas.shape_text(FontRole::DisplayScript, spanish, Some(spec.max_line_width_em / 0.92));
    if shaped_spanish.lines.len() > 1 {
        let extra = 0.28;
        for g in &mut shaped_spanish.glyphs {
            if g.line_index > 0 {
                g.y_em -= g.line_index as f32 * extra;
            }
        }
    }

    let frame = build_frame_typography_with_blockers(
        atlas,
        &cam,
        &[],
        Some(&spec),
        &[],
        DEFAULT_MAX_AMBIENT_LABELS,
        DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    );
    let dist_m = frame.active_narration_distance_m.expect("narration distance");
    let main_world_h = cosmos_typo::apparent_world_height(dist_m, spec.base_line_angle_rad)
        / atlas.font_metrics(FontRole::DisplayScript).line_height_em.max(1.0);

    let count_outlined = |role: FontRole, text: &str, mul: f32| -> usize {
        atlas
            .shape_text(role, text, Some(spec.max_line_width_em / mul.max(0.35)))
            .glyphs
            .iter()
            .filter(|g| g.metrics.has_outline)
            .count()
    };
    let n_eye = count_outlined(FontRole::UiSans, eyebrow, 0.62);
    let n_head = count_outlined(FontRole::UiSans, heading, 0.82);
    let n_es = count_outlined(FontRole::DisplayScript, spanish, 0.92);
    let n_tr = count_outlined(FontRole::DisplayScript, translation, 1.0);
    let n_foot = count_outlined(FontRole::UiSans, footer, 0.62);
    assert_eq!(frame.glyphs.len(), n_eye + n_head + n_es + n_tr + n_foot);

    let y_bounds = |start: usize, count: usize| -> (f32, f32) {
        let mut lo = f32::INFINITY;
        let mut hi = f32::NEG_INFINITY;
        for g in &frame.glyphs[start..start + count] {
            let cy = g.pos_center[1];
            let hh = g.up_vec[1].abs();
            lo = lo.min(cy - hh);
            hi = hi.max(cy + hh);
        }
        (lo, hi)
    };

    let (head_lo, _) = y_bounds(n_eye, n_head);
    let (es_lo, es_hi) = y_bounds(n_eye + n_head, n_es);
    let (tr_lo, tr_hi) = y_bounds(n_eye + n_head + n_es, n_tr);
    let (_, foot_hi) = y_bounds(n_eye + n_head + n_es + n_tr, n_foot);

    let gap_above_es = head_lo - es_hi;
    let gap_es_to_tr = es_lo - tr_hi;
    let gap_tr_to_foot = tr_lo - foot_hi;
    assert!(
        gap_above_es >= 0.30 * main_world_h,
        "DisplayScript collided too closely with UiSans heading above: gap={gap_above_es:.5}, min={:.5}",
        0.30 * main_world_h
    );
    assert!(
        gap_es_to_tr >= 0.30 * main_world_h,
        "ghost DisplayScript collided with translation DisplayScript below: gap={gap_es_to_tr:.5}"
    );
    assert!(
        gap_tr_to_foot >= 0.30 * main_world_h,
        "DisplayScript translation collided with footer below: gap={gap_tr_to_foot:.5}"
    );

    // Ensure wrapped multi-line rows inside Spanish DisplayScript have clean positive air.
    assert!(
        shaped_spanish.lines.len() >= 2,
        "test string should wrap onto >=2 lines at max_line_width_em={}",
        spec.max_line_width_em
    );
    let es_slice = &frame.glyphs[n_eye + n_head..n_eye + n_head + n_es];
    let outlined_line_idx: Vec<usize> = shaped_spanish
        .glyphs
        .iter()
        .filter(|g| g.metrics.has_outline)
        .map(|g| g.line_index)
        .collect();
    let row0_lo = es_slice
        .iter()
        .zip(&outlined_line_idx)
        .filter(|(_, l)| **l == 0)
        .map(|(g, _)| g.pos_center[1] - g.up_vec[1].abs())
        .fold(f32::INFINITY, f32::min);
    let row1_hi = es_slice
        .iter()
        .zip(&outlined_line_idx)
        .filter(|(_, l)| **l == 1)
        .map(|(g, _)| g.pos_center[1] + g.up_vec[1].abs())
        .fold(f32::NEG_INFINITY, f32::max);
    assert!(
        row0_lo - row1_hi > 0.12 * main_world_h,
        "wrapped DisplayScript rows nearly collided: row0_lo={row0_lo:.5}, row1_hi={row1_hi:.5}"
    );
    let _ = shaped_heading;
}

#[test]
fn ambient_dual_language_landmark_clears_reticle_dot_while_world_exact_stays_centered() {
    use cosmos_typo::AnchorMode;

    let atlas = load_or_bake_workspace_atlas();
    let cam = CameraBasis::default();

    let dot_emocion = Vec3::new(0.0, 0.0, -3.0);
    let dot_symbol = Vec3::new(0.9, 0.25, -3.2);
    let tick_pos = Vec3::new(-0.9, 0.15, -2.5);

    let mut tick_label = AmbientLabelInput::landmark(3, "10^0", tick_pos, 12.0, -10.0, 20.0);
    tick_label.anchor_mode = AnchorMode::WorldExact;
    tick_label.tether_to_world = false;

    let mut emocion =
        AmbientLabelInput::landmark(1, "Emoción · Emotion", dot_emocion, 11.0, -10.0, 20.0);
    emocion.anchor_mode = AnchorMode::ComfortDistanceBand {
        min_m: 1.5,
        max_m: 14.0,
    };
    emocion.tether_to_world = false; // even minor landmarks with '·' must clear their reticle dot!

    let mut symbol =
        AmbientLabelInput::landmark(2, "Símbolo · Symbol", dot_symbol, 10.0, -10.0, 20.0);
    symbol.anchor_mode = AnchorMode::ComfortDistanceBand {
        min_m: 1.5,
        max_m: 14.0,
    };
    symbol.tether_to_world = true;

    let frame = build_frame_typography_with_blockers(
        atlas,
        &cam,
        &[emocion, symbol, tick_label],
        None,
        &[],
        DEFAULT_MAX_AMBIENT_LABELS,
        DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    );

    let find_placed = |id: u64| {
        frame
            .placed_ambient
            .iter()
            .find(|p| p.id == id)
            .expect("label must be placed")
    };

    for (id, anchor) in [(1_u64, dot_emocion), (2_u64, dot_symbol)] {
        let p = find_placed(id);
        let anchor_el = anchor.normalize().dot(cam.up).asin();
        let lowest_glyph_el = frame.glyphs[p.glyph_start..p.glyph_end]
            .iter()
            .map(|g| {
                let bot = Vec3::new(
                    g.pos_center[0],
                    g.pos_center[1] - g.up_vec[1].abs(),
                    g.pos_center[2],
                );
                (bot - cam.eye_center).normalize().dot(cam.up).asin()
            })
            .fold(f32::INFINITY, f32::min);
        assert!(
            lowest_glyph_el - anchor_el > 0.45_f32.to_radians(),
            "landmark {id} did not clear its reticle dot cleanly: lowest={:.3}deg, dot={:.3}deg",
            lowest_glyph_el.to_degrees(),
            anchor_el.to_degrees()
        );

        let tidx = p
            .tether_index
            .expect("lifted dual-language landmark must emit hairline tether to reticle dot");
        let tether = &frame.tethers[tidx];
        // Tether p0 must attach cleanly at the bottom border (below glyph centers, never through '·').
        assert!(
            tether.p0_width[1] < p.world_pos.y,
            "tether p0 ({}) must start below label center ({}) so it never crosses '·'",
            tether.p0_width[1],
            p.world_pos.y
        );
        let p1 = Vec3::from_slice(&tether.p1_alpha[0..3]);
        assert!((p1 - anchor).length() < 1e-4);
    }

    let tick = find_placed(3);
    assert!(tick.tether_index.is_none(), "WorldExact tick must stay tether-free");
    assert!(
        (tick.world_pos - tick_pos).length() < 1e-4,
        "WorldExact tick must remain centered on its anchor position"
    );
}

#[test]
fn long_finnish_narration_stays_inside_safe_fov_margins_and_avoids_blockers() {
    let atlas = load_or_bake_workspace_atlas();
    let fov_y = 65.0_f32.to_radians();
    let aspect = 16.0 / 9.0;
    let cam = CameraBasis {
        fov_y_rad: fov_y,
        aspect,
        ..CameraBasis::default()
    };

    let spec = BilingualNarrationSpec {
        anchor_world_pos: Vec3::new(0.0, 0.0, -3.2),
        eyebrow: Some("ES → SUOMI · HYPER LOTUS · 1.50x".into()),
        remembered_lines: vec!["Valo · Monikulmainen heijastus".into()],
        active_heading: Some("Yli-ulotteinen lootuskukka ja säteittäinen symmetria".into()),
        spanish_text:
            "Pétalos geométricos desplegándose en simetría cuadridimensional luminosa.".into(),
        translation_text: Some(
            "Geometriset terälehdet avautuvat valoisassa neliulotteisessa symmetriassa.".into(),
        ),
        footer_text: Some("KAIKKI ULOTTUVUUDET · KATSOJA KESKELLÄ".into()),
        phase: BilingualPhase::TranslationOverGhost { trans_reveal: 0.9 },
        ..Default::default()
    };
    assert!(
        (spec.max_line_width_em - 24.0).abs() < 1e-5,
        "default max_line_width_em should be tightened to 24.0"
    );

    let hero_blocker = AngularRect::from_deg(0.0, 0.0, 12.0, 11.5);
    let wrist_ring_blocker = AngularRect::from_deg(-23.0, -16.0, 10.5, 10.5);

    let frame = build_frame_typography_with_blockers(
        atlas,
        &cam,
        &[],
        Some(&spec),
        &[hero_blocker, wrist_ring_blocker],
        DEFAULT_MAX_AMBIENT_LABELS,
        DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    );
    let narr_rect = frame.active_narration_rect.expect("narration rect must exist");
    assert!(!narr_rect.overlaps(&hero_blocker));
    assert!(!narr_rect.overlaps(&wrist_ring_blocker));

    let half_fov_y = 0.5 * fov_y;
    let half_fov_x = (half_fov_y.tan() * aspect).atan();
    let min_safe_margin = 2.0_f32.to_radians();

    for (gi, g) in frame.glyphs.iter().enumerate() {
        let c = Vec3::from_slice(&g.pos_center[0..3]);
        let r = Vec3::from_slice(&g.right_vec[0..3]);
        let u = Vec3::from_slice(&g.up_vec[0..3]);
        for (sx, sy) in [(-1.0_f32, -1.0_f32), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
            let corner = c + r * sx + u * sy;
            let z_depth = corner.dot(cam.forward);
            assert!(z_depth > 0.5, "corner behind near plane");
            let ang_x = corner.dot(cam.right).atan2(z_depth).abs();
            let ang_y = corner.dot(cam.up).atan2(z_depth).abs();
            assert!(
                ang_x <= half_fov_x - min_safe_margin,
                "glyph {gi} clipped horizontal safe FOV edge: {ang_x:.3} vs {:.3} rad",
                half_fov_x - min_safe_margin
            );
            assert!(
                ang_y <= half_fov_y - min_safe_margin,
                "glyph {gi} clipped vertical safe FOV edge: {ang_y:.3} vs {:.3} rad",
                half_fov_y - min_safe_margin
            );
        }
    }
}

#[test]
fn dense_panorama_solar_system_landmarks_mandala_suppresses_clutter_and_avoids_geometry() {
    use cosmos_typo::AnchorMode;

    let atlas = load_or_bake_workspace_atlas();
    let cam = CameraBasis {
        fov_y_rad: 68.0_f32.to_radians(),
        aspect: 16.0 / 9.0,
        u_cam: 11.0,
        ..CameraBasis::default()
    };

    // Simulate a 6-lens mandala / dense foreground spheres as external angular blockers
    let mut blockers = vec![AngularRect::from_deg(0.0, 0.0, 11.0, 10.5)];
    for k in 0..6 {
        let ang = (k as f32) * std::f32::consts::TAU / 6.0;
        blockers.push(AngularRect::from_deg(
            18.0 * ang.cos(),
            12.0 * ang.sin(),
            3.5,
            3.5,
        ));
    }

    let mut ambient = Vec::new();
    // 1) Scale Meridian WorldExact tick placed directly on default (0, -20 deg) candidate
    let tick_pos = Vec3::new(0.0, -20.0_f32.to_radians().sin(), -20.0_f32.to_radians().cos()) * 3.0;
    let mut scale_tick = AmbientLabelInput::landmark(900, "10^+10 m", tick_pos, 95.0, -35.0, 30.0);
    scale_tick.anchor_mode = AnchorMode::WorldExact;
    scale_tick.tether_to_world = false;
    ambient.push(scale_tick);

    // 2) Close-up foreground moons/nodes (0.95..1.35 m) that must stay in front of geometry
    let close_positions = [
        ("Ío · Io", Vec3::new(-0.36, 0.24, -1.02), 88.0),
        ("Europa · Europa", Vec3::new(-0.37, 0.21, -1.04), 72.0), // tightly crowded near Io
        ("Ganímedes · Ganymede", Vec3::new(0.45, 0.30, -1.10), 82.0),
        ("Calisto · Callisto", Vec3::new(0.28, -0.18, -1.18), 76.0),
    ];
    for (i, &(name, pos, sal)) in close_positions.iter().enumerate() {
        let mut lbl = AmbientLabelInput::landmark(10 + i as u64, name, pos, sal, -5.0, 25.0);
        lbl.anchor_mode = AnchorMode::ComfortDistanceBand {
            min_m: 1.5,
            max_m: 14.0,
        };
        ambient.push(lbl);
    }

    // 3) 20 dense panoramic bodies & landmarks + 4 peripheral off-FOV labels that must be culled
    for i in 0..20_u64 {
        let az_deg = -32.0 + (i % 7) as f32 * 10.5;
        let el_deg = -14.0 + (i / 7) as f32 * 11.0;
        let dist = 4.5 + (i % 4) as f32 * 3.0;
        let dir = Vec3::new(
            el_deg.to_radians().cos() * az_deg.to_radians().sin(),
            el_deg.to_radians().sin(),
            -el_deg.to_radians().cos() * az_deg.to_radians().cos(),
        );
        ambient.push(AmbientLabelInput::landmark(
            100 + i,
            format!("Región {i:02} · Sector {i:02}"),
            dir * dist,
            50.0 - i as f32,
            -5.0,
            25.0,
        ));
    }
    for j in 0..4_u64 {
        let off_dir = Vec3::new(
            (55.0 + j as f32 * 3.0).to_radians().sin(),
            0.0,
            -(55.0 + j as f32 * 3.0).to_radians().cos(),
        );
        ambient.push(AmbientLabelInput::landmark(
            500 + j,
            format!("Fuera {j} · Offscreen {j}"),
            off_dir * 4.0,
            200.0, // very high salience, still MUST be culled cleanly outside FOV!
            -5.0,
            25.0,
        ));
    }

    // 4) Close opaque surface (1.20 m) forcing foreground card vergence pull in front of geometry
    let narration = BilingualNarrationSpec {
        anchor_world_pos: Vec3::new(0.0, 0.0, -1.20),
        surface_distance_m: Some(1.20),
        eyebrow: Some("ES → ENGLISH · SISTEMA SOLAR".into()),
        active_heading: Some("Lunas galileanas".into()),
        spanish_text: "Mundos oceánicos en resonancia orbital.".into(),
        translation_text: Some("Ocean worlds in orbital resonance.".into()),
        phase: BilingualPhase::TranslationOverGhost { trans_reveal: 0.75 },
        ..Default::default()
    };

    let frame = build_frame_typography_with_blockers(
        atlas,
        &cam,
        &ambient,
        Some(&narration),
        &blockers,
        DEFAULT_MAX_AMBIENT_LABELS,
        DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    );

    let dist_m = frame
        .active_narration_distance_m
        .expect("narration distance must be set");
    assert!(
        (cosmos_typo::ANGULAR_DIST_MIN_M..1.20).contains(&dist_m),
        "close opaque surface should pull narration card cleanly in front of 1.20 m surface (got {dist_m})"
    );
    let narr_rect = frame.active_narration_rect.expect("narration rect");
    for b in &blockers {
        assert!(!narr_rect.overlaps(b), "narration overlapped geometry blocker");
    }

    // Off-screen high-salience candidates must never consume top-18 slots
    let placed_ids: Vec<u64> = frame.placed_ambient.iter().map(|p| p.id).collect();
    for j in 0..4_u64 {
        assert!(
            !placed_ids.contains(&(500 + j)),
            "off-FOV label {} leaked past viewport margin cull",
            500 + j
        );
    }
    assert!(frame.placed_ambient.len() <= DEFAULT_MAX_AMBIENT_LABELS);

    // Close-up moon Io (raw_dist ~ 1.11 m) must NOT be shoved backward behind min_m=1.5 m
    if let Some(io) = frame.placed_ambient.iter().find(|p| p.id == 10) {
        let io_dist = io.world_pos.length();
        assert!(
            io_dist < 1.35,
            "close-up ComfortDistanceBand label pushed behind object to {io_dist:.2} m"
        );
    }

    // Zero overlaps across all placed ambient labels & blockers & Scale Meridian tick dot guard
    for (i, a) in frame.placed_ambient.iter().enumerate() {
        assert!(!a.rect.overlaps(&narr_rect));
        for b in &blockers {
            assert!(!a.rect.overlaps(b));
        }
        for b in &frame.placed_ambient[i + 1..] {
            assert!(!a.rect.overlaps(&b.rect));
        }
    }
    if let Some(tick) = frame.placed_ambient.iter().find(|p| p.id == 900) {
        assert!(
            !tick.rect.overlaps(&narr_rect),
            "narration overlapped WorldExact Scale Meridian tick"
        );
    }
}

#[test]
fn narration_tether_never_pierces_card_header_under_depth_parallax_and_bottom_descenders() {
    use cosmos_typo::{AnchorMode, BilingualPhase};

    let atlas = load_or_bake_workspace_atlas();
    let cam = CameraBasis {
        fov_y_rad: 68.0_f32.to_radians(),
        aspect: 1600.0 / 900.0,
        ..Default::default()
    };

    // 1) Reproduces 23b_perspectives_six_lenses: deep anchor (3.2 m) slightly above fwd while
    // lower blockers push card onto upper-right lateral shelf at ~2.0 m vergence depth.
    let hero_blocker = AngularRect::from_deg(0.0, 0.0, 12.0, 12.0);
    let wrist_ring_blocker = AngularRect::from_deg(-23.0, -16.0, 11.0, 11.0);
    let lower_shelf_blocker = AngularRect::from_deg(0.0, -20.0, 36.0, 10.0);
    let lower_right_blocker = AngularRect::from_deg(26.0, -9.0, 14.0, 8.0);
    let blockers_23b = [
        hero_blocker,
        wrist_ring_blocker,
        lower_shelf_blocker,
        lower_right_blocker,
    ];

    let spec_23b = BilingualNarrationSpec {
        anchor_world_pos: Vec3::new(0.0, 0.14, -3.2),
        eyebrow: Some("ES → ENGLISH · PERSPECTIVAS CÓSMICAS · 1.50x".into()),
        active_heading: Some("Conocimiento complementario".into()),
        spanish_text: "Perspectivas que se corrigen · combinar revela variables ocultas".into(),
        translation_text: Some(
            "Mutually correcting perspectives · combining them reveals hidden variables".into(),
        ),
        footer_text: Some("COMBINAR PERSPECTIVAS CAMBIA EL MAPA DEL PROBLEMA".into()),
        phase: BilingualPhase::Complete,
        ..Default::default()
    };

    let frame_23b = build_frame_typography_with_blockers(
        atlas,
        &cam,
        &[],
        Some(&spec_23b),
        &blockers_23b,
        DEFAULT_MAX_AMBIENT_LABELS,
        DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    );

    assert!(!frame_23b.glyphs.is_empty());
    let tether = frame_23b
        .tethers
        .first()
        .expect("narration tether must exist");
    let fwd = cam.forward.normalize();
    let right = cam.right.normalize();
    let up = cam.up.normalize();

    let mut u_min = f32::INFINITY;
    let mut u_max = f32::NEG_INFINITY;
    let mut v_min = f32::INFINITY;
    let mut v_max = f32::NEG_INFINITY;
    for g in &frame_23b.glyphs {
        let c = Vec3::from_slice(&g.pos_center[0..3]);
        let r = Vec3::from_slice(&g.right_vec[0..3]);
        let uvec = Vec3::from_slice(&g.up_vec[0..3]);
        let to_eye = cam.eye_center - c;
        let dist = to_eye.length().max(1e-4);
        let pull = (to_eye / dist) * g.pos_center[3].min(dist * 0.15);
        for (sx, sy) in [(-1.0_f32, -1.0_f32), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
            let p = c + pull + r * sx + uvec * sy - cam.eye_center;
            let f = p.dot(fwd);
            assert!(f > 0.1);
            let u = p.dot(right) / f;
            let v = p.dot(up) / f;
            u_min = u_min.min(u);
            u_max = u_max.max(u);
            v_min = v_min.min(v);
            v_max = v_max.max(v);
        }
    }

    let p0 = Vec3::from_slice(&tether.p0_width[0..3]);
    let p1 = Vec3::from_slice(&tether.p1_alpha[0..3]);
    for step in 0..=40 {
        let t = step as f32 / 40.0;
        let pt = p0.lerp(p1, t) - cam.eye_center;
        let f = pt.dot(fwd);
        let u = pt.dot(right) / f;
        let v = pt.dot(up) / f;
        let inside_glyph_bbox = u > u_min && u < u_max && v > v_min && v < v_max;
        assert!(
            !inside_glyph_bbox,
            "tether point t={t:.2} ({u:.4}, {v:.4}) pierced card glyph/header bbox [{u_min:.4}..{u_max:.4}] x [{v_min:.4}..{v_max:.4}]"
        );
    }

    // 2) Reproduces 05_cosmic_web: card ends cleanly with DisplayScript "Galaksisäikeet"
    // (no footer) while an ambient calligraphic label sits directly under its right descenders.
    let spec_05 = BilingualNarrationSpec {
        anchor_world_pos: Vec3::new(0.0, 0.0, -3.5),
        eyebrow: Some("ES → SUOMI · RED CÓSMICA · 1.50x".into()),
        spanish_text: "Filamentos galácticos".into(),
        translation_text: Some("Galaksisäikeet".into()),
        footer_text: None,
        phase: BilingualPhase::Complete,
        ..Default::default()
    };
    let card_only = build_frame_typography_with_blockers(
        atlas,
        &cam,
        &[],
        Some(&spec_05),
        &[],
        DEFAULT_MAX_AMBIENT_LABELS,
        DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    );
    let lowest_card_el = card_only
        .glyphs
        .iter()
        .map(|g| {
            let c = Vec3::from_slice(&g.pos_center[0..3]);
            let uvec = Vec3::from_slice(&g.up_vec[0..3]);
            let dir = (c - uvec - cam.eye_center).normalize();
            dir.dot(up).clamp(-0.999, 0.999).asin()
        })
        .fold(f32::INFINITY, f32::min);
    let narr_rect = card_only.active_narration_rect.expect("05 narration rect");
    let under_az = narr_rect.az_center + 0.55 * narr_rect.half_az;
    let under_el = lowest_card_el - 0.85_f32.to_radians();
    let probe_pos =
        (fwd * (under_el.cos() * under_az.cos()) + right * (under_el.cos() * under_az.sin()) + up * under_el.sin())
            * 3.5;
    let mut amb_under = AmbientLabelInput::landmark(777, "Vacío cósmico", probe_pos, 90.0, -5.0, 20.0);
    amb_under.role = FontRole::DisplayScript;
    amb_under.anchor_mode = AnchorMode::ComfortDistanceBand {
        min_m: 1.5,
        max_m: 14.0,
    };

    let frame_05 = build_frame_typography_with_blockers(
        atlas,
        &cam,
        &[amb_under],
        Some(&spec_05),
        &[],
        DEFAULT_MAX_AMBIENT_LABELS,
        DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    );
    for p in &frame_05.placed_ambient {
        let amb_top_el = frame_05.glyphs[p.glyph_start..p.glyph_end]
            .iter()
            .map(|g| {
                let c = Vec3::from_slice(&g.pos_center[0..3]);
                let uvec = Vec3::from_slice(&g.up_vec[0..3]);
                ((c + uvec - cam.eye_center).normalize()).dot(up).clamp(-0.999, 0.999).asin()
            })
            .fold(f32::NEG_INFINITY, f32::max);
        assert!(
            lowest_card_el - amb_top_el > 0.65_f32.to_radians(),
            "ambient label {:.3}deg clipped too close under bottom of Galaksisäikeet ({:.3}deg)",
            amb_top_el.to_degrees(),
            lowest_card_el.to_degrees()
        );
    }
}

#[test]
fn all_landmarks_and_meridian_ticks_clear_dots_and_rings_by_required_margin() {
    use cosmos_typo::AnchorMode;

    let atlas = load_or_bake_workspace_atlas();
    let cam = CameraBasis {
        fov_y_rad: 68.0_f32.to_radians(),
        aspect: 1600.0 / 900.0,
        ..Default::default()
    };
    let fwd = cam.forward.normalize();
    let right = cam.right.normalize();
    let up = cam.up.normalize();

    // 1) Scale Meridian WorldExact labels (`02b`) placed at `tick_pos + radial_right * 0.055`
    let ticks = [
        (950_110_u64, "10^+10 m", Vec3::new(0.49, 0.34, -1.45)),
        (950_107_u64, "10^+7 m", Vec3::new(0.49, 0.08, -1.45)),
        (950_105_u64, "10^+5 m", Vec3::new(0.49, -0.18, -1.45)),
    ];
    let mut inputs = Vec::new();
    for &(id, txt, ring_pos) in &ticks {
        let mut lbl =
            AmbientLabelInput::landmark(id, txt, ring_pos + right * 0.055, 40.0, -50.0, 50.0);
        lbl.role = FontRole::UiSans;
        lbl.target_angle_rad = 0.72_f32.to_radians();
        lbl.anchor_mode = AnchorMode::WorldExact;
        lbl.tether_to_world = false;
        inputs.push(lbl);
    }

    // 2) Single-language Milky Way feature landmarks WITHOUT '·' or tether_to_world (`04`, `04b`)
    let dots = [
        (201_u64, "corriente estelar", Vec3::new(-0.95, -0.75, -3.5)),
        (202_u64, "galaxia satélite", Vec3::new(-0.85, -0.90, -3.5)),
        (203_u64, "estrella", Vec3::new(1.10, -0.78, -3.5)),
        (204_u64, "carril de polvo", Vec3::new(0.95, -0.86, -3.5)),
    ];
    for &(id, txt, pos) in &dots {
        let mut lbl = AmbientLabelInput::landmark(id, txt, pos, 25.0, -50.0, 50.0);
        lbl.role = FontRole::UiSans;
        lbl.target_angle_rad = 0.88_f32.to_radians();
        lbl.anchor_mode = AnchorMode::ComfortDistanceBand {
            min_m: 1.5,
            max_m: 14.0,
        };
        lbl.tether_to_world = false;
        inputs.push(lbl);
    }

    let frame = build_frame_typography_with_blockers(
        atlas,
        &cam,
        &inputs,
        None,
        &[],
        DEFAULT_MAX_AMBIENT_LABELS,
        DEFAULT_MAX_VERTICAL_NUDGE_RAD,
    );

    // Every Scale Meridian tick label must survive and clear its ring by >= ring_radius + 0.55 deg.
    for &(id, txt, ring_pos) in &ticks {
        let p = frame
            .placed_ambient
            .iter()
            .find(|x| x.id == id)
            .unwrap_or_else(|| panic!("Scale Meridian tick {txt} must survive placement"));
        let r_dir = ring_pos.normalize();
        let r_az = r_dir.dot(right).atan2(r_dir.dot(fwd));
        let r_el = r_dir.dot(up).asin();
        let ring_radius_rad = (0.022 / ring_pos.length()).atan();

        let min_ang_dist = frame.glyphs[p.glyph_start..p.glyph_end]
            .iter()
            .flat_map(|g| {
                let c = Vec3::from_slice(&g.pos_center[0..3]);
                let rv = Vec3::from_slice(&g.right_vec[0..3]);
                let uv = Vec3::from_slice(&g.up_vec[0..3]);
                [(-1.0_f32, -1.0_f32), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)]
                    .into_iter()
                    .map(move |(sx, sy)| c + rv * sx + uv * sy)
            })
            .map(|corner| {
                let d = corner.normalize();
                let az = d.dot(right).atan2(d.dot(fwd));
                let el = d.dot(up).asin();
                ((az - r_az).powi(2) + (el - r_el).powi(2)).sqrt()
            })
            .fold(f32::INFINITY, f32::min);

        assert!(
            min_ang_dist >= ring_radius_rad + 0.55_f32.to_radians(),
            "{txt} overlapped meridian ring: dist={:.3}deg < required {:.3}deg",
            min_ang_dist.to_degrees(),
            (ring_radius_rad + 0.55_f32.to_radians()).to_degrees()
        );
    }

    // Every feature landmark ("corriente estelar", "estrella", etc.) must clear EVERY dot by >= dot_radius + 0.55 deg.
    for p in frame.placed_ambient.iter().filter(|x| x.id >= 200) {
        for &(_, dtxt, dpos) in &dots {
            let ddir = dpos.normalize();
            let daz = ddir.dot(right).atan2(ddir.dot(fwd));
            let del = ddir.dot(up).asin();
            let dot_radius_rad = (0.020 / dpos.length()).atan().max(0.38_f32.to_radians());

            let min_dist = frame.glyphs[p.glyph_start..p.glyph_end]
                .iter()
                .flat_map(|g| {
                    let c = Vec3::from_slice(&g.pos_center[0..3]);
                    let rv = Vec3::from_slice(&g.right_vec[0..3]);
                    let uv = Vec3::from_slice(&g.up_vec[0..3]);
                    [(-1.0_f32, -1.0_f32), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)]
                        .into_iter()
                        .map(move |(sx, sy)| c + rv * sx + uv * sy)
                })
                .map(|corner| {
                    let d = corner.normalize();
                    let az = d.dot(right).atan2(d.dot(fwd));
                    let el = d.dot(up).asin();
                    let dx = (az - daz).abs();
                    let dy = (el - del).abs();
                    dx.hypot(dy)
                })
                .fold(f32::INFINITY, f32::min);

            assert!(
                min_dist >= dot_radius_rad + 0.55_f32.to_radians(),
                "landmark id={} collided with dot '{}': dist={:.3}deg < required {:.3}deg",
                p.id,
                dtxt,
                min_dist.to_degrees(),
                (dot_radius_rad + 0.55_f32.to_radians()).to_degrees()
            );
        }
    }
}


