use std::collections::HashMap;

use bytemuck::{Pod, Zeroable};
use glam::{DQuat, DVec3};
use serde::{Deserialize, Serialize};

use crate::MathError;

/// Canonical identifier for a hierarchical coordinate sector in `HyperCosmosVR`.
///
/// Each sector normalizes coordinates in its own physical scale (`1 unit = 10^scale_exp meters`),
/// enabling smooth, lossless `f64` subtraction before GPU camera-relative `f32` emission.
#[repr(transparent)]
#[derive(
    Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize, Pod,
    Zeroable,
)]
pub struct SectorId(pub u32);

impl SectorId {
    pub const UNIVERSE_ROOT: Self = Self(0);
    pub const PLANCK_ORIGIN: Self = Self(1);
    pub const QUANTUM_FIELD: Self = Self(2);
    pub const EARTH_CORE: Self = Self(3);
    pub const EARTH_SURFACE: Self = Self(4);
    pub const EARTH_ORBIT: Self = Self(5);
    pub const COMPLEXITY_HELIX: Self = Self(6);
    pub const LIFE_BIOSPHERE: Self = Self(7);
    pub const MIND_CATHEDRAL: Self = Self(8);
    pub const COLORS_PRISM: Self = Self(9);
    pub const MUSIC_HARP: Self = Self(10);
    pub const CULTURE_WEB: Self = Self(11);
    pub const TECH_RING: Self = Self(12);
    pub const FUTURES_ARCS: Self = Self(13);
    pub const SOLAR_SYSTEM: Self = Self(14);
    pub const MILKY_WAY: Self = Self(15);
    pub const ORION_SPUR: Self = Self(16);
    pub const LOCAL_GROUP: Self = Self(17);
    pub const LANIAKEA: Self = Self(18);
    pub const COSMIC_WEB: Self = Self(19);
    pub const OBSERVABLE_CMB: Self = Self(20);
    pub const EARLY_INFLATION: Self = Self(21);
    pub const HYPER_LOTUS_11_EVIDENCE: Self = Self(22);
    pub const HYPER_LOTUS_12_FUTURE: Self = Self(23);
    pub const HYPER_LOTUS_23_PERSPECTIVES: Self = Self(24);
    pub const HYPER_LOTUS_24_MEANING: Self = Self(25);
    pub const HYPER_LOTUS_25_HORIZON: Self = Self(26);
}

/// One hierarchical coordinate sector node in [`SectorTree`].
///
/// Coordinates inside a sector are expressed in canonical units of `10^scale_exp` meters.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SectorNode {
    pub id: SectorId,
    pub name: String,
    pub parent: Option<SectorId>,
    /// Offset of this sector's origin measured in **parent sector's** canonical units.
    pub offset_in_parent: DVec3,
    /// Orientation of this sector's axes relative to its parent sector.
    pub orient_in_parent: DQuat,
    /// Exponent `u` such that `1.0` local unit equals `10^u` meters.
    pub scale_exp: f64,
    /// Intrinsic spatial bounding radius of this sector measured in meters.
    pub radius_m: f64,
}

impl SectorNode {
    /// Creates a new sector node with identity relative rotation.
    pub fn new(
        id: SectorId,
        name: impl Into<String>,
        parent: Option<SectorId>,
        offset_in_parent: DVec3,
        scale_exp: f64,
        radius_m: f64,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            parent,
            offset_in_parent,
            orient_in_parent: DQuat::IDENTITY,
            scale_exp,
            radius_m,
        }
    }
}

/// Tree of nested coordinate frames spanning Planck foam (`u = -35`) to the Hyper-Horizon (`u > +28`).
#[derive(Clone, Debug, Default)]
pub struct SectorTree {
    nodes: HashMap<SectorId, SectorNode>,
}

impl SectorTree {
    /// Creates a tree pre-populated with a single root node (`UNIVERSE_ROOT` at `u = 0`).
    pub fn with_root(scale_exp: f64) -> Self {
        let mut tree = Self::default();
        tree.nodes.insert(
            SectorId::UNIVERSE_ROOT,
            SectorNode::new(
                SectorId::UNIVERSE_ROOT,
                "Universe Root",
                None,
                DVec3::ZERO,
                scale_exp,
                10.0f64.powf(scale_exp),
            ),
        );
        tree
    }

    /// Inserts or replaces the root node (only allowed when `parent` is `None`).
    pub fn insert_root(&mut self, node: SectorNode) -> Result<(), MathError> {
        if node.parent.is_some() {
            return Err(MathError::InvalidSectorHierarchy(
                "root sector must have parent = None".into(),
            ));
        }
        self.nodes.insert(node.id, node);
        Ok(())
    }

    /// Inserts a child sector, validating that `parent` exists and no cycle can form.
    pub fn insert_sector(&mut self, node: SectorNode) -> Result<(), MathError> {
        let Some(parent_id) = node.parent else {
            return self.insert_root(node);
        };
        if parent_id == node.id {
            return Err(MathError::CycleDetected(node.id));
        }
        if !self.nodes.contains_key(&parent_id) {
            return Err(MathError::UnknownSector(parent_id));
        }
        // Walk from parent_id upward to ensure node.id is not an ancestor.
        let mut cursor = Some(parent_id);
        let mut guard = 0usize;
        while let Some(cur) = cursor {
            if cur == node.id {
                return Err(MathError::CycleDetected(node.id));
            }
            guard += 1;
            if guard > 128 {
                return Err(MathError::CycleDetected(cur));
            }
            cursor = self.nodes.get(&cur).and_then(|n| n.parent);
        }
        self.nodes.insert(node.id, node);
        Ok(())
    }

