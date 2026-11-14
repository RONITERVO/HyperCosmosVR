use cosmos_manifest::{
    rgb_to_hsl, Locale, MetricMode, PhraseTriple, UnitKind, UniverseCatalog, ZoneRecord,
};
use cosmos_spatial::{hsl_to_prism_local, prism_local_to_hsl, EntityKind, VisibleEntity};
use glam::DVec3;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::memory::{MemoryRole, MemorySegmentInput};

/// Semantic segment classification matching `Spanish-Quick-Apps` learning narration pipeline.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum SegmentRole {
    EvidenceClass,
    RegionHeading,
    Item,
    SharedContext,
    Explanation,
}

impl SegmentRole {
    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EvidenceClass => "evidence-class",
            Self::RegionHeading => "region-heading",
            Self::Item => "item",
            Self::SharedContext => "shared-context",
            Self::Explanation => "explanation",
        }
    }

    #[inline]
    pub const fn memory_role(&self) -> Option<MemoryRole> {
        match self {
            Self::RegionHeading => Some(MemoryRole::RegionHeading),
            Self::EvidenceClass | Self::SharedContext => Some(MemoryRole::SharedContext),
            Self::Item | Self::Explanation => None,
        }
    }
}

/// Controls whether App `12..15` speak their packed `evidence_class` certainty badge by default.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum EvidencePolicy {
    /// Emits `EvidenceClass` for all `11..25` zones whose packed `evidence_class` triple is present.
    #[default]
    AllPresent,
    /// Honors the per-zone `speak_evidence_default` flag (`1` on `11, 16..25`, `0` on original `12..15`).
    RespectSpeakDefault,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResolverConfig {
    pub evidence_policy: EvidencePolicy,
}

/// Paired `es-ES`, `en-US`, and `fi-FI` audio asset indices into `UniverseCatalog::audio_assets()`.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AudioAssetIds {
    pub es: Option<u32>,
    pub en: Option<u32>,
    pub fi: Option<u32>,
    /// Additional sequential asset triples for multi-clip segments (e.g. App `04 sun` canonical metric).
    pub extra_tokens: Vec<PhraseTripleIds>,
}

/// Single phrase clip triple `(es, en, fi)` without nested recursion.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct PhraseTripleIds {
    pub es: Option<u32>,
    pub en: Option<u32>,
    pub fi: Option<u32>,
}

impl From<PhraseTriple> for PhraseTripleIds {
    #[inline]
    fn from(t: PhraseTriple) -> Self {
        Self {
            es: t.asset_for(Locale::EsEs),
            en: t.asset_for(Locale::EnUs),
            fi: t.asset_for(Locale::FiFi),
        }
    }
}

impl AudioAssetIds {
    pub fn from_triple(t: PhraseTriple) -> Self {
        Self {
            es: t.asset_for(Locale::EsEs),
            en: t.asset_for(Locale::EnUs),
            fi: t.asset_for(Locale::FiFi),
            extra_tokens: Vec::new(),
        }
    }

    pub fn from_triples(triples: &[PhraseTriple]) -> Self {
        let mut iter = triples.iter().copied();
        let Some(first) = iter.next() else {
            return Self::default();
        };
        let mut out = Self::from_triple(first);
        out.extra_tokens = iter.map(PhraseTripleIds::from).collect();
        out
    }

    /// Returns the flat ordered list of clip IDs required to speak this segment in `locale`.
    pub fn playlist_for(&self, locale: Locale) -> Vec<u32> {
        let pick = |es: Option<u32>, en: Option<u32>, fi: Option<u32>| match locale {
            Locale::EsEs => es,
            Locale::EnUs => en,
            Locale::FiFi => fi,
        };
        let mut ids = Vec::with_capacity(1 + self.extra_tokens.len());
        if let Some(id) = pick(self.es, self.en, self.fi) {
            ids.push(id);
        }
        for extra in &self.extra_tokens {
            if let Some(id) = pick(extra.es, extra.en, extra.fi) {
                ids.push(id);
            }
        }
        ids
    }
}

/// Documentary visual stage cue classification for one narration segment.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum VisualCueTag {
    ChromaticSample,
    HarmonicFilament,
    EvidenceBadge,
    ZoneOverview,
    FeatureSpotlight,
    MetricScale,
    #[default]
    NarrativeDetail,
}

/// Extracted perceptual HSL + physical material preset hint for App `01` Colors.
#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ChromaticVisualHint {
    pub rgb: [u8; 3],
    /// Hue angle in `[0.0, 360.0)` degrees.
    pub hue_deg: f32,
    /// Normalized saturation in `[0.0, 1.0]`.
    pub saturation: f32,
    /// Normalized lightness in `[0.0, 1.0]`.
    pub lightness: f32,
    /// Material preset matching App `01` physical spheres (`0=gold metal`, `1=thin-film`, `2=matte`, `3=plasma`).
    pub material_class: u8,
}

/// Extracted synthesis & visual wave hint for App `02` Music (`[nota, octava N, timbre]`).
#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct HarmonicVisualHint {
    pub midi: u8,
    pub octave: i32,
    pub timbre_index: u8,
    pub freq_hz: f32,
    pub amp_hint: f32,
}

/// Rich documentary shader & choreography metadata attached per [`NarrationSegment`].
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SegmentVisualHint {
    pub experience_id: u8,
    /// Active zone index (`0..`), or `u16::MAX` for non-zonal App `01`/`02` targets.
    pub zone_index: u16,
    /// Focused feature index inside the zone (`None` on macro/overview or non-zonal apps).
    pub feature_index: Option<u16>,
    /// `ZoneRecord::scene_index` (`-1` when not backed by a catalog zone).
    pub scene_index: i32,
    pub cue_tag: VisualCueTag,
    /// Normalized domain/concept cue tags extracted from zone id, feature text, and role.
    pub visual_tags: Vec<String>,
    pub chromatic: Option<ChromaticVisualHint>,
    pub harmonic: Option<HarmonicVisualHint>,
    /// Normalized documentary spotlight emphasis `[0.0 ..= 1.0]`.
    pub emphasis_boost: f32,
    /// Shared GPU telemetry `[f32; 4]` matching `DocumentaryFocusState::telemetry`:
    /// - App `01`: `[hue_deg, saturation, lightness, material_class]`
    /// - App `02`: `[midi, timbre_index, freq_hz, amp_hint]`
    /// - Apps `03..25`: `[range_start, range_end, phys_low, phys_high]`
    pub telemetry: [f32; 4],
}

/// Snapshot aligning 1-to-1 with `cosmos_render::universe_scene::DocumentaryFocusState`
/// so `cosmos_app` can drive documentary shader focus without coupling crates.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct DocumentaryCueSnapshot {
    pub active_zone_index: u16,
    pub active_feature_index: u16,
    pub scene_index: i32,
    pub narration_cue_progress: f32,
    pub narration_active_weight: f32,
    pub zone_accent_rgb: [f32; 3],
    pub telemetry: [f32; 4],
    pub emphasis_boost: f32,
    pub cue_tags: Vec<String>,
}

/// One structured display + bilingual audio segment inside a [`NarrationPlan`].
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NarrationSegment {
    pub id: String,
    pub role: SegmentRole,
    pub memory_role: Option<MemoryRole>,
    pub display_text: String,
    pub spoken_es_text: String,
    pub spoken_en_text: String,
    pub spoken_fi_text: String,
    pub assets: AudioAssetIds,
    pub visual_hint: SegmentVisualHint,
}

