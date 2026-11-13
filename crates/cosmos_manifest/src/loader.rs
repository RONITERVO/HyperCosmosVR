use std::fs::File;
use std::ops::Deref;
use std::path::Path;

use bytemuck::Pod;
use memmap2::Mmap;

use crate::color_solver::{contrast_ink, rgb_to_hsl, spanish_color_name, HslColor};
use crate::error::ManifestError;
use crate::format::{
    AppKeyEntryRecord, AudioAssetRecord, BodyLayoutRecord, CueRecord, ExperienceRecord,
    FeatureRecord, HcvbHeader, LandmarkRecord, MetricTokenRecord, MusicNoteRecord,
    MusicTimbreRecord, PhraseTriple, StrSlice, ZoneRecord, FORMAT_VERSION, MAGIC_AUD, MAGIC_CAT,
    NONE_ASSET,
};
use crate::music::{
    color_hue_for_midi, midi_for_x, midi_to_hz, octave_for_midi, pitch_class, timbre_index_for_y,
};

/// Backing byte buffer guaranteeing >= 8-byte alignment for zero-copy Pod slice casts.
enum Backing {
    Mmap(Mmap),
    Aligned { words: Vec<u64>, len: usize },
}

impl Backing {
    fn from_copied_bytes(bytes: &[u8]) -> Self {
        let word_count = bytes.len().div_ceil(8);
        let mut words = vec![0_u64; word_count];
        if !bytes.is_empty() {
            let dst: &mut [u8] = bytemuck::cast_slice_mut(&mut words);
            dst[..bytes.len()].copy_from_slice(bytes);
        }
        Self::Aligned {
            words,
            len: bytes.len(),
        }
    }
}

impl Deref for Backing {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        match self {
            Self::Mmap(map) => map.as_ref(),
            Self::Aligned { words, len } => {
                let bytes: &[u8] = bytemuck::cast_slice(words.as_slice());
                &bytes[..*len]
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct TableSpan {
    offset: usize,
    end: usize,
    count: usize,
}

impl TableSpan {
    fn slice<'a, T: Pod>(&self, backing: &'a [u8]) -> &'a [T] {
        if self.count == 0 {
            return &[];
        }
        // Validated once during `UniverseCatalog::validate_and_construct`.
        bytemuck::cast_slice(&backing[self.offset..self.end])
    }
}

#[derive(Copy, Clone, Debug)]
struct SectionLayout {
    experiences: TableSpan,
    zones: TableSpan,
    features: TableSpan,
    metric_tokens: TableSpan,
    key_entries: TableSpan,
    assets: TableSpan,
    cues: TableSpan,
    body_layout: TableSpan,
    landmarks: TableSpan,
    music_notes: TableSpan,
    music_timbres: TableSpan,
    strings_offset: usize,
    strings_end: usize,
}

/// Result of resolving an `(r, g, b)` point in App `01`.
#[derive(Clone, Debug, PartialEq)]
pub struct SolvedColor {
    pub name: String,
    pub hsl: HslColor,
    pub ink_rgb: [u8; 3],
    pub triple: PhraseTriple,
}

/// Result of resolving an `(x_norm, y_norm)` touch point in App `02`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SolvedMusicTouch<'a> {
    pub midi: u8,
    pub pitch_class: u8,
    pub octave: i32,
    pub freq_hz: f64,
    pub hue_deg: f32,
    pub note: &'a MusicNoteRecord,
    pub timbre: &'a MusicTimbreRecord,
    pub octave_triple: PhraseTriple,
}

/// Real-time highlight state within one spoken narration asset at `elapsed_ms`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CueProgress<'a> {
    pub active_cue_index: Option<usize>,
    pub active_cue: Option<&'a CueRecord>,
    pub revealed_end_byte: usize,
    pub revealed_end_scalar: usize,
    pub revealed_text: &'a str,
}

/// Zero-copy validated handle to `universe_catalog.hcvb` and optional `universe_audio.hcvp`.
pub struct UniverseCatalog {
    catalog_bytes: Backing,
    audio_bytes: Option<Backing>,
    header: HcvbHeader,
    layout: SectionLayout,
}

