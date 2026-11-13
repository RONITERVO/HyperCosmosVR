//! Real-time Spatial Audio (`hrtf` + `cpal` 48 kHz WASAPI), SyncVoice decoder (`symphonia` + `wsola` + `rubato`),
//! FunDSP procedural synthesizer, and lock-free `AudioReactiveUniforms` GPU bridge for `HyperCosmosVR`.

pub mod decoder;
pub mod engine;
pub mod error;
pub mod spatial;
pub mod synth;
pub mod uniforms;

pub use decoder::{
    clamp_playback_rate, decode_mp3_bytes, snap_playback_rate, stretch_and_resample_to_48k,
    DecodedPcm, NarrationCache, PreparedClip, OUTPUT_SAMPLE_RATE, SUPPORTED_PLAYBACK_RATES,
};
pub use engine::{
    AtomicPlaybackClock, AudioCommand, AudioEngine, AudioMixer, AudioOutputMode,
    PlaybackClockSnapshot, SyncVoiceClockState,
};
pub use error::AudioError;
pub use spatial::{
    SpatialNarrationRenderer, AUDIO_BLOCK_FRAMES, HEAD_LOCKED_CLARITY_BLEND,
    SPATIAL_HRTF_BLEND,
};
pub use synth::{FeedbackChimeSynth, MusicPolySynth, SynthTelemetry, ZoneAmbientSynth};
pub use uniforms::{AtomicAudioReactiveBus, AudioReactiveUniforms, OutputAnalyzer};
