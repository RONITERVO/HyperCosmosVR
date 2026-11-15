# HyperCosmosVR — GPU Shader Contracts (`shaders/`)

> Target Backend: `wgpu 30.0.2` (Naga) -> DirectX 12 `DynamicDxc` SM 6.5+/6.8  
> Required Features: `MULTIVIEW` (`@builtin(view_index)`), `TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`, `Rgba16Float` HDR render/storage targets (`SHADER_FLOAT16` / `SUBGROUP` optional enhancements).  
> Coordinate Convention: **Camera-relative Reversed-Z Infinite Far Plane** (`near -> 1.0`, `far/inf -> 0.0`, DepthCompare `CompareFunction::GreaterEqual`). Camera sits at `(0.0, 0.0, 0.0)`; stereo HMD eyes sit at `u_frame.eye_offset[eye_idx].xyz`.

---

## 1. Shader Suite Inventory & Compilation Rule

**CRITICAL:** Pass shaders (`compute_universe.wgsl`, `far_shell.wgsl`, `instanced_sprites.wgsl`, `msdf_text.wgsl`, `post_bloom_agx.wgsl`) **require `common_oled.wgsl` prepended verbatim**.  
The modular documentary hero pipeline concatenates **6 WGSL files in strict order** prior to `device.create_shader_module`:
`common_oled.wgsl` -> `doc_common.wgsl` -> `hero_doc_cosmos.wgsl` -> `hero_doc_matter_mind.wgsl` -> `hero_doc_civilization_horizon.wgsl` -> `hero_volumetric.wgsl`.

| File | Role | Entry Points | Validation |
| :--- | :--- | :--- | :--- |
| `common_oled.wgsl` | Shared `@group(0)` uniforms, Reversed-Z, stereo ray, FBM/curl noise, blackbody, HSL, iridescence, `#000000` void clamp | *(prelude — 0 entry points)* | Naga + HLSL SM 6.5 OK |
| `doc_common.wgsl` | Shared `HeroVolumeInstance`, `@group(1)` bindings, `HeroVsOut`/`SurfaceFsOut`, bit-packed `doc_zone`/`doc_feat`/`doc_scene`/`doc_cue`/`doc_weight`/`doc_submode` decoders, SDF primitive library | *(prelude — 0 entry points)* | Naga + HLSL SM 6.5 OK |
| `hero_doc_cosmos.wgsl` | 14-zone Deep Earth surface & cutaway (`03`), 11 Solar System worlds & PBR materials (`04`, `01`), Cassini/Goldilocks rings, Kerr BH, Local Group/Laniakea/Web/CMB/Landmarks (`05..10`) | *(module helper functions)* | Naga + HLSL SM 6.5 OK |
| `hero_doc_matter_mind.wgsl` | Colors Prism & 3D HSL Bicone (`01`), Chladni Plate & 48-String Plucked Harp + 4 Timbre Chambers (`02`), Matter/QM & Spacetime (`13..14`), Complexity/Life & Sculpted 3D Anatomical Cortex Connectome (`15..17`) | *(module helper functions)* | Naga + HLSL SM 6.5 OK |
| `hero_doc_civilization_horizon.wgsl` | Evidence/Future (`11..12`), Culture/Tech/Futures (`18..20`), Cosmic Life/Contact SETI Dish & Prime Beacon/6-Lens Mandala/Meaning/25-Petal Final Horizon (`21..25`) with `doc_zone`/`doc_feat` disambiguation & fade | *(module helper functions)* | Naga + HLSL SM 6.5 OK |
| `compute_universe.wgsl` | Multi-mode compute particle advection (modes `0..16`) & 2D ping-pong PDE grids (modes `0..2`) | `cs_advect_particles` `@workgroup_size(256,1,1)`, `cs_field_grid` `@workgroup_size(8,8,1)` | Naga + HLSL SM 6.5 OK |
| `far_shell.wgsl` | Infinite boundaryless sky dome (deep starfield, Milky Way dust, CMB, Inflation, Quantum Void, Hyper-Horizon Lotus) | `vs_main`, `vs_stereo`, `fs_main` | Naga + HLSL SM 6.5 OK |
| `hero_volumetric.wgsl` | Modular entry-point dispatcher calling `doc_*` functions | `vs_main`, `vs_stereo`, `fs_hero_surface`, `fs_hero_volume` | Naga + HLSL SM 6.5 OK |
| `instanced_sprites.wgsl` | Zero-VB procedural quad sprites (`0..7`): stars, filaments, asteroids, spectrograph/DNA barcodes, diegetic wrist/dwell rings, hex Dyson mirrors/mini-moons, Minkowski/SETI cones, hairline orbits | `vs_particle_main`, `vs_particle_stereo`, `vs_sprite_main`, `vs_sprite_stereo`, `fs_main` | Naga + HLSL SM 6.5 OK |
| `msdf_text.wgsl` | Google-Earth 3D world-space MSDF glyphs (obsidian halo, `fwidth` AA, SyncVoice live-ink sweep + HDR nib) + glowing 3D leader tethers | `vs_glyph_main`, `vs_glyph_stereo`, `fs_glyph_main`, `vs_tether_main`, `vs_tether_stereo`, `fs_tether_main` | Naga + HLSL SM 6.5 OK |
| `post_bloom_agx.wgsl` | Tight-threshold (`>=1.05`) Dual-Kawase bloom, radial dispersion, Troy Sobotka AgX OLED tonemap, hard `#000000` void floor gate, TPDF 128x128 blue-noise dither | `vs_main`, `vs_stereo`, `fs_bloom_prefilter`, `fs_kawase_down`, `fs_kawase_up`, `fs_final_oled_agx` | Naga + HLSL SM 6.5 OK |

