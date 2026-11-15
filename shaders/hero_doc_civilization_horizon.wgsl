// ============================================================================
// HyperCosmosVR — Documentary Hero Shaders: Evidence, Future, Culture,
//                 Technology, Futures, Cosmic Life, Contact, Perspectives,
//                 Meaning & Final Horizon (Experiences 11, 12, 18..25)
// Pure OLED #000000 void, architectural SDF sculptures & sharp bloom edges.
// Respects doc_zone + doc_feat disambiguation (App 22 shared scenes,
// Apps 23..25 intra-zone feature morphs) & App 25 thermodynamic fade.
// Concatenated after common_oled.wgsl, doc_common.wgsl, hero_doc_cosmos.wgsl,
// and hero_doc_matter_mind.wgsl.
// ============================================================================

fn ch_parabolic_dish_sdf(p: vec3<f32>, radius: f32, depth: f32) -> f32 {
    let r = length(p.xz);
    let y_surf = depth * (r * r) / max(radius * radius, 1e-4) - depth * 0.45;
    let d_shell = abs(p.y - y_surf) - 0.014;
    let d_rim = r - radius;
    return max(d_shell, d_rim);
}

fn ch_exp_and_sub(vid: u32) -> vec2<u32> {
    if (doc_is_canonical_zone_vid(vid)) {
        return vec2<u32>(doc_vid_exp(vid), doc_vid_zone(vid));
    }
    if (vid >= 100u) {
        return vec2<u32>(vid / 10u, vid % 10u);
    }
    return vec2<u32>(vid, 99u);
}

// ============================================================================
// EXP 11 (COSMIC EVIDENCE) & EXP 12 (FUTURE OF UNIVERSE)
// ============================================================================
fn march_doc_evidence_future(
    exp_id: u32,
    sub_v: u32,
    p: vec3<f32>,
    scene: i32,
    zone: u32,
    feat: u32,
    cue: f32,
    zone_col: vec3<f32>,
    time: f32,
    voice: f32,
    bass: f32,
) -> vec4<f32> {
    var col = vec3<f32>(0.0);
    var dens: f32 = 0.0;
    let r = length(p);

    if (exp_id == 11u) {
        let mode = select(u32(max(scene, 0)), sub_v, sub_v < 10u);
        if (mode == 0u) {
            // Hubble Redshift Ladder & Fraunhofer Spectral Absorption Comb
            for (var k = 0; k < 5; k = k + 1) {
                let fk = f32(k) / 4.0;
                let pz = mix(-0.48, 0.48, fk);
                let gal_r = mix(0.28, 0.14, fk);
                let d_disk = sd_torus(vec3<f32>(p.x, p.z - pz, p.y), gal_r * 0.65, 0.015);
                let g_disk = doc_line_glow(d_disk, 58.0);
                let shift_col = mix(vec3<f32>(0.32, 0.88, 1.48), vec3<f32>(1.45, 0.24, 0.18), fk);
                col += shift_col * g_disk;
                dens += g_disk * 0.8;
            }
            if (p.y < -0.24 && p.y > -0.38 && abs(p.z) < 0.56 && abs(p.x) < 0.08) {
                let z_t = (p.z + 0.56) / 1.12;
                let lines = smoothstep(0.08, 0.02, abs(fract(z_t * 6.0 - 0.15) - 0.5));
                let bar = mix(vec3<f32>(0.24, 0.72, 1.38), vec3<f32>(1.42, 0.22, 0.18), z_t) * (1.0 - 0.85 * lines);
                let g_bar = exp(-pow((p.y + 0.31) * 28.0, 2.0)) * exp(-pow(p.x * 26.0, 2.0));
                col += bar * g_bar * 1.1;
                dens += g_bar * 0.75;
            }
        } else if (mode == 1u) {
            // CMB Acoustic Multipole Sphere (l=200 Doppler filigree, pure black valleys)
            let d_sph = abs(r - 0.46);
            if (d_sph < 0.05) {
                let dir = normalize(p + vec3<f32>(1e-4));
                let ylm = sin(dir.x * 9.0 + cos(dir.y * 8.0)) * cos(dir.z * 9.0 - sin(dir.x * 7.0));
                let shell = exp(-pow(d_sph * 58.0, 2.0));
                let hot = smoothstep(0.05, 0.55, ylm);
                let cold = smoothstep(-0.05, -0.55, ylm);
                let cmb_c = vec3<f32>(1.42, 0.82, 0.28) * hot + vec3<f32>(0.22, 0.68, 1.45) * cold;
                col += cmb_c * shell * 1.15;
                dens += shell * (hot + cold) * 0.9;
            }
        } else if (mode == 2u) {
            // Einstein Cross Gravitational Lens Quasar
            let lens_r = 0.34;
            let phi = atan2(p.y, p.x);
            let four_fold = pow(max(cos(2.0 * phi), 0.0), 6.0);
            let arc_d = length(vec2<f32>(length(p.xy) - lens_r, p.z));
            let arc_g = exp(-pow(arc_d * 46.0, 2.0)) * (0.15 + 1.25 * four_fold);
            let quasar = exp(-pow(length(vec2<f32>(length(p.xy) - lens_r, p.z)) * 22.0, 2.0)) * pow(abs(sin(2.0 * phi + 0.785)), 12.0);
            let dark_halo = exp(-pow(abs(r - 0.14) * 58.0, 2.0));
            col += vec3<f32>(0.42, 0.95, 1.48) * arc_g + vec3<f32>(1.48, 1.18, 0.52) * quasar + vec3<f32>(0.65, 0.42, 1.25) * dark_halo * 0.55;
            dens += arc_g * 0.85 + quasar + dark_halo * 0.4;
        } else {
            // Cosmic Distance & BBN Isotope Concentric Ladder Rings
            for (var n = 0; n < 4; n = n + 1) {
                let fn_ = f32(n);
                let rad = 0.16 + fn_ * 0.14;
                let pr = doc_rot_x(p, fn_ * 0.42 + f32(feat) * 0.15);
                let rg = doc_line_glow(sd_torus(pr, rad, 0.006), 64.0);
                let ring_c = mix(vec3<f32>(0.32, 1.18, 1.45), zone_col * 1.25, fn_ / 3.0);
                col += ring_c * rg;
                dens += rg * 0.75;
            }
        }
    } else {
        // EXP 12 — FUTURE OF THE UNIVERSE
        let mode = select(u32(max(scene, 0)), sub_v, sub_v < 10u);
        if (mode <= 1u) {
            // Milkomeda Twin-Galaxy Tidal Merger
            let c1 = vec3<f32>(-0.22 * cos(time * 0.3), 0.08, -0.14 * sin(time * 0.3));
            let c2 = -c1;
            let d1 = sd_torus(doc_rot_x(p - c1, 0.45), 0.26, 0.022);
            let d2 = sd_torus(doc_rot_z(p - c2, -0.52), 0.24, 0.020);
            let g1 = doc_line_glow(d1, 52.0);
            let g2 = doc_line_glow(d2, 52.0);
            let core_g = exp(-pow(length(p - c1) * 14.0, 2.0)) + exp(-pow(length(p - c2) * 14.0, 2.0));
            col += vec3<f32>(1.38, 0.88, 0.38) * g1 + vec3<f32>(0.42, 0.92, 1.48) * g2 + vec3<f32>(1.48, 1.25, 0.78) * core_g;
            dens += g1 + g2 + core_g * 0.8;
        } else if (mode == 2u) {
            // Cooling Red -> Degenerate Black Dwarf Ember
            let ember = exp(-pow(abs(r - 0.34) * 42.0, 2.0)) * (0.65 + 0.35 * sin(p.x * 16.0) * cos(p.y * 16.0));
            let core_glow = exp(-pow(r * 9.0, 2.0)) * 0.45;
            col += vec3<f32>(1.18, 0.22, 0.08) * ember + vec3<f32>(0.85, 0.20, 0.12) * core_glow;
            dens += ember + core_glow;
        } else if (mode == 3u) {
            // Hawking Evaporation Black Hole
            let ring_g = exp(-pow(abs(r - 0.24) * 58.0, 2.0));
            let rad_ang = atan2(p.z, p.x);
            let escape_ray = pow(max(sin(rad_ang * 7.0 - r * 14.0 + time * 3.0), 0.0), 10.0) * smoothstep(0.24, 0.28, r) * smoothstep(0.75, 0.3, r);
            col += vec3<f32>(0.55, 1.05, 1.48) * (ring_g + escape_ray * 0.9);
            dens += ring_g + escape_ray * 0.75;
        } else {
            // Arrow of Time / Big Rip / Heat Death Expanding Horizon Cone
            let r_yz = length(p.yz);
            let cone_r = 0.06 + 0.56 * smoothstep(-0.60, 0.60, p.x);
            let cone_g = exp(-pow(abs(r_yz - cone_r) * 44.0, 2.0)) * smoothstep(0.65, 0.55, abs(p.x));
            let slice_r = exp(-pow(abs(fract(p.x * 5.0 - time * 0.2) - 0.5) * 14.0, 2.0));
            let time_col = mix(vec3<f32>(1.45, 0.92, 0.28), vec3<f32>(0.25, 0.45, 1.38), smoothstep(-0.5, 0.5, p.x));
            col += time_col * cone_g * (0.35 + 0.95 * slice_r) * (1.0 + 0.25 * cue);
            dens += cone_g * (0.4 + 0.8 * slice_r);
        }
    }
    return vec4<f32>(col, dens);
}

// ============================================================================
// EXP 18 (CULTURE), EXP 19 (TECHNOLOGY) & EXP 20 (FUTURES)
// ============================================================================
fn march_doc_civilization(
    exp_id: u32,
    sub_v: u32,
    p: vec3<f32>,
    scene: i32,
    zone: u32,
    feat: u32,
    cue: f32,
    zone_col: vec3<f32>,
    time: f32,
    voice: f32,
    bass: f32,
) -> vec4<f32> {
    var col = vec3<f32>(0.0);
    var dens: f32 = 0.0;

    if (exp_id == 18u) {
        let ch = select(u32(max(scene, 0)), sub_v, sub_v < 10u);
        for (var tier = 0; tier < 3; tier = tier + 1) {
            let ft = f32(tier);
            let tr = 0.52 - ft * 0.13;
            let ty = -0.30 + ft * 0.10;
            let tg = doc_line_glow(sd_torus(p - vec3<f32>(0.0, ty, 0.0), tr, 0.006), 66.0);
            col += mix(vec3<f32>(1.32, 0.88, 0.34), zone_col, 0.35) * tg * 0.75;
            dens += tg * 0.55;
        }
        if (ch == 0u) {
            let flame = exp(-pow(length(vec3<f32>(p.x * 1.4, p.y + 0.05, p.z * 1.4)) * 11.0, 2.0));
            col += vec3<f32>(1.48, 0.62, 0.16) * flame * (0.9 + 0.35 * sin(time * 7.0 + p.y * 12.0));
            dens += flame;
        } else if (ch == 1u || ch == 2u) {
            let zig = min(
                sd_box(p - vec3<f32>(0.0, -0.10, 0.0), vec3<f32>(0.24, 0.06, 0.24)),
                sd_box(p - vec3<f32>(0.0, 0.04, 0.0), vec3<f32>(0.15, 0.06, 0.15))
            );
            let zg = exp(-pow(abs(zig) * 52.0, 2.0));
            col += vec3<f32>(1.42, 1.02, 0.36) * zg;
            dens += zg * 0.9;
        } else if (ch == 3u) {
            let dome = abs(length(p - vec3<f32>(0.0, 0.06, 0.0)) - 0.24);
            let dg = exp(-pow(dome * 52.0, 2.0)) * step(0.06, p.y);
            col += vec3<f32>(0.92, 1.25, 1.45) * dg;
            dens += dg * 0.85;
        } else if (ch == 4u || ch == 5u) {
            let a1 = doc_line_glow(sd_torus(doc_rot_x(p, 0.6), 0.34, 0.007), 64.0);
            let a2 = doc_line_glow(sd_torus(doc_rot_z(p, 0.8), 0.30, 0.007), 64.0);
            col += vec3<f32>(0.32, 1.22, 1.45) * a1 + vec3<f32>(1.42, 0.92, 0.32) * a2;
            dens += (a1 + a2) * 0.85;
        } else {
            let web_r = 0.32;
            let lat = exp(-pow(abs(fract(p.y * 12.0) - 0.5) * 16.0, 2.0));
            let lon = exp(-pow(abs(fract(atan2(p.z, p.x) / TAU * 10.0) - 0.5) * 16.0, 2.0));
            let wg = exp(-pow(abs(length(p) - web_r) * 52.0, 2.0)) * max(lat, lon);
            col += vec3<f32>(0.35, 1.25, 1.48) * wg;
            dens += wg * 0.9;
        }
    } else if (exp_id == 19u) {
        let tech = select(u32(max(scene, 0)), sub_v, sub_v < 10u);
        if (tech <= 1u) {
            for (var g = 0; g < 2; g = g + 1) {
                let off = select(vec3<f32>(-0.20, 0.0, 0.0), vec3<f32>(0.20, 0.0, 0.0), g == 1);
                let pg = p - off;
                let a = atan2(pg.y, pg.x) + select(time * 0.9, -time * 0.9 + 0.25, g == 1);
                let tooth = 0.19 + 0.03 * step(0.0, sin(a * 12.0));
                let rim = exp(-pow(abs(length(pg.xy) - tooth) * 48.0, 2.0)) * exp(-pow(abs(pg.z) * 28.0, 2.0));
                col += mix(vec3<f32>(1.42, 0.92, 0.32), vec3<f32>(0.52, 1.15, 1.45), f32(g)) * rim;
                dens += rim * 0.9;
            }
        } else if (tech == 2u || tech == 3u) {
            let c1 = sd_cylinder_y(p - vec3<f32>(-0.30, -0.08, 0.0), 0.07, 0.28);
            let c2 = sd_cylinder_y(p - vec3<f32>(0.30, -0.08, 0.0), 0.07, 0.28);
            let coil_g = exp(-pow(abs(min(c1, c2)) * 45.0, 2.0)) * (0.4 + 0.7 * abs(sin(p.y * 42.0)));
            let arc_y = 0.22 + 0.05 * sin(p.x * 22.0 + time * 14.0) * sin(p.x * 13.0 - time * 9.0);
            let arc_d = length(vec2<f32>(p.y - arc_y, p.z)) + step(0.32, abs(p.x)) * 0.5;
            let arc_g = exp(-pow(arc_d * 48.0, 2.0));
            col += vec3<f32>(1.32, 0.78, 0.28) * coil_g + vec3<f32>(0.42, 1.25, 1.50) * arc_g * (1.0 + 0.45 * voice);
            dens += coil_g + arc_g;
        } else if (tech == 4u || tech == 5u) {
            // Crisp Etched Monocrystalline Silicon Die Wafer + Gold Micro-Bus Traces + Transformer Attention Web
            let p_wafer = doc_rot_x(p - vec3<f32>(0.0, -0.16, 0.0), 0.36);
            let wafer_box = sd_box(p_wafer, vec3<f32>(0.46, 0.022, 0.46));
            let wafer_edge = exp(-pow(abs(wafer_box) * 95.0, 2.0));
            let wafer_fill = smoothstep(0.015, -0.02, wafer_box) * 0.14;
            let gx = exp(-pow(abs(fract(p_wafer.x * 7.0) - 0.5) * 26.0, 2.0));
            let gz = exp(-pow(abs(fract(p_wafer.z * 7.0) - 0.5) * 26.0, 2.0));
            let in_die = smoothstep(0.46, 0.40, max(abs(p_wafer.x), abs(p_wafer.z))) * exp(-pow(p_wafer.y * 56.0, 2.0));
            let traces = max(gx, gz) * in_die;
            let inner_core = exp(-pow(abs(max(abs(p_wafer.x), abs(p_wafer.z)) - 0.18) * 65.0, 2.0)) * in_die;
            col += vec3<f32>(0.22, 0.92, 1.48) * (wafer_edge + wafer_fill)
                 + vec3<f32>(1.45, 0.98, 0.28) * (traces * 0.95 + inner_core);
            dens += wafer_edge * 0.85 + wafer_fill * 0.45 + traces * 0.85 + inner_core * 0.7;

            // Elevated 3-Layer Transformer Self-Attention Graph above Silicon Die
            for (var n = 0; n < 3; n = n + 1) {
                let x_in = (f32(n) - 1.0) * 0.28;
                let a0 = vec3<f32>(x_in, -0.04, 0.0);
                let a1 = vec3<f32>(-x_in * 0.65, 0.22, 0.0);
                let a2 = vec3<f32>(x_in * 0.45, 0.44, 0.0);
                let g_nodes = exp(-pow(length(p - a0) * 24.0, 2.0))
                            + exp(-pow(length(p - a1) * 24.0, 2.0))
                            + exp(-pow(length(p - a2) * 22.0, 2.0));
                let ray1 = doc_line_glow(sd_capsule(p, a0, a1, 0.006), 96.0);
                let ray2 = doc_line_glow(sd_capsule(p, a1, a2, 0.006), 96.0);
                let cross_r = doc_line_glow(sd_capsule(p, a0, vec3<f32>(0.0, 0.22, 0.0), 0.005), 96.0);
                col += vec3<f32>(0.32, 1.35, 1.52) * g_nodes + vec3<f32>(1.48, 0.95, 0.30) * (ray1 + ray2 + cross_r);
                dens += g_nodes * 0.9 + (ray1 + ray2 + cross_r) * 0.85;
            }
        } else if (tech == 6u) {
            let th = p.y * 8.0 + time * 0.5;
            let r_h = 0.19;
            let g1 = exp(-pow(length(p.xz - vec2<f32>(cos(th) * r_h, sin(th) * r_h)) * 34.0, 2.0));
            let g2 = exp(-pow(length(p.xz + vec2<f32>(cos(th) * r_h, sin(th) * r_h)) * 34.0, 2.0));
            let gate = doc_line_glow(sd_torus(p, 0.28, 0.012), 58.0);
            col += vec3<f32>(0.24, 1.32, 0.85) * (g1 + g2) * smoothstep(0.56, 0.42, abs(p.y)) + vec3<f32>(1.48, 0.92, 0.24) * gate;
            dens += (g1 + g2) * 0.8 + gate;
        } else {
            let q = fract((p + vec3<f32>(0.36)) * 2.8) - 0.5;
            let node = exp(-pow(length(q) * 14.0, 2.0)) * smoothstep(0.55, 0.38, max(max(abs(p.x), abs(p.y)), abs(p.z)));
            let beam = doc_line_glow(sd_capsule(p, vec3<f32>(-0.34, -0.34, 0.0), vec3<f32>(0.34, 0.34, 0.0), 0.008), 64.0)
                     + doc_line_glow(sd_capsule(p, vec3<f32>(-0.34, 0.34, 0.0), vec3<f32>(0.34, -0.34, 0.0), 0.008), 64.0);
            col += vec3<f32>(0.38, 1.15, 1.48) * node + vec3<f32>(1.45, 0.95, 0.28) * beam * (0.8 + 0.5 * cue);
            dens += node * 0.85 + beam * 0.8;
        }
    } else {
        let fut = select(u32(max(scene, 0)), sub_v, sub_v < 10u);
        if (fut == 0u) {
            // Crisp Multicolor Bifurcating Future Possibility Arcs radiating from Present Core
            let origin_p = vec3<f32>(-0.50, -0.10, 0.0);
            let core_g = exp(-pow(length(p - origin_p) * 18.0, 2.0));
            col += vec3<f32>(1.50, 1.25, 0.72) * core_g;
            dens += core_g * 1.1;
            if (p.x >= -0.52 && p.x <= 0.62) {
                let t_x = clamp((p.x + 0.50) / 1.10, 0.0, 1.0);
                for (var b = 0; b < 5; b = b + 1) {
                    let fb = f32(b) - 2.0;
                    let arc_y = -0.10 + fb * 0.24 * pow(t_x, 0.78) + 0.05 * sin(t_x * 6.0 - time * 1.5 + fb);
                    let arc_z = sin(fb * 1.3 + t_x * 2.2) * 0.14 * t_x;
                    let d_arc = length(vec2<f32>(p.y - arc_y, p.z - arc_z));
                    let ag = exp(-pow(d_arc * 68.0, 2.0)) * smoothstep(0.0, 0.06, t_x) * smoothstep(1.02, 0.92, t_x);
                    let tip = exp(-pow(length(p - vec3<f32>(0.58, -0.10 + fb * 0.24 + 0.05 * sin(6.0 - time * 1.5 + fb), sin(fb * 1.3 + 2.2) * 0.14)) * 24.0, 2.0));
                    let branch_c = hsl_to_rgb(fract(0.04 + f32(b) * 0.14), 0.92, 0.56);
                    col += branch_c * (ag * 1.25 + tip);
                    dens += ag * 0.92 + tip * 0.95;
                }
            }
        } else if (fut == 2u) {
            // Kardashev Future: Rotating Tilted O'Neill Habitat Cylinder + Concentric Dyson Mirror Swarm
            let po = doc_rot_x(p - vec3<f32>(-0.24, -0.04, 0.0), 0.34);
            let r_cyl = length(po.xy);
            let d_cyl = abs(r_cyl - 0.22);
            let in_len = smoothstep(0.42, 0.36, abs(po.z));
            let bay = exp(-pow(abs(fract(po.z * 7.0) - 0.5) * 18.0, 2.0));
            let strip = smoothstep(0.15, 0.45, sin(atan2(po.y, po.x) * 3.0 + time * 0.9));
            let cyl_g = exp(-pow(d_cyl * 72.0, 2.0)) * in_len;
            let end_caps = exp(-pow(abs(abs(po.z) - 0.36) * 65.0, 2.0)) * smoothstep(0.24, 0.18, r_cyl);
            col += mix(vec3<f32>(0.24, 1.32, 0.78), vec3<f32>(0.85, 1.15, 1.45), max(bay, 0.3)) * cyl_g * (0.45 + 0.75 * strip)
                 + vec3<f32>(1.42, 1.02, 0.38) * end_caps;
            dens += cyl_g * 0.95 + end_caps * 0.85;

            // Orbiting Dyson Collector Mirror Rings beside/around Star Core (+0.30, +0.06, 0)
            let p_dy = p - vec3<f32>(0.30, 0.06, 0.0);
            let star_c = exp(-pow(length(p_dy) * 26.0, 2.0));
            col += vec3<f32>(1.52, 1.22, 0.52) * star_c;
            dens += star_c;
            for (var b = 0; b < 3; b = b + 1) {
                let fb = f32(b);
                let pb = doc_rot_x(doc_rot_z(p_dy, 0.52 + fb * 0.72), 0.48 + fb * 0.96);
                let ring_d = abs(length(pb.xz) - (0.24 + fb * 0.075));
                let ang = atan2(pb.z, pb.x) + time * (0.45 + fb * 0.15);
                let mirror_seg = step(0.15, cos(ang * 8.0 + f32(feat) * 0.4));
                let dyson_g = exp(-pow(ring_d * 78.0, 2.0)) * exp(-pow(pb.y * 46.0, 2.0)) * (0.25 + 0.95 * mirror_seg);
                col += vec3<f32>(0.36, 1.20, 1.52) * dyson_g * 1.15;
                dens += dyson_g * 0.9;
            }
        } else {
            let star_g = exp(-pow(length(p) * 18.0, 2.0));
            col += vec3<f32>(1.50, 1.18, 0.55) * star_g;
            dens += star_g;
            for (var b = 0; b < 3; b = b + 1) {
                let fb = f32(b);
                let pb = doc_rot_x(doc_rot_z(p, fb * 0.68), fb * 1.04);
                let ring_d = abs(length(pb.xz) - (0.36 + fb * 0.08));
                let ang = atan2(pb.z, pb.x) + time * (0.35 + fb * 0.12);
                let mirror_seg = pow(max(cos(ang * 9.0 + f32(feat) * 0.4), 0.0), 6.0);
                let dyson_g = exp(-pow(ring_d * 68.0, 2.0)) * exp(-pow(pb.y * 42.0, 2.0)) * mirror_seg;
                col += vec3<f32>(0.38, 1.15, 1.48) * dyson_g * 1.2;
                dens += dyson_g * 0.9;
            }
        }
    }
    return vec4<f32>(col, dens);
}

