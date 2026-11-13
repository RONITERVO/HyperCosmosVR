use std::ffi::c_void;

use cosmos_math::DEFAULT_VR_NEAR_RENDER_UNITS;
use glam::{Mat4, Quat, Vec3, Vec4};
use openxr as xr;
use windows::Win32::Graphics::Direct3D12::ID3D12Resource;
use windows::Win32::Graphics::Dxgi::Common::{
    DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_B8G8R8A8_UNORM_SRGB, DXGI_FORMAT_R8G8B8A8_UNORM,
    DXGI_FORMAT_R8G8B8A8_UNORM_SRGB, DXGI_FORMAT_R10G10B10A2_UNORM,
    DXGI_FORMAT_R16G16B16A16_FLOAT,
};
use windows_core::Interface;

use crate::error::XrError;
use crate::input::{
    XrActionRig, XrInputFrame, XrPose, compute_stereo_matrices,
};
use crate::runtime::XrSystemDetails;

/// User preference for OpenXR D3D12 stereo swapchain negotiation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwapchainFormatPreference {
    /// Prefer 8-bit hardware sRGB (`DXGI_FORMAT_R8G8B8A8_UNORM_SRGB` / `B8G8R8A8_UNORM_SRGB`),
    /// `output_encode_mode = 0.0` (maximum OLED/compositor compatibility across SteamVR, Oculus, VDXR).
    #[default]
    SrgbFirst,
    /// Prefer 10-bit UNORM (`DXGI_FORMAT_R10G10B10A2_UNORM`, `output_encode_mode = 1.0`) for reduced banding.
    Hdr10BitFirst,
    /// Prefer linear 16-bit floating point (`DXGI_FORMAT_R16G16B16A16_FLOAT`, `output_encode_mode = 2.0`).
    LinearFloatFirst,
}

/// Negotiated D3D12 OpenXR swapchain format paired with `wgpu::TextureFormat` and the exact
/// `post_bloom_agx.wgsl` `grade_params.x` (`output_encode_mode`) value from `shaders/SHADER_CONTRACTS.md`.
///
/// Exact mapping:
/// - `DXGI_FORMAT_R8G8B8A8_UNORM_SRGB` (`29`) -> `Rgba8UnormSrgb`, `output_encode_mode = 0.0` (hardware sRGB)
/// - `DXGI_FORMAT_B8G8R8A8_UNORM_SRGB` (`91`) -> `Bgra8UnormSrgb`, `output_encode_mode = 0.0` (hardware sRGB)
/// - `DXGI_FORMAT_R10G10B10A2_UNORM` (`24`)   -> `Rgb10a2Unorm`,    `output_encode_mode = 1.0` (manual sRGB into UNORM)
/// - `DXGI_FORMAT_R16G16B16A16_FLOAT` (`10`)  -> `Rgba16Float`,     `output_encode_mode = 2.0` (direct linear HDR)
/// - `DXGI_FORMAT_R8G8B8A8_UNORM` (`28`)      -> `Rgba8Unorm`,      `output_encode_mode = 1.0` (manual sRGB into UNORM)
/// - `DXGI_FORMAT_B8G8R8A8_UNORM` (`87`)      -> `Bgra8Unorm`,      `output_encode_mode = 1.0` (manual sRGB into UNORM)
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NegotiatedSwapchainFormat {
    pub dxgi_format: u32,
    pub wgpu_format: wgpu::TextureFormat,
    /// Passed verbatim into `PostPassUniforms.grade_params.x` (`shaders/post_bloom_agx.wgsl`).
    pub output_encode_mode: f32,
    pub name: &'static str,
}

