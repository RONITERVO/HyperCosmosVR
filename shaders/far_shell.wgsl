// HyperCosmosVR — Infinite Boundaryless Far-Shell Sky Dome
// Requires prelude: common_oled.wgsl
// Renders behind all scale geometry (Reversed-Z depth = 0.0, depth_write = false).
// Enforces literal #000000 OLED void floor across deep space & outer horizons.

struct FarShellUniforms {
    // x = galaxy_plane_alpha, y = cmb_alpha, z = inflation_alpha, w = lotus_alpha
    regime_weights: vec4<f32>,
    // xyz = normalized galactic north pole, w = star_density_scale
    galaxy_north: vec4<f32>,
    // xyz = normalized sun/core direction, w = sun_sky_glow
    solar_dir: vec4<f32>,
    // x = quantum_void_alpha (u < -22), y = cosmic_web_sky_alpha, z = seed_offset, w = reserved
    extra_weights: vec4<f32>,
};

@group(1) @binding(0) var<uniform> u_far: FarShellUniforms;

struct FarVsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) ndc_xy: vec2<f32>,
    @location(1) @interpolate(flat) eye_idx: u32,
};

fn build_far_vs(vid: u32, eye_idx: u32) -> FarVsOut {
    var pts = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0)
    );
    let p = pts[vid];
    var out: FarVsOut;
    // Reversed-Z far plane is 0.0
    out.clip_pos = vec4<f32>(p, 0.0, 1.0);
    out.ndc_xy = p;
    out.eye_idx = eye_idx;
    return out;
}

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> FarVsOut {
    return build_far_vs(vid, 0u);
}

@vertex
fn vs_stereo(
    @builtin(vertex_index) vid: u32,
    @builtin(view_index) view_idx: u32,
) -> FarVsOut {
    return build_far_vs(vid, view_idx);
}

fn eval_starfield_layer(dir: vec3<f32>, cell_scale: f32, threshold: f32, brightness: f32) -> vec3<f32> {
    let p = dir * cell_scale;
    let cell = floor(p);
    let f = fract(p) - vec3<f32>(0.5);
    let h = hash33(cell + vec3<f32>(u_far.extra_weights.z));
    if (h.x < threshold) {
        return vec3<f32>(0.0);
    }
    let jitter = (h - vec3<f32>(0.5)) * 0.62;
    let d = length(f - jitter);
    let core = exp(-d * d * 88.0);
    if (core < 1e-4) {
        return vec3<f32>(0.0);
    }
    let kelvin = mix(2900.0, 24000.0, pow(h.y, 1.6));
    let twinkle = 0.88 + 0.12 * sin(u_frame.scale_params.w * (1.2 + h.z * 2.5) + h.y * TAU);
    return blackbody_rgb(kelvin) * core * brightness * twinkle;
}

