use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

/// Magic header for `universe_catalog.hcvb`.
pub const MAGIC_CAT: [u8; 8] = *b"HCVRCAT1";
/// Magic header for `universe_audio.hcvp`.
pub const MAGIC_AUD: [u8; 8] = *b"HCVRAUD1";
/// Current binary catalog schema version.
pub const FORMAT_VERSION: u32 = 1;
/// Sentinel indicating an absent `AudioAssetRecord` index inside a `PhraseTriple`.
pub const NONE_ASSET: u32 = u32::MAX;
/// Audio engine timestretch backend expected by `cosmos_audio` (`platform_spec.md`).
pub const AUDIO_STRETCH_BACKEND: &str = "wsola 0.3.0";

/// Narration audio/caption locale.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Locale {
    EsEs = 0,
    EnUs = 1,
    FiFi = 2,
}

impl Locale {
    pub const ALL: [Self; 3] = [Self::EsEs, Self::EnUs, Self::FiFi];

    pub const fn code(self) -> &'static str {
        match self {
            Self::EsEs => "es-ES",
            Self::EnUs => "en-US",
            Self::FiFi => "fi-FI",
        }
    }

    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::EsEs),
            1 => Some(Self::EnUs),
            2 => Some(Self::FiFi),
            _ => None,
        }
    }
}

/// Interactive topology kind for an experience (`01..25`).
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ExperienceKind {
    ColorSpectrum = 0,
    MusicSynth = 1,
    StratifiedZones = 2,
}

impl ExperienceKind {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::ColorSpectrum),
            1 => Some(Self::MusicSynth),
            2 => Some(Self::StratifiedZones),
            _ => None,
        }
    }
}

/// Procedural synth oscillator waveform (shared by App 02 and App 03..25 drones).
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Waveform {
    Sine = 0,
    Triangle = 1,
    Sawtooth = 2,
    Square = 3,
}

impl Waveform {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::Sine),
            1 => Some(Self::Triangle),
            2 => Some(Self::Sawtooth),
            3 => Some(Self::Square),
            _ => None,
        }
    }
}

/// How a zone's spoken metric readout is assembled from catalog entries.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum MetricMode {
    PlainMetric = 0,
    Material = 1,
    SolarCenter = 2,
    EarthContinuity = 3,
    PlanetAu = 4,
    RangeAndType = 5,
}

impl MetricMode {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::PlainMetric),
            1 => Some(Self::Material),
            2 => Some(Self::SolarCenter),
            3 => Some(Self::EarthContinuity),
            4 => Some(Self::PlanetAu),
            5 => Some(Self::RangeAndType),
            _ => None,
        }
    }
}

/// App 03 vertical scale unit (`altitude` vs ocean `depth`).
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum UnitKind {
    None = 0,
    Altitude = 1,
    Depth = 2,
}

impl UnitKind {
    pub const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::None),
            1 => Some(Self::Altitude),
            2 => Some(Self::Depth),
            _ => None,
        }
    }
}

/// Byte slice `(byte_off, byte_len)` into the interned UTF-8 string arena.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Pod, Zeroable)]
pub struct StrSlice {
    pub byte_off: u32,
    pub byte_len: u32,
}

/// Word/phrase highlight timing cue with preconverted UTF-8 byte & Unicode scalar indices.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Pod, Zeroable)]
pub struct CueRecord {
    pub start_ms: u32,
    pub end_ms: u32,
    pub start_byte: u32,
    pub end_byte: u32,
    pub start_scalar: u32,
    pub end_scalar: u32,
}

/// Metadata & slice locator for one syncvoice MP3 asset inside `universe_audio.hcvp`.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Pod, Zeroable)]
pub struct AudioAssetRecord {
    pub mp3_offset: u64,
    pub mp3_len: u32,
    pub duration_ms: u32,
    pub cue_start: u32,
    pub cue_count: u32,
    pub text: StrSlice,
    pub external_id: StrSlice,
    pub sha256: [u8; 32],
    pub locale: u8,
    pub app_num: u8,
    pub _pad: [u8; 6],
}

