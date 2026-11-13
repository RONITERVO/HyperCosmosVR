use thiserror::Error;

/// Errors surfaced by OpenXR discovery, D3D12/`wgpu` swapchain interop, and input polling.
#[derive(Debug, Error)]
pub enum XrError {
    #[error("OpenXR API error: {0}")]
    OpenXr(#[from] openxr::sys::Result),

    #[error("required OpenXR extension `{0}` is not supported by active runtime")]
    ExtensionMissing(&'static str),

    #[error("no supported stereo swapchain format matched the DXGI preference list")]
    UnsupportedSwapchainFormat,

    #[error("null native handle encountered for {0}")]
    NullNativeHandle(&'static str),

    #[error("OpenXR requires D3D12 adapter LUID {required:?}, got {actual:?}")]
    AdapterLuidMismatch {
        required: (u32, i32),
        actual: Option<(u32, i32)>,
    },

    #[error("expected 2 stereo views for PRIMARY_STEREO, found {0}")]
    InvalidEyeCount(usize),

    #[error("OpenXR session state error: {0}")]
    InvalidSessionState(String),

    #[error("OpenXR runtime unavailable: {0}")]
    RuntimeUnavailable(String),
}

