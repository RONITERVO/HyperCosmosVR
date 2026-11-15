use std::collections::HashMap;

use cosmos_math::{
    CameraRelativePose, FrustumPlanes, MathError, ScalePose64, SectorId, SectorTree,
};
use glam::{DVec3, Mat4};
use rayon::prelude::*;

use crate::octree::{LodTier, SparseOctree, SpatialEntity, TraversalStats};

/// Padding (in log10 meters) below `u_min` and above `u_max` where a ScaleShell remains active.
pub const SHELL_ACTIVE_PAD_BELOW: f64 = 1.5;
pub const SHELL_ACTIVE_PAD_ABOVE: f64 = 2.5;

/// Radius of authored documentary content, distinct from the much broader
/// spatial-query bounds used to keep an experience discoverable across scales.
/// Rendering and navigation share this value so approaching a stage never
/// changes its size and an inspection camera frames the stage that is drawn.
pub fn documentary_anchor_radius_m(entity: &SpatialEntity) -> f64 {
    if entity.kind == crate::octree::EntityKind::ExperienceAnchor && entity.experience_id != 3 {
        entity.radius_m.min(1.5 * 10.0_f64.powf(entity.scale_exp))
    } else {
        entity.radius_m
    }
}

/// Self-contained normalized scale band `[u_min, u_max]` backed by a [`SparseOctree`].
#[derive(Clone, Debug)]
pub struct ScaleShell {
    pub sector_id: SectorId,
    pub u_min: f64,
    pub u_max: f64,
    pub octree: SparseOctree,
}

impl ScaleShell {
    /// Creates a new [`ScaleShell`] spanning local bounds `[-half_extent, +half_extent]^3`.
    pub fn new(
        sector_id: SectorId,
        sector_scale_exp: f64,
        u_min: f64,
        u_max: f64,
        half_extent: f64,
    ) -> Self {
        let min = DVec3::splat(-half_extent);
        let max = DVec3::splat(half_extent);
        Self {
            sector_id,
            u_min,
            u_max,
            octree: SparseOctree::new(min, max, sector_scale_exp),
        }
    }

    /// True if `u_cam` lies inside `[u_min - 1.5, u_max + 2.5]`.
    #[inline]
    pub fn is_active_for_scale(&self, u_cam: f64) -> bool {
        u_cam >= (self.u_min - SHELL_ACTIVE_PAD_BELOW)
            && u_cam <= (self.u_max + SHELL_ACTIVE_PAD_ABOVE)
    }
}

/// GPU-ready visible entity projected relative to the active camera pose.
#[derive(Clone, Debug)]
pub struct VisibleEntity {
    pub id: u64,
    pub experience_id: u8,
    pub zone_index: u16,
    pub feature_index: u16,
    pub kind: crate::octree::EntityKind,
    pub pose: CameraRelativePose,
    pub lod: LodTier,
    pub salience_score: f32,
}

/// Output of [`CosmosSpatialIndex::query_visible`].
#[derive(Clone, Debug, Default)]
pub struct QueryResult {
    pub visible: Vec<VisibleEntity>,
    pub shells_active: usize,
    pub stats: TraversalStats,
}

/// Hierarchical universe spatial index partitioning the 62+ order cosmos into Nested Scale Shells.
#[derive(Clone, Debug)]
pub struct CosmosSpatialIndex {
    pub tree: SectorTree,
    pub shells: Vec<ScaleShell>,
    sector_to_shell: HashMap<SectorId, usize>,
}

impl CosmosSpatialIndex {
    /// Constructs a spatial index from a pre-populated [`SectorTree`] and `ScaleShell`s.
    pub fn new(tree: SectorTree, shells: Vec<ScaleShell>) -> Self {
        let sector_to_shell = shells
            .iter()
            .enumerate()
            .map(|(idx, s)| (s.sector_id, idx))
            .collect();
        Self {
            tree,
            shells,
            sector_to_shell,
        }
    }

