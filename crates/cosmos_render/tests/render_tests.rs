use cosmos_math::{ScalePose64, SectorId, camera_view_matrix, reversed_z_infinite_perspective_rh};
use cosmos_spatial::build_default_universe_index;
use glam::{DQuat, DVec3, Quat, Vec3};

use cosmos_render::{
    AudioReactiveUniforms, CosmosRenderer, DiegeticUiInput, EyeView, RenderCamera,
    SceneBuildInput, UniverseSceneBuilder,
};

#[test]
fn device_selects_rtx5070_and_extracts_dx12_handles() {
    let renderer = CosmosRenderer::init_desktop(256, 256).expect("RTX 5070 DX12 init");
    let info = renderer.adapter_info();
    assert_eq!(info.backend, wgpu::Backend::Dx12);
    assert_eq!(info.device_type, wgpu::DeviceType::DiscreteGpu);
    assert_eq!(info.vendor, 0x10DE, "expected discrete NVIDIA adapter, got {}", info.name);
    assert!(
        renderer.device().features().contains(wgpu::Features::MULTIVIEW),
        "DynamicDxc must expose Features::MULTIVIEW"
    );

    let (raw_dev, raw_queue) = renderer
        .dx12_raw_device_and_queue()
        .expect("DX12 raw device & queue pointers");
    assert!(!raw_dev.is_null());
    assert!(!raw_queue.is_null());

    // Pin explicitly by LUID (matching how OpenXR D3D12 picks the exact DXGI node)
    let luid = renderer.adapter_luid().expect("DXGI adapter LUID");
    let pinned = CosmosRenderer::init_for_dx12_luid(Some(luid), 128, 128, false)
        .expect("LUID-pinned DX12 init");
    assert_eq!(pinned.adapter_luid(), Some(luid));
}

#[test]
fn offscreen_multiscale_scenes_preserve_oled_void_and_luminous_heroes() {
    let mut renderer = CosmosRenderer::init_desktop(384, 384).expect("RTX 5070 DX12 init");
    let index = build_default_universe_index();
    let mut builder = UniverseSceneBuilder::new();

    let proj = reversed_z_infinite_perspective_rh(72.0_f32.to_radians(), 1.0, 0.02);
    let vp = proj * camera_view_matrix(Quat::IDENTITY);
    let cam = RenderCamera::mono(Quat::IDENTITY, 72.0_f32.to_radians(), 1.0, 0.02);

    // 1. Quantum Spin-Foam Abyss (14 Spacetime, u = -35.0): zero stars on far shell, genuine #000000 OLED void pixels
    {
        let pose = ScalePose64::new(SectorId::PLANCK_ORIGIN, DVec3::new(0.0, 0.0, 3.0), -35.0, DQuat::IDENTITY);
        let mut input = SceneBuildInput::desktop_default(pose, cam, 384, 384);
        input.active_experience_id = 14;
        input.particle_budget = 4096;
        input.time_sec = 1.2;
        let scene = builder.build(&index, &input);
        let pixels = renderer
            .render_to_offscreen_rgba8(&scene, 384, 384, |_, is_stereo| {
                assert!(!is_stereo);
            })
            .expect("quantum render");

        let (void_px, lit_px) = count_void_and_lit(&pixels, 35);
        assert!(
            void_px > 5_000,
            "expected genuine #000000 OLED void pixels at u=-35, found {void_px}"
        );
        assert!(lit_px > 1_000, "expected luminous quantum lattice/particles, found {lit_px}");
    }

    // 2. Deep Earth Cutaway & Volumetric Aurora (03 Deep Earth, u = 6.65)
    {
        let pose = ScalePose64::new(SectorId::EARTH_ORBIT, DVec3::new(0.0, 0.0, 4.2), 6.65, DQuat::IDENTITY);
        let mut input = SceneBuildInput::desktop_default(pose, cam, 384, 384);
        input.active_experience_id = 3;
        input.earth_cutaway_blend = 1.0;
        input.particle_budget = 8192;
        input.diegetic = DiegeticUiInput {
            focus_ray: Vec3::NEG_Z,
            dwell_progress: 0.65,
            show_focus_shimmer: true,
            wrist_pos: Some(Vec3::new(-0.22, -0.25, -0.45)),
            wrist_orient: Quat::IDENTITY,
            playback_rate: 1.75,
            locale_idx: 0,
        };
        let scene = builder.build(&index, &input);
        let pixels = renderer
            .render_to_offscreen_rgba8(&scene, 384, 384, |_, _| {})
            .expect("earth cutaway render");
        let (void_px, lit_px) = count_void_and_lit(&pixels, 45);
        assert!(void_px > 10_000, "Earth space background must keep #000000 OLED void, got {void_px}");
        assert!(lit_px > 2_500, "Earth + aurora + core cutaway must emit bright pixels, got {lit_px}");
    }

    // 3. App 01 Volumetric HSL Prism inside Mind (must NOT leak Earth cutaway!)
    {
        let pose = ScalePose64::new(SectorId::COLORS_PRISM, DVec3::new(0.0, 0.0, 3.0), 2.0, DQuat::IDENTITY);
        let mut input = SceneBuildInput::desktop_default(pose, cam, 384, 384);
        input.active_experience_id = 1;
        input.particle_budget = 4096;
        let scene = builder.build(&index, &input);
        assert!(
            !scene.heroes_surface.iter().any(|h| h.kind_tags[0] == 0),
            "Earth cutaway must not render inside 01 Colors at u=2.0"
        );
        let pixels = renderer
            .render_to_offscreen_rgba8(&scene, 384, 384, |_, _| {})
            .expect("colors prism render");
        let (void_px, lit_px) = count_void_and_lit(&pixels, 40);
        assert!(void_px > 10_000, "Colors prism void must remain pure black, got {void_px}");
        assert!(lit_px > 2_000, "Colors HSL prism should illuminate viewport, got {lit_px}");
    }

    // 4. App 02 Cymatic Harp + Live Audio Reactive Bus
    {
        let pose = ScalePose64::new(SectorId::MUSIC_HARP, DVec3::new(0.0, 0.0, 3.0), 2.0, DQuat::IDENTITY);
        let mut input = SceneBuildInput::desktop_default(pose, cam, 384, 384);
        input.active_experience_id = 2;
        input.particle_budget = 4096;
        input.audio = AudioReactiveUniforms {
            levels_a: [0.6, 0.3, 0.85, 0.5],
            levels_b: [0.7, 0.25, 0.4, 1.5],
            synth_a: [110.0, 1200.0, 0.9, 0.1],
            synth_b: [0.35, 2.0, 0.95, 1.0],
        };
        let scene = builder.build(&index, &input);
        assert!(
            !scene.heroes_surface.iter().any(|h| h.kind_tags[0] == 0),
            "Earth cutaway must not render inside 02 Music at u=2.0"
        );
        let pixels = renderer
            .render_to_offscreen_rgba8(&scene, 384, 384, |_, _| {})
            .expect("cymatic harp render");
        let (void_px, lit_px) = count_void_and_lit(&pixels, 35);
        assert!(void_px > 10_000, "Cymatic harp void must remain pure black, got {void_px}");
        assert!(lit_px > 1_500, "Cymatic harp must light strings & nodal rings, got {lit_px}");
    }

    // 5. 13 Quantum Field (u = -10.0) & 17 Mind Connectome (u = 2.15): hero volumes bounded cleanly in front of camera
    for (sector, u, exp_id, label) in [
        (SectorId::QUANTUM_FIELD, -10.0, 13u8, "Quantum u=-10"),
        (SectorId::MIND_CATHEDRAL, 2.15, 17u8, "Mind u=2.15"),
    ] {
        let pose = ScalePose64::new(sector, DVec3::new(0.0, 0.0, 3.2), u, DQuat::IDENTITY);
        let mut input = SceneBuildInput::desktop_default(pose, cam, 384, 384);
        input.active_experience_id = exp_id;
        input.particle_budget = 4096;
        let scene = builder.build(&index, &input);
        assert!(
            !scene.heroes_surface.iter().any(|h| h.kind_tags[0] == 0),
            "Earth cutaway must not leak into {label}"
        );
        for h in &scene.heroes_volume {
            assert!(
                h.pos_radius[3] <= 6.0,
                "{label} hero radius {} exceeds 6.0 cap",
                h.pos_radius[3]
            );
        }
        let pixels = renderer
            .render_to_offscreen_rgba8(&scene, 384, 384, |_, _| {})
            .expect(label);
        let (void_px, lit_px) = count_void_and_lit(&pixels, 35);
        assert!(void_px > 10_000, "{label} must keep OLED #000000 void, got {void_px}");
        assert!(lit_px > 1_000, "{label} hero volume must illuminate frame, got {lit_px}");
    }

    // 6. Kerr Black Hole Relativistic Lensing (Milky Way / Spacetime)
    {
        let pose = ScalePose64::new(SectorId::MILKY_WAY, DVec3::new(0.0, 0.2, 3.5), 18.0, DQuat::IDENTITY);
        let mut input = SceneBuildInput::desktop_default(pose, cam, 384, 384);
        input.active_experience_id = 5;
        input.particle_budget = 8192;
        let scene = builder.build(&index, &input);
        let mut overlay_called = false;
        let pixels = renderer
            .render_to_offscreen_rgba8(&scene, 384, 384, |_rpass, is_stereo| {
                overlay_called = true;
                assert!(!is_stereo);
            })
            .expect("kerr black hole render");
        assert!(overlay_called, "overlay closure must execute inside HDR pass");
        let (void_px, lit_px) = count_void_and_lit(&pixels, 40);
        assert!(void_px > 5_000, "Kerr void & event horizon must preserve true black, got {void_px}");
        assert!(lit_px > 1_500, "Kerr accretion disk + galaxy particles must illuminate scene, got {lit_px}");
        let _ = vp;
    }
}

