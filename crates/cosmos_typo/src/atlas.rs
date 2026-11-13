use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use ab_glyph::{Font as AbFont, FontVec};
use bytemuck::{Pod, Zeroable};
use fdsm::bezier::scanline::FillRule;
use fdsm::correct_error::{ErrorCorrectionConfig, correct_error_mtsdf};
use fdsm::generate::generate_mtsdf;
use fdsm::render::correct_sign_mtsdf;
use fdsm::shape::Shape;
use fdsm::transform::Transform;
use image::Rgba32FImage;
use nalgebra as na;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use ttf_parser::Face;

use crate::error::TypoError;

const CACHE_MAGIC: [u8; 8] = *b"HCMSDF01";
const CACHE_VERSION: u32 = 1;
pub const CACHE_BIN_FILENAME: &str = "msdf_atlas_v1.bin";
pub const CACHE_JSON_FILENAME: &str = "msdf_atlas_v1.json";

/// Required Spanish, Finnish, math, and catalog non-ASCII coverage (beyond printable ASCII).
pub const REQUIRED_EXTENDED_CHARS: &str =
    "áéíóúüñÁÉÍÓÚÜÑ¿¡äöåÄÖÅšΩ♯⁻²³⁵°·→—×«»¹÷–‘’“”⁰⁴⁶⁸₂↔−≈≠";

/// Optional cosmic/music/science symbols baked whenever present in the font fallback chain.
pub const OPTIONAL_COSMIC_CHARS: &str =
    "♭♮±∞µαβγδεζηθικλμνξοπρστυφχψωΣΔΦΨ•…←⁺₀₁₃₄₅₆₇₈₉≤≥";

/// Logical typography role selecting the primary font and fallback chain.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum FontRole {
    /// Expressive calligraphic display script (`display_script_bold.ttf`) for live-ink narration & callouts.
    DisplayScript = 0,
    /// Geometric crisp sans (`ui_sans_bold.ttf`) for landmarks, region headings, metrics & eyebrow tags.
    UiSans = 1,
}

impl FontRole {
    pub const ALL: [Self; 2] = [Self::DisplayScript, Self::UiSans];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisplayScript => "display_script",
            Self::UiSans => "ui_sans",
        }
    }

    const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::DisplayScript),
            1 => Some(Self::UiSans),
            _ => None,
        }
    }
}

/// Physical `.ttf` asset that supplied a glyph's vector outline and metrics.
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum FontSource {
    DisplayScriptBold = 0,
    UiSansBold = 1,
    SymbolsFallback = 2,
}

impl FontSource {
    const fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(Self::DisplayScriptBold),
            1 => Some(Self::UiSansBold),
            2 => Some(Self::SymbolsFallback),
            _ => None,
        }
    }
}

/// Configuration for parallel MSDF atlas baking & disk cache validation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MsdfBakeConfig {
    pub atlas_width: u32,
    pub atlas_height: u32,
    /// Pixels per `1.0 em` in the baked atlas (e.g., `64.0`).
    pub em_px: f32,
    /// Full signed-distance range in atlas pixels (`[-range/2, +range/2]`, e.g., `8.0`).
    pub px_range: f32,
    /// Tile padding around the glyph tight bounding box in pixels (must exceed `px_range * 0.5`).
    pub pad_px: u32,
    /// Empty zeroed gutter between packed glyph tiles in pixels.
    pub gutter_px: u32,
    /// Required characters that must resolve in the fallback chain for every [`FontRole`].
    pub required_charset: String,
    /// Optional characters included whenever found in the font fallback chain.
    pub optional_charset: String,
}

impl Default for MsdfBakeConfig {
    fn default() -> Self {
        Self {
            atlas_width: 2048,
            atlas_height: 2048,
            em_px: 64.0,
            px_range: 8.0,
            pad_px: 6,
            gutter_px: 1,
            required_charset: default_required_charset(),
            optional_charset: OPTIONAL_COSMIC_CHARS.to_string(),
        }
    }
}

/// Returns printable ASCII (`0x20..=0x7E`) + NBSP + Spanish + Finnish + cosmic/catalog required chars.
pub fn default_required_charset() -> String {
    let mut chars = BTreeSet::new();
    for code in 0x20u32..=0x7E {
        if let Some(ch) = char::from_u32(code) {
            chars.insert(ch);
        }
    }
    chars.insert('\u{00A0}');
    for ch in REQUIRED_EXTENDED_CHARS.chars() {
        chars.insert(ch);
    }
    chars.into_iter().collect()
}

/// Line-layout metrics for one [`FontRole`] normalized to `1.0 em`.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FontLineMetrics {
    pub ascent_em: f32,
    pub descent_em: f32,
    pub line_gap_em: f32,
    pub line_height_em: f32,
    pub cap_height_em: f32,
    pub x_height_em: f32,
}

/// Baked glyph geometry, atlas UV rectangle, and advance metrics for one `(FontRole, char)`.
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GlyphMetrics {
    pub codepoint: u32,
    pub role: FontRole,
    pub source_font: FontSource,
    pub has_outline: bool,
    /// Horizontal cursor advance in `em` units.
    pub advance_em: f32,
    /// MSDF distance range in atlas pixels (`right_vec.w`).
    pub px_range: f32,
    /// `[left_em, bottom_em, right_em, top_em]` relative to baseline origin `(0, 0)` with `+Y` up.
    pub plane_bounds: [f32; 4],
    /// `[u_left, v_top, u_right, v_bottom]` in GPU top-down texture space (`v=0` top, `v=1` bottom).
    pub uv_rect: [f32; 4],
    /// `[x, y, width, height]` inside the `Rgba8Unorm` atlas texture (top-down coordinates).
    pub pixel_rect: [u32; 4],
}

/// One positioned glyph inside a shaped text run.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShapedGlyph {
    pub ch: char,
    pub byte_start: usize,
    pub byte_end: usize,
    pub scalar_index: usize,
    pub line_index: usize,
    pub x_em: f32,
    pub y_em: f32,
    pub metrics: GlyphMetrics,
}

/// One wrapped line inside a [`ShapedText`].
#[derive(Clone, Debug, PartialEq)]
pub struct ShapedLine {
    pub width_em: f32,
    pub glyph_start: usize,
    pub glyph_end: usize,
}

/// Result of shaping & optional word-wrapping a UTF-8 string in `em` space.
#[derive(Clone, Debug, PartialEq)]
pub struct ShapedText {
    pub role: FontRole,
    pub glyphs: Vec<ShapedGlyph>,
    pub lines: Vec<ShapedLine>,
    pub total_scalars: usize,
    pub max_width_em: f32,
    pub total_height_em: f32,
    pub ascent_em: f32,
    pub line_height_em: f32,
}

#[derive(Clone, Debug, Deserialize)]
struct FontManifest {
    #[serde(default)]
    fonts: Vec<FontManifestEntry>,
}

