// Resolved solar plasma: world-space field-aligned curves with analytic Gaussian
// optical columns. Quads bound finite cylinders; fragment integration, Sun
// occlusion and depth are evaluated on the eye ray, not on a billboard plane.
// This is an authored magnetic/thermal illustration, not an MHD simulation.
// Includes common_oled, doc_common, hero_doc_sun, hero_solar_interior.

// The closed-loop emission pipeline fixes this to0, removing unrelated material
// branches/register pressure. All other draws retain the instance's family.
override SOLAR_PLASMA_FAMILY: u32 = 0xffffffffu;

struct SolarPlasmaVertex {
    @builtin(position) clip: vec4<f32>,
    @location(0) local_position: vec3<f32>,
    @location(1) @interpolate(flat) a: vec3<f32>,
    @location(2) @interpolate(flat) b: vec3<f32>,
    @location(3) @interpolate(flat) eye: vec3<f32>,
    @location(4) @interpolate(flat) data: vec4<f32>, // sigma, u0, du, seed
    @location(5) @interpolate(flat) tags: vec4<u32>, // hero, family, strand, eye
};

fn solar_plasma_hash(i: f32) -> vec3<f32> {
    return hash33(vec3<f32>(i * 0.6180339 + 1.0, i * 1.173 + 19.7, i * 0.731 + 71.3));
}

fn solar_bezier(a: vec3<f32>, b: vec3<f32>, c: vec3<f32>, d: vec3<f32>, u: f32) -> vec3<f32> {
    let v = 1.0 - u;
    return v*v*v*a + 3.0*v*v*u*b + 3.0*v*u*u*c + u*u*u*d;
}

// x/y identify curve family/strand; z/w are segment/count. Fixed world geometry
// is evaluated independently for both eyes, at the same simulation time.
fn solar_plasma_address(segment: u32) -> vec4<u32> {
    if (segment < 49152u) { return vec4<u32>(0u, segment / 64u, segment % 64u, 64u); }
    if (segment < 57344u) { let s = segment - 49152u; return vec4<u32>(1u, s / 64u, s % 64u, 64u); }
    if (segment < 65536u) { let s = segment - 57344u; return vec4<u32>(2u, s / 64u, s % 64u, 64u); }
    if (segment < 98304u) { let s = segment - 65536u; return vec4<u32>(3u, s / 8u, s % 8u, 8u); }
    if (segment < 106496u) { let s = segment - 98304u; return vec4<u32>(4u,s/32u,s%32u,32u); }
    let s = segment - 106496u;
    return vec4<u32>(5u,s/32u,s%32u,32u);
}

// Coherent cloud groups inside the authored erupting flux rope. Adjacent
// strands share a slowly evolving mass concentration, with finer material
// variations within it. This avoids both a uniformly glowing tube and unrelated
// particles. The three-part morphology is inspired by observed multi-temperature
// CME structure; this is not a synthetic coronagraph or an MHD reconstruction.
// https://doi.org/10.1088/0004-637X/794/2/149
fn solar_eruption_cloud(strand: u32, u: f32, time: f32) -> vec3<f32> {
    let core = strand < 32u;
    let index = select(max(strand, 32u) - 32u, strand, core);
    let group_id = select(index / 8u + 17u, index / 4u, core);
    let group = solar_plasma_hash(f32(group_id) + 2631.0);
    let event_time = fract(time / 60.0) * 60.0;
    let flow = u * (7.0 + group.y * 3.0) - event_time * (0.045 + group.z * 0.035);
    let mass = doc_sun_noise1(flow + group.x * 79.0);
    let fine = doc_sun_noise1(u * 29.0 - event_time * 0.13 + f32(strand) * 1.371);
    let condensation = smoothstep(0.22, 0.79, mass);
    let density = select(0.035 + condensation * (0.20 + fine * 0.80),
        0.16 + condensation * (0.55 + fine * 0.29), core);
    return vec3<f32>(density, mass, fine);
}

// Low-cost C2-continuous material coordinates for the resolved CME body. These
// modulate density inside the analytic Gaussian envelope; they do not move its
// geometry or replace the opaque-star/ray intersection.
fn solar_eruption_noise(position: vec3<f32>) -> vec3<f32> {
    return solar_cached_noise(position);
}