impl NarrationSegment {
    /// Returns localized spoken text (`EnUs` or `FiFi`; falls back to Spanish for `EsEs`).
    #[inline]
    pub fn spoken_text(&self, locale: Locale) -> &str {
        match locale {
            Locale::EsEs => &self.spoken_es_text,
            Locale::EnUs => &self.spoken_en_text,
            Locale::FiFi => &self.spoken_fi_text,
        }
    }

    /// Borrows this segment in the shape expected by [`crate::memory::SharedNarrationMemory`].
    #[inline]
    pub fn as_memory_input(&self, locale: Locale) -> MemorySegmentInput<'_> {
        MemorySegmentInput {
            id: &self.id,
            role: self.memory_role,
            display: &self.display_text,
            narration: &self.spoken_es_text,
            translation: match locale {
                Locale::EnUs => &self.spoken_en_text,
                Locale::FiFi => &self.spoken_fi_text,
                Locale::EsEs => &self.spoken_es_text,
            },
        }
    }
}

/// Immediate synthesis event emitted when touching or hovering an App `02` cymatic filament.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MusicTriggerEvent {
    pub midi: u8,
    pub octave: i32,
    pub timbre_index: u8,
    pub freq_hz: f64,
    pub hue_deg: f32,
    pub waveform: u8,
    pub detune_cents: f32,
    pub filter_hz: f32,
    pub attack_sec: f32,
    pub release_sec: f32,
}

/// Complete resolved target descriptor consumed by 3D typography HUD and `cosmos_audio` sequencer.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NarrationPlan {
    pub experience_id: u8,
    pub zone_index: u16,
    pub feature_index: Option<u16>,
    pub scene_index: i32,
    pub region_id: String,
    pub accent_rgb: [u8; 3],
    pub zone_accent_linear_rgb: [f32; 3],
    pub telemetry: [f32; 4],
    pub is_macro_overview: bool,
    pub footer_banner: Option<String>,
    pub music_trigger: Option<MusicTriggerEvent>,
    pub segments: Vec<NarrationSegment>,
}

impl NarrationPlan {
    /// Evaluates segment skip flags against `memory` for `locale` and begins a tracking session.
    pub fn begin_memory(
        &self,
        memory: &mut crate::memory::SharedNarrationMemory,
        locale: Locale,
    ) -> crate::memory::MemoryPlanState {
        let inputs: Vec<_> = self
            .segments
            .iter()
            .map(|s| s.as_memory_input(locale))
            .collect();
        memory.begin_plan(&self.region_id, &inputs, locale)
    }

    /// Exports GPU documentary focus telemetry & visual cue tags for the currently speaking segment.
    pub fn documentary_snapshot(
        &self,
        active_segment_idx: Option<usize>,
        cue_progress: f32,
        active_weight: f32,
    ) -> DocumentaryCueSnapshot {
        let seg_hint = active_segment_idx
            .and_then(|idx| self.segments.get(idx))
            .map(|seg| &seg.visual_hint)
            .or_else(|| self.segments.first().map(|seg| &seg.visual_hint));

        let (telemetry, emphasis_boost, cue_tags) = if let Some(h) = seg_hint {
            (h.telemetry, h.emphasis_boost, h.visual_tags.clone())
        } else {
            (self.telemetry, 0.5, Vec::new())
        };

        DocumentaryCueSnapshot {
            active_zone_index: self.zone_index,
            active_feature_index: self.feature_index.unwrap_or(u16::MAX),
            scene_index: self.scene_index,
            narration_cue_progress: cue_progress.clamp(0.0, 1.0),
            narration_active_weight: active_weight.clamp(0.0, 1.0),
            zone_accent_rgb: self.zone_accent_linear_rgb,
            telemetry,
            emphasis_boost,
            cue_tags,
        }
    }
}

/// Spatial target query resolved against [`UniverseCatalog`].
#[derive(Clone, Debug, PartialEq)]
pub enum TargetFocus {
    /// App `01` ray/hand point inside `SectorId::COLORS_PRISM` local coordinate frame.
    ChromaticLocal(DVec3),
    /// App `01` direct sRGB `[0..255]` sample.
    ChromaticRgb([u8; 3]),
    /// App `02` exact cymatic filament (`midi: 48..=95`, `timbre_idx: 0..=3`).
    MusicFilament { midi: u8, timbre_idx: u8 },
    /// App `02` continuous normalized prism/harp surface coordinate `(x_norm, y_norm)` in `[0..1]^2`.
    MusicNormalized { x_norm: f32, y_norm: f32 },
    /// App `03..25` zone/feature resolution (zoomed-out `macro_fact` vs focused feature).
    CatalogItem {
        experience_id: u8,
        zone_index: u16,
        feature_index: Option<u16>,
        is_macro_zoomed_out: bool,
    },
    /// App `03..25` fractional `(p_norm, cross_norm)` resolution along radial scale & cross lane.
    CatalogNormalized {
        experience_id: u8,
        p_norm: f32,
        cross_norm: f32,
        is_macro_zoomed_out: bool,
    },
    /// App `01` exact catalog color lexicon entry with representative sRGB tint.
    ChromaticKey {
        key_index: u16,
        rgb: [u8; 3],
    },
    /// Any experience (`01..=25`) exact lexicon entry by `app_key_entries(experience_id)` index.
    LexiconKey {
        experience_id: u8,
        key_index: u16,
    },
}

impl TargetFocus {
    /// Builds a [`TargetFocus`] directly from a frustum-projected [`VisibleEntity`].
    ///
    /// For App `01` / `02` continuous fields, pass `local_hit_in_sector` when ray-casting directly inside
    /// `COLORS_PRISM` or `MUSIC_HARP` for sub-node resolution.
    pub fn from_visible_entity(
        entity: &VisibleEntity,
        local_hit_in_sector: Option<DVec3>,
    ) -> Self {
        match (entity.experience_id, entity.kind) {
            (1, EntityKind::ChromaticSample) | (1, EntityKind::ExperienceAnchor) => {
                if let Some(local) = local_hit_in_sector {
                    Self::ChromaticLocal(local)
                } else {
                    let hue = (entity.feature_index as f64) * 30.0;
                    Self::ChromaticLocal(hsl_to_prism_local(hue, 85.0, 50.0))
                }
            }
            (2, EntityKind::MusicalFilament) => Self::MusicFilament {
                midi: 48_u8.saturating_add((entity.feature_index as u8).min(47)),
                timbre_idx: (entity.zone_index as u8).min(3),
            },
            (2, EntityKind::ExperienceAnchor) => Self::MusicNormalized {
                x_norm: 0.5,
                y_norm: 0.5,
            },
            _ => {
                let is_macro = matches!(
                    entity.kind,
                    EntityKind::ExperienceAnchor | EntityKind::ZoneShell
                );
                let z = if entity.zone_index == u16::MAX {
                    0
                } else {
                    entity.zone_index
                };
                let f = if is_macro || entity.feature_index == u16::MAX {
                    None
                } else {
                    Some(entity.feature_index)
                };
                Self::CatalogItem {
                    experience_id: entity.experience_id,
                    zone_index: z,
                    feature_index: f,
                    is_macro_zoomed_out: is_macro,
                }
            }
        }
    }
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ResolverError {
    #[error("experience {0} does not have zone index {1}")]
    MissingZone(u8, u16),
    #[error("experience {0} does not have lexicon key index {1}")]
    MissingKeyEntry(u8, u16),
    #[error("experience {0} has no catalog zones")]
    EmptyExperienceZones(u8),
    #[error("unknown experience id {0} (expected 1..=25)")]
    UnknownExperience(u8),
}

/// Resolves any of the 25 cosmic experiences against the baked `UniverseCatalog`.
#[derive(Clone, Debug, Default)]
pub struct TargetResolver {
    pub config: ResolverConfig,
}

impl TargetResolver {
    pub fn new(config: ResolverConfig) -> Self {
        Self { config }
    }

