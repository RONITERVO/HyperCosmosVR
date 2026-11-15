use cosmos_math::{SectorId, SectorNode, SectorTree};
use glam::{DQuat, DVec3};
use serde::{Deserialize, Serialize};

use crate::octree::{EntityKind, SpatialEntity};
use crate::scale_shell::{CosmosSpatialIndex, ScaleShell};

/// Canonical 1..=25 experience identifier matching `Spanish-Quick-Apps` `01..25`.
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct ExperienceId(pub u8);

/// Visual shader/renderer archetype assigned to an experience region.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum ManifoldArchetype {
    VolumetricChromaticPrism, // 01 Colors
    CymaticHarmonicHarp,      // 02 Music
    PlanetaryCutaway,         // 03 Deep Earth
    HeliocentricSystem,       // 04 Solar System
    SpiralGalaxy,             // 05 Milky Way, 06 Local Group
    GravitationalFlowBasin,   // 07 Laniakea
    VoronoiCosmicWeb,         // 08 Cosmic Web
    DeepFieldSphericalShell,  // 09 Observable, 10 Early Universe
    QuantumSubatomicLattice,  // 13 Matter, 14 Spacetime
    EmergenceHelix,           // 15 Complexity, 16 Life
    ConnectomeCathedral,      // 17 Mind
    OrbitalCivilizationWeb,   // 18 Culture, 19 Technology, 20 Futures
    ExoplanetBiosphereBelt,   // 21 Cosmic Life, 22 Cosmic Contact
    HyperHorizonLotusWing,    // 11 Evidence, 12 Future, 23 Perspectives, 24 Meaning, 25 Horizon
}

/// Navigational bridge tying continuous cosmic domains together.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ContinuityEdge {
    pub from: ExperienceId,
    pub to: ExperienceId,
    pub kind: BridgeKind,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum BridgeKind {
    ScaleDiveInward,
    ScaleRiseOutward,
    EmergenceHelix,
    NestedSensoryLobe,
    EmbeddedStellarArm,
    HyperHorizonLotusPetal,
    OuroborosReturn,
}

/// Complete topological descriptor for one of the 25 cosmic experiences.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExperiencePlacement {
    pub id: ExperienceId,
    pub slug: &'static str,
    pub title: &'static str,
    pub sector_id: SectorId,
    pub anchor_local: DVec3,
    pub u_min: f64,
    pub u_max: f64,
    pub focus_scale_exp: f64,
    pub bounding_radius_m: f64,
    pub zone_count: u16,
    pub archetype: ManifoldArchetype,
}

