use glam::Vec3;
use serde::{Deserialize, Serialize};

/// Spatial candidate visible inside the current camera frustum or near VR hand ray.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FocusCandidate {
    pub entity_id: u64,
    /// Unit direction vector from camera (or pointer origin) toward candidate center in camera space.
    pub dir_cam: Vec3,
    /// Apparent angular radius of the entity in degrees (`0` for point targets).
    pub angular_radius_deg: f32,
    pub salience_tier: f32,
    pub visibility_weight: f32,
    pub experience_id: u8,
    pub zone_index: u16,
    pub feature_index: u16,
}

impl FocusCandidate {
    /// Scores focus preference `S = salience_tier * exp(-(theta / cone)^2) * visibility_weight`.
    #[inline]
    pub fn score(&self, theta_rad: f32, cone_rad: f32) -> f32 {
        let ratio = theta_rad / cone_rad.max(1e-5);
        self.salience_tier.max(0.0) * (-(ratio * ratio)).exp() * self.visibility_weight.max(0.0)
    }
}

/// Snapshot of the currently tracked / locked focus target.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TargetIdentity {
    pub entity_id: u64,
    pub experience_id: u8,
    pub zone_index: u16,
    pub feature_index: u16,
}

impl From<&FocusCandidate> for TargetIdentity {
    #[inline]
    fn from(c: &FocusCandidate) -> Self {
        Self {
            entity_id: c.entity_id,
            experience_id: c.experience_id,
            zone_index: c.zone_index,
            feature_index: c.feature_index,
        }
    }
}

/// Exact Rest-to-Learn FSM state progression: `Moving -> Settling (0..200ms) -> Locked (200..900ms) -> Narrating -> Complete`.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum FsmState {
    Moving,
    Settling {
        elapsed_ms: f32,
        target: TargetIdentity,
    },
    Locked {
        elapsed_ms: f32,
        target: TargetIdentity,
    },
    Narrating {
        target: TargetIdentity,
        active_ms: f32,
    },
    Complete {
        target: TargetIdentity,
    },
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum FsmStateKind {
    Moving,
    Settling,
    Locked,
    Narrating,
    Complete,
}

impl FsmState {
    #[inline]
    pub const fn kind(&self) -> FsmStateKind {
        match self {
            Self::Moving => FsmStateKind::Moving,
            Self::Settling { .. } => FsmStateKind::Settling,
            Self::Locked { .. } => FsmStateKind::Locked,
            Self::Narrating { .. } => FsmStateKind::Narrating,
            Self::Complete { .. } => FsmStateKind::Complete,
        }
    }

    #[inline]
    pub const fn target(&self) -> Option<TargetIdentity> {
        match *self {
            Self::Moving => None,
            Self::Settling { target, .. }
            | Self::Locked { target, .. }
            | Self::Narrating { target, .. }
            | Self::Complete { target } => Some(target),
        }
    }
}

/// Physiological tremor tolerance, cone hysteresis, and Rest-to-Learn dwell timing configuration.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FsmConfig {
    /// Low-pass filter time constant `tau` for pointer/gaze ray smoothing (`80 ms` default).
    pub ray_tau_sec: f32,
    /// Minimum entry selection cone half-angle (`2.4 deg`).
    pub entry_cone_deg: f32,
    /// Exit hysteresis multiplier on entry cone (`1.35x` -> `3.24 deg` on `2.4 deg` entry cone).
    pub exit_cone_multiplier: f32,
    /// Relative score margin (`1.25x`) required for a competing candidate to steal focus.
    pub candidate_switch_margin: f32,
    /// Upper boundary of `Settling` phase (`200 ms`).
    pub settling_end_ms: f32,
    /// Total stillness dwell required before entering `Narrating` (`900 ms`).
    pub lock_dwell_ms: f32,
    /// Audio/glow cancellation fadeout duration (`120 ms`).
    pub cancel_fade_ms: f32,
    /// Linear speed ceiling (render units / s) to enter rest.
    pub enter_linear_speed: f32,
    /// Linear speed threshold (render units / s) that breaks rest once settled.
    pub exit_linear_speed: f32,
    /// Scale speed `|du/dt|` ceiling to enter rest.
    pub enter_scale_speed: f32,
    /// Scale speed `|du/dt|` threshold that breaks rest once settled.
    pub exit_scale_speed: f32,
    /// Smoothed ray angular velocity ceiling (deg / s) to enter rest.
    pub enter_ray_angular_deg_per_sec: f32,
    /// Smoothed ray angular velocity threshold (deg / s) that breaks rest once settled.
    pub exit_ray_angular_deg_per_sec: f32,
}

