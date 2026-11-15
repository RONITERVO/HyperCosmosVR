use std::collections::HashMap;

use cosmos_manifest::{AudioAssetRecord, CueRecord, UniverseCatalog};
use cosmos_math::smoothstep;
use glam::Vec3;

use crate::atlas::{FontRole, MsdfAtlas};
use crate::gpu::{GlyphInstance, TetherInstance};

/// Minimum distance clamp (meters) in apparent angular label sizing.
pub const ANGULAR_DIST_MIN_M: f32 = 0.85;
/// Maximum distance clamp (meters) in apparent angular label sizing.
pub const ANGULAR_DIST_MAX_M: f32 = 18.0;
/// Maximum simultaneous ambient 3D labels visible per frame (`universe_architecture.md` §6.3).
pub const DEFAULT_MAX_AMBIENT_LABELS: usize = 18;
/// Bounded vertical angular nudge budget (`±1.5°` in radians).
pub const DEFAULT_MAX_VERTICAL_NUDGE_RAD: f32 = 1.5_f32 * std::f32::consts::PI / 180.0;
/// Comfortable VR stereo vergence band `[1.6 m, 2.8 m]`.
pub const DEFAULT_VERGENCE_MIN_M: f32 = 1.6;
pub const DEFAULT_VERGENCE_MAX_M: f32 = 2.8;

const DEFAULT_OBSIDIAN_HALO: [f32; 4] = [0.002, 0.003, 0.006, 0.94];

/// Shader live-ink mode matching `msdf_text.wgsl` (`GlyphInstance::ink_params.w`).
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GlyphInkMode {
    SpatialAnchor = 0,
    LiveInkActive = 1,
    SpanishGhostRibbon = 2,
    RememberedMuted = 3,
    DwellFocusedHeading = 4,
    /// Solid, softly edged world-space quad behind a spatial control (no atlas sampling).
    ControlBackdrop = 5,
}

/// How an ambient label's radial placement handles distant/far-shell entities.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AnchorMode {
    /// Place at exact `world_pos_cam_rel`; apparent size scales via `clamp(dist, 0.85, 18.0)`.
    WorldExact,
    /// Clamp distance along camera-to-entity ray into `[min_m, max_m]` so distant/far-shell
    /// landmarks remain legible at their target angular size without shrinking subpixel.
    ComfortDistanceBand { min_m: f32, max_m: f32 },
}

impl Default for AnchorMode {
    fn default() -> Self {
        Self::ComfortDistanceBand {
            min_m: ANGULAR_DIST_MIN_M,
            max_m: ANGULAR_DIST_MAX_M,
        }
    }
}

/// Camera orientation & projection parameters needed for 3D label layout.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CameraBasis {
    pub eye_center: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub fov_y_rad: f32,
    pub aspect: f32,
    pub u_cam: f64,
    /// World horizon up vector (`Vec3::Y` default) used to horizon-stabilize ambient labels.
    pub horizon_up: Vec3,
}

impl Default for CameraBasis {
    fn default() -> Self {
        Self {
            eye_center: Vec3::ZERO,
            forward: Vec3::NEG_Z,
            right: Vec3::X,
            up: Vec3::Y,
            fov_y_rad: 90.0_f32.to_radians(),
            aspect: 16.0 / 9.0,
            u_cam: 6.6,
            horizon_up: Vec3::Y,
        }
    }
}

/// Input specification for one Google-Earth ambient world-space label.
#[derive(Clone, Debug)]
pub struct AmbientLabelInput {
    pub id: u64,
    pub text: String,
    pub role: FontRole,
    pub world_pos_cam_rel: Vec3,
    /// Target vertical line height in radians (`0.85°..1.3°` major, `0.55°..0.75°` minor).
    pub target_angle_rad: f32,
    /// Pre-scored salience (`TierWeight * FocusProximity * Centrality`).
    pub salience: f32,
    /// Scale band `[u_min, u_max]` (`log10` meters) where this label is visible.
    pub u_min: f64,
    pub u_max: f64,
    /// Width of smoothstep fade ramp at scale-band boundaries (`log10` meters).
    pub fade_band_u: f64,
    pub color_hdr: [f32; 4],
    pub halo_color: [f32; 4],
    pub mode: GlyphInkMode,
    pub anchor_mode: AnchorMode,
    /// Optional gentle cylindrical curve radius (meters, `0.0` = flat billboard).
    pub arc_radius_m: f32,
    /// Force emitting a hairline tether back to `world_pos_cam_rel` even without vertical nudge.
    pub tether_to_world: bool,
}

impl AmbientLabelInput {
    pub fn landmark(
        id: u64,
        text: impl Into<String>,
        world_pos_cam_rel: Vec3,
        salience: f32,
        u_min: f64,
        u_max: f64,
    ) -> Self {
        Self {
            id,
            text: text.into(),
            role: FontRole::UiSans,
            world_pos_cam_rel,
            target_angle_rad: 1.05_f32.to_radians(),
            salience,
            u_min,
            u_max,
            fade_band_u: 0.45,
            color_hdr: [0.92, 0.96, 1.02, 0.95],
            halo_color: DEFAULT_OBSIDIAN_HALO,
            mode: GlyphInkMode::SpatialAnchor,
            anchor_mode: AnchorMode::default(),
            arc_radius_m: 0.0,
            tether_to_world: false,
        }
    }
}

/// Bilingual SyncVoice display phase for the active spatial narration block.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BilingualPhase {
    /// Phase 1: `es-ES` speech active, sweeping Spanish live-ink `[0.0..1.0]`.
    SpanishActive { es_reveal: f32 },
    /// Phase 2: Spanish line dims to `16%` ghost calligraphy (`mode = 2`), while `en-US`/`fi-FI`
    /// translation sweeps `[0.0..1.0]` in bright HDR ink + trailing nib spark.
    TranslationOverGhost { trans_reveal: f32 },
    /// Completed resting bilingual state.
    Complete,
}

/// Active rest-to-learn bilingual narration spatial block placed at stereo vergence `1.6..2.8 m`.
#[derive(Clone, Debug)]
pub struct BilingualNarrationSpec {
    /// Exact 3D world coordinate of the focused feature (camera-relative meters).
    pub anchor_world_pos: Vec3,
    /// Optional ray hit distance to an opaque planetary/object surface to pull text in front of it.
    pub surface_distance_m: Option<f32>,
    /// Optional `UiSans` eyebrow tag (e.g. `"ESPAÑOL → ENGLISH · INFERENCIA PROFUNDA"`).
    pub eyebrow: Option<String>,
    /// Already-heard `region-heading` / `shared-context` lines displayed at calm ~35% `mode = 3`.
    pub remembered_lines: Vec<String>,
    /// Optional unskipped zone/item heading above the active narration phrase.
    pub active_heading: Option<String>,
    /// Primary Spanish phrase (`DisplayScript`).
    pub spanish_text: String,
    /// Translation phrase (`en-US` / `fi-FI`, `DisplayScript`).
    pub translation_text: Option<String>,
    /// Optional footer banner (Experiences `21..25`, `UiSans`).
    pub footer_text: Option<String>,
    /// Current SyncVoice live-ink phase & sweep cursor.
    pub phase: BilingualPhase,
    /// Active cosmic zone HDR accent RGB for tether and nib spark.
    pub accent_rgb: [f32; 3],
    /// Vertical line height for the main live-ink line (`1.6°..2.2°` default `1.70°`).
    pub base_line_angle_rad: f32,
    /// Maximum horizontal line width in `em` units before word-wrapping.
    pub max_line_width_em: f32,
    pub vergence_min_m: f32,
    pub vergence_max_m: f32,
    /// Curve lines along a comfortable constant-vergence cylindrical arc centered on the viewer.
    pub arc_curve: bool,
    /// Global opacity multiplier `[0..1]` (e.g. during 120ms cancel fade).
    pub opacity: f32,
}

impl Default for BilingualNarrationSpec {
    fn default() -> Self {
        Self {
            anchor_world_pos: Vec3::new(0.0, 0.0, -2.2),
            surface_distance_m: None,
            eyebrow: None,
            remembered_lines: Vec::new(),
            active_heading: None,
            spanish_text: String::new(),
            translation_text: None,
            footer_text: None,
            phase: BilingualPhase::SpanishActive { es_reveal: 1.0 },
            accent_rgb: [0.42, 0.86, 1.25],
            base_line_angle_rad: 1.70_f32.to_radians(),
            max_line_width_em: 24.0,
            vergence_min_m: DEFAULT_VERGENCE_MIN_M,
            vergence_max_m: DEFAULT_VERGENCE_MAX_M,
            arc_curve: true,
            opacity: 1.0,
        }
    }
}

/// Center-eye angular bounding box in `(azimuth, elevation)` radians.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AngularRect {
    pub az_center: f32,
    pub el_center: f32,
    pub half_az: f32,
    pub half_el: f32,
}

impl AngularRect {
    /// Constructs an angular bounding box from center and half-extents in degrees.
    #[inline]
    pub fn from_deg(az_center_deg: f32, el_center_deg: f32, half_az_deg: f32, half_el_deg: f32) -> Self {
        Self {
            az_center: az_center_deg.to_radians(),
            el_center: el_center_deg.to_radians(),
            half_az: half_az_deg.abs().to_radians(),
            half_el: half_el_deg.abs().to_radians(),
        }
    }

