use std::path::PathBuf;

use cosmos_app::assets::UniverseAssets;
use cosmos_app::engine::UniverseRuntime;
use cosmos_app::headless::run_headless_render;
use cosmos_audio::AudioOutputMode;
use cosmos_manifest::Locale;
use cosmos_math::DEFAULT_DESKTOP_NEAR_RENDER_UNITS;
use cosmos_narration::FsmStateKind;
use cosmos_nav::{COLD_START_SCALE_EXP, DesktopFlightInput};
use cosmos_render::RenderCamera;
use glam::{DVec3, Vec3};

#[test]
fn spatial_guide_renders_welcome_help_and_close_up_tour_controls() {
    use cosmos_app::guide::GuideAction;
    use cosmos_render::{CosmosRenderer, DEPTH_FORMAT, HDR_SCENE_FORMAT};
    use cosmos_typo::{MsdfPipelineConfig, MsdfTextRenderer, TextPassUniforms};

    let assets = UniverseAssets::load().expect("assets");
    let shaders = assets.shaders_dir.clone();
    let mut rt = UniverseRuntime::new(assets, AudioOutputMode::Headless).expect("runtime");
    rt.enable_spatial_guide(false);
    let mut renderer = CosmosRenderer::init_desktop(1280, 720).expect("DX12 renderer");
    let mut text = MsdfTextRenderer::from_shaders_dir(
        renderer.device(),
        renderer.queue(),
        &rt.atlas,
        shaders,
        renderer.shared_frame_bind_group_layout(),
        MsdfPipelineConfig {
            color_format: HDR_SCENE_FORMAT,
            depth_format: Some(DEPTH_FORMAT),
            enable_stereo_multiview: true,
            sample_count: 1,
        },
    )
    .expect("spatial text renderer");
    let export = std::env::var_os("HYPERCOSMOS_UX_CAPTURE_DIR").map(PathBuf::from);
    let output = export.clone().unwrap_or_else(|| {
        std::env::temp_dir().join(format!("hypercosmos_ux_{}", std::process::id()))
    });
    std::fs::create_dir_all(&output).expect("capture directory");
    for (name, width, height) in [
        ("welcome", 1280, 720),
        ("help_fi", 720, 900),
        ("tour_paused", 1280, 720),
        ("vr_hologram", 2048, 2048),
        ("vr_hologram_lean", 2048, 2048),
    ] {
        if name == "vr_hologram" {
            rt.perform_guide_action(GuideAction::Explore);
            rt.flight.pose = cosmos_math::ScalePose64::new(
                cosmos_math::SectorId::EARTH_ORBIT,
                cosmos_nav::EARTH_ORBIT_HOME_LOCAL_POS,
                COLD_START_SCALE_EXP,
                glam::DQuat::IDENTITY,
            );
            rt.enable_spatial_guide(true);
            rt.update_vr_tracking(Vec3::ZERO, glam::Quat::IDENTITY, glam::Quat::IDENTITY);
        } else if name == "vr_hologram_lean" {
            rt.update_vr_tracking(
                Vec3::new(0.32, 0.08, 0.0),
                glam::Quat::from_rotation_y(-0.16),
                glam::Quat::IDENTITY,
            );
        } else if name == "help_fi" {
            rt.toggle_locale();
            rt.perform_guide_action(GuideAction::Help);
        } else if name == "tour_paused" {
            rt.toggle_locale();
            rt.perform_guide_action(GuideAction::Highlights);
            for _ in 0..100 {
                let cam = RenderCamera::mono(
                    rt.flight.pose.orient.as_quat(),
                    72.0_f32.to_radians(),
                    width as f32 / height as f32,
                    DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
                );
                rt.step_desktop_frame(
                    &DesktopFlightInput::default(),
                    Vec3::NEG_Z,
                    false,
                    false,
                    false,
                    &cam,
                    0.05,
                    width,
                    height,
                );
            }
            rt.perform_guide_action(GuideAction::ToggleTour);
        }
        let vr = name.starts_with("vr_");
        let mut cam = RenderCamera::mono(
            if name == "vr_hologram_lean" {
                glam::Quat::from_rotation_y(-0.16)
            } else {
                rt.flight.pose.orient.as_quat()
            },
            if vr {
                90.0_f32.to_radians()
            } else {
                72.0_f32.to_radians()
            },
            width as f32 / height as f32,
            DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
        );
        if vr {
            let rotation = if name == "vr_hologram_lean" {
                glam::Quat::from_rotation_y(-0.16)
            } else {
                glam::Quat::IDENTITY
            };
            let projection = cosmos_math::reversed_z_infinite_perspective_rh(
                cam.fov_y_rad,
                cam.aspect,
                cam.near_plane,
            );
            for (eye, x) in [-0.032, 0.032].into_iter().enumerate() {
                let position = rotation * Vec3::new(x, 0.0, 0.0);
                let view = glam::Mat4::from_rotation_translation(rotation, position).inverse();
                cam.eyes[eye] = cosmos_render::EyeView::from_view_proj(projection * view, position);
            }
            cam.is_stereo = true;
        }
        let pkt = if vr {
            rt.step_vr_frame(
                &cosmos_nav::VrFlightInput::default(),
                &cosmos_nav::WristSummonInput::default(),
                &cam,
                Vec3::NEG_Z,
                0.0,
                0.016,
                width,
                height,
            )
        } else {
            rt.step_desktop_frame(
                &DesktopFlightInput::default(),
                Vec3::NEG_Z,
                false,
                false,
                false,
                &cam,
                0.016,
                width,
                height,
            )
        };
        if vr {
            assert!(pkt.typo.foreground_start.is_none());
            assert!(pkt.guide.hologram.is_some());
            assert!(pkt.typo.glyphs.iter().all(
                |g| g.ink_params[3] != cosmos_typo::GlyphInkMode::ControlBackdrop as u8 as f32
            ));
        } else {
            let (glyph_start, _) = pkt.typo.foreground_start.expect("foreground controls");
            assert!(pkt.typo.glyphs.len() - glyph_start > 60);
        }
        text.upload(
            renderer.device(),
            renderer.queue(),
            &TextPassUniforms::default(),
            &pkt.typo.glyphs,
            &pkt.typo.tethers,
        );
        text.set_foreground_start(pkt.typo.foreground_start);
        let path = output.join(format!("{name}.png"));
        let pixels = renderer
            .render_to_png(&path, &pkt.scene, width, height, |pass, stereo| {
                text.render(pass, None, stereo)
            })
            .expect("guide capture");
        if vr {
            capture_guide_stereo(
                &mut renderer,
                &text,
                &pkt.scene,
                width,
                height,
                &output,
                name,
            );
        }
        assert!(
            pixels
                .as_chunks::<4>()
                .0
                .iter()
                .filter(|p| p[0] > 180 && p[1] > 180 && p[2] > 180)
                .count()
                > 500,
            "missing readable text"
        );
        if name == "tour_paused" {
            // Desktop foreground controls must survive a nearby opaque object.
            // Real orbital Earth is millions of meters away, so moving its
            // center a fraction of that distance cannot occlude room-scale UI.
            // Use a deliberate two-meter matte sphere for this depth fixture;
            // the actual Earth scene above retains its physical dimensions.
            let mut close_scene = pkt.scene.clone();
            let mut occluder = *close_scene.heroes_surface.iter()
                .find(|hero| hero.kind_tags[0] == 0).expect("Earth surface template");
            let center = cam.cam_forward * 2.2;
            occluder.pos_radius = [center.x, center.y, center.z, 2.0 / 0.84];
            occluder.kind_tags = [1, 12, 0, 3 << 24];
            occluder.phys_a = [0.0; 4];
            occluder.color_a = [0.85, 0.38, 0.22, 1.0];
            close_scene.heroes_surface = vec![occluder];
            close_scene.heroes_volume.clear();
            let close_pixels = renderer
                .render_to_png(
                    &output.join("controls_close_up.png"),
                    &close_scene,
                    width,
                    height,
                    |pass, stereo| text.render(pass, None, stereo),
                )
                .expect("close-up controls");
            text.set_foreground_start(None);
            let occluded_path = output.join("tour_world_depth.png");
            let occluded = renderer
                .render_to_png(
                    &occluded_path,
                    &close_scene,
                    width,
                    height,
                    |pass, stereo| text.render(pass, None, stereo),
                )
                .expect("occlusion reference");
            let changed = close_pixels
                .as_chunks::<4>()
                .0
                .iter()
                .zip(occluded.as_chunks::<4>().0)
                .filter(|(a, b)| a.iter().zip(b.iter()).any(|(x, y)| x.abs_diff(*y) > 24))
                .count();
            assert!(
                changed > 1_000,
                "close-up must exercise foreground visibility, changed={changed}"
            );
        }
    }
    if export.is_none() {
        std::fs::remove_dir_all(output).expect("remove test captures");
    }
}

