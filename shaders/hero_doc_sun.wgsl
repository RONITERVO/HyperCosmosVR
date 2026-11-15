// HyperCosmosVR — Solar photosphere and shared active-region frame.
// Concatenated after doc_common and before hero_doc_cosmos / hero_solar_corona.
// Solar-local lengths are fractions of the physical photospheric radius. The
// observer frame co-rotates with this region; evolving plasma does not rotate the
// entire authored region away from a stationary observing pose.
const DOC_SUN_RADIUS_M: f32 = 695700000.0;
const DOC_SUN_REGION_CENTER: vec3<f32> = vec3<f32>(0.0, 0.3420201433, 0.9396926208);
const DOC_SUN_REGION_EAST: vec3<f32> = vec3<f32>(1.0, 0.0, 0.0);
const DOC_SUN_REGION_NORTH: vec3<f32> = vec3<f32>(0.0, 0.9396926208, -0.3420201433);
const DOC_SUN_BIPOLE_HALF_SEPARATION: f32 = 0.055;
const DOC_SUN_TIME_SCALE: f32 = 30.0;

fn doc_sun_region_frame() -> mat3x3<f32> {
    return mat3x3<f32>(DOC_SUN_REGION_EAST, DOC_SUN_REGION_NORTH, DOC_SUN_REGION_CENTER);
}

fn doc_sun_region_uv(direction: vec3<f32>) -> vec2<f32> {
    let n = normalize(direction);
    return vec2<f32>(atan2(dot(n, DOC_SUN_REGION_EAST), dot(n, DOC_SUN_REGION_CENTER)),
        asin(clamp(dot(n, DOC_SUN_REGION_NORTH), -1.0, 1.0)));
}

// Index0 is the western/negative-E footpoint; index1 the eastern/positive-E footpoint.
fn doc_sun_spot_direction(index: u32) -> vec3<f32> {
    let side = select(-1.0, 1.0, index == 1u);
    return normalize(DOC_SUN_REGION_CENTER + DOC_SUN_REGION_EAST * (side * DOC_SUN_BIPOLE_HALF_SEPARATION));
}

fn doc_sun_spot_umbra_radius(index: u32) -> f32 {
    return select(0.007, 0.0058, index == 1u);
}

fn doc_sun_spot_penumbra_radius(index: u32) -> f32 {
    return select(0.028, 0.023, index == 1u);
}

fn doc_sun_clock(time: f32) -> f32 {
    return time * DOC_SUN_TIME_SCALE;
}

// The eruption and outflow observations use an explicitly instrument-mapped
// EUV atmosphere. Other observations retain the filtered visible continuum.
fn doc_sun_euv_weight(inst: HeroVolumeInstance) -> f32 {
    let feature = doc_feat(inst);
    return select(0.0, 1.0, feature == 3u || feature == 4u);
}

// Shared open-field source for low-corona emissivity and resolved wind roots.
// It is an atmospheric coronal-hole mask, never a photospheric dark sunspot.
// The source footprint spans approximately +/-0.09R east and +/-0.045R north.
fn doc_sun_open_region(direction: vec3<f32>) -> f32 {
    let n = normalize(direction);
    let center = normalize(DOC_SUN_REGION_CENTER + DOC_SUN_REGION_NORTH * 0.315);
    // The full perturbed support is inside this cap. Most atmospheric samples
    // cannot contribute to the hole and must not evaluate two fine noise fields.
    if (dot(n,center) < 0.97) { return 0.0; }
    let north = cross(center, DOC_SUN_REGION_EAST);
    let local = vec2<f32>(dot(n, DOC_SUN_REGION_EAST), dot(n, north));
    if (abs(local.x) > 0.18 || abs(local.y) > 0.11) { return 0.0; }
    let warped = local + vec2<f32>(0.017*sin(local.y*65.0),0.009*sin(local.x*48.0));
    let shape = warped / vec2<f32>(0.115, 0.065);
    // A softened irregular footprint encloses the distributed funnel roots;
    // its persistent boundary follows magnetic-network-scale structure.
    let boundary = length(shape)
        + snoise3(n * 42.0) * 0.14 + snoise3(n * 97.0) * 0.065;
    let facing = smoothstep(0.86, 0.98, dot(n, center));
    return (1.0 - smoothstep(0.68, 1.22, boundary)) * facing;
}