fn solar_eruption_structure(sample_point: vec3<f32>, a: vec3<f32>, b: vec3<f32>,
    along: f32, u: f32, sigma: f32, pixel: f32, strand: u32, time: f32) -> vec2<f32> {
    let core = strand < 32u;
    let index = select(max(strand,32u) - 32u, strand, core);
    let group_id = select(index / 8u + 17u, index / 4u, core);
    let group = solar_plasma_hash(f32(group_id) + 2631.0);
    let event_time = fract(time / 60.0) * 60.0;
    let rise = smoothstep(12.0,34.0,event_time);
    let height_factor = select(0.94+group.z*0.06,0.82+group.z*0.10,core);
    let height = 0.027+1.55*rise*rise*height_factor;
    // A continuous reference derivative supplies the transported frame. Using
    // each proxy chord's tangent here would rotate the material discontinuously
    // at all64 segment joins even though their geometric endpoints coincide.
    let endpoint_derivative = 6.0*u*(1.0-u);
    let lateral_derivative = 3.0*(-1.0+6.0*u-6.0*u*u);
    let axis = normalize(DOC_SUN_REGION_CENTER*(3.0*(1.0-2.0*u)*height*1.34)
        + DOC_SUN_REGION_EAST*(0.097*endpoint_derivative+height*0.95*lateral_derivative)
        + DOC_SUN_REGION_NORTH*(-0.007*endpoint_derivative-0.014*lateral_derivative));
    let transverse = normalize(cross(DOC_SUN_REGION_NORTH, axis));
    let binormal = normalize(cross(axis, transverse));
    let displacement = sample_point - mix(a, b, along);
    let cross_section = vec2<f32>(dot(displacement, transverse), dot(displacement, binormal))
        / max(sigma, 1e-6);
    let footprint = pixel / max(sigma, 1e-6);
    let coarse_weight = 1.0 - smoothstep(0.30, 1.10, footprint);
    if (coarse_weight <= 0.0) { return vec2<f32>(1.0, 0.5); }
    let phase = u * 9.0 - event_time * 0.09;
    let coordinate = vec3<f32>(cross_section * 1.55, phase) + group * 19.0;
    let coarse = solar_eruption_noise(coordinate);
    let cloud_density = mix(1.0, max(0.06, 1.0 + coarse.x * 2.35), coarse_weight);
    let fine_weight = 1.0 - smoothstep(0.04, 0.22, footprint);
    var fiber_density = 1.0;
    if (fine_weight > 0.0) {
        // Faster transverse than longitudinal variation produces transported
        // fine fibers rather than isotropic speckle or a regular spoke comb.
        let fine_coordinate = vec3<f32>(cross_section * 6.0 + coarse.yz * 0.72,
            u * 3.0 - event_time * 0.035) + group * 31.0;
        let fine = solar_eruption_noise(fine_coordinate);
        fiber_density = mix(1.0, max(0.10, 1.0 + fine.x * 1.45 - fine.y * 0.45), fine_weight);
    }
    return vec2<f32>(min(cloud_density * fiber_density, 4.0),
        clamp(0.5 + coarse.y * 0.5 * coarse_weight, 0.0, 1.0));
}