impl UniverseCatalog {
    /// Memory-maps `universe_catalog.hcvb` (and optional `universe_audio.hcvp`), verifies CRC32
    /// checksums, section bounds, UTF-8 validity, sorted key tables, and cue codepoint boundaries.
    pub fn open(
        hcvb_path: impl AsRef<Path>,
        hcvp_path: Option<&Path>,
    ) -> Result<Self, ManifestError> {
        let hcvb_file = File::open(hcvb_path.as_ref()).map_err(|source| ManifestError::Io {
            path: hcvb_path.as_ref().display().to_string(),
            source,
        })?;
        // SAFETY: Baked `.hcvb` catalog artifact is read-only and immutable during process lifetime.
        let cat_map = unsafe { Mmap::map(&hcvb_file) }.map_err(|source| ManifestError::Io {
            path: hcvb_path.as_ref().display().to_string(),
            source,
        })?;

        let audio_backing = match hcvp_path {
            Some(path) => {
                let hcvp_file = File::open(path).map_err(|source| ManifestError::Io {
                    path: path.display().to_string(),
                    source,
                })?;
                // SAFETY: Baked `.hcvp` audio pack is read-only and immutable during process lifetime.
                let aud_map = unsafe { Mmap::map(&hcvp_file) }.map_err(|source| ManifestError::Io {
                    path: path.display().to_string(),
                    source,
                })?;
                Some(Backing::Mmap(aud_map))
            }
            None => None,
        };

        Self::validate_and_construct(Backing::Mmap(cat_map), audio_backing)
    }

    /// Loads from in-memory slices (copying into 8-byte-aligned buffers) and runs full validation.
    pub fn from_bytes(hcvb: &[u8], hcvp: Option<&[u8]>) -> Result<Self, ManifestError> {
        let cat = Backing::from_copied_bytes(hcvb);
        let aud = hcvp.map(Backing::from_copied_bytes);
        Self::validate_and_construct(cat, aud)
    }

