//! Zero-copy mmap loader and domain solvers for all 25 experiences of `HyperCosmosVR`.

pub mod color_solver;
pub mod error;
pub mod format;
pub mod loader;
pub mod music;

pub use color_solver::{contrast_ink, rgb_to_hsl, spanish_color_name, spanish_color_parts, srgb_relative_luminance, HslColor};
pub use error::ManifestError;
pub use format::{
    AppKeyEntryRecord, AudioAssetRecord, BodyLayoutRecord, CueRecord, ExperienceKind,
    ExperienceRecord, FeatureRecord, HcvbHeader, LandmarkRecord, Locale, MetricMode,
    MetricTokenRecord, MusicNoteRecord, MusicTimbreRecord, PhraseTriple, StrSlice, UnitKind,
    Waveform, ZoneRecord, ZoneSoundRecord, AUDIO_STRETCH_BACKEND, FORMAT_VERSION, MAGIC_AUD,
    MAGIC_CAT, NONE_ASSET,
};
pub use loader::{CueProgress, SolvedColor, SolvedMusicTouch, UniverseCatalog};
pub use music::{
    color_hue_for_midi, midi_for_x, midi_to_hz, octave_for_midi, pitch_class, timbre_index_for_y,
    MIDI_COUNT, MIDI_MIN,
};