fn descriptor_for_dxgi(dxgi_format: u32) -> Option<NegotiatedSwapchainFormat> {
    match dxgi_format {
        f if f == DXGI_FORMAT_R8G8B8A8_UNORM_SRGB.0 as u32 => Some(NegotiatedSwapchainFormat {
            dxgi_format: f,
            wgpu_format: wgpu::TextureFormat::Rgba8UnormSrgb,
            output_encode_mode: 0.0,
            name: "DXGI_FORMAT_R8G8B8A8_UNORM_SRGB",
        }),
        f if f == DXGI_FORMAT_B8G8R8A8_UNORM_SRGB.0 as u32 => Some(NegotiatedSwapchainFormat {
            dxgi_format: f,
            wgpu_format: wgpu::TextureFormat::Bgra8UnormSrgb,
            output_encode_mode: 0.0,
            name: "DXGI_FORMAT_B8G8R8A8_UNORM_SRGB",
        }),
        f if f == DXGI_FORMAT_R10G10B10A2_UNORM.0 as u32 => Some(NegotiatedSwapchainFormat {
            dxgi_format: f,
            wgpu_format: wgpu::TextureFormat::Rgb10a2Unorm,
            output_encode_mode: 1.0,
            name: "DXGI_FORMAT_R10G10B10A2_UNORM",
        }),
        f if f == DXGI_FORMAT_R16G16B16A16_FLOAT.0 as u32 => Some(NegotiatedSwapchainFormat {
            dxgi_format: f,
            wgpu_format: wgpu::TextureFormat::Rgba16Float,
            output_encode_mode: 2.0,
            name: "DXGI_FORMAT_R16G16B16A16_FLOAT",
        }),
        f if f == DXGI_FORMAT_R8G8B8A8_UNORM.0 as u32 => Some(NegotiatedSwapchainFormat {
            dxgi_format: f,
            wgpu_format: wgpu::TextureFormat::Rgba8Unorm,
            output_encode_mode: 1.0,
            name: "DXGI_FORMAT_R8G8B8A8_UNORM",
        }),
        f if f == DXGI_FORMAT_B8G8R8A8_UNORM.0 as u32 => Some(NegotiatedSwapchainFormat {
            dxgi_format: f,
            wgpu_format: wgpu::TextureFormat::Bgra8Unorm,
            output_encode_mode: 1.0,
            name: "DXGI_FORMAT_B8G8R8A8_UNORM",
        }),
        _ => None,
    }
}

/// Negotiates the preferred D3D12 OpenXR swapchain format and its exact `output_encode_mode`.
pub fn negotiate_swapchain_format(
    supported_dxgi_formats: &[u32],
    pref: SwapchainFormatPreference,
) -> Result<NegotiatedSwapchainFormat, XrError> {
    let r8_srgb = DXGI_FORMAT_R8G8B8A8_UNORM_SRGB.0 as u32;
    let b8_srgb = DXGI_FORMAT_B8G8R8A8_UNORM_SRGB.0 as u32;
    let r10 = DXGI_FORMAT_R10G10B10A2_UNORM.0 as u32;
    let r16f = DXGI_FORMAT_R16G16B16A16_FLOAT.0 as u32;
    let r8_unorm = DXGI_FORMAT_R8G8B8A8_UNORM.0 as u32;
    let b8_unorm = DXGI_FORMAT_B8G8R8A8_UNORM.0 as u32;

    let order: [u32; 6] = match pref {
        SwapchainFormatPreference::SrgbFirst => {
            [r8_srgb, b8_srgb, r10, r16f, r8_unorm, b8_unorm]
        }
        SwapchainFormatPreference::Hdr10BitFirst => {
            [r10, r16f, r8_srgb, b8_srgb, r8_unorm, b8_unorm]
        }
        SwapchainFormatPreference::LinearFloatFirst => {
            [r16f, r10, r8_srgb, b8_srgb, r8_unorm, b8_unorm]
        }
    };

    for candidate in order {
        if supported_dxgi_formats.contains(&candidate)
            && let Some(desc) = descriptor_for_dxgi(candidate)
        {
            return Ok(desc);
        }
    }
    Err(XrError::UnsupportedSwapchainFormat)
}

