use std::collections::HashSet;

use cosmos_manifest::{Locale, UniverseCatalog};
use cosmos_math::{ScalePose64, SectorId};
use cosmos_narration::TargetFocus;
use cosmos_spatial::all_experience_placements;
use glam::{DQuat, DVec3};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Current supported `.hcj.json` manifest schema version.
pub const CURRENT_SCHEMA_VERSION: u32 = 1;

/// URI scheme prefix for portable Story-Journey deep-links.
pub const SHARE_URI_PREFIX: &str = "hypercosmos://journey?hcj=";

#[derive(Debug, Error)]
pub enum JourneyError {
    #[error("unsupported journey schema_version {0} (expected {CURRENT_SCHEMA_VERSION})")]
    UnsupportedSchemaVersion(u32),
    #[error("journey manifest `{0}` has empty id")]
    EmptyJourneyId(String),
    #[error("journey `{0}` contains no chapters or no beats")]
    EmptyJourney(String),
    #[error("unknown sector identifier `{0}`")]
    UnknownSector(String),
    #[error("non-finite or degenerate pose in beat `{0}`")]
    InvalidPose(String),
    #[error("beat `{beat_id}` references missing experience {experience_id}")]
    InvalidExperience { beat_id: String, experience_id: u8 },
    #[error("beat `{beat_id}` references missing zone {zone_index} in experience {experience_id}")]
    InvalidZone {
        beat_id: String,
        experience_id: u8,
        zone_index: u16,
    },
    #[error("beat `{beat_id}` references missing feature {feature_index} in exp {experience_id} zone {zone_index}")]
    InvalidFeature {
        beat_id: String,
        experience_id: u8,
        zone_index: u16,
        feature_index: u16,
    },
    #[error("beat `{beat_id}` references invalid lexicon key {key_index} in experience {experience_id}")]
    InvalidKeyEntry {
        beat_id: String,
        experience_id: u8,
        key_index: u16,
    },
    #[error("journey `{0}` not found in registry")]
    JourneyNotFound(String),
    #[error("invalid share URI format")]
    InvalidShareUri,
    #[error("share URI checksum mismatch (expected {expected:#010x}, got {actual:#010x})")]
    ShareChecksumMismatch { expected: u32, actual: u32 },
    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

/// Multilingual title container matching the runtime's ES / EN / FI narration locales.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct LocalizedTitle {
    #[serde(default, alias = "es-ES")]
    pub es: String,
    #[serde(default, alias = "en-US")]
    pub en: String,
    #[serde(default, alias = "fi-FI")]
    pub fi: String,
}

impl LocalizedTitle {
    pub fn new(es: impl Into<String>, en: impl Into<String>, fi: impl Into<String>) -> Self {
        Self {
            es: es.into(),
            en: en.into(),
            fi: fi.into(),
        }
    }

    pub fn unified(label: impl Into<String>) -> Self {
        let text = label.into();
        Self {
            es: text.clone(),
            en: text.clone(),
            fi: text,
        }
    }

    /// Selects the localized string with non-empty fallback across ES -> EN -> FI.
    pub fn pick(&self, locale: Locale) -> &str {
        let primary = match locale {
            Locale::EsEs => &self.es,
            Locale::EnUs => &self.en,
            Locale::FiFi => &self.fi,
        };
        if !primary.is_empty() {
            return primary;
        }
        if !self.en.is_empty() {
            return &self.en;
        }
        if !self.es.is_empty() {
            return &self.es;
        }
        &self.fi
    }
}

fn default_orient_xyzw() -> [f64; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

/// Portable double-precision logarithmic-scale pose serialized inside `.hcj.json`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SerializedPose64 {
    #[serde(alias = "sector_id")]
    pub sector: String,
    pub local_pos: [f64; 3],
    pub scale_exp: f64,
    #[serde(default = "default_orient_xyzw")]
    pub orient_xyzw: [f64; 4],
}

impl SerializedPose64 {
    pub fn from_pose64(pose: &ScalePose64) -> Self {
        let q = pose.orient.to_array();
        Self {
            sector: sector_to_name(pose.sector_id),
            local_pos: pose.local_pos.to_array(),
            scale_exp: pose.scale_exp,
            orient_xyzw: q,
        }
    }

