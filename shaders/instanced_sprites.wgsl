// HyperCosmosVR — GPU-Instanced Blackbody Stars, Filaments, Exoplanet/Signal Cones & Diegetic 3D UI
// Requires prelude: common_oled.wgsl
// Zero-vertex-buffer procedural quad expansion (6 vertices per instance via @builtin(vertex_index)).
// Entry points:
//   - vs_particle_main, vs_particle_stereo -> reads compute_universe SimParticle buffer (@binding(1))
//   - vs_sprite_main,   vs_sprite_stereo   -> reads explicit SpriteInstance buffer (@binding(2))
//   - fs_main                              -> shared OLED sprite fragment shader

struct SpriteBatchUniforms {
    // xyz = sector-local origin relative to camera (render units), w = sector_scale_multiplier
    sector_origin_scale: vec4<f32>,
    // x = default_sprite_kind (for SimParticle pass), y = min_pixel_size, z = global_alpha, w = time_sec
    batch_params: vec4<f32>,
};

// Matches compute_universe.wgsl SimParticle (64 B).
struct SimParticle {
    pos_age: vec4<f32>,
    vel_mass: vec4<f32>,
    color_size: vec4<f32>,
    tags: vec4<u32>,
};

// Explicit 64-byte world-anchored or diegetic 3D sprite instance.
struct SpriteInstance {
    // xyz = camera-relative position (render units), w = half-size radius (render units)
    pos_size: vec4<f32>,
    // xyz = elongation axis / cone forward dir (render units), w = aspect elongation (1.0 = square quad)
    axis_aspect: vec4<f32>,
    // rgb = linear HDR color, a = alpha / glow multiplier
    color_alpha: vec4<f32>,
    // x = sprite_kind (0:star, 1:filament, 2:shard, 3:biosig/signal, 4:wrist/dwell_ring), y = phase, z = sub_param, w = flags
    tags_f: vec4<f32>,
};

@group(1) @binding(0) var<uniform> u_batch: SpriteBatchUniforms;
@group(1) @binding(1) var<storage, read> sim_particles: array<SimParticle>;
@group(1) @binding(2) var<storage, read> sprite_instances: array<SpriteInstance>;

struct SpriteVsOut {
    @builtin(position) clip_pos: vec4<f32>,
    @location(0) quad_uv: vec2<f32>,
    @location(1) color_alpha: vec4<f32>,
    @location(2) @interpolate(flat) kind_phase: vec4<f32>,
};

fn quad_corner(vid: u32) -> vec2<f32> {
    var corners = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>(-1.0,  1.0)
    );
    return corners[vid % 6u];
}

fn expand_billboard_vs(
    world_pos: vec3<f32>,
    half_size: f32,
    axis_dir: vec3<f32>,
    aspect: f32,
    color_alpha: vec4<f32>,
    kind_phase: vec4<f32>,
    vid: u32,
    eye_idx: u32,
) -> SpriteVsOut {
    let corner = quad_corner(vid);
    let eye_pos = u_frame.eye_offset[eye_idx].xyz;
    let to_eye = normalize(eye_pos - world_pos);

    var right = u_frame.cam_right.xyz;
    var up = u_frame.cam_up.xyz;
    if (aspect > 1.05 && length(axis_dir) > 1e-4) {
        up = normalize(axis_dir);
        right = normalize(cross(up, to_eye));
        if (length(right) < 1e-4) {
            right = u_frame.cam_right.xyz;
        }
    } else {
        right = normalize(cross(u_frame.cam_up.xyz, to_eye));
        if (length(right) < 1e-4) {
            right = u_frame.cam_right.xyz;
        }
        up = normalize(cross(to_eye, right));
    }

    // Distance-aware minimum angular pixel guard + graceful near-camera fade for particles & background motes
    let dist = max(length(world_pos - eye_pos), 1e-4);
    let min_world_half = dist * u_batch.batch_params.y * u_frame.viewport_params.w;
    let is_ui = abs(kind_phase.x - 4.0) < 0.25;
    let capped_half = select(min(half_size, dist * 0.085), half_size, is_ui);
    let r_eff = max(capped_half, min_world_half);
    let near_fade = select(smoothstep(0.45, 1.35, dist), 1.0, is_ui);
    let aspect_eff = select(min(aspect, 2.2), aspect, is_ui);

    let displaced = world_pos + right * (corner.x * r_eff) + up * (corner.y * r_eff * aspect_eff);
    var out: SpriteVsOut;
    out.clip_pos = world_pos_to_clip(displaced, eye_idx);
    out.quad_uv = corner;
    out.color_alpha = vec4<f32>(color_alpha.rgb, color_alpha.a * near_fade);
    out.kind_phase = kind_phase;
    return out;
}