    fn validate_and_construct(
        catalog_bytes: Backing,
        audio_bytes: Option<Backing>,
    ) -> Result<Self, ManifestError> {
        let raw_cat: &[u8] = &catalog_bytes;
        let hdr_size = std::mem::size_of::<HcvbHeader>();
        if raw_cat.len() < hdr_size {
            return Err(ManifestError::TruncatedHeader {
                actual: raw_cat.len(),
                expected: hdr_size,
            });
        }

        let header: &HcvbHeader = bytemuck::try_from_bytes(&raw_cat[..hdr_size]).map_err(|e| {
            ManifestError::PodCastError {
                name: "HcvbHeader",
                reason: e.to_string(),
            }
        })?;

        if header.magic != MAGIC_CAT {
            return Err(ManifestError::BadCatalogMagic);
        }
        if header.version != FORMAT_VERSION {
            return Err(ManifestError::UnsupportedVersion(header.version));
        }

        let computed_cat_crc = crc32fast::hash(&raw_cat[hdr_size..]);
        if computed_cat_crc != header.catalog_crc32 {
            return Err(ManifestError::CatalogCrcMismatch {
                expected: header.catalog_crc32,
                computed: computed_cat_crc,
            });
        }

        if let Some(ref aud) = audio_bytes {
            let raw_aud: &[u8] = aud;
            if raw_aud.len() as u64 != header.audio_pack_size {
                return Err(ManifestError::AudioPackSizeMismatch {
                    expected: header.audio_pack_size,
                    actual: raw_aud.len(),
                });
            }
            if raw_aud.len() < 16 || raw_aud[..8] != MAGIC_AUD {
                return Err(ManifestError::BadAudioMagic);
            }
            let aud_ver = u32::from_le_bytes(raw_aud[8..12].try_into().unwrap());
            if aud_ver != FORMAT_VERSION {
                return Err(ManifestError::UnsupportedAudioVersion(aud_ver));
            }
            let computed_aud_crc = crc32fast::hash(raw_aud);
            if computed_aud_crc != header.audio_crc32 {
                return Err(ManifestError::AudioCrcMismatch {
                    expected: header.audio_crc32,
                    computed: computed_aud_crc,
                });
            }
        }

        let experiences =
            check_table::<ExperienceRecord>(raw_cat, "experiences", header.experiences_off, header.experiences_len)?;
        let zones = check_table::<ZoneRecord>(raw_cat, "zones", header.zones_off, header.zones_len)?;
        let features =
            check_table::<FeatureRecord>(raw_cat, "features", header.features_off, header.features_len)?;
        let metric_tokens = check_table::<MetricTokenRecord>(
            raw_cat,
            "metric_tokens",
            header.metric_tokens_off,
            header.metric_tokens_len,
        )?;
        let key_entries = check_table::<AppKeyEntryRecord>(
            raw_cat,
            "key_entries",
            header.key_entries_off,
            header.key_entries_len,
        )?;
        let assets =
            check_table::<AudioAssetRecord>(raw_cat, "assets", header.assets_off, header.assets_len)?;
        let cues = check_table::<CueRecord>(raw_cat, "cues", header.cues_off, header.cues_len)?;
        let body_layout = check_table::<BodyLayoutRecord>(
            raw_cat,
            "body_layout",
            header.body_layout_off,
            header.body_layout_len,
        )?;
        let landmarks =
            check_table::<LandmarkRecord>(raw_cat, "landmarks", header.landmarks_off, header.landmarks_len)?;
        let music_notes = check_table::<MusicNoteRecord>(
            raw_cat,
            "music_notes",
            header.music_notes_off,
            header.music_notes_len,
        )?;
        let music_timbres = check_table::<MusicTimbreRecord>(
            raw_cat,
            "music_timbres",
            header.music_timbres_off,
            header.music_timbres_len,
        )?;

        let strings_offset = header.strings_off as usize;
        let strings_end = strings_offset
            .checked_add(header.strings_len as usize)
            .filter(|&end| end <= raw_cat.len())
            .ok_or(ManifestError::SectionOutOfBounds {
                name: "strings",
                offset: strings_offset,
                byte_len: header.strings_len as usize,
                total: raw_cat.len(),
            })?;
        let arena = &raw_cat[strings_offset..strings_end];

        let exp_slice = experiences.slice::<ExperienceRecord>(raw_cat);
        let zone_slice = zones.slice::<ZoneRecord>(raw_cat);
        let feat_slice = features.slice::<FeatureRecord>(raw_cat);
        let mtok_slice = metric_tokens.slice::<MetricTokenRecord>(raw_cat);
        let key_slice = key_entries.slice::<AppKeyEntryRecord>(raw_cat);
        let asset_slice = assets.slice::<AudioAssetRecord>(raw_cat);
        let cue_slice = cues.slice::<CueRecord>(raw_cat);
        let body_slice = body_layout.slice::<BodyLayoutRecord>(raw_cat);
        let lm_slice = landmarks.slice::<LandmarkRecord>(raw_cat);
        let note_slice = music_notes.slice::<MusicNoteRecord>(raw_cat);
        let timbre_slice = music_timbres.slice::<MusicTimbreRecord>(raw_cat);

        // Validate every string slice, cue byte boundary, range reference, and sorted invariant.
        for (i, a) in asset_slice.iter().enumerate() {
            let text = validate_str(arena, a.text)?;
            validate_str(arena, a.external_id)?;

            let cue_end = validate_range("AudioAssetRecord", i, "cues", a.cue_start, a.cue_count, cue_slice.len())?;
            for (ci, cue) in cue_slice[a.cue_start as usize..cue_end].iter().enumerate() {
                let sb = cue.start_byte as usize;
                let eb = cue.end_byte as usize;
                if eb > text.len() || sb > eb || !text.is_char_boundary(sb) || !text.is_char_boundary(eb) {
                    return Err(ManifestError::CueBadUtf8Boundary {
                        asset_index: i,
                        cue_index: ci,
                        start: cue.start_byte,
                        end: cue.end_byte,
                    });
                }
            }

            let mp3_end = a.mp3_offset.checked_add(a.mp3_len as u64).ok_or(ManifestError::InvalidRange {
                kind: "AudioAssetRecord",
                index: i,
                field: "mp3",
                start: a.mp3_offset,
                end: u64::MAX,
                limit: header.audio_pack_size,
            })?;
            if mp3_end > header.audio_pack_size {
                return Err(ManifestError::InvalidRange {
                    kind: "AudioAssetRecord",
                    index: i,
                    field: "mp3",
                    start: a.mp3_offset,
                    end: mp3_end,
                    limit: header.audio_pack_size,
                });
            }
        }

        for (i, k) in key_slice.iter().enumerate() {
            validate_str(arena, k.key)?;
            validate_triple("AppKeyEntryRecord", i, k.triple, asset_slice.len())?;
        }
        for (i, f) in feat_slice.iter().enumerate() {
            validate_triple("FeatureRecord", i, f.name, asset_slice.len())?;
            validate_triple("FeatureRecord", i, f.fact, asset_slice.len())?;
        }
        for (i, m) in mtok_slice.iter().enumerate() {
            validate_triple("MetricTokenRecord", i, m.triple, asset_slice.len())?;
        }
        for (i, z) in zone_slice.iter().enumerate() {
            validate_str(arena, z.id)?;
            validate_str(arena, z.raw_metric)?;
            validate_str(arena, z.period)?;
            validate_str(arena, z.footer)?;
            validate_range("ZoneRecord", i, "features", z.feature_start, z.feature_count, feat_slice.len())?;
            validate_range(
                "ZoneRecord",
                i,
                "metric_tokens",
                z.metric_token_start,
                z.metric_token_count,
                mtok_slice.len(),
            )?;
            validate_triple("ZoneRecord", i, z.name, asset_slice.len())?;
            validate_triple("ZoneRecord", i, z.macro_fact, asset_slice.len())?;
            validate_triple("ZoneRecord", i, z.metric_narration, asset_slice.len())?;
            if z.evidence_class.is_present() {
                validate_triple("ZoneRecord", i, z.evidence_class, asset_slice.len())?;
            }
        }
        for (i, b) in body_slice.iter().enumerate() {
            validate_str(arena, b.zone_id)?;
            if b.zone_index as usize >= zone_slice.len() {
                return Err(ManifestError::InvalidRange {
                    kind: "BodyLayoutRecord",
                    index: i,
                    field: "zone_index",
                    start: b.zone_index as u64,
                    end: (b.zone_index as u64) + 1,
                    limit: zone_slice.len() as u64,
                });
            }
        }
        for (i, lm) in lm_slice.iter().enumerate() {
            validate_str(arena, lm.zone_id)?;
            if lm.app_num < 1 || lm.app_num as usize > exp_slice.len() {
                return Err(ManifestError::InvalidRange {
                    kind: "LandmarkRecord",
                    index: i,
                    field: "app_num",
                    start: lm.app_num as u64,
                    end: (lm.app_num as u64) + 1,
                    limit: exp_slice.len() as u64,
                });
            }
        }
        for (i, mn) in note_slice.iter().enumerate() {
            validate_str(arena, mn.symbol)?;
            validate_triple("MusicNoteRecord", i, mn.latin, asset_slice.len())?;
        }
        for (i, mt) in timbre_slice.iter().enumerate() {
            validate_triple("MusicTimbreRecord", i, mt.label, asset_slice.len())?;
        }

        for (i, exp) in exp_slice.iter().enumerate() {
            validate_str(arena, exp.id)?;
            validate_str(arena, exp.label)?;
            validate_str(arena, exp.title)?;
            validate_range("ExperienceRecord", i, "zones", exp.zone_start, exp.zone_count, zone_slice.len())?;
            let k_end = validate_range(
                "ExperienceRecord",
                i,
                "key_entries",
                exp.key_start,
                exp.key_count,
                key_slice.len(),
            )?;
            let slice = &key_slice[exp.key_start as usize..k_end];
            for w in slice.windows(2) {
                let s0 = str_from_validated_arena(arena, w[0].key);
                let s1 = str_from_validated_arena(arena, w[1].key);
                if s0.as_bytes() >= s1.as_bytes() {
                    return Err(ManifestError::UnsortedKeyTable { app_num: exp.app_num });
                }
            }
        }

        let header_copy = *header;
        let layout = SectionLayout {
            experiences,
            zones,
            features,
            metric_tokens,
            key_entries,
            assets,
            cues,
            body_layout,
            landmarks,
            music_notes,
            music_timbres,
            strings_offset,
            strings_end,
        };

        Ok(Self {
            catalog_bytes,
            audio_bytes,
            header: header_copy,
            layout,
        })
    }

