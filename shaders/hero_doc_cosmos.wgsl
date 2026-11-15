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
        let e4_f = doc_feat(inst);
        let gate4 = select(0.0, 1.0, e4_f != 255u);
        let f4_atmo = smoothstep(0.20, 1.0, doc_feat_focus(inst, 0u) - 1.0) * gate4;
        let f4_ocean = smoothstep(0.20, 1.0, doc_feat_focus(inst, 1u) - 1.0) * gate4;
        let f4_cont = smoothstep(0.20, 1.0, doc_feat_focus(inst, 2u) - 1.0) * gate4;
        let f4_moon = smoothstep(0.20, 1.0, doc_feat_focus(inst, 3u) - 1.0) * gate4;
        let f4_mag = smoothstep(0.20, 1.0, doc_feat_focus(inst, 4u) - 1.0) * gate4;

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

fn doc_solar_feat_gate(inst: HeroVolumeInstance, target_feat: u32) -> f32 {
    if (doc_feat(inst) == 255u) {
        return 0.0;
    }
    return smoothstep(0.20, 1.0, doc_feat_focus(inst, target_feat) - 1.0);
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
        // Z10 Kuiper Belt KBO: dark crimson Cthulhu tholin patches & frosty white-cyan N2/CH4 volatiles under distant pinpoint Sun
        let f0 = doc_solar_feat_gate(inst, 0u);
        let f1 = doc_solar_feat_gate(inst, 1u);
        let f2 = doc_solar_feat_gate(inst, 2u);
        let f3 = doc_solar_feat_gate(inst, 3u);
        let f4 = doc_solar_feat_gate(inst, 4u);
        let th = fbm3(p_surf * 5.2 + vec3<f32>(0.8, -1.4, 2.2), 4);
        let frost = smoothstep(0.06, 0.34, th);
        var kbo = mix(vec3<f32>(0.28, 0.12, 0.10), vec3<f32>(0.84, 0.90, 0.96), frost);
        let ice_glint = pow(ndotl, 12.0) * frost * (0.35 + 0.85 * f0);
        let res_belt = exp(-abs(p_surf.y) * 28.0) * (f1 + f2 + f3 + f4);
        return (kbo * (ndotl * 1.18 + 0.015)
             + vec3<f32>(0.75, 0.98, 1.25) * ice_glint
             + vec3<f32>(0.36, 0.95, 1.48) * res_belt * 0.72) * inst.color_a.a;
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
            // Kuiper dwarf / comet outgassing
            let jet = pow(max(0.0, snoise3(p_surf * 12.0 + vec3<f32>(time * 0.5))), 6.0) * select(0.0, 1.0, sub == 4u);
            m_col += vec3<f32>(0.36, 1.15, 1.65) * jet;
        }
        return (m_col * (ndotl * 1.12 + 0.025) + inst.color_b.rgb * fresnel * 0.35) * inst.color_a.a;
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
                        // Kuiper Belt (1410): Cold 3:2 Plutino Resonance & Scattered Icy Planetesimals
                        let r32 = 0.45 + 0.85 * exp(-abs(r - 0.59) * 28.0) * doc_feat_focus(inst, 1u);
                        let edge21 = 1.0 - 0.55 * exp(-abs(r - 0.84) * 32.0);
                        let frost_specks = 0.42 + 0.58 * pow(max(0.0, ring_n + 0.25), 1.4);
                        let alpha = smoothstep(r_min, r_min + 0.04, r) * (1.0 - smoothstep(0.91, 0.97, r))
                            * r32 * edge21 * frost_specks * doc_feat_focus(inst, 0u);
                        let kbo_col = mix(vec3<f32>(0.46, 0.70, 1.08), vec3<f32>(0.84, 0.96, 1.24), frost_specks);
                        return vec4<f32>(kbo_col * alpha * inst.color_a.a, clamp(alpha * 0.68, 0.0, 0.78));
                    } else if (is_uranus) {
                        // Uranus (1408): Dark Charcoal Epsilon & Inner Shepherd Ringlets
                        let r_inner = exp(-abs(r - 0.62) * 110.0) + exp(-abs(r - 0.71) * 120.0);
                        let r_eps = exp(-abs(r - 0.85) * 85.0) * (1.15 + 0.35 * sin(az * 2.0 + time * 0.3));
                        let ring_band = min(r_inner * 0.65 + r_eps * 1.15, 1.35);
                        let f_ring = select(1.0, 1.35, doc_feat(inst) == 4u || doc_feat(inst) == 255u);
                        let alpha = smoothstep(r_min, r_min + 0.02, r) * (1.0 - smoothstep(0.92, 0.97, r))
                            * ring_band * 0.38 * f_ring * doc_feat_focus(inst, 4u);
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
            } else if (vid == 20u || vid == 21u || vid == 22u || (vid >= 3660u && vid <= 3671u)) {
                let mw_slot = select(vid - 20u, doc_submode(inst) % 3u, vid >= 3660u);
                if (mw_slot == 0u) {
                    // Landmark: Heliopause Bow Shock & Magnetotail
                    let bow = exp(-abs(length(vec2<f32>(p.y, p.z)) - sqrt(max(0.0, (0.48 - p.x) * 0.55))) * 26.0) * smoothstep(0.50, -0.75, p.x);
                    let sun_spark = exp(-length(p - vec3<f32>(0.22, 0.0, 0.0)) * 18.0);
                    dens = bow * 1.1 + sun_spark * 1.2;
                    neb_col = mix(vec3<f32>(0.25, 0.95, 1.65), vec3<f32>(1.75, 1.15, 0.42), sun_spark);
                } else if (mw_slot == 1u) {
                    // Landmark: Orion / Eagle Stellar Nursery Creation Pillars
                    let p_tilt = doc_rot_z(p, 0.25);
                    let c1 = exp(-length(p_tilt.xz - vec2<f32>(-0.22, 0.0)) * 16.0) * smoothstep(0.55, -0.45, p_tilt.y);
                    let c2 = exp(-length(p_tilt.xz - vec2<f32>(0.06, 0.12)) * 19.0) * smoothstep(0.42, -0.45, p_tilt.y);
                    let c3 = exp(-length(p_tilt.xz - vec2<f32>(0.28, -0.08)) * 22.0) * smoothstep(0.28, -0.45, p_tilt.y);
                    let edge_fbm = 0.4 + 0.6 * abs(fbm3(p * 6.5, 3));
                    let stars = pow(max(0.0, snoise3(p * 16.0)), 8.0) * 1.6;
                    dens = ((c1 + c2 + c3) * edge_fbm * 0.9 + stars) * (1.0 - smoothstep(0.70, 0.94, r));
                    neb_col = mix(vec3<f32>(1.45, 0.42, 0.85), vec3<f32>(0.28, 1.25, 1.65), clamp(p_tilt.y + 0.5, 0.0, 1.0));
                } else {
                    // Landmark: Local Bubble Carved Supernova Shock Cavity Shell
                    let cav_w = snoise3(p * 3.8) * 0.14;
                    let shell = exp(-abs(r - 0.68 - cav_w) * 32.0);
                    let rim = exp(-abs(sin(atan2(p.z, p.x) * 6.0)) * 8.0);
                    dens = shell * (0.28 + 0.65 * rim);
                    neb_col = mix(vec3<f32>(0.18, 1.15, 1.45), vec3<f32>(0.85, 0.55, 1.65), rim);
                }
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
