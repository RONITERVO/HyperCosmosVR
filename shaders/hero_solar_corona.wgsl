// Diffuse solar atmosphere. Resolved field-aligned strands are actual 3D curves
// in solar_plasma.wgsl; this pass supplies the optically thin medium between them.
// Instrument-mapped coronal colors are illustrative, not naked-eye visible light.

// Three smooth independent channels share a static GPU lattice lookup. The
// diffuse atmosphere keeps its 3D domain and per-ray optical integration.
// Quintic interpolation retains C2 continuity;
// an orthonormal domain rotation and subsequent warp avoid axis-aligned bands.
fn solar_atmosphere_noise(p: vec3<f32>) -> vec3<f32> {
    let q = mat3x3<f32>(vec3<f32>(0.8,0.0,0.6),vec3<f32>(-0.36,0.8,0.48),
        vec3<f32>(-0.48,-0.6,0.64))*p;
    return solar_cached_noise(q);
}

fn solar_corona_components(p: vec3<f32>) -> vec3<f32> {
    let r = length(p);
    let direction = p / max(r, 1e-6);
    let height = max(r - 1.0, 0.0);
    let active_uv = vec2<f32>(dot(direction, DOC_SUN_REGION_EAST), dot(direction, DOC_SUN_REGION_NORTH));
    let near_active = smoothstep(0.70, 0.95, dot(direction, DOC_SUN_REGION_CENTER));
    let arcade = exp(-pow(active_uv.x / (0.10 + height * 0.22), 2.0)
        - pow(active_uv.y / (0.065 + height * 0.12), 2.0)) * near_active;
    // A sparse surrounding streamer envelope; the fixed latitude structure
    // co-rotates with the surface instead of scrolling through it.
    let latitude = direction.y;
    let equator = exp(-pow(latitude / (0.28 + height * 0.08), 2.0));
    let inv_r = 1.0/max(r,1.0);
    let inv2 = inv_r*inv_r;
    let inv4 = inv2*inv2;
    let quiet = inv4*inv4*inv2*0.015;
    let streamer = equator*inv4*inv2*sqrt(inv_r)*0.035;
    let moss = arcade * exp(-height / 0.027) * 0.22;
    // Broad hot material associated with the persistent arcade, without
    // substituting a noisy shell for the resolved loop geometry.
    let envelope = arcade * exp(-height / 0.17) * 0.055;
    let edge_fade = 1.0 - smoothstep(2.6, 3.0, r);
    return vec3<f32>(quiet,streamer,moss+envelope)*edge_fade;
}

// Instrument-mapped lower-corona response. This is a three-dimensional emitting
// layer above the opaque photosphere, not dark coronal holes painted into visible
// continuum light. Pattern scales describe magnetic network and moss, not granules.
// Exact box average of an exponential ridge under a locally linear field.
// Its width includes travel through the integration cell, not only pixel size:
// unresolved network crossings must not form stacked contours at grazing views.
fn solar_euv_ridge(value: f32, half_width: f32) -> f32 {
    let w = max(half_width,1e-4);
    let a = value-w;
    let b = value+w;
    let primitive_a = sign(a)*(1.0-exp(-abs(a)*10.0))*0.1;
    let primitive_b = sign(b)*(1.0-exp(-abs(b)*10.0))*0.1;
    return max(0.0,(primitive_b-primitive_a)/(2.0*w));
}