    pub const fn header(&self) -> &HcvbHeader {
        &self.header
    }

    pub fn has_audio_pack(&self) -> bool {
        self.audio_bytes.is_some()
    }

    pub fn experiences(&self) -> &[ExperienceRecord] {
        self.layout.experiences.slice(&self.catalog_bytes)
    }

    pub fn experience(&self, app_num: u8) -> Option<&ExperienceRecord> {
        let idx = app_num.checked_sub(1)? as usize;
        self.experiences().get(idx)
    }

    pub fn all_zones(&self) -> &[ZoneRecord] {
        self.layout.zones.slice(&self.catalog_bytes)
    }

    pub fn zones(&self, app_num: u8) -> &[ZoneRecord] {
        let Some(exp) = self.experience(app_num) else {
            return &[];
        };
        let start = exp.zone_start as usize;
        let end = start + exp.zone_count as usize;
        &self.all_zones()[start..end]
    }

    pub fn zone_at_progress(&self, app_num: u8, p_norm: f32) -> Option<&ZoneRecord> {
        let p = p_norm.clamp(0.0, 0.999_999);
        let zs = self.zones(app_num);
        zs.iter()
            .find(|z| p >= z.range_start && p < z.range_end)
            .or_else(|| zs.last())
    }

    pub fn all_features(&self) -> &[FeatureRecord] {
        self.layout.features.slice(&self.catalog_bytes)
    }