    /// Projects a camera-relative 3D bounding sphere into a conservative center-eye [`AngularRect`].
    ///
    /// Returns `None` if the sphere is behind the view hemisphere.
    pub fn from_world_sphere(
        camera: &CameraBasis,
        center_world_cam_rel: Vec3,
        radius_m: f32,
        margin_rad: f32,
    ) -> Option<Self> {
        let fwd = camera.forward.normalize_or(Vec3::NEG_Z);
        let right = camera.right.normalize_or(Vec3::X);
        let up = camera.up.normalize_or(Vec3::Y);
        let rel = center_world_cam_rel - camera.eye_center;
        let dist = rel.length();
        if dist <= 1e-4 {
            return None;
        }
        let dir = rel / dist;
        let fwd_c = dir.dot(fwd);
        if fwd_c <= 0.05 {
            return None;
        }
        let az = dir.dot(right).atan2(fwd_c);
        let el = dir.dot(up).clamp(-0.999, 0.999).asin();
        let half_ang = if dist <= radius_m {
            0.45 * camera.fov_y_rad
        } else {
            (radius_m / dist).clamp(0.0, 0.999).asin()
        } + margin_rad.max(0.0);
        Some(Self {
            az_center: az,
            el_center: el,
            half_az: (half_ang / el.cos().max(0.35)).min(1.4),
            half_el: half_ang.min(1.3),
        })
    }

    #[inline]
    pub fn overlaps(&self, other: &Self) -> bool {
        let d_az = (self.az_center - other.az_center).abs();
        let d_el = (self.el_center - other.el_center).abs();
        d_az < (self.half_az + other.half_az) && d_el < (self.half_el + other.half_el)
    }

    #[inline]
    pub fn overlap_area(&self, other: &Self) -> f32 {
        let ox = (self.half_az + other.half_az) - (self.az_center - other.az_center).abs();
        let oy = (self.half_el + other.half_el) - (self.el_center - other.el_center).abs();
        if ox > 0.0 && oy > 0.0 {
            ox * oy
        } else {
            0.0
        }
    }
}

/// Candidate submitted to the `O(M log M)` angular spatial-hash de-overlap solver.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AmbientCandidateAngular {
    pub id: u64,
    pub salience: f32,
    pub rect: AngularRect,
}

/// Output placement of a surviving candidate from [`solve_ambient_deoverlap`].
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DeoverlapPlacement {
    pub id: u64,
    pub candidate_index: usize,
    pub placed_rect: AngularRect,
    pub nudge_el_rad: f32,
}

/// Metadata for an ambient label placed in the current frame.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlacedAmbientLabel {
    pub id: u64,
    pub rect: AngularRect,
    pub nudge_el_rad: f32,
    pub world_pos: Vec3,
    pub alpha: f32,
    pub glyph_start: usize,
    pub glyph_end: usize,
    pub tether_index: Option<usize>,
}

/// Complete GPU-ready typography output for one frame.
#[derive(Clone, Debug, Default)]
pub struct TypoFrameOutput {
    pub glyphs: Vec<GlyphInstance>,
    pub tethers: Vec<TetherInstance>,
    /// First foreground glyph and tether; trailing instances are spatial controls.
    pub foreground_start: Option<(usize, usize)>,
    pub placed_ambient: Vec<PlacedAmbientLabel>,
    pub ambient_culled_count: usize,
    pub active_narration_rect: Option<AngularRect>,
    pub active_narration_distance_m: Option<f32>,
}

/// Computes apparent world height (meters) clamped smoothly in `[0.85 m, 18.0 m]`:
/// `world_height = clamp(dist, 0.85, 18.0) * 2.0 * tan(target_angle_rad * 0.5)`.
#[inline]
pub fn apparent_world_height(dist_to_eye: f32, target_angle_rad: f32) -> f32 {
    let d = dist_to_eye.clamp(ANGULAR_DIST_MIN_M, ANGULAR_DIST_MAX_M);
    d * 2.0 * (0.5 * target_angle_rad.max(0.0)).tan()
}

/// Smoothstep scale-band visibility factor in `[0.0, 1.0]` for log10 scale `u_cam`.
#[inline]
pub fn scale_band_alpha(u_cam: f64, u_min: f64, u_max: f64, fade_band_u: f64) -> f32 {
    if u_cam < u_min || u_cam > u_max || u_max <= u_min {
        return 0.0;
    }
    let band = fade_band_u.max(1e-4).min(0.45 * (u_max - u_min));
    let enter = smoothstep(u_min, u_min + band, u_cam);
    let exit = 1.0 - smoothstep(u_max - band, u_max, u_cam);
    (enter * exit) as f32
}

/// Computes continuous normalized SyncVoice sweep cursor (`0.0 -> 1.0`) from UTF-8 [`CueRecord`] timings.
pub fn cue_reveal_cursor_for_cues(
    cues: &[CueRecord],
    total_scalars: usize,
    duration_ms: u32,
    elapsed_ms: u32,
) -> f32 {
    if total_scalars == 0 {
        return 1.0;
    }
    if cues.is_empty() {
        return (elapsed_ms as f32 / duration_ms.max(1) as f32).clamp(0.0, 1.0);
    }

    let inv_total = 1.0 / total_scalars as f32;
    let mut prev_end_scalar = 0u32;

    for cue in cues {
        if elapsed_ms < cue.start_ms {
            return (prev_end_scalar as f32 * inv_total).clamp(0.0, 1.0);
        }
        if elapsed_ms < cue.end_ms {
            let span_ms = (cue.end_ms - cue.start_ms).max(1) as f32;
            let frac = (elapsed_ms - cue.start_ms) as f32 / span_ms;
            let scalar = cue.start_scalar as f32
                + frac * cue.end_scalar.saturating_sub(cue.start_scalar) as f32;
            return (scalar * inv_total).clamp(0.0, 1.0);
        }
        prev_end_scalar = cue.end_scalar;
    }

    1.0
}

/// Convenience wrapper computing normalized SyncVoice live-ink cursor (`0.0 -> 1.0`) directly from
/// a [`UniverseCatalog`] [`AudioAssetRecord`].
pub fn cue_reveal_cursor_for_asset(
    catalog: &UniverseCatalog,
    asset: &AudioAssetRecord,
    elapsed_ms: u32,
) -> f32 {
    let cues = catalog.asset_cues(asset);
    let text = catalog.asset_text(asset);
    cue_reveal_cursor_for_cues(cues, text.chars().count(), asset.duration_ms, elapsed_ms)
}

/// `O(M log M)` angular spatial-hash collision solver with bounded `±1.5°` vertical nudge.
///
/// Guarantees **zero pairwise overlap** across all returned [`DeoverlapPlacement`] rectangles and
/// against all pre-claimed `blocker_rects` (e.g. active live-ink narration block).
pub fn solve_ambient_deoverlap(
    candidates: &[AmbientCandidateAngular],
    blocker_rects: &[AngularRect],
    max_visible: usize,
    max_vertical_nudge_rad: f32,
) -> Vec<DeoverlapPlacement> {
    solve_ambient_deoverlap_with_dots(
        candidates,
        blocker_rects,
        None,
        max_visible,
        max_vertical_nudge_rad,
    )
}

