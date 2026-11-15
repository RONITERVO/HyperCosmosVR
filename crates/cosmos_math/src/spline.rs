use glam::{DQuat, DVec3};
use serde::{Deserialize, Serialize};

use crate::{MathError, ScalePose64, SectorId, SectorTree};

/// Control point along a multi-scale cosmic camera trajectory.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScaleSplineKeyframe {
    pub t: f64,
    pub sector_id: SectorId,
    pub local_pos: DVec3,
    pub scale_exp: f64,
    pub orient: DQuat,
}

/// Smooth exponential scale-dive trajectory spanning arbitrary orders of magnitude (`u ∈ [-35, +28]`).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ScaleSpline {
    pub keyframes: Vec<ScaleSplineKeyframe>,
}

impl ScaleSpline {
    /// Builds a validated keyframe trajectory sorted by ascending `t`.
    pub fn new(mut keyframes: Vec<ScaleSplineKeyframe>) -> Self {
        keyframes.sort_by(|a, b| a.t.total_cmp(&b.t));
        Self { keyframes }
    }

    /// Evaluates pose at parameter `t` (clamped at endpoints), interpolating `scale_exp` linearly in log-10 space
    /// (exponentially in meters) and interpolating sector coordinates smoothly in their `LCA` frame.
    pub fn evaluate(&self, tree: &SectorTree, t: f64) -> Result<ScalePose64, MathError> {
        self.evaluate_segments(tree, t, true)
    }

    /// Evaluates a densely sampled path without braking at each interior knot.
    /// The caller can ease the overall journey parameter once at its endpoints.
    pub fn evaluate_without_segment_easing(&self, tree: &SectorTree, t: f64) -> Result<ScalePose64, MathError> {
        self.evaluate_segments(tree, t, false)
    }

    fn evaluate_segments(&self, tree: &SectorTree, t: f64, ease_segments: bool) -> Result<ScalePose64, MathError> {
        if self.keyframes.is_empty() {
            return Err(MathError::EmptySpline);
        }
        if self.keyframes.len() == 1 || t <= self.keyframes[0].t {
            let k = &self.keyframes[0];
            return Ok(ScalePose64::new(k.sector_id, k.local_pos, k.scale_exp, k.orient));
        }
        let last = self.keyframes.last().unwrap();
        if t >= last.t {
            return Ok(ScalePose64::new(
                last.sector_id,
                last.local_pos,
                last.scale_exp,
                last.orient,
            ));
        }

        let idx = self
            .keyframes
            .partition_point(|k| k.t <= t)
            .saturating_sub(1)
            .min(self.keyframes.len() - 2);
        let k0 = &self.keyframes[idx];
        let k1 = &self.keyframes[idx + 1];
        let span = (k1.t - k0.t).max(1e-12);
        let alpha = ((t - k0.t) / span).clamp(0.0, 1.0);
        let s = if ease_segments { smoothstep(0.0, 1.0, alpha) } else { alpha };

        // Log-scale Catmull-Rom when interior neighbors exist, smooth Hermite at ends.
        let u0 = if idx > 0 { self.keyframes[idx - 1].scale_exp } else { k0.scale_exp };
        let u3 = if idx + 2 < self.keyframes.len() {
            self.keyframes[idx + 2].scale_exp
        } else {
            k1.scale_exp
        };
        let scale_exp = catmull_rom_f64(u0, k0.scale_exp, k1.scale_exp, u3, alpha);

        let lca = tree.lowest_common_ancestor(k0.sector_id, k1.sector_id)?;
        let (p0, r0) = tree.lift_to_ancestor(k0.sector_id, k0.local_pos, k0.orient, lca)?;
        let (p1, r1) = tree.lift_to_ancestor(k1.sector_id, k1.local_pos, k1.orient, lca)?;
        let pos_lca = p0.lerp(p1, s);
        let orient_lca = r0.slerp(r1, s).normalize();

        // Anchor into the destination keyframe's sector on the second half for crisp local precision.
        let target_sector = if alpha < 0.5 { k0.sector_id } else { k1.sector_id };
        let (local_pos, orient) = tree.transform_between(lca, pos_lca, orient_lca, target_sector)?;

        Ok(ScalePose64::new(target_sector, local_pos, scale_exp, orient))
    }
}

/// Frame-rate independent exponential smoothing toward `target` using half-life `half_life_sec`.
#[inline]
pub fn exponential_damp(current: f64, target: f64, half_life_sec: f64, dt_sec: f64) -> f64 {
    if half_life_sec <= 0.0 {
        return target;
    }
    let decay = 0.5f64.powf(dt_sec.max(0.0) / half_life_sec);
    target + (current - target) * decay
}

#[inline]
pub fn smoothstep(edge0: f64, edge1: f64, x: f64) -> f64 {
    let t = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

#[inline]
fn catmull_rom_f64(p0: f64, p1: f64, p2: f64, p3: f64, t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    0.5 * ((2.0 * p1)
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}

