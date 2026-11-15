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

fn doc_complexity_feat_gain(feat: u32, idx: u32, cue: f32) -> f32 {
    if (feat == 0xFFu || feat >= 8u) {
        return 0.42;
    }
    if (feat == idx) {
        return 1.24 + 0.18 * sin(cue * TAU);
    }
    return 0.14;
}

fn shade_doc_complexity_stage(
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
    col_a: vec3<f32>,
    col_b: vec3<f32>,
) -> vec4<f32> {
    var col = vec3<f32>(0.0);
    var dens: f32 = 0.0;
    let ff = f32(feat & 7u);
    let has_f = feat < 8u;
    let accent_z = mix(vec3<f32>(0.28, 1.18, 0.88), clamp(col_b, vec3<f32>(0.18), vec3<f32>(1.24)), 0.45);
    let accent_p = clamp(col_a, vec3<f32>(0.22), vec3<f32>(1.24));

    if (zone == 0u) {
        // Z00: Foundations of Reality — 6-Ring Concentric Scale Ladder & Cross-Scale Emergence Filaments
        let g0 = doc_complexity_feat_gain(feat, 0u, cue);
        let g1 = doc_complexity_feat_gain(feat, 1u, cue);
        let g2 = doc_complexity_feat_gain(feat, 2u, cue);
        let g3 = doc_complexity_feat_gain(feat, 3u, cue);
        let g4 = doc_complexity_feat_gain(feat, 4u, cue);
        let g5 = doc_complexity_feat_gain(feat, 5u, cue);
        let spread = 1.0 + select(0.0, 0.18, feat == 3u);
        for (var k = 0; k < 6; k = k + 1) {
            let kf = f32(k);
            let y_lv = (-0.34 + kf * 0.136) * spread;
            let rad_lv = 0.15 + kf * 0.068;
            let ring_d = sd_torus(p - vec3<f32>(0.0, y_lv, 0.0), rad_lv, 0.011);
            let ring_g = doc_line_glow(ring_d, 74.0);
            let lv_tint = mix(vec3<f32>(0.26, 0.94, 1.24), vec3<f32>(1.24, 0.86, 0.28), kf / 5.0);
            let lv_boost = select(0.55, 1.35, (feat == 2u) && ((k % 2) == 0));
            col += lv_tint * ring_g * lv_boost * (0.65 + 0.35 * g2);
            dens += ring_g * 0.46;
        }
        let core_lat = exp(-pow(max(length(p - vec3<f32>(0.0, -0.34, 0.0)) - 0.09, 0.0) * 58.0, 2.0));
        col += accent_p * core_lat * (0.45 + 0.75 * g0);
        dens += core_lat * (0.45 + 0.45 * g0);

        let f_a = doc_line_glow(sd_capsule(p, vec3<f32>(-0.14, -0.34, 0.0), vec3<f32>(-0.44, 0.34, 0.0), 0.008), 84.0);
        let f_b = doc_line_glow(sd_capsule(p, vec3<f32>(0.14, -0.34, 0.0), vec3<f32>(0.44, 0.34, 0.0), 0.008), 84.0);
        col += vec3<f32>(0.38, 1.22, 0.94) * (f_a + f_b) * (0.45 + 0.85 * g1);
        dens += (f_a + f_b) * (0.35 + 0.45 * g1);

        if (feat == 3u || feat == 4u) {
            let p_x = doc_line_glow(sd_capsule(p, vec3<f32>(-0.22, -0.22, 0.12), vec3<f32>(0.22, -0.22, -0.12), 0.009), 76.0);
            col += vec3<f32>(1.22, 0.62, 0.28) * p_x * (0.5 + 0.6 * g3 + 0.7 * g4);
            dens += p_x * 0.55;
        } else if (feat == 5u) {
            let apex = doc_line_glow(sd_torus(doc_rot_x(p - vec3<f32>(0.0, 0.42, 0.0), 1.5708), 0.16, 0.012), 72.0);
            col += vec3<f32>(1.26, 1.04, 0.34) * apex * (0.65 + 0.65 * g5);
            dens += apex * 0.65;
        }
    } else if (zone == 1u) {
        // Z01: Physical Laws & Symmetries — Symmetric Polyhedral Phase Chamber + Twin Diverging Ribbons
        let p_c = doc_rot_y(p, select(0.0, ff * 0.52, feat == 2u) + time * 0.12);
        let cage = exp(-pow(abs(sd_octahedron(p_c, 0.54)) * 46.0, 2.0));
        let wire_box = exp(-pow(abs(sd_box(p_c, vec3<f32>(0.36))) * 54.0, 2.0));
        let wire_cyl = exp(-pow(abs(sd_cylinder_y(p_c, 0.36, 0.36)) * 52.0, 2.0));
        var chamb_m = cage;
        if (feat == 0u) { chamb_m = wire_box; }
        else if (feat == 6u) { chamb_m = wire_cyl; }
        col += mix(vec3<f32>(0.24, 0.96, 1.24), accent_z, 0.35) * chamb_m * 0.74;
        dens += chamb_m * 0.64;

        let span = smoothstep(0.50, 0.35, abs(p.y));
        let div_k = select(0.24, 0.42, feat == 5u);
        let r_path = (p.y + 0.42) * div_k;
        let rib_a = exp(-pow(length(p.xz - vec2<f32>(r_path, 0.12 * sin(p.y * 7.0 + time))) * 34.0, 2.0)) * span;
        let rib_b = exp(-pow(length(p.xz - vec2<f32>(-r_path, -0.12 * sin(p.y * 7.0 + time))) * 34.0, 2.0)) * span;
        col += vec3<f32>(1.24, 0.88, 0.28) * rib_a + vec3<f32>(0.32, 1.22, 0.86) * rib_b;
        dens += (rib_a + rib_b) * 0.80;

        if (feat == 1u) {
            let vox = doc_line_glow(sd_box(p, vec3<f32>(0.16)), 68.0);
            col += vec3<f32>(1.18, 0.56, 0.24) * vox;
            dens += vox * 0.58;
        } else if (feat == 3u) {
            let cons_a = doc_line_glow(sd_torus(doc_rot_x(p, 0.65), 0.32, 0.010), 76.0);
            let cons_b = doc_line_glow(sd_torus(doc_rot_z(p, 0.65), 0.32, 0.010), 76.0);
            col += vec3<f32>(1.26, 1.02, 0.32) * (cons_a + cons_b);
            dens += (cons_a + cons_b) * 0.65;
        } else if (feat == 4u) {
            let prob = exp(-pow(abs(r - 0.36) * 28.0, 2.0)) * (0.45 + 0.55 * abs(sin(p.y * 12.0)));
            col += vec3<f32>(0.68, 0.54, 1.24) * prob;
            dens += prob * 0.55;
        } else if (feat == 7u) {
            let p_cone = doc_line_glow(sd_cone_up(p - vec3<f32>(0.0, -0.34, 0.0), vec2<f32>(0.62, 0.78), 0.42), 68.0);
            col += vec3<f32>(0.36, 1.18, 1.28) * p_cone;
            dens += p_cone * 0.68;
        }
    } else if (zone == 2u) {
        // Z02: Spontaneous Symmetry Breaking — 3D Mexican-Hat Potential Well V(r) + Multi-Domain Spin Lattice
        let rho = length(p.xz);
        let r_n = rho * 1.45;
        let v_y = 0.22 - 1.15 * (r_n * r_n) + 1.42 * (r_n * r_n * r_n * r_n);
        let pot_shell = exp(-pow(abs(p.y - v_y) * 44.0, 2.0)) * smoothstep(0.68, 0.24, rho);
        let dom_idx = select(0.0, 1.0, p.x > 0.0) + select(0.0, 2.0, p.z > 0.0);
        var dom_c = vec3<f32>(0.24, 0.94, 1.24);
        if (dom_idx > 2.5) {
            dom_c = vec3<f32>(1.22, 0.78, 0.24);
        } else if (dom_idx > 1.5) {
            dom_c = vec3<f32>(0.28, 1.16, 0.72);
        } else if (dom_idx > 0.5) {
            dom_c = vec3<f32>(0.88, 0.44, 1.22);
        }
        let slab = exp(-pow(abs(p.y + 0.36) * 42.0, 2.0)) * smoothstep(0.64, 0.28, rho);
        col += mix(vec3<f32>(0.28, 1.04, 1.24), vec3<f32>(1.22, 0.82, 0.28), smoothstep(-0.18, 0.22, v_y)) * pot_shell * 0.85
             + dom_c * slab * select(0.42, 0.95, feat == 3u);
        dens += pot_shell * 0.74 + slab * 0.56;

        let probe_pos = select(vec3<f32>(0.36 * cos(time * 0.5), -0.02, 0.36 * sin(time * 0.5)), vec3<f32>(0.0, 0.22, 0.0), feat == 0u || feat == 5u);
        let probe = exp(-pow(max(length(p - probe_pos) - 0.068, 0.0) * 64.0, 2.0));
        col += vec3<f32>(1.28, 1.02, 0.34) * probe;
        dens += probe * 0.88;

        if (feat == 4u) {
            let wall_x = exp(-pow(abs(p.x) * 48.0, 2.0)) * smoothstep(0.56, 0.16, abs(p.z)) * smoothstep(0.45, 0.15, abs(p.y + 0.15));
            let wall_z = exp(-pow(abs(p.z) * 48.0, 2.0)) * smoothstep(0.56, 0.16, abs(p.x)) * smoothstep(0.45, 0.15, abs(p.y + 0.15));
            col += vec3<f32>(1.28, 0.32, 0.62) * (wall_x + wall_z);
            dens += (wall_x + wall_z) * 0.78;
        } else if (feat == 2u) {
            let front = exp(-pow(abs(rho - 0.36) * 44.0, 2.0)) * smoothstep(0.42, 0.10, abs(p.y));
            col += vec3<f32>(1.22, 0.58, 0.24) * front;
            dens += front * 0.64;
        } else if (feat == 5u) {
            let jit = pow(0.5 + 0.5 * sin(rho * 28.0 - time * 3.0), 6.0) * smoothstep(0.42, 0.06, rho) * exp(-pow((p.y - 0.18) * 18.0, 2.0));
            col += vec3<f32>(1.24, 0.84, 0.28) * jit;
            dens += jit * 0.76;
        } else if (feat == 6u) {
            let coll_r = doc_line_glow(sd_torus(p - vec3<f32>(0.0, -0.34, 0.0), 0.44, 0.014), 72.0);
            col += vec3<f32>(0.32, 1.24, 0.92) * coll_r;
            dens += coll_r * 0.78;
        }
    } else if (zone == 3u) {
        // Z03: Collective States of Matter — 4-Quadrant Condensed-Matter Stage (FCC / Liquid / Dipoles / Meissner Coil)
        let lat_c = fract(p * 3.8 + 0.5) - 0.5;
        let lat_n = exp(-pow(length(lat_c) * 14.0, 2.0)) * smoothstep(0.56, 0.18, r) * select(0.25, 1.0, p.x < 0.08 && p.y > -0.08);
        let flow_s = pow(0.5 + 0.5 * sin(p.x * 16.0 - time * 2.4 + cos(p.y * 8.0)), 6.0) * smoothstep(0.56, 0.18, r) * select(0.2, 1.0, p.x > -0.08);
        let meissner_r = doc_line_glow(sd_torus(p - vec3<f32>(0.22, -0.22, 0.0), 0.18, 0.018), 64.0);
        let dipole_b = doc_line_glow(sd_capsule(p, vec3<f32>(-0.34, -0.22, 0.0), vec3<f32>(-0.10, -0.22, 0.0), 0.014), 68.0);
        let g0 = doc_complexity_feat_gain(feat, 0u, cue);
        let g5 = doc_complexity_feat_gain(feat, 5u, cue);
        let g6 = doc_complexity_feat_gain(feat, 6u, cue);
        col += vec3<f32>(0.34, 1.12, 1.28) * lat_n * (0.55 + 0.65 * g0)
             + vec3<f32>(0.26, 1.22, 0.86) * flow_s * select(0.48, 1.15, feat == 1u)
             + vec3<f32>(0.32, 0.96, 1.28) * meissner_r * (0.55 + 0.75 * g6)
             + vec3<f32>(1.24, 0.72, 0.24) * dipole_b * (0.55 + 0.75 * g5);
        dens += lat_n * 0.68 + flow_s * 0.52 + meissner_r * 0.72 + dipole_b * 0.68;
        if (feat == 2u) {
            let jet = doc_line_glow(sd_capsule(p, vec3<f32>(0.02, 0.16, 0.0), vec3<f32>(0.42, 0.16, 0.0), 0.014), 72.0);
            col += vec3<f32>(0.28, 1.24, 1.18) * jet;
            dens += jet * 0.78;
        } else if (feat == 3u) {
            let wall = exp(-pow(abs(p.x - 0.44) * 48.0, 2.0)) * smoothstep(0.42, 0.12, length(p.yz));
            col += vec3<f32>(1.26, 0.46, 0.24) * wall;
            dens += wall * 0.72;
        } else if (feat == 4u) {
            let shear = exp(-pow(abs(p.y - 0.18 * sin(p.x * 5.0)) * 38.0, 2.0)) * smoothstep(0.46, 0.10, length(p.xz));
            col += vec3<f32>(0.88, 0.48, 1.24) * shear;
            dens += shear * 0.76;
        } else if (feat == 7u) {
            let bound = exp(-pow(abs(p.x) * 48.0, 2.0)) * smoothstep(0.56, 0.16, length(p.yz));
            col += vec3<f32>(1.24, 1.06, 0.36) * bound;
            dens += bound * 0.72;
        }
    } else if (zone == 4u) {
        // Z04: Far From Equilibrium — Lorenz Strange Attractor inside Dissipative Hot-to-Cold Carnot Frame
        let pl = doc_rot_x(doc_rot_y(p - vec3<f32>(-0.22, 0.04, 0.0), 0.35), 0.24);
        let pr = doc_rot_x(doc_rot_y(p - vec3<f32>(0.22, -0.04, 0.0), -0.35), -0.24);
        var s1: f32 = 0.0;
        var s2: f32 = 0.0;
        for (var ring = 0; ring < 3; ring = ring + 1) {
            let rr = 0.11 + f32(ring) * 0.072;
            s1 += exp(-pow(abs(length(pl.xy) - rr) * 58.0, 2.0)) * exp(-pow(pl.z * 32.0, 2.0));
            s2 += exp(-pow(abs(length(pr.xy) - rr) * 58.0, 2.0)) * exp(-pow(pr.z * 32.0, 2.0));
        }
        let throttle = select(1.0, 0.35 + 0.65 * (0.5 + 0.5 * sin(cue * TAU)), feat == 6u);
        let hot_plate = exp(-pow(abs(p.y + 0.44) * 38.0, 2.0)) * smoothstep(0.54, 0.18, length(p.xz));
        let cold_plate = exp(-pow(abs(p.y - 0.44) * 38.0, 2.0)) * smoothstep(0.54, 0.18, length(p.xz));
        col += vec3<f32>(0.22, 1.12, 1.28) * s1 * throttle + vec3<f32>(1.24, 0.46, 0.86) * s2 * throttle
             + vec3<f32>(1.24, 0.44, 0.18) * hot_plate + vec3<f32>(0.24, 1.22, 0.88) * cold_plate;
        dens += (s1 + s2) * 0.68 * throttle + (hot_plate + cold_plate) * 0.62;
        if (feat == 0u || feat == 3u) {
            let slice_r = exp(-pow(abs(p.y - select(0.14, -0.18, feat == 0u)) * 38.0, 2.0)) * smoothstep(0.46, 0.10, length(p.xz));
            col += vec3<f32>(1.24, 0.92, 0.32) * slice_r;
            dens += slice_r * 0.72;
        } else if (feat == 1u || feat == 2u) {
            let plume = exp(-pow(length(p.xz) * select(14.0, 8.0, feat == 2u), 2.0)) * smoothstep(0.46, 0.14, abs(p.y));
            col += vec3<f32>(1.22, 0.68, 0.24) * plume;
            dens += plume * 0.64;
        } else if (feat == 4u || feat == 5u) {
            let ent = exp(-pow(abs(r - select(0.56, 0.42, feat == 5u)) * 40.0, 2.0));
            col += vec3<f32>(1.18, 0.36, 0.22) * ent;
            dens += ent * 0.66;
        }
    } else if (zone == 5u) {
        // Z05: Fluctuations — Stochastic Curl-Noise Mist & Double-Bifurcation Tree
        let trunk = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, -0.44, 0.0), vec3<f32>(0.0, -0.10, 0.0), 0.016), 72.0);
        let bl = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, -0.10, 0.0), vec3<f32>(-0.24, 0.18, 0.04), 0.013), 72.0);
        let br = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, -0.10, 0.0), vec3<f32>(0.24, 0.18, -0.04), 0.013), 72.0);
        let sub_a = doc_line_glow(sd_capsule(p, vec3<f32>(-0.24, 0.18, 0.04), vec3<f32>(-0.42, 0.44, 0.10), 0.010), 74.0);
        let sub_b = doc_line_glow(sd_capsule(p, vec3<f32>(-0.24, 0.18, 0.04), vec3<f32>(-0.10, 0.44, -0.06), 0.010), 74.0);
        let sub_c = doc_line_glow(sd_capsule(p, vec3<f32>(0.24, 0.18, -0.04), vec3<f32>(0.10, 0.44, 0.06), 0.010), 74.0);
        let sub_d = doc_line_glow(sd_capsule(p, vec3<f32>(0.24, 0.18, -0.04), vec3<f32>(0.42, 0.44, -0.10), 0.010), 74.0);
        let tree_g = trunk + bl + br + sub_a + sub_b + sub_c + sub_d;
        let haze = pow(max(0.0, 0.5 + 0.5 * snoise3(p * 11.0 + vec3<f32>(0.0, time * 1.4, 0.0))), 6.0) * smoothstep(0.62, 0.18, r);
        col += vec3<f32>(0.28, 1.18, 0.94) * tree_g + vec3<f32>(1.18, 0.72, 0.28) * haze * select(0.45, 1.15, feat == 0u || feat == 5u);
        dens += tree_g * 0.78 + haze * 0.52;
        if (feat == 1u || feat == 2u) {
            let delta = doc_line_glow(sd_capsule(p, vec3<f32>(0.0, -0.28, 0.0), vec3<f32>(select(0.28, 0.42, feat == 2u), 0.34, 0.0), 0.012), 74.0);
            col += vec3<f32>(1.24, 0.94, 0.32) * delta;
            dens += delta * 0.78;
        } else if (feat == 4u) {
            let seed_f = exp(-pow(length(p - vec3<f32>(0.0, -0.10, 0.0)) * 18.0, 2.0));
            col += vec3<f32>(1.28, 1.04, 0.36) * seed_f;
            dens += seed_f * 0.85;
        } else if (feat == 3u) {
            let hop = exp(-pow(length(p - vec3<f32>(0.24 * sin(time * 3.0), 0.26, 0.0)) * 18.0, 2.0));
            col += vec3<f32>(1.22, 0.44, 0.92) * hop;
            dens += hop * 0.82;
        }
    } else if (zone == 6u) {
        // Z06: Spontaneous Patterns — Multi-Pattern Morphogenesis Specimen Table sampling Gray-Scott field_grid_tex
        let field_s = doc_sample_field_grid(fract(p.xz * 0.74 + vec2<f32>(0.5)));
        let rd_pde = field_s.r - field_s.g;
        let plate = exp(-pow(abs(p.y + 0.28) * 44.0, 2.0)) * smoothstep(0.60, 0.18, length(p.xz));
        let pde_stripes = clamp(0.45 + rd_pde * 1.6 + 0.35 * sin(p.x * 18.0 + p.z * 16.0), 0.0, 1.0);
        col += mix(vec3<f32>(0.22, 0.96, 1.18), vec3<f32>(1.22, 0.76, 0.26), pde_stripes) * plate * select(0.75, 1.25, feat == 2u);
        dens += plate * 0.74;

        let motif_c = p - vec3<f32>(0.0, 0.14, 0.0);
        if (feat == 1u) {
            let crys = exp(-pow(abs(sd_octahedron(motif_c, 0.32)) * 46.0, 2.0));
            col += vec3<f32>(0.52, 1.18, 1.28) * crys;
            dens += crys * 0.82;
        } else if (feat == 3u) {
            let w_shell = exp(-pow(abs(length(motif_c) - 0.28) * 46.0, 2.0)) * (0.5 + 0.5 * sin(length(motif_c) * 28.0 - time * 2.4));
            col += vec3<f32>(0.28, 1.18, 1.26) * w_shell;
            dens += w_shell * 0.80;
        } else if (feat == 4u) {
            let a_bz = atan2(motif_c.z, motif_c.x);
            let bz = pow(max(0.0, cos(a_bz * 2.0 - length(motif_c.xz) * 26.0 + time * 2.2)), 6.0) * exp(-pow(motif_c.y * 28.0, 2.0)) * smoothstep(0.46, 0.08, length(motif_c.xz));
            col += vec3<f32>(1.24, 0.42, 0.84) * bz;
            dens += bz * 0.80;
        } else if (feat == 5u) {
            let d_tr = doc_line_glow(sd_capsule(motif_c, vec3<f32>(0.0, -0.22, 0.0), vec3<f32>(0.0, 0.04, 0.0), 0.015), 76.0)
                     + doc_line_glow(sd_capsule(motif_c, vec3<f32>(0.0, 0.04, 0.0), vec3<f32>(-0.18, 0.26, 0.0), 0.011), 76.0)
                     + doc_line_glow(sd_capsule(motif_c, vec3<f32>(0.0, 0.04, 0.0), vec3<f32>(0.18, 0.26, 0.0), 0.011), 76.0);
            col += vec3<f32>(0.34, 1.22, 0.68) * d_tr;
            dens += d_tr * 0.82;
        } else if (feat == 6u) {
            let osc_r = exp(-pow(abs(length(motif_c.xz) - 0.38) * 40.0, 2.0)) * exp(-pow(motif_c.y * 24.0, 2.0)) * (0.45 + 0.55 * cos(atan2(motif_c.z, motif_c.x) * 12.0));
            col += vec3<f32>(1.26, 1.02, 0.34) * osc_r;
            dens += osc_r * 0.82;
        } else {
            let hex_cell = exp(-pow(abs(sd_hex_prism(doc_rot_x(motif_c, 1.5708), 0.28, 0.14)) * 46.0, 2.0));
            col += vec3<f32>(0.32, 1.22, 0.88) * hex_cell * select(0.85, 1.25, feat == 0u);
            dens += hex_cell * 0.72;
        }
    } else if (zone == 7u) {
        // Z07: Cybernetic Feedback — Dual-Torus Counter-Looping Regulator + Controlled Core
        let expand_p = select(0.42, 0.54, feat == 2u);
        let damp_n = select(0.32, 0.24, feat == 3u || feat == 6u);
        let pos_torus = doc_line_glow(sd_torus(doc_rot_x(p, 0.52), expand_p, 0.014), 72.0);
        let neg_torus = doc_line_glow(sd_torus(doc_rot_z(p, -0.62), damp_n, 0.014), 72.0);
        let core_r = select(0.14, 0.20 + 0.05 * sin(time * 4.0), feat == 5u);
        let core_o = exp(-pow(max(r - core_r, 0.0) * 54.0, 2.0));
        col += vec3<f32>(1.24, 0.76, 0.22) * pos_torus * select(0.75, 1.25, feat == 2u)
             + vec3<f32>(0.24, 1.18, 1.28) * neg_torus * select(0.75, 1.25, feat == 3u || feat == 6u)
             + mix(vec3<f32>(0.32, 1.24, 0.76), vec3<f32>(1.28, 0.36, 0.28), select(0.0, 1.0, feat == 7u)) * core_o;
        dens += pos_torus * 0.74 + neg_torus * 0.74 + core_o * 0.82;
        if (feat == 0u || feat == 1u) {
            let pkt_p = select(vec3<f32>(0.42, 0.0, 0.0), vec3<f32>(0.0, 0.32, 0.0), feat == 1u);
            let pkt = exp(-pow(max(length(p - pkt_p) - 0.06, 0.0) * 60.0, 2.0));
            col += vec3<f32>(1.28, 1.02, 0.34) * pkt;
            dens += pkt * 0.85;
        } else if (feat == 4u) {
            let thr = doc_line_glow(sd_torus(p, 0.26, 0.008), 80.0);
            col += vec3<f32>(1.26, 1.04, 0.36) * thr;
            dens += thr * 0.68;
        }
    } else if (zone == 8u) {
        // Z08: Complex Networks — 3D Scale-Free Modular Graph with High-Degree Hubs & Bridges
        let h1 = vec3<f32>(-0.22, 0.14, 0.04);
        let h2 = vec3<f32>(0.24, -0.10, -0.04);
        let n1 = vec3<f32>(-0.44, -0.14, 0.16);
        let n2 = vec3<f32>(-0.34, 0.36, -0.14);
        let n3 = vec3<f32>(0.02, 0.34, 0.22);
        let n4 = vec3<f32>(0.44, 0.20, 0.12);
        let n5 = vec3<f32>(0.36, -0.36, 0.16);
        let n6 = vec3<f32>(-0.02, -0.34, -0.22);

        let e_hub = doc_line_glow(sd_capsule(p, h1, h2, 0.013), 76.0);
        let edges = doc_line_glow(sd_capsule(p, h1, n1, 0.009), 78.0)
                  + doc_line_glow(sd_capsule(p, h1, n2, 0.009), 78.0)
                  + doc_line_glow(sd_capsule(p, h1, n3, 0.009), 78.0)
                  + doc_line_glow(sd_capsule(p, h2, n4, 0.009), 78.0)
                  + doc_line_glow(sd_capsule(p, h2, n5, 0.009), 78.0)
                  + doc_line_glow(sd_capsule(p, h2, n6, 0.009), 78.0);
        let hub_rad = select(0.068, 0.095, feat == 2u);
        let g_h1 = exp(-pow(max(length(p - h1) - hub_rad, 0.0) * 62.0, 2.0)) * select(1.0, 0.22, feat == 6u);
        let g_h2 = exp(-pow(max(length(p - h2) - hub_rad, 0.0) * 62.0, 2.0));
        let leaf_rad = select(0.044, 0.068, feat == 0u);
        let g_leaves = exp(-pow(max(min(min(length(p - n1), length(p - n2)), length(p - n3)) - leaf_rad, 0.0) * 68.0, 2.0))
                     + exp(-pow(max(min(min(length(p - n4), length(p - n5)), length(p - n6)) - leaf_rad, 0.0) * 68.0, 2.0));
        let link_c = select(vec3<f32>(0.28, 1.08, 1.24), vec3<f32>(1.28, 0.32, 0.24), feat == 4u || feat == 6u);
        col += vec3<f32>(1.26, 0.94, 0.26) * (g_h1 + g_h2 + e_hub * select(1.0, 1.35, feat == 1u))
             + link_c * (edges * select(1.0, 1.35, feat == 1u) + g_leaves);
        dens += (g_h1 + g_h2 + e_hub) * 0.82 + (edges + g_leaves) * 0.68;
        if (feat == 3u) {
            let cage_l = doc_line_glow(sd_torus(p - h1, 0.26, 0.008), 76.0);
            let cage_r = doc_line_glow(sd_torus(p - h2, 0.26, 0.008), 76.0);
            col += vec3<f32>(0.36, 1.22, 0.96) * (cage_l + cage_r);
            dens += (cage_l + cage_r) * 0.66;
        } else if (feat == 5u || feat == 7u) {
            let bypass = doc_line_glow(sd_capsule(p, n3, select(n4, n6, feat == 7u), 0.011), 76.0);
            col += vec3<f32>(0.32, 1.24, 0.78) * bypass;
            dens += bypass * 0.76;
        }
    } else if (zone == 9u) {
        // Z09: Self-Organization — Live Gray-Scott Turing Sphere + Coherent Boid Murmuration Ring
        let field_s = doc_sample_field_grid(fract(p.xy * 0.85 + vec2<f32>(0.5)));
        let rd_val = clamp(0.5 + (field_s.r - field_s.g) * 1.4, 0.0, 1.0);
        let t_freq = select(14.0, 22.0, feat == 3u);
        let t_wave = 0.5 + 0.5 * sin(p.x * t_freq + cos(p.y * t_freq + time * 0.6)) * rd_val;
        let sphere_shell = exp(-pow(abs(r - 0.34) * 52.0, 2.0)) * (0.35 + 0.65 * t_wave) * select(1.0, 0.42, feat == 5u);
        col += mix(vec3<f32>(0.24, 1.08, 0.86), accent_z, 0.45 + 0.35 * sin(time * 0.6 * select(0.0, 1.0, feat == 6u))) * sphere_shell;
        dens += sphere_shell * 0.78;

        for (var b = 0; b < 6; b = b + 1) {
            let ang_b = f32(b) * TAU / 6.0 + time * 0.65;
            let bp = vec3<f32>(cos(ang_b) * 0.48, sin(ang_b * 2.0) * 0.14, sin(ang_b) * 0.48);
            let boid = exp(-pow(max(length(p - bp) - 0.036, 0.0) * 68.0, 2.0));
            let nb = exp(-pow(abs(length(p - bp) - 0.12) * 44.0, 2.0)) * select(0.0, 1.0, feat == 0u || feat == 4u);
            col += vec3<f32>(1.24, 0.92, 0.32) * boid + vec3<f32>(0.34, 1.18, 1.28) * nb;
            dens += boid * 0.78 + nb * 0.44;
        }
        if (feat == 1u || feat == 2u) {
            let ring_f = doc_line_glow(sd_torus(p, select(0.48, 0.24, feat == 2u), 0.012), 74.0);
            col += vec3<f32>(0.32, 1.24, 0.94) * ring_f;
            dens += ring_f * 0.74;
        }
    } else if (zone == 10u) {
        // Z10: Functional Information — 3D Helical Symbol Code Ribbon + Reader Collar Gate
        let ang_h = p.y * 8.4 + time * 0.5;
        let rib_d = length(p - vec3<f32>(cos(ang_h) * 0.20, p.y, sin(ang_h) * 0.20));
        let sym_rib = exp(-pow(max(rib_d - 0.032, 0.0) * 64.0, 2.0)) * smoothstep(0.54, 0.18, abs(p.y));
        let collar = doc_line_glow(sd_torus(p, 0.32, 0.018), 68.0);
        let bad_bit = exp(-pow(length(p - vec3<f32>(0.20, -0.16, 0.0)) * 22.0, 2.0)) * select(0.0, 1.0, feat == 4u || feat == 5u);
        col += vec3<f32>(0.28, 1.12, 1.24) * sym_rib * select(1.0, 1.32, feat == 0u || feat == 1u)
             + vec3<f32>(1.24, 0.94, 0.28) * collar * select(0.75, 1.32, feat == 3u || feat == 6u)
             + select(vec3<f32>(1.28, 0.26, 0.76), vec3<f32>(0.28, 1.26, 0.62), feat == 5u) * bad_bit;
        dens += sym_rib * 0.78 + collar * 0.72 + bad_bit * 0.80;
        if (feat == 1u) {
            let mem_r = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.26, 0.0), 0.22, 0.012), 74.0);
            col += vec3<f32>(1.22, 0.92, 0.32) * mem_r;
            dens += mem_r * 0.74;
        } else if (feat == 2u || feat == 7u) {
            let branch = doc_line_glow(sd_capsule(p, vec3<f32>(0.28, 0.0, 0.0), vec3<f32>(0.48, select(-0.24, 0.28, feat == 7u), 0.0), 0.012), 74.0);
            col += vec3<f32>(0.34, 1.24, 0.86) * branch;
            dens += branch * 0.78;
        }
    } else if (zone == 11u) {
        // Z11: Organized Chemistry — 5-Node Eigen Autocatalytic Hypercycle + Lock-and-Key Substrate Dock
        for (var k = 0; k < 5; k = k + 1) {
            let a = f32(k) * TAU / 5.0 + time * 0.24;
            let na = f32(k + 1) * TAU / 5.0 + time * 0.24;
            let np = vec3<f32>(cos(a) * 0.38, sin(a * 2.0) * 0.07, sin(a) * 0.38);
            let n_next = vec3<f32>(cos(na) * 0.38, sin(na * 2.0) * 0.07, sin(na) * 0.38);
            let ng = exp(-pow(max(length(p - np) - 0.045, 0.0) * 62.0, 2.0));
            let edge = doc_line_glow(sd_capsule(p, np, n_next, 0.009), 76.0);
            let nc = hsl_to_rgb(f32(k) / 5.0, 0.88, 0.54) * 0.92;
            col += nc * (ng + edge * select(0.85, 1.28, feat == 1u));
            dens += ng * 0.74 + edge * 0.62;
        }
        if (feat == 0u || feat == 2u || feat == 3u) {
            let dock_s = select(0.14, 0.22, feat == 2u);
            let dock = doc_line_glow(sd_octahedron(p - vec3<f32>(0.0, select(0.0, 0.14, feat == 3u), 0.0), dock_s), 68.0);
            col += select(vec3<f32>(1.26, 0.96, 0.28), vec3<f32>(0.34, 1.24, 0.92), feat == 2u) * dock;
            dens += dock * 0.78;
        } else if (feat == 4u || feat == 5u) {
            let raf_c = doc_line_glow(sd_torus(doc_rot_x(p, select(0.25, 0.95, feat == 5u)), 0.22, 0.011), 74.0);
            col += vec3<f32>(1.24, 0.82, 0.28) * raf_c;
            dens += raf_c * 0.76;
        } else if (feat == 6u) {
            let comp_s = exp(-pow(abs(r - 0.50) * 44.0, 2.0));
            col += vec3<f32>(0.26, 1.18, 0.86) * comp_s;
            dens += comp_s * 0.64;
        }
    } else if (zone == 12u) {
        // Z12: Molecular Replication & Variation — Semiconservative Template Zipper + Variant Lineage
        let upper = p.y >= 0.04;
        let sep = select(0.18 + max(0.04 - p.y, 0.0) * 0.55, 0.11, upper);
        let th = p.y * 7.2 + time * 0.4;
        let s_l = vec3<f32>(-sep * cos(th * 0.25 + 0.2), p.y, -sep * sin(th * 0.25 + 0.2));
        let s_r = vec3<f32>(sep * cos(th * 0.25 + 0.2), p.y, sep * sin(th * 0.25 + 0.2));
        let mask_y = smoothstep(0.54, 0.18, abs(p.y));
        let strand_g = (exp(-pow(max(length(p - s_l) - 0.022, 0.0) * 68.0, 2.0))
                      + exp(-pow(max(length(p - s_r) - 0.022, 0.0) * 68.0, 2.0))) * mask_y;
        let ridge = exp(-pow(abs(p.y + 0.40) * 40.0, 2.0)) * smoothstep(0.56, 0.16, length(p.xz));
        let drift_dim = select(1.0, 0.48, feat == 7u);
        col += vec3<f32>(0.28, 1.16, 0.92) * strand_g * drift_dim + vec3<f32>(1.22, 0.88, 0.28) * ridge * 0.75;
        dens += strand_g * 0.82 * drift_dim + ridge * 0.58;
        if (feat == 1u) {
            let arc = doc_line_glow(sd_capsule(p, vec3<f32>(-0.22, 0.16, 0.0), vec3<f32>(0.22, -0.16, 0.0), 0.011), 76.0);
            col += vec3<f32>(1.24, 0.92, 0.30) * arc;
            dens += arc * 0.76;
        } else if (feat == 2u || feat == 3u) {
            let mut_n = exp(-pow(max(length(p - vec3<f32>(0.24, -0.18, 0.0)) - select(0.055, 0.08, feat == 3u), 0.0) * 60.0, 2.0));
            col += vec3<f32>(1.26, 0.28, 0.82) * mut_n;
            dens += mut_n * 0.86;
        } else if (feat == 4u) {
            let nut = exp(-pow(max(length(p - vec3<f32>(0.0, -0.32, 0.0)) - 0.07, 0.0) * 58.0, 2.0));
            col += vec3<f32>(0.32, 1.24, 0.62) * nut;
            dens += nut * 0.80;
        } else if (feat == 5u || feat == 6u) {
            let fit_p = doc_line_glow(sd_cone_up(p - vec3<f32>(0.0, -0.38, 0.0), vec2<f32>(select(0.55, 0.72, feat == 6u), 0.83), 0.28), 64.0);
            col += vec3<f32>(1.24, 0.96, 0.30) * fit_p;
            dens += fit_p * 0.72;
        }
    } else if (zone == 13u) {
        // Z13: Protocells — Upgraded Cutaway Lipid Bilayer Budding Vesicle + Ion Channels + Lumen
        let pinch = select(0.0, 0.08, feat == 6u);
        let c1 = length(p - vec3<f32>(-0.20 - pinch, 0.0, 0.0)) - select(0.28, 0.34, feat == 5u);
        let c2 = length(p - vec3<f32>(0.20 + pinch, 0.0, 0.0)) - 0.24;
        let d_ves = min(c1, c2);
        let cutaway = select(1.0, 0.32, (feat == 0u || feat == 1u) && (p.z > 0.05 && p.x > 0.0));
        let mem = exp(-pow(abs(d_ves) * 48.0, 2.0)) * cutaway;
        let inner = exp(-pow(abs(d_ves + 0.045) * 48.0, 2.0)) * cutaway * 0.55;
        let lumen = exp(-pow(length(p - vec3<f32>(-0.20, 0.0, 0.0)) * 14.0, 2.0));
        let f_tor = doc_line_glow(sd_torus(vec3<f32>(p.y, p.x, p.z), 0.18, 0.011), 70.0);
        col += vec3<f32>(0.26, 1.18, 0.76) * (mem + inner)
             + vec3<f32>(1.24, 0.92, 0.28) * (lumen * select(0.45, 1.15, feat == 1u) + f_tor * select(0.55, 1.25, feat == 6u));
        dens += (mem + inner) * 0.76 + lumen * 0.58 + f_tor * 0.68;
        if (feat == 2u) {
            let ext = exp(-pow(abs(r - 0.58) * 38.0, 2.0));
            col += vec3<f32>(0.34, 1.16, 1.28) * ext;
            dens += ext * 0.62;
        } else if (feat == 3u || feat == 4u) {
            let pore = doc_line_glow(sd_capsule(p, vec3<f32>(-0.20, 0.20, 0.0), vec3<f32>(-0.20, select(0.36, 0.08, feat == 4u), 0.0), 0.016), 70.0);
            col += vec3<f32>(0.36, 1.26, 1.18) * pore;
            dens += pore * 0.80;
        } else if (feat == 7u) {
            let tether = doc_line_glow(sd_capsule(p, vec3<f32>(-0.20, 0.0, 0.0), vec3<f32>(0.20, 0.0, 0.0), 0.012), 76.0);
            col += vec3<f32>(1.26, 0.96, 0.30) * tether;
            dens += tether * 0.78;
        }
    } else if (zone == 14u) {
        // Z14: Chemical Evolution — Prebiotic Hydrothermal Vent Chimney + Clay Terraces + RNA Hairpin
        let chim = doc_line_glow(sd_cylinder_y(p - vec3<f32>(-0.22, -0.10, 0.0), 0.075, 0.26), 62.0);
        let plume = pow(max(0.0, snoise3(p * 12.0 - vec3<f32>(0.0, time * 1.8, 0.0))), 4.0) * smoothstep(0.28, 0.04, length(p.xz - vec2<f32>(-0.22, 0.0)));
        let clay = exp(-pow(abs(fract((p.y + 0.12) * 8.0) - 0.5) * 14.0, 2.0)) * exp(-pow(abs(p.x - 0.20) * 14.0, 2.0)) * smoothstep(0.32, 0.06, abs(p.y + 0.16));
        let rna_loop = doc_line_glow(sd_torus(doc_rot_x(p - vec3<f32>(0.0, 0.24, 0.0), 0.78), 0.14, 0.012), 72.0);
        col += vec3<f32>(1.22, 0.56, 0.22) * (chim + plume * 0.75) * select(1.0, 1.32, feat == 3u)
             + vec3<f32>(0.28, 1.14, 0.88) * clay * select(1.0, 1.32, feat == 4u)
             + vec3<f32>(0.96, 0.44, 1.24) * rna_loop * select(1.0, 1.32, feat == 0u);
        dens += (chim + plume * 0.65) * 0.72 + clay * 0.68 + rna_loop * 0.74;
        if (feat == 1u || feat == 2u) {
            let wheel = doc_line_glow(sd_torus(p - vec3<f32>(0.0, -0.04, 0.0), select(0.24, 0.12, feat == 2u), 0.012), 72.0);
            col += vec3<f32>(1.24, 0.92, 0.30) * wheel;
            dens += wheel * 0.76;
        } else if (feat == 5u) {
            let pool = exp(-pow(abs(p.y + 0.26) * 44.0, 2.0)) * smoothstep(0.56, 0.18, length(p.xz));
            col += vec3<f32>(0.26, 1.18, 1.24) * pool;
            dens += pool * 0.72;
        } else if (feat == 6u || feat == 7u) {
            let streak = doc_line_glow(sd_capsule(p, vec3<f32>(-0.42, 0.44, 0.0), vec3<f32>(0.18, select(0.08, -0.20, feat == 7u), 0.0), 0.012), 76.0);
            col += vec3<f32>(1.28, 0.94, 0.34) * streak;
            dens += streak * 0.80;
        }
    } else {
        // Z15: Threshold of Life — Chemical-to-Cellular Transition Bridge & Emerald Life Portal
        let chem_r = doc_line_glow(sd_torus(doc_rot_y(p - vec3<f32>(-0.26, 0.0, 0.0), time * 0.3), 0.18, 0.010), 74.0);
        let bridge = doc_line_glow(sd_capsule(p, vec3<f32>(-0.26, 0.0, 0.0), vec3<f32>(0.18, 0.0, 0.0), 0.011), 76.0);
        let bio_shell = exp(-pow(abs(length(p - vec3<f32>(0.18, 0.0, 0.0)) - 0.24) * 50.0, 2.0));
        let bio_core = exp(-pow(length(p - vec3<f32>(0.18, 0.0, 0.0)) * 16.0, 2.0));
        let portal = doc_line_glow(sd_torus(doc_rot_z(p - vec3<f32>(0.42, 0.0, 0.0), 1.5708), 0.22, 0.012), 74.0);
        col += vec3<f32>(1.22, 0.76, 0.26) * chem_r
             + vec3<f32>(0.32, 1.22, 0.88) * (bridge * select(1.0, 1.35, feat == 0u) + bio_shell + portal * select(0.45, 1.25, feat == 5u || feat == 6u))
             + vec3<f32>(0.36, 1.28, 0.62) * bio_core * select(1.0, 1.35, feat == 1u);
        dens += chem_r * 0.70 + bridge * 0.66 + bio_shell * 0.78 + bio_core * 0.72 + portal * 0.62;
        if (feat == 2u) {
            let gen_r = doc_line_glow(sd_torus(p - vec3<f32>(0.18, 0.0, 0.0), 0.14, 0.011), 76.0);
            col += vec3<f32>(1.26, 0.94, 0.30) * gen_r;
            dens += gen_r * 0.78;
        } else if (feat == 3u) {
            let phylo = doc_line_glow(sd_capsule(p, vec3<f32>(0.18, 0.0, 0.0), vec3<f32>(0.42, 0.32, 0.0), 0.011), 76.0);
            col += vec3<f32>(0.38, 1.26, 0.74) * phylo;
            dens += phylo * 0.78;
        } else if (feat == 4u) {
            let mist = exp(-pow(abs(length(p - vec3<f32>(-0.26, 0.0, 0.0)) - 0.34) * 32.0, 2.0));
            col += vec3<f32>(0.62, 0.86, 1.24) * mist;
            dens += mist * 0.64;
        }
    }

    if (has_f) {
        let fh = mm_feat_halo(p_raw, feat, accent_z, time);
        col += fh.rgb * (0.45 + 0.55 * w);
        dens += fh.a * (0.45 + 0.55 * w);
    }
    let pulse = 1.0 + bass * 0.12 + voice * 0.08 + cue * 0.06;
    return vec4<f32>(clamp(col * pulse, vec3<f32>(0.0), vec3<f32>(1.38)), clamp(dens, 0.0, 2.2));
}

