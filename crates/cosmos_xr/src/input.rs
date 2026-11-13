use cosmos_math::reversed_z_infinite_asymmetric_rh;
use glam::{Mat4, Quat, Vec2, Vec3, Vec4};
use openxr as xr;

use crate::error::XrError;

/// Left or right VR controller / hand subaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum XrHand {
    Left = 0,
    Right = 1,
}

impl XrHand {
    #[inline]
    pub const fn index(self) -> usize {
        self as usize
    }
}

/// Rigid tracking pose in OpenXR local/reference space (meters).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XrPose {
    pub position: Vec3,
    pub orientation: Quat,
    pub valid: bool,
}

impl Default for XrPose {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            orientation: Quat::IDENTITY,
            valid: false,
        }
    }
}

impl XrPose {
    #[inline]
    pub fn from_openxr(pose: xr::Posef, valid: bool) -> Self {
        let q = Quat::from_xyzw(
            pose.orientation.x,
            pose.orientation.y,
            pose.orientation.z,
            pose.orientation.w,
        );
        let orientation = if q.length_squared() > 1e-8 {
            q.normalize()
        } else {
            Quat::IDENTITY
        };
        Self {
            position: Vec3::new(pose.position.x, pose.position.y, pose.position.z),
            orientation,
            valid,
        }
    }

    /// Forward ray (`-Z` in OpenXR right-handed pose coordinates).
    #[inline]
    pub fn forward(&self) -> Vec3 {
        self.orientation * -Vec3::Z
    }
}

/// Camera-relative Reversed-Z stereo matrices ready for `FrameUniforms` (`shaders/SHADER_CONTRACTS.md`).
#[derive(Clone, Copy, Debug)]
pub struct XrStereoMatrices {
    pub view_proj: [Mat4; 2],
    pub inv_view_proj: [Mat4; 2],
    pub eye_offset: [Vec4; 2],
    pub head_pose: XrPose,
    pub eye_poses: [XrPose; 2],
    pub cam_forward: Vec3,
    pub cam_right: Vec3,
    pub cam_up: Vec3,
    pub center_fov_y_rad: f32,
    pub aspect: f32,
}

/// Converts OpenXR asymmetric FOV angles into tangents `(tan_left, tan_right, tan_down, tan_up)`.
#[inline]
pub fn xr_fov_to_tangents(fov: xr::Fovf) -> (f32, f32, f32, f32) {
    (
        fov.angle_left.tan(),
        fov.angle_right.tan(),
        fov.angle_down.tan(),
        fov.angle_up.tan(),
    )
}

/// Builds stereo Reversed-Z infinite far-plane `view_proj`, `inv_view_proj`, and camera-relative
/// `eye_offset` matrices (`w = 1.0` indicating stereo active) from OpenXR `xr::View` poses & FOVs.
///
/// The head midpoint sits at camera-relative world `(0, 0, 0)` while `world_rig_orient` rotates
/// OpenXR local tracking axes into the active `ScalePose64` universe orientation.
pub fn compute_stereo_matrices(
    eye_poses: &[xr::Posef; 2],
    eye_fovs: &[xr::Fovf; 2],
    eye_width: u32,
    eye_height: u32,
    near: f32,
    world_rig_orient: Quat,
) -> XrStereoMatrices {
    let rig_rot = if world_rig_orient.length_squared() > 1e-8 {
        world_rig_orient.normalize()
    } else {
        Quat::IDENTITY
    };
    let safe_near = near.max(1e-4);

    let p0 = XrPose::from_openxr(eye_poses[0], true);
    let p1 = XrPose::from_openxr(eye_poses[1], true);
    let head_pos_local = 0.5 * (p0.position + p1.position);
    let head_q_local = p0.orientation.slerp(p1.orientation, 0.5).normalize();
    let head_q_world = (rig_rot * head_q_local).normalize();

    let mut view_proj = [Mat4::IDENTITY; 2];
    let mut inv_view_proj = [Mat4::IDENTITY; 2];
    let mut eye_offset = [Vec4::W; 2];

    for (idx, (pose, fov)) in [p0, p1].iter().zip(eye_fovs.iter()).enumerate() {
        let local_rel = pose.position - head_pos_local;
        let eye_pos_camrel = rig_rot * local_rel;
        let eye_rot_world = (rig_rot * pose.orientation).normalize();

        eye_offset[idx] = Vec4::new(eye_pos_camrel.x, eye_pos_camrel.y, eye_pos_camrel.z, 1.0);

        let view = Mat4::from_rotation_translation(eye_rot_world, eye_pos_camrel).inverse();
        let (tl, tr, td, tu) = xr_fov_to_tangents(*fov);
        let proj = reversed_z_infinite_asymmetric_rh(tl, tr, td, tu, safe_near);
        let vp = proj * view;

        view_proj[idx] = vp;
        inv_view_proj[idx] = vp.inverse();
    }

    let fov0_y = (eye_fovs[0].angle_up - eye_fovs[0].angle_down).abs();
    let fov1_y = (eye_fovs[1].angle_up - eye_fovs[1].angle_down).abs();

    XrStereoMatrices {
        view_proj,
        inv_view_proj,
        eye_offset,
        head_pose: XrPose {
            position: head_pos_local,
            orientation: head_q_local,
            valid: true,
        },
        eye_poses: [p0, p1],
        cam_forward: (head_q_world * -Vec3::Z).normalize(),
        cam_right: (head_q_world * Vec3::X).normalize(),
        cam_up: (head_q_world * Vec3::Y).normalize(),
        center_fov_y_rad: 0.5 * (fov0_y + fov1_y),
        aspect: eye_width.max(1) as f32 / eye_height.max(1) as f32,
    }
}