fn doc_sun_hash33(value: vec3<f32>) -> vec3<f32> {
    var p = fract(value * vec3<f32>(0.1031, 0.1030, 0.0973));
    p += dot(p, p.yxz + vec3<f32>(33.33));
    return fract((p.xxy + p.yxx) * p.zyx);
}

fn doc_sun_hash11(value: f32) -> f32 {
    var p = fract(value * 0.1031);
    p *= p + 33.33;
    return fract(p * (p + p));
}

fn doc_sun_noise1(value: f32) -> f32 {
    let cell = floor(value);
    let f = fract(value);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(doc_sun_hash11(cell), doc_sun_hash11(cell + 1.0), u);
}

struct SunGranule {
    // Distance to the nearest power-cell boundary in units of one granule.
    edge: f32,
    // Independent cell life cycle; rising/hot centers give way to cooler lanes.
    heat: f32,
    seed: f32,
    detail: f32,
    // Tangential hot-plume profile and late-life internal cooling lanes.
    plume: f32,
    fragmentation: f32,
};

fn doc_sun_granule(n: vec3<f32>, physical_time: f32, footprint: f32) -> SunGranule {
    // 900 km convection cells, with 280–680 s independent life cycles. A 3D
    // power diagram intersected by the sphere avoids latitude seams and polar
    // stretching. Centers move continuously and weights grow/shrink; cell
    // boundaries actually migrate rather than translating a noise texture.
    let cell_radius = 900000.0 / DOC_SUN_RADIUS_M;
    let detail = 1.0 - smoothstep(cell_radius * 0.22, cell_radius * 1.15, footprint);
    var result: SunGranule;
    result.edge = 0.17;
    result.heat = 0.5;
    result.seed = 0.5;
    result.detail = detail;
    result.plume = 0.5;
    result.fragmentation = 0.0;
    if (detail <= 0.0) { return result; }
    let q0 = n / cell_radius;
    // Advected cellular distortion curves the lane network. It operates at
    // subgranular scale, never at the scale of continents on the star.
    let flow_clock = vec3<f32>(physical_time * 0.0011, physical_time * 0.0007, 0.0);
    let warp = vec3<f32>(snoise3(q0 * 0.95 + flow_clock),
        snoise3(q0 * 0.95 + vec3<f32>(19.7) + flow_clock),
        snoise3(q0 * 0.95 + vec3<f32>(-32.8) + flow_clock)) * 0.24;
    let q = q0 + warp;
    let base = floor(q);
    var first = 1e6;
    var second = 1e6;
    var nearest = vec3<f32>(0.0);
    var runner_up = vec3<f32>(0.0);
    var nearest_seed = vec3<f32>(0.5);
    var nearest_phase = 0.0;
    for (var z = -1; z <= 1; z = z + 1) {
        for (var y = -1; y <= 1; y = y + 1) {
            for (var x = -1; x <= 1; x = x + 1) {
                let cell = base + vec3<f32>(f32(x), f32(y), f32(z));
                let seed = doc_sun_hash33(cell);
                let phase = physical_time * TAU / (280.0 + seed.z * 400.0) + seed.x * TAU;
                let drift = vec3<f32>(sin(phase), sin(phase * 0.83 + seed.y * TAU), cos(phase * 0.71 + seed.z * TAU)) * 0.10;
                let center = cell + vec3<f32>(0.5) + (seed - vec3<f32>(0.5)) * 0.60 + drift;
                let delta = center - q;
                let growth = 0.5 + 0.5 * sin(phase);
                let distance = dot(delta, delta) - (growth - 0.5) * 0.22;
                if (distance < first) {
                    second = first;
                    runner_up = nearest;
                    first = distance;
                    nearest = center;
                    result.heat = growth;
                    result.seed = seed.y;
                    nearest_seed = seed;
                    nearest_phase = phase;
                } else if (distance < second) {
                    second = distance;
                    runner_up = center;
                }
            }
        }
    }
    result.edge = max(0.0, (second - first) / max(2.0 * length(runner_up - nearest), 1e-4));
    let displacement = q - nearest;
    let tangent_offset = displacement - n * dot(displacement, n);
    let direction_seed = nearest_seed - vec3<f32>(0.5);
    let tangent = normalize(cross(n, direction_seed + vec3<f32>(0.127, 0.329, 0.713)));
    let transverse = cross(n, tangent);
    let local = vec2<f32>(dot(tangent_offset, tangent), dot(tangent_offset, transverse));
    let aspect = vec2<f32>(0.44 + nearest_seed.x * 0.25, 0.33 + nearest_seed.z * 0.19);
    let plume_radius = dot(local / aspect, local / aspect);
    // A bright rising plume has an uneven interior, not a constant filled
    // polygon. About400km structure spreads slowly away from its center.
    let spreading = tangent_offset * (2.0 - result.heat * 0.35);
    let mottling = snoise3(nearest * 2.1 + spreading * 2.1
        + vec3<f32>(physical_time * 0.0013, 0.0, physical_time * 0.0008));
    result.plume = clamp(exp(-plume_radius * 0.65) + mottling * 0.23, 0.0, 1.0);
    // Cooling intrudes into older granules as off-center, curved lanes. The
    // cell then changes its power weight and neighbors take over its area.
    let aging = max(0.0, -cos(nearest_phase));
    let split_curve = local.x - 0.09 * sin(local.y * 8.0 + nearest_seed.z * TAU)
        - (nearest_seed.y - 0.5) * 0.24;
    let split_width = 0.035 + 0.025 * aging;
    let split_resolved = 1.0 - smoothstep(cell_radius * 0.045, cell_radius * 0.18, footprint);
    result.fragmentation = exp(-split_curve * split_curve / (split_width * split_width))
        * smoothstep(0.07, 0.27, length(local)) * aging * split_resolved;
    return result;
}