// ============================================================================
// EXP 16 (LIFE) — Canonical `[TIER-A]` 19-Zone Stage (`vid = 2600..=2618`, 153 features incl. Z18 F8)
// ============================================================================
fn doc_life_feat_gain(feat: u32, idx: u32, cue: f32) -> f32 {
    if (feat >= 9u) {
        return 0.42;
    }
    if (feat == idx) {
        return 1.24 + 0.18 * sin(cue * TAU);
    }
    return 0.14;
}

fn mm_life_blob(p: vec3<f32>, c: vec3<f32>, rad: f32, k: f32) -> f32 {
    return exp(-pow(max(length(p - c) - rad, 0.0) * k, 2.0));
}

fn mm_life_seg(p: vec3<f32>, a: vec3<f32>, b: vec3<f32>, rad: f32, k: f32) -> f32 {
    return doc_line_glow(sd_capsule(p, a, b, rad), k);
}

fn mm_life_shell(p: vec3<f32>, c: vec3<f32>, rad: f32, k: f32) -> f32 {
    return exp(-pow(abs(length(p - c) - rad) * k, 2.0));
}

fn mm_life_sparkle(p: vec3<f32>, freq: f32) -> f32 {
    return pow(max(sin(p.x * freq) * sin(p.y * freq * 0.93) * sin(p.z * freq * 1.07), 0.0), 8.0);
}

