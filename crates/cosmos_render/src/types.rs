use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Quat, Vec3};

/// 16-byte aligned global `@group(0) @binding(0)` uniform buffer (432 bytes).
/// Matches `shaders/common_oled.wgsl` (`FrameUniforms`) byte-for-byte.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct FrameUniforms {
    pub view_proj: [[[f32; 4]; 4]; 2],
    pub inv_view_proj: [[[f32; 4]; 4]; 2],
    pub eye_offset: [[f32; 4]; 2],
    pub cam_forward: [f32; 4],
    pub cam_right: [f32; 4],
    pub cam_up: [f32; 4],
    pub scale_params: [f32; 4],
    pub post_params: [f32; 4],
    pub viewport_params: [f32; 4],
    pub stereo_params: [f32; 4],
    pub focus_ray: [f32; 4],
    pub region_accent: [f32; 4],
}

const _: () = assert!(std::mem::size_of::<FrameUniforms>() == 432);

impl Default for FrameUniforms {
    fn default() -> Self {
        let id = Mat4::IDENTITY.to_cols_array_2d();
        Self {
            view_proj: [id, id],
            inv_view_proj: [id, id],
            eye_offset: [[0.0; 4]; 2],
            cam_forward: [0.0, 0.0, -1.0, std::f32::consts::FRAC_PI_2],
            cam_right: [1.0, 0.0, 0.0, 16.0 / 9.0],
            cam_up: [0.0, 1.0, 0.0, 0.02],
            scale_params: [6.8, 0.0, 1.0, 0.0],
            post_params: [1.05, 0.14, 0.004, 1.0],
            viewport_params: [1280.0, 720.0, 1.0 / 1280.0, 1.0 / 720.0],
            stereo_params: [1.0, 0.34, 0.0, 0.0],
            focus_ray: [0.0, 0.0, -1.0, 0.0],
            region_accent: [0.25, 0.68, 1.25, 3.0],
        }
    }
}

/// 16-byte aligned `@group(0) @binding(1)` real-time audio reactive bus (64 bytes).
/// Matches `shaders/common_oled.wgsl` (`AudioReactiveUniforms`) byte-for-byte.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct AudioReactiveUniforms {
    /// `[voice_rms, ambient_rms, bass_energy, mid_energy]`
    pub levels_a: [f32; 4],
    /// `[harmonic_centroid, beat_phase, high_energy, playback_rate]`
    pub levels_b: [f32; 4],
    /// `[drone_base_hz, synth_filter_hz, synth_pulse, drone_noise]`
    pub synth_a: [f32; 4],
    /// `[active_note_hue, active_timbre_id, music_energy, resonance_glow]`
    pub synth_b: [f32; 4],
}

const _: () = assert!(std::mem::size_of::<AudioReactiveUniforms>() == 64);

impl Default for AudioReactiveUniforms {
    fn default() -> Self {
        Self {
            levels_a: [0.0; 4],
            levels_b: [0.5, 0.0, 0.0, 1.0],
            synth_a: [55.0, 440.0, 0.0, 0.0],
            synth_b: [0.55, 0.0, 0.0, 0.0],
        }
    }
}

/// Compute dispatch uniform (`compute_universe.wgsl` `@group(1) @binding(0)`, 80 bytes).
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SimDispatchUniforms {
    pub params_a: [f32; 4],
    pub params_b: [f32; 4],
    pub attractor_a: [f32; 4],
    pub attractor_b: [f32; 4],
    pub counts: [u32; 4],
}

const _: () = assert!(std::mem::size_of::<SimDispatchUniforms>() == 80);

impl Default for SimDispatchUniforms {
    fn default() -> Self {
        Self {
            params_a: [1.0 / 90.0, 0.0, 0.94, 1.0],
            params_b: [4.0, 2.2, 0.85, 6.0],
            attractor_a: [0.0, 0.0, 0.0, 1.0],
            attractor_b: [1.5, 0.4, -1.2, 0.6],
            counts: [0, 0, 256, 256],
        }
    }
}

/// Shared 64-byte particle record read/written by `compute_universe.wgsl` and rendered by `instanced_sprites.wgsl`.
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SimParticle {
    pub pos_age: [f32; 4],
    pub vel_mass: [f32; 4],
    pub color_size: [f32; 4],
    pub tags: [u32; 4],
}

const _: () = assert!(std::mem::size_of::<SimParticle>() == 64);

/// Infinite sky shell uniform (`far_shell.wgsl` `@group(1) @binding(0)`, 64 bytes).
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct FarShellUniforms {
    pub regime_weights: [f32; 4],
    pub galaxy_north: [f32; 4],
    pub solar_dir: [f32; 4],
    pub extra_weights: [f32; 4],
}

const _: () = assert!(std::mem::size_of::<FarShellUniforms>() == 64);