impl Default for FsmConfig {
    fn default() -> Self {
        Self {
            ray_tau_sec: 0.080,
            entry_cone_deg: 2.4,
            exit_cone_multiplier: 1.35,
            candidate_switch_margin: 1.25,
            settling_end_ms: 200.0,
            lock_dwell_ms: 900.0,
            cancel_fade_ms: 120.0,
            enter_linear_speed: 0.06,
            exit_linear_speed: 0.09,
            enter_scale_speed: 0.08,
            exit_scale_speed: 0.12,
            enter_ray_angular_deg_per_sec: 3.5,
            exit_ray_angular_deg_per_sec: 5.5,
        }
    }
}

/// Per-frame kinematic & audio completion inputs fed into [`RestNarrationFsm::step`].
#[derive(Clone, Debug, PartialEq)]
pub struct FsmInput<'a> {
    pub raw_ray_dir_cam: Vec3,
    pub linear_speed: f32,
    pub scale_speed: f32,
    pub dt_sec: f32,
    pub candidates: &'a [FocusCandidate],
    /// Set `true` by the audio mixer on the frame the current target finishes speaking all pairs.
    pub narration_finished: bool,
}

/// Frame output emitted by [`RestNarrationFsm::step`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FsmOutput {
    pub state: FsmState,
    /// Normalized dwell progress `[0.0 .. 1.0]` written into `FrameUniforms.focus_ray.w`.
    pub dwell_progress: f32,
    /// Tremor-filtered ray direction (`tau = 80ms`) in camera-local space.
    pub smoothed_ray_dir_cam: Vec3,
    /// Instantaneous smoothed ray angular velocity in degrees / second.
    pub ray_angular_deg_per_sec: f32,
    /// Target that just crossed the 900ms threshold and entered `Narrating` this frame.
    pub start_narration_for: Option<TargetIdentity>,
    /// True when movement or leaving cone cancelled an active `Narrating` (or `Locked`) session.
    pub should_cancel_audio: bool,
    /// Remaining fadeout time in milliseconds (`0 ..= 120.0`).
    pub cancel_fade_remaining_ms: f32,
    /// Normalized fadeout alpha (`1.0` at cancel moment decaying linearly to `0.0` over `120 ms`).
    pub cancel_fade_alpha: f32,
}

/// Rest-to-Learn Velocity-Stop Narration Finite State Machine with physiological tremor tolerance.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RestNarrationFsm {
    pub config: FsmConfig,
    pub state: FsmState,
    ray_initialized: bool,
    smoothed_ray: Vec3,
    ray_angular_deg_per_sec: f32,
    cancel_fade_remaining_ms: f32,
    current_entry_cone_deg: f32,
}

impl Default for RestNarrationFsm {
    fn default() -> Self {
        Self::new(FsmConfig::default())
    }
}

impl RestNarrationFsm {
    pub fn new(config: FsmConfig) -> Self {
        Self {
            config,
            state: FsmState::Moving,
            ray_initialized: false,
            smoothed_ray: Vec3::NEG_Z,
            ray_angular_deg_per_sec: 0.0,
            cancel_fade_remaining_ms: 0.0,
            current_entry_cone_deg: 2.4,
        }
    }

    #[inline]
    pub fn smoothed_ray(&self) -> Vec3 {
        self.smoothed_ray
    }

