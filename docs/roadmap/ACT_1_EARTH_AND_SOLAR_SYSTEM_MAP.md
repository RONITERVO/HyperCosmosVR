# Act I Gap Map — Earth & Solar System (`03_deep_earth`, `04_solar_system`)

> **Navigation:** [Master Visual Roadmap](../CONTRIBUTOR_VISUAL_ROADMAP.md) · [Contributing Guide](../../CONTRIBUTING.md) · [Shader Contracts](../../shaders/SHADER_CONTRACTS.md) · [Act II: Cosmic Web & Origins](./ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md) · [Act III: Matter & Spacetime](./ACT_3_MATTER_AND_SPACETIME_MAP.md) · [Act IV: Life, Mind & Senses](./ACT_4_LIFE_MIND_AND_SENSES_MAP.md) · [Act V: Civilization & Horizons](./ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md)

## 1. Act I Scope, Odyssey Placement & Ground-Truth Scorecard

Odyssey Act I corresponds to **`ODYSSEY_ACTS[0] = &[3, 4]`** in [`crates/cosmos_journey/src/odyssey.rs`](../../crates/cosmos_journey/src/odyssey.rs#L24-L45), guiding the viewer from Earth's outer magnetosphere down into Earth's solid inner core (`Exp 03`) and outward from the Sun across the eight planets, Asteroid Belt, Kuiper Belt, and Pluto-Charon binary (`Exp 04`). Across the repository's **`88` canonical Pick-Up Packages** (`3` P0 core plumbing + `85` content packages), Act I comprises **`6` claimable Pick-Up Packages** (`3–5` zones each).

| Exp | Catalog Slug | Odyssey Position & Scale Band | Zones | Features | Zone `[A / B / C / D]` | Feature `[A / B / C / D]` | Reserved `vid` Block | Companion `vid` Block |
|---:|---|---|---:|---:|:---:|:---:|---|---|
| `03` | `03_deep_earth` (`Espectro de la Tierra`) | Act 1 · Ch `1/2` (`SectorId::EARTH_ORBIT`, `u in [0.0, 7.4]`, `focus=6.8`, `COLD_START=6.6`) | `14` | `70` | `4 / 9 / 0 / 1` | `20 / 15 / 0 / 35` | `1300..=1313` | `3620..=3629` |
| `04` | `04_solar_system` (`Espectro del Sistema Solar · Del Sol a Plutón`) | Act 1 · Ch `2/2` (`SectorId::SOLAR_SYSTEM`, `u in [6.8, 13.6]`, `focus=11.2`, `11 body_layout`) | `12` | `60` | `0 / 12 / 0 / 0` | `0 / 26 / 1 / 33` | `1400..=1411` | `3640..=3651` |
| **ACT I** | **`03 + 04`** | **Odyssey Act I (`2` Experiences)** | **`26`** | **`130`** | **`4 / 21 / 0 / 1`** | **`20 / 41 / 1 / 68`** | **`1300..=1411`** | **`3620..=3651`** |

> **Critical Audit & Reference Exemplar Progress:** Ground-truth inspection of [`universe_catalog.v1.json`](../../assets/baked/universe_catalog.v1.json), [`doc_stages.rs`](../../crates/cosmos_render/src/doc_stages.rs), [`hero_doc_cosmos.wgsl`](../../shaders/hero_doc_cosmos.wgsl), [`hero_volumetric.wgsl`](../../shaders/hero_volumetric.wgsl), and [`compute_universe.wgsl`](../../shaders/compute_universe.wgsl) shows **`4 / 26` Act I zones (`Z05..Z08` via `PKG-ACT1-EXP03-Z05-08`)** and **`20 / 130` features** are now **`[TIER-A]` reference implementations** (`vid = 1305..=1308`, full `doc_feat(inst)` sub-staging, premature `Z06 Surface` cutaway bug fixed, Moho discontinuity split). Remaining Act I functional gaps today:
> 1. **Missing `Z03` Mesosphere Shader (`[TIER-D]`):** Zero branch exists for `z_idx == 3u` in [`hero_doc_cosmos.wgsl`](../../shaders/hero_doc_cosmos.wgsl) despite comments promising meteors and ISS orbit.
> 2. **Fixed in `PKG-ACT1-EXP03-Z05-08`:** Premature Earth cutaway on `Z06 Surface` (`rel_scale_u = 6.05`) and `Z07` crust / `Z08` lithosphere Moho conflation are now resolved in [`doc_stages.rs`](../../crates/cosmos_render/src/doc_stages.rs) and [`hero_doc_cosmos.wgsl`](../../shaders/hero_doc_cosmos.wgsl).
> 3. **Venus Prograde Spin Contradiction (`doc_stages.rs:338`):** Positive spin `+input.time_sec * 0.03` & `hero_doc_cosmos.wgsl:131` `+time * 0.08` rotate Venus prograde directly contradicting retrograde narration (`Z02 F2`).
> 4. **Exp 04 Earth Isolation & Belt Placeholder Leaks:** Moon + magnetosphere are gated to `Exp 03` only (`doc_stages.rs:588-609`), `Exp 03` `z_idx == 4/5/6/7` checks leak onto `Exp 04` planets (`hero_doc_cosmos.wgsl:25,33,43`), `Z05 Asteroid Belt` reuses Mercury (`vid=3`) + Saturn Cassini gaps (`F4 [TIER-C]`), `Z06 Jupiter` never spawns existing `vid=10` Europa, and `Z10 Kuiper Belt` misuses cracked-ocean Europa (`vid=10`).

## 2. Core Prerequisites & Act I Package Index (`6` Pick-Up Cards)

Before implementing procedural feature branches (`vid = 1300..1313` / `1400..1411`), check or land the Core P0 prerequisites documented in [the Master Visual Roadmap](../CONTRIBUTOR_VISUAL_ROADMAP.md):
- [`PKG-P0-DISPATCH-AND-IDS`](../CONTRIBUTOR_VISUAL_ROADMAP.md#pkg-p0-dispatch-and-ids) — `vid >= 1000u` shader dispatcher + `doc_stages.rs` variant mapping
- [`PKG-P0-FEATURE-AND-CUE-PLUMBING`](../CONTRIBUTOR_VISUAL_ROADMAP.md#pkg-p0-feature-and-cue-plumbing) — propagate `SegmentVisualHint::cue_tags` -> `base_flags` (`engine.rs:1691-1699` -> `universe_scene.rs:17-25` -> WGSL `doc_feat`/`doc_cue`/`doc_weight`)
- [`PKG-P0-CATALOG-SCENE-AND-CI`](../CONTRIBUTOR_VISUAL_ROADMAP.md#pkg-p0-catalog-scene-and-ci) — `scene` backfill & verification harness coverage

| Package ID (Jump Link) | Phase & Priority | Target Zones | Zone Tiers (`A/B/C/D`) | Feat Tiers (`A/B/C/D`) | Headline Bugs Fixed & Deliverables |
|---|---|---|---:|---:|---|
| [`PKG-ACT1-EXP03-Z00-04`](#pkg-act1-exp03-z00-04) | Phase 3 (High) | `Exp 03 Z00..Z04` (`5` zones, `25` feats) | `0/4/0/1` | `0/3/0/22` | Expand `r_vol` (`1.14 -> 2.1*R`), bow-shock dayside vs Lyman-alpha geocorona split, ISS & GEO/HEO rings, **build missing `Z03` Mesosphere meteors/NLCs/rocket**, ozone UV attenuation |
| [`PKG-ACT1-EXP03-Z05-08`](#pkg-act1-exp03-z05-08) | Phase 1/3 (`[TIER-A]` Reference Exemplar Done) | `Exp 03 Z05..Z08` (`4` zones, `20` feats) | `4/0/0/0` | `20/0/0/0` | **Completed Reference Implementation (`1305..=1308`):** Fixed `Z06 Surface` premature cutaway, split Moho at `0.938..0.962` (`Z07` crust vs `Z08` rigid mantle + Wadati-Benioff foci), wired 20 `doc_feat` 1:1 transitions |
| [`PKG-ACT1-EXP03-Z09-13`](#pkg-act1-exp03-z09-13) | Phase 3 (Medium) | `Exp 03 Z09..Z13` (`5` zones, `25` feats) | `0/5/0/0` | `0/12/0/13` | Add partial-melt pockets (`Z09`), `seam_410` + ringwoodite hydration (`Z10`), bridgmanite + `D''` LLSVPs (`Z11`), Taylor columns (`Z12`), anisotropic HCP crystals & 360 GPa reticle (`Z13`) |
| [`PKG-ACT1-EXP04-Z00-03`](#pkg-act1-exp04-z00-03) | Phase 3 (Critical) | `Exp 04 Z00..Z03` (`4` zones, `20` feats) | `0/4/0/0` | `0/10/0/10` | Sun H->He cutaway & Parker solar wind, Mercury scarps/polar ice, **negate Venus prograde spin bug (`-time`) + IR/radar cutaway**, restore Exp-04 Earth Moon/magnetosphere & isolate Exp-03 `z_idx` |
| [`PKG-ACT1-EXP04-Z04-07`](#pkg-act1-exp04-z04-07) | Phase 3 (High) | `Exp 04 Z04..Z07` (`4` zones, `20` feats) | `0/4/0/0` | `0/9/1/10` | Mars Olympus Mons/rover/Phobos-Deimos, **replace Saturn Cassini & Mercury reuse on Asteroid Belt w/ Vesta/Fe-Ni/Kirkwood gaps**, spawn Galilean moons + `vid=10` Europa (`Z06`), Titan & Enceladus (`Z07`) |
| [`PKG-ACT1-EXP04-Z08-11`](#pkg-act1-exp04-z08-11) | Phase 3 (High) | `Exp 04 Z08..Z11` (`4` zones, `20` feats) | `0/4/0/0` | `0/7/0/13` | Uranus dark epsilon rings + Miranda (`Z08`), Neptune `>2000 km/h` jets + retrograde Triton/N2 geysers (`Z09`), **replace cracked Europa on Kuiper Belt** w/ Haumea/Makemake/3:2 lobes/comet (`Z10`), Pluto blue haze + Charon binary + 4 moonlets (`Z11`) |

---

## 3. Claimable Work Packages (`6` Pick-Up Cards)

<a id="pkg-act1-exp03-z00-04"></a>
### PKG-ACT1-EXP03-Z00-04: Exp 03 Deep Earth — Outer Space, Exosphere, Thermosphere, Mesosphere & Stratosphere (`Z00..Z04`)
- **Priority & Rollout:** Phase 3 (`Z03 Mesosphere [TIER-D]` High-Priority) · Depends on `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Scope:** Exp `03` (`03`) Zones `Z00..Z04` (`5` zones, `25` features)
- **Current Card Tiers:** Zones `0A / 4B / 0C / 1D` · Features `0A / 3B / 0C / 22D`

**Current State:**
- **Primary Dispatch:** [`doc_stages.rs:567-611`](../../crates/cosmos_render/src/doc_stages.rs#L567-L611) emits `kind=0, vid=0` surface globe + shell volume (`r_vol = earth_solid_r * 1.14` at `L591`, which clips the outer magnetosphere and high exosphere) + 1 Moon billboard sprite (`tags_f=[5.0,0.25,0.48,0.0]` at `L600-610`).
- **Compute / PDE Wiring:** [`universe_scene.rs:220,305`](../../crates/cosmos_render/src/universe_scene.rs#L220-L305) dispatches `cs_field_grid` Mode 2 auroral PDE curtain ([`compute_universe.wgsl:428-436`](../../shaders/compute_universe.wgsl#L428-L436)) and Compute Mode 11 toroidal plasma/trade-wind particles (`L297-309`).
- **WGSL Branches (`Z00..Z04`):** [`hero_doc_cosmos.wgsl:256-308`](../../shaders/hero_doc_cosmos.wgsl#L256-L308) merges `z_idx == 0u` and `z_idx == 1u` onto identical dipole lines (`mag_gain=0.92`, `L262`), highlights polar aurora (`aur_gain=1.65`) on `z_idx == 2u` (`L274-293`), leaves **`z_idx == 3u` Mesosphere completely unimplemented (`[TIER-D]`)** despite `L256` comment (`Mesosphere Meteors`), leaves `ISS Orbit` (`L295`) and `Exo-Satellites` (`L256`) unimplemented, and boosts indigo `ozone_glow` on `z_idx == 4u` (`L43`). `doc_feat(inst)` is unread.
- **Card Tier Breakdown (`5` zones, `25` features):** Zones `0A / 4B / 0C / 1D` · Features `0A / 3B / 0C / 22D`.

**Reserved Target IDs:**

| Zone | Slug | Canonical `vid` | Companion `vid` (`3600..3869`) | `submode` (`bits 24..31`) | `base_flags` (`bits 0..7`) | `doc_feat` Slots (`0..7`, `0xFF=macro`) |
|---|---|---:|---|---|---|---:|
| `Z00` | `space` | `1300` | `3620` (Moon), `3621` (`submode=32` Bow-Shock Magnetosphere), `3622` (`submode=32` GEO/GNSS) | `0` (shell `2.1*R`) | `0x02` + cue mask | `0..4` |
| `Z01` | `exosphere` | `1301` | `3621` (`submode=33` Geocorona Envelope), `3622` (`submode=33` HEO/Molniya Tracks) | `0` (exobase `1.85*R`) | cue mask | `0..4` |
| `Z02` | `thermosphere` | `1302` | `3622` (`submode=34` ISS Truss + 8 Solar Wings), `cs_field_grid` Mode 2 | `8` (PDE aurora) | cue mask | `0..4` |
| `Z03` | `mesosphere` | `1303` | `3623` (Suborbital Sounding Rocket Arc) | `0` (`50-80 km` shell) | cue mask | `0..4` |
| `Z04` | `stratosphere` | `1304` | `3624` (Stratospheric Balloon & Supersonic Jet/Contrail) | `0` (`12-50 km` shell) | cue mask | `0..4` |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `[TIER-B]` Exp `03` `Z00` `space` — *Espacio* / *Space* (`vid = 1300` | Feats: `0A / 1B / 0C / 4D`)
- **Metadata:** `raw_metric`: "vacío casi total" *(EN: "near-total vacuum")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[10000.0, 100000.0]` km
- **Spoken Macro (ES // EN):** La atmósfera ya se ha fundido gradualmente con el entorno interplanetario. // *The atmosphere has already gradually merged with the interplanetary environment.*
- **Rendered Today:** Full Earth globe (`shade_doc_earth_surface`) + `show_magnetosphere` dipole Van Allen L-shells (`L=1.18, 1.48`, `mag_gain=0.92`, `hero_doc_cosmos.wgsl:262-306`) + Moon impostor sprite (`doc_stages.rs:600-610`) + Compute Mode 11 toroidal particles.
- **Audit Gap:** Zero solar-wind particle stream, no dayside bow-shock compression vs magnetotail asymmetry, no relativistic cosmic rays, no high-orbit GEO/GNSS satellites (despite comment at `hero_doc_cosmos.wgsl:256`). Note `r_vol = earth_solid_r * 1.14` (`doc_stages.rs:591`) clips outer magnetosphere tightly around the globe. `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1300`):** Expand Earth shell volume box to `earth_solid_r * 2.1`; compute solar-wind directional ram-pressure compression (`dot(p, sun_local)`) squashing dayside magnetopause (`L <= 1.35`) and stretching anti-solar magnetotail (`-sun_local` twin plasma lobes); spawn GEO ring (`r = 1.65`) of 3-axis satellite glyphs (`sd_box` bus + twin solar wings) and occasional glowing relativistic cosmic-ray streaks via `doc_feat` switches (`0`: solar wind particles along `-sun_local`, `1`: bright B-field dipole tubes + neutral sheet, `2`: Cherenkov/cosmic streaks deflected by Lorentz `cross(v, B)`, `3`: GEO satellite constellation ring + ground link beams, `4`: interplanetary density scalar gradient).

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **viento solar** / *solar wind* | `[TIER-D]` | Una corriente de partículas procedente del Sol atraviesa el espacio cercano. // *A stream of particles from the Sun passes through nearby space.* | No directional solar-origin stream or dayside magnetopause shock. | Emit directional solar-wind plasma stream along `-sun_local` colliding with Earth's dayside bow shock (`comp=3621`, `doc_feat == 0`). |
| `1` | **magnetosfera** / *magnetosphere* | `[TIER-B]` | El campo magnético terrestre desvía gran parte del viento solar. // *The Earth's magnetic field deflects much of the solar wind.* | Dipole Van Allen flux arcs (`z_idx == 0u`) depict magnetosphere, though unisolated on `doc_feat`. | Isolate inner/outer Van Allen dipole flux arcs (`L=1.18, 1.48`) + equatorial neutral sheet on `comp=3621` (`doc_feat == 1`). |
| `2` | **radiación cósmica** / *cosmic radiation* | `[TIER-D]` | Partículas de enorme energía llegan desde el Sol y desde más allá del sistema solar. // *Particles of enormous energy arrive from the Sun and from beyond the solar system.* | No high-energy cosmic-ray trajectory tracks or solar energetic particle bursts. | Trace bent relativistic Cherenkov/cosmic-ray particle arcs deflected via Lorentz `cross(v, B)` around dipole (`doc_feat == 2`). |
| `3` | **satélite lejano** / *distant satellite* | `[TIER-D]` | Los satélites de órbita alta observan, comunican y navegan alrededor del planeta. // *High orbit satellites observe, communicate and navigate around the planet.* | Zero high-orbit GEO/GPS satellites despite line 256 shader comment. | Render equatorial GEO/GNSS constellation ring (`r = 1.65`) of 3-axis satellite glyphs (`comp=3622`) + nadir RF cones (`doc_feat == 3`). |
| `4` | **vacío interplanetario** / *interplanetary vacuum* | `[TIER-D]` | No está completamente vacío: contiene campos, fotones y partículas muy dispersas. // *It is not completely empty: it contains widely dispersed fields, photons and particles.* | No interplanetary field/photon/tenuous plasma visualization. | Display subtle interplanetary plasma density gradient + passing solar photon wavefront vectors (`doc_feat == 4`). |

#### `[TIER-B]` Exp `03` `Z01` `exosphere` — *Exosfera* / *Exosphere* (`vid = 1301` | Feats: `0A / 0B / 0C / 5D`)
- **Metadata:** `raw_metric`: "átomos extremadamente dispersos" *(EN: "extremely dispersed atoms")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[700.0, 10000.0]` km
- **Spoken Macro (ES // EN):** Es la capa exterior de la atmósfera y se mezcla poco a poco con el espacio. // *It is the outer layer of the atmosphere and gradually mixes with space.*
- **Rendered Today:** Shares identical `show_magnetosphere` (`z_idx == 1u`, `mag_gain=0.92`, `hero_doc_cosmos.wgsl:262`) Van Allen dipole lines + Rayleigh atmospheric limb + Mode 11 particles.
- **Audit Gap:** Visually indistinguishable from `z=0` except camera altitude! Missing diffuse UV Lyman-alpha hydrogen geocorona cloud, thermal Jeans atmospheric escape trails, helium boundary band, and HEO satellites. `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1301`):** Add dedicated `z_idx == 1u` volumetric branch inside `raymarch_doc_earth_shell`: wide soft Rayleigh/Lyman-alpha purple-cyan radial exponential halo (`exp(-(r - 0.80) * 6.0)` out to `1.85*R`) modulated on `doc_feat`: (`0`: flickering light H-atom density shells, `1`: inner He-rich band `0.78..0.95`, `2`: ballistic Jeans-escape outward streaks curving away from thermobase, `3`: multi-Earth-radii geocorona glow, `4`: elliptical Molniya/HEO satellite orbital tracks).

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **hidrógeno** / *hydrogen* | `[TIER-D]` | Los átomos ligeros de hidrógeno pueden recorrer grandes distancias sin chocar. // *Light hydrogen atoms can travel long distances without colliding.* | No tenuous hydrogen atom halo or scattering envelope. | Render wide diffuse hydrogen envelope with flickering light H-atom scattering density out to `1.85*R` (`doc_feat == 0`). |
| `1` | **helio** / *helium* | `[TIER-D]` | El helio es uno de los gases que dominan en esta región extremadamente tenue. // *Helium is one of the dominant gases in this extremely tenuous region.* | No dominant helium shell band distinguishable from magnetosphere. | Highlight distinct inner helium-dominated shell (`0.78..0.95*R`) tinted warm pale gold (`doc_feat == 1`). |
| `2` | **escape atmosférico** / *atmospheric escape* | `[TIER-D]` | Algunas partículas rápidas vencen la gravedad y abandonan la Tierra. // *Some fast particles overcome gravity and leave the Earth.* | No upward escaping fast-atom trajectories overcoming gravity. | Animate outward ballistic Jeans-escape hydrogen atom streaks curving radially past escape velocity (`doc_feat == 2`). |
| `3` | **geocorona** / *geocorona* | `[TIER-D]` | Una nube muy tenue de hidrógeno rodea al planeta y se extiende mucho más allá. // *A very faint cloud of hydrogen surrounds the planet and extends far beyond it.* | No faint multi-radii hydrogen geocorona cloud. | Illuminate UV Lyman-alpha hydrogen geocorona halo spanning multiple Earth radii on enlarged shell box (`doc_feat == 3`). |
| `4` | **satélite** / *satellite* | `[TIER-D]` | Numerosos satélites de órbita alta atraviesan esta frontera difusa. // *Numerous high-orbit satellites cross this diffuse boundary.* | No high-orbit satellites traversing boundary. | Trace inclined elliptical Molniya / HEO satellite trajectories (`comp=3622`) sweeping through exobase (`doc_feat == 4`). |

#### `[TIER-B]` Exp `03` `Z02` `thermosphere` — *Termosfera* / *Thermosphere* (`vid = 1302` | Feats: `0A / 1B / 0C / 4D`)
- **Metadata:** `raw_metric`: "gas muy tenue y parcialmente ionizado" *(EN: "very tenuous and partially ionized gas")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[80.0, 700.0]` km
- **Spoken Macro (ES // EN):** Absorbe radiación solar energética; aquí brillan muchas auroras y orbita la estación espacial. // *It absorbs high-energy solar radiation; many auroras glow here, and the space station orbits here.*
- **Rendered Today:** Polar auroral oval curtains (`r in 0.739..0.915`, `lat ~ 0.75`, `aur_gain = 1.65` for `z_idx == 2u`, `hero_doc_cosmos.wgsl:274-293`) driven by 2D `cs_field_grid` Mode 2 PDE curtain (`compute_universe.wgsl:428-436`) with green O / violet N2 altitude gradient.
- **Audit Gap:** Despite `hero_doc_cosmos.wgsl:295` comment (`+ Satellites / ISS Orbit`), ZERO ISS geometry exists. Missing global F/E-layer ionosphere plasma shell, dissociated atomic-oxygen layer marker, and bouncing HF radio waves. `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1302`):** Keep PDE auroral curtains (`f=0` isolates curtains + field-aligned Birkeland current tubes); add `f=1`: concentric glowing cyan-magenta F2/E ionosphere plasma shell (`r = 0.78..0.85`) excited on sunlit hemisphere; `f=2`: procedural ISS `sd_box` truss + 8 golden solar-array wings orbiting at inclination `51.6 deg` (`r = 0.765`); `f=3`: O2->2O UV photodissociation zone tinted 557.7nm green; `f=4`: animated sinusoidal HF radio beams bouncing between ground transmitter and curved ionosphere shell.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **aurora** / *aurora* | `[TIER-B]` | Partículas solares excitan gases de la alta atmósfera y producen cortinas de luz. // *Solar particles excite gases in the upper atmosphere and produce curtains of light.* | PDE-driven green/violet polar auroral curtains explicitly boosted (`aur_gain=1.65`) for `z_idx == 2u`. | Isolate `cs_field_grid` Mode 2 PDE polar auroral curtains + vertical Birkeland field-aligned current filaments (`doc_feat == 0`). |
| `1` | **ionosfera** / *ionosphere* | `[TIER-D]` | La radiación solar libera electrones y transforma parte del gas en plasma. // *Solar radiation releases electrons and transforms part of the gas into plasma.* | No global dayside ionized ionosphere plasma shell separate from polar auroral ovals. | Render sunlit concentric cyan-magenta F2/E ionosphere plasma shell (`r = 0.78..0.85`, `doc_feat == 1`). |
| `2` | **estación espacial** / *space station* | `[TIER-D]` | La Estación Espacial Internacional orbita dentro de la termosfera. // *The International Space Station orbits within the thermosphere.* | Zero ISS spacecraft geometry/orbit despite `hero_doc_cosmos.wgsl:295` comment. | Orbit procedural ISS `sd_box` truss + 8 golden solar-array wings (`comp=3622`) at `51.6 deg` inclination (`doc_feat == 2`). |
| `3` | **oxígeno atómico** / *atomic oxygen* | `[TIER-D]` | La radiación rompe moléculas y deja átomos de oxígeno muy separados. // *The radiation breaks up molecules and leaves oxygen atoms far apart.* | No UV molecular-oxygen dissociation / atomic-oxygen layer indicator. | Tint `r ~ 0.76` O2 -> 2O UV photodissociation zone in 557.7 nm atomic-oxygen emerald emission (`doc_feat == 3`). |
| `4` | **onda de radio** / *radio wave* | `[TIER-D]` | La ionosfera modifica y, en ciertas frecuencias, devuelve ondas de radio. // *The ionosphere modifies and, at certain frequencies, returns radio waves.* | No ground-to-ionosphere skywave radio bounce arcs. | Animate bouncing sinusoidal HF skywave radio arcs reflecting off the underside of the ionized ionosphere shell (`doc_feat == 4`). |

#### `[TIER-D]` Exp `03` `Z03` `mesosphere` — *Mesosfera* / *Mesosphere* (`vid = 1303` | Feats: `0A / 0B / 0C / 5D`)
- **Metadata:** `raw_metric`: "aire extremadamente tenue" *(EN: "extremely thin air")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[50.0, 80.0]` km
- **Spoken Macro (ES // EN):** Es la capa atmosférica más fría y el lugar donde se desintegran la mayoría de los meteoros. // *It is the coldest atmospheric layer and the place where most meteors disintegrate.*
- **Rendered Today:** NO `z_idx == 3u` branch anywhere in `shade_doc_earth_surface` or `raymarch_doc_earth_shell` (`hero_doc_cosmos.wgsl:15-48, 259-308`). Despite `hero_doc_cosmos.wgsl:256` comment (`Mesosphere Meteors`), draws ONLY generic Earth clouds/surface + ambient aurora (`aur_gain=1.0`); magnetosphere hidden.
- **Audit Gap:** CRITICAL [TIER-D]: Every spoken mesosphere phenomenon—incandescent meteor ablation streaks, high-altitude twilight noctilucent ice clouds, mesopause cold minimum (~-90 C), and sounding rockets—is completely absent from screen.
- **3D Zone Blueprint (`vid = 1303`):** Implement dedicated `z_idx == 3u` atmospheric shell layer (`r = 0.72..0.77`, 50–80 km altitude mapped to shell): high-velocity hashed incandescent meteor streaks using rotating tangential `sd_capsule` plasma wakes with white-hot leading tip + cooling orange Na/Fe trail (`f=0`); wispy electric-blue terminator-lit noctilucent ice clouds at `|lat| > 0.78` (`f=1`); icy-cyan thermal inversion trough contour at mesopause `r = 0.765` (`f=2`); ascending suborbital sounding rocket parabolic arc with exhaust plume (`f=3`); and compression shock sheath around incoming micrometeoroids (`f=4`).

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **meteoro** / *meteor* | `[TIER-D]` | La fricción y la compresión del aire calientan y erosionan partículas espaciales. // *Friction and air compression heat and erode space particles.* | Zero meteor entry/ablation streaks despite explicit line 256 shader comment. | Generate high-velocity tangential `sd_capsule` incandescent meteor ablation trails (`r = 0.72..0.77`) with Na/Fe wakes (`doc_feat == 0`). |
| `1` | **nube noctilucente** / *noctilucent cloud* | `[TIER-D]` | Cristales de hielo muy altos pueden brillar después de la puesta del Sol. // *Ice crystals at very high altitudes can glow after sunset.* | No high-latitude twilight noctilucent ice-crystal clouds. | Synthesize wispy electric-blue terminator-lit polar noctilucent ice-crystal clouds at `\|lat\| > 0.78` (`doc_feat == 1`). |
| `2` | **mesopausa** / *mesopause* | `[TIER-D]` | Cerca del borde superior se encuentran algunas de las temperaturas más bajas de la Tierra. // *Near the top edge are some of the lowest temperatures on Earth.* | No mesopause ~50-80 km temperature-minimum boundary halo. | Highlight icy-cyan ~-90 C mesopause thermal-minimum iso-contour shell at `r = 0.765` (`doc_feat == 2`). |
| `3` | **cohete sonda** / *sounding rocket* | `[TIER-D]` | Los cohetes científicos cruzan esta región difícil de alcanzar. // *Scientific rockets cross this difficult-to-reach region.* | No ascending scientific sounding-rocket ballistic trajectory. | Trace ascending parabolic scientific sounding-rocket exhaust plume (`comp=3623`) piercing 50–80 km band (`doc_feat == 3`). |
| `4` | **aire tenue** / *thin air* | `[TIER-D]` | Aunque casi no se nota, todavía hay suficiente gas para frenar meteoros. // *Although it is barely noticeable, there is still enough gas to slow down meteors.* | No friction/compression deceleration boundary layer. | Visualize aerodynamic compression bow sheath braking tiny incoming micrometeoroids (`doc_feat == 4`). |

#### `[TIER-B]` Exp `03` `Z04` `stratosphere` — *Estratosfera* / *Stratosphere* (`vid = 1304` | Feats: `0A / 1B / 0C / 4D`)
- **Metadata:** `raw_metric`: "aire seco y estable" *(EN: "dry and stable air")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[12.0, 50.0]` km
- **Spoken Macro (ES // EN):** La capa de ozono absorbe gran parte de la radiación ultravioleta y calienta esta región desde arriba. // *The ozone layer absorbs much of the ultraviolet radiation and warms this region from above.*
- **Rendered Today:** Earth globe (`shade_doc_earth_surface`) with explicit `z_idx == 4u` indigo Fresnel `ozone_glow` (`0.95` gain at `hero_doc_cosmos.wgsl:43`) along atmospheric limb.
- **Audit Gap:** Missing incoming solar UV ray absorption attenuation, stratospheric weather balloon, commercial/supersonic jet contrail arcs, and temperature-inversion stratified air bands. `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1304`):** Add `z_idx == 4u` sub-staging: `f=0`: bright UV-absorbing ozone band ring at 20–35 km limb offset; `f=1`: translucent white high-altitude research balloon + suspended gondola SDF rising smoothly in the shell; `f=2`: sleek delta-wing jet + persistent white condensation trail along tropopause/lower stratosphere; `f=3`: horizontally banded laminar thermal-inversion layers resisting vertical plumes; `f=4`: incident violet UV rays along `sun_local` visibly quenching inside the indigo ozone band.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **capa de ozono** / *ozone layer* | `[TIER-B]` | La mayor parte del ozono atmosférico está aquí y ayuda a filtrar radiación ultravioleta. // *Most of the atmospheric ozone is here and helps filter ultraviolet radiation.* | `ozone_glow` indigo Fresnel rim explicitly boosts on `z_idx == 4u` (`hero_doc_cosmos.wgsl:43`). | Isolate bright indigo-violet UV-absorbing ozone belt ring offset at 20–35 km limb altitude (`doc_feat == 0`). |
| `1` | **globo estratosférico** / *stratospheric balloon* | `[TIER-D]` | Globos científicos ascienden hasta el aire delgado para observar la Tierra y el cielo. // *Scientific balloons ascend into the thin air to observe the Earth and the sky.* | No rising stratospheric scientific balloon + payload. | Render slowly ascending translucent white stratospheric sounding balloon + gondola SDF (`comp=3624`, `doc_feat == 1`). |
| `2` | **avión a reacción** / *jet plane* | `[TIER-D]` | Los vuelos de gran altitud aprovechan el aire más estable cerca de la baja estratosfera. // *High-altitude flights take advantage of the more stable air near the lower stratosphere.* | No high-altitude jet contrail / stable lower-stratosphere flight path. | Trace high-altitude supersonic/jetliner SDF + persistent white contrail ribbon hugging lower stratosphere (`doc_feat == 2`). |
| `3` | **aire estratificado** / *stratified air* | `[TIER-D]` | Las capas se mezclan lentamente porque la temperatura aumenta con la altura. // *The layers mix slowly because the temperature increases with altitude.* | No stratified non-mixing temperature inversion layering. | Banding horizontal laminar temperature-inversion strata visually damping vertical convective plumes (`doc_feat == 3`). |
| `4` | **radiación ultravioleta** / *ultraviolet radiation* | `[TIER-D]` | El ozono y el oxígeno absorben energía ultravioleta procedente del Sol. // *Ozone and oxygen absorb ultraviolet energy from the Sun.* | No solar UV absorption / attenuation arrows. | Stream incoming violet UV vectors from `sun_local` visibly attenuating inside the indigo ozone band (`doc_feat == 4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT1-EXP03-Z00-04)</summary>

```markdown
Task: Implement Pick-Up Package PKG-ACT1-EXP03-Z00-04 (Exp 03 Deep Earth — Outer Space, Exosphere, Thermosphere, Mesosphere & Stratosphere (`Z00..Z04`)) in HyperCosmosVR.
Prerequisites: Confirm PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING have landed (vid >= 1000u decoder, base_flags cue packing, doc_feat(inst) active).

Target Files:
- `crates/cosmos_render/src/doc_stages.rs` (`emit_doc_stage_hero` `L567-611`: expand `r_vol = earth_solid_r * 2.1`, map `Z00..Z04` -> `vid = 1300..1304`, spawn companion GEO/ISS/rocket/balloon rigs `3621..3624`)
- `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS` `L952-961`: leverage existing `aurora, iss, satelite, ozono, luna` + add `exosfera, geocorona, termosfera, ionosfera, mesosfera, meteoro, noctilucente, estratosfera, globo`)
- `shaders/hero_doc_cosmos.wgsl` (`shade_doc_earth_surface` `L4-49` & `raymarch_doc_earth_shell` `L245-309`: decode `vid >= 1000u`, implement `z=3` Mesosphere ablation/noctilucent clouds/sounding rocket, decouple `z=0` bow shock from `z=1` Lyman-alpha geocorona, wire `doc_feat(inst) == 0..4`)
- `shaders/compute_universe.wgsl` (`L297-309, L428-436`: couple Mode 11 solar-wind bow-shock advection when `active_zone_index == 0` and Jeans-escape particle vectors on `Z01`)

Reserved Canonical Shader IDs & Wiring:
- Primary zone vids: `1300` (`Z00 space`), `1301` (`Z01 exosphere`), `1302` (`Z02 thermosphere`), `1303` (`Z03 mesosphere`), `1304` (`Z04 stratosphere`)
- Companion/overlay vids: `3620` (Moon), `3621` (`submode=32` Bow-Shock Magnetosphere), `3622` (`submode=32` GEO/GNSS), `3621` (`submode=33` Geocorona Envelope), `3622` (`submode=33` HEO/Molniya Tracks), `3622` (`submode=34` ISS Truss + 8 Solar Wings), `cs_field_grid` Mode 2, `3623` (Suborbital Sounding Rocket Arc), `3624` (Stratospheric Balloon & Supersonic Jet/Contrail)
- Preserve bits 24..31 submode and bits 0..7 base_flags; branch explicitly on doc_feat(inst) (0..k for spoken features, 0xFF/255 for zone macro), smoothed by doc_cue(inst) and doc_weight(inst) clamped cleanly into HDR AgX range without highlight blowout.
- Wire bilingual Spanish/English narration keywords & SegmentVisualHint::cue_tags in resolver.rs covering: viento solar / magnetosfera / radiación cósmica / satélite lejano / exosfera / geocorona / escape atmosférico / aurora / ionosfera / estación espacial / meteoro / nube noctilucente / mesopausa / cohete sonda / capa de ozono / globo estratosférico / radiación ultravioleta.

Strict Rendering & Performance Budgets (Mandatory):
- Pure-Black OLED Void Budget: preserve >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout; use bounded local integration in shaders/hero_volumetric.wgsl with ZERO full-screen gray haze or background lift.
- VR 90/120 Hz Frame-Time Budget: cap any new volumetric documentary hero shell at <= 28 raymarch steps inside bounded r <= 0.96 with early exit trans < 0.03, preserving the RTX 5070 DX12 baseline (min = 0.843 ms, mean = 1.863 ms / 536.7 FPS, p99 = 4.661 ms, max = 5.802 ms in artifacts/verification_renders/benchmark_sweep.txt). Keep `raymarch_doc_earth_shell` strictly `<= 28` march steps with bounded `r <= 0.96` envelope (`trans < 0.03` early-exit) so enlarged `2.1*earth_solid_r` volume box preserves zero background lift.
- Architecture & Repo Hygiene (CONTRIBUTING.md): zero 2D HUD overlays (keep UI purely diegetic 3D); commit zero files > 50 MiB, zero .ttf/.otf font binaries, and zero machine-specific .cargo/config.toml.

Required Verification Gates Before Opening PR:
1. cargo clippy --workspace --all-targets -- -D warnings
2. cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey
3. cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe
4. cargo test -p cosmos_spatial topology
5. cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets
6. Local GPU validation (if Discrete DX12 GPU present): cargo test --workspace -- --test-threads=1 && cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
```

</details>

<a id="pkg-act1-exp03-z05-08"></a>
### PKG-ACT1-EXP03-Z05-08: Exp 03 Deep Earth — Troposphere, Surface Cutaway Bug Fix, Crust & Lithosphere Moho Split (`Z05..Z08`) `[COMPLETED — TIER-A REFERENCE EXEMPLAR]`
- **Status & Rollout:** **`[COMPLETED — TIER-A]`** · Phase 1/3 Reference Exemplar (`1305..=1308`, `doc_feat 0..=4` wired & GPU-verified in `p0_canonical_id_registry_cue_plumbing_and_exp03_z05_08_tier_a_hold`)
- **Scope:** Exp `03` (`03`) Zones `Z05..Z08` (`4` zones, `20` features)
- **Current Card Tiers:** Zones `4A / 0B / 0C / 0D` · Features `20A / 0B / 0C / 0D`

**Implemented State (Reference Pattern for Contributors):**
- **Primary Dispatch & Cutaway Guard Fixed:** [`doc_stages.rs`](../../crates/cosmos_render/src/doc_stages.rs) guards `is_surface_or_atmo_zone = doc.active_zone_index <= 6` so `Z06 Surface` (`rel_scale_u = 6.05`) stays intact (`cutaway = 0`), and routes `Z05..=Z08` onto canonical IDs `1305..=1308` via `resolve_zone_shader_vid` / `canonical_zone_vid`.
- **WGSL Branches (`Z05..Z08` -> `doc_feat 0..4`):** [`hero_doc_cosmos.wgsl`](../../shaders/hero_doc_cosmos.wgsl) decodes canonical `1305..=1308`, drives 1:1 `doc_feat_focus` choreography across all 20 features (`Z05` cumulus clouds / cyclone + lightning / trade-wind ribbons / great-circle flight strobes / ITCZ vapor; `Z06` ocean trenches / continental shelf coast / forest chlorophyll / alpine ridges / terracotta soil margin; `Z07` oceanic vs continental crust above `moho_r = 0.938..0.962` / plate sutures / seismic fault waves / magma conduits; `Z08` composite lithospheric plates / subducting slab / continental keel / Wadati-Benioff deep-focus earthquakes / rigid upper-mantle stratum `0.885..moho_r`).
- **Card Tier Breakdown (`4` zones, `20` features):** Zones `4A / 0B / 0C / 0D` · Features `20A / 0B / 0C / 0D`.

**Reserved Target IDs:**

| Zone | Slug | Canonical `vid` | Companion `vid` (`3600..3869`) | `submode` (`bits 24..31`) | `base_flags` (`bits 0..7`) | `doc_feat` Slots (`0..7`, `0xFF=macro`) |
|---|---|---:|---|---|---|---:|
| `Z05` | `troposphere` | `1305` | `3625` (Great-Circle Commercial Jets) | `0` (solid globe + cloud/cyclone shell) | cue mask | `0..4` |
| `Z06` | `surface` | `1306` | `3626` (Soil/Pedosphere Margin Inset) | `0` (`cutaway = 0` guard enforced!) | cue mask | `0..4` |
| `Z07` | `crust` | `1307` | `3627` (Volcanic Magma Conduit Inset) | `1` (cutaway enabled, Moho at `0.94..0.96`) | `0x01` + cue mask | `0..4` |
| `Z08` | `lithosphere` | `1308` | `3628` (Benioff Seismic Rupture Sparks) | `1` (cutaway enabled, rigid mantle `0.89..0.96`) | `0x01` + cue mask | `0..4` |

**Spoken Transcript Summary & 3D Reference Implementation (`[TIER-A]` Exemplar):**

#### `[TIER-A]` Exp `03` `Z05` `troposphere` — *Troposfera* / *Troposphere* (`vid = 1305` | Feats: `5A / 0B / 0C / 0D`)
- **Metadata:** `raw_metric`: "aire denso, vapor y aerosoles" *(EN: "dense air, vapor and aerosols")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[0.0, 12.0]` km
- **Spoken Macro (ES // EN):** Es la capa que respiramos: contiene casi todo el vapor de agua y donde ocurre el tiempo meteorológico. // *It is the layer in which we live and breathe: it contains almost all atmospheric water vapor and is where weather occurs.*
- **Implemented Reference State:** Solid globe + tropospheric shell (`submode = 2` via `doc_stages.rs:645-667`, `vid = 1305`) decoded in `hero_doc_cosmos.wgsl:15-107` & `364-394`, driving smooth `doc_feat_focus(inst, 0..=4)` choreography across cumulus decks (`F0`), synoptic cyclone + lightning (`F1`), equatorial Hadley/trade-wind ribbons + limb jet (`F2`), great-circle commercial jet corridors + pulsing navigation strobes (`F3`), and ITCZ translucent water-vapor plume belts (`F4`). Pairwise pixel separation verified across all `5` features on RTX 5070 in `p0_canonical_id_registry_cue_plumbing_and_exp03_z05_08_tier_a_hold`.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Implemented Code & Reference Behavior (`hero_doc_cosmos.wgsl`, `doc_stages.rs`) |
|---:|---|:---:|---|---|
| `0` | **nube** / *cloud* | `[TIER-A]` | El vapor de agua se condensa en gotas o cristales de hielo suspendidos. // *Water vapor condenses into suspended droplets or ice crystals.* | Procedural FBM `clouds` modulated by `f_cloud = doc_feat_focus(inst, 0u)` (`hero_doc_cosmos.wgsl:55,61-62`). |
| `1` | **tormenta** / *storm* | `[TIER-A]` | El aire cálido asciende y puede organizar nubes, lluvia, rayos y fuertes corrientes. // *Warm air rises and can produce organized systems of clouds, rain, lightning, and strong air currents.* | Rotating sunlit synoptic cyclone spiral + crackle `storm_bolt` flashes gated by `f_storm = doc_feat_focus(inst, 1u)` (`hero_doc_cosmos.wgsl:67-75`). |
| `2` | **viento** / *wind* | `[TIER-A]` | Las diferencias de presión ponen el aire en movimiento alrededor del planeta. // *Pressure differences put air in motion around the planet.* | Equatorial/mid-latitude wind streamlines (`tropo_fx`) + volumetric limb jet stream driven by `f_wind = doc_feat_focus(inst, 2u)` (`hero_doc_cosmos.wgsl:77-81,381-393`). |
| `3` | **avión** / *airplane* | `[TIER-A]` | La mayoría de los vuelos atraviesan la troposfera y su borde superior. // *Most flights pass through the troposphere and its upper edge.* | Dual intersecting great-circle transoceanic flight corridors + pulsing amber strobe aircraft controlled by `f_plane = doc_feat_focus(inst, 3u)` (`hero_doc_cosmos.wgsl:83-87`). |
| `4` | **vapor de agua** / *water vapor* | `[TIER-A]` | Casi todo el vapor atmosférico está concentrado en esta capa inferior. // *Almost all atmospheric vapor is concentrated in this lower layer.* | Teal-cyan ITCZ column water-vapor band + equatorial volumetric limb wisps via `f_vapor = doc_feat_focus(inst, 4u)` (`hero_doc_cosmos.wgsl:89-91,384-393`). |

#### `[TIER-A]` Exp `03` `Z06` `surface` — *Superficie* / *Surface* (`vid = 1306` | Feats: `5A / 0B / 0C / 0D`)
- **Metadata:** `raw_metric`: "roca, suelo, agua y vida" *(EN: "rock, soil, water and life")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[-11.0, 2.0]` km
- **Spoken Macro (ES // EN):** Una frontera finísima separa la atmósfera del océano y de la Tierra sólida. // *An extremely thin boundary separates the atmosphere from the ocean and solid Earth.*
- **Implemented Reference State:** Fixed premature cutaway bug in `engine.rs` (`documentary.active_zone_index > 6`) and `doc_stages.rs:629-639` (`is_surface_or_atmo_zone = doc.active_zone_index <= 6`), holding Earth intact (`cutaway = 0`, `vid = 1306`) at `Z06` (`u = 6.05`). `shade_doc_earth_surface` (`hero_doc_cosmos.wgsl:25-51`) spotlights ocean bathymetry & abyssal trenches (`F0`), continental shelf ribbons & coastal city lights (`F1`), chlorophyll forest belts (`F2`), alpine mountain ridges (`F3`), and living terracotta pedosphere soil bands (`F4`). Pairwise pixel separation verified across all `5` features in `render_tests.rs`.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Implemented Code & Reference Behavior (`hero_doc_cosmos.wgsl`, `doc_stages.rs`) |
|---:|---|:---:|---|---|
| `0` | **océano** / *ocean* | `[TIER-A]` | El agua líquida cubre la mayor parte de la superficie terrestre. // *Liquid water covers most of the Earth's surface.* | Globe held solid (`cutaway = 0` at `u = 6.05`); abyssal ocean bathymetry, trenches & specular glints spotlighted via `f_ocean = doc_feat_focus(inst, 0u)` (`hero_doc_cosmos.wgsl:33-35,95`). |
| `1` | **costa** / *coast* | `[TIER-A]` | Aquí se encuentran y cambian continuamente el océano, la roca y la atmósfera. // *Here the ocean, rock and atmosphere meet and continually change.* | Turquoise continental shelf `shelf_ribbon` + nightside coastal city glow driven by `f_coast = doc_feat_focus(inst, 1u)` (`hero_doc_cosmos.wgsl:36,47,99`). |
| `2` | **bosque** / *forest* | `[TIER-A]` | La vida intercambia agua, carbono, oxígeno y energía con el aire y el suelo. // *Life exchanges water, carbon, oxygen and energy with air and soil.* | Mid-latitude/tropical vegetation chlorophyll canopy (`veg_mask`) accentuated by `f_forest = doc_feat_focus(inst, 2u)` (`hero_doc_cosmos.wgsl:39,42`). |
| `3` | **montaña** / *mountain* | `[TIER-A]` | La corteza continental se eleva y deja expuestas rocas formadas durante millones de años. // *The continental crust rises and exposes rocks formed over millions of years.* | Amplified orogenic mountain relief + snow-dusted highland ridges spotlighted by `f_mtn = doc_feat_focus(inst, 3u)` (`hero_doc_cosmos.wgsl:37,44`). |
| `4` | **suelo** / *soil* | `[TIER-A]` | Una capa viva y compleja conecta la roca, el agua, el aire y los organismos. // *A complex living layer connects rock, water, air and organisms.* | Warm terracotta continental `soil_band` regolith transition highlighted by `f_soil = doc_feat_focus(inst, 4u)` (`hero_doc_cosmos.wgsl:40,43`). |

#### `[TIER-A]` Exp `03` `Z07` `crust` — *Corteza* / *Crust* (`vid = 1307` | Feats: `5A / 0B / 0C / 0D`)
- **Metadata:** `raw_metric`: "roca sólida y quebradiza" *(EN: "solid and brittle rock")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[35.0, 0.0]` km
- **Spoken Macro (ES // EN):** Es la piel rocosa del planeta: delgada bajo los océanos y más gruesa bajo los continentes. // *It is the rocky skin of the planet: thin under the oceans and thicker under the continents.*
- **Implemented Reference State:** Routed to canonical `vid = 1307` (`cutaway = 1`) with `seam_moho` placed accurately at `moho_r = mix(0.962, 0.938, cont_region)` in `shade_doc_earth_cutaway` (`hero_doc_cosmos.wgsl:126-159,192`), cleanly separating `Z07` crust (`wall_r > moho_r`) from `Z08` rigid lithospheric mantle below. Individual `doc_feat_focus(inst, 0..=4)` channels isolate thin basaltic oceanic crust (`F0`), thick granitic continental crust (`F1`), mosaic plate sutures (`F2`), fault planes + concentric seismic wave rings (`F3`), and rising amber volcanic magma conduits (`F4`). Pairwise pixel separation verified across all `5` features in `render_tests.rs`.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Implemented Code & Reference Behavior (`hero_doc_cosmos.wgsl`, `doc_stages.rs`) |
|---:|---|:---:|---|---|
| `0` | **corteza oceánica** / *oceanic crust* | `[TIER-A]` | Es más delgada y densa, formada principalmente por rocas basálticas. // *It is thinner and denser and is composed mainly of basaltic rock.* | Thin cool-blue basaltic oceanic crust (`wall_r = 0.962..1.0` above `moho_r`) driven by `c_ocean = doc_feat_focus(inst, 0u)` (`hero_doc_cosmos.wgsl:147,149`). |
| `1` | **corteza continental** / *continental crust* | `[TIER-A]` | Es más gruesa, menos densa y conserva rocas muy antiguas. // *It is thicker, less dense and preserves very old rocks.* | Thickened warm granitic continental crust (`wall_r = 0.938..1.0` above `moho_r`) driven by `c_cont = doc_feat_focus(inst, 1u)` (`hero_doc_cosmos.wgsl:148,149`). |
| `2` | **placa tectónica** / *tectonic plate* | `[TIER-A]` | La corteza y el manto superior rígido se desplazan juntos como placas. // *The crust and the rigid upper mantle move together as plates.* | Crisp tectonic plate suture seams (`plate_suture`) highlighted across crust wall by `c_plate = doc_feat_focus(inst, 2u)` (`hero_doc_cosmos.wgsl:151,158`). |
| `3` | **falla** / *fault* | `[TIER-A]` | Una fractura permite que bloques de roca se muevan y liberen energía sísmica. // *A fracture allows blocks of rock to move and release seismic energy.* | Glowing brittle shear fault plane (`fault_glow`) + animated concentric `seismic_ring` pulses driven by `c_fault = doc_feat_focus(inst, 3u)` (`hero_doc_cosmos.wgsl:152-155,158`). |
| `4` | **cámara magmática** / *magma chamber* | `[TIER-A]` | Roca fundida puede acumularse antes de ascender hacia un volcán. // *Molten rock can accumulate before rising toward a volcano.* | Molten amber magma conduit + feeder piercing through Moho into upper crust controlled by `c_magma = doc_feat_focus(inst, 4u)` (`hero_doc_cosmos.wgsl:156-158,168`). |

#### `[TIER-A]` Exp `03` `Z08` `lithosphere` — *Litosfera* / *Lithosphere* (`vid = 1308` | Feats: `5A / 0B / 0C / 0D`)
- **Metadata:** `raw_metric`: "roca rígida" *(EN: "rigid rock")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[100.0, 35.0]` km
- **Spoken Macro (ES // EN):** Incluye la corteza y la parte superior rígida del manto; está fragmentada en placas tectónicas. // *It includes the crust and the rigid uppermost mantle and is fragmented into tectonic plates.*
- **Implemented Reference State:** Routed to canonical `vid = 1308` (`cutaway = 1`) and isolated cleanly below the Moho across `litho_base_r = 0.885 .. moho_r` (`hero_doc_cosmos.wgsl:126-143,159-174,193`). Distinct `doc_feat_focus(inst, 0..=4)` responses animate coupled crust+mantle plates (`F0`), thermal cold-blue-to-amber subducting slab diving into the upper mantle (`F1`), thickened continental mountain keels (`F2`), sparkling Wadati-Benioff deep-focus rupture foci along the slab (`F3`), and the isolated rigid upper-mantle stratum bounded by `seam_moho` and `seam_litho` (`F4`). Pairwise pixel separation verified across all `5` features in `render_tests.rs`.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Implemented Code & Reference Behavior (`hero_doc_cosmos.wgsl`, `doc_stages.rs`) |
|---:|---|:---:|---|---|
| `0` | **placa tectónica** / *tectonic plate* | `[TIER-A]` | Una gran pieza rígida se desplaza lentamente sobre roca más dúctil. // *A large rigid piece moves slowly over more ductile rock.* | Composite crust + rigid upper-mantle slab and plate sutures modulated by `l_plate = doc_feat_focus(inst, 0u)` (`hero_doc_cosmos.wgsl:149-151,163`). |
| `1` | **subducción** / *subduction* | `[TIER-A]` | Una placa se hunde bajo otra y transporta material hacia el manto. // *One plate sinks under another and transports material into the mantle.* | Thermal blue-to-amber descending subducting slab (`subduct_col`) diving across lithosphere and asthenosphere via `l_subduct = doc_feat_focus(inst, 1u)` (`hero_doc_cosmos.wgsl:166-167,174`). |
| `2` | **raíz continental** / *continental root* | `[TIER-A]` | La corteza gruesa bajo una cordillera se prolonga hacia el manto. // *The thick crust beneath a mountain range extends into the mantle.* | Thickened continental mountain root keel (`root_keel`) extending below `moho_r` illuminated by `l_root = doc_feat_focus(inst, 2u)` (`hero_doc_cosmos.wgsl:161,169`). |
| `3` | **falla profunda** / *deep fault* | `[TIER-A]` | Las tensiones de las placas deforman y rompen la roca rígida. // *Plate stresses deform and break rigid rock.* | Gold Wadati-Benioff deep-focus shear rupture sparks (`slab_foci`) along the subducting slab driven by `l_quake = doc_feat_focus(inst, 3u)` (`hero_doc_cosmos.wgsl:165,169`). |
| `4` | **manto superior rígido** / *rigid upper mantle* | `[TIER-A]` | Esta roca forma la base fuerte de la litosfera. // *This rock forms the strong base of the lithosphere.* | Rigid upper-mantle stratum (`litho_base_r = 0.885 .. moho_r`) framed between `seam_moho` and `seam_litho` via `l_mantle = doc_feat_focus(inst, 4u)` (`hero_doc_cosmos.wgsl:159-163,192-194`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT1-EXP03-Z05-08)</summary>

```markdown
Task: Implement Pick-Up Package PKG-ACT1-EXP03-Z05-08 (Exp 03 Deep Earth — Troposphere, Surface Cutaway Bug Fix, Crust & Lithosphere Moho Split (`Z05..Z08`)) in HyperCosmosVR.
Prerequisites: Confirm PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING have landed (vid >= 1000u decoder, base_flags cue packing, doc_feat(inst) active).

Target Files:
- `crates/cosmos_render/src/doc_stages.rs` (`L573-576`: fix cutaway condition to `doc.active_zone_index >= 7 || (input.earth_cutaway_blend > 0.25 && doc.active_zone_index >= 7)` so `Z06` stays solid; map `Z05..Z08` -> `vid = 1305..1308` with `submode = 1` & `base_flags |= 0x01` on `Z07..Z08`)
- `crates/cosmos_narration/src/resolver.rs` (`L952-961`: leverage `corteza, subduccion` + add `troposfera, tormenta, avion, vapor, suelo, litosfera, placa, falla, magma, sismo`)
- `shaders/hero_doc_cosmos.wgsl` (`shade_doc_earth_surface` `L4-49` & `shade_doc_earth_cutaway` `L51-92`: move `seam_moho` to `0.96` ocean / `0.94` continent, split `z_idx == 7u` crust above Moho from `z_idx == 8u` `0.89..0.96` lithosphere + craton keels + Wadati-Benioff earthquakes, wire `doc_feat 0..4`)
- `shaders/compute_universe.wgsl` (`L297-309`: drive cyclone Coriolis spirals on `Z05 F1` and great-circle air corridors on `Z05 F3` inside Mode 11)

Reserved Canonical Shader IDs & Wiring:
- Primary zone vids: `1305` (`Z05 troposphere`), `1306` (`Z06 surface`), `1307` (`Z07 crust`), `1308` (`Z08 lithosphere`)
- Companion/overlay vids: `3625` (Great-Circle Commercial Jets), `3626` (Soil/Pedosphere Margin Inset), `3627` (Volcanic Magma Conduit Inset), `3628` (Benioff Seismic Rupture Sparks)
- Preserve bits 24..31 submode and bits 0..7 base_flags; branch explicitly on doc_feat(inst) (0..k for spoken features, 0xFF/255 for zone macro), smoothed by doc_cue(inst) and doc_weight(inst) clamped cleanly into HDR AgX range without highlight blowout.
- Wire bilingual Spanish/English narration keywords & SegmentVisualHint::cue_tags in resolver.rs covering: nube / tormenta / viento / avión / vapor de agua / océano / costa / vegetación / montaña / suelo / corteza oceánica / corteza continental / placa tectónica / falla sísmica / magma / litosfera / subducción / sismo profundo.

Strict Rendering & Performance Budgets (Mandatory):
- Pure-Black OLED Void Budget: preserve >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout; use bounded local integration in shaders/hero_volumetric.wgsl with ZERO full-screen gray haze or background lift.
- VR 90/120 Hz Frame-Time Budget: cap any new volumetric documentary hero shell at <= 28 raymarch steps inside bounded r <= 0.96 with early exit trans < 0.03, preserving the RTX 5070 DX12 baseline (min = 0.843 ms, mean = 1.863 ms / 536.7 FPS, p99 = 4.661 ms, max = 5.802 ms in artifacts/verification_renders/benchmark_sweep.txt). Maintain solid opaque surface intersection and single-pass cutaway wall evaluation in `shade_doc_earth_cutaway` (`<= 24` raymarch steps for atmospheric cloud/vapor layers; `>= 50.91%` exact `#000000` background pixels).
- Architecture & Repo Hygiene (CONTRIBUTING.md): zero 2D HUD overlays (keep UI purely diegetic 3D); commit zero files > 50 MiB, zero .ttf/.otf font binaries, and zero machine-specific .cargo/config.toml.

Required Verification Gates Before Opening PR:
1. cargo clippy --workspace --all-targets -- -D warnings
2. cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey
3. cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe
4. cargo test -p cosmos_spatial topology
5. cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets
6. Local GPU validation (if Discrete DX12 GPU present): cargo test --workspace -- --test-threads=1 && cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
```

</details>

<a id="pkg-act1-exp03-z09-13"></a>
### PKG-ACT1-EXP03-Z09-13: Exp 03 Deep Earth — Asthenosphere, Transition Zone (`410-660 km`), Lower Mantle, Outer Geodynamo & Inner Core (`Z09..Z13`)
- **Priority & Rollout:** Phase 3 (Upgrades 5 `[TIER-B]` Cutaway Core Layers to Full `doc_feat` Interactive Sub-Stages) · Depends on `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Scope:** Exp `03` (`03`) Zones `Z09..Z13` (`5` zones, `25` features)
- **Current Card Tiers:** Zones `0A / 5B / 0C / 0D` · Features `0A / 12B / 0C / 13D`

**Current State:**
- **Primary Dispatch:** [`doc_stages.rs:567-611`](../../crates/cosmos_render/src/doc_stages.rs#L567-L611) sets `cutaway = 1` for `Z09..Z13` (`vid=0`) + external magnetosphere reactivated on `z_idx == 12u` (`L262`) + Compute Mode 11 geodynamo circulation.
- **WGSL Branches (`Z09..Z13`):** [`hero_doc_cosmos.wgsl:68-88`](../../shaders/hero_doc_cosmos.wgsl#L68-L88) isolates radial cutaway shells (`0.78..0.91` `Z09` asthenosphere, `0.68..0.78` `Z10` transition zone with `seam_660` at `0.72` `L87` but no `seam_410` top ring, `0.46..0.68` `Z11` lower mantle with 4-fold `plume` `L60` + `seam_cmb` `L88`, `<= 0.46` `Z12` liquid outer core `helix` `L83`); [`hero_volumetric.wgsl:84-97`](../../shaders/hero_volumetric.wgsl#L84-L97) renders exposed `r = 0.26` inner core (`1.5x` on `z_idx == 13u` `L90`) with isotropic `cryst` noise. `doc_feat(inst)` is unread.
- **Card Tier Breakdown (`5` zones, `25` features):** Zones `0A / 5B / 0C / 0D` · Features `0A / 12B / 0C / 13D`.

**Reserved Target IDs:**

| Zone | Slug | Canonical `vid` | Companion `vid` (`3600..3869`) | `submode` (`bits 24..31`) | `base_flags` (`bits 0..7`) | `doc_feat` Slots (`0..7`, `0xFF=macro`) |
|---|---|---:|---|---|---|---:|
| `Z09` | `asthenosphere` | `1309` | `3629` (`submode=32` Seismic LVZ Wavefront Overlay) | `1` (cutaway `0.78..0.89`) | `0x01` + cue mask | `0..4` |
| `Z10` | `transition` | `1310` | `3629` (`submode=33` Olivine->Ringwoodite Crystal Inset) | `1` (cutaway `0.68..0.78`) | `0x01` + cue mask | `0..4` |
| `Z11` | `lower-mantle` | `1311` | `3629` (`submode=34` Bridgmanite Crystal + `D''` LLSVPs) | `1` (cutaway `0.46..0.68`) | `0x01` + cue mask | `0..4` |
| `Z12` | `outer-core` | `1312` | `3621` (`submode=34` Coupled External Dipole Lines) | `1` (cutaway `0.26..0.46`) | `0x01` + cue mask | `0..4` |
| `Z13` | `inner-core` | `1313` | `3629` (`submode=35` 360 GPa Caliper & Center Crosshair) | `1` (inner sphere `r <= 0.26`) | `0x01` + cue mask | `0..4` |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `[TIER-B]` Exp `03` `Z09` `asthenosphere` — *Astenosfera* / *Asthenosphere* (`vid = 1309` | Feats: `0A / 2B / 0C / 3D`)
- **Metadata:** `raw_metric`: "roca sólida, caliente y dúctil" *(EN: "solid, hot, ductile rock")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[410.0, 100.0]` km
- **Spoken Macro (ES // EN):** No es un océano de magma: la roca es mayormente sólida, pero puede fluir muy lentamente. // *It is not a magma ocean: the rock is mostly solid, but it can flow very slowly.*
- **Rendered Today:** Cutaway shell `0.78 < wall_r <= 0.91` explicitly boosted (`hi=1.55` on `z_idx == 9u`, `hero_doc_cosmos.wgsl:68-71`) with orange-red flowing FBM convection (`conv`).
- **Audit Gap:** Missing explicit convection roll streamlines, bright partial-melt pocket blobs, upwelling mantle plume stems through asthenosphere (`plume` only activates below `0.78`), and P/S seismic wave-speed curvature. `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1309`):** Add `doc_feat` sub-stages inside `0.78..0.89`: `f=0`: viscoelastic shear-strain grid warping slowly; `f=1`: curl-vector convection cell circulation loops; `f=2`: glowing yellow-white 1-5% partial-melt filigree pockets feeding mid-ocean/arc volcanism; `f=3`: localized hot upwelling mantle plume column piercing upward; `f=4`: animated P/S seismic arcs slowing and curving cleanly through low-velocity zone.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **roca dúctil** / *ductile rock* | `[TIER-B]` | La presión y la temperatura permiten que la roca se deforme sin romperse de inmediato. // *The pressure and temperature allow the rock to deform without immediately breaking.* | Hot glowing ductile rock band explicitly highlighted for `z_idx == 9u`. | Overlay flexing viscoelastic strain wireframe warping slowly inside `0.78..0.89` asthenosphere (`doc_feat == 0`). |
| `1` | **convección** / *convection* | `[TIER-B]` | El calor impulsa movimientos lentos que ayudan a desplazar las placas. // *The heat drives slow movements that help move the plates.* | Slowly drifting FBM `conv` flow animates inside asthenosphere. | Trace closed curl-vector mantle convection roll loops circulating heat upward (`doc_feat == 1`). |
| `2` | **fusión parcial** / *partial melting* | `[TIER-D]` | Una pequeña fracción de la roca puede fundirse y alimentar magmas. // *A small fraction of the rock can melt and feed magmas.* | No discrete partial-melt magma pockets or melt-fraction filigree. | Sparkle yellow-white 1–5% partial-melt magma filigree pockets feeding surface volcanism (`doc_feat == 2`). |
| `3` | **pluma mantélica** / *mantle plume* | `[TIER-D]` | Una columna de material caliente puede ascender desde gran profundidad. // *A column of hot material can rise from great depths.* | No upwelling mantle plume column inside `0.78..0.91` (`plume` starts at `<=0.78`). | Thread hot buoyant upwelling mantle plume columns upward through asthenosphere (`doc_feat == 3`). |
| `4` | **onda sísmica** / *seismic wave* | `[TIER-D]` | Las ondas cambian de velocidad y revelan las propiedades de esta zona. // *The waves change speed and reveal the properties of this area.* | No seismic P/S wavefront slowdown visualization. | Propagate curved P/S seismic wavefronts visibly slowing down and bending inside low-velocity zone (`doc_feat == 4`). |

#### `[TIER-B]` Exp `03` `Z10` `transition` — *Zona de transición* / *transition zone* (`vid = 1310` | Feats: `0A / 1B / 0C / 4D`)
- **Metadata:** `raw_metric`: "minerales comprimidos por enorme presión" *(EN: "minerals compressed by enormous pressure")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[660.0, 410.0]` km
- **Spoken Macro (ES // EN):** Los minerales cambian de estructura aquí, aumentando la densidad sin cambiar su composición básica. // *Minerals change structure here, increasing density without changing their basic composition.*
- **Rendered Today:** Cutaway `0.68 < wall_r <= 0.78` (`hi=1.75` for `z_idx == 10u`, `hero_doc_cosmos.wgsl:72-75`) + `seam_660` ruby-orange discontinuity arc (`line 87`).
- **Audit Gap:** Missing 410-km top boundary seam (`0.78`), atomic crystal compaction contrast (olivine -> wadsleyite/ringwoodite), hydrous OH-mineral pockets, and refracted seismic wave reflection off 410/660 discontinuities. `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1310`):** Add top `seam_410` ring at `wall_r = 0.78` pairing with `seam_660` (`line 87`); wire `doc_feat`: `f=0`: overlaid micro-crystal SDF wireframe morphing from open olivine tetrahedra to dense spinel/ringwoodite octahedra; `f=1`: radial pressure-density color ramp across 410–660 km; `f=2`: glowing cyan hydrous ringwoodite pockets storing ~ocean-scale OH; `f=3`: incident seismic ray reflecting/refracting sharply across both 410 and 660 seams; `f=4`: isolate blazing `seam_660` phase barrier.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **olivino transformado** / *transformed olivine* | `[TIER-D]` | La presión reorganiza los átomos del mineral en estructuras más compactas. // *The pressure rearranges the atoms of the mineral into more compact structures.* | No mineral crystal lattice compaction visual (olivine -> ringwoodite). | Inset morphing silicate tetrahedral->octahedral lattice wireframe (olivine -> wadsleyite/ringwoodite, `doc_feat == 0`). |
| `1` | **alta presión** / *high pressure* | `[TIER-D]` | La materia ocupa menos volumen y se vuelve más densa con la profundidad. // *Matter occupies less volume and becomes denser with depth.* | No pressure-driven density step indicator. | Overlay stepped radial pressure-density color calibration ramp spanning 410–660 km (`doc_feat == 1`). |
| `2` | **agua mineral** / *water stored in minerals* | `[TIER-D]` | Algunos minerales pueden almacenar hidrógeno dentro de su estructura cristalina. // *Some minerals can store hydrogen within their crystalline structure.* | No water-bearing hydrated mineral pockets. | Highlight cyan-glowing hydrous ringwoodite mineral pockets trapping ocean-mass equivalent OH water (`doc_feat == 2`). |
| `3` | **discontinuidad sísmica** / *seismic discontinuity* | `[TIER-D]` | Las ondas sísmicas cambian bruscamente al cruzar transformaciones minerales. // *Seismic waves change abruptly as they cross mineral transformations.* | No 410-km top seam or refracting seismic discontinuity wavefront. | Render new `seam_410` ring paired with `seam_660`, showing seismic rays reflecting/refracting off both (`doc_feat == 3`). |
| `4` | **límite de 660 km** / *660-kilometer boundary* | `[TIER-B]` | Esta frontera marca el paso hacia el manto inferior. // *This boundary marks the passage to the lower mantle.* | `seam_660` glowing discontinuity ring (`line 87`) explicitly marks the 660 km boundary. | Isolate crimson-gold `seam_660` post-spinel mineral phase barrier at `wall_r = 0.68` (`doc_feat == 4`). |

#### `[TIER-B]` Exp `03` `Z11` `lower-mantle` — *Manto inferior* / *Lower mantle* (`vid = 1311` | Feats: `0A / 3B / 0C / 2D`)
- **Metadata:** `raw_metric`: "roca sólida que fluye lentamente" *(EN: "solid rock that flows slowly")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[2900.0, 660.0]` km
- **Spoken Macro (ES // EN):** Es la capa más voluminosa de la Tierra y transporta calor mediante convección extremadamente lenta. // *It is Earth's largest layer by volume and transports heat through extremely slow convection.*
- **Rendered Today:** Cutaway `0.46 < wall_r <= 0.68` (`hi=1.6` for `z_idx == 11u`, `hero_doc_cosmos.wgsl:76-79`) with 4-fold azimuthal `plume` lobes (`line 60`) + `seam_cmb` glowing core-mantle boundary (`line 88`).
- **Audit Gap:** Missing perovskite/bridgmanite lattice inset, closed overturning convection loops + sinking cold slabs, dense solid-state marker, and distinct ULVZ / LLSVP `D''` slab-graveyard morphology above CMB. `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1311`):** Upgrade `0.46..0.68` cutaway wall on `doc_feat`: `f=0`: high-pressure MgSiO3 bridgmanite octahedral crystal inset; `f=1`: full thermal convection dipoles (blue sinking slabs vs red rising superplumes); `f=2`: high-contrast thin buoyantly rising mantle plumes threading from CMB to lithosphere; `f=3`: solid-viscosity stress contour grid; `f=4`: textured `D''` post-perovskite domes (LLSVPs) & ultra-low-velocity lenses hugging `r=0.46..0.49` CMB.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **bridgmanita** / *bridgmanite* | `[TIER-D]` | Este mineral de alta presión es probablemente el más abundante de la Tierra. // *This high-pressure mineral is probably the most abundant on Earth.* | No bridgmanite perovskite crystal structure visual. | Inset ultra-dense high-pressure MgSiO3 bridgmanite crystal cage (`doc_feat == 0`). |
| `1` | **convección profunda** / *deep convection* | `[TIER-B]` | La roca caliente asciende y la más fría desciende durante millones de años. // *Hot rock rises and colder rock sinks over millions of years.* | `conv` + `plume` upwelling bands animate slow deep convection. | Animate full core-to-crust thermal convection dipoles (blue sinking slabs balancing orange plumes, `doc_feat == 1`). |
| `2` | **pluma caliente** / *hot plume* | `[TIER-B]` | Grandes columnas térmicas pueden conectar el manto profundo con volcanismo superficial. // *Large thermal columns can connect the deep mantle with surface volcanism.* | 4-fold azimuthal `plume` pillars (`hero_doc_cosmos.wgsl:60, 79`) highlight rising plumes. | Sharpen thin luminous superplume conduits ascending continuously from CMB toward lithosphere (`doc_feat == 2`). |
| `3` | **roca densa** / *dense rock* | `[TIER-D]` | La presión mantiene la materia sólida incluso a temperaturas muy elevadas. // *The pressure keeps the matter solid even at very high temperatures.* | No high-pressure solid-vs-liquid physical state demarcation. | Render glowing high-pressure solid crystalline viscosity contour rings (`doc_feat == 3`). |
| `4` | **capa D doble prima** / *D-double-prime layer* | `[TIER-B]` | La base del manto contiene estructuras complejas junto al núcleo. // *The base of the mantle contains complex structures next to the core.* | `seam_cmb` core-mantle thermal boundary seam (`line 88`) frames the `D''` region. | Sculpt dense `D''` post-perovskite domes (LLSVPs) & ultra-low-velocity lenses hugging `seam_cmb` at `r=0.46` (`doc_feat == 4`). |

#### `[TIER-B]` Exp `03` `Z12` `outer-core` — *Núcleo externo* / *Outer core* (`vid = 1312` | Feats: `0A / 4B / 0C / 1D`)
- **Metadata:** `raw_metric`: "hierro y níquel líquidos" *(EN: "liquid iron and nickel")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[5150.0, 2900.0]` km
- **Spoken Macro (ES // EN):** El metal líquido en movimiento genera la mayor parte del campo magnético terrestre. // *Moving liquid metal generates most of the Earth's magnetic field.*
- **Rendered Today:** Cutaway `wall_r <= 0.46` (`hi=1.55` for `z_idx == 12u`, `hero_doc_cosmos.wgsl:80-85`) with animated helical Coriolis shear (`helix`) in 3200–4600K blackbody + external dipole magnetosphere lines (`z_idx == 12u` at `line 262`) + Mode 11 toroidal particles.
- **Audit Gap:** Missing explicit Fe vs Ni + ~10% light-element compositional distinction, Taylor-column convective rolls parallel to rotation axis, and internal toroidal-to-poloidal B-field winding inside the liquid shell. `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1312`):** Enhance outer core `0.26 < wall_r <= 0.46`: `f=0`: molten iron convective blackbody swirls; `f=1`: subtle compositional segregation sparkles showing Ni + O/Si/S light elements buoyantly rising off inner core boundary; `f=2`: spiraling 3D Taylor convection columns aligned parallel to Earth's Y rotation axis generating dipole loops; `f=3`: radial upward-downward thermal plume vortices; `f=4`: intensify continuous 3D magnetic flux lines emerging directly from core dynamo coils out into magnetosphere.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **hierro líquido** / *liquid iron* | `[TIER-B]` | La temperatura supera el punto de fusión del metal bajo esta presión. // *The temperature exceeds the melting point of the metal under this pressure.* | 3200–4600K blackbody outer core shell represents molten iron. | Swirl 3,200–4,600 K molten iron-nickel blackbody convective currents across `0.26..0.46` shell (`doc_feat == 0`). |
| `1` | **níquel líquido** / *liquid nickel* | `[TIER-D]` | El núcleo contiene principalmente hierro, acompañado de níquel y elementos ligeros. // *The core contains mainly iron, accompanied by nickel and light elements.* | No Fe-Ni + light-element compositional distinction. | Emit rising light-element (S/O/Si) compositional buoyancy sparkles ascending off inner-core boundary (`doc_feat == 1`). |
| `2` | **geodinamo** / *geodynamo* | `[TIER-B]` | Convección, rotación y electricidad convierten el núcleo en un generador planetario. // *Convection, rotation and electricity turn the core into a planetary generator.* | Animated `helix` Coriolis band pattern (`line 83`) + dipole coupling depicts geodynamo. | Twist helical Y-aligned Taylor convective columns winding toroidal magnetic flux tubes (`doc_feat == 2`). |
| `3` | **convección metálica** / *metallic convection* | `[TIER-B]` | El metal caliente asciende y el más frío desciende, transportando calor. // *The hot metal rises and the colder metal falls, transporting heat.* | Rotating FBM `conv` turbulence inside outer core. | Highlight rapid radial thermal convection plumes transferring inner heat toward `seam_cmb` (`doc_feat == 3`). |
| `4` | **campo magnético** / *magnetic field* | `[TIER-B]` | Las corrientes eléctricas del núcleo producen una envoltura magnética global. // *Electric currents in the core produce a global magnetic envelope.* | `show_magnetosphere` reactivates external dipole lines specifically on `z_idx == 12u` (`line 262`). | Couple internal dynamo coils continuously into external dipole magnetosphere lines (`comp=3621`, `doc_feat == 4`). |

#### `[TIER-B]` Exp `03` `Z13` `inner-core` — *Núcleo interno* / *Inner core* (`vid = 1313` | Feats: `0A / 2B / 0C / 3D`)
- **Metadata:** `raw_metric`: "hierro y níquel sólidos" *(EN: "solid iron and nickel")* · `evidence_class`: `None` · `scene`: `None` · **Altitude/Depth (`phys_range`):** `[6371.0, 5150.0]` km
- **Spoken Macro (ES // EN):** Aunque está extremadamente caliente, la presión colosal mantiene sólido el centro del planeta. // *Although it is extremely hot, immense pressure keeps the planet's center solid.*
- **Rendered Today:** Exposed carved solid inner core sphere (`r = 0.26`, `hero_volumetric.wgsl:86-97`), highlighted `1.5x` on `z_idx == 13u` (`line 90`) in 5400–6000K blackbody with isotropic `cryst` FBM.
- **Audit Gap:** Missing oriented anisotropic HCP iron crystals explaining N-S vs E-W seismic travel speed bias, ~360 GPa extreme pressure indicator, asymmetric East-West core crystallization growth front, and 6,371 km Earth-center crosshair. `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1313`):** Enhance carved inner-core sphere (`r <= 0.26`) via `doc_feat`: `f=0`: solid-phase iron-nickel faceted luster contrasting against fluid outer core; `f=1`: N-S aligned elongated HCP crystal grains + fast polar vs slow equatorial seismic ray overlay; `f=2`: concentric high-pressure stress rings toward 360 GPa center; `f=3`: sparkling dendritic freezing front at `r = 0.25..0.27` with latent heat plumes rising into outer core; `f=4`: glowing 6,371 km geometric center reticle + radial depth caliper.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **hierro sólido** / *solid iron* | `[TIER-B]` | La presión impide que el hierro se funda pese a la temperatura extrema. // *The pressure prevents the iron from melting despite the extreme temperature.* | Glowing carved solid inner sphere (`r=0.26`) explicitly brightened on `z_idx == 13u`. | Highlight crystalline solid-iron metallic specular luster contrasting starkly with liquid outer core (`doc_feat == 0`). |
| `1` | **cristales metálicos** / *metallic crystals* | `[TIER-D]` | La orientación de los cristales influye en cómo viajan las ondas sísmicas. // *The orientation of the crystals influences how seismic waves travel.* | Isotropic FBM noise only; no polar-aligned metallic crystals or seismic anisotropy. | Align elongated N-S hexagonal-close-packed (HCP) iron crystal grains + fast polar vs slow equatorial seismic ray comparison (`doc_feat == 1`). |
| `2` | **presión extrema** / *extreme pressure* | `[TIER-D]` | En el centro, millones de atmósferas comprimen la materia. // *In the center, millions of atmospheres compress the matter.* | No extreme 360 GPa pressure gradient indicator. | Render inward-compressing ~360 GPa hydrostatic pressure gradient rings toward `(0,0,0)` (`doc_feat == 2`). |
| `3` | **crecimiento del núcleo** / *core growth* | `[TIER-D]` | Al enfriarse lentamente la Tierra, parte del núcleo externo se solidifica. // *As the Earth slowly cools, part of the outer core solidifies.* | No solidifying growth boundary releasing light elements into outer core. | Sparkle dendritic iron crystallization freezing front at `r = 0.25..0.27` releasing latent heat/light elements (`doc_feat == 3`). |
| `4` | **centro terrestre** / *Earth's center* | `[TIER-B]` | A unos 6 371 kilómetros bajo la superficie termina toda dirección descendente. // *At about 6,371 kilometers below the surface, there is no farther down.* | Innermost core center directly visible at cutaway vertex (`hero_volumetric.wgsl:86`). | Activate glowing 6,371 km Earth-center geometric crosshair & radial depth caliper at cutaway vertex (`doc_feat == 4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT1-EXP03-Z09-13)</summary>

```markdown
Task: Implement Pick-Up Package PKG-ACT1-EXP03-Z09-13 (Exp 03 Deep Earth — Asthenosphere, Transition Zone (`410-660 km`), Lower Mantle, Outer Geodynamo & Inner Core (`Z09..Z13`)) in HyperCosmosVR.
Prerequisites: Confirm PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING have landed (vid >= 1000u decoder, base_flags cue packing, doc_feat(inst) active).

Target Files:
- `crates/cosmos_render/src/doc_stages.rs` (`L567-611`: dispatch `Z09..Z13` as `vid = 1309..1313` with `submode = 1`, `base_flags |= 0x01`; enable `3621` external dipole coupling cleanly on `Z12 F4`)
- `crates/cosmos_narration/src/resolver.rs` (`L952-961`: leverage `manto, nucleo, geodinamo` + add `astenosfera, fusion parcial, ringwoodita, bridgmanita, conveccion, taylortemp, anisotropia, presion`)
- `shaders/hero_doc_cosmos.wgsl` (`L51-92`: align `seam_410` at `0.78` + `seam_660` at `0.68` + `seam_cmb` at `0.46`; add partial-melt filigree `Z09`, hydrous pockets & seismic ray reflection `Z10`, `D''` domes & cold sinking slabs `Z11`, Taylor columns & light-element buoyancy `Z12`, wiring `doc_feat 0..4`)
- `shaders/hero_volumetric.wgsl` & `shaders/compute_universe.wgsl` (`hero_volumetric.wgsl:84-97` anisotropic N-S HCP iron grain pattern + 360 GPa pressure rings + dendritic freezing front on `Z13`; `compute_universe.wgsl:297-309` convective iron spirals on `Z12`)

Reserved Canonical Shader IDs & Wiring:
- Primary zone vids: `1309` (`Z09 asthenosphere`), `1310` (`Z10 transition`), `1311` (`Z11 lower-mantle`), `1312` (`Z12 outer-core`), `1313` (`Z13 inner-core`)
- Companion/overlay vids: `3629` (`submode=32` Seismic LVZ Wavefront Overlay), `3629` (`submode=33` Olivine->Ringwoodite Crystal Inset), `3629` (`submode=34` Bridgmanite Crystal + `D''` LLSVPs), `3621` (`submode=34` Coupled External Dipole Lines), `3629` (`submode=35` 360 GPa Caliper & Center Crosshair)
- Preserve bits 24..31 submode and bits 0..7 base_flags; branch explicitly on doc_feat(inst) (0..k for spoken features, 0xFF/255 for zone macro), smoothed by doc_cue(inst) and doc_weight(inst) clamped cleanly into HDR AgX range without highlight blowout.
- Wire bilingual Spanish/English narration keywords & SegmentVisualHint::cue_tags in resolver.rs covering: roca dúctil / convección lenta / fusión parcial / pluma ascendente / zona de transición / cambio cristalino / mineral hidratado / discontinuidad 660 km / bridgmanita / manto inferior / capa D'' / hierro fundido / geodinamo / columnas de Taylor / núcleo sólido / anisotropía / 360 GPa / cristalización.

Strict Rendering & Performance Budgets (Mandatory):
- Pure-Black OLED Void Budget: preserve >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout; use bounded local integration in shaders/hero_volumetric.wgsl with ZERO full-screen gray haze or background lift.
- VR 90/120 Hz Frame-Time Budget: cap any new volumetric documentary hero shell at <= 28 raymarch steps inside bounded r <= 0.96 with early exit trans < 0.03, preserving the RTX 5070 DX12 baseline (min = 0.843 ms, mean = 1.863 ms / 536.7 FPS, p99 = 4.661 ms, max = 5.802 ms in artifacts/verification_renders/benchmark_sweep.txt). Interior cutaway surfaces intersect analytically in `hero_volumetric.wgsl:84-127` with zero extra raymarch steps; keep auxiliary overlay capsules <= 20 steps so total frame mean remains near `1.863 ms` (`p99 <= 4.661 ms`).
- Architecture & Repo Hygiene (CONTRIBUTING.md): zero 2D HUD overlays (keep UI purely diegetic 3D); commit zero files > 50 MiB, zero .ttf/.otf font binaries, and zero machine-specific .cargo/config.toml.

Required Verification Gates Before Opening PR:
1. cargo clippy --workspace --all-targets -- -D warnings
2. cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey
3. cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe
4. cargo test -p cosmos_spatial topology
5. cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets
6. Local GPU validation (if Discrete DX12 GPU present): cargo test --workspace -- --test-threads=1 && cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
```

</details>

<a id="pkg-act1-exp04-z00-03"></a>
### PKG-ACT1-EXP04-Z00-03: Exp 04 Solar System — Sun Fusion Core/Wind, Mercury Scarps, Venus Retrograde Fix & Earth Moon/Magnetosphere (`Z00..Z03`)
- **Priority & Rollout:** Phase 3 (Fixes Venus Prograde Spin Contradiction `doc_stages.rs:338` + Exp-04 Earth Missing Moon/Magnetosphere & `z_idx` Bug) · Depends on `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Scope:** Exp `04` (`04`) Zones `Z00..Z03` (`4` zones, `20` features)
- **Current Card Tiers:** Zones `0A / 4B / 0C / 0D` · Features `0A / 10B / 0C / 10D`

**Current State:**
- **Primary Dispatch:** [`doc_stages.rs:173-206,315-344,616-652`](../../crates/cosmos_render/src/doc_stages.rs#L173-L344) emits `Z00 Sun` (`kind=1 vid=1` + `kind=3 vid=4` corona), `Z01 Mercury` (`vid=3`), `Z02 Venus` (`vid=4`, **bug**: positive prograde spin `+input.time_sec * 0.03` at `L338`), and `Z03 Earth` (`kind=0 vid=0`).
- **Exp 04 Earth Isolation Bugs:** [`doc_stages.rs:588-609`](../../crates/cosmos_render/src/doc_stages.rs#L588-L609) emits the Moon sprite and magnetosphere ONLY when `active_experience_id == 3`, leaving Exp 04 Earth bare! Meanwhile [`hero_doc_cosmos.wgsl:25,33,43`](../../shaders/hero_doc_cosmos.wgsl#L25-L43) runs Exp-03 `z_idx == 4/5/6/7` checks directly on Exp 04 planets, and `L131` adds `+time * 0.08` prograde drift to Venus. `doc_feat(inst)` is unread.
- **Card Tier Breakdown (`4` zones, `20` features):** Zones `0A / 4B / 0C / 0D` · Features `0A / 10B / 0C / 10D`.

**Reserved Target IDs:**

| Zone | Slug | Canonical `vid` | Companion `vid` (`3600..3869`) | `submode` (`bits 24..31`) | `base_flags` (`bits 0..7`) | `doc_feat` Slots (`0..7`, `0xFF=macro`) |
|---|---|---:|---|---|---|---:|
| `Z00` | `sun` | `1400` | `3640` (Coronal Streamer / Parker Solar Wind Shell) | `0` (`doc_feat==0` enables octant core cutaway `0x01`) | cue mask (+`0x01` on `F0`) | `0..4` |
| `Z01` | `mercury` | `1401` | `3641` (88-Day Keplerian Phase Sweep Beacon) | `0` (crater/scarp/polar ice) | cue mask | `0..4` |
| `Z02` | `venus` | `1402` | `3642` (Magellan Radar Wedge & Retrograde Axis Ring) | `0` (`-time` retrograde spin fixed!) | cue mask (+`0x01` on `F3`) | `0..4` |
| `Z03` | `earth` | `1403` | `3643` (Orbiting Moon Sphere + Dipole Magnetosphere) | `0` (Exp-03 `z_idx` bleed gated off) | `0x02` (dual Moon) + cue | `0..4` |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `[TIER-B]` Exp `04` `Z00` `sun` — *Sol* / *Sun* (`vid = 1400` | Feats: `0A / 3B / 0C / 2D`)
- **Metadata:** `raw_metric`: "Centro del sistema solar · estrella · fuente de luz y energía" *(EN: "Center of the solar system")* · `evidence_class`: `None` · `scene`: `None` · **`body_layout`:** `id=sun, x=0.025, lane=0, r=92`
- **Spoken Macro (ES // EN):** Es una estrella y concentra casi toda la masa del sistema solar. Su energía hace posibles el clima y la vida en la Tierra. // *It is a star and contains almost all the mass in the solar system. Its energy makes Earth's climate and life possible.*
- **Rendered Today:** `doc_stages.rs:173-194` emits `kind=1, vid=1` convective photosphere (`hero_doc_cosmos.wgsl:107-118`) + `kind=3, vid=4` volumetric corona (`hero_doc_cosmos.wgsl:520-526`) + Keplerian track (`doc_stages.rs:638-651`). `doc_feat` unused.
- **Audit Gap:** Static combined Sun + corona regardless of which solar layer or phenomenon is narrated; zero proton-proton nuclear core cutaway and zero outward Parker-spiral solar wind particle stream (`Compute Mode 10` only orbits flat belt particles at `compute_universe.wgsl:286-296`).
- **3D Zone Blueprint (`vid = 1400`):** Dispatch `vid=1400` (`comp=3640` corona/wind overlay) with `doc_feat` switching: `F0` opens an equatorial octant thermonuclear core cutaway (15M K p-p H->He fusion core + radiative photon random-walk shell + convective columns); `F1` spotlights 5,500 C granulation cell lattice + thermal caliper; `F2` extrudes dipolar magnetic flux tube capsules arching above dark 2,850 K sunspot umbrae; `F3` isolates million-degree coronal streamer arcade while dampening photosphere glare; `F4` couples `submode=10` in `compute_universe.wgsl` to radial Parker-spiral charged particle outflow from coronal holes.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **núcleo** / *core* | `[TIER-D]` | En el centro, la fusión nuclear convierte hidrógeno en helio y libera enormes cantidades de energía. // *At the center, nuclear fusion converts hydrogen into helium and releases enormous amounts of energy.* | No subsurface nuclear core cutaway beneath opaque `vid=1` photosphere (`hero_doc_cosmos.wgsl:107-118`). | Octant cutaway wall (`dot(p, n_cut)`) exposing bright H->He fusion core (`exp(-r*12)`), radiative shell & convective plumes when `doc_feat == 0`. |
| `1` | **fotosfera** / *photosphere* | `[TIER-B]` | La superficie visible tiene una temperatura cercana a cinco mil quinientos grados Celsius. // *The visible surface has a temperature close to five thousand five hundred degrees Celsius.* | Convection granulation cells & blackbody photosphere render continuously (`hero_doc_cosmos.wgsl:109-117`), ignoring `doc_feat`. | Pulse granulation boundaries + 5,500 C blackbody scale reticle when `doc_feat == 1`. |
| `2` | **mancha solar** / *sunspot* | `[TIER-B]` | Regiones más frías y oscuras aparecen donde el campo magnético es especialmente intenso. // *Colder, darker regions appear where the magnetic field is especially intense.* | Cooler magnetic sunspot umbrae rendered (`hero_doc_cosmos.wgsl:112`), ignoring `doc_feat` and lacking 3D flux loops. | Highlight umbra/penumbra pair and arch dipolar magnetic flux capsules above active sunspots when `doc_feat == 2`. |
| `3` | **corona** / *solar corona* | `[TIER-B]` | La atmósfera exterior del Sol alcanza temperaturas de millones de grados y se extiende al espacio. // *The Sun's outer atmosphere reaches temperatures of millions of degrees and extends into space.* | Coronal plasma loops & flares active (`kind=3, vid=4`, `hero_doc_cosmos.wgsl:520-526`), never isolated on `doc_feat`. | Isolate million-K coronal streamer loops in `comp=3640` while attenuating photosphere glare (`doc_feat == 3`). |
| `4` | **viento solar** / *solar wind* | `[TIER-D]` | Un flujo de partículas cargadas sale del Sol y llena todo el sistema planetario. // *A stream of charged particles leaves the Sun and fills the entire planetary system.* | Compute Mode 10 (`compute_universe.wgsl:286-296`) only orbits flat belt particles; zero radial solar wind stream. | Switch Compute Mode 10 outflow emitter on `Z00`/`doc_feat == 4` to stream outward Parker-spiral protons/electrons. |

#### `[TIER-B]` Exp `04` `Z01` `mercury` — *Mercurio* / *Mercury* (`vid = 1401` | Feats: `0A / 3B / 0C / 2D`)
- **Metadata:** `raw_metric`: "Planeta rocoso" *(EN: "rocky planet")* · `evidence_class`: `None` · `scene`: `None` · **`body_layout`:** `id=mercury, x=0.13, lane=-0.12, r=8`
- **Spoken Macro (ES // EN):** Es el planeta más cercano al Sol y el más pequeño de los ocho planetas. // *It is the closest planet to the Sun and the smallest of the eight planets.*
- **Rendered Today:** `doc_stages.rs:315-344` emits `kind=1, vid=3` cratered basalt globe (`hero_doc_cosmos.wgsl:120-127`) + Keplerian inner orbit track (`kind=2, vid=4`, `oi == 0` highlighted via `oi == z_idx - 1u` at `hero_doc_cosmos.wgsl:350,367`).
- **Audit Gap:** Generic crater noise omits curved thrust lobate scarps (despite `hero_doc_cosmos.wgsl:121` comment) and permanently shadowed polar crater water ice; `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1401`):** Dispatch `vid=1401` (`comp=3641` orbital phase sweep) with `doc_feat`: `F0` emphasizes Caloris-style multi-ring impact basin + ejecta rays; `F1` carves curved `sd_capsule` planetary contraction lobate scarps with sharp cliff shadows; `F2` splits airless day (+430 C amber) vs cryogenic night (-180 C indigo) contrast cue; `F3` illuminates bright water-ice frost deposits inside permanently shadowed polar craters (`abs(p.y) > 0.85`); `F4` animates 88-day orbital phase beacon sweeping `oi == 0` Keplerian track.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **cráter** / *crater* | `[TIER-B]` | Su superficie conserva una larga historia de impactos, parecida a la de la Luna. // *Its surface preserves a long history of impacts, similar to that of the Moon.* | Procedural impact `crater_rim` rings appear across basalt regolith (`hero_doc_cosmos.wgsl:122-125`), ignoring `doc_feat`. | Highlight multi-ringed Caloris impact basin + bright ray ejecta (`doc_feat == 0`). |
| `1` | **acantilado** / *scarp* | `[TIER-D]` | Al enfriarse y contraerse, el planeta formó enormes escarpes que atraviesan el terreno. // *As the planet cooled and contracted, it formed huge scarps that cut through the terrain.* | `hero_doc_cosmos.wgsl:121` comment mentions scarps, yet `L122-126` evaluates crater noise only; zero fault-cliff lineaments. | Carve curved `sd_capsule` lobate thrust-fault scarps slicing across older craters (`doc_feat == 1`). |
| `2` | **día extremo** / *extreme day* | `[TIER-B]` | Sin una atmósfera densa, la temperatura cambia radicalmente entre el día y la noche. // *Without a dense atmosphere, the temperature changes radically between day and night.* | Airless sharp day/night terminator (`ndotl * 1.15 + 0.02`, zero Fresnel atmo `L126`), ignoring `doc_feat`. | Overlay extreme day (+430 C amber) vs night (-180 C deep indigo) thermal terminator readout (`doc_feat == 2`). |
| `3` | **hielo polar** / *polar ice* | `[TIER-D]` | Cráteres que nunca reciben luz solar pueden conservar hielo de agua cerca de los polos. // *Craters that never receive sunlight can preserve water ice near the poles.* | No permanently shadowed polar crater water-ice deposits near poles (`\|p.y\| > 0.85`). | Place specular white-cyan water-ice frost patches tucked inside poleward shadowed crater floors (`doc_feat == 3`). |
| `4` | **órbita rápida** / *fast orbit* | `[TIER-B]` | Mercurio completa una vuelta alrededor del Sol en solo ochenta y ocho días terrestres. // *Mercury completes one revolution around the Sun in just eighty-eight Earth days.* | Keplerian hairline `vid=4` boosts Mercury track `oi == 0` when `z_idx == 1` (`hero_doc_cosmos.wgsl:350,367`), ignoring `doc_feat`. | Animate bright 88-day orbital velocity arc pulsing along `oi == 0` ellipse (`doc_feat == 4`). |

#### `[TIER-B]` Exp `04` `Z02` `venus` — *Venus* / *Venus* (`vid = 1402` | Feats: `0A / 1B / 0C / 4D`)
- **Metadata:** `raw_metric`: "Planeta rocoso" *(EN: "rocky planet")* · `evidence_class`: `None` · `scene`: `None` · **`body_layout`:** `id=venus, x=0.21, lane=0.13, r=18`
- **Spoken Macro (ES // EN):** Aunque no es el más cercano al Sol, es el planeta más caliente por su intenso efecto invernadero. // *Although it is not the closest to the Sun, it is the hottest planet due to its intense greenhouse effect.*
- **Rendered Today:** `doc_stages.rs:317,338` emits `kind=1, vid=4` (`hero_doc_cosmos.wgsl:129-136`). **Active physics contradiction**: both `doc_stages.rs:338` (`phys_a.x = +input.time_sec * 0.03`) and `hero_doc_cosmos.wgsl:131` (`+time * 0.08`) spin prograde instead of retrograde!
- **Audit Gap:** Dense sulfuric acid clouds obscure all geology and atmospheric thermodynamics while rotating in the wrong (prograde) direction during retrograde rotation narration (`F2`).
- **3D Zone Blueprint (`vid = 1402`):** Dispatch `vid=1402` (`comp=3642` radar cutaway / rotation vector ring) and **negate spin sign (`-time`)** in `doc_stages.rs:338` & `hero_doc_cosmos.wgsl:131`. `doc_feat`: `F0` isolates banded sulfuric acid aerosol shroud; `F1` opens cross-section heat-trap arrow loop showing runaway CO2 infrared retention; `F2` overlays clockwise retrograde spin arrow vs prograde ecliptic orbital direction; `F3` slices radar-false-color wedge exposing Maxwell Montes & basalt volcanic shields; `F4` animates radial 90-bar surface barometric pressure compression shell.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **nubes de ácido** / *acid clouds* | `[TIER-B]` | Una capa global de nubes de ácido sulfúrico oculta casi por completo la superficie. // *A global layer of sulfuric acid clouds almost completely obscures the surface.* | Sulfuric acid chevron cloud shroud + dense amber limb render continuously (`hero_doc_cosmos.wgsl:131-135`). | Isolate upper/lower sulfuric acid cloud decks with golden aerosol shimmer (`doc_feat == 0`). |
| `1` | **efecto invernadero** / *greenhouse effect* | `[TIER-D]` | Su atmósfera rica en dióxido de carbono retiene calor de forma extrema. // *Its carbon dioxide-rich atmosphere retains extreme heat.* | No infrared heat-trapping loop or CO2 greenhouse thermal diagram. | Cut open greenhouse radiative window showing incoming sunlight vs trapped IR bouncing under clouds (`doc_feat == 1`). |
| `2` | **rotación retrógrada** / *retrograde rotation* | `[TIER-D]` | Venus gira lentamente en sentido contrario al de la mayoría de los planetas. // *Venus rotates slowly in the opposite direction to most planets.* | **Active contradiction**: `doc_stages.rs:338` & `hero_doc_cosmos.wgsl:131` spin positive prograde (`+time * 0.03` / `+0.08`), opposite retrograde speech. | Fix spin velocity sign (`-time`) and render twin counter-rotating ecliptic vs spin axis arrows (`doc_feat == 2`). |
| `3` | **volcán** / *volcano* | `[TIER-D]` | Grandes llanuras volcánicas y montañas cubren buena parte de la superficie. // *Large volcanic plains and mountains cover much of the surface.* | Opaque cloud shader provides zero radar cutaway or volcanic mountain shields. | Open Magellan-style radar false-color cutaway wedge revealing Maxwell Montes & shield volcanoes (`doc_feat == 3`). |
| `4` | **presión** / *pressure* | `[TIER-D]` | En el suelo, la presión atmosférica es unas noventa veces mayor que en la superficie terrestre. // *At Venus's surface, atmospheric pressure is about ninety times greater than at Earth's surface.* | No 90-bar surface atmospheric pressure visualization. | Compress radial barometric pressure gradient shells downward onto incandescent rocky floor (`doc_feat == 4`). |

#### `[TIER-B]` Exp `04` `Z03` `earth` — *Tierra* / *Earth* (`vid = 1403` | Feats: `0A / 3B / 0C / 2D`)
- **Metadata:** `raw_metric`: "Planeta rocoso · punto de continuidad · estás aquí" *(EN: "Rocky planet · point of continuity")* · `evidence_class`: `None` · `scene`: `None` · **`body_layout`:** `id=earth, x=0.295, lane=-0.08, r=21`
- **Spoken Macro (ES // EN):** Aquí termina la exploración de la atmósfera de la app anterior y comienza el mapa completo del sistema solar. // *Here the exploration of the atmosphere in the previous app ends and the complete map of the solar system begins.*
- **Rendered Today:** `doc_stages.rs:195-206` emits bare `kind=0, vid=0` (`shade_doc_earth_surface`, `hero_doc_cosmos.wgsl:4-49`). **Exp 03 bug**: `hero_doc_cosmos.wgsl:25,33,43` read `z_idx == 4,5,6,7` (Exp 03 layers!). Both Moon sprite (`doc_stages.rs:600-608`) & magnetosphere (`doc_stages.rs:588-596`) are gated to Exp 03 ONLY.
- **Audit Gap:** Earth omits Moon satellite & dipolar magnetosphere completely in Exp 04, and `shade_doc_earth_surface` triggers Exp-03-only zone highlights when cycling Exp 04 planets.
- **3D Zone Blueprint (`vid = 1403`):** Dispatch `vid=1403` + companion `comp=3643` (Moon sphere + dipolar magnetosphere shell decoupled from `active_experience_id == 3` gate at `doc_stages.rs:588-609`, with Exp-03 `z_idx` checks in `hero_doc_cosmos.wgsl:25,33,43` gated strictly on `exp == 3u`!). `doc_feat`: `F0` boosts Rayleigh/stratospheric rim; `F1` illuminates 70% liquid ocean + thermohaline gyres; `F2` sharpens continental crust shields; `F3` spotlights orbiting Moon (`comp=3643`) + tidal ocean bulge ellipsoid; `F4` pulses dipole magnetic shield deflecting solar wind.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **atmósfera** / *atmosphere* | `[TIER-B]` | Una envoltura fina regula la temperatura, transporta agua y permite respirar. // *A thin envelope regulates temperature, transports water and allows breathing.* | Rayleigh blue atmospheric limb present (`hero_doc_cosmos.wgsl:44`), though `ozone_glow` (`L43`) checks Exp-03 `z_idx == 4u`. | Gate Exp-03 `z_idx` checks properly and boost Rayleigh thin atmospheric envelope on `doc_feat == 0`. |
| `1` | **océano** / *ocean* | `[TIER-B]` | El agua líquida cubre cerca de siete décimas de la superficie y redistribuye calor por el planeta. // *Liquid water covers about 70 percent of the surface and redistributes heat around the planet.* | Deep/coastal ocean albedo + specular glint (`L24, L36`), while `ocean_trench` (`L25`) checks Exp-03 `z_idx == 6/7`. | Illuminate 70% global liquid ocean + warm/cold surface heat currents (`doc_feat == 1`). |
| `2` | **continente** / *continent* | `[TIER-B]` | La corteza continental conserva montañas, ríos, suelos y una larga historia geológica. // *The continental crust preserves mountains, rivers, soils and a long geological history.* | Procedural continental crust, coastlines & mountain relief (`L19-21, L26-27`). | Sharpen continental tectonic shields & mountain belts (`doc_feat == 2`). |
| `3` | **Luna** / *Moon* | `[TIER-D]` | Nuestro satélite estabiliza parcialmente la inclinación terrestre y produce la mayor parte de las mareas. // *Our satellite partially stabilizes the Earth's tilt and produces most of the tides.* | Moon sprite billboard (`doc_stages.rs:600-608`) is ONLY emitted for Exp 03, absent in Exp 04 `body_idx == 3`. | Spawn orbiting Moon companion (`comp=3643`) + ocean tidal bulge axes when `doc_feat == 3`. |
| `4` | **magnetosfera** / *magnetosphere* | `[TIER-D]` | El campo magnético desvía gran parte del viento solar y protege la atmósfera. // *The magnetic field deflects much of the solar wind and protects the atmosphere.* | Magnetosphere volume (`doc_stages.rs:588-596`) is Exp-03-only; zero magnetic shield in Exp 04 Earth. | Spawn dipole field-line shell (`comp=3643`) + dayside bow shock deflection (`doc_feat == 4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT1-EXP04-Z00-03)</summary>

```markdown
Task: Implement Pick-Up Package PKG-ACT1-EXP04-Z00-03 (Exp 04 Solar System — Sun Fusion Core/Wind, Mercury Scarps, Venus Retrograde Fix & Earth Moon/Magnetosphere (`Z00..Z03`)) in HyperCosmosVR.
Prerequisites: Confirm PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING have landed (vid >= 1000u decoder, base_flags cue packing, doc_feat(inst) active).

Target Files:
- `crates/cosmos_render/src/doc_stages.rs` (`L173-206, 315-344, 588-609`: dispatch `Z00..Z03` -> `vid = 1400..1403`, fix Venus `phys_a.x` spin sign `-input.time_sec * 0.03` at `L338`, emit Moon + magnetosphere companions `3643` for Exp 04 `body_idx == 3`)
- `crates/cosmos_narration/src/resolver.rs` (`L952-961`: leverage `sol, fusion, mercurio, venus, tierra, luna` + add `fotosfera, mancha solar, viento solar, acantilado, retrograda, invernadero, presion, magnetosfera`)
- `shaders/hero_doc_cosmos.wgsl` (`L4-49, 107-136, 350-367, 520-526`: gate Exp-03 `z_idx==4/5/6/7` checks strictly on `exp==3u`, negate Venus spin `-time*0.08`, add Sun H->He core cutaway + magnetic loops, Mercury thrust scarps & shadowed polar ice, Venus IR loop/radar slice/90-bar shell, Earth Moon tidal bulge & dipole field on `doc_feat 0..4`)
- `shaders/compute_universe.wgsl` (`L285-296`: switch Mode 10 to radial Parker-spiral solar wind when `Z00` / `doc_feat == 4` is active)

Reserved Canonical Shader IDs & Wiring:
- Primary zone vids: `1400` (`Z00 sun`), `1401` (`Z01 mercury`), `1402` (`Z02 venus`), `1403` (`Z03 earth`)
- Companion/overlay vids: `3640` (Coronal Streamer / Parker Solar Wind Shell), `3641` (88-Day Keplerian Phase Sweep Beacon), `3642` (Magellan Radar Wedge & Retrograde Axis Ring), `3643` (Orbiting Moon Sphere + Dipole Magnetosphere)
- Preserve bits 24..31 submode and bits 0..7 base_flags; branch explicitly on doc_feat(inst) (0..k for spoken features, 0xFF/255 for zone macro), smoothed by doc_cue(inst) and doc_weight(inst) clamped cleanly into HDR AgX range without highlight blowout.
- Wire bilingual Spanish/English narration keywords & SegmentVisualHint::cue_tags in resolver.rs covering: núcleo fusión / fotosfera / mancha solar / corona / viento solar / cráter / acantilado / día extremo / hielo polar / órbita rápida / nubes de ácido / efecto invernadero / rotación retrógrada / volcán / presión / atmósfera / océano / continente / Luna / magnetosfera.

Strict Rendering & Performance Budgets (Mandatory):
- Pure-Black OLED Void Budget: preserve >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout; use bounded local integration in shaders/hero_volumetric.wgsl with ZERO full-screen gray haze or background lift.
- VR 90/120 Hz Frame-Time Budget: cap any new volumetric documentary hero shell at <= 28 raymarch steps inside bounded r <= 0.96 with early exit trans < 0.03, preserving the RTX 5070 DX12 baseline (min = 0.843 ms, mean = 1.863 ms / 536.7 FPS, p99 = 4.661 ms, max = 5.802 ms in artifacts/verification_renders/benchmark_sweep.txt). Bound coronal/magnetosphere shell raymarching to `<= 26` steps inside `r <= 0.96` with early exit `trans < 0.03`; maintain `>= 50.91%` `#000000` background void across inner system flythrough.
- Architecture & Repo Hygiene (CONTRIBUTING.md): zero 2D HUD overlays (keep UI purely diegetic 3D); commit zero files > 50 MiB, zero .ttf/.otf font binaries, and zero machine-specific .cargo/config.toml.

Required Verification Gates Before Opening PR:
1. cargo clippy --workspace --all-targets -- -D warnings
2. cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey
3. cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe
4. cargo test -p cosmos_spatial topology
5. cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets
6. Local GPU validation (if Discrete DX12 GPU present): cargo test --workspace -- --test-threads=1 && cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
```

</details>

<a id="pkg-act1-exp04-z04-07"></a>
### PKG-ACT1-EXP04-Z04-07: Exp 04 Solar System — Mars Relief/Moons, Asteroid Belt Kirkwood Decouple, Jupiter Galilean/Europa & Saturn Moons (`Z04..Z07`)
- **Priority & Rollout:** Phase 3 (Fixes `Z05` Saturn/Mercury Reuse `[TIER-C F4]` & Unlocks Unused `vid=10` Europa for Jupiter) · Depends on `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Scope:** Exp `04` (`04`) Zones `Z04..Z07` (`4` zones, `20` features)
- **Current Card Tiers:** Zones `0A / 4B / 0C / 0D` · Features `0A / 9B / 1C / 10D`

**Current State:**
- **Primary Dispatch:** [`doc_stages.rs:208-288,318-342`](../../crates/cosmos_render/src/doc_stages.rs#L208-L342) emits `Z04 Mars` (`vid=5`, zero Phobos/Deimos), `Z05 Asteroid Belt` (**placeholder bug**: central dwarf reuses `vid=3` Mercury `L209`, ring `kind=2 vid=1` renders Saturn `cassini_gap`/`encke_gap` `hero_doc_cosmos.wgsl:395-396`, `7` uniform matte brown sprites `L230,253`), `Z06 Jupiter` (`vid=6`, zero Galilean moons despite [`hero_doc_cosmos.wgsl:187-194`](../../shaders/hero_doc_cosmos.wgsl#L187-L194) having a bespoke `vid=10` Europa shader!), and `Z07 Saturn` (`vid=2` planet + `kind=2 vid=2` rings, zero Titan/Enceladus). `doc_feat(inst)` is unread.
- **Card Tier Breakdown (`4` zones, `20` features):** Zones `0A / 4B / 0C / 0D` · Features `0A / 9B / 1C / 10D` (`F4` Kirkwood gap is `[TIER-C]`).

**Reserved Target IDs:**

| Zone | Slug | Canonical `vid` | Companion `vid` (`3600..3869`) | `submode` (`bits 24..31`) | `base_flags` (`bits 0..7`) | `doc_feat` Slots (`0..7`, `0xFF=macro`) |
|---|---|---:|---|---|---|---:|
| `Z04` | `mars` | `1404` | `3644` (Rover Beacon, Phobos & Deimos Irregular Moons) | `0` | cue mask | `0..4` |
| `Z05` | `asteroid-belt` | `1405` | `3645` (Vesta Ellipsoid, Fe-Ni Psyche Shard, Orbital Family) | `0` (Kirkwood gaps) | `0x02` + cue mask | `0..4` |
| `Z06` | `jupiter` | `1406` | `3646` (Galilean Quartet incl. `vid=10` Europa Ocean Wedge) | `0` (`1:2:4` Laplace tracks) | `0x02` + cue mask | `0..4` |
| `Z07` | `saturn` | `1407` | `3647` (Orange Titan w/ Hydrocarbon Sea & Plume Enceladus) | `0` (B-ring / Cassini) | `0x02` + cue mask | `0..4` |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `[TIER-B]` Exp `04` `Z04` `mars` — *Marte* / *Mars* (`vid = 1404` | Feats: `0A / 2B / 0C / 3D`)
- **Metadata:** `raw_metric`: "Planeta rocoso" *(EN: "rocky planet")* · `evidence_class`: `None` · `scene`: `None` · **`body_layout`:** `id=mars, x=0.385, lane=0.15, r=14`
- **Spoken Macro (ES // EN):** Es un desierto frío con volcanes gigantes, cañones profundos y señales de antiguos ríos y lagos. // *It is a cold desert with giant volcanoes, deep canyons, and signs of ancient rivers and lakes.*
- **Rendered Today:** `doc_stages.rs:318,333-342` emits `kind=1, vid=5` (`hero_doc_cosmos.wgsl:138-148`) showing rust basalt regolith, Valles Marineris equatorial scar, and polar ice caps.
- **Audit Gap:** Missing Olympus Mons 20+ km shield edifice, robotic Mars rover surface beacon, and irregular orbiting moons Phobos & Deimos; `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1404`):** Dispatch `vid=1404` + companions `comp=3644` (Mars rover landing beacon + Phobos & Deimos irregular moon sprites): `F0` pulses seasonal H2O/CO2 polar cap halo & sublimation front; `F1` sculpts raised northwest shield-volcano dome & summit caldera rim for Olympus Mons (>20 km contour); `F2` highlights equatorial Valles Marineris chasm floor + paleolake deltas; `F3` illuminates gold robotic landing crosshair + rover traverse arc (`comp=3644`); `F4` orbits twin Phobos & Deimos rocky moonlets (`comp=3644`).

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **casquete polar** / *polar cap* | `[TIER-B]` | Hielo de agua y dióxido de carbono cambia con las estaciones marcianas. // *Water ice and carbon dioxide change with the Martian seasons.* | Bright white water/CO2 polar caps render continuously (`hero_doc_cosmos.wgsl:140,145`), ignoring `doc_feat`. | Animate seasonal CO2 dry-ice frost expansion/retreat ring (`doc_feat == 0`). |
| `1` | **Olympus Mons** / *Olympus Mons* | `[TIER-D]` | El volcán más alto conocido del sistema solar se eleva más de veinte kilómetros. // *The tallest known volcano in the solar system rises more than twenty kilometers.* | No shield-volcano edifice or summit caldera (`Tharsis` / `Olympus Mons` absent from `L138-147`). | Sculpt raised basalt shield dome + 20 km elevation contour ring in NW Tharsis hemisphere (`doc_feat == 1`). |
| `2` | **Valles Marineris** / *Valles Marineris* | `[TIER-B]` | Un sistema de cañones recorre miles de kilómetros junto al ecuador. // *A system of canyons runs thousands of kilometers along the equator.* | Dark equatorial canyon scar (`Valles Marineris`, `hero_doc_cosmos.wgsl:141,144`), ignoring `doc_feat`. | Illuminate rift chasm walls & branching ancient river delta outflow channels (`doc_feat == 2`). |
| `3` | **rover** / *rover* | `[TIER-D]` | Laboratorios móviles estudian rocas, clima y antiguos ambientes potencialmente habitables. // *Mobile laboratories study rocks, climate and ancient potentially habitable environments.* | No Mars rover beacon / surface exploration target on rocky plains. | Place glowing telemetry landing reticle + tiny rover track glyph on Jezero/Gale meridian (`doc_feat == 3`). |
| `4` | **Fobos y Deimos** / *Phobos and Deimos* | `[TIER-D]` | Dos pequeñas lunas irregulares acompañan a Marte. // *Two small irregular moons accompany Mars.* | No orbiting Phobos and Deimos irregular asteroid-like moon sprites. | Orbit two irregular rocky sprites (`comp=3644`) on close/outer prograde equatorial tracks (`doc_feat == 4`). |

#### `[TIER-B]` Exp `04` `Z05` `asteroid-belt` — *Cinturón de asteroides* / *asteroid belt* (`vid = 1405` | Feats: `0A / 1B / 1C / 3D`)
- **Metadata:** `raw_metric`: "Aprox. 2,1–3,3 UA del Sol · Región de cuerpos rocosos" *(EN: "Approx. 2.1–3.3 AU from the Sun")* · `evidence_class`: `None` · `scene`: `None` · **`body_layout`:** `id=ceres, x=0.475, lane=-0.13, r=5`
- **Spoken Macro (ES // EN):** Es una región enorme y poco densa: una nave puede atravesarla sin acercarse a ningún asteroide grande. // *It is a huge and sparse region: a ship can pass through it without approaching any large asteroid.*
- **Rendered Today:** `doc_stages.rs:208-264` emits `body_idx == 5`: central dwarf reuses `vid=3` (Mercury!), tilted debris ring uses `kind=2, vid=1` (hardcoded Saturn `cassini_gap` at `r=0.71` & `encke_gap` at `r=0.88`, `hero_doc_cosmos.wgsl:395-396`), plus `7` uniform matte brown `kind_f=2` rock sprites (`doc_stages.rs:230,253`).
- **Audit Gap:** Reuses Mercury (`vid=3`) and Saturn's Cassini/Encke gaps (`vid=1`) for Asteroid Belt (`F4` `[TIER-C]`) while omitting Ceres Occator bright carbonate salt spots, Vesta's south-polar Rheasilvia basin, metallic Fe-Ni Psyche shard, and clustered collisional asteroid families.
- **3D Zone Blueprint (`vid = 1405`):** Dispatch `vid=1405` + companions `comp=3645` (Vesta ellipsoid, Fe-Ni metallic Psyche fragment, clustered orbital family sprites, true 3:1/5:2/2:1 Kirkwood resonance belt). `F0` upgrades central Ceres (`vid=1405`) with Occator bright carbonate salt domes + subsurface water-ice layer hint; `F1` spawns triaxial irregular Vesta (`comp=3645`) with deep south-polar Rheasilvia crater; `F2` spawns high-specular brushed Fe-Ni core fragment (`comp=3645`); `F3` concentrates a collisional fragment arc; `F4` switches belt ring shader away from Saturn `cassini_gap` to true Jupiter 3:1 / 5:2 / 2:1 Kirkwood gaps (`compute_universe` Mode 10 resonance carving).

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **Ceres** / *Ceres* | `[TIER-B]` | El mayor objeto del cinturón es un planeta enano con sales y abundante hielo de agua. // *The largest object in the belt is a dwarf planet with salt and abundant water ice.* | Central rocky cratered sphere (`vid=3` Mercury) acts as Ceres (`doc_stages.rs:209`), lacking Occator carbonate salt spots. | Upgrade Ceres globe with bright Occator carbonate salt deposits + thin briny ice crust glint (`doc_feat == 0`). |
| `1` | **Vesta** / *Vesta* | `[TIER-D]` | Un gran asteroide rocoso conserva una enorme cuenca de impacto cerca de su polo sur. // *A large rocky asteroid preserves a huge impact basin near its south pole.* | No Vesta body or south-polar Rheasilvia impact basin. | Spawn triaxial cratered Vesta companion (`comp=3645`) showing oversized south-polar bowl & central peak (`doc_feat == 1`). |
| `2` | **asteroide metálico** / *metallic asteroid* | `[TIER-D]` | Algunos fragmentos exponen hierro y níquel que antes estuvieron dentro de cuerpos mayores. // *Some fragments expose iron and nickel that were once inside larger bodies.* | All `7` rock sprites share matte brown dust shading (`doc_stages.rs:230`); zero exposed metallic Fe-Ni shard. | Render specular metallic Fe-Ni core asteroid (`comp=3645` brushed nickel-iron facet glint, `doc_feat == 2`). |
| `3` | **familia de asteroides** / *asteroid family* | `[TIER-D]` | Una colisión puede repartir miles de fragmentos por órbitas parecidas. // *A collision can spread thousands of fragments through similar orbits.* | Uniformly spaced rock sprites (`j * TAU / 7`, `doc_stages.rs:253`) show zero collisional family clustering. | Cluster orbital family fragments along a shared high-density post-collision arc (`doc_feat == 3`). |
| `4` | **hueco de Kirkwood** / *Kirkwood gap* | `[TIER-C]` | La gravedad repetida de Júpiter vacía ciertas órbitas resonantes. // *Jupiter's repeated gravity empties certain resonant orbits.* | Ring shader `kind=2, vid=1` renders Saturn's `cassini_gap` (`r=0.71`) & `encke_gap` (`r=0.88`, `hero_doc_cosmos.wgsl:395-396`). | Replace Saturn gaps with Jovian 3:1 / 5:2 / 2:1 Kirkwood resonant voids + Jupiter resonance vector lines (`doc_feat == 4`). |

#### `[TIER-B]` Exp `04` `Z06` `jupiter` — *Júpiter* / *Jupiter* (`vid = 1406` | Feats: `0A / 3B / 0C / 2D`)
- **Metadata:** `raw_metric`: "Gigante gaseoso" *(EN: "gas giant")* · `evidence_class`: `None` · `scene`: `None` · **`body_layout`:** `id=jupiter, x=0.575, lane=0.05, r=44`
- **Spoken Macro (ES // EN):** El planeta más grande reúne más masa que todos los demás planetas juntos. // *The largest planet packs more mass than all the other planets combined.*
- **Rendered Today:** `doc_stages.rs:319,333-342` emits `kind=1, vid=6` (`hero_doc_cosmos.wgsl:150-161`) with shear cloud bands, crimson Great Red Spot, and polar indigo aurora.
- **Audit Gap:** Zero Galilean moons (Io, Europa, Ganymede, Callisto) orbit Jupiter despite cracked-ice `vid=10` Europa shader already existing in `hero_doc_cosmos.wgsl:187-194` (unused for Jupiter!); `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1406`):** Dispatch `vid=1406` + companions `comp=3646` (Galilean moon quartet incl. `vid=10` cracked Europa cutout). `F0` intensifies UV/indigo polar auroral ovals + Io magnetic flux footprint tether; `F1` animates counter-flowing zonal cloud band shear arrows; `F2` zooms/frames Great Red Spot anticyclone swirl; `F3` spawns/spotlights `vid=10` Europa sphere with crust cutaway revealing global saltwater ocean; `F4` displays all 4 differentiated Galilean moons (volcanic sulfur Io, icy Europa, grooved Ganymede, heavily cratered Callisto) in `1:2:4` Laplace resonance.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **aurora polar** / *polar aurora* | `[TIER-B]` | Su enorme magnetosfera acelera partículas y crea auroras mucho más potentes que las terrestres. // *Its enormous magnetosphere accelerates particles and creates auroras much more powerful than those on Earth.* | Indigo polar auroral oval renders continuously (`pole_aur`, `hero_doc_cosmos.wgsl:159`). | Pulse polar auroral ovals + magnetic flux footprint tether connecting to Io (`doc_feat == 0`). |
| `1` | **bandas de nubes** / *cloud bands* | `[TIER-B]` | Corrientes opuestas forman franjas claras y oscuras alrededor del planeta. // *Opposing currents form light and dark stripes around the planet.* | Curl-sheared ochre/cream/terracotta zonal cloud belts (`L152-155`). | Overlay opposing eastward/westward jet velocity chevrons along belt boundaries (`doc_feat == 1`). |
| `2` | **Gran Mancha Roja** / *Great Red Spot* | `[TIER-B]` | Una tormenta anticiclónica gigantesca gira desde hace siglos. // *A gigantic anticyclonic storm has been spinning for centuries.* | Anticyclonic crimson Great Red Spot oval (`L156-158`). | Frame spinning vortex spiral eye + historical scale reticle around Great Red Spot (`doc_feat == 2`). |
| `3` | **Europa** / *Europa* | `[TIER-D]` | Bajo una corteza de hielo podría existir un océano global de agua salada. // *Under a crust of ice there could exist a global ocean of salt water.* | Europa cracked-ice shader `vid=10` (`hero_doc_cosmos.wgsl:187-194`) is NEVER spawned at Jupiter (`only misused on Z10 Kuiper`). | Spawn dedicated `vid=10` / `comp=3646` Europa companion with ice-crust wedge exposing subsurface ocean (`doc_feat == 3`). |
| `4` | **lunas galileanas** / *Galilean moons* | `[TIER-D]` | Ío, Europa, Ganímedes y Calisto son cuatro mundos muy diferentes. // *Io, Europa, Ganymede and Callisto are four very different worlds.* | Zero Io, Europa, Ganymede, Callisto orbital companions (`doc_stages.rs:319`). | Spawn 4 differentiated orbiting Galilean moon spheres + `1:2:4` Laplace resonance tracks (`doc_feat == 4`). |

#### `[TIER-B]` Exp `04` `Z07` `saturn` — *Saturno* / *Saturn* (`vid = 1407` | Feats: `0A / 3B / 0C / 2D`)
- **Metadata:** `raw_metric`: "Gigante gaseoso" *(EN: "gas giant")* · `evidence_class`: `None` · `scene`: `None` · **`body_layout`:** `id=saturn, x=0.68, lane=-0.11, r=32`
- **Spoken Macro (ES // EN):** Sus anillos parecen sólidos desde lejos, pero están formados por incontables partículas de hielo y roca. // *Its rings appear solid from afar, but they are made of countless particles of ice and rock.*
- **Rendered Today:** `doc_stages.rs:266-288` emits `kind=1, vid=2` Saturn (`hero_doc_cosmos.wgsl:231-243`, north polar hexagon) + `kind=2, vid=2` translucent ring disk (`L392-405`, B-ring & `cassini_gap` at `r=0.71`).
- **Audit Gap:** Zero Titan or Enceladus moons orbit Saturn, and `doc_feat` never isolates polar hexagon vs B-ring vs Cassini Division.
- **3D Zone Blueprint (`vid = 1407`):** Dispatch `vid=1407` + companions `comp=3647` (orange-haze Titan + cryo-active Enceladus). `F0` rotates/highlights cyan 6-sided north polar jet hexagon (`L235-240`); `F1` boosts B-ring radial spiral density waves & spokelike dust shadows inside `0.50 < r < 0.69`; `F2` illuminates cleared walls flanking `cassini_gap` (`r=0.71`); `F3` spawns hazy orange photochemical Titan companion with dark polar specular Kraken Mare hydrocarbon sea; `F4` spawns icy Enceladus companion (`comp=3647`) venting bright south-polar water-ice geyser plumes feeding E ring.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **hexágono polar** / *polar hexagon* | `[TIER-B]` | Una corriente atmosférica forma una figura de seis lados alrededor del polo norte. // *An atmospheric current forms a six-sided figure around the north pole.* | Cyan north-polar 6-sided jet hexagon (`hero_doc_cosmos.wgsl:235-240`). | Rotate & sharpen 6-node jet stream polygon over Saturn's north pole (`doc_feat == 0`). |
| `1` | **anillo B** / *B ring* | `[TIER-B]` | La sección más brillante y densa contiene ondas, huecos y estructuras temporales. // *The brightest and densest section contains waves, gaps, and temporary structures.* | Bright dense inner B-ring zone (`r < 0.71`, `L393-404`). | Highlight radial spiral density waves & icy ring-particle glint inside `0.50..0.69` (`doc_feat == 1`). |
| `2` | **división de Cassini** / *Cassini division* | `[TIER-B]` | Una amplia región relativamente vacía separa los anillos A y B. // *A large, relatively empty region separates the A and B rings.* | Explicit `cassini_gap` empty division (`r=0.71`, `L395`). | Highlight empty Cassini corridor rimmed by resonant edge waves (`doc_feat == 2`). |
| `3` | **Titán** / *Titan* | `[TIER-D]` | Esta luna posee una atmósfera densa y mares de metano y etano líquidos. // *This moon has a dense atmosphere and seas of liquid methane and ethane.* | No Titan satellite, orange photochemical atmosphere, or liquid methane seas. | Spawn dense orange-haze Titan companion (`comp=3647`) with dark specular Kraken Mare hydrocarbon sea (`doc_feat == 3`). |
| `4` | **Encélado** / *Enceladus* | `[TIER-D]` | Géiseres de hielo lanzan al espacio material de un océano subterráneo. // *Ice geysers launch material from an underground ocean into space.* | No Enceladus icy satellite or south-polar ice geyser plumes (`vid=10` unused here). | Spawn grooved white Enceladus companion (`comp=3647`) with south-polar tiger-stripe cryo-plume cone (`doc_feat == 4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT1-EXP04-Z04-07)</summary>

```markdown
Task: Implement Pick-Up Package PKG-ACT1-EXP04-Z04-07 (Exp 04 Solar System — Mars Relief/Moons, Asteroid Belt Kirkwood Decouple, Jupiter Galilean/Europa & Saturn Moons (`Z04..Z07`)) in HyperCosmosVR.
Prerequisites: Confirm PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING have landed (vid >= 1000u decoder, base_flags cue packing, doc_feat(inst) active).

Target Files:
- `crates/cosmos_render/src/doc_stages.rs` (`L208-288, 318-342`: dispatch `Z04..Z07` -> `vid = 1404..1407`; spawn Phobos/Deimos + Mars rover `3644`, Vesta + metallic asteroid + clustered family `3645`, Galilean quartet incl. `vid=10` Europa `3646`, and Titan/Enceladus `3647`)
- `crates/cosmos_narration/src/resolver.rs` (`L952-961`: leverage `marte, ceres, jupiter, europa, saturno, encelado` + add `olympus, valles marineris, rover, fobos, deimos, vesta, kirkwood, mancha roja, titan, hexagono`)
- `shaders/hero_doc_cosmos.wgsl` (`L138-161, 187-194, 231-243, 392-405`: sculpt Olympus Mons NW caldera & rover crosshair `Z04`, Occator carbonate salt spots on Ceres & 3:1/5:2/2:1 Kirkwood gaps replacing Saturn Cassini on `Z05`, Europa ocean cutaway & Laplace orbits `Z06`, rotating north hexagon + Kraken Mare Titan + Enceladus south-polar plumes `Z07`, keyed on `doc_feat 0..4`)
- `shaders/compute_universe.wgsl` & `shaders/instanced_sprites.wgsl` (carve Jovian Kirkwood resonance voids into Mode 10 belt particle radii `L285-296` + support brushed metallic specular tint on `comp=3645`)

Reserved Canonical Shader IDs & Wiring:
- Primary zone vids: `1404` (`Z04 mars`), `1405` (`Z05 asteroid-belt`), `1406` (`Z06 jupiter`), `1407` (`Z07 saturn`)
- Companion/overlay vids: `3644` (Rover Beacon, Phobos & Deimos Irregular Moons), `3645` (Vesta Ellipsoid, Fe-Ni Psyche Shard, Orbital Family), `3646` (Galilean Quartet incl. `vid=10` Europa Ocean Wedge), `3647` (Orange Titan w/ Hydrocarbon Sea & Plume Enceladus)
- Preserve bits 24..31 submode and bits 0..7 base_flags; branch explicitly on doc_feat(inst) (0..k for spoken features, 0xFF/255 for zone macro), smoothed by doc_cue(inst) and doc_weight(inst) clamped cleanly into HDR AgX range without highlight blowout.
- Wire bilingual Spanish/English narration keywords & SegmentVisualHint::cue_tags in resolver.rs covering: casquete polar / Olympus Mons / Valles Marineris / rover / Fobos y Deimos / Ceres / Vesta / asteroide metálico / familia de asteroides / hueco de Kirkwood / aurora polar / bandas / Gran Mancha Roja / Europa / lunas galileanas / hexágono polar / anillo B / división de Cassini / Titán / Encélado.

Strict Rendering & Performance Budgets (Mandatory):
- Pure-Black OLED Void Budget: preserve >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout; use bounded local integration in shaders/hero_volumetric.wgsl with ZERO full-screen gray haze or background lift.
- VR 90/120 Hz Frame-Time Budget: cap any new volumetric documentary hero shell at <= 28 raymarch steps inside bounded r <= 0.96 with early exit trans < 0.03, preserving the RTX 5070 DX12 baseline (min = 0.843 ms, mean = 1.863 ms / 536.7 FPS, p99 = 4.661 ms, max = 5.802 ms in artifacts/verification_renders/benchmark_sweep.txt). Keep secondary moon companions lightweight analytic ray-marched spheres (`1` sphere intersection per companion inside `r <= 0.96`); preserve `<= 28` volume steps on Enceladus plume/Titan haze halos and `0.0000%` highlight blowout.
- Architecture & Repo Hygiene (CONTRIBUTING.md): zero 2D HUD overlays (keep UI purely diegetic 3D); commit zero files > 50 MiB, zero .ttf/.otf font binaries, and zero machine-specific .cargo/config.toml.

Required Verification Gates Before Opening PR:
1. cargo clippy --workspace --all-targets -- -D warnings
2. cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey
3. cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe
4. cargo test -p cosmos_spatial topology
5. cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets
6. Local GPU validation (if Discrete DX12 GPU present): cargo test --workspace -- --test-threads=1 && cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
```

</details>

<a id="pkg-act1-exp04-z08-11"></a>
### PKG-ACT1-EXP04-Z08-11: Exp 04 Solar System — Uranus Dark Rings, Neptune Supersonic/Triton, Kuiper Belt & Pluto-Charon Binary (`Z08..Z11`)
- **Priority & Rollout:** Phase 3 (Replaces `Z10` Cracked-Ocean Europa Placeholder & Adds Missing `Z11` Pluto Haze + Charon Binary) · Depends on `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Scope:** Exp `04` (`04`) Zones `Z08..Z11` (`4` zones, `20` features)
- **Current Card Tiers:** Zones `0A / 4B / 0C / 0D` · Features `0A / 7B / 0C / 13D`

**Current State:**
- **Primary Dispatch:** [`doc_stages.rs:208-264,290-342`](../../crates/cosmos_render/src/doc_stages.rs#L208-L342) emits `Z08 Uranus` (`vid=7` tilted `1.71 rad` + bright cyan Saturn-like ring `kind=2 vid=1` `alpha*0.45` `hero_doc_cosmos.wgsl:402`), `Z09 Neptune` (`vid=8` slow `time*0.04` cirrus, zero Triton), `Z10 Kuiper Belt` (**misuse**: reuses cracked-ocean Europa `vid=10` + Saturn Cassini ring `vid=1` + 7 icy sprites), and `Z11 Pluto` (`vid=9` [`hero_doc_cosmos.wgsl:176-185`](../../shaders/hero_doc_cosmos.wgsl#L176-L185) omits atmospheric Fresnel haze `L184` and omits Charon/4 moonlets). `doc_feat(inst)` is unread.
- **Card Tier Breakdown (`4` zones, `20` features):** Zones `0A / 4B / 0C / 0D` · Features `0A / 7B / 0C / 13D`.

**Reserved Target IDs:**

| Zone | Slug | Canonical `vid` | Companion `vid` (`3600..3869`) | `submode` (`bits 24..31`) | `base_flags` (`bits 0..7`) | `doc_feat` Slots (`0..7`, `0xFF=macro`) |
|---|---|---:|---|---|---|---:|
| `Z08` | `uranus` | `1408` | `3648` (Patchy Miranda Moon & Axial Solstice Rod) | `0` (`~98 deg` dark charcoal ringlets) | cue mask | `0..4` |
| `Z09` | `neptune` | `1409` | `3649` (Retrograde Cantaloupe Triton + N2 Plumes) | `0` (reversed orbit vector) | `0x02` + cue mask | `0..4` |
| `Z10` | `kuiper` | `1410` | `3650` (Triaxial Ringed Haumea, Makemake & Comet Tails) | `0` (`cos(3*theta)` 3:2 resonance belt) | `0x02` + cue mask | `0..4` |
| `Z11` | `pluto` | `1411` | `3651` (Tidally Locked Charon Barycenter & 4 Moonlets) | `0` (layered blue N2/CH4 Fresnel haze) | `0x02` + cue mask | `0..4` |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `[TIER-B]` Exp `04` `Z08` `uranus` — *Urano* / *Uranus* (`vid = 1408` | Feats: `0A / 3B / 0C / 2D`)
- **Metadata:** `raw_metric`: "Gigante de hielo" *(EN: "ice giant")* · `evidence_class`: `None` · `scene`: `None` · **`body_layout`:** `id=uranus, x=0.775, lane=0.15, r=23`
- **Spoken Macro (ES // EN):** Gira casi acostado: su eje está inclinado unos noventa y ocho grados respecto a su órbita. // *It rotates almost on its side: its axis is tilted about ninety-eight degrees relative to its orbit.*
- **Rendered Today:** `doc_stages.rs:290-313` emits `kind=1, vid=7` tilted `1.71 rad` (`~98 deg`, `hero_doc_cosmos.wgsl:163-174`) + ring `kind=2, vid=1` (`alpha * 0.45`, `L402`, still drawing Saturn Cassini/Encke gaps!).
- **Audit Gap:** Ring shader reuses bright Saturn-like Cassini gaps at `0.45` alpha instead of Uranus's dark charcoal epsilon ringlets; zero axial solstice arrow, patchy Verona Rupes Miranda moon, or supersonic icy jet streaks; `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1408`):** Dispatch `vid=1408` + companions `comp=3648` (dark epsilon ring profile + patchy Miranda companion). `F0` renders glowing spin-axis rod (`~98 deg` obliquity) showing sunlit polar solstice alignment; `F1` visualizes red-absorption vs cyan-green methane Rayleigh scattering spectrum halo; `F2` replaces Saturn `cassini_gap` on `z_idx == 8` with razor-thin charcoal-grey dark epsilon ringlets (`alpha ~ 0.14`); `F3` spawns jagged patchy Miranda (`comp=3648`) with giant Verona Rupes scarp; `F4` overlays crisp hundreds-of-km/h zonal wind stream arcs.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **polo iluminado** / *illuminated pole* | `[TIER-B]` | Durante parte de su larga órbita, un polo apunta casi directamente hacia el Sol. // *During part of its long orbit, one pole points almost directly toward the Sun.* | Tilted spin axis `1.71 rad` (`~98 deg`, `doc_stages.rs:292`) renders sideways orientation. | Draw axial pole rod + solstice solar incidence cone illuminating sunlit pole (`doc_feat == 0`). |
| `1` | **metano** / *methane* | `[TIER-B]` | Este gas absorbe luz roja y contribuye al tono azul verdoso del planeta. // *This gas absorbs red light and contributes to the planet's blue-green hue.* | Blue-green aquamarine methane atmosphere (`vid=7`, `hero_doc_cosmos.wgsl:164-173`). | Highlight incoming red photon absorption fading into cyan methane scatter haze (`doc_feat == 1`). |
| `2` | **anillos oscuros** / *dark rings* | `[TIER-B]` | Urano está rodeado por anillos estrechos y mucho menos brillantes que los de Saturno. // *Uranus is surrounded by narrow rings that are much less bright than those of Saturn.* | Tilted ring plane `kind=2, vid=1` emitted (`doc_stages.rs:304-311`), though bright cyan `0.45` alpha + Saturn Cassini gap misrepresents dark rings. | Switch `z_idx == 8` ring shader to narrow dark charcoal-dust epsilon ringlets (`doc_feat == 2`). |
| `3` | **Miranda** / *Miranda* | `[TIER-D]` | Su pequeña luna presenta acantilados, surcos y terrenos ensamblados de forma extraña. // *Its small moon features cliffs, grooves and strangely assembled terrain.* | No Miranda satellite or chaotic grooved cliffs. | Spawn patchy icy-rocky Miranda companion (`comp=3648`) featuring bright Verona Rupes cliff groove (`doc_feat == 3`). |
| `4` | **vientos** / *winds* | `[TIER-D]` | En su atmósfera extremadamente fría soplan corrientes de cientos de kilómetros por hora. // *In its extremely cold atmosphere currents blow at hundreds of kilometers per hour.* | Slow low-contrast `vid=7` shading (`L166`) omits high-speed jet currents. | Add crisp pale-cyan high-velocity jet streamlines across mid/polar bands (`doc_feat == 4`). |

#### `[TIER-B]` Exp `04` `Z09` `neptune` — *Neptuno* / *Neptune* (`vid = 1409` | Feats: `0A / 2B / 0C / 3D`)
- **Metadata:** `raw_metric`: "Gigante de hielo" *(EN: "ice giant")* · `evidence_class`: `None` · `scene`: `None` · **`body_layout`:** `id=neptune, x=0.86, lane=-0.14, r=23`
- **Spoken Macro (ES // EN):** Es el planeta más lejano del Sol y posee algunos de los vientos más rápidos medidos en el sistema solar. // *It is the farthest planet from the Sun and has some of the fastest winds measured in the solar system.*
- **Rendered Today:** `doc_stages.rs:320,333-342` emits `kind=1, vid=8` cobalt-blue methane globe (`hero_doc_cosmos.wgsl:163-174`) with high white cirrus streaks and Great Dark Spot oval.
- **Audit Gap:** Omits supersonic >2,000 km/h westward wind velocity streamlines (`time * 0.04` drift barely perceptible), captured retrograde moon Triton, and Voyager 2 nitrogen geyser plumes; `doc_feat` unused.
- **3D Zone Blueprint (`vid = 1409`):** Dispatch `vid=1409` + companions `comp=3649` (retrograde cantaloupe-crusted Triton + dark nitrogen geysers). `F0` raises bright white high-altitude methane cirrus parallax decks casting subtle lower-cloud shadows; `F1` highlights evolving Great Dark Spot vortex + white orographic companion cloud; `F2` animates rapid westward >2,000 km/h supersonic wind tracer streaks across equatorial latitudes; `F3` spawns inclined retrograde-orbiting icy Triton (`comp=3649`) with reversed orbital vector arrows; `F4` erupts dark soot-colored vertical nitrogen plumes bending downwind across Triton.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **nubes de metano** / *methane clouds* | `[TIER-B]` | Nubes brillantes aparecen a gran altura sobre una atmósfera azul profunda. // *Bright clouds appear high above a deep blue atmosphere.* | High white `cirrus` streaks over deep azure methane disk (`hero_doc_cosmos.wgsl:170`). | Lift bright methane ice clouds into crisp upper-troposphere parallax with subtle cloud shadow (`doc_feat == 0`). |
| `1` | **tormenta oscura** / *dark storm* | `[TIER-B]` | Grandes vórtices pueden surgir, cambiar y desaparecer en pocos años. // *Large vortices can arise, change and disappear within a few years.* | Deep indigo `dark_spot` storm oval (`L169, L171`). | Frame Great Dark Spot cyclonic collar + transient bright companion cloud (`doc_feat == 1`). |
| `2` | **viento supersónico** / *supersonic wind* | `[TIER-D]` | Algunas corrientes superan los dos mil kilómetros por hora. // *Some currents exceed two thousand kilometers per hour.* | `time * 0.04` drift lacks >2,000 km/h supersonic wind speed representation. | Animate fast westward equatorial supersonic jet streamlines + speed vector (`doc_feat == 2`). |
| `3` | **Tritón** / *Triton* | `[TIER-D]` | La gran luna orbita en sentido retrógrado y probablemente fue capturada del cinturón de Kuiper. // *The large moon orbits retrograde and was probably captured from the Kuiper Belt.* | No retrograde-orbiting Triton satellite or inclined capture orbit track. | Spawn cantaloupe-textured icy Triton (`comp=3649`) orbiting retrograde against prograde marker (`doc_feat == 3`). |
| `4` | **géiseres de nitrógeno** / *nitrogen geysers* | `[TIER-D]` | Voyager 2 observó columnas oscuras elevándose desde la superficie helada de Tritón. // *Voyager 2 observed dark plumes rising from Triton's icy surface.* | No dark nitrogen geyser plumes observed by Voyager 2 rising off Triton. | Erupt vertical dark carbon/N2 cryo-geyser plumes curving downwind across Triton's illuminated cap (`doc_feat == 4`). |

#### `[TIER-B]` Exp `04` `Z10` `kuiper` — *Cinturón de Kuiper* / *Kuiper Belt* (`vid = 1410` | Feats: `0A / 1B / 0C / 4D`)
- **Metadata:** `raw_metric`: "Aprox. 30–50 UA del Sol · Región de mundos helados" *(EN: "Approx. 30–50 AU from the Sun")* · `evidence_class`: `None` · `scene`: `None` · **`body_layout`:** `None` (ring/belt stage)
- **Spoken Macro (ES // EN):** Más allá de Neptuno orbitan innumerables cuerpos helados: restos conservados de la formación planetaria. // *Beyond Neptune orbit countless icy bodies: preserved remains of planetary formation.*
- **Rendered Today:** `doc_stages.rs:208-264` emits `body_idx == 10`: reuses `vid=10` (Europa cracked ocean!), `kind=2, vid=1` ring (Saturn Cassini/Encke gaps!), and `7` cyan icy sprites (`tags_f.z = 0.5`).
- **Audit Gap:** Placeholder composite borrows Europa's warm cracked-ocean sphere and Saturn's rings instead of nitrogen/methane-frosted KBOs; zero Haumea triaxial ringed dwarf, zero bright Makemake, zero Neptune 3:2/2:1 resonance lobes, and zero short-period comet tail.
- **3D Zone Blueprint (`vid = 1410`):** Dispatch `vid=1410` + companions `comp=3650` (triaxial fast-spinning ringed Haumea, bright Makemake, comet coma/tails). `F0` replaces `vid=10` warm ocean cracks with dark red-tholin & pale N2/CH4 frost KBO globe facing tiny pinpoint Sun glare; `F1` spawns flattened rapidly spinning triaxial Haumea (`vec3(1.55, 1.0, 0.72)`) + faint dust ring; `F2` spawns bright high-albedo Makemake globe; `F3` modulates outer icy torus via `cos(3*theta - phase)` Neptune 3:2 orbital resonance clusters; `F4` animates perturbed icy nucleus plunging inward growing cyan ion + curved dust tails.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **mundo helado** / *frozen world* | `[TIER-B]` | Agua, metano, amoníaco y nitrógeno pueden permanecer congelados bajo un Sol diminuto. // *Water, methane, ammonia and nitrogen can remain frozen under a tiny Sun.* | Icy dwarf sphere + 7 icy shard sprites + outer icy torus (`doc_stages.rs:215-264`), reusing cracked-ocean Europa `vid=10`. | Replace warm-ocean cracks with cratered H2O/CH4/NH3/N2 frosted KBOs + pinpoint distant Sun (`doc_feat == 0`). |
| `1` | **Haumea** / *Haumea* | `[TIER-D]` | Este planeta enano gira tan rápido que tiene una forma muy alargada y además posee un anillo. // *This dwarf planet rotates so fast that it has a very elongated shape and also has a ring.* | Zero fast-spinning elongated triaxial Haumea dwarf or Haumea ring. | Spawn fast-rotating triaxial ellipsoid Haumea (`comp=3650`) + thin equatorial ring (`doc_feat == 1`). |
| `2` | **Makemake** / *Makemake* | `[TIER-D]` | Una superficie brillante rica en hielos rodea a uno de los grandes objetos transneptunianos. // *A bright, ice-rich surface surrounds one of the large trans-Neptunian objects.* | Zero bright methane-frost Makemake companion. | Spawn bright high-albedo reddish-white Makemake dwarf (`comp=3650`) beside belt (`doc_feat == 2`). |
| `3` | **resonancia orbital** / *orbital resonance* | `[TIER-D]` | La gravedad de Neptuno organiza muchos cuerpos en ritmos orbitales estables. // *Neptune's gravity organizes many bodies into stable orbital rhythms.* | Ring `kind=2, vid=1` displays Saturn's Cassini/Encke gaps instead of Neptune orbital resonances. | Modulate belt ring density with Neptune 3:2 Plutino resonance lobes (`cos(3*theta - phase)`, `doc_feat == 3`). |
| `4` | **cometa de período corto** / *short-period comet* | `[TIER-D]` | Algunas órbitas alteradas envían cuerpos helados hacia el sistema solar interior. // *Some altered orbits send icy bodies toward the inner solar system.* | Zero inward-plunging short-period comet nucleus or twin ion/dust tails. | Emit perturbed parabolic/elliptical comet nucleus growing anti-solar blue ion + amber curved dust tails (`doc_feat == 4`). |

#### `[TIER-B]` Exp `04` `Z11` `pluto` — *Plutón* / *Pluto* (`vid = 1411` | Feats: `0A / 1B / 0C / 4D`)
- **Metadata:** `raw_metric`: "Planeta enano · cinturón de Kuiper" *(EN: "Dwarf planet · Kuiper Belt")* · `evidence_class`: `None` · `scene`: `None` · **`body_layout`:** `id=pluto, x=0.98, lane=0.075, r=12`
- **Spoken Macro (ES // EN):** Es un mundo activo de hielos móviles, montañas de agua congelada y una atmósfera estacional. // *It is an active world of moving ices, mountains of water ice, and a seasonal atmosphere.*
- **Rendered Today:** `doc_stages.rs:321,333-342` emits `kind=1, vid=9` (`hero_doc_cosmos.wgsl:176-185`) featuring dark reddish Cthulhu tholin terrain and twin-lobe cream Sputnik Planitia nitrogen-ice heart.
- **Audit Gap:** `hero_doc_cosmos.wgsl:184` omits atmospheric Fresnel haze completely (so Pluto appears airless despite thin N2/CH4 haze!), smooth FBM lacks 3–4 km water-ice mountain ridge relief, and zero Charon binary barycenter or 4 small moons (Nix, Hydra, Kerberos, Styx) exist.
- **3D Zone Blueprint (`vid = 1411`):** Dispatch `vid=1411` + companions `comp=3651` (Charon binary partner `0.51*r_pluto` + mutual barycenter crosshair + 4 small moonlet sprites). `F0` adds tenuous layered pale-blue Rayleigh haze Fresnel halo (`pow(1 - view_dot, 5.2) * vec3(0.45,0.75,1.35)`); `F1` overlays convective polygonal Voronoi cells inside western Sputnik Planitia nitrogen-ice heart; `F2` bumps sharp jagged al-Idrisi water-ice mountains along Sputnik's western edge; `F3` spawns Mordor-capped Charon (`comp=3651`) tidally locked orbiting external common barycenter beacon; `F4` orbits 4 elongated tiny sprites for Nix, Hydra, Kerberos & Styx.

| `f` | Feature (`ES` / `EN`) | Tier | Spoken Content (`ES` // `EN`) | Code Reality & Gap | 3D Feature Sub-Stage Blueprint (`doc_feat(inst)`) |
|---:|---|:---:|---|---|---|
| `0` | **atmósfera tenue** / *tenuous atmosphere* | `[TIER-D]` | Nitrógeno, metano y monóxido de carbono forman una envoltura que cambia con la distancia al Sol. // *Nitrogen, methane and carbon monoxide form an envelope that changes with distance from the Sun.* | `hero_doc_cosmos.wgsl:176-185` lacks atmospheric Fresnel limb haze entirely (unlike Mars `L146` / Neptune `L173`). | Add delicate stratified blue nitrogen/methane haze Fresnel rim pulsing softly (`doc_feat == 0`). |
| `1` | **Sputnik Planitia** / *Sputnik Planitia* | `[TIER-B]` | Una llanura de hielo de nitrógeno ocupa el lóbulo occidental del famoso corazón brillante. // *A nitrogen ice plain occupies the western lobe of the famous bright heart.* | Iconic twin-lobe cream Sputnik Planitia heart rendered cleanly (`L178-183`), ignoring `doc_feat`. | Subdivide western Sputnik Planitia lobe into shimmering nitrogen-ice convection polygons (`doc_feat == 1`). |
| `2` | **montañas de hielo** / *ice mountains* | `[TIER-D]` | Bloques de agua congelada se elevan varios kilómetros sobre la superficie. // *Blocks of frozen water rise several kilometers above the surface.* | Smooth `fbm3` terrain (`L181`) lacks jagged multi-kilometer water-ice mountain relief. | Perturb surface normal heightmap along Sputnik western margin to cast sharp 3-4 km mountain terminator shadows (`doc_feat == 2`). |
| `3` | **Caronte** / *Charon* | `[TIER-D]` | Plutón y su gran luna muestran siempre la misma cara el uno al otro mientras orbitan un punto común. // *Pluto and its large moon always show the same face to each other as they orbit a common point.* | Zero Charon binary companion or mutual off-center barycenter ring (`doc_stages.rs:321`). | Spawn grey red-polar Charon companion (`r=0.51*r_pluto`, `comp=3651`) locked across external barycenter ring (`doc_feat == 3`). |
| `4` | **cuatro lunas pequeñas** / *four small moons* | `[TIER-D]` | Nix, Hidra, Cerbero y Estigia acompañan al sistema doble de Plutón y Caronte. // *Nix, Hydra, Kerberos, and Styx accompany the Pluto–Charon binary system.* | Zero Nix, Hydra, Kerberos, Styx outer tiny moonlet tracks or sprites. | Emit 4 coplanar circular outer orbits with elongated bright moonlet sprites (`comp=3651`, `doc_feat == 4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT1-EXP04-Z08-11)</summary>

```markdown
Task: Implement Pick-Up Package PKG-ACT1-EXP04-Z08-11 (Exp 04 Solar System — Uranus Dark Rings, Neptune Supersonic/Triton, Kuiper Belt & Pluto-Charon Binary (`Z08..Z11`)) in HyperCosmosVR.
Prerequisites: Confirm PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING have landed (vid >= 1000u decoder, base_flags cue packing, doc_feat(inst) active).

Target Files:
- `crates/cosmos_render/src/doc_stages.rs` (`L208-264, 290-342`: dispatch `Z08..Z11` -> `vid = 1408..1411`, replace `Z10` `vid=10` reference, spawn Miranda `3648`, retrograde Triton `3649`, Haumea/Makemake/comet `3650`, and Pluto-Charon barycenter + 4 outer moonlets `3651`)
- `crates/cosmos_narration/src/resolver.rs` (`L952-961`: leverage `urano, neptuno, pluton` + add `metano, anillos oscuros, miranda, triton, geiser, kuiper, haumea, makemake, cometa, caronte, sputnik`)
- `shaders/hero_doc_cosmos.wgsl` (`L163-185, 392-405`: narrow `Z08` rings to charcoal dark epsilon profile + solstice pole rod, animate `>2000 km/h` westward jet streaks & dark Triton N2 geysers `Z09`, build N2/CH4 frosted KBO + triaxial Haumea + `cos(3*theta)` resonance + ion/dust comet tails `Z10`, add Pluto blue Fresnel haze `pow(1-view_dot,5.2)` + Sputnik convective cells + water-ice peaks `Z11` on `doc_feat 0..4`)
- `shaders/compute_universe.wgsl` (`L285-296`: sculpt 3:2/2:1 Neptune resonance clumps on Mode 10 outer Kuiper particles & comet anti-solar tail stream on `Z10 F4`)

Reserved Canonical Shader IDs & Wiring:
- Primary zone vids: `1408` (`Z08 uranus`), `1409` (`Z09 neptune`), `1410` (`Z10 kuiper`), `1411` (`Z11 pluto`)
- Companion/overlay vids: `3648` (Patchy Miranda Moon & Axial Solstice Rod), `3649` (Retrograde Cantaloupe Triton + N2 Plumes), `3650` (Triaxial Ringed Haumea, Makemake & Comet Tails), `3651` (Tidally Locked Charon Barycenter & 4 Moonlets)
- Preserve bits 24..31 submode and bits 0..7 base_flags; branch explicitly on doc_feat(inst) (0..k for spoken features, 0xFF/255 for zone macro), smoothed by doc_cue(inst) and doc_weight(inst) clamped cleanly into HDR AgX range without highlight blowout.
- Wire bilingual Spanish/English narration keywords & SegmentVisualHint::cue_tags in resolver.rs covering: polo iluminado / metano / anillos oscuros / Miranda / vientos / nubes de metano / tormenta oscura / viento supersónico / Tritón / géiseres de nitrógeno / mundo helado / Haumea / Makemake / resonancia orbital / cometa de período corto / atmósfera tenue / Sputnik Planitia / montañas de hielo / Caronte / cuatro lunas pequeñas.

Strict Rendering & Performance Budgets (Mandatory):
- Pure-Black OLED Void Budget: preserve >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout; use bounded local integration in shaders/hero_volumetric.wgsl with ZERO full-screen gray haze or background lift.
- VR 90/120 Hz Frame-Time Budget: cap any new volumetric documentary hero shell at <= 28 raymarch steps inside bounded r <= 0.96 with early exit trans < 0.03, preserving the RTX 5070 DX12 baseline (min = 0.843 ms, mean = 1.863 ms / 536.7 FPS, p99 = 4.661 ms, max = 5.802 ms in artifacts/verification_renders/benchmark_sweep.txt). Keep comet coma/Pluto haze raymarching strictly `<= 24` steps within `r <= 0.96`; ensure dark charcoal epsilon rings (`alpha ~ 0.14`) and outer Kuiper void maintain `>= 50.91%` exact `#000000` background.
- Architecture & Repo Hygiene (CONTRIBUTING.md): zero 2D HUD overlays (keep UI purely diegetic 3D); commit zero files > 50 MiB, zero .ttf/.otf font binaries, and zero machine-specific .cargo/config.toml.

Required Verification Gates Before Opening PR:
1. cargo clippy --workspace --all-targets -- -D warnings
2. cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey
3. cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe
4. cargo test -p cosmos_spatial topology
5. cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets
6. Local GPU validation (if Discrete DX12 GPU present): cargo test --workspace -- --test-threads=1 && cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
```

</details>