fn solar_plasma_curve(family: u32, strand: u32, u: f32, time: f32) -> vec4<f32> {
    let h = solar_plasma_hash(f32(strand) + f32(family) * 917.0);
    let c = DOC_SUN_REGION_CENTER;
    let e = DOC_SUN_REGION_EAST;
    let n = DOC_SUN_REGION_NORTH;
    let clock = doc_sun_clock(time);
    let event_time = fract(time / 60.0) * 60.0;
    if (family == 0u) {
        // Twenty-four magnetic bundles, each resolved into thirty-two distinct
        // strands. Root offsets are stable inside the same two penumbrae.
        let group = solar_plasma_hash(f32(strand / 32u) + 411.0);
        let angle = group.x * TAU;
        let spread = 0.007 + 0.013 * sqrt(group.y);
        let root_y = cos(angle) * spread;
        let root_x = sin(angle) * spread;
        let micro = (h.xy - vec2<f32>(0.5)) * 0.006;
        let left = normalize(c + e * (-DOC_SUN_BIPOLE_HALF_SEPARATION + root_x + micro.x)
            + n * (root_y + micro.y));
        let right = normalize(c + e * (DOC_SUN_BIPOLE_HALF_SEPARATION - root_x * 0.72 + micro.x)
            + n * (-root_y * 0.55 + micro.y));
        let height = 0.032 + 0.143 * pow(group.z, 0.65) + (h.z - 0.5) * 0.004;
        let shear = n * ((group.x - 0.5) * 0.085);
        var p = solar_bezier(left * 1.0005, left * 1.0005 + c * height * 1.34 + shear,
            right * 1.0005 + c * height * 1.34 - shear * 0.35, right * 1.0005, u);
        let anchored = sin(PI * u) * sin(PI * u);
        p += n * sin(u * 21.0 - clock * 0.002 + h.x * TAU) * anchored * 0.00032;
        p += e * sin(u * 13.0 + h.y * TAU) * anchored * 0.00022;
        let expansion = 0.62 + 0.55 * pow(max(sin(PI*u), 0.0), 0.65);
        return vec4<f32>(p, (0.00009 + 0.00018 * h.y) * expansion);
    }
    if (family == 1u) {
        // 32 denser prominence strands trail within 96 broader, patchy sheath
        // strands. A depleted space separates cool material from the leading
        // envelope. The outermost 24 strands loosen into fanning tendrils.
        let rise = smoothstep(12.0, 34.0, event_time);
        let core = strand < 32u;
        let fraying = strand >= 104u;
        let index = select(max(strand, 32u) - 32u, strand, core);
        let group_count = select(12.0, 8.0, core);
        let group_id = select(index / 8u, index / 4u, core);
        let group = solar_plasma_hash(f32(group_id) + select(2648.0, 2631.0, core));
        // The cooler interior trails below the leading sheath. Small coherent
        // bundle differences give the envelope depth instead of stacking every
        // strand on one identical U-shaped outline. All heights remain bounded
        // by the original1.55R rise and every foot remains stationary.
        let height_factor = select(0.94 + group.z * 0.06, 0.82 + group.z * 0.10, core);
        let height = 0.027 + 1.55 * rise * rise * height_factor;
        // Stable distributed feet prevent all 128 strands collapsing into two
        // brilliant mathematical points as the apex expands.
        let source_width = select(0.014, 0.007, core);
        let foot_x = (h.x - 0.5) * source_width;
        let foot_y = (h.y - 0.5) * source_width;
        let left = normalize(c + e * (-0.050 + foot_x) + n * (0.004 + foot_y));
        let right = normalize(c + e * (0.047 + foot_x * 0.7) + n * (-0.003 + foot_y));
        let control_left = left + c*height*1.34 - e*height*0.95 + n*0.014;
        let control_right = right + c*height*1.34 + e*height*0.95 - n*0.014;
        let axis = solar_bezier(left, control_left, control_right, right, u);
        let delta = 0.001;
        let next = solar_bezier(left, control_left, control_right, right, min(u+delta,1.0));
        let prev = solar_bezier(left, control_left, control_right, right, max(u-delta,0.0));
        let tangent = normalize(next - prev);
        let transverse = normalize(cross(n, tangent));
        let binormal = normalize(cross(tangent, transverse));
        let anchored = pow(max(sin(PI * u), 0.0), 0.85);
        let apex = exp(-pow((u - 0.5) / 0.30, 2.0));
        let cloud = solar_eruption_cloud(strand, u, time);
        let phase = (f32(group_id) + 0.5) / group_count * TAU
            + (h.x - 0.5) * 0.40 + u * TAU * (1.8 - rise * 0.8)
            + rise * 0.35 * sin(u * 9.0 + group.z * TAU);
        let core_radius = (0.003 + 0.038 * rise) * sqrt(h.y);
        let sheath_radius = (0.006 + 0.128 * rise) * (0.64 + 0.36 * h.y);
        let rope_radius = select(sheath_radius, core_radius, core)
            * anchored * (0.70 + 0.30 * apex) * (0.86 + cloud.y * 0.14);
        let cross_section = transverse * cos(phase) + binormal * sin(phase);
        let region_offset = select(0.042, -0.082, core) * rise * anchored * apex;
        var p = axis + cross_section * rope_radius + c * region_offset;
        if (fraying) {
            // Persistent peripheral strands stretch and separate, maintaining
            // their footpoint connections while their outer sections disperse.
            let loosening = rise * rise * anchored;
            p += cross_section * (0.018 + h.z * 0.027) * loosening
                + binormal * sin(u * 13.0 - rise * 1.7 + group.x * TAU)
                    * (0.007 + h.x * 0.011) * loosening;
        }
        let core_sigma = 0.00035 + rise * (0.0011 + h.z * 0.0022);
        let sheath_sigma = 0.00065 + rise * (0.0045 + h.z * 0.0100);
        let sigma = select(sheath_sigma, core_sigma, core)
            * (0.20 + 0.80 * anchored) * (0.68 + cloud.y * 0.32)
            * select(1.0, 0.55, fraying);
        return vec4<f32>(p, sigma);
    }
    if (family == 2u) {
        // A neighboring coronal-hole funnel is open throughout the cycle; it
        // does not imply that the cooling closed loops all become solar wind.
        let az = h.x * TAU;
        let source_x = cos(az)*0.09*sqrt(h.y);
        let source_y = sin(az)*0.045*sqrt(h.y);
        let root = normalize(c + n * (0.315+source_y) + e * source_x);
        let radius = 1.006 + 1.70 * u;
        // Super-radial expansion fans the individual strands into a low-density
        // open funnel rather than a cylindrical blue jet attached to the star.
        let spread = 1.0 + 1.8 * u;
        let bend = e * (source_x*(spread-1.0) + 0.055*u*u)
            + n * (source_y*(spread-1.0) + 0.045*u*u);
        let braiding = (e * sin(az + 8.0*u) + n * cos(az + 8.0*u)) * (0.0015 + 0.003*u);
        return vec4<f32>(normalize(root + bend + braiding) * radius,
            (0.00045 + h.y * 0.0008) * (1.0 + u * 2.5));
    }
    if (family == 3u) {
        // Dense chromospheric fibrils around the active network, the quiet
        // inspection patch, and the full star. Independent birth/decay avoids
        // a synchronized furry shell or a repeating angular comb.
        var root: vec3<f32>;
        if (strand < 2048u) {
            let network_center = select(c, normalize(c - e * 0.38 - n * 0.15), strand >= 1024u);
            let r = sqrt(h.y) * select(0.16, 0.11, strand >= 1024u);
            root = normalize(network_center + e * cos(h.x * TAU) * r + n * sin(h.x * TAU) * r);
        } else {
            let y = 2.0 * h.x - 1.0;
            let az = h.y * TAU;
            root = vec3<f32>(sqrt(max(0.0, 1.0-y*y)) * cos(az), y,
                sqrt(max(0.0, 1.0-y*y)) * sin(az));
        }
        let age = fract(clock / (150.0 + h.z * 170.0) + h.x);
        let envelope = sin(PI * age);
        let height = (0.003 + h.z * 0.011) * pow(envelope, 0.75);
        let tangent = normalize(cross(root, vec3<f32>(0.132, 0.941, 0.311)));
        let p = root * (1.00015 + height * u) + tangent * (u*u * 0.002 * sin(h.y * TAU));
        return vec4<f32>(p, (0.000045 + h.y * 0.000065) * (1.0 - u * 0.6));
    }
    if (family == 5u) {
        // Schematic energy packets random-walk through the radiative layer.
        // Macroscopic steps/time are deliberately compressed for explanation;
        // these are not visible individual photons moving at the solar clock.
        let az = 0.07 + h.x*1.39;
        let latitude = 0.06 + h.y*1.39;
        let outward = normalize(e*sin(az)*cos(latitude)+n*sin(latitude)+c*cos(az)*cos(latitude));
        let node = floor(u*32.0+0.001);
        let scatter = solar_plasma_hash(f32(strand)*43.0+node*7.0+3009.0)-vec3<f32>(0.5);
        let p = outward*(0.252+0.444*u) + scatter*0.12*sin(PI*u);
        return vec4<f32>(p,0.0008+0.0012*h.z);
    }
    // Convective circulation on the revealed side of the interior. Separate
    // from coronal strands: material rises, spreads and returns inside .70–1R.
    let az = 0.08 + h.x * 1.34;
    let latitude = 0.07 + h.y * 1.30;
    let outward = normalize(e * sin(az) * cos(latitude) + n * sin(latitude)
        + c * cos(az) * cos(latitude));
    let lateral = normalize(cross(outward, c + n * 0.173));
    let phase = u * TAU;
    let p = outward * (0.842 + 0.135 * cos(phase))
        + lateral * (0.011 + h.z * 0.029) * sin(phase);
    return vec4<f32>(p, 0.0008 + h.z * 0.0012);
}

