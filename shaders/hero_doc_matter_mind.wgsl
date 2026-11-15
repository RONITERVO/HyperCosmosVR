// ============================================================================
// HyperCosmosVR — Documentary Hero Shaders: Colors, Music, Matter, Spacetime,
//                 Complexity, Life & Mind (Experiences 01, 02, 13..17)
// Pure OLED #000000 void, razor-sharp SDF geometry & physically grounded forms.
// Concatenated after common_oled.wgsl, doc_common.wgsl, and hero_doc_cosmos.wgsl.
// ============================================================================

fn mm_sd_tri_prism_z(p: vec3<f32>, h: vec2<f32>) -> f32 {
    let q = abs(p);
    return max(q.z - h.y, max(q.x * 0.8660254 + p.y * 0.5, -p.y) - h.x * 0.5);
}

fn mm_spectral_wavelength(t_in: f32) -> vec3<f32> {
    // 0.0 -> 380nm violet .. 1.0 -> 720nm crimson
    let t = clamp(t_in, 0.0, 1.0);
    let r = smoothstep(0.45, 0.72, t) + 0.35 * (1.0 - smoothstep(0.0, 0.18, t));
    let g = smoothstep(0.16, 0.42, t) * (1.0 - smoothstep(0.68, 0.88, t));
    let b = smoothstep(0.0, 0.12, t) * (1.0 - smoothstep(0.36, 0.55, t));
    return clamp(vec3<f32>(r, g, b) * 1.15, vec3<f32>(0.0), vec3<f32>(1.3));
}

// ============================================================================
// EXP 01 — COLORS (Prism, Spectral Fan, Photoreceptors, 3D HSL Bicone,
//                 Quantum Absorption, Tetrachromacy Crown)
// ============================================================================
fn march_doc_colors(
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
    let cue = doc_cue(inst);
    let weight = doc_weight(inst);
    let submode = doc_submode(inst);
    // inst.phys_b.xyz: interactive Hue (0..1 or 0..360), Sat, Light
    let raw_h = inst.phys_b.x;
    let lock_h = select(raw_h, fract(raw_h / 360.0), raw_h > 1.01);
    let lock_hsl = vec3<f32>(clamp(lock_h, 0.0, 1.0), inst.phys_b.y, inst.phys_b.z);

    let show_bicone = (vid == 10u) || (scene == 2);
    let show_cones = (vid != 10u) && (scene == 1);
    let show_quantum = (vid != 10u) && (scene == 3);
    let show_tetra = (vid != 10u) && (scene >= 4);

    let steps: i32 = i32(clamp(42.0 * fovea_k, 24.0, 46.0));
    let dt = (t1 - t0) / f32(steps);
    var t = t0 + dt * jitter;
    var accum = vec3<f32>(0.0);
    var trans: f32 = 1.0;

    for (var i = 0; i < 46; i = i + 1) {
        if (i >= steps || trans < 0.03) { break; }
        let p_raw = ro + rd * t;
        let r_sph = length(p_raw);
        if (r_sph < 0.94) {
            let spin_angle = time * 0.16;
            let p = select(p_raw, doc_rot_y(p_raw, spin_angle), show_bicone || show_quantum || show_tetra);
            var col = vec3<f32>(0.0);
            var dens: f32 = 0.0;

            if (show_bicone) {
                // Sculpted 3D HSL Bicone wireframe + iridescent hue equator
                let r_xz = length(p.xz);
                let cone_r = max(0.02, 0.56 * (1.0 - abs(p.y) / 0.58));
                let shell_d = abs(r_xz - cone_r);
                if (abs(p.y) < 0.58 && shell_d < 0.045) {
                    let hue = fract(atan2(p.z, p.x) / TAU + 1.0);
                    let lit = clamp(p.y / 1.16 + 0.5, 0.08, 0.92);
                    let sat = clamp(r_xz / max(cone_r, 0.05), 0.15, 1.0);
                    let hue_col = hsl_to_rgb(hue, sat, lit);
                    let lat_line = exp(-pow(abs(fract(p.y * 6.0 + 0.5) - 0.5) * 14.0, 2.0));
                    let lon_line = exp(-pow(abs(fract(hue * 12.0) - 0.5) * 16.0, 2.0));
                    let eq_ring = exp(-pow(abs(p.y) * 32.0, 2.0));
                    let wire = max(eq_ring, 0.65 * max(lat_line, lon_line));
                    let shell = exp(-pow(shell_d * 58.0, 2.0));
                    col += hue_col * (shell * (0.25 + 1.05 * wire));
                    dens += shell * (0.28 + 0.85 * wire);
                }
                let spine_d = sd_capsule(p, vec3<f32>(0.0, -0.56, 0.0), vec3<f32>(0.0, 0.56, 0.0), 0.008);
                let spine_g = doc_line_glow(spine_d, 68.0);
                let spine_lum = clamp(p.y / 1.12 + 0.5, 0.04, 1.0);
                col += vec3<f32>(spine_lum) * spine_g * 0.9;
                dens += spine_g * 0.5;

                // Live interactive target HSL lock bead when active
                if (submode > 0u || weight > 0.05) {
                    let ang = lock_hsl.x * TAU;
                    let ty = clamp((lock_hsl.z - 0.5) * 1.08, -0.52, 0.52);
                    let tr = 0.54 * (1.0 - abs(ty) / 0.58) * clamp(lock_hsl.y, 0.25, 1.0);
                    let target_p = vec3<f32>(cos(ang) * tr, ty, sin(ang) * tr);
                    let d_tgt = length(p - target_p);
                    let halo = exp(-pow(abs(d_tgt - 0.075) * 70.0, 2.0)) + 1.45 * exp(-pow(d_tgt * 32.0, 2.0));
                    col += hsl_to_rgb(lock_hsl.x, max(lock_hsl.y, 0.6), max(lock_hsl.z, 0.55)) * halo * (1.0 + 0.35 * cue);
                    dens += halo * 0.85;
                }
            } else if (show_cones) {
                // L/M/S Trichromatic photoreceptor sensitivity curves
                for (var c = 0; c < 3; c = c + 1) {
                    let fc = f32(c);
                    let z_off = (fc - 1.0) * 0.22;
                    let peak_x = select(-0.30, select(0.04, 0.26, c == 2), c == 1);
                    let width = select(0.16, 0.19, c >= 1);
                    let arch_y = -0.30 + 0.68 * exp(-pow((p.x - peak_x) / width, 2.0)) * (1.0 + 0.14 * voice);
                    let d_curve = length(vec2<f32>(p.y - arch_y, p.z - z_off));
                    let glow = exp(-pow(d_curve * 48.0, 2.0));
                    let c_rgb = select(vec3<f32>(0.18, 0.42, 1.45), select(vec3<f32>(0.16, 1.28, 0.42), vec3<f32>(1.42, 0.18, 0.18), c == 2), c == 1);
                    col += c_rgb * glow;
                    dens += glow * 0.85;
                }
                let d_axis = sd_capsule(p, vec3<f32>(-0.68, -0.30, 0.0), vec3<f32>(0.68, -0.30, 0.0), 0.006);
                let g_axis = doc_line_glow(d_axis, 72.0);
                col += mm_spectral_wavelength((p.x + 0.68) / 1.36) * g_axis;
                dens += g_axis * 0.5;
            } else if (show_quantum) {
                // Bohr-Schrodinger discrete orbital spectral rings
                for (var ring = 1; ring <= 4; ring = ring + 1) {
                    let fr = f32(ring) * 0.165;
                    let d_r = sd_torus(p, fr, 0.005);
                    let g_r = doc_line_glow(d_r, 64.0);
                    let r_col = mm_spectral_wavelength(f32(ring - 1) / 3.0);
                    col += r_col * g_r * (0.7 + 0.4 * sin(time * 2.2 + f32(ring)));
                    dens += g_r * 0.7;
                }
                let nuc = exp(-pow(length(p) * 18.0, 2.0));
                col += vec3<f32>(1.25, 0.95, 0.65) * nuc;
                dens += nuc;
            } else if (show_tetra) {
                // Tetrachromacy / 12-channel mantis spectral resonance crown
                let a = atan2(p.z, p.x);
                let sector = fract(a / TAU + 1.0);
                let petal = pow(max(0.0, cos(a * 6.0 + time * 0.7)), 3.0);
                let d_crown = abs(length(p.xz) - (0.38 + 0.16 * petal));
                let ring_g = exp(-pow(d_crown * 52.0, 2.0)) * exp(-pow(p.y * 14.0, 2.0));
                let crown_col = mm_spectral_wavelength(sector);
                col += crown_col * ring_g * (0.95 + 0.45 * voice + 0.25 * bass);
                dens += ring_g * 0.9;
                let inner_core = exp(-pow(abs(length(p) - 0.18) * 35.0, 2.0));
                col += iridescence_spectral(dot(normalize(p + vec3<f32>(1e-4)), -rd), time * 0.2, voice * 0.15) * inner_core;
                dens += inner_core * 0.7;
            } else {
                // Default / Scene 0: Crisp Glass Dispersion Prism + 7-Ray Rainbow Fan (zero surface stipple)
                let pr = doc_rot_z(p - vec3<f32>(-0.08, 0.0, 0.0), -0.08);
                let d_prism = mm_sd_tri_prism_z(pr, vec2<f32>(0.54, 0.24));
                let prism_edge = exp(-pow(abs(d_prism) * 92.0, 2.0));
                let rim_sheen = smoothstep(0.015, -0.03, d_prism) * smoothstep(-0.14, -0.005, d_prism) * 0.045;
                col += vec3<f32>(0.68, 0.92, 1.25) * (prism_edge * 1.25 + rim_sheen);
                dens += prism_edge * 0.95 + rim_sheen * 0.35;

                // Collimated white beam entering left face
                let beam_in = sd_capsule(p, vec3<f32>(-0.80, -0.05, 0.0), vec3<f32>(-0.20, 0.02, 0.0), 0.007);
                let g_in = doc_line_glow(beam_in, 96.0);
                let wave_pulse = 0.85 + 0.25 * sin(p.x * 26.0 - time * 5.0);
                col += vec3<f32>(1.28, 1.22, 1.15) * g_in * wave_pulse;
                dens += g_in * 0.92;

                // 7 crisp refracted spectral rays fanning from right prism face into pure OLED black void
                for (var s = 0; s < 7; s = s + 1) {
                    let fs = f32(s) / 6.0;
                    let y_start = mix(-0.04, 0.06, fs);
                    let y_end = mix(-0.46, 0.40, fs);
                    let seg = sd_capsule(p, vec3<f32>(0.05, y_start, 0.0), vec3<f32>(0.85, y_end, 0.0), 0.005);
                    let gs = doc_line_glow(seg, 98.0);
                    let s_col = mm_spectral_wavelength(1.0 - fs);
                    let highlight = 1.0 + 0.55 * cue * exp(-pow((fs - lock_hsl.x) * 6.0, 2.0));
                    col += s_col * gs * highlight * 1.15;
                    dens += gs * 0.88;
                }
            }

            let alpha_s = clamp(dens * dt * 3.4, 0.0, 0.72);
            accum += trans * col * (dt * 3.4);
            trans *= (1.0 - alpha_s);
        }
        t += dt;
    }
    return vec4<f32>(accum, 1.0 - trans);
}