/// Selects the highest comfortable OLED display refresh rate (`90 Hz` or `120 Hz` preferred).
pub fn select_best_refresh_rate(supported_hz: &[f32], requested_hz: Option<f32>) -> Option<f32> {
    let mut valid: Vec<f32> = supported_hz
        .iter()
        .copied()
        .filter(|r| r.is_finite() && *r > 10.0)
        .collect();
    if valid.is_empty() {
        return None;
    }
    valid.sort_by(|a, b| a.total_cmp(b));

    if let Some(req) = requested_hz
        && let Some(exact) = valid.iter().copied().find(|&r| (r - req).abs() < 0.75)
    {
        return Some(exact);
    }

    // Prefer 120 Hz or 90 Hz when supported, else highest available up to 120 Hz, else max.
    for target in [120.0f32, 90.0f32] {
        if let Some(hit) = valid.iter().copied().find(|&r| (r - target).abs() < 0.75) {
            return Some(hit);
        }
    }
    valid.last().copied()
}

/// Configuration for creating a D3D12 + `wgpu` OpenXR session bridge.
#[derive(Clone, Copy, Debug)]
pub struct XrSessionConfig {
    pub format_preference: SwapchainFormatPreference,
    pub requested_refresh_rate_hz: Option<f32>,
    pub near_plane_render_units: f32,
    pub resolution_scale: f32,
    pub sample_count: u32,
    pub enable_hand_tracking: bool,
}

impl Default for XrSessionConfig {
    fn default() -> Self {
        Self {
            format_preference: SwapchainFormatPreference::SrgbFirst,
            requested_refresh_rate_hz: None,
            near_plane_render_units: DEFAULT_VR_NEAR_RENDER_UNITS,
            resolution_scale: 1.0,
            sample_count: 1,
            enable_hand_tracking: true,
        }
    }
}

/// Wrapped OpenXR 2-layer `ID3D12Resource` swapchain image exposed to `wgpu`.
pub struct XrStereoSwapchainImage {
    /// Holds an AddRef'd COM handle over the runtime-owned `ID3D12Resource`.
    pub texture: wgpu::Texture,
    /// `TextureViewDimension::D2Array` (2 layers) for single-pass `MULTIVIEW` composite.
    pub multiview_view: wgpu::TextureView,
    /// `[left, right]` individual `D2` layer views for fallback per-eye composite.
    pub eye_views: [wgpu::TextureView; 2],
}

/// Per-frame OpenXR timing, stereo Reversed-Z matrices, and active swapchain target index.
#[derive(Clone, Copy, Debug)]
pub struct XrFrameData {
    pub predicted_display_time: xr::Time,
    pub predicted_display_period_ns: i64,
    pub should_render: bool,
    pub swapchain_image_index: Option<usize>,
    pub eye_width: u32,
    pub eye_height: u32,
    pub format: NegotiatedSwapchainFormat,
    pub view_proj: [Mat4; 2],
    pub inv_view_proj: [Mat4; 2],
    pub eye_offset: [Vec4; 2],
    pub head_pose: XrPose,
    pub eye_poses: [XrPose; 2],
    pub eye_fovs: [xr::Fovf; 2],
    pub cam_forward: Vec3,
    pub cam_right: Vec3,
    pub cam_up: Vec3,
    pub center_fov_y_rad: f32,
    pub aspect: f32,
    pub active_refresh_rate_hz: Option<f32>,
}

/// Result of pumping `xrPollEvent`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct XrLifecycleStatus {
    pub session_state: xr::SessionState,
    pub session_running: bool,
    pub should_exit: bool,
    pub focused: bool,
}

/// Decoupled `XR_KHR_D3D12_enable` + `wgpu::hal::api::Dx12` stereo swapchain bridge.
pub struct XrSessionBridge {
    instance: xr::Instance,
    details: XrSystemDetails,
    session: xr::Session<xr::D3D12>,
    frame_waiter: xr::FrameWaiter,
    frame_stream: xr::FrameStream<xr::D3D12>,
    swapchain: xr::Swapchain<xr::D3D12>,
    swapchain_images: Vec<XrStereoSwapchainImage>,
    tracking_space: xr::Space,
    view_space: xr::Space,
    event_buf: xr::EventDataBuffer,
    session_state: xr::SessionState,
    session_running: bool,
    exit_requested: bool,
    image_acquired: bool,
    swapchain_format: NegotiatedSwapchainFormat,
    eye_width: u32,
    eye_height: u32,
    near_plane: f32,
    supported_refresh_rates_hz: Vec<f32>,
    active_refresh_rate_hz: Option<f32>,
    actions: XrActionRig,
    hand_trackers: Option<[xr::HandTracker; 2]>,
}

