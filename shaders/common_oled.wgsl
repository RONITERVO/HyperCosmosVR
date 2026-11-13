// HyperCosmosVR — Shared OLED, Scale, Audio-Reactive & Procedural Noise Prelude
// Strict WGSL (Naga / wgpu 30.0.2 DX12 DynamicDxc SM 6.8 + Multiview compatible)
// Prepend this file before compiling any pass shader (`far_shell`, `hero_volumetric`,
// `instanced_sprites`, `compute_universe`, `msdf_text`, `post_bloom_agx`).

const PI: f32 = 3.14159265358979323846;
const TAU: f32 = 6.28318530717958647692;
const INV_PI: f32 = 0.31830988618379067154;

// 16-byte aligned global camera, scale, stereo, and post-processing uniforms (432 bytes).
struct FrameUniforms {
    view_proj: array<mat4x4<f32>, 2>,       // offset   0 (128 B): camera-relative Reversed-Z infinite far projection
    inv_view_proj: array<mat4x4<f32>, 2>,   // offset 128 (128 B): inverse view-projection per eye
    eye_offset: array<vec4<f32>, 2>,        // offset 256 ( 32 B): xyz = eye offset from camera center (0,0,0), w = stereo flag
    cam_forward: vec4<f32>,                 // offset 288 ( 16 B): xyz = forward unit vector, w = vertical FOV (rad)
    cam_right: vec4<f32>,                   // offset 304 ( 16 B): xyz = right unit vector, w = aspect ratio
    cam_up: vec4<f32>,                      // offset 320 ( 16 B): xyz = up unit vector, w = near plane (render units)
    scale_params: vec4<f32>,                // offset 336 ( 16 B): x = u_cam (log10 m), y = du_dt, z = exposure, w = time_sec
    post_params: vec4<f32>,                 // offset 352 ( 16 B): x = bloom_thresh (>=1.05), y = bloom_gain, z = dispersion, w = dither_amp
    viewport_params: vec4<f32>,             // offset 368 ( 16 B): x = width, y = height, z = inv_width, w = inv_height
    stereo_params: vec4<f32>,               // offset 384 ( 16 B): x = eye_count (1 or 2), y = fovea_rad, z = frame_idx_mod, w = active_region_phase
    focus_ray: vec4<f32>,                   // offset 400 ( 16 B): xyz = focus/gaze ray dir, w = rest dwell progress [0..1]
    region_accent: vec4<f32>,               // offset 416 ( 16 B): rgb = linear accent RGB of active zone, w = experience_id (1..25)
};

// 16-byte aligned real-time audio-to-shader reactive uniform bus (64 bytes).
struct AudioReactiveUniforms {
    // x = voice_rms, y = ambient_rms, z = bass_energy, w = mid_energy
    levels_a: vec4<f32>,
    // x = harmonic_centroid [0..1], y = beat_phase [0..1], z = high_energy, w = playback_rate [1..3]
    levels_b: vec4<f32>,
    // x = drone_base_hz, y = synth_filter_hz, z = synth_pulse, w = drone_noise
    synth_a: vec4<f32>,
    // x = active_note_hue [0..1], y = active_timbre_id (0..3), z = music_energy, w = resonance_glow
    synth_b: vec4<f32>,
};

@group(0) @binding(0) var<uniform> u_frame: FrameUniforms;
@group(0) @binding(1) var<uniform> u_audio: AudioReactiveUniforms;

// --- Reversed-Z Infinite Far Plane & Stereo Ray Helpers ---

fn reversed_z_depth(clip_z: f32, clip_w: f32) -> f32 {
    return clip_z / max(clip_w, 1e-7);
}

// Infinite far plane Reversed-Z: near maps to 1.0, far (inf) maps to 0.0.
// Linear distance from camera = near_plane / ndc_z.
fn linearize_reversed_z(ndc_z: f32, near_plane: f32) -> f32 {
    return near_plane / max(ndc_z, 1e-7);
}