#[derive(Clone, Debug, Deserialize)]
struct FontManifestEntry {
    role: String,
    file: String,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CacheHeaderPod {
    magic: [u8; 8],
    version: u32,
    width: u32,
    height: u32,
    px_range: f32,
    em_px: f32,
    pad_px: u32,
    gutter_px: u32,
    _pad0: u32,
    fonts_fingerprint: u64,
    font_metrics_count: u32,
    glyph_count: u32,
    kern_count: u32,
    payload_crc32: u32,
    _reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct FontLineMetricsPod {
    role: u32,
    ascent_em: f32,
    descent_em: f32,
    line_gap_em: f32,
    line_height_em: f32,
    cap_height_em: f32,
    x_height_em: f32,
    _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CachedGlyphPod {
    codepoint: u32,
    role: u8,
    source_font: u8,
    has_outline: u8,
    _pad: u8,
    advance_em: f32,
    px_range: f32,
    plane_bounds: [f32; 4],
    uv_rect: [f32; 4],
    pixel_rect: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CachedKernPod {
    left: u32,
    right: u32,
    role: u8,
    _pad: [u8; 3],
    kern_em: f32,
}

#[derive(Clone, Debug, Serialize)]
struct AtlasJsonManifest {
    version: u32,
    width: u32,
    height: u32,
    em_px: f32,
    px_range: f32,
    pad_px: u32,
    gutter_px: u32,
    fonts_fingerprint: String,
    font_metrics: Vec<(String, FontLineMetrics)>,
    glyphs: Vec<GlyphMetrics>,
}

/// Combined dual-font `2048x2048` `Rgba8Unorm` MSDF/MTSDF atlas (`.rgb` MSDF, `.a` true SDF).
#[derive(Clone, Debug)]
pub struct MsdfAtlas {
    width: u32,
    height: u32,
    em_px: f32,
    px_range: f32,
    pad_px: u32,
    gutter_px: u32,
    fonts_fingerprint: u64,
    font_metrics: [FontLineMetrics; 2],
    glyphs: HashMap<(FontRole, char), GlyphMetrics>,
    /// Sorted by [`pack_kern_key`] for allocation-free binary search and sub-millisecond cache loads.
    kerns: Vec<(u64, f32)>,
    pixels_rgba8: Vec<u8>,
}

#[inline]
const fn pack_kern_key(role: FontRole, left: char, right: char) -> u64 {
    ((role as u64) << 42) | ((left as u64) << 21) | (right as u64)
}

struct BakedTile {
    ch: char,
    role: FontRole,
    source_font: FontSource,
    has_outline: bool,
    advance_em: f32,
    plane_bounds: [f32; 4],
    tile_w: u32,
    tile_h: u32,
    pixels_rgba8: Vec<u8>,
}

struct FontData<'a> {
    face: Face<'a>,
    ab_font: FontVec,
    units_per_em: f64,
}

impl MsdfAtlas {
    /// Loads the cached dual-font MSDF atlas from `cache_dir` in `< 30 ms`, or bakes it in parallel
    /// across `rayon` threads from `fonts_dir` and persists the binary + JSON cache if missing or stale.
    pub fn load_or_bake(
        fonts_dir: impl AsRef<Path>,
        cache_dir: impl AsRef<Path>,
        config: &MsdfBakeConfig,
    ) -> Result<Self, TypoError> {
        let fonts_dir = fonts_dir.as_ref();
        let cache_dir = cache_dir.as_ref();
        let (script_bytes, sans_bytes, sym_bytes) = load_font_files(fonts_dir)?;
        let fingerprint = compute_bake_fingerprint(&script_bytes, &sans_bytes, &sym_bytes, config);

        if let Ok(atlas) = Self::load_from_cache(cache_dir, Some(fingerprint))
            && atlas.width == config.atlas_width
            && atlas.height == config.atlas_height
            && (atlas.px_range - config.px_range).abs() < 1e-4
            && (atlas.em_px - config.em_px).abs() < 1e-4
        {
            return Ok(atlas);
        }

        let atlas = Self::bake_from_font_bytes_with_fingerprint(
            &script_bytes,
            &sans_bytes,
            &sym_bytes,
            config,
            fingerprint,
        )?;
        atlas.save_cache(cache_dir)?;
        Ok(atlas)
    }

    /// Loads an existing binary MSDF atlas cache from `cache_dir` without touching `.ttf` files.
    pub fn load_from_cache(
        cache_dir: impl AsRef<Path>,
        expected_fingerprint: Option<u64>,
    ) -> Result<Self, TypoError> {
        let bin_path = cache_dir.as_ref().join(CACHE_BIN_FILENAME);
        let mut file = fs::File::open(&bin_path).map_err(|source| TypoError::Io {
            path: bin_path.display().to_string(),
            source,
        })?;
        let file_len = file
            .metadata()
            .map_err(|source| TypoError::Io {
                path: bin_path.display().to_string(),
                source,
            })?
            .len() as usize;

        let hdr_size = std::mem::size_of::<CacheHeaderPod>();
        if file_len < hdr_size {
            return Err(TypoError::InvalidCache {
                reason: "truncated cache header".into(),
            });
        }

        let mut hdr_buf = vec![0u8; hdr_size];
        file.read_exact(&mut hdr_buf).map_err(|source| TypoError::Io {
            path: bin_path.display().to_string(),
            source,
        })?;
        let header = parse_and_validate_header(&hdr_buf, expected_fingerprint)?;
        let layout = CacheSectionLayout::from_header(&header, hdr_size, file_len)?;

        let meta_len = layout.fm_bytes + layout.gl_bytes + layout.kn_bytes;
        let mut meta_buf = Vec::with_capacity(meta_len);
        (&mut file)
            .take(meta_len as u64)
            .read_to_end(&mut meta_buf)
            .map_err(|source| TypoError::Io {
                path: bin_path.display().to_string(),
                source,
            })?;
        if meta_buf.len() != meta_len {
            return Err(TypoError::InvalidCache {
                reason: "truncated cache metadata".into(),
            });
        }

        let mut pixels_rgba8 = Vec::with_capacity(layout.px_bytes);
        file.take(layout.px_bytes as u64)
            .read_to_end(&mut pixels_rgba8)
            .map_err(|source| TypoError::Io {
                path: bin_path.display().to_string(),
                source,
            })?;
        if pixels_rgba8.len() != layout.px_bytes {
            return Err(TypoError::InvalidCache {
                reason: "truncated cache pixel payload".into(),
            });
        }

        let mut hasher = crc32fast::Hasher::new();
        hasher.update(&meta_buf);
        hasher.update(&pixels_rgba8);
        let computed_crc = hasher.finalize();
        if computed_crc != header.payload_crc32 {
            return Err(TypoError::InvalidCache {
                reason: format!(
                    "payload CRC32 mismatch (expected {:#010x}, got {:#010x})",
                    header.payload_crc32, computed_crc
                ),
            });
        }

        let fm_slice = &meta_buf[..layout.fm_bytes];
        let gl_slice = &meta_buf[layout.fm_bytes..layout.fm_bytes + layout.gl_bytes];
        let kn_slice = &meta_buf[layout.fm_bytes + layout.gl_bytes..];
        build_atlas_from_sections(&header, fm_slice, gl_slice, kn_slice, pixels_rgba8)
    }

    /// Parses a binary cache buffer (CRC32-checked).
    pub fn from_cache_bytes(
        raw: &[u8],
        expected_fingerprint: Option<u64>,
    ) -> Result<Self, TypoError> {
        let hdr_size = std::mem::size_of::<CacheHeaderPod>();
        if raw.len() < hdr_size {
            return Err(TypoError::InvalidCache {
                reason: "truncated cache header".into(),
            });
        }

        let header = parse_and_validate_header(&raw[..hdr_size], expected_fingerprint)?;
        let layout = CacheSectionLayout::from_header(&header, hdr_size, raw.len())?;

        let computed_crc = crc32fast::hash(&raw[hdr_size..]);
        if computed_crc != header.payload_crc32 {
            return Err(TypoError::InvalidCache {
                reason: format!(
                    "payload CRC32 mismatch (expected {:#010x}, got {:#010x})",
                    header.payload_crc32, computed_crc
                ),
            });
        }

        let mut offset = hdr_size;
        let fm_slice = &raw[offset..offset + layout.fm_bytes];
        offset += layout.fm_bytes;
        let gl_slice = &raw[offset..offset + layout.gl_bytes];
        offset += layout.gl_bytes;
        let kn_slice = &raw[offset..offset + layout.kn_bytes];
        offset += layout.kn_bytes;
        let px_slice = &raw[offset..offset + layout.px_bytes];

        build_atlas_from_sections(&header, fm_slice, gl_slice, kn_slice, px_slice.to_vec())
    }

    /// Forces a fresh parallel bake from `fonts_dir` and saves both binary + JSON caches under `cache_dir`.
    pub fn bake_and_cache(
        fonts_dir: impl AsRef<Path>,
        cache_dir: impl AsRef<Path>,
        config: &MsdfBakeConfig,
    ) -> Result<Self, TypoError> {
        let (script_bytes, sans_bytes, sym_bytes) = load_font_files(fonts_dir.as_ref())?;
        let fingerprint = compute_bake_fingerprint(&script_bytes, &sans_bytes, &sym_bytes, config);
        let atlas = Self::bake_from_font_bytes_with_fingerprint(
            &script_bytes,
            &sans_bytes,
            &sym_bytes,
            config,
            fingerprint,
        )?;
        atlas.save_cache(cache_dir)?;
        Ok(atlas)
    }

    /// Bakes a dual-font `Rgba8Unorm` MSDF atlas in parallel across `rayon` threads from raw `.ttf` bytes.
    pub fn bake_from_font_bytes(
        display_script_bold: &[u8],
        ui_sans_bold: &[u8],
        symbols_fallback: &[u8],
        config: &MsdfBakeConfig,
    ) -> Result<Self, TypoError> {
        let fingerprint =
            compute_bake_fingerprint(display_script_bold, ui_sans_bold, symbols_fallback, config);
        Self::bake_from_font_bytes_with_fingerprint(
            display_script_bold,
            ui_sans_bold,
            symbols_fallback,
            config,
            fingerprint,
        )
    }

    fn bake_from_font_bytes_with_fingerprint(
        display_script_bold: &[u8],
        ui_sans_bold: &[u8],
        symbols_fallback: &[u8],
        config: &MsdfBakeConfig,
        fonts_fingerprint: u64,
    ) -> Result<Self, TypoError> {
        let fonts = [
            parse_font_data("display_script_bold.ttf", display_script_bold)?,
            parse_font_data("ui_sans_bold.ttf", ui_sans_bold)?,
            parse_font_data("symbols_fallback.ttf", symbols_fallback)?,
        ];

        let font_metrics = [
            extract_line_metrics(&fonts[FontSource::DisplayScriptBold as usize]),
            extract_line_metrics(&fonts[FontSource::UiSansBold as usize]),
        ];

        let mut required_set = BTreeSet::new();
        for ch in config.required_charset.chars() {
            required_set.insert(ch);
        }
        let mut all_chars: Vec<(char, bool)> = required_set.iter().map(|&c| (c, true)).collect();
        for ch in config.optional_charset.chars() {
            if !required_set.contains(&ch) {
                required_set.insert(ch);
                all_chars.push((ch, false));
            }
        }

        let mut jobs = Vec::with_capacity(all_chars.len() * 2);
        for role in FontRole::ALL {
            for &(ch, required) in &all_chars {
                jobs.push((role, ch, required));
            }
        }

        let baked_results: Vec<Result<Option<BakedTile>, TypoError>> = jobs
            .par_iter()
            .map(|&(role, ch, required)| bake_single_glyph(&fonts, role, ch, required, config))
            .collect();

        let mut tiles = Vec::with_capacity(jobs.len());
        for res in baked_results {
            if let Some(tile) = res? {
                tiles.push(tile);
            }
        }

        // Deterministic packing order: tallest first, then widest, then role/codepoint.
        tiles.sort_by(|a, b| {
            b.tile_h
                .cmp(&a.tile_h)
                .then_with(|| b.tile_w.cmp(&a.tile_w))
                .then_with(|| a.role.cmp(&b.role))
                .then_with(|| a.ch.cmp(&b.ch))
        });

        let mut pixels_rgba8 = vec![0u8; (config.atlas_width * config.atlas_height * 4) as usize];
        let mut glyphs = HashMap::with_capacity(tiles.len());

        let mut cursor_x = config.gutter_px;
        let mut cursor_y = config.gutter_px;
        let mut shelf_h = 0u32;

        for tile in &tiles {
            if !tile.has_outline || tile.tile_w == 0 || tile.tile_h == 0 {
                glyphs.insert(
                    (tile.role, tile.ch),
                    GlyphMetrics {
                        codepoint: tile.ch as u32,
                        role: tile.role,
                        source_font: tile.source_font,
                        has_outline: false,
                        advance_em: tile.advance_em,
                        px_range: config.px_range,
                        plane_bounds: [0.0; 4],
                        uv_rect: [0.0; 4],
                        pixel_rect: [0; 4],
                    },
                );
                continue;
            }

            if cursor_x + tile.tile_w + config.gutter_px > config.atlas_width {
                cursor_x = config.gutter_px;
                cursor_y += shelf_h + config.gutter_px;
                shelf_h = 0;
            }
            if cursor_y + tile.tile_h + config.gutter_px > config.atlas_height {
                return Err(TypoError::AtlasOverflow {
                    width: config.atlas_width,
                    height: config.atlas_height,
                    count: tiles.len(),
                });
            }

            let dst_x = cursor_x;
            let dst_y = cursor_y;
            for row in 0..tile.tile_h {
                let src_off = (row * tile.tile_w * 4) as usize;
                let dst_off = (((dst_y + row) * config.atlas_width + dst_x) * 4) as usize;
                let row_bytes = (tile.tile_w * 4) as usize;
                pixels_rgba8[dst_off..dst_off + row_bytes]
                    .copy_from_slice(&tile.pixels_rgba8[src_off..src_off + row_bytes]);
            }

            let inv_w = 1.0 / config.atlas_width as f32;
            let inv_h = 1.0 / config.atlas_height as f32;
            let uv_rect = [
                dst_x as f32 * inv_w,
                dst_y as f32 * inv_h,
                (dst_x + tile.tile_w) as f32 * inv_w,
                (dst_y + tile.tile_h) as f32 * inv_h,
            ];

            glyphs.insert(
                (tile.role, tile.ch),
                GlyphMetrics {
                    codepoint: tile.ch as u32,
                    role: tile.role,
                    source_font: tile.source_font,
                    has_outline: true,
                    advance_em: tile.advance_em,
                    px_range: config.px_range,
                    plane_bounds: tile.plane_bounds,
                    uv_rect,
                    pixel_rect: [dst_x, dst_y, tile.tile_w, tile.tile_h],
                },
            );

            cursor_x += tile.tile_w + config.gutter_px;
            shelf_h = shelf_h.max(tile.tile_h);
        }

        // Precompute non-zero kerning pairs across baked characters using ab_glyph + ttf_parser.
        let role_chars: Vec<(FontRole, Vec<(char, FontSource)>)> = FontRole::ALL
            .iter()
            .map(|&role| {
                let mut list: Vec<(char, FontSource)> = glyphs
                    .values()
                    .filter(|g| g.role == role)
                    .filter_map(|g| char::from_u32(g.codepoint).map(|c| (c, g.source_font)))
                    .collect();
                list.sort_by_key(|&(c, _)| c);
                (role, list)
            })
            .collect();

        let kern_entries: Vec<(FontRole, char, char, f32)> = role_chars
            .par_iter()
            .flat_map_iter(|(role, list)| {
                let mut pairs = Vec::new();
                for &(c1, s1) in list {
                    let f1 = &fonts[s1 as usize];
                    let g1_ab = f1.ab_font.glyph_id(c1);
                    let g1_tt = f1.face.glyph_index(c1);
                    for &(c2, s2) in list {
                        if s1 != s2 {
                            continue;
                        }
                        let g2_ab = f1.ab_font.glyph_id(c2);
                        let mut k = f1.ab_font.kern_unscaled(g1_ab, g2_ab) as f64;
                        if k == 0.0
                            && let (Some(t1), Some(t2)) = (g1_tt, f1.face.glyph_index(c2))
                            && let Some(kern_tbl) = f1.face.tables().kern
                        {
                            for sub in kern_tbl.subtables {
                                if sub.horizontal
                                    && !sub.variable
                                    && let Some(v) = sub.glyphs_kerning(t1, t2)
                                {
                                    k += v as f64;
                                }
                            }
                        }
                        if k.abs() > 1e-3 {
                            pairs.push((*role, c1, c2, (k / f1.units_per_em) as f32));
                        }
                    }
                }
                pairs
            })
            .collect();

        let mut kerns: Vec<(u64, f32)> = kern_entries
            .into_iter()
            .map(|(role, c1, c2, k)| (pack_kern_key(role, c1, c2), k))
            .collect();
        kerns.sort_unstable_by_key(|&(k, _)| k);

        Ok(Self {
            width: config.atlas_width,
            height: config.atlas_height,
            em_px: config.em_px,
            px_range: config.px_range,
            pad_px: config.pad_px,
            gutter_px: config.gutter_px,
            fonts_fingerprint,
            font_metrics,
            glyphs,
            kerns,
            pixels_rgba8,
        })
    }

    /// Saves both the fast binary atlas (`msdf_atlas_v1.bin`) and JSON metrics (`msdf_atlas_v1.json`)
    /// under `cache_dir`.
    pub fn save_cache(&self, cache_dir: impl AsRef<Path>) -> Result<(), TypoError> {
        let dir = cache_dir.as_ref();
        fs::create_dir_all(dir).map_err(|source| TypoError::Io {
            path: dir.display().to_string(),
            source,
        })?;

        let mut sorted_glyphs: Vec<GlyphMetrics> = self.glyphs.values().copied().collect();
        sorted_glyphs.sort_by_key(|g| (g.role, g.codepoint));

        let mut sorted_kerns = self.kerns.clone();
        if !sorted_kerns.is_sorted_by_key(|&(k, _)| k) {
            sorted_kerns.sort_unstable_by_key(|&(k, _)| k);
        }

        let fm_pods: Vec<FontLineMetricsPod> = FontRole::ALL
            .iter()
            .map(|&role| {
                let m = self.font_metrics[role as usize];
                FontLineMetricsPod {
                    role: role as u32,
                    ascent_em: m.ascent_em,
                    descent_em: m.descent_em,
                    line_gap_em: m.line_gap_em,
                    line_height_em: m.line_height_em,
                    cap_height_em: m.cap_height_em,
                    x_height_em: m.x_height_em,
                    _pad: 0,
                }
            })
            .collect();

        let glyph_pods: Vec<CachedGlyphPod> = sorted_glyphs
            .iter()
            .map(|g| CachedGlyphPod {
                codepoint: g.codepoint,
                role: g.role as u8,
                source_font: g.source_font as u8,
                has_outline: u8::from(g.has_outline),
                _pad: 0,
                advance_em: g.advance_em,
                px_range: g.px_range,
                plane_bounds: g.plane_bounds,
                uv_rect: g.uv_rect,
                pixel_rect: g.pixel_rect,
            })
            .collect();

        let kern_pods: Vec<CachedKernPod> = sorted_kerns
            .iter()
            .map(|&(packed, kern_em)| CachedKernPod {
                left: ((packed >> 21) & 0x1F_FFFF) as u32,
                right: (packed & 0x1F_FFFF) as u32,
                role: ((packed >> 42) & 0x3) as u8,
                _pad: [0; 3],
                kern_em,
            })
            .collect();

        let mut payload = Vec::with_capacity(
            std::mem::size_of_val(fm_pods.as_slice())
                + std::mem::size_of_val(glyph_pods.as_slice())
                + std::mem::size_of_val(kern_pods.as_slice())
                + self.pixels_rgba8.len(),
        );
        payload.extend_from_slice(bytemuck::cast_slice(&fm_pods));
        payload.extend_from_slice(bytemuck::cast_slice(&glyph_pods));
        payload.extend_from_slice(bytemuck::cast_slice(&kern_pods));
        payload.extend_from_slice(&self.pixels_rgba8);

        let payload_crc32 = crc32fast::hash(&payload);
        let header = CacheHeaderPod {
            magic: CACHE_MAGIC,
            version: CACHE_VERSION,
            width: self.width,
            height: self.height,
            px_range: self.px_range,
            em_px: self.em_px,
            pad_px: self.pad_px,
            gutter_px: self.gutter_px,
            _pad0: 0,
            fonts_fingerprint: self.fonts_fingerprint,
            font_metrics_count: fm_pods.len() as u32,
            glyph_count: glyph_pods.len() as u32,
            kern_count: kern_pods.len() as u32,
            payload_crc32,
            _reserved: [0; 2],
        };

        static CACHE_TMP_SEQ: AtomicU64 = AtomicU64::new(0);
        let seq = CACHE_TMP_SEQ.fetch_add(1, Ordering::Relaxed);
        let pid = std::process::id();

        let bin_path = dir.join(CACHE_BIN_FILENAME);
        let tmp_bin = dir.join(format!("{CACHE_BIN_FILENAME}.{pid}.{seq}.tmp"));
        {
            let mut f = fs::File::create(&tmp_bin).map_err(|source| TypoError::Io {
                path: tmp_bin.display().to_string(),
                source,
            })?;
            f.write_all(bytemuck::bytes_of(&header))
                .and_then(|_| f.write_all(&payload))
                .and_then(|_| f.sync_all())
                .map_err(|source| TypoError::Io {
                    path: tmp_bin.display().to_string(),
                    source,
                })?;
        }
        fs::rename(&tmp_bin, &bin_path).map_err(|source| TypoError::Io {
            path: bin_path.display().to_string(),
            source,
        })?;

        let json_manifest = AtlasJsonManifest {
            version: CACHE_VERSION,
            width: self.width,
            height: self.height,
            em_px: self.em_px,
            px_range: self.px_range,
            pad_px: self.pad_px,
            gutter_px: self.gutter_px,
            fonts_fingerprint: format!("{:#018x}", self.fonts_fingerprint),
            font_metrics: FontRole::ALL
                .iter()
                .map(|r| (r.as_str().to_string(), self.font_metrics[*r as usize]))
                .collect(),
            glyphs: sorted_glyphs,
        };
        let json_path = dir.join(CACHE_JSON_FILENAME);
        let tmp_json = dir.join(format!("{CACHE_JSON_FILENAME}.{pid}.{seq}.tmp"));
        let json_str = serde_json::to_string_pretty(&json_manifest)?;
        fs::write(&tmp_json, json_str).map_err(|source| TypoError::Io {
            path: tmp_json.display().to_string(),
            source,
        })?;
        fs::rename(&tmp_json, &json_path).map_err(|source| TypoError::Io {
            path: json_path.display().to_string(),
            source,
        })?;

        Ok(())
    }

    #[inline]
    pub const fn width(&self) -> u32 {
        self.width
    }

    #[inline]
    pub const fn height(&self) -> u32 {
        self.height
    }

    #[inline]
    pub const fn px_range(&self) -> f32 {
        self.px_range
    }

    #[inline]
    pub const fn em_px(&self) -> f32 {
        self.em_px
    }

    #[inline]
    pub const fn fonts_fingerprint(&self) -> u64 {
        self.fonts_fingerprint
    }

    #[inline]
    pub fn rgba8_pixels(&self) -> &[u8] {
        &self.pixels_rgba8
    }

    #[inline]
    pub fn glyph_count(&self) -> usize {
        self.glyphs.len()
    }

    #[inline]
    pub fn font_metrics(&self, role: FontRole) -> FontLineMetrics {
        self.font_metrics[role as usize]
    }

    #[inline]
    pub fn glyph(&self, role: FontRole, ch: char) -> Option<&GlyphMetrics> {
        self.glyphs.get(&(role, ch))
    }

    /// Resolves `(role, ch)`, falling back cleanly to the alternate role, `'?'`, or `' '`.
    pub fn glyph_or_fallback(&self, role: FontRole, ch: char) -> &GlyphMetrics {
        if let Some(g) = self.glyphs.get(&(role, ch)) {
            return g;
        }
        let alt_role = match role {
            FontRole::DisplayScript => FontRole::UiSans,
            FontRole::UiSans => FontRole::DisplayScript,
        };
        if let Some(g) = self.glyphs.get(&(alt_role, ch)) {
            return g;
        }
        self.glyphs
            .get(&(role, '?'))
            .or_else(|| self.glyphs.get(&(FontRole::UiSans, '?')))
            .or_else(|| self.glyphs.get(&(role, ' ')))
            .expect("atlas always contains '?' or ' ' fallback")
    }

    #[inline]
    pub fn kern_em(&self, role: FontRole, left: char, right: char) -> f32 {
        let key = pack_kern_key(role, left, right);
        match self.kerns.binary_search_by_key(&key, |&(k, _)| k) {
            Ok(idx) => self.kerns[idx].1,
            Err(_) => 0.0,
        }
    }

    /// Returns `true` if every non-zero-width character in `text` exists in `role`'s baked glyphs.
    pub fn covers_string(&self, role: FontRole, text: &str) -> bool {
        text.chars()
            .filter(|&c| !matches!(c, '\n' | '\r' | '\u{200B}'))
            .all(|c| self.glyph(role, c).is_some())
    }

    /// Shapes `text` into positioned `em`-space glyphs with optional word wrapping (`max_width_em`).
    ///
    /// Preserves exact UTF-8 byte offsets and Unicode scalar indices (`0..total_scalars`) so
    /// SyncVoice `CueRecord` timings drive multi-line live-ink sweep seamlessly.
    pub fn shape_text(&self, role: FontRole, text: &str, max_width_em: Option<f32>) -> ShapedText {
        let fm = self.font_metrics(role);
        let wrapped_lines = wrap_lines_em(self, role, text, max_width_em);
        let mut shaped_glyphs = Vec::with_capacity(text.len());
        let mut shaped_lines = Vec::with_capacity(wrapped_lines.len());

        let mut max_w = 0.0f32;
        for (line_idx, segment) in wrapped_lines.iter().enumerate() {
            let line_start = shaped_glyphs.len();
            let mut pen_x = 0.0f32;
            let baseline_y = -(line_idx as f32) * fm.line_height_em;
            let mut prev_ch: Option<char> = None;

            for (byte_rel, ch) in segment.text.char_indices() {
                if ch == '\r' || ch == '\u{200B}' {
                    continue;
                }
                if let Some(prev) = prev_ch {
                    pen_x += self.kern_em(role, prev, ch);
                }
                let g = *self.glyph_or_fallback(role, ch);
                let byte_abs = segment.byte_offset + byte_rel;
                let scalar_idx = segment.scalar_offset + segment.text[..byte_rel].chars().count();
                shaped_glyphs.push(ShapedGlyph {
                    ch,
                    byte_start: byte_abs,
                    byte_end: byte_abs + ch.len_utf8(),
                    scalar_index: scalar_idx,
                    line_index: line_idx,
                    x_em: pen_x,
                    y_em: baseline_y,
                    metrics: g,
                });
                pen_x += g.advance_em;
                prev_ch = Some(ch);
            }

            max_w = max_w.max(pen_x);
            shaped_lines.push(ShapedLine {
                width_em: pen_x,
                glyph_start: line_start,
                glyph_end: shaped_glyphs.len(),
            });
        }

        let total_scalars = text.chars().count();
        let line_count = shaped_lines.len().max(1);
        let total_height_em =
            fm.ascent_em - fm.descent_em + (line_count.saturating_sub(1) as f32) * fm.line_height_em;

        ShapedText {
            role,
            glyphs: shaped_glyphs,
            lines: shaped_lines,
            total_scalars,
            max_width_em: max_w,
            total_height_em,
            ascent_em: fm.ascent_em,
            line_height_em: fm.line_height_em,
        }
    }

    /// Samples the decoded MSDF `median(r, g, b)` inside a glyph's baked tile using top-down
    /// normalized tile coordinates (`norm_x, norm_y_top_down in [0, 1]`).
    ///
    /// Values `> 0.5` lie inside the glyph outline, `< 0.5` outside.
    pub fn sample_tile_median(
        &self,
        role: FontRole,
        ch: char,
        norm_x: f32,
        norm_y_top_down: f32,
    ) -> Option<f32> {
        let g = self.glyph(role, ch)?;
        if !g.has_outline || g.pixel_rect[2] == 0 || g.pixel_rect[3] == 0 {
            return None;
        }
        let [rx, ry, rw, rh] = g.pixel_rect;
        let px = rx + ((norm_x.clamp(0.0, 0.999_999) * rw as f32).floor() as u32);
        let py = ry + ((norm_y_top_down.clamp(0.0, 0.999_999) * rh as f32).floor() as u32);
        let idx = ((py * self.width + px) * 4) as usize;
        let r = self.pixels_rgba8[idx] as f32 / 255.0;
        let gr = self.pixels_rgba8[idx + 1] as f32 / 255.0;
        let b = self.pixels_rgba8[idx + 2] as f32 / 255.0;
        Some(median3_f32(r, gr, b))
    }
}

struct WrappedSegment<'a> {
    text: &'a str,
    byte_offset: usize,
    scalar_offset: usize,
}

fn wrap_lines_em<'a>(
    atlas: &MsdfAtlas,
    role: FontRole,
    text: &'a str,
    max_width_em: Option<f32>,
) -> Vec<WrappedSegment<'a>> {
    let mut out = Vec::new();
    let mut byte_cursor = 0usize;
    let mut scalar_cursor = 0usize;