#[test]
fn overlay_hook_and_stereo_multiview_render_both_eye_layers() {
    let mut renderer =
        CosmosRenderer::init_for_dx12_luid(None, 256, 256, true).expect("stereo DX12 init");
    assert!(renderer.shared_frame_bind_group_layout().size_check_not_needed());
    let index = build_default_universe_index();
    let mut builder = UniverseSceneBuilder::new();

    let proj = reversed_z_infinite_perspective_rh(90.0_f32.to_radians(), 1.0, 0.04);
    let view = camera_view_matrix(Quat::IDENTITY);
    let left_offset = Vec3::new(-0.032, 0.0, 0.0);
    let right_offset = Vec3::new(0.032, 0.0, 0.0);
    let vp_l = proj * glam::Mat4::from_translation(-left_offset) * view;
    let vp_r = proj * glam::Mat4::from_translation(-right_offset) * view;

    let stereo_cam = RenderCamera {
        eyes: [
            EyeView::from_view_proj(vp_l, left_offset),
            EyeView::from_view_proj(vp_r, right_offset),
        ],
        is_stereo: true,
        cam_forward: Vec3::NEG_Z,
        cam_right: Vec3::X,
        cam_up: Vec3::Y,
        fov_y_rad: 90.0_f32.to_radians(),
        aspect: 1.0,
        near_plane: 0.04,
    };

    let pose = ScalePose64::new(SectorId::SOLAR_SYSTEM, DVec3::new(0.0, 0.0, 3.0), 11.2, DQuat::IDENTITY);
    let mut input = SceneBuildInput::desktop_default(pose, stereo_cam, 256, 256);
    input.active_experience_id = 4;
    input.particle_budget = 2048;
    let scene = builder.build(&index, &input);

    // 2-layer stereo D2Array target mirroring an OpenXR D3D12 swapchain image
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let stereo_tex = renderer.device().create_texture(&wgpu::TextureDescriptor {
        label: Some("stereo_swapchain_test"),
        size: wgpu::Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 2,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let stereo_view = stereo_tex.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        base_array_layer: 0,
        array_layer_count: Some(2),
        ..Default::default()
    });

    let mut hook_stereo = false;
    renderer.render_frame(
        &scene,
        &stereo_view,
        format,
        256,
        256,
        true,
        |_rpass, is_stereo| {
            hook_stereo = is_stereo;
        },
    );
    assert!(hook_stereo, "overlay hook must receive is_stereo = true");

    let left_px = readback_layer(renderer.device(), renderer.queue(), &stereo_tex, 256, 256, 0);
    let right_px = readback_layer(renderer.device(), renderer.queue(), &stereo_tex, 256, 256, 1);
    let (_, left_lit) = count_void_and_lit(&left_px, 35);
    let (_, right_lit) = count_void_and_lit(&right_px, 35);
    assert!(left_lit > 1_000, "left eye rendered pixels expected, got {left_lit}");
    assert!(right_lit > 1_000, "right eye rendered pixels expected, got {right_lit}");
    assert_ne!(
        left_px, right_px,
        "left and right stereo multiview eyes must exhibit horizontal IPD parallax disparity"
    );
}