**Stereo Multiview Pattern (`vs_main` vs `vs_stereo`):**
Every render shader provides a Desktop Mono vertex entry point (`vs_*_main`, passes `eye_idx = 0u`) and an OpenXR Multiview vertex entry point (`vs_*_stereo`, uses `@builtin(view_index)` and requires `PipelineCompilationOptions` / `RenderPassDescriptor { multiview_mask: NonZeroU32::new(0b11) }` or wgpu 30 `multiview: NonZeroU32::new(2)` as configured in `wgpu 30.0.2`). Fragment entry points never touch `@builtin(view_index)` directly, so mono and stereo share one fragment entry point.

---

## 2. Shared `@group(0)` Layout (`common_oled.wgsl`)

Bound on **every** compute and render pipeline (`ShaderStages::VERTEX_FRAGMENT | ShaderStages::COMPUTE` or separate per-pipeline visibility matching `VERTEX | FRAGMENT | COMPUTE`).

| Binding | Resource Type | WGSL Variable | Size (B) | Notes |
| :---: | :--- | :--- | :---: | :--- |
| `0` | Uniform Buffer | `u_frame: FrameUniforms` | **432** | Global camera, stereo, scale, post & dwell state |
| `1` | Uniform Buffer | `u_audio: AudioReactiveUniforms` | **64** | Real-time voice RMS, FFT bands, App 02 synth bus |

### `FrameUniforms` (432 bytes, 16-byte aligned)

