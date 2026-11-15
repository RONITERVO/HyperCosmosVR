use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use cosmos_manifest::{Locale, UniverseCatalog};
use cosmos_math::{ScalePose64, SectorId};
use cosmos_narration::TargetFocus;
use cosmos_render::experience_accent_linear;
use cosmos_spatial::{
    all_experience_placements, build_default_universe_index, landmark_local_pose,
    solar_body_local_pose, zone_local_pose, CosmosSpatialIndex, EntityKind, SpatialEntity,
    VisibleEntity,
};
use cosmos_typo::{
    AnchorMode, AmbientLabelInput, FontRole, GlyphInkMode, MsdfAtlas, MsdfBakeConfig,
};
use glam::{DQuat, DVec3};

const REQUIRED_MARKER: &str = "assets/baked/universe_catalog.hcvb";
const SHADER_MARKER: &str = "shaders/common_oled.wgsl";

/// Discovered and memory-mapped runtime assets required across Desktop, VR, and Headless modes.
pub struct UniverseAssets {
    pub root: PathBuf,
    pub catalog: Arc<UniverseCatalog>,
    pub atlas: MsdfAtlas,
    pub spatial: CosmosSpatialIndex,
    pub hrir_path: PathBuf,
    pub shaders_dir: PathBuf,
}

impl UniverseAssets {
    /// Locates the repository root and loads all zero-copy `.hcvb`/`.hcvp`, MSDF atlas cache,
    /// HRIR sphere, and enriched 25-experience spatial index.
    pub fn load() -> Result<Self> {
        let root = locate_project_root()?;
        Self::load_from_root(&root)
    }

    pub fn load_from_root(root: &Path) -> Result<Self> {
        let hcvb = root.join("assets/baked/universe_catalog.hcvb");
        let hcvp = root.join("assets/baked/universe_audio.hcvp");
        let hcvp_opt = if hcvp.exists() {
            Some(hcvp.as_path())
        } else {
            eprintln!(
                "[HyperCosmosVR] Note: optional narration audio pack not found at {} — launching in visual + spatial synth mode. Run tools/fetch_audio_pack.ps1 or tools/bake_universe.py to enable recorded narration.",
                hcvp.display()
            );
            None
        };
        let catalog = Arc::new(
            UniverseCatalog::open(&hcvb, hcvp_opt)
                .with_context(|| format!("failed to open universe catalog at {}", hcvb.display()))?,
        );

        let fonts_dir = root.join("assets/fonts");
        let msdf_cache_dir = root.join("assets/baked/msdf_cache");
        let atlas = MsdfAtlas::load_or_bake(&fonts_dir, &msdf_cache_dir, &MsdfBakeConfig::default())
            .with_context(|| format!("failed to load/bake MSDF atlas from {}", msdf_cache_dir.display()))?;

        let mut spatial = build_default_universe_index();
        populate_catalog_entities(&mut spatial, &catalog);

        let hrir_path = root.join("assets/hrir/irc_1040_c.bin");
        let shaders_dir = root.join("shaders");

        Ok(Self {
            root: root.to_path_buf(),
            catalog,
            atlas,
            spatial,
            hrir_path,
            shaders_dir,
        })
    }

    /// Computes a clean inspection `(ScalePose64, look_target_in_sector)` for any catalog zone.
    pub fn navigation_pose_for_zone(
        &self,
        exp_id: u8,
        zone_index: u16,
        view_dir: DVec3,
    ) -> Option<(ScalePose64, DVec3)> {
        let ent = self.spatial.find_zone_entity(exp_id, zone_index)?;
        self.spatial.navigation_pose_for_entity(ent, view_dir)
    }

    /// Computes a clean inspection `(ScalePose64, look_target_in_sector)` for any catalog feature.
    pub fn navigation_pose_for_feature(
        &self,
        exp_id: u8,
        zone_index: u16,
        feature_index: u16,
        view_dir: DVec3,
    ) -> Option<(ScalePose64, DVec3)> {
        let ent = self
            .spatial
            .find_feature_entity(exp_id, zone_index, feature_index)?;
        self.spatial.navigation_pose_for_entity(ent, view_dir)
    }
}

