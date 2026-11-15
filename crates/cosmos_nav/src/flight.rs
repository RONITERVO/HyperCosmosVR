use cosmos_math::{
    MathError, ScalePose64, ScaleSpline, ScaleSplineKeyframe, SectorId,
};
use cosmos_spatial::{CosmosSpatialIndex, VisibleEntity};
use glam::{DQuat, DVec2, DVec3};
use serde::{Deserialize, Serialize};

/// Cold-start camera scale exponent (`u = +6.6`, Earth Low Orbit overlooking `03: Termosfera/Aurora`).
pub const COLD_START_SCALE_EXP: f64 = 6.6;

/// Default camera position in `SectorId::EARTH_ORBIT` at `u = +6.6` (~1.4 render units above Earth's surface).
pub const EARTH_ORBIT_HOME_LOCAL_POS: DVec3 = DVec3::new(0.0, 0.0, 3.0);

/// Default inertial exponential damping time constant (`tau = 0.18s`).
pub const DEFAULT_INERTIAL_TAU_SEC: f64 = 0.18;

/// Minimum allowed two-hand span (meters / render units) to avoid zoom singularity.
pub const MIN_PINCH_SPAN_M: f64 = 0.05;

/// Minimum magnetic approach throttle floor in render units / s.
pub const MAGNETIC_MIN_SPEED: f64 = 0.05;

/// Minimum supported exponential scale coordinate `u`.
pub const MIN_SCALE_EXP: f64 = -36.0;

/// Maximum supported exponential scale coordinate `u`.
pub const MAX_SCALE_EXP: f64 = 30.0;

/// Applies exponential time-constant damping `v(t) = target + (v(0) - target) * exp(-dt / tau)`.
#[inline]
pub fn exponential_tau_damp_f64(current: f64, target: f64, tau_sec: f64, dt_sec: f64) -> f64 {
    if dt_sec <= 0.0 {
        return current;
    }
    if tau_sec <= 0.0 {
        return target;
    }
    let alpha = 1.0 - (-dt_sec / tau_sec).exp();
    current + (target - current) * alpha
}

/// Vector form of [`exponential_tau_damp_f64`].
#[inline]
pub fn exponential_tau_damp_dvec3(
    current: DVec3,
    target: DVec3,
    tau_sec: f64,
    dt_sec: f64,
) -> DVec3 {
    DVec3::new(
        exponential_tau_damp_f64(current.x, target.x, tau_sec, dt_sec),
        exponential_tau_damp_f64(current.y, target.y, tau_sec, dt_sec),
        exponential_tau_damp_f64(current.z, target.z, tau_sec, dt_sec),
    )
}

/// Computes the Magnetic Approach Auto-Throttle speed ceiling: `clamp(d / 3.0, 0.05, max_speed)`.
///
/// Prevents overshooting when diving directly toward an atom, filament, or planet surface.
#[inline]
pub fn magnetic_approach_throttle(distance_render_units: f64, max_speed: f64) -> f64 {
    let hi = max_speed.max(MAGNETIC_MIN_SPEED);
    let d = if distance_render_units.is_finite() {
        distance_render_units.max(0.0)
    } else {
        hi * 3.0
    };
    (d / 3.0).clamp(MAGNETIC_MIN_SPEED, hi)
}

/// Scales lateral/vertical inspection strafe speed near close-up targets for jitter-free framing.
#[inline]
pub fn close_up_strafe_throttle(distance_render_units: f64, max_speed: f64) -> f64 {
    let min_strafe = 0.08;
    let hi = max_speed.max(min_strafe);
    let d = if distance_render_units.is_finite() {
        distance_render_units.max(0.0)
    } else {
        hi * 1.5
    };
    (d / 1.5).clamp(min_strafe, hi)
}

/// Shapes raw thumbstick / axis input with a deadzone (`0.06` default) and fine-control precision curve.
///
/// Preserves exact endpoints (`0.0 -> 0.0`, `±1.0 -> ±1.0`) while eliminating resting micro-drift
/// that would otherwise keep [`cosmos_narration::RestNarrationFsm`] from locking onto close features.
#[inline]
pub fn shape_stick_axis(raw: f64, deadzone: f64) -> f64 {
    let clamped = raw.clamp(-1.0, 1.0);
    let abs = clamped.abs();
    let dz = deadzone.clamp(0.0, 0.5);
    if abs <= dz {
        return 0.0;
    }
    let norm = ((abs - dz) / (1.0 - dz)).clamp(0.0, 1.0);
    clamped.signum() * norm.powf(1.6)
}

/// Tuning parameters for scale-invariant 6DoF navigation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FlightConfig {
    /// Inertial damping time constant (`tau = 0.18s` by default).
    pub inertial_tau_sec: f64,
    /// Maximum linear flight speed in render units / second.
    pub max_linear_speed: f64,
    /// Maximum angular look/turn rate in radians / second.
    pub max_angular_speed: f64,
    /// Maximum continuous exponential zoom rate `|du/dt|` in decades / second.
    pub max_scale_speed: f64,
    /// Exponential scale increment per mouse-wheel scroll step.
    pub scroll_scale_step: f64,
    /// Orbit sensitivity multiplier (radians per radian of mouse drag).
    pub orbit_sensitivity: f64,
    /// Free-flight mouse look sensitivity multiplier.
    pub look_sensitivity: f64,
    /// Speed multiplier when boost (Shift) is active.
    pub boost_multiplier: f64,
    /// Lower cosmic scale bound `u_min`.
    pub min_scale_exp: f64,
    /// Upper cosmic scale bound `u_max`.
    pub max_scale_exp: f64,
    /// Minimum Home hyper-flight spline duration in seconds.
    pub home_duration_min_sec: f64,
    /// Maximum Home hyper-flight spline duration in seconds.
    pub home_duration_max_sec: f64,
}

impl Default for FlightConfig {
    fn default() -> Self {
        Self {
            inertial_tau_sec: DEFAULT_INERTIAL_TAU_SEC,
            max_linear_speed: 4.5,
            max_angular_speed: 2.6,
            max_scale_speed: 6.0,
            scroll_scale_step: 0.22,
            orbit_sensitivity: 1.0,
            look_sensitivity: 1.0,
            boost_multiplier: 2.5,
            min_scale_exp: MIN_SCALE_EXP,
            max_scale_exp: MAX_SCALE_EXP,
            home_duration_min_sec: 1.4,
            home_duration_max_sec: 4.2,
        }
    }
}

/// Active 3D point/object targeted by hover-scan ray or gaze for focal zoom, orbit, and auto-throttle.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FocusTarget {
    pub entity_id: Option<u64>,
    /// Target hit point in camera-local render space (1 unit = `10^u` m; forward is `-Z`).
    pub focus_point_cam: DVec3,
    /// Clearance distance in render units from camera to hit point / bounding surface.
    pub distance_render_units: f64,
    /// Bounding radius of the focused entity in render units.
    pub bounding_radius_render_units: f64,
}