fn solar_euv_emission(p: vec3<f32>, rd: vec3<f32>, time: f32, footprint: f32, step: f32) -> vec3<f32> {
    let r = length(p);
    let h = r - 1.0;
    if (h < 0.001 || h > 0.075) { return vec3<f32>(0.0); }
    let n = p / r;
    // A 25Mm magnetic network organizes a genuinely volumetric 12Mm emitting
    // field. Using position (not only direction) gives height variation rather
    // than radially extruding one angular texture into smooth organ-pipe cells.
    let magnetic_q = n*(DOC_SUN_RADIUS_M/25000000.0);
    let q = p*(DOC_SUN_RADIUS_M/12000000.0);
    let distortion = solar_atmosphere_noise(magnetic_q*1.7)*0.65;
    let broad = solar_atmosphere_noise(q+distortion);
    let weave = solar_atmosphere_noise(q*3.1+distortion*1.7+vec3<f32>(0.0,time*0.004,0.0));
    let field = broad.x + weave.z*0.32;
    let field_width = max(footprint,step)*DOC_SUN_RADIUS_M/12000000.0*2.1*0.5;
    let network = solar_euv_ridge(field,field_width);
    let coarse = 0.5+0.5*weave.y;
    let fine_filter = 1.0-smoothstep(0.0005,0.002,max(footprint,step));
    var fine = 0.5;
    if (fine_filter > 0.0) {
        fine = mix(0.5,0.5+0.5*solar_atmosphere_noise(p*620.0+vec3<f32>(time*0.08,0.0,0.0)).z,fine_filter);
    }
    let root0 = length(n-doc_sun_spot_direction(0u));
    let root1 = length(n-doc_sun_spot_direction(1u));
    let roots = exp(-pow(root0/0.036,2.0))+exp(-pow(root1/0.031,2.0));
    let network_height = exp(-h/0.016) * smoothstep(0.001,0.004,h);
    let moss = roots * pow(fine,2.0) * exp(-h/0.007) * smoothstep(0.001,0.003,h);
    let hole = doc_sun_open_region(n);
    // Persistent regional magnetic heating gives quiet patches, bright plage
    // and corridors between them. The 25Mm network remains small within these
    // regions; no arbitrary enlargement of photospheric granules is involved.
    let c = DOC_SUN_REGION_CENTER;
    let e = DOC_SUN_REGION_EAST;
    let north = DOC_SUN_REGION_NORTH;
    let plage0 = exp(-pow(length(n-c)/0.17,2.0));
    let plage1 = exp(-pow(length(n-normalize(c+e*0.78-north*0.36))/0.23,2.0));
    let plage2 = exp(-pow(length(n-normalize(c-e*0.90+north*0.20))/0.20,2.0));
    let plage3 = exp(-pow(length(n-normalize(-c+e*0.65-north*0.24))/0.27,2.0));
    let magnetic_domains = 0.5+0.5*solar_atmosphere_noise(n*5.3+vec3<f32>(8.2,-3.1,1.4)).x;
    let heating = 0.12 + 0.65*pow(magnetic_domains,2.0)
        + plage0*1.7 + plage1*1.2 + plage2*0.9 + plage3*1.4;
    let density = (0.035 + network*(0.6+coarse*0.9) + fine*0.055) * network_height * heating;
    let temperature_color = mix(vec3<f32>(1.15,0.23,0.025), vec3<f32>(1.9,0.90,0.24), coarse);
    return (temperature_color*density*19.0 + vec3<f32>(2.1,1.3,0.46)*moss*37.0)
        * mix(1.0,0.045,hole) * (1.0-smoothstep(0.065,0.075,h));
}

// Gauss-Legendre nodes/weights on [0,1]. Radial atmosphere integration uses
// these on each side of closest approach; zero-length sides are skipped.
const SOLAR_GL8_NODES = array<f32,8>(0.019855072,0.101666761,0.237233795,0.408282679,
    0.591717321,0.762766205,0.898333239,0.980144928);
const SOLAR_GL8_WEIGHTS = array<f32,8>(0.050614268,0.111190517,0.156853323,0.181341892,
    0.181341892,0.156853323,0.111190517,0.050614268);