/// Returns the canonical static placement descriptors for all 25 experiences (`01..25`).
pub fn all_experience_placements() -> Vec<ExperiencePlacement> {
    vec![
        // Nested inside 17 Mind (Sensación -> Luz & Sonido)
        ExperiencePlacement {
            id: ExperienceId(1),
            slug: "01_colors",
            title: "Colores · Prisma Cromático",
            sector_id: SectorId::COLORS_PRISM,
            anchor_local: DVec3::ZERO,
            u_min: 0.5,
            u_max: 3.5,
            focus_scale_exp: 2.0,
            bounding_radius_m: 120.0,
            zone_count: 1,
            archetype: ManifoldArchetype::VolumetricChromaticPrism,
        },
        ExperiencePlacement {
            id: ExperienceId(2),
            slug: "02_music",
            title: "Música · Arpa Cimática",
            sector_id: SectorId::MUSIC_HARP,
            anchor_local: DVec3::ZERO,
            u_min: 0.5,
            u_max: 3.5,
            focus_scale_exp: 2.0,
            bounding_radius_m: 120.0,
            zone_count: 1,
            archetype: ManifoldArchetype::CymaticHarmonicHarp,
        },
        // Physical Earth-outward & inward backbone
        ExperiencePlacement {
            id: ExperienceId(3),
            slug: "03_deep_earth",
            title: "Tierra profunda",
            sector_id: SectorId::EARTH_ORBIT,
            anchor_local: DVec3::ZERO,
            u_min: 0.0,
            u_max: 7.4,
            focus_scale_exp: 6.8,
            bounding_radius_m: 100_000_000.0,
            zone_count: 14,
            archetype: ManifoldArchetype::PlanetaryCutaway,
        },
        ExperiencePlacement {
            id: ExperienceId(4),
            slug: "04_solar_system",
            title: "Sistema Solar",
            sector_id: SectorId::SOLAR_SYSTEM,
            anchor_local: DVec3::ZERO,
            u_min: 6.8,
            u_max: 13.6,
            focus_scale_exp: 11.2,
            bounding_radius_m: 4.0e13,
            zone_count: 12,
            archetype: ManifoldArchetype::HeliocentricSystem,
        },
        ExperiencePlacement {
            id: ExperienceId(5),
            slug: "05_milky_way",
            title: "Vía Láctea",
            sector_id: SectorId::MILKY_WAY,
            anchor_local: DVec3::ZERO,
            u_min: 13.6,
            u_max: 21.1,
            focus_scale_exp: 20.5,
            bounding_radius_m: 1.5e21,
            zone_count: 13,
            archetype: ManifoldArchetype::SpiralGalaxy,
        },
        ExperiencePlacement {
            id: ExperienceId(6),
            slug: "06_local_group",
            title: "Grupo Local",
            sector_id: SectorId::LOCAL_GROUP,
            anchor_local: DVec3::ZERO,
            u_min: 21.1,
            u_max: 22.6,
            focus_scale_exp: 22.0,
            bounding_radius_m: 4.5e22,
            zone_count: 9,
            archetype: ManifoldArchetype::SpiralGalaxy,
        },
        ExperiencePlacement {
            id: ExperienceId(7),
            slug: "07_laniakea",
            title: "Laniakea",
            sector_id: SectorId::LANIAKEA,
            anchor_local: DVec3::ZERO,
            u_min: 22.6,
            u_max: 24.5,
            focus_scale_exp: 23.8,
            bounding_radius_m: 2.6e24,
            zone_count: 10,
            archetype: ManifoldArchetype::GravitationalFlowBasin,
        },
        ExperiencePlacement {
            id: ExperienceId(8),
            slug: "08_cosmic_web",
            title: "Red Cósmica",
            sector_id: SectorId::COSMIC_WEB,
            anchor_local: DVec3::ZERO,
            u_min: 24.5,
            u_max: 26.0,
            focus_scale_exp: 25.3,
            bounding_radius_m: 5.0e25,
            zone_count: 11,
            archetype: ManifoldArchetype::VoronoiCosmicWeb,
        },
        ExperiencePlacement {
            id: ExperienceId(9),
            slug: "09_observable_universe",
            title: "Universo Observable",
            sector_id: SectorId::OBSERVABLE_CMB,
            anchor_local: DVec3::ZERO,
            u_min: 26.0,
            u_max: 26.6,
            focus_scale_exp: 26.4,
            bounding_radius_m: 4.4e26,
            zone_count: 11,
            archetype: ManifoldArchetype::DeepFieldSphericalShell,
        },
        ExperiencePlacement {
            id: ExperienceId(10),
            slug: "10_early_universe",
            title: "Universo Temprano",
            sector_id: SectorId::EARLY_INFLATION,
            anchor_local: DVec3::ZERO,
            u_min: 26.6,
            u_max: 27.2,
            focus_scale_exp: 26.9,
            bounding_radius_m: 1.5e27,
            zone_count: 9,
            archetype: ManifoldArchetype::DeepFieldSphericalShell,
        },
        // Hyper-Horizon Lotus Wings (11, 12, 23, 24, 25)
        ExperiencePlacement {
            id: ExperienceId(11),
            slug: "11_cosmic_evidence",
            title: "Evidencia Cósmica",
            sector_id: SectorId::HYPER_LOTUS_11_EVIDENCE,
            anchor_local: DVec3::ZERO,
            u_min: 27.2,
            u_max: 28.6,
            focus_scale_exp: 27.6,
            bounding_radius_m: 6.0e27,
            zone_count: 9,
            archetype: ManifoldArchetype::HyperHorizonLotusWing,
        },
        ExperiencePlacement {
            id: ExperienceId(12),
            slug: "12_future_of_universe",
            title: "Futuro del Universo",
            sector_id: SectorId::HYPER_LOTUS_12_FUTURE,
            anchor_local: DVec3::ZERO,
            u_min: 27.2,
            u_max: 28.6,
            focus_scale_exp: 27.6,
            bounding_radius_m: 6.0e27,
            zone_count: 10,
            archetype: ManifoldArchetype::HyperHorizonLotusWing,
        },
        // Micro / Planck Quantum Root
        ExperiencePlacement {
            id: ExperienceId(13),
            slug: "13_matter_energy",
            title: "Materia y Energía",
            sector_id: SectorId::QUANTUM_FIELD,
            anchor_local: DVec3::ZERO,
            u_min: -22.0,
            u_max: 0.0,
            focus_scale_exp: -11.0,
            bounding_radius_m: 1.0,
            zone_count: 12,
            archetype: ManifoldArchetype::QuantumSubatomicLattice,
        },
        ExperiencePlacement {
            id: ExperienceId(14),
            slug: "14_spacetime",
            title: "Espacio-tiempo",
            sector_id: SectorId::PLANCK_ORIGIN,
            anchor_local: DVec3::ZERO,
            u_min: -35.0,
            u_max: -22.0,
            focus_scale_exp: -30.0,
            bounding_radius_m: 1.0e-21,
            zone_count: 15,
            archetype: ManifoldArchetype::QuantumSubatomicLattice,
        },
        // Emergence Helix -> Biosphere -> Mind -> Civilization -> Futures
        ExperiencePlacement {
            id: ExperienceId(15),
            slug: "15_complexity",
            title: "Complejidad",
            sector_id: SectorId::COMPLEXITY_HELIX,
            anchor_local: DVec3::ZERO,
            u_min: -9.0,
            u_max: -5.0,
            focus_scale_exp: -7.0,
            bounding_radius_m: 1.0e-4,
            zone_count: 16,
            archetype: ManifoldArchetype::EmergenceHelix,
        },
        ExperiencePlacement {
            id: ExperienceId(16),
            slug: "16_life",
            title: "Vida",
            sector_id: SectorId::LIFE_BIOSPHERE,
            anchor_local: DVec3::ZERO,
            u_min: -6.0,
            u_max: 2.0,
            focus_scale_exp: -1.0,
            bounding_radius_m: 200.0,
            zone_count: 19,
            archetype: ManifoldArchetype::EmergenceHelix,
        },
        ExperiencePlacement {
            id: ExperienceId(17),
            slug: "17_mind",
            title: "Mente",
            sector_id: SectorId::MIND_CATHEDRAL,
            anchor_local: DVec3::ZERO,
            u_min: 1.5,
            u_max: 4.5,
            focus_scale_exp: 2.8,
            bounding_radius_m: 25_000.0,
            zone_count: 18,
            archetype: ManifoldArchetype::ConnectomeCathedral,
        },
        ExperiencePlacement {
            id: ExperienceId(18),
            slug: "18_culture",
            title: "Cultura",
            sector_id: SectorId::CULTURE_WEB,
            anchor_local: DVec3::ZERO,
            u_min: 4.5,
            u_max: 6.9,
            focus_scale_exp: 6.2,
            bounding_radius_m: 6.4e6,
            zone_count: 20,
            archetype: ManifoldArchetype::OrbitalCivilizationWeb,
        },
        ExperiencePlacement {
            id: ExperienceId(19),
            slug: "19_technology",
            title: "Tecnología",
            sector_id: SectorId::TECH_RING,
            anchor_local: DVec3::ZERO,
            u_min: 6.9,
            u_max: 8.2,
            focus_scale_exp: 7.6,
            bounding_radius_m: 3.84e8,
            zone_count: 20,
            archetype: ManifoldArchetype::OrbitalCivilizationWeb,
        },
        ExperiencePlacement {
            id: ExperienceId(20),
            slug: "20_futures",
            title: "Futuros",
            sector_id: SectorId::FUTURES_ARCS,
            anchor_local: DVec3::ZERO,
            u_min: 8.2,
            u_max: 16.0,
            focus_scale_exp: 11.8,
            bounding_radius_m: 1.0e16,
            zone_count: 19,
            archetype: ManifoldArchetype::OrbitalCivilizationWeb,
        },
        // Embedded inside Orion Spur (Milky Way)
        ExperiencePlacement {
            id: ExperienceId(21),
            slug: "21_cosmic_life",
            title: "Vida Cósmica",
            sector_id: SectorId::ORION_SPUR,
            anchor_local: DVec3::new(-0.35, 0.0, 0.25),
            u_min: 15.5,
            u_max: 20.0,
            focus_scale_exp: 18.0,
            bounding_radius_m: 5.0e19,
            zone_count: 25,
            archetype: ManifoldArchetype::ExoplanetBiosphereBelt,
        },
        ExperiencePlacement {
            id: ExperienceId(22),
            slug: "22_cosmic_contact",
            title: "Contacto Cósmico",
            sector_id: SectorId::ORION_SPUR,
            anchor_local: DVec3::new(0.40, 0.05, -0.30),
            u_min: 16.0,
            u_max: 20.8,
            focus_scale_exp: 18.6,
            bounding_radius_m: 1.0e20,
            zone_count: 28,
            archetype: ManifoldArchetype::ExoplanetBiosphereBelt,
        },
        // Remaining Hyper-Horizon Lotus Wings + Zenith Crown
        ExperiencePlacement {
            id: ExperienceId(23),
            slug: "23_cosmic_perspectives",
            title: "Perspectivas Cósmicas",
            sector_id: SectorId::HYPER_LOTUS_23_PERSPECTIVES,
            anchor_local: DVec3::ZERO,
            u_min: 27.2,
            u_max: 28.6,
            focus_scale_exp: 27.6,
            bounding_radius_m: 6.0e27,
            zone_count: 25,
            archetype: ManifoldArchetype::HyperHorizonLotusWing,
        },
        ExperiencePlacement {
            id: ExperienceId(24),
            slug: "24_cosmic_meaning",
            title: "Significado Cósmico",
            sector_id: SectorId::HYPER_LOTUS_24_MEANING,
            anchor_local: DVec3::ZERO,
            u_min: 27.2,
            u_max: 28.6,
            focus_scale_exp: 27.6,
            bounding_radius_m: 6.0e27,
            zone_count: 26,
            archetype: ManifoldArchetype::HyperHorizonLotusWing,
        },
        ExperiencePlacement {
            id: ExperienceId(25),
            slug: "25_final_horizon",
            title: "Horizonte Final",
            sector_id: SectorId::HYPER_LOTUS_25_HORIZON,
            anchor_local: DVec3::ZERO,
            u_min: 27.5,
            u_max: 29.0,
            focus_scale_exp: 28.2,
            bounding_radius_m: 2.0e28,
            zone_count: 25,
            archetype: ManifoldArchetype::HyperHorizonLotusWing,
        },
    ]
}