// ============================================================================
// EXP 02 — MUSIC (Chladni Plate, 48-String Plucked Harp, 4 Timbre Chambers)
// ============================================================================
fn march_doc_music(
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
    let cue = doc_cue(inst);
    let submode = doc_submode(inst);
    // inst.phys_b.x: MIDI 48..95 (from cosmos_narration) or normalized [0..1]
    let raw_pitch = inst.phys_b.x;
    let active_pitch = clamp(select(raw_pitch, (raw_pitch - 48.0) / 47.0, raw_pitch > 1.01), 0.0, 1.0);
    let pitch_active = step(0.01, inst.phys_b.z + cue);

    let steps: i32 = i32(clamp(42.0 * fovea_k, 24.0, 46.0));
    let dt = (t1 - t0) / f32(steps);
    var t = t0 + dt * jitter;
    var accum = vec3<f32>(0.0);
    var trans: f32 = 1.0;

    let n_mode = 2.0 + f32((u32(max(scene, 0)) + submode) % 4u);
    let m_mode = 3.0 + f32((doc_zone(inst) + 1u) % 4u);

    for (var i = 0; i < 46; i = i + 1) {
        if (i >= steps || trans < 0.03) { break; }
        let p_raw = ro + rd * t;
        if (length(p_raw) < 0.94) {
            let p = doc_rot_y(p_raw, time * 0.14);
            var col = vec3<f32>(0.0);
            var dens: f32 = 0.0;

            if (vid == 11u) {
                // Chamber 1: Cristal — octahedral chime lattice + shimmering FM partials
                let p_rot = doc_rot_x(p, 0.4);
                let d_oct = sd_octahedron(p_rot, 0.44);
                let edge_g = exp(-pow(abs(d_oct) * 58.0, 2.0));
                let chime_wave = 0.7 + 0.5 * sin(length(p) * 32.0 - time * 6.5);
                col += vec3<f32>(0.65, 1.15, 1.45) * edge_g * chime_wave;
                dens += edge_g * 0.9;
                let inner_ring = doc_line_glow(sd_torus(p, 0.54, 0.006), 68.0);
                col += vec3<f32>(0.95, 1.25, 1.50) * inner_ring;
                dens += inner_ring * 0.6;
            } else if (vid == 12u) {
                // Chamber 2: Aire — vertical pan-flute organ pipes + helical breath standing wave
                for (var k = 0; k < 5; k = k + 1) {
                    let fk = f32(k) - 2.0;
                    let pipe_h = 0.42 - fk * 0.055;
                    let d_pipe = sd_cylinder_y(p - vec3<f32>(fk * 0.16, -0.08, 0.0), 0.028, pipe_h);
                    let gp = exp(-pow(abs(d_pipe) * 52.0, 2.0));
                    col += vec3<f32>(0.35, 0.92, 1.38) * gp;
                    dens += gp * 0.75;
                }
                let breath_r = 0.24 + 0.14 * sin(p.y * 9.0 - time * 4.0);
                let swirl = exp(-pow(abs(length(p.xz) - breath_r) * 36.0, 2.0)) * smoothstep(0.58, 0.2, abs(p.y));
                col += vec3<f32>(0.55, 1.20, 1.35) * swirl * (0.6 + 0.55 * voice);
                dens += swirl * 0.6;
            } else if (vid == 13u) {
                // Chamber 3: Calido — warm acoustic cedar/amber violin soundbox ribs + strings
                let d_box = sd_torus(vec3<f32>(p.x * 1.1, p.z, p.y * 0.78), 0.36, 0.09);
                let rim = exp(-pow(abs(d_box) * 50.0, 2.0));
                col += vec3<f32>(1.38, 0.68, 0.20) * rim;
                dens += rim * 0.85;
                let str_x = fract(p.x * 8.0 + 0.5) - 0.5;
                let str_g = exp(-pow(abs(str_x) * 38.0, 2.0)) * exp(-pow(abs(p.z) * 45.0, 2.0)) * smoothstep(0.45, 0.1, abs(p.y));
                col += vec3<f32>(1.45, 1.05, 0.42) * str_g * (0.8 + 0.45 * voice);
                dens += str_g * 0.7;
            } else if (vid == 14u) {
                // Chamber 4: Profundo — concentric sub-bass bronze gong rings
                for (var g = 0; g < 3; g = g + 1) {
                    let fg = f32(g);
                    let gr = 0.20 + fg * 0.17;
                    let ring_d = sd_torus(vec3<f32>(p.x, p.z, p.y - (fg - 1.0) * 0.08), gr, 0.012);
                    let gg = doc_line_glow(ring_d, 56.0);
                    let gong_pulse = 0.75 + 0.55 * bass * sin(time * 3.0 - fg * 1.2);
                    col += vec3<f32>(0.72, 0.38, 1.35) * gg * gong_pulse;
                    dens += gg * 0.85;
                }
            } else {
                // Main Exp 02 Hero: Equatorial Chladni Nodal Plate + Floating 48-String Chromatic Harp
                let r_xz = length(p.xz);
                let plate_d = max(r_xz - 0.62, abs(p.y + 0.26) - 0.018);
                if (abs(plate_d) < 0.055) {
                    let cx = p.x * 1.6;
                    let cz = p.z * 1.6;
                    let field_s = doc_sample_field_grid(vec2<f32>(p.x * 0.72 + 0.5, p.z * 0.72 + 0.5));
                    let wave_mod = clamp(field_s.r * 0.28 + field_s.a * 0.16, -0.25, 0.35);
                    let chladni = cos(n_mode * PI * cx) * cos(m_mode * PI * cz)
                                - cos(m_mode * PI * cx) * cos(n_mode * PI * cz) + wave_mod * 0.45;
                    let nodal = exp(-pow(abs(chladni) * 14.0, 2.0));
                    let plate_mask = smoothstep(0.055, 0.0, abs(p.y + 0.26)) * smoothstep(0.64, 0.56, r_xz);
                    let rim = exp(-pow(abs(r_xz - 0.62) * 58.0, 2.0)) * smoothstep(0.04, 0.0, abs(p.y + 0.26));
                    let ch_base = mix(vec3<f32>(0.22, 0.78, 1.25), vec3<f32>(1.45, 1.02, 0.35), nodal);
                    let ch_col = mix(ch_base, max(field_s.rgb, ch_base), clamp(field_s.a * 0.28, 0.0, 0.32));
                    col += ch_col * (nodal * plate_mask * 1.15 + rim * 0.9);
                    dens += (nodal * plate_mask + rim) * 0.95;
                }

                // 48-string Chromatic Harp arching cleanly above the Chladni plate (y in [-0.22, 0.56])
                if (p.y > -0.24 && p.y < 0.60 && abs(p.z) < 0.14 && abs(p.x) < 0.58) {
                    let s_norm = clamp((p.x + 0.54) / 1.08, 0.0, 1.0);
                    let s_cell = fract(s_norm * 24.0 + 0.5) - 0.5;
                    let top_arch = 0.22 + 0.30 * (1.0 - s_norm * 0.72) + 0.05 * sin(s_norm * PI);
                    let vib_amp = 0.012 + 0.03 * voice + 0.04 * pitch_active * exp(-pow((s_norm - active_pitch) * 10.0, 2.0));
                    let vib_z = vib_amp * sin((p.y + 0.22) * PI / max(top_arch + 0.22, 0.2)) * sin(time * 22.0 + s_norm * 19.0);
                    let string_d = length(vec2<f32>(s_cell / 24.0 * 1.08, p.z - vib_z));
                    let str_mask = smoothstep(-0.24, -0.20, p.y) * smoothstep(top_arch + 0.02, top_arch - 0.01, p.y);
                    let str_g = exp(-pow(string_d * 115.0, 2.0)) * str_mask;
                    let active_boost = 1.0 + 1.4 * pitch_active * exp(-pow((s_norm - active_pitch) * 14.0, 2.0));
                    let s_col = mix(vec3<f32>(1.28, 0.88, 0.32), vec3<f32>(0.55, 1.35, 1.48), s_norm);
                    col += s_col * str_g * active_boost;
                    dens += str_g * 0.85;

                    let neck_d = abs(p.y - top_arch) + abs(p.z) * 1.4;
                    let neck_g = exp(-pow(neck_d * 54.0, 2.0));
                    col += vec3<f32>(1.35, 0.92, 0.35) * neck_g * 0.9;
                    dens += neck_g * 0.65;
                }
            }

            let alpha_s = clamp(dens * dt * 3.5, 0.0, 0.74);
            accum += trans * col * (dt * 3.5);
            trans *= (1.0 - alpha_s);
        }
        t += dt;
    }
    return vec4<f32>(accum, 1.0 - trans);
}

// ============================================================================
// EXP 13 (MATTER & ENERGY) & EXP 14 (SPACETIME) — kind == 7u (TIER-A Act III)
// Bespoke volumetric stages: 13_matter_energy (Z00..=Z11, 2300..=2311) &
//                            14_spacetime (Z00..=Z14, 2400..=2414)
// ============================================================================
fn mm_dipole_pair_glow(p: vec3<f32>, sep: f32, rad: f32) -> vec2<f32> {
    let gl = exp(-pow(abs(length(p - vec3<f32>(-sep, 0.0, 0.0)) - rad) * 44.0, 2.0));
    let gr = exp(-pow(abs(length(p - vec3<f32>(sep, 0.0, 0.0)) - rad) * 44.0, 2.0));
    return vec2<f32>(gl, gr);
}

fn mm_feat_halo(p: vec3<f32>, feat: u32, col: vec3<f32>, time: f32) -> vec4<f32> {
    if (feat >= 8u) {
        return vec4<f32>(0.0);
    }
    let ff = f32(feat);
    let ang = ff * 0.78539816 + time * 0.18;
    let rp = doc_rot_z(doc_rot_x(p, 0.32 + ff * 0.21), ang * 0.55);
    let r_ring = 0.58 - ff * 0.028;
    let ring = doc_line_glow(sd_torus(rp, r_ring, 0.0055), 88.0);
    let node_p = vec3<f32>(cos(ang) * r_ring, 0.0, sin(ang) * r_ring);
    let marker = exp(-pow(max(length(rp - node_p) - 0.038, 0.0) * 62.0, 2.0));
    return vec4<f32>(col * (ring * 0.42 + marker * 1.05), ring * 0.28 + marker * 0.75);
}

