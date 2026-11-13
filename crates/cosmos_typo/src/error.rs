use thiserror::Error;

/// Errors produced during MSDF atlas baking, cache loading, layout, or GPU pipeline setup.
#[derive(Debug, Error)]
pub enum TypoError {
    #[error("I/O error at '{path}': {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("JSON serialization/parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("invalid font file '{path}': {reason}")]
    InvalidFont { path: String, reason: String },

    #[error("font fallback chain missing required glyph '{ch}' (U+{code:04X}) for role {role}")]
    MissingGlyph {
        role: &'static str,
        ch: char,
        code: u32,
    },

    #[error("MSDF atlas {width}x{height} overflowed packing {count} glyphs")]
    AtlasOverflow {
        width: u32,
        height: u32,
        count: usize,
    },

    #[error("MSDF cache invalid or stale ({reason})")]
    InvalidCache { reason: String },

    #[error("invalid shader directory or missing WGSL shader '{path}': {reason}")]
    ShaderLoad { path: String, reason: String },
}
