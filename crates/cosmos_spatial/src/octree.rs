use cosmos_math::{FrustumPlanes, SectorId, apparent_angular_rad};
use glam::{DQuat, DVec3, Vec3};
use serde::{Deserialize, Serialize};

use crate::morton::morton_for_local_pos;

const MAX_LEAF_ENTITIES: usize = 32;
const MAX_OCTREE_DEPTH: u8 = 10;

/// Semantic and rendering category of an entity indexed inside a [`crate::ScaleShell`].
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum EntityKind {
    ExperienceAnchor,
    ZoneShell,
    FeatureNode,
    CelestialBody,
    GalacticLandmark,
    ChromaticSample,
    MusicalFilament,
    SyntheticCluster,
}

/// Strict 3-tier shader LOD ladder (`docs/universe_architecture.md §7.1`).
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LodTier {
    /// Tier 0 (`alpha > 8 deg`): Foveated raymarching / compute hero pass.
    Hero,
    /// Tier 1 (`0.45 deg < alpha <= 8 deg`): Analytical SDF / Fresnel impostor.
    Impostor,
    /// Tier 2 (`alpha <= 0.45 deg`): GPU-instanced blackbody / filament sprite.
    MacroCloud,
}

impl LodTier {
    /// Classifies an apparent angular diameter (radians) into one of the three LOD tiers.
    #[inline]
    pub fn from_angular_rad(alpha_rad: f32) -> Self {
        const EIGHT_DEG: f32 = 8.0 * std::f32::consts::PI / 180.0;
        const IMPOSTOR_DEG: f32 = 0.45 * std::f32::consts::PI / 180.0;
        if alpha_rad > EIGHT_DEG {
            Self::Hero
        } else if alpha_rad > IMPOSTOR_DEG {
            Self::Impostor
        } else {
            Self::MacroCloud
        }
    }
}

/// Spatial record stored inside a [`SparseOctree`] leaf.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpatialEntity {
    pub id: u64,
    pub experience_id: u8,
    pub zone_index: u16,
    pub feature_index: u16,
    pub sector_id: SectorId,
    /// Position in `sector_id` canonical local units (`meters * 10^-sector.scale_exp`).
    pub local_pos: DVec3,
    pub orient: DQuat,
    /// Physical bounding radius in meters.
    pub radius_m: f64,
    /// Intrinsic log10 scale exponent `u = log10(radius_m)`.
    pub scale_exp: f64,
    pub kind: EntityKind,
    /// Multiplier applied during angular threshold & salience sorting (default `1.0`).
    pub salience_weight: f32,
}

#[derive(Clone, Debug)]
struct OctreeNode {
    bounds_min: DVec3,
    bounds_max: DVec3,
    children: [u32; 8],
    is_leaf: bool,
    /// Indices into `SparseOctree::entities` (valid when `is_leaf == true`).
    leaf_items: Vec<u32>,
    /// Largest entity bounding radius inside this entire subtree (in meters).
    max_radius_m: f64,
}

impl OctreeNode {
    fn new_leaf(bounds_min: DVec3, bounds_max: DVec3) -> Self {
        Self {
            bounds_min,
            bounds_max,
            children: [u32::MAX; 8],
            is_leaf: true,
            leaf_items: Vec::new(),
            max_radius_m: 0.0,
        }
    }
}

/// Sparse hierarchical 3D octree with Morton spatial hashing over normalized sector coordinates.
///
/// Internal nodes cache `max_radius_m` so frustum-culled or sub-threshold angular branches are pruned
/// in `O(1)`, keeping frame query cost proportional to visible items (`O(K_visible)`) across 100x growth.
#[derive(Clone, Debug)]
pub struct SparseOctree {
    bounds_min: DVec3,
    bounds_max: DVec3,
    sector_scale_exp: f64,
    nodes: Vec<OctreeNode>,
    entities: Vec<SpatialEntity>,
    morton_keys: Vec<u64>,
}