/// Returns all canonical navigational continuity bridges connecting the 25 experiences.
pub fn continuity_edges() -> Vec<ContinuityEdge> {
    vec![
        ContinuityEdge { from: ExperienceId(3), to: ExperienceId(13), kind: BridgeKind::ScaleDiveInward },
        ContinuityEdge { from: ExperienceId(13), to: ExperienceId(14), kind: BridgeKind::ScaleDiveInward },
        ContinuityEdge { from: ExperienceId(14), to: ExperienceId(15), kind: BridgeKind::EmergenceHelix },
        ContinuityEdge { from: ExperienceId(15), to: ExperienceId(16), kind: BridgeKind::EmergenceHelix },
        ContinuityEdge { from: ExperienceId(16), to: ExperienceId(17), kind: BridgeKind::EmergenceHelix },
        ContinuityEdge { from: ExperienceId(17), to: ExperienceId(1), kind: BridgeKind::NestedSensoryLobe },
        ContinuityEdge { from: ExperienceId(17), to: ExperienceId(2), kind: BridgeKind::NestedSensoryLobe },
        ContinuityEdge { from: ExperienceId(17), to: ExperienceId(18), kind: BridgeKind::ScaleRiseOutward },
        ContinuityEdge { from: ExperienceId(18), to: ExperienceId(19), kind: BridgeKind::ScaleRiseOutward },
        ContinuityEdge { from: ExperienceId(19), to: ExperienceId(20), kind: BridgeKind::ScaleRiseOutward },
        ContinuityEdge { from: ExperienceId(3), to: ExperienceId(4), kind: BridgeKind::ScaleRiseOutward },
        ContinuityEdge { from: ExperienceId(4), to: ExperienceId(5), kind: BridgeKind::ScaleRiseOutward },
        ContinuityEdge { from: ExperienceId(5), to: ExperienceId(21), kind: BridgeKind::EmbeddedStellarArm },
        ContinuityEdge { from: ExperienceId(21), to: ExperienceId(22), kind: BridgeKind::EmbeddedStellarArm },
        ContinuityEdge { from: ExperienceId(5), to: ExperienceId(6), kind: BridgeKind::ScaleRiseOutward },
        ContinuityEdge { from: ExperienceId(6), to: ExperienceId(7), kind: BridgeKind::ScaleRiseOutward },
        ContinuityEdge { from: ExperienceId(7), to: ExperienceId(8), kind: BridgeKind::ScaleRiseOutward },
        ContinuityEdge { from: ExperienceId(8), to: ExperienceId(9), kind: BridgeKind::ScaleRiseOutward },
        ContinuityEdge { from: ExperienceId(9), to: ExperienceId(10), kind: BridgeKind::ScaleRiseOutward },
        ContinuityEdge { from: ExperienceId(10), to: ExperienceId(11), kind: BridgeKind::HyperHorizonLotusPetal },
        ContinuityEdge { from: ExperienceId(10), to: ExperienceId(12), kind: BridgeKind::HyperHorizonLotusPetal },
        ContinuityEdge { from: ExperienceId(10), to: ExperienceId(23), kind: BridgeKind::HyperHorizonLotusPetal },
        ContinuityEdge { from: ExperienceId(10), to: ExperienceId(24), kind: BridgeKind::HyperHorizonLotusPetal },
        ContinuityEdge { from: ExperienceId(10), to: ExperienceId(25), kind: BridgeKind::HyperHorizonLotusPetal },
        ContinuityEdge { from: ExperienceId(12), to: ExperienceId(13), kind: BridgeKind::OuroborosReturn },
    ]
}

