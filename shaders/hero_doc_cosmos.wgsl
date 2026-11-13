// HyperCosmosVR — Documentary Cosmos Domain Shaders (Apps 03 Deep Earth, 04 Solar System, 05..10 Cosmology)
// Concatenated after common_oled.wgsl + doc_common.wgsl.

fn shade_doc_earth_surface(
    p_surf: vec3<f32>,
    n_surf: vec3<f32>,
    v_dir: vec3<f32>,
    sun_local: vec3<f32>,
    inst: HeroVolumeInstance,
    time: f32,
) -> vec3<f32> {
    let ndotl = max(dot(n_surf, sun_local), 0.0);
    let view_dot = max(dot(n_surf, v_dir), 0.0);
    let fresnel = pow(1.0 - view_dot, 3.6);
    let z_idx = doc_zone(inst);
    let w_doc = doc_weight(inst);

    let spin_p = doc_rot_y(p_surf, time * 0.03 + inst.phys_a.x * 0.5);
    let cont = fbm3(spin_p * 2.4 + vec3<f32>(0.4, -0.2, 0.9), 5);
    let mtn = pow(max(0.0, snoise3(spin_p * 11.0)), 2.0) * smoothstep(0.04, 0.25, cont);
    let coast = smoothstep(0.02, 0.08, cont);
    let polar_ice = smoothstep(0.76, 0.92, abs(p_surf.y) + cont * 0.12);

    let ocean_col = mix(vec3<f32>(0.01, 0.045, 0.16), vec3<f32>(0.02, 0.14, 0.34), smoothstep(-0.32, 0.01, cont));
    let ocean_trench = exp(-abs(cont + 0.18) * 28.0) * (1.0 - coast) * select(0.0, 0.65, z_idx == 6u || z_idx == 7u);
    let land_base = mix(vec3<f32>(0.045, 0.18, 0.06), vec3<f32>(0.24, 0.18, 0.10), smoothstep(0.06, 0.38, cont));
    var albedo = mix(ocean_col + vec3<f32>(0.02, 0.35, 0.55) * ocean_trench, land_base + vec3<f32>(0.18, 0.14, 0.10) * mtn, coast);
    albedo = mix(albedo, vec3<f32>(0.85, 0.90, 0.96), polar_ice);

    // Troposphere spiral clouds + lightning flashes (zone 5 highlights storm systems!)
    let cloud_n = fbm3(spin_p * 4.4 + vec3<f32>(time * 0.016, 0.0, -time * 0.01), 4);
    let clouds = smoothstep(0.14, 0.46, cloud_n);
    let storm_bolt = pow(max(0.0, snoise3(spin_p * 19.0 + vec3<f32>(time * 3.5))), 14.0) * clouds * select(0.15, 1.4, z_idx == 5u);

    let half_v = normalize(sun_local + v_dir);
    let ocean_spec = pow(max(dot(n_surf, half_v), 0.0), 64.0) * (1.0 - coast) * (1.0 - clouds) * 1.35;

    let night_mask = smoothstep(0.08, -0.22, dot(n_surf, sun_local));
    let city_spark = pow(max(0.0, snoise3(spin_p * 24.0)), 6.0) * coast * (1.0 - polar_ice);
    let city_glow = vec3<f32>(1.75, 1.05, 0.34) * city_spark * night_mask * 1.5;

    // Ozone indigo glint on stratosphere zone (4), blue Rayleigh atmospheric limb
    let ozone_glow = vec3<f32>(0.38, 0.22, 1.65) * fresnel * select(0.1, 0.95, z_idx == 4u);
    let rayleigh = inst.color_b.rgb * fresnel * (0.35 + 1.35 * smoothstep(-0.2, 0.4, dot(n_surf, sun_local)));

    let day_lit = mix(albedo, vec3<f32>(0.90), clouds) * (ndotl + 0.025) + vec3<f32>(ocean_spec);
    let zone_rim = mix(1.0, 1.25, w_doc * fresnel);
    return (day_lit + city_glow + rayleigh + ozone_glow + vec3<f32>(0.55, 0.85, 1.95) * storm_bolt) * inst.color_a.a * zone_rim;
}

fn shade_doc_earth_cutaway(
    p_wall: vec3<f32>,
    surf_r: f32,
    inst: HeroVolumeInstance,
    time: f32,
) -> vec3<f32> {
    let wall_r = length(p_wall) / max(surf_r, 1e-4);
    let z_idx = doc_zone(inst);
    let conv = fbm3(p_wall * 7.5 + vec3<f32>(0.0, time * 0.08, 0.0), 4);
    let plume = pow(max(0.0, sin(atan2(p_wall.z, p_wall.x) * 4.0 + conv * 2.4)), 4.0);
    let subduct = exp(-abs(wall_r - 0.93 + max(p_wall.y, 0.0) * 0.28) * 34.0) * smoothstep(0.1, 0.5, p_wall.z / max(surf_r, 1e-4));

    var col = vec3<f32>(0.0);
    if (wall_r > 0.91) {
        // Crust & Tectonic Lithosphere Slab (zones 7, 8)
        let hi = select(1.0, 1.65, z_idx == 7u || z_idx == 8u);
        col = vec3<f32>(0.24, 0.19, 0.14) * (0.72 + 0.45 * conv) * hi + vec3<f32>(1.1, 0.42, 0.15) * subduct;
    } else if (wall_r > 0.78) {
        // Asthenosphere (zone 9) ductile flow + partial melt
        let hi = select(1.0, 1.55, z_idx == 9u);
        col = mix(vec3<f32>(0.55, 0.14, 0.03), vec3<f32>(1.25, 0.42, 0.08), clamp(0.55 + conv * 0.6, 0.0, 1.0)) * hi;
    } else if (wall_r > 0.68) {
        // Mantle Transition Zone (660 km discontinuity ruby ring, zone 10)
        let hi = select(1.0, 1.75, z_idx == 10u);
        col = mix(vec3<f32>(0.68, 0.16, 0.04), vec3<f32>(1.55, 0.58, 0.14), plume) * hi;
    } else if (wall_r > 0.46) {
        // Lower Mantle bridgmanite + rising hot superplumes + D'' layer (zone 11)
        let hi = select(1.0, 1.6, z_idx == 11u);
        col = mix(vec3<f32>(0.42, 0.08, 0.02), vec3<f32>(1.35, 0.46, 0.08), clamp(0.45 + plume * 0.75 + conv * 0.4, 0.0, 1.0)) * hi;
    } else {
        // Liquid Fe-Ni Outer Core helical geodynamo shear (zone 12)
        let hi = select(1.0, 1.55, z_idx == 12u);
        let helix = 0.5 + 0.5 * sin(p_wall.y * 28.0 + atan2(p_wall.z, p_wall.x) * 3.0 - time * 1.4);
        col = blackbody_rgb(mix(3200.0, 4600.0, clamp(0.45 + helix * 0.4 + conv * 0.25, 0.0, 1.0))) * 1.25 * hi;
    }

    let seam_660 = exp(-pow((wall_r - 0.72) * 44.0, 2.0)) * 0.42;
    let seam_cmb = exp(-pow((wall_r - 0.46) * 44.0, 2.0)) * 0.58;
    let seam_moho = exp(-pow((wall_r - 0.91) * 48.0, 2.0)) * 0.28;
    col += vec3<f32>(1.45, 0.72, 0.22) * (seam_660 + seam_cmb + seam_moho);
    return col;
}

