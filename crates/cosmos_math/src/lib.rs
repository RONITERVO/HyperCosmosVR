//! Double-precision multiscale coordinate frames (`ScalePose64`), Reversed-Z projections,
//! and exponential scale splines spanning >62 orders of magnitude (`10^-35 m` to `10^+28 m`).

pub mod pose;
pub mod projection;
pub mod sector;
pub mod spline;

pub use pose::{
    CameraRelativePose, FAR_SHELL_LOG_THRESHOLD, FAR_SHELL_PROXY_RADIUS, ScalePose64,
    apparent_angular_rad, far_shell_proxy,
};
pub use projection::{
    DEFAULT_DESKTOP_NEAR_RENDER_UNITS, DEFAULT_VR_NEAR_RENDER_UNITS, FrustumPlanes,
    camera_view_matrix, reversed_z_infinite_asymmetric_rh, reversed_z_infinite_perspective_rh,
    reversed_z_infinite_perspective_rh_f64,
};
pub use sector::{SectorId, SectorNode, SectorTree};
pub use spline::{ScaleSpline, ScaleSplineKeyframe, exponential_damp, smoothstep};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum MathError {
    #[error("unknown sector id: {0:?}")]
    UnknownSector(SectorId),
    #[error("cycle detected in sector hierarchy at {0:?}")]
    CycleDetected(SectorId),
    #[error("sectors {0:?} and {1:?} share no common ancestor")]
    DisconnectedSectors(SectorId, SectorId),
    #[error("invalid sector hierarchy: {0}")]
    InvalidSectorHierarchy(String),
    #[error("cannot evaluate empty ScaleSpline")]
    EmptySpline,
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::{DQuat, DVec3, Mat4, Vec3, Vec4};

    fn test_tree() -> SectorTree {
        let mut tree = SectorTree::with_root(0.0);
        tree.insert_sector(SectorNode::new(
            SectorId::PLANCK_ORIGIN,
            "Planck Foam",
            Some(SectorId::UNIVERSE_ROOT),
            DVec3::ZERO,
            -35.0,
            1e-33,
        ))
        .unwrap();
        tree.insert_sector(SectorNode::new(
            SectorId::QUANTUM_FIELD,
            "Quantum Fields",
            Some(SectorId::UNIVERSE_ROOT),
            DVec3::ZERO,
            -18.0,
            1e-12,
        ))
        .unwrap();
        tree.insert_sector(SectorNode::new(
            SectorId::EARTH_SURFACE,
            "Earth Surface",
            Some(SectorId::UNIVERSE_ROOT),
            DVec3::ZERO,
            -3.0,
            1e6,
        ))
        .unwrap();
        tree.insert_sector(SectorNode::new(
            SectorId::SOLAR_SYSTEM,
            "Solar System",
            Some(SectorId::UNIVERSE_ROOT),
            DVec3::new(0.0, 0.0, 0.0),
            11.0,
            1e13,
        ))
        .unwrap();
        tree.insert_sector(SectorNode::new(
            SectorId::EARTH_ORBIT,
            "Earth Orbit",
            Some(SectorId::SOLAR_SYSTEM),
            DVec3::new(1.496, 0.0, 0.0), // 1.496e11 m
            6.8,
            1e8,
        ))
        .unwrap();
        tree.insert_sector(SectorNode::new(
            SectorId::MILKY_WAY,
            "Milky Way",
            Some(SectorId::UNIVERSE_ROOT),
            DVec3::ZERO,
            19.0,
            1e21,
        ))
        .unwrap();
        tree.insert_sector(SectorNode::new(
            SectorId::COSMIC_WEB,
            "Cosmic Web",
            Some(SectorId::UNIVERSE_ROOT),
            DVec3::ZERO,
            25.0,
            1e26,
        ))
        .unwrap();
        tree.insert_sector(SectorNode::new(
            SectorId::HYPER_LOTUS_25_HORIZON,
            "Final Horizon",
            Some(SectorId::UNIVERSE_ROOT),
            DVec3::new(0.0, 0.0, 0.0),
            28.0,
            1e29,
        ))
        .unwrap();
        tree
    }

    #[test]
    fn zero_jitter_across_minus_35_to_plus_28_orders_of_magnitude() {
        let tree = test_tree();
        // Each u_cam is anchored in the nearest scale sector (|u_cam - sector.scale_exp| <= 7),
        // matching how Nested Scale Shells hand off camera anchors across 63+ orders of magnitude.
        let sectors = [
            (SectorId::PLANCK_ORIGIN, -35.0),
            (SectorId::QUANTUM_FIELD, -25.0),
            (SectorId::QUANTUM_FIELD, -15.0),
            (SectorId::EARTH_SURFACE, -9.0),
            (SectorId::EARTH_SURFACE, 0.0),
            (SectorId::EARTH_ORBIT, 6.8),
            (SectorId::SOLAR_SYSTEM, 13.5),
            (SectorId::MILKY_WAY, 21.0),
            (SectorId::COSMIC_WEB, 25.0),
            (SectorId::HYPER_LOTUS_25_HORIZON, 28.0),
        ];

        for (sector_id, u_cam) in sectors {
            let s_node = tree.get(sector_id).unwrap();
            let render_unit_in_sector = 10.0f64.powf(u_cam - s_node.scale_exp);
            // Put camera far from sector origin (`1.23` sector units) + test sub-millimeter-render step.
            let base = DVec3::new(1.234567, -0.42, 0.875);
            let cam = ScalePose64::new(sector_id, base, u_cam, DQuat::IDENTITY);
            let ent = ScalePose64::new(
                sector_id,
                base + DVec3::new(0.75, -0.25, 1.5) * render_unit_in_sector,
                u_cam,
                DQuat::IDENTITY,
            );
            let projected = cam
                .project_entity(&tree, &ent, 0.1 * 10.0f64.powf(u_cam))
                .unwrap();
            let err = (projected.pos - Vec3::new(0.75, -0.25, 1.5)).length();
            assert!(
                err < 1e-6,
                "jitter too large at u_cam={u_cam} in {:?}: err={err}",
                sector_id
            );
            assert!(!projected.is_far_shell);
        }
    }

    #[test]
    fn far_shell_prevents_f32_overflow_from_planck_to_horizon() {
        let tree = test_tree();
        let cam = ScalePose64::new(SectorId::PLANCK_ORIGIN, DVec3::ZERO, -35.0, DQuat::IDENTITY);
        let horizon = ScalePose64::new(
            SectorId::HYPER_LOTUS_25_HORIZON,
            DVec3::new(0.0, 0.0, -2.0),
            28.0,
            DQuat::IDENTITY,
        );
        let p = cam.project_entity(&tree, &horizon, 1e27).unwrap();
        assert!(p.is_far_shell);
        assert!(p.pos.is_finite());
        assert!((p.pos.length() - FAR_SHELL_PROXY_RADIUS).abs() < 1e-2);
    }

    #[test]
    fn reversed_z_infinite_invariants() {
        let near = 0.04f32;
        let proj = reversed_z_infinite_perspective_rh(90.0f32.to_radians(), 16.0 / 9.0, near);

        let project_z = |d: f32| -> f32 {
            let clip = proj * Vec4::new(0.0, 0.0, -d, 1.0);
            clip.z / clip.w
        };

        assert!((project_z(near) - 1.0).abs() < 1e-6);
        assert!((project_z(near * 2.0) - 0.5).abs() < 1e-6);
        assert!((project_z(1e6) - (near / 1e6)).abs() < 1e-10);
        assert!(project_z(1e6) > 0.0);

        let asym = reversed_z_infinite_asymmetric_rh(-1.0, 1.0, -1.0, 1.0, near);
        let sym = reversed_z_infinite_perspective_rh(90.0f32.to_radians(), 1.0, near);
        assert!((asym - sym).abs_diff_eq(Mat4::ZERO, 1e-6));
    }

    #[test]
    fn frustum_culling_reversed_z_checks() {
        let proj = reversed_z_infinite_perspective_rh(90.0f32.to_radians(), 1.0, 0.04);
        let view = camera_view_matrix(glam::Quat::IDENTITY);
        let frustum = FrustumPlanes::from_reversed_z_view_proj(proj * view);

        assert!(frustum.intersects_sphere(Vec3::new(0.0, 0.0, -10.0), 0.5));
        assert!(frustum.intersects_sphere(Vec3::new(0.0, 0.0, -500_000.0), 1.0));
        assert!(!frustum.intersects_sphere(Vec3::new(50.0, 0.0, -10.0), 1.0));
        assert!(!frustum.intersects_sphere(Vec3::new(0.0, 0.0, 2.0), 0.5));
        assert!(frustum.intersects_aabb(Vec3::new(-1.0, -1.0, -20.0), Vec3::new(1.0, 1.0, -15.0)));
        assert!(!frustum.intersects_aabb(Vec3::new(20.0, 20.0, -10.0), Vec3::new(25.0, 25.0, -5.0)));
    }

    #[test]
    fn multi_sector_reanchor_preserves_projection() {
        let tree = test_tree();
        let mut cam = ScalePose64::new(
            SectorId::EARTH_ORBIT,
            DVec3::new(0.001, 0.0, 0.0), // near Earth center in 10^6.8m units
            10.0,
            DQuat::IDENTITY,
        );
        let sat = ScalePose64::new(
            SectorId::EARTH_ORBIT,
            DVec3::new(0.002, 0.0005, -0.001),
            6.8,
            DQuat::IDENTITY,
        );
        let before = cam.project_entity(&tree, &sat, 6_371_000.0).unwrap();
        cam.reanchor(&tree, SectorId::SOLAR_SYSTEM).unwrap();
        let after = cam.project_entity(&tree, &sat, 6_371_000.0).unwrap();
        assert!((before.pos - after.pos).length() < 1e-6);
    }

    #[test]
    fn scale_spline_interpolates_across_orders() {
        let tree = test_tree();
        let spline = ScaleSpline::new(vec![
            ScaleSplineKeyframe {
                t: 0.0,
                sector_id: SectorId::EARTH_ORBIT,
                local_pos: DVec3::ZERO,
                scale_exp: 6.8,
                orient: DQuat::IDENTITY,
            },
            ScaleSplineKeyframe {
                t: 1.0,
                sector_id: SectorId::SOLAR_SYSTEM,
                local_pos: DVec3::ZERO,
                scale_exp: 13.5,
                orient: DQuat::IDENTITY,
            },
        ]);
        let mid = spline.evaluate(&tree, 0.5).unwrap();
        assert!((mid.scale_exp - 10.15).abs() < 0.05);
    }
}
