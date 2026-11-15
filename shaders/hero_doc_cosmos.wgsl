// HyperCosmosVR — Documentary Cosmos Domain Shaders (Apps 03 Deep Earth, 04 Solar System, 05..10 Cosmology)
// Concatenated after common_oled.wgsl + doc_common.wgsl.

fn doc_solar_feat_gate(inst: HeroVolumeInstance, target_feat: u32) -> f32 {
    if (doc_feat(inst) == 255u) {
        return 0.0;
    }
    return smoothstep(0.02, 0.45, doc_feat_focus(inst, target_feat) - 1.0);
}

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
    let raw_vid = inst.kind_tags.y;
    let is_exp03 = (doc_is_canonical_zone_vid(raw_vid) && doc_vid_exp(raw_vid) == 3u)
                || (raw_vid == 0u && doc_submode(inst) > 0u);
    let is_exp04_earth = (raw_vid == 1403u);
    let z_idx = select(
        99u,
        select(doc_zone(inst), doc_vid_zone(raw_vid), doc_is_canonical_zone_vid(raw_vid)),
        is_exp03,
    );
    let w_doc = doc_weight(inst);

    let spin_p = doc_rot_y(p_surf, time * 0.03 + inst.phys_a.x * 0.5);
    let cont = fbm3(spin_p * 2.4 + vec3<f32>(0.4, -0.2, 0.9), 5);
    let mtn_raw = pow(max(0.0, snoise3(spin_p * 11.0)), 2.0) * smoothstep(0.04, 0.25, cont);
    let coast = smoothstep(0.02, 0.08, cont);
    let polar_ice = smoothstep(0.76, 0.92, abs(p_surf.y) + cont * 0.12);

    // Z06 Surface 1:1 feature spotlights (0=ocean, 1=coast, 2=forest, 3=mountain, 4=soil)
    let is_surface_zone = z_idx == 6u;
    let f_ocean = select(1.0, doc_feat_focus(inst, 0u), is_surface_zone);
    let f_coast = select(1.0, doc_feat_focus(inst, 1u), is_surface_zone);
    let f_forest = select(1.0, doc_feat_focus(inst, 2u), is_surface_zone);
    let f_mtn = select(1.0, doc_feat_focus(inst, 3u), is_surface_zone);
    let f_soil = select(1.0, doc_feat_focus(inst, 4u), is_surface_zone);

    let trench_gain = select(0.0, 0.72 * f_ocean, z_idx == 6u || z_idx == 7u);
    let ocean_col = mix(vec3<f32>(0.01, 0.045, 0.16), vec3<f32>(0.02, 0.15, 0.38), smoothstep(-0.32, 0.01, cont)) * f_ocean;
    let ocean_trench = exp(-abs(cont + 0.18) * 28.0) * (1.0 - coast) * trench_gain;
    let shelf_ribbon = exp(-pow((cont - 0.035) * 34.0, 2.0)) * select(0.0, 0.85 * f_coast, is_surface_zone);
    let mtn = mtn_raw * select(1.0, 1.35 * f_mtn, is_surface_zone);

    let veg_mask = smoothstep(0.06, 0.24, cont) * (1.0 - polar_ice) * (1.0 - smoothstep(0.35, 0.65, abs(p_surf.y)));
    let soil_band = smoothstep(0.18, 0.34, cont) * (1.0 - smoothstep(0.36, 0.48, cont)) * select(0.0, 1.0, is_surface_zone);
    var land_base = mix(vec3<f32>(0.045, 0.18, 0.06), vec3<f32>(0.24, 0.18, 0.10), smoothstep(0.06, 0.38, cont));
    land_base = mix(land_base, vec3<f32>(0.04, 0.34, 0.10) * f_forest, veg_mask * select(0.15, 0.75, is_surface_zone));
    land_base = mix(land_base, vec3<f32>(0.42, 0.24, 0.12) * f_soil, soil_band * 0.72);
    let alpine_col = vec3<f32>(0.24, 0.20, 0.16) * mtn + select(vec3<f32>(0.0), vec3<f32>(0.55, 0.58, 0.64) * smoothstep(0.32, 0.65, mtn_raw) * f_mtn, is_surface_zone);

    var albedo = mix(
        ocean_col + vec3<f32>(0.02, 0.38, 0.62) * ocean_trench + vec3<f32>(0.08, 0.68, 0.72) * shelf_ribbon,
        land_base + alpine_col,
        coast
    );
    albedo = mix(albedo, vec3<f32>(0.85, 0.90, 0.96), polar_ice);

    // Z05 Troposphere 1:1 feature choreography (0=cloud, 1=storm cyclone+lightning, 2=wind ribbons, 3=airplanes, 4=water vapor)
    let is_tropo_zone = z_idx == 5u;
    let f_cloud = select(1.0, doc_feat_focus(inst, 0u), is_tropo_zone);
    let f_storm = select(1.0, doc_feat_focus(inst, 1u), is_tropo_zone);
    let f_wind = select(1.0, doc_feat_focus(inst, 2u), is_tropo_zone);
    let f_plane = select(1.0, doc_feat_focus(inst, 3u), is_tropo_zone);
    let f_vapor = select(1.0, doc_feat_focus(inst, 4u), is_tropo_zone);

    let cloud_n = fbm3(spin_p * 4.4 + vec3<f32>(time * 0.016, 0.0, -time * 0.01), 4);
    var clouds = smoothstep(0.14, 0.46, cloud_n) * select(1.0, clamp(f_cloud * 0.95, 0.55, 1.15), is_tropo_zone);
    var storm_bolt = pow(max(0.0, snoise3(spin_p * 19.0 + vec3<f32>(time * 3.5))), 14.0) * clouds * select(0.15, 1.35 * f_storm, is_tropo_zone);
    var tropo_fx = vec3<f32>(0.0);

    if (is_tropo_zone) {
        // Rotating synoptic cyclone vortex centered over sunlit ocean hemisphere
        let cyc_center = normalize(vec3<f32>(0.24, 0.34, 0.90));
        let cyc_vec = spin_p - cyc_center;
        let cyc_r = length(cyc_vec);
        let cyc_ang = atan2(cyc_vec.y, cyc_vec.x) - cyc_r * 16.0 + time * 0.65;
        let cyc_arms = 0.5 + 0.5 * sin(cyc_ang * 3.0);
        let cyc_mask = smoothstep(0.32, 0.06, cyc_r) * smoothstep(0.025, 0.06, cyc_r) * (0.45 + 0.55 * cyc_arms);
        clouds = clamp(clouds + cyc_mask * 0.68 * f_storm, 0.0, 1.0);
        storm_bolt += cyc_mask * pow(max(0.0, snoise3(spin_p * 26.0 + vec3<f32>(time * 5.2))), 10.0) * f_storm * 1.1;

        // Equatorial Hadley & mid-latitude wind streamline ribbons
        let lon = atan2(spin_p.z, spin_p.x);
        let wind_wave = exp(-pow(sin(spin_p.y * 12.0 + sin(lon * 3.0 - time * 0.55) * 0.45) * 4.5, 2.0));
        let wind_flow = 0.55 + 0.45 * sin(lon * 8.0 - time * 1.6);
        tropo_fx += vec3<f32>(0.32, 0.78, 1.15) * wind_wave * wind_flow * 0.28 * f_wind * (0.25 + 0.75 * ndotl);

        // Great-circle transoceanic commercial aviation tracks + pulsing strobe aircraft
        let arc_d = min(abs(dot(spin_p, normalize(vec3<f32>(0.2, 0.85, -0.45)))), abs(dot(spin_p, normalize(vec3<f32>(-0.35, 0.78, 0.52)))));
        let corridor = exp(-pow(arc_d * 52.0, 2.0));
        let strobe = pow(max(0.0, sin(lon * 10.0 - time * 2.2)), 24.0) * corridor;
        tropo_fx += vec3<f32>(0.95, 0.72, 0.28) * (corridor * 0.18 + strobe * 0.95) * f_plane;

        // Column-integrated translucent water vapor plumes along ITCZ
        let vapor_band = exp(-pow(spin_p.y * 2.8 + cloud_n * 1.1, 2.0)) * smoothstep(-0.1, 0.35, cloud_n);
        tropo_fx += vec3<f32>(0.10, 0.72, 0.88) * vapor_band * 0.38 * f_vapor * (0.3 + 0.7 * ndotl);
    }

    // Z00..=Z04 Outer Space -> Stratosphere surface/limb overlays (paired with volumetric shell)
    let is_feat_mode = doc_feat(inst) != 255u;
    let feat_disk = select(0.05, 1.0, is_feat_mode);
    let nat_rim = select(0.42, 1.0, is_feat_mode);
    var atmo_disk_fx = vec3<f32>(0.0);
    if (z_idx == 0u) {
        let f_sw = doc_feat_focus(inst, 0u);
        let f_mag = doc_feat_focus(inst, 1u);
        let f_cr = doc_feat_focus(inst, 2u);
        let f_geo = doc_feat_focus(inst, 3u);
        let f_vac = doc_feat_focus(inst, 4u);
        let bow = smoothstep(0.45, 0.98, dot(n_surf, sun_local)) * fresnel;
        let cusp = exp(-pow((abs(p_surf.y) - 0.80) * 16.0, 2.0));
        let cr_spark = pow(max(0.0, snoise3(spin_p * 18.0 + vec3<f32>(time * 1.4))), 10.0);
        let geo_lat = exp(-pow(p_surf.y * 22.0, 2.0)) * pow(0.5 + 0.5 * cos(atan2(spin_p.z, spin_p.x) * 3.0 - time * 0.4), 6.0);
        atmo_disk_fx += vec3<f32>(1.15, 0.72, 0.25) * bow * 0.65 * f_sw * nat_rim
                      + vec3<f32>(0.24, 0.72, 1.45) * cusp * 0.52 * f_mag * nat_rim
                      + vec3<f32>(0.55, 1.15, 1.48) * cr_spark * 0.75 * f_cr * feat_disk
                      + vec3<f32>(1.35, 0.98, 0.35) * geo_lat * 0.78 * f_geo * feat_disk
                      + vec3<f32>(0.18, 0.35, 0.72) * fresnel * 0.28 * f_vac * nat_rim;
    } else if (z_idx == 1u) {
        let f_h = doc_feat_focus(inst, 0u);
        let f_he = doc_feat_focus(inst, 1u);
        let f_esc = doc_feat_focus(inst, 2u);
        let f_geo = doc_feat_focus(inst, 3u);
        let f_sat = doc_feat_focus(inst, 4u);
        let term = exp(-pow(dot(n_surf, sun_local) * 4.2, 2.0));
        let esc_spark = pow(max(0.0, snoise3(spin_p * 14.0 - normalize(spin_p + vec3<f32>(1e-4)) * time * 0.8)), 8.0) * term;
        let heo_trk = exp(-pow((p_surf.y - 0.82 * sin(atan2(spin_p.z, spin_p.x))) * 16.0, 2.0));
        atmo_disk_fx += vec3<f32>(0.32, 0.78, 1.45) * fresnel * 0.55 * f_h * nat_rim
                      + vec3<f32>(1.25, 0.92, 0.42) * pow(fresnel, 0.85) * 0.42 * f_he * nat_rim
                      + vec3<f32>(0.42, 1.25, 1.48) * esc_spark * 0.85 * f_esc * feat_disk
                      + vec3<f32>(0.88, 0.38, 1.45) * smoothstep(-0.1, 0.7, dot(n_surf, sun_local)) * fresnel * 0.65 * f_geo * nat_rim
                      + vec3<f32>(0.95, 1.15, 1.45) * heo_trk * 0.55 * f_sat * feat_disk;
    } else if (z_idx == 2u) {
        let f_aur = doc_feat_focus(inst, 0u);
        let f_ion = doc_feat_focus(inst, 1u);
        let f_iss = doc_feat_focus(inst, 2u);
        let f_o = doc_feat_focus(inst, 3u);
        let f_rad = doc_feat_focus(inst, 4u);
        let aur_oval = exp(-pow((abs(p_surf.y) - 0.75) * 14.0, 2.0)) * (0.45 + 0.55 * sin(atan2(spin_p.z, spin_p.x) * 8.0 + time * 1.1));
        let ion_day = smoothstep(0.05, 0.85, dot(n_surf, sun_local)) * (0.28 + 0.72 * fresnel);
        let lon_s = atan2(spin_p.z, spin_p.x);
        let iss_path = p_surf.y - 0.62 * sin(lon_s);
        let iss_trk = exp(-pow(iss_path * 24.0, 2.0));
        let iss_dot = exp(-length(spin_p - normalize(vec3<f32>(cos(time * 0.5), 0.62 * sin(time * 0.5), sin(time * 0.5)))) * 18.0);
        let hf_arc = pow(0.5 + 0.5 * sin(spin_p.x * 22.0 + spin_p.y * 14.0 - time * 3.4), 6.0)
                   * smoothstep(0.28, 0.86, fresnel) * smoothstep(-0.1, 0.75, spin_p.y)
                   * select(0.10, 0.58 * smoothstep(0.20, 1.0, f_rad), is_feat_mode);
        atmo_disk_fx += vec3<f32>(0.14, 1.42, 0.55) * aur_oval * 0.85 * f_aur * select(0.52, 1.0, is_feat_mode)
                      + vec3<f32>(0.32, 0.88, 1.48) * ion_day * 0.52 * f_ion * feat_disk
                      + (vec3<f32>(0.95, 0.78, 0.32) * iss_trk * 0.42 + vec3<f32>(1.45, 1.15, 0.55) * iss_dot * 1.1) * f_iss * feat_disk
                      + vec3<f32>(0.12, 1.35, 0.68) * fresnel * 0.72 * f_o * nat_rim
                      + vec3<f32>(0.35, 1.25, 1.48) * hf_arc * feat_disk;
    } else if (z_idx == 3u) {
        let f_met = doc_feat_focus(inst, 0u);
        let f_nlc = doc_feat_focus(inst, 1u);
        let f_mp = doc_feat_focus(inst, 2u);
        let f_rkt = doc_feat_focus(inst, 3u);
        let f_brk = doc_feat_focus(inst, 4u);
        let met_line = exp(-pow((p_surf.y - 0.35 * p_surf.x - 0.22) * 32.0, 2.0)) * smoothstep(-0.3, 0.6, p_surf.z);
        let nlc = exp(-pow((abs(p_surf.y) - 0.68) * 11.0, 2.0)) * smoothstep(0.05, 0.55, snoise3(spin_p * 9.0 + vec3<f32>(time * 0.2)));
        let rkt_flare = exp(-length(spin_p - vec3<f32>(-0.22, 0.42, 0.88)) * 16.0);
        let brk_sparks = pow(max(0.0, snoise3(spin_p * 20.0 - vec3<f32>(time * 1.8, 0.0, 0.0))), 10.0);
        atmo_disk_fx += vec3<f32>(1.48, 0.82, 0.24) * met_line * 0.92 * f_met * feat_disk
                      + vec3<f32>(0.38, 1.12, 1.55) * nlc * 0.82 * f_nlc * nat_rim
                      + vec3<f32>(0.55, 1.35, 1.55) * fresnel * 0.75 * f_mp * nat_rim
                      + vec3<f32>(1.48, 0.62, 0.22) * rkt_flare * 0.95 * f_rkt * feat_disk
                      + vec3<f32>(1.28, 0.92, 0.45) * brk_sparks * 0.72 * f_brk * feat_disk;
    } else if (z_idx == 4u) {
        let f_oz = doc_feat_focus(inst, 0u);
        let f_bal = doc_feat_focus(inst, 1u);
        let f_jet = doc_feat_focus(inst, 2u);
        let f_inv = doc_feat_focus(inst, 3u);
        let f_uv = doc_feat_focus(inst, 4u);
        let bal_spot = exp(-length(spin_p - vec3<f32>(0.18, 0.32, 0.93)) * 20.0);
        let jet_trail = exp(-pow(abs(dot(spin_p, normalize(vec3<f32>(0.15, 0.88, -0.45)))) * 38.0, 2.0));
        let strata = (0.5 + 0.5 * sin(p_surf.y * 42.0)) * (1.0 - polar_ice) * 0.38;
        let uv_day = smoothstep(0.1, 0.85, dot(n_surf, sun_local));
        atmo_disk_fx += vec3<f32>(0.42, 0.26, 1.45) * fresnel * 0.52 * f_oz * nat_rim
                      + vec3<f32>(1.42, 1.42, 1.48) * bal_spot * 0.95 * f_bal * feat_disk
                      + vec3<f32>(1.32, 1.18, 0.92) * jet_trail * 0.72 * f_jet * feat_disk
                      + vec3<f32>(0.52, 0.88, 1.25) * strata * f_inv * feat_disk
                      + vec3<f32>(0.85, 0.35, 1.48) * uv_day * (0.22 + 0.78 * fresnel) * 0.65 * f_uv * feat_disk;
    }

    let half_v = normalize(sun_local + v_dir);
    let ocean_spec = pow(max(dot(n_surf, half_v), 0.0), 64.0) * (1.0 - coast) * (1.0 - clouds) * 1.35 * f_ocean;

    let night_mask = smoothstep(0.08, -0.22, dot(n_surf, sun_local));
    let city_spark = pow(max(0.0, snoise3(spin_p * 24.0)), 6.0) * coast * (1.0 - polar_ice);
    let city_glow = vec3<f32>(1.75, 1.05, 0.34) * city_spark * night_mask * 1.5 * f_coast;

    // Ozone indigo glint on stratosphere zone (4), blue Rayleigh atmospheric limb
    let ozone_glow = vec3<f32>(0.38, 0.22, 1.65) * fresnel * select(0.1, 0.95 * doc_feat_focus(inst, 0u), z_idx == 4u);
    let rayleigh = inst.color_b.rgb * fresnel * (0.35 + 1.35 * smoothstep(-0.2, 0.4, dot(n_surf, sun_local)));

    var exp04_fx = vec3<f32>(0.0);
    if (is_exp04_earth) {
        let f4_atmo = doc_solar_feat_gate(inst, 0u);
        let f4_ocean = doc_solar_feat_gate(inst, 1u);
        let f4_cont = doc_solar_feat_gate(inst, 2u);
        let f4_moon = doc_solar_feat_gate(inst, 3u);
        let f4_mag = doc_solar_feat_gate(inst, 4u);

        let gyre = sin(spin_p.y * 12.0 + cos(atan2(spin_p.z, spin_p.x) * 3.0) * 1.4);
        let gyre_line = exp(-pow(gyre * 4.5, 2.0)) * (1.0 - coast) * (0.3 + 0.7 * ndotl);
        let bulge = pow(max(0.0, 1.0 - abs(spin_p.y) * 1.4), 2.0) * (0.5 + 0.5 * cos(atan2(spin_p.z, spin_p.x) * 2.0)) * (1.0 - coast);
        let cusp_ring = exp(-pow((abs(p_surf.y) - 0.78) * 14.0, 2.0));
        let bow_rim = smoothstep(0.35, 0.96, dot(n_surf, sun_local)) * fresnel;
        exp04_fx = vec3<f32>(0.36, 0.88, 1.65) * fresnel * 1.25 * f4_atmo
                 + vec3<f32>(0.08, 0.72, 1.28) * (gyre_line * 0.65 + (1.0 - coast) * 0.32 * ndotl) * f4_ocean
                 + (vec3<f32>(0.42, 0.88, 0.28) * coast * 0.48 + vec3<f32>(1.05, 0.78, 0.36) * mtn_raw * 0.95) * f4_cont * (0.2 + 0.8 * ndotl)
                 + vec3<f32>(0.24, 1.05, 1.45) * bulge * 0.78 * f4_moon
                 + (vec3<f32>(0.24, 0.88, 1.75) * cusp_ring * 0.92 + vec3<f32>(0.32, 0.72, 1.55) * bow_rim * 1.05) * f4_mag;
    }

    let day_lit = mix(albedo, vec3<f32>(0.90), clouds) * (ndotl + 0.025) + vec3<f32>(ocean_spec);
    let zone_rim = mix(1.0, 1.25, w_doc * fresnel);
    return (day_lit + tropo_fx + atmo_disk_fx + exp04_fx + city_glow + rayleigh + ozone_glow + vec3<f32>(0.55, 0.85, 1.95) * storm_bolt) * inst.color_a.a * zone_rim;
}

fn shade_doc_earth_cutaway(
    p_wall: vec3<f32>,
    surf_r: f32,
    inst: HeroVolumeInstance,
    time: f32,
) -> vec3<f32> {
    let wall_r = length(p_wall) / max(surf_r, 1e-4);
    let raw_vid = inst.kind_tags.y;
    let z_idx = select(doc_zone(inst), doc_vid_zone(raw_vid), doc_is_canonical_zone_vid(raw_vid));
    let conv = fbm3(p_wall * 7.5 + vec3<f32>(0.0, time * 0.08, 0.0), 4);
    let phi = atan2(p_wall.z, p_wall.x);
    let plume = pow(max(0.0, sin(phi * 4.0 + conv * 2.4)), 4.0);
    let subduct = exp(-abs(wall_r - 0.92 + max(p_wall.y, 0.0) * 0.30) * 34.0) * smoothstep(0.1, 0.5, p_wall.z / max(surf_r, 1e-4));

    // Decoupled Moho discontinuity: thin oceanic crust (0.962..1.0) vs thicker continental crust (0.938..1.0),
    // with rigid lithospheric upper mantle cleanly separated below Moho (0.885..moho_r).
    let cont_region = smoothstep(0.05, 0.35, sin(phi * 3.0 + p_wall.y * 3.6 + conv * 0.6));
    let moho_r = mix(0.962, 0.938, cont_region);
    let litho_base_r = 0.885;

    let is_crust_zone = z_idx == 7u;
    let is_litho_zone = z_idx == 8u;
    let c_ocean = select(1.0, doc_feat_focus(inst, 0u), is_crust_zone);
    let c_cont = select(1.0, doc_feat_focus(inst, 1u), is_crust_zone);
    let c_plate = select(1.0, doc_feat_focus(inst, 2u), is_crust_zone);
    let c_fault = select(1.0, doc_feat_focus(inst, 3u), is_crust_zone);
    let c_magma = select(1.0, doc_feat_focus(inst, 4u), is_crust_zone);

    let l_plate = select(1.0, doc_feat_focus(inst, 0u), is_litho_zone);
    let l_subduct = select(1.0, doc_feat_focus(inst, 1u), is_litho_zone);
    let l_root = select(1.0, doc_feat_focus(inst, 2u), is_litho_zone);
    let l_quake = select(1.0, doc_feat_focus(inst, 3u), is_litho_zone);
    let l_mantle = select(1.0, doc_feat_focus(inst, 4u), is_litho_zone);

    var col = vec3<f32>(0.0);
    if (wall_r > moho_r) {
        // Crust (Z07): basaltic oceanic crust vs buoyant granitic continental crust
        let ocean_col = vec3<f32>(0.024, 0.055, 0.080) * (0.75 + 0.4 * conv) * c_ocean;
        let cont_col = vec3<f32>(0.085, 0.048, 0.026) * (0.75 + 0.45 * conv) * c_cont;
        let base_crust = mix(ocean_col, cont_col, cont_region) * select(0.72, 1.38, is_crust_zone) * select(1.0, l_plate, is_litho_zone);
        // Plate sutures & brittle fault + seismic pulse rings + rising magma conduits
        let plate_suture = exp(-pow(sin(phi * 6.0 + p_wall.y * 5.0) * 18.0, 2.0)) * 0.42 * c_plate * l_plate;
        let fault_d = abs(wall_r - 0.968 + (phi - 0.78) * 0.08);
        let fault_glow = exp(-pow(fault_d * 95.0, 2.0)) * 0.65 * select(0.08, c_fault, is_crust_zone);
        let seismic_ring = (0.5 + 0.5 * sin(length(p_wall - vec3<f32>(0.55, 0.45, 0.65) * surf_r) * 32.0 - time * 4.5))
                         * fault_glow * select(0.0, 0.85, is_crust_zone);
        let magma_ch = exp(-pow((phi - 1.05) * 9.0, 2.0)) * smoothstep(moho_r - 0.02, 0.995, wall_r);
        let magma_col = vec3<f32>(1.15, 0.34, 0.05) * magma_ch * select(0.10, c_magma, is_crust_zone);
        col = base_crust + vec3<f32>(0.65, 0.34, 0.14) * (plate_suture + fault_glow + seismic_ring) + magma_col;
    } else if (wall_r > litho_base_r) {
        // Rigid Upper Mantle (Z08 Lithosphere below Moho discontinuity)
        let root_keel = smoothstep(0.2, 0.8, cont_region) * smoothstep(litho_base_r, moho_r, wall_r) * l_root;
        let mantle_rigid = mix(vec3<f32>(0.052, 0.036, 0.012), vec3<f32>(0.095, 0.052, 0.016), clamp(0.5 + conv * 0.5, 0.0, 1.0));
        let hi_l = select(0.75, 1.35 * l_mantle * l_plate, is_litho_zone);
        // Wadati-Benioff deep-focus rupture foci along subducting slab
        let slab_foci = pow(max(0.0, sin(wall_r * 110.0 - phi * 18.0)), 14.0) * subduct * select(0.05, l_quake, is_litho_zone);
        let subduct_col = mix(vec3<f32>(0.18, 0.52, 0.98), vec3<f32>(0.98, 0.34, 0.08), smoothstep(0.95, 0.89, wall_r))
                        * subduct * select(0.28, 1.25 * l_subduct, is_litho_zone);
        let magma_feeder = exp(-pow((phi - 1.05) * 11.0, 2.0)) * vec3<f32>(0.95, 0.26, 0.05) * select(0.08, c_magma, is_crust_zone);
        col = mantle_rigid * hi_l + vec3<f32>(0.16, 0.10, 0.04) * root_keel * 0.45 + subduct_col + vec3<f32>(1.15, 0.85, 0.25) * slab_foci + magma_feeder;
    } else if (wall_r > 0.78) {
        // Asthenosphere (Z09): ductile shear + closed convection rolls + partial melt filigree + plume + seismic LVZ
        let is_astheno = z_idx == 9u;
        let is_ast_feat = is_astheno && (doc_feat(inst) != 255u);
        let a_duct = select(1.0, doc_feat_focus(inst, 0u), is_astheno);
        let a_conv = select(1.0, doc_feat_focus(inst, 1u), is_astheno);
        let a_melt = select(1.0, doc_feat_focus(inst, 2u), is_astheno);
        let a_plume = select(1.0, doc_feat_focus(inst, 3u), is_astheno);
        let a_seis = select(1.0, doc_feat_focus(inst, 4u), is_astheno);
        let hi = select(0.62, select(0.78, 1.18, is_ast_feat), is_astheno);
        let shear_grid = exp(-pow(sin(phi * 10.0 + p_wall.y * 8.0) * 10.0, 2.0)) * 0.45 * select(0.04, a_duct, is_astheno);
        let roll_cells = (0.5 + 0.5 * sin(phi * 6.0) * cos((wall_r - 0.78) * 34.0)) * select(0.06, 0.58 * a_conv, is_astheno);
        let melt_pockets = pow(max(0.0, snoise3(p_wall * 19.0 + vec3<f32>(0.0, time * 0.12, 0.0))), 6.0) * select(0.04, 1.55 * a_melt, is_astheno);
        let ast_plume = exp(-pow((phi - 0.65) * 8.0, 2.0)) * (0.5 + 0.5 * plume) * select(0.05, 0.92 * a_plume, is_astheno);
        let lvz_wave = pow(0.5 + 0.5 * sin(wall_r * 92.0 - phi * 8.0 - time * 3.8), 10.0) * select(0.02, 0.85 * a_seis, is_astheno);
        col = mix(vec3<f32>(0.095, 0.008, 0.0015), vec3<f32>(0.26, 0.034, 0.004), clamp(0.42 + conv * 0.55 + roll_cells * 0.45, 0.0, 1.0)) * hi * a_duct * a_conv
            + vec3<f32>(0.72, 0.24, 0.06) * subduct * select(0.16, l_subduct, is_litho_zone)
            + vec3<f32>(1.15, 0.52, 0.14) * shear_grid
            + vec3<f32>(1.55, 1.12, 0.28) * melt_pockets
            + vec3<f32>(1.35, 0.52, 0.08) * ast_plume
            + vec3<f32>(0.24, 0.95, 1.38) * lvz_wave;
    } else if (wall_r > 0.68) {
        // Mantle Transition Zone (Z10): olivine->ringwoodite compaction + hydrous pockets + 410/660 discontinuities
        let is_trans = z_idx == 10u;
        let is_tr_feat = is_trans && (doc_feat(inst) != 255u);
        let t_oliv = select(1.0, doc_feat_focus(inst, 0u), is_trans);
        let t_press = select(1.0, doc_feat_focus(inst, 1u), is_trans);
        let t_water = select(1.0, doc_feat_focus(inst, 2u), is_trans);
        let t_disc = select(1.0, doc_feat_focus(inst, 3u), is_trans);
        let t_660 = select(1.0, doc_feat_focus(inst, 4u), is_trans);
        let hi = select(0.60, select(0.78, 1.20, is_tr_feat), is_trans);
        let lattice = pow(abs(sin(p_wall.y * 36.0) * sin(phi * 18.0)), 4.0) * select(0.025, 0.65 * t_oliv, is_trans);
        let depth_t = clamp((0.78 - wall_r) / 0.10, 0.0, 1.0);
        let step_band = (0.38 + 0.62 * floor(depth_t * 4.0) / 4.0) * select(1.0, t_press, is_trans);
        let hydro = pow(max(0.0, snoise3(p_wall * 15.0 + vec3<f32>(1.7))), 5.0) * select(0.03, 1.48 * t_water, is_trans);
        let bounce_ray = pow(0.5 + 0.5 * sin((wall_r - 0.68) * 78.0 + sin(phi * 6.0) * 2.5 - time * 3.4), 10.0) * select(0.025, 0.90 * t_disc, is_trans);
        col = mix(vec3<f32>(0.13, 0.012, 0.002), vec3<f32>(0.34, 0.048, 0.006), clamp(plume * 0.68 + step_band * 0.32, 0.0, 1.0)) * hi * step_band
            + vec3<f32>(1.18, 0.72, 0.22) * lattice
            + vec3<f32>(0.18, 1.05, 1.42) * hydro
            + vec3<f32>(1.22, 1.02, 0.28) * bounce_ray
            + vec3<f32>(1.35, 0.38, 0.10) * exp(-pow((wall_r - 0.695) * 48.0, 2.0)) * select(0.04, t_660, is_trans);
    } else if (wall_r > 0.46) {
        // Lower Mantle (Z11): bridgmanite lattice + deep convection + superplumes + solid creep + D'' LLSVPs
        let is_lm = z_idx == 11u;
        let is_lm_feat = is_lm && (doc_feat(inst) != 255u);
        let m_bridg = select(1.0, doc_feat_focus(inst, 0u), is_lm);
        let m_conv = select(1.0, doc_feat_focus(inst, 1u), is_lm);
        let m_plume = select(1.0, doc_feat_focus(inst, 2u), is_lm);
        let m_dense = select(1.0, doc_feat_focus(inst, 3u), is_lm);
        let m_dpp = select(1.0, doc_feat_focus(inst, 4u), is_lm);
        let hi = select(0.60, select(0.78, 1.18, is_lm_feat), is_lm);
        let perov = pow(abs(sin(p_wall.x * 24.0) * sin(p_wall.y * 24.0)), 6.0) * select(0.03, 0.56 * m_bridg, is_lm);
        let cold_slab = smoothstep(0.35, 0.95, sin(phi * 3.0 + 1.2)) * select(0.04, 0.58 * m_conv, is_lm);
        let super_stem = pow(max(0.0, sin(phi * 2.0 + conv * 1.4)), 10.0) * select(0.05, 1.22 * m_plume, is_lm);
        let isobar = exp(-pow(sin(wall_r * 84.0) * 6.0, 2.0)) * select(0.03, 0.46 * m_dense, is_lm);
        let dpp_dome = smoothstep(0.52, 0.462, wall_r) * (0.45 + 0.55 * sin(phi * 2.0)) * select(0.05, 1.38 * m_dpp, is_lm);
        col = mix(vec3<f32>(0.075, 0.006, 0.001), vec3<f32>(0.30, 0.038, 0.004), clamp(0.36 + plume * 0.68 * m_plume + conv * 0.38 * m_conv, 0.0, 1.0)) * hi
            + vec3<f32>(1.12, 0.78, 0.28) * perov
            + vec3<f32>(0.18, 0.52, 0.98) * cold_slab
            + vec3<f32>(1.35, 0.64, 0.14) * super_stem
            + vec3<f32>(0.98, 0.54, 0.24) * isobar
            + vec3<f32>(1.28, 0.26, 0.48) * dpp_dome;
    } else {
        // Liquid Fe-Ni Outer Core (Z12): molten swirls + buoyant light elements + Taylor columns + dynamo flux + CMB coupling
        let is_oc = z_idx == 12u;
        let is_oc_feat = is_oc && (doc_feat(inst) != 255u);
        let o_melt = select(1.0, doc_feat_focus(inst, 0u), is_oc);
        let o_comp = select(1.0, doc_feat_focus(inst, 1u), is_oc);
        let o_taylor = select(1.0, doc_feat_focus(inst, 2u), is_oc);
        let o_flux = select(1.0, doc_feat_focus(inst, 3u), is_oc);
        let o_couple = select(1.0, doc_feat_focus(inst, 4u), is_oc);
        let hi = select(0.64, select(0.84, 1.22, is_oc_feat), is_oc);
        let helix = 0.5 + 0.5 * sin(p_wall.y * 28.0 + phi * 3.0 - time * 1.4);
        let taylor_col = exp(-pow(sin(length(p_wall.xz) / max(surf_r, 1e-4) * 38.0 - phi * 2.0) * 6.0, 2.0)) * select(0.03, 0.72 * o_taylor, is_oc);
        let light_rise = pow(max(0.0, snoise3(p_wall * 22.0 - normalize(p_wall + vec3<f32>(1e-4)) * time * 0.9)), 6.0) * select(0.03, 1.28 * o_comp, is_oc);
        let flux_loop = pow(max(0.0, cos(atan2(p_wall.y, length(p_wall.xz)) * 6.0 - time * 1.8)), 8.0) * select(0.03, 0.80 * o_flux, is_oc);
        let cmb_link = smoothstep(0.36, 0.455, wall_r) * pow(max(0.0, cos(phi * 6.0)), 6.0) * select(0.04, 1.15 * o_couple, is_oc);
        col = blackbody_rgb(mix(1850.0, 2750.0, clamp(0.35 + helix * 0.45 * o_melt + conv * 0.25, 0.0, 1.0))) * 0.36 * hi * o_melt
            + vec3<f32>(1.18, 0.98, 0.42) * light_rise
            + vec3<f32>(0.38, 0.98, 1.32) * taylor_col
            + vec3<f32>(1.18, 0.64, 0.16) * flux_loop
            + vec3<f32>(0.28, 0.92, 1.38) * cmb_link;
    }

    let seam_410 = exp(-pow((wall_r - 0.78) * 72.0, 2.0)) * select(0.24, 0.78 * doc_feat_focus(inst, 3u), z_idx == 10u);
    let seam_660 = exp(-pow((wall_r - 0.68) * 64.0, 2.0)) * select(0.28, 0.84 * doc_feat_focus(inst, 4u), z_idx == 10u);
    let seam_cmb = exp(-pow((wall_r - 0.46) * 68.0, 2.0)) * select(0.38, 0.90 * doc_feat_focus(inst, 4u), z_idx == 11u || z_idx == 12u);
    let seam_icb = exp(-pow((wall_r - 0.265) * 76.0, 2.0)) * select(0.26, 0.85 * doc_feat_focus(inst, 3u), z_idx == 13u);
    let seam_moho = exp(-pow((wall_r - moho_r) * 86.0, 2.0)) * select(0.22, 0.56, is_crust_zone || is_litho_zone);
    let seam_litho = exp(-pow((wall_r - litho_base_r) * 72.0, 2.0)) * select(0.18, 0.48, is_litho_zone);
    col += vec3<f32>(0.78, 0.24, 0.025) * (seam_410 + seam_660 + seam_cmb + seam_icb + seam_moho + seam_litho);
    return col;
}