impl Default for FarShellUniforms {
    fn default() -> Self {
        let north = Vec3::new(0.18, 0.96, 0.21).normalize();
        let sun = Vec3::new(0.62, 0.48, -0.62).normalize();
        Self {
            regime_weights: [0.55, 0.0, 0.0, 0.0],
            galaxy_north: [north.x, north.y, north.z, 1.0],
            solar_dir: [sun.x, sun.y, sun.z, 0.35],
            extra_weights: [0.0, 0.0, 1.0, 0.0],
        }
    }
}

/// Raymarched / analytical volumetric hero instance (`hero_volumetric.wgsl`, 112 bytes).
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct HeroVolumeInstance {
    pub pos_radius: [f32; 4],
    pub rot_quat: [f32; 4],
    pub color_a: [f32; 4],
    pub color_b: [f32; 4],
    pub phys_a: [f32; 4],
    pub phys_b: [f32; 4],
    pub kind_tags: [u32; 4],
}

const _: () = assert!(std::mem::size_of::<HeroVolumeInstance>() == 112);

/// Batch parameters for `instanced_sprites.wgsl` (`@group(1) @binding(0)`, 32 bytes).
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SpriteBatchUniforms {
    pub sector_origin_scale: [f32; 4],
    pub batch_params: [f32; 4],
}

const _: () = assert!(std::mem::size_of::<SpriteBatchUniforms>() == 32);

impl Default for SpriteBatchUniforms {
    fn default() -> Self {
        Self {
            sector_origin_scale: [0.0, 0.0, 0.0, 1.0],
            batch_params: [0.0, 1.25, 1.0, 0.0],
        }
    }
}

/// Explicit 64-byte world-space or diegetic UI sprite (`instanced_sprites.wgsl`).
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct SpriteInstance {
    pub pos_size: [f32; 4],
    pub axis_aspect: [f32; 4],
    pub color_alpha: [f32; 4],
    pub tags_f: [f32; 4],
}

const _: () = assert!(std::mem::size_of::<SpriteInstance>() == 64);

/// Post-process pass uniform (`post_bloom_agx.wgsl` `@group(1) @binding(0)`, 32 bytes).
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct PostPassUniforms {
    pub texel_scale: [f32; 4],
    pub grade_params: [f32; 4],
}

const _: () = assert!(std::mem::size_of::<PostPassUniforms>() == 32);

/// Unit-cube vertex (`hero_volumetric.wgsl`, 12 bytes, `Float32x3`).
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct HeroVertex {
    pub local_pos: [f32; 3],
}

const _: () = assert!(std::mem::size_of::<HeroVertex>() == 12);

/// 8 corners of `[-1, 1]^3` unit bounding cube for `hero_volumetric.wgsl`.
pub const HERO_CUBE_VERTICES: [HeroVertex; 8] = [
    HeroVertex { local_pos: [-1.0, -1.0, -1.0] }, // 0
    HeroVertex { local_pos: [ 1.0, -1.0, -1.0] }, // 1
    HeroVertex { local_pos: [ 1.0,  1.0, -1.0] }, // 2
    HeroVertex { local_pos: [-1.0,  1.0, -1.0] }, // 3
    HeroVertex { local_pos: [-1.0, -1.0,  1.0] }, // 4
    HeroVertex { local_pos: [ 1.0, -1.0,  1.0] }, // 5
    HeroVertex { local_pos: [ 1.0,  1.0,  1.0] }, // 6
    HeroVertex { local_pos: [-1.0,  1.0,  1.0] }, // 7
];

/// Outward-CCW triangle indices for unit cube (`CullMode::Front` renders back faces cleanly).
#[rustfmt::skip]
pub const HERO_CUBE_INDICES: [u16; 36] = [
    // +Z front
    4, 5, 6,  4, 6, 7,
    // -Z back
    1, 0, 3,  1, 3, 2,
    // +X right
    5, 1, 2,  5, 2, 6,
    // -X left
    0, 4, 7,  0, 7, 3,
    // +Y top
    7, 6, 2,  7, 2, 3,
    // -Y bottom
    0, 1, 5,  0, 5, 4,
];

/// Controls output color transfer in `post_bloom_agx.wgsl` (`fs_final_oled_agx`).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum OutputEncodeMode {
    /// Emit linear RGB so hardware `*Srgb` target formats perform exact sRGB encoding.
    HardwareSrgbTarget = 0,
    /// Manually encode sRGB transfer before write (for non-sRGB `*Unorm` targets).
    ManualSrgbUnorm = 1,
    /// Emit direct linear HDR (e.g., `Rgba16Float` / 10-bit+ HDR swapchains).
    LinearHdrTarget = 2,
}

impl OutputEncodeMode {
    #[inline]
    pub fn from_texture_format(format: wgpu::TextureFormat) -> Self {
        match format {
            wgpu::TextureFormat::Rgba8UnormSrgb | wgpu::TextureFormat::Bgra8UnormSrgb => {
                Self::HardwareSrgbTarget
            }
            wgpu::TextureFormat::Rgba16Float => Self::LinearHdrTarget,
            _ => Self::ManualSrgbUnorm,
        }
    }