    pub fn features(&self, zone: &ZoneRecord) -> &[FeatureRecord] {
        let start = zone.feature_start as usize;
        let end = start + zone.feature_count as usize;
        &self.all_features()[start..end]
    }

    pub fn feature_at(&self, zone: &ZoneRecord, cross_norm: f32) -> Option<(usize, &FeatureRecord)> {
        let feats = self.features(zone);
        if feats.is_empty() {
            return None;
        }
        let clamped = cross_norm.clamp(0.0, 0.999_999);
        let idx = ((clamped * feats.len() as f32).floor() as usize).min(feats.len() - 1);
        Some((idx, &feats[idx]))
    }

    pub fn all_metric_tokens(&self) -> &[MetricTokenRecord] {
        self.layout.metric_tokens.slice(&self.catalog_bytes)
    }

    pub fn metric_tokens(&self, zone: &ZoneRecord) -> &[MetricTokenRecord] {
        let start = zone.metric_token_start as usize;
        let end = start + zone.metric_token_count as usize;
        &self.all_metric_tokens()[start..end]
    }

    pub fn all_key_entries(&self) -> &[AppKeyEntryRecord] {
        self.layout.key_entries.slice(&self.catalog_bytes)
    }

    pub fn app_key_entries(&self, app_num: u8) -> &[AppKeyEntryRecord] {
        let Some(exp) = self.experience(app_num) else {
            return &[];
        };
        let start = exp.key_start as usize;
        let end = start + exp.key_count as usize;
        &self.all_key_entries()[start..end]
    }

    /// `O(log K)` binary search inside experience `app_num`'s sorted UTF-8 key table.
    pub fn find_key_triple(&self, app_num: u8, source_key: &str) -> Option<PhraseTriple> {
        let entries = self.app_key_entries(app_num);
        let target = source_key.as_bytes();
        entries
            .binary_search_by(|entry| self.str_slice(entry.key).as_bytes().cmp(target))
            .ok()
            .map(|idx| entries[idx].triple)
    }

    pub fn audio_assets(&self) -> &[AudioAssetRecord] {
        self.layout.assets.slice(&self.catalog_bytes)
    }

    pub fn audio_asset(&self, asset_idx: u32) -> Option<&AudioAssetRecord> {
        if asset_idx == NONE_ASSET {
            return None;
        }
        self.audio_assets().get(asset_idx as usize)
    }

    pub fn all_cues(&self) -> &[CueRecord] {
        self.layout.cues.slice(&self.catalog_bytes)
    }