// Reconstructs camera-relative world ray direction from NDC xy in [-1, 1] for eye_idx (0 or 1).
// Samples near (z=1.0) and mid (z=0.5) planes to stay well clear of infinite far singularity (z=0).
fn ndc_to_world_ray(ndc_xy: vec2<f32>, eye_idx: u32) -> vec3<f32> {
    let inv_vp = u_frame.inv_view_proj[eye_idx];
    let p_near_h = inv_vp * vec4<f32>(ndc_xy, 1.0, 1.0);
    let p_mid_h  = inv_vp * vec4<f32>(ndc_xy, 0.5, 1.0);
    let p_near = p_near_h.xyz / p_near_h.w;
    let p_mid  = p_mid_h.xyz / p_mid_h.w;
    return normalize(p_mid - p_near);
}

fn world_pos_to_clip(pos_cam_rel: vec3<f32>, eye_idx: u32) -> vec4<f32> {
    return u_frame.view_proj[eye_idx] * vec4<f32>(pos_cam_rel, 1.0);
}

// Foveated step multiplier: 1.0 inside foveal overlap cone, down to 0.42 in lens periphery.
fn fovea_step_factor(ray_dir: vec3<f32>) -> f32 {
    let cos_ang = clamp(dot(normalize(ray_dir), normalize(u_frame.cam_forward.xyz)), -1.0, 1.0);
    let ang = acos(cos_ang);
    let fovea = max(u_frame.stereo_params.y, 0.25);
    return mix(1.0, 0.42, smoothstep(fovea, fovea * 2.1, ang));
}

// --- Hashes & Procedural 3D Noise / Curl / FBM ---

fn hash13(p3_in: vec3<f32>) -> f32 {
    var p3 = fract(p3_in * 0.1031);
    p3 += vec3<f32>(dot(p3, p3.zyx + vec3<f32>(31.32)));
    return fract((p3.x + p3.y) * p3.z);
}

fn hash33(p3_in: vec3<f32>) -> vec3<f32> {
    var p3 = fract(p3_in * vec3<f32>(0.1031, 0.1030, 0.0973));
    p3 += vec3<f32>(dot(p3, p3.yxz + vec3<f32>(33.33)));
    return fract((p3.xxy + p3.yxx) * p3.zyx);
}

// 3D Simplex Noise (Ashima / Gustavson) — smooth, isotropic, zero lattice axis artifacts.
fn mod289_v3(x: vec3<f32>) -> vec3<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn mod289_v4(x: vec4<f32>) -> vec4<f32> {
    return x - floor(x * (1.0 / 289.0)) * 289.0;
}

fn permute4(x: vec4<f32>) -> vec4<f32> {
    return mod289_v4(((x * 34.0) + vec4<f32>(10.0)) * x);
}

fn taylor_inv_sqrt4(r: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(1.79284291400159) - 0.85373472095314 * r;
}