    #[inline]
    pub fn as_shader_f32(self) -> f32 {
        match self {
            Self::HardwareSrgbTarget => 0.0,
            Self::ManualSrgbUnorm => 1.0,
            Self::LinearHdrTarget => 2.0,
        }
    }
}

/// Per-eye camera matrices relative to head origin `(0, 0, 0)`.
#[derive(Copy, Clone, Debug)]
pub struct EyeView {
    pub view_proj: Mat4,
    pub inv_view_proj: Mat4,
    pub eye_offset: Vec3,
}

impl EyeView {
    pub fn from_view_proj(view_proj: Mat4, eye_offset: Vec3) -> Self {
        Self {
            view_proj,
            inv_view_proj: view_proj.inverse(),
            eye_offset,
        }
    }
}

/// Camera view parameters covering both Desktop Mono and OpenXR Multiview Stereo.
#[derive(Copy, Clone, Debug)]
pub struct RenderCamera {
    pub eyes: [EyeView; 2],
    pub is_stereo: bool,
    pub cam_forward: Vec3,
    pub cam_right: Vec3,
    pub cam_up: Vec3,
    pub fov_y_rad: f32,
    pub aspect: f32,
    pub near_plane: f32,
}

impl RenderCamera {
    /// Constructs a Desktop Mono camera from orientation quaternion, FOV, aspect, and Reversed-Z near plane.
    pub fn mono(orient: Quat, fov_y_rad: f32, aspect: f32, near_plane: f32) -> Self {
        let proj = cosmos_math::reversed_z_infinite_perspective_rh(fov_y_rad, aspect, near_plane);
        let view = cosmos_math::camera_view_matrix(orient);
        let vp = proj * view;
        let eye = EyeView::from_view_proj(vp, Vec3::ZERO);
        Self {
            eyes: [eye, eye],
            is_stereo: false,
            cam_forward: orient * Vec3::NEG_Z,
            cam_right: orient * Vec3::X,
            cam_up: orient * Vec3::Y,
            fov_y_rad,
            aspect,
            near_plane,
        }
    }
}

/// OLED HDR post-processing tuning parameters.
#[derive(Copy, Clone, Debug)]
pub struct PostSettings {
    pub exposure: f32,
    /// Must stay `>= 1.05` so diffuse/void pixels never bloom into grey haze.
    pub bloom_thresh: f32,
    pub bloom_gain: f32,
    pub dispersion: f32,
    pub dither_amp: f32,
    pub vignette_strength: f32,
    pub bloom_soft_knee: f32,
    pub void_floor_eps: f32,
}

impl Default for PostSettings {
    fn default() -> Self {
        Self {
            exposure: 1.0,
            bloom_thresh: 1.06,
            bloom_gain: 0.12,
            dispersion: 0.0035,
            dither_amp: 1.0,
            vignette_strength: 0.0,
            bloom_soft_knee: 0.25,
            void_floor_eps: 2.5e-4,
        }
    }
}

/// Diegetic in-world 3D UI indicators emitted cleanly as luminous OLED sprites (zero 2D HUD).
#[derive(Copy, Clone, Debug, Default)]
pub struct DiegeticUiInput {
    /// Camera-relative wrist center (only drawn when `Some` in VR / hand focus).
    pub wrist_pos: Option<Vec3>,
    pub wrist_orient: Quat,
    /// `[1.0..3.0]` SyncVoice speed bead highlight on Wrist Resonance Ring.
    pub playback_rate: f32,
    /// Active speed ring bead highlight (`0 = ES`, `1 = EN`, `2 = FI`, etc. as subtle language indicator).
    pub locale_idx: u8,
    /// Focus/gaze ray (unit vector) and `[0..1]` motion-stop rest dwell progress.
    pub focus_ray: Vec3,
    pub dwell_progress: f32,
    /// Show subtle focal dwell shimmer ring along `focus_ray` when dwelling cleanly (`> 0.08`).
    pub show_focus_shimmer: bool,
}

/// Assembled GPU frame payload fed into [`crate::CosmosRenderer::render_frame`].
#[derive(Clone, Debug, Default)]
pub struct RenderSceneFrame {
    pub frame: FrameUniforms,
    pub audio: AudioReactiveUniforms,
    pub far: FarShellUniforms,
    pub compute: SimDispatchUniforms,
    /// When `Some(mode)`, CPU initializes `particles` cleanly for that mode and uploads before compute step.
    pub reseed_compute_particles: Option<(u32, u32)>,
    /// Optional compute field grid dispatch `(mode, width, height)` for Turing/Chladni/Aurora ping-pong.
    pub dispatch_field_grid: Option<u32>,
    pub particle_batch: SpriteBatchUniforms,
    pub particle_draw_count: u32,
    pub heroes_surface: Vec<HeroVolumeInstance>,
    pub heroes_volume: Vec<HeroVolumeInstance>,
    pub sprite_batch: SpriteBatchUniforms,
    pub sprites: Vec<SpriteInstance>,
    pub post: PostSettings,
}