fn doc_sun_small_scale(n: vec3<f32>, physical_time: f32, footprint: f32) -> f32 {
    // Approximately120km substructure, explicitly removed when unresolved.
    // Its advection is much slower than cell growth, rather than whole-surface boil.
    let size = 120000.0 / DOC_SUN_RADIUS_M;
    let resolved = 1.0 - smoothstep(size * 0.25, size * 1.0, footprint);
    if (resolved <= 0.0) { return 0.0; }
    return snoise3(n / size + vec3<f32>(physical_time * 0.0017, physical_time * 0.0009, 0.0)) * resolved;
}

// An irregular set of radial channels, tied to the spot rather than the camera.
// The bright material travels outwards along them; the field-aligned skeleton
// stays persistent. Analytic pixel integration avoids a regular spoke comb.
fn doc_sun_penumbral_mean(radial_fraction: f32) -> vec3<f32> {
    // Angular ensemble means of the variable-length channels below. A constant
    // distant mean brightens short outer tips as they become unresolved.
    let means = array<vec3<f32>,12>(
        vec3<f32>(0.282,0.125,0.265), vec3<f32>(0.416,0.199,0.408),
        vec3<f32>(0.458,0.223,0.470), vec3<f32>(0.466,0.230,0.513),
        vec3<f32>(0.337,0.176,0.424), vec3<f32>(0.235,0.127,0.319),
        vec3<f32>(0.163,0.088,0.221), vec3<f32>(0.105,0.057,0.144),
        vec3<f32>(0.063,0.034,0.087), vec3<f32>(0.026,0.014,0.036),
        vec3<f32>(0.004,0.002,0.005), vec3<f32>(0.0));
    let coordinate = clamp(radial_fraction*10.0,0.0,10.999);
    let i = u32(coordinate);
    return mix(means[i],means[i+1u],fract(coordinate));
}