// ============================================================================
// EXP 21: 21_cosmic_life — 25 CANONICAL [TIER-A] STAGES (vid = 3100..=3124)
// Shared helpers + full 25-zone x 8-feature (200 features) procedural choreography
// ============================================================================
fn cl_spot(p: vec3<f32>, c: vec3<f32>, sharp: f32) -> f32 {
    return exp(-pow(length(p - c) * sharp, 2.0));
}

fn cl_ring_xz(p: vec3<f32>, r_maj: f32, r_min: f32, sharp: f32) -> f32 {
    return doc_line_glow(sd_torus(p, r_maj, r_min), sharp);
}

fn cl_ring_xy(p: vec3<f32>, r_maj: f32, r_min: f32, sharp: f32) -> f32 {
    return doc_line_glow(sd_torus(doc_rot_x(p, 1.570796), r_maj, r_min), sharp);
}

fn cl_bar(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, r_w: f32, sharp: f32) -> f32 {
    return doc_line_glow(sd_capsule(p, a, b, r_w), sharp);
}

fn cl_feature_beacon(p: vec3<f32>, feat: u32, rad: f32, elev: f32, time: f32) -> f32 {
    if (feat > 7u) {
        return 0.0;
    }
    let a = f32(feat) * TAU / 8.0 + sin(time * 0.6) * 0.08;
    let c = vec3<f32>(cos(a) * rad, elev + 0.06 * sin(f32(feat) * 1.9 + time * 1.1), sin(a) * rad);
    let node = cl_spot(p, c, 15.0);
    let ring = cl_ring_xy(p - c, 0.075, 0.005, 86.0);
    let spoke = cl_bar(p, vec3<f32>(0.0, elev * 0.4, 0.0), c * 0.88, 0.004, 90.0);
    return node * 0.95 + ring * 0.72 + spoke * 0.42;
}