fn solve_ambient_deoverlap_with_dots(
    candidates: &[AmbientCandidateAngular],
    blocker_rects: &[AngularRect],
    anchor_dots: Option<&[Option<AngularRect>]>,
    max_visible: usize,
    max_vertical_nudge_rad: f32,
) -> Vec<DeoverlapPlacement> {
    if candidates.is_empty() || max_visible == 0 {
        return Vec::new();
    }

    let mut order: Vec<usize> = (0..candidates.len()).collect();
    order.sort_by(|&a, &b| {
        candidates[b]
            .salience
            .total_cmp(&candidates[a].salience)
            .then_with(|| candidates[a].id.cmp(&candidates[b].id))
    });

    let cell_az = 2.0_f32.to_radians();
    let cell_el = 1.0_f32.to_radians();
    let mut occupied: Vec<AngularRect> = Vec::with_capacity(blocker_rects.len() + max_visible * 2);
    let mut grid: HashMap<(i32, i32), Vec<usize>> =
        HashMap::with_capacity((blocker_rects.len() + max_visible) * 4);

    let insert_rect = |rect: AngularRect,
                       occupied: &mut Vec<AngularRect>,
                       grid: &mut HashMap<(i32, i32), Vec<usize>>| {
        let idx = occupied.len();
        occupied.push(rect);
        let (x0, x1, y0, y1) = cell_bounds(rect, cell_az, cell_el);
        for gx in x0..=x1 {
            for gy in y0..=y1 {
                grid.entry((gx, gy)).or_default().push(idx);
            }
        }
    };

    for &b in blocker_rects {
        insert_rect(b, &mut occupied, &mut grid);
    }
    if let Some(dots) = anchor_dots {
        for dot_rect in dots.iter().flatten() {
            insert_rect(*dot_rect, &mut occupied, &mut grid);
        }
    }

    let mut placements = Vec::with_capacity(max_visible.min(candidates.len()));
    let mut colliding_scratch = Vec::<usize>::with_capacity(16);
    let mut nudge_probes = Vec::<f32>::with_capacity(24);
    let eps = 0.22_f32.to_radians();

    for cand_idx in order {
        if placements.len() >= max_visible {
            break;
        }
        let cand = candidates[cand_idx];

        collect_cell_neighbors(cand.rect, cell_az, cell_el, &grid, &mut colliding_scratch);
        if !has_overlap(cand.rect, &colliding_scratch, &occupied) {
            insert_rect(cand.rect, &mut occupied, &mut grid);
            placements.push(DeoverlapPlacement {
                id: cand.id,
                candidate_index: cand_idx,
                placed_rect: cand.rect,
                nudge_el_rad: 0.0,
            });
            continue;
        }

        // Gather candidate vertical shifts within `[-max_vertical_nudge_rad, +max_vertical_nudge_rad]`:
        // exact clearing offsets around neighboring rects plus uniform micro-steps, smallest shift first.
        nudge_probes.clear();
        let search_rect = AngularRect {
            az_center: cand.rect.az_center,
            el_center: cand.rect.el_center,
            half_az: cand.rect.half_az,
            half_el: cand.rect.half_el + max_vertical_nudge_rad,
        };
        collect_cell_neighbors(search_rect, cell_az, cell_el, &grid, &mut colliding_scratch);
        for &occ_idx in &colliding_scratch {
            let occ = occupied[occ_idx];
            if (cand.rect.az_center - occ.az_center).abs() < (cand.rect.half_az + occ.half_az) {
                let req_dist = occ.half_el + cand.rect.half_el + eps;
                let above = (occ.el_center + req_dist) - cand.rect.el_center;
                let below = (occ.el_center - req_dist) - cand.rect.el_center;
                if above.abs() <= max_vertical_nudge_rad {
                    nudge_probes.push(above);
                }
                if below.abs() <= max_vertical_nudge_rad {
                    nudge_probes.push(below);
                }
            }
        }
        for step_deg in [0.35_f32, 0.70, 1.05, 1.40] {
            let rad = step_deg.to_radians();
            if rad <= max_vertical_nudge_rad {
                nudge_probes.push(rad);
                nudge_probes.push(-rad);
            }
        }
        nudge_probes.sort_by(|a, b| a.abs().total_cmp(&b.abs()));

        let mut chosen: Option<(AngularRect, f32)> = None;
        for &nudge in &nudge_probes {
            let shifted = AngularRect {
                el_center: cand.rect.el_center + nudge,
                ..cand.rect
            };
            if !has_overlap(shifted, &colliding_scratch, &occupied) {
                chosen = Some((shifted, nudge));
                break;
            }
        }

        if let Some((placed_rect, nudge_el_rad)) = chosen {
            insert_rect(placed_rect, &mut occupied, &mut grid);
            placements.push(DeoverlapPlacement {
                id: cand.id,
                candidate_index: cand_idx,
                placed_rect,
                nudge_el_rad,
            });
        }
    }

    placements
}

#[inline]
fn cell_bounds(rect: AngularRect, cell_az: f32, cell_el: f32) -> (i32, i32, i32, i32) {
    let x0 = ((rect.az_center - rect.half_az) / cell_az).floor() as i32;
    let x1 = ((rect.az_center + rect.half_az) / cell_az).floor() as i32;
    let y0 = ((rect.el_center - rect.half_el) / cell_el).floor() as i32;
    let y1 = ((rect.el_center + rect.half_el) / cell_el).floor() as i32;
    (x0, x1, y0, y1)
}

fn collect_cell_neighbors(
    rect: AngularRect,
    cell_az: f32,
    cell_el: f32,
    grid: &HashMap<(i32, i32), Vec<usize>>,
    out: &mut Vec<usize>,
) {
    out.clear();
    let (x0, x1, y0, y1) = cell_bounds(rect, cell_az, cell_el);
    for gx in x0..=x1 {
        for gy in y0..=y1 {
            if let Some(bucket) = grid.get(&(gx, gy)) {
                out.extend_from_slice(bucket);
            }
        }
    }
    out.sort_unstable();
    out.dedup();
}

#[inline]
fn has_overlap(rect: AngularRect, neighbor_indices: &[usize], occupied: &[AngularRect]) -> bool {
    neighbor_indices
        .iter()
        .any(|&idx| rect.overlaps(&occupied[idx]))
}

/// Temporal smooth fade tracker preventing abrupt pops when labels appear or shift scale bands.
#[derive(Clone, Debug)]
pub struct TypoLayoutEngine {
    fade_alphas: HashMap<u64, f32>,
    pub max_ambient_labels: usize,
    pub max_vertical_nudge_rad: f32,
    pub fade_tau_sec: f32,
    /// Optional per-frame angular blockers (e.g., Wrist Resonance Ring, focal hero disc) avoided by
    /// both the active bilingual narration block and all ambient Google-Earth labels.
    pub external_blockers: Vec<AngularRect>,
}

impl Default for TypoLayoutEngine {
    fn default() -> Self {
        Self {
            fade_alphas: HashMap::new(),
            max_ambient_labels: DEFAULT_MAX_AMBIENT_LABELS,
            max_vertical_nudge_rad: DEFAULT_MAX_VERTICAL_NUDGE_RAD,
            fade_tau_sec: 0.12,
            external_blockers: Vec::new(),
        }
    }
}

impl TypoLayoutEngine {
    /// Updates stored angular exclusion zones (such as the Wrist Resonance Ring or hero focal core).
    pub fn set_external_blockers(&mut self, blockers: impl Into<Vec<AngularRect>>) {
        self.external_blockers = blockers.into();
    }

    /// Builds a frame of world-space typography using [`Self::external_blockers`] as exclusion zones.
    pub fn build_frame(
        &mut self,
        atlas: &MsdfAtlas,
        camera: &CameraBasis,
        ambient: &[AmbientLabelInput],
        narration: Option<&BilingualNarrationSpec>,
        dt_sec: f32,
    ) -> TypoFrameOutput {
        let mut out = build_frame_typography_with_blockers(
            atlas,
            camera,
            ambient,
            narration,
            &self.external_blockers,
            self.max_ambient_labels,
            self.max_vertical_nudge_rad,
        );
        self.apply_temporal_fades(&mut out, dt_sec);
        out
    }

    /// Builds a frame of world-space typography while avoiding explicit `external_blockers`.
    pub fn build_frame_with_blockers(
        &mut self,
        atlas: &MsdfAtlas,
        camera: &CameraBasis,
        ambient: &[AmbientLabelInput],
        narration: Option<&BilingualNarrationSpec>,
        external_blockers: &[AngularRect],
        dt_sec: f32,
    ) -> TypoFrameOutput {
        let mut out = build_frame_typography_with_blockers(
            atlas,
            camera,
            ambient,
            narration,
            external_blockers,
            self.max_ambient_labels,
            self.max_vertical_nudge_rad,
        );
        self.apply_temporal_fades(&mut out, dt_sec);
        out
    }

    fn apply_temporal_fades(&mut self, out: &mut TypoFrameOutput, dt_sec: f32) {
        let alpha_step = if dt_sec > 0.0 && self.fade_tau_sec > 1e-4 {
            1.0 - (-dt_sec / self.fade_tau_sec).exp()
        } else {
            1.0
        };

        let mut active_ids = Vec::with_capacity(out.placed_ambient.len());
        for placed in &mut out.placed_ambient {
            active_ids.push(placed.id);
            let target_alpha = placed.alpha;
            let prev = self.fade_alphas.get(&placed.id).copied().unwrap_or(0.0);
            let next = prev + (target_alpha - prev) * alpha_step;
            self.fade_alphas.insert(placed.id, next);
            placed.alpha = next;

            let fade_ratio = if target_alpha > 1e-5 {
                (next / target_alpha).clamp(0.0, 1.0)
            } else {
                0.0
            };
            for g in &mut out.glyphs[placed.glyph_start..placed.glyph_end] {
                g.color_hdr[3] *= fade_ratio;
                g.halo_color[3] *= fade_ratio;
            }
            if let Some(ti) = placed.tether_index {
                out.tethers[ti].p1_alpha[3] *= fade_ratio;
            }
        }
        self.fade_alphas.retain(|id, _| active_ids.contains(id));
    }
}

/// Builds all 3D world-space `GlyphInstance`s and `TetherInstance`s for one frame.
///
/// 1. Places the active bilingual live-ink narration block at stereo vergence (`1.6..2.8 m`) on an
///    unobstructed offset shelf with a glowing [`TetherInstance`] filament back to the 3D anchor.
/// 2. Solves the `O(M log M)` angular de-overlap problem for ambient Google-Earth labels (excluding
///    the active narration angular footprint), guaranteeing zero overlapping visible labels.
pub fn build_frame_typography(
    atlas: &MsdfAtlas,
    camera: &CameraBasis,
    ambient: &[AmbientLabelInput],
    narration: Option<&BilingualNarrationSpec>,
    max_ambient_labels: usize,
    max_vertical_nudge_rad: f32,
) -> TypoFrameOutput {
    build_frame_typography_with_blockers(
        atlas,
        camera,
        ambient,
        narration,
        &[],
        max_ambient_labels,
        max_vertical_nudge_rad,
    )
}