fn snoise3(v: vec3<f32>) -> f32 {
    let C = vec2<f32>(1.0 / 6.0, 1.0 / 3.0);
    let D = vec4<f32>(0.0, 0.5, 1.0, 2.0);

    var i = floor(v + vec3<f32>(dot(v, C.yyy)));
    let x0 = v - i + vec3<f32>(dot(i, C.xxx));

    let g = step(x0.yzx, x0.xyz);
    let l = vec3<f32>(1.0) - g;
    let i1 = min(g.xyz, l.zxy);
    let i2 = max(g.xyz, l.zxy);

    let x1 = x0 - i1 + C.xxx;
    let x2 = x0 - i2 + C.yyy;
    let x3 = x0 - D.yyy;

    i = mod289_v3(i);
    let p = permute4(
        permute4(
            permute4(vec4<f32>(i.z) + vec4<f32>(0.0, i1.z, i2.z, 1.0))
            + vec4<f32>(i.y) + vec4<f32>(0.0, i1.y, i2.y, 1.0)
        )
        + vec4<f32>(i.x) + vec4<f32>(0.0, i1.x, i2.x, 1.0)
    );

    let n_ = 0.142857142857;
    let ns = n_ * D.wyz - D.xzx;

    let j = p - 49.0 * floor(p * ns.z * ns.z);
    let x_ = floor(j * ns.z);
    let y_ = floor(j - 7.0 * x_);

    let x = x_ * ns.x + ns.yyyy;
    let y = y_ * ns.x + ns.yyyy;
    let h = vec4<f32>(1.0) - abs(x) - abs(y);

    let b0 = vec4<f32>(x.xy, y.xy);
    let b1 = vec4<f32>(x.zw, y.zw);

    let s0 = floor(b0) * 2.0 + vec4<f32>(1.0);
    let s1 = floor(b1) * 2.0 + vec4<f32>(1.0);
    let sh = -step(h, vec4<f32>(0.0));

    let a0 = b0.xzyw + s0.xzyw * sh.xxyy;
    let a1 = b1.xzyw + s1.xzyw * sh.zzww;

    var p0 = vec3<f32>(a0.xy, h.x);
    var p1 = vec3<f32>(a0.zw, h.y);
    var p2 = vec3<f32>(a1.xy, h.z);
    var p3 = vec3<f32>(a1.zw, h.w);

    let norm = taylor_inv_sqrt4(vec4<f32>(dot(p0, p0), dot(p1, p1), dot(p2, p2), dot(p3, p3)));
    p0 *= norm.x;
    p1 *= norm.y;
    p2 *= norm.z;
    p3 *= norm.w;

    var m = max(vec4<f32>(0.5) - vec4<f32>(dot(x0, x0), dot(x1, x1), dot(x2, x2), dot(x3, x3)), vec4<f32>(0.0));
    m = m * m;
    return 105.0 * dot(m * m, vec4<f32>(dot(p0, x0), dot(p1, x1), dot(p2, x2), dot(p3, x3)));
}

fn fbm3(p_in: vec3<f32>, octaves: i32) -> f32 {
    var p = p_in;
    var sum: f32 = 0.0;
    var amp: f32 = 0.5;
    for (var i: i32 = 0; i < 6; i = i + 1) {
        if (i >= octaves) {
            break;
        }
        sum += amp * snoise3(p);
        p = p * 2.03 + vec3<f32>(13.7, -9.2, 5.4);
        amp *= 0.5;
    }
    return sum;
}

// Divergence-free 3D curl noise for auroral curtains, plasma filaments, and cosmic streamlines.
fn curl_noise3(p: vec3<f32>) -> vec3<f32> {
    let e: f32 = 0.015;
    let dx = vec3<f32>(e, 0.0, 0.0);
    let dy = vec3<f32>(0.0, e, 0.0);
    let dz = vec3<f32>(0.0, 0.0, e);

    let ox = vec3<f32>(31.4, 17.2, 89.7);
    let oy = vec3<f32>(-47.1, 63.8, 11.3);
    let oz = vec3<f32>(12.9, -28.4, 54.6);

    let p_y1 = vec3<f32>(snoise3(p + dy + ox), snoise3(p + dy + oy), snoise3(p + dy + oz));
    let p_y0 = vec3<f32>(snoise3(p - dy + ox), snoise3(p - dy + oy), snoise3(p - dy + oz));
    let p_z1 = vec3<f32>(snoise3(p + dz + ox), snoise3(p + dz + oy), snoise3(p + dz + oz));
    let p_z0 = vec3<f32>(snoise3(p - dz + ox), snoise3(p - dz + oy), snoise3(p - dz + oz));
    let p_x1 = vec3<f32>(snoise3(p + dx + ox), snoise3(p + dx + oy), snoise3(p + dx + oz));
    let p_x0 = vec3<f32>(snoise3(p - dx + ox), snoise3(p - dx + oy), snoise3(p - dx + oz));

    let inv2e = 0.5 / e;
    let cx = (p_y1.z - p_y0.z - p_z1.y + p_z0.y) * inv2e;
    let cy = (p_z1.x - p_z0.x - p_x1.z + p_x0.z) * inv2e;
    let cz = (p_x1.y - p_x0.y - p_y1.x + p_y0.x) * inv2e;
    return vec3<f32>(cx, cy, cz);
}

// --- Color Physics & OLED Radiance Helpers ---

