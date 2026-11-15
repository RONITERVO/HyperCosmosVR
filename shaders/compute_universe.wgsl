// HyperCosmosVR — Multi-Mode GPU Compute Particle & Field Advection
// Requires prelude: common_oled.wgsl
// Entry points:
//   - cs_advect_particles (@workgroup_size(256, 1, 1)) -> uses @group(0), @group(1)
//   - cs_field_grid       (@workgroup_size(8, 8, 1))   -> uses @group(0), @group(1), @group(2)

struct SimDispatchUniforms {
    // x = dt_sec, y = time_sec, z = damping [0..1], w = audio_reactivity_gain
    params_a: vec4<f32>,
    // x = scale_radius, y = spiral_tightness, z = field_strength, w = wave_frequency
    params_b: vec4<f32>,
    // xyz = primary attractor position (sector-local), w = attractor_a strength
    attractor_a: vec4<f32>,
    // xyz = secondary attractor / target position, w = attractor_b strength / charge
    attractor_b: vec4<f32>,
    // x = sim_mode (0..16 particle, 0..3 grid), y = particle_count, z = grid_width, w = grid_height
    counts: vec4<u32>,
};

// 64-byte storage particle record (matches instanced_sprites.wgsl particle read layout).
struct SimParticle {
    pos_age: vec4<f32>,    // xyz = sector-local pos, w = normalized age / phase [0..1)
    vel_mass: vec4<f32>,   // xyz = velocity, w = temperature_K or energy
    color_size: vec4<f32>, // rgb = linear HDR base tint, a = base sprite radius
    tags: vec4<u32>,       // x = sector_kind, y = sub_id (e.g. orbital n/l/m), z = seed_hash, w = flags
};

@group(1) @binding(0) var<uniform> u_sim: SimDispatchUniforms;
@group(1) @binding(1) var<storage, read_write> particles: array<SimParticle>;

@group(2) @binding(0) var field_in: texture_2d<f32>;
@group(2) @binding(1) var field_out: texture_storage_2d<rgba16float, write>;

fn unpack_orbital_n(packed: u32) -> i32 {
    return i32(clamp(packed & 0xFFu, 1u, 4u));
}

fn unpack_orbital_l(packed: u32) -> i32 {
    return i32((packed >> 8u) & 0xFFu);
}

fn eval_spherical_orbital_density(p: vec3<f32>, n_orb: i32, l_orb: i32) -> f32 {
    let r = max(length(p), 1e-4);
    let cos_t = clamp(p.y / r, -1.0, 1.0);
    let sin_t = sqrt(max(0.0, 1.0 - cos_t * cos_t));
    let phi = atan2(p.z, p.x);

    var angular: f32 = 1.0;
    if (l_orb == 1) {
        angular = abs(cos_t);
    } else if (l_orb == 2) {
        angular = abs(3.0 * cos_t * cos_t - 1.0) * 0.5 + 0.35 * sin_t * sin_t * abs(cos(2.0 * phi));
    } else if (l_orb >= 3) {
        angular = abs(sin_t * sin_t * sin_t * cos(3.0 * phi)) + 0.4 * abs(cos_t * (5.0 * cos_t * cos_t - 3.0));
    }

    let r_scaled = r * f32(n_orb) * 2.2;
    let radial = pow(r_scaled, f32(l_orb)) * exp(-r_scaled * 0.55);
    return clamp(radial * angular * 2.2, 0.0, 2.0);
}