impl XrSessionBridge {
    /// Creates an `xr::Session<xr::D3D12>` from raw `ID3D12Device` / `ID3D12CommandQueue` pointers
    /// exposed by `cosmos_render` and wraps OpenXR's 2-layer stereo swapchain textures into `wgpu`.
    ///
    /// # Safety
    /// `d3d12_device_ptr` must be a valid `*mut ID3D12Device` belonging to `wgpu_device` on adapter
    /// LUID `details.adapter_luid`, and `d3d12_queue_ptr` must be its primary graphics `*mut ID3D12CommandQueue`.
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn create_d3d12(
        instance: xr::Instance,
        system_id: xr::SystemId,
        details: &XrSystemDetails,
        d3d12_device_ptr: *mut c_void,
        d3d12_queue_ptr: *mut c_void,
        wgpu_device: &wgpu::Device,
        config: XrSessionConfig,
    ) -> Result<Self, XrError> {
        if d3d12_device_ptr.is_null() {
            return Err(XrError::NullNativeHandle("ID3D12Device"));
        }
        if d3d12_queue_ptr.is_null() {
            return Err(XrError::NullNativeHandle("ID3D12CommandQueue"));
        }

        let create_info = xr::d3d::SessionCreateInfoD3D12 {
            device: d3d12_device_ptr.cast(),
            queue: d3d12_queue_ptr.cast(),
        };

        let (session, frame_waiter, frame_stream) =
            unsafe { instance.create_session::<xr::D3D12>(system_id, &create_info)? };

        let supported_formats = session.enumerate_swapchain_formats()?;
        let swapchain_format =
            negotiate_swapchain_format(&supported_formats, config.format_preference)?;

        let scale = config.resolution_scale.clamp(0.5, 1.5);
        let base_w = details.recommended_eye_res[0].width.max(64);
        let base_h = details.recommended_eye_res[0].height.max(64);
        let eye_width = ((base_w as f32) * scale)
            .round()
            .clamp(64.0, details.recommended_eye_res[0].max_width as f32)
            as u32;
        let eye_height = ((base_h as f32) * scale)
            .round()
            .clamp(64.0, details.recommended_eye_res[0].max_height as f32)
            as u32;

        let sc_info = xr::SwapchainCreateInfo {
            create_flags: xr::SwapchainCreateFlags::EMPTY,
            usage_flags: xr::SwapchainUsageFlags::COLOR_ATTACHMENT
                | xr::SwapchainUsageFlags::SAMPLED,
            format: swapchain_format.dxgi_format,
            sample_count: config.sample_count.max(1),
            width: eye_width,
            height: eye_height,
            face_count: 1,
            array_size: 2,
            mip_count: 1,
        };
        let swapchain = session.create_swapchain(&sc_info)?;
        let raw_images = swapchain.enumerate_images()?;

        let mut swapchain_images = Vec::with_capacity(raw_images.len());
        let tex_size = wgpu::Extent3d {
            width: eye_width,
            height: eye_height,
            depth_or_array_layers: 2,
        };
        let tex_desc = wgpu::TextureDescriptor {
            label: Some("OpenXR Stereo D2Array Swapchain"),
            size: tex_size,
            mip_level_count: 1,
            sample_count: config.sample_count.max(1),
            dimension: wgpu::TextureDimension::D2,
            format: swapchain_format.wgpu_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };

        for raw_ptr in raw_images {
            let void_ptr: *mut c_void = raw_ptr.cast();
            // Borrow raw ID3D12Resource and clone to increment COM refcount so both
            // `wgpu::Texture` drop and `xr::Swapchain` destruction remain balanced.
            let borrowed = unsafe { ID3D12Resource::from_raw_borrowed(&void_ptr) }
                .ok_or(XrError::NullNativeHandle("ID3D12Resource"))?;
            let owned_com: ID3D12Resource = borrowed.clone();

            let hal_tex = unsafe {
                wgpu::hal::dx12::Device::texture_from_raw(
                    owned_com,
                    swapchain_format.wgpu_format,
                    wgpu::TextureDimension::D2,
                    tex_size,
                    1,
                    config.sample_count.max(1),
                )
            };
            let texture = unsafe {
                wgpu_device.create_texture_from_hal::<wgpu::hal::api::Dx12>(
                    hal_tex,
                    &tex_desc,
                    wgpu::TextureUses::COLOR_TARGET,
                )
            };
            let multiview_view = texture.create_view(&wgpu::TextureViewDescriptor {
                label: Some("OpenXR Multiview D2Array View"),
                dimension: Some(wgpu::TextureViewDimension::D2Array),
                base_array_layer: 0,
                array_layer_count: Some(2),
                ..Default::default()
            });
            let left_view = texture.create_view(&wgpu::TextureViewDescriptor {
                label: Some("OpenXR Eye Left View"),
                dimension: Some(wgpu::TextureViewDimension::D2),
                base_array_layer: 0,
                array_layer_count: Some(1),
                ..Default::default()
            });
            let right_view = texture.create_view(&wgpu::TextureViewDescriptor {
                label: Some("OpenXR Eye Right View"),
                dimension: Some(wgpu::TextureViewDimension::D2),
                base_array_layer: 1,
                array_layer_count: Some(1),
                ..Default::default()
            });
            swapchain_images.push(XrStereoSwapchainImage {
                texture,
                multiview_view,
                eye_views: [left_view, right_view],
            });
        }

        let tracking_space = session
            .create_reference_space(xr::ReferenceSpaceType::LOCAL, xr::Posef::IDENTITY)?;
        let view_space =
            session.create_reference_space(xr::ReferenceSpaceType::VIEW, xr::Posef::IDENTITY)?;

        let mut supported_refresh_rates_hz = Vec::new();
        let mut active_refresh_rate_hz = None;
        if details.extensions.fb_display_refresh_rate
            && let Ok(rates) = session.enumerate_display_refresh_rates()
        {
            supported_refresh_rates_hz = rates.clone();
            if let Some(best) = select_best_refresh_rate(&rates, config.requested_refresh_rate_hz) {
                let _ = session.request_display_refresh_rate(best);
            }
            active_refresh_rate_hz = session.get_display_refresh_rate().ok();
        }

        let actions = XrActionRig::attach(&instance, &session)?;
        let hand_trackers = if config.enable_hand_tracking && details.extensions.ext_hand_tracking {
            match (
                session.create_hand_tracker(xr::Hand::LEFT),
                session.create_hand_tracker(xr::Hand::RIGHT),
            ) {
                (Ok(left), Ok(right)) => Some([left, right]),
                _ => None,
            }
        } else {
            None
        };

        let mut updated_details = details.clone();
        updated_details.supported_refresh_rates_hz = supported_refresh_rates_hz.clone();

        Ok(Self {
            instance,
            details: updated_details,
            session,
            frame_waiter,
            frame_stream,
            swapchain,
            swapchain_images,
            tracking_space,
            view_space,
            event_buf: xr::EventDataBuffer::new(),
            session_state: xr::SessionState::UNKNOWN,
            session_running: false,
            exit_requested: false,
            image_acquired: false,
            swapchain_format,
            eye_width,
            eye_height,
            near_plane: config.near_plane_render_units.max(1e-4),
            supported_refresh_rates_hz,
            active_refresh_rate_hz,
            actions,
            hand_trackers,
        })
    }

