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
) -> vec4<f32> {
    var col = vec3<f32>(0.0);
    var dens: f32 = 0.0;
    let r = length(p);

    if (exp_id == 21u) {
        let mode = select(u32(max(scene, 0)), sub_v, sub_v < 10u);
        if (mode == 2u) {
            // Crisp Close-Up Cracked Europa Ice Crust + Hydrothermal Cyan Lineae Fissures + Cryo-Plumes
            let pe = p - vec3<f32>(0.0, -0.10, 0.0);
            let re = length(pe);
            let ice_shell = exp(-pow(abs(re - 0.38) * 76.0, 2.0));
            let n_dir = normalize(pe + vec3<f32>(1e-4));
            let ice_rim = 0.28 + 0.85 * max(dot(n_dir, normalize(vec3<f32>(0.45, 0.70, 0.55))), 0.0);
            let crack_val = sin(pe.x * 18.0 + pe.y * 12.0) + cos(pe.z * 16.0 - pe.x * 9.0);
            let lineae = exp(-pow(abs(crack_val) * 10.0, 2.0)) * ice_shell;
            let plume_a = exp(-pow(length(pe.xz) * 24.0, 2.0)) * smoothstep(0.35, 0.42, pe.y) * smoothstep(0.84, 0.46, pe.y);
            let plume_b = exp(-pow(length(pe.xz - vec2<f32>(0.16, 0.08)) * 30.0, 2.0)) * smoothstep(0.32, 0.40, pe.y) * smoothstep(0.70, 0.44, pe.y);
            col += vec3<f32>(0.78, 0.95, 1.25) * ice_shell * ice_rim * 0.65
                 + vec3<f32>(0.22, 1.42, 1.55) * (lineae * 1.25 + (plume_a + plume_b) * 1.15);
            dens += ice_shell * 0.65 + lineae * 0.9 + (plume_a + plume_b) * 0.95;
        } else {
            let star = exp(-pow(r * 16.0, 2.0));
            let inner_hz = doc_line_glow(sd_torus(p, 0.34, 0.006), 88.0);
            let outer_hz = doc_line_glow(sd_torus(p, 0.54, 0.006), 88.0);
            let orb_ang = time * 0.8 + f32(feat) * 0.45;
            let exo_p = vec3<f32>(cos(orb_ang) * 0.44, 0.0, sin(orb_ang) * 0.44);
            let exo_g = exp(-pow(length(p - exo_p) * 24.0, 2.0));
            col += vec3<f32>(1.48, 1.05, 0.35) * star + vec3<f32>(0.22, 1.35, 0.72) * (inner_hz + outer_hz) + vec3<f32>(0.35, 1.15, 1.48) * exo_g;
            dens += star + (inner_hz + outer_hz) * 0.75 + exo_g;
        }
    } else if (exp_id == 22u) {
        // Disambiguate App 22 shared scene indices using BOTH zone and scene, tinted cleanly with zone_col
        let mode = select(u32(max(scene, 0)), sub_v, sub_v < 10u);
        let odd_zone = (zone % 2u) == 1u;
        let pulse_phase = select(0.0, 1.8, odd_zone);
        if (mode == 1u || mode == 2u) {
            // Tilted Face-On Prime Number Concentric Beacons (2, 3, 5, 7, 11) + Binary Pulsar Rays
            let pm = doc_rot_x(p, 0.42);
            let p1 = doc_line_glow(sd_torus(pm, 0.12, 0.007), 92.0);
            let p2 = doc_line_glow(sd_torus(pm, 0.19, 0.007), 92.0);
            let p3 = doc_line_glow(sd_torus(pm, 0.31, 0.007), 92.0);
            let p4 = doc_line_glow(sd_torus(pm, 0.44, 0.007), 92.0);
            let p5 = doc_line_glow(sd_torus(pm, 0.62, 0.007), 92.0);
            let ring_sum = p1 + p2 + p3 + p4 + p5;
            let ang_pm = atan2(pm.z, pm.x);
            let bin_code = 0.55 + 0.55 * step(0.15, sin(ang_pm * 13.0 + length(pm.xz) * 18.0));
            let pulse = 0.75 + 0.45 * sin(r * 16.0 - time * 3.5 + pulse_phase);
            let pulsar_rays = exp(-pow(abs(pm.y) * 44.0, 2.0)) * exp(-pow(min(abs(pm.x), abs(pm.z)) * 46.0, 2.0)) * smoothstep(0.68, 0.16, length(pm.xz));
            let tint = mix(vec3<f32>(0.32, 1.38, 1.48), zone_col * 1.45, select(0.18, 0.62, odd_zone));
            col += tint * ring_sum * pulse * bin_code + vec3<f32>(1.45, 1.05, 0.35) * pulsar_rays;
            dens += ring_sum * 0.92 + pulsar_rays * 0.8;
        } else {
            // Crisp Parabolic SETI Dish + Tripod Feed Horn + Coherent Laser Beacon Pulse
            let pd = doc_rot_x(doc_rot_y(p - vec3<f32>(0.0, -0.18, 0.0), f32(zone) * 0.25), 0.42);
            let d_dish = ch_parabolic_dish_sdf(pd, 0.44, 0.22);
            let dish_edge = exp(-pow(abs(d_dish) * 78.0, 2.0));
            let grid_s = max(
                exp(-pow(abs(fract(atan2(pd.z, pd.x) / TAU * 12.0) - 0.5) * 18.0, 2.0)),
                exp(-pow(abs(fract(length(pd.xz) * 10.0) - 0.5) * 18.0, 2.0))
            );
            let tripod = doc_line_glow(sd_capsule(pd, vec3<f32>(0.28, 0.0, 0.0), vec3<f32>(0.0, 0.34, 0.0), 0.008), 90.0)
                       + doc_line_glow(sd_capsule(pd, vec3<f32>(-0.14, 0.0, 0.24), vec3<f32>(0.0, 0.34, 0.0), 0.008), 90.0)
                       + doc_line_glow(sd_capsule(pd, vec3<f32>(-0.14, 0.0, -0.24), vec3<f32>(0.0, 0.34, 0.0), 0.008), 90.0);
            let beam_core = doc_line_glow(sd_capsule(pd, vec3<f32>(0.0, 0.34, 0.0), vec3<f32>(0.0, 0.88, 0.0), 0.012), 78.0);
            let beam_wave = 0.45 + 0.85 * pow(max(sin(pd.y * 28.0 - time * 5.0 + pulse_phase), 0.0), 4.0);
            let beam_c = mix(vec3<f32>(0.24, 1.48, 0.95), zone_col * 1.45, select(0.15, 0.60, odd_zone));
            col += vec3<f32>(0.85, 1.15, 1.45) * dish_edge * (0.28 + 0.88 * grid_s)
                 + vec3<f32>(1.45, 0.95, 0.34) * tripod
                 + beam_c * beam_core * beam_wave;
            dens += dish_edge * 0.88 + tripod * 0.8 + beam_core * beam_wave;
        }
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

    let steps: i32 = i32(clamp(42.0 * fovea_k, 24.0, 46.0));
    let dt = (t1 - t0) / f32(steps);
    var t = t0 + dt * jitter;
    var accum = vec3<f32>(0.0);
    var trans: f32 = 1.0;

    let front_locked = (exp_id == 19u) || (exp_id == 20u && sub_v != 2u) || (exp_id == 23u) || (exp_id == 24u);
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
                s = march_doc_horizon_endgame(exp_id, sub_v, p, scene, zone, feat, cue, zone_col, time, voice, bass);
            }
            let alpha_s = clamp(s.a * dt * 3.5, 0.0, 0.74);
            accum += trans * s.rgb * (dt * 3.5);
            trans *= (1.0 - alpha_s);
        }
        t += dt;
    }
    return vec4<f32>(accum, 1.0 - trans);
}