@fragment
fn fs_main(in: FarVsOut) -> @location(0) vec4<f32> {
    let dir = ndc_to_world_ray(in.ndc_xy, in.eye_idx);
    let time = u_frame.scale_params.w;
    let voice = u_audio.levels_a.x;
    let bass = u_audio.levels_a.z;

    var col = vec3<f32>(0.0);

    // 1. Milky Way Narrow Galactic Band & Pinpoint Starfield (u in [-12, +21])
    let mw_alpha = u_far.regime_weights.x;
    if (mw_alpha > 0.002) {
        let g_north = normalize(u_far.galaxy_north.xyz);
        let lat = abs(dot(dir, g_north));
        // Tighter galactic belt keeps >=75% of deep sky pure #000000 black
        let band = exp(-lat * lat * 74.0);

        let warp = fbm3(dir * 3.1 + vec3<f32>(time * 0.001, -0.7, 1.3), 3);
        let raw_neb = fbm3(dir * 5.6 + vec3<f32>(warp * 0.28), 4);
        let n_neb = pow(clamp((raw_neb + 0.08) * 1.55, 0.0, 1.0), 2.1);
        // Smooth broad dust attenuation without sharp crescent fingernail edges
        let dust_lanes = smoothstep(-0.32, 0.45, fbm3(dir * 7.2 + vec3<f32>(2.1, -1.4, 0.8), 3));
        let galactic_band = exp(-lat * lat * 86.0);

        let band_rgb = mix(vec3<f32>(0.012, 0.028, 0.075), vec3<f32>(0.15, 0.092, 0.052), n_neb);
        col += band_rgb * galactic_band * n_neb * (0.25 + 0.75 * dust_lanes) * (0.68 + bass * 0.18) * mw_alpha;

        // Razor-sharp blackbody pinpoint stars (zero smudgy halation)
        let density = max(u_far.galaxy_north.w, 1.0);
        col += eval_starfield_layer(dir, 180.0 * density, 0.974, 2.4) * mw_alpha;
        col += eval_starfield_layer(dir, 360.0 * density, 0.985, 1.15) * (0.35 + 0.65 * galactic_band) * mw_alpha;
    }

    // 2. Cosmic Web Deep Background Skeleton (u in [22, 26.2])
    let web_alpha = u_far.extra_weights.y;
    if (web_alpha > 0.002) {
        let web_n = abs(fbm3(dir * 3.6, 3));
        let ridge = pow(smoothstep(0.015, 0.0, web_n), 2.4);
        let node = pow(smoothstep(0.68, 0.96, snoise3(dir * 6.5)), 2.2);
        col += (vec3<f32>(0.24, 0.36, 0.95) * ridge + vec3<f32>(0.82, 0.52, 1.35) * node) * 0.45 * web_alpha;
    }

    // 3. Cosmic Microwave Background Organic Deep-Planck Curvature (09 / 10, u ~ 26.5)
    // Cool subdued Planck multipole fluctuations fading around central sightlines so cards & heroes pop cleanly over >=55% #000000 void.
    let cmb_alpha = u_far.regime_weights.y;
    if (cmb_alpha > 0.002) {
        // Zero-mean spherical harmonic curvature over the entire unit sphere (ZERO straight stripes!)
        let harm_l2 = 0.11 * (dir.x * dir.y + dir.y * dir.z + dir.z * dir.x) + 0.09 * sin(dir.x * 2.2) * cos(dir.y * 2.4);
        let fluc_broad = fbm3(dir * 4.4 + vec3<f32>(1.7, -3.2, 0.8), 4);
        let fluc_fine = snoise3(dir * 11.5 + vec3<f32>(-2.4, 1.9, 3.1)) * 0.18;
        let cmb_sig = fluc_broad + fluc_fine + harm_l2 - 0.08;

        // Broad smooth falloff with cool indigo-slate-amber Planck contrast (no bright beige leopard blotches)
        let island_mask = smoothstep(0.01, 0.34, cmb_sig);
        let cam_align = abs(dot(dir, normalize(u_frame.cam_forward.xyz)));
        let sight_atten = 0.42 + 0.58 * smoothstep(0.98, 0.65, cam_align);
        let deep_planck = vec3<f32>(0.016, 0.058, 0.21);
        let cool_slate = vec3<f32>(0.048, 0.105, 0.28);
        let muted_amber = vec3<f32>(0.22, 0.11, 0.045);
        let cmb_col = mix(
            deep_planck,
            mix(cool_slate, muted_amber, smoothstep(0.20, 0.48, cmb_sig)),
            smoothstep(0.03, 0.24, cmb_sig)
        );
        col += cmb_col * island_mask * sight_atten * (0.22 + 0.06 * voice) * cmb_alpha;
    }

    // 4. Primordial Inflationary Spherical Quantum Bubble Caustics (10, u ~ 27.0)
    // Organic 3D spherical bubble walls sculpted by smooth broad pockets (ZERO rectangular belt).
    let inf_alpha = u_far.regime_weights.z;
    if (inf_alpha > 0.002) {
        let inf_pocket = smoothstep(-0.08, 0.34, fbm3(dir * 2.1 + vec3<f32>(0.8, -1.1, 0.4), 3));
        let inf_n = fbm3(dir * 6.2 + vec3<f32>(time * 0.04, -time * 0.03, 0.0), 4);
        let bubble_wall = pow(smoothstep(0.038, 0.0, abs(inf_n)), 2.8);
        let irid = iridescence_spectral(abs(dot(dir, u_frame.cam_forward.xyz)), inf_n * 1.4, time * 0.05 + voice * 0.2);
        col += irid * bubble_wall * inf_pocket * 0.85 * inf_alpha;
    }

    // 5. Outer 5-Petaled Hyper-Horizon Dark-Void Filigree (11, 12, 23, 24, 25, u > 27.2)
    // Razor-thin sky filigree ridges only — zero broad neon corner blotches!
    let lotus_alpha = u_far.regime_weights.w;
    if (lotus_alpha > 0.002) {
        let az = atan2(dir.z, dir.x);
        let elev = dir.y;
        let arc_wave = abs(elev) - 0.14 - 0.06 * cos(az * 5.0 + time * 0.015);
        let ridge = pow(smoothstep(0.014, 0.0, abs(arc_wave)), 2.4) * (0.30 + 0.70 * pow(abs(cos(az * 2.5 + time * 0.015)), 5.0));
        let hairline = pow(smoothstep(0.009, 0.0, abs(elev + 0.08 * sin(az * 5.0))), 2.6) * 0.35;
        let petal_tint = mix(hsl_to_rgb(fract(az * INV_PI * 0.5 + 0.55), 0.65, 0.50), vec3<f32>(0.55, 0.62, 0.85), 0.45);
        col += petal_tint * (ridge + hairline) * (0.14 + 0.05 * voice) * lotus_alpha;
    }

    // 6. Sub-Planck Quantum Vacuum Horizon (u < -22) — delicate deep-void lattice preserving >=70% OLED black
    let q_alpha = u_far.extra_weights.x;
    if (q_alpha > 0.002) {
        let grid_f = abs(sin(dir.x * 22.0 + time * 0.15) * sin(dir.y * 22.0) * sin(dir.z * 22.0));
        let edge = pow(smoothstep(0.042, 0.0, grid_f), 3.0);
        let q_col = iridescence_spectral(dir.y, time * 0.04, bass * 0.2);
        let sight_vignette = mix(0.28, 0.95, smoothstep(0.14, 0.72, length(dir.xy)));
        col += q_col * edge * 0.105 * sight_vignette * q_alpha;
    }

    // Preserve literal #000000 OLED floor everywhere sub-threshold
    return vec4<f32>(oled_void_clamp(col), 1.0);
}

