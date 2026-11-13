use cosmos_manifest::Locale;
use glam::{Quat, Vec3};
use serde::{Deserialize, Serialize};

/// Canonical 9 narration playback speeds (`1.0x` through `3.0x` in `0.25x` increments).
pub const PLAYBACK_RATES: [f32; 9] = [1.0, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5, 2.75, 3.0];

const PLAYBACK_RATE_LABELS: [&str; 9] = [
    "1.0x", "1.25x", "1.5x", "1.75x", "2.0x", "2.25x", "2.5x", "2.75x", "3.0x",
];

/// Minimum logarithmic scale speed (`|du/dt|`) that summons the ephemeral Scale Meridian Arc.
pub const MERIDIAN_SPEED_THRESHOLD: f64 = 0.25;

/// Fadeout duration in seconds (`600 ms`) after scale velocity settles below `MERIDIAN_SPEED_THRESHOLD`.
pub const MERIDIAN_FADEOUT_SEC: f32 = 0.600;

/// Semantic role of a bead on the 3D Wrist / Orbital Resonance Ring.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum BeadKind {
    PlaybackRate { index: u8, rate_hundredths: u16 },
    LocaleToggle { locale: Locale },
    HomeAnchor,
}

impl BeadKind {
    #[inline]
    pub fn playback_rate(index: u8) -> Self {
        let rate = PLAYBACK_RATES[index as usize];
        Self::PlaybackRate {
            index,
            rate_hundredths: (rate * 100.0).round() as u16,
        }
    }

    #[inline]
    pub fn rate_f32(&self) -> Option<f32> {
        match *self {
            Self::PlaybackRate { index, .. } => PLAYBACK_RATES.get(index as usize).copied(),
            _ => None,
        }
    }
}

struct BeadSpec {
    id: u32,
    kind: BeadKind,
    local_pos: Vec3,
    radius: f32,
    selected: bool,
    color_hdr: [f32; 3],
    label: &'static str,
}

/// 3D spatial pearl rendered directly in world/render space (`sprite_kind = 4, sub_param <= 0.5`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bead {
    pub id: u32,
    pub kind: BeadKind,
    pub local_pos: Vec3,
    pub world_pos: Vec3,
    pub radius: f32,
    pub highlighted: bool,
    pub selected: bool,
    pub hover_t: f32,
    pub color_hdr: [f32; 3],
    pub label: String,
}

/// Semantic action emitted when the user selects a diegetic 3D bead.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum DiegeticAction {
    SetPlaybackRate(f32),
    SetLocale(Locale),
    TriggerEarthHome,
}

/// Summon trigger mode for the diegetic 3D Orbital / Wrist Resonance Ring.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum SummonMode {
    Hidden,
    VrWristPalmUp,
    DesktopHeld,
    DesktopPinnedTab,
}

/// Per-frame VR / Desktop inputs driving summon state, bead coordinates, and ray/touch selection.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WristSummonInput {
    pub is_vr: bool,
    pub wrist_pos: Vec3,
    pub wrist_rot: Quat,
    pub palm_normal_world: Vec3,
    pub head_pos: Vec3,
    pub head_up: Vec3,
    pub cam_pos: Vec3,
    pub cam_rot: Quat,
    /// Desktop `Right-Hold` summon state.
    pub desktop_right_hold: bool,
    /// Desktop `Tab` key press edge toggling persistent orbital ring visibility.
    pub desktop_tab_just_pressed: bool,
    /// Active selection ray origin in render space (controller ray or desktop cursor ray).
    pub pointer_origin: Option<Vec3>,
    /// Active unit selection ray direction in render space.
    pub pointer_dir: Option<Vec3>,
    /// Direct index-finger touch coordinate in render space (VR near-field poke).
    pub finger_tip_world: Option<Vec3>,
    /// Trigger pinch / left-click confirmation edge.
    pub select_triggered: bool,
}