fn shade_doc_matter_energy_stage(
    zone: u32,
    feat: u32,
    p_raw: vec3<f32>,
    p: vec3<f32>,
    r: f32,
    cue: f32,
    w: f32,
    time: f32,
    voice: f32,
    bass: f32,
) -> vec4<f32> {
    var col = vec3<f32>(0.0);
    var dens: f32 = 0.0;
    let ff = f32(feat & 7u);
    let has_f = feat < 8u;

    if (zone == 0u) {
        // Z00: Van der Waals Residual Electrostatic Forces Between Neutral Atoms
        let sep = 0.30 - select(0.0, 0.025 * ff, has_f && feat < 4u);
        let dp = mm_dipole_pair_glow(p, sep, 0.22);
        let shift_l = smoothstep(-sep - 0.1, -sep + 0.15, p.x);
        let c_l = mix(vec3<f32>(0.22, 0.88, 1.32), vec3<f32>(1.28, 0.48, 0.36), shift_l);
        let bridge = doc_line_glow(sd_capsule(p, vec3<f32>(-sep * 0.45, 0.0, 0.0), vec3<f32>(sep * 0.45, 0.0, 0.0), 0.012), 78.0);
        col += c_l * dp.x * 0.92 + vec3<f32>(0.24, 0.92, 1.32) * dp.y * 0.92 + vec3<f32>(1.32, 1.04, 0.36) * bridge;
        dens += (dp.x + dp.y) * 0.78 + bridge * 0.85;
        if (feat == 1u) {
            let shell = exp(-pow(abs(length(p - vec3<f32>(sep, 0.0, 0.0)) - 0.14) * 52.0, 2.0));
            col += vec3<f32>(0.42, 1.24, 1.05) * shell;
            dens += shell * 0.72;
        } else if (feat == 2u) {
            let nuc_pair = exp(-pow(length(p - vec3<f32>(-sep, 0.0, 0.0)) * 34.0, 2.0))
                         + exp(-pow(length(p - vec3<f32>(sep, 0.0, 0.0)) * 34.0, 2.0));
            col += vec3<f32>(1.38, 1.18, 0.62) * nuc_pair;
            dens += nuc_pair * 0.95;
        } else if (feat == 3u) {
            let pn_l = exp(-pow(length(p - vec3<f32>(-sep, 0.04, 0.0)) * 36.0, 2.0));
            let pn_r = exp(-pow(length(p - vec3<f32>(sep, -0.04, 0.0)) * 36.0, 2.0));
            col += vec3<f32>(1.32, 0.38, 0.42) * pn_l + vec3<f32>(0.42, 0.86, 1.32) * pn_r;
            dens += (pn_l + pn_r) * 0.92;
        } else if (feat == 4u) {
            let tri = doc_line_glow(sd_torus(p - vec3<f32>(-sep, 0.0, 0.0), 0.08, 0.009), 78.0)
                    + doc_line_glow(sd_torus(p - vec3<f32>(sep, 0.0, 0.0), 0.08, 0.009), 78.0);
            col += vec3<f32>(0.96, 0.36, 1.28) * tri;
            dens += tri * 0.85;
        } else if (feat == 5u) {
            let ripple = pow(0.5 + 0.5 * sin(r * 22.0 - time * 1.8), 6.0) * smoothstep(0.75, 0.18, r);
            col += vec3<f32>(0.28, 0.96, 1.30) * ripple;
            dens += ripple * 0.68;
        }
    } else if (zone == 1u) {
        // Z01: Diamond Cubic Lattice & Four Phases of Matter
        let p_c = fract(p * 2.15 + 0.5) - 0.5;
        let node = exp(-pow(length(p_c) * 14.0, 2.0)) * smoothstep(0.74, 0.24, r);
        let sp3 = min(min(doc_line_glow(sd_capsule(p_c, vec3<f32>(0.0), vec3<f32>(0.22, 0.22, 0.22), 0.008), 58.0),
                          doc_line_glow(sd_capsule(p_c, vec3<f32>(0.0), vec3<f32>(-0.22, -0.22, 0.22), 0.008), 58.0)),
                      1.0);
        let b_str = doc_line_glow(sd_capsule(p_c, vec3<f32>(0.0), vec3<f32>(0.22, 0.22, 0.22), 0.008), 68.0)
                  + doc_line_glow(sd_capsule(p_c, vec3<f32>(0.0), vec3<f32>(-0.22, -0.22, 0.22), 0.008), 68.0);
        let mask_lat = smoothstep(0.76, 0.22, r);
        col += vec3<f32>(0.34, 1.12, 1.38) * (node + b_str * 0.55) * mask_lat;
        dens += (node * 0.92 + b_str * 0.45) * mask_lat + sp3 * 0.05;
        if (feat == 1u) {
            // 4-phase quadrant contrast
            let q_glow = exp(-pow(abs(r - 0.48) * 32.0, 2.0));
            let phase_c = select(select(vec3<f32>(0.24, 0.92, 1.32), vec3<f32>(0.28, 1.18, 0.72), p.x > 0.0),
                                 select(vec3<f32>(1.28, 0.88, 0.24), vec3<f32>(1.32, 0.34, 0.78), p.x > 0.0), p.y > 0.0);
            col += phase_c * q_glow * 0.82;
            dens += q_glow * 0.65;
        } else if (feat == 2u) {
            let oct_facet = doc_shell_glow(sd_octahedron(p, 0.48), 56.0);
            col += vec3<f32>(0.82, 1.24, 1.38) * oct_facet;
            dens += oct_facet * 0.88;
        } else if (feat == 3u) {
            let eq_ring = doc_line_glow(sd_torus(doc_rot_x(p, 0.785), 0.34, 0.009), 78.0);
            col += vec3<f32>(1.32, 0.92, 0.28) * eq_ring;
            dens += eq_ring * 0.82;
        } else if (feat == 4u) {
            let e_sea = pow(0.5 + 0.5 * sin(p.x * 18.0 + time * 2.2) * cos(p.z * 16.0), 4.0) * smoothstep(0.58, 0.15, r);
            col += vec3<f32>(1.28, 0.98, 0.34) * e_sea;
            dens += e_sea * 0.78;
        } else if (feat == 5u) {
            let tunnel = exp(-pow(max(length(p.xy) - 0.16, 0.0) * 42.0, 2.0)) * smoothstep(0.68, 0.18, abs(p.z));
            col += vec3<f32>(0.92, 0.44, 1.34) * tunnel;
            dens += tunnel * 0.82;
        }
    } else if (zone == 2u) {
        // Z02: Covalent Molecules (Showcase 07c: H2O bent + Benzene C6H6 pi clouds)
        let po = p - vec3<f32>(0.0, 0.25, 0.0);
        let ph1 = p - vec3<f32>(-0.22, 0.08, 0.0);
        let ph2 = p - vec3<f32>(0.22, 0.08, 0.0);
        let key_l = normalize(vec3<f32>(0.45, 0.72, 0.55));
        let shade_o = 0.35 + 0.82 * max(dot(normalize(po + vec3<f32>(1e-4)), key_l), 0.0);
        let shade_h1 = 0.35 + 0.82 * max(dot(normalize(ph1 + vec3<f32>(1e-4)), key_l), 0.0);
        let shade_h2 = 0.35 + 0.82 * max(dot(normalize(ph2 + vec3<f32>(1e-4)), key_l), 0.0);
        let g_o = exp(-pow(max(length(po) - 0.105, 0.0) * 68.0, 2.0));
        let g_h1 = exp(-pow(max(length(ph1) - 0.062, 0.0) * 76.0, 2.0));
        let g_h2 = exp(-pow(max(length(ph2) - 0.062, 0.0) * 76.0, 2.0));
        let bond1 = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, 0.25, 0.0), vec3<f32>(-0.22, 0.08, 0.0), 0.011), 96.0);
        let bond2 = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, 0.25, 0.0), vec3<f32>(0.22, 0.08, 0.0), 0.011), 96.0);
        col += vec3<f32>(1.34, 0.24, 0.26) * g_o * shade_o
             + vec3<f32>(0.84, 1.12, 1.36) * (g_h1 * shade_h1 + g_h2 * shade_h2 + bond1 + bond2);
        dens += g_o + g_h1 + g_h2 + (bond1 + bond2) * 0.85;

        let pb = doc_rot_x(p - vec3<f32>(0.0, -0.24, 0.0), 0.35);
        let hex_ring = abs(max(abs(pb.x) * 0.8660254 + abs(pb.z) * 0.5, abs(pb.z)) - 0.34);
        let hex_edge = exp(-pow(hex_ring * 78.0, 2.0)) * exp(-pow(pb.y * 42.0, 2.0));
        let inner_ar = doc_line_glow(sd_torus(pb, 0.24, 0.008), 86.0);
        let pi_up = doc_line_glow(sd_torus(pb - vec3<f32>(0.0, 0.09, 0.0), 0.27, 0.008), 76.0);
        let pi_dn = doc_line_glow(sd_torus(pb + vec3<f32>(0.0, 0.09, 0.0), 0.27, 0.008), 76.0);
        var c_nodes: f32 = 0.0;
        for (var v = 0; v < 6; v = v + 1) {
            let va = f32(v) * TAU / 6.0;
            let vp = vec3<f32>(cos(va) * 0.34, 0.0, sin(va) * 0.34);
            c_nodes += exp(-pow(max(length(pb - vp) - 0.036, 0.0) * 85.0, 2.0));
        }
        col += vec3<f32>(0.24, 1.18, 1.34) * (hex_edge + c_nodes + inner_ar * 0.8)
             + vec3<f32>(0.88, 0.44, 1.38) * (pi_up + pi_dn);
        dens += hex_edge + c_nodes + inner_ar * 0.7 + (pi_up + pi_dn) * 0.75;

        if (feat == 1u) {
            let ang_arc = doc_line_glow(sd_torus(doc_rot_x(po, 1.5708), 0.16, 0.007), 82.0) * smoothstep(0.28, 0.06, p.y);
            col += vec3<f32>(1.34, 1.05, 0.28) * ang_arc;
            dens += ang_arc * 0.82;
        } else if (feat == 2u) {
            let dipole = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, 0.04, 0.0), vec3<f32>(0.0, 0.44, 0.0), 0.009), 78.0);
            col += vec3<f32>(1.28, 0.88, 0.24) * dipole;
            dens += dipole * 0.82;
        } else if (feat == 3u) {
            let mir_a = doc_line_glow(sd_capsule(p, vec3<f32>(-0.36, 0.02, 0.0), vec3<f32>(-0.18, -0.08, 0.16), 0.01), 76.0);
            let mir_b = doc_line_glow(sd_capsule(p, vec3<f32>(0.36, 0.02, 0.0), vec3<f32>(0.18, -0.08, 0.16), 0.01), 76.0);
            col += vec3<f32>(0.36, 1.28, 0.72) * (mir_a + mir_b);
            dens += (mir_a + mir_b) * 0.82;
        } else if (feat == 4u) {
            let helix_a = exp(-pow(length(p.xz - vec2<f32>(0.18 * cos(p.y * 8.0), 0.18 * sin(p.y * 8.0))) * 46.0, 2.0)) * smoothstep(0.55, 0.15, abs(p.y));
            let helix_b = exp(-pow(length(p.xz + vec2<f32>(0.18 * cos(p.y * 8.0), 0.18 * sin(p.y * 8.0))) * 46.0, 2.0)) * smoothstep(0.55, 0.15, abs(p.y));
            col += vec3<f32>(0.24, 1.24, 0.96) * helix_a + vec3<f32>(1.24, 0.62, 0.28) * helix_b;
            dens += (helix_a + helix_b) * 0.75;
        } else if (feat == 5u) {
            let vib = exp(-pow(abs(length(po) - 0.18 - 0.03 * sin(time * 5.0)) * 54.0, 2.0));
            col += vec3<f32>(1.28, 0.42, 0.96) * vib;
            dens += vib * 0.84;
        }
    } else if (zone == 3u) {
        // Z03: Hydrogen 3d_z2 Electron Orbitals + Bohr Rings + Nucleus (Showcase 07)
        let r_len = max(length(p), 1e-4);
        let cos_th = p.y / r_len;
        let ang_harm = select(
            abs(3.0 * cos_th * cos_th - 1.0) * 0.45,
            abs(cos_th),
            feat == 1u
        );
        let lobe_r = 0.44 * ang_harm;
        let lobe_shell = exp(-pow(abs(r_len - lobe_r) * 58.0, 2.0)) * smoothstep(0.06, 0.22, lobe_r);
        let core_fill = exp(-pow((r_len - lobe_r * 0.65) * 14.0, 2.0)) * pow(ang_harm, 2.0) * 0.42;
        let lobe_col = select(vec3<f32>(0.22, 0.94, 1.38), vec3<f32>(1.32, 0.40, 0.92), cos_th < 0.0);
        let bohr_1 = doc_line_glow(sd_torus(doc_rot_x(p, 0.38), 0.56, 0.005), 96.0);
        let bohr_2 = doc_line_glow(sd_torus(doc_rot_z(p, -0.52), 0.56, 0.005), 96.0);
        let nuc = exp(-pow(r_len * 32.0, 2.0)) * 1.35;
        col += lobe_col * (lobe_shell * 1.05 + core_fill)
             + vec3<f32>(1.28, 0.98, 0.42) * (bohr_1 + bohr_2) * 0.62
             + vec3<f32>(1.38, 1.18, 0.68) * nuc;
        dens += lobe_shell * 0.92 + core_fill * 0.48 + (bohr_1 + bohr_2) * 0.52 + nuc;

        if (feat == 0u) {
            let void_r = exp(-pow(abs(r_len - 0.64) * 52.0, 2.0));
            col += vec3<f32>(0.28, 0.88, 1.24) * void_r * 0.72;
            dens += void_r * 0.55;
        } else if (feat == 2u) {
            let bohr_in = doc_line_glow(sd_torus(doc_rot_x(p, 0.38), 0.28, 0.007), 86.0);
            col += vec3<f32>(1.34, 1.02, 0.36) * bohr_in;
            dens += bohr_in * 0.82;
        } else if (feat == 3u) {
            let spin_u = doc_line_glow(sd_capsule(p, vec3<f32>(-0.34, -0.18, 0.0), vec3<f32>(-0.34, 0.18, 0.0), 0.01), 78.0);
            let spin_d = doc_line_glow(sd_capsule(p, vec3<f32>(0.34, 0.18, 0.0), vec3<f32>(0.34, -0.18, 0.0), 0.01), 78.0);
            col += vec3<f32>(0.24, 1.24, 0.88) * spin_u + vec3<f32>(1.28, 0.38, 0.58) * spin_d;
            dens += (spin_u + spin_d) * 0.82;
        } else if (feat == 4u) {
            let oct_r = doc_line_glow(sd_torus(p, 0.46, 0.008), 82.0);
            col += vec3<f32>(0.42, 1.22, 1.32) * oct_r;
            dens += oct_r * 0.78;
        } else if (feat == 5u) {
            let photon = exp(-pow(length(p - vec3<f32>(0.36, 0.12 * sin(p.x * 16.0 - time * 3.0), 0.0)) * 22.0, 2.0));
            col += vec3<f32>(1.36, 0.26, 0.22) * photon;
            dens += photon * 0.88;
        }
    } else if (zone == 4u) {
        // Z04: Atomic Nucleus — Proton/Neutron Cluster + Yukawa/Coulomb Balance
        var clus_c = vec3<f32>(0.0);
        var clus_d: f32 = 0.0;
        for (var k = 0; k < 6; k = k + 1) {
            let a = f32(k) * 1.0471975 + time * 0.25;
            let off = vec3<f32>(cos(a) * 0.12, sin(f32(k) * 1.7) * 0.10, sin(a) * 0.12);
            let g_n = exp(-pow(max(length(p - off) - 0.072, 0.0) * 64.0, 2.0));
            let is_p = (k % 2) == 0;
            clus_c += select(vec3<f32>(0.34, 0.78, 1.28), vec3<f32>(1.32, 0.28, 0.26), is_p) * g_n;
            clus_d += g_n;
        }
        col += clus_c;
        dens += clus_d;
        if (feat == 1u) {
            let barrier = exp(-pow(abs(r - 0.36) * 52.0, 2.0));
            col += vec3<f32>(1.32, 0.84, 0.24) * barrier;
            dens += barrier * 0.78;
        } else if (feat == 2u) {
            let iso_r = doc_line_glow(sd_torus(doc_rot_x(p, 0.45), 0.38, 0.009), 78.0);
            col += vec3<f32>(0.28, 1.08, 1.36) * iso_r;
            dens += iso_r * 0.78;
        } else if (feat == 3u) {
            let fe_peak = exp(-pow(abs(p.y - (0.32 * exp(-pow(p.x * 2.2, 2.0)) - 0.12)) * 46.0, 2.0)) * smoothstep(0.58, 0.2, abs(p.x));
            col += vec3<f32>(1.28, 0.96, 0.28) * fe_peak;
            dens += fe_peak * 0.85;
        } else if (feat == 4u) {
            let t_a = doc_line_glow(sd_capsule(p, vec3<f32>(0.0), vec3<f32>(0.48, 0.28, 0.0), 0.008), 72.0);
            let t_b = doc_line_glow(sd_capsule(p, vec3<f32>(0.0), vec3<f32>(-0.46, 0.32, 0.12), 0.008), 72.0);
            col += vec3<f32>(1.32, 0.62, 0.22) * t_a + vec3<f32>(0.32, 1.24, 1.32) * t_b;
            dens += (t_a + t_b) * 0.82;
        } else if (feat == 5u) {
            let neck_l = exp(-pow(max(length(p - vec3<f32>(-0.26, 0.0, 0.0)) - 0.14, 0.0) * 44.0, 2.0));
            let neck_r = exp(-pow(max(length(p - vec3<f32>(0.26, 0.0, 0.0)) - 0.14, 0.0) * 44.0, 2.0));
            col += vec3<f32>(1.34, 0.72, 0.22) * (neck_l + neck_r);
            dens += (neck_l + neck_r) * 0.84;
        }
    } else if (zone == 5u) {
        // Z05: Nucleons — 99% QCD Kinetic/Gluon Binding Energy Cutaway
        let shell = exp(-pow(abs(r - 0.44) * 52.0, 2.0)) * select(1.0, 0.35, p.z > 0.0 && p.x > 0.0);
        let q1 = exp(-pow(max(length(p - vec3<f32>(-0.14, 0.12, 0.0)) - 0.055, 0.0) * 68.0, 2.0));
        let q2 = exp(-pow(max(length(p - vec3<f32>(0.15, 0.10, 0.0)) - 0.055, 0.0) * 68.0, 2.0));
        let q3 = exp(-pow(max(length(p - vec3<f32>(0.0, -0.16, 0.0)) - 0.055, 0.0) * 68.0, 2.0));
        let sea_w = pow(0.5 + 0.5 * sin(p.x * 16.0 + p.y * 14.0 - time * 2.2), 4.0) * smoothstep(0.44, 0.08, r);
        col += vec3<f32>(0.24, 0.92, 1.32) * (shell * 0.75 + sea_w * 0.65)
             + vec3<f32>(1.32, 0.28, 0.28) * q1 + vec3<f32>(0.28, 1.26, 0.42) * q2 + vec3<f32>(0.34, 0.62, 1.34) * q3;
        dens += shell * 0.65 + q1 + q2 + q3 + sea_w * 0.58;
        if (feat == 1u) {
            let e_tor = doc_line_glow(sd_torus(doc_rot_x(p, 0.62), 0.30, 0.012), 72.0);
            col += vec3<f32>(1.34, 0.96, 0.24) * e_tor;
            dens += e_tor * 0.85;
        } else if (feat == 2u) {
            let mom = doc_line_glow(sd_torus(p, 0.26, 0.008), 82.0);
            col += vec3<f32>(0.92, 0.42, 1.34) * mom;
            dens += mom * 0.78;
        } else if (feat == 3u) {
            let qq = exp(-pow(max(length(p - vec3<f32>(0.0, 0.02, 0.16)) - 0.06, 0.0) * 58.0, 2.0));
            col += vec3<f32>(1.32, 0.48, 0.92) * qq;
            dens += qq * 0.82;
        } else if (feat == 4u) {
            let mag = doc_line_glow(sd_torus(doc_rot_z(p, 1.5708), 0.48, 0.008), 80.0);
            col += vec3<f32>(0.36, 1.22, 1.34) * mag;
            dens += mag * 0.78;
        } else if (feat == 5u) {
            let pion = exp(-pow(abs(length(p.xz) - 0.56) * 44.0, 2.0)) * smoothstep(0.24, 0.02, abs(p.y));
            col += vec3<f32>(1.32, 0.56, 0.24) * pion;
            dens += pion * 0.78;
        }
    } else if (zone == 6u) {
        // Z06: Quarks & Color Confinement Y-Flux Tubes (Showcase 07b)
        let q1 = vec3<f32>(0.26 * cos(time * 1.4), 0.12 * sin(time * 1.9), 0.26 * sin(time * 1.4));
        let q2 = vec3<f32>(0.26 * cos(time * 1.4 + 2.094), -0.14 * cos(time * 1.7), 0.26 * sin(time * 1.4 + 2.094));
        let q3 = vec3<f32>(0.26 * cos(time * 1.4 + 4.188), 0.10 * sin(time * 2.3), 0.26 * sin(time * 1.4 + 4.188));
        let junc = (q1 + q2 + q3) * 0.18;
        let g1 = exp(-pow(max(length(p - q1) - 0.075, 0.0) * 62.0, 2.0));
        let g2 = exp(-pow(max(length(p - q2) - 0.075, 0.0) * 62.0, 2.0));
        let g3 = exp(-pow(max(length(p - q3) - 0.075, 0.0) * 62.0, 2.0));
        let f1 = doc_line_glow(sd_capsule(p, junc, q1, 0.012), 86.0);
        let f2 = doc_line_glow(sd_capsule(p, junc, q2, 0.012), 86.0);
        let f3 = doc_line_glow(sd_capsule(p, junc, q3, 0.012), 86.0);
        let ring_c = doc_line_glow(sd_torus(doc_rot_x(p, 0.45), 0.56, 0.005), 88.0);
        let bag = exp(-pow(abs(r - 0.56) * 68.0, 2.0)) * (0.28 + 0.18 * voice);
        col += vec3<f32>(1.36, 0.22, 0.22) * (g1 + f1)
             + vec3<f32>(0.22, 1.32, 0.38) * (g2 + f2)
             + vec3<f32>(0.28, 0.62, 1.38) * (g3 + f3)
             + vec3<f32>(0.76, 0.52, 1.22) * (bag + ring_c * 0.6);
        dens += g1 + g2 + g3 + f1 + f2 + f3 + bag + ring_c * 0.5;
        if (feat == 0u) {
            let ladder = doc_line_glow(sd_capsule(p, vec3<f32>(-0.42, -0.34, 0.0), vec3<f32>(-0.42, 0.34, 0.0), 0.01), 76.0);
            col += vec3<f32>(1.32, 1.02, 0.32) * ladder;
            dens += ladder * 0.82;
        } else if (feat == 1u) {
            let white_c = exp(-pow(length(p - junc) * 24.0, 2.0));
            col += vec3<f32>(1.32, 1.28, 1.22) * white_c;
            dens += white_c * 0.92;
        } else if (feat == 2u) {
            let tri_g = doc_line_glow(sd_torus(p, 0.19, 0.009), 82.0);
            col += vec3<f32>(1.28, 0.72, 0.24) * tri_g;
            dens += tri_g * 0.85;
        } else if (feat == 3u) {
            let stretch = doc_line_glow(sd_capsule(p, vec3<f32>(-0.46, 0.0, 0.0), vec3<f32>(0.46, 0.0, 0.0), 0.012), 78.0);
            col += vec3<f32>(1.34, 0.42, 0.88) * stretch;
            dens += stretch * 0.86;
        } else if (feat == 4u) {
            let ir_wall = exp(-pow(abs(r - 0.66) * 48.0, 2.0));
            col += vec3<f32>(0.92, 0.32, 1.32) * ir_wall;
            dens += ir_wall * 0.74;
        } else if (feat == 5u) {
            let lep = doc_line_glow(sd_torus(doc_rot_z(p, 0.95), 0.42, 0.008), 80.0);
            col += vec3<f32>(0.24, 1.18, 1.34) * lep;
            dens += lep * 0.78;
        }
    } else if (zone == 7u) {
        // Z07: 17-Field Standard Model Mandala & Higgs Mexican-Hat Potential
        let rho = length(p.xz);
        let hat_y = -0.30 + 1.18 * (pow(rho / 0.56, 4.0) - 0.78 * pow(rho / 0.56, 2.0));
        if (rho < 0.70) {
            let d_hat = abs(p.y - hat_y);
            let ring_g = exp(-pow(abs(fract(rho * 9.0) - 0.5) * 12.0, 2.0));
            let rad_s = exp(-pow(abs(fract(atan2(p.z, p.x) / TAU * 12.0) - 0.5) * 14.0, 2.0));
            let sg = exp(-pow(d_hat * 44.0, 2.0)) * (0.20 + 0.88 * max(ring_g, rad_s * 0.7));
            col += mix(vec3<f32>(0.25, 0.78, 1.30), vec3<f32>(1.32, 0.88, 0.24), smoothstep(0.1, 0.42, rho)) * sg;
            dens += sg * 0.82;
        }
        let b_ang = time * 0.9 + ff * 0.52;
        let boson_p = vec3<f32>(cos(b_ang) * 0.34, -0.44 + 0.06, sin(b_ang) * 0.34);
        let b_g = exp(-pow(length(p - boson_p) * 20.0, 2.0));
        col += vec3<f32>(1.36, 1.08, 0.38) * b_g;
        dens += b_g;
        if (feat == 1u) {
            let stair = doc_line_glow(sd_capsule(p, vec3<f32>(-0.36, -0.24, 0.0), vec3<f32>(0.36, 0.24, 0.0), 0.009), 76.0);
            col += vec3<f32>(0.42, 1.24, 0.96) * stair;
            dens += stair * 0.82;
        } else if (feat == 2u) {
            let osc = exp(-pow(length(p.xy - vec2<f32>(0.22 * sin(p.z * 9.0 + time * 1.4), 0.14 * cos(p.z * 9.0))) * 38.0, 2.0)) * smoothstep(0.58, 0.15, abs(p.z));
            col += vec3<f32>(0.34, 1.26, 1.32) * osc;
            dens += osc * 0.84;
        } else if (feat == 3u) {
            let pillar = doc_line_glow(sd_cylinder_y(p - vec3<f32>(0.0, 0.05, 0.0), 0.05, 0.36), 66.0);
            col += vec3<f32>(1.32, 0.48, 0.92) * pillar;
            dens += pillar * 0.82;
        } else if (feat == 5u) {
            let sm_ring = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.18, 0.0), 0.48, 0.008), 80.0);
            col += vec3<f32>(1.28, 1.02, 0.36) * sm_ring;
            dens += sm_ring * 0.80;
        }
    } else if (zone == 8u) {
        // Z08: Four Fundamental Forces + Feynman Vertices + Running Couplings
        let gamma_a = doc_line_glow(sd_capsule(p, vec3<f32>(-0.46, -0.28, 0.0), vec3<f32>(0.0, 0.0, 0.0), 0.009), 76.0);
        let gamma_b = doc_line_glow(sd_capsule(p, vec3<f32>(-0.46, 0.28, 0.0), vec3<f32>(0.0, 0.0, 0.0), 0.009), 76.0);
        let prop_m = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, 0.0, 0.0), vec3<f32>(0.42, 0.0, 0.0), 0.011), 76.0)
                   * (0.55 + 0.45 * sin(p.x * 36.0 - time * 2.5));
        let vert = exp(-pow(r * 24.0, 2.0));
        col += vec3<f32>(0.24, 1.08, 1.34) * (gamma_a + gamma_b) + vec3<f32>(1.34, 0.98, 0.28) * (prop_m + vert);
        dens += (gamma_a + gamma_b + prop_m) * 0.85 + vert;
        if (feat == 2u) {
            let tri = doc_line_glow(sd_torus(doc_rot_x(p, 1.5708), 0.26, 0.009), 80.0);
            col += vec3<f32>(0.34, 1.32, 0.48) * tri;
            dens += tri * 0.82;
        } else if (feat == 3u) {
            let chiral = exp(-pow(length(p.yz - vec2<f32>(0.16 * cos(p.x * 10.0), 0.16 * sin(p.x * 10.0))) * 44.0, 2.0)) * smoothstep(0.55, 0.15, abs(p.x));
            col += vec3<f32>(0.96, 0.36, 1.32) * chiral;
            dens += chiral * 0.82;
        } else if (feat == 4u) {
            let bars = exp(-pow(abs(abs(p.y) - 0.28) * 52.0, 2.0)) * smoothstep(0.52, 0.18, abs(p.x));
            col += vec3<f32>(1.28, 0.56, 0.24) * bars;
            dens += bars * 0.78;
        } else if (feat == 5u) {
            let conv = doc_line_glow(sd_capsule(p, vec3<f32>(-0.42, -0.32, 0.0), vec3<f32>(0.42, 0.02, 0.0), 0.009), 76.0);
            col += vec3<f32>(1.34, 1.12, 0.42) * conv;
            dens += conv * 0.82;
        }
    } else if (zone == 9u) {
        // Z09: Quantum Fields — 3D Harmonic Oscillator Lattice & Localized Excitations
        let pf = p * 3.0;
        let cell = fract(pf + 0.5) - 0.5;
        let wave_h = 0.08 * sin(p.x * 9.0 - time * 2.2) * cos(p.z * 9.0 + time * 1.4);
        let pts = exp(-pow(length(cell - vec3<f32>(0.0, wave_h, 0.0)) * 12.0, 2.0)) * smoothstep(0.76, 0.24, r);
        let pkt = exp(-pow(length(p - vec3<f32>(0.0, wave_h * 1.8, 0.0)) * 12.0, 2.0));
        col += vec3<f32>(0.24, 0.88, 1.32) * pts * 0.85 + vec3<f32>(1.32, 0.94, 0.32) * pkt;
        dens += pts * 0.78 + pkt * 0.88;
        if (feat == 2u) {
            let slit = exp(-pow(abs(p.z) * 48.0, 2.0)) * smoothstep(0.56, 0.16, r) * (0.5 + 0.5 * cos(p.x * 24.0));
            col += vec3<f32>(0.36, 1.24, 0.88) * slit;
            dens += slit * 0.82;
        } else if (feat == 3u) {
            let pair_ep = exp(-pow(length(p - vec3<f32>(-0.22, 0.0, 0.0)) * 22.0, 2.0))
                        + exp(-pow(length(p - vec3<f32>(0.22, 0.0, 0.0)) * 22.0, 2.0));
            col += vec3<f32>(1.32, 0.38, 0.92) * pair_ep;
            dens += pair_ep * 0.85;
        } else if (feat == 4u) {
            let mob = doc_line_glow(sd_torus(doc_rot_y(p, p.y * 1.5), 0.34, 0.011), 76.0);
            col += vec3<f32>(0.92, 0.48, 1.34) * mob;
            dens += mob * 0.82;
        } else if (feat == 5u) {
            let symph = exp(-pow(abs(r - 0.36) * 44.0, 2.0)) * (0.5 + 0.5 * cos(atan2(p.z, p.x) * 5.0));
            col += vec3<f32>(1.28, 1.08, 0.42) * symph;
            dens += symph * 0.78;
        }
    } else if (zone == 10u) {
        // Z10: Boiling Quantum Vacuum & Parallel Casimir Conducting Plates
        let boil = pow(max(sin(p.x * 19.0 + time * 2.4) * cos(p.y * 17.0 - time * 1.9) * sin(p.z * 21.0 + time * 2.1), 0.0), 3.0) * smoothstep(0.72, 0.18, r);
        col += vec3<f32>(0.36, 0.72, 1.32) * boil;
        dens += boil * 0.78;
        if (feat == 1u) {
            let loops = doc_line_glow(sd_torus(p - vec3<f32>(-0.18, 0.12, 0.0), 0.11, 0.008), 80.0)
                      + doc_line_glow(sd_torus(p - vec3<f32>(0.18, -0.10, 0.0), 0.11, 0.008), 80.0);
            col += vec3<f32>(1.32, 0.52, 0.98) * loops;
            dens += loops * 0.86;
        } else if (feat == 2u) {
            let heis = exp(-pow(abs(r - 0.34 - 0.08 * sin(time * 2.8)) * 46.0, 2.0));
            col += vec3<f32>(1.32, 0.96, 0.28) * heis;
            dens += heis * 0.82;
        } else if (feat == 3u) {
            // Parallel Casimir plates at x = +-0.22 suppressing interior modes
            let pl_l = doc_line_glow(sd_box(p - vec3<f32>(-0.22, 0.0, 0.0), vec3<f32>(0.012, 0.34, 0.34)), 74.0);
            let pl_r = doc_line_glow(sd_box(p - vec3<f32>(0.22, 0.0, 0.0), vec3<f32>(0.012, 0.34, 0.34)), 74.0);
            col += vec3<f32>(0.88, 1.12, 1.36) * (pl_l + pl_r);
            dens += (pl_l + pl_r) * 0.88;
        } else if (feat == 4u) {
            let scr = exp(-pow(r * 18.0, 2.0)) + exp(-pow(abs(r - 0.28) * 44.0, 2.0)) * 0.7;
            col += vec3<f32>(0.24, 1.24, 1.18) * scr;
            dens += scr * 0.82;
        } else if (feat == 5u) {
            let horizon = exp(-pow(abs(r - 0.62) * 48.0, 2.0));
            col += vec3<f32>(1.34, 0.42, 0.24) * horizon;
            dens += horizon * 0.82;
        }
    } else {
        // Z11: Symmetries, Noether Conservation, Matter-Antimatter & Unification Frontier
        let d_tor = sd_torus(p, 0.40, 0.13);
        let wire = exp(-pow(abs(fract(atan2(p.z, p.x) * 3.0) - 0.5) * 12.0, 2.0));
        let tg = exp(-pow(abs(d_tor) * 48.0, 2.0)) * (0.28 + 0.86 * wire);
        col += vec3<f32>(0.32, 1.02, 1.34) * tg;
        dens += tg * 0.82;
        if (feat == 1u) {
            let fib = doc_line_glow(sd_torus(doc_rot_x(p, 1.5708), 0.26, 0.008), 80.0);
            col += vec3<f32>(0.92, 0.44, 1.34) * fib;
            dens += fib * 0.84;
        } else if (feat == 2u) {
            let b_roll = exp(-pow(length(p - vec3<f32>(0.28, -0.14, 0.0)) * 20.0, 2.0));
            col += vec3<f32>(1.34, 1.02, 0.28) * b_roll;
            dens += b_roll * 0.88;
        } else if (feat == 3u) {
            let m_surv = exp(-pow(length(p - vec3<f32>(0.0, 0.04, 0.0)) * 22.0, 2.0));
            col += vec3<f32>(1.34, 1.24, 0.62) * m_surv;
            dens += m_surv * 0.90;
        } else if (feat == 4u) {
            let dm_halo = exp(-pow(abs(r - 0.62) * 38.0, 2.0));
            col += vec3<f32>(0.34, 0.48, 1.32) * dm_halo;
            dens += dm_halo * 0.72;
        } else if (feat == 5u) {
            let qg = doc_line_glow(sd_octahedron(p, 0.34), 68.0);
            col += vec3<f32>(1.28, 0.68, 0.24) * qg;
            dens += qg * 0.82;
        } else if (feat == 6u) {
            let crown = doc_line_glow(sd_torus(doc_rot_z(p, 0.785), 0.54, 0.008), 80.0);
            col += vec3<f32>(1.32, 1.08, 0.44) * crown;
            dens += crown * 0.82;
        }
    }

    if (has_f) {
        let fh = mm_feat_halo(p_raw, feat, vec3<f32>(0.82, 1.12, 1.34), time);
        col += fh.rgb * (0.45 + 0.55 * w);
        dens += fh.a * (0.45 + 0.55 * w);
    }
    let pulse = 1.0 + bass * 0.14 + voice * 0.10 + cue * 0.08;
    return vec4<f32>(col * pulse, dens);
}

