// Analytic solar interior, exposed only by the explicit fusion/core observation.
// Requires common_oled + doc_common. Coordinates share the unrotated Sun frame:
// +Y rotation axis; active region at latitude +20 degrees on the +Z hemisphere.
// The removed octant is E>0, N>0, C>0. This is an explanatory section through
// actual geometry, not a hole or a luminous icon painted onto the photosphere.
// Radius fractions: NASA/MSFC solar interior (core ~0.25R; convection >~0.70R).
// https://solarscience.msfc.nasa.gov/interior.shtml

struct SolarCutawayHit {
    t: f32,
    normal: vec3<f32>,
    // 0=photosphere, 1=fusion core, 2=radiative cut wall, 3=convective cut wall.
    region: u32,
};

fn solar_cutaway_coordinates(p: vec3<f32>) -> vec3<f32> {
    return vec3<f32>(p.x, dot(p, vec3<f32>(0.0, 0.939692621, -0.342020143)),
        dot(p, vec3<f32>(0.0, 0.342020143, 0.939692621)));
}

fn solar_cutaway_open(p: vec3<f32>) -> bool {
    return all(solar_cutaway_coordinates(p) > vec3<f32>(0.0));
}

// Nearest opaque retained geometry. Returning 1e30 means the ray stays clear.
// The same function clips coronal emission and plasma strands against the true
// core/walls, allowing the opening to reveal deeper structure without back-side
// emission leaking through the remaining photosphere.
fn solar_cutaway_hit(ro: vec3<f32>, rd: vec3<f32>, radius: f32) -> SolarCutawayHit {
    var hit = SolarCutawayHit(1e30, vec3<f32>(0.0, 0.0, 1.0), 0u);
    let sphere = intersect_sphere(ro, rd, radius);
    for (var side = 0u; side < 2u; side += 1u) {
        let t = select(sphere.x, sphere.y, side == 1u);
        if (sphere.y > 0.0 && t > 0.0 && t < hit.t) {
            let p = ro + rd * t;
            if (!solar_cutaway_open(p)) {
                hit = SolarCutawayHit(t, normalize(p), 0u);
            }
        }
    }

    let core_r = radius * 0.25;
    let core = intersect_sphere(ro, rd, core_r);
    let core_t = select(core.x, core.y, core.x <= 0.0);
    if (core.y > 0.0 && core_t > 0.0 && core_t < hit.t) {
        hit = SolarCutawayHit(core_t, normalize(ro + rd * core_t), 1u);
    }

    let axes = array<vec3<f32>, 3>(vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 0.939692621, -0.342020143),
        vec3<f32>(0.0, 0.342020143, 0.939692621));
    for (var axis = 0u; axis < 3u; axis += 1u) {
        let n = axes[axis];
        let denom = dot(rd, n);
        if (abs(denom) > 1e-7) {
            let t = -dot(ro, n) / denom;
            if (t > 0.0 && t < hit.t) {
                let p = ro + rd * t;
                let radial = length(p);
                let c = solar_cutaway_coordinates(p);
                let a = (axis + 1u) % 3u;
                let b = (axis + 2u) % 3u;
                if (c[a] >= 0.0 && c[b] >= 0.0 && radial <= radius && radial >= core_r) {
                    // The physical face points into the removed wedge. Orient
                    // its shading normal toward an observer on either side.
                    let face_n = select(n, -n, denom > 0.0);
                    let region = select(2u, 3u, radial > radius * 0.70);
                    hit = SolarCutawayHit(t, face_n, region);
                }
            }
        }
    }
    return hit;
}

fn solar_interior_hash(value: f32) -> f32 {
    var p = fract(value * 0.1031);
    p *= p + 33.33;
    return fract(p * (p + p));
}

// Continuous qualitative temperature mapping, not visible blackbody colors or
// a tabulated solar model. No luminous ring marks the radiative/convection boundary.
fn solar_interior_stratification(radial: f32) -> vec3<f32> {
    let core_edge = vec3<f32>(0.92, 0.40, 0.115);
    let convection_base = vec3<f32>(0.34, 0.085, 0.021);
    let surface_base = vec3<f32>(0.21, 0.037, 0.009);
    let inner = mix(core_edge, convection_base, smoothstep(0.25, 0.70, radial));
    let outer = mix(convection_base, surface_base, smoothstep(0.70, 1.0, radial));
    return select(inner, outer, radial > 0.70);
}