impl Default for WristSummonInput {
    fn default() -> Self {
        Self {
            is_vr: false,
            wrist_pos: Vec3::ZERO,
            wrist_rot: Quat::IDENTITY,
            palm_normal_world: Vec3::Y,
            head_pos: Vec3::ZERO,
            head_up: Vec3::Y,
            cam_pos: Vec3::ZERO,
            cam_rot: Quat::IDENTITY,
            desktop_right_hold: false,
            desktop_tab_just_pressed: false,
            pointer_origin: None,
            pointer_dir: None,
            finger_tip_world: None,
            select_triggered: false,
        }
    }
}

/// Evaluates whether the tracked VR wrist palm is rotated face-up toward the HMD viewer.
#[inline]
pub fn is_vr_palm_up(
    palm_normal_world: Vec3,
    head_pos: Vec3,
    wrist_pos: Vec3,
    head_up: Vec3,
) -> bool {
    let normal = palm_normal_world.normalize_or_zero();
    if normal.length_squared() < 0.5 {
        return false;
    }
    let to_head = (head_pos - wrist_pos).normalize_or_zero();
    let up = head_up.normalize_or_zero();
    normal.dot(to_head) > 0.55 && normal.dot(up) > 0.20
}

/// Diegetic 3D Wrist / Orbital Resonance Ring carrying playback speed (`1.0x..3.0x`),
/// translation locale pearls (`EN` / `FI`), and the central Earth Home anchor pearl.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WristResonanceRing {
    pub summon_mode: SummonMode,
    pub desktop_tab_pinned: bool,
    pub open_alpha: f32,
    pub anchor_pos: Vec3,
    pub anchor_rot: Quat,
    pub selected_rate_index: u8,
    pub selected_locale: Locale,
    pub hovered_bead_id: Option<u32>,
    pub beads: Vec<Bead>,
}

impl Default for WristResonanceRing {
    fn default() -> Self {
        let mut ring = Self {
            summon_mode: SummonMode::Hidden,
            desktop_tab_pinned: false,
            open_alpha: 0.0,
            anchor_pos: Vec3::ZERO,
            anchor_rot: Quat::IDENTITY,
            selected_rate_index: 0,
            selected_locale: Locale::EnUs,
            hovered_bead_id: None,
            beads: Vec::with_capacity(12),
        };
        ring.rebuild_beads(0.0);
        ring
    }
}

impl WristResonanceRing {
    /// Returns the currently active playback speed (`1.0..=3.0` in `0.25` steps).
    #[inline]
    pub fn active_playback_rate(&self) -> f32 {
        PLAYBACK_RATES[self.selected_rate_index as usize]
    }