    pub fn asset_cues(&self, asset: &AudioAssetRecord) -> &[CueRecord] {
        let start = asset.cue_start as usize;
        let end = start + asset.cue_count as usize;
        &self.all_cues()[start..end]
    }

    pub fn asset_text(&self, asset: &AudioAssetRecord) -> &str {
        self.str_slice(asset.text)
    }

    pub fn asset_external_id(&self, asset: &AudioAssetRecord) -> &str {
        self.str_slice(asset.external_id)
    }

    pub fn cue_slice<'a>(&'a self, asset: &AudioAssetRecord, cue: &CueRecord) -> &'a str {
        let text = self.asset_text(asset);
        &text[cue.start_byte as usize..cue.end_byte as usize]
    }

    /// Computes active cue & UTF-8-safe revealed caption substring at `elapsed_ms`.
    pub fn cue_progress_at_ms<'a>(
        &'a self,
        asset: &AudioAssetRecord,
        elapsed_ms: u32,
    ) -> CueProgress<'a> {
        let text = self.asset_text(asset);
        let cues = self.asset_cues(asset);
        if cues.is_empty() {
            return CueProgress {
                active_cue_index: None,
                active_cue: None,
                revealed_end_byte: text.len(),
                revealed_end_scalar: text.chars().count(),
                revealed_text: text,
            };
        }

        let mut active_idx = None;
        let mut revealed_end_byte = 0usize;
        let mut revealed_end_scalar = 0usize;
        for (i, cue) in cues.iter().enumerate() {
            if elapsed_ms >= cue.start_ms {
                revealed_end_byte = cue.end_byte as usize;
                revealed_end_scalar = cue.end_scalar as usize;
                if elapsed_ms < cue.end_ms {
                    active_idx = Some(i);
                }
            } else {
                break;
            }
        }

        if let Some(last) = cues.last()
            && elapsed_ms >= last.end_ms
        {
            revealed_end_byte = text.len();
            revealed_end_scalar = last.end_scalar as usize;
        }

        CueProgress {
            active_cue_index: active_idx,
            active_cue: active_idx.map(|i| &cues[i]),
            revealed_end_byte,
            revealed_end_scalar,
            revealed_text: &text[..revealed_end_byte],
        }
    }

    /// Returns the raw zero-copy MP3 byte slice for `asset` from mmap'd `universe_audio.hcvp`.
    pub fn mp3_slice(&self, asset: &AudioAssetRecord) -> Option<&[u8]> {
        let aud = self.audio_bytes.as_deref()?;
        let start = asset.mp3_offset as usize;
        let end = start.checked_add(asset.mp3_len as usize)?;
        aud.get(start..end)
    }

    pub fn body_layout(&self) -> &[BodyLayoutRecord] {
        self.layout.body_layout.slice(&self.catalog_bytes)
    }

    pub fn all_landmarks(&self) -> &[LandmarkRecord] {
        self.layout.landmarks.slice(&self.catalog_bytes)
    }

    pub fn landmarks(&self, app_num: u8) -> impl Iterator<Item = &LandmarkRecord> {
        self.all_landmarks().iter().filter(move |lm| lm.app_num == app_num)
    }

    pub fn music_notes(&self) -> &[MusicNoteRecord] {
        self.layout.music_notes.slice(&self.catalog_bytes)
    }

    pub fn music_timbres(&self) -> &[MusicTimbreRecord] {
        self.layout.music_timbres.slice(&self.catalog_bytes)
    }

    /// Resolves any `(r, g, b)` point for App `01` (`Colores`), including its bilingual lexicon triple.
    pub fn solve_color(&self, r: u8, g: u8, b: u8) -> SolvedColor {
        let name = spanish_color_name(r, g, b);
        let hsl = rgb_to_hsl(r, g, b);
        let ink_rgb = contrast_ink(r, g, b);
        let triple = self.find_key_triple(1, &name).unwrap_or(PhraseTriple::NONE);
        SolvedColor {
            name,
            hsl,
            ink_rgb,
            triple,
        }
    }

    /// Resolves `(x_norm, y_norm)` touch coordinates for App `02` (`Música`) into synthesis + narration metadata.
    pub fn solve_music_touch(&self, x_norm: f32, y_norm: f32) -> SolvedMusicTouch<'_> {
        let midi = midi_for_x(x_norm);
        let pc = pitch_class(midi);
        let octave = octave_for_midi(midi);
        let freq_hz = midi_to_hz(midi);
        let hue_deg = color_hue_for_midi(midi);
        let note = &self.music_notes()[pc as usize];
        let t_idx = timbre_index_for_y(y_norm);
        let timbre = &self.music_timbres()[t_idx];
        let oct_key = format!("octava {octave}");
        let octave_triple = self.find_key_triple(2, &oct_key).unwrap_or(PhraseTriple::NONE);
        SolvedMusicTouch {
            midi,
            pitch_class: pc,
            octave,
            freq_hz,
            hue_deg,
            note,
            timbre,
            octave_triple,
        }
    }

    /// Returns a UTF-8 `&str` for any `StrSlice` stored inside the catalog.
    pub fn str_slice(&self, s: StrSlice) -> &str {
        let arena = &self.catalog_bytes[self.layout.strings_offset..self.layout.strings_end];
        str_from_validated_arena(arena, s)
    }
}

