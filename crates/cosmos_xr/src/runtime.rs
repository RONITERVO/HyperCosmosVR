use openxr::{self as xr, Graphics as _};

/// Windows DXGI adapter LUID reported by `XR_KHR_D3D12_enable`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AdapterLuid {
    pub low_part: u32,
    pub high_part: i32,
}

impl AdapterLuid {
    #[inline]
    pub const fn from_raw(raw: xr::sys::platform::LUID) -> Self {
        Self {
            low_part: raw.LowPart,
            high_part: raw.HighPart,
        }
    }

    /// Returns `(LowPart, HighPart)` matching `cosmos_render`'s DXGI LUID selector.
    #[inline]
    pub const fn to_tuple(self) -> (u32, i32) {
        (self.low_part, self.high_part)
    }
}

/// Extension flags discovered on the active OpenXR runtime.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct XrExtensionSupport {
    pub khr_d3d12_enable: bool,
    pub ext_hand_tracking: bool,
    pub fb_display_refresh_rate: bool,
    pub fb_color_space: bool,
    pub ext_debug_utils: bool,
}

impl XrExtensionSupport {
    #[inline]
    pub fn from_openxr(exts: &xr::ExtensionSet) -> Self {
        Self {
            khr_d3d12_enable: exts.khr_d3d12_enable,
            ext_hand_tracking: exts.ext_hand_tracking,
            fb_display_refresh_rate: exts.fb_display_refresh_rate,
            fb_color_space: exts.fb_color_space,
            ext_debug_utils: exts.ext_debug_utils,
        }
    }

    #[inline]
    pub fn to_enabled_set(&self) -> xr::ExtensionSet {
        let mut set = xr::ExtensionSet::default();
        set.khr_d3d12_enable = self.khr_d3d12_enable;
        set.ext_hand_tracking = self.ext_hand_tracking;
        set.fb_display_refresh_rate = self.fb_display_refresh_rate;
        set.fb_color_space = self.fb_color_space;
        set.ext_debug_utils = self.ext_debug_utils;
        set
    }
}

/// Per-eye recommended swapchain dimensions from `xrEnumerateViewConfigurationViews`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecommendedViewConfig {
    pub width: u32,
    pub height: u32,
    pub max_width: u32,
    pub max_height: u32,
    pub recommended_swapchain_sample_count: u32,
}

/// Discovered OpenXR runtime and HMD hardware metadata.
#[derive(Clone, Debug)]
pub struct XrSystemDetails {
    pub runtime_name: String,
    pub runtime_version: String,
    pub system_name: String,
    pub vendor_id: u32,
    pub adapter_luid: AdapterLuid,
    pub min_d3d_feature_level: i32,
    pub orientation_tracking: bool,
    pub position_tracking: bool,
    pub recommended_eye_res: [RecommendedViewConfig; 2],
    pub supported_refresh_rates_hz: Vec<f32>,
    pub extensions: XrExtensionSupport,
}

/// Reason why OpenXR VR could not be initialized (cleanly falls back to Desktop HDR).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NoHeadsetReason {
    LoaderFailed(String),
    RequiredExtensionMissing(&'static str),
    InstanceCreationFailed(String),
    HeadsetUnavailable(String),
    GraphicsRequirementsFailed(String),
    ViewConfigurationUnsupported(String),
}

/// Outcome of probing OpenXR via the statically linked Khronos loader (`xr::Entry::linked`).
pub enum XrAvailability {
    Available {
        entry: xr::Entry,
        instance: xr::Instance,
        system_id: xr::SystemId,
        details: XrSystemDetails,
    },
    NoHeadset {
        reason: NoHeadsetReason,
        runtime_name: Option<String>,
        extensions: Option<XrExtensionSupport>,
    },
}

impl XrAvailability {
    #[inline]
    pub fn is_available(&self) -> bool {
        matches!(self, Self::Available { .. })
    }

    #[inline]
    pub fn adapter_luid(&self) -> Option<(u32, i32)> {
        match self {
            Self::Available { details, .. } => Some(details.adapter_luid.to_tuple()),
            Self::NoHeadset { .. } => None,
        }
    }

    #[inline]
    pub fn details(&self) -> Option<&XrSystemDetails> {
        match self {
            Self::Available { details, .. } => Some(details),
            Self::NoHeadset { .. } => None,
        }
    }
}