    /// Looks up a mutable reference to the shell owning `sector_id`.
    #[inline]
    pub fn shell_mut(&mut self, sector_id: SectorId) -> Option<&mut ScaleShell> {
        self.sector_to_shell
            .get(&sector_id)
            .copied()
            .map(|idx| &mut self.shells[idx])
    }

    /// Looks up an immutable reference to the shell owning `sector_id`.
    #[inline]
    pub fn shell(&self, sector_id: SectorId) -> Option<&ScaleShell> {
        self.sector_to_shell
            .get(&sector_id)
            .copied()
            .map(|idx| &self.shells[idx])
    }

    /// Inserts a [`SpatialEntity`] into the [`ScaleShell`] matching `entity.sector_id`.
    pub fn insert_entity(&mut self, entity: SpatialEntity) -> Result<(), MathError> {
        let sid = entity.sector_id;
        let shell = self.shell_mut(sid).ok_or(MathError::UnknownSector(sid))?;
        shell.octree.insert(entity);
        Ok(())
    }

    /// Batch inserts entities into matching sector shells.
    pub fn extend_entities<I: IntoIterator<Item = SpatialEntity>>(
        &mut self,
        entities: I,
    ) -> Result<(), MathError> {
        for e in entities {
            self.insert_entity(e)?;
        }
        Ok(())
    }

    /// Selects the closest matching active [`SectorId`] for camera anchoring at `u_cam`.
    pub fn best_anchor_sector_for_scale(&self, u_cam: f64) -> SectorId {
        let mut best_id = SectorId::UNIVERSE_ROOT;
        let mut best_dist = f64::MAX;
        for shell in &self.shells {
            if !shell.is_active_for_scale(u_cam) {
                continue;
            }
            if let Some(node) = self.tree.get(shell.sector_id) {
                let d = (node.scale_exp - u_cam).abs();
                if d < best_dist {
                    best_dist = d;
                    best_id = shell.sector_id;
                }
            }
        }
        best_id
    }

    /// Executes `O(K_visible)` frustum + angular visibility query across only the active [`ScaleShell`]s.
    ///
    /// Runs octree traversals in parallel across active shells with `rayon` and projects surviving
    /// entities cleanly into camera-relative coordinates.
    pub fn query_visible(
        &self,
        camera: &ScalePose64,
        view_proj: Mat4,
        min_angular_rad: f32,
        max_results: Option<usize>,
    ) -> Result<QueryResult, MathError> {
        let frustum = FrustumPlanes::from_reversed_z_view_proj(view_proj);
        let active_shells: Vec<&ScaleShell> = self
            .shells
            .iter()
            .filter(|s| s.is_active_for_scale(camera.scale_exp) && !s.octree.is_empty())
            .collect();

        let shells_active = active_shells.len();
        let per_shell: Vec<Result<(Vec<VisibleEntity>, TraversalStats), MathError>> = active_shells
            .par_iter()
            .map(|shell| {
                let (cam_in_shell, _) = self.tree.transform_between(
                    camera.sector_id,
                    camera.local_pos,
                    camera.orient,
                    shell.sector_id,
                )?;
                let mut indices = Vec::with_capacity(64);
                let mut stats = TraversalStats::default();
                shell.octree.query_frustum(
                    cam_in_shell,
                    camera.scale_exp,
                    &frustum,
                    min_angular_rad as f64,
                    &mut indices,
                    &mut stats,
                );

                let ents = shell.octree.entities();
                let mut projected = Vec::with_capacity(indices.len());
                for idx in indices {
                    let e = &ents[idx as usize];
                    let ent_pose =
                        ScalePose64::new(e.sector_id, e.local_pos, e.scale_exp, e.orient);
                    let rel = camera.project_entity(&self.tree, &ent_pose, e.radius_m)?;
                    let lod = LodTier::from_angular_rad(rel.angular_rad);
                    let salience = rel.angular_rad * e.salience_weight;
                    projected.push(VisibleEntity {
                        id: e.id,
                        experience_id: e.experience_id,
                        zone_index: e.zone_index,
                        feature_index: e.feature_index,
                        kind: e.kind,
                        pose: rel,
                        lod,
                        salience_score: salience,
                    });
                }
                Ok((projected, stats))
            })
            .collect();

        let mut result = QueryResult {
            visible: Vec::new(),
            shells_active,
            stats: TraversalStats::default(),
        };
        for item in per_shell {
            let (mut vis, st) = item?;
            result.visible.append(&mut vis);
            result.stats.nodes_visited += st.nodes_visited;
            result.stats.nodes_pruned_frustum += st.nodes_pruned_frustum;
            result.stats.nodes_pruned_angular += st.nodes_pruned_angular;
            result.stats.entities_tested += st.entities_tested;
        }

        result
            .visible
            .sort_by(|a, b| b.salience_score.total_cmp(&a.salience_score));
        if let Some(limit) = max_results
            && result.visible.len() > limit
        {
            result.visible.truncate(limit);
        }
        Ok(result)
    }