impl FocusTarget {
    /// Builds a focus target directly from a camera-local ray unit direction and hit distance.
    pub fn from_ray(ray_dir_cam: DVec3, distance_render_units: f64, entity_id: Option<u64>) -> Self {
        let dir = if ray_dir_cam.length_squared() > 1e-12 {
            ray_dir_cam.normalize()
        } else {
            DVec3::NEG_Z
        };
        let d = distance_render_units.max(0.0);
        Self {
            entity_id,
            focus_point_cam: dir * d,
            distance_render_units: d,
            bounding_radius_render_units: 0.0,
        }
    }

    /// Builds a focus target from a projected [`VisibleEntity`] using surface clearance for magnetic throttle.
    pub fn from_visible(entity: &VisibleEntity, ray_point_cam: Option<DVec3>) -> Self {
        let center_cam = entity.pose.pos.as_dvec3();
        let point = ray_point_cam.unwrap_or(center_cam);
        let radius = entity.pose.rel_scale as f64;
        let dist = (point.length() - radius * 0.25).max(0.0);
        Self {
            entity_id: Some(entity.id),
            focus_point_cam: point,
            distance_render_units: dist,
            bounding_radius_render_units: radius,
        }
    }
}

/// Per-frame VR controller & hand tracking input in camera-relative VR space (`1m = 1 render unit`).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct VrFlightInput {
    pub left_grip: bool,
    pub right_grip: bool,
    pub left_hand_pos: DVec3,
    pub right_hand_pos: DVec3,
    /// Dominant controller forward vector in camera-local space (used for single-hand ray thrust).
    pub dominant_hand_forward_cam: DVec3,
    /// Left thumbstick `(x = strafe right, y = forward/back)` in `[-1, 1]`.
    pub left_stick: DVec2,
    /// Right thumbstick `(x = yaw, y = scale dive/zoom)` in `[-1, 1]`.
    pub right_stick: DVec2,
    /// Optional 6DoF heave axis (`+1` up, `-1` down) and pitch/roll input.
    pub vertical_axis: f64,
    pub pitch_axis: f64,
    pub roll_axis: f64,
    /// Single-hand trigger pull `[0..1]` for smooth forward flight along hand ray with magnetic throttle.
    pub trigger_pull: f64,
    /// Triggers continuous Home hyper-flight spline back to Earth Low Orbit (`u = +6.6`).
    pub home_pressed: bool,
}

/// Per-frame Desktop mouse & keyboard input.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DesktopFlightInput {
    /// Mouse-wheel notches (`> 0` scrolls forward / dives inward `du < 0`; `< 0` zooms outward `du > 0`).
    pub scroll_delta: f64,
    /// Left mouse drag (Google-Earth orbit around focus or camera-relative pan).
    pub left_drag_active: bool,
    /// Right mouse drag (free-flight look).
    pub right_drag_active: bool,
    /// `F` key held (or toggled by host) enabling free-flight mode alongside mouse delta.
    pub free_flight_held: bool,
    /// Angular pointer drag delta in radians (`x = yaw right`, `y = pitch up`).
    pub mouse_delta_rad: DVec2,
    /// Camera-local planar pan delta in render units (used on middle/shift-left pan if supplied).
    pub pan_delta_cam: DVec3,
    /// Keyboard 6DoF movement `(x = A/D right, y = Q/E up, z = S/W forward)` where `-z` is forward or
    /// positive `z` is W-forward (`move_axes.z > 0` pushes camera forward along `-Z`).
    pub move_axes: DVec3,
    /// Keyboard/bind roll rate axis `[-1, 1]`.
    pub roll_axis: f64,
    /// Optional boost modifier (Shift).
    pub boost: bool,
    /// Triggers continuous Home hyper-flight spline back to Earth Low Orbit (`u = +6.6`).
    pub home_pressed: bool,
}

/// Persistent state between frames while both VR grips are held for two-hand world-pinch navigation.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TwoHandPinchState {
    pub active: bool,
    pub prev_left: DVec3,
    pub prev_right: DVec3,
}

/// Exact output of one two-hand world-pinch navigation step in camera-local render units.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PinchStepResult {
    /// Change in logarithmic scale exponent (`< 0` diving inward when pulling apart, `> 0` zooming out).
    pub delta_u: f64,
    /// Camera displacement in pre-step (`u_old`) camera-local render units keeping `midpoint` pinned.
    pub cam_local_displacement_old: DVec3,
    /// World-around-midpoint twist angle (radians around camera `+Y` up axis).
    pub delta_yaw_rad: f64,
    /// Current midpoint in camera-local render space.
    pub midpoint_cam: DVec3,
    /// Ratio `d_cur / d_prev`.
    pub span_ratio: f64,
}

/// Computes exact world-pinch scale-dive, midpoint pan, and horizontal twist kinematics.
///
/// Pulling hands apart (`d_cur > d_prev`) increases world magnification (`span_ratio > 1`), yielding
/// `delta_u = -log10(span_ratio) < 0` and translating the camera so the world point under `prev_midpoint`
/// lands cleanly at `cur_midpoint` in post-step render units.
pub fn compute_two_hand_pinch_step(
    prev_left: DVec3,
    prev_right: DVec3,
    cur_left: DVec3,
    cur_right: DVec3,
) -> PinchStepResult {
    let d_prev = prev_left.distance(prev_right).max(MIN_PINCH_SPAN_M);
    let d_cur = cur_left.distance(cur_right).max(MIN_PINCH_SPAN_M);
    let mid_prev = 0.5 * (prev_left + prev_right);
    let mid_cur = 0.5 * (cur_left + cur_right);

    let span_ratio = d_cur / d_prev;
    let delta_u = -span_ratio.log10();
    let zoom_factor = span_ratio;
    // Exact world-pinch invariance: (mid_prev - delta_cam_old) * zoom_factor == mid_cur
    let cam_local_displacement_old = mid_prev - mid_cur / zoom_factor;

    let v_prev_xz = DVec2::new(prev_right.x - prev_left.x, prev_right.z - prev_left.z);
    let v_cur_xz = DVec2::new(cur_right.x - cur_left.x, cur_right.z - cur_left.z);
    let delta_yaw_rad = if v_prev_xz.length() >= MIN_PINCH_SPAN_M && v_cur_xz.length() >= MIN_PINCH_SPAN_M {
        let a = v_prev_xz.normalize();
        let b = v_cur_xz.normalize();
        // Right-handed +Y yaw flips XZ perp sign because +X rotates toward -Z.
        (-a.perp_dot(b)).atan2(a.dot(b))
    } else {
        0.0
    };

    PinchStepResult {
        delta_u,
        cam_local_displacement_old,
        delta_yaw_rad,
        midpoint_cam: mid_cur,
        span_ratio,
    }
}

