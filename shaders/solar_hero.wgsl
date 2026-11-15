// Dedicated solar entry points keep the atmospheric and photospheric programs
// independent of the much larger multi-experience documentary dispatch.
// Includes common_oled, doc_common, hero_doc_sun, hero_solar_interior, hero_solar_corona.

fn solar_hero_vertex(local:vec3<f32>, instance:u32, eye:u32) -> HeroVsOut {
    let inst = hero_instances[instance];
    let q = normalize(inst.rot_quat);
    let radius = max(inst.pos_radius.w,1e-5);
    let world = inst.pos_radius.xyz+quat_rotate(q,local*radius);
    var out:HeroVsOut;
    out.clip_pos = world_pos_to_clip(world,eye);
    out.local_pos = local;
    out.cam_local = quat_rotate(quat_conj(q),(u_frame.eye_offset[eye].xyz-inst.pos_radius.xyz)/radius);
    out.inst_idx = instance;
    out.eye_idx = eye;
    return out;
}

@vertex fn vs_solar_hero(@location(0) local:vec3<f32>, @builtin(instance_index) instance:u32) -> HeroVsOut {
    return solar_hero_vertex(local,instance,0u);
}
@vertex fn vs_solar_hero_stereo(@location(0) local:vec3<f32>, @builtin(instance_index) instance:u32,
    @builtin(view_index) eye:u32) -> HeroVsOut {
    return solar_hero_vertex(local,instance,eye);
}

@fragment fn fs_solar_surface(in:HeroVsOut) -> SurfaceFsOut {
    let inst = hero_instances[in.inst_idx];
    let ro = in.cam_local;
    let rd = normalize(in.local_pos-ro);
    let radius = 0.84;
    let sphere = intersect_sphere(ro,rd,radius);
    if (sphere.y <= 0.0) { discard; }
    var t = select(sphere.x,sphere.y,sphere.x<=0.0);
    var region = 0u;
    var normal = vec3<f32>(0.0,0.0,1.0);
    if (doc_feat(inst) == 0u) {
        let cut = solar_cutaway_hit(ro,rd,radius);
        if (cut.t > 1e20) { discard; }
        t = cut.t;
        region = cut.region;
        normal = cut.normal;
    }
    let p = ro+rd*t;
    let footprint = max(t*2.0*tan(u_frame.cam_forward.w*0.5)/u_frame.viewport_params.y/radius,1e-7);
    var color:vec3<f32>;
    if (region == 0u) {
        color = shade_doc_sun_surface(p/radius,normalize(p),-rd,inst,u_frame.scale_params.w,footprint);
    } else {
        color = shade_solar_cutaway(p,normal,-rd,radius,region,u_frame.scale_params.w,footprint);
    }
    let world = inst.pos_radius.xyz+quat_rotate(normalize(inst.rot_quat),p*inst.pos_radius.w);
    let clip = world_pos_to_clip(world,in.eye_idx);
    var out:SurfaceFsOut;
    out.color = vec4<f32>(oled_void_clamp(color),1.0);
    out.depth = reversed_z_depth(clip.z,clip.w);
    return out;
}

@fragment fn fs_solar_corona(in:HeroVsOut) -> SurfaceFsOut {
    let inst = hero_instances[in.inst_idx];
    let ro = in.cam_local;
    let rd = normalize(in.local_pos-ro);
    let shell = intersect_sphere(ro,rd,inst.phys_a.y*3.0);
    if (shell.y <= max(shell.x,0.0)) { discard; }
    let light = raymarch_solar_corona(ro,rd,inst,u_frame.scale_params.w);
    if (max(light.r,max(light.g,light.b)) < 1e-6 && light.a < 1e-6) { discard; }
    let p = ro+rd*max(shell.x,0.0);
    let world = inst.pos_radius.xyz+quat_rotate(normalize(inst.rot_quat),p*inst.pos_radius.w);
    let clip = world_pos_to_clip(world,in.eye_idx);
    var out:SurfaceFsOut;
    out.color = vec4<f32>(oled_void_clamp(light.rgb),light.a);
    // An observer inside the shell starts on the near plane. Projecting the
    // eye itself has w=0 and does not produce a meaningful fragment depth.
    out.depth = select(clamp(reversed_z_depth(clip.z,clip.w),0.0,1.0),1.0,shell.x<=0.0);
    return out;
}
