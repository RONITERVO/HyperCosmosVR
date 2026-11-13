// HyperCosmosVR — Shared Documentary Shader Prelude (struct, bindings, packed zone/cue decoders, 3D SDF primitives)
// Concatenated strictly after common_oled.wgsl and before hero_doc_*.wgsl -> hero_volumetric.wgsl.

struct HeroVolumeInstance {
    // xyz = camera-relative world center (render units), w = bounding radius (render units)
    pos_radius: vec4<f32>,
    // unit quaternion (x, y, z, w) rotating local space -> world space
    rot_quat: vec4<f32>,
    // rgb = primary linear HDR base/emission color, a = HDR intensity multiplier
    color_a: vec4<f32>,
    // rgb = secondary atmosphere/aurora/zone-accent color, a = optical density / blend
    color_b: vec4<f32>,
    // x = spin/phase/param0, y = obliquity/gap/param1, z = sub_param/param2, w = dwell/emphasis [0..1]
    phys_a: vec4<f32>,
    // xyz = normalized primary light/Sun direction in world space, w = lod_quality_scale [0.25..1.0]
    phys_b: vec4<f32>,
    // x = hero_kind (0..9), y = variant_id, z = packed zone/feat/scene, w = flags + cue/weight/submode
    kind_tags: vec4<u32>,
};

@group(1) @binding(0) var<storage, read> hero_instances: array<HeroVolumeInstance>;
@group(1) @binding(1) var blue_noise_tex: texture_2d<f32>;
@group(1) @binding(2) var blue_noise_samp: sampler;
@group(1) @binding(3) var field_grid_tex: texture_2d<f32>;
@group(1) @binding(4) var field_grid_samp: sampler;

struct HeroVsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) local_pos: vec3<f32>,
    @location(1) cam_local: vec3<f32>,
    @location(2) @interpolate(flat) inst_idx: u32,
    @location(3) @interpolate(flat) eye_idx: u32,
};

struct SurfaceFsOut {
    @location(0) color: vec4<f32>,
    @builtin(frag_depth) depth: f32,
};

fn quat_rotate(q: vec4<f32>, v: vec3<f32>) -> vec3<f32> {
    let t = 2.0 * cross(q.xyz, v);
    return v + q.w * t + cross(q.xyz, t);
}

fn quat_conj(q: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(-q.xyz, q.w);
}

fn intersect_box(ro: vec3<f32>, rd: vec3<f32>, bmin: vec3<f32>, bmax: vec3<f32>) -> vec2<f32> {
    let inv_rd = vec3<f32>(1.0) / select(rd, vec3<f32>(1e-7), abs(rd) < vec3<f32>(1e-7));
    let t0 = (bmin - ro) * inv_rd;
    let t1 = (bmax - ro) * inv_rd;
    let tmin_v = min(t0, t1);
    let tmax_v = max(t0, t1);
    let t_enter = max(max(tmin_v.x, tmin_v.y), tmin_v.z);
    let t_exit  = min(min(tmax_v.x, tmax_v.y), tmax_v.z);
    return vec2<f32>(t_enter, t_exit);
}

fn intersect_sphere(ro: vec3<f32>, rd: vec3<f32>, radius: f32) -> vec2<f32> {
    let b = dot(ro, rd);
    let c = dot(ro, ro) - radius * radius;
    let disc = b * b - c;
    if (disc < 0.0) {
        return vec2<f32>(1.0, -1.0);
    }
    let s = sqrt(disc);
    return vec2<f32>(-b - s, -b + s);
}

fn sample_blue_noise_jitter(frag_xy: vec2<f32>) -> f32 {
    let uv = fract((frag_xy + vec2<f32>(u_frame.stereo_params.z * 37.0, u_frame.stereo_params.z * 59.0)) / 128.0);
    return textureSampleLevel(blue_noise_tex, blue_noise_samp, uv, 0.0).r;
}

// --- Documentary State Unpacking (Zero Extra Struct Bytes) ---

fn doc_zone(inst: HeroVolumeInstance) -> u32 {
    return inst.kind_tags.z & 0xFFFFu;
}

fn doc_feat(inst: HeroVolumeInstance) -> u32 {
    return (inst.kind_tags.z >> 16u) & 0xFFu;
}

fn doc_scene(inst: HeroVolumeInstance) -> i32 {
    let enc = (inst.kind_tags.z >> 24u) & 0xFFu;
    if (enc == 0u) {
        return -1;
    }
    return i32(enc) - 1;
}