/// Like [`build_frame_typography`], plus explicit angular `external_blockers` (such as the
/// diegetic Wrist Resonance Ring or focal hero core) excluded from both narration and ambient text.
pub fn build_frame_typography_with_blockers(
    atlas: &MsdfAtlas,
    camera: &CameraBasis,
    ambient: &[AmbientLabelInput],
    narration: Option<&BilingualNarrationSpec>,
    external_blockers: &[AngularRect],
    max_ambient_labels: usize,
    max_vertical_nudge_rad: f32,
) -> TypoFrameOutput {
    let mut output = TypoFrameOutput::default();
    let mut blockers = Vec::with_capacity(external_blockers.len() + 1);
    blockers.extend_from_slice(external_blockers);

    struct PreparedAmbient<'a> {
        input: &'a AmbientLabelInput,
        shaped: crate::atlas::ShapedText,
        placed_dist: f32,
        world_height: f32,
        world_w_m: f32,
        world_h_m: f32,
        amb_line_y_offset_m: f32,
        scale_alpha: f32,
        az: f32,
        el: f32,
        dot_guard: Option<AngularRect>,
        el_lift_rad: f32,
        canon_full: String,
        canon_left: Option<String>,
    }

    let fwd = camera.forward.normalize_or(Vec3::NEG_Z);
    let right = camera.right.normalize_or(Vec3::X);
    let up = camera.up.normalize_or(Vec3::Y);
    let (level_right, level_up) = camera_level_basis(camera);
    let half_fov_y = (0.5 * camera.fov_y_rad).clamp(0.18, 1.35);
    let half_fov_x = (half_fov_y.tan() * camera.aspect.max(0.6)).atan();
    let min_fwd_cos = (0.5 * camera.fov_y_rad * camera.aspect.max(1.0) + 0.08).cos().max(0.05);
    let pad_az = 0.85_f32.to_radians();
    let pad_el = 0.65_f32.to_radians();
    let frustum_margin = 1.0_f32.to_radians();

    let mut raw_prepared = Vec::with_capacity(ambient.len());
    let mut narration_dot_guards = Vec::<AngularRect>::new();
    for item in ambient {
        let scale_alpha = scale_band_alpha(camera.u_cam, item.u_min, item.u_max, item.fade_band_u);
        if scale_alpha <= 0.005 || item.text.is_empty() {
            continue;
        }

        let rel = item.world_pos_cam_rel - camera.eye_center;
        let raw_dist = rel.length();
        if raw_dist <= 1e-4 {
            continue;
        }
        let dir = rel / raw_dist;
        let fwd_c = dir.dot(fwd);
        if fwd_c < min_fwd_cos {
            continue;
        }

        let placed_dist = match item.anchor_mode {
            AnchorMode::WorldExact => raw_dist,
            AnchorMode::ComfortDistanceBand { min_m, max_m } => {
                // Pull distant anchors inward for legibility without shoving close-up diorama
                // labels (< min_m, e.g. 0.85..1.49 m) backward behind the 3D geometry they label.
                let safe_lo = min_m.min(raw_dist.max(ANGULAR_DIST_MIN_M));
                raw_dist.clamp(safe_lo, max_m.max(safe_lo))
            }
        };
        let world_height = apparent_world_height(placed_dist, item.target_angle_rad);

        let shaped = atlas.shape_text(item.role, &item.text, None);
        if shaped.glyphs.is_empty() || shaped.max_width_em <= 1e-4 {
            continue;
        }

        let (min_x_em, max_x_em, min_y_em, max_y_em) = shaped_visual_bounds_em(&shaped);
        let half_w_em = max_x_em
            .abs()
            .max(min_x_em.abs())
            .max(0.5 * shaped.max_width_em)
            + 0.12;
        let half_h_em = (0.5 * (max_y_em - min_y_em))
            .max(0.5 * shaped.total_height_em)
            + 0.12;
        // Center ambient glyph quads vertically around `final_pos` instead of leaving the
        // baseline at `y = 0`, so actual top ascenders never protrude above `AngularRect`.
        let amb_line_y_offset_m = -0.5 * (min_y_em + max_y_em) * world_height;
        let world_w_m = 2.0 * half_w_em * world_height;
        let world_h_m = 2.0 * half_h_em * world_height;
        let half_el_text = (0.5 * world_h_m / placed_dist).atan();

        let raw_az = dir.dot(right).atan2(fwd_c);
        let raw_anchor_el = dir.dot(up).clamp(-0.999, 0.999).asin();

        let is_meridian_with_ring = matches!(item.anchor_mode, AnchorMode::WorldExact)
            && item.text.starts_with("10^")
            && (item.text.contains('+') || item.text.contains('-') || item.text.ends_with(" m"));

        let (az, anchor_el, el_lift_rad, dot_guard) = if is_meridian_with_ring {
            // Scale Meridian places `world_pos_cam_rel` at `tick.world_pos + 0.055 * radial_right`.
            // Ensure `eff_az` sits strictly right of `(ring_right_az + half_az_text + pad_az)` so
            // wide `"10^+10 m"` labels clear the circular tick ring `⊙` cleanly in 3D perspective.
            let ring_radius_rad = (0.022 / raw_dist.clamp(ANGULAR_DIST_MIN_M, ANGULAR_DIST_MAX_M))
                .atan()
                .clamp(0.45_f32.to_radians(), 1.05_f32.to_radians());
            let ring_pos = item.world_pos_cam_rel - level_right * 0.055;
            let ring_rel = ring_pos - camera.eye_center;
            let (ring_guard, min_eff_az) = if ring_rel.length() > 1e-4 {
                let rdir = ring_rel.normalize();
                if rdir.dot(fwd) > 0.05 {
                    let ring_az = rdir.dot(right).atan2(rdir.dot(fwd));
                    let ring_el = rdir.dot(up).clamp(-0.999, 0.999).asin();
                    let cos_el = raw_anchor_el.cos().max(0.3);
                    let half_az_text = (0.5 * world_w_m / (placed_dist * cos_el)).atan();
                    let min_az = ring_az
                        + ring_radius_rad
                        + half_az_text
                        + pad_az.max(0.65_f32.to_radians())
                        + 0.12_f32.to_radians();
                    (
                        Some(AngularRect {
                            az_center: ring_az,
                            el_center: ring_el,
                            half_az: ring_radius_rad,
                            half_el: ring_radius_rad,
                        }),
                        min_az,
                    )
                } else {
                    (None, raw_az)
                }
            } else {
                (None, raw_az)
            };

            let eff_az = raw_az.max(min_eff_az);
            let eff_el = raw_anchor_el;
            if let Some(rg) = ring_guard {
                narration_dot_guards.push(rg);
            }
            let half_az_lbl = (0.5 * world_w_m / (placed_dist * eff_el.cos().max(0.3))).atan();
            narration_dot_guards.push(AngularRect {
                az_center: eff_az,
                el_center: eff_el,
                half_az: half_az_lbl + 0.5_f32.to_radians(),
                half_el: half_el_text + 0.5_f32.to_radians(),
            });
            (eff_az, eff_el, 0.0, ring_guard)
        } else if matches!(item.anchor_mode, AnchorMode::ComfortDistanceBand { .. }) {
            // Clear EVERY floating landmark label >= (dot_radius + 0.65 deg) above its 3D anchor dot
            // so neither dual-language separators nor single-language labels ever overlap dots/stars.
            let dot_radius_rad = (0.020 / raw_dist.clamp(ANGULAR_DIST_MIN_M, ANGULAR_DIST_MAX_M))
                .atan()
                .clamp(0.38_f32.to_radians(), 0.90_f32.to_radians());
            let lift = half_el_text + dot_radius_rad + pad_el + 0.12_f32.to_radians();
            let guard = AngularRect {
                az_center: raw_az,
                el_center: raw_anchor_el,
                half_az: dot_radius_rad + 0.04_f32.to_radians(),
                half_el: dot_radius_rad + 0.04_f32.to_radians(),
            };
            narration_dot_guards.push(guard);
            (raw_az, raw_anchor_el, lift, Some(guard))
        } else {
            narration_dot_guards.push(AngularRect {
                az_center: raw_az,
                el_center: raw_anchor_el,
                half_az: 1.0_f32.to_radians(),
                half_el: 0.9_f32.to_radians(),
            });
            (raw_az, raw_anchor_el, 0.0, None)
        };

        let el = (anchor_el + el_lift_rad).clamp(-1.54, 1.54);
        let (canon_full, canon_left) = canonical_ambient_keys(&item.text);

        raw_prepared.push(PreparedAmbient {
            input: item,
            shaped,
            placed_dist,
            world_height,
            world_w_m,
            world_h_m,
            amb_line_y_offset_m,
            scale_alpha,
            az,
            el,
            dot_guard,
            el_lift_rad,
            canon_full,
            canon_left,
        });
    }

    if let Some(spec) = narration
        && spec.opacity > 0.005
        && let Some((rect, dist_m)) = emit_bilingual_narration_block(
            atlas,
            camera,
            spec,
            external_blockers,
            &narration_dot_guards,
            &mut output.glyphs,
            &mut output.tethers,
        )
    {
        output.active_narration_rect = Some(rect);
        output.active_narration_distance_m = Some(dist_m);
        blockers.push(rect);
    }

    // Deduplicate identical/paired display labels in salience order before de-overlap solving
    // so neighboring catalog/star/zone items never show duplicate strings simultaneously.
    let mut dedup_order: Vec<usize> = (0..raw_prepared.len()).collect();
    dedup_order.sort_by(|&a, &b| {
        raw_prepared[b]
            .input
            .salience
            .total_cmp(&raw_prepared[a].input.salience)
            .then_with(|| raw_prepared[a].input.id.cmp(&raw_prepared[b].input.id))
    });

    let mut seen_full = HashMap::<String, ()>::with_capacity(raw_prepared.len());
    let mut seen_single = HashMap::<String, ()>::with_capacity(raw_prepared.len());
    let mut seen_lefts = HashMap::<String, ()>::with_capacity(raw_prepared.len());
    let mut prepared = Vec::with_capacity(raw_prepared.len());
    let mut angular_cands = Vec::with_capacity(raw_prepared.len());
    let mut anchor_dots = Vec::with_capacity(raw_prepared.len());

    for idx in dedup_order {
        let item = &raw_prepared[idx];
        let world_w = item.world_w_m;
        let world_h = item.world_h_m;
        let cos_el = item.el.cos().max(0.3);
        let half_el_text = (0.5 * world_h / item.placed_dist).atan();
        let half_az_text = (0.5 * world_w / (item.placed_dist * cos_el)).atan();
        let screen_el = (item.el.tan() / item.az.cos().max(0.25)).atan();
        // Skip peripheral labels that would clip past viewport edges and steal budget from on-screen entities.
        if item.az.abs() + half_az_text > half_fov_x - frustum_margin
            || screen_el.abs() + half_el_text > half_fov_y - frustum_margin
        {
            continue;
        }

        if item.canon_full.is_empty() || seen_full.contains_key(&item.canon_full) {
            continue;
        }
        if let Some(ref left) = item.canon_left {
            if seen_single.contains_key(left) {
                continue;
            }
        } else if seen_lefts.contains_key(&item.canon_full) {
            continue;
        }

        seen_full.insert(item.canon_full.clone(), ());
        if let Some(ref left) = item.canon_left {
            seen_lefts.insert(left.clone(), ());
        } else if item.canon_full.chars().count() >= 4 {
            seen_single.insert(item.canon_full.clone(), ());
        }

        let half_el = half_el_text + pad_el;
        let half_az = half_az_text + pad_az;

        angular_cands.push(AmbientCandidateAngular {
            id: item.input.id,
            salience: item.input.salience,
            rect: AngularRect {
                az_center: item.az,
                el_center: item.el,
                half_az,
                half_el,
            },
        });
        anchor_dots.push(item.dot_guard);
        prepared.push(raw_prepared[idx].input);
        let _ = idx;
    }

    // Re-align `prepared` with full shaped data via simple index lookup!
    let total_in_frustum = angular_cands.len();
    let placements = solve_ambient_deoverlap_with_dots(
        &angular_cands,
        &blockers,
        Some(&anchor_dots),
        max_ambient_labels,
        max_vertical_nudge_rad,
    );
    output.ambient_culled_count = raw_prepared.len().saturating_sub(placements.len());
    let _ = total_in_frustum;

    // Look up the surviving prepared items cleanly by index mapping.
    let _ = prepared;
    let mut surviving_map = HashMap::<u64, usize>::with_capacity(raw_prepared.len());
    for (i, rp) in raw_prepared.iter().enumerate() {
        surviving_map.entry(rp.input.id).or_insert(i);
    }

    for placement in placements {
        let rp_idx = surviving_map[&placement.id];
        let p = &raw_prepared[rp_idx];
        let new_az = p.az;
        let new_el = (p.el + placement.nudge_el_rad).clamp(-1.56, 1.56);
        let el_cos = new_el.cos();
        let nudged_dir = (fwd * (el_cos * new_az.cos())
            + right * (el_cos * new_az.sin())
            + up * new_el.sin())
            .normalize();
        let final_pos = camera.eye_center + nudged_dir * p.placed_dist;

        let (basis_right, basis_up) = (level_right, level_up);

        let mut color = p.input.color_hdr;
        color[3] *= p.scale_alpha;
        let mut halo = p.input.halo_color;
        halo[3] *= p.scale_alpha;

        let glyph_start = output.glyphs.len();
        emit_shaped_text_quads(
            &p.shaped,
            final_pos,
            p.amb_line_y_offset_m,
            camera.eye_center,
            basis_right,
            basis_up,
            p.world_height,
            p.input.arc_radius_m,
            0.024,
            0.18,
            color,
            halo,
            2.0, // static non-sweeping
            p.input.mode,
            true,
            &mut output.glyphs,
        );
        let glyph_end = output.glyphs.len();

        let tether_index = if p.input.tether_to_world
            || p.el_lift_rad > 0.002
            || placement.nudge_el_rad.abs() > 0.002
            || (final_pos - p.input.world_pos_cam_rel).length() > 0.15
        {
            let tether_start = compute_tether_start_on_screen_perimeter(
                camera,
                &output.glyphs[glyph_start..glyph_end],
                placement.placed_rect,
                final_pos,
                p.input.world_pos_cam_rel,
                p.placed_dist,
                0.003,
                0.001,
            );
            let idx = output.tethers.len();
            output.tethers.push(TetherInstance {
                p0_width: [tether_start.x, tether_start.y, tether_start.z, 0.0014],
                p1_alpha: [
                    p.input.world_pos_cam_rel.x,
                    p.input.world_pos_cam_rel.y,
                    p.input.world_pos_cam_rel.z,
                    0.55 * p.scale_alpha,
                ],
                color_hdr: [color[0], color[1], color[2], 0.2],
            });
            Some(idx)
        } else {
            None
        };

        output.placed_ambient.push(PlacedAmbientLabel {
            id: p.input.id,
            rect: placement.placed_rect,
            nudge_el_rad: placement.nudge_el_rad,
            world_pos: final_pos,
            alpha: p.scale_alpha,
            glyph_start,
            glyph_end,
            tether_index,
        });
    }

    output
}