    /// Returns a reference to a registered sector node.
    #[inline]
    pub fn get(&self, id: SectorId) -> Option<&SectorNode> {
        self.nodes.get(&id)
    }

    /// Returns all registered sectors.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &SectorNode> {
        self.nodes.values()
    }

    /// Returns total number of sectors.
    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns true if no sectors are registered.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Builds the chain `[id, parent, ..., root]` and returns `Err` on unknown/cyclic nodes.
    pub fn ancestor_chain(&self, id: SectorId) -> Result<Vec<SectorId>, MathError> {
        let mut chain = Vec::with_capacity(8);
        let mut cursor = Some(id);
        while let Some(cur) = cursor {
            if chain.contains(&cur) || chain.len() >= 128 {
                return Err(MathError::CycleDetected(cur));
            }
            let node = self.nodes.get(&cur).ok_or(MathError::UnknownSector(cur))?;
            chain.push(cur);
            cursor = node.parent;
        }
        Ok(chain)
    }

    /// Finds the Lowest Common Ancestor (`LCA`) of sectors `a` and `b`.
    pub fn lowest_common_ancestor(&self, a: SectorId, b: SectorId) -> Result<SectorId, MathError> {
        if a == b {
            if !self.nodes.contains_key(&a) {
                return Err(MathError::UnknownSector(a));
            }
            return Ok(a);
        }
        let chain_a = self.ancestor_chain(a)?;
        let chain_b = self.ancestor_chain(b)?;
        for anc in &chain_a {
            if chain_b.contains(anc) {
                return Ok(*anc);
            }
        }
        Err(MathError::DisconnectedSectors(a, b))
    }

    /// Transforms `(pos, orient)` expressed in `from_sector` into ancestor `ancestor_sector` units.
    pub fn lift_to_ancestor(
        &self,
        from_sector: SectorId,
        local_pos: DVec3,
        local_orient: DQuat,
        ancestor_sector: SectorId,
    ) -> Result<(DVec3, DQuat), MathError> {
        let mut cur = from_sector;
        let mut pos = local_pos;
        let mut orient = local_orient;
        let mut guard = 0usize;

        while cur != ancestor_sector {
            let child = self.nodes.get(&cur).ok_or(MathError::UnknownSector(cur))?;
            let Some(parent_id) = child.parent else {
                return Err(MathError::DisconnectedSectors(from_sector, ancestor_sector));
            };
            let parent = self
                .nodes
                .get(&parent_id)
                .ok_or(MathError::UnknownSector(parent_id))?;
            let scale_ratio = 10.0f64.powf(child.scale_exp - parent.scale_exp);
            pos = child.offset_in_parent + child.orient_in_parent * (pos * scale_ratio);
            orient = (child.orient_in_parent * orient).normalize();
            cur = parent_id;
            guard += 1;
            if guard > 128 {
                return Err(MathError::CycleDetected(cur));
            }
        }
        Ok((pos, orient))
    }

    /// Transforms `(pos, orient)` from `from_sector` into `to_sector` via their LCA in `f64`.
    pub fn transform_between(
        &self,
        from_sector: SectorId,
        local_pos: DVec3,
        local_orient: DQuat,
        to_sector: SectorId,
    ) -> Result<(DVec3, DQuat), MathError> {
        if from_sector == to_sector {
            if !self.nodes.contains_key(&from_sector) {
                return Err(MathError::UnknownSector(from_sector));
            }
            return Ok((local_pos, local_orient));
        }
        let lca = self.lowest_common_ancestor(from_sector, to_sector)?;
        let (in_lca_pos, in_lca_orient) =
            self.lift_to_ancestor(from_sector, local_pos, local_orient, lca)?;

        // Collect path from to_sector up to lca (excluding lca), then invert step-by-step downward.
        let mut down_path = Vec::with_capacity(8);
        let mut cur = to_sector;
        while cur != lca {
            let node = self.nodes.get(&cur).ok_or(MathError::UnknownSector(cur))?;
            down_path.push(cur);
            cur = node
                .parent
                .ok_or(MathError::DisconnectedSectors(from_sector, to_sector))?;
        }

        let mut pos = in_lca_pos;
        let mut orient = in_lca_orient;
        for sector_id in down_path.into_iter().rev() {
            let child = &self.nodes[&sector_id];
            let parent = &self.nodes[&child.parent.unwrap()];
            let inv_rot = child.orient_in_parent.inverse();
            let scale_ratio = 10.0f64.powf(parent.scale_exp - child.scale_exp);
            pos = (inv_rot * (pos - child.offset_in_parent)) * scale_ratio;
            orient = (inv_rot * orient).normalize();
        }
        Ok((pos, orient))
    }
}