/// Single `XR_EXT_hand_tracking` joint measurement.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HandJointSample {
    pub position: Vec3,
    pub orientation: Quat,
    pub radius: f32,
    pub valid: bool,
}

/// 26-joint `XR_EXT_hand_tracking` snapshot plus derived pinch & palm-up gestures.
#[derive(Clone, Copy, Debug)]
pub struct HandJointSnapshot {
    pub joints: [HandJointSample; xr::HAND_JOINT_COUNT],
    pub pinch_strength: f32,
    pub is_pinching: bool,
    pub pinch_midpoint: Vec3,
    /// Dot product of palmar outward vector (`joint_palm * -Y`) against world/view up `[ -1 .. +1 ]`.
    pub palm_up_score: f32,
}

/// Extracts pinch strength, pinch midpoint, and palm-up dot product from OpenXR hand joints.
///
/// Per `XR_EXT_hand_tracking`, joint local `-Y` faces outward from the palm (`PALM = 0`,
/// `THUMB_TIP = 5`, `INDEX_TIP = 10`).
pub fn extract_pinch_and_palm_from_joints(
    joints: &[HandJointSample; xr::HAND_JOINT_COUNT],
    view_up: Vec3,
) -> (f32, bool, Vec3, f32) {
    let palm = joints[xr::HandJoint::PALM.into_raw() as usize];
    let thumb = joints[xr::HandJoint::THUMB_TIP.into_raw() as usize];
    let index = joints[xr::HandJoint::INDEX_TIP.into_raw() as usize];

    let (pinch_strength, is_pinching, pinch_midpoint) = if thumb.valid && index.valid {
        let dist = (thumb.position - index.position).length();
        // 6.5 cm open -> 0.0, 2.2 cm closed -> 1.0
        let strength = ((0.065 - dist) / (0.065 - 0.022)).clamp(0.0, 1.0);
        let mid = 0.5 * (thumb.position + index.position);
        (strength, strength >= 0.75, mid)
    } else {
        (0.0, false, Vec3::ZERO)
    };

    let palm_up_score = if palm.valid {
        let up = if view_up.length_squared() > 1e-8 {
            view_up.normalize()
        } else {
            Vec3::Y
        };
        let palm_out = palm.orientation * -Vec3::Y;
        palm_out.dot(up).clamp(-1.0, 1.0)
    } else {
        0.0
    };

    (pinch_strength, is_pinching, pinch_midpoint, palm_up_score)
}

/// Combined controller + hand tracking state for a single hand.
#[derive(Clone, Copy, Debug, Default)]
pub struct XrHandState {
    pub aim_pose: XrPose,
    pub grip_pose: XrPose,
    pub thumbstick: Vec2,
    pub trigger: f32,
    pub grip: f32,
    pub primary_click: bool,
    pub secondary_click: bool,
    pub thumbstick_click: bool,
    pub menu_click: bool,
    pub select_click: bool,
    pub pinch_strength: f32,
    pub is_pinching: bool,
    /// `[-1.0 .. 1.0]`: `> 0.65` summons the Wrist Resonance Ring on the palm.
    pub palm_up_score: f32,
    pub hand_joints: Option<HandJointSnapshot>,
}