#[test]
fn phase4_oled_voids_cold_start_earth_and_pde_field_dispatch_hold() {
    let mut renderer = CosmosRenderer::init_desktop(384, 384).expect("RTX 5070 DX12 init");
    let index = build_default_universe_index();
    let mut builder = UniverseSceneBuilder::new();
    let cam = RenderCamera::mono(Quat::IDENTITY, 72.0_f32.to_radians(), 1.0, 0.02);
    let total_px = 384 * 384;

    // 1. Cold-start Earth (03, u=6.6, earth_cutaway_blend=0.0): intact globe, NO forced cutaway, >= 60% #000000 void
    {
        let pose = ScalePose64::new(SectorId::EARTH_ORBIT, DVec3::new(0.0, 0.0, 3.0), 6.6, DQuat::IDENTITY);
        let mut input = SceneBuildInput::desktop_default(pose, cam, 384, 384);
        input.active_experience_id = 3;
        input.earth_cutaway_blend = 0.0;
        input.particle_budget = 2048;
        let scene = builder.build(&index, &input);
        let earth = scene
            .heroes_surface
            .iter()
            .find(|h| h.kind_tags[0] == 0)
            .expect("Earth surface hero must emit at u=6.6");
        assert_eq!(
            earth.kind_tags[3] & 1,
            0,
            "Cold-start Earth at u=6.6 must be intact (cutaway bit unset unless earth_cutaway_blend > 0.25)"
        );
        let px = renderer.render_to_offscreen_rgba8(&scene, 384, 384, |_, _| {}).expect("earth cold start");
        let (void_px, lit_px) = count_void_and_lit(&px, 35);
        assert!(void_px * 100 >= total_px * 60, "Earth cold-start must have >=60% #000000 void, got {}/{}", void_px, total_px);
        assert!(lit_px > 2_000, "Intact Earth + Rayleigh/Aurora must illuminate frame, got {lit_px}");
    }

    // 2. Cosmic Web (08, u=25.35): CMB suppressed to 0.0, >= 60% #000000 void
    {
        let pose = ScalePose64::new(SectorId::COSMIC_WEB, DVec3::new(0.0, 0.0, 3.2), 25.35, DQuat::IDENTITY);
        let mut input = SceneBuildInput::desktop_default(pose, cam, 384, 384);
        input.active_experience_id = 8;
        input.particle_budget = 4096;
        let scene = builder.build(&index, &input);
        assert_eq!(scene.far.regime_weights[1], 0.0, "CMB alpha must be 0.0 at u=25.35 Cosmic Web");
        let px = renderer.render_to_offscreen_rgba8(&scene, 384, 384, |_, _| {}).expect("cosmic web");
        let (void_px, lit_px) = count_void_and_lit(&px, 28);
        assert!(void_px * 100 >= total_px * 60, "Cosmic Web must keep >=60% #000000 void, got {}/{}", void_px, total_px);
        assert!(lit_px > 400, "Cosmic Web filaments/nodes must illuminate frame, got {lit_px}");
    }

    // 3. CMB & Inflation Horizon Cutaway Arc (09/10, u=26.6): >= 50% #000000 void
    {
        let pose = ScalePose64::new(SectorId::OBSERVABLE_CMB, DVec3::new(0.0, 0.0, 3.2), 26.6, DQuat::IDENTITY);
        let mut input = SceneBuildInput::desktop_default(pose, cam, 384, 384);
        input.active_experience_id = 9;
        input.particle_budget = 2048;
        let scene = builder.build(&index, &input);
        let px = renderer.render_to_offscreen_rgba8(&scene, 384, 384, |_, _| {}).expect("cmb horizon");
        let (void_px, lit_px) = count_void_and_lit(&px, 30);
        assert!(void_px * 100 >= total_px * 50, "CMB horizon arc must preserve >=50% #000000 void, got {}/{}", void_px, total_px);
        assert!(lit_px > 1_500, "CMB horizon arc must render clearly, got {lit_px}");
    }

    // 4. Final Horizon Lotus (25, u=28.5): Inflation = 0.0, >= 70% #000000 quiet void
    {
        let pose = ScalePose64::new(SectorId::HYPER_LOTUS_25_HORIZON, DVec3::new(0.0, 0.0, 3.2), 28.5, DQuat::IDENTITY);
        let mut input = SceneBuildInput::desktop_default(pose, cam, 384, 384);
        input.active_experience_id = 25;
        input.particle_budget = 1024;
        let scene = builder.build(&index, &input);
        assert_eq!(scene.far.regime_weights[2], 0.0, "Inflation alpha must fade to 0.0 above u=27.4");
        let px = renderer.render_to_offscreen_rgba8(&scene, 384, 384, |_, _| {}).expect("final horizon");
        let (void_px, lit_px) = count_void_and_lit(&px, 28);
        assert!(void_px * 100 >= total_px * 70, "Final Horizon Lotus must keep >=70% #000000 void, got {}/{}", void_px, total_px);
        assert!(lit_px > 300, "Final Horizon concentric rings & lotus petals must glow, got {lit_px}");
    }

    // 5. Distinct cs_field_grid modes (Turing 0 vs Chladni 1) & Hero diversity (15, 02, 22, 04) + Wrist Locale bead
    {
        let pose15 = ScalePose64::new(SectorId::COMPLEXITY_HELIX, DVec3::new(0.0, 0.0, 2.8), -7.0, DQuat::IDENTITY);
        let mut in15 = SceneBuildInput::desktop_default(pose15, cam, 384, 384);
        in15.active_experience_id = 15;
        in15.particle_budget = 2048;
        in15.diegetic = DiegeticUiInput {
            show_focus_shimmer: true,
            focus_ray: Vec3::NEG_Z,
            dwell_progress: 0.5,
            wrist_pos: Some(Vec3::new(-0.22, -0.18, -0.45)),
            wrist_orient: Quat::IDENTITY,
            playback_rate: 1.5,
            locale_idx: 1,
        };
        let mut lit15 = 0;
        for step in 0..3 {
            in15.time_sec = 0.2 * step as f32;
            let scene15 = builder.build(&index, &in15);
            assert_eq!(scene15.dispatch_field_grid, Some(0));
            assert_ne!(scene15.compute.counts[0], 0, "particle compute_mode must stay intact while dispatch_field_grid=Some(0)");
            let px = renderer.render_to_offscreen_rgba8(&scene15, 384, 384, |_, _| {}).expect("turing render");
            let (void_px, lit_px) = count_void_and_lit(&px, 28);
            assert!(void_px * 100 >= total_px * 55, "Complexity Turing must preserve >=55% void, got {void_px}");
            lit15 = lit_px;
        }
        assert!(lit15 > 800, "Turing field grid shell must illuminate viewport, got {lit15}");

        // 22 Cosmic Contact SETI cones & rings + 02 Chladni dispatch
        let pose22 = ScalePose64::new(SectorId::ORION_SPUR, DVec3::new(0.0, 0.0, 3.0), 17.8, DQuat::IDENTITY);
        let mut in22 = SceneBuildInput::desktop_default(pose22, cam, 384, 384);
        in22.active_experience_id = 22;
        in22.particle_budget = 2048;
        let scene22 = builder.build(&index, &in22);
        let px22 = renderer.render_to_offscreen_rgba8(&scene22, 384, 384, |_, _| {}).expect("cosmic contact render");
        let (void22, lit22) = count_void_and_lit(&px22, 30);
        assert!(void22 * 100 >= total_px * 60, "Cosmic Contact SETI scene must preserve >=60% void, got {void22}");
        assert!(lit22 > 500, "Cosmic Contact SETI laser & prime rings must illuminate, got {lit22}");
    }
}

