use glam::{DMat4, Mat4, Quat, Vec3, Vec4};

pub const DEFAULT_VR_NEAR_RENDER_UNITS: f32 = 0.04;
pub const DEFAULT_DESKTOP_NEAR_RENDER_UNITS: f32 = 0.02;

/// Camera-centered view matrix (`camera_pos = (0, 0, 0)`) from world orientation quaternion.
#[inline]
pub fn camera_view_matrix(orient: Quat) -> Mat4 {
    Mat4::from_quat(orient.conjugate())
}

/// Right-handed Reversed-Z infinite far-plane perspective matrix (`wgpu` depth `[0, 1]`).
///
/// Maps `z = -near` (in right-handed view space, `-Z` forward) to `z_ndc = 1.0`,
/// and `z -> -infinity` asymptotically to `z_ndc = 0.0` (`z_ndc = near / d`).
#[inline]
pub fn reversed_z_infinite_perspective_rh(fov_y_rad: f32, aspect: f32, near: f32) -> Mat4 {
    let f = 1.0 / (0.5 * fov_y_rad).tan();
    Mat4::from_cols(
        Vec4::new(f / aspect, 0.0, 0.0, 0.0),
        Vec4::new(0.0, f, 0.0, 0.0),
        Vec4::new(0.0, 0.0, 0.0, -1.0),
        Vec4::new(0.0, 0.0, near, 0.0),
    )
}

/// Double-precision variant of [`reversed_z_infinite_perspective_rh`] for CPU frustum extraction.
#[inline]
pub fn reversed_z_infinite_perspective_rh_f64(fov_y_rad: f64, aspect: f64, near: f64) -> DMat4 {
    let f = 1.0 / (0.5 * fov_y_rad).tan();
    DMat4::from_cols(
        glam::DVec4::new(f / aspect, 0.0, 0.0, 0.0),
        glam::DVec4::new(0.0, f, 0.0, 0.0),
        glam::DVec4::new(0.0, 0.0, 0.0, -1.0),
        glam::DVec4::new(0.0, 0.0, near, 0.0),
    )
}

/// Right-handed Reversed-Z infinite far-plane projection from asymmetric eye tangents
/// (matching `openxr::Fovf`: `tan_left < 0`, `tan_right > 0`, `tan_down < 0`, `tan_up > 0`).
#[inline]
pub fn reversed_z_infinite_asymmetric_rh(
    tan_left: f32,
    tan_right: f32,
    tan_down: f32,
    tan_up: f32,
    near: f32,
) -> Mat4 {
    let inv_w = 1.0 / (tan_right - tan_left);
    let inv_h = 1.0 / (tan_up - tan_down);
    let sx = 2.0 * inv_w;
    let sy = 2.0 * inv_h;
    let cx = (tan_right + tan_left) * inv_w;
    let cy = (tan_up + tan_down) * inv_h;
    Mat4::from_cols(
        Vec4::new(sx, 0.0, 0.0, 0.0),
        Vec4::new(0.0, sy, 0.0, 0.0),
        Vec4::new(cx, cy, 0.0, -1.0),
        Vec4::new(0.0, 0.0, near, 0.0),
    )
}

/// Six frustum planes in camera-relative world space `(nx, ny, nz, d)` where `n·p + d >= 0` is inside.
#[derive(Clone, Copy, Debug)]
pub struct FrustumPlanes {
    /// `[left, right, bottom, top, near, far]` (in Reversed-Z infinite projection, `far` enforces `w >= 0` forward half-space).
    pub planes: [Vec4; 6],
}

impl FrustumPlanes {
    /// Extracts planes from a Reversed-Z `view_proj = proj * view` matrix using `wgpu` NDC `[0, 1]`.
    pub fn from_reversed_z_view_proj(view_proj: Mat4) -> Self {
        let r0 = Vec4::new(view_proj.x_axis.x, view_proj.y_axis.x, view_proj.z_axis.x, view_proj.w_axis.x);
        let r1 = Vec4::new(view_proj.x_axis.y, view_proj.y_axis.y, view_proj.z_axis.y, view_proj.w_axis.y);
        let r2 = Vec4::new(view_proj.x_axis.z, view_proj.y_axis.z, view_proj.z_axis.z, view_proj.w_axis.z);
        let r3 = Vec4::new(view_proj.x_axis.w, view_proj.y_axis.w, view_proj.z_axis.w, view_proj.w_axis.w);

        // WGPU clip conditions: -w <= x <= w, -w <= y <= w, 0 <= z <= w (for Reversed-Z near=1 => w-z >= 0, far=0 => z >= 0).
        let raw = [
            r3 + r0, // Left: x + w >= 0
            r3 - r0, // Right: w - x >= 0
            r3 + r1, // Bottom: y + w >= 0
            r3 - r1, // Top: w - y >= 0
            r3 - r2, // Near (Reversed-Z z_ndc <= 1): w - z >= 0
            r2,      // Far (Reversed-Z z_ndc >= 0): z >= 0 (in infinite far plane this is near >= 0, satisfied for w > 0 when combined with w - r2, actually use r3 for forward halfspace z_view <= 0? Wait: let's check!)
        ];

        // Wait: in `reversed_z_infinite_perspective_rh`, r2 = (0, 0, 0, near), so `r2` is constant `(0, 0, 0, near)` whose normal is ZERO!
        // Instead, forward half-space `w >= 0` is `r3`! For camera view_proj where row3 = (-fwd, cam_d), `r3` gives `dot(fwd_inward, p) >= 0`!
        let mut planes = raw;
        planes[5] = r3;
        for p in &mut planes {
            let len = Vec3::new(p.x, p.y, p.z).length();
            if len > 1e-8 {
                *p /= len;
            }
        }
        Self { planes }
    }

    /// Returns `true` if sphere `(center, radius)` is inside or intersects the frustum.
    #[inline]
    pub fn intersects_sphere(&self, center: Vec3, radius: f32) -> bool {
        for p in &self.planes {
            let dist = p.x * center.x + p.y * center.y + p.z * center.z + p.w;
            if dist < -radius {
                return false;
            }
        }
        true
    }

    /// Returns `true` if axis-aligned bounding box `[min, max]` is inside or intersects the frustum.
    #[inline]
    pub fn intersects_aabb(&self, min: Vec3, max: Vec3) -> bool {
        for p in &self.planes {
            // Positive vertex along plane normal
            let px = if p.x >= 0.0 { max.x } else { min.x };
            let py = if p.y >= 0.0 { max.y } else { min.y };
            let pz = if p.z >= 0.0 { max.z } else { min.z };
            if p.x * px + p.y * py + p.z * pz + p.w < 0.0 {
                return false;
            }
        }
        true
    }
}