    pub fn to_pose64(&self, beat_id: &str) -> Result<ScalePose64, JourneyError> {
        let sector = sector_from_name(&self.sector)?;
        if !self.scale_exp.is_finite()
            || !self.local_pos.iter().all(|v| v.is_finite())
            || !self.orient_xyzw.iter().all(|v| v.is_finite())
        {
            return Err(JourneyError::InvalidPose(beat_id.to_owned()));
        }
        let raw_q = DQuat::from_array(self.orient_xyzw);
        if raw_q.length_squared() < 1e-12 {
            return Err(JourneyError::InvalidPose(beat_id.to_owned()));
        }
        Ok(ScalePose64::new(
            sector,
            DVec3::from_array(self.local_pos),
            self.scale_exp,
            raw_q.normalize(),
        ))
    }
}

/// Converts a canonical `SectorId` into its stable `.hcj.json` sector token.
pub fn sector_to_name(sector: SectorId) -> String {
    match sector {
        SectorId::UNIVERSE_ROOT => "UNIVERSE_ROOT".to_owned(),
        SectorId::COLORS_PRISM => "COLORS_PRISM".to_owned(),
        SectorId::MUSIC_HARP => "MUSIC_HARP".to_owned(),
        SectorId::EARTH_ORBIT => "EARTH_ORBIT".to_owned(),
        SectorId::SOLAR_SYSTEM => "SOLAR_SYSTEM".to_owned(),
        SectorId::HYPER_LOTUS_25_HORIZON => "HYPER_LOTUS_25_HORIZON".to_owned(),
        other => {
            if let Some(pl) = all_experience_placements()
                .iter()
                .find(|p| p.sector_id == other)
            {
                format!("EXP_{:02}", pl.id.0)
            } else {
                format!("SECTOR_{}", other.0)
            }
        }
    }
}

/// Resolves a serialized sector token (`"EARTH_ORBIT"`, `"EXP_05"`, `"SECTOR_12"`, etc.) to `SectorId`.
pub fn sector_from_name(name: &str) -> Result<SectorId, JourneyError> {
    let trimmed = name.trim();
    match trimmed {
        "UNIVERSE_ROOT" => return Ok(SectorId::UNIVERSE_ROOT),
        "COLORS_PRISM" => return Ok(SectorId::COLORS_PRISM),
        "MUSIC_HARP" => return Ok(SectorId::MUSIC_HARP),
        "EARTH_ORBIT" => return Ok(SectorId::EARTH_ORBIT),
        "SOLAR_SYSTEM" | "SOLAR_BARYCENTER" => return Ok(SectorId::SOLAR_SYSTEM),
        "HYPER_LOTUS_25_HORIZON" => return Ok(SectorId::HYPER_LOTUS_25_HORIZON),
        _ => {}
    }

    if let Some(rest) = trimmed.strip_prefix("EXP_")
        && let Ok(exp_id) = rest.parse::<u8>()
        && let Some(pl) = all_experience_placements()
            .iter()
            .find(|p| p.id.0 == exp_id)
    {
        return Ok(pl.sector_id);
    }

    if let Some(rest) = trimmed.strip_prefix("SECTOR_")
        && let Ok(raw) = rest.parse::<u32>()
        && raw <= SectorId::HYPER_LOTUS_25_HORIZON.0
    {
        return Ok(SectorId(raw));
    }

    // Match against experience placement slugs (`05_milky_way`, `milky_way`, etc.).
    let lower = trimmed.to_ascii_lowercase();
    for pl in all_experience_placements() {
        let slug_lower = pl.slug.to_ascii_lowercase();
        if lower == slug_lower || lower == format!("exp_{:02}_{}", pl.id.0, slug_lower) {
            return Ok(pl.sector_id);
        }
    }

    Err(JourneyError::UnknownSector(trimmed.to_owned()))
}

/// Target audio/narration selector attached to a `JourneyBeat`.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum BeatTargetSelector {
    #[default]
    None,
    CatalogZone {
        experience_id: u8,
        zone_index: u16,
        #[serde(default)]
        feature_index: Option<u16>,
        #[serde(default)]
        is_macro_zoomed_out: bool,
    },
    ChromaticRgb {
        rgb: [u8; 3],
    },
    ChromaticKey {
        key_index: u16,
        rgb: [u8; 3],
    },
    MusicFilament {
        midi: u8,
        timbre_idx: u8,
    },
    LexiconKey {
        experience_id: u8,
        key_index: u16,
    },
}