    /// Sets the active playback speed to the closest valid step if present.
    pub fn set_playback_rate(&mut self, rate: f32) {
        if let Some((idx, _)) = PLAYBACK_RATES
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| (*a - rate).abs().total_cmp(&(*b - rate).abs()))
        {
            self.selected_rate_index = idx as u8;
        }
    }

    /// Steps visibility, transforms 3D bead coordinates, and evaluates pointer/touch selection.
    pub fn step(&mut self, input: &WristSummonInput, dt_sec: f32) -> Option<DiegeticAction> {
        let dt = dt_sec.max(0.0);

        if input.desktop_tab_just_pressed {
            self.desktop_tab_pinned = !self.desktop_tab_pinned;
        }

        let target_mode = if input.is_vr {
            if is_vr_palm_up(
                input.palm_normal_world,
                input.head_pos,
                input.wrist_pos,
                input.head_up,
            ) {
                SummonMode::VrWristPalmUp
            } else {
                SummonMode::Hidden
            }
        } else if input.desktop_right_hold {
            SummonMode::DesktopHeld
        } else if self.desktop_tab_pinned {
            SummonMode::DesktopPinnedTab
        } else {
            SummonMode::Hidden
        };
        self.summon_mode = target_mode;

        let target_open = if target_mode == SummonMode::Hidden {
            0.0
        } else {
            1.0
        };
        let alpha_step = if dt > 0.0 {
            1.0 - (-dt / 0.065).exp()
        } else {
            1.0
        };
        self.open_alpha = self.open_alpha + (target_open - self.open_alpha) * alpha_step;
        if target_open == 0.0 && self.open_alpha < 0.01 {
            self.open_alpha = 0.0;
        } else if target_open == 1.0 && self.open_alpha > 0.99 {
            self.open_alpha = 1.0;
        }

        // Update 3D anchor pose in render space.
        if input.is_vr {
            self.anchor_pos = input.wrist_pos + input.palm_normal_world.normalize_or_zero() * 0.085;
            self.anchor_rot = input.wrist_rot;
        } else {
            let forward = input.cam_rot * Vec3::NEG_Z;
            let up = input.cam_rot * Vec3::Y;
            self.anchor_pos = input.cam_pos + forward * 0.58 - up * 0.11;
            self.anchor_rot = input.cam_rot;
        }

        self.rebuild_beads(dt);

        if self.summon_mode == SummonMode::Hidden && self.open_alpha <= 0.05 {
            self.hovered_bead_id = None;
            return None;
        }

        self.update_selection(input)
    }

    fn rebuild_beads(&mut self, dt_sec: f32) {
        let prev_hover: Vec<(u32, f32)> = self.beads.iter().map(|b| (b.id, b.hover_t)).collect();
        self.beads.clear();

        let scale = self.open_alpha.clamp(0.05, 1.0);
        let rate_ring_r = 0.095 * scale;
        let locale_ring_r = 0.056 * scale;

            // 0: Central Earth Home anchor pearl.
        self.push_bead(
            BeadSpec {
                id: 0,
                kind: BeadKind::HomeAnchor,
                local_pos: Vec3::ZERO,
                radius: 0.017 * scale,
                selected: false,
                color_hdr: [0.35, 1.15, 2.0],
                label: "HOME",
            },
            &prev_hover,
            dt_sec,
        );

        // 1..=2: EN / FI locale pearls on lower inner flanking arc.
        let locales = [
            (1u32, Locale::EnUs, -0.58_f32, "EN", [0.45, 1.45, 1.85]),
            (2u32, Locale::FiFi, 0.58_f32, "FI", [0.85, 1.10, 2.05]),
        ];
        for (id, locale, angle_rad, label, color_hdr) in locales {
            let local_pos = Vec3::new(
                angle_rad.sin() * locale_ring_r,
                -angle_rad.cos() * locale_ring_r,
                0.0,
            );
            self.push_bead(
                BeadSpec {
                    id,
                    kind: BeadKind::LocaleToggle { locale },
                    local_pos,
                    radius: 0.014 * scale,
                    selected: self.selected_locale == locale,
                    color_hdr,
                    label,
                },
                &prev_hover,
                dt_sec,
            );
        }

        // 3..=11: 9 playback speed beads along upper 200-degree orbital arc (-100 deg to +100 deg).
        let arc_span = 200.0_f32.to_radians();
        for (idx, label) in PLAYBACK_RATE_LABELS.iter().enumerate() {
            let t = idx as f32 / (PLAYBACK_RATES.len() - 1) as f32;
            let theta = -0.5 * arc_span + t * arc_span;
            let local_pos = Vec3::new(theta.sin() * rate_ring_r, theta.cos() * rate_ring_r, 0.0);
            let is_selected = self.selected_rate_index as usize == idx;
            self.push_bead(
                BeadSpec {
                    id: 10 + idx as u32,
                    kind: BeadKind::playback_rate(idx as u8),
                    local_pos,
                    radius: if is_selected { 0.0145 } else { 0.0115 } * scale,
                    selected: is_selected,
                    color_hdr: if is_selected {
                        [2.1, 1.55, 0.45]
                    } else {
                        [0.95, 1.10, 1.35]
                    },
                    label,
                },
                &prev_hover,
                dt_sec,
            );
        }
    }

    fn push_bead(&mut self, spec: BeadSpec, prev_hover: &[(u32, f32)], dt_sec: f32) {
        let highlighted = self.hovered_bead_id == Some(spec.id);
        let prev_t = prev_hover
            .iter()
            .find(|(pid, _)| *pid == spec.id)
            .map_or(0.0, |(_, t)| *t);
        let target = if highlighted { 1.0 } else { 0.0 };
        let alpha = if dt_sec > 0.0 {
            1.0 - (-dt_sec / 0.060).exp()
        } else {
            1.0
        };
        let hover_t = prev_t + (target - prev_t) * alpha;

        let world_pos = self.anchor_pos + self.anchor_rot * spec.local_pos;
        self.beads.push(Bead {
            id: spec.id,
            kind: spec.kind,
            local_pos: spec.local_pos,
            world_pos,
            radius: spec.radius * (1.0 + 0.25 * hover_t),
            highlighted,
            selected: spec.selected,
            hover_t,
            color_hdr: spec.color_hdr,
            label: spec.label.to_owned(),
        });
    }

    fn update_selection(&mut self, input: &WristSummonInput) -> Option<DiegeticAction> {
        let mut best: Option<(u32, BeadKind, f32)> = None;

        // 1. Direct VR fingertip touch takes priority when within proximity.
        if let Some(tip) = input.finger_tip_world {
            for bead in &self.beads {
                let d = bead.world_pos.distance(tip);
                let hit_r = bead.radius + 0.016;
                if d <= hit_r && best.is_none_or(|(_, _, bd)| d < bd) {
                    best = Some((bead.id, bead.kind, d));
                }
            }
        }

        // 2. Ray cone intersection from controller / desktop cursor ray.
        if best.is_none()
            && let (Some(origin), Some(dir_raw)) = (input.pointer_origin, input.pointer_dir)
        {
            let dir = dir_raw.normalize_or_zero();
            if dir.length_squared() > 0.5 {
                for bead in &self.beads {
                    let to_bead = bead.world_pos - origin;
                    let proj = to_bead.dot(dir);
                    if proj <= 0.0 {
                        continue;
                    }
                    let perp_dist = (to_bead - dir * proj).length();
                    let hit_cone_r = (bead.radius * 1.45).max(0.014);
                    if perp_dist <= hit_cone_r && best.is_none_or(|(_, _, bd)| perp_dist < bd) {
                        best = Some((bead.id, bead.kind, perp_dist));
                    }
                }
            }
        }

        let new_hover = best.map(|(id, _, _)| id);
        self.hovered_bead_id = new_hover;
        for bead in &mut self.beads {
            bead.highlighted = Some(bead.id) == new_hover;
        }

        if !input.select_triggered {
            return None;
        }

        let (_, kind, _) = best?;
        match kind {
            BeadKind::PlaybackRate { index, .. } => {
                self.selected_rate_index = index;
                let rate = PLAYBACK_RATES[index as usize];
                for bead in &mut self.beads {
                    bead.selected = bead.kind == kind;
                }
                Some(DiegeticAction::SetPlaybackRate(rate))
            }
            BeadKind::LocaleToggle { locale } => {
                self.selected_locale = locale;
                for bead in &mut self.beads {
                    if matches!(bead.kind, BeadKind::LocaleToggle { .. }) {
                        bead.selected = bead.kind == kind;
                    }
                }
                Some(DiegeticAction::SetLocale(locale))
            }
            BeadKind::HomeAnchor => Some(DiegeticAction::TriggerEarthHome),
        }
    }
}