@compute @workgroup_size(256, 1, 1)
fn cs_advect_particles(@builtin(global_invocation_id) gid: vec3<u32>) {
    let idx = gid.x;
    if (idx >= u_sim.counts.y) {
        return;
    }

    var pt = particles[idx];
    let dt = clamp(u_sim.params_a.x, 0.0001, 0.05);
    let time = u_sim.params_a.y;
    let damping = u_sim.params_a.z;
    let audio_gain = u_sim.params_a.w;
    let radius_scale = max(u_sim.params_b.x, 1e-3);
    let mode = u_sim.counts.x;

    let voice = u_audio.levels_a.x * audio_gain;
    let bass = u_audio.levels_a.z * audio_gain;
    let centroid = u_audio.levels_b.x;
    let beat = u_audio.levels_b.y;

    var pos = pt.pos_age.xyz;
    var age = pt.pos_age.w;
    var vel = pt.vel_mass.xyz;
    var energy = pt.vel_mass.w;
    var tint = pt.color_size.rgb;
    var size = pt.color_size.a;

    let seed_v = hash33(vec3<f32>(f32(pt.tags.z & 0xFFFFu), f32((pt.tags.z >> 16u) & 0xFFFFu), f32(idx) * 0.017));

    if (mode == 0u) {
        // Mode 0: Milky Way (05) Logarithmic Spiral Density Wave + Vertical Disk Harmonic
        let xz = pos.xz;
        let r = max(length(xz), 0.03 * radius_scale);
        let theta = atan2(xz.y, xz.x);
        let omega = u_sim.params_b.z / pow(r + 0.12 * radius_scale, 0.72);
        let spiral_phase = 4.0 * (theta - u_sim.params_b.y * log(r / radius_scale + 0.08));
        let arm_boost = 0.5 + 0.5 * cos(spiral_phase - time * 0.15);

        let tangent = vec3<f32>(-xz.y / r, 0.0, xz.x / r);
        let radial_pull = vec3<f32>(-xz.x / r, 0.0, -xz.y / r) * (sin(spiral_phase) * 0.18 * radius_scale);
        let vertical_spring = vec3<f32>(0.0, -pos.y * (1.8 + arm_boost * 1.4), 0.0);
        let swirl = curl_noise3(pos / (radius_scale * 0.35) + vec3<f32>(0.0, time * 0.04, 0.0)) * (0.06 * radius_scale);

        vel = mix(vel, tangent * (omega * r) + radial_pull + swirl, clamp(dt * 4.5, 0.0, 1.0));
        vel += vertical_spring * dt;
        pos += vel * dt;

        let kelvin = mix(3400.0, 19500.0, clamp(arm_boost * 0.75 + seed_v.x * 0.35 + bass * 0.25, 0.0, 1.0));
        tint = blackbody_rgb(kelvin) * (0.85 + arm_boost * 1.15 + bass * 0.6);
        age = fract(age + dt * 0.05);
    } else if (mode == 1u) {
        // Mode 1: Laniakea (07) Gravitational Watershed Streamlines into Great Attractor
        let to_ga = u_sim.attractor_a.xyz - pos;
        let dist_ga = max(length(to_ga), 0.05 * radius_scale);
        let dir_ga = to_ga / dist_ga;

        let to_sh = u_sim.attractor_b.xyz - pos;
        let dist_sh = max(length(to_sh), 0.08 * radius_scale);
        let dir_sh = to_sh / dist_sh;

        let shear = curl_noise3(pos / (radius_scale * 0.45) + vec3<f32>(time * 0.02)) * 0.42;
        let flow = dir_ga * (u_sim.attractor_a.w / (dist_ga * 0.6 + 0.15))
                 + dir_sh * (u_sim.attractor_b.w / (dist_sh * 0.9 + 0.25))
                 + shear;

        vel = mix(vel, flow * u_sim.params_b.z, clamp(dt * 3.0, 0.0, 1.0));
        pos += vel * dt;
        age += dt * 0.14;

        if (age >= 1.0 || dist_ga < 0.04 * radius_scale) {
            // Recycle cleanly back onto watershed rim without popping
            let rim_dir = normalize(seed_v * 2.0 - vec3<f32>(1.0) + vec3<f32>(1e-4));
            pos = u_sim.attractor_a.xyz + rim_dir * (radius_scale * (0.65 + 0.35 * seed_v.y));
            vel = vec3<f32>(0.0);
            age = 0.0;
        }
        let speed_glow = clamp(length(vel) * 0.6, 0.2, 2.4);
        tint = mix(vec3<f32>(0.18, 0.55, 1.4), vec3<f32>(1.8, 0.95, 0.45), clamp(1.0 - dist_ga / radius_scale, 0.0, 1.0)) * (speed_glow + voice * 0.5);
    } else if (mode == 2u) {
        // Mode 2: Cosmic Web (08) Zel'dovich Filaments & True Dark Voids
        let p_scaled = pos / (radius_scale * 0.28);
        let eps: f32 = 0.04;
        let f0 = abs(snoise3(p_scaled));
        let fx = abs(snoise3(p_scaled + vec3<f32>(eps, 0.0, 0.0)));
        let fy = abs(snoise3(p_scaled + vec3<f32>(0.0, eps, 0.0)));
        let fz = abs(snoise3(p_scaled + vec3<f32>(0.0, 0.0, eps)));
        // Push away from zero-crossing void interiors toward ridge walls & filament intersections
        let ridge_grad = vec3<f32>(f0 - fx, f0 - fy, f0 - fz) / eps;
        let drift = curl_noise3(p_scaled * 0.6 + vec3<f32>(time * 0.015)) * 0.25;

        vel = vel * damping + (ridge_grad * u_sim.params_b.z * 0.45 + drift) * dt;
        pos += vel * dt;
        // Soft toroidal wrap inside sector cube
        pos = clamp(pos, vec3<f32>(-radius_scale), vec3<f32>(radius_scale));

        let f1 = snoise3(p_scaled * 0.95 + vec3<f32>(4.1, -2.7, 3.3));
        let d_fil = length(vec2<f32>(f0, f1));
        let filament_density = smoothstep(0.068, 0.010, d_fil);
        tint = mix(vec3<f32>(0.18, 0.28, 0.88), vec3<f32>(0.98, 0.78, 1.38), filament_density) * (filament_density * (0.95 + bass * 0.2));
        age = fract(age + dt * 0.04);
    } else if (mode == 3u) {
        // Mode 3: QCD Quark-Gluon Color Flux Tubes (13)
        let center = u_sim.attractor_a.xyz;
        let offset = pos - center;
        let r = length(offset);
        let color_charge = pt.tags.y % 3u;
        let angle_offset = f32(color_charge) * (TAU / 3.0) + time * (2.6 + bass * 1.5);
        let target_orbit = center + vec3<f32>(cos(angle_offset), sin(angle_offset * 1.3) * 0.45, sin(angle_offset)) * (radius_scale * 0.38);

        let confinement = (target_orbit - pos) * (14.0 + u_sim.params_b.z * 8.0);
        let gluon_turb = curl_noise3(pos * (4.0 / radius_scale) + vec3<f32>(time * 1.8)) * (radius_scale * (2.2 + centroid * 2.0));
        vel = vel * 0.88 + (confinement + gluon_turb) * dt;
        pos += vel * dt;
        if (r > radius_scale * 1.1) {
            pos = center + normalize(offset) * radius_scale;
        }

        if (color_charge == 0u) {
            tint = vec3<f32>(2.4, 0.28, 0.22);
        } else if (color_charge == 1u) {
            tint = vec3<f32>(0.22, 2.4, 0.55);
        } else {
            tint = vec3<f32>(0.28, 0.65, 2.8);
        }
        tint *= (0.48 + voice * 0.42 + 0.16 * sin(time * 9.0 + f32(idx)));
        age = fract(age + dt * 0.45);
    } else if (mode == 4u) {
        // Mode 4: Electron Spherical-Harmonic Orbitals (13)
        let n_orb = unpack_orbital_n(pt.tags.y);
        let l_orb = unpack_orbital_l(pt.tags.y);
        let center = u_sim.attractor_a.xyz;
        let rel = (pos - center) / radius_scale;
        let eps: f32 = 0.03;
        let psi0 = eval_spherical_orbital_density(rel, n_orb, l_orb);
        let psix = eval_spherical_orbital_density(rel + vec3<f32>(eps, 0.0, 0.0), n_orb, l_orb);
        let psiy = eval_spherical_orbital_density(rel + vec3<f32>(0.0, eps, 0.0), n_orb, l_orb);
        let psiz = eval_spherical_orbital_density(rel + vec3<f32>(0.0, 0.0, eps), n_orb, l_orb);
        let grad_psi = vec3<f32>(psix - psi0, psiy - psi0, psiz - psi0) / eps;

        let azim = vec3<f32>(-rel.z, 0.0, rel.x) * (1.8 + f32(l_orb) * 0.6);
        let jitter = curl_noise3(rel * 3.5 + vec3<f32>(time * 0.7)) * 0.55;
        vel = mix(vel, (grad_psi * 1.4 + azim + jitter) * radius_scale, clamp(dt * 6.0, 0.0, 1.0));
        pos += vel * dt;
        age = fract(age + dt * 0.25);

        let phase_col = iridescence_spectral(psi0 * 0.5, age + f32(l_orb) * 0.2, centroid * 0.3);
        tint = phase_col * (0.42 + psi0 * 0.78 + voice * 0.35);
    } else if (mode == 5u) {
        // Mode 5: Planck Spin-Foam & Quantum Vacuum Fluctuations (14 / 13)
        let p_norm = pos / radius_scale;
        let foam = curl_noise3(p_norm * 5.0 + vec3<f32>(time * 1.1));
        vel = mix(vel, foam * radius_scale * 0.9, clamp(dt * 5.0, 0.0, 1.0));
        pos += vel * dt;
        age += dt * (0.9 + seed_v.x * 1.4);
        if (age >= 1.0) {
            pos = (seed_v * 2.0 - vec3<f32>(1.0)) * radius_scale;
            age = 0.0;
        }
        let flash = sin(age * PI);
        let spec = iridescence_spectral(dot(normalize(vel + vec3<f32>(1e-4)), u_frame.cam_forward.xyz), seed_v.z + time * 0.1, voice);
        tint = spec * (flash * flash * (0.95 + bass * 0.75));
    } else if (mode == 6u) {
        // Mode 6: Turing / Belousov-Zhabotinsky Protocell & Cellular Self-Organization (15 Z09, 16 Z00..=Z14)
        let center = u_sim.attractor_a.xyz;
        let offset = pos - center;
        let r = max(length(offset), 1e-4);
        let target_shell = radius_scale * 0.68;
        let shell_force = (offset / r) * ((target_shell - r) * 3.2);
        let bz_wave = sin(r * (14.0 / radius_scale) - time * 2.4 + atan2(offset.z, offset.x) * 3.0);
        let tangent_flow = curl_noise3(offset / (radius_scale * 0.35) + vec3<f32>(0.0, time * 0.25, 0.0)) * (radius_scale * 0.6);

        vel = vel * 0.90 + (shell_force + tangent_flow + (offset / r) * bz_wave * 0.35 * radius_scale) * dt;
        pos += vel * dt;
        age = fract(age + dt * 0.18);
        let bio_teal = mix(vec3<f32>(0.12, 0.95, 0.78), vec3<f32>(0.95, 0.35, 1.35), 0.5 + 0.5 * bz_wave);
        tint = bio_teal * (0.7 + 0.9 * max(bz_wave, 0.0) + voice * 0.5);
    } else if (mode == 7u) {
        // Mode 7: Living Neural Connectome Firing Cascades (16 Z15..=Z18 / 17 Mind)
        let axon_dir = normalize(u_sim.attractor_b.xyz - u_sim.attractor_a.xyz + vec3<f32>(1e-4));
        let curl_branch = curl_noise3(pos / (radius_scale * 0.3) + vec3<f32>(seed_v.x * 5.0, time * 0.08, 0.0));
        let spike_speed = radius_scale * (0.8 + voice * 1.4 + sin(beat * TAU) * 0.35);

        vel = mix(vel, (axon_dir * 0.65 + curl_branch * 0.75) * spike_speed, clamp(dt * 4.0, 0.0, 1.0));
        pos += vel * dt;
        age += dt * (0.45 + seed_v.y * 0.55);
        if (age >= 1.0) {
            pos = u_sim.attractor_a.xyz + (seed_v * 2.0 - vec3<f32>(1.0)) * (radius_scale * 0.18);
            age = 0.0;
        }
        let firing = pow(sin(age * PI), 2.5);
        let syn_col = mix(vec3<f32>(0.22, 0.78, 1.85), vec3<f32>(2.4, 1.45, 0.42), clamp(voice * 1.3 + firing * 0.4, 0.0, 1.0));
        tint = syn_col * (0.55 + firing * 2.2);
    } else if (mode == 8u) {
        // Mode 8: App 02 Cymatic Standing-Wave Ripples (02 Music)
        let freq_mode = max(u_sim.params_b.w, 3.0);
        let r = max(length(pos.xz), 1e-4);
        let theta = atan2(pos.z, pos.x);
        let chladni = sin(freq_mode * r / radius_scale * PI) * cos(floor(3.0 + u_audio.synth_b.y * 3.0) * theta + time * 1.2);
        let radial_dir = vec3<f32>(pos.x / r, 0.0, pos.z / r);
        let node_force = radial_dir * (-chladni * radius_scale * 1.8);
        let lift = vec3<f32>(0.0, chladni * radius_scale * 0.45 - pos.y * 2.4, 0.0);

        vel = vel * 0.86 + (node_force + lift) * dt;
        pos += vel * dt;
        let note_rgb = hsl_to_rgb(u_audio.synth_b.x, 0.95, 0.62);
        let node_glow = exp(-abs(chladni) * 3.8);
        tint = note_rgb * (0.55 + node_glow * (1.8 + u_audio.synth_b.z * 2.2));
        age = fract(age + dt * 0.35);
    } else if (mode == 9u) {
        // Mode 9: App 01 Chromatic Dust & Gentle Focus-Dwell Bioluminescent Motes
        let drift = curl_noise3(pos / (radius_scale * 0.5) + vec3<f32>(time * 0.12));
        let dwell = u_frame.focus_ray.w;
        let focus_target = u_frame.focus_ray.xyz * (radius_scale * 0.55);
        let attract = (focus_target - pos) * (dwell * 1.6);

        vel = vel * 0.92 + (drift * radius_scale * 0.32 + attract) * dt;
        pos += vel * dt;
        if (length(pos) > radius_scale * 1.25) {
            pos = (seed_v * 2.0 - vec3<f32>(1.0)) * (radius_scale * 0.9);
        }
        let accent = max(u_frame.region_accent.rgb, vec3<f32>(0.25, 0.55, 1.0));
        tint = accent * (0.28 + dwell * 0.72 + voice * 0.42);
        age = fract(age + dt * 0.1);
    } else if (mode == 10u) {
        // Mode 10: Solar System Asteroid & Kuiper Keplerian Ring (04)
        let r_xz = max(length(pos.xz), radius_scale * 0.15);
        let r_norm = r_xz / radius_scale;
        let tang = vec3<f32>(-pos.z / r_xz, 0.0, pos.x / r_xz);
        let v_kep = (0.42 / sqrt(max(r_norm, 0.12))) * radius_scale;
        vel = mix(vel, tang * v_kep, clamp(dt * 4.0, 0.0, 1.0));
        pos += vel * dt;
        pos.y *= 0.985;
        let belt_c = mix(vec3<f32>(1.15, 0.80, 0.42), vec3<f32>(0.32, 0.82, 1.32), smoothstep(0.45, 0.92, r_norm));
        // Jovian 3:1, 5:2, 2:1 Kirkwood gaps + outer 3:2 Plutino Kuiper resonance modulation
        let k31 = smoothstep(0.015, 0.040, abs(r_norm - 0.35));
        let k52 = smoothstep(0.015, 0.038, abs(r_norm - 0.40));
        let k21 = smoothstep(0.016, 0.042, abs(r_norm - 0.46));
        let plutino = select(1.0, 0.48 + 0.68 * exp(-abs(r_norm - 0.78) * 14.0), r_norm > 0.72);
        tint = belt_c * (k31 * k52 * k21) * plutino * (0.42 + 0.25 * sin(seed_v.x * TAU + time * 1.4));
        age = fract(age + dt * 0.15);
    } else if (mode == 11u) {
        // Mode 11: Geodynamo & Atmospheric Trade Winds (03, 18) hugged smoothly inside magnetosphere
        let r = max(length(pos), 1e-4);
        let lat = pos.y / r;
        let tor = vec3<f32>(-pos.z, sin(lat * PI * 3.0) * 0.35 * r, pos.x);
        vel = mix(vel, normalize(tor + vec3<f32>(1e-4)) * radius_scale * 0.38, clamp(dt * 3.5, 0.0, 1.0));
        pos += vel * dt;
        if (r > radius_scale * 0.68 || r < radius_scale * 0.24) {
            pos = normalize(seed_v * 2.0 - vec3<f32>(1.0)) * (radius_scale * (0.28 + 0.36 * seed_v.y));
        }
        let radial_fade = smoothstep(radius_scale * 0.70, radius_scale * 0.42, r);
        tint = mix(vec3<f32>(1.10, 0.42, 0.16), vec3<f32>(0.22, 0.95, 1.28), smoothstep(0.32, 0.60, r / radius_scale)) * (0.55 * radial_fade);
        age = fract(age + dt * 0.18);
    } else if (mode == 12u) {
        // Mode 12: Prebiotic Rayleigh-Benard Convection Micro-Swarm (15, 13)
        let swirl = curl_noise3(pos / (radius_scale * 0.38) + vec3<f32>(0.0, time * 0.2, 0.0));
        let restore = -pos * (1.2 / max(radius_scale, 1e-4));
        vel = vel * 0.90 + (swirl * radius_scale * 0.85 + restore * radius_scale * 0.25) * dt;
        pos += vel * dt;
        let temp_y = clamp(pos.y / (radius_scale * 0.8) + 0.5, 0.0, 1.0);
        tint = mix(vec3<f32>(1.05, 0.42, 0.16), vec3<f32>(0.18, 0.98, 0.68), temp_y) * 0.58;
        age = fract(age + dt * 0.22);
    } else if (mode == 13u) {
        // Mode 13: Cosmological Redshift Expansion Stream & Photon Flux (11, 12, 16)
        let r = max(length(pos), 1e-4);
        let radial = pos / r;
        vel = mix(vel, radial * radius_scale * 0.42, clamp(dt * 3.0, 0.0, 1.0));
        pos += vel * dt;
        if (r > radius_scale * 1.05) {
            pos = (seed_v * 2.0 - vec3<f32>(1.0)) * (radius_scale * 0.12);
        }
        let z_shift = clamp(r / radius_scale, 0.0, 1.0);
        tint = mix(vec3<f32>(0.24, 0.92, 1.25), vec3<f32>(1.15, 0.20, 0.16), z_shift) * (0.62 - 0.18 * z_shift);
        age = z_shift;
    } else if (mode == 14u) {
        // Mode 14: Transformer Token & Optical Bus Lattice Highway (19, 20, 22)
        let cell = floor(pos / (radius_scale * 0.25) + 4.0);
        let dir_ax = u32(abs(cell.x + cell.y + cell.z)) % 3u;
        var axis_v = vec3<f32>(1.0, 0.0, 0.0);
        if (dir_ax == 1u) { axis_v = vec3<f32>(0.0, 1.0, 0.0); }
        else if (dir_ax == 2u) { axis_v = vec3<f32>(0.0, 0.0, 1.0); }
        let sgn = select(-1.0, 1.0, (idx % 2u) == 0u);
        vel = mix(vel, axis_v * (sgn * radius_scale * 0.85), clamp(dt * 8.0, 0.0, 1.0));
        pos += vel * dt;
        if (max(max(abs(pos.x), abs(pos.y)), abs(pos.z)) > radius_scale) {
            pos = (floor(seed_v * 6.0) - 2.5) * (radius_scale * 0.25);
        }
        tint = mix(vec3<f32>(0.20, 0.96, 1.24), vec3<f32>(1.18, 0.76, 0.24), fract(seed_v.z * 3.0)) * 0.56;
        age = fract(age + dt * 0.45);
    } else if (mode == 15u) {
        // Mode 15: SETI & 25-Petal Hyper-Lotus Equilibrium Convergence (21..25)
        let a = atan2(pos.z, pos.x);
        let r_tar = radius_scale * (0.48 + 0.16 * abs(cos(a * 12.5 + time * 0.18)));
        let r_xz = max(length(pos.xz), 1e-4);
        let pull = vec3<f32>(pos.x / r_xz * (r_tar - r_xz), -pos.y * 1.4, pos.z / r_xz * (r_tar - r_xz));
        let swirl = vec3<f32>(-pos.z, 0.0, pos.x) * 0.22;
        vel = vel * 0.90 + (pull + swirl) * dt * 2.0;
        pos += vel * dt;
        tint = hsl_to_rgb(fract(a / TAU + 1.0), 0.90, 0.56) * (0.45 + 0.25 * voice);
        age = fract(age + dt * 0.12);
    } else {
        // Mode 16: Relativistic Geodesic Lensing & Quadrupolar Strain Flow (14: Spacetime Z06..=Z09)
        let r_cyl = max(length(pos.xz), 0.04 * radius_scale);
        let a = atan2(pos.z, pos.x);
        let plus_strain = cos(2.0 * a - time * 2.4);
        let cross_strain = sin(2.0 * a - time * 2.4);
        let r_isco = radius_scale * (0.44 + 0.10 * plus_strain);
        let tangent = vec3<f32>(-pos.z / r_cyl, 0.0, pos.x / r_cyl);
        let radial = vec3<f32>(pos.x / r_cyl, 0.0, pos.z / r_cyl);
        let swirl_spd = 1.65 / sqrt(r_cyl / radius_scale + 0.22);
        let target_v = tangent * (swirl_spd * radius_scale)
                     + radial * ((r_isco - r_cyl) * 1.85)
                     + vec3<f32>(0.0, -pos.y * 2.4 + 0.18 * cross_strain * radius_scale, 0.0);
        vel = mix(vel * damping, target_v, clamp(dt * 4.2, 0.0, 1.0));
        pos += vel * dt;
        let doppler = clamp(0.5 + 0.5 * plus_strain, 0.0, 1.0);
        let geo_gold = vec3<f32>(1.18, 0.78, 0.28);
        let geo_cyan = vec3<f32>(0.28, 0.88, 1.32);
        tint = mix(geo_gold, geo_cyan, doppler) * (0.44 + 0.22 * bass);
        age = fract(age + dt * 0.14);
    }

    energy = clamp(luminance_rec709(tint), 0.0, 16.0);
    size = max(pt.color_size.a, 1e-4);

    pt.pos_age = vec4<f32>(pos, age);
    pt.vel_mass = vec4<f32>(vel, energy);
    pt.color_size = vec4<f32>(tint, size);
    particles[idx] = pt;
}