fn shade_doc_solar_planet_surface(
    vid: u32,
    p_surf: vec3<f32>,
    n_surf: vec3<f32>,
    v_dir: vec3<f32>,
    sun_local: vec3<f32>,
    inst: HeroVolumeInstance,
    time: f32,
) -> vec3<f32> {
    let ndotl = max(dot(n_surf, sun_local), 0.0);
    let view_dot = max(dot(n_surf, v_dir), 0.0);
    let fresnel = pow(1.0 - view_dot, 3.6);

    if (vid == 1u) {
        // Solar Photosphere: convective granules + dark magnetic umbra sunspots + chromosphere limb
        let gran = fbm3(p_surf * 13.5 + vec3<f32>(time * 0.14), 4);
        let cells = abs(snoise3(p_surf * 26.0 + vec3<f32>(0.0, time * 0.25, 0.0)));
        let gran_lanes = smoothstep(0.04, 0.26, cells);
        let spot = smoothstep(-0.30, -0.48, fbm3(p_surf * 3.8 + vec3<f32>(0.0, time * 0.03, 0.0), 3));
        let temp = mix(5200.0, 2850.0, spot) + gran * 980.0 - (1.0 - gran_lanes) * 650.0;
        let limb = pow(view_dot, 0.62);
        let chromo_rim = vec3<f32>(0.98, 0.20, 0.04) * fresnel * 0.68;
        let cell_contrast = (0.34 + 0.66 * gran_lanes) * mix(1.0, 0.22, spot);
        return (blackbody_rgb(temp) * (0.36 + 0.56 * limb + u_audio.levels_a.z * 0.12) * cell_contrast + chromo_rim) * min(inst.color_a.a, 0.92);
    }

    if (vid == 3u) {
        // Mercury: cratered basalt/regolith basins + sunlit bright scarps
        let crater_a = abs(snoise3(p_surf * 9.5 + vec3<f32>(1.2)));
        let crater_rim = exp(-abs(crater_a - 0.22) * 28.0);
        let regolith = fbm3(p_surf * 4.5, 4);
        let col = mix(vec3<f32>(0.24, 0.22, 0.20), vec3<f32>(0.55, 0.51, 0.46), clamp(0.5 + regolith * 0.5 + crater_rim * 0.45, 0.0, 1.0));
        return col * (ndotl * 1.15 + 0.02) * inst.color_a.a;
    }

    if (vid == 4u) {
        // Venus: super-rotating cream/amber sulfuric acid chevron cloud bands + thick haze limb
        let warp = fbm3(p_surf * 3.2 + vec3<f32>(time * 0.08, 0.0, 0.0), 4);
        let chevron = sin(p_surf.y * 8.0 + abs(p_surf.x) * 3.5 + warp * 2.8);
        let col = mix(vec3<f32>(0.56, 0.40, 0.20), vec3<f32>(0.96, 0.80, 0.48), 0.5 + 0.5 * chevron);
        let limb = vec3<f32>(1.1, 0.85, 0.42) * fresnel * (0.3 + 0.7 * ndotl);
        return (col * (ndotl * 1.05 + 0.02) + limb) * inst.color_a.a;
    }

    if (vid == 5u) {
        // Mars: ferric rust dunes + dark Syrtis basalt + Valles Marineris canyon scar + bright white polar ice caps
        let ice = smoothstep(0.84, 0.94, abs(p_surf.y) + snoise3(p_surf * 8.0) * 0.06);
        let canyon = exp(-abs(p_surf.y - 0.06) * 22.0) * smoothstep(0.45, 0.15, abs(p_surf.x)) * smoothstep(0.0, 0.4, p_surf.z);
        let terrain = fbm3(p_surf * 5.5, 4);
        var rust = mix(vec3<f32>(0.42, 0.14, 0.06), vec3<f32>(0.88, 0.36, 0.16), clamp(0.5 + terrain * 0.6, 0.0, 1.0));
        rust = mix(rust, vec3<f32>(0.22, 0.07, 0.03), canyon * 0.75);
        rust = mix(rust, vec3<f32>(0.92, 0.94, 0.98), ice);
        let atmo = vec3<f32>(0.88, 0.44, 0.25) * fresnel * 0.32 * (0.2 + 0.8 * ndotl);
        return (rust * (ndotl * 1.12 + 0.02) + atmo) * inst.color_a.a;
    }

    if (vid == 6u) {
        // Jupiter: turbulent cream/ochre/terracotta belts + swirling Great Red Spot + polar indigo aurora
        let shear = curl_noise3(p_surf * 3.8 + vec3<f32>(time * 0.03, 0.0, 0.0));
        let belt = sin(p_surf.y * 19.0 + shear.y * 1.8);
        let fine_edd = fbm3(p_surf * 9.0 + shear * 0.4, 3);
        var jup = mix(vec3<f32>(0.78, 0.46, 0.26), vec3<f32>(0.94, 0.82, 0.64), clamp(0.5 + 0.5 * belt + fine_edd * 0.25, 0.0, 1.0));
        let grs_d = length(vec2<f32>((p_surf.x - 0.18) * 1.2, (p_surf.y + 0.24) * 2.1)) + select(1.0, 0.0, p_surf.z > 0.0);
        let grs = exp(-grs_d * grs_d * 26.0);
        jup = mix(jup, vec3<f32>(0.92, 0.24, 0.12), grs);
        let pole_aur = exp(-pow((abs(p_surf.y) - 0.88) * 14.0, 2.0)) * vec3<f32>(0.45, 0.65, 1.75) * 0.55;
        return (jup * (ndotl * 1.1 + 0.02) + pole_aur) * inst.color_a.a;
    }

    if (vid == 7u || vid == 8u) {
        // Uranus (7: calm cyan-aquamarine methane orb) / Neptune (8: vivid cobalt-azure + dark oval + cirrus streaks)
        let n_band = fbm3(p_surf * 5.2 + vec3<f32>(time * 0.04, 0.0, 0.0), 3);
        let bands = sin(p_surf.y * select(12.0, 6.0, vid == 7u) + n_band * 2.0);
        var col = mix(inst.color_a.rgb, inst.color_b.rgb, 0.5 + 0.4 * bands);
        if (vid == 8u) {
            let dark_spot = exp(-length(vec2<f32>(p_surf.x + 0.14, (p_surf.y + 0.18) * 1.9)) * 12.0) * select(0.0, 1.0, p_surf.z > 0.0);
            let cirrus = pow(max(0.0, sin(p_surf.y * 28.0 + n_band * 4.0)), 8.0) * 0.35;
            col = mix(col, vec3<f32>(0.04, 0.08, 0.32), dark_spot) + vec3<f32>(0.6, 0.8, 1.0) * cirrus;
        }
        return (col * (ndotl * 1.1 + 0.02) + inst.color_b.rgb * fresnel * 0.35) * inst.color_a.a;
    }

    if (vid == 9u) {
        // Pluto: copper Cthulhu tholin terrain + iconic cream nitrogen-ice heart (Sputnik Planitia)
        let heart_l = length((p_surf.xy - vec2<f32>(-0.08, -0.02)) * vec2<f32>(1.5, 1.1));
        let heart_r = length((p_surf.xy - vec2<f32>(0.09, -0.02)) * vec2<f32>(1.5, 1.1));
        let heart = smoothstep(0.28, 0.16, min(heart_l, heart_r)) * select(0.0, 1.0, p_surf.z > 0.15);
        let tholin = fbm3(p_surf * 5.5, 4);
        var col = mix(vec3<f32>(0.38, 0.18, 0.14), vec3<f32>(0.78, 0.62, 0.50), clamp(0.5 + tholin * 0.6, 0.0, 1.0));
        col = mix(col, vec3<f32>(0.94, 0.89, 0.80), heart);
        return col * (ndotl * 1.15 + 0.02) * inst.color_a.a;
    }

    if (vid == 10u) {
        // Europa / Enceladus (21 Cosmic Life Ocean Worlds): cracked bright water-ice plates glowing cyan along lineae fractures!
        let crack_n = abs(fbm3(p_surf * 7.5 + vec3<f32>(1.9, -0.7, 2.3), 4));
        let fissure = exp(-crack_n * 22.0);
        let ice = mix(vec3<f32>(0.68, 0.78, 0.88), vec3<f32>(0.24, 0.16, 0.12), fissure * 0.5);
        let subsurface_cyan = vec3<f32>(0.15, 1.25, 1.75) * fissure * 0.95;
        return (ice * (ndotl * 1.08 + 0.025) + subsurface_cyan + vec3<f32>(0.25, 0.75, 1.25) * fresnel * 0.25) * inst.color_a.a;
    }

    if (vid == 11u) {
        // Exoplanet Biosignature World / Dying Star / Material Target
        let n_t = fbm3(p_surf * 4.2 + vec3<f32>(time * 0.02), 4);
        let bio_land = smoothstep(0.05, 0.25, n_t);
        let col = mix(vec3<f32>(0.02, 0.12, 0.28), mix(inst.color_a.rgb, inst.color_b.rgb, clamp(0.5 + n_t, 0.0, 1.0)), bio_land);
        return (col * (ndotl * 1.1 + 0.02) + inst.color_b.rgb * fresnel * 0.5) * inst.color_a.a;
    }

    if (vid == 12u) {
        // App 01 Physical Material Spheres OR Red/Black Dwarf Cooling Remnants
        let m_mode = doc_submode(inst);
        if (m_mode == 1u) {
            // Studio Brushed Gold PBR (sharp anisotropic specular + warm bronze bounce)
            let half_v = normalize(sun_local + v_dir);
            let spec = pow(max(dot(n_surf, half_v), 0.0), 64.0);
            let spec_broad = pow(max(dot(n_surf, half_v), 0.0), 12.0);
            let rim = pow(1.0 - view_dot, 3.0);
            return vec3<f32>(0.98, 0.74, 0.26) * (0.24 + 0.85 * ndotl + spec_broad * 0.45 + rim * 0.55)
                 + vec3<f32>(1.45, 1.22, 0.72) * spec * 1.9;
        } else if (m_mode == 2u) {
            // Thin-Film Iridescent Dielectric Glass (clear dark glass body + bright prismatic rim + crisp specular)
            let irid = iridescence_spectral(view_dot, fbm3(p_surf * 2.5, 2) * 0.85, time * 0.08);
            let half_v = normalize(sun_local + v_dir);
            let spec = pow(max(dot(n_surf, half_v), 0.0), 110.0);
            return irid * (0.26 + fresnel * 1.65) + vec3<f32>(1.75) * spec;
        } else if (m_mode == 3u) {
            // Matte Terracotta Clay Sphere (Oren-Nayar diffuse softness)
            let oren = 0.14 + 0.88 * pow(ndotl, 0.78) + fresnel * 0.12;
            return inst.color_a.rgb * oren;
        }
        // Cooling Crimson / Black Dwarf Ember Crust
        let crack = pow(max(0.0, snoise3(p_surf * 6.5)), 4.0);
        return mix(vec3<f32>(0.08, 0.015, 0.01), vec3<f32>(1.45, 0.28, 0.04), crack * clamp(inst.color_b.a, 0.1, 1.2));
    }

    // Banded Gas Giants (Saturn default vid == 2) w/ North Polar Hexagon Hint
    let band_warp = fbm3(p_surf * 4.5 + vec3<f32>(time * 0.02, 0.0, 0.0), 4);
    let bands = sin(p_surf.y * 15.0 + band_warp * 3.2);
    var surf_col = mix(inst.color_a.rgb, inst.color_b.rgb, 0.5 + 0.5 * bands);
    if (vid == 2u && p_surf.y > 0.78) {
        let hex_r = length(p_surf.xz) * 2.8;
        let hex_ang = atan2(p_surf.z, p_surf.x);
        let hex = exp(-abs(hex_r - 0.58 - 0.08 * cos(6.0 * hex_ang)) * 18.0);
        surf_col = mix(surf_col, vec3<f32>(0.32, 0.85, 1.15), hex * 0.65);
    }
    let rim_glow = inst.color_b.rgb * fresnel * (0.25 + 0.75 * ndotl);
    return (surf_col * (ndotl * 1.12 + 0.018) + rim_glow) * inst.color_a.a;
}