/// 3D tick sample on the ephemeral Scale Meridian Arc rendered via `sprite_kind = 4, sub_param > 0.5`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleMeridianTick {
    pub scale_exp_tick: i32,
    pub local_pos: Vec3,
    pub world_pos: Vec3,
    pub is_major: bool,
    pub alpha: f32,
    pub sub_param: f32,
}

/// Ephemeral Scale Meridian Arc appearing only while `|du/dt| > 0.25` and fading over `600 ms` after stopping.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleMeridianArc {
    pub visible: bool,
    pub opacity: f32,
    pub fade_elapsed_sec: f32,
    pub current_scale_exp: f64,
    pub current_scale_vel: f64,
}

impl Default for ScaleMeridianArc {
    fn default() -> Self {
        Self {
            visible: false,
            opacity: 0.0,
            fade_elapsed_sec: MERIDIAN_FADEOUT_SEC,
            current_scale_exp: 6.6,
            current_scale_vel: 0.0,
        }
    }
}

impl ScaleMeridianArc {
    /// Updates activation & `600 ms` fadeout envelope given scale velocity `du/dt`.
    pub fn step(&mut self, scale_exp: f64, du_dt: f64, dt_sec: f32) {
        let dt = dt_sec.max(0.0);
        self.current_scale_exp = scale_exp;
        self.current_scale_vel = du_dt;

        if du_dt.abs() > MERIDIAN_SPEED_THRESHOLD {
            self.visible = true;
            self.opacity = 1.0;
            self.fade_elapsed_sec = 0.0;
        } else if self.opacity > 0.0 {
            self.fade_elapsed_sec += dt;
            let remaining = (1.0 - self.fade_elapsed_sec / MERIDIAN_FADEOUT_SEC).clamp(0.0, 1.0);
            self.opacity = remaining;
            if remaining <= 0.0 {
                self.visible = false;
            }
        } else {
            self.visible = false;
        }
    }