fn shade_doc_spacetime_stage(
    zone: u32,
    feat: u32,
    p_raw: vec3<f32>,
    p: vec3<f32>,
    r: f32,
    cue: f32,
    w: f32,
    time: f32,
    voice: f32,
    bass: f32,
) -> vec4<f32> {
    var col = vec3<f32>(0.0);
    var dens: f32 = 0.0;
    let ff = f32(feat & 7u);
    let has_f = feat < 8u;

    if (zone == 0u) {
        // Z00: Wheeler Reciprocal Law — Matter Tells Spacetime How to Curve
        let rho = length(p.xz);
        let well_y = -0.34 + 0.38 * smoothstep(0.10, 0.65, rho);
        let grid = max(exp(-pow(abs(fract(p.x * 5.0) - 0.5) * 14.0, 2.0)), exp(-pow(abs(fract(p.z * 5.0) - 0.5) * 14.0, 2.0)));
        let sheet = exp(-pow(abs(p.y - well_y) * 46.0, 2.0)) * smoothstep(0.76, 0.24, rho) * (0.25 + 0.85 * grid);
        let core = exp(-pow(r * 18.0, 2.0));
        col += vec3<f32>(0.24, 0.88, 1.32) * sheet + vec3<f32>(1.32, 0.92, 0.28) * core;
        dens += sheet * 0.84 + core;
        if (feat == 1u) {
            let beam = doc_line_glow(sd_capsule(p, vec3<f32>(-0.56, 0.16, 0.0), vec3<f32>(0.56, 0.16, 0.0), 0.01), 78.0);
            col += vec3<f32>(1.36, 1.16, 0.36) * beam;
            dens += beam * 0.85;
        } else if (feat == 2u) {
            let min_c = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.22, 0.0), 0.38, 0.008), 80.0);
            col += vec3<f32>(0.36, 1.24, 0.92) * min_c;
            dens += min_c * 0.80;
        } else if (feat == 3u) {
            let fun_r = doc_line_glow(sd_torus(p + vec3<f32>(0.0, 0.16, 0.0), 0.26, 0.009), 80.0);
            col += vec3<f32>(1.32, 0.52, 0.24) * fun_r;
            dens += fun_r * 0.82;
        } else if (feat == 4u) {
            let rip = exp(-pow(abs(rho - 0.52) * 48.0, 2.0)) * (0.5 + 0.5 * cos(atan2(p.z, p.x) * 2.0 - time * 2.0));
            col += vec3<f32>(0.42, 0.96, 1.36) * rip;
            dens += rip * 0.78;
        } else if (feat == 5u) {
            let fq = exp(-pow(abs(r - 0.20) * 52.0, 2.0));
            col += vec3<f32>(0.92, 0.42, 1.34) * fq;
            dens += fq * 0.82;
        }
    } else if (zone == 1u || zone == 2u) {
        // Z01 Euclidean Reference Lattice | Z02 Special Relativity Light Clock & Boost
        let cgrid = max(
            min(exp(-pow(abs(fract(p.x * 3.5) - 0.5) * 18.0, 2.0)), exp(-pow(abs(fract(p.y * 3.5) - 0.5) * 18.0, 2.0))),
            exp(-pow(abs(fract(p.z * 3.5) - 0.5) * 18.0, 2.0)) * 0.25
        ) * smoothstep(0.74, 0.22, r);
        let base_c = select(vec3<f32>(0.24, 0.78, 1.26), vec3<f32>(1.28, 0.96, 0.28), zone == 2u);
        col += base_c * cgrid * 0.68;
        dens += cgrid * 0.62;
        if (zone == 1u) {
            let clk = doc_line_glow(sd_torus(doc_rot_x(p, 1.5708), 0.32, 0.01), 78.0);
            col += vec3<f32>(1.28, 1.04, 0.42) * clk;
            dens += clk * 0.82;
            if (feat == 2u) {
                let para = exp(-pow(abs(p.y - (0.28 - 0.85 * p.x * p.x)) * 48.0, 2.0)) * smoothstep(0.56, 0.16, abs(p.x));
                col += vec3<f32>(0.24, 1.24, 0.88) * para;
                dens += para * 0.84;
            } else if (feat == 4u) {
                let arm1 = doc_line_glow(sd_capsule(p, vec3<f32>(0.0), vec3<f32>(0.48, 0.0, 0.0), 0.009), 78.0);
                let arm2 = doc_line_glow(sd_capsule(p, vec3<f32>(0.0), vec3<f32>(0.0, 0.0, 0.48), 0.009), 78.0);
                col += vec3<f32>(1.34, 0.36, 0.36) * (arm1 + arm2);
                dens += (arm1 + arm2) * 0.84;
            } else if (feat >= 5u) {
                let em = exp(-pow(abs(p.y - 0.18 * sin(p.x * 12.0 - time * 2.4)) * 46.0, 2.0)) * smoothstep(0.56, 0.18, abs(p.x));
                col += vec3<f32>(1.32, 0.84, 0.22) * em;
                dens += em * 0.84;
            }
        } else {
            // Z02: Stationary vertical light clock vs dilated sawtooth diagonal light clock
            let lsph = exp(-pow(abs(r - 0.28) * 48.0, 2.0));
            let clk_a = doc_line_glow(sd_capsule(p, vec3<f32>(-0.26, -0.28, 0.0), vec3<f32>(-0.26, 0.28, 0.0), 0.01), 78.0);
            let clk_b = doc_line_glow(sd_capsule(p, vec3<f32>(0.06, -0.28, 0.0), vec3<f32>(0.24, 0.28, 0.0), 0.01), 78.0)
                      + doc_line_glow(sd_capsule(p, vec3<f32>(0.24, 0.28, 0.0), vec3<f32>(0.42, -0.28, 0.0), 0.01), 78.0);
            col += vec3<f32>(1.34, 1.08, 0.32) * (lsph * 0.65 + clk_a) + vec3<f32>(0.28, 1.18, 1.36) * clk_b;
            dens += lsph * 0.62 + (clk_a + clk_b) * 0.82;
            if (feat == 1u || feat == 3u) {
                let bar = doc_line_glow(sd_box(p, vec3<f32>(select(0.42, 0.24, feat == 3u), 0.06, 0.06)), 74.0);
                col += vec3<f32>(1.28, 0.44, 0.88) * bar;
                dens += bar * 0.82;
            } else if (feat >= 4u) {
                let hyp = exp(-pow(abs(p.y * p.y - p.x * p.x - 0.08) * 32.0, 2.0)) * smoothstep(0.62, 0.18, r);
                col += vec3<f32>(0.92, 0.46, 1.34) * hyp;
                dens += hyp * 0.82;
            }
        }
    } else if (zone == 3u || zone == 4u) {
        // Z03 Minkowski Light Cones & Twin Paradox | Z04 4D Block Universe & Simultaneous Slices
        let r_xz = length(p.xz);
        let cone_d = abs(r_xz - abs(p.y) * 0.88);
        let in_h = smoothstep(0.66, 0.58, abs(p.y));
        let lon = exp(-pow(abs(fract(atan2(p.z, p.x) / TAU * 12.0) - 0.5) * 14.0, 2.0));
        let lat = exp(-pow(abs(fract(p.y * 5.0) - 0.5) * 14.0, 2.0));
        let cone_g = exp(-pow(cone_d * 52.0, 2.0)) * in_h * (0.25 + 0.88 * max(lon, lat));
        let cone_c = select(vec3<f32>(0.34, 0.84, 1.36), vec3<f32>(1.34, 0.76, 0.28), p.y < 0.0);
        col += cone_c * cone_g;
        dens += cone_g * 0.85;
        let orig = exp(-pow(r * 22.0, 2.0)) * (1.0 + 0.32 * cue);
        col += vec3<f32>(1.36, 1.24, 0.96) * orig;
        dens += orig;
        if (zone == 3u) {
            let w1 = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, -0.58, 0.0), vec3<f32>(0.0, 0.58, 0.0), 0.009), 78.0);
            let w2 = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, -0.58, 0.0), vec3<f32>(0.28, 0.0, 0.0), 0.009), 78.0)
                   + doc_line_glow(sd_capsule(p, vec3<f32>(0.28, 0.0, 0.0), vec3<f32>(0.0, 0.58, 0.0), 0.009), 78.0);
            col += vec3<f32>(0.32, 1.32, 0.86) * w1 + vec3<f32>(1.34, 0.52, 0.84) * select(w2 * 0.35, w2, feat == 5u);
            dens += w1 * 0.85 + w2 * select(0.28, 0.85, feat == 5u);
        } else {
            // Z04: Helical worldline & tilted simultaneity slab
            let h_off = vec2<f32>(0.22 * cos(p.y * 8.0), 0.22 * sin(p.y * 8.0));
            let helix = exp(-pow(length(p.xz - h_off) * 44.0, 2.0)) * in_h;
            let tilt = exp(-pow(abs(p.y - p.x * (0.28 + ff * 0.06)) * 48.0, 2.0)) * smoothstep(0.58, 0.18, r_xz);
            col += vec3<f32>(0.28, 1.26, 0.94) * helix + vec3<f32>(1.32, 0.94, 0.32) * tilt;
            dens += helix * 0.84 + tilt * 0.72;
        }
    } else if (zone == 5u) {
        // Z05: Equivalence Principle Cabin — Free-Fall vs Accelerated Laser Arc
        let cab_l = doc_line_glow(sd_box(p - vec3<f32>(-0.28, 0.0, 0.0), vec3<f32>(0.18, 0.28, 0.18)), 64.0);
        let cab_r = doc_line_glow(sd_box(p - vec3<f32>(0.28, 0.0, 0.0), vec3<f32>(0.18, 0.28, 0.18)), 64.0);
        let laser_arc = exp(-pow(abs(p.y - (0.08 - 0.45 * p.x * p.x)) * 52.0, 2.0)) * smoothstep(0.46, 0.08, abs(p.x));
        col += vec3<f32>(0.24, 0.88, 1.32) * (cab_l + cab_r) * 0.75 + vec3<f32>(1.36, 0.32, 0.28) * laser_arc;
        dens += (cab_l + cab_r) * 0.68 + laser_arc * 0.88;
        if (feat == 4u) {
            let tidal = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, -0.34, 0.0), vec3<f32>(0.0, 0.34, 0.0), 0.011), 76.0);
            col += vec3<f32>(1.32, 1.02, 0.34) * tidal;
            dens += tidal * 0.85;
        }
    } else if (zone == 6u) {
        // Z06: Flamm's Paraboloid + Mercury Rosette Precession + GPS Sync
        let rho = length(p.xz);
        let throat_r = 0.14;
        let flamm_y = -0.50 + 0.52 * sqrt(max(rho - throat_r, 0.0));
        if (rho > throat_r - 0.02 && rho < 0.76) {
            let d_well = abs(p.y - flamm_y);
            let grid_r = exp(-pow(abs(fract(rho * 9.0) - 0.5) * 13.0, 2.0));
            let grid_a = exp(-pow(abs(fract(atan2(p.z, p.x) / TAU * 16.0) - 0.5) * 14.0, 2.0));
            let wg = exp(-pow(d_well * 52.0, 2.0)) * (0.18 + 0.92 * max(grid_r, grid_a));
            col += mix(vec3<f32>(1.32, 0.52, 0.20), vec3<f32>(0.28, 0.82, 1.34), smoothstep(0.16, 0.55, rho)) * wg;
            dens += wg * 0.88;
        }
        let a_m = atan2(p.z, p.x);
        let r_prec = 0.42 + 0.11 * cos(3.0 * a_m - time * 0.8);
        let d_orb = length(vec2<f32>(rho - r_prec, p.y - 0.12));
        let og = exp(-pow(d_orb * 48.0, 2.0));
        col += vec3<f32>(1.34, 1.04, 0.38) * og;
        dens += og * 0.82;
        if (feat == 4u) {
            let e_ring = doc_line_glow(sd_torus(doc_rot_x(p - vec3<f32>(0.0, 0.18, 0.0), 1.5708), 0.34, 0.009), 80.0);
            col += vec3<f32>(0.36, 1.24, 1.36) * e_ring;
            dens += e_ring * 0.84;
        } else if (feat == 6u) {
            let gps = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.24, 0.0), 0.48, 0.008), 80.0);
            col += vec3<f32>(0.32, 1.28, 0.82) * gps;
            dens += gps * 0.82;
        }
    } else if (zone == 7u) {
        // Z07: Kerr Rotating Black Hole, Photon Ring, Frame-Dragging & Ergosphere
        let rho = length(p.xz);
        let dark_c = smoothstep(0.175, 0.15, r);
        let p_ring = exp(-pow(abs(r - 0.21) * 68.0, 2.0));
        let doppler = 0.65 + 0.55 * clamp(-p.x * 2.5, -0.45, 0.65);
        let disk = exp(-pow(p.y * 34.0, 2.0)) * smoothstep(0.19, 0.26, rho) * smoothstep(0.68, 0.28, rho)
                 * (0.55 + 0.45 * sin(atan2(p.z, p.x) * 3.0 + rho * 16.0 - time * 2.2));
        let ergo = exp(-pow(abs(length(vec3<f32>(p.x * 0.78, p.y * 1.15, p.z * 0.78)) - 0.28) * 52.0, 2.0)) * 0.55;
        col += vec3<f32>(1.36, 0.98, 0.34) * p_ring + vec3<f32>(1.28, 0.54, 0.18) * disk * doppler + vec3<f32>(0.34, 0.78, 1.32) * ergo;
        dens += dark_c * 2.4 + p_ring + disk * 0.92 + ergo * 0.62;
    } else if (zone == 8u) {
        // Z08: Binary BH Inspiral & Quadrupole Gravitational Wave Strain (Showcase 08b)
        let sep = 0.18;
        let ang = time * 2.6;
        let bh1 = vec3<f32>(cos(ang) * sep, 0.0, sin(ang) * sep);
        let bh2 = -bh1;
        let d_bh = min(length(p - bh1), length(p - bh2));
        let rim_glow = exp(-pow(abs(d_bh - 0.08) * 78.0, 2.0));
        let dark_core = smoothstep(0.082, 0.068, d_bh);
        col += vec3<f32>(1.38, 0.94, 0.34) * rim_glow;
        dens += rim_glow + dark_core * 2.2;
        let r_xz = length(p.xz);
        if (r_xz > 0.16 && r_xz < 0.88 && abs(p.y) < 0.12) {
            let phi = atan2(p.z, p.x);
            let spiral = cos(2.0 * (phi - r_xz * 9.5 + time * 2.6));
            let strain_wave = pow(max(spiral, 0.0), 4.0) * exp(-pow((p.y - 0.03 * spiral) * 44.0, 2.0)) * smoothstep(0.88, 0.22, r_xz);
            let outer_ring = exp(-pow(abs(r_xz - 0.52) * 58.0, 2.0)) * exp(-pow(p.y * 42.0, 2.0)) * 0.35;
            col += vec3<f32>(0.24, 0.92, 1.38) * (strain_wave * 1.32 + outer_ring);
            dens += strain_wave + outer_ring * 0.5;
        }
        if (feat == 4u) {
            let ligo = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, 0.22, 0.0), vec3<f32>(0.54, 0.22, 0.0), 0.008), 78.0)
                     + doc_line_glow(sd_capsule(p, vec3<f32>(0.0, 0.22, 0.0), vec3<f32>(0.0, 0.22, 0.54), 0.008), 78.0);
            col += vec3<f32>(1.32, 0.36, 0.32) * ligo;
            dens += ligo * 0.82;
        } else if (feat == 6u) {
            let jet = exp(-pow(length(p.xz) * 28.0, 2.0)) * smoothstep(0.56, 0.10, abs(p.y));
            col += vec3<f32>(1.32, 0.98, 0.28) * jet;
            dens += jet * 0.86;
        }
    } else if (zone == 9u || zone == 10u) {
        // Z09 FLRW Expanding Metric Grid & Redshift | Z10 Causal Horizons, Bekenstein Area & ER=EPR
        if (zone == 9u) {
            let scale_k = 1.0 / (1.0 + 0.45 * clamp(p.y + 0.5, 0.0, 1.0));
            let gx = exp(-pow(abs(fract(p.x * 4.5 * scale_k) - 0.5) * 16.0, 2.0));
            let gz = exp(-pow(abs(fract(p.z * 4.5 * scale_k) - 0.5) * 16.0, 2.0));
            let mesh = max(gx, gz) * smoothstep(0.72, 0.22, r);
            let rs_c = mix(vec3<f32>(0.24, 0.88, 1.34), vec3<f32>(1.32, 0.42, 0.24), clamp(p.y + 0.5, 0.0, 1.0));
            col += rs_c * mesh * 0.78;
            dens += mesh * 0.72;
        } else {
            let veil = exp(-pow(abs(r - 0.36) * 48.0, 2.0));
            let tiles = exp(-pow(abs(sin(p.y * 22.0)) * 10.0, 2.0)) * exp(-pow(abs(sin(atan2(p.z, p.x) * 10.0)) * 10.0, 2.0));
            let pair_h = doc_line_glow(sd_capsule(p, vec3<f32>(0.30, 0.0, 0.0), vec3<f32>(0.48, 0.14, 0.0), 0.009), 76.0);
            col += vec3<f32>(0.28, 0.92, 1.32) * veil * (0.45 + 0.65 * tiles) + vec3<f32>(1.32, 0.72, 0.24) * pair_h;
            dens += veil * 0.78 + pair_h * 0.82;
        }
    } else if (zone == 11u) {
        // Z11: Planck Spin-Foam & Calabi-Yau String Loops (Showcase 08)
        let p_f = p * 3.2;
        let cell = fract(p_f + 0.5) - 0.5;
        let strut_oct = min(min(length(cell.xy), length(cell.yz)), length(cell.zx));
        let strut_diag = min(length(cell - vec3<f32>(cell.x + cell.y + cell.z) * 0.3333), strut_oct);
        let node_dot = exp(-pow(length(cell) * 14.0, 2.0)) * 0.8;
        let edge_mask = smoothstep(0.76, 0.28, r);
        let foam_g = (exp(-pow(strut_diag * 46.0, 2.0)) + node_dot) * edge_mask;
        col += vec3<f32>(0.44, 0.64, 1.38) * foam_g * 0.92;
        dens += foam_g * 0.70;
        let a_xz = atan2(p.z, p.x);
        let cy_r = 0.36 + 0.09 * sin(a_xz * 5.0 + time * 1.4);
        let cy_r2 = 0.24 + 0.06 * cos(a_xz * 3.0 - time * 1.2);
        let cy_g = doc_line_glow(abs(length(vec2<f32>(length(p.xz) - cy_r, p.y - 0.11 * sin(a_xz * 3.0 - time)))), 82.0)
                 + doc_line_glow(abs(length(vec2<f32>(length(p.xy) - cy_r2, p.z + 0.08 * cos(a_xz * 4.0)))), 82.0);
        col += vec3<f32>(1.36, 0.88, 0.32) * cy_g * 1.15;
        dens += cy_g * 0.88;
    } else if (zone == 12u) {
        // Z12: Quantum Gravity Frameworks — Closed String Loops, Calabi-Yau, LQG Spin Network, CDT
        let loop_s = doc_line_glow(sd_torus(doc_rot_y(p, time * 0.4), 0.36 + 0.05 * sin(atan2(p.z, p.x) * 4.0), 0.012), 76.0);
        let sn_node = exp(-pow(r * 16.0, 2.0));
        let cy = exp(-pow(abs(sd_octahedron(p, 0.44)) * 42.0, 2.0));
        col += vec3<f32>(1.32, 0.92, 0.28) * loop_s + vec3<f32>(0.32, 0.96, 1.34) * (sn_node + cy * 0.72);
        dens += loop_s * 0.86 + sn_node * 0.78 + cy * 0.68;
    } else if (zone == 13u) {
        // Z13: Emergent Spacetime — AdS/CFT Holographic Cylinder & MERA Entanglement Web
        let rho = length(p.xz);
        let bnd_cft = exp(-pow(abs(rho - 0.48) * 52.0, 2.0)) * smoothstep(0.56, 0.24, abs(p.y));
        let rt_arch = doc_line_glow(sd_torus(doc_rot_x(p - vec3<f32>(0.0, 0.0, 0.26), 1.5708), 0.24, 0.01), 78.0);
        let mera = exp(-pow(abs(rho - (0.48 - abs(p.y) * 0.65)) * 46.0, 2.0)) * smoothstep(0.52, 0.14, abs(p.y));
        col += vec3<f32>(0.24, 1.08, 1.34) * bnd_cft + vec3<f32>(1.32, 0.94, 0.28) * (rt_arch + mera * 0.65);
        dens += bnd_cft * 0.80 + (rt_arch + mera * 0.65) * 0.82;
    } else {
        // Z14: Quantum Origin — Hartle-Hawking No-Boundary Rounded Cup & LQC Big Bounce
        let rho = length(p.xz);
        let hh_wall = exp(-pow(abs(rho - (0.16 + 0.44 * smoothstep(-0.46, 0.48, p.y))) * 46.0, 2.0)) * smoothstep(0.58, 0.22, abs(p.y));
        let cup_base = exp(-pow(abs(length(p - vec3<f32>(0.0, -0.34, 0.0)) - 0.16) * 46.0, 2.0)) * select(0.0, 1.0, p.y < -0.32);
        let bounce_c = exp(-pow(length(p - vec3<f32>(0.0, -0.02, 0.0)) * 18.0, 2.0));
        col += vec3<f32>(0.34, 0.88, 1.34) * (hh_wall + cup_base) + vec3<f32>(1.34, 1.02, 0.36) * bounce_c;
        dens += (hh_wall + cup_base) * 0.82 + bounce_c;
    }

    if (has_f) {
        let fh = mm_feat_halo(p_raw, feat, vec3<f32>(1.32, 0.96, 0.38), time);
        col += fh.rgb * (0.45 + 0.55 * w);
        dens += fh.a * (0.45 + 0.55 * w);
    }
    let pulse = 1.0 + bass * 0.14 + voice * 0.10 + cue * 0.08;
    return vec4<f32>(col * pulse, dens);
}