fn raymarch_doc_earth_shell(
    ro: vec3<f32>,
    rd: vec3<f32>,
    t0: f32,
    t1: f32,
    jitter: f32,
    fovea_k: f32,
    inst: HeroVolumeInstance,
    time: f32,
    bass: f32,
) -> vec4<f32> {
    // 03 Earth Magnetosphere Van Allen Flux Lines, Equatorial Exo-Satellites, Mesosphere Meteors & Anchored Auroras!
    var accum_rgb = vec3<f32>(0.0);
    var accum_a: f32 = 0.0;
    let z_idx = doc_zone(inst);
    let steps: i32 = i32(mix(16.0, 28.0, fovea_k));
    let dt = (t1 - t0) / f32(steps);
    let show_magnetosphere = z_idx == 0u || z_idx == 1u || z_idx == 12u || z_idx == 65535u;
    let mag_gain = select(0.28, 0.92, z_idx == 0u || z_idx == 1u || z_idx == 12u);

    for (var i: i32 = 0; i < 28; i = i + 1) {
        if (i >= steps || accum_a > 0.95) {
            break;
        }
        let t = t0 + (f32(i) + jitter) * dt;
        let p = ro + rd * t;
        let r = length(p);

        // 1. Anchored Polar Auroral Oval Curtains touching Earth limb at r = 0.739 .. 0.915 (03 Termosfera)
        if (r > 0.739 && r < 0.915) {
            let lat = abs(p.y / r);
            let oval = exp(-pow((lat - 0.75) * 15.0, 2.0));
            if (oval > 0.01) {
                let h_alt = clamp((r - 0.739) / 0.17, 0.0, 1.0);
                let alt_env = smoothstep(0.0, 0.09, h_alt) * exp(-h_alt * 2.7);
                let b_dir = normalize(vec3<f32>(p.x, p.y * 0.12, p.z) + vec3<f32>(1e-4));
                let field_uv = fract(vec2<f32>(b_dir.x * 0.35 + b_dir.z * 0.25 + 0.5, h_alt));
                let field_s = textureSampleLevel(field_grid_tex, field_grid_samp, clamp(field_uv, vec2<f32>(0.01), vec2<f32>(0.99)), 0.0);
                let shear = curl_noise3(b_dir * 3.8 + vec3<f32>(0.0, time * 0.14, 0.0));
                let curtain = snoise3(b_dir * 7.8 + shear * 0.65 + vec3<f32>(0.0, h_alt * 1.4, time * 0.22));
                let sheet = exp(-pow(curtain * 5.0, 2.0)) * oval * alt_env;
                let aur_gain = select(1.0, 1.65, z_idx == 2u);
                let pde_mod = mix(vec3<f32>(0.08, 1.75, 0.58), field_s.rgb, clamp(field_s.a * 0.45, 0.0, 0.55));
                let alt_col = mix(pde_mod, vec3<f32>(0.95, 0.18, 1.85), smoothstep(0.32, 0.86, h_alt)) * aur_gain;
                let sample_a = sheet * dt * (2.4 + bass * 1.0);
                accum_rgb += (1.0 - accum_a) * alt_col * sample_a;
                accum_a += (1.0 - accum_a) * min(sample_a, 0.44);
            }
        }

        // 2. Dipole Magnetosphere Van Allen Flux Arcs + Satellites / ISS Orbit (r in [0.76, 0.98])
        if (show_magnetosphere && r > 0.76 && r < 0.98) {
            let sin2_t = 1.0 - (p.y * p.y) / (r * r);
            let l_shell = r / max(sin2_t, 0.08);
            let flux_ring = exp(-abs(l_shell - 1.18) * 30.0) + exp(-abs(l_shell - 1.48) * 32.0) * 0.65;
            let phi_ribbons = exp(-abs(sin(atan2(p.z, p.x) * 4.0 + time * 0.08)) * 12.0);
            let mag_a = flux_ring * phi_ribbons * mag_gain * dt * 1.4;
            if (mag_a > 0.002) {
                accum_rgb += (1.0 - accum_a) * vec3<f32>(0.24, 0.72, 1.85) * mag_a;
                accum_a += (1.0 - accum_a) * min(mag_a * 0.55, 0.22);
            }
        }
    }
    return vec4<f32>(accum_rgb, accum_a);
}

