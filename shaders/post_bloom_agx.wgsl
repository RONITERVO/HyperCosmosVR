// ============================================================================
// post_bloom_agx.wgsl -- Tight-Threshold Dual-Kawase Bloom + Chromatic
// Dispersion + AgX OLED Filmic Tonemapping + TPDF Blue-Noise Dither
// Requires prelude: common_oled.wgsl (@group(0) @binding(0..1))
// Targets: wgpu 30.0.2 (Naga) -> DX12 HLSL SM 6.5+ (Desktop Mono + OpenXR Stereo)
// ============================================================================

struct PostPassUniforms {
    // x = inv_src_width, y = inv_src_height, z = kawase_offset_scale, w = target_layer_f32 (0 or 1 for mono mip views)
    texel_scale: vec4<f32>,
    // x = output_encode_mode (0=linear for sRGB target, 1=manual sRGB encode for UNORM target, 2=linear HDR 10/16-bit)
    // y = subtle_vignette_strength (0.0 for pure VR OLED default)
    // z = bloom_soft_knee (default 0.25)
    // w = void_floor_eps (default 1e-5 -> literal #000000 below this radiance)
    grade_params: vec4<f32>,
};

@group(1) @binding(0) var<uniform> u_post: PostPassUniforms;
@group(1) @binding(1) var hdr_scene_tex: texture_2d_array<f32>;
@group(1) @binding(2) var bloom_tex: texture_2d_array<f32>;
@group(1) @binding(3) var blue_noise_tex: texture_2d<f32>;
@group(1) @binding(4) var linear_samp: sampler;
@group(1) @binding(5) var point_samp: sampler;

struct PostVSOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) @interpolate(flat) layer_idx: u32,
};

fn eval_post_fullscreen(vertex_idx: u32, layer_idx: u32) -> PostVSOut {
    var out: PostVSOut;
    let x = f32(i32(vertex_idx & 1u) * 4 - 1);
    let y = f32(i32(vertex_idx & 2u) * 2 - 1);
    out.clip_pos = vec4<f32>(x, y, 0.5, 1.0);
    // NDC (+1 up, -1 down) -> GPU UV (0 top, 1 bottom)
    out.uv = vec2<f32>(x * 0.5 + 0.5, 0.5 - y * 0.5);
    out.layer_idx = layer_idx;
    return out;
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_idx: u32) -> PostVSOut {
    let layer_idx = u32(round(max(u_post.texel_scale.w, 0.0)));
    return eval_post_fullscreen(vertex_idx, layer_idx);
}

@vertex
fn vs_stereo(
    @builtin(vertex_index) vertex_idx: u32,
    @builtin(view_index) view_idx: u32,
) -> PostVSOut {
    return eval_post_fullscreen(vertex_idx, view_idx);
}

fn soft_threshold_bloom(rgb: vec3<f32>, thresh: f32, knee: f32) -> vec3<f32> {
    let br = max(rgb.r, max(rgb.g, rgb.b));
    if br <= 1e-6 {
        return vec3<f32>(0.0);
    }
    let k = max(knee, 0.05);
    let rq = clamp(br - thresh + k, 0.0, 2.0 * k);
    let soft = (rq * rq) / (4.0 * k + 1e-5);
    let weight = max(soft, br - thresh) / br;
    return max(rgb * max(weight, 0.0), vec3<f32>(0.0));
}

// Karis firefly suppressor prevents shimmering single-pixel star spikes in VR HMDs
fn karis_weight(c: vec3<f32>) -> f32 {
    return 1.0 / (1.0 + luminance_rec709(max(c, vec3<f32>(0.0))));
}