#[test]
fn phase7_documentary_stages_01_through_25_oled_and_telemetry_hold() {
    use cosmos_render::DocumentaryFocusState;

    let mut renderer = CosmosRenderer::init_desktop(384, 384).expect("RTX 5070 DX12 init");
    let index = build_default_universe_index();
    let mut builder = UniverseSceneBuilder::new();
    let cam = RenderCamera::mono(Quat::IDENTITY, 72.0_f32.to_radians(), 1.0, 0.02);
    let total_px = 384 * 384;

    struct DocStageCase {
        exp_id: u8,
        sector: SectorId,
        scale_u: f64,
        scene_idx: i32,
        zone_idx: u16,
        feat_idx: u16,
        telemetry: [f32; 4],
        expected_surface: usize,
    }
    let cases = [
        DocStageCase { exp_id: 1, sector: SectorId::COLORS_PRISM, scale_u: 2.0, scene_idx: 2, zone_idx: 3, feat_idx: 1, telemetry: [215.0, 0.92, 0.56, 2.0], expected_surface: 3 },
        DocStageCase { exp_id: 2, sector: SectorId::MUSIC_HARP, scale_u: 2.0, scene_idx: 1, zone_idx: 2, feat_idx: 4, telemetry: [69.0, 2.0, 440.0, 0.85], expected_surface: 0 },
        DocStageCase { exp_id: 13, sector: SectorId::QUANTUM_FIELD, scale_u: -10.0, scene_idx: 2, zone_idx: 4, feat_idx: 0, telemetry: [0.0; 4], expected_surface: 0 },
        DocStageCase { exp_id: 14, sector: SectorId::PLANCK_ORIGIN, scale_u: -18.0, scene_idx: 1, zone_idx: 2, feat_idx: 1, telemetry: [0.0; 4], expected_surface: 0 },
        DocStageCase { exp_id: 17, sector: SectorId::MIND_CATHEDRAL, scale_u: 2.15, scene_idx: 1, zone_idx: 3, feat_idx: 2, telemetry: [0.0; 4], expected_surface: 0 },
        DocStageCase { exp_id: 22, sector: SectorId::ORION_SPUR, scale_u: 17.8, scene_idx: 0, zone_idx: 1, feat_idx: 0, telemetry: [0.0; 4], expected_surface: 0 },
        DocStageCase { exp_id: 23, sector: SectorId::HYPER_LOTUS_23_PERSPECTIVES, scale_u: 27.6, scene_idx: 2, zone_idx: 2, feat_idx: 3, telemetry: [0.0; 4], expected_surface: 0 },
        DocStageCase { exp_id: 25, sector: SectorId::HYPER_LOTUS_25_HORIZON, scale_u: 28.5, scene_idx: 0, zone_idx: 24, feat_idx: 5, telemetry: [0.0; 4], expected_surface: 0 },
    ];

    for c in cases {
        let DocStageCase {
            exp_id,
            sector,
            scale_u,
            scene_idx,
            zone_idx,
            feat_idx,
            telemetry,
            expected_surface,
        } = c;
        let pose = ScalePose64::new(sector, DVec3::new(0.0, 0.0, 3.0), scale_u, DQuat::IDENTITY);
        let mut input = SceneBuildInput::desktop_default(pose, cam, 384, 384);
        input.active_experience_id = exp_id;
        input.particle_budget = 2048;
        input.documentary = DocumentaryFocusState {
            active_zone_index: zone_idx,
            active_feature_index: feat_idx,
            scene_index: scene_idx,
            narration_cue_progress: 0.68,
            narration_active_weight: 0.90,
            zone_accent_rgb: [0.35, 1.15, 1.45],
            telemetry,
            ..DocumentaryFocusState::default()
        };
        let scene = builder.build(&index, &input);
        assert!(
            scene.heroes_surface.len() >= expected_surface,
            "exp {exp_id}: expected >= {expected_surface} surface heroes, got {}",
            scene.heroes_surface.len()
        );
        assert!(
            !scene.heroes_volume.is_empty(),
            "exp {exp_id}: must emit documentary volumetric hero"
        );
        let h0 = &scene.heroes_volume[0];
        assert_eq!(h0.kind_tags[2] & 0xFFFF, zone_idx as u32, "exp {exp_id} packed zone_idx");
        assert_eq!((h0.kind_tags[2] >> 16) & 0xFF, feat_idx as u32, "exp {exp_id} packed feat_idx");
        assert_eq!((h0.kind_tags[2] >> 24) & 0xFF, (scene_idx + 1) as u32, "exp {exp_id} packed scene_idx");

        let px = renderer
            .render_to_offscreen_rgba8(&scene, 384, 384, |_, _| {})
            .expect("phase7 documentary render");
        let (void_px, lit_px) = count_void_and_lit(&px, 28);
        let blown_px = px
            .as_chunks::<4>()
            .0
            .iter()
            .filter(|c| c[0] >= 250 && c[1] >= 250 && c[2] >= 250)
            .count();
        assert!(
            void_px * 100 >= total_px * 58,
            "exp {exp_id} must preserve >=58% #000000 OLED void, got {void_px}/{total_px}"
        );
        assert!(
            lit_px > 250,
            "exp {exp_id} must illuminate documentary stage, got {lit_px}"
        );
        assert!(
            blown_px * 1000 < total_px * 5,
            "exp {exp_id} must avoid clipped white burnout, got {blown_px}"
        );
    }
}