const SOLAR_GL16_NODES = array<f32,16>(0.0052995325,0.0277124885,0.0671843988,0.1222977958,0.1910618778,0.2709916112,0.3591982246,0.4524937451,0.5475062549,0.6408017754,0.7290083888,0.8089381222,0.8777022042,0.9328156012,0.9722875115,0.9947004675);
const SOLAR_GL16_WEIGHTS = array<f32,16>(0.0135762297,0.0311267620,0.0475792558,0.0623144856,0.0747979944,0.0845782597,0.0913017075,0.0947253052,0.0947253052,0.0913017075,0.0845782597,0.0747979944,0.0623144856,0.0475792558,0.0311267620,0.0135762297);

struct SolarCoronalColumns { value:vec3<f32>, direction:vec3<f32> };
fn solar_corona_columns(ro:vec3<f32>,rd:vec3<f32>,start:f32,stop:f32) -> SolarCoronalColumns {
    var coronal_columns = vec3<f32>(0.0);
    let inner = intersect_sphere(ro,rd,1.0);
    var streamer_direction = vec3<f32>(0.0);
    // Split at closest approach: quadratic spacing resolves the near-limb
    // atmosphere independently of a long empty foreground path.
    let closest = clamp(-dot(ro, rd), start, stop);
    for (var side = 0u; side < 2u; side += 1u) {
        let far_t = select(start, stop, side == 1u);
        var near_t = closest;
        if (inner.y > inner.x) {
            near_t = select(min(closest,inner.x),max(closest,inner.y),side==1u);
        }
        if ((side==0u && near_t<=far_t) || (side==1u && near_t>=far_t)) { continue; }
        let span = far_t - near_t;
        if (abs(span) < 1e-7) { continue; }
        for (var i = 0u; i < 16u; i += 1u) {
            let u = SOLAR_GL16_NODES[i];
            let t = near_t+span*u*u;
            let weight = abs(span)*2.0*u*SOLAR_GL16_WEIGHTS[i];
            let p = ro+rd*t;
            if (length(p) >= 1.0) {
                let columns = solar_corona_components(p)*weight;
                coronal_columns += columns;
                streamer_direction += normalize(p)*columns.y;
            }
        }
    }
    return SolarCoronalColumns(coronal_columns,streamer_direction);
}