    /// Advances tremor ray filtering, hysteresis lock checks, 0->200->900ms dwell, and 120ms cancel fade.
    pub fn step(&mut self, input: &FsmInput<'_>) -> FsmOutput {
        let dt_sec = input.dt_sec.max(0.0);
        let dt_ms = dt_sec * 1000.0;

        // Tick active 120ms cancellation fade.
        if self.cancel_fade_remaining_ms > 0.0 {
            self.cancel_fade_remaining_ms = (self.cancel_fade_remaining_ms - dt_ms).max(0.0);
        }

        // Low-pass filter focus ray (`tau = 80ms`) & measure angular jitter speed.
        let raw_dir = if input.raw_ray_dir_cam.length_squared() > 1e-8 {
            input.raw_ray_dir_cam.normalize()
        } else {
            Vec3::NEG_Z
        };
        if !self.ray_initialized {
            self.smoothed_ray = raw_dir;
            self.ray_initialized = true;
            self.ray_angular_deg_per_sec = 0.0;
        } else {
            let prev_ray = self.smoothed_ray;
            let alpha = if dt_sec > 0.0 && self.config.ray_tau_sec > 0.0 {
                1.0 - (-dt_sec / self.config.ray_tau_sec).exp()
            } else {
                1.0
            };
            self.smoothed_ray = (prev_ray + (raw_dir - prev_ray) * alpha).normalize_or_zero();
            if self.smoothed_ray.length_squared() < 0.5 {
                self.smoothed_ray = raw_dir;
            }
            let cos_step = prev_ray.dot(self.smoothed_ray).clamp(-1.0, 1.0);
            let step_deg = cos_step.acos().to_degrees();
            self.ray_angular_deg_per_sec = if dt_sec > 1e-6 {
                step_deg / dt_sec
            } else {
                0.0
            };
        }

        let in_dwell_or_active = self.state.kind() != FsmStateKind::Moving;
        let is_still = if in_dwell_or_active {
            input.linear_speed < self.config.exit_linear_speed
                && input.scale_speed.abs() < self.config.exit_scale_speed
                && self.ray_angular_deg_per_sec < self.config.exit_ray_angular_deg_per_sec
        } else {
            input.linear_speed < self.config.enter_linear_speed
                && input.scale_speed.abs() < self.config.enter_scale_speed
                && self.ray_angular_deg_per_sec < self.config.enter_ray_angular_deg_per_sec
        };

        let mut start_narration_for = None;
        let mut should_cancel_audio = false;

        if !is_still {
            if matches!(
                self.state,
                FsmState::Locked { .. } | FsmState::Narrating { .. }
            ) {
                should_cancel_audio = matches!(self.state, FsmState::Narrating { .. });
                self.cancel_fade_remaining_ms = self.config.cancel_fade_ms;
            }
            self.state = FsmState::Moving;
            return self.build_output(start_narration_for, should_cancel_audio);
        }

        // Evaluate candidate lock / switch hysteresis (`1.35x` exit cone, `1.25x` switch margin).
        let current_target = self.state.target();
        let current_match = current_target.and_then(|cur| {
            input
                .candidates
                .iter()
                .find(|c| TargetIdentity::from(*c) == cur)
                .and_then(|c| {
                    let theta = ray_angle_rad(self.smoothed_ray, c.dir_cam);
                    let entry_deg = self.config.entry_cone_deg.max(c.angular_radius_deg);
                    let exit_rad = (entry_deg * self.config.exit_cone_multiplier).to_radians();
                    if theta <= exit_rad {
                        let score = c.score(theta, entry_deg.to_radians());
                        Some((c, entry_deg, score))
                    } else {
                        None
                    }
                })
        });

        let best_entry = input
            .candidates
            .iter()
            .filter_map(|c| {
                let theta = ray_angle_rad(self.smoothed_ray, c.dir_cam);
                let entry_deg = self.config.entry_cone_deg.max(c.angular_radius_deg);
                let entry_rad = entry_deg.to_radians();
                if theta <= entry_rad {
                    Some((c, entry_deg, c.score(theta, entry_rad)))
                } else {
                    None
                }
            })
            .max_by(|a, b| a.2.total_cmp(&b.2));

        let chosen = match (current_match, best_entry) {
            (Some((cur_c, cur_cone, cur_score)), Some((alt_c, alt_cone, alt_score))) => {
                if TargetIdentity::from(alt_c) != TargetIdentity::from(cur_c)
                    && alt_score >= cur_score * self.config.candidate_switch_margin
                {
                    Some((TargetIdentity::from(alt_c), alt_cone, true))
                } else {
                    Some((TargetIdentity::from(cur_c), cur_cone, false))
                }
            }
            (Some((cur_c, cur_cone, _)), None) => {
                Some((TargetIdentity::from(cur_c), cur_cone, false))
            }
            (None, Some((alt_c, alt_cone, _))) => {
                Some((TargetIdentity::from(alt_c), alt_cone, current_target.is_some()))
            }
            (None, None) => None,
        };

        let Some((target, cone_deg, target_changed)) = chosen else {
            // Left exit cone (`1.35x`) or no candidate present.
            if matches!(
                self.state,
                FsmState::Locked { .. } | FsmState::Narrating { .. }
            ) {
                should_cancel_audio = matches!(self.state, FsmState::Narrating { .. });
                self.cancel_fade_remaining_ms = self.config.cancel_fade_ms;
            }
            self.state = FsmState::Moving;
            return self.build_output(start_narration_for, should_cancel_audio);
        };

        self.current_entry_cone_deg = cone_deg;

        if target_changed {
            if matches!(
                self.state,
                FsmState::Locked { .. } | FsmState::Narrating { .. }
            ) {
                should_cancel_audio = matches!(self.state, FsmState::Narrating { .. });
                self.cancel_fade_remaining_ms = self.config.cancel_fade_ms;
            }
            self.state = FsmState::Settling {
                elapsed_ms: dt_ms.min(self.config.settling_end_ms),
                target,
            };
        }

        // Advance current state timeline cleanly across 0..200ms -> 200..900ms -> Narrating -> Complete.
        if !target_changed {
            match self.state {
                FsmState::Moving => {
                    let next_ms = dt_ms;
                    self.state = Self::dwell_state_for_ms(target, next_ms, &self.config, &mut start_narration_for);
                }
                FsmState::Settling { elapsed_ms, target: t } => {
                    let next_ms = elapsed_ms + dt_ms;
                    self.state = Self::dwell_state_for_ms(t, next_ms, &self.config, &mut start_narration_for);
                }
                FsmState::Locked { elapsed_ms, target: t } => {
                    let next_ms = elapsed_ms + dt_ms;
                    self.state = Self::dwell_state_for_ms(t, next_ms, &self.config, &mut start_narration_for);
                }
                FsmState::Narrating { target: t, active_ms } => {
                    if input.narration_finished {
                        self.state = FsmState::Complete { target: t };
                    } else {
                        self.state = FsmState::Narrating {
                            target: t,
                            active_ms: active_ms + dt_ms,
                        };
                    }
                }
                FsmState::Complete { target: t } => {
                    self.state = FsmState::Complete { target: t };
                }
            }
        }

        self.build_output(start_narration_for, should_cancel_audio)
    }