/// Locates workspace root whether launched from repo root, `target/release/`, or `cargo test`.
pub fn locate_project_root() -> Result<PathBuf> {
    if let Ok(env_root) = std::env::var("HYPERCOSMOS_ROOT") {
        let p = PathBuf::from(env_root);
        if is_valid_root(&p) {
            return Ok(p);
        }
    }

    let mut candidates = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        let mut cur = Some(cwd.as_path());
        while let Some(dir) = cur {
            candidates.push(dir.to_path_buf());
            cur = dir.parent();
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        let mut cur = exe.parent();
        while let Some(dir) = cur {
            candidates.push(dir.to_path_buf());
            cur = dir.parent();
        }
    }
    if let Some(manifest_dir) = option_env!("CARGO_MANIFEST_DIR") {
        let m = Path::new(manifest_dir);
        if let Some(root) = m.parent().and_then(Path::parent) {
            candidates.push(root.to_path_buf());
        }
    }

    for cand in candidates {
        if is_valid_root(&cand) {
            return Ok(cand);
        }
    }

    anyhow::bail!(
        "could not locate HyperCosmosVR root containing '{}' and '{}'",
        REQUIRED_MARKER,
        SHADER_MARKER
    )
}

fn is_valid_root(p: &Path) -> bool {
    p.join(REQUIRED_MARKER).is_file() && p.join(SHADER_MARKER).is_file()
}

/// Enriches the spatial octree with catalog zones, all 12 Solar System bodies (`04`),
/// all 14 Earth shells (`03`), all 31 galactic landmarks (`05..07`), and all 2,456 features so every
/// scale band carries authentic hover-scan targets, spatial navigation, and Google-Earth labels.
fn canonical_landmark_zone_slug(app_num: u8, landmark_id: &str) -> &str {
    match (app_num, landmark_id) {
        (5, "sun") => "frontier",
        (5, "alpha") => "alpha-centauri",
        (5, "bubble") => "local-bubble",
        (5, "orion") => "orion-spur",
        (5, "nebula") => "stellar-nursery",
        (5, "disk") => "milky-way-disk",
        (5, "center") => "galactic-center",
        (6, "milky-edge") => "galactic-edge",
        (6, "dwarfs") => "dwarf-galaxies",
        (6, "intergalactic") => "intergalactic-space",
        (6, "magellanic") => "magellanic-clouds",
        (6, "center") => "local-group-center",
        (6, "boundary") => "local-group-limit",
        (7, "boundary") => "laniakea-limit",
        _ => landmark_id,
    }
}

