use thiserror::Error;

/// Errors produced by audio decoding, timestretching, HRTF initialization, or WASAPI output.
#[derive(Debug, Error)]
pub enum AudioError {
    #[error("MP3 container probe failed: {0}")]
    ProbeFailed(String),

    #[error("no audio track found inside encoded stream")]
    MissingAudioTrack,

    #[error("unsupported or missing audio codec parameters")]
    MissingCodecParams,

    #[error("audio decoder initialization failed: {0}")]
    DecoderInit(String),

    #[error("decoded MP3 contained zero PCM samples")]
    EmptyPcm,

    #[error("WSOLA timestretch error: {0}")]
    Wsola(#[from] wsola::Error),

    #[error("Rubato resampler error: {0}")]
    Resample(String),

    #[error("HRIR sphere load failed for {path}: {reason}")]
    HrirLoad { path: String, reason: String },

    #[error("audio output initialization error: {0}")]
    OutputDevice(String),
}