fn shade_doc_cosmic_life_stage(
    p: vec3<f32>,
    z_idx: u32,
    feat: u32,
    inst: HeroVolumeInstance,
    zone_col: vec3<f32>,
    time: f32,
    voice: f32,
    bass: f32,
) -> vec4<f32> {
    let r = length(p);
    if (r > 0.72) {
        return vec4<f32>(0.0);
    }
    let env = smoothstep(0.72, 0.50, r);
    let ff = f32(min(feat, 7u));
    let feat_hi = select(0.55, 1.0, feat <= 7u);
    let fg = doc_sample_field_grid(clamp(p.xy * 0.75 + 0.5, vec2<f32>(0.02), vec2<f32>(0.98))).rg;
    let grid_mod = 0.78 + 0.35 * fg.x;
    let cue_p = 0.80 + 0.25 * doc_cue(inst);
    var col = vec3<f32>(0.0);
    var dens: f32 = 0.0;

    if (z_idx == 0u) {
        // Z00 unknown-horizon: Astrobiology Horizon Aperture & Scan Cones
        let tilt = doc_rot_x(p, 0.38);
        let aperture = cl_ring_xz(tilt, 0.48, 0.009, 86.0) + cl_ring_xy(p, 0.36, 0.006, 86.0) * 0.65;
        let scan_ang = ff * 0.785398 + time * 0.25;
        let scan_dir = vec3<f32>(cos(scan_ang) * 0.52, sin(scan_ang * 0.7) * 0.22, sin(scan_ang) * 0.52);
        let cone = cl_bar(p, vec3<f32>(0.0), scan_dir, 0.022, 42.0);
        let noise_wave = exp(-abs(p.y - 0.12 * sin(p.x * 16.0 + ff * 1.4 + time * 2.0)) * 34.0) * smoothstep(0.52, 0.10, length(p.xz));
        let stars = pow(max(0.0, snoise3(p * 11.0 + vec3<f32>(ff * 0.7))), 5.0) * smoothstep(0.62, 0.16, r);
        col = vec3<f32>(0.24, 0.86, 1.32) * (aperture + cone * 0.85) * grid_mod
            + mix(vec3<f32>(0.42, 0.75, 1.38), zone_col, 0.35) * (noise_wave * select(0.25, 1.15, feat == 5u) + stars * 1.15)
            + hsl_to_rgb(fract(0.52 + ff * 0.09), 0.88, 0.54) * cl_feature_beacon(p, feat, 0.36, 0.04, time) * 0.78;
        dens = (aperture * 0.8 + cone * 0.7 + noise_wave * 0.5 + stars * 0.55) * cue_p;
    } else if (z_idx == 1u) {
        // Z01 cosmic-ingredients: CHNOPS Nested Nuclear Alchemy & Bok Globule
        let shell_r = 0.14 + ff * 0.055;
        let chnops_shell = exp(-abs(r - shell_r) * 44.0) * doc_feat_focus(inst, min(feat, 7u));
        let c_hex = doc_line_glow(sd_hex_prism(doc_rot_y(p, ff * 0.45), 0.26 + 0.03 * sin(ff), 0.012), 78.0);
        let bok = smoothstep(0.08, 0.45, fbm3(p * 4.2 + vec3<f32>(ff * 0.9), 3)) * smoothstep(0.66, 0.18, r);
        let el_col = hsl_to_rgb(fract(0.06 + ff * 0.12), 0.92, 0.54);
        col = el_col * chnops_shell * 1.05
            + vec3<f32>(0.34, 1.05, 1.38) * c_hex * select(0.42, 1.10, feat == 1u || feat == 7u)
            + vec3<f32>(0.95, 0.58, 0.22) * bok * select(0.45, 0.95, feat == 6u || feat == 7u);
        dens = chnops_shell * 0.88 + c_hex * 0.65 + bok * 0.52;
    } else if (z_idx == 2u) {
        // Z02 stellar-stability: Host Star, CME Coronal Loop & Binary Companion
        let is_binary = (feat == 5u);
        let star_a = cl_spot(p, vec3<f32>(select(0.0, -0.18, is_binary), 0.0, 0.0), select(13.0, 17.0, is_binary));
        let star_b = cl_spot(p, vec3<f32>(0.20, 0.04, 0.0), 18.0) * select(0.0, 1.0, is_binary || feat == 0u);
        let flux_shell = exp(-abs(r - (0.28 + ff * 0.035)) * 46.0);
        let loop_p = doc_rot_z(p - vec3<f32>(0.16, 0.10, 0.0), ff * 0.55);
        let cme_arc = cl_ring_xy(loop_p, 0.16, 0.012, 68.0) * select(0.35, 1.25, feat == 2u || feat == 3u || feat == 4u);
        let star_c = select(vec3<f32>(1.28, 0.72, 0.26), vec3<f32>(0.42, 0.88, 1.48), feat == 0u || feat == 2u);
        col = star_c * (star_a * 1.2 + flux_shell * 0.65)
            + vec3<f32>(1.35, 0.36, 0.14) * (star_b + cme_arc)
            + vec3<f32>(0.26, 0.98, 1.32) * cl_feature_beacon(p, feat, 0.42, 0.0, time) * 0.68;
        dens = star_a * 0.9 + star_b * 0.75 + flux_shell * 0.6 + cme_arc * 0.72;
    } else if (z_idx == 3u) {
        // Z03 planetary-systems: Keplerian Protoplanetary Disk & Migration Tracks
        let pd = doc_rot_x(p, 0.42 + ff * 0.04);
        let r_xz = length(pd.xz);
        let gap = 1.0 - 0.82 * exp(-abs(r_xz - (0.24 + ff * 0.03)) * 36.0);
        let disk = exp(-abs(pd.y) * 28.0) * smoothstep(0.60, 0.14, r_xz) * smoothstep(0.07, 0.15, r_xz) * gap;
        let spiral = cl_ring_xz(pd, 0.22 + ff * 0.038, 0.010, 72.0);
        let core_star = cl_spot(pd, vec3<f32>(0.0), 15.0);
        let comet_tail = cl_bar(pd, vec3<f32>(0.36, 0.04, 0.12), vec3<f32>(0.08, 0.0, -0.06), 0.012, 60.0) * select(0.25, 1.15, feat == 6u || feat == 7u);
        col = vec3<f32>(1.28, 0.88, 0.34) * (core_star * 1.15 + disk * 0.72)
            + vec3<f32>(0.34, 1.08, 1.42) * (spiral * 0.82 + comet_tail)
            + vec3<f32>(0.92, 0.46, 0.22) * cl_feature_beacon(pd, feat, 0.26 + ff * 0.03, 0.02, time) * 0.85;
        dens = core_star * 0.85 + disk * 0.70 + spiral * 0.65 + comet_tail * 0.6;
    } else if (z_idx == 4u) {
        // Z04 habitable-zone: Circumstellar HZ Belt, Eccentricity & Eyeball Worlds
        let ph = doc_rot_x(p, 0.34);
        let star = cl_spot(ph, vec3<f32>(0.0), 15.0);
        let hz_shift = select(0.0, 0.08, feat == 6u);
        let inner_hz = cl_ring_xz(ph, 0.30 + hz_shift, 0.008, 84.0);
        let outer_hz = cl_ring_xz(ph, 0.50 + hz_shift, 0.008, 84.0);
        let belt = smoothstep(0.11, 0.02, abs(length(ph.xz) - (0.40 + hz_shift))) * exp(-abs(ph.y) * 26.0);
        let ecc_x = cos(time * 0.7 + ff * 0.85) * select(0.40, 0.50, feat == 4u);
        let ecc_z = sin(time * 0.7 + ff * 0.85) * select(0.40, 0.26, feat == 4u);
        let exo_pos = vec3<f32>(ecc_x + select(0.0, -0.10, feat == 2u) + select(0.0, 0.10, feat == 3u), 0.0, ecc_z);
        let exo_orb = cl_spot(ph, exo_pos, 18.0);
        let exo_c = select(vec3<f32>(0.24, 1.16, 1.38), vec3<f32>(1.38, 0.42, 0.16), feat == 2u);
        col = vec3<f32>(1.35, 0.96, 0.32) * star
            + vec3<f32>(0.18, 1.24, 0.66) * (inner_hz + outer_hz + belt * 0.38)
            + exo_c * exo_orb
            + vec3<f32>(0.55, 1.05, 1.42) * cl_feature_beacon(ph, feat, 0.38, 0.06, time) * 0.52;
        dens = star * 0.88 + (inner_hz + outer_hz) * 0.78 + belt * 0.34 + exo_orb * 0.86;
    } else if (z_idx == 5u) {
        // Z05 real-habitability: Cutaway Globe + Mantle Subduction + Dipole Magnetosphere
        let shell = exp(-abs(r - 0.32) * 52.0);
        let atmo = exp(-abs(r - 0.39) * 38.0) * select(0.45, 1.15, feat <= 2u);
        let bow = cl_ring_xy(doc_rot_y(p, ff * 0.35), 0.46, 0.010, 74.0) * smoothstep(-0.35, 0.25, p.x);
        let tilt_axis = cl_bar(p, vec3<f32>(-0.14, -0.46, 0.0), vec3<f32>(0.14, 0.46, 0.0), 0.008, 86.0);
        let core = cl_spot(p, vec3<f32>(0.0), 14.0);
        col = vec3<f32>(1.28, 0.48, 0.16) * (core * 0.85 + tilt_axis * 0.65)
            + vec3<f32>(0.18, 0.92, 1.35) * (shell * 0.82 + atmo * 0.65 + bow * 0.95)
            + hsl_to_rgb(fract(0.32 + ff * 0.11), 0.90, 0.54) * cl_feature_beacon(p, feat, 0.44, 0.0, time) * 0.78;
        dens = core * 0.65 + shell * 0.82 + atmo * 0.55 + bow * 0.75 + tilt_axis * 0.55;
    } else if (z_idx == 6u) {
        // Z06 prebiotic-chemistry: PAH Hexagons, Ribose/Nucleobases & Hydrothermal Chimney
        let pah = doc_line_glow(sd_hex_prism(doc_rot_y(p - vec3<f32>(0.0, 0.10, 0.0), ff * 0.5), 0.24, 0.015), 76.0);
        let chimney = cl_bar(p, vec3<f32>(0.0, -0.46, 0.0), vec3<f32>(0.0, 0.08, 0.0), 0.045, 42.0);
        let plume = smoothstep(0.0, 0.45, snoise3(p * 6.5 - vec3<f32>(0.0, time * 0.6, 0.0))) * exp(-length(p.xz) * 6.5) * smoothstep(-0.1, 0.35, p.y);
        let beads = exp(-abs(length(p - vec3<f32>(cos(ff * 0.8) * 0.24, 0.14, sin(ff * 0.8) * 0.24)) - 0.12) * 48.0);
        col = vec3<f32>(0.22, 1.24, 0.92) * (pah * 0.95 + plume * 0.78)
            + vec3<f32>(1.28, 0.62, 0.18) * (chimney * 0.75 + beads * 0.95)
            + vec3<f32>(0.42, 0.88, 1.42) * cl_feature_beacon(p, feat, 0.34, 0.12, time) * 0.76;
        dens = pah * 0.82 + chimney * 0.65 + plume * 0.60 + beads * 0.78;
    } else if (z_idx == 7u) {
        // Z07 origin-of-life: Unzipping RNA/DNA Strand, Ribozymes & Binary-Fission Protocells
        let sep = 0.12 + 0.04 * f32(feat == 3u);
        let ves_a = exp(-abs(length(p - vec3<f32>(-sep, 0.0, 0.0)) - 0.22) * 52.0);
        let ves_b = exp(-abs(length(p - vec3<f32>(sep, 0.0, 0.0)) - 0.22) * 52.0);
        let helix_x = 0.08 * sin(p.y * 18.0 + ff * 0.9 + time);
        let strand = cl_bar(vec3<f32>(p.x - helix_x, p.y, p.z), vec3<f32>(0.0, -0.36, 0.0), vec3<f32>(0.0, 0.36, 0.0), 0.012, 68.0);
        let ring_cycle = cl_ring_xy(p, 0.36, 0.008, 82.0) * select(0.35, 1.15, feat == 1u || feat == 7u);
        col = vec3<f32>(0.24, 1.28, 1.05) * (ves_a + ves_b) * 0.85
            + vec3<f32>(1.34, 0.82, 0.24) * (strand + ring_cycle * 0.7)
            + hsl_to_rgb(fract(0.45 + ff * 0.11), 0.90, 0.56) * cl_feature_beacon(p, feat, 0.38, 0.0, time) * 0.72;
        dens = (ves_a + ves_b) * 0.68 + strand * 0.82 + ring_cycle * 0.55;
    } else if (z_idx == 8u) {
        // Z08 simple-biospheres: Layered Stromatolite Dome & Cyanobacteria O2 Bubbles
        let dome_d = max(length(vec3<f32>(p.x, p.y + 0.14, p.z)) - 0.34, -p.y - 0.18);
        let dome = doc_line_glow(dome_d, 46.0);
        let strata = 0.5 + 0.5 * sin((p.y + 0.22) * 44.0 + ff * 0.8);
        let bubbles = pow(max(0.0, sin(p.x * 24.0 + ff) * cos(p.y * 20.0 - time * 1.4) * sin(p.z * 24.0)), 4.0) * smoothstep(-0.1, 0.45, p.y);
        let dome_c = mix(vec3<f32>(0.16, 0.92, 0.64), vec3<f32>(0.78, 0.34, 1.24), f32(feat == 2u));
        col = dome_c * dome * (0.45 + 0.75 * strata)
            + vec3<f32>(0.28, 1.28, 1.48) * bubbles * 1.25
            + vec3<f32>(1.18, 0.84, 0.32) * cl_feature_beacon(p, feat, 0.38, 0.08, time) * 0.82;
        dens = dome * 0.86 + bubbles * 0.75;
    } else if (z_idx == 9u) {
        // Z09 complex-life: Eukaryotic Nucleus, Symbiotic Mitochondria & Branching Synapses
        let cell_wall = exp(-abs(r - 0.42) * 52.0);
        let nucleus = cl_spot(p, vec3<f32>(0.0, 0.02, 0.0), 11.0);
        let mito_p = vec3<f32>(0.18 * cos(ff * 0.9), 0.08 * sin(ff * 1.3), 0.16 * sin(ff * 0.9));
        let mito = cl_bar(p, mito_p - vec3<f32>(0.06, 0.0, 0.0), mito_p + vec3<f32>(0.06, 0.0, 0.0), 0.024, 54.0);
        let volvox = cl_ring_xy(p, 0.30, 0.008, 80.0) * (0.5 + 0.5 * sin(atan2(p.y, p.x) * 8.0 + ff));
        col = vec3<f32>(0.22, 1.15, 1.38) * (cell_wall * 0.82 + volvox * 0.72)
            + vec3<f32>(1.28, 0.64, 0.24) * (nucleus * 0.75 + mito * 1.25)
            + hsl_to_rgb(fract(0.55 + ff * 0.10), 0.92, 0.56) * cl_feature_beacon(p, feat, 0.36, 0.0, time) * 0.82;
        dens = cell_wall * 0.78 + nucleus * 0.68 + mito * 0.85 + volvox * 0.60;
    } else if (z_idx == 10u) {
        // Z10 extremophiles: 8-Niche Extremophile Habitat Crystal Chamber
        let niche_c = hsl_to_rgb(fract(ff * 0.125 + 0.04), 0.94, 0.54);
        let cage = doc_line_glow(abs(sd_octahedron(doc_rot_y(p, ff * 0.39), 0.42)) - 0.01, 74.0);
        let core_p = doc_rot_x(p, ff * 0.52);
        let core_grid = exp(-abs(snoise3(core_p * (5.5 + ff))) * 16.0) * smoothstep(0.42, 0.10, r);
        let ray_cherenkov = cl_bar(p, vec3<f32>(-0.32, -0.25, 0.0), vec3<f32>(0.32, 0.25, 0.0), 0.010, 68.0) * select(0.3, 1.25, feat == 4u || feat == 0u);
        col = niche_c * (cage * 1.1 + core_grid * 0.92)
            + vec3<f32>(0.26, 0.98, 1.48) * ray_cherenkov
            + vec3<f32>(1.18, 0.88, 0.36) * cl_feature_beacon(p, feat, 0.38, 0.0, time) * 0.76;
        dens = cage * 0.85 + core_grid * 0.72 + ray_cherenkov * 0.62;
    } else if (z_idx == 11u) {
        // Z11 mars: Dusty Rust-Red Paleoclimate Globe, Paleorivers & Aquifer Cutaway
        let globe = exp(-abs(r - 0.38) * 56.0);
        let p_spin = doc_rot_y(p, ff * 0.48 + time * 0.08);
        let canyon = exp(-abs(p_spin.y + 0.06 * sin(p_spin.x * 9.0)) * 28.0) * step(0.0, p_spin.z);
        let polar_cap = smoothstep(0.28, 0.37, abs(p_spin.y)) * globe;
        let wind_tail = exp(-abs(p.y) * 18.0) * smoothstep(0.25, 0.58, p.x) * select(0.28, 1.15, feat == 2u || feat == 5u);
        let aquifer = cl_ring_xz(p - vec3<f32>(0.0, -0.06, 0.0), 0.30, 0.010, 74.0) * select(0.25, 1.2, feat == 6u || feat == 0u);
        col = vec3<f32>(1.26, 0.46, 0.18) * globe * (0.65 + 0.55 * canyon)
            + vec3<f32>(0.88, 0.96, 1.08) * polar_cap * 0.88
            + vec3<f32>(0.26, 0.96, 1.44) * (wind_tail + aquifer)
            + vec3<f32>(1.28, 0.92, 0.32) * cl_feature_beacon(p, feat, 0.44, 0.02, time) * 0.74;
        dens = globe * 0.88 + polar_cap * 0.55 + wind_tail * 0.55 + aquifer * 0.68;
    } else if (z_idx == 12u) {
        // Z12 ocean-worlds: Cracked Ice Crust, Lineae, Cutaway Ocean & Hydrothermal Plumes
        let stretch = select(1.0, 1.14, feat == 5u);
        let pe = vec3<f32>(p.x / stretch, p.y - (-0.08), p.z);
        let re = length(pe);
        let cutout = (feat == 4u) && (pe.x > 0.0 && pe.z > 0.0);
        let ice_shell = exp(-abs(re - 0.38) * 70.0) * select(1.0, 0.18, cutout);
        let ocean_layer = exp(-abs(re - 0.27) * 52.0) * select(0.35, 1.25, feat == 4u || feat == 6u);
        let crack_val = sin(pe.x * 18.0 + pe.y * 12.0 + ff * 0.7) + cos(pe.z * 16.0 - pe.x * 9.0);
        let lineae = exp(-pow(abs(crack_val) * 9.5, 2.0)) * ice_shell;
        let plume_a = exp(-pow(length(pe.xz) * 22.0, 2.0)) * smoothstep(0.34, 0.41, pe.y) * smoothstep(0.82, 0.45, pe.y);
        let plume_b = exp(-pow(length(pe.xz - vec2<f32>(0.15, 0.07)) * 28.0, 2.0)) * smoothstep(0.31, 0.39, pe.y) * smoothstep(0.68, 0.43, pe.y);
        let crust_tint = select(vec3<f32>(0.76, 0.94, 1.22), vec3<f32>(1.24, 0.68, 0.24), feat == 2u);
        let dipole = cl_ring_xy(pe, 0.46, 0.008, 78.0) * select(0.18, 1.15, feat == 3u || feat == 1u);
        col = crust_tint * ice_shell * 0.68
            + vec3<f32>(0.22, 1.28, 1.48) * (lineae * 1.18 + (plume_a + plume_b) * 1.08 + ocean_layer * 0.85 + dipole)
            + vec3<f32>(1.22, 0.56, 0.24) * cl_feature_beacon(p, feat, 0.44, -0.04, time) * 0.68;
        dens = ice_shell * 0.66 + lineae * 0.85 + (plume_a + plume_b) * 0.90 + ocean_layer * 0.70 + dipole * 0.55;
    } else if (z_idx == 13u) {
        // Z13 exoplanets: Edge-On Stellar Transit + Photometric Dip Curve + Coronagraph Mask
        let star = cl_spot(p - vec3<f32>(0.0, 0.10, 0.0), vec3<f32>(0.0), 10.5);
        let planet_x = mix(-0.28, 0.28, ff / 7.0);
        let planet = cl_spot(p, vec3<f32>(planet_x, 0.10, 0.16), 24.0);
        let orbit_ring = cl_ring_xz(p - vec3<f32>(0.0, 0.10, 0.0), 0.36, 0.006, 86.0);
        let dip_y = -0.30 - 0.12 * exp(-pow((p.x - planet_x) * 8.0, 2.0));
        let dip_curve = exp(-abs(p.y - dip_y) * 44.0) * smoothstep(0.56, 0.10, abs(p.x)) * exp(-abs(p.z) * 12.0);
        col = vec3<f32>(1.34, 0.94, 0.32) * star * (1.0 - 0.82 * planet)
            + vec3<f32>(0.24, 1.14, 1.48) * (orbit_ring * 0.85 + dip_curve * 1.2)
            + vec3<f32>(0.12, 0.72, 1.25) * cl_feature_beacon(p, feat, 0.40, -0.10, time) * 0.75;
        dens = star * 0.82 + orbit_ring * 0.65 + dip_curve * 0.85 + planet * 0.7;
    } else if (z_idx == 14u) {
        // Z14 biosignatures: Exoplanet Limb Atmosphere + O2/CH4 Absorption Barcode Bands
        let globe = exp(-abs(r - 0.28) * 58.0);
        let atmo_limb = exp(-abs(r - 0.34) * 44.0);
        let spec_band_x = -0.44 + ff * 0.125;
        let barcode = exp(-abs(p.x - spec_band_x) * 42.0) * smoothstep(0.02, -0.05, abs(p.y + 0.32)) * smoothstep(0.52, 0.05, length(p));
        let eq_loop = cl_ring_xy(doc_rot_y(p, ff * 0.4), 0.42, 0.007, 82.0);
        col = vec3<f32>(0.18, 1.05, 0.64) * globe * 0.88
            + vec3<f32>(0.24, 1.18, 1.46) * (atmo_limb * 0.92 + eq_loop * 0.68)
            + hsl_to_rgb(fract(0.30 + ff * 0.10), 0.94, 0.56) * (barcode * 1.35 + cl_feature_beacon(p, feat, 0.42, 0.0, time) * 0.78);
        dens = globe * 0.80 + atmo_limb * 0.72 + barcode * 0.85 + eq_loop * 0.58;
    } else if (z_idx == 15u) {
        // Z15 false-positives: Abiotic Volcanic / UV Photolysis Mimic + Split Comparison Ring
        let rock_globe = exp(-abs(r - 0.26) * 58.0);
        let lava_cracks = pow(max(0.0, snoise3(p * 10.0 + vec3<f32>(ff * 0.8))), 3.0) * rock_globe;
        let uv_beam = cl_bar(p, vec3<f32>(-0.48, 0.28, 0.0), vec3<f32>(-0.08, 0.06, 0.0), 0.016, 56.0);
        let false_wave = exp(-abs(p.y + 0.28 - 0.08 * sin(p.x * 22.0 + ff * 1.5)) * 42.0) * smoothstep(0.52, 0.10, abs(p.x));
        col = vec3<f32>(1.32, 0.44, 0.16) * (rock_globe * 0.75 + lava_cracks * 1.25 + false_wave * 0.95)
            + vec3<f32>(0.68, 0.42, 1.42) * uv_beam * 0.95
            + vec3<f32>(1.28, 0.88, 0.26) * cl_feature_beacon(p, feat, 0.42, -0.05, time) * 0.82;
        dens = rock_globe * 0.76 + lava_cracks * 0.72 + uv_beam * 0.70 + false_wave * 0.75;
    } else if (z_idx == 16u) {
        // Z16 evolutionary-intelligence: Non-Linear Radial Phylogenetic Cognition Bush
        var branch_g: f32 = 0.0;
        for (var b: u32 = 0u; b < 8u; b = b + 1u) {
            let ba = f32(b) * TAU / 8.0;
            let tip = vec3<f32>(cos(ba) * 0.44, sin(ba) * 0.36, 0.14 * sin(ba * 2.0 + ff * 0.5));
            let w_b = select(0.45, 1.25, b == (feat & 7u));
            branch_g += cl_bar(p, vec3<f32>(0.0), tip, 0.008, 80.0) * w_b;
            branch_g += cl_spot(p, tip, 18.0) * w_b * 0.85;
        }
        let stem = cl_spot(p, vec3<f32>(0.0), 15.0);
        col = vec3<f32>(0.26, 1.18, 1.38) * (branch_g * 0.78 + stem * 0.9)
            + hsl_to_rgb(fract(0.10 + ff * 0.11), 0.92, 0.56) * cl_feature_beacon(p, feat, 0.44, 0.0, time) * 0.85;
        dens = min(branch_g * 0.58 + stem * 0.78, 1.35);
    } else if (z_idx == 17u) {
        // Z17 alien-intelligence-forms: Xenocognitive Polyhedral & Swarm Morphologies
        let rot_p = doc_rot_y(doc_rot_x(p, ff * 0.38), time * 0.22 + ff * 0.42);
        let octa = exp(-abs(sd_octahedron(rot_p, 0.32 + 0.03 * sin(ff))) * 64.0);
        let swarm = pow(max(0.0, sin(rot_p.x * 22.0) * cos(rot_p.y * 22.0) * sin(rot_p.z * 22.0 + time)), 4.0) * smoothstep(0.48, 0.12, r);
        let halo = cl_ring_xy(doc_rot_y(p, ff * 0.6), 0.44, 0.007, 84.0);
        let m_col = hsl_to_rgb(fract(0.48 + ff * 0.115), 0.92, 0.54);
        col = m_col * (octa * 1.05 + swarm * 1.15) + vec3<f32>(0.34, 1.16, 1.44) * halo * 0.75;
        dens = octa * 0.82 + swarm * 0.75 + halo * 0.62;
    } else if (z_idx == 18u) {
        // Z18 technosignatures: Narrowband Waterfall, Night-Side City Lights & Dyson Ring
        let dyson = cl_ring_xz(doc_rot_x(p, 0.36 + ff * 0.06), 0.42, 0.009, 82.0);
        let night_orb = exp(-abs(r - 0.22) * 58.0);
        let city_dots = pow(max(0.0, snoise3(p * 18.0 + vec3<f32>(ff * 1.1))), 4.0) * night_orb;
        let spike_x = -0.36 + ff * 0.10;
        let laser_beam = cl_bar(p, vec3<f32>(spike_x, -0.42, 0.0), vec3<f32>(spike_x, 0.46, 0.0), 0.009, 86.0);
        col = vec3<f32>(0.24, 1.24, 1.42) * dyson
            + vec3<f32>(1.36, 0.92, 0.26) * (city_dots * 1.28 + night_orb * 0.35)
            + vec3<f32>(0.22, 1.38, 0.72) * laser_beam * 1.1
            + vec3<f32>(1.18, 0.56, 0.22) * cl_feature_beacon(p, feat, 0.42, 0.0, time) * 0.65;
        dens = dyson * 0.80 + night_orb * 0.62 + city_dots * 0.65 + laser_beam * 0.84;
    } else if (z_idx == 19u) {
        // Z19 fermi-paradox: Milky Way Spiral Disk & Expanding Colonization Wavefront
        let pm = doc_rot_x(p, 0.48);
        let r_xz = length(pm.xz);
        let ang = atan2(pm.z, pm.x);
        let arms = pow(max(0.0, cos(ang * 2.0 - r_xz * 12.0 + ff * 0.45)), 3.0);
        let spiral_disk = exp(-abs(pm.y) * 24.0) * smoothstep(0.56, 0.08, r_xz) * (0.28 + 0.72 * arms);
        let wave_r = 0.16 + ff * 0.048;
        let wave_ring = cl_ring_xz(pm, wave_r, 0.008, 76.0);
        let silence_mask = select(1.0, 0.28, feat == 3u || feat == 6u);
        col = vec3<f32>(0.36, 0.92, 1.38) * spiral_disk * silence_mask
            + vec3<f32>(1.32, 0.86, 0.32) * (wave_ring * 1.05 + cl_spot(pm, vec3<f32>(0.22, 0.02, 0.0), 20.0) * 0.9);
        dens = spiral_disk * 0.80 * silence_mask + wave_ring * 0.82;
    } else if (z_idx == 20u) {
        // Z20 great-filter: 8-Gate Horizontal Probability Funnel
        var gate_acc: f32 = 0.0;
        var gate_hi: f32 = 0.0;
        for (var g: u32 = 0u; g < 8u; g = g + 1u) {
            let gx = -0.44 + f32(g) * (0.88 / 7.0);
            let gr = 0.30 - f32(g) * 0.026;
            let ring_g = doc_line_glow(sd_torus(doc_rot_z(p - vec3<f32>(gx, 0.0, 0.0), 1.570796), gr, 0.006), 86.0);
            if (g == (feat & 7u)) {
                gate_hi += ring_g * 1.5;
            } else {
                gate_acc += ring_g * 0.55;
            }
        }
        let stream = exp(-length(p.yz) * (10.0 + p.x * 8.0)) * smoothstep(0.50, 0.02, abs(p.x));
        col = vec3<f32>(0.24, 1.05, 1.38) * (gate_acc + stream * 0.65)
            + vec3<f32>(1.38, 0.72, 0.24) * gate_hi;
        dens = min(gate_acc * 0.6 + gate_hi * 0.92 + stream * 0.52, 1.35);
    } else if (z_idx == 21u) {
        // Z21 alien-civilizations: Archetypes (O'Neill Rings, Worldship, Tree-City, Matryoshka)
        let tilted = doc_rot_y(doc_rot_x(p, 0.42), ff * 0.45);
        let belt_a = cl_ring_xz(tilted, 0.34, 0.012, 74.0);
        let belt_b = cl_ring_xy(tilted, 0.44, 0.008, 82.0);
        let ship = cl_bar(tilted, vec3<f32>(-0.22, 0.0, 0.0), vec3<f32>(0.22, 0.0, 0.0), 0.028, 48.0);
        let arch_c = hsl_to_rgb(fract(0.24 + ff * 0.115), 0.92, 0.54);
        col = arch_c * (belt_a * 0.95 + ship * 1.05)
            + vec3<f32>(0.34, 1.18, 1.46) * (belt_b * 0.85 + cl_feature_beacon(p, feat, 0.42, 0.02, time) * 0.78);
        dens = belt_a * 0.78 + belt_b * 0.72 + ship * 0.76;
    } else if (z_idx == 22u) {
        // Z22 interstellar-communication: Two-Star Light-Cone & Prime Relay Bridge
        let star_l = cl_spot(p, vec3<f32>(-0.38, 0.0, 0.0), 16.0);
        let star_r = cl_spot(p, vec3<f32>(0.38, 0.0, 0.0), 16.0);
        let bridge = cl_bar(p, vec3<f32>(-0.36, 0.0, 0.0), vec3<f32>(0.36, 0.0, 0.0), 0.008, 86.0);
        let pulse_x = mix(-0.32, 0.32, fract(time * 0.25 + ff * 0.125));
        let shell_wave = cl_spot(p, vec3<f32>(pulse_x, 0.0, 0.0), 18.0)
                       + exp(-abs(length(p - vec3<f32>(-0.38, 0.0, 0.0)) - (0.14 + ff * 0.055)) * 52.0);
        col = vec3<f32>(1.32, 0.88, 0.28) * (star_l + star_r * 0.9)
            + vec3<f32>(0.22, 1.32, 1.44) * (bridge * 0.95 + shell_wave * 0.88)
            + vec3<f32>(0.42, 1.26, 0.82) * cl_feature_beacon(p, feat, 0.38, 0.05, time) * 0.65;
        dens = star_l * 0.8 + star_r * 0.75 + bridge * 0.8 + shell_wave * 0.78;
    } else if (z_idx == 23u) {
        // Z23 uncertain-contact: Multi-Observatory Wireframe Earth & Triangulation Baselines
        let lat_grid = exp(-abs(sin(p.y * 22.0)) * 18.0);
        let lon_grid = exp(-abs(sin(atan2(p.z, p.x) * 8.0)) * 18.0);
        let earth_wf = exp(-abs(r - 0.30) * 62.0) * max(lat_grid, lon_grid);
        let tri_angle = ff * 0.785398;
        let obs_a = vec3<f32>(0.28 * cos(tri_angle), 0.14, 0.28 * sin(tri_angle));
        let obs_b = vec3<f32>(0.28 * cos(tri_angle + 2.094), -0.10, 0.28 * sin(tri_angle + 2.094));
        let sky_pt = vec3<f32>(0.0, 0.46, 0.0);
        let rays = cl_bar(p, obs_a, sky_pt, 0.006, 88.0) + cl_bar(p, obs_b, sky_pt, 0.006, 88.0);
        col = vec3<f32>(0.24, 1.05, 1.46) * (earth_wf * 0.95 + rays)
            + vec3<f32>(1.34, 0.86, 0.28) * (cl_spot(p, sky_pt, 16.0) + cl_feature_beacon(p, feat, 0.42, 0.0, time) * 0.75);
        dens = earth_wf * 0.86 + rays * 0.82;
    } else {
        // Z24 unknown-cosmic-life: Non-Terran Exotic Biospheres Holographic Prism
        let pe = doc_rot_y(doc_rot_x(p, 0.44), time * 0.25 + ff * 0.42);
        let prism = doc_line_glow(sd_hex_prism(pe, 0.32, 0.16), 72.0);
        let crystal = cl_ring_xy(pe, 0.22 + 0.02 * ff, 0.008, 80.0);
        let exotic = pow(max(0.0, snoise3(pe * 9.0 + vec3<f32>(ff * 1.2))), 3.0) * smoothstep(0.46, 0.10, r);
        let e_col = hsl_to_rgb(fract(0.76 + ff * 0.11), 0.94, 0.56);
        col = e_col * (prism * 0.95 + exotic * 1.15)
            + vec3<f32>(0.26, 1.28, 1.42) * crystal * 0.85
            + vec3<f32>(1.28, 0.88, 0.34) * cl_feature_beacon(p, feat, 0.40, 0.0, time) * 0.72;
        dens = prism * 0.84 + crystal * 0.75 + exotic * 0.72;
    }

    col *= (0.86 + 0.18 * voice + 0.14 * bass) * feat_hi;
    return vec4<f32>(col * env, dens * env);
}