fn march_doc_matter_spacetime(
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
    let cue = doc_cue(inst);
    let feat = doc_feat(inst);
    let weight = doc_weight(inst);

    let is_canon = doc_is_canonical_zone_vid(vid);
    let canon_exp = select(0u, doc_vid_exp(vid), is_canon);
    let canon_zone = select(zone, doc_vid_zone(vid), is_canon);
    let is_spacetime = (vid == 2u) || (vid == 14u) || (vid >= 20u && vid < 30u) || (is_canon && canon_exp == 14u);

    let steps: i32 = i32(clamp(26.0 * fovea_k, 18.0, 28.0));
    let dt = (t1 - t0) / f32(steps);
    var t = t0 + dt * jitter;
    var accum = vec3<f32>(0.0);
    var trans: f32 = 1.0;

    for (var i = 0; i < 28; i = i + 1) {
        if (i >= steps || trans < 0.03) { break; }
        let p_raw = ro + rd * t;
        let r = length(p_raw);
        if (r < 0.94) {
            let p = doc_rot_y(p_raw, time * 0.14);
            var col = vec3<f32>(0.0);
            var dens: f32 = 0.0;

            if (is_canon && canon_exp == 13u) {
                let st = shade_doc_matter_energy_stage(canon_zone, feat, p_raw, p, r, cue, weight, time, voice, bass);
                col = st.rgb;
                dens = st.a;
            } else if (is_canon && canon_exp == 14u) {
                let st = shade_doc_spacetime_stage(canon_zone, feat, p_raw, p, r, cue, weight, time, voice, bass);
                col = st.rgb;
                dens = st.a;
            } else if (!is_spacetime) {
                // ---------------- EXP 13: MATTER & ENERGY ----------------
                let canon_m = select(u32(max(scene, 0)), canon_zone % 5u, scene < 0);
                let m_mode = select(select(u32(max(scene, 0)), canon_m, is_canon), vid - 10u, vid >= 10u && vid < 20u);
                if (m_mode == 0u) {
                    // Crisp glossy H2O bent molecule + Hexagonal Benzene C6H6 ring & aromatic pi clouds
                    let po = p - vec3<f32>(0.0, 0.25, 0.0);
                    let ph1 = p - vec3<f32>(-0.22, 0.08, 0.0);
                    let ph2 = p - vec3<f32>(0.22, 0.08, 0.0);
                    let key_l = normalize(vec3<f32>(0.45, 0.72, 0.55));
                    let shade_o = 0.35 + 0.85 * max(dot(normalize(po + vec3<f32>(1e-4)), key_l), 0.0);
                    let shade_h1 = 0.35 + 0.85 * max(dot(normalize(ph1 + vec3<f32>(1e-4)), key_l), 0.0);
                    let shade_h2 = 0.35 + 0.85 * max(dot(normalize(ph2 + vec3<f32>(1e-4)), key_l), 0.0);
                    let g_o = exp(-pow(max(length(po) - 0.105, 0.0) * 68.0, 2.0));
                    let g_h1 = exp(-pow(max(length(ph1) - 0.062, 0.0) * 76.0, 2.0));
                    let g_h2 = exp(-pow(max(length(ph2) - 0.062, 0.0) * 76.0, 2.0));
                    let bond1 = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, 0.25, 0.0), vec3<f32>(-0.22, 0.08, 0.0), 0.011), 96.0);
                    let bond2 = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, 0.25, 0.0), vec3<f32>(0.22, 0.08, 0.0), 0.011), 96.0);
                    col += vec3<f32>(1.45, 0.24, 0.26) * g_o * shade_o
                         + vec3<f32>(0.88, 1.18, 1.48) * (g_h1 * shade_h1 + g_h2 * shade_h2 + bond1 + bond2);
                    dens += g_o + g_h1 + g_h2 + (bond1 + bond2) * 0.85;

                    // Lower planar Benzene C6H6 hexagonal frame + 6 Carbon spheres + twin pi clouds
                    let pb = doc_rot_x(p - vec3<f32>(0.0, -0.24, 0.0), 0.35);
                    let hex_ring = abs(max(abs(pb.x) * 0.8660254 + abs(pb.z) * 0.5, abs(pb.z)) - 0.34);
                    let hex_edge = exp(-pow(hex_ring * 78.0, 2.0)) * exp(-pow(pb.y * 42.0, 2.0));
                    let inner_ar = doc_line_glow(sd_torus(pb, 0.24, 0.008), 86.0);
                    let pi_up = doc_line_glow(sd_torus(pb - vec3<f32>(0.0, 0.09, 0.0), 0.27, 0.008), 76.0);
                    let pi_dn = doc_line_glow(sd_torus(pb + vec3<f32>(0.0, 0.09, 0.0), 0.27, 0.008), 76.0);
                    var c_nodes: f32 = 0.0;
                    for (var v = 0; v < 6; v = v + 1) {
                        let va = f32(v) * TAU / 6.0;
                        let vp = vec3<f32>(cos(va) * 0.34, 0.0, sin(va) * 0.34);
                        c_nodes += exp(-pow(max(length(pb - vp) - 0.036, 0.0) * 85.0, 2.0));
                    }
                    col += vec3<f32>(0.24, 1.25, 1.42) * (hex_edge + c_nodes + inner_ar * 0.8)
                         + vec3<f32>(0.92, 0.45, 1.50) * (pi_up + pi_dn);
                    dens += hex_edge + c_nodes + inner_ar * 0.7 + (pi_up + pi_dn) * 0.75;
                } else if (m_mode == 1u) {
                    // Crisp Hydrogen 2p / 3d_z2 Electron Orbital Contour Lobes + Bohr Rings + Nucleus
                    let r_len = max(length(p), 1e-4);
                    let cos_th = p.y / r_len;
                    let ang_harm = select(
                        abs(3.0 * cos_th * cos_th - 1.0) * 0.45,
                        abs(cos_th),
                        (zone % 2u) == 0u
                    );
                    let lobe_r = 0.44 * ang_harm;
                    let lobe_shell = exp(-pow(abs(r_len - lobe_r) * 58.0, 2.0)) * smoothstep(0.06, 0.22, lobe_r);
                    let core_fill = exp(-pow((r_len - lobe_r * 0.65) * 14.0, 2.0)) * pow(ang_harm, 2.0) * 0.45;
                    let lobe_col = select(vec3<f32>(0.22, 0.98, 1.52), vec3<f32>(1.45, 0.42, 0.96), cos_th < 0.0);
                    let bohr_1 = doc_line_glow(sd_torus(doc_rot_x(p, 0.38), 0.56, 0.005), 96.0);
                    let bohr_2 = doc_line_glow(sd_torus(doc_rot_z(p, -0.52), 0.56, 0.005), 96.0);
                    let nuc = exp(-pow(r_len * 32.0, 2.0)) * 1.6;
                    col += lobe_col * (lobe_shell * 1.15 + core_fill)
                         + vec3<f32>(1.35, 1.05, 0.45) * (bohr_1 + bohr_2) * 0.65
                         + vec3<f32>(1.55, 1.30, 0.75) * nuc;
                    dens += lobe_shell * 0.95 + core_fill * 0.5 + (bohr_1 + bohr_2) * 0.55 + nuc;
                } else if (m_mode == 2u) {
                    // Proton / Quark Confinement Bag + Crisp 3 RGB Quarks + Y-Gluon Color Flux Springs
                    let q1 = vec3<f32>(0.26 * cos(time * 1.4), 0.12 * sin(time * 1.9), 0.26 * sin(time * 1.4));
                    let q2 = vec3<f32>(0.26 * cos(time * 1.4 + 2.094), -0.14 * cos(time * 1.7), 0.26 * sin(time * 1.4 + 2.094));
                    let q3 = vec3<f32>(0.26 * cos(time * 1.4 + 4.188), 0.10 * sin(time * 2.3), 0.26 * sin(time * 1.4 + 4.188));
                    let junc = (q1 + q2 + q3) * 0.18;
                    let g1 = exp(-pow(max(length(p - q1) - 0.075, 0.0) * 62.0, 2.0));
                    let g2 = exp(-pow(max(length(p - q2) - 0.075, 0.0) * 62.0, 2.0));
                    let g3 = exp(-pow(max(length(p - q3) - 0.075, 0.0) * 62.0, 2.0));
                    let f1 = doc_line_glow(sd_capsule(p, junc, q1, 0.012), 86.0);
                    let f2 = doc_line_glow(sd_capsule(p, junc, q2, 0.012), 86.0);
                    let f3 = doc_line_glow(sd_capsule(p, junc, q3, 0.012), 86.0);
                    let ring_c = doc_line_glow(sd_torus(doc_rot_x(p, 0.45), 0.56, 0.005), 88.0);
                    let bag = exp(-pow(abs(r - 0.56) * 68.0, 2.0)) * (0.28 + 0.18 * voice);
                    col += vec3<f32>(1.48, 0.22, 0.22) * (g1 + f1)
                         + vec3<f32>(0.22, 1.45, 0.38) * (g2 + f2)
                         + vec3<f32>(0.28, 0.62, 1.52) * (g3 + f3)
                         + vec3<f32>(0.78, 0.52, 1.25) * (bag + ring_c * 0.6);
                    dens += g1 + g2 + g3 + f1 + f2 + f3 + bag + ring_c * 0.5;
                } else if (m_mode == 3u) {
                    // Higgs Mexican-Hat Quartic Potential + Symmetry-Broken Boson Sphere
                    let rho = length(p.xz);
                    let hat_y = -0.34 + 1.35 * (pow(rho / 0.58, 4.0) - 0.78 * pow(rho / 0.58, 2.0));
                    if (rho < 0.72) {
                        let d_hat = abs(p.y - hat_y);
                        let ring_grid = exp(-pow(abs(fract(rho * 10.0) - 0.5) * 12.0, 2.0));
                        let radial_spoke = exp(-pow(abs(fract(atan2(p.z, p.x) / TAU * 12.0) - 0.5) * 14.0, 2.0));
                        let grid = max(ring_grid, radial_spoke * 0.7);
                        let surf_g = exp(-pow(d_hat * 45.0, 2.0)) * (0.22 + 0.95 * grid);
                        col += mix(vec3<f32>(0.25, 0.78, 1.35), vec3<f32>(1.42, 0.92, 0.24), smoothstep(0.1, 0.42, rho)) * surf_g;
                        dens += surf_g * 0.85;
                    }
                    let min_r = 0.36;
                    let b_ang = time * 0.9;
                    let boson_p = vec3<f32>(cos(b_ang) * min_r, -0.54 + 0.055, sin(b_ang) * min_r);
                    let b_glow = exp(-pow(length(p - boson_p) * 20.0, 2.0));
                    col += vec3<f32>(1.50, 1.18, 0.42) * b_glow;
                    dens += b_glow;
                } else {
                    // Pair Production / Noether Symmetry Torus + Propagator Arcs
                    let d_tor = sd_torus(p, 0.42, 0.14);
                    let wire = exp(-pow(abs(fract(atan2(p.z, p.x) * 3.0) - 0.5) * 12.0, 2.0));
                    let tg = exp(-pow(abs(d_tor) * 48.0, 2.0)) * (0.28 + 0.9 * wire);
                    col += vec3<f32>(0.35, 1.05, 1.45) * tg;
                    dens += tg * 0.85;
                    let gamma_a = doc_line_glow(sd_capsule(p, vec3<f32>(0.0), vec3<f32>(0.55, 0.42, 0.0), 0.008), 66.0);
                    let gamma_b = doc_line_glow(sd_capsule(p, vec3<f32>(0.0), vec3<f32>(-0.55, -0.42, 0.0), 0.008), 66.0);
                    col += vec3<f32>(1.45, 0.88, 0.28) * (gamma_a + gamma_b);
                    dens += (gamma_a + gamma_b) * 0.8;
                }
            } else {
                // ---------------- EXP 14: SPACETIME ----------------
                let canon_s = select(u32(max(scene, 0)), canon_zone % 4u, scene < 0);
                let s_mode = select(select(u32(max(scene, 0)), canon_s, is_canon), vid - 20u, vid >= 20u && vid < 30u);
                if (s_mode == 0u) {
                    // Minkowski Hourglass Light Cones + Worldline + Null Geodesic
                    let r_xz = length(p.xz);
                    let cone_d = abs(r_xz - abs(p.y) * 0.88);
                    let in_h = smoothstep(0.68, 0.62, abs(p.y));
                    let lon = exp(-pow(abs(fract(atan2(p.z, p.x) / TAU * 12.0) - 0.5) * 14.0, 2.0));
                    let lat = exp(-pow(abs(fract(p.y * 5.0) - 0.5) * 14.0, 2.0));
                    let cone_g = exp(-pow(cone_d * 52.0, 2.0)) * in_h * (0.25 + 0.9 * max(lon, lat));
                    let cone_c = select(vec3<f32>(0.35, 0.85, 1.45), vec3<f32>(1.42, 0.78, 0.28), p.y < 0.0);
                    col += cone_c * cone_g;
                    dens += cone_g * 0.88;
                    let origin_g = exp(-pow(r * 22.0, 2.0)) * (1.0 + 0.4 * cue);
                    let worldline = doc_line_glow(sd_capsule(p, vec3<f32>(-0.18, -0.62, 0.0), vec3<f32>(0.22, 0.62, 0.08 * sin(time)), 0.008), 68.0);
                    col += vec3<f32>(1.48, 1.38, 1.15) * origin_g + vec3<f32>(0.35, 1.45, 0.92) * worldline;
                    dens += origin_g + worldline * 0.85;
                } else if (s_mode == 1u) {
                    // Flamm's Paraboloid Gravity Well + Mercury Rosette Precession Orbit
                    let rho = length(p.xz);
                    let throat_r = 0.14;
                    let flamm_y = -0.52 + 0.54 * sqrt(max(rho - throat_r, 0.0));
                    if (rho > throat_r - 0.02 && rho < 0.78) {
                        let d_well = abs(p.y - flamm_y);
                        let grid_r = exp(-pow(abs(fract(rho * 9.0) - 0.5) * 13.0, 2.0));
                        let grid_a = exp(-pow(abs(fract(atan2(p.z, p.x) / TAU * 16.0) - 0.5) * 14.0, 2.0));
                        let wg = exp(-pow(d_well * 52.0, 2.0)) * (0.18 + 0.95 * max(grid_r, grid_a));
                        col += mix(vec3<f32>(1.38, 0.52, 0.20), vec3<f32>(0.28, 0.82, 1.42), smoothstep(0.16, 0.55, rho)) * wg;
                        dens += wg * 0.9;
                    }
                    let a_m = atan2(p.z, p.x);
                    let r_prec = 0.42 + 0.11 * cos(3.0 * a_m - time * 0.8);
                    let d_orb = length(vec2<f32>(rho - r_prec, p.y - 0.12));
                    let og = exp(-pow(d_orb * 48.0, 2.0));
                    col += vec3<f32>(1.45, 1.12, 0.42) * og;
                    dens += og * 0.8;
                } else if (s_mode == 2u) {
                    // Binary Black Hole Inspiral & High-Contrast Quadrupole Gravitational Wave Strain
                    let sep = 0.18;
                    let ang = time * 2.6;
                    let bh1 = vec3<f32>(cos(ang) * sep, 0.0, sin(ang) * sep);
                    let bh2 = -bh1;
                    let d_bh = min(length(p - bh1), length(p - bh2));
                    let rim_glow = exp(-pow(abs(d_bh - 0.08) * 78.0, 2.0));
                    let dark_core = smoothstep(0.082, 0.068, d_bh);
                    col += vec3<f32>(1.52, 0.98, 0.35) * rim_glow;
                    dens += rim_glow + dark_core * 2.2;
                    let r_xz = length(p.xz);
                    if (r_xz > 0.16 && r_xz < 0.88 && abs(p.y) < 0.12) {
                        let phi = atan2(p.z, p.x);
                        let spiral = cos(2.0 * (phi - r_xz * 9.5 + time * 2.6));
                        let strain_wave = pow(max(spiral, 0.0), 4.0) * exp(-pow((p.y - 0.03 * spiral) * 44.0, 2.0)) * smoothstep(0.88, 0.22, r_xz);
                        let outer_ring = exp(-pow(abs(r_xz - 0.52) * 58.0, 2.0)) * exp(-pow(p.y * 42.0, 2.0)) * 0.35;
                        col += vec3<f32>(0.24, 0.95, 1.52) * (strain_wave * 1.45 + outer_ring);
                        dens += strain_wave + outer_ring * 0.5;
                    }
                } else {
                    // Bold 3D Spin-Foam Tetrahedral Lattice + Luminous Calabi-Yau String Loops
                    let p_f = p * 3.2;
                    let cell = fract(p_f + 0.5) - 0.5;
                    let strut_oct = min(min(length(cell.xy), length(cell.yz)), length(cell.zx));
                    let strut_diag = min(length(cell - vec3<f32>(cell.x + cell.y + cell.z) * 0.3333), strut_oct);
                    let node_dot = exp(-pow(length(cell) * 14.0, 2.0)) * 0.8;
                    let edge_mask = smoothstep(0.76, 0.28, r);
                    let foam_g = (exp(-pow(strut_diag * 46.0, 2.0)) + node_dot) * edge_mask;
                    col += vec3<f32>(0.45, 0.65, 1.52) * foam_g * 0.95;
                    dens += foam_g * 0.72;
                    let a_xz = atan2(p.z, p.x);
                    let cy_r = 0.36 + 0.09 * sin(a_xz * 5.0 + time * 1.4);
                    let cy_r2 = 0.24 + 0.06 * cos(a_xz * 3.0 - time * 1.2);
                    let cy_g = doc_line_glow(abs(length(vec2<f32>(length(p.xz) - cy_r, p.y - 0.11 * sin(a_xz * 3.0 - time)))), 82.0)
                             + doc_line_glow(abs(length(vec2<f32>(length(p.xy) - cy_r2, p.z + 0.08 * cos(a_xz * 4.0)))), 82.0);
                    col += vec3<f32>(1.48, 0.92, 0.32) * cy_g * 1.25;
                    dens += cy_g * 0.92;
                }
            }

            let alpha_s = clamp(dens * dt * 3.5, 0.0, 0.74);
            accum += trans * col * (dt * 3.5);
            trans *= (1.0 - alpha_s);
        }
        t += dt;
    }
    return vec4<f32>(accum, 1.0 - trans);
}