impl BeatTargetSelector {
    pub fn to_target_focus(&self) -> Option<TargetFocus> {
        match *self {
            Self::None => None,
            Self::CatalogZone {
                experience_id,
                zone_index,
                feature_index,
                is_macro_zoomed_out,
            } => Some(TargetFocus::CatalogItem {
                experience_id,
                zone_index,
                feature_index,
                is_macro_zoomed_out,
            }),
            Self::ChromaticRgb { rgb } => Some(TargetFocus::ChromaticRgb(rgb)),
            Self::ChromaticKey { key_index, rgb } => Some(TargetFocus::ChromaticKey {
                key_index,
                rgb,
            }),
            Self::MusicFilament { midi, timbre_idx } => Some(TargetFocus::MusicFilament {
                midi: midi.clamp(48, 95),
                timbre_idx: timbre_idx.min(3),
            }),
            Self::LexiconKey {
                experience_id,
                key_index,
            } => Some(TargetFocus::LexiconKey {
                experience_id,
                key_index,
            }),
        }
    }

    pub const fn experience_id(&self) -> Option<u8> {
        match *self {
            Self::None => None,
            Self::CatalogZone { experience_id, .. }
            | Self::LexiconKey { experience_id, .. } => Some(experience_id),
            Self::ChromaticRgb { .. } | Self::ChromaticKey { .. } => Some(1),
            Self::MusicFilament { .. } => Some(2),
        }
    }
}

/// Observer movement during a beat. `StaticHold` preserves the authored pose so
/// tracked head movement, rather than an automatic orbit, controls the viewpoint.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum MotionProfileKind {
    #[default]
    OrbitalParallax,
    HelicalAscent,
    ScaleDiveCruise,
    FigureEightShowcase,
    StaticHold,
}

fn default_yaw_rate() -> f32 {
    5.5
}
fn default_pitch_wave() -> f32 {
    1.8
}
fn default_scale_breathing() -> f64 {
    0.08
}
fn default_orbit_radius_factor() -> f32 {
    0.35
}
fn default_max_sentences() -> u8 {
    2
}
fn default_min_dwell() -> f32 {
    2.4
}
fn default_max_dwell() -> f32 {
    18.0
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeatMotionProfile {
    #[serde(default)]
    pub kind: MotionProfileKind,
    #[serde(default = "default_yaw_rate")]
    pub yaw_deg_per_sec: f32,
    #[serde(default = "default_pitch_wave")]
    pub pitch_wave_deg: f32,
    #[serde(default = "default_scale_breathing")]
    pub scale_breathing_du: f64,
    #[serde(default = "default_orbit_radius_factor")]
    pub orbit_radius_factor: f32,
    #[serde(default = "default_max_sentences")]
    pub max_sentences_per_vantage: u8,
    #[serde(default = "default_min_dwell")]
    pub min_dwell_sec: f32,
    #[serde(default = "default_max_dwell")]
    pub max_dwell_sec: f32,
}

impl Default for BeatMotionProfile {
    fn default() -> Self {
        Self {
            kind: MotionProfileKind::OrbitalParallax,
            yaw_deg_per_sec: default_yaw_rate(),
            pitch_wave_deg: default_pitch_wave(),
            scale_breathing_du: default_scale_breathing(),
            orbit_radius_factor: default_orbit_radius_factor(),
            max_sentences_per_vantage: default_max_sentences(),
            min_dwell_sec: default_min_dwell(),
            max_dwell_sec: default_max_dwell(),
        }
    }
}

/// Arrival spline curve linking adjacent beats across scales & sectors.
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum TransitionKind {
    #[default]
    CatmullRomScaleSpline,
    LogScaleHyperDive,
    ContinuousOrbitBridge,
    /// Spherical travel around Earth's center without cutting through the planet.
    EarthOrbitArc,
    /// Spherical travel around the physical Sun, keeping exterior observers outside it.
    SolarOrbitArc,
    /// Authored cut to the destination on the next playing update, with no travel frames.
    InstantBookmark,
}

fn default_transition_duration() -> f32 {
    2.2
}
fn default_narration_lead() -> f32 {
    0.35
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BeatTransition {
    #[serde(default)]
    pub kind: TransitionKind,
    #[serde(default = "default_transition_duration")]
    pub duration_sec: f32,
    #[serde(default = "default_narration_lead")]
    pub narration_lead_sec: f32,
}

impl Default for BeatTransition {
    fn default() -> Self {
        Self {
            kind: TransitionKind::CatmullRomScaleSpline,
            duration_sec: default_transition_duration(),
            narration_lead_sec: default_narration_lead(),
        }
    }
}

/// Single curated stop along a Story-Journey.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JourneyBeat {
    pub beat_id: String,
    pub pose: SerializedPose64,
    #[serde(default)]
    pub look_at_pose: Option<SerializedPose64>,
    #[serde(default)]
    pub target: BeatTargetSelector,
    #[serde(default)]
    pub motion: BeatMotionProfile,
    #[serde(default)]
    pub transition_in: BeatTransition,
    #[serde(default)]
    pub caption_hint: Option<String>,
}

/// Ordered chapter grouping contiguous thematic beats.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct JourneyChapter {
    pub id: String,
    #[serde(default)]
    pub act_index: u8,
    pub title: LocalizedTitle,
    pub beats: Vec<JourneyBeat>,
}

