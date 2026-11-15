use std::time::Instant;

use anyhow::Result;
use cosmos_audio::AudioOutputMode;
use cosmos_math::DEFAULT_VR_NEAR_RENDER_UNITS;
use cosmos_nav::{VrFlightInput, WristSummonInput};
use cosmos_render::{CosmosRenderer, DEPTH_FORMAT, EyeView, HDR_SCENE_FORMAT, RenderCamera};
use cosmos_typo::{MsdfPipelineConfig, MsdfTextRenderer, TextPassUniforms};
use cosmos_xr::{XrAvailability, XrHand, XrSessionBridge, XrSessionConfig};
use glam::{DVec2, Quat, Vec3};

use crate::assets::UniverseAssets;
use crate::engine::{LaunchJourneyConfig, UniverseRuntime};

/// Attempts to initialize and run the native OpenXR D3D12 stereo multiview loop (`90/120 Hz`)
/// pinned strictly to the OpenXR HMD DXGI `adapter_luid`.
///
/// Returns `Ok(true)` if a VR session ran and exited cleanly, or `Ok(false)` when no HMD is active
/// so the caller seamlessly falls back to the Desktop HDR window.
pub fn try_run_openxr_session(availability: XrAvailability) -> Result<bool> {
    try_run_openxr_session_with_journey(availability, LaunchJourneyConfig::default())
}