    /// Total number of indexed entities across all scale shells.
    pub fn total_entities(&self) -> usize {
        self.shells.iter().map(|s| s.octree.len()).sum()
    }

    /// Iterates immutably over every [`SpatialEntity`] across all scale shells.
    pub fn iter_entities(&self) -> impl Iterator<Item = &SpatialEntity> {
        self.shells.iter().flat_map(|s| s.octree.entities().iter())
    }

    /// Finds the primary [`EntityKind::ExperienceAnchor`] for `experience_id` (`1..=25`).
    pub fn find_experience_anchor(&self, experience_id: u8) -> Option<&SpatialEntity> {
        self.iter_entities().find(|e| {
            e.experience_id == experience_id && e.kind == crate::octree::EntityKind::ExperienceAnchor
        })
    }

    /// Finds the spatial zone/body/landmark entity for `(experience_id, zone_index)`.
    pub fn find_zone_entity(&self, experience_id: u8, zone_index: u16) -> Option<&SpatialEntity> {
        self.iter_entities().find(|e| {
            e.experience_id == experience_id
                && e.zone_index == zone_index
                && matches!(
                    e.kind,
                    crate::octree::EntityKind::CelestialBody
                        | crate::octree::EntityKind::GalacticLandmark
                        | crate::octree::EntityKind::ZoneShell
                )
        })
    }

    /// Finds the exact navigable [`EntityKind::FeatureNode`] for `(experience_id, zone_index, feature_index)`.
    pub fn find_feature_entity(
        &self,
        experience_id: u8,
        zone_index: u16,
        feature_index: u16,
    ) -> Option<&SpatialEntity> {
        self.iter_entities().find(|e| {
            e.experience_id == experience_id
                && e.zone_index == zone_index
                && e.feature_index == feature_index
                && e.kind == crate::octree::EntityKind::FeatureNode
        })
    }

    /// Returns all navigable zone/body/landmark entities for `experience_id`, sorted by `zone_index`.
    pub fn zone_entities_for_experience(&self, experience_id: u8) -> Vec<&SpatialEntity> {
        let mut out: Vec<&SpatialEntity> = self
            .iter_entities()
            .filter(|e| {
                e.experience_id == experience_id
                    && matches!(
                        e.kind,
                        crate::octree::EntityKind::CelestialBody
                            | crate::octree::EntityKind::GalacticLandmark
                            | crate::octree::EntityKind::ZoneShell
                    )
            })
            .collect();
        out.sort_by_key(|e| (e.zone_index, e.id));
        out
    }

    /// Returns all navigable [`EntityKind::FeatureNode`] entities inside `(experience_id, zone_index)`.
    pub fn feature_entities_for_zone(
        &self,
        experience_id: u8,
        zone_index: u16,
    ) -> Vec<&SpatialEntity> {
        let mut out: Vec<&SpatialEntity> = self
            .iter_entities()
            .filter(|e| {
                e.experience_id == experience_id
                    && e.zone_index == zone_index
                    && e.kind == crate::octree::EntityKind::FeatureNode
            })
            .collect();
        out.sort_by_key(|e| (e.feature_index, e.id));
        out
    }

