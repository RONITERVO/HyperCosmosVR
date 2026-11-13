//! AAA+ DX12 (`DynamicDxc`) OLED HDR + OpenXR `MULTIVIEW` Renderer for `HyperCosmosVR`.
//!
//! Renders continuous multiscale scenes from Planck foam (`10^-35 m`) through Earth, Solar
//! System, Milky Way, Laniakea, Cosmic Web, Inflation, and the 5-petaled Hyper-Horizon Lotus
//! (`>10^+28 m`) with true `#000000` OLED void preservation, Reversed-Z infinite far plane,
//! Dual-Kawase bloom (`threshold >= 1.05`), chromatic edge dispersion, AgX tonemapping, and
//! 128x128 triangular-PDF blue-noise dithering.

pub mod device;
pub mod doc_stages;
pub mod pipeline;
pub mod post_pass;
pub mod renderer;
pub mod scene_pass;
pub mod types;
pub mod universe_scene;

pub use device::{GpuContext, adapter_dxgi_luid, dx12_raw_device_and_queue, init_for_dx12_luid, locate_dxc_compiler};
pub use pipeline::{DEPTH_FORMAT, HDR_SCENE_FORMAT, PipelineBank, STEREO_MULTIVIEW_MASK};
pub use renderer::CosmosRenderer;
pub use scene_pass::seed_particles_for_mode;
pub use types::{
    AudioReactiveUniforms, DiegeticUiInput, EyeView, FarShellUniforms, FrameUniforms, HeroVertex,
    HeroVolumeInstance, OutputEncodeMode, PostPassUniforms, PostSettings, RenderCamera,
    RenderSceneFrame, SimDispatchUniforms, SimParticle, SpriteBatchUniforms, SpriteInstance,
};
pub use universe_scene::{
    DocumentaryFocusState, SceneBuildInput, UniverseSceneBuilder, experience_accent_linear,
    initial_particles_for_mode,
};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum RenderError {
    #[error("unable to locate dxcompiler.dll alongside exe or Windows Kits 10 bin")]
    DxcNotFound,
    #[error("DX12 adapter not found: {0}")]
    AdapterNotFound(String),
    #[error("DX12 device initialization error: {0}")]
    DeviceInit(String),
    #[error("shader or pipeline initialization error: {0}")]
    PipelineInit(String),
    #[error("GPU offscreen readback error: {0}")]
    Readback(String),
}