#[test]
fn celestial_body_isolation_and_solar_mapping_are_exact() {
    use cosmos_math::CameraRelativePose;
    use cosmos_spatial::{EntityKind, LodTier, VisibleEntity};

    let index = build_default_universe_index();
    let mut builder = UniverseSceneBuilder::new();
    let cam = RenderCamera::mono(Quat::IDENTITY, 72.0_f32.to_radians(), 1.0, 0.02);

    // 1. Cross-experience isolation in full scene builds (03, 18, 19 must never inherit Solar System bodies)
    for (exp_id, u_cam) in [(18u8, 6.8_f64), (19u8, 8.0_f64), (3u8, 5.5_f64)] {
        let pose = ScalePose64::new(SectorId::EARTH_ORBIT, DVec3::new(0.0, 0.0, 3.8), u_cam, DQuat::IDENTITY);
        let mut input = SceneBuildInput::desktop_default(pose, cam, 384, 384);
        input.active_experience_id = exp_id;
        let scene = builder.build(&index, &input);
        let earth_surface_count = scene
            .heroes_surface
            .iter()
            .filter(|h| h.kind_tags[0] == 0 && h.kind_tags[1] == 0)
            .count();
        let solar_planet_count = scene
            .heroes_surface
            .iter()
            .filter(|h| h.kind_tags[0] == 1 && (1..=11).contains(&h.kind_tags[1]))
            .count();

        if exp_id == 3 {
            assert_eq!(
                earth_surface_count, 1,
                "Deep Earth (03) must have exactly 1 cutaway Earth surface hero, got {earth_surface_count}"
            );
            let earth = scene
                .heroes_surface
                .iter()
                .find(|h| h.kind_tags[0] == 0 && h.kind_tags[1] == 0)
                .unwrap();
            assert_eq!(earth.kind_tags[3] & 0xFF, 1, "Deep Earth (03) must keep cutaway active");
        } else {
            assert_eq!(
                earth_surface_count, 0,
                "exp {exp_id} must not be swallowed by Solar Earth surface hero"
            );
        }
        assert_eq!(
            solar_planet_count, 0,
            "exp {exp_id} must not receive Solar planet surface heroes"
        );
    }

    // 2. Direct emit_celestial_or_landmark isolation across isolated chapters
    let mk_vis = |kind: EntityKind, exp: u8, zone: u16| VisibleEntity {
        id: 42 + zone as u64,
        experience_id: exp,
        zone_index: zone,
        feature_index: 0,
        kind,
        pose: CameraRelativePose {
            pos: Vec3::new(0.0, 0.0, -3.5),
            rel_scale: 1.2,
            orient: Quat::IDENTITY,
            angular_rad: 0.6,
            distance_render_units: 3.5,
            is_far_shell: false,
        },
        lod: LodTier::Hero,
        salience_score: 1.0,
    };

    for isolated_exp in [1u8, 2, 3, 18, 19, 22] {
        let mut input = SceneBuildInput::desktop_default(
            ScalePose64::new(SectorId::EARTH_ORBIT, DVec3::ZERO, 6.8, DQuat::IDENTITY),
            cam,
            384,
            384,
        );
        input.active_experience_id = isolated_exp;
        let mut hs = Vec::new();
        let mut hv = Vec::new();
        let mut sp = Vec::new();
        cosmos_render::doc_stages::emit_celestial_or_landmark(
            &mk_vis(EntityKind::CelestialBody, 4, 3),
            &input,
            Vec3::Y,
            0.0,
            &mut hs,
            &mut hv,
            &mut sp,
        );
        cosmos_render::doc_stages::emit_celestial_or_landmark(
            &mk_vis(EntityKind::GalacticLandmark, 5, 0),
            &input,
            Vec3::Y,
            0.0,
            &mut hs,
            &mut hv,
            &mut sp,
        );
        assert!(
            hs.is_empty() && hv.is_empty() && sp.is_empty(),
            "exp {isolated_exp} must reject leaked CelestialBody & GalacticLandmark geometry"
        );
    }

    // Also ensure GalacticLandmark(5) does not leak into Solar System (4)
    {
        let mut input = SceneBuildInput::desktop_default(
            ScalePose64::new(SectorId::SOLAR_SYSTEM, DVec3::ZERO, 11.2, DQuat::IDENTITY),
            cam,
            384,
            384,
        );
        input.active_experience_id = 4;
        let mut hs = Vec::new();
        let mut hv = Vec::new();
        let mut sp = Vec::new();
        cosmos_render::doc_stages::emit_celestial_or_landmark(
            &mk_vis(EntityKind::GalacticLandmark, 5, 0),
            &input,
            Vec3::Y,
            0.0,
            &mut hs,
            &mut hv,
            &mut sp,
        );
        assert!(hs.is_empty() && hv.is_empty() && sp.is_empty());
    }

    // 3. Exact 1:1 Solar System body_idx (0..=11) variant + ring/belt mapping
    let mut solar_input = SceneBuildInput::desktop_default(
        ScalePose64::new(SectorId::SOLAR_SYSTEM, DVec3::ZERO, 11.2, DQuat::IDENTITY),
        cam,
        384,
        384,
    );
    solar_input.active_experience_id = 4;

    let expected = [
        (0u16, 1u32, 1u32, Some((3u32, 4u32)), 0usize), // Sun + Corona
        (1, 1, 3, None, 0),                             // Mercury
        (2, 1, 4, None, 0),                             // Venus
        (3, 0, 0, None, 0),                             // Earth
        (4, 1, 5, None, 0),                             // Mars
        (5, 1, 3, Some((2, 1)), 7),                     // Asteroid Belt / Ceres + belt ring + 7 shards (NOT Jupiter!)
        (6, 1, 6, None, 0),                             // Jupiter
        (7, 1, 2, Some((2, 2)), 0),                     // Saturn + Cassini/Encke ring
        (8, 1, 7, Some((2, 1)), 0),                     // Uranus + delicate ring
        (9, 1, 8, None, 0),                             // Neptune
        (10, 1, 10, Some((2, 1)), 7),                   // Kuiper Belt icy world + icy disk ring + 7 shards
        (11, 1, 9, None, 0),                            // Pluto + Sputnik Planitia heart
    ];
    for (zone, exp_kind, exp_vid, exp_vol, exp_sprites) in expected {
        let mut hs = Vec::new();
        let mut hv = Vec::new();
        let mut sp = Vec::new();
        cosmos_render::doc_stages::emit_celestial_or_landmark(
            &mk_vis(EntityKind::CelestialBody, 4, zone),
            &solar_input,
            Vec3::Y,
            0.0,
            &mut hs,
            &mut hv,
            &mut sp,
        );
        assert_eq!(hs.len(), 1, "solar zone {zone} must emit 1 surface hero");
        assert_eq!(
            (hs[0].kind_tags[0], hs[0].kind_tags[1]),
            (exp_kind, exp_vid),
            "solar zone {zone} surface kind/vid mismatch"
        );
        match exp_vol {
            Some((vk, vv)) => {
                assert_eq!(hv.len(), 1, "solar zone {zone} must emit 1 volume companion");
                assert_eq!((hv[0].kind_tags[0], hv[0].kind_tags[1]), (vk, vv));
            }
            None => assert!(hv.is_empty(), "solar zone {zone} should have no volume"),
        }
        assert_eq!(sp.len(), exp_sprites, "solar zone {zone} sprite shard count mismatch");
    }
}