@fragment
fn fs_bloom_prefilter(in: PostVSOut) -> @location(0) vec4<f32> {
    let d = u_post.texel_scale.xy * 0.5;
    let layer = i32(in.layer_idx);
    let c0 = textureSampleLevel(hdr_scene_tex, linear_samp, in.uv + vec2<f32>(-d.x, -d.y), layer, 0.0).rgb;
    let c1 = textureSampleLevel(hdr_scene_tex, linear_samp, in.uv + vec2<f32>( d.x, -d.y), layer, 0.0).rgb;
    let c2 = textureSampleLevel(hdr_scene_tex, linear_samp, in.uv + vec2<f32>(-d.x,  d.y), layer, 0.0).rgb;
    let c3 = textureSampleLevel(hdr_scene_tex, linear_samp, in.uv + vec2<f32>( d.x,  d.y), layer, 0.0).rgb;

    let thresh = max(u_frame.post_params.x, 1.05);
    let knee = max(u_post.grade_params.z, 0.20);

    let b0 = soft_threshold_bloom(c0, thresh, knee);
    let b1 = soft_threshold_bloom(c1, thresh, knee);
    let b2 = soft_threshold_bloom(c2, thresh, knee);
    let b3 = soft_threshold_bloom(c3, thresh, knee);

    let w0 = karis_weight(b0);
    let w1 = karis_weight(b1);
    let w2 = karis_weight(b2);
    let w3 = karis_weight(b3);
    let w_sum = max(w0 + w1 + w2 + w3, 1e-5);

    let out_rgb = (b0 * w0 + b1 * w1 + b2 * w2 + b3 * w3) / w_sum;
    return vec4<f32>(oled_void_clamp(out_rgb), 1.0);
}

@fragment
fn fs_kawase_down(in: PostVSOut) -> @location(0) vec4<f32> {
    let offset = u_post.texel_scale.xy * max(u_post.texel_scale.z, 0.5);
    let layer = i32(in.layer_idx);

    let center = textureSampleLevel(bloom_tex, linear_samp, in.uv, layer, 0.0).rgb * 4.0;
    let c0 = textureSampleLevel(bloom_tex, linear_samp, in.uv + vec2<f32>(-offset.x, -offset.y), layer, 0.0).rgb;
    let c1 = textureSampleLevel(bloom_tex, linear_samp, in.uv + vec2<f32>( offset.x, -offset.y), layer, 0.0).rgb;
    let c2 = textureSampleLevel(bloom_tex, linear_samp, in.uv + vec2<f32>(-offset.x,  offset.y), layer, 0.0).rgb;
    let c3 = textureSampleLevel(bloom_tex, linear_samp, in.uv + vec2<f32>( offset.x,  offset.y), layer, 0.0).rgb;

    let sum_rgb = (center + c0 + c1 + c2 + c3) * (1.0 / 8.0);
    return vec4<f32>(oled_void_clamp(sum_rgb), 1.0);
}

@fragment
fn fs_kawase_up(in: PostVSOut) -> @location(0) vec4<f32> {
    let hp = u_post.texel_scale.xy * max(u_post.texel_scale.z, 0.5);
    let layer = i32(in.layer_idx);

    var acc = vec3<f32>(0.0);
    acc += textureSampleLevel(bloom_tex, linear_samp, in.uv + vec2<f32>(-hp.x * 2.0, 0.0), layer, 0.0).rgb;
    acc += textureSampleLevel(bloom_tex, linear_samp, in.uv + vec2<f32>( hp.x * 2.0, 0.0), layer, 0.0).rgb;
    acc += textureSampleLevel(bloom_tex, linear_samp, in.uv + vec2<f32>(0.0, -hp.y * 2.0), layer, 0.0).rgb;
    acc += textureSampleLevel(bloom_tex, linear_samp, in.uv + vec2<f32>(0.0,  hp.y * 2.0), layer, 0.0).rgb;
    acc += textureSampleLevel(bloom_tex, linear_samp, in.uv + vec2<f32>(-hp.x, -hp.y), layer, 0.0).rgb * 2.0;
    acc += textureSampleLevel(bloom_tex, linear_samp, in.uv + vec2<f32>( hp.x, -hp.y), layer, 0.0).rgb * 2.0;
    acc += textureSampleLevel(bloom_tex, linear_samp, in.uv + vec2<f32>(-hp.x,  hp.y), layer, 0.0).rgb * 2.0;
    acc += textureSampleLevel(bloom_tex, linear_samp, in.uv + vec2<f32>( hp.x,  hp.y), layer, 0.0).rgb * 2.0;

    return vec4<f32>(oled_void_clamp(acc * (1.0 / 12.0)), 1.0);
}