fn populate_catalog_entities(spatial: &mut CosmosSpatialIndex, catalog: &UniverseCatalog) {
    let mut next_id = 10_000_u64;
    let placements = all_experience_placements();
    let mut anchored_zones: HashMap<(u8, u16), (DVec3, f64)> = HashMap::new();

    // Solar System (04) bodies from BODY_LAYOUT (0..=9, 11) + Kuiper Belt (zone 10).
    for body in catalog.body_layout() {
        let z_idx = body.zone_index as u16;
        let (local_pos, body_u, radius_m) =
            solar_body_local_pose(z_idx, body.x, body.lane, body.radius);
        let _ = spatial.insert_entity(SpatialEntity {
            id: next_id,
            experience_id: 4,
            zone_index: z_idx,
            feature_index: 0,
            sector_id: SectorId::SOLAR_SYSTEM,
            local_pos,
            orient: DQuat::IDENTITY,
            radius_m,
            scale_exp: body_u,
            kind: EntityKind::CelestialBody,
            salience_weight: 1.65,
        });
        next_id += 1;
        anchored_zones.insert((4, z_idx), (local_pos, body_u));
    }
    // Ensure zone 10 (`kuiper`) is also indexed as a first-class navigable CelestialBody.
    if !anchored_zones.contains_key(&(4, 10)) && catalog.zones(4).len() > 10 {
        let (local_pos, body_u, radius_m) = solar_body_local_pose(10, 0.92, -0.04, 20.0);
        let _ = spatial.insert_entity(SpatialEntity {
            id: next_id,
            experience_id: 4,
            zone_index: 10,
            feature_index: 0,
            sector_id: SectorId::SOLAR_SYSTEM,
            local_pos,
            orient: DQuat::IDENTITY,
            radius_m,
            scale_exp: body_u,
            kind: EntityKind::CelestialBody,
            salience_weight: 1.55,
        });
        next_id += 1;
        anchored_zones.insert((4, 10), (local_pos, body_u));
    }

    // Cosmic landmarks (05 Milky Way, 06 Local Group, 07 Laniakea — 31 records).
    for lm in catalog.all_landmarks() {
        if let Some(placement) = placements.iter().find(|p| p.id.0 == lm.app_num) {
            let zones = catalog.zones(lm.app_num);
            let target_id = canonical_landmark_zone_slug(lm.app_num, catalog.str_slice(lm.zone_id));
            if let Some(z_idx) = zones
                .iter()
                .position(|z| catalog.str_slice(z.id) == target_id)
                .map(|i| i as u16)
            {
                let (local_pos, scale_exp, radius_m) =
                    landmark_local_pose(lm.p, lm.lane, lm.size, placement.focus_scale_exp);
                let _ = spatial.insert_entity(SpatialEntity {
                    id: next_id,
                    experience_id: lm.app_num,
                    zone_index: z_idx,
                    feature_index: 0,
                    sector_id: placement.sector_id,
                    local_pos,
                    orient: DQuat::IDENTITY,
                    radius_m,
                    scale_exp,
                    kind: EntityKind::GalacticLandmark,
                    salience_weight: 1.5,
                });
                next_id += 1;
                anchored_zones.insert((lm.app_num, z_idx), (local_pos, scale_exp));
            }
        }
    }
    // Ensure Exp 05 Z12 (`galactic-edge`) is indexed as a first-class GalacticLandmark (13th milestone)
    if !anchored_zones.contains_key(&(5, 12))
        && catalog.zones(5).len() > 12
        && let Some(placement) = placements.iter().find(|p| p.id.0 == 5)
    {
        let (local_pos, scale_exp, radius_m) =
            landmark_local_pose(0.96, 0.08, 175.0, placement.focus_scale_exp);
        let _ = spatial.insert_entity(SpatialEntity {
            id: next_id,
            experience_id: 5,
            zone_index: 12,
            feature_index: 0,
            sector_id: SectorId::MILKY_WAY,
            local_pos,
            orient: DQuat::IDENTITY,
            radius_m,
            scale_exp,
            kind: EntityKind::GalacticLandmark,
            salience_weight: 1.5,
        });
        next_id += 1;
        anchored_zones.insert((5, 12), (local_pos, scale_exp));
    }

    // Stratified zones + ALL interactive catalog features across experiences (`03..=25`).
    const GOLDEN_ANGLE_RAD: f64 = 2.399963229728653;
    for placement in &placements {
        let exp_id = placement.id.0;
        if exp_id <= 2 {
            continue;
        }
        let zones = catalog.zones(exp_id);
        let z_count = zones.len().max(1);
        for (zi, zone) in zones.iter().enumerate() {
            let z_idx = zi as u16;
            let (anchor_pos, feature_base_u, ring_min, ring_span, y_step) =
                if let Some(&(existing_pos, existing_u)) = anchored_zones.get(&(exp_id, z_idx)) {
                    let (r0, span, dy) = if exp_id == 4 {
                        (0.05, 0.14, 0.024)
                    } else {
                        (0.12, 0.34, 0.045)
                    };
                    (existing_pos, existing_u - 0.55, r0, span, dy)
                } else {
                    let (local_pos, zone_u) = zone_local_pose(
                        exp_id,
                        z_idx,
                        z_count,
                        placement.anchor_local,
                        placement.focus_scale_exp,
                    );
                    let _ = spatial.insert_entity(SpatialEntity {
                        id: next_id,
                        experience_id: exp_id,
                        zone_index: z_idx,
                        feature_index: 0,
                        sector_id: placement.sector_id,
                        local_pos,
                        orient: DQuat::IDENTITY,
                        radius_m: 10.0_f64.powf(zone_u),
                        scale_exp: zone_u,
                        kind: EntityKind::ZoneShell,
                        salience_weight: 1.25,
                    });
                    next_id += 1;

                    let (r0, span, dy) = if exp_id == 3 {
                        (0.08, 0.18, 0.032)
                    } else {
                        (0.10, 0.28, 0.040)
                    };
                    (local_pos, zone_u - 0.55, r0, span, dy)
                };

            let features = catalog.features(zone);
            let f_count = features.len().max(1);
            for (fi, _feat) in features.iter().enumerate() {
                let theta = (fi as f64) * GOLDEN_ANGLE_RAD + (zi as f64) * 0.37;
                let t = (fi as f64 + 0.5) / f_count as f64;
                let ring_r = ring_min + t * ring_span;
                let ring_y = ((fi % 5) as f64 - 2.0) * y_step;
                let feat_pos =
                    anchor_pos + DVec3::new(ring_r * theta.cos(), ring_y, ring_r * theta.sin());
                let _ = spatial.insert_entity(SpatialEntity {
                    id: next_id,
                    experience_id: exp_id,
                    zone_index: z_idx,
                    feature_index: fi as u16,
                    sector_id: placement.sector_id,
                    local_pos: feat_pos,
                    orient: DQuat::IDENTITY,
                    radius_m: 10.0_f64.powf(feature_base_u),
                    scale_exp: feature_base_u,
                    kind: EntityKind::FeatureNode,
                    salience_weight: 1.0,
                });
                next_id += 1;
            }
        }
    }
}

