use cosmos_math::{ScalePose64, SectorTree};
use cosmos_nav::{exponential_tau_damp_dvec3, exponential_tau_damp_f64};
use glam::{DQuat, DVec3, Vec3};
use serde::{Deserialize, Serialize};

/// Top-level playback state of the autonomous Story-Journey director.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum JourneyPlayState {
    #[default]
    Inactive,
    Playing,
    Paused,
    PausedByLeash,
    Completed,
}

/// Control inputs harvested ONLY from intentional user override axes (never HMD head-tracking).
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ManualOverrideInput {
    /// Local lateral/vertical/forward translation push from thumbstick / WASD (`-1..=1`).
    pub move_local: Vec3,
    /// Yaw rate push in radians/sec (right stick / mouse look drag / A-D rotation).
    pub yaw_delta_rad: f64,
    /// Pitch rate push in radians/sec.
    pub pitch_delta_rad: f64,
    /// Logarithmic scale zoom rate push in `du/sec` (scroll wheel / pinch / trigger zoom).
    pub scale_delta_du: f64,
}

impl ManualOverrideInput {
    #[inline]
    pub fn has_input(&self) -> bool {
        self.move_local.length_squared() > 1e-6
            || self.yaw_delta_rad.abs() > 1e-6
            || self.pitch_delta_rad.abs() > 1e-6
            || self.scale_delta_du.abs() > 1e-6
    }
}

/// Tuning constants for the elastic camera tether and non-occluding peripheral exit prompt.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ElasticOverrideConfig {
    pub move_speed_ru_per_sec: f64,
    pub scale_speed_du_per_sec: f64,
    pub max_pos_offset_ru: f64,
    pub max_yaw_offset_rad: f64,
    pub max_pitch_offset_rad: f64,
    pub max_scale_offset_du: f64,
    /// Spring decay time constant (`tau` sec) while user is idle or returning to ghost rail.
    pub rejoin_tau_sec: f64,
    /// Softer magnetic pull time constant while actively exploring within leash.
    pub active_pull_tau_sec: f64,
    /// Offsets exceeding these bounds for `leash_hold_sec` pause the ghost rail and show exit pearls.
    pub leash_dist_ru: f64,
    pub leash_angle_rad: f64,
    pub leash_scale_du: f64,
    pub leash_hold_sec: f32,
}

impl Default for ElasticOverrideConfig {
    fn default() -> Self {
        Self {
            move_speed_ru_per_sec: 2.2,
            scale_speed_du_per_sec: 1.4,
            max_pos_offset_ru: 3.6,
            max_yaw_offset_rad: 95.0_f64.to_radians(),
            max_pitch_offset_rad: 68.0_f64.to_radians(),
            max_scale_offset_du: 2.5,
            rejoin_tau_sec: 0.42,
            active_pull_tau_sec: 3.6,
            leash_dist_ru: 1.25,
            leash_angle_rad: 36.0_f64.to_radians(),
            leash_scale_du: 0.75,
            leash_hold_sec: 1.15,
        }
    }
}

/// Selection outcome when aiming/clicking on the peripheral `FOLLOW` or `EXIT` pearls.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum PeripheralChoice {
    Follow,
    Exit,
}

/// World-space geometry for the twin non-occluding 3D peripheral pearls rendered outside the 12° fovea.
#[derive(Clone, Debug, PartialEq)]
pub struct PeripheralPromptVisual {
    pub follow_world: Vec3,
    pub exit_world: Vec3,
    pub ghost_world: Vec3,
    pub hovered: Option<PeripheralChoice>,
    pub radius_m: f32,
    pub alpha: f32,
}

/// Elastic offset state composing manual thumbstick/keyboard exploration on top of `ghost_pose`.
///
/// Crucially, VR HMD head rotation and 6-DoF roomscale translation operate on the downstream
/// headset pose and NEVER mutate `ElasticOverrideState`, preserving effortless look-around freedom.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ElasticOverrideState {
    pub config: ElasticOverrideConfig,
    /// Offset in `ghost_pose` local render-unit space (`+X` right, `+Y` up, `+Z` back).
    pub offset_local_ru: DVec3,
    pub offset_yaw_rad: f64,
    pub offset_pitch_rad: f64,
    pub offset_scale_du: f64,
    pub leash_exceeded_sec: f32,
    pub rejoin_active: bool,
    pub prompt_alpha: f32,
    pub hovered_choice: Option<PeripheralChoice>,
}