/// Maps App 01 continuous `(hue_deg [0, 360), sat_pct [0, 100], light_pct [0, 100])` into
/// normalized `COLORS_PRISM` cylinder/torus local coordinates (`r ∈ [0.15, 1.0]`, `y ∈ [-0.8, 0.8]`).
#[inline]
pub fn hsl_to_prism_local(hue_deg: f64, sat_pct: f64, light_pct: f64) -> DVec3 {
    let theta = hue_deg.rem_euclid(360.0).to_radians();
    let r = 0.15 + 0.85 * (sat_pct / 100.0).clamp(0.0, 1.0);
    let y = ((light_pct / 100.0).clamp(0.0, 1.0) - 0.5) * 1.6;
    DVec3::new(r * theta.cos(), y, r * theta.sin())
}

/// Inverse mapping from `COLORS_PRISM` local coordinates back to `(hue_deg, sat_pct, light_pct)`.
#[inline]
pub fn prism_local_to_hsl(pos: DVec3) -> (f64, f64, f64) {
    let hue_deg = pos.z.atan2(pos.x).to_degrees().rem_euclid(360.0);
    let r = (pos.x * pos.x + pos.z * pos.z).sqrt();
    let sat_pct = (((r - 0.15) / 0.85).clamp(0.0, 1.0)) * 100.0;
    let light_pct = ((pos.y / 1.6 + 0.5).clamp(0.0, 1.0)) * 100.0;
    (hue_deg, sat_pct, light_pct)
}

