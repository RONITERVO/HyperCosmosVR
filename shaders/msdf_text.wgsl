// ============================================================================
// msdf_text.wgsl -- Google-Earth-Style 3D World-Space MSDF Typography & Tethers
// Requires prelude: common_oled.wgsl (@group(0) @binding(0..1))
// Targets: wgpu 30.0.2 (Naga) -> DX12 HLSL SM 6.5+ (Desktop Mono + OpenXR Multiview)
// Pipeline: Rgba16Float HDR linear scene target, Reversed-Z depth test (GreaterEqual),
//           depth_write_enabled = false, BlendState::ALPHA_BLENDING
// ============================================================================

struct TextPassUniforms {
    // x = global_alpha [0..1]
    // y = edge_softness_scale (1.0 = sharp analytic AA)
    // z = nib_glow_gain (HDR bloom boost at live-ink cursor, e.g. 1.35)
    // w = depth_bias_scale (camera-ward pull scale on pos_center.w to stop z-fighting)
    params_a: vec4<f32>,
    // rgb = linear HDR live-ink nib accent (falls back to region_accent when 0)
    // w   = halo_scale (1.0 = default obsidian contrast outline)
    nib_color: vec4<f32>,
};

// 112 bytes (7 x vec4<f32>) per world-space glyph quad
struct GlyphInstance {
    // xyz = camera-relative world center (meters), w = anti-z-fighting pull toward camera (meters)
    pos_center: vec4<f32>,
    // xyz = right half-extent in world meters, w = atlas msdf pixel range (e.g. 6.0 or 8.0)
    right_vec: vec4<f32>,
    // xyz = up half-extent in world meters, w = halo thickness in normalized SDF units (e.g. 0.22)
    up_vec: vec4<f32>,
    // x = u_left, y = v_top, z = u_right, w = v_bottom (GPU texture convention: v=0 top, v=1 bottom)
    uv_rect: vec4<f32>,
    // rgb = linear HDR glyph fill color, w = opacity [0..1]
    color_hdr: vec4<f32>,
    // rgb = linear HDR halo color (typically obsidian 0.002, 0.003, 0.006), w = halo opacity [0..1]
    halo_color: vec4<f32>,
    // x = glyph_t0 [0..1], y = glyph_t1 [0..1], z = reveal_cursor (>=1.5 static, <-0.5 hidden, 0..1.2 live ink),
    // w = mode (0=spatial label, 1=live-ink phrase, 2=bilingual ghost track, 3=remembered/visited, 4=focused)
    ink_params: vec4<f32>,
};

// 48 bytes (3 x vec4<f32>) per 3D glowing leader filament connecting label <-> world anchor
struct TetherInstance {
    // xyz = start position in camera-relative world meters, w = half_width in meters
    p0_width: vec4<f32>,
    // xyz = end position in camera-relative world meters, w = opacity [0..1]
    p1_alpha: vec4<f32>,
    // rgb = linear HDR filament color, w = end_taper [0..1] (width multiplier at p1)
    color_hdr: vec4<f32>,
};

@group(1) @binding(0) var<uniform> u_text_pass: TextPassUniforms;
@group(1) @binding(1) var<storage, read> glyphs: array<GlyphInstance>;
@group(1) @binding(2) var<storage, read> tethers: array<TetherInstance>;
@group(1) @binding(3) var msdf_atlas: texture_2d<f32>;
@group(1) @binding(4) var msdf_samp: sampler;

var<private> QUAD_CORNERS: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(-1.0, -1.0),
    vec2<f32>( 1.0, -1.0),
    vec2<f32>( 1.0,  1.0),
    vec2<f32>(-1.0, -1.0),
    vec2<f32>( 1.0,  1.0),
    vec2<f32>(-1.0,  1.0),
);

struct GlyphVSOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) local_uv: vec2<f32>,
    @location(2) color_hdr: vec4<f32>,
    @location(3) halo_color: vec4<f32>,
    @location(4) ink_params: vec4<f32>,
    @location(5) sdf_metrics: vec2<f32>, // x = px_range, y = halo_sdf
};

struct TetherVSOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) seg_uv: vec2<f32>, // x = along [0..1], y = across [-1..1]
    @location(1) color_hdr: vec4<f32>,
};

fn median3(a: f32, b: f32, c: f32) -> f32 {
    return max(min(a, b), min(max(a, b), c));
}