    fn dwell_state_for_ms(
        target: TargetIdentity,
        elapsed_ms: f32,
        config: &FsmConfig,
        start_narration_for: &mut Option<TargetIdentity>,
    ) -> FsmState {
        if elapsed_ms < config.settling_end_ms {
            FsmState::Settling { elapsed_ms, target }
        } else if elapsed_ms < config.lock_dwell_ms {
            FsmState::Locked { elapsed_ms, target }
        } else {
            *start_narration_for = Some(target);
            FsmState::Narrating {
                target,
                active_ms: elapsed_ms - config.lock_dwell_ms,
            }
        }
    }

    fn build_output(
        &self,
        start_narration_for: Option<TargetIdentity>,
        should_cancel_audio: bool,
    ) -> FsmOutput {
        let lock_ms = self.config.lock_dwell_ms.max(1.0);
        let dwell_progress = match self.state {
            FsmState::Moving => 0.0,
            FsmState::Settling { elapsed_ms, .. } | FsmState::Locked { elapsed_ms, .. } => {
                (elapsed_ms / lock_ms).clamp(0.0, 1.0)
            }
            FsmState::Narrating { .. } | FsmState::Complete { .. } => 1.0,
        };
        let fade_alpha = if self.config.cancel_fade_ms > 0.0 {
            (self.cancel_fade_remaining_ms / self.config.cancel_fade_ms).clamp(0.0, 1.0)
        } else {
            0.0
        };

        FsmOutput {
            state: self.state,
            dwell_progress,
            smoothed_ray_dir_cam: self.smoothed_ray,
            ray_angular_deg_per_sec: self.ray_angular_deg_per_sec,
            start_narration_for,
            should_cancel_audio,
            cancel_fade_remaining_ms: self.cancel_fade_remaining_ms,
            cancel_fade_alpha: fade_alpha,
        }
    }
}