/// Resolves a concise Google-Earth 3D spatial label for a visible entity.
pub fn label_text_for_visible(
    catalog: &UniverseCatalog,
    vis: &VisibleEntity,
    locale: Locale,
) -> Option<String> {
    match vis.kind {
        EntityKind::ExperienceAnchor => {
            let exp = catalog.experience(vis.experience_id)?;
            let label = catalog.str_slice(exp.label).trim();
            if label.is_empty() {
                None
            } else {
                Some(label.to_owned())
            }
        }
        EntityKind::ZoneShell | EntityKind::CelestialBody | EntityKind::GalacticLandmark => {
            let zone = catalog.zones(vis.experience_id).get(vis.zone_index as usize)?;
            let es = triple_text(catalog, zone.name, Locale::EsEs)
                .unwrap_or_else(|| catalog.str_slice(zone.id));
            if locale != Locale::EsEs
                && let Some(tr) = triple_text(catalog, zone.name, locale)
                && !tr.eq_ignore_ascii_case(es)
                && tr.len() <= 24
                && es.len() <= 22
            {
                return Some(format!("{es} · {tr}"));
            }
            Some(es.to_owned())
        }
        EntityKind::FeatureNode => {
            let zone = catalog.zones(vis.experience_id).get(vis.zone_index as usize)?;
            let feat = catalog.features(zone).get(vis.feature_index as usize)?;
            let es = triple_text(catalog, feat.name, Locale::EsEs)?;
            Some(es.to_owned())
        }
        EntityKind::ChromaticSample => {
            let hue = (vis.feature_index as f32 * 30.0) % 360.0;
            let [r, g, b] = cosmos_narration::hsl_to_srgb_u8(hue as f64, 85.0, 50.0);
            let solved = catalog.solve_color(r, g, b);
            Some(solved.name)
        }
        EntityKind::MusicalFilament => {
            let midi = 48_u8.saturating_add((vis.feature_index as u8).min(47));
            let pitch = (midi % 12) as usize;
            let oct = (midi as i32 / 12) - 1;
            let note = catalog.music_notes().get(pitch)?;
            let sym = catalog.str_slice(note.symbol);
            let latin = triple_text(catalog, note.latin, Locale::EsEs).unwrap_or(sym);
            Some(format!("{latin} · {sym}{oct}"))
        }
        EntityKind::SyntheticCluster => None,
    }
}

