# Odyssey Act II: Cosmic Web & Origins — Visual Expansion Map

[Master Visual Roadmap](../CONTRIBUTOR_VISUAL_ROADMAP.md) · [README](../../README.md) · [CONTRIBUTING](../../CONTRIBUTING.md) · [Shader Contracts](../../shaders/SHADER_CONTRACTS.md) · [Act I Map](ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md) · [Act III Map](ACT_3_MATTER_AND_SPACETIME_MAP.md) · [Act IV Map](ACT_4_LIFE_MIND_AND_SENSES_MAP.md) · [Act V Map](ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md)

> **Scope & Ground Truth:** Complete Act II contributor & AI-agent execution map covering **8 Experiences**, **116 Zones**, **684 Features**, **31 Baked Landmarks** *(+1 runtime `Z12 galactic-edge` landmark in `assets.rs` = `32` live entities)*, and **26 Claimable Work Packages** (`PKG-ACT2-*`, out of **88** total packages catalog-wide: `3` P0 + `85` Content; **`3 / 26` Act II packages = `100%` of Ch 1 `05_milky_way` now `[COMPLETED — TIER-A]`**) in exact [`ODYSSEY_ACTS[1]`](../../crates/cosmos_journey/src/odyssey.rs#L29-L32) travel order (`&[5, 21, 22, 6, 7, 8, 9, 10]`). Every transcript summary, metric, evidence badge, landmark ID, code line number, and Tier count below is extracted directly from [`assets/baked/universe_catalog.v1.json`](../../assets/baked/universe_catalog.v1.json) and active Rust/WGSL sources (never aspirational prose in [documentary_visuals_spec.md](../documentary_visuals_spec.md)).

> [!IMPORTANT]
> **Odyssey Act II Travel Order (`&[5, 21, 22, 6, 7, 8, 9, 10]`):** [`odyssey.rs#L29-L32`](../../crates/cosmos_journey/src/odyssey.rs#L29-L32) embeds **Exp 21 (`21_cosmic_life`)** and **Exp 22 (`22_cosmic_contact`)** inside the Orion Spur between **Exp 05 (`05_milky_way`)** and **Exp 06 (`06_local_group`)**, while [`topology.rs#L419-L426`](../../crates/cosmos_spatial/src/topology.rs#L419-L426) links both `5 -> 21 -> 22` (`EmbeddedStellarArm`) and the outward cosmological ladder `5 -> 6 -> 7 -> 8 -> 9 -> 10` (`ScaleRiseOutward`). Chapters and Pick-Up Cards below follow exact **Odyssey Act II progression (`05 -> 21 -> 22 -> 06 -> 07 -> 08 -> 09 -> 10`)**.

## 1. Act II Executive Scorecard & Tier Audit (`116` Zones / `684` Features)

| Pos | Exp | Slug | Catalog Title (ES) | Zones | Zone `[A / B / C / D]` | Feats | Feat `[A / B / C / D]` | Landmarks | Current `doc_stage_variant_id` / Dispatch | Reserved `vid` Block | Packages |
|---:|---:|---|---|---:|:---:|---:|:---:|---:|---|---|---:|
| 1 / 8 | `05` | `05_milky_way` | Espectro de la Vía Láctea · Del sistema solar al borde galáctico | **13** | `13 / 0 / 0 / 0` | **65** | `65 / 0 / 0 / 0` | 13 (`12` baked + `Z12`) | **`[TIER-A]` Complete:** canonical `1500..=1512` stages + `3660..=3671` companions + `Z12` landmark + `65/65` `doc_feat` | `1500..=1512` (3660..=3671) | `3` *(3 done)* |
| 2 / 8 | `21` | `21_cosmic_life` | Espectro de la Vida Cósmica: del Planeta Habitable a la Inteligencia Extraterrestre | **25** | `0 / 4 / 4 / 17` | **200** | `0 / 10 / 32 / 158` | 0 | `z=12 -> vid=212`, all other `24` zones -> `vid=210` + twin `11/10` globes (`sc` ignored) | `3100..=3124` (3820..=3829) | `5` |
| 3 / 8 | `22` | `22_cosmic_contact` | Espectro del Contacto Cósmico: de la Primera Señal al Diálogo entre Civilizaciones | **28** | `0 / 5 / 15 / 8` | **168** | `0 / 11 / 88 / 69` | 0 | `z=12 -> vid=221`, all other `27` zones -> `vid=220` SETI dish (`5` shared `scene` pairs) | `3200..=3227` (3830..=3839) | `6` |
| 4 / 8 | `06` | `06_local_group` | Espectro del Grupo Local: de la Vía Láctea a Andrómeda | **9** | `0 / 3 / 0 / 6` | **45** | `0 / 9 / 0 / 36` | 9 | Anchor & all `9` landmarks emit identical `kind=3, vid=6` static spiral diorama | `1600..=1608` (3680..=3688) | `2` |
| 5 / 8 | `07` | `07_laniakea` | Espectro de Laniakea: del Grupo Local al Gran Atractor | **10** | `0 / 5 / 0 / 5` | **50** | `0 / 10 / 0 / 40` | 10 | Anchor & all `10` landmarks emit identical `kind=3, vid=7` watershed diorama (Mode 1) | `1700..=1709` (3700..=3709) | `2` |
| 6 / 8 | `08` | `08_cosmic_web` | Espectro de la Red Cósmica: de Laniakea a la Estructura del Universo | **11** | `0 / 6 / 3 / 2` | **55** | `0 / 19 / 6 / 30` | 0 | Single static `kind=3, vid=8` uniform Voronoi diorama (Mode 2) | `1800..=1810` (Zone-parameterized `submode` + `cs_field_grid` dark-matter scaffold) | `3` |
| 7 / 8 | `09` | `09_observable_universe` | Espectro del Universo Observable: de la Red Cósmica al Horizonte Cósmico | **11** | `0 / 5 / 4 / 2` | **55** | `0 / 12 / 6 / 37` | 0 | Single static `kind=3, vid=9` CMB + quasar jet (Mode 2 lock + `cmb_alpha=1.0` leak) | `1900..=1910` (Epoch-gated `compute_mode`) | `3` |
| 8 / 8 | `10` | `10_early_universe` | Espectro del Universo Temprano: del Fondo Cósmico a la Inflación | **9** | `0 / 2 / 4 / 3` | **46** | `0 / 6 / 8 / 32` | 0 | Single static `kind=3, vid=10` expansion ring (Mode 2 lock; Modes 3/5 unused) | `2000..=2008` (Zone-gated compute) | `2` |
| **ALL** | **`8` Exps** | **Odyssey Act II** | **Milky Way, Life, Contact, Local Group, Laniakea, Cosmic Web, Observable & Early Universe** | **`116`** | **`13 / 30 / 30 / 43`** | **`684`** | **`65 / 77 / 140 / 402`** | **`32`** *(`31` baked + `Z12`)* | **11.2% Tier-A · 25.9% B · 25.9% C · 37.1% D (Zones)** | **`1500..=3227` + `3660..=3839`** | **`26`** *(`3` done)* |

*Feature distribution across Act II (`684` features): `9.5%` Tier-A (`65`), `11.3%` Tier-B (`77`), `20.5%` Tier-C (`140`), `58.8%` Tier-D (`402`).*

## 2. P0 Core Pipeline Prerequisites & Act II Architectural Findings

Every Act II content package inherits four upstream pipeline blockers tracked by the **3 Core P0 Packages** in [CONTRIBUTOR_VISUAL_ROADMAP.md](../CONTRIBUTOR_VISUAL_ROADMAP.md). Contributors claiming Act II cards should ensure these P0 contracts are either merged or explicitly stubbed cleanly via feature branches:

1. **[`PKG-P0-DISPATCH-AND-IDS`](../CONTRIBUTOR_VISUAL_ROADMAP.md#pkg-p0-dispatch-and-ids)** (`crates/cosmos_render/src/doc_stages.rs#L36-L91`, `#L358-L367`, `#L657-L691`, `#L764-L786` + WGSL `>= 1000` decoders):
   - Extend `ch_exp_and_sub(vid)` ([`shaders/hero_doc_civilization_horizon.wgsl#L20-L25`](../../shaders/hero_doc_civilization_horizon.wgsl#L20-L25)) and add an analogous `cos_exp_and_sub(vid)` decoder inside [`shaders/hero_doc_cosmos.wgsl`](../../shaders/hero_doc_cosmos.wgsl) so `vid >= 1000u` routes cleanly as `exp = (vid - 1000u) / 100u`, `sub = (vid - 1000u) % 100u`, while legacy `vid < 1000u` keeps untouched fallbacks working.
   - Eliminate `zone_index % 3` landmark hashing in **Exp 05** (`doc_stages.rs#L359-L362`), constant `6/7/8/9/10` landmark/anchor collapse (`doc_stages.rs#L363-L367, #L696-L708`), and constant `210`/`220` flattening in **Exps 21 & 22** (`doc_stages.rs#L75-L82`).
2. **[`PKG-P0-FEATURE-AND-CUE-PLUMBING`](../CONTRIBUTOR_VISUAL_ROADMAP.md#pkg-p0-feature-and-cue-plumbing)** (`crates/cosmos_app/src/engine.rs#L1691-L1699`, `crates/cosmos_render/src/universe_scene.rs#L17-L25`, `#L301-L377`, `crates/cosmos_narration/src/resolver.rs#L952-L1095`):
   - Forward `SegmentVisualHint::cue_tags` & `emphasis_boost` into `DocumentaryFocusState` (currently dropped at `engine.rs#L1691-L1699`), pack `base_flags` + `submode` via `pack_doc_tags`, and replace the single global `doc_feat * 0.45` orbital offset with real per-feature `doc_feat(inst)` (`0..=7`) + smooth `doc_cue(inst)` + `doc_weight(inst)` shader choreography.
   - Uncouple **`select_compute_mode(u_cam, experience_id)`** ([`universe_scene.rs#L301-L315`](../../crates/cosmos_render/src/universe_scene.rs#L301-L315)) and **`build_far_shell_uniforms`** ([`universe_scene.rs#L344-L377`](../../crates/cosmos_render/src/universe_scene.rs#L344-L377)): stop locking Exps `08..=10` to Compute Mode 2 (Cosmic Web filaments) across Reionization, CMB, Quark Era (`compute_universe.wgsl#L162-L187` Mode 3 already implemented!), and Planck/Inflation (`compute_universe.wgsl#L209-L222` Mode 5 already implemented!); stop leaking `cmb_alpha=1.0` across pre-CMB zones (`09 Z00..Z07`) and stop drawing `star_density = 0.25` stars inside pre-stellar epochs (`09 Z07..Z08`, `10 Z00..Z08`).
3. **[`PKG-P0-CATALOG-SCENE-AND-CI`](../CONTRIBUTOR_VISUAL_ROADMAP.md#pkg-p0-catalog-scene-and-ci)** ([`assets/baked/universe_catalog.v1.json`](../../assets/baked/universe_catalog.v1.json)):
   - Split **Exp 22**'s 5 duplicated `scene` indices (`0`: `Z00/Z01`, `8`: `Z09/Z10`, `13`: `Z15/Z16`, `14`: `Z17/Z18`, `15`: `Z19/Z20`) — or guarantee `doc_stage_variant_id(22)` keys on `doc.active_zone_index` so shared scenes never collide.
   - Backfill explicit `scene = 0..z_max` values across **Exps `05, 06, 07, 08, 09, 10`** (currently `null` in all `63` zones) and resolve **Exp 05 `z=12` (`galactic-edge`)** lacking a baked landmark entry.

### VR 90/120 Hz & Pure-Black OLED Guardrails (All 26 Packages)
- **Pure-Black OLED Void Budget:** `>= 50.91%` exact `#000000` background pixels (`0.0000%` highlight blowout; strict bounded local integration inside `shaders/hero_volumetric.wgsl` — zero full-screen additive wash or milky rectangular glow belts).
- **VR Frame-Time Budget:** Cap new documentary hero raymarches at `<= 28` steps with `r <= 0.96` sphere-bound clipping and `trans < 0.03` early termination, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- **Verification Gates per PR:** `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace -- --test-threads=1` (+ zero `> 50 MiB` assets, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml`).

## 3. Quick-Jump Index: All `26` Act II Pick-Up Cards

| Package ID | Exp | Zones | Feats | Phase & Priority | Zone Tier `[A/B/C/D]` | Feat Tier `[A/B/C/D]` | Focus Summary |
|---|---:|:---:|---:|---|:---:|:---:|---|
| [`PKG-ACT2-EXP05-Z00-04`](#pkg-act2-exp05-z00-04) | `05` | `Z00..Z04` (5) | `25` | Phase 1 + Phase 3 · **`[COMPLETED — TIER-A]`** | `5/0/0/0` | `25/0/0/0` | **Completed (`1500..=1504` + `3660..=3664`):** Solar frontier -> Alpha Centauri bespoke geometry, landmark LOD floor, `25/25` `doc_feat` transitions |
| [`PKG-ACT2-EXP05-Z05-08`](#pkg-act2-exp05-z05-08) | `05` | `Z05..Z08` (4) | `20` | Phase 1 + Phase 3 · **`[COMPLETED — TIER-A]`** | `4/0/0/0` | `20/0/0/0` | **Completed (`1505..=1508` + `3665..=3668`):** Stellar neighborhood, Local Bubble cavity, Orion Spur, pillar nursery + `20/20` `doc_feat` transitions |
| [`PKG-ACT2-EXP05-Z09-12`](#pkg-act2-exp05-z09-12) | `05` | `Z09..Z12` (4) | `20` | Phase 1 + Phase 3 · **`[COMPLETED — TIER-A]`** | `4/0/0/0` | `20/0/0/0` | **Completed (`1509..=1512` + `3669..=3671` + 13th `Z12` landmark):** Galactic disk, Sgr A*, halo, Milky Way edge + `20/20` `doc_feat` transitions |
| [`PKG-ACT2-EXP21-Z00-04`](#pkg-act2-exp21-z00-04) | `21` | `Z00..Z04` (5) | `40` | Phase 1 · **Critical** | `0/1/1/3` | `0/3/5/32` | Cosmic Life Origins & Planetary Habitability Lab (`z=00..04`) |
| [`PKG-ACT2-EXP21-Z05-09`](#pkg-act2-exp21-z05-09) | `21` | `Z05..Z09` (5) | `40` | Phase 1 · **Critical** | `0/0/1/4` | `0/0/6/34` | Ocean Worlds, Biosphere Diversity & Evolutionary Thresholds (`z=05..09`) |
| [`PKG-ACT2-EXP21-Z10-14`](#pkg-act2-exp21-z10-14) | `21` | `Z10..Z14` (5) | `40` | Phase 1 · **Critical** | `0/3/0/2` | `0/7/0/33` | Extremophiles, Europa Plumes, Exoplanet Detection & Biosignatures (`z=10..14`) |
| [`PKG-ACT2-EXP21-Z15-19`](#pkg-act2-exp21-z15-19) | `21` | `Z15..Z19` (5) | `40` | Phase 1 · **Critical** | `0/0/0/5` | `0/0/8/32` | Technosignatures, Great Filter & Fermi Silence (`z=15..19`) |
| [`PKG-ACT2-EXP21-Z20-24`](#pkg-act2-exp21-z20-24) | `21` | `Z20..Z24` (5) | `40` | Phase 1 · **Critical** | `0/0/2/3` | `0/0/13/27` | Cosmic Humility, Alien Societal Archetypes & Interstellar Ethics (`z=20..24`) |
| [`PKG-ACT2-EXP22-Z00-04`](#pkg-act2-exp22-z00-04) | `22` | `Z00..Z04` (5) | `30` | Phase 1 · **Critical** | `0/3/0/2` | `0/5/5/20` | SETI Anomaly Triage, Noise Verification & Signal Confirmation (`z=00..04`, `scene=0` split) |
| [`PKG-ACT2-EXP22-Z05-09`](#pkg-act2-exp22-z05-09) | `22` | `Z05..Z09` (5) | `30` | Phase 1 · **Critical** | `0/0/3/2` | `0/1/17/12` | Passive vs Active Searches & Global Coordination Forums (`z=05..09`, `scene=8` part 1) |
| [`PKG-ACT2-EXP22-Z10-14`](#pkg-act2-exp22-z10-14) | `22` | `Z10..Z14` (5) | `30` | Phase 1 · **Critical** | `0/1/2/2` | `0/2/16/12` | Social Reaction, Prime Math Signatures & Cryptographic Decoding (`z=10..14`, `scene=8` part 2) |
| [`PKG-ACT2-EXP22-Z15-19`](#pkg-act2-exp22-z15-19) | `22` | `Z15..Z19` (5) | `30` | Phase 1 · **Critical** | `0/1/3/1` | `0/3/15/12` | Xenocognition, Reply Dilemmas & Relativistic Light-Cones (`z=15..19`, `scene=13/14/15` split) |
| [`PKG-ACT2-EXP22-Z20-23`](#pkg-act2-exp22-z20-23) | `22` | `Z20..Z23` (4) | `24` | Phase 1 · **Critical** | `0/0/3/1` | `0/0/13/11` | Past-Ghost Senders, Multigenerational Relays & Epistemic Limits (`z=20..23`) |
| [`PKG-ACT2-EXP22-Z24-27`](#pkg-act2-exp22-z24-27) | `22` | `Z24..Z27` (4) | `24` | Phase 1 · **Critical** | `0/0/4/0` | `0/0/22/2` | Diplomatic Protocols, Cultural Transformation & Cosmic Mirror (`z=24..27`) |
| [`PKG-ACT2-EXP06-Z00-04`](#pkg-act2-exp06-z00-04) | `06` | `Z00..Z04` (5) | `25` | Phase 3 · **High** | `0/1/0/4` | `0/3/0/22` | Galactic Edge, Outer Halo, Dwarfs & Magellanic Clouds (`z=00..04`) |
| [`PKG-ACT2-EXP06-Z05-08`](#pkg-act2-exp06-z05-08) | `06` | `Z05..Z08` (4) | `20` | Phase 3 · **High** | `0/2/0/2` | `0/6/0/14` | Triangulum, Andromeda Approach, Barycenter & Local Group Rim (`z=05..08`) |
| [`PKG-ACT2-EXP07-Z00-04`](#pkg-act2-exp07-z00-04) | `07` | `Z00..Z04` (5) | `25` | Phase 3 · **High** | `0/2/0/3` | `0/4/0/21` | Local Volume, Local Sheet, Virgo Cluster & Supercluster (`z=00..04`) |
| [`PKG-ACT2-EXP07-Z05-09`](#pkg-act2-exp07-z05-09) | `07` | `Z05..Z09` (5) | `25` | Phase 3 · **High** | `0/3/0/2` | `0/6/0/19` | Cosmic Flows, Local Void, Great Attractor & Laniakea Rim (`z=05..09`) |
| [`PKG-ACT2-EXP08-Z00-03`](#pkg-act2-exp08-z00-03) | `08` | `Z00..Z03` (4) | `20` | Phase 3 · **Medium-High** | `0/2/2/0` | `0/6/4/10` | Laniakea Divide, Inter-Supercluster Space, Filaments & Nodes (`z=00..03`) |
| [`PKG-ACT2-EXP08-Z04-07`](#pkg-act2-exp08-z04-07) | `08` | `Z04..Z07` (4) | `20` | Phase 3 · **High** | `0/2/1/1` | `0/8/2/10` | Cosmic Voids, Galactic Walls, Junctions & Supercluster Complexes (`z=04..07`) |
| [`PKG-ACT2-EXP08-Z08-10`](#pkg-act2-exp08-z08-10) | `08` | `Z08..Z10` (3) | `15` | Phase 3 · **Medium-High** | `0/2/0/1` | `0/5/0/10` | Deep Cosmic Web, Homogeneity Scale & Large-Scale Architecture (`z=08..10`) |
| [`PKG-ACT2-EXP09-Z00-03`](#pkg-act2-exp09-z00-03) | `09` | `Z00..Z03` (4) | `20` | Phase 3 · **High** | `0/1/1/2` | `0/3/1/16` | Large-Scale Continuity, Deep Field, Lookback Cone & High-z Galaxies (`z=00..03`) |
| [`PKG-ACT2-EXP09-Z04-07`](#pkg-act2-exp09-z04-07) | `09` | `Z04..Z07` (4) | `20` | Phase 3 · **High** | `0/1/3/0` | `0/2/4/14` | Quasar Era, First Galaxies, Reionization & Cosmic Dark Ages (`z=04..07`) |
| [`PKG-ACT2-EXP09-Z08-10`](#pkg-act2-exp09-z08-10) | `09` | `Z08..Z10` (3) | `15` | Phase 3 · **High** | `0/3/0/0` | `0/7/1/7` | CMB Last-Scattering Shell, Cosmological Horizon & Observable Limit (`z=08..10`) |
| [`PKG-ACT2-EXP10-Z00-04`](#pkg-act2-exp10-z00-04) | `10` | `Z00..Z04` (5) | `25` | Phase 3 + Phase 4 · **Critical** | `0/1/1/3` | `0/3/3/19` | CMB Stretched Sky, Recombination Fog, Plasma Acoustic Oscillations, BBN & Leptons (`z=00..04`) |
| [`PKG-ACT2-EXP10-Z05-08`](#pkg-act2-exp10-z05-08) | `10` | `Z05..Z08` (4) | `21` | Phase 3 + Phase 4 · **Critical** | `0/1/3/0` | `0/3/5/13` | Quark-Gluon Plasma (Mode 3), Electroweak Higgs (Mode 5), Inflation & Known-Physics Limit (`z=05..08`) |

---

## 4. Experience-by-Experience Gap Maps & Pick-Up Cards (`ODYSSEY_ACTS[1]`)

### Exp 05: `05_milky_way` — Vía Láctea / Milky Way (`13` Zones, `65` Features, `13` Live Landmarks — **`100% [TIER-A]` COMPLETE**)

> *Espectro de la Vía Láctea · Del sistema solar al borde galáctico* — Spectrum of the Milky Way · From the Solar System to the Galactic Edge

- **Odyssey Position (`odyssey.rs#L29-L32`):** Act 2, Position **1 / 8** (Chapter `act2_exp05`)
- **Spatial & Scale Band:** `SectorId::MILKY_WAY` · `u ∈ [13.6, 21.1]` · `focus_scale_exp = 20.5`
- **Compute & Far Shell:** `mode = 0` (Milky Way 4-arm logarithmic spiral density wave + vertical disk harmonic, `compute_universe.wgsl:91-111`, `universe_scene.rs:307`) · `mw_alpha ≈ 1.0` narrow tilted galactic belt + pinpoint blackbody stars (`far_shell.wgsl:82-103`)
- **Current Render Dispatch (`[COMPLETED — TIER-A]`):** Canonical zone hero dispatch (`doc_stages.rs` `is_zone_migrated_tier_a((5, 0..=12))` -> `1500..=1512`) + canonical `13` `GalacticLandmark` entities (`3660..=3671` for `Z00..=Z11` + synthesized 13th `Z12 galactic-edge` landmark routed via `1512` in `assets.rs` / `doc_stages.rs`) with active landmark LOD promotion/floor clamping and zone-aware macro modulation + `shade_doc_milky_way_stage` in `hero_doc_cosmos.wgsl` across all `65/65` features (`render_tests.rs` Section 6 GPU-verified).
- **Live Audit Tally (`[COMPLETED — TIER-A]`):** Zones `13A / 0B / 0C / 0D` (`13`) · Features `65A / 0B / 0C / 0D` (`65`)
- **`DOMAIN_CUE_KEYWORDS` (`resolver.rs`):** Added `40` bilingual `Exp 05` tokens (`frontera`, `heliosfera`, `oort`, `alfa`, `centauri`, `burbuja`, `orion`, `nebulosa`, `espiral`, `halo`, `disco`, `nucleo`, plus EN/ES companion cues).

#### Baked Landmarks Cross-Reference (`universe_catalog.v1.json` -> Companion IDs `3660..=3671`)

| `lm` | Landmark `id` | `zone_index` | `p` · `lane` · `size` | Current Emitted `vid` (`doc_stages.rs`) | Reserved Companion `vid` |
|---:|---|---:|---|---|---|
| `LM00` | `sun` (`frontier`) | `0` | `p=0.048`, `lane=-0.1`, `size=20` | `20` (`z%3=0`) | `3660` (`vid=1500` synced) |
| `LM01` | `heliosphere` (`heliosphere`) | `1` | `p=0.122`, `lane=0.08`, `size=58` | `21` (`z%3=1`) | `3661` (`vid=1501` synced) |
| `LM02` | `oort` (`oort`) | `2` | `p=0.218`, `lane=-0.05`, `size=72` | `22` (`z%3=2`) | `3662` (`vid=1502` synced) |
| `LM03` | `interstellar` (`interstellar`) | `3` | `p=0.305`, `lane=0.12`, `size=52` | `20` (`z%3=0`) | `3663` (`vid=1503` synced) |
| `LM04` | `alpha` (`alpha-centauri`) | `4` | `p=0.392`, `lane=-0.1`, `size=29` | `21` (`z%3=1`) | `3664` (`vid=1504` synced) |
| `LM05` | `neighborhood` (`neighborhood`) | `5` | `p=0.482`, `lane=0.13`, `size=62` | `22` (`z%3=2`) | `3665` (`vid=1505` synced) |
| `LM06` | `bubble` (`local-bubble`) | `6` | `p=0.578`, `lane=-0.07`, `size=78` | `20` (`z%3=0`) | `3666` (`vid=1506` synced) |
| `LM07` | `orion` (`orion-spur`) | `7` | `p=0.666`, `lane=0.08`, `size=84` | `21` (`z%3=1`) | `3667` (`vid=1507` synced) |
| `LM08` | `nebula` (`stellar-nursery`) | `8` | `p=0.748`, `lane=-0.12`, `size=54` | `22` (`z%3=2`) | `3668` (`vid=1508` synced) |
| `LM09` | `disk` (`milky-way-disk`) | `9` | `p=0.832`, `lane=0.08`, `size=94` | `20` (`z%3=0`) | `3669` (`vid=1509` synced) |
| `LM10` | `center` (`galactic-center`) | `10` | `p=0.909`, `lane=-0.06`, `size=40` | `21` (`z%3=1`) | `3670` (`vid=1510` synced) |
| `LM11` | `halo` (`halo`) | `11` | `p=0.963`, `lane=0.04`, `size=84` | `22` (`z%3=2`) | `3671` (`vid=1511` synced) |
| `LM12` | `galactic-edge` (`galactic-edge`, synthesized in `assets.rs`) | `12` | `p=0.985`, `lane=-0.05`, `size=110` | `1512` (promoted primary/edge rim) | Primary `vid=1512` (`submode=0..1`, `65/65` synced) |

<a id="pkg-act2-exp05-z00-04"></a>
### PKG-ACT2-EXP05-Z00-04: Milky Way Local Vicinity & Solar Neighborhood (`z=00..04` + Landmark `z%3` Fix) `[COMPLETED — TIER-A]`
- **Status & Rollout:** **`[COMPLETED — TIER-A]`** · Phase 1 + Phase 3 (`1500..=1504` + `3660..=3664`, active landmark LOD promotion/floor clamping, `25/25` `doc_feat 0..=4` wired & GPU-verified) · Priority: **High** (`Completed`)

**Audited State (`[COMPLETED — TIER-A]`):** Zones `Z00..Z04` (`5` zones, `25` features) | Live Zone Tiers `5A / 0B / 0C / 0D` *(up from `0A / 0B / 5C / 0D`)* | Live Feature Tiers `25A / 0B / 0C / 0D` *(up from `0A / 0B / 25C / 0D`)* | Live render path: [`doc_stages.rs`](../../crates/cosmos_render/src/doc_stages.rs) (`1500..=1504`, companions `3660..=3664`) -> `shade_doc_milky_way_stage` in [`shaders/hero_doc_cosmos.wgsl`](../../shaders/hero_doc_cosmos.wgsl), GPU-verified in [`render_tests.rs`](../../crates/cosmos_render/tests/render_tests.rs) Section 6.

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z00` | `frontier` | Frontera planetaria / *Planetary boundary* | **`1500`** | `3660` (`LM00` `sun`), `submode=0..1` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z01` | `heliosphere` | Heliosfera / *Heliosphere* | **`1501`** | `3661` (`LM01` `heliosphere`), `submode=0..1` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z02` | `oort` | Nube de Oort / *Oort Cloud* | **`1502`** | `3662` (`LM02` `oort`), `submode=0..1` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z03` | `interstellar` | Espacio interestelar / *interstellar space* | **`1503`** | `3663` (`LM03` `interstellar`), `submode=0..1` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z04` | `alpha-centauri` | Alfa Centauri / *Alpha Centauri* | **`1504`** | `3664` (`LM04` `alpha`), `submode=0..1` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 05 Z00` `frontier` — Frontera planetaria / *Planetary boundary* (`[TIER-C]` · Target `vid=1500`)
- **Metric (`raw_metric`):** "Más allá de Plutón · unos 40 UA del Sol" (*Beyond Pluto · about 40 AU from the Sun*) · `range=[0.000, 0.075]`
- **Spoken Macro (ES // EN):** Aquí termina el mapa planetario anterior. El Sol ya parece una estrella muy brillante, pero su influencia todavía llena el espacio. // *This is where the previous planetary map ends. The Sun already seems like a very bright star, but its influence still fills space.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** MODULO MISMATCH: `LM[0] id=sun -> frontier` has `zone_index=0`, `0%3=0` -> **`vid=20` (Heliopause Bow Shock)** (`doc_stages.rs:360`, `hero_doc_cosmos.wgsl:573-578`) + Kerr BH / Barred MW `ExperienceAnchor` (`vid=5`) + Compute Mode 0 spiral!
- **Audit Gap:** Planetary boundary beyond Pluto (~40 AU) wrongly renders the Heliopause Bow Shock (which belongs to `z=1`) plus a supermassive BH disk. Zero Pluto dwarf planet, zero Kuiper Belt icy population, zero shrinking 40 AU Sun, zero eccentric orbits.
- **Zone 3D Procedural Blueprint:** Decouple `emit_celestial_or_landmark` from `zone_index % 3` by mapping explicit landmark/zone ID -> `vid`; give `frontier` (`z=0`) an icy outer-solar diorama (`vid=1500`): bright pinpoint 40 AU Sun + hairline inclined Pluto-Charon binary orbit + scattered `kind=2` icy Kuiper Belt torus (`30-50 AU`) with `doc_feat` (`0..4`) spotlighting Pluto, Kuiper torus, distant Sun glare, century-long orbital sweep, and tenuous boundary fade.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **Plutón** / *Pluto* | `[TIER-C]` | El planeta enano queda atrás mientras la ruta continúa hacia regiones mucho más grandes y vacías. // *The dwarf planet is left behind as the route continues towards much larger and emptier regions.* | Wrongly routed through `0 % 3 == 0` -> `vid=20` Heliopause Bow Shock + Kerr BH `vid=5` anchor. | Highlight inclined Pluto-Charon binary dwarf orbit at r~40 AU (`doc_feat==0`). |
| `F1` | **Cinturón de Kuiper** / *Kuiper Belt* | `[TIER-C]` | Una extensa población de mundos helados rodea al Sol más allá de la órbita de Neptuno. // *An extensive population of icy worlds surrounds the Sun beyond the orbit of Neptune.* | Wrongly routed through `0 % 3 == 0` -> `vid=20` Heliopause Bow Shock + Kerr BH `vid=5` anchor. | Illuminate scattered icy Kuiper Belt debris ring (30-50 AU, `kind=2` facets, `doc_feat==1`). |
| `F2` | **Sol lejano** / *distant sun* | `[TIER-C]` | Desde aquí, el Sol se ve pequeño, aunque sigue siendo con diferencia el objeto más brillante del cielo. // *From here, the Sun looks small, although it is still by far the brightest object in the sky.* | Wrongly routed through `0 % 3 == 0` -> `vid=20` Heliopause Bow Shock + Kerr BH `vid=5` anchor. | Pulse shrinking pinpoint 40 AU Sun intensity glare vs planet disk size (`doc_feat==2`). |
| `F3` | **órbita** / *orbit* | `[TIER-C]` | La gravedad del Sol todavía mantiene ligados a innumerables cuerpos que tardan siglos en completar una vuelta. // *The Sun's gravity still binds countless bodies that take centuries to complete one revolution.* | Wrongly routed through `0 % 3 == 0` -> `vid=20` Heliopause Bow Shock + Kerr BH `vid=5` anchor. | Trace hairline century-period outer Keplerian orbital ellipses (`doc_feat==3`). |
| `F4` | **espacio exterior** / *outer space* | `[TIER-C]` | No existe una pared al final del sistema planetario: sus regiones se vuelven gradualmente más tenues. // *There is no wall at the end of the planetary system: its regions gradually become fainter.* | Wrongly routed through `0 % 3 == 0` -> `vid=20` Heliopause Bow Shock + Kerr BH `vid=5` anchor. | Fade outer Kuiper density smoothly into pure void without sharp wall (`doc_feat==4`). |

#### `Exp 05 Z01` `heliosphere` — Heliosfera / *Heliosphere* (`[TIER-C]` · Target `vid=1501`)
- **Metric (`raw_metric`):** "Burbuja del viento solar · frontera variable" (*Solar-wind bubble · variable boundary*) · `range=[0.075, 0.170]`
- **Spoken Macro (ES // EN):** El viento solar forma una enorme burbuja alrededor del Sol. Su borde cambia de forma según el medio interestelar. // *The solar wind forms a huge bubble around the Sun. Its edge changes shape depending on the interstellar medium.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** MODULO MISMATCH: `LM[1] id=heliosphere` has `zone_index=1`, `1%3=1` -> **`vid=21` (Orion/Eagle Star-Forming Creation Pillars!!)** (`doc_stages.rs:361`, `hero_doc_cosmos.wgsl:579-588`) + Kerr BH `vid=5`.
- **Audit Gap:** CRITICAL OFF-BY-ONE BUG: Dedicated Heliopause Bow Shock shader (`vid=20`) ALREADY EXISTS at `hero_doc_cosmos.wgsl:573-578`, yet `heliosphere` hits `1 % 3 = 1` and renders star-forming pillars (`vid=21`)! Also lacks Voyager trajectories, termination shock inner shell, and turbulent heliosheath.
- **Zone 3D Procedural Blueprint:** Route `(5, "heliosphere")` explicitly to upgraded `vid=20`: add concentric inner `termination shock` teardrop shell (`r ~ 0.35`, `f=1`), swirling curl-noise `heliosheath` between termination shock and bow `heliopause` (`f=2,3`), radially streaming solar-wind vectors (`f=0`), and twin golden `Voyager 1 & 2` probe trajectories piercing the bow shock nose (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **viento solar** / *solar wind* | `[TIER-C]` | Partículas cargadas salen continuamente del Sol y atraviesan el espacio en todas direcciones. // *Charged particles continually leave the Sun and traverse space in all directions.* | Misrouted by `1 % 3 == 1` to `vid=21` (Creation Pillars) instead of existing `vid=20` Heliopause. | Animate radially streaming high-speed solar-wind proton vectors (`doc_feat==0`). |
| `F1` | **choque de terminación** / *termination shock* | `[TIER-C]` | El viento solar se frena bruscamente cuando comienza a sentir la presión del espacio interestelar. // *The solar wind slows sharply when it begins to feel the pressure of interstellar space.* | Misrouted by `1 % 3 == 1` to `vid=21` (Creation Pillars) instead of existing `vid=20` Heliopause. | Isolate inner spherical termination-shock slowdown discontinuity (`r ~ 0.35`, `doc_feat==1`). |
| `F2` | **heliofunda** / *heliosheath* | `[TIER-C]` | Entre el choque de terminación y la heliopausa, el viento solar avanza más lento y se vuelve turbulento. // *Between the termination shock and the heliopause, the solar wind progresses more slowly and becomes turbulent.* | Misrouted by `1 % 3 == 1` to `vid=21` (Creation Pillars) instead of existing `vid=20` Heliopause. | Swirl turbulent curl-noise heliosheath shell between shocks (`doc_feat==2`). |
| `F3` | **heliopausa** / *heliopause* | `[TIER-C]` | Esta frontera marca el punto donde domina el medio interestelar en lugar del viento solar. // *This boundary marks the point where the interstellar medium dominates instead of the solar wind.* | Misrouted by `1 % 3 == 1` to `vid=21` (Creation Pillars) instead of existing `vid=20` Heliopause. | Light teardrop heliopause bow-shock pressure boundary (`vid=1501 / comp=3661`, `doc_feat==3`). |
| `F4` | **Voyager** / *Voyager* | `[TIER-C]` | Las sondas Voyager han cruzado la heliopausa y continúan enviando datos desde el espacio interestelar. // *The Voyager probes have crossed the heliopause and continue sending data from interstellar space.* | Misrouted by `1 % 3 == 1` to `vid=21` (Creation Pillars) instead of existing `vid=20` Heliopause. | Trace twin golden Voyager 1 & 2 trajectory arcs piercing the bow nose (`doc_feat==4`). |

#### `Exp 05 Z02` `oort` — Nube de Oort / *Oort Cloud* (`[TIER-C]` · Target `vid=1502`)
- **Metric (`raw_metric`):** "Se cree que alcanza decenas de miles de UA" (*It is believed to reach tens of thousands of AU*) · `range=[0.170, 0.265]`
- **Spoken Macro (ES // EN):** Una inmensa nube hipotética de cuerpos helados podría envolver al sistema solar y ser la fuente de muchos cometas de período largo. // *An immense, hypothetical cloud of icy bodies may surround the solar system and be the source of many long-period comets.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** MODULO MISMATCH: `LM[2] id=oort` has `zone_index=2`, `2%3=2` -> **`vid=22` (Local Bubble Supernova Shock Cavity)** (`doc_stages.rs:362`, `hero_doc_cosmos.wgsl:589-595`) + Kerr BH `vid=5`.
- **Audit Gap:** Oort Cloud renders an exploded supernova shock shell instead of a vast spherical cloud of ancient icy planetesimals, falling comets, millennia-long ellipses, and perturbing passing stars.
- **Zone 3D Procedural Blueprint:** Build bespoke Oort spherical swarm (`vid=1502`): dim icy micro-facets distributed spherically (`r=0.30..0.92`, `f=1,3`), pinpoint central Sun (`r=0`), eccentric high-inclination comet dropping sunward with growing cyan ion + white dust tail (`f=0,2`), and bright passing neighbor star warping outer shell gravitational trajectories (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **cometa** / *comet* | `[TIER-C]` | Un pequeño mundo de hielo puede caer hacia el Sol y formar una coma brillante y una larga cola. // *A small world of ice can fall towards the Sun and form a bright coma and a long tail.* | Misrouted by `2 % 3 == 2` to `vid=22` (Local Bubble SN Cavity) + Kerr BH `vid=5`. | Animate eccentric high-inclination comet diving sunward with cyan ion + white dust tail (`doc_feat==0`). |
| `F1` | **hielo antiguo** / *ancient ice* | `[TIER-C]` | Estos cuerpos conservarían materiales muy poco alterados desde el nacimiento del sistema solar. // *These bodies may preserve materials that have changed very little since the solar system formed.* | Misrouted by `2 % 3 == 2` to `vid=22` (Local Bubble SN Cavity) + Kerr BH `vid=5`. | Glint pristine cryogenic Oort icy planetesimal micro-facets (`doc_feat==1`). |
| `F2` | **órbita lejana** / *far orbit* | `[TIER-C]` | Una vuelta alrededor del Sol puede durar miles, millones o incluso más años. // *One orbit around the Sun can take thousands, millions, or even more years.* | Misrouted by `2 % 3 == 2` to `vid=22` (Local Bubble SN Cavity) + Kerr BH `vid=5`. | Trace 10,000-AU orbital ellipse sweeping far from pinpoint central Sun (`doc_feat==2`). |
| `F3` | **nube esférica** / *spherical cloud* | `[TIER-C]` | A diferencia del cinturón de Kuiper, esta región se imagina rodeando al Sol en casi todas las direcciones. // *Unlike the Kuiper Belt, this region is imagined surrounding the Sun in almost all directions.* | Misrouted by `2 % 3 == 2` to `vid=22` (Local Bubble SN Cavity) + Kerr BH `vid=5`. | Reveal 3D isotropic spherical Oort swarm shell (`r=0.30..0.92`, `doc_feat==3`). |
| `F4` | **estrella pasajera** / *passing star* | `[TIER-C]` | La gravedad de una estrella cercana puede alterar lentamente la órbita de un cometa distante. // *The gravity of a nearby star can slowly alter the orbit of a distant comet.* | Misrouted by `2 % 3 == 2` to `vid=22` (Local Bubble SN Cavity) + Kerr BH `vid=5`. | Sweep bright passing star trajectory warping outer Oort trajectories (`doc_feat==4`). |

#### `Exp 05 Z03` `interstellar` — Espacio interestelar / *interstellar space* (`[TIER-C]` · Target `vid=1503`)
- **Metric (`raw_metric`):** "Entre las estrellas · materia extremadamente dispersa" (*Between the stars · extremely sparse matter*) · `range=[0.265, 0.350]`
- **Spoken Macro (ES // EN):** El espacio interestelar no está completamente vacío: contiene gas, polvo, campos magnéticos y partículas de alta energía. // *Interstellar space is not completely empty: it contains gas, dust, magnetic fields and high-energy particles.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** MODULO MISMATCH: `LM[3] id=interstellar` (`z=3`, `3%3=0`) -> **`vid=20` (Heliopause Bow Shock)** (`doc_stages.rs:360`) + Kerr BH `vid=5` + Mode 0 disk.
- **Audit Gap:** Interstellar space between stars collapses back onto Sun's Heliopause Bow Shock. Missing diffuse multi-phase ISM gas, infrared/dark cosmic dust extinction clouds, relativistic cosmic rays spiraling around weak galactic magnetic field lines, and the Local Interstellar Cloud envelope.
- **Zone 3D Procedural Blueprint:** Build bespoke ISM medium volume (`vid=1503`): wispy FBM HI/HII gas (`f=0`) + light-absorbing dust filaments (`f=1`), helical galactic magnetic field lines (`curl_noise3`, `f=3`) guiding glowing relativistic cosmic-ray streaks (`f=2`), and solar system drifting through the slightly denser warm Local Interstellar Cloud puff (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **gas interestelar** / *interstellar gas* | `[TIER-C]` | Átomos y moléculas muy separados llenan el enorme volumen que existe entre las estrellas. // *Widely spaced atoms and molecules fill the enormous volume between the stars.* | Misrouted by `3 % 3 == 0` to `vid=20` (Heliopause Bow Shock) + Kerr BH `vid=5`. | Pulse wispy multi-octave FBM HI/HII interstellar gas filaments (`doc_feat==0`). |
| `F1` | **polvo cósmico** / *cosmic dust* | `[TIER-C]` | Granos diminutos absorben y dispersan luz, y también ayudan a formar nuevas estrellas y planetas. // *Tiny grains absorb and scatter light, and also help form new stars and planets.* | Misrouted by `3 % 3 == 0` to `vid=20` (Heliopause Bow Shock) + Kerr BH `vid=5`. | Darken cold light-absorbing interstellar silicate/PAH dust lanes (`doc_feat==1`). |
| `F2` | **rayo cósmico** / *cosmic ray* | `[TIER-C]` | Partículas muy energéticas viajan por la galaxia a velocidades cercanas a la de la luz. // *High-energy particles travel through the galaxy at nearly the speed of light.* | Misrouted by `3 % 3 == 0` to `vid=20` (Heliopause Bow Shock) + Kerr BH `vid=5`. | Streak glowing relativistic cosmic-ray paths along field lines (`doc_feat==2`). |
| `F3` | **campo magnético** / *magnetic field* | `[TIER-C]` | Campos débiles pero extensos guían partículas cargadas a través del medio interestelar. // *Weak but extensive fields guide charged particles through the interstellar medium.* | Misrouted by `3 % 3 == 0` to `vid=20` (Heliopause Bow Shock) + Kerr BH `vid=5`. | Trace helical interstellar magnetic flux tubes (`curl_noise3`, `doc_feat==3`). |
| `F4` | **nube interestelar local** / *local interstellar cloud* | `[TIER-C]` | El sistema solar atraviesa actualmente una pequeña región de gas dentro del vecindario galáctico. // *The solar system is currently passing through a small region of gas within the galactic neighborhood.* | Misrouted by `3 % 3 == 0` to `vid=20` (Heliopause Bow Shock) + Kerr BH `vid=5`. | Highlight Solar System traversing warmer Local Interstellar Cloud puff (`doc_feat==4`). |

#### `Exp 05 Z04` `alpha-centauri` — Alfa Centauri / *Alpha Centauri* (`[TIER-C]` · Target `vid=1504`)
- **Metric (`raw_metric`):** "Próxima Centauri · 4,24 años luz" (*Proxima Centauri · 4.24 light-years*) · `range=[0.350, 0.435]`
- **Spoken Macro (ES // EN):** El sistema estelar más cercano al Sol contiene tres estrellas. Próxima Centauri es la más cercana de todas. // *The star system closest to the Sun contains three stars. Proxima Centauri is the closest of all.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** MODULO MISMATCH: `LM[4] id=alpha -> alpha-centauri` (`z=4`, `4%3=1`) -> **`vid=21` (Orion/Eagle Creation Pillars)** (`doc_stages.rs:361`) + Kerr BH `vid=5`.
- **Audit Gap:** Closest star system renders gas-pillar stellar nursery instead of the hierarchical triple system: yellow G2V Alpha Centauri A + orange K1V Alpha Centauri B binary, distant orbiting M5.5V Proxima Centauri red dwarf, Proxima b exoplanet, and 4.24 ly light-travel ruler.
- **Zone 3D Procedural Blueprint:** Build bespoke triple-star SDF (`vid=1504`): Keplerian-orbiting luminous blackbody spheres A (5790K) & B (5260K) (`f=1,2`) + wider-orbit flaring red dwarf Proxima (3040K, `f=0`) orbited closely by rocky habitable-zone exoplanet (`f=3`), plus animated concentric 1-ly/4.24-ly light-sphere pulse from Sun (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **Próxima Centauri** / *Proxima Centauri* | `[TIER-C]` | Es una pequeña enana roja y la estrella conocida más cercana al Sol. // *It is a small red dwarf and the nearest known star to the Sun.* | Misrouted by `4 % 3 == 1` to `vid=21` (Creation Pillars) + Kerr BH `vid=5`. | Spotlight faint flaring red dwarf Proxima Centauri (3040K) on outer orbit (`doc_feat==0`). |
| `F1` | **Alfa Centauri A** / *Alpha Centauri A* | `[TIER-C]` | Esta estrella amarilla se parece al Sol y forma una pareja con Alfa Centauri B. // *This yellow star resembles the Sun and forms a pair with Alpha Centauri B.* | Misrouted by `4 % 3 == 1` to `vid=21` (Creation Pillars) + Kerr BH `vid=5`. | Spotlight luminous G-type Alpha Centauri A (5790K) primary (`doc_feat==1`). |
| `F2` | **Alfa Centauri B** / *Alpha Centauri B* | `[TIER-C]` | Es algo más pequeña y fría que el Sol, y orbita junto a Alfa Centauri A. // *It is somewhat smaller and colder than the Sun, and orbits alongside Alpha Centauri A.* | Misrouted by `4 % 3 == 1` to `vid=21` (Creation Pillars) + Kerr BH `vid=5`. | Highlight K-type Alpha Centauri B (5260K) tight binary orbit with A (`doc_feat==2`). |
| `F3` | **exoplaneta** / *exoplanet* | `[TIER-C]` | Alrededor de Próxima se han detectado mundos que orbitan una estrella distinta del Sol. // *Worlds orbiting a star other than the Sun have been detected around Proxima.* | Misrouted by `4 % 3 == 1` to `vid=21` (Creation Pillars) + Kerr BH `vid=5`. | Zoom rocky habitable-zone exoplanet (Proxima b) orbit + terminator glow (`doc_feat==3`). |
| `F4` | **año luz** / *light year* | `[TIER-C]` | Es la distancia que recorre la luz en un año, no una medida de tiempo. // *It is the distance that light travels in a year, not a measurement of time.* | Misrouted by `4 % 3 == 1` to `vid=21` (Creation Pillars) + Kerr BH `vid=5`. | Expand concentric 1-ly / 4.24-ly light-sphere timing rings from Sun (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP05-Z00-04)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 05 (05_milky_way) Zones Z00..Z04 (`frontier`, `heliosphere`, `oort`, `interstellar`, `alpha-centauri`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z00->1500`, `Z01->1501`, `Z02->1502`, `Z03->1503`, `Z04->1504` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Replace legacy modulo/static landmark dispatch with explicit companion IDs: LM00(sun)->3660, LM01(heliosphere)->3661, LM02(oort)->3662, LM03(interstellar)->3663, LM04(alpha)->3664.
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `frontera`, `heliosfera`, `oort`, `alfa`, `centauri`, `burbuja`, `orion`, `nebulosa`, `espiral`, `halo`, `disco`, `nucleo` (`sagitario` already exists!); verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z00->1500`, `Z01->1501`, `Z02->1502`, `Z03->1503`, `Z04->1504`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z00 (vid=1500, frontier): F0:Pluto; F1:Kuiper Belt; F2:distant sun; F3:orbit; F4:outer space
     * Z01 (vid=1501, heliosphere): F0:solar wind; F1:termination shock; F2:heliosheath; F3:heliopause; F4:Voyager
     * Z02 (vid=1502, oort): F0:comet; F1:ancient ice; F2:far orbit; F3:spherical cloud; F4:passing star
     * Z03 (vid=1503, interstellar): F0:interstellar gas; F1:cosmic dust; F2:cosmic ray; F3:magnetic field; F4:local interstellar cloud
     * Z04 (vid=1504, alpha-centauri): F0:Proxima Centauri; F1:Alpha Centauri A; F2:Alpha Centauri B; F3:exoplanet; F4:light year
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp05-z05-08"></a>
### PKG-ACT2-EXP05-Z05-08: Local Bubble, Orion Spur & Stellar Nurseries (`z=05..08`) `[COMPLETED — TIER-A]`
- **Status & Rollout:** **`[COMPLETED — TIER-A]`** · Phase 1 + Phase 3 (`1505..=1508` + `3665..=3668`, Local Bubble supernova cavity, Orion Spur stellar filaments, stellar nursery pillar/IMF/feedback shaders, `20/20` `doc_feat 0..=4` wired & GPU-verified) · Priority: **High** (`Completed`)

**Audited State (`[COMPLETED — TIER-A]`):** Zones `Z05..Z08` (`4` zones, `20` features) | Live Zone Tiers `4A / 0B / 0C / 0D` *(up from `0A / 1B / 3C / 0D`)* | Live Feature Tiers `20A / 0B / 0C / 0D` *(up from `0A / 3B / 15C / 2D`)* | Live render path: [`doc_stages.rs`](../../crates/cosmos_render/src/doc_stages.rs) (`1505..=1508`, companions `3665..=3668`) -> `shade_doc_milky_way_stage` in [`shaders/hero_doc_cosmos.wgsl`](../../shaders/hero_doc_cosmos.wgsl), GPU-verified in [`render_tests.rs`](../../crates/cosmos_render/tests/render_tests.rs) Section 6.

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z05` | `neighborhood` | Vecindario estelar / *Stellar neighborhood* | **`1505`** | `3665` (`LM05` `neighborhood`), `submode=0..1` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z06` | `local-bubble` | Burbuja Local / *Local Bubble* | **`1506`** | `3666` (`LM06` `bubble`), `submode=0..1` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z07` | `orion-spur` | Espolón de Orión / *Orion Spur* | **`1507`** | `3667` (`LM07` `orion`), `submode=0..1` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z08` | `stellar-nursery` | Guardería estelar / *stellar nursery* | **`1508`** | `3668` (`LM08` `nebula`), `submode=0..1` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 05 Z05` `neighborhood` — Vecindario estelar / *Stellar neighborhood* (`[TIER-C]` · Target `vid=1505`)
- **Metric (`raw_metric`):** "Decenas de años luz alrededor del Sol" (*Tens of light years around the Sun*) · `range=[0.435, 0.535]`
- **Spoken Macro (ES // EN):** Nuestro vecindario reúne estrellas brillantes, enanas rojas, enanas blancas, mundos invisibles y sistemas muy distintos. // *Our neighborhood brings together bright stars, red dwarfs, white dwarfs, invisible worlds and very different systems.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** MODULO MISMATCH: `LM[5] id=neighborhood` (`z=5`, `5%3=2`) -> **`vid=22` (Local Bubble Supernova Cavity)** (`doc_stages.rs:362`) + Kerr BH `vid=5`.
- **Audit Gap:** Stellar neighborhood (Sirius A+B, high-proper-motion Barnard's Star, white dwarfs, brown dwarfs, multiple star systems) wrongly shows Local Bubble SN cavity.
- **Zone 3D Procedural Blueprint:** Build bespoke 30-ly Stellar Census diorama (`vid=1505`): bright blue-white Sirius A + tiny dense UV white-dwarf Sirius B pair (`f=0,2`), fast proper-motion trail marker on red dwarf Barnard's Star (`f=1`), dim glowing magenta-infrared methane brown dwarf (`f=3`), and hairline binary/triple orbital tethers highlighting star multiplicity (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **Sirio** / *Sirius* | `[TIER-C]` | Es la estrella más brillante del cielo nocturno terrestre y forma un sistema doble. // *It is the brightest star in the Earth's night sky and forms a double system.* | Misrouted by `5 % 3 == 2` to `vid=22` (Local Bubble SN Cavity) + Kerr BH `vid=5`. | Illuminate blazing blue-white Sirius A binary primary (`doc_feat==0`). |
| `F1` | **estrella de Barnard** / *Barnard's star* | `[TIER-C]` | Una enana roja cercana se desplaza por el cielo más rápido que cualquier otra estrella conocida. // *A nearby red dwarf is moving across the sky faster than any other known star.* | Misrouted by `5 % 3 == 2` to `vid=22` (Local Bubble SN Cavity) + Kerr BH `vid=5`. | Trace fast proper-motion tangential velocity trail on Barnard's Star (`doc_feat==1`). |
| `F2` | **enana blanca** / *white dwarf* | `[TIER-C]` | Es el núcleo caliente y denso que puede quedar cuando una estrella parecida al Sol termina su vida. // *It is the hot, dense core that can remain when a Sun-like star ends its life.* | Misrouted by `5 % 3 == 2` to `vid=22` (Local Bubble SN Cavity) + Kerr BH `vid=5`. | Pinpoint ultra-dense UV white dwarf Sirius B companion beside Sirius A (`doc_feat==2`). |
| `F3` | **enana marrón** / *brown dwarf* | `[TIER-C]` | Tiene más masa que un planeta gigante, pero no suficiente para brillar como una estrella normal. // *It has more mass than a giant planet, but not enough to shine like a normal star.* | Misrouted by `5 % 3 == 2` to `vid=22` (Local Bubble SN Cavity) + Kerr BH `vid=5`. | Reveal dim magenta-infrared methane brown dwarf beneath fusion floor (`doc_feat==3`). |
| `F4` | **sistema estelar** / *star system* | `[TIER-C]` | Muchas estrellas viven en parejas o grupos unidos por la gravedad, no solas como el Sol. // *Many stars live in pairs or groups linked by gravity, not alone like the Sun.* | Misrouted by `5 % 3 == 2` to `vid=22` (Local Bubble SN Cavity) + Kerr BH `vid=5`. | Connect multi-star binary/triple pairs with hairline Keplerian tethers (`doc_feat==4`). |

#### `Exp 05 Z06` `local-bubble` — Burbuja Local / *Local Bubble* (`[TIER-C]` · Target `vid=1506`)
- **Metric (`raw_metric`):** "Una cavidad de gas caliente alrededor del Sol" (*A cavity of hot gas around the Sun*) · `range=[0.535, 0.620]`
- **Spoken Macro (ES // EN):** El Sol se encuentra dentro de una amplia región con menos gas frío que sus alrededores, probablemente modelada por antiguas supernovas. // *The Sun lies within a wide region with less cold gas than its surroundings, probably shaped by ancient supernovae.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** MODULO MISMATCH: `LM[6] id=bubble -> local-bubble` (`z=6`, `6%3=0`) -> **`vid=20` (Heliopause Bow Shock!!)** (`doc_stages.rs:360`) + Kerr BH `vid=5`.
- **Audit Gap:** CRITICAL MODULO BUG: Dedicated `vid=22` ('Local Bubble Carved Supernova Shock Cavity Shell', `hero_doc_cosmos.wgsl:589-595`) ALREADY EXISTS, yet `local-bubble` (`z=6`) hits `6 % 3 = 0` and displays Heliopause Bow Shock (`vid=20`)!
- **Zone 3D Procedural Blueprint:** Route `(5, "local-bubble")` explicitly to `vid=22`; upgrade `vid=22` with `doc_feat` highlights: historical supernova blast rings inside cavity (`f=0`), tenuous 10^6 K X-ray hot plasma interior glow (`f=1`), dense star-forming molecular shell wall along Sco-Cen/Taurus/Orion rim (`f=2`), 3D dust-extinction hourglass silhouette (`f=3`), and central Sun trajectory piercing the cavity (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **supernova antigua** / *ancient supernova* | `[TIER-C]` | Explosiones estelares del pasado pudieron calentar y apartar parte del gas de esta región. // *Past stellar explosions could have heated and removed some of the gas in this region.* | Misrouted by `6 % 3 == 0` to `vid=20` (Heliopause) instead of existing `vid=22` Local Bubble Cavity. | Pulse nested ancient supernova blast shockwave rings inside cavity (`doc_feat==0`). |
| `F1` | **plasma caliente** / *hot plasma* | `[TIER-C]` | El gas extremadamente tenue puede alcanzar temperaturas muy altas sin sentirse denso como una atmósfera. // *The extremely tenuous gas can reach very high temperatures without feeling dense like an atmosphere.* | Misrouted by `6 % 3 == 0` to `vid=20` (Heliopause) instead of existing `vid=22` Local Bubble Cavity. | Glow tenuous 10^6 K X-ray hot plasma interior filling Local Bubble (`doc_feat==1`). |
| `F2` | **pared de gas** / *gas wall* | `[TIER-C]` | En el contorno de la burbuja se acumulan nubes más densas donde pueden nacer estrellas. // *Around the bubble, denser clouds accumulate where stars can be born.* | Misrouted by `6 % 3 == 0` to `vid=20` (Heliopause) instead of existing `vid=22` Local Bubble Cavity. | Brighten swept-up star-forming cold molecular rim along Sco-Cen/Taurus/Orion wall (`doc_feat==2`). |
| `F3` | **mapa tridimensional** / *three-dimensional map* | `[TIER-C]` | El polvo y la luz de muchas estrellas permiten reconstruir la forma irregular de nuestro entorno. // *The dust and light of many stars allow us to reconstruct the irregular shape of our environment.* | Misrouted by `6 % 3 == 0` to `vid=20` (Heliopause) instead of existing `vid=22` Local Bubble Cavity. | Reveal 3D hourglass dust-extinction tomography wireframe slice (`doc_feat==3`). |
| `F4` | **vecindario del Sol** / *neighborhood of the Sun* | `[TIER-C]` | La Tierra viaja con el Sol a través de esta cavidad mientras toda la galaxia gira. // *The Earth travels with the Sun through this cavity as the entire galaxy rotates.* | Misrouted by `6 % 3 == 0` to `vid=20` (Heliopause) instead of existing `vid=22` Local Bubble Cavity. | Trace Solar trajectory passing inside off-center quiet cavity core (`doc_feat==4`). |

#### `Exp 05 Z07` `orion-spur` — Espolón de Orión / *Orion Spur* (`[TIER-B]` · Target `vid=1507`)
- **Metric (`raw_metric`):** "Nuestra dirección dentro del disco galáctico" (*Our direction within the galactic disk*) · `range=[0.620, 0.710]`
- **Spoken Macro (ES // EN):** El sistema solar vive en una estructura menor situada entre dos grandes brazos espirales de la Vía Láctea. // *The solar system lies in a smaller structure between two major spiral arms of the Milky Way.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `LM[7] id=orion -> orion-spur` (`z=7`, `7%3=1`) -> **`vid=21` (Orion/Eagle Creation Pillars)** + `ExperienceAnchor` (`vid=5` 4-arm MW disk) + Compute Mode 0 spiral stars.
- **Audit Gap:** Fortuitous `7 % 3 = 1` match displays Orion pillars + `vid=5` spiral arms, but `vid=21` depicts a nebula pillar rather than the 3,500×10,000 ly Orion Spur minor spiral bridge between Sagittarius & Perseus arms, Sun marker, or galactic orbital track (`230 Myr`). `doc_feat` unused.
- **Zone 3D Procedural Blueprint:** Add `z_idx == 7u` spotlight on `vid=5` barred spiral disk + `vid=1507` Orion Spur zoom inset: highlight inter-arm Spur arc (`f=0`), glowing golden Sun position marker at `R ~ 8.1 kpc` (`f=1`), cold molecular clouds along Spur (`f=2`), young dispersing blue open clusters (`f=3`), and 230 Myr circular Solar-orbit arrow (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **brazo espiral** / *spiral arm* | `[TIER-B]` | Una franja de estrellas, gas y polvo destaca dentro del disco de una galaxia espiral. // *A strip of stars, gas and dust stands out within the disk of a spiral galaxy.* | `vid=5` spiral arms + Mode 0 density wave depict galactic arms. | Highlight inter-arm Orion Spur arc bridging Sagittarius and Perseus arms (`doc_feat==0`). |
| `F1` | **Sol** / *Sun* | `[TIER-D]` | Nuestra estrella es solo una entre cientos de miles de millones que forman la Vía Láctea. // *Our star is just one of the hundreds of billions of stars that make up the Milky Way.* | No highlighted Sun beacon positioned on Orion Spur. | Pulse golden Sun beacon marker at galactocentric R ~ 8.1 kpc (`doc_feat==1`). |
| `F2` | **nube molecular** / *molecular cloud* | `[TIER-B]` | El gas frío y denso puede colapsar por gravedad y comenzar a producir nuevas estrellas. // *The cold, dense gas can collapse under gravity and begin producing new stars.* | `vid=21` dense molecular dust pillars depict molecular clouds. | Condense dark-rimmed cold molecular star-forming clouds along Spur (`doc_feat==2`). |
| `F3` | **cúmulo abierto** / *open star cluster* | `[TIER-B]` | Un grupo joven de estrellas nace de la misma nube y se dispersa lentamente con el tiempo. // *A young group of stars is born from the same cloud and slowly disperses over time.* | `stars` newborn bright cluster sparkles inside `vid=21`. | Scatter dispersing sapphire open star clusters born in shared Spur knots (`doc_feat==3`). |
| `F4` | **rotación galáctica** / *galactic rotation* | `[TIER-D]` | El Sol tarda cientos de millones de años en completar una órbita alrededor del centro galáctico. // *It takes hundreds of millions of years for the Sun to complete one orbit around the galactic center.* | No 230-Myr galactic rotation trajectory vector around galactic core. | Animate ~230 Myr circular Solar galactic orbit direction arrow (`doc_feat==4`). |

#### `Exp 05 Z08` `stellar-nursery` — Guardería estelar / *stellar nursery* (`[TIER-C]` · Target `vid=1508`)
- **Metric (`raw_metric`):** "Nebulosas, cúmulos y estrellas recién nacidas" (*Nebulae, clusters and newborn stars*) · `range=[0.710, 0.790]`
- **Spoken Macro (ES // EN):** En las nubes más densas de los brazos galácticos, la gravedad transforma gas y polvo en nuevas generaciones de estrellas. // *In the densest clouds of the galactic arms, gravity transforms gas and dust into new generations of stars.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** MODULO MISMATCH: `LM[8] id=nebula -> stellar-nursery` (`z=8`, `8%3=2`) -> **`vid=22` (Local Bubble Supernova Cavity!!)** (`doc_stages.rs:362`) + Kerr BH `vid=5`.
- **Audit Gap:** CRITICAL MODULO BUG: `vid=21` ('Orion / Eagle Stellar Nursery Creation Pillars', `hero_doc_cosmos.wgsl:579-588`) ALREADY EXISTS specifically for star birth, yet `stellar-nursery` (`z=8`) hits `8 % 3 = 2` and renders Local Bubble Cavity (`vid=22`)! Also lacks protostar accretion jets, protoplanetary disk, Pleiades reflection nebula, and opaque Barnard dark nebula silhouettes.
- **Zone 3D Procedural Blueprint:** Route `(5, "stellar-nursery")` explicitly to upgraded `vid=21`; add `doc_feat` sub-stages: backlit Orion HII emission nebula (`f=0`), collapsing infrared protostar with bipolar Herbig-Haro jets (`f=1`), Keplerian dusty protoplanetary ring with gap bands (`f=2`), sapphire Pleiades reflection cluster (`f=3`), and opaque cold dark nebula cloud silhouetted against background Milky Way starlight (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **nebulosa de Orión** / *Orion nebula* | `[TIER-C]` | Una gran nube iluminada revela estrellas jóvenes que nacen dentro de gas y polvo. // *A large illuminated cloud reveals young stars being born within gas and dust.* | Misrouted by `8 % 3 == 2` to `vid=22` (Local Bubble) instead of existing `vid=21` Creation Pillars. | Illuminate backlit crimson-cyan Orion (M42) HII emission nebula pillars (`doc_feat==0`). |
| `F1` | **protoestrella** / *protostar* | `[TIER-C]` | Antes de encender la fusión, una estrella crece acumulando materia de la nube que la rodea. // *Before fusion ignites, a star grows by accreting matter from the cloud around it.* | Misrouted by `8 % 3 == 2` to `vid=22` (Local Bubble) instead of existing `vid=21` Creation Pillars. | Ignite collapsing infrared protostar core with bipolar Herbig-Haro jets (`doc_feat==1`). |
| `F2` | **disco protoplanetario** / *protoplanetary disk* | `[TIER-C]` | Un disco de gas y polvo alrededor de una estrella joven puede formar planetas, lunas y asteroides. // *A disk of gas and dust around a young star can form planets, moons and asteroids.* | Misrouted by `8 % 3 == 2` to `vid=22` (Local Bubble) instead of existing `vid=21` Creation Pillars. | Spin dusty Keplerian protoplanetary disk with dark planet-carved gaps (`doc_feat==2`). |
| `F3` | **Pléyades** / *Pleiades* | `[TIER-C]` | Este cúmulo joven reúne estrellas azules brillantes que nacieron aproximadamente juntas. // *This young cluster contains bright blue stars that formed at about the same time.* | Misrouted by `8 % 3 == 2` to `vid=22` (Local Bubble) instead of existing `vid=21` Creation Pillars. | Highlight sapphire Pleiades reflection nebula enveloping young B stars (`doc_feat==3`). |
| `F4` | **nebulosa oscura** / *dark nebula* | `[TIER-C]` | Una nube de polvo puede bloquear la luz de las estrellas que están detrás y parecer un hueco negro. // *A dust cloud can block the light from stars behind it and look like a black void.* | Misrouted by `8 % 3 == 2` to `vid=22` (Local Bubble) instead of existing `vid=21` Creation Pillars. | Silhouette opaque cold dark-nebula dust trunk against starfield (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP05-Z05-08)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 05 (05_milky_way) Zones Z05..Z08 (`neighborhood`, `local-bubble`, `orion-spur`, `stellar-nursery`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z05->1505`, `Z06->1506`, `Z07->1507`, `Z08->1508` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Replace legacy modulo/static landmark dispatch with explicit companion IDs: LM05(neighborhood)->3665, LM06(bubble)->3666, LM07(orion)->3667, LM08(nebula)->3668.
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `frontera`, `heliosfera`, `oort`, `alfa`, `centauri`, `burbuja`, `orion`, `nebulosa`, `espiral`, `halo`, `disco`, `nucleo` (`sagitario` already exists!); verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z05->1505`, `Z06->1506`, `Z07->1507`, `Z08->1508`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z05 (vid=1505, neighborhood): F0:Sirius; F1:Barnard's star; F2:white dwarf; F3:brown dwarf; F4:star system
     * Z06 (vid=1506, local-bubble): F0:ancient supernova; F1:hot plasma; F2:gas wall; F3:three-dimensional map; F4:neighborhood of the Sun
     * Z07 (vid=1507, orion-spur): F0:spiral arm; F1:Sun; F2:molecular cloud; F3:open star cluster; F4:galactic rotation
     * Z08 (vid=1508, stellar-nursery): F0:Orion nebula; F1:protostar; F2:protoplanetary disk; F3:Pleiades; F4:dark nebula
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp05-z09-12"></a>
### PKG-ACT2-EXP05-Z09-12: Galactic Disk, Sgr A* Core, Halo & Galactic Edge (`z=09..12` + 13th Landmark) `[COMPLETED — TIER-A]`
- **Status & Rollout:** **`[COMPLETED — TIER-A]`** · Phase 1 + Phase 3 (`1509..=1512` + `3669..=3671` + synthesized 13th `Z12` landmark in `assets.rs`, barred spiral disk, Sgr A* core, stellar halo & globular clusters, outer disk flare & dark halo rim, `20/20` `doc_feat 0..=4` wired & GPU-verified) · Priority: **High** (`Completed`)

**Audited State (`[COMPLETED — TIER-A]`):** Zones `Z09..Z12` (`4` zones, `20` features) | Live Zone Tiers `4A / 0B / 0C / 0D` *(up from `0A / 2B / 2C / 0D`)* | Live Feature Tiers `20A / 0B / 0C / 0D` *(up from `0A / 6B / 10C / 4D`)* | Live render path: [`doc_stages.rs`](../../crates/cosmos_render/src/doc_stages.rs) (`1509..=1512`, companions `3669..=3671`) -> `shade_doc_milky_way_stage` + zone-aware `is_mw` macro modulation in [`shaders/hero_doc_cosmos.wgsl`](../../shaders/hero_doc_cosmos.wgsl), GPU-verified in [`render_tests.rs`](../../crates/cosmos_render/tests/render_tests.rs) Section 6.

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z09` | `milky-way-disk` | Disco galáctico / *galactic disk* | **`1509`** | `3669` (`LM09` `disk`), `submode=0..1` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z10` | `galactic-center` | Centro galáctico / *galactic center* | **`1510`** | `3670` (`LM10` `center`), `submode=0..1` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z11` | `halo` | Halo galáctico / *galactic halo* | **`1511`** | `3671` (`LM11` `halo`), `submode=0..1` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z12` | `galactic-edge` | Borde de la Vía Láctea / *Edge of the Milky Way* | **`1512`** | `submode=32` (edge rim), `base_flags` bits `2..7` | Exp `05` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 05 Z09` `milky-way-disk` — Disco galáctico / *galactic disk* (`[TIER-B]` · Target `vid=1509`)
- **Metric (`raw_metric`):** "La Vía Láctea mide alrededor de 100.000 años luz de ancho" (*The Milky Way is about 100,000 light-years wide*) · `range=[0.790, 0.875]`
- **Spoken Macro (ES // EN):** La mayor parte de las estrellas visibles, el gas y el polvo forman un disco aplanado con una barra central y brazos espirales. // *Most of the visible stars, gas and dust form a flattened disk with a central bar and spiral arms.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `ExperienceAnchor` `vid=5` (`hero_doc_cosmos.wgsl:462-475` barred 4-arm MW disk + `dust` lanes) + Compute Mode 0 spiral starfield. HOWEVER `LM[9] id=disk` (`z=9`, `9%3=0`) wrongly renders **`vid=20` (Heliopause Bow Shock)** at Hero LOD!
- **Audit Gap:** Zone-scale anchor `vid=5` matches Milky Way disk/bar/dust/stars, EXCEPT (a) center is dominated by an oversized Kerr BH accretion disk (`rs=0.21`), (b) landmark `disk` misfires as Heliopause `vid=20` when zoomed in, (c) zero dark matter halo or flat rotation-curve visual, and (d) `doc_feat` ignored.
- **Zone 3D Procedural Blueprint:** Fix `LM[9]` -> `vid=1509` (bar/disk cutaway without oversized central accretion disk); add `doc_feat` overlays onto `vid=5` / `vid=1509`: edge-on skyband projection toggle (`f=0`), boosted golden inner galactic bar (`f=1`), high-contrast opaque midplane dust rifts (`f=2`), expansive violet NFW dark-matter halo shell + flat rotation curve diagram (`f=3`), and spectral-class stellar population highlight (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **Vía Láctea** / *Milky Way* | `[TIER-B]` | Desde la Tierra vemos el disco de nuestra galaxia de canto como una banda luminosa que cruza el cielo. // *From Earth, we see our galaxy's disk edge-on as a luminous band across the sky.* | `vid=5` disk + `far_shell` band depict Milky Way disk silhouette. | Toggle edge-on full-sky Milky Way band projection overlay (`doc_feat==0`). |
| `F1` | **barra central** / *central bar* | `[TIER-B]` | Una estructura alargada de estrellas atraviesa la región interior de la galaxia. // *An elongated structure of stars runs through the inner region of the galaxy.* | Elongated golden `bar` geometry explicitly rendered in `hero_doc_cosmos.wgsl:466`. | Boost warm golden inner Galactic Bar geometry (`vid=1509`, `doc_feat==1`). |
| `F2` | **carril de polvo** / *dust lane* | `[TIER-B]` | Nubes oscuras ocultan parte de la luz y dibujan grietas dentro de la franja galáctica. // *Dark clouds obscure some of the light and trace dark rifts across the galactic band.* | Procedural `dust` attenuating rifts in `vid=5` and `far_shell.wgsl:93`. | Deepen opaque midplane dust lane rifts cutting through bright spiral arms (`doc_feat==2`). |
| `F3` | **materia oscura** / *dark matter* | `[TIER-D]` | Una forma de materia invisible aporta gran parte de la gravedad que mantiene unida la galaxia. // *An invisible form of matter provides much of the gravity that holds the galaxy together.* | Zero dark-matter invisible halo or galactic rotation-curve cue (`LM[9]` draws Heliopause!). | Overlay expansive violet NFW dark-matter halo + flat rotation-curve gauge (`doc_feat==3`). |
| `F4` | **estrella** / *star* | `[TIER-B]` | Las estrellas del disco tienen edades, masas, colores y sistemas planetarios muy diferentes. // *The stars in the disk have very different ages, masses, colors, and planetary systems.* | Compute Mode 0 advects 3400K–19500K blackbody disk stars. | Sparkle spectral-class O-to-M star population density across thin disk (`doc_feat==4`). |

#### `Exp 05 Z10` `galactic-center` — Centro galáctico / *galactic center* (`[TIER-B]` · Target `vid=1510`)
- **Metric (`raw_metric`):** "A unos 26.000 años luz del Sol" (*About 26,000 light-years from the Sun*) · `range=[0.875, 0.940]`
- **Spoken Macro (ES // EN):** El corazón de la Vía Láctea está lleno de estrellas, gas caliente y fuertes campos magnéticos, oculto tras nubes de polvo. // *The heart of the Milky Way is full of stars, hot gas and strong magnetic fields, hidden behind clouds of dust.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `ExperienceAnchor` `vid=5` (`hero_doc_cosmos.wgsl:411-504` Kerr BH `#000000` shadow + photon ring + Doppler-beamed accretion disk). HOWEVER `LM[10] id=center` (`z=10`, `10%3=1`) wrongly renders **`vid=21` (Orion Creation Pillars)** at Hero LOD!
- **Audit Gap:** Kerr BH (`vid=5`) matches Sgr A* + hot accretion disk, EXCEPT `LM[10]` misfires as Orion Creation Pillars (`10%3=1`), `gal_r > 0.28` carves out stars right around Sgr A* (zero dense nuclear star cluster!), zero fast S-star Keplerian orbits (`S2` precession), and zero IR/radio dust-penetrating mode. `doc_feat` ignored.
- **Zone 3D Procedural Blueprint:** Fix `LM[10]` -> dedicated Sgr A* sub-parsec stage (`vid=1510`) and integrate with `vid=5`: isolate black hole + EHT bright photon ring (`f=0`), populate `r < 0.28` nuclear star cluster cusp (`f=1`), render bright hairline elliptical high-eccentricity S2-like star orbits whipping around `rs` (`f=2`), hot shock-heated gas/dust arms (`f=3`), and wavelength slider peeling opaque optical foreground dust to reveal 1.3mm radio/IR view (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **Sagitario A*** / *Sagittarius A** | `[TIER-B]` | Un agujero negro supermasivo de unos cuatro millones de masas solares ocupa el centro de la galaxia. // *A supermassive black hole of about four million solar masses occupies the center of the galaxy.* | `kind=4` Kerr BH `#000000` core + photon ring depicts Sagittarius A*. | Isolate Sagittarius A* supermassive BH shadow + EHT amber photon ring (`doc_feat==0`). |
| `F1` | **cúmulo nuclear** / *nuclear star cluster* | `[TIER-D]` | Millones de estrellas se concentran en una región pequeña alrededor del agujero negro central. // *Millions of stars are concentrated in a small region around the central black hole.* | No dense nuclear star cluster cusp (`gal_r > 0.28` excludes stars near core). | Densify `r < 0.28` nuclear star cluster cusp surrounding Sgr A* (`doc_feat==1`). |
| `F2` | **órbita estelar** / *stellar orbit* | `[TIER-D]` | Algunas estrellas cercanas se mueven tan rápido que permiten medir la masa invisible del agujero negro. // *Some nearby stars move so fast that they allow the invisible mass of the black hole to be measured.* | No fast elliptical S-star Keplerian orbits whipping around Sgr A*. | Trace bright hairline high-eccentricity relativistic S2-like star ellipses (`doc_feat==2`). |
| `F3` | **polvo caliente** / *hot dust* | `[TIER-B]` | La radiación y los choques calientan nubes que brillan en longitudes de onda invisibles para nuestros ojos. // *Radiation and shocks heat clouds that glow at wavelengths invisible to our eyes.* | Hot Doppler-beamed accretion filaments depict superheated inner dust/plasma. | Swirl hot shock-heated spiral gas & circumnuclear dust streamers (`doc_feat==3`). |
| `F4` | **radioastronomía** / *radio astronomy* | `[TIER-D]` | Las ondas de radio y la luz infrarroja atraviesan mejor el polvo que oculta el centro galáctico. // *Radio waves and infrared light are better able to pass through the dust hiding the galactic center.* | No optical-dust-to-radio/IR wavelength penetrating transition (`LM[10]` draws Pillars!). | Sweep optical-to-1.3mm radio dust-peeling transparency slider (`doc_feat==4`). |

#### `Exp 05 Z11` `halo` — Halo galáctico / *galactic halo* (`[TIER-C]` · Target `vid=1511`)
- **Metric (`raw_metric`):** "Una envoltura extensa alrededor del disco" (*An extended halo around the disk*) · `range=[0.940, 0.982]`
- **Spoken Macro (ES // EN):** Fuera del disco brillante existe un halo con estrellas antiguas, cúmulos globulares, corrientes estelares y una gran cantidad de materia oscura. // *Outside the bright disk there is a halo with ancient stars, globular clusters, stellar streams and a large amount of dark matter.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** MODULO MISMATCH: `LM[11] id=halo` (`z=11`, `11%3=2`) -> **`vid=22` (Local Bubble Supernova Cavity)** (`doc_stages.rs:362`) + flat `vid=5` MW disk (`abs(pos.y) < 0.16` clips 3D halo!) + flat Mode 0 particles.
- **Audit Gap:** Milky Way halo renders a local supernova bubble (`vid=22`) over a razor-thin flat disk. Missing ancient spherical stellar halo, dense ~150 globular clusters, torn tidal stellar streams, hot X-ray galactic corona, satellite galaxies, and invisible dark-matter halo shell.
- **Zone 3D Procedural Blueprint:** Route `halo` (`z=11`) to dedicated 3D galactic halo diorama (`vid=1511`): dense compact golden spheres for ~150 globular clusters (`f=0,1`), graceful great-circle stellar stream ribbons wrapping outer halo (`f=2`), vast translucent spherical dark-matter halo envelope (`r=0.92`, `f=3`), hot diffuse X-ray corona glow (`f=3`), and dwarf satellite smudges (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **cúmulo globular** / *globular cluster* | `[TIER-C]` | Cientos de miles de estrellas antiguas pueden reunirse en una esfera compacta que orbita la galaxia. // *Hundreds of thousands of ancient stars can be packed into a compact sphere that orbits the galaxy.* | Misrouted by `11 % 3 == 2` to `vid=22` (Local Bubble) on top of flat `vid=5` disk. | Cluster compact bright golden spheres for ~150 halo globular clusters (`doc_feat==0`). |
| `F1` | **estrella antigua** / *ancient star* | `[TIER-C]` | Algunas estrellas del halo nacieron cuando la Vía Láctea todavía estaba formándose. // *Some halo stars were born when the Milky Way was still forming.* | Misrouted by `11 % 3 == 2` to `vid=22` (Local Bubble) on top of flat `vid=5` disk. | Tint ancient metal-poor red/gold Population II halo stars (`doc_feat==1`). |
| `F2` | **corriente estelar** / *stellar stream* | `[TIER-C]` | Una galaxia pequeña o un cúmulo desgarrado puede dejar una larga cinta de estrellas. // *A small galaxy or torn cluster can leave behind a long ribbon of stars.* | Misrouted by `11 % 3 == 2` to `vid=22` (Local Bubble) on top of flat `vid=5` disk. | Wrap great-circle tidal stellar-stream ribbons arcing across outer halo (`doc_feat==2`). |
| `F3` | **halo de materia oscura** / *dark matter halo* | `[TIER-C]` | La mayor parte de la masa de la galaxia podría estar distribuida en una envoltura invisible mucho mayor que el disco. // *Most of the galaxy's mass could be distributed in an invisible envelope much larger than the disk.* | Misrouted by `11 % 3 == 2` to `vid=22` (Local Bubble) on top of flat `vid=5` disk. | Envelop galaxy in translucent spherical NFW dark-matter + X-ray corona shell (`r=0.92`, `doc_feat==3`). |
| `F4` | **galaxia satélite** / *satellite galaxy* | `[TIER-C]` | Pequeñas galaxias orbitan la Vía Láctea y pueden intercambiar estrellas y gas con ella. // *Small galaxies orbit the Milky Way and can exchange stars and gas with it.* | Misrouted by `11 % 3 == 2` to `vid=22` (Local Bubble) on top of flat `vid=5` disk. | Float faint dwarf satellite smudges orbiting outer Milky Way halo (`doc_feat==4`). |

#### `Exp 05 Z12` `galactic-edge` — Borde de la Vía Láctea / *Edge of the Milky Way* (`[TIER-C]` · Target `vid=1512`)
- **Metric (`raw_metric`):** "Salida hacia el Grupo Local" (*Toward the Local Group*) · `range=[0.982, 1.000]`
- **Spoken Macro (ES // EN):** La galaxia no termina en una línea nítida. Su luz se vuelve tenue y su halo se mezcla con el espacio entre galaxias. // *The galaxy does not end in a sharp line. Its light becomes dim and its halo blends with the space between galaxies.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** GENERIC FALLBACK: Exp 05 has only **12 landmarks for 13 zones** (`z=12 galactic-edge` has NO landmark -> generic pinpoint `ZoneShell` sprite only, `assets.rs:273-293`), plus flat `vid=5` Kerr BH + MW disk (`abs(pos.y) < 0.16`) + Mode 0.
- **Audit Gap:** Missing 13th landmark! Flat disk stops sharply at `r=0.94` with no diffuse halo fade into intergalactic space and no distant Andromeda (`M31`) / Local Group preview on the horizon.
- **Zone 3D Procedural Blueprint:** Add 13th zone stage (`vid=1512`) or `z_idx == 12u` blend on `vid=5`: zoom disk back inside diffuse faint halo gradient fading into black void (`f=0,1`), place faint distant tilted M31 & dwarf silhouettes across deep void (`f=2,3`), and transition smoothly into `u ~ 21.1` Local Group scale (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **borde difuso** / *diffuse boundary* | `[TIER-C]` | Cada componente de la galaxia tiene un tamaño distinto, por eso no existe una única frontera exacta. // *Each component of the galaxy has a different size, so there is no single exact boundary.* | No landmark entity exists (`12` landmarks / `13` zones); only generic `ZoneShell` dot + `vid=5` Kerr/disk. | Fade stellar disk exponentially into ultra-faint exponential outer halo edge (`doc_feat==0`). |
| `F1` | **espacio intergaláctico** / *intergalactic space* | `[TIER-C]` | Entre las galaxias hay enormes distancias con gas extremadamente tenue y materia invisible. // *Between galaxies there are enormous distances with extremely tenuous gas and invisible matter.* | No landmark entity exists (`12` landmarks / `13` zones); only generic `ZoneShell` dot + `vid=5` Kerr/disk. | Clear foreground Milky Way density into cold pure-black intergalactic void (`doc_feat==1`). |
| `F2` | **Grupo Local** / *Local Group* | `[TIER-C]` | La Vía Láctea pertenece a una familia de galaxias donde también están Andrómeda y muchas galaxias pequeñas. // *The Milky Way belongs to a family of galaxies that also includes Andromeda and many small galaxies.* | No landmark entity exists (`12` landmarks / `13` zones); only generic `ZoneShell` dot + `vid=5` Kerr/disk. | Reveal distant Local Group dwarf & sister silhouettes across deep darkness (`doc_feat==2`). |
| `F3` | **Andrómeda** / *Andromeda* | `[TIER-C]` | Es la gran galaxia vecina y el siguiente destino natural para ampliar esta aventura. // *It is our large neighboring galaxy and the natural next destination for extending this adventure.* | No landmark entity exists (`12` landmarks / `13` zones); only generic `ZoneShell` dot + `vid=5` Kerr/disk. | Pinpoint faint tilted Andromeda (M31) disk glowing 2.5 Mly beyond (`doc_feat==3`). |
| `F4` | **universo** / *universe* | `[TIER-C]` | Más allá esperan grupos, cúmulos, filamentos cósmicos y miles de millones de galaxias. // *Beyond, groups, clusters, cosmic filaments and billions of galaxies await.* | No landmark entity exists (`12` landmarks / `13` zones); only generic `ZoneShell` dot + `vid=5` Kerr/disk. | Bridge scale transition seamlessly from Galactic Edge into Exp 06 Local Group (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP05-Z09-12)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 05 (05_milky_way) Zones Z09..Z12 (`milky-way-disk`, `galactic-center`, `halo`, `galactic-edge`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z09->1509`, `Z10->1510`, `Z11->1511`, `Z12->1512` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Replace legacy modulo/static landmark dispatch with explicit companion IDs: LM09(disk)->3669, LM10(center)->3670, LM11(halo)->3671.
   - Handle Z12 galactic-edge without leaving an orphaned ZoneShell pinpoint dot (emit rim hero vid=1512 + submode=32).
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `frontera`, `heliosfera`, `oort`, `alfa`, `centauri`, `burbuja`, `orion`, `nebulosa`, `espiral`, `halo`, `disco`, `nucleo` (`sagitario` already exists!); verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z09->1509`, `Z10->1510`, `Z11->1511`, `Z12->1512`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z09 (vid=1509, milky-way-disk): F0:Milky Way; F1:central bar; F2:dust lane; F3:dark matter; F4:star
     * Z10 (vid=1510, galactic-center): F0:Sagittarius A*; F1:nuclear star cluster; F2:stellar orbit; F3:hot dust; F4:radio astronomy
     * Z11 (vid=1511, halo): F0:globular cluster; F1:ancient star; F2:stellar stream; F3:dark matter halo; F4:satellite galaxy
     * Z12 (vid=1512, galactic-edge): F0:diffuse boundary; F1:intergalactic space; F2:Local Group; F3:Andromeda; F4:universe
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

---

### Exp 21: `21_cosmic_life` — Vida Cósmica / Cosmic Life (`25` Zones, `200` Features, `0` Landmarks)

> *Espectro de la Vida Cósmica: del Planeta Habitable a la Inteligencia Extraterrestre* — Spectrum of Cosmic Life: from Habitable Planets to Extraterrestrial Intelligence

- **Odyssey Position (`odyssey.rs#L29-L32`):** Act 2, Position **2 / 8** (Chapter `act2_exp21`)
- **Spatial & Scale Band:** `SectorId::ORION_SPUR` (`anchor_local=(-0.35, 0.0, 0.25)`) · `u ∈ [15.5, 20.0]` · `focus_scale_exp = 18.0` (`topology.rs:340`)
- **Compute & Far Shell:** `mode = 13` (cosmological radial redshift stream — mismatch for exoplanet biospheres!, `compute_universe.wgsl:319-330`, `universe_scene.rs:313`) · `galaxy_plane = 1.0`, `star_density = 1.0` Orion Spur background (`universe_scene.rs:350-370`)
- **Current Render Dispatch:** `doc_stage_variant_id(21)` (`doc_stages.rs:75-78`) routes `z=12 -> vid=212` (Europa plume shell) and ALL other 24 zones -> `vid=210` (`mode=0`: twin-ring Habitable Zone orbiter, ignoring `scene=0..24` completely!), plus twin `heroes_surface` spheres `vid=11 / vid=10` (`doc_stages.rs:764-786`); `hero_doc_civilization_horizon.wgsl:363-388` shifts `orb_ang` by `f32(feat)*0.45` only.
- **Granular Audit Tally:** Zones `0A / 4B / 4C / 17D` (`25`) · Features `0A / 10B / 32C / 158D` (`200`)
- **`DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`) Needed Additions:** `astrobiologia`, `habitabilidad`, `agua`, `exoplaneta`, `biofirma`, `extremofilo`, `encelado` & `europa` (exist), `tránsito`, `espectro`, `biosfera`, `tecnofirma`, `fermi`

<a id="pkg-act2-exp21-z00-04"></a>
### PKG-ACT2-EXP21-Z00-04: Cosmic Life Origins & Planetary Habitability Lab (`z=00..04`) — `Phase 1` · Priority: **Critical**

**Current State:** Zones `Z00..Z04` (`5` zones, `40` features) | Zone Tiers `0A / 1B / 1C / 3D` | Feature Tiers `0A / 3B / 5C / 32D` | Render path: [`doc_stages.rs#L75-L78`](../../crates/cosmos_render/src/doc_stages.rs#L75-L78) -> [`shaders/hero_doc_civilization_horizon.wgsl#L363-L388`](../../shaders/hero_doc_civilization_horizon.wgsl#L363-L388) — `z=12 -> vid=212`, all other `24` zones -> `vid=210` + twin `11/10` globes (`sc` ignored). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `40` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z00` | `unknown-horizon` | Horizonte desconocido / *unknown horizon* | **`3100`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z01` | `cosmic-ingredients` | Ingredientes cósmicos / *Cosmic ingredients* | **`3101`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z02` | `stellar-stability` | Estrellas y estabilidad / *Stars and stability* | **`3102`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z03` | `planetary-systems` | Sistemas planetarios / *Planetary systems* | **`3103`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z04` | `habitable-zone` | Zona habitable / *Habitable zone* | **`3104`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 21 Z00` `unknown-horizon` — Horizonte desconocido / *unknown horizon* (`[TIER-C]` · Target `vid=3100`)
- **Metric (`raw_metric`):** "Imaginación informada + observación + incertidumbre → preguntas investigables" (*Informed imagination + observation + uncertainty → researchable questions*) · `scene=0` · `footer="DE NUESTRO FUTURO A OTRAS POSIBILIDADES"` · `evidence_class`: "Punto de partida · preguntas abiertas, no afirmaciones" (*Starting point · open questions, not statements*) · `range=[0.000, 0.040]`
- **Spoken Macro (ES // EN):** La astrobiología no supone que exista vida extraterrestre: estudia qué condiciones podrían producirla y qué evidencias permitirían reconocerla. // *Astrobiology does not assume that extraterrestrial life exists: it studies what conditions could produce it and what evidence would allow it to be recognized.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** doc_stage_variant_id ignores sc=0 and routes to fallback vid=210 (Goldilocks ring + 1 dot + twin kind=1 surface spheres vid=11 lush exoplanet & vid=10 ice moon). Z00 is the epistemic transition from human futures (Exp 20) to astrobiological inquiry ('distant stars, possible planets, probes, unknown minds, weak signals in noise').
- **Zone 3D Procedural Blueprint:** Astrobiology Horizon Aperture (vid=3100): dim deep-starfield nebula pierced by thin multi-wavelength telescopic scan cones

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **estrellas lejanas** / *distant stars* | `[TIER-D]` | Más allá de nuestros sistemas conocidos existen mundos que apenas podemos inferir a partir de su luz. // *Beyond our known systems there are worlds that we can barely infer from their light.* | `vid=210` ignores `scene=0` & `doc_feat=0` | faint distant G/M star pinpoints |
| `F1` | **planetas posibles** / *possible planets* | `[TIER-C]` | La presencia de un planeta no implica océanos, atmósfera estable ni condiciones adecuadas para la vida. // *The presence of a planet does not imply oceans, a stable atmosphere or conditions suitable for life.* | `vid=210` ignores `scene=0` & `doc_feat=1` | ghostly translucent unconfirmed planet silhouette |
| `F2` | **sondas autónomas** / *autonomous probes* | `[TIER-D]` | Una señal tecnológica podría proceder de una máquina, de una civilización activa o de un sistema abandonado. // *A technological signal could come from a machine, an active civilization, or an abandoned system.* | `vid=210` ignores `scene=0` & `doc_feat=2` | lone derelict autonomous probe coasting through dust |
| `F3` | **inteligencias desconocidas** / *unknown intelligences* | `[TIER-C]` | Otras mentes, si existen, podrían no compartir cuerpos, ritmos, sentidos ni objetivos humanos. // *Other minds, if they exist, may not share human bodies, rhythms, senses, or goals.* | `vid=210` ignores `scene=0` & `doc_feat=3` | morphing non-human neural wave outline |
| `F4` | **regiones inexploradas** / *unexplored regions* | `[TIER-C]` | La mayor parte del cosmos nunca ha sido examinada con resolución suficiente para buscar vida. // *Most of the cosmos has never been examined with sufficient resolution to search for life.* | `vid=210` ignores `scene=0` & `doc_feat=4` | wide dark unprobed cosmic sector map |
| `F5` | **señales en el ruido** / *signals in the noise* | `[TIER-D]` | Una señal débil puede ser un fenómeno natural, interferencia, error instrumental o una pista auténtica. // *A weak signal may be a natural phenomenon, interference, instrumental error, or a genuine clue.* | `vid=210` ignores `scene=0` & `doc_feat=5` | noisy spike trace oscillating near thermal floor |
| `F6` | **futuros ajenos** / *foreign futures* | `[TIER-C]` | La evolución y la historia podrían estar desarrollándose de formas distintas en innumerables mundos. // *Evolution and history could be unfolding in different ways on countless worlds.* | `vid=210` ignores `scene=0` & `doc_feat=6` | bifurcating foreign world ribbons |
| `F7` | **cambio de perspectiva** / *change of perspective* | `[TIER-C]` | En lugar de preguntar solo hasta dónde llegará la humanidad, preguntamos qué podría estar viviendo o construyendo en otros lugares. // *Instead of just asking how far humanity will go, we ask what it might be living or building elsewhere.* | `vid=210` ignores `scene=0` & `doc_feat=7` | panoramic camera pivot from Solar hub out to Orion Spur |

#### `Exp 21 Z01` `cosmic-ingredients` — Ingredientes cósmicos / *Cosmic ingredients* (`[TIER-D]` · Target `vid=3101`)
- **Metric (`raw_metric`):** "Nucleosíntesis + nubes moleculares + química → materiales potencialmente biológicos" (*Nucleosynthesis + molecular clouds + chemistry → potentially biological materials*) · `scene=1` · `footer="INGREDIENTES COMUNES · ORGANIZACIÓN INCIERTA"` · `evidence_class`: "Materia común · organización no garantizada" (*Common matter · organization not guaranteed*) · `range=[0.040, 0.080]`
- **Spoken Macro (ES // EN):** La vida terrestre utiliza elementos comunes en el universo, pero disponer de los ingredientes no garantiza que lleguen a organizarse como vida. // *Terrestrial life uses common elements in the universe, but having the ingredients does not guarantee that they will be organized as life.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Renders fully formed Goldilocks planets (`210` + `vid=11`/`10`) instead of cosmic elemental origins: Big Bang primordial hydrogen, stellar core carbon/oxygen/nitrogen/phosphorus/sulfur synthesis, AGB/supernova stardust shells, and cold molecular PAH clouds.
- **Zone 3D Procedural Blueprint:** CHNOPS Stellar Alchemy & Molecular Cloud (vid=3101): dusty dark Bok-globule cloud wrapping glowing multi-shell red giant/supernova ejecta

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **hidrógeno primordial** / *primordial hydrogen* | `[TIER-D]` | El hidrógeno se formó en el universo temprano y sigue siendo el elemento más abundante del cosmos. // *Hydrogen formed in the early universe and remains the most abundant element in the cosmos.* | `vid=210` ignores `scene=1` & `doc_feat=0` | primordial hydrogen gas envelope + 21cm glow |
| `F1` | **carbono** / *carbon* | `[TIER-D]` | Las estrellas fabrican carbono, capaz de formar cadenas y estructuras químicas muy diversas. // *Stars make carbon, capable of forming very diverse chains and chemical structures.* | `vid=210` ignores `scene=1` & `doc_feat=1` | triple-alpha carbon shell lattice forming graphene/chain backbones |
| `F2` | **oxígeno** / *oxygen* | `[TIER-D]` | El oxígeno participa en agua, rocas y metabolismos, aunque también puede producirse sin vida. // *Oxygen participates in water, rocks and metabolisms, although it can also be produced without life.* | `vid=210` ignores `scene=1` & `doc_feat=2` | bright oxygen nuclear shell -> icy H2O frost grain accretion |
| `F3` | **nitrógeno** / *nitrogen* | `[TIER-D]` | El nitrógeno forma parte de proteínas y ácidos nucleicos, pero debe encontrarse en formas químicamente accesibles. // *Nitrogen is part of proteins and nucleic acids, but must be found in chemically accessible forms.* | `vid=210` ignores `scene=1` & `doc_feat=3` | cyan nitrogen cycle wisps |
| `F4` | **fósforo** / *phosphorus* | `[TIER-D]` | El fósforo participa en membranas y transferencia de energía, y su disponibilidad puede limitar ecosistemas. // *Phosphorus is involved in membranes and energy transfer, and its availability can limit ecosystems.* | `vid=210` ignores `scene=1` & `doc_feat=4` | concentrated phosphorus flare knots from massive supernovae |
| `F5` | **azufre** / *sulfur* | `[TIER-D]` | El azufre interviene en proteínas y metabolismos capaces de operar sin oxígeno. // *Sulfur intervenes in proteins and metabolisms capable of operating without oxygen.* | `vid=210` ignores `scene=1` & `doc_feat=5` | golden-amber sulfur vents & dust |
| `F6` | **polvo estelar** / *star dust* | `[TIER-D]` | Elementos pesados expulsados por estrellas se incorporan a nuevas nubes, planetas y organismos potenciales. // *Heavy elements ejected by stars are incorporated into new clouds, planets and potential organisms.* | `vid=210` ignores `scene=1` & `doc_feat=6` | expanding filamentary supernova shockwave shell seeding silica dust |
| `F7` | **moléculas interestelares** / *interstellar molecules* | `[TIER-D]` | En nubes frías se detectan moléculas orgánicas, pero orgánico significa basado en carbono, no vivo. // *Organic molecules are detected in cold clouds, but organic means carbon-based, not living.* | `vid=210` ignores `scene=1` & `doc_feat=7` | cold dark molecular cloud highlighting complex carbon-ring molecules (labeled 'ORGANIC != ALIVE') |

#### `Exp 21 Z02` `stellar-stability` — Estrellas y estabilidad / *Stars and stability* (`[TIER-D]` · Target `vid=3102`)
- **Metric (`raw_metric`):** "Vida estelar + radiación + actividad + órbitas → entorno energético de larga duración" (*Stellar life + radiation + activity + orbits → long-lived energy environment*) · `scene=2` · `footer="ENERGÍA DURADERA · ACTIVIDAD CAMBIANTE"` · `evidence_class`: "Fuente de energía · entorno dinámico" (*Energy source dynamic environment*) · `range=[0.080, 0.120]`
- **Spoken Macro (ES // EN):** Una estrella puede suministrar energía durante miles de millones de años y, al mismo tiempo, erosionar atmósferas o irradiar intensamente sus planetas. // *A star can supply energy for billions of years and at the same time erode atmospheres or intensely irradiate its planets.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Static yellow central dot in `210` ignores every stellar astrophysics process narrated: spectral lifespan divergence (O/B vs G vs M-dwarf), bolometric luminosity distance, UV prebiotic-vs-destructive flux, violent M-dwarf flares/CMEs, magnetic stellar wind atmospheric stripping, binary star orbits, secular stability, and multi-generation metallicity.
- **Zone 3D Procedural Blueprint:** Host-Star Astrophysics & Space-Weather Arena (vid=3102): central star switching temperature/luminosity/activity dynamically per doc_feat: 0 side-by-side blue O-giant burning fast vs long-lived orange-red M-dwarf glow, 1 bolometric irradiance flux sphere scaling inverse-square, 2 violet UV band splitting/recombining organic bonds, 3 explosive red-orange CME magnetic coronal loop slamming close-in planet, 4 plasma wind ribbons stripping tenuous unshielded blue atmosphere tail, 5 binary companion star perturbing figure-8 planetary orbit, 6 calm circular multi-Gyr harmonic ring, 7 supernovae enrichment glow boosting dusty rocky core fraction.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **vida estelar** / *stellar life* | `[TIER-D]` | Las estrellas pequeñas pueden durar muchísimo tiempo; las muy masivas evolucionan con rapidez. // *Small stars can last a very long time; the very massive ones evolve quickly.* | `vid=210` ignores `scene=2` & `doc_feat=0` | side-by-side blue O-giant burning fast vs long-lived orange-red M-dwarf glow |
| `F1` | **luminosidad** / *luminosity* | `[TIER-D]` | La energía recibida por un planeta depende de la potencia estelar y de la distancia orbital. // *The energy received by a planet depends on the stellar power and the orbital distance.* | `vid=210` ignores `scene=2` & `doc_feat=1` | bolometric irradiance flux sphere scaling inverse-square |
| `F2` | **radiación ultravioleta** / *ultraviolet radiation* | `[TIER-D]` | La radiación ultravioleta puede impulsar química prebiótica y también dañar moléculas y atmósferas. // *Ultraviolet radiation can boost prebiotic chemistry and also damage molecules and atmospheres.* | `vid=210` ignores `scene=2` & `doc_feat=2` | violet UV band splitting/recombining organic bonds |
| `F3` | **fulguraciones** / *stellar flares* | `[TIER-D]` | Erupciones frecuentes pueden bombardear planetas cercanos con radiación y partículas. // *Frequent flares can bombard nearby planets with radiation and particles.* | `vid=210` ignores `scene=2` & `doc_feat=3` | explosive red-orange CME magnetic coronal loop slamming close-in planet |
| `F4` | **actividad magnética** / *magnetic activity* | `[TIER-D]` | El viento y el campo magnético estelares modifican la pérdida atmosférica y el clima espacial. // *The stellar wind and magnetic field modify atmospheric loss and space weather.* | `vid=210` ignores `scene=2` & `doc_feat=4` | plasma wind ribbons stripping tenuous unshielded blue atmosphere tail |
| `F5` | **sistemas binarios** / *binary systems* | `[TIER-D]` | Los planetas pueden existir alrededor de una o dos estrellas, aunque la estabilidad orbital varía. // *Planets can exist around one or two stars, although orbital stability varies.* | `vid=210` ignores `scene=2` & `doc_feat=5` | binary companion star perturbing figure-8 planetary orbit |
| `F6` | **órbitas estables** / *stable orbits* | `[TIER-D]` | La habitabilidad prolongada requiere evitar perturbaciones que alteren drásticamente temperatura o trayectoria. // *Prolonged habitability requires avoiding disturbances that drastically alter temperature or trajectory.* | `vid=210` ignores `scene=2` & `doc_feat=6` | calm circular multi-Gyr harmonic ring |
| `F7` | **enriquecimiento químico** / *chemical enrichment* | `[TIER-D]` | Las generaciones estelares anteriores aportan los elementos necesarios para formar mundos rocosos y química compleja. // *Previous stellar generations provide the elements necessary to form rocky worlds and complex chemistry.* | `vid=210` ignores `scene=2` & `doc_feat=7` | supernovae enrichment glow boosting dusty rocky core fraction |

#### `Exp 21 Z03` `planetary-systems` — Sistemas planetarios / *Planetary systems* (`[TIER-D]` · Target `vid=3103`)
- **Metric (`raw_metric`):** "Disco protoplanetario + acreción + migración + impactos → arquitectura planetaria" (*Protoplanetary disk + accretion + migration + impacts → planetary architecture*) · `scene=3` · `footer="LOS SISTEMAS CAMBIAN MIENTRAS SE FORMAN"` · `evidence_class`: "Mundos en evolución · posiciones cambiantes" (*Evolving worlds changing positions*) · `range=[0.120, 0.160]`
- **Spoken Macro (ES // EN):** La posición actual de un planeta no siempre es el lugar donde se formó: los sistemas planetarios cambian durante su evolución. // *A planet's current position is not always where it formed: planetary systems change during their evolution.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Shows static emerald HZ torus + twin finished planets (`210` + `vid=11`/`10`) while narration explicitly builds a solar system from chaos: swirling protoplanetary gas/dust disk, rocky inner worlds, gas giants, ice giants, moons, asteroid rubble, volatile-delivering comets, and planetary orbital migration.
- **Zone 3D Procedural Blueprint:** Dynamic Protoplanetary Disk & Migration Lab (vid=3103)

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **disco protoplanetario** / *protoplanetary disk* | `[TIER-D]` | Gas y polvo giran alrededor de una estrella joven y se agrupan gradualmente en cuerpos mayores. // *Gas and dust swirl around a young star and gradually clump together into larger bodies.* | `vid=210` ignores `scene=3` & `doc_feat=0` | flared golden-dust Keplerian disk with dark concentric accretion gaps cleared by protoplanets |
| `F1` | **planetas rocosos** / *rocky planets* | `[TIER-D]` | Mundos dominados por silicatos y metales pueden sostener superficies, atmósferas o actividad interna. // *Worlds dominated by silicates and metals can support surfaces, atmospheres or internal activity.* | `vid=210` ignores `scene=3` & `doc_feat=1` | glowing molten inner silicate rocky spheres inside frost line |
| `F2` | **gigantes gaseosos** / *gas giants* | `[TIER-D]` | Planetas masivos alteran órbitas, distribuyen cometas y pueden proteger o desestabilizar mundos interiores. // *Massive planets alter orbits, distribute comets, and can protect or destabilize inner worlds.* | `vid=210` ignores `scene=3` & `doc_feat=2` | banded Jovian gas giant sweeping wide gap and capturing moons |
| `F3` | **gigantes helados** / *ice giants* | `[TIER-D]` | Mundos ricos en hielos y gases amplían la diversidad de composiciones planetarias. // *Worlds rich in ice and gases expand the diversity of planetary compositions.* | `vid=210` ignores `scene=3` & `doc_feat=3` | pale azure/cyan Uranus-Neptune ice giant beyond frost line |
| `F4` | **lunas** / *moons* | `[TIER-D]` | Satélites grandes pueden tener atmósferas, océanos internos, volcanismo y fuentes propias de energía. // *Large satellites can have atmospheres, internal oceans, volcanism and their own energy sources.* | `vid=210` ignores `scene=3` & `doc_feat=4` | miniature Galilean/Titan sub-disk satellite hierarchy |
| `F5` | **asteroides** / *asteroids* | `[TIER-D]` | Restos rocosos conservan materiales primitivos y participan en impactos y transporte químico. // *Rock remains preserve primitive materials and participate in impacts and chemical transport.* | `vid=210` ignores `scene=3` & `doc_feat=5` | tumbling rocky asteroid belt resonance ring |
| `F6` | **cometas** / *comets* | `[TIER-D]` | Cuerpos helados trasladan agua, carbono y otros compuestos, aunque su contribución varía entre sistemas. // *Frozen bodies transport water, carbon and other compounds, although their contribution varies between systems.* | `vid=210` ignores `scene=3` & `doc_feat=6` | eccentric bright-tailed comet showering icy volatiles onto inner world |
| `F7` | **migración planetaria** / *planetary migration* | `[TIER-D]` | Interacciones con el disco y otros planetas pueden desplazar mundos a enormes distancias de su lugar de nacimiento. // *Interactions with the disk and other planets can move worlds enormous distances from their birthplace.* | `vid=210` ignores `scene=3` & `doc_feat=7` | inward Hot-Jupiter / Grand-Tack spiral migration trail |

#### `Exp 21 Z04` `habitable-zone` — Zona habitable / *Habitable zone* (`[TIER-B]` · Target `vid=3104`)
- **Metric (`raw_metric`):** "Distancia + luminosidad + atmósfera básica → intervalo térmico potencial" (*Distance + luminosity + basic atmosphere → potential thermal range*) · `scene=4` · `footer="ZONA HABITABLE ≠ PLANETA HABITADO"` · `evidence_class`: "Agua superficial posible · habitabilidad no garantizada" (*Surface water possible · habitability not guaranteed*) · `range=[0.160, 0.200]`
- **Spoken Macro (ES // EN):** Estar dentro de la zona habitable no significa estar habitado, ni siquiera ser realmente habitable. // *Being within the habitable zone does not mean being inhabited, or even being truly habitable.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** vid=210 (`Goldilocks Habitable Ring`) literally depicts F00/F01/F07 at the archetype level (central star + 2 emerald HZ torus rings + 1 orbiting planet + twin spheres), yet `doc_feat` ONLY offsets the single dot's starting angle (`* 0.45`). Inner runaway greenhouse limit, outer snowball freeze, eccentric orbits, tidal locking, and evolving stellar luminosity migration are all missing.
- **Zone 3D Procedural Blueprint:** Upgrade vid=210 Interactive Circumstellar HZ Simulator: keep emerald belt and drive geometry from doc_feat: 0 AU radial ruler ticks, 1 1/r^2 incident flux ring gradient, 2 hot orange-red inner edge boiling steaming planet, 3 cold icy-blue outer edge snowball planet, 4 high-eccentricity e=0.6 ellipse ducking in/out of green zone with flash-freeze/thaw phases, 5 synchronously locked eyeball-world (parched red substellar eye + frozen blue night side), 6 slowly brightening aging star pushing emerald zone radially outward, 7 translucent fuzzy model-dependent boundary veil.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **distancia estelar** / *stellar distance* | `[TIER-B]` | Demasiada energía puede evaporar océanos; demasiado poca puede congelar la superficie. // *Too much energy can evaporate oceans; too little can freeze the surface.* | `vid=210` ignores `scene=4` & `doc_feat=0` | AU radial ruler ticks |
| `F1` | **radiación recibida** / *radiation received* | `[TIER-B]` | La energía total depende de la estrella, la órbita, el albedo y la atmósfera. // *The total energy depends on the star, orbit, albedo and atmosphere.* | `vid=210` ignores `scene=4` & `doc_feat=1` | 1/r^2 incident flux ring gradient |
| `F2` | **límite interior** / *inner limit* | `[TIER-D]` | Cerca de la estrella puede desencadenarse un efecto invernadero intenso y pérdida de agua. // *An intense greenhouse effect and water loss can be triggered near the star.* | `vid=210` ignores `scene=4` & `doc_feat=2` | hot orange-red inner edge boiling steaming planet |
| `F3` | **límite exterior** / *outer limit* | `[TIER-D]` | Lejos de la estrella, incluso una atmósfera con gases de efecto invernadero puede no evitar la congelación. // *Far from the star, even an atmosphere with greenhouse gases may not prevent freezing.* | `vid=210` ignores `scene=4` & `doc_feat=3` | cold icy-blue outer edge snowball planet |
| `F4` | **excentricidad** / *eccentricity* | `[TIER-D]` | Una órbita alargada produce variaciones estacionales extremas y modifica el promedio energético. // *An elongated orbit produces extreme seasonal variations and modifies the energy average.* | `vid=210` ignores `scene=4` & `doc_feat=4` | high-eccentricity e=0.6 ellipse ducking in/out of green zone with flash-freeze/thaw phases |
| `F5` | **acoplamiento de marea** / *tidal coupling* | `[TIER-D]` | Un planeta puede mostrar siempre la misma cara a su estrella sin quedar automáticamente inhabitable. // *A planet can always show the same face to its star without automatically becoming uninhabitable.* | `vid=210` ignores `scene=4` & `doc_feat=5` | synchronously locked eyeball-world (parched red substellar eye + frozen blue night side) |
| `F6` | **estrella cambiante** / *changing star* | `[TIER-D]` | La luminosidad estelar evoluciona, por lo que la zona habitable se desplaza con el tiempo. // *Stellar luminosity evolves, so the habitable zone shifts over time.* | `vid=210` ignores `scene=4` & `doc_feat=6` | slowly brightening aging star pushing emerald zone radially outward |
| `F7` | **franja móvil** / *moving strip* | `[TIER-B]` | La zona habitable es un rango dependiente de modelos, no un anillo verde rígido que certifique vida. // *The habitable zone is a model-dependent range, not a rigid green ring that certifies life.* | `vid=210` ignores `scene=4` & `doc_feat=7` | translucent fuzzy model-dependent boundary veil |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP21-Z00-04)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 21 (21_cosmic_life) Zones Z00..Z04 (`unknown-horizon`, `cosmic-ingredients`, `stellar-stability`, `planetary-systems`, `habitable-zone`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z00->3100`, `Z01->3101`, `Z02->3102`, `Z03->3103`, `Z04->3104` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `astrobiologia`, `habitabilidad`, `agua`, `exoplaneta`, `biofirma`, `extremofilo`, `encelado` & `europa` (exist), `tránsito`, `espectro`, `biosfera`, `tecnofirma`, `fermi`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_civilization_horizon.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z00->3100`, `Z01->3101`, `Z02->3102`, `Z03->3103`, `Z04->3104`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z00 (vid=3100, unknown-horizon): F0:distant stars; F1:possible planets; F2:autonomous probes; F3:unknown intelligences; F4:unexplored regions; F5:signals in the noise; F6:foreign futures; F7:change of perspective
     * Z01 (vid=3101, cosmic-ingredients): F0:primordial hydrogen; F1:carbon; F2:oxygen; F3:nitrogen; F4:phosphorus; F5:sulfur; F6:star dust; F7:interstellar molecules
     * Z02 (vid=3102, stellar-stability): F0:stellar life; F1:luminosity; F2:ultraviolet radiation; F3:stellar flares; F4:magnetic activity; F5:binary systems; F6:stable orbits; F7:chemical enrichment
     * Z03 (vid=3103, planetary-systems): F0:protoplanetary disk; F1:rocky planets; F2:gas giants; F3:ice giants; F4:moons; F5:asteroids; F6:comets; F7:planetary migration
     * Z04 (vid=3104, habitable-zone): F0:stellar distance; F1:radiation received; F2:inner limit; F3:outer limit; F4:eccentricity; F5:tidal coupling; F6:changing star; F7:moving strip
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp21-z05-09"></a>
### PKG-ACT2-EXP21-Z05-09: Ocean Worlds, Biosphere Diversity & Evolutionary Thresholds (`z=05..09`) — `Phase 1` · Priority: **Critical**

**Current State:** Zones `Z05..Z09` (`5` zones, `40` features) | Zone Tiers `0A / 0B / 1C / 4D` | Feature Tiers `0A / 0B / 6C / 34D` | Render path: [`doc_stages.rs#L75-L78`](../../crates/cosmos_render/src/doc_stages.rs#L75-L78) -> [`shaders/hero_doc_civilization_horizon.wgsl#L363-L388`](../../shaders/hero_doc_civilization_horizon.wgsl#L363-L388) — `z=12 -> vid=212`, all other `24` zones -> `vid=210` + twin `11/10` globes (`sc` ignored). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `40` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z05` | `real-habitability` | Habitabilidad real / *Real habitability* | **`3105`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z06` | `prebiotic-chemistry` | Química prebiótica / *Prebiotic chemistry* | **`3106`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z07` | `origin-of-life` | Origen de la vida / *Origin of life* | **`3107`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z08` | `simple-biospheres` | Biosferas simples / *Simple biospheres* | **`3108`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z09` | `complex-life` | Vida compleja / *complex life* | **`3109`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 21 Z05` `real-habitability` — Habitabilidad real / *Real habitability* (`[TIER-C]` · Target `vid=3105`)
- **Metric (`raw_metric`):** "Atmósfera + presión + clima + geología + energía → ambientes capaces de sostener procesos vivos" (*Atmosphere + pressure + climate + geology + energy → environments capable of sustaining living processes*) · `scene=5` · `footer="ATMÓSFERA · CLIMA · GEOLOGÍA · ENERGÍA"` · `evidence_class`: "Planeta, ambiente y biosfera son niveles distintos" (*Planet, environment and biosphere are different levels*) · `range=[0.200, 0.240]`
- **Spoken Macro (ES // EN):** Zona habitable, planeta potencialmente habitable, ambiente habitable y planeta habitado no son conceptos equivalentes. // *Habitable zone, potentially habitable planet, habitable environment and inhabited planet are not equivalent concepts.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Shares fallback vid=210 + green/ice twin spheres. Z05 explicitly contrasts naive HZ geometry against real planetary habitability controls (atmosphere envelope, surface pressure phase diagram, balanced greenhouse shell, alternative liquid solvents, carbonate-silicate tectonic thermostat, magnetosphere shielding, climate stability, chemical/light energy), none of which appear.
- **Zone 3D Procedural Blueprint:** Planetary Habitability Cutaway & Magnetosphere (vid=3105): large planetary sphere with layered cutaway

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **atmósfera** / *atmosphere* | `[TIER-D]` | Una envoltura gaseosa regula temperatura, química, radiación y transporte de materiales. // *A gas envelope regulates temperature, chemistry, radiation and transport of materials.* | `vid=210` ignores `scene=5` & `doc_feat=0` | Rayleigh-scattering gaseous atmospheric shell |
| `F1` | **presión superficial** / *surface pressure* | `[TIER-D]` | El agua líquida y otros solventes dependen tanto de la temperatura como de la presión. // *Liquid water and other solvents depend on both temperature and pressure.* | `vid=210` ignores `scene=5` & `doc_feat=1` | triple-point pressure phase gauge locking liquid ocean |
| `F2` | **efecto invernadero** / *greenhouse effect* | `[TIER-D]` | Algunos gases retienen calor; demasiado poco o demasiado puede impedir condiciones estables. // *Some gases retain heat; Too little or too much can prevent stable conditions.* | `vid=210` ignores `scene=5` & `doc_feat=2` | infrared trapping loop (Venus runaway vs Mars freeze vs balanced Earth) |
| `F3` | **solventes líquidos** / *liquid solvents* | `[TIER-D]` | El agua es central para la vida terrestre, pero se estudian también otros medios químicos posibles. // *Water is central to terrestrial life, but other possible chemical media are also studied.* | `vid=210` ignores `scene=5` & `doc_feat=3` | comparative liquid water vs liquid methane solvent pools |
| `F4` | **ciclos geológicos** / *geological cycles* | `[TIER-D]` | Intercambios entre interior, superficie y atmósfera pueden estabilizar nutrientes y clima durante largos periodos. // *Exchanges between interior, surface and atmosphere can stabilize nutrients and climate for long periods.* | `vid=210` ignores `scene=5` & `doc_feat=4` | glowing magma mantle subduction + volcano outgassing CO2 silicate-weathering loop |
| `F5` | **protección magnética** / *magnetic protection* | `[TIER-D]` | Un campo magnético puede reducir parte del impacto del viento estelar, aunque no es la única protección posible. // *A magnetic field can reduce some of the impact of the stellar wind, although it is not the only possible protection.* | `vid=210` ignores `scene=5` & `doc_feat=5` | bright cyan dipole magnetosphere bowshock deflecting solar wind particles |
| `F6` | **estabilidad climática** / *climate stability* | `[TIER-C]` | La vida compleja puede requerir intervalos prolongados sin cambios planetarios esterilizantes. // *Complex life may require long intervals without sterilizing planetary changes.* | `vid=210` ignores `scene=5` & `doc_feat=6` | calm steady planetary axial tilt lock |
| `F7` | **energía disponible** / *available energy* | `[TIER-D]` | Luz, reacciones químicas, calor interno y gradientes físicos pueden alimentar metabolismo. // *Light, chemical reactions, internal heat and physical gradients can fuel metabolism.* | `vid=210` ignores `scene=5` & `doc_feat=7` | sunlight + hydrothermal redox arrow vectors |

#### `Exp 21 Z06` `prebiotic-chemistry` — Química prebiótica / *Prebiotic chemistry* (`[TIER-D]` · Target `vid=3106`)
- **Metric (`raw_metric`):** "Moléculas + superficies + energía + ciclos → química organizada antes de la biología" (*Molecules + surfaces + energy + cycles → organized chemistry before biology*) · `scene=6` · `footer="ORGÁNICO NO SIGNIFICA VIVO"` · `evidence_class`: "Moléculas complejas · todavía no vida" (*Complex molecules not yet life*) · `range=[0.240, 0.280]`
- **Spoken Macro (ES // EN):** Las moléculas orgánicas no están necesariamente vivas: son materiales químicos que pueden participar en procesos previos a la vida. // *Organic molecules are not necessarily alive: they are chemical materials that can participate in processes prior to life.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Renders macro stars and Earth-like globes (`210` + `vid=11`/`10`) while narration describes micro/molecular prebiotic chemistry: organic carbon compounds, amino acids, nucleotide assembly, self-assembling lipid bilayer vesicles, catalytic mineral crystal surfaces, deep-sea hydrothermal vents, energy gradients, and spontaneous molecular self-assembly.
- **Zone 3D Procedural Blueprint:** Micro-Scale Prebiotic Chemistry Chamber (vid=3106): zoom cleanly into sub-millimeter glowing fluid space

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **moléculas orgánicas** / *organic molecules* | `[TIER-D]` | Compuestos de carbono pueden formarse en atmósferas, océanos, rocas, meteoritos y nubes interestelares. // *Carbon compounds can form in atmospheres, oceans, rocks, meteorites, and interstellar clouds.* | `vid=210` ignores `scene=6` & `doc_feat=0` | hexagonal PAH & branched carbon-chain molecule swarm |
| `F1` | **aminoácidos** / *amino acids* | `[TIER-D]` | Son componentes de proteínas terrestres, pero su presencia aislada no demuestra organismos. // *They are components of terrestrial proteins, but their isolated presence does not demonstrate organisms.* | `vid=210` ignores `scene=6` & `doc_feat=1` | tetrahedral amino-acid polymerizing beads |
| `F2` | **nucleótidos** / *nucleotides* | `[TIER-D]` | Construir unidades capaces de almacenar información requiere rutas químicas, energía y condiciones adecuadas. // *Building units capable of storing information requires chemical pathways, energy and suitable conditions.* | `vid=210` ignores `scene=6` & `doc_feat=2` | complementary purine-pyrimidine rung pairing |
| `F3` | **membranas lipídicas** / *lipid membranes* | `[TIER-D]` | Algunas moléculas se autoensamblan en vesículas que separan un interior del ambiente. // *Some molecules self-assemble into vesicles that separate an interior from the environment.* | `vid=210` ignores `scene=6` & `doc_feat=3` | amphiphilic dual-tail molecules curving spontaneously into hollow spherical lipid vesicles |
| `F4` | **minerales catalíticos** / *catalytic minerals* | `[TIER-D]` | Superficies minerales pueden concentrar reactivos y acelerar reacciones químicas. // *Mineral surfaces can concentrate reagents and accelerate chemical reactions.* | `vid=210` ignores `scene=6` & `doc_feat=4` | crystalline pyrite/montmorillonite mineral lattice concentrating monomers on flat facets |
| `F5` | **fuentes hidrotermales** / *hydrothermal vents* | `[TIER-D]` | Gradientes de temperatura y química proporcionan energía continua y microambientes protegidos. // *Gradients of temperature and chemistry provide continuous energy and protected microenvironments.* | `vid=210` ignores `scene=6` & `doc_feat=5` | underwater black-smoker chimney jetting hot reduced mineral plume |
| `F6` | **gradientes energéticos** / *energy gradients* | `[TIER-D]` | La diferencia entre dos estados puede impulsar reacciones sin que exista todavía metabolismo. // *The difference between two states can drive reactions without metabolism yet existing.* | `vid=210` ignores `scene=6` & `doc_feat=6` | proton/redox pH boundary gradient glowing across mineral membrane |
| `F7` | **autoensamblaje** / *self-assembly* | `[TIER-D]` | El orden molecular puede surgir espontáneamente, pero autoorganización no equivale a evolución biológica. // *Molecular order can arise spontaneously, but self-organization does not equate to biological evolution.* | `vid=210` ignores `scene=6` & `doc_feat=7` | spontaneous fractal crystallization vs non-heritable crystal order |

#### `Exp 21 Z07` `origin-of-life` — Origen de la vida / *Origin of life* (`[TIER-D]` · Target `vid=3107`)
- **Metric (`raw_metric`):** "Compartimentos + herencia + metabolismo + variación → sistemas capaces de evolucionar" (*Compartments + heredity + metabolism + variation → systems capable of evolving*) · `scene=7` · `footer="HERENCIA + VARIACIÓN + SELECCIÓN"` · `evidence_class`: "Problema abierto · hipótesis que interactúan" (*Open problem · interacting hypotheses*) · `range=[0.280, 0.320]`
- **Spoken Macro (ES // EN):** Sabemos que la vida apareció temprano en la historia terrestre, pero todavía no conocemos el camino completo desde la química no viva hasta los primeros sistemas evolutivos. // *We know that life appeared early in Earth's history, but we still don't know the full path from non-living chemistry to the first evolutionary systems.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Macro Goldilocks star+planets (`210`) provides zero visual counterpart for origin-of-life biology: template-copying replicating polymers with mutations, autocatalytic metabolic feedback loops, catalytic RNA-world hairpins, dividing protocell vesicles, semi-permeable membrane compartmentalization, heredity, and natural variation.
- **Zone 3D Procedural Blueprint:** Abiogenesis Transition Protocell Lab (vid=3107): buoyant translucent vesicle network in warm fluid

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **moléculas replicantes** / *replicating molecules* | `[TIER-D]` | Copiar información con errores heredables permitiría competencia y selección, si el sistema puede sostenerse. // *Copying information with heritable errors would allow competition and selection, if the system can be sustained.* | `vid=210` ignores `scene=7` & `doc_feat=0` | unzipping template strand spawning daughter copy with glowing point-mutation color shift |
| `F1` | **metabolismo primero** / *metabolism first* | `[TIER-D]` | Redes químicas autocatalíticas podrían haber organizado energía antes de una genética moderna. // *Autocatalytic chemical networks could have organized energy before modern genetics.* | `vid=210` ignores `scene=7` & `doc_feat=1` | closed glowing cyclic autocatalytic reaction web without strands |
| `F2` | **mundo de ARN** / *RNA world* | `[TIER-D]` | Moléculas parecidas al ARN podrían combinar almacenamiento de información y actividad catalítica. // *RNA-like molecules could combine information storage and catalytic activity.* | `vid=210` ignores `scene=7` & `doc_feat=2` | folded 3D ribozyme RNA hairpin catalyzing cleavage |
| `F3` | **protocélulas** / *protocells* | `[TIER-D]` | Compartimentos simples pueden mantener componentes juntos y competir por recursos. // *Simple compartments can keep components together and compete for resources.* | `vid=210` ignores `scene=7` & `doc_feat=3` | swelling vesicle pinching via binary fission into two daughter protocells |
| `F4` | **compartimentación** / *compartmentalization* | `[TIER-D]` | Separar interior y exterior permite concentrar reacciones y controlar intercambios. // *Separating interior and exterior allows us to concentrate reactions and control exchanges.* | `vid=210` ignores `scene=7` & `doc_feat=4` | selective pore channels trapping red/gold internal metabolites |
| `F5` | **herencia** / *inheritance* | `[TIER-C]` | La continuidad biológica requiere que parte de la organización pase a nuevas generaciones. // *Biological continuity requires that part of the organization be passed on to new generations.* | `vid=210` ignores `scene=7` & `doc_feat=5` | multi-generation lineage chain passing core ring |
| `F6` | **variación** / *variation* | `[TIER-C]` | Sin diferencias heredables no existe evolución acumulativa por selección natural. // *Without heritable differences there is no cumulative evolution by natural selection.* | `vid=210` ignores `scene=7` & `doc_feat=6` | color/shape mutation diversity across competing vesicles |
| `F7` | **varias rutas** / *several routes* | `[TIER-C]` | El origen pudo depender de múltiples ambientes y etapas, no de una única reacción milagrosa. // *The origin may have depended on multiple environments and stages, not on a single miraculous reaction.* | `vid=210` ignores `scene=7` & `doc_feat=7` | 3-branch converging river/pond/vent pathway diagram |

#### `Exp 21 Z08` `simple-biospheres` — Biosferas simples / *Simple biospheres* (`[TIER-D]` · Target `vid=3108`)
- **Metric (`raw_metric`):** "Metabolismo + reproducción + ecosistemas microbianos → planeta químicamente transformado" (*Metabolism + reproduction + microbial ecosystems → chemically transformed planet*) · `scene=8` · `footer="UN MUNDO VIVO PUEDE SER MICROSCÓPICO"` · `evidence_class`: "Vida microscópica · efectos planetarios" (*Microscopic life planetary effects*) · `range=[0.320, 0.360]`
- **Spoken Macro (ES // EN):** Un planeta vivo no necesita bosques, animales ni inteligencia: durante gran parte de la historia terrestre, la vida fue exclusivamente microscópica. // *A living planet does not need forests, animals or intelligence: for much of Earth's history, life was exclusively microscopic.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Renders macro habitable rings (`210`) + continents instead of the 3-billion-year microbial world: single prokaryotic cells, layered microbial mats/stromatolites, anaerobic sulfur/iron metabolisms, oxygenic photosynthesis cyanobacteria, deep-sea chemosynthesis, endolithic subsurface rock biofilms, dormant spores, and microbial atmospheric transformation.
- **Zone 3D Procedural Blueprint:** Microbial Biosphere & Stromatolite Stage (vid=3108): cross-section from sunlit shallow lagoon down into dark deep basalt cracks

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **células individuales** / *single cells* | `[TIER-D]` | Un solo organismo microscópico puede detectar recursos, mantener metabolismo y reproducirse. // *A single microscopic organism can detect resources, maintain metabolism, and reproduce.* | `vid=210` ignores `scene=8` & `doc_feat=0` | flagellated single prokaryotic cell dividing |
| `F1` | **ecosistemas microbianos** / *microbial ecosystems* | `[TIER-D]` | Comunidades de microbios intercambian nutrientes, señales y productos metabólicos. // *Communities of microbes exchange nutrients, signals, and metabolic products.* | `vid=210` ignores `scene=8` & `doc_feat=1` | multi-colored layered biofilm/stromatolite dome exchanging nutrient packets |
| `F2` | **metabolismo anaerobio** / *anaerobic metabolism* | `[TIER-D]` | La vida puede obtener energía sin oxígeno mediante muchas rutas químicas. // *Life can obtain energy without oxygen through many chemical routes.* | `vid=210` ignores `scene=8` & `doc_feat=2` | violet anaerobic purple-sulfur cloud devoid of O2 |
| `F3` | **fotosíntesis** / *photosynthesis* | `[TIER-D]` | Capturar energía luminosa puede transformar atmósferas y ampliar la productividad planetaria. // *Capturing light energy can transform atmospheres and expand planetary productivity.* | `vid=210` ignores `scene=8` & `doc_feat=3` | golden sunlight beams hitting teal cyanobacteria chains liberating bright cyan O2 bubbles |
| `F4` | **quimiosíntesis** / *chemosynthesis* | `[TIER-D]` | Reacciones entre minerales y fluidos sostienen ecosistemas independientes de la luz solar. // *Reactions between minerals and fluids sustain ecosystems independent of sunlight.* | `vid=210` ignores `scene=8` & `doc_feat=4` | dark abyssal chimney mats oxidizing H2S cleanly without sunlight |
| `F5` | **vida subterránea** / *underground life* | `[TIER-D]` | Rocas y sedimentos pueden proteger organismos de radiación, sequedad y cambios superficiales. // *Rocks and sediments can protect organisms from radiation, dryness, and surface changes.* | `vid=210` ignores `scene=8` & `doc_feat=5` | glowing micro-colonies lined along deep rock pore fractures |
| `F6` | **latencia** / *dormancy* | `[TIER-D]` | Algunos sistemas reducen su actividad durante periodos adversos sin permanecer indefinidamente viables. // *Some systems reduce their activity during adverse periods without remaining viable indefinitely.* | `vid=210` ignores `scene=8` & `doc_feat=6` | armored dormant cyst shell hibernating in ice |
| `F7` | **huella planetaria** / *planetary footprint* | `[TIER-D]` | Una biosfera extensa puede alterar gases, minerales e isótopos a escala global. // *An extensive biosphere can alter gases, minerals and isotopes on a global scale.* | `vid=210` ignores `scene=8` & `doc_feat=7` | micro-to-macro zoom showing microbial haze tinting sky chemistry |

#### `Exp 21 Z09` `complex-life` — Vida compleja / *complex life* (`[TIER-D]` · Target `vid=3109`)
- **Metric (`raw_metric`):** "Células complejas + cooperación + diferenciación + ecología → organismos multicelulares" (*Complex cells + cooperation + differentiation + ecology → multicellular organisms*) · `scene=9` · `footer="COMPLEJIDAD: POSIBILIDAD, NO DESTINO"` · `evidence_class`: "Transiciones adicionales · resultado no inevitable" (*Additional transitions · non-inevitable result*) · `range=[0.360, 0.400]`
- **Spoken Macro (ES // EN):** La aparición de vida simple no implica que la evolución produzca inevitablemente organismos complejos. // *The emergence of simple life does not imply that evolution inevitably produces complex organisms.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Zero representation of Great Oxidation toxicity, complex eukaryotic internal organelles, mitochondrial endosymbiosis, multicellular aggregation, cellular differentiation into organs, ecological arms races, or primitive neural nets — just static star + rings (`210`) + Earth/Europa spheres.
- **Zone 3D Procedural Blueprint:** Eukaryogenesis & Multicellular Morphogenesis (vid=3109)

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **oxigenación** / *oxygenation* | `[TIER-D]` | El oxígeno amplía algunas rutas energéticas, pero fue tóxico para muchos organismos y no garantiza complejidad. // *Oxygen expands some energy pathways, but was toxic to many organisms and did not guarantee complexity.* | `vid=210` ignores `scene=9` & `doc_feat=0` | cyan O2 gas cloud oxidizing rust bands and wiping vulnerable anaerobes |
| `F1` | **células complejas** / *complex cells* | `[TIER-D]` | Las células eucariotas contienen compartimentos internos y surgieron tras una historia evolutiva prolongada. // *Eukaryotic cells contain internal compartments and emerged after a long evolutionary history.* | `vid=210` ignores `scene=9` & `doc_feat=1` | translucent eukaryotic cell housing bright nucleus, ER folds & glowing mitochondria |
| `F2` | **simbiosis** / *symbiosis* | `[TIER-D]` | La cooperación íntima entre organismos puede originar nuevas capacidades y nuevas unidades evolutivas. // *Intimate cooperation between organisms can give rise to new capabilities and new evolutionary units.* | `vid=210` ignores `scene=9` & `doc_feat=2` | larger host engulfing glowing proto-mitochondrion bacterium into permanent symbiosis |
| `F3` | **multicelularidad** / *multicellularity* | `[TIER-D]` | Las células pueden permanecer unidas y coordinar crecimiento, reproducción y defensa. // *Cells can stay together and coordinate growth, reproduction and defense.* | `vid=210` ignores `scene=9` & `doc_feat=3` | loose Volvox-like sphere of adhering synchronized cells |
| `F4` | **diferenciación celular** / *cell differentiation* | `[TIER-D]` | Distintos tipos celulares permiten tejidos, órganos y cuerpos con funciones especializadas. // *Different cell types allow tissues, organs and bodies with specialized functions.* | `vid=210` ignores `scene=9` & `doc_feat=4` | stem cells differentiating into specialized luminescent cyan nerve, emerald leaf & crimson muscle clusters |
| `F5` | **competencia ecológica** / *ecological competition* | `[TIER-C]` | Depredación, recursos y cooperación modifican las trayectorias evolutivas sin dirigirlas hacia una meta. // *Predation, resources and cooperation modify evolutionary trajectories without directing them towards a goal.* | `vid=210` ignores `scene=9` & `doc_feat=5` | predator-prey spiral chase without fixed ladder arrow |
| `F6` | **sistemas nerviosos** / *nervous systems* | `[TIER-D]` | Coordinar movimiento y percepción favorece redes de procesamiento, pero no todas las formas complejas los necesitan. // *Coordinating movement and perception favors processing networks, but not all complex life forms need them.* | `vid=210` ignores `scene=9` & `doc_feat=6` | glowing branching synaptic ganglion network pulsing action potentials |
| `F7` | **tiempo estable** / *stable time* | `[TIER-C]` | Las transiciones complejas pueden requerir miles de millones de años y sobrevivir a crisis planetarias. // *Complex transitions can require billions of years and survive planetary crises.* | `vid=210` ignores `scene=9` & `doc_feat=7` | compressed deep-time survival spiral passing extinction shock rings |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP21-Z05-09)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 21 (21_cosmic_life) Zones Z05..Z09 (`real-habitability`, `prebiotic-chemistry`, `origin-of-life`, `simple-biospheres`, `complex-life`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z05->3105`, `Z06->3106`, `Z07->3107`, `Z08->3108`, `Z09->3109` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `astrobiologia`, `habitabilidad`, `agua`, `exoplaneta`, `biofirma`, `extremofilo`, `encelado` & `europa` (exist), `tránsito`, `espectro`, `biosfera`, `tecnofirma`, `fermi`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_civilization_horizon.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z05->3105`, `Z06->3106`, `Z07->3107`, `Z08->3108`, `Z09->3109`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z05 (vid=3105, real-habitability): F0:atmosphere; F1:surface pressure; F2:greenhouse effect; F3:liquid solvents; F4:geological cycles; F5:magnetic protection; F6:climate stability; F7:available energy
     * Z06 (vid=3106, prebiotic-chemistry): F0:organic molecules; F1:amino acids; F2:nucleotides; F3:lipid membranes; F4:catalytic minerals; F5:hydrothermal vents; F6:energy gradients; F7:self-assembly
     * Z07 (vid=3107, origin-of-life): F0:replicating molecules; F1:metabolism first; F2:RNA world; F3:protocells; F4:compartmentalization; F5:inheritance; F6:variation; F7:several routes
     * Z08 (vid=3108, simple-biospheres): F0:single cells; F1:microbial ecosystems; F2:anaerobic metabolism; F3:photosynthesis; F4:chemosynthesis; F5:underground life; F6:dormancy; F7:planetary footprint
     * Z09 (vid=3109, complex-life): F0:oxygenation; F1:complex cells; F2:symbiosis; F3:multicellularity; F4:cell differentiation; F5:ecological competition; F6:nervous systems; F7:stable time
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp21-z10-14"></a>
### PKG-ACT2-EXP21-Z10-14: Extremophiles, Europa Plumes, Exoplanet Detection & Biosignatures (`z=10..14`) — `Phase 1` · Priority: **Critical**

**Current State:** Zones `Z10..Z14` (`5` zones, `40` features) | Zone Tiers `0A / 3B / 0C / 2D` | Feature Tiers `0A / 7B / 0C / 33D` | Render path: [`doc_stages.rs#L75-L78`](../../crates/cosmos_render/src/doc_stages.rs#L75-L78) -> [`shaders/hero_doc_civilization_horizon.wgsl#L363-L388`](../../shaders/hero_doc_civilization_horizon.wgsl#L363-L388) — `z=12 -> vid=212`, all other `24` zones -> `vid=210` + twin `11/10` globes (`sc` ignored). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `40` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z10` | `extremophiles` | Extremófilos / *Extremophiles* | **`3110`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z11` | `mars` | Marte / *Mars* | **`3111`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z12` | `ocean-worlds` | Mundos oceánicos / *ocean worlds* | **`3112`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z13` | `exoplanets` | Exoplanetas / *exoplanets* | **`3113`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z14` | `biosignatures` | Biofirmas / *Biosignatures* | **`3114`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 21 Z10` `extremophiles` — Extremófilos / *Extremophiles* (`[TIER-D]` · Target `vid=3110`)
- **Metric (`raw_metric`):** "Vida establecida + selección + tiempo → adaptación a condiciones extremas" (*Established life + selection + time → adaptation to extreme conditions*) · `scene=10` · `footer="ADAPTACIÓN EXTREMA · ORIGEN NO DEMOSTRADO"` · `evidence_class`: "Límites de adaptación · no prueba de origen universal" (*Limits of adaptation no proof of universal origin*) · `range=[0.400, 0.440]`
- **Spoken Macro (ES // EN):** Los extremófilos no demuestran que la vida pueda originarse en cualquier condición extrema. Demuestran que la vida, una vez establecida, puede adaptarse a ambientes antes considerados inhabitables. // *Extremophiles do not prove that life can originate in any extreme condition. They demonstrate that life, once established, can adapt to environments previously considered uninhabitable.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Every feature names a concrete extreme physical habitat and adaptive mechanism — boiling hydrothermal chimneys (>110C), sub-zero liquid brine veins in glacial ice, strong acid/alkali pumps, high-dose gamma DNA repair, crushing abyssal pressure, kilometers-deep fracture rock pores, ultra-slow energetic metabolism — none visible in `210`.
- **Zone 3D Procedural Blueprint:** 8-Niche Extremophile Habitat Matrix (vid=3110): split radial carousel or dynamically re-themed harsh environment chamber

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **temperaturas altas** / *high temperatures* | `[TIER-D]` | Algunos organismos mantienen proteínas y membranas funcionales cerca de fuentes hidrotermales. // *Some organisms maintain functional proteins and membranes near hydrothermal vents.* | `vid=210` ignores `scene=10` & `doc_feat=0` | superheated black smoker plume with intact glowing lipid-monolayer microbes |
| `F1` | **frío extremo** / *extreme cold* | `[TIER-D]` | La vida puede ralentizarse, usar anticongelantes y ocupar películas microscópicas de agua. // *Life can slow down, use antifreeze and occupy microscopic films of water.* | `vid=210` ignores `scene=10` & `doc_feat=1` | microscopic liquid brine channel winding through crystalline blue icebergs |
| `F2` | **acidez** / *acidity* | `[TIER-D]` | Organismos acidófilos regulan su química interna en medios que dañan la mayoría de las células. // *Acidophilic organisms regulate their internal chemistry in environments that damage most cells.* | `vid=210` ignores `scene=10` & `doc_feat=2` | bubbling neon-amber sulfuric acid pool with proton-pumping cells |
| `F3` | **salinidad** / *salinity* | `[TIER-D]` | Los halófilos equilibran grandes concentraciones de sales mediante moléculas y bombas celulares. // *Halophiles balance large concentrations of salts through cellular molecules and pumps.* | `vid=210` ignores `scene=10` & `doc_feat=3` | pink hypersaline salt-crystal lattice |
| `F4` | **radiación** / *radiation* | `[TIER-D]` | Ciertos microbios reparan daños genéticos con eficacia, pero ningún organismo es invulnerable. // *Certain microbes repair genetic damage effectively, but no organism is invulnerable.* | `vid=210` ignores `scene=10` & `doc_feat=4` | blue Cherenkov radiation tracks fragmenting DNA followed by rapid golden enzymatic stitch repair |
| `F5` | **presión intensa** / *intense pressure* | `[TIER-D]` | La vida de aguas profundas adapta membranas y proteínas a presiones enormes. // *Deep-sea life adapts membranes and proteins to enormous pressures.* | `vid=210` ignores `scene=10` & `doc_feat=5` | crushed deep trench ocean floor (1000 bar arrows) |
| `F6` | **roca profunda** / *deep rock* | `[TIER-D]` | Comunidades lentas viven en poros y fracturas, alimentadas por reacciones entre agua y minerales. // *Slow communities live in pores and fractures, fueled by reactions between water and minerals.* | `vid=210` ignores `scene=10` & `doc_feat=6` | solid basalt grain micro-pores glowing deep beneath surface |
| `F7` | **energía mínima** / *minimum energy* | `[TIER-D]` | Algunos metabolismos operan cerca de límites energéticos y pueden tardar muchísimo en reproducirse. // *Some metabolisms operate near energetic limits and can take a long time to reproduce.* | `vid=210` ignores `scene=10` & `doc_feat=7` | ultra-slow dimming energy trickle sustaining millennial division tick |

#### `Exp 21 Z11` `mars` — Marte / *Mars* (`[TIER-D]` · Target `vid=3111`)
- **Metric (`raw_metric`):** "Geología + química + muestras + contexto → evaluación de habitabilidad marciana" (*Geology + chemistry + samples + context → Martian habitability assessment*) · `scene=11` · `footer="AGUA ANTIGUA · EVIDENCIA BIOLÓGICA NO CONFIRMADA"` · `evidence_class`: "Agua antigua · vida todavía no demostrada" (*Ancient water · life not yet demonstrated*) · `range=[0.440, 0.480]`
- **Spoken Macro (ES // EN):** Evidencia de agua, ambiente habitable, química orgánica y posible evidencia biológica son niveles distintos; ninguno demuestra por sí solo que haya existido vida. // *Evidence of water, habitable environment, organic chemistry and possible biological evidence are different levels; None alone proves that life existed.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** CRITICAL MISMATCH: Narration specifically studies Mars (`marte` keyword hit!) — ancient Noachian river valleys/deltas, crater lake sedimentary strata, lost early atmosphere, subsurface polar ice, abiotic organic detections, destructive surface UV/cosmic radiation, shielded deep aquifers, and sample-return contamination controls. Yet Exp 21 Z11 emits `210` + `vid=11` (a bright green forested Earth-like exoplanet!) + `vid=10` (Europa).
- **Zone 3D Procedural Blueprint:** Red-Planet Paleoclimate & Mars Stratigraphy (vid=3111): suppress generic `vid=11`/`10` side spheres; promote dedicated dusty rust-red Mars globe (`marte` shader) with time-slider morphology

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **ríos antiguos** / *ancient rivers* | `[TIER-D]` | Valles y deltas indican que agua líquida fluyó sobre la superficie marciana en el pasado. // *Valleys and deltas indicate that liquid water flowed over the Martian surface in the past.* | `vid=210` ignores `scene=11` & `doc_feat=0` | winding dendritic paleorivers & Jezero-style delta fan |
| `F1` | **sedimentos lacustres** / *lake sediments* | `[TIER-D]` | Capas depositadas en antiguos lagos pueden conservar minerales, moléculas y estructuras delicadas. // *Layers deposited in ancient lakes can preserve minerals, molecules and delicate structures.* | `vid=210` ignores `scene=11` & `doc_feat=1` | cutaway crater-lake stratified clay/sulfate layers |
| `F2` | **atmósfera perdida** / *lost atmosphere* | `[TIER-D]` | Marte tuvo una atmósfera más densa y perdió gran parte de ella durante su evolución. // *Mars had a denser atmosphere and lost much of it during its evolution.* | `vid=210` ignores `scene=11` & `doc_feat=2` | thick blue early atmosphere eroding away into solar-wind tail |
| `F3` | **hielo subterráneo** / *underground ice* | `[TIER-D]` | Reservas de agua congelada sobreviven bajo el suelo y en regiones polares. // *Reserves of frozen water survive underground and in polar regions.* | `vid=210` ignores `scene=11` & `doc_feat=3` | translucent X-ray reveal of bright white polar caps & buried mid-latitude glacier ice |
| `F4` | **química orgánica** / *organic chemistry* | `[TIER-D]` | Se han detectado compuestos de carbono, pero pueden originarse mediante procesos no biológicos. // *Carbon compounds have been detected, but they may originate through non-biological processes.* | `vid=210` ignores `scene=11` & `doc_feat=4` | wispy organic carbon ring tags inside mudstone |
| `F5` | **radiación superficial** / *surface radiation* | `[TIER-D]` | La atmósfera tenue y la falta de protección global exponen la superficie a radiación destructiva. // *The thin atmosphere and lack of global shielding expose the surface to destructive radiation.* | `vid=210` ignores `scene=11` & `doc_feat=5` | intense UV/proton particle shower sterilizing exposed red dust |
| `F6` | **hábitats profundos** / *deep habitats* | `[TIER-D]` | El subsuelo podría conservar agua, calor y protección durante más tiempo que la superficie. // *The subsoil could retain water, heat and protection for longer than the surface.* | `vid=210` ignores `scene=11` & `doc_feat=6` | protected wet deep-crust thermal aquifer band beneath barren ground |
| `F7` | **análisis de muestras** / *sample analysis* | `[TIER-D]` | La interpretación exige controlar contaminación, historia geológica y múltiples explicaciones alternativas. // *Interpretation requires controlling contamination, geological history, and multiple alternative explanations.* | `vid=210` ignores `scene=11` & `doc_feat=7` | sterile sample-tube core drill surrounded by cleanroom contamination filter cone |

#### `Exp 21 Z12` `ocean-worlds` — Mundos oceánicos / *ocean worlds* (`[TIER-B]` · Target `vid=3112`)
- **Metric (`raw_metric`):** "Océano + calor de marea + química rocosa + tiempo → ambiente subterráneo potencial" (*Ocean + tidal heat + rock chemistry + time → potential underground environment*) · `scene=12` · `footer="OCÉANOS OCULTOS BAJO KILÓMETROS DE HIELO"` · `evidence_class`: "Habitabilidad bajo el hielo · energía sin superficie terrestre" (*Habitability under ice · energy without land surface*) · `range=[0.480, 0.520]`
- **Spoken Macro (ES // EN):** La habitabilidad no requiere necesariamente una superficie similar a la Tierra. Un océano protegido bajo kilómetros de hielo podría contener ambientes químicos de larga duración. // *Habitability does not necessarily require an Earth-like surface. An ocean protected under miles of ice could contain long-lasting chemical environments.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** z==12 -> vid=212 (`mode=2`: Cracked Europa ice shell + 2 cryo-plumes; side spheres suppressed; `doc_feat` unused)
- **Audit Gap:** Bespoke z==12 -> vid=212 (`mode=2` in hero_doc_civilization_horizon.wgsl:363-388) renders a cracked Europa water-ice sphere + 2 bright cryo-plumes (directly matching F00 Europa and F01 Enceladus), and cleanly omits the twin side spheres (`doc_stages.rs:764`). However, `doc_feat` is completely unread: Titan's orange haze & methane lakes (`F02`), Ganymede's magnetosphere (`F03`), ice-shell cutaway (`F04`), Jovian tidal flexing (`F05`), seafloor hydrothermal chimneys (`F06`), and organics (`F07`) are unrendered.
- **Zone 3D Procedural Blueprint:** Upgrade vid=212 Interactive Ocean-World Showcase: preserve Europa ice+plumes as baseline and drive rich sub-stages via doc_feat: 0 Europa red-brown lineae ridges + Jupiter orbit indicator, 1 Enceladus south-polar tiger stripes feeding diffuse E-ring halo, 2 Titan opaque orange nitrogen/tholin haze cutaway revealing dark polar liquid hydrocarbon lakes + deep buried water ocean, 3 Ganymede multi-layer ice-sandwich + glowing cyan intrinsic dipole magnetosphere, 4 90-degree wedge cutaway exposing global blue liquid ocean above rocky silicate core, 5 animated ellipsoid tidal bulge flexing synchronously as giant planet sweeps overhead, 6 zoom to warm seafloor alkaline white/black hydrothermal vents, 7 descending oxidant/meteoritic organic particles mixing down fractures.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **Europa** / *Europa* | `[TIER-B]` | Su corteza helada probablemente cubre un océano global calentado por las mareas de Júpiter. // *Its icy crust likely covers a global ocean warmed by Jupiter's tides.* | `vid=210` ignores `scene=12` & `doc_feat=0` | Europa red-brown lineae ridges + Jupiter orbit indicator |
| `F1` | **Encélado** / *Enceladus* | `[TIER-B]` | Géiseres expulsan al espacio agua, sales, compuestos orgánicos y material procedente del océano interior. // *Geysers expel water, salts, organic compounds and material from the interior ocean into space.* | `vid=210` ignores `scene=12` & `doc_feat=1` | Enceladus south-polar tiger stripes feeding diffuse E-ring halo |
| `F2` | **Titán** / *Titan* | `[TIER-D]` | Posee lagos de hidrocarburos en la superficie y probablemente un océano de agua en profundidad. // *It has hydrocarbon lakes on the surface and probably an ocean of water at depth.* | `vid=210` ignores `scene=12` & `doc_feat=2` | Titan opaque orange nitrogen/tholin haze cutaway revealing dark polar liquid hydrocarbon lakes + deep buried water ocean |
| `F3` | **Ganímedes** / *Ganymede* | `[TIER-D]` | La luna más grande del Sistema Solar contiene capas de hielo y agua y posee campo magnético propio. // *The largest moon in the Solar System contains layers of ice and water and has its own magnetic field.* | `vid=210` ignores `scene=12` & `doc_feat=3` | Ganymede multi-layer ice-sandwich + glowing cyan intrinsic dipole magnetosphere |
| `F4` | **océanos internos** / *internal oceans* | `[TIER-D]` | El agua líquida puede mantenerse gracias a calor interno incluso muy lejos del Sol. // *Liquid water can be maintained thanks to internal heat even very far from the Sun.* | `vid=210` ignores `scene=12` & `doc_feat=4` | 90-degree wedge cutaway exposing global blue liquid ocean above rocky silicate core |
| `F5` | **calentamiento de marea** / *tidal warming* | `[TIER-D]` | La gravedad variable de un planeta gigante deforma las lunas y genera calor. // *A giant planet's varying gravity warps moons and generates heat.* | `vid=210` ignores `scene=12` & `doc_feat=5` | animated ellipsoid tidal bulge flexing synchronously as giant planet sweeps overhead |
| `F6` | **actividad hidrotermal** / *hydrothermal activity* | `[TIER-D]` | El contacto entre agua y roca podría producir gradientes químicos semejantes a los de fondos oceánicos terrestres. // *The contact between water and rock could produce chemical gradients similar to those of terrestrial ocean floors.* | `vid=210` ignores `scene=12` & `doc_feat=6` | zoom to warm seafloor alkaline white/black hydrothermal vents |
| `F7` | **química orgánica** / *organic chemistry* | `[TIER-D]` | Materiales carbonados pueden llegar por impactos, formarse internamente o mezclarse entre océano y superficie. // *Carbonaceous materials can arrive through impacts, form internally or mix between the ocean and the surface.* | `vid=210` ignores `scene=12` & `doc_feat=7` | descending oxidant/meteoritic organic particles mixing down fractures |

#### `Exp 21 Z13` `exoplanets` — Exoplanetas / *exoplanets* (`[TIER-B]` · Target `vid=3113`)
- **Metric (`raw_metric`):** "Luz estelar + movimiento + modelos → masa, radio, órbita y atmósfera inferidos" (*Starlight + motion + models → inferred mass, radius, orbit and atmosphere*) · `scene=13` · `footer="MUNDOS INFERIDOS A PARTIR DE SU LUZ"` · `evidence_class`: "Mundos reconstruidos a partir de señales diminutas" (*Worlds reconstructed from tiny signs*) · `range=[0.520, 0.560]`
- **Spoken Macro (ES // EN):** La mayoría de los exoplanetas no se observan como mundos detallados: sus propiedades se reconstruyen a partir de cambios muy pequeños en la luz de sus estrellas. // *Most exoplanets are not observed as detailed worlds: their properties are reconstructed from very small changes in the light from their stars.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Fallback `210` (star + single orbiting planet dot) + twin `kind=1, vid=11` (`Exoplanet Biosignature World`) matches the macro subject of exoplanets (`F03 mass`, `F04 radius`, `F06 orbital period`), but `doc_feat` only shifts orbital angle (`*0.45`) and ignores all iconic observational methods (`F00` transit light curve, `F01` RV Doppler wobble, `F02` coronagraph mask, `F05` bulk density cutaways, `F07` transmission atmospheric spectra).
- **Zone 3D Procedural Blueprint:** Exoplanet Detection & Characterization Lab (upgrade vid=3113 + companion vid=3820/3823)

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **tránsito** / *transit* | `[TIER-D]` | Un planeta que cruza frente a su estrella produce una disminución periódica de brillo. // *A planet crossing in front of its star produces a periodic decrease in brightness.* | `vid=210` ignores `scene=13` & `doc_feat=0` | edge-on transit across stellar disk with real-time glowing photometric dip curve below |
| `F1` | **velocidad radial** / *radial velocity* | `[TIER-D]` | El tirón gravitatorio del planeta mueve ligeramente la estrella y desplaza sus líneas espectrales. // *The planet's gravitational pull moves the star slightly and shifts its spectral lines.* | `vid=210` ignores `scene=13` & `doc_feat=1` | exaggerated stellar barycenter wobble shifting spectral barcode lines red<->blue |
| `F2` | **imagen directa** / *direct image* | `[TIER-D]` | Bloquear el brillo estelar permite detectar algunos planetas grandes, jóvenes o muy separados. // *Blocking the stellar brightness allows us to detect some large, young or very separated planets.* | `vid=210` ignores `scene=13` & `doc_feat=2` | black circular coronagraph starshade mask suppressing blinding central star glare to reveal faint warm orbital planet pinpoints |
| `F3` | **masa** / *mass* | `[TIER-B]` | La masa ayuda a distinguir mundos rocosos, ricos en volátiles o gigantes gaseosos. // *Mass helps distinguish rocky, volatile-rich worlds or gas giants.* | `vid=210` ignores `scene=13` & `doc_feat=3` | comparative mass/radius spheres from super-Earth to hot-Jupiter |
| `F4` | **radio** / *radius* | `[TIER-B]` | El tamaño combinado con la masa permite estimar densidad y composición probable. // *Size combined with mass allows estimating density and probable composition.* | `vid=210` ignores `scene=13` & `doc_feat=4` | comparative mass/radius spheres from super-Earth to hot-Jupiter |
| `F5` | **densidad** / *density* | `[TIER-D]` | Dos planetas del mismo tamaño pueden contener proporciones muy distintas de roca, agua y gas. // *Two planets of the same size can contain very different proportions of rock, water and gas.* | `vid=210` ignores `scene=13` & `doc_feat=5` | 3-way cutaway comparing iron-silicate terrestrial, deep high-pressure H2O ocean-world, and puffy H/He mini-Neptune |
| `F6` | **periodo orbital** / *orbital period* | `[TIER-B]` | La repetición de señales revela cuánto tarda el planeta en completar una órbita. // *Repetition of signals reveals how long it takes the planet to complete an orbit.* | `vid=210` ignores `scene=13` & `doc_feat=6` | Keplerian period timer ring |
| `F7` | **espectros atmosféricos** / *atmospheric spectra* | `[TIER-D]` | La luz filtrada o emitida por una atmósfera puede revelar moléculas, nubes y temperatura. // *Light filtered or emitted by an atmosphere can reveal molecules, clouds, and temperature.* | `vid=210` ignores `scene=13` & `doc_feat=7` | backlight starlight filtering through thin translucent atmospheric limb breaking into colored absorption rainbow |

#### `Exp 21 Z14` `biosignatures` — Biofirmas / *Biosignatures* (`[TIER-B]` · Target `vid=3114`)
- **Metric (`raw_metric`):** "Múltiples observaciones + desequilibrio + contexto → hipótesis biológica contrastable" (*Multiple observations + imbalance + context → testable biological hypothesis*) · `scene=14` · `footer="UNA SEÑAL AISLADA NO BASTA"` · `evidence_class`: "Señales posibles · requieren contexto planetario" (*Possible signals require planetary context*) · `range=[0.560, 0.600]`
- **Spoken Macro (ES // EN):** Una biofirma aislada puede tener explicaciones no biológicas. La evidencia más convincente combinaría varias señales independientes dentro de un contexto planetario coherente. // *An isolated biosignature may have non-biological explanations. The most compelling evidence would combine several independent signals within a coherent planetary context.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Twin surface sphere `kind=1, vid=11` (`Exoplanet Biosignature World` with green continents and oceans) visually anchors `F00 atmospheric oxygen` and `F04 surface pigments` at the shared-archetype level, yet `doc_feat` is ignored: stratospheric ozone (`F01`), trace methane (`F02`), O2+CH4 chemical disequilibrium (`F03`), seasonal oscillation (`F05`), organic pattern bias (`F06`), and isotopic fractionation (`F07`) have zero representation.
- **Zone 3D Procedural Blueprint:** Exoplanet Biosignature Context Dashboard (upgrade vid=11/210 for Z14)

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **oxígeno atmosférico** / *atmospheric oxygen* | `[TIER-B]` | La fotosíntesis puede producir oxígeno, pero ciertos procesos fotoquímicos también lo generan. // *Photosynthesis can produce oxygen, but certain photochemical processes also generate it.* | `vid=210` ignores `scene=14` & `doc_feat=0` | pulsing atmospheric O2 A-band limb glow |
| `F1` | **ozono** / *ozone* | `[TIER-D]` | El ozono puede indicar oxígeno y protege de radiación ultravioleta, aunque su interpretación depende de la estrella. // *Ozone can indicate oxygen and protects from ultraviolet radiation, although its interpretation depends on the star.* | `vid=210` ignores `scene=14` & `doc_feat=1` | pale-blue stratospheric O3 shell blocking incoming violet UV spikes |
| `F2` | **metano** / *methane* | `[TIER-D]` | Puede ser biológico, volcánico, hidrotermal o producido por reacciones entre agua y roca. // *It can be biological, volcanic, hydrothermal or produced by reactions between water and rock.* | `vid=210` ignores `scene=14` & `doc_feat=2` | emerald-cyan volcanic/biogenic CH4 puffs swirling up |
| `F3` | **desequilibrio químico** / *chemical imbalance* | `[TIER-D]` | Gases que reaccionarían entre sí pueden requerir una fuente continua, posiblemente biológica. // *Gases that would react with each other may require a continuous source, possibly biological.* | `vid=210` ignores `scene=14` & `doc_feat=3` | glowing coupled disequilibrium loop showing O2+CH4 annihilating unless continually replenished by biosphere ring |
| `F4` | **pigmentos superficiales** / *surface pigments* | `[TIER-B]` | Organismos fotosintéticos podrían modificar el color reflejado por continentes u océanos. // *Photosynthetic organisms could modify the color reflected by continents or oceans.* | `vid=210` ignores `scene=14` & `doc_feat=4` | near-IR vegetation 'red edge' false-color shift scanning green continents |
| `F5` | **variación estacional** / *seasonal variation* | `[TIER-D]` | Cambios periódicos pueden seguir ciclos biológicos, climáticos o geológicos. // *Periodic changes can follow biological, climatic or geological cycles.* | `vid=210` ignores `scene=14` & `doc_feat=5` | axially tilted world pulsing winter/summer green-vs-white ice hemispheres + sawtooth CO2 curve |
| `F6` | **distribución orgánica** / *organic distribution* | `[TIER-D]` | Patrones moleculares selectivos podrían resultar más difíciles de explicar mediante química aleatoria. // *Selective molecular patterns could be more difficult to explain by random chemistry.* | `vid=210` ignores `scene=14` & `doc_feat=6` | selective homochiral helical cluster vs racemic scatter |
| `F7` | **isótopos** / *isotopes* | `[TIER-D]` | La vida puede preferir ciertos isótopos, pero los procesos geológicos también fraccionan elementos. // *Life may prefer certain isotopes, but geological processes also fractionate elements.* | `vid=210` ignores `scene=14` & `doc_feat=7` | weighted C12 vs C13 isotope sorting stream |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP21-Z10-14)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 21 (21_cosmic_life) Zones Z10..Z14 (`extremophiles`, `mars`, `ocean-worlds`, `exoplanets`, `biosignatures`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z10->3110`, `Z11->3111`, `Z12->3112`, `Z13->3113`, `Z14->3114` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `astrobiologia`, `habitabilidad`, `agua`, `exoplaneta`, `biofirma`, `extremofilo`, `encelado` & `europa` (exist), `tránsito`, `espectro`, `biosfera`, `tecnofirma`, `fermi`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_civilization_horizon.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z10->3110`, `Z11->3111`, `Z12->3112`, `Z13->3113`, `Z14->3114`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z10 (vid=3110, extremophiles): F0:high temperatures; F1:extreme cold; F2:acidity; F3:salinity; F4:radiation; F5:intense pressure; F6:deep rock; F7:minimum energy
     * Z11 (vid=3111, mars): F0:ancient rivers; F1:lake sediments; F2:lost atmosphere; F3:underground ice; F4:organic chemistry; F5:surface radiation; F6:deep habitats; F7:sample analysis
     * Z12 (vid=3112, ocean-worlds): F0:Europa; F1:Enceladus; F2:Titan; F3:Ganymede; F4:internal oceans; F5:tidal warming; F6:hydrothermal activity; F7:organic chemistry
     * Z13 (vid=3113, exoplanets): F0:transit; F1:radial velocity; F2:direct image; F3:mass; F4:radius; F5:density; F6:orbital period; F7:atmospheric spectra
     * Z14 (vid=3114, biosignatures): F0:atmospheric oxygen; F1:ozone; F2:methane; F3:chemical imbalance; F4:surface pigments; F5:seasonal variation; F6:organic distribution; F7:isotopes
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp21-z15-19"></a>
### PKG-ACT2-EXP21-Z15-19: Technosignatures, Great Filter & Fermi Silence (`z=15..19`) — `Phase 1` · Priority: **Critical**

**Current State:** Zones `Z15..Z19` (`5` zones, `40` features) | Zone Tiers `0A / 0B / 0C / 5D` | Feature Tiers `0A / 0B / 8C / 32D` | Render path: [`doc_stages.rs#L75-L78`](../../crates/cosmos_render/src/doc_stages.rs#L75-L78) -> [`shaders/hero_doc_civilization_horizon.wgsl#L363-L388`](../../shaders/hero_doc_civilization_horizon.wgsl#L363-L388) — `z=12 -> vid=212`, all other `24` zones -> `vid=210` + twin `11/10` globes (`sc` ignored). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `40` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z15` | `false-positives` | Falsos positivos / *False positives* | **`3115`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z16` | `evolutionary-intelligence` | Inteligencia evolutiva / *evolutionary intelligence* | **`3116`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z17` | `alien-intelligence-forms` | Formas de inteligencia alienígena / *Forms of alien intelligence* | **`3117`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z18` | `technosignatures` | Tecnofirmas / *Technosignatures* | **`3118`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z19` | `fermi-paradox` | Paradoja de Fermi / *Fermi paradox* | **`3119`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 21 Z15` `false-positives` — Falsos positivos / *False positives* (`[TIER-D]` · Target `vid=3115`)
- **Metric (`raw_metric`):** "Señal observada × alternativas × repetición × control → grado de confianza" (*Observed signal × alternatives × repetition × control → degree of confidence*) · `scene=15` · `footer="SEÑAL → ALTERNATIVAS → VERIFICACIÓN"` · `evidence_class`: "Interpretaciones rivales · evidencia extraordinaria" (*Rival interpretations · extraordinary evidence*) · `range=[0.600, 0.640]`
- **Spoken Macro (ES // EN):** Detectar una molécula asociada con la vida no equivale a detectar un organismo. // *Detecting a molecule associated with life is not equivalent to detecting an organism.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Shows vibrant inhabited Earth-like continents (`vid=11`) and calm HZ rings (`210`) while narration specifically warns of sterile false positives: UV water-vapor photodissociation + hydrogen escape leaving abiotic O2 around dry orange-red planets, volcanic CH4 plumes, serpentinization rocks, instrument contamination, sensor noise, and incomplete atmospheric chemistry models.
- **Zone 3D Procedural Blueprint:** Abiotic False-Positive Mimic Gallery (vid=3115): swap lush green `vid=11` world for barren volcanic/desiccated exoplanet + live spectrometer comparison

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **oxígeno abiótico** / *abiotic oxygen* | `[TIER-D]` | La pérdida de agua o ciertas reacciones fotoquímicas pueden acumular oxígeno sin una biosfera. // *Water loss or certain photochemical reactions can accumulate oxygen without a biosphere.* | `vid=210` ignores `scene=15` & `doc_feat=0` | intense M-dwarf UV beam splitting H2O vapor while light hydrogen escapes into space leaving thick false O2 layer |
| `F1` | **metano volcánico** / *volcanic methane* | `[TIER-D]` | Volcanes y sistemas hidrotermales pueden producir gases que también aparecen en metabolismos. // *Volcanoes and hydrothermal systems can produce gases that also appear in metabolisms.* | `vid=210` ignores `scene=15` & `doc_feat=1` | erupting shield volcano pumping abiotic CH4 cloud |
| `F2` | **escape atmosférico** / *atmospheric escape* | `[TIER-D]` | La radiación estelar puede separar moléculas y eliminar hidrógeno, alterando profundamente la composición. // *Stellar radiation can separate molecules and remove hydrogen, profoundly altering the composition.* | `vid=210` ignores `scene=15` & `doc_feat=2` | desiccated thin-air planet losing tenuous steam halo |
| `F3` | **reacciones minerales** / *mineral reactions* | `[TIER-D]` | Agua, roca y calor producen compuestos orgánicos e hidrógeno sin intervención biológica. // *Water, rock and heat produce organic compounds and hydrogen without biological intervention.* | `vid=210` ignores `scene=15` & `doc_feat=3` | submerged dark olivine rock reacting with hot brine generating abiotic H2/hydrocarbons |
| `F4` | **contaminación** / *contamination* | `[TIER-D]` | Material terrestre o señales locales pueden introducir una falsa detección en muestras o instrumentos. // *Ground material or local signals can introduce false detection in samples or instruments.* | `vid=210` ignores `scene=15` & `doc_feat=4` | foreground Earth satellite/dust contamination streak crossing telescope aperture |
| `F5` | **ruido instrumental** / *instrumental noise* | `[TIER-D]` | Errores, calibración incompleta y artefactos de procesamiento pueden imitar patrones reales. // *Errors, incomplete calibration, and processing artifacts can mimic real patterns.* | `vid=210` ignores `scene=15` & `doc_feat=5` | noisy fluctuating FFT spectrum spikes mimicking narrow line |
| `F6` | **modelos incompletos** / *incomplete models* | `[TIER-C]` | Una atmósfera alienígena puede comportarse de formas no incluidas en nuestros modelos actuales. // *An alien atmosphere can behave in ways not included in our current models.* | `vid=210` ignores `scene=15` & `doc_feat=6` | cracked incomplete model wireframe overlay |
| `F7` | **observaciones conflictivas** / *conflicting observations* | `[TIER-C]` | La ciencia aumenta confianza cuando distintos instrumentos y métodos convergen en la misma explicación. // *Science increases confidence when different instruments and methods converge on the same explanation.* | `vid=210` ignores `scene=15` & `doc_feat=7` | dual independent telescopes converging on ground truth |

#### `Exp 21 Z16` `evolutionary-intelligence` — Inteligencia evolutiva / *evolutionary intelligence* (`[TIER-D]` · Target `vid=3116`)
- **Metric (`raw_metric`):** "Percepción + aprendizaje + cooperación + cultura → capacidades cognitivas contingentes" (*Perception + learning + cooperation + culture → contingent cognitive abilities*) · `scene=16` · `footer="LA EVOLUCIÓN NO TIENE UNA ESCALERA ÚNICA"` · `evidence_class`: "Una posibilidad entre muchas · no una meta" (*One possibility among many · not a goal*) · `range=[0.640, 0.680]`
- **Spoken Macro (ES // EN):** La evolución no trabaja hacia un objetivo predeterminado. La inteligencia tecnológica es una posibilidad evolutiva, no una etapa obligatoria. // *Evolution does not work towards a predetermined goal. Technological intelligence is an evolutionary possibility, not a mandatory stage.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Goldilocks star+planets (`210`) fails to depict the evolutionary cognition themes: sensory transduction loops, lifetime neural learning plasticity, non-cumulative tool manipulation, social cooperative packs, multi-modal signaling (chemical/acoustic/electrical/visual), cumulative multi-generation culture, abstract problem-solving, and non-human niche construction (beaver/coral/termite).
- **Zone 3D Procedural Blueprint:** Non-Linear Bush of Evolutionary Cognition (vid=3116): explicitly replace linear 'ladder' with radially branching phylogenetic bush of diverse luminescent nodes

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **procesamiento sensorial** / *sensory processing* | `[TIER-D]` | Los organismos construyen respuestas a partir de señales relevantes para su propio ambiente. // *Organisms construct responses based on signals relevant to their own environment.* | `vid=210` ignores `scene=16` & `doc_feat=0` | diverse sensory wave rings (echolocation/electroreception/chemo) |
| `F1` | **aprendizaje** / *learning* | `[TIER-D]` | Modificar conducta con la experiencia permite adaptación durante una vida sin alterar genes. // *Modifying behavior with experience allows adaptation over a lifetime without altering genes.* | `vid=210` ignores `scene=16` & `doc_feat=1` | dynamically rewiring plastic neural synaptome knot |
| `F2` | **uso de herramientas** / *use of tools* | `[TIER-D]` | Manipular objetos puede resolver problemas sin producir necesariamente tecnología acumulativa. // *Manipulating objects can solve problems without necessarily producing cumulative technology.* | `vid=210` ignores `scene=16` & `doc_feat=2` | simple mechanical crow/octopus tool lever interaction |
| `F3` | **cooperación social** / *social cooperation* | `[TIER-D]` | Grupos coordinados comparten vigilancia, alimento, crianza o conocimiento. // *Coordinated groups share vigilance, food, child-rearing, or knowledge.* | `vid=210` ignores `scene=16` & `doc_feat=3` | swarming cohesive pack synchronization vectors |
| `F4` | **comunicación** / *communication* | `[TIER-D]` | Señales químicas, acústicas, visuales o eléctricas pueden transmitir estados e intenciones. // *Chemical, acoustic, visual or electrical signals can transmit states and intentions.* | `vid=210` ignores `scene=16` & `doc_feat=4` | pulsing chemical/electric/acoustic wave bursts between nodes |
| `F5` | **cultura acumulativa** / *cumulative culture* | `[TIER-C]` | Las innovaciones se vuelven más complejas cuando sobreviven a individuos y generaciones. // *Innovations become more complex when they survive individuals and generations.* | `vid=210` ignores `scene=16` & `doc_feat=5` | stacked generational memory rings passing glowing artifacts forward |
| `F6` | **razonamiento abstracto** / *abstract reasoning* | `[TIER-C]` | Representar relaciones ausentes permite planificación, matemática y explicaciones generales. // *Representing absent relationships allows for planning, mathematics, and general explanations.* | `vid=210` ignores `scene=16` & `doc_feat=6` | suspended floating geometric abstraction graph |
| `F7` | **modificación ambiental** / *environmental modification* | `[TIER-D]` | Muchas especies transforman hábitats; la escala tecnológica humana es un caso extremo, no la definición de inteligencia. // *Many species transform habitats; the human technological scale is an extreme case, not the definition of intelligence.* | `vid=210` ignores `scene=16` & `doc_feat=7` | branching termite-mound/coral-reef habitat sculptor network |

#### `Exp 21 Z17` `alien-intelligence-forms` — Formas de inteligencia alienígena / *Forms of alien intelligence* (`[TIER-D]` · Target `vid=3117`)
- **Metric (`raw_metric`):** "Cuerpo + ambiente + tiempo + organización → formas posibles de mente" (*Body + environment + time + organization → possible forms of mind*) · `scene=17` · `footer="¿RECONOCERÍAMOS OTRA FORMA DE MENTE?"` · `evidence_class`: "Arquitecturas cognitivas sin modelo humano obligatorio" (*Cognitive architectures without mandatory human model*) · `range=[0.680, 0.720]`
- **Spoken Macro (ES // EN):** Podríamos no reconocer como inteligente a una entidad que no habla, no construye ciudades, no vive a nuestra velocidad o no posee un cuerpo individual. // *We might not recognize as intelligent an entity that does not speak, does not build cities, does not live at our speed, or does not possess an individual body.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Zero representation of alien cognitive architectures: solitary octopus-like problem solvers, decentralized swarm cognition, liquid-medium acoustic/electric oceanic minds, colonial superorganisms without clear individual borders, post-biological machine descendants, distributed hive minds, ultra-slow geological-timescale minds, and magnetic/neutrino/chemical sensory modes.
- **Zone 3D Procedural Blueprint:** Xenocognitive Morphologies Carousel (vid=3117): eight radically non-anthropomorphic glowing volumetric/network forms

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **inteligencia solitaria** / *solitary intelligence* | `[TIER-D]` | Una especie podría resolver problemas complejos sin sociedades grandes ni comunicación intensa. // *A species could solve complex problems without large societies or intense communication.* | `vid=210` ignores `scene=17` & `doc_feat=0` | lone self-contained bioluminescent fractal brain node |
| `F1` | **cognición de enjambre** / *swarm cognition* | `[TIER-D]` | Muchos agentes simples pueden producir decisiones colectivas sin un centro controlador. // *Many simple agents can produce collective decisions without a controlling center.* | `vid=210` ignores `scene=17` & `doc_feat=1` | boids/compute-particle murmuration flock making collective steering turns without leader |
| `F2` | **mente oceánica** / *oceanic mind* | `[TIER-D]` | Un organismo distribuido en un medio líquido podría percibir mediante corrientes, presión, química o electricidad. // *An organism distributed in a liquid medium could perceive through currents, pressure, chemistry or electricity.* | `vid=210` ignores `scene=17` & `doc_feat=2` | translucent submerged fluid sphere pulsing standing acoustic-electric interference fields |
| `F3` | **organismo distribuido** / *distributed organism* | `[TIER-D]` | La frontera entre individuo, colonia y ecosistema podría ser distinta de la terrestre. // *The border between individual, colony and ecosystem could be different from the terrestrial one.* | `vid=210` ignores `scene=17` & `doc_feat=3` | porous reef-like connected colony where nodes merge & bud |
| `F4` | **descendientes mecánicos** / *mechanical descendants* | `[TIER-D]` | Una civilización biológica podría ser reemplazada o continuada por sistemas artificiales. // *A biological civilization could be replaced or continued by artificial systems.* | `vid=210` ignores `scene=17` & `doc_feat=4` | crystalline geometric machine polyhedron self-assembling lattice |
| `F5` | **mente colectiva** / *collective mind* | `[TIER-D]` | Memorias y decisiones podrían estar repartidas entre numerosos cuerpos conectados. // *Memories and decisions could be distributed among numerous connected bodies.* | `vid=210` ignores `scene=17` & `doc_feat=5` | ultra-slow amber crystalline pulse morphing across simulated millennia |
| `F6` | **cognición lenta** / *slow cognition* | `[TIER-D]` | Procesos mentales podrían durar años o siglos y resultar invisibles en observaciones breves. // *Mental processes could last years or centuries and be invisible in brief observations.* | `vid=210` ignores `scene=17` & `doc_feat=6` | distributed wireless lattice spanning planet |
| `F7` | **sentidos desconocidos** / *unknown senses* | `[TIER-D]` | Una mente puede basarse en campos magnéticos, neutrinos, química fina u otras señales poco intuitivas para nosotros. // *A mind can be based on magnetic fields, neutrinos, fine chemistry or other signals that are unintuitive to us.* | `vid=210` ignores `scene=17` & `doc_feat=7` | shimmering dipole magnetic/chemical gradient tendrils replacing eyes/ears |

#### `Exp 21 Z18` `technosignatures` — Tecnofirmas / *Technosignatures* (`[TIER-D]` · Target `vid=3118`)
- **Metric (`raw_metric`):** "Energía organizada + artefactos + patrones no naturales → actividad tecnológica posible" (*Organized energy + artifacts + unnatural patterns → possible technological activity*) · `scene=18` · `footer="TECNOLOGÍA POSIBLE · INTENCIÓN DESCONOCIDA"` · `evidence_class`: "Efectos tecnológicos · intención y supervivencia desconocidas" (*Technological effects · unknown intent and survival*) · `range=[0.720, 0.760]`
- **Spoken Macro (ES // EN):** Una tecnofirma sería evidencia de actividad tecnológica, no necesariamente de una civilización todavía activa, amistosa o interesada en establecer contacto. // *A technosignature would be evidence of technological activity, not necessarily of a civilization still active, friendly or interested in establishing contact.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Note ironic gap: Exp 20 has Dyson rings and Exp 22 has a SETI dish+laser, YET Exp 21 Z18 (`Technosignatures`) renders ONLY the naive uninhabited Goldilocks planet (`210`+`vid=11`) with NO narrowband RF line, NO pulsed laser beacon, NO CFC atmospheric absorption, NO dark-side city lights, NO infrared Dyson waste heat, NO triangular/shuttered transit shapes, and NO interstellar probe.
- **Zone 3D Procedural Blueprint:** Technosignatures Observatory Suite (vid=3118): reuse/customize planet shader + orbital transit ring

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **radio artificial** / *artificial radio signal* | `[TIER-D]` | Emisiones estrechas, moduladas o repetitivas pueden destacar frente a fuentes naturales. // *Narrow, modulated or repetitive emissions can stand out compared to natural sources.* | `vid=210` ignores `scene=18` & `doc_feat=0` | crisp single-pixel narrowband waterfall spike cutting thermal noise |
| `F1` | **pulsos ópticos** / *optical pulses* | `[TIER-D]` | Destellos láser breves podrían transportar información o servir para propulsión. // *Brief laser flashes could carry information or serve for propulsion.* | `vid=210` ignores `scene=18` & `doc_feat=1` | nanosecond emerald laser flash bursting cleanly from planetary dark limb |
| `F2` | **contaminantes atmosféricos** / *air pollutants* | `[TIER-D]` | Moléculas industriales difíciles de producir naturalmente podrían permanecer detectables. // *Industrial molecules difficult to produce naturally could remain detectable.* | `vid=210` ignores `scene=18` & `doc_feat=2` | distinct industrial CFC/NO2 molecular IR spectral band overlay |
| `F3` | **iluminación nocturna** / *night lighting* | `[TIER-D]` | Una superficie con luz artificial intensa modificaría el brillo del lado oscuro de un planeta. // *A surface with intense artificial light would modify the brightness of a planet's dark side.* | `vid=210` ignores `scene=18` & `doc_feat=3` | terminator-turned exoplanet blazing golden city-light webs across night hemisphere |
| `F4` | **calor industrial** / *industrial heat* | `[TIER-D]` | Toda actividad energética genera calor residual, aunque distinguirlo de fuentes naturales es difícil. // *All energy activities generate waste heat, although distinguishing it from natural sources is difficult.* | `vid=210` ignores `scene=18` & `doc_feat=4` | excess glowing mid-infrared Carnot heat shell surrounding system |
| `F5` | **estructuras orbitales** / *orbital structures* | `[TIER-D]` | Grandes colectores, hábitats o redes podrían alterar curvas de tránsito y emisión infrarroja. // *Large collectors, habitats, or networks could alter transit light curves and infrared emissions.* | `vid=210` ignores `scene=18` & `doc_feat=5` | orbiting geometric Dyson/ring collectors casting weird sharp asymmetric transit dip curves |
| `F6` | **tránsitos diseñados** / *engineered transits* | `[TIER-D]` | Objetos con geometrías o secuencias improbables podrían actuar como balizas deliberadas. // *Objects with unlikely geometries or sequences could act as deliberate beacons.* | `vid=210` ignores `scene=18` & `doc_feat=6` | periodic artificial prime-timed shadow shutters |
| `F7` | **sondas interestelares** / *interstellar probes* | `[TIER-D]` | Artefactos pequeños podrían visitar sistemas sin que exista comunicación continua con su origen. // *Small artifacts could visit systems without continuous communication with their source.* | `vid=210` ignores `scene=18` & `doc_feat=7` | sleek metallic reflective Bracewell/interstellar probe glinting through outer belt |

#### `Exp 21 Z19` `fermi-paradox` — Paradoja de Fermi / *Fermi paradox* (`[TIER-D]` · Target `vid=3119`)
- **Metric (`raw_metric`):** "Número de mundos × tiempo cósmico × detectabilidad → expectativa comparada con evidencia" (*Number of worlds × cosmic time × detectability → expectation compared to evidence*) · `scene=19` · `footer="MUCHOS MUNDOS · NINGUNA SEÑAL CONFIRMADA"` · `evidence_class`: "Abundancia posible · silencio observado" (*Possible abundance · observed silence*) · `range=[0.760, 0.800]`
- **Spoken Macro (ES // EN):** La paradoja de Fermi no demuestra que no existan otras civilizaciones. Señala una tensión entre algunas expectativas y la ausencia actual de evidencia confirmada. // *The Fermi paradox does not prove that other civilizations do not exist. It points to a tension between some expectations and the current absence of confirmed evidence.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Shows a single tiny star + 2 planets (`210`) instead of the galactic Fermi Paradox tension: hundreds of billions of stars in the Milky Way spiral, ancient stars billions of years older than the Sun, slow sub-light galactic expansion waves vs the tiny fraction of parameter space SETI has actually searched.
- **Zone 3D Procedural Blueprint:** Galactic Fermi Horizon & Search Volume (vid=3119): switch backdrop/hero to translucent glowing Milky Way spiral disk (`compute_universe.wgsl` galaxy particles!)

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **muchísimas estrellas** / *many stars* | `[TIER-D]` | La galaxia contiene enormes cantidades de estrellas y planetas, pero no conocemos la frecuencia de la vida. // *The galaxy contains enormous numbers of stars and planets, but we do not know the frequency of life.* | `vid=210` ignores `scene=19` & `doc_feat=0` | dense 100B+ star spiral density shimmer |
| `F1` | **sistemas antiguos** / *old systems* | `[TIER-D]` | Algunos mundos tuvieron miles de millones de años más que la Tierra para desarrollar historias propias. // *Some worlds have had billions more years than Earth to develop histories of their own.* | `vid=210` ignores `scene=19` & `doc_feat=1` | highlight older thick-disk red-gold stars predating Sun by 4 Gyr |
| `F2` | **expansión posible** / *possible expansion* | `[TIER-D]` | Incluso viajes lentos podrían extender presencia por una galaxia en escalas menores que su edad. // *Even slow travel could spread presence across a galaxy on scales smaller than its age.* | `vid=210` ignores `scene=19` & `doc_feat=2` | expanding spherical colonization wavefront rings rippling smoothly across spiral arms in <50 Myr |
| `F3` | **sin señales confirmadas** / *no confirmed signs* | `[TIER-C]` | Hasta ahora no existe una detección verificada de tecnología extraterrestre. // *So far there is no verified detection of extraterrestrial technology.* | `vid=210` ignores `scene=19` & `doc_feat=3` | total silence — dark muted voids extinguishing expectation waves |
| `F4` | **búsqueda limitada** / *limited search* | `[TIER-D]` | Solo hemos observado una fracción diminuta de lugares, frecuencias, épocas y tipos de señal. // *We have only observed a tiny fraction of places, frequencies, times and signal types.* | `vid=210` ignores `scene=19` & `doc_feat=4` | tiny glowing searchlight needle (solar 'cosmic haystack' cup vs galaxy ocean) |
| `F5` | **vidas tecnológicas breves** / *short technological lifetimes* | `[TIER-C]` | Una fase detectable puede durar poco comparada con la edad de un planeta. // *A detectable phase may be short-lived compared to the age of a planet.* | `vid=210` ignores `scene=19` & `doc_feat=5` | flickering brief millisecond spark stars representing short broadcast lifespans |
| `F6` | **silencio deliberado** / *deliberate silence* | `[TIER-C]` | Una civilización podría evitar emisiones o contacto, aunque no existe evidencia de esa estrategia. // *A civilization could avoid emissions or contact, although there is no evidence of such a strategy.* | `vid=210` ignores `scene=19` & `doc_feat=6` | cloaked dark nodes emitting zero leakage |
| `F7` | **vida rara** / *rare life* | `[TIER-C]` | Uno o varios pasos desde la química hasta la tecnología podrían ser extremadamente improbables. // *One or more steps from chemistry to technology could be extremely unlikely.* | `vid=210` ignores `scene=19` & `doc_feat=7` | sparse random lonely spark distribution |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP21-Z15-19)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 21 (21_cosmic_life) Zones Z15..Z19 (`false-positives`, `evolutionary-intelligence`, `alien-intelligence-forms`, `technosignatures`, `fermi-paradox`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z15->3115`, `Z16->3116`, `Z17->3117`, `Z18->3118`, `Z19->3119` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `astrobiologia`, `habitabilidad`, `agua`, `exoplaneta`, `biofirma`, `extremofilo`, `encelado` & `europa` (exist), `tránsito`, `espectro`, `biosfera`, `tecnofirma`, `fermi`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_civilization_horizon.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z15->3115`, `Z16->3116`, `Z17->3117`, `Z18->3118`, `Z19->3119`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z15 (vid=3115, false-positives): F0:abiotic oxygen; F1:volcanic methane; F2:atmospheric escape; F3:mineral reactions; F4:contamination; F5:instrumental noise; F6:incomplete models; F7:conflicting observations
     * Z16 (vid=3116, evolutionary-intelligence): F0:sensory processing; F1:learning; F2:use of tools; F3:social cooperation; F4:communication; F5:cumulative culture; F6:abstract reasoning; F7:environmental modification
     * Z17 (vid=3117, alien-intelligence-forms): F0:solitary intelligence; F1:swarm cognition; F2:oceanic mind; F3:distributed organism; F4:mechanical descendants; F5:collective mind; F6:slow cognition; F7:unknown senses
     * Z18 (vid=3118, technosignatures): F0:artificial radio signal; F1:optical pulses; F2:air pollutants; F3:night lighting; F4:industrial heat; F5:orbital structures; F6:engineered transits; F7:interstellar probes
     * Z19 (vid=3119, fermi-paradox): F0:many stars; F1:old systems; F2:possible expansion; F3:no confirmed signs; F4:limited search; F5:short technological lifetimes; F6:deliberate silence; F7:rare life
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp21-z20-24"></a>
### PKG-ACT2-EXP21-Z20-24: Cosmic Humility, Alien Societal Archetypes & Interstellar Ethics (`z=20..24`) — `Phase 1` · Priority: **Critical**

**Current State:** Zones `Z20..Z24` (`5` zones, `40` features) | Zone Tiers `0A / 0B / 2C / 3D` | Feature Tiers `0A / 0B / 13C / 27D` | Render path: [`doc_stages.rs#L75-L78`](../../crates/cosmos_render/src/doc_stages.rs#L75-L78) -> [`shaders/hero_doc_civilization_horizon.wgsl#L363-L388`](../../shaders/hero_doc_civilization_horizon.wgsl#L363-L388) — `z=12 -> vid=212`, all other `24` zones -> `vid=210` + twin `11/10` globes (`sc` ignored). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `40` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z20` | `great-filter` | El gran filtro / *The Great Filter* | **`3120`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z21` | `alien-civilizations` | Civilizaciones alienígenas / *Alien civilizations* | **`3121`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z22` | `interstellar-communication` | Comunicación interestelar / *interstellar communication* | **`3122`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z23` | `uncertain-contact` | Contacto incierto / *uncertain contact* | **`3123`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |
| `Z24` | `unknown-cosmic-life` | Vida cósmica desconocida / *Unknown cosmic life* | **`3124`** | `3820` world / `3821` ice moon / `3822` HZ / `3823` spec (`submode=0..3`) | Exp `21` compute `13` + `cs_field_grid` cues | `doc_feat 0..=7` (8 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 21 Z20` `great-filter` — El gran filtro / *The Great Filter* (`[TIER-C]` · Target `vid=3120`)
- **Metric (`raw_metric`):** "Cadena evolutiva × probabilidad de cada transición → supervivencia de civilizaciones detectables" (*Evolutionary chain × probability of each transition → survival of detectable civilizations*) · `scene=20` · `footer="¿QUÉ TRANSICIÓN ES REALMENTE RARA?"` · `evidence_class`: "Marco hipotético · ubicación desconocida" (*Hypothetical framework · unknown location*) · `range=[0.800, 0.840]`
- **Spoken Macro (ES // EN):** El gran filtro es un marco hipotético. No sabemos si existe un único filtro, muchos filtros o ninguno que pueda describirse de esta manera. // *The Great Filter is a hypothetical framework. We do not know whether there is one filter, many filters, or none that can be described this way.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Generic habitable rings (`210`) do not depict a multi-stage evolutionary probability bottleneck / Great Filter funnel across cosmic history (habitable worlds -> abiogenesis -> eukaryotes -> multicellularity -> intelligence -> tech civ -> survival -> interstellar expansion).
- **Zone 3D Procedural Blueprint:** 8-Gate Great Filter Probability Funnel (vid=3120): horizontal translucent luminous probability stream flowing through 8 vertical iris gates that narrow dramatically depending on doc_feat spotlight: 0 dense cluster of planets narrowing at Gate 1, 1 protocells pinching at Gate 2, 2 mitochondria barrier at Gate 3, 3 complex bodies at Gate 4, 4 symbolic brain at Gate 5, 5 planetary tech grid at Gate 6, 6 existential-risk hazard spikes ahead of present Gate 7, 7 diverging starship tracks beyond Gate 8.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **mundos habitables** / *habitable worlds* | `[TIER-D]` | Puede ser raro formar y conservar planetas con ambientes estables durante tiempo suficiente. // *It may be rare to form and preserve planets with stable environments for long enough.* | `vid=210` ignores `scene=20` & `doc_feat=0` | dense cluster of planets narrowing at Gate 1 |
| `F1` | **origen de la vida** / *origin of life* | `[TIER-D]` | La abiogénesis podría ser común, extremadamente rara o depender de condiciones todavía desconocidas. // *Abiogenesis could be common, extremely rare, or depend on as yet unknown conditions.* | `vid=210` ignores `scene=20` & `doc_feat=1` | protocells pinching at Gate 2 |
| `F2` | **células complejas** / *complex cells* | `[TIER-D]` | La transición a células eucariotas ocurrió una vez de forma conocida en la Tierra. // *As far as we know, the transition to eukaryotic cells occurred only once on Earth.* | `vid=210` ignores `scene=20` & `doc_feat=2` | mitochondria barrier at Gate 3 |
| `F3` | **multicelularidad** / *multicellularity* | `[TIER-D]` | Surgió varias veces, pero no siempre produjo organismos grandes, móviles o conscientes. // *It arose several times, but did not always produce large, mobile, or conscious organisms.* | `vid=210` ignores `scene=20` & `doc_feat=3` | complex bodies at Gate 4 |
| `F4` | **inteligencia** / *intelligence* | `[TIER-C]` | La cognición avanzada puede no conducir a lenguaje simbólico ni cultura acumulativa. // *Advanced cognition may not lead to symbolic language or cumulative culture.* | `vid=210` ignores `scene=20` & `doc_feat=4` | symbolic brain at Gate 5 |
| `F5` | **civilización tecnológica** / *technological civilization* | `[TIER-D]` | Construir tecnología planetaria requiere cooperación, energía, materiales y continuidad cultural. // *Building planetary technology requires cooperation, energy, materials and cultural continuity.* | `vid=210` ignores `scene=20` & `doc_feat=5` | planetary tech grid at Gate 6 |
| `F6` | **supervivencia prolongada** / *long-term survival* | `[TIER-D]` | Riesgos naturales y creados pueden limitar la duración de sociedades detectables. // *Natural and human-made risks can limit how long detectable societies endure.* | `vid=210` ignores `scene=20` & `doc_feat=6` | existential-risk hazard spikes ahead of present Gate 7 |
| `F7` | **expansión interestelar** / *interstellar expansion* | `[TIER-D]` | Una civilización puede sobrevivir sin expandirse o encontrar obstáculos físicos, sociales o éticos. // *A civilization may survive without expanding, or it may encounter physical, social, or ethical obstacles.* | `vid=210` ignores `scene=20` & `doc_feat=7` | diverging starship tracks beyond Gate 8 |

#### `Exp 21 Z21` `alien-civilizations` — Civilizaciones alienígenas / *Alien civilizations* (`[TIER-D]` · Target `vid=3121`)
- **Metric (`raw_metric`):** "Biología o máquinas + ambiente + historia + valores → organización civilizatoria posible" (*Biology or machines + environment + history + values → possible civilizational organization*) · `scene=21` · `footer="MISMA FÍSICA · HISTORIAS Y VALORES DISTINTOS"` · `evidence_class`: "Diversidad imaginada · conocimiento no reclamado" (*Imagined diversity · no claim of knowledge*) · `range=[0.840, 0.880]`
- **Spoken Macro (ES // EN):** Compartir las mismas leyes físicas no implica compartir nuestra biología, psicología, historia, tecnología ni forma de organizar la sociedad. // *Sharing the same physical laws does not imply sharing our biology, psychology, history, technology or way of organizing society.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Twin Earth/Europa + HZ rings (`210`) fails to showcase concrete diverse alien civilizational architectures: living ecological bio-tech worlds, postbiological machine habitats around cold stars, steady-state sustainable single-planet cultures, swarming orbital cylinder belts, migratory mobile world-ships, and millennia-hibernating civilizations.
- **Zone 3D Procedural Blueprint:** Archetypes of Extraterrestrial Societies (vid=3121)

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **civilización biológica** / *biological civilization* | `[TIER-D]` | Organismos podrían construir tecnología manteniendo cuerpos y ecosistemas de origen. // *Organisms could build technology while retaining their original bodies and ecosystems.* | `vid=210` ignores `scene=21` & `doc_feat=0` | bioluminescent living-tree city woven seamlessly into planet biosphere |
| `F1` | **sociedad postbiológica** / *postbiological society* | `[TIER-D]` | Mentes sintéticas podrían operar en ambientes extremos y escalas temporales distintas. // *Synthetic minds could operate in extreme environments and different time scales.* | `vid=210` ignores `scene=21` & `doc_feat=1` | crystalline cold silicon-compute matryoshka world orbiting white/red dwarf |
| `F2` | **cultura planetaria** / *planetary culture* | `[TIER-D]` | Una sociedad puede permanecer en un solo mundo y gestionar cuidadosamente sus límites. // *A society can remain in a single world and carefully manage its boundaries.* | `vid=210` ignores `scene=21` & `doc_feat=2` | ultra-stable harmonic single-world biosphere with zero orbital sprawl |
| `F3` | **hábitats artificiales** / *artificial habitats* | `[TIER-D]` | Comunidades podrían vivir en estructuras orbitales sin depender de superficies planetarias. // *Communities could live in orbital structures without depending on planetary surfaces.* | `vid=210` ignores `scene=21` & `doc_feat=3` | asteroid/orbital belt packed with glittering O'Neill habitat rings around empty star |
| `F4` | **inteligencia en red** / *networked intelligence* | `[TIER-D]` | La sociedad y la mente podrían superponerse en sistemas distribuidos de información. // *Society and mind could overlap in distributed information systems.* | `vid=210` ignores `scene=21` & `doc_feat=4` | planet-wide wireless planetary neural mesh aura |
| `F5` | **sistemas nómadas** / *nomadic systems* | `[TIER-D]` | Naves o hábitats móviles podrían viajar sin reclamar planetas como hogares permanentes. // *Ships or mobile habitats could travel without claiming planets as permanent homes.* | `vid=210` ignores `scene=21` & `doc_feat=5` | colossal glowing world-ship caravan migrating continuously through interstellar dark without host star |
| `F6` | **civilización dormida** / *sleeping civilization* | `[TIER-C]` | Una sociedad podría hibernar durante largos periodos y activarse solo ante energía o eventos concretos. // *A society could hibernate for long periods and activate only when energy is available or specific events occur.* | `vid=210` ignores `scene=21` & `doc_feat=6` | dormant cold geometric citadel briefly flaring into luminous pulse every 10,000 years |
| `F7` | **valores extraños** / *strange values* | `[TIER-C]` | Metas, emociones, cooperación y conflicto podrían carecer de equivalentes humanos directos. // *Goals, emotions, cooperation, and conflict may lack direct human equivalents.* | `vid=210` ignores `scene=21` & `doc_feat=7` | abstract alien geometric emblem clusters |

#### `Exp 21 Z22` `interstellar-communication` — Comunicación interestelar / *interstellar communication* (`[TIER-D]` · Target `vid=3122`)
- **Metric (`raw_metric`):** "Distancia + potencia + codificación + contexto → intercambio lento y parcial" (*Distance + power + encoding + context → slow and partial exchange*) · `scene=22` · `footer="PREGUNTA HOY · RESPUESTA EN OTRA GENERACIÓN"` · `evidence_class`: "Diálogo limitado por la velocidad de la luz" (*Dialogue limited by the speed of light*) · `range=[0.880, 0.920]`
- **Spoken Macro (ES // EN):** Una conversación entre estrellas podría requerir décadas, siglos o milenios entre una pregunta y su respuesta. // *A conversation between stars could require decades, centuries or millennia between a question and its answer.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Narrative moves cleanly from Fermi/civilizations to relativistic signal transmission (speed of light limit `c`, multi-decade/century travel times, narrow beam miss geometry, Prime mathematical framing, durable physical time-capsules, generational relay). Showing a tiny stationary habitable ring (`210`) completely misses the two-star interstellar light-cone geometry.
- **Zone 3D Procedural Blueprint:** Light-Cone & Interstellar Relay Bridge (vid=3122): two distant stars separated across wide stage

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **velocidad de la luz** / *speed of light* | `[TIER-D]` | Ningún mensaje conocido puede llegar antes que la luz, incluso si la transmisión es instantánea localmente. // *No known message can arrive before light, even if the transmission is locally instantaneous.* | `vid=210` ignores `scene=22` & `doc_feat=0` | spherical light-wave shell creeping measurably at fixed c toward target |
| `F1` | **años de viaje** / *years of travel time* | `[TIER-D]` | Un mensaje a una estrella cercana tarda años en llegar y otros tantos en recibir respuesta. // *A message to a nearby star takes years to arrive, and a reply takes just as many years to return.* | `vid=210` ignores `scene=22` & `doc_feat=1` | multi-tick decade clock bars ticking along transit beam |
| `F2` | **ventanas estrechas** / *narrow windows* | `[TIER-D]` | Una señal puede apuntar a un lugar, frecuencia y momento que no estemos observando. // *A signal can point to a place, frequency and time that we are not observing.* | `vid=210` ignores `scene=22` & `doc_feat=2` | sweeping razor-thin laser cone missing moving target star due to offset ephemeris |
| `F3` | **descifrado** / *decoding* | `[TIER-C]` | Reconocer estructura no garantiza comprender símbolos, referencias o intenciones. // *Recognizing structure does not guarantee understanding symbols, references or intentions.* | `vid=210` ignores `scene=22` & `doc_feat=3` | crisp prime-pulse binary raster waveform passing through dust |
| `F4` | **patrones matemáticos** / *mathematical patterns* | `[TIER-D]` | Regularidades simples pueden demostrar diseño sin proporcionar un idioma compartido. // *Simple regularities can demonstrate design without providing a shared language.* | `vid=210` ignores `scene=22` & `doc_feat=4` | concentric Prime cipher halo (`2,3,5,7,11`) |
| `F5` | **contexto cultural** / *cultural context* | `[TIER-C]` | Conceptos básicos para una especie pueden ser invisibles o irrelevantes para otra. // *Basic concepts for one species may be invisible or irrelevant for another.* | `vid=210` ignores `scene=22` & `doc_feat=5` | diverging translucent linguistic/cultural frame spheres |
| `F6` | **durabilidad** / *durability* | `[TIER-C]` | Mensajes físicos o repetidos pueden sobrevivir más que transmisiones breves y civilizaciones emisoras. // *Physical or repeated messages can survive longer than brief transmissions and sending civilizations.* | `vid=210` ignores `scene=22` & `doc_feat=6` | physical inscribed metallic Voyager/luminous crystal monument outlasting dimmed origin star |
| `F7` | **diálogo generacional** / *generational dialogue* | `[TIER-C]` | Quienes responden podrían no haber nacido cuando salió la pregunta original. // *Those who answer might not have been born when the original question was asked.* | `vid=210` ignores `scene=22` & `doc_feat=7` | multi-generation relay chain passing glowing torch packet |

#### `Exp 21 Z23` `uncertain-contact` — Contacto incierto / *uncertain contact* (`[TIER-C]` · Target `vid=3123`)
- **Metric (`raw_metric`):** "Señal + replicación + revisión + coordinación → evidencia pública y respuesta responsable" (*Signal + replication + review + coordination → public evidence and responsible response*) · `scene=23` · `footer="CONTACTO PUEDE SIGNIFICAR SOLO DATOS"` · `evidence_class`: "Verificación científica · consecuencias planetarias" (*Scientific verification · global consequences*) · `range=[0.920, 0.960]`
- **Spoken Macro (ES // EN):** El primer contacto más plausible podría consistir únicamente en datos: una señal, una composición atmosférica o una estructura difícil de explicar mediante procesos naturales. // *The most plausible first contact might consist solely of data: a signal, an atmospheric composition, or a structure difficult to explain by natural processes.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Conceptual bridge zone anticipating Exp 22 (`Uncertain contact`: ambiguous data signature, multi-country independent replication, transparency vs hype, misinformation, international governance, public response). Goldilocks habitable rings + green exoplanet do not visualize data verification or global Earth coordination.
- **Zone 3D Procedural Blueprint:** Multi-Observatory Verification Globe (vid=3123): translucent wireframe Earth crossed by global telescope baselines locking onto distant sky vector (previews Exp 22!)

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **señal ambigua** / *ambiguous signal* | `[TIER-D]` | La primera pista probablemente admitirá explicaciones rivales y requerirá nuevas observaciones. // *The first clue will probably support rival explanations and require new observations.* | `vid=210` ignores `scene=23` & `doc_feat=0` | noisy candidate waveform splitting into natural vs artificial question paths |
| `F1` | **verificación independiente** / *independent verification* | `[TIER-D]` | Equipos, países e instrumentos distintos deben confirmar que la señal existe y procede del lugar indicado. // *Different teams, countries, and instruments must confirm that the signal exists and comes from the reported location.* | `vid=210` ignores `scene=23` & `doc_feat=1` | 3+ continent radio dishes locking converging triangulation rays |
| `F2` | **transparencia científica** / *scientific transparency* | `[TIER-C]` | Compartir datos y métodos facilita corrección, aunque también puede amplificar interpretaciones prematuras. // *Sharing data and methods facilitates correction, although it can also amplify premature interpretations.* | `vid=210` ignores `scene=23` & `doc_feat=2` | open data-stream packets replicating around planet |
| `F3` | **desinformación** / *misinformation* | `[TIER-C]` | Rumores, falsificaciones y afirmaciones políticas podrían adelantarse a la evidencia. // *Rumors, falsifications and political claims could get ahead of the evidence.* | `vid=210` ignores `scene=23` & `doc_feat=3` | distorted glitch-echo misinformation ripples vs verified golden core stream |
| `F4` | **coordinación internacional** / *international coordination* | `[TIER-C]` | Ninguna institución representa automáticamente a toda la humanidad en una decisión de respuesta. // *No institution automatically represents all of humanity in a response decision.* | `vid=210` ignores `scene=23` & `doc_feat=4` | polycentric global summit nodes around Earth |
| `F5` | **decidir si responder** / *decide whether to respond* | `[TIER-C]` | Transmitir información puede tener consecuencias a muy largo plazo y no existe consenso universal. // *Transmitting information can have very long-term consequences and there is no universal consensus.* | `vid=210` ignores `scene=23` & `doc_feat=5` | bifurcating reply-vs-silence trajectory switch |
| `F6` | **impacto cultural** / *cultural impact* | `[TIER-C]` | La confirmación afectaría religiones, filosofías, identidades y expectativas de maneras imprevisibles. // *Confirmation would affect religions, philosophies, identities and expectations in unpredictable ways.* | `vid=210` ignores `scene=23` & `doc_feat=6` | shimmering worldwide cultural/philosophical spectrum aura |
| `F7` | **representación planetaria** / *planetary representation* | `[TIER-C]` | La pregunta de quién habla por la Tierra es política, ética y cultural, no solo técnica. // *The question of who speaks for the Earth is political, ethical and cultural, not just technical.* | `vid=210` ignores `scene=23` & `doc_feat=7` | multi-voice planetary harmony ring handing into Exp 22 |

#### `Exp 21 Z24` `unknown-cosmic-life` — Vida cósmica desconocida / *Unknown cosmic life* (`[TIER-D]` · Target `vid=3124`)
- **Metric (`raw_metric`):** "Observación limitada + conceptos terrestres + fenómenos extraños → clasificación provisional" (*Limited observation + terrestrial concepts + strange phenomena → provisional classification*) · `scene=24` · `footer="MÁS ALLÁ: EL CONTACTO CÓSMICO"` · `evidence_class`: "Frontera de categorías construidas desde un solo ejemplo" (*Category boundary built from a single example*) · `range=[0.960, 1.000]`
- **Spoken Macro (ES // EN):** Buscamos vida mediante conceptos construidos a partir de un solo ejemplo: la Tierra. Cuanto más diferente sea otra forma de vida, más difícil podría ser distinguirla de la geología, la química, la tecnología o incluso del ruido. // *We search for life through concepts built from a single example: the Earth. The more different another form of life is, the harder it might be to distinguish it from geology, chemistry, technology, or even noise.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=210 (`mode=0`: Goldilocks star + 2 emerald HZ rings + 1 orbiting dot [`orb_ang+=feat*0.45`]) + twin `kind=1` side spheres (`vid=11` Exoplanet right, `vid=10` Cracked Ice Moon left)
- **Audit Gap:** Every feature of the finale (`Unknown cosmic life`) contrasts Earth's single example against genuinely exotic life architectures: non-carbon silicon/boron chemistry, exotic cryogenic solvents (liquid methane/ammonia/supercritical CO2), buoyant cloud-layer aerial organisms, deep rock-sealed subterranean biospheres, self-evolving artificial life, bio-machine hybrids, ecosystem-level non-individual minds, and stubborn observational anomalies.
- **Zone 3D Procedural Blueprint:** Non-Terran Exotic Biospheres Frontier (vid=3124): rotating holographic prism of unearthly life worlds

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **química no carbonada** / *non-carbon-based chemistry* | `[TIER-D]` | Otros elementos podrían participar en sistemas complejos, aunque el carbono posee ventajas químicas extraordinarias. // *Other elements could participate in complex systems, although carbon has extraordinary chemical advantages.* | `vid=210` ignores `scene=24` & `doc_feat=0` | sparkling silicon-silicate polymer crystal chains alongside carbon baseline |
| `F1` | **solventes extraños** / *unusual solvents* | `[TIER-D]` | Líquidos distintos del agua podrían sostener reacciones, pero sus velocidades y estructuras serían diferentes. // *Liquids other than water could sustain reactions, but their speeds and structures would be different.* | `vid=210` ignores `scene=24` & `doc_feat=1` | swirling orange Titan/ammonia cryogenic ocean globe |
| `F2` | **organismos atmosféricos** / *atmospheric organisms* | `[TIER-D]` | La vida podría flotar en capas estables de una atmósfera sin tocar una superficie sólida. // *Life could float in stable layers of an atmosphere without touching a solid surface.* | `vid=210` ignores `scene=24` & `doc_feat=2` | gas-giant banded cloud deck inhabited by luminous buoyant plasma-jellyfish floaters without solid ground |
| `F3` | **biosferas subterráneas** / *underground biospheres* | `[TIER-D]` | Un mundo aparentemente muerto podría ocultar ecosistemas profundos desconectados de su superficie. // *A seemingly dead world could hide deep ecosystems disconnected from its surface.* | `vid=210` ignores `scene=24` & `doc_feat=3` | dark airless barren crust concealing vibrant bioluminescent subterranean cavern ocean below |
| `F4` | **vida artificial** / *artificial life* | `[TIER-D]` | Sistemas diseñados podrían reproducirse, evolucionar y alejarse de la biología que los creó. // *Engineered systems could reproduce, evolve, and move away from the biology that created them.* | `vid=210` ignores `scene=24` & `doc_feat=4` | metallic origami nano-organisms dividing and mutating independently |
| `F5` | **híbridos biológicos** / *biological hybrids* | `[TIER-D]` | Máquinas y organismos podrían formar unidades difíciles de separar en natural y artificial. // *Machines and organisms could form units that are difficult to separate into natural and artificial.* | `vid=210` ignores `scene=24` & `doc_feat=5` | intertwined biomechanical tendril-circuit chimeras |
| `F6` | **inteligencia sin individuos** / *intelligence without individuals* | `[TIER-D]` | La cognición podría pertenecer a un ecosistema, una red o un proceso sin cuerpos personales. // *Cognition could belong to an ecosystem, a network, or a process without personal bodies.* | `vid=210` ignores `scene=24` & `doc_feat=6` | planet-spanning fungal/mycelial electrical thought web without individuals |
| `F7` | **señal irresuelta** / *unresolved signal* | `[TIER-D]` | El descubrimiento puede comenzar no con una respuesta, sino con una anomalía que resiste explicaciones conocidas. // *Discovery may begin not with an answer, but with an anomaly that resists known explanations.* | `vid=210` ignores `scene=24` & `doc_feat=7` | pulsing stubborn unresolved anomaly crystal pointing onward into Exp 22 |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP21-Z20-24)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 21 (21_cosmic_life) Zones Z20..Z24 (`great-filter`, `alien-civilizations`, `interstellar-communication`, `uncertain-contact`, `unknown-cosmic-life`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z20->3120`, `Z21->3121`, `Z22->3122`, `Z23->3123`, `Z24->3124` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `astrobiologia`, `habitabilidad`, `agua`, `exoplaneta`, `biofirma`, `extremofilo`, `encelado` & `europa` (exist), `tránsito`, `espectro`, `biosfera`, `tecnofirma`, `fermi`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_civilization_horizon.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z20->3120`, `Z21->3121`, `Z22->3122`, `Z23->3123`, `Z24->3124`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z20 (vid=3120, great-filter): F0:habitable worlds; F1:origin of life; F2:complex cells; F3:multicellularity; F4:intelligence; F5:technological civilization; F6:long-term survival; F7:interstellar expansion
     * Z21 (vid=3121, alien-civilizations): F0:biological civilization; F1:postbiological society; F2:planetary culture; F3:artificial habitats; F4:networked intelligence; F5:nomadic systems; F6:sleeping civilization; F7:strange values
     * Z22 (vid=3122, interstellar-communication): F0:speed of light; F1:years of travel time; F2:narrow windows; F3:decoding; F4:mathematical patterns; F5:cultural context; F6:durability; F7:generational dialogue
     * Z23 (vid=3123, uncertain-contact): F0:ambiguous signal; F1:independent verification; F2:scientific transparency; F3:misinformation; F4:international coordination; F5:decide whether to respond; F6:cultural impact; F7:planetary representation
     * Z24 (vid=3124, unknown-cosmic-life): F0:non-carbon-based chemistry; F1:unusual solvents; F2:atmospheric organisms; F3:underground biospheres; F4:artificial life; F5:biological hybrids; F6:intelligence without individuals; F7:unresolved signal
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

---

### Exp 22: `22_cosmic_contact` — Contacto Cósmico / Cosmic Contact (`28` Zones, `168` Features, `0` Landmarks)

> *Espectro del Contacto Cósmico: de la Primera Señal al Diálogo entre Civilizaciones* — Spectrum of Cosmic Contact: from the First Signal to Interstellar Dialogue

- **Odyssey Position (`odyssey.rs#L29-L32`):** Act 2, Position **3 / 8** (Chapter `act2_exp22`)
- **Spatial & Scale Band:** `SectorId::ORION_SPUR` (`anchor_local=(0.40, 0.05, -0.30)`) · `u ∈ [16.0, 20.8]` · `focus_scale_exp = 18.6` (`topology.rs:353`)
- **Compute & Far Shell:** `mode = 13` (radial cosmological redshift stream, `compute_universe.wgsl:319-330`) · `galaxy_plane = 1.0`, `star_density = 1.0` Orion Spur background (`universe_scene.rs:350-370`)
- **Current Render Dispatch:** `doc_stage_variant_id(22)` (`doc_stages.rs:79-82`) routes `z=12 -> vid=221` (5 Prime rings + pulsar cross rays) and ALL 27 other zones -> `vid=220` (`mode=0`: Parabolic SETI Dish + permanent upward laser beacon, varying ONLY yaw `f32(zone)*0.25` and `odd_zone` tint/phase); **5 baked `scene` collisions** (`0`, `8`, `13`, `14`, `15`); **false header claim**: `hero_doc_civilization_horizon.wgsl:6` claims `doc_feat` support, yet `feat` is NEVER read in `exp_id == 22u` (`L389-429`).
- **Granular Audit Tally:** Zones `0A / 5B / 15C / 8D` (`28`) · Features `0A / 11B / 88C / 69D` (`168`)
- **`DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`) Needed Additions:** `seti` & `rosetta` & `primo` (exist), plus `radar`, `laser`, `radio`, `descifrado`, `silencio`, `meti`, `tratado`, `relativista`, `espejo`, `archivo`

#### Baked `scene` Collision Pairs in Exp `22` (`doc_stages.rs` MUST key on `zone_index` or split `scene` in P0)

| Colliding `scene` | First Zone (`Z..`) | Second Zone (`Z..`) | Shared `footer` | Required Visual Split |
|---:|---|---|---|---|
| `0` | `Z00` `unexplained-signal` | `Z01` `first-anomaly` | `ANOMALÍA ≠ CONCLUSIÓN` | `vid=3200` vs `vid=3201` distinct stages |
| `8` | `Z09` `international-coordination` | `Z10` `planetary-reaction` | `NINGUNA VOZ ÚNICA HABLA POR TODO EL PLANETA` | `vid=3209` vs `vid=3210` distinct stages |
| `13` | `Z15` `cultural-context` | `Z16` `message-received` | `MISMA FÍSICA · HISTORIAS MENTALES DISTINTAS` | `vid=3215` vs `vid=3216` distinct stages |
| `14` | `Z17` `decision-to-respond` | `Z18` `transmission-risks` | `RESPONDER, ESPERAR O CALLAR CAMBIA LA RELACIÓN` | `vid=3217` vs `vid=3218` distinct stages |
| `15` | `Z19` `light-speed-limit` | `Z20` `interstellar-wait` | `EL MENSAJE DESCRIBE EL PASADO DEL EMISOR` | `vid=3219` vs `vid=3220` distinct stages |

<a id="pkg-act2-exp22-z00-04"></a>
### PKG-ACT2-EXP22-Z00-04: SETI Anomaly Triage, Noise Verification & Signal Confirmation (`z=00..04`, `scene=0` split) — `Phase 1` · Priority: **Critical**

**Current State:** Zones `Z00..Z04` (`5` zones, `30` features) | Zone Tiers `0A / 3B / 0C / 2D` | Feature Tiers `0A / 5B / 5C / 20D` | Render path: [`doc_stages.rs#L79-L82`](../../crates/cosmos_render/src/doc_stages.rs#L79-L82) -> [`shaders/hero_doc_civilization_horizon.wgsl#L389-L429`](../../shaders/hero_doc_civilization_horizon.wgsl#L389-L429) — `z=12 -> vid=221`, all other `27` zones -> `vid=220` SETI dish (`5` shared `scene` pairs). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `30` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z00` | `unexplained-signal` | Una señal sin explicación definitiva / *A signal without a definitive explanation* | **`3200`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z01` | `first-anomaly` | La primera anomalía / *The first anomaly* | **`3201`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z02` | `instrumental-detection` | Detección instrumental / *Instrumental detection* | **`3202`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z03` | `interference-rejected` | Interferencia descartada / *Interference ruled out* | **`3203`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z04` | `independent-verification` | Verificación independiente / *Independent verification* | **`3204`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 22 Z00` `unexplained-signal` — Una señal sin explicación definitiva / *A signal without a definitive explanation* (`[TIER-B]` · Target `vid=3200`)
- **Metric (`raw_metric`):** "Señal observada + causas posibles + incertidumbre explícita → pregunta verificable" (*Observed signal + possible causes + explicit uncertainty → verifiable question*) · `scene=0` · `footer="ANOMALÍA ≠ CONCLUSIÓN"` · `evidence_class`: "Punto de partida · anomalía heredada, conclusión pendiente" (*Starting point · existing anomaly, conclusion pending*) · `range=[0.000, 0.036]`
- **Spoken Macro (ES // EN):** Una señal extraña no es todavía evidencia de inteligencia: es un fenómeno que exige mejores datos y explicaciones alternativas. // *A strange signal is not yet evidence of intelligence: it is a phenomenon that demands better data and alternative explanations.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=0.00 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Shares scene=0 with Z01 and runs vid=220 (`mode=0` Parabolic SETI dish + upward laser pulse; `doc_feat` & `doc_scene` unread). Parabolic dish + laser pulse broadly matches receiving an anomalous RF/optical clue (F00, F01), though bizarre transit curves (F02), atmospheric CFCs (F03), anomalous trajectories (F04), and doc_feat switching are completely absent.
- **Zone 3D Procedural Blueprint:** Upgrade vid=220 SETI Anomaly Receiver & Split scene=0: keep SETI dish for Z00 and animate incoming sky candidate per doc_feat: 0 razor-thin radio carrier beam plunging into feed horn, 1 intense brief optical laser pulse train striking auxiliary mirror, 2 background star blinking with sharp rectangular megastructure transit silhouette, 3 exoplanet limb glowing with exotic synthetic molecule bands, 4 hyperbola-curved interstellar artifact tumbling past dish horizon, 5 crisp glowing prime-number pulse train.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **emisión estrecha** / *narrow emission* | `[TIER-B]` | Una emisión concentrada en una banda pequeña puede llamar la atención, pero también puede surgir de interferencia o de fenómenos naturales poco comunes. // *An emission concentrated in a small band may attract attention, but it may also arise from interference or unusual natural phenomena.* | `vid=220` ignores `scene=0` & `doc_feat=0` | razor-thin radio carrier beam plunging into feed horn |
| `F1` | **pulso óptico** / *optical pulse* | `[TIER-B]` | Un destello breve y ordenado podría ser instrumental, astronómico o tecnológico; sin repetición no permite distinguirlos. // *A brief, orderly flash could be instrumental, astronomical, or technological; Without repetition it does not allow us to distinguish them.* | `vid=220` ignores `scene=0` & `doc_feat=1` | intense brief optical laser pulse train striking auxiliary mirror |
| `F2` | **tránsito extraño** / *unusual transit* | `[TIER-D]` | Una caída irregular de luz puede sugerir estructuras inusuales, aunque polvo, lunas, anillos o errores de modelado siguen siendo explicaciones posibles. // *An irregular dip in brightness may suggest unusual structures, although dust, moons, rings, or modeling errors remain possible explanations.* | `vid=220` ignores `scene=0` & `doc_feat=2` | background star blinking with sharp rectangular megastructure transit silhouette |
| `F3` | **química industrial** / *industrial chemistry* | `[TIER-D]` | Una molécula atmosférica difícil de producir naturalmente sería interesante, pero habría que conocer por completo la química y la geología del planeta. // *An atmospheric molecule that is difficult to produce naturally would be interesting, but the chemistry and geology of the planet would have to be fully understood.* | `vid=220` ignores `scene=0` & `doc_feat=3` | exoplanet limb glowing with exotic synthetic molecule bands |
| `F4` | **objeto inesperado** / *unexpected object* | `[TIER-D]` | Un movimiento difícil de explicar puede reflejar fuerzas mal estimadas, pérdida de material, sesgos de observación o propulsión artificial. // *Difficult-to-explain motion may reflect misestimated forces, material loss, observation biases, or artificial propulsion.* | `vid=220` ignores `scene=0` & `doc_feat=4` | hyperbola-curved interstellar artifact tumbling past dish horizon |
| `F5` | **estructura repetida** / *repeated structure* | `[TIER-C]` | La repetición matemática aumenta el interés, pero la naturaleza también genera ritmos, resonancias y regularidades sorprendentes. // *Mathematical repetition increases interest, but nature also generates surprising rhythms, resonances and regularities.* | `vid=220` ignores `scene=0` & `doc_feat=5` | crisp glowing prime-number pulse train |

#### `Exp 22 Z01` `first-anomaly` — La primera anomalía / *The first anomaly* (`[TIER-B]` · Target `vid=3201`)
- **Metric (`raw_metric`):** "Datos ordinarios − modelo esperado → residuo que merece investigación" (*Ordinary data − expected model → residual that deserves investigation*) · `scene=0` · `footer="ANOMALÍA ≠ CONCLUSIÓN"` · `evidence_class`: "Observación inicial · lo extraordinario empieza como discrepancia" (*Initial observation · the extraordinary begins as a discrepancy*) · `range=[0.036, 0.071]`
- **Spoken Macro (ES // EN):** La primera tarea no es nombrar al emisor, sino describir exactamente qué parte de los datos no encaja. // *The first task is not to name the sender, but to describe exactly which part of the data does not fit.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=0.25 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** COLLIDES on scene=0 with Z01; WGSL only shifts dish yaw by `f32(zone)*0.25` and flips `odd_zone` pulse timing (`1.8`) + color mix (`0.60`), while leaving `doc_feat` 100% unread! Still anchors at the SETI dish receiver (`F01 duration`, `F04 repetition`), yet lacks Z01's defining 5-sigma noise residual, frequency waterfall spectrogram, and Stokes polarization helix.
- **Zone 3D Procedural Blueprint:** Statistical Residual & Waterfall Holo-Screen over Dish (vid=3201): render semi-transparent holographic dynamic spectrum floating directly above dish aperture

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **residuo estadístico** / *statistical residual* | `[TIER-D]` | Una desviación solo importa cuando supera el ruido esperado y no depende de elegir después el análisis que mejor la destaca. // *A deviation only matters when it exceeds the expected noise and does not depend on later choosing the analysis that best highlights it.* | `vid=220` ignores `scene=0` & `doc_feat=0` | smooth Gaussian white-noise carpet with single bright 6-sigma anomaly peak popping above threshold plane |
| `F1` | **duración** / *duration* | `[TIER-B]` | Milisegundos, horas o años implican mecanismos distintos y determinan qué instrumentos pueden confirmar el fenómeno. // *Milliseconds, hours or years imply different mechanisms and determine which instruments can confirm the phenomenon.* | `vid=220` ignores `scene=0` & `doc_feat=1` | ms-to-year logarithmic pulse-width bar |
| `F2` | **frecuencia** / *frequency* | `[TIER-D]` | La distribución energética puede revelar propagación, fuente, modulación o contaminación terrestre. // *Energy distribution can reveal propagation, source, modulation or terrestrial contamination.* | `vid=220` ignores `scene=0` & `doc_feat=2` | swirling frequency-time waterfall cascade |
| `F3` | **polarización** / *polarization* | `[TIER-D]` | La orientación de la señal puede aportar información sobre campos magnéticos, geometría de emisión o instrumentación. // *The orientation of the signal can provide information about magnetic fields, emission geometry or instrumentation.* | `vid=220` ignores `scene=0` & `doc_feat=3` | corkscrew circular/linear Stokes polarization helix descending beam |
| `F4` | **repetición** / *repetition* | `[TIER-B]` | Una señal que vuelve permite predecirla, apuntar otros observatorios y separar un evento real de una fluctuación. // *A returning signal makes it possible to predict it, point other observatories and separate a real event from a fluctuation.* | `vid=220` ignores `scene=0` & `doc_feat=4` | periodic return ping lighting verification rings |
| `F5` | **ausencia posterior** / *subsequent absence* | `[TIER-C]` | No volver a detectar algo no demuestra que fuera falso; limita cuánto puede aprenderse y reduce la confianza. // *Not detecting something again does not prove it was false; it limits how much can be learned and reduces confidence.* | `vid=220` ignores `scene=0` & `doc_feat=5` | fading ghost echo dissolving back into gray thermal static |

#### `Exp 22 Z02` `instrumental-detection` — Detección instrumental / *Instrumental detection* (`[TIER-B]` · Target `vid=3202`)
- **Metric (`raw_metric`):** "Fuente → propagación → sensor → calibración → datos sin procesar → análisis" (*Source → propagation → sensor → calibration → raw data → analysis*) · `scene=1` · `footer="MEDIR TAMBIÉN SIGNIFICA AUDITAR EL INSTRUMENTO"` · `evidence_class`: "Medición · la cadena completa debe permanecer auditable" (*Measurement · the entire chain must remain auditable*) · `range=[0.071, 0.107]`
- **Spoken Macro (ES // EN):** Cada etapa puede introducir señales falsas; por eso conservar los datos originales y la configuración es esencial. // *Each stage can introduce false signals, so preserving the original data and configuration is essential.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=0.50 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Routed to vid=220 (`mode=0` lone parabolic dish + feed horn). Literally depicts the primary radio telescope instrument discussed (`calibration` F00 on dish/feed), yet `doc_feat` is ignored so Airy beam angular resolution (`F01`), atomic-clock wavefront synchronization (`F02`), raw IQ buffer (`F03`), and digital DSP cleanup chain (`F04`) are completely missing.
- **Zone 3D Procedural Blueprint:** Auditable Instrument Signal-Chain Cutaway (vid=3202): render dish in translucent wireframe cutaway revealing internal hardware signal flow down to base rack

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **calibración** / *calibration* | `[TIER-B]` | El instrumento debe observar referencias conocidas para comprobar sensibilidad, frecuencia, tiempo y respuesta direccional. // *The instrument must observe known references to verify sensitivity, frequency, time and directional response.* | `vid=220` ignores `scene=1` & `doc_feat=0` | reference sky standard-candle star calibrating green gain cone |
| `F1` | **resolución** / *resolution* | `[TIER-D]` | Una resolución insuficiente puede fusionar fuentes, ocultar variaciones o fabricar patrones mediante muestreo. // *Insufficient resolution can merge sources, hide variations, or fabricate patterns through sampling.* | `vid=220` ignores `scene=1` & `doc_feat=1` | wide blurry single-dish Airy PSF lobe sharpening cleanly to separate double source |
| `F2` | **marca temporal** / *timestamp* | `[TIER-D]` | Relojes precisos permiten comparar observatorios y verificar si la señal llegó con los retrasos físicos esperados. // *Accurate clocks make it possible to compare observatories and verify whether the signal arrived with the expected physical delays.* | `vid=220` ignores `scene=1` & `doc_feat=2` | concentric cesium/hydrogen-maser clock pulse rings stamping wavefront |
| `F3` | **datos originales** / *original data* | `[TIER-D]` | Los datos sin filtrar permiten repetir análisis futuros y detectar artefactos introducidos por el procesamiento. // *Unfiltered data allow future analyses to be repeated and artifacts introduced by processing to be detected.* | `vid=220` ignores `scene=1` & `doc_feat=3` | dense raw complex baseband I/Q point cloud stream exiting feed horn |
| `F4` | **cadena de software** / *software chain* | `[TIER-D]` | Conversión, compresión, limpieza y clasificación automática deben registrarse para localizar errores reproducibles. // *Conversion, compression, cleaning, and automatic sorting must be recorded to locate reproducible errors.* | `vid=220` ignores `scene=1` & `doc_feat=4` | 4-stage glowing holographic DSP blocks (ADC -> FFT -> RFI filter -> ML flag) |
| `F5` | **repetibilidad** / *repeatability* | `[TIER-C]` | Un procedimiento que nadie puede repetir no sostiene una afirmación extraordinaria, aunque su resultado parezca convincente. // *A procedure that no one can repeat does not support an extraordinary claim, even if its result seems convincing.* | `vid=220` ignores `scene=1` & `doc_feat=5` | deterministic replay mirror beam verifying pipeline output |

#### `Exp 22 Z03` `interference-rejected` — Interferencia descartada / *Interference ruled out* (`[TIER-D]` · Target `vid=3203`)
- **Metric (`raw_metric`):** "Catálogos + geometría + pruebas locales + observaciones de control → interferencia menos probable" (*Catalogs + geometry + local tests + control observations → less likely interference*) · `scene=2` · `footer="CERCA, REFLEJADO, DIGITAL O CELESTE"` · `evidence_class`: "Control de contaminación · terrestre no significa evidente" (*Interference control · terrestrial does not mean obvious*) · `range=[0.107, 0.143]`
- **Spoken Macro (ES // EN):** Descartar interferencia requiere demostrar que la señal no sigue transmisores, equipos, software ni trayectorias cercanas. // *Ruling out interference requires demonstrating that the signal does not follow nearby transmitters, equipment, software, or paths.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=0.75 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Plain skyward-beaming SETI dish (`220`) fails to depict every terrestrial/orbital interferer Z03 explicitly rejects: LEO satellite constellations crossing telescope sidelobes (`F00`), aircraft radar sweeps (`F01`), local electronics harmonics (`F02`), Moon/meteor radio reflections (`F03`), software overflow/aliasing artifacts (`F04`), and off-source ON-OFF nodding calibration (`F05`).
- **Zone 3D Procedural Blueprint:** RFI Discrimination & Sidelobe Field Arena (vid=3203): place SETI dish inside wide polar antenna radiation pattern showing central main beam + concentric sidelobe petals

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **satélites** / *satellites* | `[TIER-D]` | Una señal puede entrar por lóbulos laterales del telescopio y parecer celeste hasta comparar su dirección con órbitas conocidas. // *A signal can enter the side lobes of the telescope and appear celestial until its direction is compared to known orbits.* | `vid=220` ignores `scene=2` & `doc_feat=0` | bright Starlink-style satellite arcing rapidly through outer sidelobe with red RFI flash |
| `F1` | **aeronaves** / *aircraft* | `[TIER-D]` | Radar, comunicaciones y reflexiones móviles pueden producir eventos breves que cambian con la geometría local. // *Radar, communications, and moving reflections can produce brief events that change with local geometry.* | `vid=220` ignores `scene=2` & `doc_feat=1` | sweeping radar plane/transponder ping flickering sidelobe |
| `F2` | **electrónica cercana** / *nearby electronics* | `[TIER-D]` | Relojes digitales, motores, redes y fuentes de alimentación pueden contaminar bandas sensibles. // *Digital clocks, motors, networks and power supplies can contaminate sensitive bands.* | `vid=220` ignores `scene=2` & `doc_feat=2` | sparkling amber ground-rack EMI wave leaking from control shed |
| `F3` | **reflejos** / *reflections* | `[TIER-D]` | La Luna, meteoros o capas atmosféricas pueden redirigir transmisiones terrestres hacia un observatorio. // *The Moon, meteors or atmospheric layers can redirect ground transmissions to an observatory.* | `vid=220` ignores `scene=2` & `doc_feat=3` | ground radar pulse bouncing off Moon crater/meteor ion trail back into dish |
| `F4` | **artefacto digital** / *digital artifact* | `[TIER-D]` | Desbordamientos, cuantización, compresión o errores de memoria pueden crear periodicidad sin señal física externa. // *Overflows, quantization, compression, or memory errors can create periodicity without an external physical signal.* | `vid=220` ignores `scene=2` & `doc_feat=4` | jagged sawtooth bit-overflow FFT artifact inside local terminal |
| `F5` | **observación fuera de fuente** / *off-source observation* | `[TIER-D]` | Apuntar a una región vecina ayuda a comprobar si el patrón permanece ligado al cielo o al instrumento. // *Aiming at a neighboring region helps check whether the pattern remains tied to the sky or the instrument.* | `vid=220` ignores `scene=2` & `doc_feat=5` | animated dish gimbal physically nodding ON-source -> OFF-source confirming sky signal vanishes off-target |

#### `Exp 22 Z04` `independent-verification` — Verificación independiente / *Independent verification* (`[TIER-D]` · Target `vid=3204`)
- **Metric (`raw_metric`):** "Equipos distintos + métodos distintos + predicción común → evidencia independiente" (*Different equipment + different methods + common prediction → independent evidence*) · `scene=3` · `footer="ERRORES DISTINTOS · RESULTADO COMPATIBLE"` · `evidence_class`: "Reproducción · una observación aislada no basta" (*Replication · an isolated observation is not enough*) · `range=[0.143, 0.179]`
- **Spoken Macro (ES // EN):** La confirmación es más fuerte cuando observatorios que no comparten los mismos errores obtienen resultados compatibles. // *Confirmation is stronger when observatories that do not share the same errors obtain compatible results.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=1.00 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Lone yawed SETI dish (`220`) cannot show independent replication across continents, opposite planetary hemispheres, or multi-wavelength observatories (`F00..F02`), blind test injections (`F03`), or historical plate archives (`F04`).
- **Zone 3D Procedural Blueprint:** Global Multi-Band Independent Replication Globe (vid=3204): replace single dish with translucent rotating Earth studded with synchronized observatories

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **otro telescopio** / *another telescope* | `[TIER-D]` | Un instrumento independiente reduce la posibilidad de que el hallazgo pertenezca a un receptor o ubicación concreta. // *An independent instrument reduces the chance that the finding is specific to one receiver or location.* | `vid=220` ignores `scene=3` & `doc_feat=0` | two distant green radio dishes on opposite horizons locking onto identical star coordinate |
| `F1` | **otra longitud de onda** / *another wavelength* | `[TIER-D]` | Una fuente física puede dejar huellas relacionadas en radio, óptico, infrarrojo, rayos X u otras bandas. // *A physical source may leave related traces in radio, optical, infrared, x-ray, or other bands.* | `vid=220` ignores `scene=3` & `doc_feat=1` | layered radio + optical + IR + X-ray spectral rainbow slices converging on single celestial point |
| `F2` | **otro hemisferio** / *other hemisphere* | `[TIER-D]` | Una ubicación distante cambia el entorno de interferencias y ofrece una geometría de observación diferente. // *A distant location changes the interference environment and offers a different observing geometry.* | `vid=220` ignores `scene=3` & `doc_feat=2` | north & south hemisphere dishes contrasting local noise backgrounds |
| `F3` | **análisis ciego** / *blind analysis* | `[TIER-C]` | Ocultar qué segmento contiene la posible señal ayuda a reducir ajustes inconscientes del método. // *Hiding which segment contains the possible signal helps reduce unconscious adjustments to the method.* | `vid=220` ignores `scene=3` & `doc_feat=3` | golden envelope 'blind injection' seal hidden beside genuine sky spikes |
| `F4` | **archivo histórico** / *historical archive* | `[TIER-D]` | Datos antiguos pueden mostrar que el fenómeno existía antes de que la hipótesis guiara la búsqueda. // *Old data may show that the phenomenon existed before the hypothesis guided the search.* | `vid=220` ignores `scene=3` & `doc_feat=4` | dusty glowing stacked glass photographic/archival survey plates showing same ancient coordinate |
| `F5` | **resultado nulo** / *null result* | `[TIER-C]` | No detectar la señal en condiciones adecuadas también informa sobre su duración, dirección o variabilidad. // *Not detecting the signal under appropriate conditions also informs about its duration, direction or variability.* | `vid=220` ignores `scene=3` & `doc_feat=5` | informative empty null-detection contour narrowing sky box |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP22-Z00-04)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 22 (22_cosmic_contact) Zones Z00..Z04 (`unexplained-signal`, `first-anomaly`, `instrumental-detection`, `interference-rejected`, `independent-verification`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z00->3200`, `Z01->3201`, `Z02->3202`, `Z03->3203`, `Z04->3204` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Disambiguate colliding baked scene indices in Z00..Z27 by keying directly on doc.active_zone_index -> vid=3200..3227 and spawning companion stages 3830..3835 where dual actors/wavefronts apply.
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `seti` & `rosetta` & `primo` (exist), plus `radar`, `laser`, `radio`, `descifrado`, `silencio`, `meti`, `tratado`, `relativista`, `espejo`, `archivo`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_civilization_horizon.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z00->3200`, `Z01->3201`, `Z02->3202`, `Z03->3203`, `Z04->3204`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z00 (vid=3200, unexplained-signal): F0:narrow emission; F1:optical pulse; F2:unusual transit; F3:industrial chemistry; F4:unexpected object; F5:repeated structure
     * Z01 (vid=3201, first-anomaly): F0:statistical residual; F1:duration; F2:frequency; F3:polarization; F4:repetition; F5:subsequent absence
     * Z02 (vid=3202, instrumental-detection): F0:calibration; F1:resolution; F2:timestamp; F3:original data; F4:software chain; F5:repeatability
     * Z03 (vid=3203, interference-rejected): F0:satellites; F1:aircraft; F2:nearby electronics; F3:reflections; F4:digital artifact; F5:off-source observation
     * Z04 (vid=3204, independent-verification): F0:another telescope; F1:another wavelength; F2:other hemisphere; F3:blind analysis; F4:historical archive; F5:null result
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp22-z05-09"></a>
### PKG-ACT2-EXP22-Z05-09: Passive vs Active Searches & Global Coordination Forums (`z=05..09`, `scene=8` part 1) — `Phase 1` · Priority: **Critical**

**Current State:** Zones `Z05..Z09` (`5` zones, `30` features) | Zone Tiers `0A / 0B / 3C / 2D` | Feature Tiers `0A / 1B / 17C / 12D` | Render path: [`doc_stages.rs#L79-L82`](../../crates/cosmos_render/src/doc_stages.rs#L79-L82) -> [`shaders/hero_doc_civilization_horizon.wgsl#L389-L429`](../../shaders/hero_doc_civilization_horizon.wgsl#L389-L429) — `z=12 -> vid=221`, all other `27` zones -> `vid=220` SETI dish (`5` shared `scene` pairs). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `30` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z05` | `distant-origin` | Origen distante / *Distant origin* | **`3205`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z06` | `artificial-pattern` | Patrón artificial / *artificial pattern* | **`3206`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z07` | `accumulated-evidence` | Evidencia acumulada / *Accumulated evidence* | **`3207`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z08` | `scientific-announcement` | Anuncio científico / *Scientific announcement* | **`3208`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z09` | `international-coordination` | Coordinación internacional / *International coordination* | **`3209`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 22 Z05` `distant-origin` — Origen distante / *Distant origin* (`[TIER-D]` · Target `vid=3205`)
- **Metric (`raw_metric`):** "Dirección + deriva + dispersión + paralaje + repetición → región de origen limitada" (*Direction + drift + dispersion + parallax + repetition → limited source region*) · `scene=4` · `footer="DISTANTE NO SIGNIFICA ARTIFICIAL"` · `evidence_class`: "Localización física · procedencia celeste cada vez más probable" (*Physical location · increasingly probable celestial origin*) · `range=[0.179, 0.214]`
- **Spoken Macro (ES // EN):** Demostrar que una señal viene de lejos no demuestra que sea artificial, pero elimina muchas causas locales. // *Proving that a signal comes from afar does not prove that it is artificial, but it eliminates many local causes.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=1.25 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Narrates concrete astrophysical propagation & astrometry diagnostics proving cosmic distance: sidereal tracking against rotating Earth (`F00`), continuous Doppler frequency drift curve from planetary spin/orbit (`F01`), interstellar plasma dispersion delay $\Delta t \propto f^{-2}$ (`F02`), multi-dish VLBI parallax triangulation (`F03`), host star identification (`F04`), and moving transmitter orbit (`F05`). None exist in `220`.
- **Zone 3D Procedural Blueprint:** Astrophysical Astrometry & ISM Dispersion Stage (vid=3205): spinning Earth base linked by long sky beam to distant star system

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **posición celeste** / *celestial position* | `[TIER-D]` | La señal debe seguir la misma coordenada mientras gira la Tierra y cambian la orientación y el horizonte. // *The signal must follow the same coordinate as the Earth rotates and the orientation and horizon change.* | `vid=220` ignores `scene=4` & `doc_feat=0` | beam staying locked onto RA/Dec star while rotating terrestrial ground sweeps beneath |
| `F1` | **deriva Doppler** / *Doppler drift* | `[TIER-D]` | El movimiento relativo entre fuente, planeta y observatorio desplaza la frecuencia de una forma físicamente modelable. // *The relative motion between source, planet and observatory shifts the frequency in a physically modelable way.* | `vid=220` ignores `scene=4` & `doc_feat=1` | diagonal S-curve Doppler drift line sweeping cleanly across frequency-time panel |
| `F2` | **dispersión** / *dispersion* | `[TIER-D]` | El medio interestelar retrasa frecuencias distintas; ese retraso puede estimar cuánto plasma atravesó la señal. // *The interstellar medium delays different frequencies; that delay can estimate how much plasma the signal passed through.* | `vid=220` ignores `scene=4` & `doc_feat=2` | high-frequency violet wave crest arriving cleanly ahead of delayed low-frequency red crest through glowing ionized ISM plasma cloud |
| `F3` | **paralaje** / *parallax* | `[TIER-D]` | Observatorios separados pueden medir pequeñas diferencias angulares y distinguir una fuente cercana de una distante. // *Separate observatories can measure small angular differences and distinguish a nearby source from a distant one.* | `vid=220` ignores `scene=4` & `doc_feat=3` | wide baseline triangle spanning opposite sides of Earth orbit measuring tiny arcsecond parallax apex |
| `F4` | **estrella asociada** / *associated star* | `[TIER-D]` | Vincular la señal a un sistema estelar requiere precisión suficiente para excluir fuentes de fondo. // *Linking the signal to a star system requires sufficient precision to exclude background sources.* | `vid=220` ignores `scene=4` & `doc_feat=4` | magnified host star orbited by candidate habitable planet |
| `F5` | **fuente móvil** / *moving source* | `[TIER-D]` | Una sonda o transmisor orbital podría desplazarse, por lo que la localización puede cambiar de forma coherente. // *An orbital probe or transmitter could move, so the location can change coherently.* | `vid=220` ignores `scene=4` & `doc_feat=5` | wobbling target orbit modulating phase coherently |

#### `Exp 22 Z06` `artificial-pattern` — Patrón artificial / *artificial pattern* (`[TIER-D]` · Target `vid=3206`)
- **Metric (`raw_metric`):** "Modulación + estructura improbable + corrección de errores + repetición → diseño más plausible" (*Modulation + improbable structure + error correction + repetition → more plausible design*) · `scene=5` · `footer="ORDEN FUNCIONAL · ORIGEN TODAVÍA ABIERTO"` · `evidence_class`: "Hipótesis tecnológica · orden no equivale automáticamente a intención" (*Technological hypothesis · order does not automatically equate to intention*) · `range=[0.214, 0.250]`
- **Spoken Macro (ES // EN):** Un patrón parece artificial cuando combina regularidades funcionales difíciles de producir por mecanismos naturales conocidos. // *A pattern appears artificial when it combines functional regularities that are difficult to produce by known natural mechanisms.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=1.50 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Narrates engineered signal structure — Prime sequences (`F00`), controlled phase/FM/AM carrier modulation (`F01`), low-entropy ordered lattices (`F02`), error-correcting parity blocks (`F03`), narrow energy-efficient beacon (`F04` — partially echoed by upward beam), and pulsar impostor (`F05`). Note WGSL already has Prime rings + pulsar rays in `221` yet routes Z06 to plain `220`!
- **Zone 3D Procedural Blueprint:** Carrier Modulation & Natural-Impostor Stage (vid=3206): combine waveform constellation visualizer with dish sky beam

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **números primos** / *prime numbers* | `[TIER-D]` | Una secuencia de cantidades divisibles solo por uno y por sí mismas puede señalar selección deliberada, pero necesita una codificación inequívoca. // *A sequence of quantities divisible only by one and by themselves may signal deliberate selection, but needs unambiguous codification.* | `vid=220` ignores `scene=5` & `doc_feat=0` | crisp prime pulse train 2-3-5-7-11 (prefiguring Z12) |
| `F1` | **modulación** / *modulation* | `[TIER-D]` | Cambios controlados de frecuencia, fase o amplitud pueden transportar información y revelar una portadora. // *Controlled changes in frequency, phase, or amplitude can carry information and reveal a carrier.* | `vid=220` ignores `scene=5` & `doc_feat=1` | smooth sine carrier modulated cleanly into sharp QPSK/FSK phase jumps |
| `F2` | **baja entropía** / *low entropy* | `[TIER-D]` | Demasiado orden puede indicar una baliza; demasiada complejidad puede parecer ruido hasta encontrar su estructura. // *Too much order can indicate a beacon; Too much complexity can seem like noise until you find its structure.* | `vid=220` ignores `scene=5` & `doc_feat=2` | high-entropy fuzzy background dust contrasting razor-sharp low-entropy geometric lattice block |
| `F3` | **corrección de errores** / *error correction* | `[TIER-D]` | Redundancia diseñada para reconstruir datos dañados sería una fuerte pista de comunicación intencional. // *Redundancy designed to reconstruct corrupted data would be a strong clue to intentional communication.* | `vid=220` ignores `scene=5` & `doc_feat=3` | 2D Reed-Solomon parity block auto-healing deliberately blanked corrupted bits |
| `F4` | **economía energética** / *energy economy* | `[TIER-B]` | Una señal estrecha, dirigida y repetida puede ser una forma eficiente de hacerse detectable a gran distancia. // *A narrow, targeted, repeated signal can be an efficient way to make yourself detectable over long distances.* | `vid=220` ignores `scene=5` & `doc_feat=4` | hyper-collimated narrow laser needle beating wide isotropic sphere budget |
| `F5` | **imitación natural** / *natural imitation* | `[TIER-D]` | Púlsares y otros fenómenos recuerdan que la naturaleza puede parecer diseñada antes de comprender su mecanismo. // *Pulsars and other phenomena remind us that nature can seem designed before its mechanism is understood.* | `vid=220` ignores `scene=5` & `doc_feat=5` | rapidly spinning oblique magnetized neutron-star lighthouse beam warning against LGM-1 pulsar impostors |

#### `Exp 22 Z07` `accumulated-evidence` — Evidencia acumulada / *Accumulated evidence* (`[TIER-C]` · Target `vid=3207`)
- **Metric (`raw_metric`):** "Observaciones compatibles × explicaciones fallidas × predicciones acertadas → confianza creciente" (*Compatible observations × failed explanations × successful predictions → increasing confidence*) · `scene=6` · `footer="EVIDENCIA CONVERGENTE, CONFIANZA REVISABLE"` · `evidence_class`: "Inferencia convergente · ninguna prueba debe cargar sola la conclusión" (*Convergent inference · no proof should carry the conclusion alone*) · `range=[0.250, 0.286]`
- **Spoken Macro (ES // EN):** La evidencia se fortalece cuando diferentes mediciones apuntan a la misma explicación y las alternativas producen predicciones peores. // *The evidence is strengthened when different measurements point to the same explanation and alternatives produce worse predictions.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=1.75 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Routed to generic `220` SETI dish (`odd_zone=1` tint shift only). Z07 represents progressive Bayesian/Rio-scale evidentiary accumulation (`interesting -> unexplained -> candidate -> probable -> confirmed -> revisable`) across multiple failed alternatives.
- **Zone 3D Procedural Blueprint:** Convergent Inference Rio-Scale Pyramid (vid=3207): translucent 6-tier luminous glowing pyramid fed by converging diagnostic check-beams

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **señal interesante** / *interesting signal* | `[TIER-C]` | El fenómeno merece seguimiento, pero todavía encaja en varias categorías naturales o instrumentales. // *The phenomenon deserves follow-up, but still fits into several natural or instrumental categories.* | `vid=220` ignores `scene=6` & `doc_feat=0` | dim wide baseline question glow |
| `F1` | **señal inexplicada** / *unexplained signal* | `[TIER-C]` | Los análisis actuales no encuentran una causa satisfactoria; desconocido no significa artificial. // *Current analyses have not found a satisfactory cause; unknown does not mean artificial.* | `vid=220` ignores `scene=6` & `doc_feat=1` | unexplained floating anomaly sphere |
| `F2` | **posible tecnofirma** / *possible technosignature* | `[TIER-C]` | Algunas propiedades son compatibles con tecnología, aunque las alternativas siguen abiertas. // *Some properties are consistent with technology, although alternative explanations remain open.* | `vid=220` ignores `scene=6` & `doc_feat=2` | shimmering candidate halo with open rival doors |
| `F3` | **tecnofirma probable** / *probable technosignature* | `[TIER-C]` | La hipótesis tecnológica explica mejor múltiples datos independientes y sobrevive a pruebas específicas. // *The technological hypothesis best explains multiple independent data and survives specific tests.* | `vid=220` ignores `scene=6` & `doc_feat=3` | collapsing rival explanations turning dim while technological spine brightens |
| `F4` | **evidencia confirmada** / *confirmed evidence* | `[TIER-C]` | La comunidad puede reproducir el resultado, predecir nuevas observaciones y excluir de forma robusta otras causas. // *The community can reproduce the result, predict new observations, and robustly exclude other causes.* | `vid=220` ignores `scene=6` & `doc_feat=4` | golden solid locked multi-instrument apex |
| `F5` | **confianza revisable** / *reviewable trust* | `[TIER-C]` | Incluso una conclusión muy fuerte debe poder cambiar ante mejores datos o una explicación natural nueva. // *Even a very strong conclusion must be able to change in the face of better data or a new natural explanation.* | `vid=220` ignores `scene=6` & `doc_feat=5` | open hinged top cap letting fresh data ring adjust needle smoothly |

#### `Exp 22 Z08` `scientific-announcement` — Anuncio científico / *Scientific announcement* (`[TIER-C]` · Target `vid=3208`)
- **Metric (`raw_metric`):** "Revisión + datos abiertos + lenguaje preciso + correcciones → conocimiento público resistente" (*Review + open data + precise language + corrections → resilient public knowledge*) · `scene=7` · `footer="LO OBSERVADO · LO INFERIDO · LO DESCONOCIDO"` · `evidence_class`: "Transparencia responsable · comunicar certeza y límites" (*Responsible transparency · communicate certainty and limits*) · `range=[0.286, 0.321]`
- **Spoken Macro (ES // EN):** El anuncio tendría que separar lo observado, lo inferido y lo todavía desconocido para evitar convertir probabilidad en certeza. // *The announcement would have to separate the observed, the inferred and the still unknown to avoid converting probability into certainty.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=2.00 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Generic `220` SETI dish with zero relation to scientific communication transparency: peer review scrubbing, open data repository sharing, premature leaks vs verified error bars, Rio terminology tiers (`signal/technosignature/message/contact`), probability ribbons, and public errata corrections.
- **Zone 3D Procedural Blueprint:** Open Science Disclosure & Verification Filter (vid=3208): multi-stage translucent glass prism pipeline separating noisy raw packet into clean transparent bands

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **revisión por pares** / *peer review* | `[TIER-C]` | Especialistas examinan métodos, supuestos y alternativas antes de que la afirmación se estabilice. // *Specialists examine methods, assumptions and alternatives before the claim is stabilized.* | `vid=220` ignores `scene=7` & `doc_feat=0` | rotating team inspection lenses probing central node |
| `F1` | **datos abiertos** / *open data* | `[TIER-C]` | Compartir observaciones y código permite que otros grupos encuentren errores o confirmen el resultado. // *Sharing observations and code allows other groups to find errors or confirm the result.* | `vid=220` ignores `scene=7` & `doc_feat=1` | radially broadcast open-data packet streams reaching independent labs |
| `F2` | **filtración prematura** / *premature leak* | `[TIER-C]` | Una noticia incompleta puede circular antes de las verificaciones y fijar interpretaciones difíciles de corregir. // *Incomplete news can circulate before verification and establish interpretations that are difficult to correct.* | `vid=220` ignores `scene=7` & `doc_feat=2` | premature red/orange jagged leak spark outpacing slow rigorous verification track |
| `F3` | **terminología** / *terminology* | `[TIER-C]` | Palabras como señal, tecnofirma, mensaje y contacto implican niveles de evidencia diferentes. // *Words like signal, technosignature, message and contact imply different levels of evidence.* | `vid=220` ignores `scene=7` & `doc_feat=3` | 4-step ascending vocabulary blocks (`signal -> signature -> message -> contact`) |
| `F4` | **incertidumbre pública** / *public uncertainty* | `[TIER-C]` | Expresar probabilidades no es debilidad: muestra qué parte del conocimiento puede cambiar. // *Expressing probabilities is not weakness: it shows what part of knowledge can change.* | `vid=220` ignores `scene=7` & `doc_feat=4` | translucent Gaussian probability cloud rather than hard crisp block |
| `F5` | **rectificación** / *rectification* | `[TIER-C]` | Una corrección rápida y visible protege mejor la ciencia que defender una conclusión por prestigio. // *A quick and visible correction protects science better than defending a conclusion for prestige.* | `vid=220` ignores `scene=7` & `doc_feat=5` | bright golden self-correcting patch seamlessly replacing flawed segment |

#### `Exp 22 Z09` `international-coordination` — Coordinación internacional / *International coordination* (`[TIER-C]` · Target `vid=3209`)
- **Metric (`raw_metric`):** "Ciencia + gobiernos + instituciones + ciudadanía → coordinación sin una voz única" (*Science + governments + institutions + citizens → coordination without a single voice*) · `scene=8` · `footer="NINGUNA VOZ ÚNICA HABLA POR TODO EL PLANETA"` · `evidence_class`: "Gobernanza planetaria · autoridad dispersa, consecuencias compartidas" (*Planetary governance · dispersed authority, shared consequences*) · `range=[0.321, 0.357]`
- **Spoken Macro (ES // EN):** Ninguna organización representa por sí sola a la humanidad; cualquier proceso legítimo tendría que admitir pluralidad y desacuerdo. // *No organization alone represents humanity; any legitimate process would have to accommodate pluralism and disagreement.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=2.25 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** COLLIDES on scene=8 with Z10 (same `footer`: `NINGUNA VOZ ÚNICA HABLA POR TODO EL PLANETA`); both get identical `220` dish separated only by 0.25 rad yaw and odd/even tint! Z09 speaks of pluralistic international coordination across observatory networks, national states, global forums, private companies, scientists, and civil society.
- **Zone 3D Procedural Blueprint:** Polycentric Earth Stakeholder Ring (vid=3209): split scene=8; show wireframe Earth encircled by six distinct un-centralized orbiting rings/constellations without a single monolithic crown

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **observatorios** / *observatories* | `[TIER-D]` | Las redes científicas pueden coordinar seguimiento, formatos de datos y criterios de verificación. // *Scientific networks can coordinate monitoring, data formats and verification criteria.* | `vid=220` ignores `scene=8` & `doc_feat=0` | interconnected telescope dish network across continents |
| `F1` | **gobiernos** / *governments* | `[TIER-C]` | Los Estados podrían controlar infraestructuras, espectro radioeléctrico o información, pero no monopolizar el significado del hallazgo. // *States could control infrastructure, radio spectrum or information, but not monopolize the meaning of the discovery.* | `vid=220` ignores `scene=8` & `doc_feat=1` | national state nodes regulating RF/optical bands |
| `F2` | **instituciones globales** / *global institutions* | `[TIER-C]` | Foros internacionales pueden facilitar deliberación, aunque sus decisiones no serían automáticamente universales ni vinculantes. // *International forums can facilitate deliberation, although their decisions would not automatically be universal or binding.* | `vid=220` ignores `scene=8` & `doc_feat=2` | wide UN/IAU circular deliberative table with non-binding dotted links |
| `F3` | **empresas** / *companies* | `[TIER-C]` | Compañías privadas podrían poseer telescopios, transmisores o plataformas informativas con influencia real. // *Private companies could own telescopes, transmitters or information platforms with real influence.* | `vid=220` ignores `scene=8` & `doc_feat=3` | glowing commercial mega-array & private satellite clusters |
| `F4` | **comunidades científicas** / *scientific communities* | `[TIER-C]` | La experiencia técnica es imprescindible, pero no decide por sí sola valores, riesgos o representación. // *Technical expertise is essential, but it cannot by itself decide questions of values, risks, or representation.* | `vid=220` ignores `scene=8` & `doc_feat=4` | crisp technical data core beside separate wider ethical/value ring |
| `F5` | **sociedad civil** / *civil society* | `[TIER-C]` | La diversidad cultural y generacional importa porque una respuesta podría comprometer a personas que nunca participaron en la decisión. // *Cultural and generational diversity matters because a response could bind people who never took part in the decision.* | `vid=220` ignores `scene=8` & `doc_feat=5` | diverse glowing civic & intergenerational halo enclosing the whole Earth |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP22-Z05-09)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 22 (22_cosmic_contact) Zones Z05..Z09 (`distant-origin`, `artificial-pattern`, `accumulated-evidence`, `scientific-announcement`, `international-coordination`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z05->3205`, `Z06->3206`, `Z07->3207`, `Z08->3208`, `Z09->3209` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Disambiguate colliding baked scene indices in Z00..Z27 by keying directly on doc.active_zone_index -> vid=3200..3227 and spawning companion stages 3830..3835 where dual actors/wavefronts apply.
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `seti` & `rosetta` & `primo` (exist), plus `radar`, `laser`, `radio`, `descifrado`, `silencio`, `meti`, `tratado`, `relativista`, `espejo`, `archivo`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_civilization_horizon.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z05->3205`, `Z06->3206`, `Z07->3207`, `Z08->3208`, `Z09->3209`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z05 (vid=3205, distant-origin): F0:celestial position; F1:Doppler drift; F2:dispersion; F3:parallax; F4:associated star; F5:moving source
     * Z06 (vid=3206, artificial-pattern): F0:prime numbers; F1:modulation; F2:low entropy; F3:error correction; F4:energy economy; F5:natural imitation
     * Z07 (vid=3207, accumulated-evidence): F0:interesting signal; F1:unexplained signal; F2:possible technosignature; F3:probable technosignature; F4:confirmed evidence; F5:reviewable trust
     * Z08 (vid=3208, scientific-announcement): F0:peer review; F1:open data; F2:premature leak; F3:terminology; F4:public uncertainty; F5:rectification
     * Z09 (vid=3209, international-coordination): F0:observatories; F1:governments; F2:global institutions; F3:companies; F4:scientific communities; F5:civil society
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp22-z10-14"></a>
### PKG-ACT2-EXP22-Z10-14: Social Reaction, Prime Math Signatures & Cryptographic Decoding (`z=10..14`, `scene=8` part 2) — `Phase 1` · Priority: **Critical**

**Current State:** Zones `Z10..Z14` (`5` zones, `30` features) | Zone Tiers `0A / 1B / 2C / 2D` | Feature Tiers `0A / 2B / 16C / 12D` | Render path: [`doc_stages.rs#L79-L82`](../../crates/cosmos_render/src/doc_stages.rs#L79-L82) -> [`shaders/hero_doc_civilization_horizon.wgsl#L389-L429`](../../shaders/hero_doc_civilization_horizon.wgsl#L389-L429) — `z=12 -> vid=221`, all other `27` zones -> `vid=220` SETI dish (`5` shared `scene` pairs). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `30` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z10` | `planetary-reaction` | Reacción planetaria / *planetary reaction* | **`3210`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z11` | `interpretation` | Interpretación / *Interpretation* | **`3211`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z12` | `shared-mathematics` | Matemáticas compartidas / *Shared Mathematics* | **`3212`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z13` | `meaning-construction` | Construcción de significado / *Construction of meaning* | **`3213`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z14` | `unknown-language` | Lenguaje desconocido / *Unknown language* | **`3214`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 22 Z10` `planetary-reaction` — Reacción planetaria / *planetary reaction* (`[TIER-C]` · Target `vid=3210`)
- **Metric (`raw_metric`):** "Información + identidad + confianza institucional + cultura → respuestas humanas múltiples" (*Information + identity + institutional trust + culture → multiple human responses*) · `scene=8` · `footer="NINGUNA VOZ ÚNICA HABLA POR TODO EL PLANETA"` · `evidence_class`: "Descubrimiento compartido · interpretaciones sociales divergentes" (*Shared discovery · divergent social interpretations*) · `range=[0.357, 0.393]`
- **Spoken Macro (ES // EN):** La misma evidencia podría provocar curiosidad, temor, esperanza, indiferencia o disputa sin que una reacción represente a toda la especie. // *The same evidence could provoke curiosity, fear, hope, indifference or dispute without a reaction representing the entire species.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=2.50 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** COLLIDES on scene=8 with Z09; still shows a lone SETI parabolic dish (`220`) while narration explores divergent human social reactions: sensationalized media, viral disinformation/conspiracies, world religions, philosophy, ordinary daily life continuing unabated, and global informational inequality.
- **Zone 3D Procedural Blueprint:** Social Refraction Kaleidoscope Dome (vid=3210): central verified celestial beacon refracting outward through swirling translucent multi-faceted human culture prisms around Earth

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **medios** / *media* | `[TIER-C]` | Titulares simplificados pueden acelerar el conocimiento y también borrar la diferencia entre posibilidad y confirmación. // *Simplified headlines can spread awareness quickly while also blurring the difference between possibility and confirmation.* | `vid=220` ignores `scene=8` & `doc_feat=0` | flashing headline wave flattening subtle spectrum into stark binary bars |
| `F1` | **desinformación** / *disinformation* | `[TIER-C]` | Imágenes falsas, mensajes inventados y teorías de conspiración competirían con los datos verificables. // *Fake images, fabricated messages, and conspiracy theories would compete with verifiable data.* | `vid=220` ignores `scene=8` & `doc_feat=1` | distorted fractal fake-signal mirror swarms competing with true beam |
| `F2` | **religiones** / *religions* | `[TIER-C]` | Tradiciones distintas podrían integrar el hallazgo de maneras compatibles, tensas o transformadoras. // *Different traditions could integrate the finding in compatible, tense, or transformative ways.* | `vid=220` ignores `scene=8` & `doc_feat=2` | harmonic radiant golden-teal sacred geometric arches integrating light differently |
| `F3` | **filosofía** / *philosophy* | `[TIER-C]` | El descubrimiento reabriría preguntas sobre mente, valor, singularidad humana y pertenencia cósmica. // *The discovery would reopen questions about mind, value, human uniqueness and cosmic belonging.* | `vid=220` ignores `scene=8` & `doc_feat=3` | expanding contemplative Cartesian/cosmic horizon ring around neural silhouette |
| `F4` | **vida cotidiana** / *everyday life* | `[TIER-C]` | La mayoría de las personas seguiría viviendo entre obligaciones inmediatas aunque cambiara la visión del universo. // *Most people would continue to live among immediate obligations even if their view of the universe changed.* | `vid=220` ignores `scene=8` & `doc_feat=4` | quiet mundane spinning orbital workday cities barely glancing up |
| `F5` | **desigualdad** / *inequality* | `[TIER-C]` | El acceso a datos, educación y decisiones podría distribuirse de forma muy desigual entre regiones y grupos. // *Access to data, education and decisions could be distributed very unevenly between regions and groups.* | `vid=220` ignores `scene=8` & `doc_feat=5` | steep illuminated-vs-darkness divide across northern/southern hemisphere access arcs |

#### `Exp 22 Z11` `interpretation` — Interpretación / *Interpretation* (`[TIER-D]` · Target `vid=3211`)
- **Metric (`raw_metric`):** "Segmentación + repetición + relaciones + contexto físico → hipótesis de lectura" (*Segmentation + repetition + relationships + physical context → reading hypothesis*) · `scene=9` · `footer="PRIMERO ESTRUCTURA · DESPUÉS SIGNIFICADO"` · `evidence_class`: "Análisis semiótico · detectar estructura antes de atribuir significado" (*Semiotic analysis · detect structure before attributing meaning*) · `range=[0.393, 0.429]`
- **Spoken Macro (ES // EN):** Antes de traducir, habría que descubrir dónde empiezan y terminan las unidades, qué relaciones conservan y qué función cumple el mensaje. // *Before translating, you should discover where the units begin and end, what relationships they maintain, and what function the message fulfills.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=2.75 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Narrates concrete cryptanalytic / information-theory preprocessing before translation: bitstream boundary segmentation (`F00`), 1D-to-2D prime matrix folding (`F01`), redundancy padding (`F02`), lossless algorithmic compression appearing random (`F03`), beacon-vs-archive-vs-leak structure (`F04`), and delimiter silences (`F05`). Plain dish (`220`) shows none of this.
- **Zone 3D Procedural Blueprint:** Semiotics & Bitstream Matrix Unfolder (vid=3211): glowing serial binary pulse stream entering central de-framing prism

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **segmentación** / *segmentation* | `[TIER-D]` | Pausas, cabeceras, ritmos o bloques repetidos podrían indicar unidades, pero también formar parte del canal. // *Pauses, headers, rhythms or repeated blocks could indicate units, but also be part of the channel.* | `vid=220` ignores `scene=9` & `doc_feat=0` | hovering cursor slicing continuous stream at rhythmic header/pause gaps |
| `F1` | **orden** / *ordering* | `[TIER-D]` | Determinar si los datos se leen en una dimensión, en una matriz, por capas o en varias escalas cambia todo el contenido. // *Determining whether data is read in one dimension, in an array, in layers, or at multiple scales changes everything.* | `vid=220` ignores `scene=9` & `doc_feat=1` | interactive 1D ribbon smoothly folding sequentially into crisp 23x73 Arecibo-style 2D bitmap image |
| `F2` | **redundancia** / *redundancy* | `[TIER-D]` | La repetición puede ser énfasis, sincronización, corrección de errores o simplemente una propiedad del emisor. // *Repetition can be emphasis, timing, error correction, or simply a property of the sender.* | `vid=220` ignores `scene=9` & `doc_feat=2` | duplicated shimmering parity blocks stacking beside core frames |
| `F3` | **compresión** / *compression* | `[TIER-D]` | Un mensaje muy comprimido parece aleatorio hasta conocer el algoritmo o las regularidades que elimina. // *A highly compressed message appears random until you know the algorithm or the regularities it eliminates.* | `vid=220` ignores `scene=9` & `doc_feat=3` | razor-flat dense compressed white-noise column blooming suddenly into ordered structure when key unlocks |
| `F4` | **intención** / *intention* | `[TIER-D]` | Una baliza, archivo, saludo, advertencia o fuga tecnológica exigirían estrategias de interpretación diferentes. // *A beacon, file, greeting, warning or technological leak would require different interpretation strategies.* | `vid=220` ignores `scene=9` & `doc_feat=4` | 5 branching envelope silhouettes (lighthouse/library/greeting/warning/exhaust) |
| `F5` | **silencio interno** / *internal silence* | `[TIER-D]` | Ausencias y intervalos pueden transportar información, delimitar símbolos o no significar nada. // *Absences and intervals can carry information, delimit symbols or mean nothing.* | `vid=220` ignores `scene=9` & `doc_feat=5` | dark empty gap intervals pulsing as meaningful delimiter beats |

#### `Exp 22 Z12` `shared-mathematics` — Matemáticas compartidas / *Shared Mathematics* (`[TIER-B]` · Target `vid=3212`)
- **Metric (`raw_metric`):** "Relaciones invariantes + ejemplos construidos → primeros puntos de apoyo" (*Invariant relations + constructed examples → first support points*) · `scene=10` · `footer="REFERENCIAS COMUNES · NO UN LENGUAJE COMPLETO"` · `evidence_class`: "Referencia potencial · universalidad física, convención representacional" (*Potential reference · physical universality, representational convention*) · `range=[0.429, 0.464]`
- **Spoken Macro (ES // EN):** Las matemáticas podrían proporcionar puntos de referencia comunes, pero no constituyen automáticamente un lenguaje completo. // *Mathematics might provide common reference points, but it does not automatically constitute a complete language.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** z==12 -> vid=221 (`mode=1`: 5 Prime cipher rings `2,3,5,7,11` + pulsar rays; `doc_feat` unused; `odd_zone` modulates phase/tint)
- **Audit Gap:** Bespoke z==12 -> vid=221 (`mode=1` in hero_doc_civilization_horizon.wgsl:389-429): renders 5 tilted concentric Prime cipher rings (`2,3,5,7,11`) + central pulsar cross rays, matching F00 Prime numbers and circular geometry hints (`F01`). However, `doc_feat` is completely unread: operational triangles/proportions (`F01`), 21-cm hydrogen physical constants (`F02`), dimensionless ratios (`F03`), Boolean logic (`F04`), and mathematical limits (`F05`) are absent.
- **Zone 3D Procedural Blueprint:** Upgrade vid=221 Shared-Mathematics Cipher Theater: keep 5 concentric Prime rings (`2,3,5,7,11`) as anchor

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **números primos** / *prime numbers* | `[TIER-B]` | Las cantidades pueden demostrarse mediante pulsos, aunque la base numérica y la forma de agruparlos siguen siendo convenciones. // *Quantities can be demonstrated by pulses, although the numerical basis and how they are grouped remain conventions.* | `vid=220` ignores `scene=10` & `doc_feat=0` | ignite each ring sequentially with discrete counted dot-pulses (2,3,5,7,11) plus binary-vs-base10 grouping comparison |
| `F1` | **geometría** / *geometry* | `[TIER-B]` | Triángulos, círculos y proporciones pueden definirse operacionalmente sin compartir palabras. // *Triangles, circles, and proportions can be defined operationally without sharing words.* | `vid=220` ignores `scene=10` & `doc_feat=1` | spawn Euclidean 3-4-5 right triangles and golden spirals rotating cleanly inside rings |
| `F2` | **constantes físicas** / *physical constants* | `[TIER-D]` | Frecuencias atómicas o velocidades relativas pueden crear unidades reproducibles en ambos mundos. // *Atomic frequencies or relative velocities can create reproducible units in both worlds.* | `vid=220` ignores `scene=10` & `doc_feat=2` | render iconic neutral-hydrogen 21cm hyperfine spin-flip twin-electron icon + c-pulse ruler |
| `F3` | **proporciones** / *proportions* | `[TIER-D]` | Relaciones sin dimensiones evitan depender de metros, segundos o escalas corporales humanas. // *Dimensionless relationships avoid depending on meters, seconds or human body scales.* | `vid=220` ignores `scene=10` & `doc_feat=3` | nest dimensionless pi circumference/diameter & fine-structure ratio calipers |
| `F4` | **lógica** / *logic* | `[TIER-C]` | Patrones de implicación y consistencia pueden ser comunes, pero no garantizan la misma filosofía del razonamiento. // *Patterns of implication and consistency may be common, but they do not guarantee the same philosophy of reasoning.* | `vid=220` ignores `scene=10` & `doc_feat=4` | illuminate branching AND/OR/IMPLIES logic gate nodes |
| `F5` | **límite matemático** / *mathematical limit* | `[TIER-C]` | Las matemáticas podrían proporcionar puntos de referencia comunes, pero no constituyen automáticamente un lenguaje completo. // *Mathematics might provide common reference points, but it does not automatically constitute a complete language.* | `vid=220` ignores `scene=10` & `doc_feat=5` | dissolve outer ring edge softly into misty natural-language void ('references != complete language') |

#### `Exp 22 Z13` `meaning-construction` — Construcción de significado / *Construction of meaning* (`[TIER-C]` · Target `vid=3213`)
- **Metric (`raw_metric`):** "Referencia conocida → transformación → contraste → generalización → concepto provisional" (*Known reference → transformation → contrast → generalization → provisional concept*) · `scene=11` · `footer="EJEMPLO → CONTRASTE → RELACIÓN → CONCEPTO"` · `evidence_class`: "Anclaje progresivo · símbolos conectados con relaciones observables" (*Progressive grounding · symbols connected to observable relationships*) · `range=[0.464, 0.500]`
- **Spoken Macro (ES // EN):** El significado podría construirse mostrando ejemplos y variaciones, no asignando de inmediato una palabra alienígena a una palabra humana. // *Meaning could be constructed by showing examples and variations, not immediately assigning an alien word to a human word.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=3.25 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Plain SETI dish (`220`) fails to illustrate Lincos-style operational meaning grounding: pointing to observable astronomical/physical referents (`F00`), minimal-pair contrast flips (`F01`), geometric rotation/addition/inversion (`F02`), positive/negative classification sets (`F03`), syntactic parse trees (`F04`), and revisable translation hypotheses (`F05`).
- **Zone 3D Procedural Blueprint:** Visual Lincos Meaning-Grounding Sandbox (vid=3213): clean holographic floating slate above subtle faint emitter glyph

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **señalar referentes** / *identify referents* | `[TIER-D]` | Un mensaje puede vincular patrones con una estrella, una frecuencia o una geometría observable por ambos. // *A message can link patterns to a star, a frequency, or a geometry observable by both.* | `vid=220` ignores `scene=11` & `doc_feat=0` | golden pointer laser connecting binary symbol packet directly to twinkling nearby star & hydrogen orbital |
| `F1` | **pares mínimos** / *minimum pairs* | `[TIER-C]` | Cambiar una sola parte y mostrar el efecto ayuda a inferir qué componente controla cantidad, tiempo o relación. // *Changing a single part and showing the effect helps infer which component controls quantity, time, or ratio.* | `vid=220` ignores `scene=11` & `doc_feat=1` | twin flickering side-by-side glyph panels flipping single dark pixel while watching count change |
| `F2` | **transformaciones** / *transformations* | `[TIER-D]` | Rotar, sumar, repetir o invertir figuras puede enseñar operaciones antes que vocabulario. // *Rotating, adding, repeating or inverting figures can teach operations before vocabulary.* | `vid=220` ignores `scene=11` & `doc_feat=2` | spinning/translating/mirroring glowing polyhedra teaching verbs (+, rotate, invert) |
| `F3` | **clasificación** / *classification* | `[TIER-C]` | Conjuntos de ejemplos positivos y negativos permiten inferir categorías sin asumir que coinciden con las humanas. // *Sets of positive and negative examples allow categories to be inferred without assuming that they coincide with human ones.* | `vid=220` ignores `scene=11` & `doc_feat=3` | luminous green '+' cluster vs amber '-' cluster bounding shapes |
| `F4` | **sintaxis** / *syntax* | `[TIER-C]` | El orden y la dependencia entre símbolos pueden revelar funciones incluso cuando cada elemento siga sin traducirse. // *The order and dependency between symbols can reveal functions even when each element remains untranslated.* | `vid=220` ignores `scene=11` & `doc_feat=4` | directed dependency arcs linking abstract placeholder boxes without English labels |
| `F5` | **semántica provisional** / *provisional semantics* | `[TIER-C]` | Toda traducción inicial sería un modelo revisable, no una equivalencia definitiva entre dos mentes. // *Any initial translation would be a revisable model, not a definitive equivalence between two minds.* | `vid=220` ignores `scene=11` & `doc_feat=5` | shifting translucent overlapping translation hypothesis lattices |

#### `Exp 22 Z14` `unknown-language` — Lenguaje desconocido / *Unknown language* (`[TIER-D]` · Target `vid=3214`)
- **Metric (`raw_metric`):** "Patrones + contexto + respuestas futuras → gramática tentativa y significado parcial" (*Patterns + context + future answers → tentative grammar and partial meaning*) · `scene=12` · `footer="ESTRUCTURA SIN VOZ, ESCRITURA NI IMAGEN HUMANA"` · `evidence_class`: "Sistema ajeno · forma, función y mente todavía inciertas" (*Alien system · form, function and mind still uncertain*) · `range=[0.500, 0.536]`
- **Spoken Macro (ES // EN):** Un mensaje puede contener estructura sin parecerse a voz, escritura, imagen ni lenguaje humano. // *A message can contain structure without resembling human voice, writing, image, or language.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=3.50 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Narrates non-human multidimensional signal encoding across simultaneous physical observables (`frequency x phase x polarization x time` `F00`), deeply nested recursive bracket structures (`F01`), untranslatable sensory concepts (`F02`), absent extraterrestrial referents (`F03`), and AI hallucinated coherence (`F04`). Lone dish (`220`) gives no hint of multi-channel multiplexing or recursive trees.
- **Zone 3D Procedural Blueprint:** 4D Alien Multiplexing Cube & Recursive Grammar Tree (vid=3214)

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **múltiples canales** / *multiple channels* | `[TIER-D]` | El contenido podría distribuirse simultáneamente entre frecuencia, fase, polarización y tiempo. // *The content could be distributed simultaneously across frequency, phase, polarization and time.* | `vid=220` ignores `scene=12` & `doc_feat=0` | rotating iridescent 4D hypercube/voxel lattice projecting simultaneous frequency bands, spiral polarization vectors, phase shifts & time depth |
| `F1` | **gramática recursiva** / *recursive grammar* | `[TIER-D]` | Estructuras dentro de estructuras podrían codificar relaciones complejas o simplemente optimizar transmisión. // *Structures within structures could encode complex relationships or simply optimize transmission.* | `vid=220` ignores `scene=12` & `doc_feat=1` | fractal self-similar nested torus-within-torus recursive tree glowing downward endlessly |
| `F2` | **conceptos intraducibles** / *untranslatable concepts* | `[TIER-C]` | Una mente con otros sentidos o ciclos vitales puede distinguir categorías que no tenemos palabras para expresar. // *A mind with other senses or life cycles can distinguish categories that we do not have words to express.* | `vid=220` ignores `scene=12` & `doc_feat=2` | weird non-visual vortex glyphs evaporating into mist beyond human cone |
| `F3` | **referencias ausentes** / *absent references* | `[TIER-C]` | El mensaje puede asumir conocimiento de lugares, especies o acontecimientos imposibles de observar desde la Tierra. // *The message may assume knowledge of places, species or events impossible to observe from Earth.* | `vid=220` ignores `scene=12` & `doc_feat=3` | empty silhouette sockets pointing to dark uncharted exoplanet constellations |
| `F4` | **traducción automática** / *automatic translation* | `[TIER-C]` | Los modelos computacionales pueden buscar regularidades, pero también amplificar supuestos humanos y producir coherencia falsa. // *Computational models can look for regularities, but also amplify human assumptions and produce false coherence.* | `vid=220` ignores `scene=12` & `doc_feat=4` | overfitted neural hallucination grid drawing crisp illusory human faces over pure random noise |
| `F5` | **varias lecturas** / *several readings* | `[TIER-C]` | Interpretaciones distintas pueden explicar los mismos datos hasta que una respuesta posterior permita discriminarlas. // *Different interpretations can explain the same data until a subsequent response allows them to be discriminated.* | `vid=220` ignores `scene=12` & `doc_feat=5` | two completely distinct crystal graphs projecting equally well onto identical raw data shadow |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP22-Z10-14)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 22 (22_cosmic_contact) Zones Z10..Z14 (`planetary-reaction`, `interpretation`, `shared-mathematics`, `meaning-construction`, `unknown-language`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z10->3210`, `Z11->3211`, `Z12->3212`, `Z13->3213`, `Z14->3214` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Disambiguate colliding baked scene indices in Z00..Z27 by keying directly on doc.active_zone_index -> vid=3200..3227 and spawning companion stages 3830..3835 where dual actors/wavefronts apply.
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `seti` & `rosetta` & `primo` (exist), plus `radar`, `laser`, `radio`, `descifrado`, `silencio`, `meti`, `tratado`, `relativista`, `espejo`, `archivo`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_civilization_horizon.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z10->3210`, `Z11->3211`, `Z12->3212`, `Z13->3213`, `Z14->3214`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z10 (vid=3210, planetary-reaction): F0:media; F1:disinformation; F2:religions; F3:philosophy; F4:everyday life; F5:inequality
     * Z11 (vid=3211, interpretation): F0:segmentation; F1:ordering; F2:redundancy; F3:compression; F4:intention; F5:internal silence
     * Z12 (vid=3212, shared-mathematics): F0:prime numbers; F1:geometry; F2:physical constants; F3:proportions; F4:logic; F5:mathematical limit
     * Z13 (vid=3213, meaning-construction): F0:identify referents; F1:minimum pairs; F2:transformations; F3:classification; F4:syntax; F5:provisional semantics
     * Z14 (vid=3214, unknown-language): F0:multiple channels; F1:recursive grammar; F2:untranslatable concepts; F3:absent references; F4:automatic translation; F5:several readings
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp22-z15-19"></a>
### PKG-ACT2-EXP22-Z15-19: Xenocognition, Reply Dilemmas & Relativistic Light-Cones (`z=15..19`, `scene=13/14/15` split) — `Phase 1` · Priority: **Critical**

**Current State:** Zones `Z15..Z19` (`5` zones, `30` features) | Zone Tiers `0A / 1B / 3C / 1D` | Feature Tiers `0A / 3B / 15C / 12D` | Render path: [`doc_stages.rs#L79-L82`](../../crates/cosmos_render/src/doc_stages.rs#L79-L82) -> [`shaders/hero_doc_civilization_horizon.wgsl#L389-L429`](../../shaders/hero_doc_civilization_horizon.wgsl#L389-L429) — `z=12 -> vid=221`, all other `27` zones -> `vid=220` SETI dish (`5` shared `scene` pairs). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `30` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z15` | `cultural-context` | Contexto cultural / *Cultural context* | **`3215`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z16` | `message-received` | Mensaje recibido / *Received message* | **`3216`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z17` | `decision-to-respond` | Decisión de responder / *Decision to respond* | **`3217`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z18` | `transmission-risks` | Riesgos de transmisión / *Transmission risks* | **`3218`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z19` | `light-speed-limit` | Límites de la velocidad de la luz / *Limits imposed by the speed of light* | **`3219`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 22 Z15` `cultural-context` — Contexto cultural / *Cultural context* (`[TIER-C]` · Target `vid=3215`)
- **Metric (`raw_metric`):** "Biología + ambiente + instituciones + memoria → marco cultural no compartido" (*Biology + environment + institutions + memory → non-shared cultural framework*) · `scene=13` · `footer="MISMA FÍSICA · HISTORIAS MENTALES DISTINTAS"` · `evidence_class`: "Mente situada · significado ligado a historia, cuerpo y sociedad" (*Situated mind · meaning linked to history, body and society*) · `range=[0.536, 0.571]`
- **Spoken Macro (ES // EN):** Comprender frases no bastaría si ignoramos qué cuenta como individuo, obligación, peligro, verdad o tiempo para el emisor. // *Understanding sentences would not be enough if we ignore what counts as an individual, obligation, danger, truth or time for the sender.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=3.75 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** COLLIDES on scene=13 with Z16 (shared footer `MISMA FÍSICA · HISTORIAS MENTALES DISTINTAS`) and runs plain `220` dish. Narration contrasts situated alien cognition (`individual vs colony/continuum`, `ms-vs-century thought timescales`, `magnetic/chemical senses`, `genetic/ecosystem memory`, `non-human values`, `ocean/binary-star metaphors`).
- **Zone 3D Procedural Blueprint:** Situated Xenocultural Mind Prisms (vid=3215): split scene=13

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **individualidad** / *individuality* | `[TIER-D]` | La civilización podría estar compuesta por organismos separados, colonias, redes o una continuidad sin personas discretas. // *Civilization could be composed of separate organisms, colonies, networks, or a continuum without discrete people.* | `vid=220` ignores `scene=13` & `doc_feat=0` | discrete solitary node morphing into fused colonial web & continuous wave |
| `F1` | **tiempo** / *time* | `[TIER-C]` | Un pensamiento puede durar milisegundos, estaciones o siglos, alterando qué significa promesa, urgencia o espera. // *A thought can last milliseconds, seasons, or centuries, altering what promise, urgency, or waiting means.* | `vid=220` ignores `scene=13` & `doc_feat=1` | side-by-side hyper-fast flashing insectoid tick vs glacial tectonic pulse wave |
| `F2` | **sentidos** / *senses* | `[TIER-D]` | Conceptos centrales podrían nacer de campos magnéticos, química, presión o percepciones sin equivalente humano. // *Central concepts could be born from magnetic fields, chemistry, pressure or perceptions with no human equivalent.* | `vid=220` ignores `scene=13` & `doc_feat=2` | shimmering magnetic field lines & swirling scent/pressure eddies replacing visual light |
| `F3` | **memoria** / *memory* | `[TIER-C]` | Una cultura puede conservar el pasado en individuos, genética, máquinas, ecosistemas o estructuras planetarias. // *A culture can preserve the past in individuals, genetics, machines, ecosystems, or planetary structures.* | `vid=220` ignores `scene=13` & `doc_feat=3` | memory stored in helical DNA rings, silicon cores, living forest mycelium, & colossal planetary ruins |
| `F4` | **valores** / *values* | `[TIER-C]` | Cooperación, expansión, conocimiento y supervivencia no tienen por qué ocupar el mismo lugar que en nuestras sociedades. // *Cooperation, expansion, knowledge and survival do not have to occupy the same place as in our societies.* | `vid=220` ignores `scene=13` & `doc_feat=4` | un-human triangular balance prism tilting knowledge/continuity above individual survival |
| `F5` | **metáfora** / *metaphor* | `[TIER-C]` | Una figura construida desde océanos, enjambres o estrellas binarias puede parecer literal cuando desconocemos su experiencia. // *A figure constructed from oceans, swarms, or binary stars can seem literal when we are unaware of their experience.* | `vid=220` ignores `scene=13` & `doc_feat=5` | binary twin-sun shadow tide sculpture turning literal metaphor |

#### `Exp 22 Z16` `message-received` — Mensaje recibido / *Received message* (`[TIER-C]` · Target `vid=3216`)
- **Metric (`raw_metric`):** "Estructura + referencias + contexto inferido → modelo parcial del emisor" (*Structure + references + inferred context → partial model of the sender*) · `scene=13` · `footer="MISMA FÍSICA · HISTORIAS MENTALES DISTINTAS"` · `evidence_class`: "Contenido auténtico · interpretación todavía incompleta" (*Authentic content · interpretation still incomplete*) · `range=[0.571, 0.607]`
- **Spoken Macro (ES // EN):** Recibir información no equivale a entenderla ni a confirmar que describe con honestidad el presente de quien la envió. // *Receiving information is not the same as understanding it or confirming that it honestly describes the sender's present.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=4.00 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** COLLIDES on scene=13 with Z15; still renders lone upward-firing SETI dish (`220`) while describing decoded alien message contents: host star spectra/orbits (`F00`), alien self-portrait (`F01`), curated cultural timeline (`F02`), locally testable physics models (`F03`), ambiguous warning (`F04`), and uncertain sender identity (entire world vs rogue faction vs AI archive `F05`).
- **Zone 3D Procedural Blueprint:** Decoded Alien Dossier Hologram (vid=3216): open segmented floating holographic polyhedron unveiling fragmentary decoded panels

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **descripción del entorno** / *environment description* | `[TIER-D]` | Espectros, órbitas o mapas podrían revelar las condiciones físicas del sistema de origen. // *Spectra, orbits or maps could reveal the physical conditions of the source system.* | `vid=220` ignores `scene=13` & `doc_feat=0` | crisp alien planetary orbital map + stellar absorption barcode |
| `F1` | **autorretrato** / *self portrait* | `[TIER-D]` | El emisor podría representar su cuerpo, red o arquitectura de una forma que confundamos con otra cosa. // *The sender could represent its body, network or architecture in a way that we mistake for something else.* | `vid=220` ignores `scene=13` & `doc_feat=1` | abstract non-human interconnected network/silhouette diagram flickering indeterminately |
| `F2` | **historia** / *history* | `[TIER-C]` | Una cronología transmitida sería una selección cultural, no un registro neutral de todos los acontecimientos. // *A transmitted chronology would be a cultural selection, not a neutral record of all events.* | `vid=220` ignores `scene=13` & `doc_feat=2` | selectively illuminated glowing epochs separated by dark redacted gaps |
| `F3` | **conocimiento científico** / *scientific knowledge* | `[TIER-D]` | Modelos verificables localmente ofrecen una vía para comprobar si parte del mensaje se entiende correctamente. // *Locally verifiable models offer a way to check whether part of the message is understood correctly.* | `vid=220` ignores `scene=13` & `doc_feat=3` | glowing testable particle-decay / gravitational diagram snapping cleanly into Earth lab curve |
| `F4` | **advertencia** / *warning* | `[TIER-C]` | Un aviso puede ser sincero, metafórico, desactualizado, dirigido a otro receptor o interpretado fuera de contexto. // *A warning may be sincere, metaphorical, outdated, intended for another recipient, or interpreted out of context.* | `vid=220` ignores `scene=13` & `doc_feat=4` | amber/crimson warning glyph pulsing beside 5 diverging interpretation arrows |
| `F5` | **identidad dudosa** / *dubious identity* | `[TIER-C]` | No sabríamos de inmediato si habla una civilización completa, una facción, una máquina, un archivo o un impostor. // *We wouldn't immediately know if an entire civilization, a faction, a machine, an archive, or an impostor was speaking.* | `vid=220` ignores `scene=13` & `doc_feat=5` | rotating mask shifting between unified globe, tiny splinter shard, autonomous bot & dusty dead fossil beacon |

#### `Exp 22 Z17` `decision-to-respond` — Decisión de responder / *Decision to respond* (`[TIER-C]` · Target `vid=3217`)
- **Metric (`raw_metric`):** "Beneficios posibles + riesgos irreversibles + legitimidad → decisión disputada" (*Possible benefits + irreversible risks + legitimacy → disputed decision*) · `scene=14` · `footer="RESPONDER, ESPERAR O CALLAR CAMBIA LA RELACIÓN"` · `evidence_class`: "Elección colectiva · ninguna opción es neutral" (*Collective choice · no option is neutral*) · `range=[0.607, 0.643]`
- **Spoken Macro (ES // EN):** Responder, esperar o guardar silencio cambia la relación y plantea quién puede comprometer a generaciones futuras. // *Responding, waiting, or remaining silent changes the relationship and raises the question of who may bind future generations.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=4.25 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** COLLIDES on scene=14 with Z18 (`RESPONDER, ESPERAR O CALLAR CAMBIA LA RELACIÓN`) and runs `vid=220` (`mode=0` dish permanently blasting laser beacon upward!). That always-on laser beam actively contradicts `F00 remain silent` and `F05 wait for consensus`, even while `F01 acknowledge receipt` and `F04 reveal location` vaguely echo the upward beam.
- **Zone 3D Procedural Blueprint:** Earth Reply Decision Console & Consensus Globe (vid=3217): split scene=14; replace always-firing dish with controllable Dish + Earth Council Ring

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **permanecer en silencio** / *remain silent* | `[TIER-C]` | Reduce información enviada, pero también comunica ausencia o incapacidad y deja la iniciativa a actores independientes. // *It reduces information sent, but also communicates absence or incapacity and leaves the initiative to independent actors.* | `vid=220` ignores `scene=14` & `doc_feat=0` | dish powered DOWN dark & silent against quiet stars |
| `F1` | **confirmar recepción** / *acknowledge receipt* | `[TIER-B]` | Un acuse breve limita el contenido y demuestra que la señal fue comprendida al menos como comunicación. // *A brief acknowledgment limits the content and demonstrates that the signal was understood at least as communication.* | `vid=220` ignores `scene=14` & `doc_feat=1` | ultra-brief single white ping wave exiting feed horn once |
| `F2` | **enviar ciencia** / *send science* | `[TIER-C]` | Referencias físicas compartidas pueden ser verificables, aunque incluso el conocimiento básico revela capacidades. // *Shared physical references may be verifiable, although even basic knowledge reveals capabilities.* | `vid=220` ignores `scene=14` & `doc_feat=2` | clean geometric physical-constants pulse ring leaving dish |
| `F3` | **transmitir cultura** / *transmit culture* | `[TIER-C]` | Arte, historias y diversidad muestran más que datos, pero seleccionar qué representa a la humanidad es profundamente político. // *Art, stories and diversity show more than data, but selecting what represents humanity is deeply political.* | `vid=220` ignores `scene=14` & `doc_feat=3` | warm multi-hued Voyager-style cultural mosaic sphere orbiting globe |
| `F4` | **revelar ubicación** / *reveal location* | `[TIER-B]` | La respuesta ya puede delatar nuestra dirección; añadir coordenadas precisas aumenta claridad y exposición. // *The answer may already give away our direction; Adding precise coordinates increases clarity and exposure.* | `vid=220` ignores `scene=14` & `doc_feat=4` | bright directional coordinate beacon piercing outward exposing Earth pin on galactic map |
| `F5` | **esperar acuerdo** / *wait for consensus* | `[TIER-C]` | ¿Quién puede hablar en nombre de un planeta que nunca ha hablado con una sola voz? Buscar legitimidad global mejora representación, pero puede retrasar la respuesta. // *Who can speak for a planet that has never spoken with one voice? Seeking global legitimacy improves representation, but may delay response.* | `vid=220` ignores `scene=14` & `doc_feat=5` | swirling undecided circular assembly ring around Earth delaying beam trigger |

#### `Exp 22 Z18` `transmission-risks` — Riesgos de transmisión / *Transmission risks* (`[TIER-B]` · Target `vid=3218`)
- **Metric (`raw_metric`):** "Contenido × alcance × permanencia × incertidumbre del receptor → riesgo de exposición" (*Content × scope × permanence × receiver uncertainty → risk of exposure*) · `scene=14` · `footer="RESPONDER, ESPERAR O CALLAR CAMBIA LA RELACIÓN"` · `evidence_class`: "Información irreversible · consecuencias difíciles de estimar" (*Irreversible information · consequences that are difficult to estimate*) · `range=[0.643, 0.679]`
- **Spoken Macro (ES // EN):** Una emisión interestelar no puede retirarse una vez enviada y puede ser recibida por destinatarios distintos del previsto. // *An interstellar broadcast cannot be withdrawn once sent and may be received by recipients other than the intended one.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=4.50 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** COLLIDES on scene=14 with Z17, yet the parabolic dish + upward laser pulse (`vid=220`) matches Z18's physical subject (`F02 directed narrow beam` vs transmission risk geometry), though `doc_feat` is ignored — missing wide omnidirectional broadcast (`F03`), competing spoofed planetary dishes (`F04`), sensitive biological payload warnings (`F00`), or dual-use hazards (`F01`).
- **Zone 3D Procedural Blueprint:** Upgrade vid=220 METI Exposure & Beam-Risk Stage (vid=3218): split scene=14; drive beam cone solid-angle & Earth globe transmitters via doc_feat: 0 translucent vulnerability scan over biological/defense node layers, 1 glowing amber dual-use science schematic splitting green/red, 2 razor-thin collinear emerald laser beam threading carefully to single target star, 3 blazing giant 360-degree omnidirectional beacon flooding entire starfield uncontrollably, 4 3 competing rogue dishes around Earth firing conflicting red/cyan/yellow beams upward, 5 lone giant privileged super-dish towering over darkened unconsulted continents.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **información sensible** / *sensitive information* | `[TIER-C]` | Biología, defensas, infraestructura o vulnerabilidades podrían tener usos imposibles de anticipar. // *Biology, defenses, infrastructure or vulnerabilities could have uses that are impossible to anticipate.* | `vid=220` ignores `scene=14` & `doc_feat=0` | translucent vulnerability scan over biological/defense node layers |
| `F1` | **conocimiento peligroso** / *dangerous knowledge* | `[TIER-C]` | Una idea científica puede ser valiosa y también permitir tecnologías destructivas en contextos diferentes. // *A scientific idea can be valuable and also enable destructive technologies in different contexts.* | `vid=220` ignores `scene=14` & `doc_feat=1` | glowing amber dual-use science schematic splitting green/red |
| `F2` | **señal dirigida** / *directed signal* | `[TIER-B]` | Un haz estrecho reduce exposición, pero necesita conocer con precisión la posición futura del receptor. // *A narrow beam reduces exposure, but requires precise knowledge of the receiver's future position.* | `vid=220` ignores `scene=14` & `doc_feat=2` | razor-thin collinear emerald laser beam threading carefully to single target star |
| `F3` | **difusión amplia** / *broad broadcast* | `[TIER-D]` | Una baliza para muchos sistemas maximiza descubrimiento y elimina casi todo control sobre la audiencia. // *One beacon for many systems maximizes discovery and eliminates almost all control over the audience.* | `vid=220` ignores `scene=14` & `doc_feat=3` | blazing giant 360-degree omnidirectional beacon flooding entire starfield uncontrollably |
| `F4` | **suplantación** / *impersonation* | `[TIER-D]` | Otros actores humanos podrían enviar mensajes falsos, contradictorios o diseñados para parecer oficiales. // *Other human actors could send messages that are false, contradictory, or designed to appear official.* | `vid=220` ignores `scene=14` & `doc_feat=4` | 3 competing rogue dishes around Earth firing conflicting red/cyan/yellow beams upward |
| `F5` | **desigualdad de voz** / *voice inequality* | `[TIER-C]` | Quienes controlen transmisores potentes podrían representar al planeta sin consentimiento de la mayoría. // *Those who control powerful transmitters could represent the planet without the consent of the majority.* | `vid=220` ignores `scene=14` & `doc_feat=5` | lone giant privileged super-dish towering over darkened unconsulted continents |

#### `Exp 22 Z19` `light-speed-limit` — Límites de la velocidad de la luz / *Limits imposed by the speed of light* (`[TIER-D]` · Target `vid=3219`)
- **Metric (`raw_metric`):** "Distancia en años luz × trayecto de ida y vuelta → ritmo mínimo del diálogo" (*Distance in light years × round trip → minimum dialogue pace*) · `scene=15` · `footer="EL MENSAJE DESCRIBE EL PASADO DEL EMISOR"` · `evidence_class`: "Física del canal · ninguna respuesta puede ser inmediata" (*Channel physics · no response can be immediate*) · `range=[0.679, 0.714]`
- **Spoken Macro (ES // EN):** Cada mensaje muestra el pasado del emisor; una conversación de ida y vuelta puede durar décadas, siglos o más. // *Each message shows the sender's past; a back-and-forth conversation can last decades, centuries, or more.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=4.75 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** COLLIDES on scene=15 with Z20 (`EL MENSAJE DESCRIBE EL PASADO DEL EMISOR`) and renders single static Earth dish (`220`). Completely misses every physical relativistic light-cone feature: light-year spatial ruler (`F00`), sender appearing as distant past ghost (`F01`), stellar proper motion requiring future target-lead aiming (`F02`), $1/r^2$ spherical beam divergence (`F03`), and ISM noise accumulation (`F04`).
- **Zone 3D Procedural Blueprint:** Relativistic Light-Cone & Target-Lead Simulator (vid=3219): wide stage anchored by two moving star nodes across void

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **años luz** / *light years* | `[TIER-D]` | Una señal tarda un año por cada año luz recorrido, incluso en el vacío ideal. // *A signal takes one year for every light year traveled, even in the ideal vacuum.* | `vid=220` ignores `scene=15` & `doc_feat=0` | concentric expanding pale-blue light-year wavefront shells stepping rhythmically outward at fixed c |
| `F1` | **pasado del emisor** / *sender's past* | `[TIER-D]` | El mensaje recibido describiría el pasado del emisor, no necesariamente su presente. // *The received message would describe the sender's past, not necessarily their present.* | `vid=220` ignores `scene=15` & `doc_feat=1` | pale ghostly historic star silhouette emitting beam while true star has already drifted onward |
| `F2` | **movimiento estelar** / *stellar motion* | `[TIER-D]` | Durante el viaje, ambos sistemas cambian de posición y el transmisor debe apuntar a un destino futuro. // *During the trip, both systems change position and the transmitter must point to a future destination.* | `vid=220` ignores `scene=15` & `doc_feat=2` | dotted predicted future motion vector showing dish aiming ahead of visible star position |
| `F3` | **dispersión del haz** / *beam spread* | `[TIER-D]` | Incluso una transmisión dirigida se abre con la distancia, reduciendo la energía recibida. // *Even a directed transmission opens with distance, reducing the energy received.* | `vid=220` ignores `scene=15` & `doc_feat=3` | narrow crisp initial laser flare spreading dramatically wider & dimmer across light-years (`1/r^2` gradient) |
| `F4` | **ruido acumulado** / *accumulated noise* | `[TIER-D]` | El medio, los receptores y fuentes de fondo degradan la señal y obligan a usar redundancia. // *The medium, receivers and background sources degrade the signal and force the use of redundancy.* | `vid=220` ignores `scene=15` & `doc_feat=4` | sparkling dust/plasma clumps speckling and broadening the traveling wave packet |
| `F5` | **sin conversación en vivo** / *no live conversation* | `[TIER-C]` | No habría turnos inmediatos, gestos compartidos ni aclaraciones rápidas como en una interacción presencial. // *There would be no immediate turns, shared gestures, or quick clarifications like in a face-to-face interaction.* | `vid=220` ignores `scene=15` & `doc_feat=5` | asynchronous long silent gap separating isolated light bullets |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP22-Z15-19)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 22 (22_cosmic_contact) Zones Z15..Z19 (`cultural-context`, `message-received`, `decision-to-respond`, `transmission-risks`, `light-speed-limit`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z15->3215`, `Z16->3216`, `Z17->3217`, `Z18->3218`, `Z19->3219` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Disambiguate colliding baked scene indices in Z00..Z27 by keying directly on doc.active_zone_index -> vid=3200..3227 and spawning companion stages 3830..3835 where dual actors/wavefronts apply.
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `seti` & `rosetta` & `primo` (exist), plus `radar`, `laser`, `radio`, `descifrado`, `silencio`, `meti`, `tratado`, `relativista`, `espejo`, `archivo`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_civilization_horizon.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z15->3215`, `Z16->3216`, `Z17->3217`, `Z18->3218`, `Z19->3219`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z15 (vid=3215, cultural-context): F0:individuality; F1:time; F2:senses; F3:memory; F4:values; F5:metaphor
     * Z16 (vid=3216, message-received): F0:environment description; F1:self portrait; F2:history; F3:scientific knowledge; F4:warning; F5:dubious identity
     * Z17 (vid=3217, decision-to-respond): F0:remain silent; F1:acknowledge receipt; F2:send science; F3:transmit culture; F4:reveal location; F5:wait for consensus
     * Z18 (vid=3218, transmission-risks): F0:sensitive information; F1:dangerous knowledge; F2:directed signal; F3:broad broadcast; F4:impersonation; F5:voice inequality
     * Z19 (vid=3219, light-speed-limit): F0:light years; F1:sender's past; F2:stellar motion; F3:beam spread; F4:accumulated noise; F5:no live conversation
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp22-z20-23"></a>
### PKG-ACT2-EXP22-Z20-23: Past-Ghost Senders, Multigenerational Relays & Epistemic Limits (`z=20..23`) — `Phase 1` · Priority: **Critical**

**Current State:** Zones `Z20..Z23` (`4` zones, `24` features) | Zone Tiers `0A / 0B / 3C / 1D` | Feature Tiers `0A / 0B / 13C / 11D` | Render path: [`doc_stages.rs#L79-L82`](../../crates/cosmos_render/src/doc_stages.rs#L79-L82) -> [`shaders/hero_doc_civilization_horizon.wgsl#L389-L429`](../../shaders/hero_doc_civilization_horizon.wgsl#L389-L429) — `z=12 -> vid=221`, all other `27` zones -> `vid=220` SETI dish (`5` shared `scene` pairs). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `24` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z20` | `interstellar-wait` | Espera interestelar / *Interstellar wait* | **`3220`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z21` | `first-reply` | Primera respuesta / *First answer* | **`3221`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z22` | `discontinuous-dialogue` | Diálogo discontinuo / *Discontinuous dialogue* | **`3222`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z23` | `knowledge-exchange` | Intercambio de conocimiento / *Knowledge exchange* | **`3223`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 22 Z20` `interstellar-wait` — Espera interestelar / *Interstellar wait* (`[TIER-C]` · Target `vid=3220`)
- **Metric (`raw_metric`):** "Archivo + instituciones + traducción + paciencia → memoria capaz de recibir la respuesta" (*Archive + institutions + translation + patience → memory capable of receiving the answer*) · `scene=15` · `footer="EL MENSAJE DESCRIBE EL PASADO DEL EMISOR"` · `evidence_class`: "Continuidad generacional · mantener una pregunta durante siglos" (*Generational continuity · maintaining a question for centuries*) · `range=[0.714, 0.750]`
- **Spoken Macro (ES // EN):** La civilización que envía una pregunta puede ser cultural y políticamente distinta cuando llegue la respuesta. // *The civilization that sends a question may be culturally and politically different when the answer arrives.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=5.00 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** COLLIDES on scene=15 with Z19; lone rotating SETI dish (`220`) fails to express century-scale multi-generational transmission continuity: passing questions across unborn generations (`F00`), durable archival media (`F01`), semantic language drift (`F02`), fluctuating political support (`F03`), extinct/migrated distant sender (`F04`), and ambiguous decades of silence (`F05`).
- **Zone 3D Procedural Blueprint:** Generational Archive Torch & Century Hourglass (vid=3220): translucent vertical timeline hourglass connecting ancient dish at bottom to futuristic observatory spire at top across rotating generations

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **generaciones** / *generations* | `[TIER-C]` | Quienes redactan el mensaje quizá nunca conozcan su resultado; otros heredarán la responsabilidad de interpretarlo. // *Those who compose the message may never know its outcome; others will inherit the responsibility of interpreting it.* | `vid=220` ignores `scene=15` & `doc_feat=0` | cascading luminous generational relay rings passing glowing question packet upward |
| `F1` | **archivos duraderos** / *durable files* | `[TIER-D]` | Formatos, claves y contexto deben sobrevivir cambios tecnológicos para que el mensaje futuro siga siendo legible. // *Formats, keys and context must survive technological changes so that the future message remains readable.* | `vid=220` ignores `scene=15` & `doc_feat=1` | sealed gold-diamond crystalline vault preserving raw codec stone-tables |
| `F2` | **lengua cambiante** / *changing language* | `[TIER-C]` | Las palabras humanas, instituciones y valores pueden transformarse antes de la siguiente ronda. // *Human words, institutions and values can be transformed before the next round.* | `vid=220` ignores `scene=15` & `doc_feat=2` | shifting morphing linguistic symbols evolving across century strata |
| `F3` | **continuidad política** / *political continuity* | `[TIER-C]` | Un acuerdo de respuesta puede perder apoyo, fragmentarse o ser reinterpretado por gobiernos posteriores. // *A response agreement may lose support, fragment or be reinterpreted by subsequent governments.* | `vid=220` ignores `scene=15` & `doc_feat=3` | flickering fluctuating political support halo around base station |
| `F4` | **emisor desaparecido** / *vanished sender* | `[TIER-D]` | La fuente podría haber cambiado, migrado o dejado de existir antes de recibir nuestro mensaje. // *The source could have changed, migrated, or ceased to exist before our message reached it.* | `vid=220` ignores `scene=15` & `doc_feat=4` | distant target star quietly cooling into extinguished dark ember mid-transit while light beam still travels |
| `F5` | **paciencia científica** / *scientific patience* | `[TIER-C]` | Décadas sin respuesta no distinguen entre silencio deliberado, fallo técnico, distancia mayor o ausencia del receptor. // *Decades without a response cannot distinguish among deliberate silence, technical failure, greater distance, or the receiver's absence.* | `vid=220` ignores `scene=15` & `doc_feat=5` | four equal ambiguous question branches suspended over silent void |

#### `Exp 22 Z21` `first-reply` — Primera respuesta / *First answer* (`[TIER-D]` · Target `vid=3221`)
- **Metric (`raw_metric`):** "Referencia compartida + eco transformado + nueva información → comunicación bidireccional" (*Shared reference + transformed echo + new information → two-way communication*) · `scene=16` · `footer="RECEPCIÓN DEMOSTRADA · COMPRENSIÓN TODAVÍA PARCIAL"` · `evidence_class`: "Reconocimiento mutuo · prueba inicial de comprensión" (*Mutual recognition · initial understanding test*) · `range=[0.750, 0.786]`
- **Spoken Macro (ES // EN):** La primera respuesta convincente probablemente demostraría recepción y comprensión parcial, no amistad ni acuerdo. // *The first convincing response would probably demonstrate partial reception and understanding, not friendship or agreement.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=5.25 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Shows only a one-way lone Earth dish (`220`) firing upward, with zero return wave, zero distant responder star, and zero signal transformation (`echo verified` F00, `predictive checksum` F01, `binary Q&A` F02, `parity error correction` F03, `newly coined symbol` F04).
- **Zone 3D Procedural Blueprint:** Two-Node Bidirectional Ping & Echo Transform Stage (vid=3221): twin distant planetary receiver-transmitters exchanging wave packets

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **eco verificado** / *echo verified* | `[TIER-D]` | Devolver una parte transformada del mensaje puede demostrar que fue recibido y procesado, no solo reflejado. // *Returning a transformed part of the message can demonstrate that it was received and processed, not just reflected.* | `vid=220` ignores `scene=16` & `doc_feat=0` | Earth packet arriving -> node transforms geometric motif (e.g., primes squared or reversed) and fires bright cyan echo back proving cognition over reflection |
| `F1` | **suma de referencia** / *reference sum* | `[TIER-D]` | Añadir un dato predecible, como una posición futura, permite comprobar una operación compartida. // *Adding a predictable piece of information, such as a future position, allows you to verify a shared operation.* | `vid=220` ignores `scene=16` & `doc_feat=1` | target fires predicted future planetary position dot snapping cleanly onto true orbital ephemeris |
| `F2` | **pregunta simple** / *simple question* | `[TIER-D]` | Una elección binaria o cantidad concreta reduce ambigüedad y prueba si ambos lados usan el mismo marco. // *A binary choice or concrete quantity reduces ambiguity and tests whether both sides use the same framework.* | `vid=220` ignores `scene=16` & `doc_feat=2` | sharp binary 0/1 illuminated switch pulse |
| `F3` | **corrección** / *correction* | `[TIER-D]` | Señalar un error o repetir una sección puede revelar que el canal admite reparación. // *Pointing out an error or repeating a section may reveal that the channel is repairable.* | `vid=220` ignores `scene=16` & `doc_feat=3` | target highlights damaged red sector of Earth packet and prompts clean retransmission patch |
| `F4` | **nuevo símbolo** / *new symbol* | `[TIER-C]` | La respuesta puede introducir una convención y mostrarla mediante ejemplos antes de usarla en conceptos mayores. // *The answer can introduce a convention and show it through examples before using it in larger concepts.* | `vid=220` ignores `scene=16` & `doc_feat=4` | crisp alien geometric token flanked by 3 operational definition examples |
| `F5` | **ninguna respuesta** / *no response* | `[TIER-C]` | El silencio posterior seguiría siendo ambiguo: recepción fallida, decisión política, demora o desaparición. // *The subsequent silence would remain ambiguous: failed reception, political decision, delay or disappearance.* | `vid=220` ignores `scene=16` & `doc_feat=5` | incoming return path fading dark into 4 open uncertainty echoes |

#### `Exp 22 Z22` `discontinuous-dialogue` — Diálogo discontinuo / *Discontinuous dialogue* (`[TIER-C]` · Target `vid=3222`)
- **Metric (`raw_metric`):** "Mensajes autosuficientes + versiones + contexto acumulado → conversación entre generaciones" (*Self-contained messages + versions + accumulated context → conversation between generations*) · `scene=17` · `footer="CADA TURNO PUEDE PERTENECER A OTRA GENERACIÓN"` · `evidence_class`: "Comunicación asíncrona · cada turno necesita sostenerse por sí solo" (*Asynchronous communication · each turn must stand on its own*) · `range=[0.786, 0.821]`
- **Spoken Macro (ES // EN):** Cada transmisión tendría que anticipar preguntas, explicar su contexto y permitir correcciones que quizá lleguen un siglo después. // *Each transmission would have to anticipate questions, explain its context, and allow for corrections that may arrive a century later.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=5.50 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Generic SETI dish (`220`) gives no visual expression to asynchronous versioned self-contained discourse (`thread IDs`, `simple primer turns`, `pulsar/supernova astronomical sync clocks`, `model correction`, `generational ambiguity error propagation`, `deep philosophical layers`).
- **Zone 3D Procedural Blueprint:** Self-Contained Asynchronous Dialogue Capsules (vid=3222): train of glowing modular multi-ringed telemetry capsules cruising bidirectionally along light-bridge

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **reconocimiento** / *recognition* | `[TIER-C]` | Ambas partes mantienen identificadores y referencias que enlazan cada mensaje con los anteriores. // *Both parties maintain identifiers and references that link each message to previous ones.* | `vid=220` ignores `scene=17` & `doc_feat=0` | crisp glowing thread-ID header linking Capsule #1 -> #2 -> #3 |
| `F1` | **preguntas simples** / *simple questions* | `[TIER-C]` | Los primeros turnos reducen ambigüedad antes de intentar filosofía, historia o tecnología. // *The first few turns reduce ambiguity before attempting philosophy, history, or technology.* | `vid=220` ignores `scene=17` & `doc_feat=1` | compact inner elementary geometry kernel leading outward layers |
| `F2` | **referencias compartidas** / *shared references* | `[TIER-D]` | Eventos astronómicos y constantes físicas sirven como puntos de sincronización. // *Astronomical events and physical constants serve as synchronization points.* | `vid=220` ignores `scene=17` & `doc_feat=2` | distant pulsing Crab-like pulsar/supernova clock beacon synchronizing both ends simultaneously |
| `F3` | **intentos de traducción** / *translation attempts* | `[TIER-C]` | Cada lado propone modelos y ejemplos para que el otro confirme o corrija interpretaciones. // *Each side proposes models and examples for the other to confirm or correct interpretations.* | `vid=220` ignores `scene=17` & `doc_feat=3` | split A/B tentative model lattice swapping validation checkmarks |
| `F4` | **ambigüedad** / *ambiguity* | `[TIER-C]` | Una pregunta mal entendida puede contaminar varias generaciones de mensajes antes de detectarse. // *A misunderstood question can contaminate several generations of messages before it is detected.* | `vid=220` ignores `scene=17` & `doc_feat=4` | cascading red misunderstanding wobble propagating down three successive capsules before correction patch arrives |
| `F5` | **conceptos profundos** / *deep concepts* | `[TIER-C]` | Solo tras construir suficiente base podrían abordarse conciencia, ética, arte o identidad. // *Only after building enough foundation could consciousness, ethics, art or identity be addressed.* | `vid=220` ignores `scene=17` & `doc_feat=5` | deep luminous philosophical polyhedron nested cleanly inside outer foundational shell |

#### `Exp 22 Z23` `knowledge-exchange` — Intercambio de conocimiento / *Knowledge exchange* (`[TIER-C]` · Target `vid=3223`)
- **Metric (`raw_metric`):** "Predicción comprobable + comparación independiente + límites de uso → conocimiento compartido" (*Testable prediction + independent comparison + usage limits → shared knowledge*) · `scene=18` · `footer="CONOCIMIENTO AJENO · AUTORIDAD NO AUTOMÁTICA"` · `evidence_class`: "Aprendizaje mutuo · información valiosa, falible y contextual" (*Mutual learning · valuable, fallible and contextual information*) · `range=[0.821, 0.857]`
- **Spoken Macro (ES // EN):** La información alienígena no sería automáticamente verdadera, segura ni aplicable a nuestras condiciones. // *Information from an extraterrestrial civilization would not automatically be true, safe, or applicable to our conditions.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=5.75 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Generic SETI dish (`220`) fails to express cross-civilization scientific/cultural exchange and healthy epistemic skepticism (`off-Earth galactic vantage astronomy` F00, `testable vs authority physics` F01, `exotic high-pressure chemistry` F02, `comparative second-genesis biology` F03, `incomplete dangerous blueprints` F04, `untranslatable alien art/warnings` F05).
- **Zone 3D Procedural Blueprint:** Comparative Two-Biosphere Holographic Exchange Table (vid=3223): twin glowing analytical holo-frames bridging alien beacon and Earth lab

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **astronomía** / *astronomy* | `[TIER-D]` | Observaciones desde otra región de la galaxia podrían revelar perspectivas imposibles desde la Tierra. // *Observations from another region of the galaxy could reveal perspectives impossible from Earth.* | `vid=220` ignores `scene=18` & `doc_feat=0` | parallax dual-vantage 3D view of Milky Way core from opposite spiral arm |
| `F1` | **física** / *physics* | `[TIER-C]` | Una teoría nueva tendría que producir predicciones verificables, no aceptarse por autoridad del emisor. // *A new theory would have to produce testable predictions rather than be accepted solely on the sender's authority.* | `vid=220` ignores `scene=18` & `doc_feat=1` | alien mathematical curve tested directly against glowing terrestrial particle-physics verification ring |
| `F2` | **química** / *chemistry* | `[TIER-D]` | Rutas y materiales desconocidos podrían depender de recursos o presiones que no existen aquí. // *Unknown processes and materials could depend on resources or pressures that do not exist here.* | `vid=220` ignores `scene=18` & `doc_feat=2` | exotic crystal/supercritical-fluid molecular reactor diagram |
| `F3` | **biología** / *biology* | `[TIER-D]` | Comparar dos historias de vida transformaría la evolución, pero requeriría evitar categorías exclusivamente terrestres. // *Comparing two life histories would transform evolution, but would require avoiding exclusively terrestrial categories.* | `vid=220` ignores `scene=18` & `doc_feat=3` | twin parallel double-helix vs alien triple-ring genetic evolution tree side-by-side |
| `F4` | **tecnología** / *technology* | `[TIER-D]` | Una descripción incompleta puede ser inútil o peligrosa si omite infraestructura, materiales o controles culturales. // *An incomplete description can be useless or dangerous if it omits infrastructure, materials, or cultural controls.* | `vid=220` ignores `scene=18` & `doc_feat=4` | glowing incomplete blueprint missing highlighted safety/material foundation blocks turning amber warning |
| `F5` | **arte y advertencias** / *art and warnings* | `[TIER-C]` | Estructuras estéticas o relatos de fracaso podrían ser profundamente significativos y extremadamente difíciles de traducir. // *Aesthetic structures or narratives of failure could be deeply meaningful and extremely difficult to translate.* | `vid=220` ignores `scene=18` & `doc_feat=5` | swirling hyper-complex non-representational alien aesthetic sculpture/cautionary myth veil |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP22-Z20-23)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 22 (22_cosmic_contact) Zones Z20..Z23 (`interstellar-wait`, `first-reply`, `discontinuous-dialogue`, `knowledge-exchange`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z20->3220`, `Z21->3221`, `Z22->3222`, `Z23->3223` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Disambiguate colliding baked scene indices in Z00..Z27 by keying directly on doc.active_zone_index -> vid=3200..3227 and spawning companion stages 3830..3835 where dual actors/wavefronts apply.
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `seti` & `rosetta` & `primo` (exist), plus `radar`, `laser`, `radio`, `descifrado`, `silencio`, `meti`, `tratado`, `relativista`, `espejo`, `archivo`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_civilization_horizon.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z20->3220`, `Z21->3221`, `Z22->3222`, `Z23->3223`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z20 (vid=3220, interstellar-wait): F0:generations; F1:durable files; F2:changing language; F3:political continuity; F4:vanished sender; F5:scientific patience
     * Z21 (vid=3221, first-reply): F0:echo verified; F1:reference sum; F2:simple question; F3:correction; F4:new symbol; F5:no response
     * Z22 (vid=3222, discontinuous-dialogue): F0:recognition; F1:simple questions; F2:shared references; F3:translation attempts; F4:ambiguity; F5:deep concepts
     * Z23 (vid=3223, knowledge-exchange): F0:astronomy; F1:physics; F2:chemistry; F3:biology; F4:technology; F5:art and warnings
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp22-z24-27"></a>
### PKG-ACT2-EXP22-Z24-27: Diplomatic Protocols, Cultural Transformation & Cosmic Mirror (`z=24..27`) — `Phase 1` · Priority: **Critical**

**Current State:** Zones `Z24..Z27` (`4` zones, `24` features) | Zone Tiers `0A / 0B / 4C / 0D` | Feature Tiers `0A / 0B / 22C / 2D` | Render path: [`doc_stages.rs#L79-L82`](../../crates/cosmos_render/src/doc_stages.rs#L79-L82) -> [`shaders/hero_doc_civilization_horizon.wgsl#L389-L429`](../../shaders/hero_doc_civilization_horizon.wgsl#L389-L429) — `z=12 -> vid=221`, all other `27` zones -> `vid=220` SETI dish (`5` shared `scene` pairs). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `24` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z24` | `cosmic-misunderstandings` | Malentendidos cósmicos / *Cosmic misunderstandings* | **`3224`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z25` | `trust` | Confianza / *Trust* | **`3225`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z26` | `interstellar-diplomacy` | Diplomacia interestelar / *Interstellar diplomacy* | **`3226`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z27` | `uncertain-coexistence` | Coexistencia incierta / *Uncertain coexistence* | **`3227`** | `3830..3835` dual-node/waterfall/Earth-net/Rosetta/cone (`submode=0..4`) | Exp `22` compute `13` + `cs_field_grid` cues | `doc_feat 0..=5` (6 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 22 Z24` `cosmic-misunderstandings` — Malentendidos cósmicos / *Cosmic misunderstandings* (`[TIER-C]` · Target `vid=3224`)
- **Metric (`raw_metric`):** "Supuestos ocultos + contexto perdido + demora → errores persistentes" (*Hidden assumptions + lost context + delay → persistent errors*) · `scene=19` · `footer="DATOS ENTENDIDOS · INTENCIÓN TODAVÍA AMBIGUA"` · `evidence_class`: "Fallo de interpretación · comprender datos no garantiza comprender intención" (*Misinterpretation · understanding data does not guarantee understanding intent*) · `range=[0.857, 0.893]`
- **Spoken Macro (ES // EN):** Un error pequeño puede tardar generaciones en corregirse y adquirir consecuencias políticas antes de ser reconocido. // *A small error may take generations to correct and may have political consequences before it is recognized.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=6.00 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Generic SETI dish (`220`) does not communicate semantic, temporal, biological, packet-loss, or deceptive translation hazards (`incompatible sense metaphors`, `hive vs individual pronouns`, `ms vs century promises`, `literal metaphor alarm`, `missing negation bits`, `strategic deception`).
- **Zone 3D Procedural Blueprint:** Semantic Drift & Signal-Corruption Prism (vid=3224): identical geometric signal entering twin mismatched cultural lenses that bend and distort it differently

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **sentidos incompatibles** / *incompatible senses* | `[TIER-C]` | Una descripción basada en percepciones desconocidas puede traducirse como objeto cuando realmente expresa una experiencia. // *A description based on unknown perceptions can be translated as an object when it really expresses an experience.* | `vid=220` ignores `scene=19` & `doc_feat=0` | alien wave mapped wrongly into jagged rigid terrestrial box icon |
| `F1` | **individuo distinto** / *different individual* | `[TIER-C]` | Pronombres o agentes pueden referirse a colonias, linajes o redes, alterando responsabilidad y consentimiento. // *Pronouns or agents can refer to colonies, lineages or networks, altering responsibility and consent.* | `vid=220` ignores `scene=19` & `doc_feat=1` | single glowing entity splitting ambiguously into networked swarm vs singular crown |
| `F2` | **escalas temporales** / *time scales* | `[TIER-C]` | Una promesa de respuesta pronta puede significar segundos para una mente y siglos para otra. // *A promise of a prompt response can mean seconds to one mind and centuries to another.* | `vid=220` ignores `scene=19` & `doc_feat=2` | overlapping fast hummingbird vs glacial geological clocks misaligning 'soon' markers |
| `F3` | **metáfora literal** / *literal metaphor* | `[TIER-C]` | Una imagen cultural puede parecer instrucción física o amenaza cuando se interpreta fuera de su tradición. // *A cultural image can seem like physical instruction or threat when interpreted outside of its tradition.* | `vid=220` ignores `scene=19` & `doc_feat=3` | poetic wave symbol sparking alarming literal weapon-vector mirage |
| `F4` | **mensaje incompleto** / *incomplete message* | `[TIER-D]` | La pérdida de paquetes puede borrar negaciones, condiciones o claves esenciales sin que el receptor lo note. // *Packet loss can erase essential negations, conditions, or keys without the receiver noticing.* | `vid=220` ignores `scene=19` & `doc_feat=4` | dark dropout hole chewing out crucial 'NOT' inversion gate flipping message polarity green<->red |
| `F5` | **engaño deliberado** / *deliberate deception* | `[TIER-C]` | Una fuente puede ocultar intenciones, exagerar capacidades o probar nuestra conducta mediante información falsa. // *A source may hide its intentions, exaggerate its capabilities, or test our behavior with false information.* | `vid=220` ignores `scene=19` & `doc_feat=5` | shimmering Trojan hologram concealing masked inner geometry |

#### `Exp 22 Z25` `trust` — Confianza / *Trust* (`[TIER-C]` · Target `vid=3225`)
- **Metric (`raw_metric`):** "Consistencia + reciprocidad + límites respetados + correcciones → confianza gradual" (*Consistency + reciprocity + respected boundaries + corrections → gradual trust*) · `scene=20` · `footer="CONFIANZA PARCIAL · VERIFICACIÓN CONTINUA"` · `evidence_class`: "Relación verificable · cooperación sin certeza total" (*Verifiable relationship · cooperation without total certainty*) · `range=[0.893, 0.929]`
- **Spoken Macro (ES // EN):** La confianza interestelar tendría que apoyarse en comportamientos observables durante largos periodos, no en una sola declaración. // *Interstellar trust would have to be based on observable behavior over long periods, not a single statement.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=6.25 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Generic SETI dish (`220`) fails to visualize long-horizon trust-building mechanics (`verified observational predictions`, `symmetric reciprocity`, `honest error correction`, `bounded opaque privacy domains`, `explained planned silences`, `partial pragmatic scientific trust amidst geopolitical caution`).
- **Zone 3D Procedural Blueprint:** Reciprocal Verification Trust Lattice (vid=3225): twin glowing spheres building slender luminous crystal bridge strands over successive verified cycles

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **predicciones cumplidas** / *successful predictions* | `[TIER-C]` | Información que permite anticipar observaciones fortalece la fiabilidad del canal y de parte del contenido. // *Information that allows anticipating observations strengthens the reliability of the channel and part of the content.* | `vid=220` ignores `scene=20` & `doc_feat=0` | alien forecast ring matching independently observed supernova/comet flash snapping bright gold link |
| `F1` | **reciprocidad** / *reciprocity* | `[TIER-C]` | Compartir riesgos y beneficios de forma equilibrada puede indicar cooperación, aunque medir equilibrio sea difícil. // *Sharing risks and benefits in a balanced way can indicate cooperation, although measuring balance is difficult.* | `vid=220` ignores `scene=20` & `doc_feat=1` | balanced dual-pan flux arc showing symmetric risk/info flow |
| `F2` | **correcciones honestas** / *honest corrections* | `[TIER-C]` | Reconocer errores ayuda a distinguir aprendizaje conjunto de propaganda infalible. // *Recognizing mistakes helps distinguish joint learning from infallible propaganda.* | `vid=220` ignores `scene=20` & `doc_feat=2` | glowing self-issued red-to-amber correction patch strengthening bridge truss |
| `F3` | **límites informativos** / *informational limits* | `[TIER-C]` | Respetar temas que una parte decide no revelar muestra que la cooperación no exige transparencia total. // *Respecting issues that one party chooses not to reveal shows that cooperation does not require complete transparency.* | `vid=220` ignores `scene=20` & `doc_feat=3` | opaque shielded privacy core cleanly accepted inside cooperative outer shell |
| `F4` | **silencio explicado** / *explained silence* | `[TIER-C]` | Pausas acompañadas de contexto reducen la tendencia a interpretar ausencia como hostilidad. // *Pauses accompanied by context reduce the tendency to interpret absence as hostility.* | `vid=220` ignores `scene=20` & `doc_feat=4` | pre-announced calm countdown window darkening link predictably without alarm |
| `F5` | **confianza parcial** / *partial trust* | `[TIER-C]` | Puede existir cooperación científica mientras persisten dudas sobre política, identidad o intenciones a largo plazo. // *Scientific cooperation can exist while doubts persist about policy, identity or long-term intentions.* | `vid=220` ignores `scene=20` & `doc_feat=5` | bright blue scientific truss thriving peacefully alongside cautious translucent amber political barrier |

#### `Exp 22 Z26` `interstellar-diplomacy` — Diplomacia interestelar / *Interstellar diplomacy* (`[TIER-C]` · Target `vid=3226`)
- **Metric (`raw_metric`):** "Legitimidad + verificación + reciprocidad + memoria → acuerdos de larga duración" (*Legitimacy + verification + reciprocity + memory → long-term agreements*) · `scene=21` · `footer="PROCEDIMIENTOS PARA CONVIVIR CON EL DESACUERDO"` · `evidence_class`: "Relación política · representación, intereses y protocolos a gran distancia" (*Political relationship · representation, interests and long-distance protocols*) · `range=[0.929, 0.964]`
- **Spoken Macro (ES // EN):** La diplomacia no eliminaría diferencias; crearía procedimientos para convivir con ellas sin interpretación inmediata ni autoridad común. // *Diplomacy would not eliminate differences; it would create procedures for living with them without immediate interpretation or a common authority.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=6.50 rad, `odd_zone=0`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Generic SETI dish (`220`) conveys zero diplomatic/protocol architecture (`speaker attribution headers`, `authentication & correction protocols`, `divergent resource/expansion interests`, `non-interference protective boundaries`, `fragile cultural preservation`, `generational treaty persistence`).
- **Zone 3D Procedural Blueprint:** Interstellar Protocol & Non-Interference Seal (vid=3226): interlocking concentric cryptographic and diplomatic ring seal rotating smoothly between dual worlds

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **representación** / *representation* | `[TIER-C]` | Cada mensaje debe aclarar quién habla, a quién compromete y qué grupos quedan fuera. // *Each message must clarify who is speaking, who is involved, and which groups are left out.* | `vid=220` ignores `scene=21` & `doc_feat=0` | crisp illuminated origin signature header isolating speaking faction |
| `F1` | **protocolos** / *protocols* | `[TIER-C]` | Formatos, tiempos, autenticación y procedimientos de corrección reducen ambigüedad antes de discutir contenido. // *Formats, timing, authentication, and correction procedures reduce ambiguity before the content is discussed.* | `vid=220` ignores `scene=21` & `doc_feat=1` | synchronized framing/ACK/signature ring teeth locking cleanly |
| `F2` | **intereses distintos** / *different interests* | `[TIER-C]` | Cooperación científica puede coexistir con desacuerdos sobre expansión, recursos o contacto con terceros. // *Scientific cooperation can coexist with disagreements about expansion, resources or contact with third parties.* | `vid=220` ignores `scene=21` & `doc_feat=2` | dual science bridge spanning smoothly across diverging political vectors |
| `F3` | **no interferencia** / *non-interference* | `[TIER-C]` | Limitar información o acciones puede proteger autonomía cultural, aunque definir interferencia sea difícil. // *Limiting information or actions can protect cultural autonomy, although defining interference is difficult.* | `vid=220` ignores `scene=21` & `doc_feat=3` | shimmering protective non-interference bubble enclosing fragile young world cleanly outside channel |
| `F4` | **preservación cultural** / *cultural preservation* | `[TIER-C]` | El intercambio puede enriquecer y también desplazar conocimientos, lenguas o instituciones más frágiles. // *Exchange can enrich and also displace more fragile knowledge, languages or institutions.* | `vid=220` ignores `scene=21` & `doc_feat=4` | delicate iridescent cultural lattice buffered from overwhelming bright high-power beam |
| `F5` | **acuerdos generacionales** / *generational agreements* | `[TIER-C]` | Los compromisos deben poder revisarse sin que cada cambio político destruya décadas de confianza. // *Commitments must be able to be reviewed without each political change destroying decades of trust.* | `vid=220` ignores `scene=21` & `doc_feat=5` | durable deep-anchor ring surviving flickering foreground political regime pulses |

#### `Exp 22 Z27` `uncertain-coexistence` — Coexistencia incierta / *Uncertain coexistence* (`[TIER-C]` · Target `vid=3227`)
- **Metric (`raw_metric`):** "Mensajes + memoria + transformación mutua + desacuerdo → historia cósmica compartida" (*Messages + memory + mutual transformation + disagreement → shared cosmic history*) · `scene=22` · `footer="MÁS ALLÁ: PERSPECTIVAS CÓSMICAS"` · `evidence_class`: "Relación abierta · conocimiento compartido, diferencias persistentes" (*Open relationship · shared knowledge, persistent differences*) · `range=[0.964, 1.000]`
- **Spoken Macro (ES // EN):** El contacto maduro no sería unidad perfecta, sino la capacidad de reconocer otra perspectiva y sostener una relación incompleta. // *Mature contact would not mean perfect unity, but the ability to recognize another perspective and sustain an incomplete relationship.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** _ => vid=220 (`mode=0`: Parabolic SETI dish + tripod feed + upward laser pulse; yaw=`zone*0.25`=6.75 rad, `odd_zone=1`; `doc_feat` & `doc_scene` unused)
- **Audit Gap:** Final closing zone of Exp 22 (`Uncertain coexistence`: signal threads across immense cosmic darkness, guardian generations, sharpening partial clarity amidst enduring plural disagreement, reciprocal civilizational evolution, recognizing another mind's reality). Ends still looking at a single yawed Parabolic SETI dish (`220`) instead of twin distinct celestial worlds/perspectives!
- **Zone 3D Procedural Blueprint:** Dual-Perspective Cosmic Coexistence Finale (vid=3227): step camera back cleanly into serene deep starfield showing TWO distinctly colored luminous world-spheres separated by light-years, linked by fine shimmering bidirectional light threads

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **mensajes en la oscuridad** / *messages in the dark* | `[TIER-D]` | La relación existiría como señales que cruzan espacios donde ninguna parte puede observar directamente la vida cotidiana de la otra. // *The relationship would exist through signals crossing spaces where neither party can directly observe the other's daily life.* | `vid=220` ignores `scene=22` & `doc_feat=0` | slender golden/cyan starlight signal arcs bridging vast dark space without domestic windows |
| `F1` | **generaciones guardianas** / *guardian generations* | `[TIER-C]` | Personas, máquinas e instituciones mantendrían archivos y protocolos que comenzaron antes de su nacimiento. // *People, machines, and institutions would maintain archives and protocols established before their birth.* | `vid=220` ignores `scene=22` & `doc_feat=1` | steady glowing sentinel ring tending ancient beam across rotating generations |
| `F2` | **significados más claros** / *clearer meanings* | `[TIER-C]` | La comprensión crecería mediante correcciones, ejemplos y paciencia, sin volverse nunca total. // *Understanding would grow through corrections, examples, and patience, without ever becoming total.* | `vid=220` ignores `scene=22` & `doc_feat=2` | blurred fuzzy halo glyphs slowly sharpening clear edges while keeping soft outer mystery |
| `F3` | **desacuerdos duraderos** / *lasting disagreements* | `[TIER-C]` | Dos civilizaciones pueden comprenderse mejor y seguir rechazando las prioridades de la otra. // *Two civilizations can understand each other better and still reject each other's priorities.* | `vid=220` ignores `scene=22` & `doc_feat=3` | two harmonic non-identical geometric polyhedra rotating peacefully side-by-side without merging |
| `F4` | **historias transformadas** / *transformed stories* | `[TIER-C]` | El contacto se convertiría en parte de la ciencia, cultura e identidad de ambos mundos. // *Contact would become part of the science, culture and identity of both worlds.* | `vid=220` ignores `scene=22` & `doc_feat=4` | subtle mutual hue shift rippling across both worlds' starlight aura |
| `F5` | **perspectiva no terrestre** / *non-terrestrial perspective* | `[TIER-C]` | El verdadero contacto no ocurriría cuando detectáramos una señal, sino cuando dos civilizaciones lograran reconocer que detrás de sus mensajes existe otra perspectiva sobre la realidad. // *True contact would occur not merely when we detected a signal, but when two civilizations recognized another perspective on reality behind each other's messages.* | `vid=220` ignores `scene=22` & `doc_feat=5` | wide panoramic pull-back seeing reality reflected differently through both glowing minds simultaneously |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP22-Z24-27)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 22 (22_cosmic_contact) Zones Z24..Z27 (`cosmic-misunderstandings`, `trust`, `interstellar-diplomacy`, `uncertain-coexistence`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z24->3224`, `Z25->3225`, `Z26->3226`, `Z27->3227` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Disambiguate colliding baked scene indices in Z00..Z27 by keying directly on doc.active_zone_index -> vid=3200..3227 and spawning companion stages 3830..3835 where dual actors/wavefronts apply.
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `seti` & `rosetta` & `primo` (exist), plus `radar`, `laser`, `radio`, `descifrado`, `silencio`, `meti`, `tratado`, `relativista`, `espejo`, `archivo`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_civilization_horizon.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z24->3224`, `Z25->3225`, `Z26->3226`, `Z27->3227`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z24 (vid=3224, cosmic-misunderstandings): F0:incompatible senses; F1:different individual; F2:time scales; F3:literal metaphor; F4:incomplete message; F5:deliberate deception
     * Z25 (vid=3225, trust): F0:successful predictions; F1:reciprocity; F2:honest corrections; F3:informational limits; F4:explained silence; F5:partial trust
     * Z26 (vid=3226, interstellar-diplomacy): F0:representation; F1:protocols; F2:different interests; F3:non-interference; F4:cultural preservation; F5:generational agreements
     * Z27 (vid=3227, uncertain-coexistence): F0:messages in the dark; F1:guardian generations; F2:clearer meanings; F3:lasting disagreements; F4:transformed stories; F5:non-terrestrial perspective
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

---

### Exp 06: `06_local_group` — Grupo Local / Local Group (`9` Zones, `45` Features, `9` Landmarks)

> *Espectro del Grupo Local: de la Vía Láctea a Andrómeda* — Spectrum of the Local Group: from the Milky Way to Andromeda

- **Odyssey Position (`odyssey.rs#L29-L32`):** Act 2, Position **4 / 8** (Chapter `act2_exp06`)
- **Spatial & Scale Band:** `SectorId::LOCAL_GROUP` · `u ∈ [21.1, 22.6]` · `focus_scale_exp = 22.0` (`topology.rs:141`)
- **Compute & Far Shell:** `mode = 0` (spins ONLY single Milky Way spiral — does not include Andromeda/Triangulum co-orbit!, `universe_scene.rs:307`) · `mw_alpha` retains galactic band glow (~0.72) at `u=22.0` (`far_shell.wgsl:82-103`)
- **Current Render Dispatch:** Both `ExperienceAnchor` (`doc_stages.rs:696-708`) and all `9` `GalacticLandmark` entities (`doc_stages.rs:363`) emit identical `kind=3, vid=6` (`hero_doc_cosmos.wgsl:527-542`: 3 static spirals — MW, M31, M33 — with unimplemented `+ Magellanic satellite smudges` comment at `L528` and ZERO `doc_zone`/`doc_feat` reads).
- **Granular Audit Tally:** Zones `0A / 3B / 0C / 6D` (`9`) · Features `0A / 9B / 0C / 36D` (`45`)
- **`DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`) Needed Additions:** `andromeda` (exists), plus `magallanes`, `triangulo`, `enana`, `satelite`, `halo`, `intergalactico`, `colision`, `mareas`

#### Baked Landmarks Cross-Reference (`universe_catalog.v1.json` -> Companion IDs `3680..=3688`)

| `lm` | Landmark `id` | `zone_index` | `p` · `lane` · `size` | Current Emitted `vid` (`doc_stages.rs`) | Reserved Companion `vid` |
|---:|---|---:|---|---|---|
| `LM00` | `milky-edge` (`galactic-edge`) | `0` | `p=0.035`, `lane=-0.04`, `size=170` | `6` (static Exp 06 clone) | `3680` (`vid=1600` synced) |
| `LM01` | `outer-halo` (`outer-halo`) | `1` | `p=0.125`, `lane=0.05`, `size=118` | `6` (static Exp 06 clone) | `3681` (`vid=1601` synced) |
| `LM02` | `dwarfs` (`dwarf-galaxies`) | `2` | `p=0.24`, `lane=-0.03`, `size=84` | `6` (static Exp 06 clone) | `3682` (`vid=1602` synced) |
| `LM03` | `intergalactic` (`intergalactic-space`) | `3` | `p=0.36`, `lane=0.06`, `size=104` | `6` (static Exp 06 clone) | `3683` (`vid=1603` synced) |
| `LM04` | `magellanic` (`magellanic-clouds`) | `4` | `p=0.49`, `lane=-0.07`, `size=94` | `6` (static Exp 06 clone) | `3684` (`vid=1604` synced) |
| `LM05` | `triangulum` (`triangulum`) | `5` | `p=0.605`, `lane=0.15`, `size=68` | `6` (static Exp 06 clone) | `3685` (`vid=1605` synced) |
| `LM06` | `andromeda` (`andromeda`) | `6` | `p=0.745`, `lane=-0.02`, `size=168` | `6` (static Exp 06 clone) | `3686` (`vid=1606` synced) |
| `LM07` | `center` (`local-group-center`) | `7` | `p=0.868`, `lane=0.09`, `size=86` | `6` (static Exp 06 clone) | `3687` (`vid=1607` synced) |
| `LM08` | `boundary` (`local-group-limit`) | `8` | `p=0.965`, `lane=0`, `size=150` | `6` (static Exp 06 clone) | `3688` (`vid=1608` synced) |

<a id="pkg-act2-exp06-z00-04"></a>
### PKG-ACT2-EXP06-Z00-04: Galactic Edge, Outer Halo, Dwarfs & Magellanic Clouds (`z=00..04`) — `Phase 3` · Priority: **High**

**Current State:** Zones `Z00..Z04` (`5` zones, `25` features) | Zone Tiers `0A / 1B / 0C / 4D` | Feature Tiers `0A / 3B / 0C / 22D` | Render path: [`doc_stages.rs#L363`](../../crates/cosmos_render/src/doc_stages.rs#L363) -> [`shaders/hero_doc_cosmos.wgsl#L527-L542`](../../shaders/hero_doc_cosmos.wgsl#L527-L542) — Anchor & all `9` landmarks emit identical `kind=3, vid=6` static spiral diorama. `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `25` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z00` | `galactic-edge` | Borde galáctico / *galactic edge* | **`1600`** | `3680` (`LM00` `milky-edge`), `submode=0..1` | Exp `06` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z01` | `outer-halo` | Halo exterior / *Outer halo* | **`1601`** | `3681` (`LM01` `outer-halo`), `submode=0..1` | Exp `06` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z02` | `dwarf-galaxies` | Galaxias enanas / *dwarf galaxies* | **`1602`** | `3682` (`LM02` `dwarfs`), `submode=0..1` | Exp `06` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z03` | `intergalactic-space` | Espacio intergaláctico / *intergalactic space* | **`1603`** | `3683` (`LM03` `intergalactic`), `submode=0..1` | Exp `06` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z04` | `magellanic-clouds` | Nubes de Magallanes / *Magellanic Clouds* | **`1604`** | `3684` (`LM04` `magellanic`), `submode=0..1` | Exp `06` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 06 Z00` `galactic-edge` — Borde galáctico / *galactic edge* (`[TIER-B]` · Target `vid=1600`)
- **Metric (`raw_metric`):** "Continuidad · salida de la Vía Láctea" (*Continuation · leaving the Milky Way*) · `range=[0.000, 0.070]`
- **Spoken Macro (ES // EN):** Aquí termina exactamente el mapa anterior. La luz del disco se debilita, pero la gravedad de la Vía Láctea todavía alcanza mucho más lejos. // *This is exactly where the previous map ends. The light from the disk is weakening, but the Milky Way's gravity still reaches much further.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Static Local Group 3-galaxy diorama (`vid=6`, `hero_doc_cosmos.wgsl:527-542`: `q1` MW, `q2` M31, `q3` M33) + `LM[0]` also `vid=6` + Compute Mode 0 single MW spiral + `far_shell` MW skyband (`0.72` at `u=22`).
- **Audit Gap:** Same `vid=6` diorama across all 9 zones (`doc_zone`/`doc_feat` ignored). Mode 0 particles still spin solely around MW origin instead of spanning Local Group; missing diffuse radial falloff around `q1` and true-vs-compressed scale caliper.
- **Zone 3D Procedural Blueprint:** Highlight `q1` MW outer radial falloff (`f=0,1`), pull camera transition outward from MW disk to Local Group context (`f=2,3`), and overlay physical kpc/Mly compression scale indicator (`f=4`). Fix Mode 0 compute particles at `u >= 21.1` to transition into `Mode 1` / dual-galaxy clusters.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **Vía Láctea** / *Milky Way* | `[TIER-B]` | Nuestra galaxia queda debajo: una gran espiral barrada con el Sol situado lejos de su centro. // *Our galaxy lies below: a large barred spiral galaxy with the Sun far from its center.* | `q1` barred spiral represents Milky Way from outside. | Highlight isolated barred spiral `q1` (Milky Way) from outside (`doc_feat==0`). |
| `F1` | **borde difuso** / *diffuse boundary* | `[TIER-D]` | Una galaxia no termina como una esfera sólida; sus estrellas, gas y materia oscura se vuelven gradualmente más dispersos. // *A galaxy does not end like a solid sphere; its stars, gas, and dark matter gradually become more spread out.* | No extended stellar/DM radial fade boundary around `q1`. | Fade diffuse outer stellar-halo boundary cleanly into void (`doc_feat==1`). |
| `F2` | **salida galáctica** / *galactic exit* | `[TIER-B]` | La ruta abandona el disco brillante y entra en una escala donde galaxias enteras se convierten en vecinas. // *The path leaves the bright disk and enters a scale where entire galaxies become neighbors.* | Macro context exits Milky Way into multi-galaxy Local Group frame. | Pull camera outward past final MW outer spiral tendrils (`doc_feat==2`). |
| `F3` | **Grupo Local** / *Local Group* | `[TIER-B]` | La Vía Láctea forma parte de una familia gravitatoria con Andrómeda, Triángulo y numerosas galaxias pequeñas. // *The Milky Way is part of a gravitational family with Andromeda, Triangulum and numerous small galaxies.* | Shows MW (`q1`), Andromeda (`q2`), and Triangulum (`q3`) together. | Fade in faint sister M31 (`q2`) & M33 (`q3`) anchoring Local Group frame (`doc_feat==3`). |
| `F4` | **escala comprimida** / *compressed scale* | `[TIER-D]` | Las distancias reales son inmensas. El recorrido las comprime para poder compararlas en una sola pantalla. // *The real distances are immense. The tour compresses them so they can be compared on a single screen.* | No true-vs-compressed scale indicator comparing galaxy diameter to Mly separation. | Overlay compressed Mly scale ticks showing distance compression (`doc_feat==4`). |

#### `Exp 06 Z01` `outer-halo` — Halo exterior / *Outer halo* (`[TIER-D]` · Target `vid=1601`)
- **Metric (`raw_metric`):** "Más allá del disco · envoltura extensa" (*Beyond the disk · extended envelope*) · `range=[0.070, 0.180]`
- **Spoken Macro (ES // EN):** El halo exterior contiene estrellas antiguas, cúmulos, corrientes estelares, gas caliente y una gran envoltura invisible de materia oscura. // *The outer halo contains ancient stars, clusters, stellar streams, hot gas and a large invisible envelope of dark matter.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Static `vid=6` (`q1, q2, q3` bare spiral disks) + duplicate `LM[1]` `vid=6` + Mode 0 single spiral.
- **Audit Gap:** CRITICAL [TIER-D]: Neither `vid=6` nor `Mode 0` draws dark matter halos, globular clusters, tidal stellar streams, hot galactic corona gas, or hypervelocity runaway stars.
- **Zone 3D Procedural Blueprint:** Add `z_idx == 1u` overlay centered on `q1` / `q2`: vast translucent purple-indigo NFW dark-matter envelopes (`r ~ 2.5x` disk radius, `f=0`), looping golden stellar stream ribbons (`f=1`), halo globular cluster beads (`f=2`), diffuse soft X-ray corona (`f=3`), and escaping runaway star trajectories piercing halo edge (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **halo de materia oscura** / *dark matter halo* | `[TIER-D]` | Una envoltura invisible concentra gran parte de la masa que mantiene unida a la galaxia. // *An invisible envelope holds much of the mass that keeps the galaxy together.* | Zero extended dark-matter halo shell around MW or M31. | Inflate vast purple-indigo NFW dark-matter halo envelopes around `q1` & `q2` (`doc_feat==0`). |
| `F1` | **corriente estelar** / *stellar stream* | `[TIER-D]` | Restos de galaxias pequeñas pueden formar cintas de estrellas al ser estiradas por la gravedad. // *Remnants of small galaxies can form ribbons of stars when stretched by gravity.* | Zero tidal stellar stream ribbons orbiting halo. | Trace looping golden tidal stellar stream arcs around outer halos (`doc_feat==1`). |
| `F2` | **cúmulo globular** / *globular cluster* | `[TIER-D]` | Cientos de miles de estrellas antiguas pueden reunirse en una esfera compacta que orbita lejos del disco. // *Hundreds of thousands of ancient stars can be gathered into a compact sphere orbiting far from the disk.* | Zero globular cluster spheres outside disk. | Illuminate sparse distant halo globular-cluster tracer beads (`doc_feat==2`). |
| `F3` | **corona galáctica** / *galactic corona* | `[TIER-D]` | Gas muy caliente y tenue rodea la Vía Láctea mucho más allá de la parte que vemos a simple vista. // *Very hot, tenuous gas surrounds the Milky Way far beyond the part we can see with the naked eye.* | Zero hot X-ray galactic corona halo glow. | Pulse faint tenuous soft X-ray hot galactic corona mist (`doc_feat==3`). |
| `F4` | **estrella fugitiva** / *runaway star* | `[TIER-D]` | Algunas estrellas viajan por el halo en órbitas extremas o incluso pueden escapar de la galaxia. // *Some stars travel through the halo in extreme orbits or may even escape from the galaxy.* | Zero extreme-orbit / escaping runaway star tracks. | Streak hypervelocity runaway star trajectories piercing halo rim (`doc_feat==4`). |

#### `Exp 06 Z02` `dwarf-galaxies` — Galaxias enanas / *dwarf galaxies* (`[TIER-D]` · Target `vid=1602`)
- **Metric (`raw_metric`):** "Decenas de pequeñas compañeras" (*Dozens of small companion galaxies*) · `range=[0.180, 0.300]`
- **Spoken Macro (ES // EN):** La mayor parte de las galaxias del Grupo Local son pequeñas. Muchas orbitan la Vía Láctea o Andrómeda y cambian por sus fuerzas de marea. // *Most galaxies in the Local Group are small. Many orbit the Milky Way or Andromeda and are changed by their tidal forces.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Static `vid=6` (only 3 large galaxies `q1, q2, q3`) + `LM[2]` `vid=6`. Zero dwarf galaxies exist in `vid=6`.
- **Audit Gap:** CRITICAL [TIER-D]: Sagittarius Dwarf (being tidally shredded through MW disk), Fornax dwarf spheroidal, ultra-faint companions, and tidal disruption tails are completely missing.
- **Zone 3D Procedural Blueprint:** Add `z_idx == 2u` dwarf-swarm sub-nodes (`exp(-length*42)`) orbiting `q1` and `q2` (`f=0`): warped polar-orbiting Sagittarius Dwarf connected by wrapping stellar stream ribbon (`f=1,4`), gas-poor amber Fornax spheroidal (`f=2`), and faint low-surface-brightness ultra-faint halo speckles highlighted via reticle (`f=3`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **galaxia enana** / *dwarf galaxy* | `[TIER-D]` | Puede contener desde unos miles hasta miles de millones de estrellas, mucho menos que una gran galaxia espiral. // *It can contain anywhere from a few thousand to billions of stars, much less than a large spiral galaxy.* | Zero dwarf galaxies rendered in `vid=6`. | Pop dwarf galaxy swarm sub-nodes (`exp(-length*42)`) around `q1` and `q2` (`doc_feat==0`). |
| `F1` | **Sagitario Enana** / *Sagittarius Dwarf Galaxy* | `[TIER-D]` | Esta pequeña galaxia está siendo deformada y absorbida gradualmente por la Vía Láctea. // *This small galaxy is being gradually deformed and absorbed by the Milky Way.* | No Sagittarius Dwarf or polar tidal accretion stream around MW. | Highlight polar-orbiting Sagittarius Dwarf connected by wrapping stream (`doc_feat==1`). |
| `F2` | **Fornax** / *Fornax* | `[TIER-D]` | Una galaxia enana esferoidal puede tener muy poco gas y una población dominada por estrellas antiguas. // *A dwarf spheroidal galaxy may have very little gas and a population dominated by old stars.* | No Fornax dwarf spheroidal visual. | Isolate warm gas-poor amber Fornax dwarf spheroidal (`doc_feat==2`). |
| `F3` | **galaxia ultradébil** / *ultra-faint galaxy* | `[TIER-D]` | Algunas compañeras son tan tenues que solo se descubren contando cuidadosamente sus estrellas. // *Some companions are so faint that they can only be discovered by carefully counting their stars.* | No ultra-faint resolved stellar-count satellite halos. | Target faint low-surface-brightness ultra-faint halo dwarfs with reticle (`doc_feat==3`). |
| `F4` | **fuerza de marea** / *tidal force* | `[TIER-D]` | La gravedad de una galaxia grande puede estirar, calentar y deshacer lentamente a una galaxia pequeña. // *The gravity of a large galaxy can slowly stretch, heat, and break up a small galaxy.* | No tidal stretching / heating deformation plumes. | Animate tidal elongation & stripping tails along dwarf orbital paths (`doc_feat==4`). |

#### `Exp 06 Z03` `intergalactic-space` — Espacio intergaláctico / *intergalactic space* (`[TIER-D]` · Target `vid=1603`)
- **Metric (`raw_metric`):** "Entre galaxias · materia extremadamente tenue" (*Between galaxies · extremely tenuous matter*) · `range=[0.300, 0.420]`
- **Spoken Macro (ES // EN):** El espacio entre galaxias parece vacío, pero contiene gas ionizado, campos magnéticos, radiación y la estructura gravitatoria invisible del Grupo Local. // *The space between galaxies appears empty, but contains ionized gas, magnetic fields, radiation, and the invisible gravitational structure of the Local Group.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Static `vid=6` (`q1, q2, q3` separated by un-annotated black void + faint FBM `dust_cut` strictly on galactic disks).
- **Audit Gap:** CRITICAL [TIER-D]: Diffuse WHIM intergalactic plasma between galaxies, Local Group dark-matter gravitational bridge/halo envelope, 2.5-million-year traveling photon wavefronts between MW & M31, and intergalactic magnetic fields are unvisualized.
- **Zone 3D Procedural Blueprint:** Add `z_idx == 3u` intergalactic medium volume filling region between `q1` & `q2`: ultra-faint cyan ionized WHIM filaments (`f=0,4`), deep violet shared dark-matter potential bridge (`f=1`), subtle magnetic vector streamlines (`f=3`), and tiny pulsing golden photon wave-packet traveling along the 2.54 Mly geodesic between `q1` and `q2` (`f=2`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **gas intergaláctico** / *intergalactic gas* | `[TIER-D]` | Átomos y partículas muy separados ocupan el enorme volumen entre una galaxia y otra. // *Widely separated atoms and particles occupy the enormous volume between one galaxy and another.* | No ionized intergalactic WHIM gas between galaxies. | Render ultra-faint cyan ionized WHIM gas filaments between `q1` and `q2` (`doc_feat==0`). |
| `F1` | **materia oscura** / *dark matter* | `[TIER-D]` | La gravedad del Grupo Local depende en gran medida de materia que no emite ni refleja luz. // *The gravity of the Local Group depends largely on matter that neither emits nor reflects light.* | No shared dark-matter mass distribution bridging Local Group. | Reveal deep-violet shared dark-matter potential bridge spanning intergalactic gap (`doc_feat==1`). |
| `F2` | **fotón viajero** / *traveling photon* | `[TIER-D]` | La luz necesita millones de años para cruzar las distancias que separan a las grandes galaxias vecinas. // *Light takes millions of years to cross the distances that separate large neighboring galaxies.* | No light-travel pulse illustrating ~2.5 Mly photon transit time. | Launch pulsing golden photon wave-packet traversing 2.54 Mly geodesic (`doc_feat==2`). |
| `F3` | **campo magnético** / *magnetic field* | `[TIER-D]` | Campos muy débiles pueden extenderse por regiones enormes y guiar partículas cargadas. // *Very weak fields can spread over huge regions and guide charged particles.* | No weak intergalactic magnetic field streamlines. | Trace subtle faint intergalactic magnetic vector streamlines (`doc_feat==3`). |
| `F4` | **vacío aparente** / *apparent void* | `[TIER-D]` | Vacío no significa nada absoluto: incluso aquí existen partículas, radiación y gravedad. // *A vacuum does not mean absolute nothingness: particles, radiation, and gravity exist even here.* | Intergalactic space appears empty `#000000` void rather than structured fields/gas. | Attenuate brightness cleanly between galaxies to convey apparent void (`doc_feat==4`). |

#### `Exp 06 Z04` `magellanic-clouds` — Nubes de Magallanes / *Magellanic Clouds* (`[TIER-D]` · Target `vid=1604`)
- **Metric (`raw_metric`):** "Dos galaxias irregulares vecinas" (*Two neighboring irregular galaxies*) · `range=[0.420, 0.550]`
- **Spoken Macro (ES // EN):** La Gran y la Pequeña Nube de Magallanes son galaxias satélite visibles desde el hemisferio sur y están interactuando con la Vía Láctea. // *The Large and Small Magellanic Clouds are satellite galaxies visible from the southern hemisphere and are interacting with the Milky Way.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Static `vid=6` (`q1, q2, q3` ONLY!). Note `hero_doc_cosmos.wgsl:528` comment explicitly promises `+ Magellanic satellite smudges`, NEVER implemented in lines 529–542!
- **Audit Gap:** CRITICAL [TIER-D]: Unimplemented shader promise! LMC (`q4`), SMC (`q5`), 200-kpc Magellanic HI Stream & Bridge, and explosive 30 Doradus / Tarantula Nebula starburst are completely absent.
- **Zone 3D Procedural Blueprint:** Fulfill line 528 comment + `z_idx == 4u` focus: add barred irregular `LMC` (`q4`) near `q1` with glowing magenta **Tarantula Nebula** HII starburst knot (`f=0,3`), smaller tilted `SMC` (`q5`, `f=1`), sweeping cyan-blue HI **Magellanic Stream** & Bridge linking `q4-q5-q1` (`f=2`), and tidal velocity arrows (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **Gran Nube** / *Large Magellanic Cloud* | `[TIER-D]` | Es la mayor de las dos y contiene regiones intensas de formación estelar. // *It is the larger of the two and contains intense regions of star formation.* | Large Magellanic Cloud missing despite line 528 comment. | Spawn barred irregular Large Magellanic Cloud (`q4`) near `q1` (`doc_feat==0`). |
| `F1` | **Pequeña Nube** / *Small Magellanic Cloud* | `[TIER-D]` | Esta galaxia irregular está más lejos y aparece unida a su compañera por gas y fuerzas gravitatorias. // *This irregular galaxy is further away and appears bound to its companion by gas and gravitational forces.* | Small Magellanic Cloud missing despite line 528 comment. | Spawn smaller tilted Small Magellanic Cloud (`q5`) paired with LMC (`doc_feat==1`). |
| `F2` | **Corriente de Magallanes** / *Magellanic Stream* | `[TIER-D]` | Una larga cinta de gas sigue a las Nubes mientras se mueven alrededor de la Vía Láctea. // *A long ribbon of gas follows the Clouds as they move around the Milky Way.* | No trailing Magellanic gas stream & bridge toward MW. | Sweep cyan HI Magellanic Stream & Bridge linking `q4-q5-q1` (`doc_feat==2`). |
| `F3` | **Nebulosa de la Tarántula** / *Tarantula Nebula* | `[TIER-D]` | Una enorme región de nacimiento estelar brilla dentro de la Gran Nube de Magallanes. // *A huge star birth region shines within the Large Magellanic Cloud.* | No Tarantula Nebula starburst emission knot. | Ignite magenta Tarantula Nebula (30 Doradus) starburst knot in LMC (`doc_feat==3`). |
| `F4` | **interacción galáctica** / *galactic interaction* | `[TIER-D]` | Las Nubes se deforman entre sí y también por la gravedad de nuestra galaxia. // *The Clouds are deformed by each other and also by the gravity of our galaxy.* | No tidal Magellanic-MW gravitational deformation. | Overlay tidal interaction shear vectors between Clouds and Milky Way (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP06-Z00-04)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 06 (06_local_group) Zones Z00..Z04 (`galactic-edge`, `outer-halo`, `dwarf-galaxies`, `intergalactic-space`, `magellanic-clouds`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z00->1600`, `Z01->1601`, `Z02->1602`, `Z03->1603`, `Z04->1604` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Replace legacy modulo/static landmark dispatch with explicit companion IDs: LM00(milky-edge)->3680, LM01(outer-halo)->3681, LM02(dwarfs)->3682, LM03(intergalactic)->3683, LM04(magellanic)->3684.
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `andromeda` (exists), plus `magallanes`, `triangulo`, `enana`, `satelite`, `halo`, `intergalactico`, `colision`, `mareas`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z00->1600`, `Z01->1601`, `Z02->1602`, `Z03->1603`, `Z04->1604`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z00 (vid=1600, galactic-edge): F0:Milky Way; F1:diffuse boundary; F2:galactic exit; F3:Local Group; F4:compressed scale
     * Z01 (vid=1601, outer-halo): F0:dark matter halo; F1:stellar stream; F2:globular cluster; F3:galactic corona; F4:runaway star
     * Z02 (vid=1602, dwarf-galaxies): F0:dwarf galaxy; F1:Sagittarius Dwarf Galaxy; F2:Fornax; F3:ultra-faint galaxy; F4:tidal force
     * Z03 (vid=1603, intergalactic-space): F0:intergalactic gas; F1:dark matter; F2:traveling photon; F3:magnetic field; F4:apparent void
     * Z04 (vid=1604, magellanic-clouds): F0:Large Magellanic Cloud; F1:Small Magellanic Cloud; F2:Magellanic Stream; F3:Tarantula Nebula; F4:galactic interaction
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp06-z05-08"></a>
### PKG-ACT2-EXP06-Z05-08: Triangulum, Andromeda Approach, Barycenter & Local Group Rim (`z=05..08`) — `Phase 3` · Priority: **High**

**Current State:** Zones `Z05..Z08` (`4` zones, `20` features) | Zone Tiers `0A / 2B / 0C / 2D` | Feature Tiers `0A / 6B / 0C / 14D` | Render path: [`doc_stages.rs#L363`](../../crates/cosmos_render/src/doc_stages.rs#L363) -> [`shaders/hero_doc_cosmos.wgsl#L527-L542`](../../shaders/hero_doc_cosmos.wgsl#L527-L542) — Anchor & all `9` landmarks emit identical `kind=3, vid=6` static spiral diorama. `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `20` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z05` | `triangulum` | Galaxia del Triángulo / *Triangulum Galaxy* | **`1605`** | `3685` (`LM05` `triangulum`), `submode=0..1` | Exp `06` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z06` | `andromeda` | Galaxia de Andrómeda / *Andromeda Galaxy* | **`1606`** | `3686` (`LM06` `andromeda`), `submode=0..1` | Exp `06` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z07` | `local-group-center` | Centro del Grupo Local / *Center of the Local Group* | **`1607`** | `3687` (`LM07` `center`), `submode=0..1` | Exp `06` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z08` | `local-group-limit` | Límite del Grupo Local / *Boundary of the Local Group* | **`1608`** | `3688` (`LM08` `boundary`), `submode=0..1` | Exp `06` compute `0` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 06 Z05` `triangulum` — Galaxia del Triángulo / *Triangulum Galaxy* (`[TIER-B]` · Target `vid=1605`)
- **Metric (`raw_metric`):** "M33 · gran espiral del Grupo Local" (*M33 · major spiral galaxy in the Local Group*) · `range=[0.550, 0.660]`
- **Spoken Macro (ES // EN):** La galaxia del Triángulo, M33, es la tercera gran espiral del Grupo Local y destaca por sus brillantes regiones de formación estelar. // *The Triangulum Galaxy, M33, is the Local Group's third major spiral galaxy and is notable for its bright star-forming regions.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `q3` at `(+0.22, -0.28, +0.18)` inside `vid=6` (`hero_doc_cosmos.wgsl:538-539`) depicts M33 Triangulum galaxy alongside MW & M31.
- **Audit Gap:** `q3` is a featureless fuzzy exponential disk+bulge (`exp(-length*24) + exp(-abs(y)*36)*exp(-xz*12)`), lacking `M33`'s flocculent blue spiral arms (`f=1`) and iconic **NGC 604** giant HII star-forming knot (`f=2`), and ignoring `doc_zone`/`doc_feat`.
- **Zone 3D Procedural Blueprint:** Upgrade `q3` in `hero_doc_cosmos.wgsl:538` with logarithmic 2-arm blue spiral wave (`arm3`), bright magenta-cyan off-center **NGC 604** emission knot (`f=2`), highlight halo comparing M33 vs M31/MW (`f=3`), and faint dynamical tether to Andromeda subgroup (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **M33** / *M33* | `[TIER-B]` | El número procede del catálogo de Messier, una lista histórica de objetos difusos del cielo. // *The number comes from Messier's catalogue, a historical list of diffuse objects in the sky.* | `q3` represents M33 inside Local Group `vid=6`. | Zoom `q3` Triangulum (M33) flocculent blue face-on spiral (`doc_feat==0`). |
| `F1` | **galaxia espiral** / *spiral galaxy* | `[TIER-D]` | Sus estrellas, gas y polvo forman un disco con brazos curvos alrededor del centro. // *Its stars, gas, and dust form a disk with arms that curve around the center.* | `q3` is an axisymmetric smooth disk without spiral arms (`hero_doc_cosmos.wgsl:539`). | Highlight crisp 2-arm logarithmic spiral density wave (`arm3`, `doc_feat==1`). |
| `F2` | **NGC 604** / *NGC 604* | `[TIER-D]` | Una de las regiones de formación estelar más grandes del Grupo Local resplandece dentro de M33. // *One of the largest star-forming regions in the Local Group shines within M33.* | No luminous NGC 604 giant HII star-forming region inside `q3`. | Ignite bright magenta-cyan off-center NGC 604 giant HII nursery (`doc_feat==2`). |
| `F3` | **tercera gran galaxia** / *third major galaxy* | `[TIER-B]` | Es menor que Andrómeda y la Vía Láctea, pero mucho mayor que la mayoría de las galaxias enanas. // *It is smaller than Andromeda and the Milky Way, but much larger than most dwarf galaxies.* | `q3` rendered visually smaller than MW (`q1`) and M31 (`q2`). | Display mass/size comparison halo ranking M33 as third major member (`doc_feat==3`). |
| `F4` | **vecina de Andrómeda** / *neighbor of Andromeda* | `[TIER-B]` | Su movimiento y su historia están estrechamente ligados al subgrupo dominado por Andrómeda. // *Its movement and history are closely linked to the subgroup dominated by Andromeda.* | `q3` positioned close to M31 (`q2`) subgroup. | Draw faint dynamical gravitational tether linking M33 to M31 subgroup (`doc_feat==4`). |

#### `Exp 06 Z06` `andromeda` — Galaxia de Andrómeda / *Andromeda Galaxy* (`[TIER-B]` · Target `vid=1606`)
- **Metric (`raw_metric`):** "M31 · gran destino del recorrido" (*M31 · a major destination on the journey*) · `range=[0.660, 0.820]`
- **Spoken Macro (ES // EN):** Andrómeda es la gran galaxia espiral vecina de la Vía Láctea. Su disco inclinado domina visualmente esta parte del viaje. // *Andromeda is the large spiral galaxy neighboring the Milky Way. Its tilted disc visually dominates this part of the trip.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `q2` at `(+0.36, +0.12, -0.05)` tilted `0.6 rad` inside `vid=6` (`hero_doc_cosmos.wgsl:533-537`) with bright central `bulge2`, spiral `arm2`, and FBM `dust_cut`.
- **Audit Gap:** Missing M32 & M110 elliptical satellite galaxies (`f=3`) and future ~4.5 Gyr MW-M31 collision/merger tidal bridge (`f=4`). `doc_zone`/`doc_feat` ignored.
- **Zone 3D Procedural Blueprint:** Add `z_idx == 6u` close-up zoom onto `q2` + two compact amber elliptical companions **M32** and **M110** orbiting `q2` (`f=3`), isolate golden bulge (`f=1`) vs dark dust lanes (`f=2`), and animate `f=4`: projected future approach arc showing `q1` and `q2` sweeping together with tidal plumes.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **M31** / *M31* | `[TIER-B]` | Andrómeda también se conoce como M31 y puede verse a simple vista desde cielos oscuros. // *Andromeda is also known as M31 and can be seen with the naked eye from dark skies.* | `q2` renders tilted spiral Andromeda galaxy. | Zoom & brighten towering tilted Andromeda (`q2`) spiral disk (`doc_feat==0`). |
| `F1` | **bulbo central** / *central bulge* | `[TIER-B]` | Una enorme concentración de estrellas antiguas produce el resplandor brillante del centro. // *A huge concentration of ancient stars produces the bright glow at the center.* | `bulge2` renders warm bright ancient-star central core. | Isolate warm golden M31 central bulge (`doc_feat==1`). |
| `F2` | **bandas de polvo** / *dust lanes* | `[TIER-B]` | Franjas oscuras de polvo cruzan el disco y revelan su estructura inclinada. // *Dark streaks of dust cross the disk and reveal its tilted structure.* | `dust_cut` cuts dark absorbing dust bands across tilted `arm2` disk. | Carve concentric high-contrast dark dust lanes across M31 disk (`doc_feat==2`). |
| `F3` | **M32 y M110** / *M32 and M110* | `[TIER-D]` | Dos galaxias elípticas pequeñas acompañan a Andrómeda como satélites destacados. // *Two small elliptical galaxies accompany Andromeda as prominent satellites.* | Zero M32 or M110 satellite elliptical galaxies around `q2`. | Orbit compact elliptical M32 & bright M110 satellite cores around `q2` (`doc_feat==3`). |
| `F4` | **encuentro futuro** / *future encounter* | `[TIER-D]` | Andrómeda y la Vía Láctea se aproximan; dentro de varios miles de millones de años su estructura cambiará profundamente. // *Andromeda and the Milky Way are approaching each other; within several billion years, their structures will change profoundly.* | Static positions; no future Milky Way–Andromeda encounter/merger trajectory. | Animate future MW-M31 approach trajectory arcs & tidal merger plumes (`doc_feat==4`). |

#### `Exp 06 Z07` `local-group-center` — Centro del Grupo Local / *Center of the Local Group* (`[TIER-D]` · Target `vid=1607`)
- **Metric (`raw_metric`):** "Baricentro dinámico · no es un objeto visible" (*Dynamic barycenter · not a visible object*) · `range=[0.820, 0.920]`
- **Spoken Macro (ES // EN):** El centro del Grupo Local es una idea gravitatoria: un baricentro que depende de cómo se distribuye y se mueve toda la masa del sistema. // *The center of the Local Group is a gravitational concept: a barycenter determined by how all the system's mass is distributed and moves.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Static `vid=6` (`q1, q2, q3` with empty space at center `(0,0,0)` — zero force vectors, zero barycenter reticle, zero subgroup boundaries).
- **Audit Gap:** CRITICAL [TIER-D]: Invisible dynamical barycenter, MW vs Andromeda+M33 subgroup gravitational envelopes, orbital force vectors, and collective Local Group bulk-flow drift toward Virgo/GA have zero visual counterpart.
- **Zone 3D Procedural Blueprint:** Add `z_idx == 7u` dynamical overlay on `vid=6`: crisp dotted wireframe barycenter crosshair + saddle potential contours at mass-weighted center between `q1` and `q2` (`f=0,4`), twin wireframe subgroup spheres (`q1`-family vs `q2+q3`-family, `f=1`), reciprocal gravity force arrows (`f=2`), and collective bulk-velocity arrow pointing toward Virgo (`f=3`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **baricentro** / *barycenter* | `[TIER-D]` | Es el punto alrededor del cual se equilibra idealmente el movimiento de un conjunto de masas. // *It is the point around which the movement of a set of masses is ideally balanced.* | No barycenter balance reticle / Lagrange-style potential minimum indicator. | Place glowing dotted wireframe barycenter crosshair between `q1` and `q2` (`doc_feat==0`). |
| `F1` | **dos subgrupos** / *two subgroups* | `[TIER-D]` | Una gran familia rodea a la Vía Láctea y otra a Andrómeda, con M33 vinculada al entorno de Andrómeda. // *One large family surrounds the Milky Way and another surrounds Andromeda, with M33 linked to Andromeda's surroundings.* | No visual grouping separating Milky Way subgroup from Andromeda+M33 subgroup. | Enclose twin MW-family and M31+M33-family wireframe subgroup spheres (`doc_feat==1`). |
| `F2` | **gravedad compartida** / *shared gravity* | `[TIER-D]` | Todas las galaxias del Grupo Local alteran, aunque sea ligeramente, el movimiento de las demás. // *Every galaxy in the Local Group alters the motion of the others, even if only slightly.* | No mutual gravitational perturbation vector field. | Point reciprocal inward gravity force vectors across empty center (`doc_feat==2`). |
| `F3` | **movimiento colectivo** / *collective movement* | `[TIER-D]` | El Grupo Local viaja por una región mucho mayor del universo mientras sus miembros orbitan e interactúan. // *The Local Group travels across a much larger region of the universe as its members orbit and interact.* | No collective bulk-velocity vector moving Local Group through larger cosmos. | Overlay collective Local Group bulk-velocity vector pointing toward Virgo (`doc_feat==3`). |
| `F4` | **mapa invisible** / *invisible map* | `[TIER-D]` | No hay una señal luminosa en el centro: se deduce a partir de masas, velocidades y distancias. // *There is no light signal in the center: it is deduced from masses, speeds and distances.* | No inferred invisible mass/velocity contour overlay (`invisible map`). | Reveal translucent gravitational-potential saddle contours (`doc_feat==4`). |

#### `Exp 06 Z08` `local-group-limit` — Límite del Grupo Local / *Boundary of the Local Group* (`[TIER-D]` · Target `vid=1608`)
- **Metric (`raw_metric`):** "Frontera gravitatoria · transición a escalas mayores" (*Gravitational boundary · transition to larger scales*) · `range=[0.920, 1.000]`
- **Spoken Macro (ES // EN):** El Grupo Local tampoco tiene una pared nítida. En su límite, la gravedad local cede paso a estructuras mayores y al flujo general de expansión cósmica. // *The Local Group also does not have a clear wall. At its limit, local gravity gives way to larger structures and the general flow of cosmic expansion.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Static `vid=6` (`q1, q2, q3` only) + mild `far_shell` MW fade at `u=22.0` (`universe_scene.rs:346`).
- **Audit Gap:** CRITICAL [TIER-D]: Diffuse outer gravitational boundary, ~3 Mly zero-velocity turnaround surface sphere, transition toward Virgo Supercluster & Laniakea flows, and outward Hubble-expansion arrows are completely absent.
- **Zone 3D Procedural Blueprint:** Add `z_idx == 8u` outer turnaround sphere (`r = 0.78`): glowing cyan dashed geodesic zero-velocity surface where inward infall arrows reverse into outward Hubble-expansion arrows (`f=0,1`), plus faint distant M87/Virgo and Laniakea filament preview emerging outside the shell (`f=2,3,4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **frontera difusa** / *diffuse boundary* | `[TIER-D]` | La pertenencia se decide por movimiento y gravedad, no por una línea visible en el espacio. // *Belonging is decided by movement and gravity, not by a visible line in space.* | No diffuse gravitational membership / infall-vs-outflow visualization. | Soften outer boundary density gradient fading toward cosmic background (`doc_feat==0`). |
| `F1` | **superficie de velocidad cero** / *zero velocity surface* | `[TIER-D]` | Cerca de esta región ideal, la atracción del grupo y la expansión del universo se equilibran aproximadamente. // *Near this ideal region, the attraction of the group and the expansion of the universe approximately balance.* | No ~3 Mly zero-velocity turnaround surface sphere. | Illuminate cyan dashed geodesic zero-velocity turnaround surface (`r=0.78`, `doc_feat==1`). |
| `F2` | **Supercúmulo de Virgo** / *Virgo Supercluster* | `[TIER-D]` | El siguiente salto reúne al Grupo Local con muchos otros grupos y cúmulos de galaxias. // *The next leap connects the Local Group with many other galaxy groups and clusters.* | No visible Virgo Supercluster node beyond Local Group edge. | Hint distant M87 / Virgo Cluster glow beyond turnaround radius (`doc_feat==2`). |
| `F3` | **Laniakea** / *Laniakea* | `[TIER-D]` | Una estructura todavía mayor describe el flujo de numerosas galaxias hacia regiones gravitatorias comunes. // *An even larger structure describes the flow of numerous galaxies toward common gravitational regions.* | No visible Laniakea flow streamline horizon. | Preview incoming Laniakea flow streamlines outside boundary (`doc_feat==3`). |
| `F4` | **Más allá** / *beyond* | `[TIER-D]` | Más allá: el Supercúmulo de Virgo y Laniakea. Allí comienza la siguiente aplicación de la secuencia. // *Beyond lie the Virgo Supercluster and Laniakea. The next app in the sequence begins there.* | No scale-transition outward pull preview. | Transition outward camera flow cleanly into Exp 07 Laniakea watershed (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP06-Z05-08)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 06 (06_local_group) Zones Z05..Z08 (`triangulum`, `andromeda`, `local-group-center`, `local-group-limit`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z05->1605`, `Z06->1606`, `Z07->1607`, `Z08->1608` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Replace legacy modulo/static landmark dispatch with explicit companion IDs: LM05(triangulum)->3685, LM06(andromeda)->3686, LM07(center)->3687, LM08(boundary)->3688.
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `andromeda` (exists), plus `magallanes`, `triangulo`, `enana`, `satelite`, `halo`, `intergalactico`, `colision`, `mareas`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z05->1605`, `Z06->1606`, `Z07->1607`, `Z08->1608`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z05 (vid=1605, triangulum): F0:M33; F1:spiral galaxy; F2:NGC 604; F3:third major galaxy; F4:neighbor of Andromeda
     * Z06 (vid=1606, andromeda): F0:M31; F1:central bulge; F2:dust lanes; F3:M32 and M110; F4:future encounter
     * Z07 (vid=1607, local-group-center): F0:barycenter; F1:two subgroups; F2:shared gravity; F3:collective movement; F4:invisible map
     * Z08 (vid=1608, local-group-limit): F0:diffuse boundary; F1:zero velocity surface; F2:Virgo Supercluster; F3:Laniakea; F4:beyond
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

---

### Exp 07: `07_laniakea` — Laniakea / Laniakea (`10` Zones, `50` Features, `10` Landmarks)

> *Espectro de Laniakea: del Grupo Local al Gran Atractor* — Spectrum of Laniakea: from the Local Group to the Great Attractor

- **Odyssey Position (`odyssey.rs#L29-L32`):** Act 2, Position **5 / 8** (Chapter `act2_exp07`)
- **Spatial & Scale Band:** `SectorId::LANIAKEA` · `u ∈ [22.6, 24.5]` · `focus_scale_exp = 23.8` (`topology.rs:154`)
- **Compute & Far Shell:** `mode = 1` (Laniakea dual-attractor gravitational watershed streamlines into Great Attractor & Shapley, `compute_universe.wgsl:112-139`, `universe_scene.rs:308`) · Fade between `mw_alpha` -> `web_alpha` (`far_shell.wgsl:82-112`)
- **Current Render Dispatch:** Both `ExperienceAnchor` (`doc_stages.rs:696-708`) and all `10` `GalacticLandmark` entities (`doc_stages.rs:364`) emit identical `kind=3, vid=7` (`hero_doc_cosmos.wgsl:543-555`: Great Attractor knot + watershed threads + Virgo/M87 jet, ignoring `doc_zone`/`doc_feat` completely).
- **Granular Audit Tally:** Zones `0A / 5B / 0C / 5D` (`10`) · Features `0A / 10B / 0C / 40D` (`50`)
- **`DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`) Needed Additions:** `laniakea` (exists), plus `virgo`, `atractor`, `cuenca`, `flujo`, `vacio`, `lamina`, `shapley`, `m87`, `divisoria`

#### Baked Landmarks Cross-Reference (`universe_catalog.v1.json` -> Companion IDs `3700..=3709`)

| `lm` | Landmark `id` | `zone_index` | `p` · `lane` · `size` | Current Emitted `vid` (`doc_stages.rs`) | Reserved Companion `vid` |
|---:|---|---:|---|---|---|
| `LM00` | `local-group-limit` (`local-group-limit`) | `0` | `p=0.032`, `lane=-0.02`, `size=150` | `7` (static Exp 07 clone) | `3700` (`vid=1700` synced) |
| `LM01` | `local-volume` (`local-volume`) | `1` | `p=0.12`, `lane=0.04`, `size=104` | `7` (static Exp 07 clone) | `3701` (`vid=1701` synced) |
| `LM02` | `local-sheet` (`local-sheet`) | `2` | `p=0.225`, `lane=-0.04`, `size=140` | `7` (static Exp 07 clone) | `3702` (`vid=1702` synced) |
| `LM03` | `virgo-cluster` (`virgo-cluster`) | `3` | `p=0.345`, `lane=0.09`, `size=112` | `7` (static Exp 07 clone) | `3703` (`vid=1703` synced) |
| `LM04` | `virgo-supercluster` (`virgo-supercluster`) | `4` | `p=0.46`, `lane=-0.03`, `size=145` | `7` (static Exp 07 clone) | `3704` (`vid=1704` synced) |
| `LM05` | `galaxy-flows` (`galaxy-flows`) | `5` | `p=0.585`, `lane=0`, `size=160` | `7` (static Exp 07 clone) | `3705` (`vid=1705` synced) |
| `LM06` | `local-void` (`local-void`) | `6` | `p=0.69`, `lane=-0.18`, `size=118` | `7` (static Exp 07 clone) | `3706` (`vid=1706` synced) |
| `LM07` | `great-attractor` (`great-attractor`) | `7` | `p=0.805`, `lane=0.06`, `size=196` | `7` (static Exp 07 clone) | `3707` (`vid=1707` synced) |
| `LM08` | `laniakea-basin` (`laniakea-basin`) | `8` | `p=0.915`, `lane=0`, `size=230` | `7` (static Exp 07 clone) | `3708` (`vid=1708` synced) |
| `LM09` | `boundary` (`laniakea-limit`) | `9` | `p=0.982`, `lane=0`, `size=180` | `7` (static Exp 07 clone) | `3709` (`vid=1709` synced) |

<a id="pkg-act2-exp07-z00-04"></a>
### PKG-ACT2-EXP07-Z00-04: Local Volume, Local Sheet, Virgo Cluster & Supercluster (`z=00..04`) — `Phase 3` · Priority: **High**

**Current State:** Zones `Z00..Z04` (`5` zones, `25` features) | Zone Tiers `0A / 2B / 0C / 3D` | Feature Tiers `0A / 4B / 0C / 21D` | Render path: [`doc_stages.rs#L364`](../../crates/cosmos_render/src/doc_stages.rs#L364) -> [`shaders/hero_doc_cosmos.wgsl#L543-L555`](../../shaders/hero_doc_cosmos.wgsl#L543-L555) — Anchor & all `10` landmarks emit identical `kind=3, vid=7` watershed diorama (Mode 1). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `25` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z00` | `local-group-limit` | Límite del Grupo Local / *Boundary of the Local Group* | **`1700`** | `3700` (`LM00` `local-group-limit`), `submode=0..1` | Exp `07` compute `1` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z01` | `local-volume` | Volumen local / *Local Volume* | **`1701`** | `3701` (`LM01` `local-volume`), `submode=0..1` | Exp `07` compute `1` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z02` | `local-sheet` | Lámina Local / *Local Sheet* | **`1702`** | `3702` (`LM02` `local-sheet`), `submode=0..1` | Exp `07` compute `1` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z03` | `virgo-cluster` | Cúmulo de Virgo / *Virgo Cluster* | **`1703`** | `3703` (`LM03` `virgo-cluster`), `submode=0..1` | Exp `07` compute `1` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z04` | `virgo-supercluster` | Supercúmulo de Virgo / *Virgo Supercluster* | **`1704`** | `3704` (`LM04` `virgo-supercluster`), `submode=0..1` | Exp `07` compute `1` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 07 Z00` `local-group-limit` — Límite del Grupo Local / *Boundary of the Local Group* (`[TIER-D]` · Target `vid=1700`)
- **Metric (`raw_metric`):** "Continuidad · comienza el siguiente salto" (*Continuation · the next leap begins*) · `range=[0.000, 0.065]`
- **Spoken Macro (ES // EN):** El recorrido comienza exactamente donde terminó el anterior: en la frontera dinámica y difusa de nuestra pequeña familia de galaxias. // *The tour begins exactly where the previous one ended: at the dynamic and diffuse border of our small family of galaxies.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Static Laniakea watershed threads + Great Attractor knot + M87 Virgo jet (`vid=7`, `hero_doc_cosmos.wgsl:543-555`, identical across ALL 10 zones/landmarks) + Compute Mode 1 dual-attractor streamlines + `FeatureNode` directional splats.
- **Audit Gap:** CRITICAL [TIER-D]: Lacks Local Group compact sub-cluster marker on outer spur, diffuse gravitational zero-velocity shell (`r ~ 3 Mly`), and visual continuity bridge transitioning from Exp 06 galaxies to cosmic streamlines.
- **Zone 3D Procedural Blueprint:** Add `z_idx == 0u` overlay onto `vid=7`: bright cyan Local Group beacon knot + dotted turnaround boundary shell sitting on the peripheral branch feeding Virgo/Laniakea (`f=0,1,2`), zooming out to reveal galaxies becoming connected basin flow threads (`f=3,4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **Grupo Local** / *Local Group* | `[TIER-D]` | La Vía Láctea, Andrómeda, Triángulo y muchas galaxias pequeñas forman nuestra familia galáctica inmediata. // *The Milky Way, Andromeda, Triangulum and many small galaxies form our immediate galactic family.* | No compact Local Group marker distinct inside Laniakea. | Mark Local Group beacon knot on outer peripheral spur of `vid=1700` (`doc_feat==0`). |
| `F1` | **frontera difusa** / *diffuse boundary* | `[TIER-D]` | No existe una pared. La pertenencia se deduce comparando distancias, velocidades y gravedad. // *There is no wall. Membership is deduced by comparing distances, speeds and gravity.* | No diffuse velocity/gravity membership indicator. | Draw diffuse outer Local Group kinematic membership halo (`doc_feat==1`). |
| `F2` | **superficie de velocidad cero** / *zero velocity surface* | `[TIER-D]` | Es una frontera ideal donde la atracción local y la expansión cósmica llegan aproximadamente a equilibrarse. // *It is an ideal frontier where local attraction and cosmic expansion come to approximately balance.* | No zero-velocity turnaround boundary shell. | Render dotted zero-velocity turnaround shell separating internal vs basin flow (`doc_feat==2`). |
| `F3` | **salto de escala** / *scale jump* | `[TIER-D]` | Ahora las galaxias dejan de ser destinos aislados y pasan a formar corrientes, láminas, cúmulos y cuencas. // *Now galaxies are no longer isolated destinations and begin to form streams, sheets, clusters and basins.* | No continuous scale-jump transition from galaxies to basin threads. | Zoom scale ruler jump from 10 Mly group into 100+ Mly basin flows (`doc_feat==3`). |
| `F4` | **Laniakea** / *Laniakea* | `[TIER-D]` | El Grupo Local ocupa una pequeña rama dentro de una región dinámica muchísimo mayor. // *The Local Group occupies a small branch within a much larger dynamic region.* | No highlighted peripheral Local Group branch inside Laniakea. | Morph galaxy cluster dots into connected Laniakea flow streamlines (`doc_feat==4`). |

#### `Exp 07 Z01` `local-volume` — Volumen local / *Local Volume* (`[TIER-D]` · Target `vid=1701`)
- **Metric (`raw_metric`):** "Entorno cercano · grupos y galaxias dispersas" (*Nearby environment · groups and scattered galaxies*) · `range=[0.065, 0.170]`
- **Spoken Macro (ES // EN):** Al ampliar la vista aparecen numerosas galaxias y grupos vecinos, separados por grandes extensiones de espacio tenue. // *As the view zooms out, numerous neighboring galaxies and groups appear, separated by vast expanses of nearly empty space.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Same static `vid=7` watershed threads + `virgo` jet + `ga` knot (`doc_zone`/`doc_feat` ignored) + Compute Mode 1 streamlines.
- **Audit Gap:** CRITICAL [TIER-D]: Lacks ~35 Mly Local Volume discrete neighboring galaxies & loose groups (Maffei, Sculptor, M81, Centaurus A), peculiar-velocity vector arrows, and intergroup void spacing.
- **Zone 3D Procedural Blueprint:** Add `z_idx == 1u` scattered Local Volume cluster nodes (`f=0,1`), Mpc/Mly distance grid arcs (`f=2`), Hubble-subtracted peculiar velocity arrows on each group (`f=3`), and reconstructed 3D redshift-space density field slice (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **galaxia vecina** / *neighboring galaxy* | `[TIER-D]` | Fuera del Grupo Local existen muchas galaxias cercanas que pertenecen a otros grupos pequeños. // *Outside the Local Group there are many nearby galaxies that belong to other small groups.* | No discrete nearby galaxies outside Local Group. | Highlight bright nearby Local Volume galaxy knots (Centaurus A, M81, Sculptor, `doc_feat==0`). |
| `F1` | **grupo de galaxias** / *group of galaxies* | `[TIER-D]` | Varias galaxias pueden compartir una envoltura gravitatoria sin formar todavía un cúmulo gigantesco. // *Several galaxies can share a gravitational envelope without yet forming a gigantic cluster.* | No loose neighboring galaxy group halos. | Draw soft wireframe halos grouping neighbor galaxies into compact groups (`doc_feat==1`). |
| `F2` | **distancia cósmica** / *cosmic distance* | `[TIER-D]` | En esta escala, incluso los vecinos más próximos están a millones de años luz unos de otros. // *At this scale, even the nearest neighbors are millions of light-years apart.* | No Mly inter-group cosmic distance grid. | Overlay calibrated Mpc / Mly concentric distance arc rings (`doc_feat==2`). |
| `F3` | **velocidad peculiar** / *peculiar velocity* | `[TIER-D]` | Además de participar en la expansión del universo, cada galaxia posee un movimiento local causado por la gravedad. // *In addition to participating in the expansion of the universe, each galaxy has local motion caused by gravity.* | No local peculiar-velocity arrow indicators. | Attach Hubble-subtracted peculiar-velocity vectors onto each group (`doc_feat==3`). |
| `F4` | **mapa tridimensional** / *three-dimensional map* | `[TIER-D]` | Las distancias y velocidades permiten reconstruir una geografía invisible alrededor de nosotros. // *Distances and speeds allow us to reconstruct an invisible geography around us.* | No 3D local geography / redshift-distance reconstruction overlay. | Slice reconstructed 3D redshift-space galaxy density map across volume (`doc_feat==4`). |

#### `Exp 07 Z02` `local-sheet` — Lámina Local / *Local Sheet* (`[TIER-D]` · Target `vid=1702`)
- **Metric (`raw_metric`):** "Estructura aplanada · nuestro vecindario cósmico" (*Flattened structure · our cosmic neighborhood*) · `range=[0.170, 0.285]`
- **Spoken Macro (ES // EN):** El Grupo Local y otras galaxias cercanas ocupan una estructura relativamente delgada llamada Lámina Local. // *The Local Group and other nearby galaxies occupy a relatively thin structure called the Local Sheet.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Same static `vid=7` converging 3D watershed threads (`r_ga` spherical convergence, NOT flattened!).
- **Audit Gap:** CRITICAL [TIER-D]: Completely lacks the iconic flattened 2D **Local Sheet** plane (`exp(-abs(p.y)*36)`), the surrounding **Council of Giants** ring of 12 luminous spirals/ellipticals, the abutting Local Void border, and the planar bridge to Virgo.
- **Zone 3D Procedural Blueprint:** Add `z_idx == 2u` thin planar Supergalactic/Local-Sheet disk (`exp(-abs(p.y - 0.05)*42)`) (`f=0,2`) containing a glowing ring of ~12 bright Council of Giants galaxy nodes around Local Group (`f=1`), dark carved Local Void abutting `-y` side (`f=3`), and luminous filamentary spine bridging toward `virgo` (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **lámina galáctica** / *galactic sheet* | `[TIER-D]` | Una lámina es una distribución aplanada de galaxias: ancha en dos direcciones y mucho más delgada en la tercera. // *A sheet is a flattened arrangement of galaxies: wide in two directions and much thinner in the third.* | No flattened pancake sheet geometry (`wide in 2D, thin in 3rd`). | Isolate thin planar Local Sheet disk (`exp(-abs(p.y-0.05)*42)`, `doc_feat==0`). |
| `F1` | **Consejo de Gigantes** / *Council of Giants* | `[TIER-D]` | Varias galaxias grandes cercanas forman un anillo aproximado alrededor del Grupo Local dentro de la Lámina Local. // *Several nearby large galaxies form a rough ring around the Local Group within the Local Sheet.* | No ring of nearby large galaxies ('Council of Giants'). | Highlight glowing ring of ~12 massive Council of Giants galaxies (`doc_feat==1`). |
| `F2` | **plano local** / *Local Plane* | `[TIER-D]` | Nuestro vecindario no está repartido al azar; muchas galaxias cercanas comparten un plano espacial. // *Our neighborhood is not distributed randomly; many nearby galaxies share a spatial plane.* | No tilted Local Plane reference grid. | Light Local Supergalactic Plane coordinate grid along sheet (`doc_feat==2`). |
| `F3` | **borde del Vacío Local** / *edge of the Local Void* | `[TIER-D]` | La Lámina Local limita con una extensa región pobre en galaxias que influye en el patrón de movimientos. // *The Local Sheet borders an extensive galaxy-poor region that influences the pattern of motions.* | No border wall demarcating adjacent Local Void. | Carve sharp dark Local Void drop-off directly beneath `-y` side (`doc_feat==3`). |
| `F4` | **puente hacia Virgo** / *bridge to Virgo* | `[TIER-D]` | La distribución local se conecta con estructuras más densas en dirección al cúmulo de Virgo. // *The local distribution connects to denser structures toward the Virgo Cluster.* | No planar filament bridge toward Virgo Cluster. | Illuminate luminous filament spine bridging Local Sheet toward Virgo (`doc_feat==4`). |

#### `Exp 07 Z03` `virgo-cluster` — Cúmulo de Virgo / *Virgo Cluster* (`[TIER-B]` · Target `vid=1703`)
- **Metric (`raw_metric`):** "Gran nodo cercano · miles de galaxias" (*Large nearby node · thousands of galaxies*) · `range=[0.285, 0.405]`
- **Spoken Macro (ES // EN):** Virgo es el gran cúmulo de galaxias más cercano: una región densa donde galaxias, gas caliente y materia oscura comparten un profundo potencial gravitatorio. // *Virgo is the nearest large galaxy cluster: a dense region where galaxies, hot gas and dark matter share a deep gravitational potential.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `virgo = p - vec3(-0.38, 0.22, -0.10)` + bright vertical `m87_jet` relativistic plasma jet (`hero_doc_cosmos.wgsl:552-553`) inside `vid=7` + Mode 1 streamlines.
- **Audit Gap:** `virgo` is ONLY a single line jet (`m87_jet`)! Missing the ~2,000-galaxy rich cluster swarm, diffuse hot X-ray intracluster plasma halo (`T ~ 10^7-8 K`), dark-matter gravitational well contours, and fast-infalling ram-pressure-stripped spiral wakes. `doc_zone`/`doc_feat` ignored.
- **Zone 3D Procedural Blueprint:** Enhance `virgo` node on `z_idx == 3u`: dense cluster of warm/blue elliptical & spiral micro-dots (`f=0`), retain razor-sharp `m87_jet` + central SMBH shadow (`f=1`), spherical soft violet-blue X-ray bremsstrahlung ICM glow (`f=2`), gravitational potential contour cage (`f=3`), and comet-like ram-pressure stripped gas tails (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **cúmulo de galaxias** / *galaxy cluster* | `[TIER-D]` | Un cúmulo reúne muchísimas galaxias, gas extremadamente caliente y una gran cantidad de materia oscura. // *A cluster brings together many galaxies, extremely hot gas and a large amount of dark matter.* | No dense 2000-galaxy cluster swarm around `virgo` (`only 1 bare jet line`). | Populate rich Virgo Cluster swarm of golden elliptical & blue spirals (`doc_feat==0`). |
| `F1` | **M87** / *M87* | `[TIER-B]` | Esta galaxia elíptica gigante domina una de las regiones centrales de Virgo y alberga un agujero negro supermasivo. // *This giant elliptical galaxy dominates one of the central regions of Virgo and is home to a supermassive black hole.* | `m87_jet` explicitly renders M87's active relativistic jet (`line 553`). | Spotlight central giant elliptical M87 + sharp relativistic jet (`doc_feat==1`). |
| `F2` | **gas intracumular** / *intracluster gas* | `[TIER-D]` | Entre las galaxias hay plasma tan caliente que emite principalmente rayos X. // *Between galaxies there is plasma so hot that it emits mainly X-rays.* | No hot X-ray intracluster plasma glow. | Fill cluster core with soft violet-blue X-ray bremsstrahlung ICM glow (`doc_feat==2`). |
| `F3` | **materia oscura** / *dark matter* | `[TIER-D]` | La mayor parte de la masa gravitatoria del cúmulo no brilla y se detecta por sus efectos. // *Most of the cluster's gravitational mass does not shine and is detected by its effects.* | No dark-matter deep potential contour overlay. | Wrap Virgo Cluster in gravitational-potential contour dark-matter cage (`doc_feat==3`). |
| `F4` | **órbitas rápidas** / *fast orbits* | `[TIER-D]` | Las galaxias atraviesan el cúmulo a grandes velocidades y pueden perder gas durante el recorrido. // *Galaxies pass through the cluster at high speeds and can lose gas along the way.* | No high-speed crossing orbits or ram-pressure stripping tails. | Animate high-speed galaxy orbital paths & ram-pressure stripped tails (`doc_feat==4`). |

#### `Exp 07 Z04` `virgo-supercluster` — Supercúmulo de Virgo / *Virgo Supercluster* (`[TIER-B]` · Target `vid=1704`)
- **Metric (`raw_metric`):** "Rama histórica · grupos, cúmulos y filamentos" (*Historical grouping · groups, clusters, and filaments*) · `range=[0.405, 0.515]`
- **Spoken Macro (ES // EN):** El nombre Supercúmulo de Virgo describe la concentración extensa que incluye nuestro entorno y Virgo; en el mapa dinámico moderno aparece como parte de Laniakea. // *The name Virgo Supercluster describes the extensive concentration that includes our environment and Virgo; on the modern dynamic map it appears as part of Laniakea.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=7` (`virgo` branch embedded in converging Laniakea filaments) + Compute Mode 1 streamlines.
- **Audit Gap:** Missing distinct Virgo Supercluster branch envelope vs full Laniakea basin boundary (`f=3,4`) and discrete bead-chain cluster nodes (`f=2`). `doc_zone`/`doc_feat` ignored.
- **Zone 3D Procedural Blueprint:** Add `z_idx == 4u` translucent branch boundary hugging Local Group + Virgo node (`f=0,1,3`), brighten discrete cluster knots along Virgo filament chain (`f=2`), and flash dimmer outer Laniakea streamlines to show Virgo Supercluster is a peripheral sub-branch (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **supercúmulo** / *supercluster* | `[TIER-B]` | Es una asociación enorme de grupos, cúmulos, láminas y filamentos; no tiene por qué estar completamente unida por gravedad. // *It is an enormous association of groups, clusters, sheets, and filaments; it does not have to be completely bound by gravity.* | Macro `vid=7` filamentary network + `virgo` node depicts supercluster branch. | Outline translucent Virgo Supercluster branch lobe (`doc_feat==0`). |
| `F1` | **rama de Virgo** / *Virgo branch* | `[TIER-B]` | El Grupo Local ocupa una zona periférica conectada con el gran nodo de Virgo. // *The Local Group occupies a peripheral zone connected to the great node of Virgo.* | `virgo` branch visibly connects peripheral region toward main attractor. | Brighten Virgo-to-Local-Group filament arm as sub-branch of Laniakea (`doc_feat==1`). |
| `F2` | **cadena de cúmulos** / *chain of galaxy clusters* | `[TIER-D]` | Las galaxias se concentran en ramificaciones en vez de llenar el espacio de forma uniforme. // *Galaxies are concentrated in branches rather than filling space evenly.* | Smooth noise threads; no discrete chain of clusters (`chain of galaxy clusters`). | Pulse discrete cluster beads strung along the Virgo filament chain (`doc_feat==2`). |
| `F3` | **escala histórica** / *historical context* | `[TIER-D]` | Antes de Laniakea, esta era la dirección cósmica usada habitualmente para describir nuestro domicilio mayor. // *Before Laniakea was defined, this was the cosmic region commonly used to describe our larger cosmic address.* | No historical Virgo-Supercluster vs modern Laniakea boundary toggle. | Toggle legacy historical Virgo-Supercluster halo vs full Laniakea basin (`doc_feat==3`). |
| `F4` | **subestructura** / *substructure* | `[TIER-B]` | Dentro de Laniakea, Virgo es una rama importante de una cuenca de flujo mucho más extensa. // *Within Laniakea, Virgo is an important branch of a much more extensive flow basin.* | Shows Virgo branch nested inside larger flow field (`substructure`). | Fade surrounding Laniakea streams to reveal branching substructure hierarchy (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP07-Z00-04)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 07 (07_laniakea) Zones Z00..Z04 (`local-group-limit`, `local-volume`, `local-sheet`, `virgo-cluster`, `virgo-supercluster`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z00->1700`, `Z01->1701`, `Z02->1702`, `Z03->1703`, `Z04->1704` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Replace legacy modulo/static landmark dispatch with explicit companion IDs: LM00(local-group-limit)->3700, LM01(local-volume)->3701, LM02(local-sheet)->3702, LM03(virgo-cluster)->3703, LM04(virgo-supercluster)->3704.
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `laniakea` (exists), plus `virgo`, `atractor`, `cuenca`, `flujo`, `vacio`, `lamina`, `shapley`, `m87`, `divisoria`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z00->1700`, `Z01->1701`, `Z02->1702`, `Z03->1703`, `Z04->1704`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z00 (vid=1700, local-group-limit): F0:Local Group; F1:diffuse boundary; F2:zero velocity surface; F3:scale jump; F4:Laniakea
     * Z01 (vid=1701, local-volume): F0:neighboring galaxy; F1:group of galaxies; F2:cosmic distance; F3:peculiar velocity; F4:three-dimensional map
     * Z02 (vid=1702, local-sheet): F0:galactic sheet; F1:Council of Giants; F2:Local Plane; F3:edge of the Local Void; F4:bridge to Virgo
     * Z03 (vid=1703, virgo-cluster): F0:galaxy cluster; F1:M87; F2:intracluster gas; F3:dark matter; F4:fast orbits
     * Z04 (vid=1704, virgo-supercluster): F0:supercluster; F1:Virgo branch; F2:chain of galaxy clusters; F3:historical context; F4:substructure
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp07-z05-09"></a>
### PKG-ACT2-EXP07-Z05-09: Cosmic Flows, Local Void, Great Attractor & Laniakea Rim (`z=05..09`) — `Phase 3` · Priority: **High**

**Current State:** Zones `Z05..Z09` (`5` zones, `25` features) | Zone Tiers `0A / 3B / 0C / 2D` | Feature Tiers `0A / 6B / 0C / 19D` | Render path: [`doc_stages.rs#L364`](../../crates/cosmos_render/src/doc_stages.rs#L364) -> [`shaders/hero_doc_cosmos.wgsl#L543-L555`](../../shaders/hero_doc_cosmos.wgsl#L543-L555) — Anchor & all `10` landmarks emit identical `kind=3, vid=7` watershed diorama (Mode 1). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `25` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z05` | `galaxy-flows` | Corrientes galácticas / *Galactic flows* | **`1705`** | `3705` (`LM05` `galaxy-flows`), `submode=0..1` | Exp `07` compute `1` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z06` | `local-void` | Vacío Local / *Local Void* | **`1706`** | `3706` (`LM06` `local-void`), `submode=0..1` | Exp `07` compute `1` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z07` | `great-attractor` | Gran Atractor / *Great Attractor* | **`1707`** | `3707` (`LM07` `great-attractor`), `submode=0..1` | Exp `07` compute `1` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z08` | `laniakea-basin` | Cuenca de Laniakea / *Laniakea Basin* | **`1708`** | `3708` (`LM08` `laniakea-basin`), `submode=0..1` | Exp `07` compute `1` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z09` | `laniakea-limit` | Límite de Laniakea / *Laniakea Boundary* | **`1709`** | `3709` (`LM09` `boundary`), `submode=0..1` | Exp `07` compute `1` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 07 Z05` `galaxy-flows` — Corrientes galácticas / *Galactic flows* (`[TIER-B]` · Target `vid=1705`)
- **Metric (`raw_metric`):** "Velocidades peculiares · rutas gravitatorias" (*Peculiar velocities · gravitational routes*) · `range=[0.515, 0.645]`
- **Spoken Macro (ES // EN):** Al retirar matemáticamente la expansión general, los movimientos residuales dibujan corrientes que siguen valles del campo gravitatorio. // *After the overall expansion is mathematically removed, the remaining motions trace flows that follow valleys in the gravitational field.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=7` animated inward-flowing `thread` filaments (`-time * 0.25`, `line 548`) + Compute Mode 1 (`particle_kind=1.0`) directional watershed flow splats.
- **Audit Gap:** Missing gravitational potential saddle/valley equipotential mesh (`f=2`), toggleable Hubble-expansion subtraction comparison (`f=3`), and Cosmicflows peculiar-velocity sampling vector glyphs (`f=4`). `doc_zone`/`doc_feat` ignored.
- **Zone 3D Procedural Blueprint:** Highlight high-contrast `thread` streamlines (`f=0,1`), overlay wireframe gravitational-valley equipotential iso-contours (`f=2`), toggle isotropic expanding Hubble arrows vs residual peculiar flow (`f=3`), and highlight redshift-cone reconstruction sightlines (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **flujo peculiar** / *peculiar flow* | `[TIER-B]` | Es el movimiento de una galaxia respecto a la expansión cósmica media de su entorno. // *It is the movement of a galaxy with respect to the average cosmic expansion of its environment.* | Animated Mode 1 directional splats & inward `thread` pulses depict peculiar flow. | Boost directional residual peculiar-flow ribbons along curved threads (`doc_feat==0`). |
| `F1` | **línea de corriente** / *streamline* | `[TIER-B]` | Una línea de corriente representa la dirección reconstruida del movimiento en cada punto del mapa. // *A streamline represents the reconstructed direction of motion at each point on the map.* | Continuous curved `thread` streamlines trace cosmic flow paths. | Illuminate luminous coherent Laniakea streamline trajectories (`doc_feat==1`). |
| `F2` | **valle gravitatorio** / *gravitational valley* | `[TIER-D]` | Las regiones con más masa curvan las trayectorias colectivas como valles en un paisaje invisible. // *Regions with more mass curve collective trajectories like valleys in an invisible landscape.* | No gravitational potential iso-surface valleys. | Overlay wireframe gravitational-valley equipotential iso-contours (`doc_feat==2`). |
| `F3` | **expansión cósmica** / *cosmic expansion* | `[TIER-D]` | La expansión aumenta las separaciones a gran escala, mientras la gravedad añade movimientos locales sobre ese fondo. // *Expansion increases large-scale separations, while gravity adds local motions over that background.* | No toggleable background Hubble-expansion vs peculiar-flow comparison. | Toggle isotropic Hubble-expansion vectors vs residual gravitational infall (`doc_feat==3`). |
| `F4` | **reconstrucción** / *reconstruction* | `[TIER-D]` | Estas corrientes se infieren combinando posiciones, distancias, desplazamientos al rojo y modelos cosmológicos. // *These flows are inferred by combining positions, distances, redshifts, and cosmological models.* | No observational Cosmicflows distance/redshift sampling cone overlay. | Sweep Cosmicflows redshift-distance survey cone reconstruction (`doc_feat==4`). |

#### `Exp 07 Z06` `local-void` — Vacío Local / *Local Void* (`[TIER-D]` · Target `vid=1706`)
- **Metric (`raw_metric`):** "Región subdensa · pocas galaxias" (*Underdense region · few galaxies*) · `range=[0.645, 0.735]`
- **Spoken Macro (ES // EN):** El Vacío Local es una región vecina con mucha menos materia que el promedio. Los flujos se apartan de ella y se dirigen hacia estructuras más densas. // *The Local Void is a neighboring region with much less matter than average. The flows move away from it and towards denser structures.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Same static `vid=7` solid converging filament network + Mode 1 streamlines (`ZERO carved underdense cavity`).
- **Audit Gap:** CRITICAL [TIER-D]: **Local Void** is completely invisible! `vid=7` has no dark carved underdense sphere/lobe pushing velocity streamlines radially outward away from the void toward surrounding walls (`Local Sheet`).
- **Zone 3D Procedural Blueprint:** Carve an unmistakable dark underdense void pocket (`r_void < 0.32` centered off-axis at `vec3(-0.34, -0.18, 0.16)`) in `vid=7` (`dens *= smoothstep(0.18, 0.34, r_void)`), bounded by a glowing thin wall rim (`f=0,1,4`) and outward diverging repulsive-push blue arrows pointing from void core toward Local Sheet & filaments (`f=2,3`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **vacío cósmico** / *cosmic void* | `[TIER-D]` | Un vacío no está completamente vacío: contiene materia, pero mucha menos que las láminas y los filamentos vecinos. // *A void is not completely empty: it contains matter, but much less than neighboring sheets and filaments.* | No visible dark underdense cosmic void cavity. | Carve dark empty Local Void cavity (`r_void < 0.32`) contrasting dense walls (`doc_feat==0`). |
| `F1` | **subdensidad** / *underdensity* | `[TIER-D]` | Al haber menos masa, existe menos atracción desde esa dirección que desde las regiones densas opuestas. // *Since there is less mass, there is less attraction from that direction than from the opposite dense regions.* | No density-contrast / depleted attraction asymmetry. | Fade central potential well to visualize extreme gravitational underdensity (`doc_feat==1`). |
| `F2` | **expansión más rápida** / *faster expansion* | `[TIER-D]` | Los vacíos tienden a expandirse relativamente y a desplazar materia hacia sus paredes y bordes. // *Voids tend to expand relatively and move matter towards their walls and edges.* | No void boundary walls illuminated by fleeing matter. | Illuminate thin boundary walls fleeing faster via differential expansion (`doc_feat==2`). |
| `F3` | **movimiento local** / *local movement* | `[TIER-D]` | Parte del movimiento de nuestro vecindario se entiende por la combinación de alejamiento del vacío y atracción hacia zonas densas. // *Part of our neighborhood's motion results from moving away from the void while being attracted toward denser regions.* | No push-away velocity arrows demonstrating void repulsion. | Point blue outward-push repulsion arrows away from void core (`doc_feat==3`). |
| `F4` | **silencio visual** / *visual silence* | `[TIER-D]` | La oscuridad de esta región contrasta con los nodos luminosos y ayuda a revelar la estructura del mapa. // *The darkness of this region contrasts with the bright nodes and helps reveal the structure of the map.* | No dramatic visual void darkness contrasting bright filament walls. | Hold quiet dark cavity (`>= 70% #000000`) visualizing void silence (`doc_feat==4`). |

#### `Exp 07 Z07` `great-attractor` — Gran Atractor / *Great Attractor* (`[TIER-B]` · Target `vid=1707`)
- **Metric (`raw_metric`):** "Convergencia gravitatoria · no es un objeto único" (*Gravitational convergence · not a single object*) · `range=[0.735, 0.875]`
- **Spoken Macro (ES // EN):** El Gran Atractor es una región de convergencia del campo de velocidades, asociada a grandes concentraciones de masa; no es una esfera, estrella ni agujero negro gigantesco. // *The Great Attractor is a region where the velocity field converges, associated with large concentrations of mass; it is not one gigantic sphere, star, or black hole.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `ga = p - vec3(0.18, 0.04, 0.0)` convergence `knot` + converging `thread` curves (`hero_doc_cosmos.wgsl:545-555`) + Mode 1 `attractor_a`.
- **Audit Gap:** CAVEAT: `macro_fact` warns Great Attractor *'is not one gigantic sphere, star, or black hole'*, yet `knot = exp(-r_ga*16)` renders as a single bright glowing sphere! Also lacks Hydra-Centaurus & Norma multi-cluster knots, foreground Milky Way **Zone of Avoidance** dust silhouette (`f=3`), and scale-smoothing Shapley shift (`f=4`).
- **Zone 3D Procedural Blueprint:** Replace single glowing `knot` ball (`line 551`) when `z_idx == 7u` with an extended multi-cluster convergence cloud (`Hydra`, `Centaurus`, `Norma` cluster knots, `f=0,1,2`), lay a translucent dusty Milky Way band across the line-of-sight (`Zone of Avoidance`, `f=3`), and shift apparent convergence peak toward Mode 1 `attractor_b` (Shapley) on `f=4`!

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **convergencia** / *convergence* | `[TIER-B]` | Muchas líneas de corriente reconstruidas se acercan a esta región desde direcciones distintas. // *Many reconstructed streamlines approach this region from different directions.* | Multiple `thread` streamlines & Mode 1 splats visibly converge onto `ga`. | Focus converging streamlines & Mode 1 splats onto multi-node valley (`doc_feat==0`). |
| `F1` | **región de Hydra-Centauro** / *Hydra–Centaurus region* | `[TIER-D]` | La convergencia se relaciona con una extensa concentración de galaxias y cúmulos en esta zona del cielo. // *The convergence is related to an extensive concentration of galaxies and clusters in this area of ​​the sky.* | Collapsed to single `knot` sphere; lacks Hydra-Centaurus cluster group. | Split single `knot` ball into Hydra & Centaurus supercluster sub-knots (`doc_feat==1`). |
| `F2` | **cúmulo de Norma** / *Norma Cluster* | `[TIER-D]` | Norma es uno de los grandes cúmulos próximos a la región clásica del Gran Atractor. // *Norma is one of the large clusters near the classical region of the Great Attractor.* | No Norma Cluster substructure knot near convergence region. | Highlight dense Norma Cluster core buried inside convergence trough (`doc_feat==2`). |
| `F3` | **Zona de Evitación** / *Zone of Avoidance* | `[TIER-D]` | El polvo y las estrellas de la Vía Láctea ocultan parte de esta dirección y dificultan observar galaxias lejanas. // *Dust and stars in the Milky Way obscure part of this direction and make it difficult to observe distant galaxies.* | No obscuring foreground Milky Way dust band (`Zone of Avoidance`). | Draw translucent foreground dusty Milky Way Zone of Avoidance band (`doc_feat==3`). |
| `F4` | **depende de la escala** / *depends on the scale* | `[TIER-D]` | El punto aparente de convergencia cambia según la resolución y el volumen empleados para reconstruir el flujo. // *The apparent point of convergence changes depending on the resolution and volume used to reconstruct the flow.* | No scale-dependent convergence shift toward deeper Shapley Concentration. | Shift apparent flow convergence deeper toward Mode 1 Shapley `attractor_b` (`doc_feat==4`). |

#### `Exp 07 Z08` `laniakea-basin` — Cuenca de Laniakea / *Laniakea Basin* (`[TIER-B]` · Target `vid=1708`)
- **Metric (`raw_metric`):** "Cuenca de flujo · cientos de millones de años luz" (*Flow basin · hundreds of millions of light-years*) · `range=[0.875, 0.955]`
- **Spoken Macro (ES // EN):** Laniakea se define como una inmensa cuenca de velocidades peculiares: dentro de su frontera, las líneas reconstruidas pertenecen al mismo sistema de drenaje cósmico. // *Laniakea is defined as an immense basin of peculiar velocities: within its boundary, the reconstructed streamlines belong to the same cosmic drainage basin.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=7` full converging filamentary basin + Mode 1 dual-attractor (`attractor_a` GA vs `attractor_b` Shapley) watershed splats.
- **Audit Gap:** Missing explicit translucent wireframe/contour watershed boundary hull enclosing the ~500 Mly Laniakea basin (`f=1`), velocity-vector clarification (`f=3`), and outer dark-energy unbound acceleration markers (`f=4`). `doc_zone`/`doc_feat` ignored.
- **Zone 3D Procedural Blueprint:** Wrap `vid=7` on `z_idx == 8u` in a subtle glowing geodesic watershed separatrix hull (`r ~ 0.82`) showing streamlines converging inside (`f=0,1`), pulse tens-of-thousands of galaxy tracer points (`f=2`), toggle crisp vector arrows (`f=3`), and tint outer hull edges crimson where cosmic acceleration overcomes gravitational collapse (`f=4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **Laniakea** / *Laniakea* | `[TIER-B]` | Su nombre hawaiano suele interpretarse como cielo inmenso o cielo inconmensurable. // *Its Hawaiian name is usually interpreted as immense sky or immeasurable sky.* | Full filamentary drainage plume + Mode 1 streamlines render Laniakea basin. | Render full illuminated 520-Mly Laniakea watershed feather (`doc_feat==0`). |
| `F1` | **cuenca de atracción** / *basin of attraction* | `[TIER-B]` | La frontera se dibuja como una divisoria de aguas entre patrones de flujo que convergen hacia regiones diferentes. // *The border is drawn as a watershed between flow patterns that converge toward different regions.* | Converging `thread` drainage pattern illustrates cosmic watershed basin. | Enclose converging interior streamlines inside geodesic separatrix hull (`doc_feat==1`). |
| `F2` | **decenas de miles de galaxias** / *tens of thousands of galaxies* | `[TIER-B]` | La cuenca contiene una población enorme distribuida en grupos, cúmulos, láminas y ramificaciones. // *The basin contains an enormous population of galaxies distributed among groups, clusters, sheets, and branches.* | High-budget Mode 1 compute particles represent galaxy tracer population. | Pulse high-budget Mode 1 galaxy tracer particle stream (`doc_feat==2`). |
| `F3` | **mapa dinámico** / *dynamic map* | `[TIER-D]` | Laniakea no es una figura sólida: es una clasificación basada en movimientos reconstruidos. // *Laniakea is not a solid object: it is a classification based on reconstructed motions.* | No velocity-vector arrows vs solid-boundary clarity overlay. | Overlay clean kinematic velocity-vector arrows vs solid wall clarity (`doc_feat==3`). |
| `F4` | **no está totalmente ligada** / *not fully bound by gravity* | `[TIER-D]` | A esta escala, la expansión acelerada impide asumir que toda la región colapsará en un único sistema futuro. // *At this scale, accelerated expansion precludes the assumption that the entire region will collapse into a single future system.* | No cosmic-acceleration outward divergence cue (`not fully bound`). | Tint outer Laniakea fringe crimson where dark energy beats infall (`doc_feat==4`). |

#### `Exp 07 Z09` `laniakea-limit` — Límite de Laniakea / *Laniakea Boundary* (`[TIER-D]` · Target `vid=1709`)
- **Metric (`raw_metric`):** "Divisoria dinámica · entrada a la red cósmica" (*Dynamic divide · entrance to the cosmic web*) · `range=[0.955, 1.000]`
- **Spoken Macro (ES // EN):** En el borde de la cuenca, las líneas de flujo se separan hacia otras grandes regiones. La vista está lista para revelar la red cósmica completa. // *At the basin's edge, streamlines diverge toward other large regions. The view is ready to reveal the full cosmic web.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Same `vid=7` basin fading softly toward `r=0.92` (`1 - smoothstep(0.68, 0.92, r)`) + faint 23% `far_shell` Cosmic Web sky (`web_alpha=0.23` at `u=23.8`). Note Mode 1 particles reset to `attractor_a` rim rather than diverging into neighbor basins!
- **Audit Gap:** CRITICAL [TIER-D]: Zero diverging watershed **flow divide** saddle where streamlines split toward neighboring Perseus-Pisces / Shapley / Coma basins! Currently all streamlines in `vid=7` converge solely inward toward `ga`.
- **Zone 3D Procedural Blueprint:** Add `z_idx == 9u` watershed-divide saddle shader: render translucent permeable ridge surface at `r ~ 0.68` where inner Laniakea streamlines turn toward `ga` while exterior streamlines curve AWAY toward dual neighboring basin attractors (`Perseus-Pisces` & `Shapley`, `f=0,1,2`), smoothly blending into `vid=8` 1D Voronoi Cosmic Web filaments (`f=3,4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **divisoria de flujo** / *flow divide* | `[TIER-D]` | A cada lado de esta frontera ideal, las trayectorias reconstruidas pertenecen a cuencas cósmicas diferentes. // *On each side of this ideal border, the reconstructed trajectories belong to different cosmic basins.* | No watershed flow-divide surface splitting converging vs diverging trajectories. | Render translucent saddle divide surface splitting diverging flow (`doc_feat==0`). |
| `F1` | **frontera permeable** / *permeable boundary* | `[TIER-D]` | No hay una pared física y la definición puede cambiar al mejorar los datos y los modelos. // *There is no physical wall and the definition can change as data and models improve.* | No permeable dashed model boundary / uncertainty glow. | Style divide as dashed permeable statistical watershed surface (`doc_feat==1`). |
| `F2` | **cuencas vecinas** / *neighboring basins* | `[TIER-D]` | Más allá existen otras regiones dinámicas enormes conectadas por estructuras todavía mayores. // *Beyond there are other enormous dynamic regions connected by even larger structures.* | No opposing neighboring attraction basins (`Perseus-Pisces`, `Shapley`). | Place opposing Perseus-Pisces & Shapley attractors across divide (`doc_feat==2`). |
| `F3` | **red cósmica** / *cosmic web* | `[TIER-D]` | Filamentos, vacíos y nodos forman el patrón de materia a gran escala del universo. // *Filaments, voids and nodes form the large-scale pattern of matter in the universe.* | Cosmic web `vid=8` geometry absent in hero SDF (only faint 23% skybox hints at web). | Blend exterior streamlines into `vid=1800` Cosmic Web Voronoi filaments (`doc_feat==3`). |
| `F4` | **Más allá** / *beyond* | `[TIER-D]` | Más allá: los filamentos, vacíos y nodos de la red cósmica. Allí continúa la siguiente aplicación. // *Beyond lie the filaments, voids, and nodes of the cosmic web. The next app continues there.* | No seamless scale transition bridge into Exp 08 Voronoi Cosmic Web. | Open outward camera zoom seamlessly into Exp 08 Cosmic Web (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP07-Z05-09)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 07 (07_laniakea) Zones Z05..Z09 (`galaxy-flows`, `local-void`, `great-attractor`, `laniakea-basin`, `laniakea-limit`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z05->1705`, `Z06->1706`, `Z07->1707`, `Z08->1708`, `Z09->1709` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
   - Replace legacy modulo/static landmark dispatch with explicit companion IDs: LM05(galaxy-flows)->3705, LM06(local-void)->3706, LM07(great-attractor)->3707, LM08(laniakea-basin)->3708, LM09(boundary)->3709.
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `laniakea` (exists), plus `virgo`, `atractor`, `cuenca`, `flujo`, `vacio`, `lamina`, `shapley`, `m87`, `divisoria`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z05->1705`, `Z06->1706`, `Z07->1707`, `Z08->1708`, `Z09->1709`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z05 (vid=1705, galaxy-flows): F0:peculiar flow; F1:streamline; F2:gravitational valley; F3:cosmic expansion; F4:reconstruction
     * Z06 (vid=1706, local-void): F0:cosmic void; F1:underdensity; F2:faster expansion; F3:local movement; F4:visual silence
     * Z07 (vid=1707, great-attractor): F0:convergence; F1:Hydra–Centaurus region; F2:Norma Cluster; F3:Zone of Avoidance; F4:depends on the scale
     * Z08 (vid=1708, laniakea-basin): F0:Laniakea; F1:basin of attraction; F2:tens of thousands of galaxies; F3:dynamic map; F4:not fully bound by gravity
     * Z09 (vid=1709, laniakea-limit): F0:flow divide; F1:permeable boundary; F2:neighboring basins; F3:cosmic web; F4:beyond
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

---

### Exp 08: `08_cosmic_web` — Red Cósmica / Cosmic Web (`11` Zones, `55` Features, `0` Landmarks)

> *Espectro de la Red Cósmica: de Laniakea a la Estructura del Universo* — Spectrum of the Cosmic Web: from Laniakea to the Structure of the Universe

- **Odyssey Position (`odyssey.rs#L29-L32`):** Act 2, Position **6 / 8** (Chapter `act2_exp08`)
- **Spatial & Scale Band:** `SectorId::COSMIC_WEB` · `u ∈ [24.5, 26.0]` · `focus_scale_exp = 25.3` (`topology.rs:167`)
- **Compute & Far Shell:** `mode = 2` (Zel'dovich cosmic web filaments & void evacuation, `compute_universe.wgsl:140-161`, `universe_scene.rs:309`) · `web_alpha` 3D cosmic-web background filaments (`u ∈ [22, 26.2]`, `far_shell.wgsl:106-112`)
- **Current Render Dispatch:** `doc_stages.rs:696-708` emits single static `kind=3, vid=8` (`hero_doc_cosmos.wgsl:556-564`: uniform Voronoi edges + supercluster spheres; zero `doc_zone`/`doc_feat` branching, 0 baked landmarks -> `ZoneShell`/`FeatureNode` spiral dots in `assets.rs:273-326`).
- **Granular Audit Tally:** Zones `0A / 6B / 3C / 2D` (`11`) · Features `0A / 19B / 6C / 30D` (`55`)
- **`DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`) Needed Additions:** `laniakea` (exists), plus `filamento`, `nodo`, `vacio`, `muralla`, `pared`, `cumulo`, `materia`, `oscura`, `homogeneidad`

<a id="pkg-act2-exp08-z00-03"></a>
### PKG-ACT2-EXP08-Z00-03: Laniakea Divide, Inter-Supercluster Space, Filaments & Nodes (`z=00..03`) — `Phase 3` · Priority: **Medium-High**

**Current State:** Zones `Z00..Z03` (`4` zones, `20` features) | Zone Tiers `0A / 2B / 2C / 0D` | Feature Tiers `0A / 6B / 4C / 10D` | Render path: [`doc_stages.rs#L696-L708`](../../crates/cosmos_render/src/doc_stages.rs#L696-L708) -> [`shaders/hero_doc_cosmos.wgsl#L556-L564`](../../shaders/hero_doc_cosmos.wgsl#L556-L564) — Single static `kind=3, vid=8` uniform Voronoi diorama (Mode 2). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `20` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z00` | `laniakea-limit` | Límite de Laniakea / *Laniakea Boundary* | **`1800`** | `submode=0..4` (`base_flags` cue mask) | Exp `08` compute `2` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z01` | `intersupercluster-space` | Espacio intersupercúmulo / *Space between superclusters* | **`1801`** | `submode=0..4` (`base_flags` cue mask) | Exp `08` compute `2` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z02` | `galactic-filaments` | Filamentos galácticos / *Galaxy filaments* | **`1802`** | `submode=0..4` (`base_flags` cue mask) | Exp `08` compute `2` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z03` | `cluster-nodes` | Nodos de cúmulos / *Cluster nodes* | **`1803`** | `submode=0..4` (`base_flags` cue mask) | Exp `08` compute `2` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 08 Z00` `laniakea-limit` — Límite de Laniakea / *Laniakea Boundary* (`[TIER-C]` · Target `vid=1800`)
- **Metric (`raw_metric`):** "Continuidad · una cuenca dentro de una red mayor" (*Continuation · a basin within a larger web*) · `range=[0.000, 0.065]`
- **Spoken Macro (ES // EN):** El recorrido comienza exactamente donde terminó el anterior: la divisoria dinámica de Laniakea se desvanece dentro de una estructura mucho más extensa. // *The tour begins exactly where the previous one ended: the dynamic divide of Laniakea fades into a much larger structure.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `doc_stages.rs:696-708` emits generic `kind=3, vid=8` (`hero_doc_cosmos.wgsl:556-564`, uniform 1D Voronoi ridge field); Compute Mode 2 (`compute_universe.wgsl:140-161`); zero landmarks (`landmarks` = 0 for Exp 08); `doc_zone`/`doc_feat` unused.
- **Audit Gap:** Uniform Voronoi tile shows generic cosmic-web filaments without Laniakea's dipole/watershed flow divide or adjacent Perseus-Pisces / Shapley basins.
- **Zone 3D Procedural Blueprint:** Add `doc_zone==0` watershed overlay to `vid=8`: flow-streamlines diverging across a shimmering translucent zero-velocity watershed saddle separating glowing gold Laniakea streamlines (`F0`) from cyan neighboring basin (`F2`), plus Mly zoom reticle (`F3`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **Laniakea** / *Laniakea* | `[TIER-C]` | Nuestra cuenca cósmica sigue siendo reconocible abajo, pero pronto se reduce a una sola región entre muchas. // *Our cosmic basin remains recognizable below, but it soon shrinks to a single region among many.* | Generic uniform Voronoi threads (`L556-564`) omit local Laniakea Great Attractor funnel vs web. | Highlight local Laniakea funnel sub-volume in warm amber at lower edge (`doc_feat==0`). |
| `F1` | **divisoria dinámica** / *dynamic divide* | `[TIER-D]` | La frontera separa patrones reconstruidos de velocidad; no es una pared material. // *The boundary separates reconstructed velocity patterns; it is not a physical wall.* | Zero reconstructed velocity-streamline watershed divide (no opposing flow vectors). | Render opposing flow particle streams repelled from a non-solid dotted watershed membrane (`doc_feat==1`). |
| `F2` | **cuenca vecina** / *neighboring basin* | `[TIER-C]` | Al otro lado aparecen otras cuencas de flujo conectadas por la distribución de materia a gran escala. // *On the other side, other flow basins appear connected by the large-scale distribution of matter.* | Uniform noise doesn't delineate an adjacent neighboring flow basin across the divide. | Color opposite watershed half with independent Perseus-Pisces cyan attractor (`doc_feat==2`). |
| `F3` | **escala ampliada** / *expanded scale* | `[TIER-D]` | Los nombres locales pierden protagonismo a medida que el campo de visión abarca centenares de millones de años luz. // *Local names become less prominent as the field of view spans hundreds of millions of light-years.* | Static sphere radius (`u=25.3`) lacks hundreds-of-Mly field-of-view pullback cue. | Animate expanding 100 Mly -> 500 Mly grid scale rings during `doc_feat==3`. |
| `F4` | **entrada a la red** / *entry to the cosmic web* | `[TIER-B]` | Filamentos, nodos, paredes y vacíos reemplazan a una única dirección dominante. // *Filaments, nodes, walls and voids replace a single dominant direction.* | Voronoi ridges + supernodes (`L558-563`) visually depict entry into web topology. | Fade local watershed overlay outward to reveal full web filaments (`doc_feat==4`). |

#### `Exp 08 Z01` `intersupercluster-space` — Espacio intersupercúmulo / *Space between superclusters* (`[TIER-C]` · Target `vid=1801`)
- **Metric (`raw_metric`):** "Región tenue · conexiones entre grandes concentraciones" (*Sparse region · links between large concentrations*) · `range=[0.065, 0.155]`
- **Spoken Macro (ES // EN):** Entre complejos de supercúmulos no hay una nada absoluta: persisten galaxias dispersas, gas difuso, materia oscura y conexiones muy débiles. // *Between supercluster complexes there is no absolute nothingness: scattered galaxies, diffuse gas, dark matter and very weak connections persist.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Identical `kind=3, vid=8` uniform bright filaments & nodes + bright Mode 2 filaments + `cosmic_web_sky_alpha`.
- **Audit Gap:** Uniformly bright Voronoi threads contradict narration about sparse, tenuous inter-supercluster voids, isolated field galaxies, and incomplete deep surveys.
- **Zone 3D Procedural Blueprint:** For `doc_zone==1`, attenuate brightness `0.25x` between two widely separated massive supernodes (`p = ±0.52`), place faint detached field-galaxy dots in dark zones (`F1`), draw one ghostly faint dark-matter bridge (`F2`), and apply a distance radial survey-selection mask (`F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **materia tenue** / *sparse matter* | `[TIER-C]` | La densidad cae mucho, aunque el espacio conserva materia y campos gravitatorios. // *Density drops greatly, although space retains matter and gravitational fields.* | Bright dense Voronoi ridges contradict very low sparse density between superclusters. | Dim filament contrast by `75%` across wide central gap, leaving diffuse low-density haze (`doc_feat==0`). |
| `F1` | **galaxia de campo** / *field galaxy* | `[TIER-D]` | Algunas galaxias viven lejos de los grandes cúmulos y parecen aisladas en mapas poco profundos. // *Some galaxies lie far from large clusters and appear isolated in shallow surveys.* | Zero isolated field spiral/dwarf galaxy sprites inside sparse gaps. | Seed isolated pinpoint field galaxies (`salience=0.25`) floating far from dense nodes (`doc_feat==1`). |
| `F2` | **puente débil** / *weak bridge* | `[TIER-C]` | Estructuras de baja densidad pueden enlazar regiones masivas sin formar un filamento brillante y compacto. // *Low-density structures can link massive regions without forming a bright, compact filament.* | Thick crisp ridges don't depict a single faint tenuous inter-cluster tidal bridge. | Render one tenuous gossamer matter bridge spanning two bright distant nodes (`doc_feat==2`). |
| `F3` | **distancia inmensa** / *immense distance* | `[TIER-D]` | Las separaciones entre concentraciones alcanzan decenas o centenares de millones de años luz. // *Large concentrations can be separated by tens or hundreds of millions of light-years.* | No tens/hundreds-of-Mly inter-concentration distance scale baseline. | Draw glowing baseline ruler arc connecting the two separated supernodes (`doc_feat==3`). |
| `F4` | **mapa incompleto** / *incomplete map* | `[TIER-D]` | Cuanto más tenue es la materia, más difícil resulta reconstruir su distribución tridimensional. // *The more sparsely matter is distributed, the harder it is to reconstruct its three-dimensional distribution.* | No radial telescope completeness/selection falloff showing missing faint matter. | Add radial Malmquist survey wedge fade demonstrating observational drop-off (`doc_feat==4`). |

#### `Exp 08 Z02` `galactic-filaments` — Filamentos galácticos / *Galaxy filaments* (`[TIER-B]` · Target `vid=1802`)
- **Metric (`raw_metric`):** "Hebras de materia · galaxias alineadas por la estructura" (*Strands of matter · galaxies aligned by the structure*) · `range=[0.155, 0.255]`
- **Spoken Macro (ES // EN):** Los filamentos son regiones alargadas donde se concentran materia oscura, gas y galaxias. Sus luces dibujan la columna vertebral de la red cósmica. // *Filaments are elongated regions where dark matter, gas, and galaxies concentrate. The galaxies' light traces the backbone of the cosmic web.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=8` (`L556-564`) draws Voronoi curl-warped density filaments `length(vec2(f1,f2))` + Mode 2 ridge-attracted filament particles (`compute_universe.wgsl:148-160`). `doc_feat` unused.
- **Audit Gap:** Core filament geometry directly matches topic, but lacks `doc_feat` toggle separating dark-matter gravitational sheath from WHIM intergalactic gas and aligned galaxy beads.
- **Zone 3D Procedural Blueprint:** Wire `doc_feat` on `doc_zone==2`: `F0` isolates dominant ridge spine; `F1` splits translucent deep-violet dark-matter halo sheath around brighter luminous beads; `F2` adds warm-orange WHIM ionized gas glow (`f0 < 0.08`); `F3` sharpens galaxy bead clusters vs wireframe potential; `F4` pulses gravitational potential contour lines.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **filamento** / *filament* | `[TIER-B]` | Una franja alargada de sobredensidad conecta grupos y cúmulos a través de enormes distancias. // *An elongated band of overdensity connects groups and clusters across enormous distances.* | 1D Voronoi ridge intersections (`L558-561`) directly depict cosmic filaments. | Spotlight dominant elongated filament bridge connecting twin nodes (`doc_feat==0`). |
| `F1` | **materia oscura** / *dark matter* | `[TIER-D]` | La mayor parte del andamiaje gravitatorio no emite luz, pero organiza la materia visible. // *Most of the gravitational scaffolding does not emit light, but organizes visible matter.* | No visual distinction between invisible dark-matter gravitational scaffold and luminous baryons. | Render wide translucent indigo/ghost dark-matter tube enclosing narrower bright galaxy beads (`doc_feat==1`). |
| `F2` | **gas intergaláctico** / *intergalactic gas* | `[TIER-D]` | Gas caliente y templado ocupa parte de las conexiones entre galaxias y cúmulos. // *Hot and warm gas fills parts of the links between galaxies and clusters.* | No WHIM warm-hot intergalactic gas envelope distinct from galaxy points. | Add warm soft-glowing diffuse gas sheath enveloping ridge centers (`doc_feat==2`). |
| `F3` | **galaxias alineadas** / *aligned galaxies* | `[TIER-B]` | Las galaxias aparecen preferentemente cerca de las crestas de densidad, no dentro de un tubo sólido. // *Galaxies preferentially appear near density ridges, not inside a solid tube.* | Compute Mode 2 discrete particle splats + `supernode` beads cluster along invisible noise ridges (`compute_universe.wgsl:158-160`). | Disperse continuous glow into crisp discrete pinpoint galaxy nodes along density ridges (`doc_feat==3`). |
| `F4` | **trazo luminoso** / *luminous trace* | `[TIER-B]` | En esta aplicación, la luz representa distribución de materia y potencial gravitatorio, no una autopista visible en el espacio. // *In this application, light represents matter distribution and gravitational potential, not a visible highway in space.* | Translucent glowing density trace renders as non-solid field (`L560-564`). | Overlay faint topographic gravitational potential contours proving light traces potential (`doc_feat==4`). |

#### `Exp 08 Z03` `cluster-nodes` — Nodos de cúmulos / *Cluster nodes* (`[TIER-B]` · Target `vid=1803`)
- **Metric (`raw_metric`):** "Intersecciones densas · cúmulos y materia caliente" (*Dense intersections · galaxy clusters and hot matter*) · `range=[0.255, 0.355]`
- **Spoken Macro (ES // EN):** Donde varios filamentos se encuentran, la densidad aumenta y nacen nodos dominados por grandes cúmulos de galaxias. // *Where several filaments meet, the density increases and nodes dominated by large galaxy clusters are born.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=8` (`L562`) emphasizes `supernode = pow(smoothstep(0.66, 0.95, snoise3(p*6.5)), 2.8)` at filament intersections. `doc_feat` unused.
- **Audit Gap:** Supernodes mark intersections well, yet omit hot X-ray intracluster plasma halos, gravitational lensing arcs, and hierarchical infall streams.
- **Zone 3D Procedural Blueprint:** Add `doc_feat` focus camera/shader sub-components for central node when `doc_zone==3`: `F0` traces converging filament spokes; `F1` resolves central cluster into ~40 discrete swarming galaxy dots + wide indigo DM halo; `F2` ignites magenta/cyan bremsstrahlung X-ray plasma halo; `F3` activates Einstein-arc lens distortion (reusing `vid=11` math) around cluster; `F4` speeds inward Mode 2 accretion streams.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **nodo** / *node* | `[TIER-B]` | Es una región de alta conectividad donde convergen varias crestas de la red. // *It is a highly connected region where several ridges of the web converge.* | High-connectivity bright intersections rendered via `supernode` (`L562`). | Highlight a high-connectivity 5-spoke central node (`doc_feat==0`). |
| `F1` | **cúmulo de galaxias** / *galaxy cluster* | `[TIER-B]` | Miles de galaxias pueden compartir un enorme halo de materia oscura y gas caliente. // *Thousands of galaxies may share a huge halo of dark matter and hot gas.* | Dense bright node cores (`L562`) + Mode 2 particle clusters. | Resolve central knot into swarming discrete galaxy points inside a dark-matter halo (`doc_feat==1`). |
| `F2` | **gas de rayos X** / *X-ray-emitting gas* | `[TIER-D]` | El plasma entre galaxias alcanza temperaturas extremas y delata la profundidad del potencial gravitatorio. // *The plasma between galaxies reaches extreme temperatures and reveals the depth of the gravitational potential.* | Zero diffuse X-ray emitting intracluster plasma halo distinction. | Ignite volumetric hot rose-violet X-ray core halo smoothly filling the potential well (`doc_feat==2`). |
| `F3` | **lente gravitatoria** / *gravitational lens* | `[TIER-D]` | La masa del nodo curva la luz de objetos situados detrás y ayuda a cartografiar materia invisible. // *The node's mass bends light from objects behind it and helps map invisible matter.* | Zero gravitational lensing arc distortion around massive cluster node (`vid=11` exists elsewhere, unreferenced). | Warp background point-source light into tangential blue Einstein arcs around central cluster (`doc_feat==3`). |
| `F4` | **crecimiento jerárquico** / *hierarchical growth* | `[TIER-D]` | Los nodos aumentan al incorporar grupos, gas y materia procedentes de su entorno. // *Nodes grow by absorbing groups, gas, and matter from their surroundings.* | Zero infalling sub-group merger animation along feeding filaments. | Send secondary galaxy sub-clumps sliding inward along filaments into the main node (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP08-Z00-03)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 08 (08_cosmic_web) Zones Z00..Z03 (`laniakea-limit`, `intersupercluster-space`, `galactic-filaments`, `cluster-nodes`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z00->1800`, `Z01->1801`, `Z02->1802`, `Z03->1803` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `laniakea` (exists), plus `filamento`, `nodo`, `vacio`, `muralla`, `pared`, `cumulo`, `materia`, `oscura`, `homogeneidad`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z00->1800`, `Z01->1801`, `Z02->1802`, `Z03->1803`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z00 (vid=1800, laniakea-limit): F0:Laniakea; F1:dynamic divide; F2:neighboring basin; F3:expanded scale; F4:entry to the cosmic web
     * Z01 (vid=1801, intersupercluster-space): F0:sparse matter; F1:field galaxy; F2:weak bridge; F3:immense distance; F4:incomplete map
     * Z02 (vid=1802, galactic-filaments): F0:filament; F1:dark matter; F2:intergalactic gas; F3:aligned galaxies; F4:luminous trace
     * Z03 (vid=1803, cluster-nodes): F0:node; F1:galaxy cluster; F2:X-ray-emitting gas; F3:gravitational lens; F4:hierarchical growth
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp08-z04-07"></a>
### PKG-ACT2-EXP08-Z04-07: Cosmic Voids, Galactic Walls, Junctions & Supercluster Complexes (`z=04..07`) — `Phase 3` · Priority: **High**

**Current State:** Zones `Z04..Z07` (`4` zones, `20` features) | Zone Tiers `0A / 2B / 1C / 1D` | Feature Tiers `0A / 8B / 2C / 10D` | Render path: [`doc_stages.rs#L696-L708`](../../crates/cosmos_render/src/doc_stages.rs#L696-L708) -> [`shaders/hero_doc_cosmos.wgsl#L556-L564`](../../shaders/hero_doc_cosmos.wgsl#L556-L564) — Single static `kind=3, vid=8` uniform Voronoi diorama (Mode 2). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `20` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z04` | `cosmic-voids` | Vacíos cósmicos / *Cosmic voids* | **`1804`** | `submode=0..4` (`base_flags` cue mask) | Exp `08` compute `2` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z05` | `galactic-walls` | Paredes galácticas / *galactic walls* | **`1805`** | `submode=0..4` (`base_flags` cue mask) | Exp `08` compute `2` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z06` | `filament-intersections` | Intersecciones de filamentos / *Filament intersections* | **`1806`** | `submode=0..4` (`base_flags` cue mask) | Exp `08` compute `2` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z07` | `supercluster-complexes` | Complejos de supercúmulos / *Supercluster complexes* | **`1807`** | `submode=0..4` (`base_flags` cue mask) | Exp `08` compute `2` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 08 Z04` `cosmic-voids` — Vacíos cósmicos / *Cosmic voids* (`[TIER-B]` · Target `vid=1804`)
- **Metric (`raw_metric`):** "Grandes regiones subdensas · el volumen dominante" (*Large underdense regions · most of the volume*) · `range=[0.355, 0.455]`
- **Spoken Macro (ES // EN):** Los vacíos ocupan enormes volúmenes con muchas menos galaxias que el promedio. No están completamente vacíos, pero hacen visible la geometría celular de la red. // *The voids occupy huge volumes with far fewer galaxies than average. They are not completely empty, but they make the cellular geometry of the network visible.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=8` (`L560-563`) + Mode 2 (`compute_universe.wgsl:148-159`) intentionally repel density out of zero-crossing cavities to create true black cellular voids. `doc_feat` unused.
- **Audit Gap:** Cellular void cavities visually match at macro scale, yet lack dim void-galaxy specks inside interiors, local expansion vectors, and void-wall volume metrics.
- **Zone 3D Procedural Blueprint:** Add `doc_feat` modulation for `doc_zone==4`: `F0` frames a clean spherical dark cell cavity (`r_cell ~ 0.35`); `F1` reveals 4 ultra-faint dwarf void galaxies inside the dark cell; `F2` draws slow outward radial velocity arrows emptying the void toward its rim; `F3` brightens bounding walls/filaments around the void; `F4` flashes transparent wireframe polyhedral void-volume tessellation.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **vacío** / *void* | `[TIER-B]` | Una región subdensa rodeada por filamentos y paredes, parecida a una gran celda cósmica. // *A subdense region surrounded by filaments and walls, resembling a large cosmic cell.* | Dark subdense Voronoi cavities surrounded by bright filaments (`L561-563`, `compute_universe.wgsl:148`). | Center and spotlight a clean Voronoi void cell enclosed by glowing borders (`doc_feat==0`). |
| `F1` | **interior tenue** / *sparse interior* | `[TIER-D]` | Puede contener galaxias pequeñas y gas, aunque con una densidad muy inferior a la de los bordes. // *It may contain small galaxies and gas, although with a density much lower than that of the edges.* | Void interiors clamp strictly dark (`oled_void_clamp`); zero sparse void galaxies/gas visible inside. | Seed subtle `0.15` alpha dwarf void-galaxy specks & faint HI mist inside the cavity (`doc_feat==1`). |
| `F2` | **expansión local** / *local expansion* | `[TIER-D]` | Las regiones subdensas se expanden relativamente más; con el tiempo, la materia queda comparativamente más concentrada en sus límites. // *Underdense regions expand relatively faster; over time, matter becomes more concentrated along their boundaries.* | Zero outward differential Hubble expansion arrows emptying void center toward ridges. | Animate outward radial particle flow / faint cyan arrows moving from void center to rim (`doc_feat==2`). |
| `F3` | **borde del vacío** / *edge of the void* | `[TIER-B]` | Las galaxias se acumulan en las superficies y filamentos que separan vacíos vecinos. // *Galaxies accumulate on surfaces and filaments that separate neighboring voids.* | Bright glowing density ridges trace void boundaries (`L561`). | Highlight crisp border shell where galaxies accumulate around cavity perimeter (`doc_feat==3`). |
| `F4` | **volumen cósmico** / *cosmic volume* | `[TIER-B]` | Buena parte del espacio pertenece a vacíos, mientras la mayor parte de la materia se concentra en la red que los rodea. // *Voids occupy much of space, while most matter is concentrated in the cosmic web around them.* | Dark void interiors visibly dominate sphere volume (`~80%` dark space vs thin filaments). | Overlay faint Voronoi cell wireframe polyhedra quantifying void volume fraction (`doc_feat==4`). |

#### `Exp 08 Z05` `galactic-walls` — Paredes galácticas / *galactic walls* (`[TIER-D]` · Target `vid=1805`)
- **Metric (`raw_metric`):** "Láminas inmensas · superficies entre vacíos" (*Immense sheets · surfaces between voids*) · `range=[0.455, 0.560]`
- **Spoken Macro (ES // EN):** Las paredes son distribuciones anchas y relativamente delgadas de galaxias que separan grandes vacíos y alimentan filamentos más densos. // *Walls are wide, relatively thin distributions of galaxies that separate large voids and feed denser filaments.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=8` (`L559-561`) & Mode 2 (`compute_universe.wgsl:158`) compute `length(vec2(f1, f2))` (codimension-2 intersection of two fields = strictly **1D tubes**). **ZERO 2D sheet/wall primitives exist!**
- **Audit Gap:** Mathematically impossible to depict a 2D galactic sheet/wall using `length(vec2(f1, f2))` — the user hears about immense flat sheets while seeing only 1D strings.
- **Zone 3D Procedural Blueprint:** Inject codimension-1 wall term `sheet = exp(-abs(f1) * 34.0)` when `doc_zone==5`: render broad curved galaxy-stippled Voronoi face membranes partitioning two void chambers, with `F4` rotating the sheet from face-on (`broad speckled plane`) to edge-on (`mimics a 1D filament!`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **pared** / *wall* | `[TIER-D]` | Una estructura extensa y aplanada, mucho más ancha que gruesa. // *An extensive and flattened structure, much wider than thick.* | Zero 2D sheet geometry (`length(vec2(f1,f2))` yields only 1D intersection lines). | Add codimension-1 curved `fbm3` zero-surface membrane `exp(-abs(f1)*32.0)` stippled with galaxies (`doc_feat==0`). |
| `F1` | **lámina de galaxias** / *galaxy sheet* | `[TIER-D]` | Las galaxias dibujan superficies curvas, no planos perfectos ni barreras sólidas. // *Galaxies draw curved surfaces, not perfect planes or solid barriers.* | No curved translucent sheet of discrete galaxy points. | Scatter discrete pinpoint galaxy beads onto undulating 2D sheet surface (`doc_feat==1`). |
| `F2` | **frontera entre vacíos** / *boundary between voids* | `[TIER-D]` | Dos regiones subdensas pueden compartir una pared donde la materia se ha acumulado. // *Two underdense regions may share a wall where matter has accumulated.* | No planar wall partition visibly shared between two flanking dark void chambers. | Highlight twin adjacent black voids separated by a single glowing Voronoi face sheet (`doc_feat==2`). |
| `F3` | **ramificación** / *branching* | `[TIER-B]` | Dentro de una pared aparecen filamentos más estrechos y nodos todavía más densos. // *Narrower filaments and even denser nodes appear within a wall.* | Embedded denser filaments/nodes exist within the Voronoi ridge field (`L560-562`). | Blend embedded `length(vec2(f1,f2))` bright filaments & nodes embedded inside the fainter 2D sheet (`doc_feat==3`). |
| `F4` | **perspectiva tridimensional** / *three-dimensional perspective* | `[TIER-D]` | Una pared vista de canto puede parecer un filamento; la forma depende del ángulo de observación. // *A wall seen edge-on may look like a filament; its apparent shape depends on the viewing angle.* | No interactive/animated 90-degree viewpoint rotation comparing face-on sheet vs edge-on line. | Slowly tilt the isolated 2D galaxy sheet 90 degrees from broad face-on sheet to thin edge-on filament (`doc_feat==4`). |

#### `Exp 08 Z06` `filament-intersections` — Intersecciones de filamentos / *Filament intersections* (`[TIER-B]` · Target `vid=1806`)
- **Metric (`raw_metric`):** "Conectividad máxima · cruces, puentes y bifurcaciones" (*Maximum connectivity · crossings, bridges and forks*) · `range=[0.560, 0.665]`
- **Spoken Macro (ES // EN):** La red no es una cuadrícula regular. Sus filamentos se dividen, curvan y conectan en intersecciones de distinta masa y complejidad. // *The network is not a regular grid. Its filaments divide, curve and connect at intersections of different mass and complexity.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=8` curl-warped ridges naturally fork and cross asymmetrically (`L558-563`). `doc_feat` unused.
- **Audit Gap:** Branching ridge topology works natively, but lacks accretion directionality vectors, bridge diffuse haze, and minor-vs-major node hierarchy.
- **Zone 3D Procedural Blueprint:** Add `doc_feat` controls for `doc_zone==6`: `F0` highlights Y-fork bifurcation junction; `F1` renders node degree (`N=3..6` spokes) glow intensity; `F2` adds soft diffuse baryonic gas glow across thin bridges; `F3` animates directional infall particle chevrons; `F4` contrasts minor wall-crossing knots against massive cluster intersections.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **bifurcación** / *bifurcation* | `[TIER-B]` | Un filamento puede dividirse en ramas que enlazan nodos diferentes. // *A filament can be divided into branches that link different nodes.* | Curled Voronoi filaments fork and curve organically (`L558-561`). | Highlight a clean Y-bifurcation where one trunk splits into two distinct branches (`doc_feat==0`). |
| `F1` | **conectividad** / *connectivity* | `[TIER-B]` | El número y grosor de las conexiones revela cómo se integra un nodo en la red. // *The number and thickness of its connections reveal how a node fits into the web.* | Varying filament intersections show multiple converging branches. | Color-code spoke thickness and node brightness by connection count (`doc_feat==1`). |
| `F2` | **puente de materia** / *matter bridge* | `[TIER-B]` | Las zonas de enlace contienen materia difusa además de galaxias visibles. // *Connecting regions contain diffuse matter as well as visible galaxies.* | Soft diffuse glow between `f1/f2` ridges hints at tenuous connecting bridges. | Boost faint inter-node diffuse gas glow connecting bright knots (`doc_feat==2`). |
| `F3` | **anisotropía** / *anisotropy* | `[TIER-D]` | La materia llega desde direcciones preferentes; la distribución no es igual alrededor de cada nodo. // *Matter arrives from preferred directions; it is not distributed equally around every node.* | No anisotropic velocity flow arrows along preferred accretion filaments. | Animate directional infall particles flowing preferentially along dominant axes (`doc_feat==3`). |
| `F4` | **nodo secundario** / *secondary node* | `[TIER-D]` | No toda intersección contiene un cúmulo gigantesco: existen cruces menores dentro de paredes y filamentos. // *Not every intersection contains a gigantic galaxy cluster: smaller crossings exist within walls and filaments.* | Uniform `supernode` intensity threshold (`L562`) ignores minor secondary crossings vs giant clusters. | Reveal dim secondary intersection knots inside mid-filaments vs bright primary nodes (`doc_feat==4`). |

#### `Exp 08 Z07` `supercluster-complexes` — Complejos de supercúmulos / *Supercluster complexes* (`[TIER-C]` · Target `vid=1807`)
- **Metric (`raw_metric`):** "Asociaciones enormes · múltiples cuencas y nodos" (*Huge associations · multiple basins and nodes*) · `range=[0.665, 0.775]`
- **Spoken Macro (ES // EN):** A esta escala, varios supercúmulos, paredes, filamentos y vacíos forman complejos extensos sin comportarse necesariamente como un único objeto ligado. // *At this scale, multiple superclusters, walls, filaments, and voids form vast complexes without necessarily behaving as a single gravitationally bound object.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Identical single-scale `fbm3(p * 3.4)` + `snoise3(p * 6.5)` Voronoi block (`L558-563`) has no clustered multi-supercluster complexes, multi-basin colors, or extensive wall arcs.
- **Audit Gap:** Uniform tileable noise fails to illustrate regional associations of multiple superclusters, multiple dynamical flow basins, and extensive great walls with no unique center.
- **Zone 3D Procedural Blueprint:** For `doc_zone==7`, synthesize 3 distinct non-centric multi-node supercluster complexes (plus an extended Great Wall sheet arc) color-tinted by separate local velocity basins (`F2`), proving no single attractor dominates.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **complejo** / *complex* | `[TIER-C]` | Una asociación espacial de estructuras gigantes que puede contener numerosos cúmulos y supercúmulos. // *A spatial association of enormous structures that may contain many galaxy clusters and superclusters.* | Uniform tileable noise lacks grouped regional associations / distinct complexes. | Cluster groups of bright nodes into 3 discrete regional super-complexes (`doc_feat==0`). |
| `F1` | **supercúmulo** / *supercluster* | `[TIER-C]` | Es una concentración regional de grupos y cúmulos; no siempre colapsará como una sola unidad. // *It is a regional concentration of galaxy groups and clusters; it will not necessarily collapse as one unit.* | Single-scale `snoise3` supernodes don't distinguish cluster vs supercluster hierarchy. | Nest cluster-of-clusters groupings inside wispy shared halos (`doc_feat==1`). |
| `F2` | **cuencas múltiples** / *multiple basins* | `[TIER-D]` | Laniakea ya es solo una de varias cuencas dinámicas reconocibles dentro del campo. // *Laniakea is now just one of several recognizable dynamic basins in view.* | No multiple velocity flow basins shown (`vid=8` is single-color uniform field). | Color streamlines into 3 non-overlapping flow basins (`doc_feat==2`). |
| `F3` | **pared extensa** / *extensive wall* | `[TIER-D]` | Algunas concentraciones forman arcos o láminas que atraviesan grandes porciones del mapa. // *Some concentrations form arcs or sheets that traverse large portions of the map.* | No extensive sheet/wall arc traversing across complexes. | Sweep an extensive Sloan-Great-Wall-like 2D galaxy arc across the field (`doc_feat==3`). |
| `F4` | **sin centro único** / *no single center* | `[TIER-B]` | En esta vista no existe un destino gravitatorio universal que domine toda la estructura. // *At this scale, no single gravitational destination dominates the entire structure.* | Uniform isotropic Voronoi noise accurately avoids any bright central attractor. | Pulse moving origin reticle across complexes to reinforce absence of unique center (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP08-Z04-07)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 08 (08_cosmic_web) Zones Z04..Z07 (`cosmic-voids`, `galactic-walls`, `filament-intersections`, `supercluster-complexes`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z04->1804`, `Z05->1805`, `Z06->1806`, `Z07->1807` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `laniakea` (exists), plus `filamento`, `nodo`, `vacio`, `muralla`, `pared`, `cumulo`, `materia`, `oscura`, `homogeneidad`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z04->1804`, `Z05->1805`, `Z06->1806`, `Z07->1807`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z04 (vid=1804, cosmic-voids): F0:void; F1:sparse interior; F2:local expansion; F3:edge of the void; F4:cosmic volume
     * Z05 (vid=1805, galactic-walls): F0:wall; F1:galaxy sheet; F2:boundary between voids; F3:branching; F4:three-dimensional perspective
     * Z06 (vid=1806, filament-intersections): F0:bifurcation; F1:connectivity; F2:matter bridge; F3:anisotropy; F4:secondary node
     * Z07 (vid=1807, supercluster-complexes): F0:complex; F1:supercluster; F2:multiple basins; F3:extensive wall; F4:no single center
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp08-z08-10"></a>
### PKG-ACT2-EXP08-Z08-10: Deep Cosmic Web, Homogeneity Scale & Large-Scale Architecture (`z=08..10`) — `Phase 3` · Priority: **Medium-High**

**Current State:** Zones `Z08..Z10` (`3` zones, `15` features) | Zone Tiers `0A / 2B / 0C / 1D` | Feature Tiers `0A / 5B / 0C / 10D` | Render path: [`doc_stages.rs#L696-L708`](../../crates/cosmos_render/src/doc_stages.rs#L696-L708) -> [`shaders/hero_doc_cosmos.wgsl#L556-L564`](../../shaders/hero_doc_cosmos.wgsl#L556-L564) — Single static `kind=3, vid=8` uniform Voronoi diorama (Mode 2). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `15` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z08` | `deep-cosmic-web` | Red cósmica profunda / *Deep cosmic web* | **`1808`** | `submode=0..4` (`base_flags` cue mask) | Exp `08` compute `2` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z09` | `homogeneity-scale` | Escala de homogeneidad / *Scale of homogeneity* | **`1809`** | `submode=0..4` (`base_flags` cue mask) | Exp `08` compute `2` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z10` | `large-scale-structure` | Estructura a gran escala / *Large-scale structure* | **`1810`** | `submode=0..4` (`base_flags` cue mask) | Exp `08` compute `2` + `cs_field_grid` cues | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 08 Z08` `deep-cosmic-web` — Red cósmica profunda / *Deep cosmic web* (`[TIER-B]` · Target `vid=1808`)
- **Metric (`raw_metric`):** "Patrón tridimensional · capas de filamentos y vacíos" (*Three-dimensional pattern · layers of filaments and voids*) · `range=[0.775, 0.880]`
- **Spoken Macro (ES // EN):** Al ampliar aún más, la red se repite en profundidad: las conexiones cercanas se mezclan con capas lejanas hasta formar un entramado tridimensional. // *As the view zooms out further, the web repeats in depth: nearby connections blend with distant layers to form a three-dimensional mesh.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Transparent 3D raymarched Voronoi layers + Mode 2 particles + `far_shell` web (`L106-112`) convey multi-depth layering. `doc_feat` unused.
- **Audit Gap:** 3D transparency naturally exhibits overlapping depths, yet lacks spectroscopic redshift (`z`) coloring, pie-wedge galaxy survey slicing, and Zeldovich growth simulation.
- **Zone 3D Procedural Blueprint:** Add `doc_feat` depth diagnostics to `doc_zone==8`: `F0` sweeps a Z-depth tomographic slab slicing near vs far layers; `F1` maps radial distance to redshift spectrum (`blue -> amber -> crimson`); `F2` clips sphere to a twin 2dF/SDSS pie-wedge galaxy catalog slice; `F3` animates time-evolving density contrast (`linear -> non-linear web`); `F4` restores full overlapping web.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **profundidad** / *depth* | `[TIER-B]` | Las estructuras ocupan distintas distancias; una imagen plana superpone capas que en realidad están separadas. // *The structures lie at different distances; a flat image superimposes layers that are actually separate.* | Multi-step volumetric integration (`dt`, 30 steps) physically overlaps foreground and background layers. | Sweep a thin front-to-back depth slice highlighting separated overlapping planes (`doc_feat==0`). |
| `F1` | **corrimiento al rojo** / *redshift* | `[TIER-D]` | La expansión del universo ayuda a estimar la distancia y construir mapas tridimensionales de galaxias. // *The expansion of the universe helps estimate distance and build three-dimensional maps of galaxies.* | No cosmological redshift color gradient or expanding wavelength stretch cue. | Shade `r` radially from cool blue (`z~0`) to deep crimson (`z>0.2`, `doc_feat==1`). |
| `F2` | **catálogo galáctico** / *galaxy catalog* | `[TIER-D]` | Millones de posiciones permiten revelar patrones que ninguna galaxia aislada puede mostrar. // *The positions of millions of galaxies reveal patterns that no single galaxy can show.* | No pie-wedge astronomical survey geometry (SDSS/DESI double cone) vs synthetic cube. | Clip volume to realistic twin sky-survey wedges packed with discrete galaxy dots (`doc_feat==2`). |
| `F3` | **simulación cosmológica** / *cosmological simulation* | `[TIER-D]` | Los modelos numéricos reproducen cómo pequeñas fluctuaciones iniciales crecen hasta formar la red. // *Numerical models reproduce how small initial fluctuations grow into the cosmic web.* | Static curl warp amplitude (`0.25`) cannot show structure growing from smooth initial fluctuations. | Oscillate warp & contrast from smooth early ripples to sharp cosmic web (`doc_feat==3`). |
| `F4` | **red completa** / *complete cosmic web* | `[TIER-B]` | Ningún filamento domina: el patrón surge de muchas conexiones, nodos y vacíos superpuestos. // *No single filament dominates: the pattern emerges from many overlapping connections, nodes, and voids.* | Non-dominant global mesh pattern emerges across `far_shell` + hero (`L558-563`). | Display full isotropic 3D multi-layer web continuity (`doc_feat==4`). |

#### `Exp 08 Z09` `homogeneity-scale` — Escala de homogeneidad / *Scale of homogeneity* (`[TIER-D]` · Target `vid=1809`)
- **Metric (`raw_metric`):** "Promedio estadístico · las diferencias locales se suavizan" (*Statistical average · local differences are smoothed*) · `range=[0.880, 0.950]`
- **Spoken Macro (ES // EN):** En volúmenes suficientemente grandes, las concentraciones y vacíos dejan de dominar el promedio: el universo se aproxima a una distribución homogénea e isótropa en sentido estadístico. // *In sufficiently large volumes, concentrations and voids no longer dominate the average: the universe approaches a homogeneous and isotropic distribution in a statistical sense.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=8` renders fixed-scale clumpy high-contrast Voronoi ridges (`p * 3.4`) and voids; ZERO progressive spatial averaging, sampling spheres, or isotropy sweeps exist.
- **Audit Gap:** High-contrast clumpy ridges persist unchanged while narration explicitly explains statistical homogeneity and smoothing over huge sample volumes (~300+ Mpc).
- **Zone 3D Procedural Blueprint:** For `doc_zone==9`, dynamically sweep a growing transparent glowing counting sphere (`r_sample: 0.12 -> 0.72`) inside the web while computing multi-octave blur (`fBM` low-pass fade) toward a smooth uniform cosmic-mean glow (`F0/F2/F3`), paired with directional sightline cones (`F1`) and shifting observer grid (`F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **homogeneidad estadística** / *statistical homogeneity* | `[TIER-D]` | No significa que desaparezcan las estructuras, sino que sus contrastes se compensan al promediar regiones muy grandes. // *This does not mean structures disappear; their contrasts average out when very large regions are considered.* | No coarse-graining / variance smoothing transform from clumpy ridges to smooth mean. | Morph fixed ridges into low-pass blurred mean field as sample scale exceeds homogeneity radius (`doc_feat==0`). |
| `F1` | **isotropía** / *isotropy* | `[TIER-D]` | A gran escala, las propiedades promedio no privilegian una dirección espacial. // *On large scales, the average properties do not favor any spatial direction.* | No 360-degree directional cone comparison proving large-scale isotropy. | Sweep dual opposing viewing cones showing identical average density statistics (`doc_feat==1`). |
| `F2` | **volumen de muestra** / *sample volume* | `[TIER-D]` | El resultado depende de observar un volumen suficientemente amplio y corregir límites y sesgos del catálogo. // *The result depends on observing a sufficiently large volume and correcting catalog limits and biases.* | No glowing growing counting-volume sphere comparing small biased vs large fair samples. | Grow translucent spherical sample caliper from tiny void/cluster scale to global scale (`doc_feat==2`). |
| `F3` | **promedio cósmico** / *cosmic average* | `[TIER-D]` | Filamentos y vacíos continúan presentes, pero ninguna región individual representa por sí sola el conjunto. // *Filaments and voids are still present, but no individual region alone represents the whole.* | No dual-layer blend keeping filaments dimly visible beneath uniform cosmic mean. | Superimpose dim web skeleton beneath smooth homogeneous average haze (`doc_feat==3`). |
| `F4` | **principio cosmológico** / *cosmological principle* | `[TIER-D]` | Los modelos modernos describen un universo aproximadamente homogéneo e isótropo cuando se considera la escala adecuada. // *Modern models describe an approximately homogeneous and isotropic universe when the appropriate scale is considered.* | No shifted-observer comparison grid illustrating the Cosmological Principle. | Translate coordinate grid origins across distinct points showing statistically identical appearance (`doc_feat==4`). |

#### `Exp 08 Z10` `large-scale-structure` — Estructura a gran escala / *Large-scale structure* (`[TIER-B]` · Target `vid=1810`)
- **Metric (`raw_metric`):** "Destino visual · la red cósmica como sistema completo" (*Visual destination · the cosmic web as a complete system*) · `range=[0.950, 1.000]`
- **Spoken Macro (ES // EN):** La vista culmina en una red tridimensional de filamentos, paredes, nodos y vacíos. Laniakea es apenas una cuenca reconocible dentro de este patrón. // *The view culminates in a three-dimensional network of filaments, walls, nodes and voids. Laniakea is just one recognizable basin within this pattern.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=8` complete 3D Voronoi cosmic web + `far_shell` deep filaments (`u=25.3`) matches macro continuity destination. `doc_feat` unused.
- **Audit Gap:** Macro 3D web sphere matches overview cleanly; lacks highlighted miniature Laniakea pin and outer CMB/horizon preview transition.
- **Zone 3D Procedural Blueprint:** Add `doc_feat` polish to `doc_zone==10`: `F3` illuminates a tiny gold pin marker inside the vast web labeled Laniakea; `F4` fades in a faint outer microwave/horizon shimmer previewing Exp 09 transition.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **estructura a gran escala** / *large-scale structure* | `[TIER-B]` | Es el patrón colectivo de la materia cuando se cartografían volúmenes cosmológicos inmensos. // *It is the overall structure of matter revealed by mapping immense cosmological volumes.* | Complete 3D filament-node-void cosmic web sphere matches topic. | Pulse global web volume cleanly (`doc_feat==0`). |
| `F1` | **red tridimensional** / *three-dimensional web* | `[TIER-B]` | Las conexiones se extienden hacia todas las direcciones y profundidades, no solo a lo largo de la pantalla. // *The connections extend in all directions and depths, not just across the screen.* | True 3D volumetric distribution extending in X/Y/Z depth + orbiting particles. | Slow gentle 3D inspection rotation showcasing depth parallax (`doc_feat==1`). |
| `F2` | **universo sin borde visible** / *universe without visible edge* | `[TIER-B]` | Esta escala todavía no es el horizonte cósmico: solo describe cómo se organiza la materia dentro de la parte observable. // *This scale is not yet the cosmic horizon: it only describes how matter is organized within the observable part.* | Smooth radial vignette (`1 - smoothstep(0.62, 0.88, r)`) dissolves seamlessly into `far_shell` web (`L563`). | Dissolve outer sphere shell smoothly into `far_shell` web background (`doc_feat==2`). |
| `F3` | **Laniakea diminuta** / *Tiny Laniakea* | `[TIER-D]` | Nuestra cuenca se ha reducido a una marca local entre incontables regiones semejantes. // *Our basin has shrunk to a local marker among countless similar regions.* | No miniature local Laniakea basin marker scaled inside the full cosmic web. | Place glowing micro gold ring/pin showing tiny Laniakea scale (`doc_feat==3`). |
| `F4` | **Más allá** / *beyond* | `[TIER-D]` | Más allá: la luz antigua, el horizonte cósmico y el universo observable. // *Beyond: the ancient light, the cosmic horizon and the observable universe.* | No outer faint CMB / horizon transition preview (`cmb_alpha=0` at `u=25.3`). | Tint extreme outer periphery with subtle deep-Planck blue/amber horizon glow (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP08-Z08-10)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 08 (08_cosmic_web) Zones Z08..Z10 (`deep-cosmic-web`, `homogeneity-scale`, `large-scale-structure`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z08->1808`, `Z09->1809`, `Z10->1810` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `laniakea` (exists), plus `filamento`, `nodo`, `vacio`, `muralla`, `pared`, `cumulo`, `materia`, `oscura`, `homogeneidad`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z08->1808`, `Z09->1809`, `Z10->1810`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z08 (vid=1808, deep-cosmic-web): F0:depth; F1:redshift; F2:galaxy catalog; F3:cosmological simulation; F4:complete cosmic web
     * Z09 (vid=1809, homogeneity-scale): F0:statistical homogeneity; F1:isotropy; F2:sample volume; F3:cosmic average; F4:cosmological principle
     * Z10 (vid=1810, large-scale-structure): F0:large-scale structure; F1:three-dimensional web; F2:universe without visible edge; F3:Tiny Laniakea; F4:beyond
4. shaders/compute_universe.wgsl
   - Modulate particle stream & cs_field_grid accent response by zone submode/base_flags while keeping bright accents spatially bounded.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

---

### Exp 09: `09_observable_universe` — Universo Observable / Observable Universe (`11` Zones, `55` Features, `0` Landmarks)

> *Espectro del Universo Observable: de la Red Cósmica al Horizonte Cósmico* — Spectrum of the Observable Universe: from the Cosmic Web to the Cosmic Horizon

- **Odyssey Position (`odyssey.rs#L29-L32`):** Act 2, Position **7 / 8** (Chapter `act2_exp09`)
- **Spatial & Scale Band:** `SectorId::OBSERVABLE_CMB` · `u ∈ [26.0, 26.6]` · `focus_scale_exp = 26.4` (`topology.rs:180`)
- **Compute & Far Shell:** **BUG (`universe_scene.rs:309`)**: `8..=10 => 2` locks Cosmic Web galaxy filaments even through Reionization, Dark Ages, CMB, and Horizon! · **BUG (`universe_scene.rs:351`)**: `cmb_alpha = 1.0` leaks CMB mottling behind ALL 11 zones (`Z00..Z07` should not show CMB yet; starfield `star_density=0.25` persists into pre-star Dark Ages `Z07` & CMB `Z08`).
- **Current Render Dispatch:** `doc_stages.rs:696-708` emits single static `kind=3, vid=9` (`hero_doc_cosmos.wgsl:565-572`: one CMB shell + single active relativistic quasar jet; forces wrong CMB+quasar stage on `Z00..Z03, Z05..Z07, Z09..Z10`, zero `doc_zone`/`doc_feat`).
- **Granular Audit Tally:** Zones `0A / 5B / 4C / 2D` (`11`) · Features `0A / 12B / 6C / 37D` (`55`)
- **`DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`) Needed Additions:** `cuasar`, `reionizacion`, `cmb`, `horizonte` (exist!), plus `retrospeccion`, `corrimiento`, `oscuras`, `hidrogeno`, `expansion`, `luz`

<a id="pkg-act2-exp09-z00-03"></a>
### PKG-ACT2-EXP09-Z00-03: Large-Scale Continuity, Deep Field, Lookback Cone & High-z Galaxies (`z=00..03`) — `Phase 3` · Priority: **High**

**Current State:** Zones `Z00..Z03` (`4` zones, `20` features) | Zone Tiers `0A / 1B / 1C / 2D` | Feature Tiers `0A / 3B / 1C / 16D` | Render path: [`doc_stages.rs#L696-L708`](../../crates/cosmos_render/src/doc_stages.rs#L696-L708) -> [`shaders/hero_doc_cosmos.wgsl#L565-L572`](../../shaders/hero_doc_cosmos.wgsl#L565-L572) — Single static `kind=3, vid=9` CMB + quasar jet (Mode 2 lock + `cmb_alpha=1.0` leak). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `20` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z00` | `large-scale-structure` | Estructura a gran escala / *Large-scale structure* | **`1900`** | `submode=0..4` (`base_flags` cue mask) | `mode=2` (`Z0..3`), `mode=13`/reion (`Z4..7`), `mode=5`/CMB (`Z8..10`); gate `cmb_alpha` to `Z>=8` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z01` | `deep-universe` | Universo profundo / *deep universe* | **`1901`** | `submode=0..4` (`base_flags` cue mask) | `mode=2` (`Z0..3`), `mode=13`/reion (`Z4..7`), `mode=5`/CMB (`Z8..10`); gate `cmb_alpha` to `Z>=8` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z02` | `lookback-time` | Tiempo de retrospección / *Lookback time* | **`1902`** | `submode=0..4` (`base_flags` cue mask) | `mode=2` (`Z0..3`), `mode=13`/reion (`Z4..7`), `mode=5`/CMB (`Z8..10`); gate `cmb_alpha` to `Z>=8` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z03` | `distant-galaxies` | Galaxias lejanas / *distant galaxies* | **`1903`** | `submode=0..4` (`base_flags` cue mask) | `mode=2` (`Z0..3`), `mode=13`/reion (`Z4..7`), `mode=5`/CMB (`Z8..10`); gate `cmb_alpha` to `Z>=8` | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 09 Z00` `large-scale-structure` — Estructura a gran escala / *Large-scale structure* (`[TIER-B]` · Target `vid=1900`)
- **Metric (`raw_metric`):** "Continuidad · la red cósmica en el universo cercano" (*Continuation · the cosmic web in the nearby universe*) · `range=[0.000, 0.065]`
- **Spoken Macro (ES // EN):** El recorrido comienza exactamente donde terminó el anterior: filamentos, nodos, paredes y vacíos aún forman una red tridimensional reconocible. // *The journey begins exactly where the previous one ended: filaments, nodes, walls, and voids still form a recognizable three-dimensional web.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `doc_stages.rs:696-708` emits `kind=3, vid=9` (`hero_doc_cosmos.wgsl:565-572`: horizon wireframe shell `r=0.76`, meridians, `deep_knots`, central `0.5x` quasar) + Mode 2 Cosmic Web particles (`universe_scene.rs:309`) + `cmb_alpha=1.0` (`u=26.4`).
- **Audit Gap:** Active Mode 2 filaments + `deep_knots` provide continuity from Exp 08, yet `cmb_alpha=1.0` and an active quasar jet leak prematurely into `Z00`; zero photon-travel wavefronts or lookback epoch transition exist.
- **Zone 3D Procedural Blueprint:** At `doc_zone==0`, prioritize nearby web + gradual outer horizon shell while muting premature quasar jet & CMB shell, and animate inward photon wavefront pulses (`F3`) + epoch timeline tick (`F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **red cósmica** / *cosmic web* | `[TIER-B]` | La materia se distribuye en filamentos, nodos, paredes y grandes vacíos; los trazos luminosos no son tubos físicos. // *Matter is distributed among filaments, nodes, walls, and large voids; the luminous traces are not physical tubes.* | Mode 2 web filaments + `deep_knots` visibly carry over 3D web structure (`compute_universe.wgsl:140`, `L570`). | Highlight non-solid filamentary nodes & web skeleton (`doc_feat==0`). |
| `F1` | **universo cercano** / *nearby universe* | `[TIER-B]` | Esta zona representa estructuras cuya luz llega desde épocas relativamente recientes de la historia cósmica. // *This area represents structures whose light comes from relatively recent times in cosmic history.* | Inner `deep_knots` volume (`r < 0.6`) separates nearby epoch from outer horizon shell (`r=0.76`). | Tint nearby inner region warm crisp white-blue (`doc_feat==1`). |
| `F2` | **escala ampliada** / *expanded scale* | `[TIER-B]` | Al ascender, la misma red ocupa una fracción cada vez menor del campo y se mezcla con capas más lejanas. // *As the view pulls back, the same web occupies an ever-smaller fraction of the field and blends with more distant layers.* | Nesting nearby knots inside large `r=0.76` geodesic sphere shows scale shrinkage. | Shrink inner cosmic web core smoothly as outer layers expand (`doc_feat==2`). |
| `F3` | **luz en viaje** / *light in transit* | `[TIER-D]` | Cada galaxia aparece como era cuando emitió la luz que finalmente alcanza nuestros telescopios. // *Each galaxy appears as it was when it emitted the light that eventually reaches our telescopes.* | No inward-propagating photon spheres / wave rings showing finite light travel time. | Emit faint contracting light-wave shells traveling from distant knots toward center (`doc_feat==3`). |
| `F4` | **punto de partida** / *starting point* | `[TIER-D]` | La exploración deja de ser solo un aumento de tamaño: a partir de aquí también retrocede en el tiempo. // *Exploration is no longer just an increase in size: from here on it also goes back in time.* | No radial lookback-time ruler marking space->time shift. | Draw concentric radial epoch rings (`0 -> 13.8 Gyr` lookback ticks, `doc_feat==4`). |

#### `Exp 09 Z01` `deep-universe` — Universo profundo / *deep universe* (`[TIER-C]` · Target `vid=1901`)
- **Metric (`raw_metric`):** "Campo profundo · muchas épocas en una misma mirada" (*Deep field · many eras in a single view*) · `range=[0.065, 0.155]`
- **Spoken Macro (ES // EN):** Un campo profundo reúne galaxias situadas a distancias muy distintas. Las más tenues suelen mostrarnos etapas más antiguas del universo. // *A deep field brings together galaxies located at very different distances. The faintest ones usually show us older stages of the universe.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Same `vid=9` isotropic geodesic cage + `0.5x` quasar + uniform `snoise3(p*14)` specks (`L565-572`). Zero narrow pencil-beam deep field or lensing distortion.
- **Audit Gap:** Isotropic sphere cage & uniform specks fail to illustrate a tiny-patch deep-field exposure (HUDF/JWST), distance-layer projection, faint galaxy dimming, or gravitational lensing arcs.
- **Zone 3D Procedural Blueprint:** For `doc_zone==1`, render a narrow glowing conical HUDF/JWST deep-field pencil beam (`theta < 0.28 rad`) filled with layered galaxy disks shrinking & reddening with radius (`F0/F1/F2`), plus an intervening cluster creating distorted blue Einstein arcs (`F3`) and epoch strata (`F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **campo profundo** / *deep field* | `[TIER-C]` | Una exposición prolongada revela miles de galaxias en una porción diminuta del cielo. // *A long exposure reveals thousands of galaxies in a tiny patch of the sky.* | Uniform spherical noise specks loosely imitate galaxy dots, lacking narrow tiny-sky deep-field telescope cone. | Extract a bright narrow deep-field frustum beam cutting from foreground out to `r=0.85` (`doc_feat==0`). |
| `F1` | **capas de distancia** / *distance layers* | `[TIER-D]` | Objetos próximos y remotos se proyectan en la misma imagen aunque estén separados por enormes profundidades. // *Nearby and remote objects are projected in the same image even if they are separated by enormous depths.* | No foreground-to-background projection stack separating near spirals from deep faint specks. | Render 4 discrete backlit glass-like epoch planes along beam axis that compress onto an imaging sensor (`doc_feat==1`). |
| `F2` | **galaxia tenue** / *faint galaxy* | `[TIER-D]` | La debilidad aparente puede deberse a gran distancia, poca luminosidad o ambas cosas. // *Apparent faintness may result from great distance, low intrinsic luminosity, or both.* | Uniform `deep_knots` lack brightness/surface-brightness drop-off distinguishing faint vs distant galaxies. | Vary galaxy disk sizes, intrinsic colors, and `(1+z)` fading along depth (`doc_feat==2`). |
| `F3` | **lente gravitatoria** / *gravitational lens* | `[TIER-D]` | La masa intermedia puede ampliar y deformar la luz de galaxias todavía más lejanas. // *Intervening mass can magnify and distort the light from even more distant galaxies.* | Zero gravitational lens arcs distorting distant background galaxy images (`vid=11` not wired). | Place foreground amber cluster lens stretching background blue galaxies into tangential arcs (`doc_feat==3`). |
| `F4` | **universo en evolución** / *evolving universe* | `[TIER-D]` | Comparar distintas profundidades permite estudiar cómo cambian las galaxias a lo largo del tiempo cósmico. // *Comparing different depths allows us to study how galaxies change over cosmic time.* | All `deep_knots` look identical with radius — zero morphological galaxy evolution over time. | Morph foreground settled spirals into tiny turbulent blue knots at far beam end (`doc_feat==4`). |

#### `Exp 09 Z02` `lookback-time` — Tiempo de retrospección / *Lookback time* (`[TIER-D]` · Target `vid=1902`)
- **Metric (`raw_metric`):** "Distancia y pasado · observamos luz, no el presente remoto" (*Distance and past · we observe light, not the remote present*) · `range=[0.155, 0.245]`
- **Spoken Macro (ES // EN):** Cuanto más lejos observamos, más antigua es la luz que recibimos. Mirar profundamente en el espacio es mirar profundamente en el pasado. // *The farther away we look, the older the light we receive. Looking deep into space means looking deep into the past.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Same static `vid=9` geodesic sphere + quasar jet + CMB haze; zero spacetime light-cone, photon transit animation, wavelength stretching, or simultaneity split.
- **Audit Gap:** Narrates spacetime light cones, light-year distance vs time, cosmological redshift wavelength stretching, and the impossibility of an instantaneous cosmic present — with zero visual counterpart.
- **Zone 3D Procedural Blueprint:** Build a 3D hourglass past **light-cone** wireframe (`r_cone = c * (t_0 - t)`) inside `doc_zone==2`: animate glowing photons descending cone slope toward observer apex (`F0`), bracket a 1-light-year segment (`F1`), visibly stretch a sinusoidal wave packet from blue-violet to red (`F2`), shade causal inside vs non-causal outside (`F3`), and fade out distant present slice (`F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **tiempo de viaje** / *travel time* | `[TIER-D]` | La luz tarda tiempo en cruzar el espacio; una galaxia distante se ve como era al emitir sus fotones. // *Light takes time to cross space; a distant galaxy is seen as it was when emitting its photons.* | No animated photon pulses traveling finite speed across space (`F0`). | Emit bright golden photon packets sliding along past-light-cone generators to apex (`doc_feat==0`). |
| `F1` | **año luz** / *light-year* | `[TIER-D]` | Es una unidad de distancia: lo que la luz recorre en un año, no una unidad de tiempo. // *It is a unit of distance: what light travels in a year, not a unit of time.* | No light-year distance segment vs travel-time calibration gauge. | Draw `1 yr -> 1 ly` orthogonal distance/time ruler ticks along cone edge (`doc_feat==1`). |
| `F2` | **corrimiento al rojo** / *redshift* | `[TIER-D]` | La expansión estira la longitud de onda de la luz y ayuda a ordenar las observaciones por época cósmica. // *The expansion stretches the wavelength of light and helps organize observations by cosmic epoch.* | No spectral wavelength stretching animation showing cosmological redshift. | Render an animated sinusoidal photon wave oscillating in place while stretching `lambda` blue -> red (`doc_feat==2`). |
| `F3` | **cono de luz pasado** / *past light cone* | `[TIER-D]` | Solo recibimos señales procedentes de regiones y momentos capaces de conectarse causalmente con nosotros. // *We receive signals only from regions and times that can be causally connected to us.* | No 3D spacetime past light-cone geometry separating causally connected events. | Sculpt crisp cyan geodesic 45° light-cone surface + dim unreachable outside region (`doc_feat==3`). |
| `F4` | **sin presente simultáneo** / *without simultaneous present* | `[TIER-D]` | No existe una fotografía instantánea accesible de todo el universo lejano tal como es ahora. // *There is no accessible snapshot of the entire distant universe as it is now.* | No visual distinction between observed past light-cone and inaccessible remote `now` hyper-plane. | Render ghost dotted `t = now` equatorial plane detached from visible past cone (`doc_feat==4`). |

#### `Exp 09 Z03` `distant-galaxies` — Galaxias lejanas / *distant galaxies* (`[TIER-D]` · Target `vid=1903`)
- **Metric (`raw_metric`):** "Universo más joven · formas pequeñas, turbulentas y compactas" (*Younger universe · small, turbulent, compact forms*) · `range=[0.245, 0.345]`
- **Spoken Macro (ES // EN):** Las galaxias lejanas suelen verse más jóvenes, compactas e irregulares. Su luz revela etapas en las que todavía estaban ensamblando estrellas y estructura. // *Distant galaxies often look younger, more compact, and more irregular. Their light reveals stages when they were still assembling their stars and structures.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Same `vid=9` isotropic cage + Mode 2 web + `deep_knots` specks; zero clumpy high-z irregular galaxies, starburst regions, galaxy mergers, or low-metallicity spectral cues.
- **Audit Gap:** Static tiny noise specks convey zero galaxy morphology, merger tidal tails, intense blue UV star-formation knots, or instrument/redshift image stretching.
- **Zone 3D Procedural Blueprint:** For `doc_zone==3`, place 3 volumetric high-z irregular clumpy protogalaxies in center stage: contrast turbulent clumpy knots vs ghosted late spiral outline (`F0`), ignite bright cyan-violet HII starburst clumps (`F1`), animate two colliding galaxies with tidal bridge streams (`F2`), strip dusty orange metallicity halos leaving pure pristine blue-white glow (`F3`), and stretch/redden through simulated telescope filter (`F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **galaxia joven** / *young galaxy* | `[TIER-D]` | Muchas todavía no han desarrollado los discos y brazos espirales tranquilos comunes en épocas posteriores. // *Many had not yet developed the settled disks and spiral arms common in later eras.* | Zero turbulent compact clumpy protogalaxy geometry (`snoise3` is single-pixel noise). | Sculpt multi-clump turbulent glowing protogalaxy shapes lacking tranquil spiral arms (`doc_feat==0`). |
| `F1` | **formación estelar** / *star formation* | `[TIER-D]` | Nubes de gas producen estrellas con gran intensidad y llenan las galaxias de regiones luminosas. // *Gas clouds form stars at high rates and fill galaxies with luminous regions.* | Zero intense blue star-forming cloud knots. | Pulse hot OB starburst HII knots embedded in diffuse gas clouds (`doc_feat==1`). |
| `F2` | **fusión galáctica** / *galaxy merger* | `[TIER-D]` | Las interacciones y fusiones contribuyen a construir sistemas mayores y a transformar sus formas. // *Interactions and mergers help build larger systems and transform their shapes.* | Zero galaxy-galaxy tidal interactions or merger bridges. | Animate two irregular proto-galaxies spiraling together with curved luminous tidal tails (`doc_feat==2`). |
| `F3` | **metallicidad baja** / *low metallicity* | `[TIER-D]` | Las primeras generaciones estelares habían enriquecido menos el gas con elementos pesados. // *The first stellar generations had enriched the gas less with heavy elements.* | Zero chemical enrichment / low-metallicity dust-free color cue. | Compare pristine blue-white H/He clumps vs warm dust-enriched late disk inset (`doc_feat==3`). |
| `F4` | **imagen estirada** / *stretched image* | `[TIER-D]` | La expansión, el corrimiento al rojo y los instrumentos determinan qué parte de su luz podemos detectar. // *Expansion, redshift, and our instruments determine which parts of their light we can detect.* | Zero cosmological `(1+z)` redshift / IR band-shift visualization. | Sweep RGB->IR false-color spectral shift bar showing visible UV stretched into infrared (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP09-Z00-03)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 09 (09_observable_universe) Zones Z00..Z03 (`large-scale-structure`, `deep-universe`, `lookback-time`, `distant-galaxies`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z00->1900`, `Z01->1901`, `Z02->1902`, `Z03->1903` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `cuasar`, `reionizacion`, `cmb`, `horizonte` (exist!), plus `retrospeccion`, `corrimiento`, `oscuras`, `hidrogeno`, `expansion`, `luz`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z00->1900`, `Z01->1901`, `Z02->1902`, `Z03->1903`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z00 (vid=1900, large-scale-structure): F0:cosmic web; F1:nearby universe; F2:expanded scale; F3:light in transit; F4:starting point
     * Z01 (vid=1901, deep-universe): F0:deep field; F1:distance layers; F2:faint galaxy; F3:gravitational lens; F4:evolving universe
     * Z02 (vid=1902, lookback-time): F0:travel time; F1:light-year; F2:redshift; F3:past light cone; F4:without simultaneous present
     * Z03 (vid=1903, distant-galaxies): F0:young galaxy; F1:star formation; F2:galaxy merger; F3:low metallicity; F4:stretched image
4. shaders/compute_universe.wgsl (+ `crates/cosmos_render/src/universe_scene.rs`, `shaders/far_shell.wgsl`)
   - Break the 8..=10 => 2 compute lock in universe_scene.rs:309 for Z04..Z10; gate cmb_alpha in build_far_shell_uniforms to Z>=8 only; suppress star_density in Z07..Z08.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp09-z04-07"></a>
### PKG-ACT2-EXP09-Z04-07: Quasar Era, First Galaxies, Reionization & Cosmic Dark Ages (`z=04..07`) — `Phase 3` · Priority: **High**

**Current State:** Zones `Z04..Z07` (`4` zones, `20` features) | Zone Tiers `0A / 1B / 3C / 0D` | Feature Tiers `0A / 2B / 4C / 14D` | Render path: [`doc_stages.rs#L696-L708`](../../crates/cosmos_render/src/doc_stages.rs#L696-L708) -> [`shaders/hero_doc_cosmos.wgsl#L565-L572`](../../shaders/hero_doc_cosmos.wgsl#L565-L572) — Single static `kind=3, vid=9` CMB + quasar jet (Mode 2 lock + `cmb_alpha=1.0` leak). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `20` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z04` | `quasar-era` | Era de los cuásares / *Quasar era* | **`1904`** | `submode=0..4` (`base_flags` cue mask) | `mode=2` (`Z0..3`), `mode=13`/reion (`Z4..7`), `mode=5`/CMB (`Z8..10`); gate `cmb_alpha` to `Z>=8` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z05` | `first-galaxies` | Primeras galaxias / *First galaxies* | **`1905`** | `submode=0..4` (`base_flags` cue mask) | `mode=2` (`Z0..3`), `mode=13`/reion (`Z4..7`), `mode=5`/CMB (`Z8..10`); gate `cmb_alpha` to `Z>=8` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z06` | `reionization` | Época de reionización / *Reionization epoch* | **`1906`** | `submode=0..4` (`base_flags` cue mask) | `mode=2` (`Z0..3`), `mode=13`/reion (`Z4..7`), `mode=5`/CMB (`Z8..10`); gate `cmb_alpha` to `Z>=8` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z07` | `dark-ages` | Edades oscuras / *dark ages* | **`1907`** | `submode=0..4` (`base_flags` cue mask) | `mode=2` (`Z0..3`), `mode=13`/reion (`Z4..7`), `mode=5`/CMB (`Z8..10`); gate `cmb_alpha` to `Z>=8` | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 09 Z04` `quasar-era` — Era de los cuásares / *Quasar era* (`[TIER-B]` · Target `vid=1904`)
- **Metric (`raw_metric`):** "Núcleos activos · un universo joven y extremadamente energético" (*Active nuclei · a young and extremely energetic universe*) · `range=[0.345, 0.450]`
- **Spoken Macro (ES // EN):** En una época temprana abundaron los cuásares: núcleos galácticos alimentados por agujeros negros supermasivos, visibles a través de distancias inmensas. // *In an early era, quasars abounded: galactic nuclei powered by supermassive black holes, visible over immense distances.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `hero_doc_cosmos.wgsl:569`: `quasar = exp(-length(p.xy)*28) * exp(-abs(p.z)*4.5) * select(0.5, 1.4, z_idx == 4u || z_idx == 6u)` boosts the bright bipolar relativistic jet specifically for `z_idx==4u`! `doc_feat` unused.
- **Audit Gap:** Bipolar relativistic jet highlights cleanly at zone level, but lacks central black-hole event horizon + accretion disk, surrounding faint host galaxy envelope, and quasar-absorption cosmic beacon sightline.
- **Zone 3D Procedural Blueprint:** Up-res `doc_zone==4` centerpiece with relativistic SMBH + glowing Keplerian accretion disk (`F1`) outshining faint outer host galaxy disk (`F3`), bipolar plasma jets (`F2`), and an illuminated absorption line-of-sight beam piercing intervening hydrogen clouds toward Earth (`F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **cuásar** / *quasar* | `[TIER-B]` | Un núcleo galáctico activo tan luminoso que puede superar el brillo combinado de su galaxia anfitriona. // *An active galactic nucleus so luminous that it can outshine its entire host galaxy.* | Intense central bipolar glow outshines surrounding `deep_knots` (`L569-572`). | Pulse ultra-luminous central AGN point source dominating host envelope (`doc_feat==0`). |
| `F1` | **disco de acreción** / *accretion disk* | `[TIER-D]` | Gas muy caliente gira y cae hacia un agujero negro supermasivo, liberando enorme energía antes de cruzar el horizonte. // *Extremely hot gas orbits and falls toward a supermassive black hole, releasing enormous energy before crossing the event horizon.* | No black-hole shadow or hot swirling equatorial accretion disk perpendicular to jet (`L569` is jet beam only). | Add `vid=2` style tilted golden-white swirling accretion torus + black shadow core (`doc_feat==1`). |
| `F2` | **chorro relativista** / *relativistic jet* | `[TIER-B]` | Algunos núcleos lanzan haces de plasma a velocidades cercanas a la de la luz. // *Some active nuclei launch plasma jets at nearly the speed of light.* | Collimated amber/cyan bipolar plasma jet (`exp(-length(p.xy)*28)`, `L569`) boosted `1.4x` for `z_idx==4u`. | Animate helical relativistic plasma shock knots racing outward along +/-Z jet cones (`doc_feat==2`). |
| `F3` | **galaxia anfitriona** / *host galaxy* | `[TIER-D]` | El cuásar no es una estrella aislada: ocupa el centro de una galaxia en crecimiento. // *The quasar is not an isolated star: it occupies the center of a growing galaxy.* | No surrounding turbulent host galaxy starlight halo around AGN core. | Render faint diffuse spiral/irregular host galaxy envelope (`0.15x` brightness) hugging core (`doc_feat==3`). |
| `F4` | **baliza cósmica** / *cosmic beacon* | `[TIER-D]` | Su gran luminosidad permite estudiar gas y materia situados entre el cuásar y nosotros. // *Its great luminosity allows us to study gas and matter located between the quasar and us.* | No foreground gas clouds intersected by quasar sightline beam (`Lyman-alpha` forest beacon). | Draw sharp golden sightline ray piercing 4 translucent gas clouds with spectral absorption notch ticks (`doc_feat==4`). |

#### `Exp 09 Z05` `first-galaxies` — Primeras galaxias / *First galaxies* (`[TIER-C]` · Target `vid=1905`)
- **Metric (`raw_metric`):** "Amanecer cósmico · nacen estrellas, cúmulos y galaxias" (*Cosmic dawn · stars, clusters and galaxies are born*) · `range=[0.450, 0.555]`
- **Spoken Macro (ES // EN):** Las primeras galaxias surgieron de pequeñas concentraciones de materia. Eran menos maduras y ayudaron a transformar el gas neutro del universo temprano. // *The first galaxies arose from small concentrations of matter. They were less mature and helped transform the neutral gas of the early universe.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Generic `vid=9` horizon shell + active central `0.5x` quasar jet (anachronistic here!) + Mode 2 web + `cmb_alpha=1.0`.
- **Audit Gap:** Anachronistic AGN quasar jet and fully evolved cosmic web fail to depict Cosmic Dawn: dark-matter minihalos, pristine massive blue Pop-III stars, hierarchical assembly, and emerging UV shells.
- **Zone 3D Procedural Blueprint:** For `doc_zone==5`, mute the central quasar jet and render clustered indigo dark-matter minihalos (`F1`) igniting pinpoint intense blue-UV Population-III stars (`F0, F2`) merging hierarchically (`F3`) while casting crisp faint violet UV halos (`F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **amanecer cósmico** / *cosmic dawn* | `[TIER-C]` | Es la etapa en la que aparecen las primeras estrellas y galaxias después de las edades oscuras. // *This is the era when the first stars and galaxies appear after the Cosmic Dark Ages.* | Bright `0.5x` quasar jet & dense knots misrepresent first faint pinpoints of starlight ending Dark Ages. | Mute central jet; blink tiny intense blue-white star clusters inside dim dark-matter halos (`doc_feat==0`). |
| `F1` | **halo primordial** / *primordial halo* | `[TIER-D]` | La gravedad concentra materia oscura y gas en pequeñas regiones donde puede comenzar la formación estelar. // *Gravity concentrates dark matter and gas into small regions where star formation can begin.* | No indigo dark-matter minihalos collapsing gas before stars ignite. | Render translucent dark-matter gravity wells collecting infalling hydrogen filaments (`doc_feat==1`). |
| `F2` | **estrella primitiva** / *first-generation star* | `[TIER-D]` | Las primeras estrellas se formaron con hidrógeno, helio y trazas de litio, antes del enriquecimiento químico posterior. // *The first stars formed from hydrogen, helium, and trace amounts of lithium, before later chemical enrichment.* | No pristine massive UV Population-III stars free of heavy-element red/gold dust. | Ignite crisp zero-dust blue-white giant star sparks inside minihalo centers (`doc_feat==2`). |
| `F3` | **ensamblaje** / *assembly* | `[TIER-D]` | Sistemas pequeños se fusionan y alimentan para construir galaxias progresivamente mayores. // *Small systems merge and accrete matter to build progressively larger galaxies.* | No hierarchical accretion / merging of tiny dwarf clumps. | Animate 3-4 tiny glowing proto-halo knots spiraling together (`doc_feat==3`). |
| `F4` | **luz ultravioleta** / *ultraviolet light* | `[TIER-D]` | La radiación de estrellas jóvenes y núcleos activos empieza a cambiar el estado del gas intergaláctico. // *Radiation from young stars and active nuclei begins to change the state of intergalactic gas.* | No nascent UV radiation shells escaping young star clusters. | Emit subtle violet UV ionization halos expanding from first clusters (`doc_feat==4`). |

#### `Exp 09 Z06` `reionization` — Época de reionización / *Reionization epoch* (`[TIER-C]` · Target `vid=1906`)
- **Metric (`raw_metric`):** "Burbujas ionizadas · la luz transforma el hidrógeno neutro" (*Ionized bubbles · light transforms neutral hydrogen*) · `range=[0.555, 0.665]`
- **Spoken Macro (ES // EN):** La radiación de las primeras fuentes abrió burbujas ionizadas en el hidrógeno neutro. Estas regiones crecieron y se unieron hasta volver transparente gran parte del medio intergaláctico. // *Radiation from early sources opened ionized bubbles in neutral hydrogen. These regions grew and joined together until much of the intergalactic medium became transparent.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** **Shader bug** (`hero_doc_cosmos.wgsl:566,569`): `L566` comment explicitly lists `HII Ionization Bubbles`, yet **ZERO** bubble SDFs exist — instead `L569` sets `z_idx == 6u` to boost the **quasar jet** (`1.4x`)!
- **Audit Gap:** Narration explains expanding spherical HII ionized bubbles burning through opaque neutral hydrogen until they overlap and turn the universe transparent — instead rendered as a misplaced central quasar jet!
- **Zone 3D Procedural Blueprint:** Replace `z_idx==6u` quasar boost with multi-center spherical Strömgren bubble field (`bubbles(p, t_cue)`): opaque warm-rust/ochre neutral HI fog (`F1`) punctured by sharp glowing cyan-violet spherical ionization fronts (`F0, F2`) around bright central UV sources that expand and percolate/merge (`F3`) asymmetrically (`F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **burbuja ionizada** / *ionized bubble* | `[TIER-D]` | Una región donde fotones energéticos han separado electrones de átomos de hidrógeno. // *A region where energetic photons have separated electrons from hydrogen atoms.* | Comment `L566` claims HII Ionization Bubbles, yet zero bubble SDFs exist in `L567-572`. | Carve 6-10 sharp luminous spherical transparent cavities (`exp(-abs(d_bub)*42)`) inside HI fog (`doc_feat==0`). |
| `F1` | **hidrógeno neutro** / *neutral hydrogen* | `[TIER-D]` | Fuera de las burbujas, gran parte del gas conserva sus electrones y absorbe ciertas longitudes de onda. // *Outside the bubbles, much of the gas retains its electrons and absorbs certain wavelengths.* | No semi-opaque neutral hydrogen HI fog outside bubbles. | Fill un-ionized regions with warm muted crimson/ochre HI 21cm gas mist (`doc_feat==1`). |
| `F2` | **frente de ionización** / *ionization front* | `[TIER-D]` | El límite entre gas neutro e ionizado avanza alrededor de las primeras fuentes luminosas. // *The boundary between neutral and ionized gas advances around the first light sources.* | No sharp glowing HII boundary wall separating ionized interior from neutral exterior. | Highlight crisp electric-cyan glowing thin skin along each bubble boundary (`doc_feat==2`). |
| `F3` | **solapamiento** / *overlap* | `[TIER-D]` | Las burbujas crecen, se conectan y cambian globalmente el medio intergaláctico. // *The bubbles grow and merge, transforming the intergalactic medium on a global scale.* | No animated bubble growth & percolation/overlap into global transparency. | Drive `r_bubble` smoothly with `doc_cue(inst)` so bubbles kiss, merge, and clear the sky (`doc_feat==3`). |
| `F4` | **transición gradual** / *gradual transition* | `[TIER-C]` | La reionización no ocurrió de golpe ni exactamente al mismo tiempo en todas partes. // *Reionization did not happen all at once or at exactly the same time everywhere.* | `L569` activates `1.4x` central quasar jet on `z_idx == 6u`, turning gradual multi-source patchy reionization into a single axial beam. | Remove axial jet; stagger bubble radii across uneven seeds to show patchy asymmetric reionization (`doc_feat==4`). |

#### `Exp 09 Z07` `dark-ages` — Edades oscuras / *dark ages* (`[TIER-C]` · Target `vid=1907`)
- **Metric (`raw_metric`):** "Antes de las estrellas · gas neutro en un universo en expansión" (*Before the stars · neutral gas in an expanding universe*) · `range=[0.665, 0.765]`
- **Spoken Macro (ES // EN):** Después de liberarse el fondo cósmico y antes de encenderse las primeras estrellas, el universo atravesó una etapa sin fuentes visibles de luz estelar. // *After the cosmic microwave background was released and before the first stars ignited, the universe passed through an era with no visible sources of starlight.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Still renders bright horizon wireframe, active `0.5x` central quasar jet (`L569`), bright `deep_knots`, Mode 2 Cosmic Web filaments, and `25%` background stars (`universe_scene.rs:370`).
- **Audit Gap:** Narrates the cosmic era **before any stars existed** — yet shows an active quasar jet, bright galaxy knots, bright evolved filaments, and background starlight!
- **Zone 3D Procedural Blueprint:** For `doc_zone==7`, extinguish ALL `quasar`, `deep_knots`, and visible galaxy-web emission (`F0, F4`): show ultra-dim dark-matter density ripples (`F2`) bathed in faint cooling neutral H/He gas (`F1`) and toggleable false-color 21cm radio tomography slices (`F3`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **oscuridad cósmica** / *cosmic darkness* | `[TIER-C]` | No significa ausencia de materia o radiación: todavía existe gas, materia oscura y el fondo cósmico enfriándose. // *It does not mean absence of matter or radiation: there is still gas, dark matter and the cooling cosmic background.* | Bright `0.5x` quasar jet & glowing `deep_knots` directly contradict zero visible starlight (`F0`). | Extinguish quasar jet, galaxy knots, and web glow; leave cooling faint amber CMB glow + dark matter (`doc_feat==0`). |
| `F1` | **gas neutro** / *neutral gas* | `[TIER-D]` | El hidrógeno y el helio llenan el espacio mientras pequeñas variaciones de densidad crecen por gravedad. // *Hydrogen and helium fill space while small variations in density grow by gravity.* | No diffuse neutral hydrogen/helium primordial gas mist. | Render subtle faint crimson/slate neutral HI+He gas texture filling the expanding void (`doc_feat==1`). |
| `F2` | **semillas de estructura** / *seeds of structure* | `[TIER-C]` | Las irregularidades iniciales evolucionan hacia los halos que más tarde alojarán estrellas y galaxias. // *The initial irregularities evolve into halos that will later host stars and galaxies.* | Already-collapsed bright `deep_knots` + Mode 2 filaments misrepresent subtle linear `10^-5` primordial fluctuations. | Replace bright knots with gentle smooth Gaussian gravitational potential ripples (`doc_feat==2`). |
| `F3` | **línea de 21 centímetros** / *21-centimeter line* | `[TIER-D]` | La señal del hidrógeno neutro puede ofrecer una futura cartografía de esta etapa difícil de observar. // *The neutral hydrogen signal may offer future mapping of this difficult-to-observe stage.* | No 21cm hydrogen hyperfine radio-wave tomography visualization. | Sweep a false-color cyan/indigo 21cm radio tomography slice across the dark gas cube (`doc_feat==3`). |
| `F4` | **antes del amanecer** / *before dawn* | `[TIER-D]` | La zona termina cuando las primeras fuentes luminosas comienzan a encender el universo. // *The zone ends when the first light sources begin to light up the universe.* | No transition boundary previewing first proto-halos right before Cosmic Dawn. | Flicker tiny nascent golden-blue proto-star sparks inside deepest halos only at `doc_cue > 0.8` (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP09-Z04-07)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 09 (09_observable_universe) Zones Z04..Z07 (`quasar-era`, `first-galaxies`, `reionization`, `dark-ages`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z04->1904`, `Z05->1905`, `Z06->1906`, `Z07->1907` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `cuasar`, `reionizacion`, `cmb`, `horizonte` (exist!), plus `retrospeccion`, `corrimiento`, `oscuras`, `hidrogeno`, `expansion`, `luz`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z04->1904`, `Z05->1905`, `Z06->1906`, `Z07->1907`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z04 (vid=1904, quasar-era): F0:quasar; F1:accretion disk; F2:relativistic jet; F3:host galaxy; F4:cosmic beacon
     * Z05 (vid=1905, first-galaxies): F0:cosmic dawn; F1:primordial halo; F2:first-generation star; F3:assembly; F4:ultraviolet light
     * Z06 (vid=1906, reionization): F0:ionized bubble; F1:neutral hydrogen; F2:ionization front; F3:overlap; F4:gradual transition
     * Z07 (vid=1907, dark-ages): F0:cosmic darkness; F1:neutral gas; F2:seeds of structure; F3:21-centimeter line; F4:before dawn
4. shaders/compute_universe.wgsl (+ `crates/cosmos_render/src/universe_scene.rs`, `shaders/far_shell.wgsl`)
   - Break the 8..=10 => 2 compute lock in universe_scene.rs:309 for Z04..Z10; gate cmb_alpha in build_far_shell_uniforms to Z>=8 only; suppress star_density in Z07..Z08.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp09-z08-10"></a>
### PKG-ACT2-EXP09-Z08-10: CMB Last-Scattering Shell, Cosmological Horizon & Observable Limit (`z=08..10`) — `Phase 3` · Priority: **High**

**Current State:** Zones `Z08..Z10` (`3` zones, `15` features) | Zone Tiers `0A / 3B / 0C / 0D` | Feature Tiers `0A / 7B / 1C / 7D` | Render path: [`doc_stages.rs#L696-L708`](../../crates/cosmos_render/src/doc_stages.rs#L696-L708) -> [`shaders/hero_doc_cosmos.wgsl#L565-L572`](../../shaders/hero_doc_cosmos.wgsl#L565-L572) — Single static `kind=3, vid=9` CMB + quasar jet (Mode 2 lock + `cmb_alpha=1.0` leak). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `15` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z08` | `cmb` | Fondo cósmico de microondas / *Cosmic Microwave Background* | **`1908`** | `submode=0..4` (`base_flags` cue mask) | `mode=2` (`Z0..3`), `mode=13`/reion (`Z4..7`), `mode=5`/CMB (`Z8..10`); gate `cmb_alpha` to `Z>=8` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z09` | `cosmological-horizon` | Horizonte cosmológico / *Cosmological horizon* | **`1909`** | `submode=0..4` (`base_flags` cue mask) | `mode=2` (`Z0..3`), `mode=13`/reion (`Z4..7`), `mode=5`/CMB (`Z8..10`); gate `cmb_alpha` to `Z>=8` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z10` | `observable-limit` | Límite del universo observable / *Boundary of the observable universe* | **`1910`** | `submode=0..4` (`base_flags` cue mask) | `mode=2` (`Z0..3`), `mode=13`/reion (`Z4..7`), `mode=5`/CMB (`Z8..10`); gate `cmb_alpha` to `Z>=8` | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 09 Z08` `cmb` — Fondo cósmico de microondas / *Cosmic Microwave Background* (`[TIER-B]` · Target `vid=1908`)
- **Metric (`raw_metric`):** "Luz fósil · universo de unos 380 000 años" (*Fossil light · universe about 380,000 years old*) · `range=[0.765, 0.875]`
- **Spoken Macro (ES // EN):** El fondo cósmico de microondas es la luz electromagnética más antigua que podemos observar directamente. Sus pequeñas variaciones conservan las semillas de la estructura posterior. // *The cosmic microwave background is the oldest electromagnetic light that we can directly observe. Its small variations preserve the seeds of the later structure.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `far_shell.wgsl:116-137` (`cmb_alpha=1.0` at `u=26.4`) renders spherical harmonic + fBm Planck temperature anisotropy map around the entire sky, framed by `vid=9` geodesic sphere (`r=0.76`). Still polluted by Mode 2 web particles & `0.5x` quasar jet.
- **Audit Gap:** Planck CMB anisotropy background & omnidirectional sphere match `F2, F3, F4`, yet foreground Cosmic Web filaments & quasar jet don't belong here, and Thomson scattering / recombination transitions aren't shown.
- **Zone 3D Procedural Blueprint:** Inside `doc_zone==8`, fade out Mode 2 cosmic web particles and `vid=9` quasar/knots, wrap the `r=0.76` last-scattering shell with authentic Planck dipole-free anisotropy patches (`F2, F3`) + animated hot-plasma electron scattering clearing out at `r=0.76` (`F0, F1`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **última dispersión** / *last scattering* | `[TIER-D]` | La luz dejó de rebotar continuamente cuando el plasma se enfrió y los electrones se unieron a los núcleos. // *Light stopped scattering continuously when the plasma cooled and electrons combined with atomic nuclei.* | No Thomson scattering photon-to-last-scattering transition at `r=0.76` wall. | Show blinding hot plasma fog beyond `r>0.76` emitting redshifted photons inward at `r=0.76` (`doc_feat==0`). |
| `F1` | **recombinación** / *recombination* | `[TIER-D]` | El universo se volvió mayormente transparente a los fotones que hoy detectamos como microondas. // *The universe became largely transparent to the photons that we detect today as microwaves.* | No electron + proton recombination flash clearing free-electron opacity. | Render ionized -> neutral transition ring along shell interior (`doc_feat==1`). |
| `F2` | **anisotropía** / *anisotropy* | `[TIER-B]` | Diferencias diminutas de temperatura indican pequeñas variaciones de densidad en el universo temprano. // *Tiny temperature differences indicate small density variations in the early universe.* | Deep Planck indigo-slate-amber temperature fluctuations rendered sky-wide (`far_shell.wgsl:116-136`). | Project high-contrast delta-T/T (`±200 uK`) Planck patches directly onto `r=0.76` apparent shell (`doc_feat==2`). |
| `F3` | **superficie aparente** / *apparent surface* | `[TIER-B]` | La vemos alrededor de nosotros en todas direcciones, como una capa de tiempo pasado, no como una esfera material centrada en la Tierra. // *We see it in every direction as a layer from the distant past, not as a physical sphere centered on Earth.* | `vid=9` omnidirectional `r=0.76` shell centered on observer + full-sky `far_shell` illustrates apparent surface (`L567-568`). | Pulse concentric time-shell shell highlighting observer-centered geometry without physical wall (`doc_feat==3`). |
| `F4` | **luz estirada** / *stretched light* | `[TIER-B]` | La expansión transformó aquella radiación caliente en el tenue fondo de microondas actual. // *The expansion transformed that hot radiation into today's faint microwave background.* | Cool deep-Planck microwave tint (`far_shell.wgsl:128-135`) conveys stretched radiation. | Animate 3000 K amber glow stretching smoothly along wavelets into cold microwave indigo/amber (`doc_feat==4`). |

#### `Exp 09 Z09` `cosmological-horizon` — Horizonte cosmológico / *Cosmological horizon* (`[TIER-B]` · Target `vid=1909`)
- **Metric (`raw_metric`):** "Límite causal · solo llega la luz con tiempo para alcanzarnos" (*Causal boundary · only light with enough travel time can reach us*) · `range=[0.875, 0.955]`
- **Spoken Macro (ES // EN):** El horizonte cosmológico marca una frontera de observabilidad, no una pared. Depende de la historia de expansión y de cuánto tiempo ha podido viajar la luz. // *The cosmological horizon marks a boundary of observability, not a wall. It depends on the expansion history and how long light has been able to travel.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=9` (`L567-571`) explicitly renders translucent open geodesic wireframe sphere `shell (r=0.76) + meridians` representing particle horizon (`F0, F3`), though foreground `deep_knots`, `0.5x` quasar jet, and Mode 2 web particles distract.
- **Audit Gap:** Open wireframe geodesic horizon captures 'causal observability boundary, not a solid wall' (`F0, F3`), yet misses comoving expansion scale, beyond-horizon causal separation, and pre-CMB inference.
- **Zone 3D Procedural Blueprint:** For `doc_zone==9`, clean out Mode 2 web + quasar jet, double-shell the horizon (`r=0.72` EM CMB surface vs `r=0.82` true particle horizon), show faint galaxies continuing *outside* `r=0.82` cut off by causal light-cones (`F1, F4`), and illustrate comoving expansion (`F2`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **horizonte de partículas** / *particle horizon* | `[TIER-B]` | Es la máxima distancia desde la que una señal pudo alcanzarnos desde el comienzo de la evolución cósmica observable. // *It is the greatest distance from which a signal could have reached us since observable cosmic evolution began.* | Geodesic spherical boundary `r=0.76` explicitly represents maximum particle-horizon reach (`L567-568`). | Trace outward/inward causal signal ray from horizon shell `r=0.82` to central observer (`doc_feat==0`). |
| `F1` | **límite causal** / *causal boundary* | `[TIER-C]` | Regiones más allá no han tenido tiempo de enviarnos información, aunque puedan formar parte de un universo mayor. // *Regions beyond have not had time to send us information, although they may be part of a larger universe.* | Bright `deep_knots` stop inside `0.58..0.78` (`L570`), giving false impression space ends at horizon instead of unreached exterior regions. | Render ghosted dim galaxies continuing at `r > 0.85` with broken/blocked light-rays unable to reach center (`doc_feat==1`). |
| `F2` | **expansión cósmica** / *cosmic expansion* | `[TIER-D]` | El espacio cambió mientras la luz viajaba; por eso distancia actual y tiempo de viaje no son la misma cifra. // *Space expanded while the light traveled; that is why present-day distance and light-travel time are not the same number.* | No visual difference between light-travel distance and expanding proper distance. | Stretch grid meridians outward while photon arc travels inward, comparing path lengths (`doc_feat==2`). |
| `F3` | **no es una pared** / *not a wall* | `[TIER-B]` | No encontraríamos una superficie sólida al viajar hacia allí; el horizonte depende del observador y del tiempo. // *We would find no solid surface by traveling toward it; the horizon depends on the observer and the time of observation.* | Open porous latitude/longitude wireframe (`meridians`, `L568`) visually proves horizon is not a solid wall. | Pan/pulse hollow translucent wireframe shell showing space passes unobstructed through it (`doc_feat==3`). |
| `F4` | **más allá del fondo** / *beyond the background* | `[TIER-D]` | Podemos inferir física anterior al fondo cósmico, pero no verla con luz electromagnética ordinaria. // *We can infer physics prior to the cosmic background, but not see it with ordinary electromagnetic light.* | No layered inner-vs-outer gap separating EM CMB (`r=0.72`) from true particle horizon (`r=0.82`, neutrino/GW). | Highlight opaque fog shell between `r=0.72` (CMB) and `r=0.82` (particle horizon) crossed only by GW wavelets (`doc_feat==4`). |

#### `Exp 09 Z10` `observable-limit` — Límite del universo observable / *Boundary of the observable universe* (`[TIER-B]` · Target `vid=1910`)
- **Metric (`raw_metric`):** "Borde de nuestro mapa · no borde físico del universo" (*Edge of our map · not a physical edge of the universe*) · `range=[0.955, 1.000]`
- **Spoken Macro (ES // EN):** Aquí termina lo que puede mostrarnos la luz disponible. El universo completo puede extenderse mucho más, pero esa región no es actualmente visible de forma directa. // *Here ends what the available light can show us. The entire universe may extend much further, but that region is not currently directly visible.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=9` hollow geodesic envelope + outer Planck CMB `far_shell` illustrates finite observable map boundary (`F0, F1`).
- **Audit Gap:** Translucent geodesic edge matches 'edge of our map, not edge of space', yet misses finite cosmic age clock, opaque pre-CMB plasma shroud, and indirect GW/abundance probes.
- **Zone 3D Procedural Blueprint:** Keep clean dual-boundary observable globe (`doc_zone==10`): render faint fading cosmic grid extending seamlessly beyond `r=0.82` (`F1`), age-clock gauge (`13.8 Gyr`, `F2`), opaque warm glow wall blocking light inside `0.72..0.82` (`F3`), and quadrupole gravitational wave ripples / D/He badges piercing it (`F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **universo observable** / *observable universe* | `[TIER-B]` | Es la región desde la que alguna señal ha podido llegar hasta nosotros durante la historia cósmica. // *It is the region from which some signal has been able to reach us during cosmic history.* | Observer-centered horizon sphere encloses visible galaxies (`deep_knots`) and web (`L567-570`). | Highlight entire interior sphere volume from center out to `r=0.76` (`doc_feat==0`). |
| `F1` | **sin borde físico** / *no physical boundary* | `[TIER-B]` | El límite pertenece a nuestra observación; no demuestra que el espacio termine allí. // *The limit belongs to our observation; it does not prove that space ends there.* | Transparent wireframe & open `far_shell` exterior avoid portraying a hard physical brick wall (`L568`). | Extend faint coordinate grid lines beyond `r=0.82` into deep dark space (`doc_feat==1`). |
| `F2` | **edad finita** / *finite age* | `[TIER-D]` | La historia cósmica disponible impone un tiempo máximo para el viaje de cualquier señal. // *Available cosmic history imposes a maximum travel time for any signal.* | No 13.8 Gyr finite cosmic age limit / maximum photon path counter. | Render concentric `13.8 Gyr` maximum causal arc limit (`doc_feat==2`). |
| `F3` | **plasma opaco** / *opaque plasma* | `[TIER-D]` | Antes de la última dispersión, la luz electromagnética no podía propagarse libremente a través del plasma primordial. // *Before the last scattering, electromagnetic light could not propagate freely through the primordial plasma.* | No opaque hot primordial plasma shell blocking photons beyond CMB. | Tint thin `0.74 < r < 0.82` zone as glowing opaque plasma wall reflecting light inward (`doc_feat==3`). |
| `F4` | **conocimiento indirecto** / *indirect knowledge* | `[TIER-D]` | Modelos, abundancias, ondas gravitacionales futuras y otras huellas pueden informar sobre épocas que la luz no muestra directamente. // *Models, elemental abundances, possible future gravitational-wave observations, and other traces can reveal eras that light does not show directly.* | No indirect evidence layer (BBN elemental gauges or primordial gravitational-wave ripples). | Send faint quadrupolar spacetime wavelets cleanly through opaque plasma boundary to observer (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP09-Z08-10)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 09 (09_observable_universe) Zones Z08..Z10 (`cmb`, `cosmological-horizon`, `observable-limit`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z08->1908`, `Z09->1909`, `Z10->1910` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `cuasar`, `reionizacion`, `cmb`, `horizonte` (exist!), plus `retrospeccion`, `corrimiento`, `oscuras`, `hidrogeno`, `expansion`, `luz`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z08->1908`, `Z09->1909`, `Z10->1910`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z08 (vid=1908, cmb): F0:last scattering; F1:recombination; F2:anisotropy; F3:apparent surface; F4:stretched light
     * Z09 (vid=1909, cosmological-horizon): F0:particle horizon; F1:causal boundary; F2:cosmic expansion; F3:not a wall; F4:beyond the background
     * Z10 (vid=1910, observable-limit): F0:observable universe; F1:no physical boundary; F2:finite age; F3:opaque plasma; F4:indirect knowledge
4. shaders/compute_universe.wgsl (+ `crates/cosmos_render/src/universe_scene.rs`, `shaders/far_shell.wgsl`)
   - Break the 8..=10 => 2 compute lock in universe_scene.rs:309 for Z04..Z10; gate cmb_alpha in build_far_shell_uniforms to Z>=8 only; suppress star_density in Z07..Z08.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

---

### Exp 10: `10_early_universe` — Universo Temprano / Early Universe (`9` Zones, `46` Features, `0` Landmarks)

> *Espectro del Universo Temprano: del Fondo Cósmico a la Inflación* — Spectrum of the Early Universe: from the Cosmic Microwave Background to Inflation

- **Odyssey Position (`odyssey.rs#L29-L32`):** Act 2, Position **8 / 8** (Chapter `act2_exp10`)
- **Spatial & Scale Band:** `SectorId::EARLY_INFLATION` · `u ∈ [26.6, 27.2]` · `focus_scale_exp = 26.9` (`topology.rs:193`)
- **Compute & Far Shell:** **CRITICAL UNWIRED BUG (`universe_scene.rs:309`)**: `8..=10 => 2` runs Cosmic Web GALAXY FILAMENTS across Recombination, BBN, Leptons, Quarks, Electroweak, Inflation & Planck! Meanwhile **Mode 3 (QCD Quark-Gluon, `compute_universe.wgsl:162-187`)** and **Mode 5 (Planck Spin-Foam, `compute_universe.wgsl:209-222`)** already exist and are UNWIRED! · **ANACHRONISTIC OVERLAP BUG (`universe_scene.rs:351-370`)**: `cmb_alpha = 1.0`, `inflation_alpha ≈ 0.58`, AND `star_density = 0.25` (rendered stars!) persist across ALL 9 pre-stellar zones (`Z00..Z08`)!
- **Current Render Dispatch:** `doc_stages.rs:696-708` emits single static `kind=3, vid=10` (`hero_doc_cosmos.wgsl:573`: a single thin golden expansion shock shell, falling short on every epoch from Recombination (`Z01`) and BBN (`Z03`) to QCD/Electroweak (`Z05..Z06`), Inflation (`Z07`, 6 features!), and Planck boundary (`Z08`)).
- **Granular Audit Tally:** Zones `0A / 2B / 4C / 3D` (`9`) · Features `0A / 6B / 8C / 32D` (`46`)
- **`DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`) Needed Additions:** `cmb`, `inflacion`, `quark`, `higgs`, `planck`, `horizonte` (exist!), plus `recombinacion`, `plasma`, `nucleosintesis`, `lepton`, `electrodebil`, `cuantica`, `singularidad`

<a id="pkg-act2-exp10-z00-04"></a>
### PKG-ACT2-EXP10-Z00-04: CMB Stretched Sky, Recombination Fog, Plasma Acoustic Oscillations, BBN & Leptons (`z=00..04`) — `Phase 3 + Phase 4` · Priority: **Critical**

**Current State:** Zones `Z00..Z04` (`5` zones, `25` features) | Zone Tiers `0A / 1B / 1C / 3D` | Feature Tiers `0A / 3B / 3C / 19D` | Render path: [`doc_stages.rs#L696-L708`](../../crates/cosmos_render/src/doc_stages.rs#L696-L708) -> [`shaders/hero_doc_cosmos.wgsl#L573`](../../shaders/hero_doc_cosmos.wgsl#L573) — Single static `kind=3, vid=10` expansion ring (Mode 2 lock; Modes 3/5 unused). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `25` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z00` | `cmb` | Fondo cósmico de microondas / *Cosmic Microwave Background* | **`2000`** | `submode=0..4` (`base_flags` cue mask) | `mode=13` / decoupling scatter, `star_density=0.0`, `cmb_alpha` fade `Z0->Z1` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z01` | `recombination` | Recombinación / *Recombination* | **`2001`** | `submode=0..4` (`base_flags` cue mask) | `mode=13` / decoupling scatter, `star_density=0.0`, `cmb_alpha` fade `Z0->Z1` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z02` | `primordial-plasma` | Plasma primordial / *primordial plasma* | **`2002`** | `submode=0..4` (`base_flags` cue mask) | `mode=13`/plasma acoustic (`Z2..4`), `star_density=0.0`, `cmb_alpha=0.0` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z03` | `nucleosynthesis` | Nucleosíntesis primordial / *Big Bang nucleosynthesis* | **`2003`** | `submode=0..4` (`base_flags` cue mask) | `mode=13`/plasma acoustic (`Z2..4`), `star_density=0.0`, `cmb_alpha=0.0` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z04` | `lepton-era` | Era de los leptones / *Lepton Epoch* | **`2004`** | `submode=0..4` (`base_flags` cue mask) | `mode=13`/plasma acoustic (`Z2..4`), `star_density=0.0`, `cmb_alpha=0.0` | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 10 Z00` `cmb` — Fondo cósmico de microondas / *Cosmic Microwave Background* (`[TIER-C]` · Target `vid=2000`)
- **Metric (`raw_metric`):** "≈ 380 000 años · la luz electromagnética más antigua observable" (*≈ 380,000 years · oldest observable electromagnetic light*) · `range=[0.000, 0.090]`
- **Spoken Macro (ES // EN):** El recorrido comienza exactamente en la superficie de última dispersión: un mapa de pequeñas variaciones de temperatura, no una pared ni el borde del universo. // *The journey begins exactly at the surface of last scattering: a map of small temperature variations, not a wall or the edge of the universe.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `doc_stages.rs:696-708` routes ALL Exp 10 zones to `kind=3, vid=10` (`hero_doc_cosmos.wgsl:596-605`: glowing central `core = exp(-r*15)` fireball + `shell_ring r=0.56` + rotating `qgp_rot` filaments), PLUS **Mode 2 Cosmic Web particles** (`universe_scene.rs:309`) AND overlapping `cmb_alpha=1.0` + `inflation_alpha~0.58` (`universe_scene.rs:351-353`).
- **Audit Gap:** `far_shell` CMB anisotropies match `F0, F1`, yet Exp 10's `vid=10` puts an anachronistic bright fireball `core` & spinning QGP filaments in front, zero E/B polarization vectors, and overlapping Cosmic Web particles + inflation caustics.
- **Zone 3D Procedural Blueprint:** Dedicated `doc_zone==0` CMB stage: extinguish `core`, `qgp_rot`, Mode 2 web particles, and `inflation_alpha`; wrap `r=0.68` sphere in clean Planck delta-T anisotropy texture (`F1`), E-mode & B-mode polarization headless tick marks (`F2`), and inward light-scattering decoupling rays (`F3`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **luz fósil** / *fossil light* | `[TIER-B]` | La expansión estiró esta radiación hasta las microondas que llenan el cielo actual. // *The expansion stretched this radiation to the microwaves that fill the sky today.* | `far_shell` `cmb_alpha=1.0` (`universe_scene.rs:351`) shows stretched cold microwave Planck sky (`far_shell.wgsl:116-136`). | Sweep Planck microwave shell from warm 3000K glow down to 2.7K blue/amber (`doc_feat==0`). |
| `F1` | **anisotropía** / *anisotropy* | `[TIER-B]` | Diferencias diminutas de temperatura revelan pequeñas variaciones de densidad en el universo joven. // *Tiny temperature differences reveal small density variations in the young universe.* | `far_shell` Planck multipole + fBm anisotropies visible across sphere (`L119-135`). | Accentuate acoustic peak hot/cold density spots directly on hero CMB shell (`doc_feat==1`). |
| `F2` | **polarización** / *polarization* | `[TIER-D]` | La orientación de la luz conserva información adicional sobre su último encuentro con la materia. // *The orientation of light preserves additional information about its last encounter with matter.* | Zero E-mode / B-mode electromagnetic polarization vector sticks on CMB surface. | Overlay crisp tangential/radial E-mode and curl B-mode polarization dashes over hotspots (`doc_feat==2`). |
| `F3` | **última dispersión** / *last scattering* | `[TIER-C]` | Los fotones dejaron de chocar continuamente con electrones libres y comenzaron a viajar grandes distancias. // *The photons stopped continually colliding with free electrons and began to travel long distances.* | `vid=10` shows spinning QGP fireball `core + shell_ring` instead of Thomson last-scattering surface. | Animate scattered photon zig-zags breaking straight free at `r=0.68` sphere (`doc_feat==3`). |
| `F4` | **punto de partida** / *starting point* | `[TIER-D]` | Desde aquí el viaje deja de avanzar hacia objetos lejanos y comienza a reconstruir épocas anteriores mediante huellas físicas. // *From here, the journey stops moving toward more distant objects and begins reconstructing earlier eras from physical evidence.* | No transition cue from spatial map back into microphysics time-reversal. | Animate inward epoch scale ruler (`380 kyr -> 10^-32 s`, `doc_feat==4`). |

#### `Exp 10 Z01` `recombination` — Recombinación / *Recombination* (`[TIER-D]` · Target `vid=2001`)
- **Metric (`raw_metric`):** "≈ 380 000 años · ≈ 3 000 K · el universo se vuelve transparente" (*≈ 380,000 years · ≈ 3,000 K · the universe becomes transparent*) · `range=[0.090, 0.200]`
- **Spoken Macro (ES // EN):** Electrones y núcleos pudieron formar átomos neutros, sobre todo hidrógeno. Al disminuir los electrones libres, la luz dejó de quedar atrapada en el plasma. // *Electrons and nuclei were able to form neutral atoms, especially hydrogen. As the free electrons decreased, light stopped being trapped in the plasma.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Generic `vid=10` spinning plasma fireball `core + qgp_rot + shell_ring (r=0.56)` + Cosmic Web particles; zero atoms, hydrogen binding, photon decoupling, or spatial clearing-out transition.
- **Audit Gap:** Narration describes electrons capturing onto protons to form neutral H/He at 3000K and clearing fog uniformly across space (explicitly *not* expanding from one edge) — visually contradicted by fixed `r=0.56` bubble shell + fireball core!
- **Zone 3D Procedural Blueprint:** Build `doc_zone==1` volumetric recombination diorama: zoom-in cluster of ionized protons (amber) & free electrons (cyan) scattering bouncing photon rays (`F2`) morphing via `doc_cue` into neutral hydrogen/helium orbital atoms (`F0, F1`) as global volumetric fog dissolves everywhere simultaneously (`F3`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **átomo neutro** / *neutral atom* | `[TIER-D]` | Un electrón ligado a un núcleo ya no dispersa la luz del mismo modo que un electrón libre abundante. // *An electron bound to a nucleus no longer scatters light as the many free electrons in the plasma did.* | Zero electron-binding-to-nucleus orbital atom visual. | Show free cyan electrons snapping into spherical quantum orbitals around amber nuclei (`doc_feat==0`). |
| `F1` | **hidrógeno** / *hydrogen* | `[TIER-D]` | La mayor parte de la materia ordinaria quedó en átomos sencillos de hidrógeno y helio. // *Most ordinary matter ended up in simple hydrogen and helium atoms.* | Zero hydrogen (1p) vs helium (2p+2n) atomic abundance visuals. | Contrast ~75% H single-proton orbits vs ~25% He four-nucleon clusters (`doc_feat==1`). |
| `F2` | **desacoplamiento** / *decoupling* | `[TIER-D]` | Materia y radiación dejaron de intercambiar energía de manera tan intensa; los fotones pudieron propagarse casi libremente. // *Matter and radiation stopped exchanging energy so intensely; the photons were able to propagate almost freely.* | Zero photon-matter decoupling trajectory transition. | Morph zigzagging trapped photon paths into straight ballistic rays (`doc_feat==2`). |
| `F3` | **universo transparente** / *transparent universe* | `[TIER-D]` | La transparencia no apareció desde un borde: ocurrió en todo el espacio observable a medida que el plasma se enfrió. // *The transparency did not appear from one edge: it occurred throughout observable space as the plasma cooled.* | Global fog-clearing physics completely absent (`qgp_rot` stays bright rotating plasma). | Dissolve golden plasma haze uniformly across entire volume into crystal-clear void (`doc_feat==3`). |
| `F4` | **superficie aparente** / *apparent surface* | `[TIER-C]` | Cada dirección del cielo muestra el lugar lejano donde sus fotones sufrieron su última dispersión. // *Each direction of the sky shows the distant location where its photons were last scattered.* | Hard `shell_ring` (`r=0.56`) + central `core` falsely implies transparency spreading radially from a center. | Remove `core` & single-center shell; replace with omnidirectional past time-slice reticle (`doc_feat==4`). |

#### `Exp 10 Z02` `primordial-plasma` — Plasma primordial / *primordial plasma* (`[TIER-B]` · Target `vid=2002`)
- **Metric (`raw_metric`):** "Antes de 380 000 años · materia ionizada · luz atrapada" (*Before 380,000 years · ionized matter · trapped light*) · `range=[0.200, 0.340]`
- **Spoken Macro (ES // EN):** Antes de recombinarse, el universo era una mezcla caliente de núcleos, electrones y fotones. La dispersión constante hacía el medio opaco a la luz electromagnética. // *Before recombination, the universe was a hot mixture of atomic nuclei, electrons, and photons. Constant scattering made the medium opaque to electromagnetic light.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=10` rotating opaque filamentary plasma turbulence (`qgp_rot`, `L599-600`) evokes hot dense primordial plasma (`macro/F0` loosely), YET `core = exp(-r*15)` **directly contradicts F1** ('not an explosion from a central location'), and acoustic BAO oscillations (`F3`) + photon random walk (`F2`) are absent.
- **Audit Gap:** Dense swirling plasma evokes a hot ionized soup, but the bright central `core` dot turns the Hot Big Bang into a central fireball explosion (explicitly denied by F1!) and omits photon-baryon acoustic waves.
- **Zone 3D Procedural Blueprint:** Remove central `core` from `doc_zone==2` so plasma fills whole volume uniformly; scatter fast free-electron & random-walk photon sparks (`F0, F2`), and superimpose standing **Baryon Acoustic Oscillation (BAO)** spherical pressure sound-wave shells (`F3`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **electrón libre** / *free electron* | `[TIER-D]` | Las cargas libres dispersaban los fotones una y otra vez, como una niebla extremadamente densa. // *The free charges scattered the photons over and over again, like an extremely dense fog.* | No free-electron / scattering charge particles distinct from `qgp_rot` filaments. | Spark bright cyan free electrons colliding with short glowing photon segments (`doc_feat==0`). |
| `F1` | **Big Bang caliente** / *Hot Big Bang* | `[TIER-C]` | El Big Bang describe una fase temprana, caliente y densa y la expansión del propio espacio, no una explosión desde un lugar central. // *The Big Bang describes an early, hot, dense phase and expansion of space itself, not an explosion from a central location.* | **Active contradiction**: `core = exp(-r*15.0)` (`L601`) draws an explosive fireball center, directly contradicting F1 narration ('not an explosion from a central location')! | Eliminate radial `exp(-r*15)` core; replace with uniform infinite-grid metric stretch across whole plasma (`doc_feat==1`). |
| `F2` | **fotón dispersado** / *scattered photon* | `[TIER-D]` | La luz recorría distancias cortas entre colisiones; no podía entregarnos una fotografía directa de épocas más tempranas. // *Light traveled only short distances between collisions, so it cannot give us a direct image of earlier eras.* | No short-mean-free-path photon random-walk collision trails. | Draw jagged short Brownian photon zig-zag segments bouncing inside dense fog (`doc_feat==2`). |
| `F3` | **oscilación acústica** / *acoustic oscillation* | `[TIER-D]` | Presión de radiación y gravedad produjeron ondas en el plasma; sus huellas aparecen en el fondo cósmico y la distribución de materia. // *Radiation pressure and gravity produced waves in the plasma; their traces appear in the cosmic background and the distribution of matter.* | No standing baryon acoustic sound-wave pressure ripples in plasma. | Overlay subtle concentric spherical BAO compression wave rings oscillating (`F3`: `sin(r*18 - time*3)`). |
| `F4` | **sin imagen directa** / *no direct image* | `[TIER-B]` | La opacidad marca el límite de la astronomía basada en luz, no el límite de toda evidencia física. // *Opacity marks the limit of light-based astronomy, not the limit of all physical evidence.* | Opaque glowing swirling plasma cloud (`qgp_rot`, `L599-603`) conveys electromagnetic opacity visually. | Dim optical transmission behind dense plasma fog (`F4`). |

#### `Exp 10 Z03` `nucleosynthesis` — Nucleosíntesis primordial / *Big Bang nucleosynthesis* (`[TIER-D]` · Target `vid=2003`)
- **Metric (`raw_metric`):** "Primeros minutos · se forman núcleos ligeros" (*First minutes · light nuclei form*) · `range=[0.340, 0.470]`
- **Spoken Macro (ES // EN):** Al enfriarse el universo, protones y neutrones pudieron unirse brevemente para producir deuterio, helio y pequeñas cantidades de litio. // *As the universe cooled, protons and neutrons were able to join together briefly to produce deuterium, helium, and small amounts of lithium.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Same `vid=10` fiery plasma core + Mode 2 web particles + CMB haze; zero protons, neutrons, deuterium bottleneck, helium-4 fusion, or lithium traces.
- **Audit Gap:** First 3-minute nuclear fusion reactions (75% H, ~25% He, trace D & Li-7) have zero nucleon or nuclear reaction representation on screen.
- **Zone 3D Procedural Blueprint:** Build `doc_zone==3` BBN nuclear reaction diorama: red/gold **protons** (`p+`) and slate-blue **neutrons** (`n0`) (`F0, F1`) fusing across brief **deuterium** (`d`) flashes (`F2`) into ~25% by mass tight 4-nucleon **He-4** tetrahedra (`F3`) plus one blinking **Li-7** cluster with `1/3` observational tension badge (`F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **protón** / *proton* | `[TIER-D]` | Un protón es el núcleo del hidrógeno común y uno de los componentes básicos de los núcleos más pesados. // *A proton is the nucleus of ordinary hydrogen and one of the basic components of heavier atomic nuclei.* | Zero proton sphere / quark-triplet nucleon representation. | Render glowing amber proton spheres (dominant 7:1 ratio over neutrons, `doc_feat==0`). |
| `F1` | **neutrón** / *neutron* | `[TIER-D]` | Los neutrones disponibles se incorporaron principalmente al helio antes de que muchos pudieran desintegrarse. // *The available neutrons were mainly incorporated into helium before many could decay.* | Zero neutron spheres or beta-decay freeze-out flash. | Render slate-blue neutrons gradually decaying or capturing before freeze-out (`doc_feat==1`). |
| `F2` | **deuterio** / *deuterium* | `[TIER-D]` | Este isótopo del hidrógeno es una prueba sensible de la densidad de materia ordinaria durante los primeros minutos. // *This isotope of hydrogen is a sensitive test of the density of ordinary matter during the first minutes.* | Zero proton+neutron deuterium bridge synthesis. | Animate `p + n -> D + gamma` glowing pair-binding flash (`doc_feat==2`). |
| `F3` | **helio primordial** / *primordial helium* | `[TIER-D]` | Una fracción importante de la masa de materia ordinaria terminó como helio, antes de que existiera ninguna estrella. // *A significant fraction of the mass of ordinary matter ended up as helium, before any stars existed.* | Zero 4-nucleon He-4 tetrahedral alpha nuclei (~25% mass fraction). | Assemble fast-fusing 2p+2n tetrahedral He-4 clusters glowing gold (`doc_feat==3`). |
| `F4` | **litio primordial** / *primordial lithium* | `[TIER-D]` | Se produjo en trazas; su abundancia observada todavía presenta una tensión conocida con predicciones sencillas. // *It was produced in trace amounts; its observed abundance still shows a known discrepancy with simple predictions.* | Zero trace primordial Lithium-7 cluster or prediction-tension indicator. | Display rare isolated 3p+4n Li-7 nucleus pulsing beside abundance discrepancy halo (`doc_feat==4`). |

#### `Exp 10 Z04` `lepton-era` — Era de los leptones / *Lepton Epoch* (`[TIER-D]` · Target `vid=2004`)
- **Metric (`raw_metric`):** "Alrededor del primer segundo · leptones, fotones y neutrinos" (*Around the first second · leptons, photons, and neutrinos*) · `range=[0.470, 0.590]`
- **Spoken Macro (ES // EN):** A temperaturas aún mayores, electrones, positrones, neutrinos y fotones dominaban gran parte del contenido energético mientras el universo seguía expandiéndose y enfriándose. // *At even higher temperatures, electrons, positrons, neutrinos and photons dominated much of the energy content as the universe continued to expand and cool.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** Same `vid=10` fireball plasma sphere + Mode 2 web particles; zero electrons, positrons, e⁻e⁺ annihilation flashes, relic neutrinos, or antimatter 1-in-a-billion asymmetry.
- **Audit Gap:** Whole ~1s relativistic lepton soup (`e⁻, e⁺, ν, γ` pair creation/annihilation & matter-antimatter asymmetry) has zero visual metaphor.
- **Zone 3D Procedural Blueprint:** Build `doc_zone==4` QED thermal bath: cyan `e⁻` & magenta `e⁺` pairs spiraling into opposing tracks and annihilating into twin golden `gamma` rays (`F0, F1`), asymmetric 10:9 -> 1 survivor `e⁻` highlight (`F3`), and faint ghostly uncolliding straight-line `neutrino` streaks (`F2, F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **electrón** / *electron* | `[TIER-D]` | Es un leptón con carga eléctrica; más tarde quedaría ligado a núcleos para formar átomos. // *It is an electrically charged lepton; later, electrons would bind to atomic nuclei to form atoms.* | Zero charged electron orbital/track visuals. | Render bright cyan `e⁻` particle swirls deflected by EM field lines (`doc_feat==0`). |
| `F1` | **positrón** / *positron* | `[TIER-D]` | Es la antipartícula del electrón. Pares de electrones y positrones podían crearse y aniquilarse en el baño caliente. // *It is the antiparticle of the electron. Pairs of electrons and positrons could be created and annihilated in the hot bath.* | Zero positron antiparticle creation/annihilation pairs. | Pair each `e⁻` with mirrored magenta `e⁺` track meeting in bright dual-photon `gamma` burst (`doc_feat==1`). |
| `F2` | **neutrino** / *neutrino* | `[TIER-D]` | Interactúa muy débilmente. Un fondo relicto de neutrinos es una predicción importante, aunque todavía no se ha observado directamente. // *It interacts very weakly. A relict neutrino background is an important prediction, although it has not yet been directly observed.* | Zero ghostly weakly interacting neutrino straight trajectories. | Stream translucent violet neutrinos passing clean through dense plasma without scattering (`doc_feat==2`). |
| `F3` | **antimateria** / *antimatter* | `[TIER-D]` | Partículas y antipartículas se transformaban continuamente en radiación; una pequeña asimetría dejó la materia que vemos hoy. // *Particles and antiparticles were continually transformed into radiation; a small asymmetry left the matter we see today.* | Zero matter-antimatter asymmetry survival visualization. | Freeze-flash symmetric pair annihilation leaving slight excess cyan matter particles (`doc_feat==3`). |
| `F4` | **desacoplamiento de neutrinos** / *neutrino decoupling* | `[TIER-D]` | Al caer la temperatura, los neutrinos dejaron de interactuar con frecuencia y continuaron propagándose como población relicta. // *As the temperature fell, neutrinos stopped interacting frequently and continued to spread as a relict population.* | Zero neutrino thermal decoupling wave at `T ~ 1 MeV`. | Split straight ballistic neutrino shell free from tangled EM e⁻/gamma bath (`doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP10-Z00-04)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 10 (10_early_universe) Zones Z00..Z04 (`cmb`, `recombination`, `primordial-plasma`, `nucleosynthesis`, `lepton-era`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z00->2000`, `Z01->2001`, `Z02->2002`, `Z03->2003`, `Z04->2004` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `cmb`, `inflacion`, `quark`, `higgs`, `planck`, `horizonte` (exist!), plus `recombinacion`, `plasma`, `nucleosintesis`, `lepton`, `electrodebil`, `cuantica`, `singularidad`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z00->2000`, `Z01->2001`, `Z02->2002`, `Z03->2003`, `Z04->2004`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z00 (vid=2000, cmb): F0:fossil light; F1:anisotropy; F2:polarization; F3:last scattering; F4:starting point
     * Z01 (vid=2001, recombination): F0:neutral atom; F1:hydrogen; F2:decoupling; F3:transparent universe; F4:apparent surface
     * Z02 (vid=2002, primordial-plasma): F0:free electron; F1:Hot Big Bang; F2:scattered photon; F3:acoustic oscillation; F4:no direct image
     * Z03 (vid=2003, nucleosynthesis): F0:proton; F1:neutron; F2:deuterium; F3:primordial helium; F4:primordial lithium
     * Z04 (vid=2004, lepton-era): F0:electron; F1:positron; F2:neutrino; F3:antimatter; F4:neutrino decoupling
4. shaders/compute_universe.wgsl (+ `crates/cosmos_render/src/universe_scene.rs`, `shaders/far_shell.wgsl`)
   - Wire existing compute Mode 3 (QCD Quark-Gluon, L162-187) to Z05 and Mode 5 (Planck Spin-Foam, L209-222) to Z06..Z08; set star_density=0.0 across all Z00..Z08; gate inflation_alpha exclusively to Z07..Z08.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

<a id="pkg-act2-exp10-z05-08"></a>
### PKG-ACT2-EXP10-Z05-08: Quark-Gluon Plasma (Mode 3), Electroweak Higgs (Mode 5), Inflation & Known-Physics Limit (`z=05..08`) — `Phase 3 + Phase 4` · Priority: **Critical**

**Current State:** Zones `Z05..Z08` (`4` zones, `21` features) | Zone Tiers `0A / 1B / 3C / 0D` | Feature Tiers `0A / 3B / 5C / 13D` | Render path: [`doc_stages.rs#L696-L708`](../../crates/cosmos_render/src/doc_stages.rs#L696-L708) -> [`shaders/hero_doc_cosmos.wgsl#L573`](../../shaders/hero_doc_cosmos.wgsl#L573) — Single static `kind=3, vid=10` expansion ring (Mode 2 lock; Modes 3/5 unused). `doc_feat(inst)` is unread or collapses to a single generic pulse/orbital offset across all `21` features; `cue_tags` dropped at [`engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699).

**Reserved Target IDs:**

| Zone | Slug | ES Title / EN Title | Reserved `vid` | Companion IDs & `submode` | Compute / Far-Shell Target | `doc_feat` (`0..7`) Choreography Slot Scope |
|---:|---|---|---:|---|---|---|
| `Z05` | `quark-era` | Era de los quarks / *Quark Epoch* | **`2005`** | `submode=0..4` (`base_flags` cue mask) | `mode=3` (QCD Quark-Gluon, existing), `star_density=0.0`, `cmb_alpha=0.0` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z06` | `electroweak` | Transición electrodébil / *Electroweak Transition* | **`2006`** | `submode=0..4` (`base_flags` cue mask) | `mode=5` (Planck Spin-Foam, existing), `star_density=0.0`, `inf_alpha` gated `Z>=7` | `doc_feat 0..=4` (5 features, `255` = macro) |
| `Z07` | `inflation` | Universo inflacionario / *Inflationary universe* | **`2007`** | `submode=0..5` (`base_flags` cue mask) | `mode=5` (Planck Spin-Foam, existing), `star_density=0.0`, `inf_alpha` gated `Z>=7` | `doc_feat 0..=5` (6 features, `255` = macro) |
| `Z08` | `known-physics-limit` | Límite de la física conocida / *Boundary of known physics* | **`2008`** | `submode=0..4` (`base_flags` cue mask) | `mode=5` (Planck Spin-Foam, existing), `star_density=0.0`, `inf_alpha` gated `Z>=7` | `doc_feat 0..=4` (5 features, `255` = macro) |

**Spoken Transcript Summary & 3D Visual Blueprint:**

#### `Exp 10 Z05` `quark-era` — Era de los quarks / *Quark Epoch* (`[TIER-B]` · Target `vid=2005`)
- **Metric (`raw_metric`):** "Antes de ≈ 10⁻⁶ s · plasma de quarks y gluones" (*Before ≈ 10⁻⁶ s · quark–gluon plasma*) · `range=[0.590, 0.720]`
- **Spoken Macro (ES // EN):** A energías extremas, quarks y gluones no estaban confinados dentro de protones y neutrones. Formaban un fluido de partículas elementales fuertemente interactuante. // *At extreme energies, quarks and gluons were not confined within protons and neutrons. They formed a fluid of strongly interacting elementary particles.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=10` activates inner ring `sym = select(0, ..., z_idx >= 5u)` (`L602`) around swirling `qgp_rot` strongly-interacting fluid (`F2 plasma de quarks y gluones` [B]). **Missed reuse**: `compute_universe.wgsl:163` already implements **Mode 3 QCD Color Flux Tubes** (R/G/B confinement!), yet `universe_scene.rs:309` locks Exp 10 to Cosmic Web Mode 2!
- **Audit Gap:** Swirling fluid evokes QGP at zone level, but leaves Mode 3 QCD color-charge particles completely unwired and lacks individual RGB quarks, gluon flux springs, and hadronization confinement transition.
- **Zone 3D Procedural Blueprint:** Switch `select_compute_mode` for `exp==10` QGP zone (`doc_zone==5`) to **Mode 3 QCD**, and add `doc_feat` states to `vid=10`: `F0` highlights unconfined R/G/B quark beads; `F1` renders shimmering multi-color gluon flux-tube springs; `F2` swirls collective relativistic QGP fluid; `F3` snaps triplets into confined proton/neutron hadrons (`doc_cue`); `F4` frames tiny collision fireball inset.

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **quark** / *quark* | `[TIER-D]` | Los quarks son componentes elementales de protones, neutrones y muchas otras partículas compuestas. // *Quarks are elementary components of protons, neutrons and many other composite particles.* | Compute Mode 3 QCD (`compute_universe.wgsl:163-187`) exists unreferenced; zero R/G/B quark color charges rendered in Exp 10. | Wire Compute Mode 3 + render distinct Red/Green/Blue quark point emitters (`doc_feat==0`). |
| `F1` | **gluón** / *gluon* | `[TIER-D]` | El gluón transmite la interacción fuerte que une a los quarks y domina el comportamiento de este plasma. // *The gluon transmits the strong interaction that binds the quarks and dominates the behavior of this plasma.* | Zero gluon color-flux tube springs between quarks. | Draw curved multi-hue QCD string tubes stretching/snapping between quark charges (`doc_feat==1`). |
| `F2` | **plasma de quarks y gluones** / *quark-gluon plasma* | `[TIER-B]` | Experimentos de colisiones recrean por instantes estados de materia comparables, no el universo completo. // *Collision experiments briefly recreate comparable states of matter, not the entire universe.* | Swirling rotating `qgp_rot` fluid filaments (`L599-603`) visibly represent dense strongly-interacting QGP. | Increase fluid curl vorticity & warm-amber viscosity sheen (`doc_feat==2`). |
| `F3` | **confinamiento** / *confinement* | `[TIER-D]` | Al enfriarse el cosmos, los quarks quedaron ligados en hadrones como protones y neutrones. // *As the cosmos cooled, quarks were bound into hadrons like protons and neutrons.* | No hadronization phase transition confining free quarks into `p`/`n` triplets (`z_idx >= 5` `sym` is a static `r=0.36` ring). | Condense free R/G/B particles into bound colorless white RGB triangles as `doc_cue` rises (`doc_feat==3`). |
| `F4` | **física de partículas** / *particle physics* | `[TIER-C]` | Aceleradores, teoría cuántica de campos y cálculos numéricos ayudan a probar esta reconstrucción. // *Accelerators, quantum field theory and numerical calculations help test this reconstruction.* | Static `sym` (`r=0.36`) ring vaguely hints symmetry without collider/lattice particle-physics context. | Inset miniature LHC heavy-ion collision jet cone or lattice QCD grid slice (`doc_feat==4`). |

#### `Exp 10 Z06` `electroweak` — Transición electrodébil / *Electroweak Transition* (`[TIER-C]` · Target `vid=2006`)
- **Metric (`raw_metric`):** "Escala aproximada ≈ 10⁻¹² s · simetrías de altas energías" (*Approximate scale ≈ 10⁻¹² s · high energy symmetries*) · `range=[0.720, 0.830]`
- **Spoken Macro (ES // EN):** En el modelo de partículas, el electromagnetismo y la interacción débil se describen como aspectos de una teoría electrodébil común a energías suficientemente altas. // *In the Standard Model of particle physics, electromagnetism and the weak interaction are described as aspects of one electroweak theory at sufficiently high energies.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `vid=10` shows `qgp_rot` + static `core` + single static ring `sym (r=0.36)` (`z_idx >= 5u`, `L602`) + Cosmic Web Mode 2 particles. Zero Higgs field, Mexican-hat potential, W/Z mass acquisition, or massless photon split.
- **Audit Gap:** Static ring `r=0.36` is only a vague geometric halo — zero depiction of electroweak symmetry breaking (~10^-12 s), Higgs VEV roll, heavy W/Z bosons, or remaining massless photon.
- **Zone 3D Procedural Blueprint:** For `doc_zone==6`, replace `core` fireball with a translucent rotating 3D **Higgs Mexican-Hat potential (`V(phi)`)** well (`F0, F1`) rolling from symmetric apex to vacuum trough (`doc_cue`), spawning slow massive glowing spheres for `W±/Z⁰` (`F2`) while massless `gamma` laser rays shoot unimpeded (`F3`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **campo de Higgs** / *Higgs field* | `[TIER-D]` | La transición asociada al campo de Higgs modificó cómo ciertas partículas adquieren masa efectiva. // *The transition associated with the Higgs field modified how certain particles acquire effective mass.* | Zero Higgs field potential surface or non-zero vacuum expectation condensation. | Sculpt iridescent 3D Mexican-hat `V(phi)` funnel with condensate marble settling into trough (`doc_feat==0`). |
| `F1` | **simetría electrodébil** / *electroweak symmetry* | `[TIER-C]` | A energías altas, fenómenos hoy distintos encajan en una descripción unificada comprobada por la física de partículas. // *At high energies, phenomena that appear distinct today fit a unified description tested by particle physics.* | Generic concentric `r=0.36` & `0.56` rings (`L598, L602`) weakly imply abstract symmetry with no physics unification. | Morph unified electroweak ring into split glowing `SU(2) x U(1) -> U(1)_EM` branches (`doc_feat==1`). |
| `F2` | **bosones W y Z** / *W and Z bosons* | `[TIER-D]` | Estas partículas median la interacción débil y obtienen masa mediante el mecanismo de Higgs. // *These particles mediate the weak interaction and gain mass through the Higgs mechanism.* | Zero massive short-range W and Z weak bosons. | Render slow heavy glowing `W+/W-/Z0` orbs experiencing viscous Higgs drag (`doc_feat==2`). |
| `F3` | **electromagnetismo** / *electromagnetism* | `[TIER-D]` | Tras la transición, el fotón permanece sin masa y transmite la interacción electromagnética. // *After the transition, the photon remains massless and transmits the electromagnetic interaction.* | Zero massless photon (`gamma`) trajectory unaffected by Higgs condensate. | Shoot crisp light-speed straight photon wave rays gliding friction-free across Higgs mist (`doc_feat==3`). |
| `F4` | **frontera experimental** / *experimental frontier* | `[TIER-D]` | Cuanto más retrocedemos, más depende la cosmología de teorías probadas en laboratorios pero extrapoladas a energías mayores. // *The further back we go, the more cosmology depends on theories tested in laboratories but extrapolated to higher energies.* | Zero energy-scale boundary / collider extrapolation meter (`100 GeV -> >TeV`). | Display glowing TeV collider frontier ruler fading upward into theoretical extrapolation haze (`doc_feat==4`). |

#### `Exp 10 Z07` `inflation` — Universo inflacionario / *Inflationary universe* (`[TIER-C]` · Target `vid=2007`)
- **Metric (`raw_metric`):** "Época propuesta · expansión acelerada extremadamente temprana" (*Proposed epoch · extremely early accelerated expansion*) · `range=[0.830, 0.940]`
- **Spoken Macro (ES // EN):** La inflación es un escenario en el que el propio espacio se expandió de forma extraordinariamente rápida. No fue una explosión dentro de un vacío ni tuvo un centro espacial. // *Inflation is a scenario in which space itself expanded extraordinarily quickly. It was not an explosion within a vacuum nor did it have a spatial center.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** `far_shell.wgsl:141-148` (`inflation_alpha ~ 0.58` at `u=26.9`) draws undulating spherical quantum bubble caustics (`F0, F1`), YET `vid=10` (`L596-605`) simultaneously renders a glowing central `core = exp(-r*15)` fireball + `r=0.36, 0.56` rings + Cosmic Web Mode 2 + CMB `far_shell`!
- **Audit Gap:** `core` central explosion point **directly contradicts narration** ('not an explosion within a vacuum nor did it have a spatial center'), while `F3` flatness, `F4` indirect evidence, and `F5` relic gravitational waves have zero visual representation (6 features total!).
- **Zone 3D Procedural Blueprint:** Delete `core` & rings on `doc_zone==7`; render an expanding exponential geodesic grid inflating uniformly everywhere (`F0, F3`), stretching tiny violet quantum zero-point ripples to super-horizon scale (`F1, F2`), linked to CMB imprint (`F4`) and quadrupole tensor B-mode ripples (`F5`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **expansión del espacio** / *expansion of space* | `[TIER-B]` | Las distancias entre regiones aumentan porque cambia la geometría del espacio en todas partes simultáneamente. // *Distances between regions increase because the geometry of space changes everywhere simultaneously.* | `far_shell` `inflation_alpha` (`L143-147`) shows stretching bubble caustic walls around sky. | Replace central fireball with uniformly exponentially expanding 3D coordinate lattice (`a(t) ~ e^Ht`, `doc_feat==0`). |
| `F1` | **fluctuación cuántica** / *quantum fluctuation* | `[TIER-B]` | Pequeñas variaciones microscópicas pudieron estirarse y convertirse en semillas de la estructura cósmica. // *Small microscopic variations could stretch and become seeds of cosmic structure.* | `far_shell` iridescent quantum wall ripples (`inf_n`, `L144-146`) evoke microscopic vacuum fluctuations. | Zoom/stretch tiny fluctuating zero-point quantum ripples outward to macro seed size (`doc_feat==1`). |
| `F2` | **horizonte** / *horizon* | `[TIER-C]` | La inflación ayuda a explicar por qué regiones muy separadas del fondo cósmico presentan propiedades tan parecidas. // *Inflation helps explain why widely separated regions of the cosmic background have such similar properties.* | **Active contradiction**: `vid=10` `core` (`r=0`) + crisp `r=0.36, 0.56` concentric rings contradict centerless horizon explanation (`macro_fact`/`F2`). | Remove central `core`; depict formerly adjacent tiny causal patches blown far outside each other's horizon circles (`doc_feat==2`). |
| `F3` | **planitud** / *flatness* | `[TIER-D]` | Una expansión enorme puede hacer que la geometría observable parezca muy cercana a plana, como una pequeña zona de una superficie inmensa. // *Enormous expansion can make observable geometry appear nearly flat, like a small patch of an immense surface.* | Zero curved-surface-flattening metric demonstration (`k -> 0` flatness problem). | Expand a small glowing curved balloon patch (`r: 0.2 -> 4.0`) until its local grid looks dead flat (`doc_feat==3`). |
| `F4` | **evidencia indirecta** / *indirect evidence* | `[TIER-D]` | El fondo cósmico y la estructura apoyan rasgos generales del escenario, pero el mecanismo exacto y su energía no están confirmados. // *The cosmic background and structure support general features of the scenario, but the exact mechanism and its energy are not confirmed.* | Zero indirect evidence / model-vs-confirmation uncertainty overlay. | Fade solid geometry to dashed translucent model lines over CMB template (`doc_feat==4`). |
| `F5` | **ondas gravitacionales relictas** / *relic gravitational waves* | `[TIER-D]` | Una señal primordial sería una prueba extraordinaria, pero todavía no existe una detección confirmada de esas ondas procedentes de la inflación. // *A primordial signal would be extraordinary evidence, but there is still no confirmed detection of gravitational waves from inflation.* | Zero primordial tensor gravitational wave curl ripples (B-mode). | Send faint quadrupolar `+ / x` spacetime shear waves rippling across the expanding lattice without solid emission (`doc_feat==5`). |

#### `Exp 10 Z08` `known-physics-limit` — Límite de la física conocida / *Boundary of known physics* (`[TIER-C]` · Target `vid=2008`)
- **Metric (`raw_metric`):** "Frontera teórica · gravedad cuántica y condiciones iniciales" (*Theoretical frontier · quantum gravity and initial conditions*) · `range=[0.940, 1.000]`
- **Spoken Macro (ES // EN):** La imagen se disuelve porque nuestras teorías actuales dejan de ser suficientes. Esta región no muestra una singularidad observada, un primer punto confirmado ni algo conocido antes de la inflación. // *The image dissolves because our current theories are no longer sufficient. This region does not show an observed singularity, a confirmed first point, or something known before inflation.*
- **Active `DOMAIN_CUE_KEYWORDS` (`resolver.rs#L952-L961`):** *none yet (add bilingual cues)*
- **Current Visual Reality:** **Severe contradiction**: Narration states *'does not show an observed singularity, a confirmed first point... the upper field represents that uncertainty without inventing an endpoint'*, YET `vid=10` renders a bright solid central **`core = exp(-r*15)` singularity dot** + sharp crisp rings (`r=0.36, 0.56`) + evolved Cosmic Web particles (`Mode 2`) + `25%` background stars (`universe_scene.rs:370`)! Meanwhile **Mode 5 Planck Spin-Foam** (`compute_universe.wgsl:209-222`) & `far_shell` `q_alpha` (`L163-171`) exist elsewhere unused!
- **Audit Gap:** Shows a glowing singularity point, evolved cosmic web galaxies, and background stars precisely where narration stresses zero confirmed singularity, breakdown of classical spacetime, and pure scientific humility.
- **Zone 3D Procedural Blueprint:** For `doc_zone==8`, eliminate `core`, crisp circles, Mode 2 galaxies, and stars (`star_density -> 0`): dissolve smooth spacetime grid lines into probabilistic **Mode 5** Planck foam flickers (`F0, F1`) with no central apex (`F3`), open translucent dotted `?` boundary curves representing unknown initial conditions (`F2, F4`).

| `doc_feat` | Feature (ES / *EN*) | Tier | Spoken Content (ES // *EN*) | Code Reality / Gap | Target `doc_feat` 3D Visual Choreography |
|---:|---|:---:|---|---|---|
| `F0` | **gravedad cuántica** / *quantum gravity* | `[TIER-D]` | Se necesita una teoría que describa conjuntamente la gravedad y los fenómenos cuánticos en condiciones extremas. // *A theory is needed that jointly describes gravity and quantum phenomena under extreme conditions.* | No quantum-gravity spacetime breakdown or discrete Planck spin-foam (Mode 5 unused). | Wire Compute Mode 5 + dissolve smooth classical geodesic lines into jittering Planck spin-foam vertices (`doc_feat==0`). |
| `F1` | **escala de Planck** / *Planck scale* | `[TIER-D]` | Cerca de esta escala convencional, las extrapolaciones clásicas del espacio y el tiempo pierden fiabilidad. // *Near this conventional scale, classical extrapolations of space and time lose reliability.* | No Planck-scale loss-of-resolution / spacetime blur visualization. | Blur crisp geometry below Planck length scale cutoff into iridescent shimmering fog (`doc_feat==1`). |
| `F2` | **condiciones iniciales** / *initial conditions* | `[TIER-D]` | No sabemos qué selección física produjo el estado temprano del que surgió la evolución observable. // *We do not know what physical process produced the early state from which observable evolution emerged.* | No multiple branching open-ended initial-condition possibilities. | Float 3 dim translucent hypothetical initial-state probability clouds fading at horizon (`doc_feat==2`). |
| `F3` | **sin singularidad observada** / *no observed singularity* | `[TIER-C]` | Una singularidad matemática señala el fallo de una descripción, no una fotografía confirmada de un objeto real. // *A mathematical singularity signals the failure of a description, not a confirmed photograph of a real object.* | **Direct contradiction**: `vid=10` `core = exp(-r*15.0)` (`L601`) draws an intense bright point singularity right at `(0,0,0)`! | Delete `core` & central apex completely; replace center with open void/dissolving horizon (`doc_feat==3`). |
| `F4` | **pregunta abierta** / *open question* | `[TIER-C]` | La ciencia distingue entre evidencia, modelo y especulación; el campo superior representa esa incertidumbre sin inventar un destino. // *Science distinguishes among evidence, models, and speculation; the upper field represents that uncertainty without inventing an endpoint.* | Crisp rings, Mode 2 cosmic web filaments, and `25%` background stars create a definite structure contradicting 'no invented endpoint'. | Strip rings, galaxies, and stars; keep strictly faint fading open horizon wisps (`>=80% #000000` void, `doc_feat==4`). |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT2-EXP10-Z05-08)</summary>

```text
TASK: Upgrade HyperCosmosVR Experience 10 (10_early_universe) Zones Z05..Z08 (`quark-era`, `electroweak`, `inflation`, `known-physics-limit`) from Tier B/C/D to [TIER-A] bespoke 1:1 procedural 3D documentary visuals.

PREREQUISITES: Assumes PKG-P0-DISPATCH-AND-IDS and PKG-P0-FEATURE-AND-CUE-PLUMBING are landed (or include minimal local decoding for vid >= 1000 and doc_feat/doc_cue forwarding without breaking legacy vid < 1000).

EXACT FILES TO EDIT (4 Core Files + Scope Notes):
1. crates/cosmos_render/src/doc_stages.rs
   - Map `Z05->2005`, `Z06->2006`, `Z07->2007`, `Z08->2008` explicitly in doc_stage_variant_id() and emit_doc_stage_hero() / emit_celestial_or_landmark().
2. crates/cosmos_narration/src/resolver.rs
   - Add domain bilingual keywords to DOMAIN_CUE_KEYWORDS (lines 952-961): `cmb`, `inflacion`, `quark`, `higgs`, `planck`, `horizonte` (exist!), plus `recombinacion`, `plasma`, `nucleosintesis`, `lepton`, `electrodebil`, `cuantica`, `singularidad`; verify kw:* cue_tags emit on both ES & EN narration spans.
3. shaders/hero_doc_cosmos.wgsl
   - Implement/route cos_exp_and_sub(vid) / ch_exp_and_sub(vid) for `Z05->2005`, `Z06->2006`, `Z07->2007`, `Z08->2008`.
   - Inside each zone branch, read doc_feat(inst) (0..=7, 255=macro), doc_cue(inst), and doc_weight(inst) to animate every feature F0..Fk distinctly as spoken:
     * Z05 (vid=2005, quark-era): F0:quark; F1:gluon; F2:quark-gluon plasma; F3:confinement; F4:particle physics
     * Z06 (vid=2006, electroweak): F0:Higgs field; F1:electroweak symmetry; F2:W and Z bosons; F3:electromagnetism; F4:experimental frontier
     * Z07 (vid=2007, inflation): F0:expansion of space; F1:quantum fluctuation; F2:horizon; F3:flatness; F4:indirect evidence; F5:relic gravitational waves
     * Z08 (vid=2008, known-physics-limit): F0:quantum gravity; F1:Planck scale; F2:initial conditions; F3:no observed singularity; F4:open question
4. shaders/compute_universe.wgsl (+ `crates/cosmos_render/src/universe_scene.rs`, `shaders/far_shell.wgsl`)
   - Wire existing compute Mode 3 (QCD Quark-Gluon, L162-187) to Z05 and Mode 5 (Planck Spin-Foam, L209-222) to Z06..Z08; set star_density=0.0 across all Z00..Z08; gate inflation_alpha exclusively to Z07..Z08.

HARD OLED & VR PERFORMANCE GUARDRAILS:
- Pure-Black OLED Void Budget: Maintain >= 50.91% exact #000000 background pixels and 0.0000% highlight blowout (strictly local bounded SDF/volumetric integration inside hero_volumetric.wgsl; NO full-screen haze/wash).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps per hero volume with sphere bound clipping (r <= 0.96) and early exit (trans < 0.03), preserving RTX 5070 DX12 baseline (mean = 1.927 ms, p99 = 4.873 ms in artifacts/verification_renders/benchmark_sweep.txt).

REQUIRED VERIFICATION COMMANDS & HYGIENE:
- cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
- cargo clippy --workspace --all-targets -- -D warnings
- cargo test --workspace -- --test-threads=1
- CONTRIBUTING.md hygiene: zero binaries > 50 MiB, zero .ttf/.otf font files, zero machine-specific .cargo/config.toml or local filesystem paths.
```

</details>

---