fn shade_doc_earth_inner_core(
    p_core: vec3<f32>,
    n_core: vec3<f32>,
    v_dir: vec3<f32>,
    inst: HeroVolumeInstance,
    time: f32,
) -> vec3<f32> {
    let raw_vid = inst.kind_tags.y;
    let is_exp03 = (doc_is_canonical_zone_vid(raw_vid) && doc_vid_exp(raw_vid) == 3u)
                || (raw_vid == 0u && doc_submode(inst) > 0u);
    let z_idx = select(
        99u,
        select(doc_zone(inst), doc_vid_zone(raw_vid), doc_is_canonical_zone_vid(raw_vid)),
        is_exp03,
    );
    let is_ic = z_idx == 13u;
    let is_ic_feat = is_ic && (doc_feat(inst) != 255u);
    let f_solid = select(1.0, smoothstep(0.20, 1.0, doc_feat_focus(inst, 0u)), is_ic);
    let f_aniso = select(1.0, smoothstep(0.20, 1.0, doc_feat_focus(inst, 1u)), is_ic);
    let f_press = select(1.0, smoothstep(0.20, 1.0, doc_feat_focus(inst, 2u)), is_ic);
    let f_grow = select(1.0, smoothstep(0.20, 1.0, doc_feat_focus(inst, 3u)), is_ic);
    let f_center = select(1.0, smoothstep(0.20, 1.0, doc_feat_focus(inst, 4u)), is_ic);

    let view_dot = max(dot(n_core, v_dir), 0.0);
    let sun_local = normalize(inst.phys_b.xyz + vec3<f32>(0.25, 0.35, 0.55));
    let core_shade = 0.14 + 0.86 * pow(max(dot(n_core, sun_local), 0.0), 1.25);
    let fresnel = pow(1.0 - view_dot, 2.8);
    let cryst = fbm3(p_core * 14.0 + vec3<f32>(time * 0.14), 3);
    // Polar HCP iron crystal grain anisotropy (faster P-wave propagation along N-S y-axis)
    let grain_ns = 0.5 + 0.5 * sin(p_core.x * 58.0 + p_core.z * 58.0 + snoise3(p_core * 26.0) * 1.4);
    let polar_bias = smoothstep(0.35, 0.92, abs(n_core.y));
    let aniso_fx = (grain_ns * 0.55 + polar_bias * 0.55) * select(0.06, select(0.08, 0.08 + 0.60 * f_aniso, is_ic_feat), is_ic);
    // 360 GPa hydrostatic pressure contours (explicitly zero on F0/macro so 02_earth_deep_cutaway renders a clean Fe-Ni core)
    let isobar_360 = exp(-pow(sin(n_core.y * 18.0) * 6.0, 2.0)) * select(0.0, 0.62 * f_press, is_ic_feat);
    // Dendritic crystallization freezing front on outer edge
    let dendrite = pow(max(0.0, snoise3(p_core * 34.0 + vec3<f32>(0.0, time * 0.25, 0.0))), 5.0) * fresnel * select(0.08, select(0.10, 0.10 + 1.10 * f_grow, is_ic_feat), is_ic);
    // Earth's exact center 6,371 km reticle crosshairs (explicitly zero on F0/macro)
    let center_cross = exp(-min(abs(p_core.x), abs(p_core.y)) * 88.0) * exp(-abs(p_core.z - 0.18) * 12.0) * select(0.0, 1.15 * f_center, is_ic_feat);

    let base_temp = 2050.0 + cryst * 520.0 + select(0.0, 180.0 * (f_solid - 1.0), is_ic);
    let inner_hi = select(1.0, select(1.04, 0.92 + 0.20 * f_solid, is_ic_feat), is_ic);
    let base_col = blackbody_rgb(base_temp) * (0.34 + u_audio.levels_a.z * 0.12) * core_shade * inner_hi;
    return base_col
         + vec3<f32>(1.10, 0.72, 0.20) * aniso_fx * (0.45 + 0.55 * core_shade)
         + vec3<f32>(0.36, 0.88, 1.35) * isobar_360
         + vec3<f32>(1.38, 1.05, 0.42) * dendrite
         + vec3<f32>(0.35, 1.35, 1.55) * center_cross;
}