    #[inline]
    pub fn details(&self) -> &XrSystemDetails {
        &self.details
    }

    #[inline]
    pub fn swapchain_format(&self) -> NegotiatedSwapchainFormat {
        self.swapchain_format
    }

    #[inline]
    pub fn eye_resolution(&self) -> (u32, u32) {
        (self.eye_width, self.eye_height)
    }

    #[inline]
    pub fn refresh_rates_hz(&self) -> &[f32] {
        &self.supported_refresh_rates_hz
    }

    #[inline]
    pub fn active_refresh_rate_hz(&self) -> Option<f32> {
        self.active_refresh_rate_hz
    }

    /// Pumps OpenXR runtime events and manages `READY -> begin()` and `STOPPING -> end()` transitions.
    pub fn poll_events(&mut self) -> Result<XrLifecycleStatus, XrError> {
        while let Some(event) = self.instance.poll_event(&mut self.event_buf)? {
            match event {
                xr::Event::SessionStateChanged(e) => {
                    self.session_state = e.state();
                    match self.session_state {
                        xr::SessionState::READY => {
                            if !self.session_running {
                                self.session
                                    .begin(xr::ViewConfigurationType::PRIMARY_STEREO)?;
                                self.session_running = true;
                            }
                        }
                        xr::SessionState::STOPPING => {
                            if self.session_running {
                                self.session.end()?;
                                self.session_running = false;
                            }
                        }
                        xr::SessionState::EXITING | xr::SessionState::LOSS_PENDING => {
                            self.exit_requested = true;
                        }
                        _ => {}
                    }
                }
                xr::Event::InstanceLossPending(_) => {
                    self.exit_requested = true;
                }
                xr::Event::DisplayRefreshRateChangedFB(e) => {
                    self.active_refresh_rate_hz = Some(e.to_display_refresh_rate());
                }
                _ => {}
            }
        }

        Ok(XrLifecycleStatus {
            session_state: self.session_state,
            session_running: self.session_running,
            should_exit: self.exit_requested,
            focused: self.session_state == xr::SessionState::FOCUSED,
        })
    }