| Offset | Field | WGSL Type | Bytes | Semantics |
| :---: | :--- | :--- | :---: | :--- |
| `0` | `view_proj` | `array<mat4x4<f32>, 2>` | `128` | Per-eye camera-relative Reversed-Z infinite far projection (`near=1.0`, `inf=0.0`). On Desktop mono, populate `[0]`, duplicate `[1]` or leave `[1]` identity. |
| `128` | `inv_view_proj` | `array<mat4x4<f32>, 2>` | `128` | Inverse of `view_proj[eye]`. Sampled at `z=1.0` and `z=0.5` in `ndc_to_world_ray` to avoid infinite-far `z=0.0` singularity. |
| `256` | `eye_offset` | `array<vec4<f32>, 2>` | `32` | `xyz`: per-eye offset in render units relative to head center `(0,0,0)`; `w`: `1.0` when stereo active, `0.0` mono. |
| `288` | `cam_forward` | `vec4<f32>` | `16` | `xyz`: normalized forward vector; `w`: vertical FOV in radians. |
| `304` | `cam_right` | `vec4<f32>` | `16` | `xyz`: normalized right vector; `w`: aspect ratio. |
| `320` | `cam_up` | `vec4<f32>` | `16` | `xyz`: normalized up vector; `w`: near plane distance in render units. |
| `336` | `scale_params` | `vec4<f32>` | `16` | `x`: `u_cam` ($\log_{10}(\text{meters})$); `y`: `du_dt` (scale flight velocity); `z`: `exposure` (default `1.0`); `w`: `time_sec` (monotonic). |
| `352` | `post_params` | `vec4<f32>` | `16` | `x`: `bloom_thresh` (clamped `>= 1.05` in shader); `y`: `bloom_gain`; `z`: `dispersion` (edge chromatic aberration); `w`: `dither_amp` (`1.0` recommended). |
| `368` | `viewport_params` | `vec4<f32>` | `16` | `x`: pixel `width`; `y`: pixel `height`; `z`: `1.0 / width`; `w`: `1.0 / height`. |
| `384` | `stereo_params` | `vec4<f32>` | `16` | `x`: `eye_count` (`1.0` or `2.0`); `y`: `fovea_rad` (default `~0.28..0.38` rad); `z`: `frame_idx_mod` (`(frame % 128) as f32`); `w`: `active_region_phase` `[0..1]`. |
| `400` | `focus_ray` | `vec4<f32>` | `16` | `xyz`: normalized gaze/controller/reticle focus ray; `w`: motion-stop rest dwell progress `[0..1]`. |
| `416` | `region_accent` | `vec4<f32>` | `16` | `rgb`: linear HDR accent RGB of active cosmic zone; `w`: `experience_id` (`1.0..25.0`). |

### `AudioReactiveUniforms` (64 bytes, 16-byte aligned)

| Offset | Field | WGSL Type | Bytes | Semantics |
| :---: | :--- | :--- | :---: | :--- |
| `0` | `levels_a` | `vec4<f32>` | `16` | `x`: `voice_rms`; `y`: `ambient_rms`; `z`: `bass_energy`; `w`: `mid_energy`. |
| `16` | `levels_b` | `vec4<f32>` | `16` | `x`: `harmonic_centroid` `[0..1]`; `y`: `beat_phase` `[0..1]`; `z`: `high_energy`; `w`: `playback_rate` `[1.0..3.0]`. |
| `32` | `synth_a` | `vec4<f32>` | `16` | `x`: `drone_base_hz`; `y`: `synth_filter_hz`; `z`: `synth_pulse`; `w`: `drone_noise`. |
| `48` | `synth_b` | `vec4<f32>` | `16` | `x`: `active_note_hue` `[0..1]`; `y`: `active_timbre_id` (`0..3`: `cristal`/`aire`/`cálido`/`profundo`); `z`: `music_energy`; `w`: `resonance_glow`. |

---

## 3. Pass Shader Contracts (`@group(1)` & `@group(2)`)

### 3.1 `compute_universe.wgsl`

Two compute entry points share `@group(1)` storage buffers; `cs_field_grid` additionally uses `@group(2)` textures.

- **Bindings:**
  - `@group(1) @binding(0)`: `u_sim: SimDispatchUniforms` (uniform, **80 B**)
  - `@group(1) @binding(1)`: `particles: array<SimParticle>` (`storage, read_write`, **64 B stride**)
  - `@group(2) @binding(0)` *(only `cs_field_grid`)*: `field_in: texture_2d<f32>` (`filterable: false`, `textureLoad`)
  - `@group(2) @binding(1)` *(only `cs_field_grid`)*: `field_out: texture_storage_2d<rgba16float, write>`