fn solar_absorber_segment_visible(address: vec4<u32>, time: f32) -> bool {
    let event_time = fract(time/60.0)*60.0;
    if (address.x == 0u) {
        let delay = solar_plasma_hash(f32(address.y/32u)+7.0).y*7.0;
        if (event_time <= 29.0+delay || event_time >= 57.0) { return false; }
        let seed = solar_plasma_hash(f32(address.y)+19.0).x;
        let rain_time = fract((event_time-30.0-delay)*0.085+seed);
        let rain_u = 0.5*pow(rain_time,1.65);
        let middle = (f32(address.z)+0.5)/f32(address.w);
        let distance = abs(abs(middle-0.5)-rain_u)-0.5/f32(address.w);
        return distance <= (0.003+seed*0.005)*4.0;
    }
    if (address.x == 1u) { return address.y < 32u && event_time > 0.0 && event_time < 48.0; }
    return address.x == 3u;
}

struct SolarPlasmaProxy {
    center: vec3<f32>,
    along: vec3<f32>,
    side: vec3<f32>,
    limits: vec4<f32>, // min/max along, min/max side in the center tangent plane
    full_screen: bool,
};

// A finite cylinder is the convex hull of its end caps. Perspective projection
// preserves that hull while both caps are in front of the eye. Find each disk's
// actual projected extrema, rather than independently combining its widest XY
// and nearest Z (which cannot generally be the same point on a tilted disk).
// For a screen coordinate t before multiplication by the plane distance,
// (depth*t-x)^2 = radius^2*|(screen+t*view) perpendicular to axis|^2.
// Its two roots are the cap's tangent rays. This only tightens the proxy; the
// strand radius, finite ray integral, material and true depth remain unchanged.
fn solar_plasma_proxy(ro: vec3<f32>, a: vec3<f32>, b: vec3<f32>, bound: f32) -> SolarPlasmaProxy {
    var proxy: SolarPlasmaProxy;
    proxy.center = (a+b)*0.5;
    proxy.limits = vec4<f32>(0.0);
    proxy.full_screen = false;
    let distance = length(ro-proxy.center);
    let view = (ro-proxy.center)/max(distance,1e-8);
    let ab = b-a;
    let axis = ab/max(length(ab),1e-8);
    let projected_axis = axis-view*dot(axis,view);
    let fallback = normalize(cross(view,select(vec3<f32>(0.0,1.0,0.0),
        vec3<f32>(1.0,0.0,0.0),abs(view.y)>0.9))+vec3<f32>(1e-12));
    proxy.along = select(fallback,normalize(projected_axis+vec3<f32>(1e-12)),
        dot(projected_axis,projected_axis)>1e-8);
    proxy.side = normalize(cross(proxy.along,view)+vec3<f32>(1e-12));
    let axis_depth = dot(axis,view);
    let radius_squared = bound*bound;
    let extent_depth = bound*sqrt(max(0.0,1.0-axis_depth*axis_depth));
    var low = vec2<f32>(1e30);
    var high = vec2<f32>(-1e30);
    for (var cap=0u; cap<2u; cap+=1u) {
        let p = select(a,b,cap==1u);
        let depth = dot(ro-p,view);
        if (distance<1e-7 || depth-extent_depth<=1e-7) {
            // The cap crosses the eye plane: no finite tangent-plane rectangle
            // can enclose its projection. A full viewport remains conservative.
            proxy.full_screen = true;
            return proxy;
        }
        let xy = vec2<f32>(dot(p-proxy.center,proxy.along),dot(p-proxy.center,proxy.side));
        let axis_xy = vec2<f32>(dot(axis,proxy.along),dot(axis,proxy.side));
        // Factor the denominator to retain precision for a cap close to the
        // eye plane. Express the discriminant with cross products to avoid
        // subtracting two nearly equal squared projected positions.
        let denominator = (depth-extent_depth)*(depth+extent_depth);
        let tangent_x = cross(axis,depth*proxy.along+xy.x*view);
        let tangent_y = cross(axis,depth*proxy.side+xy.y*view);
        let discriminant = max(vec2<f32>(dot(tangent_x,tangent_x),dot(tangent_y,tangent_y))
            - radius_squared*vec2<f32>(axis_xy.y*axis_xy.y,axis_xy.x*axis_xy.x),vec2<f32>(0.0));
        let center = depth*xy-radius_squared*axis_xy*axis_depth;
        let extent = bound*sqrt(discriminant);
        let scale = distance/denominator;
        low = min(low,(center-extent)*scale);
        high = max(high,(center+extent)*scale);
    }
    // Cover floating-point support error without introducing a visible tail or
    // a meaningful amount of the old longitudinal overdraw.
    let coordinate_scale = max(max(abs(low.x),abs(low.y)),max(abs(high.x),abs(high.y)));
    let epsilon = max(bound*1e-4,1e-7*(1.0+coordinate_scale));
    proxy.limits = vec4<f32>(low.x-epsilon,high.x+epsilon,low.y-epsilon,high.y+epsilon);
    return proxy;
}