    /// Waits for the next OpenXR frame, acquires and waits the stereo array swapchain image if
    /// `should_render`, begins the frame stream, and builds camera-relative Reversed-Z stereo matrices.
    pub fn begin_frame(
        &mut self,
        world_rig_orient: Quat,
    ) -> Result<Option<XrFrameData>, XrError> {
        if !self.session_running {
            return Ok(None);
        }

        let state = self.frame_waiter.wait()?;
        let image_index = if state.should_render {
            let idx = self.swapchain.acquire_image()? as usize;
            self.swapchain.wait_image(xr::Duration::INFINITE)?;
            self.image_acquired = true;
            Some(idx)
        } else {
            self.image_acquired = false;
            None
        };

        self.frame_stream.begin()?;

        let (flags, views) = self.session.locate_views(
            xr::ViewConfigurationType::PRIMARY_STEREO,
            state.predicted_display_time,
            &self.tracking_space,
        )?;
        if views.len() < 2 {
            return Err(XrError::InvalidEyeCount(views.len()));
        }

        let valid = flags.contains(
            xr::ViewStateFlags::POSITION_VALID | xr::ViewStateFlags::ORIENTATION_VALID,
        );
        let raw_poses = [views[0].pose, views[1].pose];
        let raw_fovs = [views[0].fov, views[1].fov];

        let mut stereo = compute_stereo_matrices(
            &raw_poses,
            &raw_fovs,
            self.eye_width,
            self.eye_height,
            self.near_plane,
            world_rig_orient,
        );
        stereo.head_pose.valid = valid;
        stereo.eye_poses[0].valid = valid;
        stereo.eye_poses[1].valid = valid;

        Ok(Some(XrFrameData {
            predicted_display_time: state.predicted_display_time,
            predicted_display_period_ns: state.predicted_display_period.as_nanos(),
            should_render: state.should_render,
            swapchain_image_index: image_index,
            eye_width: self.eye_width,
            eye_height: self.eye_height,
            format: self.swapchain_format,
            view_proj: stereo.view_proj,
            inv_view_proj: stereo.inv_view_proj,
            eye_offset: stereo.eye_offset,
            head_pose: stereo.head_pose,
            eye_poses: stereo.eye_poses,
            eye_fovs: raw_fovs,
            cam_forward: stereo.cam_forward,
            cam_right: stereo.cam_right,
            cam_up: stereo.cam_up,
            center_fov_y_rad: stereo.center_fov_y_rad,
            aspect: stereo.aspect,
            active_refresh_rate_hz: self.active_refresh_rate_hz,
        }))
    }