- **`SimDispatchUniforms` (80 B):**
  - `0..16`: `params_a: vec4<f32>` (`x=dt_sec`, `y=time_sec`, `z=damping [0..1]`, `w=audio_reactivity_gain`)
  - `16..32`: `params_b: vec4<f32>` (`x=scale_radius`, `y=spiral_tightness`, `z=field_strength`, `w=wave_frequency`)
  - `32..48`: `attractor_a: vec4<f32>` (`xyz=sector-local primary attractor`, `w=strength`)
  - `48..64`: `attractor_b: vec4<f32>` (`xyz=secondary attractor`, `w=strength/charge`)
  - `64..80`: `counts: vec4<u32>` (`x=sim_mode`, `y=particle_count`, `z=grid_width`, `w=grid_height`)

- **`SimParticle` (64 B stride — shared verbatim with `instanced_sprites.wgsl`):**
  - `0..16`: `pos_age: vec4<f32>` (`xyz=sector-local position`, `w=phase/age [0..1)`)
  - `16..32`: `vel_mass: vec4<f32>` (`xyz=velocity`, `w=temperature/energy`)
  - `32..48`: `color_size: vec4<f32>` (`rgb=linear HDR tint`, `a=base sprite radius`)
  - `48..64`: `tags: vec4<u32>` (`x=sector_kind`, `y=sub_id` e.g. `(l<<8)|n` orbital or QCD charge `%3`, `z=seed_hash`, `w=flags`)

- **Modes (`counts.x`):**
  - `cs_advect_particles` (`@workgroup_size(256,1,1)`, dispatch `ceil(particle_count / 256)`):
    - `0`: Milky Way (`05`) spiral density wave + vertical disk harmonic
    - `1`: Laniakea (`07`) watershed streamlines into Great Attractor & Shapley
    - `2`: Cosmic Web (`08`) Zel'dovich ridge gradient filaments + dark voids
    - `3`: QCD quark-gluon color flux confinement (`13`)
    - `4`: Electron spherical-harmonic orbitals (`13`) (`n = tags.y & 0xFF`, `l = (tags.y >> 8) & 0xFF`)
    - `5`: Planck spin-foam & quantum vacuum fluctuations (`14`/`13`)
    - `6`: Cellular Turing/BZ shell waves (`15 Z09`, `16 Z00..=Z14`)
    - `7`: Living Neural Connectome firing cascades (`16 Z15..=Z18` / `17 Mind`)
    - `8`: App `02` Cymatic standing-wave ripples driven by `u_audio.synth_b`
    - `9`: App `01` chromatic dust & dwell focus-attracted motes
    - `10`: Solar System (`04`) Keplerian asteroid & icy Kuiper belt ring
    - `11`: Geodynamo mantle circulation & atmospheric trade winds (`03`, `18`)
    - `12`: Prebiotic Rayleigh-Benard convection micro-swarm (`15`, `13`)
    - `13`: Cosmological redshift expansion stream & photon flux (`11`, `12`, `21`, `22`)
    - `14`: Transformer token & optical bus orthogonal lattice (`18..20`)
    - `15`: 25-petal Hyper-Lotus equilibrium convergence (`23..25`)
    - `16`: Relativistic geodesic lensing and quadrupolar strain flow (`14 Z06..=Z09`)
  - `cs_field_grid` (`@workgroup_size(8,8,1)`, dispatch `ceil(grid_w/8), ceil(grid_h/8)`):
    - `0`: Gray-Scott Turing reaction-diffusion (`15 Complexity`, `16 Z09` multicellularity / `Z10` development)
    - `1`: Chladni cymatic resonance PDE (`02 Music`)
    - `2+`: Auroral magnetospheric curtain field (`03 Thermosphere`)

---

### 3.2 `far_shell.wgsl`

- **Pipeline State:** Target `Rgba16Float`, **0 vertex buffers** (3-vertex fullscreen triangle, `draw(0..3, 0..1)`), `CullMode::None`, Depth `CompareFunction::GreaterEqual`, `depth_write_enabled = false`, `BlendState::REPLACE`.
- **Entry Points:** `vs_main`, `vs_stereo`, `fs_main`.
- **Bindings:** `@group(1) @binding(0)`: `u_far: FarShellUniforms` (**64 B**):
  - `0..16`: `regime_weights: vec4<f32>` (`x=galaxy_plane_alpha`, `y=cmb_alpha`, `z=inflation_alpha`, `w=lotus_alpha`)
  - `16..32`: `galaxy_north: vec4<f32>` (`xyz=normalized galactic north pole`, `w=star_density_scale`)
  - `32..48`: `solar_dir: vec4<f32>` (`xyz=normalized sun/core dir`, `w=sun_sky_glow`)
  - `48..64`: `extra_weights: vec4<f32>` (`x=quantum_void_alpha (u<-22)`, `y=cosmic_web_sky_alpha`, `z=seed_offset`, `w=reserved`)