// Troy Sobotka AgX Rec.709-in / Rec.709-out approximation tailored for OLED pure-black preservation
fn agx_contrast_poly(x: vec3<f32>) -> vec3<f32> {
    let x2 = x * x;
    let x4 = x2 * x2;
    return 15.5 * x4 * x2
        - 40.14 * x4 * x
        + 31.96 * x4
        - 6.868 * x2 * x
        + 0.4298 * x2
        + 0.1191 * x
        - 0.00232;
}

fn agx_tonemap_oled(hdr_rgb: vec3<f32>) -> vec3<f32> {
    let max_ch = max(hdr_rgb.r, max(hdr_rgb.g, hdr_rgb.b));
    if max_ch <= 0.0 {
        return vec3<f32>(0.0);
    }

    // WGSL mat3x3 columns
    let agx_inset = mat3x3<f32>(
        vec3<f32>(0.842479062253094,  0.0423282422610123, 0.0423756549057051),
        vec3<f32>(0.0784335999999992, 0.878468636469772,  0.0784336),
        vec3<f32>(0.0792237451477643, 0.0791661274605434, 0.879142973793104)
    );
    let agx_outset = mat3x3<f32>(
        vec3<f32>( 1.19687900512017,  -0.0528968517574562, -0.0529716355144438),
        vec3<f32>(-0.0980208811401368, 1.15190312990417,   -0.0980434501171241),
        vec3<f32>(-0.0990297440797205, -0.0989611768448433, 1.15107367264116)
    );

    let min_ev = -12.473931188;
    let max_ev = 4.026068812;

    var val = max(agx_inset * hdr_rgb, vec3<f32>(1e-10));
    val = clamp((log2(val) - vec3<f32>(min_ev)) / (max_ev - min_ev), vec3<f32>(0.0), vec3<f32>(1.0));
    val = clamp(agx_contrast_poly(val), vec3<f32>(0.0), vec3<f32>(1.0));
    val = max(agx_outset * val, vec3<f32>(0.0));

    // Smoothly blend ultra-dark linear nebula tail (<0.004) into AgX so sub-threshold nebulae never clip hard
    let agx_blend = smoothstep(0.0, 0.004, max_ch);
    let blended = mix(hdr_rgb, val, agx_blend);

    // Subtle OLED saturation / contrast punch preserving zero floor
    let luma = luminance_rec709(blended);
    let punched = mix(vec3<f32>(luma), blended, 1.08);
    return clamp(punched, vec3<f32>(0.0), vec3<f32>(1.0));
}

fn linear_to_srgb_vec(c: vec3<f32>) -> vec3<f32> {
    let cutoff = vec3<f32>(0.0031308);
    let low = c * 12.92;
    let high = 1.055 * pow(max(c, vec3<f32>(1e-6)), vec3<f32>(1.0 / 2.4)) - 0.055;
    return select(high, low, c <= cutoff);
}

fn srgb_to_linear_vec(c: vec3<f32>) -> vec3<f32> {
    let cutoff = vec3<f32>(0.04045);
    let low = c * (1.0 / 12.92);
    let high = pow(max((c + 0.055) * (1.0 / 1.055), vec3<f32>(0.0)), vec3<f32>(2.4));
    return select(high, low, c <= cutoff);
}

fn blue_noise_tpdf(pixel_xy: vec2<f32>, frame_mod: f32) -> f32 {
    let dims = max(vec2<i32>(textureDimensions(blue_noise_tex, 0)), vec2<i32>(1));
    // Subtle R1 golden-ratio shift per frame breaks spatial fixed patterns on high-DPI OLED
    let shift = i32(round(fract(frame_mod * 0.61803398875) * f32(dims.x)));
    let coord = vec2<i32>((i32(pixel_xy.x) + shift) % dims.x, (i32(pixel_xy.y) + shift * 7) % dims.y);
    let bn = textureLoad(blue_noise_tex, coord, 0).r;
    let u = bn * 2.0 - 1.0;
    return clamp(sign(u) * (1.0 - sqrt(max(1.0 - abs(u), 0.0))), -1.0, 1.0);
}