fn solar_plasma_vertex(vertex: u32, instance: u32, eye_index: u32, absorption: bool) -> SolarPlasmaVertex {
    let hero_index = instance >> 20u;
    let address = solar_plasma_address(instance & 1048575u);
    let inst = hero_instances[hero_index];
    if ((address.x >= 4u && doc_feat(inst) != 0u)
        || (absorption && !solar_absorber_segment_visible(address,u_frame.scale_params.w))) {
        var hidden: SolarPlasmaVertex;
        hidden.clip = vec4<f32>(2.0,2.0,2.0,1.0);
        hidden.local_position = vec3<f32>(0.0);
        hidden.a = vec3<f32>(0.0);
        hidden.b = vec3<f32>(0.0);
        hidden.eye = vec3<f32>(0.0);
        hidden.data = vec4<f32>(0.0);
        hidden.tags = vec4<u32>(0u);
        return hidden;
    }
    let solar_radius = inst.pos_radius.w * max(inst.phys_a.y, 0.001);
    let q = normalize(inst.rot_quat);
    let ro = quat_rotate(quat_conj(q), u_frame.eye_offset[eye_index].xyz - inst.pos_radius.xyz) / solar_radius;
    let u0 = f32(address.z) / f32(address.w);
    let du = 1.0 / f32(address.w);
    let a = solar_plasma_curve(address.x, address.y, u0, u_frame.scale_params.w);
    let b = solar_plasma_curve(address.x, address.y, u0 + du, u_frame.scale_params.w);
    let distance = length(ro - (a.xyz + b.xyz) * 0.5);
    let pixel = distance * 2.0 * tan(u_frame.cam_forward.w * 0.5) / u_frame.viewport_params.y;
    let sigma = max(a.w, b.w);
    let bound = 3.5 * sqrt(sigma*sigma + pixel*pixel / 12.0);
    let uv = array<vec2<f32>, 6>(vec2<f32>(0.0,-1.0), vec2<f32>(1.0,-1.0),
        vec2<f32>(0.0,1.0), vec2<f32>(0.0,1.0), vec2<f32>(1.0,-1.0), vec2<f32>(1.0,1.0))[vertex];
    let proxy = solar_plasma_proxy(ro,a.xyz,b.xyz,bound);
    let p = proxy.center + proxy.along*mix(proxy.limits.x,proxy.limits.y,uv.x)
        + proxy.side*mix(proxy.limits.z,proxy.limits.w,uv.y*0.5+0.5);
    let world = inst.pos_radius.xyz + quat_rotate(q, p * solar_radius);
    var out: SolarPlasmaVertex;
    out.clip = world_pos_to_clip(world, eye_index);
    out.local_position = p;
    if (proxy.full_screen) {
        out.clip = vec4<f32>(uv.x*2.0-1.0,uv.y,0.5,1.0);
        let world_h = u_frame.inv_view_proj[eye_index]*out.clip;
        let ray = world_h.xyz/world_h.w-u_frame.eye_offset[eye_index].xyz;
        // Keep the unprojected ray at a numerically useful local distance;
        // adding a meter-scale near point to a solar-radius origin loses it.
        out.local_position = ro+quat_rotate(quat_conj(q),ray/max(u_frame.cam_up.w,1e-7));
    }
    out.a = a.xyz;
    out.b = b.xyz;
    out.eye = ro;
    out.data = vec4<f32>(sigma, u0, du, solar_plasma_hash(f32(address.y) + 19.0).x);
    out.tags = vec4<u32>(hero_index, address.x, address.y, eye_index);
    return out;
}