struct BlockLine<'a> {
    text: &'a str,
    role: FontRole,
    scale_mul: f32,
    mode: GlyphInkMode,
    color_hdr: [f32; 4],
    halo_color: [f32; 4],
    reveal_cursor: f32,
    z_pull_m: f32,
    use_scalar_sweep: bool,
}

fn canonical_ambient_keys(text: &str) -> (String, Option<String>) {
    let collapse = |s: &str| -> String {
        let mut out = String::with_capacity(s.len());
        let mut prev_space = true;
        for ch in s.trim().chars() {
            if ch.is_whitespace() {
                if !prev_space {
                    out.push(' ');
                    prev_space = true;
                }
            } else {
                for lc in ch.to_lowercase() {
                    out.push(lc);
                }
                prev_space = false;
            }
        }
        out
    };
    let full = collapse(text);
    let left = text.split_once('·').and_then(|(l, _)| {
        let c = collapse(l);
        (c.chars().count() >= 4).then_some(c)
    });
    (full, left)
}

fn emit_bilingual_narration_block(
    atlas: &MsdfAtlas,
    camera: &CameraBasis,
    spec: &BilingualNarrationSpec,
    external_blockers: &[AngularRect],
    dot_guards: &[AngularRect],
    glyphs_out: &mut Vec<GlyphInstance>,
    tethers_out: &mut Vec<TetherInstance>,
) -> Option<(AngularRect, f32)> {
    let rel_anchor = spec.anchor_world_pos - camera.eye_center;
    let anchor_dist = rel_anchor.length();
    if anchor_dist < 1e-4 {
        return None;
    }
    let anchor_dir = rel_anchor / anchor_dist;

    let mut vergence_dist = anchor_dist.clamp(spec.vergence_min_m, spec.vergence_max_m);
    if let Some(surf_d) = spec.surface_distance_m {
        let min_safe = spec.vergence_min_m.min((surf_d - 0.25).max(ANGULAR_DIST_MIN_M));
        vergence_dist = (surf_d - 0.25)
            .clamp(min_safe, spec.vergence_max_m)
            .min(vergence_dist);
    }

    let script_line_h_em = atlas.font_metrics(FontRole::DisplayScript).line_height_em.max(1.0);
    let main_world_h =
        apparent_world_height(vergence_dist, spec.base_line_angle_rad) / script_line_h_em;
    let op = spec.opacity.clamp(0.0, 1.0);

    let mut lines = Vec::<BlockLine<'_>>::new();
    if let Some(ref eyebrow) = spec.eyebrow
        && !eyebrow.is_empty()
    {
        lines.push(BlockLine {
            text: eyebrow,
            role: FontRole::UiSans,
            scale_mul: 0.62,
            mode: GlyphInkMode::SpatialAnchor,
            color_hdr: [
                spec.accent_rgb[0].max(0.55),
                spec.accent_rgb[1].max(0.75),
                spec.accent_rgb[2].max(0.95),
                0.92 * op,
            ],
            halo_color: DEFAULT_OBSIDIAN_HALO,
            reveal_cursor: 2.0,
            z_pull_m: 0.04,
            use_scalar_sweep: false,
        });
    }

    for rem in &spec.remembered_lines {
        if rem.is_empty() {
            continue;
        }
        lines.push(BlockLine {
            text: rem,
            role: FontRole::UiSans,
            scale_mul: 0.65,
            mode: GlyphInkMode::RememberedMuted,
            // Mode 3 multiplies alpha by 0.40 in msdf_text.wgsl -> 0.88 * 0.40 = 35% muted context
            color_hdr: [0.80, 0.86, 0.94, 0.88 * op],
            halo_color: [0.002, 0.003, 0.006, 0.70 * op],
            reveal_cursor: 2.0,
            z_pull_m: 0.036,
            use_scalar_sweep: false,
        });
    }

    if let Some(ref heading) = spec.active_heading
        && !heading.is_empty()
    {
        lines.push(BlockLine {
            text: heading,
            role: FontRole::UiSans,
            scale_mul: 0.82,
            mode: GlyphInkMode::DwellFocusedHeading,
            color_hdr: [1.04, 1.05, 1.08, 0.98 * op],
            halo_color: DEFAULT_OBSIDIAN_HALO,
            reveal_cursor: 2.0,
            z_pull_m: 0.042,
            use_scalar_sweep: false,
        });
    }

    if !spec.spanish_text.is_empty() {
        match spec.phase {
            BilingualPhase::SpanishActive { es_reveal } => {
                lines.push(BlockLine {
                    text: &spec.spanish_text,
                    role: FontRole::DisplayScript,
                    scale_mul: 1.0,
                    mode: GlyphInkMode::LiveInkActive,
                    color_hdr: [1.08, 1.05, 1.00, 1.0 * op],
                    halo_color: DEFAULT_OBSIDIAN_HALO,
                    reveal_cursor: es_reveal.clamp(0.0, 1.0),
                    z_pull_m: 0.048,
                    use_scalar_sweep: true,
                });
            }
            BilingualPhase::TranslationOverGhost { .. } | BilingualPhase::Complete => {
                // Mode 2 multiplies ink_alpha by 0.72 in msdf_text.wgsl; dividing 0.16 / 0.72
                // yields exact 16% visual opacity for ghosted Spanish calligraphy!
                let ghost_alpha = (0.16 / 0.72) * op;
                lines.push(BlockLine {
                    text: &spec.spanish_text,
                    role: FontRole::DisplayScript,
                    scale_mul: 0.92,
                    mode: GlyphInkMode::SpanishGhostRibbon,
                    color_hdr: [0.78, 0.85, 0.98, ghost_alpha],
                    halo_color: [0.002, 0.003, 0.006, 0.35 * op],
                    reveal_cursor: 2.0,
                    z_pull_m: 0.035,
                    use_scalar_sweep: false,
                });
            }
        }
    }

    if let Some(ref trans) = spec.translation_text
        && !trans.is_empty()
    {
        match spec.phase {
            BilingualPhase::SpanishActive { .. } => {}
            BilingualPhase::TranslationOverGhost { trans_reveal } => {
                lines.push(BlockLine {
                    text: trans,
                    role: FontRole::DisplayScript,
                    scale_mul: 1.0,
                    mode: GlyphInkMode::LiveInkActive,
                    color_hdr: [1.08, 1.08, 1.12, 1.0 * op],
                    halo_color: DEFAULT_OBSIDIAN_HALO,
                    reveal_cursor: trans_reveal.clamp(0.0, 1.0),
                    z_pull_m: 0.052,
                    use_scalar_sweep: true,
                });
            }
            BilingualPhase::Complete => {
                lines.push(BlockLine {
                    text: trans,
                    role: FontRole::DisplayScript,
                    scale_mul: 1.0,
                    mode: GlyphInkMode::SpatialAnchor,
                    color_hdr: [1.06, 1.07, 1.10, 0.99 * op],
                    halo_color: DEFAULT_OBSIDIAN_HALO,
                    reveal_cursor: 2.0,
                    z_pull_m: 0.048,
                    use_scalar_sweep: false,
                });
            }
        }
    }

    if let Some(ref footer) = spec.footer_text
        && !footer.is_empty()
    {
        lines.push(BlockLine {
            text: footer,
            role: FontRole::UiSans,
            scale_mul: 0.62,
            mode: GlyphInkMode::SpatialAnchor,
            color_hdr: [0.88, 0.93, 1.00, 0.85 * op],
            halo_color: DEFAULT_OBSIDIAN_HALO,
            reveal_cursor: 2.0,
            z_pull_m: 0.038,
            use_scalar_sweep: false,
        });
    }

    if lines.is_empty() {
        return None;
    }

    let mut shaped_blocks = Vec::with_capacity(lines.len());
    let mut max_block_w_m = 0.0f32;
    let base_line_gap_m = 0.24 * main_world_h;
    // Give DisplayScript >= +0.30 em vertical clearance above and below so tall cursive
    // ascenders/accents and deep descenders never collide with adjacent UiSans lines.
    let display_script_extra_gap_m = 0.36 * main_world_h;

    for line in &lines {
        let line_h_m = main_world_h * line.scale_mul;
        let max_em = spec.max_line_width_em / line.scale_mul.max(0.35);
        let mut shaped = atlas.shape_text(line.role, line.text, Some(max_em));
        if line.role == FontRole::DisplayScript && shaped.lines.len() > 1 {
            let extra_row_em = 0.28;
            for g in &mut shaped.glyphs {
                if g.line_index > 0 {
                    g.y_em -= (g.line_index as f32) * extra_row_em;
                }
            }
            let extra_total_em = (shaped.lines.len() - 1) as f32 * extra_row_em;
            shaped.total_height_em += extra_total_em;
            shaped.line_height_em += extra_row_em;
        }
        let h_m = shaped.total_height_em * line_h_m;
        let w_m = shaped.max_width_em * line_h_m;
        max_block_w_m = max_block_w_m.max(w_m);
        shaped_blocks.push((line, shaped, line_h_m, h_m));
    }

    let gap_after = |idx: usize| -> f32 {
        if idx + 1 >= shaped_blocks.len() {
            return 0.0;
        }
        let touches_script = shaped_blocks[idx].0.role == FontRole::DisplayScript
            || shaped_blocks[idx + 1].0.role == FontRole::DisplayScript;
        if touches_script {
            base_line_gap_m + display_script_extra_gap_m
        } else {
            base_line_gap_m
        }
    };

    let mut total_block_h_m = 0.0f32;
    for (i, (_, _, _, h_m)) in shaped_blocks.iter().enumerate() {
        total_block_h_m += *h_m + gap_after(i);
    }

    // Measure exact visual bounds across all card lines (including actual calligraphic swashes,
    // descenders, MSDF halo overhang, and explicit bottom-line descender clearance).
    let top_y_m = 0.5 * total_block_h_m;
    let mut sim_top_m = top_y_m;
    let mut local_min_y_m = f32::INFINITY;
    let mut local_max_y_m = f32::NEG_INFINITY;
    let mut local_max_abs_x_m = 0.5 * max_block_w_m;
    for (i, (_, shaped, line_h_m, h_m)) in shaped_blocks.iter().enumerate() {
        let line_y_offset_m = sim_top_m - shaped.ascent_em * *line_h_m;
        let (gx0, gx1, gy0, gy1) = shaped_visual_bounds_em(shaped);
        let halo_pad_m = 0.14 * *line_h_m;
        local_max_abs_x_m = local_max_abs_x_m
            .max((gx0 * *line_h_m).abs().max((gx1 * *line_h_m).abs()) + halo_pad_m);
        local_max_y_m = local_max_y_m.max(line_y_offset_m + gy1 * *line_h_m + halo_pad_m);
        local_min_y_m = local_min_y_m.min(line_y_offset_m + gy0 * *line_h_m - halo_pad_m);
        sim_top_m -= *h_m + gap_after(i);
    }
    let bottom_descender_pad_m = match shaped_blocks.last() {
        Some((line, _, line_h_m, _)) if line.role == FontRole::DisplayScript => 0.48 * *line_h_m,
        Some((_, _, line_h_m, _)) => 0.24 * *line_h_m,
        None => 0.0,
    };
    local_min_y_m -= bottom_descender_pad_m;
    let block_center_y_m = 0.5 * (local_max_y_m + local_min_y_m);
    let block_half_h_m = (0.5 * (local_max_y_m - local_min_y_m)).max(0.5 * total_block_h_m);
    max_block_w_m = max_block_w_m.max(2.0 * local_max_abs_x_m);

    // Steer the narration block onto a comfortable offset spatial vergence shelf (default
    // lowered -17.5/-19.5 deg or lateral flanks), keeping generous distance from hero rims
    // while strictly clamping inside safe viewport bounds.
    let fwd = camera.forward.normalize_or(Vec3::NEG_Z);
    let right = camera.right.normalize_or(Vec3::X);
    let up = camera.up.normalize_or(Vec3::Y);
    let anchor_az = anchor_dir.dot(right).atan2(anchor_dir.dot(fwd));
    let anchor_el = anchor_dir.dot(up).clamp(-0.999, 0.999).asin();

    let half_fov_y = (0.5 * camera.fov_y_rad).clamp(0.18, 1.35);
    let half_fov_x = (half_fov_y.tan() * camera.aspect.max(0.6)).atan();
    let pad = 0.80_f32.to_radians();
    let edge_margin = 3.5_f32.to_radians();
    let safe_half_fov_x = (half_fov_x - edge_margin).max(0.05);
    let safe_half_fov_y = (half_fov_y - edge_margin).max(0.05);
    let safe_tan_x = safe_half_fov_x.tan();
    let safe_tan_y = safe_half_fov_y.tan();
    let arc_radius = if spec.arc_curve { vergence_dist } else { 0.0 };
    let bow_z = if arc_radius > 0.05 {
        let half_theta = ((0.5 * max_block_w_m) / arc_radius).min(1.0);
        arc_radius * (1.0 - half_theta.cos())
    } else {
        0.0
    };
    let half_w_safe = 0.5 * max_block_w_m + 0.45 * main_world_h;
    let half_h_safe =
        local_max_y_m.abs().max(local_min_y_m.abs()) + 0.45 * main_world_h;

    let core_guard = AngularRect {
        az_center: anchor_az,
        el_center: anchor_el,
        half_az: 12.5_f32.to_radians(),
        half_el: 12.0_f32.to_radians(),
    };

    let shelf_candidates_deg: &[(f32, f32)] = &[
        (0.0, -20.0),
        (13.5, -20.0),
        (-13.5, -20.0),
        (18.5, -18.5),
        (-18.5, -18.5),
        (29.5, -5.0),
        (-29.5, -5.0),
        (29.5, 4.0),
        (-29.5, 4.0),
        (0.0, 20.0),
        (14.5, 20.0),
        (-14.5, 20.0),
    ];

    struct ShelfChoice {
        score: (f32, f32, f32, usize),
        az: f32,
        el: f32,
        rect: AngularRect,
    }

    let mut best_choice: Option<ShelfChoice> = None;
    for (pri, &(off_az_deg, off_el_deg)) in shelf_candidates_deg.iter().enumerate() {
        let mut az = (anchor_az + off_az_deg.to_radians()).clamp(-safe_half_fov_x, safe_half_fov_x);
        let mut el = (anchor_el + off_el_deg.to_radians()).clamp(-safe_half_fov_y, safe_half_fov_y);
        for _ in 0..2 {
            let cz = (vergence_dist * el.cos() * az.cos()).max(0.25 * vergence_dist);
            let cz_wing = (cz - bow_z).max(0.25 * vergence_dist);
            let max_cx = (cz_wing * safe_tan_x - half_w_safe).max(0.0);
            let max_cy = (cz_wing * safe_tan_y - half_h_safe).max(0.0);
            let cy = (vergence_dist * el.sin()).clamp(-max_cy, max_cy);
            el = (cy / vergence_dist).clamp(-0.98, 0.98).asin();
            let denom_x = (vergence_dist * el.cos()).max(0.25);
            let cx = (denom_x * az.sin()).clamp(-max_cx, max_cx);
            az = (cx / denom_x).clamp(-0.98, 0.98).asin();
        }
        let cz_eff = (vergence_dist * el.cos() * az.cos() - bow_z).max(0.35 * vergence_dist);
        let el_shift_center = (block_center_y_m / cz_eff).atan();
        let block_half_el = (block_half_h_m / cz_eff).atan() + pad;
        let half_az = (0.5 * max_block_w_m / cz_eff).atan() + pad;

        let rect = AngularRect {
            az_center: az,
            el_center: (el + el_shift_center).clamp(-1.54, 1.54),
            half_az,
            half_el: block_half_el,
        };
        let ext_overlap: f32 = external_blockers.iter().map(|b| rect.overlap_area(b)).sum();
        let core_overlap = rect.overlap_area(&core_guard);
        let dot_overlap: f32 = dot_guards.iter().map(|d| rect.overlap_area(d)).sum();
        let score = (ext_overlap, core_overlap, dot_overlap, pri);
        let better = match &best_choice {
            None => true,
            Some(prev) => {
                let b_score = prev.score;
                (score.0 + 1e-7 < b_score.0)
                    || ((score.0 - b_score.0).abs() <= 1e-7 && score.1 + 1e-7 < b_score.1)
                    || ((score.0 - b_score.0).abs() <= 1e-7
                        && (score.1 - b_score.1).abs() <= 1e-7
                        && score.2 + 1e-7 < b_score.2)
                    || ((score.0 - b_score.0).abs() <= 1e-7
                        && (score.1 - b_score.1).abs() <= 1e-7
                        && (score.2 - b_score.2).abs() <= 1e-7
                        && score.3 < b_score.3)
            }
        };
        if better {
            best_choice = Some(ShelfChoice {
                score,
                az,
                el,
                rect,
            });
        }
    }

    let ShelfChoice {
        az: card_az,
        el: card_el,
        rect: mut chosen_rect,
        ..
    } = best_choice?;
    let el_cos = card_el.cos();
    let card_dir = (fwd * (el_cos * card_az.cos())
        + right * (el_cos * card_az.sin())
        + up * card_el.sin())
    .normalize();
    let card_center = camera.eye_center + card_dir * vergence_dist;
    let (card_right, card_up) = camera_level_basis(camera);

    let card_glyph_start = glyphs_out.len();
    let mut cursor_top_m = top_y_m;

    for (i, (line, shaped, line_h_m, h_m)) in shaped_blocks.iter().enumerate() {
        let baseline_top_offset = shaped.ascent_em * *line_h_m;
        let line_y_offset_m = cursor_top_m - baseline_top_offset;

        emit_shaped_text_quads(
            shaped,
            card_center,
            line_y_offset_m,
            camera.eye_center,
            card_right,
            card_up,
            *line_h_m,
            arc_radius,
            line.z_pull_m,
            0.20,
            line.color_hdr,
            line.halo_color,
            line.reveal_cursor,
            line.mode,
            !line.use_scalar_sweep,
            glyphs_out,
        );

        cursor_top_m -= *h_m + gap_after(i);
    }

    // Union `chosen_rect` with the exact projected bounds of all emitted card glyph quads
    // plus explicit bottom-line descender padding so no ambient label ever clips under descenders.
    let card_glyphs = &glyphs_out[card_glyph_start..];
    let bottom_descender_pad_rad = (bottom_descender_pad_m / vergence_dist.max(0.35)).atan();
    if let Some((_, glyph_ang_rect)) = project_glyphs_screen_bounds(camera, card_glyphs) {
        let g_az0 = glyph_ang_rect.az_center - glyph_ang_rect.half_az - pad;
        let g_az1 = glyph_ang_rect.az_center + glyph_ang_rect.half_az + pad;
        let g_el0 = glyph_ang_rect.el_center - glyph_ang_rect.half_el - pad - bottom_descender_pad_rad;
        let g_el1 = glyph_ang_rect.el_center + glyph_ang_rect.half_el + pad;

        let u_az0 = g_az0.min(chosen_rect.az_center - chosen_rect.half_az);
        let u_az1 = g_az1.max(chosen_rect.az_center + chosen_rect.half_az);
        let u_el0 = g_el0.min(chosen_rect.el_center - chosen_rect.half_el);
        let u_el1 = g_el1.max(chosen_rect.el_center + chosen_rect.half_el);
        chosen_rect = AngularRect {
            az_center: 0.5 * (u_az0 + u_az1),
            el_center: 0.5 * (u_el0 + u_el1),
            half_az: 0.5 * (u_az1 - u_az0),
            half_el: 0.5 * (u_el1 - u_el0),
        };
    }

    // Compute the glowing leader filament exit point on the outer perimeter of `chosen_rect`
    // (enclosing every emitted card glyph in camera-view perspective space) before mapping to
    // `vergence_dist`, so tethers never cross card headers or glyphs across depth parallax.
    let tether_start = compute_tether_start_on_screen_perimeter(
        camera,
        card_glyphs,
        chosen_rect,
        card_center,
        spec.anchor_world_pos,
        vergence_dist,
        pad.tan().clamp(0.010, 0.03),
        bottom_descender_pad_rad.tan().clamp(0.0, 0.04),
    );
    tethers_out.push(TetherInstance {
        p0_width: [tether_start.x, tether_start.y, tether_start.z, 0.0018],
        p1_alpha: [
            spec.anchor_world_pos.x,
            spec.anchor_world_pos.y,
            spec.anchor_world_pos.z,
            0.80 * op,
        ],
        color_hdr: [
            spec.accent_rgb[0] * 1.15,
            spec.accent_rgb[1] * 1.15,
            spec.accent_rgb[2] * 1.15,
            0.22,
        ],
    });

    Some((chosen_rect, vergence_dist))
}