- **OLED Guarantee:** Empty intergalactic pixels return literal `vec4(0.0, 0.0, 0.0, 1.0)`.

---

### 3.3 `hero_volumetric.wgsl`

- **Vertex Buffer Layout:** 1 vertex buffer, `VertexStepMode::Vertex`, `array_stride = 12`:
  - `@location(0) local_pos: vec3<f32>` (`Float32x3`) — unit cube `[-1, 1]^3`, **must use `CullMode::Front`** so camera inside/outside bounding box always renders back faces cleanly without double-counting rays!
- **Entry Points:**
  - Surface pass: `vs_main` / `vs_stereo` + `fs_hero_surface` — `depth_write_enabled = true`, `CompareFunction::GreaterEqual`, `BlendState::REPLACE` (writes `@builtin(frag_depth)` Reversed-Z exact sphere/cutaway/event-horizon depth).
  - Volume pass: `vs_main` / `vs_stereo` + `fs_hero_volume` — `depth_write_enabled = false`, `CompareFunction::GreaterEqual`, `BlendState::PREMULTIPLIED_ALPHA_BLENDING` (outputs premultiplied `(accum_rgb, accum_a)`!).
- **Bindings:**
  - `@group(1) @binding(0)`: `hero_instances: array<HeroVolumeInstance>` (`storage, read`, **112 B stride**)
  - `@group(1) @binding(1)`: `blue_noise_tex: texture_2d<f32>` (`assets/textures/blue_noise_128.png`)
  - `@group(1) @binding(2)`: `blue_noise_samp: sampler` (`AddressMode::Repeat`)
- **`HeroVolumeInstance` (112 B = 7 x `vec4`):**
  - `0..16`: `pos_radius: vec4<f32>` (`xyz=camera-relative center`, `w=bounding box half-extent`)
  - `16..32`: `rot_quat: vec4<f32>` (`xyzw` unit quaternion, local -> world)
  - `32..48`: `color_a: vec4<f32>` (`rgb=primary HDR base/emission`, `a=intensity scale`)
  - `48..64`: `color_b: vec4<f32>` (`rgb=secondary atmosphere/aurora/accent`, `a=optical density`)
  - `64..80`: `phys_a: vec4<f32>` (`x=spin/phase/freq`, `y=obliquity/gap`, `z=sub_param` e.g. Kerr/timbre/energy, `w=dwell_focus [0..1]`)
  - `80..96`: `phys_b: vec4<f32>` (`xyz=normalized Sun dir OR App 01 [H_01, S, L] / App 02 [pitch_01, timbre, amp]`, `w=lod_quality_scale [0.25..1.0]`)
  - `96..112`: `kind_tags: vec4<u32>` (`x=hero_kind`, `y=variant_id`, `z=zone_u16 | (feat_u8<<16) | ((scene_idx+1)_u8<<24)`, `w=flags_u8 | (cue_u8<<8) | (weight_u8<<16) | (submode_u8<<24)`)