fn doc_sun_penumbral_fibrils(angle: f32, radial_fraction: f32, physical_time: f32,
    seed_offset: f32, angular_footprint: f32, radial_footprint: f32) -> vec3<f32> {
    let strand_count = 310.0;
    let radial = clamp(radial_fraction, 0.0, 1.15);
    let warped_angle = angle + 0.020 * sin(angle * 5.0 + seed_offset) * radial
        + 0.009 * sin(angle * 13.0 - radial * 2.7 + seed_offset) * radial;
    let coordinate = fract(warped_angle / TAU) * strand_count;
    let cell = floor(coordinate);
    var filaments = 0.0;
    var transport = 0.0;
    var peripheral_coverage = 0.0;
    let pixel_width = angular_footprint * strand_count / TAU;
    let radial_aa = max(radial_footprint * 0.8, 0.008);
    // The physical width is approximately constant along each filament before
    // tapering; a constant angular width instead makes rigid expanding spokes.
    let radius_ratio = mix(0.27, 1.0, clamp(radial, 0.0, 1.0));
    for (var i = -2; i <= 2; i = i + 1) {
        let id = cell + f32(i);
        let wrapped = id - floor(id / strand_count) * strand_count;
        let seed = doc_sun_hash11(wrapped + seed_offset * 53.0);
        let seed2 = doc_sun_hash11(wrapped * 1.731 + seed_offset * 71.0);
        let seed3 = doc_sun_hash11(wrapped * 2.317 + seed_offset * 113.0);
        let head_at = -0.05 + seed2 * 0.38;
        let tip_at = 0.55 + seed3 * 0.49;
        let progress = clamp((radial_fraction - head_at) / max(tip_at - head_at, 0.20), 0.0, 1.0);
        let extent = smoothstep(head_at-radial_aa, head_at+radial_aa, radial_fraction)
            * (1.0-smoothstep(tip_at-radial_aa*2.0,tip_at+radial_aa,radial_fraction));
        // Stable individual curvature and staggered heads/tips break the radial
        // comb without moving its footpoint frame or synchronizing lifetimes.
        let bend = (0.12 + seed2 * 0.60)
            * sin(progress * 3.2 + seed3 * TAU) * progress;
        let center = id + 0.20 + seed * 0.60 + bend;
        let d = coordinate - center;
        let head_length = 0.055 + seed3 * 0.035;
        let head = exp(-pow((radial_fraction-head_at-0.035)/head_length,2.0));
        let width = (0.11 + seed * 0.13) / radius_ratio
            * (1.0-progress*0.58) * (1.0+head*0.55);
        // Convolve the Gaussian channel with an approximate pixel footprint,
        // retaining integrated energy as several fibrils merge into a pixel.
        let filtered_width = sqrt(width * width + pixel_width * pixel_width * 0.20);
        let ridge = exp(-d * d / max(filtered_width * filtered_width, 1e-6)) * width / filtered_width;
        let core_width = width * 0.26;
        let filtered_core = sqrt(core_width*core_width + pixel_width*pixel_width*0.20);
        let dark_core = exp(-d*d/max(filtered_core*filtered_core,1e-6)) * core_width/filtered_core
            * smoothstep(0.12,0.38,progress);
        let strand = max(0.0,ridge-dark_core*0.30) * extent;
        let outward_flow = doc_sun_noise1(radial_fraction*19.0-physical_time*0.0036+seed*37.0);
        filaments += strand * (0.70+seed*0.15+head*0.24);
        transport += strand * outward_flow * 0.85;
        peripheral_coverage = max(peripheral_coverage, ridge*extent);
    }
    let unresolved = smoothstep(0.8, 2.2, pixel_width);
    let resolved = vec3<f32>(clamp(filaments,0.0,1.0),clamp(transport,0.0,1.0),
        clamp(peripheral_coverage,0.0,1.0));
    return mix(resolved,doc_sun_penumbral_mean(radial_fraction),unresolved);
}

struct SunSpotMaterial {
    temperature: f32,
    coverage: f32,
    network: f32,
};

fn doc_sun_visible_radiance(temperature: f32) -> vec3<f32> {
    let reference_color = max(blackbody_rgb(5772.0), vec3<f32>(0.05));
    return blackbody_rgb(temperature) / reference_color * pow(temperature / 5772.0, 4.0);
}