    for raw_line in text.split('\n') {
        let line = raw_line.strip_suffix('\r').unwrap_or(raw_line);
        let line_byte_base = byte_cursor;
        let line_scalar_base = scalar_cursor;

        match max_width_em {
            None => {
                out.push(WrappedSegment {
                    text: line,
                    byte_offset: line_byte_base,
                    scalar_offset: line_scalar_base,
                });
            }
            Some(max_w) => {
                if line.is_empty() {
                    out.push(WrappedSegment {
                        text: "",
                        byte_offset: line_byte_base,
                        scalar_offset: line_scalar_base,
                    });
                } else {
                    let mut start_b = 0usize;
                    let mut start_s = 0usize;
                    let mut cur_w = 0.0f32;
                    let mut prev_ch: Option<char> = None;
                    let mut last_break: Option<(usize, usize, usize)> = None; // (split_end_b, next_b, next_s)
                    let mut cur_s = 0usize;

                    for (b, ch) in line.char_indices() {
                        if ch == '\u{200B}' {
                            cur_s += 1;
                            continue;
                        }
                        let kern = prev_ch.map_or(0.0, |p| atlas.kern_em(role, p, ch));
                        let adv = atlas.glyph_or_fallback(role, ch).advance_em;
                        let next_w = cur_w + kern + adv;

                        if ch.is_whitespace() {
                            let next_b = b + ch.len_utf8();
                            last_break = Some((b, next_b, cur_s + 1));
                        }

                        if next_w > max_w && b > start_b && !ch.is_whitespace() {
                            if let Some((split_end_b, next_b, next_s)) = last_break {
                                out.push(WrappedSegment {
                                    text: &line[start_b..split_end_b],
                                    byte_offset: line_byte_base + start_b,
                                    scalar_offset: line_scalar_base + start_s,
                                });
                                start_b = next_b;
                                start_s = next_s;
                                cur_w = measure_span_em(atlas, role, &line[start_b..b + ch.len_utf8()]);
                                prev_ch = Some(ch);
                                last_break = None;
                                cur_s += 1;
                                continue;
                            } else {
                                out.push(WrappedSegment {
                                    text: &line[start_b..b],
                                    byte_offset: line_byte_base + start_b,
                                    scalar_offset: line_scalar_base + start_s,
                                });
                                start_b = b;
                                start_s = cur_s;
                                cur_w = adv;
                                prev_ch = Some(ch);
                                last_break = None;
                                cur_s += 1;
                                continue;
                            }
                        }

                        cur_w = next_w;
                        prev_ch = Some(ch);
                        cur_s += 1;
                    }

                    out.push(WrappedSegment {
                        text: &line[start_b..],
                        byte_offset: line_byte_base + start_b,
                        scalar_offset: line_scalar_base + start_s,
                    });
                }
            }
        }

        byte_cursor += raw_line.len() + 1; // + '\n'
        scalar_cursor += raw_line.chars().count() + 1;
    }

