# Act IV Gap Map — Life, Mind, Colors & Music (`15, 16, 17, 01, 02`)

**Scope:** Complete contributor visual implementation & 1:1 audio-to-shader parity map for **Odyssey Act IV** (`ODYSSEY_ACTS[3] = &[15, 16, 17, 1, 2]` in [`crates/cosmos_journey/src/odyssey.rs`](../../crates/cosmos_journey/src/odyssey.rs#L24-L45)): **`53` Zones (`417` Features)** across Experiences `15_complexity`, `16_life`, and `17_mind`, plus **`2` Spatial Experiences** (`01_colors`: `130` keys / `72` lattice nodes; `02_music`: `48x4` = `192` filaments / `20` keys), packaged into **`14` claimable 1-PR Pick-Up Cards** out of the canonical **`88`-Package** HyperCosmosVR Visual Roadmap (`3` P0 + `85` content packages).

## Quick Navigation

| Document | Scope & Purpose |
|---|---|
| [`../CONTRIBUTOR_VISUAL_ROADMAP.md`](../CONTRIBUTOR_VISUAL_ROADMAP.md) | Master Index (`88` packages), P0 Core Pipeline (`#pkg-p0-*`), 4-Phase Rollout, Shader ID Registry |
| [`./ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md`](./ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md) | Act I (`03, 04` — `26` zones, `130` features, `6` packages) |
| [`./ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md`](./ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md) | Act II (`05, 21, 22, 06..10` — `116` zones, `684` features, `26` packages) |
| [`./ACT_3_MATTER_AND_SPACETIME_MAP.md`](./ACT_3_MATTER_AND_SPACETIME_MAP.md) | Act III (`13, 14` — `27` zones, `187` features, `7` packages) |
| **Act IV Map (`ACT_4_LIFE_MIND_AND_SENSES_MAP.md`)** | **This Document — Act IV (`15, 16, 17, 01, 02` — `53` zones, `417` features + `01, 02`, `14` packages)** |
| [`./ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md`](./ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md) | Act V (`18..20, 11, 12, 23..25` — `154` zones, `1,038` features, `32` packages) |

> [!NOTE]
> **Why `01_colors` & `02_music` Sit Inside Act IV:** In [`ODYSSEY_ACTS`](../../crates/cosmos_journey/src/odyssey.rs#L37-L40) and [`crates/cosmos_spatial/src/topology.rs`](../../crates/cosmos_spatial/src/topology.rs#L643-L647), the journey ascends inward-to-outward from microscopic emergence (`COMPLEXITY_HELIX`, `u=-7.0`) -> biosphere (`LIFE_BIOSPHERE`, `u=-1.0`) -> conscious mind (`MIND_CATHEDRAL`, `u=2.8`), then dives directly into Mind's two nested sensory chambers (`COLORS_PRISM` at `(-0.22, 0.02, -0.04)`, `MUSIC_HARP` at `(+0.22, 0.02, -0.04)`, `u=2.0`) before continuing outward to Culture (`18`, Act V).

---

## 1. Act IV Verified Ground-Truth Scorecard

Every zone (`53`) and spoken feature (`417`) below was audited 1:1 against [`assets/baked/universe_catalog.v1.json`](../../assets/baked/universe_catalog.v1.json), [`crates/cosmos_render/src/doc_stages.rs`](../../crates/cosmos_render/src/doc_stages.rs), [`crates/cosmos_render/src/universe_scene.rs`](../../crates/cosmos_render/src/universe_scene.rs), [`crates/cosmos_app/src/engine.rs`](../../crates/cosmos_app/src/engine.rs), [`crates/cosmos_narration/src/resolver.rs`](../../crates/cosmos_narration/src/resolver.rs), and [`shaders/hero_doc_matter_mind.wgsl`](../../shaders/hero_doc_matter_mind.wgsl):

| Exp | Slug | Catalog Title (ES) | Act IV Stop | Sector & `u` Range (`topology.rs`) | Zones | Zone `[A / B / C / D]` | Feats | Feat `[A / B / C / D]` | Current `doc_stage_variant_id` / Dispatch | Reserved `vid` Block |
|---:|---|---|---:|---|---:|:---:|---:|:---:|---|---|
| `15` | `15_complexity` | Complejidad | `1/5` | `COMPLEXITY_HELIX` (`[-9.0, -5.0]`, `1e-4 m`) | `16` | `0 / 3 / 8 / 5` | `119` | `0 / 19 / 55 / 45` | `kind=8, vid=10 + (sc % 4)` (`field_grid_tex` unread; `mode=12`) | `2500..=2515` (comp `3760..=3769`) |
| `16` | `16_life` | Vida | `2/5` | `LIFE_BIOSPHERE` (`[-6.0, 2.0]`, `200 m`) | `19` | `0 / 3 / 11 / 5` | `153` | `0 / 15 / 69 / 69` | `kind=8`: `{0->23, 4->20, _->20+(sc%5)}` (`mode=6` BZ wave) | `2600..=2618` (comp `3770..=3779`) |
| `17` | `17_mind` | Mente | `3/5` | `MIND_CATHEDRAL` (`[1.5, 4.5]`, `25 km`) | `18` | `0 / 14 / 0 / 4` | `145` | `0 / 104 / 1 / 40` | `kind=8, vid=30 + (sc % 5)` (`mm_brain_sdf`; `mode=7` connectome) | `2700..=2717` (comp `3780..=3789`) |
| `01` | `01_colors` | Espectro de colores | `4/5` | `COLORS_PRISM` (`[0.5, 3.6]`, `120 m`) | `0` | `[TIER-B]` spatial (`130` keys / `72` nodes) | `0` | — | `kind=5, vid=1/10`; `kind=1, vid=12` (`mode=9` dust) | `1100..=1199` (comp `3600..=3609`) |
| `02` | `02_music` | Espectro musical | `5/5` | `MUSIC_HARP` (`[0.5, 3.6]`, `120 m`) | `0` | `[TIER-B]` spatial (`48x4` = `192` filaments / `20` keys) | `0` | — | `kind=6, vid=1, 11..14` (`field_grid=1` unread; `mode=8` cymatics) | `1200..=1299` (comp `3610..=3619`) |
| **TOTAL** | **Act IV** | **`15, 16, 17, 01, 02`** | **5 Stops** | **Complexity -> Sensory** | **`53`** | **`0 / 20 / 19 / 14` (+2 `[TIER-B]` spatial)** | **`417`** | **`0 / 138 / 125 / 154`** | **`0%` Tier-A across all `417` spoken features** | **`14` Pick-Up Cards** |

**Rubric Key:** **`[TIER-A]`** Dedicated bespoke hero `vid` + working per-feature `doc_feat(inst)` response · **`[TIER-B]`** Correct domain stage on screen, lacks `doc_feat` sub-staging or secondary organs · **`[TIER-C]`** Mis-routed `% N` modulo / override collision (`vid` depicts a different zone's biology/physics) · **`[TIER-D]`** Narrated mechanism, apparatus, or biological structure has zero visual counterpart on screen.

---

## 2. Core Pipeline Prerequisites (`P0`) & Act IV Systemic Findings

Before or alongside claiming Act IV packages, ensure the three cross-cutting Phase 1 P0 packages in [`../CONTRIBUTOR_VISUAL_ROADMAP.md`](../CONTRIBUTOR_VISUAL_ROADMAP.md) are understood or landed:
- **[`PKG-P0-DISPATCH-AND-IDS`](../CONTRIBUTOR_VISUAL_ROADMAP.md#pkg-p0-dispatch-and-ids):** Routes `vid = 1000 + exp*100 + z` cleanly inside [`doc_stages.rs`](../../crates/cosmos_render/src/doc_stages.rs#L36-L65) and adds `>= 1000` decoding to [`shaders/hero_doc_matter_mind.wgsl`](../../shaders/hero_doc_matter_mind.wgsl) (`0..=299` fallback preserved).
- **[`PKG-P0-FEATURE-AND-CUE-PLUMBING`](../CONTRIBUTOR_VISUAL_ROADMAP.md#pkg-p0-feature-and-cue-plumbing):** Forwards `DOMAIN_CUE_KEYWORDS` / `VisualCueTag` from [`resolver.rs`](../../crates/cosmos_narration/src/resolver.rs#L952-L1093) through [`engine.rs:1691-1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699) into `base_flags` (`pack_doc_tags`), forwards `zone_c` into `color_b` at [`doc_stages.rs:739`](../../crates/cosmos_render/src/doc_stages.rs#L739) for Exp `15..=17`, and wires `doc_feat(inst)` + `field_grid_tex` inside `hero_doc_matter_mind.wgsl`.
- **[`PKG-P0-CATALOG-SCENE-AND-CI`](../CONTRIBUTOR_VISUAL_ROADMAP.md#pkg-p0-catalog-scene-and-ci):** Backfills missing `"scene": null` metadata (present on all `53` zones of Exp `15–17`) and enforces `--headless-render` CI gates.

### Act IV Systemic Code Findings (Verified Line Citations)
1. **Zero `doc_feat(inst)` Branching (`0 / 417` Features):** [`pack_doc_tags`](../../crates/cosmos_render/src/doc_stages.rs#L13-L31) packs `active_feature_index` cleanly into `packed_z` (`bits 16..23`), and [`shaders/doc_common.wgsl:82`](../../shaders/doc_common.wgsl#L82) exposes `doc_feat(inst)`, yet `shaders/hero_doc_matter_mind.wgsl` never calls `doc_feat(inst)` anywhere across lines `1..=845`.
2. **Dead GPU Field Grids (`Exp 15` Gray-Scott & `Exp 02` Chladni):** [`universe_scene.rs:217-227`](../../crates/cosmos_render/src/universe_scene.rs#L217-L227) dispatches `cs_field_grid` mode `0` (`15`, Gray-Scott PDE) and mode `1` (`02`, Chladni PDE) into `field_grid_tex` (`doc_common.wgsl:24`), yet `field_grid_tex` is ONLY sampled in `shaders/hero_doc_cosmos.wgsl:282` — `march_doc_bio_mind` (`Exp 15`) and `march_doc_music` (`Exp 02`) both draw static analytical `sin/cos` formulas instead.
3. **Systematic `% 4` / `% 5` Collisions in `Exp 15` & `Exp 16`:** `15 => 10 + (z % 4)` sends condensed-matter physics (`Z03 matter`) and feedback (`Z07 feedback`) into a budding protocell (`vid=13`) while sending `Z13 protocells` into Turing coral (`vid=11`). `16 => 0|u16::MAX => 23, 4 => 20, _ => 20 + (z % 5)` renders the 3-domain Tree of Life at pre-LUCA `Z00 threshold`, a Chloroplast at `Z16 brains`, a Mitochondrion at `Z17 perception`, and B-DNA at `Z15 nervous`.
4. **Occipital Bias in `Exp 17` (`mm_brain_sdf`) & Unused `zone_c`:** All 18 zones of Exp 17 share one `mm_brain_sdf` tinted globally with `z % 5` cortical masks (so `Z00 consciousness`, `Z05 memory`, `Z10 decisions`, and `Z15 connected` all light only posterior occipital cortex `p.z < -0.10`), while `color_b` in `doc_stages.rs:731-744` ignores `experience_accent_linear` / `zone_accent_rgb`.
5. **Compute Mode Lock (`Exp 16`) & OLED Far-Shell Void:** `universe_scene.rs:320` keeps `mode=6` (`compute_universe.wgsl:223-237`, commented `(15)` Belousov-Zhabotinsky protocell shell!) running even over multicellular brains (`Z15..Z18`), while `mw_alpha = 0.0` (`far_shell.wgsl:82-103`) leaves pure `#000000` behind every zone of `15, 16, 17, 01, 02` (ideal for OLED black floors, provided heroes supply self-contained micro-context).

---

## 3. Reserved Shader ID Registry & Performance Contract

- **Zonal ID Formula:** `vid = 1000 + (exp_id * 100) + zone_index` (`Exp 15 -> 2500..=2515`, `Exp 16 -> 2600..=2618`, `Exp 17 -> 2700..=2717`), plus Spatial IDs (`Exp 01 -> 1100..=1104`, `Exp 02 -> 1200..=1203`) and Companion Surface IDs (`01 -> 3600..=3609`, `02 -> 3610..=3619`, `15 -> 3760..=3769`, `16 -> 3770..=3779`, `17 -> 3780..=3789`).
- **Pure-Black OLED Void Budget:** Maintain `>= 50.91%` exact `#000000` background pixels and `0.0000%` highlight blowout; keep volumetric SDF integration strictly bounded (`r <= 0.96`, early-exit `trans < 0.03`) in [`shaders/hero_volumetric.wgsl`](../../shaders/hero_volumetric.wgsl) with zero full-screen ambient fog wash.
- **VR 90/120 Hz Frame-Time Budget:** Cap new documentary hero raymarches at `<= 28` steps per hero instance, preserving the verified RTX 5070 DX12 baseline (`min = 0.852 ms`, `mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in [`artifacts/verification_renders/benchmark_sweep.txt`](../../artifacts/verification_renders/benchmark_sweep.txt)).

---

## 4. Canonical Act IV Pick-Up Cards (`14` Claimable 1-PR Work Packages)

| Package ID | Exp / Scope | Zones | Zone `[A/B/C/D]` | Feats | Feat `[A/B/C/D]` | Reserved Target IDs | Phase |
|---|---|---:|:---:|---:|:---:|---|---|
| [`PKG-ACT4-EXP15-Z00-03`](#pkg-act4-exp15-z00-03) | Exp `15` (`Z00..Z03`) | `4` | `0/0/2/2` | `29` | `0/0/14/15` | `2500..=2503` | `Phase 1/2` |
| [`PKG-ACT4-EXP15-Z04-07`](#pkg-act4-exp15-z04-07) | Exp `15` (`Z04..Z07`) | `4` | `0/1/2/1` | `29` | `0/7/10/12` | `2504..=2507` (comp `3760`) | `Phase 1/2` |
| [`PKG-ACT4-EXP15-Z08-11`](#pkg-act4-exp15-z08-11) | Exp `15` (`Z08..Z11`) | `4` | `0/1/2/1` | `30` | `0/7/13/10` | `2508..=2511` (comp `3761`) | `Phase 1/2` |
| [`PKG-ACT4-EXP15-Z12-15`](#pkg-act4-exp15-z12-15) | Exp `15` (`Z12..Z15`) | `4` | `0/1/2/1` | `31` | `0/5/18/8` | `2512..=2515` (comp `3762`) | `Phase 1/2` |
| [`PKG-ACT4-EXP16-Z00-04`](#pkg-act4-exp16-z00-04) | Exp `16` (`Z00..Z04`) | `5` | `0/1/4/0` | `40` | `0/5/28/7` | `2600..=2604` (comp `3770`) | `Phase 1/2` |
| [`PKG-ACT4-EXP16-Z05-09`](#pkg-act4-exp16-z05-09) | Exp `16` (`Z05..Z09`) | `5` | `0/2/1/2` | `40` | `0/10/8/22` | `2605..=2609` (comp `3771`) | `Phase 1/2` |
| [`PKG-ACT4-EXP16-Z10-14`](#pkg-act4-exp16-z10-14) | Exp `16` (`Z10..Z14`) | `5` | `0/0/2/3` | `40` | `0/0/5/35` | `2610..=2614` (comp `3772`) | `Phase 1/2` |
| [`PKG-ACT4-EXP16-Z15-18`](#pkg-act4-exp16-z15-18) | Exp `16` (`Z15..Z18`) | `4` | `0/0/4/0` | `33` | `0/0/28/5` | `2615..=2618` (comp `3773`) | `Phase 1/2` |
| [`PKG-ACT4-EXP17-Z00-04`](#pkg-act4-exp17-z00-04) | Exp `17` (`Z00..Z04`) | `5` | `0/4/0/1` | `40` | `0/29/0/11` | `2700..=2704` (comp `3780`) | `Phase 2` |
| [`PKG-ACT4-EXP17-Z05-08`](#pkg-act4-exp17-z05-08) | Exp `17` (`Z05..Z08`) | `4` | `0/4/0/0` | `32` | `0/30/0/2` | `2705..=2708` (comp `3781`) | `Phase 2` |
| [`PKG-ACT4-EXP17-Z09-12`](#pkg-act4-exp17-z09-12) | Exp `17` (`Z09..Z12`) | `4` | `0/4/0/0` | `32` | `0/31/0/1` | `2709..=2712` (comp `3782`) | `Phase 2` |
| [`PKG-ACT4-EXP17-Z13-17`](#pkg-act4-exp17-z13-17) | Exp `17` (`Z13..Z17`) | `5` | `0/2/0/3` | `41` | `0/14/1/26` | `2713..=2717` (comp `3783`) | `Phase 2` |
| [`PKG-ACT4-EXP01-SPATIAL`](#pkg-act4-exp01-spatial) | Exp `01` (`01_colors`) | `0` (spatial) | `[TIER-B]` (`130` keys / `72` nodes) | `0` | — | `1100..=1104` (comp `3601..=3604`) | `Phase 3` |
| [`PKG-ACT4-EXP02-SPATIAL`](#pkg-act4-exp02-spatial) | Exp `02` (`02_music`) | `0` (spatial) | `[TIER-B]` (`48x4` fil. / `20` keys) | `0` | — | `1200..=1203` (comp `3610..=3614`) | `Phase 3` |
| **TOTAL (`14` Cards)** | **Act IV (`15, 16, 17, 01, 02`)** | **`53`** | **`0/20/19/14` + 2 spatial** | **`417`** | **`0/138/125/154`** | **`1100..=2717` + `3601..=3783`** | **Phase 1–3** |

---

## 4.1 Experience 15 (`15_complexity`) — *Espectro de la Complejidad* (`16` Zones, `119` Features)

<a id="pkg-act4-exp15-z00-03"></a>
### PKG-ACT4-EXP15-Z00-03: Exp `15` — Foundations of Reality, Physical Laws & Collective Matter (`Z00..Z03`)

- **Phase & Priority:** `Phase 1/2` · **P1 — High (Removes protocell/hypercycle physics mismatches + wires condensed-matter cues)** · **Scope:** `4` Zones (`Z00..Z03`), `29` Features (`doc_feat` `0..k`)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:55, 731-744`](../../crates/cosmos_render/src/doc_stages.rs#L55) & [`shaders/hero_doc_matter_mind.wgsl:609, 630-686`](../../shaders/hero_doc_matter_mind.wgsl#L630-L686); `vid`: `doc_stage_variant_id(15)` -> `10 + (sc % 4)` (`10, 11, 12, 13`; catalog `scene = null`); `doc_feat(inst)` unused in WGSL; `color_b` zone accent unused (`doc_stages.rs:731-744`). **Card Tiers — Zones:** `0A / 0B / 2C / 2D` · **Features:** `0A / 0B / 14C / 15D`.
- **Compute & Plumbing Focus:** Compute Mode `12` (`cs_advect_particles` prebiotic thermal vortex; add calm symmetric lattice advection toggle via `submode` for `Z01..Z03`). Extra files: `crates/cosmos_render/src/universe_scene.rs` (`color_b` zone accent propagation for Exp `15..=17` at `doc_stages.rs:731-744`), `shaders/compute_universe.wgsl` (`L310-L318`).

#### Reserved Target IDs

| Zone | Slug | ES / EN Title | Current `vid` | Reserved Target `vid` | Companion IDs | `submode` & `doc_feat` Range |
|---|---|---|---:|---:|---|---|
| `Z00` | `reality` | La naturaleza de la realidad / *The nature of reality* | `10` (`[TIER-C]`) | **`2500`** | None (primary hero slots `2500..=2503`) | `submode=0` overview (`0xFF`), `doc_feat=0..5` |
| `Z01` | `laws` | Leyes y simetrías / *Laws and symmetries* | `11` (`[TIER-C]`) | **`2501`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z02` | `breaking` | Ruptura de simetría / *Symmetry breaking* | `12` (`[TIER-D]`) | **`2502`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..6` |
| `Z03` | `matter` | Estados colectivos de la materia / *Collective states of matter* | `13` (`[TIER-D]`) | **`2503`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |

#### Spoken Transcript Summary & 3D Visual Blueprint

Key deliverables for this Pick-Up Card:
- `vid=2500` (`Z00 reality`): 6-Ring Concentric Scale Ladder SDF (particle lattice -> ecosystem torus) connected by vertical emergence filaments (`doc_feat 0..5`).
- `vid=2501` (`Z01 laws`): 3D Symmetric Polyhedral Phase Chamber with invariant icosahedral cage + twin trajectories diverging from 1 mm seed delta (`doc_feat 0..7`).
- `vid=2502` (`Z02 breaking`): 3D Mexican-Hat quartic potential well `V(r)` + 4 competing orientation domains partitioned by neon vortex defect lines (`doc_feat 0..6`).
- `vid=2503` (`Z03 matter`): 4-Quadrant Condensed-Matter Phase Cube (FCC crystal lattice, laminar flow Venturi stream, ferromagnetic dipoles, Meissner flux-expelling superconducting torus; `doc_feat 0..7`).

##### `[TIER-C]` E15 `Z00` (`reality`) -> Target `vid=2500` | La naturaleza de la realidad / *The nature of reality* (`0A / 0B / 6C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Punto de partida · de lo fundamental a lo emergente* / *Starting point · from the fundamental to the emergent* | **Evidence (ES / EN):** `Continuidad con Realidad Emergente` (*Continuity with Emergent Reality*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** El recorrido anterior terminó preguntando qué es fundamental. Este comienza con la dirección inversa: cómo pueden surgir organización, función y novedad colectiva a partir de reglas e ingredientes más simples.
- **Spoken Macro Summary (EN):** The previous tour ended by asking what is fundamental. This begins with the reverse direction: how organization, function, and collective novelty can emerge from simpler rules and ingredients.
- **Current Visual & Diagnosis:** `vid = 10` via `hero_doc_matter_mind.wgsl:633-654` (`c_mode=0`) -> Lorenz butterfly strange attractor (2 tilted 3-ring scrolls + 2 saddle capsules + core). Routed via `15 => 10 + (0 % 4) = 10` (`c_mode=0`, Lorenz butterfly). Chaos scroll wings do not illustrate nested emergent levels (particles -> materials -> chemical networks -> cells -> organisms -> ecosystems).
- **3D Procedural Blueprint:** Replace with a **6-Ring Concentric Scale Ladder SDF**: nested rotating translucent shells spanning micro-particle lattice core up to ecosystem torus halo, connected by vertical emergence filaments; compute `mode=12` particle upward vortex.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **realidad** (*reality*) | **ES:** La realidad observable contiene niveles de organización cuyas propiedades no aparecen en una sola partícula, molécula o regla aislada.<br>**EN:** Observable reality contains levels of organization whose properties do not appear in a single isolated particle, molecule, or rule. | **`[TIER-C]`** | `doc_feat==0`: Pulse innermost quark/particle core lattice inside translucent outer spheres |
| `F1` | **emergencia** (*emergence*) | **ES:** Una propiedad emergente surge de interacciones colectivas y no puede atribuirse sin más a un componente individual.<br>**EN:** An emergent property arises from collective interactions and cannot simply be attributed to any individual component. | **`[TIER-C]`** | `doc_feat==1`: Highlight cross-scale coupling filaments bridging bottom lattice to upper ring |
| `F2` | **nivel** (*level*) | **ES:** Las explicaciones pueden operar en niveles distintos: partículas, materiales, redes químicas, células, organismos y ecosistemas.<br>**EN:** Explanations can operate at different levels: particles, materials, chemical networks, cells, organisms and ecosystems. | **`[TIER-C]`** | `doc_feat==2`: Sequentially illuminate each of the 6 hierarchical shells (particle -> ecosystem) |
| `F3` | **componente** (*component*) | **ES:** Los componentes importan, pero también importan sus conexiones, cantidades, condiciones y flujos de energía.<br>**EN:** Components matter, as do their connections, quantities, conditions, and energy flows. | **`[TIER-C]`** | `doc_feat==3`: Explode view radial spacing showing node connections, counts, and boundary ring |
| `F4` | **interacción** (*interaction*) | **ES:** La complejidad empieza cuando muchos elementos se afectan mutuamente mediante reglas locales.<br>**EN:** Complexity begins when many elements affect each other through local rules. | **`[TIER-C]`** | `doc_feat==4`: Flash pairwise attraction/repulsion arcs inside lowest layer |
| `F5` | **pregunta** (*question*) | **ES:** ¿Cómo pueden unas pocas leyes producir estructuras y comportamientos que no existen en sus componentes aislados?<br>**EN:** How can a few laws produce structures and behaviors that do not exist in their isolated components? | **`[TIER-C]`** | `doc_feat==5`: Pulse outer question-mark torus halo + directional upward gradient shimmer |

##### `[TIER-C]` E15 `Z01` (`laws`) -> Target `vid=2501` | Leyes y simetrías / *Laws and symmetries* (`0A / 0B / 8C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Reglas locales · conservación · probabilidad · condiciones* / *Local rules · conservation · probability · conditions* | **Evidence (ES / EN):** `Principios físicos establecidos` (*Established physical principles*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** Una misma ley puede generar resultados muy distintos cuando cambian el estado inicial, los límites o las fluctuaciones. Las reglas restringen lo posible sin especificar cada detalle del resultado.
- **Spoken Macro Summary (EN):** The same law can generate very different results when the initial state, limits or fluctuations change. Rules restrict what is possible without specifying every detail of the outcome.
- **Current Visual & Diagnosis:** `vid = 11` via `hero_doc_matter_mind.wgsl:655-663` (`c_mode=1`) -> Sinusoidal Turing coral sphere + bottom plate (`field_grid_tex` NOT sampled). Routed via `1 % 4 = 1` -> `vid=11` (`c_mode=1`, sinusoidal Turing coral + plate). Coral sphere gives zero intuition for physical laws, symmetry transformations, Noether conservation, or divergent initial/boundary trajectories.
- **3D Procedural Blueprint:** **3D Symmetric Polyhedral Phase Chamber**: translucent icosahedral cage (discrete + continuous rotational symmetries) enclosing two twin trajectory ribbons diverging from nearly identical initial seeds.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **ley** (*law*) | **ES:** Una ley física resume regularidades y relaciones que se cumplen en un dominio de condiciones; no es una instrucción que diseñe cada resultado.<br>**EN:** A physical law summarizes regularities and relations that hold in a domain of conditions; it is not an instruction that designs every outcome. | **`[TIER-C]`** | `doc_feat==0`: Wireframe invariant law-surface grid bounding the chamber |
| `F1` | **regla** (*rule*) | **ES:** Una regla local determina cómo responde un elemento a su entorno inmediato.<br>**EN:** A local rule determines how an element responds to its immediate environment. | **`[TIER-C]`** | `doc_feat==1`: Short-range neighbor stencil glow highlighting immediate voxel coupling |
| `F2` | **simetría** (*symmetry*) | **ES:** Una simetría expresa que cierta transformación deja invariables las leyes o una descripción física.<br>**EN:** A symmetry expresses that a certain transformation leaves the laws or a physical description unchanged. | **`[TIER-C]`** | `doc_feat==2`: Rotate icosahedral cage smoothly onto self-congruent symmetry axes |
| `F3` | **conservación** (*conservation*) | **ES:** Magnitudes como energía, momento o carga se conservan bajo condiciones asociadas a simetrías fundamentales.<br>**EN:** Magnitudes such as energy, momentum or charge are conserved under conditions associated with fundamental symmetries. | **`[TIER-C]`** | `doc_feat==3`: Dual counter-balancing golden flux rings maintaining constant summed intensity |
| `F4` | **probabilidad** (*probability*) | **ES:** La probabilidad organiza resultados posibles cuando existe incertidumbre, ruido o comportamiento cuántico.<br>**EN:** Probability organizes possible outcomes when there is uncertainty, noise, or quantum behavior. | **`[TIER-C]`** | `doc_feat==4`: Volumetric Gaussian probability cloud branching into faint alternate paths |
| `F5` | **condición inicial** (*initial condition*) | **ES:** El estado de partida puede dirigir un sistema hacia historias muy diferentes aun cuando las reglas sean idénticas.<br>**EN:** The starting state can direct a system toward very different stories even when the rules are identical. | **`[TIER-C]`** | `doc_feat==5`: Split starting seed dot by 1 mm and watch twin ribbons diverge exponentially |
| `F6` | **condición de frontera** (*boundary condition*) | **ES:** Los límites de un sistema restringen flujos, formas y patrones posibles.<br>**EN:** The boundaries of a system restrict possible flows, shapes, and patterns. | **`[TIER-C]`** | `doc_feat==6`: Morph bounding wall (box vs cylinder vs torus) reshaping interior flow field |
| `F7` | **posibilidad** (*possibility*) | **ES:** Las leyes delimitan un espacio de posibilidades; la historia concreta selecciona una trayectoria dentro de él.<br>**EN:** Laws delimit a space of possibilities; a system's actual history selects a path within it. | **`[TIER-C]`** | `doc_feat==7`: Illuminate envelope cone of allowed phase trajectories inside chamber |

##### `[TIER-D]` E15 `Z02` (`breaking`) -> Target `vid=2502` | Ruptura de simetría / *Symmetry breaking* (`0A / 0B / 0C / 7D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Estado uniforme → elección colectiva → dominios y defectos* / *Uniform state → collective choice → domains and defects* | **Evidence (ES / EN):** `Fenómeno físico reproducible` (*Reproducible physical phenomenon*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** Una ley puede ser simétrica aunque el estado producido por ella no lo sea. Pequeñas fluctuaciones pueden decidir qué orientación adopta cada región.
- **Spoken Macro Summary (EN):** A law can be symmetrical even if the state produced by it is not. Small fluctuations can decide which orientation each region takes.
- **Current Visual & Diagnosis:** `vid = 12` via `hero_doc_matter_mind.wgsl:664-676` (`c_mode=2`) -> 5-node Eigen Hypercycle HSL spheres + connecting capsule links. Routed via `2 % 4 = 2` -> `vid=12` (5-node Eigen Hypercycle). Narration explains unstable equilibrium, Mexican-hat potential symmetry breaking, magnetic/crystal domains, and topological vortices/domain walls — concrete physical structures absent from WGSL.
- **3D Procedural Blueprint:** **3D Mexican-Hat Potential Well + Multi-Domain Spin Lattice**: rotating quartic potential surface `V(r)` paired with a 3D spin-grid slab forming competing oriented domains separated by glowing defect vortex lines.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **equilibrio inestable** (*unstable equilibrium*) | **ES:** Un estado perfectamente balanceado puede existir matemáticamente y, sin embargo, ser tan sensible que cualquier perturbación lo aparte de él.<br>**EN:** A perfectly balanced state can exist mathematically and yet be so sensitive that any disturbance pushes it away from it. | **`[TIER-D]`** | `doc_feat==0`: Balance luminous rolling probe sphere precariously atop central Mexican-hat peak |
| `F1` | **ruptura espontánea** (*spontaneous symmetry breaking*) | **ES:** El sistema selecciona una de varias configuraciones equivalentes sin que la ley favorezca una dirección concreta.<br>**EN:** The system selects one of several equivalent configurations without the law favoring a specific direction. | **`[TIER-D]`** | `doc_feat==1`: Drop probe off central apex into circular trough, breaking circular symmetry |
| `F2` | **transición de fase** (*phase transition*) | **ES:** Al cruzar un umbral de temperatura, presión u otro parámetro, puede cambiar la organización colectiva del sistema.<br>**EN:** When temperature, pressure, or another parameter crosses a threshold, the system's collective organization may change. | **`[TIER-D]`** | `doc_feat==2`: Sweep temperature colormap across lattice, triggering sharp order-parameter snap |
| `F3` | **dominio** (*domain*) | **ES:** Regiones distintas pueden elegir orientaciones diferentes y formar dominios separados.<br>**EN:** Different regions can choose different orientations and form separate domains. | **`[TIER-D]`** | `doc_feat==3`: Partition lattice into 4 differently tinted magnetic/crystal domains |
| `F4` | **defecto** (*defect*) | **ES:** Donde dominios incompatibles se encuentran pueden quedar fronteras, vórtices u otros defectos.<br>**EN:** Where incompatible domains meet, boundaries, vortices or other defects may remain. | **`[TIER-D]`** | `doc_feat==4`: Ignite neon defect walls & topological vortex cores where incompatible domains meet |
| `F5` | **fluctuación** (*fluctuation*) | **ES:** Una variación diminuta puede romper la igualdad perfecta y amplificarse durante la transición.<br>**EN:** A tiny variation can break perfect equality and be amplified during the transition. | **`[TIER-D]`** | `doc_feat==5`: Inject stochastic jitter ripples atop peak to nucleate domain selection |
| `F6` | **estado colectivo** (*collective state*) | **ES:** La orientación elegida pertenece al conjunto, no a una partícula aislada.<br>**EN:** The chosen orientation belongs to the whole, not to an isolated particle. | **`[TIER-D]`** | `doc_feat==6`: Align entire domain interior vector field cleanly in one collective glow |

##### `[TIER-D]` E15 `Z03` (`matter`) -> Target `vid=2503` | Estados colectivos de la materia / *Collective states of matter* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Muchas partículas → nuevas propiedades del conjunto* / *Many particles → new properties of the whole* | **Evidence (ES / EN):** `Propiedades macroscópicas observadas` (*Observed macroscopic properties*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** Ninguna molécula aislada es húmeda y ningún átomo aislado es un sólido. Rigidez, viscosidad, magnetismo y superconductividad aparecen mediante organización colectiva.
- **Spoken Macro Summary (EN):** No isolated molecule is wet and no isolated atom is a solid. Rigidity, viscosity, magnetism and superconductivity appear through collective organization.
- **Current Visual & Diagnosis:** `vid = 13` via `hero_doc_matter_mind.wgsl:677-686` (`c_mode=3`) -> Budding dual-sphere lipid protocell (`r=0.32, 0.28`) + equatorial pinch torus. Routed via `3 % 4 = 3` -> `vid=13` (budding lipid protocell!). Depicting collective condensed-matter states (crystal solid, flowing liquid, ferromagnet, Cooper superconductor) with a biological budding bubble is a severe concrete physics omission.
- **3D Procedural Blueprint:** **4-Quadrant Condensed-Matter Phase Cube**: crystal cubic lattice, advected liquid Voronoi stream, aligned ferromagnetic dipole arrows, and quantized magnetic flux-exclusion Meissner ring around a superconducting torus.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **sólido** (*solid*) | **ES:** En un sólido, los componentes mantienen relaciones estructurales persistentes que permiten rigidez y elasticidad.<br>**EN:** In a solid, the components maintain persistent structural relationships that allow rigidity and elasticity. | **`[TIER-D]`** | `doc_feat==0`: Lock spheres into rigid vibrating crystalline FCC lattice with elastic bond struts |
| `F1` | **líquido** (*liquid*) | **ES:** Un líquido conserva cohesión y volumen aproximado, pero reorganiza continuamente sus vecinos y puede fluir.<br>**EN:** A liquid retains cohesion and approximate volume, but continually rearranges its neighbors and can flow. | **`[TIER-D]`** | `doc_feat==1`: Melt lattice struts into sliding cohesion-coupled liquid particle stream |
| `F2` | **flujo** (*flow*) | **ES:** El flujo describe el transporte colectivo de materia o energía a través del espacio.<br>**EN:** Flow describes the collective transport of matter or energy through space. | **`[TIER-D]`** | `doc_feat==2`: Drive directional laminar/turbulent stream arrows through Venturi throat |
| `F3` | **presión** (*pressure*) | **ES:** La presión macroscópica resume innumerables transferencias microscópicas de momento.<br>**EN:** Macroscopic pressure summarizes countless microscopic transfers of momentum. | **`[TIER-D]`** | `doc_feat==3`: Bombard elastic chamber membrane wall with high-velocity particle impact flashes |
| `F4` | **viscosidad** (*viscosity*) | **ES:** La viscosidad mide la resistencia interna de un fluido a deformarse o fluir.<br>**EN:** Viscosity measures the internal resistance of a fluid to deform or flow. | **`[TIER-D]`** | `doc_feat==4`: Bend shear velocity profile layers showing viscous drag between fluid strata |
| `F5` | **magnetismo** (*magnetism*) | **ES:** La alineación cooperativa de muchos momentos magnéticos puede producir un imán estable.<br>**EN:** The cooperative alignment of many magnetic moments can produce a stable magnet. | **`[TIER-D]`** | `doc_feat==5`: Snap randomized dipole needles into parallel ferromagnetic domain alignment |
| `F6` | **superconductividad** (*superconductivity*) | **ES:** En ciertos materiales y condiciones, un estado cuántico colectivo permite corriente sin resistencia eléctrica convencional.<br>**EN:** In certain materials and conditions, a collective quantum state allows current without conventional electrical resistance. | **`[TIER-D]`** | `doc_feat==6`: Extinguish resistance heat inside ring coil & expel magnetic field lines (Meissner) |
| `F7` | **transición** (*transition*) | **ES:** Cambiar temperatura, presión o campo puede reorganizar el conjunto y crear una fase diferente.<br>**EN:** Changing temperature, pressure or field can rearrange the whole and create a different phase. | **`[TIER-D]`** | `doc_feat==7`: Sweep vertical phase boundary plane converting solid lattice smoothly into liquid |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP15-Z00-03)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP15-Z00-03` (Odyssey Act IV — Experience 15 Zones `Z00..Z03`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files (4+1 focused slice):
1. `crates/cosmos_render/src/doc_stages.rs` — route `experience_id == 15` `Z00..=Z03` directly to reserved `vid = 2500..=2503` (and populate `color_b = [zone_c.x, zone_c.y, zone_c.z, 1.0]` at `L739`), plus companion emitters `None (primary hero slots `2500..=2503`)` where specified.
2. `crates/cosmos_narration/src/resolver.rs` — enrich `SegmentVisualHint` / `DOMAIN_CUE_KEYWORDS` (`base_flags` / `cue_tags` / `telemetry`) for the Exp `15` `Z00..=Z03` spoken concepts.
3. `shaders/hero_doc_matter_mind.wgsl` — handle decoded `vid >= 2500 && vid <= 2503` inside `march_doc_bio_mind` with distinct 3D procedural SDF / volumetric structures AND `switch (doc_feat(inst))` sub-stage branching (`0..k`, `0xFF` macro overview) smoothly driven by `doc_cue(inst)` & `doc_weight(inst)` (`0.72..1.35x` HDR cap).
4. `crates/cosmos_render/src/universe_scene.rs` + `shaders/compute_universe.wgsl` — ensure `field_grid_tex` / particle compute mode matches each zone (Compute Mode `12` (`cs_advect_particles` prebiotic thermal vortex; add calm symmetric lattice advection toggle via `submode` for `Z01..Z03`)).

Per-Zone & `doc_feat` Blueprints:
  - `Z00` `reality` (`vid=2500`): Replace with a **6-Ring Concentric Scale Ladder SDF**: nested rotating translucent shells spanning micro-particle lattice core up to ecosystem torus halo, connected by vertical emergence filaments; compute `mode=12` particle upward vortex. Features: `F0` realidad/reality, `F1` emergencia/emergence, `F2` nivel/level, `F3` componente/component, `F4` interacción/interaction, `F5` pregunta/question.
  - `Z01` `laws` (`vid=2501`): **3D Symmetric Polyhedral Phase Chamber**: translucent icosahedral cage (discrete + continuous rotational symmetries) enclosing two twin trajectory ribbons diverging from nearly identical initial seeds. Features: `F0` ley/law, `F1` regla/rule, `F2` simetría/symmetry, `F3` conservación/conservation, `F4` probabilidad/probability, `F5` condición inicial/initial condition, `F6` condición de frontera/boundary condition, `F7` posibilidad/possibility.
  - `Z02` `breaking` (`vid=2502`): **3D Mexican-Hat Potential Well + Multi-Domain Spin Lattice**: rotating quartic potential surface `V(r)` paired with a 3D spin-grid slab forming competing oriented domains separated by glowing defect vortex lines. Features: `F0` equilibrio inestable/unstable equilibrium, `F1` ruptura espontánea/spontaneous symmetry breaking, `F2` transición de fase/phase transition, `F3` dominio/domain, `F4` defecto/defect, `F5` fluctuación/fluctuation, `F6` estado colectivo/collective state.
  - `Z03` `matter` (`vid=2503`): **4-Quadrant Condensed-Matter Phase Cube**: crystal cubic lattice, advected liquid Voronoi stream, aligned ferromagnetic dipole arrows, and quantized magnetic flux-exclusion Meissner ring around a superconducting torus. Features: `F0` sólido/solid, `F1` líquido/liquid, `F2` flujo/flow, `F3` presión/pressure, `F4` viscosidad/viscosity, `F5` magnetismo/magnetism, `F6` superconductividad/superconductivity, `F7` transición/transition.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; keep volumetric shells tightly bounded inside hero sphere (`r <= 0.96`, early-exit `trans < 0.03`) with zero full-screen fog wash.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps for new documentary hero shells, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

<a id="pkg-act4-exp15-z04-07"></a>
### PKG-ACT4-EXP15-Z04-07: Exp `15` — Non-Equilibrium Dissipation, Fluctuations, Patterns & Feedback Loops (`Z04..Z07`)

- **Phase & Priority:** `Phase 1/2` · **P1 — High (Connects live Gray-Scott `field_grid_tex` + fixes Turing/protocell leaks)** · **Scope:** `4` Zones (`Z04..Z07`), `29` Features (`doc_feat` `0..k`)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:55, 731-744`](../../crates/cosmos_render/src/doc_stages.rs#L55) & [`shaders/hero_doc_matter_mind.wgsl:609, 630-686`](../../shaders/hero_doc_matter_mind.wgsl#L630-L686); `vid`: `doc_stage_variant_id(15)` -> `10 + (sc % 4)` (`10, 11, 12, 13`; catalog `scene = null`); `doc_feat(inst)` unused in WGSL; `color_b` zone accent unused (`doc_stages.rs:731-744`). **Card Tiers — Zones:** `0A / 1B / 2C / 1D` · **Features:** `0A / 7B / 10C / 12D`.
- **Compute & Plumbing Focus:** Compute Mode `12` Rayleigh-Benard curl-noise + `dispatch_field_grid((_, 15)) => Some(0)` Gray-Scott 2D PDE (`compute_universe.wgsl:396-413`) sampled in `vid=2506` (`Z06`). Extra files: `shaders/compute_universe.wgsl` (`L396-L413` `cs_field_grid` mode 0), `shaders/doc_common.wgsl` (`field_grid_tex` binding at `L24`).

#### Reserved Target IDs

| Zone | Slug | ES / EN Title | Current `vid` | Reserved Target `vid` | Companion IDs | `submode` & `doc_feat` Range |
|---|---|---|---:|---:|---|---|
| `Z04` | `nonequilibrium` | Lejos del equilibrio / *Far from equilibrium* | `10` (`[TIER-B]`) | **`2504`** | `3760` (optional external heat-reservoir halo frame for `Z04`) | `submode=0` overview (`0xFF`), `doc_feat=0..6` |
| `Z05` | `fluctuations` | Fluctuaciones / *Fluctuations* | `11` (`[TIER-C]`) | **`2505`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..5` |
| `Z06` | `patterns` | Patrones espontáneos / *Spontaneous patterns* | `12` (`[TIER-D]`) | **`2506`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z07` | `feedback` | Retroalimentación / *Feedback* | `13` (`[TIER-C]`) | **`2507`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |

#### Spoken Transcript Summary & 3D Visual Blueprint

Key deliverables for this Pick-Up Card:
- `vid=2504` (`Z04 nonequilibrium`): Retain Lorenz butterfly strange attractor inside a hot-bottom (`#ff6b29`) / cold-top (`#2efaa8`) open-flux Dissipative Heat-Gradient Frame (`doc_feat 0..6`).
- `vid=2505` (`Z05 fluctuations`): Replace sinusoidal coral with 3D FBM stochastic bifurcation tree & metastable potential wells (`doc_feat 0..6`).
- `vid=2506` (`Z06 patterns`): Replace 5-node hypercycle with 3D spherical Gray-Scott morphogenesis shell sampling live `field_grid_tex` (`mode=0`) for Turing spots/stripes (`doc_feat 0..6`).
- `vid=2507` (`Z07 feedback`): Replace budding protocell with Dual Torus Closed Causal Loop (green amplifying `+` outer halo + cyan stabilizing `-` governor torus + runaway tipping threshold; `doc_feat 0..7`).

##### `[TIER-B]` E15 `Z04` (`nonequilibrium`) -> Target `vid=2504` | Lejos del equilibrio / *Far from equilibrium* (`0A / 7B / 0C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Gradientes + flujo de energía + disipación* / *Gradients + energy flow + dissipation* | **Evidence (ES / EN):** `Termodinámica de sistemas abiertos` (*Thermodynamics of open systems*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** Muchos sistemas complejos persisten porque reciben energía, transforman recursos y expulsan entropía hacia su entorno. Cuando el flujo se detiene, su estructura dinámica puede desaparecer.
- **Spoken Macro Summary (EN):** Many complex systems persist because they receive energy, transform resources, and export entropy to their surroundings. When the flow stops, their dynamic structure may disappear.
- **Current Visual & Diagnosis:** `vid = 10` via `hero_doc_matter_mind.wgsl:633-654` (`c_mode=0`) -> Lorenz butterfly strange attractor (2 tilted 3-ring scrolls + 2 saddle capsules + core). Lorenz strange attractor (`vid=10`) + `mode=12` Rayleigh-Benard thermal curl particles already embody dissipative far-from-equilibrium dynamics, making it a valid `[TIER-B]` archetype; lacks explicit heat-bath boundary box, chemical gradient vectors, and `doc_feat` response.
- **3D Procedural Blueprint:** Retain Lorenz attractor + Rayleigh-Benard `mode=12` curl particles; wrap in an open-flux **Dissipative Heat-Gradient Frame** (hot lower plate orange `#ff6b29` -> cold upper sink cyan `#2efaa8`) wired to `doc_feat`.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **gradiente** (*gradient*) | **ES:** Un gradiente es una diferencia espacial de temperatura, concentración, presión o potencial capaz de impulsar un flujo.<br>**EN:** A gradient is a spatial difference in temperature, concentration, pressure or potential capable of driving a flow. | **`[TIER-B]`** | `doc_feat==0`: Illuminate hot-to-cold vertical color ramp & iso-potential gradient slices |
| `F1` | **sistema abierto** (*open system*) | **ES:** Un sistema abierto intercambia energía o materia con el entorno y puede sostener estados imposibles en equilibrio cerrado.<br>**EN:** An open system exchanges energy or matter with the environment and can sustain impossible states in closed equilibrium. | **`[TIER-B]`** | `doc_feat==1`: Open wireframe boundary ports streaming particles inward & waste heat outward |
| `F2` | **flujo de calor** (*heat flow*) | **ES:** El calor fluye espontáneamente de regiones más calientes a más frías mientras exista una diferencia de temperatura.<br>**EN:** Heat flows spontaneously from hotter to colder regions as long as there is a temperature difference. | **`[TIER-B]`** | `doc_feat==2`: Accelerate buoyant convection plumes rising from bottom plate toward cold top |
| `F3` | **potencial químico** (*chemical potential*) | **ES:** Las diferencias de potencial químico impulsan reacciones y transporte de componentes.<br>**EN:** Chemical potential differences drive reactions and transport of components. | **`[TIER-B]`** | `doc_feat==3`: Pulse concentration difference vectors spiraling around Lorenz saddle lobes |
| `F4` | **disipación** (*dissipation*) | **ES:** La energía útil se dispersa en formas menos disponibles mientras el sistema realiza procesos irreversibles.<br>**EN:** Useful energy is dispersed into less available forms as the system carries out irreversible processes. | **`[TIER-B]`** | `doc_feat==4`: Emit irreversible fading infrared halo rings radiating away from wing orbits |
| `F5` | **entropía** (*entropy*) | **ES:** El orden local no viola la segunda ley: el sistema y su entorno, considerados juntos, producen entropía.<br>**EN:** Local order does not violate the second law: the system and its environment, considered together, produce entropy. | **`[TIER-B]`** | `doc_feat==5`: Contrast dense low-entropy ordered core against expanding outer entropy cloud |
| `F6` | **persistencia** (*persistence*) | **ES:** Una estructura disipativa puede mantenerse solo mientras atraviesa el flujo que la sostiene.<br>**EN:** A dissipative structure can persist only while the sustaining flow passes through it. | **`[TIER-B]`** | `doc_feat==6`: Throttle inflow to zero -> watch wings collapse; restore flow -> attractor reform |

##### `[TIER-C]` E15 `Z05` (`fluctuations`) -> Target `vid=2505` | Fluctuaciones / *Fluctuations* (`0A / 0B / 6C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Ruido microscópico · sensibilidad · selección de trayectorias* / *Microscopic noise · sensitivity · trajectory selection* | **Evidence (ES / EN):** `Variación inevitable y medible` (*Unavoidable and measurable variation*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** Las fluctuaciones no son siempre un defecto. Pueden explorar posibilidades, iniciar transiciones, romper simetrías y ofrecer variación sobre la que actúan procesos de selección.
- **Spoken Macro Summary (EN):** Fluctuations are not always a defect. They can explore possibilities, initiate transitions, break symmetries, and offer variation upon which selection processes act.
- **Current Visual & Diagnosis:** `vid = 11` via `hero_doc_matter_mind.wgsl:655-663` (`c_mode=1`) -> Sinusoidal Turing coral sphere + bottom plate (`field_grid_tex` NOT sampled). Routed via `5 % 4 = 1` -> `vid=11` (deterministic sinusoidal coral). Shows zero stochastic noise, micro-to-macro sensitivity amplification, nucleation seeding, or stochastic state-space exploration.
- **3D Procedural Blueprint:** **Stochastic Perturbation & Bifurcation Tree**: glowing probability mist feeding a 3D FBM-perturbed seed crystal / double-bifurcation tree where tiny random perturbations spawn distinct macroscopic branches.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **ruido** (*noise*) | **ES:** El ruido reúne variaciones rápidas o impredecibles. Puede ocultar señales, pero también activar transiciones y exploración.<br>**EN:** Noise consists of rapid or unpredictable variations. It can hide signals, but can also trigger transitions and exploration. | **`[TIER-C]`** | `doc_feat==0`: Overlay animated high-frequency 3D curl-noise jitter haze across stage |
| `F1` | **variación** (*variation*) | **ES:** Pequeñas diferencias entre elementos o momentos impiden que todos sigan exactamente la misma historia.<br>**EN:** Small differences between elements or moments prevent them all from following exactly the same story. | **`[TIER-C]`** | `doc_feat==1`: Render 5 subtly diverging spiral filaments from clustered micro-seeds |
| `F2` | **sensibilidad** (*sensitivity*) | **ES:** Algunos sistemas amplifican diferencias iniciales diminutas y producen resultados macroscópicos divergentes.<br>**EN:** Some systems amplify minute initial differences and produce divergent macroscopic results. | **`[TIER-C]`** | `doc_feat==2`: Magnify tiny initial delta vector at base into wide downstream separation |
| `F3` | **azar** (*chance*) | **ES:** El azar introduce resultados no determinados por la información disponible, sin eliminar las restricciones de las leyes.<br>**EN:** Chance introduces results not determined by the available information, without eliminating the restrictions of the laws. | **`[TIER-C]`** | `doc_feat==3`: Spark probabilistic stochastic hops between discrete metastable wells |
| `F4` | **semilla** (*seed*) | **ES:** Una perturbación local puede actuar como semilla de un dominio, cristal, onda o nueva rama química.<br>**EN:** A local perturbation can act as a seed for a domain, crystal, wave, or new chemical branch. | **`[TIER-C]`** | `doc_feat==4`: Flash a single glowing nucleation grain triggering outward crystal/wave front |
| `F5` | **exploración** (*exploration*) | **ES:** La variación permite que un sistema visite estados alternativos en lugar de quedar fijado en una única configuración.<br>**EN:** Variation allows a system to visit alternative states rather than being locked into a single configuration. | **`[TIER-C]`** | `doc_feat==5`: Sweep roving probe cloud testing 4 candidate basin minima dynamically |

##### `[TIER-D]` E15 `Z06` (`patterns`) -> Target `vid=2506` | Patrones espontáneos / *Spontaneous patterns* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Ondas · espirales · franjas · manchas · ramificación* / *Waves · spirals · stripes · spots · branching* | **Evidence (ES / EN):** `Autoorganización física reproducible` (*Reproducible physical self-organization*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** El orden puede aparecer sin un controlador central. Interacciones locales, difusión, reacción y flujo pueden producir formas globales regulares.
- **Spoken Macro Summary (EN):** Order can appear without a central controller. Local interactions, diffusion, reaction and flow can produce regular global shapes.
- **Current Visual & Diagnosis:** `vid = 12` via `hero_doc_matter_mind.wgsl:664-676` (`c_mode=2`) -> 5-node Eigen Hypercycle HSL spheres + connecting capsule links. Routed via `6 % 4 = 2` -> `vid=12` (5-node hypercycle). Worst technical waste in Exp 15: `universe_scene.rs:217-218` ALREADY dispatches Gray-Scott PDE `cs_field_grid` (`mode=0`) for Exp 15, yet `hero_doc_matter_mind.wgsl` NEVER samples `field_grid_tex`, and `Z06` isn't even routed to `vid=11` Turing coral!
- **3D Procedural Blueprint:** **Multi-Pattern Morphogenesis Specimen Table** actively sampling `field_grid_tex`: Rayleigh-Benard hexagonal convection prisms, octahedral crystal growth, live GPU Gray-Scott Turing spots/stripes, 2D spiral wave rotor, DLA fractal branching tree, and synced phase-locked ring.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **convección** (*convection*) | **ES:** Un fluido calentado desde abajo puede organizarse en celdas de circulación cuando el gradiente supera un umbral.<br>**EN:** A fluid heated from below can organize into circulation cells when the gradient exceeds a threshold. | **`[TIER-D]`** | `doc_feat==0`: Extrude rotating hexagonal Rayleigh-Benard convection cells with upwelling cores |
| `F1` | **cristalización** (*crystallization*) | **ES:** Un patrón cristalino crece cuando unidades compatibles se incorporan a una estructura ordenada.<br>**EN:** A crystalline pattern grows when compatible units are incorporated into an ordered structure. | **`[TIER-D]`** | `doc_feat==1`: Grow faceted dendritic ice/mineral crystal octahedron from central seed |
| `F2` | **reacción-difusión** (*reaction-diffusion*) | **ES:** La combinación de reacciones locales y velocidades distintas de difusión puede generar manchas, franjas y ondas.<br>**EN:** The combination of local reactions and different diffusion rates can generate spots, stripes, and waves. | **`[TIER-D]`** | `doc_feat==2`: Sample live GPU `field_grid_tex` Gray-Scott activator/inhibitor stripes onto 3D slab |
| `F3` | **onda** (*wave*) | **ES:** Una perturbación puede propagarse mediante acoplamiento local sin transportar permanentemente los mismos componentes.<br>**EN:** A disturbance can propagate by local coupling without permanently transporting the same components. | **`[TIER-D]`** | `doc_feat==3`: Propagate concentric spherical/planar standing & traveling wave crests |
| `F4` | **espiral** (*spiral*) | **ES:** Frentes de excitación y recuperación pueden enrollarse y mantener ondas rotatorias.<br>**EN:** Excitation and recovery fronts can coil and maintain rotating waves. | **`[TIER-D]`** | `doc_feat==4`: Ignite rotating double-armed Belousov-Zhabotinsky Archimedean spiral rotor |
| `F5` | **ramificación** (*branching*) | **ES:** El crecimiento guiado por transporte y competencia puede producir árboles, venas, descargas y dendritas.<br>**EN:** Growth guided by transport and competition can produce trees, veins, discharges, and dendrites. | **`[TIER-D]`** | `doc_feat==5`: Grow recursive 3D space-colonization vascular / DLA branching tree |
| `F6` | **sincronización** (*synchronization*) | **ES:** Osciladores conectados pueden ajustar sus ritmos y formar un pulso colectivo.<br>**EN:** Connected oscillators can adjust their rhythms and form a collective pulse. | **`[TIER-D]`** | `doc_feat==6`: Synchronize 12 blinking boundary oscillator orbs into phase-locked golden pulse |
| `F7` | **sin diseñador** (*without designer*) | **ES:** Que un patrón sea ordenado no implica que una mente haya especificado cada una de sus partes.<br>**EN:** That a pattern is ordered does not imply that a mind has specified each of its parts. | **`[TIER-D]`** | `doc_feat==7`: Dissolve imaginary top-down wireframe grid leaving pure self-assembled pattern |

##### `[TIER-C]` E15 `Z07` (`feedback`) -> Target `vid=2507` | Retroalimentación / *Feedback* (`0A / 0B / 4C / 4D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Señal → respuesta → nueva señal* / *Signal → response → new signal* | **Evidence (ES / EN):** `Mecanismo general de regulación` (*General regulation mechanism*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** La retroalimentación positiva amplifica cambios; la negativa los corrige o estabiliza. Combinadas con retrasos y umbrales, ambas pueden producir regulación, oscilaciones o colapso.
- **Spoken Macro Summary (EN):** Positive feedback amplifies changes; negative feedback corrects or stabilizes them. Combined with delays and thresholds, both can produce regulation, oscillations, or collapse.
- **Current Visual & Diagnosis:** `vid = 13` via `hero_doc_matter_mind.wgsl:677-686` (`c_mode=3`) -> Budding dual-sphere lipid protocell (`r=0.32, 0.28`) + equatorial pinch torus. Routed via `7 % 4 = 3` -> `vid=13` (budding lipid protocell). A dividing membrane gives no visual expression of cybernetic signal/response loops, positive runaway gain, negative homeostasis, threshold switching, oscillation, or collapse.
- **3D Procedural Blueprint:** **Dual-Torus Cybernetic Feedback Loop & Regulator**: two counter-looping signal-carrying rings (`+` reinforcing amber vs `-` stabilizing cyan) orbiting a variable-radius core orb with a threshold ring gauge and collapse shatter.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **señal** (*signal*) | **ES:** Una señal es una variación capaz de modificar el comportamiento de otra parte del sistema.<br>**EN:** A signal is a variation capable of modifying the behavior of another part of the system. | **`[TIER-C]`** | `doc_feat==0`: Trace fast luminous signal packet traveling from sensor node to actuator |
| `F1` | **respuesta** (*response*) | **ES:** La respuesta transforma el estado del sistema y puede alterar la señal que la produjo.<br>**EN:** The response transforms the state of the system and can alter the signal that produced it. | **`[TIER-C]`** | `doc_feat==1`: Pulse downstream actuator ring deformation back toward input manifold |
| `F2` | **retroalimentación positiva** (*positive feedback*) | **ES:** Una salida refuerza el cambio inicial y puede acelerar crecimiento, activación o comportamiento desbocado.<br>**EN:** An output reinforces the initial change and may accelerate growth, activation, or runaway behavior. | **`[TIER-C]`** | `doc_feat==2`: Spiral positive feedback amber ring exponentially toward bright overdrive |
| `F3` | **retroalimentación negativa** (*negative feedback*) | **ES:** Una salida contrarresta la desviación y favorece estabilidad o recuperación.<br>**EN:** An output counteracts the deviation and promotes stability or recovery. | **`[TIER-C]`** | `doc_feat==3`: Counter-rotate cyan damping torus pulling core radius smoothly to setpoint |
| `F4` | **umbral** (*threshold*) | **ES:** Una respuesta puede activarse solo después de superar cierto nivel de señal.<br>**EN:** A response can be activated only after exceeding a certain signal level. | **`[TIER-D]`** | `doc_feat==4`: Show glass threshold barrier ring snapping open only once core exceeds radius |
| `F5` | **oscilación** (*oscillation*) | **ES:** Los retrasos entre señal y corrección pueden generar ciclos repetidos en vez de un equilibrio estático.<br>**EN:** Delays between signal and correction can lead to repeated cycles rather than static equilibrium. | **`[TIER-D]`** | `doc_feat==5`: Drive delayed standing sinusoidal breathing oscillation of core sphere |
| `F6` | **homeostasis** (*homeostasis*) | **ES:** La homeostasis mantiene variables dentro de intervalos viables mediante múltiples bucles de regulación.<br>**EN:** Homeostasis maintains variables within viable ranges through multiple regulatory loops. | **`[TIER-D]`** | `doc_feat==6`: Lock core inside tight green viability tolerance band despite outer noise |
| `F7` | **colapso** (*collapse*) | **ES:** Cuando la amplificación supera la capacidad de regulación, el sistema puede abandonar su estado organizado.<br>**EN:** When the amplification exceeds the regulation capacity, the system can leave its organized state. | **`[TIER-D]`** | `doc_feat==7`: Shatter/disperse core ring violently when positive runaway exceeds damping |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP15-Z04-07)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP15-Z04-07` (Odyssey Act IV — Experience 15 Zones `Z04..Z07`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files (4+1 focused slice):
1. `crates/cosmos_render/src/doc_stages.rs` — route `experience_id == 15` `Z04..=Z07` directly to reserved `vid = 2504..=2507` (and populate `color_b = [zone_c.x, zone_c.y, zone_c.z, 1.0]` at `L739`), plus companion emitters ``3760` (optional external heat-reservoir halo frame for `Z04`)` where specified.
2. `crates/cosmos_narration/src/resolver.rs` — enrich `SegmentVisualHint` / `DOMAIN_CUE_KEYWORDS` (`base_flags` / `cue_tags` / `telemetry`) for the Exp `15` `Z04..=Z07` spoken concepts.
3. `shaders/hero_doc_matter_mind.wgsl` — handle decoded `vid >= 2504 && vid <= 2507` inside `march_doc_bio_mind` with distinct 3D procedural SDF / volumetric structures AND `switch (doc_feat(inst))` sub-stage branching (`0..k`, `0xFF` macro overview) smoothly driven by `doc_cue(inst)` & `doc_weight(inst)` (`0.72..1.35x` HDR cap).
4. `crates/cosmos_render/src/universe_scene.rs` + `shaders/compute_universe.wgsl` — ensure `field_grid_tex` / particle compute mode matches each zone (Compute Mode `12` Rayleigh-Benard curl-noise + `dispatch_field_grid((_, 15)) => Some(0)` Gray-Scott 2D PDE (`compute_universe.wgsl:396-413`) sampled in `vid=2506` (`Z06`)).

Per-Zone & `doc_feat` Blueprints:
  - `Z04` `nonequilibrium` (`vid=2504`): Retain Lorenz attractor + Rayleigh-Benard `mode=12` curl particles; wrap in an open-flux **Dissipative Heat-Gradient Frame** (hot lower plate orange `#ff6b29` -> cold upper sink cyan `#2efaa8`) wired to `doc_feat`. Features: `F0` gradiente/gradient, `F1` sistema abierto/open system, `F2` flujo de calor/heat flow, `F3` potencial químico/chemical potential, `F4` disipación/dissipation, `F5` entropía/entropy, `F6` persistencia/persistence.
  - `Z05` `fluctuations` (`vid=2505`): **Stochastic Perturbation & Bifurcation Tree**: glowing probability mist feeding a 3D FBM-perturbed seed crystal / double-bifurcation tree where tiny random perturbations spawn distinct macroscopic branches. Features: `F0` ruido/noise, `F1` variación/variation, `F2` sensibilidad/sensitivity, `F3` azar/chance, `F4` semilla/seed, `F5` exploración/exploration.
  - `Z06` `patterns` (`vid=2506`): **Multi-Pattern Morphogenesis Specimen Table** actively sampling `field_grid_tex`: Rayleigh-Benard hexagonal convection prisms, octahedral crystal growth, live GPU Gray-Scott Turing spots/stripes, 2D spiral wave rotor, DLA fractal branching tree, and synced phase-locked ring. Features: `F0` convección/convection, `F1` cristalización/crystallization, `F2` reacción-difusión/reaction-diffusion, `F3` onda/wave, `F4` espiral/spiral, `F5` ramificación/branching, `F6` sincronización/synchronization, `F7` sin diseñador/without designer.
  - `Z07` `feedback` (`vid=2507`): **Dual-Torus Cybernetic Feedback Loop & Regulator**: two counter-looping signal-carrying rings (`+` reinforcing amber vs `-` stabilizing cyan) orbiting a variable-radius core orb with a threshold ring gauge and collapse shatter. Features: `F0` señal/signal, `F1` respuesta/response, `F2` retroalimentación positiva/positive feedback, `F3` retroalimentación negativa/negative feedback, `F4` umbral/threshold, `F5` oscilación/oscillation, `F6` homeostasis/homeostasis, `F7` colapso/collapse.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; keep volumetric shells tightly bounded inside hero sphere (`r <= 0.96`, early-exit `trans < 0.03`) with zero full-screen fog wash.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps for new documentary hero shells, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

<a id="pkg-act4-exp15-z08-11"></a>
### PKG-ACT4-EXP15-Z08-11: Exp `15` — Interaction Networks, Self-Organization, Information & Prebiotic Chemistry (`Z08..Z11`)

- **Phase & Priority:** `Phase 1/2` · **P2 — Medium-High (Builds 3D scale-free networks, reaction-diffusion self-org, bit-tape & catalytic cycles)** · **Scope:** `4` Zones (`Z08..Z11`), `30` Features (`doc_feat` `0..k`)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:55, 731-744`](../../crates/cosmos_render/src/doc_stages.rs#L55) & [`shaders/hero_doc_matter_mind.wgsl:609, 630-686`](../../shaders/hero_doc_matter_mind.wgsl#L630-L686); `vid`: `doc_stage_variant_id(15)` -> `10 + (sc % 4)` (`10, 11, 12, 13`; catalog `scene = null`); `doc_feat(inst)` unused in WGSL; `color_b` zone accent unused (`doc_stages.rs:731-744`). **Card Tiers — Zones:** `0A / 1B / 2C / 1D` · **Features:** `0A / 7B / 13C / 10D`.
- **Compute & Plumbing Focus:** Compute Mode `12` particle flow + `cs_field_grid` mode `0` (`Z09` Belousov-Zhabotinsky spirals & spontaneous Turing crystal). Extra files: `crates/cosmos_narration/src/resolver.rs` (`cue_tags` for catalytic/network keywords), `shaders/compute_universe.wgsl`.

#### Reserved Target IDs

| Zone | Slug | ES / EN Title | Current `vid` | Reserved Target `vid` | Companion IDs | `submode` & `doc_feat` Range |
|---|---|---|---:|---:|---|---|
| `Z08` | `networks` | Redes complejas / *Complex networks* | `10` (`[TIER-D]`) | **`2508`** | `3761` (optional external orbital monomer cloud for `Z11`) | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z09` | `selforganization` | Autoorganización / *Self-organization* | `11` (`[TIER-B]`) | **`2509`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..6` |
| `Z10` | `information` | Información funcional / *Functional information* | `12` (`[TIER-C]`) | **`2510`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z11` | `chemistry` | Química organizada / *Organized chemistry* | `13` (`[TIER-C]`) | **`2511`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..6` |

#### Spoken Transcript Summary & 3D Visual Blueprint

Key deliverables for this Pick-Up Card:
- `vid=2508` (`Z08 networks`): Replace Lorenz scrolls with 3D Scale-Free Force-Directed Graph (degree-weighted hubs, red bottleneck bridge node, cascading failure shockwave; `doc_feat 0..6`).
- `vid=2509` (`Z09 selforganization`): Wire live `field_grid_tex` BZ reaction spirals + hexagonal honeycomb Bénard cells self-assembling without external controller (`doc_feat 0..7`).
- `vid=2510` (`Z10 information`): Move algorithmic state encoding into a DNA/Bit-Tape Double Helix & Shannon noise channel (sender -> noisy channel -> Bayesian belief receiver; `doc_feat 0..7`).
- `vid=2511` (`Z11 chemistry`): Replace budding protocell with 3D Covalent Bond Tetrahedra + Eigen Autocatalytic Hypercycle ring + enzyme lock-and-key catalyst (`doc_feat 0..7`).

##### `[TIER-D]` E15 `Z08` (`networks`) -> Target `vid=2508` | Redes complejas / *Complex networks* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Nodos + conexiones → rutas, centros, cascadas* / *Nodes + connections → paths, hubs, cascades* | **Evidence (ES / EN):** `Estructura matemática y física observable` (*Observable mathematical and physical structure*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** La organización de las conexiones puede importar tanto como la naturaleza de los componentes. La misma lógica de red aparece en moléculas, neuronas, ecosistemas, redes eléctricas y sociedades.
- **Spoken Macro Summary (EN):** The organization of the connections can matter as much as the nature of the components. The same network logic appears in molecules, neurons, ecosystems, electrical networks, and societies.
- **Current Visual & Diagnosis:** `vid = 10` via `hero_doc_matter_mind.wgsl:633-654` (`c_mode=0`) -> Lorenz butterfly strange attractor (2 tilted 3-ring scrolls + 2 saddle capsules + core). Routed via `8 % 4 = 0` -> `vid=10` (Lorenz butterfly). Narrates graph topology—nodes, weighted edges, scale-free hubs, modular clusters, cascading failures, and redundancy—concrete network architecture completely absent from WGSL.
- **3D Procedural Blueprint:** **3D Scale-Free Modular Graph Topology**: 24 capsule-linked SDF spheres arranged into 3 densely linked clusters joined by 2 prominent high-degree gold hub nodes, with edge-traveling pulse cascades.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **nodo** (*node*) | **ES:** Un nodo representa una unidad del sistema: molécula, especie, neurona, central eléctrica, persona u otra entidad.<br>**EN:** A node represents a unit of the system: molecule, species, neuron, power plant, person, or other entity. | **`[TIER-D]`** | `doc_feat==0`: Pulse individual node spheres across 3 tinted community clusters |
| `F1` | **enlace** (*link*) | **ES:** Un enlace representa interacción, transporte, dependencia o comunicación entre nodos.<br>**EN:** A link represents interaction, transport, dependency or communication between nodes. | **`[TIER-D]`** | `doc_feat==1`: Illuminate capsule link pipes carrying bidirectional packet beads |
| `F2` | **centro** (*hub*) | **ES:** Un centro o hub concentra muchas conexiones y puede facilitar coordinación o crear dependencia crítica.<br>**EN:** A center or hub concentrates many connections and can facilitate coordination or create critical dependency. | **`[TIER-D]`** | `doc_feat==2`: Enlarge and intensely gold-glow 2 central high-degree hub nodes |
| `F3` | **grupo** (*group*) | **ES:** Los nodos pueden formar comunidades densamente conectadas con enlaces más escasos hacia otras comunidades.<br>**EN:** Nodes can form densely connected communities with sparser links to other communities. | **`[TIER-D]`** | `doc_feat==3`: Highlight 3 tight intra-cluster cages separated by sparse bridge links |
| `F4` | **cascada** (*cascade*) | **ES:** Una alteración puede propagarse de nodo en nodo y producir una respuesta extensa.<br>**EN:** A disturbance can spread from node to node and produce an extensive response. | **`[TIER-D]`** | `doc_feat==4`: Trigger red ripple failure propagating outward node-by-node across links |
| `F5` | **robustez** (*robustness*) | **ES:** Rutas alternativas y diversidad pueden permitir que una red mantenga función después de perder componentes.<br>**EN:** Alternative routes and diversity can allow a network to maintain function after losing components. | **`[TIER-D]`** | `doc_feat==5`: Sever one non-hub link and route bypass packets cleanly over redundant edge |
| `F6` | **fragilidad** (*fragility*) | **ES:** Una red eficiente puede ser vulnerable si depende de pocos centros o carece de redundancia.<br>**EN:** An efficient network can be vulnerable if it depends on few centers or lacks redundancy. | **`[TIER-D]`** | `doc_feat==6`: Delete major hub node causing dramatic downstream cluster blackout |
| `F7` | **topología** (*topology*) | **ES:** La topología describe quién se conecta con quién y condiciona el comportamiento colectivo.<br>**EN:** Topology describes who connects with whom and conditions collective behavior. | **`[TIER-D]`** | `doc_feat==7`: Rotate camera-facing full topology wireframe showing edge-density skeleton |

##### `[TIER-B]` E15 `Z09` (`selforganization`) -> Target `vid=2509` | Autoorganización / *Self-organization* (`0A / 7B / 0C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Reglas locales + energía + feedback + red* / *Local rules + energy + feedback + network* | **Evidence (ES / EN):** `Orden descentralizado observado` (*Observed decentralized organization*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** Ningún objeto maestro dirige la escena. El orden global emerge de coordinación local, circulación de señales, adaptación y mantenimiento continuo.
- **Spoken Macro Summary (EN):** No master object drives the scene. Global order emerges from local coordination, signal circulation, adaptation and continuous maintenance.
- **Current Visual & Diagnosis:** `vid = 11` via `hero_doc_matter_mind.wgsl:655-663` (`c_mode=1`) -> Sinusoidal Turing coral sphere + bottom plate (`field_grid_tex` NOT sampled). Routed via `9 % 4 = 1` -> `vid=11` (Turing coral sphere). Self-organizing morphogenesis matches `Z09` conceptually, earning `[TIER-B]`; however `field_grid_tex` is ignored and `doc_feat` gives zero visual cues for boid flocking, attractor convergence, adaptation, or component turnover.
- **3D Procedural Blueprint:** Bind `vid=11` to live `field_grid_tex` Turing sphere + orbiting **Boids / Particle Flock Halo** (`mode=6` or `mode=12` neighborhood rules) driven by `doc_feat`.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **regla local** (*local rule*) | **ES:** Cada elemento responde a vecinos y señales próximas sin conocer el estado completo del sistema.<br>**EN:** Each element responds to nearby neighbors and signals without knowing the complete state of the system. | **`[TIER-B]`** | `doc_feat==0`: Show glowing short-range neighbor radius shells around sampled coral nodes |
| `F1` | **comportamiento colectivo** (*collective behavior*) | **ES:** El conjunto puede desplazarse, oscilar, dividir tareas o mantener forma de maneras ausentes en cada unidad.<br>**EN:** The whole can move, oscillate, divide tasks or maintain shape in ways absent in each unit. | **`[TIER-B]`** | `doc_feat==1`: Coalesce outer orbiting swarm particles into coherent helical Murmuration wave |
| `F2` | **atractor** (*attractor*) | **ES:** Un atractor es un estado o patrón dinámico hacia el que convergen múltiples trayectorias del sistema.<br>**EN:** An attractor is a dynamic state or pattern toward which multiple trajectories of the system converge. | **`[TIER-B]`** | `doc_feat==2`: Draw converging vector funnel streams pulling particles into attractor ring |
| `F3` | **adaptación** (*adaptation*) | **ES:** Una organización adaptativa modifica su respuesta según condiciones y experiencias anteriores.<br>**EN:** An adaptive organization modifies its response based on previous conditions and experiences. | **`[TIER-B]`** | `doc_feat==3`: Shift Turing ridge wavelength smoothly in response to outer perturbation |
| `F4` | **automantenimiento** (*self-maintenance*) | **ES:** Un sistema se mantiene cuando sus procesos reparan o reemplazan componentes y conservan las condiciones necesarias para continuar.<br>**EN:** A system is maintained when its processes repair or replace components and maintain the conditions necessary to continue. | **`[TIER-B]`** | `doc_feat==4`: Replace fading surface particles continuously while preserving global coral silhouette |
| `F5` | **coordinación** (*coordination*) | **ES:** La coordinación descentralizada distribuye decisiones entre interacciones locales en lugar de concentrarlas en un controlador.<br>**EN:** Decentralized coordination distributes decisions among local interactions rather than concentrating them in one controller. | **`[TIER-B]`** | `doc_feat==5`: Suppress central core spotlight to emphasize peer-to-peer perimeter arcs |
| `F6` | **identidad dinámica** (*dynamic identity*) | **ES:** Un sistema puede conservar organización aunque sus componentes materiales cambien con el tiempo.<br>**EN:** A system can retain organization even though its material components change over time. | **`[TIER-B]`** | `doc_feat==6`: Cycle material constituent colors continuously while keeping SDF shell rock-steady |

##### `[TIER-C]` E15 `Z10` (`information`) -> Target `vid=2510` | Información funcional / *Functional information* (`0A / 0B / 8C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Patrón + contexto + efecto sobre la persistencia* / *Pattern + context + effect on persistence* | **Evidence (ES / EN):** `Concepto dependiente del sistema` (*System dependent concept*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** No todo orden es información biológicamente útil. Un patrón se vuelve funcional cuando, dentro de un sistema, contribuye de forma reproducible a mantener, regular o reproducir una organización.
- **Spoken Macro Summary (EN):** Not all order is biologically useful information. A pattern becomes functional when, within a system, it reproducibly contributes to maintaining, regulating or reproducing an organization.
- **Current Visual & Diagnosis:** `vid = 12` via `hero_doc_matter_mind.wgsl:664-676` (`c_mode=2`) -> 5-node Eigen Hypercycle HSL spheres + connecting capsule links. Routed via `10 % 4 = 2` -> `vid=12` (5 unlabelled Eigen balls). Does not convey pattern-vs-noise, memory storage, instruction decoding, bit errors, error correction, or functional selection.
- **3D Procedural Blueprint:** **3D Helical Symbol Ribbon + Interpreter Gate**: rotating perforated 4-symbol code ribbon passing through an active reading collar torus that outputs structured pulses vs red-flagged bit errors.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **patrón** (*pattern*) | **ES:** Un patrón contiene regularidades, pero su sola existencia no garantiza una función.<br>**EN:** A pattern contains regularities, but its mere existence does not guarantee a function. | **`[TIER-C]`** | `doc_feat==0`: Render static embossed periodic notches along passive background ring |
| `F1` | **memoria** (*memory*) | **ES:** La memoria permite que un estado anterior influya en respuestas posteriores.<br>**EN:** Memory allows a previous state to influence later responses. | **`[TIER-C]`** | `doc_feat==1`: Illuminate persistent glowing glyph notches retained from past cycle |
| `F2` | **señal** (*signal*) | **ES:** Una señal funcional modifica procesos de manera dependiente del contexto.<br>**EN:** A functional signal modifies processes in a context-dependent manner. | **`[TIER-C]`** | `doc_feat==2`: Send context-triggered activation spark down side branch cable |
| `F3` | **instrucción** (*instruction*) | **ES:** Una secuencia puede actuar como instrucción solo si existe una maquinaria capaz de interpretarla y producir consecuencias.<br>**EN:** A sequence can act as an instruction only if there is machinery capable of interpreting it and producing consequences. | **`[TIER-C]`** | `doc_feat==3`: Pass ribbon through glowing reader collar that spawns functional downstream wave |
| `F4` | **error** (*error*) | **ES:** Copiar, transmitir o ejecutar patrones introduce errores que pueden degradar función o generar novedad.<br>**EN:** Copying, transmitting or executing patterns introduces errors that can degrade function or generate novelty. | **`[TIER-C]`** | `doc_feat==4`: Flash magenta corrupted glyph notch causing downstream output distortion |
| `F5` | **corrección** (*correction*) | **ES:** La detección y reparación de errores aumenta la persistencia de información a través del tiempo.<br>**EN:** Error detection and repair increases the persistence of information over time. | **`[TIER-C]`** | `doc_feat==5`: Attach sweeping enzymatic proof-reader caliper snapping bad notch back to green |
| `F6` | **función** (*function*) | **ES:** Una función se define por el efecto que una estructura o proceso tiene dentro de un sistema y un entorno.<br>**EN:** A function is defined by the effect that a structure or process has within a system and environment. | **`[TIER-C]`** | `doc_feat==6`: Link output beam directly to sustaining closed persistence loop |
| `F7` | **respuesta ambiental** (*environmental response*) | **ES:** La información funcional puede relacionar señales externas con acciones que favorecen la continuidad del sistema.<br>**EN:** Functional information can relate external signals with actions that favor the continuity of the system. | **`[TIER-C]`** | `doc_feat==7`: Sensor tip on reader collar flips ribbon state when external cue shifts |

##### `[TIER-C]` E15 `Z11` (`chemistry`) -> Target `vid=2511` | Química organizada / *Organized chemistry* (`0A / 0B / 5C / 2D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Catálisis · ciclos · reconocimiento · compartimentos* / *Catalysis · cycles · recognition · compartments* | **Evidence (ES / EN):** `Química y autoensamblaje observados` (*Observed chemistry and self-assembly*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** Las moléculas pueden formar ciclos mutuamente sostenidos en los que los productos de unas reacciones facilitan otras. La organización química convierte una lista de sustancias en una red dinámica.
- **Spoken Macro Summary (EN):** Molecules can form mutually sustained cycles in which the products of some reactions facilitate others. Chemical organization turns a list of substances into a dynamic network.
- **Current Visual & Diagnosis:** `vid = 13` via `hero_doc_matter_mind.wgsl:677-686` (`c_mode=3`) -> Budding dual-sphere lipid protocell (`r=0.32, 0.28`) + equatorial pinch torus. Routed via `11 % 4 = 3` -> `vid=13` (budding lipid protocell). Ironically the 5-node Eigen Autocatalytic Hypercycle (`vid=12`) sits one modulo step away on `Z10`/`Z14`, leaving `Z11` with an empty bubble instead of coupled catalytic reaction cycles.
- **3D Procedural Blueprint:** Route `Z11` to enhanced **Eigen Autocatalytic Hypercycle (`vid=12` upgrade)**: interlocking catalytic reaction rings, complementary SDF lock-and-key molecular docking, ATP energetic spark coupler, inside a semi-translucent boundary shell.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **catalizador** (*catalyst*) | **ES:** Un catalizador acelera una reacción al ofrecer una ruta alternativa sin consumirse de forma neta en el proceso idealizado.<br>**EN:** A catalyst speeds up a reaction by providing an alternative route without being net consumed in the idealized process. | **`[TIER-C]`** | `doc_feat==0`: Dock substrate sphere onto golden curved catalyst cradle lowering reaction flash |
| `F1` | **ciclo de reacción** (*reaction cycle*) | **ES:** En un ciclo, una secuencia de reacciones regenera componentes necesarios para continuar.<br>**EN:** In a cycle, a sequence of reactions regenerates components necessary to continue. | **`[TIER-C]`** | `doc_feat==1`: Sweep luminous closed-loop chemical arrows regenerating all 5 cycle intermediates |
| `F2` | **reconocimiento molecular** (*molecular recognition*) | **ES:** La forma, carga y afinidad permiten que moléculas se asocien selectivamente.<br>**EN:** The shape, charge and affinity allow molecules to associate selectively. | **`[TIER-D]`** | `doc_feat==2`: Snap jagged complementary key-molecule cleanly into lock-pocket SDF cavity |
| `F3` | **portador de energía** (*energy carrier*) | **ES:** Ciertas moléculas acoplan reacciones favorables con procesos que requieren aporte energético.<br>**EN:** Certain molecules couple favorable reactions with processes that require energy input. | **`[TIER-D]`** | `doc_feat==3`: Transfer pulsing amber high-energy phosphate triad (ATP motif) between rings |
| `F4` | **conjunto autocatalítico** (*autocatalytic set*) | **ES:** Una red autocatalítica contiene reacciones cuyos catalizadores son producidos, directa o indirectamente, por la propia red.<br>**EN:** An autocatalytic network contains reactions whose catalysts are produced, directly or indirectly, by the network itself. | **`[TIER-C]`** | `doc_feat==4`: Illuminate mutually cross-linked catalytic arcs closing full RAF set |
| `F5` | **cooperación molecular** (*molecular cooperation*) | **ES:** Múltiples especies químicas pueden sostenerse mutuamente aunque ninguna sea autosuficiente.<br>**EN:** Multiple chemical species can sustain each other even though none are self-sufficient. | **`[TIER-C]`** | `doc_feat==5`: Pulse alternating twin reaction loops feeding each other's bottleneck |
| `F6` | **compartimento** (*compartment*) | **ES:** Una frontera física concentra reactivos, limita pérdidas y modifica qué reacciones pueden sostenerse.<br>**EN:** A physical boundary concentrates reactants, limits losses, and modifies which reactions can be sustained. | **`[TIER-C]`** | `doc_feat==6`: Wrap network inside translucent concentrating capsule shell keeping reactants dense |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP15-Z08-11)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP15-Z08-11` (Odyssey Act IV — Experience 15 Zones `Z08..Z11`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files (4+1 focused slice):
1. `crates/cosmos_render/src/doc_stages.rs` — route `experience_id == 15` `Z08..=Z11` directly to reserved `vid = 2508..=2511` (and populate `color_b = [zone_c.x, zone_c.y, zone_c.z, 1.0]` at `L739`), plus companion emitters ``3761` (optional external orbital monomer cloud for `Z11`)` where specified.
2. `crates/cosmos_narration/src/resolver.rs` — enrich `SegmentVisualHint` / `DOMAIN_CUE_KEYWORDS` (`base_flags` / `cue_tags` / `telemetry`) for the Exp `15` `Z08..=Z11` spoken concepts.
3. `shaders/hero_doc_matter_mind.wgsl` — handle decoded `vid >= 2508 && vid <= 2511` inside `march_doc_bio_mind` with distinct 3D procedural SDF / volumetric structures AND `switch (doc_feat(inst))` sub-stage branching (`0..k`, `0xFF` macro overview) smoothly driven by `doc_cue(inst)` & `doc_weight(inst)` (`0.72..1.35x` HDR cap).
4. `crates/cosmos_render/src/universe_scene.rs` + `shaders/compute_universe.wgsl` — ensure `field_grid_tex` / particle compute mode matches each zone (Compute Mode `12` particle flow + `cs_field_grid` mode `0` (`Z09` Belousov-Zhabotinsky spirals & spontaneous Turing crystal)).

Per-Zone & `doc_feat` Blueprints:
  - `Z08` `networks` (`vid=2508`): **3D Scale-Free Modular Graph Topology**: 24 capsule-linked SDF spheres arranged into 3 densely linked clusters joined by 2 prominent high-degree gold hub nodes, with edge-traveling pulse cascades. Features: `F0` nodo/node, `F1` enlace/link, `F2` centro/hub, `F3` grupo/group, `F4` cascada/cascade, `F5` robustez/robustness, `F6` fragilidad/fragility, `F7` topología/topology.
  - `Z09` `selforganization` (`vid=2509`): Bind `vid=11` to live `field_grid_tex` Turing sphere + orbiting **Boids / Particle Flock Halo** (`mode=6` or `mode=12` neighborhood rules) driven by `doc_feat`. Features: `F0` regla local/local rule, `F1` comportamiento colectivo/collective behavior, `F2` atractor/attractor, `F3` adaptación/adaptation, `F4` automantenimiento/self-maintenance, `F5` coordinación/coordination, `F6` identidad dinámica/dynamic identity.
  - `Z10` `information` (`vid=2510`): **3D Helical Symbol Ribbon + Interpreter Gate**: rotating perforated 4-symbol code ribbon passing through an active reading collar torus that outputs structured pulses vs red-flagged bit errors. Features: `F0` patrón/pattern, `F1` memoria/memory, `F2` señal/signal, `F3` instrucción/instruction, `F4` error/error, `F5` corrección/correction, `F6` función/function, `F7` respuesta ambiental/environmental response.
  - `Z11` `chemistry` (`vid=2511`): Route `Z11` to enhanced **Eigen Autocatalytic Hypercycle (`vid=12` upgrade)**: interlocking catalytic reaction rings, complementary SDF lock-and-key molecular docking, ATP energetic spark coupler, inside a semi-translucent boundary shell. Features: `F0` catalizador/catalyst, `F1` ciclo de reacción/reaction cycle, `F2` reconocimiento molecular/molecular recognition, `F3` portador de energía/energy carrier, `F4` conjunto autocatalítico/autocatalytic set, `F5` cooperación molecular/molecular cooperation, `F6` compartimento/compartment.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; keep volumetric shells tightly bounded inside hero sphere (`r <= 0.96`, early-exit `trans < 0.03`) with zero full-screen fog wash.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps for new documentary hero shells, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

<a id="pkg-act4-exp15-z12-15"></a>
### PKG-ACT4-EXP15-Z12-15: Exp `15` — Molecular Replication, Protocells, Chemical Evolution & Life Threshold (`Z12..Z15`)

- **Phase & Priority:** `Phase 1/2` · **P1 — High (Fixes `Z13 protocells` / `Z14 chemicalevolution` / `Z15 threshold` inverted slots)** · **Scope:** `4` Zones (`Z12..Z15`), `31` Features (`doc_feat` `0..k`)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:55, 731-744`](../../crates/cosmos_render/src/doc_stages.rs#L55) & [`shaders/hero_doc_matter_mind.wgsl:609, 630-686`](../../shaders/hero_doc_matter_mind.wgsl#L630-L686); `vid`: `doc_stage_variant_id(15)` -> `10 + (sc % 4)` (`10, 11, 12, 13`; catalog `scene = null`); `doc_feat(inst)` unused in WGSL; `color_b` zone accent unused (`doc_stages.rs:731-744`). **Card Tiers — Zones:** `0A / 1B / 2C / 1D` · **Features:** `0A / 5B / 18C / 8D`.
- **Compute & Plumbing Focus:** Compute Mode `12` hydrothermal chimney plumes (`Z14` serpentinization alkaline vent & mineral pore sparks) feeding `Z15` transition into Exp `16`. Extra files: `crates/cosmos_render/src/universe_scene.rs` (`L623` accent + `mode 12` hydro-vent modulation).

#### Reserved Target IDs

| Zone | Slug | ES / EN Title | Current `vid` | Reserved Target `vid` | Companion IDs | `submode` & `doc_feat` Range |
|---|---|---|---:|---:|---|---|
| `Z12` | `replication` | Replicación y variación / *Replication and variation* | `10` (`[TIER-C]`) | **`2512`** | `3762` (vesicle lipid membrane outer aura for `Z13`) | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z13` | `protocells` | Compartimentos y protocélulas / *Compartments and protocells* | `11` (`[TIER-C]`) | **`2513`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z14` | `chemicalevolution` | Evolución química / *Chemical evolution* | `12` (`[TIER-D]`) | **`2514`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z15` | `threshold` | El umbral de la vida / *The threshold of life* | `13` (`[TIER-B]`) | **`2515`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..6` |

#### Spoken Transcript Summary & 3D Visual Blueprint

Key deliverables for this Pick-Up Card:
- `vid=2512` (`Z12 replication`): Replace Lorenz butterfly with Semiconservative Zipper Template Helix separating & assembling mirror monomers with tunable mutation glint (`doc_feat 0..7`).
- `vid=2513` (`Z13 protocells`): Move & upgrade translucent lipid bilayer protocell (`vid=13`) from `Z15` onto `Z13`, adding catalytic lumen knots, ion channels & osmotic divide (`doc_feat 0..7`).
- `vid=2514` (`Z14 chemicalevolution`): Sculpt Prebiotic Soup Bowl + UV Spark Arcs + Clay Montmorillonite Adsorption Sheets + Alkaline Hydrothermal Chimney (`doc_feat 0..7`).
- `vid=2515` (`Z15 threshold`): Build Split Bridge Portal linking prebiotic hydrothermal/mineral chemistry cleanly across the fuzzy Darwinian threshold into Exp `16 Z00` (`doc_feat 0..7`).

##### `[TIER-C]` E15 `Z12` (`replication`) -> Target `vid=2512` | Replicación y variación / *Replication and variation* (`0A / 0B / 8C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Copia imperfecta + herencia + persistencia diferencial* / *Imperfect copy + inheritance + differential persistence* | **Evidence (ES / EN):** `Principios evolutivos demostrados` (*Established evolutionary principles*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** Replicar no basta. Para que exista evolución abierta deben aparecer diferencias heredables y algunas variantes deben persistir o reproducirse más que otras en un entorno determinado.
- **Spoken Macro Summary (EN):** Replication alone is not enough. For open-ended evolution, heritable differences must arise, and some variants must persist or reproduce more than others in a given environment.
- **Current Visual & Diagnosis:** `vid = 10` via `hero_doc_matter_mind.wgsl:633-654` (`c_mode=0`) -> Lorenz butterfly strange attractor (2 tilted 3-ring scrolls + 2 saddle capsules + core). Routed via `12 % 4 = 0` -> `vid=10` (Lorenz butterfly wings). Chaos wings fail to represent template replication, heritable lineages, point mutations, resource competition, natural selection, or genetic drift.
- **3D Procedural Blueprint:** **Dual-Strand Template Zipper + Branching Variant Lineage Fan**: replicating biopolymer zipper spawning colored variant descendants across a 3D rugged fitness landscape ridge.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **copia** (*copy*) | **ES:** Copiar conserva una organización o secuencia a través de nuevas instancias.<br>**EN:** Copy preserves an organization or sequence across new instances. | **`[TIER-C]`** | `doc_feat==0`: Split parent polymer ribbon into two identical daughter strands |
| `F1` | **herencia** (*inheritance*) | **ES:** La herencia transmite características entre generaciones o ciclos de reproducción.<br>**EN:** Heredity transmits characteristics between generations or reproduction cycles. | **`[TIER-C]`** | `doc_feat==1`: Trace glowing generational lineage arcs carrying color trait downward |
| `F2` | **mutación** (*mutation*) | **ES:** Una mutación es un cambio heredable en la información copiada.<br>**EN:** A mutation is a heritable change in copied information. | **`[TIER-C]`** | `doc_feat==2`: Swap one rung glyph cleanly into bright cyan/magenta mutant variant |
| `F3` | **replicación imperfecta** (*imperfect replication*) | **ES:** La copia con errores combina continuidad con aparición de variantes.<br>**EN:** Error-prone copying combines continuity with the appearance of variants. | **`[TIER-C]`** | `doc_feat==3`: Introduce stochastic copy jitter spawning 3 distinct variant lineages |
| `F4` | **competencia** (*competition*) | **ES:** Las variantes pueden depender de recursos limitados o interferir entre sí.<br>**EN:** Variants may depend on limited resources or interfere with each other. | **`[TIER-C]`** | `doc_feat==4`: Fade starved slow-replicating strands competing around single nutrient source |
| `F5` | **selección** (*selection*) | **ES:** La selección ocurre cuando diferencias heredables causan diferencias sistemáticas de supervivencia o reproducción.<br>**EN:** Selection occurs when heritable differences cause systematic differences in survival or reproduction. | **`[TIER-C]`** | `doc_feat==5`: Amplify high-fitness golden peak lineage across successive generations |
| `F6` | **adaptación** (*adaptation*) | **ES:** Una adaptación es una característica acumulada porque mejoró la persistencia o reproducción en contextos pasados.<br>**EN:** An adaptation is a characteristic accumulated because it improved persistence or reproduction in past contexts. | **`[TIER-C]`** | `doc_feat==6`: Mold structural spine crest dynamically toward peak fitness surface |
| `F7` | **deriva** (*genetic drift*) | **ES:** El azar también puede cambiar la frecuencia de variantes, especialmente en poblaciones pequeñas.<br>**EN:** Chance can also change the frequency of variants, especially in small populations. | **`[TIER-C]`** | `doc_feat==7`: Randomly extinguish half of thin sparse population without fitness bias |

##### `[TIER-C]` E15 `Z13` (`protocells`) -> Target `vid=2513` | Compartimentos y protocélulas / *Compartments and protocells* (`0A / 0B / 8C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Membrana + interior + transporte + crecimiento + división* / *Membrane + interior + transport + growth + division* | **Evidence (ES / EN):** `Modelos experimentales, no antepasados confirmados` (*Experimental models, not confirmed ancestors*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** Vesículas y protocélulas experimentales muestran cómo una membrana puede concentrar química, crecer y dividirse. Son modelos de posibilidades, no reconstrucciones confirmadas de la primera célula.
- **Spoken Macro Summary (EN):** Experimental vesicles and protocells show how a membrane can concentrate chemicals, grow, and divide. They are models of possibilities, not confirmed reconstructions of the first cell.
- **Current Visual & Diagnosis:** `vid = 11` via `hero_doc_matter_mind.wgsl:655-663` (`c_mode=1`) -> Sinusoidal Turing coral sphere + bottom plate (`field_grid_tex` NOT sampled). Routed via `13 % 4 = 1` -> `vid=11` (Turing coral sphere). Classic modulo inversion bug: `vid=13` (literally the budding lipid protocell SDF!) exists in `hero_doc_matter_mind.wgsl:677-686` yet was skipped here in favor of Turing coral!
- **3D Procedural Blueprint:** Reassign `Z13` to **Upgraded Bilayer Budding Protocell (`vid=13+`)**: cut-away lipid bilayer shell showing hydrophobic core, embedded membrane transport pore channels, engulfed food molecules, osmotic volume swelling, and contractile fission neck.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **membrana** (*membrane*) | **ES:** Una membrana crea una frontera semipermeable entre interior y exterior.<br>**EN:** A membrane creates a semipermeable boundary between interior and exterior. | **`[TIER-C]`** | `doc_feat==0`: Slice quarter-cutaway revealing double-layer amphipathic shell boundary |
| `F1` | **interior** (*interior*) | **ES:** El interior puede mantener concentraciones y reacciones diferentes de las del entorno.<br>**EN:** The interior can maintain concentrations and reactions different from those of the environment. | **`[TIER-C]`** | `doc_feat==1`: Illuminate concentrated golden internal reaction cluster inside lumen |
| `F2` | **exterior** (*outside*) | **ES:** El exterior suministra materia y energía, pero también impone cambios y perturbaciones.<br>**EN:** The exterior supplies matter and energy, but also imposes changes and disturbances. | **`[TIER-C]`** | `doc_feat==2`: Stream turbulent external nutrient particles battering outer lipid shell |
| `F3` | **transporte selectivo** (*selective transport*) | **ES:** Una frontera selectiva deja pasar algunas sustancias más fácilmente que otras.<br>**EN:** A selective border allows some substances to pass more easily than others. | **`[TIER-C]`** | `doc_feat==3`: Open glowing barrel transmembrane pore channels selectively passing green ions |
| `F4` | **alimento** (*food*) | **ES:** Un recurso químico puede aportar materia o energía a procesos internos.<br>**EN:** A chemical resource can provide matter or energy to internal processes. | **`[TIER-C]`** | `doc_feat==4`: Pull luminous nutrient pearls inward through active membrane pores |
| `F5` | **crecimiento** (*growth*) | **ES:** Una protocélula puede incorporar componentes y aumentar su membrana o contenido.<br>**EN:** A protocell can incorporate components and increase its membrane or content. | **`[TIER-C]`** | `doc_feat==5`: Inflate protocell radius smoothly as internal oligomers accumulate |
| `F6` | **división** (*division*) | **ES:** La deformación y separación de un compartimento puede distribuir su contenido entre descendientes.<br>**EN:** The deformation and separation of a compartment can distribute its contents among descendants. | **`[TIER-C]`** | `doc_feat==6`: Constrict equatorial cleavage torus pinching mother vesicle into two daughters |
| `F7` | **acoplamiento** (*coupling*) | **ES:** Para evolucionar como unidad, la membrana, el metabolismo y la información deben beneficiarse mutuamente.<br>**EN:** To evolve as a unit, the membrane, metabolism and information must benefit each other. | **`[TIER-C]`** | `doc_feat==7`: Link internal genetic loop tether simultaneously to growing membrane & metabolic core |

##### `[TIER-D]` E15 `Z14` (`chemicalevolution`) -> Target `vid=2514` | Evolución química / *Chemical evolution* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Química observada → historia posible → origen desconocido* / *Observed chemistry → possible history → unknown origin* | **Evidence (ES / EN):** `Múltiples rutas activas de investigación` (*Multiple active research paths*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** Existen varias familias de hipótesis compatibles en parte y competidoras en parte. La evidencia identifica procesos posibles, pero todavía no determina la secuencia histórica real.
- **Spoken Macro Summary (EN):** There are several families of hypotheses that are partly compatible and partly competing. The evidence identifies possible processes, but does not yet determine the actual historical sequence.
- **Current Visual & Diagnosis:** `vid = 12` via `hero_doc_matter_mind.wgsl:664-676` (`c_mode=2`) -> 5-node Eigen Hypercycle HSL spheres + connecting capsule links. Routed via `14 % 4 = 2` -> `vid=12` (5 abstract spheres). Narration describes vivid concrete prebiotic scenarios—RNA ribozymes, Lost City alkaline hydrothermal vents, montmorillonite/pyrite mineral surfaces, wet-dry tidal pools, carbonaceous meteorites—none rendered anywhere.
- **3D Procedural Blueprint:** **Split Prebiotic Geochemical Diorama Stage**: porous serpentine hydrothermal chimney spewing warm proton-gradient plumes beside layered clay/mineral template sheets, folded glowing RNA hairpins, evaporating tidal rim, and streaking chondrite meteor.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **mundo de ARN** (*RNA world*) | **ES:** Las hipótesis de mundo de ARN estudian moléculas capaces de almacenar información y catalizar reacciones antes del sistema moderno de ADN y proteínas.<br>**EN:** RNA world hypotheses study molecules capable of storing information and catalyzing reactions before the modern system of DNA and proteins. | **`[TIER-D]`** | `doc_feat==0`: Fold catalytic hammerhead-like RNA hairpin loop storing & cleaving twin strands |
| `F1` | **metabolismo primero** (*metabolism-first*) | **ES:** Los modelos de metabolismo primero priorizan redes químicas autosostenidas antes de replicadores informacionales complejos.<br>**EN:** Metabolism-first models prioritize self-sustaining chemical networks that precede complex informational replicators. | **`[TIER-D]`** | `doc_feat==1`: Circulate interconnected central non-genetic Krebs-like reaction wheel |
| `F2` | **mundo lipídico** (*lipid world*) | **ES:** Los modelos lipídicos investigan conjuntos de moléculas anfipáticas capaces de formar compartimentos, crecer y transmitir composiciones.<br>**EN:** Lipid models investigate sets of amphipathic molecules capable of forming compartments, growing and transmitting compositions. | **`[TIER-D]`** | `doc_feat==2`: Self-assemble amphiphilic micelle capsules budded from oily film droplets |
| `F3` | **fuentes hidrotermales** (*hydrothermal vents*) | **ES:** Gradientes químicos y minerales de entornos hidrotermales podrían haber impulsado reacciones y compartimentación natural.<br>**EN:** Chemical and mineral gradients from hydrothermal environments could have driven natural reactions and compartmentalization. | **`[TIER-D]`** | `doc_feat==3`: Render porous Lost City hydrothermal chimney erupting alkaline pH-gradient plumes |
| `F4` | **superficies minerales** (*mineral surfaces*) | **ES:** Minerales pueden concentrar moléculas, orientar reacciones y ofrecer catálisis o plantillas físicas.<br>**EN:** Minerals can concentrate molecules, guide reactions, and provide catalysis or physical templates. | **`[TIER-D]`** | `doc_feat==4`: Snap flat prebiotic monomers onto ordered hexagonal clay/pyrite crystal terrace |
| `F5` | **ciclos húmedo-seco** (*wet-dry cycles*) | **ES:** Ciclos de evaporación y rehidratación pueden concentrar reactivos y favorecer la formación de enlaces.<br>**EN:** Cycles of evaporation and rehydration can concentrate reagents and promote bond formation. | **`[TIER-D]`** | `doc_feat==5`: Oscillate shallow pool water-level plane concentrating & polymerizing solute layer |
| `F6` | **aporte extraterrestre** (*extraterrestrial contribution*) | **ES:** Meteoritos y polvo cósmico entregan compuestos orgánicos, aunque eso no explica por sí solo su organización en vida.<br>**EN:** Meteorites and cosmic dust deliver organic compounds, although this alone does not explain how they became organized into life. | **`[TIER-D]`** | `doc_feat==6`: Streak glowing carbonaceous meteorite delivering glowing organic droplets from sky |
| `F7` | **certeza** (*certainty*) | **ES:** Se observan química orgánica y autoorganización; se conocen componentes prebióticos; se fabrican protocélulas. La ruta histórica concreta sigue abierta.<br>**EN:** Organic chemistry and self-organization are observed; prebiotic components are known; protocells are made. The specific historical route remains open. | **`[TIER-D]`** | `doc_feat==7`: Split horizon prism into multiple faintly shimmering parallel candidate paths |

##### `[TIER-B]` E15 `Z15` (`threshold`) -> Target `vid=2515` | El umbral de la vida / *The threshold of life* (`0A / 5B / 2C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Automantenimiento + herencia + variación + evolución* / *Self-maintenance + inheritance + variation + evolution* | **Evidence (ES / EN):** `Frontera científica aún abierta` (*An open scientific frontier*) | `speak_evidence_default = False`
- **Spoken Macro Summary (ES):** La física permite la química, y la química puede generar redes que se organizan, conservan información y se reproducen. Aún no sabemos qué secuencia concreta transformó materia no viva en los primeros sistemas biológicos.
- **Spoken Macro Summary (EN):** Physics enables chemistry, and chemistry can generate networks that organize, conserve information, and reproduce. We still do not know what specific sequence transformed non-living matter into the first biological systems.
- **Current Visual & Diagnosis:** `vid = 13` via `hero_doc_matter_mind.wgsl:677-686` (`c_mode=3`) -> Budding dual-sphere lipid protocell (`r=0.32, 0.28`) + equatorial pinch torus. Routed via `15 % 4 = 3` -> `vid=13` (budding lipid protocell). Semipermeable dividing vesicle is an accurate thematic bridge from prebiotic chemistry to cellular life (`[TIER-B]`), though `doc_feat` is ignored and hereditary polymer + adaptive lineage cues are missing.
- **3D Procedural Blueprint:** Enhance `vid=13` dividing protocell with an internal glowing circular replicator + metabolic cycle core and an outward **Emergent Lineage Portal** toward Exp 16.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **umbral** (*threshold*) | **ES:** No existe una línea universalmente simple entre química compleja y vida; varios criterios pueden aparecer gradualmente y en distinto orden.<br>**EN:** There is no universally simple line between complex chemistry and life; several criteria can appear gradually and in different order. | **`[TIER-B]`** | `doc_feat==0`: Show fuzzy dashed gradient bridge spanning chemical network to enclosed replicator |
| `F1` | **automantenimiento** (*self-maintenance*) | **ES:** Un sistema vivo conserva activamente condiciones internas mediante flujos de materia y energía.<br>**EN:** A living system actively maintains internal conditions through flows of matter and energy. | **`[TIER-B]`** | `doc_feat==1`: Pulse steady internal ATP-like metabolic fountain maintaining membrane integrity |
| `F2` | **herencia** (*inheritance*) | **ES:** La continuidad evolutiva requiere que parte de la organización pase a nuevas generaciones.<br>**EN:** Evolutionary continuity requires that part of the organization be passed on to new generations. | **`[TIER-B]`** | `doc_feat==2`: Replicate internal genetic ring cleanly across dividing constriction neck |
| `F3` | **evolución** (*evolution*) | **ES:** La selección acumulativa puede convertir variación heredable en adaptación histórica.<br>**EN:** Cumulative selection can convert heritable variation into historical adaptation. | **`[TIER-C]`** | `doc_feat==3`: Fan subtle multi-branching phylogenetic sprout behind budding daughter cells |
| `F4` | **primera vida** (*first life*) | **ES:** No conocemos la identidad, el lugar ni la secuencia exacta de los primeros sistemas biológicos de la Tierra.<br>**EN:** We do not know the exact identity, location, or sequence of Earth's first biological systems. | **`[TIER-B]`** | `doc_feat==4`: Fade misty uncertain origin haze around earliest primordial micro-vesicle |
| `F5` | **nueva historia** (*new story*) | **ES:** Más allá, cuando la química comenzó a heredarse, adaptarse y evolucionar, apareció una nueva historia: la vida.<br>**EN:** Beyond that, when chemistry began to be inherited, adapted and evolved, a new story appeared: life. | **`[TIER-B]`** | `doc_feat==5`: Ignite bioluminescent emerald core transition signaling life's new history |
| `F6` | **siguiente viaje** (*next trip*) | **ES:** Espectro de la Vida: de la Primera Célula a la Conciencia.<br>**EN:** Spectrum of Life: From the First Cell to Consciousness. | **`[TIER-C]`** | `doc_feat==6`: Project emerald portal ring forward toward `16_life` BIOSPERE sector |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP15-Z12-15)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP15-Z12-15` (Odyssey Act IV — Experience 15 Zones `Z12..Z15`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files (4+1 focused slice):
1. `crates/cosmos_render/src/doc_stages.rs` — route `experience_id == 15` `Z12..=Z15` directly to reserved `vid = 2512..=2515` (and populate `color_b = [zone_c.x, zone_c.y, zone_c.z, 1.0]` at `L739`), plus companion emitters ``3762` (vesicle lipid membrane outer aura for `Z13`)` where specified.
2. `crates/cosmos_narration/src/resolver.rs` — enrich `SegmentVisualHint` / `DOMAIN_CUE_KEYWORDS` (`base_flags` / `cue_tags` / `telemetry`) for the Exp `15` `Z12..=Z15` spoken concepts.
3. `shaders/hero_doc_matter_mind.wgsl` — handle decoded `vid >= 2512 && vid <= 2515` inside `march_doc_bio_mind` with distinct 3D procedural SDF / volumetric structures AND `switch (doc_feat(inst))` sub-stage branching (`0..k`, `0xFF` macro overview) smoothly driven by `doc_cue(inst)` & `doc_weight(inst)` (`0.72..1.35x` HDR cap).
4. `crates/cosmos_render/src/universe_scene.rs` + `shaders/compute_universe.wgsl` — ensure `field_grid_tex` / particle compute mode matches each zone (Compute Mode `12` hydrothermal chimney plumes (`Z14` serpentinization alkaline vent & mineral pore sparks) feeding `Z15` transition into Exp `16`).

Per-Zone & `doc_feat` Blueprints:
  - `Z12` `replication` (`vid=2512`): **Dual-Strand Template Zipper + Branching Variant Lineage Fan**: replicating biopolymer zipper spawning colored variant descendants across a 3D rugged fitness landscape ridge. Features: `F0` copia/copy, `F1` herencia/inheritance, `F2` mutación/mutation, `F3` replicación imperfecta/imperfect replication, `F4` competencia/competition, `F5` selección/selection, `F6` adaptación/adaptation, `F7` deriva/genetic drift.
  - `Z13` `protocells` (`vid=2513`): Reassign `Z13` to **Upgraded Bilayer Budding Protocell (`vid=13+`)**: cut-away lipid bilayer shell showing hydrophobic core, embedded membrane transport pore channels, engulfed food molecules, osmotic volume swelling, and contractile fission neck. Features: `F0` membrana/membrane, `F1` interior/interior, `F2` exterior/outside, `F3` transporte selectivo/selective transport, `F4` alimento/food, `F5` crecimiento/growth, `F6` división/division, `F7` acoplamiento/coupling.
  - `Z14` `chemicalevolution` (`vid=2514`): **Split Prebiotic Geochemical Diorama Stage**: porous serpentine hydrothermal chimney spewing warm proton-gradient plumes beside layered clay/mineral template sheets, folded glowing RNA hairpins, evaporating tidal rim, and streaking chondrite meteor. Features: `F0` mundo de ARN/RNA world, `F1` metabolismo primero/metabolism-first, `F2` mundo lipídico/lipid world, `F3` fuentes hidrotermales/hydrothermal vents, `F4` superficies minerales/mineral surfaces, `F5` ciclos húmedo-seco/wet-dry cycles, `F6` aporte extraterrestre/extraterrestrial contribution, `F7` certeza/certainty.
  - `Z15` `threshold` (`vid=2515`): Enhance `vid=13` dividing protocell with an internal glowing circular replicator + metabolic cycle core and an outward **Emergent Lineage Portal** toward Exp 16. Features: `F0` umbral/threshold, `F1` automantenimiento/self-maintenance, `F2` herencia/inheritance, `F3` evolución/evolution, `F4` primera vida/first life, `F5` nueva historia/new story, `F6` siguiente viaje/next trip.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; keep volumetric shells tightly bounded inside hero sphere (`r <= 0.96`, early-exit `trans < 0.03`) with zero full-screen fog wash.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps for new documentary hero shells, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

---

## 4.2 Experience 16 (`16_life`) — *Espectro de la Vida* (`19` Zones, `153` Features)

<a id="pkg-act4-exp16-z00-04"></a>
### PKG-ACT4-EXP16-Z00-04: Exp `16` — Origin Threshold, First Prokaryotic Life, Cellular Compartment, Metabolism & Heredity (`Z00..Z04`)

- **Phase & Priority:** `Phase 1/2` · **P1 — High (Fixes `Z00` premature Tree of Life, `Z01..03` organelle swaps & adds LUCA/DNA cues)** · **Scope:** `5` Zones (`Z00..Z04`), `40` Features (`doc_feat` `0..k`)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:56-60, 731-744`](../../crates/cosmos_render/src/doc_stages.rs#L56-L60) & [`shaders/hero_doc_matter_mind.wgsl:610, 687-759`](../../shaders/hero_doc_matter_mind.wgsl#L687-L759); `vid`: `doc_stage_variant_id(16)` -> `0->23, 4->20, _->20+(sc%5)` (`23, 21, 22, 23, 20`; catalog `scene = null`); `doc_feat(inst)` unused in WGSL; `color_b` zone accent unused (`doc_stages.rs:731-744`). **Card Tiers — Zones:** `0A / 1B / 4C / 0D` · **Features:** `0A / 5B / 28C / 7D`.
- **Compute & Plumbing Focus:** Compute Mode `6` (`compute_universe.wgsl:223-237` BZ/cellular reaction plumes; update misleading `(15)` comment). Extra files: `crates/cosmos_render/src/doc_stages.rs` (`L56-L60` remove `0 | u16::MAX => 23` override), `shaders/compute_universe.wgsl` (`L223-L237`).

#### Reserved Target IDs

| Zone | Slug | ES / EN Title | Current `vid` | Reserved Target `vid` | Companion IDs | `submode` & `doc_feat` Range |
|---|---|---|---:|---:|---|---|
| `Z00` | `threshold` | El umbral de la vida / *The threshold of life* | `23` (`[TIER-C]`) | **`2600`** | `3770` (`Z04` lagging-strand polymerase / ribonucleoprotein RNA companion) | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z01` | `firstlife` | Primeros sistemas vivos / *First living systems* | `21` (`[TIER-C]`) | **`2601`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z02` | `cells` | Células primitivas / *primitive cells* | `22` (`[TIER-C]`) | **`2602`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z03` | `metabolism` | Metabolismo / *Metabolism* | `23` (`[TIER-C]`) | **`2603`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z04` | `heredity` | Información heredable / *heritable information* | `20` (`[TIER-B]`) | **`2604`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |

#### Spoken Transcript Summary & 3D Visual Blueprint

Key deliverables for this Pick-Up Card:
- `vid=2600` (`Z00 threshold`): Replace `vid=23` Phylogenetic Tree with Pre-LUCA Protocell Threshold Triad (semipermeable vesicle, autocatalytic loop, gradient halo, open-question mist; `doc_feat 0..7`).
- `vid=2601` (`Z01 firstlife`): Replace Chloroplast with Prokaryotic Archaea/Bacteria Cell Cylinder (cell wall, nucleoid loop, ribosomes, rotary flagellum, LUCA convergence halo; `doc_feat 0..7`).
- `vid=2602` (`Z02 cells`): Replace Mitochondrion-only oval with Complete Cutaway Cell Architecture (lipid bilayer gates, ribosome assembly, enzymatic hub, cytosol flow; `doc_feat 0..7`).
- `vid=2603` (`Z03 metabolism`): Replace Tree of Life with Coupled ATP-Synthase Turbine & Redox Proton-Pump Membrane (`doc_feat 0..7`).
- `vid=2604` (`Z04 heredity`): Upgrade `vid=20` B-DNA Double Helix inside cell (`doc_feat 0..7`: single-strand RNA fold, chromatin wrap, semiconservative fork, epigenetic methyl tags).

##### `[TIER-C]` E16 `Z00` (`threshold`) -> Target `vid=2600` | El umbral de la vida / *The threshold of life* (`0A / 0B / 8C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Compartimento + energía + copia + variación* / *Compartment + energy + copy + variation* | **Evidence (ES / EN):** `Frontera histórica aún desconocida` (*Historical frontier still unknown*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** No conocemos el instante exacto ni la ruta concreta por la que comenzó la vida. Los primeros sistemas vivos pudieron surgir mediante transiciones graduales entre química organizada y evolución biológica.
- **Spoken Macro Summary (EN):** We do not know the exact moment or the specific route by which life began. The first living systems could have emerged through gradual transitions between organized chemistry and biological evolution.
- **Current Visual & Diagnosis:** `vid = 23` via `hero_doc_matter_mind.wgsl:727-750` (`l_mode=3`) -> 3D Phylogenetic Tree of Life (LUCA root -> 3 domain branches + canopy ring). Explicit bespoke override `0 | u16::MAX => 23` (`doc_stages.rs:57`) draws the modern 3-domain Phylogenetic Tree of Life (`l_mode=3`). That clashes with the pre-LUCA threshold narration (`compartment + reaction network + energy gradient + replication`).
- **3D Procedural Blueprint:** Move Tree of Life (`23`) to `Z05` and give `Z00` a **Pre-LUCA Protocell Threshold Triad**: semipermeable vesicle enclosing coupled reaction network loops, steep membrane gradient halo, and imperfect template strand.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **compartimento** (*compartment*) | **ES:** Una frontera concentra reacciones y permite que un interior mantenga condiciones distintas del entorno.<br>**EN:** A boundary concentrates reactions and allows the interior to maintain conditions different from the environment. | **`[TIER-C]`** | `doc_feat==0`: Cutaway permeable vesicle membrane retaining glowing internal solutes |
| `F1` | **red de reacciones** (*reaction network*) | **ES:** Reacciones acopladas pueden intercambiar materia y energía, producir componentes y sostener ciclos químicos.<br>**EN:** Coupled reactions can exchange matter and energy, produce components, and sustain chemical cycles. | **`[TIER-C]`** | `doc_feat==1`: Pulse multi-node autocatalytic loop humming inside lumen |
| `F2` | **gradiente** (*gradient*) | **ES:** Una diferencia de concentración, temperatura o potencial puede impulsar procesos lejos del equilibrio.<br>**EN:** A difference in concentration, temperature, or potential can drive processes away from equilibrium. | **`[TIER-C]`** | `doc_feat==2`: Render high-contrast proton/ion charge gradient sheen across lipid wall |
| `F3` | **replicación** (*replication*) | **ES:** La copia conserva parte de una organización, pero por sí sola no garantiza vida ni evolución abierta.<br>**EN:** Copying preserves part of an organization, but by itself does not guarantee life or open-ended evolution. | **`[TIER-C]`** | `doc_feat==3`: Zip template strands apart without yet spawning divergent daughters |
| `F4` | **variación** (*variation*) | **ES:** Las diferencias heredables permiten que unas variantes persistan o se reproduzcan de manera distinta.<br>**EN:** Inheritable differences allow some variants to persist or reproduce differently. | **`[TIER-C]`** | `doc_feat==4`: Tint newly copied strands with subtle spectral color variations |
| `F5` | **cooperación molecular** (*molecular cooperation*) | **ES:** Moléculas diferentes pueden sostener funciones complementarias dentro del mismo sistema.<br>**EN:** Different molecules can support complementary functions within the same system. | **`[TIER-C]`** | `doc_feat==5`: Show complementary protein/ribozyme partners docking around shared hub |
| `F6` | **herencia imperfecta** (*imperfect inheritance*) | **ES:** La continuidad con errores combina memoria química y novedad.<br>**EN:** Continuity with errors combines chemical memory and novelty. | **`[TIER-C]`** | `doc_feat==6`: Highlight single mismatched glowing base rung preserved across copy step |
| `F7` | **pregunta abierta** (*open question*) | **ES:** No sabemos qué combinación apareció primero ni cuál fue la secuencia histórica real.<br>**EN:** We do not know which combination appeared first or what the actual historical sequence was. | **`[TIER-C]`** | `doc_feat==7`: Fade wireframe question-mark mist over early pre-tree ancestral origins |

##### `[TIER-C]` E16 `Z01` (`firstlife`) -> Target `vid=2601` | Primeros sistemas vivos / *First living systems* (`0A / 0B / 8C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Unidades capaces de mantenerse, heredarse y evolucionar* / *Units capable of maintaining, inheriting and evolving* | **Evidence (ES / EN):** `Inferencia profunda a partir de la vida moderna` (*Deep inference from modern life*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La primera vida no tuvo por qué parecerse a una célula moderna. El requisito decisivo fue formar poblaciones con continuidad, variación heredable y evolución acumulativa.
- **Spoken Macro Summary (EN):** The first life did not have to look like a modern cell. The decisive requirement was to form populations with continuity, heritable variation and cumulative evolution.
- **Current Visual & Diagnosis:** `vid = 21` via `hero_doc_matter_mind.wgsl:712-718` (`l_mode=1`) -> Chloroplast thylakoid disc stacks + spinning 6-blade ATP synthase turbine. Routed via `1 % 5 = 1` -> `vid=21` (eukaryotic Chloroplast!). Renders oxygenic photosynthesis thylakoids—an evolutionary innovation over a billion years *younger* than the first living systems.
- **3D Procedural Blueprint:** **Swarming Early Replicator Population & Ghost Lineages**: cluster of simple capsule protocells dividing in variable micro-environments, with fading translucent extinct lineages alongside one unbroken golden ancestral thread leading to LUCA.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **unidad** (*unit*) | **ES:** Para que la selección actúe con continuidad, ciertos procesos deben quedar asociados en unidades que producen descendencia.<br>**EN:** For selection to act continuously, certain processes must be associated in units that produce offspring. | **`[TIER-C]`** | `doc_feat==0`: Isolate glowing mother-daughter protocell pair as indivisible selection unit |
| `F1` | **mantenimiento** (*maintenance*) | **ES:** Los sistemas vivos reparan daños, reemplazan componentes y regulan condiciones internas.<br>**EN:** Living systems repair damage, replace components, and regulate internal conditions. | **`[TIER-C]`** | `doc_feat==1`: Show membrane patch repair sparkles mending stochastic rupture holes |
| `F2` | **descendencia** (*offspring*) | **ES:** Una generación produce nuevas unidades relacionadas con ella, pero no necesariamente idénticas.<br>**EN:** A generation produces new units related to it, but not necessarily identical. | **`[TIER-C]`** | `doc_feat==2`: Bud varied tinted daughter capsules from central parent capsule |
| `F3` | **población** (*population*) | **ES:** La evolución ocurre en poblaciones de variantes, no en una única entidad aislada.<br>**EN:** Evolution occurs in populations of variants, not in a single isolated entity. | **`[TIER-C]`** | `doc_feat==3`: Spread swarm of 18 distinct capsule variants across stage volume |
| `F4` | **entorno** (*environment*) | **ES:** La disponibilidad de energía, materiales y refugios determina qué formas de organización pueden persistir.<br>**EN:** The availability of energy, materials and shelters determines what forms of organization can persist. | **`[TIER-C]`** | `doc_feat==4`: Overlay mineral shelter & chemical gradient bands modulating swarm survival |
| `F5` | **continuidad** (*continuity*) | **ES:** La vida actual desciende de una historia ininterrumpida de sistemas que lograron reproducirse.<br>**EN:** Current life descends from an uninterrupted history of systems that managed to reproduce. | **`[TIER-C]`** | `doc_feat==5`: Trace unbroken bright golden ancestral spine running forward through swarm |
| `F6` | **ancestro común** (*common ancestor*) | **ES:** El ancestro común de la vida moderna no tiene por qué haber sido el primer sistema vivo.<br>**EN:** The common ancestor of modern life need not have been the first living system. | **`[TIER-C]`** | `doc_feat==6`: Highlight confluence trunk node labeled symbolically as LUCA (non-first) |
| `F7` | **extinción temprana** (*early extinction*) | **ES:** Pudieron existir muchas formas iniciales sin descendientes actuales.<br>**EN:** Many initial forms could have existed without current descendants. | **`[TIER-C]`** | `doc_feat==7`: Dissolve peripheral dim dead-end branches into fading ghost particles |

##### `[TIER-C]` E16 `Z02` (`cells`) -> Target `vid=2602` | Células primitivas / *primitive cells* (`0A / 0B / 6C / 2D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Membrana · interior · energía · reparación · división* / *membrane · interior · energy · repair · division* | **Evidence (ES / EN):** `Biología celular reconstruida con incertidumbre` (*Cell biology reconstructed with uncertainty*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La célula integra frontera, química interna, transporte, mantenimiento, reproducción y herencia. Las primeras células debieron ser mucho más simples que las actuales.
- **Spoken Macro Summary (EN):** The cell integrates border, internal chemistry, transport, maintenance, reproduction and inheritance. The first cells must have been much simpler than today's cells.
- **Current Visual & Diagnosis:** `vid = 22` via `hero_doc_matter_mind.wgsl:719-726` (`l_mode=2`) -> Mitochondrion ellipsoid + sinusoidal inner cristae foldings. Routed via `2 % 5 = 2` -> `vid=22` (eukaryotic Mitochondrion!). Primitive prokaryotic cells had no mitochondria; showing folded mitochondrial cristae misrepresents early cell architecture.
- **3D Procedural Blueprint:** **Prokaryotic Primitive Cell Cross-Section**: simple peptidoglycan/lipid cell capsule with nucleotide-rich nucleoid loop, membrane-embedded ion channels, food intake vesicles, and central FtsZ contractile division ring.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **célula** (*cell*) | **ES:** Una célula es una unidad organizada capaz de intercambiar materia y energía mientras mantiene un interior regulado.<br>**EN:** A cell is an organized unit capable of exchanging matter and energy while maintaining a regulated interior. | **`[TIER-C]`** | `doc_feat==0`: Render cutaway primitive rod/coccus cell silhouette in fluid bath |
| `F1` | **membrana** (*membrane*) | **ES:** La membrana separa sin aislar: controla intercambios y ayuda a conservar gradientes.<br>**EN:** The membrane separates without isolating: it controls exchanges and helps preserve gradients. | **`[TIER-D]`** | `doc_feat==1`: Embed rotating barrel transporter protein channels in selectively permeable bilayer |
| `F2` | **interior** (*interior*) | **ES:** El citoplasma reúne agua, iones, moléculas y reacciones en un espacio compartido.<br>**EN:** The cytoplasm brings together water, ions, molecules and reactions in a shared space. | **`[TIER-C]`** | `doc_feat==2`: Populate crowded cytoplasmic interior with turbulent diffusing macromolecules |
| `F3` | **energía** (*energy*) | **ES:** La célula transforma fuentes externas en energía química utilizable.<br>**EN:** The cell transforms external sources into usable chemical energy. | **`[TIER-C]`** | `doc_feat==3`: Arc energetic membrane proton-motive sparks driving interior chemistry |
| `F4` | **alimento** (*food*) | **ES:** Los nutrientes aportan átomos y energía para construir, reparar y reproducir componentes.<br>**EN:** Nutrients provide atoms and energy to build, repair and reproduce components. | **`[TIER-C]`** | `doc_feat==4`: Ingest external nutrient orbs across membrane channels into core |
| `F5` | **reparación** (*repair*) | **ES:** Los procesos de reparación reducen la acumulación de daños, aunque nunca eliminan todos los errores.<br>**EN:** Repair processes reduce the accumulation of damage, although they never eliminate all errors. | **`[TIER-C]`** | `doc_feat==5`: Sweep repair clamp along damaged DNA/membrane segments sealing gaps |
| `F6` | **división** (*division*) | **ES:** La división reparte membrana, química e información entre células descendientes.<br>**EN:** Division distributes membrane, chemistry and information between descendant cells. | **`[TIER-D]`** | `doc_feat==6`: Constrict equatorial FtsZ septum ring splitting prokaryotic cell in two |
| `F7` | **entorno** (*environment*) | **ES:** Sensores y transportadores conectan la química interna con cambios externos.<br>**EN:** Sensors and transporters connect internal chemistry with external changes. | **`[TIER-C]`** | `doc_feat==7`: Activate transmembrane external chemosensor antennae flickering on surface |

##### `[TIER-C]` E16 `Z03` (`metabolism`) -> Target `vid=2603` | Metabolismo / *Metabolism* (`0A / 0B / 5C / 3D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Materia entra → reacciones acopladas → trabajo y desechos* / *Matter enters → coupled reactions → work and waste* | **Evidence (ES / EN):** `Proceso universal de la vida celular` (*Universal process of cellular life*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Vivir requiere mantener organización interna lejos del equilibrio. El metabolismo enlaza reacciones, captura energía, fabrica componentes y elimina productos que ya no sirven.
- **Spoken Macro Summary (EN):** Living requires maintaining internal organization far from equilibrium. Metabolism links reactions, captures energy, manufactures components, and eliminates products that are no longer useful.
- **Current Visual & Diagnosis:** `vid = 23` via `hero_doc_matter_mind.wgsl:727-750` (`l_mode=3`) -> 3D Phylogenetic Tree of Life (LUCA root -> 3 domain branches + canopy ring). Routed via `3 % 5 = 3` -> `vid=23` (Phylogenetic Tree of Life!). Showing an evolutionary cladogram for cellular biochemistry leaves students with zero visual sense of enzymes, linked pathways, ATP energy coupling, or membrane ion gradients.
- **3D Procedural Blueprint:** **Cellular Metabolic Reactor**: cross-sectioned enzyme-catalyzed reaction subway grid showing substrate-to-product multi-step chain, spinning `F0F1`-ATP synthase proton rotor (`l_mode=1` blade reused meaningfully!), chemiosmotic gradient, and waste efflux.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **nutriente** (*nutrient*) | **ES:** Un nutriente proporciona materia, energía o ambas para las reacciones celulares.<br>**EN:** A nutrient provides matter, energy, or both for cellular reactions. | **`[TIER-C]`** | `doc_feat==0`: Stream bright reactant spheres entering cell pore toward enzymatic hubs |
| `F1` | **reacción** (*reaction*) | **ES:** Una reacción transforma enlaces y distribuye energía entre moléculas.<br>**EN:** A reaction transforms bonds and distributes energy between molecules. | **`[TIER-C]`** | `doc_feat==1`: Snap molecular bond bridges apart & re-link into higher-order products |
| `F2` | **enzima** (*enzyme*) | **ES:** Una enzima acelera reacciones específicas y permite regular cuándo y dónde ocurren.<br>**EN:** An enzyme speeds up specific reactions and allows the cell to regulate when and where they occur. | **`[TIER-D]`** | `doc_feat==2`: Dock reactant pair inside sculpted golden enzymatic cleft catalyzing rapid turnover |
| `F3` | **ruta metabólica** (*metabolic pathway*) | **ES:** Las rutas conectan reacciones en secuencias donde el producto de una se vuelve sustrato de otra.<br>**EN:** Pathways connect reactions in sequences where the product of one becomes the substrate of another. | **`[TIER-C]`** | `doc_feat==3`: Route multi-step glowing metabolic pathway relay across 5 sequential stations |
| `F4` | **portador de energía** (*energy carrier*) | **ES:** Moléculas como ATP acoplan reacciones favorables con procesos que requieren energía.<br>**EN:** Molecules like ATP couple favorable reactions with energy-requiring processes. | **`[TIER-D]`** | `doc_feat==4`: Spin F0F1-ATP turbine releasing glowing tri-phosphate ATP carrier bursts |
| `F5` | **gradiente** (*gradient*) | **ES:** Las membranas almacenan energía mediante diferencias de concentración o carga.<br>**EN:** Membranes store energy through differences in concentration or charge. | **`[TIER-D]`** | `doc_feat==5`: Accumulate dense bright proton gradient layer along inner membrane wall |
| `F6` | **desecho** (*waste*) | **ES:** Los productos secundarios deben reciclarse, transformarse o expulsarse.<br>**EN:** Secondary products must be recycled, transformed or expelled. | **`[TIER-C]`** | `doc_feat==6`: Expel dark depleted byproduct droplets outward across efflux channels |
| `F7` | **homeostasis** (*homeostasis*) | **ES:** La regulación mantiene variables internas dentro de intervalos compatibles con la vida.<br>**EN:** Regulation maintains internal variables within ranges compatible with life. | **`[TIER-C]`** | `doc_feat==7`: Balance dual feedback regulator rings keeping pathway flux constant |

##### `[TIER-B]` E16 `Z04` (`heredity`) -> Target `vid=2604` | Información heredable / *heritable information* (`0A / 5B / 1C / 2D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Secuencia química + lectura celular + copia + reparación* / *Chemical sequence + cell reading + copy + repair* | **Evidence (ES / EN):** `Mecanismos celulares ampliamente establecidos` (*Widely established cellular mechanisms*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** El ADN no es un plano consciente. Es información química almacenada en secuencias que la maquinaria celular copia, regula e interpreta para producir ARN y proteínas.
- **Spoken Macro Summary (EN):** DNA is not a conscious blueprint. It is chemical information stored in sequences that cellular machinery copies, regulates, and interprets to produce RNA and proteins.
- **Current Visual & Diagnosis:** `vid = 20` via `hero_doc_matter_mind.wgsl:690-711` (`l_mode=0`) -> B-DNA double helix + base-pair rungs inside faint `r=0.68` spherical cell. Explicit bespoke routing `4 => 20` (`doc_stages.rs:58`) renders B-DNA double helix inside outer cell (`l_mode=0`). Matches heredity topic (`[TIER-B]`), but `doc_feat` is ignored: single-strand RNA, codon ribosome translation, folded proteins, and proofreading repair never appear.
- **3D Procedural Blueprint:** Retain `vid=20` B-DNA helix and wire `doc_feat` (`0..7`) to morph/reveal RNA transcription ribbon, codon-ribosome ring, folded protein enzyme, semiconservative replication fork, and DNA repair scan ring.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **ARN** (*RNA*) | **ES:** El ARN puede transportar información, regular procesos y, en algunos casos, catalizar reacciones.<br>**EN:** RNA can transport information, regulate processes and, in some cases, catalyze reactions. | **`[TIER-C]`** | `doc_feat==0`: Branch single-stranded glowing cyan mRNA ribbon peeling off opened DNA groove |
| `F1` | **ADN** (*DNA*) | **ES:** El ADN almacena secuencias heredables con gran estabilidad química, pero necesita una célula para ser copiado e interpretado.<br>**EN:** DNA stores heritable sequences with great chemical stability, but it needs a cell to be copied and interpreted. | **`[TIER-B]`** | `doc_feat==1`: Focus camera-side on canonical minor/major groove B-DNA double helix rungs |
| `F2` | **gen** (*gene*) | **ES:** Un gen es una región de información que contribuye a productos o regulación dentro de un sistema celular.<br>**EN:** A gene is a region of information that contributes to products or regulation within a cellular system. | **`[TIER-B]`** | `doc_feat==2`: Highlight illuminated coding gene locus bracket along middle helix turn |
| `F3` | **código genético** (*genetic code*) | **ES:** El código relaciona tripletes de nucleótidos con aminoácidos durante la síntesis de proteínas.<br>**EN:** The code relates triplets of nucleotides to amino acids during protein synthesis. | **`[TIER-D]`** | `doc_feat==3`: Slide toroidal ribosome collar over RNA reading 3-base codon triplets |
| `F4` | **proteína** (*protein*) | **ES:** Las proteínas pueden actuar como enzimas, estructuras, transportadores, señales y motores.<br>**EN:** Proteins can act as enzymes, structures, transporters, signals and motors. | **`[TIER-D]`** | `doc_feat==4`: Extrude folding nascent polypeptide ribbon into 3D globular enzyme structure |
| `F5` | **mutación** (*mutation*) | **ES:** Una mutación modifica información heredable y puede ser perjudicial, neutral o beneficiosa según el contexto.<br>**EN:** A mutation modifies heritable information and can be harmful, neutral or beneficial depending on the context. | **`[TIER-B]`** | `doc_feat==5`: Flash glowing magenta base-pair substitution ripple at one DNA rung |
| `F6` | **copia** (*copy*) | **ES:** La replicación molecular conserva secuencias mediante complementariedad química.<br>**EN:** Molecular replication conserves sequences through chemical complementarity. | **`[TIER-B]`** | `doc_feat==6`: Open Y-shaped helicase replication fork unzipping helix into twin daughter helices |
| `F7` | **reparación** (*repair*) | **ES:** Sistemas de reparación detectan ciertos errores y daños, aumentando la fidelidad sin hacerla perfecta.<br>**EN:** Repair systems detect certain errors and damage, increasing fidelity without making it perfect. | **`[TIER-B]`** | `doc_feat==7`: Run scanning amber repair ring along backbone correcting mismatched glowing rung |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP16-Z00-04)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP16-Z00-04` (Odyssey Act IV — Experience 16 Zones `Z00..Z04`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files (4+1 focused slice):
1. `crates/cosmos_render/src/doc_stages.rs` — route `experience_id == 16` `Z00..=Z04` directly to reserved `vid = 2600..=2604` (and populate `color_b = [zone_c.x, zone_c.y, zone_c.z, 1.0]` at `L739`), plus companion emitters ``3770` (`Z04` lagging-strand polymerase / ribonucleoprotein RNA companion)` where specified.
2. `crates/cosmos_narration/src/resolver.rs` — enrich `SegmentVisualHint` / `DOMAIN_CUE_KEYWORDS` (`base_flags` / `cue_tags` / `telemetry`) for the Exp `16` `Z00..=Z04` spoken concepts.
3. `shaders/hero_doc_matter_mind.wgsl` — handle decoded `vid >= 2600 && vid <= 2604` inside `march_doc_bio_mind` with distinct 3D procedural SDF / volumetric structures AND `switch (doc_feat(inst))` sub-stage branching (`0..k`, `0xFF` macro overview) smoothly driven by `doc_cue(inst)` & `doc_weight(inst)` (`0.72..1.35x` HDR cap).
4. `crates/cosmos_render/src/universe_scene.rs` + `shaders/compute_universe.wgsl` — ensure `field_grid_tex` / particle compute mode matches each zone (Compute Mode `6` (`compute_universe.wgsl:223-237` BZ/cellular reaction plumes; update misleading `(15)` comment)).

Per-Zone & `doc_feat` Blueprints:
  - `Z00` `threshold` (`vid=2600`): Move Tree of Life (`23`) to `Z05` and give `Z00` a **Pre-LUCA Protocell Threshold Triad**: semipermeable vesicle enclosing coupled reaction network loops, steep membrane gradient halo, and imperfect template strand. Features: `F0` compartimento/compartment, `F1` red de reacciones/reaction network, `F2` gradiente/gradient, `F3` replicación/replication, `F4` variación/variation, `F5` cooperación molecular/molecular cooperation, `F6` herencia imperfecta/imperfect inheritance, `F7` pregunta abierta/open question.
  - `Z01` `firstlife` (`vid=2601`): **Swarming Early Replicator Population & Ghost Lineages**: cluster of simple capsule protocells dividing in variable micro-environments, with fading translucent extinct lineages alongside one unbroken golden ancestral thread leading to LUCA. Features: `F0` unidad/unit, `F1` mantenimiento/maintenance, `F2` descendencia/offspring, `F3` población/population, `F4` entorno/environment, `F5` continuidad/continuity, `F6` ancestro común/common ancestor, `F7` extinción temprana/early extinction.
  - `Z02` `cells` (`vid=2602`): **Prokaryotic Primitive Cell Cross-Section**: simple peptidoglycan/lipid cell capsule with nucleotide-rich nucleoid loop, membrane-embedded ion channels, food intake vesicles, and central FtsZ contractile division ring. Features: `F0` célula/cell, `F1` membrana/membrane, `F2` interior/interior, `F3` energía/energy, `F4` alimento/food, `F5` reparación/repair, `F6` división/division, `F7` entorno/environment.
  - `Z03` `metabolism` (`vid=2603`): **Cellular Metabolic Reactor**: cross-sectioned enzyme-catalyzed reaction subway grid showing substrate-to-product multi-step chain, spinning `F0F1`-ATP synthase proton rotor (`l_mode=1` blade reused meaningfully!), chemiosmotic gradient, and waste efflux. Features: `F0` nutriente/nutrient, `F1` reacción/reaction, `F2` enzima/enzyme, `F3` ruta metabólica/metabolic pathway, `F4` portador de energía/energy carrier, `F5` gradiente/gradient, `F6` desecho/waste, `F7` homeostasis/homeostasis.
  - `Z04` `heredity` (`vid=2604`): Retain `vid=20` B-DNA helix and wire `doc_feat` (`0..7`) to morph/reveal RNA transcription ribbon, codon-ribosome ring, folded protein enzyme, semiconservative replication fork, and DNA repair scan ring. Features: `F0` ARN/RNA, `F1` ADN/DNA, `F2` gen/gene, `F3` código genético/genetic code, `F4` proteína/protein, `F5` mutación/mutation, `F6` copia/copy, `F7` reparación/repair.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; keep volumetric shells tightly bounded inside hero sphere (`r <= 0.96`, early-exit `trans < 0.03`) with zero full-screen fog wash.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps for new documentary hero shells, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

<a id="pkg-act4-exp16-z05-09"></a>
### PKG-ACT4-EXP16-Z05-09: Exp `16` — Evolutionary Tree, Photosynthesis, Complex Eukaryotes, Sexual Recombination & Multicellularity (`Z05..Z09`)

- **Phase & Priority:** `Phase 1/2` · **P1 — High (Places Tree of Life at `Z05`, Chloroplast at `Z06`, Mitochondrion+Nucleus at `Z07`, Sex/Multicell)** · **Scope:** `5` Zones (`Z05..Z09`), `40` Features (`doc_feat` `0..k`)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:56-60, 731-744`](../../crates/cosmos_render/src/doc_stages.rs#L56-L60) & [`shaders/hero_doc_matter_mind.wgsl:610, 687-759`](../../shaders/hero_doc_matter_mind.wgsl#L687-L759); `vid`: `doc_stage_variant_id(16)` -> `0->23, 4->20, _->20+(sc%5)` (`20, 21, 22, 23, 24`; catalog `scene = null`); `doc_feat(inst)` unused in WGSL; `color_b` zone accent unused (`doc_stages.rs:731-744`). **Card Tiers — Zones:** `0A / 2B / 1C / 2D` · **Features:** `0A / 10B / 8C / 22D`.
- **Compute & Plumbing Focus:** Compute Mode `6` cellular reaction (`Z05..Z08`) transitioning toward cooperative intercellular signaling streams on `Z09`. Extra files: `crates/cosmos_render/src/universe_scene.rs` (`L320` dynamic Exp 16 compute parameter via `doc.active_zone_index`).

#### Reserved Target IDs

| Zone | Slug | ES / EN Title | Current `vid` | Reserved Target `vid` | Companion IDs | `submode` & `doc_feat` Range |
|---|---|---|---:|---:|---|---|
| `Z05` | `evolution` | Evolución darwiniana / *Darwinian evolution* | `20` (`[TIER-C]`) | **`2605`** | `3771` (`Z07` endosymbiont mitochondria/plastid cluster inside eukaryotic host) | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z06` | `photosynthesis` | Fotosíntesis / *Photosynthesis* | `21` (`[TIER-B]`) | **`2606`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z07` | `complexcells` | Células complejas / *Complex cells* | `22` (`[TIER-B]`) | **`2607`** | `3771` (`Z07` endosymbiont mitochondria/plastid cluster inside eukaryotic host) | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z08` | `sex` | Reproducción sexual / *sexual reproduction* | `23` (`[TIER-D]`) | **`2608`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z09` | `multicellular` | Multicelularidad / *Multicellularity* | `24` (`[TIER-D]`) | **`2609`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |

#### Spoken Transcript Summary & 3D Visual Blueprint

Key deliverables for this Pick-Up Card:
- `vid=2605` (`Z05 evolution`): Relocate `vid=23` 3D Phylogenetic Tree of Life here + evolutionary branching fitness peaks & selection sieve (`doc_feat 0..7`).
- `vid=2606` (`Z06 photosynthesis`): Relocate `vid=21` Chloroplast Thylakoid Stacks + ATP Turbine here + UV photolysis water-splitting & Calvin carbon ring (`doc_feat 0..7`).
- `vid=2607` (`Z07 complexcells`): Upgrade `vid=22` Eukaryotic Host enclosing Nuclear Envelope + Endoplasmic Reticulum + Mitochondrion Cristae + Cytoskeleton cables (`doc_feat 0..7`).
- `vid=2608` (`Z08 sex`): Replace Tree of Life with Meiotic Homologous Chromosome Pair + Chiasma Crossover + Fusion Gametes (`doc_feat 0..7`).
- `vid=2609` (`Z09 multicellular`): Replace single axon with Volvox-to-Metazoan Cell Adhesion Cluster (tight junctions, differentiated core/surface fates, vascular nutrient tubes; `doc_feat 0..7`).

##### `[TIER-C]` E16 `Z05` (`evolution`) -> Target `vid=2605` | Evolución darwiniana / *Darwinian evolution* (`0A / 0B / 8C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Variación + herencia + reproducción diferencial + generaciones* / *Variation + inheritance + differential reproduction + generations* | **Evidence (ES / EN):** `Principio central confirmado por múltiples evidencias` (*Central principle confirmed by multiple lines of evidence*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Los individuos no evolucionan durante su vida; evolucionan las poblaciones a través de generaciones. Selección, deriva, mutación, migración y cooperación cambian la historia de los linajes.
- **Spoken Macro Summary (EN):** Individuals do not evolve during their lifetimes; populations evolve over generations. Selection, drift, mutation, migration, and cooperation shape the history of lineages.
- **Current Visual & Diagnosis:** `vid = 20` via `hero_doc_matter_mind.wgsl:690-711` (`l_mode=0`) -> B-DNA double helix + base-pair rungs inside faint `r=0.68` spherical cell. Routed via `5 % 5 = 0` -> `vid=20` (single static B-DNA helix). Population-level Darwinian evolution (selection, drift, cooperation, extinction, common descent) needs lineages & populations, not one static molecule—while the Tree of Life (`vid=23`) sits idle on `Z00/Z03/Z08/Z13/Z18`!
- **3D Procedural Blueprint:** Combine **Population Variant Cloud + Phylogenetic Tree (`vid=23+`)**: multi-colored trait orbs competing across shifting environmental gradients whose survivors weave upward into branching evolutionary tree trunks & ghosted extinct boughs.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **variación** (*variation*) | **ES:** Los miembros de una población difieren en rasgos heredables y no heredables.<br>**EN:** Members of a population differ in heritable and non-heritable traits. | **`[TIER-C]`** | `doc_feat==0`: Tint swarming population variants across 6 distinct heritable morphology hues |
| `F1` | **selección** (*selection*) | **ES:** Las variantes que dejan más descendencia tienden a aumentar su frecuencia bajo esas condiciones.<br>**EN:** Variants that leave more offspring tend to increase in frequency under these conditions. | **`[TIER-C]`** | `doc_feat==1`: Expand turquoise fast-reproducing clade under adaptive filter gradient |
| `F2` | **adaptación** (*adaptation*) | **ES:** Una adaptación es un rasgo acumulado porque favoreció reproducción o supervivencia en ambientes anteriores.<br>**EN:** An adaptation is a trait accumulated because it favored reproduction or survival in previous environments. | **`[TIER-C]`** | `doc_feat==2`: Glow beneficial traits interlocking smoothly with jagged environment barrier |
| `F3` | **deriva genética** (*genetic drift*) | **ES:** El azar puede cambiar frecuencias génicas, especialmente en poblaciones pequeñas.<br>**EN:** Chance can change gene frequencies, especially in small populations. | **`[TIER-C]`** | `doc_feat==3`: Fluctuate tiny bottleneck sub-cluster frequencies randomly with zero selective bias |
| `F4` | **competencia** (*competition*) | **ES:** Organismos pueden depender de recursos limitados, pero la evolución no se reduce a lucha directa.<br>**EN:** Organisms may depend on limited resources, but evolution is not reduced to direct struggle. | **`[TIER-C]`** | `doc_feat==4`: Show dense crowding around sparse nutrient pool triggering competitive pruning |
| `F5` | **cooperación** (*cooperation*) | **ES:** La cooperación puede evolucionar cuando produce beneficios que favorecen su persistencia.<br>**EN:** Cooperation can evolve when it produces benefits that favor its persistence. | **`[TIER-C]`** | `doc_feat==5`: Link cooperative mutualist pairs generating boosted dual-halo survival |
| `F6` | **extinción** (*extinction*) | **ES:** La mayoría de los linajes que existieron desapareció sin descendientes actuales.<br>**EN:** Most of the lineages that existed disappeared without current descendants. | **`[TIER-C]`** | `doc_feat==6`: Fade 80% of lower canopy sub-branches into translucent extinct fossil ghosts |
| `F7` | **ancestro común** (*common ancestor*) | **ES:** La semejanza profunda entre seres vivos se explica por descendencia con modificación.<br>**EN:** The profound similarity between living beings is explained by descent with modification. | **`[TIER-C]`** | `doc_feat==7`: Converge all surviving living branches cleanly back down to single LUCA root |

##### `[TIER-B]` E16 `Z06` (`photosynthesis`) -> Target `vid=2606` | Fotosíntesis / *Photosynthesis* (`0A / 5B / 0C / 3D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Luz + pigmentos + carbono → energía química y biomasa* / *Light + pigments + carbon → chemical energy and biomass* | **Evidence (ES / EN):** `Innovación metabólica con impacto planetario` (*Metabolic innovation with planetary impact*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La fotosíntesis captura energía luminosa. La variante que libera oxígeno transformó océanos, atmósfera y posibilidades metabólicas durante enormes escalas de tiempo.
- **Spoken Macro Summary (EN):** Photosynthesis captures light energy. The oxygen-releasing variant transformed oceans, atmosphere and metabolic possibilities over enormous time scales.
- **Current Visual & Diagnosis:** `vid = 21` via `hero_doc_matter_mind.wgsl:712-718` (`l_mode=1`) -> Chloroplast thylakoid disc stacks + spinning 6-blade ATP synthase turbine. Routed via `6 % 5 = 1` -> `vid=21` (Chloroplast thylakoid disc stacks + spinning ATP synthase turbine). Directly depicts photosynthesis (`[TIER-B]`), though `doc_feat` is ignored and concrete historical phenomena (`tapete microbiano`, planetary atmospheric oxygenation) have no visual.
- **3D Procedural Blueprint:** Keep `vid=21` chloroplast thylakoids & ATP rotor; add `doc_feat`-driven golden photon rays, chlorophyll antenna resonance, glowing carbon-fixation sugar rings, layered stromatolite disc base, and rising $O_2$ bubble stream.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **luz solar** (*sunlight*) | **ES:** Los fotones entregan energía que los pigmentos pueden absorber.<br>**EN:** Photons deliver energy that pigments can absorb. | **`[TIER-B]`** | `doc_feat==0`: Fire cascading golden photon ray packets striking thylakoid stack face |
| `F1` | **pigmento** (*pigment*) | **ES:** Un pigmento absorbe ciertas longitudes de onda y transfiere energía a reacciones químicas.<br>**EN:** A pigment absorbs certain wavelengths and transfers energy to chemical reactions. | **`[TIER-B]`** | `doc_feat==1`: Resonate emerald chlorophyll antenna rings absorbing specific wavelengths |
| `F2` | **fijación de carbono** (*carbon fixation*) | **ES:** El carbono inorgánico se incorpora a moléculas orgánicas que forman biomasa.<br>**EN:** Inorganic carbon is incorporated into organic molecules that form biomass. | **`[TIER-B]`** | `doc_feat==2`: Assemble glowing hexagonal glucose sugar rings in surrounding stroma matrix |
| `F3` | **tapete microbiano** (*microbial mat*) | **ES:** Comunidades estratificadas de microbios pueden cooperar y modificar sedimentos y química local.<br>**EN:** Stratified communities of microbes can cooperate and modify sediments and local chemistry. | **`[TIER-D]`** | `doc_feat==3`: Extrude stratified dome-shaped microbial mat stromatolite column below |
| `F4` | **oxígeno** (*oxygen*) | **ES:** La fotosíntesis oxigénica libera oxígeno como producto secundario.<br>**EN:** Oxygenic photosynthesis releases oxygen as a by-product. | **`[TIER-B]`** | `doc_feat==4`: Stream buoyant cerulean $O_2$ gas bubbles rising off thylakoid membranes |
| `F5` | **atmósfera** (*atmosphere*) | **ES:** La acumulación de oxígeno alteró minerales, océanos, clima y metabolismo global.<br>**EN:** The accumulation of oxygen altered minerals, oceans, climate and global metabolism. | **`[TIER-D]`** | `doc_feat==5`: Wrap stage inside translucent blue planetary atmosphere halo gradually brightening |
| `F6` | **estrés oxidativo** (*oxidative stress*) | **ES:** El oxígeno permite reacciones energéticas, pero también genera especies químicas dañinas.<br>**EN:** Oxygen allows energetic reactions, but also generates harmful chemical species. | **`[TIER-B]`** | `doc_feat==6`: Flash jagged reactive-oxygen radical sparks around overloaded reaction centers |
| `F7` | **transformación planetaria** (*planetary transformation*) | **ES:** La vida no solo se adapta al planeta: también cambia sus ciclos y condiciones.<br>**EN:** Life not only adapts to the planet: it also changes its cycles and conditions. | **`[TIER-D]`** | `doc_feat==7`: Sweep planetary globe rim converting anoxic rust-iron ocean into cyan biosphere |

##### `[TIER-B]` E16 `Z07` (`complexcells`) -> Target `vid=2607` | Células complejas / *Complex cells* (`0A / 5B / 0C / 3D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Núcleo + orgánulos + membranas internas + endosimbiosis* / *Nucleus + organelles + internal membranes + endosymbiosis* | **Evidence (ES / EN):** `Eucariogénesis parcialmente reconstruida` (*Partially reconstructed eukaryogenesis*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Las células eucariotas integraron compartimentos internos y antiguos socios microbianos. Las mitocondrias descienden de bacterias que se volvieron partes permanentes de otra célula.
- **Spoken Macro Summary (EN):** Eukaryotic cells integrated internal compartments and ancient microbial partners. Mitochondria descend from bacteria that became permanent parts of another cell.
- **Current Visual & Diagnosis:** `vid = 22` via `hero_doc_matter_mind.wgsl:719-726` (`l_mode=2`) -> Mitochondrion ellipsoid + sinusoidal inner cristae foldings. Routed via `7 % 5 = 2` -> `vid=22` (Mitochondrion ellipsoid w/ sinusoidal cristae). Mitochondrion is the signature emblem of eukaryotic complex cells (`[TIER-B]`), yet it lacks the enclosing nucleus, ER/Golgi endomembrane network, and cytoskeleton filaments.
- **3D Procedural Blueprint:** **Full Eukaryotic Cell Cutaway** anchoring `vid=22` mitochondria inside a porous nuclear envelope, concentric rough-ER cisternae folds, actin/microtubule cytoskeleton lattice, and animated archaeal-host bacterial engulfment.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **núcleo** (*nucleus*) | **ES:** El núcleo separa gran parte del ADN del citoplasma y organiza su acceso y copia.<br>**EN:** The nucleus separates much of the DNA from the cytoplasm and organizes its access and copying. | **`[TIER-D]`** | `doc_feat==0`: Render porous double-walled purple nucleus enclosing condensed chromatin loops |
| `F1` | **orgánulo** (*organelle*) | **ES:** Un orgánulo concentra funciones dentro de un compartimento celular.<br>**EN:** An organelle concentrates functions within a cellular compartment. | **`[TIER-B]`** | `doc_feat==1`: Cutaway bounding eukaryotic plasma sphere housing distinct colored organelle pods |
| `F2` | **mitocondria** (*mitochondria*) | **ES:** Las mitocondrias transforman energía y conservan señales de su origen bacteriano.<br>**EN:** Mitochondria transform energy and conserve signals from their bacterial origin. | **`[TIER-B]`** | `doc_feat==2`: Spotlight inner folded cristae mitochondrion (`vid=22` core hero) |
| `F3` | **endosimbiosis** (*endosymbiosis*) | **ES:** Una asociación entre organismos distintos puede volverse dependencia e integración heredable.<br>**EN:** An association between different organisms can become dependence and heritable integration. | **`[TIER-B]`** | `doc_feat==3`: Animate ancient phagocytosis loop engulfing smaller bacterium into permanent host |
| `F4` | **membrana interna** (*inner membrane*) | **ES:** Redes internas separan procesos incompatibles y permiten transporte dirigido.<br>**EN:** Internal networks separate incompatible processes and allow directed transport. | **`[TIER-D]`** | `doc_feat==4`: Fold layered wavy ER/Golgi endomembrane ribbons branching from nuclear envelope |
| `F5` | **citoesqueleto** (*cytoskeleton*) | **ES:** Filamentos internos dan forma, mueven materiales y participan en división y movimiento.<br>**EN:** Internal filaments give shape, move materials and participate in division and movement. | **`[TIER-D]`** | `doc_feat==5`: Spin spanning structural microtubule & actin cytoskeleton lattice beams |
| `F6` | **genoma mayor** (*major genome*) | **ES:** La regulación eucariota permite coordinar genomas extensos, aunque tamaño no equivale directamente a complejidad.<br>**EN:** Eukaryotic regulation allows extensive genomes to be coordinated, although size does not directly equate to complexity. | **`[TIER-B]`** | `doc_feat==6`: Expand glowing nuclear genome ring density relative to tiny mitochondrial ring |
| `F7` | **socio integrado** (*integrated partner*) | **ES:** Una cooperación histórica puede convertirse en una nueva unidad evolutiva.<br>**EN:** A historical cooperation can become a new evolutionary unit. | **`[TIER-B]`** | `doc_feat==7`: Pulse bi-directional ATP-for-pyruvate metabolic exchange bridges host<->mitochondrion |

##### `[TIER-D]` E16 `Z08` (`sex`) -> Target `vid=2608` | Reproducción sexual / *sexual reproduction* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Meiosis + gametos + recombinación + herencia combinada* / *Meiosis + gametes + recombination + combined inheritance* | **Evidence (ES / EN):** `Proceso evolutivo diverso y costoso` (*Diverse and costly evolutionary process*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La reproducción sexual mezcla material genético y crea combinaciones nuevas. Aumenta diversidad, pero requiere coordinación y tiene costes; la vida utiliza muchas estrategias sexuales y asexuales.
- **Spoken Macro Summary (EN):** Sexual reproduction mixes genetic material and creates new combinations. It increases diversity but requires coordination and has costs; life uses many sexual and asexual strategies.
- **Current Visual & Diagnosis:** `vid = 23` via `hero_doc_matter_mind.wgsl:727-750` (`l_mode=3`) -> 3D Phylogenetic Tree of Life (LUCA root -> 3 domain branches + canopy ring). Routed via `8 % 5 = 3` -> `vid=23` (Phylogenetic Tree of Life). Narrates meiosis, chromosome crossover recombination, gametes, and fertilization—concrete cytogenetic choreography completely absent from WGSL.
- **3D Procedural Blueprint:** **Meiosis & Syngamy Stage**: paired homologous X-chromosomes undergoing chiasmata segment-swapping recombination, bisecting twice into 4 haploid gametes, followed by flagellated sperm fusing with ovum shell.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **meiosis** (*meiosis*) | **ES:** La meiosis reduce el número de cromosomas y redistribuye variantes genéticas.<br>**EN:** Meiosis reduces the number of chromosomes and redistributes genetic variants. | **`[TIER-D]`** | `doc_feat==0`: Split diploid pair through 2-stage meiotic reduction into 4 haploid tetrads |
| `F1` | **gameto** (*gamete*) | **ES:** Un gameto es una célula reproductiva que puede fusionarse con otra.<br>**EN:** A gamete is a reproductive cell that can fuse with another. | **`[TIER-D]`** | `doc_feat==1`: Animate nimble flagellated micro-gamete beside large translucent macro-gamete |
| `F2` | **fecundación** (*fertilization*) | **ES:** La fusión reúne información de dos células y forma una nueva combinación genética.<br>**EN:** The fusion brings together information from two cells and forms a new genetic combination. | **`[TIER-D]`** | `doc_feat==2`: Fuse gamete membranes & merge twin pronuclei into glowing diploid zygote |
| `F3` | **recombinación** (*recombination*) | **ES:** Los cromosomas intercambian segmentos, produciendo combinaciones no presentes exactamente en los progenitores.<br>**EN:** Chromosomes exchange segments, producing combinations not exactly present in the parents. | **`[TIER-D]`** | `doc_feat==3`: Exchange glowing cyan/amber chromosome arms at X-shaped crossover chiasmata |
| `F4` | **diversidad genética** (*genetic diversity*) | **ES:** La mezcla acelera la exploración de combinaciones heredables.<br>**EN:** Admixture accelerates the exploration of heritable combinations. | **`[TIER-D]`** | `doc_feat==4`: Fan 6 differently patchwork-banded daughter chromosomes showing shuffled traits |
| `F5` | **dos progenitores** (*two parents*) | **ES:** Muchos organismos heredan material de dos progenitores, aunque existen sistemas reproductivos muy variados.<br>**EN:** Many organisms inherit material from two parents, although there are very varied reproductive systems. | **`[TIER-D]`** | `doc_feat==5`: Pair complementary paternal blue & maternal gold chromosome sets side-by-side |
| `F6` | **coste** (*cost*) | **ES:** Encontrar pareja, producir gametos y transmitir solo parte del genoma puede ser costoso.<br>**EN:** Finding a mate, producing gametes, and passing on only part of the genome can be costly. | **`[TIER-D]`** | `doc_feat==6`: Pulse high energetic swarm overhead illustrating mating/reproductive cost |
| `F7` | **ventaja contextual** (*contextual advantage*) | **ES:** La sexualidad puede ayudar frente a ambientes cambiantes, parásitos y acumulación de mutaciones.<br>**EN:** Sexuality can help against changing environments, parasites and accumulation of mutations. | **`[TIER-D]`** | `doc_feat==7`: Contrast clonal identical swarm facing invading parasites vs mosaic sexual swarm |

##### `[TIER-D]` E16 `Z09` (`multicellular`) -> Target `vid=2609` | Multicelularidad / *Multicellularity* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Adhesión + comunicación + cooperación + especialización* / *Adhesion + communication + cooperation + specialization* | **Evidence (ES / EN):** `Evolucionó muchas veces de forma independiente` (*Evolved many times independently*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Un organismo multicelular no es solo una colonia de células idénticas. Sus células coordinan crecimiento, división de tareas, reparación, reproducción y muerte programada.
- **Spoken Macro Summary (EN):** A multicellular organism is not just a colony of identical cells. Its cells coordinate growth, division of tasks, repair, reproduction and programmed death.
- **Current Visual & Diagnosis:** `vid = 24` via `hero_doc_matter_mind.wgsl:751-759` (`l_mode=4`) -> Myelinated axon shaft + saltatory action potential pulse ring. Routed via `9 % 5 = 4` -> `vid=24` (Myelinated neuron axon!). A single nerve fiber gives zero intuition for multicellular epithelial adhesion, morphogen axes, division of labor, programmed apoptosis, or rogue tumor proliferation.
- **3D Procedural Blueprint:** **Multicellular Tissue Epithelial Sheet / Morula Sphere**: packed deformed Voronoi-like SDF cells bound by bright cadherin junction bridges, differentiated sub-layers, dissolving apoptotic cell blebs, and localized rogue over-proliferating nodule.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **adhesión** (*adhesion*) | **ES:** Moléculas de unión permiten que células permanezcan juntas y transmitan fuerzas.<br>**EN:** Bonding molecules allow cells to stay together and transmit forces. | **`[TIER-D]`** | `doc_feat==0`: Ignite bright intercellular cadherin junction rivets binding adjacent cell membranes |
| `F1` | **comunicación** (*communication*) | **ES:** Señales químicas y eléctricas coordinan decisiones entre células.<br>**EN:** Chemical and electrical signals coordinate decisions between cells. | **`[TIER-D]`** | `doc_feat==1`: Propagate calcium & chemical paracrine signaling waves smoothly across cell sheet |
| `F2` | **cooperación** (*cooperation*) | **ES:** Las células renuncian a parte de su independencia para sostener el organismo completo.<br>**EN:** Cells give up some of their independence to sustain the entire organism. | **`[TIER-D]`** | `doc_feat==2`: Suppress peripheral self-division rings while sharing metabolic glow across core |
| `F3` | **especialización** (*specialization*) | **ES:** Tipos celulares distintos realizan tareas complementarias.<br>**EN:** Different cell types perform complementary tasks. | **`[TIER-D]`** | `doc_feat==3`: Morph tissue strata into distinct sensory, contractile, and structural cell shapes |
| `F4` | **muerte programada** (*programmed death*) | **ES:** La eliminación controlada de células puede formar estructuras y proteger al organismo.<br>**EN:** The controlled removal of cells can form structures and protect the body. | **`[TIER-D]`** | `doc_feat==4`: Sculpt hollow cavity cleanly by fizzing targeted central cells (apoptosis) |
| `F5` | **desarrollo** (*development*) | **ES:** Reglas locales de división, movimiento y señalización construyen un cuerpo.<br>**EN:** Local rules of division, movement and signaling build a body. | **`[TIER-D]`** | `doc_feat==5`: Sweep anterior-posterior morphogen wave folding flat sheet into tubular fold |
| `F6` | **conflicto** (*conflict*) | **ES:** Células que dejan de cooperar pueden proliferar de forma dañina, como ocurre en el cáncer.<br>**EN:** Cells that stop cooperating can proliferate in a harmful way, as occurs in cancer. | **`[TIER-D]`** | `doc_feat==6`: Erupt rebellious crimson rogue cell cluster breaking adhesion borders (cancer) |
| `F7` | **plan corporal** (*body plan*) | **ES:** Los patrones de desarrollo organizan ejes, regiones y relaciones entre partes.<br>**EN:** Development patterns organize axes, regions and relationships between parts. | **`[TIER-D]`** | `doc_feat==7`: Overlay glowing orthogonal embryo axes (anterior-posterior, dorsal-ventral) |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP16-Z05-09)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP16-Z05-09` (Odyssey Act IV — Experience 16 Zones `Z05..Z09`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files (4+1 focused slice):
1. `crates/cosmos_render/src/doc_stages.rs` — route `experience_id == 16` `Z05..=Z09` directly to reserved `vid = 2605..=2609` (and populate `color_b = [zone_c.x, zone_c.y, zone_c.z, 1.0]` at `L739`), plus companion emitters ``3771` (`Z07` endosymbiont mitochondria/plastid cluster inside eukaryotic host)` where specified.
2. `crates/cosmos_narration/src/resolver.rs` — enrich `SegmentVisualHint` / `DOMAIN_CUE_KEYWORDS` (`base_flags` / `cue_tags` / `telemetry`) for the Exp `16` `Z05..=Z09` spoken concepts.
3. `shaders/hero_doc_matter_mind.wgsl` — handle decoded `vid >= 2605 && vid <= 2609` inside `march_doc_bio_mind` with distinct 3D procedural SDF / volumetric structures AND `switch (doc_feat(inst))` sub-stage branching (`0..k`, `0xFF` macro overview) smoothly driven by `doc_cue(inst)` & `doc_weight(inst)` (`0.72..1.35x` HDR cap).
4. `crates/cosmos_render/src/universe_scene.rs` + `shaders/compute_universe.wgsl` — ensure `field_grid_tex` / particle compute mode matches each zone (Compute Mode `6` cellular reaction (`Z05..Z08`) transitioning toward cooperative intercellular signaling streams on `Z09`).

Per-Zone & `doc_feat` Blueprints:
  - `Z05` `evolution` (`vid=2605`): Combine **Population Variant Cloud + Phylogenetic Tree (`vid=23+`)**: multi-colored trait orbs competing across shifting environmental gradients whose survivors weave upward into branching evolutionary tree trunks & ghosted extinct boughs. Features: `F0` variación/variation, `F1` selección/selection, `F2` adaptación/adaptation, `F3` deriva genética/genetic drift, `F4` competencia/competition, `F5` cooperación/cooperation, `F6` extinción/extinction, `F7` ancestro común/common ancestor.
  - `Z06` `photosynthesis` (`vid=2606`): Keep `vid=21` chloroplast thylakoids & ATP rotor; add `doc_feat`-driven golden photon rays, chlorophyll antenna resonance, glowing carbon-fixation sugar rings, layered stromatolite disc base, and rising $O_2$ bubble stream. Features: `F0` luz solar/sunlight, `F1` pigmento/pigment, `F2` fijación de carbono/carbon fixation, `F3` tapete microbiano/microbial mat, `F4` oxígeno/oxygen, `F5` atmósfera/atmosphere, `F6` estrés oxidativo/oxidative stress, `F7` transformación planetaria/planetary transformation.
  - `Z07` `complexcells` (`vid=2607`): **Full Eukaryotic Cell Cutaway** anchoring `vid=22` mitochondria inside a porous nuclear envelope, concentric rough-ER cisternae folds, actin/microtubule cytoskeleton lattice, and animated archaeal-host bacterial engulfment. Features: `F0` núcleo/nucleus, `F1` orgánulo/organelle, `F2` mitocondria/mitochondria, `F3` endosimbiosis/endosymbiosis, `F4` membrana interna/inner membrane, `F5` citoesqueleto/cytoskeleton, `F6` genoma mayor/major genome, `F7` socio integrado/integrated partner.
  - `Z08` `sex` (`vid=2608`): **Meiosis & Syngamy Stage**: paired homologous X-chromosomes undergoing chiasmata segment-swapping recombination, bisecting twice into 4 haploid gametes, followed by flagellated sperm fusing with ovum shell. Features: `F0` meiosis/meiosis, `F1` gameto/gamete, `F2` fecundación/fertilization, `F3` recombinación/recombination, `F4` diversidad genética/genetic diversity, `F5` dos progenitores/two parents, `F6` coste/cost, `F7` ventaja contextual/contextual advantage.
  - `Z09` `multicellular` (`vid=2609`): **Multicellular Tissue Epithelial Sheet / Morula Sphere**: packed deformed Voronoi-like SDF cells bound by bright cadherin junction bridges, differentiated sub-layers, dissolving apoptotic cell blebs, and localized rogue over-proliferating nodule. Features: `F0` adhesión/adhesion, `F1` comunicación/communication, `F2` cooperación/cooperation, `F3` especialización/specialization, `F4` muerte programada/programmed death, `F5` desarrollo/development, `F6` conflicto/conflict, `F7` plan corporal/body plan.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; keep volumetric shells tightly bounded inside hero sphere (`r <= 0.96`, early-exit `trans < 0.03`) with zero full-screen fog wash.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps for new documentary hero shells, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

<a id="pkg-act4-exp16-z10-14"></a>
### PKG-ACT4-EXP16-Z10-14: Exp `16` — Embryonic Development, Organisms, Ecosystems, Active Movement & Sensory Transduction (`Z10..Z14`)

- **Phase & Priority:** `Phase 1/2` · **P1 — High (Builds missing embryo morphogenesis, biome globe, musculoskeletal locomotion & sensory organs)** · **Scope:** `5` Zones (`Z10..Z14`), `40` Features (`doc_feat` `0..k`)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:56-60, 731-744`](../../crates/cosmos_render/src/doc_stages.rs#L56-L60) & [`shaders/hero_doc_matter_mind.wgsl:610, 687-759`](../../shaders/hero_doc_matter_mind.wgsl#L687-L759); `vid`: `doc_stage_variant_id(16)` -> `0->23, 4->20, _->20+(sc%5)` (`20, 21, 22, 23, 24`; catalog `scene = null`); `doc_feat(inst)` unused in WGSL; `color_b` zone accent unused (`doc_stages.rs:731-744`). **Card Tiers — Zones:** `0A / 0B / 2C / 3D` · **Features:** `0A / 0B / 5C / 35D`.
- **Compute & Plumbing Focus:** Compute Mode `6` morphogenetic diffusion (`Z10..Z11`), trophic nutrient cycling (`Z12`), chemo/mechanosensory flux (`Z13..Z14`). Extra files: `crates/cosmos_narration/src/resolver.rs` (`cue_tags` morphogen/homeostasis/ecosystem cues).

#### Reserved Target IDs

| Zone | Slug | ES / EN Title | Current `vid` | Reserved Target `vid` | Companion IDs | `submode` & `doc_feat` Range |
|---|---|---|---:|---:|---|---|
| `Z10` | `development` | Diferenciación y desarrollo / *Differentiation and development* | `20` (`[TIER-C]`) | **`2610`** | `3772` (`Z12` food-web trophic torus / abiotic biome atmosphere ring) | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z11` | `organisms` | Organismos / *Organisms* | `21` (`[TIER-D]`) | **`2611`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z12` | `ecosystems` | Ecosistemas / *Ecosystems* | `22` (`[TIER-D]`) | **`2612`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z13` | `movement` | Movimiento / *Movement* | `23` (`[TIER-D]`) | **`2613`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z14` | `senses` | Sentidos / *Senses* | `24` (`[TIER-C]`) | **`2614`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |

#### Spoken Transcript Summary & 3D Visual Blueprint

Key deliverables for this Pick-Up Card:
- `vid=2610` (`Z10 development`): Replace B-DNA with Morphogenetic Embryo Sphere (zygote cleavage, striped Hox gene bands, folding neural groove & limb buds; `doc_feat 0..7`).
- `vid=2611` (`Z11 organisms`): Replace Chloroplast with Modular Integrated Vertebrate/Metazoan Silhouette (internal circulatory-respiratory loops + homeostatic core; `doc_feat 0..7`).
- `vid=2612` (`Z12 ecosystems`): Replace Mitochondrion with Miniature Multi-Biome Biosphere Terrarium (ocean gyres, canopy, desert/tundra bands, sunlight->detritus trophic web; `doc_feat 0..7`).
- `vid=2613` (`Z13 movement`): Replace Tree of Life with Myosin/Actin Sarcomere & Articulated Limb Mechanics (phototactic cilia, contractile muscle fibers, skeletal levers; `doc_feat 0..7`).
- `vid=2614` (`Z14 senses`): Upgrade `vid=24` Myelinated Axon into Sensory Organ Array (camera eye retina, cochlear spiral, olfactory receptors & nociceptor pathways feeding the axon; `doc_feat 0..7`).

##### `[TIER-C]` E16 `Z10` (`development`) -> Target `vid=2610` | Diferenciación y desarrollo / *Differentiation and development* (`0A / 0B / 5C / 3D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Regulación génica + señales + gradientes → tejidos y órganos* / *Gene regulation + signals + gradients → tissues and organs* | **Evidence (ES / EN):** `Un genoma, muchos estados celulares` (*One genome, many cell states*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Casi todas las células de un organismo comparten el mismo genoma, pero activan programas diferentes. Señales, historia celular y posición producen tejidos, órganos y forma.
- **Spoken Macro Summary (EN):** Almost all cells in an organism share the same genome, but activate different programs. Signals, cellular history and position produce tissues, organs and form.
- **Current Visual & Diagnosis:** `vid = 20` via `hero_doc_matter_mind.wgsl:690-711` (`l_mode=0`) -> B-DNA double helix + base-pair rungs inside faint `r=0.68` spherical cell. Routed via `10 % 5 = 0` -> `vid=20` (B-DNA helix). Static single DNA molecule conveys gene sequence (`f1`) weakly, while stem-cell multipotency, embryonic morphogen gradients, tissue differentiation, and organogenesis are completely unrepresented.
- **3D Procedural Blueprint:** **Embryonic Lineage Differentiation Tree & Morphogen Blastula**: central multipotent stem cluster radiating outward along Waddington-valley paths into distinct contractile, neural, and epithelial tissues sculpted by chemical gradients.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **célula madre** (*stem cell*) | **ES:** Una célula madre puede autorrenovarse y generar uno o más tipos celulares especializados.<br>**EN:** A stem cell can self-renew and generate one or more specialized cell types. | **`[TIER-D]`** | `doc_feat==0`: Glow multipotent golden stem sphere dividing asymmetrically into specialist daughter |
| `F1` | **regulación génica** (*gene regulation*) | **ES:** Activar o silenciar genes cambia qué moléculas produce una célula.<br>**EN:** Turning genes on or silencing changes what molecules a cell produces. | **`[TIER-C]`** | `doc_feat==1`: Toggle radial gene-switch ring nodes lighting distinct subsets along genome |
| `F2` | **gradiente químico** (*chemical gradient*) | **ES:** Una señal distribuida en concentración puede informar a las células sobre su posición.<br>**EN:** A signal distributed in concentration can inform cells about their position. | **`[TIER-C]`** | `doc_feat==2`: Project smooth vertical chemical morphogen concentration gradient across blastula |
| `F3` | **tejido** (*tissue*) | **ES:** Un tejido combina células y materiales extracelulares con una función compartida.<br>**EN:** A tissue combines cells and extracellular materials with a shared function. | **`[TIER-D]`** | `doc_feat==3`: Pack uniform differentiated cells along cohesive layered tissue sheet |
| `F4` | **órgano** (*organ*) | **ES:** Un órgano integra varios tejidos en una unidad funcional.<br>**EN:** An organ integrates several tissues into a functional unit. | **`[TIER-D]`** | `doc_feat==4`: Fold multilayered tissues into pulsing miniature tubular heart/organ unit |
| `F5` | **crecimiento** (*growth*) | **ES:** El crecimiento coordina aumento de tamaño, división celular y disponibilidad de recursos.<br>**EN:** Growth coordinates increase in size, cell division and availability of resources. | **`[TIER-C]`** | `doc_feat==5`: Expand blastula volume dynamically via synchronized mitotic wave rings |
| `F6` | **forma** (*shape*) | **ES:** La forma emerge de fuerzas, señales, geometría, división y movimiento celular.<br>**EN:** Form emerges from forces, signals, geometry, cell division and movement. | **`[TIER-C]`** | `doc_feat==6`: Pinch & buckle epithelial sheet via differential apical tension forces |
| `F7` | **desarrollo** (*development*) | **ES:** El desarrollo es un proceso dinámico que transforma una célula inicial en un organismo organizado.<br>**EN:** Development is a dynamic process that transforms an initial cell into an organized organism. | **`[TIER-C]`** | `doc_feat==7`: Animate full zygote -> morula -> gastrula -> organoid time-lapse morph |

##### `[TIER-D]` E16 `Z11` (`organisms`) -> Target `vid=2611` | Organismos / *Organisms* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Cuerpos capaces de alimentarse, defenderse y reproducirse* / *Bodies capable of feeding, defending themselves and reproducing* | **Evidence (ES / EN):** `Diversidad de cuerpos y estrategias` (*Diversity of bodies and strategies*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La evolución produjo organismos microscópicos y gigantes, móviles y sésiles, simples y altamente diferenciados. No existe una única dirección inevitable hacia mayor complejidad.
- **Spoken Macro Summary (EN):** Evolution produced microscopic and giant, mobile and sessile, simple and highly differentiated organisms. There is no single inevitable direction toward greater complexity.
- **Current Visual & Diagnosis:** `vid = 21` via `hero_doc_matter_mind.wgsl:712-718` (`l_mode=1`) -> Chloroplast thylakoid disc stacks + spinning 6-blade ATP synthase turbine. Routed via `11 % 5 = 1` -> `vid=21` (single Chloroplast organelle!). Narrates macroscopic organisms—bodies, feeding, defense, endoskeletons, muscles, navigation, biodiversity—concrete organismal anatomy completely missing.
- **3D Procedural Blueprint:** **Morphing Comparative Organismal Body-Plan Silhouette**: translucent articulated metazoan body housing internal segmented skeletal spine/carapace, contracting stripy muscle bundles, sensory head cluster, surrounding biodiversity fan.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **cuerpo** (*body*) | **ES:** Un cuerpo coordina límites, soporte, transporte, intercambio y reproducción.<br>**EN:** A body coordinates boundaries, support, transportation, exchange, and reproduction. | **`[TIER-D]`** | `doc_feat==0`: Render unified bilateral body envelope organizing internal functional compartments |
| `F1` | **alimentación** (*feeding*) | **ES:** Los organismos obtienen materia y energía mediante estrategias adaptadas a su ambiente.<br>**EN:** Organisms obtain matter and energy through strategies adapted to their environment. | **`[TIER-D]`** | `doc_feat==1`: Animate ingestive mouth/filter apparatus drawing suspended food particles inward |
| `F2` | **defensa** (*defense*) | **ES:** Barreras, toxinas, camuflaje, inmunidad y conducta reducen daños.<br>**EN:** Barriers, toxins, camouflage, immunity and behavior reduce damage. | **`[TIER-D]`** | `doc_feat==2`: Deploy hardened armored dorsal plates & camouflage shimmer ripples |
| `F3` | **reproducción** (*reproduction*) | **ES:** La reproducción transmite linajes, no garantiza supervivencia de cada individuo.<br>**EN:** Reproduction continues lineages; it does not guarantee every individual's survival. | **`[TIER-D]`** | `doc_feat==3`: Bud or release reproductive spore/egg clutch off mature organism |
| `F4` | **esqueleto** (*skeleton*) | **ES:** Estructuras internas o externas sostienen el cuerpo y permiten transmitir fuerzas.<br>**EN:** Internal or external structures support the body and allow forces to be transmitted. | **`[TIER-D]`** | `doc_feat==4`: Reveal luminous segmented internal vertebra/exoskeleton structural scaffolding |
| `F5` | **músculo** (*muscle*) | **ES:** Tejidos contráctiles convierten energía química en movimiento.<br>**EN:** Contractile tissues convert chemical energy into movement. | **`[TIER-D]`** | `doc_feat==5`: Contract glowing crimson-orange striated muscle bundles along skeletal levers |
| `F6` | **navegación** (*navigation*) | **ES:** Un organismo puede orientar su conducta mediante señales externas e internas.<br>**EN:** An organism can guide its behavior through external and internal signals. | **`[TIER-D]`** | `doc_feat==6`: Sweep forward-mounted sensory antennae vectoring body along light/chemical trail |
| `F7` | **diversidad** (*diversity*) | **ES:** Las formas actuales son ramas supervivientes de un árbol mucho más amplio de posibilidades.<br>**EN:** The current forms are surviving branches of a much broader tree of possibilities. | **`[TIER-D]`** | `doc_feat==7`: Fan radial carousel of 5 contrasting phyla silhouettes (radial, worm, arthropod, plant, vertebrate) |

##### `[TIER-D]` E16 `Z12` (`ecosystems`) -> Target `vid=2612` | Ecosistemas / *Ecosystems* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Productores + consumidores + descomponedores + ciclos* / *Producers + consumers + decomposers + cycles* | **Evidence (ES / EN):** `Redes vivas de materia, energía e interacción` (*Living networks of matter, energy and interaction*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Ningún organismo vive completamente aislado. Los ecosistemas conectan seres vivos, clima, agua, suelo y química mediante redes de alimentación, competencia, cooperación y reciclaje.
- **Spoken Macro Summary (EN):** No organism lives completely isolated. Ecosystems connect living beings, climate, water, soil and chemistry through networks of feeding, competition, cooperation and recycling.
- **Current Visual & Diagnosis:** `vid = 22` via `hero_doc_matter_mind.wgsl:719-726` (`l_mode=2`) -> Mitochondrion ellipsoid + sinusoidal inner cristae foldings. Routed via `12 % 5 = 2` -> `vid=22` (single Mitochondrion!). Ecosystems involve planetary biogeochemical cycles, trophic webs, producers/consumers/decomposers, and symbiosis—no visual counterpart exists.
- **3D Procedural Blueprint:** **Suspended Biospheric Trophic Web & Nutrient Terrarium**: sunlit emerald producer canopy over golden herbivore/predator mid-nodes and earthy decomposer root bed, tied by closed circulating carbon/nitrogen cycle tubes.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **productor** (*producer*) | **ES:** Los productores convierten energía externa y materia inorgánica en biomasa.<br>**EN:** Producers convert external energy and inorganic matter into biomass. | **`[TIER-D]`** | `doc_feat==0`: Illuminate sun-drenched chlorophyll autotroph canopy generating emerald biomass |
| `F1` | **consumidor** (*consumer*) | **ES:** Los consumidores obtienen materia y energía al alimentarse de otros organismos o sus productos.<br>**EN:** Consumers obtain matter and energy by feeding on other organisms or their products. | **`[TIER-D]`** | `doc_feat==1`: Stream upward trophic energy filaments linking producers to mobile consumer nodes |
| `F2` | **descomponedor** (*decomposer*) | **ES:** Los descomponedores transforman restos y liberan nutrientes reutilizables.<br>**EN:** Decomposers transform remains and release reusable nutrients. | **`[TIER-D]`** | `doc_feat==2`: Activate benthic fungal/bacterial decomposer halo recycling falling detritus |
| `F3` | **red trófica** (*food web*) | **ES:** La alimentación forma redes, no cadenas simples, porque cada especie suele tener múltiples relaciones.<br>**EN:** Feeding forms networks, not simple chains, because each species usually has multiple relationships. | **`[TIER-D]`** | `doc_feat==3`: Draw full interconnected 3D food-web mesh instead of linear vertical chain |
| `F4` | **simbiosis** (*symbiosis*) | **ES:** La convivencia estrecha puede beneficiar, perjudicar o afectar de manera distinta a cada participante.<br>**EN:** Close coexistence can benefit, harm or affect each participant differently. | **`[TIER-D]`** | `doc_feat==4`: Show tightly braided dual-species mutualist & parasitic halo orbits |
| `F5` | **competencia** (*competition*) | **ES:** Las especies compiten cuando dependen de recursos limitados compartidos.<br>**EN:** Species compete when they depend on shared limited resources. | **`[TIER-D]`** | `doc_feat==5`: Oscillate two competing species clusters vying over single bottleneck resource node |
| `F6` | **nicho** (*niche*) | **ES:** Un nicho reúne condiciones, recursos e interacciones que permiten persistir a una población.<br>**EN:** A niche brings together conditions, resources and interactions that allow a population to persist. | **`[TIER-D]`** | `doc_feat==6`: Highlight distinct ecological niche envelope slices (canopy, burrow, pelagic) |
| `F7` | **ciclo de nutrientes** (*nutrient cycle*) | **ES:** Los átomos circulan entre organismos, aire, agua, suelo y roca.<br>**EN:** Atoms circulate between organisms, air, water, soil and rock. | **`[TIER-D]`** | `doc_feat==7`: Spin continuous golden biogeochemical nutrient loop soil -> flora -> fauna -> soil |

##### `[TIER-D]` E16 `Z13` (`movement`) -> Target `vid=2613` | Movimiento / *Movement* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Detectar → decidir → generar fuerza → desplazarse* / *Detect → decide → generate force → move* | **Evidence (ES / EN):** `Interacción activa con un entorno cambiante` (*Active interaction with a changing environment*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Moverse permite buscar energía, escapar, explorar, reproducirse y modificar el entorno. También exige control, orientación, gasto energético y coordinación del cuerpo.
- **Spoken Macro Summary (EN):** Movement allows an organism to seek energy, escape, explore, reproduce, and modify its environment. It also requires control, orientation, energy expenditure, and bodily coordination.
- **Current Visual & Diagnosis:** `vid = 23` via `hero_doc_matter_mind.wgsl:727-750` (`l_mode=3`) -> 3D Phylogenetic Tree of Life (LUCA root -> 3 domain branches + canopy ring). Routed via `13 % 5 = 3` -> `vid=23` (static Phylogenetic Tree!). Movement requires kinematic actuation—cilia/flagella propulsion, muscle sarcomere contraction, skeletal levers, escape reflexes, chemotactic foraging—none present.
- **3D Procedural Blueprint:** **Biomechanical Locomotion & Chemotaxis Stage**: helical rotating bacterial/eukaryotic flagellum + articulated muscle-tendon-bone lever arm navigating toward glowing nutrient gradient while dodging threat.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **locomoción** (*locomotion*) | **ES:** La locomoción desplaza el organismo mediante cilios, flagelos, músculos u otros mecanismos.<br>**EN:** Locomotion moves the organism through cilia, flagella, muscles or other mechanisms. | **`[TIER-D]`** | `doc_feat==0`: Spin helical whip flagellum & coordinated metachronal cilia waves propelling capsule |
| `F1` | **músculo** (*muscle*) | **ES:** La contracción coordinada genera fuerzas internas que se transmiten al entorno.<br>**EN:** Coordinated contraction generates internal forces that are transmitted to the environment. | **`[TIER-D]`** | `doc_feat==1`: Slide interlocking actin-myosin sarcomere filaments shortening muscle belly |
| `F2` | **esqueleto** (*skeleton*) | **ES:** Un soporte rígido o hidrostático permite convertir contracciones en movimiento dirigido.<br>**EN:** A rigid or hydrostatic support allows contractions to be converted into directed movement. | **`[TIER-D]`** | `doc_feat==2`: Pivot rigid skeletal lever bar around fulcrum driven by contracting muscle chord |
| `F3` | **alimentación** (*feeding*) | **ES:** Moverse amplía el acceso a recursos, pero aumenta el consumo de energía.<br>**EN:** Moving expands access to resources, but increases energy consumption. | **`[TIER-D]`** | `doc_feat==3`: Sweep forward foraging arc sweeping sparse nutrient cloud at metabolic cost |
| `F4` | **huida** (*escape*) | **ES:** Respuestas rápidas pueden reducir el riesgo de depredación o daño.<br>**EN:** Quick responses can reduce the risk of predation or damage. | **`[TIER-D]`** | `doc_feat==4`: Fire instant explosive recoil vector jetting body clear of sudden crimson threat |
| `F5` | **búsqueda** (*search*) | **ES:** Patrones de exploración equilibran información, distancia y energía.<br>**EN:** Search patterns balance information gained, distance traveled, and energy used. | **`[TIER-D]`** | `doc_feat==5`: Trace intermittent run-and-tumble / Levy-walk exploratory foraging path |
| `F6` | **orientación** (*orientation*) | **ES:** Luz, gravedad, campos, sustancias y memoria pueden guiar una trayectoria.<br>**EN:** Light, gravity, fields, substances and memory can guide a trajectory. | **`[TIER-D]`** | `doc_feat==6`: Lock heading alignment onto ambient light rays, gravity vector & odor plume |
| `F7` | **conducta** (*behavior*) | **ES:** El movimiento observable es la salida de procesos sensoriales, internos y motores.<br>**EN:** Observable movement is the output of sensory, internal and motor processes. | **`[TIER-D]`** | `doc_feat==7`: Couple sensory input flash -> internal circuit -> motor output locomotion burst |

##### `[TIER-C]` E16 `Z14` (`senses`) -> Target `vid=2614` | Sentidos / *Senses* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Estímulo → receptor → señal → respuesta* / *Stimulus → receptor → signal → response* | **Evidence (ES / EN):** `Conversión biológica de estímulos en señales` (*Biological conversion of stimuli into signals*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Los sentidos no copian el mundo completo. Cada organismo detecta rangos limitados de energía y sustancias relevantes para su supervivencia, cuerpo y forma de vida.
- **Spoken Macro Summary (EN):** The senses do not copy the entire world. Each organism detects limited ranges of energy and substances relevant to its survival, body and way of life.
- **Current Visual & Diagnosis:** `vid = 24` via `hero_doc_matter_mind.wgsl:751-759` (`l_mode=4`) -> Myelinated axon shaft + saltatory action potential pulse ring. Routed via `14 % 5 = 4` -> `vid=24` (single Myelinated axon). While axon conduction transmits sensory signals, `Z14` exclusively describes 8 distinct biological **receptors** (eye, chemoreceptor, ear vibration, skin touch, thermoreceptor, vestibular, proprioceptor, viscera) with zero receptor geometry!
- **3D Procedural Blueprint:** **8-Modal Biological Sensory Transducer Ring** feeding central ascending nerve axon (`vid=24`): retinal cup, odorant lock pocket, vibrating stereocilia hair bundle, pacinian touch bulb, thermal channel, semicircular vestibular tubes, muscle spindle, and visceral pulse.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **luz** (*light*) | **ES:** Fotorreceptores responden a determinados rangos de radiación y permiten orientación o visión.<br>**EN:** Photoreceptors respond to certain ranges of radiation and allow orientation or vision. | **`[TIER-D]`** | `doc_feat==0`: Illuminate curved optic retinal cup with rod/cone photoreceptors catching photons |
| `F1` | **sustancia química** (*chemical substance*) | **ES:** Quimiorreceptores detectan moléculas relacionadas con alimento, peligro, pareja o estado interno.<br>**EN:** Chemoreceptors detect molecules related to food, danger, a partner, or internal state. | **`[TIER-D]`** | `doc_feat==1`: Snap odorant ligand droplets into star-shaped chemoreceptor binding clefts |
| `F2` | **vibración** (*vibration*) | **ES:** Vibraciones del aire, agua, suelo o cuerpo pueden revelar eventos distantes.<br>**EN:** Vibrations of the air, water, soil or body can reveal distant events. | **`[TIER-D]`** | `doc_feat==2`: Oscillate stepped stereocilia hair-cell bundle riding acoustic air/fluid waves |
| `F3` | **presión** (*pressure*) | **ES:** Mecanorreceptores responden a contacto, estiramiento, flujo y fuerza.<br>**EN:** Mechanoreceptors respond to touch, stretch, flow and force. | **`[TIER-D]`** | `doc_feat==3`: Compress concentric lamellar Pacinian corpuscle under downward force ripple |
| `F4` | **temperatura** (*temperature*) | **ES:** Termorreceptores detectan cambios que afectan metabolismo, daño y conducta.<br>**EN:** Thermoreceptors detect changes that affect metabolism, damage and behavior. | **`[TIER-D]`** | `doc_feat==4`: Trigger glowing red-blue TRP thermal ion-channel pore opening with heat wave |
| `F5` | **orientación** (*orientation*) | **ES:** Gravedad, aceleración y campos físicos pueden contribuir a mapas de dirección.<br>**EN:** Gravity, acceleration, and physical fields can contribute to direction maps. | **`[TIER-D]`** | `doc_feat==5`: Tilt three orthogonal fluid-filled vestibular semicircular canals sensing spin |
| `F6` | **propiocepción** (*proprioception*) | **ES:** Señales internas informan sobre posición, tensión y movimiento del cuerpo.<br>**EN:** Internal signals inform about position, tension and movement of the body. | **`[TIER-D]`** | `doc_feat==6`: Stretch helical annulospiral proprioceptor wrapped cleanly around muscle fiber |
| `F7` | **interocepción** (*interoception*) | **ES:** El organismo detecta hambre, oxígeno, presión y otros estados internos.<br>**EN:** The body detects hunger, oxygen, pressure and other internal states. | **`[TIER-D]`** | `doc_feat==7`: Pulse beating cardiac/visceral organ ring emitting internal oxygen & pressure waves |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP16-Z10-14)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP16-Z10-14` (Odyssey Act IV — Experience 16 Zones `Z10..Z14`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files (4+1 focused slice):
1. `crates/cosmos_render/src/doc_stages.rs` — route `experience_id == 16` `Z10..=Z14` directly to reserved `vid = 2610..=2614` (and populate `color_b = [zone_c.x, zone_c.y, zone_c.z, 1.0]` at `L739`), plus companion emitters ``3772` (`Z12` food-web trophic torus / abiotic biome atmosphere ring)` where specified.
2. `crates/cosmos_narration/src/resolver.rs` — enrich `SegmentVisualHint` / `DOMAIN_CUE_KEYWORDS` (`base_flags` / `cue_tags` / `telemetry`) for the Exp `16` `Z10..=Z14` spoken concepts.
3. `shaders/hero_doc_matter_mind.wgsl` — handle decoded `vid >= 2610 && vid <= 2614` inside `march_doc_bio_mind` with distinct 3D procedural SDF / volumetric structures AND `switch (doc_feat(inst))` sub-stage branching (`0..k`, `0xFF` macro overview) smoothly driven by `doc_cue(inst)` & `doc_weight(inst)` (`0.72..1.35x` HDR cap).
4. `crates/cosmos_render/src/universe_scene.rs` + `shaders/compute_universe.wgsl` — ensure `field_grid_tex` / particle compute mode matches each zone (Compute Mode `6` morphogenetic diffusion (`Z10..Z11`), trophic nutrient cycling (`Z12`), chemo/mechanosensory flux (`Z13..Z14`)).

Per-Zone & `doc_feat` Blueprints:
  - `Z10` `development` (`vid=2610`): **Embryonic Lineage Differentiation Tree & Morphogen Blastula**: central multipotent stem cluster radiating outward along Waddington-valley paths into distinct contractile, neural, and epithelial tissues sculpted by chemical gradients. Features: `F0` célula madre/stem cell, `F1` regulación génica/gene regulation, `F2` gradiente químico/chemical gradient, `F3` tejido/tissue, `F4` órgano/organ, `F5` crecimiento/growth, `F6` forma/shape, `F7` desarrollo/development.
  - `Z11` `organisms` (`vid=2611`): **Morphing Comparative Organismal Body-Plan Silhouette**: translucent articulated metazoan body housing internal segmented skeletal spine/carapace, contracting stripy muscle bundles, sensory head cluster, surrounding biodiversity fan. Features: `F0` cuerpo/body, `F1` alimentación/feeding, `F2` defensa/defense, `F3` reproducción/reproduction, `F4` esqueleto/skeleton, `F5` músculo/muscle, `F6` navegación/navigation, `F7` diversidad/diversity.
  - `Z12` `ecosystems` (`vid=2612`): **Suspended Biospheric Trophic Web & Nutrient Terrarium**: sunlit emerald producer canopy over golden herbivore/predator mid-nodes and earthy decomposer root bed, tied by closed circulating carbon/nitrogen cycle tubes. Features: `F0` productor/producer, `F1` consumidor/consumer, `F2` descomponedor/decomposer, `F3` red trófica/food web, `F4` simbiosis/symbiosis, `F5` competencia/competition, `F6` nicho/niche, `F7` ciclo de nutrientes/nutrient cycle.
  - `Z13` `movement` (`vid=2613`): **Biomechanical Locomotion & Chemotaxis Stage**: helical rotating bacterial/eukaryotic flagellum + articulated muscle-tendon-bone lever arm navigating toward glowing nutrient gradient while dodging threat. Features: `F0` locomoción/locomotion, `F1` músculo/muscle, `F2` esqueleto/skeleton, `F3` alimentación/feeding, `F4` huida/escape, `F5` búsqueda/search, `F6` orientación/orientation, `F7` conducta/behavior.
  - `Z14` `senses` (`vid=2614`): **8-Modal Biological Sensory Transducer Ring** feeding central ascending nerve axon (`vid=24`): retinal cup, odorant lock pocket, vibrating stereocilia hair bundle, pacinian touch bulb, thermal channel, semicircular vestibular tubes, muscle spindle, and visceral pulse. Features: `F0` luz/light, `F1` sustancia química/chemical substance, `F2` vibración/vibration, `F3` presión/pressure, `F4` temperatura/temperature, `F5` orientación/orientation, `F6` propiocepción/proprioception, `F7` interocepción/interoception.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; keep volumetric shells tightly bounded inside hero sphere (`r <= 0.96`, early-exit `trans < 0.03`) with zero full-screen fog wash.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps for new documentary hero shells, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

<a id="pkg-act4-exp16-z15-18"></a>
### PKG-ACT4-EXP16-Z15-18: Exp `16` — Nervous Coordination, Central Brains, Biological Perception & Sentience (`Z15..Z18`)

- **Phase & Priority:** `Phase 1/2` · **P1 — High (Eliminates DNA/Chloroplast/Mitochondrion/Tree errors across neurobiology & bridges into Exp 17)** · **Scope:** `4` Zones (`Z15..Z18`), `33` Features (`doc_feat` `0..k`)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:56-60, 731-744`](../../crates/cosmos_render/src/doc_stages.rs#L56-L60) & [`shaders/hero_doc_matter_mind.wgsl:610, 687-759`](../../shaders/hero_doc_matter_mind.wgsl#L687-L759); `vid`: `doc_stage_variant_id(16)` -> `0->23, 4->20, _->20+(sc%5)` (`20, 21, 22, 23`; catalog `scene = null`); `doc_feat(inst)` unused in WGSL; `color_b` zone accent unused (`doc_stages.rs:731-744`). **Card Tiers — Zones:** `0A / 0B / 4C / 0D` · **Features:** `0A / 0B / 28C / 5D`.
- **Compute & Plumbing Focus:** Switch Exp `16 Z15..Z18` compute mode smoothly from `mode=6` cellular BZ into `mode=7` Connectome Firing (`universe_scene.rs:320` + `compute_universe.wgsl:238-253`). Extra files: `crates/cosmos_render/src/universe_scene.rs` (`L320` zone-aware compute transition into neural firing).

#### Reserved Target IDs

| Zone | Slug | ES / EN Title | Current `vid` | Reserved Target `vid` | Companion IDs | `submode` & `doc_feat` Range |
|---|---|---|---:|---:|---|---|
| `Z15` | `nervous` | Sistemas nerviosos / *nervous systems* | `20` (`[TIER-C]`) | **`2615`** | `3773` (peripheral sensory-motor reflex loop ring for `Z15`) | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z16` | `brains` | Cerebros y aprendizaje / *Brains and learning* | `21` (`[TIER-C]`) | **`2616`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z17` | `perception` | Percepción / *Perception* | `22` (`[TIER-C]`) | **`2617`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z18` | `consciousness` | La experiencia consciente / *The conscious experience* | `23` (`[TIER-C]`) | **`2618`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..8` |

#### Spoken Transcript Summary & 3D Visual Blueprint

Key deliverables for this Pick-Up Card:
- `vid=2615` (`Z15 nervous`): Replace B-DNA (`vid=20`) with Synaptic Relay & Spinal Reflex Arch (dendritic spines, neurotransmitter cleft vesicles, myelinated tracts; `doc_feat 0..7`).
- `vid=2616` (`Z16 brains`): Replace **Chloroplast (`vid=21`)** with Comparative Nervous Topology -> Primate Prefrontal-Neocortex Silhouette (nerve net -> ganglia -> folded gyri; `doc_feat 0..7`).
- `vid=2617` (`Z17 perception`): Replace **Mitochondrion (`vid=22`)** with Thalamo-Cortical Predictive Decoding Loop (bottom-up sensory signals colliding with top-down expectation pulses; `doc_feat 0..7`).
- `vid=2618` (`Z18 consciousness`): Replace Tree of Life (`vid=23`) with Global Brain Ignition Portal framing `Z18` as the evolutionary bridge directly into `Exp 17 Z00` (`doc_feat 0..7`).

##### `[TIER-C]` E16 `Z15` (`nervous`) -> Target `vid=2615` | Sistemas nerviosos / *nervous systems* (`0A / 0B / 7C / 1D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Neuronas + impulsos + sinapsis + circuitos* / *Neurons + impulses + synapses + circuits* | **Evidence (ES / EN):** `Comunicación rápida dentro del organismo` (*Rapid communication within the body*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Los sistemas nerviosos integran señales, coordinan músculos y seleccionan respuestas. Pueden ser redes distribuidas, cordones, ganglios o estructuras centralizadas.
- **Spoken Macro Summary (EN):** Nervous systems integrate signals, coordinate muscles, and select responses. They can be distributed networks, cords, ganglia or centralized structures.
- **Current Visual & Diagnosis:** `vid = 20` via `hero_doc_matter_mind.wgsl:690-711` (`l_mode=0`) -> B-DNA double helix + base-pair rungs inside faint `r=0.68` spherical cell. Routed via `15 % 5 = 0` -> `vid=20` (B-DNA helix!). Textbook modulo collision: Nervous Systems (`Z15`) gets B-DNA while `vid=24` Myelinated Axon sits one slot earlier on `Z14`, and `mode=7` connectome particles (`universe_scene.rs:320`) aren't even enabled until Exp 17!
- **3D Procedural Blueprint:** **Multi-Neuron Connectome & Synaptic Cleft (`vid=24+`)**: branched dendritic soma tree + myelinated axon saltatory jump + zoomed vesicular synaptic cleft + reflexive spinal arc + centralized ganglion cluster; switch particles locally or upgrade Exp 16 `mode=6`.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **neurona** (*neuron*) | **ES:** Una neurona recibe, integra y transmite señales mediante cambios eléctricos y químicos.<br>**EN:** A neuron receives, integrates and transmits signals through electrical and chemical changes. | **`[TIER-C]`** | `doc_feat==0`: Render star-shaped soma + branching dendritic arbor + single output axon |
| `F1` | **impulso eléctrico** (*electrical impulse*) | **ES:** El potencial de acción propaga una señal regenerativa a lo largo de la membrana.<br>**EN:** The action potential propagates a regenerative signal across the membrane. | **`[TIER-C]`** | `doc_feat==1`: Propagate regenerative glowing voltage ring leaping smoothly across Nodes of Ranvier |
| `F2` | **sinapsis** (*synapse*) | **ES:** Una sinapsis modifica la actividad de otra célula mediante neurotransmisores o conexión eléctrica.<br>**EN:** A synapse modifies the activity of another cell through neurotransmitters or electrical connection. | **`[TIER-D]`** | `doc_feat==2`: Zoom cleft gap fusing neurotransmitter vesicles onto postsynaptic receptors |
| `F3` | **reflejo** (*reflex*) | **ES:** Un circuito breve convierte un estímulo en respuesta rápida sin requerir deliberación consciente.<br>**EN:** A short neural circuit converts a stimulus into a rapid response without requiring conscious deliberation. | **`[TIER-C]`** | `doc_feat==3`: Fire tight high-speed 2-neuron U-loop reflex bypassing central ganglion |
| `F4` | **integración sensorial** (*sensory integration*) | **ES:** Circuitos combinan señales de distintas fuentes y momentos.<br>**EN:** Circuits combine signals from different sources and times. | **`[TIER-C]`** | `doc_feat==4`: Converge 3 differently colored modality inputs onto summing dendritic spine |
| `F5` | **control motor** (*motor control*) | **ES:** La actividad neural coordina fuerza, secuencia y corrección del movimiento.<br>**EN:** Neural activity coordinates the force, sequence, and correction of movement. | **`[TIER-C]`** | `doc_feat==5`: Send coordinated rhythmic pulse train down motor axon to contractile target |
| `F6` | **red distribuida** (*distributed network*) | **ES:** Un sistema nervioso puede funcionar sin un único centro dominante.<br>**EN:** A nervous system can function without a single dominant center. | **`[TIER-C]`** | `doc_feat==6`: Spread uniform decentralized hydra-like nerve net evenly across shell |
| `F7` | **centralización** (*centralization*) | **ES:** Concentrar procesamiento cerca de órganos sensoriales puede acelerar coordinación y predicción.<br>**EN:** Concentrating processing near sensory organs can speed coordination and prediction. | **`[TIER-C]`** | `doc_feat==7`: Condense dense anterior cephalized ganglion cluster toward sensory pole |

##### `[TIER-C]` E16 `Z16` (`brains`) -> Target `vid=2616` | Cerebros y aprendizaje / *Brains and learning* (`0A / 0B / 7C / 1D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Memoria + predicción + recompensa + plasticidad* / *Memory + prediction + reward + plasticity* | **Evidence (ES / EN):** `Procesamiento adaptable basado en experiencia` (*Experience-based adaptive processing*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** El aprendizaje permite que la experiencia cambie conductas futuras. Los cerebros combinan respuestas heredadas con plasticidad, memoria, predicción y selección de acciones.
- **Spoken Macro Summary (EN):** Learning allows experience to change future behaviors. Brains combine inherited responses with plasticity, memory, prediction, and action selection.
- **Current Visual & Diagnosis:** `vid = 21` via `hero_doc_matter_mind.wgsl:712-718` (`l_mode=1`) -> Chloroplast thylakoid disc stacks + spinning 6-blade ATP synthase turbine. Routed via `16 % 5 = 1` -> `vid=21` (Chloroplast!). Explaining animal brains, synaptic plasticity, prediction, and reward with plant chlorophyll discs is completely off-topic—even though `mm_brain_sdf` already exists in the exact same WGSL file!
- **3D Procedural Blueprint:** Render **Cross-Species Compact Brain SDF** (simplified vertebrate/cortical precursor of `mm_brain_sdf`) with glowing synaptic weight strengthening (LTP), dopaminergic reward arc, and predictive error spark.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **memoria** (*memory*) | **ES:** La memoria conserva efectos del pasado que pueden modificar decisiones posteriores.<br>**EN:** Memory preserves effects from the past that can modify subsequent decisions. | **`[TIER-C]`** | `doc_feat==0`: Retain fading phosphorescent echo along recently traversed neural circuit |
| `F1` | **plasticidad** (*plasticity*) | **ES:** Las conexiones y propiedades neuronales cambian con actividad, desarrollo y experiencia.<br>**EN:** Neural connections and properties change with activity, development, and experience. | **`[TIER-C]`** | `doc_feat==1`: Thicken & brighten repeated synaptic capsule bridges over time (Hebbian LTP) |
| `F2` | **predicción** (*prediction*) | **ES:** Un cerebro puede anticipar consecuencias y reducir incertidumbre antes de actuar.<br>**EN:** A brain can anticipate consequences and reduce uncertainty before acting. | **`[TIER-C]`** | `doc_feat==2`: Project translucent anticipatory ghost trajectory ahead of moving stimulus |
| `F3` | **recompensa** (*reward*) | **ES:** Sistemas de valoración refuerzan acciones asociadas con resultados relevantes.<br>**EN:** Valuation systems reinforce actions associated with relevant outcomes. | **`[TIER-D]`** | `doc_feat==3`: Flood ventral striatum/midbrain core with golden dopaminergic reward halo |
| `F4` | **atención** (*attention*) | **ES:** La atención prioriza parte de la información disponible y regula procesamiento y conducta.<br>**EN:** Attention prioritizes part of the available information and regulates processing and behavior. | **`[TIER-C]`** | `doc_feat==4`: Spotlight one sensory input channel while dimming competing background fibers |
| `F5` | **decisión** (*decision*) | **ES:** Decidir implica comparar estados, necesidades, riesgos, hábitos y posibilidades.<br>**EN:** Deciding involves comparing states, needs, risks, habits and possibilities. | **`[TIER-C]`** | `doc_feat==5`: Weigh two diverging output loops toggling smoothly at bifurcation hub |
| `F6` | **instinto** (*instinct*) | **ES:** Un instinto es una predisposición evolucionada, pero puede interactuar con aprendizaje.<br>**EN:** An instinct is an evolved predisposition, but it can interact with learning. | **`[TIER-C]`** | `doc_feat==6`: Highlight fixed hard-wired deep brainstem reflex core beneath plastic cortex |
| `F7` | **flexibilidad** (*flexibility*) | **ES:** La conducta flexible permite responder a situaciones no idénticas a las del pasado.<br>**EN:** Flexible behavior allows us to respond to situations that are not identical to those of the past. | **`[TIER-C]`** | `doc_feat==7`: Rewire sparse cortical shortcut paths dynamically when goal location moves |

##### `[TIER-C]` E16 `Z17` (`perception`) -> Target `vid=2617` | Percepción / *Perception* (`0A / 0B / 6C / 2D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Señales incompletas + contexto + memoria → experiencia del mundo* / *Incomplete cues + context + memory → experience of the world* | **Evidence (ES / EN):** `Representaciones construidas por sistemas nerviosos` (*Representations constructed by nervous systems*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** No experimentamos una copia directa del exterior. El cerebro integra señales ambiguas con contexto, expectativas y estado corporal para construir una interpretación útil.
- **Spoken Macro Summary (EN):** We do not experience a direct copy of the outside. The brain integrates ambiguous signals with context, expectations, and bodily state to construct a useful interpretation.
- **Current Visual & Diagnosis:** `vid = 22` via `hero_doc_matter_mind.wgsl:719-726` (`l_mode=2`) -> Mitochondrion ellipsoid + sinusoidal inner cristae foldings. Routed via `17 % 5 = 2` -> `vid=22` (Mitochondrion!). Perception builds color, pitch, object boundaries, motion, and spatial maps in sensory cortex—completely unrepresented by a mitochondrial cristae sac.
- **3D Procedural Blueprint:** **Multi-Plane Cortical Sensory Reconstruction Stack**: raw fragmented pixel/wave input plane transforming upward through edge-contour extraction, color-constancy, motion-flow & cognitive spatial map hologram inside cortical canopy.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **color** (*color*) | **ES:** El color es una experiencia construida a partir de luz, receptores, contexto y procesamiento neural.<br>**EN:** Color is an experience constructed from light, receptors, context and neural processing. | **`[TIER-C]`** | `doc_feat==0`: Decompose white beam through opponent-process R/G & B/Y cortical channels |
| `F1` | **sonido** (*sound*) | **ES:** El sonido percibido organiza vibraciones en tono, timbre, ritmo, fuente y significado.<br>**EN:** Perceived sound organizes vibrations into pitch, timbre, rhythm, source, and meaning. | **`[TIER-C]`** | `doc_feat==1`: Group scattered acoustic pulses into crisp harmonic pitch & directional arc |
| `F2` | **forma** (*shape*) | **ES:** El sistema visual infiere bordes, superficies y objetos a partir de patrones incompletos.<br>**EN:** The visual system infers edges, surfaces, and objects from incomplete patterns. | **`[TIER-C]`** | `doc_feat==2`: Snap broken illusory Kanizsa-style fragments into complete solid triangle/cube |
| `F3` | **movimiento** (*movement*) | **ES:** La percepción de movimiento combina cambios temporales, seguimiento y predicción.<br>**EN:** Motion perception combines temporal changes, tracking, and prediction. | **`[TIER-C]`** | `doc_feat==3`: Trace coherent apparent-motion streak across sequentially strobed spatial dots |
| `F4` | **mapa espacial** (*space map*) | **ES:** El cerebro integra señales para representar posiciones del cuerpo, objetos y rutas.<br>**EN:** The brain integrates signals to represent body positions, objects, and routes. | **`[TIER-D]`** | `doc_feat==4`: Project floating top-down allocentric spatial grid map with self-beacon |
| `F5` | **reconocimiento** (*recognition*) | **ES:** Reconocer relaciona la entrada actual con categorías y recuerdos aprendidos.<br>**EN:** Recognition relates current input to learned categories and memories. | **`[TIER-C]`** | `doc_feat==5`: Match incoming noisy point cloud onto crisp wireframe template memory overlay |
| `F6` | **cuerpo** (*body*) | **ES:** La experiencia del cuerpo combina tacto, visión, equilibrio, propiocepción e interocepción.<br>**EN:** The experience of the body combines touch, vision, balance, proprioception and interoception. | **`[TIER-D]`** | `doc_feat==6`: Align multimodal touch/balance/vision shells around unified body-schema torso |
| `F7` | **interpretación** (*interpretation*) | **ES:** ¿Experimentamos el mundo directamente o una interpretación construida por el cerebro?<br>**EN:** Do we experience the world directly or an interpretation constructed by the brain? | **`[TIER-C]`** | `doc_feat==7`: Morph ambiguous bistable silhouette back and forth with subtle top-down pulse |

##### `[TIER-C]` E16 `Z18` (`consciousness`) -> Target `vid=2618` | La experiencia consciente / *The conscious experience* (`0A / 0B / 8C / 1D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Actividad cerebral → experiencia subjetiva aún no explicada por completo* / *Brain activity → subjective experience not yet fully explained* | **Evidence (ES / EN):** `Frontera científica abierta` (*Open scientific frontier*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La evolución produjo organismos capaces de sentir, aprender y construir modelos de su entorno. Todavía no comprendemos por completo cómo la actividad física de un cerebro produce experiencia subjetiva.
- **Spoken Macro Summary (EN):** Evolution produced organisms capable of sensing, learning, and building models of their environment. We still do not fully understand how the physical activity of a brain produces subjective experience.
- **Current Visual & Diagnosis:** `vid = 23` via `hero_doc_matter_mind.wgsl:727-750` (`l_mode=3`) -> 3D Phylogenetic Tree of Life (LUCA root -> 3 domain branches + canopy ring). Routed via `18 % 5 = 3` -> `vid=23` (Phylogenetic Tree of Life!). Exp 16's climax bridges straight into Exp 17 (`La experiencia consciente`), yet `18 % 5 = 3` replaces consciousness with the Tree of Life while `vid=34` global workspace brain already exists!
- **3D Procedural Blueprint:** **Global Neuronal Workspace Pre-Cathedral**: translucent brain envelope illuminating wide fronto-parietal ignition, recursive self-model inner mirror shell, and cross-species comparative brain constellation leading to `17_mind`.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **vigilia** (*wakefulness*) | **ES:** La vigilia describe un nivel de activación y capacidad de interacción, pero no agota el concepto de conciencia.<br>**EN:** Wakefulness describes a level of activation and capacity for interaction, but does not exhaust the concept of consciousness. | **`[TIER-C]`** | `doc_feat==0`: Contrast bright activated reticular/thalamic arousal core vs resting cortical mantle |
| `F1` | **experiencia subjetiva** (*subjective experience*) | **ES:** La conciencia incluye que exista algo que se siente desde la perspectiva del organismo.<br>**EN:** Consciousness includes there being something that is felt from the perspective of the organism. | **`[TIER-C]`** | `doc_feat==1`: Project inward first-person volumetric perspective cone from center of stage |
| `F2` | **atención** (*attention*) | **ES:** Atención y conciencia se relacionan, pero no son necesariamente idénticas.<br>**EN:** Attention and awareness are related, but are not necessarily identical. | **`[TIER-C]`** | `doc_feat==2`: Focus sharp narrow spotlight beam gliding independently over glowing workspace |
| `F3` | **modelo de sí** (*self-model*) | **ES:** Un sistema puede representar su cuerpo, historia, posibilidades y relación con otros.<br>**EN:** A system can represent its body, history, possibilities and relationship with others. | **`[TIER-C]`** | `doc_feat==3`: Nest miniature glowing inner self-schema sphere inside larger body/world ring |
| `F4` | **comunicación** (*communication*) | **ES:** Poder informar una experiencia ayuda a estudiarla, aunque la ausencia de lenguaje no demuestra ausencia de conciencia.<br>**EN:** Being able to report an experience helps to study it, although the absence of language does not demonstrate the absence of consciousness. | **`[TIER-C]`** | `doc_feat==4`: Link internal conscious halo to external symbolic emission antenna |
| `F5` | **conciencia animal** (*animal consciousness*) | **ES:** La evidencia sugiere capacidades conscientes en diversos animales, con grados y formas aún debatidos.<br>**EN:** Evidence suggests conscious capacities in diverse animals, with degrees and forms still debated. | **`[TIER-D]`** | `doc_feat==5`: Orbit comparative animal brain silhouettes (corvid, octopus, mammal) around hub |
| `F6` | **correlato neural** (*neural correlate*) | **ES:** Los correlatos neurales identifican actividad asociada con experiencia, pero asociación no equivale a explicación completa.<br>**EN:** Neural correlates identify activity associated with experience, but association does not equal complete explanation. | **`[TIER-C]`** | `doc_feat==6`: Couple flickering neural correlation clusters (NCC) along posterior/frontal loop |
| `F7` | **teorías** (*theories*) | **ES:** Existen teorías competidoras sobre integración, acceso global, predicción y orden superior; ninguna ha resuelto definitivamente el problema.<br>**EN:** There are competing theories of integration, global access, prediction, and higher order; none has definitively solved the problem. | **`[TIER-C]`** | `doc_feat==7`: Toggle 4 competing theory overlays (IIT grid, GNW ignition, predictive wave, HOT) |
| `F8` | **siguiente viaje** (*next trip*) | **ES:** Más allá: Espectro de la Mente, de la percepción al pensamiento, el lenguaje, la identidad y la cultura.<br>**EN:** Beyond: Spectrum of the Mind, from perception to thought, language, identity and culture. | **`[TIER-C]`** | `doc_feat==8`: Open glowing orchid cathedral portal forward to `17_mind` (`Z00`) |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP16-Z15-18)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP16-Z15-18` (Odyssey Act IV — Experience 16 Zones `Z15..Z18`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files (4+1 focused slice):
1. `crates/cosmos_render/src/doc_stages.rs` — route `experience_id == 16` `Z15..=Z18` directly to reserved `vid = 2615..=2618` (and populate `color_b = [zone_c.x, zone_c.y, zone_c.z, 1.0]` at `L739`), plus companion emitters ``3773` (peripheral sensory-motor reflex loop ring for `Z15`)` where specified.
2. `crates/cosmos_narration/src/resolver.rs` — enrich `SegmentVisualHint` / `DOMAIN_CUE_KEYWORDS` (`base_flags` / `cue_tags` / `telemetry`) for the Exp `16` `Z15..=Z18` spoken concepts.
3. `shaders/hero_doc_matter_mind.wgsl` — handle decoded `vid >= 2615 && vid <= 2618` inside `march_doc_bio_mind` with distinct 3D procedural SDF / volumetric structures AND `switch (doc_feat(inst))` sub-stage branching (`0..k`, `0xFF` macro overview) smoothly driven by `doc_cue(inst)` & `doc_weight(inst)` (`0.72..1.35x` HDR cap).
4. `crates/cosmos_render/src/universe_scene.rs` + `shaders/compute_universe.wgsl` — ensure `field_grid_tex` / particle compute mode matches each zone (Switch Exp `16 Z15..Z18` compute mode smoothly from `mode=6` cellular BZ into `mode=7` Connectome Firing (`universe_scene.rs:320` + `compute_universe.wgsl:238-253`)).

Per-Zone & `doc_feat` Blueprints:
  - `Z15` `nervous` (`vid=2615`): **Multi-Neuron Connectome & Synaptic Cleft (`vid=24+`)**: branched dendritic soma tree + myelinated axon saltatory jump + zoomed vesicular synaptic cleft + reflexive spinal arc + centralized ganglion cluster; switch particles locally or upgrade Exp 16 `mode=6`. Features: `F0` neurona/neuron, `F1` impulso eléctrico/electrical impulse, `F2` sinapsis/synapse, `F3` reflejo/reflex, `F4` integración sensorial/sensory integration, `F5` control motor/motor control, `F6` red distribuida/distributed network, `F7` centralización/centralization.
  - `Z16` `brains` (`vid=2616`): Render **Cross-Species Compact Brain SDF** (simplified vertebrate/cortical precursor of `mm_brain_sdf`) with glowing synaptic weight strengthening (LTP), dopaminergic reward arc, and predictive error spark. Features: `F0` memoria/memory, `F1` plasticidad/plasticity, `F2` predicción/prediction, `F3` recompensa/reward, `F4` atención/attention, `F5` decisión/decision, `F6` instinto/instinct, `F7` flexibilidad/flexibility.
  - `Z17` `perception` (`vid=2617`): **Multi-Plane Cortical Sensory Reconstruction Stack**: raw fragmented pixel/wave input plane transforming upward through edge-contour extraction, color-constancy, motion-flow & cognitive spatial map hologram inside cortical canopy. Features: `F0` color/color, `F1` sonido/sound, `F2` forma/shape, `F3` movimiento/movement, `F4` mapa espacial/space map, `F5` reconocimiento/recognition, `F6` cuerpo/body, `F7` interpretación/interpretation.
  - `Z18` `consciousness` (`vid=2618`): **Global Neuronal Workspace Pre-Cathedral**: translucent brain envelope illuminating wide fronto-parietal ignition, recursive self-model inner mirror shell, and cross-species comparative brain constellation leading to `17_mind`. Features: `F0` vigilia/wakefulness, `F1` experiencia subjetiva/subjective experience, `F2` atención/attention, `F3` modelo de sí/self-model, `F4` comunicación/communication, `F5` conciencia animal/animal consciousness, `F6` correlato neural/neural correlate, `F7` teorías/theories, `F8` siguiente viaje/next trip.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; keep volumetric shells tightly bounded inside hero sphere (`r <= 0.96`, early-exit `trans < 0.03`) with zero full-screen fog wash.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps for new documentary hero shells, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

---

## 4.3 Experience 17 (`17_mind`) — *Espectro de la Mente* (`18` Zones, `145` Features)

<a id="pkg-act4-exp17-z00-04"></a>
### PKG-ACT4-EXP17-Z00-04: Exp `17` — Consciousness, Sensation, Perception, Attention & Emotion (`Z00..Z04`)

- **Phase & Priority:** `Phase 2` · **P1 — High (Fixes `mm_brain_sdf` posterior bias + adds somatic torso, sensory receptor & limbic structures)** · **Scope:** `5` Zones (`Z00..Z04`), `40` Features (`doc_feat` `0..k`)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:61, 731-744`](../../crates/cosmos_render/src/doc_stages.rs#L61) & [`shaders/hero_doc_matter_mind.wgsl:556-590, 611, 760-809`](../../shaders/hero_doc_matter_mind.wgsl#L760-L809); `vid`: `doc_stage_variant_id(17)` -> `30 + (sc % 5)` (`30, 31, 32, 33, 34`; single shared `mm_brain_sdf`, catalog `scene = null`); `doc_feat(inst)` unused in WGSL; `color_b` zone accent unused (`doc_stages.rs:731-744`). **Card Tiers — Zones:** `0A / 4B / 0C / 1D` · **Features:** `0A / 29B / 0C / 11D`.
- **Compute & Plumbing Focus:** Compute Mode `7` Neural Connectome (`compute_universe.wgsl:238-253`), zone-modulated to brainstem/sensory/limbic circuits. Extra files: `crates/cosmos_render/src/universe_scene.rs` (`L625` orchid accent propagation to `color_b`), `shaders/hero_doc_matter_mind.wgsl` (`L556-L590, L760-L809`).

#### Reserved Target IDs

| Zone | Slug | ES / EN Title | Current `vid` | Reserved Target `vid` | Companion IDs | `submode` & `doc_feat` Range |
|---|---|---|---:|---:|---|---|
| `Z00` | `consciousness` | La experiencia consciente / *The conscious experience* | `30` (`[TIER-B]`) | **`2700`** | `3780` (`Z00` somatic human torso wireframe + `Z01` retina/cochlea/skin peripheral receptor cluster) | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z01` | `sensation` | Sensación / *Sensation* | `31` (`[TIER-D]`) | **`2701`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z02` | `perception` | Percepción / *Perception* | `32` (`[TIER-B]`) | **`2702`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z03` | `attention` | Atención / *Attention* | `33` (`[TIER-B]`) | **`2703`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z04` | `emotion` | Emoción / *Emotion* | `34` (`[TIER-B]`) | **`2704`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |

#### Spoken Transcript Summary & 3D Visual Blueprint

Key deliverables for this Pick-Up Card:
- `vid=2700` (`Z00 consciousness`): Switch from occipital-only `m_sc=0` to Global Neuronal Workspace ignition + brainstem reticular core + translucent somatic outline (`doc_feat 0..7`).
- `vid=2701` (`Z01 sensation`): Add peripheral receptor cluster (retina, cochlear spiral, mechanoreceptors) feeding lemniscal ascending tracts rather than prefrontal mask (`doc_feat 0..7`).
- `vid=2702` (`Z02 perception`): Render stereoscopic binocular disparity overlay + top-down predictive wireframes + bistable Gestalt flip (`doc_feat 0..7`).
- `vid=2703` (`Z03 attention`): Replace generic traveling z-wave with Dorsal/Ventral Fronto-Parietal Spotlight Beam focusing one cortical cluster while attenuating distractors (`doc_feat 0..7`).
- `vid=2704` (`Z04 emotion`): Sculpt bilateral Amygdala almonds, Ventral Striatum, Anterior Insula & Hypothalamic autonomic torso pulses (`doc_feat 0..7`).

##### `[TIER-B]` E17 `Z00` (`consciousness`) -> Target `vid=2700` | La experiencia consciente / *The conscious experience* (`0A / 7B / 0C / 1D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Vigilia + experiencia + cuerpo + atención + actividad neural* / *Wakefulness + experience + body + attention + neural activity* | **Evidence (ES / EN):** `Frontera científica abierta` (*Open scientific frontier*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La experiencia consciente es el punto de partida de este viaje, no una explicación terminada. Experimentar algo no es lo mismo que comprenderlo, recordarlo o expresarlo.
- **Spoken Macro Summary (EN):** Conscious experience is the starting point of this journey, not a finished explanation. Experiencing something is not the same as understanding it, remembering it, or expressing it.
- **Current Visual & Diagnosis:** `vid = 30` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=0`) -> `mm_brain_sdf` (2 hemispheres + cerebellum + callosum + hippocampi) w/ posterior occipital highlight (`p.z < -0.10`). Shares anatomical `mm_brain_sdf` + connectome `mode=7` (`[TIER-B]` domain match), but `0 % 5 = 0` (`m_sc=0`) wrongly highlights only posterior/occipital cortex (`p.z < -0.10`) instead of Global Neuronal Workspace `m_sc=4`; `doc_feat` is completely ignored.
- **3D Procedural Blueprint:** Keep `mm_brain_sdf` (`vid=30`), switch consciousness highlight from occipital-only to thalamo-fronto-parietal global ignition + brainstem arousal stalk (`reg == 3.0`), and map `doc_feat(0..7)` to arousal, first-person cone, attention decouple, body outline, and NCC clusters.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **vigilia** (*wakefulness*) | **ES:** La vigilia permite responder al entorno, pero estar despierto no describe por sí solo todo el contenido de la conciencia.<br>**EN:** Wakefulness allows us to respond to the environment, but being awake does not in itself describe the entire content of consciousness. | **`[TIER-B]`** | `doc_feat==0`: Ignite brainstem ascending reticular core (`reg==3`) + diffuse thalamocortical halo |
| `F1` | **experiencia subjetiva** (*subjective experience*) | **ES:** Una experiencia posee una perspectiva: existe algo que se siente para el organismo que la vive.<br>**EN:** An experience has a perspective: there is something that is felt for the organism that experiences it. | **`[TIER-B]`** | `doc_feat==1`: Project translucent first-person volumetric observation cone outward from prefrontal hub |
| `F2` | **conciencia sensorial** (*sensory awareness*) | **ES:** Colores, sonidos, tacto y dolor pueden formar parte del campo consciente sin estar plenamente comprendidos.<br>**EN:** Colors, sounds, touch and pain can be part of the conscious field without being fully understood. | **`[TIER-B]`** | `doc_feat==2`: Illuminate sensory occipital-temporal-parietal belt softly beneath conscious glaze |
| `F3` | **atención** (*attention*) | **ES:** Atención y conciencia se relacionan, pero una puede cambiar sin que la otra cambie de la misma manera.<br>**EN:** Attention and consciousness are related, but one can change without the other changing in the same way. | **`[TIER-B]`** | `doc_feat==3`: Sweep tight detached spotlight torus scanning across broader dim glowing cortex |
| `F4` | **conciencia corporal** (*body awareness*) | **ES:** La experiencia del cuerpo integra tacto, postura, equilibrio, movimiento e información de los órganos internos.<br>**EN:** The experience of the body integrates touch, posture, balance, movement and information from internal organs. | **`[TIER-D]`** | `doc_feat==4`: Wrap faint translucent standing human torso silhouette around brain for somatic feedback |
| `F5` | **estado interno** (*internal state*) | **ES:** Hambre, fatiga, respiración y activación corporal modifican lo que destaca y cómo se interpreta.<br>**EN:** Hunger, fatigue, breathing, and bodily activation modify what is highlighted and how it is interpreted. | **`[TIER-B]`** | `doc_feat==5`: Pulse autonomic insular/hypothalamic glow modulated by metabolic urgency waves |
| `F6` | **actividad neural** (*neural activity*) | **ES:** Se conocen correlatos neurales de la experiencia, pero correlación no equivale todavía a una explicación completa.<br>**EN:** Neural correlates of the experience are known, but correlation does not yet equal a complete explanation. | **`[TIER-B]`** | `doc_feat==6`: Flash synchronized gamma-like cortical+hippocampal NCC sparkle bursts without solving bind |
| `F7` | **pregunta abierta** (*open question*) | **ES:** Aún se debate cómo procesos físicos distribuidos producen una experiencia unificada y subjetiva.<br>**EN:** How distributed physical processes produce a unified, subjective experience is still debated. | **`[TIER-B]`** | `doc_feat==7`: Suspend shimmering fractured glass prism ring over cortex marking unresolved hard problem |

##### `[TIER-D]` E17 `Z01` (`sensation`) -> Target `vid=2701` | Sensación / *Sensation* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Evento físico o químico → receptor → señal biológica* / *Physical or chemical event → receptor → biological signal* | **Evidence (ES / EN):** `Transducción biológica ampliamente establecida` (*Widely established biological transduction*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Los sentidos no envían copias completas del mundo. Receptores especializados convierten ciertas formas de energía y sustancias en señales que el sistema nervioso puede procesar.
- **Spoken Macro Summary (EN):** The senses do not send complete copies of the world. Specialized receptors convert certain forms of energy and substances into signals that the nervous system can process.
- **Current Visual & Diagnosis:** `vid = 31` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=1`) -> `mm_brain_sdf` w/ anterior prefrontal highlight (`p.z > 0.12`). Routed via `1 % 5 = 1` -> `vid=31` (anterior prefrontal glow on brain SDF!). `Z01` explicitly explains peripheral **biological sensory transduction**—photoreceptors, ossicles/cochlea, cutaneous pressure, thermoreceptors, nociceptors, semicircular vestibular canals, proprioceptors, visceroceptors—with ZERO sensory organs anywhere in WGSL.
- **3D Procedural Blueprint:** **Peripheral Receptor Halo -> Thalamus Relay Stage**: arrange 8 compact procedural sensory organ transducers (optic eye cup, cochlear spiral, tactile skin disc, vestibular 3-axis orthogonal rings, visceral beat torus) feeding primary somatosensory/sensory cortices along glowing afferent tracks.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **luz** (*light*) | **ES:** Los fotorreceptores responden a rangos de radiación; el color percibido surge después, mediante comparación y contexto.<br>**EN:** Photoreceptors respond to ranges of radiation; the perceived color emerges later, through comparison and context. | **`[TIER-D]`** | `doc_feat==0`: Render tiny curved retina eye-cup catching photon rays and firing optic tract spike |
| `F1` | **sonido** (*sound*) | **ES:** Las vibraciones mueven estructuras del oído y se transforman en patrones eléctricos, no en pequeños sonidos dentro del cerebro.<br>**EN:** The vibrations move structures in the ear and become electrical patterns, not small sounds within the brain. | **`[TIER-D]`** | `doc_feat==1`: Render coiled logarithmic cochlear tube transforming acoustic ripple into auditory nerve pulse |
| `F2` | **presión** (*pressure*) | **ES:** Receptores de la piel y tejidos detectan deformación, contacto, vibración y estiramiento.<br>**EN:** Receptors in the skin and tissues detect deformation, contact, vibration and stretch. | **`[TIER-D]`** | `doc_feat==2`: Compress layered translucent dermal disc showing tactile mechanoreceptor depolarization |
| `F3` | **calor** (*heat*) | **ES:** Termorreceptores responden a cambios de temperatura dentro de intervalos biológicamente relevantes.<br>**EN:** Thermoreceptors respond to temperature changes within biologically relevant ranges. | **`[TIER-D]`** | `doc_feat==3`: Glow warm/cold dual-band cutaneous thermal sensor triggering spinothalamic spark |
| `F4` | **dolor** (*pain*) | **ES:** La nocicepción detecta amenaza o daño potencial; la experiencia de dolor también depende de contexto, atención y aprendizaje.<br>**EN:** Nociception detects threats or potential harm; the experience of pain also depends on context, attention, and learning. | **`[TIER-D]`** | `doc_feat==4`: Flash sharp red nociceptive alarm arc gated cleanly by prefrontal/PAG descending modulator |
| `F5` | **equilibrio** (*balance*) | **ES:** El sistema vestibular registra aceleraciones y orientación de la cabeza para estabilizar postura y mirada.<br>**EN:** The vestibular system registers accelerations and orientation of the head to stabilize posture and gaze. | **`[TIER-D]`** | `doc_feat==5`: Orbit three orthogonal translucent vestibular semicircular fluid loops rotating with head tilt |
| `F6` | **cuerpo** (*body*) | **ES:** La propiocepción informa sobre posición y tensión de músculos y articulaciones.<br>**EN:** Proprioception informs about position and tension of muscles and joints. | **`[TIER-D]`** | `doc_feat==6`: Highlight helical muscle-spindle tension coils projecting toward cerebellum (`reg==3`) |
| `F7` | **interocepción** (*interoception*) | **ES:** Señales del interior corporal contribuyen a hambre, sed, respiración, latido y estado emocional.<br>**EN:** Signals from inside the body contribute to hunger, thirst, breathing, heartbeat and emotional state. | **`[TIER-D]`** | `doc_feat==7`: Pulse ascending vagal/visceral trunk from faint silhouette torso heart/gut to insula |

##### `[TIER-B]` E17 `Z02` (`perception`) -> Target `vid=2702` | Percepción / *Perception* (`0A / 7B / 0C / 1D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Señales incompletas + contexto + predicción → interpretación* / *Incomplete signals + context + prediction → interpretation* | **Evidence (ES / EN):** `Construcción activa apoyada por amplia evidencia` (*Active construction supported by extensive evidence*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La percepción organiza señales ambiguas en formas, objetos, movimientos y escenas. Es una interpretación activa y útil, no una fotografía perfecta de la realidad.
- **Spoken Macro Summary (EN):** Perception organizes ambiguous signals into shapes, objects, movements, and scenes. It is an active and useful interpretation, not a perfect photograph of reality.
- **Current Visual & Diagnosis:** `vid = 32` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=2`) -> `mm_brain_sdf` w/ bilateral temporal lobe highlight (`reg == 2.0`). Routed via `2 % 5 = 2` -> `vid=32` (temporal lobe highlight on `mm_brain_sdf`). Ventral temporal object/face pathway matches `[TIER-B]`, yet misses primary visual occipital cascade (`m_sc=0`) and zero perceptual phenomena (edges, color constancy, optical depth, Rubin ambiguity) appear via `doc_feat`.
- **3D Procedural Blueprint:** Drive floating **Anterior Visual Construction Plane** in front of `mm_brain_sdf` tied to occipital->temporal dorsal/ventral streams per `doc_feat` (`0..7`).

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **bordes** (*edges*) | **ES:** El sistema visual compara regiones y cambios locales para inferir límites que no siempre coinciden con objetos reales.<br>**EN:** The visual system compares regions and local changes to infer boundaries that do not always coincide with real objects. | **`[TIER-B]`** | `doc_feat==0`: Extract crisp Sobel-like luminous boundary edges across noisy projected test disc |
| `F1` | **color** (*color*) | **ES:** La misma luz puede parecer distinta según iluminación, contraste, adaptación y expectativas.<br>**EN:** The same light can appear different depending on lighting, contrast, adaptation and expectations. | **`[TIER-B]`** | `doc_feat==1`: Shift warm vs cool ambient surround cube demonstrating invariant color constancy |
| `F2` | **movimiento** (*movement*) | **ES:** El cerebro integra cambios a través del tiempo y puede percibir movimiento incluso en imágenes estáticas secuenciales.<br>**EN:** The brain integrates changes over time and can perceive movement even in sequential static images. | **`[TIER-B]`** | `doc_feat==2`: Strobe sequential discrete phantom orbs fusing into smooth continuous motion ribbon |
| `F3` | **profundidad** (*depth*) | **ES:** Perspectiva, disparidad binocular, movimiento y experiencia previa ayudan a inferir distancia.<br>**EN:** Perspective, binocular disparity, motion, and prior experience help infer distance. | **`[TIER-B]`** | `doc_feat==3`: Fan converging binocular disparity rays from twin eye nodes onto depth-layered plane |
| `F4` | **ritmo** (*rhythm*) | **ES:** La mente agrupa eventos en pulsos y patrones temporales que no están contenidos en un instante aislado.<br>**EN:** The mind groups events into pulses and temporal patterns that are not contained in an isolated instant. | **`[TIER-B]`** | `doc_feat==4`: Beat temporal perceptual grouping pulses into symmetric 4-bar rhythmic arcs |
| `F5` | **objeto** (*object*) | **ES:** Reconocer un objeto exige separar figura y fondo, completar partes ocultas y mantener identidad pese a cambios de vista.<br>**EN:** Recognizing an object requires separating figure and background, completing hidden parts and maintaining identity despite changes in view. | **`[TIER-B]`** | `doc_feat==5`: Separate glowing wireframe foreground figure clearly out of textured noisy ground |
| `F6` | **rostro** (*face*) | **ES:** Los rostros reciben procesamiento especializado, pero el contexto todavía puede alterar identidad, emoción y dirección de la mirada.<br>**EN:** Faces receive specialized processing, but context can still alter identity, emotion, and gaze direction. | **`[TIER-D]`** | `doc_feat==6`: Render wireframe facial landmark topology lighting ventral fusiform temporal patch |
| `F7` | **ambigüedad** (*ambiguity*) | **ES:** Una misma figura puede admitir interpretaciones incompatibles; el contexto inclina cuál se vuelve dominante.<br>**EN:** The same figure can admit incompatible interpretations; the context tilts which becomes dominant. | **`[TIER-B]`** | `doc_feat==7`: Alternate Necker-cube / vase-faces contour weights slowly flipping ventral/dorsal lock |

##### `[TIER-B]` E17 `Z03` (`attention`) -> Target `vid=2703` | Atención / *Attention* (`0A / 8B / 0C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Muchos estímulos → selección → procesamiento prioritario* / *Many stimuli → selection → priority processing* | **Evidence (ES / EN):** `Selección limitada y dinámica` (*Limited and dynamic selection*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La capacidad de procesamiento es limitada. La atención vuelve algunas señales más nítidas mientras otras permanecen débiles, aunque sigan influyendo en la conducta.
- **Spoken Macro Summary (EN):** Processing capacity is limited. Attention makes some signals clearer while others remain weak, although they continue to influence behavior.
- **Current Visual & Diagnosis:** `vid = 33` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=3`) -> `mm_brain_sdf` w/ traveling cortical z-sine wave. Routed via `3 % 5 = 3` -> `vid=33` (traveling cortical wave on `mm_brain_sdf`). Moving wave provides a natural metaphor for dynamic attentional scanning (`[TIER-B]`), but `doc_feat` is ignored: competition, capture, multitasking split, and inattentional blindness lack visual sub-stages.
- **3D Procedural Blueprint:** Add a steerable **Conic Attentional Spotlight Beam + Peripheral Suppression Filter** around `mm_brain_sdf` and particle stream `mode=7`, responsive to `doc_feat(0..7)`.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **selección** (*selection*) | **ES:** Atender favorece cierta información sobre alternativas que compiten por procesamiento y acción.<br>**EN:** Attending favors certain information about alternatives that compete for processing and action. | **`[TIER-B]`** | `doc_feat==0`: Boost amplitude of one particle bundle while damping 4 competing input filaments |
| `F1` | **foco** (*focus*) | **ES:** El foco puede concentrarse en una posición, una característica, un objeto, una tarea o un pensamiento.<br>**EN:** The focus can be on a position, a feature, an object, a task, or a thought. | **`[TIER-B]`** | `doc_feat==1`: Tighten golden spotlight cone selectively onto spatial point vs broad cortical area |
| `F2` | **distracción** (*distraction*) | **ES:** Un estímulo novedoso, intenso o relevante puede capturar atención incluso contra la intención actual.<br>**EN:** A novel, intense, or relevant stimulus can capture attention even against current intention. | **`[TIER-B]`** | `doc_feat==2`: Pierce spotlight sideways toward sudden flashing red exogenous salience beacon |
| `F3` | **novedad** (*novelty*) | **ES:** Los cambios inesperados reciben prioridad porque pueden señalar oportunidades, errores o peligro.<br>**EN:** Unexpected changes receive priority because they can signal opportunities, errors, or danger. | **`[TIER-B]`** | `doc_feat==3`: Ignite bright cyan novelty prediction-mismatch pop when unexpected node swaps color |
| `F4` | **objetivo** (*target*) | **ES:** Las metas preparan al sistema para buscar unas señales e ignorar otras.<br>**EN:** Goals prepare the system to look for some signals and ignore others. | **`[TIER-B]`** | `doc_feat==4`: Highlight sustained prefrontal goal-template ring filtering out non-matching particle hues |
| `F5` | **atención dividida** (*divided attention*) | **ES:** Realizar varias tareas a la vez suele implicar alternar recursos y aceptar pérdidas de precisión o velocidad.<br>**EN:** Multitasking often means switching resources and accepting losses in accuracy or speed. | **`[TIER-B]`** | `doc_feat==5`: Rapidly ping-pong spotlight cone between two spaced targets leaving dim switching gaps |
| `F6` | **ceguera por inatención** (*inattentional blindness*) | **ES:** Un evento visible puede pasar inadvertido cuando la atención está comprometida con otra tarea.<br>**EN:** A visible event may go unnoticed when attention is engaged with another task. | **`[TIER-B]`** | `doc_feat==6`: Pass bright obvious decoy orb smoothly across dark periphery undetected by fixed cone |
| `F7` | **vigilancia** (*sustained attention*) | **ES:** Mantener atención durante largos periodos requiere esfuerzo y suele degradarse con fatiga y monotonía.<br>**EN:** Maintaining attention for long periods requires effort and is often degraded by fatigue and monotony. | **`[TIER-B]`** | `doc_feat==7`: Dim vigilance halo gradually over dwell time unless re-energized by novelty pulse |

##### `[TIER-B]` E17 `Z04` (`emotion`) -> Target `vid=2704` | Emoción / *Emotion* (`0A / 7B / 0C / 1D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Valoración + estado corporal + motivación + expresión* / *Appraisal + bodily state + motivation + expression* | **Evidence (ES / EN):** `Sistemas de valoración, cuerpo y acción` (*Valuation systems, body and action*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Las emociones no son enemigas de la razón. Coordinan cambios corporales, valoración, memoria, atención y tendencias de acción para señalar qué importa ahora.
- **Spoken Macro Summary (EN):** Emotions are not enemies of reason. They coordinate bodily changes, appraisal, memory, attention, and action tendencies to signal what matters now.
- **Current Visual & Diagnosis:** `vid = 34` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=4`) -> `mm_brain_sdf` w/ global workspace z-sweep wave + hippocampal glow. Routed via `4 % 5 = 4` -> `vid=34` (global cortical sweep + bilateral limbic hippocampi). Fits integrative mind/limbic arousal (`[TIER-B]`), yet omits explicit amygdala spheres, visceral vagal loop, facial/vocal expression, and reappraisal modulation across `doc_feat(0..7)`.
- **3D Procedural Blueprint:** Augment limbic core inside `mm_brain_sdf` with paired almond **Amygdala Nodes**, insula/autonomic chest-loop torus, and chromatic mood palette modulation via `doc_feat(0..7)`.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **alegría** (*joy*) | **ES:** La alegría puede reforzar exploración, vínculo y repetición de situaciones valoradas positivamente.<br>**EN:** Joy can reinforce exploration, bonding, and repetition of positively valued situations. | **`[TIER-B]`** | `doc_feat==0`: Flood dopaminergic nucleus-accumbens/cortical sweep with vibrant golden exploratory warmth |
| `F1` | **miedo** (*fear*) | **ES:** El miedo prioriza posibles amenazas, acelera aprendizaje defensivo y prepara huida, inmovilidad o protección.<br>**EN:** Fear prioritizes possible threats, accelerates defensive learning and prepares flight, immobility or protection. | **`[TIER-B]`** | `doc_feat==1`: Pulse acute crimson-amber bilateral amygdala nodes driving fast brainstem recoil stream |
| `F2` | **sorpresa** (*surprise*) | **ES:** La sorpresa marca una discrepancia entre lo esperado y lo ocurrido, orientando atención y actualización.<br>**EN:** Surprise marks a discrepancy between what was expected and what happened, guiding attention and updating. | **`[TIER-B]`** | `doc_feat==2`: Snap rapid cyan surprise ring linking unexpected sensory spike directly to frontoparietal reset |
| `F3` | **tristeza** (*sadness*) | **ES:** La tristeza puede acompañar pérdida, reducción de actividad y búsqueda de apoyo o reevaluación.<br>**EN:** Sadness may accompany loss, reduced activity, and seeking support or reappraisal. | **`[TIER-B]`** | `doc_feat==3`: Slow cortical particle velocity & shift tone into deep contemplative indigo trough |
| `F4` | **ira** (*anger*) | **ES:** La ira suele aparecer ante obstáculos, daño o injusticia percibida y prepara confrontación o corrección.<br>**EN:** Anger usually appears in the face of obstacles, harm, or perceived injustice and prepares for confrontation or correction. | **`[TIER-B]`** | `doc_feat==4`: Concentrate tense pressurized red-orange glow at anterior cingulate & prefrontal block |
| `F5` | **calma** (*calm*) | **ES:** La calma implica menor urgencia y puede ampliar la exploración, la regulación y la deliberación.<br>**EN:** Calm implies less urgency and can expand exploration, regulation, and deliberation. | **`[TIER-B]`** | `doc_feat==5`: Steady gentle breathing sine wave expanding cerulean parasympathetic clarity |
| `F6` | **expresión** (*expression*) | **ES:** Rostro, voz y postura comunican estados, aunque las expresiones varían con contexto, cultura y control voluntario.<br>**EN:** Face, voice, and posture communicate states, although expressions vary with context, culture, and voluntary control. | **`[TIER-D]`** | `doc_feat==6`: Add faint facial/postural kinematic wireframe ring reacting alongside internal limbic state |
| `F7` | **regulación** (*regulation*) | **ES:** Reinterpretar, respirar, cambiar atención o modificar la situación puede transformar una respuesta emocional.<br>**EN:** Reinterpreting, breathing, shifting attention, or modifying the situation can transform an emotional response. | **`[TIER-B]`** | `doc_feat==7`: Illuminate descending ventromedial prefrontal inhibitory beams calming glowing amygdala core |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP17-Z00-04)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP17-Z00-04` (Odyssey Act IV — Experience 17 Zones `Z00..Z04`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files (4+1 focused slice):
1. `crates/cosmos_render/src/doc_stages.rs` — route `experience_id == 17` `Z00..=Z04` directly to reserved `vid = 2700..=2704` (and populate `color_b = [zone_c.x, zone_c.y, zone_c.z, 1.0]` at `L739`), plus companion emitters ``3780` (`Z00` somatic human torso wireframe + `Z01` retina/cochlea/skin peripheral receptor cluster)` where specified.
2. `crates/cosmos_narration/src/resolver.rs` — enrich `SegmentVisualHint` / `DOMAIN_CUE_KEYWORDS` (`base_flags` / `cue_tags` / `telemetry`) for the Exp `17` `Z00..=Z04` spoken concepts.
3. `shaders/hero_doc_matter_mind.wgsl` — handle decoded `vid >= 2700 && vid <= 2704` inside `march_doc_bio_mind` with distinct 3D procedural SDF / volumetric structures AND `switch (doc_feat(inst))` sub-stage branching (`0..k`, `0xFF` macro overview) smoothly driven by `doc_cue(inst)` & `doc_weight(inst)` (`0.72..1.35x` HDR cap).
4. `crates/cosmos_render/src/universe_scene.rs` + `shaders/compute_universe.wgsl` — ensure `field_grid_tex` / particle compute mode matches each zone (Compute Mode `7` Neural Connectome (`compute_universe.wgsl:238-253`), zone-modulated to brainstem/sensory/limbic circuits).

Per-Zone & `doc_feat` Blueprints:
  - `Z00` `consciousness` (`vid=2700`): Keep `mm_brain_sdf` (`vid=30`), switch consciousness highlight from occipital-only to thalamo-fronto-parietal global ignition + brainstem arousal stalk (`reg == 3.0`), and map `doc_feat(0..7)` to arousal, first-person cone, attention decouple, body outline, and NCC clusters. Features: `F0` vigilia/wakefulness, `F1` experiencia subjetiva/subjective experience, `F2` conciencia sensorial/sensory awareness, `F3` atención/attention, `F4` conciencia corporal/body awareness, `F5` estado interno/internal state, `F6` actividad neural/neural activity, `F7` pregunta abierta/open question.
  - `Z01` `sensation` (`vid=2701`): **Peripheral Receptor Halo -> Thalamus Relay Stage**: arrange 8 compact procedural sensory organ transducers (optic eye cup, cochlear spiral, tactile skin disc, vestibular 3-axis orthogonal rings, visceral beat torus) feeding primary somatosensory/sensory cortices along glowing afferent tracks. Features: `F0` luz/light, `F1` sonido/sound, `F2` presión/pressure, `F3` calor/heat, `F4` dolor/pain, `F5` equilibrio/balance, `F6` cuerpo/body, `F7` interocepción/interoception.
  - `Z02` `perception` (`vid=2702`): Drive floating **Anterior Visual Construction Plane** in front of `mm_brain_sdf` tied to occipital->temporal dorsal/ventral streams per `doc_feat` (`0..7`). Features: `F0` bordes/edges, `F1` color/color, `F2` movimiento/movement, `F3` profundidad/depth, `F4` ritmo/rhythm, `F5` objeto/object, `F6` rostro/face, `F7` ambigüedad/ambiguity.
  - `Z03` `attention` (`vid=2703`): Add a steerable **Conic Attentional Spotlight Beam + Peripheral Suppression Filter** around `mm_brain_sdf` and particle stream `mode=7`, responsive to `doc_feat(0..7)`. Features: `F0` selección/selection, `F1` foco/focus, `F2` distracción/distraction, `F3` novedad/novelty, `F4` objetivo/target, `F5` atención dividida/divided attention, `F6` ceguera por inatención/inattentional blindness, `F7` vigilancia/sustained attention.
  - `Z04` `emotion` (`vid=2704`): Augment limbic core inside `mm_brain_sdf` with paired almond **Amygdala Nodes**, insula/autonomic chest-loop torus, and chromatic mood palette modulation via `doc_feat(0..7)`. Features: `F0` alegría/joy, `F1` miedo/fear, `F2` sorpresa/surprise, `F3` tristeza/sadness, `F4` ira/anger, `F5` calma/calm, `F6` expresión/expression, `F7` regulación/regulation.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; keep volumetric shells tightly bounded inside hero sphere (`r <= 0.96`, early-exit `trans < 0.03`) with zero full-screen fog wash.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps for new documentary hero shells, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

<a id="pkg-act4-exp17-z05-08"></a>
### PKG-ACT4-EXP17-Z05-08: Exp `17` — Memory Consolidation, Learning, Concepts & Symbolic Language (`Z05..Z08`)

- **Phase & Priority:** `Phase 2` · **P2 — Medium-High (Replaces occipital/temporal swaps with hippocampus-neocortex replay, LTP spines & language tracts)** · **Scope:** `4` Zones (`Z05..Z08`), `32` Features (`doc_feat` `0..k`)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:61, 731-744`](../../crates/cosmos_render/src/doc_stages.rs#L61) & [`shaders/hero_doc_matter_mind.wgsl:556-590, 611, 760-809`](../../shaders/hero_doc_matter_mind.wgsl#L760-L809); `vid`: `doc_stage_variant_id(17)` -> `30 + (sc % 5)` (`30, 31, 32, 33`; single shared `mm_brain_sdf`, catalog `scene = null`); `doc_feat(inst)` unused in WGSL; `color_b` zone accent unused (`doc_stages.rs:731-744`). **Card Tiers — Zones:** `0A / 4B / 0C / 0D` · **Features:** `0A / 30B / 0C / 2D`.
- **Compute & Plumbing Focus:** Compute Mode `7` hippocampal sharp-wave ripple arcs (`Z05`) & Hebbian co-firing synaptic sparks (`Z06`). Extra files: `shaders/hero_doc_matter_mind.wgsl` (`L580-L586, L800` enhance bilateral `hippo_g` & cerebellar `cereb_g` masks).

#### Reserved Target IDs

| Zone | Slug | ES / EN Title | Current `vid` | Reserved Target `vid` | Companion IDs | `submode` & `doc_feat` Range |
|---|---|---|---:|---:|---|---|
| `Z05` | `memory` | Memoria / *Memory* | `30` (`[TIER-B]`) | **`2705`** | `3781` (`Z08` external vocal larynx + airborne sound-wave rings / written clay-to-code glyphs) | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z06` | `learning` | Aprendizaje / *Learning* | `31` (`[TIER-B]`) | **`2706`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z07` | `concepts` | Conceptos / *Concepts* | `32` (`[TIER-B]`) | **`2707`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z08` | `language` | Lenguaje / *Language* | `33` (`[TIER-B]`) | **`2708`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |

#### Spoken Transcript Summary & 3D Visual Blueprint

Key deliverables for this Pick-Up Card:
- `vid=2705` (`Z05 memory`): Center bilaterally on Hippocampal Arcs (`hippo_g`), Working-Memory Prefrontal Loops & Cerebellar Procedural Traces (`doc_feat 0..7`).
- `vid=2706` (`Z06 learning`): Zoom onto cortical synaptic dendritic spines strengthening under LTP co-activation + dopamine RPE reward wave + myelin thickening (`doc_feat 0..7`).
- `vid=2707` (`Z07 concepts`): Render Hierarchical Semantic Concept Constellation anchored in anterior temporal hub linking categories, metaphors & shared symbols (`doc_feat 0..7`).
- `vid=2708` (`Z08 language`): Illuminate left-lateralized Broca/Wernicke nodes connected by arched Arcuate Fasciculus ribbon + external vocal wave & cuneiform/phonetic glyphs (`doc_feat 0..7`).

##### `[TIER-B]` E17 `Z05` (`memory`) -> Target `vid=2705` | Memoria / *Memory* (`0A / 8B / 0C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Codificación + conservación + recuperación + reconstrucción* / *Encoding + storage + retrieval + reconstruction* | **Evidence (ES / EN):** `Múltiples sistemas reconstructivos` (*Multiple reconstructive systems*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La memoria permite que el pasado modifique el presente, pero recordar no significa reproducir una grabación exacta. Cada recuperación reconstruye con pistas, conocimiento y estado actual.
- **Spoken Macro Summary (EN):** Memory allows the past to shape the present, but remembering does not reproduce an exact recording. Each retrieval reconstructs the memory using cues, knowledge, and the current state.
- **Current Visual & Diagnosis:** `vid = 30` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=0`) -> `mm_brain_sdf` (2 hemispheres + cerebellum + callosum + hippocampi) w/ posterior occipital highlight (`p.z < -0.10`). Routed via `5 % 5 = 0` -> `vid=30` (posterior occipital cortex!). Fortunately `mm_brain_sdf` always renders glowing bilateral **hippocampi** and **cerebellum** (`[TIER-B]` domain match), yet occipital-only `m_sc=0` is anatomically misaligned for memory, and `doc_feat` ignores working/episodic/semantic/procedural/sleep consolidation.
- **3D Procedural Blueprint:** Map `doc_feat(0..7)` cleanly onto existing anatomical regions of `mm_brain_sdf`: `f0` DLPFC reverberating ring, `f1` hippocampus, `f2` distributed cortex, `f3` cerebellum (`reg==3.0`), `f5` decaying trace, `f6` rewoven trace, `f7` spindle consolidation burst.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **memoria de trabajo** (*working memory*) | **ES:** Mantiene y manipula una cantidad limitada de información durante segundos para comprender, calcular o planificar.<br>**EN:** Holds and manipulates a limited amount of information for seconds to understand, calculate, or plan. | **`[TIER-B]`** | `doc_feat==0`: Hold small luminous transient ring cycling in anterior prefrontal buffer (`p.z > 0.12`) |
| `F1` | **memoria episódica** (*episodic memory*) | **ES:** Relaciona acontecimientos con lugares, tiempos y una perspectiva autobiográfica.<br>**EN:** Relates events to places, times and an autobiographical perspective. | **`[TIER-B]`** | `doc_feat==1`: Highlight glowing bilateral C-shaped hippocampal arches (`hippo_g`) with spatial/time rings |
| `F2` | **memoria semántica** (*semantic memory*) | **ES:** Conserva conceptos, palabras y conocimientos sin exigir recordar cuándo se aprendieron.<br>**EN:** Preserves concepts, words and knowledge without requiring you to remember when they were learned. | **`[TIER-B]`** | `doc_feat==2`: Illuminate diffuse lattice of semantic concept links across temporal-parietal neocortex |
| `F3` | **memoria procedimental** (*procedural memory*) | **ES:** El entrenamiento transforma secuencias en habilidades que pueden ejecutarse sin describir cada paso.<br>**EN:** The training transforms sequences into skills that can be executed without describing each step. | **`[TIER-B]`** | `doc_feat==3`: Spotlight dense folded cerebellar (`reg==3.0`) & striatal motor-sequence loop |
| `F4` | **reconocimiento** (*recognition*) | **ES:** Una sensación de familiaridad puede aparecer antes de recuperar detalles sobre el origen de un recuerdo.<br>**EN:** A sense of familiarity may appear before recovering details about the origin of a memory. | **`[TIER-B]`** | `doc_feat==4`: Light rapid faint resonance echo ring upon probe stimulus matching cortical trace |
| `F5` | **olvido** (*forgetting*) | **ES:** Olvidar puede reflejar pérdida, interferencia, falta de pistas o una selección útil de información relevante.<br>**EN:** Forgetting may reflect loss, interference, lack of cues, or useful selection of relevant information. | **`[TIER-B]`** | `doc_feat==5`: Erode pale old synaptic trace arcs gradually amidst competing foreground noise |
| `F6` | **reconstrucción** (*reconstruction*) | **ES:** Nuevos datos, preguntas y expectativas pueden alterar la forma en que se reconstruye un episodio.<br>**EN:** New data, questions, and expectations can alter the way an episode is reconstructed. | **`[TIER-B]`** | `doc_feat==6`: Splice slightly shifted golden retrieval bead into reconstructed trace bundle |
| `F7` | **consolidación** (*consolidation*) | **ES:** Sueño, repetición y reactivación ayudan a estabilizar cambios, aunque ninguna memoria queda completamente inmóvil.<br>**EN:** Sleep, repetition, and reactivation help stabilize changes, although no memory remains completely still. | **`[TIER-B]`** | `doc_feat==7`: Pulse rhythmic slow-wave ripple transferring hippocampal bursts upward into neocortex |

##### `[TIER-B]` E17 `Z06` (`learning`) -> Target `vid=2706` | Aprendizaje / *Learning* (`0A / 7B / 0C / 1D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Experiencia + error de predicción + práctica → cambio* / *Experience + prediction error + practice → change* | **Evidence (ES / EN):** `Cambio duradero producido por experiencia` (*Lasting Change Produced by Experience*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Aprender significa cambiar de manera relativamente duradera por experiencia. El organismo detecta regularidades, consecuencias, modelos sociales y diferencias entre lo esperado y lo ocurrido.
- **Spoken Macro Summary (EN):** Learning means changing relatively lastingly through experience. The organism detects regularities, consequences, social models and differences between what was expected and what happened.
- **Current Visual & Diagnosis:** `vid = 31` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=1`) -> `mm_brain_sdf` w/ anterior prefrontal highlight (`p.z > 0.12`). Routed via `6 % 5 = 1` -> `vid=31` (anterior prefrontal highlight + connectome sparkles). Domain match (`[TIER-B]`), lacking `doc_feat` visualization of Hebbian spike-timing wiring, dopamine reinforcement, prediction-error discrepancy vectors, and social observation.
- **3D Procedural Blueprint:** Overlay magnified **Dynamic Plastic Synapse / Error-Vector Overlay** anchored cleanly to prefrontal cortex (`mm_brain_sdf` `vid=31`) via `doc_feat`.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **asociación** (*association*) | **ES:** Eventos que coinciden o se predicen mutuamente pueden quedar vinculados, aunque correlación no demuestra causalidad.<br>**EN:** Events that coincide or predict each other can be linked, although correlation does not prove causation. | **`[TIER-B]`** | `doc_feat==0`: Bridge two flashing cortical nodes with persistent glowing capsule on synchronous fire |
| `F1` | **refuerzo** (*reinforcement*) | **ES:** Las consecuencias modifican la probabilidad de repetir una acción en contextos semejantes.<br>**EN:** Consequences modify the probability of repeating an action in similar contexts. | **`[TIER-B]`** | `doc_feat==1`: Flood strengthened action-outcome arc with warm golden dopamine reward wave |
| `F2` | **imitación** (*imitation*) | **ES:** Observar a otros permite adquirir conductas sin descubrir cada paso mediante ensayo directo.<br>**EN:** Observing others allows behaviors to be learned without discovering every step through direct trial and error. | **`[TIER-B]`** | `doc_feat==2`: Pair two adjacent brain silhouettes (mirror/observer trace) synchronizing motor pathway |
| `F3` | **práctica** (*practice*) | **ES:** La repetición con retroalimentación mejora precisión, velocidad y automatización.<br>**EN:** Repetition with feedback improves precision, speed and automation. | **`[TIER-B]`** | `doc_feat==3`: Smoothly thicken & brighten heavily traveled connectome track over repeated cycles |
| `F4` | **error de predicción** (*prediction error*) | **ES:** La diferencia entre resultado esperado y real impulsa actualización de expectativas.<br>**EN:** The difference between expected and actual results drives updating of expectations. | **`[TIER-B]`** | `doc_feat==4`: Flash crisp opposing discrepancy arrow whenever prediction ring misses actual outcome orb |
| `F5` | **hábito** (*habit*) | **ES:** Una conducta repetida puede volverse dependiente de señales del contexto y menos sensible a la meta actual.<br>**EN:** A repeated behavior may become dependent on contextual cues and less sensitive to the current goal. | **`[TIER-B]`** | `doc_feat==5`: Shift control loop cleanly down from prefrontal cortex to automatic striatal shortcut |
| `F6` | **generalización** (*generalization*) | **ES:** Lo aprendido se extiende a situaciones parecidas, a veces de manera útil y otras de manera excesiva.<br>**EN:** What is learned is extended to similar situations, sometimes usefully and sometimes excessively. | **`[TIER-B]`** | `doc_feat==6`: Expand narrow learned point cloud outward across soft Gaussian generalization halo |
| `F7` | **aprendizaje social** (*social learning*) | **ES:** Enseñanza, cooperación, prestigio y conformidad influyen en qué información se adopta y transmite.<br>**EN:** Teaching, cooperation, prestige, and conformity influence what information is adopted and transmitted. | **`[TIER-D]`** | `doc_feat==7`: Add twin flanking teacher/peer silhouette beaming model demonstration wave inward |

##### `[TIER-B]` E17 `Z07` (`concepts`) -> Target `vid=2707` | Conceptos / *Concepts* (`0A / 8B / 0C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Experiencias distintas → rasgos y relaciones → categoría* / *Different experiences → traits and relationships → category* | **Evidence (ES / EN):** `Categorías flexibles construidas y aprendidas` (*Constructed and learned flexible categories*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Los conceptos permiten tratar experiencias diferentes como ejemplos de una misma idea. Sus límites pueden ser claros, graduales, contextuales o discutidos.
- **Spoken Macro Summary (EN):** Concepts allow different experiences to be treated as examples of the same idea. Their boundaries may be clear, gradual, context-dependent, or disputed.
- **Current Visual & Diagnosis:** `vid = 32` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=2`) -> `mm_brain_sdf` w/ bilateral temporal lobe highlight (`reg == 2.0`). Routed via `7 % 5 = 2` -> `vid=32` (bilateral temporal lobe highlight!). Semantic hub-and-spoke organization maps remarkably well to anterior/lateral temporal lobes (`[TIER-B]`), yet `doc_feat` triggers zero visual clustering for concrete examples (`object`, `animal`, `tool`, `number`, `cause`, `boundary`).
- **3D Procedural Blueprint:** Suspend glowing **Anterior-Temporal Semantic Prototype Manifold** beside `reg==2.0` lobes exhibiting category prototypes, fuzzy radial boundaries, and causal directed links per `doc_feat`.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **objeto** (*object*) | **ES:** El concepto de objeto reúne continuidad, forma, función y expectativas sobre cómo algo puede comportarse.<br>**EN:** The concept of object brings together continuity, form, function and expectations about how something can behave. | **`[TIER-B]`** | `doc_feat==0`: Group persistent solid wireframe polyhedron cues linking shape + grasp affordance |
| `F1` | **animal** (*animal*) | **ES:** Una categoría biológica cotidiana puede combinar movimiento, apariencia, conocimiento cultural y excepciones.<br>**EN:** An everyday biological category can combine movement, appearance, cultural knowledge, and exceptions. | **`[TIER-B]`** | `doc_feat==1`: Cluster animated organic locomotion motifs inside furry/feathered biological taxonomy node |
| `F2` | **herramienta** (*tool*) | **ES:** Las herramientas se agrupan por función y posibilidad de acción, no solo por aspecto visual.<br>**EN:** Tools are grouped by function and actionability, not just visual appearance. | **`[TIER-B]`** | `doc_feat==2`: Highlight action-affordance handle vector linking hand/effector node to manipulandum |
| `F3` | **peligro** (*danger*) | **ES:** El peligro integra señales sensoriales, memoria, probabilidad y posibles consecuencias.<br>**EN:** Danger integrates sensory signals, memory, probability and possible consequences. | **`[TIER-B]`** | `doc_feat==3`: Link threat cue + fast amygdala spike + avoidance path inside crimson danger badge |
| `F4` | **número** (*number*) | **ES:** Los números permiten representar cantidades y relaciones más allá de conjuntos visibles inmediatos.<br>**EN:** Numbers allow us to represent quantities and relationships beyond immediate visible sets. | **`[TIER-B]`** | `doc_feat==4`: Project abstract quantized 1-2-3-5 glowing discrete lattice pips independent of orb sizes |
| `F5` | **causa** (*cause*) | **ES:** Pensar causalmente distingue entre sucesión, correlación y mecanismos que producen cambios.<br>**EN:** Thinking causally distinguishes between succession, correlation, and mechanisms that produce change. | **`[TIER-B]`** | `doc_feat==5`: Replace mere simultaneous blinking with crisp directed A -> B mechanism arrow |
| `F6` | **semejanza** (*similarity*) | **ES:** Dos experiencias pueden parecer similares según forma, función, origen o cualquier dimensión relevante para la tarea.<br>**EN:** Two experiences may appear similar according to form, function, origin, or any dimension relevant to the task. | **`[TIER-B]`** | `doc_feat==6`: Stretch elastic multi-axis similarity axes rotating clusters along shape vs function |
| `F7` | **frontera de categoría** (*category border*) | **ES:** Los ejemplos atípicos muestran que muchas categorías no poseen una única característica necesaria y suficiente.<br>**EN:** Outlier examples show that many categories do not possess a single necessary and sufficient characteristic. | **`[TIER-B]`** | `doc_feat==7`: Hover fuzzy ambiguous outlier node straddling translucent border of overlapping halos |

##### `[TIER-B]` E17 `Z08` (`language`) -> Target `vid=2708` | Lenguaje / *Language* (`0A / 7B / 0C / 1D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Sonidos o gestos + símbolos + reglas + diálogo* / *Sounds or gestures + symbols + rules + dialogue* | **Evidence (ES / EN):** `Sistema simbólico biológico y cultural` (*Biological and cultural symbolic system*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** El lenguaje permite referirse a lo ausente, coordinar perspectivas y conservar ideas. No solo etiqueta pensamientos terminados: también reorganiza memoria, abstracción y cooperación.
- **Spoken Macro Summary (EN):** Language allows us to refer to what is absent, coordinate perspectives and preserve ideas. It not only labels finished thoughts: it also reorganizes memory, abstraction and cooperation.
- **Current Visual & Diagnosis:** `vid = 33` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=3`) -> `mm_brain_sdf` w/ traveling cortical z-sine wave. Routed via `8 % 5 = 3` -> `vid=33` (traveling cortical wave). Covers neuro-cognitive language processing (`[TIER-B]`), but `mm_brain_sdf` is bilaterally symmetric (ignoring left Broca–Wernicke arcuate asymmetry), and `doc_feat` shows no syntax hierarchy, metaphor mapping, or external writing.
- **3D Procedural Blueprint:** Asymmetrically light **Left-Hemisphere Arcuate Fasciculus Fiber Bridge** (`p.x < 0` + frontal-temporal loop) + orbiting syntactic glyph tree & metaphorical bridge arc per `doc_feat`.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **palabra** (*word*) | **ES:** Una palabra enlaza una forma sonora, escrita o gestual con usos y significados compartidos.<br>**EN:** A word links a sound, written or gestural form with shared uses and meanings. | **`[TIER-B]`** | `doc_feat==0`: Bind auditory temporal shimmer pulse cleanly onto symbolic token bead |
| `F1` | **nombre** (*name*) | **ES:** Nombrar permite volver a identificar, comparar y comunicar una entidad o categoría.<br>**EN:** Naming allows you to re-identify, compare, and communicate an entity or category. | **`[TIER-B]`** | `doc_feat==1`: Attach persistent identity halo tag hovering over tracked target orb |
| `F2` | **frase** (*phrase*) | **ES:** La combinación ordenada de palabras expresa relaciones que ninguna palabra contiene por sí sola.<br>**EN:** The ordered combination of words expresses relationships that no word contains on its own. | **`[TIER-B]`** | `doc_feat==2`: Snap linear word beads upward into nested binary hierarchical syntax arch |
| `F3` | **significado** (*meaning*) | **ES:** El significado depende de convenciones, contexto, intención, conocimiento y relación con otras expresiones.<br>**EN:** Meaning depends on conventions, context, intention, knowledge, and relationship to other expressions. | **`[TIER-B]`** | `doc_feat==3`: Glow rich contextual aura halo lighting multi-node neighborhood around active token |
| `F4` | **pregunta** (*question*) | **ES:** Preguntar hace visible una ausencia de información y solicita cooperación para reducirla.<br>**EN:** Asking makes an absence of information visible and requests cooperation to reduce it. | **`[TIER-B]`** | `doc_feat==4`: Leave bright unfilled gap bracket inviting incoming complementary reply bead |
| `F5` | **respuesta** (*answer*) | **ES:** Una respuesta puede informar, explicar, corregir, persuadir o revelar nueva incertidumbre.<br>**EN:** A response can inform, explain, correct, persuade, or reveal new uncertainty. | **`[TIER-B]`** | `doc_feat==5`: Stream filling complement bead smoothly closing question bracket into full arch |
| `F6` | **metáfora** (*metaphor*) | **ES:** La metáfora traslada estructura entre dominios y permite pensar lo abstracto mediante relaciones conocidas.<br>**EN:** Metaphor transfers structure between domains and allows us to think about the abstract through known relationships. | **`[TIER-B]`** | `doc_feat==6`: Project radiant parabolic bridge copying structural geometry from concrete domain to abstract |
| `F7` | **escritura** (*writing*) | **ES:** La escritura separa el mensaje del momento de habla y amplía memoria, revisión y transmisión entre generaciones.<br>**EN:** Writing separates the message from the moment of speech and expands memory, review, and transmission between generations. | **`[TIER-D]`** | `doc_feat==7`: Etch detached floating inscribed glyph tablet beside brain persisting after speech fades |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP17-Z05-08)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP17-Z05-08` (Odyssey Act IV — Experience 17 Zones `Z05..Z08`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files (4+1 focused slice):
1. `crates/cosmos_render/src/doc_stages.rs` — route `experience_id == 17` `Z05..=Z08` directly to reserved `vid = 2705..=2708` (and populate `color_b = [zone_c.x, zone_c.y, zone_c.z, 1.0]` at `L739`), plus companion emitters ``3781` (`Z08` external vocal larynx + airborne sound-wave rings / written clay-to-code glyphs)` where specified.
2. `crates/cosmos_narration/src/resolver.rs` — enrich `SegmentVisualHint` / `DOMAIN_CUE_KEYWORDS` (`base_flags` / `cue_tags` / `telemetry`) for the Exp `17` `Z05..=Z08` spoken concepts.
3. `shaders/hero_doc_matter_mind.wgsl` — handle decoded `vid >= 2705 && vid <= 2708` inside `march_doc_bio_mind` with distinct 3D procedural SDF / volumetric structures AND `switch (doc_feat(inst))` sub-stage branching (`0..k`, `0xFF` macro overview) smoothly driven by `doc_cue(inst)` & `doc_weight(inst)` (`0.72..1.35x` HDR cap).
4. `crates/cosmos_render/src/universe_scene.rs` + `shaders/compute_universe.wgsl` — ensure `field_grid_tex` / particle compute mode matches each zone (Compute Mode `7` hippocampal sharp-wave ripple arcs (`Z05`) & Hebbian co-firing synaptic sparks (`Z06`)).

Per-Zone & `doc_feat` Blueprints:
  - `Z05` `memory` (`vid=2705`): Map `doc_feat(0..7)` cleanly onto existing anatomical regions of `mm_brain_sdf`: `f0` DLPFC reverberating ring, `f1` hippocampus, `f2` distributed cortex, `f3` cerebellum (`reg==3.0`), `f5` decaying trace, `f6` rewoven trace, `f7` spindle consolidation burst. Features: `F0` memoria de trabajo/working memory, `F1` memoria episódica/episodic memory, `F2` memoria semántica/semantic memory, `F3` memoria procedimental/procedural memory, `F4` reconocimiento/recognition, `F5` olvido/forgetting, `F6` reconstrucción/reconstruction, `F7` consolidación/consolidation.
  - `Z06` `learning` (`vid=2706`): Overlay magnified **Dynamic Plastic Synapse / Error-Vector Overlay** anchored cleanly to prefrontal cortex (`mm_brain_sdf` `vid=31`) via `doc_feat`. Features: `F0` asociación/association, `F1` refuerzo/reinforcement, `F2` imitación/imitation, `F3` práctica/practice, `F4` error de predicción/prediction error, `F5` hábito/habit, `F6` generalización/generalization, `F7` aprendizaje social/social learning.
  - `Z07` `concepts` (`vid=2707`): Suspend glowing **Anterior-Temporal Semantic Prototype Manifold** beside `reg==2.0` lobes exhibiting category prototypes, fuzzy radial boundaries, and causal directed links per `doc_feat`. Features: `F0` objeto/object, `F1` animal/animal, `F2` herramienta/tool, `F3` peligro/danger, `F4` número/number, `F5` causa/cause, `F6` semejanza/similarity, `F7` frontera de categoría/category border.
  - `Z08` `language` (`vid=2708`): Asymmetrically light **Left-Hemisphere Arcuate Fasciculus Fiber Bridge** (`p.x < 0` + frontal-temporal loop) + orbiting syntactic glyph tree & metaphorical bridge arc per `doc_feat`. Features: `F0` palabra/word, `F1` nombre/name, `F2` frase/phrase, `F3` significado/meaning, `F4` pregunta/question, `F5` respuesta/answer, `F6` metáfora/metaphor, `F7` escritura/writing.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; keep volumetric shells tightly bounded inside hero sphere (`r <= 0.96`, early-exit `trans < 0.03`) with zero full-screen fog wash.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps for new documentary hero shells, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

<a id="pkg-act4-exp17-z09-12"></a>
### PKG-ACT4-EXP17-Z09-12: Exp `17` — Reasoning, Decision-Making, Imagination & Personal Identity (`Z09..Z12`)

- **Phase & Priority:** `Phase 2` · **P2 — Medium-High (Builds deductive branching trees, basal-ganglia utility scales & episodic time-travel mirror)** · **Scope:** `4` Zones (`Z09..Z12`), `32` Features (`doc_feat` `0..k`)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:61, 731-744`](../../crates/cosmos_render/src/doc_stages.rs#L61) & [`shaders/hero_doc_matter_mind.wgsl:556-590, 611, 760-809`](../../shaders/hero_doc_matter_mind.wgsl#L760-L809); `vid`: `doc_stage_variant_id(17)` -> `30 + (sc % 5)` (`34, 30, 31, 32`; single shared `mm_brain_sdf`, catalog `scene = null`); `doc_feat(inst)` unused in WGSL; `color_b` zone accent unused (`doc_stages.rs:731-744`). **Card Tiers — Zones:** `0A / 4B / 0C / 0D` · **Features:** `0A / 31B / 0C / 1D`.
- **Compute & Plumbing Focus:** Compute Mode `7` directed multi-step prefrontal-parietal tree cascades (`Z09, Z11`). Extra files: `shaders/hero_doc_matter_mind.wgsl` (`L770-L809` dorsolateral prefrontal + basal ganglia + DMN masks).

#### Reserved Target IDs

| Zone | Slug | ES / EN Title | Current `vid` | Reserved Target `vid` | Companion IDs | `submode` & `doc_feat` Range |
|---|---|---|---:|---:|---|---|
| `Z09` | `reasoning` | Razonamiento / *Reasoning* | `34` (`[TIER-B]`) | **`2709`** | `3782` (`Z10` probabilistic payoff balance beam / `Z12` longitudinal autobiographical memory timeline halo) | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z10` | `decisions` | Toma de decisiones / *Decision making* | `30` (`[TIER-B]`) | **`2710`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z11` | `imagination` | Imaginación / *Imagination* | `31` (`[TIER-B]`) | **`2711`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z12` | `identity` | Identidad / *Identity* | `32` (`[TIER-B]`) | **`2712`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |

#### Spoken Transcript Summary & 3D Visual Blueprint

Key deliverables for this Pick-Up Card:
- `vid=2709` (`Z09 reasoning`): Sculpt DLPFC-Parietal Working-Memory Lattice + branching premise-to-conclusion inference tree + Bayesian updating envelope (`doc_feat 0..7`).
- `vid=2710` (`Z10 decisions`): Replace occipital highlight with Orbitofrontal-Striatal Value Comparator + competing action corridors + delayed-gratification horizon (`doc_feat 0..7`).
- `vid=2711` (`Z11 imagination`): Project Hippocampal-DMN Counterfactual Holo-Stage (transparent split future timelines + mental rotation wireframe; `doc_feat 0..7`).
- `vid=2712` (`Z12 identity`): Illuminate Default Mode Network midline + temporal autobiographical thread (`t_past -> t_now -> t_future`) + mirror-reflection body aura (`doc_feat 0..7`).

##### `[TIER-B]` E17 `Z09` (`reasoning`) -> Target `vid=2709` | Razonamiento / *Reasoning* (`0A / 8B / 0C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Premisas + relaciones + método → conclusión provisional* / *Premises + relationships + method → provisional conclusion* | **Evidence (ES / EN):** `Inferencias falibles guiadas por reglas y evidencia` (*Fallible inferences guided by rules and evidence*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Razonar conecta ideas para producir inferencias. No garantiza una conclusión correcta: depende de premisas, evidencia, métodos y de cómo se representan las alternativas.
- **Spoken Macro Summary (EN):** Reasoning connects ideas to produce inferences. It does not guarantee a correct conclusion: it depends on premises, evidence, methods and how alternatives are represented.
- **Current Visual & Diagnosis:** `vid = 34` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=4`) -> `mm_brain_sdf` w/ global workspace z-sweep wave + hippocampal glow. Routed via `9 % 5 = 4` -> `vid=34` (global workspace fronto-parietal sweep). Matches integrative executive reasoning (`[TIER-B]`), while lacking visual representation of deductive trees, inductive uncertainty bands, causal controls, or probability cones via `doc_feat`.
- **3D Procedural Blueprint:** Render translucent **Logical Inference & Probability Lattice** framing global workspace cortex (`vid=34`) with crisp vs fuzzy edge branches switched by `doc_feat`.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **comparación** (*comparison*) | **ES:** Comparar exige elegir dimensiones relevantes; cambiar el criterio puede invertir cuál opción parece mejor.<br>**EN:** Comparing requires choosing relevant dimensions; changing the criteria can reverse which option seems better. | **`[TIER-B]`** | `doc_feat==0`: Align two candidate bars against rotating criterion prism flipping winner on tilt |
| `F1` | **deducción** (*deduction*) | **ES:** Una deducción válida preserva la verdad desde premisas a conclusión, pero no demuestra que las premisas sean ciertas.<br>**EN:** A valid deduction preserves the truth from premises to conclusion, but does not prove that the premises are true. | **`[TIER-B]`** | `doc_feat==1`: Collapse strict rigid wireframe premise pyramid deterministically onto terminal apex |
| `F2` | **inducción** (*induction*) | **ES:** La inducción generaliza desde casos observados y siempre conserva algún grado de incertidumbre.<br>**EN:** Induction generalizes from observed cases and always preserves some degree of uncertainty. | **`[TIER-B]`** | `doc_feat==2`: Project expanding dotted cone extrapolating forward from 4 observed sample points |
| `F3` | **analogía** (*analogy*) | **ES:** Una analogía transfiere relaciones entre situaciones, pero falla cuando las diferencias relevantes superan las semejanzas.<br>**EN:** An analogy transfers relationships between situations, but fails when the relevant differences outweigh the similarities. | **`[TIER-B]`** | `doc_feat==3`: Overlay structural isomorphism lines transferring pattern between paired boxes |
| `F4` | **causalidad** (*causality*) | **ES:** Inferir una causa requiere considerar mecanismos, controles y explicaciones alternativas.<br>**EN:** Inferring a cause requires considering alternative mechanisms, controls, and explanations. | **`[TIER-B]`** | `doc_feat==4`: Split confounded edge into controlled twin branches isolating true causal driver |
| `F5` | **probabilidad** (*probability*) | **ES:** El razonamiento probabilístico representa grados de posibilidad en vez de exigir certeza completa.<br>**EN:** Probabilistic reasoning represents degrees of possibility rather than requiring complete certainty. | **`[TIER-B]`** | `doc_feat==5`: Shade glowing Gaussian probability fan spanning 3 weighed fractional outcome orbs |
| `F6` | **evidencia** (*evidence*) | **ES:** La fuerza de una evidencia depende de su calidad, independencia y capacidad de distinguir hipótesis.<br>**EN:** The strength of evidence depends on its quality, independence, and ability to distinguish hypotheses. | **`[TIER-B]`** | `doc_feat==6`: Test two rival hypothesis orbs against incoming bright independent data probes |
| `F7` | **incertidumbre** (*uncertainty*) | **ES:** Reconocer incertidumbre permite buscar datos, comparar escenarios y evitar una confianza injustificada.<br>**EN:** Recognizing uncertainty allows you to search for data, compare scenarios and avoid unjustified confidence. | **`[TIER-B]`** | `doc_feat==7`: Soften rigid sharp conclusion point into breathing translucent credible-interval shell |

##### `[TIER-B]` E17 `Z10` (`decisions`) -> Target `vid=2710` | Toma de decisiones / *Decision making* (`0A / 8B / 0C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Opciones + valor + riesgo + tiempo + contexto → acción* / *Options + value + risk + time + context → action* | **Evidence (ES / EN):** `Competencia entre metas, valores y restricciones` (*Competition between goals, values and constraints*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Decidir combina metas, recompensas, hábitos, emociones, información social y consecuencias futuras. La misma elección puede cambiar bajo presión, incertidumbre o un marco diferente.
- **Spoken Macro Summary (EN):** Deciding combines goals, rewards, habits, emotions, social information, and future consequences. The same choice can change under pressure, uncertainty, or a different framework.
- **Current Visual & Diagnosis:** `vid = 30` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=0`) -> `mm_brain_sdf` (2 hemispheres + cerebellum + callosum + hippocampi) w/ posterior occipital highlight (`p.z < -0.10`). Routed via `10 % 5 = 0` -> `vid=30` (posterior occipital cortex!). Domain brain SDF + connectome matches neuro-decision-making (`[TIER-B]`), yet `10 % 5 = 0` lights the visual occipital lobe instead of orbitofrontal/ventromedial value cortex (`p.z > 0.12`), and `doc_feat` is ignored.
- **3D Procedural Blueprint:** Switch base region to **OFC/DLPFC Value-vs-Risk Circuit** + floating dual-option decision scales and temporal-discounting horizon driven by `doc_feat`.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **meta** (*goal*) | **ES:** Una meta define qué resultados cuentan como progreso y qué información merece atención.<br>**EN:** A goal defines what results count as progress and what information deserves attention. | **`[TIER-B]`** | `doc_feat==0`: Anchor glowing distant goal beacon pulling prefrontal planner pathways forward |
| `F1` | **recompensa** (*reward*) | **ES:** El valor esperado integra atractivo, probabilidad, demora y estado actual del organismo.<br>**EN:** The expected value integrates attractiveness, probability, delay and current state of the organism. | **`[TIER-B]`** | `doc_feat==1`: Modulate twin choice orb brightness by expected payoff x size x immediacy |
| `F2` | **riesgo** (*risk*) | **ES:** El riesgo combina posibles resultados con incertidumbre, no solo con la magnitud del daño.<br>**EN:** Risk combines possible outcomes with uncertainty, not just the magnitude of damage. | **`[TIER-B]`** | `doc_feat==2`: Surround high-payoff option orb with jagged uncertain red variance shell |
| `F3` | **hábito** (*habit*) | **ES:** Una respuesta habitual puede dominar aunque la situación actual favorezca otra opción.<br>**EN:** A habitual response may dominate even if the current situation favors another option. | **`[TIER-B]`** | `doc_feat==3`: Route impulse down fast dim striatal shortcut bypassing reflective frontal weigher |
| `F4` | **emoción** (*emotion*) | **ES:** Las emociones aportan señales de urgencia, importancia y aprendizaje previo a la comparación de alternativas.<br>**EN:** Emotions provide signals of urgency, importance and learning prior to comparing alternatives. | **`[TIER-B]`** | `doc_feat==4`: Flash rapid somatic-marker gut/amygdala urgency ring tipping tilted scale |
| `F5` | **información social** (*social information*) | **ES:** Consejos, normas y comportamiento ajeno pueden mejorar decisiones o propagar errores.<br>**EN:** Advice, rules and behavior from others can improve decisions or propagate errors. | **`[TIER-B]`** | `doc_feat==5`: Feed convergent social consensus stream nudging neutral choice weight sideways |
| `F6` | **presión temporal** (*time pressure*) | **ES:** Con menos tiempo, la mente usa reglas rápidas y explora menos consecuencias posibles.<br>**EN:** With less time, the mind uses quick rules and explores fewer possible consequences. | **`[TIER-B]`** | `doc_feat==6`: Shrink deliberation timer ring rapidly forcing immediate reflex branch selection |
| `F7` | **consecuencia diferida** (*deferred consequence*) | **ES:** Elegir entre beneficio inmediato y resultado futuro exige representar un yo y un mundo que todavía no existen.<br>**EN:** Choosing between immediate benefit and future outcome requires representing a self and a world that do not yet exist. | **`[TIER-B]`** | `doc_feat==7`: Link small near golden orb via temporal bridge to much larger distant glowing orb |

##### `[TIER-B]` E17 `Z11` (`imagination`) -> Target `vid=2711` | Imaginación / *Imagination* (`0A / 8B / 0C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Memoria recombinada + simulación → escenarios posibles* / *Recombined memory + simulation → possible scenarios* | **Evidence (ES / EN):** `Simulación de lo ausente y lo posible` (*Simulation of the absent and the possible*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Imaginar combina elementos conservados para construir escenas que no están presentes. Permite explorar posibilidades sin tener que vivirlas primero.
- **Spoken Macro Summary (EN):** Imagination combines stored elements to construct scenes that are not present. It allows possibilities to be explored without first experiencing them.
- **Current Visual & Diagnosis:** `vid = 31` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=1`) -> `mm_brain_sdf` w/ anterior prefrontal highlight (`p.z > 0.12`). Routed via `11 % 5 = 1` -> `vid=31` (anterior prefrontal highlight). Prefrontal + hippocampus default-mode imagery fits imagination (`[TIER-B]`), lacking `doc_feat` holographic replay ribbons, counterfactual branching, or dream dissociation.
- **3D Procedural Blueprint:** Project shimmering semi-transparent **Mental Simulation Hologram Canopy** above prefrontal cortex (`vid=31`) replaying recombined geometric fragments.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **imagen mental** (*mental image*) | **ES:** Una imagen mental puede representar forma, espacio o movimiento sin entrada sensorial equivalente en ese momento.<br>**EN:** A mental image can represent shape, space, or movement without equivalent sensory input at that moment. | **`[TIER-B]`** | `doc_feat==0`: Render ghostly wireframe rotated geometric object spinning without sensory input stream |
| `F1` | **simulación** (*simulation*) | **ES:** La mente ensaya acciones y consecuencias posibles antes de comprometer el cuerpo.<br>**EN:** The mind rehearses possible actions and consequences before engaging the body. | **`[TIER-B]`** | `doc_feat==1`: Rehearse translucent phantom action arc testing path before solid motor impulse fires |
| `F2` | **contrafactual** (*counterfactual*) | **ES:** Pensar qué habría ocurrido de otro modo ayuda a explicar, aprender y asignar responsabilidad.<br>**EN:** Thinking about what would have happened differently helps explain, learn, and assign responsibility. | **`[TIER-B]`** | `doc_feat==2`: Split past timeline node backward into glowing alternate 'what-if' shadow branch |
| `F3` | **creatividad** (*creativity*) | **ES:** La creatividad combina novedad con utilidad, expresividad o adecuación a una meta.<br>**EN:** Creativity combines novelty with usefulness, expressiveness or adaptation to a goal. | **`[TIER-B]`** | `doc_feat==3`: Splice incompatible colored fragments seamlessly into novel glowing chimeric shape |
| `F4` | **planificación** (*planning*) | **ES:** Planificar organiza pasos futuros, recursos y contingencias alrededor de un objetivo.<br>**EN:** Planning organizes future steps, resources, and contingencies around an objective. | **`[TIER-B]`** | `doc_feat==4`: Sequence 4 floating waypoint rings ahead toward horizon goal orb |
| `F5` | **sueño** (*dream*) | **ES:** Los sueños mezclan memoria, emoción y generación interna con menor control del entorno inmediato.<br>**EN:** Dreams mix memory, emotion and internal generation with less control of the immediate environment. | **`[TIER-B]`** | `doc_feat==5`: Dim external sensory brainstem gate while firing kaleidoscopic associative hippocampal dreams |
| `F6` | **relato** (*story*) | **ES:** Una narración ordena agentes, causas, conflictos y cambios en una secuencia significativa.<br>**EN:** A narrative orders agents, causes, conflicts and changes in a meaningful sequence. | **`[TIER-B]`** | `doc_feat==6`: String discrete simulated scenes along coherent glowing causal narrative arc |
| `F7` | **futuro posible** (*possible future*) | **ES:** Representar varios futuros permite comparar caminos, aunque ninguna simulación elimina la incertidumbre real.<br>**EN:** Representing several futures allows paths to be compared, although no simulation eliminates real uncertainty. | **`[TIER-B]`** | `doc_feat==7`: Fan 3 translucent diverging candidate future probability trajectories forward |

##### `[TIER-B]` E17 `Z12` (`identity`) -> Target `vid=2712` | Identidad / *Identity* (`0A / 7B / 0C / 1D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Cuerpo + memoria autobiográfica + valores + relaciones + relato* / *Body + autobiographical memory + values + relationships + story* | **Evidence (ES / EN):** `Continuidad construida a través del cambio` (*Continuity built through change*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La identidad combina continuidad y transformación. No es un objeto perfectamente fijo ni queda reducida a una ilusión: emerge de cuerpo, memoria, rasgos, compromisos y reconocimiento social.
- **Spoken Macro Summary (EN):** Identity combines continuity and transformation. It is not a perfectly fixed object nor is it reduced to an illusion: it emerges from body, memory, traits, commitments and social recognition.
- **Current Visual & Diagnosis:** `vid = 32` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=2`) -> `mm_brain_sdf` w/ bilateral temporal lobe highlight (`reg == 2.0`). Routed via `12 % 5 = 2` -> `vid=32` (temporal lobe highlight). Autobiographical & self-narrative networks sit heavily in medial/temporal-parietal regions (`[TIER-B]`), yet physical body envelope (`f0`), social roles (`f3`), and temporal autobiography strip are absent via `doc_feat`.
- **3D Procedural Blueprint:** Wrap `mm_brain_sdf` in a **Temporal Continuity Mobius / Autobiographical Thread** passing from past episodes through current embodied core.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **cuerpo** (*body*) | **ES:** El cuerpo proporciona una perspectiva estable pero cambiante desde la que se actúa y se siente.<br>**EN:** The body provides a stable but changing perspective from which to act and feel. | **`[TIER-D]`** | `doc_feat==0`: Render translucent breathing bodily vessel silhouette grounding internal brain perspective |
| `F1` | **memoria autobiográfica** (*autobiographical memory*) | **ES:** Los recuerdos enlazan episodios en una historia personal, aunque esa historia se reconstruya.<br>**EN:** Memories link episodes into a personal story, even if that story is reconstructed. | **`[TIER-B]`** | `doc_feat==1`: Link past glowing hippocampal episode beads along continuous spiraling timeline ribbon |
| `F2` | **personalidad** (*personality*) | **ES:** Tendencias relativamente estables orientan conducta, pero se expresan de manera distinta según situación y desarrollo.<br>**EN:** Relatively stable tendencies guide behavior, but they are expressed differently depending on the situation and development. | **`[TIER-B]`** | `doc_feat==2`: Display stable persistent inner geometric prism tint remaining recognizable as environment shifts |
| `F3` | **rol social** (*social role*) | **ES:** Una persona ocupa funciones y relaciones que influyen en expectativas, conducta y autodescripción.<br>**EN:** A person occupies roles and relationships that influence expectations, behavior, and self-description. | **`[TIER-B]`** | `doc_feat==3`: Attach 4 orbiting outer persona rings representing distinct social role interfaces |
| `F4` | **valor** (*value*) | **ES:** Los valores organizan prioridades y permiten juzgar qué clase de acción concuerda con el yo que se desea sostener.<br>**EN:** Values organize priorities and allow us to judge what kind of action agrees with the self that we wish to sustain. | **`[TIER-B]`** | `doc_feat==4`: Illuminate steady golden axial spine vector aligning chosen decisions with commitment |
| `F5` | **relato personal** (*personal story*) | **ES:** El relato de sí conecta causas, cambios y metas, pero siempre selecciona entre muchos hechos posibles.<br>**EN:** The self-story connects causes, changes, and goals, but always selects from among many possible facts. | **`[TIER-B]`** | `doc_feat==5`: Spotlight bright narrative filter illuminating selected beads along timeline while dimming others |
| `F6` | **continuidad** (*continuity*) | **ES:** Sentirse la misma persona depende de conexiones corporales, psicológicas y sociales a través del tiempo.<br>**EN:** Feeling the same person depends on bodily, psychological and social connections over time. | **`[TIER-B]`** | `doc_feat==6`: Pulse unbroken golden thread connecting early dim seed bead cleanly to current core |
| `F7` | **transformación** (*transformation*) | **ES:** Aprender, perder, elegir y relacionarse puede cambiar profundamente una identidad sin borrar toda continuidad.<br>**EN:** Learning, losing, choosing, and relating can profoundly change an identity without erasing all continuity. | **`[TIER-B]`** | `doc_feat==7`: Gradually morph outer polyhedral facet colors while keeping unbroken central spine intact |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP17-Z09-12)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP17-Z09-12` (Odyssey Act IV — Experience 17 Zones `Z09..Z12`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files (4+1 focused slice):
1. `crates/cosmos_render/src/doc_stages.rs` — route `experience_id == 17` `Z09..=Z12` directly to reserved `vid = 2709..=2712` (and populate `color_b = [zone_c.x, zone_c.y, zone_c.z, 1.0]` at `L739`), plus companion emitters ``3782` (`Z10` probabilistic payoff balance beam / `Z12` longitudinal autobiographical memory timeline halo)` where specified.
2. `crates/cosmos_narration/src/resolver.rs` — enrich `SegmentVisualHint` / `DOMAIN_CUE_KEYWORDS` (`base_flags` / `cue_tags` / `telemetry`) for the Exp `17` `Z09..=Z12` spoken concepts.
3. `shaders/hero_doc_matter_mind.wgsl` — handle decoded `vid >= 2709 && vid <= 2712` inside `march_doc_bio_mind` with distinct 3D procedural SDF / volumetric structures AND `switch (doc_feat(inst))` sub-stage branching (`0..k`, `0xFF` macro overview) smoothly driven by `doc_cue(inst)` & `doc_weight(inst)` (`0.72..1.35x` HDR cap).
4. `crates/cosmos_render/src/universe_scene.rs` + `shaders/compute_universe.wgsl` — ensure `field_grid_tex` / particle compute mode matches each zone (Compute Mode `7` directed multi-step prefrontal-parietal tree cascades (`Z09, Z11`)).

Per-Zone & `doc_feat` Blueprints:
  - `Z09` `reasoning` (`vid=2709`): Render translucent **Logical Inference & Probability Lattice** framing global workspace cortex (`vid=34`) with crisp vs fuzzy edge branches switched by `doc_feat`. Features: `F0` comparación/comparison, `F1` deducción/deduction, `F2` inducción/induction, `F3` analogía/analogy, `F4` causalidad/causality, `F5` probabilidad/probability, `F6` evidencia/evidence, `F7` incertidumbre/uncertainty.
  - `Z10` `decisions` (`vid=2710`): Switch base region to **OFC/DLPFC Value-vs-Risk Circuit** + floating dual-option decision scales and temporal-discounting horizon driven by `doc_feat`. Features: `F0` meta/goal, `F1` recompensa/reward, `F2` riesgo/risk, `F3` hábito/habit, `F4` emoción/emotion, `F5` información social/social information, `F6` presión temporal/time pressure, `F7` consecuencia diferida/deferred consequence.
  - `Z11` `imagination` (`vid=2711`): Project shimmering semi-transparent **Mental Simulation Hologram Canopy** above prefrontal cortex (`vid=31`) replaying recombined geometric fragments. Features: `F0` imagen mental/mental image, `F1` simulación/simulation, `F2` contrafactual/counterfactual, `F3` creatividad/creativity, `F4` planificación/planning, `F5` sueño/dream, `F6` relato/story, `F7` futuro posible/possible future.
  - `Z12` `identity` (`vid=2712`): Wrap `mm_brain_sdf` in a **Temporal Continuity Mobius / Autobiographical Thread** passing from past episodes through current embodied core. Features: `F0` cuerpo/body, `F1` memoria autobiográfica/autobiographical memory, `F2` personalidad/personality, `F3` rol social/social role, `F4` valor/value, `F5` relato personal/personal story, `F6` continuidad/continuity, `F7` transformación/transformation.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; keep volumetric shells tightly bounded inside hero sphere (`r <= 0.96`, early-exit `trans < 0.03`) with zero full-screen fog wash.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps for new documentary hero shells, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

<a id="pkg-act4-exp17-z13-17"></a>
### PKG-ACT4-EXP17-Z13-17: Exp `17` — Intelligence, Metacognition, Connected Minds, Culture & Knowledge (`Z13..Z17`)

- **Phase & Priority:** `Phase 2` · **P2 — Medium-High (Splits dyadic dual-brain empathy bridge, intergenerational library & shared knowledge threshold)** · **Scope:** `5` Zones (`Z13..Z17`), `41` Features (`doc_feat` `0..k`)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:61, 731-744`](../../crates/cosmos_render/src/doc_stages.rs#L61) & [`shaders/hero_doc_matter_mind.wgsl:556-590, 611, 760-809`](../../shaders/hero_doc_matter_mind.wgsl#L760-L809); `vid`: `doc_stage_variant_id(17)` -> `30 + (sc % 5)` (`33, 34, 30, 31, 32`; single shared `mm_brain_sdf`, catalog `scene = null`); `doc_feat(inst)` unused in WGSL; `color_b` zone accent unused (`doc_stages.rs:731-744`). **Card Tiers — Zones:** `0A / 2B / 0C / 3D` · **Features:** `0A / 14B / 1C / 26D`.
- **Compute & Plumbing Focus:** Compute Mode `7` cross-brain synchronization filaments (`Z15..Z17`) bridging smoothly into Exp `18` (`CULTURE_WEB`). Extra files: `crates/cosmos_render/src/doc_stages.rs` (`L742` emit twin peer brain hero `3783` when `exp == 17 && z >= 15`), `shaders/hero_doc_matter_mind.wgsl`.

#### Reserved Target IDs

| Zone | Slug | ES / EN Title | Current `vid` | Reserved Target `vid` | Companion IDs | `submode` & `doc_feat` Range |
|---|---|---|---:|---:|---|---|
| `Z13` | `intelligence` | Inteligencia / *Intelligence* | `33` (`[TIER-B]`) | **`2713`** | `3783` (`Z15` second mirrored `mm_brain_sdf` peer lobe for Theory of Mind + `Z16/17` orbital cultural artifact & library torus) | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z14` | `metacognition` | Metacognición / *Metacognition* | `34` (`[TIER-B]`) | **`2714`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z15` | `connected` | Mentes conectadas / *Connected minds* | `30` (`[TIER-D]`) | **`2715`** | `3783` (`Z15` second mirrored `mm_brain_sdf` peer lobe for Theory of Mind + `Z16/17` orbital cultural artifact & library torus) | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z16` | `culture` | Cultura / *Culture* | `31` (`[TIER-D]`) | **`2716`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..7` |
| `Z17` | `knowledge` | El conocimiento compartido / *Shared knowledge* | `32` (`[TIER-D]`) | **`2717`** | — | `submode=0` overview (`0xFF`), `doc_feat=0..8` |

#### Spoken Transcript Summary & 3D Visual Blueprint

Key deliverables for this Pick-Up Card:
- `vid=2713` (`Z13 intelligence`): Render whole-brain Global Connectome Flexibility Web contrasting fluid compression, transfer bridges, and adaptive feedback (`doc_feat 0..7`).
- `vid=2714` (`Z14 metacognition`): Suspend a Translucent Mirror-Lens Dome above Anterior Prefrontal Cortex (`BA10`) monitoring lower confidence/bias gauges (`doc_feat 0..7`).
- `vid=2715` (`Z15 connected`): Spawn twin mirrored `mm_brain_sdf` hemispheres (`3783`) linked by synchronized mirror-neuron & Theory-of-Mind empathy arcs (`doc_feat 0..7`).
- `vid=2716` (`Z16 culture`): Wrap mind cathedral inside rotating Outer Symbolic Scaffolding Torus (tool, script, music, institution runes; `doc_feat 0..7`).
- `vid=2717` (`Z17 knowledge`): Construct Intergenerational Luminous Library Ring archiving distributed artifacts across time toward Exp `18 Z00` (`doc_feat 0..7`).

##### `[TIER-B]` E17 `Z13` (`intelligence`) -> Target `vid=2713` | Inteligencia / *Intelligence* (`0A / 6B / 0C / 2D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Aprender + adaptar + resolver + comunicar + crear* / *Learn + adapt + solve + communicate + create* | **Evidence (ES / EN):** `Familia plural de capacidades adaptativas` (*Plural family of adaptive capacities*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La inteligencia no es una sola facultad ni pertenece únicamente a los humanos. Incluye capacidades diferentes para aprender, adaptarse, resolver problemas, cooperar, planificar y usar herramientas.
- **Spoken Macro Summary (EN):** Intelligence is not a single faculty nor does it belong only to humans. It includes different capacities to learn, adapt, solve problems, cooperate, plan and use tools.
- **Current Visual & Diagnosis:** `vid = 33` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=3`) -> `mm_brain_sdf` w/ traveling cortical z-sine wave. Routed via `13 % 5 = 3` -> `vid=33` (traveling cortical wave). Whole-brain adaptive coordination matches `[TIER-B]` neural intelligence archetype; ignores `doc_feat`, especially multi-agent social intelligence (`f6`) and physical external tool use (`f7`).
- **3D Procedural Blueprint:** Add modular **Multi-Faceted Problem-Solving Maze / Constraint Cage** around `vid=33` cortical web, activating social peer-orb (`f6`) and extended tool lever (`f7`).

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **aprendizaje** (*learning*) | **ES:** Un sistema inteligente modifica representaciones o estrategias a partir de experiencia.<br>**EN:** An intelligent system modifies representations or strategies based on experience. | **`[TIER-B]`** | `doc_feat==0`: Rewire sparse initial network cleanly into low-cost shortcut graph after feedback |
| `F1` | **adaptación** (*adaptation*) | **ES:** Adaptarse significa cambiar conducta o estructura para funcionar bajo condiciones nuevas.<br>**EN:** Adapting means changing behavior or structure to function under new conditions. | **`[TIER-B]`** | `doc_feat==1`: Flex cortex lattice symmetry swiftly when surrounding constraint frame shears |
| `F2` | **resolución de problemas** (*problem solving*) | **ES:** Resolver problemas exige representar una meta, restricciones y operaciones posibles.<br>**EN:** Solving problems requires representing a goal, constraints, and possible operations. | **`[TIER-B]`** | `doc_feat==2`: Propagate A*-like parallel search wavefront testing corridors inside puzzle cage |
| `F3` | **comunicación** (*communication*) | **ES:** Coordinar información con otros amplía las capacidades disponibles para cada individuo.<br>**EN:** Coordinating information with others expands the capabilities available to each individual. | **`[TIER-B]`** | `doc_feat==3`: Synchronize rhythmic pulse packet transmission toward nearby listening receiver node |
| `F4` | **planificación** (*planning*) | **ES:** La planificación compara secuencias antes de actuar y reserva recursos para metas futuras.<br>**EN:** Planning compares sequences before acting and reserves resources for future goals. | **`[TIER-B]`** | `doc_feat==4`: Reserve bright buffered energy ring across multi-step lookahead detour nodes |
| `F5` | **creatividad** (*creativity*) | **ES:** Generar posibilidades originales puede abrir soluciones que las reglas habituales no producen.<br>**EN:** Generating original possibilities can open up solutions that the usual rules do not produce. | **`[TIER-B]`** | `doc_feat==5`: Bypass blocked conventional wall via unexpected vertical out-of-plane leap arc |
| `F6` | **inteligencia social** (*social intelligence*) | **ES:** Comprender atención, intención, emoción y reputación facilita cooperación y competencia.<br>**EN:** Understanding attention, intention, emotion, and reputation facilitates cooperation and competition. | **`[TIER-D]`** | `doc_feat==6`: Spawn second glowing social agent sphere tracking shared gaze & reputation halo |
| `F7` | **uso de herramientas** (*use of tools*) | **ES:** Una herramienta extiende acción y cognición al incorporar propiedades del entorno en la solución.<br>**EN:** A tool extends action and cognition by incorporating properties of the environment into the solution. | **`[TIER-D]`** | `doc_feat==7`: Extend physical manipulative lever/probe artifact coupled directly to motor cortex output |

##### `[TIER-B]` E17 `Z14` (`metacognition`) -> Target `vid=2714` | Metacognición / *Metacognition* (`0A / 8B / 0C / 0D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Estimación + control + revisión + cambio de estrategia* / *Estimation + control + review + strategy change* | **Evidence (ES / EN):** `Pensamiento que supervisa pensamiento` (*Thought that supervises thought*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La metacognición permite evaluar confianza, detectar errores y elegir estrategias. No ofrece acceso perfecto a la propia mente: también puede estar sesgada o equivocada.
- **Spoken Macro Summary (EN):** Metacognition allows you to evaluate confidence, detect errors and choose strategies. It does not offer perfect access to one's own mind: it can also be biased or wrong.
- **Current Visual & Diagnosis:** `vid = 34` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=4`) -> `mm_brain_sdf` w/ global workspace z-sweep wave + hippocampal glow. Routed via `14 % 5 = 4` -> `vid=34` (global workspace sweep). Higher-order prefrontal monitoring matches metacognition (`[TIER-B]`); `doc_feat` triggers zero visual feedback for second-order observer loop, error detection, or confidence calibration.
- **3D Procedural Blueprint:** Enclose `mm_brain_sdf` inside an upper **Second-Order Prefrontal Meta-Monitor Halo** inspecting lower sensory-motor processing loop with dynamic error & confidence rings.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **confianza** (*confidence*) | **ES:** La confianza estima la probabilidad de estar en lo correcto, pero puede separarse de la precisión real.<br>**EN:** Confidence estimates the probability of being correct, but can be separated from actual accuracy. | **`[TIER-B]`** | `doc_feat==0`: Contrast inner bright confidence-halo radius against real outer noisy hit-target scatter |
| `F1` | **incertidumbre** (*uncertainty*) | **ES:** Representar que falta información es el primer paso para buscarla o suspender una conclusión.<br>**EN:** Representing that information is missing is the first step in searching for it or suspending a conclusion. | **`[TIER-B]`** | `doc_feat==1`: Leave flashing translucent '?' void gap inside model map prompting inward probe search |
| `F2` | **detección de error** (*error detection*) | **ES:** Conflicto, sorpresa o retroalimentación pueden señalar que una respuesta necesita revisión.<br>**EN:** Conflict, surprise, or feedback may signal that an answer needs revision. | **`[TIER-B]`** | `doc_feat==2`: Strobe sharp anterior-cingulate amber conflict burst upon contradictory signal collision |
| `F3` | **autocorrección** (*self-correction*) | **ES:** Corregirse exige conservar el error, identificar su origen y modificar el proceso que lo produjo.<br>**EN:** Correcting yourself requires preserving the error, identifying its origin and modifying the process that produced it. | **`[TIER-B]`** | `doc_feat==3`: Trace descending top-down calibration beam correcting erroneous sub-circuit weight |
| `F4` | **reflexión** (*reflection*) | **ES:** La reflexión desacopla parcialmente respuesta inmediata y acción para examinar razones y alternativas.<br>**EN:** Reflection partially decouples immediate response and action to examine reasons and alternatives. | **`[TIER-B]`** | `doc_feat==4`: Pause fast reflex spark temporarily while higher-order ring cycles calmly above |
| `F5` | **estrategia** (*strategy*) | **ES:** Elegir una estrategia implica anticipar qué método se ajusta mejor a la tarea y a los recursos disponibles.<br>**EN:** Choosing a strategy involves anticipating which method best fits the task and the available resources. | **`[TIER-B]`** | `doc_feat==5`: Toggle between 3 distinct structural sub-routing heuristics testing task speed/fit |
| `F6` | **límite** (*limit*) | **ES:** Saber que la memoria, atención o conocimiento son limitados ayuda a usar notas, herramientas y colaboración.<br>**EN:** Knowing that memory, attention, or knowledge are limited helps with using notes, tools, and collaboration. | **`[TIER-B]`** | `doc_feat==6`: Show small overflowing working-memory ring offloading beads smoothly onto external note tray |
| `F7` | **ignorancia reconocida** (*acknowledged ignorance*) | **ES:** Una mente mejora cuando puede distinguir entre no saber, creer saber y disponer de evidencia suficiente.<br>**EN:** A mind improves when it can distinguish between not knowing, believing it knows, and having sufficient evidence. | **`[TIER-B]`** | `doc_feat==7`: Partition knowledge sphere distinctly into solid lit core, fuzzy boundary, and clear dark unknown |

##### `[TIER-D]` E17 `Z15` (`connected`) -> Target `vid=2715` | Mentes conectadas / *Connected minds* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Atención compartida + enseñanza + cooperación + memoria externa* / *Shared attention + teaching + cooperation + external memory* | **Evidence (ES / EN):** `Cognición distribuida entre personas y herramientas` (*Distributed cognition between people and tools*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Una comunidad puede conservar y procesar más información que cualquier individuo aislado. Esto no crea necesariamente una conciencia única, sino una red de capacidades coordinadas.
- **Spoken Macro Summary (EN):** A community can hold and process more information than any isolated individual. This does not necessarily create a single consciousness, but rather a network of coordinated capabilities.
- **Current Visual & Diagnosis:** `vid = 30` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=0`) -> `mm_brain_sdf` (2 hemispheres + cerebellum + callosum + hippocampi) w/ posterior occipital highlight (`p.z < -0.10`). Routed via `15 % 5 = 0` -> `vid=30` (SINGLE solitary brain w/ posterior visual mask!). Narration explicitly describes **distributed multi-person cognition**: two people sharing gaze, teacher->learner transmission, division of labor, argumentative disagreement, external notes/maps, and institutions. A lone brain cannot depict interpersonal cognition.
- **3D Procedural Blueprint:** **Triadic Inter-Brain Network + External Memory Artifacts**: 3 spaced simplified `mm_brain_sdf` nodes orbiting shared glowing table holding holographic map, notebook tablet, and synchronized joint-attention gaze frustums.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **atención compartida** (*shared attention*) | **ES:** Dos personas pueden orientar la mirada y la acción hacia el mismo referente y saber que lo comparten.<br>**EN:** Two people can direct their gaze and action towards the same reference and know that they share it. | **`[TIER-D]`** | `doc_feat==0`: Converge dual glowing gaze frustums from two brain orbs cleanly onto one shared artifact |
| `F1` | **enseñanza** (*teaching*) | **ES:** Enseñar modifica deliberadamente la experiencia de otro para acelerar aprendizaje y evitar repetir todos los errores.<br>**EN:** Teaching deliberately modifies another's experience to accelerate learning and avoid repeating all mistakes. | **`[TIER-D]`** | `doc_feat==1`: Project structured glowing step-by-step trajectory wave from mentor brain to novice brain |
| `F2` | **imitación** (*imitation*) | **ES:** La imitación transmite habilidades visibles, pero también puede copiar acciones irrelevantes o convencionales.<br>**EN:** Imitation conveys visible skills, but can also copy irrelevant or conventional actions. | **`[TIER-D]`** | `doc_feat==2`: Mirror real-time kinematic motor posture arcs directly across twin adjacent brains |
| `F3` | **cooperación** (*cooperation*) | **ES:** La cooperación divide tareas, combina especialización y permite metas inaccesibles a un individuo.<br>**EN:** Cooperation divides tasks, combines specialization and allows goals inaccessible to an individual. | **`[TIER-D]`** | `doc_feat==3`: Split complex heavy glowing construct into 3 color-coded tasks solved jointly by trio |
| `F4` | **desacuerdo** (*disagreement*) | **ES:** El desacuerdo revela modelos distintos y puede producir corrección cuando existen reglas para comparar razones y pruebas.<br>**EN:** Disagreement reveals different models and can produce correction when there are rules for comparing reasons and evidence. | **`[TIER-D]`** | `doc_feat==4`: Collide two mismatched hypothesis wireframe overlays over shared evidence cube until aligned |
| `F5` | **memoria distribuida** (*distributed memory*) | **ES:** Las personas recuerdan quién sabe qué, mientras notas, mapas y dispositivos conservan contenido fuera del cerebro.<br>**EN:** People remember who knows what, while notes, maps and devices preserve content outside the brain. | **`[TIER-D]`** | `doc_feat==5`: Float external glowing cartographic map & incised notebook beside brains storing excess state |
| `F6` | **institución** (*institution*) | **ES:** Una institución estabiliza roles, procedimientos y registros más allá de participantes concretos.<br>**EN:** An institution stabilizes roles, procedures and records beyond specific participants. | **`[TIER-D]`** | `doc_feat==6`: Anchor multi-brain triangle around stable columnar institutional hub surviving transient nodes |
| `F7` | **acción coordinada** (*coordinated action*) | **ES:** Lenguaje, señales y normas sincronizan muchas decisiones en tiempo y espacio.<br>**EN:** Language, signals and norms synchronize many decisions in time and space. | **`[TIER-D]`** | `doc_feat==7`: Pulse rhythmic phase-locked carrier waves uniting all three brains into coordinated motion |

##### `[TIER-D]` E17 `Z16` (`culture`) -> Target `vid=2716` | Cultura / *Culture* (`0A / 0B / 0C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Aprendizaje social + símbolos + prácticas + acumulación* / *Social learning + symbols + practices + accumulation* | **Evidence (ES / EN):** `Herencia acumulativa transmitida socialmente` (*Cumulative socially transmitted inheritance*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** La cultura es un sistema de herencia aprendido: transmite costumbres, herramientas, relatos, arte, valores e instituciones. Cambia por innovación, selección, mezcla, conflicto y olvido.
- **Spoken Macro Summary (EN):** Culture is a learned inheritance system: it transmits customs, tools, stories, art, values and institutions. It changes through innovation, selection, mixing, conflict and forgetting.
- **Current Visual & Diagnosis:** `vid = 31` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=1`) -> `mm_brain_sdf` w/ anterior prefrontal highlight (`p.z > 0.12`). Routed via `16 % 5 = 1` -> `vid=31` (solitary prefrontal brain!). Narration enumerates concrete external cultural vehicles—customs, stories, physical tools, art, sacred values, communal rituals, technology, laws/schools/markets/archives—none represented on a single floating brain.
- **3D Procedural Blueprint:** **Cultural Accumulation Sanctuary Ring**: miniature brain halo suspended above rotating cultural artifact platform displaying hand tool silhouette, artistic sculpture/vessel, ritual circle, story glyph ribbon, and temple/archive colonnade.

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **costumbre** (*custom*) | **ES:** Una costumbre coordina expectativas mediante prácticas repetidas y socialmente reconocidas.<br>**EN:** A custom coordinates expectations through repeated and socially recognized practices. | **`[TIER-D]`** | `doc_feat==0`: Pulse synchronized walking/gathering loop trace shared repeatedly among 5 peer nodes |
| `F1` | **relato** (*story*) | **ES:** Los relatos conservan modelos de personas, lugares, causas y valores en formas fáciles de recordar y transmitir.<br>**EN:** Stories preserve patterns of people, places, causes, and values in ways that are easy to remember and transmit. | **`[TIER-D]`** | `doc_feat==1`: Unfurl glowing episodic narrative bead ribbon winding around communal hearth core |
| `F2` | **herramienta** (*tool*) | **ES:** Las herramientas encarnan conocimiento acumulado que el usuario no necesita reinventar por completo.<br>**EN:** Tools embody accumulated knowledge that the user does not need to completely reinvent. | **`[TIER-D]`** | `doc_feat==2`: Display cumulative multi-part hafted artifact hovering ready for instant pickup |
| `F3` | **arte** (*art*) | **ES:** El arte explora percepción, emoción, identidad y significado mediante formas compartidas y abiertas a interpretación.<br>**EN:** Art explores perception, emotion, identity and meaning through forms that are shared and open to interpretation. | **`[TIER-D]`** | `doc_feat==3`: Render harmonic ornamental lattice + painted luminous vessel rotating slowly |
| `F4` | **valor cultural** (*cultural value*) | **ES:** Los valores aprendidos orientan qué conductas se elogian, toleran, prohíben o consideran sagradas.<br>**EN:** Learned values guide what behaviors are praised, tolerated, prohibited, or considered sacred. | **`[TIER-D]`** | `doc_feat==4`: Highlight luminous elevated totem pedestal signaling protected shared sacred norms |
| `F5` | **ritual** (*ritual*) | **ES:** Los rituales sincronizan atención, emoción y pertenencia mediante secuencias reconocibles.<br>**EN:** Rituals synchronize attention, emotion, and belonging through recognizable sequences. | **`[TIER-D]`** | `doc_feat==5`: Lock 6 surrounding community nodes into unified phase-synchronous ritual ring dance |
| `F6` | **tecnología** (*technology*) | **ES:** La tecnología organiza materiales, energía e información para extender capacidades humanas.<br>**EN:** Technology organizes materials, energy and information to extend human capabilities. | **`[TIER-D]`** | `doc_feat==6`: Assemble layered material-energy-information mechanical armature on outer platform |
| `F7` | **institución** (*institution*) | **ES:** Leyes, escuelas, mercados, archivos y gobiernos conservan reglas y recursos a gran escala.<br>**EN:** Laws, schools, markets, archives, and governments preserve large-scale rules and resources. | **`[TIER-D]`** | `doc_feat==7`: Raise miniature multi-pillar institutional pavilion housing permanent shared rule tablets |

##### `[TIER-D]` E17 `Z17` (`knowledge`) -> Target `vid=2717` | El conocimiento compartido / *Shared knowledge* (`0A / 0B / 1C / 8D`)

- **Catalog Metadata:** `scene = None` | **Metric (ES / EN):** *Observación + registro + prueba + crítica + revisión* / *Observation + registration + test + criticism + review* | **Evidence (ES / EN):** `Ideas conservadas, contrastadas y revisables` (*Ideas preserved, contrasted and reviewable*) | `speak_evidence_default = True`
- **Spoken Macro Summary (ES):** Cuando muchas mentes comparten símbolos, corrigen errores y conservan descubrimientos, el pensamiento individual se convierte en cultura y conocimiento acumulativo.
- **Spoken Macro Summary (EN):** When many minds share symbols, correct errors, and preserve discoveries, individual thought becomes culture and cumulative knowledge.
- **Current Visual & Diagnosis:** `vid = 32` via `hero_doc_matter_mind.wgsl:556-590,760-809` (`m_sc=2`) -> `mm_brain_sdf` w/ bilateral temporal lobe highlight (`reg == 2.0`). Routed via `17 % 5 = 2` -> `vid=32` (solitary temporal brain!). Closes Exp 17 with concrete externalized epistemic tools—systematic measurement instruments, writing, physical archives, mathematical notation, scientific experimental rigs, and peer criticism—all completely missing.
- **3D Procedural Blueprint:** **Epistemic Observatory & Scientific Archive**: scaled optical telescope/caliper + incised measurement bar + glowing tablet archive rack + formal equation polyhedron + falsification loop bridging forward to Exp 18 (`18_culture`).

| Feat | ES / EN Name | ES + EN Spoken Summary (`universe_catalog.v1.json`) | Tier | Target `doc_feat` Sub-Stage Cue (`shaders/hero_doc_matter_mind.wgsl`) |
|---:|---|---|---:|---|
| `F0` | **observación** (*observation*) | **ES:** Observar de forma sistemática separa lo que se encontró de lo que se esperaba encontrar.<br>**EN:** Observing systematically separates what was found from what was expected to be found. | **`[TIER-D]`** | `doc_feat==0`: Aim dual calibrated optical lens/sensor beam contrasting expectation box vs raw data |
| `F1` | **testimonio** (*testimony*) | **ES:** El testimonio transmite experiencia ajena, pero su fiabilidad depende de memoria, competencia, incentivos y corroboración.<br>**EN:** Testimony conveys other people's experience, but its reliability depends on memory, competence, incentives, and corroboration. | **`[TIER-D]`** | `doc_feat==1`: Beam verbal testimonial wave between peer minds with cross-check verification arc |
| `F2` | **escritura** (*writing*) | **ES:** Los registros permiten comparar afirmaciones con versiones anteriores y ampliar la memoria de una comunidad.<br>**EN:** Records allow us to compare statements with previous versions and expand the memory of a community. | **`[TIER-D]`** | `doc_feat==2`: Etch luminous persistent cuneiform/alphabetical glyph strip onto floating stone tablet |
| `F3` | **medición** (*measurement*) | **ES:** Medir relaciona observaciones con procedimientos y unidades que otros pueden repetir y criticar.<br>**EN:** Measurement relates observations to procedures and units that others can reproduce and scrutinize. | **`[TIER-D]`** | `doc_feat==3`: Overlay crisp graduated reference ruler & balance needle validating reproducible units |
| `F4` | **archivo** (*archive*) | **ES:** Un archivo conserva información, procedencia y contexto para que el pasado siga disponible a nuevas preguntas.<br>**EN:** An archive preserves information, provenance and context so that the past remains available to new questions. | **`[TIER-D]`** | `doc_feat==4`: Rack glowing vertical archive tablets indexed cleanly by provenance links |
| `F5` | **matemáticas** (*mathematics*) | **ES:** Las matemáticas expresan estructuras y relaciones con precisión, revelando consecuencias que la intuición no siempre anticipa.<br>**EN:** Mathematics expresses structures and relationships with precision, revealing consequences that intuition does not always anticipate. | **`[TIER-D]`** | `doc_feat==5`: Spin crisp platonic geometric/algebraic polyhedron emitting exact predictive rays |
| `F6` | **modelo científico** (*scientific model*) | **ES:** Un modelo comprime observaciones, explica mecanismos y produce predicciones que pueden fallar.<br>**EN:** A model compresses observations, explains mechanisms, and produces predictions that may fail. | **`[TIER-D]`** | `doc_feat==6`: Fire empirical probe through compact experimental apparatus shattering failed hypothesis shell |
| `F7` | **crítica y revisión** (*criticism and review*) | **ES:** El conocimiento se fortalece cuando las afirmaciones pueden ser examinadas, replicadas, corregidas o reemplazadas.<br>**EN:** Knowledge is strengthened when statements can be examined, replicated, corrected or replaced. | **`[TIER-D]`** | `doc_feat==7`: Circulate open peer-review arrows inspecting, falsifying, and upgrading central model core |
| `F8` | **siguiente viaje** (*next trip*) | **ES:** Más allá: Espectro de la Cultura, del lenguaje compartido a herramientas, ciudades, ciencia, industria y civilización planetaria.<br>**EN:** Beyond: Spectrum of Culture, from shared language to tools, cities, science, industry and planetary civilization. | **`[TIER-C]`** | `doc_feat==8`: Open golden Culture Web orbital bridge forward into `18_culture` (`Z00`) |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP17-Z13-17)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP17-Z13-17` (Odyssey Act IV — Experience 17 Zones `Z13..Z17`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files (4+1 focused slice):
1. `crates/cosmos_render/src/doc_stages.rs` — route `experience_id == 17` `Z13..=Z17` directly to reserved `vid = 2713..=2717` (and populate `color_b = [zone_c.x, zone_c.y, zone_c.z, 1.0]` at `L739`), plus companion emitters ``3783` (`Z15` second mirrored `mm_brain_sdf` peer lobe for Theory of Mind + `Z16/17` orbital cultural artifact & library torus)` where specified.
2. `crates/cosmos_narration/src/resolver.rs` — enrich `SegmentVisualHint` / `DOMAIN_CUE_KEYWORDS` (`base_flags` / `cue_tags` / `telemetry`) for the Exp `17` `Z13..=Z17` spoken concepts.
3. `shaders/hero_doc_matter_mind.wgsl` — handle decoded `vid >= 2713 && vid <= 2717` inside `march_doc_bio_mind` with distinct 3D procedural SDF / volumetric structures AND `switch (doc_feat(inst))` sub-stage branching (`0..k`, `0xFF` macro overview) smoothly driven by `doc_cue(inst)` & `doc_weight(inst)` (`0.72..1.35x` HDR cap).
4. `crates/cosmos_render/src/universe_scene.rs` + `shaders/compute_universe.wgsl` — ensure `field_grid_tex` / particle compute mode matches each zone (Compute Mode `7` cross-brain synchronization filaments (`Z15..Z17`) bridging smoothly into Exp `18` (`CULTURE_WEB`)).

Per-Zone & `doc_feat` Blueprints:
  - `Z13` `intelligence` (`vid=2713`): Add modular **Multi-Faceted Problem-Solving Maze / Constraint Cage** around `vid=33` cortical web, activating social peer-orb (`f6`) and extended tool lever (`f7`). Features: `F0` aprendizaje/learning, `F1` adaptación/adaptation, `F2` resolución de problemas/problem solving, `F3` comunicación/communication, `F4` planificación/planning, `F5` creatividad/creativity, `F6` inteligencia social/social intelligence, `F7` uso de herramientas/use of tools.
  - `Z14` `metacognition` (`vid=2714`): Enclose `mm_brain_sdf` inside an upper **Second-Order Prefrontal Meta-Monitor Halo** inspecting lower sensory-motor processing loop with dynamic error & confidence rings. Features: `F0` confianza/confidence, `F1` incertidumbre/uncertainty, `F2` detección de error/error detection, `F3` autocorrección/self-correction, `F4` reflexión/reflection, `F5` estrategia/strategy, `F6` límite/limit, `F7` ignorancia reconocida/acknowledged ignorance.
  - `Z15` `connected` (`vid=2715`): **Triadic Inter-Brain Network + External Memory Artifacts**: 3 spaced simplified `mm_brain_sdf` nodes orbiting shared glowing table holding holographic map, notebook tablet, and synchronized joint-attention gaze frustums. Features: `F0` atención compartida/shared attention, `F1` enseñanza/teaching, `F2` imitación/imitation, `F3` cooperación/cooperation, `F4` desacuerdo/disagreement, `F5` memoria distribuida/distributed memory, `F6` institución/institution, `F7` acción coordinada/coordinated action.
  - `Z16` `culture` (`vid=2716`): **Cultural Accumulation Sanctuary Ring**: miniature brain halo suspended above rotating cultural artifact platform displaying hand tool silhouette, artistic sculpture/vessel, ritual circle, story glyph ribbon, and temple/archive colonnade. Features: `F0` costumbre/custom, `F1` relato/story, `F2` herramienta/tool, `F3` arte/art, `F4` valor cultural/cultural value, `F5` ritual/ritual, `F6` tecnología/technology, `F7` institución/institution.
  - `Z17` `knowledge` (`vid=2717`): **Epistemic Observatory & Scientific Archive**: scaled optical telescope/caliper + incised measurement bar + glowing tablet archive rack + formal equation polyhedron + falsification loop bridging forward to Exp 18 (`18_culture`). Features: `F0` observación/observation, `F1` testimonio/testimony, `F2` escritura/writing, `F3` medición/measurement, `F4` archivo/archive, `F5` matemáticas/mathematics, `F6` modelo científico/scientific model, `F7` crítica y revisión/criticism and review, `F8` siguiente viaje/next trip.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; keep volumetric shells tightly bounded inside hero sphere (`r <= 0.96`, early-exit `trans < 0.03`) with zero full-screen fog wash.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps for new documentary hero shells, preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

---

## 4.4 Spatial Experience 01 (`01_colors`) — *Espectro de Colores* (`130` Keys, `72` Lattice Nodes)

<a id="pkg-act4-exp01-spatial"></a>
### PKG-ACT4-EXP01-SPATIAL: Exp `01` — Colors Spectrum: Solver Completeness, Prism Optics, HSL Camera Targets & 4th Plasma Material (`130` Keys / `72` Lattice Nodes)

- **Phase & Priority:** `Phase 3` · **P2 — Medium (Spatial / Sensory Polish inside Act IV)** · **Scope:** Spatial Experience `01_colors` (`0` catalog zones, **`130` Chromatic Lexicon Keys**, **`72` Spatial Lattice Nodes**, `5` volumetric/surface hero instances)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:472-522`](../../crates/cosmos_render/src/doc_stages.rs#L472-L522) (`kind=5, vid=1` prism + `kind=5, vid=10` bicone + `kind=1, vid=12, submode=1..=3` spheres), [`shaders/hero_doc_matter_mind.wgsl:13-185`](../../shaders/hero_doc_matter_mind.wgsl#L13-L185), [`shaders/hero_doc_cosmos.wgsl:204-225`](../../shaders/hero_doc_cosmos.wgsl#L204-L225), [`crates/cosmos_manifest/src/color_solver.rs:63-126`](../../crates/cosmos_manifest/src/color_solver.rs#L63-L126), [`crates/cosmos_journey/src/odyssey.rs:486-585`](../../crates/cosmos_journey/src/odyssey.rs#L486-L585), [`crates/cosmos_app/src/assets.rs:367-372`](../../crates/cosmos_app/src/assets.rs#L367-L372), [`crates/cosmos_render/src/universe_scene.rs:303`](../../crates/cosmos_render/src/universe_scene.rs#L303) (`mode 9` dust). **Tier:** **`[TIER-B]` Spatial**.

#### Reserved Target IDs

| Component | Current ID / Slot | Reserved Target ID | Companion / Submode | Purpose |
|---|---|---|---|---|
| Primary Dispersion Prism + 7 Rays | `kind=5, vid=1` | **`1100`** (`kind=5`) | `submode=0..3` | Physical refraction, true 380 nm violet, wavelength-mapped highlight |
| Photoreceptor LMS Cones Stage (`scene==1`) | Unreachable (`scene=-1`) | **`1101`** (`kind=5`) | `scene=1` / `vid=1101` | Expose `show_cones` S/M/L spectral sensitivity cone curves |
| 3D HSL Bicone + Neutral Axis | `kind=5, vid=10` | **`1102`** (`kind=5`) | `submode=mat` | Remove neutral/black clamp distortion (`negro/gris/blanco` lock at `r=0`) |
| Quantum RGB Photons (`scene==3`) | Unreachable (`scene=-1`) | **`1103`** (`kind=5`) | `scene=3` / `vid=1103` | Expose `show_quantum` quantized R/G/B photon wavepackets |
| Tetrachromatic UV Axis (`scene>=4`) | Unreachable (`scene=-1`) | **`1104`** (`kind=5`) | `scene>=4` / `vid=1104` | Expose `show_tetra` 4-channel tetrachromatic perception manifold |
| Studio PBR Material Spheres | `kind=1, vid=12, sub=1..=3` | Keep `12` or **`3601..=3604`** | `3601` Gold, `3602` Glass, `3603` Matte, **`3604` Plasma** | Align `classify_chromatic_material` (`0..=3`) cleanly without `mat.max(1)` scrambling |

#### Spoken Transcript Summary & 3D Visual Blueprint

**7 Verified 1:1 Code & Visual Bugs to Resolve:**
1. **`39 / 130` Color Keys Mathematically Unproducible (`color_solver.rs:66-126`):** Early returns for `negro`, `blanco`, `gris`, `granate`, `marrón`, `beige`, `dorado`, `verde oliva`, `azul marino`, and `rosa` fire before `modifier` (`claro`, `oscuro`, `apagado`, `pálido`), while `amarillo oscuro` is swallowed by `verde oliva` (`52..82 && l < 0.32`).
2. **`40 / 130` Color Keys Hit Synthetic Rainbow Fallback (`odyssey.rs:486-559`):** Coarse `16^3` matches `89`, fine sweep matches `1` (`naranja oscuro`), missing producible **`ámbar pálido`** (exists only in `72 <= s < 76, l > 0.79, 38 <= h < 52`). Unmatched keys snap to `h = idx * 360 / 130` (e.g. `'azul marino claro'` renders bright orange!).
3. **Normalized-vs-Percent Camera Scale Bug (`odyssey.rs:583` vs `topology.rs:439-443`):** `make_color_beat` passes `s, l ∈ [0, 1]` directly into `hsl_to_prism_local(h_deg, s, l)` which divides by `100.0`, pinning ALL 130 Odyssey beats at `r ∈ [0.15, 0.1585], y ∈ [-0.80, -0.784]` (the bottom black cap).
4. **Dead Stages (`hero_doc_matter_mind.wgsl:49-51, 104-147`):** `show_cones`, `show_quantum`, and `show_tetra` are dead code because `resolver.rs:578,638` always emit `scene_index = -1`.
5. **Material Enum Mismatch & Missing Plasma Sphere (`resolver.rs:940-950`, `doc_stages.rs:116`, `hero_doc_cosmos.wgsl:207-225`):** `0=Gold, 1=ThinFilm, 2=Matte, 3=Plasma` collides with `mat.max(1)` (`1=Gold, 2=Glass, 3=Matte` — missing `Plasma`!).
6. **Prism & Bicone Shader Bugs (`hero_doc_matter_mind.wgsl:13-20, 94-103, 165-175`):** `mm_spectral_wavelength(0.0)` turns the 7th violet ray dim red `(0.40, 0, 0)`; `L172` compares cyclic hue to linear spectrum; `L97, 101` force `negro/gris/oscuro` into saturated midtones.
7. **Lattice Label Aliasing (`topology.rs:682-706`, `assets.rs:367-372`):** `72` entities map to `25` unique Spanish names, while `assets.rs:369` evaluates `hsl_to_srgb_u8(hue, 85.0, 50.0)` ignoring node `(s, l)` -> only `12` distinct floating MSDF labels.

**Engineering Deliverables:**
- **1. `color_solver.rs` Early-Return Precedence Fix (`39 -> <=3` Unproducible Keys)** (`L63-126`): `crates/cosmos_manifest/src/color_solver.rs:63-126`: Evaluate lightness/saturation modifiers (`oscuro`, `pálido`, `claro`, `apagado`) cleanly alongside neutral/earth/skin branches or route composite names directly, eliminating the `36` blocked modifier variants + `amarillo oscuro` (`L95` `verde oliva` overlap) + `gris apagado/pálido` (`L72-80`).
- **2. Odyssey Camera Target Scale Bug + Exhaustive RGB Table (`40` Fallbacks + All `130` Beats)** (`L486-559, L583`): `crates/cosmos_journey/src/odyssey.rs:486-559, 583`: Multiply normalized `s * 100.0, l * 100.0` before calling `hsl_to_prism_local(h_deg, s, l)` (`topology.rs:439`) so beats orbit the true chromatic locus instead of the bottom black `negro` cap; switch fine sweep to 1% perceptual steps + exact canonical seed LUT so `'ámbar pálido'` and all repaired keys resolve cleanly.
- **3. Material Enum Alignment + 4th Plasma Target Sphere (`3601..=3604`)** (`doc_stages.rs:116,502-521`; `hero_doc_cosmos.wgsl:204-225`): `crates/cosmos_narration/src/resolver.rs:940-950`, `crates/cosmos_render/src/doc_stages.rs:114-116, 502-521`, `shaders/hero_doc_cosmos.wgsl:204-225`: Align `material_class 0..=3` cleanly (`0=Gold -> 3601`, `1=ThinFilm -> 3602`, `2=Matte -> 3603`, `3=Plasma -> 3604`) instead of clamping `mat.max(1)`; add the missing **4th Plasma emissive sphere** (`submode=4` in `hero_doc_cosmos.wgsl:204-225`); update `engine.rs:1767` so idle hover selects diverse material tiers.
- **4. True Violet Dispersion, Linear Wavelength Ray Highlight & Neutral Bicone Lock** (`hero_doc_matter_mind.wgsl:13-20,94-103,165-175`): `shaders/hero_doc_matter_mind.wgsl:13-20, 94-103, 165-175`: Fix `mm_spectral_wavelength(0.0)` to output true spectral violet `vec3(0.45, 0.05, 1.0)` instead of dim red `(0.40, 0, 0)`; map cyclic `lock_hsl.x` to physical 380–700 nm index before `L172` ray highlight (magenta/pink `~0.95` no longer falsely lights UV/violet); remove `clamp(s, 0.25, 1.0)` and `max(s,0.6)/max(l,0.55)` overrides at `L97, L101` so `'negro'`, `'gris'`, and `'blanco'` lock directly on the central `r=0` neutral axis with true lightness.
- **5. Activate Dead Photoreceptor / Quantum / Tetrachromat Stages (`1101..=1104`) & 72-Node Lattice Labels** (`hero_doc_matter_mind.wgsl:49-51,104-147`; `assets.rs:367-372`): `crates/cosmos_render/src/doc_stages.rs:472-500`, `crates/cosmos_narration/src/resolver.rs:578,638`, `crates/cosmos_app/src/assets.rs:367-372`: Expose `scene = 1..=4` (via journey sub-chapters / `vid=1101..=1104`) so `show_cones` (LMS cones), `show_quantum` (RGB photon wavepackets), and `show_tetra` (4-cone bird/mantis axis) actually render; pass actual `(s_step, l_step)` of each of the `72` `ChromaticSample` lattice nodes into `label_text_for_visible` so labels display all `25+` real shade names instead of collapsing to `12` hue-only names.

##### Complete `130` Color Lexicon Inventory (`lexicons["01"]` — ES / EN Spoken Pairs & `color_solver.rs` Audit)

| # | Base Spanish Color Family (`es`) | Canonical Variant Keys (`base`, `apagado`, `claro`, `oscuro`, `pálido`) -> Spoken English (`en`) | Current Solver Status (of `130` Keys) |
|---:|---|---|---|
| 1 | **amarillo** (5) | `amarillo` (*yellow*), `amarillo apagado` (*muted yellow*), `amarillo claro` (*light yellow*), `amarillo oscuro` (*dark yellow*), `amarillo pálido` (*pale yellow*) | 4 matched; `amarillo oscuro`: Unproducible in color_solver (synthetic fallback) |
| 2 | **azul** (5) | `azul` (*blue*), `azul apagado` (*muted blue*), `azul claro` (*light blue*), `azul oscuro` (*dark blue*), `azul pálido` (*pale blue*) | Matched (odyssey sweep) |
| 3 | **azul cielo** (5) | `azul cielo` (*sky blue*), `azul cielo apagado` (*muted sky blue*), `azul cielo claro` (*light sky blue*), `azul cielo oscuro` (*dark sky blue*), `azul cielo pálido` (*pale sky blue*) | Matched (odyssey sweep) |
| 4 | **azul marino** (5) | `azul marino` (*navy blue*), `azul marino apagado` (*muted navy blue*), `azul marino claro` (*light navy blue*), `azul marino oscuro` (*dark navy blue*), `azul marino pálido` (*pale navy blue*) | 1 matched; `azul marino apagado`: Unproducible in color_solver (synthetic fallback); `azul marino claro`: Unproducible in color_solver (synthetic fallback); `azul marino oscuro`: Unproducible in color_solver (synthetic fallback); `azul marino pálido`: Unproducible in color_solver (synthetic fallback) |
| 5 | **añil** (5) | `añil` (*indigo*), `añil apagado` (*muted indigo*), `añil claro` (*light indigo*), `añil oscuro` (*dark indigo*), `añil pálido` (*pale indigo*) | Matched (odyssey sweep) |
| 6 | **beige** (5) | `beige` (*beige*), `beige apagado` (*muted beige*), `beige claro` (*light beige*), `beige oscuro` (*dark beige*), `beige pálido` (*pale beige*) | 1 matched; `beige apagado`: Unproducible in color_solver (synthetic fallback); `beige claro`: Unproducible in color_solver (synthetic fallback); `beige oscuro`: Unproducible in color_solver (synthetic fallback); `beige pálido`: Unproducible in color_solver (synthetic fallback) |
| 7 | **blanco** (5) | `blanco` (*white*), `blanco apagado` (*muted white*), `blanco claro` (*light white*), `blanco oscuro` (*dark white*), `blanco pálido` (*pale white*) | 1 matched; `blanco apagado`: Unproducible in color_solver (synthetic fallback); `blanco claro`: Unproducible in color_solver (synthetic fallback); `blanco oscuro`: Unproducible in color_solver (synthetic fallback); `blanco pálido`: Unproducible in color_solver (synthetic fallback) |
| 8 | **cian** (5) | `cian` (*cyan*), `cian apagado` (*muted cyan*), `cian claro` (*light cyan*), `cian oscuro` (*dark cyan*), `cian pálido` (*pale cyan*) | Matched (odyssey sweep) |
| 9 | **coral** (5) | `coral` (*coral*), `coral apagado` (*muted coral*), `coral claro` (*light coral*), `coral oscuro` (*dark coral*), `coral pálido` (*pale coral*) | Matched (odyssey sweep) |
| 10 | **dorado** (5) | `dorado` (*gold*), `dorado apagado` (*muted gold*), `dorado claro` (*light gold*), `dorado oscuro` (*dark gold*), `dorado pálido` (*pale gold*) | 1 matched; `dorado apagado`: Unproducible in color_solver (synthetic fallback); `dorado claro`: Unproducible in color_solver (synthetic fallback); `dorado oscuro`: Unproducible in color_solver (synthetic fallback); `dorado pálido`: Unproducible in color_solver (synthetic fallback) |
| 11 | **esmeralda** (5) | `esmeralda` (*emerald*), `esmeralda apagado` (*muted emerald*), `esmeralda claro` (*light emerald*), `esmeralda oscuro` (*dark emerald*), `esmeralda pálido` (*pale emerald*) | Matched (odyssey sweep) |
| 12 | **granate** (5) | `granate` (*maroon*), `granate apagado` (*muted maroon*), `granate claro` (*light maroon*), `granate oscuro` (*dark maroon*), `granate pálido` (*pale maroon*) | 1 matched; `granate apagado`: Unproducible in color_solver (synthetic fallback); `granate claro`: Unproducible in color_solver (synthetic fallback); `granate oscuro`: Unproducible in color_solver (synthetic fallback); `granate pálido`: Unproducible in color_solver (synthetic fallback) |
| 13 | **gris** (5) | `gris` (*gray*), `gris apagado` (*muted gray*), `gris claro` (*light gray*), `gris oscuro` (*dark gray*), `gris pálido` (*pale gray*) | 3 matched; `gris apagado`: Unproducible in color_solver (synthetic fallback); `gris pálido`: Unproducible in color_solver (synthetic fallback) |
| 14 | **magenta** (5) | `magenta` (*magenta*), `magenta apagado` (*muted magenta*), `magenta claro` (*light magenta*), `magenta oscuro` (*dark magenta*), `magenta pálido` (*pale magenta*) | Matched (odyssey sweep) |
| 15 | **marrón** (5) | `marrón` (*brown*), `marrón apagado` (*muted brown*), `marrón claro` (*light brown*), `marrón oscuro` (*dark brown*), `marrón pálido` (*pale brown*) | 1 matched; `marrón apagado`: Unproducible in color_solver (synthetic fallback); `marrón claro`: Unproducible in color_solver (synthetic fallback); `marrón oscuro`: Unproducible in color_solver (synthetic fallback); `marrón pálido`: Unproducible in color_solver (synthetic fallback) |
| 16 | **naranja** (5) | `naranja` (*orange*), `naranja apagado` (*muted orange*), `naranja claro` (*light orange*), `naranja oscuro` (*dark orange*), `naranja pálido` (*pale orange*) | Matched (odyssey sweep) |
| 17 | **negro** (5) | `negro` (*black*), `negro apagado` (*muted black*), `negro claro` (*light black*), `negro oscuro` (*dark black*), `negro pálido` (*pale black*) | 1 matched; `negro apagado`: Unproducible in color_solver (synthetic fallback); `negro claro`: Unproducible in color_solver (synthetic fallback); `negro oscuro`: Unproducible in color_solver (synthetic fallback); `negro pálido`: Unproducible in color_solver (synthetic fallback) |
| 18 | **púrpura** (5) | `púrpura` (*purple*), `púrpura apagado` (*muted purple*), `púrpura claro` (*light purple*), `púrpura oscuro` (*dark purple*), `púrpura pálido` (*pale purple*) | Matched (odyssey sweep) |
| 19 | **rojo** (5) | `rojo` (*red*), `rojo apagado` (*muted red*), `rojo claro` (*light red*), `rojo oscuro` (*dark red*), `rojo pálido` (*pale red*) | 3 matched; `rojo claro`: Unproducible in color_solver (synthetic fallback); `rojo pálido`: Unproducible in color_solver (synthetic fallback) |
| 20 | **rosa** (5) | `rosa` (*pink*), `rosa apagado` (*muted pink*), `rosa claro` (*light pink*), `rosa oscuro` (*dark pink*), `rosa pálido` (*pale pink*) | 3 matched; `rosa claro`: Unproducible in color_solver (synthetic fallback); `rosa pálido`: Unproducible in color_solver (synthetic fallback) |
| 21 | **turquesa** (5) | `turquesa` (*turquoise*), `turquesa apagado` (*muted turquoise*), `turquesa claro` (*light turquoise*), `turquesa oscuro` (*dark turquoise*), `turquesa pálido` (*pale turquoise*) | Matched (odyssey sweep) |
| 22 | **verde** (5) | `verde` (*green*), `verde apagado` (*muted green*), `verde claro` (*light green*), `verde oscuro` (*dark green*), `verde pálido` (*pale green*) | Matched (odyssey sweep) |
| 23 | **verde lima** (5) | `verde lima` (*lime green*), `verde lima apagado` (*muted lime green*), `verde lima claro` (*light lime green*), `verde lima oscuro` (*dark lime green*), `verde lima pálido` (*pale lime green*) | Matched (odyssey sweep) |
| 24 | **verde oliva** (5) | `verde oliva` (*olive green*), `verde oliva apagado` (*muted olive green*), `verde oliva claro` (*light olive green*), `verde oliva oscuro` (*dark olive green*), `verde oliva pálido` (*pale olive green*) | 1 matched; `verde oliva apagado`: Unproducible in color_solver (synthetic fallback); `verde oliva claro`: Unproducible in color_solver (synthetic fallback); `verde oliva oscuro`: Unproducible in color_solver (synthetic fallback); `verde oliva pálido`: Unproducible in color_solver (synthetic fallback) |
| 25 | **violeta** (5) | `violeta` (*violet*), `violeta apagado` (*muted violet*), `violeta claro` (*light violet*), `violeta oscuro` (*dark violet*), `violeta pálido` (*pale violet*) | Matched (odyssey sweep) |
| 26 | **ámbar** (5) | `ámbar` (*amber*), `ámbar apagado` (*muted amber*), `ámbar claro` (*light amber*), `ámbar oscuro` (*dark amber*), `ámbar pálido` (*pale amber*) | 4 matched; `ámbar pálido`: Producible but missed (synthetic fallback) |

**72-Node Spatial Lattice Coverage (`topology.rs:682-706`):** `12` hues (`0°..330°` step `30°`) × `2` saturations (`40%, 85%`) × `3` lightnesses (`30%, 50%, 75%`) = `72` `ChromaticSample` entities (`COLORS_PRISM` parent `MIND_CATHEDRAL`, `offset=(-0.22, 0.02, -0.04)`, `focus_scale_exp=2.0`, `u=[0.5, 3.6]`). Wire each node's real `(h, s, l)` into `assets.rs:367-372` and `engine.rs:1767` instead of fixed `(85, 50)` / `zone_index % 4`.

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP01-SPATIAL)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP01-SPATIAL` (Odyssey Act IV Stop 4/5 — Spatial Experience `01_colors`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files:
1. `crates/cosmos_manifest/src/color_solver.rs` (`L63-L126`) — restructure early-return dominance so modified variants (`apagado/claro/oscuro/pálido`) and `amarillo oscuro` / `gris apagado` / `gris pálido` are reachable cleanly.
2. `crates/cosmos_journey/src/odyssey.rs` (`L486-L585`) — pass `s * 100.0, l * 100.0` to `hsl_to_prism_local`, tighten HSL sweep to 1% steps + LUT seed so `'ámbar pálido'` and all 130 keys resolve accurately.
3. `crates/cosmos_narration/src/resolver.rs` (`L578, L638, L940-L950`), `crates/cosmos_render/src/doc_stages.rs` (`L105-L117, L472-L522`), `crates/cosmos_app/src/engine.rs` (`L1751-L1772`), `crates/cosmos_app/src/assets.rs` (`L367-L372`) — align `0=Gold, 1=Glass, 2=Matte, 3=Plasma` (`3601..=3604`), route `show_cones/show_quantum/show_tetra` (`1101..=1104`), and render unique `(s, l)` shade labels for all 72 lattice nodes.
4. `shaders/hero_doc_matter_mind.wgsl` (`L13-L185`) & `shaders/hero_doc_cosmos.wgsl` (`L204-L225`) — fix `mm_spectral_wavelength(0.0)` true violet, map cyclic hue -> linear 380-700 nm wavelength highlight, remove bicone `max(s,0.6)/max(l,0.55)` & `clamp(s,0.25,1.0)` neutral distortion, and add 4th Plasma PBR sphere shader (`3604`).

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; prism & bicone bounds remain compact (`r <= 0.96`).
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>

---

## 4.5 Spatial Experience 02 (`02_music`) — *Espectro Musical* (`48×4 = 192` Filaments, `20` Keys)

<a id="pkg-act4-exp02-spatial"></a>
### PKG-ACT4-EXP02-SPATIAL: Exp `02` — Music Spectrum: 48-String Chromatic Harp, Live Chladni PDE Sampling & Timbre Harmonics (`48x4` Filaments / `20` Keys)

- **Phase & Priority:** `Phase 3` · **P2 — Medium (Spatial / Sensory Polish completing Act IV)** · **Scope:** Spatial Experience `02_music` (`0` catalog zones, **`48` MIDI Notes (`C3..B6`) × `4` Timbres = `192` Spatial Filaments**, **`20` Lexicon Keys**, `5` volumetric hero instances)
- **Current State:** [`crates/cosmos_render/src/doc_stages.rs:527-562`](../../crates/cosmos_render/src/doc_stages.rs#L527-L562) (`kind=6, vid=1` central Chladni/Harp + `kind=6, vid=11..=14` Timbre Chambers), [`shaders/hero_doc_matter_mind.wgsl:190-320`](../../shaders/hero_doc_matter_mind.wgsl#L190-L320), [`crates/cosmos_render/src/universe_scene.rs:219, 304`](../../crates/cosmos_render/src/universe_scene.rs#L219) (`dispatch_field_grid` mode `1` + Compute Mode `8`), [`crates/cosmos_narration/src/resolver.rs:642, 652-725`](../../crates/cosmos_narration/src/resolver.rs#L652-L725), [`crates/cosmos_journey/src/odyssey.rs:623-665`](../../crates/cosmos_journey/src/odyssey.rs#L623-L665), [`crates/cosmos_spatial/src/topology.rs:708-729`](../../crates/cosmos_spatial/src/topology.rs#L708-L729). **Tier:** **`[TIER-B]` Spatial**.

#### Reserved Target IDs

| Component | Current ID / Slot | Reserved Target ID | Companion / Submode | Purpose |
|---|---|---|---|---|
| Central 48-String Harp + Live Chladni Plate | `kind=6, vid=1` | **`1200`** (`kind=6`) | `submode=0..3` | `48.0` strings (`C3..B6`) + octave ticks; samples `field_grid_tex` (`mode 1`); live pitch/octave `n_mode/m_mode` |
| Interactive Standing-Wave Mode Overlay | Analytical `cos-cos` only | **`1201..=1203`** | `submode=timbre` | Harmonic interval/nodal mode overlays & octave spiral rings (`C3..B6`) |
| Timbre Chamber 0 (`cristal` sine) | `kind=6, vid=11` | **`3610`** (`kind=6`) | `submode=0` | `5200 Hz` pure sine octahedron crystal + fundamental-only ring pulse |
| Timbre Chamber 1 (`aire` triangle) | `kind=6, vid=12` | **`3611`** (`kind=6`) | `submode=1` | `3400 Hz` odd-harmonic hollow flute air column swirl |
| Timbre Chamber 2 (`cálido` sawtooth) | `kind=6, vid=13` | **`3612`** (`kind=6`) | `submode=2` | `1900 Hz` rich integer-overtone bowed cedar resonance ribs |
| Timbre Chamber 3 (`profundo` square) | `kind=6, vid=14` | **`3613`** (`kind=6`) | `submode=3` | `980 Hz` dark odd-harmonic bronze gong shock rings (+ optional `3614` resonator ring) |

#### Spoken Transcript Summary & 3D Visual Blueprint

**4 Verified 1:1 Code & Visual Bugs to Resolve:**
1. **Harp String Count Mismatch (`24` vs `48`, `hero_doc_matter_mind.wgsl:291-300`):** Comments claim a *"48-String Chromatic Harp"*, yet `L294` & `L298` hardcode `24.0`, folding two semitones onto each visual string across `C3..B6`.
2. **Live Chladni PDE Ignored (`universe_scene.rs:219` vs `hero_doc_matter_mind.wgsl:281-282`):** `cs_field_grid` mode `1` writes `256x256` Chladni patterns every frame, yet `march_doc_music` never samples `field_grid_tex` (`doc_common.wgsl:24`).
3. **Static `n_mode` & Locked `m_mode=3.0` During Narration (`L217-218`):** `n_mode` ignores pitch/octave, while `m_mode = 3.0 + f32((doc_zone(inst) + 1u) % 4u)` pins `m_mode = 3.0` whenever `zone_index == u16::MAX` (`(65535 + 1) % 4 == 0`) during live narration!
4. **`LexiconKey` Telemetry Collision Hazard (`resolver.rs:642` vs `doc_stages.rs:120-135`):** `resolve_lexicon_key` sends `[2.0, key_index, 0.0, 0.0]`, which `normalized_music_telemetry` misreads as `raw_midi=2.0 -> 0.0` (C3) and `key_index` (`0..19`) clamped as wrong timbre.

**Engineering Deliverables:**
- **1. Upgrade Chromatic Harp from `24` -> `48` Strings (`C3..B6`) + Octave Ticks** (`hero_doc_matter_mind.wgsl:291-300`): `shaders/hero_doc_matter_mind.wgsl:291-300`: Change hardcoded `24.0` string cell division at `L294` & `L298` (`fract(s_norm * 24.0 + 0.5)`, `s_cell / 24.0`) to **`48.0`** matching MIDI `48..=95` (`C3..B6`), and add subtle golden octave divider ticks every `12th` string (`C3, C4, C5, C6`) so adjacent semitones no longer share the same physical string.
- **2. Sample Live Chladni PDE (`field_grid_tex`, Mode `1`) in `march_doc_music` (`1200`)** (`hero_doc_matter_mind.wgsl:275-290`; `doc_common.wgsl:24`): `crates/cosmos_render/src/universe_scene.rs:219`, `shaders/compute_universe.wgsl:414-428`, `shaders/hero_doc_matter_mind.wgsl:275-290`: Sample `textureSampleLevel(field_grid_tex, ...)` (`doc_common.wgsl:24`) directly onto the Chladni plate surface/nodal sand ridges inside `march_doc_music` so the already-dispatched `256x256` 2D standing-wave PDE drives real GPU wave interference instead of sitting unread.
- **3. Couple `n_mode/m_mode` to Active MIDI/Octave & Unlock `m_mode` During Live Narration** (`hero_doc_matter_mind.wgsl:217-218`; `resolver.rs:710`): `shaders/hero_doc_matter_mind.wgsl:217-218`: Derive `n_mode` and `m_mode` continuously from `(midi % 12)` pitch-class, octave `(3..6)`, and `active_timbre`, rather than static `(max(scene,0)+submode)%4` and `(doc_zone(inst)+1)%4` — eliminating the `u16::MAX` (`65535`) wrap bug `(65535 + 1) % 4 == 0` that freezes `m_mode = 3.0` on every spoken narration turn!
- **4. Harmonize Timbre Chambers (`3610..=3613`) + Sympathetic Resonator Ring (`3614`)** (`doc_stages.rs:543-561`; `hero_doc_matter_mind.wgsl:225-274`): `crates/cosmos_render/src/doc_stages.rs:543-561`, `shaders/hero_doc_matter_mind.wgsl:225-274`: Couple active chamber glow & overtone geometry in `3610` Cristal (`sine 5200Hz`), `3611` Aire (`triangle 3400Hz`), `3612` Cálido (`sawtooth 1900Hz`), `3613` Profundo (`square 980Hz`) directly to live overtone partials, and optionally emit `3614` Sympathetic Resonator Ring (`2:1`, `3:2`, `5:4` harmonic ratios).
- **5. Guard `LexiconKey` Telemetry Sentinel & Complete `192`-Filament Odyssey Traversal** (`resolver.rs:642`; `doc_stages.rs:119-135`): `crates/cosmos_narration/src/resolver.rs:642`, `crates/cosmos_render/src/doc_stages.rs:119-135`, `crates/cosmos_journey/src/odyssey.rs:623-631`: Set sentinel `telemetry[2] = -1.0` for `resolve_lexicon_key` and check `telemetry[2] >= 0.0` inside `normalized_music_telemetry` so lexicon item playback never passes `experience_id=2.0` into `(raw_midi - 48)/47` or overwrites timbre with `key_index`; document `48 / 192` Odyssey cycling vs all `192` interactive filaments (`topology.rs:708-729`).

##### Complete `20` Music Lexicon Keys & `48 × 4 = 192` Filament Specification (`lexicons["02"]` + `music`)

**A. 12 Chromatic Notes (`music.notes`, `midi = 48..=95`, `C3..B6` across 4 octaves = `48` MIDI pitches × `4` timbres = `192` spatial filaments in `MUSIC_HARP` at `topology.rs:647, 708-729`):**

| Pitch Class | Latin ES (`es`) | English (`en`) | Symbol | Audio Triple Indices | Target Harp / Chladni Blueprint (`vid=1200`) |
|---:|---|---|---|---|---|
| `0` | **do** | *C* | `C` | `[133, 6201, 12616]` | Dedicated semitone strings `k = 0, 12, 24, 36` of `48`; couples `n_mode` to pitch class `2` & live Chladni PDE peak |
| `1` | **do sostenido** | *C sharp* | `C♯` | `[134, 6202, 12617]` | Dedicated semitone strings `k = 1, 13, 25, 37` of `48`; couples `n_mode` to pitch class `3` & live Chladni PDE peak |
| `2` | **re** | *D* | `D` | `[145, 6213, 12628]` | Dedicated semitone strings `k = 2, 14, 26, 38` of `48`; couples `n_mode` to pitch class `4` & live Chladni PDE peak |
| `3` | **re sostenido** | *D sharp* | `D♯` | `[146, 6214, 12629]` | Dedicated semitone strings `k = 3, 15, 27, 39` of `48`; couples `n_mode` to pitch class `5` & live Chladni PDE peak |
| `4` | **mi** | *E* | `E` | `[139, 6207, 12622]` | Dedicated semitone strings `k = 4, 16, 28, 40` of `48`; couples `n_mode` to pitch class `6` & live Chladni PDE peak |
| `5` | **fa** | *F* | `F` | `[135, 6203, 12618]` | Dedicated semitone strings `k = 5, 17, 29, 41` of `48`; couples `n_mode` to pitch class `2` & live Chladni PDE peak |
| `6` | **fa sostenido** | *F sharp* | `F♯` | `[136, 6204, 12619]` | Dedicated semitone strings `k = 6, 18, 30, 42` of `48`; couples `n_mode` to pitch class `3` & live Chladni PDE peak |
| `7` | **sol** | *G* | `G` | `[148, 6216, 12631]` | Dedicated semitone strings `k = 7, 19, 31, 43` of `48`; couples `n_mode` to pitch class `4` & live Chladni PDE peak |
| `8` | **sol sostenido** | *G sharp* | `G♯` | `[149, 6217, 12632]` | Dedicated semitone strings `k = 8, 20, 32, 44` of `48`; couples `n_mode` to pitch class `5` & live Chladni PDE peak |
| `9` | **la** | *A* | `A` | `[137, 6205, 12620]` | Dedicated semitone strings `k = 9, 21, 33, 45` of `48`; couples `n_mode` to pitch class `6` & live Chladni PDE peak |
| `10` | **la sostenido** | *A sharp* | `A♯` | `[138, 6206, 12621]` | Dedicated semitone strings `k = 10, 22, 34, 46` of `48`; couples `n_mode` to pitch class `2` & live Chladni PDE peak |
| `11` | **si** | *B* | `B` | `[147, 6215, 12630]` | Dedicated semitone strings `k = 11, 23, 35, 47` of `48`; couples `n_mode` to pitch class `3` & live Chladni PDE peak |

**B. 4 Musical Timbres (`music.timbres` -> Chambers `vid=11..14` / Reserved `3610..=3613`):**

| Timbre Idx | ES / EN Label (`es` / `en`) | Waveform | Filter Hz | Detune (cents) | Attack / Release (s) | Reserved Companion Chamber Visual Blueprint |
|---:|---|---|---:|---:|---:|---|
| `0` | **cristal** (*crystal*) | `sine` | `5200 Hz` | `7.0` | `0.015 / 0.62` | `3610` Faceted Quartz Octahedron spinning purely with smooth fundamental-sine orbital halo |
| `1` | **aire** (*airy*) | `triangle` | `3400 Hz` | `5.0` | `0.025 / 0.46` | `3611` 5 Pan-Flute Vertical Tubes with triangular odd-harmonic air-column breath plumes |
| `2` | **cálido** (*warm*) | `sawtooth` | `1900 Hz` | `3.0` | `0.035 / 0.38` | `3612` Sculpted Cedar Violin Soundbox with continuous f-hole sawtooth harmonic overtone ribs |
| `3` | **profundo** (*deep*) | `square` | `980 Hz` | `2.0` | `0.045 / 0.50` | `3613` Concentric Cast Bronze Gong with heavy odd-harmonic square-wave pressure rings |

**C. 4 Octave Keys (`lexicons["02"]["octava 3..6"]` — completing all `20 / 20` Exp `02` Lexicon Keys):**

| Lexicon Key (`es`) | Spoken English (`en`) | MIDI Range | Triple `[ES, EN, FI]` | Spatial Spiral & `m_mode` Binding |
|---|---|---|---|---|
| **octava 3** | *octave 3* | MIDI 48..=59 (`C3..B3`) | `[140, 6208, 12623]` | Highlights 12-string octave quadrant (`C_oct` gold divider tick) & scales Chladni `m_mode` (`2 + oct - 2`) |
| **octava 4** | *octave 4* | MIDI 60..=71 (`C4..B4`) | `[141, 6209, 12624]` | Highlights 12-string octave quadrant (`C_oct` gold divider tick) & scales Chladni `m_mode` (`2 + oct - 2`) |
| **octava 5** | *octave 5* | MIDI 72..=83 (`C5..B5`) | `[142, 6210, 12625]` | Highlights 12-string octave quadrant (`C_oct` gold divider tick) & scales Chladni `m_mode` (`2 + oct - 2`) |
| **octava 6** | *octave 6* | MIDI 84..=95 (`C6..B6`) | `[143, 6211, 12626]` | Highlights 12-string octave quadrant (`C_oct` gold divider tick) & scales Chladni `m_mode` (`2 + oct - 2`) |

<details><summary>Copy-Paste AI Sub-Prompt (PKG-ACT4-EXP02-SPATIAL)</summary>

```markdown
Task: Implement `PKG-ACT4-EXP02-SPATIAL` (Odyssey Act IV Stop 5/5 — Spatial Experience `02_music`) in HyperCosmosVR.

Prerequisites assumed merged: `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI`.

Touched Files:
1. `shaders/hero_doc_matter_mind.wgsl` (`L190-L320`) — change harp string division from `24.0` to `48.0` strings (`C3..B6`) with octave divider ticks; sample `field_grid_tex` (`doc_common.wgsl:24`) live Chladni mode 1 PDE height onto the plate; replace `(doc_zone(inst) + 1) % 4` lock during `u16::MAX` narration with continuous `n_mode/m_mode` derived from pitch class, octave, and timbre.
2. `crates/cosmos_render/src/doc_stages.rs` (`L119-L135, L527-L562`) & `crates/cosmos_narration/src/resolver.rs` (`L642`) — tag `LexiconKey` telemetry with `telemetry[2] = -1.0` sentinel and guard `normalized_music_telemetry`; route central harp/plate to `1200` (`1201..=1203` mode overlays) and Timbre Chambers to `3610..=3613` (+ optional `3614` sympathetic interval ring).
3. `crates/cosmos_render/src/universe_scene.rs` (`L219, L304`) & `shaders/compute_universe.wgsl` (`L255-L269, L414-L428`) — ensure active MIDI/timbre modulates Mode 8 cymatic particles and Mode 1 `cs_field_grid` driving frequency.

Mandatory Rendering, OLED & VR Verification Gates:
- OLED Pure Black Void Budget: >= 50.91% exact `#000000` background pixels and 0.0000% highlight blowout; harp & flanking chambers sit cleanly on pure black void.
- VR 90/120 Hz Frame-Time Budget: <= 28 raymarch steps preserving RTX 5070 DX12 baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
- Hygiene (`CONTRIBUTING.md`): zero files > 50 MiB, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

Verification Commands:
  cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test --workspace -- --test-threads=1
```

</details>