fn eval_glyph_vertex(vertex_idx: u32, inst_idx: u32, eye_idx: u32) -> GlyphVSOut {
    var out: GlyphVSOut;
    let g = glyphs[inst_idx];
    let corner = QUAD_CORNERS[vertex_idx];

    var pos_w = g.pos_center.xyz + g.right_vec.xyz * corner.x + g.up_vec.xyz * corner.y;

    // Gentle view-ray bias prevents coplanar z-fighting against planets/panels without breaking stereo
    let eye_pos = u_frame.eye_offset[eye_idx].xyz;
    let to_eye = eye_pos - g.pos_center.xyz;
    let dist = max(length(to_eye), 1e-4);
    let max_pull = min(g.pos_center.w * u_text_pass.params_a.w, dist * 0.15);
    pos_w += (to_eye / dist) * max(max_pull, 0.0);

    out.clip_pos = world_pos_to_clip(pos_w, eye_idx);

    let local_x = corner.x * 0.5 + 0.5;
    // corner.y = +1 is top of quad -> maps to uv_rect.y (v_top); corner.y = -1 -> uv_rect.w (v_bottom)
    let top_down_t = 0.5 - corner.y * 0.5;
    out.uv = vec2<f32>(
        mix(g.uv_rect.x, g.uv_rect.z, local_x),
        mix(g.uv_rect.y, g.uv_rect.w, top_down_t)
    );
    out.local_uv = vec2<f32>(local_x, corner.y * 0.5 + 0.5);
    out.color_hdr = g.color_hdr;
    out.halo_color = g.halo_color;
    out.ink_params = g.ink_params;
    out.sdf_metrics = vec2<f32>(g.right_vec.w, g.up_vec.w);
    return out;
}

@vertex
fn vs_glyph_main(
    @builtin(vertex_index) vertex_idx: u32,
    @builtin(instance_index) inst_idx: u32,
) -> GlyphVSOut {
    return eval_glyph_vertex(vertex_idx, inst_idx, 0u);
}

@vertex
fn vs_glyph_stereo(
    @builtin(vertex_index) vertex_idx: u32,
    @builtin(instance_index) inst_idx: u32,
    @builtin(view_index) view_idx: u32,
) -> GlyphVSOut {
    return eval_glyph_vertex(vertex_idx, inst_idx, view_idx);
}

@fragment
fn fs_glyph_main(in: GlyphVSOut) -> @location(0) vec4<f32> {
    if in.ink_params.z < -0.5 || u_text_pass.params_a.x <= 1e-4 {
        discard;
    }

    let msdf_sample = textureSample(msdf_atlas, msdf_samp, in.uv);
    let sd = median3(msdf_sample.r, msdf_sample.g, msdf_sample.b);

    let tex_dims = max(vec2<f32>(textureDimensions(msdf_atlas, 0)), vec2<f32>(1.0));
    let unit_range = vec2<f32>(max(in.sdf_metrics.x, 1.0)) / tex_dims;
    let uv_deriv = max(fwidth(in.uv), vec2<f32>(1e-6));
    let screen_tex_size = vec2<f32>(1.0) / uv_deriv;
    let screen_px_range = clamp(0.5 * dot(unit_range, screen_tex_size), 1.0, 32.0);

    let softness = max(u_text_pass.params_a.y, 0.25);
    let sd_px = (sd - 0.5) * screen_px_range / softness;

    let fill_cov = smoothstep(-0.55, 0.55, sd_px);
    let halo_scale = max(u_text_pass.nib_color.w, 0.0);
    // Keep halo strictly within the MSDF interior distance range so outer quad padding stays 0.0 alpha
    let halo_edge = clamp(0.14 * screen_px_range / softness, 0.18, 0.55);
    let max_safe_halo_px = max(0.32 * screen_px_range / softness - halo_edge, 0.0);
    let halo_px = min(
        clamp(in.sdf_metrics.y * screen_px_range * halo_scale, 0.0, 3.2),
        max_safe_halo_px
    );
    let stroke_band = smoothstep(0.12, 0.26, sd);
    var halo_cov = 0.0;
    if in.sdf_metrics.y > 0.001 && halo_scale > 0.001 && max_safe_halo_px > 0.01 {
        halo_cov = smoothstep(-halo_px - halo_edge, -halo_px + halo_edge, sd_px) * stroke_band * in.halo_color.a;
    }

    // Evaluate live-ink mode and SyncVoice progression
    let mode = u32(round(clamp(in.ink_params.w, 0.0, 15.0)));
    var ink_alpha = in.color_hdr.a * u_text_pass.params_a.x;
    var fill_rgb = in.color_hdr.rgb;

    var nib_accent = u_text_pass.nib_color.rgb;
    if dot(nib_accent, nib_accent) <= 1e-5 {
        nib_accent = max(u_frame.region_accent.rgb, vec3<f32>(0.45, 0.85, 1.15));
    }

    let voice_rms = u_audio.levels_a.x;
    let is_sweep_active = in.ink_params.z < 1.5;

    if mode == 1u && is_sweep_active {
        let local_t = mix(in.ink_params.x, in.ink_params.y, in.local_uv.x);
        let pen_dist = in.ink_params.z - local_t;
        let reveal = smoothstep(-0.025, 0.015, pen_dist);
        let nib_wave = exp(-pow((pen_dist - 0.004) / 0.028, 2.0));
        let nib_boost = u_text_pass.params_a.z * (1.0 + 0.55 * voice_rms);
        ink_alpha *= mix(0.22, 1.0, reveal);
        fill_rgb = mix(fill_rgb, nib_accent * nib_boost, clamp(nib_wave * 0.85, 0.0, 1.0));
    } else if mode == 2u {
        // Spanish original track / dual-ribbon ghost ink
        if is_sweep_active {
            let local_t = mix(in.ink_params.x, in.ink_params.y, in.local_uv.x);
            let pen_dist = in.ink_params.z - local_t;
            let reveal = smoothstep(-0.03, 0.02, pen_dist);
            let nib_wave = exp(-pow((pen_dist - 0.004) / 0.035, 2.0));
            ink_alpha *= mix(0.28, 0.85, reveal);
            fill_rgb = mix(fill_rgb, nib_accent * 1.15, nib_wave * 0.55);
        } else {
            ink_alpha *= 0.72 + 0.12 * voice_rms;
        }
    } else if mode == 3u {
        // Remembered / skip-memory explored item: calm low-glare slate ink
        ink_alpha *= 0.40;
        fill_rgb *= 0.82;
    } else if mode == 4u {
        // Focused / reticle-dwell highlighted anchor
        let dwell = clamp(u_frame.focus_ray.w, 0.0, 1.0);
        let pulse = 1.0 + 0.18 * dwell + 0.08 * sin(u_frame.scale_params.w * 4.5);
        fill_rgb = mix(fill_rgb, max(fill_rgb, nib_accent), 0.25 * dwell) * pulse;
    }

    let eff_fill = fill_cov * ink_alpha;
    let eff_halo = halo_cov * ink_alpha;

    let out_alpha = eff_fill + eff_halo * (1.0 - eff_fill);
    if out_alpha <= 0.002 {
        discard;
    }

    let premul_rgb = fill_rgb * eff_fill + in.halo_color.rgb * eff_halo * (1.0 - eff_fill);
    let out_rgb = oled_void_clamp(premul_rgb / max(out_alpha, 1e-5));
    return vec4<f32>(out_rgb, clamp(out_alpha, 0.0, 1.0));
}