fn raymarch_doc_cosmos_macro(
    kind: u32,
    vid: u32,
    ro: vec3<f32>,
    rd: vec3<f32>,
    t0: f32,
    t1: f32,
    jitter: f32,
    fovea_k: f32,
    inst: HeroVolumeInstance,
    sun_local: vec3<f32>,
    time: f32,
    voice: f32,
    bass: f32,
) -> vec4<f32> {
    var accum_rgb = vec3<f32>(0.0);
    var accum_a: f32 = 0.0;
    let z_idx = doc_zone(inst);

    if (kind == 2u) {
        // Analytical Planetary Rings & Goldilocks / Keplerian Orbital Disks
        if (abs(rd.y) > 1e-4) {
            let t_plane = -ro.y / rd.y;
            if (t_plane > t0 && t_plane < t1) {
                let p = ro + rd * t_plane;
                let r = length(p.xz);
                if (vid == 4u) {
                    // Solar System Distance-Compensated Optical Hairline Keplerian Orbit Tracks
                    let ts = select(1.0, inst.phys_a.z, inst.phys_a.z > 1e-4);
                    let r_canon = r / ts;
                    if (r_canon > 0.025 && r_canon < 0.995) {
                        let hit_dist = max(length(p - ro) * inst.pos_radius.w, 1e-3);
                        let px_rad = (2.0 * tan(max(u_frame.cam_forward.w, 0.2) * 0.5))
                            * max(u_frame.viewport_params.w, 1.0 / 1080.0);
                        let px_world = max(hit_dist * px_rad, 1e-4);
                        let max_orb_w = inst.pos_radius.w * ts;
                        let line_w = 1.45 * px_world * mix(1.45, 1.0, clamp(abs(rd.y), 0.0, 1.0));
                        let inv_sig = 1.0 / max(line_w * 0.78, 1e-5);
                        var orbits = array<f32, 11>(
                            0.42 / 11.2,
                            0.68 / 11.2,
                            0.9439 / 11.2,
                            1.48 / 11.2,
                            2.38 / 11.2,
                            3.65 / 11.2,
                            5.05 / 11.2,
                            6.65 / 11.2,
                            8.20 / 11.2,
                            9.55 / 11.2,
                            10.70 / 11.2
                        );
                        var line_sum = 0.0;
                        for (var oi: u32 = 0u; oi < 11u; oi = oi + 1u) {
                            let d_world = abs(r_canon - orbits[oi]) * max_orb_w;
                            let k = d_world * inv_sig;
                            if (k < 3.6) {
                                let w_boost = select(0.76, 1.0, oi == 2u || oi == 6u || (z_idx >= 1u && oi == z_idx - 1u));
                                line_sum += exp(-(k * k)) * w_boost;
                            }
                        }
                        let sun_fade = smoothstep(0.025, 0.036, r_canon) * (1.0 - smoothstep(0.97, 0.995, r_canon));
                        let col = mix(inst.color_a.rgb, inst.color_b.rgb, clamp(r_canon, 0.0, 1.0));
                        let alpha = clamp(line_sum * 0.30 * sun_fade, 0.0, 0.34);
                        if (alpha > 0.002) {
                            return vec4<f32>(col * alpha, alpha);
                        }
                    }
                    return vec4<f32>(0.0);
                }
                if (vid == 3u) {
                    // App 21 Goldilocks Habitable Annulus (razor crisp emerald orbital track)
                    if (r > 0.45 && r < 0.88) {
                        let ring_env = smoothstep(0.45, 0.52, r) * (1.0 - smoothstep(0.80, 0.88, r));
                        let inner_line = exp(-abs(r - 0.52) * 55.0) + exp(-abs(r - 0.80) * 55.0);
                        let orb = exp(-abs(r - 0.66) * 65.0) * 0.9;
                        let col = mix(vec3<f32>(0.12, 0.75, 0.42), vec3<f32>(0.35, 1.55, 0.85), inner_line + orb);
                        let alpha = clamp(ring_env * 0.14 + (inner_line + orb) * 0.58, 0.0, 0.88);
                        return vec4<f32>(col * alpha, alpha);
                    }
                    return vec4<f32>(0.0);
                }
                let r_min = select(0.43, 0.56, vid == 1u);
                if (r > r_min && r < 0.97) {
                    let ring_n = fbm3(vec3<f32>(r * 42.0, sin(p.x * 4.0) * 1.5, cos(p.z * 4.0) * 1.5), 3);
                    let cassini_gap = smoothstep(0.018, 0.042, abs(r - 0.71));
                    let encke_gap = smoothstep(0.007, 0.016, abs(r - 0.88));
                    let fine_bands = 0.52 + 0.48 * sin(r * 155.0 + ring_n * 3.5);
                    let sun_dir_n = normalize(sun_local + vec3<f32>(1e-5));
                    let proj = dot(p, sun_dir_n);
                    let perp = length(p - sun_dir_n * proj);
                    let planet_shadow = select(1.0, smoothstep(0.36, 0.43, perp), proj < 0.0);
                    let alpha = smoothstep(r_min, r_min + 0.03, r) * (1.0 - smoothstep(0.93, 0.97, r)) * cassini_gap * encke_gap * fine_bands * select(1.0, 0.45, vid == 1u);
                    let ring_col = mix(inst.color_a.rgb, inst.color_b.rgb, clamp(r, 0.0, 1.0)) * (1.05 + 0.3 * ring_n) * (0.08 + 0.92 * planet_shadow);
                    return vec4<f32>(ring_col * alpha * inst.color_a.a, clamp(alpha * 0.9, 0.0, 1.0));
                }
            }
        }
        return vec4<f32>(0.0);
    }

    if (kind == 4u) {
        // Kerr Black Hole Lensing, Barred MW Disk (5), Hawking Jets (12), Binary GW Inspiral (14), Einstein Cross (11)
        let steps: i32 = i32(mix(28.0, 50.0, fovea_k));
        let is_einstein_cross = (vid == 11u);
        let rs: f32 = select(0.21, 0.16, is_einstein_cross);
        let step_len: f32 = (t1 - t0) / f32(steps);
        var pos = ro + rd * (t0 + jitter * step_len);
        var vel = rd;
        var min_r: f32 = length(pos);
        var captured: bool = false;
        let is_degenerate = (vid == 12u);
        let is_mw = (vid == 5u);

        for (var i: i32 = 0; i < 50; i = i + 1) {
            if (i >= steps || accum_a > 0.96) {
                break;
            }
            let r = length(pos);
            min_r = min(min_r, r);
            if (!is_einstein_cross && r < rs * 1.12) {
                captured = true;
                accum_a = 1.0;
                break;
            }
            let accel = -pos * (1.45 * rs / max(r * r * r, 1e-4));
            vel = normalize(vel + accel * step_len);
            let next_pos = pos + vel * step_len;

            if (!is_einstein_cross && (pos.y * next_pos.y) <= 0.0 && r > rs * 1.50 && r < 0.78) {
                let frac = abs(pos.y) / max(abs(pos.y - next_pos.y), 1e-5);
                let hit_p = mix(pos, next_pos, frac);
                let disk_r = length(hit_p.xz);
                let angle = atan2(hit_p.z, hit_p.x);
                let spiral = 0.5 + 0.5 * cos(angle * 3.0 + disk_r * 24.0 - time * 1.6);
                let fil_noise = fbm3(vec3<f32>(hit_p.x * 11.0, hit_p.z * 11.0, disk_r * 8.0 - time * 0.45), 3);
                let filament = pow(clamp(0.42 + spiral * 0.55 + fil_noise * 0.65, 0.0, 1.0), 2.1);
                let tangent = normalize(vec3<f32>(-hit_p.z, 0.0, hit_p.x) + vec3<f32>(1e-5));
                let doppler = dot(tangent, -rd);
                let beam = pow(clamp(1.0 + doppler * 0.65, 0.22, 1.85), 2.2);
                let radial_env = smoothstep(rs * 1.50, rs * 2.15, disk_r) * (1.0 - smoothstep(0.46, 0.76, disk_r));
                let temp = select(
                    mix(2100.0, 9200.0, clamp((0.62 - disk_r) * 1.8 + doppler * 0.35, 0.0, 1.0)),
                    mix(1400.0, 3600.0, clamp((0.60 - disk_r) * 1.4, 0.0, 1.0)),
                    is_degenerate
                );
                let disk_alpha = radial_env * filament * select(0.95, 0.55, is_degenerate);
                let disk_col = blackbody_rgb(temp) * beam * select(1.42 + bass * 0.4, 0.72, is_degenerate);
                accum_rgb += (1.0 - accum_a) * disk_col * disk_alpha;
                accum_a += (1.0 - accum_a) * clamp(disk_alpha * 0.85, 0.0, 0.90);
            }

            if (is_mw && r > 0.34 && r < 0.94 && abs(pos.y) < 0.16) {
                let gal_r = length(pos.xz);
                if (gal_r > 0.28) {
                    let gal_ang = atan2(pos.z, pos.x);
                    let bar = exp(-pow(abs(sin(gal_ang - 0.36)) * gal_r * 6.2, 2.0)) * smoothstep(0.58, 0.28, gal_r);
                    let sp_a = pow(max(0.0, cos(2.0 * gal_ang - 5.2 * log(gal_r + 0.14))), 3.4);
                    let sp_b = pow(max(0.0, cos(4.0 * gal_ang - 6.8 * log(gal_r + 0.14) + 1.1)), 4.0) * 0.45;
                    let dust = smoothstep(-0.18, 0.34, fbm3(vec3<f32>(pos.x * 7.5, pos.z * 7.5, pos.y * 14.0), 3));
                    let mw_d = ((sp_a + sp_b) * smoothstep(0.32, 0.46, gal_r) * (1.0 - smoothstep(0.76, 0.94, gal_r)) * 0.65 + bar * 0.42) * dust * exp(-abs(pos.y) * 28.0) * step_len * 2.2;
                    let mw_c = mix(vec3<f32>(0.32, 0.66, 1.55), vec3<f32>(1.55, 1.02, 0.44), clamp(bar * 1.6 + (0.55 - gal_r) * 0.8, 0.0, 1.0));
                    accum_rgb += (1.0 - accum_a) * mw_c * mw_d;
                    accum_a += (1.0 - accum_a) * min(mw_d * 0.55, 0.22);
                }
            }

            if (is_degenerate && r > 0.26 && r < 0.92 && length(pos.xz) < 0.16) {
                // Hawking Polar Radiation Pairs & Evaporating Quantum Jet Cones
                let jet = exp(-length(pos.xz) * 28.0) * (1.0 - smoothstep(0.55, 0.92, abs(pos.y)));
                let pair_spark = pow(max(0.0, sin(abs(pos.y) * 18.0 - time * 3.2)), 6.0);
                let j_a = jet * (0.35 + 0.95 * pair_spark) * step_len * 1.8;
                accum_rgb += (1.0 - accum_a) * mix(vec3<f32>(0.65, 0.45, 1.85), vec3<f32>(0.45, 1.65, 1.95), pair_spark) * j_a;
                accum_a += (1.0 - accum_a) * min(j_a * 0.6, 0.25);
            }

            if (is_einstein_cross && r > 0.30 && r < 0.85 && abs(pos.z) < 0.18) {
                // 4-Image Lensed Quasar Arcs around central lensing galaxy
                let az = atan2(pos.y, pos.x);
                let ring_d = abs(length(pos.xy) - 0.52);
                let four_arcs = pow(max(0.0, cos(az * 2.0)), 6.0) * exp(-ring_d * 48.0) * exp(-abs(pos.z) * 24.0);
                let lens_core = exp(-r * 14.0) * 0.85;
                let ec_a = (four_arcs * 1.45 + lens_core) * step_len * 2.4;
                accum_rgb += (1.0 - accum_a) * mix(vec3<f32>(1.65, 1.15, 0.45), vec3<f32>(0.35, 1.65, 1.95), four_arcs) * ec_a;
                accum_a += (1.0 - accum_a) * min(ec_a * 0.6, 0.35);
            }
            pos = next_pos;
        }
        if (!captured && !is_einstein_cross) {
            let photon = exp(-pow((min_r - rs * 1.48) * 34.0, 2.0));
            let ring_tint = select(vec3<f32>(2.4, 1.45, 0.65), vec3<f32>(1.15, 0.55, 1.65), is_degenerate);
            accum_rgb += (1.0 - accum_a) * ring_tint * photon * 0.72;
            accum_a = max(accum_a, photon * 0.65);
        }
        return vec4<f32>(accum_rgb, accum_a);
    }

    // Kind 3u: Cosmological Macro + Landmarks (vid 4, 6..10, 20..23)
    let steps: i32 = i32(mix(16.0, 30.0, fovea_k));
    let dt = (t1 - t0) / f32(steps);
    for (var i: i32 = 0; i < 30; i = i + 1) {
        if (i >= steps || accum_a > 0.94) {
            break;
        }
        let t = t0 + (f32(i) + jitter) * dt;
        let p = ro + rd * t;
        let r = length(p);
        if (r < 0.94) {
            var neb_col = mix(inst.color_a.rgb, inst.color_b.rgb, 0.5);
            var dens: f32 = 0.0;
            if (vid == 4u) {
                if (r > 0.38 && r < 0.90) {
                    let loop_arc = exp(-abs(r - 0.56 - 0.14 * sin(p.x * 7.5 + time * 0.4) * cos(p.z * 6.5)) * 28.0);
                    let flare = pow(max(0.0, snoise3(p * 7.0 + vec3<f32>(time * 0.25))), 3.0);
                    dens = loop_arc * (0.20 + 0.95 * flare) * (1.0 - smoothstep(0.68, 0.90, r));
                    neb_col = mix(vec3<f32>(1.15, 0.42, 0.10), vec3<f32>(1.55, 0.85, 0.28), flare);
                }
            } else if (vid == 6u) {
                // Local Group: MW + M31 + M33 Triangulum + Magellanic satellite smudges cleanly separated in black void
                let q1 = p - vec3<f32>(-0.36, -0.10, 0.06);
                let r1 = length(q1.xz);
                let bulge1 = exp(-length(q1) * 18.0) * 1.15;
                let arm1 = exp(-abs(q1.y) * 32.0) * pow(max(0.0, cos(atan2(q1.z, q1.x) - 3.4 * log(r1 + 0.05))), 4.5) * exp(-r1 * 5.2);
                let q2_raw = p - vec3<f32>(0.36, 0.12, -0.05);
                let q2 = doc_rot_z(q2_raw, 0.6);
                let r2 = length(q2.xz);
                let bulge2 = exp(-length(q2) * 20.0) * 1.05;
                let arm2 = exp(-abs(q2.y) * 34.0) * pow(max(0.0, cos(atan2(q2.z, q2.x) + 3.8 * log(r2 + 0.05))), 4.5) * exp(-r2 * 5.6);
                let q3 = p - vec3<f32>(0.22, -0.28, 0.18);
                let m33 = exp(-length(q3) * 24.0) * 0.85 + exp(-abs(q3.y) * 36.0) * exp(-length(q3.xz) * 12.0) * 0.45;
                let dust_cut = smoothstep(-0.18, 0.32, snoise3(p * 12.0));
                dens = (bulge1 + arm1 * dust_cut + bulge2 + arm2 * dust_cut + m33) * 0.65;
                neb_col = mix(vec3<f32>(0.35, 0.62, 1.45), vec3<f32>(1.55, 1.12, 0.55), clamp((bulge1 + bulge2) * 1.1, 0.0, 1.0));
            } else if (vid == 7u) {
                // Laniakea: Flowing Watershed Filaments into Great Attractor + M87 Virgo Relativistic Jet
                let ga = p - vec3<f32>(0.18, 0.04, 0.0);
                let r_ga = length(ga);
                let dir_ga = normalize(ga + vec3<f32>(1e-4));
                let s1 = abs(snoise3(dir_ga * 4.8 + vec3<f32>(r_ga * 2.4 - time * 0.25)));
                let s2 = abs(snoise3(dir_ga * 4.8 + vec3<f32>(5.2, -3.1, 2.7)));
                let thread = pow(smoothstep(0.14, 0.0, length(vec2<f32>(s1, s2))), 2.6);
                let knot = exp(-r_ga * 16.0);
                let virgo = p - vec3<f32>(-0.38, 0.22, -0.10);
                let m87_jet = exp(-length(virgo.xz) * 36.0) * exp(-abs(virgo.y) * 5.0) * 1.1;
                dens = (thread * 0.95 * exp(-r_ga * 1.2) + knot * 1.25 + m87_jet) * (1.0 - smoothstep(0.68, 0.92, r));
                neb_col = mix(vec3<f32>(0.22, 0.65, 1.45), vec3<f32>(1.65, 0.98, 0.34), clamp(knot * 1.4 + (0.6 - r_ga) * 0.5, 0.0, 1.0));
            } else if (vid == 8u) {
                // Cosmic Web 1D Voronoi Filaments & Supernodes
                let w_warp = curl_noise3(p * 2.6);
                let f1 = fbm3(p * 3.4 + w_warp * 0.25, 3);
                let f2 = fbm3(p * 3.4 - w_warp * 0.25 + vec3<f32>(7.3, -4.1, 5.9), 3);
                let ridge = pow(smoothstep(0.038, 0.0, length(vec2<f32>(f1, f2))), 2.6);
                let supernode = pow(smoothstep(0.66, 0.95, snoise3(p * 6.5)), 2.8);
                dens = (ridge * 0.95 + supernode * 1.25) * (1.0 - smoothstep(0.62, 0.88, r));
                neb_col = mix(vec3<f32>(0.28, 0.42, 1.25), vec3<f32>(1.15, 0.72, 1.45), clamp(supernode * 1.6, 0.0, 1.0));
            } else if (vid == 9u) {
                // Observable Universe: Particle-Horizon Geodesic Shell + Quasar Era Bipolar Jets + HII Ionization Bubbles
                let shell = exp(-abs(r - 0.76) * 52.0);
                let meridians = exp(-abs(sin(p.y * 12.0)) * 18.0) + exp(-abs(sin(p.x * 10.0) * sin(p.z * 10.0)) * 20.0) * 0.65;
                let quasar = exp(-length(p.xy) * 28.0) * exp(-abs(p.z) * 4.5) * select(0.5, 1.4, z_idx == 4u || z_idx == 6u);
                let deep_knots = pow(smoothstep(0.68, 0.96, snoise3(p * 14.0)), 3.2) * (1.0 - smoothstep(0.58, 0.78, r));
                dens = shell * (0.04 + 0.82 * clamp(meridians, 0.0, 1.4)) + deep_knots + quasar;
                neb_col = mix(vec3<f32>(0.28, 0.80, 1.45), vec3<f32>(1.65, 0.75, 0.35), clamp(quasar + deep_knots * 1.4, 0.0, 1.0));
            } else if (vid == 20u) {
                // Landmark: Heliopause Bow Shock & Magnetotail
                let bow = exp(-abs(length(vec2<f32>(p.y, p.z)) - sqrt(max(0.0, (0.48 - p.x) * 0.55))) * 26.0) * smoothstep(0.50, -0.75, p.x);
                let sun_spark = exp(-length(p - vec3<f32>(0.22, 0.0, 0.0)) * 18.0);
                dens = bow * 1.1 + sun_spark * 1.2;
                neb_col = mix(vec3<f32>(0.25, 0.95, 1.65), vec3<f32>(1.75, 1.15, 0.42), sun_spark);
            } else if (vid == 21u) {
                // Landmark: Orion / Eagle Stellar Nursery Creation Pillars
                let p_tilt = doc_rot_z(p, 0.25);
                let c1 = exp(-length(p_tilt.xz - vec2<f32>(-0.22, 0.0)) * 16.0) * smoothstep(0.55, -0.45, p_tilt.y);
                let c2 = exp(-length(p_tilt.xz - vec2<f32>(0.06, 0.12)) * 19.0) * smoothstep(0.42, -0.45, p_tilt.y);
                let c3 = exp(-length(p_tilt.xz - vec2<f32>(0.28, -0.08)) * 22.0) * smoothstep(0.28, -0.45, p_tilt.y);
                let edge_fbm = 0.4 + 0.6 * abs(fbm3(p * 6.5, 3));
                let stars = pow(max(0.0, snoise3(p * 16.0)), 8.0) * 1.6;
                dens = ((c1 + c2 + c3) * edge_fbm * 0.9 + stars) * (1.0 - smoothstep(0.70, 0.94, r));
                neb_col = mix(vec3<f32>(1.45, 0.42, 0.85), vec3<f32>(0.28, 1.25, 1.65), clamp(p_tilt.y + 0.5, 0.0, 1.0));
            } else if (vid == 22u) {
                // Landmark: Local Bubble Carved Supernova Shock Cavity Shell
                let cav_w = snoise3(p * 3.8) * 0.14;
                let shell = exp(-abs(r - 0.68 - cav_w) * 32.0);
                let rim = exp(-abs(sin(atan2(p.z, p.x) * 6.0)) * 8.0);
                dens = shell * (0.28 + 0.65 * rim);
                neb_col = mix(vec3<f32>(0.18, 1.15, 1.45), vec3<f32>(0.85, 0.55, 1.65), rim);
            } else {
                // 10 Early Universe (switches cleanly by zone: Plasma / Quark-Gluon / Electroweak / Inflation)
                let shell_ring = exp(-abs(r - 0.56) * 48.0);
                let qgp_rot = doc_rot_y(p, time * 0.5 + r * 3.0);
                let fil = pow(smoothstep(0.06, 0.0, abs(snoise3(qgp_rot * 5.8))), 2.4);
                let core = exp(-r * 15.0);
                let sym = select(0.0, exp(-abs(r - 0.36) * 36.0) * 0.85, z_idx >= 5u);
                dens = (shell_ring * (0.16 + 0.85 * fil) + fil * smoothstep(0.75, 0.1, r) * 0.45 + core + sym) * (1.0 - smoothstep(0.68, 0.92, r));
                neb_col = mix(inst.color_b.rgb, vec3<f32>(1.75, 0.88, 0.34), clamp(core * 1.6, 0.0, 1.0));
            }
            let step_a = dens * dt * inst.color_b.a * (0.72 + voice * 0.24);
            accum_rgb += (1.0 - accum_a) * neb_col * min(inst.color_a.a, 1.2) * step_a;
            accum_a += (1.0 - accum_a) * min(step_a * 0.65, 0.38);
        }
    }
    return vec4<f32>(accum_rgb, accum_a);
}