#[test]
fn phase7b_zone_routing_framing_and_sprite_clamps_hold() {
    use cosmos_math::{ScalePose64, SectorId};
    use cosmos_render::doc_stages::{doc_stage_variant_id, emit_doc_stage_hero};
    use cosmos_render::universe_scene::DocumentaryFocusState;
    use cosmos_spatial::topology::build_default_universe_index;
    use glam::{DQuat, DVec3, Quat, Vec3};

    let mut doc = DocumentaryFocusState::default();
    let cases = [
        (13u8, 2u16, 10u32),
        (13, 3, 11),
        (13, 6, 12),
        (14, 8, 22),
        (14, 11, 23),
        (15, 0, 10),
        (16, 0, 23),
        (16, 4, 20),
        (19, 0, 190),
        (19, 16, 194),
        (20, 0, 200),
        (20, 16, 202),
        (21, 0, 210),
        (21, 12, 212),
        (22, 0, 220),
        (22, 12, 221),
        (24, 0, 240),
        (24, 24, 241),
    ];
    for (exp, zone, expected_vid) in cases {
        doc.active_zone_index = zone;
        doc.scene_index = -1;
        assert_eq!(
            doc_stage_variant_id(exp, &doc),
            expected_vid,
            "exp {exp} zone {zone} variant id mismatch"
        );
    }

    // Even if exp 19 zone 16 carries scene_index 7, Silicon Die + Transformer Web (194) wins.
    doc.active_zone_index = 16;
    doc.scene_index = 7;
    assert_eq!(doc_stage_variant_id(19, &doc), 194);

    let cam = RenderCamera::mono(Quat::IDENTITY, 72.0_f32.to_radians(), 1.0, 0.02);
    let mut input = SceneBuildInput::desktop_default(
        ScalePose64::new(SectorId::HYPER_LOTUS_24_MEANING, DVec3::ZERO, 27.6, DQuat::IDENTITY),
        cam,
        384,
        384,
    );

    // Side-approached stage at (-2.95, -2.65, +0.35) must rotate +Z_local horizontally toward camera (+2.95, 0, -0.35)
    let pos_side = Vec3::new(-2.95, -2.65, 0.35);
    let mut hs = Vec::new();
    let mut hv = Vec::new();
    let mut sp = Vec::new();
    input.active_experience_id = 24;
    input.documentary.active_zone_index = 24;
    emit_doc_stage_hero(
        24,
        pos_side,
        1.5,
        [0.0, 0.0, 0.0, 1.0],
        Vec3::Y,
        0.5,
        &input,
        &mut hs,
        &mut hv,
        &mut sp,
    );
    assert_eq!(hv.len(), 1);
    assert_eq!(hv[0].kind_tags[1], 241);
    let q = Quat::from_array(hv[0].rot_quat);
    let local_z_world = q * Vec3::Z;
    let to_cam_xz = Vec3::new(-pos_side.x, 0.0, -pos_side.z).normalize();
    assert!(
        local_z_world.dot(to_cam_xz) > 0.99,
        "viewer-facing stage must face camera horizontally, got dot={}",
        local_z_world.dot(to_cam_xz)
    );

    // Zero scanline sprites in 11 & 16; zero SETI wedge in 22; zero Europa flankers in exp 21 zone 12; zero duplicate Saturn in exp 4
    for exp in [4u8, 11, 16, 21, 22] {
        hs.clear();
        hv.clear();
        sp.clear();
        input.active_experience_id = exp;
        input.documentary.active_zone_index = if exp == 21 { 12 } else { 0 };
        emit_doc_stage_hero(
            exp,
            Vec3::new(0.0, 0.0, -3.2),
            1.6,
            [0.0, 0.0, 0.0, 1.0],
            Vec3::Y,
            0.5,
            &input,
            &mut hs,
            &mut hv,
            &mut sp,
        );
        if exp == 4 {
            assert!(
                hs.is_empty(),
                "exp 4 anchor must not emit duplicate Sun surface when active_experience_id == 4"
            );
            assert!(
                hv.len() == 1 && hv[0].kind_tags[0] == 2 && hv[0].kind_tags[1] == 4,
                "exp 4 anchor must emit only Keplerian hairline orbit disk (kind=2, vid=4)"
            );
        } else if exp == 21 {
            assert!(hs.is_empty(), "exp 21 zone 12 Europa close-up must emit no flanking spheres");
        }
        assert!(
            sp.is_empty(),
            "exp {exp} must not emit scanline/hex/wedge sprites, found {}",
            sp.len()
        );
    }

    // Planck Spin-Foam u = -35 must frame centered hero with vid=23 despite anchor at u = -30
    let idx = build_default_universe_index();
    let mut builder = UniverseSceneBuilder::new();
    let look_q = Quat::from_rotation_arc(Vec3::NEG_Z, Vec3::new(-0.55, -0.48, -2.75).normalize());
    let cam_planck = RenderCamera::mono(look_q, 72.0_f32.to_radians(), 1.0, 0.02);
    let mut planck_in = SceneBuildInput::desktop_default(
        ScalePose64::new(SectorId::PLANCK_ORIGIN, DVec3::new(0.55, 0.48, 2.75), -35.0, DQuat::IDENTITY),
        cam_planck,
        384,
        384,
    );
    planck_in.active_experience_id = 14;
    planck_in.documentary.active_zone_index = 11;
    let planck_scene = builder.build(&idx, &planck_in);
    assert!(
        planck_scene.heroes_volume.iter().any(|h| h.kind_tags[1] == 23),
        "08 Planck u=-35 must emit centered Spin-Foam hero vid=23"
    );
    assert!(
        planck_scene.particle_batch.batch_params[1] < 1.0
            && planck_scene.particle_batch.batch_params[2] < 0.6,
        "particle fog batch must be toned down for crisp legibility"
    );

    // Verify all non-diegetic sprites stay angular-clamped across Chroma/Music/Cosmos scenes
    for (sid, u, exp) in [
        (SectorId::COLORS_PRISM, 1.95, 1u8),
        (SectorId::MUSIC_HARP, 1.92, 2u8),
        (SectorId::LANIAKEA, 23.8, 7u8),
    ] {
        let mut s_in = SceneBuildInput::desktop_default(
            ScalePose64::new(sid, DVec3::new(0.28, 0.18, 0.92), u, DQuat::IDENTITY),
            cam,
            384,
            384,
        );
        s_in.active_experience_id = exp;
        let built = builder.build(&idx, &s_in);
        for spr in &built.sprites {
            if (spr.tags_f[0] - 4.0).abs() > 0.2 {
                assert!(
                    spr.pos_size[3] <= 0.065,
                    "exp {exp} emitted oversized non-UI sprite radius {}",
                    spr.pos_size[3]
                );
            }
        }
    }

    // Exp 4 Solar System Saturn framing (u=10.72) & Panorama (u=11.45):
    // True Saturn (kind=1, vid=2 + kind=2, vid=2) + all planets visible across LODs without enclosing anchor Sun.
    let mut solar_idx = build_default_universe_index();
    for zi in 0u16..=11u16 {
        let (local_pos, body_u, radius_m) =
            cosmos_spatial::solar_body_local_pose(zi, (zi as f32) / 11.0, 0.05, 35.0);
        let _ = solar_idx.insert_entity(cosmos_spatial::SpatialEntity {
            id: 90_000 + zi as u64,
            experience_id: 4,
            zone_index: zi,
            feature_index: 0,
            sector_id: SectorId::SOLAR_SYSTEM,
            local_pos,
            orient: DQuat::IDENTITY,
            radius_m,
            scale_exp: body_u,
            kind: cosmos_spatial::EntityKind::CelestialBody,
            salience_weight: 1.65,
        });
    }
    let saturn_ent = solar_idx
        .find_zone_entity(4, 7)
        .expect("Solar System zone 7 (Saturn) must exist");
    let saturn_eye = saturn_ent.local_pos + DVec3::new(-0.88, 0.36, 0.58);
    let saturn_target = saturn_ent.local_pos + DVec3::new(0.28, -0.03, -0.18);
    let saturn_dir = (saturn_target - saturn_eye).normalize().as_vec3();
    let saturn_q = Quat::from_rotation_arc(Vec3::NEG_Z, saturn_dir);
    let cam_saturn = RenderCamera::mono(saturn_q, 72.0_f32.to_radians(), 1.0, 0.02);
    let mut saturn_in = SceneBuildInput::desktop_default(
        ScalePose64::new(SectorId::SOLAR_SYSTEM, saturn_eye, 10.72, DQuat::IDENTITY),
        cam_saturn,
        384,
        384,
    );
    saturn_in.active_experience_id = 4;
    saturn_in.documentary.active_zone_index = 7;
    let saturn_scene = builder.build(&solar_idx, &saturn_in);
    let saturn_body = saturn_scene
        .heroes_surface
        .iter()
        .find(|h| h.kind_tags[0] == 1 && h.kind_tags[1] == 2)
        .expect("Exp 4 Saturn shot must emit Saturn surface hero (kind=1, vid=2)");
    assert!(
        saturn_scene
            .heroes_volume
            .iter()
            .any(|h| h.kind_tags[0] == 2 && h.kind_tags[1] == 2),
        "Exp 4 Saturn shot must emit Saturn Cassini/Encke ring (kind=2, vid=2)"
    );
    let saturn_dist = Vec3::from_slice(&saturn_body.pos_radius[..3]).length();
    assert!(
        saturn_body.pos_radius[3] > 0.08 && saturn_body.pos_radius[3] < saturn_dist * 0.5,
        "Saturn radius {} must frame cleanly at dist {}",
        saturn_body.pos_radius[3],
        saturn_dist
    );
    let sun_heroes: Vec<_> = saturn_scene
        .heroes_surface
        .iter()
        .filter(|h| h.kind_tags[0] == 1 && h.kind_tags[1] == 1)
        .collect();
    assert_eq!(sun_heroes.len(), 1, "Exp 4 must emit exactly one true Sun (no duplicate anchor Sun)");
    assert!(
        sun_heroes[0].pos_radius[3] < saturn_dist * 0.5,
        "true Sun radius {} must not swallow Saturn at dist {}",
        sun_heroes[0].pos_radius[3],
        saturn_dist
    );
    let kepler = saturn_scene
        .heroes_volume
        .iter()
        .find(|h| h.kind_tags[0] == 2 && h.kind_tags[1] == 4)
        .expect("Exp 4 must emit hairline Keplerian orbit plane (kind=2, vid=4)");
    assert!(kepler.phys_a[2] >= 1.0 - 1e-4, "track_scale must be encoded in phys_a[2]");

    // Panorama u=11.45: Sun, Saturn, and outer planets remain visible across all LOD tiers
    let pano_eye = DVec3::new(-4.6, 8.8, 16.2);
    let pano_dir = (DVec3::new(0.0, -0.2, 0.0) - pano_eye).normalize().as_vec3();
    let pano_q = Quat::from_rotation_arc(Vec3::NEG_Z, pano_dir);
    let cam_pano = RenderCamera::mono(pano_q, 72.0_f32.to_radians(), 1.0, 0.02);
    let mut pano_in = SceneBuildInput::desktop_default(
        ScalePose64::new(SectorId::SOLAR_SYSTEM, pano_eye, 11.45, DQuat::IDENTITY),
        cam_pano,
        384,
        384,
    );
    pano_in.active_experience_id = 4;
    let pano_scene = builder.build(&solar_idx, &pano_in);
    assert!(
        pano_scene.heroes_surface.len() >= 10,
        "Exp 4 panorama must render >= 10 analytical Solar bodies across LODs, got {}",
        pano_scene.heroes_surface.len()
    );
    assert!(
        pano_scene
            .heroes_volume
            .iter()
            .any(|h| h.kind_tags[0] == 2 && h.kind_tags[1] == 2),
        "Exp 4 panorama must emit Saturn's analytical ring (kind=2, vid=2)"
    );
}