    if out.is_empty() {
        out.push(WrappedSegment {
            text: "",
            byte_offset: 0,
            scalar_offset: 0,
        });
    }
    out
}

fn measure_span_em(atlas: &MsdfAtlas, role: FontRole, span: &str) -> f32 {
    let mut w = 0.0f32;
    let mut prev: Option<char> = None;
    for ch in span.chars() {
        if matches!(ch, '\r' | '\n' | '\u{200B}') {
            continue;
        }
        if let Some(p) = prev {
            w += atlas.kern_em(role, p, ch);
        }
        w += atlas.glyph_or_fallback(role, ch).advance_em;
        prev = Some(ch);
    }
    w
}

#[inline]
fn median3_f32(a: f32, b: f32, c: f32) -> f32 {
    a.min(b).max(a.max(b).min(c))
}

type FontByteTriple = (Vec<u8>, Vec<u8>, Vec<u8>);

fn load_font_files(fonts_dir: &Path) -> Result<FontByteTriple, TypoError> {
    let mut script_file = "display_script_bold.ttf".to_string();
    let mut sans_file = "ui_sans_bold.ttf".to_string();
    let mut sym_file = "symbols_fallback.ttf".to_string();

    let manifest_path = fonts_dir.join("manifest.json");
    if manifest_path.is_file() {
        let raw_manifest = fs::read_to_string(&manifest_path).map_err(|source| TypoError::Io {
            path: manifest_path.display().to_string(),
            source,
        })?;
        let parsed: FontManifest = serde_json::from_str(&raw_manifest)?;
        for entry in parsed.fonts {
            match entry.role.as_str() {
                "display_script_bold" => script_file = entry.file,
                "ui_sans_bold" => sans_file = entry.file,
                "symbols_fallback" => sym_file = entry.file,
                _ => {}
            }
        }
    }

    let read_font = |filename: &str| -> Result<Vec<u8>, TypoError> {
        let p = fonts_dir.join(filename);
        fs::read(&p).map_err(|source| TypoError::Io {
            path: p.display().to_string(),
            source,
        })
    };

    Ok((
        read_font(&script_file)?,
        read_font(&sans_file)?,
        read_font(&sym_file)?,
    ))
}