/// Active continuous Home hyper-flight spline transitioning back to Earth Low Orbit (`u = +6.6`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HomeHyperFlight {
    pub active: bool,
    pub elapsed_sec: f64,
    pub duration_sec: f64,
    pub spline: ScaleSpline,
    pub target_pose: ScalePose64,
}

impl HomeHyperFlight {
    /// Builds a continuous spline from `start` back to Earth Low Orbit (`u = +6.6`).
    pub fn start(
        index: &CosmosSpatialIndex,
        start: &ScalePose64,
        config: &FlightConfig,
    ) -> Result<Self, MathError> {
        let target_pose = ScalePose64::new(
            SectorId::EARTH_ORBIT,
            EARTH_ORBIT_HOME_LOCAL_POS,
            COLD_START_SCALE_EXP,
            DQuat::IDENTITY,
        );
        Self::to_pose(index, start, target_pose, config, None)
    }

    /// Builds a continuous non-teleporting `ScaleSpline` transition from `start` to any `target_pose`.
    pub fn to_pose(
        index: &CosmosSpatialIndex,
        start: &ScalePose64,
        target_pose: ScalePose64,
        config: &FlightConfig,
        duration_override_sec: Option<f64>,
    ) -> Result<Self, MathError> {
        let delta_u = (start.scale_exp - target_pose.scale_exp).abs();
        let duration_sec = duration_override_sec
            .unwrap_or(config.home_duration_min_sec + 0.08 * delta_u)
            .clamp(0.35, config.home_duration_max_sec.max(0.35));

        let mut keyframes = Vec::with_capacity(3);
        keyframes.push(ScaleSplineKeyframe {
            t: 0.0,
            sector_id: start.sector_id,
            local_pos: start.local_pos,
            scale_exp: start.scale_exp,
            orient: start.orient,
        });

        if delta_u > 3.0 {
            let mid_u = if start.scale_exp > target_pose.scale_exp {
                (start.scale_exp + target_pose.scale_exp) * 0.5
            } else {
                // Sub-atomic / tiny scale climbs outward smoothly before descending toward target.
                (target_pose.scale_exp + 1.8).max(start.scale_exp + 2.0)
            };
            let mid_sector = index.best_anchor_sector_for_scale(mid_u);
            let mid_pos = if mid_sector == SectorId::EARTH_ORBIT {
                DVec3::new(0.0, 1.0, 5.0)
            } else {
                DVec3::new(0.0, 0.25, 2.0)
            };
            keyframes.push(ScaleSplineKeyframe {
                t: 0.5,
                sector_id: mid_sector,
                local_pos: mid_pos,
                scale_exp: mid_u.clamp(config.min_scale_exp, config.max_scale_exp),
                orient: start.orient.slerp(target_pose.orient, 0.5).normalize(),
            });
        }

        keyframes.push(ScaleSplineKeyframe {
            t: 1.0,
            sector_id: target_pose.sector_id,
            local_pos: target_pose.local_pos,
            scale_exp: target_pose.scale_exp,
            orient: target_pose.orient,
        });

        Ok(Self {
            active: true,
            elapsed_sec: 0.0,
            duration_sec,
            spline: ScaleSpline::new(keyframes),
            target_pose,
        })
    }

    fn step(&mut self, index: &CosmosSpatialIndex, dt_sec: f64) -> Result<(ScalePose64, bool), MathError> {
        self.elapsed_sec += dt_sec.max(0.0);
        let t = (self.elapsed_sec / self.duration_sec).clamp(0.0, 1.0);
        let pose = self.spline.evaluate(&index.tree, t)?;
        let finished = t >= 1.0;
        if finished {
            self.active = false;
        }
        Ok((pose, finished))
    }
}

/// Scale-invariant 6DoF flight camera supporting VR two-hand world-pinch, Desktop hover-scan scroll,
/// Magnetic Approach Auto-Throttle, inertial damping (`tau = 0.18s`), and continuous Home hyper-flight.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlightCamera {
    pub pose: ScalePose64,
    /// Camera-local linear velocity in render units / second (`+X` right, `+Y` up, `-Z` forward).
    pub linear_vel_cam: DVec3,
    /// Camera-local angular velocity `(pitch_x, yaw_y, roll_z)` in radians / second.
    pub angular_vel_rad: DVec3,
    /// Exponential scale velocity `du/dt` in decades / second.
    pub scale_vel: f64,
    pub pinch: TwoHandPinchState,
    pub home_spline: Option<HomeHyperFlight>,
    pub config: FlightConfig,
}

impl FlightCamera {
    /// Creates a cold-start camera overlooking Earth Low Orbit (`u = +6.6`, `03: Termosfera/Aurora`).
    pub fn earth_low_orbit() -> Self {
        Self {
            pose: ScalePose64::new(
                SectorId::EARTH_ORBIT,
                EARTH_ORBIT_HOME_LOCAL_POS,
                COLD_START_SCALE_EXP,
                DQuat::IDENTITY,
            ),
            linear_vel_cam: DVec3::ZERO,
            angular_vel_rad: DVec3::ZERO,
            scale_vel: 0.0,
            pinch: TwoHandPinchState::default(),
            home_spline: None,
            config: FlightConfig::default(),
        }
    }

    /// Creates a flight camera initialized at an arbitrary [`ScalePose64`].
    pub fn from_pose(pose: ScalePose64, config: FlightConfig) -> Self {
        Self {
            pose,
            linear_vel_cam: DVec3::ZERO,
            angular_vel_rad: DVec3::ZERO,
            scale_vel: 0.0,
            pinch: TwoHandPinchState::default(),
            home_spline: None,
            config,
        }
    }

    /// Returns `true` when the continuous Home hyper-flight spline is animating back to Earth Low Orbit.
    pub fn is_home_flight_active(&self) -> bool {
        self.home_spline.as_ref().is_some_and(|h| h.active)
    }

    /// Instantaneous linear camera speed in render units / second.
    pub fn linear_speed(&self) -> f64 {
        self.linear_vel_cam.length()
    }

    /// Starts continuous non-teleporting Home hyper-flight back to Earth Low Orbit (`u = +6.6`).
    pub fn start_home_flight(&mut self, index: &CosmosSpatialIndex) -> Result<(), MathError> {
        let target_pose = ScalePose64::new(
            SectorId::EARTH_ORBIT,
            EARTH_ORBIT_HOME_LOCAL_POS,
            COLD_START_SCALE_EXP,
            DQuat::IDENTITY,
        );
        self.start_focus_flight(index, target_pose, None)
    }