    #[inline]
    pub fn current_swapchain_image(&self, frame: &XrFrameData) -> Option<&XrStereoSwapchainImage> {
        frame
            .swapchain_image_index
            .and_then(|idx| self.swapchain_images.get(idx))
    }

    /// Polls controller poses/inputs + `EXT_hand_tracking` joints for the active frame.
    pub fn poll_input(&self, frame: &XrFrameData, view_up: Vec3) -> Result<XrInputFrame, XrError> {
        if self.session_state != xr::SessionState::FOCUSED {
            return Ok(XrInputFrame::default());
        }
        self.actions.poll_input(
            &self.session,
            &self.tracking_space,
            frame.predicted_display_time,
            view_up,
            self.hand_trackers.as_ref(),
        )
    }

    /// Fires a brief tactile vibration pulse on the left or right controller.
    pub fn apply_haptic(
        &self,
        hand: crate::input::XrHand,
        amplitude: f32,
        duration_ns: i64,
        freq_hz: f32,
    ) -> Result<(), XrError> {
        self.actions
            .apply_haptic(&self.session, hand, amplitude, duration_ns, freq_hz)
    }

    /// Releases the swapchain image and submits stereo projection composition layers to OpenXR.
    pub fn end_frame(&mut self, frame: &XrFrameData) -> Result<(), XrError> {
        if self.image_acquired {
            self.swapchain.release_image()?;
            self.image_acquired = false;
        }

        if frame.should_render && frame.swapchain_image_index.is_some() {
            let rect = xr::Rect2Di {
                offset: xr::Offset2Di { x: 0, y: 0 },
                extent: xr::Extent2Di {
                    width: self.eye_width as i32,
                    height: self.eye_height as i32,
                },
            };

            let left_pose = to_xr_posef(frame.eye_poses[0]);
            let right_pose = to_xr_posef(frame.eye_poses[1]);

            let views = [
                xr::CompositionLayerProjectionView::new()
                    .pose(left_pose)
                    .fov(frame.eye_fovs[0])
                    .sub_image(
                        xr::SwapchainSubImage::new()
                            .swapchain(&self.swapchain)
                            .image_array_index(0)
                            .image_rect(rect),
                    ),
                xr::CompositionLayerProjectionView::new()
                    .pose(right_pose)
                    .fov(frame.eye_fovs[1])
                    .sub_image(
                        xr::SwapchainSubImage::new()
                            .swapchain(&self.swapchain)
                            .image_array_index(1)
                            .image_rect(rect),
                    ),
            ];
            let layer = xr::CompositionLayerProjection::new()
                .space(&self.tracking_space)
                .views(&views);
            self.frame_stream.end(
                frame.predicted_display_time,
                xr::EnvironmentBlendMode::OPAQUE,
                &[&*layer],
            )?;
        } else {
            self.frame_stream.end(
                frame.predicted_display_time,
                xr::EnvironmentBlendMode::OPAQUE,
                &[],
            )?;
        }

        let _ = &self.view_space;
        Ok(())
    }

    /// Requests clean session exit so the runtime transitions to `STOPPING`.
    pub fn request_exit(&self) -> Result<(), XrError> {
        self.session.request_exit()?;
        Ok(())
    }
}

#[inline]
fn to_xr_posef(pose: XrPose) -> xr::Posef {
    xr::Posef {
        orientation: xr::Quaternionf {
            x: pose.orientation.x,
            y: pose.orientation.y,
            z: pose.orientation.z,
            w: pose.orientation.w,
        },
        position: xr::Vector3f {
            x: pose.position.x,
            y: pose.position.y,
            z: pose.position.z,
        },
    }
}