impl ElasticOverrideState {
    pub fn reset(&mut self) {
        self.offset_local_ru = DVec3::ZERO;
        self.offset_yaw_rad = 0.0;
        self.offset_pitch_rad = 0.0;
        self.offset_scale_du = 0.0;
        self.leash_exceeded_sec = 0.0;
        self.rejoin_active = false;
        self.prompt_alpha = 0.0;
        self.hovered_choice = None;
    }

    pub fn snap_to_follow(&mut self) {
        self.rejoin_active = true;
        self.leash_exceeded_sec = 0.0;
        self.hovered_choice = None;
    }

    #[inline]
    pub fn is_zero(&self) -> bool {
        self.offset_local_ru.length() < 1e-4
            && self.offset_yaw_rad.abs() < 1e-4
            && self.offset_pitch_rad.abs() < 1e-4
            && self.offset_scale_du.abs() < 1e-4
    }

    /// Evaluates whether the user has stretched far enough past the ghost rail to summon exit pearls.
    #[inline]
    pub fn exceeds_leash(&self) -> bool {
        let ang = (self.offset_yaw_rad.powi(2) + self.offset_pitch_rad.powi(2)).sqrt();
        self.offset_local_ru.length() >= self.config.leash_dist_ru
            || ang >= self.config.leash_angle_rad
            || self.offset_scale_du.abs() >= self.config.leash_scale_du
    }

    /// Steps manual overrides, magnetic spring return, and leash escalation.
    pub fn step(
        &mut self,
        input: &ManualOverrideInput,
        dt_sec: f32,
        play_state: &mut JourneyPlayState,
    ) {
        let dt = dt_sec.max(0.0) as f64;
        if dt <= 0.0 {
            return;
        }

        if input.has_input() {
            self.rejoin_active = false;
            self.offset_local_ru += DVec3::new(
                input.move_local.x as f64,
                input.move_local.y as f64,
                input.move_local.z as f64,
            ) * (self.config.move_speed_ru_per_sec * dt);
            self.offset_yaw_rad += input.yaw_delta_rad;
            self.offset_pitch_rad += input.pitch_delta_rad;
            self.offset_scale_du += input.scale_delta_du * self.config.scale_speed_du_per_sec;
        }

        // Clamp offsets to safe bounded orbital cone around ghost rail.
        let dist = self.offset_local_ru.length();
        if dist > self.config.max_pos_offset_ru {
            self.offset_local_ru *= self.config.max_pos_offset_ru / dist;
        }
        self.offset_yaw_rad = self
            .offset_yaw_rad
            .clamp(-self.config.max_yaw_offset_rad, self.config.max_yaw_offset_rad);
        self.offset_pitch_rad = self
            .offset_pitch_rad
            .clamp(-self.config.max_pitch_offset_rad, self.config.max_pitch_offset_rad);
        self.offset_scale_du = self
            .offset_scale_du
            .clamp(-self.config.max_scale_offset_du, self.config.max_scale_offset_du);

        // Apply magnetic pull spring unless user is parked in PausedByLeash actively reading/choosing.
        if *play_state == JourneyPlayState::Playing || self.rejoin_active {
            let tau = if self.rejoin_active || !input.has_input() {
                self.config.rejoin_tau_sec
            } else {
                self.config.active_pull_tau_sec
            };
            self.offset_local_ru = exponential_tau_damp_dvec3(
                self.offset_local_ru,
                DVec3::ZERO,
                tau,
                dt,
            );
            self.offset_yaw_rad = exponential_tau_damp_f64(self.offset_yaw_rad, 0.0, tau, dt);
            self.offset_pitch_rad = exponential_tau_damp_f64(self.offset_pitch_rad, 0.0, tau, dt);
            self.offset_scale_du = exponential_tau_damp_f64(self.offset_scale_du, 0.0, tau, dt);
            if self.is_zero() {
                self.offset_local_ru = DVec3::ZERO;
                self.offset_yaw_rad = 0.0;
                self.offset_pitch_rad = 0.0;
                self.offset_scale_du = 0.0;
                self.rejoin_active = false;
            }
        }

        // Leash escalation / fade timing.
        if *play_state == JourneyPlayState::Playing && self.exceeds_leash() && input.has_input() {
            self.leash_exceeded_sec += dt_sec.max(0.0);
            if self.leash_exceeded_sec >= self.config.leash_hold_sec {
                *play_state = JourneyPlayState::PausedByLeash;
            }
        } else if *play_state != JourneyPlayState::PausedByLeash {
            self.leash_exceeded_sec = (self.leash_exceeded_sec - dt_sec.max(0.0) * 1.5).max(0.0);
        }

        let target_prompt = if *play_state == JourneyPlayState::PausedByLeash {
            1.0
        } else {
            0.0
        };
        self.prompt_alpha = exponential_tau_damp_f64(
            self.prompt_alpha as f64,
            target_prompt,
            0.12,
            dt,
        ) as f32;
        if target_prompt == 0.0 && self.prompt_alpha < 0.01 {
            self.prompt_alpha = 0.0;
        }
    }