/// Builds candidate Google-Earth 3D MSDF world labels for the current camera frustum.
pub fn build_ambient_label_inputs(
    catalog: &UniverseCatalog,
    visible: &[VisibleEntity],
    u_cam: f64,
    active_experience_id: u8,
    active_zone_index: u16,
    focused_entity_id: Option<u64>,
    locale: Locale,
) -> Vec<AmbientLabelInput> {
    let placements = all_experience_placements();
    let mut out = Vec::with_capacity(48);

    for vis in visible.iter().take(128) {
        if focused_entity_id == Some(vis.id) {
            continue;
        }
        let is_active_exp = vis.experience_id == active_experience_id;
        if active_experience_id == 3 && is_active_exp
            && (active_zone_index <= 6 || active_zone_index == u16::MAX)
            && matches!(vis.kind, EntityKind::ZoneShell | EntityKind::FeatureNode)
        {
            // These catalog nodes use schematic selector positions rather than
            // positions of clouds, aurora, or terrain. Explicit narration and
            // the spatial guide supply context without labeling arbitrary air.
            continue;
        }
        // Isolate 01 Colors / 02 Music strictly, and prevent sibling experience zone/landmark bleed
        // (e.g., 05 Milky Way "Vecindario estelar" hijacking 04 Solar System or 22 Exoplanets).
        if matches!(active_experience_id, 1 | 2) {
            if !is_active_exp {
                continue;
            }
        } else if !is_active_exp && vis.kind != EntityKind::ExperienceAnchor {
            continue;
        }

        let pos = vis.pose.pos;
        if !pos.is_finite() || pos.length() < 0.18 {
            continue;
        }
        let Some(text) = label_text_for_visible(catalog, vis, locale) else {
            continue;
        };

        let accent = experience_accent_linear(vis.experience_id);
        let placement = placements.iter().find(|p| p.id.0 == vis.experience_id);
        let (u_min, u_max) = match (vis.kind, placement) {
            (EntityKind::ExperienceAnchor, Some(p)) => (p.u_min - 0.85, p.u_max + 0.85),
            (_, Some(p)) => (p.u_min - 0.90, p.u_max + 0.90),
            _ => (u_cam - 6.0, u_cam + 6.0),
        };
        let is_major = matches!(
            vis.kind,
            EntityKind::ExperienceAnchor | EntityKind::CelestialBody
        );
        let target_deg = if is_major {
            1.15_f32
        } else if is_active_exp {
            0.88_f32
        } else {
            0.72_f32
        };

        let mut item = AmbientLabelInput::landmark(
            vis.id,
            text,
            pos,
            vis.salience_score * if is_active_exp { 1.4 } else { 0.85 },
            u_min,
            u_max,
        );
        item.role = if is_major {
            FontRole::DisplayScript
        } else {
            FontRole::UiSans
        };
        item.target_angle_rad = target_deg.to_radians();
        item.fade_band_u = 0.55;
        item.color_hdr = [
            0.75 + 0.45 * accent.x,
            0.78 + 0.45 * accent.y,
            0.82 + 0.45 * accent.z,
            if is_active_exp { 0.96 } else { 0.80 },
        ];
        item.mode = GlyphInkMode::SpatialAnchor;
        item.anchor_mode = AnchorMode::ComfortDistanceBand {
            min_m: 1.5,
            max_m: 14.0,
        };
        item.tether_to_world = is_major;
        out.push(item);
    }

    out
}

/// Converts a visible entity cleanly into a `cosmos_narration::TargetFocus`.
///
/// `focus_point_cam` is expressed in camera/rig-local render units (same frame as `ray_cam`),
/// whereas `vis.pose.pos` is already rotated into camera-relative world orientation.
pub fn resolve_focus_for_visible(
    index: &CosmosSpatialIndex,
    vis: &VisibleEntity,
    focus_point_cam: Option<DVec3>,
    u_cam: f64,
    cam_pose: &cosmos_math::ScalePose64,
) -> TargetFocus {
    let rel_world = focus_point_cam
        .map(|p_cam| cam_pose.orient * p_cam)
        .unwrap_or_else(|| vis.pose.pos.as_dvec3());
    let local_hit = match vis.experience_id {
        1 => project_world_offset_to_sector(&index.tree, cam_pose, SectorId::COLORS_PRISM, rel_world),
        2 => project_world_offset_to_sector(&index.tree, cam_pose, SectorId::MUSIC_HARP, rel_world),
        _ => None,
    };

    let mut focus = TargetFocus::from_visible_entity(vis, local_hit);
    // Keep Earth Low Orbit macro view zoomed cleanly onto the active Earth zone when inside radius.
    if let TargetFocus::CatalogItem {
        experience_id,
        ref mut is_macro_zoomed_out,
        ..
    } = focus
        && experience_id == 3
        && u_cam <= 6.75
        && vis.kind == EntityKind::ZoneShell
    {
        *is_macro_zoomed_out = false;
    }
    focus
}

fn project_world_offset_to_sector(
    tree: &cosmos_math::SectorTree,
    cam_pose: &cosmos_math::ScalePose64,
    target_sector: SectorId,
    rel_world: DVec3,
) -> Option<DVec3> {
    let target_node = tree.get(target_sector)?;
    let (cam_in_target, _) = tree
        .transform_between(
            cam_pose.sector_id,
            cam_pose.local_pos,
            cam_pose.orient,
            target_sector,
        )
        .ok()?;
    let scale = 10.0_f64.powf(cam_pose.scale_exp - target_node.scale_exp);
    Some(cam_in_target + rel_world * scale)
}