/// Maps App 02 musical filament `(midi 48..=95 = C3..B6, timbre_idx 0..=3)` into `MUSIC_HARP` local 3D coordinates.
///
/// The 48 notes wrap in an ascending golden spiral ring while the 4 vertical timbre strata stack cleanly along `+Y`.
#[inline]
pub fn music_filament_local(midi: u8, timbre_idx: u8) -> DVec3 {
    let note_idx = midi.saturating_sub(48).min(47) as f64;
    let frac = note_idx / 48.0;
    let angle = frac * std::f64::consts::TAU;
    let radius = 0.45 + 0.45 * frac;
    let timbre_y = (timbre_idx.min(3) as f64 - 1.5) * 0.42;
    DVec3::new(radius * angle.cos(), timbre_y, radius * angle.sin())
}

/// IAU nominal solar radius, shared by physical scene geometry and observation poses.
pub const SUN_RADIUS_M: f64 = 695_700_000.0;
pub const SOLAR_SECTOR_SCALE_EXP: f64 = 11.2;

/// Spherical volumetric mean radii from NASA's planetary fact sheets. Belt
/// extents are teaching volumes and deliberately do not masquerade as bodies.
/// https://nssdc.gsfc.nasa.gov/planetary/planetfact.html
pub fn solar_body_radius_m(zone_index: u16) -> Option<f64> {
    Some(match zone_index {
        0 => SUN_RADIUS_M,
        1 => 2_439_700.0,
        2 => 6_051_800.0,
        3 => 6_371_000.0,
        4 => 3_389_500.0,
        6 => 69_911_000.0,
        7 => 58_232_000.0,
        8 => 25_362_000.0,
        9 => 24_622_000.0,
        11 => 1_188_000.0,
        _ => return None,
    })
}

/// Maps App 04 Solar System bodies (`zone_index 0..=11`) onto canonical `SectorId::SOLAR_SYSTEM`
/// (`scale_exp = 11.2`) orbital positions with Earth (`zone 3`) aligned to `EARTH_ORBIT`'s
/// `(0.9439, 0.0, 0.0)` sector origin for seamless multi-scale zoom continuity.
///
/// Returns `(local_pos, scale_exp, physical_radius_m)` for spherical bodies;
/// belt zones 5/10 return their authored teaching-volume extent instead.
pub fn solar_body_local_pose(
    zone_index: u16,
    x_norm: f32,
    lane: f32,
    layout_r: f32,
) -> (DVec3, f64, f64) {
    let sector_unit_m = 10.0_f64.powf(SOLAR_SECTOR_SCALE_EXP);

    let orbit_r = match zone_index {
        0 => 0.0,
        1 => 0.42,
        2 => 0.68,
        3 => 0.9439,
        4 => 1.48,
        5 => 2.38,
        6 => 3.65,
        7 => 5.05,
        8 => 6.65,
        9 => 8.20,
        10 => 9.55,
        11 => 10.70,
        _ => 0.42 + (x_norm as f64).clamp(0.0, 1.0) * 10.3,
    };

    let local_pos = if zone_index == 0 {
        DVec3::ZERO
    } else if zone_index == 3 {
        DVec3::new(orbit_r, 0.0, 0.0)
    } else {
        let angle = (zone_index as f64 - 3.0) * 0.54;
        DVec3::new(
            orbit_r * angle.cos(),
            (lane as f64) * 0.45,
            orbit_r * angle.sin(),
        )
    };

    let rel_r = if zone_index == 0 {
        0.24
    } else {
        0.032 + ((layout_r as f64).clamp(5.0, 50.0) / 50.0) * 0.135
    };
    // Navigation and rendering share real body radii. Enlarged overview bounds
    // used as geometry made the planets physically larger than the Sun.
    let radius_m = solar_body_radius_m(zone_index).unwrap_or(rel_r * sector_unit_m);
    (local_pos, radius_m.log10(), radius_m)
}

/// Maps App 03 Earth layers (`zone_index 0..=13`) onto concentric atmospheric & cutaway interior
/// shells in `SectorId::EARTH_ORBIT` (`scale_exp = 6.8`, Earth solid radius `~1.01` sector units).
///
/// Returns `(local_pos, scale_exp, radius_m)`.
pub fn earth_layer_local_pose(zone_index: u16) -> (DVec3, f64, f64) {
    const EARTH_SECTOR_SCALE_EXP: f64 = 6.8;
    const EARTH_R_UNITS: f64 = 1.01;
    let sector_unit_m = 10.0_f64.powf(EARTH_SECTOR_SCALE_EXP);

    // Exterior orbital/atmospheric layers (0..=6) vs cutaway interior shells (7..=13).
    let (radial_units, az_rad, el_rad, rel_scale_u): (f64, f64, f64, f64) = match zone_index {
        0 => (1.88, 0.58, 0.44, 6.45),  // space (orbital belt)
        1 => (1.50, -0.52, 0.34, 6.32), // exosphere
        2 => (1.26, 0.18, 0.62, 6.22),  // thermosphere (auroral oval)
        3 => (1.16, 0.46, -0.18, 6.14), // mesosphere
        4 => (1.10, -0.34, 0.12, 6.10), // stratosphere / ozone
        5 => (1.05, 0.12, -0.24, 6.06), // troposphere / cyclones
        6 => (EARTH_R_UNITS, -0.14, 0.04, 6.05), // surface / crust top
        7 => (0.99 * EARTH_R_UNITS, 0.34, -0.06, 5.98), // crust
        8 => (0.95 * EARTH_R_UNITS, -0.28, -0.18, 5.95), // lithosphere
        9 => (0.89 * EARTH_R_UNITS, 0.24, -0.28, 5.92), // asthenosphere
        10 => (0.83 * EARTH_R_UNITS, -0.20, -0.36, 5.88), // transition zone
        11 => (0.68 * EARTH_R_UNITS, 0.18, -0.44, 5.94), // lower mantle
        12 => (0.46 * EARTH_R_UNITS, -0.12, -0.50, 5.90), // outer geodynamo core
        _ => (0.20 * EARTH_R_UNITS, 0.02, -0.26, 5.80),  // inner solid Fe-Ni core
    };

    let local_pos = DVec3::new(
        radial_units * az_rad.sin() * el_rad.cos(),
        radial_units * el_rad.sin(),
        radial_units * az_rad.cos() * el_rad.cos(),
    );
    let radius_m = 10.0_f64.powf(rel_scale_u).max(0.08 * sector_unit_m);
    (local_pos, rel_scale_u, radius_m)
}