    pub fn resolve(
        &self,
        catalog: &UniverseCatalog,
        focus: &TargetFocus,
    ) -> Result<NarrationPlan, ResolverError> {
        match focus {
            TargetFocus::ChromaticLocal(local_pos) => {
                let (h, s, l) = prism_local_to_hsl(*local_pos);
                let rgb = hsl_to_srgb_u8(h, s, l);
                Ok(self.resolve_color(catalog, rgb))
            }
            TargetFocus::ChromaticRgb(rgb) => Ok(self.resolve_color(catalog, *rgb)),
            TargetFocus::MusicFilament { midi, timbre_idx } => {
                let note_idx = midi.saturating_sub(48).min(47);
                let x_norm = (note_idx as f32 + 0.5) / 48.0;
                let y_norm = ((*timbre_idx).min(3) as f32 + 0.5) / 4.0;
                Ok(self.resolve_music(catalog, x_norm, y_norm, *timbre_idx))
            }
            TargetFocus::MusicNormalized { x_norm, y_norm } => {
                let t_idx = ((*y_norm).clamp(0.0, 0.999_999) * 4.0).floor() as u8;
                Ok(self.resolve_music(catalog, *x_norm, *y_norm, t_idx))
            }
            TargetFocus::CatalogItem {
                experience_id,
                zone_index,
                feature_index,
                is_macro_zoomed_out,
            } => {
                if !(3..=25).contains(experience_id) {
                    return Err(ResolverError::UnknownExperience(*experience_id));
                }
                let zones = catalog.zones(*experience_id);
                let zone = zones
                    .get(*zone_index as usize)
                    .ok_or(ResolverError::MissingZone(*experience_id, *zone_index))?;
                Ok(self.resolve_zone(catalog, zone, *feature_index, *is_macro_zoomed_out))
            }
            TargetFocus::CatalogNormalized {
                experience_id,
                p_norm,
                cross_norm,
                is_macro_zoomed_out,
            } => {
                if !(3..=25).contains(experience_id) {
                    return Err(ResolverError::UnknownExperience(*experience_id));
                }
                let zone = catalog
                    .zone_at_progress(*experience_id, *p_norm)
                    .ok_or(ResolverError::EmptyExperienceZones(*experience_id))?;
                let feat_idx = if *is_macro_zoomed_out {
                    None
                } else {
                    catalog.feature_at(zone, *cross_norm).map(|(i, _)| i as u16)
                };
                Ok(self.resolve_zone(catalog, zone, feat_idx, *is_macro_zoomed_out))
            }
            TargetFocus::ChromaticKey { key_index, rgb } => {
                self.resolve_chromatic_key(catalog, *key_index, *rgb)
            }
            TargetFocus::LexiconKey {
                experience_id,
                key_index,
            } => self.resolve_lexicon_key(catalog, *experience_id, *key_index),
        }
    }

    fn resolve_color(&self, catalog: &UniverseCatalog, rgb: [u8; 3]) -> NarrationPlan {
        let solved = catalog.solve_color(rgb[0], rgb[1], rgb[2]);
        self.build_chromatic_plan(
            catalog,
            &solved.name,
            solved.hsl.h,
            solved.hsl.s,
            solved.hsl.l,
            rgb,
            solved.triple,
        )
    }