// ============================================================================
// EXP 15 (COMPLEXITY), 16 (LIFE) & 17 (MIND / SCULPTED BRAIN) — kind == 8u
// ============================================================================
fn mm_brain_sdf(p: vec3<f32>) -> vec4<f32> {
    let px = abs(p.x);
    let fissure_carve = 0.028 * exp(-pow(px * 42.0, 2.0)) * smoothstep(-0.08, 0.16, p.y);

    let p_hemi = vec3<f32>((px - 0.14) / 0.27, (p.y - 0.06) / 0.29, (p.z - 0.02) / 0.44);
    var d_cerebrum = (length(p_hemi) - 1.0) * 0.26 + fissure_carve;

    let p_temp = vec3<f32>((px - 0.24) / 0.14, (p.y + 0.12) / 0.11, (p.z - 0.07) / 0.24);
    let d_temp = (length(p_temp) - 1.0) * 0.11;
    d_cerebrum = min(d_cerebrum, d_temp);

    let fold = sin(p.x * 26.0 + sin(p.z * 18.0) * 1.4)
             * cos(p.y * 24.0 + cos(p.x * 19.0) * 1.3)
             * sin(p.z * 25.0 + cos(p.y * 20.0) * 1.2);
    d_cerebrum += fold * 0.011;

    let p_cb = vec3<f32>((px - 0.11) / 0.16, (p.y + 0.25) / 0.11, (p.z + 0.25) / 0.14);
    let cb_folia = 0.008 * sin(p.y * 85.0);
    let d_cerebellum = (length(p_cb) - 1.0) * 0.11 + cb_folia;

    let stem_d = sd_capsule(p, vec3<f32>(0.0, -0.14, -0.04), vec3<f32>(0.0, -0.52, -0.13), 0.068);

    let d = min(min(d_cerebrum, d_cerebellum), stem_d);
    var reg: f32 = 0.0;
    if (d_cerebellum < d_cerebrum && d_cerebellum <= stem_d) {
        reg = 3.0;
    } else if (stem_d < d_cerebrum) {
        reg = 3.0;
    } else if (p.z < -0.16) {
        reg = 1.0;
    } else if (d_temp < d_cerebrum + 0.03) {
        reg = 2.0;
    }
    return vec4<f32>(d, reg, fold, 0.0);
}

