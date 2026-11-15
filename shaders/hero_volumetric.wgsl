// HyperCosmosVR — Foveated Raymarched & Analytical Volumetric Hero Dispatcher
// Requires concatenated prelude:
//   common_oled.wgsl -> doc_common.wgsl -> hero_doc_cosmos.wgsl
//   -> hero_doc_matter_mind.wgsl -> hero_doc_civilization_horizon.wgsl
// Vertex attributes: @location(0) local_pos: vec3<f32> (unit cube [-1, 1]^3, CullMode::Front)
// Entry points:
//   - vs_main, vs_stereo
//   - fs_hero_surface (opaque planets, Earth cutaway, Kerr black core, PBR material spheres)
//   - fs_hero_volume  (translucent auroras, Cassini rings, nebulae, Kerr lensing & Apps 01, 02, 11..25)

fn build_hero_vs(local_pos: vec3<f32>, inst_idx: u32, eye_idx: u32) -> HeroVsOut {
    let inst = hero_instances[inst_idx];
    let r = max(inst.pos_radius.w, 1e-5);
    let q = normalize(inst.rot_quat);
    let world_pos = inst.pos_radius.xyz + quat_rotate(q, local_pos * r);
    let eye_pos = u_frame.eye_offset[eye_idx].xyz;
    let cam_local = quat_rotate(quat_conj(q), (eye_pos - inst.pos_radius.xyz) / r);

    var out: HeroVsOut;
    out.clip_pos = world_pos_to_clip(world_pos, eye_idx);
    out.local_pos = local_pos;
    out.cam_local = cam_local;
    out.inst_idx = inst_idx;
    out.eye_idx = eye_idx;
    return out;
}

@vertex
fn vs_main(
    @location(0) local_pos: vec3<f32>,
    @builtin(instance_index) inst_idx: u32,
) -> HeroVsOut {
    return build_hero_vs(local_pos, inst_idx, 0u);
}

@vertex
fn vs_stereo(
    @location(0) local_pos: vec3<f32>,
    @builtin(instance_index) inst_idx: u32,
    @builtin(view_index) view_idx: u32,
) -> HeroVsOut {
    return build_hero_vs(local_pos, inst_idx, view_idx);
}