fn shade_doc_europa_ice(
    p_surf: vec3<f32>,
    ndotl: f32,
    fresnel: f32,
    alpha_gain: f32,
    f_cutaway: f32,
) -> vec3<f32> {
    let crack_n = abs(fbm3(p_surf * 7.5 + vec3<f32>(1.9, -0.7, 2.3), 4));
    let fissure = exp(-crack_n * 22.0);
    var col = mix(vec3<f32>(0.68, 0.78, 0.88), vec3<f32>(0.24, 0.16, 0.12), fissure * 0.5) * (ndotl * 1.08 + 0.025)
            + vec3<f32>(0.15, 1.25, 1.75) * fissure * 0.95
            + vec3<f32>(0.25, 0.75, 1.25) * fresnel * 0.25;
    if (f_cutaway > 0.01 && p_surf.x > 0.05 && abs(p_surf.y) < 0.42 && p_surf.z > 0.0) {
        let wall = exp(-min(abs(p_surf.x - 0.05), 0.42 - abs(p_surf.y)) * 34.0);
        let ocean_swirl = 0.55 + 0.45 * snoise3(p_surf * 11.0);
        let deep_ocean = mix(vec3<f32>(0.02, 0.28, 0.72), vec3<f32>(0.12, 0.92, 1.45), ocean_swirl);
        col = mix(col, deep_ocean + vec3<f32>(0.78, 0.96, 1.15) * wall * 0.65, f_cutaway * 0.94);
    }
    return col * alpha_gain;
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
    // 04 Solar System Planetary Surfaces & Dwarf/Moon Companions (1400..=1411, 3643..=3651)

    if (vid == 1u || vid == 1400u) {
        // Z00 Sun (1 / 1400): p-p H->He fusion core cutaway (F0), granulation (F1), bipolar sunspots (F2), corona rim (F3), solar wind streams (F4)
        let f_core = doc_solar_feat_gate(inst, 0u);
        let f_gran = doc_solar_feat_gate(inst, 1u);
        let f_spot = doc_solar_feat_gate(inst, 2u);
        let f_cor = doc_solar_feat_gate(inst, 3u);
        let f_wind = doc_solar_feat_gate(inst, 4u);

        let gran = fbm3(p_surf * 13.5 + vec3<f32>(time * 0.14), 4);
        let cells = abs(snoise3(p_surf * 26.0 + vec3<f32>(0.0, time * 0.25, 0.0)));
        let gran_lanes = smoothstep(0.04, mix(0.26, 0.14, f_gran), cells);
        let spot_noise = smoothstep(-0.30, -0.48, fbm3(p_surf * 3.8 + vec3<f32>(0.0, time * 0.03, 0.0), 3));
        let pair_a = exp(-length(p_surf.xy - vec2<f32>(-0.18, 0.12)) * 22.0) * select(0.0, 1.0, p_surf.z > 0.0);
        let pair_b = exp(-length(p_surf.xy - vec2<f32>(0.20, -0.08)) * 24.0) * select(0.0, 1.0, p_surf.z > 0.0);
        let spot = clamp(spot_noise + (pair_a + pair_b) * f_spot, 0.0, 1.0);
        let temp = mix(5200.0, 2850.0, spot) + gran * 980.0 - (1.0 - gran_lanes) * 650.0;
        let limb = pow(view_dot, 0.62);
        let chromo_rim = vec3<f32>(0.98, 0.20, 0.04) * fresnel * (0.68 + 0.92 * f_cor);
        let cell_contrast = (0.34 + 0.66 * gran_lanes) * mix(1.0, 0.22, spot) * mix(1.0, 0.72, f_cor);
        var col = blackbody_rgb(temp) * (0.36 + 0.56 * limb + u_audio.levels_a.z * 0.12) * cell_contrast + chromo_rim;
        let faculae = exp(-abs(length(p_surf.xy - vec2<f32>(0.01, 0.02)) - 0.21) * 32.0) * select(0.0, 1.0, p_surf.z > 0.0);
        col += vec3<f32>(1.45, 1.15, 0.55) * faculae * 0.65 * f_spot;

        // Equatorial octant H->He core cutaway on F0
        if (f_core > 0.01 && p_surf.x > 0.0 && p_surf.y > -0.04 && p_surf.z > 0.0) {
            let cut_d = min(p_surf.x / max(abs(v_dir.x), 0.18), (p_surf.y + 0.04) / max(abs(v_dir.y), 0.18));
            let r_in = clamp(length(p_surf - v_dir * min(cut_d, 0.72)), 0.0, 1.0);
            let core_glow = blackbody_rgb(12500.0) * 1.15 * smoothstep(0.42, 0.12, r_in);
            let rad_zone = vec3<f32>(1.35, 0.86, 0.22) * smoothstep(0.75, 0.34, r_in) * smoothstep(0.20, 0.42, r_in);
            let seam = exp(-min(abs(p_surf.x), abs(p_surf.y + 0.04)) * 38.0);
            col = mix(col, core_glow + rad_zone + vec3<f32>(1.45, 0.95, 0.40) * seam * 0.55, f_core * 0.95);
        }

        // Parker coronal-hole wind plumes on F4
        if (f_wind > 0.01) {
            let hole = smoothstep(0.42, 0.88, abs(p_surf.y));
            let wind_arc = pow(0.5 + 0.5 * sin(atan2(p_surf.z, p_surf.x) * 8.0 - time * 2.2), 4.0) * (0.28 + 0.72 * hole);
            col += vec3<f32>(0.42, 1.05, 1.55) * wind_arc * (0.35 + 0.65 * fresnel) * f_wind * 0.90;
        }
        return col * min(inst.color_a.a, 0.92);
    }

    if (vid == 1405u || (vid == 3u && doc_submode(inst) == 5u)) {
        // Z05 Asteroid Belt — Ceres with cratered dark carbonaceous basalt & Occator carbonate salt domes
        let f_occator = doc_solar_feat_gate(inst, 0u);
        let f_vesta = doc_solar_feat_gate(inst, 1u);
        let f_metal = doc_solar_feat_gate(inst, 2u);
        let f_family = doc_solar_feat_gate(inst, 3u);
        let f_kirk = doc_solar_feat_gate(inst, 4u);

        let reg = fbm3(p_surf * 6.0 + vec3<f32>(2.4), 4);
        let cr = exp(-abs(abs(snoise3(p_surf * 10.5)) - 0.22) * 26.0);
        var col = mix(vec3<f32>(0.24, 0.22, 0.20), vec3<f32>(0.48, 0.46, 0.42), clamp(0.5 + reg * 0.45 + cr * 0.35, 0.0, 1.0));
        let occ_a = exp(-length(p_surf.xy - vec2<f32>(0.12, 0.16)) * 22.0) * select(0.0, 1.0, p_surf.z > 0.0);
        let occ_b = exp(-length(p_surf.xy - vec2<f32>(-0.20, -0.06)) * 28.0) * select(0.0, 1.0, p_surf.z > 0.0);
        let salt = clamp(occ_a + occ_b, 0.0, 1.0) * (0.25 + 1.15 * f_occator);
        col = mix(col, vec3<f32>(0.96, 1.02, 1.08), salt);
        let fe_ribbon = exp(-abs(p_surf.y + 0.12 * sin(p_surf.x * 5.0)) * 22.0) * f_metal;
        let kirk_node = exp(-abs(p_surf.y) * 28.0) * pow(0.5 + 0.5 * sin(atan2(p_surf.z, p_surf.x) * 3.0), 4.0) * f_kirk;
        let fx = vec3<f32>(0.78, 0.62, 0.36) * fresnel * f_vesta * 0.65
               + vec3<f32>(0.92, 0.96, 1.05) * fe_ribbon * 0.85
               + vec3<f32>(1.15, 0.84, 0.42) * fresnel * f_family * 0.72
               + vec3<f32>(0.35, 0.92, 1.45) * kirk_node * 0.95;
        return (col * (ndotl * 1.15 + 0.025) + fx) * inst.color_a.a;
    }

    if (vid == 3u || vid == 1401u) {
        // Z01 Mercury: Caloris basin (F0), lobate thrust scarps (F1), +430/-180 C terminator (F2), polar water ice (F3), 88d orbit arc (F4)
        let f_crat = doc_solar_feat_gate(inst, 0u);
        let f_scarp = doc_solar_feat_gate(inst, 1u);
        let f_therm = doc_solar_feat_gate(inst, 2u);
        let f_ice = doc_solar_feat_gate(inst, 3u);
        let f_orb = doc_solar_feat_gate(inst, 4u);

        let crater_a = abs(snoise3(p_surf * 9.5 + vec3<f32>(1.2)));
        let crater_rim = exp(-abs(crater_a - 0.22) * 28.0);
        let caloris = exp(-abs(length(p_surf.xy - vec2<f32>(0.15, 0.10)) - 0.22) * 24.0) * select(0.0, 1.0, p_surf.z > 0.0);
        let regolith = fbm3(p_surf * 4.5, 4);
        var col = mix(vec3<f32>(0.24, 0.22, 0.20), vec3<f32>(0.55, 0.51, 0.46), clamp(0.5 + regolith * 0.5 + crater_rim * 0.45, 0.0, 1.0));
        col += vec3<f32>(0.88, 0.82, 0.72) * caloris * (0.20 + 0.95 * f_crat);

        let scarp_curve = abs(p_surf.y - 0.16 * sin(p_surf.x * 5.2 + 0.6)) + abs(p_surf.x - 0.05) * 0.15;
        let scarp = exp(-scarp_curve * 32.0) * smoothstep(0.0, 0.35, p_surf.z) * smoothstep(0.45, 0.10, abs(p_surf.x));
        let therm_fx = mix(vec3<f32>(0.08, 0.24, 0.85), vec3<f32>(1.45, 0.58, 0.12), smoothstep(0.0, 0.65, ndotl)) * f_therm * 0.72;
        let polar_ice = smoothstep(0.84, 0.93, abs(p_surf.y) + snoise3(p_surf * 14.0) * 0.04) * (1.0 - smoothstep(0.2, 0.6, ndotl));
        let orb_arc = exp(-abs(p_surf.y) * 34.0) * pow(0.5 + 0.5 * sin(atan2(p_surf.z, p_surf.x) * 4.0 - time * 2.4), 8.0) * f_orb;
        return (col * (ndotl * 1.15 + 0.02)
             + vec3<f32>(1.12, 0.86, 0.52) * scarp * f_scarp * 1.15
             + therm_fx
             + vec3<f32>(0.45, 1.25, 1.65) * polar_ice * (0.15 + 1.35 * f_ice)
             + vec3<f32>(0.42, 1.15, 1.55) * orb_arc) * inst.color_a.a;
    }

    if (vid == 4u || vid == 1402u) {
        // Z02 Venus: Retrograde (-time) super-rotating sulfuric chevron clouds (F0), greenhouse IR trap (F1), retrograde arrows (F2), Magellan radar volcano slice (F3), 90-bar pressure shells (F4)
        let f_cloud = doc_solar_feat_gate(inst, 0u);
        let f_gh = doc_solar_feat_gate(inst, 1u);
        let f_retro = doc_solar_feat_gate(inst, 2u);
        let f_volc = doc_solar_feat_gate(inst, 3u);
        let f_press = doc_solar_feat_gate(inst, 4u);

        let p_ven = doc_rot_y(p_surf, -time * 0.08);
        let warp = fbm3(p_ven * 3.2, 4);
        let chevron = sin(p_ven.y * 8.0 + abs(p_ven.x) * 3.5 + warp * 2.8);
        var col = mix(vec3<f32>(0.56, 0.40, 0.20), vec3<f32>(0.96, 0.80, 0.48), (0.5 + 0.5 * chevron) * (1.0 + 0.25 * f_cloud));
        let limb = vec3<f32>(1.1, 0.85, 0.42) * fresnel * (0.3 + 0.7 * ndotl);

        if (f_volc > 0.01 && p_surf.z > 0.0 && p_surf.x > -0.18 && p_surf.x < 0.36 && abs(p_surf.y) < 0.34) {
            let maxwell = pow(max(0.0, fbm3(p_surf * 9.5, 4)), 1.6);
            let caldera = exp(-length(p_surf.xy - vec2<f32>(0.08, 0.04)) * 22.0);
            let radar = mix(vec3<f32>(0.24, 0.08, 0.03), vec3<f32>(1.45, 0.62, 0.14), clamp(maxwell + caldera, 0.0, 1.0));
            col = mix(col, radar, f_volc * 0.92);
        }
        let ir_trap = exp(-pow((abs(p_surf.y) - 0.16) * 8.0, 2.0)) * (0.45 + 0.55 * sin(p_surf.x * 14.0 - time * 1.8));
        let retro_band = exp(-abs(p_surf.y) * 24.0) * pow(0.5 + 0.5 * sin(atan2(p_surf.z, p_surf.x) * 6.0 + time * 2.6), 8.0);
        let press_rings = exp(-pow(sin(p_surf.y * 16.0) * 6.0, 2.0)) * (0.4 + 0.6 * fresnel);
        return (col * (ndotl * 1.05 + 0.02) + limb
             + vec3<f32>(1.55, 0.38, 0.08) * ir_trap * f_gh * 0.95
             + vec3<f32>(0.36, 1.18, 1.55) * retro_band * f_retro * 1.05
             + vec3<f32>(1.48, 0.82, 0.22) * press_rings * f_press * 0.90) * inst.color_a.a;
    }

    if (vid == 5u || vid == 1404u) {
        // Z04 Mars: polar ice caps (F0), Olympus Mons caldera (F1), Valles Marineris rift (F2), Rover target reticle (F3), Phobos/Deimos tracks (F4)
        let f_ice = doc_solar_feat_gate(inst, 0u);
        let f_olymp = doc_solar_feat_gate(inst, 1u);
        let f_canyon = doc_solar_feat_gate(inst, 2u);
        let f_rover = doc_solar_feat_gate(inst, 3u);
        let f_moons = doc_solar_feat_gate(inst, 4u);

        let ice = smoothstep(mix(0.84, 0.78, f_ice), 0.94, abs(p_surf.y) + snoise3(p_surf * 8.0) * 0.06);
        let canyon = exp(-abs(p_surf.y - 0.06) * mix(22.0, 15.0, f_canyon)) * smoothstep(0.45, 0.15, abs(p_surf.x)) * smoothstep(0.0, 0.4, p_surf.z);
        let olymp_d = length(p_surf.xy - vec2<f32>(-0.24, 0.22)) + select(1.0, 0.0, p_surf.z > 0.0);
        let olymp_shield = exp(-olymp_d * 18.0);
        let olymp_caldera = exp(-abs(olymp_d - 0.045) * 65.0);
        let terrain = fbm3(p_surf * 5.5, 4);
        var rust = mix(vec3<f32>(0.42, 0.14, 0.06), vec3<f32>(0.88, 0.36, 0.16), clamp(0.5 + terrain * 0.6, 0.0, 1.0));
        rust = mix(rust, vec3<f32>(0.22, 0.07, 0.03), canyon * 0.75);
        rust += vec3<f32>(0.98, 0.46, 0.20) * (olymp_shield * 0.35 + olymp_caldera * 0.95) * (0.25 + 1.05 * f_olymp);
        rust = mix(rust, vec3<f32>(0.92, 0.94, 0.98) * (1.0 + 0.25 * f_ice), ice);
        let rover_d = length(p_surf.xy - vec2<f32>(0.16, 0.12)) + select(1.0, 0.0, p_surf.z > 0.0);
        let rover_ring = exp(-abs(rover_d - 0.08) * 52.0) + exp(-rover_d * 48.0);
        let atmo = vec3<f32>(0.88, 0.44, 0.25) * fresnel * 0.32 * (0.2 + 0.8 * ndotl);
        let fx = vec3<f32>(1.45, 0.56, 0.18) * canyon * f_canyon * 0.85
               + vec3<f32>(0.32, 1.28, 1.58) * rover_ring * f_rover * 1.15
               + vec3<f32>(0.98, 0.86, 0.52) * exp(-abs(p_surf.y) * 28.0) * f_moons * 0.85;
        return (rust * (ndotl * 1.12 + 0.02) + atmo + fx) * inst.color_a.a;
    }

    if (vid == 6u || vid == 1406u) {
        // Z06 Jupiter: polar aurora (F0), counter-flowing bands (F1), Great Red Spot (F2), Europa ice resonance (F3), Galilean Laplace tracks (F4)
        let f_aur = doc_solar_feat_gate(inst, 0u);
        let f_band = doc_solar_feat_gate(inst, 1u);
        let f_grs = doc_solar_feat_gate(inst, 2u);
        let f_eur = doc_solar_feat_gate(inst, 3u);
        let f_gal = doc_solar_feat_gate(inst, 4u);

        let shear = curl_noise3(p_surf * 3.8 + vec3<f32>(time * (0.03 + 0.08 * f_band), 0.0, 0.0));
        let belt = sin(p_surf.y * 19.0 + shear.y * 1.8);
        let fine_edd = fbm3(p_surf * 9.0 + shear * 0.4, 3);
        var jup = mix(vec3<f32>(0.78, 0.46, 0.26), vec3<f32>(0.94, 0.82, 0.64), clamp(0.5 + (0.5 + 0.18 * f_band) * belt + fine_edd * 0.25, 0.0, 1.0));
        let grs_d = length(vec2<f32>((p_surf.x - 0.18) * 1.2, (p_surf.y + 0.24) * 2.1)) + select(1.0, 0.0, p_surf.z > 0.0);
        let grs = exp(-grs_d * grs_d * 26.0);
        let grs_collar = exp(-abs(grs_d - 0.24) * 32.0) * f_grs;
        jup = mix(jup, vec3<f32>(0.92, 0.24, 0.12), grs * (1.0 + 0.35 * f_grs));
        let pole_aur = exp(-pow((abs(p_surf.y) - 0.88) * 14.0, 2.0)) * vec3<f32>(0.45, 0.65, 1.75) * (0.55 + 1.15 * f_aur);
        let fx = vec3<f32>(1.45, 0.68, 0.26) * grs_collar
               + vec3<f32>(0.32, 1.18, 1.65) * exp(-abs(p_surf.y) * 30.0) * f_eur * 0.90
               + vec3<f32>(1.25, 0.98, 0.45) * exp(-abs(abs(p_surf.y) - 0.14) * 26.0) * f_gal * 0.85;
        return (jup * (ndotl * 1.1 + 0.02) + pole_aur + fx) * inst.color_a.a;
    }

    if (vid == 7u || vid == 8u || vid == 1408u || vid == 1409u) {
        // Z08 Uranus (7 / 1408) & Z09 Neptune (8 / 1409)
        let is_uranus = (vid == 7u || vid == 1408u);
        let f0 = doc_solar_feat_gate(inst, 0u);
        let f1 = doc_solar_feat_gate(inst, 1u);
        let f2 = doc_solar_feat_gate(inst, 2u);
        let f3 = doc_solar_feat_gate(inst, 3u);
        let f4 = doc_solar_feat_gate(inst, 4u);

        let wind_speed = select(0.04 + 0.42 * f2, 0.03 + 0.18 * f4, is_uranus);
        let n_band = fbm3(p_surf * 5.2 + vec3<f32>(time * wind_speed, 0.0, 0.0), 3);
        let bands = sin(p_surf.y * select(12.0, 6.0, is_uranus) + n_band * 2.0);
        var col = mix(inst.color_a.rgb, inst.color_b.rgb, 0.5 + 0.4 * bands);
        var extra = vec3<f32>(0.0);

        if (is_uranus) {
            // F0: Sunlit polar solstice cap & pole axis tick; F1: deep methane band; F2: dim glare for charcoal rings; F3: Miranda flare; F4: zonal winds
            let hood = smoothstep(0.55, 0.92, p_surf.y) * (0.20 + 0.95 * f0);
            let axis_pin = exp(-length(p_surf.xz) * 22.0) * step(0.75, abs(p_surf.y)) * f0;
            col = mix(col, vec3<f32>(0.84, 0.96, 1.02), clamp(hood, 0.0, 1.0));
            col *= mix(1.0, 0.78, f2);
            col = mix(col, vec3<f32>(0.14, 0.82, 0.95), f1 * 0.45);
            let streak = pow(max(0.0, sin(p_surf.y * 22.0 + n_band * 4.0)), 6.0) * f4;
            extra = vec3<f32>(1.15, 1.35, 1.55) * axis_pin
                  + vec3<f32>(0.55, 1.05, 1.35) * fresnel * f3 * 0.85
                  + vec3<f32>(0.72, 1.15, 1.35) * streak * 0.72;
        } else {
            // Z09 Neptune: F0 high white methane cirrus, F1 Great Dark Spot + Scooter, F2 >2000 km/h supersonic wind streaks, F3/F4 Triton capture & geyser cues
            let spot_d = length(vec2<f32>(p_surf.x + 0.14, (p_surf.y + 0.18) * 1.9)) + select(1.0, 0.0, p_surf.z > 0.0);
            let dark_spot = exp(-spot_d * 12.0);
            let cirrus = pow(max(0.0, sin(p_surf.y * 28.0 + n_band * 4.0)), 8.0) * (0.35 + 0.95 * f0);
            let scooter = exp(-length(p_surf.xy - vec2<f32>(0.10, -0.34)) * 22.0) * select(0.0, 1.0, p_surf.z > 0.0);
            let jet_streak = pow(max(0.0, sin(p_surf.y * 16.0 - time * 3.8 + n_band * 3.0)), 10.0) * exp(-abs(p_surf.y) * 3.5);
            col = mix(col, vec3<f32>(0.04, 0.08, 0.32), dark_spot) + vec3<f32>(0.6, 0.8, 1.0) * cirrus;
            extra = vec3<f32>(1.15, 1.25, 1.45) * (scooter * (0.2 + 1.0 * f1) + exp(-abs(spot_d - 0.18) * 34.0) * f1)
                  + vec3<f32>(0.42, 1.15, 1.65) * jet_streak * f2 * 1.15
                  + vec3<f32>(0.85, 0.78, 1.15) * fresnel * (f3 + f4) * 0.75;
        }
        return (col * (ndotl * 1.1 + 0.02) + inst.color_b.rgb * fresnel * 0.35 + extra) * inst.color_a.a;
    }

    if (vid == 1410u || (vid == 10u && doc_submode(inst) == 10u)) {
        // Z10 Kuiper Belt KBO: dark crimson Cthulhu tholin patches & frosty white-cyan N2/CH4 volatiles + F1..F4 distinct cues
        let f0 = doc_solar_feat_gate(inst, 0u);
        let f1 = doc_solar_feat_gate(inst, 1u);
        let f2 = doc_solar_feat_gate(inst, 2u);
        let f3 = doc_solar_feat_gate(inst, 3u);
        let f4 = doc_solar_feat_gate(inst, 4u);
        let th = fbm3(p_surf * 5.2 + vec3<f32>(0.8, -1.4, 2.2), 4);
        let frost = smoothstep(0.06, 0.34, th);
        let az = atan2(p_surf.z, p_surf.x);
        var kbo = mix(vec3<f32>(0.28, 0.12, 0.10), vec3<f32>(0.84, 0.90, 0.96), frost);
        // F1 Haumea crystalline ice band; F2 Makemake reddish-amber methane-tholin sheen; F3 2:3 resonance 3-lobe nodes; F4 comet coma rim
        kbo = mix(kbo, vec3<f32>(0.90, 0.96, 1.05), f1 * 0.48 * smoothstep(0.35, 0.0, abs(p_surf.y)));
        kbo = mix(kbo, vec3<f32>(0.86, 0.38, 0.20), f2 * 0.52 * (1.0 - frost * 0.45));
        let ice_glint = pow(ndotl, 12.0) * frost * (0.35 + 0.85 * f0);
        let haumea_band = exp(-abs(p_surf.y) * 26.0) * f1;
        let makemake_cap = smoothstep(0.45, 0.85, abs(p_surf.y)) * f2;
        let res_3lobe = exp(-abs(p_surf.y) * 24.0) * pow(0.5 + 0.5 * cos(az * 3.0), 4.0) * f3;
        let coma_rim = fresnel * f4 * 0.88;
        return (kbo * (ndotl * 1.18 + 0.015)
             + vec3<f32>(0.75, 0.98, 1.25) * ice_glint
             + vec3<f32>(0.85, 0.94, 1.10) * haumea_band * 0.72
             + vec3<f32>(1.15, 0.62, 0.34) * makemake_cap * 0.68
             + vec3<f32>(0.36, 1.15, 1.55) * res_3lobe * 0.88
             + vec3<f32>(0.42, 0.98, 1.48) * coma_rim) * inst.color_a.a;
    }

    if (vid == 9u || vid == 1411u) {
        // Z11 Pluto: tholin crust + Sputnik Planitia convection cells (F1) + water-ice peaks (F2) + stratified blue N2/CH4 Fresnel haze (F0)
        let f_haze = doc_solar_feat_gate(inst, 0u);
        let f_sput = doc_solar_feat_gate(inst, 1u);
        let f_mtn = doc_solar_feat_gate(inst, 2u);
        let f_charon = doc_solar_feat_gate(inst, 3u);
        let f_moons = doc_solar_feat_gate(inst, 4u);

        let heart_l = length((p_surf.xy - vec2<f32>(-0.08, -0.02)) * vec2<f32>(1.5, 1.1));
        let heart_r = length((p_surf.xy - vec2<f32>(0.09, -0.02)) * vec2<f32>(1.5, 1.1));
        let heart = smoothstep(0.28, 0.16, min(heart_l, heart_r)) * select(0.0, 1.0, p_surf.z > 0.15);
        let tholin = fbm3(p_surf * 5.5, 4);
        var col = mix(vec3<f32>(0.38, 0.18, 0.14), vec3<f32>(0.78, 0.62, 0.50), clamp(0.5 + tholin * 0.6, 0.0, 1.0));
        // Polygonal nitrogen convection cells inside western Sputnik Planitia
        let poly = abs(sin(p_surf.x * 46.0 + snoise3(p_surf * 18.0) * 1.6) * sin(p_surf.y * 46.0));
        let cell_seam = exp(-poly * 12.0) * smoothstep(0.24, 0.12, heart_l) * select(0.0, 1.0, p_surf.z > 0.15);
        let sp_col = mix(vec3<f32>(0.94, 0.89, 0.80), vec3<f32>(0.68, 0.82, 0.94), cell_seam * f_sput * 0.72);
        col = mix(col, sp_col, heart);
        // Al-Idrisi water-ice mountains along western margin of Sputnik Planitia
        let ridge = exp(-abs(heart_l - 0.25) * 28.0) * smoothstep(-0.32, 0.02, -p_surf.x) * pow(max(0.0, snoise3(p_surf * 22.0)), 1.8) * select(0.0, 1.0, p_surf.z > 0.15);
        col += vec3<f32>(0.88, 0.94, 1.05) * ridge * (0.25 + 1.15 * f_mtn);

        let blue_haze = pow(1.0 - view_dot, 5.2) * vec3<f32>(0.42, 0.78, 1.42) * (0.28 + 0.72 * ndotl) * (1.0 + 1.45 * f_haze);
        let bary_arc = exp(-abs(p_surf.y) * 28.0) * (f_charon * 0.85 + f_moons * 0.55);
        return (col * (ndotl * 1.15 + 0.02) + blue_haze + vec3<f32>(0.55, 0.88, 1.32) * bary_arc) * inst.color_a.a;
    }

    if (vid == 10u) {
        // App 21 Ocean Worlds Europa / Enceladus cracked ice plates
        return shade_doc_europa_ice(p_surf, ndotl, fresnel, inst.color_a.a, 0.0);
    }

    if (vid >= 3643u && vid <= 3651u) {
        // Exp 04 Canonical Companion Moons & Dwarf Partners (`3643..=3651`)
        let sub = doc_submode(inst);
        if (vid == 3643u) {
            // Earth's Moon: dark basalt maria + bright highlands + Tycho ray highlight on F3
            let mare = smoothstep(-0.15, 0.22, fbm3(p_surf * 3.8, 4));
            let m_crater = exp(-abs(abs(snoise3(p_surf * 11.0)) - 0.24) * 24.0);
            let m_col = mix(vec3<f32>(0.24, 0.25, 0.28), vec3<f32>(0.68, 0.70, 0.74), clamp(mare + m_crater * 0.35, 0.0, 1.0));
            return (m_col * (ndotl * 1.15 + 0.02) + vec3<f32>(0.62, 0.92, 1.35) * fresnel * doc_solar_feat_gate(inst, 3u) * 0.75) * inst.color_a.a;
        }
        if (vid == 3645u && doc_feat(inst) == 2u) {
            // Asteroid Belt Fe-Ni Psyche metallic core sphere
            let half_v = normalize(sun_local + v_dir);
            let spec = pow(max(dot(n_surf, half_v), 0.0), 72.0);
            return vec3<f32>(0.62, 0.66, 0.72) * (0.18 + 0.85 * ndotl + fresnel * 0.45) + vec3<f32>(1.45, 1.48, 1.52) * spec * 1.45;
        }
        if (vid == 3646u && sub == 1u) {
            // Jupiter Europa companion (`submode == 1u`): subsurface saltwater ocean cutaway on F3!
            return shade_doc_europa_ice(p_surf, ndotl, fresnel, inst.color_a.a, doc_solar_feat_gate(inst, 3u));
        }
        if (vid == 3647u && sub == 0u) {
            // Saturn Titan: thick amber-orange photochemical haze + dark specular polar Kraken Mare hydrocarbon sea on F3!
            let f_titan = doc_solar_feat_gate(inst, 3u);
            let mare_lat = smoothstep(0.55, 0.86, p_surf.y) * smoothstep(-0.1, 0.35, snoise3(p_surf * 5.0));
            let base_titan = mix(inst.color_a.rgb, inst.color_b.rgb, 0.5 + 0.5 * sin(p_surf.y * 8.0));
            let kraken = mix(base_titan, vec3<f32>(0.03, 0.06, 0.14), mare_lat * (0.25 + 0.75 * f_titan));
            return (kraken * (ndotl * 1.05 + 0.04) + vec3<f32>(0.48, 0.85, 1.25) * fresnel * (0.35 + 0.65 * f_titan)) * inst.color_a.a;
        }
        if (vid == 3647u && sub == 1u) {
            // Saturn Enceladus: bright white H2O ice + glowing cyan south-polar tiger-stripe fissures
            let f_enc = doc_solar_feat_gate(inst, 4u);
            let stripe = pow(max(0.0, cos(p_surf.x * 24.0 + p_surf.z * 14.0)), 14.0) * smoothstep(-0.38, -0.82, p_surf.y);
            return (vec3<f32>(0.88, 0.94, 0.98) * (ndotl * 1.12 + 0.03) + vec3<f32>(0.25, 1.25, 1.75) * stripe * (0.35 + 1.05 * f_enc)) * inst.color_a.a;
        }
        if (vid == 3649u) {
            // Neptune Triton: pink-cream cantaloupe terrain + dark downwind Voyager-2 N2 geyser plumes on F4!
            let cantaloupe = abs(snoise3(p_surf * 14.0));
            let geyser = exp(-abs(p_surf.x - (p_surf.y + 0.45) * 0.8) * 26.0) * smoothstep(-0.65, -0.05, p_surf.y) * smoothstep(0.0, 0.35, p_surf.z);
            let t_col = mix(vec3<f32>(0.68, 0.58, 0.58), vec3<f32>(0.90, 0.86, 0.82), clamp(cantaloupe * 1.4, 0.0, 1.0));
            let dark_streak = mix(t_col, vec3<f32>(0.08, 0.07, 0.10), geyser * (0.15 + 0.80 * doc_solar_feat_gate(inst, 4u)));
            return (dark_streak * (ndotl * 1.12 + 0.025) + inst.color_b.rgb * fresnel * 0.32) * inst.color_a.a;
        }
        if (vid == 3651u) {
            // Pluto Charon: fractured grey water ice + dark red-rust Mordor Macula north-polar tholin cap
            let mordor = smoothstep(0.62, 0.88, p_surf.y + snoise3(p_surf * 5.0) * 0.08);
            let chasm = exp(-abs(p_surf.y + 0.04 * sin(p_surf.x * 6.0)) * 34.0);
            var ch_col = mix(vec3<f32>(0.42, 0.44, 0.48), vec3<f32>(0.68, 0.70, 0.74), 0.5 + 0.5 * fbm3(p_surf * 7.5, 3));
            ch_col = mix(ch_col, vec3<f32>(0.54, 0.18, 0.12), mordor * 0.88);
            ch_col = mix(ch_col, vec3<f32>(0.18, 0.16, 0.18), chasm * 0.65);
            return (ch_col * (ndotl * 1.12 + 0.025) + vec3<f32>(0.52, 0.85, 1.15) * fresnel * doc_solar_feat_gate(inst, 3u) * 0.55) * inst.color_a.a;
        }
        // Remaining Exp 04 companions: Phobos/Deimos (3644), Vesta Rheasilvia basin (3645), Io volcanism (3646 sub==0), Miranda Verona Rupes (3648), Haumea/Makemake/Comet nucleus (3650)
        let n_m = fbm3(p_surf * 7.5 + vec3<f32>(f32(vid - 3640u) * 1.7), 4);
        var m_col = mix(inst.color_a.rgb, inst.color_b.rgb, clamp(0.5 + n_m * 0.65, 0.0, 1.0));
        if (vid == 3645u) {
            // Vesta giant Rheasilvia south-polar impact basin + central peak
            let rhea = smoothstep(-0.42, -0.78, p_surf.y);
            m_col = mix(m_col, vec3<f32>(0.88, 0.84, 0.76), rhea * 0.65);
        } else if (vid == 3646u) {
            // Io pyroclastic lava calderas
            let volcano_hot = exp(-length(p_surf.xy - vec2<f32>(-0.12, 0.16)) * 24.0) * select(0.0, 1.0, p_surf.z > 0.0);
            m_col += vec3<f32>(1.55, 0.42, 0.06) * volcano_hot;
        } else if (vid == 3648u) {
            // Miranda bright chevron corona & Verona Rupes scarp
            let corona_v = exp(-abs(abs(p_surf.x) - abs(p_surf.y) * 0.8) * 18.0);
            m_col = mix(m_col, vec3<f32>(0.88, 0.94, 0.98), corona_v * 0.75);
        } else if (vid == 3650u) {
            // Kuiper dwarf / orbital resonance / comet companion (`sub`: 1=Haumea, 2=Makemake, 3=2:3 resonance, 4=comet)
            if (sub == 1u) {
                let spot_r = exp(-length(p_surf.xy - vec2<f32>(0.16, 0.08)) * 18.0) * select(0.0, 1.0, p_surf.z > 0.0);
                m_col = mix(vec3<f32>(0.86, 0.90, 0.96), vec3<f32>(0.58, 0.22, 0.16), spot_r * 0.72);
                m_col += vec3<f32>(0.72, 0.88, 1.05) * exp(-abs(p_surf.y) * 24.0) * 0.55;
            } else if (sub == 2u) {
                let ch4_cap = smoothstep(0.45, 0.88, abs(p_surf.y) + n_m * 0.10);
                m_col = mix(vec3<f32>(0.82, 0.36, 0.18), vec3<f32>(0.96, 0.76, 0.52), ch4_cap);
            } else if (sub == 3u) {
                let az_res = atan2(p_surf.z, p_surf.x);
                let l_arc = exp(-abs(sin(az_res * 3.0)) * 9.0) * exp(-abs(p_surf.y) * 6.0);
                let eq_line = exp(-abs(p_surf.y) * 26.0);
                m_col = mix(m_col, vec3<f32>(0.24, 0.76, 1.18), 0.55);
                m_col += vec3<f32>(0.42, 1.25, 1.65) * l_arc * 0.85 + vec3<f32>(1.28, 0.96, 0.42) * eq_line * 0.65;
            } else if (sub == 4u) {
                let jet = pow(max(0.0, snoise3(p_surf * 12.0 + vec3<f32>(time * 0.5))), 6.0);
                m_col = mix(m_col, vec3<f32>(0.22, 0.24, 0.28), 0.55) + vec3<f32>(0.36, 1.15, 1.65) * jet;
            }
        }
        return (m_col * (ndotl * 1.12 + 0.025) + inst.color_b.rgb * fresnel * 0.35) * inst.color_a.a;
    }

    if (vid == 11u || vid == 3820u) {
        // Exoplanet Biosignature World / Canonical Exp 21 Companion (`3820`) / Material Target
        let sub = select(0u, doc_submode(inst), vid == 3820u);
        let feat_f = f32(doc_feat(inst) & 7u);
        let spin_p = doc_rot_y(p_surf, feat_f * 0.42);
        let n_t = fbm3(spin_p * 4.2 + vec3<f32>(time * 0.02), 4);
        let bio_land = smoothstep(0.05, 0.25, n_t);
        var sea_c = vec3<f32>(0.02, 0.12, 0.28);
        var atmo_c = inst.color_b.rgb;
        if (sub == 3u) {
            // False-positive volcanic / desiccated abiotic mimic
            sea_c = vec3<f32>(0.16, 0.05, 0.03);
            atmo_c = vec3<f32>(1.15, 0.46, 0.18);
        } else if (sub == 2u) {
            // Biosignature world with red-edge land + O2/O3 cyan limb
            sea_c = vec3<f32>(0.02, 0.18, 0.34);
            atmo_c = vec3<f32>(0.24, 1.18, 1.42);
        } else if (sub == 1u) {
            // Transiting Super-Earth ocean world
            sea_c = vec3<f32>(0.04, 0.22, 0.42);
        }
        let col = mix(sea_c, mix(inst.color_a.rgb, atmo_c * 0.72, clamp(0.5 + n_t, 0.0, 1.0)), bio_land);
        return (col * (ndotl * 1.1 + 0.02) + atmo_c * fresnel * 0.5) * inst.color_a.a;
    }

    if (vid >= 3830u && vid <= 3835u) {
        // Canonical Exp 22 Companions (`3830..=3835`): secondary observatories, target stars, responders, trust & coexistence worlds
        let sub = doc_submode(inst);
        let feat_f = f32(doc_feat(inst) & 7u);
        let spin_p = doc_rot_y(p_surf, time * 0.08 + feat_f * 0.45);
        let n_t = fbm3(spin_p * 4.5 + vec3<f32>(f32(sub) * 1.3), 4);
        let lat_grid = exp(-abs(sin(spin_p.y * 16.0)) * 12.0);
        let lon_grid = exp(-abs(sin(atan2(spin_p.z, spin_p.x) * 6.0)) * 12.0);
        let grid_g = max(lat_grid, lon_grid);
        var surf_c = inst.color_a.rgb;
        var rim_c = inst.color_b.rgb;
        if (sub == 0u) {
            // 3830 (Z04): Synchronized terrestrial observatory node
            let land = smoothstep(0.02, 0.24, n_t);
            surf_c = mix(vec3<f32>(0.02, 0.14, 0.30), vec3<f32>(0.16, 0.62, 0.46), land) + vec3<f32>(0.22, 0.95, 1.25) * grid_g * 0.45;
        } else if (sub == 1u) {
            // 3831 (Z05): Distant solar-type host star + transiting signal belt
            let gran = 0.65 + 0.35 * sin(n_t * 9.0 + time * 1.2);
            let eq_band = exp(-abs(spin_p.y) * 18.0);
            surf_c = mix(vec3<f32>(0.92, 0.56, 0.16), vec3<f32>(1.18, 0.92, 0.42), gran) + vec3<f32>(0.24, 1.05, 1.32) * eq_band * 0.6;
        } else if (sub == 2u) {
            // 3832 (Z19): Relativistic proper-motion target star with drift wake
            let pulse = 0.5 + 0.5 * sin(atan2(spin_p.z, spin_p.x) * 4.0 - time * 2.0);
            surf_c = mix(vec3<f32>(0.28, 0.58, 0.98), vec3<f32>(1.12, 0.84, 0.34), pulse * 0.6) + vec3<f32>(0.36, 1.12, 1.42) * grid_g * 0.35;
        } else if (sub == 3u) {
            // 3833 (Z21..Z22): Alien intelligent responder world with structured carrier bands
            let rings = exp(-abs(sin(spin_p.y * 20.0 - time * 1.5)) * 10.0);
            let land = smoothstep(0.06, 0.28, n_t);
            surf_c = mix(vec3<f32>(0.04, 0.18, 0.36), vec3<f32>(0.24, 0.72, 0.96), land) + vec3<f32>(0.42, 0.86, 1.38) * rings * 0.55;
        } else if (sub == 4u) {
            // 3834 (Z25..Z26): Reciprocal trust & diplomatic treaty partner biosphere
            let seal = exp(-abs(abs(spin_p.y) - 0.35) * 16.0);
            let land = smoothstep(0.04, 0.25, n_t);
            surf_c = mix(vec3<f32>(0.08, 0.24, 0.32), vec3<f32>(0.78, 0.68, 0.24), land) + vec3<f32>(0.28, 1.15, 0.88) * max(seal, grid_g * 0.5) * 0.5;
        } else {
            // 3835 (Z27): Dual-perspective coexistence counterpart world
            let dawn = smoothstep(-0.3, 0.4, dot(normalize(spin_p), vec3<f32>(0.7, 0.3, 0.6)));
            surf_c = mix(vec3<f32>(0.14, 0.42, 0.74), vec3<f32>(0.96, 0.68, 0.26), dawn * 0.72 + 0.15 * n_t)
                   + vec3<f32>(0.26, 1.02, 1.32) * exp(-abs(spin_p.y) * 14.0) * 0.38;
        }
        return (surf_c * (ndotl * 1.08 + 0.04) + rim_c * fresnel * 0.55) * inst.color_a.a;
    }

    if (vid >= 3740u && vid <= 3759u) {
        // Canonical Act III Companions (`3740..=3749` Exp 13 Matter/Energy; `3750..=3759` Exp 14 Spacetime)
        let sub = doc_submode(inst);
        let feat_f = f32(doc_feat(inst) & 7u);
        let slot = vid - 3740u;
        let spin_p = doc_rot_y(p_surf, time * 0.14 + feat_f * 0.42);
        let lat_grid = exp(-abs(sin(spin_p.y * 14.0)) * 12.0);
        let lon_grid = exp(-abs(sin(atan2(spin_p.z, spin_p.x) * 6.0)) * 12.0);
        let grid_g = max(lat_grid, lon_grid);
        var surf_c = inst.color_a.rgb * 0.55;
        var rim_c = inst.color_b.rgb * 0.95;
        if (slot < 4u) {
            // Exp 13 Z00..Z03: crystal lattice / dipole / tetrahedral / Bohr node
            let hex_b = 0.5 + 0.5 * sin(spin_p.x * 12.0) * cos(spin_p.y * 12.0);
            surf_c = mix(vec3<f32>(0.08, 0.24, 0.42), vec3<f32>(0.24, 0.78, 0.92), hex_b * 0.65)
                   + vec3<f32>(0.22, 0.85, 1.05) * grid_g * 0.42;
            rim_c = vec3<f32>(0.28, 0.92, 1.15);
        } else if (slot < 8u) {
            // Exp 13 Z04..Z07: nucleon triad / QCD bag / Higgs potential sphere
            let triad = exp(-abs(sin(atan2(spin_p.z, spin_p.x) * 3.0 + time * 0.4)) * 10.0);
            surf_c = mix(vec3<f32>(0.34, 0.12, 0.36), vec3<f32>(0.92, 0.52, 0.18), triad * 0.72)
                   + vec3<f32>(0.95, 0.72, 0.24) * exp(-abs(spin_p.y) * 16.0) * 0.45;
            rim_c = vec3<f32>(1.05, 0.58, 0.22);
        } else if (slot < 10u) {
            // Exp 13 Z08..Z11: QFT Feynman propagator / Casimir / symmetry ring
            let mode_w = 0.5 + 0.5 * sin(spin_p.y * 18.0 - time * 1.4);
            surf_c = mix(vec3<f32>(0.10, 0.28, 0.46), vec3<f32>(0.68, 0.38, 0.92), mode_w)
                   + vec3<f32>(0.36, 0.96, 1.12) * grid_g * 0.38;
            rim_c = vec3<f32>(0.76, 0.52, 1.12);
        } else if (slot < 13u) {
            // Exp 14 Z00..Z03: Wheeler curvature / light-clock / Minkowski light-cone emblem
            let cone_band = exp(-abs(abs(spin_p.y) - length(spin_p.xz) * 0.85) * 14.0);
            surf_c = mix(vec3<f32>(0.08, 0.16, 0.32), vec3<f32>(0.96, 0.78, 0.28), cone_band * 0.8)
                   + vec3<f32>(0.24, 0.68, 0.98) * grid_g * 0.35;
            rim_c = vec3<f32>(1.02, 0.82, 0.32);
        } else if (slot < 17u) {
            // Exp 14 Z04..Z07: stress-energy source / equivalence / Shapiro lens / Kerr ergosphere
            let drag = 0.5 + 0.5 * sin(atan2(spin_p.z, spin_p.x) * 4.0 + spin_p.y * 8.0 - time * 1.1);
            surf_c = mix(vec3<f32>(0.12, 0.08, 0.24), vec3<f32>(0.94, 0.48, 0.16), drag * 0.65)
                   + vec3<f32>(0.98, 0.74, 0.34) * exp(-abs(spin_p.y) * 14.0) * 0.48;
            rim_c = vec3<f32>(1.02, 0.64, 0.24);
        } else {
            // Exp 14 Z08..Z14: LIGO quadrupole / Hawking horizon / AdS-CFT holographic boundary
            let quad = 0.5 + 0.5 * cos(atan2(spin_p.z, spin_p.x) * 2.0 - time * 1.6);
            let h_tile = exp(-abs(sin(spin_p.y * 20.0)) * 10.0) * exp(-abs(sin(atan2(spin_p.z, spin_p.x) * 10.0)) * 10.0);
            surf_c = mix(vec3<f32>(0.06, 0.20, 0.36), vec3<f32>(0.36, 0.82, 1.05), quad * 0.55)
                   + vec3<f32>(0.88, 0.64, 0.24) * h_tile * 0.62 + vec3<f32>(0.26, 0.92, 1.12) * f32(sub & 3u) * 0.04;
            rim_c = vec3<f32>(0.42, 0.92, 1.15);
        }
        return (surf_c * (ndotl * 1.04 + 0.04) + rim_c * fresnel * 0.42) * 0.78 * inst.color_a.a;
    }

    if (vid >= 3760u && vid <= 3762u) {
        // Canonical Act IV Exp 15 Complexity Companions (`3760..=3762`)
        let feat_f = f32(doc_feat(inst) & 7u);
        let spin_p = doc_rot_y(p_surf, time * 0.16 + feat_f * 0.45);
        let lat_grid = exp(-abs(sin(spin_p.y * 14.0)) * 12.0);
        let lon_grid = exp(-abs(sin(atan2(spin_p.z, spin_p.x) * 6.0)) * 12.0);
        let grid_g = max(lat_grid, lon_grid);
        var surf_c = inst.color_a.rgb * 0.55;
        var rim_c = inst.color_b.rgb * 0.95;
        if (vid == 3760u) {
            // Z04 nonequilibrium Carnot thermal gradient reservoir sphere
            let therm = smoothstep(-0.65, 0.65, spin_p.y);
            surf_c = mix(vec3<f32>(1.05, 0.38, 0.14), vec3<f32>(0.16, 0.88, 0.68), therm)
                   + vec3<f32>(0.98, 0.74, 0.28) * grid_g * 0.35;
            rim_c = mix(vec3<f32>(1.15, 0.48, 0.18), vec3<f32>(0.24, 1.05, 0.78), therm);
        } else if (vid == 3761u) {
            // Z08 networks / Z11 chemistry hub & catalytic monomer reservoir
            let net_w = pow(max(0.0, cos(spin_p.x * 10.0) * sin(spin_p.y * 10.0) * cos(spin_p.z * 10.0)), 4.0);
            surf_c = mix(vec3<f32>(0.10, 0.36, 0.44), vec3<f32>(0.98, 0.76, 0.24), net_w)
                   + vec3<f32>(0.28, 0.96, 0.82) * grid_g * 0.36;
            rim_c = vec3<f32>(0.98, 0.82, 0.32);
        } else {
            // 3762u: Z12 replication / Z13 protocells daughter lipid vesicle
            let pore = pow(max(0.0, snoise3(spin_p * 7.5 + vec3<f32>(time * 0.08))), 6.0);
            surf_c = mix(vec3<f32>(0.08, 0.34, 0.36), vec3<f32>(0.22, 0.92, 0.62), 0.45 + 0.55 * abs(spin_p.y))
                   + vec3<f32>(0.42, 1.05, 0.92) * pore * 0.75;
            rim_c = vec3<f32>(0.32, 1.08, 0.78);
        }
        return (surf_c * (ndotl * 1.04 + 0.04) + rim_c * fresnel * 0.44) * 0.76 * inst.color_a.a;
    }

    if (vid >= 3770u && vid <= 3773u) {
        // Canonical Act IV Exp 16 Life Companions (`3770..=3773`)
        let feat_f = select(0.0, f32(doc_feat(inst)), doc_feat(inst) < 9u);
        let spin_p = doc_rot_y(p_surf, time * 0.14 + feat_f * 0.40);
        let lon_a = atan2(spin_p.z, spin_p.x);
        var surf_c = inst.color_a.rgb * 0.55;
        var rim_c = inst.color_b.rgb * 0.95;
        if (vid == 3770u) {
            // Z04 heredity: RNA polymerase globe wrapped by a single-strand transcript groove
            let groove = exp(-abs(sin(lon_a * 2.0 + spin_p.y * 9.0 - time * 0.6)) * 9.0);
            let pocket = smoothstep(0.55, 0.85, spin_p.x);
            surf_c = mix(vec3<f32>(0.10, 0.22, 0.42), vec3<f32>(0.34, 0.62, 1.02), 0.5 + 0.5 * spin_p.y)
                   + vec3<f32>(1.02, 0.46, 0.92) * groove * 0.62
                   + vec3<f32>(0.98, 0.78, 0.28) * pocket * 0.30;
            rim_c = vec3<f32>(0.82, 0.48, 1.08);
        } else if (vid == 3771u) {
            // Z07 complex cells: engulfed alpha-proteobacterium endosymbiont with folded cristae
            let cristae = exp(-abs(sin(spin_p.x * 13.0 + sin(spin_p.y * 6.0) * 0.8)) * 8.0);
            surf_c = mix(vec3<f32>(0.40, 0.14, 0.08), vec3<f32>(1.02, 0.52, 0.20), cristae * 0.85)
                   + vec3<f32>(0.24, 0.62, 0.92) * smoothstep(0.75, 0.98, abs(spin_p.y)) * 0.32;
            rim_c = vec3<f32>(1.04, 0.62, 0.26);
        } else if (vid == 3772u) {
            // Z12 ecosystems: miniature biosphere globe (ocean / canopy / desert / ice bands)
            let lat = spin_p.y;
            let land = smoothstep(-0.05, 0.25, snoise3(spin_p * 3.2));
            var biome = mix(vec3<f32>(0.06, 0.26, 0.62), vec3<f32>(0.18, 0.74, 0.30), land);
            biome = mix(biome, vec3<f32>(0.84, 0.66, 0.32), land * smoothstep(0.32, 0.05, abs(lat)) * 0.65);
            biome = mix(biome, vec3<f32>(0.78, 0.88, 0.96), smoothstep(0.72, 0.88, abs(lat)));
            let trophic = exp(-abs(lat - 0.18 * sin(lon_a * 3.0 + time * 0.5)) * 26.0);
            surf_c = biome + vec3<f32>(1.02, 0.84, 0.30) * trophic * 0.38;
            rim_c = vec3<f32>(0.42, 0.88, 1.08);
        } else {
            // 3773u: Z15 nervous: peripheral sensory-motor reflex loop sphere with travelling spike band
            let band = exp(-abs(fract(lon_a / 6.2831853 + time * 0.22) - 0.5) * 18.0);
            let myelin = step(0.55, fract(spin_p.y * 6.0)) * 0.18;
            surf_c = vec3<f32>(0.08, 0.24, 0.48) + vec3<f32>(0.22, 0.62, 1.02) * myelin
                   + vec3<f32>(1.04, 0.86, 0.30) * band * 0.70;
            rim_c = vec3<f32>(0.36, 0.86, 1.10);
        }
        return (surf_c * (ndotl * 1.04 + 0.04) + rim_c * fresnel * 0.44) * 0.76 * inst.color_a.a;
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

    // Banded Gas Giants (Saturn default vid == 2 or canonical vid == 1407) w/ North Polar Hexagon
    let is_saturn = (vid == 2u || vid == 1407u);
    let f_hex = select(0.0, doc_solar_feat_gate(inst, 0u), vid == 1407u);
    let f_bring = select(0.0, doc_solar_feat_gate(inst, 1u), vid == 1407u);
    let f_cass = select(0.0, doc_solar_feat_gate(inst, 2u), vid == 1407u);
    let f_titan = select(0.0, doc_solar_feat_gate(inst, 3u), vid == 1407u);
    let f_enc = select(0.0, doc_solar_feat_gate(inst, 4u), vid == 1407u);

    let band_warp = fbm3(p_surf * 4.5 + vec3<f32>(time * 0.02, 0.0, 0.0), 4);
    let bands = sin(p_surf.y * 15.0 + band_warp * 3.2);
    var surf_col = mix(inst.color_a.rgb, inst.color_b.rgb, 0.5 + 0.5 * bands);
    if (is_saturn && p_surf.y > 0.74) {
        let hex_r = length(p_surf.xz) * 2.8;
        let hex_ang = atan2(p_surf.z, p_surf.x) - time * 0.35 * f_hex;
        let hex = exp(-abs(hex_r - 0.58 - 0.08 * cos(6.0 * hex_ang)) * 18.0);
        surf_col = mix(surf_col, vec3<f32>(0.32, 0.85, 1.15) * (1.0 + 0.55 * f_hex), hex * clamp(0.65 + 0.35 * f_hex, 0.0, 1.0));
    }
    let rim_glow = inst.color_b.rgb * fresnel * (0.25 + 0.75 * ndotl)
                 + vec3<f32>(1.15, 0.85, 0.36) * f_bring * 0.25
                 + vec3<f32>(0.42, 0.65, 1.15) * f_cass * fresnel * 0.45
                 + vec3<f32>(1.15, 0.62, 0.20) * fresnel * f_titan * 0.55
                 + vec3<f32>(0.35, 1.15, 1.55) * fresnel * f_enc * 0.65;
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
    // 03 Deep Earth Shell: Z00 Space, Z01 Exosphere, Z02 Thermosphere (ISS/Ionosphere/Aurora),
    // Z03 Mesosphere (Meteors/NLCs/Rocket), Z04 Stratosphere (Ozone/Balloon/Contrails/UV), Z05 Troposphere, Z12 Geodynamo
    var accum_rgb = vec3<f32>(0.0);
    var accum_a: f32 = 0.0;
    let raw_vid = inst.kind_tags.y;
    let is_exp03 = (doc_is_canonical_zone_vid(raw_vid) && doc_vid_exp(raw_vid) == 3u)
        || (!doc_is_canonical_zone_vid(raw_vid) && doc_submode(inst) > 0u);
    let is_exp04_earth = raw_vid == 1403u;
    let z_idx = select(
        select(99u, 98u, is_exp04_earth),
        select(doc_zone(inst), doc_vid_zone(raw_vid), doc_is_canonical_zone_vid(raw_vid)),
        is_exp03,
    );
    let steps: i32 = i32(mix(16.0, 28.0, fovea_k));
    let dt = (t1 - t0) / f32(steps);
    let limb_r = select(inst.phys_a.y, 0.737, inst.phys_a.y < 0.05);
    let shell_w = max(0.98 - limb_r, 0.15);
    let show_magnetosphere = z_idx == 0u || z_idx == 12u || z_idx == 65535u || is_exp04_earth;
    let exp04_mag = max(doc_feat_focus(inst, 3u), doc_feat_focus(inst, 4u) * 0.90) * select(0.28, 0.88, doc_feat(inst) == 3u || doc_feat(inst) == 4u);
    let mag_gain = select(
        select(0.22, exp04_mag, is_exp04_earth),
        select(0.95 * doc_feat_focus(inst, 4u), 0.92 * doc_feat_focus(inst, 1u), z_idx == 0u),
        z_idx == 0u || z_idx == 12u,
    );
    let f_tropo_wind = select(select(1.0, doc_feat_focus(inst, 2u), is_exp04_earth), doc_feat_focus(inst, 2u), z_idx == 5u);
    let f_tropo_vapor = select(select(1.0, max(doc_feat_focus(inst, 0u), doc_feat_focus(inst, 1u)), is_exp04_earth), doc_feat_focus(inst, 4u), z_idx == 5u);

    for (var i: i32 = 0; i < 28; i = i + 1) {
        if (i >= steps || accum_a > 0.95) {
            break;
        }
        let t = t0 + (f32(i) + jitter) * dt;
        let p = ro + rd * t;
        let r = length(p);
        let alt = clamp((r - limb_r) / shell_w, 0.0, 1.0);

        // 0. Z05 Troposphere low-altitude atmospheric limb shell (jet stream & water vapor wisps)
        if (z_idx == 5u && r > limb_r && r < (limb_r + 0.09)) {
            let lat = p.y / r;
            let jet = exp(-pow((abs(lat) - 0.52) * 10.0, 2.0));
            let vapor_ring = exp(-pow(lat * 4.0, 2.0));
            let limb_env = smoothstep(limb_r, limb_r + 0.02, r) * smoothstep(limb_r + 0.09, limb_r + 0.04, r);
            let wisp = 0.5 + 0.5 * sin(atan2(p.z, p.x) * 6.0 - time * 1.4);
            let tropo_limb = vec3<f32>(0.24, 0.74, 1.05) * jet * f_tropo_wind * 0.42
                           + vec3<f32>(0.10, 0.78, 0.88) * vapor_ring * f_tropo_vapor * 0.48;
            let sample_a = limb_env * wisp * dt * 1.8;
            if (sample_a > 0.002) {
                accum_rgb += (1.0 - accum_a) * tropo_limb * sample_a;
                accum_a += (1.0 - accum_a) * min(sample_a, 0.25);
            }
        }

        // 1. Polar Auroral Oval Curtains touching Earth limb (strongest on Z02 Thermosphere)
        if (r > limb_r && r < (limb_r + 0.18)) {
            let lat = abs(p.y / r);
            let oval = exp(-pow((lat - 0.75) * 15.0, 2.0));
            if (oval > 0.01) {
                let h_alt = clamp((r - limb_r) / 0.17, 0.0, 1.0);
                let alt_env = smoothstep(0.0, 0.09, h_alt) * exp(-h_alt * 2.7);
                let b_dir = normalize(vec3<f32>(p.x, p.y * 0.12, p.z) + vec3<f32>(1e-4));
                let field_uv = fract(vec2<f32>(b_dir.x * 0.35 + b_dir.z * 0.25 + 0.5, h_alt));
                let field_s = textureSampleLevel(field_grid_tex, field_grid_samp, clamp(field_uv, vec2<f32>(0.01), vec2<f32>(0.99)), 0.0);
                let shear = curl_noise3(b_dir * 3.8 + vec3<f32>(0.0, time * 0.14, 0.0));
                let curtain = snoise3(b_dir * 7.8 + shear * 0.65 + vec3<f32>(0.0, h_alt * 1.4, time * 0.22));
                let sheet = exp(-pow(curtain * 5.0, 2.0)) * oval * alt_env;
                let aur_gain = select(0.45, 1.68 * doc_feat_focus(inst, 0u), z_idx == 2u);
                let pde_mod = mix(vec3<f32>(0.08, 1.75, 0.58), field_s.rgb, clamp(field_s.a * 0.45, 0.0, 0.55));
                let alt_col = mix(pde_mod, vec3<f32>(0.95, 0.18, 1.85), smoothstep(0.32, 0.86, h_alt)) * aur_gain;
                let sample_a = sheet * dt * (2.2 + bass * 0.9);
                accum_rgb += (1.0 - accum_a) * alt_col * sample_a;
                accum_a += (1.0 - accum_a) * min(sample_a, 0.42);
            }
        }

        // 2. Z00 Space (Dayside Bow Shock, Van Allen L-shells, Cosmic Rays, GEO Satellites, Interplanetary Vacuum)
        if (z_idx == 0u && r > limb_r && r < 0.94) {
            let f_sw = doc_feat_focus(inst, 0u);
            let f_cr = doc_feat_focus(inst, 2u);
            let f_geo = doc_feat_focus(inst, 3u);
            let f_vac = doc_feat_focus(inst, 4u);
            let az = atan2(p.z, p.x);
            let dayside = smoothstep(0.05, 0.85, p.x / r);
            let bow = exp(-pow((alt - (0.56 - dayside * 0.22)) * 22.0, 2.0)) * dayside * pow(0.5 + 0.5 * sin(p.y * 18.0 - time * 1.8), 2.0);
            let cosmic = pow(max(0.0, snoise3(doc_rot_z(p * 12.0, 0.45) - vec3<f32>(time * 1.2, 0.0, 0.0))), 12.0) * smoothstep(0.94, 0.72, r);
            let geo_ring = exp(-pow((alt - 0.68) * 28.0, 2.0)) * exp(-pow(p.y * 34.0, 2.0));
            let geo_node = pow(max(0.0, cos(az * 3.0 - time * 0.35)), 18.0) * geo_ring;
            let vac_wave = exp(-pow((alt - 0.80) * 26.0, 2.0)) * pow(0.5 + 0.5 * sin(az * 6.0 - time * 0.8), 6.0) * 0.45;
            let s_a = (bow * 0.55 + cosmic * 0.85 + geo_ring * 0.75 + geo_node + vac_wave) * dt * 1.5;
            if (s_a > 0.012) {
                let s_col = vec3<f32>(1.25, 0.72, 0.22) * bow * 0.75 * f_sw
                          + vec3<f32>(0.45, 1.18, 1.55) * cosmic * 1.25 * f_cr
                          + vec3<f32>(1.35, 0.98, 0.35) * (geo_ring * 0.45 + geo_node * 1.35) * f_geo
                          + vec3<f32>(0.24, 0.55, 1.15) * vac_wave * f_vac;
                accum_rgb += (1.0 - accum_a) * s_col * dt * 1.5;
                accum_a += (1.0 - accum_a) * min(s_a, 0.28);
            }
        }

        // 2b. Z01 Exosphere (Hydrogen Halo, Helium Band, Jeans Escape, UV Geocorona, HEO Orbit)
        if (z_idx == 1u && r > limb_r && r < 0.94) {
            let f_h = doc_feat_focus(inst, 0u);
            let f_he = doc_feat_focus(inst, 1u);
            let f_esc = doc_feat_focus(inst, 2u);
            let f_geo = doc_feat_focus(inst, 3u);
            let f_heo = doc_feat_focus(inst, 4u);
            let h_halo = exp(-pow((alt - 0.14) * 18.0, 2.0)) * 0.55;
            let he_band = exp(-pow((alt - 0.24) * 20.0, 2.0)) * 0.72;
            let escape_ray = pow(max(0.0, sin(atan2(p.z, p.x) * 10.0) * cos(p.y * 8.0 - alt * 9.0 + time * 1.6)), 10.0) * smoothstep(0.10, 0.38, alt) * smoothstep(0.94, 0.65, r);
            let uv_geo = smoothstep(0.15, 0.75, p.x / r) * exp(-pow((alt - 0.48) * 18.0, 2.0));
            let p_heo = doc_rot_x(p, 0.85);
            let heo_ring = exp(-pow((alt - 0.58) * 26.0, 2.0)) * exp(-pow(p_heo.y * 30.0, 2.0));
            let e_a = (h_halo * 0.45 + he_band * 0.55 + escape_ray * 0.75 + uv_geo * 0.55 + heo_ring * 0.9) * dt * 1.5;
            if (e_a > 0.012) {
                let e_col = vec3<f32>(0.32, 0.82, 1.48) * h_halo * f_h
                          + vec3<f32>(1.32, 0.96, 0.42) * he_band * f_he
                          + vec3<f32>(0.45, 1.35, 1.48) * escape_ray * 0.95 * f_esc
                          + vec3<f32>(0.92, 0.42, 1.55) * uv_geo * 0.85 * f_geo
                          + vec3<f32>(1.25, 1.25, 1.48) * heo_ring * 0.92 * f_heo;
                accum_rgb += (1.0 - accum_a) * e_col * dt * 1.5;
                accum_a += (1.0 - accum_a) * min(e_a, 0.28);
            }
        }

        // 2c. Z02 Thermosphere (Sunlit Ionosphere, 51.6-deg ISS + Solar Arrays, Atomic O2->2O, HF Radio Bounce)
        if (z_idx == 2u && r > limb_r && r < 0.94) {
            let f_ion = doc_feat_focus(inst, 1u);
            let f_iss = doc_feat_focus(inst, 2u);
            let f_o = doc_feat_focus(inst, 3u);
            let f_rad = doc_feat_focus(inst, 4u);
            let ion_shell = exp(-pow((alt - 0.42) * 18.0, 2.0)) * smoothstep(-0.05, 0.75, p.x / r);
            let p_iss = doc_rot_z(p, -0.90); // 51.6 deg inclination
            let iss_ring = exp(-pow((alt - 0.28) * 28.0, 2.0)) * exp(-pow(p_iss.y * 34.0, 2.0));
            let iss_pos = doc_rot_z(vec3<f32>(cos(time * 0.45), 0.0, sin(time * 0.45)), 0.90) * (limb_r + shell_w * 0.28);
            let iss_body = exp(-length(p - iss_pos) * 32.0);
            let o_band = exp(-pow((alt - 0.16) * 22.0, 2.0));
            let radio_hop = pow(0.5 + 0.5 * sin(atan2(p.z, p.x) * 12.0 - time * 2.5), 12.0) * exp(-pow((alt - (0.24 + 0.14 * sin(atan2(p.z, p.x) * 6.0))) * 22.0, 2.0));
            let t_a = (ion_shell * 0.5 + iss_ring * 0.8 + iss_body + o_band * 0.6 + radio_hop * 0.85) * dt * 1.65;
            if (t_a > 0.012) {
                let t_col = vec3<f32>(0.36, 0.85, 1.52) * ion_shell * 0.75 * f_ion
                          + (vec3<f32>(0.95, 0.78, 0.28) * iss_ring * 0.72 + vec3<f32>(1.55, 1.35, 0.65) * iss_body * 1.45) * f_iss
                          + vec3<f32>(0.12, 1.48, 0.65) * o_band * 0.78 * f_o
                          + vec3<f32>(0.42, 1.28, 1.55) * radio_hop * 1.1 * f_rad;
                accum_rgb += (1.0 - accum_a) * t_col * dt * 1.65;
                accum_a += (1.0 - accum_a) * min(t_a, 0.30);
            }
        }

        // 2d. Z03 Mesosphere (Meteor Ablation Trails, Noctilucent Clouds, Mesopause -90C Ring, Sounding Rocket, Braking)
        if (z_idx == 3u && r > limb_r && r < 0.94) {
            let f_met = doc_feat_focus(inst, 0u);
            let f_nlc = doc_feat_focus(inst, 1u);
            let f_mp = doc_feat_focus(inst, 2u);
            let f_rkt = doc_feat_focus(inst, 3u);
            let f_brk = doc_feat_focus(inst, 4u);
            let met_d = abs(p.y - 0.24 - 0.12 * sin(atan2(p.z, p.x) * 3.0)) + abs(alt - 0.26) * 0.85;
            let meteor = exp(-pow(met_d * 24.0, 2.0)) * (0.4 + 0.6 * sin(atan2(p.z, p.x) * 5.0 - time * 2.2));
            let nlc = exp(-pow((abs(p.y / r) - 0.72) * 14.0, 2.0)) * exp(-pow((alt - 0.24) * 18.0, 2.0)) * smoothstep(0.05, 0.6, snoise3(p * 12.0));
            let mesopause = exp(-pow((alt - 0.40) * 24.0, 2.0));
            let rkt_plume = exp(-pow((alt - 0.28) * 18.0, 2.0)) * exp(-pow((p.x - 0.35) * 14.0, 2.0)) * smoothstep(0.05, 0.6, p.y);
            let brake_arc = pow(max(0.0, snoise3(p * 15.0 + vec3<f32>(time * 1.6, 0.0, 0.0))), 10.0) * exp(-pow((alt - 0.18) * 16.0, 2.0));
            let m_a = (meteor * 0.85 + nlc * 0.8 + mesopause * 0.6 + rkt_plume * 0.9 + brake_arc * 0.75) * dt * 1.75;
            if (m_a > 0.012) {
                let m_col = vec3<f32>(1.55, 0.88, 0.24) * meteor * 1.15 * f_met
                          + vec3<f32>(0.35, 1.18, 1.62) * nlc * 1.05 * f_nlc
                          + vec3<f32>(0.55, 1.38, 1.58) * mesopause * 0.82 * f_mp
                          + vec3<f32>(1.52, 0.62, 0.22) * rkt_plume * 1.25 * f_rkt
                          + vec3<f32>(1.38, 1.02, 0.48) * brake_arc * 0.95 * f_brk;
                accum_rgb += (1.0 - accum_a) * m_col * dt * 1.75;
                accum_a += (1.0 - accum_a) * min(m_a, 0.32);
            }
        }

        // 2e. Z04 Stratosphere (Ozone Shield, Weather Balloon, Supersonic Contrails, Inversion Strata, UV Quenching)
        if (z_idx == 4u && r > limb_r && r < 0.94) {
            let f_oz = doc_feat_focus(inst, 0u);
            let f_bal = doc_feat_focus(inst, 1u);
            let f_jet = doc_feat_focus(inst, 2u);
            let f_inv = doc_feat_focus(inst, 3u);
            let f_uv = doc_feat_focus(inst, 4u);
            let oz_ring = exp(-pow((alt - 0.22) * 20.0, 2.0));
            let bal = exp(-length(p - normalize(vec3<f32>(0.25, 0.52, 0.82)) * (limb_r + shell_w * (0.28 + 0.04 * sin(time * 0.6)))) * 28.0);
            let jet_ring = exp(-pow((alt - 0.12) * 28.0, 2.0)) * exp(-pow((p.y / r - 0.22) * 16.0, 2.0));
            let inv_strata = exp(-pow((alt - 0.34) * 22.0, 2.0)) * pow(0.5 + 0.5 * sin(p.y * 28.0), 4.0) * 0.55;
            let uv_rays = exp(-pow((alt - 0.44) * 16.0, 2.0)) * pow(max(0.0, sin(p.y * 22.0 + time * 1.5)), 10.0) * smoothstep(0.0, 0.65, p.x / r);
            let st_a = (oz_ring * 0.65 + bal + jet_ring * 0.75 + inv_strata * 0.55 + uv_rays * 0.7) * dt * 1.70;
            if (st_a > 0.012) {
                let st_col = vec3<f32>(0.42, 0.26, 1.55) * oz_ring * 0.88 * f_oz
                           + vec3<f32>(1.48, 1.48, 1.55) * bal * 1.45 * f_bal
                           + vec3<f32>(1.35, 1.25, 1.05) * jet_ring * 0.95 * f_jet
                           + vec3<f32>(0.45, 0.88, 1.32) * inv_strata * f_inv
                           + vec3<f32>(0.88, 0.38, 1.55) * uv_rays * 0.95 * f_uv;
                accum_rgb += (1.0 - accum_a) * st_col * dt * 1.70;
                accum_a += (1.0 - accum_a) * min(st_a, 0.32);
            }
        }

        // 3. Dipole Magnetosphere Van Allen Flux Arcs (Z00 Space, Z12 Geodynamo Coupling, Exp 04 Earth)
        if (show_magnetosphere && r > (limb_r + 0.02) && r < 0.94) {
            let sin2_t = 1.0 - (p.y * p.y) / (r * r);
            let r_eq_norm = r / max(limb_r, 0.4);
            let l_shell = r_eq_norm / max(sin2_t, 0.08);
            let flux_ring = exp(-abs(l_shell - 1.22) * 32.0) + exp(-abs(l_shell - 1.48) * 34.0) * 0.68;
            let phi_ribbons = exp(-abs(sin(atan2(p.z, p.x) * 4.0 + time * 0.08)) * 14.0);
            let rayleigh_rim = select(0.0, exp(-abs(r - (limb_r + 0.04)) * 42.0) * 0.28 * doc_feat_focus(inst, 1u), is_exp04_earth);
            let mag_a = flux_ring * phi_ribbons * mag_gain * dt * 1.5;
            if (mag_a > 0.010 || rayleigh_rim > 0.008) {
                let mag_rgb = vec3<f32>(0.24, 0.72, 1.85) * mag_a + vec3<f32>(0.22, 0.56, 1.28) * rayleigh_rim * dt * 2.2;
                accum_rgb += (1.0 - accum_a) * mag_rgb;
                accum_a += (1.0 - accum_a) * min((mag_a * 0.55) + rayleigh_rim * dt * 0.8, 0.24);
            }
        }
    }
    return vec4<f32>(accum_rgb, accum_a);
}

fn shade_doc_milky_way_stage(
    vid: u32,
    p_in: vec3<f32>,
    r_in: f32,
    inst: HeroVolumeInstance,
    time: f32,
) -> vec4<f32> {
    let is_comp = vid >= 3660u && vid <= 3671u;
    var mw_z: u32 = 0u;
    if (vid >= 1500u && vid <= 1512u) {
        mw_z = vid - 1500u;
    } else if (is_comp) {
        mw_z = vid - 3660u;
    } else if (vid == 20u) {
        mw_z = 1u;
    } else if (vid == 21u) {
        mw_z = 8u;
    } else {
        mw_z = 6u;
    }

    let p = select(p_in, doc_rot_y(p_in, 0.42 + f32(doc_submode(inst)) * 0.14), is_comp);
    let r = r_in;
    let f0 = doc_solar_feat_gate(inst, 0u);
    let f1 = doc_solar_feat_gate(inst, 1u);
    let f2 = doc_solar_feat_gate(inst, 2u);
    let f3 = doc_solar_feat_gate(inst, 3u);
    let f4 = doc_solar_feat_gate(inst, 4u);

    var neb_col = mix(inst.color_a.rgb, inst.color_b.rgb, 0.5);
    var dens: f32 = 0.0;

    if (mw_z == 0u) {
        // Z00 Frontier (~40 AU): Pluto-Charon node (F0), Kuiper torus (F1), pinpoint Sun flare (F2), 17-deg orbit (F3), outer fade (F4)
        let r_xz = length(p.xz);
        let p_inc = doc_rot_x(p, 0.30);
        let sun_pin = exp(-r * 22.0) + pow(max(0.0, exp(-min(abs(p.x), abs(p.y)) * 42.0) * exp(-r * 6.0)), 1.5) * (0.25 + 1.15 * f2);
        let pluto = exp(-length(p - vec3<f32>(0.34, 0.11, 0.0)) * 18.0) * (0.35 + 1.25 * f0);
        let k_ice = pow(max(0.0, snoise3(p * 13.0)), 4.0) * smoothstep(0.28, 0.46, r_xz) * (1.0 - smoothstep(0.68, 0.86, r_xz)) * exp(-abs(p.y) * 16.0);
        let kuiper_band = (exp(-abs(p.y) * 18.0) * smoothstep(0.32, 0.48, r_xz) * (1.0 - smoothstep(0.64, 0.82, r_xz)) * 0.35 + k_ice * 1.45) * (0.45 + 1.15 * f1);
        let orb_arc = exp(-abs(length(p_inc.xz) - 0.56) * 34.0) * exp(-abs(p_inc.y) * 26.0) * (0.25 + 1.25 * f3);
        let outer_void = exp(-abs(r - 0.82) * 14.0) * (0.12 + 0.85 * f4);
        dens = (sun_pin + pluto + kuiper_band + orb_arc + outer_void) * (1.0 - smoothstep(0.76, 0.93, r));
        neb_col = mix(vec3<f32>(0.42, 0.78, 1.38), vec3<f32>(1.65, 1.22, 0.56), clamp(sun_pin + pluto * 0.65, 0.0, 1.0))
            + vec3<f32>(0.24, 0.48, 1.12) * outer_void * f4;
    } else if (mw_z == 1u) {
        // Z01 Heliosphere: solar wind (F0), termination shock (F1), turbulent heliosheath (F2), bow heliopause (F3), Voyager arcs (F4)
        let sun_pos = vec3<f32>(0.20, 0.0, 0.0);
        let ps = p - sun_pos;
        let rs = length(ps);
        let az_yz = atan2(ps.z, ps.y);
        let wind_rays = pow(max(0.0, sin(az_yz * 8.0 + time * 1.4)), 6.0) * exp(-abs(ps.y) * 4.0 + 0.4) * smoothstep(0.04, 0.14, rs) * (1.0 - smoothstep(0.24, 0.36, rs));
        let term_shock = exp(-abs(rs - 0.25) * 36.0) * (0.35 + 1.25 * f1);
        let bow_r = sqrt(max(0.0, (0.48 - p.x) * 0.55));
        let bow = exp(-abs(length(vec2<f32>(p.y, p.z)) - bow_r) * 26.0) * smoothstep(0.50, -0.75, p.x);
        let sheath_turb = pow(0.5 + 0.5 * fbm3(p * 7.5 + curl_noise3(p * 3.0) * 0.4, 3), 2.2)
            * smoothstep(0.23, 0.30, rs) * smoothstep(0.02, 0.12, bow_r - length(vec2<f32>(p.y, p.z)));
        let v1_arc = exp(-abs(p.y - (0.14 + max(0.32 - p.x, 0.0) * 0.42)) * 42.0) * exp(-abs(p.z) * 24.0) * smoothstep(0.30, -0.58, p.x);
        let v2_arc = exp(-abs(p.y + (0.12 + max(0.30 - p.x, 0.0) * 0.48)) * 42.0) * exp(-abs(p.z - 0.06) * 24.0) * smoothstep(0.28, -0.56, p.x);
        let voyager = (v1_arc + v2_arc) * (0.15 + 1.45 * f4);
        let sun_spark = exp(-rs * 18.0) * (1.0 + 0.65 * f0);
        dens = bow * (0.75 + 0.95 * f3) + term_shock + sun_spark + wind_rays * (0.25 + 1.15 * f0) + sheath_turb * (0.20 + 0.95 * f2) + voyager;
        neb_col = mix(vec3<f32>(0.22, 0.92, 1.58), vec3<f32>(1.68, 1.12, 0.38), clamp(sun_spark * 0.9 + voyager * 1.15 + sheath_turb * f2 * 0.4, 0.0, 1.0));
    } else if (mw_z == 2u) {
        // Z02 Oort Cloud: comet (F0), ancient ice glints (F1), far orbit ellipse (F2), isotropic spherical shell (F3), passing star (F4)
        let sun_core = exp(-r * 22.0);
        let ice_glint = pow(max(0.0, snoise3(p * 17.5)), 6.5) * smoothstep(0.36, 0.56, r) * (1.0 - smoothstep(0.76, 0.92, r));
        let sph_shell = exp(-abs(r - 0.68) * 18.0) * (0.28 + 0.95 * f3);
        let orb_rot = doc_rot_z(p - vec3<f32>(0.14, 0.0, 0.0), 0.42);
        let far_orb = exp(-abs(length(vec2<f32>(orb_rot.x * 0.72, orb_rot.y * 1.45)) - 0.48) * 38.0) * exp(-abs(orb_rot.z) * 24.0) * (0.25 + 1.35 * f2);
        let com_head = exp(-length(p - vec3<f32>(-0.24, 0.14, 0.0)) * 20.0);
        let tail_t = clamp((-p.x - 0.24) * 2.2, 0.0, 1.0);
        let com_tail = exp(-length(p.yz - vec2<f32>(0.14 + tail_t * 0.18, 0.0)) * 24.0) * smoothstep(-0.20, -0.78, p.x);
        let comet = (com_head + com_tail * 0.85) * (0.25 + 1.35 * f0);
        let pass_star = exp(-length(p - vec3<f32>(0.54, 0.36, 0.10)) * 16.0) * (0.15 + 1.45 * f4);
        let warp_wake = exp(-abs(r - 0.74) * 24.0) * smoothstep(0.2, 0.7, p.x) * f4;
        dens = (sun_core * 0.9 + ice_glint * (1.25 + 1.45 * f1) + sph_shell + far_orb + comet + pass_star + warp_wake) * (1.0 - smoothstep(0.84, 0.94, r));
        neb_col = mix(vec3<f32>(0.36, 0.82, 1.38), vec3<f32>(1.55, 0.86, 0.38), clamp(pass_star * 1.2 + sun_core * 0.8 + far_orb * f2 * 0.5, 0.0, 1.0));
    } else if (mw_z == 3u) {
        // Z03 Interstellar Space: HI/HII gas (F0), dark cosmic dust extinction (F1), cosmic rays (F2), B-field (F3), LIC cloud (F4)
        let gas_n = fbm3(p * 3.8 + vec3<f32>(0.4, -0.2, 0.6), 4);
        let ism_gas = pow(max(0.0, 0.5 + 0.5 * gas_n), 2.2) * (0.35 + 1.15 * f0);
        let dust_trough = exp(-abs(gas_n + 0.12) * 14.0);
        let dark_cut = 1.0 - dust_trough * (0.45 + 0.48 * f1);
        let ir_rim = dust_trough * (0.20 + 1.25 * f1) * 0.65;
        let ray_line = exp(-abs(p.y - 0.55 * p.x) * 42.0) * exp(-abs(p.z) * 22.0)
            + exp(-abs(p.y + 0.38 * p.x - 0.16) * 44.0) * exp(-abs(p.z + 0.08) * 22.0);
        let cosmic_ray = ray_line * (0.20 + 1.35 * f2);
        let b_curl = sin(p.x * 9.0 + curl_noise3(p * 2.5).y * 4.5);
        let b_field = exp(-abs(b_curl) * 8.0) * exp(-abs(p.z) * 5.0) * (0.18 + 1.25 * f3);
        let lic_puff = exp(-length(p - vec3<f32>(-0.10, 0.05, 0.0)) * 6.8) * (0.25 + 1.15 * f4);
        dens = (ism_gas * dark_cut + ir_rim + cosmic_ray + b_field + lic_puff) * (1.0 - smoothstep(0.66, 0.92, r));
        neb_col = mix(vec3<f32>(0.24, 0.78, 1.38), vec3<f32>(1.42, 0.52, 0.88), clamp(ism_gas * 0.6 + cosmic_ray * 0.5, 0.0, 1.0))
            + vec3<f32>(1.25, 0.64, 0.22) * ir_rim + vec3<f32>(0.28, 1.15, 1.35) * lic_puff * 0.55;
    } else if (mw_z == 4u) {
        // Z04 Alpha Centauri: Proxima flaring M-dwarf (F0), Alpha Cen A (F1), Alpha Cen B + orbit (F2), Proxima b (F3), 4.24 ly rings (F4)
        let d_a = length(p - vec3<f32>(-0.20, 0.06, 0.0));
        let d_b = length(p - vec3<f32>(0.16, -0.04, 0.0));
        let p_prox = p - vec3<f32>(0.46, -0.26, 0.08);
        let d_p = length(p_prox);
        let star_a = exp(-d_a * 15.0) * (0.75 + 0.95 * f1);
        let star_b = exp(-d_b * 17.5) * (0.70 + 0.95 * f2);
        let bin_ring = exp(-abs(length(vec2<f32>(p.x * 1.1, p.y * 1.8)) - 0.22) * 34.0) * exp(-abs(p.z) * 20.0) * (0.25 + 0.95 * f2);
        let prox = exp(-d_p * 20.0) * (0.65 + 1.15 * f0);
        let flare = exp(-abs(d_p - 0.10) * 32.0) * max(0.0, snoise3(p * 14.0 + vec3<f32>(time * 0.6))) * f0;
        let exo_orb = exp(-abs(length(p_prox.xz) - 0.14) * 42.0) * exp(-abs(p_prox.y) * 28.0) * (0.15 + 1.35 * f3);
        let exo_dot = exp(-length(p_prox - vec3<f32>(0.14, 0.0, 0.0)) * 34.0) * (0.20 + 1.45 * f3);
        let ly_wave = (exp(-abs(length(p - vec3<f32>(-0.46, 0.34, 0.0)) - 0.36) * 34.0)
            + exp(-abs(length(p - vec3<f32>(-0.46, 0.34, 0.0)) - 0.62) * 30.0)) * (0.12 + 1.35 * f4);
        dens = (star_a + star_b + bin_ring + prox + flare + exo_orb + exo_dot + ly_wave) * (1.0 - smoothstep(0.80, 0.94, r));
        let col_ab = mix(vec3<f32>(1.55, 0.96, 0.42), vec3<f32>(1.72, 1.62, 1.32), clamp(star_a / max(star_a + star_b, 1e-4), 0.0, 1.0));
        let col_prox = vec3<f32>(1.48, 0.32, 0.16);
        neb_col = mix(col_ab, col_prox, clamp((prox + flare) * 1.2, 0.0, 1.0))
            + vec3<f32>(0.24, 1.18, 1.58) * (exo_orb + exo_dot + ly_wave * 0.8);
    } else if (mw_z == 5u) {
        // Z05 Stellar Neighborhood: Sirius A (F0), Barnard's proper-motion streak (F1), Sirius B WD (F2), brown dwarf (F3), binary tethers (F4)
        let ps_a = p - vec3<f32>(-0.26, 0.18, 0.0);
        let sirius_a = exp(-length(ps_a) * 14.0) + exp(-min(abs(ps_a.x), abs(ps_a.y)) * 42.0) * exp(-length(ps_a) * 5.0) * 0.8;
        let barn_p = p - vec3<f32>(0.32, -0.14, 0.04);
        let barn = exp(-length(barn_p) * 20.0);
        let motion_wake = exp(-abs(barn_p.y + 0.0) * 34.0) * exp(-abs(barn_p.z) * 26.0) * smoothstep(-0.28, 0.0, barn_p.x) * step(barn_p.x, 0.04);
        let wd = exp(-length(p - vec3<f32>(-0.10, 0.10, 0.02)) * 28.0);
        let brown_d = exp(-length(p - vec3<f32>(-0.08, -0.28, 0.06)) * 16.0);
        let tether_ab = exp(-abs(p.y + 0.45 * p.x - 0.06) * 44.0) * exp(-abs(p.z) * 26.0) * smoothstep(-0.30, -0.05, p.x) * step(p.x, 0.24);
        dens = (sirius_a * (0.70 + 1.05 * f0) + (barn + motion_wake * 1.2) * (0.60 + 1.25 * f1)
            + wd * (0.55 + 1.45 * f2) + brown_d * (0.45 + 1.35 * f3) + tether_ab * (0.15 + 1.25 * f4)) * (1.0 - smoothstep(0.78, 0.93, r));
        neb_col = vec3<f32>(0.95, 1.32, 1.85) * (sirius_a + wd * 1.4)
            + vec3<f32>(1.45, 0.42, 0.20) * (barn + motion_wake * 0.7)
            + vec3<f32>(0.98, 0.24, 0.78) * brown_d
            + vec3<f32>(1.38, 1.06, 0.42) * tether_ab;
    } else if (mw_z == 6u) {
        // Z06 Local Bubble: ancient SN rings (F0), hot plasma cavity (F1), dense cold rim wall (F2), 3D hourglass map (F3), Sun path (F4)
        let cav_w = snoise3(p * 3.8) * 0.14;
        let shell = exp(-abs(r - 0.68 - cav_w) * 32.0);
        let rim = exp(-abs(sin(atan2(p.z, p.x) * 6.0)) * 8.0);
        let sn_ring = (exp(-abs(r - 0.34) * 36.0) + exp(-abs(r - 0.50) * 34.0)) * (0.18 + 1.35 * f0);
        let plasma = smoothstep(0.62, 0.10, r) * 0.26 * (0.35 + 1.25 * f1);
        let wall = shell * (0.28 + 0.65 * rim) * (0.65 + 1.15 * f2);
        let hourglass = exp(-abs(length(p.xz) - (0.28 + abs(p.y) * 0.58)) * 34.0) * exp(-abs(sin(atan2(p.z, p.x) * 4.0)) * 6.0) * (0.15 + 1.25 * f3);
        let sun_track = exp(-abs(p.y - 0.18 * sin(p.x * 5.0)) * 42.0) * exp(-abs(p.z) * 24.0) * smoothstep(0.45, 0.0, abs(p.x)) * (0.20 + 1.35 * f4)
            + exp(-length(p - vec3<f32>(0.08, 0.06, 0.0)) * 22.0) * (0.45 + 1.05 * f4);
        dens = (wall + sn_ring + plasma + hourglass + sun_track) * (1.0 - smoothstep(0.84, 0.94, r));
        neb_col = mix(vec3<f32>(0.18, 1.15, 1.45), vec3<f32>(0.85, 0.55, 1.65), clamp(rim * 0.6 + plasma * 1.4, 0.0, 1.0))
            + vec3<f32>(1.55, 1.02, 0.36) * (sn_ring * 0.5 + sun_track * 0.9);
    } else if (mw_z == 7u) {
        // Z07 Orion Spur: spiral arm bridge (F0), golden Sun beacon (F1), molecular clouds (F2), open star clusters (F3), 230 Myr rotation (F4)
        let r_xz = length(p.xz);
        let az = atan2(p.z, p.x);
        let sag_arm = exp(-abs(r_xz - 0.42 - 0.12 * sin(az * 2.0)) * 24.0);
        let spur_arc = exp(-abs(p.y) * 14.0) * exp(-abs(r_xz - 0.60 - 0.08 * az) * 28.0) * (0.55 + 1.15 * f0);
        let per_arm = exp(-abs(r_xz - 0.80 - 0.10 * sin(az * 2.0 + 1.0)) * 22.0);
        let sun_spot = exp(-length(p - vec3<f32>(0.06, 0.02, 0.02)) * 20.0) * (0.55 + 1.35 * f1);
        let gmc = pow(max(0.0, fbm3(p * 7.0, 3)), 1.6) * spur_arc * 2.2 * (0.40 + 1.25 * f2);
        let open_cl = pow(max(0.0, snoise3(p * 16.0)), 6.0) * exp(-abs(p.y) * 10.0) * smoothstep(0.25, 0.78, r_xz) * (0.55 + 1.45 * f3);
        let rot_arcs = exp(-abs(r_xz - 0.60) * 32.0) * pow(0.5 + 0.5 * cos(az * 4.0 - time * 1.2), 4.0) * exp(-abs(p.y) * 18.0) * (0.15 + 1.35 * f4);
        dens = (sag_arm * 0.35 + spur_arc * 0.75 + per_arm * 0.35 + sun_spot + gmc + open_cl + rot_arcs) * exp(-abs(p.y) * 4.5) * (1.0 - smoothstep(0.76, 0.93, r));
        neb_col = mix(vec3<f32>(0.28, 0.78, 1.55), vec3<f32>(1.65, 1.18, 0.42), clamp(sun_spot + gmc * 0.45, 0.0, 1.0))
            + vec3<f32>(0.36, 1.05, 1.75) * open_cl * 0.85;
    } else if (mw_z == 8u) {
        // Z08 Stellar Nursery: Orion M42 pillars (F0), protostar HH jets (F1), protoplanetary disk (F2), Pleiades sapphire (F3), dark nebula (F4)
        let p_tilt = doc_rot_z(p, 0.25);
        let c1 = exp(-length(p_tilt.xz - vec2<f32>(-0.22, 0.0)) * 16.0) * smoothstep(0.55, -0.45, p_tilt.y);
        let c2 = exp(-length(p_tilt.xz - vec2<f32>(0.06, 0.12)) * 19.0) * smoothstep(0.42, -0.45, p_tilt.y);
        let c3 = exp(-length(p_tilt.xz - vec2<f32>(0.28, -0.08)) * 22.0) * smoothstep(0.28, -0.45, p_tilt.y);
        let edge_fbm = 0.4 + 0.6 * abs(fbm3(p * 6.5, 3));
        let stars = pow(max(0.0, snoise3(p * 16.0)), 8.0) * 1.6;
        let proto_p = p - vec3<f32>(-0.18, 0.30, 0.0);
        let hh_jets = (exp(-length(proto_p.xz) * 34.0) * exp(-abs(proto_p.y) * 3.8) + exp(-length(proto_p) * 20.0)) * (0.20 + 1.45 * f1);
        let d_pp = doc_rot_x(p - vec3<f32>(0.24, 0.22, 0.0), 0.55);
        let pp_r = length(d_pp.xz);
        let pp_disk = exp(-abs(d_pp.y) * 38.0) * exp(-abs(pp_r - 0.22) * 18.0) * smoothstep(0.02, 0.06, abs(pp_r - 0.18)) * (0.15 + 1.45 * f2);
        let pleiades = pow(max(0.0, snoise3(p * 15.0 + vec3<f32>(2.1))), 6.0) * exp(-length(p - vec3<f32>(0.0, -0.12, 0.0)) * 2.5) * (0.20 + 1.45 * f3);
        let dark_mask = 1.0 - smoothstep(0.18, 0.65, c2) * (0.20 + 0.72 * f4);
        dens = (((c1 + c2 + c3) * edge_fbm * (0.85 + 0.55 * f0) * dark_mask + stars) + hh_jets + pp_disk + pleiades) * (1.0 - smoothstep(0.70, 0.94, r));
        neb_col = mix(vec3<f32>(1.45, 0.42, 0.85), vec3<f32>(0.28, 1.25, 1.65), clamp(p_tilt.y + 0.5, 0.0, 1.0))
            + vec3<f32>(1.65, 0.82, 0.28) * (hh_jets * 0.7 + pp_disk * 0.8) + vec3<f32>(0.36, 0.95, 1.78) * pleiades * 0.8;
    } else if (mw_z == 9u) {
        // Z09 Milky Way Disk: edge-on skyband (F0), central golden bar (F1), dark dust lanes (F2), NFW violet dark matter (F3), O-M stars (F4)
        let gal_r = length(p.xz);
        let gal_ang = atan2(p.z, p.x);
        let bar = exp(-pow(abs(sin(gal_ang - 0.36)) * gal_r * 6.2, 2.0)) * smoothstep(0.56, 0.12, gal_r) * (0.85 + 1.15 * f1);
        let sp = pow(max(0.0, cos(2.0 * gal_ang - 5.2 * log(gal_r + 0.14))), 3.0)
            + 0.45 * pow(max(0.0, cos(4.0 * gal_ang - 6.8 * log(gal_r + 0.14) + 1.1)), 4.0);
        let dust_n = fbm3(vec3<f32>(p.x * 8.0, p.z * 8.0, p.y * 14.0), 3);
        let dust_cut = mix(smoothstep(-0.22, 0.32, dust_n), smoothstep(0.04, 0.42, dust_n), f2);
        let edge_band = exp(-abs(p.y * 0.92 - p.z * 0.38) * 32.0) * smoothstep(0.88, 0.15, r) * 0.55 * (0.12 + 1.35 * f0);
        let dm_halo = (exp(-abs(r - 0.78) * 22.0) + exp(-abs(r - 0.58) * 18.0) * 0.45) * (0.10 + 1.15 * f3);
        let pop_stars = pow(max(0.0, snoise3(p * 19.0)), 7.0) * exp(-abs(p.y) * 16.0) * smoothstep(0.86, 0.10, gal_r) * (0.35 + 1.45 * f4);
        let disk_d = (sp * smoothstep(0.14, 0.36, gal_r) * (1.0 - smoothstep(0.72, 0.90, gal_r)) * 0.72 + bar * 0.65) * dust_cut * exp(-abs(p.y) * 20.0);
        dens = (disk_d + edge_band + dm_halo + pop_stars) * (1.0 - smoothstep(0.82, 0.94, r));
        neb_col = mix(vec3<f32>(0.32, 0.66, 1.55), vec3<f32>(1.65, 1.12, 0.44), clamp(bar * 1.4 + (0.45 - gal_r) * 0.9, 0.0, 1.0))
            + vec3<f32>(0.55, 0.28, 1.38) * dm_halo;
    } else if (mw_z == 10u) {
        // Z10 Galactic Center (Sgr A*): EHT photon ring (F0), nuclear cluster (F1), S2 orbits (F2), mini-spiral hot dust (F3), radio arc (F4)
        let photon_ring = exp(-abs(r - 0.16) * 44.0) * smoothstep(0.07, 0.13, r) * (0.85 + 1.15 * f0);
        let bh_hole = smoothstep(0.08, 0.14, r);
        let nuc_cluster = pow(max(0.0, snoise3(p * 20.0)), 5.0) * exp(-r * 3.8) * bh_hole * (0.45 + 1.45 * f1);
        let s2_a = doc_rot_z(p, 0.62);
        let s2_orb = (exp(-abs(length(vec2<f32>(s2_a.x * 0.85 + 0.08, s2_a.y * 1.55)) - 0.32) * 38.0) * exp(-abs(s2_a.z) * 24.0)
            + exp(-abs(length(vec2<f32>(p.x * 1.35 - 0.06, p.z * 0.88)) - 0.42) * 36.0) * exp(-abs(p.y) * 24.0)) * (0.18 + 1.35 * f2);
        let az = atan2(p.z, p.x);
        let mini_sp = pow(max(0.0, cos(az * 2.0 - r * 14.0)), 3.0) * smoothstep(0.16, 0.30, r) * (1.0 - smoothstep(0.60, 0.84, r)) * exp(-abs(p.y) * 16.0) * (0.45 + 1.15 * f3);
        let radio_fil = exp(-abs(p.x - 0.26 - 0.05 * sin(p.y * 12.0)) * 38.0) * exp(-abs(p.z) * 22.0) * (1.0 - smoothstep(0.55, 0.84, abs(p.y))) * (0.15 + 1.45 * f4);
        dens = (photon_ring + nuc_cluster + s2_orb + mini_sp + radio_fil) * bh_hole * (1.0 - smoothstep(0.76, 0.93, r));
        neb_col = mix(vec3<f32>(1.65, 0.92, 0.32), vec3<f32>(0.38, 1.18, 1.68), clamp((s2_orb + radio_fil) * 0.85, 0.0, 1.0));
    } else if (mw_z == 11u) {
        // Z11 Galactic Halo: globular clusters (F0), Pop II ancient stars (F1), tidal stellar streams (F2), NFW DM halo (F3), satellite galaxies (F4)
        let disk_ref = exp(-abs(p.y) * 26.0) * smoothstep(0.42, 0.14, length(p.xz)) * 0.45;
        let gc1 = exp(-length(p - vec3<f32>(0.36, 0.38, 0.12)) * 22.0);
        let gc2 = exp(-length(p - vec3<f32>(-0.42, -0.32, 0.18)) * 22.0);
        let gc3 = exp(-length(p - vec3<f32>(0.12, -0.48, -0.22)) * 22.0);
        let globs = (gc1 + gc2 + gc3) * (0.55 + 1.25 * f0);
        let pop2 = pow(max(0.0, snoise3(p * 18.0 + vec3<f32>(4.2))), 7.0) * smoothstep(0.24, 0.42, r) * (0.45 + 1.45 * f1);
        let st_rot = doc_rot_x(p, 0.78);
        let stream = exp(-abs(length(st_rot.xz) - 0.64) * 34.0) * exp(-abs(st_rot.y) * 24.0) * (0.25 + 1.35 * f2);
        let dm_shell = exp(-abs(r - 0.82) * 18.0) * (0.18 + 1.15 * f3);
        let lmc = exp(-length((p - vec3<f32>(-0.56, 0.36, 0.16)) * vec3<f32>(1.0, 1.6, 1.2)) * 12.0);
        let smc = exp(-length((p - vec3<f32>(0.54, -0.26, -0.18)) * vec3<f32>(1.4, 1.0, 1.2)) * 14.0);
        let sats = (lmc + smc) * (0.25 + 1.35 * f4);
        dens = (disk_ref + globs + pop2 + stream + dm_shell + sats) * (1.0 - smoothstep(0.84, 0.94, r));
        neb_col = mix(vec3<f32>(1.55, 1.08, 0.44), vec3<f32>(0.52, 0.36, 1.48), clamp(dm_shell * 0.9 + sats * 0.6, 0.0, 1.0))
            + vec3<f32>(0.36, 0.92, 1.48) * stream * 0.75;
    } else {
        // Z12 Galactic Edge (1512): diffuse halo fade (F0), WHIM void wisps (F1), Local Group dwarfs (F2), tilted M31 (F3), Exp 06 bridge (F4)
        let mw_local = p - vec3<f32>(-0.24, -0.06, 0.04);
        let mw_r = length(mw_local.xz);
        let diff_fade = exp(-mw_r * 3.8) * exp(-abs(mw_local.y) * 18.0) * (0.65 + 0.95 * f0);
        let whim = pow(max(0.0, fbm3(p * 4.5, 3)), 2.8) * smoothstep(0.28, 0.56, r) * (0.18 + 0.95 * f1);
        let lg_dwarfs = (exp(-length(p - vec3<f32>(0.08, 0.38, 0.08)) * 22.0)
            + exp(-length(p - vec3<f32>(0.24, -0.34, 0.14)) * 22.0)
            + exp(-length(p - vec3<f32>(-0.46, 0.30, -0.08)) * 22.0)) * (0.25 + 1.35 * f2);
        let m31_p = doc_rot_z(p - vec3<f32>(0.46, 0.22, -0.06), 0.58);
        let m31_r = length(m31_p.xz);
        let m31 = (exp(-length(m31_p) * 16.0) + exp(-abs(m31_p.y) * 32.0) * exp(-m31_r * 8.0) * 0.75) * (0.35 + 1.45 * f3);
        let bridge = exp(-abs(p.y - 0.45 * p.x) * 32.0) * exp(-abs(p.z) * 20.0) * smoothstep(0.10, 0.72, r) * (0.12 + 1.25 * f4);
        dens = (diff_fade + whim + lg_dwarfs + m31 + bridge) * (1.0 - smoothstep(0.80, 0.94, r));
        neb_col = mix(vec3<f32>(0.34, 0.78, 1.45), vec3<f32>(1.55, 1.08, 0.48), clamp(diff_fade * 0.8 + m31 * 1.15, 0.0, 1.0));
    }

    return vec4<f32>(neb_col, dens);
}

fn shade_doc_local_group_stage(
    vid: u32,
    p: vec3<f32>,
    r: f32,
    inst: HeroVolumeInstance,
    time: f32,
) -> vec4<f32> {
    if (r > 0.76) {
        return vec4<f32>(0.0);
    }
    let is_comp = vid >= 3680u && vid <= 3688u;
    let z_idx = select(
        select(clamp(doc_zone(inst), 0u, 8u), vid - 1600u, vid >= 1600u && vid <= 1608u),
        vid - 3680u,
        is_comp
    );
    let f0 = doc_feat_focus(inst, 0u);
    let f1 = doc_feat_focus(inst, 1u);
    let f2 = doc_feat_focus(inst, 2u);
    let f3 = doc_feat_focus(inst, 3u);
    let f4 = doc_feat_focus(inst, 4u);
    let g0 = doc_solar_feat_gate(inst, 0u);
    let g1 = doc_solar_feat_gate(inst, 1u);
    let g2 = doc_solar_feat_gate(inst, 2u);
    let g3 = doc_solar_feat_gate(inst, 3u);
    let g4 = doc_solar_feat_gate(inst, 4u);

    var dens: f32 = 0.0;
    var neb_col = mix(inst.color_a.rgb, inst.color_b.rgb, 0.5);
    let env_outer = (1.0 - smoothstep(0.48, 0.72, r)) * 0.76;

    if (is_comp) {
        let az = atan2(p.z, p.x);
        let core = exp(-r * 13.0) * (0.65 + 0.55 * g0);
        let shell_a = exp(-abs(r - (0.34 + 0.05 * f32(z_idx % 3u))) * 34.0) * (0.35 + 0.95 * (g1 + g3));
        let spiral_w = exp(-abs(p.y) * 24.0) * pow(max(0.0, cos(az * 2.0 - 4.2 * log(length(p.xz) + 0.08) + f32(z_idx))), 3.5) * exp(-length(p.xz) * 4.5) * (0.45 + 0.85 * (g0 + g2));
        let stream_arc = exp(-abs(length(p.xy) - 0.46) * 32.0) * exp(-abs(p.z) * 18.0) * (0.28 + 1.05 * g4);
        dens = (core + shell_a + spiral_w + stream_arc) * env_outer;
        neb_col = mix(vec3<f32>(0.36, 0.76, 1.42), vec3<f32>(1.42, 1.02, 0.46), clamp(core + g1 * 0.55, 0.0, 1.0))
            + vec3<f32>(0.92, 0.36, 1.25) * (g2 + g3) * 0.35;
        return vec4<f32>(min(neb_col, vec3<f32>(1.28)), min(dens, 1.35));
    }

    // Shared Local Group reference galaxies (MW q1, M31 q2, M33 q3, LMC q4, SMC q5)
    let q1 = p - vec3<f32>(-0.34, -0.09, 0.06);
    let r1 = length(q1.xz);
    let mw_bulge = exp(-length(q1) * 18.0);
    let mw_arms = exp(-abs(q1.y) * 30.0) * pow(max(0.0, cos(2.0 * atan2(q1.z, q1.x) - 3.6 * log(r1 + 0.05))), 4.0) * exp(-r1 * 5.0);
    let q2 = doc_rot_z(p - vec3<f32>(0.35, 0.12, -0.05), 0.58);
    let r2 = length(q2.xz);
    let m31_bulge = exp(-length(q2) * 19.0);
    let m31_arms = exp(-abs(q2.y) * 32.0) * pow(max(0.0, cos(2.0 * atan2(q2.z, q2.x) + 3.8 * log(r2 + 0.05))), 4.0) * exp(-r2 * 5.2);
    let q3 = p - vec3<f32>(0.22, -0.26, 0.16);
    let m33_base = exp(-length(q3) * 22.0) * 0.80 + exp(-abs(q3.y) * 34.0) * exp(-length(q3.xz) * 11.0) * 0.45;

    if (z_idx == 0u) {
        // Z00 Galactic Edge (1600): MW barred spiral (F0), diffuse boundary fade (F1), galactic exit track (F2), Local Group sister frame (F3), Mly scale ticks (F4)
        let mw_c = p - vec3<f32>(-0.18, -0.04, 0.02);
        let rc = length(mw_c.xz);
        let az = atan2(mw_c.z, mw_c.x);
        let bar = exp(-pow(abs(sin(az - 0.35)) * rc * 7.5, 2.0)) * exp(-rc * 4.8) * exp(-abs(mw_c.y) * 28.0);
        let sp = exp(-abs(mw_c.y) * 28.0) * (0.45 + 0.55 * pow(max(0.0, cos(2.0 * az - 4.4 * log(rc + 0.06))), 3.4)) * exp(-rc * 4.4);
        let mw_focus = (bar * 1.15 + sp * 0.85) * (0.45 + 1.05 * f0);
        let diff_halo = exp(-abs(length(mw_c) - 0.28) * 18.0) * (0.18 + 1.15 * f1);
        let exit_cone = exp(-length(p.yz - vec2<f32>(0.12 * p.x, -0.04 * p.x)) * 24.0) * smoothstep(-0.18, 0.46, p.x) * (0.15 + 1.25 * f2);
        let sisters = ((m31_bulge + m31_arms * 0.8) + m33_base * 0.9) * (0.28 + 1.25 * f3);
        let scale_ticks = exp(-abs(p.y + 0.24) * 36.0) * exp(-abs(p.z) * 28.0) * pow(max(0.0, cos(p.x * 28.0)), 8.0) * smoothstep(0.56, 0.05, abs(p.x)) * (0.14 + 1.35 * f4);
        dens = (mw_focus + diff_halo + exit_cone + sisters + scale_ticks) * env_outer;
        neb_col = mix(vec3<f32>(0.34, 0.72, 1.42), vec3<f32>(1.45, 1.04, 0.46), clamp(bar * 1.3 + sisters * 0.65 + g0 * 0.35, 0.0, 1.0))
            + vec3<f32>(0.28, 1.18, 1.35) * (exit_cone * g2 + scale_ticks * g4) * 0.65
            + vec3<f32>(0.68, 0.42, 1.38) * diff_halo * g1 * 0.60;
    } else if (z_idx == 1u) {
        // Z01 Outer Halo (1601): NFW dark matter halo (F0), tidal stellar stream (F1), globular clusters (F2), X-ray galactic corona (F3), runaway star (F4)
        let dm_nfw = (exp(-abs(length(q1) - 0.32) * 19.0) + exp(-abs(length(q2) - 0.30) * 19.0) * 0.85) * (0.22 + 1.25 * f0);
        let stream_p = doc_rot_x(p - vec3<f32>(-0.16, 0.02, 0.0), 0.72);
        let t_stream = exp(-abs(length(stream_p.xz) - 0.38) * 34.0) * exp(-abs(stream_p.y) * 24.0) * (0.22 + 1.35 * f1);
        let gc1 = exp(-length(p - vec3<f32>(-0.42, 0.24, 0.14)) * 28.0);
        let gc2 = exp(-length(p - vec3<f32>(-0.12, -0.34, -0.16)) * 28.0);
        let gc3 = exp(-length(p - vec3<f32>(0.18, 0.32, 0.12)) * 28.0);
        let gc4 = exp(-length(p - vec3<f32>(-0.26, 0.36, -0.18)) * 28.0);
        let globs = (gc1 + gc2 + gc3 + gc4) * (0.28 + 1.45 * f2);
        let corona = exp(-length(q1) * 4.8) * smoothstep(0.08, 0.24, length(q1)) * (0.16 + 0.95 * f3);
        let run_d = length(p.yz - vec2<f32>(0.42 * p.x + 0.10, -0.25 * p.x + 0.05));
        let runaway = exp(-run_d * 34.0) * smoothstep(-0.32, 0.52, p.x) * (0.55 + 0.45 * sin(p.x * 26.0 - time * 3.0)) * (0.16 + 1.40 * f4);
        let disk_ref = (mw_bulge + mw_arms * 0.65 + m31_bulge * 0.65) * 0.52;
        dens = (disk_ref + dm_nfw + t_stream + globs + corona + runaway) * env_outer;
        neb_col = mix(vec3<f32>(0.46, 0.34, 1.38), vec3<f32>(1.48, 1.06, 0.44), clamp(globs * 1.2 + t_stream * 0.85, 0.0, 1.0))
            + vec3<f32>(0.26, 0.98, 1.42) * (corona * g3 + runaway * g4) * 0.72;
    } else if (z_idx == 2u) {
        // Z02 Dwarf Galaxies (1602): dwarf swarm (F0), Sagittarius Dwarf & polar stream (F1), Fornax spheroidal (F2), ultra-faint reticle (F3), tidal shear tails (F4)
        let d1 = exp(-length(p - vec3<f32>(-0.46, 0.22, -0.12)) * 24.0);
        let d2 = exp(-length(p - vec3<f32>(-0.14, -0.32, 0.18)) * 24.0);
        let d3 = exp(-length(p - vec3<f32>(0.44, -0.16, 0.14)) * 24.0);
        let d4 = exp(-length(p - vec3<f32>(0.24, 0.34, -0.16)) * 24.0);
        let swarm = (d1 + d2 + d3 + d4) * (0.32 + 1.30 * f0);
        let sgr_core = exp(-length((p - vec3<f32>(-0.22, 0.18, 0.06)) * vec3<f32>(1.8, 0.7, 1.4)) * 20.0);
        let sgr_loop = exp(-abs(length((p - vec3<f32>(-0.30, 0.0, 0.04)).xy) - 0.24) * 34.0) * exp(-abs(p.z - 0.04) * 24.0);
        let sgr = (sgr_core + sgr_loop * 0.78) * (0.24 + 1.38 * f1);
        let fornax_p = p - vec3<f32>(0.06, 0.26, -0.14);
        let fornax = exp(-length(fornax_p * vec3<f32>(1.0, 1.35, 1.1)) * 18.0) * (0.26 + 1.42 * f2);
        let ufd_p = p - vec3<f32>(-0.04, -0.26, -0.12);
        let ufd_r = length(ufd_p);
        let ufd = (exp(-ufd_r * 26.0) * 0.65 + exp(-abs(ufd_r - 0.11) * 44.0) * (0.55 + 0.45 * cos(atan2(ufd_p.z, ufd_p.x) * 4.0))) * (0.20 + 1.42 * f3);
        let tidal_tail = exp(-abs(p.y - 0.32 * sin(p.x * 6.5)) * 28.0) * exp(-abs(p.z) * 20.0) * smoothstep(0.55, 0.05, abs(p.x)) * (0.18 + 1.35 * f4);
        let hosts = (mw_bulge + mw_arms * 0.55 + m31_bulge * 0.55) * 0.45;
        dens = (hosts + swarm + sgr + fornax + ufd + tidal_tail) * env_outer;
        neb_col = mix(vec3<f32>(0.36, 0.78, 1.42), vec3<f32>(1.48, 0.96, 0.40), clamp(fornax * 1.3 + sgr_core * 0.9, 0.0, 1.0))
            + vec3<f32>(0.28, 1.26, 1.38) * ufd * g3 * 0.75
            + vec3<f32>(1.28, 0.48, 1.18) * tidal_tail * g4 * 0.65;
    } else if (z_idx == 3u) {
        // Z03 Intergalactic Space (1603): ionized WHIM gas (F0), shared DM bridge (F1), 2.54-Mly traveling photon (F2), magnetic field lines (F3), apparent void (F4)
        let axis_d = length(p.yz - vec2<f32>(0.30 * p.x + 0.02, -0.16 * p.x));
        let span = smoothstep(-0.36, -0.18, p.x) * (1.0 - smoothstep(0.18, 0.36, p.x));
        let whim = exp(-axis_d * 11.0) * (0.45 + 0.55 * fbm3(p * 8.5, 3)) * span * (0.26 + 1.30 * f0);
        let dm_bridge = exp(-axis_d * 6.5) * exp(-abs(r - 0.34) * 12.0) * (0.20 + 1.25 * f1);
        let ph_x = -0.18 + 0.36 * fract(time * 0.22);
        let ph_pos = vec3<f32>(ph_x, 0.30 * ph_x + 0.02, -0.16 * ph_x);
        let ph_d = length(p - ph_pos);
        let photon_pkt = (exp(-ph_d * 26.0) + exp(-abs(ph_d - 0.09) * 42.0) * 0.75 + exp(-axis_d * 34.0) * span * 0.45) * (0.20 + 1.45 * f2);
        let mag_lines = exp(-abs(sin((p.y - 0.30 * p.x) * 28.0 + sin(p.x * 8.0))) * 14.0) * exp(-axis_d * 10.0) * span * (0.18 + 1.35 * f3);
        let void_grid = exp(-abs(r - 0.54) * 34.0) * pow(max(0.0, cos(atan2(p.z, p.x) * 6.0)), 6.0) * (0.15 + 1.28 * f4);
        let void_atten = select(1.0, 0.58, doc_feat(inst) == 4u);
        let anchors = (mw_bulge + mw_arms * 0.6 + m31_bulge + m31_arms * 0.6) * 0.52 * void_atten;
        dens = (anchors + whim * void_atten + dm_bridge + photon_pkt + mag_lines + void_grid) * env_outer;
        neb_col = mix(vec3<f32>(0.26, 0.82, 1.42), vec3<f32>(0.56, 0.34, 1.44), clamp(dm_bridge * 1.2 + g1 * 0.4, 0.0, 1.0))
            + vec3<f32>(1.52, 1.16, 0.38) * photon_pkt * 0.85
            + vec3<f32>(0.24, 1.28, 1.08) * mag_lines * 0.70;
    } else if (z_idx == 4u) {
        // Z04 Magellanic Clouds (1604): LMC barred irregular (F0), SMC companion (F1), 200-kpc HI Magellanic Stream (F2), Tarantula 30 Doradus starburst (F3), tidal shear (F4)
        let lmc_p = p - vec3<f32>(0.08, -0.06, 0.02);
        let lmc_bar = exp(-length(lmc_p * vec3<f32>(0.85, 2.2, 1.4)) * 12.0);
        let lmc_arm = exp(-abs(lmc_p.y) * 24.0) * exp(-abs(length(lmc_p.xz) - 0.16) * 22.0) * smoothstep(-0.05, 0.16, lmc_p.x);
        let lmc = (lmc_bar + lmc_arm * 0.75) * (0.38 + 1.25 * f0);
        let smc_p = p - vec3<f32>(0.32, -0.22, 0.10);
        let smc = exp(-length(smc_p * vec3<f32>(1.3, 0.9, 1.5)) * 16.0) * (0.34 + 1.35 * f1);
        let stream_arc = exp(-abs(length((p - vec3<f32>(-0.06, -0.04, 0.0)).xy) - 0.34) * 24.0) * exp(-abs(p.z - 0.04) * 18.0) * (0.55 + 0.45 * fbm3(p * 9.0, 3)) * (0.24 + 1.35 * f2);
        let tarantula = exp(-length(p - vec3<f32>(0.14, -0.03, 0.05)) * 30.0) * (0.32 + 1.55 * f3);
        let bridge_shear = exp(-length((p - mix(vec3<f32>(0.08, -0.06, 0.02), vec3<f32>(0.32, -0.22, 0.10), clamp((p.x - 0.08) / 0.24, 0.0, 1.0)))) * 24.0) * (0.24 + 1.38 * f4);
        let mw_ref = (mw_bulge + mw_arms * 0.65) * 0.48;
        dens = (mw_ref + lmc + smc + stream_arc + tarantula + bridge_shear) * env_outer;
        neb_col = mix(vec3<f32>(0.34, 0.78, 1.44), vec3<f32>(1.42, 0.98, 0.44), clamp(lmc_bar + smc * 0.8, 0.0, 1.0))
            + vec3<f32>(1.48, 0.34, 1.12) * tarantula * 0.92
            + vec3<f32>(0.24, 1.18, 1.46) * (stream_arc * g2 + bridge_shear * g4) * 0.68;
    } else if (z_idx == 5u) {
        // Z05 Triangulum M33 (1605): face-on M33 disk (F0), flocculent 2-arm spiral (F1), NGC 604 giant HII nursery (F2), third-rank triad halo (F3), M31 tether (F4)
        let m33_p = doc_rot_x(p - vec3<f32>(0.04, -0.04, 0.0), 0.32);
        let r33 = length(m33_p.xz);
        let az33 = atan2(m33_p.z, m33_p.x);
        let core33 = exp(-length(m33_p) * 14.0) * (0.45 + 1.15 * f0);
        let arms33 = exp(-abs(m33_p.y) * 28.0) * (0.35 + 0.65 * pow(max(0.0, cos(2.0 * az33 - 4.2 * log(r33 + 0.06))), 3.2)) * exp(-r33 * 3.8) * (0.38 + 1.25 * f1);
        let ngc604 = exp(-length(m33_p - vec3<f32>(0.16, 0.01, 0.11)) * 28.0) * (0.30 + 1.55 * f2);
        let rank_rings = (exp(-abs(r33 - 0.28) * 36.0) + exp(-abs(length(q2) - 0.22) * 32.0) * 0.85 + exp(-abs(length(q1) - 0.20) * 32.0) * 0.75) * (0.16 + 1.35 * f3);
        let m31_tether = exp(-length(p - mix(vec3<f32>(0.04, -0.04, 0.0), vec3<f32>(0.35, 0.12, -0.05), clamp((p.x - 0.04) / 0.31, 0.0, 1.0))) * 22.0) * (0.20 + 1.38 * f4);
        dens = (core33 + arms33 + ngc604 + rank_rings + m31_tether + m31_bulge * 0.42) * env_outer;
        neb_col = mix(vec3<f32>(0.30, 0.76, 1.46), vec3<f32>(1.38, 1.02, 0.48), clamp(core33 * 0.9 + rank_rings * 0.65, 0.0, 1.0))
            + vec3<f32>(1.48, 0.36, 1.16) * ngc604 * 0.95
            + vec3<f32>(0.28, 1.24, 1.38) * m31_tether * g4 * 0.72;
    } else if (z_idx == 6u) {
        // Z06 Andromeda M31 (1606): tilted M31 spiral (F0), golden ancient bulge (F1), dark dust lanes (F2), M32 & M110 satellites (F3), future MW-M31 merger (F4)
        let m31_c = doc_rot_z(p - vec3<f32>(0.06, 0.02, 0.0), 0.56);
        let rm = length(m31_c.xz);
        let azm = atan2(m31_c.z, m31_c.x);
        let bulge_m = exp(-length(m31_c * vec3<f32>(1.0, 1.4, 1.0)) * 12.0) * (0.48 + 1.28 * f1);
        let dust_groove = 1.0 - (0.45 + 0.42 * g2) * (exp(-abs(rm - 0.18) * 38.0) + exp(-abs(rm - 0.30) * 36.0));
        let dust_ring_glow = (exp(-abs(rm - 0.22) * 38.0) + exp(-abs(rm - 0.34) * 36.0)) * exp(-abs(m31_c.y) * 28.0) * (0.22 + 1.35 * f2);
        let spiral_m = exp(-abs(m31_c.y) * 28.0) * (0.35 + 0.65 * pow(max(0.0, cos(2.0 * azm + 4.4 * log(rm + 0.06))), 3.4)) * exp(-rm * 3.4) * max(dust_groove, 0.12) * (0.42 + 1.15 * f0);
        let m32 = exp(-length(p - vec3<f32>(0.22, -0.12, 0.08)) * 28.0);
        let m110 = exp(-length((p - vec3<f32>(-0.14, 0.24, -0.10)) * vec3<f32>(1.4, 0.9, 1.2)) * 24.0);
        let sats_m = (m32 + m110) * (0.26 + 1.45 * f3);
        let merger_arc = exp(-abs(length((p - vec3<f32>(-0.08, 0.0, 0.0)).xy) - 0.28) * 26.0) * exp(-abs(p.z) * 20.0) * (0.18 + 1.38 * f4);
        dens = (bulge_m + spiral_m + dust_ring_glow + sats_m + merger_arc) * env_outer;
        neb_col = mix(vec3<f32>(0.34, 0.68, 1.44), vec3<f32>(1.52, 1.06, 0.44), clamp(bulge_m * 1.1 + sats_m * 0.9, 0.0, 1.0))
            + vec3<f32>(1.36, 0.56, 0.24) * dust_ring_glow * g2 * 0.75
            + vec3<f32>(0.92, 0.38, 1.42) * merger_arc * g4 * 0.72;
    } else if (z_idx == 7u) {
        // Z07 Local Group Center (1607): barycenter reticle (F0), two subgroup hulls (F1), shared gravity vectors (F2), collective Virgo flow (F3), potential saddle contours (F4)
        let bary_p = p - vec3<f32>(0.04, 0.02, 0.0);
        let bary_r = length(bary_p);
        let cross = (exp(-length(bary_p.yz) * 38.0) + exp(-length(bary_p.xz) * 38.0) + exp(-abs(bary_r - 0.11) * 42.0)) * smoothstep(0.24, 0.0, bary_r) * (0.28 + 1.38 * f0);
        let sub_mw = exp(-abs(length(q1) - 0.22) * 32.0);
        let sub_m31 = exp(-abs(length(p - vec3<f32>(0.30, 0.04, 0.02)) - 0.26) * 30.0);
        let subgroups = (sub_mw + sub_m31) * (0.20 + 1.35 * f1);
        let grav_line = exp(-length(p.yz - vec2<f32>(0.30 * p.x + 0.02, -0.16 * p.x)) * 28.0) * smoothstep(0.38, 0.0, abs(p.x)) * (0.55 + 0.45 * cos(p.x * 24.0)) * (0.20 + 1.35 * f2);
        let bulk_flow = exp(-abs(sin((p.y - 0.45 * p.x) * 20.0)) * 18.0) * exp(-abs(p.z) * 16.0) * (0.5 + 0.5 * sin((p.x + p.y) * 18.0 - time * 2.4)) * (0.16 + 1.35 * f3);
        let saddle = (exp(-abs(bary_r - 0.24) * 34.0) + exp(-abs(bary_r - 0.38) * 34.0)) * exp(-abs(p.y) * 18.0) * (0.18 + 1.35 * f4);
        let galaxies = (mw_bulge + mw_arms * 0.65 + m31_bulge + m31_arms * 0.65 + m33_base * 0.65) * 0.52;
        dens = (galaxies + cross + subgroups + grav_line + bulk_flow + saddle) * env_outer;
        neb_col = mix(vec3<f32>(0.34, 0.78, 1.44), vec3<f32>(1.48, 1.08, 0.42), clamp(cross * 1.1 + grav_line * 0.85, 0.0, 1.0))
            + vec3<f32>(0.58, 0.36, 1.42) * (subgroups * g1 + saddle * g4) * 0.72
            + vec3<f32>(0.24, 1.28, 1.12) * bulk_flow * g3 * 0.70;
    } else {
        // Z08 Local Group Limit (1608): diffuse membership fade (F0), zero-velocity turnaround shell (F1), Virgo Supercluster beacon (F2), Laniakea streamlines (F3), outward jump (F4)
        let diff_edge = exp(-r * 3.6) * smoothstep(0.18, 0.52, r) * (0.24 + 1.18 * f0);
        let zv_grid = 0.55 + 0.45 * cos(atan2(p.z, p.x) * 8.0) * cos(p.y * 14.0);
        let zv_shell = exp(-abs(r - 0.56) * 36.0) * zv_grid * (0.28 + 1.38 * f1);
        let virgo_node = exp(-length(p - vec3<f32>(0.48, 0.24, -0.14)) * 22.0) * (0.26 + 1.45 * f2);
        let lan_streams = exp(-abs(sin((p.y - 0.35 * p.x) * 18.0 + sin(p.z * 10.0))) * 16.0) * smoothstep(0.36, 0.68, r) * (0.20 + 1.35 * f3);
        let jump_rings = (exp(-abs(r - 0.42) * 34.0) + exp(-abs(r - 0.66) * 34.0)) * (0.18 + 1.35 * f4);
        let lg_core = (mw_bulge + m31_bulge + m33_base * 0.6) * 0.46;
        dens = (lg_core + diff_edge + zv_shell + virgo_node + lan_streams + jump_rings) * env_outer;
        neb_col = mix(vec3<f32>(0.28, 0.84, 1.44), vec3<f32>(1.50, 1.04, 0.40), clamp(virgo_node * 1.3 + lg_core, 0.0, 1.0))
            + vec3<f32>(0.24, 1.28, 1.24) * zv_shell * g1 * 0.72
            + vec3<f32>(0.88, 0.42, 1.42) * (lan_streams * g3 + jump_rings * g4) * 0.68;
    }

    let out_d = select(0.0, min(dens, 1.38), dens >= 0.045);
    return vec4<f32>(min(neb_col, vec3<f32>(1.28)), out_d);
}

fn shade_doc_laniakea_stage(
    vid: u32,
    p: vec3<f32>,
    r: f32,
    inst: HeroVolumeInstance,
    time: f32,
) -> vec4<f32> {
    if (r > 0.76) {
        return vec4<f32>(0.0);
    }
    let is_comp = vid >= 3700u && vid <= 3709u;
    let z_idx = select(
        select(clamp(doc_zone(inst), 0u, 9u), vid - 1700u, vid >= 1700u && vid <= 1709u),
        vid - 3700u,
        is_comp
    );
    let f0 = doc_feat_focus(inst, 0u);
    let f1 = doc_feat_focus(inst, 1u);
    let f2 = doc_feat_focus(inst, 2u);
    let f3 = doc_feat_focus(inst, 3u);
    let f4 = doc_feat_focus(inst, 4u);
    let g0 = doc_solar_feat_gate(inst, 0u);
    let g1 = doc_solar_feat_gate(inst, 1u);
    let g2 = doc_solar_feat_gate(inst, 2u);
    let g3 = doc_solar_feat_gate(inst, 3u);
    let g4 = doc_solar_feat_gate(inst, 4u);

    var dens: f32 = 0.0;
    var neb_col = mix(inst.color_a.rgb, inst.color_b.rgb, 0.5);
    let env_outer = (1.0 - smoothstep(0.48, 0.72, r)) * 0.76;

    if (is_comp) {
        let az = atan2(p.z, p.x);
        let knot = exp(-r * 13.0) * (0.62 + 0.58 * g0);
        let fil = pow(smoothstep(0.14, 0.0, abs(snoise3(p * 5.2 + vec3<f32>(f32(z_idx) * 0.7)))), 2.2) * (0.45 + 0.85 * (g1 + g2));
        let ring = exp(-abs(r - (0.36 + 0.04 * f32(z_idx % 3u))) * 34.0) * (0.5 + 0.5 * cos(az * 4.0)) * (0.32 + 0.95 * (g3 + g4));
        dens = (knot + fil + ring) * env_outer;
        neb_col = mix(vec3<f32>(0.26, 0.72, 1.44), vec3<f32>(1.48, 0.98, 0.36), clamp(knot + g0 * 0.45, 0.0, 1.0))
            + vec3<f32>(0.36, 1.24, 1.32) * ring * 0.45;
        return vec4<f32>(min(neb_col, vec3<f32>(1.28)), min(dens, 1.35));
    }

    // Shared Laniakea convergence coordinates
    let ga = p - vec3<f32>(0.18, 0.02, 0.0);
    let r_ga = length(ga);
    let dir_ga = normalize(ga + vec3<f32>(1e-4));
    let s1 = abs(snoise3(dir_ga * 4.6 + vec3<f32>(r_ga * 2.4 - time * 0.22)));
    let s2 = abs(snoise3(dir_ga * 4.6 + vec3<f32>(5.2, -3.1, 2.7)));
    let base_threads = pow(smoothstep(0.14, 0.0, length(vec2<f32>(s1, s2))), 2.5) * exp(-r_ga * 1.1);

    if (z_idx == 0u) {
        // Z00 Local Group Limit in Laniakea (1700): LG peripheral knot (F0), diffuse membership halo (F1), turnaround shell (F2), scale jump rings (F3), Laniakea streamlines (F4)
        let lg_p = p - vec3<f32>(-0.34, -0.06, 0.08);
        let lg_r = length(lg_p);
        let lg_knot = exp(-lg_r * 22.0) * (0.38 + 1.35 * f0);
        let diff_mem = exp(-lg_r * 7.5) * smoothstep(0.05, 0.24, lg_r) * (0.24 + 1.25 * f1);
        let zv_surf = exp(-abs(lg_r - 0.20) * 38.0) * (0.55 + 0.45 * cos(atan2(lg_p.z, lg_p.x) * 6.0)) * (0.24 + 1.38 * f2);
        let scale_jump = (exp(-abs(r - 0.30) * 34.0) + exp(-abs(r - 0.52) * 34.0)) * (0.20 + 1.35 * f3);
        let lan_flow = base_threads * (0.36 + 1.32 * f4);
        dens = (lg_knot + diff_mem + zv_surf + scale_jump + lan_flow) * env_outer;
        neb_col = mix(vec3<f32>(0.26, 0.76, 1.44), vec3<f32>(1.50, 1.04, 0.38), clamp(lg_knot * 1.2 + scale_jump * 0.7, 0.0, 1.0))
            + vec3<f32>(0.24, 1.28, 1.22) * zv_surf * g2 * 0.75;
    } else if (z_idx == 1u) {
        // Z01 Local Volume (1701): neighbor galaxies CenA/M81/Sculptor (F0), galaxy group halos (F1), Mpc distance arcs (F2), peculiar velocity vectors (F3), 3D density slice (F4)
        let g_cen = p - vec3<f32>(0.22, 0.16, -0.08);
        let g_m81 = p - vec3<f32>(-0.24, 0.22, 0.12);
        let g_scu = p - vec3<f32>(-0.18, -0.26, -0.10);
        let g_maf = p - vec3<f32>(0.26, -0.18, 0.14);
        let neighbors = (exp(-length(g_cen) * 24.0) + exp(-length(g_m81) * 24.0) + exp(-length(g_scu) * 24.0) + exp(-length(g_maf) * 24.0) + exp(-r * 20.0) * 0.8) * (0.32 + 1.30 * f0);
        let grp_halos = (exp(-abs(length(g_cen) - 0.11) * 38.0) + exp(-abs(length(g_m81) - 0.11) * 38.0) + exp(-abs(length(g_scu) - 0.11) * 38.0)) * (0.22 + 1.38 * f1);
        let dist_arcs = (exp(-abs(length(p.xz) - 0.24) * 38.0) + exp(-abs(length(p.xz) - 0.44) * 38.0)) * exp(-abs(p.y) * 22.0) * (0.22 + 1.35 * f2);
        let pec_vec = exp(-abs(sin((p.y - 0.35 * p.x) * 22.0)) * 18.0) * exp(-abs(p.z) * 16.0) * (0.55 + 0.45 * cos(p.x * 24.0 - time * 2.5)) * (0.20 + 1.35 * f3);
        let map_grid = (exp(-abs(sin(p.x * 16.0)) * 20.0) * exp(-abs(sin(p.z * 16.0)) * 20.0)) * exp(-abs(p.y) * 12.0) * (0.18 + 1.38 * f4);
        dens = (neighbors + grp_halos + dist_arcs + pec_vec + map_grid) * env_outer;
        neb_col = mix(vec3<f32>(0.32, 0.78, 1.44), vec3<f32>(1.48, 1.02, 0.40), clamp(neighbors * 1.1 + dist_arcs * 0.7, 0.0, 1.0))
            + vec3<f32>(0.26, 1.26, 1.14) * (pec_vec * g3 + map_grid * g4) * 0.70;
    } else if (z_idx == 2u) {
        // Z02 Local Sheet (1702): flattened galactic sheet (F0), Council of Giants 12-galaxy ring (F1), Local Plane grid (F2), Local Void cliff (F3), Virgo bridge (F4)
        let y_sh = p.y - 0.05;
        let r_xz = length(p.xz);
        let sheet = exp(-abs(y_sh) * 34.0) * exp(-r_xz * 2.8) * (0.36 + 1.22 * f0);
        let az = atan2(p.z, p.x);
        let council = exp(-abs(y_sh) * 36.0) * exp(-abs(r_xz - 0.34) * 32.0) * (0.35 + 0.65 * pow(max(0.0, cos(az * 6.0)), 6.0)) * (0.30 + 1.42 * f1);
        let plane_grid = exp(-abs(y_sh) * 38.0) * (exp(-abs(sin(p.x * 18.0)) * 18.0) + exp(-abs(sin(p.z * 18.0)) * 18.0)) * 0.55 * (0.20 + 1.35 * f2);
        let void_rim = exp(-abs(p.y + 0.14) * 32.0) * exp(-abs(r_xz - 0.28) * 20.0) * (0.22 + 1.38 * f3);
        let virgo_bridge = exp(-abs(y_sh) * 32.0) * exp(-abs(p.z + 0.25 * p.x) * 28.0) * smoothstep(-0.05, 0.52, p.x) * (0.24 + 1.40 * f4);
        let void_cut = select(1.0, smoothstep(-0.18, 0.02, p.y), doc_feat(inst) == 3u);
        dens = ((sheet + council + plane_grid + virgo_bridge) * void_cut + void_rim) * env_outer;
        neb_col = mix(vec3<f32>(0.30, 0.78, 1.44), vec3<f32>(1.50, 1.06, 0.42), clamp(council * 1.2 + virgo_bridge * 0.85, 0.0, 1.0))
            + vec3<f32>(0.68, 0.36, 1.42) * void_rim * g3 * 0.75;
    } else if (z_idx == 3u) {
        // Z03 Virgo Cluster (1703): 2000-galaxy swarm (F0), M87 core & relativistic jet (F1), X-ray intracluster gas (F2), dark-matter potential cage (F3), fast orbits & ram stripping (F4)
        let vc = p - vec3<f32>(-0.04, 0.02, 0.0);
        let r_vc = length(vc);
        let swarm_knots = exp(-r_vc * 6.5) * (0.45 + 0.55 * pow(max(0.0, snoise3(vc * 16.0)), 2.0)) * (0.34 + 1.25 * f0);
        let m87_core = exp(-r_vc * 22.0);
        let m87_jet = exp(-length(vc.xz - vec2<f32>(0.22 * vc.y, 0.0)) * 36.0) * smoothstep(0.56, 0.0, abs(vc.y));
        let m87 = (m87_core + m87_jet * 1.15) * (0.36 + 1.35 * f1);
        let icm_xray = exp(-r_vc * 4.8) * smoothstep(0.04, 0.22, r_vc) * (0.22 + 1.18 * f2);
        let dm_cage = (exp(-abs(r_vc - 0.26) * 36.0) + exp(-abs(r_vc - 0.44) * 36.0)) * (0.55 + 0.45 * cos(atan2(vc.z, vc.x) * 6.0)) * (0.20 + 1.35 * f3);
        let ram_tails = exp(-abs(vc.z) * 24.0) * exp(-abs(vc.y - 0.16 * sin(vc.x * 10.0)) * 28.0) * smoothstep(-0.38, 0.38, vc.x) * (0.22 + 1.38 * f4);
        dens = (swarm_knots + m87 + icm_xray + dm_cage + ram_tails) * env_outer;
        neb_col = mix(vec3<f32>(0.34, 0.72, 1.46), vec3<f32>(1.52, 1.04, 0.40), clamp(m87_core * 1.3 + swarm_knots * 0.85, 0.0, 1.0))
            + vec3<f32>(0.28, 1.18, 1.52) * (m87_jet * g1 + ram_tails * g4) * 0.72
            + vec3<f32>(0.58, 0.34, 1.44) * (icm_xray * g2 + dm_cage * g3) * 0.65;
    } else if (z_idx == 4u) {
        // Z04 Virgo Supercluster (1704): supercluster lobe (F0), Virgo branch arm (F1), cluster bead chain (F2), historical vs Laniakea frame (F3), hierarchical substructure (F4)
        let vsc_p = (p - vec3<f32>(-0.14, 0.06, 0.0)) * vec3<f32>(0.85, 1.45, 1.25);
        let lobe = exp(-abs(length(vsc_p) - 0.32) * 28.0) * (0.26 + 1.28 * f0);
        let branch = exp(-length(p.yz - vec2<f32>(0.06 - 0.18 * p.x, 0.08 * p.x)) * 22.0) * smoothstep(-0.46, 0.24, p.x) * (0.28 + 1.32 * f1);
        let chain = exp(-length(p.yz - vec2<f32>(0.06 - 0.18 * p.x, 0.0)) * 28.0) * pow(max(0.0, cos(p.x * 20.0)), 6.0) * smoothstep(0.48, 0.0, abs(p.x)) * (0.24 + 1.40 * f2);
        let hist_ring = (exp(-abs(length(vsc_p) - 0.24) * 36.0) + exp(-abs(r - 0.56) * 34.0)) * (0.20 + 1.35 * f3);
        let sub_tree = base_threads * (0.35 + 1.28 * f4);
        dens = (lobe + branch + chain + hist_ring + sub_tree) * env_outer;
        neb_col = mix(vec3<f32>(0.28, 0.76, 1.44), vec3<f32>(1.48, 1.02, 0.38), clamp(chain * 1.2 + branch * 0.85, 0.0, 1.0))
            + vec3<f32>(0.32, 1.24, 1.28) * hist_ring * g3 * 0.72;
    } else if (z_idx == 5u) {
        // Z05 Galactic Flows (1705): peculiar flow pulses (F0), coherent streamlines (F1), gravitational valley contours (F2), Hubble expansion vs infall (F3), Cosmicflows cone (F4)
        let flow_pulse = base_threads * (0.65 + 0.35 * sin(r_ga * 22.0 - time * 3.0)) * (0.36 + 1.28 * f0);
        let stream_rib = exp(-abs(sin(atan2(ga.z, ga.x) * 5.0 + r_ga * 4.0)) * 12.0) * exp(-abs(ga.y) * 14.0) * (0.26 + 1.30 * f1);
        let valley_iso = (exp(-abs(r_ga - 0.20) * 36.0) + exp(-abs(r_ga - 0.38) * 36.0)) * exp(-abs(ga.y + 0.10) * 18.0) * (0.22 + 1.35 * f2);
        let hubble_rings = exp(-abs(r - 0.48) * 34.0) * (0.55 + 0.45 * cos(atan2(p.z, p.x) * 8.0)) * (0.20 + 1.38 * f3);
        let cone_axis = length(p.yz - vec2<f32>(0.18 * (p.x + 0.36), 0.0));
        let survey_cone = exp(-abs(cone_axis - 0.32 * max(p.x + 0.36, 0.0)) * 32.0) * smoothstep(-0.36, 0.42, p.x) * (0.20 + 1.40 * f4);
        dens = (flow_pulse + stream_rib + valley_iso + hubble_rings + survey_cone) * env_outer;
        neb_col = mix(vec3<f32>(0.24, 0.74, 1.46), vec3<f32>(1.50, 1.00, 0.36), clamp((0.55 - r_ga) * 1.1 + flow_pulse * 0.5, 0.0, 1.0))
            + vec3<f32>(0.28, 1.28, 1.18) * (valley_iso * g2 + survey_cone * g4) * 0.72
            + vec3<f32>(1.38, 0.46, 0.92) * hubble_rings * g3 * 0.68;
    } else if (z_idx == 6u) {
        // Z06 Local Void (1706): carved cosmic void cavity (F0), interior underdensity contours (F1), faster wall expansion (F2), outward repulsion vectors (F3), visual silence rim (F4)
        let vp = p - vec3<f32>(-0.08, -0.04, 0.02);
        let rv = length(vp);
        let cavity_mask = smoothstep(0.22, 0.38, rv);
        let void_wall = exp(-abs(rv - 0.36) * 28.0) * (0.55 + 0.45 * fbm3(p * 8.0, 3)) * (0.38 + 1.25 * f0);
        let under_iso = exp(-abs(rv - 0.18) * 38.0) * 0.32 * (0.18 + 1.35 * f1);
        let exp_shell = exp(-abs(rv - 0.44) * 34.0) * (0.55 + 0.45 * sin(atan2(vp.z, vp.x) * 6.0 - time * 2.0)) * (0.24 + 1.35 * f2);
        let rep_rays = exp(-abs(sin(atan2(vp.z, vp.x) * 6.0)) * 18.0) * exp(-abs(vp.y) * 18.0) * smoothstep(0.14, 0.44, rv) * (0.22 + 1.38 * f3);
        let silence_crown = exp(-abs(rv - 0.40) * 38.0) * exp(-abs(vp.y) * 24.0) * (0.26 + 1.42 * f4);
        let silence_atten = select(1.0, 0.55, doc_feat(inst) == 4u);
        dens = ((base_threads * 0.55 * cavity_mask + void_wall) * silence_atten + under_iso + exp_shell + rep_rays + silence_crown) * env_outer;
        neb_col = mix(vec3<f32>(0.24, 0.72, 1.44), vec3<f32>(1.46, 0.98, 0.38), clamp(void_wall * 1.1 + silence_crown * 0.9, 0.0, 1.0))
            + vec3<f32>(0.24, 1.28, 1.36) * (rep_rays * g3 + exp_shell * g2) * 0.72;
    } else if (z_idx == 7u) {
        // Z07 Great Attractor (1707): multi-stream convergence valley (F0), Hydra-Centaurus sub-knots (F1), Norma Cluster core (F2), Zone of Avoidance MW dust band (F3), Shapley scale shift (F4)
        let conv_streams = base_threads * (0.38 + 1.25 * f0);
        let hydra = exp(-length(p - vec3<f32>(-0.14, 0.14, -0.06)) * 22.0);
        let centaurus = exp(-length(p - vec3<f32>(0.04, 0.12, 0.08)) * 22.0);
        let hc_knots = (hydra + centaurus) * (0.30 + 1.42 * f1);
        let norma = exp(-length((p - vec3<f32>(0.18, -0.02, 0.02)) * vec3<f32>(0.9, 1.3, 1.1)) * 18.0) * (0.34 + 1.45 * f2);
        let zoa_band = exp(-abs(p.y + 0.22 * p.x) * 28.0) * exp(-abs(p.z) * 14.0) * (0.22 + 1.35 * f3);
        let zoa_absorb = 1.0 - 0.55 * g3 * exp(-abs(p.y + 0.22 * p.x) * 32.0);
        let shapley = (exp(-length(p - vec3<f32>(0.42, 0.16, -0.10)) * 20.0) + exp(-length(p.yz - vec2<f32>(0.36 * p.x, -0.22 * p.x)) * 24.0) * smoothstep(0.05, 0.46, p.x)) * (0.22 + 1.45 * f4);
        dens = ((conv_streams + hc_knots + norma) * max(zoa_absorb, 0.25) + zoa_band * 0.65 + shapley) * env_outer;
        neb_col = mix(vec3<f32>(0.26, 0.72, 1.44), vec3<f32>(1.52, 1.02, 0.36), clamp(norma * 1.2 + hc_knots * 0.9, 0.0, 1.0))
            + vec3<f32>(1.28, 0.52, 0.22) * zoa_band * g3 * 0.75
            + vec3<f32>(0.32, 1.28, 1.38) * shapley * g4 * 0.78;
    } else if (z_idx == 8u) {
        // Z08 Laniakea Basin (1708): full 520-Mly feather plume (F0), watershed separatrix hull (F1), 100k galaxy tracers (F2), kinematic vector grid (F3), unbound dark-energy rim (F4)
        let feather = base_threads * (0.42 + 1.22 * f0);
        let sep_hull = exp(-abs(r - 0.54) * 32.0) * (0.55 + 0.45 * cos(atan2(p.z, p.x) * 6.0) * cos(p.y * 10.0)) * (0.24 + 1.38 * f1);
        let gal_sparks = pow(max(0.0, snoise3(p * 18.0)), 3.2) * exp(-r * 2.2) * (0.30 + 1.42 * f2);
        let dyn_grid = (exp(-abs(sin(p.x * 16.0)) * 22.0) + exp(-abs(sin(p.y * 16.0)) * 22.0)) * exp(-abs(p.z) * 16.0) * 0.55 * (0.20 + 1.35 * f3);
        let unbound_rim = exp(-abs(r - 0.60) * 30.0) * (0.22 + 1.42 * f4);
        dens = (feather + sep_hull + gal_sparks + dyn_grid + unbound_rim) * env_outer;
        neb_col = mix(vec3<f32>(0.24, 0.74, 1.44), vec3<f32>(1.50, 1.02, 0.38), clamp(gal_sparks * 1.3 + (0.5 - r_ga) * 0.7, 0.0, 1.0))
            + vec3<f32>(0.26, 1.26, 1.32) * (sep_hull * g1 + dyn_grid * g3) * 0.70
            + vec3<f32>(1.46, 0.36, 0.52) * unbound_rim * g4 * 0.78;
    } else {
        // Z09 Laniakea Boundary (1709): watershed flow divide saddle (F0), permeable statistical boundary (F1), Perseus-Pisces & Shapley basins (F2), Cosmic Web Voronoi (F3), outward scale bridge (F4)
        let divide_wall = exp(-abs(p.x - 0.04 * sin(p.y * 6.0)) * 28.0) * exp(-r * 2.4) * (0.28 + 1.35 * f0);
        let perm_dash = exp(-abs(r - 0.48) * 34.0) * pow(max(0.0, cos(atan2(p.z, p.x) * 10.0)), 4.0) * (0.22 + 1.38 * f1);
        let pp_basin = exp(-length(p - vec3<f32>(0.36, -0.16, 0.10)) * 20.0);
        let sh_basin = exp(-length(p - vec3<f32>(0.34, 0.20, -0.12)) * 20.0);
        let lan_basin = exp(-length(p - vec3<f32>(-0.30, 0.0, 0.04)) * 18.0);
        let basins = (pp_basin + sh_basin + lan_basin * 0.75) * (0.28 + 1.42 * f2);
        let web_f1 = fbm3(p * 3.8, 3);
        let web_f2 = fbm3(p * 3.8 + vec3<f32>(6.1, -3.7, 4.9), 3);
        let web_voronoi = pow(smoothstep(0.045, 0.0, length(vec2<f32>(web_f1, web_f2))), 2.2) * (0.24 + 1.38 * f3);
        let scale_bridge = (exp(-abs(r - 0.34) * 34.0) + exp(-abs(r - 0.58) * 34.0)) * (0.20 + 1.38 * f4);
        dens = (base_threads * 0.42 + divide_wall + perm_dash + basins + web_voronoi + scale_bridge) * env_outer;
        neb_col = mix(vec3<f32>(0.28, 0.68, 1.44), vec3<f32>(1.48, 0.98, 0.40), clamp(basins * 1.2 + divide_wall * 0.75, 0.0, 1.0))
            + vec3<f32>(0.92, 0.44, 1.44) * (web_voronoi * g3 + scale_bridge * g4) * 0.72
            + vec3<f32>(0.26, 1.28, 1.24) * perm_dash * g1 * 0.70;
    }

    let out_d = select(0.0, min(dens, 1.38), dens >= 0.045);
    return vec4<f32>(min(neb_col, vec3<f32>(1.28)), out_d);
}

fn doc_cosmo_finish(raw_col: vec3<f32>, dens: f32) -> vec4<f32> {
    let comp_col = raw_col / (vec3<f32>(1.0) + 0.30 * max(raw_col, vec3<f32>(0.0)));
    let out_d = select(0.0, min(dens, 1.36), dens >= 0.045);
    return vec4<f32>(min(comp_col, vec3<f32>(1.24)), out_d);
}

fn doc_cosmo_filament_field(p: vec3<f32>, scale: f32) -> vec4<f32> {
    let w_warp = curl_noise3(p * (scale * 0.65));
    let pw = p * scale + w_warp * 0.22;
    let f_a = fbm3(pw, 2);
    let f_b = fbm3(pw + vec3<f32>(6.1, -3.7, 4.9), 2);
    let ridge_d = length(vec2<f32>(f_a, f_b));
    let c_fil = pow(smoothstep(0.11, 0.0, ridge_d), 2.1);
    let c_wall = exp(-abs(f_a) * 18.0) * 0.55;
    let c_node = pow(smoothstep(0.45, 0.92, snoise3(p * (scale * 1.45))), 2.6);
    let c_void = smoothstep(0.14, 0.42, ridge_d);
    return vec4<f32>(c_fil, c_wall, c_node, c_void);
}

fn shade_doc_cosmic_web_stage(
    vid: u32,
    p: vec3<f32>,
    r: f32,
    inst: HeroVolumeInstance,
    time: f32,
) -> vec4<f32> {
    if (r > 0.76) {
        return vec4<f32>(0.0);
    }
    let z_idx = select(
        select(clamp(doc_zone(inst), 0u, 10u), 10u, doc_zone(inst) > 10u),
        vid - 1800u,
        vid >= 1800u && vid <= 1810u
    );
    let f0 = doc_feat_focus(inst, 0u);
    let f1 = doc_feat_focus(inst, 1u);
    let f2 = doc_feat_focus(inst, 2u);
    let f3 = doc_feat_focus(inst, 3u);
    let f4 = doc_feat_focus(inst, 4u);
    let g0 = doc_solar_feat_gate(inst, 0u);
    let g1 = doc_solar_feat_gate(inst, 1u);
    let g2 = doc_solar_feat_gate(inst, 2u);
    let g3 = doc_solar_feat_gate(inst, 3u);
    let g4 = doc_solar_feat_gate(inst, 4u);

    let env_outer = (1.0 - smoothstep(0.48, 0.72, r)) * 0.76;
    let web = doc_cosmo_filament_field(p, 3.2);
    let fil_base = web.x;
    let wall_base = web.y;
    let node_base = web.z;
    var dens: f32 = 0.0;
    var neb_col = mix(vec3<f32>(0.26, 0.56, 1.38), vec3<f32>(1.18, 0.72, 1.42), clamp(fil_base + node_base, 0.0, 1.0));

    if (z_idx == 0u) {
        // Z00 Laniakea Limit (1800): Laniakea funnel (F0), velocity divide membrane (F1), Perseus-Pisces basin (F2), 100->500 Mly scale rings (F3), outer web (F4)
        let lan_p = p - vec3<f32>(-0.22, -0.06, 0.04);
        let lan_r = length(lan_p);
        let lan_funnel = (exp(-lan_r * 13.0) + exp(-abs(atan2(lan_p.y, lan_p.x) * 3.0)) * exp(-lan_r * 5.0) * 0.75) * (0.26 + 1.35 * f0);
        let divide_surf = exp(-abs(p.x - 0.05 * sin(p.y * 7.0) - 0.04 * cos(p.z * 6.0)) * 34.0) * exp(-r * 2.2) * (0.20 + 1.42 * f1);
        let pp_p = p - vec3<f32>(0.26, 0.08, -0.04);
        let pp_basin = (exp(-length(pp_p) * 14.0) + exp(-abs(length(pp_p.xz) - 0.14) * 30.0) * exp(-abs(pp_p.y) * 18.0)) * (0.24 + 1.40 * f2);
        let ring_a = exp(-abs(r - 0.24) * 42.0);
        let ring_b = exp(-abs(r - 0.42) * 42.0);
        let ring_c = exp(-abs(r - 0.56) * 42.0);
        let scale_rings = (ring_a + ring_b * 0.9 + ring_c * 0.8) * (0.16 + 1.42 * f3);
        let outer_web = (fil_base * 0.92 + node_base * 0.75) * smoothstep(0.18, 0.42, r) * (0.20 + 1.38 * f4);
        dens = (lan_funnel + divide_surf + pp_basin + scale_rings + outer_web) * env_outer;
        neb_col = mix(vec3<f32>(0.24, 0.72, 1.42), vec3<f32>(1.48, 0.98, 0.36), clamp(lan_funnel * 1.15 + g0 * 0.3, 0.0, 1.0))
            + vec3<f32>(0.28, 1.32, 1.26) * (divide_surf * (0.6 + g1) + pp_basin * g2)
            + vec3<f32>(0.92, 0.48, 1.45) * (scale_rings * g3 + outer_web * g4) * 0.75;
    } else if (z_idx == 1u) {
        // Z01 Intersupercluster Space (1801): dark gap between twin supernodes (F0), sparse field galaxies (F1), tenuous bridge (F2), baseline ruler (F3), survey wedge (F4)
        let n_left = exp(-length(p - vec3<f32>(-0.34, 0.02, 0.02)) * 13.0);
        let n_right = exp(-length(p - vec3<f32>(0.34, -0.02, -0.02)) * 13.0);
        let gap_hollow = smoothstep(0.08, 0.26, abs(p.x));
        let supernodes = (n_left + n_right) * (0.34 + 1.25 * f0) + exp(-r * 6.0) * 0.14 * (1.0 - gap_hollow);
        let stipple = pow(max(0.0, snoise3(p * 18.0)), 9.0) * exp(-abs(p.y) * 5.0) * (0.22 + 1.45 * f1);
        let bridge_d = length(vec2<f32>(p.y - 0.06 * sin(p.x * 5.5), p.z));
        let bridge = exp(-bridge_d * 34.0) * smoothstep(0.42, 0.05, abs(p.x)) * (0.18 + 1.42 * f2);
        let ruler_y = abs(p.y + 0.22);
        let ticks = 0.45 + 0.55 * pow(max(0.0, cos(p.x * 36.0)), 10.0);
        let ruler = exp(-ruler_y * 42.0) * exp(-abs(p.z) * 24.0) * ticks * smoothstep(0.46, 0.04, abs(p.x)) * (0.14 + 1.44 * f3);
        let wedge_ang = atan2(p.y, p.x + 0.36);
        let survey_wedge = smoothstep(0.42, 0.06, abs(wedge_ang)) * exp(-abs(p.z) * 14.0) * smoothstep(-0.32, 0.38, p.x) * (1.0 - smoothstep(0.28, 0.54, p.x)) * 0.58 * (0.14 + 1.40 * f4);
        dens = (supernodes + stipple + bridge + ruler + survey_wedge) * env_outer;
        neb_col = mix(vec3<f32>(0.28, 0.64, 1.38), vec3<f32>(1.46, 0.94, 0.38), clamp(supernodes * 1.1 + stipple * 0.8, 0.0, 1.0))
            + vec3<f32>(0.42, 1.18, 1.46) * (bridge * (0.65 + g2) + survey_wedge * g4)
            + vec3<f32>(1.24, 0.86, 0.32) * ruler * (0.5 + g3);
    } else if (z_idx == 2u) {
        // Z02 Galactic Filaments (1802): dominant filament spine (F0), wide indigo DM sheath (F1), warm WHIM gas (F2), galaxy beads (F3), potential contour rings (F4)
        let spine_c = vec2<f32>(0.10 * sin(p.x * 4.5), 0.06 * cos(p.x * 3.8));
        let tube_r = length(p.yz - spine_c);
        let spine = exp(-tube_r * 28.0) * smoothstep(0.56, 0.06, abs(p.x)) * (0.34 + 1.32 * f0);
        let dm_sheath = exp(-tube_r * 7.5) * smoothstep(0.58, 0.08, abs(p.x)) * 0.62 * (0.20 + 1.38 * f1);
        let whim = exp(-abs(tube_r - 0.075) * 38.0) * smoothstep(0.52, 0.08, abs(p.x)) * (0.55 + 0.45 * fbm3(p * 10.0, 2)) * (0.18 + 1.42 * f2);
        let bead_wave = pow(max(0.0, cos(p.x * 34.0)), 14.0);
        let beads = exp(-tube_r * 22.0) * bead_wave * smoothstep(0.48, 0.04, abs(p.x)) * (0.20 + 1.48 * f3);
        let pot_rings = (exp(-abs(tube_r - 0.14) * 44.0) + exp(-abs(tube_r - 0.22) * 44.0)) * smoothstep(0.44, 0.08, abs(p.x)) * (0.14 + 1.42 * f4);
        dens = (spine + dm_sheath + whim + beads + pot_rings) * env_outer;
        neb_col = mix(vec3<f32>(0.38, 0.28, 1.32), vec3<f32>(0.96, 1.18, 1.48), clamp(spine * 1.4 + beads, 0.0, 1.0))
            + vec3<f32>(1.48, 0.78, 0.28) * whim * (0.65 + g2)
            + vec3<f32>(1.44, 1.12, 0.40) * beads * (0.5 + g3)
            + vec3<f32>(0.24, 1.24, 1.36) * pot_rings * g4;
    } else if (z_idx == 3u) {
        // Z03 Cluster Nodes (1803): 5-spoke converging node (F0), galaxy swarm (F1), rose-violet X-ray bremsstrahlung core (F2), blue Einstein lensing arcs (F3), infalling subclumps (F4)
        let sp1 = exp(-length(p.yz - vec2<f32>(0.32 * p.x, -0.14 * p.x)) * 22.0);
        let sp2 = exp(-length(p.xz - vec2<f32>(-0.28 * p.y, 0.30 * p.y)) * 22.0);
        let sp3 = exp(-length(p.xy - vec2<f32>(0.22 * p.z, 0.26 * p.z)) * 22.0);
        let spokes = (sp1 + sp2 + sp3 * 0.85 + exp(-r * 13.0)) * (0.28 + 1.35 * f0);
        let swarm = pow(max(0.0, snoise3(p * 24.0)), 8.0) * smoothstep(0.28, 0.04, r) * 1.35 * (0.22 + 1.42 * f1);
        let xray = exp(-r * 6.5) * 0.78 * (0.20 + 1.42 * f2);
        let arc_r = length(p.xy);
        let arc_az = abs(cos(atan2(p.y, p.x) * 2.0 + 0.4));
        let lens_arc = exp(-abs(arc_r - 0.21) * 46.0) * pow(arc_az, 4.0) * exp(-abs(p.z) * 20.0) * (0.16 + 1.48 * f3);
        let sc1 = exp(-length(p - vec3<f32>(0.32, 0.12, -0.05)) * 20.0);
        let sc2 = exp(-length(p - vec3<f32>(-0.26, -0.22, 0.10)) * 20.0);
        let sc3 = exp(-length(p - vec3<f32>(0.08, -0.30, 0.14)) * 20.0);
        let subclumps = (sc1 + sc2 + sc3) * (0.20 + 1.42 * f4);
        dens = (spokes + swarm + xray + lens_arc + subclumps) * env_outer;
        neb_col = mix(vec3<f32>(1.15, 0.62, 1.38), vec3<f32>(1.52, 1.08, 0.44), clamp(spokes * 0.7 + swarm, 0.0, 1.0))
            + vec3<f32>(1.42, 0.34, 0.86) * xray * (0.55 + g2)
            + vec3<f32>(0.28, 0.96, 1.56) * lens_arc * (0.75 + g3)
            + vec3<f32>(1.36, 0.88, 0.36) * subclumps * g4;
    } else if (z_idx == 4u) {
        // Z04 Cosmic Voids (1804): hollowed Voronoi void cell (F0), 4 sparse dwarf galaxies (F1), outward Hubble arrows (F2), bright boundary rim (F3), polyhedral wireframe (F4)
        let rim_wall = exp(-abs(r - 0.38) * 26.0) * (0.55 + 0.45 * wall_base);
        let void_fil = fil_base * smoothstep(0.28, 0.40, r) * 0.88;
        let cavity_shell = (rim_wall * 0.72 + void_fil) * (0.30 + 1.28 * f0);
        let dg1 = exp(-length(p - vec3<f32>(-0.14, 0.08, 0.05)) * 32.0);
        let dg2 = exp(-length(p - vec3<f32>(0.12, -0.10, -0.06)) * 32.0);
        let dg3 = exp(-length(p - vec3<f32>(0.04, 0.16, -0.08)) * 32.0);
        let dwarfs = (dg1 + dg2 + dg3) * (0.18 + 1.48 * f1);
        let az = atan2(p.z, p.x);
        let out_phase = pow(max(0.0, cos(r * 34.0 - time * 1.4)), 6.0);
        let hub_rays = pow(max(0.0, cos(az * 6.0)), 10.0) * exp(-abs(p.y) * 16.0) * smoothstep(0.08, 0.14, r) * (1.0 - smoothstep(0.28, 0.36, r)) * (0.45 + 0.85 * out_phase) * (0.16 + 1.46 * f2);
        let bright_rim = exp(-abs(r - 0.40) * 44.0) * (0.20 + 1.44 * f3);
        let cell_edge = exp(-abs(abs(sin(p.x * 11.0)) + abs(sin(p.y * 11.0)) - 0.92) * 28.0) * exp(-abs(r - 0.38) * 16.0) * (0.14 + 1.42 * f4);
        dens = (cavity_shell + dwarfs + hub_rays + bright_rim + cell_edge) * env_outer;
        neb_col = mix(vec3<f32>(0.24, 0.56, 1.26), vec3<f32>(1.18, 0.78, 1.42), clamp(bright_rim + void_fil, 0.0, 1.0))
            + vec3<f32>(0.36, 1.28, 1.48) * (dwarfs * (0.6 + g1) + hub_rays * (0.7 + g2))
            + vec3<f32>(1.42, 0.96, 0.38) * (bright_rim * g3 + cell_edge * g4) * 0.75;
    } else if (z_idx == 5u) {
        // Z05 Galactic Walls (1805): codimension-1 flattened 2D sheet (F0), galaxy-stippled sheet (F1), twin flanking voids (F2), embedded filaments/nodes (F3), edge-on profile tilt (F4)
        let wall_y = p.y - 0.06 * sin(p.x * 4.5) * cos(p.z * 4.5);
        let sheet = exp(-abs(wall_y) * 34.0) * smoothstep(0.56, 0.14, length(p.xz)) * (0.34 + 1.32 * f0);
        let stipple = pow(max(0.0, snoise3(p * 22.0)), 7.0) * exp(-abs(wall_y) * 28.0) * (0.20 + 1.44 * f1);
        let void_bound = (exp(-abs(p.y - 0.24) * 32.0) + exp(-abs(p.y + 0.24) * 32.0)) * smoothstep(0.46, 0.08, length(p.xz)) * 0.58 * (0.14 + 1.42 * f2);
        let vein = exp(-abs(wall_y) * 24.0) * (fil_base * 1.15 + node_base * 0.95) * (0.20 + 1.44 * f3);
        let cross_cut = exp(-abs(p.z) * 44.0) * exp(-abs(wall_y) * 22.0) * smoothstep(0.52, 0.08, abs(p.x)) * (0.14 + 1.44 * f4);
        dens = (sheet + stipple + void_bound + vein + cross_cut) * env_outer;
        neb_col = mix(vec3<f32>(0.42, 0.56, 1.42), vec3<f32>(1.46, 1.06, 0.48), clamp(stipple + vein * 0.7, 0.0, 1.0))
            + vec3<f32>(0.34, 1.24, 1.44) * (void_bound * g2 + cross_cut * (0.65 + g4))
            + vec3<f32>(1.36, 0.56, 1.24) * vein * g3;
    } else if (z_idx == 6u) {
        // Z06 Filament Intersections (1806): Y-fork bifurcation (F0), multi-spoke hub (F1), diffuse baryonic bridge (F2), infall chevrons (F3), minor vs primary knots (F4)
        let trunk = exp(-length(p.yz - vec2<f32>(0.0, 0.0)) * 28.0) * smoothstep(0.04, -0.34, p.x);
        let br_a = exp(-length(vec2<f32>(p.y - 0.72 * p.x, p.z - 0.22 * p.x)) * 28.0) * smoothstep(-0.04, 0.36, p.x);
        let br_b = exp(-length(vec2<f32>(p.y + 0.72 * p.x, p.z + 0.22 * p.x)) * 28.0) * smoothstep(-0.04, 0.36, p.x);
        let y_fork = (trunk + br_a + br_b) * (0.32 + 1.32 * f0);
        let hub_ring = exp(-abs(r - 0.14) * 44.0) * pow(0.5 + 0.5 * cos(atan2(p.y, p.x) * 5.0), 3.0) * (0.18 + 1.44 * f1);
        let b_bridge = exp(-length(p.yz) * 11.0) * smoothstep(0.46, 0.05, abs(p.x)) * 0.58 * (0.18 + 1.42 * f2);
        let chev = pow(max(0.0, sin((abs(p.x) + abs(p.y) * 0.6) * 28.0 + time * 1.5)), 6.0) * (trunk + br_a + br_b) * (0.16 + 1.46 * f3);
        let k_main = exp(-r * 14.0);
        let k_sub = exp(-length(p - vec3<f32>(0.24, 0.17, 0.05)) * 24.0) + exp(-length(p - vec3<f32>(0.24, -0.17, -0.05)) * 24.0);
        let knots = (k_main + k_sub * 0.85) * (0.22 + 1.40 * f4);
        dens = (y_fork + hub_ring + b_bridge + chev + knots) * env_outer;
        neb_col = mix(vec3<f32>(0.34, 0.66, 1.44), vec3<f32>(1.48, 0.98, 0.38), clamp(k_main + b_bridge * 0.8, 0.0, 1.0))
            + vec3<f32>(0.28, 1.28, 1.38) * (hub_ring * g1 + chev * (0.65 + g3))
            + vec3<f32>(1.32, 0.48, 1.34) * knots * g4;
    } else if (z_idx == 7u) {
        // Z07 Supercluster Complexes (1807): 3 regional complexes (F0), nested cluster-of-clusters (F1), 3 flow basins (F2), Sloan Great Wall arc (F3), decentralized reticles (F4)
        let c0 = vec3<f32>(-0.28, -0.10, 0.06);
        let c1 = vec3<f32>(0.26, 0.16, -0.08);
        let c2 = vec3<f32>(0.06, -0.22, -0.14);
        let comp_a = exp(-length(p - c0) * 11.0);
        let comp_b = exp(-length(p - c1) * 11.0);
        let comp_c = exp(-length(p - c2) * 11.0);
        let complexes = (comp_a + comp_b + comp_c) * (0.30 + 1.32 * f0);
        let sub_halos = (exp(-length(p - (c0 + vec3<f32>(0.08, 0.06, 0.0))) * 24.0)
            + exp(-length(p - (c1 - vec3<f32>(0.08, 0.05, 0.04))) * 24.0)
            + exp(-length(p - (c2 + vec3<f32>(-0.07, 0.06, 0.04))) * 24.0)) * (0.20 + 1.44 * f1);
        let basins = (exp(-abs(length(p - c0) - 0.16) * 32.0) + exp(-abs(length(p - c1) - 0.16) * 32.0) + exp(-abs(length(p - c2) - 0.15) * 32.0)) * (0.18 + 1.42 * f2);
        let sloan_arc = exp(-abs(length(p.xz) - 0.34) * 30.0) * exp(-abs(p.y - 0.06 * sin(p.x * 5.0)) * 22.0) * smoothstep(-0.25, 0.10, p.z) * (0.18 + 1.44 * f3);
        let ret_r = min(min(abs(length(p - c0) - 0.09), abs(length(p - c1) - 0.09)), abs(length(p - c2) - 0.09));
        let dec_ret = exp(-ret_r * 46.0) * (0.14 + 1.46 * f4);
        dens = (complexes + sub_halos + basins + sloan_arc + dec_ret + fil_base * 0.42) * env_outer;
        neb_col = vec3<f32>(1.48, 0.98, 0.36) * comp_a * (0.7 + g2)
            + vec3<f32>(0.28, 1.18, 1.48) * comp_b * (0.7 + g2)
            + vec3<f32>(1.18, 0.46, 1.48) * comp_c * (0.7 + g2)
            + vec3<f32>(1.38, 1.16, 0.62) * (sub_halos * g1 + sloan_arc * (0.65 + g3))
            + vec3<f32>(0.38, 1.38, 1.24) * (basins * g2 + dec_ret * g4);
    } else if (z_idx == 8u) {
        // Z08 Deep Cosmic Web (1808): stepped depth slabs (F0), redshift color gradient (F1), SDSS/DESI double survey cones (F2), linear-to-web growth (F3), 3D isotropic web (F4)
        let slab = (exp(-abs(p.z + 0.22) * 34.0) + exp(-abs(p.z) * 34.0) + exp(-abs(p.z - 0.22) * 34.0)) * (0.45 + 0.55 * fil_base) * (0.24 + 1.36 * f0);
        let red_shell = smoothstep(0.10, 0.48, r) * (fil_base * 0.9 + node_base * 0.7) * (0.26 + 1.38 * f1);
        let butterfly = smoothstep(0.08, -0.02, abs(p.y) - abs(p.x) * 0.62) * exp(-abs(p.z) * 10.0) * smoothstep(0.04, 0.14, abs(p.x)) * (0.48 + 0.68 * fil_base) * (0.16 + 1.44 * f2);
        let lin_ripples = exp(-abs(r - 0.32 - 0.03 * snoise3(p * 6.0)) * 28.0) * (0.18 + 1.42 * f3);
        let iso_web = (fil_base * 0.92 + wall_base * 0.35 + node_base * 0.65) * (0.28 + 1.34 * f4);
        dens = (slab + red_shell + butterfly + lin_ripples + iso_web) * env_outer;
        let z_col = mix(vec3<f32>(0.36, 0.82, 1.48), vec3<f32>(1.48, 0.42, 0.24), smoothstep(0.12, 0.52, r) * (0.55 + 0.65 * g1));
        neb_col = z_col * (iso_web + red_shell + 0.35)
            + vec3<f32>(0.28, 1.24, 1.38) * (slab * g0 + butterfly * (0.6 + g2))
            + vec3<f32>(1.32, 0.96, 0.38) * lin_ripples * g3;
    } else if (z_idx == 9u) {
        // Z09 Homogeneity Scale (1809): coarse-grained smoothing (F0), 360-deg isotropy sightlines (F1), sample caliper sphere (F2), underlying web skeleton (F3), shifted origin grid (F4)
        let smooth_avg = exp(-r * 3.8) * (0.36 + 0.18 * snoise3(p * 2.5)) * (0.24 + 1.36 * f0);
        let iso_axes = (exp(-length(p.yz) * 28.0) + exp(-length(p.xz) * 28.0) + exp(-length(p.xy) * 28.0)) * smoothstep(0.05, 0.18, r) * (0.16 + 1.44 * f1);
        let cal_r = 0.24 + 0.14 * f2;
        let caliper = exp(-abs(r - cal_r) * 44.0) * (0.20 + 1.46 * f2);
        let web_skel = (fil_base * 0.85 + node_base * 0.65) * (0.22 + 1.40 * f3);
        let shift_p = p - vec3<f32>(0.16, 0.10, -0.06);
        let princ_grid = exp(-abs(sin(shift_p.x * 16.0)) * 18.0) * exp(-abs(sin(shift_p.y * 16.0)) * 18.0) * exp(-abs(length(shift_p) - 0.24) * 18.0) * (0.16 + 1.42 * f4);
        dens = (smooth_avg + iso_axes + caliper + web_skel + princ_grid) * env_outer;
        neb_col = mix(vec3<f32>(0.42, 0.72, 1.38), vec3<f32>(1.18, 0.96, 1.44), clamp(smooth_avg * 1.4 + web_skel, 0.0, 1.0))
            + vec3<f32>(0.26, 1.34, 1.28) * (iso_axes * g1 + caliper * (0.65 + g2))
            + vec3<f32>(1.44, 0.96, 0.36) * princ_grid * g4;
    } else {
        // Z10 Large-Scale Structure & Vid 8 Fallback (1810 / 8): complete filament+wall+node tapestry (F0), parallax 3D depth (F1), soft vignette (F2), Laniakea pin marker (F3), outer horizon preview (F4)
        let f_fil = select(1.0, 0.28 + 1.30 * f0, vid != 8u);
        let full_web = (fil_base * 1.05 + wall_base * 0.42 + node_base * 0.95) * f_fil;
        let depth_arcs = (exp(-abs(p.z + 0.18) * 30.0) + exp(-abs(p.z - 0.18) * 30.0)) * fil_base * (0.18 + 1.38 * f1);
        let vig_halo = exp(-abs(r - 0.44) * 20.0) * 0.45 * (0.18 + 1.34 * f2);
        let lan_pin_p = p - vec3<f32>(-0.18, -0.08, 0.04);
        let lan_pin_d = length(lan_pin_p);
        let lan_pin = (exp(-lan_pin_d * 26.0) + exp(-abs(lan_pin_d - 0.09) * 46.0)) * (0.20 + 1.50 * f3);
        let horizon_rim = exp(-abs(r - 0.56) * 34.0) * (0.55 + 0.45 * wall_base) * (0.14 + 1.44 * f4);
        dens = (full_web + depth_arcs + vig_halo + lan_pin + horizon_rim) * env_outer;
        neb_col = mix(vec3<f32>(0.28, 0.54, 1.38), vec3<f32>(1.24, 0.72, 1.46), clamp(node_base * 1.3 + fil_base * 0.5, 0.0, 1.0))
            + vec3<f32>(0.26, 1.18, 1.42) * depth_arcs * g1
            + vec3<f32>(1.56, 1.14, 0.34) * lan_pin * (0.65 + g3)
            + vec3<f32>(1.38, 0.74, 0.34) * horizon_rim * (0.55 + g4);
    }

    return doc_cosmo_finish(neb_col, dens);
}

fn shade_doc_observable_universe_stage(
    vid: u32,
    p: vec3<f32>,
    r: f32,
    inst: HeroVolumeInstance,
    time: f32,
) -> vec4<f32> {
    if (r > 0.76) {
        return vec4<f32>(0.0);
    }
    let is_overview = vid == 9u;
    let z_idx = select(
        select(clamp(doc_zone(inst), 0u, 10u), 8u, doc_zone(inst) > 10u),
        vid - 1900u,
        vid >= 1900u && vid <= 1910u
    );
    let f0 = doc_feat_focus(inst, 0u);
    let f1 = doc_feat_focus(inst, 1u);
    let f2 = doc_feat_focus(inst, 2u);
    let f3 = doc_feat_focus(inst, 3u);
    let f4 = doc_feat_focus(inst, 4u);
    let g0 = doc_solar_feat_gate(inst, 0u);
    let g1 = doc_solar_feat_gate(inst, 1u);
    let g2 = doc_solar_feat_gate(inst, 2u);
    let g3 = doc_solar_feat_gate(inst, 3u);
    let g4 = doc_solar_feat_gate(inst, 4u);

    let env_outer = (1.0 - smoothstep(0.48, 0.72, r)) * 0.76;
    let web = doc_cosmo_filament_field(p, 3.6);
    let fil_base = web.x;
    let wall_base = web.y;
    let node_base = web.z;
    var dens: f32 = 0.0;
    var neb_col = mix(vec3<f32>(0.28, 0.78, 1.44), vec3<f32>(1.44, 0.86, 0.42), clamp(r * 1.4, 0.0, 1.0));

    if (is_overview) {
        // Overview fallback (vid == 9u): compact porous particle-horizon sphere + cosmic web skeleton + inner core
        let mer = exp(-abs(sin(p.y * 13.0)) * 20.0) + exp(-abs(sin(p.x * 11.0) * sin(p.z * 11.0)) * 22.0) * 0.65;
        let shell_cmb = exp(-abs(r - 0.50) * 38.0) * (0.28 + 0.78 * clamp(mer, 0.0, 1.2));
        let shell_hor = exp(-abs(r - 0.58) * 44.0) * 0.68;
        let inner_web = (fil_base * 0.88 + node_base * 0.82) * smoothstep(0.48, 0.14, r);
        let quasar_axis = exp(-length(p.xy) * 28.0) * smoothstep(0.50, 0.04, abs(p.z)) * select(0.25, 1.15, z_idx == 4u || z_idx == 6u);
        dens = (shell_cmb + shell_hor + inner_web + quasar_axis) * env_outer;
        neb_col = mix(vec3<f32>(0.28, 0.82, 1.48), vec3<f32>(1.52, 0.76, 0.34), clamp(shell_cmb * 0.95 + quasar_axis, 0.0, 1.0));
        return doc_cosmo_finish(neb_col, dens);
    }

    if (z_idx == 0u) {
        // Z00 Large-Scale Structure (1900): non-solid web skeleton (F0), warm local core (F1), nested scale shells (F2), contracting wavefront rings (F3), lookback ruler (F4)
        let web_skel = (fil_base * 0.95 + node_base * 0.75) * (0.28 + 1.32 * f0);
        let local_core = exp(-r * 9.5) * 0.72 * (0.22 + 1.42 * f1);
        let scale_shells = (exp(-abs(r - 0.22) * 38.0) + exp(-abs(r - 0.38) * 38.0)) * (0.18 + 1.42 * f2);
        let wf_phase = fract(r * 7.5 + time * 0.35);
        let wavefronts = exp(-abs(wf_phase - 0.5) * 14.0) * smoothstep(0.08, 0.18, r) * smoothstep(0.56, 0.36, r) * 0.72 * (0.16 + 1.46 * f3);
        let ticks = 0.45 + 0.55 * pow(max(0.0, cos(p.x * 36.0)), 10.0);
        let ruler = exp(-length(p.yz - vec2<f32>(-0.20, 0.0)) * 36.0) * smoothstep(0.02, 0.08, p.x + 0.04) * smoothstep(0.54, 0.44, p.x) * ticks * (0.14 + 1.44 * f4);
        dens = (web_skel + local_core + scale_shells + wavefronts + ruler) * env_outer;
        neb_col = mix(vec3<f32>(0.28, 0.66, 1.42), vec3<f32>(1.48, 1.06, 0.46), clamp(local_core + g1 * 0.3, 0.0, 1.0))
            + vec3<f32>(0.26, 1.32, 1.34) * (scale_shells * g2 + wavefronts * (0.65 + g3))
            + vec3<f32>(1.42, 0.88, 0.32) * ruler * (0.55 + g4);
    } else if (z_idx == 1u) {
        // Z01 Deep Universe HUDF (1901): conical deep-field beam (F0), 4 epoch slice planes (F1), brightness dimming (F2), foreground lens Einstein arcs (F3), spiral->irregular depth gradient (F4)
        let x_beam = clamp(p.x + 0.36, 0.0, 0.90);
        let cone_r = 0.055 + 0.30 * x_beam;
        let perp_r = length(p.yz);
        let in_cone = smoothstep(cone_r + 0.02, cone_r - 0.02, perp_r) * smoothstep(-0.38, -0.30, p.x) * (1.0 - smoothstep(0.36, 0.48, p.x));
        let cone_wall = exp(-abs(perp_r - cone_r) * 42.0) * smoothstep(-0.38, -0.28, p.x) * (1.0 - smoothstep(0.36, 0.48, p.x));
        let gal_dots = pow(max(0.0, snoise3(p * 20.0)), 7.0) * in_cone;
        let beam = (cone_wall * 0.72 + gal_dots * 1.1) * (0.28 + 1.34 * f0);
        let slices = (exp(-abs(p.x + 0.20) * 48.0) + exp(-abs(p.x + 0.02) * 48.0) + exp(-abs(p.x - 0.16) * 48.0) + exp(-abs(p.x - 0.32) * 48.0)) * in_cone * (0.16 + 1.46 * f1);
        let dim_band = smoothstep(0.05, 0.36, p.x) * in_cone * 0.55 * (0.16 + 1.40 * f2);
        let lens_p = p - vec3<f32>(-0.14, 0.0, 0.0);
        let einstein_ring = (exp(-length(lens_p) * 26.0) + exp(-abs(length(lens_p.yz) - 0.09) * 46.0) * exp(-abs(lens_p.x) * 28.0)) * (0.18 + 1.48 * f3);
        let morph = smoothstep(-0.28, 0.28, p.x) * (0.45 + 0.55 * fil_base) * in_cone * (0.18 + 1.42 * f4);
        dens = (beam + slices + dim_band + einstein_ring + morph) * env_outer;
        neb_col = mix(vec3<f32>(0.38, 0.84, 1.48), vec3<f32>(1.48, 0.48, 0.28), smoothstep(-0.24, 0.34, p.x)) * (beam + morph + 0.3)
            + vec3<f32>(0.26, 1.32, 1.24) * slices * (0.65 + g1)
            + vec3<f32>(1.50, 1.12, 0.36) * einstein_ring * (0.65 + g3);
    } else if (z_idx == 2u) {
        // Z02 Lookback Time (1902): inward photons on 45-deg light-cone (F0), 1 yr = 1 ly ticks (F1), redshifting wave packet (F2), past light-cone wireframe (F3), detached t=now hyperplane (F4)
        let cone_xz = length(p.xz);
        let target_r = clamp(0.36 - p.y * 0.82, 0.02, 0.54);
        let on_cone = exp(-abs(cone_xz - target_r) * 34.0) * smoothstep(-0.42, -0.32, p.y) * (1.0 - smoothstep(0.28, 0.38, p.y));
        let ph_phase = exp(-abs(fract(cone_xz * 8.0 + time * 0.35) - 0.5) * 10.0);
        let photons = on_cone * (0.35 + 0.95 * ph_phase) * (0.28 + 1.34 * f0);
        let ly_ticks = exp(-abs(p.x) * 40.0) * exp(-abs(p.z) * 40.0) * pow(max(0.0, cos(p.y * 42.0)), 8.0) * smoothstep(0.42, 0.04, abs(p.y)) * (0.14 + 1.44 * f1);
        let z_wave = on_cone * (0.5 + 0.5 * sin(cone_xz * 48.0 * (1.3 - p.y))) * (0.20 + 1.42 * f2);
        let wire_mer = pow(max(0.0, cos(atan2(p.z, p.x) * 8.0)), 8.0);
        let wireframe = on_cone * (0.4 + 0.85 * wire_mer) * (0.20 + 1.42 * f3);
        let t_now = exp(-abs(p.y - 0.32) * 42.0) * exp(-abs(cone_xz - 0.24) * 26.0) * pow(0.5 + 0.5 * cos(atan2(p.z, p.x) * 10.0), 4.0) * (0.14 + 1.48 * f4);
        dens = (photons + ly_ticks + z_wave + wireframe + t_now) * env_outer;
        neb_col = mix(vec3<f32>(0.38, 0.82, 1.52), vec3<f32>(1.48, 0.44, 0.26), smoothstep(0.28, -0.32, p.y))
            + vec3<f32>(1.52, 1.12, 0.34) * (photons * 0.45 + ly_ticks * g1)
            + vec3<f32>(0.26, 1.28, 1.36) * t_now * (0.65 + g4);
    } else if (z_idx == 3u) {
        // Z03 Distant Galaxies (1903): compact irregular protogalaxies (F0), blue-UV starburst knots (F1), tidal galaxy merger (F2), low-metallicity blue continuum (F3), IR redshift bar (F4)
        let proto_noise = 0.5 + 0.5 * fbm3(p * 11.0, 2);
        let pg1 = exp(-length(p - vec3<f32>(-0.24, 0.14, 0.04)) * 16.0);
        let pg2 = exp(-length(p - vec3<f32>(0.24, -0.10, -0.06)) * 16.0);
        let pg3 = exp(-length(p - vec3<f32>(0.02, 0.26, -0.10)) * 16.0);
        let protos = (pg1 + pg2 + pg3) * (0.65 + 0.55 * proto_noise) * (0.30 + 1.32 * f0);
        let h2_knots = pow(max(0.0, snoise3(p * 24.0)), 8.0) * smoothstep(0.42, 0.08, r) * 1.4 * (0.20 + 1.46 * f1);
        let m_a = exp(-length(p - vec3<f32>(-0.08, -0.04, 0.0)) * 20.0);
        let m_b = exp(-length(p - vec3<f32>(0.09, 0.05, 0.02)) * 20.0);
        let t_arc = exp(-abs(length(p.xy) - 0.14) * 36.0) * exp(-abs(p.z) * 24.0);
        let merger = (m_a + m_b + t_arc * 0.85) * (0.24 + 1.42 * f2);
        let uv_glow = exp(-r * 6.5) * 0.56 * (0.18 + 1.40 * f3);
        let ir_bar = exp(-abs(p.y + 0.26) * 42.0) * exp(-abs(p.z) * 24.0) * smoothstep(0.44, 0.04, abs(p.x)) * (0.14 + 1.48 * f4);
        dens = (protos + h2_knots + merger + uv_glow + ir_bar) * env_outer;
        neb_col = mix(vec3<f32>(0.34, 0.78, 1.54), vec3<f32>(0.92, 0.58, 1.46), clamp(protos * 0.8 + g0 * 0.2, 0.0, 1.0))
            + vec3<f32>(0.38, 1.24, 1.56) * (h2_knots * g1 + uv_glow * g3)
            + vec3<f32>(1.48, 0.48, 0.26) * ir_bar * (0.75 + smoothstep(-0.35, 0.35, p.x) * 0.5 + g4);
    } else if (z_idx == 4u) {
        // Z04 Quasar Era (1904): luminous AGN core (F0), relativistic accretion disk + BH shadow (F1), bipolar plasma jets (F2), diffuse host envelope (F3), Lyman-alpha forest beacon (F4)
        let agn_core = exp(-r * 19.0) * (0.34 + 1.38 * f0);
        let dp = doc_rot_x(p, 0.42);
        let dr = length(dp.xz);
        let acc_disk = exp(-abs(dp.y) * 46.0) * smoothstep(0.045, 0.075, dr) * (1.0 - smoothstep(0.18, 0.26, dr)) * (0.28 + 1.40 * f1);
        let bh_shadow = smoothstep(0.032, 0.056, dr);
        let jet_axis = length(dp.xz);
        let jet_knots = 0.55 + 0.45 * pow(max(0.0, cos(dp.y * 32.0 - time * 2.4)), 4.0);
        let jets = exp(-jet_axis * 34.0) * smoothstep(0.04, 0.10, abs(dp.y)) * (1.0 - smoothstep(0.36, 0.54, abs(dp.y))) * jet_knots * (0.22 + 1.46 * f2);
        let host = exp(-length(dp * vec3<f32>(1.0, 1.6, 1.0)) * 7.5) * 0.52 * (0.16 + 1.42 * f3);
        let lya_clouds = 0.35 + 0.85 * pow(max(0.0, sin(p.x * 26.0)), 6.0);
        let lya_ray = exp(-length(p.yz - vec2<f32>(0.08 * p.x, 0.0)) * 38.0) * smoothstep(0.02, 0.10, p.x) * (1.0 - smoothstep(0.40, 0.56, p.x)) * lya_clouds * (0.14 + 1.48 * f4);
        dens = (agn_core * bh_shadow + acc_disk + jets + host + lya_ray) * env_outer;
        neb_col = mix(vec3<f32>(1.52, 1.14, 0.44), vec3<f32>(0.42, 0.92, 1.56), clamp(jets * 1.15, 0.0, 1.0))
            + vec3<f32>(1.54, 1.24, 0.52) * (acc_disk * g1 + lya_ray * (0.65 + g4))
            + vec3<f32>(1.26, 0.58, 1.44) * host * g3;
    } else if (z_idx == 5u) {
        // Z05 First Galaxies / Cosmic Dawn (1905): first star clusters (F0), indigo DM minihalo wells (F1), massive UV Pop-III stars (F2), bottom-up halo mergers (F3), UV ionization shells (F4)
        let h0 = exp(-length(p - vec3<f32>(-0.24, 0.14, 0.04)) * 18.0);
        let h1 = exp(-length(p - vec3<f32>(0.22, -0.14, 0.06)) * 18.0);
        let h2 = exp(-length(p - vec3<f32>(0.02, 0.04, -0.02)) * 16.0);
        let clusters = (h0 + h1 + h2) * (0.30 + 1.32 * f0);
        let dm_wells = (exp(-length(p - vec3<f32>(-0.24, 0.14, 0.04)) * 7.5) + exp(-length(p - vec3<f32>(0.22, -0.14, 0.06)) * 7.5) + exp(-r * 6.5)) * 0.48 * (0.20 + 1.38 * f1);
        let pop3 = (exp(-length(p - vec3<f32>(-0.18, 0.10, 0.02)) * 34.0) + exp(-length(p - vec3<f32>(0.18, -0.10, 0.04)) * 34.0) + exp(-length(p - vec3<f32>(0.02, 0.04, -0.02)) * 34.0)) * (0.20 + 1.48 * f2);
        let merge_stream = exp(-length(p.yz - vec2<f32>(-0.58 * p.x, 0.12 * p.x)) * 28.0) * smoothstep(0.32, 0.04, abs(p.x)) * (0.18 + 1.42 * f3);
        let uv_shells = (exp(-abs(length(p - vec3<f32>(-0.24, 0.14, 0.04)) - 0.12) * 44.0) + exp(-abs(length(p - vec3<f32>(0.02, 0.04, -0.02)) - 0.14) * 44.0)) * (0.16 + 1.46 * f4);
        dens = (clusters + dm_wells + pop3 + merge_stream + uv_shells) * env_outer;
        neb_col = mix(vec3<f32>(0.36, 0.28, 1.36), vec3<f32>(0.54, 1.12, 1.56), clamp(clusters + pop3, 0.0, 1.0))
            + vec3<f32>(0.68, 1.38, 1.56) * pop3 * g2
            + vec3<f32>(1.18, 0.48, 1.52) * uv_shells * (0.65 + g4);
    } else if (z_idx == 6u) {
        // Z06 Reionization (1906): carved ionized bubbles (F0), neutral HI fog outside bubbles (F1), crisp cyan ionization fronts (F2), percolating bubble bridges (F3), staggered seed sizes (F4)
        let b0 = length(p - vec3<f32>(-0.20, 0.12, 0.04)) - 0.15;
        let b1 = length(p - vec3<f32>(0.20, -0.10, -0.04)) - 0.14;
        let b2 = length(p - vec3<f32>(0.02, -0.22, 0.12)) - 0.11;
        let b3 = length(p - vec3<f32>(0.08, 0.20, -0.10)) - 0.10;
        let d_bub = min(min(b0, b1), min(b2, b3));
        let in_bubble = 1.0 - smoothstep(-0.02, 0.02, d_bub);
        let outside_fog = smoothstep(-0.01, 0.04, d_bub) * smoothstep(0.56, 0.24, r);
        let bub_core = (exp(-length(p - vec3<f32>(-0.20, 0.12, 0.04)) * 22.0) + exp(-length(p - vec3<f32>(0.20, -0.10, -0.04)) * 22.0)) * (0.28 + 1.32 * f0);
        let hi_fog = outside_fog * (0.45 + 0.55 * fbm3(p * 6.0, 2)) * 0.62 * (0.24 + 1.36 * f1);
        let i_front = exp(-abs(d_bub) * 48.0) * (0.24 + 1.48 * f2);
        let perc_bridge = exp(-abs(b0 + b1 + 0.06) * 26.0) * smoothstep(0.28, 0.04, r) * (0.16 + 1.44 * f3);
        let seed_rings = exp(-abs(b2) * 44.0) * 0.85 * (0.16 + 1.42 * f4);
        dens = (bub_core * (0.6 + 0.4 * in_bubble) + hi_fog + i_front + perc_bridge + seed_rings) * env_outer;
        neb_col = mix(vec3<f32>(1.28, 0.52, 0.24), vec3<f32>(0.24, 1.32, 1.54), clamp(i_front + in_bubble, 0.0, 1.0))
            + vec3<f32>(0.28, 1.44, 1.56) * i_front * (0.55 + g2)
            + vec3<f32>(1.18, 0.68, 1.46) * (perc_bridge * g3 + seed_rings * g4);
    } else if (z_idx == 7u) {
        // Z07 Dark Ages (1907): pre-stellar cosmic darkness without stars (F0), neutral H/He mist (F1), Gaussian potential ripples (F2), 21-cm false-color slice (F3), proto-halo compression (F4)
        let dim_void = exp(-r * 5.2) * 0.28 * (0.24 + 1.34 * f0);
        let neutral_mist = (0.35 + 0.65 * fbm3(p * 5.5, 2)) * smoothstep(0.56, 0.14, r) * 0.44 * (0.22 + 1.38 * f1);
        let pot_ripple = exp(-abs(cos(r * 22.0 + snoise3(p * 4.0))) * 14.0) * smoothstep(0.54, 0.12, r) * 0.62 * (0.18 + 1.44 * f2);
        let slice_21cm = exp(-abs(p.y) * 44.0) * smoothstep(0.52, 0.10, length(p.xz)) * (0.45 + 0.55 * sin(snoise3(p * 8.0) * 6.0)) * (0.14 + 1.48 * f3);
        let proto_comp = (exp(-length(p - vec3<f32>(-0.18, 0.10, 0.06)) * 16.0) + exp(-length(p - vec3<f32>(0.20, -0.10, -0.05)) * 16.0)) * 0.65 * (0.16 + 1.42 * f4);
        dens = (dim_void + neutral_mist + pot_ripple + slice_21cm + proto_comp) * env_outer;
        neb_col = mix(vec3<f32>(0.42, 0.28, 0.78), vec3<f32>(0.96, 0.56, 0.24), clamp(neutral_mist * 1.4 + dim_void, 0.0, 1.0))
            + vec3<f32>(0.24, 1.22, 1.36) * slice_21cm * (0.75 + g3)
            + vec3<f32>(0.92, 0.44, 1.32) * (pot_ripple * g2 + proto_comp * g4) * 0.65;
    } else if (z_idx == 8u) {
        // Z08 Cosmic Microwave Background (1908): Thomson last-scattering surface (F0), clearing transparency ring (F1), Planck +-200uK anisotropy patches (F2), observer time-shell meridians (F3), 3000K->microwave redshift (F4)
        let shell_r = 0.42;
        let ls_shell = exp(-abs(r - shell_r) * 38.0) * (0.30 + 1.34 * f0);
        let clear_ring = exp(-abs(r - 0.32) * 44.0) * (0.16 + 1.42 * f1);
        let aniso_n = snoise3(normalize(p + vec3<f32>(1e-4)) * 5.2);
        let planck_patches = exp(-abs(r - shell_r) * 30.0) * smoothstep(0.05, 0.65, abs(aniso_n)) * (0.20 + 1.48 * f2);
        let mer = exp(-abs(sin(p.y * 14.0)) * 22.0) + exp(-abs(sin(atan2(p.z, p.x) * 6.0)) * 18.0);
        let mer_shell = exp(-abs(r - shell_r) * 44.0) * clamp(mer, 0.0, 1.2) * (0.16 + 1.44 * f3);
        let shift_rays = pow(max(0.0, cos(atan2(p.z, p.x) * 5.0)), 6.0) * exp(-abs(p.y) * 20.0) * smoothstep(0.14, 0.40, r) * (1.0 - smoothstep(0.42, 0.50, r)) * (0.14 + 1.44 * f4);
        dens = (ls_shell + clear_ring + planck_patches + mer_shell + shift_rays) * env_outer;
        let cmb_hot = mix(vec3<f32>(0.24, 0.62, 1.48), vec3<f32>(1.52, 0.68, 0.24), smoothstep(-0.35, 0.35, aniso_n));
        neb_col = cmb_hot * (ls_shell + planck_patches * (0.6 + g2))
            + vec3<f32>(0.34, 1.28, 1.36) * (clear_ring * g1 + mer_shell * g3)
            + vec3<f32>(1.48, 0.44, 0.22) * shift_rays * (0.55 + g4);
    } else if (z_idx == 9u) {
        // Z09 Cosmological Horizon (1909): causal particle horizon sphere (F0), external continuation beyond r=0.42 (F1), expanding comoving grid (F2), porous open wireframe (F3), pre-CMB GW-pierced plasma (F4)
        let h_r = 0.42;
        let causal_shell = exp(-abs(r - h_r) * 42.0) * (0.30 + 1.34 * f0);
        let beyond = (fil_base * 0.92 + node_base * 0.72) * smoothstep(h_r - 0.02, h_r + 0.05, r) * (0.20 + 1.46 * f1);
        let comov_grid = (exp(-abs(sin(p.x * 16.0)) * 20.0) + exp(-abs(sin(p.y * 16.0)) * 20.0)) * exp(-abs(r - 0.32) * 14.0) * (0.16 + 1.42 * f2);
        let porous_wire = exp(-abs(r - h_r) * 44.0) * (exp(-abs(sin(atan2(p.z, p.x) * 6.0)) * 18.0) + exp(-abs(sin(p.y * 16.0)) * 18.0)) * (0.18 + 1.44 * f3);
        let pre_cmb = exp(-abs(r - 0.41) * 32.0) * 0.75 + exp(-abs(p.y) * 28.0) * pow(max(0.0, cos(r * 38.0 - time * 1.6)), 6.0) * smoothstep(0.32, 0.56, r);
        let gw_layer = pre_cmb * (0.16 + 1.46 * f4);
        dens = (causal_shell + beyond + comov_grid + porous_wire + gw_layer + fil_base * 0.36 * smoothstep(0.42, 0.14, r)) * env_outer;
        neb_col = mix(vec3<f32>(0.28, 0.84, 1.48), vec3<f32>(1.46, 0.92, 0.38), clamp(causal_shell + gw_layer * 0.6, 0.0, 1.0))
            + vec3<f32>(0.88, 0.54, 1.48) * beyond * (0.65 + g1)
            + vec3<f32>(0.24, 1.36, 1.24) * gw_layer * (0.55 + g4);
    } else {
        // Z10 Observable Limit (1910): nested complete observable sphere (F0), seamless exterior grid (F1), 13.8 Gyr clock dial (F2), opaque EM plasma wall (F3), piercing GW/BBN probes (F4)
        let inner_obs = (fil_base * 0.72 + node_base * 0.65) * smoothstep(0.40, 0.12, r) * (0.30 + 1.30 * f0);
        let cont_grid = (exp(-abs(sin(p.x * 14.0)) * 18.0) + exp(-abs(sin(p.z * 14.0)) * 18.0)) * smoothstep(0.36, 0.52, r) * (0.18 + 1.44 * f1);
        let clock_arc = exp(-abs(r - 0.34) * 44.0) * (0.45 + 0.55 * pow(max(0.0, cos(atan2(p.y, p.x) * 12.0)), 8.0)) * (0.18 + 1.44 * f2);
        let em_wall = exp(-abs(r - 0.46) * 34.0) * (0.65 + 0.35 * snoise3(p * 6.0)) * (0.22 + 1.42 * f3);
        let gw_probes = exp(-length(p.yz) * 28.0) * pow(max(0.0, cos(abs(p.x) * 38.0 - time * 1.8)), 4.0) * smoothstep(0.28, 0.56, abs(p.x)) * (0.16 + 1.48 * f4);
        dens = (inner_obs + cont_grid + clock_arc + em_wall + gw_probes) * env_outer;
        neb_col = mix(vec3<f32>(0.34, 0.78, 1.48), vec3<f32>(1.52, 0.78, 0.32), clamp(em_wall * 1.15, 0.0, 1.0))
            + vec3<f32>(1.44, 1.12, 0.38) * clock_arc * (0.65 + g2)
            + vec3<f32>(0.24, 1.42, 1.34) * gw_probes * (0.65 + g4);
    }

    return doc_cosmo_finish(neb_col, dens);
}

fn shade_doc_early_universe_stage(
    vid: u32,
    p: vec3<f32>,
    r: f32,
    inst: HeroVolumeInstance,
    time: f32,
) -> vec4<f32> {
    if (r > 0.76) {
        return vec4<f32>(0.0);
    }
    let is_overview = vid == 10u;
    let z_idx = select(
        select(clamp(doc_zone(inst), 0u, 8u), 7u, doc_zone(inst) > 8u),
        vid - 2000u,
        vid >= 2000u && vid <= 2008u
    );
    let f0 = doc_feat_focus(inst, 0u);
    let f1 = doc_feat_focus(inst, 1u);
    let f2 = doc_feat_focus(inst, 2u);
    let f3 = doc_feat_focus(inst, 3u);
    let f4 = doc_feat_focus(inst, 4u);
    let f5 = doc_feat_focus(inst, 5u);
    let g0 = doc_solar_feat_gate(inst, 0u);
    let g1 = doc_solar_feat_gate(inst, 1u);
    let g2 = doc_solar_feat_gate(inst, 2u);
    let g3 = doc_solar_feat_gate(inst, 3u);
    let g4 = doc_solar_feat_gate(inst, 4u);
    let g5 = doc_solar_feat_gate(inst, 5u);

    let env_outer = (1.0 - smoothstep(0.48, 0.72, r)) * 0.76;
    // Distributed centerless metric cell coordinates (zero Big-Bang central explosion core in Z01..=Z08!)
    let cell_p = fract(p * 3.2 + vec3<f32>(0.5)) - vec3<f32>(0.5);
    let cell_d = length(cell_p);
    let qgp_rot = doc_rot_y(p, time * 0.4 + r * 2.5);
    var dens: f32 = 0.0;
    var neb_col = mix(vec3<f32>(0.38, 0.72, 1.48), vec3<f32>(1.52, 0.76, 0.34), clamp(r * 1.3, 0.0, 1.0));

    if (is_overview) {
        // Overview fallback (vid == 10u): centerless exponentially stretched inflation lattice + tensor shear waves
        let grid_x = exp(-abs(sin(p.x * 16.0 - time * 0.5)) * 18.0);
        let grid_y = exp(-abs(sin(p.y * 16.0)) * 18.0);
        let grid_z = exp(-abs(sin(p.z * 16.0)) * 18.0);
        let lattice = (grid_x * grid_y + grid_y * grid_z + grid_x * grid_z) * 0.55;
        let q_fluc = pow(smoothstep(0.08, 0.0, abs(snoise3(qgp_rot * 5.2))), 2.2) * 0.85;
        let gw_wave = exp(-abs(sin((p.x * p.x - p.y * p.y) * 18.0 - time * 1.4)) * 14.0) * 0.42;
        dens = (lattice + q_fluc + gw_wave) * smoothstep(0.62, 0.14, r) * env_outer;
        neb_col = mix(vec3<f32>(0.54, 0.36, 1.52), vec3<f32>(1.52, 0.88, 0.34), clamp(q_fluc * 1.1, 0.0, 1.0));
        return doc_cosmo_finish(neb_col, dens);
    }

    if (z_idx == 0u) {
        // Z00 Cosmic Microwave Background (2000): fossil microwave shell (F0), Planck hotspots (F1), E/B-mode polarization ticks (F2), photon escape rays (F3), inward time-reversal ruler (F4)
        let shell = exp(-abs(r - 0.46) * 36.0) * (0.30 + 1.32 * f0);
        let aniso = snoise3(normalize(p + vec3<f32>(1e-4)) * 5.8);
        let hotspots = exp(-abs(r - 0.46) * 28.0) * smoothstep(0.08, 0.65, abs(aniso)) * (0.20 + 1.46 * f1);
        let pol_ticks = exp(-abs(r - 0.46) * 44.0) * pow(max(0.0, cos(atan2(p.z, p.x) * 12.0 + p.y * 18.0)), 6.0) * (0.18 + 1.46 * f2);
        let esc_rays = pow(max(0.0, cos(atan2(p.y, p.x) * 7.0)), 8.0) * exp(-abs(p.z) * 18.0) * smoothstep(0.14, 0.44, r) * (0.16 + 1.44 * f3);
        let time_ruler = exp(-length(p.yz - vec2<f32>(-0.22, 0.0)) * 38.0) * smoothstep(0.02, 0.44, abs(p.x)) * (0.45 + 0.55 * pow(max(0.0, cos(p.x * 38.0)), 8.0)) * (0.14 + 1.46 * f4);
        dens = (shell + hotspots + pol_ticks + esc_rays + time_ruler) * env_outer;
        neb_col = mix(vec3<f32>(0.26, 0.66, 1.52), vec3<f32>(1.52, 0.72, 0.24), smoothstep(-0.32, 0.32, aniso)) * (shell + hotspots * (0.6 + g1))
            + vec3<f32>(0.28, 1.38, 1.24) * (pol_ticks * (0.55 + g2) + esc_rays * g3)
            + vec3<f32>(1.48, 1.06, 0.34) * time_ruler * (0.55 + g4);
    } else if (z_idx == 1u) {
        // Z01 Recombination (2001): centerless lattice of neutral atoms with orbitals (F0), 75% H vs 25% He nuclei (F1), decoupling photon tracks (F2), clearing plasma fog (F3), omnidirectional slice (F4)
        let orbitals = (exp(-cell_d * 18.0) + exp(-abs(cell_d - 0.16) * 42.0)) * smoothstep(0.54, 0.16, r) * (0.28 + 1.34 * f0);
        let he_sites = exp(-length(fract(p * 1.9 + vec3<f32>(0.3)) - vec3<f32>(0.5)) * 15.0) * smoothstep(0.54, 0.16, r) * (0.20 + 1.44 * f1);
        let rays = pow(max(0.0, sin(p.y * 16.0 + 0.6 * cos(p.z * 12.0))), 8.0) * smoothstep(0.52, 0.12, r) * (0.18 + 1.46 * f2);
        let fog_clear = (1.0 - smoothstep(0.18, 0.44, abs(p.x))) * 0.48 * (0.45 + 0.55 * snoise3(p * 5.0)) * (0.18 + 1.42 * f3);
        let slice_ret = (exp(-abs(r - 0.38) * 42.0) + exp(-abs(r - 0.50) * 42.0)) * (0.16 + 1.44 * f4);
        dens = (orbitals + he_sites + rays + fog_clear + slice_ret) * env_outer;
        neb_col = mix(vec3<f32>(0.24, 1.18, 1.52), vec3<f32>(1.48, 0.94, 0.34), clamp(he_sites * 1.2 + fog_clear, 0.0, 1.0))
            + vec3<f32>(0.28, 1.34, 1.54) * orbitals * g0 * 0.45
            + vec3<f32>(1.52, 0.88, 0.32) * he_sites * g1 * 0.55
            + vec3<f32>(1.52, 1.18, 0.42) * rays * (0.55 + g2)
            + vec3<f32>(0.42, 1.38, 1.34) * slice_ret * g4;
    } else if (z_idx == 2u) {
        // Z02 Primordial Plasma (2002): free e-/p+ sea (F0), centerless metric grid (F1), Brownian photon random-walk (F2), concentric BAO sound rings (F3), opaque thermal haze (F4)
        let sea = (exp(-cell_d * 20.0) * 0.72 + pow(max(0.0, snoise3(p * 12.0)), 4.0) * 0.55) * smoothstep(0.56, 0.14, r) * (0.28 + 1.32 * f0);
        let grid = (exp(-abs(sin(p.x * 16.0)) * 22.0) * exp(-abs(sin(p.y * 16.0)) * 22.0) + exp(-abs(sin(p.z * 16.0)) * 22.0) * exp(-abs(sin(p.x * 16.0)) * 22.0)) * smoothstep(0.52, 0.14, r) * (0.18 + 1.44 * f1);
        let walk = pow(smoothstep(0.09, 0.0, abs(snoise3(p * 11.0 + vec3<f32>(time * 0.25, -time * 0.18, 0.0)))), 2.4) * smoothstep(0.54, 0.14, r) * (0.20 + 1.44 * f2);
        let bao = exp(-abs(cos(r * 26.0 - time * 1.5)) * 18.0) * smoothstep(0.54, 0.14, r) * (0.18 + 1.46 * f3);
        let haze = smoothstep(0.52, 0.16, r) * (0.32 + 0.35 * snoise3(p * 4.5)) * (0.18 + 1.38 * f4);
        dens = (sea + grid + walk + bao + haze) * env_outer;
        neb_col = mix(vec3<f32>(1.48, 0.78, 0.28), vec3<f32>(0.34, 1.08, 1.54), clamp(sea * 0.8 + grid * 0.9, 0.0, 1.0))
            + vec3<f32>(1.54, 1.18, 0.44) * (walk * (0.55 + g2) + haze * g4)
            + vec3<f32>(0.38, 1.38, 1.32) * bao * (0.55 + g3);
    } else if (z_idx == 3u) {
        // Z03 Nucleosynthesis BBN (2003): distributed p+ spheres (F0), slate-blue neutrons (F1), p+n->D+gamma flashes (F2), tetrahedral He-4 clusters (F3), rare Li-7 + tension ring (F4)
        let protons = exp(-cell_d * 20.0) * smoothstep(0.54, 0.16, r) * (0.28 + 1.32 * f0);
        let n_cell = length(fract(p * 2.4 + vec3<f32>(0.22, -0.18, 0.14)) - vec3<f32>(0.5));
        let neutrons = exp(-n_cell * 22.0) * smoothstep(0.54, 0.16, r) * (0.20 + 1.44 * f1);
        let d_pair = exp(-abs(cell_d - 0.12) * 38.0) * exp(-abs(p.y - 0.05 * sin(p.x * 10.0)) * 22.0) * smoothstep(0.52, 0.14, r) * (0.18 + 1.46 * f2);
        let he_p = p - vec3<f32>(-0.18, 0.08, 0.04);
        let he4 = (exp(-length(he_p - vec3<f32>(0.03, 0.03, 0.03)) * 28.0) + exp(-length(he_p - vec3<f32>(-0.03, -0.03, 0.03)) * 28.0)
            + exp(-length(he_p - vec3<f32>(-0.03, 0.03, -0.03)) * 28.0) + exp(-length(he_p - vec3<f32>(0.03, -0.03, -0.03)) * 28.0)) * (0.20 + 1.48 * f3);
        let li_p = p - vec3<f32>(0.22, -0.12, -0.02);
        let li7 = (exp(-length(li_p) * 20.0) + exp(-abs(length(li_p) - 0.11) * 46.0)) * (0.16 + 1.50 * f4);
        dens = (protons + neutrons + d_pair + he4 + li7) * env_outer;
        neb_col = vec3<f32>(1.48, 0.72, 0.28) * protons * (0.65 + g0)
            + vec3<f32>(0.38, 0.72, 1.48) * neutrons * (0.65 + g1)
            + vec3<f32>(1.54, 1.24, 0.42) * (d_pair * (0.6 + g2) + he4 * (0.6 + g3))
            + vec3<f32>(1.44, 0.36, 1.18) * li7 * (0.65 + g4);
    } else if (z_idx == 4u) {
        // Z04 Lepton Era (2004): spiralling e- tracks (F0), e+ annihilation vertices (F1), straight neutrino streaks (F2), excess matter survival (F3), CvB decoupling shell (F4)
        let spir_a = exp(-abs(length(p.xy) - (0.18 + 0.06 * sin(atan2(p.y, p.x) * 3.0 + p.z * 14.0))) * 34.0) * smoothstep(0.48, 0.08, abs(p.z)) * (0.28 + 1.34 * f0);
        let spir_b = exp(-abs(length(p.xy) - (0.24 - 0.06 * sin(atan2(p.y, p.x) * 3.0 - p.z * 14.0))) * 34.0) * smoothstep(0.48, 0.08, abs(p.z)) * (0.20 + 1.46 * f1);
        let annih = exp(-length(p - vec3<f32>(0.18, 0.0, 0.0)) * 22.0) + exp(-length(p - vec3<f32>(-0.18, 0.0, 0.0)) * 22.0);
        let neut = exp(-abs(p.y - 0.45 * p.x) * 36.0) * exp(-abs(p.z) * 24.0) * smoothstep(0.54, 0.04, abs(p.x)) * (0.18 + 1.46 * f2);
        let excess = exp(-length(p - vec3<f32>(0.0, 0.24, 0.06)) * 18.0) * (0.18 + 1.44 * f3);
        let cvb_shell = exp(-abs(r - 0.44) * 38.0) * (0.18 + 1.46 * f4);
        dens = (spir_a + spir_b + annih * (0.35 + f1) + neut + excess + cvb_shell) * env_outer;
        neb_col = vec3<f32>(0.24, 1.18, 1.54) * spir_a * (0.6 + g0)
            + vec3<f32>(1.48, 0.36, 0.96) * spir_b * (0.6 + g1)
            + vec3<f32>(1.56, 1.24, 0.42) * annih * (0.5 + g1)
            + vec3<f32>(0.76, 0.42, 1.54) * (neut * (0.65 + g2) + cvb_shell * g4);
    } else if (z_idx == 5u) {
        // Z05 Quark Era QGP (2005): RGB unconfined quarks (F0), gluon flux springs (F1), swirling QGP perfect liquid (F2), hadronization R+G+B white bags (F3), collider/lattice slice (F4)
        let q_r = exp(-length(p - vec3<f32>(-0.18, 0.12, 0.04)) * 22.0);
        let q_g = exp(-length(p - vec3<f32>(0.20, 0.08, -0.06)) * 22.0);
        let q_b = exp(-length(p - vec3<f32>(0.02, -0.20, 0.05)) * 22.0);
        let quarks = (q_r + q_g + q_b) * (0.30 + 1.36 * f0);
        let gluon = pow(smoothstep(0.08, 0.0, abs(snoise3(qgp_rot * 6.5))), 2.2) * smoothstep(0.52, 0.14, r) * (0.22 + 1.42 * f1);
        let swirl = exp(-abs(length(qgp_rot.xz) - 0.28) * 26.0) * exp(-abs(qgp_rot.y) * 16.0) * (0.55 + 0.45 * sin(atan2(qgp_rot.z, qgp_rot.x) * 4.0 + time)) * (0.18 + 1.44 * f2);
        let bag = (exp(-abs(length(p - vec3<f32>(-0.22, -0.14, 0.04)) - 0.08) * 44.0) + exp(-length(p - vec3<f32>(-0.22, -0.14, 0.04)) * 22.0)) * (0.18 + 1.46 * f3);
        let lat_grid = (exp(-abs(sin(p.x * 20.0)) * 22.0) + exp(-abs(sin(p.y * 20.0)) * 22.0)) * exp(-abs(p.z) * 28.0) * smoothstep(0.52, 0.12, r) * (0.14 + 1.46 * f4);
        dens = (quarks + gluon + swirl + bag + lat_grid) * env_outer;
        neb_col = vec3<f32>(1.54, 0.34, 0.28) * q_r
            + vec3<f32>(0.32, 1.52, 0.46) * q_g
            + vec3<f32>(0.28, 0.72, 1.56) * q_b
            + vec3<f32>(1.28, 0.56, 1.48) * (gluon * (0.55 + g1) + swirl * g2)
            + vec3<f32>(1.42, 1.38, 1.28) * (bag * (0.65 + g3) + lat_grid * g4);
    } else if (z_idx == 6u) {
        // Z06 Electroweak (2006): 3D Higgs Mexican-hat potential V(phi) (F0), SU(2)xU(1)->U(1)_EM bifurcation (F1), heavy W/Z bosons (F2), massless photons (F3), TeV frontier bar (F4)
        let hr = length(p.xz);
        let v_hat = 1.8 * hr * hr * hr * hr - 0.56 * hr * hr + 0.04;
        let hat_surf = exp(-abs(p.y + 0.06 - clamp(v_hat, -0.16, 0.32)) * 38.0) * smoothstep(0.48, 0.12, hr) * (0.28 + 1.36 * f0);
        let sym_top = exp(-abs(length(p.xz) - 0.12) * 38.0) * exp(-abs(p.y - 0.22) * 32.0);
        let forks = (exp(-length(vec2<f32>(p.x - 0.12, p.z)) * 28.0) + exp(-length(vec2<f32>(p.x + 0.12, p.z)) * 28.0)) * smoothstep(-0.22, 0.22, p.y);
        let sym_split = (sym_top + forks * 0.75) * (0.20 + 1.46 * f1);
        let wz = (exp(-length(p - vec3<f32>(-0.26, -0.08, 0.10)) * 22.0) + exp(-length(p - vec3<f32>(0.26, -0.08, -0.10)) * 22.0)) * (0.20 + 1.48 * f2);
        let gamma_rays = exp(-length(p.yz - vec2<f32>(0.16, 0.0)) * 34.0) * smoothstep(0.50, 0.04, abs(p.x)) * (0.18 + 1.46 * f3);
        let tev_bar = exp(-length(p.yz - vec2<f32>(-0.26, 0.0)) * 38.0) * smoothstep(0.50, 0.04, abs(p.x)) * (0.14 + 1.48 * f4);
        dens = (hat_surf + sym_split + wz + gamma_rays + tev_bar) * env_outer;
        neb_col = mix(vec3<f32>(0.78, 0.38, 1.52), vec3<f32>(1.52, 1.14, 0.34), clamp(hat_surf * 1.15 + g0 * 0.3, 0.0, 1.0))
            + vec3<f32>(1.48, 0.68, 0.28) * wz * (0.65 + g2)
            + vec3<f32>(0.28, 1.38, 1.56) * (gamma_rays * g3 + tev_bar * g4 + sym_split * g1 * 0.5);
    } else if (z_idx == 7u) {
        // Z07 Inflation (2007, 6 features F0..=F5): exponential lattice (F0), quantum->macro fluctuations (F1), separated horizon patches (F2), flattened geometry (F3), indirect slow-roll curve (F4), tensor GW shear (F5)
        let gx = exp(-abs(sin(p.x * 15.0 - time * 0.45)) * 22.0);
        let gy = exp(-abs(sin(p.y * 15.0)) * 22.0);
        let gz = exp(-abs(sin(p.z * 15.0)) * 22.0);
        let exp_lat = (gx * gy + gy * gz + gx * gz) * smoothstep(0.58, 0.14, r) * 0.55 * (0.28 + 1.34 * f0);
        let q_seeds = pow(smoothstep(0.09, 0.0, abs(snoise3(p * 4.8))), 2.1) * smoothstep(0.56, 0.12, r) * (0.22 + 1.44 * f1);
        let patches = (exp(-length(p - vec3<f32>(-0.28, 0.12, 0.0)) * 16.0) + exp(-length(p - vec3<f32>(0.28, -0.10, 0.04)) * 16.0) + exp(-length(p - vec3<f32>(0.0, 0.24, -0.12)) * 16.0)) * (0.20 + 1.46 * f2);
        let flat_plane = exp(-abs(p.y) * 38.0) * smoothstep(0.54, 0.14, length(p.xz)) * (0.45 + 0.55 * max(gx, gz)) * (0.18 + 1.44 * f3);
        let slow_roll = exp(-abs(p.y - (0.18 - 0.55 * smoothstep(-0.30, 0.30, p.x))) * 40.0) * exp(-abs(p.z) * 24.0) * pow(0.5 + 0.5 * cos(p.x * 26.0), 3.0) * smoothstep(0.46, 0.04, abs(p.x)) * (0.14 + 1.48 * f4);
        let gw_shear = exp(-abs(sin((p.x * p.x - p.y * p.y) * 20.0 - time * 1.4)) * 18.0) * smoothstep(0.56, 0.14, r) * (0.16 + 1.50 * f5);
        dens = (exp_lat + q_seeds + patches + flat_plane + slow_roll + gw_shear) * env_outer;
        neb_col = mix(vec3<f32>(0.44, 0.56, 1.52), vec3<f32>(1.52, 0.96, 0.34), clamp(q_seeds * 1.2 + patches * 0.7, 0.0, 1.0))
            + vec3<f32>(0.34, 1.28, 1.48) * (exp_lat * g0 + flat_plane * g3)
            + vec3<f32>(1.38, 0.44, 1.48) * (slow_roll * g4 + gw_shear * (0.65 + g5));
    } else {
        // Z08 Known-Physics Limit (2008): grid->spin-foam dissolution (F0), Planck resolution blur (F1), 3 hypothetical branches (F2), open centerless horizon without singularity (F3), open epistemic wisps (F4)
        let foam_n = abs(snoise3(p * 14.0 + vec3<f32>(0.0, time * 0.25, 0.0)));
        let spin_foam = pow(smoothstep(0.14, 0.0, foam_n), 2.0) * smoothstep(0.56, 0.14, r) * (0.28 + 1.34 * f0);
        let p_blur = smoothstep(0.48, 0.14, r) * (0.26 + 0.36 * snoise3(p * 9.0)) * (0.18 + 1.42 * f1);
        let br_a = exp(-abs(p.y - 0.55 * max(p.x, 0.0)) * 32.0) * smoothstep(-0.04, 0.42, p.x);
        let br_b = exp(-abs(p.y + 0.55 * max(p.x, 0.0)) * 32.0) * smoothstep(-0.04, 0.42, p.x);
        let br_c = exp(-abs(p.z - 0.50 * max(p.x, 0.0)) * 32.0) * smoothstep(-0.04, 0.42, p.x);
        let branches = (br_a + br_b + br_c) * (0.18 + 1.46 * f2);
        let no_sing = exp(-abs(r - 0.32) * 38.0) * (0.20 + 1.44 * f3);
        let q_wisps = exp(-abs(length(p - vec3<f32>(0.0, 0.14, 0.0)) - 0.22) * 36.0) * smoothstep(-0.08, 0.30, p.y) * (0.16 + 1.46 * f4);
        dens = (spin_foam + p_blur + branches + no_sing + q_wisps) * env_outer;
        neb_col = mix(vec3<f32>(0.42, 0.46, 1.48), vec3<f32>(0.28, 1.28, 1.46), clamp(spin_foam + p_blur, 0.0, 1.0))
            + vec3<f32>(1.48, 0.88, 0.34) * branches * (0.65 + g2)
            + vec3<f32>(1.28, 0.48, 1.52) * (no_sing * g3 + q_wisps * (0.65 + g4));
    }

    let ef = doc_feat(inst);
    if (ef <= 5u) {
        let fa = f32(ef) * 1.0471975 + f32(z_idx) * 0.45;
        let fc = vec3<f32>(cos(fa) * 0.28, sin(fa) * 0.22, 0.08 * sin(fa * 2.0));
        let f_node = exp(-length(p - fc) * 12.0) + exp(-abs(length(p.xy - fc.xy) - 0.12) * 38.0) * exp(-abs(p.z) * 18.0);
        dens += f_node * 0.55 * env_outer;
        neb_col += hsl_to_rgb(fract(0.08 + f32(ef) * 0.16 + f32(z_idx) * 0.07), 0.92, 0.56) * f_node * 0.95;
    }

    return doc_cosmo_finish(neb_col, dens);
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
                        let orb_az = atan2(p.z, p.x);
                        for (var oi: u32 = 0u; oi < 11u; oi = oi + 1u) {
                            let d_world = abs(r_canon - orbits[oi]) * max_orb_w;
                            let k = d_world * inv_sig;
                            if (k < 3.6) {
                                let merc_pulse = select(1.0, 0.85 + 0.65 * pow(max(0.0, cos(orb_az - time * 1.85)), 4.0), oi == 0u && z_idx == 1u);
                                let act_pulse = select(1.0, 1.05 + 0.32 * sin(orb_az * 2.0 - time * 0.9), z_idx >= 1u && oi == z_idx - 1u);
                                let w_boost = select(0.76, 1.0, oi == 2u || oi == 6u || (z_idx >= 1u && oi == z_idx - 1u));
                                line_sum += exp(-(k * k)) * w_boost * merc_pulse * act_pulse;
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
                let sub_m = doc_submode(inst);
                let is_saturn = (vid == 1407u) || (vid == 2u);
                let is_uranus = (vid == 1408u) || (vid == 1u && sub_m == 8u) || (vid == 1u && sub_m == 0u);
                let is_asteroid = (vid == 1405u) || (vid == 1u && sub_m == 5u);
                let is_kuiper = (vid == 1410u) || (vid == 1u && sub_m == 10u);
                let r_min = select(select(0.43, 0.56, is_uranus), 0.39, is_asteroid || is_kuiper);
                if (r > r_min && r < 0.97) {
                    let az = atan2(p.z, p.x);
                    let ring_n = fbm3(vec3<f32>(r * 42.0, sin(p.x * 4.0) * 1.5, cos(p.z * 4.0) * 1.5), 3);
                    let sun_dir_n = normalize(sun_local + vec3<f32>(1e-5));
                    let proj = dot(p, sun_dir_n);
                    let perp = length(p - sun_dir_n * proj);
                    let planet_shadow = select(1.0, smoothstep(0.36, 0.43, perp), proj < 0.0);
                    let shadow_term = select(1.0, 0.08 + 0.92 * planet_shadow, is_saturn || is_uranus);
                    if (is_asteroid) {
                        // Main Asteroid Belt (1405): 3:1, 5:2, 2:1 Jovian Kirkwood Resonance Gaps
                        let k31 = smoothstep(0.014, 0.036, abs(r - 0.55));
                        let k52 = smoothstep(0.013, 0.033, abs(r - 0.68));
                        let k21 = smoothstep(0.016, 0.038, abs(r - 0.82));
                        let kirkwood = k31 * k52 * k21;
                        let rubble = 0.50 + 0.50 * fbm3(vec3<f32>(p.x * 16.0, r * 24.0, p.z * 16.0), 3);
                        let ceres_track = exp(-abs(r - 0.61) * 46.0) * doc_feat_focus(inst, 4u);
                        let f_res = mix(1.0, 0.35 + 1.15 * kirkwood, select(0.5, 1.0, doc_feat(inst) == 3u));
                        let alpha = smoothstep(r_min, r_min + 0.04, r) * (1.0 - smoothstep(0.91, 0.97, r))
                            * kirkwood * (0.28 + 0.58 * rubble) * doc_feat_focus(inst, 1u) * f_res;
                        let belt_col = mix(vec3<f32>(0.68, 0.54, 0.40), vec3<f32>(1.12, 0.86, 0.62), rubble)
                            + vec3<f32>(0.42, 0.36, 0.28) * ceres_track;
                        return vec4<f32>(belt_col * alpha * inst.color_a.a, clamp(alpha * 0.72, 0.0, 0.82));
                    } else if (is_kuiper) {
                        // Kuiper Belt (1410): Cold 3:2 Plutino Resonance (F3) & Scattered Icy Planetesimals (F0)
                        let r32 = 0.45 + 0.95 * exp(-abs(r - 0.59) * 28.0) * doc_feat_focus(inst, 3u)
                            * select(1.0, 0.55 + 0.95 * pow(0.5 + 0.5 * cos(az * 3.0), 3.0), doc_feat(inst) == 3u);
                        let edge21 = 1.0 - 0.55 * exp(-abs(r - 0.84) * 32.0);
                        let frost_specks = 0.42 + 0.58 * pow(max(0.0, ring_n + 0.25), 1.4);
                        let kbo_gate = max(doc_feat_focus(inst, 0u), doc_feat_focus(inst, 3u) * 0.92);
                        let alpha = smoothstep(r_min, r_min + 0.04, r) * (1.0 - smoothstep(0.91, 0.97, r))
                            * r32 * edge21 * frost_specks * kbo_gate;
                        let kbo_col = mix(vec3<f32>(0.46, 0.70, 1.08), vec3<f32>(0.84, 0.96, 1.24), frost_specks)
                            + vec3<f32>(0.24, 0.68, 0.98) * select(0.0, exp(-abs(r - 0.59) * 32.0), doc_feat(inst) == 3u);
                        return vec4<f32>(kbo_col * alpha * inst.color_a.a, clamp(alpha * 0.68, 0.0, 0.78));
                    } else if (is_uranus) {
                        // Uranus (1408): Dark Charcoal Epsilon & Inner Shepherd Ringlets (catalog feature F2)
                        let r_inner = exp(-abs(r - 0.62) * 110.0) + exp(-abs(r - 0.71) * 120.0);
                        let r_eps = exp(-abs(r - 0.85) * 85.0) * (1.15 + 0.35 * sin(az * 2.0 + time * 0.3));
                        let ring_band = min(r_inner * 0.65 + r_eps * 1.15, 1.35);
                        let f_ring = select(1.0, 1.35, doc_feat(inst) == 2u || doc_feat(inst) == 255u);
                        let alpha = smoothstep(r_min, r_min + 0.02, r) * (1.0 - smoothstep(0.92, 0.97, r))
                            * ring_band * 0.38 * f_ring * doc_feat_focus(inst, 2u);
                        let ring_col = mix(vec3<f32>(0.34, 0.40, 0.48), vec3<f32>(0.58, 0.66, 0.76), clamp(r_eps, 0.0, 1.0)) * shadow_term;
                        return vec4<f32>(ring_col * alpha * inst.color_a.a, clamp(alpha * 0.78, 0.0, 0.85));
                    } else {
                        // Saturn (1407): Cassini/Encke Divisions, B-Ring Spiral Waves & E-Ring Ice Sheath
                        let cassini_gap = smoothstep(0.018, 0.042, abs(r - 0.71));
                        let encke_gap = smoothstep(0.007, 0.016, abs(r - 0.88));
                        let spiral_wave = sin(r * 155.0 + az * 2.0 * doc_feat_focus(inst, 1u) + ring_n * 3.5 + time * 0.35);
                        let fine_bands = 0.52 + 0.48 * spiral_wave;
                        let cassini_sheen = exp(-abs(r - 0.67) * 58.0) * 0.25 * doc_feat_focus(inst, 2u);
                        let e_ring = exp(-abs(r - 0.925) * 38.0) * 0.42 * doc_feat_focus(inst, 4u);
                        let alpha = (smoothstep(r_min, r_min + 0.03, r) * (1.0 - smoothstep(0.93, 0.97, r)) * cassini_gap * encke_gap * fine_bands) + e_ring * 0.32;
                        let ring_col = (mix(inst.color_a.rgb, inst.color_b.rgb, clamp(r, 0.0, 1.0)) * (1.05 + 0.3 * ring_n)
                            + vec3<f32>(0.40, 0.36, 0.28) * cassini_sheen + vec3<f32>(0.42, 0.72, 1.10) * e_ring) * shadow_term;
                        return vec4<f32>(ring_col * alpha * inst.color_a.a, clamp(alpha * 0.9, 0.0, 1.0));
                    }
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
        let is_mw = (vid == 5u) || (vid >= 1500u && vid <= 1512u);
        let mw_stage_z = select(99u, vid - 1500u, vid >= 1500u && vid <= 1512u);

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

            if (is_mw && r > 0.34 && r < 0.94 && (abs(pos.y) < 0.16 || mw_stage_z == 11u || mw_stage_z == 12u)) {
                let gal_r = length(pos.xz);
                let inner_lim = select(0.28, 0.18, mw_stage_z == 10u);
                if (gal_r > inner_lim || mw_stage_z == 11u) {
                    let gal_ang = atan2(pos.z, pos.x);
                    let f_bar = select(1.0, doc_feat_focus(inst, 1u), mw_stage_z == 9u);
                    let f_dust = select(1.0, 1.0 + 0.45 * doc_solar_feat_gate(inst, 2u), mw_stage_z == 9u);
                    let f_halo = select(0.0, doc_feat_focus(inst, 3u), mw_stage_z == 11u || mw_stage_z == 9u);
                    let bar = exp(-pow(abs(sin(gal_ang - 0.36)) * gal_r * 6.2, 2.0)) * smoothstep(0.58, 0.24, gal_r) * f_bar;
                    let sp_a = pow(max(0.0, cos(2.0 * gal_ang - 5.2 * log(gal_r + 0.14))), 3.4);
                    let sp_b = pow(max(0.0, cos(4.0 * gal_ang - 6.8 * log(gal_r + 0.14) + 1.1)), 4.0) * 0.45;
                    let dust = smoothstep(-0.18, 0.34 * f_dust, fbm3(vec3<f32>(pos.x * 7.5, pos.z * 7.5, pos.y * 14.0), 3));
                    let edge_fade = select(1.0, exp(-gal_r * 1.4), mw_stage_z == 12u);
                    let halo_env = select(0.0, exp(-abs(r - 0.82) * 20.0) * 0.28 * f_halo, mw_stage_z == 11u || mw_stage_z == 9u);
                    let mw_d = (((sp_a + sp_b) * smoothstep(0.28, 0.46, gal_r) * (1.0 - smoothstep(0.76, 0.94, gal_r)) * 0.65 + bar * 0.42) * dust * exp(-abs(pos.y) * 28.0) * edge_fade + halo_env) * step_len * 2.2;
                    let mw_c = mix(vec3<f32>(0.32, 0.66, 1.55), vec3<f32>(1.55, 1.02, 0.44), clamp(bar * 1.6 + (0.55 - gal_r) * 0.8, 0.0, 1.0))
                        + vec3<f32>(0.48, 0.30, 1.38) * halo_env * 1.6;
                    accum_rgb += (1.0 - accum_a) * mw_c * mw_d;
                    accum_a += (1.0 - accum_a) * min(mw_d * 0.55, 0.22);
                }
            }
            if (is_mw && mw_stage_z == 10u && r > rs * 1.52 && r < 0.50) {
                let mw_st = shade_doc_milky_way_stage(vid, pos, r, inst, time);
                let st_a = mw_st.a * step_len * 0.38;
                accum_rgb += (1.0 - accum_a) * min(mw_st.rgb, vec3<f32>(1.15)) * st_a;
                accum_a += (1.0 - accum_a) * min(st_a * 0.50, 0.16);
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

    // Kind 3u: Cosmological Macro + Landmarks (vid 4, 6..10, 20..23, 1500..2008, 3660..3709)
    let max_steps = select(30.0, 26.0, vid >= 1500u || (vid >= 8u && vid <= 10u));
    let steps: i32 = i32(mix(16.0, max_steps, fovea_k));
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
            if (vid == 4u || vid == 1400u) {
                let az = atan2(p.z, p.x);
                let is_can_sun = vid == 1400u;
                let f_loops = select(1.0, doc_feat_focus(inst, 3u), is_can_sun);
                let f_wind = select(1.0, doc_feat_focus(inst, 4u), is_can_sun);
                if (r > 0.38 && r < 0.92) {
                    let loop_arc = exp(-abs(r - 0.56 - 0.14 * sin(p.x * 7.5 + time * 0.4) * cos(p.z * 6.5)) * 28.0);
                    let flare = pow(max(0.0, snoise3(p * 7.0 + vec3<f32>(time * 0.25))), 3.0);
                    let parker = pow(max(0.0, sin(az * 7.0 - r * 11.0 + time * 0.85)), 5.0) * smoothstep(0.42, 0.72, r) * (1.0 - smoothstep(0.75, 0.92, r));
                    dens = (loop_arc * (0.20 + 0.95 * flare) * f_loops + parker * 0.34 * f_wind) * (1.0 - smoothstep(0.68, 0.92, r));
                    neb_col = mix(vec3<f32>(1.15, 0.42, 0.10), vec3<f32>(1.55, 0.85, 0.28), max(flare, parker * 0.75));
                }
            } else if (vid == 3647u) {
                // Enceladus (Exp 04 Z07 F4) South-Polar Cryovolcanic Ice Plumes
                if (p.y < -0.10 && r < 0.94) {
                    let plume_r = length(p.xz);
                    let cone_w = 0.09 + (-p.y - 0.10) * 0.42;
                    let cone = exp(-pow(plume_r / max(cone_w, 0.04), 2.0));
                    let jets = 0.62 + 0.38 * sin(atan2(p.z, p.x) * 4.0 + (-p.y) * 14.0 - time * 1.8);
                    let vent = smoothstep(-0.10, -0.30, p.y) * (1.0 - smoothstep(-0.62, -0.92, p.y));
                    dens = cone * jets * vent * 0.85 * doc_feat_focus(inst, 4u);
                    neb_col = mix(vec3<f32>(0.42, 0.78, 1.32), vec3<f32>(0.92, 0.98, 1.28), cone);
                }
            } else if (vid == 3650u) {
                // Kuiper Belt Comet (Exp 04 Z10 F4) Blue CO+ Ion Tail + Curved Amber Dust Tail
                let tail_x = p.x;
                if (tail_x > -0.25 && tail_x < 0.94 && r < 0.95) {
                    let t_dist = max(tail_x + 0.18, 0.0);
                    let coma = exp(-r * 6.5) * 0.72;
                    let ion_axis = length(p.yz - vec2<f32>(0.02 * sin(t_dist * 6.0 - time * 1.6), 0.0));
                    let ion_tail = exp(-pow(ion_axis / (0.04 + t_dist * 0.09), 2.0)) * smoothstep(0.02, 0.16, t_dist) * (1.0 - smoothstep(0.65, 1.05, t_dist));
                    let dust_curve = length(vec2<f32>(p.y - (0.28 * t_dist * t_dist + 0.04 * t_dist), p.z));
                    let dust_tail = exp(-pow(dust_curve / (0.06 + t_dist * 0.16), 2.0)) * smoothstep(0.05, 0.22, t_dist) * (1.0 - smoothstep(0.60, 1.02, t_dist));
                    let f_com = doc_feat_focus(inst, 4u);
                    dens = (coma + ion_tail * 0.68 + dust_tail * 0.56) * f_com;
                    let w_sum = max(coma + ion_tail + dust_tail, 1e-4);
                    let ion_w = ion_tail / w_sum;
                    let dust_w = dust_tail / w_sum;
                    neb_col = mix(vec3<f32>(0.85, 0.96, 1.15), vec3<f32>(0.24, 0.68, 1.48), clamp(ion_w * 1.4, 0.0, 1.0)) * (1.0 - 0.35 * dust_w)
                        + vec3<f32>(1.28, 0.92, 0.44) * dust_w;
                }
            } else if (vid == 6u || (vid >= 1600u && vid <= 1608u) || (vid >= 3680u && vid <= 3688u)) {
                let lg_res = shade_doc_local_group_stage(vid, p, r, inst, time);
                neb_col = lg_res.rgb;
                dens = lg_res.a;
            } else if (vid == 7u || (vid >= 1700u && vid <= 1709u) || (vid >= 3700u && vid <= 3709u)) {
                let lan_res = shade_doc_laniakea_stage(vid, p, r, inst, time);
                neb_col = lan_res.rgb;
                dens = lan_res.a;
            } else if (vid == 8u || (vid >= 1800u && vid <= 1810u)) {
                let web_res = shade_doc_cosmic_web_stage(vid, p, r, inst, time);
                neb_col = web_res.rgb;
                dens = web_res.a;
            } else if (vid == 9u || (vid >= 1900u && vid <= 1910u)) {
                let obs_res = shade_doc_observable_universe_stage(vid, p, r, inst, time);
                neb_col = obs_res.rgb;
                dens = obs_res.a;
            } else if (vid == 20u || vid == 21u || vid == 22u || (vid >= 1500u && vid <= 1512u) || (vid >= 3660u && vid <= 3671u)) {
                let mw_res = shade_doc_milky_way_stage(vid, p, r, inst, time);
                neb_col = mw_res.rgb;
                dens = mw_res.a;
            } else {
                let early_res = shade_doc_early_universe_stage(vid, p, r, inst, time);
                neb_col = early_res.rgb;
                dens = early_res.a;
            }
            let step_a = dens * dt * inst.color_b.a * (0.72 + voice * 0.24);
            accum_rgb += (1.0 - accum_a) * neb_col * min(inst.color_a.a, 1.2) * step_a;
            accum_a += (1.0 - accum_a) * min(step_a * 0.65, 0.38);
        }
    }
    return vec4<f32>(accum_rgb, accum_a);
}