// Physically grounded Planckian blackbody color (Kelvin in [1000..40000]) in linear sRGB.
fn blackbody_rgb(kelvin_in: f32) -> vec3<f32> {
    let t = clamp(kelvin_in, 1000.0, 40000.0) * 0.01;
    var r: f32 = 1.0;
    var g: f32 = 1.0;
    var b: f32 = 1.0;

    if (t <= 66.0) {
        r = 1.0;
        g = clamp((99.4708025861 * log(max(t, 1.0)) - 161.1195681661) / 255.0, 0.0, 1.0);
        if (t <= 19.0) {
            b = 0.0;
        } else {
            b = clamp((138.5177312231 * log(max(t - 10.0, 1.0)) - 305.0447927307) / 255.0, 0.0, 1.0);
        }
    } else {
        r = clamp((329.698727446 * pow(t - 60.0, -0.1332047592)) / 255.0, 0.0, 1.0);
        g = clamp((288.1221695283 * pow(t - 60.0, -0.0755148492)) / 255.0, 0.0, 1.0);
        b = 1.0;
    }

    // Approximate sRGB -> linear conversion (pow 2.2) so HDR bloom & tonemap stay physically linear.
    return pow(vec3<f32>(r, g, b), vec3<f32>(2.2));
}

// HSL (h in [0..1], s in [0..1], l in [0..1]) -> linear sRGB (matching App 01 Colors).
fn hsl_to_rgb(h_in: f32, s_in: f32, l_in: f32) -> vec3<f32> {
    let h = fract(h_in);
    let s = clamp(s_in, 0.0, 1.0);
    let l = clamp(l_in, 0.0, 1.0);
    let c = (1.0 - abs(2.0 * l - 1.0)) * s;
    let h6 = h * 6.0;
    let x = c * (1.0 - abs((h6 % 2.0) - 1.0));
    var rgb_p = vec3<f32>(0.0);
    if (h6 < 1.0) {
        rgb_p = vec3<f32>(c, x, 0.0);
    } else if (h6 < 2.0) {
        rgb_p = vec3<f32>(x, c, 0.0);
    } else if (h6 < 3.0) {
        rgb_p = vec3<f32>(0.0, c, x);
    } else if (h6 < 4.0) {
        rgb_p = vec3<f32>(0.0, x, c);
    } else if (h6 < 5.0) {
        rgb_p = vec3<f32>(x, 0.0, c);
    } else {
        rgb_p = vec3<f32>(c, 0.0, x);
    }
    let m = l - 0.5 * c;
    let srgb = clamp(rgb_p + vec3<f32>(m), vec3<f32>(0.0), vec3<f32>(1.0));
    return pow(srgb, vec3<f32>(2.2));
}

// Thin-film spectral iridescence (cosine interference palette for quantum fields, bubbles, & prism rims).
fn iridescence_spectral(cos_theta: f32, film_phase: f32, audio_shift: f32) -> vec3<f32> {
    let angle_term = 1.0 - clamp(abs(cos_theta), 0.0, 1.0);
    let phase = (film_phase + angle_term * 2.4 + audio_shift) * TAU;
    let a = vec3<f32>(0.50, 0.50, 0.50);
    let b = vec3<f32>(0.50, 0.50, 0.50);
    let c = vec3<f32>(1.00, 1.00, 1.00);
    let d = vec3<f32>(0.00, 0.33, 0.67);
    let col = a + b * cos(TAU * (c * (phase * INV_PI * 0.5) + d));
    return pow(clamp(col, vec3<f32>(0.0), vec3<f32>(1.0)), vec3<f32>(2.2));
}

fn luminance_rec709(c: vec3<f32>) -> f32 {
    return dot(c, vec3<f32>(0.2126, 0.7152, 0.0722));
}

// Enforces true #000000 OLED void floor by clamping sub-epsilon residuals cleanly to zero.
fn oled_void_clamp(c: vec3<f32>) -> vec3<f32> {
    let lum = luminance_rec709(c);
    return select(vec3<f32>(0.0), max(c, vec3<f32>(0.0)), lum > 1e-5);
}