fn shade_doc_cosmic_contact_stage(
    p: vec3<f32>,
    z_idx: u32,
    feat: u32,
    inst: HeroVolumeInstance,
    zone_col: vec3<f32>,
    time: f32,
    voice: f32,
    bass: f32,
) -> vec4<f32> {
    let r = length(p);
    if (r > 0.72) {
        return vec4<f32>(0.0);
    }
    let env = smoothstep(0.72, 0.50, r);
    let ff = f32(min(feat, 7u));
    let feat_hi = select(0.55, 1.0, feat <= 7u);
    let f_use = select(feat, 0u, feat > 5u);
    let cue_p = 0.82 + 0.22 * doc_cue(inst);
    var col = vec3<f32>(0.0);
    var dens: f32 = 0.0;

    if (z_idx == 0u) {
        // Z00 unexplained-signal: Parabolic SETI Dish + Tripod Feed Horn + Six Candidate Sky Anomalies
        let pd = doc_rot_x(doc_rot_y(p - vec3<f32>(0.0, -0.18, 0.0), 0.18 * ff * 0.15), 0.42);
        let d_dish = ch_parabolic_dish_sdf(pd, 0.42, 0.21);
        let dish_edge = exp(-pow(abs(d_dish) * 76.0, 2.0));
        let grid_s = max(
            exp(-pow(abs(fract(atan2(pd.z, pd.x) / TAU * 12.0) - 0.5) * 18.0, 2.0)),
            exp(-pow(abs(fract(length(pd.xz) * 10.0) - 0.5) * 18.0, 2.0))
        );
        let tripod = cl_bar(pd, vec3<f32>(0.27, 0.0, 0.0), vec3<f32>(0.0, 0.32, 0.0), 0.008, 90.0)
                   + cl_bar(pd, vec3<f32>(-0.135, 0.0, 0.23), vec3<f32>(0.0, 0.32, 0.0), 0.008, 90.0)
                   + cl_bar(pd, vec3<f32>(-0.135, 0.0, -0.23), vec3<f32>(0.0, 0.32, 0.0), 0.008, 90.0);
        let feed_top = vec3<f32>(0.0, 0.32, 0.0);
        var sky = 0.0;
        var sky_col = vec3<f32>(0.24, 1.38, 0.96);
        if (f_use == 0u) {
            // F0: Narrow monochromatic RF carrier beam + phase ripples
            let wave = 0.50 + 0.72 * pow(max(sin(pd.y * 28.0 - time * 4.0), 0.0), 4.0);
            sky = cl_bar(pd, feed_top, vec3<f32>(0.0, 0.84, 0.0), 0.012, 78.0) * wave;
        } else if (f_use == 1u) {
            // F1: Brief optical laser pulse droplets into auxiliary side mirror
            for (var k = 0; k < 4; k = k + 1) {
                let yk = 0.36 + f32(k) * 0.10;
                sky += cl_spot(pd, vec3<f32>(0.12, yk, 0.0), 22.0);
            }
            sky += cl_ring_xz(pd - vec3<f32>(0.14, 0.16, 0.0), 0.12, 0.008, 84.0);
            sky_col = vec3<f32>(0.42, 1.28, 1.48);
        } else if (f_use == 2u) {
            // F2: Megastructure transit rectangular chord crossing blinking star
            let star = cl_spot(p, vec3<f32>(0.0, 0.36, 0.0), 14.0);
            let occ = exp(-pow(abs(p.y - 0.36) * 32.0, 2.0)) * smoothstep(0.18, 0.04, abs(p.x));
            sky = star * 0.9 + occ * 1.1;
            sky_col = vec3<f32>(1.38, 0.82, 0.28);
        } else if (f_use == 3u) {
            // F3: Exoplanet backlit atmospheric absorption ring with artificial notch
            let limb = cl_ring_xy(p - vec3<f32>(-0.14, 0.34, 0.0), 0.14, 0.010, 80.0);
            let band = cl_bar(p, vec3<f32>(-0.28, 0.34, 0.0), vec3<f32>(0.0, 0.34, 0.0), 0.006, 88.0);
            sky = limb + band * 0.8;
            sky_col = vec3<f32>(0.32, 1.22, 0.86);
        } else if (f_use == 4u) {
            // F4: Hyperbolic interstellar trajectory arcing past the dish
            let arc = cl_bar(p, vec3<f32>(-0.42, 0.42, 0.0), vec3<f32>(0.04, 0.26, 0.0), 0.009, 82.0)
                    + cl_bar(p, vec3<f32>(0.04, 0.26, 0.0), vec3<f32>(0.44, 0.44, 0.0), 0.009, 82.0)
                    + cl_spot(p, vec3<f32>(0.04, 0.26, 0.0), 18.0);
            sky = arc;
            sky_col = vec3<f32>(1.28, 0.68, 0.26);
        } else {
            // F5: Prime pulse sequence descending (2, 3, 5, 7 spacing)
            let p_a = cl_spot(pd, vec3<f32>(0.0, 0.42, 0.0), 20.0);
            let p_b = cl_spot(pd, vec3<f32>(0.0, 0.51, 0.0), 20.0);
            let p_c = cl_spot(pd, vec3<f32>(0.0, 0.62, 0.0), 20.0);
            let p_d = cl_ring_xz(pd - vec3<f32>(0.0, 0.72, 0.0), 0.09, 0.007, 82.0);
            sky = p_a + p_b + p_c + p_d;
            sky_col = vec3<f32>(0.54, 1.06, 1.46);
        }
        col = vec3<f32>(0.78, 1.05, 1.38) * dish_edge * (0.28 + 0.82 * grid_s)
            + vec3<f32>(1.36, 0.88, 0.30) * tripod
            + sky_col * sky
            + zone_col * cl_feature_beacon(p, feat, 0.40, 0.04, time) * 0.65;
        dens = dish_edge * 0.84 + tripod * 0.75 + sky * 0.85;
    } else if (z_idx == 1u) {
        // Z01 first-anomaly: Holographic Waterfall Spectrogram & 6-Sigma Residual Screen
        let ped = cl_ring_xz(p - vec3<f32>(0.0, -0.34, 0.0), 0.28, 0.010, 82.0)
                + cl_bar(p, vec3<f32>(0.0, -0.34, 0.0), vec3<f32>(0.0, -0.14, 0.0), 0.010, 84.0);
        let screen = exp(-pow(abs(p.z) * 26.0, 2.0)) * smoothstep(0.46, 0.36, abs(p.x)) * smoothstep(0.42, 0.32, abs(p.y - 0.08));
        let frame = cl_bar(p, vec3<f32>(-0.42, -0.12, 0.0), vec3<f32>(0.42, -0.12, 0.0), 0.007, 88.0)
                  + cl_bar(p, vec3<f32>(-0.42, 0.36, 0.0), vec3<f32>(0.42, 0.36, 0.0), 0.007, 88.0);
        var trace = 0.0;
        var t_col = vec3<f32>(0.24, 1.28, 1.42);
        if (f_use == 0u) {
            // F0: 6-sigma narrow spike rising cleanly above noise baseline
            let noise_floor = exp(-abs(p.y + 0.06) * 32.0) * smoothstep(0.40, 0.05, abs(p.x)) * (0.5 + 0.5 * sin(p.x * 48.0));
            let spike = cl_bar(p, vec3<f32>(0.08, -0.06, 0.0), vec3<f32>(0.08, 0.34, 0.0), 0.014, 72.0);
            trace = noise_floor * 0.7 + spike * 1.2;
        } else if (f_use == 1u) {
            // F1: Logarithmic ms-to-year pulse width scale bars
            for (var k = 0; k < 4; k = k + 1) {
                let bx = -0.27 + f32(k) * 0.18;
                let bw = 0.02 + f32(k) * 0.03;
                trace += cl_bar(p, vec3<f32>(bx, -0.04, 0.0), vec3<f32>(bx, 0.10 + f32(k) * 0.06, 0.0), bw * 0.35 + 0.008, 68.0);
            }
            t_col = vec3<f32>(1.28, 0.88, 0.32);
        } else if (f_use == 2u) {
            // F2: Diagonal waterfall drift ridge across time-frequency grid
            let ridge = cl_bar(p, vec3<f32>(-0.34, -0.06, 0.0), vec3<f32>(0.34, 0.30, 0.0), 0.014, 72.0);
            trace = ridge * (0.7 + 0.4 * sin(p.y * 28.0 - time * 3.0));
            t_col = vec3<f32>(0.26, 1.38, 0.92);
        } else if (f_use == 3u) {
            // F3: 3D circular polarization Stokes corkscrew helix
            let hy = 0.10 + 0.14 * cos(p.x * 16.0 - time * 2.5);
            let hz = 0.14 * sin(p.x * 16.0 - time * 2.5);
            trace = exp(-length(vec2<f32>(p.y - hy, p.z - hz)) * 32.0) * smoothstep(0.40, 0.08, abs(p.x));
            t_col = vec3<f32>(0.92, 0.42, 1.38);
        } else if (f_use == 4u) {
            // F4: Twin return-ping verification rings
            trace = cl_ring_xy(p - vec3<f32>(-0.18, 0.12, 0.0), 0.13, 0.009, 82.0)
                  + cl_ring_xy(p - vec3<f32>(0.18, 0.12, 0.0), 0.13, 0.009, 82.0);
            t_col = vec3<f32>(0.32, 1.32, 1.12);
        } else {
            // F5: Fading transient echo dissolving into sparse amber flecks
            let fade_seg = cl_bar(p, vec3<f32>(-0.36, 0.18, 0.0), vec3<f32>(0.02, 0.18, 0.0), 0.012, 74.0);
            let static_dots = pow(max(0.0, snoise3(p * 18.0)), 4.0) * smoothstep(-0.02, 0.36, p.x);
            trace = fade_seg + static_dots * 1.25;
            t_col = vec3<f32>(1.28, 0.56, 0.24);
        }
        col = vec3<f32>(0.42, 0.86, 1.32) * (ped + frame)
            + vec3<f32>(0.08, 0.22, 0.34) * screen * 0.45
            + t_col * trace
            + zone_col * cl_feature_beacon(p, feat, 0.42, 0.02, time) * 0.68;
        dens = (ped + frame) * 0.72 + screen * 0.25 + trace * 0.88;
    } else if (z_idx == 2u) {
        // Z02 instrumental-detection: Wireframe Horn Cutaway + 4-Stage Holographic DSP Pipeline
        let horn = cl_ring_xy(p - vec3<f32>(-0.34, 0.0, 0.0), 0.16, 0.009, 80.0)
                 + cl_bar(p, vec3<f32>(-0.34, 0.14, 0.0), vec3<f32>(-0.12, 0.02, 0.0), 0.008, 86.0)
                 + cl_bar(p, vec3<f32>(-0.34, -0.14, 0.0), vec3<f32>(-0.12, -0.02, 0.0), 0.008, 86.0);
        let pipe = cl_bar(p, vec3<f32>(-0.10, 0.0, 0.0), vec3<f32>(0.44, 0.0, 0.0), 0.008, 88.0);
        var dsp = 0.0;
        var d_col = vec3<f32>(0.24, 1.26, 1.44);
        if (f_use == 0u) {
            dsp = cl_spot(p, vec3<f32>(0.26, 0.24, 0.0), 15.0) + cl_ring_xy(p - vec3<f32>(0.26, 0.24, 0.0), 0.12, 0.008, 82.0);
            d_col = vec3<f32>(0.24, 1.34, 0.78);
        } else if (f_use == 1u) {
            dsp = cl_spot(p, vec3<f32>(0.18, 0.10, 0.0), 20.0) + cl_spot(p, vec3<f32>(0.32, -0.08, 0.0), 20.0)
                + cl_ring_xy(p - vec3<f32>(0.24, 0.0, 0.0), 0.18, 0.007, 82.0);
        } else if (f_use == 2u) {
            for (var k = 0; k < 3; k = k + 1) {
                dsp += cl_ring_xy(p - vec3<f32>(0.16, 0.0, 0.0), 0.09 + f32(k) * 0.08, 0.007, 82.0);
            }
            d_col = vec3<f32>(1.36, 0.94, 0.28);
        } else if (f_use == 3u) {
            for (var ix = 0; ix < 2; ix = ix + 1) {
                for (var iy = 0; iy < 2; iy = iy + 1) {
                    dsp += cl_spot(p, vec3<f32>(0.10 + f32(ix) * 0.18, -0.09 + f32(iy) * 0.18, 0.0), 20.0);
                }
            }
            d_col = vec3<f32>(0.42, 0.96, 1.48);
        } else if (f_use == 4u) {
            for (var k = 0; k < 4; k = k + 1) {
                let bx = -0.02 + f32(k) * 0.14;
                dsp += exp(-abs(sd_hex_prism(doc_rot_x(p - vec3<f32>(bx, 0.0, 0.0), 1.570796), 0.055, 0.04)) * 78.0);
            }
        } else {
            dsp = cl_bar(p, vec3<f32>(0.06, 0.0, 0.0), vec3<f32>(0.40, 0.18, 0.0), 0.008, 86.0)
                + cl_bar(p, vec3<f32>(0.06, 0.0, 0.0), vec3<f32>(0.40, -0.18, 0.0), 0.008, 86.0);
            d_col = vec3<f32>(0.82, 0.62, 1.44);
        }
        col = vec3<f32>(0.54, 1.02, 1.44) * (horn + pipe)
            + d_col * dsp
            + zone_col * cl_feature_beacon(p, feat, 0.40, 0.06, time) * 0.65;
        dens = (horn + pipe) * 0.78 + dsp * 0.86;
    } else if (z_idx == 3u) {
        // Z03 interference-rejected: Polar Antenna Radiation Lobes + Sidelobe RFI Rejection
        let ang_lobe = atan2(p.y + 0.16, length(p.xz));
        let main_lobe = exp(-abs(length(p - vec3<f32>(0.0, 0.06, 0.0)) - 0.26 * pow(max(sin(ang_lobe), 0.0), 2.5)) * 42.0) * step(-0.22, p.y);
        let side_lobe = cl_ring_xz(p - vec3<f32>(0.0, -0.18, 0.0), 0.36, 0.008, 78.0);
        let core_dish = cl_bar(p, vec3<f32>(0.0, -0.26, 0.0), vec3<f32>(0.0, 0.24, 0.0), 0.010, 82.0);
        var rfi = 0.0;
        var r_col = vec3<f32>(1.36, 0.38, 0.22);
        if (f_use == 0u) {
            // F0: LEO satellite streak through lower sidelobe
            rfi = cl_bar(p, vec3<f32>(-0.42, 0.06, 0.0), vec3<f32>(0.42, 0.22, 0.0), 0.009, 82.0)
                + cl_spot(p, vec3<f32>(0.18, 0.17, 0.0), 17.0);
        } else if (f_use == 1u) {
            // F1: Aircraft sweeping radar fan
            rfi = cl_bar(p, vec3<f32>(-0.42, -0.18, 0.0), vec3<f32>(-0.08, -0.02, 0.0), 0.012, 76.0)
                + cl_ring_xy(p - vec3<f32>(-0.28, -0.10, 0.0), 0.14, 0.008, 80.0);
        } else if (f_use == 2u) {
            // F2: Amber ground-loop EMI from local electronics shed
            rfi = cl_ring_xz(p - vec3<f32>(0.24, -0.28, 0.0), 0.14, 0.010, 74.0)
                + cl_spot(p, vec3<f32>(0.24, -0.28, 0.0), 16.0);
            r_col = vec3<f32>(1.38, 0.78, 0.22);
        } else if (f_use == 3u) {
            // F3: Oblique Moon/meteor bounce arc
            rfi = cl_bar(p, vec3<f32>(0.36, 0.36, 0.0), vec3<f32>(0.08, -0.10, 0.0), 0.008, 82.0)
                + cl_spot(p, vec3<f32>(0.36, 0.36, 0.0), 14.0);
            r_col = vec3<f32>(0.92, 0.86, 1.34);
        } else if (f_use == 4u) {
            // F4: Vertical bit-overflow clipping sawtooth
            for (var k = 0; k < 4; k = k + 1) {
                let xk = -0.24 + f32(k) * 0.16;
                rfi += cl_bar(p, vec3<f32>(xk, -0.12, 0.0), vec3<f32>(xk + 0.06, 0.16, 0.0), 0.007, 84.0);
            }
            r_col = vec3<f32>(1.34, 0.34, 0.48);
        } else {
            // F5: Twin ON-source / OFF-source nod calibration cones
            rfi = cl_bar(p, vec3<f32>(0.0, -0.16, 0.0), vec3<f32>(0.26, 0.34, 0.0), 0.014, 72.0)
                + cl_bar(p, vec3<f32>(0.0, -0.16, 0.0), vec3<f32>(-0.26, 0.34, 0.0), 0.008, 84.0) * 0.45;
            r_col = vec3<f32>(0.24, 1.38, 0.96);
        }
        col = vec3<f32>(0.26, 1.02, 1.44) * (main_lobe * 0.88 + side_lobe * 0.65 + core_dish * 0.85)
            + r_col * rfi
            + zone_col * cl_feature_beacon(p, feat, 0.38, 0.02, time) * 0.62;
        dens = (main_lobe + side_lobe + core_dish) * 0.72 + rfi * 0.85;
    } else if (z_idx == 4u) {
        // Z04 independent-verification (+companion 3830): Wireframe Earth & Synchronized Observatories
        let lat_g = exp(-abs(sin(p.y * 22.0)) * 18.0);
        let lon_g = exp(-abs(sin(atan2(p.z, p.x) * 8.0)) * 18.0);
        let globe = exp(-abs(length(p - vec3<f32>(0.0, -0.06, 0.0)) - 0.24) * 58.0) * max(lat_g, lon_g);
        let beam_a = cl_bar(p, vec3<f32>(-0.20, 0.08, 0.0), vec3<f32>(0.0, 0.44, 0.0), 0.007, 88.0);
        let beam_b = cl_bar(p, vec3<f32>(0.20, 0.04, 0.0), vec3<f32>(0.0, 0.44, 0.0), 0.007, 88.0);
        var overlay = beam_a + beam_b;
        var o_col = vec3<f32>(0.26, 1.32, 0.94);
        if (f_use == 1u) {
            for (var k = 0; k < 4; k = k + 1) {
                overlay += cl_ring_xz(p - vec3<f32>(0.0, 0.22 + f32(k) * 0.07, 0.0), 0.14 - f32(k) * 0.02, 0.007, 84.0);
            }
            o_col = vec3<f32>(0.64, 0.96, 1.46);
        } else if (f_use == 2u) {
            overlay += cl_bar(p, vec3<f32>(0.0, 0.18, 0.0), vec3<f32>(0.0, -0.30, 0.0), 0.009, 86.0);
        } else if (f_use == 3u) {
            overlay += cl_ring_xy(p - vec3<f32>(0.26, 0.24, 0.0), 0.11, 0.009, 82.0);
            o_col = vec3<f32>(1.38, 0.96, 0.28);
        } else if (f_use == 4u) {
            for (var k = 0; k < 3; k = k + 1) {
                overlay += cl_ring_xy(p - vec3<f32>(-0.26, 0.16 + f32(k) * 0.07, -0.06), 0.12, 0.006, 84.0);
            }
            o_col = vec3<f32>(0.88, 0.72, 1.38);
        } else if (f_use == 5u) {
            overlay += doc_line_glow(sd_hex_prism(doc_rot_x(p - vec3<f32>(0.0, 0.34, 0.0), 1.570796), 0.14, 0.015), 76.0);
            o_col = vec3<f32>(1.28, 0.64, 0.28);
        }
        col = vec3<f32>(0.24, 0.92, 1.42) * globe + o_col * overlay
            + zone_col * cl_feature_beacon(p, feat, 0.42, 0.0, time) * 0.62;
        dens = globe * 0.84 + overlay * 0.82;
    } else if (z_idx == 5u) {
        // Z05 distant-origin (+companion 3831): Rotating Earth Linked to Distant Star System
        let earth_wf = exp(-abs(length(p - vec3<f32>(-0.28, -0.10, 0.0)) - 0.15) * 56.0);
        let star_pt = cl_spot(p, vec3<f32>(0.30, 0.22, 0.0), 14.0);
        var link = cl_bar(p, vec3<f32>(-0.16, 0.0, 0.0), vec3<f32>(0.24, 0.18, 0.0), 0.008, 84.0);
        var l_col = vec3<f32>(0.32, 1.28, 1.44);
        if (f_use == 1u) {
            // F1: Sinusoidal barycentric Doppler S-curve
            let wave = exp(-abs(p.y - 0.12 * sin(p.x * 11.0)) * 34.0) * smoothstep(0.38, 0.06, abs(p.x));
            link += wave * 1.1;
        } else if (f_use == 2u) {
            // F2: Dispersive ISM delay cloud separating violet & red packets
            let ism = exp(-pow(length(p - vec3<f32>(0.02, 0.08, 0.0)) * 8.0, 2.0));
            link += cl_spot(p, vec3<f32>(-0.08, 0.05, 0.0), 19.0) + cl_spot(p, vec3<f32>(0.14, 0.14, 0.0), 19.0) + ism * 0.7;
            l_col = vec3<f32>(0.88, 0.46, 1.42);
        } else if (f_use == 3u) {
            // F3: Wide orbital parallax triangle baseline
            link += cl_bar(p, vec3<f32>(-0.28, -0.26, 0.0), vec3<f32>(0.30, 0.22, 0.0), 0.007, 86.0)
                  + cl_bar(p, vec3<f32>(-0.28, 0.08, 0.0), vec3<f32>(0.30, 0.22, 0.0), 0.007, 86.0);
            l_col = vec3<f32>(0.24, 1.38, 0.88);
        } else if (f_use == 4u) {
            // F4: Exoplanet orbit ring around target star
            link += cl_ring_xz(doc_rot_x(p - vec3<f32>(0.30, 0.22, 0.0), 0.45), 0.18, 0.008, 82.0);
            l_col = vec3<f32>(1.32, 0.92, 0.34);
        } else if (f_use == 5u) {
            // F5: Orbital transmitter phase wobble circles
            link += cl_ring_xy(p - vec3<f32>(0.30, 0.22, 0.0), 0.13, 0.008, 80.0)
                  + cl_ring_xy(p - vec3<f32>(0.30, 0.22, 0.0), 0.22, 0.007, 80.0);
        }
        col = vec3<f32>(0.24, 0.96, 1.46) * earth_wf
            + vec3<f32>(1.42, 0.98, 0.36) * star_pt
            + l_col * link
            + zone_col * cl_feature_beacon(p, feat, 0.40, 0.0, time) * 0.62;
        dens = earth_wf * 0.82 + star_pt * 0.88 + link * 0.80;
    } else if (z_idx == 6u) {
        // Z06 artificial-pattern: Prime Pulse Clusters, Phase Modulation, Entropy Crystal vs LGM-1 Pulsar
        var pat = 0.0;
        var p_col = vec3<f32>(0.26, 1.34, 1.28);
        if (f_use == 0u) {
            for (var k = 0; k < 5; k = k + 1) {
                let cx = -0.36 + f32(k) * 0.18;
                pat += cl_spot(p, vec3<f32>(cx, 0.12 * sin(f32(k)), 0.0), 17.0);
            }
        } else if (f_use == 1u) {
            let phase_step = select(-1.0, 1.0, fract(p.x * 5.0) > 0.5);
            pat = exp(-abs(p.y - 0.16 * phase_step * sin(p.x * 24.0)) * 34.0) * smoothstep(0.45, 0.10, abs(p.x));
            p_col = vec3<f32>(0.42, 0.96, 1.48);
        } else if (f_use == 2u) {
            let dust = pow(max(0.0, snoise3(p * 11.0)), 3.0) * smoothstep(0.04, -0.28, p.x);
            let crystal = doc_line_glow(sd_hex_prism(doc_rot_y(p - vec3<f32>(0.22, 0.0, 0.0), time * 0.25), 0.18, 0.14), 76.0);
            pat = dust + crystal;
            p_col = vec3<f32>(0.22, 1.38, 0.88);
        } else if (f_use == 3u) {
            let grid = max(exp(-abs(fract(p.x * 8.0) - 0.5) * 18.0), exp(-abs(fract(p.y * 8.0) - 0.5) * 18.0))
                     * smoothstep(0.36, 0.26, abs(p.x)) * smoothstep(0.36, 0.26, abs(p.y)) * exp(-abs(p.z) * 16.0);
            pat = grid * 1.15;
            p_col = vec3<f32>(1.36, 0.96, 0.30);
        } else if (f_use == 4u) {
            let needle = cl_bar(p, vec3<f32>(0.0, -0.44, 0.0), vec3<f32>(0.0, 0.44, 0.0), 0.011, 86.0);
            let iso = exp(-abs(r - 0.34) * 44.0) * 0.45;
            pat = needle + iso;
        } else {
            // F5: Spinning natural pulsar lighthouse impostor (LGM-1)
            let pp = doc_rot_z(p, 0.48 + time * 0.5);
            pat = cl_spot(pp, vec3<f32>(0.0), 14.0)
                + cl_bar(pp, vec3<f32>(0.0, -0.42, 0.0), vec3<f32>(0.0, 0.42, 0.0), 0.014, 70.0)
                + cl_ring_xz(pp, 0.24, 0.008, 78.0);
            p_col = vec3<f32>(1.32, 0.62, 0.26);
        }
        col = p_col * pat + zone_col * cl_feature_beacon(p, feat, 0.42, 0.02, time) * 0.70;
        dens = pat * 0.88;
    } else if (z_idx == 7u) {
        // Z07 accumulated-evidence: 6-Tier Luminous Rio-Scale Inference Pyramid
        var pyr = 0.0;
        for (var k = 0; k < 6; k = k + 1) {
            let fk = f32(k);
            let yk = -0.30 + fk * 0.12;
            let rk = 0.42 - fk * 0.062;
            let active_tier = select(0.32, 1.15, k == i32(f_use));
            pyr += cl_ring_xz(p - vec3<f32>(0.0, yk, 0.0), rk, 0.009, 82.0) * active_tier;
        }
        let spine = cl_bar(p, vec3<f32>(0.0, -0.32, 0.0), vec3<f32>(0.0, 0.36, 0.0), 0.010, 82.0)
                  * select(0.55, 1.35, f_use >= 3u);
        let apex = cl_spot(p, vec3<f32>(0.0, 0.36 - select(0.0, 0.14, f_use == 0u), 0.0), 14.0);
        let pyr_col = mix(vec3<f32>(0.32, 1.06, 1.46), vec3<f32>(1.42, 0.96, 0.28), ff * 0.18);
        col = pyr_col * (pyr + spine) + vec3<f32>(1.42, 1.04, 0.38) * apex
            + zone_col * cl_feature_beacon(p, feat, 0.38, -0.02 + ff * 0.05, time) * 0.72;
        dens = pyr * 0.78 + spine * 0.72 + apex * 0.85;
    } else if (z_idx == 8u) {
        // Z08 scientific-announcement: Translucent Disclosure Prism Pipeline
        let prism = doc_line_glow(sd_hex_prism(doc_rot_y(p, 0.35), 0.28, 0.22), 76.0);
        var stage = 0.0;
        var s_col = vec3<f32>(0.34, 1.18, 1.46);
        if (f_use == 0u) {
            for (var k = 0; k < 3; k = k + 1) {
                let a = f32(k) * 2.094 + time * 0.4;
                stage += cl_ring_xy(p - vec3<f32>(cos(a) * 0.26, sin(a) * 0.22, 0.0), 0.11, 0.008, 82.0);
            }
        } else if (f_use == 1u) {
            for (var k = 0; k < 6; k = k + 1) {
                let a = f32(k) * TAU / 6.0;
                stage += cl_bar(p, vec3<f32>(0.0), vec3<f32>(cos(a) * 0.44, sin(a) * 0.44, 0.0), 0.007, 86.0);
            }
            s_col = vec3<f32>(0.24, 1.36, 0.92);
        } else if (f_use == 2u) {
            stage = cl_bar(p, vec3<f32>(0.0, 0.06, 0.0), vec3<f32>(0.38, 0.34, 0.0), 0.012, 74.0)
                  + cl_spot(p, vec3<f32>(0.38, 0.34, 0.0), 15.0);
            s_col = vec3<f32>(1.42, 0.38, 0.18);
        } else if (f_use == 3u) {
            for (var k = 0; k < 4; k = k + 1) {
                stage += cl_ring_xz(p - vec3<f32>(-0.24 + f32(k) * 0.16, -0.18 + f32(k) * 0.12, 0.0), 0.09, 0.008, 82.0);
            }
            s_col = vec3<f32>(1.32, 0.94, 0.34);
        } else if (f_use == 4u) {
            for (var k = 0; k < 3; k = k + 1) {
                stage += exp(-abs(r - (0.14 + f32(k) * 0.11)) * 48.0);
            }
        } else {
            stage = cl_ring_xy(p - vec3<f32>(0.18, 0.18, 0.0), 0.15, 0.010, 80.0)
                  + cl_bar(p, vec3<f32>(0.06, 0.18, 0.0), vec3<f32>(0.30, 0.18, 0.0), 0.008, 84.0);
            s_col = vec3<f32>(1.38, 0.98, 0.28);
        }
        col = vec3<f32>(0.42, 0.88, 1.38) * prism * 0.85 + s_col * stage
            + zone_col * cl_feature_beacon(p, feat, 0.42, 0.04, time) * 0.65;
        dens = prism * 0.72 + stage * 0.84;
    } else if (z_idx == 9u) {
        // Z09 international-coordination: Wireframe Earth + 6 Polycentric Governance Rings
        let lat_g = exp(-abs(sin(p.y * 20.0)) * 18.0);
        let lon_g = exp(-abs(sin(atan2(p.z, p.x) * 8.0)) * 18.0);
        let globe = exp(-abs(r - 0.24) * 58.0) * max(lat_g, lon_g);
        let ring_m = cl_ring_xz(doc_rot_z(p, 0.22 * ff), 0.38, 0.008, 82.0)
                   + cl_ring_xy(doc_rot_y(p, 0.35 * ff), 0.44, 0.007, 82.0);
        var nodes = 0.0;
        for (var k = 0; k < 6; k = k + 1) {
            let a = f32(k) * TAU / 6.0 + ff * 0.25;
            let node_hi = select(0.45, 1.25, k == i32(f_use));
            nodes += cl_spot(p, vec3<f32>(cos(a) * 0.40, 0.0, sin(a) * 0.40), 17.0) * node_hi;
        }
        col = vec3<f32>(0.24, 0.96, 1.46) * (globe + ring_m * 0.82)
            + vec3<f32>(1.36, 0.94, 0.32) * nodes
            + zone_col * cl_feature_beacon(p, feat, 0.44, 0.06, time) * 0.62;
        dens = globe * 0.82 + ring_m * 0.72 + nodes * 0.80;
    } else if (z_idx == 10u) {
        // Z10 planetary-reaction: Social Refraction Kaleidoscope Dome Around Earth
        let core_e = exp(-abs(r - 0.20) * 56.0);
        var dome = 0.0;
        var d_col = vec3<f32>(0.92, 0.56, 1.38);
        if (f_use == 0u) {
            for (var k = 0; k < 4; k = k + 1) {
                let yk = -0.24 + f32(k) * 0.16;
                dome += exp(-abs(p.y - yk) * 34.0) * smoothstep(0.46, 0.16, abs(p.x)) * exp(-abs(p.z) * 16.0);
            }
            d_col = vec3<f32>(1.38, 0.42, 0.34);
        } else if (f_use == 1u) {
            for (var k = 0; k < 5; k = k + 1) {
                let a = f32(k) * 1.25 + time * 0.3;
                dome += doc_line_glow(sd_hex_prism(doc_rot_y(p - vec3<f32>(cos(a) * 0.34, sin(a) * 0.24, 0.0), a), 0.08, 0.015), 76.0);
            }
            d_col = vec3<f32>(1.32, 0.34, 0.72);
        } else if (f_use == 2u) {
            for (var k = 0; k < 3; k = k + 1) {
                dome += cl_ring_xy(doc_rot_z(p, f32(k) * 1.047), 0.36, 0.008, 82.0);
            }
            d_col = vec3<f32>(1.38, 0.96, 0.36);
        } else if (f_use == 3u) {
            for (var k = 0; k < 3; k = k + 1) {
                dome += cl_ring_xz(p, 0.26 + f32(k) * 0.10, 0.007, 82.0);
            }
            d_col = vec3<f32>(0.32, 1.18, 1.46);
        } else if (f_use == 4u) {
            let belt = exp(-abs(p.y) * 28.0) * exp(-abs(length(p.xz) - 0.32) * 36.0);
            dome = belt + cl_ring_xz(p, 0.42, 0.006, 82.0);
            d_col = vec3<f32>(1.28, 0.82, 0.32);
        } else {
            let bright_side = smoothstep(-0.04, 0.12, p.x) * exp(-abs(r - 0.38) * 44.0);
            let dim_side = smoothstep(0.04, -0.12, p.x) * exp(-abs(r - 0.38) * 44.0) * 0.18;
            dome = bright_side + dim_side;
            d_col = vec3<f32>(0.26, 1.28, 1.44);
        }
        col = vec3<f32>(0.24, 0.92, 1.42) * core_e + d_col * dome
            + zone_col * cl_feature_beacon(p, feat, 0.44, 0.02, time) * 0.65;
        dens = core_e * 0.80 + dome * 0.84;
    } else if (z_idx == 11u) {
        // Z11 interpretation: Bitstream De-Framing Prism & Arecibo Matrix Unfolder
        let ribbon = cl_bar(p, vec3<f32>(-0.44, 0.22, 0.0), vec3<f32>(0.44, 0.22, 0.0), 0.008, 86.0);
        var mat = 0.0;
        var m_col = vec3<f32>(0.26, 1.34, 1.18);
        if (f_use == 0u) {
            mat = cl_bar(p, vec3<f32>(-0.18, -0.24, 0.0), vec3<f32>(-0.18, 0.32, 0.0), 0.011, 80.0)
                + cl_bar(p, vec3<f32>(0.18, -0.24, 0.0), vec3<f32>(0.18, 0.32, 0.0), 0.011, 80.0);
            m_col = vec3<f32>(1.38, 0.96, 0.28);
        } else if (f_use == 1u) {
            let grid_2d = max(exp(-abs(fract(p.x * 10.0) - 0.5) * 18.0), exp(-abs(fract(p.y * 10.0) - 0.5) * 18.0))
                        * smoothstep(0.32, 0.22, abs(p.x)) * smoothstep(0.24, 0.14, abs(p.y + 0.06)) * exp(-abs(p.z) * 18.0);
            mat = grid_2d * 1.15;
        } else if (f_use == 2u) {
            for (var k = 0; k < 3; k = k + 1) {
                mat += cl_bar(p, vec3<f32>(-0.34, -0.22 + f32(k) * 0.14, 0.0), vec3<f32>(0.34, -0.22 + f32(k) * 0.14, 0.0), 0.008, 84.0);
            }
            m_col = vec3<f32>(0.36, 1.18, 1.48);
        } else if (f_use == 3u) {
            let dense_l = pow(max(0.0, snoise3(p * 15.0)), 3.0) * smoothstep(0.0, -0.28, p.x) * smoothstep(0.35, 0.12, abs(p.y));
            let open_r = doc_line_glow(sd_hex_prism(doc_rot_y(p - vec3<f32>(0.24, -0.04, 0.0), 0.4), 0.18, 0.14), 76.0);
            mat = dense_l + open_r;
        } else if (f_use == 4u) {
            for (var k = 0; k < 5; k = k + 1) {
                let a = f32(k) * TAU / 5.0;
                mat += cl_ring_xy(p - vec3<f32>(cos(a) * 0.28, sin(a) * 0.22 - 0.06, 0.0), 0.08, 0.007, 82.0);
            }
            m_col = vec3<f32>(1.18, 0.68, 1.42);
        } else {
            for (var k = 0; k < 4; k = k + 1) {
                mat += cl_spot(p, vec3<f32>(-0.32 + f32(k) * 0.22, -0.06, 0.0), 17.0);
            }
        }
        col = vec3<f32>(0.42, 1.02, 1.46) * ribbon + m_col * mat
            + zone_col * cl_feature_beacon(p, feat, 0.40, 0.0, time) * 0.68;
        dens = ribbon * 0.72 + mat * 0.86;
    } else if (z_idx == 12u) {
        // Z12 shared-mathematics: Prime Cipher Beacon (5 Concentric Prime Rings + Universal Constants)
        let pm = doc_rot_x(p, 0.42);
        let p1 = cl_ring_xz(pm, 0.12, 0.007, 90.0);
        let p2 = cl_ring_xz(pm, 0.19, 0.007, 90.0);
        let p3 = cl_ring_xz(pm, 0.31, 0.007, 90.0);
        let p4 = cl_ring_xz(pm, 0.44, 0.007, 90.0);
        let p5 = cl_ring_xz(pm, 0.58, 0.007, 90.0) * select(1.0, smoothstep(0.60, 0.36, r), feat == 5u);
        let ring_sum = p1 + p2 + p3 + p4 + p5;
        let ang_pm = atan2(pm.z, pm.x);
        let bin_code = 0.58 + 0.50 * step(0.15, sin(ang_pm * 13.0 + length(pm.xz) * 18.0));
        let pulse = 0.78 + 0.38 * sin(r * 16.0 - time * 3.0);
        let pulsar_rays = exp(-pow(abs(pm.y) * 44.0, 2.0)) * exp(-pow(min(abs(pm.x), abs(pm.z)) * 46.0, 2.0)) * smoothstep(0.66, 0.14, length(pm.xz));
        var extra_m = 0.0;
        var e_col = vec3<f32>(1.42, 1.02, 0.34);
        if (f_use == 1u && feat <= 7u) {
            // F1: 3-4-5 right triangle inscribed in prime plane
            extra_m = cl_bar(pm, vec3<f32>(-0.22, 0.0, -0.14), vec3<f32>(0.22, 0.0, -0.14), 0.008, 86.0)
                    + cl_bar(pm, vec3<f32>(0.22, 0.0, -0.14), vec3<f32>(0.22, 0.0, 0.18), 0.008, 86.0)
                    + cl_bar(pm, vec3<f32>(-0.22, 0.0, -0.14), vec3<f32>(0.22, 0.0, 0.18), 0.008, 86.0);
        } else if (f_use == 2u && feat <= 7u) {
            // F2: Hydrogen 21 cm hyperfine twin-spin atom pair
            extra_m = cl_spot(p, vec3<f32>(-0.16, 0.0, 0.0), 17.0) + cl_spot(p, vec3<f32>(0.16, 0.0, 0.0), 17.0)
                    + cl_bar(p, vec3<f32>(-0.16, -0.14, 0.0), vec3<f32>(-0.16, 0.14, 0.0), 0.007, 86.0)
                    + cl_bar(p, vec3<f32>(0.16, -0.14, 0.0), vec3<f32>(0.16, 0.14, 0.0), 0.007, 86.0);
            e_col = vec3<f32>(0.34, 1.38, 0.88);
        } else if (f_use == 3u && feat <= 7u) {
            // F3: Dimensionless pi & alpha calipers
            extra_m = cl_bar(pm, vec3<f32>(-0.31, 0.0, 0.0), vec3<f32>(0.31, 0.0, 0.0), 0.010, 82.0)
                    + cl_bar(pm, vec3<f32>(0.0, 0.0, -0.31), vec3<f32>(0.0, 0.0, 0.31), 0.010, 82.0);
            e_col = vec3<f32>(0.86, 0.68, 1.44);
        } else if (f_use == 4u && feat <= 7u) {
            // F4: Four Boolean logic-gate nodes around inner prime loop
            for (var k = 0; k < 4; k = k + 1) {
                let a = f32(k) * 1.570796 + 0.785;
                extra_m += cl_spot(pm, vec3<f32>(cos(a) * 0.25, 0.0, sin(a) * 0.25), 17.0);
            }
            e_col = vec3<f32>(0.24, 1.24, 1.48);
        } else if (f_use == 5u && feat <= 7u) {
            // F5: Soft outer semantic boundary halo
            extra_m = exp(-abs(r - 0.56) * 28.0);
            e_col = vec3<f32>(0.96, 0.56, 1.28);
        }
        let tint = mix(vec3<f32>(0.30, 1.32, 1.42), zone_col * 1.25, 0.22);
        col = tint * ring_sum * pulse * bin_code
            + vec3<f32>(1.38, 1.00, 0.34) * pulsar_rays
            + e_col * extra_m
            + zone_col * cl_feature_beacon(p, feat, 0.44, 0.02, time) * 0.65;
        dens = ring_sum * 0.88 + pulsar_rays * 0.76 + extra_m * 0.80;
    } else if (z_idx == 13u) {
        // Z13 meaning-construction: Lincos Ostensive Grounding & Relational Mapping Sandbox
        let glyph_a = cl_ring_xy(p - vec3<f32>(-0.26, 0.04, 0.0), 0.14, 0.009, 82.0);
        let obj_b = cl_spot(p, vec3<f32>(0.26, 0.04, 0.0), 14.0);
        var map_g = cl_bar(p, vec3<f32>(-0.12, 0.04, 0.0), vec3<f32>(0.18, 0.04, 0.0), 0.008, 84.0);
        if (f_use == 1u) {
            map_g += cl_spot(p, vec3<f32>(0.26, 0.24, 0.0), 17.0) + cl_spot(p, vec3<f32>(0.26, -0.16, 0.0), 17.0);
        } else if (f_use == 2u) {
            map_g += doc_line_glow(sd_hex_prism(doc_rot_y(p, time * 0.6), 0.16, 0.12), 76.0);
        } else if (f_use == 3u) {
            map_g += cl_ring_xz(p - vec3<f32>(-0.22, -0.18, 0.0), 0.14, 0.008, 82.0)
                   + cl_ring_xz(p - vec3<f32>(0.22, -0.18, 0.0), 0.14, 0.008, 82.0);
        } else if (f_use == 4u) {
            map_g += cl_bar(p, vec3<f32>(-0.26, 0.04, 0.0), vec3<f32>(0.0, 0.30, 0.0), 0.007, 86.0)
                   + cl_bar(p, vec3<f32>(0.0, 0.30, 0.0), vec3<f32>(0.26, 0.04, 0.0), 0.007, 86.0);
        } else if (f_use == 5u) {
            map_g += cl_ring_xy(p - vec3<f32>(0.0, 0.04, -0.12), 0.26, 0.007, 82.0)
                   + cl_ring_xy(p - vec3<f32>(0.0, 0.04, 0.12), 0.26, 0.007, 82.0);
        }
        col = vec3<f32>(0.28, 1.24, 1.46) * glyph_a
            + vec3<f32>(1.38, 0.96, 0.32) * (obj_b + map_g * 0.85)
            + zone_col * cl_feature_beacon(p, feat, 0.40, 0.04, time) * 0.65;
        dens = glyph_a * 0.8 + obj_b * 0.8 + map_g * 0.82;
    } else if (z_idx == 14u) {
        // Z14 unknown-language: 4D Alien Multiplexing Hypercube & Nested Recursive Grammar
        let pa = doc_rot_y(doc_rot_x(p, 0.38 + ff * 0.18), time * 0.25 + ff * 0.32);
        let outer_c = doc_line_glow(sd_hex_prism(pa, 0.36, 0.24), 76.0);
        let inner_c = doc_line_glow(sd_hex_prism(pa, 0.18, 0.12), 82.0);
        var xeno = 0.0;
        var x_col = vec3<f32>(0.76, 0.44, 1.44);
        if (f_use == 0u) {
            for (var k = 0; k < 4; k = k + 1) {
                let a = f32(k) * 1.570796;
                xeno += cl_bar(pa, vec3<f32>(cos(a) * 0.16, 0.0, sin(a) * 0.16), vec3<f32>(cos(a) * 0.34, 0.12, sin(a) * 0.34), 0.007, 86.0);
            }
        } else if (f_use == 1u) {
            xeno = cl_ring_xy(pa, 0.28, 0.008, 82.0) + cl_ring_xz(pa, 0.16, 0.008, 82.0);
            x_col = vec3<f32>(0.28, 1.32, 1.44);
        } else if (f_use == 2u) {
            let vortex = exp(-abs(length(pa.xz) - 0.16 - 0.10 * sin(atan2(pa.z, pa.x) * 3.0)) * 32.0);
            xeno = vortex * smoothstep(0.45, 0.10, r);
            x_col = vec3<f32>(1.28, 0.42, 0.96);
        } else if (f_use == 3u) {
            for (var k = 0; k < 3; k = k + 1) {
                let a = f32(k) * 2.094;
                xeno += cl_ring_xy(p - vec3<f32>(cos(a) * 0.28, sin(a) * 0.24, 0.0), 0.08, 0.008, 80.0);
            }
            x_col = vec3<f32>(0.44, 0.76, 1.36);
        } else if (f_use == 4u) {
            let fake_grid = max(exp(-abs(fract(pa.x * 9.0) - 0.5) * 18.0), exp(-abs(fract(pa.y * 9.0) - 0.5) * 18.0))
                          * smoothstep(0.32, 0.14, r);
            xeno = fake_grid;
            x_col = vec3<f32>(1.38, 0.46, 0.26);
        } else {
            xeno = cl_ring_xy(p - vec3<f32>(-0.22, 0.0, 0.0), 0.14, 0.009, 80.0)
                 + doc_line_glow(sd_hex_prism(p - vec3<f32>(0.22, 0.0, 0.0), 0.14, 0.10), 78.0);
            x_col = vec3<f32>(1.32, 0.96, 0.34);
        }
        col = vec3<f32>(0.34, 1.06, 1.46) * (outer_c * 0.78 + inner_c * 0.92) + x_col * xeno
            + zone_col * cl_feature_beacon(p, feat, 0.42, 0.04, time) * 0.65;
        dens = (outer_c + inner_c) * 0.72 + xeno * 0.84;
    } else if (z_idx == 15u) {
        // Z15 cultural-context: Situated Xenocultural Mind Prisms
        var cult = 0.0;
        var c_col = vec3<f32>(0.36, 1.24, 1.18);
        if (f_use == 0u) {
            cult = cl_spot(p, vec3<f32>(-0.26, 0.0, 0.0), 15.0)
                 + cl_ring_xy(p, 0.16, 0.008, 82.0)
                 + cl_bar(p, vec3<f32>(0.16, -0.22, 0.0), vec3<f32>(0.34, 0.22, 0.0), 0.009, 80.0);
        } else if (f_use == 1u) {
            cult = cl_ring_xy(p, 0.15, 0.009, 84.0) * (0.6 + 0.4 * sin(time * 6.0))
                 + cl_ring_xy(p, 0.36, 0.010, 80.0);
            c_col = vec3<f32>(1.36, 0.88, 0.32);
        } else if (f_use == 2u) {
            cult = cl_ring_xz(doc_rot_x(p, 0.78), 0.28, 0.009, 80.0)
                 + cl_ring_xz(doc_rot_x(p, -0.78), 0.28, 0.009, 80.0)
                 + cl_spot(p, vec3<f32>(0.0), 16.0);
            c_col = vec3<f32>(0.88, 0.48, 1.44);
        } else if (f_use == 3u) {
            for (var k = 0; k < 4; k = k + 1) {
                let a = f32(k) * 1.570796 + 0.785;
                cult += cl_spot(p, vec3<f32>(cos(a) * 0.26, sin(a) * 0.26, 0.0), 16.0);
            }
            c_col = vec3<f32>(0.28, 1.32, 0.86);
        } else if (f_use == 4u) {
            cult = cl_bar(p, vec3<f32>(-0.28, -0.14, 0.0), vec3<f32>(0.28, 0.18, 0.0), 0.010, 82.0)
                 + cl_bar(p, vec3<f32>(0.0, -0.32, 0.0), vec3<f32>(0.0, 0.24, 0.0), 0.009, 84.0);
            c_col = vec3<f32>(1.38, 0.96, 0.28);
        } else {
            // F5: Binary twin-star shadows across planetary sculpture
            cult = cl_spot(p, vec3<f32>(-0.24, 0.24, 0.0), 15.0)
                 + cl_spot(p, vec3<f32>(0.24, 0.24, 0.0), 15.0)
                 + cl_ring_xz(p - vec3<f32>(0.0, -0.10, 0.0), 0.24, 0.010, 80.0);
            c_col = vec3<f32>(1.28, 0.72, 0.34);
        }
        col = c_col * cult + zone_col * cl_feature_beacon(p, feat, 0.42, 0.0, time) * 0.68;
        dens = cult * 0.88;
    } else if (z_idx == 16u) {
        // Z16 message-received: Segmented Floating Holographic Dossier Polyhedron
        let shell = doc_line_glow(sd_hex_prism(doc_rot_y(doc_rot_x(p, 0.32), 0.4 + ff * 0.25), 0.34, 0.22), 76.0);
        var core = 0.0;
        var k_col = vec3<f32>(0.32, 1.32, 1.44);
        if (f_use == 0u) {
            core = cl_ring_xz(p, 0.20, 0.008, 82.0) + cl_spot(p, vec3<f32>(0.0), 15.0);
        } else if (f_use == 1u) {
            for (var k = 0; k < 6; k = k + 1) {
                let a = f32(k) * TAU / 6.0;
                core += cl_bar(p, vec3<f32>(0.0), vec3<f32>(cos(a) * 0.22, sin(a) * 0.22, 0.0), 0.007, 86.0);
            }
            k_col = vec3<f32>(0.28, 1.36, 0.88);
        } else if (f_use == 2u) {
            for (var k = 0; k < 3; k = k + 1) {
                core += cl_bar(p, vec3<f32>(-0.22, -0.18 + f32(k) * 0.18, 0.0), vec3<f32>(0.22, -0.18 + f32(k) * 0.18, 0.0), 0.009, 82.0);
            }
            k_col = vec3<f32>(1.32, 0.92, 0.34);
        } else if (f_use == 3u) {
            core = cl_ring_xy(p, 0.18, 0.009, 82.0) + cl_ring_xz(p, 0.24, 0.007, 82.0);
        } else if (f_use == 4u) {
            core = cl_spot(p, vec3<f32>(0.0), 14.0);
            for (var k = 0; k < 5; k = k + 1) {
                let a = f32(k) * TAU / 5.0;
                core += cl_bar(p, vec3<f32>(cos(a) * 0.08, sin(a) * 0.08, 0.0), vec3<f32>(cos(a) * 0.30, sin(a) * 0.30, 0.0), 0.007, 86.0);
            }
            k_col = vec3<f32>(1.42, 0.44, 0.22);
        } else {
            core = doc_line_glow(sd_hex_prism(doc_rot_y(p, time * 0.4), 0.18, 0.18), 78.0);
            k_col = vec3<f32>(0.92, 0.56, 1.44);
        }
        col = vec3<f32>(0.36, 0.96, 1.42) * shell + k_col * core
            + zone_col * cl_feature_beacon(p, feat, 0.44, 0.04, time) * 0.65;
        dens = shell * 0.78 + core * 0.84;
    } else if (z_idx == 17u) {
        // Z17 decision-to-respond: Earth Reply Console & Council Ring (CRITICAL: F0 REMAINS COMPLETELY SILENT, NO SKY LASER!)
        let pd = doc_rot_x(p - vec3<f32>(0.0, -0.20, 0.0), 0.42);
        let dish_edge = exp(-pow(abs(ch_parabolic_dish_sdf(pd, 0.38, 0.19)) * 78.0, 2.0));
        let tripod = cl_bar(pd, vec3<f32>(0.24, 0.0, 0.0), vec3<f32>(0.0, 0.28, 0.0), 0.008, 90.0)
                   + cl_bar(pd, vec3<f32>(-0.12, 0.0, 0.21), vec3<f32>(0.0, 0.28, 0.0), 0.008, 90.0)
                   + cl_bar(pd, vec3<f32>(-0.12, 0.0, -0.21), vec3<f32>(0.0, 0.28, 0.0), 0.008, 90.0);
        let council_ring = cl_ring_xz(p - vec3<f32>(0.0, -0.28, 0.0), 0.42, 0.007, 82.0);
        var reply = 0.0;
        var r_col = vec3<f32>(0.28, 1.28, 1.44);
        var dish_tint = vec3<f32>(0.76, 1.04, 1.38);
        if (f_use == 0u) {
            // F0: Remain silent — quiet indigo/steel dish + calm protective listening veil overhead (ZERO upward laser!)
            dish_tint = vec3<f32>(0.28, 0.46, 0.78);
            reply = cl_ring_xz(p - vec3<f32>(0.0, 0.16, 0.0), 0.36, 0.007, 82.0) * 0.75;
            r_col = vec3<f32>(0.24, 0.56, 0.96);
        } else if (f_use == 1u) {
            // F1: Single brief spherical acknowledgment ping
            reply = cl_ring_xy(p - vec3<f32>(0.0, 0.24, 0.0), 0.14, 0.010, 80.0);
            r_col = vec3<f32>(1.18, 1.28, 1.42);
        } else if (f_use == 2u) {
            // F2: Ascending clean prime & physical-constants rings
            reply = cl_ring_xz(p - vec3<f32>(0.0, 0.18, 0.0), 0.12, 0.008, 84.0)
                  + cl_ring_xz(p - vec3<f32>(0.0, 0.32, 0.0), 0.20, 0.008, 84.0);
            r_col = vec3<f32>(0.28, 1.38, 1.08);
        } else if (f_use == 3u) {
            // F3: Warm cultural mosaic sphere ascending cleanly
            reply = cl_spot(p, vec3<f32>(0.0, 0.24, 0.0), 13.0)
                  + doc_line_glow(sd_hex_prism(doc_rot_y(p - vec3<f32>(0.0, 0.24, 0.0), time * 0.4), 0.16, 0.12), 78.0);
            r_col = vec3<f32>(1.38, 0.94, 0.32);
        } else if (f_use == 4u) {
            // F4: Full high-power directional beacon revealing Earth coordinate
            reply = cl_bar(pd, vec3<f32>(0.0, 0.28, 0.0), vec3<f32>(0.0, 0.76, 0.0), 0.014, 74.0)
                  + cl_ring_xz(p - vec3<f32>(0.0, 0.36, 0.0), 0.22, 0.008, 82.0);
            r_col = vec3<f32>(0.34, 1.44, 0.88);
        } else {
            // F5: Global council deliberation ring + standby ember only (no sky beam)
            for (var k = 0; k < 6; k = k + 1) {
                let a = f32(k) * TAU / 6.0 + time * 0.3;
                reply += cl_spot(p, vec3<f32>(cos(a) * 0.34, 0.14, sin(a) * 0.34), 16.0);
            }
            r_col = vec3<f32>(1.38, 0.82, 0.26);
        }
        col = dish_tint * (dish_edge + tripod * 0.8) + vec3<f32>(0.42, 0.92, 1.38) * council_ring * 0.72
            + r_col * reply
            + zone_col * cl_feature_beacon(p, feat, 0.42, 0.02, time) * 0.65;
        dens = (dish_edge + tripod + council_ring) * 0.74 + reply * 0.86;
    } else if (z_idx == 18u) {
        // Z18 transmission-risks: METI Exposure, Beam Geometry & Unilateral Divergence
        var risk = 0.0;
        var rk_col = vec3<f32>(1.34, 0.68, 0.26);
        if (f_use == 0u) {
            risk = exp(-abs(r - 0.34) * 48.0)
                 + cl_ring_xy(p, 0.36, 0.008, 82.0)
                 + cl_spot(p, vec3<f32>(0.0), 14.0);
            rk_col = vec3<f32>(1.32, 0.56, 0.24);
        } else if (f_use == 1u) {
            risk = cl_bar(p, vec3<f32>(-0.36, 0.0, 0.0), vec3<f32>(0.0, 0.0, 0.0), 0.009, 82.0)
                 + cl_bar(p, vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.36, 0.24, 0.0), 0.009, 82.0)
                 + cl_bar(p, vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.36, -0.24, 0.0), 0.009, 82.0);
            rk_col = vec3<f32>(1.28, 0.92, 0.34);
        } else if (f_use == 2u) {
            risk = cl_bar(p, vec3<f32>(-0.42, -0.24, 0.0), vec3<f32>(0.42, 0.24, 0.0), 0.009, 84.0)
                 + cl_spot(p, vec3<f32>(0.42, 0.24, 0.0), 16.0);
            rk_col = vec3<f32>(0.24, 1.38, 0.82);
        } else if (f_use == 3u) {
            risk = exp(-abs(r - 0.18) * 46.0) + exp(-abs(r - 0.34) * 42.0) + exp(-abs(r - 0.48) * 38.0);
            rk_col = vec3<f32>(1.38, 0.42, 0.26);
        } else if (f_use == 4u) {
            risk = cl_bar(p, vec3<f32>(-0.28, -0.26, 0.0), vec3<f32>(0.18, 0.36, 0.0), 0.008, 84.0)
                 + cl_bar(p, vec3<f32>(0.0, -0.28, 0.0), vec3<f32>(-0.16, 0.36, 0.0), 0.008, 84.0)
                 + cl_bar(p, vec3<f32>(0.28, -0.26, 0.0), vec3<f32>(0.34, 0.36, 0.0), 0.008, 84.0);
            rk_col = vec3<f32>(1.34, 0.48, 0.56);
        } else {
            risk = cl_bar(p, vec3<f32>(0.0, -0.32, 0.0), vec3<f32>(0.0, 0.44, 0.0), 0.015, 72.0)
                 + cl_ring_xz(p - vec3<f32>(0.0, -0.32, 0.0), 0.36, 0.007, 82.0);
            rk_col = vec3<f32>(1.38, 0.94, 0.28);
        }
        col = rk_col * risk + zone_col * cl_feature_beacon(p, feat, 0.42, 0.02, time) * 0.68;
        dens = risk * 0.86;
    } else if (z_idx == 19u) {
        // Z19 light-speed-limit (+companion 3832): Finite-c Light-Cone & Drifting Star Target Lead
        let earth_pt = cl_spot(p, vec3<f32>(-0.34, -0.10, 0.0), 15.0);
        let live_star = cl_spot(p, vec3<f32>(0.30, 0.16, 0.0), 15.0);
        var rel = 0.0;
        var rl_col = vec3<f32>(0.32, 1.24, 1.48);
        if (f_use == 0u) {
            for (var k = 0; k < 4; k = k + 1) {
                rel += cl_ring_xy(p - vec3<f32>(-0.34, -0.10, 0.0), 0.14 + f32(k) * 0.14, 0.008, 80.0);
            }
        } else if (f_use == 1u) {
            rel = cl_ring_xy(p - vec3<f32>(0.30, -0.14, 0.0), 0.08, 0.008, 82.0)
                + cl_bar(p, vec3<f32>(-0.34, -0.10, 0.0), vec3<f32>(0.30, -0.14, 0.0), 0.008, 84.0);
            rl_col = vec3<f32>(1.32, 0.82, 0.34);
        } else if (f_use == 2u) {
            rel = cl_bar(p, vec3<f32>(-0.34, -0.10, 0.0), vec3<f32>(0.30, 0.26, 0.0), 0.009, 82.0)
                + cl_bar(p, vec3<f32>(0.30, -0.10, 0.0), vec3<f32>(0.30, 0.32, 0.0), 0.007, 86.0);
            rl_col = vec3<f32>(0.24, 1.38, 0.92);
        } else if (f_use == 3u) {
            let spread = 0.03 + clamp((p.x + 0.34) / 0.72, 0.0, 1.0) * 0.22;
            rel = exp(-abs(length(p.yz - vec2<f32>(0.02, 0.0)) - spread) * 44.0) * smoothstep(-0.38, 0.38, p.x);
        } else if (f_use == 4u) {
            let dust = pow(max(0.0, snoise3(p * 12.0)), 3.0) * smoothstep(0.24, 0.02, abs(p.x));
            rel = cl_bar(p, vec3<f32>(-0.34, -0.10, 0.0), vec3<f32>(0.30, 0.16, 0.0), 0.008, 82.0) + dust * 1.15;
            rl_col = vec3<f32>(1.18, 0.62, 1.42);
        } else {
            rel = cl_spot(p, vec3<f32>(-0.18, -0.02, 0.0), 17.0) + cl_spot(p, vec3<f32>(0.18, 0.10, 0.0), 17.0);
        }
        col = vec3<f32>(0.24, 0.98, 1.46) * earth_pt
            + vec3<f32>(1.38, 0.94, 0.34) * live_star
            + rl_col * rel
            + zone_col * cl_feature_beacon(p, feat, 0.42, 0.02, time) * 0.65;
        dens = earth_pt * 0.8 + live_star * 0.8 + rel * 0.84;
    } else if (z_idx == 20u) {
        // Z20 interstellar-wait: Generational Archive Torch & Century Hourglass
        let axis = cl_bar(p, vec3<f32>(0.0, -0.38, 0.0), vec3<f32>(0.0, 0.38, 0.0), 0.009, 84.0);
        var wait = 0.0;
        var w_col = vec3<f32>(1.36, 0.96, 0.34);
        for (var k = 0; k < 5; k = k + 1) {
            let yk = -0.30 + f32(k) * 0.15;
            let hi = select(0.55, 1.30, k == i32(f_use % 5u));
            wait += cl_ring_xz(p - vec3<f32>(0.0, yk, 0.0), 0.16 + 0.06 * abs(yk) * 3.0, 0.008, 82.0) * hi;
        }
        if (f_use == 1u) {
            wait += doc_line_glow(sd_hex_prism(doc_rot_y(p, time * 0.35), 0.16, 0.16), 78.0);
        } else if (f_use == 2u) {
            wait += cl_spot(p, vec3<f32>(0.0, -0.22, 0.0), 16.0) + cl_spot(p, vec3<f32>(0.0, 0.22, 0.0), 16.0);
            w_col = vec3<f32>(0.32, 1.24, 1.46);
        } else if (f_use == 3u) {
            wait += cl_ring_xz(p - vec3<f32>(0.0, -0.22, 0.0), 0.36, 0.007, 82.0);
            w_col = vec3<f32>(1.34, 0.54, 0.24);
        } else if (f_use == 4u) {
            wait += cl_spot(p, vec3<f32>(0.0, 0.36, 0.0), 14.0);
            w_col = vec3<f32>(1.18, 0.42, 0.20);
        } else if (f_use == 5u) {
            for (var k = 0; k < 4; k = k + 1) {
                let a = f32(k) * 1.570796;
                wait += cl_bar(p, vec3<f32>(0.0, 0.28, 0.0), vec3<f32>(cos(a) * 0.24, 0.44, sin(a) * 0.24), 0.007, 84.0);
            }
            w_col = vec3<f32>(0.68, 0.72, 1.42);
        }
        col = vec3<f32>(0.42, 0.98, 1.44) * axis + w_col * wait
            + zone_col * cl_feature_beacon(p, feat, 0.40, 0.0, time) * 0.65;
        dens = axis * 0.72 + wait * 0.82;
    } else if (z_idx == 21u) {
        // Z21 first-reply (+companion 3833): Bidirectional Ping & Echo Transformation
        let n_l = cl_spot(p, vec3<f32>(-0.32, 0.0, 0.0), 14.0);
        let n_r = cl_spot(p, vec3<f32>(0.32, 0.0, 0.0), 14.0);
        var echo = cl_bar(p, vec3<f32>(-0.28, 0.08, 0.0), vec3<f32>(0.28, 0.08, 0.0), 0.008, 84.0)
                 + cl_bar(p, vec3<f32>(0.28, -0.08, 0.0), vec3<f32>(-0.28, -0.08, 0.0), 0.008, 84.0);
        var e_col = vec3<f32>(0.28, 1.36, 1.12);
        if (f_use == 1u) {
            echo += cl_ring_xz(p - vec3<f32>(-0.32, 0.0, 0.0), 0.16, 0.008, 82.0);
        } else if (f_use == 2u) {
            echo += cl_ring_xy(p, 0.14, 0.009, 82.0) + cl_spot(p, vec3<f32>(0.0), 16.0);
            e_col = vec3<f32>(1.38, 0.96, 0.30);
        } else if (f_use == 3u) {
            echo += cl_spot(p, vec3<f32>(0.0, -0.08, 0.0), 16.0) + cl_ring_xy(p - vec3<f32>(0.0, -0.08, 0.0), 0.12, 0.008, 82.0);
            e_col = vec3<f32>(1.36, 0.42, 0.24);
        } else if (f_use == 4u) {
            echo += doc_line_glow(sd_hex_prism(doc_rot_y(p, time * 0.35), 0.14, 0.12), 78.0);
            e_col = vec3<f32>(0.42, 0.96, 1.48);
        } else if (f_use == 5u) {
            for (var k = 0; k < 4; k = k + 1) {
                echo += cl_bar(p, vec3<f32>(0.0, -0.08, 0.0), vec3<f32>(-0.32, -0.26 + f32(k) * 0.12, 0.0), 0.007, 86.0);
            }
            e_col = vec3<f32>(0.92, 0.56, 1.44);
        }
        col = vec3<f32>(0.24, 0.98, 1.46) * n_l
            + vec3<f32>(1.38, 0.96, 0.32) * n_r
            + e_col * echo
            + zone_col * cl_feature_beacon(p, feat, 0.40, 0.04, time) * 0.65;
        dens = (n_l + n_r) * 0.8 + echo * 0.84;
    } else if (z_idx == 22u) {
        // Z22 discontinuous-dialogue (+companion 3833): Train of Modular Dialogue Capsules
        let bridge = cl_bar(p, vec3<f32>(-0.44, 0.0, 0.0), vec3<f32>(0.44, 0.0, 0.0), 0.007, 86.0);
        var train = 0.0;
        var tr_col = vec3<f32>(0.26, 1.28, 1.44);
        for (var k = 0; k < 3; k = k + 1) {
            let cx = -0.28 + f32(k) * 0.28;
            let hi = select(0.65, 1.30, k == i32(f_use % 3u));
            train += cl_ring_xy(p - vec3<f32>(cx, 0.0, 0.0), 0.10, 0.009, 80.0) * hi;
            train += cl_spot(p, vec3<f32>(cx, 0.0, 0.0), 18.0) * hi;
        }
        if (f_use == 1u) {
            train += cl_ring_xz(p - vec3<f32>(-0.28, 0.0, 0.0), 0.14, 0.008, 82.0);
            tr_col = vec3<f32>(1.38, 0.96, 0.32);
        } else if (f_use == 2u) {
            train += cl_bar(p, vec3<f32>(0.0, -0.34, 0.0), vec3<f32>(0.0, 0.34, 0.0), 0.009, 82.0);
            tr_col = vec3<f32>(0.24, 1.38, 0.92);
        } else if (f_use == 3u) {
            train += cl_bar(p, vec3<f32>(-0.28, 0.18, 0.0), vec3<f32>(0.28, 0.18, 0.0), 0.008, 84.0)
                   + cl_bar(p, vec3<f32>(-0.28, -0.18, 0.0), vec3<f32>(0.28, -0.18, 0.0), 0.008, 84.0);
        } else if (f_use == 4u) {
            train += cl_ring_xy(p - vec3<f32>(0.0, 0.14, 0.0), 0.16, 0.008, 80.0);
            tr_col = vec3<f32>(1.38, 0.44, 0.22);
        } else if (f_use == 5u) {
            train += doc_line_glow(sd_hex_prism(doc_rot_y(p, time * 0.3), 0.22, 0.18), 76.0);
            tr_col = vec3<f32>(0.92, 0.56, 1.44);
        }
        col = vec3<f32>(0.42, 0.96, 1.44) * bridge + tr_col * train
            + zone_col * cl_feature_beacon(p, feat, 0.42, 0.06, time) * 0.65;
        dens = bridge * 0.72 + train * 0.86;
    } else if (z_idx == 23u) {
        // Z23 knowledge-exchange: Comparative Two-Biosphere Holographic Lab Table
        let table = cl_ring_xz(p - vec3<f32>(0.0, -0.16, 0.0), 0.42, 0.009, 82.0);
        var lab = 0.0;
        var l_col = vec3<f32>(0.26, 1.32, 1.24);
        if (f_use == 0u) {
            lab = cl_bar(p, vec3<f32>(-0.34, -0.16, 0.0), vec3<f32>(0.0, 0.26, 0.0), 0.009, 82.0)
                + cl_bar(p, vec3<f32>(0.34, -0.16, 0.0), vec3<f32>(0.0, 0.26, 0.0), 0.009, 82.0)
                + cl_spot(p, vec3<f32>(0.0, 0.26, 0.0), 15.0);
        } else if (f_use == 1u) {
            lab = cl_ring_xy(p - vec3<f32>(-0.20, 0.10, 0.0), 0.15, 0.009, 82.0)
                + cl_ring_xz(p - vec3<f32>(0.20, 0.10, 0.0), 0.15, 0.009, 82.0);
            l_col = vec3<f32>(0.42, 1.02, 1.48);
        } else if (f_use == 2u) {
            lab = doc_line_glow(sd_hex_prism(doc_rot_y(p - vec3<f32>(0.0, 0.08, 0.0), time * 0.3), 0.20, 0.18), 76.0);
            l_col = vec3<f32>(1.36, 0.82, 0.30);
        } else if (f_use == 3u) {
            let hx = exp(-abs(p.x + 0.18 - 0.07 * sin(p.y * 22.0)) * 34.0) * smoothstep(0.28, 0.04, abs(p.y - 0.08));
            let tr = exp(-abs(p.x - 0.18 - 0.07 * sin(p.y * 15.0)) * 34.0) * smoothstep(0.28, 0.04, abs(p.y - 0.08));
            lab = hx + tr;
            l_col = vec3<f32>(0.24, 1.38, 0.86);
        } else if (f_use == 4u) {
            lab = doc_line_glow(sd_hex_prism(p - vec3<f32>(0.0, 0.08, 0.0), 0.24, 0.14), 76.0)
                + cl_spot(p, vec3<f32>(0.22, 0.22, 0.0), 16.0);
            l_col = vec3<f32>(1.38, 0.56, 0.22);
        } else {
            lab = cl_ring_xy(doc_rot_z(p - vec3<f32>(0.0, 0.08, 0.0), 0.5), 0.22, 0.008, 82.0)
                + cl_ring_xz(doc_rot_x(p - vec3<f32>(0.0, 0.08, 0.0), 0.5), 0.26, 0.008, 82.0);
            l_col = vec3<f32>(0.94, 0.54, 1.44);
        }
        col = vec3<f32>(0.42, 0.96, 1.42) * table + l_col * lab
            + zone_col * cl_feature_beacon(p, feat, 0.42, 0.02, time) * 0.65;
        dens = table * 0.74 + lab * 0.86;
    } else if (z_idx == 24u) {
        // Z24 cosmic-misunderstandings: Semantic Drift & Signal-Corruption Prism
        let l_lens = cl_ring_xy(p - vec3<f32>(-0.24, 0.0, 0.0), 0.16, 0.009, 80.0);
        let r_lens = cl_ring_xy(p - vec3<f32>(0.24, 0.0, 0.0), 0.16, 0.009, 80.0);
        var drift = 0.0;
        var dr_col = vec3<f32>(1.36, 0.52, 0.28);
        if (f_use == 0u) {
            let smooth_w = exp(-abs(p.y - 0.12 * sin(p.x * 16.0)) * 34.0) * smoothstep(0.02, -0.34, p.x);
            let cage_r = exp(-abs(p.y - 0.12 * sign(sin(p.x * 16.0))) * 34.0) * smoothstep(-0.02, 0.34, p.x);
            drift = smooth_w + cage_r;
        } else if (f_use == 1u) {
            drift = cl_bar(p, vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.30, 0.24, 0.0), 0.008, 84.0)
                  + cl_bar(p, vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.30, -0.24, 0.0), 0.008, 84.0);
            dr_col = vec3<f32>(0.88, 0.52, 1.44);
        } else if (f_use == 2u) {
            drift = cl_ring_xy(p - vec3<f32>(-0.24, 0.0, 0.0), 0.10, 0.008, 82.0) * (0.5 + 0.5 * sin(time * 7.0))
                  + cl_ring_xy(p - vec3<f32>(0.24, 0.0, 0.0), 0.22, 0.008, 82.0);
            dr_col = vec3<f32>(0.32, 1.24, 1.44);
        } else if (f_use == 3u) {
            drift = cl_bar(p, vec3<f32>(-0.34, -0.10, 0.0), vec3<f32>(0.34, 0.22, 0.0), 0.010, 80.0);
            dr_col = vec3<f32>(1.42, 0.36, 0.24);
        } else if (f_use == 4u) {
            drift = cl_bar(p, vec3<f32>(-0.36, 0.0, 0.0), vec3<f32>(-0.08, 0.0, 0.0), 0.009, 82.0)
                  + cl_bar(p, vec3<f32>(0.08, 0.0, 0.0), vec3<f32>(0.36, 0.0, 0.0), 0.009, 82.0);
            dr_col = vec3<f32>(1.38, 0.32, 0.48);
        } else {
            drift = doc_line_glow(sd_hex_prism(doc_rot_y(p, time * 0.4), 0.22, 0.16), 76.0);
            dr_col = vec3<f32>(1.38, 0.94, 0.28);
        }
        col = vec3<f32>(0.32, 1.12, 1.44) * l_lens
            + vec3<f32>(1.28, 0.62, 0.34) * r_lens
            + dr_col * drift
            + zone_col * cl_feature_beacon(p, feat, 0.42, 0.02, time) * 0.65;
        dens = (l_lens + r_lens) * 0.78 + drift * 0.86;
    } else if (z_idx == 25u) {
        // Z25 trust (+companion 3834): Reciprocal Verification Trust Lattice Between Twin Spheres
        let orb_l = exp(-abs(length(p - vec3<f32>(-0.30, 0.0, 0.0)) - 0.14) * 54.0);
        let orb_r = exp(-abs(length(p - vec3<f32>(0.30, 0.0, 0.0)) - 0.14) * 54.0);
        var truss = cl_bar(p, vec3<f32>(-0.18, 0.08, 0.0), vec3<f32>(0.18, 0.08, 0.0), 0.008, 84.0)
                  + cl_bar(p, vec3<f32>(-0.18, -0.08, 0.0), vec3<f32>(0.18, -0.08, 0.0), 0.008, 84.0);
        var t_col = vec3<f32>(0.24, 1.36, 0.96);
        if (f_use == 0u) {
            truss += cl_ring_xy(p, 0.14, 0.009, 82.0) + cl_spot(p, vec3<f32>(0.0), 16.0);
            t_col = vec3<f32>(1.38, 0.98, 0.30);
        } else if (f_use == 1u) {
            truss += cl_bar(p, vec3<f32>(-0.18, 0.0, 0.0), vec3<f32>(0.18, 0.0, 0.0), 0.011, 80.0);
        } else if (f_use == 2u) {
            truss += cl_ring_xy(p - vec3<f32>(0.0, 0.08, 0.0), 0.11, 0.009, 82.0);
            t_col = vec3<f32>(1.38, 0.78, 0.26);
        } else if (f_use == 3u) {
            truss += cl_spot(p, vec3<f32>(-0.30, 0.0, 0.0), 18.0) + cl_spot(p, vec3<f32>(0.30, 0.0, 0.0), 18.0);
            t_col = vec3<f32>(0.42, 0.88, 1.44);
        } else if (f_use == 4u) {
            truss = cl_ring_xz(p, 0.28, 0.007, 82.0) * 0.72;
            t_col = vec3<f32>(0.26, 0.86, 1.38);
        } else {
            truss += cl_bar(p, vec3<f32>(-0.18, 0.18, 0.0), vec3<f32>(0.18, 0.18, 0.0), 0.008, 84.0);
            t_col = vec3<f32>(0.28, 1.24, 1.46);
        }
        col = vec3<f32>(0.26, 1.02, 1.46) * orb_l
            + vec3<f32>(1.34, 0.92, 0.34) * orb_r
            + t_col * truss
            + zone_col * cl_feature_beacon(p, feat, 0.42, 0.02, time) * 0.65;
        dens = (orb_l + orb_r) * 0.82 + truss * 0.84;
    } else if (z_idx == 26u) {
        // Z26 interstellar-diplomacy (+companion 3834): Interlocking Cryptographic Treaty Seal Rings
        let seal_a = cl_ring_xy(doc_rot_y(p, 0.25 + time * 0.15), 0.36, 0.009, 82.0);
        let seal_b = cl_ring_xz(doc_rot_x(p, 0.38 - time * 0.12), 0.28, 0.009, 82.0);
        let seal_c = cl_ring_xy(doc_rot_z(p, 0.52 + ff * 0.25), 0.20, 0.009, 84.0);
        var clause = 0.0;
        var cl_col = vec3<f32>(1.38, 0.96, 0.32);
        if (f_use == 0u) {
            clause = cl_spot(p, vec3<f32>(0.0, 0.34, 0.0), 15.0) + cl_ring_xy(p - vec3<f32>(0.0, 0.34, 0.0), 0.09, 0.007, 82.0);
        } else if (f_use == 1u) {
            for (var k = 0; k < 3; k = k + 1) {
                clause += cl_spot(p, vec3<f32>(-0.20 + f32(k) * 0.20, 0.0, 0.0), 17.0);
            }
            cl_col = vec3<f32>(0.24, 1.38, 0.96);
        } else if (f_use == 2u) {
            clause = cl_bar(p, vec3<f32>(-0.34, 0.18, 0.0), vec3<f32>(0.34, 0.18, 0.0), 0.009, 84.0);
        } else if (f_use == 3u) {
            clause = exp(-abs(r - 0.44) * 44.0);
            cl_col = vec3<f32>(0.34, 1.18, 1.48);
        } else if (f_use == 4u) {
            clause = cl_ring_xy(p, 0.14, 0.008, 84.0) + cl_ring_xy(p, 0.26, 0.008, 84.0);
            cl_col = vec3<f32>(0.92, 0.56, 1.44);
        } else {
            clause = cl_ring_xz(p - vec3<f32>(0.0, -0.22, 0.0), 0.34, 0.011, 78.0);
        }
        col = vec3<f32>(0.28, 1.18, 1.46) * (seal_a + seal_b * 0.9 + seal_c)
            + cl_col * clause
            + zone_col * cl_feature_beacon(p, feat, 0.44, 0.0, time) * 0.65;
        dens = (seal_a + seal_b + seal_c) * 0.74 + clause * 0.84;
    } else {
        // Z27 uncertain-coexistence (+companion 3835): Dual-Perspective Luminous Worlds & Light Threads
        let w_left = cl_spot(p, vec3<f32>(-0.30, 0.0, 0.0), 13.0) + exp(-abs(length(p - vec3<f32>(-0.30, 0.0, 0.0)) - 0.16) * 52.0);
        let w_right = cl_spot(p, vec3<f32>(0.30, 0.0, 0.0), 13.0) + exp(-abs(length(p - vec3<f32>(0.30, 0.0, 0.0)) - 0.16) * 52.0);
        var threads = cl_bar(p, vec3<f32>(-0.16, 0.08, 0.0), vec3<f32>(0.16, 0.08, 0.0), 0.007, 86.0)
                    + cl_bar(p, vec3<f32>(-0.16, -0.08, 0.0), vec3<f32>(0.16, -0.08, 0.0), 0.007, 86.0);
        var th_col = vec3<f32>(0.42, 1.24, 1.42);
        if (f_use == 1u) {
            threads += cl_ring_xz(p - vec3<f32>(-0.30, 0.0, 0.0), 0.22, 0.007, 82.0)
                     + cl_ring_xz(p - vec3<f32>(0.30, 0.0, 0.0), 0.22, 0.007, 82.0);
            th_col = vec3<f32>(1.36, 0.94, 0.34);
        } else if (f_use == 2u) {
            threads += cl_ring_xy(p, 0.14, 0.008, 82.0) + exp(-abs(r - 0.46) * 32.0) * 0.45;
        } else if (f_use == 3u) {
            threads += doc_line_glow(sd_hex_prism(p - vec3<f32>(-0.30, 0.0, 0.0), 0.12, 0.12), 78.0)
                     + doc_line_glow(sd_hex_prism(doc_rot_y(p - vec3<f32>(0.30, 0.0, 0.0), 0.5), 0.12, 0.12), 78.0);
            th_col = vec3<f32>(0.26, 1.38, 0.92);
        } else if (f_use == 4u) {
            threads += exp(-abs(length(p - vec3<f32>(-0.30, 0.0, 0.0)) - 0.24) * 44.0)
                     + exp(-abs(length(p - vec3<f32>(0.30, 0.0, 0.0)) - 0.24) * 44.0);
            th_col = vec3<f32>(1.18, 0.72, 1.38);
        } else if (f_use == 5u) {
            threads += cl_ring_xz(p, 0.42, 0.008, 82.0);
            th_col = vec3<f32>(0.78, 1.18, 1.46);
        }
        col = vec3<f32>(0.24, 0.98, 1.46) * w_left
            + vec3<f32>(1.38, 0.94, 0.32) * w_right
            + th_col * threads
            + zone_col * cl_feature_beacon(p, feat, 0.42, 0.04, time) * 0.65;
        dens = (w_left + w_right) * 0.80 + threads * 0.84;
    }

    col *= (0.86 + 0.18 * voice + 0.14 * bass) * feat_hi * cue_p;
    return vec4<f32>(col * env, dens * env);
}