fn compute_bake_fingerprint(
    display_script_bold: &[u8],
    ui_sans_bold: &[u8],
    symbols_fallback: &[u8],
    config: &MsdfBakeConfig,
) -> u64 {
    let mut h1 = crc32fast::Hasher::new();
    h1.update(&CACHE_MAGIC);
    h1.update(&CACHE_VERSION.to_le_bytes());
    h1.update(&config.atlas_width.to_le_bytes());
    h1.update(&config.atlas_height.to_le_bytes());
    h1.update(&config.em_px.to_bits().to_le_bytes());
    h1.update(&config.px_range.to_bits().to_le_bytes());
    h1.update(&config.pad_px.to_le_bytes());
    h1.update(&config.gutter_px.to_le_bytes());
    h1.update(config.required_charset.as_bytes());
    h1.update(b"|");
    h1.update(config.optional_charset.as_bytes());
    let cfg_crc = h1.finalize() as u64;

    let mut h2 = crc32fast::Hasher::new();
    h2.update(display_script_bold);
    h2.update(ui_sans_bold);
    h2.update(symbols_fallback);
    let font_crc = h2.finalize() as u64;

    (font_crc << 32) | cfg_crc
}

fn parse_font_data<'a>(label: &str, bytes: &'a [u8]) -> Result<FontData<'a>, TypoError> {
    let face = Face::parse(bytes, 0).map_err(|e| TypoError::InvalidFont {
        path: label.to_string(),
        reason: e.to_string(),
    })?;
    let ab_font = FontVec::try_from_vec(bytes.to_vec()).map_err(|e| TypoError::InvalidFont {
        path: label.to_string(),
        reason: e.to_string(),
    })?;
    let upem = face.units_per_em().max(1) as f64;
    Ok(FontData {
        face,
        ab_font,
        units_per_em: upem,
    })
}