    /// Frames a canonical documentary stage in its authored world coordinate
    /// system. Selector nodes identify content; they are not physical exhibits.
    pub fn navigation_pose_for_documentary_stage(
        &self,
        experience_id: u8,
        view_dir_from_anchor: DVec3,
        inspection: bool,
    ) -> Option<(ScalePose64, DVec3)> {
        let anchor = self.find_experience_anchor(experience_id)?;
        let sector = self.tree.get(anchor.sector_id)?;
        let dir = view_dir_from_anchor.try_normalize().unwrap_or(DVec3::Z);
        let distance_m = documentary_anchor_radius_m(anchor) * if inspection { 2.1 } else { 3.0 };
        let cam_local = anchor.local_pos + dir * distance_m * 10.0_f64.powf(-sector.scale_exp);
        let fwd = -dir;
        let right = fwd.cross(DVec3::Y).try_normalize().unwrap_or(DVec3::X);
        let up = right.cross(fwd).normalize();
        let orient = glam::DQuat::from_mat3(&glam::DMat3::from_cols(right, up, -fwd));
        Some((ScalePose64::new(anchor.sector_id, cam_local, anchor.scale_exp, orient), anchor.local_pos))
    }

    /// Computes a smooth inspection `(ScalePose64, look_target_in_sector)` framing `entity` from
    /// `view_dir_from_entity` at ~2.6 apparent radii inside the entity's owning sector.
    pub fn navigation_pose_for_entity(
        &self,
        entity: &SpatialEntity,
        view_dir_from_entity: DVec3,
    ) -> Option<(ScalePose64, DVec3)> {
        // Catalog feature coordinates organize selection, not the position of
        // the physical parent or the authored explanatory stage.
        if entity.kind == crate::octree::EntityKind::FeatureNode && entity.experience_id != 3 {
            if let Some(parent) = self.zone_entities_for_experience(entity.experience_id).into_iter()
                .find(|e| e.zone_index == entity.zone_index && matches!(e.kind,
                    crate::octree::EntityKind::CelestialBody | crate::octree::EntityKind::GalacticLandmark))
            {
                return self.navigation_pose_for_entity(parent, view_dir_from_entity);
            }
            return self.navigation_pose_for_documentary_stage(entity.experience_id, view_dir_from_entity, true);
        }
        if entity.kind == crate::octree::EntityKind::ZoneShell && entity.experience_id != 3 {
            return self.navigation_pose_for_documentary_stage(entity.experience_id, view_dir_from_entity, false);
        }
        let sector = self.tree.get(entity.sector_id)?;
        let dir = view_dir_from_entity.try_normalize().unwrap_or(DVec3::Z);
        let cam_u = (entity.scale_exp + 0.55).clamp(sector.scale_exp - 2.0, sector.scale_exp + 1.5);
        let radius_at_cam_u = entity.radius_m * 10.0_f64.powf(-cam_u);
        let dist_cam_units = (radius_at_cam_u * 2.6).clamp(1.2, 6.0);
        let sector_per_cam = 10.0_f64.powf(cam_u - sector.scale_exp);
        let cam_local = entity.local_pos + dir * (dist_cam_units * sector_per_cam);
        let fwd = (-dir).normalize_or(DVec3::NEG_Z);
        let right = fwd.cross(DVec3::Y).try_normalize().unwrap_or(DVec3::X);
        let up = right.cross(fwd).normalize();
        let orient = glam::DQuat::from_mat3(&glam::DMat3::from_cols(right, up, -fwd));
        Some((
            ScalePose64::new(entity.sector_id, cam_local, cam_u, orient),
            entity.local_pos,
        ))
    }
}