fn shaped_visual_bounds_em(shaped: &crate::atlas::ShapedText) -> (f32, f32, f32, f32) {
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut any = false;

    for sg in &shaped.glyphs {
        if !sg.metrics.has_outline {
            continue;
        }
        let line_w_em = shaped.lines[sg.line_index].width_em;
        let line_offset_x_em = sg.x_em - 0.5 * line_w_em;
        let [pl, pb, pr, pt] = sg.metrics.plane_bounds;
        min_x = min_x.min(line_offset_x_em + pl);
        max_x = max_x.max(line_offset_x_em + pr);
        min_y = min_y.min(sg.y_em + pb);
        max_y = max_y.max(sg.y_em + pt);
        any = true;
    }

    if any {
        (min_x, max_x, min_y, max_y)
    } else {
        let descent = shaped.ascent_em - shaped.total_height_em;
        (
            -0.5 * shaped.max_width_em,
            0.5 * shaped.max_width_em,
            descent,
            shaped.ascent_em,
        )
    }
}

fn project_glyphs_screen_bounds(
    camera: &CameraBasis,
    glyphs: &[GlyphInstance],
) -> Option<([f32; 4], AngularRect)> {
    let fwd = camera.forward.normalize_or(Vec3::NEG_Z);
    let right = camera.right.normalize_or(Vec3::X);
    let up = camera.up.normalize_or(Vec3::Y);

    let mut u_min = f32::INFINITY;
    let mut u_max = f32::NEG_INFINITY;
    let mut v_min = f32::INFINITY;
    let mut v_max = f32::NEG_INFINITY;
    let mut az_min = f32::INFINITY;
    let mut az_max = f32::NEG_INFINITY;
    let mut el_min = f32::INFINITY;
    let mut el_max = f32::NEG_INFINITY;
    let mut any = false;

    for g in glyphs {
        let center = Vec3::new(g.pos_center[0], g.pos_center[1], g.pos_center[2]);
        let rvec = Vec3::new(g.right_vec[0], g.right_vec[1], g.right_vec[2]);
        let uvec = Vec3::new(g.up_vec[0], g.up_vec[1], g.up_vec[2]);
        let to_eye = camera.eye_center - center;
        let dist = to_eye.length().max(1e-4);
        let pull = g.pos_center[3].max(0.0).min(dist * 0.15);
        let pull_offset = (to_eye / dist) * pull;

        for (sx, sy) in [(-1.0_f32, -1.0_f32), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
            let corner_rel = (center + pull_offset + rvec * sx + uvec * sy) - camera.eye_center;
            let fc = corner_rel.dot(fwd);
            if fc <= 0.05 {
                continue;
            }
            let rc = corner_rel.dot(right);
            let uc = corner_rel.dot(up);
            let u = rc / fc;
            let v = uc / fc;
            let dir = corner_rel.normalize();
            let az = dir.dot(right).atan2(dir.dot(fwd));
            let el = dir.dot(up).clamp(-0.999, 0.999).asin();

            u_min = u_min.min(u);
            u_max = u_max.max(u);
            v_min = v_min.min(v);
            v_max = v_max.max(v);
            az_min = az_min.min(az);
            az_max = az_max.max(az);
            el_min = el_min.min(el);
            el_max = el_max.max(el);
            any = true;
        }
    }

    if !any {
        return None;
    }

    let ang_rect = AngularRect {
        az_center: 0.5 * (az_min + az_max),
        el_center: 0.5 * (el_min + el_max),
        half_az: 0.5 * (az_max - az_min),
        half_el: 0.5 * (el_max - el_min),
    };
    Some(([u_min, u_max, v_min, v_max], ang_rect))
}

