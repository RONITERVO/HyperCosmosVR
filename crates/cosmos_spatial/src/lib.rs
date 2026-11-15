//! Nested `ScaleShell` sparse octrees, 63-bit Morton spatial hashing, `O(K_visible)` frustum
//! and angular threshold queries, and the 25-experience continuous cosmic manifold topology.

pub mod morton;
pub mod octree;
pub mod scale_shell;
pub mod topology;

pub use morton::{morton_for_local_pos, morton3_decode, morton3_encode};
pub use octree::{EntityKind, LodTier, SparseOctree, SpatialEntity, TraversalStats};
pub use scale_shell::{
    documentary_anchor_radius_m,
    CosmosSpatialIndex, QueryResult, SHELL_ACTIVE_PAD_ABOVE, SHELL_ACTIVE_PAD_BELOW, ScaleShell,
    VisibleEntity,
};
pub use topology::{
    BridgeKind, ContinuityEdge, ExperienceId, ExperiencePlacement, ManifoldArchetype,
    all_experience_placements, build_default_universe_index, continuity_edges,
    earth_layer_local_pose, hsl_to_prism_local, landmark_local_pose, music_filament_local,
    prism_local_to_hsl, solar_body_local_pose, zone_local_pose,
};

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use std::path::Path;
    use std::time::Instant;

    use cosmos_math::{
        ScalePose64, SectorId, camera_view_matrix, reversed_z_infinite_perspective_rh,
    };
    use glam::{DQuat, DVec3};

    use super::*;

    #[test]
    fn morton_roundtrip_exhaustive_samples() {
        for &(x, y, z) in &[
            (0, 0, 0),
            (1, 2, 3),
            (2097151, 2097151, 2097151),
            (12345, 67890, 1048575),
            (0, 1024, 524288),
        ] {
            let code = morton3_encode(x, y, z);
            assert_eq!(morton3_decode(code), (x, y, z));
        }
    }

    #[test]
    fn all_25_experiences_anchored_and_connected() {
        let placements = all_experience_placements();
        assert_eq!(placements.len(), 25);
        let ids: HashSet<u8> = placements.iter().map(|p| p.id.0).collect();
        assert_eq!(ids.len(), 25);
        for id in 1u8..=25 {
            assert!(ids.contains(&id), "missing experience {id}");
        }
        let total_zones: u16 = placements
            .iter()
            .filter(|p| p.id.0 >= 3)
            .map(|p| p.zone_count)
            .sum();
        assert_eq!(total_zones, 376);

        let index = build_default_universe_index();
        // Verify 01 Colors & 02 Music sit cleanly inside 17 Mind hierarchy
        let lca_colors = index
            .tree
            .lowest_common_ancestor(SectorId::MIND_CATHEDRAL, SectorId::COLORS_PRISM)
            .unwrap();
        let lca_music = index
            .tree
            .lowest_common_ancestor(SectorId::MIND_CATHEDRAL, SectorId::MUSIC_HARP)
            .unwrap();
        assert_eq!(lca_colors, SectorId::MIND_CATHEDRAL);
        assert_eq!(lca_music, SectorId::MIND_CATHEDRAL);

        // Verify all continuity edges reference valid 1..=25 experiences
        let edges = continuity_edges();
        assert!(edges.len() >= 24);
        for e in edges {
            assert!((1..=25).contains(&e.from.0));
            assert!((1..=25).contains(&e.to.0));
        }
    }

    #[test]
    fn hsl_prism_and_music_harp_geometry_invariants() {
        for &(h, s, l) in &[(0.0, 100.0, 50.0), (210.0, 75.0, 62.5), (359.0, 10.0, 90.0)] {
            let p = hsl_to_prism_local(h, s, l);
            let (rh, rs, rl) = prism_local_to_hsl(p);
            assert!((rh - h).abs() < 1e-6);
            assert!((rs - s).abs() < 1e-6);
            assert!((rl - l).abs() < 1e-6);
        }

        let index = build_default_universe_index();
        let harp = index.shell(SectorId::MUSIC_HARP).unwrap();
        let filaments = harp
            .octree
            .entities()
            .iter()
            .filter(|e| e.kind == EntityKind::MusicalFilament)
            .count();
        assert_eq!(filaments, 48 * 4);
    }

    #[test]
    fn query_visible_finds_active_scale_and_culls_hidden_scales() {
        let index = build_default_universe_index();
        let proj = reversed_z_infinite_perspective_rh(90.0f32.to_radians(), 16.0 / 9.0, 0.04);
        let view_proj = proj * camera_view_matrix(glam::Quat::IDENTITY);

        // Hover just outside Earth looking down -Z toward Earth center (0, 0, -5 render units @ u=6.8)
        let cam = ScalePose64::new(
            SectorId::EARTH_ORBIT,
            DVec3::new(0.0, 0.0, 5.0),
            6.8,
            DQuat::IDENTITY,
        );
        let q = index
            .query_visible(&cam, view_proj, 0.08_f32.to_radians(), None)
            .unwrap();
        assert!(
            q.visible.iter().any(|v| v.experience_id == 3),
            "Earth (03) should be visible in forward view"
        );
        // Subatomic Planck origin should NOT be queried at u=6.8
        assert!(!q.visible.iter().any(|v| v.experience_id == 14));
        assert!(q.shells_active <= 8);
    }

    #[test]
    fn scaling_100x_growth_keeps_o_k_visible_efficiency() {
        let mut small = build_default_universe_index();
        let mut large = build_default_universe_index();

        let seed_entities = |target: &mut CosmosSpatialIndex, count_per_shell: usize| {
            // Distribute entities across Earth Orbit, Solar System, Milky Way, and Cosmic Web
            let specs = [
                (SectorId::EARTH_ORBIT, 3u8, 6.8, 100_000.0),
                (SectorId::SOLAR_SYSTEM, 4u8, 11.2, 5.0e9),
                (SectorId::MILKY_WAY, 5u8, 20.5, 1.0e18),
                (SectorId::COSMIC_WEB, 8u8, 25.3, 2.0e23),
            ];
            let mut next_id = 10_000u64;
            for &(sector_id, exp_id, scale_exp, radius_m) in &specs {
                for i in 0..count_per_shell {
                    // Spread across [-10, +10]^3 in normalized sector coordinates
                    let a = ((i * 2654435761) & 0xFFFF) as f64 / 65535.0;
                    let b = ((i * 1597334677) & 0xFFFF) as f64 / 65535.0;
                    let c = ((i * 2246822519) & 0xFFFF) as f64 / 65535.0;
                    // Put only ~1% forward in -Z and tiny radius so angular + frustum cull work together
                    let pos = DVec3::new((a - 0.5) * 18.0, (b - 0.5) * 18.0, (c - 0.5) * 18.0);
                    target
                        .insert_entity(SpatialEntity {
                            id: next_id,
                            experience_id: exp_id,
                            zone_index: 0,
                            feature_index: 0,
                            sector_id,
                            local_pos: pos,
                            orient: DQuat::IDENTITY,
                            radius_m: radius_m * 0.02,
                            scale_exp: scale_exp - 1.5,
                            kind: EntityKind::SyntheticCluster,
                            salience_weight: 1.0,
                        })
                        .unwrap();
                    next_id += 1;
                }
            }
        };

        // 2,500 vs 250,000 (100x growth)
        seed_entities(&mut small, 625);
        seed_entities(&mut large, 62_500);
        assert!(large.total_entities() >= 250_000);

        let proj = reversed_z_infinite_perspective_rh(75.0f32.to_radians(), 1.0, 0.04);
        let view_proj = proj * camera_view_matrix(glam::Quat::IDENTITY);
        let cam = ScalePose64::new(
            SectorId::MILKY_WAY,
            DVec3::new(0.0, 0.0, 6.0),
            19.8,
            DQuat::IDENTITY,
        );

        let q_small = small
            .query_visible(&cam, view_proj, 0.25_f32.to_radians(), Some(256))
            .unwrap();
        let t0 = Instant::now();
        let q_large = large
            .query_visible(&cam, view_proj, 0.25_f32.to_radians(), Some(256))
            .unwrap();
        let dt = t0.elapsed();

        println!(
            "100x scaling check: small_total={} (tested={}) -> large_total={} (tested={}, visited_nodes={}, pruned_frustum={}, pruned_angular={}) in {:?}",
            small.total_entities(),
            q_small.stats.entities_tested,
            large.total_entities(),
            q_large.stats.entities_tested,
            q_large.stats.nodes_visited,
            q_large.stats.nodes_pruned_frustum,
            q_large.stats.nodes_pruned_angular,
            dt
        );
        // Frustum + angular pruning + active-shell filtering avoids touching vast majority of 250k entities
        assert!(q_large.stats.entities_tested < large.total_entities() / 4);
        assert!(q_small.visible.len() <= 256);
        assert!(q_large.visible.len() <= 256);
    }

    #[test]
    fn documentary_navigation_frames_fixed_content_instead_of_selector_positions() {
        let mut index = build_default_universe_index();
        for exp_id in [1, 2, 13, 14, 16, 17, 22, 25] {
            let anchor = index.find_experience_anchor(exp_id).unwrap().clone();
            let radius = documentary_anchor_radius_m(&anchor);
            let sector = index.tree.get(anchor.sector_id).unwrap();
            let meters_per_sector = 10.0_f64.powf(sector.scale_exp);
            let (overview, target) = index.navigation_pose_for_documentary_stage(exp_id, DVec3::Z, false).unwrap();
            let (inspection, _) = index.navigation_pose_for_documentary_stage(exp_id, DVec3::Z, true).unwrap();
            assert_eq!(target, anchor.local_pos);
            assert_eq!(overview.scale_exp, anchor.scale_exp);
            assert!(((overview.local_pos - target).length() * meters_per_sector / radius - 3.0).abs() < 1e-10);
            assert!(((inspection.local_pos - target).length() * meters_per_sector / radius - 2.1).abs() < 1e-10);
            let selector = SpatialEntity {
                id: 990_000 + exp_id as u64, experience_id: exp_id, zone_index: 65000, feature_index: 7,
                sector_id: anchor.sector_id, local_pos: anchor.local_pos + DVec3::new(40.0, -20.0, 15.0),
                orient: DQuat::IDENTITY, radius_m: radius * 0.001, scale_exp: anchor.scale_exp - 2.0,
                kind: EntityKind::FeatureNode, salience_weight: 1.0,
            };
            let (selected, selected_target) = index.navigation_pose_for_entity(&selector, DVec3::Z).unwrap();
            assert_eq!(selected.local_pos, inspection.local_pos);
            assert_eq!(selected_target, target);
        }
        let body = SpatialEntity {
            id: 999_981, experience_id: 4, zone_index: 7, feature_index: 0,
            sector_id: SectorId::SOLAR_SYSTEM, local_pos: DVec3::new(4.0, 2.0, -3.0),
            orient: DQuat::IDENTITY, radius_m: 5.0e7, scale_exp: 8.0,
            kind: EntityKind::CelestialBody, salience_weight: 1.0,
        };
        index.insert_entity(body.clone()).unwrap();
        let selector = SpatialEntity { id: 999_982, kind: EntityKind::FeatureNode,
            local_pos: DVec3::new(20.0, 0.0, 0.0), ..body.clone() };
        let (pose, target) = index.navigation_pose_for_entity(&selector, DVec3::Z).unwrap();
        let (body_pose, _) = index.navigation_pose_for_entity(&body, DVec3::Z).unwrap();
        assert_eq!(target, body.local_pos);
        assert_eq!(pose.local_pos, body_pose.local_pos);
    }

    #[test]
    fn verify_provisioned_fonts_hrir_and_blue_noise() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let target_chars = "áéíóúüñÁÉÍÓÚÜÑ¿¡äöåÄÖÅΩ♯⁻²³⁵—→·×";

        // UI Sans, Bold, and Symbol fallback each provide 100% glyph coverage
        for font_name in ["ui_sans.ttf", "ui_sans_bold.ttf", "symbols_fallback.ttf"] {
            let bytes = std::fs::read(root.join("assets/fonts").join(font_name)).unwrap();
            let face = ttf_parser::Face::parse(&bytes, 0).expect("valid font");
            for ch in target_chars.chars() {
                assert!(
                    face.glyph_index(ch).is_some(),
                    "font {font_name} missing glyph '{ch}'"
                );
            }
        }

        // Script + UI Sans fallback chain covers 100% of target_chars
        let script_bytes = std::fs::read(root.join("assets/fonts/display_script_bold.ttf")).unwrap();
        let ui_bytes = std::fs::read(root.join("assets/fonts/ui_sans_bold.ttf")).unwrap();
        let script_face = ttf_parser::Face::parse(&script_bytes, 0).unwrap();
        let ui_face = ttf_parser::Face::parse(&ui_bytes, 0).unwrap();
        for ch in target_chars.chars() {
            assert!(
                script_face.glyph_index(ch).is_some() || ui_face.glyph_index(ch).is_some(),
                "fallback chain missing glyph '{ch}'"
            );
        }

        // IRCAM HRIR sphere loads cleanly at 48 kHz
        let sphere = hrtf::HrirSphere::from_file(root.join("assets/hrir/irc_1040_c.bin"), 48_000)
            .expect("irc_1040_c.bin should load cleanly");
        assert!(sphere.len() > 100);

        // Blue noise 128x128 RGB PNG loads with exact dimensions
        let img = image::open(root.join("assets/textures/blue_noise_128.png"))
            .expect("blue_noise_128.png should decode cleanly")
            .to_rgb8();
        assert_eq!(img.dimensions(), (128, 128));
    }
}