@vertex
fn vs_particle_main(
    @builtin(vertex_index) vid: u32,
    @builtin(instance_index) inst_idx: u32,
) -> SpriteVsOut {
    let pt = sim_particles[inst_idx];
    let scale = u_batch.sector_origin_scale.w;
    let world_pos = u_batch.sector_origin_scale.xyz + pt.pos_age.xyz * scale;
    let dist_eye = max(length(world_pos - u_frame.eye_offset[0].xyz), 1e-4);
    let spd = length(pt.vel_mass.xyz);
    let kind = u_batch.batch_params.x;
    let aspect = select(1.0, clamp(1.0 + spd * 0.35, 1.0, 2.4), kind == 1.0);
    let axis = select(vec3<f32>(0.0), pt.vel_mass.xyz, spd > 1e-5);
    return expand_billboard_vs(
        world_pos,
        min(pt.color_size.a * scale, dist_eye * 0.0055),
        axis,
        aspect,
        vec4<f32>(pt.color_size.rgb, u_batch.batch_params.z),
        vec4<f32>(kind, pt.pos_age.w, f32(pt.tags.x), 0.0),
        vid,
        0u,
    );
}

@vertex
fn vs_particle_stereo(
    @builtin(vertex_index) vid: u32,
    @builtin(instance_index) inst_idx: u32,
    @builtin(view_index) view_idx: u32,
) -> SpriteVsOut {
    let pt = sim_particles[inst_idx];
    let scale = u_batch.sector_origin_scale.w;
    let world_pos = u_batch.sector_origin_scale.xyz + pt.pos_age.xyz * scale;
    let dist_eye = max(length(world_pos - u_frame.eye_offset[view_idx].xyz), 1e-4);
    let spd = length(pt.vel_mass.xyz);
    let kind = u_batch.batch_params.x;
    let aspect = select(1.0, clamp(1.0 + spd * 0.35, 1.0, 2.4), kind == 1.0);
    let axis = select(vec3<f32>(0.0), pt.vel_mass.xyz, spd > 1e-5);
    return expand_billboard_vs(
        world_pos,
        min(pt.color_size.a * scale, dist_eye * 0.0055),
        axis,
        aspect,
        vec4<f32>(pt.color_size.rgb, u_batch.batch_params.z),
        vec4<f32>(kind, pt.pos_age.w, f32(pt.tags.x), 0.0),
        vid,
        view_idx,
    );
}

@vertex
fn vs_sprite_main(
    @builtin(vertex_index) vid: u32,
    @builtin(instance_index) inst_idx: u32,
) -> SpriteVsOut {
    let sp = sprite_instances[inst_idx];
    return expand_billboard_vs(
        sp.pos_size.xyz,
        sp.pos_size.w,
        sp.axis_aspect.xyz,
        max(sp.axis_aspect.w, 1.0),
        sp.color_alpha,
        sp.tags_f,
        vid,
        0u,
    );
}

@vertex
fn vs_sprite_stereo(
    @builtin(vertex_index) vid: u32,
    @builtin(instance_index) inst_idx: u32,
    @builtin(view_index) view_idx: u32,
) -> SpriteVsOut {
    let sp = sprite_instances[inst_idx];
    return expand_billboard_vs(
        sp.pos_size.xyz,
        sp.pos_size.w,
        sp.axis_aspect.xyz,
        max(sp.axis_aspect.w, 1.0),
        sp.color_alpha,
        sp.tags_f,
        vid,
        view_idx,
    );
}