fn shade_doc_life_stage(
    zone: u32,
    feat: u32,
    p: vec3<f32>,
    r: f32,
    cue: f32,
    w: f32,
    time: f32,
    voice: f32,
    bass: f32,
    col_a: vec3<f32>,
    col_b: vec3<f32>,
) -> vec4<f32> {
    var col = vec3<f32>(0.0);
    var dens: f32 = 0.0;
    let accent_p = clamp(col_a, vec3<f32>(0.22), vec3<f32>(1.24));
    let c_life = vec3<f32>(0.32, 1.20, 0.70);
    let c_cyan = vec3<f32>(0.28, 0.96, 1.24);
    let c_gold = vec3<f32>(1.24, 0.92, 0.30);
    let c_mag = vec3<f32>(1.10, 0.42, 1.06);
    let c_amber = vec3<f32>(1.24, 0.62, 0.24);
    let c_blue = vec3<f32>(0.40, 0.66, 1.26);
    let c_red = vec3<f32>(1.22, 0.30, 0.34);
    let c_violet = vec3<f32>(0.72, 0.48, 1.24);

    if (zone == 0u) {
        // Z00: Threshold of Life — pre-LUCA protocell triad (vesicle, autocatalytic loop, gradient halo)
        let g0 = doc_life_feat_gain(feat, 0u, cue);
        let ves = mm_life_shell(p, vec3<f32>(0.0), 0.28, 46.0);
        col += c_life * ves * (0.55 + 0.55 * g0);
        dens += ves * (0.55 + 0.35 * g0);
        for (var k = 0; k < 4; k = k + 1) {
            let a = f32(k) * 1.5707963 + time * 0.35;
            let np = vec3<f32>(cos(a) * 0.44, 0.0, sin(a) * 0.44);
            let nn = vec3<f32>(cos(a + 1.5707963) * 0.44, 0.0, sin(a + 1.5707963) * 0.44);
            let node = mm_life_blob(p, np, 0.030, 60.0);
            let edge = mm_life_seg(p, np, nn, 0.006, 86.0);
            col += c_gold * (node * 0.9 + edge * 0.45);
            dens += node * 0.7 + edge * 0.35;
        }
        if (feat == 0u) {
            // compartimento: second bilayer leaflet seals the interior
            let inner = mm_life_shell(p, vec3<f32>(0.0), 0.245, 52.0);
            col += c_cyan * inner * 0.9;
            dens += inner * 0.7;
        } else if (feat == 1u) {
            // red de reacciones: pentagram reaction network inside the vesicle
            for (var k = 0; k < 5; k = k + 1) {
                let a = f32(k) * 1.2566371;
                let b = a + 2.5132741;
                let pa = vec3<f32>(cos(a) * 0.17, sin(a * 2.0) * 0.05, sin(a) * 0.17);
                let pb = vec3<f32>(cos(b) * 0.17, sin(b * 2.0) * 0.05, sin(b) * 0.17);
                let e = mm_life_seg(p, pa, pb, 0.006, 84.0);
                col += c_amber * e;
                dens += e * 0.6;
            }
        } else if (feat == 2u) {
            // gradiente: hot-to-cold chemical gradient column feeding the vesicle
            let colg = mm_life_seg(p, vec3<f32>(-0.50, -0.38, 0.0), vec3<f32>(-0.50, 0.38, 0.0), 0.026, 52.0);
            let tgr = smoothstep(-0.38, 0.38, p.y);
            col += mix(c_amber, c_cyan, tgr) * colg * (0.55 + 0.6 * (1.0 - tgr));
            dens += colg * 0.72;
        } else if (feat == 3u) {
            // replicación: template strand + nascent copy strand
            let s1 = mm_life_seg(p, vec3<f32>(-0.15, 0.05, 0.0), vec3<f32>(0.15, 0.05, 0.0), 0.010, 80.0);
            let s2 = mm_life_seg(p, vec3<f32>(-0.15, -0.05, 0.0), vec3<f32>(0.06, -0.05, 0.0), 0.010, 80.0);
            col += c_mag * s1 + c_cyan * s2;
            dens += (s1 + s2) * 0.8;
        } else if (feat == 4u) {
            // variación: three daughter vesicles of different size
            for (var k = 0; k < 3; k = k + 1) {
                let dv = mm_life_shell(p, vec3<f32>(0.52, -0.22 + f32(k) * 0.22, 0.0), 0.045 + f32(k) * 0.022, 58.0);
                col += mix(c_life, c_gold, f32(k) * 0.5) * dv;
                dens += dv * 0.7;
            }
        } else if (feat == 5u) {
            // cooperación molecular: two molecules locked into a mutual catalytic ring
            let ring_c = doc_line_glow(sd_torus(doc_rot_x(p - vec3<f32>(0.0, 0.44, 0.0), 1.5708), 0.10, 0.008), 80.0);
            let m1 = mm_life_blob(p, vec3<f32>(-0.10, 0.44, 0.0), 0.028, 60.0);
            let m2 = mm_life_blob(p, vec3<f32>(0.10, 0.44, 0.0), 0.028, 60.0);
            col += c_gold * ring_c + c_cyan * m1 + c_mag * m2;
            dens += ring_c * 0.6 + (m1 + m2) * 0.8;
        } else if (feat == 6u) {
            // herencia imperfecta: copied strand with a red copying error
            let st = mm_life_seg(p, vec3<f32>(-0.30, -0.44, 0.0), vec3<f32>(0.30, -0.44, 0.0), 0.009, 80.0);
            let err = mm_life_blob(p, vec3<f32>(0.08, -0.44, 0.0), 0.034, 58.0);
            col += c_cyan * st * 0.8 + c_red * err * 1.1;
            dens += st * 0.6 + err * 0.9;
        } else if (feat == 7u) {
            // pregunta abierta: open mist halo + unresolved spark above the threshold
            let mist = mm_life_shell(p, vec3<f32>(0.0), 0.56, 30.0) * (0.45 + 0.55 * abs(sin(p.y * 14.0)));
            let spark = mm_life_blob(p, vec3<f32>(0.0, 0.56, 0.0), 0.030, 56.0);
            col += c_violet * mist * 0.7 + c_gold * spark;
            dens += mist * 0.45 + spark * 0.8;
        }
    } else if (zone == 1u) {
        // Z01: First Living Systems — prokaryotic rod cell (wall, nucleoid loop, rotary flagellum)
        let g0 = doc_life_feat_gain(feat, 0u, cue);
        let rod_d = sd_capsule(p, vec3<f32>(-0.24, 0.0, 0.0), vec3<f32>(0.24, 0.0, 0.0), 0.15);
        let rod_g = exp(-pow(abs(rod_d) * 46.0, 2.0));
        let nucleoid = doc_line_glow(sd_torus(doc_rot_x(p, 1.2), 0.08, 0.008), 80.0);
        let tail_x = clamp((p.x - 0.39) / 0.24, 0.0, 1.0);
        let fl_y = 0.045 * sin(p.x * 40.0 - time * 6.0);
        let fl_z = 0.045 * cos(p.x * 40.0 - time * 6.0);
        let flag = exp(-pow(length(vec2<f32>(p.y - fl_y, p.z - fl_z)) * 60.0, 2.0)) * smoothstep(0.37, 0.41, p.x) * smoothstep(0.66, 0.60, p.x);
        col += c_life * rod_g * (0.55 + 0.5 * g0) + c_gold * nucleoid * 0.8 + c_cyan * flag * (0.8 + 0.2 * tail_x);
        dens += rod_g * 0.6 + nucleoid * 0.6 + flag * 0.7;
        if (feat == 0u) {
            // unidad: protective outer cell wall
            let wall = exp(-pow(abs(sd_capsule(p, vec3<f32>(-0.24, 0.0, 0.0), vec3<f32>(0.24, 0.0, 0.0), 0.19)) * 50.0, 2.0));
            col += c_cyan * wall * 0.9;
            dens += wall * 0.7;
        } else if (feat == 1u) {
            // mantenimiento: ribosome chain + membrane pumps exchanging matter
            for (var k = 0; k < 6; k = k + 1) {
                let rb = mm_life_blob(p, vec3<f32>(-0.20 + f32(k) * 0.08, -0.06, 0.04), 0.014, 66.0);
                col += c_amber * rb;
                dens += rb * 0.7;
            }
            let pump_a = mm_life_seg(p, vec3<f32>(-0.10, 0.08, 0.0), vec3<f32>(-0.10, 0.24, 0.0), 0.012, 74.0);
            let pump_b = mm_life_seg(p, vec3<f32>(0.10, 0.08, 0.0), vec3<f32>(0.10, 0.24, 0.0), 0.012, 74.0);
            col += c_gold * (pump_a + pump_b);
            dens += (pump_a + pump_b) * 0.7;
        } else if (feat == 2u) {
            // descendencia: binary fission septum + two daughter rods
            let sept = doc_line_glow(sd_torus(doc_rot_z(p, 1.5708), 0.15, 0.010), 78.0);
            let d1 = exp(-pow(abs(sd_capsule(p, vec3<f32>(-0.24, 0.40, 0.0), vec3<f32>(-0.06, 0.40, 0.0), 0.06)) * 54.0, 2.0));
            let d2 = exp(-pow(abs(sd_capsule(p, vec3<f32>(0.06, 0.40, 0.0), vec3<f32>(0.24, 0.40, 0.0), 0.06)) * 54.0, 2.0));
            col += c_gold * sept + c_life * (d1 + d2);
            dens += sept * 0.7 + (d1 + d2) * 0.7;
        } else if (feat == 3u) {
            // población: colony of six sibling rods around the founder
            for (var k = 0; k < 6; k = k + 1) {
                let a = f32(k) * 1.0471976;
                let cpos = vec3<f32>(cos(a) * 0.50, -0.12, sin(a) * 0.50);
                let tang = vec3<f32>(-sin(a), 0.0, cos(a)) * 0.06;
                let sib = exp(-pow(abs(sd_capsule(p, cpos - tang, cpos + tang, 0.035)) * 58.0, 2.0));
                col += c_life * sib;
                dens += sib * 0.65;
            }
        } else if (feat == 4u) {
            // entorno: mineral substrate plate with dissolved nutrient sparks
            let plate = exp(-pow(abs(p.y + 0.40) * 52.0, 2.0)) * smoothstep(0.55, 0.30, length(p.xz));
            let sp = mm_life_sparkle(p, 24.0) * smoothstep(-0.20, -0.36, p.y) * smoothstep(0.55, 0.3, length(p.xz));
            col += c_amber * plate * 0.75 + c_gold * sp;
            dens += plate * 0.6 + sp * 0.6;
        } else if (feat == 5u) {
            // continuidad: unbroken chain of generations
            for (var k = 0; k < 3; k = k + 1) {
                let gp = vec3<f32>(0.30 + f32(k) * 0.12, 0.28 + f32(k) * 0.12, 0.0);
                let gb = mm_life_shell(p, gp, 0.055 - f32(k) * 0.012, 60.0);
                col += c_cyan * gb;
                dens += gb * 0.7;
            }
            let link = mm_life_seg(p, vec3<f32>(0.22, 0.18, 0.0), vec3<f32>(0.54, 0.52, 0.0), 0.006, 84.0);
            col += c_gold * link * 0.8;
            dens += link * 0.5;
        } else if (feat == 6u) {
            // ancestro común: LUCA root node with converging lineages
            let luca = mm_life_blob(p, vec3<f32>(0.0, -0.50, 0.0), 0.040, 54.0);
            col += c_gold * luca * 1.1;
            dens += luca * 0.9;
            for (var k = 0; k < 3; k = k + 1) {
                let lx = -0.20 + f32(k) * 0.20;
                let lin = mm_life_seg(p, vec3<f32>(0.0, -0.50, 0.0), vec3<f32>(lx, -0.17, 0.0), 0.007, 82.0);
                col += c_life * lin;
                dens += lin * 0.6;
            }
        } else if (feat == 7u) {
            // extinción temprana: dim truncated lineages ending in red crosses
            for (var k = 0; k < 2; k = k + 1) {
                let ey = -0.16 + f32(k) * 0.32;
                let br = mm_life_seg(p, vec3<f32>(-0.30, ey * 0.5, 0.0), vec3<f32>(-0.50, ey, 0.0), 0.007, 82.0);
                let x1 = mm_life_seg(p, vec3<f32>(-0.55, ey - 0.05, 0.0), vec3<f32>(-0.45, ey + 0.05, 0.0), 0.007, 82.0);
                let x2 = mm_life_seg(p, vec3<f32>(-0.55, ey + 0.05, 0.0), vec3<f32>(-0.45, ey - 0.05, 0.0), 0.007, 82.0);
                col += c_life * br * 0.4 + c_red * (x1 + x2);
                dens += br * 0.35 + (x1 + x2) * 0.7;
            }
        }
    } else if (zone == 2u) {
        // Z02: Primitive Cells — complete cutaway cell (bilayer, cytosol flow, enzymatic hub)
        let g0 = doc_life_feat_gain(feat, 0u, cue);
        let outer_m = mm_life_shell(p, vec3<f32>(0.0), 0.40, 48.0) * smoothstep(-0.20, 0.10, p.z + 0.25);
        let inner_m = mm_life_shell(p, vec3<f32>(0.0), 0.37, 52.0) * smoothstep(-0.20, 0.10, p.z + 0.25);
        let cyto = pow(0.5 + 0.5 * sin(atan2(p.z, p.x) * 3.0 + length(p.xz) * 14.0 - time), 6.0) * smoothstep(0.34, 0.10, r) * 0.30;
        let hub = mm_life_blob(p, vec3<f32>(0.0), 0.05, 50.0);
        col += c_life * outer_m * (0.5 + 0.5 * g0) + c_cyan * inner_m * 0.45 + c_blue * cyto + accent_p * hub * (0.5 + 0.7 * g0);
        dens += outer_m * 0.55 + inner_m * 0.35 + cyto * 0.5 + hub * 0.7;
        if (feat == 1u) {
            // membrana: gated transmembrane channels around the equator
            for (var k = 0; k < 6; k = k + 1) {
                let a = f32(k) * 1.0471976;
                let dir = vec3<f32>(cos(a), 0.0, sin(a));
                let ch = mm_life_seg(p, dir * 0.33, dir * 0.45, 0.016, 70.0);
                col += c_gold * ch;
                dens += ch * 0.75;
            }
        } else if (feat == 2u) {
            // interior: three looping cytosolic streams
            for (var k = 0; k < 3; k = k + 1) {
                let st = doc_line_glow(sd_torus(doc_rot_x(doc_rot_y(p, f32(k) * 2.094), 0.6 + f32(k) * 0.4), 0.20, 0.008), 80.0);
                col += c_cyan * st;
                dens += st * 0.6;
            }
        } else if (feat == 3u) {
            // energía: ATP sparks orbiting the hub
            for (var k = 0; k < 5; k = k + 1) {
                let a = f32(k) * 1.2566371 + time * 0.8;
                let sp = mm_life_blob(p, vec3<f32>(cos(a) * 0.22, sin(a * 2.0) * 0.06, sin(a) * 0.22), 0.020, 64.0);
                col += c_gold * sp * 1.1;
                dens += sp * 0.8;
            }
        } else if (feat == 4u) {
            // alimento: food beads crossing a gate into the cell
            for (var k = 0; k < 4; k = k + 1) {
                let fx = 0.62 - f32(k) * 0.10 - fract(time * 0.2) * 0.10;
                let fb = mm_life_blob(p, vec3<f32>(fx, 0.06, 0.0), 0.024, 60.0);
                col += c_amber * fb;
                dens += fb * 0.75;
            }
        } else if (feat == 5u) {
            // reparación: membrane patch being resealed at the top
            let patch_r = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.39, 0.0), 0.09, 0.012), 74.0);
            let patch_c = mm_life_blob(p, vec3<f32>(0.0, 0.40, 0.0), 0.04, 54.0);
            col += c_gold * patch_r + c_life * patch_c;
            dens += patch_r * 0.75 + patch_c * 0.6;
        } else if (feat == 6u) {
            // división: equatorial constriction ring pinching the cell
            let pinch = doc_line_glow(sd_torus(doc_rot_z(p, 1.5708), 0.34, 0.014), 70.0);
            col += c_mag * pinch;
            dens += pinch * 0.8;
        } else if (feat == 7u) {
            // entorno: surrounding medium ring with drifting solutes
            let env_r = doc_line_glow(sd_torus(p, 0.58, 0.008), 80.0);
            let sol = mm_life_sparkle(p, 20.0) * smoothstep(0.46, 0.56, r) * smoothstep(0.70, 0.60, r);
            col += c_blue * env_r + c_cyan * sol;
            dens += env_r * 0.6 + sol * 0.6;
        }
    } else if (zone == 3u) {
        // Z03: Metabolism — redox membrane slab + rotary ATP synthase turbine
        let g4 = doc_life_feat_gain(feat, 4u, cue);
        let slab = exp(-pow(abs(p.y) * 56.0, 2.0)) * smoothstep(0.56, 0.36, length(p.xz));
        let d_rotor = sd_cylinder_y(p, 0.09, 0.22);
        let blade = exp(-pow(abs(sin(atan2(p.z, p.x) * 6.0 - time * 4.5)) * 12.0, 2.0)) * exp(-pow(abs(d_rotor) * 36.0, 2.0));
        col += c_life * slab * 0.65 + c_gold * blade * (0.6 + 0.4 * g4);
        dens += slab * 0.55 + blade * 0.8;
        if (feat == 0u) {
            // nutriente: hexagonal sugar ring descending onto the membrane
            for (var k = 0; k < 6; k = k + 1) {
                let a = f32(k) * 1.0471976;
                let hb = mm_life_blob(p, vec3<f32>(-0.36 + cos(a) * 0.08, 0.30, sin(a) * 0.08), 0.016, 66.0);
                col += c_amber * hb;
                dens += hb * 0.75;
            }
        } else if (feat == 1u) {
            // reacción: two reactants fusing into a product
            let ra = mm_life_blob(p, vec3<f32>(0.24, 0.34, 0.0), 0.034, 58.0);
            let rb = mm_life_blob(p, vec3<f32>(0.44, 0.34, 0.0), 0.034, 58.0);
            let br = mm_life_seg(p, vec3<f32>(0.24, 0.34, 0.0), vec3<f32>(0.44, 0.34, 0.0), 0.010, 76.0);
            col += c_cyan * ra + c_mag * rb + c_gold * br;
            dens += (ra + rb) * 0.8 + br * 0.6;
        } else if (feat == 2u) {
            // enzima: open C-shaped enzyme clamp gripping its substrate
            let q = p - vec3<f32>(-0.36, -0.24, 0.0);
            let clamp_g = doc_line_glow(sd_torus(doc_rot_x(q, 1.5708), 0.10, 0.018), 70.0) * smoothstep(0.06, -0.02, q.x);
            let subs = mm_life_blob(q, vec3<f32>(0.0), 0.034, 58.0);
            col += c_violet * clamp_g + c_gold * subs;
            dens += clamp_g * 0.8 + subs * 0.8;
        } else if (feat == 3u) {
            // ruta metabólica: cyclic pathway ring with five intermediates
            let ring_p = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.32, 0.0), 0.22, 0.007), 82.0);
            col += c_cyan * ring_p;
            dens += ring_p * 0.6;
            for (var k = 0; k < 5; k = k + 1) {
                let a = f32(k) * 1.2566371 + time * 0.4;
                let nb = mm_life_blob(p, vec3<f32>(cos(a) * 0.22, 0.32, sin(a) * 0.22), 0.022, 62.0);
                col += c_gold * nb;
                dens += nb * 0.75;
            }
        } else if (feat == 4u) {
            // portador de energía: ATP with three glowing phosphate beads
            let ad = mm_life_blob(p, vec3<f32>(0.18, -0.24, 0.0), 0.034, 58.0);
            col += c_blue * ad;
            dens += ad * 0.75;
            for (var k = 0; k < 3; k = k + 1) {
                let ph = mm_life_blob(p, vec3<f32>(0.27 + f32(k) * 0.07, -0.24, 0.0), 0.020, 64.0);
                col += c_gold * ph * 1.1;
                dens += ph * 0.8;
            }
        } else if (feat == 5u) {
            // gradiente: dense proton field above, sparse below, flowing through the turbine
            let hplus = mm_life_sparkle(p, 26.0) * smoothstep(0.04, 0.12, p.y) * smoothstep(0.44, 0.30, p.y) * smoothstep(0.52, 0.30, length(p.xz));
            let flow = mm_life_seg(p, vec3<f32>(0.0, 0.40, 0.0), vec3<f32>(0.0, -0.34, 0.0), 0.012, 70.0);
            col += c_cyan * hplus * 1.2 + c_blue * flow;
            dens += hplus * 0.8 + flow * 0.6;
        } else if (feat == 6u) {
            // desecho: waste molecules exported down and away
            for (var k = 0; k < 3; k = k + 1) {
                let wb = mm_life_blob(p, vec3<f32>(0.28 + f32(k) * 0.10, -0.28 - f32(k) * 0.10, 0.0), 0.024, 60.0);
                col += c_violet * wb * 0.9;
                dens += wb * 0.7;
            }
        } else if (feat == 7u) {
            // homeostasis: closed feedback loop balancing the whole assembly
            let loop_g = doc_line_glow(sd_torus(doc_rot_x(p, 1.5708), 0.52, 0.010), 76.0);
            let bal = mm_life_blob(p, vec3<f32>(0.0, 0.52, 0.0), 0.030, 58.0);
            col += c_life * loop_g + c_gold * bal;
            dens += loop_g * 0.65 + bal * 0.8;
        }
    } else if (zone == 4u) {
        // Z04: Heritable Information — B-DNA double helix with base-pair rungs
        let g1 = doc_life_feat_gain(feat, 1u, cue);
        if (abs(p.y) < 0.52 && length(p.xz) < 0.30) {
            let theta = p.y * 8.2 + time * 0.6;
            let r_h = 0.18;
            let s1 = vec3<f32>(cos(theta) * r_h, p.y, sin(theta) * r_h);
            let s2 = vec3<f32>(-cos(theta) * r_h, p.y, -sin(theta) * r_h);
            let h1 = exp(-pow(length(p - s1) * 48.0, 2.0));
            let h2 = exp(-pow(length(p - s2) * 48.0, 2.0));
            let rung_phase = fract(p.y * 5.2 + 0.5) - 0.5;
            let d_rung = length(vec2<f32>(rung_phase / 5.2, dot(p.xz, normalize(vec2<f32>(-sin(theta), cos(theta))))));
            let in_cyl = smoothstep(r_h + 0.01, r_h - 0.02, length(p.xz));
            let rung = exp(-pow(d_rung * 58.0, 2.0)) * in_cyl;
            let rung_col = select(c_amber, c_life, sin(theta * 2.0) > 0.0);
            let hs = 0.55 + 0.6 * g1;
            col += (c_cyan * h1 + c_mag * h2 + rung_col * rung) * hs;
            dens += (h1 + h2 + rung) * 0.85;
        }
        if (feat == 0u) {
            // ARN: single-stranded RNA with a folded hairpin loop
            let rx = 0.42 + 0.035 * sin(p.y * 18.0 + time);
            let rna = exp(-pow(length(vec2<f32>(p.x - rx, p.z)) * 56.0, 2.0)) * smoothstep(0.40, 0.30, abs(p.y + 0.04));
            let hp = doc_line_glow(sd_torus(doc_rot_x(p - vec3<f32>(0.42, 0.38, 0.0), 1.5708), 0.06, 0.008), 80.0);
            col += c_mag * (rna + hp);
            dens += (rna + hp) * 0.75;
        } else if (feat == 2u) {
            // gen: bracketed gene segment along the helix
            let ba = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.04, 0.0), 0.25, 0.010), 76.0);
            let bb = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.26, 0.0), 0.25, 0.010), 76.0);
            let band = exp(-pow(abs(length(p.xz) - 0.25) * 40.0, 2.0)) * step(0.04, p.y) * step(p.y, 0.26) * 0.4;
            col += c_gold * (ba + bb + band);
            dens += (ba + bb) * 0.7 + band * 0.4;
        } else if (feat == 3u) {
            // código genético: three codon triplets read off beside the helix
            for (var k = 0; k < 9; k = k + 1) {
                let grp = k / 3;
                let cb = mm_life_blob(p, vec3<f32>(-0.42, -0.26 + f32(k) * 0.065 + f32(grp) * 0.03, 0.0), 0.018, 64.0);
                let ccol = select(select(c_cyan, c_gold, grp == 1), c_mag, grp == 2);
                col += ccol * cb;
                dens += cb * 0.75;
            }
        } else if (feat == 4u) {
            // proteína: folded polypeptide bead chain
            for (var k = 0; k < 8; k = k + 1) {
                let t = f32(k) * 0.78;
                let pp = vec3<f32>(0.40 + 0.09 * sin(t * 2.0), -0.32 + 0.09 * cos(t * 3.0), 0.09 * sin(t * 1.5));
                let pb = mm_life_blob(p, pp, 0.018, 64.0);
                col += c_amber * pb;
                dens += pb * 0.75;
            }
        } else if (feat == 5u) {
            // mutación: one base pair flashes into a mismatch
            let mut_b = mm_life_blob(p, vec3<f32>(0.0, -0.08, 0.0), 0.040, 54.0);
            let st1 = mm_life_seg(p, vec3<f32>(-0.10, -0.08, 0.0), vec3<f32>(0.10, -0.08, 0.0), 0.006, 84.0);
            col += c_red * mut_b * 1.1 + c_gold * st1;
            dens += mut_b * 0.9 + st1 * 0.6;
        } else if (feat == 6u) {
            // copia: replication fork splitting the helix into two copies
            let fa = mm_life_seg(p, vec3<f32>(0.0, 0.42, 0.0), vec3<f32>(-0.24, 0.62, 0.0), 0.012, 74.0);
            let fb = mm_life_seg(p, vec3<f32>(0.0, 0.42, 0.0), vec3<f32>(0.24, 0.62, 0.0), 0.012, 74.0);
            col += c_cyan * fa + c_mag * fb;
            dens += (fa + fb) * 0.8;
        } else if (feat == 7u) {
            // reparación: sliding repair clamp scanning the helix
            let yc = -0.40 + 0.80 * fract(time * 0.15);
            let clamp_r = doc_line_glow(sd_torus(p - vec3<f32>(0.0, yc, 0.0), 0.24, 0.018), 66.0);
            col += c_life * clamp_r * 1.1;
            dens += clamp_r * 0.85;
        }
    } else if (zone == 5u) {
        // Z05: Darwinian Evolution — 3D phylogenetic tree (LUCA -> Bacteria / Archaea / Eukarya)
        let g7 = doc_life_feat_gain(feat, 7u, cue);
        let root = vec3<f32>(0.0, -0.46, 0.0);
        let fork = vec3<f32>(0.0, -0.18, 0.0);
        let luca = mm_life_blob(p, root, 0.030, 40.0);
        let trunk = mm_life_seg(p, root, fork, 0.012, 90.0);
        col += c_gold * (luca * (0.7 + 0.6 * g7) + trunk * 0.8);
        dens += luca * 0.8 + trunk * 0.8;
        for (var dom = 0; dom < 3; dom = dom + 1) {
            let da = f32(dom) * 2.0943951 + 0.35;
            let mid = vec3<f32>(cos(da) * 0.24, 0.06, sin(da) * 0.24);
            let tip_a = vec3<f32>(cos(da - 0.24) * 0.48, 0.42, sin(da - 0.24) * 0.48);
            let tip_b = vec3<f32>(cos(da + 0.24) * 0.46, 0.38, sin(da + 0.24) * 0.46);
            let b1 = mm_life_seg(p, fork, mid, 0.010, 88.0);
            let b2 = mm_life_seg(p, mid, tip_a, 0.007, 92.0);
            let b3 = mm_life_seg(p, mid, tip_b, 0.007, 92.0);
            let leaf = exp(-pow(min(length(p - tip_a), length(p - tip_b)) * 22.0, 2.0));
            let dom_c = select(select(c_life, c_amber, dom == 1), c_blue, dom == 2);
            col += dom_c * (b1 + b2 + b3 + leaf) * 0.85;
            dens += (b1 + b2 + b3 + leaf) * 0.8;
        }
        if (feat == 0u) {
            // variación: spray of heritable variants at the crown
            for (var k = 0; k < 6; k = k + 1) {
                let a = f32(k) * 1.0471976;
                let vb = mm_life_blob(p, vec3<f32>(cos(a) * 0.09, 0.58, sin(a) * 0.09), 0.018, 64.0);
                col += mix(c_cyan, c_mag, f32(k) / 5.0) * vb;
                dens += vb * 0.75;
            }
        } else if (feat == 1u) {
            // selección: sieve plane filtering lineages
            let plane = exp(-pow(abs(p.y - 0.24) * 70.0, 2.0)) * smoothstep(0.55, 0.40, length(p.xz));
            let holes = max(exp(-pow(abs(fract(p.x * 7.0) - 0.5) * 14.0, 2.0)), exp(-pow(abs(fract(p.z * 7.0) - 0.5) * 14.0, 2.0)));
            col += c_cyan * plane * holes;
            dens += plane * holes * 0.7;
        } else if (feat == 2u) {
            // adaptación: fitness peak with a population climbing it
            let dxz = p.xz - vec2<f32>(0.36, 0.0);
            let hy = -0.52 + 0.24 * exp(-dot(dxz, dxz) * 30.0);
            let hill = exp(-pow(abs(p.y - hy) * 50.0, 2.0)) * smoothstep(0.30, 0.12, length(dxz));
            let climber = mm_life_blob(p, vec3<f32>(0.36, -0.27, 0.0), 0.026, 60.0);
            col += c_amber * hill + c_gold * climber;
            dens += hill * 0.7 + climber * 0.8;
        } else if (feat == 3u) {
            // deriva genética: random-walk drifting lineages
            let dx = p.x + 0.45 - 0.045 * sin(p.y * 23.0 + time);
            let dz = p.z - 0.04 * cos(p.y * 17.0 - time * 0.7);
            let drift = exp(-pow(length(vec2<f32>(dx, dz)) * 50.0, 2.0)) * smoothstep(0.42, 0.30, abs(p.y));
            col += c_violet * drift;
            dens += drift * 0.75;
        } else if (feat == 4u) {
            // competencia: two rivals locked in a red tension line
            let ca = mm_life_blob(p, vec3<f32>(-0.20, 0.58, 0.0), 0.028, 60.0);
            let cb = mm_life_blob(p, vec3<f32>(0.20, 0.58, 0.0), 0.028, 60.0);
            let tens = mm_life_seg(p, vec3<f32>(-0.17, 0.58, 0.0), vec3<f32>(0.17, 0.58, 0.0), 0.007, 82.0);
            col += c_amber * (ca + cb) + c_red * tens;
            dens += (ca + cb) * 0.8 + tens * 0.65;
        } else if (feat == 5u) {
            // cooperación: mutualistic bridge between two distant lineages
            let ta = vec3<f32>(cos(0.11) * 0.48, 0.42, sin(0.11) * 0.48);
            let tb = vec3<f32>(cos(2.6843) * 0.46, 0.38, sin(2.6843) * 0.46);
            let br = mm_life_seg(p, ta, tb, 0.010, 76.0);
            col += c_life * br * 1.1;
            dens += br * 0.8;
        } else if (feat == 6u) {
            // extinción: truncated branch capped in red
            let s0 = vec3<f32>(cos(4.5388) * 0.24, 0.06, sin(4.5388) * 0.24);
            let s1 = vec3<f32>(-0.10, -0.06, -0.50);
            let stub = mm_life_seg(p, s0, s1, 0.008, 84.0);
            let cap = mm_life_blob(p, s1, 0.032, 56.0);
            col += c_blue * stub * 0.45 + c_red * cap * 1.1;
            dens += stub * 0.4 + cap * 0.85;
        } else if (feat == 7u) {
            // ancestro común: concentric rings marking LUCA at the root
            let rr1 = doc_line_glow(sd_torus(p - root, 0.08, 0.007), 80.0);
            let rr2 = doc_line_glow(sd_torus(p - root, 0.14, 0.007), 80.0);
            col += c_gold * (rr1 + rr2);
            dens += (rr1 + rr2) * 0.65;
        }
    } else if (zone == 6u) {
        // Z06: Photosynthesis — chloroplast envelope with thylakoid grana stacks
        let env_d = length(p * vec3<f32>(0.85, 1.6, 1.0)) - 0.42;
        let env_g = exp(-pow(abs(env_d) * 44.0, 2.0));
        col += c_life * env_g * 0.55;
        dens += env_g * 0.45;
        for (var k = 0; k < 3; k = k + 1) {
            let a = f32(k) * 2.0943951;
            let cxz = vec2<f32>(cos(a) * 0.20, sin(a) * 0.20);
            let stack = exp(-pow(abs(fract(p.y * 9.0) - 0.5) * 12.0, 2.0)) * exp(-pow(max(length(p.xz - cxz) - 0.08, 0.0) * 40.0, 2.0)) * smoothstep(0.20, 0.10, abs(p.y));
            col += vec3<f32>(0.20, 1.18, 0.48) * stack;
            dens += stack * 0.75;
        }
        if (feat == 0u) {
            // luz solar: three parallel sun rays striking the stacks
            for (var k = 0; k < 3; k = k + 1) {
                let oz = -0.10 + f32(k) * 0.10;
                let ray = mm_life_seg(p, vec3<f32>(-0.62, 0.56, oz), vec3<f32>(-0.18, 0.12, oz), 0.008, 80.0);
                col += c_gold * ray;
                dens += ray * 0.65;
            }
        } else if (feat == 1u) {
            // pigmento: chlorophyll porphyrin square with central magnesium
            let pc = vec3<f32>(0.46, 0.22, 0.0);
            let mg = mm_life_blob(p, pc, 0.020, 64.0);
            col += c_gold * mg;
            dens += mg * 0.8;
            for (var k = 0; k < 4; k = k + 1) {
                let a = f32(k) * 1.5707963 + 0.785;
                let pn = mm_life_blob(p, pc + vec3<f32>(cos(a) * 0.07, sin(a) * 0.07, 0.0), 0.016, 66.0);
                col += vec3<f32>(0.24, 1.22, 0.42) * pn;
                dens += pn * 0.75;
            }
        } else if (feat == 2u) {
            // fijación de carbono: Calvin ring capturing CO2 beads
            let calvin = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.40, 0.0), 0.16, 0.009), 78.0);
            let co2 = mm_life_blob(p, vec3<f32>(0.26 - fract(time * 0.2) * 0.10, 0.40, 0.0), 0.022, 62.0);
            col += c_cyan * calvin + c_amber * co2;
            dens += calvin * 0.7 + co2 * 0.8;
        } else if (feat == 3u) {
            // tapete microbiano: layered stromatolite microbial mat dome
            for (var k = 0; k < 3; k = k + 1) {
                let lay = mm_life_shell(p, vec3<f32>(0.0, -0.66, 0.0), 0.20 + f32(k) * 0.06, 60.0) * step(-0.52, p.y);
                col += mix(c_amber, c_life, f32(k) * 0.5) * lay;
                dens += lay * 0.65;
            }
        } else if (feat == 4u) {
            // oxígeno: O2 bubble pairs rising out of the leaf
            for (var k = 0; k < 3; k = k + 1) {
                let yy = -0.10 + fract(time * 0.25 + f32(k) * 0.33) * 0.50;
                let o1 = mm_life_blob(p, vec3<f32>(0.30, yy, 0.18 * f32(k - 1)), 0.016, 66.0);
                let o2 = mm_life_blob(p, vec3<f32>(0.35, yy, 0.18 * f32(k - 1)), 0.016, 66.0);
                col += c_cyan * (o1 + o2) * 1.1;
                dens += (o1 + o2) * 0.8;
            }
        } else if (feat == 5u) {
            // atmósfera: oxygenated atmosphere cap above
            let atm = mm_life_shell(p, vec3<f32>(0.0), 0.60, 34.0) * smoothstep(0.08, 0.30, p.y);
            col += c_blue * atm * 0.9;
            dens += atm * 0.6;
        } else if (feat == 6u) {
            // estrés oxidativo: reactive oxygen sparks crackling at the envelope
            let ros = mm_life_sparkle(p, 30.0) * exp(-pow(abs(env_d - 0.05) * 18.0, 2.0));
            col += c_red * ros * 1.3;
            dens += ros * 0.8;
        } else if (feat == 7u) {
            // transformación planetaria: small globe turning from rust to blue
            let q = p - vec3<f32>(-0.40, -0.36, 0.0);
            let globe = mm_life_shell(q, vec3<f32>(0.0), 0.11, 54.0);
            let tint = mix(c_amber, c_blue, smoothstep(-0.10, 0.10, q.y));
            col += tint * globe * 1.1;
            dens += globe * 0.8;
        }
    } else if (zone == 7u) {
        // Z07: Complex Cells — eukaryotic host with nucleus and mitochondrion
        let g0 = doc_life_feat_gain(feat, 0u, cue);
        let g2 = doc_life_feat_gain(feat, 2u, cue);
        let host = mm_life_shell(p, vec3<f32>(0.0), 0.46, 44.0);
        let nuc_c = vec3<f32>(-0.12, 0.04, 0.0);
        let nuc = mm_life_shell(p, nuc_c, 0.15, 56.0);
        let mito_c = vec3<f32>(0.22, -0.12, 0.06);
        let mito_d = length((p - mito_c) * vec3<f32>(0.75, 1.5, 1.3)) - 0.10;
        let mito = exp(-pow(abs(mito_d) * 52.0, 2.0));
        col += c_cyan * host * 0.5 + c_blue * nuc * (0.55 + 0.6 * g0) + c_amber * mito * (0.55 + 0.6 * g2);
        dens += host * 0.45 + nuc * 0.6 + mito * 0.6;
        if (feat == 0u) {
            // núcleo: nuclear envelope with a ring of pores
            for (var k = 0; k < 6; k = k + 1) {
                let a = f32(k) * 1.0471976;
                let pore = mm_life_blob(p, nuc_c + vec3<f32>(cos(a) * 0.15, 0.0, sin(a) * 0.15), 0.014, 66.0);
                col += c_gold * pore;
                dens += pore * 0.75;
            }
        } else if (feat == 1u) {
            // orgánulo: endoplasmic reticulum folds and Golgi stack
            let er1 = doc_line_glow(sd_torus(p - nuc_c, 0.21, 0.007), 80.0) * step(p.x, -0.05);
            let er2 = doc_line_glow(sd_torus(p - nuc_c, 0.26, 0.007), 80.0) * step(p.x, -0.05);
            col += c_violet * (er1 + er2);
            dens += (er1 + er2) * 0.65;
            for (var k = 0; k < 3; k = k + 1) {
                let gs = mm_life_seg(p, vec3<f32>(0.04, 0.22 + f32(k) * 0.04, 0.0), vec3<f32>(0.20, 0.22 + f32(k) * 0.04, 0.0), 0.006, 84.0);
                col += c_gold * gs;
                dens += gs * 0.6;
            }
        } else if (feat == 2u) {
            // mitocondria: folded cristae glowing inside the mitochondrion
            let cris = 0.035 * sin((p.x - mito_c.x) * 60.0);
            let inner = exp(-pow(abs(mito_d + 0.03 + cris) * 46.0, 2.0)) * smoothstep(0.02, -0.02, mito_d);
            col += c_gold * inner * 1.2;
            dens += inner * 0.85;
        } else if (feat == 3u) {
            // endosimbiosis: bacterium being engulfed through a membrane invagination
            let bact = exp(-pow(abs(sd_capsule(p, vec3<f32>(-0.06, 0.44, 0.0), vec3<f32>(0.06, 0.44, 0.0), 0.045)) * 56.0, 2.0));
            let invag = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.42, 0.0), 0.10, 0.008), 78.0);
            col += c_amber * bact + c_cyan * invag;
            dens += bact * 0.75 + invag * 0.65;
        } else if (feat == 4u) {
            // membrana interna: dashed internal membrane system
            let im = mm_life_shell(p, vec3<f32>(0.0), 0.32, 50.0) * step(0.0, sin(atan2(p.z, p.x) * 8.0));
            col += c_life * im * 0.9;
            dens += im * 0.6;
        } else if (feat == 5u) {
            // citoesqueleto: radiating cytoskeletal cables
            for (var k = 0; k < 6; k = k + 1) {
                let a = f32(k) * 1.0471976 + 0.3;
                let el = sin(f32(k) * 1.7) * 0.25;
                let dir = normalize(vec3<f32>(cos(a), el, sin(a)));
                let cab = mm_life_seg(p, dir * 0.08, dir * 0.44, 0.005, 88.0);
                col += c_cyan * cab;
                dens += cab * 0.6;
            }
        } else if (feat == 6u) {
            // genoma mayor: coiled chromatin filling the nucleus
            for (var k = 0; k < 3; k = k + 1) {
                let ch = doc_line_glow(sd_torus(doc_rot_z(doc_rot_x(p - nuc_c, f32(k) * 1.05), f32(k) * 0.7), 0.08, 0.007), 82.0);
                col += c_mag * ch;
                dens += ch * 0.65;
            }
        } else if (feat == 7u) {
            // socio integrado: integrated plastid partner linked to the host
            let pl_c = vec3<f32>(-0.20, -0.26, 0.08);
            let pl = exp(-pow(abs(length((p - pl_c) * vec3<f32>(0.8, 1.5, 1.2)) - 0.08) * 54.0, 2.0));
            let ln = mm_life_seg(p, pl_c, nuc_c, 0.005, 86.0);
            col += vec3<f32>(0.24, 1.22, 0.44) * pl + c_gold * ln;
            dens += pl * 0.75 + ln * 0.5;
        }
    } else if (zone == 8u) {
        // Z08: Sexual Reproduction — homologous chromosome pair (maternal blue / paternal magenta)
        let ca = mm_life_seg(p, vec3<f32>(-0.18, -0.20, 0.0), vec3<f32>(-0.02, 0.20, 0.0), 0.020, 64.0)
               + mm_life_seg(p, vec3<f32>(-0.02, -0.20, 0.0), vec3<f32>(-0.18, 0.20, 0.0), 0.020, 64.0);
        let cb = mm_life_seg(p, vec3<f32>(0.02, -0.20, 0.0), vec3<f32>(0.18, 0.20, 0.0), 0.020, 64.0)
               + mm_life_seg(p, vec3<f32>(0.18, -0.20, 0.0), vec3<f32>(0.02, 0.20, 0.0), 0.020, 64.0);
        col += c_blue * ca * 0.7 + c_mag * cb * 0.7;
        dens += (ca + cb) * 0.6;
        if (feat == 0u) {
            // meiosis: spindle fibres from two poles + four haploid daughters
            for (var k = 0; k < 2; k = k + 1) {
                let sy = select(-0.46, 0.46, k == 1);
                let sa = mm_life_seg(p, vec3<f32>(0.0, sy, 0.0), vec3<f32>(-0.10, 0.0, 0.0), 0.004, 90.0);
                let sb = mm_life_seg(p, vec3<f32>(0.0, sy, 0.0), vec3<f32>(0.10, 0.0, 0.0), 0.004, 90.0);
                col += c_gold * (sa + sb) * 0.8;
                dens += (sa + sb) * 0.5;
            }
            for (var k = 0; k < 4; k = k + 1) {
                let a = f32(k) * 1.5707963 + 0.785;
                let dc = mm_life_shell(p, vec3<f32>(cos(a) * 0.50, 0.0, sin(a) * 0.50), 0.05, 60.0);
                col += c_life * dc;
                dens += dc * 0.65;
            }
        } else if (feat == 1u) {
            // gameto: large egg and motile sperm
            let egg = mm_life_shell(p, vec3<f32>(-0.40, -0.30, 0.0), 0.13, 50.0);
            let head = mm_life_blob(p, vec3<f32>(0.40, -0.30, 0.0), 0.025, 60.0);
            let tail = exp(-pow(length(vec2<f32>(p.y + 0.30 - 0.02 * sin(p.x * 50.0 - time * 8.0), p.z)) * 60.0, 2.0)) * smoothstep(0.42, 0.45, p.x) * smoothstep(0.64, 0.58, p.x);
            col += c_mag * egg + c_blue * (head + tail);
            dens += egg * 0.7 + (head + tail) * 0.75;
        } else if (feat == 2u) {
            // fecundación: fusion flash on the egg surface with ripple
            let egg = mm_life_shell(p, vec3<f32>(0.0, -0.40, 0.0), 0.15, 50.0);
            let fuse = mm_life_blob(p, vec3<f32>(0.0, -0.25, 0.0), 0.030, 56.0);
            let rip = doc_line_glow(sd_torus(p - vec3<f32>(0.0, -0.27, 0.0), 0.08, 0.006), 82.0);
            col += c_mag * egg * 0.8 + c_gold * (fuse * 1.1 + rip);
            dens += egg * 0.6 + fuse * 0.9 + rip * 0.6;
        } else if (feat == 3u) {
            // recombinación: chiasma crossover swapping arm segments
            let chi = mm_life_blob(p, vec3<f32>(-0.10, 0.0, 0.0), 0.030, 58.0) + mm_life_blob(p, vec3<f32>(0.10, 0.0, 0.0), 0.030, 58.0);
            let swap = mm_life_seg(p, vec3<f32>(-0.06, 0.12, 0.0), vec3<f32>(0.06, 0.12, 0.0), 0.016, 66.0);
            col += c_gold * chi + c_life * swap;
            dens += chi * 0.85 + swap * 0.75;
        } else if (feat == 4u) {
            // diversidad genética: four offspring with mixed chromosome segments
            for (var k = 0; k < 4; k = k + 1) {
                let ox = -0.30 + f32(k) * 0.20;
                let mix_t = f32(k) / 3.0;
                let top = mm_life_seg(p, vec3<f32>(ox, 0.40, 0.0), vec3<f32>(ox, 0.48, 0.0), 0.014, 70.0);
                let bot = mm_life_seg(p, vec3<f32>(ox, 0.30, 0.0), vec3<f32>(ox, 0.38, 0.0), 0.014, 70.0);
                col += mix(c_blue, c_mag, mix_t) * top + mix(c_mag, c_blue, mix_t) * bot;
                dens += (top + bot) * 0.75;
            }
        } else if (feat == 5u) {
            // dos progenitores: two parents converging their contributions
            let pa = mm_life_shell(p, vec3<f32>(-0.38, 0.42, 0.0), 0.07, 56.0);
            let pb = mm_life_shell(p, vec3<f32>(0.38, 0.42, 0.0), 0.07, 56.0);
            let la = mm_life_seg(p, vec3<f32>(-0.34, 0.36, 0.0), vec3<f32>(-0.10, 0.20, 0.0), 0.006, 84.0);
            let lb = mm_life_seg(p, vec3<f32>(0.34, 0.36, 0.0), vec3<f32>(0.10, 0.20, 0.0), 0.006, 84.0);
            col += c_blue * (pa + la) + c_mag * (pb + lb);
            dens += (pa + pb) * 0.7 + (la + lb) * 0.55;
        } else if (feat == 6u) {
            // coste: tilted balance beam weighing the cost of sex
            let beam = mm_life_seg(p, vec3<f32>(-0.36, -0.36, 0.0), vec3<f32>(0.36, -0.46, 0.0), 0.010, 76.0);
            let fulc = mm_life_seg(p, vec3<f32>(0.0, -0.41, 0.0), vec3<f32>(0.0, -0.54, 0.0), 0.012, 74.0);
            let wt = mm_life_blob(p, vec3<f32>(0.32, -0.40, 0.0), 0.034, 58.0);
            col += c_gold * (beam + fulc) + c_red * wt;
            dens += (beam + fulc) * 0.7 + wt * 0.8;
        } else if (feat == 7u) {
            // ventaja contextual: shifting environment ring with a protective shield arc
            let env_r = doc_line_glow(sd_torus(p, 0.54, 0.008), 80.0);
            let shift = 0.5 + 0.5 * sin(atan2(p.z, p.x) * 2.0 + time);
            let shield = mm_life_shell(p, vec3<f32>(0.10, 0.0, 0.0), 0.26, 52.0) * step(0.18, p.x);
            col += mix(c_cyan, c_amber, shift) * env_r + c_life * shield;
            dens += env_r * 0.6 + shield * 0.6;
        }
    } else if (zone == 9u) {
        // Z09: Multicellularity — Volvox colony lattice patterned by the live Gray-Scott morphogen field
        let n = p / max(r, 1e-3);
        let q = n * 6.5;
        let cell_d = length(fract(q) - 0.5);
        let shell_m = exp(-pow(abs(r - 0.36) * 30.0, 2.0));
        let fg = doc_sample_field_grid(fract(n.xz * 0.5 + vec2<f32>(0.5)));
        let morph = 0.65 + 0.7 * smoothstep(0.10, 0.35, fg.g);
        let cells = exp(-pow(cell_d * 5.5, 2.0)) * shell_m * morph;
        col += c_life * cells * 0.85;
        dens += cells * 0.7;
        if (feat == 0u) {
            // adhesión: junctional links binding neighbouring cells
            let ex = exp(-pow(min(fract(q.x), 1.0 - fract(q.x)) * 14.0, 2.0));
            let ey = exp(-pow(min(fract(q.y), 1.0 - fract(q.y)) * 14.0, 2.0));
            let ez = exp(-pow(min(fract(q.z), 1.0 - fract(q.z)) * 14.0, 2.0));
            let edges = max(ex * ey, max(ey * ez, ex * ez)) * shell_m;
            col += c_gold * edges;
            dens += edges * 0.7;
        } else if (feat == 1u) {
            // comunicación: signalling wave sweeping across the colony
            let wave = exp(-pow(abs(n.y - sin(time * 1.2) * 0.8) * 12.0, 2.0)) * shell_m;
            col += c_cyan * wave * 1.1;
            dens += wave * 0.8;
        } else if (feat == 2u) {
            // cooperación: shared nutrient channels crossing the interior
            for (var k = 0; k < 3; k = k + 1) {
                let a = f32(k) * 2.0943951;
                let dir = vec3<f32>(cos(a), 0.35 * f32(k - 1), sin(a));
                let tube = mm_life_seg(p, -dir * 0.32, dir * 0.32, 0.010, 76.0);
                col += c_gold * tube;
                dens += tube * 0.7;
            }
        } else if (feat == 3u) {
            // especialización: germ-line cap differentiated from somatic cells
            let cap = exp(-pow(cell_d * 5.5, 2.0)) * shell_m * smoothstep(0.35, 0.60, n.y);
            col += c_gold * cap * 1.2;
            dens += cap * 0.85;
        } else if (feat == 4u) {
            // muerte programada: apoptotic cells fading into red blebs
            let apo = exp(-pow(cell_d * 5.5, 2.0)) * shell_m * smoothstep(-0.45, -0.70, n.x);
            col += c_red * apo * 1.1;
            dens += apo * 0.8;
        } else if (feat == 5u) {
            // desarrollo: inner founder cluster growing outward
            let gr = 0.07 + 0.03 * sin(time * 0.8);
            for (var k = 0; k < 4; k = k + 1) {
                let a = f32(k) * 1.5707963;
                let ib = mm_life_blob(p, vec3<f32>(cos(a) * gr, select(-0.04, 0.04, k % 2 == 0), sin(a) * gr), 0.030, 58.0);
                col += c_cyan * ib;
                dens += ib * 0.75;
            }
        } else if (feat == 6u) {
            // conflicto: selfish cheater patch proliferating on the surface
            let cheat = smoothstep(0.86, 0.96, dot(n, normalize(vec3<f32>(0.7, 0.7, 0.0)))) * shell_m;
            col += c_red * cheat * 1.2;
            dens += cheat * 0.85;
        } else if (feat == 7u) {
            // plan corporal: anterior-posterior axis with segment rings
            let ax = mm_life_seg(p, vec3<f32>(-0.54, 0.0, 0.0), vec3<f32>(0.54, 0.0, 0.0), 0.008, 80.0);
            col += c_gold * ax;
            dens += ax * 0.65;
            for (var k = 0; k < 3; k = k + 1) {
                let sx = -0.20 + f32(k) * 0.20;
                let seg_r = doc_line_glow(sd_torus(doc_rot_z(p - vec3<f32>(sx, 0.0, 0.0), 1.5708), 0.30, 0.007), 80.0);
                col += c_cyan * seg_r;
                dens += seg_r * 0.6;
            }
        }
    } else if (zone == 10u) {
        // Z10: Differentiation & Development — embryo with Hox stripes and live Turing morphogen field
        let emb_d = length(p * vec3<f32>(0.90, 1.15, 1.0)) - 0.34;
        let emb = exp(-pow(abs(emb_d) * 46.0, 2.0));
        let fg = doc_sample_field_grid(fract(p.xy * 0.8 + vec2<f32>(0.5)));
        let turing = smoothstep(0.12, 0.34, fg.g);
        let g1 = doc_life_feat_gain(feat, 1u, cue);
        let stripes = pow(0.5 + 0.5 * sin(p.x * 22.0), 6.0) * emb;
        col += c_life * emb * (0.45 + 0.4 * turing) + c_gold * stripes * (0.35 + 0.6 * g1);
        dens += emb * 0.55 + stripes * 0.45;
        if (feat == 0u) {
            // célula madre: pluripotent core budding daughter cells
            let core = mm_life_blob(p, vec3<f32>(0.0), 0.05, 50.0);
            col += c_gold * core * 1.1;
            dens += core * 0.85;
            for (var k = 0; k < 6; k = k + 1) {
                let a = f32(k) * 1.0471976;
                let db = mm_life_blob(p, vec3<f32>(cos(a) * 0.15, sin(a * 2.0) * 0.04, sin(a) * 0.15), 0.018, 64.0);
                col += c_cyan * db;
                dens += db * 0.7;
            }
        } else if (feat == 1u) {
            // regulación génica: alternating on/off Hox expression domains
            let on_off = select(c_cyan, c_mag, sin(p.x * 11.0) > 0.0);
            col += on_off * stripes * 1.1;
            dens += stripes * 0.5;
        } else if (feat == 2u) {
            // gradiente químico: morphogen gradient pole-to-pole + Turing spots
            let grad_t = smoothstep(0.34, -0.34, p.x);
            col += c_amber * emb * grad_t * 1.1 + c_mag * emb * turing * 0.6;
            dens += emb * (grad_t * 0.6 + turing * 0.4);
        } else if (feat == 3u) {
            // tejido: folding neural groove sheet along the dorsal surface
            for (var k = 0; k < 3; k = k + 1) {
                let oz = -0.05 + f32(k) * 0.05;
                let fold = mm_life_seg(p, vec3<f32>(-0.28, 0.30 - abs(oz) * 0.6, oz), vec3<f32>(0.28, 0.30 - abs(oz) * 0.6, oz), 0.007, 82.0);
                col += c_cyan * fold;
                dens += fold * 0.65;
            }
        } else if (feat == 4u) {
            // órgano: pulsing organ bud forming inside
            let org = mm_life_shell(p, vec3<f32>(-0.12, -0.10, 0.10), 0.07 + 0.01 * sin(time * 3.0), 56.0);
            col += c_red * org * 1.1;
            dens += org * 0.8;
        } else if (feat == 5u) {
            // crecimiento: expanding growth shells
            let gr_r = 0.36 + 0.16 * fract(time * 0.4);
            let grow = mm_life_shell(p, vec3<f32>(0.0), gr_r, 40.0) * (1.0 - fract(time * 0.4));
            col += c_life * grow;
            dens += grow * 0.6;
        } else if (feat == 6u) {
            // forma: paired limb buds patterned with Turing digit stripes
            for (var k = 0; k < 2; k = k + 1) {
                let sz = select(-1.0, 1.0, k == 1);
                let bud = exp(-pow(abs(sd_capsule(p, vec3<f32>(0.18, -0.20, 0.20 * sz), vec3<f32>(0.30, -0.38, 0.36 * sz), 0.05)) * 52.0, 2.0));
                col += mix(c_amber, c_gold, turing) * bud;
                dens += bud * 0.75;
            }
        } else if (feat == 7u) {
            // desarrollo: zygote -> morula -> embryo timeline arc
            let z1 = mm_life_shell(p, vec3<f32>(-0.36, 0.50, 0.0), 0.05, 60.0);
            let z2 = mm_life_blob(p, vec3<f32>(0.0, 0.56, 0.0), 0.05, 46.0) * (0.6 + 0.4 * pow(0.5 + 0.5 * sin(p.x * 80.0) * sin(p.y * 80.0), 2.0));
            let z3 = exp(-pow(abs(length((p - vec3<f32>(0.36, 0.50, 0.0)) * vec3<f32>(0.7, 1.2, 1.0)) - 0.05) * 56.0, 2.0));
            col += c_cyan * z1 + c_gold * z2 + c_life * z3;
            dens += (z1 + z2 + z3) * 0.7;
        }
    } else if (zone == 11u) {
        // Z11: Organisms — integrated metazoan body with circulatory loop and core
        let g0 = doc_life_feat_gain(feat, 0u, cue);
        let body_d = length(p * vec3<f32>(0.62, 1.6, 1.25)) - 0.24;
        let body = exp(-pow(abs(body_d) * 44.0, 2.0));
        let circ = doc_line_glow(sd_torus(doc_rot_x(p, 1.5708), 0.14, 0.007), 80.0);
        let heart = mm_life_blob(p, vec3<f32>(0.08, -0.02, 0.0), 0.028 + 0.006 * sin(time * 4.0), 58.0);
        col += c_cyan * body * (0.5 + 0.55 * g0) + c_red * (circ * 0.8 + heart);
        dens += body * 0.6 + circ * 0.55 + heart * 0.8;
        if (feat == 0u) {
            // cuerpo: tail and dorsal fins completing the body plan
            let tf1 = mm_life_seg(p, vec3<f32>(-0.38, 0.0, 0.0), vec3<f32>(-0.52, 0.14, 0.0), 0.010, 76.0);
            let tf2 = mm_life_seg(p, vec3<f32>(-0.38, 0.0, 0.0), vec3<f32>(-0.52, -0.14, 0.0), 0.010, 76.0);
            let df = mm_life_seg(p, vec3<f32>(-0.06, 0.15, 0.0), vec3<f32>(0.04, 0.26, 0.0), 0.010, 76.0);
            col += c_life * (tf1 + tf2 + df);
            dens += (tf1 + tf2 + df) * 0.7;
        } else if (feat == 1u) {
            // alimentación: mouth ring and through-gut
            let mouth = doc_line_glow(sd_torus(doc_rot_z(p - vec3<f32>(0.39, 0.0, 0.0), 1.5708), 0.05, 0.008), 80.0);
            let gut = mm_life_seg(p, vec3<f32>(0.36, 0.0, 0.0), vec3<f32>(-0.30, -0.03, 0.0), 0.010, 76.0);
            col += c_amber * (mouth + gut);
            dens += (mouth + gut) * 0.7;
        } else if (feat == 2u) {
            // defensa: protective spines along back and belly
            for (var k = 0; k < 6; k = k + 1) {
                let sx = -0.22 + f32(k / 2) * 0.20;
                let sy = select(-0.14, 0.14, k % 2 == 0);
                let sp = mm_life_seg(p, vec3<f32>(sx, sy, 0.0), vec3<f32>(sx + 0.04, sy * 2.0, 0.0), 0.007, 82.0);
                col += c_gold * sp;
                dens += sp * 0.7;
            }
        } else if (feat == 3u) {
            // reproducción: clutch of eggs released behind the body
            for (var k = 0; k < 5; k = k + 1) {
                let a = f32(k) * 1.2566371;
                let eg = mm_life_shell(p, vec3<f32>(-0.52 + cos(a) * 0.06, -0.22 + sin(a) * 0.06, 0.0), 0.022, 70.0);
                col += c_mag * eg;
                dens += eg * 0.7;
            }
        } else if (feat == 4u) {
            // esqueleto: vertebral column of beads
            for (var k = 0; k < 9; k = k + 1) {
                let vb = mm_life_blob(p, vec3<f32>(-0.32 + f32(k) * 0.075, 0.06, 0.0), 0.014, 66.0);
                col += vec3<f32>(1.18, 1.02, 0.66) * vb;
                dens += vb * 0.75;
            }
        } else if (feat == 5u) {
            // músculo: chevron myomere bands along the flank
            let myo = pow(0.5 + 0.5 * sin((p.x + abs(p.y) * 1.4) * 30.0), 8.0) * body;
            col += c_red * myo * 1.2;
            dens += myo * 0.75;
        } else if (feat == 6u) {
            // navegación: planned path arcing ahead of the animal
            for (var k = 0; k < 5; k = k + 1) {
                let t = f32(k) * 0.2;
                let nb = mm_life_blob(p, vec3<f32>(0.44 + t * 0.20, t * t * 0.45, 0.0), 0.014, 66.0);
                col += c_cyan * nb;
                dens += nb * 0.7;
            }
        } else if (feat == 7u) {
            // diversidad: three contrasting body plans orbiting
            let a0 = time * 0.3;
            let o1 = mm_life_shell(p, vec3<f32>(cos(a0) * 0.56, 0.30, sin(a0) * 0.56), 0.05, 60.0);
            let o2 = exp(-pow(abs(sd_octahedron(p - vec3<f32>(cos(a0 + 2.09) * 0.56, 0.30, sin(a0 + 2.09) * 0.56), 0.07)) * 60.0, 2.0));
            let o3c = vec3<f32>(cos(a0 + 4.19) * 0.56, 0.30, sin(a0 + 4.19) * 0.56);
            let o3 = mm_life_seg(p, o3c - vec3<f32>(0.06, 0.0, 0.0), o3c + vec3<f32>(0.06, 0.0, 0.0), 0.02, 66.0);
            col += c_gold * o1 + c_mag * o2 + c_life * o3;
            dens += (o1 + o2 + o3) * 0.7;
        }
    } else if (zone == 12u) {
        // Z12: Ecosystems — miniature biosphere terrarium (biome ground bands + dome)
        let rho = length(p.xz);
        let ang_g = atan2(p.z, p.x);
        let ground = exp(-pow(abs(p.y + 0.30) * 50.0, 2.0)) * smoothstep(0.55, 0.40, rho);
        let biome_t = 0.5 + 0.5 * sin(ang_g * 1.5);
        let biome_c = mix(mix(c_blue, c_life, smoothstep(0.0, 0.5, biome_t)), c_amber, smoothstep(0.6, 1.0, biome_t));
        let dome = mm_life_shell(p, vec3<f32>(0.0, -0.30, 0.0), 0.55, 40.0) * step(-0.30, p.y) * 0.35;
        col += biome_c * ground * 0.85 + c_cyan * dome;
        dens += ground * 0.65 + dome * 0.4;
        if (feat == 0u) {
            // productor: photosynthetic plants rising from the ground
            for (var k = 0; k < 5; k = k + 1) {
                let a = f32(k) * 1.2566371;
                let base = vec3<f32>(cos(a) * 0.28, -0.30, sin(a) * 0.28);
                let stem = mm_life_seg(p, base, base + vec3<f32>(0.0, 0.18, 0.0), 0.006, 84.0);
                let leaf = mm_life_blob(p, base + vec3<f32>(0.0, 0.20, 0.0), 0.024, 60.0);
                col += vec3<f32>(0.24, 1.22, 0.44) * (stem + leaf);
                dens += (stem + leaf) * 0.7;
            }
        } else if (feat == 1u) {
            // consumidor: herbivores feeding on producers
            for (var k = 0; k < 3; k = k + 1) {
                let a = f32(k) * 2.0943951 + 0.4;
                let hp = vec3<f32>(cos(a) * 0.20, 0.06, sin(a) * 0.20);
                let hb = mm_life_blob(p, hp, 0.030, 58.0);
                let ln = mm_life_seg(p, hp, vec3<f32>(cos(a) * 0.28, -0.12, sin(a) * 0.28), 0.005, 86.0);
                col += c_amber * hb + c_gold * ln * 0.7;
                dens += hb * 0.8 + ln * 0.5;
            }
        } else if (feat == 2u) {
            // descomponedor: violet mycelium network beneath the soil
            let myc = exp(-pow(abs(p.y + 0.37) * 60.0, 2.0)) * smoothstep(0.50, 0.30, rho) * pow(0.5 + 0.5 * sin(p.x * 26.0 + sin(p.z * 19.0) * 2.0), 6.0);
            col += c_violet * myc * 1.2;
            dens += myc * 0.8;
        } else if (feat == 3u) {
            // red trófica: interlinked food web of five species
            for (var k = 0; k < 5; k = k + 1) {
                let a = f32(k) * 1.2566371;
                let b = a + 2.5132741;
                let na = vec3<f32>(cos(a) * 0.28, 0.20, sin(a) * 0.28);
                let nb = vec3<f32>(cos(b) * 0.28, 0.20, sin(b) * 0.28);
                let node = mm_life_blob(p, na, 0.022, 62.0);
                let link = mm_life_seg(p, na, nb, 0.005, 86.0);
                col += c_gold * node + c_cyan * link * 0.8;
                dens += node * 0.75 + link * 0.5;
            }
        } else if (feat == 4u) {
            // simbiosis: two partners in mutual orbit
            let sc = vec3<f32>(-0.25, 0.22, 0.0);
            let orb = doc_line_glow(sd_torus(p - sc, 0.09, 0.006), 82.0);
            let sa = mm_life_blob(p, sc + vec3<f32>(cos(time) * 0.09, 0.0, sin(time) * 0.09), 0.022, 62.0);
            let sb = mm_life_blob(p, sc - vec3<f32>(cos(time) * 0.09, 0.0, sin(time) * 0.09), 0.022, 62.0);
            col += c_cyan * orb + c_life * sa + c_mag * sb;
            dens += orb * 0.6 + (sa + sb) * 0.8;
        } else if (feat == 5u) {
            // competencia: two consumers converging on one resource
            let res = mm_life_blob(p, vec3<f32>(0.0, 0.08, 0.0), 0.030, 58.0);
            let la = mm_life_seg(p, vec3<f32>(-0.30, 0.36, 0.0), vec3<f32>(0.0, 0.10, 0.0), 0.007, 82.0);
            let lb = mm_life_seg(p, vec3<f32>(0.30, 0.36, 0.0), vec3<f32>(0.0, 0.10, 0.0), 0.007, 82.0);
            col += c_gold * res + c_red * (la + lb);
            dens += res * 0.8 + (la + lb) * 0.65;
        } else if (feat == 6u) {
            // nicho: one partitioned niche wedge highlighted
            let wedge = step(0.0, ang_g) * step(ang_g, 0.9);
            let niche = ground * wedge;
            let wall = exp(-pow(abs(p.z) * 60.0, 2.0)) * step(0.0, p.x) * smoothstep(0.5, 0.3, rho) * smoothstep(0.1, -0.3, p.y) * step(-0.30, p.y);
            col += c_gold * niche * 1.2 + c_amber * wall * 0.8;
            dens += niche * 0.8 + wall * 0.5;
        } else if (feat == 7u) {
            // ciclo de nutrientes: circulating nutrient loop with flowing beads
            let cyc = doc_line_glow(sd_torus(doc_rot_x(p, 1.5708), 0.42, 0.008), 78.0);
            col += c_life * cyc;
            dens += cyc * 0.6;
            for (var k = 0; k < 4; k = k + 1) {
                let a = f32(k) * 1.5707963 + time * 0.6;
                let bd = mm_life_blob(p, vec3<f32>(cos(a) * 0.42, sin(a) * 0.42, 0.0), 0.020, 62.0);
                col += c_gold * bd;
                dens += bd * 0.75;
            }
        }
    } else if (zone == 13u) {
        // Z13: Movement — articulated limb (bones, joint, contractile muscle belly)
        let shoulder = vec3<f32>(-0.36, 0.20, 0.0);
        let elbow = vec3<f32>(0.0, 0.0, 0.0);
        let wrist = vec3<f32>(0.30, -0.30 + 0.04 * sin(time * 1.4), 0.0);
        let g2 = doc_life_feat_gain(feat, 2u, cue);
        let bone1 = mm_life_seg(p, shoulder, elbow, 0.014, 70.0);
        let bone2 = mm_life_seg(p, elbow, wrist, 0.012, 72.0);
        let joint = mm_life_blob(p, elbow, 0.026, 58.0);
        let mid = (shoulder + elbow) * 0.5 + vec3<f32>(0.0, 0.06, 0.0);
        let belly_d = length((p - mid) * vec3<f32>(0.55, 1.6, 1.4)) - 0.08;
        let belly = exp(-pow(abs(belly_d) * 50.0, 2.0));
        col += vec3<f32>(1.16, 1.00, 0.64) * (bone1 + bone2 + joint) * (0.55 + 0.6 * g2) + c_red * belly * 0.6;
        dens += (bone1 + bone2 + joint) * 0.7 + belly * 0.55;
        if (feat == 0u) {
            // locomoción: swing arc of the stride + beating cilia
            let arc = doc_line_glow(sd_torus(doc_rot_x(p - elbow, 1.5708), 0.42, 0.007), 80.0) * step(p.y, 0.0);
            let cil = pow(0.5 + 0.5 * sin(p.x * 40.0 - time * 6.0), 6.0) * exp(-pow(abs(p.y + 0.52) * 50.0, 2.0)) * smoothstep(0.4, 0.2, abs(p.x));
            col += c_cyan * arc + c_life * cil;
            dens += arc * 0.6 + cil * 0.6;
        } else if (feat == 1u) {
            // músculo: actin/myosin sarcomere striations contracting
            let along = dot(p - shoulder, normalize(elbow - shoulder));
            let stri = pow(0.5 + 0.5 * sin(along * 60.0 + time * 2.0), 4.0) * smoothstep(0.02, -0.02, belly_d);
            col += mix(c_red, c_gold, stri) * stri * 1.2;
            dens += stri * 0.8;
        } else if (feat == 3u) {
            // alimentación: grasping claw closing on food
            let cl1 = mm_life_seg(p, wrist, wrist + vec3<f32>(0.10, 0.06, 0.0), 0.008, 80.0);
            let cl2 = mm_life_seg(p, wrist, wrist + vec3<f32>(0.10, -0.06, 0.0), 0.008, 80.0);
            let food = mm_life_blob(p, wrist + vec3<f32>(0.14, 0.0, 0.0), 0.026, 60.0);
            col += c_gold * (cl1 + cl2) + c_life * food;
            dens += (cl1 + cl2) * 0.7 + food * 0.8;
        } else if (feat == 4u) {
            // huida: escape speed lines away from a predator eye
            for (var k = 0; k < 3; k = k + 1) {
                let ly = 0.05 + f32(k) * 0.10;
                let sl = mm_life_seg(p, vec3<f32>(-0.64, ly, 0.0), vec3<f32>(-0.44, ly, 0.0), 0.005, 86.0);
                col += c_cyan * sl;
                dens += sl * 0.6;
            }
            let eye = mm_life_blob(p, vec3<f32>(0.52, 0.34, 0.0), 0.030, 58.0);
            col += c_red * eye * 1.1;
            dens += eye * 0.8;
        } else if (feat == 5u) {
            // búsqueda: expanding search rings scanning overhead
            for (var k = 0; k < 3; k = k + 1) {
                let sr = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.42, 0.0), 0.07 + f32(k) * 0.07, 0.006), 82.0) * step(0.0, sin(atan2(p.z, p.x) * 6.0 + f32(k)));
                col += c_gold * sr;
                dens += sr * 0.6;
            }
        } else if (feat == 6u) {
            // orientación: compass ring with a heading needle
            let cc = vec3<f32>(-0.38, -0.30, 0.0);
            let ring_c = doc_line_glow(sd_torus(doc_rot_x(p - cc, 1.5708), 0.11, 0.007), 80.0);
            let ndl = mm_life_seg(p, cc - vec3<f32>(0.0, 0.09, 0.0), cc + vec3<f32>(0.0, 0.09, 0.0), 0.008, 80.0);
            col += c_cyan * ring_c + c_red * ndl;
            dens += ring_c * 0.6 + ndl * 0.75;
        } else if (feat == 7u) {
            // conducta: branching behavioural decision arrows
            let stem = mm_life_seg(p, vec3<f32>(0.0, 0.10, 0.0), vec3<f32>(0.0, 0.36, 0.0), 0.008, 80.0);
            let br_a = mm_life_seg(p, vec3<f32>(0.0, 0.36, 0.0), vec3<f32>(-0.20, 0.56, 0.0), 0.008, 80.0);
            let br_b = mm_life_seg(p, vec3<f32>(0.0, 0.36, 0.0), vec3<f32>(0.20, 0.56, 0.0), 0.008, 80.0);
            col += c_gold * stem + c_cyan * br_a + c_mag * br_b;
            dens += (stem + br_a + br_b) * 0.7;
        }
    } else if (zone == 14u) {
        // Z14: Senses — sensory receptor ring feeding a central nerve trunk
        let trunk = mm_life_seg(p, vec3<f32>(0.0, -0.46, 0.0), vec3<f32>(0.0, 0.18, 0.0), 0.012, 72.0);
        let hub = mm_life_blob(p, vec3<f32>(0.0, 0.18, 0.0), 0.036, 56.0);
        col += c_blue * trunk + c_cyan * hub;
        dens += trunk * 0.65 + hub * 0.75;
        for (var k = 0; k < 8; k = k + 1) {
            let a = f32(k) * 0.78539816;
            let rp = vec3<f32>(cos(a) * 0.42, 0.0, sin(a) * 0.42);
            let rn = mm_life_blob(p, rp, 0.016, 64.0);
            let ln = mm_life_seg(p, rp, vec3<f32>(0.0, 0.18, 0.0), 0.003, 92.0);
            col += c_life * rn * 0.8 + c_blue * ln * 0.4;
            dens += rn * 0.6 + ln * 0.3;
        }
        if (feat == 0u) {
            // luz: camera eye (retina cup + lens) receiving light rays
            let ec = vec3<f32>(0.40, 0.26, 0.0);
            let eyeball = mm_life_shell(p, ec, 0.10, 56.0) * step(p.x, ec.x + 0.05);
            let lens = mm_life_blob(p, ec + vec3<f32>(0.09, 0.0, 0.0), 0.022, 62.0);
            let ray = mm_life_seg(p, ec + vec3<f32>(0.12, 0.0, 0.0), ec + vec3<f32>(0.28, 0.08, 0.0), 0.005, 86.0);
            col += c_cyan * eyeball + c_gold * (lens + ray);
            dens += eyeball * 0.7 + lens * 0.8 + ray * 0.5;
        } else if (feat == 1u) {
            // sustancia química: olfactory cilia fanning toward drifting odorants
            let oc = vec3<f32>(-0.40, 0.26, 0.0);
            for (var k = 0; k < 5; k = k + 1) {
                let a = -0.6 + f32(k) * 0.3;
                let cil = mm_life_seg(p, oc, oc + vec3<f32>(-cos(a) * 0.12, sin(a) * 0.12, 0.0), 0.004, 88.0);
                col += c_life * cil;
                dens += cil * 0.6;
            }
            let od = mm_life_sparkle(p - oc, 30.0) * smoothstep(0.18, 0.06, length(p - oc - vec3<f32>(-0.16, 0.0, 0.0)));
            col += c_mag * od * 1.3;
            dens += od * 0.7;
        } else if (feat == 2u) {
            // vibración: cochlear spiral resonating with incoming waves
            let qc = p - vec3<f32>(0.0, 0.46, 0.0);
            let rad = length(qc.xz);
            let ang_s = atan2(qc.z, qc.x);
            let spiral = exp(-pow(abs(fract(rad * 14.0 - ang_s / TAU) - 0.5) * 7.0, 2.0)) * smoothstep(0.16, 0.12, rad) * exp(-pow(qc.y * 40.0, 2.0));
            col += c_gold * spiral * 1.1;
            dens += spiral * 0.8;
        } else if (feat == 3u) {
            // presión: layered Pacinian corpuscle under a pressing plate
            let pc = vec3<f32>(-0.38, -0.26, 0.0);
            for (var k = 0; k < 3; k = k + 1) {
                let lay = exp(-pow(abs(length((p - pc) * vec3<f32>(1.0, 1.4, 1.0)) - (0.04 + f32(k) * 0.035)) * 60.0, 2.0));
                col += mix(c_amber, c_gold, f32(k) * 0.5) * lay;
                dens += lay * 0.6;
            }
            let plate = mm_life_seg(p, pc + vec3<f32>(-0.10, 0.14, 0.0), pc + vec3<f32>(0.10, 0.14, 0.0), 0.010, 76.0);
            col += c_cyan * plate;
            dens += plate * 0.7;
        } else if (feat == 4u) {
            // temperatura: hot-to-cold thermal bar against a thermoreceptor
            let a0 = vec3<f32>(0.24, -0.42, 0.0);
            let b0 = vec3<f32>(0.54, -0.12, 0.0);
            let bar = mm_life_seg(p, a0, b0, 0.020, 64.0);
            let tt = clamp(dot(p - a0, normalize(b0 - a0)) / length(b0 - a0), 0.0, 1.0);
            col += mix(c_red, c_blue, tt) * bar;
            dens += bar * 0.8;
        } else if (feat == 5u) {
            // orientación: three orthogonal semicircular canals
            let vc = p - vec3<f32>(-0.22, 0.46, 0.0);
            let c1 = doc_line_glow(sd_torus(vc, 0.08, 0.006), 82.0);
            let c2 = doc_line_glow(sd_torus(doc_rot_x(vc, 1.5708), 0.08, 0.006), 82.0);
            let c3 = doc_line_glow(sd_torus(doc_rot_z(vc, 1.5708), 0.08, 0.006), 82.0);
            col += c_cyan * c1 + c_gold * c2 + c_mag * c3;
            dens += (c1 + c2 + c3) * 0.65;
        } else if (feat == 6u) {
            // propiocepción: muscle spindle wrapped by a stretch-sensing coil
            let sc = vec3<f32>(0.40, -0.06, 0.24);
            let sp_d = length((p - sc) * vec3<f32>(0.5, 1.6, 1.6)) - 0.05;
            let spindle = exp(-pow(abs(sp_d) * 56.0, 2.0));
            let coil = pow(0.5 + 0.5 * sin((p.x - sc.x) * 70.0), 6.0) * exp(-pow(abs(sp_d - 0.012) * 40.0, 2.0));
            col += c_red * spindle * 0.8 + c_gold * coil;
            dens += spindle * 0.6 + coil * 0.7;
        } else if (feat == 7u) {
            // interocepción: internal heartbeat signalling up the trunk
            let beat = 0.5 + 0.5 * sin(time * 4.0);
            let hrt = mm_life_blob(p, vec3<f32>(0.0, -0.26, 0.0), 0.040 + 0.008 * beat, 54.0);
            let sig = exp(-pow(abs(p.y - (-0.26 + fract(time * 0.5) * 0.44)) * 30.0, 2.0)) * exp(-pow(length(p.xz) * 40.0, 2.0));
            col += c_red * hrt * 1.1 + c_gold * sig * 1.2;
            dens += hrt * 0.85 + sig * 0.8;
        }
    } else if (zone == 15u) {
        // Z15: Nervous Systems — neuron (soma, dendrites, axon, synaptic terminal)
        let soma_c = vec3<f32>(-0.28, 0.05, 0.0);
        let g0 = doc_life_feat_gain(feat, 0u, cue);
        let soma = mm_life_shell(p, soma_c, 0.075, 56.0) + mm_life_blob(p, soma_c, 0.03, 54.0) * 0.6;
        var dend: f32 = 0.0;
        for (var k = 0; k < 5; k = k + 1) {
            let a = 1.8 + f32(k) * 0.65;
            dend += mm_life_seg(p, soma_c, soma_c + vec3<f32>(cos(a) * 0.20, sin(a) * 0.20, 0.06 * f32(k - 2)), 0.005, 86.0);
        }
        let axon = mm_life_seg(p, soma_c + vec3<f32>(0.07, 0.0, 0.0), vec3<f32>(0.40, -0.05, 0.0), 0.007, 80.0);
        let term = mm_life_blob(p, vec3<f32>(0.44, -0.06, 0.0), 0.028, 58.0);
        col += c_cyan * (soma + dend) * (0.55 + 0.6 * g0) + c_blue * axon * 0.8 + c_gold * term * 0.8;
        dens += (soma + dend) * 0.65 + axon * 0.6 + term * 0.7;
        if (feat == 1u) {
            // impulso eléctrico: action potential leaping along myelinated axon
            let px = mix(-0.21, 0.40, fract(time * 0.55));
            let spike = exp(-pow(abs(p.x - px) * 18.0, 2.0)) * exp(-pow(abs(p.y + 0.02 + (p.x + 0.21) * 0.05) * 40.0, 2.0)) * exp(-pow(p.z * 40.0, 2.0));
            let my = step(0.25, fract(p.x * 9.0)) * axon * 0.6;
            col += c_gold * spike * 1.3 + c_cyan * my;
            dens += spike * 0.9 + my * 0.5;
        } else if (feat == 2u) {
            // sinapsis: vesicles crossing the cleft to a dendritic spine
            let spine = mm_life_shell(p, vec3<f32>(0.57, -0.06, 0.0), 0.035, 60.0);
            col += c_life * spine;
            dens += spine * 0.7;
            for (var k = 0; k < 3; k = k + 1) {
                let vx = 0.47 + fract(time * 0.6 + f32(k) * 0.33) * 0.07;
                let ves = mm_life_blob(p, vec3<f32>(vx, -0.06 + 0.02 * f32(k - 1), 0.0), 0.010, 70.0);
                col += c_mag * ves * 1.2;
                dens += ves * 0.8;
            }
        } else if (feat == 3u) {
            // reflejo: sensory -> spinal -> motor reflex arc
            let arc = doc_line_glow(sd_torus(doc_rot_x(p - vec3<f32>(0.0, -0.30, 0.0), 1.5708), 0.28, 0.008), 78.0) * step(p.y, -0.30);
            let cord = exp(-pow(abs(sd_box(p - vec3<f32>(0.0, -0.58, 0.0), vec3<f32>(0.05, 0.03, 0.05))) * 50.0, 2.0));
            col += c_amber * arc + c_gold * cord;
            dens += arc * 0.7 + cord * 0.7;
        } else if (feat == 4u) {
            // integración sensorial: three inputs converging on the soma
            for (var k = 0; k < 3; k = k + 1) {
                let sx = -0.48 + f32(k) * 0.20;
                let inp = mm_life_seg(p, vec3<f32>(sx, 0.48, 0.0), soma_c, 0.006, 84.0);
                col += select(select(c_mag, c_gold, k == 1), c_life, k == 2) * inp;
                dens += inp * 0.65;
            }
        } else if (feat == 5u) {
            // control motor: motor axon commanding a striated muscle fibre
            let mot = mm_life_seg(p, vec3<f32>(0.44, -0.06, 0.0), vec3<f32>(0.44, -0.30, 0.0), 0.006, 84.0);
            let fib = exp(-pow(abs(sd_box(p - vec3<f32>(0.44, -0.38, 0.0), vec3<f32>(0.14, 0.04, 0.04))) * 50.0, 2.0)) * (0.6 + 0.4 * step(0.5, fract(p.x * 30.0)));
            col += c_blue * mot + c_red * fib;
            dens += mot * 0.6 + fib * 0.75;
        } else if (feat == 6u) {
            // red distribuida: diffuse nerve net mesh (cnidarian)
            let nq = (p / max(r, 1e-3)) * 5.0;
            let mesh = exp(-pow(length(fract(nq) - 0.5) * 6.0, 2.0)) * exp(-pow(abs(r - 0.55) * 30.0, 2.0));
            col += c_violet * mesh * 1.1;
            dens += mesh * 0.7;
        } else if (feat == 7u) {
            // centralización: ganglia condensing into a ventral cord
            for (var k = 0; k < 5; k = k + 1) {
                let gx = -0.36 + f32(k) * 0.18;
                let gang = mm_life_blob(p, vec3<f32>(gx, -0.44, 0.0), 0.024, 60.0);
                col += c_gold * gang;
                dens += gang * 0.8;
            }
            let cord = mm_life_seg(p, vec3<f32>(-0.36, -0.44, 0.0), vec3<f32>(0.36, -0.44, 0.0), 0.005, 86.0);
            col += c_cyan * cord;
            dens += cord * 0.5;
        }
    } else if (zone == 16u) {
        // Z16: Brains & Learning — comparative cortex silhouette with learning circuits
        let bp = p * 1.15;
        let binfo = mm_brain_sdf(bp);
        let bd = binfo.x / 1.15;
        let reg = binfo.y;
        let shell_g = exp(-pow(abs(bd) * 52.0, 2.0));
        var region_col = vec3<f32>(0.36, 0.72, 1.18);
        if (reg > 2.5) {
            region_col = vec3<f32>(0.62, 0.44, 1.10);
        } else if (reg > 1.5) {
            region_col = vec3<f32>(0.26, 1.10, 0.84);
        }
        col += region_col * shell_g * (0.45 + 0.35 * smoothstep(-0.35, 0.65, binfo.z));
        dens += shell_g * 0.55;
        if (feat == 0u) {
            // memoria: bilateral hippocampal C-arcs lighting up
            let hp = vec3<f32>(abs(bp.x) - 0.12, bp.y + 0.06, bp.z + 0.02);
            let hip = exp(-pow(abs(length(hp.yz) - 0.14) * 50.0, 2.0)) * exp(-pow(abs(hp.x) * 32.0, 2.0));
            col += c_gold * hip * 1.2;
            dens += hip * 0.85;
        } else if (feat == 1u) {
            // plasticidad: strengthening synapse between two neurons
            let na = vec3<f32>(-0.10, 0.10, 0.10);
            let nb = vec3<f32>(0.10, 0.12, 0.02);
            let link = mm_life_seg(p, na, nb, 0.006 + 0.004 * sin(time * 2.0), 78.0);
            let ends = mm_life_blob(p, na, 0.020, 62.0) + mm_life_blob(p, nb, 0.020, 62.0);
            col += c_cyan * link * 1.1 + c_gold * ends;
            dens += link * 0.75 + ends * 0.8;
        } else if (feat == 2u) {
            // predicción: frontal forward-model beam reaching ahead
            let beam = mm_life_seg(p, vec3<f32>(0.0, 0.06, 0.38), vec3<f32>(0.0, 0.10, 0.66), 0.010, 76.0);
            let ghost = mm_life_shell(p, vec3<f32>(0.0, 0.10, 0.70), 0.04, 60.0);
            col += c_cyan * beam + c_violet * ghost;
            dens += beam * 0.7 + ghost * 0.7;
        } else if (feat == 3u) {
            // recompensa: dopamine burst from the midbrain
            let da = mm_life_blob(p, vec3<f32>(0.0, -0.12, -0.02), 0.035, 54.0);
            col += c_gold * da * 1.2;
            dens += da * 0.85;
            for (var k = 0; k < 4; k = k + 1) {
                let a = f32(k) * 1.5707963 + 0.4;
                let ray = mm_life_seg(p, vec3<f32>(0.0, -0.12, -0.02), vec3<f32>(cos(a) * 0.20, 0.10, sin(a) * 0.20), 0.004, 88.0);
                col += c_amber * ray;
                dens += ray * 0.5;
            }
        } else if (feat == 4u) {
            // atención: spotlight beam focusing on parietal cortex
            let spot = mm_life_seg(p, vec3<f32>(0.30, 0.58, 0.0), vec3<f32>(0.12, 0.22, 0.0), 0.030, 40.0);
            let focus = shell_g * smoothstep(0.14, 0.04, length(p - vec3<f32>(0.12, 0.22, 0.0)));
            col += c_gold * spot * 0.7 + c_gold * focus * 1.2;
            dens += spot * 0.45 + focus * 0.8;
        } else if (feat == 5u) {
            // decisión: prefrontal glow with a forking choice ahead
            let pfc = shell_g * smoothstep(0.16, 0.26, p.z);
            let fa = mm_life_seg(p, vec3<f32>(0.0, 0.0, 0.42), vec3<f32>(-0.14, 0.0, 0.60), 0.007, 82.0);
            let fb = mm_life_seg(p, vec3<f32>(0.0, 0.0, 0.42), vec3<f32>(0.14, 0.0, 0.60), 0.007, 82.0);
            col += c_mag * pfc * 1.1 + c_gold * (fa + fb);
            dens += pfc * 0.75 + (fa + fb) * 0.65;
        } else if (feat == 6u) {
            // instinto: ancient brainstem circuits glowing
            let stem = mm_life_seg(p, vec3<f32>(0.0, -0.12, -0.035), vec3<f32>(0.0, -0.45, -0.11), 0.05, 50.0);
            col += c_red * stem * 0.9;
            dens += stem * 0.7;
        } else if (feat == 7u) {
            // flexibilidad: switching loop alternating strategies around frontal cortex
            let fl = doc_line_glow(sd_torus(doc_rot_x(p - vec3<f32>(0.0, 0.06, 0.30), 1.5708), 0.14, 0.008), 78.0);
            let sw = select(c_cyan, c_mag, sin(atan2(p.y - 0.06, p.x) * 2.0 + time * 2.0) > 0.0);
            col += sw * fl * 1.1;
            dens += fl * 0.75;
        }
    } else if (zone == 17u) {
        // Z17: Perception — thalamo-cortical predictive loop (bottom-up cyan vs top-down gold)
        let thal = mm_life_blob(p, vec3<f32>(0.0, -0.10, 0.0), 0.06, 50.0);
        let ctx = mm_life_shell(p, vec3<f32>(0.0, -0.10, 0.0), 0.46, 40.0) * smoothstep(0.02, 0.18, p.y);
        let up_a = mm_life_seg(p, vec3<f32>(-0.06, -0.04, 0.0), vec3<f32>(-0.22, 0.32, 0.0), 0.006, 84.0);
        let dn_a = mm_life_seg(p, vec3<f32>(0.22, 0.32, 0.0), vec3<f32>(0.06, -0.04, 0.0), 0.006, 84.0);
        col += c_gold * thal * 0.9 + c_blue * ctx * 0.5 + c_cyan * up_a + c_gold * dn_a;
        dens += thal * 0.75 + ctx * 0.45 + (up_a + dn_a) * 0.6;
        if (feat == 0u) {
            // color: spectral ring decoded into hue
            let ring_s = doc_line_glow(sd_torus(doc_rot_x(p - vec3<f32>(0.0, 0.0, 0.30), 1.5708), 0.14, 0.012), 72.0);
            let a = atan2(p.y, p.x);
            let hue = vec3<f32>(0.55 + 0.5 * cos(a), 0.55 + 0.5 * cos(a - 2.094), 0.55 + 0.5 * cos(a + 2.094));
            col += hue * ring_s;
            dens += ring_s * 0.75;
        } else if (feat == 1u) {
            // sonido: concentric sound wavefronts arriving from the left
            for (var k = 0; k < 3; k = k + 1) {
                let sw = mm_life_shell(p, vec3<f32>(-0.66, 0.0, 0.0), 0.14 + f32(k) * 0.08, 60.0) * step(-0.62, p.x) * smoothstep(0.20, 0.10, abs(p.z));
                col += c_violet * sw;
                dens += sw * 0.65;
            }
        } else if (feat == 2u) {
            // forma: wireframe solid being recognised as shape
            let bq = doc_rot_y(p - vec3<f32>(0.40, 0.22, 0.0), 0.6);
            let bx = exp(-pow(abs(sd_box(bq, vec3<f32>(0.08))) * 60.0, 2.0));
            col += c_cyan * bx;
            dens += bx * 0.75;
        } else if (feat == 3u) {
            // movimiento: moving target with motion trail
            let mx = -0.40 + 0.80 * fract(time * 0.3);
            let dot_m = mm_life_blob(p, vec3<f32>(mx, 0.40, 0.0), 0.024, 60.0);
            let trail = mm_life_seg(p, vec3<f32>(mx - 0.20, 0.40, 0.0), vec3<f32>(mx, 0.40, 0.0), 0.006, 84.0);
            col += c_gold * dot_m + c_amber * trail * 0.7;
            dens += dot_m * 0.85 + trail * 0.5;
        } else if (feat == 4u) {
            // mapa espacial: hexagonal grid-cell lattice on the floor
            let gp = vec2<f32>(p.x * 7.0, (p.z + p.x * 0.577) * 7.0);
            let hexd = length(fract(gp) - 0.5);
            let grid = exp(-pow(hexd * 6.0, 2.0)) * exp(-pow(abs(p.y + 0.40) * 50.0, 2.0)) * smoothstep(0.50, 0.30, length(p.xz));
            col += c_life * grid * 1.1;
            dens += grid * 0.75;
        } else if (feat == 5u) {
            // reconocimiento: face template matching (outline, eyes, mouth)
            let fc = vec3<f32>(-0.34, 0.26, 0.0);
            let outline = doc_line_glow(sd_torus(doc_rot_x(p - fc, 1.5708), 0.10, 0.007), 80.0);
            let eyes = mm_life_blob(p, fc + vec3<f32>(-0.035, 0.03, 0.0), 0.012, 70.0) + mm_life_blob(p, fc + vec3<f32>(0.035, 0.03, 0.0), 0.012, 70.0);
            let mouth = mm_life_seg(p, fc + vec3<f32>(-0.04, -0.04, 0.0), fc + vec3<f32>(0.04, -0.04, 0.0), 0.005, 86.0);
            col += c_gold * outline + c_cyan * (eyes + mouth);
            dens += outline * 0.65 + (eyes + mouth) * 0.75;
        } else if (feat == 6u) {
            // cuerpo: somatotopic body-map strip across the cortex
            let strip = ctx * smoothstep(0.06, 0.02, abs(p.x));
            col += c_mag * strip * 1.6;
            dens += strip * 0.9;
        } else if (feat == 7u) {
            // interpretación: prediction meets evidence in a bright collision ring
            let coll = doc_line_glow(sd_torus(p - vec3<f32>(0.0, 0.14, 0.0), 0.16, 0.010), 76.0);
            let mix_c = mix(c_cyan, c_gold, 0.5 + 0.5 * sin(atan2(p.z, p.x) * 3.0));
            col += mix_c * coll * 1.1;
            dens += coll * 0.8;
        }
    } else {
        // Z18: The Conscious Experience — global-workspace ignition + bridge portal into Exp 17
        // A quiet cortical envelope locates the workspace nodes inside the brain.
        let cortex_d = mm_brain_sdf(p * 0.82).x / 0.82;
        let cortex = exp(-pow(abs(cortex_d) * 52.0, 2.0));
        col += c_violet * cortex * 0.22;
        dens += cortex * 0.12;
        let core = mm_life_blob(p, vec3<f32>(0.0), 0.06, 40.0);
        let g6 = doc_life_feat_gain(feat, 6u, cue);
        col += c_gold * core * 0.8;
        dens += core * 0.7;
        for (var k = 0; k < 6; k = k + 1) {
            let a = f32(k) * 1.0471976 + time * 0.12;
            let np = vec3<f32>(cos(a) * 0.40, 0.08 * sin(a * 3.0), sin(a) * 0.40);
            let node = mm_life_blob(p, np, 0.022, 60.0);
            let link = mm_life_seg(p, vec3<f32>(0.0), np, 0.004, 88.0);
            col += c_cyan * node * (0.6 + 0.6 * g6) + c_blue * link * 0.5;
            dens += node * 0.7 + link * 0.4;
        }
        if (feat == 0u) {
            // vigilia: arousal column rising from the brainstem with alpha ring
            let col_v = mm_life_seg(p, vec3<f32>(0.0, -0.58, 0.0), vec3<f32>(0.0, -0.04, 0.0), 0.016, 66.0);
            let alpha = doc_line_glow(sd_torus(p - vec3<f32>(0.0, -0.30, 0.0), 0.10, 0.007), 80.0);
            col += c_amber * col_v + c_gold * alpha;
            dens += col_v * 0.75 + alpha * 0.6;
        } else if (feat == 1u) {
            // experiencia subjetiva: warm inner qualia shell
            let q1 = mm_life_shell(p, vec3<f32>(0.0), 0.13, 48.0);
            col += vec3<f32>(1.22, 0.72, 0.52) * q1 * 1.1;
            dens += q1 * 0.8;
        } else if (feat == 2u) {
            // atención: spotlight beam selecting one cortical node
            let a0 = time * 0.12;
            let tp = vec3<f32>(cos(a0) * 0.40, 0.08 * sin(a0 * 3.0), sin(a0) * 0.40);
            let beam = mm_life_seg(p, vec3<f32>(0.0, 0.50, 0.0), tp, 0.020, 50.0);
            col += c_gold * beam * 0.9;
            dens += beam * 0.6;
        } else if (feat == 3u) {
            // modelo de sí: recursively nested self-model shells
            for (var k = 0; k < 3; k = k + 1) {
                let ns = mm_life_shell(p, vec3<f32>(0.0, 0.0, 0.0), 0.08 + f32(k) * 0.07, 60.0) * step(0.0, sin(atan2(p.z, p.x) * 4.0 + f32(k)));
                col += mix(c_cyan, c_mag, f32(k) * 0.5) * ns;
                dens += ns * 0.6;
            }
        } else if (feat == 4u) {
            // comunicación: two minds linked by a shared arc
            let ma = mm_life_shell(p, vec3<f32>(-0.40, 0.34, 0.0), 0.06, 58.0);
            let mb = mm_life_shell(p, vec3<f32>(0.40, 0.34, 0.0), 0.06, 58.0);
            let arc = doc_line_glow(sd_torus(doc_rot_x(p - vec3<f32>(0.0, 0.34, 0.0), 1.5708), 0.40, 0.007), 80.0) * step(0.34, p.y);
            col += c_cyan * (ma + mb) + c_gold * arc;
            dens += (ma + mb) * 0.7 + arc * 0.6;
        } else if (feat == 5u) {
            // conciencia animal: other animal minds around the lower ring
            for (var k = 0; k < 4; k = k + 1) {
                let a = f32(k) * 1.5707963 + 0.4;
                let am = mm_life_shell(p, vec3<f32>(cos(a) * 0.36, -0.36, sin(a) * 0.36), 0.045, 60.0);
                col += mix(c_life, c_amber, f32(k) / 3.0) * am;
                dens += am * 0.7;
            }
        } else if (feat == 6u) {
            // correlato neural: synchronous gamma ring binding every node
            let sync = doc_line_glow(sd_torus(p, 0.40, 0.010), 74.0) * (0.7 + 0.3 * sin(time * 8.0));
            col += c_cyan * sync * 1.1;
            dens += sync * 0.75;
        } else if (feat == 7u) {
            // teorías: four candidate theory glyphs around the workspace
            let t1 = exp(-pow(abs(sd_octahedron(p - vec3<f32>(-0.36, 0.36, 0.0), 0.07)) * 60.0, 2.0));
            let t2 = exp(-pow(abs(sd_box(p - vec3<f32>(0.36, 0.36, 0.0), vec3<f32>(0.05))) * 60.0, 2.0));
            let t3 = doc_line_glow(sd_torus(p - vec3<f32>(-0.36, -0.36, 0.0), 0.06, 0.008), 80.0);
            let t4 = exp(-pow(abs(sd_cone_up(p - vec3<f32>(0.36, -0.42, 0.0), vec2<f32>(0.5, 0.86), 0.12)) * 60.0, 2.0));
            col += c_gold * t1 + c_cyan * t2 + c_mag * t3 + c_life * t4;
            dens += (t1 + t2 + t3 + t4) * 0.7;
        } else if (feat == 8u) {
            // siguiente viaje: emerald-to-violet portal bridging into Exp 17 Mind
            let portal = doc_line_glow(sd_torus(doc_rot_z(p - vec3<f32>(0.44, 0.0, 0.0), 1.5708), 0.20, 0.014), 70.0);
            let bridge = mm_life_seg(p, vec3<f32>(0.0), vec3<f32>(0.44, 0.0, 0.0), 0.008, 80.0);
            let pt = 0.5 + 0.5 * p.y / 0.2;
            col += mix(c_life, c_violet, clamp(pt, 0.0, 1.0)) * portal * 1.2 + c_gold * bridge;
            dens += portal * 0.85 + bridge * 0.6;
        }
    }

    let pulse = (1.0 + bass * 0.12 + voice * 0.08 + cue * 0.06) * (0.90 + 0.10 * w);
    let zone_tint = mix(vec3<f32>(1.0), clamp(col_b, vec3<f32>(0.22), vec3<f32>(1.24)), 0.06);
    return vec4<f32>(clamp(col * pulse * zone_tint, vec3<f32>(0.0), vec3<f32>(1.38)), clamp(dens, 0.0, 2.2));
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
    let feat = doc_feat(inst);
    let weight = doc_weight(inst);

    let is_canon = doc_is_canonical_zone_vid(vid);
    let canon_exp = select(0u, doc_vid_exp(vid), is_canon);
    let canon_zone = select(zone, doc_vid_zone(vid), is_canon);
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

            if (is_canon && canon_exp == 15u) {
                let st = shade_doc_complexity_stage(canon_zone, feat, p_raw, p, r, cue, weight, time, voice, bass, inst.color_a.rgb, inst.color_b.rgb);
                col = st.rgb;
                dens = st.a;
            } else if (is_canon && canon_exp == 16u) {
                let st = shade_doc_life_stage(canon_zone, feat, p, r, cue, weight, time, voice, bass, inst.color_a.rgb, inst.color_b.rgb);
                col = st.rgb;
                dens = st.a;
            } else if (mode_group == 0u) {
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