    fn resolve_chromatic_key(
        &self,
        catalog: &UniverseCatalog,
        key_index: u16,
        rgb: [u8; 3],
    ) -> Result<NarrationPlan, ResolverError> {
        let entry = catalog
            .app_key_entries(1)
            .get(key_index as usize)
            .ok_or(ResolverError::MissingKeyEntry(1, key_index))?;
        let key_name = catalog.str_slice(entry.key).to_owned();
        let hsl = rgb_to_hsl(rgb[0], rgb[1], rgb[2]);
        Ok(self.build_chromatic_plan(
            catalog,
            &key_name,
            hsl.h,
            hsl.s,
            hsl.l,
            rgb,
            entry.triple,
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn build_chromatic_plan(
        &self,
        catalog: &UniverseCatalog,
        display_name: &str,
        h_deg: f64,
        s: f64,
        l: f64,
        rgb: [u8; 3],
        triple: PhraseTriple,
    ) -> NarrationPlan {
        let mut seg = segment_from_triple(
            catalog,
            "color-name",
            SegmentRole::Item,
            Some(display_name),
            triple,
        );
        if seg.display_text.is_empty() {
            seg.display_text = display_name.to_owned();
        }
        let mat_class = classify_chromatic_material(h_deg, s, l);
        let chromatic = ChromaticVisualHint {
            rgb,
            hue_deg: h_deg as f32,
            saturation: s as f32,
            lightness: l as f32,
            material_class: mat_class,
        };
        let telemetry = [
            chromatic.hue_deg,
            chromatic.saturation,
            chromatic.lightness,
            chromatic.material_class as f32,
        ];
        let accent_linear = srgb_to_linear_hdr(rgb);
        let mut plan = NarrationPlan {
            experience_id: 1,
            zone_index: u16::MAX,
            feature_index: None,
            scene_index: -1,
            region_id: "01:prism".to_owned(),
            accent_rgb: rgb,
            zone_accent_linear_rgb: accent_linear,
            telemetry,
            is_macro_overview: false,
            footer_banner: None,
            music_trigger: None,
            segments: vec![seg],
        };
        populate_segment_visual_hints(
            &mut plan,
            Some("prism"),
            None,
            None,
            Some(chromatic),
            None,
            &[],
        );
        plan
    }

    fn resolve_lexicon_key(
        &self,
        catalog: &UniverseCatalog,
        experience_id: u8,
        key_index: u16,
    ) -> Result<NarrationPlan, ResolverError> {
        let exp = catalog
            .experience(experience_id)
            .ok_or(ResolverError::UnknownExperience(experience_id))?;
        let entry = catalog
            .app_key_entries(experience_id)
            .get(key_index as usize)
            .ok_or(ResolverError::MissingKeyEntry(experience_id, key_index))?;
        let key_str = catalog.str_slice(entry.key);
        let seg = segment_from_triple(
            catalog,
            "lexicon-item",
            SegmentRole::Item,
            Some(key_str),
            entry.triple,
        );
        let zone_accent = catalog
            .zones(experience_id)
            .first()
            .map_or([160, 210, 255], |z| z.color_rgb);
        let exp_title = catalog.str_slice(exp.title);
        let footer = if exp_title.is_empty() {
            None
        } else {
            Some(format!("{experience_id:02} · {exp_title}"))
        };
        let mut plan = NarrationPlan {
            experience_id,
            zone_index: u16::MAX,
            feature_index: None,
            scene_index: catalog
                .zones(experience_id)
                .first()
                .map_or(-1, |z| z.scene_index),
            region_id: format!("{experience_id:02}:lexicon:{key_index}"),
            accent_rgb: zone_accent,
            zone_accent_linear_rgb: srgb_to_linear_hdr(zone_accent),
            telemetry: [experience_id as f32, key_index as f32, 0.0, 0.0],
            is_macro_overview: false,
            footer_banner: footer,
            music_trigger: None,
            segments: vec![seg],
        };
        populate_segment_visual_hints(&mut plan, Some(key_str), None, None, None, None, &[]);
        Ok(plan)
    }

    fn resolve_music(
        &self,
        catalog: &UniverseCatalog,
        x_norm: f32,
        y_norm: f32,
        timbre_idx_hint: u8,
    ) -> NarrationPlan {
        let solved = catalog.solve_music_touch(x_norm, y_norm);
        let timbre_idx = solved.timbre.index.min(3);
        let note_sym = catalog.str_slice(solved.note.symbol);
        let fallback_oct = format!("octava {}", solved.octave);

        let note_seg = segment_from_triple(
            catalog,
            "music-note",
            SegmentRole::Item,
            Some(note_sym),
            solved.note.latin,
        );
        let octave_seg = segment_from_triple(
            catalog,
            "music-octave",
            SegmentRole::Item,
            Some(&fallback_oct),
            solved.octave_triple,
        );
        let timbre_seg = segment_from_triple(
            catalog,
            "music-timbre",
            SegmentRole::Item,
            None,
            solved.timbre.label,
        );

        let accent_rgb = hsl_to_srgb_u8(solved.hue_deg as f64, 85.0, 56.0);
        let effective_timbre = timbre_idx.max(timbre_idx_hint.min(3));
        let amp_hint = (0.75 + 0.05 * (3 - effective_timbre) as f32).clamp(0.4, 1.0);
        let harmonic = HarmonicVisualHint {
            midi: solved.midi,
            octave: solved.octave,
            timbre_index: solved.timbre.index,
            freq_hz: solved.freq_hz as f32,
            amp_hint,
        };
        let telemetry = [
            harmonic.midi as f32,
            harmonic.timbre_index as f32,
            harmonic.freq_hz,
            harmonic.amp_hint,
        ];
        let extra_tags = [
            format!("midi:{}", solved.midi),
            format!("timbre:{effective_timbre}"),
            format!("octave:{}", solved.octave),
        ];

        let mut plan = NarrationPlan {
            experience_id: 2,
            zone_index: u16::MAX,
            feature_index: None,
            scene_index: -1,
            region_id: format!("02:harp:{effective_timbre}"),
            accent_rgb,
            zone_accent_linear_rgb: srgb_to_linear_hdr(accent_rgb),
            telemetry,
            is_macro_overview: false,
            footer_banner: None,
            music_trigger: Some(MusicTriggerEvent {
                midi: solved.midi,
                octave: solved.octave,
                timbre_index: solved.timbre.index,
                freq_hz: solved.freq_hz,
                hue_deg: solved.hue_deg,
                waveform: solved.timbre.waveform,
                detune_cents: solved.timbre.detune_cents,
                filter_hz: solved.timbre.filter_hz,
                attack_sec: solved.timbre.attack_sec,
                release_sec: solved.timbre.release_sec,
            }),
            segments: vec![note_seg, octave_seg, timbre_seg],
        };
        populate_segment_visual_hints(
            &mut plan,
            Some("harp"),
            None,
            None,
            None,
            Some(harmonic),
            &extra_tags,
        );
        plan
    }

    fn resolve_zone(
        &self,
        catalog: &UniverseCatalog,
        zone: &ZoneRecord,
        feature_index: Option<u16>,
        is_macro_zoomed_out: bool,
    ) -> NarrationPlan {
        let zone_id_str = catalog.str_slice(zone.id);
        let region_id = format!("{:02}:{}", zone.app_num, zone_id_str);
        let footer_raw = catalog.str_slice(zone.footer).trim();
        let footer_banner = (!footer_raw.is_empty()).then(|| footer_raw.to_owned());

        let features = catalog.features(zone);
        let resolved_feat_idx = if is_macro_zoomed_out || features.is_empty() {
            None
        } else {
            Some(feature_index.map_or(0, |i| (i as usize).min(features.len() - 1)) as u16)
        };
        let selected_feature = resolved_feat_idx.map(|i| &features[i as usize]);
        let is_macro_overview = selected_feature.is_none();

        let mut segments = Vec::with_capacity(5);

        // 1. Optional EvidenceClass certainty badge (Apps 11..25).
        let emit_evidence = zone.evidence_class.is_present()
            && match self.config.evidence_policy {
                EvidencePolicy::AllPresent => true,
                EvidencePolicy::RespectSpeakDefault => zone.speak_evidence_default != 0,
            };
        if emit_evidence {
            segments.push(segment_from_triple(
                catalog,
                "evidence-class",
                SegmentRole::EvidenceClass,
                None,
                zone.evidence_class,
            ));
        }

        // 2. RegionHeading (`zone.name`).
        let heading_seg = segment_from_triple(
            catalog,
            "zone-name",
            SegmentRole::RegionHeading,
            None,
            zone.name,
        );

        // 3. Item (`feature.name`) if zoomed onto a feature, avoiding duplicate display heading.
        if let Some(feature) = selected_feature {
            let item_seg = segment_from_triple(
                catalog,
                "feature-name",
                SegmentRole::Item,
                None,
                feature.name,
            );
            if item_seg.display_text.trim() != heading_seg.display_text.trim() {
                segments.push(heading_seg);
            }
            segments.push(item_seg);
        } else {
            segments.push(heading_seg);
        }

        // 4. SharedContext metric / material / canonical App 04 multi-token readout.
        if let Some(metric_seg) = resolve_metric_segment(catalog, zone) {
            segments.push(metric_seg);
        }

        // 5. Explanation: `macro_fact` when zoomed out, otherwise `feature.fact`.
        let explanation_triple = selected_feature.map_or(zone.macro_fact, |f| f.fact);
        if explanation_triple.is_present() {
            segments.push(segment_from_triple(
                catalog,
                "fact",
                SegmentRole::Explanation,
                None,
                explanation_triple,
            ));
        }

        let telemetry = [
            zone.range_start,
            zone.range_end,
            zone.phys_low,
            zone.phys_high,
        ];
        let mut plan = NarrationPlan {
            experience_id: zone.app_num,
            zone_index: zone.zone_idx as u16,
            feature_index: resolved_feat_idx,
            scene_index: zone.scene_index,
            region_id,
            accent_rgb: zone.color_rgb,
            zone_accent_linear_rgb: srgb_to_linear_hdr(zone.color_rgb),
            telemetry,
            is_macro_overview,
            footer_banner,
            music_trigger: None,
            segments,
        };
        populate_segment_visual_hints(
            &mut plan,
            Some(zone_id_str),
            zone.unit_kind_enum(),
            zone.metric_mode_enum(),
            None,
            None,
            &[],
        );
        plan
    }
}

/// Converts CSS/W3C HSL (`h_deg ∈ [0, 360)`, `s_pct ∈ [0, 100]`, `l_pct ∈ [0, 100]`) to sRGB `u8`.
pub fn hsl_to_srgb_u8(h_deg: f64, s_pct: f64, l_pct: f64) -> [u8; 3] {
    let h = h_deg.rem_euclid(360.0);
    let s = (s_pct / 100.0).clamp(0.0, 1.0);
    let l = (l_pct / 100.0).clamp(0.0, 1.0);
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - 0.5 * c;
    let (r1, g1, b1) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    [
        ((r1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((g1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
        ((b1 + m) * 255.0).round().clamp(0.0, 255.0) as u8,
    ]
}

fn triple_texts(catalog: &UniverseCatalog, triple: PhraseTriple) -> (String, String, String) {
    let pick = |locale: Locale| -> String {
        triple
            .asset_for(locale)
            .and_then(|idx| catalog.audio_asset(idx))
            .map(|rec| catalog.asset_text(rec).trim().to_owned())
            .unwrap_or_default()
    };
    (pick(Locale::EsEs), pick(Locale::EnUs), pick(Locale::FiFi))
}

fn segment_from_triple(
    catalog: &UniverseCatalog,
    id: &'static str,
    role: SegmentRole,
    display_override: Option<&str>,
    triple: PhraseTriple,
) -> NarrationSegment {
    let (es, en, fi) = triple_texts(catalog, triple);
    let display_text = display_override
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| es.clone());
    NarrationSegment {
        id: id.to_owned(),
        role,
        memory_role: role.memory_role(),
        display_text,
        spoken_es_text: es,
        spoken_en_text: en,
        spoken_fi_text: fi,
        assets: AudioAssetIds::from_triple(triple),
        visual_hint: SegmentVisualHint::default(),
    }
}

/// Converts sRGB `u8` color into linear HDR-ready `[f32; 3]` for shader uniform accents.
pub fn srgb_to_linear_hdr(rgb: [u8; 3]) -> [f32; 3] {
    let conv = |c: u8| {
        let s = c as f32 / 255.0;
        let lin = if s <= 0.04045 {
            s / 12.92
        } else {
            ((s + 0.055) / 1.055).powf(2.4)
        };
        lin.max(0.02)
    };
    [conv(rgb[0]), conv(rgb[1]), conv(rgb[2])]
}

#[inline]
fn classify_chromatic_material(h_deg: f64, s: f64, l: f64) -> u8 {
    if s > 0.78 && (0.35..=0.68).contains(&l) {
        3 // neon / plasma emission
    } else if s >= 0.45 && (25.0..=58.0).contains(&h_deg) && (0.38..=0.65).contains(&l) {
        0 // gold metal
    } else if s < 0.35 || l < 0.25 {
        2 // matte clay / obsidian mineral
    } else {
        1 // thin-film iridescent
    }
}

const DOMAIN_CUE_KEYWORDS: &[&str] = &[
    "sol", "mercurio", "venus", "tierra", "luna", "marte", "ceres", "jupiter", "saturno",
    "urano", "neptuno", "pluton", "aurora", "iss", "satelite", "ozono", "nucleo", "manto",
    "corteza", "geodinamo", "subduccion", "sagitario", "andromeda", "laniakea", "cuasar",
    "reionizacion", "cmb", "inflacion", "higgs", "quark", "orbital", "planck", "einstein",
    "hawking", "gravitacional", "turing", "benard", "adn", "mitocondria", "cloroplasto",
    "neurona", "sinapsis", "hipocampo", "prefrontal", "crispr", "microchip", "ia",
    "fusion", "tokamak", "dyson", "oneill", "europa", "encelado", "seti", "rosetta",
    "primo", "loto", "horizonte",
];

fn normalize_ascii_lower(text: &str) -> String {
    text.chars()
        .map(|ch| match ch {
            'á' | 'à' | 'ä' | 'Á' | 'À' | 'Ä' => 'a',
            'é' | 'è' | 'ë' | 'É' | 'È' | 'Ë' => 'e',
            'í' | 'ì' | 'ï' | 'Í' | 'Ì' | 'Ï' => 'i',
            'ó' | 'ò' | 'ö' | 'Ó' | 'Ò' | 'Ö' => 'o',
            'ú' | 'ù' | 'ü' | 'Ú' | 'Ù' | 'Ü' => 'u',
            'ñ' | 'Ñ' => 'n',
            _ => ch.to_ascii_lowercase(),
        })
        .collect()
}

fn push_unique_tag(tags: &mut Vec<String>, tag: String) {
    if !tag.is_empty() && !tags.iter().any(|t| t == &tag) {
        tags.push(tag);
    }
}

fn populate_segment_visual_hints(
    plan: &mut NarrationPlan,
    zone_slug: Option<&str>,
    unit_kind: Option<UnitKind>,
    metric_mode: Option<MetricMode>,
    chromatic: Option<ChromaticVisualHint>,
    harmonic: Option<HarmonicVisualHint>,
    extra_tags: &[String],
) {
    let app_tag = format!("app:{:02}", plan.experience_id);
    for seg in &mut plan.segments {
        let cue_tag = if chromatic.is_some() {
            VisualCueTag::ChromaticSample
        } else if harmonic.is_some() {
            VisualCueTag::HarmonicFilament
        } else {
            match seg.role {
                SegmentRole::EvidenceClass => VisualCueTag::EvidenceBadge,
                SegmentRole::RegionHeading => VisualCueTag::ZoneOverview,
                SegmentRole::Item => VisualCueTag::FeatureSpotlight,
                SegmentRole::SharedContext => VisualCueTag::MetricScale,
                SegmentRole::Explanation => {
                    if plan.is_macro_overview {
                        VisualCueTag::ZoneOverview
                    } else {
                        VisualCueTag::NarrativeDetail
                    }
                }
            }
        };

        let emphasis_boost = match cue_tag {
            VisualCueTag::ChromaticSample | VisualCueTag::HarmonicFilament => 1.0,
            VisualCueTag::FeatureSpotlight => 0.95,
            VisualCueTag::NarrativeDetail => 0.85,
            VisualCueTag::ZoneOverview => {
                if plan.is_macro_overview {
                    0.80
                } else {
                    0.65
                }
            }
            VisualCueTag::MetricScale => 0.65,
            VisualCueTag::EvidenceBadge => 0.55,
        };

        let mut tags = Vec::with_capacity(8);
        push_unique_tag(&mut tags, app_tag.clone());
        push_unique_tag(&mut tags, format!("role:{}", seg.role.as_str()));
        if let Some(slug) = zone_slug
            && !slug.is_empty()
        {
            push_unique_tag(&mut tags, format!("zone:{slug}"));
        }
        if let Some(f_idx) = plan.feature_index {
            push_unique_tag(&mut tags, format!("feature:{f_idx}"));
        }
        if plan.scene_index >= 0 {
            push_unique_tag(&mut tags, format!("scene:{}", plan.scene_index));
        }
        if let Some(uk) = unit_kind {
            match uk {
                UnitKind::Altitude => push_unique_tag(&mut tags, "unit:altitude".to_owned()),
                UnitKind::Depth => push_unique_tag(&mut tags, "unit:depth".to_owned()),
                UnitKind::None => {}
            }
        }
        if let Some(mm) = metric_mode {
            let mode_name = match mm {
                MetricMode::PlainMetric => "plain",
                MetricMode::Material => "material",
                MetricMode::SolarCenter => "solar-center",
                MetricMode::EarthContinuity => "earth-continuity",
                MetricMode::PlanetAu => "planet-au",
                MetricMode::RangeAndType => "range-type",
            };
            push_unique_tag(&mut tags, format!("metric:{mode_name}"));
        }
        if let Some(c) = chromatic {
            push_unique_tag(&mut tags, format!("material:{}", c.material_class));
        }
        for et in extra_tags {
            push_unique_tag(&mut tags, et.clone());
        }

        let combined_norm = format!(
            "{} {}",
            normalize_ascii_lower(&seg.display_text),
            normalize_ascii_lower(&seg.spoken_es_text)
        );
        for kw in DOMAIN_CUE_KEYWORDS {
            if combined_norm
                .split(|c: char| !c.is_ascii_alphanumeric())
                .any(|tok| tok == *kw)
            {
                push_unique_tag(&mut tags, format!("kw:{kw}"));
            }
        }

        seg.visual_hint = SegmentVisualHint {
            experience_id: plan.experience_id,
            zone_index: plan.zone_index,
            feature_index: plan.feature_index,
            scene_index: plan.scene_index,
            cue_tag,
            visual_tags: tags,
            chromatic,
            harmonic,
            emphasis_boost,
            telemetry: plan.telemetry,
        };
    }
}

fn resolve_metric_segment(catalog: &UniverseCatalog, zone: &ZoneRecord) -> Option<NarrationSegment> {
    let raw_metric = catalog.str_slice(zone.raw_metric).trim();
    let period = catalog.str_slice(zone.period).trim();
    let tokens: Vec<PhraseTriple> = catalog
        .metric_tokens(zone)
        .iter()
        .map(|t| t.triple)
        .filter(|t| t.is_present())
        .collect();

    let display_text = match zone.metric_mode_enum() {
        Some(MetricMode::SolarCenter) => {
            if !tokens.is_empty() {
                tokens
                    .iter()
                    .map(|t| triple_texts(catalog, *t).0)
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join(" · ")
            } else {
                triple_texts(catalog, zone.metric_narration).0
            }
        }
        Some(MetricMode::EarthContinuity) => {
            let t = triple_texts(catalog, zone.metric_narration).0;
            if t.is_empty() {
                raw_metric.to_owned()
            } else {
                t
            }
        }
        Some(MetricMode::Material) => match zone.unit_kind_enum() {
            Some(UnitKind::Altitude) if !raw_metric.is_empty() && raw_metric != "0" => {
                format!("+{raw_metric}")
            }
            Some(UnitKind::Depth) if !raw_metric.is_empty() && raw_metric != "0" => {
                format!("-{raw_metric}")
            }
            _ => raw_metric.to_owned(),
        },
        Some(MetricMode::PlanetAu) => {
            if zone.distance_au > 0.0 && raw_metric.is_empty() {
                format!("{:.2} UA", zone.distance_au)
            } else {
                raw_metric.to_owned()
            }
        }
        Some(MetricMode::RangeAndType) | Some(MetricMode::PlainMetric) | None => {
            if !period.is_empty() && !raw_metric.is_empty() {
                format!("{raw_metric} · {period}")
            } else if !raw_metric.is_empty() {
                raw_metric.to_owned()
            } else if !tokens.is_empty() {
                tokens
                    .iter()
                    .map(|t| triple_texts(catalog, *t).0)
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>()
                    .join(" · ")
            } else {
                triple_texts(catalog, zone.metric_narration).0
            }
        }
    };

    if !tokens.is_empty() {
        let mut es_parts = Vec::with_capacity(tokens.len());
        let mut en_parts = Vec::with_capacity(tokens.len());
        let mut fi_parts = Vec::with_capacity(tokens.len());
        for t in &tokens {
            let (es, en, fi) = triple_texts(catalog, *t);
            if !es.is_empty() {
                es_parts.push(es);
            }
            if !en.is_empty() {
                en_parts.push(en);
            }
            if !fi.is_empty() {
                fi_parts.push(fi);
            }
        }
        return Some(NarrationSegment {
            id: "metric".to_owned(),
            role: SegmentRole::SharedContext,
            memory_role: Some(MemoryRole::SharedContext),
            display_text: if display_text.is_empty() {
                es_parts.join(" · ")
            } else {
                display_text
            },
            spoken_es_text: es_parts.join(" · "),
            spoken_en_text: en_parts.join(" · "),
            spoken_fi_text: fi_parts.join(" · "),
            assets: AudioAssetIds::from_triples(&tokens),
            visual_hint: SegmentVisualHint::default(),
        });
    }

    if zone.metric_narration.is_present() {
        return Some(segment_from_triple(
            catalog,
            "metric",
            SegmentRole::SharedContext,
            (!display_text.is_empty()).then_some(display_text.as_str()),
            zone.metric_narration,
        ));
    }

    if !display_text.is_empty() {
        return Some(NarrationSegment {
            id: "metric".to_owned(),
            role: SegmentRole::SharedContext,
            memory_role: Some(MemoryRole::SharedContext),
            display_text: display_text.clone(),
            spoken_es_text: display_text,
            spoken_en_text: String::new(),
            spoken_fi_text: String::new(),
            assets: AudioAssetIds::default(),
            visual_hint: SegmentVisualHint::default(),
        });
    }

    None
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::memory::SharedNarrationMemory;
    use cosmos_spatial::music_filament_local;

    fn load_real_catalog() -> UniverseCatalog {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf();
        let hcvb = root.join("assets").join("baked").join("universe_catalog.hcvb");
        UniverseCatalog::open(&hcvb, None).expect("baked universe_catalog.hcvb must open")
    }

    #[test]
    fn app_01_colors_prism_resolves_perceptual_hue_to_audio_triple() {
        let catalog = load_real_catalog();
        let resolver = TargetResolver::default();

        // Red saturated sample inside COLORS_PRISM.
        let red_local = hsl_to_prism_local(0.0, 100.0, 50.0);
        let plan = resolver
            .resolve(&catalog, &TargetFocus::ChromaticLocal(red_local))
            .unwrap();
        assert_eq!(plan.experience_id, 1);
        assert_eq!(plan.region_id, "01:prism");
        assert_eq!(plan.accent_rgb, [255, 0, 0]);
        assert_eq!(plan.segments.len(), 1);
        let seg = &plan.segments[0];
        assert_eq!(seg.role, SegmentRole::Item);
        assert_eq!(seg.spoken_es_text.to_lowercase(), "rojo");
        assert!(!seg.spoken_en_text.is_empty());
        assert!(!seg.spoken_fi_text.is_empty());
        assert!(seg.assets.es.is_some());
        assert!(seg.assets.en.is_some());
        assert!(seg.assets.fi.is_some());
    }

    #[test]
    fn app_02_music_harp_resolves_cymatic_filament_touch_and_rest_narration() {
        let catalog = load_real_catalog();
        let resolver = TargetResolver::default();

        // A4 = MIDI 69 -> 440 Hz, timbre 1.
        let _filament_pos = music_filament_local(69, 1);
        let plan = resolver
            .resolve(
                &catalog,
                &TargetFocus::MusicFilament {
                    midi: 69,
                    timbre_idx: 1,
                },
            )
            .unwrap();
        assert_eq!(plan.experience_id, 2);
        let trig = plan.music_trigger.as_ref().expect("music trigger");
        assert_eq!(trig.midi, 69);
        assert_eq!(trig.octave, 4);
        assert!((trig.freq_hz - 440.0).abs() < 1e-6);

        // [nota, octava N, timbre] with all ES/EN/FI audio assets.
        assert_eq!(plan.segments.len(), 3);
        assert_eq!(plan.segments[0].id, "music-note");
        assert_eq!(plan.segments[1].id, "music-octave");
        assert!(plan.segments[1].spoken_es_text.contains("4"));
        assert_eq!(plan.segments[2].id, "music-timbre");
        for seg in &plan.segments {
            assert!(seg.assets.es.is_some());
            assert!(seg.assets.en.is_some());
            assert!(seg.assets.fi.is_some());
        }
    }

    #[test]
    fn app_04_sun_resolves_canonical_three_token_metric_and_macro_vs_feature() {
        let catalog = load_real_catalog();
        let resolver = TargetResolver::default();

        // Zone 0 of App 04 is Sol (Sun).
        let macro_plan = resolver
            .resolve(
                &catalog,
                &TargetFocus::CatalogItem {
                    experience_id: 4,
                    zone_index: 0,
                    feature_index: None,
                    is_macro_zoomed_out: true,
                },
            )
            .unwrap();
        assert!(macro_plan.is_macro_overview);
        assert_eq!(macro_plan.region_id, "04:sun");
        let metric = macro_plan
            .segments
            .iter()
            .find(|s| s.id == "metric")
            .expect("sun metric");
        assert_eq!(
            metric.display_text,
            "Centro del sistema solar · estrella · fuente de luz y energía"
        );
        assert_eq!(metric.assets.playlist_for(Locale::EsEs).len(), 3);
        assert_eq!(metric.assets.playlist_for(Locale::EnUs).len(), 3);
        assert_eq!(metric.assets.playlist_for(Locale::FiFi).len(), 3);

        // Focused feature produces Item (`feature-name`) + Explanation (`feature.fact`), while SharedNarrationMemory skips repeated heading/metric.
        let feat_plan = resolver
            .resolve(
                &catalog,
                &TargetFocus::CatalogItem {
                    experience_id: 4,
                    zone_index: 0,
                    feature_index: Some(0),
                    is_macro_zoomed_out: false,
                },
            )
            .unwrap();
        assert!(!feat_plan.is_macro_overview);
        assert!(feat_plan.segments.iter().any(|s| s.role == SegmentRole::Item));

        let mut memory = SharedNarrationMemory::new();
        let st1 = macro_plan.begin_memory(&mut memory, Locale::EnUs);
        for i in 0..macro_plan.segments.len() {
            memory.mark_pair_completed(&st1, i, true, true);
        }

        let st2 = feat_plan.begin_memory(&mut memory, Locale::EnUs);
        for (seg, skipped) in feat_plan.segments.iter().zip(st2.skipped.iter()) {
            match seg.role {
                SegmentRole::RegionHeading | SegmentRole::SharedContext | SegmentRole::EvidenceClass => {
                    assert!(skipped, "shared segment {} should skip", seg.id);
                }
                SegmentRole::Item | SegmentRole::Explanation => {
                    assert!(!skipped, "item/fact {} must still speak", seg.id);
                }
            }
        }
    }

    #[test]
    fn app_03_to_25_resolve_macro_and_features_across_entire_catalog() {
        let catalog = load_real_catalog();
        let resolver = TargetResolver::default();

        // Check App 03 Deep Earth has Altitude (+) and Depth (-) metrics.
        let mut saw_plus = false;
        let mut saw_minus = false;
        for (zi, _zone) in catalog.zones(3).iter().enumerate() {
            let plan = resolver
                .resolve(
                    &catalog,
                    &TargetFocus::CatalogItem {
                        experience_id: 3,
                        zone_index: zi as u16,
                        feature_index: None,
                        is_macro_zoomed_out: true,
                    },
                )
                .unwrap();
            if let Some(m) = plan.segments.iter().find(|s| s.id == "metric") {
                if m.display_text.starts_with('+') {
                    saw_plus = true;
                }
                if m.display_text.starts_with('-') {
                    saw_minus = true;
                }
            }
        }
        assert!(saw_plus && saw_minus, "app 03 should format altitude and depth");

        // Verify ALL experiences 01..=25 resolve cleanly without errors.
        let p01 = resolver
            .resolve(&catalog, &TargetFocus::ChromaticRgb([0, 140, 255]))
            .unwrap();
        assert!(!p01.segments.is_empty());

        let p02 = resolver
            .resolve(
                &catalog,
                &TargetFocus::MusicNormalized {
                    x_norm: 0.25,
                    y_norm: 0.80,
                },
            )
            .unwrap();
        assert_eq!(p02.segments.len(), 3);

        for app_num in 3..=25 {
            let mac = resolver
                .resolve(
                    &catalog,
                    &TargetFocus::CatalogNormalized {
                        experience_id: app_num,
                        p_norm: 0.45,
                        cross_norm: 0.5,
                        is_macro_zoomed_out: true,
                    },
                )
                .unwrap();
            assert!(mac.is_macro_overview);
            assert!(!mac.segments.is_empty());

            let det = resolver
                .resolve(
                    &catalog,
                    &TargetFocus::CatalogNormalized {
                        experience_id: app_num,
                        p_norm: 0.45,
                        cross_norm: 0.5,
                        is_macro_zoomed_out: false,
                    },
                )
                .unwrap();
            assert!(!det.segments.is_empty());
        }
    }

    #[test]
    fn visible_entity_targets_resolve_across_anchors_filaments_and_color_nodes() {
        let catalog = load_real_catalog();
        let resolver = TargetResolver::default();
        let dummy_pose = cosmos_math::CameraRelativePose {
            pos: glam::Vec3::new(0.0, 0.0, -2.0),
            rel_scale: 0.2,
            orient: glam::Quat::IDENTITY,
            angular_rad: 0.1,
            distance_render_units: 2.0,
            is_far_shell: false,
        };

        // ExperienceAnchor resolves macro overview cleanly even with sentinel u16::MAX indices.
        let anchor_entity = VisibleEntity {
            id: 4,
            experience_id: 4,
            zone_index: u16::MAX,
            feature_index: u16::MAX,
            kind: EntityKind::ExperienceAnchor,
            pose: dummy_pose,
            lod: cosmos_spatial::LodTier::Hero,
            salience_score: 2.0,
        };
        let plan_anchor = resolver
            .resolve(
                &catalog,
                &TargetFocus::from_visible_entity(&anchor_entity, None),
            )
            .unwrap();
        assert!(plan_anchor.is_macro_overview);
        assert_eq!(plan_anchor.region_id, "04:sun");

        // MusicalFilament resolves exact MIDI + timbre trigger.
        let music_entity = VisibleEntity {
            id: 99,
            experience_id: 2,
            zone_index: 2,
            feature_index: 21, // MIDI 48 + 21 = 69 (A4 = 440 Hz)
            kind: EntityKind::MusicalFilament,
            pose: dummy_pose,
            lod: cosmos_spatial::LodTier::Hero,
            salience_score: 1.3,
        };
        let plan_music = resolver
            .resolve(
                &catalog,
                &TargetFocus::from_visible_entity(&music_entity, None),
            )
            .unwrap();
        assert!((plan_music.music_trigger.unwrap().freq_hz - 440.0).abs() < 1e-6);

        // ChromaticSample resolves live HSL color name.
        let color_entity = VisibleEntity {
            id: 50,
            experience_id: 1,
            zone_index: 0,
            feature_index: 4, // 120 deg -> green
            kind: EntityKind::ChromaticSample,
            pose: dummy_pose,
            lod: cosmos_spatial::LodTier::Hero,
            salience_score: 1.2,
        };
        let plan_color = resolver
            .resolve(
                &catalog,
                &TargetFocus::from_visible_entity(&color_entity, None),
            )
            .unwrap();
        assert_eq!(plan_color.experience_id, 1);
        assert!(plan_color.segments[0].assets.es.is_some());
    }

    #[test]
    fn apps_12_to_15_include_evidence_class_under_unified_policy_and_21_to_25_include_footer() {
        let catalog = load_real_catalog();
        let unified = TargetResolver::new(ResolverConfig {
            evidence_policy: EvidencePolicy::AllPresent,
        });
        let legacy = TargetResolver::new(ResolverConfig {
            evidence_policy: EvidencePolicy::RespectSpeakDefault,
        });

        for app_num in 12..=15 {
            let plan = unified
                .resolve(
                    &catalog,
                    &TargetFocus::CatalogItem {
                        experience_id: app_num,
                        zone_index: 0,
                        feature_index: Some(0),
                        is_macro_zoomed_out: false,
                    },
                )
                .unwrap();
            let ev = plan
                .segments
                .iter()
                .find(|s| s.role == SegmentRole::EvidenceClass)
                .unwrap_or_else(|| panic!("app {app_num} missing unified evidence_class"));
            assert!(ev.assets.es.is_some());
            assert!(ev.assets.en.is_some());
            assert!(ev.assets.fi.is_some());

            let orig = legacy
                .resolve(
                    &catalog,
                    &TargetFocus::CatalogItem {
                        experience_id: app_num,
                        zone_index: 0,
                        feature_index: Some(0),
                        is_macro_zoomed_out: false,
                    },
                )
                .unwrap();
            assert!(orig
                .segments
                .iter()
                .all(|s| s.role != SegmentRole::EvidenceClass));
        }

        for app_num in 21..=25 {
            let plan = unified
                .resolve(
                    &catalog,
                    &TargetFocus::CatalogItem {
                        experience_id: app_num,
                        zone_index: 0,
                        feature_index: None,
                        is_macro_zoomed_out: true,
                    },
                )
                .unwrap();
            assert!(
                plan.footer_banner.as_deref().is_some_and(|s| !s.is_empty()),
                "app {app_num} missing footer_banner"
            );
            assert!(plan
                .segments
                .iter()
                .any(|s| s.role == SegmentRole::EvidenceClass));
        }
    }

    #[test]
    fn segment_visual_hints_and_documentary_snapshot_cover_all_domains() {
        let catalog = load_real_catalog();
        let resolver = TargetResolver::default();

        // 01 Colors: HSL [H, S, L, mat] telemetry + chromatic hint + semantic tags
        let p01 = resolver
            .resolve(&catalog, &TargetFocus::ChromaticRgb([255, 0, 0]))
            .unwrap();
        let h01 = &p01.segments[0].visual_hint;
        assert_eq!(h01.cue_tag, VisualCueTag::ChromaticSample);
        assert_eq!(h01.scene_index, -1);
        assert!(h01.chromatic.is_some());
        assert!((h01.telemetry[0] - 0.0).abs() < 1.0);
        assert!((h01.telemetry[1] - 1.0).abs() < 1e-4);
        assert_eq!(h01.emphasis_boost, 1.0);
        assert!(h01.visual_tags.iter().any(|t| t == "app:01"));
        assert!(h01.visual_tags.iter().any(|t| t.starts_with("material:")));

        // 02 Music: [midi, timbre, freq_hz, amp] + harmonic cue hint
        let p02 = resolver
            .resolve(
                &catalog,
                &TargetFocus::MusicFilament {
                    midi: 69,
                    timbre_idx: 2,
                },
            )
            .unwrap();
        let h02 = &p02.segments[0].visual_hint;
        assert_eq!(h02.cue_tag, VisualCueTag::HarmonicFilament);
        assert!(h02.harmonic.is_some());
        assert_eq!(h02.telemetry[0], 69.0);
        assert!((h02.telemetry[2] - 440.0).abs() < 1e-3);
        assert!(h02.visual_tags.iter().any(|t| t == "midi:69"));
        assert!(h02.visual_tags.iter().any(|t| t == "octave:4"));

        // 04 Sun: scene_index, [range0, range1, phys0, phys1], Spotlight boost + keyword tag
        let p04 = resolver
            .resolve(
                &catalog,
                &TargetFocus::CatalogItem {
                    experience_id: 4,
                    zone_index: 0,
                    feature_index: Some(0),
                    is_macro_zoomed_out: false,
                },
            )
            .unwrap();
        let item_idx = p04
            .segments
            .iter()
            .position(|s| s.role == SegmentRole::Item)
            .unwrap();
        let snap = p04.documentary_snapshot(Some(item_idx), 0.42, 0.9);
        assert_eq!(snap.active_zone_index, 0);
        assert_eq!(snap.active_feature_index, 0);
        assert!((snap.narration_cue_progress - 0.42).abs() < 1e-6);
        assert!(snap.emphasis_boost >= 0.9);
        assert!(snap.cue_tags.iter().any(|t| t == "zone:sun"));

        // 13..22: ensure telemetry & tags never empty and snapshot scene_index propagates cleanly
        for app_num in [13, 16, 17, 20, 22] {
            let p = resolver
                .resolve(
                    &catalog,
                    &TargetFocus::CatalogNormalized {
                        experience_id: app_num,
                        p_norm: 0.5,
                        cross_norm: 0.5,
                        is_macro_zoomed_out: false,
                    },
                )
                .unwrap();
            let s = p.documentary_snapshot(Some(p.segments.len() - 1), 0.75, 1.0);
            assert!(s.active_zone_index < u16::MAX);
            assert!(!s.cue_tags.is_empty());
        }
    }

    #[test]
    fn chromatic_key_and_lexicon_key_cover_all_entries() {
        let catalog = load_real_catalog();
        let resolver = TargetResolver::default();
        let color_entries = catalog.app_key_entries(1);
        assert_eq!(color_entries.len(), 130);
        for idx in 0..color_entries.len() {
            let plan = resolver
                .resolve(
                    &catalog,
                    &TargetFocus::ChromaticKey {
                        key_index: idx as u16,
                        rgb: [200, 120, 80],
                    },
                )
                .unwrap();
            assert_eq!(plan.experience_id, 1);
            assert_eq!(plan.segments.len(), 1);
            assert!(plan.segments[0].assets.es.is_some());
            assert!(plan.segments[0].assets.en.is_some());
            assert!(plan.segments[0].assets.fi.is_some());
        }

        let lex_plan = resolver
            .resolve(
                &catalog,
                &TargetFocus::LexiconKey {
                    experience_id: 4,
                    key_index: 0,
                },
            )
            .unwrap();
        assert_eq!(lex_plan.experience_id, 4);
        assert_eq!(lex_plan.segments.len(), 1);
        assert!(lex_plan.segments[0].assets.es.is_some());
    }
}
