pub mod diegetic;
pub mod flight;

pub use diegetic::{
    is_vr_palm_up, Bead, BeadKind, DiegeticAction, ScaleMeridianArc, ScaleMeridianTick, SummonMode,
    WristResonanceRing, WristSummonInput, MERIDIAN_FADEOUT_SEC, MERIDIAN_SPEED_THRESHOLD,
    PLAYBACK_RATES,
};
pub use flight::{
    close_up_strafe_throttle, compute_two_hand_pinch_step, exponential_tau_damp_dvec3,
    exponential_tau_damp_f64, magnetic_approach_throttle, shape_stick_axis, DesktopFlightInput,
    FlightCamera, FlightConfig, FocusTarget, HomeHyperFlight, PinchStepResult, TwoHandPinchState,
    VrFlightInput, COLD_START_SCALE_EXP, DEFAULT_INERTIAL_TAU_SEC, EARTH_ORBIT_HOME_LOCAL_POS,
    MAGNETIC_MIN_SPEED, MAX_SCALE_EXP, MIN_PINCH_SPAN_M, MIN_SCALE_EXP,
};