@vertex fn vs_solar_plasma(@builtin(vertex_index) v: u32, @builtin(instance_index) i: u32) -> SolarPlasmaVertex {
    return solar_plasma_vertex(v, i, 0u, false);
}
@vertex fn vs_solar_plasma_stereo(@builtin(vertex_index) v: u32, @builtin(instance_index) i: u32,
    @builtin(view_index) eye: u32) -> SolarPlasmaVertex {
    return solar_plasma_vertex(v, i, eye, false);
}
@vertex fn vs_solar_absorption(@builtin(vertex_index) v:u32, @builtin(instance_index) i:u32) -> SolarPlasmaVertex {
    return solar_plasma_vertex(v,i,0u,true);
}
@vertex fn vs_solar_absorption_stereo(@builtin(vertex_index) v:u32, @builtin(instance_index) i:u32,
    @builtin(view_index) eye:u32) -> SolarPlasmaVertex {
    return solar_plasma_vertex(v,i,eye,true);
}

fn solar_plasma_erf(value: f32) -> f32 {
    let a = abs(value);
    let t = 1.0 / (1.0 + 0.3275911 * a);
    let p = (((((1.061405429*t - 1.453152027)*t) + 1.421413741)*t - 0.284496736)*t + 0.254829592)*t;
    return sign(value) * (1.0 - p * exp(-a*a));
}

struct SolarPlasmaIntegral {
    column: f32,
    depth: f32,
    along: f32,
};

// Normalized Gaussian optical column through one finite strand segment.
// Resolve the finite interval before choosing a footprint or closest point:
// the closest point on the infinite axis may lie behind the eye or many solar
// radii beyond the actual segment when the view is nearly parallel.
fn solar_plasma_integral(ro: vec3<f32>, rd: vec3<f32>, a: vec3<f32>, b: vec3<f32>,
    sigma0: f32, pixel_scale: f32, depth_limit: f32) -> SolarPlasmaIntegral {
    let empty = SolarPlasmaIntegral(0.0, 0.0, 0.0);
    let ab = b - a;
    let len = length(ab);
    if (len < 1e-8 || depth_limit <= 0.0) { return empty; }
    let axis = ab / len;
    let w = ro - a;
    let axial_ray = dot(rd, axis);
    let axial_origin = dot(w, axis);
    var lo = 0.0;
    var hi = depth_limit;
    if (abs(axial_ray) > 1e-7) {
        let ta = -axial_origin / axial_ray;
        let tb = (len - axial_origin) / axial_ray;
        lo = max(lo, min(ta, tb));
        hi = min(hi, max(ta, tb));
    } else if (axial_origin < 0.0 || axial_origin > len) { return empty; }
    if (hi <= lo) { return empty; }

    let d_perp = rd - axis * axial_ray;
    let w_perp = w - axis * axial_origin;
    let den = dot(d_perp, d_perp);
    let midpoint = (lo + hi) * 0.5;
    let footprint_depth = clamp(dot((a + b) * 0.5 - ro, rd), lo, hi);
    let pixel = footprint_depth * pixel_scale;
    let sigma = sqrt(sigma0*sigma0 + pixel*pixel / 12.0);
    let span = hi - lo;
    var depth = midpoint;
    var column = 0.0;
    if (den < 1e-6 || den * span * span < 0.25 * sigma * sigma) {
        // Four-point Gauss-Legendre is stable at exactly parallel and avoids
        // subtracting almost equal saturated erf values. No fictitious epsilon
        // angle is introduced; the density is evaluated on the actual ray.
        let nodes = array<f32, 4>(0.0694318442, 0.3300094782, 0.6699905218, 0.9305681558);
        let weights = array<f32, 4>(0.1739274226, 0.3260725774, 0.3260725774, 0.1739274226);
        for (var i = 0u; i < 4u; i += 1u) {
            let offset = w_perp + d_perp * (lo + span * nodes[i]);
            column += weights[i] * exp(-dot(offset, offset) / (2.0 * sigma * sigma));
        }
        column *= span / (2.506628275 * sigma) * sigma0 / sigma;
    } else {
        let tc = -dot(w_perp, d_perp) / den;
        let closest = w_perp + d_perp * tc;
        let k = sqrt(den) / (1.41421356 * sigma);
        let window = max(0.0, solar_plasma_erf((hi-tc)*k) - solar_plasma_erf((lo-tc)*k));
        column = exp(-dot(closest, closest) / (2.0*sigma*sigma))
            * window * 0.5 / sqrt(den) * sigma0 / sigma;
        depth = clamp(tc, lo, hi);
    }
    let along = clamp((axial_origin + depth * axial_ray) / len, 0.0, 1.0);
    return SolarPlasmaIntegral(column, depth, along);
}