/// Probes the statically linked Khronos OpenXR loader (`openxr/static`) and enumerates
/// `XR_KHR_D3D12_enable`, `XR_EXT_hand_tracking`, and `XR_FB_display_refresh_rate`.
///
/// When no HMD is powered on or `ERROR_FORM_FACTOR_UNAVAILABLE` is returned by the runtime,
/// returns [`XrAvailability::NoHeadset`] so the engine transitions seamlessly to Desktop HDR mode.
pub fn probe_openxr(app_name: &str, app_version: u32) -> XrAvailability {
    let entry = match xr::Entry::linked(&()) {
        Ok(entry) => entry,
        Err(err) => {
            return XrAvailability::NoHeadset {
                reason: NoHeadsetReason::LoaderFailed(err.to_string()),
                runtime_name: None,
                extensions: None,
            };
        }
    };

    let supported = match entry.enumerate_extensions() {
        Ok(exts) => XrExtensionSupport::from_openxr(&exts),
        Err(err) => {
            return XrAvailability::NoHeadset {
                reason: NoHeadsetReason::LoaderFailed(format!(
                    "enumerate_extensions failed: {err}"
                )),
                runtime_name: None,
                extensions: None,
            };
        }
    };

    if !supported.khr_d3d12_enable {
        return XrAvailability::NoHeadset {
            reason: NoHeadsetReason::RequiredExtensionMissing("XR_KHR_D3D12_enable"),
            runtime_name: None,
            extensions: Some(supported),
        };
    }

    let enabled_exts = supported.to_enabled_set();
    let app_info = xr::ApplicationInfo {
        application_name: app_name,
        application_version: app_version,
        engine_name: "HyperCosmosVR",
        engine_version: 1,
        api_version: xr::Version::new(1, 0, 0),
    };

    let instance = match entry.create_instance(&app_info, &enabled_exts, &[], &()) {
        Ok(inst) => inst,
        Err(err) => {
            return XrAvailability::NoHeadset {
                reason: NoHeadsetReason::InstanceCreationFailed(err.to_string()),
                runtime_name: None,
                extensions: Some(supported),
            };
        }
    };

    let inst_props = instance.properties().ok();
    let runtime_name = inst_props
        .as_ref()
        .map(|p| p.runtime_name.clone())
        .unwrap_or_else(|| "Unknown OpenXR Runtime".to_owned());
    let runtime_version = inst_props
        .as_ref()
        .map(|p| p.runtime_version.to_string())
        .unwrap_or_else(|| "0.0.0".to_owned());

    let system_id = match instance.system(xr::FormFactor::HEAD_MOUNTED_DISPLAY) {
        Ok(id) => id,
        Err(err) => {
            return XrAvailability::NoHeadset {
                reason: NoHeadsetReason::HeadsetUnavailable(err.to_string()),
                runtime_name: Some(runtime_name),
                extensions: Some(supported),
            };
        }
    };

    let reqs = match xr::D3D12::requirements(&instance, system_id) {
        Ok(reqs) => reqs,
        Err(err) => {
            return XrAvailability::NoHeadset {
                reason: NoHeadsetReason::GraphicsRequirementsFailed(err.to_string()),
                runtime_name: Some(runtime_name),
                extensions: Some(supported),
            };
        }
    };

    let sys_props = instance.system_properties(system_id).ok();
    let views = match instance
        .enumerate_view_configuration_views(system_id, xr::ViewConfigurationType::PRIMARY_STEREO)
    {
        Ok(v) if v.len() >= 2 => v,
        Ok(v) => {
            return XrAvailability::NoHeadset {
                reason: NoHeadsetReason::ViewConfigurationUnsupported(format!(
                    "expected >= 2 PRIMARY_STEREO views, got {}",
                    v.len()
                )),
                runtime_name: Some(runtime_name),
                extensions: Some(supported),
            };
        }
        Err(err) => {
            return XrAvailability::NoHeadset {
                reason: NoHeadsetReason::ViewConfigurationUnsupported(err.to_string()),
                runtime_name: Some(runtime_name),
                extensions: Some(supported),
            };
        }
    };

    let to_cfg = |v: &xr::ViewConfigurationView| RecommendedViewConfig {
        width: v.recommended_image_rect_width,
        height: v.recommended_image_rect_height,
        max_width: v.max_image_rect_width,
        max_height: v.max_image_rect_height,
        recommended_swapchain_sample_count: v.recommended_swapchain_sample_count,
    };

    let details = XrSystemDetails {
        runtime_name,
        runtime_version,
        system_name: sys_props
            .as_ref()
            .map(|p| p.system_name.clone())
            .unwrap_or_else(|| "OpenXR HMD".to_owned()),
        vendor_id: sys_props.as_ref().map(|p| p.vendor_id).unwrap_or(0),
        adapter_luid: AdapterLuid::from_raw(reqs.adapter_luid),
        min_d3d_feature_level: reqs.min_feature_level,
        orientation_tracking: sys_props
            .as_ref()
            .map(|p| p.tracking_properties.orientation_tracking)
            .unwrap_or(true),
        position_tracking: sys_props
            .as_ref()
            .map(|p| p.tracking_properties.position_tracking)
            .unwrap_or(true),
        recommended_eye_res: [to_cfg(&views[0]), to_cfg(&views[1])],
        // Refresh rates are enumerated on Session once FB_display_refresh_rate is initialized.
        supported_refresh_rates_hz: Vec::new(),
        extensions: supported,
    };

    XrAvailability::Available {
        entry,
        instance,
        system_id,
        details,
    }
}