@fragment
fn fs_hero_surface(in: HeroVsOut) -> SurfaceFsOut {
    var out: SurfaceFsOut;
    let inst = hero_instances[in.inst_idx];
    let kind = inst.kind_tags.x;
    let vid = inst.kind_tags.y;
    if (kind != 0u && kind != 1u && kind != 4u) {
        discard;
    }

    let ro = in.cam_local;
    let rd = normalize(in.local_pos - in.cam_local);
    let q = normalize(inst.rot_quat);
    let q_inv = quat_conj(q);
    let sun_local = normalize(quat_rotate(q_inv, normalize(inst.phys_b.xyz)));
    let time = u_frame.scale_params.w;

    // Sphere surface intersection (r = 0.84 inside [-1, 1]^3 so atmospheric auroras fit outside)
    let surf_r: f32 = select(0.84, 0.22, kind == 4u);
    let hit = intersect_sphere(ro, rd, surf_r);
    if (hit.y < 0.0) {
        discard;
    }

    let t_hit = max(hit.x, 0.0);
    let p_hit = ro + rd * t_hit;
    let n_surf = normalize(p_hit);
    var col = vec3<f32>(0.0);

    if (kind == 4u) {
        // Kerr Black Hole Event Horizon Core (pure #000000 black sphere occluding background)
        let world_p = inst.pos_radius.xyz + quat_rotate(q, p_hit * inst.pos_radius.w);
        let clip = world_pos_to_clip(world_p, in.eye_idx);
        out.color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
        out.depth = reversed_z_depth(clip.z, clip.w);
        return out;
    }

    // Deep Earth Cutaway Mode (03): bit 0 in kind_tags.w reveals 14-zone mantle & Fe-Ni core
    let is_cutaway = (kind == 0u) && ((inst.kind_tags.w & 1u) != 0u);
    if (is_cutaway && p_hit.x > 0.0 && p_hit.y > -0.1 && p_hit.z > 0.0) {
        let core_hit = intersect_sphere(ro, rd, 0.26);
        let p_core = ro + rd * max(core_hit.x, 0.0);
        if (core_hit.y > 0.0 && core_hit.x > 0.0) {
            col = shade_doc_earth_inner_core(p_core, normalize(p_core), -rd, inst, time);
            let world_p = inst.pos_radius.xyz + quat_rotate(q, p_core * inst.pos_radius.w);
            let clip = world_pos_to_clip(world_p, in.eye_idx);
            out.color = vec4<f32>(oled_void_clamp(col), 1.0);
            out.depth = reversed_z_depth(clip.z, clip.w);
            return out;
        }
        var best_t: f32 = 1e6;
        if (abs(rd.x) > 1e-5) {
            let tx = -ro.x / rd.x;
            let px = ro + rd * tx;
            if (tx > 0.0 && tx < best_t && length(px) <= surf_r && px.y >= -0.1 && px.z >= 0.0) {
                best_t = tx;
            }
        }
        if (abs(rd.y) > 1e-5) {
            let ty = (-0.1 - ro.y) / rd.y;
            let py = ro + rd * ty;
            if (ty > 0.0 && ty < best_t && length(py) <= surf_r && py.x >= 0.0 && py.z >= 0.0) {
                best_t = ty;
            }
        }
        if (abs(rd.z) > 1e-5) {
            let tz = -ro.z / rd.z;
            let pz = ro + rd * tz;
            if (tz > 0.0 && tz < best_t && length(pz) <= surf_r && pz.x >= 0.0 && pz.y >= -0.1) {
                best_t = tz;
            }
        }
        let p_wall = select(p_hit, ro + rd * best_t, best_t < 1e5);
        col = shade_doc_earth_cutaway(p_wall, surf_r, inst, time);
        let world_p = inst.pos_radius.xyz + quat_rotate(q, p_wall * inst.pos_radius.w);
        let clip = world_pos_to_clip(world_p, in.eye_idx);
        out.color = vec4<f32>(oled_void_clamp(col), 1.0);
        out.depth = reversed_z_depth(clip.z, clip.w);
        return out;
    } else if (kind == 0u) {
        let pixel_footprint = max(t_hit * 2.0 * tan(u_frame.cam_forward.w * 0.5)
            / u_frame.viewport_params.y / surf_r, 1e-6);
        col = shade_doc_earth_surface(p_hit / surf_r, n_surf, -rd, sun_local, inst, time, pixel_footprint);
    } else {
        col = shade_doc_solar_planet_surface(vid, p_hit / surf_r, n_surf, -rd, sun_local, inst, time);
    }

    // Focus dwell halo rim when stopped/narrating on this surface hero
    let dwell = max(inst.phys_a.w, doc_weight(inst) * 0.55);
    let natural_body = kind == 0u || (vid == 3643u && doc_submode(inst) == 1u);
    if (dwell > 0.01 && !natural_body) {
        let rim = pow(1.0 - max(dot(n_surf, -rd), 0.0), 4.0);
        col += u_frame.region_accent.rgb * rim * dwell * 2.0;
    }

    let world_p = inst.pos_radius.xyz + quat_rotate(q, p_hit * inst.pos_radius.w);
    let clip = world_pos_to_clip(world_p, in.eye_idx);
    out.color = vec4<f32>(oled_void_clamp(col), 1.0);
    out.depth = reversed_z_depth(clip.z, clip.w);
    return out;
}