/// Maps Apps `05..07` galactic & extragalactic `LANDMARKS` (`31` total) onto sector local space.
///
/// Returns `(local_pos, scale_exp, radius_m)`.
pub fn landmark_local_pose(
    p_norm: f32,
    lane: f32,
    size_val: f32,
    sector_scale_exp: f64,
) -> (DVec3, f64, f64) {
    let p = (p_norm as f64).clamp(0.0, 1.0);
    let theta = p * std::f64::consts::TAU * 1.45 - 0.80;
    let r = 0.45 + p * 5.20;
    let y = (lane as f64) * 1.80;
    let local_pos = DVec3::new(r * theta.cos(), y, r * theta.sin());
    let size_frac = ((size_val as f64).clamp(20.0, 230.0) - 20.0) / 210.0;
    let scale_exp = sector_scale_exp - 1.35 + size_frac * 0.80;
    (local_pos, scale_exp, 10.0_f64.powf(scale_exp))
}

/// Maps any experience (`03..=25`) zone index cleanly into structured 3D sector coordinates inside
/// `[-10.5, +10.5]^3` so all 376 zones remain spatially distinct and hover-scannable.
pub fn zone_local_pose(
    experience_id: u8,
    zone_index: u16,
    zone_count: usize,
    anchor_local: DVec3,
    focus_scale_exp: f64,
) -> (DVec3, f64) {
    if experience_id == 3 {
        let (p, u, _) = earth_layer_local_pose(zone_index);
        return (anchor_local + p, u);
    }
    let count = zone_count.max(1) as f64;
    let frac = (zone_index as f64 + 0.5) / count;
    let az = frac * std::f64::consts::TAU * 1.5;
    let radial = 0.48 + frac * 1.65;
    let y = ((zone_index % 5) as f64 - 2.0) * 0.24;
    let pos = anchor_local + DVec3::new(radial * az.cos(), y, radial * az.sin());
    (pos, focus_scale_exp - 1.45)
}

