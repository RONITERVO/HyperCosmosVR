//! 3D World-Space MSDF Typography, Google-Earth Angular De-Overlap Layout,
//! Stereo Vergence Placement, and SyncVoice Live-Ink Renderer for HyperCosmosVR.

pub mod atlas;
pub mod error;
pub mod gpu;
pub mod layout;

pub use atlas::{
    CACHE_BIN_FILENAME, CACHE_JSON_FILENAME, FontLineMetrics, FontRole, FontSource, GlyphMetrics,
    MsdfAtlas, MsdfBakeConfig, OPTIONAL_COSMIC_CHARS, REQUIRED_EXTENDED_CHARS, ShapedGlyph,
    ShapedLine, ShapedText, default_required_charset,
};
pub use error::TypoError;
pub use gpu::{
    AUDIO_REACTIVE_UNIFORMS_BYTES, FRAME_UNIFORMS_BYTES, GlyphInstance, MsdfPipelineConfig,
    MsdfTextRenderer, TetherInstance, TextPassUniforms, compile_msdf_shader_module,
    create_shared_group0_layout,
};
pub use layout::{
    ANGULAR_DIST_MAX_M, ANGULAR_DIST_MIN_M, AmbientCandidateAngular, AmbientLabelInput,
    AnchorMode, AngularRect, BilingualNarrationSpec, BilingualPhase, CameraBasis,
    DEFAULT_MAX_AMBIENT_LABELS, DEFAULT_MAX_VERTICAL_NUDGE_RAD, DEFAULT_VERGENCE_MAX_M,
    DEFAULT_VERGENCE_MIN_M, DeoverlapPlacement, GlyphInkMode, PlacedAmbientLabel, TypoFrameOutput,
    TypoLayoutEngine, apparent_world_height, build_frame_typography,
    build_frame_typography_with_blockers, cue_reveal_cursor_for_asset, cue_reveal_cursor_for_cues,
    scale_band_alpha, solve_ambient_deoverlap,
};
