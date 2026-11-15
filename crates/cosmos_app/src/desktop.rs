use std::collections::HashSet;
use std::sync::Arc;
use std::time::Instant;

use anyhow::{Context, Result};
use cosmos_audio::AudioOutputMode;
use cosmos_math::DEFAULT_DESKTOP_NEAR_RENDER_UNITS;
use cosmos_nav::DesktopFlightInput;
use cosmos_render::{CosmosRenderer, DEPTH_FORMAT, HDR_SCENE_FORMAT, RenderCamera};
use cosmos_typo::{MsdfPipelineConfig, MsdfTextRenderer, TextPassUniforms};
use glam::{DVec2, DVec3, Vec3};
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::{DeviceEvent, DeviceId, ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::{Window, WindowAttributes, WindowId};

use crate::assets::UniverseAssets;
use crate::engine::{LaunchJourneyConfig, UniverseRuntime};
use crate::guide::{GuideAction, GuidePage};

struct GpuSurfaceState {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    renderer: CosmosRenderer,
    text_renderer: MsdfTextRenderer,
}

struct DesktopApp {
    assets_shaders_dir: std::path::PathBuf,
    runtime: UniverseRuntime,
    gpu: Option<GpuSurfaceState>,
    pressed_keys: HashSet<KeyCode>,
    left_down: bool,
    right_down: bool,
    left_click_edge: bool,
    tab_just_pressed: bool,
    scroll_accum: f64,
    mouse_delta: DVec2,
    cursor_norm: [f32; 2],
    last_frame: Instant,
    prefer_hdr_swapchain: bool,
}

impl DesktopApp {
    fn new(prefer_hdr_swapchain: bool, journey_cfg: &LaunchJourneyConfig) -> Result<Self> {
        let assets = UniverseAssets::load()?;
        let shaders_dir = assets.shaders_dir.clone();
        let mut runtime = UniverseRuntime::new(assets, AudioOutputMode::Auto)?;
        journey_cfg.apply(&mut runtime)?;
        runtime.enable_spatial_guide(false);
        Ok(Self {
            assets_shaders_dir: shaders_dir,
            runtime,
            gpu: None,
            pressed_keys: HashSet::new(),
            left_down: false,
            right_down: false,
            left_click_edge: false,
            tab_just_pressed: false,
            scroll_accum: 0.0,
            mouse_delta: DVec2::ZERO,
            cursor_norm: [0.0, 0.0],
            last_frame: Instant::now(),
            prefer_hdr_swapchain,
        })
    }

    fn render_next_frame(&mut self) {
        let Some(ref mut gpu) = self.gpu else {
            return;
        };

        let now = Instant::now();
        let dt_sec = (now - self.last_frame).as_secs_f32().clamp(0.0005, 0.05);
        self.last_frame = now;

        let width = gpu.config.width.max(1);
        let height = gpu.config.height.max(1);
        let aspect = width as f32 / height as f32;
        let fov_y_rad = 72.0_f32.to_radians();

        let key = |k: KeyCode| self.pressed_keys.contains(&k);
        let ctrl_held = key(KeyCode::ControlLeft) || key(KeyCode::ControlRight);
        let x_move = if ctrl_held {
            0.0
        } else {
            (key(KeyCode::KeyD) as i32 - key(KeyCode::KeyA) as i32) as f64
        };
        let y_move = if ctrl_held {
            0.0
        } else {
            (key(KeyCode::KeyE) as i32 - key(KeyCode::KeyQ) as i32) as f64
        };
        // `+move_axes.z` pushes camera forward along `-Z` inside FlightCamera::step_desktop.
        let z_move = if ctrl_held {
            0.0
        } else {
            (key(KeyCode::KeyW) as i32 - key(KeyCode::KeyS) as i32) as f64
        };
        let roll = (key(KeyCode::KeyZ) as i32 - key(KeyCode::KeyC) as i32) as f64;

        let desktop_input = DesktopFlightInput {
            scroll_delta: self.scroll_accum,
            left_drag_active: self.left_down,
            right_drag_active: false,
            free_flight_held: key(KeyCode::KeyF),
            mouse_delta_rad: DVec2::new(
                self.mouse_delta.x * 0.0032,
                self.mouse_delta.y * 0.0032,
            ),
            pan_delta_cam: DVec3::ZERO,
            move_axes: DVec3::new(x_move, y_move, z_move),
            roll_axis: roll,
            boost: key(KeyCode::ShiftLeft) || key(KeyCode::ShiftRight),
            home_pressed: key(KeyCode::Home),
        };
        self.scroll_accum = 0.0;
        self.mouse_delta = DVec2::ZERO;

        let hover_ray = unproject_cursor_ray_cam(self.cursor_norm, fov_y_rad, aspect);
        let orient = self.runtime.flight.pose.orient.as_quat();
        let camera_pre = RenderCamera::mono(
            orient,
            fov_y_rad,
            aspect,
            DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
        );

        let pkt = self.runtime.step_desktop_frame(
            &desktop_input,
            hover_ray,
            self.right_down,
            self.tab_just_pressed,
            self.left_click_edge,
            &camera_pre,
            dt_sec,
            width,
            height,
        );
        self.tab_just_pressed = false;
        self.left_click_edge = false;

        let (frame_tex, suboptimal) = match gpu.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => (frame, false),
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => (frame, true),
            wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
                gpu.surface.configure(gpu.renderer.device(), &gpu.config);
                return;
            }
            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => return,
        };

        let view = frame_tex
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        gpu.text_renderer.upload(
            gpu.renderer.device(),
            gpu.renderer.queue(),
            &TextPassUniforms::default(),
            &pkt.typo.glyphs,
            &pkt.typo.tethers,
        );
        gpu.text_renderer
            .set_foreground_start(pkt.typo.foreground_start);

        let target_format = gpu.config.format;
        gpu.renderer.render_frame(
            &pkt.scene,
            &view,
            target_format,
            width,
            height,
            false,
            |rpass, stereo| gpu.text_renderer.render(rpass, None, stereo),
        );
        gpu.renderer.queue().present(frame_tex);
        if suboptimal {
            gpu.surface.configure(gpu.renderer.device(), &gpu.config);
        }
    }
}