/// Complete OpenXR input snapshot for a predicted frame, ready for `cosmos_nav`.
#[derive(Clone, Copy, Debug, Default)]
pub struct XrInputFrame {
    pub left: XrHandState,
    pub right: XrHandState,
    pub two_hand_pinching: bool,
    pub two_hand_span_meters: f32,
    pub two_hand_midpoint: Vec3,
}

impl XrInputFrame {
    /// Computes two-hand world-pinch telemetry when both hands squeeze/pinch concurrently.
    pub fn update_two_hand_pinch(&mut self) {
        let left_pos = self
            .left
            .hand_joints
            .and_then(|j| j.is_pinching.then_some(j.pinch_midpoint))
            .or(self.left.grip_pose.valid.then_some(self.left.grip_pose.position));
        let right_pos = self
            .right
            .hand_joints
            .and_then(|j| j.is_pinching.then_some(j.pinch_midpoint))
            .or(self.right.grip_pose.valid.then_some(self.right.grip_pose.position));

        let left_active = self.left.is_pinching || self.left.grip >= 0.70;
        let right_active = self.right.is_pinching || self.right.grip >= 0.70;

        if left_active
            && right_active
            && let (Some(lp), Some(rp)) = (left_pos, right_pos)
        {
            self.two_hand_pinching = true;
            self.two_hand_span_meters = (rp - lp).length();
            self.two_hand_midpoint = 0.5 * (lp + rp);
            return;
        }
        self.two_hand_pinching = false;
        self.two_hand_span_meters = 0.0;
        self.two_hand_midpoint = Vec3::ZERO;
    }
}

/// OpenXR action set, per-hand action spaces, and multi-profile interaction bindings.
pub struct XrActionRig {
    action_set: xr::ActionSet,
    subactions: [xr::Path; 2],
    aim_pose: xr::Action<xr::Posef>,
    grip_pose: xr::Action<xr::Posef>,
    thumbstick: xr::Action<xr::Vector2f>,
    thumbstick_click: xr::Action<bool>,
    trigger: xr::Action<f32>,
    grip: xr::Action<f32>,
    primary_click: xr::Action<bool>,
    secondary_click: xr::Action<bool>,
    menu_click: xr::Action<bool>,
    select_click: xr::Action<bool>,
    haptic: xr::Action<xr::Haptic>,
    aim_spaces: [xr::Space; 2],
    grip_spaces: [xr::Space; 2],
    pub bound_profiles: Vec<&'static str>,
}