impl AudioAssetRecord {
    pub const fn locale_enum(&self) -> Option<Locale> {
        Locale::from_u8(self.locale)
    }
}

/// Paired `es-ES`, `en-US`, and `fi-FI` asset indices for one source phrase.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Pod, Zeroable)]
pub struct PhraseTriple {
    pub es_asset: u32,
    pub en_asset: u32,
    pub fi_asset: u32,
}

impl PhraseTriple {
    pub const NONE: Self = Self {
        es_asset: NONE_ASSET,
        en_asset: NONE_ASSET,
        fi_asset: NONE_ASSET,
    };

    pub const fn is_present(&self) -> bool {
        self.es_asset != NONE_ASSET
    }

    pub const fn asset_for(&self, locale: Locale) -> Option<u32> {
        let idx = match locale {
            Locale::EsEs => self.es_asset,
            Locale::EnUs => self.en_asset,
            Locale::FiFi => self.fi_asset,
        };
        if idx == NONE_ASSET {
            None
        } else {
            Some(idx)
        }
    }
}

/// One sorted lexicon entry mapping `(app_num, source_key)` to its bilingual audio triple.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Pod, Zeroable)]
pub struct AppKeyEntryRecord {
    pub key: StrSlice,
    pub triple: PhraseTriple,
}

/// Sub-token in a zone's spoken metric sequence (e.g. App 04 Sun canonical 3-part readout).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Pod, Zeroable)]
pub struct MetricTokenRecord {
    pub triple: PhraseTriple,
}

/// Procedural drone & haptic synthesizer parameters attached to a zone (`03..25`).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct ZoneSoundRecord {
    pub base_hz: f32,
    pub filter_hz: f32,
    pub noise: f32,
    pub pulse: f32,
    pub waveform: u8,
    pub _pad: [u8; 3],
}

impl ZoneSoundRecord {
    pub const fn waveform_enum(&self) -> Option<Waveform> {
        Waveform::from_u8(self.waveform)
    }
}

/// Full metadata, spatial bounds, spoken triples, and sound recipe for one zone (`03..25`).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Pod, Zeroable)]
pub struct ZoneRecord {
    pub app_num: u8,
    pub zone_idx: u8,
    pub color_rgb: [u8; 3],
    pub speak_evidence_default: u8,
    pub has_sound: u8,
    pub metric_mode: u8,
    pub unit_kind: u8,
    pub _pad: [u8; 3],
    pub id: StrSlice,
    pub range_start: f32,
    pub range_end: f32,
    pub phys_low: f32,
    pub phys_high: f32,
    pub distance_au: f32,
    pub scene_index: i32,
    pub feature_start: u32,
    pub feature_count: u32,
    pub metric_token_start: u32,
    pub metric_token_count: u32,
    pub sound: ZoneSoundRecord,
    pub name: PhraseTriple,
    pub macro_fact: PhraseTriple,
    pub evidence_class: PhraseTriple,
    pub metric_narration: PhraseTriple,
    pub raw_metric: StrSlice,
    pub period: StrSlice,
    pub footer: StrSlice,
}

impl ZoneRecord {
    pub const fn metric_mode_enum(&self) -> Option<MetricMode> {
        MetricMode::from_u8(self.metric_mode)
    }

    pub const fn unit_kind_enum(&self) -> Option<UnitKind> {
        UnitKind::from_u8(self.unit_kind)
    }
}

/// Sub-item `(name, explanation)` within a zone (`2,456` across `03..25`).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Pod, Zeroable)]
pub struct FeatureRecord {
    pub name: PhraseTriple,
    pub fact: PhraseTriple,
}

/// App 04 solar-system celestial body placement (`BODY_LAYOUT`, `11` records).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct BodyLayoutRecord {
    pub zone_id: StrSlice,
    pub zone_index: u32,
    pub x: f32,
    pub lane: f32,
    pub radius: f32,
}