    /// Composes `Pose_cam = Pose_ghost ⊕ ΔPose_user` cleanly in double-precision logarithmic space.
    pub fn compose_camera_pose(&self, tree: &SectorTree, ghost_pose: &ScalePose64) -> ScalePose64 {
        let rot_offset =
            DQuat::from_rotation_y(self.offset_yaw_rad) * DQuat::from_rotation_x(self.offset_pitch_rad);
        let composed_orient = (ghost_pose.orient * rot_offset).normalize();
        let world_delta_ru = ghost_pose.orient * self.offset_local_ru;
        let mut out = *ghost_pose;
        let _ = out.translate_render_units(world_delta_ru, tree);
        out.orient = composed_orient;
        out.scale_exp = (ghost_pose.scale_exp + self.offset_scale_du).clamp(-36.5, 29.0);
        out
    }

    /// Computes non-occluding 3D peripheral pearls in the lower-right outer quadrant (`>24°` off-axis,
    /// leaving the central `12°` fovea completely unobstructed) and tests pointer hover/selection.
    #[allow(clippy::too_many_arguments)]
pub fn evaluate_peripheral_prompt(
        &mut self,
        play_state: JourneyPlayState,
        cam_origin: Vec3,
        cam_rot: glam::Quat,
        ghost_relative_pos: Vec3,
        pointer_origin: Option<Vec3>,
        pointer_dir: Option<Vec3>,
        pointer_triggered: bool,
    ) -> (Option<PeripheralPromptVisual>, Option<PeripheralChoice>) {
        if play_state != JourneyPlayState::PausedByLeash && self.prompt_alpha <= 0.02 {
            self.hovered_choice = None;
            return (None, None);
        }

        // Fixed angular placement in camera basis (`+X` right, `+Y` up, `-Z` forward):
        // FOLLOW: az=+16.0°, el=-19.0° (24.6° off optical axis, strictly outside 12° foveal cone)
        // EXIT:   az=+22.5°, el=-20.5° (30.1° off optical axis)
        let follow_dir = spherical_cam_dir(16.0_f32.to_radians(), -19.0_f32.to_radians());
        let exit_dir = spherical_cam_dir(22.5_f32.to_radians(), -20.5_f32.to_radians());
        let dist_m = 1.85_f32;
        let follow_world = cam_origin + cam_rot * (follow_dir * dist_m);
        let exit_world = cam_origin + cam_rot * (exit_dir * dist_m);
        let radius_m = 0.026 * self.prompt_alpha.max(0.25);

        let mut hovered = None;
        if let (Some(orig), Some(dir)) = (pointer_origin, pointer_dir) {
            let dir_n = dir.normalize_or_zero();
            if dir_n.length_squared() > 0.5 {
                let mut best_dist = f32::MAX;
                for (choice, pos) in [
                    (PeripheralChoice::Follow, follow_world),
                    (PeripheralChoice::Exit, exit_world),
                ] {
                    let to_p = pos - orig;
                    let proj = to_p.dot(dir_n);
                    if proj > 0.0 {
                        let perp = (to_p - dir_n * proj).length();
                        if perp <= (radius_m * 2.2).max(0.045) && perp < best_dist {
                            best_dist = perp;
                            hovered = Some(choice);
                        }
                    }
                }
            }
        }
        self.hovered_choice = hovered;

        let chosen = if pointer_triggered && play_state == JourneyPlayState::PausedByLeash {
            hovered
        } else {
            None
        };

        let visual = PeripheralPromptVisual {
            follow_world,
            exit_world,
            ghost_world: cam_origin + ghost_relative_pos,
            hovered,
            radius_m,
            alpha: self.prompt_alpha,
        };
        (Some(visual), chosen)
    }
}

#[inline]
fn spherical_cam_dir(az_rad: f32, el_rad: f32) -> Vec3 {
    let cos_el = el_rad.cos();
    Vec3::new(
        az_rad.sin() * cos_el,
        el_rad.sin(),
        -az_rad.cos() * cos_el,
    )
}