impl ApplicationHandler for DesktopApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.gpu.is_some() {
            return;
        }
        let attrs = WindowAttributes::default()
            .with_title("HyperCosmosVR — Desktop HDR (RTX 5070)")
            .with_inner_size(PhysicalSize::new(1280, 720));
        let window = match event_loop.create_window(attrs) {
            Ok(w) => Arc::new(w),
            Err(e) => {
                eprintln!("[cosmos_app] failed to create window: {e}");
                event_loop.exit();
                return;
            }
        };

        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let renderer = match CosmosRenderer::init_desktop(width, height) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("[cosmos_app] RTX 5070 renderer init failed: {e}");
                event_loop.exit();
                return;
            }
        };

        let surface = match renderer.instance().create_surface(Arc::clone(&window)) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("[cosmos_app] DX12 window surface creation failed: {e}");
                event_loop.exit();
                return;
            }
        };

        let caps = surface.get_capabilities(renderer.adapter());
        let format = match pick_surface_format(&caps.formats, self.prefer_hdr_swapchain) {
            Ok(f) => f,
            Err(e) => {
                eprintln!("[cosmos_app] surface format selection failed: {e}");
                event_loop.exit();
                return;
            }
        };
        let present_mode = if caps.present_modes.contains(&wgpu::PresentMode::Mailbox) {
            wgpu::PresentMode::Mailbox
        } else if caps.present_modes.contains(&wgpu::PresentMode::Fifo) {
            wgpu::PresentMode::Fifo
        } else {
            match caps
                .present_modes
                .first()
                .copied()
                .context("desktop window surface reported no supported present modes")
            {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("[cosmos_app] {e}");
                    event_loop.exit();
                    return;
                }
            }
        };
        let alpha_mode = match caps
            .alpha_modes
            .iter()
            .copied()
            .find(|m| *m == wgpu::CompositeAlphaMode::Opaque)
            .or_else(|| caps.alpha_modes.first().copied())
            .context("desktop window surface reported no supported composite alpha modes")
        {
            Ok(m) => m,
            Err(e) => {
                eprintln!("[cosmos_app] {e}");
                event_loop.exit();
                return;
            }
        };

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            width,
            height,
            present_mode,
            desired_maximum_frame_latency: 1,
            alpha_mode,
            view_formats: vec![],
        };
        surface.configure(renderer.device(), &config);

        let text_renderer = match MsdfTextRenderer::from_shaders_dir(
            renderer.device(),
            renderer.queue(),
            &self.runtime.atlas,
            &self.assets_shaders_dir,
            renderer.shared_frame_bind_group_layout(),
            MsdfPipelineConfig {
                color_format: HDR_SCENE_FORMAT,
                depth_format: Some(DEPTH_FORMAT),
                enable_stereo_multiview: true,
                sample_count: 1,
            },
        ) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("[cosmos_app] MSDF text renderer init failed: {e}");
                event_loop.exit();
                return;
            }
        };

        eprintln!(
            "[HyperCosmosVR] Cold start ready in Earth Low Orbit (u=+6.6) on '{}' ({:?})",
            renderer.adapter_info().name,
            config.format
        );

        self.gpu = Some(GpuSurfaceState {
            window,
            surface,
            config,
            renderer,
            text_renderer,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Focused(false) => {
                self.pressed_keys.clear();
                self.left_down = false;
                self.right_down = false;
                self.left_click_edge = false;
                self.tab_just_pressed = false;
                self.scroll_accum = 0.0;
                self.mouse_delta = DVec2::ZERO;
                self.runtime.pause_for_focus_loss();
            }
            WindowEvent::Resized(new_size) => {
                if let Some(ref mut gpu) = self.gpu
                    && new_size.width > 0
                    && new_size.height > 0
                {
                    gpu.config.width = new_size.width;
                    gpu.config.height = new_size.height;
                    gpu.surface.configure(gpu.renderer.device(), &gpu.config);
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(ref gpu) = self.gpu {
                    let w = gpu.config.width.max(1) as f32;
                    let h = gpu.config.height.max(1) as f32;
                    self.cursor_norm = [
                        (position.x as f32 / w) * 2.0 - 1.0,
                        1.0 - (position.y as f32 / h) * 2.0,
                    ];
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                let pressed = state == ElementState::Pressed;
                match button {
                    MouseButton::Left => {
                        if pressed && !self.left_down {
                            self.left_click_edge = true;
                        }
                        self.left_down = pressed;
                    }
                    MouseButton::Right => {
                        self.right_down = pressed;
                    }
                    _ => {}
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let steps = match delta {
                    MouseScrollDelta::LineDelta(_x, y) => y as f64,
                    MouseScrollDelta::PixelDelta(pos) => pos.y / 64.0,
                };
                self.scroll_accum += steps;
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let PhysicalKey::Code(code) = event.physical_key {
                    let pressed = event.state == ElementState::Pressed;
                    if pressed {
                        if !self.pressed_keys.contains(&code) {
                            let shift_held = self.pressed_keys.contains(&KeyCode::ShiftLeft)
                                || self.pressed_keys.contains(&KeyCode::ShiftRight);
                            let ctrl_held = self.pressed_keys.contains(&KeyCode::ControlLeft)
                                || self.pressed_keys.contains(&KeyCode::ControlRight);
                            match code {
                                KeyCode::Escape => {
                                    if self.runtime.guide_page != GuidePage::Hidden {
                                        self.runtime.perform_guide_action(GuideAction::CloseHelp);
                                    } else if self.runtime.has_tour_controls() {
                                        self.runtime.perform_guide_action(GuideAction::Explore);
                                    } else {
                                        self.runtime.toggle_spatial_help();
                                    }
                                }
                                KeyCode::F1 => self.runtime.toggle_spatial_help(),
                                KeyCode::Space if self.runtime.has_tour_controls() => {
                                    self.runtime.toggle_journey_play_pause()
                                }
                                KeyCode::Tab => self.tab_just_pressed = true,
                                KeyCode::KeyL => self.runtime.toggle_locale(),
                                KeyCode::BracketRight | KeyCode::Equal => {
                                    self.runtime.step_playback_rate(1);
                                }
                                KeyCode::BracketLeft | KeyCode::Minus => {
                                    self.runtime.step_playback_rate(-1);
                                }
                                KeyCode::Home => {
                                    self.runtime.journey_confirm_exit();
                                    self.runtime.trigger_home();
                                }
                                KeyCode::KeyJ => self.runtime.toggle_journey_play_pause(),
                                KeyCode::KeyN | KeyCode::PageDown => {
                                    if shift_held {
                                        self.runtime.journey_next_chapter();
                                    } else {
                                        self.runtime.journey_next_beat();
                                    }
                                }
                                KeyCode::KeyP | KeyCode::PageUp => {
                                    if shift_held {
                                        self.runtime.journey_prev_chapter();
                                    } else {
                                        self.runtime.journey_prev_beat();
                                    }
                                }
                                KeyCode::KeyR => {
                                    if shift_held {
                                        let _ = self.runtime.toggle_live_trace_recording();
                                    } else {
                                        let _ = self.runtime.record_manual_waypoint();
                                    }
                                }
                                KeyCode::KeyS if ctrl_held => {
                                    let _ = self.runtime.export_recorded_journey_interactive();
                                }
                                KeyCode::F9 => {
                                    let _ = self.runtime.export_recorded_journey_interactive();
                                }
                                KeyCode::Enter => self.runtime.journey_confirm_follow(),
                                KeyCode::Backspace => self.runtime.journey_confirm_exit(),
                                _ => {}
                            }
                        }
                        self.pressed_keys.insert(code);
                    } else {
                        self.pressed_keys.remove(&code);
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                self.render_next_frame();
            }
            _ => {}
        }
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: DeviceId,
        event: DeviceEvent,
    ) {
        if let DeviceEvent::MouseMotion { delta: (dx, dy) } = event
            && (self.left_down || self.right_down || self.pressed_keys.contains(&KeyCode::KeyF))
        {
            self.mouse_delta.x += dx;
            self.mouse_delta.y += dy;
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(ref gpu) = self.gpu {
            gpu.window.request_redraw();
        }
    }
}

/// Launches the interactive Desktop HDR 6DoF window directly into Earth Low Orbit (`u=+6.6`).
pub fn run_desktop_loop(prefer_hdr_swapchain: bool) -> Result<()> {
    run_desktop_loop_with_journey(prefer_hdr_swapchain, LaunchJourneyConfig::default())
}

/// Launches the interactive Desktop HDR 6DoF window with optional Story-Journey CLI configuration.
pub fn run_desktop_loop_with_journey(
    prefer_hdr_swapchain: bool,
    journey_cfg: LaunchJourneyConfig,
) -> Result<()> {
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = DesktopApp::new(prefer_hdr_swapchain, &journey_cfg)?;
    event_loop.run_app(&mut app)?;
    Ok(())
}

fn unproject_cursor_ray_cam(ndc: [f32; 2], fov_y_rad: f32, aspect: f32) -> Vec3 {
    let half_h = (0.5 * fov_y_rad).tan();
    let half_w = half_h * aspect.max(0.1);
    Vec3::new(ndc[0] * half_w, ndc[1] * half_h, -1.0).normalize()
}

fn pick_surface_format(
    formats: &[wgpu::TextureFormat],
    prefer_hdr: bool,
) -> Result<wgpu::TextureFormat> {
    let order: &[wgpu::TextureFormat] = if prefer_hdr {
        &[
            wgpu::TextureFormat::Rgba16Float,
            wgpu::TextureFormat::Bgra8UnormSrgb,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::TextureFormat::Rgba8Unorm,
        ]
    } else {
        &[
            wgpu::TextureFormat::Bgra8UnormSrgb,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            wgpu::TextureFormat::Rgba16Float,
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::TextureFormat::Rgba8Unorm,
        ]
    };

    for fmt in order {
        if formats.contains(fmt) {
            return Ok(*fmt);
        }
    }
    formats
        .first()
        .copied()
        .context("desktop window surface reported no supported texture formats")
}
