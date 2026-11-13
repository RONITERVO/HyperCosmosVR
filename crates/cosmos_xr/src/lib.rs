//! OpenXR 1.0/1.1 Windows D3D12 + `wgpu::hal::api::Dx12` stereo bridge,
//! graceful Desktop HDR fallback discovery, asymmetric Reversed-Z stereo projection,
//! multi-profile controller bindings, haptics, and `XR_EXT_hand_tracking`.

pub mod error;
pub mod input;
pub mod runtime;
pub mod session;

pub use error::XrError;
pub use input::{
    HandJointSample, HandJointSnapshot, XrActionRig, XrHand, XrHandState, XrInputFrame, XrPose,
    XrStereoMatrices, compute_stereo_matrices, extract_pinch_and_palm_from_joints,
    xr_fov_to_tangents,
};
pub use runtime::{
    AdapterLuid, NoHeadsetReason, RecommendedViewConfig, XrAvailability, XrExtensionSupport,
    XrSystemDetails, probe_openxr,
};
pub use session::{
    NegotiatedSwapchainFormat, SwapchainFormatPreference, XrFrameData, XrLifecycleStatus,
    XrSessionBridge, XrSessionConfig, XrStereoSwapchainImage, negotiate_swapchain_format,
    select_best_refresh_rate,
};

#[cfg(test)]
mod tests {
    use cosmos_math::DEFAULT_VR_NEAR_RENDER_UNITS;
    use glam::{Quat, Vec3, Vec4};
    use openxr as xr;
    use windows::Win32::Graphics::Dxgi::Common::{
        DXGI_FORMAT_B8G8R8A8_UNORM_SRGB, DXGI_FORMAT_R8G8B8A8_UNORM,
        DXGI_FORMAT_R8G8B8A8_UNORM_SRGB, DXGI_FORMAT_R10G10B10A2_UNORM,
        DXGI_FORMAT_R16G16B16A16_FLOAT,
    };

    use super::*;

    #[test]
    fn openxr_static_entry_and_graceful_headset_probe() {
        let entry = xr::Entry::linked(&()).expect("static Khronos OpenXR loader links cleanly");
        let exts = entry
            .enumerate_extensions()
            .expect("enumerate_extensions succeeds against installed OpenXR runtime");
        assert!(
            exts.khr_d3d12_enable,
            "Windows OpenXR runtime must advertise XR_KHR_D3D12_enable"
        );

        match probe_openxr("HyperCosmosVR_Test", 1) {
            XrAvailability::Available { details, .. } => {
                assert!(details.extensions.khr_d3d12_enable);
                assert!(details.recommended_eye_res[0].width >= 64);
                assert!(details.recommended_eye_res[1].height >= 64);
                println!(
                    "OpenXR HMD available: runtime={} system={} luid=({}:{}) eye={}x{}",
                    details.runtime_name,
                    details.system_name,
                    details.adapter_luid.low_part,
                    details.adapter_luid.high_part,
                    details.recommended_eye_res[0].width,
                    details.recommended_eye_res[0].height,
                );
            }
            XrAvailability::NoHeadset {
                reason,
                runtime_name,
                extensions,
            } => {
                assert!(
                    !matches!(reason, NoHeadsetReason::LoaderFailed(_)),
                    "static loader failed unexpectedly: {reason:?}"
                );
                assert!(extensions.is_some());
                println!(
                    "OpenXR graceful NoHeadset fallback verified: runtime={runtime_name:?} reason={reason:?}"
                );
            }
        }
    }