fn march_doc_bio_mind(
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
    let cue = doc_cue(inst);

    let is_canon = doc_is_canonical_zone_vid(vid);
    let canon_exp = select(0u, doc_vid_exp(vid), is_canon);
    let legacy_group = select(
        select(0u, 1u, vid == 2u || vid == 16u || (vid >= 20u && vid < 30u)),
        2u,
        vid == 3u || vid == 17u || (vid >= 30u && vid < 40u)
    );
    let mode_group = select(
        legacy_group,
        select(select(0u, 1u, canon_exp == 16u), 2u, canon_exp >= 17u),
        is_canon
    );

    let steps: i32 = i32(clamp(42.0 * fovea_k, 24.0, 46.0));
    let dt = (t1 - t0) / f32(steps);
    var t = t0 + dt * jitter;
    var accum = vec3<f32>(0.0);
    var trans: f32 = 1.0;

    for (var i = 0; i < 46; i = i + 1) {
        if (i >= steps || trans < 0.03) { break; }
        let p_raw = ro + rd * t;
        let r = length(p_raw);
        if (r < 0.94) {
            let p = doc_rot_y(p_raw, time * 0.14);
            var col = vec3<f32>(0.0);
            var dens: f32 = 0.0;

            if (mode_group == 0u) {
                // ---------------- EXP 15: COMPLEXITY ----------------
                let c_mode = select(u32(max(scene, 0)), vid - 10u, vid >= 10u && vid < 20u);
                if (c_mode == 0u) {
                    // Crisp Sweeping 3D Lorenz Strange Attractor Butterfly Wings (XY-leaning toward viewer)
                    let lobe_l = vec3<f32>(-0.26, 0.06, 0.0);
                    let lobe_r = vec3<f32>(0.26, -0.04, 0.0);
                    let pl = doc_rot_x(doc_rot_y(p - lobe_l, 0.35), 0.25);
                    let pr = doc_rot_x(doc_rot_y(p - lobe_r, -0.35), -0.25);
                    let r1 = length(pl.xy);
                    let r2 = length(pr.xy);
                    var scroll1: f32 = 0.0;
                    var scroll2: f32 = 0.0;
                    for (var ring = 0; ring < 3; ring = ring + 1) {
                        let rr = 0.14 + f32(ring) * 0.085;
                        scroll1 += exp(-pow(abs(r1 - rr) * 62.0, 2.0)) * exp(-pow(pl.z * 32.0, 2.0));
                        scroll2 += exp(-pow(abs(r2 - rr) * 62.0, 2.0)) * exp(-pow(pr.z * 32.0, 2.0));
                    }
                    let saddle_a = doc_line_glow(sd_capsule(p, vec3<f32>(-0.26, 0.22, 0.0), vec3<f32>(0.26, -0.20, 0.0), 0.008), 88.0);
                    let saddle_b = doc_line_glow(sd_capsule(p, vec3<f32>(-0.26, -0.16, 0.0), vec3<f32>(0.26, 0.18, 0.0), 0.008), 88.0);
                    let core_s = exp(-pow(length(p) * 18.0, 2.0));
                    col += vec3<f32>(0.18, 1.25, 1.48) * scroll1
                         + vec3<f32>(1.45, 0.42, 0.92) * scroll2
                         + vec3<f32>(1.42, 1.05, 0.35) * (saddle_a + saddle_b + core_s);
                    dens += scroll1 * 0.85 + scroll2 * 0.85 + saddle_a + saddle_b + core_s;
                } else if (c_mode == 1u) {
                    // Rayleigh-Benard Hexagonal Convection Cells & Reaction-Diffusion Turing Coral
                    let hex_p = vec3<f32>(p.x, p.y * 2.2, p.z) * 4.5;
                    let field_s = doc_sample_field_grid(fract(p.xz * 0.72 + vec2<f32>(0.5)));
                    let rd_pde = (field_s.r - field_s.g) * 0.55;
                    let rd_wave = sin(hex_p.x + cos(hex_p.z * 1.3 + time * 0.5) + rd_pde)
                                + sin(hex_p.z - cos(hex_p.x * 1.2 - time * 0.4) - rd_pde);
                    let coral = exp(-pow(abs(rd_wave) * 6.5, 2.0)) * exp(-pow(abs(r - 0.44) * 16.0, 2.0));
                    let plate_g = exp(-pow(abs(p.y + 0.32) * 32.0, 2.0)) * smoothstep(0.65, 0.3, length(p.xz));
                    col += mix(vec3<f32>(0.22, 1.18, 0.85), vec3<f32>(1.42, 0.65, 0.22), smoothstep(-0.3, 0.3, p.y)) * coral
                         + vec3<f32>(1.35, 0.42, 0.14) * plate_g * 0.7;
                    dens += coral * 0.95 + plate_g * 0.5;
                } else if (c_mode == 2u) {
                    // Eigen Hypercycle — 5 glowing catalytic reaction nodes & clockwise arrows
                    for (var k = 0; k < 5; k = k + 1) {
                        let a = f32(k) * TAU / 5.0 + time * 0.25;
                        let np = vec3<f32>(cos(a) * 0.42, sin(a * 2.0) * 0.08, sin(a) * 0.42);
                        let na = f32(k + 1) * TAU / 5.0 + time * 0.25;
                        let n_next = vec3<f32>(cos(na) * 0.42, sin(na * 2.0) * 0.08, sin(na) * 0.42);
                        let ng = exp(-pow(length(p - np) * 18.0, 2.0));
                        let edge = doc_line_glow(sd_capsule(p, np, n_next, 0.008), 68.0);
                        let n_col = hsl_to_rgb(f32(k) / 5.0, 0.92, 0.56);
                        col += n_col * (ng + edge * 0.85);
                        dens += ng + edge * 0.7;
                    }
                } else {
                    // Budding Lipid Protocell Membrane pinching into two daughter cells
                    let c1 = length(p - vec3<f32>(-0.22, 0.0, 0.0)) - 0.32;
                    let c2 = length(p - vec3<f32>(0.22, 0.0, 0.0)) - 0.28;
                    let d_ves = min(c1, c2);
                    let mem = exp(-pow(abs(d_ves) * 48.0, 2.0));
                    let pinch = doc_line_glow(sd_torus(vec3<f32>(p.y, p.x, p.z), 0.22, 0.008), 64.0);
                    col += vec3<f32>(0.28, 1.25, 0.72) * mem + vec3<f32>(1.45, 0.85, 0.25) * pinch;
                    dens += mem * 0.9 + pinch * 0.8;
                }
            } else if (mode_group == 1u) {
                // ---------------- EXP 16: LIFE ----------------
                let l_mode = select(u32(max(scene, 0)), vid - 20u, vid >= 20u && vid < 30u);
                if (l_mode == 0u) {
                    // Crisp B-DNA Double Helix + Complementary Base-Pair Rungs nested in faint Cell Membrane
                    if (abs(p.y) < 0.62 && length(p.xz) < 0.36) {
                        let theta = p.y * 8.2 + time * 0.6;
                        let r_h = 0.22;
                        let s1 = vec3<f32>(cos(theta) * r_h, p.y, sin(theta) * r_h);
                        let s2 = vec3<f32>(-cos(theta) * r_h, p.y, -sin(theta) * r_h);
                        let d1 = length(p - s1);
                        let d2 = length(p - s2);
                        let g1 = exp(-pow(d1 * 48.0, 2.0));
                        let g2 = exp(-pow(d2 * 48.0, 2.0));
                        let rung_phase = fract(p.y * 5.2 + 0.5) - 0.5;
                        let d_rung = length(vec2<f32>(rung_phase / 5.2, dot(p.xz, normalize(vec2<f32>(-sin(theta), cos(theta))))));
                        let in_cyl = smoothstep(r_h + 0.01, r_h - 0.02, length(p.xz));
                        let g_rung = exp(-pow(d_rung * 58.0, 2.0)) * in_cyl;
                        let rung_col = select(vec3<f32>(1.45, 0.78, 0.24), vec3<f32>(0.24, 1.42, 0.82), sin(theta * 2.0) > 0.0);
                        col += vec3<f32>(0.26, 0.95, 1.52) * g1 + vec3<f32>(0.96, 0.42, 1.48) * g2 + rung_col * g_rung;
                        dens += (g1 + g2 + g_rung) * 0.95;
                    }
                    let cell_shell = exp(-pow(abs(r - 0.68) * 72.0, 2.0)) * 0.32;
                    col += vec3<f32>(0.22, 1.15, 0.85) * cell_shell;
                    dens += cell_shell;
                } else if (l_mode == 1u) {
                    // Chloroplast Thylakoid Grana Stacks + Spinning ATP Synthase Turbine
                    let d_rotor = sd_cylinder_y(p, 0.14, 0.34);
                    let blade = exp(-pow(abs(sin(atan2(p.z, p.x) * 6.0 - time * 4.5)) * 12.0, 2.0)) * exp(-pow(abs(d_rotor) * 32.0, 2.0));
                    let disc_stack = exp(-pow(abs(fract(p.y * 8.0) - 0.5) * 14.0, 2.0)) * exp(-pow(abs(length(p.xz) - 0.36) * 28.0, 2.0)) * smoothstep(0.45, 0.1, abs(p.y));
                    col += vec3<f32>(1.42, 1.12, 0.28) * blade + vec3<f32>(0.18, 1.32, 0.52) * disc_stack;
                    dens += blade + disc_stack * 0.85;
                } else if (l_mode == 2u) {
                    // Folded Cristae Mitochondrion
                    let d_mem = length(vec3<f32>(p.x * 0.72, p.y * 1.35, p.z * 1.15)) - 0.44;
                    let cristae = 0.06 * sin(p.x * 26.0) * smoothstep(0.35, 0.0, abs(p.z));
                    let outer_g = exp(-pow(abs(d_mem) * 48.0, 2.0));
                    let inner_g = exp(-pow(abs(d_mem + 0.09 + cristae) * 42.0, 2.0));
                    col += vec3<f32>(0.32, 1.05, 1.38) * outer_g + vec3<f32>(1.45, 0.72, 0.24) * inner_g;
                    dens += outer_g * 0.7 + inner_g * 0.9;
                } else if (l_mode == 3u) {
                    // Crisp 3D Phylogenetic Tree of Life — LUCA root -> 3 Domains (Bacteria, Archaea, Eukarya)
                    let root = vec3<f32>(0.0, -0.46, 0.0);
                    let fork = vec3<f32>(0.0, -0.18, 0.0);
                    let luca = exp(-pow(length(p - root) * 24.0, 2.0)) * 1.4;
                    let trunk = doc_line_glow(sd_capsule(p, root, fork, 0.015), 90.0);
                    col += vec3<f32>(1.48, 1.12, 0.38) * (luca + trunk);
                    dens += luca + trunk * 0.9;
                    for (var dom = 0; dom < 3; dom = dom + 1) {
                        let da = f32(dom) * TAU / 3.0 + 0.35;
                        let mid = vec3<f32>(cos(da) * 0.24, 0.06, sin(da) * 0.24);
                        let tip_a = vec3<f32>(cos(da - 0.24) * 0.48, 0.42, sin(da - 0.24) * 0.48);
                        let tip_b = vec3<f32>(cos(da + 0.24) * 0.46, 0.38, sin(da + 0.24) * 0.46);
                        let b1 = doc_line_glow(sd_capsule(p, fork, mid, 0.011), 88.0);
                        let b2 = doc_line_glow(sd_capsule(p, mid, tip_a, 0.008), 92.0);
                        let b3 = doc_line_glow(sd_capsule(p, mid, tip_b, 0.008), 92.0);
                        let leaf = exp(-pow(min(length(p - tip_a), length(p - tip_b)) * 22.0, 2.0));
                        let dom_c = select(vec3<f32>(0.22, 1.38, 0.65), select(vec3<f32>(1.45, 0.68, 0.24), vec3<f32>(0.38, 0.92, 1.52), dom == 2), dom == 1);
                        col += dom_c * (b1 + b2 + b3 + leaf * 1.15);
                        dens += (b1 + b2 + b3 + leaf) * 0.9;
                    }
                    let ring_c = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.40, 0.0), 0.48, 0.004), 86.0);
                    col += vec3<f32>(0.45, 0.95, 1.25) * ring_c * 0.45;
                    dens += ring_c * 0.35;
                } else {
                    // Myelinated Axon + Saltatory Action Potential Pulse
                    let axon_d = doc_line_glow(sd_capsule(p, vec3<f32>(-0.62, 0.0, 0.0), vec3<f32>(0.62, 0.0, 0.0), 0.010), 60.0);
                    let pulse_x = fract(time * 0.55) * 1.24 - 0.62;
                    let spike = exp(-pow(abs(p.x - pulse_x) * 14.0, 2.0)) * exp(-pow(length(p.yz) * 24.0, 2.0));
                    let myelin = exp(-pow(abs(length(p.yz) - 0.058) * 45.0, 2.0)) * step(0.18, fract(p.x * 3.8)) * smoothstep(0.55, 0.1, abs(p.x));
                    col += vec3<f32>(0.24, 0.85, 1.38) * (axon_d + myelin * 0.6) + vec3<f32>(1.48, 1.18, 0.35) * spike;
                    dens += axon_d + myelin * 0.6 + spike;
                }
            } else {
                // ---------------- EXP 17: MIND — SCULPTED 3D ANATOMICAL CORTEX & CONNECTOME ----------------
                let m_sc = select(max(scene, 0), i32(vid - 30u), vid >= 30u && vid < 40u);
                let binfo = mm_brain_sdf(p);
                let bd = binfo.x;
                let reg = binfo.y;
                let fold = binfo.z;

                if (abs(bd) < 0.045) {
                    let shell_g = exp(-pow(abs(bd) * 58.0, 2.0));
                    let gyrus_shade = smoothstep(-0.35, 0.65, fold);
                    var region_col = vec3<f32>(0.36, 0.72, 1.25);
                    if (reg > 2.5) {
                        region_col = vec3<f32>(0.55, 0.42, 1.15);
                    } else if (reg > 1.5) {
                        region_col = vec3<f32>(0.24, 1.18, 0.88);
                    } else if (reg > 0.5) {
                        region_col = vec3<f32>(0.92, 0.55, 1.32);
                    }

                    var boost: f32 = 0.72;
                    if (m_sc == 0 && p.z < -0.10) { boost = 1.65; }
                    else if (m_sc == 1 && p.z > 0.12) { boost = 1.75; }
                    else if (m_sc == 2 && abs(reg - 2.0) < 0.5) { boost = 1.70; }
                    else if (m_sc == 3) {
                        boost = 1.2 + 0.6 * sin(time * 3.0 + p.z * 8.0);
                    } else if (m_sc >= 4) {
                        let gw_wave = exp(-pow(abs(p.z - sin(time * 1.8) * 0.35) * 10.0, 2.0));
                        boost = 0.8 + 1.1 * gw_wave;
                    }

                    col += region_col * shell_g * (0.35 + 0.85 * gyrus_shade) * boost * (1.0 + 0.25 * voice);
                    dens += shell_g * (0.45 + 0.55 * gyrus_shade);
                }

                // Corpus Callosum golden bridge & Bilateral Hippocampal C-Arcs
                let callosum_d = sd_torus(vec3<f32>(p.y - 0.02, p.x, p.z * 0.68), 0.16, 0.015);
                let callosum_g = exp(-pow(abs(callosum_d) * 52.0, 2.0)) * smoothstep(-0.08, 0.02, p.y);
                let hippo_p = vec3<f32>(abs(p.x) - 0.12, p.y + 0.06, p.z + 0.02);
                let hippo_g = exp(-pow(abs(length(hippo_p.yz) - 0.14) * 52.0, 2.0)) * exp(-pow(abs(hippo_p.x) * 34.0, 2.0));
                let limbic_glow = callosum_g + hippo_g * (1.0 + 0.6 * cue);
                col += vec3<f32>(1.45, 0.95, 0.32) * limbic_glow;
                dens += limbic_glow * 0.9;

                // Firing synaptic action-potential sparkles traveling along white-matter connectome tracts
                let tract = sin(p.x * 22.0 + time * 3.2) * cos(p.y * 22.0 - time * 2.5) * sin(p.z * 22.0 + time * 2.8);
                let synapse = pow(max(tract, 0.0), 10.0) * smoothstep(0.05, -0.08, bd);
                col += vec3<f32>(0.48, 1.35, 1.48) * synapse * (1.1 + 0.6 * voice);
                dens += synapse * 0.8;
            }

            let alpha_s = clamp(dens * dt * 3.5, 0.0, 0.74);
            accum += trans * col * (dt * 3.5);
            trans *= (1.0 - alpha_s);
        }
        t += dt;
    }
    return vec4<f32>(accum, 1.0 - trans);
}

// ============================================================================
// UNIFIED DISPATCHER CALLED BY hero_volumetric.wgsl
// ============================================================================
fn raymarch_doc_matter_mind(
    kind: u32,
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
    if (kind == 5u) {
        return march_doc_colors(vid, ro, rd, t0, t1, jitter, fovea_k, inst, time, voice, bass);
    } else if (kind == 6u) {
        return march_doc_music(vid, ro, rd, t0, t1, jitter, fovea_k, inst, time, voice, bass);
    } else if (kind == 7u) {
        return march_doc_matter_spacetime(vid, ro, rd, t0, t1, jitter, fovea_k, inst, time, voice, bass);
    }
    return march_doc_bio_mind(vid, ro, rd, t0, t1, jitter, fovea_k, inst, time, voice, bass);
}