fn capture_guide_stereo(
    renderer: &mut cosmos_render::CosmosRenderer,
    text: &cosmos_typo::MsdfTextRenderer,
    scene: &cosmos_render::RenderSceneFrame,
    width: u32,
    height: u32,
    output: &std::path::Path,
    name: &str,
) {
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let texture = renderer.device().create_texture(&wgpu::TextureDescriptor {
        label: Some("guide_stereo_review"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 2,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        array_layer_count: Some(2),
        ..Default::default()
    });
    renderer.render_frame(scene, &view, format, width, height, true, |pass, stereo| {
        assert!(stereo);
        text.render(pass, None, stereo);
    });
    let stride = (width * 4).div_ceil(256) * 256;
    let buffer = renderer.device().create_buffer(&wgpu::BufferDescriptor {
        label: Some("guide_stereo_readback"),
        size: u64::from(stride) * u64::from(height) * 2,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = renderer
        .device()
        .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(stride),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 2,
        },
    );
    renderer.queue().submit(Some(encoder.finish()));
    let (tx, rx) = std::sync::mpsc::channel();
    buffer.slice(..).map_async(wgpu::MapMode::Read, move |r| {
        tx.send(r).unwrap();
    });
    renderer
        .device()
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("GPU stereo completion");
    rx.recv().unwrap().expect("stereo readback map");
    let data = buffer.slice(..).get_mapped_range().expect("stereo pixels");
    let mut eyes = Vec::new();
    for eye in 0..2 {
        let mut pixels = Vec::new();
        for row in 0..height {
            let offset = ((eye * height + row) * stride) as usize;
            pixels.extend_from_slice(&data[offset..offset + (width * 4) as usize]);
        }
        let black = pixels
            .as_chunks::<4>()
            .0
            .iter()
            .filter(|p| p[0] == 0 && p[1] == 0 && p[2] == 0)
            .count();
        assert!(
            black > (width * height / 2) as usize,
            "OLED scene must retain a majority of exact-black space"
        );
        image::RgbaImage::from_raw(width, height, pixels.clone())
            .unwrap()
            .save(output.join(format!("{name}_eye_{eye}.png")))
            .expect("eye capture");
        eyes.push(pixels);
    }
    assert_ne!(
        eyes[0], eyes[1],
        "holographic scene must render both eye perspectives"
    );
    drop(data);
    buffer.unmap();
    if name == "vr_hologram" {
        let mut report = format!(
            "DX12 stereo reference: {width}x{height} per eye; 64 mm IPD\nRender submission + GPU completion, excluding OpenXR compositor and simulation.\n120 Hz total-frame budget: 8.333 ms. This is not a headset performance certification.\n"
        );
        report.push_str(if cfg!(debug_assertions) {
            "CPU build: debug (unoptimized).\n"
        } else {
            "CPU build: release (optimized).\n"
        });
        for visible in [false, true] {
            let mut samples = Vec::new();
            for frame in 0..34 {
                let start = std::time::Instant::now();
                renderer.render_frame(scene, &view, format, width, height, true, |pass, stereo| {
                    if visible {
                        text.render(pass, None, stereo);
                    }
                });
                renderer
                    .device()
                    .poll(wgpu::PollType::wait_indefinitely())
                    .expect("timed frame completion");
                if frame >= 4 {
                    samples.push(start.elapsed().as_secs_f64() * 1000.0);
                }
            }
            let mean = samples.iter().sum::<f64>() / samples.len() as f64;
            samples.sort_by(f64::total_cmp);
            report.push_str(&format!(
                "Typography {}: mean={mean:.3} ms, max={:.3} ms (30 frames)\n",
                if visible { "visible" } else { "hidden" },
                samples.last().unwrap()
            ));
        }
        std::fs::write(output.join("stereo_timing.txt"), report).expect("stereo timing report");
    }
}

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
        matches!(
            pkt.fsm_state,
            FsmStateKind::Narrating | FsmStateKind::Complete
        ),
        "expected Narrating or Complete after >900ms stationary gaze, got {:?}",
        pkt.fsm_state
    );
    assert!(
        !pkt.typo.glyphs.is_empty(),
        "expected 3D MSDF glyphs active"
    );
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
    let moved = rt.step_desktop_frame(
        &fly,
        Vec3::NEG_Z,
        false,
        false,
        false,
        &cam,
        0.05,
        1280,
        720,
    );
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
        assert!(
            s.path.is_file(),
            "missing rendered PNG {}",
            s.path.display()
        );
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
