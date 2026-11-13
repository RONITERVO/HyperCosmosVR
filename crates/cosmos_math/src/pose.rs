use glam::{DQuat, DVec3, Quat, Vec3};
use serde::{Deserialize, Serialize};

use crate::{MathError, SectorId, SectorTree};

/// Threshold (`u_entity - u_cam > FAR_SHELL_LOG_THRESHOLD` or distance `> 10^5.5` render units)
/// beyond which distant macro structures clamp onto the Far-Shell unit sky sphere.
pub const FAR_SHELL_LOG_THRESHOLD: f64 = 5.5;

/// Radius (in camera render units) at which Far-Shell geometry is placed before depth clear.
pub const FAR_SHELL_PROXY_RADIUS: f32 = 5000.0;

/// Double-precision scale-normalized pose inside a hierarchical [`SectorId`] frame.
///
/// For any camera scale exponent `u_cam`, `1.0` GPU render unit equals `10^u_cam` meters
/// (`1 VR meter = 10^u_cam meters`), keeping nearby visible geometry strictly inside `[0.01, 5000.0]`
/// around `(0, 0, 0)` with full FP32 precision.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScalePose64 {
    pub sector_id: SectorId,
    /// Position in canonical units of `sector_id` (`meters * 10^-sector.scale_exp`).
    pub local_pos: DVec3,
    /// Intrinsic scale exponent `u = log10(radius_or_meter_scale)`.
    ///
    /// For the camera, `1.0` VR/GPU render unit = `10^scale_exp` physical meters.
    pub scale_exp: f64,
    pub orient: DQuat,
}

impl Default for ScalePose64 {
    fn default() -> Self {
        Self {
            sector_id: SectorId::UNIVERSE_ROOT,
            local_pos: DVec3::ZERO,
            scale_exp: 0.0,
            orient: DQuat::IDENTITY,
        }
    }
}

impl ScalePose64 {
    /// Creates a new pose inside `sector_id`.
    #[inline]
    pub const fn new(sector_id: SectorId, local_pos: DVec3, scale_exp: f64, orient: DQuat) -> Self {
        Self {
            sector_id,
            local_pos,
            scale_exp,
            orient,
        }
    }

    /// Moves this pose by `delta_render` expressed in **current camera render units**
    /// (`1 render unit = 10^self.scale_exp meters`).
    pub fn translate_render_units(
        &mut self,
        delta_render: DVec3,
        tree: &SectorTree,
    ) -> Result<(), MathError> {
        let sector = tree
            .get(self.sector_id)
            .ok_or(MathError::UnknownSector(self.sector_id))?;
        let scale_factor = 10.0f64.powf(self.scale_exp - sector.scale_exp);
        self.local_pos += delta_render * scale_factor;
        Ok(())
    }

    /// Re-anchors this pose into `target_sector` while preserving exact `f64` world placement.
    pub fn reanchor(
        &mut self,
        tree: &SectorTree,
        target_sector: SectorId,
    ) -> Result<(), MathError> {
        let (new_pos, new_orient) =
            tree.transform_between(self.sector_id, self.local_pos, self.orient, target_sector)?;
        self.sector_id = target_sector;
        self.local_pos = new_pos;
        self.orient = new_orient;
        Ok(())
    }

    /// Computes entity pose relative to `self` (the camera) in camera-centered GPU `f32` render units.
    ///
    /// Camera sits strictly at `(0, 0, 0)` in output coordinates; orientation is transformed cleanly
    /// through the sectors' Lowest Common Ancestor (`LCA`) in `f64` before casting to `f32`.
    pub fn project_entity(
        &self,
        tree: &SectorTree,
        entity: &ScalePose64,
        entity_radius_m: f64,
    ) -> Result<CameraRelativePose, MathError> {
        let lca = tree.lowest_common_ancestor(self.sector_id, entity.sector_id)?;
        let lca_node = tree.get(lca).ok_or(MathError::UnknownSector(lca))?;

        let (cam_in_lca, _cam_rot) =
            tree.lift_to_ancestor(self.sector_id, self.local_pos, self.orient, lca)?;
        let (ent_in_lca, ent_rot) =
            tree.lift_to_ancestor(entity.sector_id, entity.local_pos, entity.orient, lca)?;

        // Subtract first in LCA normalized units before scaling to camera render units.
        // When camera & entity share a sector at e.g. u = -35 or u = +28, this preserves
        // full 53-bit f64 mantissa on small displacements with zero catastrophic cancellation.
        let diff_lca = ent_in_lca - cam_in_lca;
        let delta_exp = lca_node.scale_exp - self.scale_exp;
        let p_rel_f64 = diff_lca * 10.0f64.powf(delta_exp);

        let dist_render = p_rel_f64.length();
        let inv_cam_m = 10.0f64.powf(-self.scale_exp);
        let rel_radius_f64 = entity_radius_m * inv_cam_m;

        let far_threshold_dist = 10.0f64.powf(FAR_SHELL_LOG_THRESHOLD);
        let is_far_shell = (entity.scale_exp - self.scale_exp) > FAR_SHELL_LOG_THRESHOLD
            || dist_render > far_threshold_dist;

        let angular_rad = apparent_angular_rad(rel_radius_f64, dist_render);

        let (pos_f32, rel_scale_f32) = if is_far_shell && dist_render > 1e-12 {
            let dir = (p_rel_f64 / dist_render).as_vec3();
            let proxy_scale = (FAR_SHELL_PROXY_RADIUS as f64 / dist_render) * rel_radius_f64;
            (dir * FAR_SHELL_PROXY_RADIUS, proxy_scale as f32)
        } else {
            (p_rel_f64.as_vec3(), rel_radius_f64 as f32)
        };

        Ok(CameraRelativePose {
            pos: pos_f32,
            rel_scale: rel_scale_f32,
            orient: ent_rot.as_quat(),
            angular_rad: angular_rad as f32,
            distance_render_units: dist_render,
            is_far_shell,
        })
    }
}

/// GPU-ready camera-relative transform (`camera == (0, 0, 0)`).
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CameraRelativePose {
    /// Entity position relative to camera (`1 unit = 10^u_cam meters`), clamped to `FAR_SHELL_PROXY_RADIUS` when `is_far_shell == true`.
    pub pos: Vec3,
    /// Entity apparent radius in GPU render units (scaled proportionally if placed on Far Shell).
    pub rel_scale: f32,
    pub orient: Quat,
    /// Full apparent angular diameter `2 * atan(radius / distance)` in radians.
    pub angular_rad: f32,
    /// True unclamped center-to-center distance in camera render units (`f64`).
    pub distance_render_units: f64,
    /// Whether this object belongs to the Far-Shell background sky pass (`u_ent - u_cam > 5.5` or `dist > 10^5.5`).
    pub is_far_shell: bool,
}

/// Computes apparent angular diameter in radians: `2 * atan2(radius, distance)`.
///
/// Handles inside/intersecting shells gracefully (`distance <= radius` -> `PI`).
#[inline]
pub fn apparent_angular_rad(radius: f64, distance: f64) -> f64 {
    if radius <= 0.0 {
        return 0.0;
    }
    if distance <= radius {
        return std::f64::consts::PI;
    }
    2.0 * (radius / distance).atan()
}

/// Projects a huge background direction onto a unit far-shell sphere of `proxy_radius`.
#[inline]
pub fn far_shell_proxy(dir_f64: DVec3, proxy_radius: f32) -> Vec3 {
    let n = dir_f64.normalize_or(DVec3::NEG_Z);
    n.as_vec3() * proxy_radius
}