impl XrActionRig {
    /// Creates and attaches standard Oculus Touch, Valve Index, and KHR Simple Controller bindings.
    pub fn attach(
        instance: &xr::Instance,
        session: &xr::Session<xr::D3D12>,
    ) -> Result<Self, XrError> {
        let left_path = instance.string_to_path(xr::USER_HAND_LEFT)?;
        let right_path = instance.string_to_path(xr::USER_HAND_RIGHT)?;
        let subactions = [left_path, right_path];

        let action_set =
            instance.create_action_set("hypercosmos_gameplay", "HyperCosmos Gameplay", 0)?;

        let aim_pose: xr::Action<xr::Posef> =
            action_set.create_action("aim_pose", "Pointer Aim Pose", &subactions)?;
        let grip_pose: xr::Action<xr::Posef> =
            action_set.create_action("grip_pose", "Hand Grip Pose", &subactions)?;
        let thumbstick: xr::Action<xr::Vector2f> =
            action_set.create_action("thumbstick", "Thumbstick Move", &subactions)?;
        let thumbstick_click: xr::Action<bool> =
            action_set.create_action("thumbstick_click", "Thumbstick Click", &subactions)?;
        let trigger: xr::Action<f32> =
            action_set.create_action("trigger", "Trigger Pinch", &subactions)?;
        let grip: xr::Action<f32> =
            action_set.create_action("grip", "Grip Squeeze", &subactions)?;
        let primary_click: xr::Action<bool> =
            action_set.create_action("primary_click", "Primary Button", &subactions)?;
        let secondary_click: xr::Action<bool> =
            action_set.create_action("secondary_click", "Secondary Button", &subactions)?;
        let menu_click: xr::Action<bool> =
            action_set.create_action("menu_click", "Menu Button", &subactions)?;
        let select_click: xr::Action<bool> =
            action_set.create_action("select_click", "Select Click", &subactions)?;
        let haptic: xr::Action<xr::Haptic> =
            action_set.create_action("haptic", "Haptic Pulse", &subactions)?;

        let p = |s: &str| instance.string_to_path(s);
        let mut bound_profiles = Vec::new();

        // 1) Oculus Touch Controller
        if let Ok(profile) = p("/interaction_profiles/oculus/touch_controller") {
            let bindings = [
                xr::Binding::new(&aim_pose, p("/user/hand/left/input/aim/pose")?),
                xr::Binding::new(&aim_pose, p("/user/hand/right/input/aim/pose")?),
                xr::Binding::new(&grip_pose, p("/user/hand/left/input/grip/pose")?),
                xr::Binding::new(&grip_pose, p("/user/hand/right/input/grip/pose")?),
                xr::Binding::new(&thumbstick, p("/user/hand/left/input/thumbstick")?),
                xr::Binding::new(&thumbstick, p("/user/hand/right/input/thumbstick")?),
                xr::Binding::new(
                    &thumbstick_click,
                    p("/user/hand/left/input/thumbstick/click")?,
                ),
                xr::Binding::new(
                    &thumbstick_click,
                    p("/user/hand/right/input/thumbstick/click")?,
                ),
                xr::Binding::new(&trigger, p("/user/hand/left/input/trigger/value")?),
                xr::Binding::new(&trigger, p("/user/hand/right/input/trigger/value")?),
                xr::Binding::new(&grip, p("/user/hand/left/input/squeeze/value")?),
                xr::Binding::new(&grip, p("/user/hand/right/input/squeeze/value")?),
                xr::Binding::new(&primary_click, p("/user/hand/left/input/x/click")?),
                xr::Binding::new(&primary_click, p("/user/hand/right/input/a/click")?),
                xr::Binding::new(&secondary_click, p("/user/hand/left/input/y/click")?),
                xr::Binding::new(&secondary_click, p("/user/hand/right/input/b/click")?),
                xr::Binding::new(&menu_click, p("/user/hand/left/input/menu/click")?),
                xr::Binding::new(&haptic, p("/user/hand/left/output/haptic")?),
                xr::Binding::new(&haptic, p("/user/hand/right/output/haptic")?),
            ];
            if instance
                .suggest_interaction_profile_bindings(profile, &bindings)
                .is_ok()
            {
                bound_profiles.push("/interaction_profiles/oculus/touch_controller");
            }
        }

        // 2) Valve Index Controller
        if let Ok(profile) = p("/interaction_profiles/valve/index_controller") {
            let bindings = [
                xr::Binding::new(&aim_pose, p("/user/hand/left/input/aim/pose")?),
                xr::Binding::new(&aim_pose, p("/user/hand/right/input/aim/pose")?),
                xr::Binding::new(&grip_pose, p("/user/hand/left/input/grip/pose")?),
                xr::Binding::new(&grip_pose, p("/user/hand/right/input/grip/pose")?),
                xr::Binding::new(&thumbstick, p("/user/hand/left/input/thumbstick")?),
                xr::Binding::new(&thumbstick, p("/user/hand/right/input/thumbstick")?),
                xr::Binding::new(
                    &thumbstick_click,
                    p("/user/hand/left/input/thumbstick/click")?,
                ),
                xr::Binding::new(
                    &thumbstick_click,
                    p("/user/hand/right/input/thumbstick/click")?,
                ),
                xr::Binding::new(&trigger, p("/user/hand/left/input/trigger/value")?),
                xr::Binding::new(&trigger, p("/user/hand/right/input/trigger/value")?),
                xr::Binding::new(&grip, p("/user/hand/left/input/squeeze/value")?),
                xr::Binding::new(&grip, p("/user/hand/right/input/squeeze/value")?),
                xr::Binding::new(&primary_click, p("/user/hand/left/input/a/click")?),
                xr::Binding::new(&primary_click, p("/user/hand/right/input/a/click")?),
                xr::Binding::new(&secondary_click, p("/user/hand/left/input/b/click")?),
                xr::Binding::new(&secondary_click, p("/user/hand/right/input/b/click")?),
                xr::Binding::new(&haptic, p("/user/hand/left/output/haptic")?),
                xr::Binding::new(&haptic, p("/user/hand/right/output/haptic")?),
            ];
            if instance
                .suggest_interaction_profile_bindings(profile, &bindings)
                .is_ok()
            {
                bound_profiles.push("/interaction_profiles/valve/index_controller");
            }
        }

        // 3) KHR Simple Controller
        if let Ok(profile) = p("/interaction_profiles/khr/simple_controller") {
            let bindings = [
                xr::Binding::new(&aim_pose, p("/user/hand/left/input/aim/pose")?),
                xr::Binding::new(&aim_pose, p("/user/hand/right/input/aim/pose")?),
                xr::Binding::new(&grip_pose, p("/user/hand/left/input/grip/pose")?),
                xr::Binding::new(&grip_pose, p("/user/hand/right/input/grip/pose")?),
                xr::Binding::new(&select_click, p("/user/hand/left/input/select/click")?),
                xr::Binding::new(&select_click, p("/user/hand/right/input/select/click")?),
                xr::Binding::new(&menu_click, p("/user/hand/left/input/menu/click")?),
                xr::Binding::new(&menu_click, p("/user/hand/right/input/menu/click")?),
                xr::Binding::new(&haptic, p("/user/hand/left/output/haptic")?),
                xr::Binding::new(&haptic, p("/user/hand/right/output/haptic")?),
            ];
            if instance
                .suggest_interaction_profile_bindings(profile, &bindings)
                .is_ok()
            {
                bound_profiles.push("/interaction_profiles/khr/simple_controller");
            }
        }

        session.attach_action_sets(&[&action_set])?;

        let aim_spaces = [
            aim_pose.create_space(session, left_path, xr::Posef::IDENTITY)?,
            aim_pose.create_space(session, right_path, xr::Posef::IDENTITY)?,
        ];
        let grip_spaces = [
            grip_pose.create_space(session, left_path, xr::Posef::IDENTITY)?,
            grip_pose.create_space(session, right_path, xr::Posef::IDENTITY)?,
        ];

        Ok(Self {
            action_set,
            subactions,
            aim_pose,
            grip_pose,
            thumbstick,
            thumbstick_click,
            trigger,
            grip,
            primary_click,
            secondary_click,
            menu_click,
            select_click,
            haptic,
            aim_spaces,
            grip_spaces,
            bound_profiles,
        })
    }