/// Native OpenXR D3D12 stereo loop with optional Story-Journey launch configuration.
pub fn try_run_openxr_session_with_journey(
    availability: XrAvailability,
    journey_cfg: LaunchJourneyConfig,
) -> Result<bool> {
    let XrAvailability::Available {
        entry: _entry,
        instance,
        system_id,
        details,
    } = availability
    else {
        return Ok(false);
    };

    let eye_w = details.recommended_eye_res[0].width.max(64);
    let eye_h = details.recommended_eye_res[0].height.max(64);
    let luid = details.adapter_luid.to_tuple();

    let mut renderer = CosmosRenderer::init_for_dx12_luid(Some(luid), eye_w, eye_h, true)?;
    let (raw_dev, raw_queue) = renderer.dx12_raw_device_and_queue()?;

    let mut session = unsafe {
        XrSessionBridge::create_d3d12(
            instance,
            system_id,
            &details,
            raw_dev,
            raw_queue,
            renderer.device(),
            XrSessionConfig {
                near_plane_render_units: DEFAULT_VR_NEAR_RENDER_UNITS,
                ..Default::default()
            },
        )?
    };

    let assets = UniverseAssets::load()?;
    let shaders_dir = assets.shaders_dir.clone();
    let mut runtime = UniverseRuntime::new(assets, AudioOutputMode::Auto)?;
    journey_cfg.apply(&mut runtime)?;
    runtime.enable_spatial_guide(true);

    let mut text_renderer = MsdfTextRenderer::from_shaders_dir(
        renderer.device(),
        renderer.queue(),
        &runtime.atlas,
        &shaders_dir,
        renderer.shared_frame_bind_group_layout(),
        MsdfPipelineConfig {
            color_format: HDR_SCENE_FORMAT,
            depth_format: Some(DEPTH_FORMAT),
            enable_stereo_multiview: true,
            sample_count: 1,
        },
    )?;

    eprintln!(
        "[HyperCosmosVR] OpenXR Stereo D3D12 session started on '{}' ({}x{}/eye, LUID={:?})",
        renderer.adapter_info().name,
        eye_w,
        eye_h,
        renderer.adapter_luid()
    );

    let mut last_tick = Instant::now();
    let mut prev_menu_click = false;
    let mut prev_secondary_click = false;
    let mut prev_select = false;

    loop {
        let lifecycle = session.poll_events()?;
        if lifecycle.should_exit {
            break;
        }
        if !lifecycle.session_running {
            runtime.pause_for_focus_loss();
            std::thread::sleep(std::time::Duration::from_millis(16));
            continue;
        }

        let rig_orient = runtime.flight.pose.orient.as_quat();
        let Some(xr_frame) = session.begin_frame(rig_orient)? else {
            std::thread::sleep(std::time::Duration::from_millis(8));
            continue;
        };
        let input_active = lifecycle.focused && xr_frame.head_pose.valid && xr_frame.should_render;
        if !input_active {
            runtime.pause_for_focus_loss();
        }

        let now = Instant::now();
        let dt_sec = (now - last_tick).as_secs_f32().clamp(0.0005, 0.05);
        last_tick = now;

        let xr_cam = RenderCamera {
            eyes: [
                EyeView::from_view_proj(xr_frame.view_proj[0], xr_frame.eye_offset[0].truncate()),
                EyeView::from_view_proj(xr_frame.view_proj[1], xr_frame.eye_offset[1].truncate()),
            ],
            is_stereo: true,
            cam_forward: xr_frame.cam_forward,
            cam_right: xr_frame.cam_right,
            cam_up: xr_frame.cam_up,
            fov_y_rad: xr_frame.center_fov_y_rad,
            aspect: xr_frame.aspect,
            near_plane: DEFAULT_VR_NEAR_RENDER_UNITS,
        };

        let head_up_tracking = (xr_frame.head_pose.orientation * Vec3::Y).normalize_or_zero();
        let input = session.poll_input(&xr_frame, head_up_tracking)?;
        let head_pos = xr_frame.head_pose.position;
        if xr_frame.head_pose.valid {
            runtime.update_vr_tracking(head_pos, xr_frame.head_pose.orientation, rig_orient);
        }
        let to_cam_space = |p_track: Vec3| p_track - head_pos;

        let left_pinch_pos = input
            .left
            .hand_joints
            .and_then(|j| j.is_pinching.then_some(j.pinch_midpoint))
            .unwrap_or(input.left.grip_pose.position);
        let right_pinch_pos = input
            .right
            .hand_joints
            .and_then(|j| j.is_pinching.then_some(j.pinch_midpoint))
            .unwrap_or(input.right.grip_pose.position);

        let left_grip = input.left.grip > 0.6 || input.left.is_pinching;
        let right_grip = input.right.grip > 0.6 || input.right.is_pinching;

        let dominant = if input.right.aim_pose.valid {
            &input.right
        } else {
            &input.left
        };
        let dominant_aim_local = if dominant.aim_pose.valid {
            dominant.aim_pose.forward()
        } else {
            xr_frame.head_pose.forward()
        };

        let menu_click = input.left.menu_click || input.right.menu_click;
        let home_pressed = input_active && menu_click && !prev_menu_click;
        prev_menu_click = menu_click;

        let secondary_edge = input_active && input.right.secondary_click && !prev_secondary_click;
        prev_secondary_click = input.right.secondary_click;
        if secondary_edge && runtime.is_journey_paused_by_leash() {
            runtime.journey_confirm_exit();
        }

        let vr_flight = if input_active {
            VrFlightInput {
                left_grip,
                right_grip,
                left_hand_pos: to_cam_space(left_pinch_pos).as_dvec3(),
                right_hand_pos: to_cam_space(right_pinch_pos).as_dvec3(),
                dominant_hand_forward_cam: dominant_aim_local.as_dvec3(),
                left_stick: DVec2::new(
                    input.left.thumbstick.x as f64,
                    input.left.thumbstick.y as f64,
                ),
                right_stick: DVec2::new(
                    input.right.thumbstick.x as f64,
                    input.right.thumbstick.y as f64,
                ),
                vertical_axis: (input.right.primary_click as i32
                    - input.right.secondary_click as i32) as f64,
                pitch_axis: 0.0,
                roll_axis: 0.0,
                trigger_pull: input.right.trigger.max(input.left.trigger) as f64,
                home_pressed,
            }
        } else {
            VrFlightInput::default()
        };

        let wrist_pos_world = rig_orient * to_cam_space(input.left.grip_pose.position);
        let wrist_rot_world = (rig_orient * input.left.grip_pose.orientation).normalize();
        let palm_normal_world = if let Some(joints) = input.left.hand_joints {
            (rig_orient * joints.joints[0].orientation * -Vec3::Y).normalize_or_zero()
        } else {
            (wrist_rot_world * -Vec3::Y).normalize_or_zero()
        };
        let finger_tip_world = input.right.hand_joints.and_then(|j| {
            j.joints[10]
                .valid
                .then(|| rig_orient * to_cam_space(j.joints[10].position))
        });

        let select_down = dominant.trigger > 0.75 || dominant.select_click || dominant.is_pinching;
        let select_edge = input_active && select_down && !prev_select;
        prev_select = select_down;
        let wrist_input = WristSummonInput {
            is_vr: true,
            wrist_pos: wrist_pos_world,
            wrist_rot: wrist_rot_world,
            palm_normal_world,
            head_pos: Vec3::ZERO,
            head_up: xr_cam.cam_up,
            cam_pos: Vec3::ZERO,
            cam_rot: rig_orient,
            desktop_right_hold: false,
            desktop_tab_just_pressed: false,
            pointer_origin: dominant
                .aim_pose
                .valid
                .then(|| rig_orient * to_cam_space(dominant.aim_pose.position)),
            pointer_dir: dominant
                .aim_pose
                .valid
                .then(|| rig_orient * dominant.aim_pose.forward()),
            finger_tip_world,
            select_triggered: select_edge,
        };

        let pkt = runtime.step_vr_frame(
            &vr_flight,
            &wrist_input,
            &xr_cam,
            dominant_aim_local,
            0.0,
            dt_sec,
            xr_frame.eye_width,
            xr_frame.eye_height,
        );

        if let Some((amp, dur_ns, freq_hz)) = pkt.haptic_pulse {
            let _ = session.apply_haptic(XrHand::Left, amp, dur_ns, freq_hz);
            let _ = session.apply_haptic(XrHand::Right, amp * 0.85, dur_ns, freq_hz);
        }

        if xr_frame.should_render
            && let Some(sc_img) = session.current_swapchain_image(&xr_frame)
        {
            text_renderer.upload(
                renderer.device(),
                renderer.queue(),
                &TextPassUniforms::default(),
                &pkt.typo.glyphs,
                &pkt.typo.tethers,
            );
            text_renderer.set_foreground_start(pkt.typo.foreground_start);
            renderer.render_frame(
                &pkt.scene,
                &sc_img.multiview_view,
                xr_frame.format.wgpu_format,
                xr_frame.eye_width,
                xr_frame.eye_height,
                true,
                |rpass, stereo| text_renderer.render(rpass, None, stereo),
            );
        }

        session.end_frame(&xr_frame)?;
    }

    Ok(true)
}

#[allow(dead_code)]
fn _unused_quat_sanity(_q: Quat) {}