/// Complete serializable `.hcj.json` Story-Journey manifest (`schema_version = 1`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryJourneyManifest {
    pub schema_version: u32,
    pub id: String,
    pub title: LocalizedTitle,
    #[serde(default)]
    pub subtitle: LocalizedTitle,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub loop_journey: bool,
    pub chapters: Vec<JourneyChapter>,
}

impl StoryJourneyManifest {
    pub fn total_beats(&self) -> usize {
        self.chapters.iter().map(|c| c.beats.len()).sum()
    }

    pub fn flatten_beats(&self) -> Vec<(usize, usize, &JourneyBeat)> {
        let mut out = Vec::with_capacity(self.total_beats());
        for (c_idx, ch) in self.chapters.iter().enumerate() {
            for (b_idx, beat) in ch.beats.iter().enumerate() {
                out.push((c_idx, b_idx, beat));
            }
        }
        out
    }

    /// Validates schema version, pose finiteness/sectors, and optional catalog references.
    pub fn validate(&self, catalog: Option<&UniverseCatalog>) -> Result<(), JourneyError> {
        if self.schema_version != CURRENT_SCHEMA_VERSION {
            return Err(JourneyError::UnsupportedSchemaVersion(self.schema_version));
        }
        if self.id.trim().is_empty() {
            return Err(JourneyError::EmptyJourneyId(self.id.clone()));
        }
        if self.total_beats() == 0 {
            return Err(JourneyError::EmptyJourney(self.id.clone()));
        }

        let mut seen_ids = HashSet::new();
        for ch in &self.chapters {
            for beat in &ch.beats {
                beat.pose.to_pose64(&beat.beat_id)?;
                if let Some(ref look_at) = beat.look_at_pose {
                    look_at.to_pose64(&beat.beat_id)?;
                }
                if !beat.beat_id.is_empty() {
                    seen_ids.insert(&beat.beat_id);
                }
                if let Some(cat) = catalog {
                    validate_target_against_catalog(&beat.beat_id, &beat.target, cat)?;
                }
            }
        }
        Ok(())
    }

    pub fn from_json_str(json: &str) -> Result<Self, JourneyError> {
        let manifest: Self = serde_json::from_str(json)?;
        manifest.validate(None)?;
        Ok(manifest)
    }

    pub fn to_json_pretty(&self) -> Result<String, JourneyError> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    /// Encodes a portable `hypercosmos://journey?hcj=v1.<crc32hex>.<base64url>` deep link.
    pub fn encode_share_uri(&self) -> Result<String, JourneyError> {
        let compact = serde_json::to_vec(self)?;
        let checksum = crc32fast::hash(&compact);
        let payload_b64 = base64url_encode(&compact);
        Ok(format!("{SHARE_URI_PREFIX}v1.{checksum:08x}.{payload_b64}"))
    }

    /// Decodes and checksum-verifies a `hypercosmos://journey?hcj=...` deep link.
    pub fn decode_share_uri(uri: &str) -> Result<Self, JourneyError> {
        let body = uri
            .trim()
            .strip_prefix(SHARE_URI_PREFIX)
            .ok_or(JourneyError::InvalidShareUri)?;
        let mut parts = body.split('.');
        let ver = parts.next().ok_or(JourneyError::InvalidShareUri)?;
        let hex_crc = parts.next().ok_or(JourneyError::InvalidShareUri)?;
        let b64 = parts.next().ok_or(JourneyError::InvalidShareUri)?;
        if ver != "v1" || parts.next().is_some() {
            return Err(JourneyError::InvalidShareUri);
        }
        let expected = u32::from_str_radix(hex_crc, 16).map_err(|_| JourneyError::InvalidShareUri)?;
        let raw = base64url_decode(b64).ok_or(JourneyError::InvalidShareUri)?;
        let actual = crc32fast::hash(&raw);
        if actual != expected {
            return Err(JourneyError::ShareChecksumMismatch { expected, actual });
        }
        let manifest: Self = serde_json::from_slice(&raw)?;
        manifest.validate(None)?;
        Ok(manifest)
    }
}