// ============================================================================
// EXP 21 (COSMIC LIFE), EXP 22 (CONTACT), EXP 23 (PERSPECTIVES),
// EXP 24 (MEANING) & EXP 25 (FINAL HORIZON)
// ============================================================================
fn march_doc_horizon_endgame(
    exp_id: u32,
    sub_v: u32,
    p: vec3<f32>,
    scene: i32,
    zone: u32,
    feat: u32,
    cue: f32,
    zone_col: vec3<f32>,
    time: f32,
    voice: f32,
    bass: f32,
    inst: HeroVolumeInstance,
) -> vec4<f32> {
    var col = vec3<f32>(0.0);
    var dens: f32 = 0.0;
    let r = length(p);

    if (exp_id == 21u) {
        let legacy_mode = select(u32(max(scene, 0)), sub_v, sub_v < 10u);
        let z_idx = select(
            select(0u, 12u, legacy_mode == 2u),
            sub_v,
            doc_is_canonical_zone_vid(inst.kind_tags.y)
        );
        return shade_doc_cosmic_life_stage(p, z_idx, feat, inst, zone_col, time, voice, bass);
    } else if (exp_id == 22u) {
        let legacy_mode = select(u32(max(scene, 0)), sub_v, sub_v < 10u);
        let z_idx = select(
            select(0u, 12u, legacy_mode == 1u || legacy_mode == 2u),
            sub_v,
            doc_is_canonical_zone_vid(inst.kind_tags.y)
        );
        return shade_doc_cosmic_contact_stage(p, z_idx, feat, inst, zone_col, time, voice, bass);
    } else if (exp_id == 23u) {
        // EXP 23 — Crisp Face-On 6-Lens Sensory Mandala + Converging Sight Rays
        let active_lens = (u32(max(scene, 0)) + feat + zone) % 6u;
        let center_eye = exp(-pow(r * 22.0, 2.0));
        col += vec3<f32>(1.42, 1.25, 0.85) * center_eye * 0.8;
        dens += center_eye * 0.7;
        for (var k = 0u; k < 6u; k = k + 1u) {
            let a = f32(k) * TAU / 6.0;
            let ctr = vec3<f32>(cos(a) * 0.46, sin(a) * 0.46, 0.0);
            let pl = p - ctr;
            let hex_d = sd_hex_prism(doc_rot_x(pl, 1.570796), 0.17, 0.010);
            let frame_g = exp(-pow(abs(hex_d) * 86.0, 2.0));
            let ring_g = exp(-pow(abs(length(pl.xy) - 0.115) * 82.0, 2.0)) * exp(-pow(pl.z * 38.0, 2.0));
            let interior = smoothstep(0.15, 0.10, length(pl.xy)) * exp(-pow(pl.z * 38.0, 2.0));
            let sight_ray = doc_line_glow(sd_capsule(p, ctr * 0.56, vec3<f32>(0.0), 0.005), 94.0);
            let is_active = select(0.70, 1.55, k == active_lens);
            let lens_c = hsl_to_rgb(f32(k) / 6.0, 0.94, 0.54);
            var pat: f32 = 0.35;
            if (k == 0u) {
                pat = pow(max(sin(length(pl.xy) * 62.0 - time * 4.0), 0.0), 3.0);
            } else if (k == 1u) {
                pat = exp(-pow(abs(length(pl.xy) - 0.07) * 58.0, 2.0));
            } else if (k == 2u) {
                pat = exp(-pow(length(pl.xy) * 18.0, 2.0));
            } else if (k == 3u) {
                pat = pow(max(cos(pl.y * 42.0 + f32(feat)), 0.0), 3.0);
            } else if (k == 4u) {
                pat = exp(-pow(length(fract(pl.xy * 18.0) - 0.5) * 7.0, 2.0));
            } else {
                pat = max(exp(-pow(abs(fract(pl.x * 16.0) - 0.5) * 12.0, 2.0)), exp(-pow(abs(fract(pl.y * 16.0) - 0.5) * 12.0, 2.0)));
            }
            col += lens_c * (frame_g * 1.25 + ring_g * 0.8 + interior * pat + sight_ray * 0.75) * is_active;
            dens += frame_g * 0.9 + ring_g * 0.6 + interior * pat * 0.75 + sight_ray * 0.65;
        }
    } else if (exp_id == 24u) {
        // EXP 24 — COSMIC MEANING: Pale Blue Mote + Astrolabe Shield + Bold Golden Ethical Balance Scales
        let mode = select(u32(max(scene, 0)), sub_v, sub_v < 10u);
        let center_y = -0.14;
        let p_mote = p - vec3<f32>(0.0, center_y, 0.0);
        let rm = length(p_mote);
        let mote = exp(-pow(rm * 20.0, 2.0));
        let shield_a = doc_line_glow(sd_torus(doc_rot_x(p_mote, 0.45), 0.24, 0.008), 88.0);
        let shield_b = doc_line_glow(sd_torus(doc_rot_z(p_mote, 0.55), 0.28, 0.008), 88.0);
        let pillar = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, -0.44, 0.0), vec3<f32>(0.0, 0.34, 0.0), 0.011), 88.0);
        let base_ring = doc_line_glow(sd_torus(p - vec3<f32>(0.0, -0.44, 0.0), 0.18, 0.009), 86.0);
        let tilt = select(0.10 * sin(time * 0.8 + f32(feat) * 0.85), 0.22, mode == 1u);
        let beam_ctr = vec3<f32>(0.0, 0.30, 0.0);
        let p_beam = doc_rot_z(p - beam_ctr, tilt);
        let beam_g = doc_line_glow(sd_capsule(p_beam, vec3<f32>(-0.48, 0.0, 0.0), vec3<f32>(0.48, 0.0, 0.0), 0.011), 90.0);
        let tip_l = beam_ctr + vec3<f32>(-0.48 * cos(tilt), -0.48 * sin(tilt), 0.0);
        let tip_r = beam_ctr + vec3<f32>(0.48 * cos(tilt), 0.48 * sin(tilt), 0.0);
        let pan_l_p = tip_l - vec3<f32>(0.0, 0.32, 0.0);
        let pan_r_p = tip_r - vec3<f32>(0.0, 0.32, 0.0);
        let chains = doc_line_glow(sd_capsule(p, tip_l, pan_l_p + vec3<f32>(-0.10, 0.0, 0.0), 0.005), 92.0)
                   + doc_line_glow(sd_capsule(p, tip_l, pan_l_p + vec3<f32>(0.10, 0.0, 0.0), 0.005), 92.0)
                   + doc_line_glow(sd_capsule(p, tip_r, pan_r_p + vec3<f32>(-0.10, 0.0, 0.0), 0.005), 92.0)
                   + doc_line_glow(sd_capsule(p, tip_r, pan_r_p + vec3<f32>(0.10, 0.0, 0.0), 0.005), 92.0);
        let pan_l = doc_line_glow(sd_torus(p - pan_l_p, 0.14, 0.009), 86.0);
        let pan_r = doc_line_glow(sd_torus(p - pan_r_p, 0.14, 0.009), 86.0);
        let orb_w = exp(-pow(length(p - (pan_l_p + vec3<f32>(0.0, 0.05, 0.0))) * 28.0, 2.0))
                  + exp(-pow(length(p - (pan_r_p + vec3<f32>(0.0, 0.05, 0.0))) * 28.0, 2.0));
        let gold_c = mix(vec3<f32>(1.48, 1.08, 0.36), zone_col * 1.35, 0.22);
        col += vec3<f32>(0.24, 0.96, 1.52) * (mote * 1.2 + shield_a + shield_b * 0.8)
             + gold_c * (pillar + base_ring + beam_g + chains * 0.75 + pan_l + pan_r + orb_w);
        dens += mote + shield_a + shield_b * 0.75 + pillar + base_ring + beam_g + chains * 0.65 + pan_l + pan_r + orb_w;
    } else {
        // EXP 25 — FINAL HORIZON: Nested Horizon Spheres + 25-Petal Lotus + Thermodynamic Fade across z=0..24!
        let horizon_fade = clamp(1.0 - f32(min(zone, 24u)) / 38.0, 0.36, 1.0);
        let slow_rate = clamp(1.0 - f32(min(zone, 24u)) / 30.0, 0.20, 1.0);
        let t_slow = time * slow_rate;

        for (var h = 0; h < 3; h = h + 1) {
            let fh = f32(h);
            let hr = 0.24 + fh * 0.20;
            let ph = doc_rot_x(doc_rot_y(p, fh * 1.1 + f32(feat) * 0.3), fh * 0.78 + t_slow * 0.08);
            let lon = exp(-pow(abs(fract(atan2(ph.z, ph.x) / TAU * 8.0) - 0.5) * 16.0, 2.0));
            let sg = exp(-pow(abs(length(ph) - hr) * 56.0, 2.0)) * lon;
            col += mix(vec3<f32>(1.45, 1.08, 0.42), vec3<f32>(0.45, 0.92, 1.48), fh * 0.5) * sg * horizon_fade;
            dens += sg * 0.75 * horizon_fade;
        }
        let r_xz = length(p.xz);
        let phi = atan2(p.z, p.x);
        let petal_r = 0.48 + 0.16 * pow(abs(cos(phi * 12.5 + t_slow * 0.2)), 2.5);
        let lotus_g = exp(-pow(abs(r_xz - petal_r) * 48.0, 2.0)) * exp(-pow(p.y * 26.0, 2.0));
        let lotus_c = hsl_to_rgb(fract(phi / TAU + 1.0), 0.92, 0.56);
        let petal_hi = 1.0 + 0.35 * exp(-pow((fract(phi / TAU + 1.0) - f32(zone % 25u) / 25.0) * 10.0, 2.0));
        col += lotus_c * lotus_g * (1.05 + 0.25 * voice + 0.20 * bass) * horizon_fade * petal_hi;
        dens += lotus_g * 0.95 * horizon_fade;
    }

    return vec4<f32>(col, dens);
}