    /// Starts continuous non-teleporting `ScaleSpline` transition smoothly framing any cosmic target pose.
    pub fn start_focus_flight(
        &mut self,
        index: &CosmosSpatialIndex,
        target_pose: ScalePose64,
        duration_sec: Option<f64>,
    ) -> Result<(), MathError> {
        self.pinch.active = false;
        self.linear_vel_cam = DVec3::ZERO;
        self.angular_vel_rad = DVec3::ZERO;
        self.scale_vel = 0.0;
        self.home_spline = Some(HomeHyperFlight::to_pose(
            index,
            &self.pose,
            target_pose,
            &self.config,
            duration_sec,
        )?);
        Ok(())
    }

    /// Sweeps smoothly around a camera-local pivot at constant angular velocity while keeping distance
    /// invariant — ideal for horizon-locked panoramic documentary showcase orbits.
    pub fn step_panoramic_cruise(
        &mut self,
        index: &CosmosSpatialIndex,
        pivot_cam: DVec3,
        yaw_rate_rad_sec: f64,
        pitch_rate_rad_sec: f64,
        dt_sec: f64,
    ) -> Result<(), MathError> {
        let dt = dt_sec.max(0.0);
        if dt <= 0.0 || pivot_cam.length_squared() < 1e-8 {
            return Ok(());
        }
        let rot_local = DQuat::from_rotation_y(-yaw_rate_rad_sec * dt)
            * DQuat::from_rotation_x(-pitch_rate_rad_sec * dt);
        let cam_shift_local = pivot_cam - rot_local * pivot_cam;
        let shift_world = self.pose.orient * cam_shift_local;
        self.pose.translate_render_units(shift_world, &index.tree)?;
        self.pose.orient = (self.pose.orient * rot_local).normalize();
        self.reanchor_if_needed(index)?;
        Ok(())
    }

    fn minimum_observer_scale(&self, index: &CosmosSpatialIndex) -> f64 {
        if self.pose.sector_id == SectorId::SOLAR_SYSTEM
            && let Some(solar) = index.tree.get(SectorId::SOLAR_SYSTEM)
            && self.pose.local_pos.length() <= solar.radius_m * 10.0_f64.powf(-solar.scale_exp)
        {
            // The index and focus paths use f32 vector lengths before restoring
            // meters. Bound the entire owning domain, with a 64x linear margin
            // below sqrt(f32::MAX) for squared lengths and proxy transforms.
            // For the 4e13m Solar domain this is u≈-3.86 (sub-millimeter units),
            // comfortably below meter-scale inspection without float overflow.
            let max_normalized_radius = f64::from(f32::MAX).sqrt() / 64.0;
            return self.config.min_scale_exp.max((solar.radius_m / max_normalized_radius).log10());
        }
        self.config.min_scale_exp
    }

    /// Re-anchors the camera when leaving its owning spatial/scale domain.
    pub fn reanchor_if_needed(&mut self, index: &CosmosSpatialIndex) -> Result<(), MathError> {
        // Also normalize externally supplied/free-flight poses before they can
        // keep a physical Solar owner with unrepresentable observer units.
        if self.pose.sector_id == SectorId::SOLAR_SYSTEM {
            self.pose.scale_exp = self.pose.scale_exp.max(self.minimum_observer_scale(index));
        }
        let current = index.tree.get(self.pose.sector_id);
        if self.pose.sector_id == SectorId::SOLAR_SYSTEM
            && let Some(solar) = current
            && self.pose.scale_exp <= solar.radius_m.log10()
            && self.pose.local_pos.length() <= solar.radius_m * 10.0_f64.powf(-solar.scale_exp)
        {
            // A close Sun or planet uses much smaller camera units than this
            // heliocentric sector. Picking an unrelated sector by scale alone
            // would change the visible experience and its meter/IPD policy on
            // the first idle frame after leaving a tour. Keep the physical
            // owner inside its declared extent; outward zoom beyond that extent
            // or an explicit destination sector still releases ownership.
            return Ok(());
        }
        let cur_offset = current
            .map_or(f64::INFINITY, |n| (self.pose.scale_exp - n.scale_exp).abs());
        if cur_offset <= 1.5 {
            return Ok(());
        }
        let best = index.best_anchor_sector_for_scale(self.pose.scale_exp);
        if best != self.pose.sector_id {
            self.pose.reanchor(&index.tree, best)?;
        }
        Ok(())
    }

    /// Steps VR 6DoF navigation (two-hand world-pinch or single-hand thumbstick + trigger flight).
    pub fn step_vr(
        &mut self,
        index: &CosmosSpatialIndex,
        input: &VrFlightInput,
        focus: Option<&FocusTarget>,
        dt_sec: f64,
    ) -> Result<(), MathError> {
        let dt = dt_sec.max(0.0);
        if input.home_pressed {
            self.start_home_flight(index)?;
        }

        let two_hand_active = input.left_grip && input.right_grip;
        let has_manual_input = two_hand_active
            || input.left_stick.length_squared() > 1e-5
            || input.right_stick.length_squared() > 1e-5
            || input.vertical_axis.abs() > 1e-4
            || input.trigger_pull > 1e-4;

        if has_manual_input {
            self.home_spline = None;
        }

        if self.step_home_spline_if_active(index, dt)? {
            return Ok(());
        }

        if two_hand_active {
            if !self.pinch.active {
                self.pinch = TwoHandPinchState {
                    active: true,
                    prev_left: input.left_hand_pos,
                    prev_right: input.right_hand_pos,
                };
                self.linear_vel_cam = DVec3::ZERO;
                self.angular_vel_rad = DVec3::ZERO;
                self.scale_vel = 0.0;
                return Ok(());
            }

            let step = compute_two_hand_pinch_step(
                self.pinch.prev_left,
                self.pinch.prev_right,
                input.left_hand_pos,
                input.right_hand_pos,
            );
            self.pinch.prev_left = input.left_hand_pos;
            self.pinch.prev_right = input.right_hand_pos;

            let actual_du = self.apply_pinch_step(index, &step)?;
            if dt > 1e-6 {
                self.scale_vel = actual_du / dt;
                self.angular_vel_rad = DVec3::new(0.0, -step.delta_yaw_rad / dt, 0.0);
                self.linear_vel_cam = step.cam_local_displacement_old / dt;
            }
            return Ok(());
        }

        self.pinch.active = false;

        // Single-hand / thumbstick 6DoF flight + magnetic approach auto-throttle.
        let mut hand_dir = input.dominant_hand_forward_cam;
        if hand_dir.length_squared() < 1e-8 {
            hand_dir = DVec3::NEG_Z;
        } else {
            hand_dir = hand_dir.normalize();
        }

        let max_forward = focus.map_or(self.config.max_linear_speed, |f| {
            magnetic_approach_throttle(f.distance_render_units, self.config.max_linear_speed)
        });
        let max_strafe = focus.map_or(self.config.max_linear_speed, |f| {
            close_up_strafe_throttle(f.distance_render_units, self.config.max_linear_speed)
        });

        let stick_x = shape_stick_axis(input.left_stick.x, 0.06);
        let stick_y = shape_stick_axis(input.left_stick.y, 0.06);
        let rstick_x = shape_stick_axis(input.right_stick.x, 0.06);
        let rstick_y = shape_stick_axis(input.right_stick.y, 0.06);

        let mut target_vel = DVec3::new(
            stick_x * max_strafe,
            shape_stick_axis(input.vertical_axis, 0.04) * max_strafe,
            -stick_y * self.config.max_linear_speed,
        );
        if target_vel.z < 0.0 {
            target_vel.z = target_vel.z.max(-max_forward);
        }
        if input.trigger_pull > 1e-4 {
            let thrust = input.trigger_pull.clamp(0.0, 1.0) * max_forward;
            target_vel += hand_dir * thrust;
        }

        let target_ang = DVec3::new(
            shape_stick_axis(input.pitch_axis, 0.04) * self.config.max_angular_speed,
            -rstick_x * self.config.max_angular_speed,
            shape_stick_axis(input.roll_axis, 0.04) * self.config.max_angular_speed,
        );
        // Right stick Y up (+1) dives inward (du < 0); down (-1) zooms outward (du > 0).
        let target_scale_vel = -rstick_y * self.config.max_scale_speed;

        self.integrate_inertial(index, target_vel, target_ang, target_scale_vel, dt)
    }