fn validate_target_against_catalog(
    beat_id: &str,
    target: &BeatTargetSelector,
    catalog: &UniverseCatalog,
) -> Result<(), JourneyError> {
    match *target {
        BeatTargetSelector::None | BeatTargetSelector::ChromaticRgb { .. } => Ok(()),
        BeatTargetSelector::CatalogZone {
            experience_id,
            zone_index,
            feature_index,
            ..
        } => {
            if !(3..=25).contains(&experience_id) {
                return Err(JourneyError::InvalidExperience {
                    beat_id: beat_id.to_owned(),
                    experience_id,
                });
            }
            let zones = catalog.zones(experience_id);
            let zone = zones.get(zone_index as usize).ok_or(JourneyError::InvalidZone {
                beat_id: beat_id.to_owned(),
                experience_id,
                zone_index,
            })?;
            if let Some(f_idx) = feature_index {
                let feats = catalog.features(zone);
                if f_idx as usize >= feats.len() {
                    return Err(JourneyError::InvalidFeature {
                        beat_id: beat_id.to_owned(),
                        experience_id,
                        zone_index,
                        feature_index: f_idx,
                    });
                }
            }
            Ok(())
        }
        BeatTargetSelector::ChromaticKey { key_index, .. } => {
            if key_index as usize >= catalog.app_key_entries(1).len() {
                return Err(JourneyError::InvalidKeyEntry {
                    beat_id: beat_id.to_owned(),
                    experience_id: 1,
                    key_index,
                });
            }
            Ok(())
        }
        BeatTargetSelector::MusicFilament { .. } => Ok(()),
        BeatTargetSelector::LexiconKey {
            experience_id,
            key_index,
        } => {
            if !(1..=25).contains(&experience_id) {
                return Err(JourneyError::InvalidExperience {
                    beat_id: beat_id.to_owned(),
                    experience_id,
                });
            }
            if key_index as usize >= catalog.app_key_entries(experience_id).len() {
                return Err(JourneyError::InvalidKeyEntry {
                    beat_id: beat_id.to_owned(),
                    experience_id,
                    key_index,
                });
            }
            Ok(())
        }
    }
}

const B64URL_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

fn base64url_encode(input: &[u8]) -> String {
    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = *chunk.get(1).unwrap_or(&0) as u32;
        let b2 = *chunk.get(2).unwrap_or(&0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(B64URL_ALPHABET[((n >> 18) & 0x3f) as usize] as char);
        out.push(B64URL_ALPHABET[((n >> 12) & 0x3f) as usize] as char);
        if chunk.len() > 1 {
            out.push(B64URL_ALPHABET[((n >> 6) & 0x3f) as usize] as char);
        }
        if chunk.len() > 2 {
            out.push(B64URL_ALPHABET[(n & 0x3f) as usize] as char);
        }
    }
    out
}

fn base64url_decode(input: &str) -> Option<Vec<u8>> {
    let bytes = input.as_bytes();
    if bytes.is_empty() || bytes.len() % 4 == 1 {
        return None;
    }
    let decode_char = |c: u8| -> Option<u32> {
        match c {
            b'A'..=b'Z' => Some((c - b'A') as u32),
            b'a'..=b'z' => Some((c - b'a' + 26) as u32),
            b'0'..=b'9' => Some((c - b'0' + 52) as u32),
            b'-' => Some(62),
            b'_' => Some(63),
            _ => None,
        }
    };

    let mut out = Vec::with_capacity((bytes.len() * 3) / 4);
    for chunk in bytes.chunks(4) {
        let c0 = decode_char(chunk[0])?;
        let c1 = decode_char(chunk[1])?;
        let c2 = match chunk.get(2) {
            Some(&ch) => Some(decode_char(ch)?),
            None => None,
        };
        let c3 = match chunk.get(3) {
            Some(&ch) => Some(decode_char(ch)?),
            None => None,
        };
        let n = (c0 << 18) | (c1 << 12) | (c2.unwrap_or(0) << 6) | c3.unwrap_or(0);
        out.push(((n >> 16) & 0xff) as u8);
        if c2.is_some() {
            out.push(((n >> 8) & 0xff) as u8);
        }
        if c3.is_some() {
            out.push((n & 0xff) as u8);
        }
    }
    Some(out)
}