@fragment
fn fs_final_oled_agx(in: PostVSOut) -> @location(0) vec4<f32> {
    let layer = i32(in.layer_idx);
    let radial = in.uv - vec2<f32>(0.5);
    let r2 = dot(radial, radial);

    // Optical edge chromatic dispersion scaled gently by scale flight speed
    let speed_kick = clamp(abs(u_frame.scale_params.y) * 0.06, 0.0, 1.5);
    let disp = u_frame.post_params.z * (1.0 + speed_kick) * r2;

    var scene_rgb: vec3<f32>;
    var bloom_rgb: vec3<f32>;

    if disp > 1e-5 {
        let uv_r = clamp(in.uv + radial * disp, vec2<f32>(0.0), vec2<f32>(1.0));
        let uv_g = in.uv;
        let uv_b = clamp(in.uv - radial * disp, vec2<f32>(0.0), vec2<f32>(1.0));

        let sr = textureSampleLevel(hdr_scene_tex, linear_samp, uv_r, layer, 0.0).r;
        let sg = textureSampleLevel(hdr_scene_tex, linear_samp, uv_g, layer, 0.0).g;
        let sb = textureSampleLevel(hdr_scene_tex, linear_samp, uv_b, layer, 0.0).b;
        scene_rgb = vec3<f32>(sr, sg, sb);

        let br = textureSampleLevel(bloom_tex, linear_samp, uv_r, layer, 0.0).r;
        let bg = textureSampleLevel(bloom_tex, linear_samp, uv_g, layer, 0.0).g;
        let bb = textureSampleLevel(bloom_tex, linear_samp, uv_b, layer, 0.0).b;
        bloom_rgb = vec3<f32>(br, bg, bb);
    } else {
        scene_rgb = textureSampleLevel(hdr_scene_tex, linear_samp, in.uv, layer, 0.0).rgb;
        bloom_rgb = textureSampleLevel(bloom_tex, linear_samp, in.uv, layer, 0.0).rgb;
    }

    let bloom_gain = max(u_frame.post_params.y, 0.0);
    var hdr = oled_void_clamp(scene_rgb + bloom_rgb * bloom_gain);

    // Hard OLED Void Floor Gate: pixels near #000000 bypass exposure, tonemap, and dithering completely
    let void_eps = max(u_post.grade_params.w, 2.5e-4);
    let hdr_max = max(hdr.r, max(hdr.g, hdr.b));
    if hdr_max <= void_eps {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }

    let vignette = 1.0 - clamp(u_post.grade_params.y * r2 * 2.2, 0.0, 0.45);
    hdr *= max(u_frame.scale_params.z, 0.01) * vignette;

    let encode_mode = u_post.grade_params.x;
    let dither_amp = max(u_frame.post_params.w, 0.0);
    let tri = select(0.0, blue_noise_tpdf(in.clip_pos.xy, u_frame.stereo_params.z), dither_amp > 0.0);

    // encode_mode >= 1.5 -> direct linear HDR output (for 10/16-bit HDR swapchains)
    if encode_mode >= 1.5 {
        let out_hdr = max(hdr + vec3<f32>(tri * (dither_amp / 1023.0)), vec3<f32>(0.0));
        return vec4<f32>(out_hdr, 1.0);
    }

    let mapped_linear = agx_tonemap_oled(hdr);
    var srgb = clamp(linear_to_srgb_vec(mapped_linear), vec3<f32>(0.0), vec3<f32>(1.0));

    // Only apply TPDF anti-banding above ~2/255 sRGB so near-black bloom tails never scatter (1,1,1) grey speckles across OLED black.
    let srgb_peak = max(srgb.r, max(srgb.g, srgb.b));
    let dither_gate = smoothstep(0.004, 0.018, srgb_peak);
    if dither_amp > 0.0 && dither_gate > 0.001 {
        srgb = clamp(srgb + vec3<f32>(tri * dither_gate * (dither_amp / 255.0)), vec3<f32>(0.0), vec3<f32>(1.0));
    }

    // encode_mode < 0.5: hardware sRGB target (e.g. Bgra8UnormSrgb) expects linear RGB
    // encode_mode < 1.5: UNORM non-sRGB target (e.g. Bgra8Unorm) expects encoded sRGB
    let out_rgb = select(srgb, srgb_to_linear_vec(srgb), encode_mode < 0.5);
    return vec4<f32>(out_rgb, 1.0);
}