- **Hero Kinds (`kind_tags.x`):**
  - `fs_hero_surface`: `0=Earth (14-zone Deep Earth cutaway on bit0)`, `1=Solar planets (`1` Sun, `2` Saturn, `3..11` Mercury..Exoplanet, `12` submode `1..3` PBR Gold/Iridescent/Clay or `0` Cooling Dwarf)`, `4=Kerr Event Horizon Black Core`.
  - `fs_hero_volume`: `0=Earth Aurora Shell`, `2=Cassini/Uranus/Goldilocks Rings`, `3=Solar Corona / Local Group / Laniakea / Web / Observable / Early Universe / Landmarks (`20..22`)`, `4=Sgr A* MW / Hawking BH / Einstein Cross`, `5=App 01 Prism & 3D HSL Bicone`, `6=App 02 Chladni + 48-String Harp & Timbre Chambers (`11..14`)`, `7=Matter/QM (`10..14`) & Spacetime (`20..23`)`, `8=Complexity (`10..13`), Life (`20..24`), Sculpted 3D Anatomical Cortex Connectome (`30..34`)`, `9+=Evidence, Future, Culture, Tech, Futures, Cosmic Life, SETI Contact, 6-Lens Perspectives, Meaning & 25-Petal Final Horizon (`11,12,18..25`)`.

---

### 3.4 `instanced_sprites.wgsl`

- **Vertex Buffer Layout:** **0 vertex buffers** (`draw(0..6, 0..count)` procedural billboard expansion). Depth `CompareFunction::GreaterEqual`, `depth_write_enabled = false`, `BlendState::ALPHA_BLENDING` (or additive `One, One` for pure star/filament batches; emits straight `(rgb, alpha)` clamped via `oled_void_clamp`).
- **Entry Points:**
  - `vs_particle_main` / `vs_particle_stereo` + `fs_main` (reads `sim_particles` at `@binding(1)`)
  - `vs_sprite_main` / `vs_sprite_stereo` + `fs_main` (reads `sprite_instances` at `@binding(2)`)
- **Bindings:**
  - `@group(1) @binding(0)`: `u_batch: SpriteBatchUniforms` (uniform, **32 B**)
  - `@group(1) @binding(1)`: `sim_particles: array<SimParticle>` (`storage, read`, **64 B stride**)
  - `@group(1) @binding(2)`: `sprite_instances: array<SpriteInstance>` (`storage, read`, **64 B stride**)
  *(Bind a dummy 64-byte minimum buffer on whichever storage binding is inactive for the draw call, or populate both.)*
- **`SpriteBatchUniforms` (32 B):**
  - `0..16`: `sector_origin_scale: vec4<f32>` (`xyz=camera-relative sector origin`, `w=sector_scale_multiplier`)
  - `16..32`: `batch_params: vec4<f32>` (`x=default_sprite_kind` for particles, `y=min_pixel_size` angular anti-aliasing clamp, `z=global_alpha`, `w=time_sec`)
- **`SpriteInstance` (64 B stride):**
  - `0..16`: `pos_size: vec4<f32>` (`xyz=camera-relative pos`, `w=half_size_radius`)
  - `16..32`: `axis_aspect: vec4<f32>` (`xyz=elongation axis`, `w=aspect >= 1.0`)
  - `32..48`: `color_alpha: vec4<f32>` (`rgb=linear HDR`, `a=alpha`)
  - `48..64`: `tags_f: vec4<f32>` (`x=sprite_kind`, `y=phase`, `z=sub_param`, `w=flags`)
- **Sprite Kinds (`0..7`):** `0=Blackbody Spectral Star + diffraction spikes`, `1=Flow/Filament Ribbon`, `2=Procedural Asteroid/Ice Shard`, `3=Spectral Pinpoint Beacon Gem (`sub_param<=0.5`) or Spectrograph/DNA/Prime Barcode (`sub_param>0.5`)`, `4=Diegetic Wrist Ring Bead / Crystalline Dwell Ring`, `5=Starburst Spark Mote (`<=0.25`) / Analytic Shaded Moon/Planet (`0.25..0.75`) / Faceted Hexagonal Dyson Collector Glint (`>0.75`)`, `6=Directional Minkowski Light Cone / SETI Wave Wedge`, `7=Razor Hairline Orbital Track / Radar Ping Tick`.

---

### 3.5 `msdf_text.wgsl` (`cosmos_typo`)

