use thiserror::Error;

/// Errors returned when loading or validating a baked `HyperCosmosVR` universe catalog or audio pack.
#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("I/O error accessing {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("catalog buffer too small ({actual} bytes, expected at least {expected})")]
    TruncatedHeader { actual: usize, expected: usize },

    #[error("invalid catalog magic bytes (expected HCVRCAT1)")]
    BadCatalogMagic,

    #[error("unsupported catalog format version {0}")]
    UnsupportedVersion(u32),

    #[error("catalog CRC32 mismatch (expected 0x{expected:08x}, computed 0x{computed:08x})")]
    CatalogCrcMismatch { expected: u32, computed: u32 },

    #[error("audio pack size mismatch (expected {expected} bytes, actual {actual} bytes)")]
    AudioPackSizeMismatch { expected: u64, actual: usize },

    #[error("invalid audio pack magic bytes (expected HCVRAUD1)")]
    BadAudioMagic,

    #[error("unsupported audio pack format version {0}")]
    UnsupportedAudioVersion(u32),

    #[error("audio pack CRC32 mismatch (expected 0x{expected:08x}, computed 0x{computed:08x})")]
    AudioCrcMismatch { expected: u32, computed: u32 },

    #[error("section {name} out of bounds (offset {offset}, byte_len {byte_len}, total {total})")]
    SectionOutOfBounds {
        name: &'static str,
        offset: usize,
        byte_len: usize,
        total: usize,
    },

    #[error("section {name} misaligned or invalid size for pod cast: {reason}")]
    PodCastError { name: &'static str, reason: String },

    #[error("string slice [{off}..{end}] out of bounds (arena len {arena_len})")]
    StringOutOfBounds {
        off: usize,
        end: usize,
        arena_len: usize,
    },

    #[error("string slice [{off}..{end}] is not valid UTF-8")]
    InvalidUtf8 { off: usize, end: usize },

    #[error("record {kind}[{index}] references invalid {field} range [{start}..{end}] (limit {limit})")]
    InvalidRange {
        kind: &'static str,
        index: usize,
        field: &'static str,
        start: u64,
        end: u64,
        limit: u64,
    },

    #[error("cue {cue_index} in asset {asset_index} splits a UTF-8 codepoint ([{start}..{end}])")]
    CueBadUtf8Boundary {
        asset_index: usize,
        cue_index: usize,
        start: u32,
        end: u32,
    },

    #[error("experience {app_num} key table is not strictly sorted in UTF-8 byte order")]
    UnsortedKeyTable { app_num: u8 },
}