    /// Synchronizes OpenXR actions and polls controllers + optional `XR_EXT_hand_tracking` joints.
    pub fn poll_input(
        &self,
        session: &xr::Session<xr::D3D12>,
        base_space: &xr::Space,
        predicted_time: xr::Time,
        view_up: Vec3,
        hand_trackers: Option<&[xr::HandTracker; 2]>,
    ) -> Result<XrInputFrame, XrError> {
        session.sync_actions(&[xr::ActiveActionSet::new(&self.action_set)])?;

        let mut frame = XrInputFrame {
            left: self.poll_hand(
                session,
                base_space,
                predicted_time,
                view_up,
                XrHand::Left,
                hand_trackers.map(|t| &t[0]),
            ),
            right: self.poll_hand(
                session,
                base_space,
                predicted_time,
                view_up,
                XrHand::Right,
                hand_trackers.map(|t| &t[1]),
            ),
            ..Default::default()
        };
        frame.update_two_hand_pinch();
        Ok(frame)
    }

    fn poll_hand(
        &self,
        session: &xr::Session<xr::D3D12>,
        base_space: &xr::Space,
        predicted_time: xr::Time,
        view_up: Vec3,
        hand: XrHand,
        hand_tracker: Option<&xr::HandTracker>,
    ) -> XrHandState {
        let idx = hand.index();
        let path = self.subactions[idx];
        let mut out = XrHandState::default();

        if self.aim_pose.is_active(session, path).unwrap_or(false)
            && let Ok(loc) = self.aim_spaces[idx].locate(base_space, predicted_time)
        {
            let valid = loc.location_flags.contains(
                xr::SpaceLocationFlags::POSITION_VALID | xr::SpaceLocationFlags::ORIENTATION_VALID,
            );
            out.aim_pose = XrPose::from_openxr(loc.pose, valid);
        }

        if self.grip_pose.is_active(session, path).unwrap_or(false)
            && let Ok(loc) = self.grip_spaces[idx].locate(base_space, predicted_time)
        {
            let valid = loc.location_flags.contains(
                xr::SpaceLocationFlags::POSITION_VALID | xr::SpaceLocationFlags::ORIENTATION_VALID,
            );
            out.grip_pose = XrPose::from_openxr(loc.pose, valid);
        }

        if let Ok(st) = self.thumbstick.state(session, path)
            && st.is_active
        {
            out.thumbstick = Vec2::new(st.current_state.x, st.current_state.y);
        }
        if let Ok(st) = self.thumbstick_click.state(session, path)
            && st.is_active
        {
            out.thumbstick_click = st.current_state;
        }
        if let Ok(st) = self.trigger.state(session, path)
            && st.is_active
        {
            out.trigger = st.current_state.clamp(0.0, 1.0);
        }
        if let Ok(st) = self.grip.state(session, path)
            && st.is_active
        {
            out.grip = st.current_state.clamp(0.0, 1.0);
        }
        if let Ok(st) = self.primary_click.state(session, path)
            && st.is_active
        {
            out.primary_click = st.current_state;
        }
        if let Ok(st) = self.secondary_click.state(session, path)
            && st.is_active
        {
            out.secondary_click = st.current_state;
        }
        if let Ok(st) = self.menu_click.state(session, path)
            && st.is_active
        {
            out.menu_click = st.current_state;
        }
        if let Ok(st) = self.select_click.state(session, path)
            && st.is_active
        {
            out.select_click = st.current_state;
        }

        // Controller pinch & palm orientation estimate
        out.pinch_strength = out.trigger.max(if out.select_click { 1.0 } else { 0.0 });
        out.is_pinching = out.pinch_strength >= 0.75;
        if out.grip_pose.valid {
            // Standard grip space has +X/+Y depending on chirality; controller palm-up check uses +Y
            // unless overridden by EXT_hand_tracking joint data below.
            let up = if view_up.length_squared() > 1e-8 {
                view_up.normalize()
            } else {
                Vec3::Y
            };
            out.palm_up_score = (out.grip_pose.orientation * Vec3::Y)
                .dot(up)
                .clamp(-1.0, 1.0);
        }

        // EXT_hand_tracking fallback & enrichment when controller idle or hands tracked
        if let Some(tracker) = hand_tracker
            && let Ok(Some(raw_joints)) = base_space.locate_hand_joints(tracker, predicted_time)
        {
            let mut joints = [HandJointSample::default(); xr::HAND_JOINT_COUNT];
            for (dst, src) in joints.iter_mut().zip(raw_joints.iter()) {
                let valid = src.location_flags.contains(
                    xr::SpaceLocationFlags::POSITION_VALID
                        | xr::SpaceLocationFlags::ORIENTATION_VALID,
                );
                let pose = XrPose::from_openxr(src.pose, valid);
                *dst = HandJointSample {
                    position: pose.position,
                    orientation: pose.orientation,
                    radius: src.radius,
                    valid,
                };
            }
            let (hand_pinch, hand_pinching, pinch_mid, palm_score) =
                extract_pinch_and_palm_from_joints(&joints, view_up);
            out.palm_up_score = palm_score;

            let snap = HandJointSnapshot {
                joints,
                pinch_strength: hand_pinch,
                is_pinching: hand_pinching,
                pinch_midpoint: pinch_mid,
                palm_up_score: palm_score,
            };
            out.hand_joints = Some(snap);

            if !out.grip_pose.valid {
                let palm = joints[xr::HandJoint::PALM.into_raw() as usize];
                if palm.valid {
                    out.grip_pose = XrPose {
                        position: palm.position,
                        orientation: palm.orientation,
                        valid: true,
                    };
                }
            }
            if !out.aim_pose.valid {
                let index_prox = joints[xr::HandJoint::INDEX_PROXIMAL.into_raw() as usize];
                if index_prox.valid {
                    out.aim_pose = XrPose {
                        position: if snap.pinch_strength > 0.25 {
                            snap.pinch_midpoint
                        } else {
                            index_prox.position
                        },
                        orientation: index_prox.orientation,
                        valid: true,
                    };
                }
            }
            if out.trigger <= 1e-4 {
                out.pinch_strength = hand_pinch;
                out.is_pinching = hand_pinching;
            }
        }

        out
    }

    /// Fires an OpenXR controller haptic pulse (for Wrist Resonance Ring bead ticks and dwell lock).
    pub fn apply_haptic(
        &self,
        session: &xr::Session<xr::D3D12>,
        hand: XrHand,
        amplitude: f32,
        duration_ns: i64,
        freq_hz: f32,
    ) -> Result<(), XrError> {
        let vib = xr::HapticVibration::new()
            .amplitude(amplitude.clamp(0.0, 1.0))
            .duration(xr::Duration::from_nanos(duration_ns.max(1)))
            .frequency(freq_hz.max(0.0));
        self.haptic
            .apply_feedback(session, self.subactions[hand.index()], &vib)?;
        Ok(())
    }
}