/// Apps 05..07 cosmic landmark placement (`LANDMARKS`, `31` records).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct LandmarkRecord {
    pub app_num: u8,
    pub _pad: [u8; 3],
    pub zone_id: StrSlice,
    pub p: f32,
    pub lane: f32,
    pub size: f32,
}

/// App 02 chromatic pitch class (`0..11`, Latin solfege + concert symbol).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Pod, Zeroable)]
pub struct MusicNoteRecord {
    pub pitch_index: u8,
    pub _pad: [u8; 3],
    pub symbol: StrSlice,
    pub latin: PhraseTriple,
}

/// App 02 synth timbre band (`0..3`: cristal, aire, cálido, profundo).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct MusicTimbreRecord {
    pub index: u8,
    pub waveform: u8,
    pub _pad: [u8; 2],
    pub y_max: f32,
    pub detune_cents: f32,
    pub filter_hz: f32,
    pub attack_sec: f32,
    pub release_sec: f32,
    pub label: PhraseTriple,
}

impl MusicTimbreRecord {
    pub const fn waveform_enum(&self) -> Option<Waveform> {
        Waveform::from_u8(self.waveform)
    }
}

/// Top-level experience metadata (`01..25`).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Pod, Zeroable)]
pub struct ExperienceRecord {
    pub app_num: u8,
    pub kind: u8,
    pub feedback: u8,
    pub _pad: u8,
    pub id: StrSlice,
    pub label: StrSlice,
    pub title: StrSlice,
    pub zone_start: u32,
    pub zone_count: u32,
    pub key_start: u32,
    pub key_count: u32,
}

impl ExperienceRecord {
    pub const fn kind_enum(&self) -> Option<ExperienceKind> {
        ExperienceKind::from_u8(self.kind)
    }
}

/// 128-byte aligned header at offset `0` of `universe_catalog.hcvb`.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Pod, Zeroable)]
pub struct HcvbHeader {
    pub magic: [u8; 8],
    pub version: u32,
    pub flags: u32,
    pub catalog_crc32: u32,
    pub audio_crc32: u32,
    pub audio_pack_size: u64,
    pub experiences_off: u32,
    pub experiences_len: u32,
    pub zones_off: u32,
    pub zones_len: u32,
    pub features_off: u32,
    pub features_len: u32,
    pub metric_tokens_off: u32,
    pub metric_tokens_len: u32,
    pub key_entries_off: u32,
    pub key_entries_len: u32,
    pub assets_off: u32,
    pub assets_len: u32,
    pub cues_off: u32,
    pub cues_len: u32,
    pub body_layout_off: u32,
    pub body_layout_len: u32,
    pub landmarks_off: u32,
    pub landmarks_len: u32,
    pub music_notes_off: u32,
    pub music_notes_len: u32,
    pub music_timbres_off: u32,
    pub music_timbres_len: u32,
    pub strings_off: u32,
    pub strings_len: u32,
}

const _: () = {
    assert!(std::mem::size_of::<StrSlice>() == 8);
    assert!(std::mem::size_of::<CueRecord>() == 24);
    assert!(std::mem::size_of::<AudioAssetRecord>() == 80);
    assert!(std::mem::size_of::<PhraseTriple>() == 12);
    assert!(std::mem::size_of::<AppKeyEntryRecord>() == 20);
    assert!(std::mem::size_of::<MetricTokenRecord>() == 12);
    assert!(std::mem::size_of::<ZoneSoundRecord>() == 20);
    assert!(std::mem::size_of::<ZoneRecord>() == 152);
    assert!(std::mem::size_of::<FeatureRecord>() == 24);
    assert!(std::mem::size_of::<BodyLayoutRecord>() == 24);
    assert!(std::mem::size_of::<LandmarkRecord>() == 24);
    assert!(std::mem::size_of::<MusicNoteRecord>() == 24);
    assert!(std::mem::size_of::<MusicTimbreRecord>() == 36);
    assert!(std::mem::size_of::<ExperienceRecord>() == 44);
    assert!(std::mem::size_of::<HcvbHeader>() == 128);
};