    #[test]
    fn swapchain_format_negotiation_matches_shader_encode_modes() {
        let runtime_formats = vec![
            DXGI_FORMAT_R8G8B8A8_UNORM.0 as u32,
            DXGI_FORMAT_R8G8B8A8_UNORM_SRGB.0 as u32,
            DXGI_FORMAT_B8G8R8A8_UNORM_SRGB.0 as u32,
            DXGI_FORMAT_R10G10B10A2_UNORM.0 as u32,
            DXGI_FORMAT_R16G16B16A16_FLOAT.0 as u32,
        ];

        let srgb =
            negotiate_swapchain_format(&runtime_formats, SwapchainFormatPreference::SrgbFirst)
                .unwrap();
        assert_eq!(srgb.wgpu_format, wgpu::TextureFormat::Rgba8UnormSrgb);
        assert_eq!(srgb.output_encode_mode, 0.0);

        let hdr10 = negotiate_swapchain_format(
            &runtime_formats,
            SwapchainFormatPreference::Hdr10BitFirst,
        )
        .unwrap();
        assert_eq!(hdr10.wgpu_format, wgpu::TextureFormat::Rgb10a2Unorm);
        assert_eq!(hdr10.output_encode_mode, 1.0);

        let float16 = negotiate_swapchain_format(
            &runtime_formats,
            SwapchainFormatPreference::LinearFloatFirst,
        )
        .unwrap();
        assert_eq!(float16.wgpu_format, wgpu::TextureFormat::Rgba16Float);
        assert_eq!(float16.output_encode_mode, 2.0);

        // Fallback to manual sRGB encode when runtime exposes only UNORM
        let unorm_only = [DXGI_FORMAT_R8G8B8A8_UNORM.0 as u32];
        let fallback =
            negotiate_swapchain_format(&unorm_only, SwapchainFormatPreference::SrgbFirst)
                .unwrap();
        assert_eq!(fallback.wgpu_format, wgpu::TextureFormat::Rgba8Unorm);
        assert_eq!(fallback.output_encode_mode, 1.0);

        assert!(matches!(
            negotiate_swapchain_format(&[999_999], SwapchainFormatPreference::SrgbFirst),
            Err(XrError::UnsupportedSwapchainFormat)
        ));
    }

    #[test]
    fn refresh_rate_selection_prefers_oled_120_and_90_hz() {
        assert_eq!(
            select_best_refresh_rate(&[72.0, 80.0, 90.0, 120.0], None),
            Some(120.0)
        );
        assert_eq!(
            select_best_refresh_rate(&[72.0, 80.0, 90.0], None),
            Some(90.0)
        );
        assert_eq!(
            select_best_refresh_rate(&[72.0, 90.0, 120.0], Some(90.0)),
            Some(90.0)
        );
        assert_eq!(select_best_refresh_rate(&[], None), None);
    }

    #[test]
    fn stereo_asymmetric_reversed_z_matrices_and_ray_inversion() {
        // 64 mm IPD, realistic asymmetric VR eye FOVs
        let eye_poses = [
            xr::Posef {
                orientation: xr::Quaternionf::IDENTITY,
                position: xr::Vector3f {
                    x: -0.032,
                    y: 1.65,
                    z: 0.0,
                },
            },
            xr::Posef {
                orientation: xr::Quaternionf::IDENTITY,
                position: xr::Vector3f {
                    x: 0.032,
                    y: 1.65,
                    z: 0.0,
                },
            },
        ];
        let eye_fovs = [
            xr::Fovf {
                angle_left: -0.90,
                angle_right: 0.72,
                angle_up: 0.85,
                angle_down: -0.88,
            },
            xr::Fovf {
                angle_left: -0.72,
                angle_right: 0.90,
                angle_up: 0.85,
                angle_down: -0.88,
            },
        ];

        let rig_rot = Quat::from_rotation_y(0.35);
        let near = DEFAULT_VR_NEAR_RENDER_UNITS;
        let mats = compute_stereo_matrices(&eye_poses, &eye_fovs, 2048, 2208, near, rig_rot);

        // Camera center sits at head midpoint -> eye offsets sum cleanly to 0 with w = 1.0 (stereo active)
        assert!((mats.eye_offset[0].truncate() + mats.eye_offset[1].truncate()).length() < 1e-6);
        assert!(
            ((mats.eye_offset[0].truncate() - mats.eye_offset[1].truncate()).length() - 0.064)
                .abs()
                < 1e-5
        );
        assert_eq!(mats.eye_offset[0].w, 1.0);
        assert_eq!(mats.eye_offset[1].w, 1.0);

        // Reversed-Z invariant per eye: near plane z_ndc == 1.0, far/distant -> 0.0
        for eye in 0..2 {
            let eye_pos = mats.eye_offset[eye].truncate();
            let p_near = eye_pos + mats.cam_forward * near;
            let c_near = mats.view_proj[eye] * Vec4::from((p_near, 1.0));
            let z_near = c_near.z / c_near.w;
            assert!((z_near - 1.0).abs() < 1e-5, "eye {eye} near z_ndc={z_near}");

            let p_far = eye_pos + mats.cam_forward * 10_000.0;
            let c_far = mats.view_proj[eye] * Vec4::from((p_far, 1.0));
            let z_far = c_far.z / c_far.w;
            assert!(z_far > 0.0 && z_far < 1e-4, "eye {eye} far z_ndc={z_far}");

            // Inverse matrix cleanly reconstructs 3D world ray (SHADER_CONTRACTS.md `ndc_to_world_ray`)
            let clip_near = mats.inv_view_proj[eye] * Vec4::new(0.0, 0.0, 1.0, 1.0);
            let clip_mid = mats.inv_view_proj[eye] * Vec4::new(0.0, 0.0, 0.5, 1.0);
            let w_near = clip_near.truncate() / clip_near.w;
            let w_mid = clip_mid.truncate() / clip_mid.w;
            let dir = (w_mid - w_near).normalize();
            assert!(dir.dot(mats.cam_forward) > 0.5);
        }
    }