#[test]
fn p0_canonical_id_registry_cue_plumbing_and_exp03_z05_08_tier_a_hold() {
    use std::collections::HashSet;
    use cosmos_render::DocumentaryFocusState;
    use cosmos_render::doc_stages::{
        canonical_companion_vid, canonical_zone_vid, is_zone_migrated_tier_a, pack_doc_tags,
        resolve_zone_shader_vid,
    };
    use cosmos_spatial::all_experience_placements;

    // 1. Canonical 1100..=3599 & companion 3600..=3869 IDs must be strictly collision-free across all 25 experiences / 376 zones.
    let placements = all_experience_placements();
    assert_eq!(placements.len(), 25);
    let mut zone_vids = HashSet::new();
    let mut doc_zones = 0usize;
    let mut total_placements = 0usize;
    for exp in &placements {
        for z in 0..exp.zone_count {
            let vid = canonical_zone_vid(exp.id.0, z);
            assert!(
                (1100..=3599).contains(&vid),
                "exp {} zone {} canonical vid {vid} out of [1100..=3599]",
                exp.id.0,
                z
            );
            assert!(
                zone_vids.insert(vid),
                "duplicate canonical zone vid {vid} at exp {} zone {z}",
                exp.id.0
            );
            total_placements += 1;
            if exp.id.0 >= 3 {
                doc_zones += 1;
            }
        }
        assert_ne!(
            exp.title, "Sentido Cósmico",
            "Exp 24 title must use Significado Cósmico"
        );
    }
    assert_eq!(doc_zones, 376, "documentary catalog zones across Exps 03..=25 must equal 376");
    assert_eq!(total_placements, 378, "all_experience_placements including 01/02 spatial labs must equal 378");

    let mut comp_vids = HashSet::new();
    for exp_id in 1u8..=25u8 {
        for slot in 0u8..10u8 {
            let cvid = canonical_companion_vid(exp_id, slot);
            assert!(
                (3600..=3869).contains(&cvid),
                "companion vid {cvid} out of [3600..=3869]"
            );
            assert!(comp_vids.insert(cvid), "duplicate companion vid {cvid}");
        }
    }

    // 2. pack_doc_tags preserves structural base_flags[0..2], packs cue_tags_mask[2..8],
    //    applies submode_override when submode==0, and falls back when scene_index == -1.
    let doc_pack = DocumentaryFocusState {
        active_zone_index: 5,
        active_feature_index: 3,
        scene_index: -1,
        narration_cue_progress: 0.75,
        narration_active_weight: 0.90,
        zone_accent_rgb: [0.4, 0.9, 1.3],
        telemetry: [0.0; 4],
        cue_tags_mask: 0b01_0101,
        submode_override: 7,
        emphasis_boost: 0.85,
    };
    let tags = pack_doc_tags(0, canonical_zone_vid(3, 5), 1, 0, &doc_pack);
    assert_eq!(tags[1], 1305);
    assert_eq!((tags[2] >> 24) & 0xFF, 6, "null scene_index must fall back cleanly to zone + 1");
    assert_eq!(tags[3] & 1, 1, "cutaway bit 0 must be preserved");
    assert_eq!((tags[3] >> 2) & 0x3F, 0b01_0101, "6-bit cue_tags_mask must pack into bits 2..7");
    assert_eq!((tags[3] >> 24) & 0xFF, 7, "submode_override must populate bits 24..31 when submode==0");

    // 3. Exp 03 Z05..=Z08 [TIER-A] routing + Z06 Surface premature cutaway bug fix + GPU feature differentiation
    for z in 5u16..=8u16 {
        assert!(is_zone_migrated_tier_a(3, z));
        let d = DocumentaryFocusState {
            active_zone_index: z,
            ..Default::default()
        };
        assert_eq!(resolve_zone_shader_vid(3, &d), 1300 + z as u32);
    }

    let mut renderer = CosmosRenderer::init_desktop(384, 384).expect("RTX 5070 DX12 init");
    let index = build_default_universe_index();
    let mut builder = UniverseSceneBuilder::new();
    let cam = RenderCamera::mono(Quat::IDENTITY, 72.0_f32.to_radians(), 1.0, 0.02);
    let total_px = 384 * 384;

    let mut render_exp03 = |zone: u16, feat: u16, scale_u: f64, cam_pos: DVec3| -> (Vec<u8>, u32, u32) {
        let mut input = SceneBuildInput::desktop_default(
            ScalePose64::new(SectorId::EARTH_ORBIT, cam_pos, scale_u, DQuat::IDENTITY),
            cam,
            384,
            384,
        );
        input.active_experience_id = 3;
        input.earth_cutaway_blend = if zone >= 7 { 1.0 } else { 0.35 };
        input.documentary = DocumentaryFocusState {
            active_zone_index: zone,
            active_feature_index: feat,
            scene_index: -1,
            narration_cue_progress: 0.68,
            narration_active_weight: 0.92,
            zone_accent_rgb: [0.35, 1.05, 1.45],
            telemetry: [0.0; 4],
            cue_tags_mask: 0x05,
            submode_override: 0,
            emphasis_boost: 1.0,
        };
        let scene = builder.build(&index, &input);
        let earth = scene
            .heroes_surface
            .iter()
            .find(|h| h.kind_tags[0] == 0)
            .expect("Exp 03 must emit Earth surface hero");
        let vid = earth.kind_tags[1];
        let cutaway_bit = earth.kind_tags[3] & 1;

        let px = renderer
            .render_to_offscreen_rgba8(&scene, 384, 384, |_, _| {})
            .expect("Exp 03 offscreen render");
        let (void_px, lit_px) = count_void_and_lit(&px, 28);
        let blown_px = px
            .as_chunks::<4>()
            .0
            .iter()
            .filter(|c| c[0] >= 250 && c[1] >= 250 && c[2] >= 250)
            .count();
        assert!(
            void_px * 100 >= total_px * 58,
            "Exp 03 Z{zone:02} F{feat} must preserve >=58% #000000 OLED void, got {void_px}/{total_px}"
        );
        assert!(lit_px > 400, "Exp 03 Z{zone:02} F{feat} must illuminate hero, got {lit_px}");
        assert!(
            blown_px * 1000 < total_px * 5,
            "Exp 03 Z{zone:02} F{feat} must avoid highlight blowout, got {blown_px}"
        );
        (px, vid, cutaway_bit)
    };

    let zones = [
        (5u16, 1305u32, 0u32, 6.50f64, DVec3::new(0.0, 0.25, 3.30)),
        (6u16, 1306u32, 0u32, 6.05f64, DVec3::new(0.0, 0.25, 3.15)),
        (7u16, 1307u32, 1u32, 5.90f64, DVec3::new(0.95, 0.45, 2.48)),
        (8u16, 1308u32, 1u32, 5.75f64, DVec3::new(0.95, 0.45, 2.48)),
    ];
    for &(zone, expected_vid, expected_cutaway, scale_u, cam_pos) in &zones {
        let mut renders = Vec::with_capacity(5);
        for feat in 0u16..5u16 {
            let (px, vid, cut) = render_exp03(zone, feat, scale_u, cam_pos);
            assert_eq!(
                (vid, cut),
                (expected_vid, expected_cutaway),
                "Exp 03 Z{zone:02} F{feat} must route to canonical vid={expected_vid} with cutaway={expected_cutaway}"
            );
            renders.push(px);
        }
        for i in 0..5 {
            for j in (i + 1)..5 {
                assert_ne!(
                    renders[i], renders[j],
                    "Exp 03 Z{zone:02} features F{i} and F{j} must render distinctly"
                );
            }
        }
    }
}