Desktop guidance uses trailing glyph/tether ranges selected by `MsdfTextRenderer::set_foreground_start` after upload. These ranges render last with `CompareFunction::Always` and leave depth writes disabled. VR holograms, normal labels, and narration retain `GreaterEqual`. `GlyphInkMode::ControlBackdrop` (`ink_params.w = 5`) draws a softly edged dark quad without atlas sampling **for desktop controls only**; VR emits no backdrop instances or foreground ranges. Buffer strides and bindings are unchanged. The mono/multiview GPU test checks normal visibility, complete near-depth occlusion, and foreground visibility against that same occluder.

- **Vertex Buffer Layout:** **0 vertex buffers** (`draw(0..6, 0..instance_count)`).
- **Pipeline State:** Target `Rgba16Float`, Depth `CompareFunction::GreaterEqual`, `depth_write_enabled = false`, `CullMode::None`, `BlendState::ALPHA_BLENDING` (`SrcAlpha` / `OneMinusSrcAlpha`). Output `rgb` is already divided back out of premultiplied fill+obsidian-halo composition so borders have zero dark seams.
- **Entry Points:**
  - Glyphs: `vs_glyph_main` / `vs_glyph_stereo` + `fs_glyph_main`
  - Leader Filaments: `vs_tether_main` / `vs_tether_stereo` + `fs_tether_main`
- **Bindings (`@group(1)`):**
  - `@binding(0)`: `u_text_pass: TextPassUniforms` (uniform, **32 B**)
  - `@binding(1)`: `glyphs: array<GlyphInstance>` (`storage, read`, **112 B stride**)
  - `@binding(2)`: `tethers: array<TetherInstance>` (`storage, read`, **48 B stride**)
  - `@binding(3)`: `msdf_atlas: texture_2d<f32>` (`Rgba8Unorm`, linear non-sRGB! `fdsm` 3-channel MSDF in `.rgb`)
  - `@binding(4)`: `msdf_samp: sampler` (`FilterMode::Linear`, `AddressMode::ClampToEdge`)
- **CRITICAL UV CONVENTION (`fdsm` vs GPU):**
  - `fdsm 0.8.0` generates glyph outlines with Y-up; GPU textures sample `v=0` at top and `v=1` at bottom.
  - `msdf_text.wgsl` maps the **top** of the 3D quad (`+up_vec`) to `uv_rect.y` (`v_top`) and the **bottom** (`-up_vec`) to `uv_rect.w` (`v_bottom`). `cosmos_typo` should supply standard GPU texture coordinates `(u_left, v_top, u_right, v_bottom)` after flipping Y during atlas upload/UV computation (or swap `y`/`w` if keeping bottom-up UVs).
- **`TextPassUniforms` (32 B):**
  - `0..16`: `params_a: vec4<f32>` (`x=global_alpha [0..1]`, `y=edge_softness_scale` default `1.0`, `z=nib_glow_gain` default `1.35`, `w=depth_bias_scale` default `1.0`)
  - `16..32`: `nib_color: vec4<f32>` (`rgb=HDR live-ink nib accent` (or `0,0,0` to auto-use `u_frame.region_accent`), `w=halo_scale` default `1.0`)
- **`GlyphInstance` (112 B stride = 7 x `vec4`):**
  - `0..16`: `pos_center: vec4<f32>` (`xyz=camera-relative center`, `w=anti_zfight_pull_meters`)
  - `16..32`: `right_vec: vec4<f32>` (`xyz=world half-right`, `w=atlas_px_range` e.g. `6.0`/`8.0`)
  - `32..48`: `up_vec: vec4<f32>` (`xyz=world half-up`, `w=halo_sdf_width` e.g. `0.22`)
  - `48..64`: `uv_rect: vec4<f32>` (`u_left, v_top, u_right, v_bottom`)
  - `64..80`: `color_hdr: vec4<f32>` (`rgb=linear HDR fill`, `a=opacity`)
  - `80..96`: `halo_color: vec4<f32>` (`rgb=obsidian outline e.g. 0.002,0.003,0.006`, `a=halo opacity`)
  - `96..112`: `ink_params: vec4<f32>` (`x=glyph_t0`, `y=glyph_t1`, `z=reveal_cursor` [`>=1.5`: static, `<-0.5`: hidden, `0..1.2`: SyncVoice sweep], `w=mode`: `0=spatial anchor`, `1=live-ink active phrase`, `2=Spanish bilingual ghost ribbon`, `3=remembered/explored muted ink`, `4=dwell-focused heading`, `5=desktop control backdrop`)