    #[test]
    fn hand_pinch_palm_up_and_two_hand_scale_dive_extraction() {
        let mut joints = [HandJointSample::default(); xr::HAND_JOINT_COUNT];
        // Palm facing upward (`palm_local -Y == +World_Y` via 180 deg rotation around X)
        let rot_palm_up = Quat::from_rotation_x(std::f32::consts::PI);
        joints[xr::HandJoint::PALM.into_raw() as usize] = HandJointSample {
            position: Vec3::new(-0.20, 1.20, -0.35),
            orientation: rot_palm_up,
            radius: 0.03,
            valid: true,
        };
        joints[xr::HandJoint::THUMB_TIP.into_raw() as usize] = HandJointSample {
            position: Vec3::new(-0.18, 1.25, -0.35),
            orientation: Quat::IDENTITY,
            radius: 0.01,
            valid: true,
        };
        joints[xr::HandJoint::INDEX_TIP.into_raw() as usize] = HandJointSample {
            position: Vec3::new(-0.18, 1.265, -0.35), // 1.5 cm apart -> tight pinch
            orientation: Quat::IDENTITY,
            radius: 0.01,
            valid: true,
        };

        let (strength, pinching, mid, palm_score) =
            extract_pinch_and_palm_from_joints(&joints, Vec3::Y);
        assert!(pinching);
        assert!(strength > 0.99);
        assert!(palm_score > 0.99);
        assert!((mid - Vec3::new(-0.18, 1.2575, -0.35)).length() < 1e-5);

        // Build two-hand frame and check two-hand pinch span
        let mut frame = XrInputFrame {
            left: XrHandState {
                is_pinching: true,
                pinch_strength: 1.0,
                hand_joints: Some(HandJointSnapshot {
                    joints,
                    pinch_strength: strength,
                    is_pinching: true,
                    pinch_midpoint: Vec3::new(-0.25, 1.2, -0.4),
                    palm_up_score: palm_score,
                }),
                ..Default::default()
            },
            right: XrHandState {
                is_pinching: true,
                pinch_strength: 1.0,
                grip_pose: XrPose {
                    position: Vec3::new(0.25, 1.2, -0.4),
                    orientation: Quat::IDENTITY,
                    valid: true,
                },
                ..Default::default()
            },
            ..Default::default()
        };
        frame.update_two_hand_pinch();
        assert!(frame.two_hand_pinching);
        assert!((frame.two_hand_span_meters - 0.50).abs() < 1e-5);
        assert!((frame.two_hand_midpoint - Vec3::new(0.0, 1.2, -0.4)).length() < 1e-5);
    }
}