@fragment
fn fs_main(in: SpriteVsOut) -> @location(0) vec4<f32> {
    let uv = in.quad_uv;
    let r = length(uv);
    if (r >= 1.0) {
        discard;
    }

    let kind = i32(round(in.kind_phase.x));
    let phase = in.kind_phase.y;
    let sub_p = in.kind_phase.z;
    let time = u_frame.scale_params.w;

    var col = in.color_alpha.rgb;
    var alpha = in.color_alpha.a;

    if (kind == 0) {
        // Tier 1/2 Pinpoint Blackbody Star Sprite (zero giant bokeh bloom circles)
        let core = exp(-r * r * 20.0);
        let halo = exp(-r * 7.2) * 0.14;
        let spike_x = exp(-abs(uv.y) * 34.0) * exp(-abs(uv.x) * 4.5);
        let spike_y = exp(-abs(uv.x) * 34.0) * exp(-abs(uv.y) * 4.5);
        let edge_kill = smoothstep(0.95, 0.58, r);
        let intensity = (core * 1.35 + halo + (spike_x + spike_y) * 0.22) * edge_kill;
        col *= intensity;
        alpha *= clamp(intensity, 0.0, 1.0);
    } else if (kind == 1) {
        // Fine Volumetric Filament / Flow Streamline Splat
        let ribbon = exp(-uv.x * uv.x * 11.5) * exp(-uv.y * uv.y * 4.5) * smoothstep(0.95, 0.52, r) * 0.72;
        let pulse = 0.78 + 0.22 * sin(phase * TAU + time * 2.5);
        col *= ribbon * pulse;
        alpha *= ribbon;
    } else if (kind == 2) {
        // Procedural Asteroid / Kuiper Ice / Dyson Shard Impostor (analytic normal lighting)
        let n_warp = snoise3(vec3<f32>(uv * 3.2, phase * 10.0)) * 0.16;
        let r_rock = r + n_warp;
        if (r_rock > 0.85) {
            discard;
        }
        let z_n = sqrt(max(0.0, 0.7225 - r_rock * r_rock));
        let normal = normalize(vec3<f32>(uv, z_n));
        let light_dir = normalize(vec3<f32>(0.55, 0.65, 0.52));
        let ndotl = max(dot(normal, light_dir), 0.0);
        let edge_aa = smoothstep(0.85, 0.76, r_rock);
        col *= (ndotl * 1.15 + 0.03);
        alpha *= edge_aa;
    } else if (kind == 3) {
        if (sub_p > 0.5) {
            // Spectral / DNA / Prime Barcode Micro-Ribbon (Apps 01, 11, 16, 21, 22)
            let bar_mask = smoothstep(0.92, 0.72, abs(uv.x)) * smoothstep(0.55, 0.25, abs(uv.y));
            let bands = 0.25 + 0.75 * smoothstep(0.15, 0.55, abs(sin((uv.x * 11.0 + phase * TAU) * 1.7)));
            let spec_col = iridescence_spectral(abs(uv.y), uv.x * 0.45 + phase, 0.0) * 1.15;
            let inten = bar_mask * bands;
            col = mix(col, spec_col, 0.55) * inten;
            alpha *= clamp(inten, 0.0, 1.0);
        } else {
            // Pinpoint Spectral Beacon Gem (zero large hollow floating circle rings in deep space)
            let core = exp(-r * r * 72.0);
            let cross_arm = exp(-abs(uv.x) * 28.0) * exp(-abs(uv.y) * 9.0) + exp(-abs(uv.y) * 28.0) * exp(-abs(uv.x) * 9.0);
            let spectral = iridescence_spectral(r, phase + time * 0.04, sub_p);
            let sig = (core * 1.45 + cross_arm * 0.35) * smoothstep(1.0, 0.55, r);
            col = mix(col, spectral * 1.25, 0.40) * sig;
            alpha *= clamp(sig, 0.0, 1.0);
        }
    } else if (kind == 5) {
        if (sub_p > 0.75) {
            // Faceted Hexagonal Dyson Collector Mirror Glint (Apps 20, 25)
            let p_hex = abs(uv);
            let hex_d = max(p_hex.x * 0.8660254 + p_hex.y * 0.5, p_hex.y);
            if (hex_d > 0.78) {
                discard;
            }
            let rim = exp(-pow(abs(hex_d - 0.72) * 24.0, 2.0));
            let glint = 0.25 + 0.95 * pow(max(0.0, cos(phase * TAU + time * 1.6)), 4.0);
            col *= (rim * 1.25 + 0.18) * glint;
            alpha *= smoothstep(0.78, 0.68, hex_d);
        } else if (sub_p > 0.25) {
            // Analytic Shaded Mini-Moon / Exoplanet / Quark Impostor Sphere (Apps 04, 13, 21)
            if (r > 0.82) {
                discard;
            }
            let z_n = sqrt(max(0.0, 0.6724 - r * r));
            let n = normalize(vec3<f32>(uv, z_n));
            let ldir = normalize(vec3<f32>(cos(phase * TAU), 0.45, sin(phase * TAU) + 0.4));
            let ndotl = max(dot(n, ldir), 0.0);
            let fres = pow(1.0 - max(n.z, 0.0), 3.0);
            let tex = 0.78 + 0.22 * snoise3(n * 4.5 + vec3<f32>(phase * 8.0));
            col = col * (ndotl * tex * 1.2 + 0.02) + col * fres * 0.42;
            alpha *= smoothstep(0.82, 0.72, r);
        } else {
            // Crisp Starburst Spark Mote
            let core = exp(-r * r * 64.0);
            let glint = exp(-abs(uv.x + uv.y) * 22.0) * exp(-abs(uv.x - uv.y) * 22.0) * 0.55;
            let intensity = (core + glint) * smoothstep(1.0, 0.5, r);
            col *= intensity * 1.2;
            alpha *= clamp(intensity, 0.0, 1.0);
        }
    } else if (kind == 6) {
        // Directional Minkowski Light Cone / SETI Radio Wedge (Apps 14, 22)
        let ax_y = clamp((uv.y + 0.85) * 0.55, 0.0, 1.0);
        let cone_w = max(0.06, ax_y * 0.75);
        let in_wedge = exp(-pow(max(abs(uv.x) - cone_w, 0.0) * 24.0, 2.0)) * smoothstep(1.0, 0.8, abs(uv.y));
        let wave = pow(max(0.0, cos(ax_y * 18.0 - time * 3.5 + phase * TAU)), 4.0);
        let sig = in_wedge * (0.28 + 0.85 * wave);
        col *= sig;
        alpha *= clamp(sig, 0.0, 1.0);
    } else if (kind == 7) {
        // Razor-Crisp Hairline Orbital Track / Radar Ping Tick (Apps 04, 11, 22, 23)
        let ring = exp(-pow((r - 0.88) * 38.0, 2.0));
        let dash = 0.35 + 0.65 * smoothstep(-0.2, 0.4, sin(atan2(uv.y, uv.x) * 8.0 + phase * TAU));
        let sig = ring * mix(1.0, dash, step(0.5, sub_p)) * smoothstep(1.0, 0.92, r);
        col *= sig * 1.25;
        alpha *= clamp(sig, 0.0, 1.0);
    } else {
        // Kind 4: Diegetic 3D Wrist Resonance Ring Bead, Dwell Focus Shimmer, & Ephemeral Scale Arc
        let is_ring = sub_p > 0.5;
        if (is_ring) {
            // Crystalline glass-and-light ring / dwell focus shimmer
            let rim = exp(-pow((r - 0.78) * 14.0, 2.0));
            let center_mote = exp(-r * r * 18.0) * 0.65;
            let glow = (rim * 1.4 + center_mote) * smoothstep(1.0, 0.88, r);
            col *= glow * (1.0 + 0.25 * u_audio.levels_a.x);
            alpha *= clamp(glow, 0.0, 1.0);
        } else {
            // Solid glowing crystalline bead (1x..3x speed step or EN/FI pearl)
            let z_n = sqrt(max(0.0, 1.0 - r * r));
            let fres = pow(1.0 - z_n, 2.4);
            let spec = pow(max(dot(normalize(vec3<f32>(uv, z_n)), normalize(vec3<f32>(0.4, 0.7, 0.6))), 0.0), 32.0);
            col = col * (0.6 + fres * 1.4) + vec3<f32>(spec * 1.8);
            alpha *= smoothstep(1.0, 0.88, r);
        }
    }

    if (alpha < 1e-4) {
        discard;
    }
    return vec4<f32>(oled_void_clamp(col), clamp(alpha, 0.0, 1.0));
}