fn eval_tether_vertex(vertex_idx: u32, inst_idx: u32, eye_idx: u32) -> TetherVSOut {
    var out: TetherVSOut;
    let t = tethers[inst_idx];
    let corner = QUAD_CORNERS[vertex_idx]; // x along [-1, 1], y across [-1, 1]

    let along = corner.x * 0.5 + 0.5;
    let center = mix(t.p0_width.xyz, t.p1_alpha.xyz, along);
    let seg = t.p1_alpha.xyz - t.p0_width.xyz;
    let seg_len = max(length(seg), 1e-5);
    let seg_dir = seg / seg_len;

    let eye_pos = u_frame.eye_offset[eye_idx].xyz;
    let view_vec = eye_pos - center;
    let dist = max(length(view_vec), 1e-4);
    let view_dir = view_vec / dist;

    var side = cross(view_dir, seg_dir);
    if dot(side, side) < 1e-6 {
        side = u_frame.cam_right.xyz;
    }
    side = normalize(side);

    // Minimum angular width so filaments stay stable without subpixel VR aliasing
    let min_half_w = dist * u_frame.viewport_params.w * 1.15;
    let base_w = max(t.p0_width.w, min_half_w);
    let taper = mix(1.0, max(t.color_hdr.w, 0.15), along);
    let half_w = base_w * taper;

    let pos_w = center + side * (corner.y * half_w);
    out.clip_pos = world_pos_to_clip(pos_w, eye_idx);
    out.seg_uv = vec2<f32>(along, corner.y);
    out.color_hdr = vec4<f32>(t.color_hdr.rgb, t.p1_alpha.w);
    return out;
}

@vertex
fn vs_tether_main(
    @builtin(vertex_index) vertex_idx: u32,
    @builtin(instance_index) inst_idx: u32,
) -> TetherVSOut {
    return eval_tether_vertex(vertex_idx, inst_idx, 0u);
}

@vertex
fn vs_tether_stereo(
    @builtin(vertex_index) vertex_idx: u32,
    @builtin(instance_index) inst_idx: u32,
    @builtin(view_index) view_idx: u32,
) -> TetherVSOut {
    return eval_tether_vertex(vertex_idx, inst_idx, view_idx);
}

@fragment
fn fs_tether_main(in: TetherVSOut) -> @location(0) vec4<f32> {
    let across = abs(in.seg_uv.y);
    let core = exp(-across * across * 5.2);
    let end_fade = smoothstep(0.0, 0.08, in.seg_uv.x) * smoothstep(1.0, 0.90, in.seg_uv.x);

    // Subtle energy packet flowing from label toward anchor synchronized with voice RMS
    let travel = fract(in.seg_uv.x * 2.5 - u_frame.scale_params.w * 0.85);
    let pulse = 1.0 + (0.25 + 0.55 * u_audio.levels_a.x) * exp(-pow((travel - 0.5) / 0.14, 2.0));

    let alpha = core * end_fade * in.color_hdr.a * u_text_pass.params_a.x;
    if alpha <= 0.002 {
        discard;
    }

    let rgb = oled_void_clamp(in.color_hdr.rgb * pulse);
    return vec4<f32>(rgb, clamp(alpha, 0.0, 1.0));
}