    /// Generates curved peripheral 3D meridian ticks around `current_scale_exp` in camera space.
    pub fn sample_ticks(
        &self,
        cam_pos: Vec3,
        cam_rot: Quat,
        arc_radius_m: f32,
    ) -> Vec<ScaleMeridianTick> {
        if !self.visible || self.opacity <= 0.001 {
            return Vec::new();
        }
        let u_floor = self.current_scale_exp.floor() as i32;
        let mut ticks = Vec::with_capacity(9);
        for offset in -4..=4 {
            let tick_u = u_floor + offset;
            let rel_u = tick_u as f32 - self.current_scale_exp as f32;
            // Subtle peripheral right-hand meridian curvature.
            let angle = (rel_u * 0.14).clamp(-0.65, 0.65);
            let local_pos = Vec3::new(
                arc_radius_m * 0.34 + (1.0 - angle.cos()) * arc_radius_m * 0.25,
                angle.sin() * arc_radius_m * 0.45,
                -arc_radius_m,
            );
            let falloff = (1.0 - (rel_u.abs() / 4.5)).clamp(0.15, 1.0);
            ticks.push(ScaleMeridianTick {
                scale_exp_tick: tick_u,
                local_pos,
                world_pos: cam_pos + cam_rot * local_pos,
                is_major: tick_u.rem_euclid(5) == 0,
                alpha: self.opacity * falloff,
                sub_param: 0.85,
            });
        }
        ticks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrist_ring_has_exact_playback_rates_locales_and_home() {
        let ring = WristResonanceRing::default();
        assert_eq!(ring.beads.len(), 12);

        let rates: Vec<f32> = ring
            .beads
            .iter()
            .filter_map(|b| b.kind.rate_f32())
            .collect();
        assert_eq!(rates, PLAYBACK_RATES);
        assert_eq!(rates.len(), 9);
        assert_eq!(rates[0], 1.0);
        assert_eq!(rates[8], 3.0);

        let has_en = ring
            .beads
            .iter()
            .any(|b| b.kind == BeadKind::LocaleToggle { locale: Locale::EnUs });
        let has_fi = ring
            .beads
            .iter()
            .any(|b| b.kind == BeadKind::LocaleToggle { locale: Locale::FiFi });
        let has_home = ring.beads.iter().any(|b| b.kind == BeadKind::HomeAnchor);
        assert!(has_en && has_fi && has_home);
    }

    #[test]
    fn vr_palm_up_summons_and_selects_playback_rate_and_locale() {
        let mut ring = WristResonanceRing::default();
        let mut vr = WristSummonInput {
            is_vr: true,
            wrist_pos: Vec3::new(-0.2, -0.25, -0.45),
            wrist_rot: Quat::IDENTITY,
            palm_normal_world: Vec3::new(0.1, 0.9, 0.4).normalize(),
            head_pos: Vec3::ZERO,
            head_up: Vec3::Y,
            ..Default::default()
        };

        ring.step(&vr, 0.2);
        assert_eq!(ring.summon_mode, SummonMode::VrWristPalmUp);
        assert!(ring.open_alpha > 0.9);

        // Aim pointer directly at 2.25x bead (`PLAYBACK_RATES[5]`).
        let rate_bead_pos = ring
            .beads
            .iter()
            .find(|b| b.kind.rate_f32() == Some(2.25))
            .unwrap()
            .world_pos;
        vr.pointer_origin = Some(Vec3::ZERO);
        vr.pointer_dir = Some(rate_bead_pos.normalize());
        vr.select_triggered = true;

        let action = ring.step(&vr, 0.016);
        assert_eq!(action, Some(DiegeticAction::SetPlaybackRate(2.25)));
        assert!((ring.active_playback_rate() - 2.25).abs() < 1e-6);

        // Switch locale pearl to FI via direct finger poke.
        let fi_pos = ring
            .beads
            .iter()
            .find(|b| b.kind == BeadKind::LocaleToggle { locale: Locale::FiFi })
            .unwrap()
            .world_pos;
        vr.pointer_origin = None;
        vr.pointer_dir = None;
        vr.finger_tip_world = Some(fi_pos);
        let action2 = ring.step(&vr, 0.016);
        assert_eq!(action2, Some(DiegeticAction::SetLocale(Locale::FiFi)));
        assert_eq!(ring.selected_locale, Locale::FiFi);
    }

    #[test]
    fn desktop_right_hold_and_tab_toggle_home_anchor() {
        let mut ring = WristResonanceRing::default();

        // Tab toggles pinned mode on/off.
        ring.step(
            &WristSummonInput {
                desktop_tab_just_pressed: true,
                ..Default::default()
            },
            0.2,
        );
        assert_eq!(ring.summon_mode, SummonMode::DesktopPinnedTab);

        // Aim at HOME anchor and trigger.
        let home_pos = ring
            .beads
            .iter()
            .find(|b| b.kind == BeadKind::HomeAnchor)
            .unwrap()
            .world_pos;
        let action = ring.step(
            &WristSummonInput {
                pointer_origin: Some(Vec3::ZERO),
                pointer_dir: Some(home_pos.normalize()),
                select_triggered: true,
                ..Default::default()
            },
            0.016,
        );
        assert_eq!(action, Some(DiegeticAction::TriggerEarthHome));

        // Tab toggles off; Right-Hold still summons cleanly.
        ring.step(
            &WristSummonInput {
                desktop_tab_just_pressed: true,
                ..Default::default()
            },
            0.2,
        );
        assert_eq!(ring.summon_mode, SummonMode::Hidden);

        ring.step(
            &WristSummonInput {
                desktop_right_hold: true,
                ..Default::default()
            },
            0.2,
        );
        assert_eq!(ring.summon_mode, SummonMode::DesktopHeld);
    }

    #[test]
    fn scale_meridian_arc_activates_above_threshold_and_fades_over_600ms() {
        let mut arc = ScaleMeridianArc::default();

        // Slow drift does not summon.
        arc.step(6.6, 0.20, 0.016);
        assert!(!arc.visible);
        assert_eq!(arc.opacity, 0.0);

        // Fast dive activates immediately.
        arc.step(6.4, -0.40, 0.016);
        assert!(arc.visible);
        assert!((arc.opacity - 1.0).abs() < 1e-6);
        assert!(!arc.sample_ticks(Vec3::ZERO, Quat::IDENTITY, 1.0).is_empty());

        // Stopping fades cleanly over 600ms.
        arc.step(6.4, 0.0, 0.300);
        assert!(arc.visible);
        assert!((arc.opacity - 0.5).abs() < 1e-5);

        arc.step(6.4, 0.0, 0.300);
        assert!(!arc.visible);
        assert_eq!(arc.opacity, 0.0);
        assert!(arc.sample_ticks(Vec3::ZERO, Quat::IDENTITY, 1.0).is_empty());
    }
}