fn check_table<T: Pod>(
    raw: &[u8],
    name: &'static str,
    offset: u32,
    count: u32,
) -> Result<TableSpan, ManifestError> {
    let offset = offset as usize;
    let count = count as usize;
    let byte_len = count
        .checked_mul(std::mem::size_of::<T>())
        .ok_or(ManifestError::SectionOutOfBounds {
            name,
            offset,
            byte_len: usize::MAX,
            total: raw.len(),
        })?;
    let end = offset.checked_add(byte_len).ok_or(ManifestError::SectionOutOfBounds {
        name,
        offset,
        byte_len,
        total: raw.len(),
    })?;
    if end > raw.len() {
        return Err(ManifestError::SectionOutOfBounds {
            name,
            offset,
            byte_len,
            total: raw.len(),
        });
    }
    if count > 0 {
        bytemuck::try_cast_slice::<u8, T>(&raw[offset..end]).map_err(|e| {
            ManifestError::PodCastError {
                name,
                reason: e.to_string(),
            }
        })?;
    }
    Ok(TableSpan { offset, end, count })
}

fn validate_str(arena: &[u8], s: StrSlice) -> Result<&str, ManifestError> {
    let off = s.byte_off as usize;
    let end = off
        .checked_add(s.byte_len as usize)
        .ok_or(ManifestError::StringOutOfBounds {
            off,
            end: usize::MAX,
            arena_len: arena.len(),
        })?;
    if end > arena.len() {
        return Err(ManifestError::StringOutOfBounds {
            off,
            end,
            arena_len: arena.len(),
        });
    }
    std::str::from_utf8(&arena[off..end]).map_err(|_| ManifestError::InvalidUtf8 { off, end })
}

fn str_from_validated_arena(arena: &[u8], s: StrSlice) -> &str {
    let off = s.byte_off as usize;
    let end = off + s.byte_len as usize;
    // Validated during `UniverseCatalog::validate_and_construct`.
    std::str::from_utf8(&arena[off..end]).expect("validated UTF-8 string arena slice")
}

fn validate_range(
    kind: &'static str,
    index: usize,
    field: &'static str,
    start: u32,
    count: u32,
    limit: usize,
) -> Result<usize, ManifestError> {
    let start_u64 = start as u64;
    let end_u64 = start_u64 + count as u64;
    if end_u64 > limit as u64 {
        return Err(ManifestError::InvalidRange {
            kind,
            index,
            field,
            start: start_u64,
            end: end_u64,
            limit: limit as u64,
        });
    }
    Ok(end_u64 as usize)
}

fn validate_triple(
    kind: &'static str,
    index: usize,
    triple: PhraseTriple,
    asset_len: usize,
) -> Result<(), ManifestError> {
    for idx in [triple.es_asset, triple.en_asset, triple.fi_asset] {
        if idx == NONE_ASSET || idx as usize >= asset_len {
            return Err(ManifestError::InvalidRange {
                kind,
                index,
                field: "triple_asset",
                start: idx as u64,
                end: (idx as u64) + 1,
                limit: asset_len as u64,
            });
        }
    }
    Ok(())
}