    /// Steps Desktop 6DoF locomotion: hover-scan focal scroll zoom, left-drag orbit, right-drag/F free-flight,
    /// WASD/QE movement with Magnetic Approach Auto-Throttle, inertial damping (`tau = 0.18s`), and Home spline.
    pub fn step_desktop(
        &mut self,
        index: &CosmosSpatialIndex,
        input: &DesktopFlightInput,
        focus: Option<&FocusTarget>,
        dt_sec: f64,
    ) -> Result<(), MathError> {
        let dt = dt_sec.max(0.0);
        if input.home_pressed {
            self.start_home_flight(index)?;
        }

        let has_manual_input = input.scroll_delta.abs() > 1e-6
            || input.left_drag_active
            || input.right_drag_active
            || input.mouse_delta_rad.length_squared() > 1e-10
            || input.pan_delta_cam.length_squared() > 1e-10
            || input.move_axes.length_squared() > 1e-6
            || input.roll_axis.abs() > 1e-6;

        if has_manual_input {
            self.home_spline = None;
        }

        if self.step_home_spline_if_active(index, dt)? {
            return Ok(());
        }

        // 1. Google-Earth scroll zoom toward/away from focused 3D point.
        let mut instant_scroll_vel = 0.0;
        if input.scroll_delta.abs() > 1e-6 {
            let raw_du = -input.scroll_delta * self.config.scroll_scale_step;
            let clamped_u = (self.pose.scale_exp + raw_du)
                .clamp(self.minimum_observer_scale(index), self.config.max_scale_exp);
            let actual_du = clamped_u - self.pose.scale_exp;
            let zoom_factor = 10.0_f64.powf(-actual_du);

            let pivot = focus
                .map(|f| f.focus_point_cam)
                .unwrap_or(DVec3::new(0.0, 0.0, -2.0));
            // Keeps the focused point directly beneath the hover-scan ray while diving/zooming.
            let delta_cam_old = pivot * (1.0 - 1.0 / zoom_factor);
            if delta_cam_old.length_squared() > 0.0 {
                let delta_world = self.pose.orient * delta_cam_old;
                self.pose.translate_render_units(delta_world, &index.tree)?;
            }
            self.pose.scale_exp = clamped_u;
            self.reanchor_if_needed(index)?;
            if dt > 1e-6 {
                instant_scroll_vel = actual_du / dt;
            }
        }

        // 2. Pan delta (camera-local).
        if input.pan_delta_cam.length_squared() > 0.0 {
            let delta_world = self.pose.orient * input.pan_delta_cam;
            self.pose.translate_render_units(delta_world, &index.tree)?;
            self.reanchor_if_needed(index)?;
        }

        // 3. Mouse look / Google-Earth orbit.
        if input.mouse_delta_rad.length_squared() > 0.0 {
            if input.right_drag_active || input.free_flight_held {
                let yaw = -input.mouse_delta_rad.x * self.config.look_sensitivity;
                let pitch = -input.mouse_delta_rad.y * self.config.look_sensitivity;
                let delta_q = DQuat::from_rotation_y(yaw) * DQuat::from_rotation_x(pitch);
                self.pose.orient = (self.pose.orient * delta_q).normalize();
            } else if input.left_drag_active {
                let pivot = focus
                    .map(|f| f.focus_point_cam)
                    .filter(|p| p.length_squared() > 1e-6)
                    .unwrap_or(DVec3::new(0.0, 0.0, -2.0));
                let yaw = -input.mouse_delta_rad.x * self.config.orbit_sensitivity;
                let pitch = -input.mouse_delta_rad.y * self.config.orbit_sensitivity;
                let rot_local = DQuat::from_rotation_y(yaw) * DQuat::from_rotation_x(pitch);
                // Orbit camera around `pivot`: move camera so pivot stays at same offset relative to new orientation.
                let cam_shift_local = pivot - rot_local * pivot;
                let shift_world = self.pose.orient * cam_shift_local;
                self.pose.translate_render_units(shift_world, &index.tree)?;
                self.pose.orient = (self.pose.orient * rot_local).normalize();
                self.reanchor_if_needed(index)?;
            }
        }

        // 4. WASD/QE 6DoF flight with magnetic approach auto-throttle + boost.
        let boost = if input.boost {
            self.config.boost_multiplier
        } else {
            1.0
        };
        let base_speed = self.config.max_linear_speed * boost;
        let max_forward = focus.map_or(base_speed, |f| {
            magnetic_approach_throttle(f.distance_render_units, base_speed)
        });
        let max_strafe = focus.map_or(base_speed, |f| {
            close_up_strafe_throttle(f.distance_render_units, base_speed)
        });

        let mut target_vel = DVec3::new(
            input.move_axes.x.clamp(-1.0, 1.0) * max_strafe,
            input.move_axes.y.clamp(-1.0, 1.0) * max_strafe,
            -input.move_axes.z.clamp(-1.0, 1.0) * base_speed,
        );
        if target_vel.z < 0.0 {
            target_vel.z = target_vel.z.max(-max_forward);
        }

        let target_ang = DVec3::new(
            0.0,
            0.0,
            input.roll_axis.clamp(-1.0, 1.0) * self.config.max_angular_speed,
        );

        self.integrate_inertial(index, target_vel, target_ang, 0.0, dt)?;
        if instant_scroll_vel != 0.0 {
            self.scale_vel = instant_scroll_vel;
        }
        Ok(())
    }