/// Builds the full canonical `SectorTree` and populated [`CosmosSpatialIndex`] covering all 25 experiences,
/// including `01 Colors` volumetric HSL sample grid & `02 Music` 48-note × 4-timbre (192 filament) harp.
pub fn build_default_universe_index() -> CosmosSpatialIndex {
    let mut tree = SectorTree::with_root(28.0);
    let mut shells = Vec::new();

    // Helper closure-style macro or function to register sector + matching ScaleShell
    let mut add = |id: SectorId,
                   name: &str,
                   parent: SectorId,
                   offset: DVec3,
                   scale_exp: f64,
                   u_min: f64,
                   u_max: f64,
                   radius_m: f64| {
        tree.insert_sector(SectorNode::new(
            id,
            name,
            Some(parent),
            offset,
            scale_exp,
            radius_m,
        ))
        .expect("valid cosmos sector hierarchy");
        shells.push(ScaleShell::new(id, scale_exp, u_min, u_max, 12.0));
    };

    // Outward macro cosmic hierarchy (anchored to UNIVERSE_ROOT at u = +28)
    add(SectorId::EARLY_INFLATION, "Early Universe & Inflation", SectorId::UNIVERSE_ROOT, DVec3::ZERO, 26.9, 26.5, 27.4, 1.5e27);
    add(SectorId::OBSERVABLE_CMB, "Observable Universe & CMB", SectorId::EARLY_INFLATION, DVec3::new(-0.08, 0.04, 0.06), 26.4, 25.8, 26.8, 4.4e26);
    add(SectorId::COSMIC_WEB, "Cosmic Web Filaments", SectorId::OBSERVABLE_CMB, DVec3::new(0.02, -0.01, 0.01), 25.3, 24.4, 26.1, 5.0e25);
    add(SectorId::LANIAKEA, "Laniakea Flow Basin", SectorId::COSMIC_WEB, DVec3::new(-0.04, 0.02, 0.01), 23.8, 22.5, 24.6, 2.6e24);
    add(SectorId::LOCAL_GROUP, "Local Group", SectorId::LANIAKEA, DVec3::new(0.03, 0.01, -0.02), 22.0, 21.0, 22.7, 4.5e22);
    add(SectorId::MILKY_WAY, "Milky Way", SectorId::LOCAL_GROUP, DVec3::new(-0.02, 0.0, 0.01), 20.5, 13.5, 21.2, 1.5e21);
    add(SectorId::ORION_SPUR, "Orion Spur", SectorId::MILKY_WAY, DVec3::new(0.026, 0.0, 0.0), 18.2, 15.0, 21.0, 1.0e20);
    add(SectorId::SOLAR_SYSTEM, "Solar System", SectorId::ORION_SPUR, DVec3::ZERO, 11.2, 6.8, 13.8, 4.0e13);
    add(SectorId::EARTH_ORBIT, "Earth Orbit", SectorId::SOLAR_SYSTEM, DVec3::new(0.9439, 0.0, 0.0), 6.8, 0.0, 7.6, 1.0e8); // 1 AU ≈ 1.496e11m / 10^11.2m
    add(SectorId::EARTH_SURFACE, "Earth Surface & Atmosphere", SectorId::EARTH_ORBIT, DVec3::new(0.0, 1.01, 0.0), 0.0, -5.5, 1.8, 1.0e5);
    add(SectorId::EARTH_CORE, "Earth Deep Interior", SectorId::EARTH_ORBIT, DVec3::ZERO, 6.5, 0.0, 6.8, 6.37e6);

    // Subatomic & Planck hierarchy diving inward from Earth crust/core
    add(SectorId::QUANTUM_FIELD, "Matter & Quantum Fields", SectorId::EARTH_CORE, DVec3::new(0.0, 0.0, 0.0), -11.0, -22.2, 0.2, 1.0);
    add(SectorId::PLANCK_ORIGIN, "Spacetime & Planck Foam", SectorId::QUANTUM_FIELD, DVec3::ZERO, -30.0, -35.5, -21.8, 1.0e-21);

    // Emergence Helix -> Life -> Mind -> nested Colors (01) & Music (02) -> Civilization
    add(SectorId::COMPLEXITY_HELIX, "Complexity Helix", SectorId::EARTH_SURFACE, DVec3::new(0.0, -0.002, 0.0), -7.0, -9.2, -4.8, 1.0e-4);
    add(SectorId::LIFE_BIOSPHERE, "Biosphere", SectorId::EARTH_SURFACE, DVec3::new(0.01, 0.0, 0.0), -1.0, -6.2, 2.2, 200.0);
    add(SectorId::MIND_CATHEDRAL, "Noosphere Mind Cathedral", SectorId::EARTH_ORBIT, DVec3::new(0.0, 1.012, 0.0), 2.8, 1.4, 4.6, 25_000.0);
    add(SectorId::COLORS_PRISM, "Colors Volumetric Prism", SectorId::MIND_CATHEDRAL, DVec3::new(-0.22, 0.02, -0.04), 2.0, 0.5, 3.6, 120.0);
    add(SectorId::MUSIC_HARP, "Music Cymatic Harp", SectorId::MIND_CATHEDRAL, DVec3::new(0.22, 0.02, -0.04), 2.0, 0.5, 3.6, 120.0);
    add(SectorId::CULTURE_WEB, "Planetary Culture Web", SectorId::EARTH_ORBIT, DVec3::ZERO, 6.2, 4.4, 7.0, 6.5e6);
    add(SectorId::TECH_RING, "Orbital Tech & AI Ring", SectorId::EARTH_ORBIT, DVec3::ZERO, 7.6, 6.8, 8.3, 4.0e8);
    add(SectorId::FUTURES_ARCS, "Solar Kardashev Arcs", SectorId::SOLAR_SYSTEM, DVec3::new(0.2, 0.1, 0.0), 11.8, 8.1, 16.2, 1.0e16);

    // Five-petaled Hyper-Horizon Lotus widely fanned off the -Z macro sightline (u = 27.45 .. 29.2)
    add(SectorId::HYPER_LOTUS_11_EVIDENCE, "Lotus Wing North: Cosmic Evidence", SectorId::UNIVERSE_ROOT, DVec3::new(8.2, 2.8, -2.6), 27.7, 27.45, 28.8, 6.0e27);
    add(SectorId::HYPER_LOTUS_12_FUTURE, "Lotus Wing East: Future of Universe", SectorId::UNIVERSE_ROOT, DVec3::new(2.6, -2.4, 8.2), 27.7, 27.45, 28.8, 6.0e27);
    add(SectorId::HYPER_LOTUS_23_PERSPECTIVES, "Lotus Wing South: Perspectives", SectorId::UNIVERSE_ROOT, DVec3::new(-8.2, 2.6, 2.6), 27.7, 27.45, 28.8, 6.0e27);
    add(SectorId::HYPER_LOTUS_24_MEANING, "Lotus Wing West: Cosmic Meaning", SectorId::UNIVERSE_ROOT, DVec3::new(-6.8, -4.8, -2.8), 27.7, 27.45, 28.8, 6.0e27);
    add(SectorId::HYPER_LOTUS_25_HORIZON, "Lotus Crown Zenith: Final Horizon", SectorId::UNIVERSE_ROOT, DVec3::new(0.0, 14.5, 0.0), 28.2, 27.6, 29.2, 2.0e28);

    let mut index = CosmosSpatialIndex::new(tree, shells);

    // Insert primary anchor nodes for all 25 experiences
    let mut next_id = 1u64;
    for placement in all_experience_placements() {
        index
            .insert_entity(SpatialEntity {
                id: next_id,
                experience_id: placement.id.0,
                zone_index: u16::MAX,
                feature_index: u16::MAX,
                sector_id: placement.sector_id,
                local_pos: placement.anchor_local,
                orient: DQuat::IDENTITY,
                radius_m: placement.bounding_radius_m,
                scale_exp: placement.focus_scale_exp,
                kind: EntityKind::ExperienceAnchor,
                salience_weight: 2.0,
            })
            .expect("experience anchor insertion");
        next_id += 1;
    }

    // Populate App 01 Colors Volumetric HSL sample lattice inside COLORS_PRISM
    for h_step in 0..12 {
        let hue = h_step as f64 * 30.0;
        for s_step in [40.0, 85.0] {
            for l_step in [30.0, 50.0, 75.0] {
                let p = hsl_to_prism_local(hue, s_step, l_step);
                index
                    .insert_entity(SpatialEntity {
                        id: next_id,
                        experience_id: 1,
                        zone_index: 0,
                        feature_index: (h_step as u16),
                        sector_id: SectorId::COLORS_PRISM,
                        local_pos: p,
                        orient: DQuat::IDENTITY,
                        radius_m: 12.0,
                        scale_exp: 1.08,
                        kind: EntityKind::ChromaticSample,
                        salience_weight: 1.2,
                    })
                    .expect("color prism node insertion");
                next_id += 1;
            }
        }
    }

    // Populate App 02 Music 48 notes × 4 timbres (192 interactive filaments) inside MUSIC_HARP
    for timbre in 0u8..4 {
        for midi in 48u8..=95 {
            let p = music_filament_local(midi, timbre);
            index
                .insert_entity(SpatialEntity {
                    id: next_id,
                    experience_id: 2,
                    zone_index: timbre as u16,
                    feature_index: (midi - 48) as u16,
                    sector_id: SectorId::MUSIC_HARP,
                    local_pos: p,
                    orient: DQuat::IDENTITY,
                    radius_m: 8.0,
                    scale_exp: 0.9,
                    kind: EntityKind::MusicalFilament,
                    salience_weight: 1.3,
                })
                .expect("music harp filament insertion");
            next_id += 1;
        }
    }

    index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lotus_sectors_are_widely_separated_and_off_z_axis() {
        let idx = build_default_universe_index();
        let lotus_ids = [
            SectorId::HYPER_LOTUS_11_EVIDENCE,
            SectorId::HYPER_LOTUS_12_FUTURE,
            SectorId::HYPER_LOTUS_23_PERSPECTIVES,
            SectorId::HYPER_LOTUS_24_MEANING,
            SectorId::HYPER_LOTUS_25_HORIZON,
        ];
        let positions: Vec<DVec3> = lotus_ids
            .iter()
            .map(|id| idx.tree.get(*id).expect("lotus sector").offset_in_parent)
            .collect();
        for (i, a) in positions.iter().enumerate() {
            for b in positions.iter().skip(i + 1) {
                assert!(
                    a.distance(*b) > 8.0,
                    "lotus sectors too close: {a:?} vs {b:?}"
                );
            }
        }
        // Earth in Solar System stays aligned to SectorId::EARTH_ORBIT offset.
        let (earth_solar_pos, _, _) = solar_body_local_pose(3, 0.295, -0.08, 21.0);
        let earth_sector_pos = idx.tree.get(SectorId::EARTH_ORBIT).unwrap().offset_in_parent;
        assert!(earth_solar_pos.distance(earth_sector_pos) < 1e-9);
    }

    #[test]
    fn solar_earth_and_landmark_poses_stay_within_octree_bounds() {
        for zi in 0..=11u16 {
            let (p, u, r) = solar_body_local_pose(zi, (zi as f32) / 11.0, 0.05, 20.0);
            assert!(p.abs().max_element() < 11.5, "solar body {zi} out of bounds: {p:?}");
            assert!(r > 0.0 && u.is_finite());
            assert!((10.0_f64.powf(u) / r - 1.0).abs() < 1e-12);
            if let Some(physical_radius) = solar_body_radius_m(zi) {
                assert_eq!(r, physical_radius);
                assert!(r <= SUN_RADIUS_M, "a planet cannot be larger than the Sun");
            }
        }
        for zi in 0..=13u16 {
            let (p, u, r) = earth_layer_local_pose(zi);
            assert!(p.abs().max_element() < 3.0, "earth layer {zi} out of bounds: {p:?}");
            assert!(r > 0.0 && u > 5.0);
        }
    }
}