fn solar_plasma_fragment(in: SolarPlasmaVertex, absorption: bool) -> SurfaceFsOut {
    let inst = hero_instances[in.tags.x];
    let family = select(in.tags.y,SOLAR_PLASMA_FAMILY,SOLAR_PLASMA_FAMILY != 0xffffffffu);
    if (absorption && (family == 2u || family >= 4u)) { discard; }
    let interior = doc_feat(inst) == 0u;
    if (family >= 4u && !interior) { discard; }
    let ro = in.eye;
    let rd = normalize(in.local_position - ro);
    let outer = intersect_sphere(ro, rd, 3.0);
    if (outer.y <= 0.0) { discard; }
    var hi = outer.y;
    if (interior) {
        hi = min(hi, solar_cutaway_hit(ro, rd, 1.0).t);
    } else {
        let ground = intersect_sphere(ro, rd, 1.0);
        if (ground.y > 0.0 && ground.x > 0.0) { hi = min(hi, ground.x); }
        else if (ground.y > 0.0) { discard; }
    }
    let pixel_scale = 2.0 * tan(u_frame.cam_forward.w * 0.5) / u_frame.viewport_params.y;
    let integral = solar_plasma_integral(ro, rd, in.a, in.b, in.data.x, pixel_scale, hi);
    let column = integral.column;
    if (column < 1e-5) { discard; }
    let depth_t = integral.depth;
    let along = integral.along;
    let u = in.data.y + along * in.data.z;
    let seed = in.data.w;
    let time = u_frame.scale_params.w;
    let event_time = fract(time / 60.0) * 60.0;
    var color = vec3<f32>(1.7, 0.72, 0.21);
    var emission = 0.045;
    var opacity = 0.0;
    if (family == 0u) {
        let delay = solar_plasma_hash(f32(in.tags.z / 32u) + 7.0).y * 7.0;
        let heat = smoothstep(2.0+delay, 9.0+delay, event_time)
            * (1.0-smoothstep(25.0+delay, 40.0+delay, event_time));
        let front = clamp((event_time-3.0-delay) / 10.0, 0.0, 0.5);
        let fill = 1.0 - smoothstep(front-0.025, front+0.025, min(u, 1.0-u));
        let flow = pow(0.5 + 0.5*sin(u*37.0 - time*(1.3+seed*0.6) + seed*TAU), 3.0);
        let texture = 0.66 + 0.23*sin(u*163.0 + seed*19.0 + time*0.2)
            + 0.11*sin(u*367.0 - time*0.7 + seed*39.0);
        let strand_strength = 0.10 + 1.8*pow(seed,3.5);
        emission = (0.0012 + heat*fill*(0.008 + flow*0.024)) * texture * strand_strength;
        color = mix(vec3<f32>(1.5,0.34,0.065), vec3<f32>(2.2,1.15,0.37), heat);
        let cooling = smoothstep(29.0+delay, 36.0+delay, event_time)
            * (1.0-smoothstep(49.0, 57.0, event_time));
        let rain_time = fract((event_time-30.0-delay)*0.085 + seed);
        let rain_u = 0.5*pow(rain_time, 1.65);
        let drop_distance = abs(abs(u-0.5) - rain_u);
        // Each condensed packet forms and drains continuously. The periodic
        // travel coordinate must not teleport full opacity from foot to apex.
        let packet_life = smoothstep(0.0,0.06,rain_time)*(1.0-smoothstep(0.94,1.0,rain_time));
        let drop = exp(-pow(drop_distance / (0.003+seed*0.005), 2.0)) * cooling * packet_life;
        opacity = drop * 0.38;
        emission *= 1.0 - drop*0.93;
        emission += drop * 0.025;
        color = mix(color, vec3<f32>(0.85,0.035,0.01), drop);
    } else if (family == 1u) {
        let visible = smoothstep(0.0,5.0,event_time)*(1.0-smoothstep(36.0,48.0,event_time));
        let heating = smoothstep(16.0,24.0,event_time);
        let core = in.tags.z < 32u;
        let fraying = in.tags.z >= 104u;
        let cloud = solar_eruption_cloud(in.tags.z, u, time);
        let structure = solar_eruption_structure(ro + rd * depth_t, in.a, in.b, along,
            u, in.data.x, depth_t * pixel_scale, in.tags.z, time);
        let apex = exp(-pow((u-0.5)/0.31,2.0));
        // Cooler condensed material stays distinct from the hotter surrounding
        // envelope; color describes this illustrative multi-temperature view.
        let core_color = mix(vec3<f32>(0.86,0.035,0.012), vec3<f32>(1.25,0.24,0.045), heating);
        let hot_color = mix(vec3<f32>(1.1,0.11,0.025), vec3<f32>(1.75,0.72,0.19),
            heating * (0.40 + cloud.z * 0.25 + structure.y * 0.35));
        color = select(hot_color, core_color, core);
        let intensity = select(0.004 + heating * 0.017, 0.010 + heating * 0.020, core);
        // Most of the sheath's emission resides in clumps above the quiet feet.
        // Low-density gaps and frayed peripheral strands remain optically thin.
        let mass_structure = cloud.x * (0.35 + apex * 0.65);
        emission = visible * intensity * mass_structure * structure.x * select(1.0,0.62,fraying);
        opacity = select(0.0, visible * cloud.x * structure.x * (0.035 - heating * 0.020), core);
    } else if (family == 2u) {
        // At the shared30x clock this travels about735km/s over1.70R,
        // representative of fast coronal-hole wind, not an extreme CME.
        let flow = pow(0.5+0.5*sin(u*59.0-time*1.1+seed*TAU), 7.0);
        color = vec3<f32>(0.66,0.96,1.45);
        let fade_out = 1.0-smoothstep(0.58,1.0,u);
        emission = (0.0004 + flow*0.015) * pow(1.0-u*0.65,2.0) * fade_out;
    } else if (family == 3u) {
        let h = solar_plasma_hash(f32(in.tags.z) + 3.0*917.0);
        let age = fract(doc_sun_clock(time)/(150.0+h.z*170.0)+h.x);
        let life = smoothstep(0.0,0.10,age)*(1.0-smoothstep(0.76,1.0,age));
        color = vec3<f32>(1.65,0.17,0.035);
        emission = life * (1.0-u) * 0.12;
        opacity = life * (1.0-u) * 0.025;
    } else if (family == 4u) {
        let phase = fract(time*0.08+seed);
        let wrapped = min(abs(u-phase), 1.0-abs(u-phase));
        let packet = exp(-pow(wrapped/0.032,2.0));
        let rising = 0.5-0.5*sin(u*TAU);
        color = mix(vec3<f32>(0.95,0.12,0.025),vec3<f32>(1.8,1.12,0.38),rising);
        emission = 0.018+0.22*packet;
    } else {
        // Discrete scattering packets reveal transport direction without
        // rendering the trajectory as a permanent radial wire.
        let phase = fract(time*(0.034+seed*0.009)+seed*17.31);
        let packet = exp(-pow((u-phase)/0.007,2.0));
        let birth = smoothstep(0.0,0.018,phase)*(1.0-smoothstep(0.96,1.0,phase));
        color = mix(vec3<f32>(1.15,1.65,2.1),vec3<f32>(1.8,0.73,0.14),u);
        emission = packet*birth*0.62;
    }
    let alpha = 1.0-exp(-column*opacity);
    let radiance = color * column * emission;
    if (max(radiance.r,max(radiance.g,radiance.b)) < 1e-5 && alpha < 1e-5) { discard; }
    let solar_radius = inst.pos_radius.w * inst.phys_a.y;
    let world = inst.pos_radius.xyz + quat_rotate(normalize(inst.rot_quat), (ro+rd*depth_t)*solar_radius);
    let clip = world_pos_to_clip(world,in.tags.w);
    var out: SurfaceFsOut;
    // Absorption is composited on the opaque star before atmospheric emission.
    // Products of transmittance commute, and optically thin strand emission is
    // additive. Inter-strand extinction is intentionally not approximated by
    // arbitrary submission order.
    out.color = select(vec4<f32>(radiance,0.0),vec4<f32>(0.0,0.0,0.0,alpha),absorption);
    out.depth = reversed_z_depth(clip.z,clip.w);
    return out;
}

@fragment fn fs_solar_plasma(in: SolarPlasmaVertex) -> SurfaceFsOut {
    return solar_plasma_fragment(in,false);
}
@fragment fn fs_solar_absorption(in: SolarPlasmaVertex) -> SurfaceFsOut {
    return solar_plasma_fragment(in,true);
}