    fn apply_pinch_step(
        &mut self,
        index: &CosmosSpatialIndex,
        step: &PinchStepResult,
    ) -> Result<f64, MathError> {
        let clamped_u = (self.pose.scale_exp + step.delta_u)
            .clamp(self.minimum_observer_scale(index), self.config.max_scale_exp);
        let actual_du = clamped_u - self.pose.scale_exp;
        let actual_zoom_factor = 10.0_f64.powf(-actual_du);

        // Recompute midpoint shift using clamped `actual_zoom_factor` so boundary clamping stays exact.
        let mid_cur = step.midpoint_cam;
        let mid_prev = step.cam_local_displacement_old + mid_cur / step.span_ratio;
        let delta_cam_old = mid_prev - mid_cur / actual_zoom_factor;

        if delta_cam_old.length_squared() > 0.0 {
            let delta_world = self.pose.orient * delta_cam_old;
            self.pose.translate_render_units(delta_world, &index.tree)?;
        }
        self.pose.scale_exp = clamped_u;

        // Rotate world by `+delta_yaw_rad` around `mid_cur` => rotate camera around `mid_cur` by `-delta_yaw_rad`.
        if step.delta_yaw_rad.abs() > 1e-9 {
            let rot_local = DQuat::from_rotation_y(-step.delta_yaw_rad);
            let cam_shift_local = mid_cur - rot_local * mid_cur;
            let shift_world = self.pose.orient * cam_shift_local;
            self.pose.translate_render_units(shift_world, &index.tree)?;
            self.pose.orient = (self.pose.orient * rot_local).normalize();
        }

        self.reanchor_if_needed(index)?;
        Ok(actual_du)
    }

    fn integrate_inertial(
        &mut self,
        index: &CosmosSpatialIndex,
        target_linear_cam: DVec3,
        target_angular_rad: DVec3,
        target_scale_vel: f64,
        dt_sec: f64,
    ) -> Result<(), MathError> {
        if dt_sec <= 0.0 {
            return Ok(());
        }
        let tau = self.config.inertial_tau_sec;
        self.linear_vel_cam =
            exponential_tau_damp_dvec3(self.linear_vel_cam, target_linear_cam, tau, dt_sec);
        self.angular_vel_rad =
            exponential_tau_damp_dvec3(self.angular_vel_rad, target_angular_rad, tau, dt_sec);
        self.scale_vel = exponential_tau_damp_f64(self.scale_vel, target_scale_vel, tau, dt_sec);

        if self.linear_vel_cam.length_squared() > 1e-12 {
            let delta_world = self.pose.orient * (self.linear_vel_cam * dt_sec);
            self.pose.translate_render_units(delta_world, &index.tree)?;
        }
        if self.angular_vel_rad.length_squared() > 1e-12 {
            let rot = DQuat::from_rotation_y(self.angular_vel_rad.y * dt_sec)
                * DQuat::from_rotation_x(self.angular_vel_rad.x * dt_sec)
                * DQuat::from_rotation_z(self.angular_vel_rad.z * dt_sec);
            self.pose.orient = (self.pose.orient * rot).normalize();
        }
        if self.scale_vel.abs() > 1e-12 {
            self.pose.scale_exp = (self.pose.scale_exp + self.scale_vel * dt_sec)
                .clamp(self.minimum_observer_scale(index), self.config.max_scale_exp);
        }

        self.reanchor_if_needed(index)?;
        Ok(())
    }