fn raymarch_solar_corona(ro_bound: vec3<f32>, rd: vec3<f32>, inst: HeroVolumeInstance,
    time: f32) -> vec4<f32> {
    let ratio = max(inst.phys_a.y, 0.001);
    let ro = ro_bound / ratio;
    let outer = intersect_sphere(ro, rd, 3.0);
    let start = max(outer.x, 0.0);
    var stop = outer.y;
    if (doc_feat(inst) == 0u) {
        stop = min(stop, solar_cutaway_hit(ro, rd, 1.0).t);
    } else {
        let surface = intersect_sphere(ro, rd, 1.0);
        if (surface.y > 0.0 && surface.x > 0.0) { stop = min(stop, surface.x); }
        else if (surface.y > 0.0) { return vec4<f32>(0.0); }
    }
    if (stop <= start) { return vec4<f32>(0.0); }
    var light = vec3<f32>(0.0);
    let columns = solar_corona_columns(ro,rd,start,stop);
    let coronal_columns = columns.value;
    let streamer_direction = columns.direction;
    // The broad angular streamer field is evaluated at its emission-weighted
    // direction, after integrating its true radial optical column. Resolved
    // strands remain fully three dimensional in their separate analytic pass.
    var longitudinal = 0.66;
    if (coronal_columns.y > 1e-8 && dot(streamer_direction,streamer_direction)>1e-14) {
        let value = 0.5+0.5*solar_atmosphere_noise(normalize(streamer_direction)*7.0).x;
        longitudinal = 0.55+0.45*value*value;
    }
    light += vec3<f32>(0.85,0.91,1.0)*(coronal_columns.x+coronal_columns.y*longitudinal)
        +vec3<f32>(1.3,0.54,0.16)*coronal_columns.z;
    // The thin chromosphere has its own bounded interval so its physical
    // thickness does not disappear between samples of the outer atmosphere.
    let shell = intersect_sphere(ro, rd, 1.016);
    let shell_start = max(start, shell.x);
    let shell_stop = min(stop, shell.y);
    if (shell.y > 0.0 && shell_stop > shell_start) {
        let reference_n = normalize(ro+rd*((shell_start+shell_stop)*0.5));
        var column = 0.0;
        var direction_offset = vec3<f32>(0.0);
        var angular_second_moment = 0.0;
        var distance_moment = 0.0;
        let inner = intersect_sphere(ro,rd,1.0);
        let tangent_t = clamp(-dot(ro,rd),shell_start,shell_stop);
        for (var side=0u; side<2u; side+=1u) {
            // Integrate only the hollow shell. Cutaway rays may pass through the
            // interior gap; that gap must not consume thin-atmosphere samples.
            var a = shell_start;
            var b = tangent_t;
            if (side == 1u) { a = tangent_t; b = shell_stop; }
            if (inner.y > inner.x) {
                if (side == 0u) { b = min(b,inner.x); }
                else { a = max(a,inner.y); }
            }
            if (b <= a) { continue; }
            for (var i=0u; i<8u; i+=1u) {
                let t = mix(a,b,SOLAR_GL8_NODES[i]);
                let p = ro+rd*t;
                let r = length(p);
                let n = p/r;
                let weight = exp(-(r-1.0)/0.0016)*(b-a)*SOLAR_GL8_WEIGHTS[i];
                let offset = n-reference_n;
                column += weight;
                direction_offset += offset*weight;
                angular_second_moment += dot(offset,offset)*weight;
                distance_moment += t*weight;
            }
        }
        if (column > 1e-8) {
            let offset = direction_offset/column;
            let mean_direction = reference_n+offset;
            let direction = normalize(select(reference_n,mean_direction,dot(mean_direction,mean_direction)>1e-8));
            let variance = max(angular_second_moment/column-dot(offset,offset),0.0);
            let pixel = max(distance_moment/column,0.0)*2.0*tan(u_frame.cam_forward.w*0.5)/u_frame.viewport_params.y;
            let footprint = max(pixel,2.0*sqrt(variance));
            let detail = 1.0-smoothstep(0.0008,0.003,footprint);
            let mottles = 0.5+0.5*solar_atmosphere_noise(direction*430.0+vec3<f32>(time*0.015)).x;
            light += vec3<f32>(1.7,0.10,0.026)*column*mix(0.65,0.22+0.86*mottles,detail)*7.0;
        }
    }
    if (doc_sun_euv_weight(inst) > 0.0) {
        let euv_shell = intersect_sphere(ro,rd,1.075);
        let a = max(start,euv_shell.x);
        let b = min(stop,euv_shell.y);
        if (euv_shell.y > 0.0 && b > a) {
            let closest = clamp(-dot(ro,rd),a,b);
            let inner = intersect_sphere(ro,rd,1.001);
            for (var side=0u; side<2u; side+=1u) {
                let far_t = select(a,b,side==1u);
                var near_t = closest;
                if (inner.y > inner.x) {
                    near_t = select(min(closest,inner.x),max(closest,inner.y),side==1u);
                }
                if ((side==0u && near_t<=far_t) || (side==1u && near_t>=far_t)) { continue; }
                let span = far_t-near_t;
                for (var i=0u; i<16u; i+=1u) {
                    let u = SOLAR_GL16_NODES[i];
                    let t = near_t+span*u*u;
                    let weight = abs(span)*2.0*u*SOLAR_GL16_WEIGHTS[i];
                    let footprint = max(t,0.0)*2.0*tan(u_frame.cam_forward.w*0.5)/u_frame.viewport_params.y;
                    light += solar_euv_emission(ro+rd*t,rd,time,footprint,weight)*weight;
                }
            }
        }
    }
    return vec4<f32>(light, 0.0);
}