@compute @workgroup_size(8, 8, 1)
fn cs_field_grid(@builtin(global_invocation_id) gid: vec3<u32>) {
    let w = u_sim.counts.z;
    let h = u_sim.counts.w;
    if (gid.x >= w || gid.y >= h) {
        return;
    }

    if (u_sim.counts.x == 3u) {
        // Earth aurora profiles: the existing 256x256 RGBA16F field stores four
        // periodic 16384-sample azimuth tables, flattened into 64 rows each.
        // Compute these shared moving folds/filaments once, rather than evaluating
        // simplex noise at every integration point in both eyes. No history is used.
        let table_size = w * h / 4u;
        let flat = gid.y * w + gid.x;
        let profile = flat / table_size;
        let angle = (f32(flat % table_size) + 0.5) * TAU / f32(table_size);
        let azimuth = vec3<f32>(cos(angle), 0.0, sin(angle));
        let time = u_sim.params_a.y;
        var value: vec3<f32>;
        if (profile == 0u) {
            let drift = vec3<f32>(0.0, time * 0.025, 0.0);
            let fold = snoise3(azimuth * 7.0 + drift) * 0.0034
                + snoise3(azimuth * 24.0 - drift * 1.4) * 0.0011
                + snoise3(azimuth * 71.0 + drift * 0.7) * 0.00025;
            // Split the fold across two half-float channels. A single half's
            // quantization becomes visible when adjacent samples are differentiated.
            let high = floor(fold * 65536.0) / 65536.0;
            value = vec3<f32>(high, (fold - high) * 65536.0, 0.0);
        } else {
            let drift = vec3<f32>(0.0, time * 0.055 + f32(profile - 1u) * 3.7, 0.0);
            let epsilon = 0.00001;
            let before = vec3<f32>(cos(angle - epsilon), 0.0, sin(angle - epsilon));
            let after = vec3<f32>(cos(angle + epsilon), 0.0, sin(angle + epsilon));
            let derivative = (snoise3(after * 310.0 + drift) - snoise3(before * 310.0 + drift)) / (2.0 * epsilon * 310.0);
            // Store the strand derivative for Hermite reconstruction. The finest
            // 970-domain modulation remains procedural when resolved by a pixel;
            // forcing that frequency into this LUT would smooth away visible detail.
            value = vec3<f32>(snoise3(azimuth * 27.0 + drift * 0.4),
                snoise3(azimuth * 310.0 + drift), derivative);
        }
        textureStore(field_out, vec2<i32>(gid.xy), vec4<f32>(value, 13.0));
        return;
    }

    let ixy = vec2<i32>(i32(gid.x), i32(gid.y));
    let iw = i32(w);
    let ih = i32(h);

    let c  = textureLoad(field_in, ixy, 0);
    let xm = textureLoad(field_in, vec2<i32>((ixy.x + iw - 1) % iw, ixy.y), 0);
    let xp = textureLoad(field_in, vec2<i32>((ixy.x + 1) % iw, ixy.y), 0);
    let ym = textureLoad(field_in, vec2<i32>(ixy.x, (ixy.y + ih - 1) % ih), 0);
    let yp = textureLoad(field_in, vec2<i32>(ixy.x, (ixy.y + 1) % ih), 0);
    let lap = (xm + xp + ym + yp) * 0.25 - c;

    let dt = clamp(u_sim.params_a.x, 0.001, 0.05);
    let time = u_sim.params_a.y;
    let mode = u_sim.counts.x;
    let uv = (vec2<f32>(gid.xy) + vec2<f32>(0.5)) / vec2<f32>(f32(w), f32(h));
    let mode_tag = 10.0 + f32(mode);
    let is_initialized = abs(c.w - mode_tag) < 0.25;

    var next_val = c;
    if (mode == 0u) {
        // Gray-Scott Turing Reaction-Diffusion (15 Complexity) with crisp cold-start seed
        var u = clamp(c.x, 0.0, 1.0);
        var v = clamp(c.y, 0.0, 1.0);
        var lap_uv = lap.xy;
        if (!is_initialized) {
            let spot_n = fbm3(vec3<f32>(uv * 7.5, 1.37), 3);
            let ring_p = length(uv - vec2<f32>(0.5));
            let chemical_seed = smoothstep(0.05, 0.22, spot_n + 0.18 * cos(ring_p * 26.0));
            u = mix(0.96, 0.46, chemical_seed);
            v = mix(0.02, 0.36, chemical_seed);
            lap_uv = vec2<f32>(0.0, 0.08 * chemical_seed);
        }
        let uvv = u * v * v;
        let feed = 0.036 + 0.008 * sin(time * 0.25 + uv.x * TAU) + u_audio.levels_a.x * 0.006;
        let kill = 0.061 + 0.004 * cos(uv.y * TAU);
        let du = clamp(u + (0.21 * lap_uv.x - uvv + feed * (1.0 - u)) * (dt * 24.0), 0.0, 1.0);
        let dv = clamp(v + (0.105 * lap_uv.y + uvv - (feed + kill) * v) * (dt * 24.0), 0.0, 1.0);
        next_val = vec4<f32>(du, dv, clamp(abs(lap_uv.y) * 9.0 + dv * 0.65, 0.0, 1.5), mode_tag);
    } else if (mode == 1u) {
        // Chladni / Cymatic Resonance Field (02 Music)
        let p = uv * 2.0 - vec2<f32>(1.0);
        let r = length(p);
        let theta = atan2(p.y, p.x);
        let n_m = floor(3.0 + u_audio.synth_b.y * 4.0);
        let freq = max(u_sim.params_b.w, 4.5);
        let drive = sin(freq * r * PI) * cos(n_m * theta) * (0.35 + u_audio.synth_b.z * 1.6);
        let h_prev = select(drive * 0.72, c.x, is_initialized);
        let v_prev = select(0.0, c.y, is_initialized);
        let l_prev = select(0.0, lap.x, is_initialized);
        let h_vel = v_prev + (l_prev * 18.0 + (drive - h_prev) * 7.5) * dt;
        let h_next = clamp(h_prev * 0.992 + h_vel * dt, -2.0, 2.0);
        next_val = vec4<f32>(h_next, h_vel * 0.985, abs(h_next), mode_tag);
    } else {
        // Auroral Magnetospheric Curtain Field (03 Thermosphere)
        let n3 = fbm3(vec3<f32>(uv * 4.8, time * 0.16), 4);
        let curtain = exp(-pow((uv.y - 0.5 + n3 * 0.18) * 6.0, 2.0));
        let ripple = 0.5 + 0.5 * sin(uv.x * 28.0 + time * 1.9 + n3 * 5.0);
        let green_o = curtain * (0.65 + 0.75 * ripple) * (1.0 + u_audio.levels_a.z * 0.7);
        let violet_n2 = pow(curtain, 2.4) * (0.35 + 0.85 * (1.0 - ripple));
        next_val = vec4<f32>(violet_n2 * 0.65, green_o * 1.45, violet_n2 * 1.35 + green_o * 0.25, mode_tag);
    }

    textureStore(field_out, ixy, next_val);
}