trait BindGroupLayoutTestExt {
    fn size_check_not_needed(&self) -> bool {
        true
    }
}
impl BindGroupLayoutTestExt for wgpu::BindGroupLayout {}

fn readback_layer(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    tex: &wgpu::Texture,
    width: u32,
    height: u32,
    layer: u32,
) -> Vec<u8> {
    let unpadded = width * 4;
    let padded = unpadded.div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT)
        * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let buf = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("layer_readback"),
        size: (padded * height) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut enc = device.create_command_encoder(&Default::default());
    enc.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: tex,
            mip_level: 0,
            origin: wgpu::Origin3d {
                x: 0,
                y: 0,
                z: layer,
            },
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buf,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(Some(enc.finish()));
    let slice = buf.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| {
        let _ = tx.send(r);
    });
    device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
    rx.recv().unwrap().unwrap();
    let mapped = slice.get_mapped_range().unwrap();
    let mut out = Vec::with_capacity((unpadded * height) as usize);
    for row in 0..height as usize {
        let s = row * padded as usize;
        out.extend_from_slice(&mapped[s..s + unpadded as usize]);
    }
    drop(mapped);
    buf.unmap();
    out
}

fn count_void_and_lit(rgba: &[u8], lit_thresh: u8) -> (usize, usize) {
    let mut void_px = 0usize;
    let mut lit_px = 0usize;
    for px in rgba.as_chunks::<4>().0 {
        if px[0] == 0 && px[1] == 0 && px[2] == 0 {
            void_px += 1;
        } else if px[0].max(px[1]).max(px[2]) >= lit_thresh {
            lit_px += 1;
        }
    }
    (void_px, lit_px)
}