#[allow(clippy::too_many_arguments)]
fn compute_tether_start_on_screen_perimeter(
    camera: &CameraBasis,
    glyphs: &[GlyphInstance],
    outer_rect: AngularRect,
    fallback_center: Vec3,
    anchor_world_pos: Vec3,
    tether_depth_m: f32,
    pad_uv: f32,
    bottom_extra_uv: f32,
) -> Vec3 {
    let fwd = camera.forward.normalize_or(Vec3::NEG_Z);
    let right = camera.right.normalize_or(Vec3::X);
    let up = camera.up.normalize_or(Vec3::Y);

    let anchor_rel = anchor_world_pos - camera.eye_center;
    let anchor_fwd = anchor_rel.dot(fwd);

    if anchor_fwd > 0.05 {
        let mut u_min = f32::INFINITY;
        let mut u_max = f32::NEG_INFINITY;
        let mut v_min = f32::INFINITY;
        let mut v_max = f32::NEG_INFINITY;
        let mut have_bounds = false;

        if let Some(([gu0, gu1, gv0, gv1], _)) = project_glyphs_screen_bounds(camera, glyphs) {
            u_min = gu0 - pad_uv;
            u_max = gu1 + pad_uv;
            v_min = gv0 - pad_uv - bottom_extra_uv;
            v_max = gv1 + pad_uv;
            have_bounds = true;
        }

        // Union with `outer_rect` in pinhole `(u, v)` space when `bottom_extra_uv > 0` (card
        // blocks use the full padded `chosen_rect`, keeping leaders cleanly on its outer border).
        if bottom_extra_uv > 0.002 {
            for saz in [-1.0_f32, 1.0] {
                for sel in [-1.0_f32, 1.0] {
                    let az = (outer_rect.az_center + saz * outer_rect.half_az).clamp(-1.45, 1.45);
                    let el = (outer_rect.el_center + sel * outer_rect.half_el).clamp(-1.45, 1.45);
                    let el_cos = el.cos();
                    let dir = fwd * (el_cos * az.cos())
                        + right * (el_cos * az.sin())
                        + up * el.sin();
                    let df = dir.dot(fwd);
                    if df > 0.05 {
                        let u = dir.dot(right) / df;
                        let v = dir.dot(up) / df;
                        u_min = u_min.min(u);
                        u_max = u_max.max(u);
                        v_min = v_min.min(v);
                        v_max = v_max.max(v);
                        have_bounds = true;
                    }
                }
            }
        }

        if have_bounds && u_max > u_min + 1e-5 && v_max > v_min + 1e-5 {
            let uc = 0.5 * (u_min + u_max);
            let vc = 0.5 * (v_min + v_max);
            let hu = 0.5 * (u_max - u_min);
            let hv = 0.5 * (v_max - v_min);
            let ua = anchor_rel.dot(right) / anchor_fwd;
            let va = anchor_rel.dot(up) / anchor_fwd;
            let du = ua - uc;
            let dv = va - vc;
            let edge_t = (hu / du.abs().max(1e-5))
                .min(hv / dv.abs().max(1e-5))
                .min(1.0);
            let u_exit = uc + du * edge_t;
            let v_exit = vc + dv * edge_t;
            let exit_dir = (fwd + right * u_exit + up * v_exit).normalize();
            return camera.eye_center + exit_dir * tether_depth_m;
        }
    }

    // Fallback when anchor sits behind the camera forward hemisphere.
    let (card_right, card_up) = camera_level_basis(camera);
    let to_anchor = anchor_world_pos - fallback_center;
    let dx = to_anchor.dot(card_right);
    let dy = to_anchor.dot(card_up);
    let edge_t = (0.05 / dx.abs().max(1e-4))
        .min(0.05 / dy.abs().max(1e-4))
        .min(1.0);
    fallback_center + card_right * (dx * edge_t) + card_up * (dy * edge_t)
}