// A stationary section through an authored convection roll. Phase follows a
// closed streamline: hot material rises on one side, spreads near the top,
// cools and returns on the other. The 20–30s display cycle is schematic; it is
// deliberately not assigned the photosphere's30x physical reference clock.
fn solar_interior_roll(angle: f32, radial: f32, cell_id: f32, face_id: f32,
    time: f32, footprint: f32) -> vec4<f32> {
    let seed = solar_interior_hash(cell_id * 13.7 + face_id * 117.3 + 8.9);
    let seed2 = solar_interior_hash(cell_id * 31.9 + face_id * 71.7 + 36.2);
    let center_angle = (cell_id + 0.5 + (seed - 0.5) * 0.23) * 0.205;
    let center_radius = 0.845 + (seed2 - 0.5) * 0.009;
    let height = 0.133 + seed * 0.009;
    let width = 0.067 + seed2 * 0.021;
    let x = (angle - center_angle) * radial / width;
    let y = (radial - center_radius) / height;
    // Broad rising columns and narrower returns depart from perfect ellipses.
    let spin = select(-1.0, 1.0, fract(cell_id * 0.5) < 0.25);
    let broad_upflow = 1.0 + 0.22 * tanh(x * spin * 2.0);
    let local = vec2<f32>(x / broad_upflow, y);
    let rho = length(local);
    let support = 1.0 - smoothstep(0.82, 1.17, rho);
    if (support <= 0.0) { return vec4<f32>(0.0); }
    let theta = atan2(local.y, local.x);
    let speed = 0.22 + seed2 * 0.085;
    let phase = theta - spin * time * speed;
    // Periodic angular coordinates have no seam or reset. Texture follows the
    // same closed motion as the thermal packets instead of scrolling over a wall.
    let flowing_coordinate = vec3<f32>(rho * 19.0, cos(phase) * 2.4, sin(phase) * 2.4);
    let fine_weight = 1.0 - smoothstep(0.0015, 0.006, footprint);
    let fine = snoise3(flowing_coordinate + vec3<f32>(seed * 37.0)) * fine_weight;
    let coarse = snoise3(vec3<f32>(rho * 7.0, cos(phase) * 1.1, sin(phase) * 1.1)
        + vec3<f32>(seed2 * 23.0));
    let moving_wisp = pow(max(0.0, 0.5 + 0.5 * cos(phase + seed * TAU)), 7.0)
        * exp(-pow((rho - 0.67) / 0.27, 2.0));
    let radial_motion = spin * local.x / max(rho, 0.12);
    let rising = smoothstep(-0.45, 0.60, radial_motion);
    let cooling_near_top = smoothstep(0.90, 0.995, radial);
    let warming_at_base = 1.0 - smoothstep(0.705, 0.79, radial);
    let thermal = clamp(rising * (1.0 - cooling_near_top * 0.55)
        + warming_at_base * 0.25, 0.0, 1.0);
    let quiet = solar_interior_stratification(radial);
    let cool = quiet * vec3<f32>(0.53, 0.39, 0.32);
    let hot = quiet * vec3<f32>(1.85, 2.05, 2.35);
    let color = mix(cool, hot, thermal)
        * (0.94 + fine * 0.12 + coarse * 0.11 + moving_wisp * 0.34);
    return vec4<f32>(color * support, support);
}

// Compress the enormous interior temperature range into explanatory colors;
// these are not a claim that naked-eye visible light distinguishes hot plasma
// this way. All positions and motions are anchored to the actual cut geometry.
fn shade_solar_cutaway(p: vec3<f32>, n: vec3<f32>, view: vec3<f32>, radius: f32,
    region: u32, time: f32, pixel_footprint: f32) -> vec3<f32> {
    let q = p / radius;
    let radial = length(q);
    let mu = max(dot(n, view), 0.0);
    let facing = 0.47 + 0.53 * sqrt(mu);
    let footprint = max(pixel_footprint / max(mu, 0.14), 1e-7);
    if (region == 1u) {
        // Subdued plasma structure, not discrete atoms or convection in the
        // radiative core. Slow local fluctuations do not move the whole texture.
        let structure = snoise3(q * 24.0);
        let resolved = 1.0 - smoothstep(0.003, 0.012, footprint);
        let grain = snoise3(q * 86.0) * resolved;
        let fluctuation = sin(time * 0.19 + structure * 5.0) * 0.018;
        let core = vec3<f32>(1.32, 0.72, 0.255)
            * (0.96 + structure * 0.065 + grain * 0.045 + fluctuation);
        return core * facing;
    }
    let base = solar_interior_stratification(radial);
    if (region == 2u) {
        // Bulk material here is comparatively steady; separate3D energy packets
        // show the repeated changes in direction of radiative transport.
        let resolved = 1.0 - smoothstep(0.003, 0.012, footprint);
        let grain = snoise3(q * 72.0) * resolved;
        return base * (0.98 + grain * 0.025) * facing;
    }

    let cut = abs(solar_cutaway_coordinates(q));
    var angle = atan2(cut.y, cut.z);
    var face_id = 0.0;
    if (cut.y < cut.x && cut.y <= cut.z) {
        angle = atan2(cut.x, cut.z);
        face_id = 1.0;
    } else if (cut.z < cut.x && cut.z < cut.y) {
        angle = atan2(cut.y, cut.x);
        face_id = 2.0;
    }
    let cell = floor(angle / 0.205);
    var rolls = vec4<f32>(0.0);
    for (var offset = -1; offset <= 1; offset += 1) {
        rolls += solar_interior_roll(angle, radial, cell + f32(offset), face_id, time, footprint);
    }
    let circulation = rolls.rgb / max(rolls.a, 1e-5);
    let layer = smoothstep(0.70, 0.725, radial) * (1.0 - smoothstep(0.98, 1.0, radial));
    let coverage = clamp(rolls.a, 0.0, 1.0) * layer;
    return mix(base, circulation, coverage) * facing;
}