fn extract_line_metrics(font: &FontData<'_>) -> FontLineMetrics {
    let upem = font.units_per_em as f32;
    let ascent = font.ab_font.ascent_unscaled() / upem;
    let descent = font.ab_font.descent_unscaled() / upem;
    let line_gap = font.ab_font.line_gap_unscaled() / upem;
    let line_height = (ascent - descent + line_gap).max(1.05);
    let cap_height = font
        .face
        .capital_height()
        .map_or(ascent * 0.72, |v| v as f32 / upem);
    let x_height = font
        .face
        .x_height()
        .map_or(ascent * 0.52, |v| v as f32 / upem);

    FontLineMetrics {
        ascent_em: ascent,
        descent_em: descent,
        line_gap_em: line_gap,
        line_height_em: line_height,
        cap_height_em: cap_height,
        x_height_em: x_height,
    }
}

fn bake_single_glyph(
    fonts: &[FontData<'_>; 3],
    role: FontRole,
    ch: char,
    required: bool,
    config: &MsdfBakeConfig,
) -> Result<Option<BakedTile>, TypoError> {
    let chain: &[FontSource] = match role {
        FontRole::DisplayScript => &[
            FontSource::DisplayScriptBold,
            FontSource::UiSansBold,
            FontSource::SymbolsFallback,
        ],
        FontRole::UiSans => &[FontSource::UiSansBold, FontSource::SymbolsFallback],
    };

    let is_blank = ch.is_whitespace() || ch == '\u{200B}';

    for &src in chain {
        let f = &fonts[src as usize];
        let Some(gid) = f.face.glyph_index(ch).filter(|g| g.0 != 0) else {
            continue;
        };
        let advance_em = f
            .face
            .glyph_hor_advance(gid)
            .map_or_else(|| f.ab_font.h_advance_unscaled(f.ab_font.glyph_id(ch)), |v| v as f32)
            / f.units_per_em as f32;

        if is_blank {
            return Ok(Some(BakedTile {
                ch,
                role,
                source_font: src,
                has_outline: false,
                advance_em,
                plane_bounds: [0.0; 4],
                tile_w: 0,
                tile_h: 0,
                pixels_rgba8: Vec::new(),
            }));
        }

        let Some(bbox) = f.face.glyph_bounding_box(gid) else {
            continue;
        };
        let Some(mut shape) = fdsm_ttf_parser::load_shape_from_face(&f.face, gid) else {
            continue;
        };
        shape.contours.retain(|c| !c.segments.is_empty());
        if shape.contours.is_empty() {
            continue;
        }

        let scale = config.em_px as f64 / f.units_per_em;
        let pad = config.pad_px as f64;
        let x_min = bbox.x_min as f64;
        let y_min = bbox.y_min as f64;
        let x_max = bbox.x_max as f64;
        let y_max = bbox.y_max as f64;

        let tile_w = ((x_max - x_min) * scale + 2.0 * pad).ceil().max(1.0) as u32;
        let tile_h = ((y_max - y_min) * scale + 2.0 * pad).ceil().max(1.0) as u32;

        let tx = pad - x_min * scale;
        let ty = pad - y_min * scale;
        let affine = na::Affine2::from_matrix_unchecked(na::Matrix3::new(
            scale, 0.0, tx, 0.0, scale, ty, 0.0, 0.0, 1.0,
        ));
        shape.transform(&affine);

        let seed = ((role as u64) << 32) ^ (ch as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15);
        let colored = Shape::edge_coloring_simple(shape, 0.03, seed);
        let prepared = colored.prepare();

        let mut mtsdf = Rgba32FImage::new(tile_w, tile_h);
        generate_mtsdf(&prepared, config.px_range as f64, &mut mtsdf);
        correct_error_mtsdf(
            &mut mtsdf,
            &colored,
            &prepared,
            config.px_range as f64,
            &ErrorCorrectionConfig::default(),
        );
        correct_sign_mtsdf(&mut mtsdf, &prepared, FillRule::Nonzero);

        // Flip Y from fdsm Y-up tile space into GPU top-down texture rows (`v=0` top -> `v=1` bottom).
        let mut pixels_rgba8 = vec![0u8; (tile_w * tile_h * 4) as usize];
        for y_down in 0..tile_h {
            let y_up = tile_h - 1 - y_down;
            for x in 0..tile_w {
                let px = mtsdf.get_pixel(x, y_up).0;
                let dst = ((y_down * tile_w + x) * 4) as usize;
                pixels_rgba8[dst] = (px[0].clamp(0.0, 1.0) * 255.0).round() as u8;
                pixels_rgba8[dst + 1] = (px[1].clamp(0.0, 1.0) * 255.0).round() as u8;
                pixels_rgba8[dst + 2] = (px[2].clamp(0.0, 1.0) * 255.0).round() as u8;
                pixels_rgba8[dst + 3] = (px[3].clamp(0.0, 1.0) * 255.0).round() as u8;
            }
        }

        let left_em = ((x_min / f.units_per_em) - (pad / config.em_px as f64)) as f32;
        let bottom_em = ((y_min / f.units_per_em) - (pad / config.em_px as f64)) as f32;
        let right_em = left_em + (tile_w as f32 / config.em_px);
        let top_em = bottom_em + (tile_h as f32 / config.em_px);

        return Ok(Some(BakedTile {
            ch,
            role,
            source_font: src,
            has_outline: true,
            advance_em,
            plane_bounds: [left_em, bottom_em, right_em, top_em],
            tile_w,
            tile_h,
            pixels_rgba8,
        }));
    }

    if is_blank {
        return Ok(Some(BakedTile {
            ch,
            role,
            source_font: FontSource::UiSansBold,
            has_outline: false,
            advance_em: if ch == '\u{200B}' { 0.0 } else { 0.28 },
            plane_bounds: [0.0; 4],
            tile_w: 0,
            tile_h: 0,
            pixels_rgba8: Vec::new(),
        }));
    }

    if required {
        Err(TypoError::MissingGlyph {
            role: role.as_str(),
            ch,
            code: ch as u32,
        })
    } else {
        Ok(None)
    }
}

struct CacheSectionLayout {
    fm_bytes: usize,
    gl_bytes: usize,
    kn_bytes: usize,
    px_bytes: usize,
}

impl CacheSectionLayout {
    fn from_header(header: &CacheHeaderPod, hdr_size: usize, total_len: usize) -> Result<Self, TypoError> {
        let fm_bytes = header.font_metrics_count as usize * std::mem::size_of::<FontLineMetricsPod>();
        let gl_bytes = header.glyph_count as usize * std::mem::size_of::<CachedGlyphPod>();
        let kn_bytes = header.kern_count as usize * std::mem::size_of::<CachedKernPod>();
        let px_bytes = (header.width as usize)
            .checked_mul(header.height as usize)
            .and_then(|p| p.checked_mul(4))
            .ok_or_else(|| TypoError::InvalidCache {
                reason: "atlas dimensions overflow".into(),
            })?;

        let expected_total = hdr_size + fm_bytes + gl_bytes + kn_bytes + px_bytes;
        if total_len != expected_total {
            return Err(TypoError::InvalidCache {
                reason: format!(
                    "unexpected cache size {} (expected {})",
                    total_len, expected_total
                ),
            });
        }

        Ok(Self {
            fm_bytes,
            gl_bytes,
            kn_bytes,
            px_bytes,
        })
    }
}

fn parse_and_validate_header(
    hdr_bytes: &[u8],
    expected_fingerprint: Option<u64>,
) -> Result<CacheHeaderPod, TypoError> {
    let header: CacheHeaderPod = bytemuck::pod_read_unaligned(hdr_bytes);
    if header.magic != CACHE_MAGIC {
        return Err(TypoError::InvalidCache {
            reason: "bad cache magic".into(),
        });
    }
    if header.version != CACHE_VERSION {
        return Err(TypoError::InvalidCache {
            reason: format!("unsupported cache version {}", header.version),
        });
    }
    if let Some(fp) = expected_fingerprint
        && header.fonts_fingerprint != fp
    {
        return Err(TypoError::InvalidCache {
            reason: "font or charset fingerprint changed".into(),
        });
    }
    Ok(header)
}

fn build_atlas_from_sections(
    header: &CacheHeaderPod,
    fm_slice: &[u8],
    gl_slice: &[u8],
    kn_slice: &[u8],
    pixels_rgba8: Vec<u8>,
) -> Result<MsdfAtlas, TypoError> {
    let mut font_metrics = [FontLineMetrics {
        ascent_em: 0.8,
        descent_em: -0.2,
        line_gap_em: 0.1,
        line_height_em: 1.1,
        cap_height_em: 0.72,
        x_height_em: 0.52,
    }; 2];
    for chunk in fm_slice.chunks_exact(std::mem::size_of::<FontLineMetricsPod>()) {
        let rec: FontLineMetricsPod = bytemuck::pod_read_unaligned(chunk);
        if let Some(role) = FontRole::from_u8(rec.role as u8) {
            font_metrics[role as usize] = FontLineMetrics {
                ascent_em: rec.ascent_em,
                descent_em: rec.descent_em,
                line_gap_em: rec.line_gap_em,
                line_height_em: rec.line_height_em,
                cap_height_em: rec.cap_height_em,
                x_height_em: rec.x_height_em,
            };
        }
    }

    let mut glyphs = HashMap::with_capacity(header.glyph_count as usize);
    for chunk in gl_slice.chunks_exact(std::mem::size_of::<CachedGlyphPod>()) {
        let rec: CachedGlyphPod = bytemuck::pod_read_unaligned(chunk);
        let ch = char::from_u32(rec.codepoint).ok_or_else(|| TypoError::InvalidCache {
            reason: format!("invalid codepoint {:#x}", rec.codepoint),
        })?;
        let role = FontRole::from_u8(rec.role).ok_or_else(|| TypoError::InvalidCache {
            reason: format!("invalid font role {}", rec.role),
        })?;
        let source_font =
            FontSource::from_u8(rec.source_font).ok_or_else(|| TypoError::InvalidCache {
                reason: format!("invalid font source {}", rec.source_font),
            })?;
        glyphs.insert(
            (role, ch),
            GlyphMetrics {
                codepoint: rec.codepoint,
                role,
                source_font,
                has_outline: rec.has_outline != 0,
                advance_em: rec.advance_em,
                px_range: rec.px_range,
                plane_bounds: rec.plane_bounds,
                uv_rect: rec.uv_rect,
                pixel_rect: rec.pixel_rect,
            },
        );
    }

    let mut kerns = Vec::with_capacity(header.kern_count as usize);
    for chunk in kn_slice.chunks_exact(std::mem::size_of::<CachedKernPod>()) {
        let rec: CachedKernPod = bytemuck::pod_read_unaligned(chunk);
        if let (Some(l), Some(r), Some(role)) = (
            char::from_u32(rec.left),
            char::from_u32(rec.right),
            FontRole::from_u8(rec.role),
        ) {
            kerns.push((pack_kern_key(role, l, r), rec.kern_em));
        }
    }
    if !kerns.is_sorted_by_key(|&(k, _)| k) {
        kerns.sort_unstable_by_key(|&(k, _)| k);
    }

    Ok(MsdfAtlas {
        width: header.width,
        height: header.height,
        em_px: header.em_px,
        px_range: header.px_range,
        pad_px: header.pad_px,
        gutter_px: header.gutter_px,
        fonts_fingerprint: header.fonts_fingerprint,
        font_metrics,
        glyphs,
        kerns,
        pixels_rgba8,
    })
}