    fn step_home_spline_if_active(
        &mut self,
        index: &CosmosSpatialIndex,
        dt: f64,
    ) -> Result<bool, MathError> {
        let Some(mut home) = self.home_spline.take() else {
            return Ok(false);
        };
        if !home.active {
            return Ok(false);
        }
        let prev_u = self.pose.scale_exp;
        let (next_pose, finished) = home.step(index, dt)?;
        self.scale_vel = if dt > 1e-6 {
            (next_pose.scale_exp - prev_u) / dt
        } else {
            0.0
        };
        self.pose = next_pose;
        self.reanchor_if_needed(index)?;
        if !finished {
            self.home_spline = Some(home);
        } else {
            self.scale_vel = 0.0;
        }
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmos_spatial::build_default_universe_index;

    #[test]
    fn cold_start_earth_low_orbit() {
        let index = build_default_universe_index();
        let mut cam = FlightCamera::earth_low_orbit();
        assert_eq!(cam.pose.sector_id, SectorId::EARTH_ORBIT);
        assert!((cam.pose.scale_exp - 6.6).abs() < 1e-12);
        cam.reanchor_if_needed(&index).unwrap();
        assert_eq!(cam.pose.sector_id, SectorId::EARTH_ORBIT);
    }

    fn close_solar_observation() -> ScalePose64 {
        // solar_granulation_horizon's physical observer, after its rail ends.
        ScalePose64::new(SectorId::SOLAR_SYSTEM,
            DVec3::new(-0.0018753014155864836, 0.0007204302590712021, 0.004172571681299502),
            8.8, DQuat::from_xyzw(0.3160160610058623, -0.37828431157035886,
                -0.41377003449100497, 0.7653947918318961))
    }

    #[test]
    fn desktop_and_vr_free_exploration_keep_the_physical_solar_owner() {
        let index = build_default_universe_index();
        for vr in [false, true] {
            let start = close_solar_observation();
            let mut cam = FlightCamera::from_pose(start, FlightConfig::default());
            if vr {
                cam.step_vr(&index, &VrFlightInput::default(), None, 1.0/120.0).unwrap();
            } else {
                cam.step_desktop(&index, &DesktopFlightInput::default(), None, 1.0/120.0).unwrap();
            }
            assert_eq!(cam.pose, start, "an idle free-flight frame cannot change the physical owner");
            for _ in 0..120 {
                if vr {
                    cam.step_vr(&index, &VrFlightInput {
                        left_stick: DVec2::new(0.25, 0.0),
                        right_stick: DVec2::new(0.1, -0.2),
                        ..Default::default()
                    }, None, 1.0/120.0).unwrap();
                } else {
                    cam.step_desktop(&index, &DesktopFlightInput {
                        move_axes: DVec3::new(0.25, 0.0, 0.0),
                        scroll_delta: 0.005,
                        mouse_delta_rad: DVec2::new(0.001, 0.0),
                        right_drag_active: true,
                        ..Default::default()
                    }, None, 1.0/120.0).unwrap();
                }
                assert_eq!(cam.pose.sector_id, SectorId::SOLAR_SYSTEM);
            }
            assert!(cam.pose.local_pos.distance(start.local_pos) > 1e-6);
            assert!((cam.pose.scale_exp-start.scale_exp).abs() > 0.01);
        }

        // Canonical whole-body inspection also uses small physical camera units,
        // including Pluto and Mercury far from the Earth's coordinate domain.
        for zone in [0, 1, 11] {
            let (center, body_scale, radius) = cosmos_spatial::solar_body_local_pose(zone, 0.0, 0.0, 20.0);
            let pose = ScalePose64::new(SectorId::SOLAR_SYSTEM,
                center+DVec3::Z*radius*3.0*10.0_f64.powf(-cosmos_spatial::SOLAR_SECTOR_SCALE_EXP),
                body_scale+0.55, DQuat::IDENTITY);
            let mut cam = FlightCamera::from_pose(pose, FlightConfig::default());
            cam.step_vr(&index, &VrFlightInput::default(), None, 1.0/120.0).unwrap();
            assert_eq!(cam.pose, pose, "body {zone} must keep its physical owner");
        }
    }

    #[test]
    fn solar_ownership_releases_for_outward_zoom_domain_exit_and_explicit_home() {
        let index = build_default_universe_index();
        let solar = index.tree.get(SectorId::SOLAR_SYSTEM).unwrap();
        let mut cam = FlightCamera::from_pose(close_solar_observation(), FlightConfig::default());
        cam.pose.scale_exp = solar.radius_m.log10()-0.01;
        cam.step_desktop(&index, &DesktopFlightInput::default(), None, 1.0/120.0).unwrap();
        assert_eq!(cam.pose.sector_id, SectorId::SOLAR_SYSTEM);
        let stationary_pivot = FocusTarget::from_ray(DVec3::NEG_Z, 0.0, None);
        cam.step_desktop(&index, &DesktopFlightInput { scroll_delta: -1.0, ..Default::default() },
            Some(&stationary_pivot), 1.0/120.0).unwrap();
        assert_ne!(cam.pose.sector_id, SectorId::SOLAR_SYSTEM, "outward zoom must leave the Solar domain");

        cam = FlightCamera::from_pose(close_solar_observation(), FlightConfig::default());
        cam.pose.local_pos = DVec3::Z*solar.radius_m*1.01*10.0_f64.powf(-solar.scale_exp);
        cam.step_vr(&index, &VrFlightInput::default(), None, 1.0/120.0).unwrap();
        assert_ne!(cam.pose.sector_id, SectorId::SOLAR_SYSTEM, "Solar ownership is spatially bounded");

        cam = FlightCamera::from_pose(close_solar_observation(), FlightConfig::default());
        cam.start_home_flight(&index).unwrap();
        for _ in 0..300 {
            cam.step_vr(&index, &VrFlightInput::default(), None, 0.02).unwrap();
            if !cam.is_home_flight_active() { break; }
        }
        assert!(!cam.is_home_flight_active());
        assert_eq!(cam.pose.sector_id, SectorId::EARTH_ORBIT);
        assert!(cam.pose.local_pos.distance(EARTH_ORBIT_HOME_LOCAL_POS) < 1e-8);
        cam.step_vr(&index, &VrFlightInput::default(), None, 1.0/120.0).unwrap();
        assert_eq!(cam.pose.sector_id, SectorId::EARTH_ORBIT, "Earth's existing owner policy remains intact");
    }

    #[test]
    fn physical_solar_zoom_has_a_finite_domain_derived_numerical_floor() {
        let index = build_default_universe_index();
        let mut cam = FlightCamera::from_pose(close_solar_observation(), FlightConfig::default());
        let minimum = cam.minimum_observer_scale(&index);
        assert!(minimum < -3.0, "meter-scale exploration remains comfortably inside the supported range");
        for vr in [false, true] {
            cam.pose = close_solar_observation();
            cam.pose.scale_exp = 0.0;
            if vr {
                for _ in 0..120 {
                    cam.step_vr(&index, &VrFlightInput {
                        right_stick: DVec2::new(0.0, 1.0), ..Default::default()
                    }, None, 0.1).unwrap();
                }
            } else {
                cam.step_desktop(&index, &DesktopFlightInput {
                    scroll_delta: 1000.0, ..Default::default()
                }, None, 1.0/120.0).unwrap();
            }
            assert_eq!(cam.pose.sector_id, SectorId::SOLAR_SYSTEM);
            assert!((cam.pose.scale_exp-minimum).abs() < 1e-10);
            let domain_radius = index.tree.get(SectorId::SOLAR_SYSTEM).unwrap().radius_m;
            let normalized = DVec3::splat(domain_radius*10.0_f64.powf(-cam.pose.scale_exp)).as_vec3();
            assert!(normalized.length_squared().is_finite());
        }
        let position = cam.pose.local_pos;
        cam.pose.scale_exp = -36.0;
        cam.reanchor_if_needed(&index).unwrap();
        assert_eq!(cam.pose.sector_id, SectorId::SOLAR_SYSTEM);
        assert_eq!(cam.pose.local_pos, position);
        assert!((cam.pose.scale_exp-minimum).abs() < 1e-10);
    }

    #[test]
    fn magnetic_approach_throttle_matches_spec() {
        assert!((magnetic_approach_throttle(0.0, 5.0) - 0.05).abs() < 1e-12);
        assert!((magnetic_approach_throttle(0.06, 5.0) - 0.05).abs() < 1e-12);
        assert!((magnetic_approach_throttle(1.5, 5.0) - 0.5).abs() < 1e-12);
        assert!((magnetic_approach_throttle(6.0, 5.0) - 2.0).abs() < 1e-12);
        assert!((magnetic_approach_throttle(30.0, 5.0) - 5.0).abs() < 1e-12);
    }

    #[test]
    fn inertial_damping_decays_by_one_over_e_at_tau() {
        let v0 = 10.0;
        let v_tau = exponential_tau_damp_f64(v0, 0.0, 0.18, 0.18);
        assert!((v_tau - v0 * (-1.0_f64).exp()).abs() < 1e-12);
    }

    #[test]
    fn two_hand_pinch_zooms_inward_on_pull_apart_and_outward_on_push_together() {
        let pull_apart = compute_two_hand_pinch_step(
            DVec3::new(-0.15, 0.0, -1.0),
            DVec3::new(0.15, 0.0, -1.0),
            DVec3::new(-0.30, 0.0, -1.0),
            DVec3::new(0.30, 0.0, -1.0),
        );
        assert!(pull_apart.delta_u < 0.0, "pulling apart must dive inward");
        assert!((pull_apart.delta_u - (-2.0_f64.log10())).abs() < 1e-12);

        let push_together = compute_two_hand_pinch_step(
            DVec3::new(-0.30, 0.0, -1.0),
            DVec3::new(0.30, 0.0, -1.0),
            DVec3::new(-0.15, 0.0, -1.0),
            DVec3::new(0.15, 0.0, -1.0),
        );
        assert!(push_together.delta_u > 0.0, "pushing together must zoom out");
        assert!((push_together.delta_u - 2.0_f64.log10()).abs() < 1e-12);
    }

    #[test]
    fn two_hand_pinch_keeps_world_midpoint_pinned_under_hands() {
        let index = build_default_universe_index();
        let mut cam = FlightCamera::earth_low_orbit();
        let p_mid_prev = DVec3::new(0.12, -0.08, -1.4);
        let l0 = p_mid_prev + DVec3::new(-0.20, 0.0, 0.0);
        let r0 = p_mid_prev + DVec3::new(0.20, 0.0, 0.0);

        let mut mid_world_pose = cam.pose;
        mid_world_pose
            .translate_render_units(cam.pose.orient * p_mid_prev, &index.tree)
            .unwrap();
        let mut right_world_pose = cam.pose;
        right_world_pose
            .translate_render_units(cam.pose.orient * r0, &index.tree)
            .unwrap();

        // Translate midpoint, pull apart 2.5x, AND twist yaw simultaneously!
        let p_mid_cur = DVec3::new(-0.05, 0.04, -1.1);
        let twist = 0.37_f64;
        let half_cur = DVec3::new(twist.cos(), 0.0, twist.sin()) * 0.50;
        let l1 = p_mid_cur - half_cur;
        let r1 = p_mid_cur + half_cur;

        cam.step_vr(
            &index,
            &VrFlightInput {
                left_grip: true,
                right_grip: true,
                left_hand_pos: l0,
                right_hand_pos: r0,
                ..Default::default()
            },
            None,
            0.016,
        )
        .unwrap();
        cam.step_vr(
            &index,
            &VrFlightInput {
                left_grip: true,
                right_grip: true,
                left_hand_pos: l1,
                right_hand_pos: r1,
                ..Default::default()
            },
            None,
            0.016,
        )
        .unwrap();

        let rel_mid = cam
            .pose
            .project_entity(&index.tree, &mid_world_pose, 100.0)
            .unwrap();
        let mid_cam = cam.pose.orient.conjugate() * rel_mid.pos.as_dvec3();
        assert!(mid_cam.distance(p_mid_cur) < 1e-5, "midpoint drifted: {mid_cam:?}");

        let rel_right = cam
            .pose
            .project_entity(&index.tree, &right_world_pose, 100.0)
            .unwrap();
        let right_cam = cam.pose.orient.conjugate() * rel_right.pos.as_dvec3();
        assert!(right_cam.distance(r1) < 1e-5, "right grip drifted: {right_cam:?} vs {r1:?}");
    }

    #[test]
    fn desktop_scroll_zooms_toward_focused_point_without_drift() {
        let index = build_default_universe_index();
        let mut cam = FlightCamera::earth_low_orbit();
        let focus_point = DVec3::new(0.4, -0.25, -2.5);
        let mut entity_pose = cam.pose;
        entity_pose
            .translate_render_units(cam.pose.orient * focus_point, &index.tree)
            .unwrap();
        let focus = FocusTarget {
            entity_id: Some(7),
            focus_point_cam: focus_point,
            distance_render_units: focus_point.length(),
            bounding_radius_render_units: 0.1,
        };

        cam.step_desktop(
            &index,
            &DesktopFlightInput {
                scroll_delta: 4.0,
                ..Default::default()
            },
            Some(&focus),
            0.016,
        )
        .unwrap();

        assert!(cam.pose.scale_exp < COLD_START_SCALE_EXP);
        let rel = cam.pose.project_entity(&index.tree, &entity_pose, 10.0).unwrap();
        let rel_cam = cam.pose.orient.conjugate() * rel.pos.as_dvec3();
        assert!(rel_cam.distance(focus_point) < 1e-5);
    }

    #[test]
    fn home_hyper_flight_continuously_returns_to_earth_low_orbit() {
        let index = build_default_universe_index();
        let deep_pose = ScalePose64::new(
            SectorId::SOLAR_SYSTEM,
            DVec3::new(2.0, -1.0, 4.0),
            12.4,
            DQuat::from_rotation_y(0.8),
        );
        let mut cam = FlightCamera::from_pose(deep_pose, FlightConfig::default());
        cam.start_home_flight(&index).unwrap();
        assert!(cam.is_home_flight_active());

        let mut samples = Vec::new();
        for _ in 0..300 {
            cam.step_desktop(&index, &DesktopFlightInput::default(), None, 0.02)
                .unwrap();
            samples.push(cam.pose.scale_exp);
            if !cam.is_home_flight_active() {
                break;
            }
        }

        assert!(!cam.is_home_flight_active(), "home flight should finish");
        assert!((cam.pose.scale_exp - COLD_START_SCALE_EXP).abs() < 1e-6);
        assert_eq!(cam.pose.sector_id, SectorId::EARTH_ORBIT);
        // No teleport jump between consecutive samples.
        for pair in samples.windows(2) {
            assert!((pair[1] - pair[0]).abs() < 0.25, "teleport detected: {:?}", pair);
        }
    }

    #[test]
    fn magnetic_throttle_caps_forward_flight_near_target() {
        let index = build_default_universe_index();
        let mut cam = FlightCamera::earth_low_orbit();
        let close_focus = FocusTarget::from_ray(DVec3::NEG_Z, 0.6, Some(1));
        for _ in 0..40 {
            cam.step_desktop(
                &index,
                &DesktopFlightInput {
                    move_axes: DVec3::new(0.0, 0.0, 1.0),
                    ..Default::default()
                },
                Some(&close_focus),
                0.05,
            )
            .unwrap();
        }
        // `clamp(0.6 / 3.0, 0.05, 4.5) = 0.20` render units / s.
        assert!(cam.linear_speed() <= 0.2001);
        assert!(cam.linear_speed() > 0.15);
    }

    #[test]
    fn panoramic_cruise_and_close_up_strafe_preserve_stability() {
        let index = build_default_universe_index();
        let mut cam = FlightCamera::earth_low_orbit();
        let pivot = DVec3::new(0.0, 0.0, -3.0);
        let mut body_pose = cam.pose;
        body_pose
            .translate_render_units(cam.pose.orient * pivot, &index.tree)
            .unwrap();

        for _ in 0..30 {
            cam.step_panoramic_cruise(&index, pivot, 0.4, 0.05, 0.02)
                .unwrap();
        }
        let rel = cam.pose.project_entity(&index.tree, &body_pose, 100.0).unwrap();
        let rel_cam = cam.pose.orient.conjugate() * rel.pos.as_dvec3();
        assert!(
            (rel_cam.length() - pivot.length()).abs() < 1e-4,
            "panoramic orbit radius drifted"
        );

        assert_eq!(shape_stick_axis(0.04, 0.06), 0.0);
        assert!((shape_stick_axis(1.0, 0.06) - 1.0).abs() < 1e-12);
        assert!((close_up_strafe_throttle(0.15, 4.5) - 0.10).abs() < 1e-12);
    }
}