@fragment
fn fs_hero_volume(in: HeroVsOut) -> SurfaceFsOut {
    let inst = hero_instances[in.inst_idx];
    let kind = inst.kind_tags.x;
    let vid = inst.kind_tags.y;
    let ro = in.cam_local;
    let rd = normalize(in.local_pos - in.cam_local);
    let box_t = intersect_box(ro, rd, vec3<f32>(-1.0), vec3<f32>(1.0));
    if (box_t.y <= max(box_t.x, 0.0)) {
        discard;
    }

    var t0 = max(box_t.x, 0.0);
    var t1 = box_t.y;
    // Shrink documentary volume march interval tightly to active r <= 0.96 sphere so steps are 2x denser
    if (kind >= 5u) {
        let sph_t = intersect_sphere(ro, rd, 0.96);
        if (sph_t.y <= max(sph_t.x, 0.0)) {
            discard;
        }
        t0 = max(t0, max(sph_t.x, 0.0));
        t1 = min(t1, sph_t.y);
        if (t1 <= t0) {
            discard;
        }
    }
    let raw_jitter = sample_blue_noise_jitter(in.clip_pos.xy);
    let jitter = select(raw_jitter, raw_jitter * 0.18, kind >= 5u);
    let q = normalize(inst.rot_quat);
    let world_rd = normalize(quat_rotate(q, rd));
    let fovea_k = fovea_step_factor(world_rd) * clamp(inst.phys_b.w, 0.35, 1.0);
    let time = u_frame.scale_params.w;
    let voice = u_audio.levels_a.x;
    let bass = u_audio.levels_a.z;
    let sun_local = normalize(quat_rotate(quat_conj(q), normalize(inst.phys_b.xyz)));

    var res = vec4<f32>(0.0);

    if (kind == 0u) {
        res = raymarch_doc_earth_shell(ro, rd, t0, t1, jitter, fovea_k, inst, time, bass);
    } else if (kind <= 4u) {
        res = raymarch_doc_cosmos_macro(kind, vid, ro, rd, t0, t1, jitter, fovea_k, inst, sun_local, time, voice, bass);
    } else if (kind <= 8u) {
        res = raymarch_doc_matter_mind(kind, vid, ro, rd, t0, t1, jitter, fovea_k, inst, time, voice, bass);
        res = vec4<f32>(res.rgb * min(inst.color_a.a, 1.35), res.a);
    } else {
        res = raymarch_doc_civilization_horizon(vid, ro, rd, t0, t1, jitter, fovea_k, inst, time, voice, bass);
        res = vec4<f32>(res.rgb * min(inst.color_a.a, 1.35), res.a);
    }

    // Optically thin Earth emission may have negligible alpha and still carry
    // visible light. An alpha-only threshold cuts auroral tails into hard shapes.
    let empty_earth = res.a < 1e-6 && max(res.r, max(res.g, res.b)) < 1e-6;
    if (select(res.a < 1e-4, empty_earth, kind == 0u)) {
        discard;
    }
    var out: SurfaceFsOut;
    out.color = vec4<f32>(oled_void_clamp(res.rgb), clamp(res.a, 0.0, 1.0));
    out.depth = in.clip_pos.z;
    if (kind == 0u) {
        // The back face of the volume cube is behind the opaque Earth. Test the
        // near atmospheric boundary instead so elevated clouds can cover the globe.
        // Integration itself stops at the planet surface; no back-side glow leaks through.
        let limb_r = select(inst.phys_a.y, 0.737, inst.phys_a.y <= 0.0);
        let shell_hit = intersect_sphere(ro, rd, limb_r * 1.095);
        var front_t = max(shell_hit.x, 0.0);
        if (doc_feat(inst) != 255u || shell_hit.y <= 0.0) { front_t = t0; }
        let local_p = ro + rd * front_t;
        let world_p = inst.pos_radius.xyz + quat_rotate(q, local_p * inst.pos_radius.w);
        let clip = world_pos_to_clip(world_p, in.eye_idx);
        out.depth = reversed_z_depth(clip.z, clip.w);
    }
    return out;
}