#[inline]
fn camera_level_basis(camera: &CameraBasis) -> (Vec3, Vec3) {
    let fwd = camera.forward.normalize_or(Vec3::NEG_Z);
    // Project camera.up perpendicular to camera view direction so text billboards stay
    // locked to 0 deg viewer-horizon roll regardless of camera pitch/roll or off-axis shelf.
    let mut up = (camera.up - fwd * camera.up.dot(fwd)).normalize_or(Vec3::Y);
    let right = fwd.cross(up).normalize_or(camera.right.normalize_or(Vec3::X));
    up = right.cross(fwd).normalize_or(up);
    (right, up)
}

#[allow(clippy::too_many_arguments)]
fn emit_shaped_text_quads(
    shaped: &crate::atlas::ShapedText,
    arc_anchor_world: Vec3,
    line_y_offset_m: f32,
    eye_center: Vec3,
    basis_right: Vec3,
    basis_up: Vec3,
    em_world_h: f32,
    arc_radius_m: f32,
    z_pull_m: f32,
    halo_sdf_width: f32,
    color_hdr: [f32; 4],
    halo_color: [f32; 4],
    reveal_cursor: f32,
    mode: GlyphInkMode,
    static_no_sweep: bool,
    out: &mut Vec<GlyphInstance>,
) {
    let total_scalars = shaped.total_scalars.max(1) as f32;
    let to_eye_center = (eye_center - arc_anchor_world).normalize_or(Vec3::Z);

    for sg in &shaped.glyphs {
        if !sg.metrics.has_outline {
            continue;
        }
        let line_w_em = shaped.lines[sg.line_index].width_em;
        let line_offset_x_em = sg.x_em - 0.5 * line_w_em;

        let [pl, pb, pr, pt] = sg.metrics.plane_bounds;
        let cx_world = (line_offset_x_em + 0.5 * (pl + pr)) * em_world_h;
        let cy_world = line_y_offset_m + (sg.y_em + 0.5 * (pb + pt)) * em_world_h;
        let hw = 0.5 * (pr - pl) * em_world_h;
        let hh = 0.5 * (pt - pb) * em_world_h;
        if hw <= 1e-6 || hh <= 1e-6 {
            continue;
        }

        let (pos_w, g_right) = if arc_radius_m > 0.05 {
            let theta = cx_world / arc_radius_m;
            let (sin_t, cos_t) = theta.sin_cos();
            let card_normal = basis_right.cross(basis_up).normalize_or(to_eye_center);
            let pos = arc_anchor_world
                + basis_right * (arc_radius_m * sin_t)
                + card_normal * (arc_radius_m * (1.0 - cos_t))
                + basis_up * cy_world;
            let tangent_r = (basis_right * cos_t + card_normal * sin_t).normalize();
            (pos, tangent_r)
        } else {
            (
                arc_anchor_world + basis_right * cx_world + basis_up * cy_world,
                basis_right,
            )
        };

        // Map glyph scalar index into [0.02, 0.98] so reveal_cursor in [0.0..1.0] covers
        // the whole phrase cleanly with smooth leading & trailing brush stroke edges.
        let t0 = 0.02 + 0.96 * (sg.scalar_index as f32 / total_scalars);
        let t1 = 0.02 + 0.96 * ((sg.scalar_index + 1) as f32 / total_scalars);
        let eff_cursor = if static_no_sweep { 2.0 } else { reveal_cursor };

        out.push(GlyphInstance {
            pos_center: [pos_w.x, pos_w.y, pos_w.z, z_pull_m],
            right_vec: [
                g_right.x * hw,
                g_right.y * hw,
                g_right.z * hw,
                sg.metrics.px_range,
            ],
            up_vec: [
                basis_up.x * hh,
                basis_up.y * hh,
                basis_up.z * hh,
                halo_sdf_width,
            ],
            uv_rect: sg.metrics.uv_rect,
            color_hdr,
            halo_color,
            ink_params: [t0, t1, eff_cursor, mode as u8 as f32],
        });
    }
}