#[inline]
fn ray_angle_rad(ray: Vec3, dir: Vec3) -> f32 {
    let d = dir.normalize_or_zero();
    if d.length_squared() < 0.5 {
        return std::f32::consts::PI;
    }
    ray.normalize_or_zero().dot(d).clamp(-1.0, 1.0).acos()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn candidate(id: u64, yaw_deg: f32, salience: f32) -> FocusCandidate {
        let rad = yaw_deg.to_radians();
        FocusCandidate {
            entity_id: id,
            dir_cam: Vec3::new(rad.sin(), 0.0, -rad.cos()),
            angular_radius_deg: 0.5,
            salience_tier: salience,
            visibility_weight: 1.0,
            experience_id: 4,
            zone_index: 2,
            feature_index: id as u16,
        }
    }

    #[test]
    fn dwell_transitions_cleanly_from_moving_through_settling_locked_narrating_complete() {
        let mut fsm = RestNarrationFsm::default();
        let c = [candidate(10, 0.0, 1.5)];

        // Moving resets dwell to zero.
        let out_m = fsm.step(&FsmInput {
            raw_ray_dir_cam: Vec3::NEG_Z,
            linear_speed: 0.4,
            scale_speed: 0.0,
            dt_sec: 0.05,
            candidates: &c,
            narration_finished: false,
        });
        assert_eq!(out_m.state.kind(), FsmStateKind::Moving);
        assert_eq!(out_m.dwell_progress, 0.0);

        // 0..200ms -> Settling
        let out_s = fsm.step(&FsmInput {
            raw_ray_dir_cam: Vec3::NEG_Z,
            linear_speed: 0.01,
            scale_speed: 0.0,
            dt_sec: 0.100,
            candidates: &c,
            narration_finished: false,
        });
        assert_eq!(out_s.state.kind(), FsmStateKind::Settling);
        assert!((out_s.dwell_progress - 100.0 / 900.0).abs() < 1e-4);

        // Cross 200ms -> Locked
        let out_l = fsm.step(&FsmInput {
            raw_ray_dir_cam: Vec3::NEG_Z,
            linear_speed: 0.01,
            scale_speed: 0.0,
            dt_sec: 0.200,
            candidates: &c,
            narration_finished: false,
        });
        assert_eq!(out_l.state.kind(), FsmStateKind::Locked);
        assert!((out_l.dwell_progress - 300.0 / 900.0).abs() < 1e-4);

        // Reach 900ms -> Narrating trigger!
        let out_n = fsm.step(&FsmInput {
            raw_ray_dir_cam: Vec3::NEG_Z,
            linear_speed: 0.01,
            scale_speed: 0.0,
            dt_sec: 0.600,
            candidates: &c,
            narration_finished: false,
        });
        assert_eq!(out_n.state.kind(), FsmStateKind::Narrating);
        assert_eq!(out_n.dwell_progress, 1.0);
        assert_eq!(out_n.start_narration_for.map(|t| t.entity_id), Some(10));

        // Finish audio -> Complete
        let out_c = fsm.step(&FsmInput {
            raw_ray_dir_cam: Vec3::NEG_Z,
            linear_speed: 0.0,
            scale_speed: 0.0,
            dt_sec: 0.050,
            candidates: &c,
            narration_finished: true,
        });
        assert_eq!(out_c.state.kind(), FsmStateKind::Complete);
        assert_eq!(out_c.dwell_progress, 1.0);
    }

    #[test]
    fn tremor_hysteresis_holds_within_135x_cone_and_120ms_cancels_outside() {
        let mut fsm = RestNarrationFsm::default();
        let c = [candidate(20, 0.0, 1.0)];

        // Lock and reach Narrating at 0 deg.
        for _ in 0..10 {
            fsm.step(&FsmInput {
                raw_ray_dir_cam: Vec3::NEG_Z,
                linear_speed: 0.0,
                scale_speed: 0.0,
                dt_sec: 0.100,
                candidates: &c,
                narration_finished: false,
            });
        }
        assert_eq!(fsm.state.kind(), FsmStateKind::Narrating);

        // Small jitter to 2.9 deg (outside 2.4 deg entry cone, inside 2.4 * 1.35 = 3.24 deg exit cone) holds lock!
        let deg_29 = 2.9_f32.to_radians();
        let out_hold = fsm.step(&FsmInput {
            raw_ray_dir_cam: Vec3::new(deg_29.sin(), 0.0, -deg_29.cos()),
            linear_speed: 0.0,
            scale_speed: 0.0,
            dt_sec: 1.0, // let tau=80ms settle smoothly onto 2.9 deg without spiking speed
            candidates: &c,
            narration_finished: false,
        });
        assert_eq!(out_hold.state.kind(), FsmStateKind::Narrating);
        assert!(!out_hold.should_cancel_audio);

        // Moving outside 1.35x exit cone (4.5 deg) cancels cleanly with 120ms fade!
        let deg_45 = 4.5_f32.to_radians();
        let out_exit = fsm.step(&FsmInput {
            raw_ray_dir_cam: Vec3::new(deg_45.sin(), 0.0, -deg_45.cos()),
            linear_speed: 0.0,
            scale_speed: 0.0,
            dt_sec: 1.0,
            candidates: &c,
            narration_finished: false,
        });
        assert_eq!(out_exit.state.kind(), FsmStateKind::Moving);
        assert!(out_exit.should_cancel_audio);
        assert!((out_exit.cancel_fade_remaining_ms - 120.0).abs() < 1e-4);
        assert!((out_exit.cancel_fade_alpha - 1.0).abs() < 1e-4);

        // 60ms later fade alpha is 0.5; 120ms later it reaches 0.0.
        let out_half = fsm.step(&FsmInput {
            raw_ray_dir_cam: Vec3::new(deg_45.sin(), 0.0, -deg_45.cos()),
            linear_speed: 0.5,
            scale_speed: 0.0,
            dt_sec: 0.060,
            candidates: &c,
            narration_finished: false,
        });
        assert!((out_half.cancel_fade_alpha - 0.5).abs() < 1e-4);

        let out_zero = fsm.step(&FsmInput {
            raw_ray_dir_cam: Vec3::new(deg_45.sin(), 0.0, -deg_45.cos()),
            linear_speed: 0.5,
            scale_speed: 0.0,
            dt_sec: 0.060,
            candidates: &c,
            narration_finished: false,
        });
        assert_eq!(out_zero.cancel_fade_alpha, 0.0);
    }

    #[test]
    fn candidate_switch_requires_125x_score_margin() {
        let mut fsm = RestNarrationFsm::default();
        // First lock onto candidate 1 cleanly.
        let c_initial = [candidate(1, 0.0, 1.0)];
        for _ in 0..4 {
            fsm.step(&FsmInput {
                raw_ray_dir_cam: Vec3::NEG_Z,
                linear_speed: 0.0,
                scale_speed: 0.0,
                dt_sec: 0.100,
                candidates: &c_initial,
                narration_finished: false,
            });
        }
        assert_eq!(fsm.state.target().unwrap().entity_id, 1);

        // Competing candidate 2 has 1.15x (< 1.25x switch margin): must hold candidate 1.
        let c_weak = [candidate(1, 0.0, 1.0), candidate(2, 0.0, 1.15)];
        fsm.step(&FsmInput {
            raw_ray_dir_cam: Vec3::NEG_Z,
            linear_speed: 0.0,
            scale_speed: 0.0,
            dt_sec: 0.050,
            candidates: &c_weak,
            narration_finished: false,
        });
        assert_eq!(fsm.state.target().unwrap().entity_id, 1);

        // Competing candidate 2 rises to 1.40x (>= 1.25x): switches to 2 and resets dwell to Settling.
        let c_strong = [candidate(1, 0.0, 1.0), candidate(2, 0.0, 1.40)];
        let out = fsm.step(&FsmInput {
            raw_ray_dir_cam: Vec3::NEG_Z,
            linear_speed: 0.0,
            scale_speed: 0.0,
            dt_sec: 0.050,
            candidates: &c_strong,
            narration_finished: false,
        });
        assert_eq!(out.state.target().unwrap().entity_id, 2);
        assert_eq!(out.state.kind(), FsmStateKind::Settling);
    }
}