fn doc_sun_spot_material(n: vec3<f32>, index: u32, physical_time: f32,
    footprint: f32, granule: SunGranule, small_scale: f32) -> SunSpotMaterial {
    let center = doc_sun_spot_direction(index);
    let east = normalize(DOC_SUN_REGION_EAST - center * dot(DOC_SUN_REGION_EAST, center));
    let north = cross(center, east);
    let local = vec2<f32>(dot(n, east), dot(n, north));
    let radial = length(local);
    let facing = smoothstep(0.85, 0.98, dot(n, center));
    let penumbra = doc_sun_spot_penumbra_radius(index);
    var result: SunSpotMaterial;
    result.temperature = 5772.0;
    result.coverage = 0.0;
    result.network = exp(-radial * radial / (penumbra * penumbra * 9.0)) * facing;
    if (radial > penumbra * 1.5 || facing <= 0.0) { return result; }

    let angle = atan2(local.y, local.x);
    let circle = vec3<f32>(cos(angle), sin(angle), f32(index) * 2.7);
    let broad_shape = snoise3(circle * 2.1)*0.65 + snoise3(circle * 4.3)*0.35;
    let fringe = snoise3(circle * 8.7);
    let umbra_r = doc_sun_spot_umbra_radius(index) * (1.0 + broad_shape * 0.16 + fringe * 0.025);
    let penumbra_r = penumbra * (1.0 + broad_shape * 0.13 + fringe * 0.030);
    let aa = max(footprint * 0.8, 0.00008);
    let umbra = 1.0 - smoothstep(umbra_r - aa * 2.0, umbra_r + aa * 2.0, radial);
    let radial_fraction = (radial - umbra_r) / max(penumbra_r - umbra_r, 0.001);
    let radial_footprint = footprint / max(penumbra_r - umbra_r, 0.001);
    let angular_footprint = footprint / max(radial, umbra_r * 0.45);
    let fibrils = doc_sun_penumbral_fibrils(angle, radial_fraction, physical_time,
        f32(index) * 4.17 + 1.3, angular_footprint, radial_footprint);
    // The outer boundary is formed partly by individual tapered filament tips;
    // quiet granulation shows between them instead of a uniform circular rim.
    let penumbral_base = 1.0-smoothstep(0.72-radial_footprint,0.96+radial_footprint,radial_fraction);
    let peripheral = fibrils.z * smoothstep(0.52,0.80,radial_fraction);
    let penumbral_extent = max(penumbral_base,peripheral*0.95);
    let penumbral_temperature = 4680.0 + clamp(radial_fraction,0.0,1.0) * 460.0
        + fibrils.x * 580.0 + fibrils.y * 180.0;
    // Umbrae remain cool emitting plasma. Sparse hotter dots and a granular
    // light bridge are structures, not a black circular decal with a glow ring.
    let dot_field = pow(max(0.0, small_scale + 0.12), 4.0);
    var umbral_temperature = 3970.0 + dot_field * 1600.0 + granule.heat * 90.0;
    if (index == 0u) {
        let bridge_curve = local.y - local.x * 0.28 - umbra_r * 0.13 * sin(local.x / umbra_r * 3.1);
        let bridge_width = 0.00038 + aa;
        let bridge = exp(-bridge_curve * bridge_curve / (bridge_width * bridge_width))
            * (1.0 - smoothstep(umbra_r * 0.70, umbra_r * 1.04, abs(local.x)));
        let bridge_temp = 5350.0 + (0.5 + 0.5 * granule.heat) * 240.0;
        umbral_temperature = mix(umbral_temperature, bridge_temp, bridge * 0.92);
    }
    result.temperature = mix(penumbral_temperature, umbral_temperature, umbra);
    result.coverage = penumbral_extent * facing;
    return result;
}