#[inline]
pub fn triple_text(
    catalog: &UniverseCatalog,
    triple: cosmos_manifest::PhraseTriple,
    locale: Locale,
) -> Option<&str> {
    let idx = triple.asset_for(locale)?;
    let rec = catalog.audio_asset(idx)?;
    let s = catalog.asset_text(rec).trim();
    (!s.is_empty()).then_some(s)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmos_math::{CameraRelativePose, ScalePose64};
    use cosmos_spatial::LodTier;
    use glam::Vec3;

    #[test]
    fn populate_emits_all_catalog_features_including_solar_bodies() {
        let assets = UniverseAssets::load().expect("workspace assets");
        let expected_features: usize = (3_u8..=25)
            .flat_map(|exp_id| assets.catalog.zones(exp_id).iter())
            .map(|z| assets.catalog.features(z).len())
            .sum();
        assert!(
            expected_features > 2_000,
            "expected full catalog feature count > 2000, got {expected_features}"
        );

        let actual_features = assets
            .spatial
            .shells
            .iter()
            .flat_map(|shell| shell.octree.entities().iter())
            .filter(|e| e.kind == EntityKind::FeatureNode)
            .count();
        assert_eq!(actual_features, expected_features);

        // Verify all 376 zones, 12 Solar System bodies (0..=11), 14 Earth layers (0..=13),
        // and 32 Landmarks (31 baked + Exp 05 Z12 galactic-edge) are uniquely indexed via CosmosSpatialIndex.
        assert_eq!(assets.spatial.zone_entities_for_experience(4).len(), 12);
        assert_eq!(assets.spatial.zone_entities_for_experience(3).len(), 14);
        assert_eq!(assets.spatial.zone_entities_for_experience(5).len(), 13);
        let landmark_count = assets
            .spatial
            .iter_entities()
            .filter(|e| e.kind == EntityKind::GalacticLandmark)
            .count();
        assert_eq!(landmark_count, assets.catalog.all_landmarks().len() + 1);

        let expected_zones: usize = (3_u8..=25)
            .map(|exp_id| assets.catalog.zones(exp_id).len())
            .sum();
        let actual_zones: usize = (3_u8..=25)
            .map(|exp_id| assets.spatial.zone_entities_for_experience(exp_id).len())
            .sum();
        assert_eq!(actual_zones, expected_zones);

        // Sample navigation pose generation across Solar body, Earth core, landmark, and feature.
        assert!(assets.navigation_pose_for_zone(4, 7, DVec3::Z).is_some());
        assert!(assets.navigation_pose_for_zone(3, 13, DVec3::Z).is_some());
        assert!(assets.navigation_pose_for_zone(5, 10, DVec3::Z).is_some());
        assert!(assets.navigation_pose_for_feature(13, 3, 0, DVec3::Z).is_some());
    }

    #[test]
    fn resolve_focus_for_visible_does_not_double_rotate_world_pose_fallback() {
        let index = build_default_universe_index();
        let cam_orient = DQuat::from_rotation_y(std::f64::consts::FRAC_PI_2);
        let cam_pose = ScalePose64::new(
            SectorId::COLORS_PRISM,
            DVec3::ZERO,
            2.0,
            cam_orient,
        );
        let world_offset = Vec3::new(0.4, 0.2, -0.3);
        let vis = VisibleEntity {
            id: 42,
            experience_id: 1,
            zone_index: 0,
            feature_index: 0,
            kind: EntityKind::ChromaticSample,
            pose: CameraRelativePose {
                pos: world_offset,
                orient: glam::Quat::IDENTITY,
                rel_scale: 0.1,
                distance_render_units: world_offset.length() as f64,
                angular_rad: 0.1,
                is_far_shell: false,
            },
            lod: LodTier::Hero,
            salience_score: 1.0,
        };

        let focus = resolve_focus_for_visible(&index, &vis, None, 2.0, &cam_pose);
        match focus {
            TargetFocus::ChromaticLocal(local) => {
                assert!(
                    (local - world_offset.as_dvec3()).length() < 1e-5,
                    "expected un-double-rotated local hit {world_offset:?}, got {local:?}"
                );
            }
            other => panic!("expected ChromaticLocal, got {other:?}"),
        }
    }

    #[test]
    fn catalog_earth_keeps_its_center_and_hides_schematic_selector_sprites() {
        let assets = UniverseAssets::load().expect("workspace assets");
        let mut builder = cosmos_render::UniverseSceneBuilder::new();
        let camera = cosmos_render::RenderCamera::mono(glam::Quat::IDENTITY, 72.0_f32.to_radians(), 1.0, 0.02);
        let mut input = cosmos_render::SceneBuildInput::desktop_default(
            ScalePose64::new(SectorId::EARTH_ORBIT, DVec3::new(0.0, 0.0, 3.0), 6.6, DQuat::IDENTITY),
            camera, 384, 384,
        );
        let query = assets.spatial.query_visible(&input.cam_pose, camera.eyes[0].view_proj, 0.001, None).unwrap();
        let selectors: Vec<_> = query.visible.iter().filter(|entity| {
            entity.experience_id == 3 && matches!(entity.kind, EntityKind::ZoneShell | EntityKind::FeatureNode)
        }).collect();
        assert!(!selectors.is_empty(), "the actual catalog must exercise selector filtering");
        let mut earth_bounds = Vec::new();
        for zone in [0, 2, 5, 6] {
            input.documentary.active_zone_index = zone;
            let scene = builder.build(&assets.spatial, &input);
            earth_bounds.push(scene.heroes_surface.iter().find(|h| h.kind_tags[0] == 0).unwrap().pos_radius);
            for selector in &selectors {
                assert!(!scene.sprites.iter().any(|sprite| {
                    let selector_meters = selector.pose.pos * 10.0_f64.powf(input.cam_pose.scale_exp) as f32;
                    Vec3::from_slice(&sprite.pos_size).distance(selector_meters) < 1.0
                }), "catalog node {} must not become a false atmospheric particle", selector.id);
            }
        }
        assert!(earth_bounds.windows(2).all(|pair| pair[0] == pair[1]),
            "actual catalog zone changes must not relocate or resize Earth");
    }

    #[test]
    fn exterior_earth_labels_keep_real_anchors_without_catalog_selector_clutter() {
        let assets = UniverseAssets::load().expect("workspace assets");
        let entities = [
            assets.spatial.find_experience_anchor(3).unwrap(),
            assets.spatial.find_zone_entity(3, 2).unwrap(),
            assets.spatial.find_feature_entity(3, 2, 0).unwrap(),
        ];
        let visible: Vec<_> = entities.into_iter().map(|entity| VisibleEntity {
            id: entity.id,
            experience_id: entity.experience_id,
            zone_index: entity.zone_index,
            feature_index: entity.feature_index,
            kind: entity.kind,
            pose: CameraRelativePose {
                pos: Vec3::new(0.0, 0.2, -3.0),
                orient: glam::Quat::IDENTITY,
                rel_scale: 0.2,
                distance_render_units: 3.0,
                angular_rad: 0.1,
                is_far_shell: false,
            },
            lod: LodTier::Hero,
            salience_score: 1.0,
        }).collect();
        for zone in [0, 2, 5, 6, u16::MAX] {
            let labels = build_ambient_label_inputs(
                &assets.catalog, &visible, 5.65, 3, zone, None, Locale::EnUs,
            );
            assert_eq!(labels.len(), 1,
                "exterior Z{zone} keeps the Earth anchor, but no schematic zone/feature labels");
        }
        let interior = build_ambient_label_inputs(
            &assets.catalog, &visible, 5.65, 3, 12, None, Locale::EnUs,
        );
        assert_eq!(interior.len(), 3, "interior layer annotation remains available");
        assert!(assets.spatial.find_feature_entity(3, 2, 0).is_some(),
            "hiding schematic labels must not remove indexed navigation targets");
    }

    #[test]
    fn ambient_labels_prevent_cross_experience_bleed() {
        let assets = UniverseAssets::load().expect("workspace assets");
        let vis_milk = VisibleEntity {
            id: 9001,
            experience_id: 5,
            zone_index: 0,
            feature_index: 0,
            kind: EntityKind::GalacticLandmark,
            pose: CameraRelativePose {
                pos: Vec3::new(0.0, 0.2, -2.0),
                orient: glam::Quat::IDENTITY,
                rel_scale: 0.5,
                distance_render_units: 2.0,
                angular_rad: 0.25,
                is_far_shell: false,
            },
            lod: LodTier::Impostor,
            salience_score: 1.5,
        };
        let labels = build_ambient_label_inputs(
            &assets.catalog,
            &[vis_milk],
            12.8,
            4,
            u16::MAX,
            None,
            Locale::EnUs,
        );
        assert!(
            labels.is_empty(),
            "05 Milky Way GalacticLandmark must not bleed into 04 Solar System: {labels:?}"
        );
    }
}