// ============================================================================
// UNIFIED DISPATCHER CALLED BY hero_volumetric.wgsl
// ============================================================================
fn raymarch_doc_civilization_horizon(
    vid: u32,
    ro: vec3<f32>,
    rd: vec3<f32>,
    t0: f32,
    t1: f32,
    jitter: f32,
    fovea_k: f32,
    inst: HeroVolumeInstance,
    time: f32,
    voice: f32,
    bass: f32,
) -> vec4<f32> {
    let scene = doc_scene(inst);
    let zone = doc_zone(inst);
    let feat = doc_feat(inst);
    let cue = doc_cue(inst);
    let zone_col = max(inst.color_b.rgb, vec3<f32>(0.15));
    let pair = ch_exp_and_sub(vid);
    let exp_id = pair.x;
    let sub_v = pair.y;

    let base_steps = select(42.0, 26.0, doc_is_canonical_zone_vid(vid) || exp_id == 21u || exp_id == 22u);
    let cap_steps = select(46.0, 28.0, doc_is_canonical_zone_vid(vid) || exp_id == 21u || exp_id == 22u);
    let steps: i32 = i32(clamp(base_steps * fovea_k, 18.0, cap_steps));
    let dt = (t1 - t0) / f32(steps);
    var t = t0 + dt * jitter;
    var accum = vec3<f32>(0.0);
    var trans: f32 = 1.0;

    let front_locked = (exp_id == 19u) || (exp_id == 20u && sub_v != 2u) || (exp_id == 21u) || (exp_id == 22u) || (exp_id == 23u) || (exp_id == 24u);
    let rot_y_ang = select(time * 0.12, sin(time * 0.35) * 0.12, front_locked);

    for (var i = 0; i < 46; i = i + 1) {
        if (i >= steps || trans < 0.03) { break; }
        let p_raw = ro + rd * t;
        if (length(p_raw) < 0.94) {
            let p = doc_rot_y(p_raw, rot_y_ang);
            var s = vec4<f32>(0.0);
            if (exp_id == 11u || exp_id == 12u) {
                s = march_doc_evidence_future(exp_id, sub_v, p, scene, zone, feat, cue, zone_col, time, voice, bass);
            } else if (exp_id >= 18u && exp_id <= 20u) {
                s = march_doc_civilization(exp_id, sub_v, p, scene, zone, feat, cue, zone_col, time, voice, bass);
            } else {
                s = march_doc_horizon_endgame(exp_id, sub_v, p, scene, zone, feat, cue, zone_col, time, voice, bass, inst);
            }
            let alpha_s = clamp(s.a * dt * 3.5, 0.0, 0.74);
            accum += trans * s.rgb * (dt * 3.5);
            trans *= (1.0 - alpha_s);
        }
        t += dt;
    }
    if (doc_is_canonical_zone_vid(vid) || exp_id == 21u) {
        let peak = max(accum.r, max(accum.g, accum.b));
        if (peak > 0.88) {
            let sc = (0.88 + 0.18 * tanh((peak - 0.88) / 0.45)) / max(peak, 1e-4);
            accum *= sc;
        }
    }
    return vec4<f32>(accum, 1.0 - trans);
}