- **`TetherInstance` (48 B stride = 3 x `vec4`):**
  - `0..16`: `p0_width: vec4<f32>` (`xyz=start world pos`, `w=half_width_m`)
  - `16..32`: `p1_alpha: vec4<f32>` (`xyz=end world pos`, `w=opacity [0..1]`)
  - `32..48`: `color_hdr: vec4<f32>` (`rgb=linear HDR filament color`, `w=end_taper [0..1]`)

---

### 3.6 `post_bloom_agx.wgsl`

- **Vertex Buffer Layout:** **0 vertex buffers** (`draw(0..3, 0..1)`), `CullMode::None`, Depth `None`, `BlendState::REPLACE`.
- **Entry Points:** `vs_main` (layer from `u_post.texel_scale.w`), `vs_stereo` (layer from `@builtin(view_index)`), paired with:
  1. `fs_bloom_prefilter` (`hdr_scene_tex` -> bloom mip 0; Karis firefly weight + tight threshold `>= 1.05`)
  2. `fs_kawase_down` (`bloom_tex` mip `k` -> mip `k+1`)
  3. `fs_kawase_up` (`bloom_tex` mip `k+1` -> mip `k`)
  4. `fs_final_oled_agx` (scene + bloom + edge chromatic dispersion + void floor lock + AgX OLED tonemap + TPDF blue-noise dither -> swapchain)
- **Bindings (`@group(1)`):**
  - `@binding(0)`: `u_post: PostPassUniforms` (uniform, **32 B**)
  - `@binding(1)`: `hdr_scene_tex: texture_2d_array<f32>` (`TextureViewDimension::D2Array` even for 1-layer Desktop mono!)
  - `@binding(2)`: `bloom_tex: texture_2d_array<f32>` (`TextureViewDimension::D2Array`)
  - `@binding(3)`: `blue_noise_tex: texture_2d<f32>` (`128x128` tile, read via `textureLoad`)
  - `@binding(4)`: `linear_samp: sampler` (`FilterMode::Linear`, `AddressMode::ClampToEdge`)
  - `@binding(5)`: `point_samp: sampler` *(reserved/non-filtered utility slot matching group layout)*
- **`PostPassUniforms` (32 B):**
  - `0..16`: `texel_scale: vec4<f32>` (`x=1.0/src_w`, `y=1.0/src_h`, `z=kawase_offset_scale`, `w=target_layer_f32` `0.0` or `1.0`)
  - `16..32`: `grade_params: vec4<f32>` (`x=output_encode_mode`: `0=linear for hardware sRGB swapchain`, `1=manual sRGB encode for UNORM swapchain`, `2=direct linear HDR 10/16-bit`; `y=vignette_strength` `0.0` default; `z=bloom_soft_knee` `0.25`; `w=void_floor_eps` `1e-5`)

---

## 4. Verification Summary

Validated all 7 `.wgsl` files simultaneously with Naga `30.0.2` (`Capabilities::MULTIVIEW | FLOAT64 | SHADER_FLOAT16 | SUBGROUP | STORAGE_TEXTURE_16BIT_NORM_FORMATS`) and compiled to DX12 HLSL `ShaderModel::V6_5`:
- `common_oled.wgsl`: `0` entry points, `11373` HLSL bytes — **PASS**
- `compute_universe.wgsl`: `2` entry points, `38158` HLSL bytes — **PASS**
- `far_shell.wgsl`: `3` entry points, `18335` HLSL bytes — **PASS**
- `hero_volumetric.wgsl`: `4` entry points, `50574` HLSL bytes — **PASS**
- `instanced_sprites.wgsl`: `5` entry points, `24977` HLSL bytes — **PASS**
- `msdf_text.wgsl`: `6` entry points, `26565` HLSL bytes — **PASS**
- `post_bloom_agx.wgsl`: `6` entry points, `26690` HLSL bytes — **PASS**