/// Traversal telemetry returned by [`SparseOctree::query_node_frustum`] / [`crate::CosmosSpatialIndex::query_visible`].
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TraversalStats {
    pub nodes_visited: usize,
    pub nodes_pruned_frustum: usize,
    pub nodes_pruned_angular: usize,
    pub entities_tested: usize,
}

impl SparseOctree {
    /// Creates an empty sparse octree covering `[bounds_min, bounds_max]` in canonical sector units.
    pub fn new(bounds_min: DVec3, bounds_max: DVec3, sector_scale_exp: f64) -> Self {
        Self {
            bounds_min,
            bounds_max,
            sector_scale_exp,
            nodes: vec![OctreeNode::new_leaf(bounds_min, bounds_max)],
            entities: Vec::new(),
            morton_keys: Vec::new(),
        }
    }

    /// Total number of stored entities.
    #[inline]
    pub fn len(&self) -> usize {
        self.entities.len()
    }

    /// True if no entities are stored.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.entities.is_empty()
    }

    /// Total allocated octree nodes.
    #[inline]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Read-only access to all entities stored in this octree.
    #[inline]
    pub fn entities(&self) -> &[SpatialEntity] {
        &self.entities
    }

    /// Inserts an entity into the sparse octree and computes its 63-bit Morton code.
    pub fn insert(&mut self, entity: SpatialEntity) {
        let idx = self.entities.len() as u32;
        let morton = morton_for_local_pos(
            entity.local_pos.as_vec3(),
            self.bounds_min.as_vec3(),
            (self.bounds_max - self.bounds_min).as_vec3().max(Vec3::splat(1e-6)),
        );
        self.morton_keys.push(morton);
        self.entities.push(entity);
        self.insert_into_node(0, idx, 0);
    }

    /// Batch inserts entities (growing internal octree capacity cleanly).
    pub fn extend<I: IntoIterator<Item = SpatialEntity>>(&mut self, iter: I) {
        for item in iter {
            self.insert(item);
        }
    }

    fn insert_into_node(&mut self, node_idx: usize, entity_idx: u32, depth: u8) {
        let ent_radius = self.entities[entity_idx as usize].radius_m;
        if ent_radius > self.nodes[node_idx].max_radius_m {
            self.nodes[node_idx].max_radius_m = ent_radius;
        }

        if self.nodes[node_idx].is_leaf {
            self.nodes[node_idx].leaf_items.push(entity_idx);
            if self.nodes[node_idx].leaf_items.len() > MAX_LEAF_ENTITIES && depth < MAX_OCTREE_DEPTH
            {
                self.subdivide(node_idx, depth);
            }
            return;
        }

        let child_slot = self.child_index_for(node_idx, self.entities[entity_idx as usize].local_pos);
        let child_node_idx = self.ensure_child(node_idx, child_slot);
        self.insert_into_node(child_node_idx, entity_idx, depth + 1);
    }

    fn child_index_for(&self, node_idx: usize, pos: DVec3) -> usize {
        let node = &self.nodes[node_idx];
        let mid = (node.bounds_min + node.bounds_max) * 0.5;
        let mut oct = 0usize;
        if pos.x >= mid.x {
            oct |= 1;
        }
        if pos.y >= mid.y {
            oct |= 2;
        }
        if pos.z >= mid.z {
            oct |= 4;
        }
        oct
    }

    fn ensure_child(&mut self, parent_idx: usize, octant: usize) -> usize {
        let existing = self.nodes[parent_idx].children[octant];
        if existing != u32::MAX {
            return existing as usize;
        }
        let p_min = self.nodes[parent_idx].bounds_min;
        let p_max = self.nodes[parent_idx].bounds_max;
        let mid = (p_min + p_max) * 0.5;

        let c_min = DVec3::new(
            if octant & 1 != 0 { mid.x } else { p_min.x },
            if octant & 2 != 0 { mid.y } else { p_min.y },
            if octant & 4 != 0 { mid.z } else { p_min.z },
        );
        let c_max = DVec3::new(
            if octant & 1 != 0 { p_max.x } else { mid.x },
            if octant & 2 != 0 { p_max.y } else { mid.y },
            if octant & 4 != 0 { p_max.z } else { mid.z },
        );
        let new_idx = self.nodes.len() as u32;
        self.nodes.push(OctreeNode::new_leaf(c_min, c_max));
        self.nodes[parent_idx].children[octant] = new_idx;
        new_idx as usize
    }

    fn subdivide(&mut self, node_idx: usize, depth: u8) {
        let items = std::mem::take(&mut self.nodes[node_idx].leaf_items);
        self.nodes[node_idx].is_leaf = false;
        for entity_idx in items {
            let child_slot =
                self.child_index_for(node_idx, self.entities[entity_idx as usize].local_pos);
            let child_node_idx = self.ensure_child(node_idx, child_slot);
            self.insert_into_node(child_node_idx, entity_idx, depth + 1);
        }
    }

    /// Queries visible candidate indices inside this octree given camera position in this sector's units,
    /// camera `u_cam`, frustum planes (in camera-relative render units), and minimum angular size `min_angular_rad`.
    pub fn query_frustum(
        &self,
        cam_in_sector: DVec3,
        u_cam: f64,
        frustum: &FrustumPlanes,
        min_angular_rad: f64,
        out_indices: &mut Vec<u32>,
        stats: &mut TraversalStats,
    ) {
        if self.entities.is_empty() {
            return;
        }
        let scale_to_render = 10.0f64.powf(self.sector_scale_exp - u_cam);
        let inv_cam_m = 10.0f64.powf(-u_cam);
        let mut stack = Vec::with_capacity(64);
        stack.push(0usize);

        while let Some(node_idx) = stack.pop() {
            stats.nodes_visited += 1;
            let node = &self.nodes[node_idx];

            // Transform node AABB to camera-relative f32 render units safely.
            let rel_min_f64 = (node.bounds_min - cam_in_sector) * scale_to_render;
            let rel_max_f64 = (node.bounds_max - cam_in_sector) * scale_to_render;
            let rel_min = rel_min_f64.as_vec3();
            let rel_max = rel_max_f64.as_vec3();

            if !frustum.intersects_aabb(rel_min, rel_max) {
                stats.nodes_pruned_frustum += 1;
                continue;
            }

            // Minimum distance from camera (0, 0, 0) to node AABB in render units.
            let closest = DVec3::ZERO.clamp(rel_min_f64, rel_max_f64);
            let min_dist_render = closest.length();
            let max_r_render = node.max_radius_m * inv_cam_m;
            if min_dist_render > max_r_render
                && apparent_angular_rad(max_r_render, min_dist_render) < min_angular_rad * 0.5
            {
                stats.nodes_pruned_angular += 1;
                continue;
            }

            if node.is_leaf {
                for &idx in &node.leaf_items {
                    stats.entities_tested += 1;
                    let ent = &self.entities[idx as usize];
                    let rel_pos_f64 = (ent.local_pos - cam_in_sector) * scale_to_render;
                    let ent_r_render = ent.radius_m * inv_cam_m;
                    let dist = rel_pos_f64.length();
                    let eff_threshold = min_angular_rad / (ent.salience_weight.max(0.05) as f64);
                    if apparent_angular_rad(ent_r_render, dist) < eff_threshold {
                        continue;
                    }
                    if frustum.intersects_sphere(rel_pos_f64.as_vec3(), ent_r_render as f32) {
                        out_indices.push(idx);
                    }
                }
            } else {
                for &c in &node.children {
                    if c != u32::MAX {
                        stack.push(c as usize);
                    }
                }
            }
        }
    }
}