fn shade_doc_sun_surface(p_surf: vec3<f32>, n_surf: vec3<f32>, v_dir: vec3<f32>,
    inst: HeroVolumeInstance, time: f32, pixel_footprint: f32) -> vec3<f32> {
    let mu = clamp(dot(n_surf, v_dir), 0.0, 1.0);
    let n = normalize(p_surf);
    let physical_time = doc_sun_clock(time);
    // Convert a screen pixel to a conservative surface footprint at the limb.
    let footprint = max(pixel_footprint / max(mu, 0.12), 1e-7);
    let granule = doc_sun_granule(n, physical_time, footprint);
    let small_scale = doc_sun_small_scale(n, physical_time, footprint);
    // Smooth thermal profiles preserve variation throughout each granule.
    // Bright upflows are rounded and irregular; cool lanes have variable width
    // and develop internal branches rather than outlining flat white tiles.
    let lane_width = 0.046 + (1.0 - granule.heat) * 0.026;
    let upflow = 1.0 - exp(-pow(granule.edge / lane_width, 1.35));
    let interior_heat = 0.57 + 0.43 * granule.plume;
    let cell_temperature = 5150.0 + granule.heat * 140.0
        + upflow * (990.0 * interior_heat - granule.fragmentation * 320.0);
    // The distant mean is not another broad noise octave. Real granules become
    // unresolved at a whole-Sun view instead of growing into continent-sized blobs.
    var magnetic_network = 0.0;
    var spot_coverage = 0.0;
    var spot_radiance = vec3<f32>(0.0);
    for (var spot_index = 0u; spot_index < 2u; spot_index = spot_index + 1u) {
        let spot = doc_sun_spot_material(n, spot_index, physical_time, footprint, granule, small_scale);
        if (spot.coverage > 0.0) {
            spot_radiance += doc_sun_visible_radiance(spot.temperature) * spot.coverage * (1.0 - spot_coverage);
        }
        magnetic_network = max(magnetic_network, spot.network);
        spot_coverage += spot.coverage * (1.0 - spot_coverage);
    }
    // Bright magnetic concentrations occupy intergranular lanes, especially near
    // the limb where their hot walls are exposed. They form broken networks around
    // the active region; no arbitrary concentric emissive halo is added.
    let lane = 1.0 - smoothstep(0.018, 0.10, granule.edge);
    let facular_detail = mix(0.06, lane * pow(max(0.0, small_scale + 0.25), 2.0), granule.detail);
    let faculae = magnetic_network * (1.0 - spot_coverage) * facular_detail * (0.18 + 0.82 * pow(1.0 - mu, 1.5));
    let temperature = cell_temperature + small_scale * (70.0 + upflow * 90.0) + faculae * 620.0;
    // Filter emitted radiance, not temperature. The sampled power-cell ensemble
    // has measured mean relative radiance (1.0764,1.0997,1.1181); normalize that
    // mean before fading to the unresolved photosphere so approaching it does
    // not change its total energy or white balance.
    let resolved = doc_sun_visible_radiance(temperature) / vec3<f32>(1.0764, 1.0997, 1.1181);
    let quiet = mix(vec3<f32>(1.0), resolved, granule.detail);
    var radiance = quiet * (1.0 - spot_coverage) + spot_radiance;
    if (doc_feat(inst) == 1u || doc_feat(inst) == 2u) {
        // Explicit contrast-enhanced visible-light observing mode. Preserve
        // temperature-derived chromaticity and the real limb law; redistribute
        // display contrast around the quiet mean to expose cool lanes/umbrae.
        let luminance = dot(radiance,vec3<f32>(0.2126,0.7152,0.0722));
        radiance *= pow(max(luminance,1e-4),1.1) / (1.0+0.063*granule.detail*granule.detail);
    }
    // Broad visible-band limb darkening and a neutral filtered photosphere.
    // Coronal instrument colors are handled separately, never painted onto this surface.
    let limb = 1.0 - 0.49 * (1.0 - mu) - 0.16 * pow(1.0 - mu, 2.0);
    // Neutral solar-filter exposure is deliberately below the tone-map shoulder:
    // the emitted spectrum stays white-balanced while its thermal contrast and
    // limb darkening remain visible. This is not an unfiltered human-eye view.
    // EUV features retain only a faint spatial reference to the continuum;
    // their coronal emission and dark open source live in the volume above it.
    let continuum_weight = mix(1.0, 0.035, doc_sun_euv_weight(inst));
    return radiance * limb * 0.58 * continuum_weight;
}