fn doc_cue(inst: HeroVolumeInstance) -> f32 {
    return f32((inst.kind_tags.w >> 8u) & 0xFFu) / 255.0;
}

fn doc_weight(inst: HeroVolumeInstance) -> f32 {
    return f32((inst.kind_tags.w >> 16u) & 0xFFu) / 255.0;
}

fn doc_submode(inst: HeroVolumeInstance) -> u32 {
    return (inst.kind_tags.w >> 24u) & 0xFFu;
}

fn doc_rot_y(p: vec3<f32>, a: f32) -> vec3<f32> {
    let c = cos(a);
    let s = sin(a);
    return vec3<f32>(c * p.x + s * p.z, p.y, -s * p.x + c * p.z);
}

fn doc_rot_x(p: vec3<f32>, a: f32) -> vec3<f32> {
    let c = cos(a);
    let s = sin(a);
    return vec3<f32>(p.x, c * p.y - s * p.z, s * p.y + c * p.z);
}

fn doc_rot_z(p: vec3<f32>, a: f32) -> vec3<f32> {
    let c = cos(a);
    let s = sin(a);
    return vec3<f32>(c * p.x - s * p.y, s * p.x + c * p.y, p.z);
}

// --- Signed Distance Function Library for Sharp Botanical/Architectural/Quantum/Civilization Geometry ---

fn sd_sphere(p: vec3<f32>, r: f32) -> f32 {
    return length(p) - r;
}

fn sd_box(p: vec3<f32>, b: vec3<f32>) -> f32 {
    let q = abs(p) - b;
    return length(max(q, vec3<f32>(0.0))) + min(max(q.x, max(q.y, q.z)), 0.0);
}

fn sd_torus(p: vec3<f32>, r_major: f32, r_minor: f32) -> f32 {
    let q = vec2<f32>(length(p.xz) - r_major, p.y);
    return length(q) - r_minor;
}

fn sd_capsule(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, r: f32) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / max(dot(ba, ba), 1e-6), 0.0, 1.0);
    return length(pa - ba * h) - r;
}

fn sd_cylinder_y(p: vec3<f32>, r: f32, half_h: f32) -> f32 {
    let d = abs(vec2<f32>(length(p.xz), p.y)) - vec2<f32>(r, half_h);
    return min(max(d.x, d.y), 0.0) + length(max(d, vec2<f32>(0.0)));
}

fn sd_octahedron(p_in: vec3<f32>, s: f32) -> f32 {
    let p = abs(p_in);
    return (p.x + p.y + p.z - s) * 0.57735027;
}

fn sd_hex_prism(p_in: vec3<f32>, h_r: f32, h_z: f32) -> f32 {
    let k = vec3<f32>(-0.8660254, 0.5, 0.57735027);
    var p = abs(p_in);
    let refl = 2.0 * min(dot(k.xy, p.xy), 0.0) * k.xy;
    p = vec3<f32>(p.x - refl.x, p.y - refl.y, p.z);
    let d = vec2<f32>(
        length(p.xy - vec2<f32>(clamp(p.x, -k.z * h_r, k.z * h_r), h_r)) * sign(p.y - h_r),
        p.z - h_z
    );
    return min(max(d.x, d.y), 0.0) + length(max(d, vec2<f32>(0.0)));
}

fn sd_cone_up(p: vec3<f32>, sin_cos: vec2<f32>, h: f32) -> f32 {
    let q = h * vec2<f32>(sin_cos.x / max(sin_cos.y, 1e-4), -1.0);
    let w = vec2<f32>(length(p.xz), p.y);
    let a = w - q * clamp(dot(w, q) / max(dot(q, q), 1e-6), 0.0, 1.0);
    let b = w - q * vec2<f32>(clamp(w.x / max(q.x, 1e-4), 0.0, 1.0), 1.0);
    let k = sign(q.y);
    let d = min(dot(a, a), dot(b, b));
    let s = max(k * (w.x * q.y - w.y * q.x), k * (w.y - q.y));
    return sqrt(d) * sign(s);
}

fn doc_line_glow(d: f32, sharpness: f32) -> f32 {
    return exp(-max(d, 0.0) * sharpness);
}

fn doc_shell_glow(d: f32, sharpness: f32) -> f32 {
    return exp(-abs(d) * sharpness);
}
