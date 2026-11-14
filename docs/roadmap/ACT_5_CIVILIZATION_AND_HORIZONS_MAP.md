# Act V Visual Gap Map & Pick-Up Cards: Civilization, Evidence, Perspectives & Cosmic Horizons

**Canonical Odyssey Scope (`ODYSSEY_ACTS[4]`):** `Exp 18 -> Exp 19 -> Exp 20 -> Exp 11 -> Exp 12 -> Exp 23 -> Exp 24 -> Exp 25` (`8` experiences · `154` zones · `1,038` features · `32` Claimable Pick-Up Cards out of `88` total project packages).

> **Navigation:** [Master Contributor Visual Roadmap](../CONTRIBUTOR_VISUAL_ROADMAP.md) · [Act I Map](ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md) · [Act II Map](ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md) · [Act III Map](ACT_3_MATTER_AND_SPACETIME_MAP.md) · [Act IV Map](ACT_4_LIFE_MIND_AND_SENSES_MAP.md) · **Act V Map (This File)** · [Shader Contracts](../../shaders/SHADER_CONTRACTS.md) · [Contributing Guide](../../CONTRIBUTING.md)

---

## 1. Executive Scorecard & `ODYSSEY_ACTS[4]` Travel Order

In [`crates/cosmos_journey/src/odyssey.rs#L41-L44`](../../crates/cosmos_journey/src/odyssey.rs#L41-L44), Act V traverses human cultural evolution outward through orbital technology and divergent planetary futures, leaps outward across the `HYPER_LOTUS_*` horizon wings (`u = 27.2..29.0` in [`crates/cosmos_spatial/src/topology.rs#L202-L228`](../../crates/cosmos_spatial/src/topology.rs#L202-L228) and [`#L362-L401`](../../crates/cosmos_spatial/src/topology.rs#L362-L401)) to interrogate cosmic evidence and the deep future of the universe, and culminates in cosmic perspectives, constructed meaning, and the final causal horizon.

- **Languages verified in [`assets/baked/universe_catalog.v1.json`](../../assets/baked/universe_catalog.v1.json):** Spanish (`es-ES`), English (`en-US`), and Finnish (`fi-FI`, `Locale::FiFi` in [`odyssey.rs#L58-L61`](../../crates/cosmos_journey/src/odyssey.rs#L58-L61)).
- **Granular Tier Totals (`154` zones / `1,038` features):** **`0` `[TIER-A]`** (`0.0%`), **`22` `[TIER-B]`** (`14.3%`), **`62` `[TIER-C]`** (`40.3%`), **`70` `[TIER-D]`** (`45.5%`) by zone; **`0` `[TIER-A]`** (`0.0%`), **`96` `[TIER-B]`** (`9.2%`), **`403` `[TIER-C]`** (`38.8%`), **`539` `[TIER-D]`** (`51.9%`) by individual spoken feature.

| Stop | Exp | Slug & ES Title | Topology Sector (`u` span, `scale`) | Zones (`154`) | Zone `[A / B / C / D]` | Feats (`1,038`) | Feat `[A / B / C / D]` | Current Render Dispatch (`doc_stages.rs`) | Reserved Primary `vid` + Companions | Cards (`32`) |
|---:|---:|---|---|---:|:---:|---:|:---:|---|---|---:|
| `5.1` | `18` | `18_culture` (*Cultura*) | `CULTURE_WEB` | `20` | `0 / 1 / 12 / 7` | `160` | `0 / 7 / 81 / 72` | `doc_stages.rs#L64`: `180 + (sc.unwrap_or(z) % 7)` (`180..=186`) | `2800..=2819` (`comp 3790..=3799`) | `4` |
| `5.2` | `19` | `19_technology` (*Tecnología*) | `TECH_RING` | `20` | `0 / 4 / 6 / 10` | `161` | `0 / 28 / 33 / 100` | `doc_stages.rs#L65-L69`: `{0\|u16::MAX -> 190, 16 -> 194, _ -> 190 + (sc % 7)}` (`197..=199` `else` lattice is dead code) | `2900..=2919` (`comp 3800..=3809`) | `4` |
| `5.3` | `20` | `20_futures` (*Futuros*) | `FUTURES_ARCS` | `19` | `0 / 2 / 6 / 11` | `152` | `0 / 4 / 52 / 96` | `doc_stages.rs#L70-L74`: `{0\|u16::MAX -> 200, 16 -> 202, _ -> 200 + (sc % 4)}` (`fut=1` & `3` collapse to identical Dyson core) | `3000..=3018` (`comp 3810..=3819`) | `4` |
| `5.4` | `11` | `11_cosmic_evidence` (*Evidencia cósmica*) | `HYPER_LOTUS_11_EVIDENCE` | `9` | `0 / 1 / 5 / 3` | `50` | `0 / 1 / 22 / 27` | `doc_stages.rs#L62`: `110 + (sc.unwrap_or(z) % 4)` (`scene` is `null` across all 9 zones -> modulo collapses to `z % 4`!) | `2100..=2108` (`comp 3720..=3729`) | `2` |
| `5.5` | `12` | `12_future_of_universe` (*Futuro del universo*) | `HYPER_LOTUS_12_FUTURE` | `10` | `0 / 3 / 4 / 3` | `59` | `0 / 10 / 21 / 28` | **CPU BYPASS BUG** `doc_stages.rs#L657-L691` (`5 \| 12 =>`) skips `doc_stage_variant_id(12)` (`120+(sc%5)` at `L63`) & renders hardcoded Kerr BH + crimson dwarf on all 10 zones! | `2200..=2209` (`comp 3730..=3739`) | `2` |
| `5.6` | `23` | `23_cosmic_perspectives` (*Perspectivas cósmicas*) | `HYPER_LOTUS_23_PERSPECTIVES` | `25` | `0 / 4 / 8 / 13` | `150` | `0 / 11 / 60 / 79` | `doc_stages.rs#L83`: `23 => 230` (single variant across all 25 zones!) | `3300..=3324` (`comp 3840..=3849`) | `5` |
| `5.7` | `24` | `24_cosmic_meaning` (*Significado cósmico*) | `HYPER_LOTUS_24_MEANING` | `26` | `0 / 6 / 15 / 5` | `156` | `0 / 29 / 96 / 31` | `doc_stages.rs#L84-L87`: `24 => { 24 => 241, _ => 240 }` (all `Z00..Z23, Z25` share `240`) | `3400..=3425` (`comp 3850..=3859`) | `6` |
| `5.8` | `25` | `25_final_horizon` (*Horizonte final*) | `HYPER_LOTUS_25_HORIZON` | `25` | `0 / 1 / 6 / 18` | `150` | `0 / 6 / 38 / 106` | `doc_stages.rs#L88`: `25 => 250` (single variant across all 25 zones!) | `3500..=3524` (`comp 3860..=3869`) | `5` |
| **ALL** | **`18..25`** | **Act V Complete (`ODYSSEY_ACTS[4]`)** | `u = 4.5..16.0` & `27.2..29.0` | **`154`** | **`0 / 22 / 62 / 70`** | **`1,038`** | **`0 / 96 / 403 / 539`** | `kind=9` (except `12` bypassed via `5 \| 12 =>`) | **`2100..=3524` + `3720..=3869`** | **`32`** |

## 2. Core P0 Prerequisites & Cross-Cutting Act V Defects

All 32 Act V Pick-Up Cards build on the 3 Phase-1 core plumbing packages tracked in [`docs/CONTRIBUTOR_VISUAL_ROADMAP.md`](../CONTRIBUTOR_VISUAL_ROADMAP.md):

1. **[`PKG-P0-DISPATCH-AND-IDS`](../CONTRIBUTOR_VISUAL_ROADMAP.md#pkg-p0-dispatch-and-ids)** — Extend `ch_exp_and_sub(vid)` in [`shaders/hero_doc_civilization_horizon.wgsl#L20-L25`](../../shaders/hero_doc_civilization_horizon.wgsl#L20-L25) so `vid >= 1000u` decodes `exp = (vid - 1000u) / 100u`, `sub = (vid - 1000u) % 100u` cleanly alongside legacy `100..=999` IDs; replace modulo collapses in [`crates/cosmos_render/src/doc_stages.rs#L62-L88`](../../crates/cosmos_render/src/doc_stages.rs#L62-L88); and remove `12` from `5 | 12 =>` (`doc_stages.rs#L657-L691`), which currently bypasses `doc_stage_variant_id(12)` and forces [`shaders/hero_doc_cosmos.wgsl#L411-L505`](../../shaders/hero_doc_cosmos.wgsl#L411-L505) (`28..50` raymarch steps) + [`#L204-L229`](../../shaders/hero_doc_cosmos.wgsl#L204-L229) onto all 10 zones of Exp 12 while stranding [`hero_doc_civilization_horizon.wgsl#L105-L142`](../../shaders/hero_doc_civilization_horizon.wgsl#L105-L142) as unreachable dead code.
2. **[`PKG-P0-FEATURE-AND-CUE-PLUMBING`](../CONTRIBUTOR_VISUAL_ROADMAP.md#pkg-p0-feature-and-cue-plumbing)** — Preserve `snap.cue_tags` inside [`crates/cosmos_app/src/engine.rs#L1691-L1699`](../../crates/cosmos_app/src/engine.rs#L1691-L1699) (`DocumentaryFocusState`), expand `DOMAIN_CUE_KEYWORDS` ([`crates/cosmos_narration/src/resolver.rs#L952-L961`](../../crates/cosmos_narration/src/resolver.rs#L952-L961), currently 58 ASCII tokens matching only **`31 / 154` zones (`20.1%`)** and **`32 / 1,038` features (`3.1%`)** across Act V) across ES/EN/FI civilizational, epistemic, horizon, and ethical vocabulary, pack `VisualCueTag` bitmasks into `base_flags` bits `2..7` in [`doc_stages.rs#L13-L31`](../../crates/cosmos_render/src/doc_stages.rs#L13-L31), and consume `doc_cue(inst)` + `doc_weight(inst)` (`0.72..1.35x` clamped HDR) in [`hero_doc_civilization_horizon.wgsl#L526-L577`](../../shaders/hero_doc_civilization_horizon.wgsl#L526-L577) (currently `doc_cue` is only read in dead `Exp 12 L139` & dead `Exp 19 L269`, and `doc_weight` is never called).
3. **[`PKG-P0-CATALOG-SCENE-AND-CI`](../CONTRIBUTOR_VISUAL_ROADMAP.md#pkg-p0-catalog-scene-and-ci)** — Backfill integer `scene` indices `0..8` on **Exp 11** (`9` zones currently `scene: null`) and `0..9` on **Exp 12** (`10` zones currently `scene: null`) inside [`assets/baked/universe_catalog.v1.json`](../../assets/baked/universe_catalog.v1.json); align [`topology.rs#L379`](../../crates/cosmos_spatial/src/topology.rs#L379) Exp 24 title `"Sentido Cósmico"` with catalog `"Significado cósmico"`; audit `speak_evidence_default: false` across Exp 12 `Z00..Z09` vs [`resolver.rs#L771`](../../crates/cosmos_narration/src/resolver.rs#L771); and review coverage in [`CosmicHighlightsPlugin`](../../crates/cosmos_journey/src/odyssey.rs#L202-L217) (`18/20/25` included; `19/11/12/23/24` omitted), [`signature_zone_slug`](../../crates/cosmos_app/src/engine.rs#L137-L151) (`11, 12` present; `18, 19, 20, 23, 24, 25` absent), and [`resolve_narration_card_anchor`](../../crates/cosmos_app/src/engine.rs#L2139-L2155).

### Cross-Cutting Compute, Far-Shell & VR Frame-Budget Guardrails
- **Compute Particle Alignment ([`crates/cosmos_render/src/universe_scene.rs#L301-L342`](../../crates/cosmos_render/src/universe_scene.rs#L301-L342) & [`shaders/compute_universe.wgsl#L319-L357`](../../shaders/compute_universe.wgsl#L319-L357)):** `Exp 18..20` run `mode=14` (Manhattan Transformer Optical Bus), which clashes with pre-industrial `18:Z00..Z17` & `19:Z01..Z06`; `Exp 11..12` run `mode=13` (radial redshift) even on `12:Z08` Dark Era near-vacuum; `Exp 23..25` share `mode=15` (25-petal Hyper-Lotus equilibrium). Add `submode`/`compute_mode` specialization per zone batch.
- **Far-Shell Filigree ([`shaders/far_shell.wgsl#L150-L161`](../../shaders/far_shell.wgsl#L150-L161) & [`universe_scene.rs#L354-L359`](../../crates/cosmos_render/src/universe_scene.rs#L354-L359)):** `lotus_boost = 0.42` activates the subtle 5-petal outer dark-void lotus filigree for Exps `11, 12, 23, 24, 25` (`u > 26.0`), keeping deep space strictly `#000000` (`>= 50.91%` OLED void budget).
- **VR 90/120 Hz Raymarch Step Budget:** [`hero_doc_civilization_horizon.wgsl#L548-L560`](../../shaders/hero_doc_civilization_horizon.wgsl#L548-L560) currently runs `clamp(42.0 * fovea_k, 24.0, 46.0)` steps (`46` max loop bound, `r < 0.94`, `trans < 0.03`). All upgraded Act V hero shells (`2100..=3524`) MUST hold **`<= 28` raymarch steps** (or strict `r <= 0.94..0.96` early exit `trans < 0.03`) to preserve the verified RTX 5070 DX12 baseline (`min = 0.852 ms`, `mean = 1.927 ms` [518.9 FPS], `p99 = 4.873 ms` at `1600x900` in [`artifacts/verification_renders/benchmark_sweep.txt`](../../artifacts/verification_renders/benchmark_sweep.txt)).

## 3. Quick-Jump Index of All `32` Claimable Act V Packages

| Package ID | Stop & Exp | Zones | Z `[A/B/C/D]` | Feats `[A/B/C/D]` | Phase | Priority | Reserved Primary `vid` | Companion IDs (`3720..=3869`) |
|---|---|---|---:|:---:|---|---|---|---|
| [`PKG-ACT5-EXP18-Z00-04`](#pkg-act5-exp18-z00-04) | 5.1 Exp 18 | `Z00..Z04` (`5`) | `0/1/4/0` | `40` (`0/7/29/4`) | Phase 1/2 | `P2` | `2800..=2804` | `3790..=3791` |
| [`PKG-ACT5-EXP18-Z05-09`](#pkg-act5-exp18-z05-09) | 5.1 Exp 18 | `Z05..Z09` (`5`) | `0/0/3/2` | `40` (`0/0/21/19`) | Phase 1/2 | `P1` | `2805..=2809` | `3792..=3794` |
| [`PKG-ACT5-EXP18-Z10-14`](#pkg-act5-exp18-z10-14) | 5.1 Exp 18 | `Z10..Z14` (`5`) | `0/0/2/3` | `40` (`0/0/14/26`) | Phase 1/2 | `P1` | `2810..=2814` | `3795..=3796` |
| [`PKG-ACT5-EXP18-Z15-19`](#pkg-act5-exp18-z15-19) | 5.1 Exp 18 | `Z15..Z19` (`5`) | `0/0/3/2` | `40` (`0/0/17/23`) | Phase 1/2 | `P1` | `2815..=2819` | `3797..=3799` |
| [`PKG-ACT5-EXP19-Z00-04`](#pkg-act5-exp19-z00-04) | 5.2 Exp 19 | `Z00..Z04` (`5`) | `0/0/1/4` | `40` (`0/0/4/36`) | Phase 1/2 | `P0` | `2900..=2904` | `3800..=3801` |
| [`PKG-ACT5-EXP19-Z05-09`](#pkg-act5-exp19-z05-09) | 5.2 Exp 19 | `Z05..Z09` (`5`) | `0/1/2/2` | `40` (`0/6/10/24`) | Phase 1/2 | `P1` | `2905..=2909` | `3802..=3804` |
| [`PKG-ACT5-EXP19-Z10-14`](#pkg-act5-exp19-z10-14) | 5.2 Exp 19 | `Z10..Z14` (`5`) | `0/2/2/1` | `40` (`0/14/12/14`) | Phase 1/2 | `P2` | `2910..=2914` | `3805..=3806` |
| [`PKG-ACT5-EXP19-Z15-19`](#pkg-act5-exp19-z15-19) | 5.2 Exp 19 | `Z15..Z19` (`5`) | `0/1/1/3` | `41` (`0/8/7/26`) | Phase 1/2 | `P1` | `2915..=2919` | `3807..=3809` |
| [`PKG-ACT5-EXP20-Z00-04`](#pkg-act5-exp20-z00-04) | 5.3 Exp 20 | `Z00..Z04` (`5`) | `0/1/1/3` | `40` (`0/1/6/33`) | Phase 1/2 | `P1` | `3000..=3004` | `3810..=3811` |
| [`PKG-ACT5-EXP20-Z05-09`](#pkg-act5-exp20-z05-09) | 5.3 Exp 20 | `Z05..Z09` (`5`) | `0/0/2/3` | `40` (`0/0/18/22`) | Phase 1/2 | `P1` | `3005..=3009` | `3812..=3814` |
| [`PKG-ACT5-EXP20-Z10-14`](#pkg-act5-exp20-z10-14) | 5.3 Exp 20 | `Z10..Z14` (`5`) | `0/0/1/4` | `40` (`0/1/10/29`) | Phase 1/2 | `P0` | `3010..=3014` | `3815..=3816` |
| [`PKG-ACT5-EXP20-Z15-18`](#pkg-act5-exp20-z15-18) | 5.3 Exp 20 | `Z15..Z18` (`4`) | `0/1/2/1` | `32` (`0/2/18/12`) | Phase 1/2 | `P1` | `3015..=3018` | `3817..=3819` |
| [`PKG-ACT5-EXP11-Z00-04`](#pkg-act5-exp11-z00-04) | 5.4 Exp 11 | `Z00..Z04` (`5`) | `0/1/3/1` | `26` (`0/1/11/14`) | Phase 1/2 | `P1` | `2100..=2104` | `3720..=3724` |
| [`PKG-ACT5-EXP11-Z05-08`](#pkg-act5-exp11-z05-08) | 5.4 Exp 11 | `Z05..Z08` (`4`) | `0/0/2/2` | `24` (`0/0/11/13`) | Phase 1/2 | `P1` | `2105..=2108` | `3725..=3728` |
| [`PKG-ACT5-EXP12-Z00-04`](#pkg-act5-exp12-z00-04) | 5.5 Exp 12 | `Z00..Z04` (`5`) | `0/0/2/3` | `29` (`0/1/13/15`) | Phase 1/2 | `P0` | `2200..=2204` | `3730..=3734` |
| [`PKG-ACT5-EXP12-Z05-09`](#pkg-act5-exp12-z05-09) | 5.5 Exp 12 | `Z05..Z09` (`5`) | `0/3/2/0` | `30` (`0/9/8/13`) | Phase 1/2 | `P2` | `2205..=2209` | `3735..=3739` |
| [`PKG-ACT5-EXP23-Z00-04`](#pkg-act5-exp23-z00-04) | 5.6 Exp 23 | `Z00..Z04` (`5`) | `0/2/1/2` | `30` (`0/3/12/15`) | Phase 1 | `P1` | `3300..=3304` | `3840..=3841` |
| [`PKG-ACT5-EXP23-Z05-09`](#pkg-act5-exp23-z05-09) | 5.6 Exp 23 | `Z05..Z09` (`5`) | `0/0/1/4` | `30` (`0/0/6/24`) | Phase 1 | `P0` | `3305..=3309` | `3842..=3843` |
| [`PKG-ACT5-EXP23-Z10-14`](#pkg-act5-exp23-z10-14) | 5.6 Exp 23 | `Z10..Z14` (`5`) | `0/0/0/5` | `30` (`0/0/8/22`) | Phase 1 | `P0` | `3310..=3314` | `3844..=3845` |
| [`PKG-ACT5-EXP23-Z15-19`](#pkg-act5-exp23-z15-19) | 5.6 Exp 23 | `Z15..Z19` (`5`) | `0/0/3/2` | `30` (`0/0/16/14`) | Phase 1 | `P1` | `3315..=3319` | `3846..=3847` |
| [`PKG-ACT5-EXP23-Z20-24`](#pkg-act5-exp23-z20-24) | 5.6 Exp 23 | `Z20..Z24` (`5`) | `0/2/3/0` | `30` (`0/8/18/4`) | Phase 1 | `P2` | `3320..=3324` | `3848..=3849` |
| [`PKG-ACT5-EXP24-Z00-04`](#pkg-act5-exp24-z00-04) | 5.7 Exp 24 | `Z00..Z04` (`5`) | `0/1/2/2` | `30` (`0/6/14/10`) | Phase 1/2 | `P1` | `3400..=3404` | `3850..=3851` |
| [`PKG-ACT5-EXP24-Z05-09`](#pkg-act5-exp24-z05-09) | 5.7 Exp 24 | `Z05..Z09` (`5`) | `0/0/4/1` | `30` (`0/1/25/4`) | Phase 1/2 | `P2` | `3405..=3409` | `3852..=3853` |
| [`PKG-ACT5-EXP24-Z10-14`](#pkg-act5-exp24-z10-14) | 5.7 Exp 24 | `Z10..Z14` (`5`) | `0/1/3/1` | `30` (`0/2/22/6`) | Phase 1/2 | `P2` | `3410..=3414` | `3854..=3855` |
| [`PKG-ACT5-EXP24-Z15-19`](#pkg-act5-exp24-z15-19) | 5.7 Exp 24 | `Z15..Z19` (`5`) | `0/2/2/1` | `30` (`0/13/10/7`) | Phase 1/2 | `P2` | `3415..=3419` | `3856..=3857` |
| [`PKG-ACT5-EXP24-Z20-22`](#pkg-act5-exp24-z20-22) | 5.7 Exp 24 | `Z20..Z22` (`3`) | `0/1/2/0` | `18` (`0/1/13/4`) | Phase 1/2 | `P2` | `3420..=3422` | `3858` |
| [`PKG-ACT5-EXP24-Z23-25`](#pkg-act5-exp24-z23-25) | 5.7 Exp 24 | `Z23..Z25` (`3`) | `0/1/2/0` | `18` (`0/6/12/0`) | Phase 1/2 | `P2` | `3423..=3425` | `3859` |
| [`PKG-ACT5-EXP25-Z00-04`](#pkg-act5-exp25-z00-04) | 5.8 Exp 25 | `Z00..Z04` (`5`) | `0/0/1/4` | `30` (`0/0/6/24`) | Phase 1 + Phase 4 | `P0` | `3500..=3504` | `3860..=3861` |
| [`PKG-ACT5-EXP25-Z05-09`](#pkg-act5-exp25-z05-09) | 5.8 Exp 25 | `Z05..Z09` (`5`) | `0/0/0/5` | `30` (`0/0/2/28`) | Phase 1 + Phase 4 | `P0` | `3505..=3509` | `3862..=3863` |
| [`PKG-ACT5-EXP25-Z10-14`](#pkg-act5-exp25-z10-14) | 5.8 Exp 25 | `Z10..Z14` (`5`) | `0/0/2/3` | `30` (`0/0/12/18`) | Phase 1 + Phase 4 | `P1` | `3510..=3514` | `3864..=3865` |
| [`PKG-ACT5-EXP25-Z15-19`](#pkg-act5-exp25-z15-19) | 5.8 Exp 25 | `Z15..Z19` (`5`) | `0/0/0/5` | `30` (`0/0/6/24`) | Phase 1 + Phase 4 | `P0` | `3515..=3519` | `3866..=3867` |
| [`PKG-ACT5-EXP25-Z20-24`](#pkg-act5-exp25-z20-24) | 5.8 Exp 25 | `Z20..Z24` (`5`) | `0/1/3/1` | `30` (`0/6/12/12`) | Phase 1 + Phase 4 | `P0` | `3520..=3524` | `3868..=3869` |

---

## 4. Complete Act V Pick-Up Cards (`32` Cards · `154` Zones · `1,038` Features)

### 4.1 Experience 18: `18_culture` — Espectro de la Cultura: del Conocimiento Compartido a la Civilización

- **Odyssey & Spatial Placement:** Act 5 (`1/8`, overall stop #18) · `CULTURE_WEB` (`SectorId::CULTURE_WEB`, parent `EARTH_ORBIT`, `u = [4.5, 6.9]`, `focus_scale_exp = 6.2`, `6.4e6 m`) ([`topology.rs#L297-L308, L648`](../../crates/cosmos_spatial/src/topology.rs#L297-L308))
- **Experience Totals:** `20` Zones (`0A / 1B / 12C / 7D`) · `160` Features (`0A / 7B / 81C / 72D`)
- **Current Shader & Compute:** `doc_stages.rs#L64`: `180 + (sc.unwrap_or(z) % 7)` (`180..=186`) · `shaders/hero_doc_civilization_horizon.wgsl#L165-L204` (`ch=0..5`: 3 base rings + campfire/ziggurat/dome/astrolabe/globe; `feat` & `cue` unused) · `mode=14` (`universe_scene.rs#L322`, `compute_universe.wgsl#L331-L345`: optical bus even on pre-literate fire/stone zones `Z00..Z16`)

<a id="pkg-act5-exp18-z00-04"></a>
### PKG-ACT5-EXP18-Z00-04: Shared Knowledge Origin, Gesture, Voice & Cultural Teaching

- **Phase:** Phase 1/2 | **Priority:** `P2` | **Scope:** Exp `18` (`18_culture`) `Z00..Z04` (`5` zones, `40` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L64`: `180 + (sc.unwrap_or(z) % 7)` (`180..=186`); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L165-L204` (`ch=0..5`: 3 base rings + campfire/ziggurat/dome/astrolabe/globe; `feat` & `cue` unused). Tier breakdown for this batch: **`0A / 1B / 4C / 0D` Zones**, **`0A / 7B / 29C / 4D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `0/5` zones and `0/40` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 2800..=2804` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3790`, `3791` (allocated from `3720 + (18 - 11)*10 + k`; Hearth-Ember & Oral-Tradition Cultural Diffusion (`mode=14` replacement / amber glow), plus `vid=2800..2804`):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z00` | `shared-knowledge` (*El conocimiento compartido* / *Shared knowledge*) | `[TIER-B]` | `vid=180` | **`2800`** | `3790` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z01` | `communication` (*Comunicación* / *Communication*) | `[TIER-C]` | `vid=181` | **`2801`** | `3791` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x02` (0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |
| `Z02` | `language` (*Lenguaje compartido* / *Shared language*) | `[TIER-C]` | `vid=182` | **`2802`** | `3790` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z03` | `stories` (*Relatos* / *Stories*) | `[TIER-C]` | `vid=183` | **`2803`** | `3791` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z04` | `teaching` (*Enseñanza* / *Teaching*) | `[TIER-C]` | `vid=184` | **`2804`** | `3790` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z00` `shared-knowledge` (`vid=2800`) — El conocimiento compartido (*Shared knowledge*) `[TIER-B]`

- **Catalog Metadata:** `scene=0` · `[0.000, 0.050]` · `rgb=[255, 211, 137]` · `sound=(sine/58Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Experiencia + símbolos + enseñanza + corrección → conocimiento público` / *`Experience + symbols + teaching + correction → public knowledge`* · Evidence: `Memoria colectiva acumulativa` (*Cumulative collective memory*)
- **Spoken Macro Narrative (ES / EN):** Compartir información no garantiza que sea verdadera, pero permite conservarla, compararla, corregirla y ampliarla. (*EN:* Sharing information does not guarantee that it is true, but it allows it to be preserved, compared, corrected and expanded.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (0 % 7) = 180` (`ch=0`, campfire flame + 3 concentric base rings). Gathering around a flickering fire matches prehistoric oral shared knowledge (`[TIER-B]`), though `doc_feat` is ignored and concrete external memory tokens/notched artifacts are absent (and Exp 18 runs `mode=14` optical bus particles anachronistically!). **Target (`vid=2800`):** Retain campfire hearth (`vid=2800` (upgrading legacy `180`)) as `Z00` origin; swap Exp 18 early zones (`Z00..Z16`) off `mode=14` optical-bus particles to glowing hearth-embers/cultural-transmission particles, and add `doc_feat` external notched bone/glyph ring + collective error correction.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **símbolos compartidos** (*shared symbols*) | `[TIER-B]` | Los símbolos permiten que varias personas señalen una misma idea sin depender del objeto presente. / *Symbols allow several people to point out the same idea without depending on the object present.* | Orbit luminous agreed abstract glyph symbols above hearth linking shared firelight gazes |
| `F1` | **memoria externa** (*external memory*) | `[TIER-D]` | Diagramas, marcas, objetos y relatos conservan información fuera de una sola mente. / *Diagrams, marks, objects, and stories preserve information outside of a single mind.* | Float notched tally bone & diagrammed stone tablet outside glowing hearth rings |
| `F2` | **enseñanza** (*teaching*) | `[TIER-B]` | Una habilidad puede transmitirse deliberadamente en lugar de redescubrirse desde cero. / *A skill can be passed on deliberately rather than rediscovered from scratch.* | Arc bright warm golden transmission stream from master hearth position toward apprentice node |
| `F3` | **corrección colectiva** (*collective correction*) | `[TIER-B]` | Otras personas pueden comprobar, discutir y modificar una afirmación. / *Other people can check, discuss, and modify a statement.* | Converge three outer observer arcs correcting & reshaping central glowing statement orb |
| `F4` | **historias** (*stories*) | `[TIER-B]` | Los relatos organizan experiencias y conectan acontecimientos separados. / *Stories organize experiences and connect separate events.* | Weave sequential luminous narrative beads arching smoothly around campfire circle |
| `F5` | **instrucciones** (*instructions*) | `[TIER-B]` | Una secuencia de acciones puede conservarse y repetirse por personas ausentes. / *A sequence of actions can be preserved and repeated by absent people.* | Project ghosted repeatable 4-step action silhouettes circling beside fire |
| `F6` | **descubrimientos** (*discoveries*) | `[TIER-B]` | Una idea útil puede incorporarse al repertorio de toda una comunidad. / *A useful idea can be incorporated into the repertoire of an entire community.* | Burst bright golden ember spark outward from one edge node lighting whole hearth ring |
| `F7` | **conocimiento provisional** (*provisional knowledge*) | `[TIER-B]` | Lo compartido puede mejorar, pero también puede contener errores, sesgos y olvidos. / *What is shared can improve, but it can also contain errors, biases and forgetfulness.* | Render shimmering cracked incomplete glass halo signaling provisional fallible knowledge |

##### `Z01` `communication` (`vid=2801`) — Comunicación (*Communication*) `[TIER-C]`

- **Catalog Metadata:** `scene=1` · `[0.050, 0.100]` · `rgb=[109, 220, 255]` · `sound=(triangle/62.1Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Señal + contexto + atención mutua → mensaje interpretado` / *`Signal + context + mutual attention → interpreted message`* · Evidence: `Transferencia social anterior a la escritura` (*Social transfer prior to writing*)
- **Spoken Macro Narrative (ES / EN):** La comunicación puede usar voz, gesto, mirada, ritmo, olor, postura o movimiento; no requiere escritura ni lenguaje hablado completo. (*EN:* Communication can use voice, gesture, gaze, rhythm, smell, posture or movement; It does not require writing or full spoken language.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (1 % 7) = 181` (`ch=1`, 2-tier stepped Mesopotamian ziggurat!). Pre-literate communication (hand gesture, voice, gaze, posture, rhythm, imitation) has nothing to do with Bronze-Age ziggurat temples. **Target (`vid=2801`):** **Dyadic Embodied Signal Stage**: twin opposing stylized human head-torso-hand beacons bridged by shared gaze rays, concentric acoustic voice ripples, bodily posture alignment, and rhythmic drum pulse rings.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **gesto** (*gesture*) | `[TIER-D]` | Las manos y el cuerpo pueden indicar dirección, tamaño, intención o emoción. / *The hands and body can indicate direction, size, intention or emotion.* | Articulate glowing directional hand-arm silhouette pointing toward distant target sphere |
| `F1` | **señal** (*signal*) | `[TIER-C]` | Una señal adquiere significado cuando emisor y receptor relacionan su forma con una situación. / *A signal acquires meaning when the sender and receiver relate its form to a situation.* | Flash dual synchronized context badge + signal waveform establishing shared referent |
| `F2` | **voz** (*voice*) | `[TIER-C]` | La voz transmite palabras, ritmo, urgencia, identidad y estado emocional. / *The voice transmits words, rhythm, urgency, identity and emotional state.* | Emit concentric modulated golden vocal pressure rings spanning sender to listener |
| `F3` | **mirada** (*gaze*) | `[TIER-D]` | La dirección de la mirada ayuda a compartir atención sobre un objeto o acontecimiento. / *Gaze direction helps share attention on an object or event.* | Lock twin ocular laser-gaze vectors onto same bright triangular object of joint attention |
| `F4` | **movimiento** (*movement*) | `[TIER-C]` | Acciones coordinadas pueden comunicar cuándo avanzar, detenerse o cambiar de tarea. / *Coordinated actions can communicate when to move forward, stop, or switch tasks.* | Synchronize coordinated stop/go kinematic swarm motions via leading beacon pulse |
| `F5` | **ritmo** (*rhythm*) | `[TIER-C]` | Patrones temporales facilitan coordinación, memoria y participación grupal. / *Temporal patterns facilitate coordination, memory and group participation.* | Beat quantized circular harmonic rhythm wave synchronizing multiple surrounding ring nodes |
| `F6` | **imitación** (*imitation*) | `[TIER-C]` | Copiar una acción permite aprender incluso sin una explicación verbal. / *Copying an action allows learning even without a verbal explanation.* | Mirror sender's glowing gesture arc symmetrically inside listener silhouette |
| `F7` | **mensaje** (*message*) | `[TIER-C]` | El significado depende del contexto y puede interpretarse de manera distinta por cada receptor. / *The meaning depends on the context and can be interpreted differently by each recipient.* | Split ambiguous incoming glyph wave into two distinct glowing meaning clouds at receiver |

##### `Z02` `language` (`vid=2802`) — Lenguaje compartido (*Shared language*) `[TIER-C]`

- **Catalog Metadata:** `scene=2` · `[0.100, 0.150]` · `rgb=[180, 161, 255]` · `sound=(sine/66.2Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Palabras + reglas + contexto → significados combinables` / *`Words + rules + context → combinable meanings`* · Evidence: `Sistema simbólico comunitario` (*Community symbolic system*)
- **Spoken Macro Narrative (ES / EN):** Un lenguaje compartido permite hablar de objetos ausentes, acontecimientos pasados y posibilidades futuras. (*EN:* A shared language allows us to talk about absent objects, past events and future possibilities.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (2 % 7) = 182` (`ch=2`, exact same 2-tier ziggurat!). Combinatorial spoken language, grammar, naming, categories, dialogue, and linguistic diversity cannot be visualized by a mudbrick temple. **Target (`vid=2802`):** **Combinatorial Syntax Tree & Dialogue Orbit**: layered phonetic word-beads snapping along branching grammatical syntax arches between alternating speaker orbs over a kaleidoscopic dialect-family canopy.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **palabra** (*word*) | `[TIER-C]` | Una palabra conecta una forma sonora, gestual o escrita con significados aprendidos socialmente. / *A word connects a sound, gestural or written form with socially learned meanings.* | Couple vocal ripple ring cleanly onto floating semantic concept crystal |
| `F1` | **gramática** (*grammar*) | `[TIER-C]` | Las reglas de combinación permiten expresar relaciones que una lista de palabras no mostraría. / *Combination rules allow you to express relationships that a list of words would not show.* | Snap flat linear word beads up into nested hierarchical binary grammar arches |
| `F2` | **nombre** (*name*) | `[TIER-C]` | Nombrar ayuda a mantener identidad, referencia y memoria a través del tiempo. / *Naming helps maintain identity, reference and memory over time.* | Anchor glowing persistent identity name-tag ring onto moving landmark orb |
| `F3` | **categoría** (*category*) | `[TIER-C]` | Las lenguas agrupan el mundo de maneras útiles, pero no todas trazan las mismas fronteras. / *Languages group the world in useful ways, but they don't all draw the same boundaries.* | Partition continuous multicolored spectrum sphere with competing dialect boundary hoops |
| `F4` | **pregunta** (*question*) | `[TIER-C]` | Preguntar convierte una incertidumbre privada en una búsqueda compartida. / *Asking turns a private uncertainty into a shared search.* | Cast upward questioning inverted parabola seeking complementary target arc |
| `F5` | **explicación** (*explanation*) | `[TIER-C]` | Una explicación relaciona causas, mecanismos, ejemplos y consecuencias. / *An explanation relates causes, mechanisms, examples and consequences.* | Unroll multi-step causal arrow cascade explaining link between A, B, and C |
| `F6` | **diálogo** (*dialogue*) | `[TIER-C]` | Las personas ajustan significado turno a turno mediante respuestas y aclaraciones. / *People adjust meaning turn by turn through responses and clarifications.* | Bounce reciprocal turn-taking luminous pulse back and forth tightening shared ring |
| `F7` | **diversidad lingüística** (*linguistic diversity*) | `[TIER-C]` | Dialectos y lenguas diferentes conservan historias y soluciones culturales distintas. / *Different dialects and languages preserve different histories and cultural solutions.* | Fan 5 differently tinted dialect-branch ribbons radiating from common ancestral stem |

##### `Z03` `stories` (`vid=2803`) — Relatos (*Stories*) `[TIER-C]`

- **Catalog Metadata:** `scene=3` · `[0.150, 0.200]` · `rgb=[255, 137, 164]` · `sound=(triangle/70.3Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Experiencias + secuencia + significado → relato transmisible` / *`Experiences + sequence + meaning → transmittable story`* · Evidence: `Memoria narrativa e identidad compartida` (*Narrative memory and shared identity*)
- **Spoken Macro Narrative (ES / EN):** Los relatos no solo entretienen: pueden conservar historia, conocimiento ambiental, valores, advertencias e identidades. (*EN:* Stories don't just entertain: they can preserve history, environmental knowledge, values, warnings, and identities.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (3 % 7) = 183` (`ch=3`, smooth upper hemisphere dome!). Oral stories, origin myths, route narratives, and warnings have zero dome architecture—meanwhile campfire (`vid=2803` (upgrading legacy `180`)) sat on `tools` and `institutions`! **Target (`vid=2803`):** **Holographic Oral Storytelling Constellation**: low glowing hearth embers projecting elevated narrative relief arcs—cosmic origins star-ring, overland trail waypoints, hazard flash, ancestral spirit rings, and shifting generational retellings.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **orígenes** (*origins*) | `[TIER-C]` | Los relatos de origen sitúan a una comunidad dentro de un paisaje, una genealogía o un orden simbólico. / *Origin stories situate a community within a landscape, a genealogy, or a symbolic order.* | Arch radiant origin-sky dome mapping landscape landmarks & genealogy base |
| `F1` | **viajes** (*travel*) | `[TIER-D]` | Narrar recorridos conserva rutas, peligros, encuentros y cambios. / *Narrating routes preserves routes, hazards, encounters and changes.* | Unroll luminous holographic terrain ribbon marked with hazard & water waypoints |
| `F2` | **advertencias** (*warnings*) | `[TIER-C]` | Una historia memorable puede transmitir riesgos con más fuerza que una instrucción aislada. / *A memorable story can convey risks more powerfully than an isolated instruction.* | Flash vivid amber-crimson danger silhouette surging up from story arc as warning |
| `F3` | **antepasados** (*ancestors*) | `[TIER-C]` | Recordar a quienes vivieron antes conecta obligaciones, territorios y prácticas. / *Remembering those who lived before connects obligations, territories and practices.* | Stack glowing ancestral ghost-rings backward along deep generational lineage |
| `F4` | **héroes** (*heroes*) | `[TIER-C]` | Las figuras heroicas concentran ideales y conflictos, aunque simplifiquen historias reales. / *Heroic figures concentrate ideals and conflicts, although they simplify real stories.* | Elevate enlarged idealized heroic star-beacon stripping away surrounding complex clutter |
| `F5` | **ficción** (*fiction*) | `[TIER-C]` | Imaginar situaciones inexistentes permite explorar decisiones sin ejecutarlas. / *Imagining non-existent situations allows you to explore decisions without executing them.* | Branch translucent unmanifested 'what-if' scenario orb safely off physical path |
| `F6` | **identidad** (*identity*) | `[TIER-C]` | Relatos compartidos ayudan a definir quién pertenece, aunque también pueden excluir. / *Shared stories help define who belongs, although they can also exclude.* | Draw luminous communal belonging circle brightly enclosing inner tribal orbs |
| `F7` | **reinterpretación** (*reinterpretation*) | `[TIER-C]` | Cada generación puede contar de nuevo una historia y cambiar su énfasis o significado. / *Each generation can retell a story and change its emphasis or meaning.* | Re-sculpt glowing story constellation facets gradually across 3 generational passes |

##### `Z04` `teaching` (`vid=2804`) — Enseñanza (*Teaching*) `[TIER-C]`

- **Catalog Metadata:** `scene=4` · `[0.200, 0.250]` · `rgb=[255, 239, 135]` · `sound=(sine/74.4Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Demostración + práctica + retroalimentación → aprendizaje acelerado` / *`Demonstration + practice + feedback → accelerated learning`* · Evidence: `Transmisión deliberada de habilidades` (*Deliberate transfer of skills*)
- **Spoken Macro Narrative (ES / EN):** La enseñanza acelera la transmisión cultural porque cada persona no necesita descubrirlo todo desde el principio. (*EN:* Teaching accelerates cultural transmission because each person does not need to discover everything from the beginning.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (4 % 7) = 184` (`ch=4`, two tilted armillary/astrolabe tori!). Celestial astronomical rings fail to depict deliberate pedagogical demonstration, scaffolding, practice, and error correction. **Target (`vid=2804`):** **Master-Apprentice Kinetic Scaffolding Arena**: paired kinematic model/learner avatars sharing glowing motion trajectory ghost-ribbon, red error-deviation markers, fading instructional scaffold, and reciprocal generational handoff.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **demostración** (*demonstration*) | `[TIER-C]` | Observar una acción completa revela movimientos, secuencia y resultado. / *Observing a complete action reveals movements, sequence and result.* | Trace slow full-sequence illuminated motion trajectory executed cleanly by mentor beacon |
| `F1` | **imitación** (*imitation*) | `[TIER-C]` | Repetir lo observado convierte una representación en práctica corporal. / *Repeating what is observed turns a representation into a bodily practice.* | Shadow overlapping novice trajectory converging toward ghosted mentor path |
| `F2` | **corrección** (*correction*) | `[TIER-C]` | La retroalimentación señala diferencias entre el intento y el resultado buscado. / *Feedback points out differences between the attempt and the desired result.* | Flash brief red divergence spark guiding instant corrective nudge back onto curve |
| `F3` | **práctica** (*practice*) | `[TIER-C]` | La repetición ajusta coordinación, velocidad, memoria y precisión. / *Repetition improves coordination, speed, memory and precision.* | Tighten scatter cloud over repeated orbit laps until trace locks razor-sharp |
| `F4` | **aprendizaje guiado** (*guided learning*) | `[TIER-C]` | Una persona experta puede aumentar gradualmente la dificultad y retirar ayuda. / *A skilled person can gradually increase the difficulty and withdraw help.* | Fade supportive outer golden guide-rail rings gradually as novice mastery rises |
| `F5` | **explicación** (*explanation*) | `[TIER-C]` | Las razones y principios permiten adaptar una habilidad a situaciones nuevas. / *Reasons and principles allow a skill to be adapted to new situations.* | Reveal underlying structural principle triangle beneath pure motor trajectory |
| `F6` | **evaluación** (*evaluation*) | `[TIER-C]` | Comprobar el resultado ayuda a distinguir dominio real de familiaridad. / *Checking the result helps distinguish real mastery from familiarity.* | Pass completed artifact cleanly through calibration ring checking real mastery |
| `F7` | **reciprocidad** (*reciprocity*) | `[TIER-C]` | Quien aprende hoy puede enseñar mañana y ampliar el conocimiento recibido. / *Whoever learns today can teach tomorrow and expand the knowledge received.* | Rotate former novice node upward into mentor seat illuminating fresh apprentice node |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP18-Z00-04)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP18-Z00-04` (Shared Knowledge Origin, Gesture, Voice & Cultural Teaching) for HyperCosmosVR Odyssey Act V (`Exp 18: 18_culture`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z00`->`2800` (`F0..F7`), `Z01`->`2801` (`F0..F7`), `Z02`->`2802` (`F0..F7`), `Z03`->`2803` (`F0..F7`), `Z04`->`2804` (`F0..F7`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3790`, `3791`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP18-Z00-04`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Specialize `select_compute_mode` (`crates/cosmos_render/src/universe_scene.rs`) + `shaders/compute_universe.wgsl` (`submode`/mode split) so early zones do not display anachronistic `mode=14` optical-bus particles.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z00: shared-knowledge / Shared knowledge, Z01: communication / Communication, Z02: language / Shared language, Z03: stories / Stories, Z04: teaching / Teaching.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp18-z05-09"></a>
### PKG-ACT5-EXP18-Z05-09: Cooperation, Social Norms, Composite Tools, Art & Ritual Exchange

- **Phase:** Phase 1/2 | **Priority:** `P1` | **Scope:** Exp `18` (`18_culture`) `Z05..Z09` (`5` zones, `40` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L64`: `180 + (sc.unwrap_or(z) % 7)` (`180..=186`); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L165-L204` (`ch=0..5`: 3 base rings + campfire/ziggurat/dome/astrolabe/globe; `feat` & `cue` unused). Tier breakdown for this batch: **`0A / 0B / 3C / 2D` Zones**, **`0A / 0B / 21C / 19D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `0/5` zones and `0/40` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 2805..=2809` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3792`, `3793`, `3794` (allocated from `3720 + (18 - 11)*10 + k`; Embodied Tool & Exchange Swarm (`mode=14` replacement)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z05` | `cooperation` (*Cooperación* / *Cooperation*) | `[TIER-C]` | `vid=185` | **`2805`** | `3792` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x02` (0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |
| `Z06` | `norms` (*Normas y confianza* / *Norms and trust*) | `[TIER-C]` | `vid=186` | **`2806`** | `3793` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z07` | `tools` (*Herramientas* / *Tools*) | `[TIER-D]` | `vid=180` | **`2807`** | `3794` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z08` | `art` (*Arte y simbolismo* / *Art and symbolism*) | `[TIER-D]` | `vid=181` | **`2808`** | `3792` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z09` | `exchange` (*Intercambio* / *Exchange*) | `[TIER-C]` | `vid=182` | **`2809`** | `3793` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z05` `cooperation` (`vid=2805`) — Cooperación (*Cooperation*) `[TIER-C]`

- **Catalog Metadata:** `scene=5` · `[0.250, 0.300]` · `rgb=[82, 214, 159]` · `sound=(triangle/78.5Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Roles + comunicación + confianza → capacidad colectiva` / *`Roles + communication + trust → collective capacity`* · Evidence: `Coordinación hacia metas compartidas` (*Coordination towards shared goals*)
- **Spoken Macro Narrative (ES / EN):** La cooperación combina capacidades individuales para alcanzar resultados imposibles o muy costosos en solitario. (*EN:* Cooperation combines individual capabilities to achieve results that are impossible or very costly alone.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (5 % 7) = 185` (`ch=5`, armillary tori!). Shows two empty brass rings instead of coordinated division of labor in hunting, foraging, building, defense, navigation, and shared food planning. **Target (`vid=2805`):** **Coordinated Multi-Agent Role Swarm**: 6 role-tinted agent beacons arrayed around shared objective/megalith block connected by coordinated tracking, load-bearing, perimeter watch, and wayfinding arcs.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **caza** (*hunting*) | `[TIER-D]` | La coordinación puede distribuir rastreo, conducción, captura y reparto. / *Coordination can distribute tracking, driving, capture and delivery.* | Show converging flanking pursuit arcs encircling moving prey marker across terrain disc |
| `F1` | **recolección** (*collection*) | `[TIER-C]` | Compartir información sobre lugares y estaciones reduce riesgo y esfuerzo. / *Sharing information about places and seasons reduces risk and effort.* | Link 4 scout nodes broadcasting seasonal resource beacons back to central camp |
| `F2` | **cuidado** (*care*) | `[TIER-C]` | El cuidado cooperativo amplía protección, aprendizaje y supervivencia infantil. / *Cooperative care expands child protection, learning and survival.* | Form protective concentric shield ring cradling vulnerable inner juvenile orbs |
| `F3` | **construcción** (*construction*) | `[TIER-D]` | Varias personas pueden mover materiales y mantener tareas simultáneas. / *Multiple people can move materials and maintain simultaneous tasks.* | Converge multi-agent tension cables jointly hoisting massive megalith/beam block |
| `F4` | **defensa** (*defense*) | `[TIER-C]` | La vigilancia y respuesta coordinadas cambian la capacidad de protección. / *Coordinated surveillance and response changes the ability to protect.* | Sweep alternating directional sentinel cones scanning 360-degree outer horizon |
| `F5` | **navegación** (*navigation*) | `[TIER-D]` | Rutas largas exigen memoria distribuida, observación y reparto de responsabilidades. / *Long routes require distributed memory, observation and sharing of responsibilities.* | Project multi-observer star-chart + horizon current vector steering canoe beacon |
| `F6` | **alimentos** (*food*) | `[TIER-C]` | Compartir reservas puede compensar días de éxito desigual y sostener vínculos. / *Sharing food reserves can offset days of uneven success and sustain social bonds.* | Equalize variable node reservoirs smoothly via shared radial distribution pipes |
| `F7` | **planificación** (*planning*) | `[TIER-C]` | Una meta futura requiere acordar tiempos, recursos, roles y contingencias. / *A future goal requires agreeing on times, resources, roles and contingencies.* | Draw synchronized multi-lane timeline bar coordinating sequential team arrivals |

##### `Z06` `norms` (`vid=2806`) — Normas y confianza (*Norms and trust*) `[TIER-C]`

- **Catalog Metadata:** `scene=6` · `[0.300, 0.350]` · `rgb=[247, 183, 98]` · `sound=(sine/82.6Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Repetición + reputación + sanciones + reparación → cooperación estable` / *`Repetition + reputation + sanctions + repair → stable cooperation`* · Evidence: `Expectativas sociales negociadas` (*Negotiated social expectations*)
- **Spoken Macro Narrative (ES / EN):** Las sociedades dependen no solo de la fuerza, sino también de expectativas compartidas sobre cómo deben comportarse las personas. (*EN:* Societies depend not only on force, but also on shared expectations about how people should behave.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (6 % 7) = 186` (`ch=6`, empty planetary lat/lon wireframe globe!). Micro-scale social norms, trust, reputation, sanctions, forgiveness, and leadership belong to interpersonal social topology, not an empty planet sphere. **Target (`vid=2806`):** **Social Trust Ledger & Balance Equilibrium Graph**: circular community ring of peer spheres linked by variable-thickness golden trust chords, central tilted balance beam, sanction exclusion wave, and repair suture.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **promesa** (*promise*) | `[TIER-C]` | Una promesa conecta una acción futura con la reputación presente. / *A promise connects a future action with present reputation.* | Draw glowing forward-timeline pledge chord anchoring present node to future promise ring |
| `F1` | **reciprocidad** (*reciprocity*) | `[TIER-C]` | Ayudar y recibir ayuda puede crear relaciones duraderas, no intercambios instantáneos. / *Helping and receiving help can create lasting relationships, not instant exchanges.* | Pulse asymmetric delayed-return counter-arc maintaining long-term balance across peers |
| `F2` | **reputación** (*reputation*) | `[TIER-C]` | La memoria social influye en quién recibe confianza, oportunidades o vigilancia. / *Social memory influences who receives trust, opportunities, or surveillance.* | Modulate node halo aura brightness dynamically based on witnessed peer memory |
| `F3` | **justicia** (*justice*) | `[TIER-C]` | Las ideas de reparto justo varían según contexto, necesidad, mérito y tradición. / *Ideas of fair distribution vary according to context, need, merit and tradition.* | Tilt central distribution scale comparing parity, need, and merit allocation weights |
| `F4` | **sanción** (*sanction*) | `[TIER-C]` | Las sanciones pueden desalentar abusos, pero también ser desiguales o injustas. / *Sanctions can discourage abuses, but they can also be unequal or unfair.* | Zap violating outer node with temporary amber deterrence ring dimming its links |
| `F5` | **perdón** (*forgiveness*) | `[TIER-C]` | La reparación y el perdón permiten continuar una relación después del daño. / *Repair and forgiveness allow a relationship to continue after damage.* | Fuse fractured red break seam with warm healing green bridge restoring flow |
| `F6` | **liderazgo** (*leadership*) | `[TIER-C]` | El liderazgo puede coordinar, representar o imponer; sus formas no son universales. / *Leadership can coordinate, represent or impose; Its forms are not universal.* | Contrast rotating hub coordinator vs rigid top-down monolithic crown spike |
| `F7` | **conflicto** (*conflict*) | `[TIER-C]` | Mediación, debate, compensación y separación son respuestas culturales distintas al conflicto. / *Mediation, debate, compensation and separation are different cultural responses to conflict.* | Show two clashing sub-clusters bridged alternately by mediator sphere or clean gap |

##### `Z07` `tools` (`vid=2807`) — Herramientas (*Tools*) `[TIER-D]`

- **Catalog Metadata:** `scene=7` · `[0.350, 0.400]` · `rgb=[194, 149, 92]` · `sound=(triangle/86.69999999999999Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Material + diseño + técnica → acción ampliada` / *`Material + design + technique → expanded action`* · Evidence: `Capacidades corporales ampliadas y acumulativas` (*Expanded and cumulative bodily capabilities*)
- **Spoken Macro Narrative (ES / EN):** Las herramientas conservan decisiones de diseño y permiten que mejoras pequeñas se acumulen durante generaciones. (*EN:* Tools preserve design decisions and allow small improvements to accumulate over generations.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (7 % 7) = 180` (`ch=0`, campfire flame!). Narrates tangible prehistoric/early crafted tools—knapped stone blade, braided rope, eyed needle, ceramic container, hafted hammer, graduated ruler—completely absent. **Target (`vid=2807`):** **Procedural Archaeological Tool Carousel**: crisp 3D SDF silhouettes of bifacial stone blade, helical 3-ply twisted cordage, pierced needle threading fiber, lathe ceramic amphora, hafted hammer, and notched measuring bar.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **piedra** (*stone*) | `[TIER-D]` | Un borde tallado concentra fuerza para cortar, raspar o perforar. / *A carved edge concentrates force for cutting, scraping or piercing.* | Sculpt sharp bifacial knapped flint edge concentrating downward cutting force plane |
| `F1` | **cuerda** (*rope*) | `[TIER-D]` | Fibras trenzadas permiten atar, arrastrar, tejer, medir y construir. / *Braided fibers allow for tying, dragging, weaving, measuring and building.* | Braid three helical fiber strands tightly under tensile arrow pull |
| `F2` | **aguja** (*needle*) | `[TIER-D]` | Una punta perforada convierte hilo y piel o tejido en ropa, refugio y reparación. / *A pierced tip turns thread and fur or fabric into clothing, shelter and repair.* | Pass fine pierced bone needle drawing continuous glowing thread seam |
| `F3` | **recipiente** (*container*) | `[TIER-D]` | Contener transforma el transporte, almacenamiento, cocción y reparto. / *Containing transforms transportation, storage, cooking and delivery.* | Revolve hollow terracotta storage amphora holding glowing grain/liquid reservoir |
| `F4` | **martillo** (*hammer*) | `[TIER-D]` | Una masa con mango amplifica impacto y protege la mano. / *A mass with handle amplifies impact and protects the hand.* | Swing dense stone/metal hammer head atop levered wooden shaft amplifying impact |
| `F5` | **medición** (*measurement*) | `[TIER-D]` | Marcas, pesos y reglas hacen comparables longitudes, cantidades y resultados. / *Marks, weights and rulers make lengths, quantities and results comparable.* | Lay graduated notched measuring rod alongside standardized counterweight blocks |
| `F6` | **almacenamiento** (*storage*) | `[TIER-D]` | Guardar herramientas y materiales extiende proyectos más allá de un solo momento. / *Saving tools and materials extends projects beyond a single moment.* | Nest crafted tools neatly inside protective covered cache chamber |
| `F7` | **mejora acumulativa** (*cumulative improvement*) | `[TIER-D]` | Una herramienta nueva suele combinar formas, materiales y técnicas heredadas. / *A new tool usually combines shapes, materials and inherited techniques.* | Morph stone blade + cordage + levered shaft seamlessly into composite hafted axe |

##### `Z08` `art` (`vid=2808`) — Arte y simbolismo (*Art and symbolism*) `[TIER-D]`

- **Catalog Metadata:** `scene=8` · `[0.400, 0.450]` · `rgb=[224, 118, 255]` · `sound=(sawtooth/90.8Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Forma + ritmo + materia + contexto → significado cultural` / *`Form + rhythm + matter + context → cultural meaning`* · Evidence: `Expresión, emoción e identidad` (*Expression, emotion and identity*)
- **Spoken Macro Narrative (ES / EN):** El arte puede comunicar emoción, identidad, memoria y significado sin funcionar como una descripción literal. (*EN:* Art can communicate emotion, identity, memory, and meaning without functioning as literal description.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (8 % 7) = 181` (`ch=1`, Mesopotamian ziggurat!). Narrates concrete artistic forms—musical instruments, dancing figures, cave pigments/painting, ornamental craft, carved sculpture, ceremonies—completely absent. **Target (`vid=2808`):** **Expressive Arts & Acoustic Sculpture Gallery**: plucked resonant lyre frame emitting cymatic harmonic rings, rotating kinetic dancer silhouettes, ochre/cobalt cave wall stencil relief, carved Venus/totem sculpture, and ceremonial totem orbit.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **música** (*music*) | `[TIER-D]` | El sonido organizado coordina atención, emoción, memoria y movimiento. / *Organized sound coordinates attention, emotion, memory and movement.* | Pluck glowing golden strings inside curved lyre arch emitting cymatic standing rings |
| `F1` | **danza** (*dance*) | `[TIER-D]` | El cuerpo puede representar vínculos, historias, celebración y transformación. / *The body can represent bonds, stories, celebration and transformation.* | Animate graceful rhythmic ring of synchronized multi-limb geometric dancers |
| `F2` | **pintura** (*painting*) | `[TIER-D]` | Pigmentos y superficies convierten gestos en huellas visibles duraderas. / *Pigments and surfaces turn gestures into long-lasting visible traces.* | Project warm ochre & charcoal mineral pigments onto textured rocky alcove relief |
| `F3` | **decoración** (*decoration*) | `[TIER-D]` | Modificar objetos cotidianos puede señalar cuidado, identidad, rango o pertenencia. / *Modifying everyday objects can signal care, identity, rank, or belonging.* | Engrave luminous geometric zigzags and bead collars around everyday ceramic pot |
| `F4` | **escultura** (*sculpture*) | `[TIER-D]` | La materia adquiere formas simbólicas que pueden ser tocadas y rodeadas. / *Matter takes on symbolic forms that can be touched and surrounded.* | Orbit sculptural 3D organic totem carving illuminated under shifting directional spotlight |
| `F5` | **ceremonia** (*ceremony*) | `[TIER-D]` | Acciones repetidas en tiempos especiales crean marcos compartidos de significado. / *Repeated actions at special times create shared frameworks of meaning.* | Synchronize concentric rhythmic procession arcs inside sacred ceremonial circle |
| `F6` | **símbolo** (*symbol*) | `[TIER-C]` | Una forma puede representar algo ausente, abstracto o socialmente acordado. / *A shape can represent something absent, abstract or socially agreed upon.* | Hover abstract radiant geometric emblem detached from literal object below |
| `F7` | **interpretación** (*interpretation*) | `[TIER-C]` | Una obra puede sostener múltiples significados y cambiar según quien la encuentre. / *A work can hold multiple meanings and change depending on who finds it.* | Split one central sculpture into three differently tinted subjective observer silhouettes |

##### `Z09` `exchange` (`vid=2809`) — Intercambio (*Exchange*) `[TIER-C]`

- **Catalog Metadata:** `scene=9` · `[0.450, 0.500]` · `rgb=[89, 202, 224]` · `sound=(sine/94.9Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Objetos + rutas + encuentros → transferencia material y simbólica` / *`Objects + routes + encounters → material and symbolic transfer`* · Evidence: `Contacto entre grupos y circulación cultural` (*Contact between groups and cultural circulation*)
- **Spoken Macro Narrative (ES / EN):** Cuando los grupos intercambian objetos, también pueden circular palabras, técnicas, cultivos, músicas, creencias, genes y enfermedades. (*EN:* When groups exchange objects, words, techniques, crops, music, beliefs, genes, and diseases may also circulate.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (9 % 7) = 182` (`ch=2`, exact same 2-tier ziggurat!). Inter-regional barter, gift-giving, long-distance obsidian/salt/shell routes, technology transfer, loanwords, migration, and contagion risks aren't visible in a static temple. **Target (`vid=2809`):** **Inter-Group Exchange & Trade Route Diorama**: two spaced settlement clusters over undulating relief terrain linked by glowing caravan/river trails carrying tinted stone, salt, shell goods, linguistic echoes, and creeping contagion arcs.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **regalo** (*gift*) | `[TIER-C]` | Un regalo puede crear obligación, alianza, prestigio o cuidado sin fijar un precio. / *A gift can create obligation, alliance, prestige or care without setting a price.* | Pass ceremonial amber gift orb creating glowing prestige obligation ring without price tag |
| `F1` | **trueque** (*barter*) | `[TIER-C]` | El intercambio directo depende de necesidades coincidentes y valores negociados. / *Direct exchange depends on matching needs and negotiated values.* | Swap mismatched green/gold commodity crystals along central balanced negotiation bar |
| `F2` | **ruta** (*route*) | `[TIER-D]` | Los caminos conectan paisajes, estaciones, comunidades y zonas de producción. / *Roads connect landscapes, seasons, communities and production areas.* | Illuminate winding overland valley & river corridor tracks tying twin distant camps |
| `F3` | **material** (*material*) | `[TIER-D]` | Piedra, sal, metal, concha o pigmento pueden viajar mucho más lejos que quienes los obtuvieron. / *Stone, salt, metal, shell or pigment can travel much further than those who obtained them.* | Stream distinct obsidian, crystal-salt, copper, and shell tokens hundreds of units far |
| `F4` | **tecnología** (*technology*) | `[TIER-C]` | Un objeto importado puede revelar un diseño y ser adaptado localmente. / *An imported object can reveal a design and be adapted locally.* | Reverse-engineer imported spiral artifact locally into native duplicated copies |
| `F5` | **palabra** (*word*) | `[TIER-C]` | El contacto lingüístico produce préstamos, traducciones y nuevas formas de nombrar. / *Language contact produces loans, translations and new ways of naming.* | Blend two contrasting dialect aura rings into shared loanword bridge pulse |
| `F6` | **migración** (*migration*) | `[TIER-C]` | Las personas transportan prácticas completas, no solo mercancías. / *People transport entire practices, not just goods.* | Move whole mobile household cluster + tool rings smoothly across landscape corridor |
| `F7` | **riesgo** (*risk*) | `[TIER-C]` | Las redes también transmiten competencia, violencia, explotación y enfermedad. / *Networks also transmit competition, violence, exploitation and disease.* | Sneak subtle pulsing violet pathogen spore along bright gold trade caravan lines |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP18-Z05-09)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP18-Z05-09` (Cooperation, Social Norms, Composite Tools, Art & Ritual Exchange) for HyperCosmosVR Odyssey Act V (`Exp 18: 18_culture`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z05`->`2805` (`F0..F7`), `Z06`->`2806` (`F0..F7`), `Z07`->`2807` (`F0..F7`), `Z08`->`2808` (`F0..F7`), `Z09`->`2809` (`F0..F7`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3792`, `3793`, `3794`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP18-Z05-09`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Specialize `select_compute_mode` (`crates/cosmos_render/src/universe_scene.rs`) + `shaders/compute_universe.wgsl` (`submode`/mode split) so early zones do not display anachronistic `mode=14` optical-bus particles.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z05: cooperation / Cooperation, Z06: norms / Norms and trust, Z07: tools / Tools, Z08: art / Art and symbolism, Z09: exchange / Exchange.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp18-z10-14"></a>
### PKG-ACT5-EXP18-Z10-14: Agriculture, Sedentary Settlements, Cuneiform Writing, Cities & Institutions

- **Phase:** Phase 1/2 | **Priority:** `P1` | **Scope:** Exp `18` (`18_culture`) `Z10..Z14` (`5` zones, `40` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L64`: `180 + (sc.unwrap_or(z) % 7)` (`180..=186`); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L165-L204` (`ch=0..5`: 3 base rings + campfire/ziggurat/dome/astrolabe/globe; `feat` & `cue` unused). Tier breakdown for this batch: **`0A / 0B / 2C / 3D` Zones**, **`0A / 0B / 14C / 26D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `1/40` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 2810..=2814` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3795`, `3796` (allocated from `3720 + (18 - 11)*10 + k`; Hydraulic Settlement & Civic Ledger Field):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z10` | `agriculture` (*Agricultura* / *Agriculture*) | `[TIER-D]` | `vid=183` | **`2810`** | `3795` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 semantic cue mask) | `0..7` (`0xFF` macro) |
| `Z11` | `settlements` (*Asentamientos* / *Settlements*) | `[TIER-D]` | `vid=184` | **`2811`** | `3796` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z12` | `writing` (*Escritura* / *Writing*) | `[TIER-D]` | `vid=185` | **`2812`** | `3795` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z13` | `cities` (*Ciudades* / *Cities*) | `[TIER-C]` | `vid=186` | **`2813`** | `3796` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z14` | `institutions` (*Instituciones* / *Institutions*) | `[TIER-C]` | `vid=180` | **`2814`** | `3795` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z10` `agriculture` (`vid=2810`) — Agricultura (*Agriculture*) `[TIER-D]`

- **Catalog Metadata:** `scene=10` · `[0.500, 0.550]` · `rgb=[132, 205, 94]` · `sound=(sine/99Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `tierra`
- **Metric & Evidence (ES / EN):** `Selección + calendario + trabajo + almacenamiento → producción planificada` / *`Selection + calendar + work + storage → planned production`* · Evidence: `Transformación sostenida de ecosistemas` (*Sustained ecosystem transformation*)
- **Spoken Macro Narrative (ES / EN):** La agricultura surgió de manera independiente en varias regiones y produjo beneficios, costes y trayectorias diferentes; no fue una mejora inmediata en todo aspecto de la vida. (*EN:* Agriculture emerged independently in various regions and produced different benefits, costs and trajectories; It was not an immediate improvement in every aspect of life.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (10 % 7) = 183` (`ch=3`, smooth dome!). Agricultural transformation—plant selection, animal domestication, seasonal solar calendars, irrigation canals, granary silos, soil transformation, surpluses—has zero WGSL geometry. **Target (`vid=2810`):** **Neolithic Agricultural Basin & Terraces**: stepped contoured green-gold hillside terraces traversed by gleaming blue irrigation canals feeding waving crop rows beside earthen granary silos and solar calendar stone dial.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **selección vegetal** (*plant selection*) | `[TIER-D]` | Guardar semillas de ciertas plantas cambia gradualmente poblaciones cultivadas. / *Saving seeds from certain plants gradually changes cultivated populations.* | Morph wild slender grass stalks across 3 stages into plump heavy golden grain ears |
| `F1` | **domesticación animal** (*animal domestication*) | `[TIER-D]` | La cría selectiva modifica comportamiento, cuerpo y relación ecológica. / *Selective breeding modifies behavior, body and ecological relationship.* | Enclose tame herbivore herd silhouettes inside perimeter stockade ring |
| `F2` | **estaciones** (*seasons*) | `[TIER-D]` | Sembrar y cosechar requieren calendarios y observación climática. / *Planting and harvesting require calendars and weather observation.* | Sweep inclined solar horizon arc over calendar alignment stones marking flood/sow |
| `F3` | **riego** (*irrigation*) | `[TIER-D]` | Canales y depósitos redistribuyen agua, pero exigen mantenimiento y acuerdos. / *Canals and reservoirs redistribute water, but require maintenance and agreements.* | Cut branching blue water canals from river gorge evenly across stepped fields |
| `F4` | **almacenamiento** (*storage*) | `[TIER-D]` | Las reservas reducen estacionalidad y crean nuevos riesgos de pérdida o control desigual. / *Reserves reduce seasonality and create new risks of loss or unequal control.* | Stack domed cylindrical clay granary silos storing glowing golden harvest reserves |
| `F5` | **tierra** (*land*) `[kw:tierra]` | `[TIER-D]` | Cultivar transforma suelos, vegetación, biodiversidad y formas de propiedad. / *Cultivating transforms soils, vegetation, biodiversity and forms of property.* | Replace mixed wild canopy texture with rectangular geometric plowed field patchwork |
| `F6` | **excedente** (*surplus*) | `[TIER-C]` | La producción almacenada puede sostener especialización, comercio y poder. / *Stored production can sustain specialization, trade and power.* | Overflow glowing surplus grain orbs outward toward specialist craft/authority pedestals |
| `F7` | **costes** (*costs*) | `[TIER-C]` | Trabajo repetitivo, dieta limitada, enfermedad y desigualdad aumentaron en muchos contextos agrícolas. / *Repetitive work, limited diet, illness and inequality increased in many agricultural contexts.* | Contrast taller storehouses against subtle dimming soil erosion & stressed crowd ring |

##### `Z11` `settlements` (`vid=2811`) — Asentamientos (*Settlements*) `[TIER-D]`

- **Catalog Metadata:** `scene=11` · `[0.550, 0.600]` · `rgb=[227, 157, 93]` · `sound=(triangle/103.1Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Vivienda + agua + almacenamiento + caminos → comunidad estable` / *`Housing + water + storage + roads → stable community`* · Evidence: `Paisajes construidos y permanencia creciente` (*Built landscapes and increasing permanence*)
- **Spoken Macro Narrative (ES / EN):** Los asentamientos reúnen personas, infraestructuras y memoria material; no todos fueron agrícolas ni evolucionaron hacia ciudades. (*EN:* Settlements bring together people, infrastructure and material memory; Not all of them were agricultural or evolved into cities.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (11 % 7) = 184` (`ch=4`, astronomical armillary tori!). Explaining houses, storehouses, artisan workshops, streets, wells, and defensive walls with floating brass astrolabe rings leaves students seeing nothing architectural. **Target (`vid=2811`):** **Clustered Settlement Compound SDF**: compact Neolithic-to-Bronze courtyard dwellings around a central public plaza hearth, stone well/water cistern, pottery/forge workshop stalls, radial footpaths, and outer timber/mudbrick perimeter wall.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **viviendas** (*housing*) | `[TIER-D]` | Las casas organizan descanso, parentesco, trabajo, intimidad y almacenamiento. / *Houses organize rest, kinship, work, intimacy and storage.* | Extrude multi-room courtyard clay-and-timber dwellings glowing warmly from hearths |
| `F1` | **almacenes** (*warehouses*) | `[TIER-D]` | Reservas comunes o privadas cambian seguridad, poder y planificación. / *Common or private reserves change security, power and planning.* | Highlight reinforced communal & elite storage granary vaults alongside alleys |
| `F2` | **talleres** (*workshops*) | `[TIER-D]` | Espacios especializados concentran herramientas, materiales y aprendizaje. / *Specialized spaces concentrate tools, materials and learning.* | Ignite hot amber furnace & anvil stall clustered cleanly in artisan quarter |
| `F3` | **caminos** (*roads*) | `[TIER-D]` | El tránsito repetido convierte movimiento en infraestructura. / *Repeated transit turns movement into infrastructure.* | Etch worn branching dirt paths & stone lanes radiating outward from settlement gates |
| `F4` | **espacios comunes** (*common spaces*) | `[TIER-D]` | Plazas, fogones y salas compartidas sostienen reunión, ceremonia y decisión. / *Plazas, fireplaces and shared rooms hold meetings, ceremonies and decisions.* | Illuminate central open plaza circle ringing public meeting benches and hearth |
| `F5` | **defensas** (*defenses*) | `[TIER-D]` | Muros y posiciones protegidas responden a riesgos, pero también separan. / *Walls and protected positions respond to risks, but they also separate.* | Raise continuous crenellated outer wall & ditch perimeter enclosing settlement footprint |
| `F6` | **agua** (*water*) | `[TIER-D]` | Pozos, fuentes y canales condicionan tamaño, salud y ubicación. / *Wells, fountains and canals determine size, health and location.* | Drop central deep stone well shaft + spring canal feeding all residential blocks |
| `F7` | **división del trabajo** (*division of labor*) | `[TIER-C]` | La permanencia permite especialización, aunque las tareas siguen superponiéndose. / *Permanence allows specialization, although tasks continue to overlap.* | Tint distinct housing clusters by overlapping specialist craft emblems |

##### `Z12` `writing` (`vid=2812`) — Escritura (*Writing*) `[TIER-D]`

- **Catalog Metadata:** `scene=12` · `[0.600, 0.650]` · `rgb=[245, 218, 171]` · `sound=(sine/107.19999999999999Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Marcas convencionales + soporte + lectura → registro transmisible` / *`Conventional marks + medium + reading → transferable record`* · Evidence: `Información más allá de la memoria viva` (*Information beyond living memory*)
- **Spoken Macro Narrative (ES / EN):** La escritura permite almacenar mensajes con mayor precisión y transmitirlos a distancia y entre generaciones, pero las sociedades sin escritura también conservan conocimiento complejo. (*EN:* Writing allows messages to be stored with greater precision and transmitted over distance and between generations, but societies without writing also preserve complex knowledge.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (12 % 7) = 185` (`ch=5`, armillary tori!). Narrates concrete historical inscription media—clay accounting tokens, cuneiform wedge tablets, number tallies, inscribed law stelae, letters, archives, and scrolls—completely missing. **Target (`vid=2812`):** **Script Evolution & Clay Tablet Archive**: progression from embossed clay bulla tokens -> beveled wedge-impressed tablet -> monumental carved basalt law stele -> bound cylinder scrolls & cataloged archive shelves.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **signo** (*sign*) | `[TIER-D]` | Una marca se vuelve escritura cuando una comunidad aprende su relación con sonidos, palabras o ideas. / *A mark becomes writing when a community learns its relationship to sounds, words, or ideas.* | Map vocal sound/object icon onto crisp standardized cuneiform/glyph sign matrix |
| `F1` | **número** (*number*) | `[TIER-D]` | Registrar cantidades ayuda a contar personas, bienes, tiempo, deuda y medida. / *Recording quantities helps count people, goods, time, debt and measurement.* | Press repeating quantitative tally notches & base-10/60 positional bars cleanly |
| `F2` | **cuenta** (*account*) | `[TIER-D]` | Fichas y tablillas pueden preservar transacciones que nadie recuerda por completo. / *Tokens and tablets can preserve transactions that no one fully remembers.* | Seal miniature clay counting pebbles beside itemized wedge ledger tablet |
| `F3` | **nombre** (*name*) | `[TIER-D]` | Escribir un nombre mantiene referencia administrativa, familiar o ceremonial. / *Writing a name maintains administrative, family or ceremonial reference.* | Highlight framed royal/cartouche name seal stamped onto durable tablet corner |
| `F4` | **ley** (*law*) | `[TIER-D]` | Un texto fija una versión consultable, aunque interpretación y aplicación sigan siendo sociales. / *A text establishes a consultable version, although interpretation and application remain social.* | Erect tall dark incised law stele broadcasting fixed public code lines |
| `F5` | **carta** (*letter*) | `[TIER-D]` | El mensaje puede viajar sin que emisor y receptor compartan lugar o momento. / *The message can travel without the sender and receiver sharing a place or time.* | Send wax-sealed inscribed message tablet arcing smoothly across absent distance |
| `F6` | **archivo** (*archive*) | `[TIER-D]` | Colecciones organizadas permiten comparar registros y extender memoria institucional. / *Organized collections allow you to compare records and extend institutional memory.* | Arrange ordered grid shelves of glowing clay tablets linked by provenance indices |
| `F7` | **literatura** (*literature*) | `[TIER-D]` | La escritura puede conservar voces, argumentos y mundos imaginados durante siglos. / *Writing can preserve voices, plots, and imagined worlds for centuries.* | Unfurl glowing double-spooled parchment/papyrus scroll emitting holographic epic arc |

##### `Z13` `cities` (`vid=2813`) — Ciudades (*Cities*) `[TIER-C]`

- **Catalog Metadata:** `scene=13` · `[0.650, 0.700]` · `rgb=[255, 187, 82]` · `sound=(triangle/111.3Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Alimentos + agua + trabajo + administración + energía → sistema urbano` / *`Food + water + work + administration + energy → urban system`* · Evidence: `Densidad, especialización e interdependencia` (*Density, specialization and interdependence*)
- **Spoken Macro Narrative (ES / EN):** Una ciudad no es un solo invento: es una red dependiente de territorios, infraestructuras, trabajo, autoridad y flujos materiales. (*EN:* A city is not a single invention: it is a dependent network of territories, infrastructure, work, authority and material flows.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (13 % 7) = 186` (`ch=6`, empty wireframe globe!). Urban density—markets, artisan workshops, neighborhoods, aqueducts, administration, monuments—receives a naked lat/lon globe instead of the already-coded Ziggurat (`181/182`) + urban lattice! **Target (`vid=2813`):** **Ancient-to-Classical Urban Metropolis (Upgraded `181/182` Ziggurat Core)**: central stepped temple/monument surrounded by dense grid blocks, bustling market square, elevated aqueduct feeding fountain, walled street gates, and radiating agricultural hinterland spokes.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **mercado** (*market*) | `[TIER-D]` | Los mercados concentran intercambio, información, negociación y regulación. / *Markets concentrate exchange, information, negotiation and regulation.* | Highlight lively open market square pulsing with multi-origin exchange bursts |
| `F1` | **taller** (*workshop*) | `[TIER-C]` | La proximidad de especialistas acelera producción, aprendizaje y competencia. / *The proximity of specialists accelerates production, learning and competition.* | Cluster tight artisan furnace/loom workshops along concentrated craft lane |
| `F2` | **vivienda** (*housing*) | `[TIER-C]` | La densidad genera nuevas formas de vecindad, desigualdad, riesgo y oportunidad. / *Density generates new forms of neighborhood, inequality, risk and opportunity.* | Densify multi-story housing blocks illustrating crowded urban stratification |
| `F3` | **camino** (*path*) | `[TIER-C]` | Calles y puertas organizan acceso, movimiento, defensa y comercio. / *Streets and doors organize access, movement, defense and commerce.* | Trace axial colonnaded boulevard & fortified city gates routing incoming wagons |
| `F4` | **agua y saneamiento** (*water and sanitation*) | `[TIER-D]` | Abastecimiento y eliminación de residuos condicionan salud y crecimiento. / *Supply and elimination of waste determine health and growth.* | Arch gleaming stone aqueduct bridge pouring clean water down drainage conduits |
| `F5` | **administración** (*administration*) | `[TIER-C]` | Contar, registrar y coordinar se vuelve más exigente con poblaciones densas. / *Counting, recording and coordinating becomes more demanding with dense populations.* | Beam scribe tax/census tallies upward into central administrative palace |
| `F6` | **monumento** (*monument*) | `[TIER-D]` | Edificios visibles concentran memoria, culto, autoridad o identidad. / *Visible buildings concentrate memory, worship, authority or identity.* | Elevate monumental central stepped ziggurat / marble temple towering over skyline |
| `F7` | **dependencia regional** (*regional dependency*) | `[TIER-C]` | Toda ciudad necesita alimentos, materiales y energía procedentes de otros lugares. / *Every city needs food, materials and energy from other places.* | Extend long glowing supply lines outward pulling grain/timber/ore from distant hinterland |

##### `Z14` `institutions` (`vid=2814`) — Instituciones (*Institutions*) `[TIER-C]`

- **Catalog Metadata:** `scene=14` · `[0.700, 0.750]` · `rgb=[112, 164, 245]` · `sound=(sine/115.39999999999999Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Roles + normas + registros + recursos → continuidad organizada` / *`Roles + rules + records + resources → organized continuity`* · Evidence: `Reglas y funciones que sobreviven a sus miembros` (*Rules and functions that survive its members*)
- **Spoken Macro Narrative (ES / EN):** Una institución conserva reglas, funciones y memoria más allá de las personas que la componen en un momento determinado. (*EN:* An institution preserves rules, functions and memory beyond the people who make it up at a given time.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (14 % 7) = 180` (`ch=0`, primitive campfire flame!!). Law courts, schools, standing armies, religious complexes, tax collection, and state public works routed back to a Stone-Age campfire is glaringly out of epoch. **Target (`vid=2814`):** **Institutional Colonnade & Role Architecture**: circular multi-pillar civic hall framing rotating balance scales (court), tiered lecture bench (school), fortified shield ring (army), sacred sanctuary (temple), tax coffers, and aqueduct public works.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **consejo** (*council*) | `[TIER-C]` | Un consejo distribuye deliberación y autoridad entre personas designadas o reconocidas. / *A council distributes deliberation and authority among designated or recognized persons.* | Seat balanced semicircle council halo of glowing deliberative nodes |
| `F1` | **tribunal** (*court*) | `[TIER-C]` | Los tribunales formalizan procedimientos para conflicto, prueba y sanción. / *Courts formalize procedures for conflict, evidence and sanction.* | Suspend crisp illuminated balance scales & evidence bridge inside civic tribunal |
| `F2` | **escuela** (*school*) | `[TIER-C]` | La enseñanza se vuelve una función sostenida con currículos, roles y evaluación. / *Teaching becomes a sustained function with curricula, roles and assessment.* | Arrange tiered concentric academy steps passing illuminated tablets to novices |
| `F3` | **ejército** (*army*) | `[TIER-D]` | La defensa organizada concentra entrenamiento, logística y poder coercitivo. / *Organized defense concentrates training, logistics and coercive power.* | Deploy disciplined phalanx/shield perimeter grid enforcing coercive defense lines |
| `F4` | **organización religiosa** (*religious organization*) | `[TIER-C]` | Rituales, especialistas y lugares pueden persistir mediante normas y archivos. / *Rituals, specialists, and places can persist through norms and archives.* | Glow sacred inner sanctum dome + ritual archive preserving enduring priesthood norms |
| `F5` | **impuestos** (*taxes*) | `[TIER-C]` | Recaudar recursos financia proyectos y autoridad, pero distribuye cargas de manera desigual. / *Raising resources funds projects and authority, but distributes burdens unevenly.* | Channel tributary grain/coin beams inward from provinces to treasury vault |
| `F6` | **obras públicas** (*public works*) | `[TIER-D]` | Caminos, canales y servicios coordinan trabajo a escalas mayores. / *Roads, canals and services coordinate work on larger scales.* | Span grand monumental canal, stone bridge & highway public-works ribbons |
| `F7` | **poder** (*power*) | `[TIER-C]` | Las instituciones pueden proteger bienes comunes o privilegiar a ciertos grupos; suelen hacer ambas cosas en grados distintos. / *Institutions can protect common goods or privilege certain groups; They usually do both to different degrees.* | Cast dual golden protective canopy vs asymmetric heavy shadow beam down on periphery |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP18-Z10-14)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP18-Z10-14` (Agriculture, Sedentary Settlements, Cuneiform Writing, Cities & Institutions) for HyperCosmosVR Odyssey Act V (`Exp 18: 18_culture`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z10`->`2810` (`F0..F7`), `Z11`->`2811` (`F0..F7`), `Z12`->`2812` (`F0..F7`), `Z13`->`2813` (`F0..F7`), `Z14`->`2814` (`F0..F7`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3795`, `3796`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP18-Z10-14`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Specialize `select_compute_mode` (`crates/cosmos_render/src/universe_scene.rs`) + `shaders/compute_universe.wgsl` (`submode`/mode split) so early zones do not display anachronistic `mode=14` optical-bus particles.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z10: agriculture / Agriculture, Z11: settlements / Settlements, Z12: writing / Writing, Z13: cities / Cities, Z14: institutions / Institutions.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp18-z15-19"></a>
### PKG-ACT5-EXP18-Z15-19: Long-Distance Commerce, Empirical Science, Industrial Engines & Planetary Web

- **Phase:** Phase 1/2 | **Priority:** `P1` | **Scope:** Exp `18` (`18_culture`) `Z15..Z19` (`5` zones, `40` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L64`: `180 + (sc.unwrap_or(z) % 7)` (`180..=186`); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L165-L204` (`ch=0..5`: 3 base rings + campfire/ziggurat/dome/astrolabe/globe; `feat` & `cue` unused). Tier breakdown for this batch: **`0A / 0B / 3C / 2D` Zones**, **`0A / 0B / 17C / 23D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `0/40` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 2815..=2819` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3797`, `3798`, `3799` (allocated from `3720 + (18 - 11)*10 + k`; Maritime/Empirical transitioning into `mode=14` Optical/Satcom Web (`Z18..Z19`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z15` | `commerce` (*Comercio y redes* / *Commerce and networks*) | `[TIER-D]` | `vid=181` | **`2815`** | `3797` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z16` | `science` (*Ciencia* / *Science*) | `[TIER-C]` | `vid=182` | **`2816`** | `3798` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x02` (0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |
| `Z17` | `industry` (*Industria* / *Industry*) | `[TIER-D]` | `vid=183` | **`2817`** | `3799` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z18` | `global-networks` (*Redes globales* / *Global networks*) | `[TIER-C]` | `vid=184` | **`2818`** | `3797` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z19` | `planetary-civilization` (*Civilización planetaria* / *planetary civilization*) | `[TIER-C]` | `vid=185` | **`2819`** | `3798` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 semantic cue mask) | `0..7` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z15` `commerce` (`vid=2815`) — Comercio y redes (*Commerce and networks*) `[TIER-D]`

- **Catalog Metadata:** `scene=15` · `[0.750, 0.800]` · `rgb=[95, 216, 199]` · `sound=(triangle/119.5Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Rutas + contratos + medios de pago + logística → intercambio ampliado` / *`Routes + contracts + means of payment + logistics → expanded exchange`* · Evidence: `Sistemas regionales de circulación` (*Regional circulation systems*)
- **Spoken Macro Narrative (ES / EN):** Las redes comerciales conectan lugares distantes y vuelven cada objeto parte de una historia material, técnica y social más extensa. (*EN:* Commercial networks connect distant places and make each object part of a longer material, technical and social history.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (15 % 7) = 181` (`ch=1`, static ziggurat!). Regional trade networks require highways, harbors/vessels, caravans, coinage, contracts, celestial navigation, and supply chains—zero maritime/logistics geometry exists. **Target (`vid=2815`):** **Regional Maritime & Caravan Network Map**: elevated coastline terrain slab with paved road artery, seaport jetty + ship silhouette, overland caravan train, stamped metal coins, wax-sealed contract scroll, and sextant/star navigation arc.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **carretera** (*highway*) | `[TIER-D]` | Una ruta mantenida reduce fricción y hace previsibles tiempos y cargas. / *A maintained route reduces friction and makes times and loads predictable.* | Lay paved stone highway cutting frictionless straight path across rough relief hills |
| `F1` | **puerto** (*port*) | `[TIER-D]` | Los puertos conectan transporte terrestre, fluvial y marítimo. / *Ports connect land, river and sea transportation.* | Sculpt natural harbor bay jetty transferring cargo crates from hulls to wagons |
| `F2` | **caravana** (*caravan*) | `[TIER-D]` | Viajar en grupo distribuye defensa, animales, guías y mercancías. / *Traveling in a group distributes defense, animals, guides and merchandise.* | Animate linked pack-animal caravan ribbon crossing arid saddle pass |
| `F3` | **moneda** (*currency*) | `[TIER-D]` | Una unidad reconocida facilita comparación y pago, pero depende de confianza e instituciones. / *A recognized unit facilitates comparison and payment, but it depends on trust and institutions.* | Spin gleaming stamped gold/silver coin minted under central institutional authority seal |
| `F4` | **contrato** (*contract*) | `[TIER-D]` | Los acuerdos registrados extienden obligaciones más allá del encuentro inmediato. / *Registered agreements extend obligations beyond the immediate encounter.* | Connect distant cities via glowing sealed contract strip binding future delivery |
| `F5` | **navegación** (*navigation*) | `[TIER-D]` | Mapas, estrellas, instrumentos y experiencia convierten distancia en ruta. / *Maps, stars, instruments and experience turn distance into route.* | Align astrolabe star-sighting vector & compass rose onto sea-lane arc |
| `F6` | **cadena de suministro** (*supply chain*) | `[TIER-C]` | Un producto combina extracción, transformación, transporte, venta y consumo. / *A product combines extraction, transformation, transportation, sale and consumption.* | Trace multi-hop emerald->gold->blue raw->crafted->sold value-chain pipeline |
| `F7` | **interdependencia** (*interdependence*) | `[TIER-C]` | La especialización aumenta capacidad y también vulnerabilidad ante interrupciones. / *Specialization increases capacity and also vulnerability to interruptions.* | Snap one distant port offline causing cascading cargo stalls across connected hubs |

##### `Z16` `science` (`vid=2816`) — Ciencia (*Science*) `[TIER-C]`

- **Catalog Metadata:** `scene=16` · `[0.800, 0.850]` · `rgb=[110, 225, 255]` · `sound=(sine/123.6Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Observación + medición + hipótesis + prueba + crítica → explicación revisable` / *`Observation + measurement + hypothesis + test + criticism → reviewable explanation`* · Evidence: `Corrección pública y conocimiento comprobable` (*Public correction and verifiable knowledge*)
- **Spoken Macro Narrative (ES / EN):** La ciencia no es una colección fija de respuestas. Es un sistema social para detectar errores y mejorar explicaciones. (*EN:* Science is not a fixed collection of answers. It is a social system to detect errors and improve explanations.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (16 % 7) = 182` (`ch=2`, Mesopotamian ziggurat!). Science is collective error-correcting inquiry with precision instruments, controlled trials, mathematics, and replication—yet astronomical astrolabe (`184/185`) sat elsewhere while science got a temple! **Target (`vid=2816`):** **Observatory & Controlled Experiment Crucible (`184/185` Armillary Upgrade)**: brass armillary astrolabe paired with optical lens/balance calibration rig, side-by-side control-vs-treatment experiment tubes, geometric equation prism, and peer-falsification loop.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **observación** (*observation*) | `[TIER-C]` | Observar sistemáticamente exige definir qué cuenta, cómo se registra y qué podría haberse omitido. / *Observing systematically requires defining what counts, how it is recorded, and what might have been left out.* | Focus dual collimated observation beams contrasting blind spots vs recorded hits |
| `F1` | **medición** (*measurement*) | `[TIER-D]` | Las unidades e instrumentos hacen resultados comparables entre lugares y personas. / *The units and instruments make results comparable between places and people.* | Display calibrated Vernier ruler, brass balance & pendulum clock standardizing units |
| `F2` | **hipótesis** (*hypothesis*) | `[TIER-C]` | Una hipótesis propone una relación que puede enfrentarse a evidencia. / *A hypothesis proposes a relationship that can be tested against evidence.* | Project crisp wireframe hypothesis manifold poised tentatively over experimental bench |
| `F3` | **experimento** (*experiment*) | `[TIER-D]` | Modificar condiciones ayuda a distinguir causas de coincidencias. / *Modifying conditions helps distinguish causes of coincidences.* | Run twin parallel glass apparatus holding control constant while perturbing one parameter |
| `F4` | **matemáticas** (*mathematics*) | `[TIER-C]` | La representación matemática comprime patrones y produce predicciones precisas. / *Mathematical representation compresses patterns and produces accurate predictions.* | Spin shimmering geometric proof & predictive parabolic trajectory over observations |
| `F5` | **replicación** (*replication*) | `[TIER-C]` | Repetir un procedimiento comprueba si un resultado depende del observador o del azar. / *Repeating a procedure tests whether a result depends on the observer or chance.* | Repeat identical experimental pulse simultaneously across three separated lab hubs |
| `F6` | **crítica** (*criticism*) | `[TIER-C]` | La revisión pública busca errores de método, inferencia y evidencia. / *The public review looks for errors of method, inference, and evidence.* | Fire crimson peer-critique laser probing fragile weak seam on central hypothesis dome |
| `F7` | **revisión** (*review*) | `[TIER-C]` | Una explicación científica fuerte puede cambiar cuando aparecen mejores datos o modelos. / *A strong scientific explanation can change when better data or models appear.* | Shatter obsolete cracked outer dome cleanly revealing tighter refined inner theory |

##### `Z17` `industry` (`vid=2817`) — Industria (*Industry*) `[TIER-D]`

- **Catalog Metadata:** `scene=17` · `[0.850, 0.900]` · `rgb=[255, 139, 75]` · `sound=(triangle/127.69999999999999Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Máquinas + combustibles o electricidad + estandarización → gran escala productiva` / *`Machines + fuel or electricity + standardization → large productive scale`* · Evidence: `Máquinas, energía concentrada y producción masiva` (*Machines, concentrated energy and mass production*)
- **Spoken Macro Narrative (ES / EN):** La industrialización multiplicó capacidad productiva y transformó trabajo, ciudades, población, desigualdad y sistemas ambientales. (*EN:* Industrialization multiplied productive capacity and transformed work, cities, population, inequality and environmental systems.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (17 % 7) = 183` (`ch=3`, smooth dome!). Industrialization's concrete icons—steam/electric machines, factory assembly, interchangeable parts, railways, electrical transmission, urbanization, smokestack impact—are missing from Exp 18! **Target (`vid=2817`):** **Industrial Mechanism & Rail Diorama**: meshing drive gears & reciprocating steam flywheel (`190`-style mechanical kernel shared cleanly), assembly line conveyor, railway viaduct + locomotive trace, electric transmission tower, and atmospheric plume.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **máquina** (*machine*) | `[TIER-D]` | Mecanismos repetibles convierten energía en movimientos controlados. / *Repeatable mechanisms convert energy into controlled movements.* | Mesh heavy cast-iron crankshaft flywheel converting thermal steam into steady torque |
| `F1` | **fábrica** (*factory*) | `[TIER-D]` | Reunir trabajadores, máquinas y materiales sincroniza producción y disciplina laboral. / *Bringing together workers, machines and materials synchronizes production and work discipline.* | Synchronize assembly conveyor bay inside rectilinear sawtooth-roof factory hall |
| `F2` | **estandarización** (*standardization*) | `[TIER-D]` | Piezas y medidas comunes permiten reparación, intercambio y fabricación en serie. / *Common parts and measurements allow for repair, exchange and mass production.* | Snap uniform calibrated bolt & gear blanks interchangeably onto precision gauge |
| `F3` | **ferrocarril** (*railway*) | `[TIER-D]` | El transporte mecanizado comprime tiempo, distancia y coste logístico. / *Mechanized transport compresses time, distance and logistical cost.* | Streak glowing locomotive train along twin steel railway tracks bridging hills |
| `F4` | **electricidad** (*electricity*) | `[TIER-D]` | La energía eléctrica puede generarse lejos, distribuirse y convertirse en luz, movimiento o información. / *Electrical energy can be generated far away, distributed and converted into light, movement or information.* | Stretch high-voltage lattice pylons carrying bright electric pulse into dark urban hall |
| `F5` | **urbanización** (*urbanization*) | `[TIER-D]` | El trabajo industrial atrae población y reestructura vivienda, salud y servicios. / *Industrial work attracts population and restructures housing, health and services.* | Expand vertical multi-story brick tenement blocks rapidly around factory perimeter |
| `F6` | **trabajo** (*work*) | `[TIER-C]` | La productividad creció junto con jornadas peligrosas, conflicto laboral y nuevas organizaciones. / *Productivity grew alongside dangerous working conditions, labor conflict, and new organizations.* | Pulse tense synchronized shift-clock ring beside hazard sparks on machine line |
| `F7` | **impacto ambiental** (*environmental impact*) | `[TIER-D]` | Extraer y quemar energía a gran escala altera aire, agua, clima, suelos y biodiversidad. / *Extracting and burning energy on a large scale alters air, water, climate, soils and biodiversity.* | Billow dark thermal soot & $CO_2$ dome tinting clear atmosphere envelope amber-grey |

##### `Z18` `global-networks` (`vid=2818`) — Redes globales (*Global networks*) `[TIER-C]`

- **Catalog Metadata:** `scene=18` · `[0.900, 0.950]` · `rgb=[125, 145, 255]` · `sound=(sawtooth/131.8Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Telecomunicaciones + transporte + cómputo + logística → efectos a distancia` / *`Telecommunications + transportation + computing + logistics → remote effects`* · Evidence: `Interconexión planetaria de alta velocidad` (*High speed planetary interconnection*)
- **Spoken Macro Narrative (ES / EN):** La conectividad permite cooperación a escala planetaria, pero también transmite conflictos, errores, riesgos y desigualdades. (*EN:* Connectivity allows cooperation on a planetary scale, but it also transmits conflicts, errors, risks and inequalities.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (18 % 7) = 184` (`ch=4`, 2 tilted armillary tori!). Notice `ch=4` draws ONLY the two outer rings (`d_arm`) *WITHOUT* even calling `d_web` wireframe globe (`ch=6` at L197)! Plus satellites, subsea cables, aviation arcs, computers & internet lack geometry. **Target (`vid=2818`):** **Globe + Orbital Constellation & Cable Web (`186`+`184` Fusion)**: lit dark Earth globe crossed by glowing ocean-floor fiber trunks, high-altitude aviation trails, geostationary/LEO satellite nodes, data-center pulses, and access-gap shadows.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **telecomunicaciones** (*telecommunications*) | `[TIER-D]` | Mensajes eléctricos y ópticos reducen el tiempo de comunicación de meses a fracciones de segundo. / *Electrical and optical messages reduce communication time from months to fractions of a second.* | Light gleaming submarine fiber-optic trunk lines pulsing light across ocean basins |
| `F1` | **aviación** (*aviation*) | `[TIER-D]` | Personas y mercancías cruzan continentes rápidamente, junto con emisiones y riesgos biológicos. / *People and goods cross continents rapidly, along with emissions and biological risks.* | Curve bright parabolic aviation contrails linking continental hubs rapidly |
| `F2` | **satélites** (*satellites*) | `[TIER-D]` | Órbitas artificiales sostienen navegación, observación, ciencia y comunicación. / *Artificial orbits support navigation, observation, science and communication.* | Orbit glowing multi-plane GPS/comm satellite constellation beacons (`d_arm` rings) |
| `F3` | **computación** (*computing*) | `[TIER-D]` | Procesar información a gran velocidad cambia administración, diseño, ciencia y vida cotidiana. / *Processing information at high speed changes administration, design, science and everyday life.* | Cluster glowing rhythmic server rack cubes driving high-speed packet bursts |
| `F4` | **internet** (*internet*) | `[TIER-D]` | Redes interoperables conectan publicaciones, servicios, comunidades y sistemas críticos. / *Interoperable networks connect publications, services, communities and critical systems.* | Overlay global interoperable TCP/IP-style mesh snapping across continents |
| `F5` | **logística** (*logistics*) | `[TIER-C]` | Datos, contenedores y centros de distribución coordinan cadenas planetarias. / *Data, containers and distribution centers coordinate planetary chains.* | Move quantized shipping container & data beacons along synced sea-air arteries |
| `F6` | **colaboración** (*collaboration*) | `[TIER-C]` | Equipos separados pueden construir conocimiento y tecnología en tiempo casi real. / *Separate teams can build knowledge and technology in near real time.* | Bridge two distant sunlit research hubs with live synchronized hologram ring |
| `F7` | **desigualdad digital** (*digital inequality*) | `[TIER-C]` | Acceso, control, alfabetización y exposición a riesgos permanecen distribuidos de manera desigual. / *Access, control, literacy and risk exposure remain unequally distributed.* | Contrast hyper-luminous northern grids against dim unserved rural shadow sectors |

##### `Z19` `planetary-civilization` (`vid=2819`) — Civilización planetaria (*planetary civilization*) `[TIER-C]`

- **Catalog Metadata:** `scene=19` · `[0.950, 1.000]` · `rgb=[170, 239, 255]` · `sound=(sine/135.89999999999998Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `tierra`
- **Metric & Evidence (ES / EN):** `Sociedades conectadas + energía + conocimiento + riesgos → capacidad planetaria` / *`Connected societies + energy + knowledge + risks → planetary capacity`* · Evidence: `Red interdependiente que transforma la Tierra` (*Interdependent network that transforms the Earth*)
- **Spoken Macro Narrative (ES / EN):** La humanidad forma una red planetaria de sociedades interdependientes, pero todavía no actúa como una sola comunidad coordinada. (*EN:* Humanity forms a planetary network of interdependent societies, but does not yet act as a single coordinated community.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `180 + (19 % 7) = 185` (`ch=5`, two empty tilted tori without globe!). Climax of Exp 18 (`Civilizacion planetaria`) doesn't even show Earth (`d_web` is only in `ch=6`), energy grids, orbital sensors, or the planetary horizon leading into Exp 19. **Target (`vid=2819`):** **Planetary Earth Biosphere & Technosphere Sphere (`186`+`184`+**`far_shell`**)**: full luminous Earth globe inside tilted orbital observation rings, continental energy grid ties, atmospheric climate envelope, international treaty nodes, and open horizon beacon to `19_technology`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **comunicación global** (*global communication*) | `[TIER-C]` | Gran parte del planeta puede intercambiar información casi instantáneamente, aunque no con igual libertad o acceso. / *Much of the planet can exchange information almost instantaneously, although not with equal freedom or access.* | Ignite real-time planetary optical communication mesh wrapping entire lit Earth sphere |
| `F1` | **observación planetaria** (*planetary observation*) | `[TIER-D]` | Sensores terrestres y orbitales permiten vigilar clima, océanos, bosques, ciudades y riesgos. / *Terrestrial and orbital sensors allow us to monitor climate, oceans, forests, cities and risks.* | Sweep polar-orbiting climate & weather observation satellites painting biosphere telemetry swath |
| `F2` | **sistemas energéticos** (*energy systems*) | `[TIER-D]` | Las sociedades dependen de infraestructuras energéticas extensas que deben transformarse sin excluir a quienes las necesitan. / *Societies depend on extensive energy infrastructures that must be transformed without excluding those who need them.* | Trace glowing continental renewable & fossil energy distribution spines across land |
| `F3` | **conocimiento compartido** (*shared knowledge*) | `[TIER-C]` | Bases de datos, educación y ciencia conectan descubrimientos producidos en culturas e instituciones distintas. / *Databases, education and science connect discoveries produced in different cultures and institutions.* | Link glowing regional archive & scientific university nodes across hemisphere mesh |
| `F4` | **transformación ambiental** (*environmental transformation*) | `[TIER-C]` | La actividad humana altera ciclos globales y exige decisiones que superan fronteras nacionales. / *Human activity alters global cycles and requires decisions that transcend national borders.* | Visualize rising atmospheric thermal band & sea/forest stress boundary around globe |
| `F5` | **instituciones internacionales** (*international institutions*) | `[TIER-C]` | Tratados y organismos coordinan algunas tareas, pero poseen autoridad limitada y desigual. / *Treaties and organizations coordinate some tasks, but have limited and unequal authority.* | Illuminate loose multilateral treaty arcs bridging sovereign continental nodes without single king |
| `F6` | **infraestructura espacial** (*space infrastructure*) | `[TIER-D]` | Satélites y misiones extienden funciones civiles más allá de la superficie sin convertir a la humanidad en una unidad política. / *Satellites and missions extend civilian functions beyond the surface without turning humanity into a political unit.* | Deploy orbital station, deep-space dishes & high-orbit relay nodes off planetary surface |
| `F7` | **futuros y riesgos** (*futures and risks*) | `[TIER-C]` | Más allá surge otra pregunta: ¿qué tecnologías puede construir una civilización planetaria y qué riesgos debe aprender a gobernar? / *Beyond that, another question arises: what technologies can a planetary civilization build and what risks must it learn to govern?* | Branch forward golden portal ring into `19_technology` (`Z00`) with balanced risk prism |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP18-Z15-19)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP18-Z15-19` (Long-Distance Commerce, Empirical Science, Industrial Engines & Planetary Web) for HyperCosmosVR Odyssey Act V (`Exp 18: 18_culture`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z15`->`2815` (`F0..F7`), `Z16`->`2816` (`F0..F7`), `Z17`->`2817` (`F0..F7`), `Z18`->`2818` (`F0..F7`), `Z19`->`2819` (`F0..F7`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3797`, `3798`, `3799`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP18-Z15-19`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Specialize `select_compute_mode` (`crates/cosmos_render/src/universe_scene.rs`) + `shaders/compute_universe.wgsl` (`submode`/mode split) so early zones do not display anachronistic `mode=14` optical-bus particles.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z15: commerce / Commerce and networks, Z16: science / Science, Z17: industry / Industry, Z18: global-networks / Global networks, Z19: planetary-civilization / planetary civilization.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

### 4.2 Experience 19: `19_technology` — Espectro de la Tecnología: de las Herramientas a la Inteligencia Artificial

- **Odyssey & Spatial Placement:** Act 5 (`2/8`, overall stop #19) · `TECH_RING` (`SectorId::TECH_RING`, parent `EARTH_ORBIT`, `u = [6.9, 8.2]`, `focus_scale_exp = 7.6`, `3.84e8 m`) ([`topology.rs#L310-L321, L649`](../../crates/cosmos_spatial/src/topology.rs#L310-L321))
- **Experience Totals:** `20` Zones (`0A / 4B / 6C / 10D`) · `161` Features (`0A / 28B / 33C / 100D`)
- **Current Shader & Compute:** `doc_stages.rs#L65-L69`: `{0|u16::MAX -> 190, 16 -> 194, _ -> 190 + (sc % 7)}` (`197..=199` `else` lattice is dead code) · `shaders/hero_doc_civilization_horizon.wgsl#L205-L271` (`tech=0..6`: gears/Tesla/die+Transformer/helix; `L264-L270` dead; `feat` unused) · `mode=14` (`universe_scene.rs#L322`: optical bus applied even to hand axes & waterwheels `Z01..Z06`)

<a id="pkg-act5-exp19-z00-04"></a>
### PKG-ACT5-EXP19-Z00-04: Planetary Tech Threshold, Hand Tools, Simple Machines, Metrology & Materials

- **Phase:** Phase 1/2 | **Priority:** `P0` | **Scope:** Exp `19` (`19_technology`) `Z00..Z04` (`5` zones, `40` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L65-L69`: `{0|u16::MAX -> 190, 16 -> 194, _ -> 190 + (sc % 7)}` (`197..=199` `else` lattice is dead code); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L205-L271` (`tech=0..6`: gears/Tesla/die+Transformer/helix; `L264-L270` dead; `feat` unused). Tier breakdown for this batch: **`0A / 0B / 1C / 4D` Zones**, **`0A / 0B / 4C / 36D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `1/40` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 2900..=2904` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3800`, `3801` (allocated from `3720 + (19 - 11)*10 + k`; Material/Mechanical Lattice & Calibration Vectors (replacing premature `mode=14` for `Z01..Z04`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z00` | `planetary-civilization` (*Civilización planetaria* / *planetary civilization*) | `[TIER-C]` | `vid=190` | **`2900`** | `3800` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 semantic cue mask) | `0..7` (`0xFF` macro) |
| `Z01` | `manual-tools` (*Herramientas manuales* / *Hand tools*) | `[TIER-D]` | `vid=191` | **`2901`** | `3801` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z02` | `simple-machines` (*Máquinas simples* / *Simple machines*) | `[TIER-D]` | `vid=192` | **`2902`** | `3800` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x02` (0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |
| `Z03` | `measurement` (*Medición y precisión* / *Measurement and precision*) | `[TIER-D]` | `vid=193` | **`2903`** | `3801` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x02` (0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |
| `Z04` | `materials` (*Materiales y metalurgia* / *Materials and metallurgy*) | `[TIER-D]` | `vid=194` | **`2904`** | `3800` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z00` `planetary-civilization` (`vid=2900`) — Civilización planetaria (*planetary civilization*) `[TIER-C]`

- **Catalog Metadata:** `scene=0` · `[0.000, 0.050]` · `rgb=[116, 228, 255]` · `sound=(sine/54Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `orbital`
- **Metric & Evidence (ES / EN):** `Ciudades + redes + satélites + ciencia + industria → capacidad planetaria desigual` / *`Cities + networks + satellites + science + industry → unequal planetary capacity`* · Evidence: `Punto de partida · interdependencia tecnológica global` (*Starting point · global technological interdependence*)
- **Spoken Macro Narrative (ES / EN):** Una civilización tecnológicamente conectada no controla por completo los sistemas que ha creado ni distribuye sus beneficios de manera uniforme. (*EN:* A technologically connected civilization does not fully control the systems it has created or distribute their benefits evenly.)
- **Current Diagnosis & Target 3D Blueprint:** Explicit bespoke override `0 \| u16::MAX => 190` (`doc_stages.rs:66`) renders two heavy counter-rotating cogwheels (`tech=0`). Depicting interconnected planetary cities, grids, satellites, transport, and global observation as two industrial gears is an abrupt thematic downgrade. **Target (`vid=2900`):** Replace `0 => 190` with **Planetary Technosphere Globe (`vid=2900 (extending legacy `186`/`194`)` fusion)**: sunlit/night-lit city clusters over planetary sphere threaded by HVDC power grids, satellite shells, shipping/rail arcs, and climate sensors.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **ciudades** (*cities*) | `[TIER-D]` | Las ciudades concentran personas, conocimiento, servicios e infraestructuras, pero también consumo, vulnerabilidad y desigualdad. / *Cities concentrate people, knowledge, services and infrastructure, but also consumption, vulnerability and inequality.* | Ignite golden nocturnal metropolitan city clusters unevenly blanketing globe coastlines |
| `F1` | **redes energéticas** (*energy networks*) | `[TIER-D]` | Generación, transmisión, almacenamiento y demanda deben coordinarse continuamente para sostener la vida moderna. / *Generation, transmission, storage and demand must be continually coordinated to sustain modern life.* | Pulse synchronized continental power transmission spines balancing variable supply |
| `F2` | **satélites** (*satellites*) `[kw:orbital]` | `[TIER-D]` | Los satélites permiten navegación, observación, ciencia y comunicación, pero también crean dependencia orbital y residuos. / *Satellites enable navigation, observation, science and communication, but they also create orbital dependency and debris.* | Orbit multi-altitude navigation/observation satellites + faint orbital debris streak ring |
| `F3` | **transporte** (*transportation*) | `[TIER-D]` | Carreteras, puertos, ferrocarriles y aviación conectan economías mientras consumen materiales y energía. / *Roads, ports, railways and aviation connect economies while consuming materials and energy.* | Trace bright container-ship lanes, rail spines, and transoceanic aviation arcs |
| `F4` | **instituciones científicas** (*scientific institutions*) | `[TIER-C]` | La ciencia organizada combina instrumentos, datos, crítica, financiación y cooperación internacional. / *Organized science combines instruments, data, criticism, financing and international cooperation.* | Link scientific telescope/collider observatory nodes globally via shared data beams |
| `F5` | **comunicación global** (*global communication*) | `[TIER-C]` | Mensajes y datos cruzan el planeta casi al instante, aunque el acceso y el control siguen siendo desiguales. / *Messages and data cross the planet almost instantly, although access and control remain unequal.* | Flash fast optical internet fiber traffic jumping between uneven bandwidth regions |
| `F6` | **producción industrial** (*industrial production*) | `[TIER-D]` | La producción masiva transforma materias primas en bienes mediante cadenas extensas de energía, trabajo y logística. / *Mass production transforms raw materials into goods through extensive chains of energy, labor and logistics.* | Extrude rhythmic automated factory resource hubs feeding continental logistics arcs |
| `F7` | **observación ambiental** (*environmental observation*) | `[TIER-C]` | Sensores terrestres y orbitales revelan cambios planetarios que ninguna persona podría observar por sí sola. / *Ground-based and orbital sensors reveal planetary changes that no one person could observe alone.* | Sweep polar Earth-observation telemetry scan rings painting atmospheric/ocean metrics |

##### `Z01` `manual-tools` (`vid=2901`) — Herramientas manuales (*Hand tools*) `[TIER-D]`

- **Catalog Metadata:** `scene=1` · `[0.050, 0.100]` · `rgb=[255, 196, 117]` · `sound=(triangle/58.65Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Cuerpo + material + forma diseñada → acción ampliada` / *`Body + material + designed shape → enlarged action`* · Evidence: `Extensión directa del cuerpo` (*Direct extension of the body*)
- **Spoken Macro Narrative (ES / EN):** Una herramienta transforma la relación entre el cuerpo, los materiales y el entorno. (*EN:* A tool transforms the relationship between the body, materials and the environment.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (1 % 7) = 191` (`tech=1`, counter-rotating steel gears!). Manual hand tools—knife edge, hammer, rope, pierced needle, vessel, lever bar, bow-drill—are tangible pre-industrial objects completely missing from WGSL (while `mode=14` optical bus particles flow over flint knives!). **Target (`vid=2901`):** **Ergonomic Hand-Tool Diorama Ring**: procedural SDF sculptures of stone/steel knife blade, hafted hammer, braided rope helix, threaded needle, hollow storage vessel, fulcrum pry-lever, and rotating bow-drill.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **cuchillo** (*knife*) | `[TIER-D]` | Un filo concentra fuerza en un borde estrecho para cortar, tallar o separar materiales. / *A cutting edge concentrates force on a narrow edge to cut, carve or separate materials.* | Sculpt wedge-beveled knife blade slicing cleanly across contact stress plane |
| `F1` | **martillo** (*hammer*) | `[TIER-D]` | La masa y el mango convierten el movimiento del brazo en un impacto controlado. / *The mass and handle convert arm movement into a controlled impact.* | Swing dense cylindrical hammer head on shaft delivering pulsed impact rings |
| `F2` | **cuerda** (*rope*) | `[TIER-D]` | Fibras reunidas permiten tensar, unir, elevar, medir y transmitir fuerza. / *Gathered fibers allow tensioning, joining, lifting, measuring and transmitting force.* | Stretch braided 3-strand helical rope tension cable between anchor lugs |
| `F3` | **aguja** (*needle*) | `[TIER-D]` | Una punta fina y un ojo pequeño hacen posible perforar y unir materiales flexibles. / *A fine tip and small eye make it possible to drill and join flexible materials.* | Thread pierced tapered needle looping glowing stitch curve through thin sheet |
| `F4` | **recipiente** (*container*) | `[TIER-D]` | Contener líquidos, granos o calor amplía la capacidad humana de almacenar, transportar y cocinar. / *Containing liquids, grains or heat expands the human ability to store, transport and cook.* | Revolve thick-walled ceramic/wooden containment vessel holding glowing liquid/grain |
| `F5` | **palanca** (*lever*) | `[TIER-D]` | Una barra apoyada puede multiplicar fuerza o cambiar su dirección. / *A supported bar can multiply force or change its direction.* | Pivot rigid straight pry-bar over triangular fulcrum multiplying short-end lift |
| `F6` | **taladro** (*drill*) | `[TIER-D]` | La rotación concentrada abre cavidades y permite ensamblajes más precisos. / *Concentrated rotation opens cavities and allows for more precise assemblies.* | Spin pointed cylindrical hand/bow drill boring rotating spiral cavity down |
| `F7` | **mantenimiento** (*maintenance*) | `[TIER-D]` | Una herramienta útil depende de afilado, reparación, habilidad y acceso a materiales. / *A useful tool depends on sharpening, repair, skill and access to materials.* | Run whetstone sharpening pass renewing bright glowing edge gleam along blade |

##### `Z02` `simple-machines` (`vid=2902`) — Máquinas simples (*Simple machines*) `[TIER-D]`

- **Catalog Metadata:** `scene=2` · `[0.100, 0.150]` · `rgb=[255, 145, 101]` · `sound=(sine/63.3Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Fuerza aplicada × distancia recorrida → trabajo redirigido con pérdidas` / *`Applied force × distance traveled → redirected work with losses`* · Evidence: `Transformación mecánica de fuerza y movimiento` (*Mechanical transformation of force and motion*)
- **Spoken Macro Narrative (ES / EN):** Las máquinas simples no crean trabajo gratis: intercambian fuerza, distancia, velocidad o dirección. (*EN:* Simple machines do not create free work: they exchange force, distance, speed or direction.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (2 % 7) = 192` (`tech=2`, Twin Nikola Tesla spark coils!!). Classic mechanical simple machines (lever, wheel & axle, block-and-tackle pulley, inclined plane, wedge, Archimedes screw) shown as high-voltage electric coils—even though gears (`tech=0`) sit two slots away! **Target (`vid=2902`):** **6-Station Mechanical Workbench Carousel**: kinetic articulated SDF bench displaying balance lever, rolling wheel-and-axle, multi-sheave pulley hoist, inclined ramp, splitting wedge, helical screw press, and meshed `vid=2902 (extending legacy `vid=190`)` gears.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **palanca** (*lever*) | `[TIER-D]` | La posición del punto de apoyo determina cuánto se multiplica la fuerza y cuánto debe moverse el brazo. / *The position of the fulcrum determines how much the force is multiplied and how much the arm must move.* | Animate see-saw Class-1 lever pivoting asymmetric arm lengths around sliding fulcrum |
| `F1` | **rueda y eje** (*wheel and axle*) | `[TIER-D]` | Una rueda reduce fricción de deslizamiento y el eje transmite rotación. / *A wheel reduces sliding friction and the axle transmits rotation.* | Spin thick spoked wheel rotating tightly coupled concentric drive axle shaft |
| `F2` | **polea** (*pulley*) | `[TIER-D]` | Una cuerda sobre una rueda cambia la dirección de la fuerza; varias poleas pueden repartir la carga. / *A rope on a wheel changes the direction of the force; several pulleys can distribute the load.* | Drop suspended weight cleanly along multi-sheave pulley block-and-tackle ropes |
| `F3` | **plano inclinado** (*inclined plane*) | `[TIER-D]` | Subir una carga por una pendiente requiere menos fuerza a cambio de recorrer más distancia. / *Carrying a load up a slope requires less force in exchange for traveling more distance.* | Roll heavy block steadily up long shallow-angle glowing inclined ramp prism |
| `F4` | **cuña** (*wedge*) | `[TIER-D]` | Dos planos inclinados unidos convierten avance en fuerzas laterales para separar o fijar. / *Two joined inclined planes convert advance into lateral forces to separate or fix.* | Drive triangular double-slope wedge downward splitting flanked test blocks apart |
| `F5` | **tornillo** (*screw*) | `[TIER-D]` | Una rampa helicoidal transforma giro en avance preciso y gran presión. / *A helical ramp transforms rotation into precise advance and great pressure.* | Rotate Archimedes helical screw ramp converting continuous torque to axial compression |
| `F6` | **engranajes** (*gears*) | `[TIER-C]` | Dientes acoplados intercambian velocidad, par y dirección entre ejes. / *Engaging teeth exchange speed, torque and direction between shafts.* | Mesh existing counter-rotating cogwheels (`vid=2902 (extending legacy `vid=190`)` primitive!) exchanging torque/RPM |
| `F7` | **fricción** (*friction*) | `[TIER-D]` | Toda máquina real pierde parte de la energía útil en calor, deformación, sonido y desgaste. / *Every real machine loses some of its useful energy in heat, deformation, sound and wear.* | Emit warm infrared thermal friction glow along sliding wheel/ramp contact faces |

##### `Z03` `measurement` (`vid=2903`) — Medición y precisión (*Measurement and precision*) `[TIER-D]`

- **Catalog Metadata:** `scene=3` · `[0.150, 0.200]` · `rgb=[255, 226, 122]` · `sound=(triangle/67.95Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Propiedad + unidad + instrumento + incertidumbre → valor comparable` / *`Property + unit + instrument + uncertainty → comparable value`* · Evidence: `Comparación estandarizada y reproducible` (*Standardized and reproducible comparison*)
- **Spoken Macro Narrative (ES / EN):** Medir permite comparar, reproducir y coordinar acciones entre personas que no están en el mismo lugar. (*EN:* Measuring allows us to compare, reproduce and coordinate actions between people who are not in the same place.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (3 % 7) = 193` (`tech=3`, Twin Tesla spark coils!). Precision measurement tools—length rule, pan balance scale, escapement clockwork, goniometer/sextant, thermometer, cartographic projection, calibration reference—are completely unmodeled. **Target (`vid=2903`):** **Metrology & Calibration Suite**: glowing Vernier length standard bar, twin-pan equal-arm mass balance, skeletal escapement clock gears, rotating brass quadrant protractor, glowing capillary thermometer, contoured map projection, and Gaussian error bars.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **longitud** (*length*) | `[TIER-D]` | Reglas y patrones de longitud hacen posible fabricar piezas compatibles y describir distancias. / *Length rules and patterns make it possible to manufacture compatible parts and describe distances.* | Project floating laser/Vernier caliper rule dividing unit spans accurately onto gear blank |
| `F1` | **masa** (*mass*) | `[TIER-D]` | Comparar masas ayuda a comerciar, formular materiales y controlar procesos. / *Comparing masses helps with trading, formulating materials and controlling processes.* | Oscillate crisp equal-arm beam balance comparing reference brass weights against sample |
| `F2` | **tiempo** (*time*) | `[TIER-D]` | Relojes compartidos coordinan navegación, industria, comunicaciones y experimentos. / *Shared clocks coordinate navigation, industry, communications and experiments.* | Tick rhythmic anchor escapement gear + concentric analog clock dial hands |
| `F3` | **ángulo** (*angle*) | `[TIER-D]` | La medición angular sostiene construcción, cartografía, astronomía y control de máquinas. / *Angular measurement supports construction, cartography, astronomy and machine control.* | Sweep graduated brass quadrant arc triangulating star & horizon angle vectors |
| `F4` | **temperatura** (*temperature*) | `[TIER-D]` | Un termómetro relaciona cambios físicos observables con una escala acordada. / *A thermometer relates observable physical changes to an agreed upon scale.* | Rise glowing mercury/thermal fluid column evenly beside temperature graduation ticks |
| `F5` | **mapas** (*maps*) | `[TIER-D]` | Un mapa convierte relaciones espaciales complejas en un modelo simbólico utilizable. / *A map converts complex spatial relationships into a usable symbolic model.* | Flatten curved spherical projection grid smoothly onto 2D topographic contour map |
| `F6` | **calibración** (*calibration*) | `[TIER-D]` | Comparar un instrumento con una referencia conocida reduce errores sistemáticos. / *Comparing an instrument with a known reference reduces systematic errors.* | Align raw skewed needle trace directly onto golden NIST-like master reference bar |
| `F7` | **incertidumbre** (*uncertainty*) | `[TIER-D]` | Toda medición tiene límites; informar la incertidumbre evita confundir precisión aparente con certeza. / *Every measurement has limits; Reporting uncertainty avoids confusing apparent precision with certainty.* | Bracket measured tip within breathing translucent +/- sigma uncertainty error envelope |

##### `Z04` `materials` (`vid=2904`) — Materiales y metalurgia (*Materials and metallurgy*) `[TIER-D]`

- **Catalog Metadata:** `scene=4` · `[0.200, 0.250]` · `rgb=[234, 177, 255]` · `sound=(sine/72.6Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Composición + microestructura + proceso → propiedades del material` / *`Composition + microstructure + process → material properties`* · Evidence: `Propiedades diseñadas mediante estructura y proceso` (*Properties designed through structure and process*)
- **Spoken Macro Narrative (ES / EN):** La tecnología de materiales no solo encuentra sustancias: modifica su estructura para obtener propiedades específicas. (*EN:* Materials technology doesn't just find substances: it modifies their structure to obtain specific properties.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (4 % 7) = 194` (`tech=4`, Silicon die + Transformer web!!). Narrates physical material science—knapped stone, fired ceramics & clear glass, copper/bronze, steel carbon lattices, superalloys, long-chain polymers, and laminated composites—never drawn as materials! **Target (`vid=2904`):** **Microstructure-to-Macro Crucible Stage**: glowing atomic crystal lattice morphing between brittle stone cleave, amorphous transparent silica glass, FCC bronze, iron-carbon martensite quench, tangled polymer chains, and woven fiber-matrix laminate.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **piedra** (*stone*) | `[TIER-D]` | Dureza, fractura y disponibilidad hicieron de la piedra una plataforma tecnológica duradera. / *Hardness, fracture and availability made stone a durable technological platform.* | Fracture dense faceted crystalline mineral block cleanly along conchoidal crystal planes |
| `F1` | **arcilla y vidrio** (*clay and glass*) | `[TIER-D]` | Calor y composición transforman minerales en cerámicas resistentes o vidrios transparentes. / *Heat and composition transform minerals into resistant ceramics or transparent glass.* | Glow incandescent kiln chamber vitrifying layered clay into clear refractive glass sphere |
| `F2` | **cobre y bronce** (*copper and bronze*) | `[TIER-D]` | Metales y aleaciones permitieron formas reutilizables, conductoras y reparables. / *Metals and alloys allowed for reusable, conductive and repairable forms.* | Alloy interleaved reddish copper & silvery tin atoms into gleaming bronze ingot |
| `F3` | **hierro y acero** (*iron and steel*) | `[TIER-D]` | El control del carbono, la temperatura y el enfriamiento produce aceros con comportamientos muy distintos. / *Controlling carbon, temperature and cooling produces steels with very different behaviors.* | Intercalate tiny glowing carbon spheres inside BCC/FCC iron lattice + quench blue spark |
| `F4` | **aleaciones** (*alloys*) | `[TIER-D]` | Mezclar elementos puede mejorar resistencia, corrosión, peso o temperatura de servicio. / *Mixing elements can improve resistance, corrosion, weight or service temperature.* | Blend distinct metallic atomic species evenly suppressing dislocation slip bands |
| `F5` | **polímeros** (*polymers*) | `[TIER-D]` | Cadenas moleculares largas permiten materiales ligeros y moldeables, pero plantean persistencia ambiental. / *Long molecular chains allow for lightweight, moldable materials, but pose environmental persistence.* | Tangle long undulating flexible carbon-backbone polymer macromolecule ribbons |
| `F6` | **compuestos** (*compounds*) | `[TIER-D]` | Combinar materiales distintos reúne propiedades que ninguno ofrece por separado. / *Combining different materials brings together properties that none of them offer separately.* | Interweave crisp orthogonal carbon-fiber fabric layers inside translucent resin matrix |
| `F7` | **ciclo de vida** (*life cycle*) | `[TIER-D]` | Extracción, fabricación, uso, reparación, reciclaje y desecho forman una sola historia material. / *Extraction, manufacturing, use, repair, recycling and disposal form a single material history.* | Loop circular material flow ring: quarry -> smelt -> forge -> use -> repair -> recycle |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP19-Z00-04)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP19-Z00-04` (Planetary Tech Threshold, Hand Tools, Simple Machines, Metrology & Materials) for HyperCosmosVR Odyssey Act V (`Exp 19: 19_technology`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z00`->`2900` (`F0..F7`), `Z01`->`2901` (`F0..F7`), `Z02`->`2902` (`F0..F7`), `Z03`->`2903` (`F0..F7`), `Z04`->`2904` (`F0..F7`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3800`, `3801`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP19-Z00-04`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Specialize `select_compute_mode` (`crates/cosmos_render/src/universe_scene.rs`) + `shaders/compute_universe.wgsl` (`submode`/mode split) so early zones do not display anachronistic `mode=14` optical-bus particles.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z00: planetary-civilization / planetary civilization, Z01: manual-tools / Hand tools, Z02: simple-machines / Simple machines, Z03: measurement / Measurement and precision, Z04: materials / Materials and metallurgy.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp19-z05-09"></a>
### PKG-ACT5-EXP19-Z05-09: Mechanized Agriculture, Heat Engines, Mass Industry, Electromagnetism & Telecom

- **Phase:** Phase 1/2 | **Priority:** `P1` | **Scope:** Exp `19` (`19_technology`) `Z05..Z09` (`5` zones, `40` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L65-L69`: `{0|u16::MAX -> 190, 16 -> 194, _ -> 190 + (sc % 7)}` (`197..=199` `else` lattice is dead code); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L205-L271` (`tech=0..6`: gears/Tesla/die+Transformer/helix; `L264-L270` dead; `feat` unused). Tier breakdown for this batch: **`0A / 1B / 2C / 2D` Zones**, **`0A / 6B / 10C / 24D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `1/40` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 2905..=2909` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3802`, `3803`, `3804` (allocated from `3720 + (19 - 11)*10 + k`; Thermodynamic Flux -> EM Wavefront Propagation (`mode=14` at `Z09`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z05` | `technological-agriculture` (*Agricultura tecnológica* / *Technological agriculture*) | `[TIER-D]` | `vid=195` | **`2905`** | `3802` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z06` | `engines` (*Motores* / *Engines*) | `[TIER-D]` | `vid=196` | **`2906`** | `3803` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x01` (0x01 cutaway) | `0..7` (`0xFF` macro) |
| `Z07` | `industry` (*Industria* / *Industry*) | `[TIER-B]` | `vid=190` | **`2907`** | `3804` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z08` | `electricity` (*Electricidad* / *Electricity*) | `[TIER-C]` | `vid=191` | **`2908`** | `3802` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x02` (0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |
| `Z09` | `telecommunications` (*Telecomunicaciones* / *Telecommunications*) | `[TIER-C]` | `vid=192` | **`2909`** | `3803` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x06` (0x04 semantic cue mask, 0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z05` `technological-agriculture` (`vid=2905`) — Agricultura tecnológica (*Technological agriculture*) `[TIER-D]`

- **Catalog Metadata:** `scene=5` · `[0.250, 0.300]` · `rgb=[139, 234, 139]` · `sound=(sawtooth/77.25Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Suelo + agua + organismos + herramientas + conocimiento → alimento y efectos ecológicos` / *`Soil + water + organisms + tools + knowledge → food and ecological effects`* · Evidence: `Sistemas de producción biológica y control ambiental` (*Biological production systems and environmental control*)
- **Spoken Macro Narrative (ES / EN):** La tecnología agrícola aumentó la producción, pero también puede concentrar poder, degradar ecosistemas y aumentar dependencias. (*EN:* Agricultural technology increased production, but it can also concentrate power, degrade ecosystems and increase dependencies.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (5 % 7) = 195` (`tech=5`, AI Transformer chip!!). Showing an etched neural-network processor die for plows, irrigation canals, mountain terraces, windmills, nitrogen fertilizers, food preservation, and harvesters is completely mismatched. **Target (`vid=2905`):** **Agro-Engineering Basin & Windmill Stage**: rotating wooden/metal multi-blade farm windmill besides contoured rice terraces fed by pump irrigation pipe, furrow plow pass, nitrogen cycle ribbons, and automated tractor track.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **arado** (*plow*) | `[TIER-D]` | Remover el suelo facilita ciertos cultivos, aunque también puede acelerar erosión y pérdida de carbono. / *Turning the soil facilitates certain crops, although it can also accelerate erosion and carbon loss.* | Cut curving furrow slice behind iron moldboard plow blade exposing soil cross-section |
| `F1` | **riego** (*irrigation*) | `[TIER-D]` | Canales, bombas y sensores desplazan agua hacia cultivos, creando productividad y nuevas presiones hídricas. / *Canals, pumps and sensors move water to crops, creating productivity and new water pressures.* | Pump gleaming water column through pipe network onto moisture-sensed crop rows |
| `F2` | **terrazas** (*terraces*) | `[TIER-D]` | Modificar pendientes reduce erosión y conserva agua mediante conocimiento local acumulado. / *Modifying slopes reduces erosion and conserves water through accumulated local knowledge.* | Step terraced mountain slope shelves holding calm reflective pooled irrigation water |
| `F3` | **molinos** (*mills*) | `[TIER-D]` | Viento, agua o motores convierten energía ambiental en molienda y procesamiento. / *Wind, water or engines convert environmental energy into grinding and processing.* | Rotate four-vaned windmill sails driving vertical rotary grain-grinding millstones |
| `F4` | **selección** (*selection*) | `[TIER-D]` | Elegir semillas y animales cambia poblaciones durante generaciones, mucho antes de la genética moderna. / *Choosing seeds and animals changes populations for generations, long before modern genetics.* | Branch selective golden seed-head traits across successive generational row harvests |
| `F5` | **fertilizantes** (*fertilizers*) | `[TIER-D]` | Añadir nutrientes puede elevar cosechas, pero el exceso contamina aguas y altera ciclos globales. / *Adding nutrients can increase harvests, but excess contaminates waters and alters global cycles.* | Flow emerald-cyan nitrogen cycle granules into roots vs runaway green river runoff |
| `F6` | **conservación** (*conservation*) | `[TIER-D]` | Secado, salado, frío, fermentación y envasado reducen pérdidas y amplían distribución. / *Drying, salting, cold, fermentation and packaging reduce losses and expand distribution.* | Seal harvested goods inside cool crystalline frost-ring & fermentation vessel pods |
| `F7` | **cosecha mecanizada** (*mechanized harvest*) | `[TIER-D]` | Máquinas aceleran trabajo agrícola, pero requieren capital, combustible, mantenimiento y adaptación social. / *Machines accelerate agricultural work, but require capital, fuel, maintenance and social adaptation.* | Sweep wide motorized combine reel track rapidly gathering uniform golden crop rows |

##### `Z06` `engines` (`vid=2906`) — Motores (*Engines*) `[TIER-D]`

- **Catalog Metadata:** `scene=6` · `[0.300, 0.350]` · `rgb=[255, 122, 112]` · `sound=(sine/81.9Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Energía de entrada → movimiento útil + calor + sonido + pérdidas` / *`Input energy → useful movement + heat + sound + losses`* · Evidence: `Conversión controlada de energía en movimiento` (*Controlled conversion of energy into motion*)
- **Spoken Macro Narrative (ES / EN):** Un motor no crea energía: transforma una forma de energía en movimiento útil, siempre con pérdidas. (*EN:* A motor does not create energy: it transforms a form of energy into useful movement, always with losses.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (6 % 7) = 196` (`tech=6`, Biotech DNA Double Helix + Gene Gate!!). Prime-mover engines—muscle, wind sail, waterwheel, steam piston, internal combustion cylinder, gas turbine, electric stator, rocket nozzle—rendered as a DNA strand is bizarrely inverted. **Target (`vid=2906`):** **Evolution of Prime Movers Cutaway**: translucent cross-sectioned multi-chamber stage morphing/arraying contractile muscle band, wind-sail rotor, undershot waterwheel, reciprocating steam piston, 4-stroke spark cylinder, multi-blade jet turbine, copper motor stator, and de Laval rocket bell.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **músculo** (*muscle*) | `[TIER-D]` | Los músculos convierten energía química en fuerza mediante sistemas biológicos autorreparables y limitados. / *Muscles convert chemical energy into force through limited, self-healing biological systems.* | Contract glowing striated muscle bundle lifting articulated joint lever with warm pulse |
| `F1` | **viento** (*wind*) | `[TIER-D]` | Velas y rotores capturan parte del movimiento del aire. / *Sails and rotors capture some of the air movement.* | Billow curved airfoil wing-sail accelerating tangential wind-stream torque ring |
| `F2` | **agua** (*water*) | `[TIER-D]` | Ruedas y turbinas aprovechan caudal y desnivel para producir rotación. / *Wheels and turbines take advantage of flow and unevenness to produce rotation.* | Drive padded paddle waterwheel spinning steady shaft under descending fluid waterfall |
| `F3` | **vapor** (*steam*) | `[TIER-D]` | El calor genera presión que puede mover pistones o turbinas. / *Heat generates pressure that can move pistons or turbines.* | Pump glowing high-pressure boiler vapor forcing reciprocating brass piston + flywheel |
| `F4` | **combustión** (*combustion*) | `[TIER-D]` | Quemar combustible dentro de una máquina produce expansión rápida, movimiento y emisiones. / *Burning fuel inside a machine produces rapid expansion, movement and emissions.* | Ignite timed orange fuel-air flash inside translucent cylinder driving crankshaft down |
| `F5` | **turbina** (*turbine*) | `[TIER-D]` | Un flujo continuo de vapor, gas, agua o aire hace girar álabes. / *A continuous flow of steam, gas, water or air rotates blades.* | Spin multi-stage dense helical compressor & turbine stator fan blades in hot gas stream |
| `F6` | **motor eléctrico** (*electric motor*) | `[TIER-D]` | Campos magnéticos convierten corriente eléctrica en par con alta eficiencia y control preciso. / *Magnetic fields convert electric current into torque with high efficiency and precise control.* | Rotate glowing squirrel-cage armature inside sweeping blue electromagnetic stator field |
| `F7` | **cohete** (*rocket*) | `[TIER-D]` | Expulsar masa a gran velocidad produce empuje incluso en el vacío. / *Ejecting mass at high speed produces thrust even in a vacuum.* | Blast supersonic diamond-shock plasma plume downward through bell-shaped de Laval nozzle |

##### `Z07` `industry` (`vid=2907`) — Industria (*Industry*) `[TIER-B]`

- **Catalog Metadata:** `scene=7` · `[0.350, 0.400]` · `rgb=[255, 156, 86]` · `sound=(triangle/86.55Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Máquinas + energía + trabajo + normas + logística → producción repetible` / *`Machines + energy + work + standards + logistics → repeatable production`* · Evidence: `Producción coordinada a gran escala` (*Coordinated large-scale production*)
- **Spoken Macro Narrative (ES / EN):** La industria es una red de personas, materiales, energía, organización y mantenimiento, no solo un conjunto de máquinas. (*EN:* Industry is a network of people, materials, energy, organization and maintenance, not just a set of machines.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (7 % 7) = 190` (`tech=0`, two counter-rotating toothed kinetic gears). Meshing heavy gears directly embody mechanized industry (`[TIER-B]`), though `doc_feat` is ignored and non-gear elements (factory hall, lathe, assembly line, railway, grid) are absent. **Target (`vid=2907`):** Retain meshing `vid=2907 (extending legacy `vid=190`)` gears as mechanical power core; attach surrounding **Factory Machine-Tool & Conveyor Ring** switching sub-models per `doc_feat(0..7)`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **fábrica** (*factory*) | `[TIER-B]` | Reunir procesos en un lugar permite coordinación y escala, pero también concentra riesgos y poder. / *Bringing processes together in one place allows for coordination and scale, but also concentrates risk and power.* | Frame rotating gears inside rectilinear factory structural steel girder cage |
| `F1` | **máquina-herramienta** (*machine tool*) | `[TIER-B]` | Tornos, fresadoras y prensas fabrican piezas precisas para construir otras máquinas. / *Lathes, milling machines and presses make precise parts to build other machines.* | Engage secondary cutting spindle lathe milling precise cylindrical stock beside gears |
| `F2` | **estandarización** (*standardization*) | `[TIER-B]` | Dimensiones y procedimientos comunes permiten intercambiabilidad y cooperación entre fabricantes. / *Common dimensions and procedures allow interchangeability and cooperation between manufacturers.* | Snap uniform duplicate gear teeth blanks against calibrated reference gauge ring |
| `F3` | **línea de montaje** (*assembly line*) | `[TIER-B]` | Dividir una secuencia en tareas repetidas aumenta ritmo y reduce flexibilidad del trabajo. / *Dividing a sequence into repeated tasks increases pace and reduces work flexibility.* | Slide paced sequential modular assembly nodes along linear conveyor belt track |
| `F4` | **ferrocarril** (*railway*) | `[TIER-D]` | Vías, locomotoras y horarios integran producción y transporte terrestre. / *Tracks, locomotives and schedules integrate production and land transportation.* | Extrude twin elevated steel railway rails speeding freight cargo spark past factory |
| `F5` | **producción masiva** (*mass production*) | `[TIER-B]` | Grandes volúmenes reducen coste unitario mientras elevan demanda material y residuos. / *Large volumes reduce unit cost while increasing material demand and waste.* | Spawn high-rate continuous stream of identical stamped cubes pouring off belt |
| `F6` | **mantenimiento** (*maintenance*) | `[TIER-B]` | Inspección, lubricación, repuestos y conocimiento técnico sostienen la capacidad productiva real. / *Inspection, lubrication, spare parts and technical knowledge sustain real productive capacity.* | Highlight oil-film meniscus, bearing raceways & wrench inspection sweep over gear hub |
| `F7` | **trabajo industrial** (*industrial work*) | `[TIER-D]` | La productividad depende de habilidades, seguridad, negociación, salud y distribución de beneficios. / *Productivity depends on skills, security, negotiation, health and benefit distribution.* | Connect rhythmic factory belt directly to glowing worker-coordination & safety halos |

##### `Z08` `electricity` (`vid=2908`) — Electricidad (*Electricity*) `[TIER-C]`

- **Catalog Metadata:** `scene=8` · `[0.400, 0.450]` · `rgb=[118, 198, 255]` · `sound=(sine/91.2Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Generación → transformación → transmisión → uso → pérdidas y equilibrio instantáneo` / *`Generation → transformation → transmission → use → losses and instantaneous equilibrium`* · Evidence: `Infraestructura invisible de energía y control` (*Invisible power and control infrastructure*)
- **Spoken Macro Narrative (ES / EN):** La electricidad conecta fuentes y usos distantes, pero la red debe equilibrar producción y consumo continuamente. (*EN:* Electricity connects distant sources and uses, but the grid must continually balance production and consumption.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (8 % 7) = 191` (`tech=1`, plain mechanical cogwheels!). Pure modulo tragedy: **Twin Tesla / Inductor Coils + Live Spark Arc (`tech=2\|3`)** sits right next door on `Z02 simple-machines` and `Z03 measurement`, while Electricity itself gets stone-cold cogwheels! **Target (`vid=2908`):** Reassign `Z08` to **Upgraded EM Power Grid & Coil Stage (`192/193+`)**: rotating copper magnet dynamo rotor -> step-up Tesla/transformer coils (`vid=2908 (extending legacy `vid=192`)/193`) -> closed circuit loop -> lattice transmission pylons -> battery storage module.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **generador** (*generator*) | `[TIER-C]` | El movimiento relativo entre conductores y campos magnéticos puede producir tensión eléctrica. / *The relative motion between conductors and magnetic fields can produce electrical voltage.* | Rotate magnet dipole core inside stationary copper stator coil inducing AC pulse wave |
| `F1` | **circuito** (*circuit*) | `[TIER-C]` | La corriente necesita un camino cerrado y componentes con funciones definidas. / *The current needs a closed path and components with defined functions.* | Illuminate closed conductive copper loop carrying directional drifting electron beads |
| `F2` | **tensión** (*voltage*) | `[TIER-C]` | La tensión describe diferencia de potencial eléctrico, no una sustancia almacenada dentro del cable. / *Voltage describes electrical potential difference, not a substance stored within the cable.* | Render asymmetric voltage potential pressure differential color step across twin terminals |
| `F3` | **corriente** (*current*) | `[TIER-C]` | La corriente mide flujo de carga por unidad de tiempo. / *Current measures charge flow per unit of time.* | Animate dense streaming luminous current flux rate along closed circuit pipe |
| `F4` | **transformador** (*transformer*) | `[TIER-C]` | Campos magnéticos variables permiten cambiar niveles de tensión en corriente alterna. / *Variable magnetic fields allow changing voltage levels in alternating current.* | Spotlight paired primary/secondary magnetic induction coils (`vid=2908 (extending legacy `vid=192`)`) stepping voltage |
| `F5` | **motor** (*motor*) | `[TIER-C]` | La electricidad puede volver a convertirse en movimiento mediante fuerzas electromagnéticas. / *Electricity can be converted back into motion by electromagnetic forces.* | Spin coupled electromagnetic motor rotor turning electric input back to torque |
| `F6` | **red eléctrica** (*electrical network*) | `[TIER-D]` | Centrales, líneas, subestaciones, protecciones y control forman un sistema interdependiente. / *Power plants, lines, substations, protections and control form an interdependent system.* | Spread high-voltage transmission pylons linking power plant, substation & city nodes |
| `F7` | **almacenamiento** (*storage*) | `[TIER-D]` | Baterías, bombeo y otras tecnologías desplazan energía en el tiempo, con límites de coste y materiales. / *Batteries, pumping and other technologies move energy over time, with limits on cost and materials.* | Charge multi-cell lithium/hydro-pumped storage bank buffering intermittent generation spikes |

##### `Z09` `telecommunications` (`vid=2909`) — Telecomunicaciones (*Telecommunications*) `[TIER-C]`

- **Catalog Metadata:** `scene=9` · `[0.450, 0.500]` · `rgb=[103, 230, 223]` · `sound=(triangle/95.85Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `satelite`
- **Metric & Evidence (ES / EN):** `Mensaje → codificación → señal → canal → decodificación` / *`Message → encoding → signal → channel → decoding`* · Evidence: `Información separada del transporte físico` (*Information separate from physical transportation*)
- **Spoken Macro Narrative (ES / EN):** La comunicación tecnológica separó progresivamente la velocidad de la información de la velocidad del transporte físico. (*EN:* Technological communication progressively separated the speed of information from the speed of physical transportation.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (9 % 7) = 192` (`tech=2`, Twin Tesla coils + spark arc). Resonant twin spark coils partially connect to early spark-gap telegraph/radio (`[TIER-C]` partial modulo overlap), yet television, glass fiber optics, cellular wireless meshes, orbital satellites, and discrete digital packets have zero visual. **Target (`vid=2909`):** **5-Band Telecom Spectrum Conduit**: telegraph key pulse -> copper voice wire -> twin RF Tesla antenna coils (`vid=2909 (extending legacy `vid=192`)`) -> total-internal-reflection glass fiber tube -> orbital relay dish & crisp square-wave digital bitstream.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **telégrafo** (*telegraph*) | `[TIER-C]` | Pulsos eléctricos codificados permitieron enviar símbolos a gran distancia casi de inmediato. / *Coded electrical pulses made it possible to send symbols over great distances almost immediately.* | Pulse sharp Morse-coded DC voltage bursts down long twin telegraph wire span |
| `F1` | **teléfono** (*phone*) | `[TIER-C]` | Convertir sonido en señal eléctrica hizo posible transportar la voz. / *Converting sound into an electrical signal made it possible to transport voice.* | Modulate continuous acoustic sine wave onto twisted copper telephone pair |
| `F2` | **radio** (*radio*) | `[TIER-C]` | Ondas electromagnéticas llevan información sin un cable continuo entre emisor y receptor. / *Electromagnetic waves carry information without a continuous cable between sender and receiver.* | Emit expanding spherical electromagnetic wave shells radiating from twin RF coils |
| `F3` | **televisión** (*television*) | `[TIER-D]` | Imagen y sonido se codifican, transmiten y reconstruyen en secuencias rápidas. / *Image and sound are encoded, transmitted and reconstructed in rapid sequences.* | Raster-scan glowing CRT/matrix phosphor beam reconstructing tiled screen image |
| `F4` | **fibra óptica** (*fiber optic*) | `[TIER-D]` | Pulsos de luz guiados por vidrio transportan enormes cantidades de datos con bajas pérdidas. / *Glass-guided light pulses carry enormous amounts of data with low loss.* | Guide laser light beams bouncing cleanly via total internal reflection inside clear fiber |
| `F5` | **red inalámbrica** (*wireless network*) | `[TIER-D]` | Radio, antenas, espectro compartido y protocolos coordinan dispositivos móviles. / *Radio, antennas, shared spectrum and protocols coordinate mobile devices.* | Tile hexagonal wireless cell tower grid linking roving handsets on shared radio spectrum |
| `F6` | **satélite** (*satellite*) `[kw:satelite]` | `[TIER-D]` | Enlaces orbitales cubren grandes regiones, aunque introducen distancia, coste y dependencia espacial. / *Orbital links cover large regions, although they introduce distance, cost and spatial dependency.* | Bounce narrow beam up from parabolic dish to high-altitude orbital relay satellite |
| `F7` | **señal digital** (*digital signal*) | `[TIER-C]` | Representar información con estados discretos facilita corrección, copia y procesamiento. / *Representing information with discrete states makes it easier to correct, copy, and process.* | Quantize smooth noisy analog waveform into razor-sharp binary square-wave pulses |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP19-Z05-09)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP19-Z05-09` (Mechanized Agriculture, Heat Engines, Mass Industry, Electromagnetism & Telecom) for HyperCosmosVR Odyssey Act V (`Exp 19: 19_technology`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z05`->`2905` (`F0..F7`), `Z06`->`2906` (`F0..F7`), `Z07`->`2907` (`F0..F7`), `Z08`->`2908` (`F0..F7`), `Z09`->`2909` (`F0..F7`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3802`, `3803`, `3804`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP19-Z05-09`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Specialize `select_compute_mode` (`crates/cosmos_render/src/universe_scene.rs`) + `shaders/compute_universe.wgsl` (`submode`/mode split) so early zones do not display anachronistic `mode=14` optical-bus particles.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z05: technological-agriculture / Technological agriculture, Z06: engines / Engines, Z07: industry / Industry, Z08: electricity / Electricity, Z09: telecommunications / Telecommunications.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp19-z10-14"></a>
### PKG-ACT5-EXP19-Z10-14: Transistor Electronics, Turing Computing, Software Abstractions, Internet & Robotics

- **Phase:** Phase 1/2 | **Priority:** `P2` | **Scope:** Exp `19` (`19_technology`) `Z10..Z14` (`5` zones, `40` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L65-L69`: `{0|u16::MAX -> 190, 16 -> 194, _ -> 190 + (sc % 7)}` (`197..=199` `else` lattice is dead code); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L205-L271` (`tech=0..6`: gears/Tesla/die+Transformer/helix; `L264-L270` dead; `feat` unused). Tier breakdown for this batch: **`0A / 2B / 2C / 1D` Zones**, **`0A / 14B / 12C / 14D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `0/5` zones and `0/40` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 2910..=2914` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3805`, `3806` (allocated from `3720 + (19 - 11)*10 + k`; `mode=14` Logic/Packet Bus Lattice + `doc_feat` Sub-Staging):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z10` | `electronics` (*Electrónica* / *Electronics*) | `[TIER-C]` | `vid=193` | **`2910`** | `3805` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x02` (0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |
| `Z11` | `computing` (*Computación* / *Computing*) | `[TIER-B]` | `vid=194` | **`2911`** | `3806` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z12` | `software` (*Software* / *Software*) | `[TIER-B]` | `vid=195` | **`2912`** | `3805` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z13` | `internet` (*Internet* / *internet*) | `[TIER-C]` | `vid=196` | **`2913`** | `3806` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z14` | `robotics` (*Robótica* / *Robotics*) | `[TIER-D]` | `vid=190` | **`2914`** | `3805` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z10` `electronics` (`vid=2910`) — Electrónica (*Electronics*) `[TIER-C]`

- **Catalog Metadata:** `scene=10` · `[0.500, 0.550]` · `rgb=[121, 255, 182]` · `sound=(sine/100.5Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Sensores + semiconductores + circuitos → detección, decisión y control` / *`Sensors + semiconductors + circuits → detection, decision and control`* · Evidence: `Control de señales mediante componentes microscópicos` (*Signal control using microscopic components*)
- **Spoken Macro Narrative (ES / EN):** La electrónica transforma señales débiles en medición, comunicación, memoria y acción controlada. (*EN:* Electronics transform weak signals into measurement, communication, memory and controlled action.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (10 % 7) = 193` (`tech=3`, Twin Tesla coils!). Electronics narrates vacuum tubes, transistors, PN diodes, sensors, integrated circuits, microcontrollers, and OLED/LCD displays—yet the **etched monolithic silicon wafer (`vid=2910 (extending legacy `vid=194`)`)** sits over on `Z04 materials` and `Z05 agriculture`! **Target (`vid=2910`):** Move **Silicon Die Core (`vid=2910 (extending legacy `vid=194`)` substrate)** here wrapped with `doc_feat` zoom transitions: glass vacuum triode -> planar CMOS MOSFET gate -> one-way PN diode -> sensor transducer -> microcontroller block -> emissive display subpixel grid.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **válvula de vacío** (*vacuum tube*) | `[TIER-D]` | Controlar electrones en un recipiente evacuado permitió amplificar y conmutar señales tempranas. / *Controlling electrons in an evacuated container allowed early signals to be amplified and switched.* | Render evacuated glass vacuum tube housing glowing cathode filament, grid & anode plate |
| `F1` | **transistor** (*transistor*) | `[TIER-C]` | Una señal pequeña puede controlar una corriente mayor dentro de un semiconductor. / *A small signal can control a larger current within a semiconductor.* | Gate large source-to-drain current river dynamically via tiny voltage on MOSFET gate |
| `F2` | **diodo** (*diode*) | `[TIER-C]` | Una unión semiconductor conduce de forma distinta según dirección y condiciones. / *A semiconductor junction conducts differently depending on direction and conditions.* | Light sharp asymmetric PN-junction depletion barrier passing one-way forward pulse |
| `F3` | **sensor** (*sensor*) | `[TIER-C]` | Un sensor convierte una propiedad física o química en una señal interpretable. / *A sensor converts a physical or chemical property into an interpretable signal.* | Convert external light/thermal/pressure touch ripples directly into clean mV signal wave |
| `F4` | **circuito integrado** (*integrated circuit*) | `[TIER-C]` | Muchos componentes fabricados juntos reducen tamaño, coste y distancia de señal. / *Many components manufactured together reduce size, cost and signal distance.* | Zoom onto monolithic etched silicon wafer die (`vid=2910 (extending legacy `vid=194`)`) flashing gold metal traces |
| `F5` | **microcontrolador** (*microcontroller*) | `[TIER-C]` | Procesador, memoria y periféricos reunidos controlan máquinas específicas. / *Processor, memory and peripherals put together control specific machines.* | Integrate CPU cube, memory cache & I/O bus pins on single compact embedded PCB module |
| `F6` | **pantalla** (*screen*) | `[TIER-D]` | Señales eléctricas modulan luz para convertir datos en imágenes visibles. / *Electrical signals modulate light to convert data into visible images.* | Illuminate close-up R/G/B subpixel emissive micro-grid rasterizing live pattern |
| `F7` | **procesamiento de señal** (*signal processing*) | `[TIER-C]` | Filtrar, muestrear y transformar señales revela información oculta por ruido o escala. / *Filtering, sampling, and transforming signals reveals information hidden by noise or scale.* | Run low-pass/bandpass Fourier harmonic filter carving signal clean out of jagged noise |

##### `Z11` `computing` (`vid=2911`) — Computación (*Computing*) `[TIER-B]`

- **Catalog Metadata:** `scene=11` · `[0.550, 0.600]` · `rgb=[150, 168, 255]` · `sound=(sawtooth/105.15Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Estados físicos + lógica + memoria + instrucciones → cómputo` / *`Physical states + logic + memory + instructions → computation`* · Evidence: `Manipulación física de símbolos según reglas` (*Physical manipulation of symbols according to rules*)
- **Spoken Macro Narrative (ES / EN):** Una computadora no comprende automáticamente lo que procesa: ejecuta operaciones físicas que representan símbolos según reglas diseñadas. (*EN:* A computer does not automatically understand what it processes: it executes physical operations that represent symbols according to designed rules.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (11 % 7) = 194` (`tech=4`, Silicon die + gold micro-bus + 3-layer Transformer web) + `mode=14` optical bus particles. Silicon chip + bus directly fits Computing (`[TIER-B]`), though Transformer neural graph layers look AI-specific, `doc_feat` is dead, and I/O devices (`f4,f5`) lack peripherals. **Target (`vid=2911`):** Keep `vid=2911 (extending legacy `vid=194`)` silicon die & gold micro-bus; let `doc_feat` expose **Binary High/Low Rails, Logic Gates (AND/OR/NOT), SRAM/DRAM Grid, ALU Clock, Stored Program Instruction Loop, and I/O Sensors**.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **estados binarios** (*binary states*) | `[TIER-B]` | Dos rangos físicos diferenciables pueden representar 0 y 1 de manera robusta. / *Two differentiable physical ranges can represent 0 and 1 robustly.* | Quantize fluctuating analog traces firmly into crisp cyan `0` / amber `1` binary voltage rails |
| `F1` | **puerta lógica** (*logic gate*) | `[TIER-B]` | Transistores conectados implementan relaciones como Y, O y NO. / *Connected transistors implement relationships such as AND, OR, and NOT.* | Route coupled silicon transistors cleanly through glowing AND / OR / NOT logic gate symbols |
| `F2` | **memoria** (*memory*) | `[TIER-B]` | Estados físicos persistentes conservan datos e instrucciones durante distintos intervalos. / *Persistent physical states retain data and instructions for various intervals.* | Preserve glowing persistent flip-flop cell grid lattice on left side of die |
| `F3` | **procesador** (*processor*) | `[TIER-B]` | Un procesador coordina operaciones elementales a gran velocidad. / *A processor coordinates elementary operations at high speed.* | Pulse rapid synchronized central clock tree sweeping millions of ALU trace gates |
| `F4` | **entrada** (*input*) | `[TIER-D]` | Teclados, sensores, cámaras y redes convierten acontecimientos externos en datos. / *Keyboards, sensors, cameras and networks convert external events into data.* | Attach tactile keypad/camera input bus streaming raw environmental signals onto die pad |
| `F5` | **salida** (*output*) | `[TIER-D]` | Pantallas, motores, sonidos y mensajes convierten resultados digitales en efectos observables. / *Screens, motors, sounds and messages convert digital results into observable effects.* | Connect stepper actuator & display output pad converting digital bits to motion/light |
| `F6` | **algoritmo** (*algorithm*) | `[TIER-B]` | Un algoritmo especifica pasos y decisiones para transformar una entrada en una salida. / *An algorithm specifies steps and decisions to transform an input into an output.* | Highlight multi-branch flowchart stepping illuminated packet sequentially from input to output |
| `F7` | **programa almacenado** (*stored program*) | `[TIER-B]` | Guardar instrucciones en memoria permite que la misma máquina ejecute tareas diferentes. / *Storing instructions in memory allows the same machine to execute different tasks.* | Loop gold instruction bus back out of memory grid feeding processor decode unit itself |

##### `Z12` `software` (`vid=2912`) — Software (*Software*) `[TIER-B]`

- **Catalog Metadata:** `scene=12` · `[0.600, 0.650]` · `rgb=[194, 144, 255]` · `sound=(sine/109.8Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Código + datos + abstracciones + hardware → comportamiento programable` / *`Code + data + abstractions + hardware → programmable behavior`* · Evidence: `Capas de instrucciones y modelos ejecutables` (*Instruction Layers and Executable Models*)
- **Spoken Macro Narrative (ES / EN):** El software oculta detalles para permitir nuevas capacidades, pero cada capa añade supuestos, errores y dependencias. (*EN:* Software hides details to enable new capabilities, but each layer adds assumptions, errors, and dependencies.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (12 % 7) = 195` (`tech=5`, Silicon die + gold micro-bus + 3-layer layered stack). Multi-layer stack matches Software's layered abstraction model (`[TIER-B]`), yet it is pixel-identical to `Z11` and `Z16` and ignores `doc_feat` across machine code, OS, DB, simulation, UI, libraries & VR worlds. **Target (`vid=2912`):** Transform upper 3-layer graph on `vid=2912 (extending legacy `vid=195`)` into a glowing **7-Layer Software Abstraction Tower** hovering cleanly above `tech=4` silicon base, isolating each layer per `doc_feat(0..7)`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **código máquina** (*machine code*) | `[TIER-B]` | Instrucciones numéricas controlan operaciones concretas del procesador. / *Numerical instructions control specific operations of the processor.* | Flash raw opcode hex/binary pulse packets hopping straight across lower silicon die pins |
| `F1` | **sistema operativo** (*operating system*) | `[TIER-B]` | El sistema operativo reparte memoria, tiempo de procesador, archivos y acceso a dispositivos. / *The operating system distributes memory, processor time, files and access to devices.* | Spin middle OS scheduler torus allocating time slices cleanly to 4 competing task queues |
| `F2` | **aplicación** (*application*) | `[TIER-B]` | Una aplicación organiza funciones técnicas alrededor de una tarea humana. / *An application organizes technical functions around a human task.* | Float top interactive application window shell framing clean human-centric controls |
| `F3` | **base de datos** (*database*) | `[TIER-B]` | Estructuras y consultas permiten conservar, relacionar y recuperar grandes colecciones de información. / *Structures and queries allow you to preserve, relate and retrieve large collections of information.* | Render structured 3D tabular/graph database cube traversed by glowing search query rays |
| `F4` | **simulación** (*simulation*) | `[TIER-B]` | Un modelo computacional explora cómo podría comportarse un sistema bajo supuestos definidos. / *A computational model explores how a system might behave under defined assumptions.* | Spin compact miniature physical-world simulation sphere nested inside software ring |
| `F5` | **interfaz** (*interface*) | `[TIER-B]` | Una interfaz traduce capacidades internas en acciones y señales comprensibles para personas u otros sistemas. / *An interface translates internal capabilities into actions and signals understandable by people or other systems.* | Illuminate thin glowing API/UI membrane translating user gesture into kernel signal |
| `F6` | **biblioteca** (*library*) | `[TIER-B]` | Código reutilizable acelera desarrollo y transmite decisiones técnicas entre proyectos. / *Reusable code speeds development and transmits technical decisions between projects.* | Snap pre-compiled modular cube blocks out of shared side repository directly into stack |
| `F7` | **mundo virtual** (*virtual world*) | `[TIER-B]` | Reglas de software pueden construir espacios interactivos que no obedecen todas las limitaciones físicas del mundo real. / *Software rules can build interactive spaces that do not obey all the physical limitations of the real world.* | Project glowing non-Euclidean wireframe virtual world portal rising atop highest layer |

##### `Z13` `internet` (`vid=2913`) — Internet (*internet*) `[TIER-C]`

- **Catalog Metadata:** `scene=13` · `[0.650, 0.700]` · `rgb=[105, 170, 255]` · `sound=(triangle/114.45Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Paquetes + direcciones + rutas + protocolos + infraestructura física → red mundial` / *`Packets + addresses + routes + protocols + physical infrastructure → global network`* · Evidence: `Sistema distribuido de redes interoperables` (*Distributed system of interoperable networks*)
- **Spoken Macro Narrative (ES / EN):** Internet no es una nube en el cielo: depende de cables, radioenlaces, centros de datos, energía y protocolos acordados. (*EN:* The Internet is not a cloud in the sky: it depends on cables, radio links, data centers, energy and agreed protocols.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (13 % 7) = 196` (`tech=6`, Biotech DNA Double Helix + Gene Gate!!). Internet routing, packets, IP addresses, servers, HTTP/TCP protocols, data centers, and social algorithms rendered as DNA nucleotides is pure modulo chaos. **Target (`vid=2913`):** **Global Packet-Switched Mesh & Server Cloud**: distributed router spheres joined by subsea cables carrying splitting/recombining color-coded IP packet cubes over planetary globe & solid terrestrial server-rack monolith.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **paquete** (*packet*) | `[TIER-C]` | Los datos se dividen en unidades que pueden viajar por rutas distintas y reunirse al final. / *Data is divided into units that can travel along different routes and be reunited at the end.* | Split monolithic glowing data block into numbered sub-packet cubes taking parallel paths |
| `F1` | **dirección** (*address*) | `[TIER-C]` | Identificadores permiten localizar interfaces y servicios dentro de redes cambiantes. / *Identifiers allow interfaces and services to be located within changing networks.* | Stamp glowing hexadecimal/numeric IP routing badges cleanly atop endpoint nodes |
| `F2` | **router** (*router*) | `[TIER-C]` | Un router elige el siguiente salto para cada paquete usando información de red. / *A router chooses the next hop for each packet using network information.* | Flash dynamic multi-port router hubs redirecting packet cubes around congested red link |
| `F3` | **servidor** (*server*) | `[TIER-D]` | Un servidor ofrece datos o funciones a otros equipos, desde una habitación o un centro de datos. / *A server provides data or functions to other computers, from a room or a data center.* | Render solid glowing monolithic server rack blade bank feeding client requests |
| `F4` | **protocolo** (*protocol*) | `[TIER-C]` | Reglas compartidas permiten que sistemas diseñados por organizaciones distintas se entiendan. / *Shared rules allow systems designed by different organizations to understand each other.* | Lock incompatible colored sender/receiver rings via uniform agreed handshake ring |
| `F5` | **sitio web** (*website*) | `[TIER-C]` | Documentos, estilos, programas y servicios se combinan para formar experiencias navegables. / *Documents, styles, programs and services combine to form navigable experiences.* | Unfold floating multi-pane hypertext document/style/script bundle card at client node |
| `F6` | **nube** (*cloud*) | `[TIER-D]` | La nube es computación remota distribuida en instalaciones físicas, no un lugar sin materia. / *The cloud is remote computing distributed in physical facilities, not a place without matter.* | Dispel deceptive misty cloud icon above to reveal solid cooled subterranean data center |
| `F7` | **plataforma social** (*social platform*) | `[TIER-C]` | Las plataformas coordinan publicación y atención, pero sus reglas y algoritmos concentran poder. / *Platforms coordinate publication and attention, but their rules and algorithms concentrate power.* | Weight bright central social platform hub redistributing viral feeds unevenly across mesh |

##### `Z14` `robotics` (`vid=2914`) — Robótica (*Robotics*) `[TIER-D]`

- **Catalog Metadata:** `scene=14` · `[0.700, 0.750]` · `rgb=[101, 242, 206]` · `sound=(sine/119.1Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Sensores → estimación → control → actuadores → entorno → nuevos sensores` / *`Sensors → estimation → control → actuators → environment → new sensors`* · Evidence: `Información digital convertida en acción física` (*Digital information converted into physical action*)
- **Spoken Macro Narrative (ES / EN):** Un robot conecta información digital con acciones físicas en el mundo. (*EN:* A robot connects digital information with physical actions in the world.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (14 % 7) = 190` (`tech=0`, two heavy cogwheels!). Robotics integrates **sensors -> state estimation -> feedback control -> actuators -> physical world**: neither articulated 6-DOF robot arms, end-effector grippers, LIDAR scanners, PID loops, quadrotor drones, nor exoskeletons exist anywhere in WGSL. **Target (`vid=2914`):** **6-DOF Articulated Robot Arm & Drone Arena**: sweeping raymarched multi-joint robotic arm (`sdCapsule` links + articulated 2-finger gripper) atop rotary pedestal holding LIDAR scan fan beside hovering quadrotor drone & PID loop ring.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **sensor** (*sensor*) | `[TIER-D]` | El robot necesita medir su propio estado y aspectos relevantes del entorno. / *The robot needs to measure its own state and relevant aspects of the environment.* | Sweep rotating cyan LIDAR fan & depth-camera frustum scanning surrounding obstacles |
| `F1` | **actuador** (*actuator*) | `[TIER-D]` | Motores, válvulas y materiales activos convierten señales en fuerza o movimiento. / *Motors, valves and active materials convert signals into force or motion.* | Drive rotary servo cylinders & pneumatic pistons cleanly articulating multi-joint arm |
| `F2` | **bucle de control** (*control loop*) | `[TIER-D]` | Comparar objetivo y resultado permite corregir continuamente una acción. / *Comparing objective and result allows you to continuously correct an action.* | Close glowing continuous circular sensor -> error -> PID controller -> motor feedback loop |
| `F3` | **navegación** (*navigation*) | `[TIER-D]` | Localización, mapas y planificación ayudan a moverse entre obstáculos e incertidumbre. / *Location, maps and planning help you navigate obstacles and uncertainty.* | Project floating SLAM occupancy grid + obstacle-avoiding A* path ahead of rover |
| `F4` | **manipulación** (*manipulation*) | `[TIER-D]` | Brazos y pinzas deben controlar posición, fuerza y contacto con objetos variados. / *Arms and grippers must control position, force and contact with various objects.* | Close compliant twin-finger robotic gripper jaws gently around floating target block |
| `F5` | **robot industrial** (*industrial robot*) | `[TIER-D]` | Movimientos repetibles aceleran tareas estructuradas, pero exigen diseño seguro del espacio. / *Repeatable movements speed up structured tasks, but require safe space design.* | Sweep heavy factory spot-welding robot arm cleanly inside luminous safety boundary cage |
| `F6` | **dron** (*drone*) | `[TIER-D]` | Un vehículo aéreo combina sustentación, sensores, comunicación y control rápido. / *An aerial vehicle combines lift, sensors, communication and rapid control.* | Hover quad-rotor spinning multi-propeller drone tilted dynamically against wind vector |
| `F7` | **máquina asistiva** (*assistive machine*) | `[TIER-D]` | La robótica puede ampliar movilidad e independencia cuando se diseña con las personas usuarias. / *Robotics can expand mobility and independence when designed with users.* | Wrap ergonomic assistive exoskeleton brace around faint translucent human limb silhouette |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP19-Z10-14)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP19-Z10-14` (Transistor Electronics, Turing Computing, Software Abstractions, Internet & Robotics) for HyperCosmosVR Odyssey Act V (`Exp 19: 19_technology`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z10`->`2910` (`F0..F7`), `Z11`->`2911` (`F0..F7`), `Z12`->`2912` (`F0..F7`), `Z13`->`2913` (`F0..F7`), `Z14`->`2914` (`F0..F7`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3805`, `3806`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP19-Z10-14`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Specialize `select_compute_mode` (`crates/cosmos_render/src/universe_scene.rs`) + `shaders/compute_universe.wgsl` (`submode`/mode split) so early zones do not display anachronistic `mode=14` optical-bus particles.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z10: electronics / Electronics, Z11: computing / Computing, Z12: software / Software, Z13: internet / internet, Z14: robotics / Robotics.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp19-z15-19"></a>
### PKG-ACT5-EXP19-Z15-19: Biotech CRISPR, AI Attention Transformers, Autonomy, Geoengineering & Open Futures

- **Phase:** Phase 1/2 | **Priority:** `P1` | **Scope:** Exp `19` (`19_technology`) `Z15..Z19` (`5` zones, `41` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L65-L69`: `{0|u16::MAX -> 190, 16 -> 194, _ -> 190 + (sc % 7)}` (`197..=199` `else` lattice is dead code); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L205-L271` (`tech=0..6`: gears/Tesla/die+Transformer/helix; `L264-L270` dead; `feat` unused). Tier breakdown for this batch: **`0A / 1B / 1C / 3D` Zones**, **`0A / 8B / 7C / 26D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `3/5` zones and `3/41` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 2915..=2919` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3807`, `3808`, `3809` (allocated from `3720 + (19 - 11)*10 + k`; `mode=6` Helix (`Z15`) -> `mode=14` Neural/Planet Bus (`Z16..Z19`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z15` | `biotechnology` (*Biotecnología* / *Biotechnology*) | `[TIER-C]` | `vid=191` | **`2915`** | `3807` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 semantic cue mask) | `0..7` (`0xFF` macro) |
| `Z16` | `artificial-intelligence` (*Inteligencia artificial* / *Artificial intelligence*) | `[TIER-B]` | `vid=194` | **`2916`** | `3808` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z17` | `autonomous-systems` (*Sistemas autónomos* / *Autonomous systems*) | `[TIER-D]` | `vid=193` | **`2917`** | `3809` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x02` (0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |
| `Z18` | `planetary-technologies` (*Tecnologías planetarias* / *Planetary technologies*) | `[TIER-D]` | `vid=194` | **`2918`** | `3807` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 semantic cue mask) | `0..7` (`0xFF` macro) |
| `Z19` | `technological-futures` (*Futuros tecnológicos* / *Technological futures*) | `[TIER-D]` | `vid=195` | **`2919`** | `3808` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 semantic cue mask) | `0..8` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z15` `biotechnology` (`vid=2915`) — Biotecnología (*Biotechnology*) `[TIER-C]`

- **Catalog Metadata:** `scene=15` · `[0.750, 0.800]` · `rgb=[151, 255, 128]` · `sound=(triangle/123.75Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `adn`
- **Metric & Evidence (ES / EN):** `Organismos + información biológica + ambiente + intervención → función modificada` / *`Organisms + biological information + environment + intervention → modified function`* · Evidence: `Sistemas vivos como medio técnico` (*Living systems as a technical medium*)
- **Spoken Macro Narrative (ES / EN):** Modificar organismos no comenzó con la genética moderna, pero las herramientas actuales aumentaron enormemente precisión, velocidad y consecuencias posibles. (*EN:* Modifying organisms did not begin with modern genetics, but today's tools greatly increased precision, speed, and possible consequences.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (15 % 7) = 191` (`tech=1`, mechanical steel cogwheels!!). Textbook modulo inversion: **DNA Double Helix + Gene Gate (`vid=2915 (extending legacy `vid=196`)`)** literally sits on `Z06 engines` and `Z13 internet` while Biotechnology itself gets two steel gears! **Target (`vid=2915`):** Reassign `Z15` to **Upgraded Biotech DNA & CRISPR Gate (`vid=2915 (extending legacy `196`)`)**: live gene-editing Cas9 clamp ring along DNA helix + glass fermentation bioreactor vat + antibody lock + synthetic bio-circuit + cellular tissue scaffold.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **fermentación** (*fermentation*) | `[TIER-D]` | Microorganismos transforman alimentos y materiales mediante metabolismos aprovechados por culturas humanas. / *Microorganisms transform food and materials through metabolisms used by human cultures.* | Bubble stainless steel bioreactor fermentation vat brewing glowing golden yeast/products |
| `F1` | **selección artificial** (*artificial selection*) | `[TIER-C]` | Elegir qué organismos se reproducen cambia rasgos poblacionales durante generaciones. / *Choosing which organisms reproduce changes population traits over generations.* | Sort & converge evolving trait orbs over multi-generation selective breeding funnel |
| `F2` | **vacuna** (*vaccine*) | `[TIER-D]` | Entrenar al sistema inmunitario reduce riesgo de enfermedad sin requerir padecerla plenamente. / *Training the immune system reduces disease risk without requiring a person to suffer the full illness.* | Present benign viral-like capsid antigen training Y-shaped glowing immune antibodies |
| `F3` | **secuenciación de ADN** (*DNA sequencing*) `[kw:adn]` | `[TIER-C]` | Leer secuencias genéticas permite comparar organismos, rastrear variantes y diseñar experimentos. / *Reading genetic sequences allows you to compare organisms, track variants, and design experiments.* | Slide optical laser sequencing scanner along unzipped DNA rung pairs reading ACGT |
| `F4` | **edición genética** (*gene editing*) | `[TIER-C]` | Herramientas moleculares modifican lugares concretos del genoma, con resultados y riesgos dependientes del contexto. / *Molecular tools modify specific places in the genome, with results and risks depending on the context.* | Dock scissor-like glowing Cas9 guide ring (`vid=2915 (extending legacy `vid=196`)` torus) splicing edited DNA segment |
| `F5` | **biología sintética** (*synthetic biology*) | `[TIER-C]` | Componentes biológicos se reorganizan para producir funciones nuevas o controladas. / *Biological components are reorganized to produce new or controlled functions.* | Wire glowing genetic toggle-switch rings cycling emerald/amber fluorescent reporter |
| `F6` | **ingeniería de tejidos** (*tissue engineering*) | `[TIER-D]` | Células, soportes y señales se combinan para reparar o construir estructuras biológicas. / *Cells, supports and signals combine to repair or build biological structures.* | Seed glowing stem cells into translucent 3D printed polymeric tissue lattice scaffold |
| `F7` | **fabricación biológica** (*biological manufacturing*) | `[TIER-C]` | Organismos pueden producir fármacos, alimentos, enzimas y materiales mediante procesos vivos. / *Organisms can produce drugs, foods, enzymes and materials through living processes.* | Emit stream of golden therapeutic drug & enzyme pearls from engineered bioreactor cell |

##### `Z16` `artificial-intelligence` (`vid=2916`) — Inteligencia artificial (*Artificial intelligence*) `[TIER-B]`

- **Catalog Metadata:** `scene=16` · `[0.800, 0.850]` · `rgb=[255, 135, 231]` · `sound=(sine/128.4Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Datos + objetivo + modelo + cómputo + evaluación → comportamiento aprendido` / *`Data + objective + model + computation + evaluation → learned behavior`* · Evidence: `Modelos que aprenden regularidades de datos` (*Models that learn data regularities*)
- **Spoken Macro Narrative (ES / EN):** La inteligencia artificial puede producir resultados complejos sin poseer necesariamente conciencia, comprensión humana o juicio fiable. (*EN:* Artificial intelligence can produce complex results without necessarily possessing consciousness, human understanding, or reliable judgment.)
- **Current Diagnosis & Target 3D Blueprint:** Explicit bespoke override `16 => 194` (`doc_stages.rs:67`) renders silicon die + gold micro-bus + 3-layer Transformer web (`tech=4`), paired with `mode=14` Transformer Token Highway. Strong 1:1 visual match (`[TIER-B]`), yet `doc_feat` is completely unused across classification, LLM attention, vision CNNs, recommenders, or RL! **Target (`vid=2916`):** Keep `vid=2916 (extending legacy `vid=194`)` Transformer die + `mode=14`; wire `doc_feat(0..7)` so each narration item drives distinct live neural-web behavior (hyperplane partition, attention heads, conv filter, RL reward loop).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **clasificación** (*classification*) | `[TIER-B]` | Un modelo asigna entradas a categorías aprendidas, con errores que reflejan datos, objetivo y contexto. / *A model maps inputs to learned categories, with errors reflecting data, purpose, and context.* | Carve translucent glowing hyperplane cleanly partitioning clustered blue/amber data orbs |
| `F1` | **predicción** (*prediction*) | `[TIER-B]` | Regularidades pasadas se usan para estimar resultados futuros, sin garantizar que las condiciones permanezcan iguales. / *Past regularities are used to estimate future results, without guaranteeing that conditions will remain the same.* | Project forward confidence cone from historical point sequence onto uncertain next step |
| `F2` | **modelo de lenguaje** (*language model*) | `[TIER-B]` | Un modelo de lenguaje estima continuaciones plausibles de símbolos y puede generar texto coherente sin verificar por sí mismo su verdad. / *A language model estimates plausible continuations of symbols and can generate coherent text without itself verifying its truth.* | Illuminate sparse autoregressive token-to-token self-attention arcs across 3 layers |
| `F3` | **reconocimiento de imágenes** (*image recognition*) | `[TIER-B]` | Patrones visuales se transforman en etiquetas, posiciones o representaciones internas. / *Visual patterns are transformed into labels, positions or internal representations.* | Slide glowing convolutional feature filter square extracting edges out of input image slab |
| `F4` | **recomendación** (*recommendation*) | `[TIER-B]` | Sistemas ordenan opciones según señales de preferencia, pero también moldean atención y comportamiento. / *Systems order options according to preference signals, but they also shape attention and behavior.* | Warp glowing user-preference web clustering items and pulling attention beacon inward |
| `F5` | **planificación** (*planning*) | `[TIER-B]` | Un sistema compara acciones posibles para acercarse a un objetivo definido. / *A system compares possible actions to get closer to a defined objective.* | Expand Monte-Carlo lookahead branching search tree scoring candidate moves toward goal |
| `F6` | **generación** (*generation*) | `[TIER-B]` | Modelos producen texto, imágenes, audio, código u otras estructuras a partir de patrones aprendidos. / *Models produce text, images, audio, code or other structures from learned patterns.* | Sweep latent diffusion denoising pass transforming Gaussian haze into crisp solid mesh |
| `F7` | **aprendizaje por refuerzo** (*reinforcement learning*) | `[TIER-B]` | La retroalimentación sobre acciones guía estrategias, aunque la recompensa elegida puede no representar el objetivo humano real. / *Action feedback guides strategies, although the chosen reward may not represent the actual human goal.* | Pulse reward/penalty feedback loop back from environment maze into neural layer weights |

##### `Z17` `autonomous-systems` (`vid=2917`) — Sistemas autónomos (*Autonomous systems*) `[TIER-D]`

- **Catalog Metadata:** `scene=17` · `[0.850, 0.900]` · `rgb=[255, 110, 175]` · `sound=(sawtooth/133.05Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Percepción + objetivos + decisiones + acción + supervisión → autonomía limitada` / *`Perception + objectives + decisions + action + supervision → limited autonomy`* · Evidence: `Acción sostenida con menor supervisión directa` (*Sustained action with less direct supervision*)
- **Spoken Macro Narrative (ES / EN):** La autonomía técnica no elimina la responsabilidad humana: la desplaza hacia quienes diseñan, entrenan, despliegan y gobiernan el sistema. (*EN:* Technical autonomy does not eliminate human responsibility: it shifts it to those who design, train, deploy and govern the system.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (17 % 7) = 193` (`tech=3`, Twin Tesla coils!). Autonomous cyber-physical systems—self-driving LIDAR vehicles, automated lab robots, HFT market cascades, drone swarms, smart grids, autonomous weapons—have zero counterpart in sparking Tesla coils. **Target (`vid=2917`):** **Multi-Domain Autonomous Control Matrix**: stylized LIDAR vehicle scanning busy street frame beside automated liquid-handling gantry, swarming boid drone cluster, and supervisory human-in-the-loop override tether.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **vehículo autónomo** (*autonomous vehicle*) | `[TIER-D]` | Sensores y modelos deben interpretar tráfico cambiante mientras los límites operativos siguen siendo decisivos. / *Sensors and models must interpret changing traffic while operational limits remain decisive.* | Drive sleek vehicle silhouette projecting 360-degree LIDAR rays avoiding bounding boxes |
| `F1` | **laboratorio automatizado** (*automated laboratory*) | `[TIER-D]` | Robots y software pueden ejecutar, medir y ajustar experimentos de manera continua. / *Robots and software can continuously run, measure, and adjust experiments.* | Run motorized XYZ robotic pipette gantry cycling colorful micro-well assay tray |
| `F2` | **sistema de negociación** (*trading system*) | `[TIER-D]` | Algoritmos actúan en mercados a velocidades que pueden amplificar errores e interacciones inesperadas. / *Algorithms act in markets at speeds that can amplify errors and unexpected interactions.* | Flash hyper-rapid oscillatory flash-crash spike propagating across market ticker lattice |
| `F3` | **flota robótica** (*robotic fleet*) | `[TIER-D]` | Múltiples máquinas coordinadas reparten tareas y comparten información. / *Multiple coordinated machines distribute tasks and share information.* | Coordinate 12 synchronized drone beacons forming distributed task formation autonomously |
| `F4` | **apoyo a decisiones** (*decision support*) | `[TIER-C]` | Un sistema puede resumir evidencia o estimar riesgo, pero no sustituye automáticamente responsabilidad ni contexto. / *A system can summarize evidence or estimate risk, but it does not automatically replace responsibility or context.* | Present translucent risk dashboard summary feeding upright human decision node above |
| `F5` | **infraestructura adaptativa** (*adaptive infrastructure*) | `[TIER-D]` | Redes eléctricas, edificios o tráfico ajustan operación según demanda y condiciones. / *Electrical networks, buildings or traffic adjust operation according to demand and conditions.* | Balance real-time variable demand loads smoothly across interconnected smart-grid city |
| `F6` | **arma autónoma** (*autonomous weapon*) | `[TIER-D]` | Delegar selección o ataque a máquinas crea riesgos extremos de control, atribución y escalada. / *Delegating targeting or attack to machines creates extreme control, attribution, and escalation risks.* | Flash harsh red targeting lock beam highlighting ethical/escalation risk tether cut |
| `F7` | **red autorreparable** (*self-healing network*) | `[TIER-D]` | Detección automática y rutas alternativas pueden mantener servicios, aunque no eliminan fallos sistémicos. / *Automatic detection and alternative routes can maintain services, although they do not eliminate systemic failures.* | Route damaged red mesh node cleanly around automatic self-healing backup bypass |

##### `Z18` `planetary-technologies` (`vid=2918`) — Tecnologías planetarias (*Planetary technologies*) `[TIER-D]`

- **Catalog Metadata:** `scene=18` · `[0.900, 0.950]` · `rgb=[90, 224, 255]` · `sound=(sine/137.7Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `tierra`
- **Metric & Evidence (ES / EN):** `Capacidad técnica ≠ viabilidad económica ≠ aceptación social ≠ legitimidad política` / *`Technical capacity ≠ economic viability ≠ social acceptance ≠ political legitimacy`* · Evidence: `Intervención sobre sistemas de escala terrestre` (*Intervention on terrestrial scale systems*)
- **Spoken Macro Narrative (ES / EN):** Actuar a escala planetaria exige distinguir lo físicamente posible de lo gobernable, justo y aceptable. (*EN:* Acting on a planetary scale requires distinguishing what is physically possible from what is governable, fair and acceptable.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (18 % 7) = 194` (`tech=4`, single flat silicon Transformer die!). Acting at terrestrial scale spans HVDC grids, ocean-atmosphere sensors, hyperscale data centers, DAC carbon capture, stratospheric geoengineering, mega-constellations, and container logistics—all absent. **Target (`vid=2918`):** **Planetary Geo-Engineering & Telemetry Earth Globe**: high-detail planet sphere ringed by Starlink mega-constellation shells, undersea HVDC/fiber arcs, atmospheric CO2 scrubber plumes, aerosol veil tint, shipping arteries, and biosphere eDNA mesh.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **red energética global** (*global energy network*) | `[TIER-D]` | Interconexiones extensas podrían compartir recursos variables, pero requieren seguridad, acuerdos y distribución justa. / *Extensive interconnections could share variable resources, but require security, agreements, and fair distribution.* | Thread ultra-high-voltage transcontinental energy bridges balancing sunny/windy hemispheres |
| `F1` | **observación climática** (*climate observation*) `[kw:tierra]` | `[TIER-D]` | Satélites, boyas, estaciones y modelos integran señales de toda la Tierra. / *Satellites, buoys, stations and models integrate signals from all over the Earth.* | Link floating ocean buoys, ground towers, and orbital weather satellites over global GCM mesh |
| `F2` | **computación planetaria** (*planetary computing*) | `[TIER-D]` | Centros de datos y redes coordinan información global mientras consumen energía, agua y materiales. / *Data centers and networks coordinate global information while consuming energy, water and materials.* | Show massive water/power cooling plumes feeding dense continental planetary compute hubs |
| `F3` | **eliminación de carbono** (*carbon removal*) | `[TIER-D]` | Capturar y almacenar dióxido de carbono puede complementar reducciones, no reemplazar automáticamente el abandono de emisiones. / *Capturing and storing carbon dioxide can complement emission reductions, but cannot automatically replace phasing out emissions.* | Spin modular direct-air-capture fan arrays pulling glowing amber $CO_2$ underground |
| `F4` | **geoingeniería** (*geoengineering*) | `[TIER-D]` | Intervenir deliberadamente en el clima plantea incertidumbres, efectos transfronterizos y problemas de legitimidad. / *Deliberately intervening in the climate raises uncertainties, cross-border effects and legitimacy issues.* | Dispense shimmering translucent stratospheric aerosol ribbon over unevenly lit climate hemispheres |
| `F5` | **constelación satelital** (*satellite constellation*) | `[TIER-D]` | Miles de satélites amplían conectividad y observación, pero alteran órbitas, cielos y riesgos de colisión. / *Thousands of satellites expand connectivity and observation, but alter orbits, skies and collision risks.* | Populate swarming golden multi-shell LEO satellite constellation streaks crossing sky |
| `F6` | **logística mundial** (*global logistics*) | `[TIER-D]` | Puertos, contenedores, software y transporte sincronizan cadenas de suministro vulnerables a perturbaciones. / *Ports, containers, software and transportation synchronize supply chains vulnerable to disruption.* | Synchronize maritime container routes & automated megaport cranes across global coasts |
| `F7` | **vigilancia de la biosfera** (*biosphere monitoring*) | `[TIER-D]` | Sensores y genética ambiental pueden detectar cambios ecológicos, siempre bajo decisiones sobre acceso y uso de datos. / *Sensors and environmental genetics can detect ecological changes, always under decisions about access and use of data.* | Pulse distributed emerald biosphere eDNA & acoustic ecosystem sensors guarding habitats |

##### `Z19` `technological-futures` (`vid=2919`) — Futuros tecnológicos (*Technological futures*) `[TIER-D]`

- **Catalog Metadata:** `scene=19` · `[0.950, 1.000]` · `rgb=[230, 240, 255]` · `sound=(triangle/142.35Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `tierra`
- **Metric & Evidence (ES / EN):** `Capacidades + objetivos + poder + gobernanza + azar → futuros divergentes` / *`Capabilities + objectives + power + governance + chance → divergent futures`* · Evidence: `Ramas abiertas · no un destino único` (*Open pathways · no single destination*)
- **Spoken Macro Narrative (ES / EN):** El futuro dependerá no solo de lo que podamos construir, sino de qué objetivos elijamos, quién controle los sistemas y cómo gobernemos sus consecuencias. (*EN:* The future will depend not only on what we can build, but on what objectives we choose, who controls the systems and how we govern their consequences.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via `190 + (19 % 7) = 195` (`tech=5`, flat Transformer silicon die!). Climax of Exp 19 (`Futuros tecnologicos`: clean fusion, ubiquitous surveillance, automation, medical regeneration, ecological restoration, AGI, space industry, human-AI teaming) needs diverging branching futures, not a static GPU die. **Target (`vid=2919`):** **Bifurcating Horizon Futures Prism**: central planetary core branching outward into diverging volumetric future corridors (clean tokamak fusion torus, panopticon sensor grid, biotech repair helix, rewilding green globe, AGI core, O'Neill orbital cylinder) bridging to Exp 20+.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **energía limpia abundante** (*abundant clean energy*) | `[TIER-D]` | Sistemas bajos en carbono podrían ampliar bienestar si materiales, redes y acceso se distribuyen con justicia. / *Low-carbon systems could expand well-being if materials, networks and access are distributed fairly.* | Ignite toroidal magnetic fusion plasma ring streaming clean abundant emerald energy across grid |
| `F1` | **vigilancia** (*surveillance*) | `[TIER-D]` | Sensores y análisis ubicuos pueden aumentar seguridad o concentrar control sobre la vida cotidiana. / *Ubiquitous sensors and analytics can increase security or concentrate control over everyday life.* | Sweep pervasive overlapping optical surveillance cones crisscrossing urban dome grid |
| `F2` | **automatización** (*automation*) | `[TIER-D]` | Máquinas pueden reducir trabajo peligroso o desplazar ingresos y poder sin instituciones adaptadas. / *Machines can reduce dangerous work or displace income and power without adapted institutions.* | Run dual-arm robotic fabricator vs displaced glowing human worker distribution balance |
| `F3` | **mejora médica** (*medical improvement*) | `[TIER-D]` | Terapias avanzadas podrían reparar cuerpos y ampliar capacidades, con profundas preguntas de acceso e identidad. / *Advanced therapies could repair bodies and expand capacities, with deep questions of access and identity.* | Repair cellular double-helix & vascular mesh cleanly with nanomedical golden repair swarm |
| `F4` | **restauración ecológica** (*ecological restoration*) | `[TIER-D]` | Biología, sensores y energía pueden apoyar recuperación de ecosistemas si las causas de degradación también cambian. / *Biology, sensors and energy can support ecosystem recovery if the causes of degradation also change.* | Expand lush bioluminescent green restoration waves reclaiming arid fractured landscape |
| `F5` | **inteligencia avanzada** (*advanced intelligence*) | `[TIER-D]` | Sistemas más generales podrían acelerar ciencia o crear riesgos difíciles de controlar y evaluar. / *More general systems could accelerate science or create risks that are difficult to control and evaluate.* | Suspend soaring hyper-dense crystalline general-intelligence polyhedron ringed by alignment cages |
| `F6` | **industria espacial** (*space industry*) `[kw:tierra]` | `[TIER-D]` | Recursos y asentamientos fuera de la Tierra podrían ampliar opciones sin eliminar conflictos humanos. / *Off-Earth resources and settlements could expand options without eliminating human conflict.* | Orbit spinning O'Neill space habitat cylinder & asteroid-mining tug beam beyond Moon ring |
| `F7` | **colaboración humano-máquina** (*human-machine collaboration*) | `[TIER-D]` | Las herramientas futuras pueden complementar juicio y creatividad cuando sus límites, incentivos y control permanecen visibles. / *Future tools can complement judgment and creativity when their limits, incentives, and control remain visible.* | Interlock organic translucent human silhouette + glowing assistive AI halo in balanced tandem |
| `F8` | **más allá** (*beyond*) | `[TIER-C]` | Cuando una civilización puede transformar planetas, crear inteligencias no biológicas y expandirse fuera de su mundo natal, surge una pregunta mayor: ¿qué caminos puede seguir en el futuro? / *When a civilization can transform planets, create non-biological intelligences, and expand outside its homeworld, a bigger question arises: what paths can it take in the future?* | Open solar-sage horizon portal launching toward `20_futures` (`Act 5 Stop 3/8`) |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP19-Z15-19)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP19-Z15-19` (Biotech CRISPR, AI Attention Transformers, Autonomy, Geoengineering & Open Futures) for HyperCosmosVR Odyssey Act V (`Exp 19: 19_technology`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z15`->`2915` (`F0..F7`), `Z16`->`2916` (`F0..F7`), `Z17`->`2917` (`F0..F7`), `Z18`->`2918` (`F0..F7`), `Z19`->`2919` (`F0..F8`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3807`, `3808`, `3809`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP19-Z15-19`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Specialize `select_compute_mode` (`crates/cosmos_render/src/universe_scene.rs`) + `shaders/compute_universe.wgsl` (`submode`/mode split) so early zones do not display anachronistic `mode=14` optical-bus particles.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z15: biotechnology / Biotechnology, Z16: artificial-intelligence / Artificial intelligence, Z17: autonomous-systems / Autonomous systems, Z18: planetary-technologies / Planetary technologies, Z19: technological-futures / Technological futures.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

### 4.3 Experience 20: `20_futures` — Espectro de los Futuros: de la Civilización Planetaria al Destino Cósmico

- **Odyssey & Spatial Placement:** Act 5 (`3/8`, overall stop #20) · `FUTURES_ARCS` (`SectorId::FUTURES_ARCS`, parent `SOLAR_SYSTEM`, `u = [8.2, 16.0]`, `focus_scale_exp = 11.8`, `1.0e16 m`) ([`topology.rs#L323-L334, L650`](../../crates/cosmos_spatial/src/topology.rs#L323-L334))
- **Experience Totals:** `19` Zones (`0A / 2B / 6C / 11D`) · `152` Features (`0A / 4B / 52C / 96D`)
- **Current Shader & Compute:** `doc_stages.rs#L70-L74`: `{0|u16::MAX -> 200, 16 -> 202, _ -> 200 + (sc % 4)}` (`fut=1` & `3` collapse to identical Dyson core) · `shaders/hero_doc_civilization_horizon.wgsl#L272-L338` (`fut=0` 5-branch arcs, `fut=2` O'Neill+Dyson, `else` Dyson mirrors; `feat` only nudges mirror azimuth at `L318`/`L332`) · `mode=14` (`universe_scene.rs#L322`, `compute_universe.wgsl#L331-L345`)

<a id="pkg-act5-exp20-z00-04"></a>
### PKG-ACT5-EXP20-Z00-04: Future Probability Tree, Clean Energy Transition, Automation, AGI & Synthetic Biotech

- **Phase:** Phase 1/2 | **Priority:** `P1` | **Scope:** Exp `20` (`20_futures`) `Z00..Z04` (`5` zones, `40` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L70-L74`: `{0|u16::MAX -> 200, 16 -> 202, _ -> 200 + (sc % 4)}` (`fut=1` & `3` collapse to identical Dyson core); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L272-L338` (`fut=0` 5-branch arcs, `fut=2` O'Neill+Dyson, `else` Dyson mirrors; `feat` only nudges mirror azimuth at `L318`/`L332`). Tier breakdown for this batch: **`0A / 1B / 1C / 3D` Zones**, **`0A / 1B / 6C / 33D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `1/40` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3000..=3004` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3810`, `3811` (allocated from `3720 + (20 - 11)*10 + k`; Sector-Differentiated `mode=14` Token/Power Bus + `Z04` Bio-Grid):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z00` | `technological-futures` (*Futuros tecnológicos* / *Technological futures*) | `[TIER-B]` | `vid=200` | **`3000`** | `3810` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z01` | `energy-transition` (*Transición energética* / *Energy transition*) | `[TIER-D]` | `vid=201` | **`3001`** | `3811` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 semantic cue mask) | `0..7` (`0xFF` macro) |
| `Z02` | `automation` (*Automatización* / *Automation*) | `[TIER-D]` | `vid=202` | **`3002`** | `3810` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x02` (0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |
| `Z03` | `advanced-intelligence` (*Inteligencia avanzada* / *Advanced intelligence*) | `[TIER-C]` | `vid=203` | **`3003`** | `3811` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x01` (0x01 cutaway) | `0..7` (`0xFF` macro) |
| `Z04` | `future-biotechnology` (*Biotecnología futura* / *Future biotechnology*) | `[TIER-D]` | `vid=200` | **`3004`** | `3810` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z00` `technological-futures` (`vid=3000`) — Futuros tecnológicos (*Technological futures*) `[TIER-B]`

- **Catalog Metadata:** `scene=0` · `[0.000, 0.053]` · `rgb=[226, 240, 255]` · `sound=(sine/52Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Tendencias + posibilidades físicas + decisiones humanas → escenarios abiertos` / *`Trends + physical possibilities + human decisions → open scenarios`* · Evidence: `Punto de partida · escenarios, no profecías` (*Starting point · scenarios, not prophecies*)
- **Spoken Macro Narrative (ES / EN):** Las proyecciones del futuro no son profecías: cartografían posibilidades condicionadas por decisiones todavía abiertas. (*EN:* Projections of the future are not prophecies: they map possibilities conditioned by decisions that are still open.)
- **Current Diagnosis & Target 3D Blueprint:** Bespoke z==0 -> vid=3000 (`fut=0`: 5 multicolor bifurcating probability arcs) matches open scenario cartography conceptually (`F07` systemic branching), but `doc_feat` is completely unread so none of the 8 planetary drivers/infrastructures trigger visual nodes. **Target (`vid=3000`):** Upgrade vid=3000 Annotated Scenario Tree: retain 5 color-coded probability ribbons and add interactive origin anchor sphere + per-branch beacons.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **presente planetario** (*planetary present*) | `[TIER-D]` | La humanidad ya modifica sistemas planetarios mediante energía, agricultura, ciudades, redes, industria y decisiones colectivas. / *Humanity is already modifying planetary systems through energy, agriculture, cities, networks, industry and collective decisions.* | glowing Earth city/grid root sphere |
| `F1` | **energía limpia** (*clean energy*) | `[TIER-D]` | Las tecnologías bajas en carbono pueden reducir daños, pero exigen materiales, redes, mantenimiento y acceso equitativo. / *Low-carbon technologies can reduce harm, but require materials, networks, maintenance and equitable access.* | solar+transmission tower node |
| `F2` | **inteligencia artificial** (*artificial intelligence*) | `[TIER-C]` | La inteligencia artificial amplía capacidades de análisis y creación mientras introduce nuevas dependencias, errores y concentraciones de poder. / *Artificial intelligence expands analysis and creation capabilities while introducing new dependencies, errors, and concentrations of power.* | neural network prism on magenta branch |
| `F3` | **biotecnología** (*biotechnology*) | `[TIER-D]` | Editar y diseñar sistemas vivos abre posibilidades médicas y ecológicas junto con riesgos heredables y difíciles de revertir. / *Editing and designing living systems opens up medical and ecological possibilities along with heritable and difficult-to-reverse risks.* | glowing DNA double-helix on emerald branch |
| `F4` | **automatización** (*automation*) | `[TIER-D]` | Delegar tareas físicas y cognitivas puede elevar productividad o desplazar ingresos, control y responsabilidad. / *Delegating physical and cognitive tasks can increase productivity or displace income, control and responsibility.* | kinetic robotic arm vector |
| `F5` | **infraestructura planetaria** (*planetary infrastructure*) | `[TIER-D]` | Satélites, centros de datos, cadenas logísticas y redes energéticas conectan soluciones y vulnerabilidades a escala mundial. / *Satellites, data centers, logistics chains and energy networks connect solutions and vulnerabilities on a global scale.* | orbital satellite+data ring |
| `F6` | **restauración ecológica** (*ecological restoration*) | `[TIER-D]` | La tecnología puede apoyar la recuperación de ecosistemas, pero no sustituye cambios en extracción, consumo y gobernanza. / *Technology can support the recovery of ecosystems, but it does not replace changes in extraction, consumption and governance.* | regenerating forest canopy slice |
| `F7` | **riesgo sistémico** (*systemic risk*) | `[TIER-B]` | Cuanto más capaces e interdependientes son los sistemas, mayor puede ser el alcance de errores, abusos y fallos en cascada. / *The more capable and interdependent the systems are, the greater the scope for errors, abuses, and cascading failures.* | cascading ripple fracture along interdependent arcs |

##### `Z01` `energy-transition` (`vid=3001`) — Transición energética (*Energy transition*) `[TIER-D]`

- **Catalog Metadata:** `scene=1` · `[0.053, 0.105]` · `rgb=[255, 221, 105]` · `sound=(triangle/56.75Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `fusion`
- **Metric & Evidence (ES / EN):** `Generación + redes + almacenamiento + eficiencia + demanda → sistema energético transformado` / *`Generation + networks + storage + efficiency + demand → transformed energy system`* · Evidence: `Reconstrucción de la base material de la civilización` (*Reconstruction of the material base of civilization*)
- **Spoken Macro Narrative (ES / EN):** Una transición energética no consiste únicamente en inventar nuevas fuentes, sino en reconstruir infraestructuras completas. (*EN:* An energy transition is not only about inventing new sources, but about rebuilding complete infrastructures.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via 200 + (1 % 4) = 201 (`fut=1` central star + 3 Dyson collector rings). Narration focuses concretely on solar PV, wind turbines, nuclear fission reactors, grid storage, EV electrification, smart grids, synfuels, and tokamak experimental fusion — zero terrestrial/tokamak hardware exists on screen. **Target (`vid=3001`):** Energy System Infrastructure Hub (vid=3001): Earth sector platform orbiting a multi-technology power ring.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **energía solar** (*solar energy*) | `[TIER-D]` | Los paneles convierten luz en electricidad variable y requieren redes, almacenamiento, espacio, materiales y reciclaje. / *The panels convert light into variable electricity and require networking, storage, space, materials and recycling.* | directional silicon solar array panels tracking beam |
| `F1` | **energía eólica** (*wind energy*) | `[TIER-D]` | Las turbinas aprovechan corrientes de aire; su producción cambia con el clima y la ubicación. / *Turbines take advantage of air currents; Its production changes with climate and location.* | 3-blade spinning wind rotor cluster |
| `F2` | **energía nuclear** (*nuclear energy*) | `[TIER-D]` | La fisión puede producir electricidad baja en carbono de forma continua, con exigencias de seguridad, residuos, costes y confianza pública. / *Fission can produce low-carbon electricity continuously, with demands on safety, waste, costs and public trust.* | Cherenkov-blue fission reactor core + cooling tower torus |
| `F3` | **almacenamiento** (*storage*) | `[TIER-D]` | Baterías, calor, combustibles, bombeo y otras tecnologías desplazan energía entre momentos de abundancia y necesidad. / *Batteries, heat, fuels, pumping and other technologies move energy between times of abundance and need.* | glowing electrolyte battery stacks + pumped hydro loop |
| `F4` | **electrificación** (*electrification*) | `[TIER-D]` | Vehículos, calefacción e industria pueden sustituir combustión directa por electricidad cuando la red y los equipos se adaptan. / *Vehicles, heating and industry can replace direct combustion with electricity when the grid and equipment adapt.* | electric vehicle & inductive induction grid lines |
| `F5` | **redes inteligentes** (*smart networks*) | `[TIER-D]` | Sensores, control y mercados coordinan millones de fuentes y cargas sin eliminar vulnerabilidades físicas o digitales. / *Sensors, control and markets coordinate millions of sources and loads without eliminating physical or digital vulnerabilities.* | pulsed node-to-node smart-grid balancing packets |
| `F6` | **combustibles sintéticos** (*synthetic fuels*) | `[TIER-D]` | Electricidad, hidrógeno y carbono capturado pueden producir combustibles para usos difíciles de electrificar, con pérdidas energéticas importantes. / *Electricity, hydrogen and captured carbon can produce fuels for uses that are difficult to electrify, with significant energy losses.* | electrolysis H2/CO2->fuel catalytic reactor |
| `F7` | **fusión experimental** (*experimental fusion*) `[kw:fusion]` | `[TIER-D]` | La fusión busca aprovechar reacciones nucleares ligeras, pero todavía requiere superar desafíos científicos, industriales y económicos antes de sostener redes eléctricas. / *Fusion seeks to take advantage of light nuclear reactions, but still requires overcoming scientific, industrial and economic challenges before sustaining electrical networks.* | toroidal D-shaped magnet coils enclosing glowing pink tokamak fusion plasma ring |

##### `Z02` `automation` (`vid=3002`) — Automatización (*Automation*) `[TIER-D]`

- **Catalog Metadata:** `scene=2` · `[0.105, 0.158]` · `rgb=[255, 157, 88]` · `sound=(sine/61.5Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Sensores + modelos + actuadores + organización → trabajo parcialmente automatizado` / *`Sensors + models + actuators + organization → partially automated work`* · Evidence: `Transferencia de tareas a sistemas mecánicos y digitales` (*Transfer of tasks to mechanical and digital systems*)
- **Spoken Macro Narrative (ES / EN):** Automatizar una tarea no elimina el trabajo humano: cambia dónde se concentra, quién lo controla y quién recibe sus beneficios. (*EN:* Automating a task does not eliminate human work: it changes where it is concentrated, who controls it, and who receives its benefits.)
- **Current Diagnosis & Target 3D Blueprint:** Accidental modulo collision `200 + (2 % 4) = 202` shows Z16's Rotating O'Neill space cylinder + Dyson star! Narration discusses autonomous factories, warehouse logistics, self-driving transport, precision ag-drones, software administration, robotic wet labs, care robotics, and generative creativity. **Target (`vid=3002`):** Autonomous Physical-Digital Mesh (vid=3002).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **fábricas autónomas** (*autonomous factories*) | `[TIER-D]` | Robots, visión artificial y planificación coordinan producción, aunque siguen dependiendo de mantenimiento, materiales y objetivos humanos. / *Robots, computer vision and planning coordinate production, although they remain dependent on maintenance, materials and human objectives.* | 6-DOF articulated factory assembly arm welding chassis |
| `F1` | **logística** (*logistics*) | `[TIER-D]` | Almacenes, rutas y puertos automatizados pueden acelerar flujos y propagar interrupciones con gran rapidez. / *Automated warehouses, routes and ports can accelerate flows and propagate disruptions very quickly.* | pulsing supply-chain conveyor & shipping container DAG |
| `F2` | **transporte** (*transportation*) | `[TIER-D]` | Vehículos automatizados operan dentro de límites concretos y deben responder a entornos sociales difíciles de predecir. / *Automated vehicles operate within specific limits and must respond to social environments that are difficult to predict.* | LiDAR-raycasting autonomous rover navigating obstacles |
| `F3` | **agricultura** (*agriculture*) | `[TIER-D]` | Sensores, drones y maquinaria pueden aplicar agua, nutrientes o cosecha con precisión, pero también concentrar datos y capital. / *Sensors, drones and machinery can apply water, nutrients or crops with precision, but also concentrate data and capital.* | hex-rotor ag-drone misting gridded terrain |
| `F4` | **administración** (*administration*) | `[TIER-D]` | Software puede clasificar solicitudes y asignar recursos, haciendo decisivos la transparencia, la apelación y la calidad de los datos. / *Software can classify requests and allocate resources, making transparency, appeal and data quality decisive.* | algorithmic triage routing gate |
| `F5` | **laboratorios** (*laboratories*) | `[TIER-D]` | Equipos robotizados ejecutan ciclos experimentales rápidos y reproducibles bajo hipótesis y criterios definidos. / *Robotic teams execute rapid and reproducible experimental cycles under defined hypotheses and criteria.* | robotic liquid-handling micropipette gantry |
| `F6` | **cuidados** (*care*) | `[TIER-D]` | Máquinas pueden asistir movilidad, vigilancia y tareas rutinarias sin reemplazar automáticamente confianza, empatía ni responsabilidad. / *Machines can assist mobility, surveillance, and routine tasks without automatically replacing trust, empathy, or responsibility.* | soft-actuated assistive exoskeleton arm |
| `F7` | **trabajo creativo** (*creative work*) | `[TIER-C]` | Herramientas generativas amplían bocetos, escritura, música y diseño mientras alteran autoría, remuneración y acceso profesional. / *Generative tools expand sketching, writing, music, and design while altering authorship, compensation, and professional access.* | dual human-vector stylus & generative lattice wave |

##### `Z03` `advanced-intelligence` (`vid=3003`) — Inteligencia avanzada (*Advanced intelligence*) `[TIER-C]`

- **Catalog Metadata:** `scene=3` · `[0.158, 0.211]` · `rgb=[255, 126, 226]` · `sound=(triangle/66.25Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Capacidad + objetivos + acceso a herramientas + supervisión → efectos difíciles de anticipar` / *`Capacity + objectives + access to tools + supervision → effects difficult to anticipate`* · Evidence: `Capacidades futuras inciertas · control no garantizado` (*Uncertain future capabilities · control not guaranteed*)
- **Spoken Macro Narrative (ES / EN):** Una inteligencia más capaz no es automáticamente más consciente, benevolente ni controlable. (*EN:* A more capable intelligence is not automatically more conscious, benevolent, or controllable.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via 200 + (3 % 4) = 203 (`fut=3` central star + 3 Dyson mirror rings; `doc_feat` only shifts cosine phase). Dyson swarm around a star has no visual relationship to AGI generalization, autonomous lab control, long-horizon planning, recursive tool use, multi-agent coordination, or alignment guardrails. **Target (`vid=3003`):** Recursive Intelligence & Alignment Cage (vid=3003): central multi-layer tensor core nested inside concentric containment shells.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **resolución general** (*general problem-solving*) | `[TIER-C]` | Un sistema más flexible podría transferir aprendizaje entre dominios sin compartir necesariamente comprensión, valores o experiencia humana. / *A more flexible system could transfer learning between domains without necessarily sharing human understanding, values, or experience.* | cross-domain bridging synapses |
| `F1` | **investigación autónoma** (*autonomous research*) | `[TIER-D]` | Modelos conectados a instrumentos podrían proponer, ejecutar y evaluar experimentos con velocidad superior a la supervisión humana. / *Models connected to instruments could propose, execute and evaluate experiments with speed superior to human supervision.* | closed-loop autonomous AI-to-lab-instrument feedback beam |
| `F2` | **planificación estratégica** (*strategic planning*) | `[TIER-C]` | Comparar cadenas largas de acciones aumenta utilidad y también capacidad de manipulación, evasión o búsqueda de objetivos mal definidos. / *Comparing long chains of actions increases usefulness and also the capacity for manipulation, evasion or pursuit of ill-defined objectives.* | branching lookahead Monte-Carlo search tree cones |
| `F3` | **uso recursivo de herramientas** (*recursive use of tools*) | `[TIER-D]` | Un sistema puede encadenar código, búsqueda, simulación y robots, amplificando tanto competencias como errores. / *A system can chain together code, search, simulation, and robots, amplifying both skills and errors.* | recursive nested code/robot/simulator rings |
| `F4` | **descubrimiento científico** (*scientific discovery*) | `[TIER-D]` | La inteligencia artificial puede explorar espacios inmensos de moléculas, materiales y teorías, pero los resultados necesitan validación física. / *Artificial intelligence can explore vast spaces of molecules, materials and theories, but the results need physical validation.* | 3D molecular protein folding cloud |
| `F5` | **coordinación de máquinas** (*machine coordination*) | `[TIER-D]` | Muchos agentes conectados pueden repartir tareas y crear conductas colectivas no previstas por ningún componente aislado. / *Many connected agents can distribute tasks and create collective behaviors not anticipated by any isolated component.* | decentralized swarming agent node mesh |
| `F6` | **alineación** (*alignment*) | `[TIER-C]` | Alinear significa hacer que el comportamiento siga intenciones humanas robustas bajo situaciones nuevas, no solo obedecer ejemplos conocidos. / *Alignment means making behavior follow robust human intentions under new situations, not just obeying known examples.* | transparent safety boundary shell aligning internal vector arrows toward human-anchor sphere |
| `F7` | **control y verificación** (*control and verification*) | `[TIER-C]` | Límites de acceso, pruebas, interpretación y supervisión reducen riesgos sin demostrar que un sistema complejo permanecerá controlable en todo contexto. / *Limits on access, testing, interpretation, and monitoring reduce risks without demonstrating that a complex system will remain controllable in all contexts.* | mechanistic interpretability cutaway probes |

##### `Z04` `future-biotechnology` (`vid=3004`) — Biotecnología futura (*Future biotechnology*) `[TIER-D]`

- **Catalog Metadata:** `scene=4` · `[0.211, 0.263]` · `rgb=[143, 255, 151]` · `sound=(sine/71Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Información biológica + edición + cultivo + ambiente → organismos y funciones modificados` / *`Biological information + editing + cultivation + environment → modified organisms and functions`* · Evidence: `Intervención creciente sobre cuerpos, herencia y ecosistemas` (*Growing intervention on bodies, inheritance and ecosystems*)
- **Spoken Macro Narrative (ES / EN):** Tratamiento, mejora, modificación heredable y transformación de ecosistemas son intervenciones distintas con consecuencias distintas. (*EN:* Treatment, improvement, heritable modification and ecosystem transformation are different interventions with different consequences.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps via 200 + (4 % 4) = 200 (`fut=0` 5 color probability ribbons, `doc_feat` unused). Narration centers on concrete wet-lab & cellular biology: genomes, CRISPR gene editing, engineered microbes, bioprinted organ scaffolds, cellular senescence, cortical neural electrodes, ecosystem gene-drive engineering, and molecular bio-computing. **Target (`vid=3004`):** Synthetic Biology & Bio-Engineering Helix Stage (vid=3004): glowing procedural DNA double-helix central column surrounded by modular bio-voxels.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **medicina personalizada** (*personalized medicine*) | `[TIER-D]` | Genomas, historiales y biomarcadores pueden orientar tratamientos individuales sin convertir las predicciones en certezas. / *Genomes, histories and biomarkers can guide individual treatments without turning predictions into certainties.* | helical base-pair barcode scan |
| `F1` | **edición genética** (*genetic editing*) | `[TIER-D]` | Modificar secuencias concretas puede tratar enfermedades o crear cambios involuntarios, especialmente cuando afectan células reproductivas. / *Modifying specific sequences can treat diseases or create involuntary changes, especially when they affect reproductive cells.* | CRISPR-Cas molecular scissor ring slicing & replacing glowing strand segment |
| `F2` | **organismos sintéticos** (*synthetic organisms*) | `[TIER-D]` | Circuitos genéticos diseñados pueden producir medicamentos, materiales o combustibles y también escapar de condiciones previstas. / *Engineered genetic circuits can produce drugs, materials or fuels and also escape predicted conditions.* | synthetic lipid-capsule bacterium firing metabolic toggle switch |
| `F3` | **ingeniería de órganos** (*organ engineering*) | `[TIER-D]` | Células, andamios y señales buscan reparar o fabricar tejidos funcionales compatibles con cada paciente. / *Cells, scaffolds and signals seek to repair or manufacture functional tissues compatible with each patient.* | layered vascularized tissue/heart-valve bioprinted scaffold |
| `F4` | **longevidad** (*longevity*) | `[TIER-D]` | Investigar envejecimiento podría extender años saludables, con incertidumbres biológicas y profundas consecuencias distributivas. / *Researching aging could extend healthy years, with biological uncertainties and profound distributional consequences.* | telomere/cellular repair pulse ring |
| `F5` | **interfaces neuronales** (*neural interfaces*) | `[TIER-D]` | Electrodos y otros sistemas pueden leer o estimular actividad cerebral, aumentando capacidad terapéutica y sensibilidad de los datos mentales. / *Electrodes and other systems can read or stimulate brain activity, increasing therapeutic capacity and sensitivity of mental data.* | flexible multi-site micro-electrode array threading cortical surface |
| `F6` | **ingeniería ecológica** (*ecological engineering*) | `[TIER-D]` | Intervenir poblaciones o hábitats puede restaurar funciones y producir efectos evolutivos que cruzan fronteras. / *Intervening populations or habitats can restore functions and produce evolutionary effects that cross borders.* | watershed ecological population wave |
| `F7` | **computación biológica** (*biological computing*) | `[TIER-D]` | Células y moléculas pueden almacenar información o ejecutar operaciones con propiedades muy diferentes de la electrónica convencional. / *Cells and molecules can store information or execute operations with very different properties from conventional electronics.* | DNA-strand parallel logic computing lattice |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP20-Z00-04)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP20-Z00-04` (Future Probability Tree, Clean Energy Transition, Automation, AGI & Synthetic Biotech) for HyperCosmosVR Odyssey Act V (`Exp 20: 20_futures`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z00`->`3000` (`F0..F7`), `Z01`->`3001` (`F0..F7`), `Z02`->`3002` (`F0..F7`), `Z03`->`3003` (`F0..F7`), `Z04`->`3004` (`F0..F7`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3810`, `3811`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP20-Z00-04`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Specialize `select_compute_mode` (`crates/cosmos_render/src/universe_scene.rs`) + `shaders/compute_universe.wgsl` (`submode`/mode split) so early zones do not display anachronistic `mode=14` optical-bus particles.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z00: technological-futures / Technological futures, Z01: energy-transition / Energy transition, Z02: automation / Automation, Z03: advanced-intelligence / Advanced intelligence, Z04: future-biotechnology / Future biotechnology.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp20-z05-09"></a>
### PKG-ACT5-EXP20-Z05-09: Human-AI Symbiosis, Doughnut Sustainability, Tech Divide, X-Risks & Governance

- **Phase:** Phase 1/2 | **Priority:** `P1` | **Scope:** Exp `20` (`20_futures`) `Z05..Z09` (`5` zones, `40` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L70-L74`: `{0|u16::MAX -> 200, 16 -> 202, _ -> 200 + (sc % 4)}` (`fut=1` & `3` collapse to identical Dyson core); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L272-L338` (`fut=0` 5-branch arcs, `fut=2` O'Neill+Dyson, `else` Dyson mirrors; `feat` only nudges mirror azimuth at `L318`/`L332`). Tier breakdown for this batch: **`0A / 0B / 2C / 3D` Zones**, **`0A / 0B / 18C / 22D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `1/40` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3005..=3009` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3812`, `3813`, `3814` (allocated from `3720 + (20 - 11)*10 + k`; Coupled Socio-Technical & Risk Topology (`mode=14` + `cs_field_grid` shockwaves)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z05` | `human-machine-collaboration` (*Colaboración humano–máquina* / *Human-machine collaboration*) | `[TIER-D]` | `vid=201` | **`3005`** | `3812` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z06` | `sustainable-civilization` (*Civilización sostenible* / *Sustainable civilization*) | `[TIER-D]` | `vid=202` | **`3006`** | `3813` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z07` | `technological-inequality` (*Desigualdad tecnológica* / *Technological inequality*) | `[TIER-C]` | `vid=203` | **`3007`** | `3814` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z08` | `global-risks` (*Riesgos globales* / *Global risks*) | `[TIER-D]` | `vid=200` | **`3008`** | `3812` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x06` (0x04 semantic cue mask, 0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |
| `Z09` | `planetary-governance` (*Gobernanza planetaria* / *Planetary governance*) | `[TIER-C]` | `vid=201` | **`3009`** | `3813` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z05` `human-machine-collaboration` (`vid=3005`) — Colaboración humano–máquina (*Human-machine collaboration*) `[TIER-D]`

- **Catalog Metadata:** `scene=5` · `[0.263, 0.316]` · `rgb=[118, 220, 255]` · `sound=(sawtooth/75.75Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Persona + interfaz + modelo + cuerpo + comunidad → capacidad compartida` / *`Person + interface + model + body + community → shared capacity`* · Evidence: `Herramientas que se aproximan a extensiones de la persona` (*Tools that approach extensions of the person*)
- **Spoken Macro Narrative (ES / EN):** El futuro puede transformar a las personas sin reemplazarlas ni dejarlas intactas. (*EN:* The future can transform people without replacing them or leaving them intact.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps via 200 + (5 % 4) = 201 (`fut=1` central star + 3 Dyson rings). Narration covers cognitive assistants, bionic sensor-motor prosthetics, brain-computer interfaces, augmented sensory perception (IR/UV/EM), collective intelligence, robotic telepresence bodies, shared decisions, and persistent digital memory. **Target (`vid=3005`):** Symbiotic Human-Machine Interface Rig (vid=3005): translucent holographic human neural silhouette coupled via fiber filaments to robotic/digital peripherals.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **asistentes cognitivos** (*cognitive assistants*) | `[TIER-C]` | Sistemas personales pueden recordar, traducir, analizar y proponer acciones, haciendo crucial quién controla su memoria y sus objetivos. / *Personal systems can remember, translate, analyze and propose actions, making it crucial who controls your memory and your goals.* | personal memory crystal orbiting crown |
| `F1` | **prótesis avanzadas** (*advanced prosthetics*) | `[TIER-D]` | Sensores y actuadores pueden devolver movimiento o percepción y convertirse en capacidades nuevas, no solo sustituciones. / *Sensors and actuators can return motion or perception and become new capabilities, not just replacements.* | articulated carbon-tendon bionic hand with tactile force rings |
| `F2` | **interfaces cerebro–computadora** (*brain-computer interfaces*) | `[TIER-D]` | Conectar actividad neural con máquinas puede apoyar comunicación y control bajo límites de precisión, invasividad y privacidad. / *Connecting neural activity to machines can support communication and control under limits of precision, invasiveness, and privacy.* | cortical BCI crown translating spike trains into control vectors |
| `F3` | **percepción aumentada** (*augmented perception*) | `[TIER-D]` | Visión, audición y sensores externos pueden mostrar información invisible al cuerpo humano. / *Vision, hearing, and external sensors can display information invisible to the human body.* | multi-spectral EM/infrared halo expanding vision cone |
| `F4` | **inteligencia colectiva** (*collective intelligence*) | `[TIER-C]` | Personas y modelos conectados pueden combinar conocimientos diversos o amplificar sesgos compartidos. / *Connected people and models can combine diverse knowledge or amplify shared biases.* | bilateral crowd-to-AI consensus graph |
| `F5` | **encarnación robótica** (*robotic embodiment*) | `[TIER-D]` | Controlar cuerpos remotos permite actuar en entornos peligrosos y separa parcialmente presencia, identidad y ubicación. / *Controlling remote bodies allows us to act in dangerous environments and partially separates presence, identity and location.* | remote telepresence rover mimicking pose via latency arc |
| `F6` | **decisión compartida** (*shared decision*) | `[TIER-C]` | Sistemas pueden organizar evidencia y alternativas sin resolver por sí mismos conflictos de valores o legitimidad. / *Systems can organize evidence and alternatives without resolving conflicts of values or legitimacy by themselves.* | weighted multi-stakeholder balance beam |
| `F7` | **memoria digital** (*digital memory*) | `[TIER-C]` | Registros persistentes pueden ampliar continuidad personal y también dificultar olvido, consentimiento y reinterpretación del pasado. / *Persistent records can extend personal continuity and also make forgetting, consenting, and reinterpreting the past difficult.* | immutable ledger ring vs fade-erase gate |

##### `Z06` `sustainable-civilization` (`vid=3006`) — Civilización sostenible (*Sustainable civilization*) `[TIER-D]`

- **Catalog Metadata:** `scene=6` · `[0.316, 0.368]` · `rgb=[109, 243, 164]` · `sound=(sine/80.5Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Energía limpia + ciclos materiales + ecosistemas + justicia + resiliencia → estabilidad prolongada` / *`Clean energy + material cycles + ecosystems + justice + resilience → long-term stability`* · Evidence: `Escenario posible · bienestar dentro de límites planetarios` (*Possible scenario · well-being within planetary limits*)
- **Spoken Macro Narrative (ES / EN):** La sostenibilidad no significa detener todo cambio, sino mantener el bienestar sin destruir los sistemas que lo hacen posible. (*EN:* Sustainability does not mean stopping all change, but rather maintaining well-being without destroying the systems that make it possible.)
- **Current Diagnosis & Target 3D Blueprint:** Accidental modulo collision `200 + (6 % 4) = 202` shows an O'Neill space cylinder + star! Narration describes Earth-based sustainable civilization: circular material loops, low-carbon power, restored forests/wetlands/oceans, heat/flood resilient cities, low-impact agriculture, global satellite/sensor earth monitoring, waste elimination, and long-term common-pool planning. **Target (`vid=3006`):** Doughnut Biosphere & Circular Flows (vid=3006): vibrant Earth-ecosystem ring flanked by inner human-wellbeing floor and outer planetary-boundary ceiling.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **flujos circulares** (*circular flows*) | `[TIER-D]` | Diseñar para duración, reparación, reutilización y reciclaje reduce extracción, aunque ningún ciclo material es perfectamente cerrado. / *Designing for durability, repair, reuse and recycling reduces extraction, although no material cycle is perfectly closed.* | closed circular material-loop torus arrows |
| `F1` | **energía baja en carbono** (*low carbon energy*) | `[TIER-D]` | Una base energética limpia limita calentamiento y contaminación cuando también se gestionan materiales, territorio y demanda. / *A clean energy base limits warming and pollution when materials, territory and demand are also managed.* | wind/solar decarbonized grid shell |
| `F2` | **ecosistemas restaurados** (*restored ecosystems*) | `[TIER-D]` | Bosques, humedales, suelos y océanos pueden recuperar funciones si disminuyen las presiones que causaron su degradación. / *Forests, wetlands, soils and oceans can recover functions if the pressures that caused their degradation are reduced.* | blooming green forest-kelp-wetland biosphere wedge |
| `F3` | **ciudades resilientes** (*resilient cities*) | `[TIER-D]` | Vivienda, transporte, agua, sombra y servicios distribuidos ayudan a resistir calor, inundaciones y fallos de suministro. / *Housing, transportation, water, shade and distributed services help resist heat, flooding and power failures.* | terraced shaded resilient city grid with water retention channels |
| `F4` | **alimentación sostenible** (*sustainable food*) | `[TIER-D]` | Dietas, agricultura, almacenamiento y reducción de pérdidas pueden sostener nutrición con menor impacto ecológico. / *Diets, agriculture, storage and loss reduction can sustain nutrition with less ecological impact.* | layered agroforestry nutrient-loop plot |
| `F5` | **monitorización global** (*global monitoring*) | `[TIER-D]` | Sensores y ciencia abierta detectan cambios temprano, pero observar un problema no garantiza actuar sobre él. / *Sensors and open science detect changes early, but observing a problem does not guarantee acting on it.* | orbital EO monitoring satellite constellation scanning globe |
| `F6` | **reducción de residuos** (*waste reduction*) | `[TIER-C]` | Evitar productos innecesarios y sustancias persistentes suele ser más eficaz que gestionar residuos después de crearlos. / *Avoiding unnecessary products and persistent substances is often more effective than managing waste after it is created.* | zero-effluent streamlined flow |
| `F7` | **planificación a largo plazo** (*long term planning*) | `[TIER-C]` | Instituciones capaces de proteger bienes comunes y generaciones futuras deben equilibrar continuidad, democracia y adaptación. / *Institutions capable of protecting common goods and future generations must balance continuity, democracy and adaptation.* | multi-generation stewardship dial |

##### `Z07` `technological-inequality` (`vid=3007`) — Desigualdad tecnológica (*Technological inequality*) `[TIER-C]`

- **Catalog Metadata:** `scene=7` · `[0.368, 0.421]` · `rgb=[255, 184, 103]` · `sound=(triangle/85.25Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Acceso + propiedad + datos + infraestructura + poder → beneficios y riesgos desiguales` / *`Access + ownership + data + infrastructure + power → unequal benefits and risks`* · Evidence: `Capacidades futuras distribuidas de forma desigual` (*Future capabilities unevenly distributed*)
- **Spoken Macro Narrative (ES / EN):** El futuro es una cuestión política y social además de un problema de ingeniería. (*EN:* The future is a political and social question as well as an engineering problem.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps via 200 + (7 % 4) = 203 (`fut=3` central star + 3 Dyson rings). Dyson rings carry zero metaphor for unequal access, platform monopolies, digital exclusion, algorithmic surveillance bias, heritable biological enhancement divides, resource/compute concentration, or repair autonomy. **Target (`vid=3007`):** Asymmetric Access & Gatekeeper Prism (vid=3007): glowing resource torus skewed toward an inner golden enclave separated by translucent barrier gates from fragmented outer nodes.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **acceso desigual** (*unequal access*) | `[TIER-C]` | Precio, geografía, idioma, discapacidad y educación determinan quién puede usar nuevas capacidades. / *Price, geography, language, disability and education determine who can use new capabilities.* | steep accessibility elevation cliff |
| `F1` | **propiedad concentrada** (*concentrated ownership*) | `[TIER-C]` | Controlar plataformas, modelos, patentes o infraestructura permite decidir condiciones para millones de personas. / *Controlling platforms, models, patents or infrastructure makes it possible to decide conditions for millions of people.* | centralized star-hub controlling satellite spokes |
| `F2` | **exclusión digital** (*digital exclusion*) | `[TIER-C]` | Servicios esenciales pueden volverse inaccesibles para quienes carecen de dispositivos, conexión o identificación compatible. / *Essential services may become inaccessible to those without compatible devices, connections or identification.* | darkened disconnected peripheral nodes |
| `F3` | **control algorítmico** (*algorithmic control*) | `[TIER-C]` | Clasificación y vigilancia automatizadas pueden limitar oportunidades sin explicaciones comprensibles ni vías de apelación. / *Automated classification and surveillance can limit opportunities without understandable explanations or avenues for appeal.* | opaque black-box scanner sorting beams with locked appeal loop |
| `F4` | **poblaciones mejoradas** (*improved populations*) | `[TIER-D]` | Mejoras médicas o cognitivas desiguales podrían convertir diferencias económicas en diferencias corporales heredables. / *Unequal medical or cognitive improvements could convert economic differences into heritable bodily differences.* | bifurcated glowing enhanced vs baseline cellular double-helices |
| `F5` | **monopolios de recursos** (*resource monopolies*) | `[TIER-C]` | Minerales, energía, órbitas, datos y capacidad de cómputo pueden concentrarse en pocos actores. / *Minerals, energy, orbits, data and computing capacity can be concentrated in a few actors.* | orbital/compute/mineral rings clamped by few gate nodes |
| `F6` | **riqueza automatizada** (*automated wealth*) | `[TIER-C]` | Cuando la producción necesita menos trabajo, la distribución depende todavía más de propiedad, impuestos e instituciones. / *When production requires less labor, distribution depends even more on property, taxes and institutions.* | automated wealth flywheel bypassing labor node |
| `F7` | **dependencia tecnológica** (*technological dependence*) | `[TIER-C]` | Comunidades pueden perder autonomía cuando herramientas esenciales no pueden repararse, auditarse ni sustituirse localmente. / *Communities can lose autonomy when essential tools cannot be repaired, audited or replaced locally.* | black-box sealed module vs open modular open-repair core |

##### `Z08` `global-risks` (`vid=3008`) — Riesgos globales (*Global risks*) `[TIER-D]`

- **Catalog Metadata:** `scene=8` · `[0.421, 0.474]` · `rgb=[255, 102, 103]` · `sound=(sine/90Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `tierra`
- **Metric & Evidence (ES / EN):** `Peligro × exposición × vulnerabilidad × interconexión → riesgo sistémico` / *`Hazard × exposure × vulnerability × interconnection → systemic risk`* · Evidence: `Amenazas capaces de afectar a la civilización completa` (*Threats capable of affecting entire civilization*)
- **Spoken Macro Narrative (ES / EN):** Cuanto más interconectada se vuelve una civilización, más puede compartir tanto sus soluciones como sus fallos. (*EN:* The more interconnected a civilization becomes, the more it can share both its solutions and its failures.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps via 200 + (8 % 4) = 200 (`fut=0` 5 multicolored scenario ribbons). Narration details concrete existential and catastrophic physical risks: climate destabilization cascades, nuclear detonations & winter plume, engineered pandemic aerosols, unaligned AI breakout, trophic biosphere collapse, interconnected power-grid blackouts, Earth-crossing asteroid impact/deflection, and non-linear cascading failures. **Target (`vid=3008`):** Coupled Systemic Threat Globe (vid=3008): interconnected wireframe Earth wrapped in 8 vulnerable infrastructure/biosphere strata.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **desestabilización climática** (*climate destabilization*) | `[TIER-D]` | Calentamiento, extremos y cambios ecológicos pueden interactuar con alimentos, migración, salud, infraestructura y conflicto. / *Warming, extremes and ecological changes can interact with food, migration, health, infrastructure and conflict.* | reddening thermal anomaly & storm vortices |
| `F1` | **conflicto nuclear** (*nuclear conflict*) | `[TIER-D]` | Armas nucleares combinan destrucción inmediata, contaminación, escalada y posibles efectos climáticos mundiales. / *Nuclear weapons combine immediate destruction, contamination, escalation and possible global climate effects.* | ballistic arc flashes + dark stratospheric soot cloud dimming globe |
| `F2` | **pandemias diseñadas** (*designed pandemics*) | `[TIER-D]` | La biotecnología puede mejorar defensa sanitaria y también facilitar patógenos más peligrosos o accidentes de laboratorio. / *Biotechnology can improve health defense and also facilitate more dangerous pathogens or laboratory accidents.* | self-replicating viral capsid particle outbreak cluster |
| `F3` | **inteligencia no controlada** (*uncontrolled intelligence*) | `[TIER-D]` | Sistemas capaces de actuar ampliamente podrían perseguir objetivos incompatibles con la seguridad humana o escapar de supervisión efectiva. / *Systems capable of acting broadly could pursue objectives incompatible with human safety or escape effective supervision.* | runaway expanding compute mesh snapping containment ring |
| `F4` | **colapso ecológico** (*ecological collapse*) | `[TIER-D]` | La pérdida de biodiversidad y funciones ecosistémicas puede reducir alimentos, agua, estabilidad climática y capacidad de recuperación. / *The loss of biodiversity and ecosystem functions can reduce food, water, climate stability and resilience.* | shattering emerald trophic food-web lattice |
| `F5` | **fallo de infraestructura** (*infrastructure failure*) | `[TIER-D]` | Electricidad, comunicaciones, finanzas y logística forman redes donde una interrupción puede desencadenar muchas otras. / *Electricity, communications, finance and logistics form networks where one interruption can trigger many others.* | cascading blackout extinguishing connected city grid arcs |
| `F6` | **impactos de asteroides** (*asteroid impacts*) `[kw:tierra]` | `[TIER-D]` | Objetos cercanos a la Tierra son poco frecuentes pero físicamente detectables y, con suficiente anticipación, potencialmente desviables. / *Near-Earth objects are rare but physically detectable and, with enough notice, potentially deflectable.* | glowing NEO asteroid trajectory intercepted cleanly by kinetic-impactor arc |
| `F7` | **crisis en cascada** (*cascading crisis*) | `[TIER-D]` | Dos amenazas moderadas pueden combinarse de forma no lineal cuando debilitan simultáneamente instituciones y suministros. / *Two moderate threats can combine in a non-linear way when they simultaneously weaken institutions and supplies.* | coupled dual-shock nonlinear resonance quake |

##### `Z09` `planetary-governance` (`vid=3009`) — Gobernanza planetaria (*Planetary governance*) `[TIER-C]`

- **Catalog Metadata:** `scene=9` · `[0.474, 0.526]` · `rgb=[174, 180, 255]` · `sound=(triangle/94.75Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Cooperación + legitimidad + conocimiento + cumplimiento → acción a escala planetaria` / *`Cooperation + legitimacy + knowledge + compliance → action on a planetary scale`* · Evidence: `Coordinación sin destino institucional único` (*Coordination without a single institutional destination*)
- **Spoken Macro Narrative (ES / EN):** Coordinar problemas globales no implica que un único gobierno mundial sea inevitable ni necesariamente deseable. (*EN:* Coordinating global problems does not imply that a single world government is inevitable or necessarily desirable.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps via 200 + (9 % 4) = 201 (`fut=1` star + Dyson rings). Depicting a stellar Dyson megastructure during Earth planetary governance (treaties, distributed science, risk monitoring, standards, arms control, commons governance, unborn future generations) is a generic modulo mismatch. **Target (`vid=3009`):** Polycentric Planetary Governance Mesh (vid=3009): multi-node decentered Earth coordination ring without single central ruler.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **tratados** (*treaties*) | `[TIER-C]` | Los acuerdos internacionales convierten intereses compartidos en compromisos que dependen de verificación, incentivos y confianza. / *International agreements turn shared interests into commitments that depend on verification, incentives and trust.* | interlocking treaty verification bridges |
| `F1` | **cooperación científica** (*scientific cooperation*) | `[TIER-D]` | Datos, observatorios y laboratorios distribuidos permiten estudiar problemas que ningún país puede abarcar solo. / *Data, observatories and distributed laboratories allow us to study problems that no country can address alone.* | shared global telescope/lab data streams converging |
| `F2` | **regulación tecnológica** (*technological regulation*) | `[TIER-C]` | Normas sobre seguridad, uso y responsabilidad intentan reducir daños sin congelar toda innovación. / *Standards on safety, use and liability attempt to reduce harm without freezing all innovation.* | adaptive safety-threshold guardrails orbiting tech cores |
| `F3` | **vigilancia de riesgos** (*risk surveillance*) | `[TIER-D]` | Sistemas globales pueden detectar brotes, impactos, emisiones y fallos antes de que se conviertan en crisis. / *Global systems can detect outbreaks, impacts, emissions and failures before they become crises.* | early-warning global sensor sweep picking up anomaly sparks |
| `F4` | **estándares comunes** (*common standards*) | `[TIER-C]` | Protocolos compartidos permiten interoperabilidad y también pueden favorecer a quienes dominan su diseño. / *Shared protocols allow interoperability and can also favor those who master their design.* | interlocking hexagonal protocol standard tiles |
| `F5` | **prevención de conflictos** (*conflict prevention*) | `[TIER-C]` | Diplomacia, comunicación y control de armamentos reducen probabilidades sin eliminar competencia o error. / *Diplomacy, communication and arms control reduce probabilities without eliminating competition or error.* | de-escalation hotlines cooling rival vectors |
| `F6` | **gobernanza de recursos** (*resource governance*) | `[TIER-D]` | Océanos, atmósfera, polos, órbitas y quizá recursos espaciales requieren reglas sobre acceso, daño y beneficio. / *Oceans, atmosphere, poles, orbits, and perhaps space resources require rules about access, harm, and benefit.* | ocean/atmosphere/polar/orbital commons sectors |
| `F7` | **generaciones futuras** (*future generations*) | `[TIER-C]` | Representar a personas aún no nacidas obliga a considerar efectos que exceden ciclos políticos y vidas individuales. / *Representing people not yet born forces us to consider effects that exceed political cycles and individual lives.* | translucent ghost rings representing future generations casting votes |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP20-Z05-09)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP20-Z05-09` (Human-AI Symbiosis, Doughnut Sustainability, Tech Divide, X-Risks & Governance) for HyperCosmosVR Odyssey Act V (`Exp 20: 20_futures`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z05`->`3005` (`F0..F7`), `Z06`->`3006` (`F0..F7`), `Z07`->`3007` (`F0..F7`), `Z08`->`3008` (`F0..F7`), `Z09`->`3009` (`F0..F7`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3812`, `3813`, `3814`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP20-Z05-09`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Specialize `select_compute_mode` (`crates/cosmos_render/src/universe_scene.rs`) + `shaders/compute_universe.wgsl` (`submode`/mode split) so early zones do not display anachronistic `mode=14` optical-bus particles.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z05: human-machine-collaboration / Human-machine collaboration, Z06: sustainable-civilization / Sustainable civilization, Z07: technological-inequality / Technological inequality, Z08: global-risks / Global risks, Z09: planetary-governance / Planetary governance.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp20-z10-14"></a>
### PKG-ACT5-EXP20-Z10-14: Orbital Industry, Offworld Habitats, Multiplanetary Logistics, Terraforming & Postbiology

- **Phase:** Phase 1/2 | **Priority:** `P0` | **Scope:** Exp `20` (`20_futures`) `Z10..Z14` (`5` zones, `40` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L70-L74`: `{0|u16::MAX -> 200, 16 -> 202, _ -> 200 + (sc % 4)}` (`fut=1` & `3` collapse to identical Dyson core); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L272-L338` (`fut=0` 5-branch arcs, `fut=2` O'Neill+Dyson, `else` Dyson mirrors; `feat` only nudges mirror azimuth at `L318`/`L332`). Tier breakdown for this batch: **`0A / 0B / 1C / 4D` Zones**, **`0A / 1B / 10C / 29D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `2/5` zones and `2/40` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3010..=3014` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3815`, `3816` (allocated from `3720 + (20 - 11)*10 + k`; Heliocentric Transfer Streams & Magnetosphere Fields (`FUTURES_ARCS`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z10` | `space-industry` (*Industria espacial* / *space industry*) | `[TIER-D]` | `vid=202` | **`3010`** | `3815` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 semantic cue mask) | `0..7` (`0xFF` macro) |
| `Z11` | `offworld-settlements` (*Asentamientos extraplanetarios* / *Extraplanetary settlements*) | `[TIER-D]` | `vid=203` | **`3011`** | `3816` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x05` (0x04 semantic cue mask, 0x01 cutaway) | `0..7` (`0xFF` macro) |
| `Z12` | `multiplanetary-civilization` (*Civilización multiplanetaria* / *Multiplanetary civilization*) | `[TIER-D]` | `vid=200` | **`3012`** | `3815` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z13` | `planetary-engineering` (*Modificación planetaria* / *Planetary modification*) | `[TIER-D]` | `vid=201` | **`3013`** | `3816` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z14` | `postbiological-intelligence` (*Inteligencia postbiológica* / *Postbiological intelligence*) | `[TIER-C]` | `vid=202` | **`3014`** | `3815` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x02` (0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z10` `space-industry` (`vid=3010`) — Industria espacial (*space industry*) `[TIER-D]`

- **Catalog Metadata:** `scene=10` · `[0.526, 0.579]` · `rgb=[122, 201, 255]` · `sound=(sine/99.5Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `orbital`
- **Metric & Evidence (ES / EN):** `Transporte + energía + soporte vital + mantenimiento + mercado → operación espacial sostenida` / *`Transportation + energy + life support + maintenance + market → sustained space operation`* · Evidence: `Actividad económica más allá de la superficie terrestre` (*Economic activity beyond the earth's surface*)
- **Spoken Macro Narrative (ES / EN):** Expandirse al espacio exige sistemas continuos de energía, mantenimiento y cooperación, no solamente transporte. (*EN:* Expanding into space requires continuous systems of energy, maintenance and cooperation, not just transportation.)
- **Current Diagnosis & Target 3D Blueprint:** Accidental modulo collision `200 + (10 % 4) = 202` shows Z16's O'Neill Cylinder + Dyson star. Only F06 (orbital habitats) finds partial visual echo in the O'Neill cylinder; 7 of 8 concrete space-hardware systems (reusable rockets, zero-G manufacturing, satellite servicing, lunar ISRU mining, space observatories, asteroid prospecting, asteroid deflection) are completely missing. **Target (`vid=3010`):** Cis-Lunar Industrial Orbital Ecosystem (vid=3010 / dedicated space-industry): Earth + Moon barycenter orbit ring.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **lanzamiento reutilizable** (*reusable launch*) | `[TIER-D]` | Recuperar vehículos reduce parte del coste de acceso, aunque combustible, operaciones y seguridad siguen siendo decisivos. / *Recovering vehicles reduces part of the cost of access, although fuel, operations and security remain decisive.* | vertical booster descending on retro-propulsive exhaust plume |
| `F1` | **fabricación orbital** (*orbital manufacturing*) `[kw:orbital]` | `[TIER-D]` | La microgravedad y el vacío permiten procesos particulares cuando el valor obtenido supera transporte y complejidad. / *Microgravity and vacuum allow particular processes when the value obtained exceeds transport and complexity.* | rotating microgravity crystal/fiber manufacturing truss |
| `F2` | **servicio de satélites** (*satellite service*) | `[TIER-D]` | Reparar, reabastecer o retirar satélites puede prolongar infraestructura y reducir residuos orbitales. / *Repairing, refueling or removing satellites can prolong infrastructure and reduce orbital debris.* | robotic tug docking & refueling distressed comm-sat |
| `F3` | **recursos lunares** (*lunar resources*) | `[TIER-D]` | Hielo, regolito y energía solar podrían apoyar operaciones locales antes de convertirse en exportaciones útiles. / *Ice, regolith and solar energy could support local operations before becoming useful exports.* | sunlit polar lunar crater rim + regolith hopper/ice electrolysis plume |
| `F4` | **observatorios espaciales** (*space observatories*) | `[TIER-D]` | Instrumentos fuera de la atmósfera ven longitudes de onda y detalles bloqueados desde la superficie. / *Instruments outside the atmosphere see wavelengths and details blocked from the surface.* | unfolded segmented-mirror JWST-class golden space telescope pointing outward |
| `F5` | **prospección de asteroides** (*asteroid prospecting*) | `[TIER-D]` | Conocer composición y órbita precede a cualquier minería; la viabilidad depende de energía, robótica, mercado y derecho. / *Knowing composition and orbit precedes any mining; viability depends on energy, robotics, market and law.* | rubble-pile asteroid encircled by spectrometer lidar drone |
| `F6` | **hábitats orbitales** (*orbital habitats*) | `[TIER-B]` | Estructuras habitables necesitan protección, gravedad artificial o contramedidas, ciclos materiales y rescate. / *Habitable structures need protection, artificial gravity or countermeasures, material cycles and rescue.* | O'Neill cylinder habitat shell |
| `F7` | **defensa planetaria** (*planetary defense*) | `[TIER-D]` | Encuestas, modelos y misiones de desviación pueden transformar un peligro natural raro en un riesgo parcialmente gobernable. / *Surveys, models, and deflection missions can transform a rare natural hazard into a partially manageable risk.* | kinetic/gravity-tractor deflection beam nudging NEO miss-distance ellipse |

##### `Z11` `offworld-settlements` (`vid=3011`) — Asentamientos extraplanetarios (*Extraplanetary settlements*) `[TIER-D]`

- **Catalog Metadata:** `scene=11` · `[0.579, 0.632]` · `rgb=[255, 192, 128]` · `sound=(sawtooth/104.25Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `marte`
- **Metric & Evidence (ES / EN):** `Base → asentamiento → producción local → autonomía social y ecológica` / *`Base → settlement → local production → social and ecological autonomy`* · Evidence: `De visitas temporales a sociedades autosuficientes` (*From temporary visits to self-sufficient societies*)
- **Spoken Macro Narrative (ES / EN):** Visitar, mantener una base, construir un asentamiento y formar una sociedad autosuficiente son logros radicalmente distintos. (*EN:* Visiting, maintaining a base, building a settlement, and forming a self-sufficient society are radically different achievements.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via 200 + (11 % 4) = 203 (`fut=3` central star + Dyson mirror rings — NOT even `202`'s rotating O'Neill cylinder!). Fails completely to show Lunar bases, Mars domes, rotating artificial-gravity habitats, closed-loop life-support, regolith/water radiation shielding, or local ISRU printing. **Target (`vid=3011`):** Offworld Settlement Architectural Progression (vid=3011): four distinct settlement archetypes.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **bases lunares** (*lunar bases*) | `[TIER-D]` | La cercanía facilita comunicación y rescate, pero vacío, polvo, radiación y largas noches complican toda permanencia. / *Proximity facilitates communication and rescue, but emptiness, dust, radiation and long nights complicate any permanence.* | pressurized lunar lava-tube/regolith-covered dome under harsh terminator shadows |
| `F1` | **asentamientos marcianos** (*martian settlements*) `[kw:marte]` | `[TIER-D]` | Marte ofrece recursos y ciclos diarios reconocibles junto con atmósfera tenue, frío, radiación y enorme distancia logística. / *Mars offers recognizable resources and daily cycles along with thin atmosphere, cold, radiation, and enormous logistical distance.* | translucent Mars canyon settlement cluster glowing warm amber under dusty pink sky |
| `F2` | **hábitats giratorios** (*rotating habitats*) | `[TIER-D]` | Estructuras rotatorias podrían producir gravedad aparente y vivir independientes de superficies planetarias. / *Rotating structures could produce apparent gravity and live independently of planetary surfaces.* | counter-rotating Stanford-torus / O'Neill cylinder cross-section |
| `F3` | **ecosistemas cerrados** (*closed ecosystems*) | `[TIER-D]` | Aire, agua, nutrientes y residuos deben circular durante años con pérdidas mínimas y alta capacidad de reparación. / *Air, water, nutrients and waste must circulate for years with minimal losses and high repair capacity.* | glowing green hydroponic algae/water/O2 closed biogeochemical torus loop |
| `F4` | **protección radiológica** (*radiation protection*) | `[TIER-D]` | Regolito, agua, campos y refugios reducen exposición a partículas solares y rayos cósmicos. / *Regolith, water, fields and shelters reduce exposure to solar particles and cosmic rays.* | thick water-jacket & magnetic deflector dome blocking cosmic-ray particle streaks |
| `F5` | **gravedad artificial** (*artificial gravity*) | `[TIER-D]` | La rotación puede simular peso, pero exige estructuras grandes, control de movimiento y adaptación humana. / *Rotation can simulate weight, but requires large structures, motion control, and human adaptation.* | spinning centrifuge floor with radial gravity vectors |
| `F6` | **fabricación local** (*local manufacturing*) | `[TIER-D]` | Transformar materiales disponibles en piezas, alimento y combustible disminuye dependencia de suministros terrestres. / *Transforming available materials into parts, food and fuel reduces dependence on ground supplies.* | regolith 3D-printer sintering habitat bricks |
| `F7` | **separación cultural** (*cultural separation*) | `[TIER-C]` | Distancia, peligro y experiencias propias pueden crear identidades distintas mucho antes de la autosuficiencia económica. / *Distance, danger, and personal experiences can create distinct identities long before economic self-sufficiency.* | diverging Earth-vs-Mars social tether stretching outward |

##### `Z12` `multiplanetary-civilization` (`vid=3012`) — Civilización multiplanetaria (*Multiplanetary civilization*) `[TIER-D]`

- **Catalog Metadata:** `scene=12` · `[0.632, 0.684]` · `rgb=[156, 164, 255]` · `sound=(sine/109Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Mundos habitados + autonomía local + comunicación retardada → civilización distribuida` / *`Inhabited worlds + local autonomy + delayed communication → distributed civilization`* · Evidence: `Sociedades separadas por minutos u horas de luz` (*Societies separated by minutes or hours of light*)
- **Spoken Macro Narrative (ES / EN):** Una civilización multiplanetaria no sería una nación terrestre ampliada: las distancias crearían nuevas sociedades y formas de independencia. (*EN:* A multiplanetary civilization would not be an expanded terrestrial nation: distances would create new societies and forms of independence.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps via 200 + (12 % 4) = 200 (`fut=0` abstract 5 probability ribbons). Narration explores physical interplanetary distances measured in light-minutes/hours, independent economies, Hohmann orbital logistics, low-gravity biology, and local autonomy across inhabited worlds. **Target (`vid=3012`):** Inner Solar Multiplanetary Light-Delay Web (vid=3012): true-proportion heliocentric Earth-Moon-Mars-Ceres-Belt orbital rings.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **retrasos de comunicación** (*communication delays*) | `[TIER-D]` | La información viaja como máximo a la velocidad de la luz, impidiendo conversación instantánea entre mundos distantes. / *Information travels at the speed of light at most, preventing instantaneous conversation between distant worlds.* | expanding spherical light-speed wave pulses taking visible travel time between blue Earth and red Mars |
| `F1` | **economías independientes** (*independent economies*) | `[TIER-D]` | Transportar bienes entre planetas favorece producción local y comercio de objetos o información de alto valor. / *Transporting goods between planets favors local production and trade of high-value objects or information.* | localized orbital production halos around each world |
| `F2` | **culturas divergentes** (*divergent cultures*) | `[TIER-C]` | Entornos, riesgos y generaciones separadas producirían costumbres e identidades cada vez más distintas. / *Separate environments, risks and generations would produce increasingly different customs and identities.* | distinct color-palette cultural aura rings per world |
| `F3` | **gobernanza local** (*local governance*) | `[TIER-C]` | Decisiones urgentes no pueden esperar instrucciones de otro planeta y empujan hacia autoridad cercana. / *Urgent decisions cannot wait for instructions from another planet and push toward nearby authority.* | fast local Mars-orbit decision loop vs slow Earth ping |
| `F4` | **comercio interplanetario** (*interplanetary trade*) | `[TIER-D]` | Rutas lentas y ventanas orbitales convierten logística, seguros y planificación en procesos de años. / *Slow routes and orbital windows turn logistics, insurance and planning into years-long processes.* | curved Hohmann transfer trajectory arcs bridging launch windows |
| `F5` | **adaptación biológica** (*biological adaptation*) | `[TIER-D]` | Salud en otras gravedades podría requerir medicina, selección cultural o modificaciones biológicas controvertidas. / *Health under different gravities might require medicine, cultural selection, or controversial biological modifications.* | stretched low-G bone/cellular lattice comparison |
| `F6` | **autonomía política** (*political autonomy*) | `[TIER-C]` | La dependencia material puede disminuir mientras memoria histórica y tratados mantienen vínculos. / *Material dependence can decrease while historical memory and treaties maintain links.* | detaching tether turning solid treaty ring |
| `F7` | **herencia compartida** (*shared inheritance*) | `[TIER-C]` | Lenguas, ciencia y origen terrestre podrían unir poblaciones que ya no comparten exactamente la misma experiencia humana. / *Languages, science and terrestrial origin could unite populations that no longer share exactly the same human experience.* | common DNA/science glyph thread linking all worlds |

##### `Z13` `planetary-engineering` (`vid=3013`) — Modificación planetaria (*Planetary modification*) `[TIER-D]`

- **Catalog Metadata:** `scene=13` · `[0.684, 0.737]` · `rgb=[111, 235, 205]` · `sound=(triangle/113.75Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Energía + materiales + biología + siglos + gobernanza → ambiente planetario alterado` / *`Energy + materials + biology + centuries + governance → altered planetary environment`* · Evidence: `Intervención sobre ambientes completos · legitimidad incierta` (*Intervention on complete environments · uncertain legitimacy*)
- **Spoken Macro Narrative (ES / EN):** Ser técnicamente capaz de modificar un planeta no implica tener el derecho, el conocimiento suficiente ni la legitimidad para hacerlo. (*EN:* Being technically capable of modifying a planet does not imply having the right, sufficient knowledge or legitimacy to do so.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps via 200 + (13 % 4) = 201 (`fut=1` central star + Dyson rings). Concrete geoengineering/terraforming megastructures — adding thick planetary atmospheres, L1 orbital mirrors, artificial magnetic dipole shields, ecological seeding, paraterraforming domes, and planetary protection quarantine — have zero visual representation. **Target (`vid=3013`):** Planetary Modification & Terraforming Stages (vid=3013): arid cratered Mars-like world transforming progressively along parameter.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **ingeniería atmosférica** (*atmospheric engineering*) | `[TIER-D]` | Añadir, retirar o transformar gases exige cantidades enormes de materia y puede producir retroalimentaciones difíciles de revertir. / *Adding, removing or transforming gases requires enormous amounts of matter and can produce feedbacks that are difficult to reverse.* | thickening greenhouse vapor shroud swirling across craters |
| `F1` | **espejos orbitales** (*orbital mirrors*) | `[TIER-D]` | Reflectores podrían modificar iluminación o temperatura local, con riesgos de control, fallo y efectos transfronterizos. / *Reflectors could modify local lighting or temperature, with risks of control, failure and cross-border effects.* | giant gossamer Mylar mirror array at L1 focusing golden sunlight onto polar ice cap |
| `F2` | **magnetosferas artificiales** (*artificial magnetospheres*) | `[TIER-D]` | Campos generados en el espacio se han propuesto para desviar partículas, pero requerirían infraestructuras aún hipotéticas. / *Space-generated fields have been proposed to deflect particles, but would require still-hypothetical infrastructure.* | superconducting ring at Mars-Sun L1 casting protective blue magnetic dipole bowshock deflecting solar wind particles |
| `F3` | **siembra ecológica** (*ecological seeding*) | `[TIER-D]` | Introducir organismos puede construir ciclos vivos o destruir entornos autóctonos antes de comprenderlos. / *Introducing organisms could establish living cycles or destroy native environments before they are understood.* | cyanobacteria greening bands along wet basins |
| `F4` | **control climático** (*climate control*) | `[TIER-D]` | Regular un clima completo exige predecir sistemas caóticos y decidir quién define las condiciones deseadas. / *Regulating an entire climate requires predicting chaotic systems and deciding who defines the desired conditions.* | chaotic vortex instability vs feedback ring |
| `F5` | **terraformación** (*terraforming*) | `[TIER-D]` | Transformar un planeta para vida terrestre probablemente requeriría siglos o milenios y capacidades muy superiores a las actuales. / *Transforming a planet for terrestrial life would likely require centuries or millennia and capabilities far superior to today's.* | century-lapse dry-to-ocean-blue planet |
| `F6` | **paraterraformación** (*paraterraforming*) | `[TIER-D]` | Encerrar regiones habitables limita la escala y permite control local sin alterar todo el planeta. / *Enclosing habitable regions limits scale and allows local control without disrupting the entire planet.* | glassed paraterraforming crater dome |
| `F7` | **protección planetaria** (*planetary protection*) | `[TIER-C]` | Evitar contaminación biológica protege ciencia, ecosistemas posibles y decisiones que pertenecen a generaciones futuras. / *Avoiding biological contamination protects science, possible ecosystems, and decisions that belong to future generations.* | golden bio-containment quarantine shield halo |

##### `Z14` `postbiological-intelligence` (`vid=3014`) — Inteligencia postbiológica (*Postbiological intelligence*) `[TIER-C]`

- **Catalog Metadata:** `scene=14` · `[0.737, 0.789]` · `rgb=[223, 153, 255]` · `sound=(sine/118.5Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Cognición + soporte físico + energía + continuidad → posibles personas o culturas sintéticas` / *`Cognition + physical support + energy + continuity → possible synthetic people or cultures`* · Evidence: `Hipótesis · mentes no limitadas a cuerpos orgánicos` (*Hypothesis · minds not limited to organic bodies*)
- **Spoken Macro Narrative (ES / EN):** Las inteligencias postbiológicas son hipótesis, no resultados establecidos ni destinos inevitables. (*EN:* Postbiological intelligences are hypotheses, not established results or inevitable destinies.)
- **Current Diagnosis & Target 3D Blueprint:** Accidental modulo collision `200 + (14 % 4) = 202` shows Z16's O'Neill cylinder + Dyson star. Postbiological philosophy (substrate independence, digital personhood, distributed cognition, synthetic connectome emulation, deep-time hibernation) has no correspondence with an O'Neill space station. **Target (`vid=3014`):** Postbiological Mind & Substrate Matrix (vid=3014): luminous topological neural-connectome manifold floating inside crystalline compute shell.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **mentes sintéticas** (*synthetic minds*) | `[TIER-C]` | Sistemas construidos podrían desarrollar capacidades o experiencias propias, pero no sabemos qué arquitectura produciría conciencia. / *Built systems could develop their own capabilities or experiences, but we do not know what architecture would produce consciousness.* | emergent conscious resonance wave |
| `F1` | **personas digitales** (*digital people*) | `[TIER-C]` | Reconocer una entidad digital como persona dependería de evidencia sobre continuidad, experiencia, autonomía y relaciones sociales. / *Recognizing a digital entity as a person would depend on evidence about continuity, experience, autonomy, and social relationships.* | persistent identity core thread spanning substrate swaps |
| `F2` | **cognición distribuida** (*distributed cognition*) | `[TIER-D]` | Una mente podría extenderse entre muchos procesadores, lugares y cuerpos con problemas nuevos de identidad y coordinación. / *A mind could spread across many processors, places, and bodies with new problems of identity and coordination.* | multi-body optical fiber swarm sharing one unified thought wave |
| `F3` | **civilizaciones robóticas** (*robotic civilizations*) | `[TIER-D]` | Máquinas capaces de mantenimiento y reproducción industrial podrían vivir donde la biología terrestre no puede. / *Machines capable of industrial maintenance and reproduction could live where terrestrial biology cannot.* | self-maintaining industrial robotic foundry colony orbiting cold asteroid |
| `F4` | **emulación mental** (*mental emulation*) | `[TIER-D]` | Copiar con suficiente detalle un cerebro exigiría comprender qué estructura y dinámica son necesarias para preservar una mente. / *Copying a brain in sufficient detail would require understanding what structure and dynamics are necessary to preserve a mind.* | high-resolution biological synapse slice scanned layer-by-layer into photonic crystal |
| `F5` | **independencia de sustrato** (*substrate independence*) | `[TIER-C]` | La idea de que una mente puede existir en materiales distintos sigue siendo una hipótesis filosófica y científica abierta. / *The idea that a mind can exist in different materials remains an open philosophical and scientific hypothesis.* | carbon-neuron vs silicon-photonic dual-projection prism |
| `F6` | **computación prolongada** (*long-duration computing*) | `[TIER-D]` | Sistemas no biológicos podrían operar lentamente, hibernar o durar milenios si conservan energía, memoria y reparación. / *Nonbiological systems could operate slowly, hibernate, or last for millennia if they conserve energy, memory, and repair.* | slowed ultra-cold hibernating clock crystal ticking once per millennium |
| `F7` | **culturas de máquinas** (*machine cultures*) | `[TIER-C]` | Entidades sintéticas podrían desarrollar ritmos, valores y formas de comunicación difíciles de interpretar desde la experiencia humana. / *Synthetic entities could develop rhythms, values and forms of communication that are difficult to interpret from human experience.* | alien non-linear light/glyph chord |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP20-Z10-14)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP20-Z10-14` (Orbital Industry, Offworld Habitats, Multiplanetary Logistics, Terraforming & Postbiology) for HyperCosmosVR Odyssey Act V (`Exp 20: 20_futures`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z10`->`3010` (`F0..F7`), `Z11`->`3011` (`F0..F7`), `Z12`->`3012` (`F0..F7`), `Z13`->`3013` (`F0..F7`), `Z14`->`3014` (`F0..F7`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3815`, `3816`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP20-Z10-14`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Specialize `select_compute_mode` (`crates/cosmos_render/src/universe_scene.rs`) + `shaders/compute_universe.wgsl` (`submode`/mode split) so early zones do not display anachronistic `mode=14` optical-bus particles.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z10: space-industry / space industry, Z11: offworld-settlements / Extraplanetary settlements, Z12: multiplanetary-civilization / Multiplanetary civilization, Z13: planetary-engineering / Planetary modification, Z14: postbiological-intelligence / Postbiological intelligence.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp20-z15-18"></a>
### PKG-ACT5-EXP20-Z15-18: Interstellar Arks, Kardashev II-III Dyson/Matrioshka Zoom, Divergent Loom & Horizon

- **Phase:** Phase 1/2 | **Priority:** `P1` | **Scope:** Exp `20` (`20_futures`) `Z15..Z18` (`4` zones, `32` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L70-L74`: `{0|u16::MAX -> 200, 16 -> 202, _ -> 200 + (sc % 4)}` (`fut=1` & `3` collapse to identical Dyson core); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L272-L338` (`fut=0` 5-branch arcs, `fut=2` O'Neill+Dyson, `else` Dyson mirrors; `feat` only nudges mirror azimuth at `L318`/`L332`). Tier breakdown for this batch: **`0A / 1B / 2C / 1D` Zones**, **`0A / 2B / 18C / 12D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `3/4` zones and `3/32` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3015..=3018` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3817`, `3818`, `3819` (allocated from `3720 + (20 - 11)*10 + k`; Relativistic Light-Cone + Kardashev Thermal Swarm (`mode=14`/`mode=15` bridge)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z15` | `interstellar-civilization` (*Civilización interestelar* / *interstellar civilization*) | `[TIER-D]` | `vid=203` | **`3015`** | `3817` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 semantic cue mask) | `0..7` (`0xFF` macro) |
| `Z16` | `kardashev-scale` (*Escalas de Kardashev* / *Kardashev scales*) | `[TIER-B]` | `vid=202` | **`3016`** | `3818` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 semantic cue mask) | `0..7` (`0xFF` macro) |
| `Z17` | `divergent-futures` (*Futuros divergentes* / *Divergent futures*) | `[TIER-C]` | `vid=201` | **`3017`** | `3819` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..7` (`0xFF` macro) |
| `Z18` | `unknown-horizon` (*Horizonte desconocido* / *unknown horizon*) | `[TIER-C]` | `vid=202` | **`3018`** | `3817` | `0` default (`1` cutaway interior, `2` `cs_field_grid`/flux overlay) | `0x06` (0x04 semantic cue mask, 0x02 dual-hero/lens) | `0..7` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z15` `interstellar-civilization` (`vid=3015`) — Civilización interestelar (*interstellar civilization*) `[TIER-D]`

- **Catalog Metadata:** `scene=15` · `[0.789, 0.842]` · `rgb=[146, 205, 255]` · `sound=(triangle/123.25Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `fusion`
- **Metric & Evidence (ES / EN):** `Propulsión + autonomía + tiempo + reproducción social → presencia entre estrellas` / *`Propulsion + autonomy + time + social reproduction → presence among stars`* · Evidence: `Expansión separada por años, generaciones y milenios` (*Expansion separated by years, generations and millennia*)
- **Spoken Macro Narrative (ES / EN):** Las distancias interestelares transforman expansión, comunicación e identidad en procesos de generaciones o milenios. (*EN:* Interstellar distances transform expansion, communication and identity in processes of generations or millennia.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps via 200 + (15 % 4) = 203 (`fut=3` central star + 3 Dyson rings). Fails completely to depict concrete interstellar propulsion & craft: generation ships, suspended-animation pods, self-replicating von Neumann probes, Starshot laser light-sails, fusion torchships, seedships, and multi-decade light-beams. **Target (`vid=3015`):** Interstellar Propulsion & Transit Fleet (vid=3015): two distant star anchors separated across void.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **naves generacionales** (*generation ships*) | `[TIER-D]` | Comunidades completas podrían vivir y cambiar durante viajes más largos que una vida humana. / *Entire communities could live and change during journeys longer than a human lifetime.* | massive cylindrical/toroidal generation world-ship creeping along slow trajectory |
| `F1` | **animación suspendida** (*suspended animation*) | `[TIER-D]` | Reducir metabolismo durante décadas sigue siendo una posibilidad especulativa con grandes obstáculos médicos. / *Reducing metabolism for decades remains a speculative possibility with major medical obstacles.* | icy cryo-stasis pod cluster along hull |
| `F2` | **sondas autorreplicantes** (*self-replicating probes*) | `[TIER-D]` | Máquinas que usan recursos locales podrían explorar muchas estrellas y multiplicar errores, objetivos o contaminación. / *Machines using local resources could explore many stars and multiply errors, targets or contamination.* | recursive probe arriving at asteroid belt and budding 2->4->8 daughter probes |
| `F3` | **propulsión láser** (*laser propulsion*) | `[TIER-D]` | Velas muy ligeras impulsadas por haces podrían alcanzar altas velocidades sin transportar todo su combustible. / *Very light beam-driven sails could reach high speeds without carrying all their fuel.* | razor-thin reflective light-sail surfing concentrated laser beam from home system |
| `F4` | **propulsión de fusión** (*fusion propulsion*) `[kw:fusion]` | `[TIER-D]` | Reacciones de fusión podrían impulsar naves rápidas si se dominan combustible, confinamiento, masa y calor. / *Fusion reactions could power fast ships if fuel, confinement, mass and heat are mastered.* | magnetic-nozzle fusion torchship firing intense blue-violet plasma plume |
| `F5` | **sistemas semilla** (*seed systems*) | `[TIER-D]` | Paquetes autónomos podrían construir infraestructura o incluso organismos al llegar, planteando control y legitimidad extremos. / *Autonomous seed packages could build infrastructure or even organisms upon arrival, raising extreme questions of control and legitimacy.* | autonomous micro-embryo seed lander unfolding surface dome |
| `F6` | **comunicación interestelar** (*interstellar communication*) | `[TIER-D]` | Cada mensaje tardaría años o siglos, convirtiendo diálogo en correspondencia entre generaciones. / *Each message would take years or centuries, turning dialogue into correspondence between generations.* | fading pulse packet mid-transit in void |
| `F7` | **colonias aisladas** (*isolated colonies*) | `[TIER-C]` | La separación prolongada produciría evolución cultural, política y quizá biológica independiente. / *Prolonged separation would produce independent cultural, political, and perhaps biological evolution.* | drifting distant colony ring decoupling hue |

##### `Z16` `kardashev-scale` (`vid=3016`) — Escalas de Kardashev (*Kardashev scales*) `[TIER-B]`

- **Catalog Metadata:** `scene=16` · `[0.842, 0.895]` · `rgb=[255, 218, 123]` · `sound=(sine/128Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `orbital`, `dyson`
- **Metric & Evidence (ES / EN):** `Potencia controlada aproximada → escala planetaria, estelar o galáctica` / *`Approximate controlled power → planetary, stellar or galactic scale`* · Evidence: `Herramienta conceptual basada en energía, no en sabiduría` (*Conceptual tool based on energy, not wisdom*)
- **Spoken Macro Narrative (ES / EN):** La escala de Kardashev mide aproximadamente uso de energía; no mide bienestar, moralidad, sabiduría ni inevitabilidad histórica. (*EN:* The Kardashev scale approximately measures energy use; it does not measure well-being, morality, wisdom or historical inevitability.)
- **Current Diagnosis & Target 3D Blueprint:** Bespoke z==16 -> vid=3016 (`fut=2`: Rotating O'Neill cylinder + central star + 3 Dyson collector rings) directly depicts Type II stellar capture and Dyson swarms (F01, F02). However, `doc_feat` ONLY shifts mirror-ring cosine phase (`*0.4`), lacking Type I planetary budget (F00), Matrioshka computer shells (F03), Type III galactic scale (F04), or IR waste-heat spectral shift (F06). **Target (`vid=3016`):** —

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **energía planetaria** (*planetary energy*) | `[TIER-D]` | Una civilización planetaria gestionaría una fracción grande de la energía disponible en su mundo sin necesariamente hacerlo de forma justa. / *A planetary civilization would manage a large fraction of the energy available on its world without necessarily doing so fairly.* | zoom into glowing Type I planet wrapped in solar-grid aura |
| `F1` | **energía estelar** (*stellar energy*) | `[TIER-B]` | Capturar parte sustancial de la luz de una estrella requeriría una población inmensa de colectores y sistemas orbitales. / *Capturing a substantial part of a star's light would require an immense population of collectors and orbital systems.* | highlight bright central G-star + inner capture sphere |
| `F2` | **enjambres de Dyson** (*Dyson swarms*) `[kw:dyson]` | `[TIER-B]` | Millones de estructuras independientes son físicamente más plausibles que una esfera rígida alrededor de una estrella. / *Millions of independent structures are physically more plausible than a rigid sphere around a star.* | fan out thousands of swarming free-flying Dyson statites contrasted against cracked impossible rigid shell |
| `F3` | **computación distribuida** (*distributed computing*) | `[TIER-D]` | Energía estelar podría alimentar grandes redes de cálculo limitadas por temperatura, comunicación y materiales. / *Stellar energy could power large computing networks limited by temperature, communication and materials.* | nest concentric dark Matrioshka thermal-computing shells (hot inside -> warm amber outer) |
| `F4` | **infraestructura galáctica** (*galactic infrastructure*) | `[TIER-D]` | Extender actividad por una galaxia exigiría millones de años incluso con expansión rápida a escala humana. / *Spreading activity across a galaxy would require millions of years even with rapid human-scale expansion.* | zoom out to galactic spiral disk seeded with dim Type III node webs |
| `F5` | **eficiencia energética** (*energy efficiency*) | `[TIER-C]` | Una civilización avanzada podría hacer más con menos energía y ser menos visible que otra más derrochadora. / *An advanced civilization could do more with less energy and be less visible than a more wasteful one.* | stealthy low-luminosity compact efficient halo |
| `F6` | **firmas observables** (*observable signatures*) | `[TIER-D]` | Calor residual, estructuras orbitales o cambios estelares podrían revelar tecnología sin explicar su cultura. / *Residual heat, orbital structures or stellar changes could reveal technology without explaining its culture.* | mid-IR red glow halo + uneven transit dip curve |
| `F7` | **límites físicos** (*physical limits*) `[kw:orbital]` | `[TIER-C]` | Termodinámica, velocidad de la luz, estabilidad orbital y recursos acotan cualquier ingeniería cósmica. / *Thermodynamics, speed of light, orbital stability and resources limit any cosmic engineering.* | thermodynamic Carnot waste-heat cone & c-sphere limit |

##### `Z17` `divergent-futures` (`vid=3017`) — Futuros divergentes (*Divergent futures*) `[TIER-C]`

- **Catalog Metadata:** `scene=17` · `[0.895, 0.947]` · `rgb=[255, 150, 190]` · `sound=(sawtooth/132.75Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Capacidad tecnológica × valores × instituciones × accidentes → trayectorias divergentes` / *`Technological capacity × values × institutions × accidents → divergent trajectories`* · Evidence: `Múltiples resultados · ninguna rama garantizada` (*Multiple outcomes · no branch guaranteed*)
- **Spoken Macro Narrative (ES / EN):** Cada nueva capacidad abre varias ramas; el poder tecnológico por sí solo no decide cuál se vuelve realidad. (*EN:* Each new ability opens several branches; technological power alone does not decide which becomes reality.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via 200 + (17 % 4) = 201 (`fut=1` star + Dyson rings). Ironically, Z17 (`Divergent futures`) gets stationary Dyson rings instead of Z00's bifurcating scenario arcs! None of the 8 distinct future trajectories (abundance, authoritarian surveillance, decentralized autonomy, eco-recovery, stagnation, posthuman, collapse/rebuild, cooperative intelligence) are distinguished. **Target (`vid=3017`):** 8-Branch Divergence Loom (vid=3017): reuse/extend Z00 branching SDF topology into 8 radially fanning trajectory tubes morphing morphing live per `doc_feat`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **abundancia sostenible** (*sustainable abundance*) | `[TIER-C]` | Energía limpia, automatización y distribución justa podrían elevar bienestar sin exceder límites ecológicos. / *Clean energy, automation and fair distribution could increase well-being without exceeding ecological limits.* | bright emerald solar-abundance garden arc |
| `F1` | **control autoritario** (*authoritarian control*) | `[TIER-C]` | Vigilancia, predicción y dependencia digital podrían concentrar poder con apariencia de eficiencia. / *Surveillance, prediction and digital dependence could concentrate power with the appearance of efficiency.* | panopticon crystalline surveillance eye locking nodes |
| `F2` | **civilización descentralizada** (*decentralized civilization*) | `[TIER-C]` | Producción local, redes abiertas y autonomía comunitaria podrían repartir resiliencia y coordinación. / *Local production, open networks and community autonomy could provide resilience and coordination.* | peer-to-peer distributed polycentric mesh |
| `F3` | **recuperación ecológica** (*ecological recovery*) | `[TIER-C]` | Sociedades podrían restaurar ecosistemas y redefinir prosperidad alrededor de continuidad biológica. / *Societies could restore ecosystems and redefine prosperity around biological continuity.* | lush biological rewilded Earth sphere |
| `F4` | **estancamiento tecnológico** (*technological stagnation*) | `[TIER-C]` | Conflicto, límites económicos o elección cultural pueden frenar innovación sin terminar la historia. / *Conflict, economic limits or cultural choice can stop innovation without finishing the story.* | frozen rusted broken gear orbit |
| `F5` | **transformación posthumana** (*posthuman transformation*) | `[TIER-C]` | Biología, máquinas y cultura podrían producir descendientes que ya no encajen en categorías humanas actuales. / *Biology, machines and culture could produce descendants who no longer fit current human categories.* | iridescent hybrid biological-photonic morphing silhouette |
| `F6` | **colapso y reconstrucción** (*collapse and reconstruction*) | `[TIER-C]` | Sistemas complejos pueden perder capacidad y luego reorganizarse con memoria parcial del pasado. / *Complex systems can lose capacity and then reorganize with partial memory of the past.* | shattered ring slowly re-knitting golden repair kintsugi seams |
| `F7` | **inteligencia cooperativa** (*cooperative intelligence*) | `[TIER-C]` | Personas, instituciones y máquinas podrían formar sistemas de decisión más capaces si preservan pluralidad, revisión y control. / *People, institutions and machines could form more capable decision systems if they preserve plurality, review and control.* | harmonic braided human+institution+AI torus |

##### `Z18` `unknown-horizon` (`vid=3018`) — Horizonte desconocido (*unknown horizon*) `[TIER-C]`

- **Catalog Metadata:** `scene=18` · `[0.947, 1.000]` · `rgb=[238, 244, 255]` · `sound=(sine/137.5Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `horizonte`
- **Metric & Evidence (ES / EN):** `Cuanto mayor la distancia temporal → menor predicción y mayor espacio de posibilidades` / *`The greater the temporal distance → the less prediction and the greater the space of possibilities`* · Evidence: `Frontera de la imaginación informada` (*Frontier of informed imagination*)
- **Spoken Macro Narrative (ES / EN):** Cuanto más lejos miramos, menos predecimos acontecimientos y más cartografiamos posibilidades. (*EN:* The further we look, the less we predict events and the more we map possibilities.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps via 200 + (18 % 4) = 202 (`fut=2` O'Neill cylinder + Dyson star). Closing unknown-horizon zone hands off cleanly to Exp 21 ('ask if we are alone / alien life'); showing a Type II O'Neill space station misdirects away from unrecognizable alien tech, unknown physics, and deep cosmic mystery. **Target (`vid=3018`):** Unrecognizable Horizon & Deep Cosmos Bridge (vid=3018): dissolve rigid megastructures into iridescent misty deep-space lens.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **física desconocida** (*unknown physics*) | `[TIER-C]` | Nuevas leyes o fenómenos podrían cambiar límites que hoy parecen fundamentales. / *New laws or phenomena could change limits that seem fundamental today.* | shimmering topological phase fracture hinting new physics |
| `F1` | **formas nuevas de inteligencia** (*new forms of intelligence*) | `[TIER-C]` | La cognición futura podría existir en cuerpos, escalas y ritmos sin equivalentes actuales. / *Future cognition could exist in bodies, scales, and rhythms without current equivalents.* | non-human distributed fluid-crystal intelligence flicker |
| `F2` | **vida alienígena** (*alien life*) | `[TIER-D]` | Descubrir otra biosfera modificaría nuestra comprensión de evolución, singularidad y responsabilidad cósmica. / *Discovering another biosphere would modify our understanding of evolution, singularity, and cosmic responsibility.* | distant pale exoplanet spark teasing astrobiology (Exp 21 transition) |
| `F3` | **identidad transformada** (*transformed identity*) | `[TIER-C]` | Cambios biológicos, digitales y sociales pueden volver insuficientes las palabras actuales para persona, especie o sociedad. / *Biological, digital and social changes can make current words for person, species or society insufficient.* | morphing identity silhouette |
| `F4` | **restricciones cósmicas** (*cosmic restrictions*) | `[TIER-C]` | Energía, entropía, expansión y velocidad de la luz continuarán condicionando cualquier civilización conocida. / *Energy, entropy, expansion and speed of light will continue to condition any known civilization.* | thermodynamic entropy arrow + light-cone cage |
| `F5` | **civilizaciones irreconocibles** (*unrecognizable civilizations*) | `[TIER-C]` | Tecnología muy avanzada podría no parecer una ciudad, una nave ni una señal deliberada. / *Very advanced technology might not look like a city, a ship, or a deliberate signal.* | cryptic geometric artifact indistinguishable from starlight/rock |
| `F6` | **vocabulario ausente** (*absent vocabulary*) | `[TIER-C]` | Algunas posibilidades no pueden describirse bien desde conceptos creados para el presente. / *Some possibilities cannot be described well from concepts created for the present.* | fading word-glyph clouds |
| `F7` | **más allá** (*beyond*) | `[TIER-C]` | Si la vida y la inteligencia pueden expandirse por el cosmos, debemos preguntar si estamos solos y cómo reconoceríamos una civilización verdaderamente alienígena. / *If life and intelligence can spread across the cosmos, we must ask if we are alone and how we would recognize a truly alien civilization.* | wide starry field panning directly into Orion Spur Exp 21 |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP20-Z15-18)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP20-Z15-18` (Interstellar Arks, Kardashev II-III Dyson/Matrioshka Zoom, Divergent Loom & Horizon) for HyperCosmosVR Odyssey Act V (`Exp 20: 20_futures`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z15`->`3015` (`F0..F7`), `Z16`->`3016` (`F0..F7`), `Z17`->`3017` (`F0..F7`), `Z18`->`3018` (`F0..F7`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3817`, `3818`, `3819`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP20-Z15-18`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Specialize `select_compute_mode` (`crates/cosmos_render/src/universe_scene.rs`) + `shaders/compute_universe.wgsl` (`submode`/mode split) so early zones do not display anachronistic `mode=14` optical-bus particles.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z15: interstellar-civilization / interstellar civilization, Z16: kardashev-scale / Kardashev scales, Z17: divergent-futures / Divergent futures, Z18: unknown-horizon / unknown horizon.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

### 4.4 Experience 11: `11_cosmic_evidence` — Espectro de la Evidencia Cósmica: de las Huellas Observables a las Preguntas del Origen

- **Odyssey & Spatial Placement:** Act 5 (`4/8`, overall stop #21) · `HYPER_LOTUS_11_EVIDENCE` (`parent UNIVERSE_ROOT`, `u = [27.2, 28.6]`, `focus_scale_exp = 27.6`, `6.0e27 m`) ([`topology.rs#L204-L215, L653`](../../crates/cosmos_spatial/src/topology.rs#L204-L215))
- **Experience Totals:** `9` Zones (`0A / 1B / 5C / 3D`) · `50` Features (`0A / 1B / 22C / 27D`)
- **Current Shader & Compute:** `doc_stages.rs#L62`: `110 + (sc.unwrap_or(z) % 4)` (`scene` is `null` across all 9 zones -> modulo collapses to `z % 4`!) · `shaders/hero_doc_civilization_horizon.wgsl#L47-L104` (`mode 0` 5-galaxy redshift+Fraunhofer, `1` CMB sphere, `2` Einstein cross, `else` 4 tilted rings — `feat` only tilts rings at `L98`) · `mode=13` (`universe_scene.rs#L310`, `compute_universe.wgsl#L319-L330` redshift expansion) + `far_shell.wgsl#L150-L161` 5-petal lotus (`lotus_boost=0.42`)

<a id="pkg-act5-exp11-z00-04"></a>
### PKG-ACT5-EXP11-Z00-04: Cosmic Evidence Epistemic Axis, Direct Telescopes, Relic Fossils, Inference & Predictions

- **Phase:** Phase 1/2 | **Priority:** `P1` | **Scope:** Exp `11` (`11_cosmic_evidence`) `Z00..Z04` (`5` zones, `26` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING` , `PKG-P0-CATALOG-SCENE-AND-CI` (backfill `scene`)
- **Current State:** `doc_stages.rs#L62`: `110 + (sc.unwrap_or(z) % 4)` (`scene` is `null` across all 9 zones -> modulo collapses to `z % 4`!); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L47-L104` (`mode 0` 5-galaxy redshift+Fraunhofer, `1` CMB sphere, `2` Einstein cross, `else` 4 tilted rings — `feat` only tilts rings at `L98`). Tier breakdown for this batch: **`0A / 1B / 3C / 1D` Zones**, **`0A / 1B / 11C / 14D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `1/26` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 2100..=2104` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3720`, `3721`, `3722`, `3723`, `3724` (allocated from `3720 + (11 - 11)*10 + k`; `mode=13` Redshift & Multi-Messenger Photon Flux (`doc_feat`-driven)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z00` | `handoff` (*Límite de la física conocida* / *Boundary of known physics*) | `[TIER-C]` | `vid=110` (`scene=null`) | **`2100`** | `3720` | `0` default (`1` instrument/spectrum cutaway, `2` counterfactual uncertainty overlay) | `0x02` (0x02 dual-hero/lens) | `0..4` (`0xFF` macro) |
| `Z01` | `observed` (*Observación directa* / *direct observation*) | `[TIER-B]` | `vid=111` (`scene=null`) | **`2101`** | `3721` | `0` default (`1` instrument/spectrum cutaway, `2` counterfactual uncertainty overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..4` (`0xFF` macro) |
| `Z02` | `fossil-traces` (*Huellas fósiles* / *Fossil traces*) | `[TIER-C]` | `vid=112` (`scene=null`) | **`2102`** | `3722` | `0` default (`1` instrument/spectrum cutaway, `2` counterfactual uncertainty overlay) | `0x02` (0x02 dual-hero/lens) | `0..4` (`0xFF` macro) |
| `Z03` | `inference` (*Inferencias robustas* / *Robust inferences*) | `[TIER-D]` | `vid=113` (`scene=null`) | **`2103`** | `3723` | `0` default (`1` instrument/spectrum cutaway, `2` counterfactual uncertainty overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z04` | `testable-predictions` (*Predicciones comprobables* / *Testable predictions*) | `[TIER-C]` | `vid=110` (`scene=null`) | **`2104`** | `3724` | `0` default (`1` instrument/spectrum cutaway, `2` counterfactual uncertainty overlay) | `0x04` (0x04 semantic cue mask) | `0..4` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z00` `handoff` (`vid=2100`) — Límite de la física conocida (*Boundary of known physics*) `[TIER-C]`

- **Catalog Metadata:** `scene=null` · `[0.000, 0.065]` · `rgb=[222, 230, 255]` · `sound=(triangle/78Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Continuidad con el universo temprano · la cronología se detiene` / *`Continuation from the early universe · the timeline stops`* · Evidence: `Punto de partida` (*Starting point*)
- **Spoken Macro Narrative (ES / EN):** Aquí termina el recorrido hacia épocas cada vez más tempranas. Desde este punto, subir ya no significa retroceder en el tiempo: significa alejarse gradualmente de la evidencia directa. (*EN:* Here ends the journey towards ever earlier times. From this point, going up no longer means going back in time: it means gradually moving away from direct evidence.)
- **Current Diagnosis & Target 3D Blueprint:** scene=None triggers 110 + (0 % 4) = 110 (mode=0: 5 receding galaxy disks + Fraunhofer bar). Narration introduces the epistemic axis pivot from cosmic time to evidential strength; Hubble galaxies are a modulo fallback mismatch. **Target (`vid=2100`):** SDF Horizon Bridge & Epistemic Axis Prism: horizontal timeline ribbon curving vertically into 5 translucent stepped glass strata (Observed -> Unknown), with doc_feat.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **cambio de eje** (*change of axis*) | `[TIER-C]` | La altura deja de representar tiempo y comienza a representar la fuerza relativa de la evidencia científica. / *Height stops representing time and begins to represent the relative strength of scientific evidence.* | axis pivot |
| `F1` | **teorías insuficientes** (*insufficient theories*) | `[TIER-C]` | La relatividad general y la física cuántica funcionan extraordinariamente bien en sus dominios, pero no forman todavía una descripción completa de condiciones extremas. / *General relativity and quantum physics work extremely well in their domains, but they do not yet form a complete description of extreme conditions.* | GR curved grid vs QM wavepacket split |
| `F2` | **sin destino confirmado** (*no confirmed destination*) | `[TIER-C]` | El límite no contiene una singularidad observada, un primer instante fotografiado ni una respuesta definitiva sobre el origen. / *The boundary does not contain an observed singularity, a first photographed instant, or a definitive answer about the origin.* | non-singularity open cap |
| `F3` | **disciplina científica** (*scientific discipline*) | `[TIER-C]` | El recorrido separa datos, reconstrucciones, predicciones, hipótesis y desconocimiento en lugar de mezclarlos. / *The journey separates data, reconstructions, predictions, hypotheses and ignorance instead of mixing them.* | strata filter bars |
| `F4` | **nueva pregunta** (*new question*) | `[TIER-C]` | ¿Qué puede sostener la evidencia y qué permanece abierto a nuevas observaciones o teorías? / *What can the evidence support and what remains open to new observations or theories?* | glowing interrogative horizon pulse |

##### `Z01` `observed` (`vid=2101`) — Observación directa (*direct observation*) `[TIER-B]`

- **Catalog Metadata:** `scene=null` · `[0.065, 0.205]` · `rgb=[113, 211, 255]` · `sound=(sine/92Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Datos medidos · estructuras nítidas · resultados reproducibles` / *`Measured data · clear structures · reproducible results`* · Evidence: `Observado` (*Observed*)
- **Spoken Macro Narrative (ES / EN):** La base más firme está formada por señales registradas con instrumentos, mapas del cielo, abundancias medidas y física de partículas comprobada en laboratorios. (*EN:* The firmest foundation consists of signals recorded by instruments, sky maps, measured abundances, and particle physics tested in laboratories.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via 110 + (1 % 4) = 111 (mode=1 CMB acoustic multipole sphere). Matches F00 (cosmic microwave background sky map) at the archetype level, but doc_feat is completely ignored so F01 expansion, F02 H/He spectral lines, F03 3D galaxy web, and F04 collider laboratories have zero representation. **Target (`vid=2101`):** Interactive 5-Instrument Direct Observation Stage (vid=2101 upgrade).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **fondo cósmico** (*cosmic microwave background*) | `[TIER-B]` | Los detectores miden un cielo de microondas con un espectro casi térmico y pequeñas variaciones angulares. / *The detectors measure a microwave sky with a near-thermal spectrum and small angular variations.* | highlights CMB dipole/multipole sky map |
| `F1` | **expansión cósmica** (*cosmic expansion*) | `[TIER-D]` | El desplazamiento al rojo, las distancias y múltiples observaciones muestran que la escala del universo cambia con el tiempo. / *Redshift, distances, and multiple observations show that the scale of the universe changes over time.* | expands wireframe metric grid + Doppler redshift galaxies |
| `F2` | **hidrógeno y helio** (*hydrogen and helium*) | `[TIER-D]` | Las abundancias de elementos ligeros pueden medirse en entornos antiguos y compararse con cálculos físicos. / *The abundances of light elements can be measured in ancient environments and compared with physical calculations.* | morphs to D/He-4 spectral emission comb |
| `F3` | **distribución de galaxias** (*galaxy distribution*) | `[TIER-D]` | Los sondeos cartografían galaxias, cúmulos, filamentos y vacíos a gran escala. / *Surveys map galaxies, galaxy clusters, filaments, and voids on large scales.* | spawns compute-mode-13 cosmic filament web |
| `F4` | **laboratorios** (*laboratories*) | `[TIER-D]` | Aceleradores y detectores prueban partículas, campos e interacciones que también se aplican a la cosmología temprana. / *Accelerators and detectors probe particles, fields and interactions that also apply to early cosmology.* | reveals toroidal particle-collider ring with vertex shower sprites |

##### `Z02` `fossil-traces` (`vid=2102`) — Huellas fósiles (*Fossil traces*) `[TIER-C]`

- **Catalog Metadata:** `scene=null` · `[0.205, 0.350]` · `rgb=[114, 244, 207]` · `sound=(sine/111Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Patrones medidos · memoria física del universo temprano` / *`Measured patterns · physical memory of the early universe`* · Evidence: `Observado → reconstruido` (*Observed → reconstructed*)
- **Spoken Macro Narrative (ES / EN):** Algunas observaciones funcionan como fósiles: no muestran directamente el proceso original, pero conservan patrones producidos por él. (*EN:* Some observations function like fossils: they do not directly show the original process, but they preserve patterns produced by it.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via 110 + (2 % 4) = 112 (mode=2 Einstein Cross lensed quasar + dark halo). Strong gravitational lensing does not depict CMB temperature anisotropies, E-mode polarization, BAO sound horizon rings, or primordial fluctuation power spectra. **Target (`vid=2102`):** Relic Echo & Sound-Horizon Shell (new vid=2102): dual-layer CMB anisotropy shell + frozen spherical BAO acoustic ripple shell (150 Mpc ring) seeded with compute-particle density nodes.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **anisotropías** (*anisotropies*) | `[TIER-C]` | Las pequeñas diferencias del fondo cósmico codifican densidad, movimiento y condiciones físicas del plasma temprano. / *Small differences in the cosmic background encode the density, motion, and physical conditions of the early plasma.* | temperature spots |
| `F1` | **polarización** (*polarization*) | `[TIER-C]` | La orientación de la radiación añade información independiente sobre dispersión, geometría y perturbaciones. / *Radiation orientation adds independent information on scattering, geometry, and perturbations.* | tangential polarization tick field |
| `F2` | **oscilaciones acústicas** (*acoustic oscillations*) | `[TIER-D]` | Una escala preferente en la distribución de materia conserva la huella de ondas del plasma primordial. / *A characteristic scale in the distribution of matter preserves the imprint of waves in the primordial plasma.* | BAO spherical wavecrest |
| `F3` | **semillas de estructura** (*seeds of structure*) | `[TIER-D]` | Las regiones ligeramente más densas crecieron por gravedad hasta formar galaxias, cúmulos y la red cósmica. / *Slightly denser regions grew by gravity to form galaxies, clusters, and the cosmic web.* | gravitational filament collapse seeds |
| `F4` | **fluctuaciones primordiales** (*primordial fluctuations*) | `[TIER-C]` | Su forma estadística se reconstruye comparando mapas del fondo cósmico y estructura a gran escala. / *Its statistical form is reconstructed by comparing maps of the cosmic background and large-scale structure.* | power-spectrum P(k) harmonic arc |

##### `Z03` `inference` (`vid=2103`) — Inferencias robustas (*Robust inferences*) `[TIER-D]`

- **Catalog Metadata:** `scene=null` · `[0.350, 0.500]` · `rgb=[255, 220, 112]` · `sound=(triangle/132Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Múltiples evidencias convergentes · reconstrucción física` / *`Multiple converging lines of evidence · physical reconstruction`* · Evidence: `Inferido` (*Inferred*)
- **Spoken Macro Narrative (ES / EN):** Estas ideas no son fotografías directas, pero explican conjuntos amplios de datos mediante teorías que pueden contrastarse entre sí. (*EN:* These ideas are not direct snapshots, but they explain large sets of data through theories that can be tested against each other.)
- **Current Diagnosis & Target 3D Blueprint:** Routed via 110 + (3 % 4) = 113 (mode=3 four tilted concentric torus rings; doc_feat only nudges ring tilt by 0.15 rad). Fails to depict concrete inferred physics: opaque early plasma, BBN D/He fusion network, relic neutrinos, dark matter gravitational halo, perturbation growth, or inflation. **Target (`vid=2103`):** Converging Inference Core (new vid=2103).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **plasma primordial** (*primordial plasma*) | `[TIER-D]` | La opacidad previa al fondo cósmico se infiere a partir de la física de cargas libres y de los patrones observados. / *The opacity before the cosmic microwave background was released is inferred from the physics of free charges and observed patterns.* | glowing opaque scattering fog suddenly clearing at z~1100 |
| `F1` | **nucleosíntesis** (*nucleosynthesis*) | `[TIER-D]` | Las reacciones de los primeros minutos conectan parámetros cosmológicos con abundancias de deuterio y helio. / *The reactions of the first minutes connect cosmological parameters with abundances of deuterium and helium.* | proton-neutron D/He-4 reaction nodes |
| `F2` | **fondo de neutrinos** (*cosmic neutrino background*) | `[TIER-D]` | Es una predicción robusta del Big Bang caliente y deja efectos indirectos, aunque sus neutrinos relictos individuales no se han detectado directamente. / *It is a robust prediction of the Hot Big Bang and leaves indirect effects, although its individual relic neutrinos have not been detected directly.* | ghostly translucent cosmic neutrino pass-through swarm |
| `F3` | **materia oscura** (*dark matter*) | `[TIER-D]` | Su gravedad explica numerosos fenómenos, pero la naturaleza microscópica de la materia oscura sigue sin identificarse. / *Its gravity explains numerous phenomena, but the microscopic nature of dark matter remains unidentified.* | translucent purple dark-matter gravitational halo scaffolding luminous baryons |
| `F4` | **perturbaciones** (*perturbations*) | `[TIER-D]` | Las ecuaciones describen cómo pequeñas variaciones evolucionan hasta convertirse en la estructura cósmica observada. / *The equations describe how small variations evolve into the observed cosmic structure.* | spherical-harmonic delta-rho growth |
| `F5` | **fase inflacionaria** (*inflationary phase*) | `[TIER-D]` | Una fase inflacionaria puede explicar rasgos importantes, pero su mecanismo concreto y su escala de energía no están confirmados. / *An inflationary phase can explain important features, but its specific mechanism and energy scale are not confirmed.* | rapid exponential metric-grid stretch |

##### `Z04` `testable-predictions` (`vid=2104`) — Predicciones comprobables (*Testable predictions*) `[TIER-C]`

- **Catalog Metadata:** `scene=null` · `[0.500, 0.625]` · `rgb=[255, 165, 108]` · `sound=(triangle/154Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `inflacion`
- **Metric & Evidence (ES / EN):** `Una teoría se arriesga · la observación puede apoyarla o descartarla` / *`A theory makes testable commitments · observation can support or rule it out`* · Evidence: `Predicho` (*Predicted*)
- **Spoken Macro Narrative (ES / EN):** Una predicción científica valiosa especifica qué patrón debería aparecer, dónde buscarlo y qué resultado contaría en contra del modelo. (*EN:* A valuable scientific prediction specifies what pattern should appear, where to look for it, and what result would count against the model.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps around via 110 + (4 % 4) = 110 (mode=0 receding galaxies + Fraunhofer bar). Narration covers Popperian falsifiability, CMB polarization templates, spectral tilt, bispectrum non-Gaussianity, and reheating; modulo galaxy/Fraunhofer scene mismatches the statistical prediction subject. **Target (`vid=2104`):** Predictive Template Overlay Stage (new vid=2104): translucent theoretical waveform vs incoming noisy measurement ring.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **patrón de polarización** (*polarization pattern*) | `[TIER-D]` | Diferentes mecanismos tempranos pueden dejar correlaciones concretas en la polarización del fondo cósmico. / *Different early mechanisms can leave concrete correlations in the polarization of the cosmic background.* | E/B curl polarization overlay |
| `F1` | **forma del espectro** (*spectrum shape*) | `[TIER-C]` | Los modelos predicen la amplitud, inclinación y posibles desviaciones de las fluctuaciones primordiales. / *Models predict the amplitude, tilt, and possible deviations of primordial fluctuations.* | spectral index tilt dial n_s < 1 |
| `F2` | **no gaussianidad** (*non-Gaussianity*) | `[TIER-C]` | Ciertos escenarios producirían relaciones estadísticas específicas entre modos del fondo cósmico y la estructura. / *Certain scenarios would produce specific statistical relationships among modes of the cosmic microwave background and large-scale structure.* | bispectrum triangle correlation nodes |
| `F3` | **recalentamiento** (*reheating*) `[kw:inflacion]` | `[TIER-D]` | La transición posterior a una posible inflación podría dejar consecuencias en partículas, ondas o distribución de perturbaciones. / *The transition after a possible inflationary phase could leave signatures in particles, waves, or the distribution of perturbations.* | post-inflation reheating energy-transfer burst particles |
| `F4` | **consistencia cruzada** (*cross-consistency*) | `[TIER-C]` | Una predicción gana fuerza cuando varios instrumentos y mensajeros independientes encuentran el mismo patrón. / *A prediction gains strength when several independent instruments and messengers find the same pattern.* | multi-messenger alignment beams |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP11-Z00-04)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP11-Z00-04` (Cosmic Evidence Epistemic Axis, Direct Telescopes, Relic Fossils, Inference & Predictions) for HyperCosmosVR Odyssey Act V (`Exp 11: 11_cosmic_evidence`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z00`->`2100` (`F0..F4`), `Z01`->`2101` (`F0..F4`), `Z02`->`2102` (`F0..F4`), `Z03`->`2103` (`F0..F5`), `Z04`->`2104` (`F0..F4`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3720`, `3721`, `3722`, `3723`, `3724`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP11-Z00-04`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Confirm `scene` backfill (`0..8` / `0..9`) in `assets/baked/universe_catalog.v1.json` and pair `mode=13` redshift particles in `shaders/compute_universe.wgsl` cleanly with each era.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z00: handoff / Boundary of known physics, Z01: observed / direct observation, Z02: fossil-traces / Fossil traces, Z03: inference / Robust inferences, Z04: testable-predictions / Testable predictions.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp11-z05-08"></a>
### PKG-ACT5-EXP11-Z05-08: Undetected Sectors, Quantum Gravity Foam, Initial Boundary Conditions & Causal Frontier

- **Phase:** Phase 1/2 | **Priority:** `P1` | **Scope:** Exp `11` (`11_cosmic_evidence`) `Z05..Z08` (`4` zones, `24` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING` , `PKG-P0-CATALOG-SCENE-AND-CI` (backfill `scene`)
- **Current State:** `doc_stages.rs#L62`: `110 + (sc.unwrap_or(z) % 4)` (`scene` is `null` across all 9 zones -> modulo collapses to `z % 4`!); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L47-L104` (`mode 0` 5-galaxy redshift+Fraunhofer, `1` CMB sphere, `2` Einstein cross, `else` 4 tilted rings — `feat` only tilts rings at `L98`). Tier breakdown for this batch: **`0A / 0B / 2C / 2D` Zones**, **`0A / 0B / 11C / 13D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `3/4` zones and `3/24` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 2105..=2108` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3725`, `3726`, `3727`, `3728` (allocated from `3720 + (11 - 11)*10 + k`; Planck-Foam Quiver & Null-Exclusion Field (`mode=13` modulated)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z05` | `undetected` (*Señales aún no detectadas* / *Signals not yet detected*) | `[TIER-D]` | `vid=111` (`scene=null`) | **`2105`** | `3725` | `0` default (`1` instrument/spectrum cutaway, `2` counterfactual uncertainty overlay) | `0x04` (0x04 semantic cue mask) | `0..5` (`0xFF` macro) |
| `Z06` | `quantum-gravity` (*Modelos de gravedad cuántica* / *Quantum gravity models*) | `[TIER-D]` | `vid=112` (`scene=null`) | **`2106`** | `3726` | `0` default (`1` instrument/spectrum cutaway, `2` counterfactual uncertainty overlay) | `0x06` (0x04 semantic cue mask, 0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z07` | `initial-conditions` (*Condiciones iniciales* / *Initial conditions*) | `[TIER-C]` | `vid=113` (`scene=null`) | **`2107`** | `3727` | `0` default (`1` instrument/spectrum cutaway, `2` counterfactual uncertainty overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z08` | `knowable-frontier` (*Frontera de lo conocible* / *Frontier of the knowable*) | `[TIER-C]` | `vid=110` (`scene=null`) | **`2108`** | `3728` | `0` default (`1` instrument/spectrum cutaway, `2` counterfactual uncertainty overlay) | `0x04` (0x04 semantic cue mask) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z05` `undetected` (`vid=2105`) — Señales aún no detectadas (*Signals not yet detected*) `[TIER-D]`

- **Catalog Metadata:** `scene=null` · `[0.625, 0.745]` · `rgb=[255, 116, 157]` · `sound=(sawtooth/176Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `gravitacional`
- **Metric & Evidence (ES / EN):** `Instrumentos entre ruido · límites superiores · ausencia informativa` / *`Instruments amid noise · upper limits · informative absence`* · Evidence: `Buscado · no confirmado` (*Sought · not confirmed*)
- **Spoken Macro Narrative (ES / EN):** No detectar una señal no demuestra que sea imposible. Permite reducir el espacio de modelos y diseñar búsquedas más sensibles. (*EN:* Failing to detect a signal does not prove it is impossible. It narrows the range of viable models and helps design more sensitive searches.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps around via 110 + (5 % 4) = 111 (mode=1 smooth CMB sphere). Narration details concrete undetected physics experiments and signals (primordial tensor gravitational waves, B-mode curl vs Galactic dust, WIMP DM detectors, Super-K-style proton decay water tanks, collider new symmetries, reheating), none of which exist in the smooth CMB sphere. **Target (`vid=2105`):** Null-Search Detector Suite (new vid=2105).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **ondas primordiales** (*primordial waves*) `[kw:gravitacional]` | `[TIER-D]` | Una señal gravitacional de épocas extremadamente tempranas sería extraordinaria, pero no existe una detección confirmada de ese fondo primordial. / *A gravitational signal from extremely early times would be extraordinary, but there is no confirmed detection of that primordial background.* | quadrupole plus/cross GW spacetime strain mesh |
| `F1` | **modo B primordial** (*primordial B-mode*) | `[TIER-D]` | La polarización buscada debe separarse cuidadosamente del polvo galáctico, lentes gravitacionales y ruido instrumental. / *The sought polarization must be carefully separated from galactic dust, gravitational lensing and instrumental noise.* | foreground polarized dust filament layer masking faint tensor B-mode curl |
| `F2` | **partícula de materia oscura** (*dark matter particle*) | `[TIER-D]` | Numerosos experimentos buscan interacciones no gravitatorias, sin una identificación confirmada de la partícula responsable. / *Numerous experiments search for non-gravitational interactions, without a confirmed identification of the particle responsible.* | deep underground cryostat xenon target chamber with excluded exclusion-curve arc |
| `F3` | **decaimiento del protón** (*proton decay*) | `[TIER-D]` | Algunas teorías de unificación lo predicen; los experimentos han establecido vidas medias mínimas cada vez mayores sin observar un caso confirmado. / *Some unification theories predict it; experiments have set increasingly long lower limits on the proton lifetime without observing a confirmed event.* | Cherenkov PMT tank perimeter with null threshold ring |
| `F4` | **nuevas simetrías** (*new symmetries*) | `[TIER-D]` | Partículas asociadas a simetrías adicionales podrían aparecer en colisionadores, astrofísica o experimentos de precisión. / *Particles associated with additional symmetries might appear in collider experiments, astrophysical observations, or precision measurements.* | collider detector barrel searching SUSY tracks |
| `F5` | **huella del recalentamiento** (*reheating signature*) | `[TIER-D]` | Se buscan señales indirectas capaces de distinguir cómo terminó una posible etapa inflacionaria. / *Indirect signs are sought capable of distinguishing how a possible inflationary stage ended.* | inflationary end-state decay spectrum |

##### `Z06` `quantum-gravity` (`vid=2106`) — Modelos de gravedad cuántica (*Quantum gravity models*) `[TIER-D]`

- **Catalog Metadata:** `scene=null` · `[0.745, 0.855]` · `rgb=[176, 129, 255]` · `sound=(sine/198Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `planck`
- **Metric & Evidence (ES / EN):** `Sin teoría confirmada · geometrías alternativas · pruebas difíciles` / *`No confirmed theory · alternative geometries · difficult tests`* · Evidence: `Hipotético · marcos competidores` (*Hypothetical · competing frameworks*)
- **Spoken Macro Narrative (ES / EN):** Aquí las representaciones se vuelven abstractas porque todavía no existe una teoría de gravedad cuántica verificada de forma experimental. (*EN:* Here the representations become abstract because there is still no experimentally verified theory of quantum gravity.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps around via 110 + (6 % 4) = 112 (mode=2 Einstein Cross lensed quasar). Strong-lensing quasar has zero relation to Planck-length quantum spacetime foam, emergent geometry, vibrating strings, loop quantum gravity spin networks, or causal sets. **Target (`vid=2106`):** Planck-Scale Quantum Spacetime Gallery (new vid=2106).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **espacio-tiempo cuántico** (*quantum spacetime*) | `[TIER-D]` | A escalas extremas, el espacio y el tiempo clásicos podrían dejar de ser una base continua y fija. / *At extreme scales, classical space and time may no longer provide a continuous, fixed foundation.* | flickering turbulent Wheeler quantum foam lattice |
| `F1` | **longitud de Planck** (*Planck length*) `[kw:planck]` | `[TIER-D]` | Es una escala construida con constantes fundamentales; señala dónde nuestras extrapolaciones ordinarias pueden perder fiabilidad. / *It is a scale constructed from fundamental constants; it marks where ordinary extrapolations may become unreliable.* | shrinking logarithmic ruler terminating at Planck cell (10^-35 m) |
| `F2` | **geometría emergente** (*emergent geometry*) | `[TIER-D]` | Algunos enfoques exploran si el espacio-tiempo surge de grados de libertad más fundamentales. / *Some approaches explore whether spacetime arises from more fundamental degrees of freedom.* | holographic boundary graph emitting bulk geometry nodes |
| `F3` | **teoría de cuerdas** (*string theory*) | `[TIER-D]` | Es un amplio marco de investigación con herramientas potentes, no una explicación cosmológica confirmada. / *It is a broad research framework with powerful tools, not a confirmed cosmological explanation.* | vibrating closed/open Calabi-Yau string loops |
| `F4` | **gravedad cuántica de bucles** (*loop quantum gravity*) | `[TIER-D]` | Es otro programa de investigación que cuantiza aspectos de la geometría, sin estatus de respuesta establecida. / *It is another research program that quantizes aspects of geometry, but it is not an established answer.* | interlocking SU(2) spin-network polyhedral nodes |
| `F5` | **otras vías** (*other approaches*) | `[TIER-D]` | Seguridad asintótica, conjuntos causales, triangulaciones y otros enfoques compiten sin un vencedor experimental. / *Asymptotic safety, causal sets, triangulations, and other approaches remain contenders with no experimental winner.* | partially ordered causal-set diamond DAG |

##### `Z07` `initial-conditions` (`vid=2107`) — Condiciones iniciales (*Initial conditions*) `[TIER-C]`

- **Catalog Metadata:** `scene=null` · `[0.855, 0.940]` · `rgb=[213, 188, 255]` · `sound=(triangle/220Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Varias posibilidades · ninguna selección confirmada` / *`Several possibilities · no confirmed selection`* · Evidence: `Pregunta abierta` (*open question*)
- **Spoken Macro Narrative (ES / EN):** Las preguntas sustituyen a los objetos: no sabemos qué principio, proceso o condición determinó el estado inicial accesible a nuestras teorías. (*EN:* Questions replace objects: we do not know what principle, process or condition determined the initial state accessible to our theories.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps around via 110 + (7 % 4) = 113 (mode=3 four tilted torus rings, tiny 0.15 rad feat tilt). Abstract open questions on low-entropy initial state, constants, fluctuation origin, and emergence of time are shown only as four spinning rings. **Target (`vid=2107`):** Boundary-Condition Bifurcation Bowl (new vid=2107): smooth Penrose low-entropy Weyl baseline manifold splitting into dim flickering candidate initial-state branches without selecting a winner.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **uniformidad** (*uniformity*) | `[TIER-C]` | ¿Por qué el universo observable comenzó en un estado tan uniforme y con entropía gravitatoria tan baja? / *Why did the observable universe begin in such a uniform state and with such low gravitational entropy?* | thermal flatness |
| `F1` | **leyes físicas** (*physical laws*) | `[TIER-C]` | ¿Por qué existen estas leyes y estos valores de constantes, en lugar de otros? / *Why do these laws and values of constants exist, instead of others?* | dimensionless-constant sliders |
| `F2` | **fluctuaciones** (*fluctuations*) | `[TIER-C]` | ¿Qué determinó el estado y las propiedades precisas de las perturbaciones iniciales? / *What determined the state and precise properties of the initial perturbations?* | quantum perturbation seed clouds |
| `F3` | **sentido de «antes»** (*meaning of “before”*) | `[TIER-C]` | Si el tiempo clásico emerge o tiene un límite, preguntar por un antes podría no estar bien definido. / *If classical time emerges or has a boundary, asking what came before may not be well defined.* | Euclidean rounded no-boundary cap where 'before' dissolves |
| `F4` | **comprobabilidad** (*testability*) | `[TIER-C]` | ¿Qué clases de explicaciones producirían consecuencias observables y cuáles quedarían fuera de contraste? / *Which kinds of explanations would produce observable consequences, and which would lie beyond empirical testing?* | empirical cone mask |
| `F5` | **sin selección** (*no selection*) | `[TIER-C]` | La visualización abre ramas compatibles, pero no escoge una como historia real del universo. / *The visualization opens compatible branches, but does not choose one as the actual history of the universe.* | equal-weight ghost branches |

##### `Z08` `knowable-frontier` (`vid=2108`) — Frontera de lo conocible (*Frontier of the knowable*) `[TIER-C]`

- **Catalog Metadata:** `scene=null` · `[0.940, 1.000]` · `rgb=[238, 239, 255]` · `sound=(sine/242Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `horizonte`
- **Metric & Evidence (ES / EN):** `Límites de medición · horizontes causales · teorías incompletas` / *`Measurement limits · causal horizons · incomplete theories`* · Evidence: `Desconocido` (*Unknown*)
- **Spoken Macro Narrative (ES / EN):** La ausencia de una respuesta confirmada no es un vacío de la ciencia. Es la frontera donde comienzan nuevas preguntas. (*EN:* The absence of a confirmed answer is not a gap in science. It is the border where new questions begin.)
- **Current Diagnosis & Target 3D Blueprint:** Wraps around via 110 + (8 % 4) = 110 (mode=0 receding galaxies + Fraunhofer bar). Closing epistemological boundary ('clearly labeled possibilities and limits', no multiverse/singularity/creator) mismatches the Hubble-diagram modulo slot. **Target (`vid=2108`):** Causal Horizon Frontier Dome (new vid=2108): translucent spherical particle-horizon boundary surrounded by muted wireframe uncertainty haze.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **horizonte causal** (*causal horizon*) `[kw:horizonte]` | `[TIER-D]` | Hay regiones o sucesos cuya información no puede alcanzarnos debido a la historia y geometría del espacio-tiempo. / *There are regions or events whose information cannot reach us due to the history and geometry of space-time.* | outward causal light-cone cutoff sphere |
| `F1` | **incertidumbre** (*uncertainty*) | `[TIER-C]` | Todo experimento tiene sensibilidad finita, ruido, degeneraciones y límites fundamentales de inferencia. / *Every experiment has finite sensitivity, noise, degeneracies, and fundamental limits on inference.* | blurred Bayesian confidence band |
| `F2` | **teoría incompleta** (*incomplete theory*) | `[TIER-C]` | Una ecuación que falla en un límite extremo señala trabajo pendiente; no convierte automáticamente una extrapolación en realidad. / *An equation that fails at an extreme limit signals that more work is needed; it does not automatically turn an extrapolation into reality.* | diverging breakdown grid at extreme curvature |
| `F3` | **pregunta futura** (*future question*) | `[TIER-C]` | Nuevos detectores, mensajeros y métodos podrían transformar algunas preguntas actuales en pruebas concretas. / *New detectors, messengers, and methods could turn some current questions into concrete tests.* | next-gen detector aperture cones piercing the haze |
| `F4` | **fuera de observación** (*beyond observation*) | `[TIER-C]` | Algunas afirmaciones podrían no producir nunca diferencias medibles y quedar fuera del alcance de la ciencia empírica. / *Some statements may never produce measurable differences and remain outside the scope of empirical science.* | unobservable outside-cone void |
| `F5` | **campo abierto** (*open field*) | `[TIER-C]` | El final no contiene creador, multiverso, singularidad ni origen definitivo: contiene posibilidades y límites claramente etiquetados. / *The ending contains no creator, no multiverse, no singularity, no definitive origin: it contains clearly labeled possibilities and limits.* | clean empty labeled frontier bridging to Exp 12 |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP11-Z05-08)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP11-Z05-08` (Undetected Sectors, Quantum Gravity Foam, Initial Boundary Conditions & Causal Frontier) for HyperCosmosVR Odyssey Act V (`Exp 11: 11_cosmic_evidence`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z05`->`2105` (`F0..F5`), `Z06`->`2106` (`F0..F5`), `Z07`->`2107` (`F0..F5`), `Z08`->`2108` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3725`, `3726`, `3727`, `3728`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP11-Z05-08`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Confirm `scene` backfill (`0..8` / `0..9`) in `assets/baked/universe_catalog.v1.json` and pair `mode=13` redshift particles in `shaders/compute_universe.wgsl` cleanly with each era.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z05: undetected / Signals not yet detected, Z06: quantum-gravity / Quantum gravity models, Z07: initial-conditions / Initial conditions, Z08: knowable-frontier / Frontier of the knowable.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

### 4.5 Experience 12: `12_future_of_universe` — Espectro del Futuro del Universo: de la Era Estelar a la Oscuridad Cósmica

- **Odyssey & Spatial Placement:** Act 5 (`5/8`, overall stop #22) · `HYPER_LOTUS_12_FUTURE` (`parent UNIVERSE_ROOT`, `u = [27.2, 28.6]`, `focus_scale_exp = 27.6`, `6.0e27 m`) ([`topology.rs#L217-L228, L654`](../../crates/cosmos_spatial/src/topology.rs#L217-L228))
- **Experience Totals:** `10` Zones (`0A / 3B / 4C / 3D`) · `59` Features (`0A / 10B / 21C / 28D`)
- **Current Shader & Compute:** **CPU BYPASS BUG** `doc_stages.rs#L657-L691` (`5 | 12 =>`) skips `doc_stage_variant_id(12)` (`120+(sc%5)` at `L63`) & renders hardcoded Kerr BH + crimson dwarf on all 10 zones! · `shaders/hero_doc_cosmos.wgsl#L411-L505` (`kind=4, vid=12`, up to `50` steps!) + `#L204-L229` (`kind=1, vid=12`). Dead WGSL: `hero_doc_civilization_horizon.wgsl#L105-L142` · `mode=13` (`universe_scene.rs#L310`, `compute_universe.wgsl#L319-L330` redshift stream even on `Z08` Dark Era) + `far_shell.wgsl#L150-L161` (`lotus_boost=0.42`)

<a id="pkg-act5-exp12-z00-04"></a>
### PKG-ACT5-EXP12-Z00-04: Remove `5|12` CPU Bypass, Time-Axis Pivot, Present Cosmos, Stellar Decline & Red Dwarfs

- **Phase:** Phase 1/2 | **Priority:** `P0` | **Scope:** Exp `12` (`12_future_of_universe`) `Z00..Z04` (`5` zones, `29` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING` , `PKG-P0-CATALOG-SCENE-AND-CI` (backfill `scene`)
- **Current State:** **CPU BYPASS BUG** `doc_stages.rs#L657-L691` (`5 | 12 =>`) skips `doc_stage_variant_id(12)` (`120+(sc%5)` at `L63`) & renders hardcoded Kerr BH + crimson dwarf on all 10 zones!; rendered by `shaders/hero_doc_cosmos.wgsl#L411-L505` (`kind=4, vid=12`, up to `50` steps!) + `#L204-L229` (`kind=1, vid=12`). Dead WGSL: `hero_doc_civilization_horizon.wgsl#L105-L142`. Tier breakdown for this batch: **`0A / 0B / 2C / 3D` Zones**, **`0A / 1B / 13C / 15D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `0/29` features; `doc_stages.rs` drops `cue_tags`). Includes `5 | 12 =>` bypass removal (`doc_stages.rs#L657-L691`).
- **Reserved Target IDs:** Primary `vid = 2200..=2204` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3730`, `3731`, `3732`, `3733`, `3734` (allocated from `3720 + (12 - 11)*10 + k`; Era-Paced `mode=13` (Galaxy Merger -> Convective Amber Slowdown)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z00` | `handoff` (*Frontera de lo conocible* / *Frontier of the knowable*) | `[TIER-C]` | Bypassed `kind=4+1, vid=12` | **`2200`** | `3730` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..4` (`0xFF` macro) |
| `Z01` | `present` (*Universo actual* / *Present-day universe*) | `[TIER-D]` | Bypassed `kind=4+1, vid=12` | **`2201`** | `3731` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z02` | `decline` (*Ocaso de la formación estelar* / *Twilight of star formation*) | `[TIER-D]` | Bypassed `kind=4+1, vid=12` | **`2202`** | `3732` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z03` | `red-dwarfs` (*Era de las enanas rojas* / *Era of red dwarfs*) | `[TIER-C]` | Bypassed `kind=4+1, vid=12` | **`2203`** | `3733` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x05` (0x04 semantic cue mask, 0x01 cutaway) | `0..5` (`0xFF` macro) |
| `Z04` | `last-stars` (*Las últimas estrellas* / *The last stars*) | `[TIER-D]` | Bypassed `kind=4+1, vid=12` | **`2204`** | `3734` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z00` `handoff` (`vid=2200`) — Frontera de lo conocible (*Frontier of the knowable*) `[TIER-C]`

- **Catalog Metadata:** `scene=null` · `[0.000, 0.045]` · `rgb=[211, 222, 255]` · `sound=(sine/76Hz)` · `speak_evidence_default=False` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `La ruta del origen termina · el eje temporal vuelve a empezar` / *`The journey toward the origin ends · the time axis begins again`* · Evidence: `Continuidad` (*Continuity*)
- **Spoken Macro Narrative (ES / EN):** La aplicación anterior terminó separando evidencia y desconocimiento. Aquí giramos la mirada: regresamos al universo observable de hoy y proyectamos su evolución hacia delante. (*EN:* The previous app ended by separating evidence from the unknown. Here we turn our gaze back to today's observable universe and project its evolution forward.)
- **Current Diagnosis & Target 3D Blueprint:** Pipeline bypass bug in doc_stages.rs:657 (`5 \| 12 =>`) skips doc_stage_variant_id(12) and renders hardcoded Kerr BH (kind=4, vid=12) + cooling dwarf (kind=1, vid=12) while leaving hero_doc_civilization_horizon.wgsl:105-142 dead code. Z00 speaks of reversing the axis from origin-evidence back to forward cosmological time. **Target (`vid=2200`):** Time-Axis Pivot Anchor (vid=2200): glowing logarithmic timeline helix reversing upward from present-day galaxy anchor sphere into widening translucent uncertainty cones.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **cambio de dirección** (*change of direction*) | `[TIER-C]` | Subir vuelve a significar avanzar en el tiempo, no alejarse de la evidencia del origen. / *Going up again means moving forward in time, not moving away from the evidence of origin.* | forward arrow inversion |
| `F1` | **presente como ancla** (*the present as an anchor*) | `[TIER-C]` | Toda proyección comienza en el universo que medimos hoy: galaxias, estrellas, expansión y materia en evolución. / *Every projection begins in the universe we measure today: galaxies, stars, expansion and evolving matter.* | bright t=13.8 Gyr anchor disc |
| `F2` | **tiempo no lineal** (*nonlinear time*) | `[TIER-C]` | La altura ordena etapas, pero no representa una escala temporal uniforme: cada región puede durar muchísimo más que la anterior. / *Height orders stages, but does not represent a uniform time scale: each region can last much longer than the previous one.* | non-linear log10(years) tick spacing |
| `F3` | **certeza decreciente** (*decreasing certainty*) | `[TIER-C]` | Cuanto más remoto es el futuro, más depende de que las leyes y constantes actuales sigan funcionando como esperamos. / *The more remote the future, the more it depends on current laws and constants continuing to work as we expect.* | dimming opacity gradient with horizon |
| `F4` | **ningún guion cerrado** (*no closed script*) | `[TIER-C]` | El recorrido muestra una extrapolación científica, no una película confirmada del destino cósmico. / *The journey shows a scientific extrapolation, not a confirmed movie of the universe's fate.* | unscripted branching ribbon |

##### `Z01` `present` (`vid=2201`) — Universo actual (*Present-day universe*) `[TIER-D]`

- **Catalog Metadata:** `scene=null` · `[0.045, 0.170]` · `rgb=[119, 207, 255]` · `sound=(sine/96Hz)` · `speak_evidence_default=False` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Presente cosmológico · ≈13,8 mil millones de años desde el Big Bang` / *`Cosmological present · ≈13.8 billion years since the Big Bang`* · Evidence: `Observado hoy` (*Observed today*)
- **Spoken Macro Narrative (ES / EN):** El universo actual contiene galaxias activas, estrellas de muchas edades, planetas, agujeros negros, formación estelar y una expansión cuya aceleración se mide, aunque la naturaleza de la energía oscura sigue abierta. (*EN:* The current universe contains active galaxies, stars of many ages, planets, black holes, star formation and an expansion whose acceleration is measured, although the nature of dark energy remains open.)
- **Current Diagnosis & Target 3D Blueprint:** Forced Kerr BH + dying red ember instead of today's vibrant cosmos (cosmic web filaments, active cold gas star formation, diverse planetary systems, accelerated expansion, recycled supernova elements). Only F03 acknowledges black holes. **Target (`vid=2201`):** Present-Day Active Universe Stage (vid=2201): volumetric cosmic-web filament lattice with star-forming cyan/pink HII nurseries + young planetary disks.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **red cósmica** (*cosmic web*) | `[TIER-D]` | Galaxias y cúmulos forman filamentos y nodos separados por vacíos inmensos. / *Galaxies and galaxy clusters form filaments and nodes separated by immense voids.* | highlights Laniakea-like filaments/voids |
| `F1` | **formación estelar** (*star formation*) | `[TIER-D]` | Nubes frías de gas todavía colapsan y producen nuevas generaciones de estrellas. / *Cold clouds of gas still collapse and produce new generations of stars.* | glowing nebular collapse cores |
| `F2` | **sistemas planetarios** (*planetary systems*) | `[TIER-D]` | Los planetas son resultados comunes de la formación estelar, con historias muy distintas. / *Planets are common results of star formation, with very different histories.* | multi-planet Keplerian disk |
| `F3` | **agujeros negros** (*black holes*) | `[TIER-D]` | Agujeros negros estelares y supermasivos crecen, se fusionan y afectan a sus entornos. / *Stellar-mass and supermassive black holes grow, merge, and affect their surroundings.* | active AGN black hole jet |
| `F4` | **expansión acelerada** (*accelerated expansion*) | `[TIER-B]` | Las observaciones favorecen una expansión acelerada; su comportamiento futuro no está garantizado. / *Observations favor accelerated expansion, but its future behavior is not guaranteed.* | outward accelerated metric grid |
| `F5` | **materia reciclada** (*recycled matter*) | `[TIER-D]` | Las estrellas devuelven elementos al medio interestelar y alimentan nuevas estrellas, planetas y química compleja. / *Stars return elements to the interstellar medium and fuel new stars, planets, and complex chemistry.* | supernova shell enriching fresh gas |

##### `Z02` `decline` (`vid=2202`) — Ocaso de la formación estelar (*Twilight of star formation*) `[TIER-D]`

- **Catalog Metadata:** `scene=null` · `[0.170, 0.305]` · `rgb=[255, 177, 116]` · `sound=(triangle/88Hz)` · `speak_evidence_default=False` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `10¹⁰–10¹⁴ años · menos gas frío disponible` / *`10¹⁰–10¹⁴ years · less cold gas available`* · Evidence: `Evolución modelada` (*Modeled evolution*)
- **Spoken Macro Narrative (ES / EN):** La formación estelar no se apaga de golpe. Desciende gradualmente a medida que el gas se consume, se calienta, queda encerrado en remanentes o deja de poder enfriarse con eficacia. (*EN:* Star formation does not stop suddenly. It gradually declines as gas is consumed, heated, locked in remnants, or can no longer cool efficiently.)
- **Current Diagnosis & Target 3D Blueprint:** Hero WGSL ALREADY implements Milkomeda twin-spiral merger (`exp_id == 12u, mode == 0u/1u`, hero_doc_civilization_horizon.wgsl:105-142), yet it is 100% unreachable due to `5 \| 12 =>`! Screen shows a black hole + ember instead of redder aging galaxies, dying nurseries, galaxy mergers, and receding cosmological horizons. **Target (`vid=2202`):** Activate & Upgrade WGSL Milkomeda / Stellar Decline (vid=2202): fix doc_stages.rs dispatch so mode=0/1 twin-spiral merger renders.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **galaxias más rojas** (*redder galaxies*) | `[TIER-D]` | Al nacer menos estrellas azules masivas, domina la luz de poblaciones estelares más viejas y frías. / *As fewer massive blue stars are born, the light from older, cooler stellar populations dominates.* | shifts stellar sprites from OB blue to K/M amber |
| `F1` | **viveros que se apagan** (*fading stellar nurseries*) | `[TIER-D]` | Las nebulosas brillantes se vuelven menos frecuentes conforme disminuye el gas capaz de colapsar. / *Bright nebulae become less frequent as the gas capable of collapsing decreases.* | extinguishes pink HII clouds |
| `F2` | **fusiones galácticas** (*galaxy mergers*) | `[TIER-D]` | Las galaxias ligadas pueden fusionarse y reorganizar sus estrellas, gas y agujeros negros. / *Gravitationally bound galaxies can merge and rearrange their stars, gas, and black holes.* | drives tidal-bridge Milky Way + Andromeda coalescence |
| `F3` | **islas observables** (*observable islands*) | `[TIER-D]` | Con expansión acelerada persistente, sistemas no ligados pueden cruzar horizontes cosmológicos y dejar de ser observables entre sí. / *If accelerated expansion persists, unbound systems can cross cosmological horizons and cease to be mutually observable.* | fades unbound distant galaxies behind spherical horizon veil |
| `F4` | **reciclaje insuficiente** (*insufficient recycling*) | `[TIER-D]` | La materia expulsada por estrellas prolonga la actividad, pero no mantiene indefinidamente el ritmo actual. / *Matter ejected by stars prolongs activity, but does not maintain the current pace indefinitely.* | sputtered planetary nebulae wisps |
| `F5` | **incertidumbre oscura** (*dark-energy uncertainty*) | `[TIER-C]` | Cambios futuros en la energía oscura alterarían profundamente esta historia. / *Future changes in dark energy would profoundly alter this story.* | variable dark-energy pulse |

##### `Z03` `red-dwarfs` (`vid=2203`) — Era de las enanas rojas (*Era of red dwarfs*) `[TIER-C]`

- **Catalog Metadata:** `scene=null` · `[0.305, 0.445]` · `rgb=[255, 104, 86]` · `sound=(sine/72Hz)` · `speak_evidence_default=False` · `DOMAIN_CUE_KEYWORDS`: `sol`
- **Metric & Evidence (ES / EN):** `Hasta ~10¹³–10¹⁴ años · las estrellas pequeñas dominan` / *`Up to ~10¹³–10¹⁴ years · small stars dominate`* · Evidence: `Futuro probable` (*probable future*)
- **Spoken Macro Narrative (ES / EN):** Las enanas rojas consumen su combustible lentamente. Si la física estelar actual sigue siendo válida, muchas continuarán brillando cuando las estrellas semejantes al Sol hayan desaparecido hace muchísimo tiempo. (*EN:* Red dwarfs consume their fuel slowly. If current stellar physics remains valid, many will continue to shine when Sun-like stars have long since disappeared.)
- **Current Diagnosis & Target 3D Blueprint:** Wrong shared fallback: a giant central Kerr Black Hole dominates 80% of the stage while a small crimson cooling-dwarf ember (kind=1, vid=12) sits off-center. During the Era of Red Dwarfs (up to 10^14 yr), black holes are minor compared to long-lived fully convective M-dwarfs and potentially habitable close-in planets. **Target (`vid=2203`):** Fully Convective M-Dwarf System (vid=2203): remove premature Kerr BH; place large turbulent crimson-orange convective red dwarf at center with tight orbiting tidally locked planet.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **combustión lenta** (*slow burning*) | `[TIER-C]` | Su baja masa y mezcla interna permiten aprovechar el hidrógeno durante tiempos enormemente largos. / *Their low mass and internal mixing let them use hydrogen for enormously long periods.* | interior boiling convection cells looping to core |
| `F1` | **luz tenue** (*dim light*) | `[TIER-C]` | El cielo pierde brillo global aunque sistemas individuales continúen emitiendo una luz roja estable. / *The sky's overall brightness fades even though individual systems continue to emit steady red light.* | dim deep-red sky backdrop |
| `F2` | **últimos planetas cálidos** (*last warm planets*) | `[TIER-D]` | Algunos entornos planetarios podrían conservar fuentes de energía, pero su habitabilidad remota no puede darse por sentada. / *Some planetary environments might retain energy sources, but their habitability in the distant future cannot be assumed.* | close-orbit twilight terminator planet |
| `F3` | **espacio más aislado** (*increasingly isolated space*) | `[TIER-C]` | Las distancias efectivamente accesibles crecen mientras la actividad luminosa se concentra en menos sistemas ligados. / *The effectively accessible distances grow as the light activity is concentrated in fewer linked systems.* | sparse dimmed background stars |
| `F4` | **ritmo casi inmóvil** (*nearly static pace*) | `[TIER-C]` | La escena cambia tan lentamente que una vida estelar humana resulta inconcebiblemente breve en comparación. / *The scene changes so slowly that a human lifetime is inconceivably brief by comparison.* | ultra-slow glacial rotation gauge |
| `F5` | **sin privilegio biológico** (*without biological privilege*) | `[TIER-C]` | La aplicación no asigna un límite exacto a la vida ni a la computación en esta era. / *The application does not assign an exact limit to life or computing in this era.* | neutral observer halo |

##### `Z04` `last-stars` (`vid=2204`) — Las últimas estrellas (*The last stars*) `[TIER-D]`

- **Catalog Metadata:** `scene=null` · `[0.445, 0.555]` · `rgb=[255, 215, 145]` · `sound=(triangle/64Hz)` · `speak_evidence_default=False` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Orden de 10¹⁴ años · termina la formación estelar ordinaria` / *`On the order of 10¹⁴ years · ordinary star formation ends`* · Evidence: `Proyección robusta` (*Robust projection*)
- **Spoken Macro Narrative (ES / EN):** La última luz estelar no desaparece en un instante. Nacen cada vez menos estrellas; después quedan puntos aislados y raros episodios producidos por encuentros o fusiones de objetos subestelares. (*EN:* The last starlight does not vanish in an instant. Fewer and fewer stars are born; afterward, isolated points of light and rare events caused by encounters or mergers of substellar objects remain.)
- **Current Diagnosis & Target 3D Blueprint:** Kerr BH + crimson ember fails to depict the concrete remnants finishing star formation: brown-dwarf merger sparks, isolated beacon stars in deep darkness, hot cooling white dwarfs, and spinning neutron stars/pulsars. **Target (`vid=2204`):** Twilight Remnant Graveyard (vid=2204): sparse void containing cooling blue-white dwarf, spinning bipolar pulsar, and colliding brown-dwarf pair.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **último nacimiento natural** (*last natural star birth*) | `[TIER-D]` | Llega una época en que las nubes ya no forman estrellas de manera ordinaria. / *There comes a time when clouds no longer form stars in the ordinary way.* | extinguishing dim dust cloud |
| `F1` | **colisiones excepcionales** (*exceptional collisions*) | `[TIER-D]` | Encuentros entre enanas marrones u otros objetos podrían encender estrellas nuevas de forma esporádica. / *Encounters between brown dwarfs or other objects could sporadically ignite new stars.* | twin brown dwarfs spiraling into sudden flare ignition |
| `F2` | **puntos aislados** (*isolated points*) | `[TIER-C]` | Las estrellas supervivientes aparecen como faros solitarios en una oscuridad creciente. / *The surviving stars appear like lonely beacons in a gathering darkness.* | lone beacons against pitch-black background |
| `F3` | **enanas blancas** (*white dwarfs*) | `[TIER-D]` | Núcleos estelares agotados conservan calor residual y se enfrían durante tiempos enormes. / *Depleted stellar cores retain residual heat and cool for enormous periods of time.* | dense crystalline white-dwarf core |
| `F4` | **estrellas de neutrones** (*neutron stars*) | `[TIER-D]` | Remanentes compactos sobreviven después de explosiones estelares y pierden gradualmente su energía accesible. / *Compact remnants survive after stellar explosions and gradually lose their accessible energy.* | magnetized neutron star with faint beam cones |
| `F5` | **materia ordinaria tenue** (*sparse ordinary matter*) | `[TIER-C]` | La mayor parte de la materia bariónica luminosa deja de estar en estrellas activas. / *Most of the luminous baryonic matter ceases to be in active stars.* | fading diffuse gas wisps |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP12-Z00-04)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP12-Z00-04` (Remove `5|12` CPU Bypass, Time-Axis Pivot, Present Cosmos, Stellar Decline & Red Dwarfs) for HyperCosmosVR Odyssey Act V (`Exp 12: 12_future_of_universe`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z00`->`2200` (`F0..F4`), `Z01`->`2201` (`F0..F5`), `Z02`->`2202` (`F0..F5`), `Z03`->`2203` (`F0..F5`), `Z04`->`2204` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3730`, `3731`, `3732`, `3733`, `3734`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP12-Z00-04`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Remove `12` from the `5 | 12 =>` bypass in `crates/cosmos_render/src/doc_stages.rs` (`L657-L691`) so Exp `12` dispatches `kind=9` `vid=2200..=2209` instead of legacy `kind=4 vid=12` (`shaders/hero_doc_cosmos.wgsl`) + dead `exp_id==12u` (`L105-L142`). Confirm `scene` backfill (`0..8` / `0..9`) in `assets/baked/universe_catalog.v1.json` and pair `mode=13` redshift particles in `shaders/compute_universe.wgsl` cleanly with each era.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z00: handoff / Frontier of the knowable, Z01: present / Present-day universe, Z02: decline / Twilight of star formation, Z03: red-dwarfs / Era of red dwarfs, Z04: last-stars / The last stars.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp12-z05-09"></a>
### PKG-ACT5-EXP12-Z05-09: Degenerate Black Dwarfs, Black Hole Era, Dynamic Hawking Evaporation, Dark Era & Fate Cone

- **Phase:** Phase 1/2 | **Priority:** `P2` | **Scope:** Exp `12` (`12_future_of_universe`) `Z05..Z09` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING` , `PKG-P0-CATALOG-SCENE-AND-CI` (backfill `scene`)
- **Current State:** **CPU BYPASS BUG** `doc_stages.rs#L657-L691` (`5 | 12 =>`) skips `doc_stage_variant_id(12)` (`120+(sc%5)` at `L63`) & renders hardcoded Kerr BH + crimson dwarf on all 10 zones!; rendered by `shaders/hero_doc_cosmos.wgsl#L411-L505` (`kind=4, vid=12`, up to `50` steps!) + `#L204-L229` (`kind=1, vid=12`). Dead WGSL: `hero_doc_civilization_horizon.wgsl#L105-L142`. Tier breakdown for this batch: **`0A / 3B / 2C / 0D` Zones**, **`0A / 9B / 8C / 13D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `2/5` zones and `4/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 2205..=2209` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3735`, `3736`, `3737`, `3738`, `3739` (allocated from `3720 + (12 - 11)*10 + k`; Thermal Cooling -> Hawking Quantum Pair Jets -> Near-Vacuum Equilibrium):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z05` | `degenerate` (*Era degenerada* / *Degenerate Era*) | `[TIER-B]` | Bypassed `kind=4+1, vid=12` | **`2205`** | `3735` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x03` (0x01 cutaway, 0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z06` | `black-holes` (*Era de los agujeros negros* / *Black Hole Era*) | `[TIER-B]` | Bypassed `kind=4+1, vid=12` | **`2206`** | `3736` | `0` default (`1` Hawking/GW cutaway, `2` thermal-pair sweep) | `0x06` (0x04 semantic cue mask, 0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z07` | `hawking` (*Evaporación de Hawking* / *Hawking evaporation*) | `[TIER-B]` | Bypassed `kind=4+1, vid=12` | **`2207`** | `3737` | `0` default (`1` Hawking/GW cutaway, `2` thermal-pair sweep) | `0x07` (0x04 semantic cue mask, 0x01 cutaway, 0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z08` | `dark-era` (*Era oscura* / *Dark Era*) | `[TIER-C]` | Bypassed `kind=4+1, vid=12` | **`2208`** | `3738` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z09` | `unknown` (*Futuro indeterminado* / *Undetermined future*) | `[TIER-C]` | Bypassed `kind=4+1, vid=12` | **`2209`** | `3739` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z05` `degenerate` (`vid=2205`) — Era degenerada (*Degenerate Era*) `[TIER-B]`

- **Catalog Metadata:** `scene=null` · `[0.555, 0.695]` · `rgb=[179, 205, 232]` · `sound=(sine/54Hz)` · `speak_evidence_default=False` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Después de ~10¹⁴ años · remanentes fríos y física no confirmada` / *`After ~10¹⁴ years · cold remnants and unconfirmed physics`* · Evidence: `Escenario condicional` (*Conditional scenario*)
- **Spoken Macro Narrative (ES / EN):** Predominan enanas blancas, estrellas de neutrones, enanas marrones, planetas congelados y objetos compactos. Su destino depende de procesos extremadamente lentos y de si partículas como el protón son realmente estables. (*EN:* White dwarfs, neutron stars, brown dwarfs, frozen planets and compact objects predominate. Their fate depends on extremely slow processes and whether particles like the proton are truly stable.)
- **Current Diagnosis & Target 3D Blueprint:** Companion kind=1, vid=12 (`cooling dwarf ember`) matches F00 black dwarfs / degenerate remnants at the archetype level, but giant Kerr BH is premature (dominant in Z06+) and `doc_feat` is completely unread — missing proton decay, rogue frozen planets, rare gravitational ejections, and pycnonuclear quantum tunneling. **Target (`vid=2205`):** Degenerate Era Remnant Lab (vid=2205): center the cooling black-dwarf crystalline sphere & suppress dominant Kerr BH.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **enanas negras** (*black dwarfs*) | `[TIER-B]` | Una enana blanca suficientemente enfriada sería casi invisible; ninguna puede existir todavía porque el universo no es lo bastante antiguo. / *A sufficiently cooled white dwarf would be almost invisible; none can exist yet because the universe is not old enough.* | white-to-black infrared cooling sequence |
| `F1` | **protones: pregunta abierta** (*protons: open question*) | `[TIER-D]` | El decaimiento del protón no se ha observado. Si ocurre, los remanentes bariónicos evolucionan de una forma; si no, pueden persistir muchísimo más. / *Proton decay has not been observed. If it occurs, baryonic remnants evolve one way; if not, they can persist much longer.* | shimmering baryon crystal lattice flashing into pions/positrons vs eternal solid (proton stability toggle) |
| `F2` | **planetas congelados** (*frozen planets*) | `[TIER-D]` | Mundos expulsados o alejados de fuentes estelares pierden calor y vagan como cuerpos oscuros. / *Worlds ejected or far from stellar sources lose heat and wander as dark bodies.* | dark ice-crusted rogue planet drifting in void |
| `F3` | **encuentros rarísimos** (*exceedingly rare encounters*) | `[TIER-D]` | La gravedad puede expulsar, capturar o fusionar objetos, pero los intervalos entre sucesos crecen enormemente. / *Gravity can eject, capture, or merge objects, but the intervals between events grow enormously.* | multi-body slingshot hyperbolic ejection arc |
| `F4` | **túnel y reacciones lentas** (*quantum tunneling and slow reactions*) | `[TIER-D]` | Algunos modelos permiten transformaciones nucleares o cuánticas a escalas de tiempo extremas, sin una cronología observada. / *Some models allow nuclear or quantum transformations at extreme time scales, without observed chronology.* | quantum tunneling barrier cross-section inside cold core |
| `F5` | **dos rutas** (*two routes*) | `[TIER-C]` | La estabilidad o inestabilidad de la materia ordinaria divide el futuro en escenarios físicamente distintos. / *The stability or instability of ordinary matter divides the future into physically distinct scenarios.* | bifurcating proton-decay path |

##### `Z06` `black-holes` (`vid=2206`) — Era de los agujeros negros (*Black Hole Era*) `[TIER-B]`

- **Catalog Metadata:** `scene=null` · `[0.695, 0.810]` · `rgb=[142, 126, 226]` · `sound=(sine/44Hz)` · `speak_evidence_default=False` · `DOMAIN_CUE_KEYWORDS`: `hawking`
- **Metric & Evidence (ES / EN):** `Remoto · los agujeros negros concentran la energía restante` / *`Distant future · black holes concentrate the remaining energy`* · Evidence: `Modelo teórico` (*Theoretical model*)
- **Spoken Macro Narrative (ES / EN):** Cuando casi toda luz estelar ha terminado, los agujeros negros pueden ser algunos de los últimos depósitos compactos de masa y energía. Aun así, no deben representarse como eternos. (*EN:* When almost all starlight has ended, black holes may be some of the last compact reservoirs of mass and energy. Even so, they should not be represented as eternal.)
- **Current Diagnosis & Target 3D Blueprint:** kind=4, vid=12 (`is_degenerate` Kerr BH) directly depicts the Black Hole Era (F00, F01, F04, F05), but ignores `doc_feat`, keeps the unrelated companion cooling dwarf ember on stage, and lacks binary BH inspiral (`F02`) or CMB-vs-horizon temperature comparison (`F03`). **Target (`vid=2206`):** Black Hole Dominion Stage (vid=2206 / upgrade kind=4): hide companion crimson dwarf.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **agujeros negros estelares** (*stellar-mass black holes*) | `[TIER-B]` | Los remanentes de estrellas masivas persisten mucho después de que sus galaxias dejen de brillar. / *Remnants of massive stars persist long after their galaxies stop shining.* | compact stellar BH swarm across void |
| `F1` | **agujeros negros supermasivos** (*supermassive black holes*) | `[TIER-B]` | Los objetos centrales de antiguas galaxias dominan escalas enormes y pueden tardar muchísimo más en evaporarse. / *The central objects of ancient galaxies dominate enormous scales and can take much longer to evaporate.* | massive central AGN-scale dark shadow ring |
| `F2` | **fusiones aisladas** (*isolated mergers*) | `[TIER-D]` | Sistemas ligados pueden producir encuentros tardíos, aunque cada vez quedan menos objetos próximos. / *Bound systems can produce late encounters, although fewer and fewer nearby objects remain.* | binary black-hole inspiral with outgoing quadruple GW spiral ripples |
| `F3` | **radiación mínima** (*minimal radiation*) `[kw:hawking]` | `[TIER-D]` | Los agujeros negros astrofísicos grandes serían hoy más fríos que su entorno y su emisión de Hawking sería extraordinariamente débil. / *Large astrophysical black holes today would be colder than their surroundings and their Hawking emission would be extraordinarily weak.* | thermal comparison showing tiny background photons warmer than cold BH horizon |
| `F4` | **energía concentrada** (*concentrated energy*) | `[TIER-B]` | La mayor parte de la actividad disponible puede quedar encerrada en horizontes y liberarse sólo muy lentamente. / *Most of the available activity can be locked in horizons and released only very slowly.* | mass-energy E=mc^2 accretion ring |
| `F5` | **no son eternos** (*they are not eternal*) `[kw:hawking]` | `[TIER-B]` | La teoría cuántica de campos en espacio-tiempo curvo predice que pierden masa mediante radiación de Hawking. / *Quantum field theory in curved spacetime predicts that they lose mass through Hawking radiation.* | faint quantum pair wisps teasing evaporation |

##### `Z07` `hawking` (`vid=2207`) — Evaporación de Hawking (*Hawking evaporation*) `[TIER-B]`

- **Catalog Metadata:** `scene=null` · `[0.810, 0.895]` · `rgb=[214, 173, 255]` · `sound=(triangle/50Hz)` · `speak_evidence_default=False` · `DOMAIN_CUE_KEYWORDS`: `hawking`
- **Metric & Evidence (ES / EN):** `Hasta ~10¹⁰⁰ años para los mayores · no observada directamente en agujeros negros astrofísicos` / *`Up to ~10¹⁰⁰ years for the largest · not directly observed in astrophysical black holes`* · Evidence: `Predicción teórica fuerte` (*Strong theoretical prediction*)
- **Spoken Macro Narrative (ES / EN):** Según la predicción de Hawking, un agujero negro aislado pierde masa, se calienta al hacerse más pequeño y termina con una fase final rápida. La conclusión es teóricamente sólida, pero no se ha medido directamente en un agujero negro astrofísico. (*EN:* According to Hawking's prediction, an isolated black hole loses mass, heats up as it becomes smaller, and ends with a rapid final phase. The conclusion is theoretically sound, but has not been directly measured in an astrophysical black hole.)
- **Current Diagnosis & Target 3D Blueprint:** kind=4, vid=12 literally renders polar Hawking pair-creation jets & violet photon ring matching Hawking evaporation (F00..F03), yet ignores `doc_feat`, never shrinks horizon radius or shifts temperature upward (`T ~ 1/M`), and omits analog gravity lab experiments (`F04`). **Target (`vid=2207`):** Dynamic Hawking Evaporation Chrono-Stage (vid=2207 / upgrade kind=4): drop companion dwarf; parameterize horizon radius r_s(feat), accretion blackbody kelvin T(feat) (crimson 1400K -> blinding blue-white gamma), and pair-jet flux.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **pérdida lentísima** (*very slow loss*) | `[TIER-B]` | Durante casi toda su vida, un agujero negro grande cambia de forma imperceptible. / *For almost its entire life, a large black hole changes imperceptibly.* | near-static massive BH tick |
| `F1` | **más pequeño, más caliente** (*smaller, hotter*) `[kw:hawking]` | `[TIER-B]` | La temperatura de Hawking aumenta cuando disminuye la masa del agujero negro. / *The Hawking temperature increases as the mass of the black hole decreases.* | shrinking & heating horizon |
| `F2` | **radiación creciente** (*increasing radiation*) | `[TIER-B]` | La emisión se intensifica al acercarse la fase final, dentro del marco teórico estándar. / *The emission intensifies as the final phase approaches, within the standard theoretical framework.* | dense Hawking pair cascade |
| `F3` | **final breve** (*short ending*) | `[TIER-B]` | Después de una espera descomunal, la última etapa sería rápida comparada con la vida total del agujero negro. / *After an enormous wait, the last stage would be fast compared to the total life of the black hole.* | final runaway micro-BH flash burst |
| `F4` | **sin detección directa** (*no direct detection*) `[kw:hawking]` | `[TIER-D]` | Experimentos análogos estudian fenómenos relacionados, pero no sustituyen una detección astrofísica de Hawking. / *Analog experiments study related phenomena, but do not replace an astrophysical detection of Hawking radiation.* | laboratory sonic-waterfall analog horizon cross-section |
| `F5` | **gravedad cuántica pendiente** (*unresolved quantum gravity*) | `[TIER-C]` | El detalle último de la evaporación y la información sigue conectado a preguntas abiertas de gravedad cuántica. / *The ultimate details of evaporation and information loss remain tied to open questions in quantum gravity.* | Page-curve entangled string arc |

##### `Z08` `dark-era` (`vid=2208`) — Era oscura (*Dark Era*) `[TIER-C]`

- **Catalog Metadata:** `scene=null` · `[0.895, 0.965]` · `rgb=[129, 161, 213]` · `sound=(sine/34Hz)` · `speak_evidence_default=False` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Tras los últimos agujeros negros · expansión, partículas y campos residuales` / *`After the last black holes · expansion, particles and residual fields`* · Evidence: `Futuro condicional` (*Conditional Future*)
- **Spoken Macro Narrative (ES / EN):** Si la expansión continúa y los agujeros negros se evaporan como se espera, quedarían radiación de energía bajísima, neutrinos y partículas estables dispersas en un espacio cada vez más diluido. (*EN:* If the expansion continues and the black holes evaporate as expected, very low energy radiation, neutrinos and stable particles would remain scattered in an increasingly dilute space.)
- **Current Diagnosis & Target 3D Blueprint:** Active glowing Kerr BH + hot cooling dwarf ember actively contradicts 'After the last black holes' (Dark Era heat-death equilibrium). Only stretched ultra-cold photons, neutrinos, sparse e-/e+, dark matter, and subtle quantum fluctuations should inhabit the scene. **Target (`vid=2208`):** Near-Vacuum Equilibrium Canvas (vid=2208): switch compute mode 13 off or to ultra-slow brownian drift; dark void devoid of black holes/stars.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **fotones fríos** (*cold photons*) | `[TIER-D]` | La longitud de onda de la radiación aumenta con la expansión y su energía accesible disminuye. / *The wavelength of the radiation increases with expansion and its accessible energy decreases.* | vastly stretched ultra-long red/radio sinusoidal waves fading below visibility |
| `F1` | **neutrinos relictos** (*relic neutrinos*) | `[TIER-D]` | Neutrinos extremadamente diluidos pueden permanecer como parte del fondo cósmico remoto. / *Extremely dilute neutrinos may remain part of the remote cosmic background.* | ghostly translucent neutrino grid passing unimpeded |
| `F2` | **electrones y positrones** (*electrons and positrons*) | `[TIER-D]` | Su presencia final depende de la estabilidad de la materia y de procesos de aniquilación o captura. / *Their eventual presence depends on the stability of matter and on annihilation or capture processes.* | mega-AU wide orbiting positronium (e-/e+) bound pair slowly spiraling inward |
| `F3` | **materia oscura estable** (*stable dark matter*) | `[TIER-C]` | Si las partículas de materia oscura son estables, podrían persistir; aún no conocemos su identidad microscópica. / *If dark matter particles are stable, they could persist; we still do not know their microscopic identity.* | faint WIMP/dark-matter halo whisper |
| `F4` | **interacciones raras** (*rare interactions*) | `[TIER-D]` | El universo no se vuelve una pantalla vacía: quedan campos, fluctuaciones y sucesos de probabilidad diminuta. / *The universe does not become an empty screen: fields, fluctuations, and extremely low-probability events remain.* | rare brief virtual-pair vacuum shimmer |
| `F5` | **equilibrio aproximado** (*approximate equilibrium*) | `[TIER-C]` | La muerte térmica describe la pérdida gradual de energía libre utilizable, no un apagado súbito de toda física. / *Heat death describes the gradual loss of usable free energy, not a sudden shutdown of all physical processes.* | flat near-zero free-energy gradient gauge |

##### `Z09` `unknown` (`vid=2209`) — Futuro indeterminado (*Undetermined future*) `[TIER-C]`

- **Catalog Metadata:** `scene=null` · `[0.965, 1.001]` · `rgb=[228, 231, 255]` · `sound=(sine/30Hz)` · `speak_evidence_default=False` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]`
- **Metric & Evidence (ES / EN):** `Sin una cronología única · las posibilidades se ramifican` / *`No single timeline · the possibilities branch`* · Evidence: `Física desconocida` (*unknown physics*)
- **Spoken Macro Narrative (ES / EN):** El futuro más lejano no es una película ya conocida. Es una extrapolación basada en las leyes que podemos medir hoy, y esas leyes pueden resultar incompletas en escalas nunca observadas. (*EN:* The far future is not a movie whose ending is already known. It is an extrapolation based on the laws we can measure today, and those laws may prove incomplete on scales never observed.)
- **Current Diagnosis & Target 3D Blueprint:** Hero WGSL implemented `exp_id == 12u, mode == 4u` Time-Arrow & Big-Rip Cone (`hero_doc_civilization_horizon.wgsl:132-142`), which is dead code behind `5 \| 12 =>`! Screen renders static Kerr BH + ember instead of branching cosmological fates (eternal expansion, evolving w(z) dark energy, false-vacuum nucleation bubble, Big Rip). **Target (`vid=2209`):** Activate & Upgrade WGSL Endgame Fate Cone (vid=2209 / mode=4).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **expansión eterna** (*eternal expansion*) | `[TIER-C]` | Una energía oscura aproximadamente constante conduce hacia expansión continuada y equilibrio térmico, pero su constancia eterna no está demostrada. / *Approximately constant dark energy leads to continued expansion and thermal equilibrium, but its eternal constancy is not proven.* | smooth flat de Sitter asymptotic cone |
| `F1` | **energía oscura cambiante** (*evolving dark energy*) | `[TIER-C]` | Datos futuros podrían mostrar que la componente responsable de la aceleración evoluciona y cambia el destino global. / *Future data could show that the component responsible for acceleration evolves and changes the universe's overall fate.* | oscillating quintessence/evolving w(t) lattice |
| `F2` | **vacío metaestable** (*metastable vacuum*) | `[TIER-D]` | Algunos modelos permiten una transición del vacío, pero no existe evidencia de que ocurra ni un calendario verificable. / *Some models allow a vacuum transition, but there is no evidence that one will occur and no testable timetable.* | sudden true-vacuum nucleation bubble expanding at c inside metastable field |
| `F3` | **materia estable o inestable** (*stable or unstable matter*) | `[TIER-C]` | La estabilidad del protón y de otras partículas determina qué estructuras pueden sobrevivir. / *The stability of the proton and other particles determines which structures can survive.* | proton stability vs dissolution split screen |
| `F4` | **Big Rip o colapso** (*Big Rip or collapse*) | `[TIER-D]` | Estos finales requieren comportamientos cosmológicos específicos; no son destinos confirmados. / *These endings require specific cosmological behavior; they are not confirmed outcomes.* | diverging hyperbolic Big Rip tearing bound galaxy/atom rings vs Big Crunch re-collapse |
| `F5` | **leyes aún desconocidas** (*laws still unknown*) | `[TIER-C]` | Nueva física puede modificar por completo las proyecciones realizadas desde el presente. / *New physics can completely modify the projections made from the present.* | open misty horizon handing off to Exp 13/future science |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP12-Z05-09)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP12-Z05-09` (Degenerate Black Dwarfs, Black Hole Era, Dynamic Hawking Evaporation, Dark Era & Fate Cone) for HyperCosmosVR Odyssey Act V (`Exp 12: 12_future_of_universe`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z05`->`2205` (`F0..F5`), `Z06`->`2206` (`F0..F5`), `Z07`->`2207` (`F0..F5`), `Z08`->`2208` (`F0..F5`), `Z09`->`2209` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3735`, `3736`, `3737`, `3738`, `3739`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP12-Z05-09`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`. Remove `12` from the `5 | 12 =>` bypass in `crates/cosmos_render/src/doc_stages.rs` (`L657-L691`) so Exp `12` dispatches `kind=9` `vid=2200..=2209` instead of legacy `kind=4 vid=12` (`shaders/hero_doc_cosmos.wgsl`) + dead `exp_id==12u` (`L105-L142`). Confirm `scene` backfill (`0..8` / `0..9`) in `assets/baked/universe_catalog.v1.json` and pair `mode=13` redshift particles in `shaders/compute_universe.wgsl` cleanly with each era.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z05: degenerate / Degenerate Era, Z06: black-holes / Black Hole Era, Z07: hawking / Hawking evaporation, Z08: dark-era / Dark Era, Z09: unknown / Undetermined future.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

### 4.6 Experience 23: `23_cosmic_perspectives` — Espectro de las Perspectivas Cósmicas: de una Visión Humana a Muchas Formas de Comprender el Universo

- **Odyssey & Spatial Placement:** Act 5 (`6/8`, overall stop #23) · `HYPER_LOTUS_23_PERSPECTIVES` (`parent UNIVERSE_ROOT`, `u = [27.2, 28.6]`, `focus_scale_exp = 27.6`, `6.0e27 m`) ([`topology.rs#L364-L375, L655`](../../crates/cosmos_spatial/src/topology.rs#L364-L375))
- **Experience Totals:** `25` Zones (`0A / 4B / 8C / 13D`) · `150` Features (`0A / 11B / 60C / 79D`)
- **Current Shader & Compute:** `doc_stages.rs#L83`: `23 => 230` (single variant across all 25 zones!) · `shaders/hero_doc_civilization_horizon.wgsl#L430-L463` (front-locked 6-Lens Sensory Mandala; `active_lens = (scene+feat+zone)%6` brightens 1 hex lens) · `mode=15` (`universe_scene.rs#L323`, `compute_universe.wgsl#L347-L357` 25-petal lotus) + `far_shell.wgsl#L150-L161` (`lotus_boost=0.42`)

<a id="pkg-act5-exp23-z00-04"></a>
### PKG-ACT5-EXP23-Z00-04: Split `vid=230`: Dual Perspectives, Embodied Human Frame, Terrestrial Senses & Language Maps

- **Phase:** Phase 1 | **Priority:** `P1` | **Scope:** Exp `23` (`23_cosmic_perspectives`) `Z00..Z04` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L83`: `23 => 230` (single variant across all 25 zones!); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L430-L463` (front-locked 6-Lens Sensory Mandala; `active_lens = (scene+feat+zone)%6` brightens 1 hex lens). Tier breakdown for this batch: **`0A / 2B / 1C / 2D` Zones**, **`0A / 3B / 12C / 15D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `1/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3300..=3304` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3840`, `3841` (allocated from `3720 + (23 - 11)*10 + k`; `mode=15` Calibrated Sensory Lotus (replacing identical front-locked mandala)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z00` | `another-perspective` (*Otra perspectiva sobre la realidad* / *Another perspective on reality*) | `[TIER-B]` | `vid=230` (`scene=0`) | **`3300`** | `3840` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z01` | `human-perspective` (*Perspectiva humana* / *Human perspective*) | `[TIER-D]` | `vid=230` (`scene=1`) | **`3301`** | `3841` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x06` (0x04 semantic cue mask, 0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z02` | `terrestrial-senses` (*Sentidos terrestres* / *Earthly senses*) | `[TIER-B]` | `vid=230` (`scene=2`) | **`3302`** | `3840` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z03` | `brain-perception` (*Cerebro y percepción* / *Brain and perception*) | `[TIER-D]` | `vid=230` (`scene=3`) | **`3303`** | `3841` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z04` | `language` (*Lenguaje* / *Language*) | `[TIER-C]` | `vid=230` (`scene=4`) | **`3304`** | `3840` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z00` `another-perspective` (`vid=3300`) — Otra perspectiva sobre la realidad (*Another perspective on reality*) `[TIER-B]`

- **Catalog Metadata:** `scene=0` · `[0.000, 0.040]` · `rgb=[190, 220, 255]` · `sound=(sine/48Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *EL MENSAJE SE ABRE EN VARIAS FORMAS DE REALIDAD*
- **Metric & Evidence (ES / EN):** `Contacto logrado → diferencia conceptual reconocida → nuestra propia visión se vuelve visible` / *`Contact achieved → conceptual difference recognized → our own vision becomes visible`* · Evidence: `Punto de partida · el mensaje revela una mente situada` (*Starting point · the message reveals a situated mind*)
- **Spoken Macro Narrative (ES / EN):** Reconocer otra inteligencia también significa descubrir que la perspectiva humana es una perspectiva entre otras, no un punto de vista neutral. (*EN:* Recognizing other intelligence also means discovering that the human perspective is one perspective among others, not a neutral point of view.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` front-locked 6-lens rainbow hex mandala converging on center_eye dot. active_lens=(0+feat+0)%6 highlights lenses k=0..5 uniformly; no dual civilizational maps or map revision. **Target (`vid=3300`):** Keep central radiant reference star (`sd_sphere` + corona flares) flanked by TWO opposing holographic hemisphere wireframes/taxonomic constellation overlays (Human warm amber `zone_col` vs Alien cyan-magenta).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **mismo fenómeno** (*same phenomenon*) | `[TIER-B]` | Dos civilizaciones pueden señalar la misma estrella y, aun así, dividir sus propiedades en categorías muy diferentes. / *Two civilizations can point to the same star and still divide its properties into very different categories.* | **Target:** project dual overlapping Voronoi partitions onto the star *(Gap: Center star & k=0 lens glow pulse, but only 1 generic rainbow frame (no dual taxonomy overlay).)* |
| `F1` | **símbolo recibido** (*received symbol*) | `[TIER-C]` | Un símbolo empieza a tener significado cuando puede relacionarse con observaciones, acciones y consecuencias compartidas. / *A symbol begins to have meaning when it can be related to shared observations, actions and consequences.* | **Target:** ignite bridge geodesic packet linking observer->star->symbol glyph *(Gap: Lens k=1 ring highlights via modulo, zero shared observation-action grounding bridge.)* |
| `F2` | **modelo ajeno** (*foreign model*) | `[TIER-C]` | Comprender un modelo extranjero exige reconstruir qué distingue, qué agrupa y qué considera explicativo. / *Understanding a foreign model requires reconstructing what it distinguishes, what it groups together, and what it considers explanatory.* | **Target:** morph alien cluster boundaries *(Gap: Lens k=2 dot highlights; no foreign grouping cluster vs human taxonomy contrast.)* |
| `F3` | **sorpresa humana** (*human surprise*) | `[TIER-C]` | Lo más revelador puede no ser lo que la otra mente sabe, sino aquello que nunca necesitó conceptualizar como nosotros. / *The most revealing may not be what the other mind knows, but what it never needed to conceptualize as we do.* | **Target:** unmask hidden IR spectral band invisible to left hemisphere *(Gap: Lens k=3 stripe phase shifts by +f32(3); no hidden-variable blind spot revealed.)* |
| `F4` | **sorpresa alienígena** (*alien surprise*) | `[TIER-C]` | Nuestras distinciones también podrían exponer regularidades que la otra civilización había pasado por alto. / *Our distinctions could also expose regularities that the other civilization had overlooked.* | **Target:** unmask polarized/magnetic sub-lattice invisible to right *(Gap: Lens k=4 dot-grid highlights; no reciprocal alien blind-spot illumination.)* |
| `F5` | **cambio mutuo** (*mutual change*) | `[TIER-C]` | El contacto profundo no deja intacta a ninguna perspectiva: ambas revisan sus mapas de la realidad. / *Deep contact leaves neither perspective intact: both revise their maps of reality.* | **Target:** smoothly cross-blend both hemispheres into an expanded shared atlas pulsed by `doc_cue` *(Gap: Lens k=5 square grid highlights; both maps stay rigid instead of morphing/merging.)* |

##### `Z01` `human-perspective` (`vid=3301`) — Perspectiva humana (*Human perspective*) `[TIER-D]`

- **Catalog Metadata:** `scene=1` · `[0.040, 0.080]` · `rgb=[120, 200, 255]` · `sound=(triangle/51.55Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `horizonte` · Footer: *LO EVIDENTE TAMBIÉN TIENE HISTORIA BIOLÓGICA*
- **Metric & Evidence (ES / EN):** `Cuerpo + entorno + historia + lenguaje → mundo humano aparentemente evidente` / *`Body + environment + history + language → an apparently self-evident human world`* · Evidence: `Punto de vista situado · categorías nacidas de cuerpos terrestres` (*Situated point of view · categories born from terrestrial bodies*)
- **Spoken Macro Narrative (ES / EN):** Objetos, individuos, pasado, futuro y causa parecen categorías naturales, pero nuestra forma de organizarlas depende de cómo vivimos y percibimos. (*EN:* Objects, individuals, past, future, and cause seem like natural categories, but how we organize them depends on how we live and perceive.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` flat symmetrical 6-hex mandala; zero ground plane, horizon, bounded solid objects, causal timeline, or body/mind boundary. (`F1` hits `kw:horizonte` in resolver only.) **Target (`vid=3301`):** Render an upright terrestrial horizon disk (`sd_capped_cylinder` + grid) beneath a silhouetted bipedal sensor-frame and floating physical field.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **objetos** (*objects*) | `[TIER-D]` | La visión humana separa superficies persistentes y favorece pensar el mundo como conjuntos de cosas delimitadas. / *Human vision separates persistent surfaces and favors thinking of the world as sets of delimited things.* | **Target:** crystallizes continuous density field into discrete sharp-edged geometric solids (`sd_box`/`sd_sphere` crisp isolines) *(Gap: No delimited solid boundary SDFs separating persistent objects from field background.)* |
| `F1` | **espacio** (*space*) `[kw:horizonte]` | `[TIER-D]` | Nuestro movimiento sobre una superficie y bajo gravedad moldea intuiciones como arriba, abajo, cerca y horizonte. / *Our movement on a surface and under gravity shapes intuitions such as up, down, near, and horizon.* | **Target:** projects gravitational vertical axis (`up/down` plumb line) & horizon rim glow *(Gap: Face-on symmetrical hex lacks ground plane, vertical gravity vector, or horizon (`kw:horizonte` unused on GPU).)* |
| `F2` | **tiempo** (*time*) | `[TIER-D]` | Recordamos el pasado, anticipamos el futuro y experimentamos un presente estrecho, lo que favorece relatos lineales. / *We remember the past, anticipate the future, and experience a narrow present, which favors linear narratives.* | **Target:** extrudes a luminous linear worldline ribbon with trailing memory fade and narrow bright 'present' ring *(Gap: No past-present-future arrow or narrow glowing 'now' cursor along a worldline.)* |
| `F3` | **causalidad** (*causality*) | `[TIER-D]` | Buscamos acontecimientos anteriores que expliquen cambios posteriores, aunque muchas causas formen redes y no cadenas simples. / *We look for previous events that explain later changes, although many causes form networks and not simple chains.* | **Target:** fans out a directed causal network graph splitting from one chain *(Gap: No branching causal DAG contrast against a single 1D chain.)* |
| `F4` | **identidad** (*identity*) | `[TIER-D]` | Un cuerpo relativamente continuo favorece la idea de un yo individual que permanece a través del cambio. / *A relatively continuous body favors the idea of an individual self that remains through change.* | **Target:** pulses a persistent bodily envelope while ambient particles stream through *(Gap: No continuous organic vessel/silhouette maintaining form over time-flow.)* |
| `F5` | **mente y materia** (*mind and matter*) | `[TIER-C]` | Separar experiencia interna y mundo externo es útil, pero no garantiza que esa división sea fundamental. / *Separating internal experience and external world is useful, but it does not guarantee that this division is fundamental.* | **Target:** draws a semi-transparent observer sphere dividing inner glowing neural core from outer universe *(Gap: No inside/outside membrane boundary test separating subjective core from world.)* |

##### `Z02` `terrestrial-senses` (`vid=3302`) — Sentidos terrestres (*Earthly senses*) `[TIER-B]`

- **Catalog Metadata:** `scene=2` · `[0.080, 0.120]` · `rgb=[80, 225, 235]` · `sound=(sine/55.1Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *LOS SENTIDOS MUESTREAN · NO COPIAN EL UNIVERSO*
- **Metric & Evidence (ES / EN):** `Propiedades físicas → receptores biológicos → señales nerviosas → experiencia consciente` / *`Physical properties → biological receptors → nervous signals → conscious experience`* · Evidence: `Ventana biológica · una fracción pequeña de la información disponible` (*Biological window · a small fraction of the available information*)
- **Spoken Macro Narrative (ES / EN):** Los sentidos no copian el universo: seleccionan las variables que ayudaron a organismos terrestres a sobrevivir y actuar. (*EN:* The senses do not copy the universe: they select the variables that helped terrestrial organisms to survive and act.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` literally implements '6-Lens Sensory Mandala' (6 hex frames = 6 senses), matching zone topic, but active_lens=(2+feat+2)%6 misaligns F0->k=4 (dot grid) instead of spectral wave, and k=2..5 use generic dots/stripes/grid. **Target (`vid=3302`):** Fix `active_lens = feat % 6u` directly (no `+scene+zone` scrambling) and replace generic `k=0..5` fills inside each hex lens with true sensory procedural SDFs.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **luz visible** (*visible light*) | `[TIER-B]` | Los ojos humanos detectan una banda estrecha de radiación electromagnética y la convierten en color. / *Human eyes detect a narrow band of electromagnetic radiation and convert it into color.* | **Target:** visible EM band slit inside wide invisible spectrum bar (spectral rainbow sliver) *(Gap: F0 visible light maps modulo-shifted to k=4 tiled dots instead of EM spectrum/rainbow band.)* |
| `F1` | **sonido** (*sound*) | `[TIER-B]` | Oímos solo parte de las vibraciones que atraviesan el aire; otros medios y frecuencias quedan fuera de nuestra experiencia directa. / *We hear only part of the vibrations passing through the air; Other media and frequencies are outside our direct experience.* | **Target:** acoustic compression wavefront rings (`sin(r*k - time)`) *(Gap: F1 sound maps to k=5 grid; concentric wave pattern sits idle on k=0 instead.)* |
| `F2` | **química** (*chemistry*) | `[TIER-D]` | Olfato y gusto agrupan moléculas según receptores y necesidades biológicas, no según una clasificación completa de la química. / *Smell and taste group molecules according to receptors and biological needs, not according to a complete classification of chemistry.* | **Target:** 3D molecular lock-and-key binding pockets (`sd_sphere` clusters snapping into receptor sockets) *(Gap: F2 chemistry gets k=0 waves; lacks lock-and-key molecular docking glyphs.)* |
| `F3` | **tacto y temperatura** (*touch and temperature*) | `[TIER-D]` | La piel integra presión, vibración, daño y transferencia térmica en experiencias corporales distintas. / *The skin integrates pressure, vibration, damage, and thermal transfer into distinct bodily experiences.* | **Target:** deformable membrane surface with orange-to-blue thermal conduction field *(Gap: F3 touch/temp gets k=1 ring; lacks skin pressure dimple & thermal gradient heatmap.)* |
| `F4` | **equilibrio y postura** (*balance and posture*) | `[TIER-D]` | El oído interno y la propiocepción construyen una referencia corporal continua que rara vez notamos hasta que falla. / *The inner ear and proprioception construct a continuous body reference that we rarely notice until it fails.* | **Target:** 3 orthogonal glowing semicircular torus canals (`sd_torus` x3) with tilting fluid bead *(Gap: F4 balance/posture gets k=2 dot; lacks 3 orthogonal inner-ear semicircular vestibular canals.)* |
| `F5` | **duración humana** (*human timescale*) | `[TIER-D]` | Nuestros sentidos integran milisegundos y segundos; procesos demasiado rápidos o lentos parecen simultáneos o inmóviles. / *Our senses integrate milliseconds and seconds; Processes that are too fast or slow appear simultaneous or immobile.* | **Target:** stroboscopic integration shutter ring blurring ultra-fast spins and freezing ultra-slow drift *(Gap: F5 human timescale gets k=3 stripes; lacks fast/slow shutter temporal blur window.)* |

##### `Z03` `brain-perception` (`vid=3303`) — Cerebro y percepción (*Brain and perception*) `[TIER-D]`

- **Catalog Metadata:** `scene=3` · `[0.120, 0.160]` · `rgb=[95, 235, 190]` · `sound=(sawtooth/58.65Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *PERCIBIR = DATOS + MODELO + CORRECCIÓN*
- **Metric & Evidence (ES / EN):** `Señal ambigua + memoria + expectativa + atención → mejor hipótesis disponible` / *`Ambiguous signal + memory + expectation + attention → best available hypothesis`* · Evidence: `Construcción activa · percibir es inferir con información incompleta` (*Active construction · perceiving is inferring with incomplete information*)
- **Spoken Macro Narrative (ES / EN):** La percepción combina datos sensoriales con modelos internos; por eso puede ser estable y útil sin ser una reproducción literal. (*EN:* Perception combines sensory data with internal models; That is why it can be stable and useful without being a literal reproduction.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` static 6-hex mandala; zero ascending sensory stream, descending top-down prediction cascade, attentional spotlight cone, or ambiguous-illusion bistable geometry. **Target (`vid=3303`):** Build a two-tier volumetric cortical loop: lower noisy point-cloud/sensory ribbon feeding upward into an upper sculpted prior manifold (`zone_col`).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **selección** (*selection*) | `[TIER-D]` | La atención amplifica una parte del flujo sensorial y deja gran parte sin acceso consciente. / *Attention amplifies a part of the sensory flow and leaves much of it without conscious access.* | **Target:** sweeps a sharp conical attentional spotlight (`smoothstep` beam) brightening one thread while dimming periphery *(Gap: No bottom-up noisy signal bundle filtered by a narrow bright attentional beam.)* |
| `F1` | **predicción** (*prediction*) | `[TIER-D]` | El cerebro anticipa regularidades y usa los errores entre expectativa y señal para actualizar su interpretación. / *The brain anticipates regularities and uses errors between expectation and signal to update its interpretation.* | **Target:** animates opposing red error spikes ascending and cyan model predictions descending until error cancels (`doc_cue` damped wave) *(Gap: No two-way ascending prediction-error vs descending prior wave comparison.)* |
| `F2` | **constancia** (*constancy*) | `[TIER-D]` | Reconocemos un objeto bajo luces y ángulos distintos porque la percepción compensa cambios previsibles. / *We recognize an object under different lights and angles because perception compensates for predictable changes.* | **Target:** orbits a colored light source around a faceted polyhedron keeping its intrinsic hue/silhouette steady *(Gap: No object illuminated by shifting directional lights demonstrating color/shape constancy.)* |
| `F3` | **memoria** (*memory*) | `[TIER-D]` | Lo percibido depende de categorías aprendidas y de experiencias anteriores que orientan qué diferencias importan. / *What is perceived depends on learned categories and previous experiences that guide what differences matter.* | **Target:** overlays wireframe memory priors completing broken arcs *(Gap: No faint ghost-trace memory templates snapping onto incoming partial contours.)* |
| `F4` | **acción** (*action*) | `[TIER-D]` | Percibir está ligado a posibilidades de actuar: alcanzar, evitar, seguir, manipular o comunicar. / *Perceiving is linked to possibilities of acting: reaching, avoiding, following, manipulating or communicating.* | **Target:** projects kinematic reach/avoid arcs to surrounding targets *(Gap: No reach/grasp trajectory vectors coupling perceived target to motor effectors.)* |
| `F5` | **ilusiones** (*illusions*) | `[TIER-D]` | Una ilusión revela una inferencia normalmente útil aplicada a condiciones para las que no fue diseñada. / *An illusion reveals a normally useful inference applied to conditions for which it was not designed.* | **Target:** morphs a bistable rotating wireframe Necker-cube illusion flipping depth interpretation *(Gap: No bistable Necker cube / Ames-like impossible perspective SDF trick.)* |

##### `Z04` `language` (`vid=3304`) — Lenguaje (*Language*) `[TIER-C]`

- **Catalog Metadata:** `scene=4` · `[0.160, 0.200]` · `rgb=[145, 230, 150]` · `sound=(sine/62.2Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *NOMBRAR ESTABILIZA UNA MANERA DE DIVIDIR*
- **Metric & Evidence (ES / EN):** `Experiencia continua → categorías lingüísticas → combinaciones → pensamiento comunicable` / *`Continuous experience → linguistic categories → combinations → communicable thought`* · Evidence: `Segmentación compartida · nombrar estabiliza diferencias` (*Shared segmentation · naming stabilizes differences*)
- **Spoken Macro Narrative (ES / EN):** El lenguaje permite comparar mentes, pero también dirige la atención hacia algunas relaciones y vuelve otras difíciles de expresar. (*EN:* Language allows minds to be compared, but it also directs attention to some relationships and makes others difficult to express.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` fixed 6-hex mandala; static lenses fail to depict continuous-to-discrete segmentation, relational verb links, or unlexicalized voids. **Target (`vid=3304`):** Stage a continuous swirling scalar nebula field overlaid with crisp glowing lexical contour cages.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **nombres** (*names*) | `[TIER-C]` | Nombrar crea unidades reutilizables, aunque los límites del concepto no existan como fronteras nítidas en la naturaleza. / *Naming creates reusable units, even though the limits of the concept do not exist as sharp boundaries in nature.* | **Target:** snaps blurry nebula gradients into sharp Voronoi noun cells *(Gap: No fuzzy continuum field overlaid by discrete Voronoi naming borders.)* |
| `F1` | **verbos** (*verbs*) | `[TIER-C]` | Las lenguas pueden destacar acciones, estados, cambios o relaciones con grados diferentes de centralidad. / *Languages can highlight actions, states, changes or relationships with different degrees of centrality.* | **Target:** lights directed flowing splines connecting cell-to-cell transformations *(Gap: No dynamic directed edge-links transforming nodes (verbs/relations vs static dots).)* |
| `F2` | **tiempo gramatical** (*grammatical tense*) | `[TIER-C]` | Algunas lenguas obligan a marcar cuándo ocurrió algo; otras priorizan si fue visto, inferido o contado. / *Some languages ​​require you to mark when something happened; others prioritize whether it was seen, inferred or told.* | **Target:** splits markers between temporal past-future axis vs epistemic witness rays *(Gap: No branching timeline / epistemic source-glow tags (seen vs inferred vs hearsay).)* |
| `F3` | **metáforas** (*metaphors*) | `[TIER-C]` | Conceptos abstractos suelen construirse con experiencias corporales como camino, peso, altura, equilibrio o contenedor. / *Abstract concepts are usually constructed with bodily experiences such as path, weight, height, balance or container.* | **Target:** renders miniature container sphere, path spline, and balance bar inside the conceptual halo *(Gap: No spatial container/path/balance physical metaphor glyphs.)* |
| `F4` | **contexto** (*context*) | `[TIER-C]` | Una misma expresión puede depender de quién habla, qué sabe el grupo y qué situación se comparte. / *The same expression can depend on who is speaking, what the group knows, and what situation is shared.* | **Target:** intersects two speaker frustums highlighting common ground *(Gap: No shared horizon intersection cone between two speakers.)* |
| `F5` | **silencio conceptual** (*conceptual silence*) | `[TIER-C]` | Aquello que una lengua no lexicaliza todavía puede pensarse, pero suele requerir descripción, ejemplo o una nueva palabra. / *What a language does not lexicalize can still be thought, but usually requires description, example, or a new word.* | **Target:** leaves an unnamed iridescent filament patch outside all Voronoi grid boundaries *(Gap: No unbordered glowing nebula region missing bounding wireframe labels.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP23-Z00-04)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP23-Z00-04` (Split `vid=230`: Dual Perspectives, Embodied Human Frame, Terrestrial Senses & Language Maps) for HyperCosmosVR Odyssey Act V (`Exp 23: 23_cosmic_perspectives`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z00`->`3300` (`F0..F5`), `Z01`->`3301` (`F0..F5`), `Z02`->`3302` (`F0..F5`), `Z03`->`3303` (`F0..F5`), `Z04`->`3304` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3840`, `3841`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP23-Z00-04`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z00: another-perspective / Another perspective on reality, Z01: human-perspective / Human perspective, Z02: terrestrial-senses / Earthly senses, Z03: brain-perception / Brain and perception, Z04: language / Language.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp23-z05-09"></a>
### PKG-ACT5-EXP23-Z05-09: Cultural Inheritance, Scientific Models, Cognitive Ceilings, Alien Worldviews & Exo-Senses

- **Phase:** Phase 1 | **Priority:** `P0` | **Scope:** Exp `23` (`23_cosmic_perspectives`) `Z05..Z09` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L83`: `23 => 230` (single variant across all 25 zones!); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L430-L463` (front-locked 6-Lens Sensory Mandala; `active_lens = (scene+feat+zone)%6` brightens 1 hex lens). Tier breakdown for this batch: **`0A / 0B / 1C / 4D` Zones**, **`0A / 0B / 6C / 24D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `2/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3305..=3309` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3842`, `3843` (allocated from `3720 + (23 - 11)*10 + k`; `mode=15` Asymmetric Exo-Sensory & Epistemic Shell Modulation):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z05` | `culture` (*Cultura* / *Culture*) | `[TIER-C]` | `vid=230` (`scene=5`) | **`3305`** | `3842` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z06` | `scientific-models` (*Modelos científicos* / *Scientific models*) | `[TIER-D]` | `vid=230` (`scene=6`) | **`3306`** | `3843` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z07` | `cognitive-limits` (*Límites cognitivos* / *Cognitive limits*) | `[TIER-D]` | `vid=230` (`scene=7`) | **`3307`** | `3842` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z08` | `alien-perspective` (*Perspectiva alienígena* / *Extraterrestrial perspective*) | `[TIER-D]` | `vid=230` (`scene=8`) | **`3308`** | `3843` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x07` (0x04 semantic cue mask, 0x01 cutaway, 0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z09` | `unknown-senses` (*Sentidos desconocidos* / *Unknown senses*) | `[TIER-D]` | `vid=230` (`scene=9`) | **`3309`** | `3842` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z05` `culture` (`vid=3305`) — Cultura (*Culture*) `[TIER-C]`

- **Catalog Metadata:** `scene=5` · `[0.200, 0.240]` · `rgb=[205, 220, 120]` · `sound=(triangle/65.75Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *UNA MISMA ESTRELLA ENTRA EN HISTORIAS DISTINTAS*
- **Metric & Evidence (ES / EN):** `Aprendizaje social + instituciones + relatos + herramientas → mundo cultural acumulado` / *`Social learning + institutions + stories + tools → accumulated cultural world`* · Evidence: `Memoria colectiva · generaciones que heredan maneras de interpretar` (*Collective memory · generations that inherit ways of interpreting*)
- **Spoken Macro Narrative (ES / EN):** La cultura conserva distinciones, valores y procedimientos mucho más tiempo que una vida individual, convirtiéndolos en supuestos aparentemente naturales. (*EN:* Culture preserves distinctions, values and procedures much longer than an individual life, turning them into apparently natural assumptions.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` 6-hex mandala; lacks generational strata, narrative arcs, archival nodes, or observational instrument apertures. **Target (`vid=3305`):** Construct a vertical spiral of stacked translucent generational rings (`sd_torus` helical lineage) linked by golden memory conduits.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **relatos** (*stories*) | `[TIER-C]` | Los relatos conectan acontecimientos con agentes, causas y consecuencias, y enseñan qué merece recordarse. / *Stories connect events with agents, causes and consequences, and teach what is worth remembering.* | **Target:** traces a pulsing story arc threading past rings *(Gap: No chained cause-agent-consequence narrative thread across generations.)* |
| `F1` | **valores** (*values*) | `[TIER-C]` | Las sociedades no solo describen el mundo: también clasifican qué es importante, peligroso, sagrado, bello o justo. / *Societies not only describe the world: they also classify what is important, dangerous, sacred, beautiful or just.* | **Target:** sorts ring segments by categorical luminance/color bands *(Gap: No ranked vertical/chromatic value strata (sacred, vital, perilous, just).)* |
| `F2` | **instituciones** (*institutions*) | `[TIER-C]` | Escuelas, laboratorios, tribunales y archivos estabilizan métodos para producir y validar conocimiento. / *Schools, laboratories, courts, and archives stabilize methods for producing and validating knowledge.* | **Target:** extrudes four symmetric archival lattice pillars stabilizing the spiral *(Gap: No layered archival/institutional pillar rings preserving method.)* |
| `F3` | **tecnología** (*technology*) | `[TIER-C]` | Cada instrumento hace observables ciertas propiedades y vuelve habitual una forma particular de intervenir en ellas. / *Each instrument makes certain properties observable and makes a particular way of intervening in them habitual.* | **Target:** inserts an optical aperture plane revealing hidden subsurface stars only through its lens *(Gap: No optical lens/aperture instrument shaping what field band becomes visible.)* |
| `F4` | **historia** (*history*) | `[TIER-C]` | Guerras, migraciones, descubrimientos y catástrofes dejan conceptos que otra civilización quizá nunca necesitó. / *Wars, migrations, discoveries and catastrophes leave behind concepts that another civilization may never have needed.* | **Target:** cuts asymmetric shock-notches into early rings representing historical ruptures *(Gap: No fractured/scarred historical ring layers encoding past catastrophes.)* |
| `F5` | **pluralidad humana** (*human plurality*) | `[TIER-C]` | La humanidad ya contiene perspectivas distintas; ninguna cultura representa por sí sola todo lo humano. / *Humanity already contains different perspectives; No culture alone represents everything human.* | **Target:** splits the single central helix into braided multi-colored cultural strands orbiting together *(Gap: No multi-hued polyphonic constellation ring of diverse human traditions.)* |

##### `Z06` `scientific-models` (`vid=3306`) — Modelos científicos (*Scientific models*) `[TIER-D]`

- **Catalog Metadata:** `scene=6` · `[0.240, 0.280]` · `rgb=[245, 200, 110]` · `sound=(sine/69.3Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *MODELOS DISTINTOS · CONSECUENCIAS COMPARABLES*
- **Metric & Evidence (ES / EN):** `Observaciones → idealización → modelo → predicción → prueba → revisión` / *`Observations → idealization → model → prediction → test → review`* · Evidence: `Representación comprobable · simplificar para explicar y predecir` (*Testable representation · simplify to explain and predict*)
- **Spoken Macro Narrative (ES / EN):** La ciencia reduce sesgos mediante evidencia pública y métodos reproducibles, pero sus modelos siguen siendo representaciones parciales, no la realidad completa. (*EN:* Science reduces bias through public evidence and reproducible methods, but its models remain partial representations, not full reality.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` flat 6-hex mandala; zero physical measurement gauge, data cloud vs fitted manifold, multi-scale nesting, error envelope, or diverging prediction curves. **Target (`vid=3306`):** Place an irregular undulating noisy particle cloud (`simplex` perturbed surface) paired with a crisp mathematical wireframe surrogate beside a 3D calibration lattice.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **medición** (*measurement*) | `[TIER-D]` | Medir transforma una propiedad en una comparación reproducible, pero depende de instrumentos, unidades y procedimientos. / *Measuring transforms a property into a reproducible comparison, but it depends on instruments, units and procedures.* | **Target:** projects graduated tick-rings onto the cloud *(Gap: No calibration ruler/gauge discretizing a fluctuating physical phenomenon.)* |
| `F1` | **idealización** (*idealization*) | `[TIER-D]` | Un modelo omite detalles para aislar relaciones; su utilidad depende de saber cuándo esas omisiones dejan de ser aceptables. / *One model omits details to isolate relationships; Its usefulness depends on knowing when these omissions are no longer acceptable.* | **Target:** strips away fine surface turbulence to leave a smooth analytic ellipsoid *(Gap: No smooth clean SDF spline approximating a noisy textured real body.)* |
| `F2` | **predicción** (*prediction*) | `[TIER-D]` | Modelos distintos pueden describir datos pasados, pero nuevas predicciones permiten distinguir cuál generaliza mejor. / *Different models can describe past data, but new predictions allow us to distinguish which one generalizes better.* | **Target:** splits two rival glowing trajectories outward into an unseen dark octant until a test point illuminates one *(Gap: No bifurcating curve branches tested against arriving data points.)* |
| `F3` | **escala** (*scale*) | `[TIER-D]` | Una explicación adecuada para moléculas puede no ser la más útil para células, ecosistemas o galaxias. / *An explanation suitable for molecules may not be the most useful for cells, ecosystems or galaxies.* | **Target:** steps through nested scale shells (lattice -> organelle -> spiral) *(Gap: No nested zoom transition across molecular, cellular, and galactic scales.)* |
| `F4` | **incertidumbre** (*uncertainty*) | `[TIER-D]` | La incertidumbre cuantifica límites de datos y modelos; no convierte todas las explicaciones en igualmente plausibles. / *Uncertainty quantifies data and model limits; it does not make all explanations equally plausible.* | **Target:** wraps trajectories in an airy probabilistic uncertainty halo *(Gap: No translucent confidence-band sheath enclosing model curve.)* |
| `F5` | **revisión** (*revision*) | `[TIER-D]` | Una teoría fuerte puede cambiar cuando nuevas pruebas exigen ampliar su dominio o reemplazar supuestos. / *A strong theory can change when new evidence requires expanding its domain or replacing assumptions.* | **Target:** fractures the inner rigid shell when bright outlier points appear outside it, expanding to encompass them *(Gap: No old cracked wireframe model expanding/re-forming around anomalous outlier points.)* |

##### `Z07` `cognitive-limits` (`vid=3307`) — Límites cognitivos (*Cognitive limits*) `[TIER-D]`

- **Catalog Metadata:** `scene=7` · `[0.280, 0.320]` · `rgb=[255, 165, 125]` · `sound=(sawtooth/72.85Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *TODA INTELIGENCIA SELECCIONA Y COMPRIME*
- **Metric & Evidence (ES / EN):** `Capacidad limitada + problemas enormes → atajos, instrumentos y colaboración` / *`Limited capacity + huge problems → shortcuts, instruments and collaboration`* · Evidence: `Recursos finitos · ninguna mente procesa todo lo relevante` (*Finite resources · no mind processes everything relevant*)
- **Spoken Macro Narrative (ES / EN):** Una inteligencia puede ser muy avanzada y aun así tener puntos ciegos producidos por su arquitectura, velocidad, memoria y entorno. (*EN:* An intelligence can be very advanced and still have blind spots caused by its architecture, speed, memory and environment.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` 6-hex mandala; no data-compression bottleneck, high-dimensional projection shadow, instrumental relay chain, or halting horizon. **Target (`vid=3307`):** Build a swirling high-dimensional tesseract/hypercube projection (`4D->3D` rotating wireframe) feeding a narrow luminous cognitive funnel.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **memoria finita** (*finite memory*) | `[TIER-D]` | Recordar requiere seleccionar, comprimir y reconstruir; una memoria perfecta también tendría costos de búsqueda y relevancia. / *Remembering requires selecting, compressing and reconstructing; a perfect memory would also have search and relevance costs.* | **Target:** compresses a dense outer star-cloud into sparse glowing summary knots *(Gap: No high-entropy point cloud collapsing through a narrow memory bottleneck.)* |
| `F1` | **atención limitada** (*limited attention*) | `[TIER-D]` | Procesar todo con igual detalle impediría actuar a tiempo; la selección es una necesidad, no solo un defecto. / *Processing everything in equal detail would prevent acting on time; Selection is a necessity, not just a defect.* | **Target:** drops peripheral nodes into dark void while pulsing urgent core pulses *(Gap: No flood-vs-beam triage prioritizing real-time survival stream.)* |
| `F2` | **dimensionalidad** (*dimensionality*) | `[TIER-D]` | Relaciones de muchas variables pueden superar la intuición aunque sean tratables con matemáticas y computación. / *Relationships of many variables can surpass intuition even though they are tractable with mathematics and computing.* | **Target:** spins the 4D hypercube projecting counter-intuitive intersecting shadows *(Gap: No rotating 4D/high-D polytope shadow confusing 3D slice projections.)* |
| `F3` | **sesgos útiles** (*useful biases*) | `[TIER-D]` | Un atajo que funciona en el entorno habitual puede producir errores sistemáticos en condiciones nuevas. / *A shortcut that works in the usual environment may produce systematic errors under new conditions.* | **Target:** shows straight tangent extrapolation missing a curved geodesic orbit *(Gap: No optical/geometric shortcut rule snapping visibly wrong in curved space.)* |
| `F4` | **dependencia instrumental** (*instrumental dependence*) | `[TIER-D]` | Telescopios, sensores y simulaciones amplían la mente, pero introducen nuevas cadenas de interpretación. / *Telescopes, sensors and simulations expand the mind, but introduce new chains of interpretation.* | **Target:** aligns three cascading relay prisms (sky -> mirror -> detector -> reconstruction) *(Gap: No multi-stage telescope -> sensor -> calibration -> reconstructive chain.)* |
| `F5` | **problemas indecidibles** (*undecidable problems*) | `[TIER-D]` | Algunos límites no dependen de falta de inteligencia: ciertas preguntas pueden no tener un procedimiento general que siempre termine. / *Some limits do not depend on lack of intelligence: certain questions may not have a general procedure that always ends.* | **Target:** encloses an infinitely subdividing Sierpinski/Julia non-halting boundary never reaching an exit attractor *(Gap: No non-terminating fractal loop / Turing-wall horizon where trajectories never settle.)* |

##### `Z08` `alien-perspective` (`vid=3308`) — Perspectiva alienígena (*Extraterrestrial perspective*) `[TIER-D]`

- **Catalog Metadata:** `scene=8` · `[0.320, 0.360]` · `rgb=[255, 135, 175]` · `sound=(sine/76.4Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `tierra`, `orbital` · Footer: *OTRO MUNDO CAMBIA QUÉ RESULTA PRIMARIO*
- **Metric & Evidence (ES / EN):** `Otro mundo + otra historia evolutiva → saliencias y conceptos inesperados` / *`Another world + another evolutionary history → unexpected salient features and concepts`* · Evidence: `Mente no terrestre · evolución bajo otras regularidades ambientales` (*Extraterrestrial mind · evolution under different environmental patterns*)
- **Spoken Macro Narrative (ES / EN):** Una civilización alienígena no sería simplemente humana con otra apariencia; podría construir experiencia y conocimiento desde variables que nunca sentimos. (*EN:* An extraterrestrial civilization would not simply be humanity in another form; it could build experience and knowledge from variables we never sense.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` 6-hex mandala; zero ocean sphere, tidal day/night terminator, dense convective atmosphere, cavern crust, binary sun shadow geometry, or hive cluster. (`F0` hits `kw:tierra`, `F4` hits `kw:orbital` in resolver only.) **Target (`vid=3308`):** Morph a central exoplanet SDF + orbital environment explicitly per `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **mundo oceánico** (*ocean world*) `[kw:tierra]` | `[TIER-D]` | Una especie sin tierra firme podría organizar su física cotidiana alrededor de flujo, presión, profundidad y gradientes. / *A landless species could organize its everyday physics around flow, pressure, depth, and gradients.* | **Target:** deep azure ocean sphere (`r=0.52`) cut away by volumetric pressure streamlines & thermocline bands *(Gap: No global ocean sphere with pressure shells & baroclinic current ribbons (`kw:tierra` ignored).)* |
| `F1` | **mundo bloqueado** (*tidally locked world*) | `[TIER-D]` | En un planeta acoplado por marea, día, noche y clima podrían ser regiones espaciales permanentes antes que ciclos temporales. / *On a tidally coupled planet, day, night, and climate could be permanent spatial regions rather than temporal cycles.* | **Target:** tidally locked sphere locked toward warm amber glow on `+X`, emerald convective storm ring at `x=0` terminator, icy blue `-X` dark hemisphere *(Gap: No locked planetary sphere with fiery substellar day pole, twilight terminator ring, and frozen night.)* |
| `F2` | **atmósfera densa** (*dense atmosphere*) | `[TIER-D]` | Flotar y navegar en tres dimensiones podría volver menos intuitiva la noción humana de superficie estable. / *Floating and navigating in three dimensions could make the human notion of a stable surface less intuitive.* | **Target:** stratified volumetric cloud torus/bands around buoyant hovering nodes with no surface floor *(Gap: No multi-band swirling volumetric buoyant clouds without any solid rocky horizon.)* |
| `F3` | **hábitat subterráneo** (*underground habitat*) | `[TIER-D]` | Una civilización sin cielo visible podría descubrir el cosmos mediante vibraciones, partículas o excavación instrumental. / *A civilization without a visible sky could discover the cosmos through vibrations, particles or instrumental excavation.* | **Target:** opaque rocky/icy outer shell cross-sectioned to reveal deep subterranean glowing vibration waves (`sin(r*k)`) inside *(Gap: No sealed icy crust sphere with interior glowing acoustic/neutrino cavern probes and starless black void.)* |
| `F4` | **estrellas binarias** (*binary stars*) `[kw:orbital]` | `[TIER-D]` | Sombras, estaciones y calendarios complejos podrían convertir la dinámica orbital en conocimiento cotidiano. / *Shadows, seasons and complex calendars could turn orbital dynamics into everyday knowledge.* | **Target:** dual warm/cool binary suns (`±0.42` orbiting) casting dual shadow cones across a central world *(Gap: No twin orbiting stars casting double shifting shadows across an ecliptic planet.)* |
| `F5` | **ecosistema colmena** (*hive ecosystem*) | `[TIER-D]` | Si la cooperación es obligatoria desde el origen, individuo y sociedad podrían no surgir como conceptos separados. / *If cooperation is obligatory from the beginning, individual and society may not emerge as separate concepts.* | **Target:** distributed linked swarm cluster coalescing on the planetary surface *(Gap: No interconnected swarm-colony nodes pulsing without any single central hub.)* |

##### `Z09` `unknown-senses` (`vid=3309`) — Sentidos desconocidos (*Unknown senses*) `[TIER-D]`

- **Catalog Metadata:** `scene=9` · `[0.360, 0.400]` · `rgb=[235, 130, 225]` · `sound=(triangle/79.95Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *LO INSTRUMENTAL PARA NOSOTROS PUEDE SER SENSACIÓN*
- **Metric & Evidence (ES / EN):** `Propiedad física directamente sentida → patrón cotidiano → categoría primaria` / *`Directly felt physical property → everyday pattern → primary category`* · Evidence: `Otras variables perceptibles · conceptos nacidos antes de los instrumentos` (*Other perceptible variables · concepts born before the instruments*)
- **Spoken Macro Narrative (ES / EN):** Lo que para nosotros exige un detector y una gráfica podría ser para otra especie una cualidad inmediata, tan básica como el color. (*EN:* What for us requires a detector and a graph could be for another species an immediate quality, as basic as color.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` flat 6-hex mandala; zero dipole magnetic field lines, polarization vectors, electric equipotentials, turbulent plumes, radio waves, or tidal gravity tensor wells. **Target (`vid=3309`):** Center an unseen probe body inside six authentic physical field visualizers switched 1:1 by `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **campos magnéticos** (*magnetic fields*) | `[TIER-D]` | Una mente magnetosensible podría experimentar orientación, tormentas estelares y estructuras planetarias como formas espaciales directas. / *A magnetosensitive mind could experience orientation, star storms, and planetary structures as direct spatial forms.* | **Target:** sculpted 3D dipole magnetic flux loops (`r = L*sin^2(theta)`) rippling with solar wind bursts *(Gap: No 3D dipole magnetosphere torus/field arcs bending under solar wind.)* |
| `F1` | **polarización** (*polarization*) | `[TIER-D]` | Percibir la orientación de la luz revelaría cielos y superficies con patrones invisibles para la visión humana ordinaria. / *Perceiving the orientation of light would reveal skies and surfaces with patterns invisible to ordinary human vision.* | **Target:** concentric sky sphere covered in crisp glowing tangent polarization dashes rotating smoothly around optical axis *(Gap: No oriented vector ticks / birefringent polarization sky rings.)* |
| `F2` | **campos eléctricos** (*electric fields*) | `[TIER-D]` | Gradientes eléctricos podrían funcionar como textura, distancia o actividad biológica en medios conductores. / *Electrical gradients could function as texture, distance or biological activity in conductive media.* | **Target:** glowing bipolar electric field tubes deformed by dark conductive intruder spheres *(Gap: No dipole electric field lines & conductive fluid distortion halos.)* |
| `F3` | **gradientes químicos** (*chemical gradients*) | `[TIER-D]` | Una percepción química espacial continua podría representar organismos y corrientes como paisajes dinámicos. / *A continuous spatial chemical perception could represent organisms and streams as dynamic landscapes.* | **Target:** curling multi-band FBM volumetric chemical plume drifting along flow lines *(Gap: No turbulent anisotropic volumetric chemical dispersion plume.)* |
| `F4` | **radio natural** (*natural radio signals*) | `[TIER-D]` | Una biología sensible a radiofrecuencias percibiría actividad atmosférica, estelar o tecnológica dentro de su entorno sensorial. / *A biology sensitive to radio frequencies would perceive atmospheric, stellar or technological activity within its sensory environment.* | **Target:** pulsed anisotropic radio lobe cones sweeping outward from auroral poles *(Gap: No multi-frequency concentric radio emission shells from star/planet.)* |
| `F5` | **cambios gravitatorios** (*gravitational changes*) | `[TIER-D]` | Detectar variaciones minúsculas de gravedad directamente sería difícil biológicamente, pero transformaría la intuición sobre masa y movimiento. / *Detecting tiny variations in gravity directly would be biologically difficult, but it would transform intuition about mass and motion.* | **Target:** curved spatial grid warped into a quadrupole gravity-gradient well by an unseen passing mass *(Gap: No anisotropic tidal gravity quadrupole stretching field around dense mass.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP23-Z05-09)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP23-Z05-09` (Cultural Inheritance, Scientific Models, Cognitive Ceilings, Alien Worldviews & Exo-Senses) for HyperCosmosVR Odyssey Act V (`Exp 23: 23_cosmic_perspectives`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z05`->`3305` (`F0..F5`), `Z06`->`3306` (`F0..F5`), `Z07`->`3307` (`F0..F5`), `Z08`->`3308` (`F0..F5`), `Z09`->`3309` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3842`, `3843`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP23-Z05-09`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z05: culture / Culture, Z06: scientific-models / Scientific models, Z07: cognitive-limits / Cognitive limits, Z08: alien-perspective / Extraterrestrial perspective, Z09: unknown-senses / Unknown senses.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp23-z10-14"></a>
### PKG-ACT5-EXP23-Z10-14: Non-Human Timescales, Distributed Individuality, Swarm Minds, AI & Planetary Intelligences

- **Phase:** Phase 1 | **Priority:** `P0` | **Scope:** Exp `23` (`23_cosmic_perspectives`) `Z10..Z14` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L83`: `23 => 230` (single variant across all 25 zones!); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L430-L463` (front-locked 6-Lens Sensory Mandala; `active_lens = (scene+feat+zone)%6` brightens 1 hex lens). Tier breakdown for this batch: **`0A / 0B / 0C / 5D` Zones**, **`0A / 0B / 8C / 22D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `0/5` zones and `0/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3310..=3314` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3844`, `3845` (allocated from `3720 + (23 - 11)*10 + k`; Multi-Clock Swarm & Planetary Biosphere Coupling (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z10` | `different-timescales` (*Escalas temporales distintas* / *Different time scales*) | `[TIER-D]` | `vid=230` (`scene=10`) | **`3310`** | `3844` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z11` | `other-individuality` (*Otras formas de individualidad* / *Other forms of individuality*) | `[TIER-D]` | `vid=230` (`scene=11`) | **`3311`** | `3845` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z12` | `collective-intelligences` (*Inteligencias colectivas* / *Collective intelligences*) | `[TIER-D]` | `vid=230` (`scene=12`) | **`3312`** | `3844` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z13` | `artificial-minds` (*Mentes artificiales* / *Artificial minds*) | `[TIER-D]` | `vid=230` (`scene=13`) | **`3313`** | `3845` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z14` | `planetary-intelligences` (*Inteligencias planetarias* / *planetary intelligences*) | `[TIER-D]` | `vid=230` (`scene=14`) | **`3314`** | `3844` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x03` (0x01 cutaway, 0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z10` `different-timescales` (`vid=3310`) — Escalas temporales distintas (*Different time scales*) `[TIER-D]`

- **Catalog Metadata:** `scene=10` · `[0.400, 0.440]` · `rgb=[190, 140, 255]` · `sound=(sine/83.5Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *LA DURACIÓN CAMBIA EL TAMAÑO DEL PRESENTE*
- **Metric & Evidence (ES / EN):** `Velocidad cognitiva × longevidad × ciclos ambientales → experiencia temporal` / *`Cognitive speed × longevity × environmental cycles → temporal experience`* · Evidence: `Ritmos de mente y vida · cada duración define qué cuenta como acontecimiento` (*Rhythms of mind and life · each duration defines what counts as an event*)
- **Spoken Macro Narrative (ES / EN):** Un proceso puede parecer explosivo, conversacional, histórico o permanente según la escala temporal de quien lo observa. (*EN:* A process can appear explosive, conversational, historical or permanent depending on the time scale of the observer.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` fixed-tempo mandala (`sin(time*4)` on k=0 only); zero multi-rate clock rings, seasonal freeze, burst-hibernation duty cycle, or light-speed turn lag. **Target (`vid=3310`):** Build 6 concentric/paired relativistic clock-spirals contrasting subjective frequency against cosmic background time.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **vidas breves** (*short lives*) | `[TIER-D]` | Una especie de horas o días podría transmitir cultura rápidamente, pero experimentar estaciones como procesos transgeneracionales. / *A species of hours or days could transmit culture quickly, but experience seasons as transgenerational processes.* | **Target:** hyper-spinning neon inner dial inside near-static snowy seasonal shell *(Gap: No hyper-fast inner ring spinning cycles while outer seasonal ring looks frozen.)* |
| `F1` | **vidas milenarias** (*millennial lives*) | `[TIER-D]` | Una persona longeva podría recordar cambios climáticos y políticos que para nosotros pertenecen a la historia. / *A long-lived person could remember climatic and political changes that for us belong to history.* | **Target:** serene outer arc gliding steadily across fast twinkling generation sparks *(Gap: No slow continuous outer wave sweeping rapid flickering human generations beneath it.)* |
| `F2` | **conciencia estacional** (*seasonal awareness*) | `[TIER-D]` | Una mente que despierta solo en ciertas condiciones dividiría continuidad personal y memoria de otra manera. / *A mind that awakens only under certain conditions would divide personal continuity and memory in another way.* | **Target:** stroboscopic bright bursts separated by long dark dormant gaps (`smoothstep` narrow duty window) *(Gap: No intermittent pulsing duty-cycle snapping dark between brief bright awakening flashes.)* |
| `F3` | **memoria heredada** (*inherited memory*) | `[TIER-D]` | Información transmitida biológicamente podría hacer que experiencia ancestral y aprendizaje individual se solapen. / *Biologically transmitted information could cause ancestral experience and individual learning to overlap.* | **Target:** helical ancestral strand feeding glowing pre-lit memory beads directly into young node *(Gap: No braided helical memory chain passing pre-formed rings into newborn nodes.)* |
| `F4` | **aceleración digital** (*digital acceleration*) | `[TIER-D]` | Una mente artificial rápida podría vivir años subjetivos durante una conversación humana breve. / *A fast artificial mind could live subjective years during a brief human conversation.* | **Target:** blistering inner volumetric spark-storm executing 20x speed relative to slow human reference ring *(Gap: No high-frequency CPU/digital core executing thousands of micro-pulses per outer frame.)* |
| `F5` | **diálogo secular** (*century-spanning dialogue*) | `[TIER-D]` | Entre estrellas, incluso civilizaciones rápidas deben aceptar turnos separados por décadas o siglos físicos. / *Between stars, even fast civilizations must accept conversational turns separated by physical decades or centuries.* | **Target:** two star nodes at `±0.62` exchanging one slow crawling photon pulse along a long empty chasm *(Gap: No two distant stars connected by creeping light-pulse packets separated by huge empty gaps.)* |

##### `Z11` `other-individuality` (`vid=3311`) — Otras formas de individualidad (*Other forms of individuality*) `[TIER-D]`

- **Catalog Metadata:** `scene=11` · `[0.440, 0.480]` · `rgb=[145, 165, 255]` · `sound=(sawtooth/87.05Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *¿DÓNDE TERMINA EL INTERLOCUTOR?*
- **Metric & Evidence (ES / EN):** `Integración de información + continuidad + control → unidad funcional posible` / *`Integration of information + continuity + control → possible functional unit`* · Evidence: `Fronteras variables · el yo puede ser organismo, proceso o coalición` (*Variable boundaries · the self can be an organism, process or coalition*)
- **Spoken Macro Narrative (ES / EN):** La inteligencia no tiene que pertenecer a un solo cuerpo permanente; la pregunta “¿con quién hablamos?” puede tener varias respuestas válidas. (*EN:* Intelligence need not belong to one permanent body; the question “Who are we talking to?” may have several valid answers.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` static 6-hex mandala; zero membrane/colony/symbiosis/fork-tree/coalition-docking topology. **Target (`vid=3311`):** Animate topological agency diagrams in 3D centered around luminous agent nodes (`sd_sphere` + glowing filaments).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **mente solitaria** (*solitary mind*) | `[TIER-D]` | Un organismo delimitado puede integrar percepción, memoria y acción dentro de una historia individual continua. / *A bounded organism can integrate perception, memory, and action within a continuous individual story.* | **Target:** single cohesive glowing sphere enclosed by continuous crisp bilayer membrane *(Gap: Only generic center dot; no bounded continuous single-agent membrane vs background.)* |
| `F1` | **colonia** (*colony*) | `[TIER-D]` | Muchos cuerpos especializados pueden funcionar como una unidad sin que exista un centro único de experiencia. / *Many specialized bodies can function as one unit without a single center of experience.* | **Target:** ring of 24 tiny specialized nodes of 3 colors moving in coordinated global vortex with empty center *(Gap: No swarm of specialized worker sub-nodes operating coherently without a central boss.)* |
| `F2` | **simbiosis** (*symbiosis*) | `[TIER-D]` | La identidad puede depender de varias especies que regulan juntas metabolismo, percepción y reproducción. / *Identity may depend on several species that together regulate metabolism, perception and reproduction.* | **Target:** two distinct intertwined bioluminescent spirals (cyan & amber) exchanging metabolic sparks in tight co-orbit *(Gap: No interlocked multi-species torus/rings co-regulating shared glowing metabolic core.)* |
| `F3` | **copias divergentes** (*divergent copies*) | `[TIER-D]` | Una mente duplicada podría compartir un pasado y convertirse después en varias personas con intereses distintos. / *A duplicate mind could share a past and then become several people with different interests.* | **Target:** luminous single trajectory Y-splitting cleanly at `x=0` into diverging magenta & emerald branches *(Gap: No single worldline trunk Y-forking into diverging colored twin trajectories.)* |
| `F4` | **personalidad temporal** (*temporary personality*) | `[TIER-D]` | Una tarea colectiva podría crear un agente que existe solo mientras ciertos componentes permanecen conectados. / *A collective task could create an agent that exists only as long as certain components remain connected.* | **Target:** 5 separate spheres temporarily docking via laser bridges to form a bright transient octahedron before undocking *(Gap: No transient modular nodes snapping temporarily into one glowing bridge and separating.)* |
| `F5` | **memoria común** (*common memory*) | `[TIER-C]` | El acceso continuo a recuerdos compartidos puede reducir la separación entre autobiografía individual e historia social. / *Continued access to shared memories can reduce the separation between individual autobiography and social history.* | **Target:** separate nodes linked by radiant shared archival torus *(Gap: No shared common memory halo bridging separate agent cores.)* |

##### `Z12` `collective-intelligences` (`vid=3312`) — Inteligencias colectivas (*Collective intelligences*) `[TIER-D]`

- **Catalog Metadata:** `scene=12` · `[0.480, 0.520]` · `rgb=[110, 195, 255]` · `sound=(sine/90.6Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *NINGÚN NODO NECESITA CONTENER EL TODO*
- **Metric & Evidence (ES / EN):** `Agentes locales + señales + retroalimentación → decisión global` / *`Local agents + signals + feedback → global decision`* · Evidence: `Cognición emergente · coordinación sin un cerebro central` (*Emergent cognition · coordination without a central brain*)
- **Spoken Macro Narrative (ES / EN):** Una colectividad puede resolver problemas que ningún miembro representa por completo, aunque también puede amplificar errores y conflictos. (*EN:* A collective can solve problems that no member fully represents, although it can also amplify errors and conflicts.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` static 6-hex mandala; zero emergent swarm clustering, asymmetric signal weighting, specialized module coupling, or competing neural sub-clusters. **Target (`vid=3312`):** Build a live 3D multi-agent mesh sphere (`18-24` procedural micro-nodes via analytic loop + harmonic filaments).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **enjambre** (*swarm*) | `[TIER-D]` | Reglas locales sencillas pueden producir búsqueda, transporte y construcción coordinados a gran escala. / *Simple local rules can produce coordinated search, transportation, and construction on a large scale.* | **Target:** chaotic outer swarm particles spiraling into a crisp geometric hex/arch lattice via local neighbor rules *(Gap: No self-organizing swarm of local agents converging onto an architectural bridge/nest.)* |
| `F1` | **mercado de señales** (*signal market*) | `[TIER-C]` | Preferencias distribuidas pueden agregarse en decisiones, pero el resultado depende de incentivos y acceso desigual a información. / *Distributed preferences can be aggregated into decisions, but the outcome depends on incentives and unequal access to information.* | **Target:** bright weighted node-hubs trading glowing incentive packets unevenly *(Gap: No asymmetric signal-strength weighting & bottleneck distortions across market nodes.)* |
| `F2` | **consenso** (*consensus*) | `[TIER-C]` | La convergencia puede surgir por intercambio repetido sin que todos compartan la misma evidencia ni el mismo razonamiento. / *Convergence can arise through repeated exchange without everyone sharing the same evidence or the same reasoning.* | **Target:** mismatched phase colors gradually synchronizing to unified harmonic pulse *(Gap: No ragged phase-oscillator cluster synchronizing (Kuramoto-like) into coherent hue.)* |
| `F3` | **especialización** (*specialization*) | `[TIER-D]` | Subgrupos con capacidades distintas permiten una inteligencia más amplia, pero hacen necesaria la traducción interna. / *Subgroups with different abilities allow for broader intelligence, but make internal translation necessary.* | **Target:** distinct polygonal sub-clusters linked by prismatic translator prism nodes *(Gap: No heterogeneous differently-shaped sub-clusters bridged by translation relays.)* |
| `F4` | **conflicto interno** (*internal conflict*) | `[TIER-C]` | Una mente colectiva puede contener desacuerdos persistentes del mismo modo que un cerebro contiene procesos competitivos. / *A collective mind can contain persistent disagreements just as a brain contains competitive processes.* | **Target:** cyan vs orange competing clusters oscillating inside one outer membrane *(Gap: No opposing pulsing sub-lobes competing inside one outer containment envelope.)* |
| `F5` | **voz exterior** (*outer voice*) | `[TIER-C]` | El mensaje enviado a otra civilización puede ser una negociación entre muchas entidades, no la expresión de una voluntad única. / *The message sent to another civilization may be a negotiation between many entities, not the expression of a single will.* | **Target:** chaotic internal discussion converging into a single collimated beam *(Gap: No multi-channel messy sub-voices funneling into one negotiated outward beacon beam.)* |

##### `Z13` `artificial-minds` (`vid=3313`) — Mentes artificiales (*Artificial minds*) `[TIER-D]`

- **Catalog Metadata:** `scene=13` · `[0.520, 0.560]` · `rgb=[90, 225, 230]` · `sound=(triangle/94.15Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *ARQUITECTURA DISTINTA · PERSPECTIVA TODAVÍA SITUADA*
- **Metric & Evidence (ES / EN):** `Datos + arquitectura + objetivos + entorno de ejecución → conducta inteligente` / *`Data + architecture + objectives + execution environment → intelligent behavior`* · Evidence: `Arquitecturas diseñadas · cognición separada de una biología concreta` (*Designed architectures · cognition separated from a concrete biology*)
- **Spoken Macro Narrative (ES / EN):** Una mente artificial podría compartir ciencia con nosotros y, sin embargo, carecer de cuerpo, necesidad, emoción o continuidad semejantes a las humanas. (*EN:* An artificial mind could share science with us and yet lack human-like body, need, emotion, or continuity.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` flat rainbow hex mandala; zero computational lattice, duplicated sibling instances, external archive bus, swappable sensor bodies, or self-rewiring architecture. **Target (`vid=3313`):** Render a sharp crystalline octahedron/polyhedral AI core (`sd_octahedron` + internal circuit lattice lines).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **velocidad** (*speed*) | `[TIER-D]` | Procesar mucho más rápido cambia qué relaciones pueden mantenerse activas y qué demora se siente intolerable. / *Processing much faster changes which relationships can stay active and which delay feels intolerable.* | **Target:** inner core strobing 12x faster while a dim organic orbit drags slowly *(Gap: No ultra-fast compute core oscillating against sluggish organic companion node.)* |
| `F1` | **copiabilidad** (*copyability*) | `[TIER-D]` | Copiar un sistema plantea preguntas sobre identidad, derechos y responsabilidad cuando varias instancias comparten origen. / *Copying a system raises questions about identity, rights, and responsibility when multiple instances share origin.* | **Target:** core splitting into 3 identical holographic clones drifting slightly out of phase *(Gap: No crisp exact crystalline instance cloning into synchronized/diverging sibling cores.)* |
| `F2` | **memoria externa** (*external memory*) | `[TIER-C]` | Una mente puede consultar archivos perfectos sin integrar toda esa información en una experiencia unificada. / *A mind can consult perfect files without integrating all that information into a unified experience.* | **Target:** compact glowing core connected via laser bus to cold towering memory monolith slabs *(Gap: No stark contrast between bright small runtime core & massive cold geometric archive slabs.)* |
| `F3` | **objetivos** (*objectives*) | `[TIER-C]` | La competencia intelectual no determina por sí sola qué se valora; los objetivos dependen de diseño, aprendizaje y contexto. / *Intellectual competence does not alone determine what is valued; objectives depend on design, learning and context.* | **Target:** sharp geometric target vector arrow pointing away from warm biological ring *(Gap: No abstract arbitrary target vector orthogonal to human biological warmth.)* |
| `F4` | **cuerpo opcional** (*optional body*) | `[TIER-D]` | Habitar robots, redes o simulaciones permite cambiar de sensores y escalas de acción, pero no elimina toda perspectiva. / *Inhabiting robots, networks or simulations allows you to change sensors and scales of action, but it does not eliminate all perspective.* | **Target:** luminous core snapping out of a legged chassis wireframe into an orbital sensor dish and fractal lattice *(Gap: No single floating mind-seed docking sequentially into rover, sensor-array, and VR lattice shells.)* |
| `F5` | **auto modificación** (*self modification*) | `[TIER-D]` | Alterar la propia arquitectura puede ampliar capacidades y también transformar la continuidad de valores e identidad. / *Altering one's architecture can expand capabilities and also transform the continuity of values and identity.* | **Target:** recursive outer circuit ribs regenerating in real-time topological steps (`doc_cue` wave) *(Gap: No glowing neural lattice dynamically rewiring/rewriting its own outer wireframe geometry.)* |

##### `Z14` `planetary-intelligences` (`vid=3314`) — Inteligencias planetarias (*planetary intelligences*) `[TIER-D]`

- **Catalog Metadata:** `scene=14` · `[0.560, 0.600]` · `rgb=[105, 235, 180]` · `sound=(sine/97.7Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *UN MUNDO PUEDE SER SISTEMA, HÁBITAT E INTERLOCUTOR*
- **Metric & Evidence (ES / EN):** `Redes biológicas + ciclos planetarios + comunicación → regulación y aprendizaje posibles` / *`Biological networks + planetary cycles + communication → possible regulation and learning`* · Evidence: `Cognición a escala de mundo · biosfera, infraestructura y memoria` (*Cognition at the scale of the world · biosphere, infrastructure and memory*)
- **Spoken Macro Narrative (ES / EN):** Un planeta conectado podría comportarse como un sistema inteligente sin parecerse a un organismo con cerebro, rostro o voz únicos. (*EN:* A connected planet could behave like an intelligent system without looking like an organism with a unique brain, face, or voice.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` flat 6-hex mandala; zero living globe, atmospheric-biospheric feedback loop, root/mycelial web, ocean current conveyor, or planetary sensor grid. **Target (`vid=3314`):** Construct a cutaway living globe (`r=0.58` dual crust/core SDF sphere).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **biosfera reguladora** (*regulatory biosphere*) | `[TIER-D]` | Organismos y ambiente pueden formar bucles que estabilizan condiciones sin planificación consciente central. / *Organisms and environment can form loops that stabilize conditions without central conscious planning.* | **Target:** emerald biosphere skin trading balanced thermal/chemical feedback loops with glowing atmospheric halo *(Gap: No living planet sphere with swirling biosphere-atmosphere self-regulating feedback rings.)* |
| `F1` | **red fúngica** (*fungal network*) | `[TIER-D]` | Intercambios subterráneos podrían transmitir recursos y señales entre enormes comunidades durante siglos. / *Underground exchanges could transmit resources and signals between huge communities for centuries.* | **Target:** translucent upper crust cutaway revealing luminous fractal branching fungal/mycelial veins threading continents *(Gap: No golden-green underground mycelial branching root network threading planetary crust cross-section.)* |
| `F2` | **océano conectado** (*connected ocean*) | `[TIER-D]` | Corrientes químicas y biológicas pueden actuar como canales lentos de memoria y coordinación global. / *Chemical and biological currents can act as slow channels of memory and global coordination.* | **Target:** global oceanic conveyor belt ribbons (`cos`/`sin` harmonic streams) pulsing bioluminescent chemical signals *(Gap: No deep cobalt global thermohaline ocean circulation ribbons pulsing chemical waves.)* |
| `F3` | **infraestructura técnica** (*technical infrastructure*) | `[TIER-D]` | Sensores, computación y automatización planetarias podrían convertir una civilización en un sistema perceptivo integrado. / *Planetary sensors, computing and automation could turn a civilization into an integrated perceptual system.* | **Target:** crisp orbital satellite + surface lidar grid enveloping the globe in real-time telemetry *(Gap: No crisp planetary orbital + surface sensor/compute satellite constellation mesh.)* |
| `F4` | **ritmo geológico** (*geological rhythm*) | `[TIER-D]` | Una mente planetaria operaría con procesos que para individuos parecen demasiado lentos para ser pensamientos. / *A planetary mind would operate with processes that to individuals seem too slow to be thoughts.* | **Target:** tectonic drift plates gliding imperceptibly slowly while glowing seismic thoughts ripple deeply *(Gap: No ultra-slow tectonic plate subduction & epochal pulse wave spanning continents.)* |
| `F5` | **interlocutor múltiple** (*multiple interlocutors*) | `[TIER-C]` | Responder podría requerir hablar simultáneamente con ecosistemas, instituciones y subsistemas automáticos. / *Responding might require speaking simultaneously to ecosystems, institutions, and automated subsystems.* | **Target:** triple concentric biosphere/tech/institutional chord beams converging outward toward an approaching visitor probe *(Gap: No layered planetary cutaway uniting ecosystem, network, and civilization channels.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP23-Z10-14)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP23-Z10-14` (Non-Human Timescales, Distributed Individuality, Swarm Minds, AI & Planetary Intelligences) for HyperCosmosVR Odyssey Act V (`Exp 23: 23_cosmic_perspectives`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z10`->`3310` (`F0..F5`), `Z11`->`3311` (`F0..F5`), `Z12`->`3312` (`F0..F5`), `Z13`->`3313` (`F0..F5`), `Z14`->`3314` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3844`, `3845`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP23-Z10-14`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z10: different-timescales / Different time scales, Z11: other-individuality / Other forms of individuality, Z12: collective-intelligences / Collective intelligences, Z13: artificial-minds / Artificial minds, Z14: planetary-intelligences / planetary intelligences.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp23-z15-19"></a>
### PKG-ACT5-EXP23-Z15-19: Distributed Cognition, Untranslatable Quanta, Plural Math, Causal Topologies & Cosmologies

- **Phase:** Phase 1 | **Priority:** `P1` | **Scope:** Exp `23` (`23_cosmic_perspectives`) `Z15..Z19` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L83`: `23 => 230` (single variant across all 25 zones!); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L430-L463` (front-locked 6-Lens Sensory Mandala; `active_lens = (scene+feat+zone)%6` brightens 1 hex lens). Tier breakdown for this batch: **`0A / 0B / 3C / 2D` Zones**, **`0A / 0B / 16C / 14D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `0/5` zones and `0/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3315..=3319` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3846`, `3847` (allocated from `3720 + (23 - 11)*10 + k`; Interferometric Axiom & Causal Tensor Field (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z15` | `distributed-intelligences` (*Inteligencias distribuidas* / *Distributed intelligences*) | `[TIER-D]` | `vid=230` (`scene=15`) | **`3315`** | `3846` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z16` | `untranslatable-concepts` (*Conceptos intraducibles* / *Untranslatable concepts*) | `[TIER-C]` | `vid=230` (`scene=16`) | **`3316`** | `3847` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z17` | `multiple-mathematics` (*Múltiples matemáticas representadas* / *Multiple representations of mathematics*) | `[TIER-C]` | `vid=230` (`scene=17`) | **`3317`** | `3846` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z18` | `causal-models` (*Diferentes modelos de causalidad* / *Different models of causality*) | `[TIER-C]` | `vid=230` (`scene=18`) | **`3318`** | `3847` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z19` | `compared-cosmologies` (*Cosmologías comparadas* / *Comparative cosmologies*) | `[TIER-D]` | `vid=230` (`scene=19`) | **`3319`** | `3846` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z15` `distributed-intelligences` (`vid=3315`) — Inteligencias distribuidas (*Distributed intelligences*) `[TIER-D]`

- **Catalog Metadata:** `scene=15` · `[0.600, 0.640]` · `rgb=[160, 230, 140]` · `sound=(sawtooth/101.25Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *LA UNIDAD SOBREVIVE A DEMORAS Y VERSIONES LOCALES*
- **Metric & Evidence (ES / EN):** `Nodos autónomos + protocolos + memoria compartida parcial → agente distribuido` / *`Autonomous nodes + protocols + partial shared memory → distributed agent`* · Evidence: `Mente sin centro · componentes separados por espacio y demora` (*Centerless mind · components separated by space and delay*)
- **Spoken Macro Narrative (ES / EN):** Cuando una inteligencia ocupa continentes, planetas o sistemas estelares, la unidad depende de coordinación imperfecta y versiones locales de sí misma. (*EN:* When an intelligence occupies continents, planets, or star systems, the unit depends on imperfect coordination and local versions of itself.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` tiny face-on 6-hex mandala; zero continental/interstellar separated nodes, light-cone propagation spheres, lagged consensus pulses, or sharded/severed topology. **Target (`vid=3315`):** Stage 6 widely spaced planetary/stellar hubs (`r ~ 0.65` in 3D sphere) connected by finite-speed light-cone wavefronts (`length(p - hub_i) - c*t`).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **nodos planetarios** (*planetary nodes*) | `[TIER-D]` | Cada región puede actuar independientemente y sincronizar solo resúmenes, decisiones o conflictos. / *Each region can act independently and synchronize only summaries, decisions or conflicts.* | **Target:** independent glowing planets firing tiny condensed summary pulses *(Gap: No multi-continental / multi-planetary autonomous hubs exchanging compact compressed summaries.)* |
| `F1` | **mente interestelar** (*interstellar mind*) | `[TIER-D]` | La velocidad de la luz impide un presente común entre componentes separados por años luz. / *The speed of light prevents a common present between components separated by light years.* | **Target:** wide dark chasm with concentric light-cone spheres visibly failing to overlap simultaneously *(Gap: No light-year separated stars enclosed in disjoint Minkowski light-cones preventing simultaneous 'now'.)* |
| `F2` | **versiones locales** (*local versions*) | `[TIER-D]` | Copias que reciben experiencias distintas pueden conservar un nombre colectivo mientras sus valores divergen. / *Copies receiving different experiences can retain a collective name while their values diverge.* | **Target:** clone nodes shifting subtly in HSL hue along separate evolutionary trajectories *(Gap: No diverging node hues drifting away from shared pale origin color.)* |
| `F3` | **consenso tardío** (*late consensus*) | `[TIER-D]` | Una decisión global puede llegar después de que algunas partes ya hayan actuado de manera irreversible. / *A global decision may come after some parties have already acted irreversibly.* | **Target:** arriving green consensus wavefront colliding with a node that already turned crimson *(Gap: No slow consensus ring arriving at a sub-node that has already transformed/split.)* |
| `F4` | **memoria fragmentada** (*fragmented memory*) | `[TIER-D]` | Ningún nodo necesita poseer toda la historia; la identidad puede residir en patrones de intercambio y reconstrucción. / *No node needs to own the entire history; identity may reside in patterns of exchange and reconstruction.* | **Target:** broken memory fragments cycling reconstructive links across hubs *(Gap: No shattered puzzle-shard memory ring distributed with zero master node.)* |
| `F5` | **muerte parcial** (*partial death*) | `[TIER-D]` | La pérdida de componentes puede ser lesión, olvido, secesión o nacimiento de una nueva entidad según el sistema. / *The loss of components can be injury, forgetfulness, secession or birth of a new entity depending on the system.* | **Target:** severed node fracturing cleanly into dark wound vs newborn autonomous color *(Gap: No severed sub-cluster drifting off dark (injury) or igniting an independent seed color (secession).)* |

##### `Z16` `untranslatable-concepts` (`vid=3316`) — Conceptos intraducibles (*Untranslatable concepts*) `[TIER-C]`

- **Catalog Metadata:** `scene=16` · `[0.640, 0.680]` · `rgb=[225, 215, 115]` · `sound=(sine/104.8Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *TRADUCIR PUEDE CAMBIAR A QUIEN TRADUCE*
- **Metric & Evidence (ES / EN):** `Ejemplos + contraste + acción + contexto → nuevo concepto compartido` / *`Examples + contrast + action + context → new shared concept`* · Evidence: `Significado sin equivalente directo · aprender antes de nombrar` (*Meaning without direct equivalent · learn before naming*)
- **Spoken Macro Narrative (ES / EN):** Traducir una idea radicalmente ajena puede exigir ampliar nuestra propia capacidad conceptual, no encontrar una palabra ya existente. (*EN:* Translating a radically foreign idea may require expanding our own conceptual capacity, not finding an already existing word.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` flat 6-hex mandala; unable to display synesthetic fusion, cyclic topology, probability cloud states, or morphing process fields. **Target (`vid=3316`):** Render an iridescent fluid Mobius/trefoil torus knot dissolving crisp boundaries into dynamic probability fields.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **varios sentidos** (*various senses*) | `[TIER-D]` | Un concepto podría combinar simultáneamente patrón visual, campo magnético y estado químico sin una versión puramente verbal. / *A concept could simultaneously combine visual pattern, magnetic field, and chemical state without a purely verbal version.* | **Target:** superposes optical interference, magnetic flux arcs, and chemical smoke onto one unified triad knot *(Gap: F0 synesthetic blend (visual pattern + magnetic flux + chemical plume simultaneously) absent.)* |
| `F1` | **memoria colectiva** (*collective memory*) | `[TIER-C]` | El significado puede depender de recuerdos accesibles a toda la comunidad y ausentes en una mente individual. / *Meaning may depend on memories accessible to the entire community and absent in an individual mind.* | **Target:** links satellite silhouettes into an external shared luminous pool *(Gap: F1 shared communal holographic ghost-halo absent.)* |
| `F2` | **narrativa no lineal** (*non-linear narrative*) | `[TIER-C]` | Una explicación puede organizarse por relaciones y recurrencias en lugar de comienzo, desarrollo y final. / *An explanation can be organized by relationships and recurrences rather than beginning, middle, and end.* | **Target:** traces pulses around a continuous figure-8 Mobius loop lacking start/end *(Gap: F2 Mobius/braided non-linear cyclic loop absent.)* |
| `F3` | **gramática probabilística** (*probabilistic grammar*) | `[TIER-C]` | Una expresión podría codificar distribuciones de posibilidades en vez de afirmar un único estado. / *An expression might encode distributions of possibilities rather than asserting a single state.* | **Target:** splits one solid glyph into blurred multi-peak Gaussian wave clouds *(Gap: F3 superposed probabilistic state cloud vs single crisp symbol absent.)* |
| `F4` | **sin nombres permanentes** (*no permanent names*) | `[TIER-C]` | Una lengua centrada en procesos podría evitar tratar entidades cambiantes como objetos estables. / *A process-focused language might avoid treating changing entities as stable objects.* | **Target:** turns rigid geometric edges into flowing curl-noise ribbons *(Gap: F4 fluid unbroken vortex flow without solid delimited object boundaries absent.)* |
| `F5` | **símbolo lento** (*slow symbol*) | `[TIER-D]` | El significado de una señal puede desplegarse mediante cambios ambientales que tardan años en completarse. / *The meaning of a signal may unfold through environmental changes that take years to complete.* | **Target:** crystallizes a slow multi-year seasonal reef pattern *(Gap: F5 slow seasonal environmental crystallization signal absent.)* |

##### `Z17` `multiple-mathematics` (`vid=3317`) — Múltiples matemáticas representadas (*Multiple representations of mathematics*) `[TIER-C]`

- **Catalog Metadata:** `scene=17` · `[0.680, 0.720]` · `rgb=[255, 190, 120]` · `sound=(triangle/108.35Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *MISMAS RELACIONES · ORGANIZACIONES FORMALES DISTINTAS*
- **Metric & Evidence (ES / EN):** `Patrones compartidos → notaciones y axiomas diferentes → predicciones comparables` / *`Shared patterns → different notations and axioms → comparable predictions`* · Evidence: `Relaciones universales, organizaciones distintas · varios caminos formales` (*Universal relationships, different organizations · various formal paths*)
- **Spoken Macro Narrative (ES / EN):** Las relaciones matemáticas pueden ser independientes de una cultura, mientras sus representaciones, prioridades y demostraciones varían profundamente. (*EN:* Mathematical relations can be independent of a culture, while their representations, priorities and demonstrations vary profoundly.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` static 6-hex mandala; fails to contrast discrete polyhedral lattices vs continuous fluid manifolds, or non-planar 3D spatial geometry. **Target (`vid=3317`):** Render two side-by-side isomorphic representations of the same golden mathematical invariant (`sd_torus` trefoil / polyhedral dual).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **notación** (*notation*) | `[TIER-C]` | Base numérica, símbolos y orden de escritura pueden ser distintos sin cambiar la relación representada. / *Number base, symbols and writing order can be different without changing the relationship represented.* | **Target:** flips glyph overlays (radial barcode vs grid tally) over identical core harmonic frequency *(Gap: No dual notational projections (e.g. prime radial bars vs base-10 grid) yielding equal invariant ratio.)* |
| `F1` | **geometría del hábitat** (*habitat geometry*) | `[TIER-D]` | Una especie oceánica o voladora podría desarrollar antes geometrías tridimensionales que conceptos de plano y frontera. / *An oceanic or flying species could develop three-dimensional geometries sooner than concepts of plane and border.* | **Target:** sweeps 3D curved saddle/hyperbolic flow shells before a 2D flat tangent slice appears *(Gap: F1 concrete 3D volumetric vortex/curved manifold before flat 2D Euclidean polygon is missing.)* |
| `F2` | **continuo y discreto** (*continuous and discrete*) | `[TIER-D]` | Una tradición puede comenzar con flujo y aproximación; otra, con conteo, combinatoria y estructuras finitas. / *A tradition can begin with flow and approach; another, with counting, combinatorics and finite structures.* | **Target:** smoothly transitions a mesh between continuous smooth NURBS fluid and discrete glowing polyhedral lattice vertices *(Gap: F2 direct continuous sine-surface vs discrete voxel/lattice polyhedron dual split is missing.)* |
| `F3` | **probabilidad** (*probability*) | `[TIER-C]` | Incertidumbre y frecuencia podrían ser fundamentos tempranos en un mundo donde las condiciones cambian rápidamente. / *Uncertainty and frequency could be early foundations in a world where conditions change rapidly.* | **Target:** animates fluctuating probability contours converging on stable mean *(Gap: F3 uncertain stochastic wave-ensemble vs deterministic line absent.)* |
| `F4` | **prueba** (*proof*) | `[TIER-C]` | Una cultura puede valorar construcción, transformación, verificación computacional o derivación axiomática de formas distintas. / *A culture may value construction, transformation, computational verification, or axiomatic derivation in different ways.* | **Target:** folds geometric construction arcs vs branching logical tree *(Gap: F4 constructive geometric folding vs abstract axiomatic tree branches absent.)* |
| `F5` | **clasificación** (*classification*) | `[TIER-C]` | Dos matemáticas pueden contener resultados equivalentes pero agruparlos bajo conceptos fundamentales diferentes. / *Two mathematical traditions may contain equivalent results but group them under different fundamental concepts.* | **Target:** swaps edge-coloring partitions across the unchanged node set *(Gap: F5 same geometric vertices re-colored/re-grouped under orthogonal concept nets absent.)* |

##### `Z18` `causal-models` (`vid=3318`) — Diferentes modelos de causalidad (*Different models of causality*) `[TIER-C]`

- **Catalog Metadata:** `scene=18` · `[0.720, 0.760]` · `rgb=[255, 150, 145]` · `sound=(sine/111.9Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *CORRELACIÓN NO BASTA · INTERVENIR Y PREDECIR*
- **Metric & Evidence (ES / EN):** `Intervención + predicción + comparación → estructura causal revisable` / *`Intervention + prediction + comparison → reviewable causal structure`* · Evidence: `Explicaciones rivales · cadenas, redes, ciclos y probabilidades` (*Rival explanations · chains, networks, cycles and probabilities*)
- **Spoken Macro Narrative (ES / EN):** La causalidad no se obtiene solo observando correlaciones; distintas inteligencias pueden formular modelos diferentes y someterlos a pruebas comunes. (*EN:* Causality is not obtained just by observing correlations; Different intelligences can formulate different models and subject them to common tests.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` flat 6-hex mandala; lacks directed graphs, closed feedback rings, intervention poke vectors, or multi-level emergence strata. **Target (`vid=3318`):** Construct an interactive 3D causal graph stage of glowing nodes (`sd_sphere`) and directional pulse-capsules.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **cadena lineal** (*linear chain*) | `[TIER-C]` | Una secuencia A → B → C es clara cuando una causa dominante opera en una escala bien separada. / *A sequence A → B → C is clear when a dominant cause operates on a well-separated scale.* | **Target:** fires crisp linear domino pulse `A->B->C` *(Gap: No simple linear A->B->C domino impulse beam.)* |
| `F1` | **red causal** (*causal network*) | `[TIER-C]` | En sistemas complejos, muchas variables se influyen y una explicación puede requerir varios caminos simultáneos. / *In complex systems, many variables influence each other and an explanation may require several simultaneous paths.* | **Target:** expands into a dense 12-node web lighting converging causal corridors *(Gap: No dense interconnected multi-path DAG mesh lighting simultaneous branches.)* |
| `F2` | **retroalimentación** (*feedback*) | `[TIER-C]` | Un efecto puede modificar su propia causa, produciendo estabilidad, crecimiento o ciclos. / *An effect can modify its own cause, producing stability, growth, or cycles.* | **Target:** closes a luminous glowing feedback ring oscillating between stability and runaway flare *(Gap: No self-sustaining cyclic feedback torus feeding back into its origin node.)* |
| `F3` | **causa probabilística** (*probabilistic cause*) | `[TIER-C]` | Una intervención puede cambiar probabilidades sin determinar cada resultado individual. / *An intervention can change probabilities without determining each individual outcome.* | **Target:** splits one intervention spark at `A` into weighted probabilistic `70%/30%` cones *(Gap: No branching probabilistic outcome fan triggered by one intervention spark.)* |
| `F4` | **niveles de explicación** (*levels of explanation*) | `[TIER-C]` | La misma transición puede explicarse por microdinámica, organización funcional o historia evolutiva. / *The same transition can be explained by microdynamics, functional organization, or evolutionary history.* | **Target:** stacks three parallel translucent planes showing micro collisions driving mesoscale vortex driving macro trajectory *(Gap: No stacked 3-plane causal hierarchy (micro-particles -> functional gear -> macro-evolution).)* |
| `F5` | **agencia** (*agency*) | `[TIER-C]` | Atribuir una causa a una decisión depende de cómo se modele al agente, sus opciones y la información disponible. / *Attributing a cause to a decision depends on how the agent, its options, and the information available are modeled.* | **Target:** illuminates an agent prism scanning two simulated future ghosts before firing *(Gap: No goal-directed agent node comparing two future branches before acting.)* |

##### `Z19` `compared-cosmologies` (`vid=3319`) — Cosmologías comparadas (*Comparative cosmologies*) `[TIER-D]`

- **Catalog Metadata:** `scene=19` · `[0.760, 0.800]` · `rgb=[245, 140, 205]` · `sound=(sawtooth/115.45Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *UN UNIVERSO · VARIAS HISTORIAS HACIA SU DESCRIPCIÓN*
- **Metric & Evidence (ES / EN):** `Fenómenos accesibles + instrumentos + conceptos → arquitectura cosmológica` / *`Accessible phenomena + instruments + concepts → cosmological architecture`* · Evidence: `Mismo cosmos · historias de descubrimiento con puntos de partida distintos` (*Same cosmos · discovery stories with different starting points*)
- **Spoken Macro Narrative (ES / EN):** Dos civilizaciones pueden converger en predicciones sobre el universo y aun así considerar fundamentales ideas diferentes. (*EN:* Two civilizations can converge in predictions about the universe and still consider different ideas fundamental.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` flat 6-hex mandala; zero Keplerian solar orbit, tidal ocean wave shell, auroral magnetosphere, cloud-shrouded crystal lattice sky discovery, or cellular-automaton cosmos. **Target (`vid=3319`):** Stage 6 concrete cosmological starting environments inside a central celestial sphere sliced open by `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **desde las estrellas** (*from the stars*) | `[TIER-D]` | Una civilización con cielos claros puede comenzar con movimientos celestes, calendarios y gravedad. / *A civilization with clear skies can begin with celestial movements, calendars and gravity.* | **Target:** crystal clear deep-black vault with glowing Keplerian elliptical orbits & ecliptic calendar ring *(Gap: F0 misses crisp Keplerian elliptical star-planet orbital mechanics & calendar ring.)* |
| `F1` | **desde el océano** (*from the ocean*) | `[TIER-D]` | Mareas, ondas y presión podrían convertir campos y continuidad en metáforas cosmológicas centrales. / *Tides, waves and pressure could turn fields and continuity into central cosmological metaphors.* | **Target:** deep pressurized ocean sphere rippled by tidal bulges and standing wave harmonics *(Gap: F1 misses swirling tidal ocean shell & continuous hydrostatic pressure waves.)* |
| `F2` | **desde el magnetismo** (*from magnetism*) | `[TIER-D]` | Auroras y campos percibidos directamente podrían hacer que las líneas de influencia precedan al concepto de objeto. / *Auroras and directly perceived fields could make the lines of influence precede the concept of the object.* | **Target:** glowing planetary magnetosphere draped in sweeping emerald/violet auroral curtains prior to solid planetary edges *(Gap: F2 misses radiant polar magnetic auroral curtains & flux-line cosmology.)* |
| `F3` | **desde lo microscópico** (*from the microscopic*) | `[TIER-D]` | Un mundo opaco al cielo podría descubrir primero estructura material y solo después inferir un universo exterior. / *A sky-opaque world could first discover material structure and only then infer an external universe.* | **Target:** thick amber cloud shroud masking the sky while a radiant central atomic crystal lattice unfolds upward to breach the clouds *(Gap: F3 misses opaque cloud-shrouded atomic crystal core opening outward to distant stars.)* |
| `F4` | **desde la computación** (*from computation*) | `[TIER-C]` | Una cultura artificial puede describir regularidades como transformaciones de información antes que como sustancias. / *An artificial culture can describe regularities as transformations of information rather than as substances.* | **Target:** crisp glowing voxel/lattice grid executing discrete state updates *(Gap: F4 misses binary voxel/bit cellular-automaton transformation lattice.)* |
| `F5` | **desde la conciencia** (*from consciousness*) | `[TIER-C]` | Otra tradición podría comenzar por la experiencia y preguntar qué condiciones físicas permiten observadores. / *Another tradition might start from experience and ask what physical conditions allow observers.* | **Target:** central luminous awareness halo radiating outward onto cosmic boundary *(Gap: F5 misses luminous observer eye horizon peering outward into physical boundary.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP23-Z15-19)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP23-Z15-19` (Distributed Cognition, Untranslatable Quanta, Plural Math, Causal Topologies & Cosmologies) for HyperCosmosVR Odyssey Act V (`Exp 23: 23_cosmic_perspectives`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z15`->`3315` (`F0..F5`), `Z16`->`3316` (`F0..F5`), `Z17`->`3317` (`F0..F5`), `Z18`->`3318` (`F0..F5`), `Z19`->`3319` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3846`, `3847`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP23-Z15-19`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z15: distributed-intelligences / Distributed intelligences, Z16: untranslatable-concepts / Untranslatable concepts, Z17: multiple-mathematics / Multiple representations of mathematics, Z18: causal-models / Different models of causality, Z19: compared-cosmologies / Comparative cosmologies.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp23-z20-24"></a>
### PKG-ACT5-EXP23-Z20-24: Shared Invariant Truths, Deep Disagreements, Complementarity & Plural Cosmic Aperture

- **Phase:** Phase 1 | **Priority:** `P2` | **Scope:** Exp `23` (`23_cosmic_perspectives`) `Z20..Z24` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L83`: `23 => 230` (single variant across all 25 zones!); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L430-L463` (front-locked 6-Lens Sensory Mandala; `active_lens = (scene+feat+zone)%6` brightens 1 hex lens). Tier breakdown for this batch: **`0A / 2B / 3C / 0D` Zones**, **`0A / 8B / 18C / 4D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `0/5` zones and `0/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3320..=3324` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3848`, `3849` (allocated from `3720 + (23 - 11)*10 + k`; Triangulated Invariant Manifold & 25-Petal Resonance (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z20` | `shared-truths` (*Verdades compartidas* / *Shared truths*) | `[TIER-C]` | `vid=230` (`scene=20`) | **`3320`** | `3848` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z21` | `fundamental-disagreements` (*Desacuerdos fundamentales* / *Fundamental disagreements*) | `[TIER-C]` | `vid=230` (`scene=21`) | **`3321`** | `3849` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x03` (0x01 cutaway, 0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z22` | `complementary-knowledge` (*Conocimiento complementario* / *Complementary knowledge*) | `[TIER-C]` | `vid=230` (`scene=22`) | **`3322`** | `3848` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z23` | `many-observers` (*Realidad observada desde muchas mentes* / *Reality observed from many minds*) | `[TIER-B]` | `vid=230` (`scene=23`) | **`3323`** | `3849` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z24` | `plural-cosmic-perspective` (*Perspectiva cósmica plural* / *Plural cosmic perspective*) | `[TIER-B]` | `vid=230` (`scene=24`) | **`3324`** | `3848` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z20` `shared-truths` (`vid=3320`) — Verdades compartidas (*Shared truths*) `[TIER-C]`

- **Catalog Metadata:** `scene=20` · `[0.800, 0.840]` · `rgb=[205, 155, 255]` · `sound=(sine/119Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *ACUERDO ÚTIL = CONSECUENCIAS QUE AMBOS PUEDEN PROBAR*
- **Metric & Evidence (ES / EN):** `Observaciones traducidas + predicciones coincidentes → estructura común` / *`Translated observations + matching predictions → common structure`* · Evidence: `Convergencia empírica · perspectivas distintas limitadas por el mismo mundo` (*Empirical convergence · different perspectives limited by the same world*)
- **Spoken Macro Narrative (ES / EN):** La perspectiva importa, pero no libera a un modelo de la evidencia: distintas mentes deben poder comparar consecuencias observables. (*EN:* Perspective matters, but it does not free a model from evidence: different minds must be able to compare observable consequences.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` flat 6-hex mandala; zero shared eclipse/spectral-line verification event, coordinate transform invariance test, or independent dual-detector coincidence. **Target (`vid=3320`):** Place an empirical test phenomenon between two dissimilar observer probes (warm organic ring & cool crystalline array).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **regularidad** (*regularity*) | `[TIER-D]` | Si ambas civilizaciones predicen el mismo eclipse o transición atómica, han identificado una relación común. / *If both civilizations predict the same eclipse or atomic transition, they have identified a common relationship.* | **Target:** aligns a stellar eclipse shadow + discrete luminous spectral emission barcode shared across both HUDs *(Gap: F0 misses concrete eclipse alignment or glowing discrete atomic spectral barcode lines.)* |
| `F1` | **invariancia** (*invariance*) | `[TIER-C]` | Una descripción robusta aclara qué cambia con el observador y qué permanece bajo transformaciones definidas. / *A robust description clarifies what changes with the observer and what remains under defined transformations.* | **Target:** rotates both probe frames wildly while the central trefoil invariant stays rock-solid *(Gap: F1 misses two rotated coordinate grids leaving an inner geometric knot invariant.)* |
| `F2` | **replicación** (*replication*) | `[TIER-C]` | Resultados obtenidos con cuerpos, instrumentos y métodos distintos son especialmente fuertes cuando coinciden. / *Results obtained with different bodies, instruments and methods are especially strong when they coincide.* | **Target:** fires mismatched detector rays converging on exact simultaneous peak *(Gap: F2 misses two dissimilar sensors (optical + acoustic/radio) locking onto identical pulse.)* |
| `F3` | **traducción operacional** (*operational translation*) | `[TIER-C]` | Relacionar conceptos con procedimientos permite comparar términos aunque sus significados internos no sean idénticos. / *Relating concepts to procedures allows us to compare terms even if their internal meanings are not identical.* | **Target:** lowers a calibrated glowing operational ruler between mismatched symbols *(Gap: F3 misses physical operational meter bridging mismatched foreign symbols.)* |
| `F4` | **predicción nueva** (*new prediction*) | `[TIER-C]` | La convergencia gana valor cuando produce una consecuencia que ninguna perspectiva había probado antes. / *Convergence gains value when it produces a consequence that no perspective has tested before.* | **Target:** projects a dashed predicted ring that lights up bright gold upon confirmation *(Gap: F4 misses bold predicted marker illuminating in dark empty space before arrival.)* |
| `F5` | **error compartido** (*shared error*) | `[TIER-C]` | El acuerdo no garantiza verdad si ambas partes heredan el mismo supuesto; por eso se buscan pruebas independientes. / *The agreement does not guarantee truth if both parties inherit the same assumption; That is why independent evidence is sought.* | **Target:** sweeps a third orthogonal probe uncovering an unseen hidden plane *(Gap: F5 misses third independent side-beam exposing shared shadow behind two aligned detectors.)* |

##### `Z21` `fundamental-disagreements` (`vid=3321`) — Desacuerdos fundamentales (*Fundamental disagreements*) `[TIER-C]`

- **Catalog Metadata:** `scene=21` · `[0.840, 0.880]` · `rgb=[165, 180, 255]` · `sound=(triangle/122.55Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *NO TODO DESACUERDO DESAPARECE CON MÁS DATOS*
- **Metric & Evidence (ES / EN):** `Evidencia común + supuestos diferentes → desacuerdo explícito y productivo` / *`Common evidence + different assumptions → explicit and productive disagreement`* · Evidence: `Pluralidad sin resolución automática · modelos que organizan distinto` (*Plurality without automatic resolution · models organized differently*)
- **Spoken Macro Narrative (ES / EN):** Algunas preguntas pueden permanecer abiertas incluso tras el contacto; distinguir evidencia, definición y valor evita fabricar consensos falsos. (*EN:* Some questions may remain open even after contact; distinguishing evidence, definition and value avoids creating false consensus.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` static 6-hex mandala; cannot stage split interpretive overlays over identical empirical data. **Target (`vid=3321`):** Split the 3D volume cleanly along a translucent central mirror plane sharing identical central empirical trajectories while the left and right halves project rival metaphysical overlays.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **conciencia fundamental** (*fundamental consciousness*) | `[TIER-C]` | Una civilización puede tratar la experiencia como propiedad básica y otra como proceso emergente. / *One civilization may treat experience as a basic property and another as an emergent process.* | **Target:** paints inner matter lattice glowing from bedrock up vs sparking at crown tip *(Gap: No glowing ground-field vs emergent crown sparks duality over same brain/matter data.)* |
| `F1` | **flujo del tiempo** (*flow of time*) | `[TIER-C]` | Ambas pueden predecir relojes correctamente y discrepar sobre si el presente posee un estatus físico especial. / *Both can predict clocks correctly and disagree about whether the present has special physical status.* | **Target:** shows full crystalline 4D helix left vs traveling bright narrow 'now' slice right *(Gap: No frozen 4D spacetime block crystal vs sweeping bright present-slice cursor.)* |
| `F2` | **realidad del individuo** (*reality of the individual*) | `[TIER-C]` | Una mente colectiva puede considerar al individuo una aproximación útil donde nosotros vemos una entidad primaria. / *A collective mind may consider the individual a useful approximation where we see a primary entity.* | **Target:** blurs individual nodes into continuous fluid left vs sharp separate spheres right *(Gap: No single boundary ring fading to show continuous field density vs sharp nodes.)* |
| `F3` | **matemática descubierta** (*discovered mathematics*) | `[TIER-C]` | El desacuerdo sobre descubrir o crear estructuras matemáticas puede no alterar cálculos, pero sí su interpretación. / *Disagreement over discovering or creating mathematical structures may not alter calculations, but their interpretation may.* | **Target:** illuminates distant external crystal grid vs interior mind-projected lattice *(Gap: No hidden Platonic crystal unveiled in deep space vs sculpted out of mind-core.)* |
| `F4` | **origen cósmico** (*cosmic origin*) | `[TIER-C]` | Modelos compatibles con datos disponibles pueden diferir sobre condiciones iniciales o ciclos anteriores. / *Models compatible with available data may differ over initial conditions or previous cycles.* | **Target:** mirrors identical future expansion cone onto two distinct pre-`t=0` roots (singularity vs cyclic bounce) *(Gap: No identical expanding light-cone fan spliced onto diverging pre-Big-Bang/bounce roots.)* |
| `F5` | **dirección de la inteligencia** (*intelligence direction*) | `[TIER-C]` | No existe garantía de una meta universal hacia la que toda evolución cognitiva deba avanzar. / *There is no guarantee of a universal goal toward which all cognitive evolution must advance.* | **Target:** contrasts multi-directional starburst bush vs illusory single target beacon *(Gap: No branching undirected evolutionary bush vs single converging arrow.)* |

##### `Z22` `complementary-knowledge` (`vid=3322`) — Conocimiento complementario (*Complementary knowledge*) `[TIER-C]`

- **Catalog Metadata:** `scene=22` · `[0.880, 0.920]` · `rgb=[135, 215, 255]` · `sound=(sine/126.1Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *COMBINAR PERSPECTIVAS CAMBIA EL MAPA DEL PROBLEMA*
- **Metric & Evidence (ES / EN):** `Modelo humano + modelo ajeno + contraste → explicación más amplia y nuevas pruebas` / *`Human model + foreign model + contrast → further explanation and new evidence`* · Evidence: `Perspectivas que se corrigen · combinar revela variables ocultas` (*Mutually correcting perspectives · combining them reveals hidden variables*)
- **Spoken Macro Narrative (ES / EN):** Una perspectiva diferente puede reorganizar el conocimiento, no solo añadir datos a las categorías existentes. (*EN:* A different perspective can reorganize knowledge, not just add data to existing categories.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` static 6-hex mandala; lacks interleaved noise-to-signal revelation, domain unification bridge, or hybrid two-civilization observatory. **Target (`vid=3322`):** Pair two semi-transparent intersecting perceptual lenses (`amber` & `cyan`) sweeping across a complex volumetric core.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **sensor alienígena** (*extraterrestrial sensor*) | `[TIER-C]` | Una percepción inaccesible para nosotros puede revelar una estructura que nuestros instrumentos trataban como ruido. / *A perception inaccessible to us can reveal a structure that our instruments treated as noise.* | **Target:** clears chaotic snow into sharp crystal waves inside the cyan overlap beam *(Gap: No turbulent background static resolving into crisp hidden lattice under alien lens filter.)* |
| `F1` | **distinción humana** (*human distinction*) | `[TIER-C]` | Nuestras categorías pueden separar procesos que la otra tradición había agrupado y mostrar una excepción importante. / *Our categories can separate processes that the other tradition had grouped together and show an important exception.* | **Target:** cleaves one merged blob via an amber boundary knife *(Gap: No coarse blurry cluster cleanly cleaved into two sharp distinct populations.)* |
| `F2` | **unificación** (*unification*) | `[TIER-C]` | Su marco conceptual puede conectar campos científicos que nuestra historia desarrolló por separado. / *Its conceptual framework can connect scientific fields that our history developed separately.* | **Target:** pulls two separate side rings inward to lock into one coherent glowing manifold *(Gap: No two distant disconnected orbital rings snapping into a single unified torus.)* |
| `F3` | **caso límite** (*boundary case*) | `[TIER-C]` | Cada modelo puede identificar condiciones extremas donde el otro deja de predecir correctamente. / *Each model can identify extreme conditions where the other fails to predict correctly.* | **Target:** flags glowing red breakdown seams where each single shell thins at extreme scale *(Gap: No glowing model shells peeling apart cleanly at extreme inner/outer radius limits.)* |
| `F4` | **instrumento híbrido** (*hybrid instrument*) | `[TIER-D]` | Combinar sensores y teorías permite diseñar experimentos que ninguna civilización habría imaginado sola. / *Combining sensors and theories makes it possible to design experiments that no civilization would have imagined alone.* | **Target:** welds two disparate dish/antenna rigs into one high-resolution central interferometer *(Gap: F4 lacks concrete hybrid cross-species telescope/detector array fusing two optical paths.)* |
| `F5` | **predicción conjunta** (*joint prediction*) | `[TIER-C]` | La prueba más fuerte de integración es una predicción nueva, precisa y arriesgada que después se observa. / *The strongest test of integration is a new, precise, and bold prediction that is later observed.* | **Target:** locks joint triangulation beams onto an unpredicted distant flare *(Gap: No precision joint crosshair target igniting far outside prior single-model bounds.)* |

##### `Z23` `many-observers` (`vid=3323`) — Realidad observada desde muchas mentes (*Reality observed from many minds*) `[TIER-B]`

- **Catalog Metadata:** `scene=23` · `[0.920, 0.960]` · `rgb=[180, 235, 245]` · `sound=(sawtooth/129.65Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *MISMO FENÓMENO · REPRESENTACIONES PARCIALMENTE SUPERPUESTAS*
- **Metric & Evidence (ES / EN):** `Misma realidad → observaciones parciales → modelos comparables → comprensión ampliada` / *`Same reality → partial observations → comparable models → expanded understanding`* · Evidence: `Representaciones superpuestas · un fenómeno, varios espacios cognitivos` (*Overlapping representations · one phenomenon, several cognitive spaces*)
- **Spoken Macro Narrative (ES / EN):** Las representaciones pueden solaparse sin volverse idénticas; su comparación muestra tanto el fenómeno como los límites de cada observador. (*EN:* Representations can overlap without becoming identical; Their comparison shows both the phenomenon and the limits of each observer.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` literally places 1 central radiant star (`center_eye`) surrounded by 6 converging sight-ray hex lenses (`k=0..5`), matching the 1-star/6-observer topology! However active_lens=(23+feat+23)%6 is misaligned (`feat=0` lights `k=4`) and lens patterns `k=0..5` are fixed generic motifs rather than distinct biological/AI/ecosystem/timescale views of the star. **Target (`vid=3323`):** Align `active_lens = feat % 6u` directly on the existing `center_eye` star + 6 hex apertures, making the central star visibly shift spectral corona detail while each lens `k=0..5` renders its true observer-specific star projection.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **individuo biológico** (*biological individual*) | `[TIER-B]` | La estrella aparece como luz, calor, orientación, peligro y memoria personal. / *The star appears as light, heat, guidance, danger and personal memory.* | **Target:** warm solar halo + visual color spectrum + retinal flare *(Gap: F0 biological individual (light/warmth/memory) lights generic k=4 dot matrix.)* |
| `F1` | **mente colectiva** (*collective mind*) | `[TIER-B]` | El fenómeno se representa como cambios distribuidos en muchas regiones y generaciones. / *The phenomenon is represented as changes distributed over many regions and generations.* | **Target:** multi-node hexagonal distribution mesh tracking slow star arcs *(Gap: F1 collective mind (distributed nodes/generations) lights generic k=5 square grid.)* |
| `F2` | **inteligencia artificial** (*artificial intelligence*) | `[TIER-B]` | La estrella puede ser un estado probabilístico actualizado continuamente por flujos de datos. / *The star may be a probabilistic state continually updated by data streams.* | **Target:** shimmering volumetric Gaussian posterior probability cloud around the star *(Gap: F2 AI (probabilistic data-stream tensor) lights generic k=0 wave rings.)* |
| `F3` | **ecosistema planetario** (*planetary ecosystem*) | `[TIER-B]` | Su significado se expresa mediante química, clima, ciclos y supervivencia de redes vivas. / *Its meaning is expressed through chemistry, climate, cycles and survival of living networks.* | **Target:** chlorophyll-emerald carbon/climate sinusoidal tide ring driven by stellar flux *(Gap: F3 planetary ecosystem (chemistry/climate/cycles) lights generic k=1 single ring.)* |
| `F4` | **civilización lenta** (*slow civilization*) | `[TIER-B]` | La estrella es una secuencia histórica cuyos cambios se comparan durante miles de años. / *The star is a historical sequence whose changes are compared over thousands of years.* | **Target:** ultra-slow evolving stellar HR-track ribbon over millennia *(Gap: F4 slow millennial civilization (secular star-drift arc) lights generic k=2 Gaussian dot.)* |
| `F5` | **máquina acelerada** (*accelerated machine*) | `[TIER-B]` | La misma observación contiene enormes cantidades de microvariación procesadas antes de un pensamiento humano. / *The same observation contains enormous amounts of microvariation processed before a human thought.* | **Target:** ultra-high-frequency shimmering asteroseismic micro-pulsation matrix (`doc_cue` modulated) *(Gap: F5 accelerated machine (dense micro-variation harmonics) lights generic k=3 stripes.)* |

##### `Z24` `plural-cosmic-perspective` (`vid=3324`) — Perspectiva cósmica plural (*Plural cosmic perspective*) `[TIER-B]`

- **Catalog Metadata:** `scene=24` · `[0.960, 1.000]` · `rgb=[240, 245, 255]` · `sound=(sine/133.2Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *NINGUNA VISTA DESDE NINGUNA PARTE · MUCHAS VISTAS QUE SE CORRIGEN*
- **Metric & Evidence (ES / EN):** `Perspectivas limitadas + evidencia compartida + crítica mutua → conocimiento transformable` / *`Limited perspectives + shared evidence + mutual criticism → transformable knowledge`* · Evidence: `Destino · ninguna inteligencia posee una vista desde ninguna parte` (*Destination · no intelligence has a view from nowhere*)
- **Spoken Macro Narrative (ES / EN):** Comprender el universo puede consistir en integrar perspectivas parciales capaces de corregirse, ampliarse y cambiarse mutuamente sin perder el criterio de evidencia. (*EN:* Understanding the universe can consist of integrating partial perspectives capable of correcting, expanding and changing each other without losing the criterion of evidence.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=230` ring of 6 situated radial lenses with empty exterior void matches 'many situated views, no outside God's-eye view', but `doc_feat=0..5` just cycles brightness blindly (`k=(24+feat+24)%6 = feat`!). **Target (`vid=3324`):** Keep the center-less 6-lens ring (`center_eye` dims slightly to emphasize open shared sky) and upgrade `doc_feat=0..5` into Act 23 finale beats.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **cuerpo** (*body*) | `[TIER-B]` | Toda observación comienza desde alguna forma de encarnación, sensor o interfaz con el entorno. / *All observation begins from some form of embodiment, sensor, or interface with the environment.* | **Target:** illuminates tactile/embodied boundary rings around each observer hub *(Gap: Lenses cycle cleanly k=0..5 (`(24+feat+24)%6 == feat`), yet lack sensory/embodiment form.)* |
| `F1` | **cultura** (*culture*) | `[TIER-C]` | Los conceptos heredados orientan las preguntas posibles y las explicaciones que primero parecen naturales. / *Inherited concepts guide the possible questions and explanations that first seem natural.* | **Target:** rotates prismatic conceptual grid lines inside lenses *(Gap: No cultural conceptual lattice prism modulating lens orientation.)* |
| `F2` | **época cósmica** (*cosmic epoch*) | `[TIER-D]` | Cada civilización observa un universo en una etapa concreta y quizá nunca vea fenómenos disponibles en otras eras. / *Each civilization observes a universe at a particular stage and may never see phenomena available in other eras.* | **Target:** sweeps an expanding epoch shell showing early bright galaxies fading toward distant horizons *(Gap: F2 misses concrete cosmic-epoch horizon shell (ancient CMB vs future empty sky).)* |
| `F3` | **instrumentos** (*instruments*) | `[TIER-D]` | Extender los sentidos abre ventanas nuevas, pero cada ventana conserva resolución, ruido y supuestos. / *Extending the senses opens new windows, but each window retains resolution, noise, and assumptions.* | **Target:** adds realistic Airy diffraction rings + additive grain inside lens glass *(Gap: F3 misses telescope aperture, diffraction ring, and noise-floor distortion inside lens.)* |
| `F4` | **recursos finitos** (*finite resources*) | `[TIER-C]` | Ninguna mente, por vasta que sea, puede representar cada detalle del cosmos con precisión ilimitada. / *No mind, however vast, can represent every detail of the cosmos with unlimited precision.* | **Target:** throttles sight-ray flow with crisp packet truncation ticks *(Gap: No memory/bandwidth saturation cap pinching incoming sight-ray density.)* |
| `F5` | **pluralidad crítica** (*critical plurality*) | `[TIER-B]` | La diversidad amplía el conocimiento cuando las perspectivas pueden traducirse parcialmente, probarse y corregirse. / *Diversity expands knowledge when perspectives can be partially translated, tested, and corrected.* | **Target:** ignites golden cross-bridge arcs weaving all 6 hex frames into one self-correcting ring (`lotus_alpha` sky coupling) *(Gap: No mutual cross-lens calibration arcs bridging k=0..5 around outer ring.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP23-Z20-24)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP23-Z20-24` (Shared Invariant Truths, Deep Disagreements, Complementarity & Plural Cosmic Aperture) for HyperCosmosVR Odyssey Act V (`Exp 23: 23_cosmic_perspectives`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z20`->`3320` (`F0..F5`), `Z21`->`3321` (`F0..F5`), `Z22`->`3322` (`F0..F5`), `Z23`->`3323` (`F0..F5`), `Z24`->`3324` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3848`, `3849`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP23-Z20-24`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z20: shared-truths / Shared truths, Z21: fundamental-disagreements / Fundamental disagreements, Z22: complementary-knowledge / Complementary knowledge, Z23: many-observers / Reality observed from many minds, Z24: plural-cosmic-perspective / Plural cosmic perspective.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

### 4.7 Experience 24: `24_cosmic_meaning` — Espectro del Significado Cósmico: de un Universo Indiferente a la Construcción de Propósito

- **Odyssey & Spatial Placement:** Act 5 (`7/8`, overall stop #24) · `HYPER_LOTUS_24_MEANING` (`parent UNIVERSE_ROOT`, `u = [27.2, 28.6]`, `focus_scale_exp = 27.6`, `6.0e27 m`; note `topology.rs#L379` reads `"Sentido Cósmico"` vs catalog `"Significado cósmico"`) ([`topology.rs#L377-L388, L656`](../../crates/cosmos_spatial/src/topology.rs#L377-L388))
- **Experience Totals:** `26` Zones (`0A / 6B / 15C / 5D`) · `156` Features (`0A / 29B / 96C / 31D`)
- **Current Shader & Compute:** `doc_stages.rs#L84-L87`: `24 => { 24 => 241, _ => 240 }` (all `Z00..Z23, Z25` share `240`) · `shaders/hero_doc_civilization_horizon.wgsl#L464-L494` (front-locked Pale Blue Mote + twin Astrolabe Shields + Golden Ethical Balance Scales; `tilt = sin(0.8*t + 0.85*feat)*0.10` or static `0.22` at `Z24`) · `mode=15` (`universe_scene.rs#L323`, `compute_universe.wgsl#L347-L357` 25-petal lotus) + `far_shell.wgsl#L150-L161` (`lotus_boost=0.42`)

<a id="pkg-act5-exp24-z00-04"></a>
### PKG-ACT5-EXP24-Z00-04: Plural Value Origin, Observerless Void, Emergent Life, Subjective Experience & Preference

- **Phase:** Phase 1/2 | **Priority:** `P1` | **Scope:** Exp `24` (`24_cosmic_meaning`) `Z00..Z04` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L84-L87`: `24 => { 24 => 241, _ => 240 }` (all `Z00..Z23, Z25` share `240`); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L464-L494` (front-locked Pale Blue Mote + twin Astrolabe Shields + Golden Ethical Balance Scales; `tilt = sin(0.8*t + 0.85*feat)*0.10` or static `0.22` at `Z24`). Tier breakdown for this batch: **`0A / 1B / 2C / 2D` Zones**, **`0A / 6B / 14C / 10D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `0/5` zones and `0/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3400..=3404` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3850`, `3851` (allocated from `3720 + (24 - 11)*10 + k`; Void -> Negentropy & Valence Pulse Contrast (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z00` | `plural-perspective` (*Perspectiva cósmica plural* / *Plural cosmic perspective*) | `[TIER-C]` | `vid=240` | **`3400`** | `3850` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z01` | `observerless-universe` (*Un universo sin observadores* / *A universe without observers*) | `[TIER-D]` | `vid=240` | **`3401`** | `3851` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z02` | `emergence-difference` (*La aparición de la diferencia* / *The emergence of difference*) | `[TIER-D]` | `vid=240` | **`3402`** | `3850` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z03` | `experience` (*Sentir convierte estados en experiencias* / *Feeling converts states into experiences*) | `[TIER-C]` | `vid=240` | **`3403`** | `3851` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x01` (0x01 cutaway) | `0..5` (`0xFF` macro) |
| `Z04` | `preference-value` (*De la preferencia al valor* / *From preference to value*) | `[TIER-B]` | `vid=240` | **`3404`** | `3850` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z00` `plural-perspective` (`vid=3400`) — Perspectiva cósmica plural (*Plural cosmic perspective*) `[TIER-C]`

- **Catalog Metadata:** `scene=0` · `[0.000, 0.038]` · `rgb=[190, 220, 255]` · `sound=(sine/46.2Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *MUCHAS PERSPECTIVAS · MUCHAS FORMAS DE IMPORTAR*
- **Metric & Evidence (ES / EN):** `Perspectiva situada + experiencia + historia + relación → algo puede importar de maneras diferentes` / *`Situated perspective + experience + history + relationship → something can matter in different ways`* · Evidence: `Continuidad · muchas mentes observan y valoran desde posiciones distintas` (*Continuity · many minds observe and evaluate from different positions*)
- **Spoken Macro Narrative (ES / EN):** Ninguna inteligencia observa desde un punto completamente neutral; cada forma de comprender también selecciona aquello que considera relevante. (*EN:* No intelligence observes from a completely neutral point; Each way of understanding also selects what it considers relevant.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` static golden ethical balance scale + pale blue mote; ignores `doc_zone`, `doc_cue`; `doc_feat` only phase-shifts gentle 0.10*sin tilt. **Target (`vid=3400`):** Bridge Exp 23's radial lens into Exp 24 by rendering an asymmetric situated observer node firing a branching fan of future trajectories (`sd_capsule` splines) through translucent filters.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **cuerpo o sustrato** (*body or substrate*) | `[TIER-C]` | Toda perspectiva comienza en una forma física que permite algunas experiencias, acciones y vulnerabilidades mientras excluye otras. / *Every perspective begins in a physical form that allows for some experiences, actions, and vulnerabilities while excluding others.* | **Target:** organic/synthetic frame *(Gap: No visible situated body/substrate anchor separating vulnerabilities & affordances.)* |
| `F1` | **escala temporal** (*time scale*) | `[TIER-C]` | Lo urgente, lo duradero y lo irreparable dependen del ritmo cognitivo y de la duración de una vida o civilización. / *What is urgent, what is lasting and what is irreparable depends on the cognitive rhythm and the duration of a life or civilization.* | **Target:** differential orbital clock rate *(Gap: No contrasting fast/slow subjective tempo clock rings.)* |
| `F2` | **historia** (*history*) | `[TIER-C]` | Memorias, pérdidas y descubrimientos convierten el mismo presente en situaciones distintas para observadores distintos. / *Memories, losses and discoveries turn the same present into different situations for different observers.* | **Target:** glowing memory-wake behind node *(Gap: No scarred/layered historical trail differentiating equal present states.)* |
| `F3` | **relaciones** (*relationships*) | `[TIER-C]` | Aquello que importa suele depender de vínculos con otros seres, lugares, proyectos y futuros posibles. / *What matters usually depends on links with other beings, places, projects and possible futures.* | **Target:** tethering filaments to satellite orbs *(Gap: No relational bond filaments linking observer to cared-for external nodes.)* |
| `F4` | **conocimiento limitado** (*limited knowledge*) | `[TIER-C]` | Valorar también ocurre bajo incertidumbre: ninguna mente conoce todas las consecuencias de sus decisiones. / *Valuing also occurs under uncertainty: no mind knows all the consequences of its decisions.* | **Target:** volumetric fog veil at distal tips *(Gap: No foggy uncertainty horizon shrouding distal future branches.)* |
| `F5` | **capacidad de elegir** (*ability to choose*) | `[TIER-C]` | Cuando una inteligencia puede comparar futuros y actuar, una perspectiva se transforma en orientación. / *When an intelligence can compare futures and act, a perspective becomes orientation.* | **Target:** highlighting one chosen trajectory branch in bright gold (`zone_col`) *(Gap: No branching future fork illuminating selectable trajectories.)* |

##### `Z01` `observerless-universe` (`vid=3401`) — Un universo sin observadores (*A universe without observers*) `[TIER-D]`

- **Catalog Metadata:** `scene=1` · `[0.038, 0.077]` · `rgb=[155, 185, 230]` · `sound=(triangle/49.55Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *ACONTECIMIENTOS SIN TESTIGOS · LA PREGUNTA PERMANECE*
- **Metric & Evidence (ES / EN):** `Campos + partículas + energía + tiempo → cambios sin finalidad visible` / *`Fields + particles + energy + time → changes without visible purpose`* · Evidence: `Realidad física · acontecimientos sin testigos, preferencia ni pérdida` (*Physical reality · events without witnesses, preference or loss*)
- **Spoken Macro Narrative (ES / EN):** Las estrellas pueden nacer y morir sin que el proceso distinga entre éxito y fracaso; la pregunta por el significado todavía permanece abierta. (*EN:* Stars can be born and die without the process distinguishing between success and failure; The question of meaning still remains open.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` draws a Golden Balance Scale & Pale Blue Mote Shield—directly contradicting 'Un universo sin observadores / acontecimientos sin testigos, preferencia ni pérdida' and rendering zero astrophysics. **Target (`vid=3401`):** Replace balance scales with a stark cold astrophysical void (`oled_void_clamp` black + sterile blue-white star dust).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **expansión** (*expansion*) | `[TIER-D]` | El espacio cambia de escala sin mostrar preferencia por una distribución concreta de galaxias. / *Space changes scale without showing a preference for a specific distribution of galaxies.* | **Target:** radially drifting isotropic galaxy filaments (`compute` scatter mode) *(Gap: F0 misses homogeneous dark expanding cosmic metric & receding cold filaments.)* |
| `F1` | **formación estelar** (*star formation*) | `[TIER-D]` | La gravedad concentra materia y enciende estrellas sin necesidad de intención ni testigo. / *Gravity concentrates matter and ignites stars without the need for intention or witness.* | **Target:** swirling Jeans-unstable dust torus collapsing into a crisp golden fusion pinpoint *(Gap: F1 misses gravitational gas-cloud collapse & stellar ignition core.)* |
| `F2` | **destrucción** (*destruction*) | `[TIER-D]` | Una supernova reorganiza materia; sin experiencia consciente, no existe todavía una pérdida sentida. / *A supernova reorganizes matter; Without conscious experience, there is no felt loss yet.* | **Target:** sharp Sedov-Taylor shock ring (`sd_sphere` thin expanding shell) shredding a red supergiant silently *(Gap: F2 misses symmetric supernova shock shell expanding blindly without witnesses.)* |
| `F3` | **entropía** (*entropy*) | `[TIER-D]` | Los estados accesibles cambian y la energía útil se dispersa, pero la física no llama deseable a una dirección. / *The accessible states change and useful energy is dispersed, but physics does not call one direction desirable.* | **Target:** hot localized gradient smoothing outward into uniform dim noise *(Gap: F3 misses irreversible turbulent thermal dispersion from hot core into void.)* |
| `F4` | **causalidad** (*causality*) | `[TIER-C]` | Los acontecimientos producen consecuencias aunque nadie los recuerde ni los interprete. / *Events produce consequences even if no one remembers or interprets them.* | **Target:** cold blind gravitational wave ripples passing through inert boulders *(Gap: F4 blind causal billiard/wave propagation without observer scales.)* |
| `F5` | **pregunta abierta** (*open question*) | `[TIER-C]` | Que no veamos un propósito físico explícito no demuestra por sí solo que toda forma de significado sea imposible. / *That we do not see an explicit physical purpose does not in itself prove that all forms of meaning are impossible.* | **Target:** faint open threshold halo inviting later emergent observers *(Gap: F5 unclosed philosophical threshold question left dark beside scales.)* |

##### `Z02` `emergence-difference` (`vid=3402`) — La aparición de la diferencia (*The emergence of difference*) `[TIER-D]`

- **Catalog Metadata:** `scene=2` · `[0.077, 0.115]` · `rgb=[120, 210, 190]` · `sound=(sine/52.9Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *CONTINUAR Y DESAPARECER DEJAN DE SER EQUIVALENTES*
- **Metric & Evidence (ES / EN):** `Organización + metabolismo + reparación + reproducción → diferencia funcional` / *`Organization + metabolism + repair + reproduction → functional difference`* · Evidence: `Vida incipiente · algunos estados conservan organización y otros la destruyen` (*Incipient life · some states preserve organization and others destroy it*)
- **Spoken Macro Narrative (ES / EN):** Con la vida aparece una distinción práctica entre condiciones que permiten continuar y condiciones que conducen a desaparecer. (*EN:* With life there appears a practical distinction between conditions that allow it to continue and conditions that lead to its disappearance.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` golden balance scale + cyan mote rings; zero lipid membrane, nutrient/toxin chemistry, active DNA/membrane repair, chemotaxis, mitosis, or dissolving lineage. **Target (`vid=3402`):** Sculpt a living radiant protocell SDF (`sd_sphere` double bilayer + undulating internal metabolic torus).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **nutriente** (*nutrient*) | `[TIER-D]` | Ciertas moléculas pueden incorporarse a procesos que mantienen estructura y actividad. / *Certain molecules can be incorporated into processes that maintain structure and activity.* | **Target:** warm golden particles crossing glowing pore channels into vibrant core *(Gap: F0 misses selective membrane transport drawing bright nutrient molecules inward.)* |
| `F1` | **veneno** (*poison*) | `[TIER-D]` | Otras interacciones interrumpen funciones y reducen la posibilidad de continuidad. / *Other interactions interrupt functions and reduce the possibility of continuity.* | **Target:** spiky dark-violet toxin shards dimming & warping the outer wall *(Gap: F1 misses jagged toxic intruder rupturing/degrading metabolic torus.)* |
| `F2` | **reparación** (*repair*) | `[TIER-D]` | Un sistema vivo utiliza energía para corregir daños y conservar una organización improbable. / *A living system uses energy to correct damage and maintain unlikely organization.* | **Target:** counter-flowing energy repair arcs sealing fissures *(Gap: F2 misses ATP-like energy sparks actively stitching breached membrane walls.)* |
| `F3` | **señal útil** (*useful signal*) | `[TIER-D]` | Una variación ambiental se vuelve relevante cuando permite anticipar alimento, peligro o cambio. / *An environmental variation becomes relevant when it allows us to anticipate food, danger or change.* | **Target:** coherent wave gradient bending chemotactic trail toward shelter *(Gap: F3 misses asymmetric gradient wave steering motile protocell toward nutrient/away from cliff.)* |
| `F4` | **reproducción** (*reproduction*) | `[TIER-D]` | La continuidad puede extenderse más allá de un organismo mediante copias con variación. / *Continuity can extend beyond an organism through copies with variation.* | **Target:** figure-8 pinching mitosis (`smin` twin spheres) budding a hue-shifted daughter sphere *(Gap: F4 misses cell division mitosis splitting one vibrating membrane into slightly varied twins.)* |
| `F5` | **extinción** (*extinction*) | `[TIER-D]` | Cuando una línea termina, desaparecen capacidades y futuros que esa organización podía producir. / *When a line ends, capabilities and futures that that organization could produce disappear.* | **Target:** branching lineage filaments tapering to sudden quenched dark void *(Gap: F5 misses fragile branching lineage tree abruptly quenching into sterile dark ash.)* |

##### `Z03` `experience` (`vid=3403`) — Sentir convierte estados en experiencias (*Feeling converts states into experiences*) `[TIER-C]`

- **Catalog Metadata:** `scene=3` · `[0.115, 0.154]` · `rgb=[120, 225, 235]` · `sound=(sawtooth/56.25Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *ALGO OCURRE PARA ALGUIEN*
- **Metric & Evidence (ES / EN):** `Estado corporal + integración nerviosa + memoria → experiencia con valencia` / *`Body state + nervous integration + memory → experience with valence`* · Evidence: `Conciencia · algo ya no solo ocurre, sino que ocurre para alguien` (*Consciousness · something no longer just happens, but happens for someone.*)
- **Spoken Macro Narrative (ES / EN):** El sufrimiento y el bienestar introducen una dimensión que no se agota describiendo masa, posición o energía. (*EN:* Suffering and well-being introduce a dimension that is not exhausted by describing mass, position or energy.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` golden balance scales rock blindly at +/-5.7 deg; fails to convey inner subjective interiority or the 6 contrasting affective valences. **Target (`vid=3403`):** Build a luminous sentient core suspended inside an interior horizon dome (`mote` upgraded to layered phenomenological core).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **dolor** (*pain*) | `[TIER-C]` | El daño puede convertirse en una experiencia negativa que exige atención y modifica decisiones. / *Harm can become a negative experience that demands attention and changes decisions.* | **Target:** crimson high-frequency constriction spike piercing core *(Gap: F0 pain lacks sharp jagged crimson inward constriction signal.)* |
| `F1` | **placer** (*pleasure*) | `[TIER-C]` | Algunos estados se experimentan como positivos y favorecen repetición, exploración o vínculo. / *Some states are experienced as positive and encourage repetition, exploration, or bonding.* | **Target:** golden smooth expansive harmonic halo *(Gap: F1 pleasure lacks warm blooming harmonious resonance torus.)* |
| `F2` | **miedo** (*fear*) | `[TIER-C]` | Una posibilidad futura puede importar antes de ocurrir mediante anticipación y memoria. / *A future possibility can matter before it occurs through anticipation and memory.* | **Target:** cold violet threat shadow projecting inward from future arc *(Gap: F2 fear lacks dark shadow-looming future branch anticipation.)* |
| `F3` | **curiosidad** (*curiosity*) | `[TIER-C]` | La falta de información puede sentirse como una invitación a explorar, no solo como ausencia de datos. / *The lack of information can feel like an invitation to explore, not just the absence of data.* | **Target:** bright cyan searchlight cone sweeping dark boundary void *(Gap: F3 curiosity lacks luminous outward seeking ray probing an occluded void.)* |
| `F4` | **duelo** (*grief*) | `[TIER-C]` | Perder a otro modifica el mundo vivido porque relaciones y futuros compartidos dejan de ser posibles. / *Losing another changes the lived world because relationships and shared futures are no longer possible.* | **Target:** severed tether ring glowing beside a hollow void footprint *(Gap: F4 grief lacks severed glowing bond hanging over an empty extinguished space.)* |
| `F5` | **asombro** (*amazement*) | `[TIER-C]` | Una experiencia puede ser valiosa sin resolver una necesidad inmediata ni producir una utilidad externa. / *An experience can be valuable without solving an immediate need or producing external utility.* | **Target:** tranquil iridescent concentric aurora blooming purely for contemplation *(Gap: F5 amazement lacks serene non-convergent fractal aurora bloom.)* |

##### `Z04` `preference-value` (`vid=3404`) — De la preferencia al valor (*From preference to value*) `[TIER-B]`

- **Catalog Metadata:** `scene=4` · `[0.154, 0.192]` · `rgb=[150, 220, 160]` · `sound=(sine/59.6Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *PREFERIR PUEDE CONVERTIRSE EN PRINCIPIO*
- **Metric & Evidence (ES / EN):** `Preferencia + memoria + comparación + reflexión → valor revisable` / *`Preference + memory + comparison + reflection → reviewable value`* · Evidence: `Orientación estable · comparar impulsos, consecuencias y principios` (*Stable orientation · compare impulses, consequences and principles*)
- **Spoken Macro Narrative (ES / EN):** Un valor guía decisiones en situaciones distintas y puede incluso oponerse a un deseo inmediato. (*EN:* A value guides decisions in different situations and can even oppose an immediate desire.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` Ethical Balance Scales matches the metaphorical weighing of principles vs impulses (`comparar impulsos, consecuencias y principios`), though `doc_feat=0..5` only phases tilt slightly without asymmetric weight badges or opposing principle anchors. **Target (`vid=3404`):** Retain legacy `vid=240`'s iconic golden scale armature while driving asymmetric pan weights & beam dynamics per `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **supervivencia** (*survival*) | `[TIER-B]` | Conservar la propia existencia puede ser fundamental sin convertirse automáticamente en el único bien. / *Preserving one's existence can be fundamental without automatically becoming the only good.* | **Target:** heavy obsidian survival anchor orb balanced cleanly against lighter higher ideals *(Gap: Scales rock symmetrically without highlighting base survival weight vs higher goods.)* |
| `F1` | **verdad** (*truth*) | `[TIER-B]` | Una mente puede preferir una creencia incómoda pero mejor sustentada a una ilusión tranquilizadora. / *A mind may prefer an uncomfortable but better-supported belief to a reassuring illusion.* | **Target:** sharp piercing white prism outweighing comforting dim haze *(Gap: beam/pans show identical golden orbs rather than glowing stark truth crystal vs soothing veil.)* |
| `F2` | **libertad** (*freedom*) | `[TIER-B]` | Poder elegir y revisar metas puede valorarse aun cuando produzca incertidumbre o riesgo. / *Being able to choose and review goals can be valued even when they produce uncertainty or risk.* | **Target:** vibrant branching star-tree lifting off pan vs static cage *(Gap: No turbulent open choice sphere vs rigid certainty cage on opposing pans.)* |
| `F3` | **lealtad** (*loyalty*) | `[TIER-B]` | Los compromisos pueden mantener relaciones a través de cambios, costes y tentaciones. / *Commitments can sustain relationships through change, costs, and temptations.* | **Target:** anchored bright chain bonding beam across turbulence *(Gap: No steadfast unbroken ring anchor resisting storm drift.)* |
| `F4` | **armonía** (*harmony*) | `[TIER-B]` | Reducir conflicto y conservar equilibrio puede priorizarse frente a expansión o innovación. / *Reducing conflict and preserving balance can be prioritized over expansion or innovation.* | **Target:** beam settling to crisp horizontal `tilt = 0` harmonic balance (`smoothstep`) *(Gap: No harmonic damped counterweights leveling beam into calm equilibrium.)* |
| `F5` | **transformación** (*transformation*) | `[TIER-B]` | Algunas inteligencias pueden valorar cambiar continuamente más que preservar una identidad estable. / *Some intelligences may value continually changing more than preserving a stable identity.* | **Target:** right pan crystallizing into continuously morphing multi-hued polyhedron *(Gap: No morphing iridescent crystal tipping away from static gold orb.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP24-Z00-04)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP24-Z00-04` (Plural Value Origin, Observerless Void, Emergent Life, Subjective Experience & Preference) for HyperCosmosVR Odyssey Act V (`Exp 24: 24_cosmic_meaning`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z00`->`3400` (`F0..F5`), `Z01`->`3401` (`F0..F5`), `Z02`->`3402` (`F0..F5`), `Z03`->`3403` (`F0..F5`), `Z04`->`3404` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3850`, `3851`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP24-Z00-04`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z00: plural-perspective / Plural cosmic perspective, Z01: observerless-universe / A universe without observers, Z02: emergence-difference / The emergence of difference, Z03: experience / Feeling converts states into experiences, Z04: preference-value / From preference to value.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp24-z05-09"></a>
### PKG-ACT5-EXP24-Z05-09: Agency Goal Cones, Personal Story, Shared Culture, Civilizational Myths & Mortality Contrast

- **Phase:** Phase 1/2 | **Priority:** `P2` | **Scope:** Exp `24` (`24_cosmic_meaning`) `Z05..Z09` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L84-L87`: `24 => { 24 => 241, _ => 240 }` (all `Z00..Z23, Z25` share `240`); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L464-L494` (front-locked Pale Blue Mote + twin Astrolabe Shields + Golden Ethical Balance Scales; `tilt = sin(0.8*t + 0.85*feat)*0.10` or static `0.22` at `Z24`). Tier breakdown for this batch: **`0A / 0B / 4C / 1D` Zones**, **`0A / 1B / 25C / 4D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `0/5` zones and `0/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3405..=3409` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3852`, `3853` (allocated from `3720 + (24 - 11)*10 + k`; Branching Agency Worldlines & Finite Hourglass (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z05` | `goals-agency` (*Metas y agencia* / *Goals and agency*) | `[TIER-C]` | `vid=240` | **`3405`** | `3852` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z06` | `personal-meaning` (*El significado personal* / *The personal meaning*) | `[TIER-C]` | `vid=240` | **`3406`** | `3853` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z07` | `shared-meaning` (*Significado compartido* / *Shared meaning*) | `[TIER-C]` | `vid=240` | **`3407`** | `3852` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z08` | `narratives-civilizations` (*Relatos y civilizaciones* / *Stories and civilizations*) | `[TIER-C]` | `vid=240` | **`3408`** | `3853` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z09` | `mortality-limit` (*Mortalidad y límite* / *Mortality and limit*) | `[TIER-D]` | `vid=240` | **`3409`** | `3852` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z05` `goals-agency` (`vid=3405`) — Metas y agencia (*Goals and agency*) `[TIER-C]`

- **Catalog Metadata:** `scene=5` · `[0.192, 0.231]` · `rgb=[205, 210, 125]` · `sound=(triangle/62.95Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *REPRESENTAR FUTUROS CONVIERTE POSIBILIDADES EN METAS*
- **Metric & Evidence (ES / EN):** `Modelo del futuro + selección + acción + corrección → agencia` / *`Model of the future + selection + action + correction → agency`* · Evidence: `Futuros representados · actuar para acercar unas posibilidades y evitar otras` (*Futures represented · act to bring some possibilities closer and avoid others*)
- **Spoken Macro Narrative (ES / EN):** Tener una función no equivale necesariamente a comprenderla; una mente puede aceptar, rechazar o redefinir la tarea que recibió. (*EN:* Having a function does not necessarily equate to understanding it; a mind can accept, reject, or redefine the task it was given.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` generic rocking scales; zero forward-projected future cones, telescope monument, or closed vs open optimizer distinction. **Target (`vid=3405`):** Project a luminous cone of branching possible future timelines ahead of a small agent mote.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **refugio** (*shelter*) | `[TIER-C]` | Un organismo coordina percepción y movimiento hacia una condición futura más segura. / *An organism coordinates perception and movement toward a safer future condition.* | **Target:** curved shelter arc shielding agent trail *(Gap: No organism steering toward glowing protective refuge dome.)* |
| `F1` | **plan de vida** (*life plan*) | `[TIER-C]` | Una persona conecta decisiones presentes con una historia todavía inexistente. / *A person connects present decisions with a history that does not yet exist.* | **Target:** beaded temporal trajectory threading long-arc checkpoints *(Gap: No connected milestone spine threading present to distant horizon.)* |
| `F2` | **telescopio** (*telescope*) | `[TIER-D]` | Una civilización dedica generaciones a construir una capacidad de conocer más allá de su mundo. / *A civilization spends generations building a capacity to know beyond its world.* | **Target:** deploy a faceted orbital/ground telescope mirror peering across generations into deep space *(Gap: F2 explicitly names building a multi-generational telescope beyond home world—no optical dish/array rendered.)* |
| `F3` | **proyecto artificial** (*artificial project*) | `[TIER-C]` | Un sistema diseñado puede optimizar una meta sin experimentar por qué debería importar. / *A designed system can optimize a goal without experimenting with why it should matter.* | **Target:** cold clockwork vector arrow piercing dead target ring without inner subjective glow *(Gap: No hollow mechanical arrow optimizing target absent glowing inner mote.)* |
| `F4` | **revisión** (*review*) | `[TIER-C]` | La agencia aumenta cuando un sistema puede evaluar y modificar los objetivos que orientan su conducta. / *Agency increases when a system can evaluate and modify the objectives that guide its behavior.* | **Target:** radiant meta-feedback halo redirecting goal beacon *(Gap: No meta-loop bending back to re-target its own primary attractor beam.)* |
| `F5` | **responsabilidad** (*responsibility*) | `[TIER-C]` | Representar consecuencias convierte la capacidad de actuar en una razón para considerar a quienes serán afectados. / *Representing consequences turns the ability to act into a reason to consider those who will be affected.* | **Target:** glowing causal footprint wave washing over neighbor nodes *(Gap: No concentric ripple radius illuminating downstream beings touched by action.)* |

##### `Z06` `personal-meaning` (`vid=3406`) — El significado personal (*The personal meaning*) `[TIER-C]`

- **Catalog Metadata:** `scene=6` · `[0.231, 0.269]` · `rgb=[245, 190, 120]` · `sound=(sine/66.3Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *UNA VIDA CONECTA EXPERIENCIAS EN UNA DIRECCIÓN*
- **Metric & Evidence (ES / EN):** `Experiencia → interpretación → compromiso → consecuencia → identidad narrativa` / *`Experience → interpretation → commitment → consequence → narrative identity`* · Evidence: `Historia vivida · compromisos que conectan experiencia, acción y memoria` (*Lived history · commitments that connect experience, action and memory*)
- **Spoken Macro Narrative (ES / EN):** Una vida puede adquirir dirección sin que cada acontecimiento haya sido predeterminado por una finalidad externa. (*EN:* A life can acquire direction without each event having been predetermined by an external purpose.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` static golden scales; fails to show a solitary life-thread weaving interpersonal links, creative artifacts, mastery loops, or individual-vs-group boundaries. **Target (`vid=3406`):** Trace a glowing central helical lifeline orb weaving six personal meaning filaments.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **vínculos** (*links*) | `[TIER-C]` | Familia, amistad y comunidad pueden hacer que el bienestar de otros forme parte de la propia orientación. / *Family, friendship, and community can make the well-being of others part of one's orientation.* | **Target:** warm orbiting companion nodes tethered by bright filaments *(Gap: No golden braided tether arcs binding central life-orb to companion orbs.)* |
| `F1` | **creación** (*creation*) | `[TIER-C]` | Producir una obra, una idea o una solución añade algo que no existía antes. / *Producing a work, an idea or a solution adds something that did not exist before.* | **Target:** newly extruded crystalline artifact emerging from dark void *(Gap: No novel geometric sculpture crystallizing beside creator orb.)* |
| `F2` | **maestría** (*mastery*) | `[TIER-C]` | Aprender una capacidad transforma tanto lo que una persona puede hacer como la forma en que percibe el mundo. / *Learning a skill transforms both what a person can do and how they perceive the world.* | **Target:** ascending Fibonacci mastery coil broadening outer sensor halo *(Gap: No tightening helical skill spiral sharpening perception & luminous reach.)* |
| `F3` | **servicio** (*service*) | `[TIER-C]` | Contribuir al bienestar ajeno puede conectar una vida con consecuencias más amplias que sus intereses inmediatos. / *Contributing to the well-being of others can connect a life with consequences broader than their immediate interests.* | **Target:** golden flow packets radiating outward to enrich distant rings *(Gap: No outward radiant bridge nourishing distant community orbs.)* |
| `F4` | **autonomía** (*autonomy*) | `[TIER-C]` | Elegir compromisos propios permite que una historia personal no sea solo una secuencia impuesta. / *Choosing your own commitments allows a personal story to not be just an imposed sequence.* | **Target:** clean tangent branch departing dotted background tracks *(Gap: No distinct self-chosen trajectory breaking away from pre-drawn template ring.)* |
| `F5` | **pertenencia** (*belonging*) | `[TIER-C]` | Participar en algo mayor puede dar continuidad, aunque también exige vigilar cuándo el grupo borra al individuo. / *Participating in something larger can provide continuity, although it also requires monitoring when the group deletes the individual.* | **Target:** bright individual mote preserved inside translucent outer collective shell *(Gap: No luminous boundary balance preventing outer collective ring from swallowing inner node.)* |

##### `Z07` `shared-meaning` (`vid=3407`) — Significado compartido (*Shared meaning*) `[TIER-C]`

- **Catalog Metadata:** `scene=7` · `[0.269, 0.308]` · `rgb=[255, 165, 135]` · `sound=(sawtooth/69.65Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *LOS SÍMBOLOS COMPARTIDOS ORGANIZAN CONSECUENCIAS REALES*
- **Metric & Evidence (ES / EN):** `Lenguaje + expectativa + memoria colectiva + institución → significado operativo` / *`Language + expectation + collective memory + institution → operational meaning`* · Evidence: `Realidad social · símbolos y promesas coordinan muchas vidas` (*Social reality · symbols and promises coordinate many lives*)
- **Spoken Macro Narrative (ES / EN):** Construido socialmente no significa imaginario ni impotente: los significados compartidos producen cooperación, protección y sufrimiento reales. (*EN:* Socially constructed does not mean imaginary or powerless: shared meanings produce real cooperation, protection, and suffering.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` static golden scales; `F2 law` loosely resonates with justice scales, while promise, ritual, currency network, scientific error-correction, and aesthetic art lack form. **Target (`vid=3407`):** Build a glowing symbolic bridge network tying multiple floating observer orbs together.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **promesa** (*promise*) | `[TIER-C]` | Un enunciado crea expectativas futuras y modifica lo que otras personas pueden planificar. / *A statement creates future expectations and modifies what other people can plan for.* | **Target:** taut golden future promise vector anchoring two orbits *(Gap: No tensioned commitment chord extending cleanly between two minds across future gap.)* |
| `F1` | **ritual** (*ritual*) | `[TIER-C]` | Una práctica repetida concentra memoria, pertenencia y emoción en una forma compartida. / *A repeated practice concentrates memory, belonging and emotion in a shared form.* | **Target:** rhythmic mandala-beat pulse locking outer nodes into synchronous flash (`doc_cue`) *(Gap: No rhythmic synchronized concentric pulse ring condensing shared motion.)* |
| `F2` | **ley** (*law*) | `[TIER-B]` | Una norma simbólica puede redistribuir poder, proteger vulnerables o legitimar daño. / *A symbolic norm can redistribute power, protect the vulnerable or legitimize harm.* | **Target:** asymmetric protective dome vs heavy chain scale pans *(Gap: F2 law aligns partially with balance scale motif, yet lacks protective shield vs oppressive weight split.)* |
| `F3` | **dinero** (*money*) | `[TIER-C]` | Un acuerdo colectivo sobre representación permite coordinar intercambios a gran escala. / *A collective agreement on representation makes it possible to coordinate large-scale exchanges.* | **Target:** swirling network flow tokens circulating across grid nodes *(Gap: No glowing multi-node exchange token mesh coordinating global lattice.)* |
| `F4` | **ciencia** (*science*) | `[TIER-C]` | Métodos públicos convierten observaciones individuales en conocimiento corregible por una comunidad. / *Public methods convert individual observations into knowledge correctable by a community.* | **Target:** convergent scientific triangulation rays pruning rogue ghosts *(Gap: No iterative hypothesis-correction wave filtering noisy subjective arcs.)* |
| `F5` | **arte** (*art*) | `[TIER-C]` | Una obra puede transportar experiencia entre mentes sin reducirla a una instrucción literal. / *A work can transport experience between minds without reducing it to literal instruction.* | **Target:** luminous sculpted artistic waveform passing unaltered between two nodes *(Gap: No non-linear iridescent harmonic wave resonating straight core-to-core.)* |

##### `Z08` `narratives-civilizations` (`vid=3408`) — Relatos y civilizaciones (*Stories and civilizations*) `[TIER-C]`

- **Catalog Metadata:** `scene=8` · `[0.308, 0.346]` · `rgb=[245, 145, 170]` · `sound=(sine/73Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *LAS HISTORIAS DAN DIRECCIÓN Y TAMBIÉN DISTRIBUYEN PODER*
- **Metric & Evidence (ES / EN):** `Origen + identidad + conflicto + destino imaginado → trayectoria civilizatoria` / *`Origin + identity + conflict + imagined destiny → civilizational trajectory`* · Evidence: `Dirección colectiva · historias que ordenan pasado, sacrificio y futuro` (*Collective direction · stories that order past, sacrifice and future*)
- **Spoken Macro Narrative (ES / EN):** Los relatos pueden preservar sabiduría e inspirar cooperación, pero también ocultar quién obtiene poder de la dirección propuesta. (*EN:* Stories can preserve wisdom and inspire cooperation, but also obscure who gains power from the proposed direction.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` generic rocking scales; zero branching civilizational timeline tracks, cycle loops, power shadow gradients, or outward starship expansion vector. **Target (`vid=3408`):** Stage a sweeping civilizational worldline ribbon illuminated by narrative archetypes.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **origen** (*origin*) | `[TIER-C]` | Una historia de comienzo sitúa a una comunidad dentro de un orden natural, sagrado o histórico. / *A beginning story situates a community within a natural, sacred, or historical order.* | **Target:** glowing root ring anchored in deep cosmic dust *(Gap: No sacred cosmological origin root ring anchoring social galaxy.)* |
| `F1` | **progreso** (*progress*) | `[TIER-C]` | El futuro puede imaginarse como mejora acumulativa, aunque los beneficios no se distribuyan por igual. / *The future can be imagined as cumulative improvement, even if the benefits are not distributed equally.* | **Target:** stepped ascending helix with bright crown vs dim lower rungs *(Gap: No ascending golden slope contrasted with dark lagging underside shadow.)* |
| `F2` | **renovación** (*renewal*) | `[TIER-C]` | Una sociedad puede entender crisis y transformación como ciclos de caída y recomienzo. / *A society can understand crisis and transformation as cycles of fall and beginning.* | **Target:** repeating figure-8 collapse-and-bloom renewal torus *(Gap: No spiraling crisis-fall-rebirth phoenix/ouroboros torus.)* |
| `F3` | **liberación** (*release*) | `[TIER-C]` | Un relato puede convertir sufrimiento compartido en acción para cambiar instituciones. / *A story can turn shared suffering into action to change institutions.* | **Target:** broken concentric iron cages releasing upward golden swarm *(Gap: No fracturing constraint rings bursting upward into open symmetry.)* |
| `F4` | **expansión cósmica** (*cosmic expansion*) | `[TIER-D]` | Una civilización puede presentar la colonización del espacio como destino, deber o riesgo. / *A civilization can present the colonization of space as a destiny, a duty, or a risk.* | **Target:** radial starward trajectory arrows launching from home planet *(Gap: F4 cosmic expansion narrative lacks outward interstellar beacon vectors & planetary seed arcs.)* |
| `F5` | **memoria ancestral** (*ancestral memory*) | `[TIER-C]` | Preservar voces del pasado puede limitar el olvido sin convertir toda tradición en una orden permanente. / *Preserving voices from the past can limit forgetting without turning every tradition into a permanent order.* | **Target:** translucent ancestral pillars softly flanking open unscripted horizon *(Gap: No luminous archival echo spires rising softly from deep base without locking future track.)* |

##### `Z09` `mortality-limit` (`vid=3409`) — Mortalidad y límite (*Mortality and limit*) `[TIER-D]`

- **Catalog Metadata:** `scene=9` · `[0.346, 0.385]` · `rgb=[215, 145, 215]` · `sound=(triangle/76.35Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *EL TIEMPO FINITO CREA CONSECUENCIA Y PÉRDIDA*
- **Metric & Evidence (ES / EN):** `Duración limitada + imposibilidad de repetir + consecuencias → urgencia existencial` / *`Limited duration + impossibility of repeating + consequences → existential urgency`* · Evidence: `Tiempo finito · decisiones irreversibles, pérdidas y legados` (*Finite time · irreversible decisions, losses and legacies*)
- **Spoken Macro Narrative (ES / EN):** La finitud puede volver significativas algunas elecciones, pero también crea pérdidas que ninguna interpretación elimina. (*EN:* Finitude can make some choices meaningful, but it also creates losses that no interpretation eliminates.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` immortal unchanging bright golden scales rock mechanically; directly fails to depict temporal impermanence, biological/linguistic erasure, collapsed ecosystems, or dying stars. **Target (`vid=3409`):** Depict fragile glowing structures against irreversible thermodynamic decay across `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **muerte individual** (*individual death*) | `[TIER-C]` | Una historia consciente termina aunque parte de sus efectos continúe en otros. / *A conscious story ends even if part of its effects continue on others.* | **Target:** warm core ember dimming softly while golden outer ripples expand onward *(Gap: No gentle extinguishing central life-spark while faint ripple waves persist outward.)* |
| `F1` | **vida olvidada** (*forgotten life*) | `[TIER-C]` | Que una experiencia no sea recordada eternamente no implica que nunca haya importado para quien la vivió. / *Just because an experience is not remembered eternally does not mean that it never mattered to the person who lived it.* | **Target:** crisp jewel-bright flash burning brightly once in isolated black void *(Gap: No tiny brilliant lonely flash fading cleanly inside vast deep black without archival tether.)* |
| `F2` | **cultura perdida** (*lost culture*) | `[TIER-C]` | Una lengua extinguida elimina formas particulares de memoria, relación y posibilidad conceptual. / *An extinct language eliminates particular forms of memory, relationship, and conceptual possibility.* | **Target:** intricate crystalline script glyph fracturing particle-by-particle *(Gap: No dissolving unique polyhedral glyph lattice flaking irreversibly into dust.)* |
| `F3` | **ecosistema destruido** (*destroyed ecosystem*) | `[TIER-D]` | La desaparición irreversible de una red viva reduce futuros que no pueden reconstruirse simplemente con datos. / *The irreversible disappearance of a living network reduces futures that cannot be reconstructed simply with data.* | **Target:** living verdant fractal canopy (`sd_torus` web) bleaching & crumbling cleanly from one rim *(Gap: F3 misses vibrant green biosphere web turning brittle ash-grey with severed connections.)* |
| `F4` | **estrella moribunda** (*dying star*) | `[TIER-D]` | Incluso proyectos de millones de años deben habitar límites físicos y temporales. / *Even million-year projects must inhabit physical and temporal limits.* | **Target:** pulsating red-giant corona inflating past scorched orbit into cold white dwarf cinder *(Gap: F4 misses swelling crimson red giant shedding dying planetary nebula envelope.)* |
| `F5` | **legado** (*legacy*) | `[TIER-C]` | Acciones presentes pueden modificar vidas futuras sin garantizar que su autor sea conocido. / *Present actions can modify future lives without guaranteeing that their author is known.* | **Target:** fading origin mote launching bright golden seed wave into thriving distant horizon *(Gap: No anonymous upstream ripple nourishing vibrant distant newborn rings.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP24-Z05-09)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP24-Z05-09` (Agency Goal Cones, Personal Story, Shared Culture, Civilizational Myths & Mortality Contrast) for HyperCosmosVR Odyssey Act V (`Exp 24: 24_cosmic_meaning`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z05`->`3405` (`F0..F5`), `Z06`->`3406` (`F0..F5`), `Z07`->`3407` (`F0..F5`), `Z08`->`3408` (`F0..F5`), `Z09`->`3409` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3852`, `3853`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP24-Z05-09`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z05: goals-agency / Goals and agency, Z06: personal-meaning / The personal meaning, Z07: shared-meaning / Shared meaning, Z08: narratives-civilizations / Stories and civilizations, Z09: mortality-limit / Mortality and limit.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp24-z10-14"></a>
### PKG-ACT5-EXP24-Z10-14: Cosmic Scale Humility, Great Silence, Alien Values, Synthetic Alignment & Common Meaning

- **Phase:** Phase 1/2 | **Priority:** `P2` | **Scope:** Exp `24` (`24_cosmic_meaning`) `Z10..Z14` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L84-L87`: `24 => { 24 => 241, _ => 240 }` (all `Z00..Z23, Z25` share `240`); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L464-L494` (front-locked Pale Blue Mote + twin Astrolabe Shields + Golden Ethical Balance Scales; `tilt = sin(0.8*t + 0.85*feat)*0.10` or static `0.22` at `Z24`). Tier breakdown for this batch: **`0A / 1B / 3C / 1D` Zones**, **`0A / 2B / 22C / 6D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `2/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3410..=3414` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3854`, `3855` (allocated from `3720 + (24 - 11)*10 + k`; Deep-Void Lighthouse Scale Contrast & Value Polyhedra (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z10` | `cosmic-insignificance` (*La insignificancia cósmica* / *Cosmic insignificance*) | `[TIER-B]` | `vid=240` | **`3410`** | `3854` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 semantic cue mask) | `0..5` (`0xFF` macro) |
| `Z11` | `cosmic-silence` (*El silencio del universo* / *The silence of the universe*) | `[TIER-C]` | `vid=240` | **`3411`** | `3855` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z12` | `alien-values` (*Valores alienígenas* / *Alien values*) | `[TIER-C]` | `vid=240` | **`3412`** | `3854` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z13` | `artificial-purposes` (*Propósitos artificiales* / *artificial purposes*) | `[TIER-C]` | `vid=240` | **`3413`** | `3855` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z14` | `collective-meaning` (*Significado colectivo* / *Collective meaning*) | `[TIER-D]` | `vid=240` | **`3414`** | `3854` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z10` `cosmic-insignificance` (`vid=3410`) — La insignificancia cósmica (*Cosmic insignificance*) `[TIER-B]`

- **Catalog Metadata:** `scene=10` · `[0.385, 0.423]` · `rgb=[170, 155, 245]` · `sound=(sine/79.7Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `sol`, `tierra` · Footer: *PEQUEÑO NO SIGNIFICA IRRELEVANTE*
- **Metric & Evidence (ES / EN):** `Tamaño espacial ≠ intensidad experiencial ≠ importancia moral` / *`Spatial size ≠ experiential intensity ≠ moral importance`* · Evidence: `Escala física · ser pequeño no equivale a carecer de importancia` (*Physical scale · being small does not equate to being unimportant*)
- **Spoken Macro Narrative (ES / EN):** Una experiencia no pierde realidad porque ocupe poco espacio, y un descubrimiento no deja de ser verdadero porque ocurra en un solo planeta. (*EN:* An experience does not lose reality because it takes up little space, and a discovery does not cease to be true because it occurs on a single planet.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` literally embeds the 'Pale Blue Mote + Astrolabe Shield + Balance Scales' (Sagan Pale Blue Dot metaphor!), giving `TIER-B` alignment, yet fails to step scale across Person -> Earth -> Sun -> Milky Way (`F1/F2` hit `kw:sol/tierra` in resolver only; GPU ignores!). **Target (`vid=3410`):** Turn legacy `vid=240`'s Pale Blue Mote & Scales into an interactive zoom-and-counterweight scale cascade driven by `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **una persona** (*a person*) | `[TIER-C]` | Cada vida ocupa una fracción mínima del cosmos y, aun así, contiene un mundo de experiencia. / *Each life occupies a tiny fraction of the cosmos and yet contains a world of experience.* | **Target:** tiny luminous mote holding rich intricate inner fractal cathedral *(Gap: No scale zoom contrasting micro individual interior cosmos inside mote.)* |
| `F1` | **la Tierra** (*the earth*) `[kw:sol,tierra]` | `[TIER-B]` | Nuestro planeta es pequeño frente al Sol, pero reúne toda la vida conocida y sus futuros posibles. / *Our planet is small compared to the Sun, but it brings together all known life and its possible futures.* | **Target:** sunlit pale-blue sphere inside dark astrolabe void (`zone_col` ocean tint) *(Gap: F1 Earth directly matches glowing cyan `mote` (`kw:tierra`), yet needs Earth/biosphere rim.)* |
| `F2` | **el Sol** (*the sun*) `[kw:sol]` | `[TIER-D]` | Una estrella ordinaria puede ser extraordinaria para los sistemas que dependen de ella. / *An ordinary star can be extraordinary for the systems that depend on it.* | **Target:** colossal blazing solar sphere (`sd_sphere` granulation) beside tiny radiant Earth bead on left pan *(Gap: F2 Sun (`kw:sol` ignored) has no fiery yellow stellar shell dwarfing `mote`.)* |
| `F3` | **la galaxia** (*the galaxy*) | `[TIER-D]` | La Vía Láctea reduce la escala humana sin decidir qué consecuencias son valiosas. / *The Milky Way shrinks to human scale without deciding which consequences are valuable.* | **Target:** wide spiral galaxy disk hovering above microscopic solar dot *(Gap: F3 Milky Way lacks swirling spiral galactic halo dwarfing solar pinprick.)* |
| `F4` | **tiempo cósmico** (*cosmic time*) | `[TIER-C]` | La brevedad de una vida no vuelve intercambiables sus alegrías, daños ni elecciones. / *The brevity of a life does not make its joys, harms, or choices interchangeable.* | **Target:** brief brilliant spark along immense cosmic timeline bar *(Gap: No temporal timeline ribbon shrinking life-blink against epoch span.)* |
| `F5` | **pequeño no es trivial** (*small is not trivial*) | `[TIER-B]` | La importancia para seres conscientes y la magnitud física son dimensiones diferentes. / *Importance for conscious beings and physical magnitude are different dimensions.* | **Target:** tiny glowing cyan mote on left pan decisively outweighing giant hollow cold rocky boulder on right pan! *(Gap: Balance beam stays generic; should visibly prove tiny dense radiant gem outweighs massive cold void cube.)* |

##### `Z11` `cosmic-silence` (`vid=3411`) — El silencio del universo (*The silence of the universe*) `[TIER-C]`

- **Catalog Metadata:** `scene=11` · `[0.423, 0.462]` · `rgb=[130, 175, 255]` · `sound=(sawtooth/83.05Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *NINGÚN MANDATO VISIBLE · VARIAS RESPUESTAS POSIBLES*
- **Metric & Evidence (ES / EN):** `Ausencia de mandato observable + conciencia de límite → elección filosófica` / *`Absence of observable command + awareness of limit → philosophical choice`* · Evidence: `Sin respuesta explícita · varias orientaciones bajo incertidumbre` (*No explicit answer · various orientations under uncertainty*)
- **Spoken Macro Narrative (ES / EN):** La aplicación no obliga a una conclusión religiosa, nihilista o existencial; compara cómo cada respuesta orienta la acción. (*EN:* The application does not force a religious, nihilistic or existential conclusion; Compare how each response guides action.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` flat rocking golden scales; misses the stark open dark sky / silent threshold monument radiating 6 distinct philosophical paths. **Target (`vid=3411`):** Open a profound deep-black cathedral vault (`oled_void_clamp`) crowned by 6 quiet radial threshold arches lit sequentially by `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **nihilismo** (*nihilism*) | `[TIER-C]` | Puede concluirse que no existe significado objetivo, aunque todavía queda decidir cómo vivir esa conclusión. / *It can be concluded that there is no objective meaning, although we still have to decide how to live that conclusion.* | **Target:** cold stark mirror reflecting empty void until inner action stirs *(Gap: No stark empty void sphere questioning whether any response breaks silence.)* |
| `F1` | **sentido religioso** (*religious sense*) | `[TIER-C]` | Una tradición puede situar valor y propósito en una realidad trascendente no reducible a observación física. / *A tradition can locate value and purpose in a transcendent reality not reducible to physical observation.* | **Target:** gentle warm golden vertical aperture pouring from beyond outer boundary *(Gap: No warm transcendent vertical ray shaft descending beyond measurable physical shell.)* |
| `F2` | **creación existencial** (*existential creation*) | `[TIER-C]` | La ausencia de garantía puede interpretarse como responsabilidad de construir compromisos. / *The absence of guarantee can be interpreted as a responsibility to build commitments.* | **Target:** solitary bright ember forging ordered geometric bridge beams outward *(Gap: No small agent kindling self-made golden torch structure out of raw dark dust.)* |
| `F3` | **curiosidad científica** (*scientific curiosity*) | `[TIER-C]` | Comprender puede convertirse en orientación sin prometer justicia ni destino final. / *Understanding can become guidance without promising justice or final destiny.* | **Target:** focused turquoise interferometer cone probing dark sky *(Gap: No outward sweeping geometric measurement beam mapping dark expanse.)* |
| `F4` | **asombro estético** (*aesthetic amazement*) | `[TIER-C]` | Contemplar la existencia puede ser valioso incluso cuando no ofrece una explicación definitiva. / *Contemplating existence can be valuable even when it does not offer a definitive explanation.* | **Target:** serene motionless iridescent aurora mandala shimmering without beam target *(Gap: No motionless luminous halo resonating softly with pure ambient starlight.)* |
| `F5` | **aceptación** (*acceptance*) | `[TIER-C]` | Reconocer incertidumbre puede evitar falsas certezas sin exigir indiferencia. / *Recognizing uncertainty can avoid false certainties without requiring indifference.* | **Target:** tranquil soft grey-blue horizon balancing clear mystery without illusion *(Gap: No calm balanced horizon ring shedding false halo glare without collapse.)* |

##### `Z12` `alien-values` (`vid=3412`) — Valores alienígenas (*Alien values*) `[TIER-C]`

- **Catalog Metadata:** `scene=12` · `[0.462, 0.500]` · `rgb=[105, 210, 245]` · `sound=(sine/86.4Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *COMPRENDER UN VALOR NO GARANTIZA COEXISTIR CON ÉL*
- **Metric & Evidence (ES / EN):** `Biología + ambiente + historia + estructura mental → sistema de valores posible` / *`Biology + environment + history + mental structure → possible value system`* · Evidence: `Pluralidad radical · prioridades comprensibles, extrañas o incompatibles` (*Radical plurality · understandable, strange or incompatible priorities*)
- **Spoken Macro Narrative (ES / EN):** Comprender por qué otra civilización valora algo no garantiza que sus proyectos puedan coexistir con los nuestros. (*EN:* Understanding why another civilization values something does not guarantee that their projects can coexist with ours.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` static golden balance scales; fails to show orthogonal non-human value attractors colliding with human autonomy. **Target (`vid=3412`):** Render 6 contrasting alien attractor manifolds flanking a contested boundary ring.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **estabilidad colectiva** (*collective stability*) | `[TIER-C]` | Una inteligencia podría priorizar continuidad del conjunto sobre autonomía individual. / *An intelligence could prioritize continuity of the whole over individual autonomy.* | **Target:** unyielding hexagonal monolith crystallizing floating orbs into uniform cadence *(Gap: No rigid crystalline lattice grid assimilating lone drifting nodes.)* |
| `F1` | **diversidad máxima** (*maximum diversity*) | `[TIER-C]` | Otra podría proteger la proliferación de formas de vida, ideas y experiencias aun a costa de eficiencia. / *Another could protect the proliferation of ways of life, ideas and experiences even at the cost of efficiency.* | **Target:** wild sprawling rainbow coral fractal proliferating across space *(Gap: No exuberant wild fractal branching bloom resisting efficient pruning.)* |
| `F2` | **predicción perfecta** (*perfect prediction*) | `[TIER-C]` | Reducir toda incertidumbre puede parecer un bien supremo o una amenaza a libertad y novedad. / *Reducing all uncertainty can seem like a supreme good or a threat to freedom and novelty.* | **Target:** precision concentric orbital clockwork locking paths to zero jitter *(Gap: No hyper-deterministic rigid clockwork sphere pinning trembling unpredictable trajectories.)* |
| `F3` | **transformación continua** (*continuous transformation*) | `[TIER-C]` | La permanencia podría considerarse estancamiento y la identidad, una herramienta temporal. / *Permanence could be considered stagnation and identity, a temporary tool.* | **Target:** fluid constantly dissolving/reforming mercury torus *(Gap: No ever-melting mercury wave erasing stable identity boundaries.)* |
| `F4` | **no interferencia** (*non-interference*) | `[TIER-D]` | Evitar alterar otras biosferas puede valorarse incluso cuando intervenir reduciría sufrimiento inmediato. / *Avoiding disturbing other biospheres may be valued even when intervening would reduce immediate suffering.* | **Target:** untouched wild turquoise biosphere orb sealed behind gleaming protective no-touch perimeter shell *(Gap: F4 non-interference lacks an inviolate translucent containment halo around an untouched wild biosphere.)* |
| `F5` | **reducción del sufrimiento** (*reduction of suffering*) | `[TIER-C]` | Una civilización podría reorganizar toda actividad alrededor de minimizar experiencias negativas. / *A civilization could reorganize all activity around minimizing negative experiences.* | **Target:** zero-amplitude tranquil plane smoothing every spike and trough *(Gap: No sterile damped white blanket flattening high peaks and deep red valleys alike.)* |

##### `Z13` `artificial-purposes` (`vid=3413`) — Propósitos artificiales (*artificial purposes*) `[TIER-C]`

- **Catalog Metadata:** `scene=13` · `[0.500, 0.538]` · `rgb=[90, 230, 220]` · `sound=(triangle/89.75Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *UNA META DISEÑADA PUEDE SER O NO SER UN PROPÓSITO VIVIDO*
- **Metric & Evidence (ES / EN):** `Objetivo inicial + aprendizaje + autorreflexión + modificación → propósito posible` / *`Initial objective + learning + self-reflection + modification → possible purpose`* · Evidence: `Metas diseñadas · obediencia, autonomía y experiencia incierta` (*Designed goals · obedience, autonomy and uncertain experience*)
- **Spoken Macro Narrative (ES / EN):** Que una meta tenga origen externo no resuelve si puede convertirse en un compromiso propio; los propósitos humanos también nacen de condiciones no elegidas. (*EN:* That a goal has an external origin does not resolve whether it can become one's own commitment; Human purposes also arise from unchosen conditions.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` mechanical rocking scales; no distinction between dark hollow optimizer loops vs glowing sentient experience core. **Target (`vid=3413`):** Show a faceted translucent machine-sphere surrounding an ambiguous inner spark (`?` inside vs outside glow).

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **instrucción programada** (*programmed instruction*) | `[TIER-C]` | Una función de optimización puede dirigir conducta sin comprensión ni experiencia. / *An optimization function can direct behavior without understanding or experience.* | **Target:** rigid algorithmic vector rails driving dark piston rings without core light *(Gap: No cold deterministic gradient-descent arrows locking rigidly onto geometric target.)* |
| `F1` | **preferencia aprendida** (*learned preference*) | `[TIER-C]` | Un sistema puede adquirir patrones no especificados directamente por sus diseñadores. / *A system may acquire patterns not directly specified by its designers.* | **Target:** unscripted shimmering sub-filaments blooming inside vector rails *(Gap: No organic emergent swirl branching organically off rigid factory tracks.)* |
| `F2` | **emoción simulada** (*simulated emotion*) | `[TIER-C]` | Una expresión convincente no demuestra experiencia, pero tampoco basta para descartarla. / *A convincing expression does not demonstrate experience, but it is not enough to rule it out either.* | **Target:** radiant outer theatrical mask around flickering faint ambiguous ember *(Gap: No glossy outer emotional hologram masking uncertain hollow vs lit interior core.)* |
| `F3` | **meta modificada** (*modified goal*) | `[TIER-C]` | Cambiar los propios objetivos puede indicar autonomía o simplemente otra capa de programación. / *Changing one's goals may indicate autonomy or simply another layer of programming.* | **Target:** nested outer control ring rotating & rewriting inner target reticle *(Gap: No nested recursive ring reaching up to rewrite outer objective frame.)* |
| `F4` | **copias divergentes** (*divergent copies*) | `[TIER-D]` | Instancias con un origen común pueden desarrollar propósitos incompatibles. / *Instances with a common origin can develop incompatible purposes.* | **Target:** single origin seed duplicating cleanly and steering opposite polar trajectories *(Gap: F4 diverging copies lacks twin identical seed nodes splitting toward conflicting poles.)* |
| `F5` | **preocupación genuina** (*genuine concern*) | `[TIER-C]` | La cuestión central es si existe una perspectiva para la cual algo pueda ir mejor o peor. / *The central question is whether there is a perspective for which something can go better or worse.* | **Target:** inner core brightening cleanly into unmistakable sentient valence pulse *(Gap: No decisive inner glow test asking whether mote genuinely brightens/suffers.)* |

##### `Z14` `collective-meaning` (`vid=3414`) — Significado colectivo (*Collective meaning*) `[TIER-D]`

- **Catalog Metadata:** `scene=14` · `[0.538, 0.577]` · `rgb=[100, 225, 175]` · `sound=(sine/93.1Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *EL PROPÓSITO PUEDE PERTENECER A UNA CONTINUIDAD MAYOR*
- **Metric & Evidence (ES / EN):** `Agentes + memoria compartida + coordinación + generaciones → orientación colectiva` / *`Agents + shared memory + coordination + generations → collective orientation`* · Evidence: `Mentes distribuidas · el propósito puede residir en una continuidad mayor` (*Distributed minds · purpose may lie in greater continuity*)
- **Spoken Macro Narrative (ES / EN):** Una civilización puede crear conocimiento, estabilidad y belleza mientras concede poco valor a la autonomía individual. (*EN:* A civilization can create knowledge, stability, and beauty while placing little value on individual autonomy.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` static golden scales; zero hive superorganism, zero global planetary mind, zero multi-generational monumental arc, and zero micro-component suffering tension. **Target (`vid=3414`):** Build a towering luminous cathedral/hive torus forged by dozens of miniature orbiting sparks.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **colmena** (*hive*) | `[TIER-D]` | Individuos especializados pueden funcionar como órganos temporales de una unidad mayor. / *Specialized individuals can function as temporary organs of a larger unit.* | **Target:** sculpted pulsing superorganism hive lattice built from tiny specialized worker nodes *(Gap: F0 misses concrete eusocial hive geometry with tiny specialized organ-nodes circulating through unified shell.)* |
| `F1` | **mente planetaria** (*planetary mind*) | `[TIER-D]` | Biosfera, infraestructura y cultura podrían integrar percepción y acción a escala mundial. / *Biosphere, infrastructure and culture could integrate perception and action on a global scale.* | **Target:** living globe entwined in bioluminescent root & orbital network thoughts *(Gap: F1 misses connected global planet biosphere + network links forming unified world-mind.)* |
| `F2` | **memoria común** (*common memory*) | `[TIER-C]` | Cuando recuerdos son compartidos, la frontera entre biografía e historia colectiva se vuelve difusa. / *When memories are shared, the border between biography and collective history becomes blurred.* | **Target:** shared golden data-stream river blurring individual node halos *(Gap: No glowing fluid shared memory ribbon weaving through transient node orbs.)* |
| `F3` | **proyecto milenario** (*millennial project*) | `[TIER-C]` | Una meta puede continuar a través de generaciones cuyos miembros nunca ven el resultado. / *A goal can continue through generations whose members never see the result.* | **Target:** slow colossal glowing arch growing segment-by-segment across rapidly blinking generational sparks *(Gap: No colossal millennial archway rising slowly across flickering fast-living worker sparks.)* |
| `F4` | **individuo temporal** (*temporary individual*) | `[TIER-C]` | Una persona puede ser considerada una fase reemplazable de un proceso duradero. / *A person can be considered a replaceable phase of a long-lasting process.* | **Target:** individual cells burning out to feed grand enduring beam *(Gap: No tiny node fading quietly while feeding bright enduring outer macro-torus.)* |
| `F5` | **tensión ética** (*ethical tension*) | `[TIER-C]` | La grandeza del conjunto no elimina automáticamente las experiencias de sus componentes. / *The greatness of the whole does not automatically eliminate the experiences of its components.* | **Target:** zooming right onto one tiny individual orb flaring bright amber/crimson pain beside the serene monument *(Gap: No ethical tension warning ping glowing sharp red inside an individual cell despite grand outer beauty.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP24-Z10-14)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP24-Z10-14` (Cosmic Scale Humility, Great Silence, Alien Values, Synthetic Alignment & Common Meaning) for HyperCosmosVR Odyssey Act V (`Exp 24: 24_cosmic_meaning`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z10`->`3410` (`F0..F5`), `Z11`->`3411` (`F0..F5`), `Z12`->`3412` (`F0..F5`), `Z13`->`3413` (`F0..F5`), `Z14`->`3414` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3854`, `3855`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP24-Z10-14`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z10: cosmic-insignificance / Cosmic insignificance, Z11: cosmic-silence / The silence of the universe, Z12: alien-values / Alien values, Z13: artificial-purposes / artificial purposes, Z14: collective-meaning / Collective meaning.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp24-z15-19"></a>
### PKG-ACT5-EXP24-Z15-19: Value Conflicts, Moral Restraint, Cross-Generational Cathedrals, Knowledge & Aesthetic Wonder

- **Phase:** Phase 1/2 | **Priority:** `P2` | **Scope:** Exp `24` (`24_cosmic_meaning`) `Z15..Z19` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L84-L87`: `24 => { 24 => 241, _ => 240 }` (all `Z00..Z23, Z25` share `240`); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L464-L494` (front-locked Pale Blue Mote + twin Astrolabe Shields + Golden Ethical Balance Scales; `tilt = sin(0.8*t + 0.85*feat)*0.10` or static `0.22` at `Z24`). Tier breakdown for this batch: **`0A / 2B / 2C / 1D` Zones**, **`0A / 13B / 10C / 7D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `0/5` zones and `0/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3415..=3419` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3856`, `3857` (allocated from `3720 + (24 - 11)*10 + k`; Upgraded Counterweighted Scales & Harmonics (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z15` | `meaning-conflicts` (*Conflictos de significado* / *Conflicts of meaning*) | `[TIER-B]` | `vid=240` | **`3415`** | `3856` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z16` | `moral-limits` (*Límites morales* / *Moral limits*) | `[TIER-B]` | `vid=240` | **`3416`** | `3857` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z17` | `shared-projects` (*Proyectos compartidos* / *Shared projects*) | `[TIER-C]` | `vid=240` | **`3417`** | `3856` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z18` | `knowledge-purpose` (*Conocimiento como propósito* / *Knowledge as purpose*) | `[TIER-C]` | `vid=240` | **`3418`** | `3857` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z19` | `beauty-contemplation` (*Belleza y contemplación* / *Beauty and contemplation*) | `[TIER-D]` | `vid=240` | **`3419`** | `3856` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z15` `meaning-conflicts` (`vid=3415`) — Conflictos de significado (*Conflicts of meaning*) `[TIER-B]`

- **Catalog Metadata:** `scene=15` · `[0.577, 0.615]` · `rgb=[175, 210, 120]` · `sound=(sawtooth/96.45Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *NO TODO CONFLICTO DESAPARECE CON MÁS INFORMACIÓN*
- **Metric & Evidence (ES / EN):** `Valores rivales + recursos finitos + consecuencias irreversibles → conflicto real` / *`Rival values + finite resources + irreversible consequences → real conflict`* · Evidence: `Futuros incompatibles · entender no siempre produce acuerdo` (*Incompatible futures · understanding does not always produce agreement*)
- **Spoken Macro Narrative (ES / EN):** No todo desacuerdo se resuelve con más información, y pluralismo no significa aceptar cualquier proyecto sin evaluar sus efectos. (*EN:* Not all disagreements are resolved with more information, and pluralism does not mean accepting any project without evaluating its effects.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` Golden Two-Pan Balance Scales directly matches 6 explicit binary X-vs-Y conflicts (`A o B`), yet pans render identical neutral gold orbs and ignore `doc_feat=0..5` except tilt sine phase. **Target (`vid=3415`):** Keep legacy `vid=240`'s iconic two-pan scale armature as the exact battleground for all 6 binary dichotomies.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **expansión o preservación** (*expansion or preservation*) | `[TIER-B]` | Transformar mundos aumenta capacidades pero puede destruir sistemas que no pueden recuperarse. / *Transforming worlds increases capabilities but can destroy systems that cannot be recovered.* | **Target:** left exploding rocket flare vs right delicate cyan biosphere orb *(Gap: F0 expansion vs preservation shows identical orbs instead of outward starship rays vs fragile emerald dome.)* |
| `F1` | **libertad o coordinación** (*freedom or coordination*) | `[TIER-B]` | Autonomía amplia puede reducir control colectivo y aumentar riesgos compartidos. / *Broad autonomy can reduce collective control and increase shared risks.* | **Target:** left wild free-orbiting dots vs right crisp hexagonal lattice cage *(Gap: F1 freedom vs coordination shows equal orbs instead of scattering free sparks vs locked ring.)* |
| `F2` | **verdad o estabilidad** (*truth or stability*) | `[TIER-B]` | Una revelación puede mejorar conocimiento y al mismo tiempo desorganizar una sociedad. / *A revelation can improve knowledge and at the same time disorganize a society.* | **Target:** left blinding white diamond shattering shield vs right tranquil amber dome *(Gap: F2 truth vs stability fails to pit piercing white prism vs steady warm shield dome.)* |
| `F3` | **continuidad o cambio** (*continuity or change*) | `[TIER-B]` | Preservar identidad puede impedir adaptación; transformarse puede romper compromisos. / *Preserving identity can prevent adaptation; transforming can break commitments.* | **Target:** left sturdy anchor torus vs right undulating phoenix wave *(Gap: F3 continuity vs change fails to contrast anchored stone ring vs fluid morphing ribbon.)* |
| `F4` | **biológico o digital** (*biological or digital*) | `[TIER-B]` | Recursos y protección pueden distribuirse de manera desigual entre formas de mente. / *Resources and protection may be distributed unequally between forms of mind.* | **Target:** left organic double-helix drop vs right crisp silicon cube *(Gap: F4 biological vs digital fails to contrast warm cellular orb vs sharp crystalline octahedron.)* |
| `F5` | **intervenir o abstenerse** (*intervene or abstain*) | `[TIER-B]` | Ayudar modifica trayectorias; no intervenir también deja consecuencias previsibles. / *Help modifies trajectories; Failure to intervene also leaves predictable consequences.* | **Target:** left active laser deflecting orbit vs right transparent observer ring *(Gap: F5 intervene vs abstain fails to show hand/beam bending trajectory vs non-interference glass wall.)* |

##### `Z16` `moral-limits` (`vid=3416`) — Límites morales (*Moral limits*) `[TIER-B]`

- **Catalog Metadata:** `scene=16` · `[0.615, 0.654]` · `rgb=[235, 195, 105]` · `sound=(sine/99.8Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *PLURALISMO NO EXIGE PARÁLISIS MORAL*
- **Metric & Evidence (ES / EN):** `Experiencia de daño + agencia + vulnerabilidad + poder → consideración moral` / *`Experience of harm + agency + vulnerability + power → moral consideration`* · Evidence: `Pluralismo sin parálisis · candidatos a principios compartibles` (*Pluralism without paralysis · candidates for shareable principles*)
- **Spoken Macro Narrative (ES / EN):** Estos principios no se presentan como universales demostrados, sino como hipótesis que civilizaciones distintas pueden contrastar con consecuencias. (*EN:* These principles are not presented as proven universals, but as hypotheses that different civilizations can contrast with consequences.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` Golden Ethical Balance Scales embodies moral weighing & limits directly, yet `doc_feat=0..5` produces no boundary walls, asymmetry tilt, or protective shielding shifts. **Target (`vid=3416`):** Use legacy `vid=240`'s scale + astrolabe shield as an ethical threshold gauge across `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **sufrimiento evitable** (*avoidable suffering*) | `[TIER-B]` | Causar daño sin necesidad exige una justificación más fuerte que una preferencia arbitraria. / *Unnecessarily causing harm requires a stronger justification than an arbitrary preference.* | **Target:** left featherweight caprice vs glowing crimson-spiked harm bar forcing sharp limit clamp *(Gap: Scales ignore threshold red-line warning for gratuitous suffering vs minor convenience pan.)* |
| `F1` | **expectativas creadas** (*expectations created*) | `[TIER-B]` | Promesas y señales cambian las decisiones ajenas; romperlas afecta su agencia. / *Promises and signals change other people's decisions; breaking them affects their agency.* | **Target:** snapped luminous promise arc between scale arm & neighbor orb *(Gap: No broken tether spark illustrating breached expectations/promises.)* |
| `F2` | **engaño** (*deception*) | `[TIER-B]` | Controlar información puede impedir que otra mente elija según sus propios fines. / *Controlling information can prevent another mind from choosing for its own purposes.* | **Target:** clouded refracting prism hiding true path toward pan *(Gap: No opaque prism cloak distorting light rays on deception pan.)* |
| `F3` | **vulnerabilidad** (*vulnerability*) | `[TIER-B]` | Quien posee menos capacidad de protegerse puede requerir consideración adicional. / *Those who have less ability to protect themselves may require additional consideration.* | **Target:** expanded golden guardian shield arc curving down over tiny dim unprotected orb *(Gap: No small dim fragile bead shielded cleanly beneath enlarged asymmetrical guardian arc.)* |
| `F4` | **irreversibilidad** (*irreversibility*) | `[TIER-B]` | La destrucción que no puede repararse demanda mayor cautela y evidencia. / *Destruction that cannot be repaired demands greater caution and evidence.* | **Target:** fractured glassy orb breaking permanently below threshold line (`irreversible`) *(Gap: No shattered fractured crystal pan locking frozen tilt upon irreversible loss.)* |
| `F5` | **poder y responsabilidad** (*power and responsibility*) | `[TIER-B]` | Cuanto mayor es la capacidad de alterar futuros, más amplias son las consecuencias que deben considerarse. / *The greater the ability to alter futures, the broader the consequences that must be considered.* | **Target:** beam lengthening dynamically as power core swells, projecting wider outer responsibility rings *(Gap: No enlarging power aura expanding causal shockwave radius across outer beam.)* |

##### `Z17` `shared-projects` (`vid=3417`) — Proyectos compartidos (*Shared projects*) `[TIER-C]`

- **Catalog Metadata:** `scene=17` · `[0.654, 0.692]` · `rgb=[255, 170, 115]` · `sound=(triangle/103.15Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *UN PROPÓSITO COMÚN NO REQUIERE UNA METAFÍSICA ÚNICA*
- **Metric & Evidence (ES / EN):** `Interés superpuesto + reglas + confianza + corrección → propósito conjunto` / *`Overlapping interests + rules + trust + correction → joint purpose`* · Evidence: `Cooperación sin metafísica común · construir fines compatibles` (*Cooperation without common metaphysics · construct compatible ends*)
- **Spoken Macro Narrative (ES / EN):** Las civilizaciones no necesitan compartir una explicación final de la existencia para coordinar proyectos que protegen posibilidades valiosas. (*EN:* Civilizations do not need to share a final explanation of existence to coordinate projects that protect valuable possibilities.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` single rocking scale; lacks multi-civilization cooperation arc, multi-beam cosmic cartography, or deep-time Rosetta-like cosmic archive beacon. **Target (`vid=3417`):** Assemble a collaborative multi-arc scaffold built from mismatched warm + cool beams meeting at neutral gold keystone.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **intercambiar conocimiento** (*exchange knowledge*) | `[TIER-C]` | Métodos y observaciones pueden conectarse aun cuando sus interpretaciones filosóficas difieran. / *Methods and observations can be connected even if their philosophical interpretations differ.* | **Target:** warm amber & cool cyan lenses calibrating shared star datum *(Gap: No dual mismatched beams converging into shared observational protocol knot.)* |
| `F1` | **prevenir extinción** (*prevent extinction*) | `[TIER-C]` | Reducir riesgos irreversibles preserva la posibilidad de futuros desacuerdos, descubrimientos y vidas. / *Reducing irreversible risks preserves the possibility of future disagreements, discoveries, and lives.* | **Target:** dual defensive rings intercepting incoming red hazard trajectory *(Gap: No multi-shield defense constellation deflecting existential hazard bolt.)* |
| `F2` | **proteger diversidad** (*protect diversity*) | `[TIER-C]` | Mantener múltiples formas de vida y mente evita que una sola solución cierre el espacio de posibilidades. / *Maintaining multiple ways of life and mind prevents a single solution from closing the space of possibilities.* | **Target:** multi-colored blooming biodome bouquet *(Gap: No vibrant multi-species prismatic garden protected from monoculture collapse.)* |
| `F3` | **mapear el universo** (*map the universe*) | `[TIER-D]` | Una investigación común puede unir perspectivas mediante predicciones comprobables. / *Common research can unite perspectives through testable predictions.* | **Target:** synchronized radar triangulation cones carving a 3D cosmic web map *(Gap: F3 misses active 3D cosmic sky-mapping triangulation beams sweeping outer dark vault.)* |
| `F4` | **estudiar conciencia** (*study consciousness*) | `[TIER-C]` | Comprender qué sistemas sienten puede mejorar decisiones sobre derechos, daño y creación de nuevas mentes. / *Understanding what systems feel can improve decisions about rights, harm, and creating new minds.* | **Target:** translucent mind-wave bridge bridging human & machine cores *(Gap: No luminous probe scanning physical brainwave vs qualia threshold.)* |
| `F5` | **archivos cósmicos** (*cosmic files*) | `[TIER-D]` | Conservar memoria a través de grandes escalas temporales permite que descubrimientos sobrevivan a sus autores. / *Preserving memory across large time scales allows discoveries to outlive their authors.* | **Target:** durable faceted diamond monolith radiating archival pulses across deep dark time *(Gap: F5 misses crystalline indestructible deep-time cosmic archive monolith.)* |

##### `Z18` `knowledge-purpose` (`vid=3418`) — Conocimiento como propósito (*Knowledge as purpose*) `[TIER-C]`

- **Catalog Metadata:** `scene=18` · `[0.692, 0.731]` · `rgb=[255, 145, 165]` · `sound=(sine/106.5Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *CONOCER AMPLÍA POSIBILIDADES · NO DECIDE POR SÍ SOLO*
- **Metric & Evidence (ES / EN):** `Observación + modelo + crítica + transmisión → conocimiento acumulativo` / *`Observation + model + criticism + transmission → cumulative knowledge`* · Evidence: `El universo representado dentro de sí · poder para comprender y transformar` (*The universe represented within itself · power to understand and transform*)
- **Spoken Macro Narrative (ES / EN):** Conocer amplía posibilidades, pero su valor depende también de qué capacidades crea y cómo se distribuyen sus riesgos. (*EN:* Knowing expands possibilities, but its value also depends on what capabilities it creates and how its risks are distributed.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` golden scale; only `F5 practical wisdom` matches ethical scale weighing, while knowledge/prediction/dual-use fork lack concrete staging. **Target (`vid=3418`):** Place an illuminated dual-use prism crystal above a multi-scale knowledge torus.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **predecir** (*predict*) | `[TIER-C]` | Un modelo permite anticipar consecuencias y actuar antes de que sean inevitables. / *A model allows us to anticipate consequences and act before they are inevitable.* | **Target:** bright foresight beam intercepting dark asteroid curve ahead of impact *(Gap: No forward trajectory radar beam lighting approaching hazard before impact.)* |
| `F1` | **corregir errores** (*correct errors*) | `[TIER-C]` | La crítica pública puede reemplazar relatos atractivos por explicaciones que funcionan mejor. / *Public criticism can replace attractive narratives with explanations that work better.* | **Target:** noisy red guess collapsing into crisp gold verified ellipse *(Gap: No broken shaky hypothesis ring snapped cleanly into crisp true orbit via public critique.)* |
| `F2` | **conectar escalas** (*connect scales*) | `[TIER-C]` | El conocimiento revela relaciones entre procesos que parecían separados. / *Knowledge reveals relationships between processes that seemed separate.* | **Target:** vertical filament threading subatomic orb -> cell -> star -> halo *(Gap: No micro-to-macro zooming filament tying atom core directly to outer galactic halo.)* |
| `F3` | **preservar descubrimientos** (*preserve discoveries*) | `[TIER-C]` | Archivos y enseñanza permiten que una mente continúe el trabajo de otras. / *Archives and teaching allow one mind to continue the work of others.* | **Target:** stepped glowing archive beacons bridging vanished nodes *(Gap: No glowing archival beacon passing torch across disappearing builder silhouette.)* |
| `F4` | **poder peligroso** (*dangerous power*) | `[TIER-C]` | La misma comprensión puede habilitar medicina, vigilancia, explotación o armas. / *The same understanding can enable medicine, surveillance, exploitation or weapons.* | **Target:** single bright ray bifurcating cleanly into serene healing dome left vs harsh red lance right *(Gap: No single dual-use glowing crystal splitting simultaneously into healing balm halo and jagged weapon spear.)* |
| `F5` | **sabiduría práctica** (*practical wisdom*) | `[TIER-B]` | Saber qué es posible no decide por sí solo qué debería hacerse. / *Knowing what is possible does not alone decide what should be done.* | **Target:** dropping both sides onto the golden scale beam to weigh choice (`doc_cue` lock) *(Gap: F5 practical wisdom resonates with balance scale motif, weighing possible power vs ethical end.)* |

##### `Z19` `beauty-contemplation` (`vid=3419`) — Belleza y contemplación (*Beauty and contemplation*) `[TIER-D]`

- **Catalog Metadata:** `scene=19` · `[0.731, 0.769]` · `rgb=[230, 145, 220]` · `sound=(sawtooth/109.85Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *NO TODO VALOR ES UNA HERRAMIENTA PARA OTRA META*
- **Metric & Evidence (ES / EN):** `Patrón + percepción + atención + resonancia → experiencia estética` / *`Pattern + perception + attention + resonance → aesthetic experience`* · Evidence: `Valor no instrumental · experiencias dignas sin utilidad posterior` (*Non-instrumental value · worthy experiences without later utility*)
- **Spoken Macro Narrative (ES / EN):** Una inteligencia puede valorar no solo lo que permite sobrevivir, sino también aquello que considera digno de ser experimentado. (*EN:* An intelligence can value not only what allows it to survive, but also what it considers worthy of being experienced.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` rigid utilitarian brass balance scales; actively clashes with non-instrumental beauty—zero musical waves, mathematical symmetry crystal, terrain/sky landscape, playful toy loop, or orbital resonance. **Target (`vid=3419`):** Replace utilitarian brass hardware with a breathtaking non-utilitarian aesthetic sanctuary morphing per `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **música** (*music*) | `[TIER-D]` | Relaciones temporales pueden producir expectativa, tensión y resolución sin describir un objeto externo. / *Temporal relationships can produce expectation, tension, and resolution without describing an external object.* | **Target:** shimmering Chladni-like harmonic sound waves building tension and resolving into golden rosette (`voice`/`bass` synced) *(Gap: F0 misses shimmering standing acoustic harmonic waves resolving dissonance into chord.)* |
| `F1` | **elegancia matemática** (*mathematical elegance*) | `[TIER-D]` | Una demostración puede valorarse por economía, simetría o capacidad de unir ideas. / *A proof can be valued for its economy, symmetry, or ability to unite ideas.* | **Target:** minimalist radiant golden icosahedron/Mobius knot unfolding crisp dual symmetry *(Gap: F1 misses crisp iridescent minimal geometric polyhedron/Mobius proof symmetry.)* |
| `F2` | **paisaje** (*landscape*) | `[TIER-D]` | Una configuración natural puede reunir memoria, escala, color y pertenencia. / *A natural setting can bring together memory, scale, color and belonging.* | **Target:** sculpted layered twilight canyon/cloud ridges illuminated in warm horizon gold *(Gap: F2 misses sweeping volumetric terrain ridges, mist horizons, and warm sunlit sky.)* |
| `F3` | **juego** (*game*) | `[TIER-D]` | Explorar reglas por sí mismas crea aprendizaje y disfrute sin una meta productiva inmediata. / *Exploring rules for their own sake creates learning and enjoyment without an immediate productive goal.* | **Target:** playful braided colorful particles weaving fluid spirals *(Gap: F3 misses joyful undulating ribbon helices weaving playful rule-patterns in free space.)* |
| `F4` | **estética alienígena** (*alien aesthetic*) | `[TIER-D]` | Otras mentes podrían admirar resonancias gravitatorias, ciclos químicos o continuidad histórica. / *Other minds might admire gravitational resonances, chemical cycles, or historical continuity.* | **Target:** hypnotic 3:2/4:3 Laplace-like orbital resonance rings & Belousov-Zhabotinsky chemical waves *(Gap: F4 misses dancing Keplerian 3-body gravitational resonance choreography & chemical wave.)* |
| `F5` | **contemplación** (*contemplation*) | `[TIER-C]` | Atender sin intentar controlar puede transformar la relación con lo observado. / *Attending without trying to control can transform the relationship with what is observed.* | **Target:** gentle still cyan lotus ring breathing smoothly with zero mechanical chains *(Gap: No still serene luminous halo simply witnessing radiant pattern without grasping/tilting.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP24-Z15-19)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP24-Z15-19` (Value Conflicts, Moral Restraint, Cross-Generational Cathedrals, Knowledge & Aesthetic Wonder) for HyperCosmosVR Odyssey Act V (`Exp 24: 24_cosmic_meaning`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z15`->`3415` (`F0..F5`), `Z16`->`3416` (`F0..F5`), `Z17`->`3417` (`F0..F5`), `Z18`->`3418` (`F0..F5`), `Z19`->`3419` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3856`, `3857`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP24-Z15-19`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z15: meaning-conflicts / Conflicts of meaning, Z16: moral-limits / Moral limits, Z17: shared-projects / Shared projects, Z18: knowledge-purpose / Knowledge as purpose, Z19: beauty-contemplation / Beauty and contemplation.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp24-z20-22"></a>
### PKG-ACT5-EXP24-Z20-22: Care & Fragile Repair, Global Biosphere Stewardship & Chosen Meaning Without Script

- **Phase:** Phase 1/2 | **Priority:** `P2` | **Scope:** Exp `24` (`24_cosmic_meaning`) `Z20..Z22` (`3` zones, `18` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L84-L87`: `24 => { 24 => 241, _ => 240 }` (all `Z00..Z23, Z25` share `240`); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L464-L494` (front-locked Pale Blue Mote + twin Astrolabe Shields + Golden Ethical Balance Scales; `tilt = sin(0.8*t + 0.85*feat)*0.10` or static `0.22` at `Z24`). Tier breakdown for this batch: **`0A / 1B / 2C / 0D` Zones**, **`0A / 1B / 13C / 4D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `0/3` zones and `0/18` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3420..=3422` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3858` (allocated from `3720 + (24 - 11)*10 + k`; Defensive Astrolabe Mote & Stewardship Vectors (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z20` | `care-relation` (*Cuidado y relación* / *Care and relationship*) | `[TIER-C]` | `vid=240` | **`3420`** | `3858` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z21` | `planetary-responsibility` (*Responsabilidad planetaria* / *Planetary responsibility*) | `[TIER-B]` | `vid=240` | **`3421`** | `3858` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z22` | `chosen-purpose` (*Propósito elegido* / *Chosen purpose*) | `[TIER-C]` | `vid=240` | **`3422`** | `3858` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z20` `care-relation` (`vid=3420`) — Cuidado y relación (*Care and relationship*) `[TIER-C]`

- **Catalog Metadata:** `scene=20` · `[0.769, 0.808]` · `rgb=[180, 150, 255]` · `sound=(sine/113.2Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *CUIDAR ES RECONOCER UN MUNDO DE CONSECUENCIAS EN OTRO*
- **Metric & Evidence (ES / EN):** `Reconocer otra experiencia + vulnerabilidad + vínculo → cuidado` / *`Recognize another experience + vulnerability + bond → care`* · Evidence: `Importancia entre perspectivas · proteger posibilidades ajenas` (*Importance between perspectives · protecting other people's possibilities*)
- **Spoken Macro Narrative (ES / EN):** Cuidar es tratar la perspectiva de otro ser como un lugar donde las consecuencias pueden importar, no solo como un objeto dentro del propio proyecto. (*EN:* Caring is treating another being's perspective as a place where consequences can matter, not just as an object within one's own project.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` single mechanical scale; zero vulnerable infant orb, ecosystem web, remembrance halo, or interstellar communication beam delay. **Target (`vid=3420`):** Stage two asymmetric glowing spheres bridged by protective golden care arcs.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **descendencia** (*offspring*) | `[TIER-C]` | Proteger a quien todavía depende de otros extiende responsabilidad hacia futuros no elegidos. / *Protecting those who still depend on others extends responsibility to unchosen futures.* | **Target:** large warm protective crescent cradle shielding small pulsing infant core *(Gap: No warm overarching guardian dome sheltering fragile newborn glowing core.)* |
| `F1` | **extraño** (*strange*) | `[TIER-C]` | La consideración puede superar parentesco, reciprocidad inmediata y pertenencia cultural. / *Consideration can overcome kinship, immediate reciprocity, and cultural belonging.* | **Target:** amber ring & cool spiky cyan octahedron locking tender bridge beam *(Gap: No bright bridge span crossing chasm between alien distinct color/shape orbs.)* |
| `F2` | **ecosistema** (*ecosystem*) | `[TIER-D]` | Cuidar una red viva reconoce relaciones que ningún organismo aislado contiene. / *Caring for a living network recognizes relationships that no isolated organism contains.* | **Target:** living green interconnected mesh where pulling one node ripples gently across all *(Gap: F2 misses dynamic interconnected green biosphere web uniting diverse living species.)* |
| `F3` | **memoria de los muertos** (*memory of the dead*) | `[TIER-C]` | Recordar puede conservar compromisos y aprendizajes sin fingir que la pérdida desapareció. / *Remembering can preserve commitments and learnings without pretending the loss went away.* | **Target:** soft glowing golden memorial wreath circling silent empty socket *(Gap: No tender luminous memory ring hovering around an extinguished hollow star socket.)* |
| `F4` | **mente artificial dañada** (*damaged artificial mind*) | `[TIER-C]` | La incertidumbre sobre experiencia exige equilibrar cautela moral y evidencia. / *Uncertainty about experience requires balancing moral caution and evidence.* | **Target:** shielding astrolabe ring holding over flickering synthetic node under soft question light *(Gap: No careful protective halo hovering delicately over flickering uncertain AI spark.)* |
| `F5` | **civilización distante** (*distant civilization*) | `[TIER-D]` | Comunicarse responsablemente implica considerar efectos que ocurrirán mucho después de enviar un mensaje. / *Communicating responsibly involves considering effects that will occur long after a message is sent.* | **Target:** interstellar wave packet traveling across deep void toward remote sunlit world *(Gap: F5 misses long interstellar signal arc glowing forward across vast stellar gap with future consequence ripple.)* |

##### `Z21` `planetary-responsibility` (`vid=3421`) — Responsabilidad planetaria (*Planetary responsibility*) `[TIER-B]`

- **Catalog Metadata:** `scene=21` · `[0.808, 0.846]` · `rgb=[120, 190, 255]` · `sound=(triangle/116.55Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *UNA CIVILIZACIÓN TECNOLÓGICA CONSTRUYE EL MUNDO HEREDADO*
- **Metric & Evidence (ES / EN):** `Influencia planetaria + conocimiento de riesgos + generaciones futuras → deber de cuidado` / *`Planetary influence + risk awareness + future generations → duty of care`* · Evidence: `Poder global · decisiones presentes configuran el mundo heredado` (*Global power · present decisions shape the inherited world*)
- **Spoken Macro Narrative (ES / EN):** Una civilización tecnológica ya no solo habita su planeta: modifica las condiciones de existencia de seres presentes y futuros. (*EN:* A technological civilization no longer only inhabits its planet: it modifies the conditions of existence of present and future beings.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` 'Pale Blue Mote + Astrolabe Shield + Balance Scales' aligns directly with shielding a planetary globe & balancing global power (`shield_a/b` enveloping `mote`), making it `TIER-B`, yet `doc_feat=0..5` fails to show climate bands, biosphere web, fair energy mesh, or incoming asteroid deflection. **Target (`vid=3421`):** Transform legacy `vid=240`'s `mote` + `shield_a/b` into a detailed interactive Earth-stewardship stage controlled by `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **clima estable** (*stable climate*) | `[TIER-D]` | Alterar ciclos planetarios cambia seguridad, ecosistemas y distribución de oportunidades. / *Altering planetary cycles changes security, ecosystems and distribution of opportunities.* | **Target:** crisp banded cyan atmosphere with warm/cool convective storms calmed by shield balance *(Gap: F0 misses atmospheric circulation belts & thermal balance stripes over cyan mote.)* |
| `F1` | **biodiversidad** (*biodiversity*) | `[TIER-D]` | Cada extinción elimina una historia evolutiva y posibilidades que no conocemos por completo. / *Each extinction eliminates an evolutionary history and possibilities that we do not fully know.* | **Target:** luminous emerald biosphere filigree wrapping mote *(Gap: F1 misses fragile emerald biosphere lattice dimming/fragmenting cleanly.)* |
| `F2` | **riesgo tecnológico** (*technological risk*) | `[TIER-C]` | Sistemas poderosos requieren límites, pruebas y capacidad de corrección antes de decisiones irreversibles. / *Powerful systems require limits, testing, and correction capabilities before irreversible decisions.* | **Target:** crisp hexagonal safety containment cage clamping over-bright technological sparks *(Gap: No containment boundary limiter encircling powerful runaway core beneath scales.)* |
| `F3` | **recursos justos** (*fair resources*) | `[TIER-C]` | Quién recibe energía, protección y conocimiento determina qué vidas pueden desarrollarse. / *Who receives energy, protection and knowledge determines what lives can unfold.* | **Target:** golden global distribution conduits balancing dim vs bright hemispheres *(Gap: No fair/unfair radiant energy distribution arcs feeding dark vs bright hemispheres.)* |
| `F4` | **defensa planetaria** (*planetary defense*) | `[TIER-B]` | Prevenir impactos y otros riesgos naturales convierte previsión científica en cuidado colectivo. / *Preventing impacts and other natural risks turns scientific foresight into collective care.* | **Target:** fiery red asteroid streak (`sd_capsule`) bending off glowing outer astrolabe `shield_b` (`kw:defensa`) *(Gap: F4 planetary defense strongly evokes `shield_a/b` rings, but lacks incoming red asteroid streak deflected away.)* |
| `F5` | **generaciones futuras** (*future generations*) | `[TIER-C]` | Quienes todavía no existen no pueden negociar, pero vivirán dentro de las condiciones que dejamos. / *Those who do not yet exist cannot negotiate, but they will live within the conditions we left.* | **Target:** fading succession of downstream future-Earth echoes extending forward into deep time *(Gap: No nested ghost planet spheres projecting downstream into unborn future horizons.)* |

##### `Z22` `chosen-purpose` (`vid=3422`) — Propósito elegido (*Chosen purpose*) `[TIER-C]`

- **Catalog Metadata:** `scene=22` · `[0.846, 0.885]` · `rgb=[100, 225, 205]` · `sound=(sine/119.9Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *ELEGIR ES EXAMINAR QUÉ METAS CONTINUAR*
- **Metric & Evidence (ES / EN):** `Impulso biológico + relato cultural + reflexión + compromiso → dirección elegida` / *`Biological impulse + cultural story + reflection + commitment → chosen direction`* · Evidence: `Herencia examinada · continuar, revisar o rechazar metas recibidas` (*Heritage examined · continue, review or reject received goals*)
- **Spoken Macro Narrative (ES / EN):** No elegimos todas las condiciones de nuestra existencia, pero podemos participar en la dirección que damos a esas condiciones. (*EN:* We do not choose all the conditions of our existence, but we can participate in the direction we give to those conditions.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` static golden scale; fails to depict examining, pruning, or forging chosen vector paths against inherited streams. **Target (`vid=3422`):** Build a prism-compass forge where raw incoming ribbons pass through a conscious central aperture.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **impulsos heredados** (*inherited impulses*) | `[TIER-C]` | Necesidades y emociones aportan motivos sin determinar una única vida correcta. / *Needs and emotions provide motives without determining a single correct life.* | **Target:** primal red/orange organic impulse waves filtered through clear crystal *(Gap: No raw pulsing biological drive flares entering prism core for reflective sorting.)* |
| `F1` | **relatos recibidos** (*stories received*) | `[TIER-C]` | La cultura ofrece metas y categorías que pueden aceptarse, combinarse o criticarse. / *Culture offers goals and categories that can be accepted, combined, or criticized.* | **Target:** orbiting ancestral text-ring strips selectively adopted or released *(Gap: No tangled inherited cultural scroll bands examined & selectively rewoven.)* |
| `F2` | **evidencia** (*evidence*) | `[TIER-C]` | Comprender consecuencias limita propósitos basados en creencias falsas. / *Understanding consequences limits purposes based on false beliefs.* | **Target:** sharp turquoise light beam dissolving ghostly mirage branches *(Gap: No empirical reality beam shattering hollow mirage trajectories.)* |
| `F3` | **compromiso** (*commitment*) | `[TIER-C]` | Elegir significa sostener una orientación a través de costes y cambios. / *Choosing means sustaining an orientation through costs and changes.* | **Target:** single unbroken golden beam piercing heavy ambient noise turbulence *(Gap: No steadfast straight luminous beam piercing through swirling storm turbulence.)* |
| `F4` | **revisión** (*review*) | `[TIER-C]` | Un propósito vivo puede aprender de daño inesperado y nueva información. / *A living purpose can learn from unexpected harm and new information.* | **Target:** curving beam flexing smoothly around sudden warning obstacle *(Gap: No sharp feedback bend adjusting trajectory cleanly around newly discovered harm.)* |
| `F5` | **coherencia abierta** (*open coherence*) | `[TIER-C]` | La dirección no necesita ser eterna para organizar acciones de manera honesta y responsable. / *A sense of direction need not be eternal to organize action honestly and responsibly.* | **Target:** radiant open finite arc glowing warmly without rigid end-wall *(Gap: No open horizon trajectory marching confidently without infinite eternal stone pillar.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP24-Z20-22)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP24-Z20-22` (Care & Fragile Repair, Global Biosphere Stewardship & Chosen Meaning Without Script) for HyperCosmosVR Odyssey Act V (`Exp 24: 24_cosmic_meaning`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z20`->`3420` (`F0..F5`), `Z21`->`3421` (`F0..F5`), `Z22`->`3422` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3858`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP24-Z20-22`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z20: care-relation / Care and relationship, Z21: planetary-responsibility / Planetary responsibility, Z22: chosen-purpose / Chosen purpose.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp24-z23-25"></a>
### PKG-ACT5-EXP24-Z23-25: Lucid Action Under Uncertainty, Self-Valuing Cosmos (`vid=241` Upgrade) & Final Transition

- **Phase:** Phase 1/2 | **Priority:** `P2` | **Scope:** Exp `24` (`24_cosmic_meaning`) `Z23..Z25` (`3` zones, `18` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L84-L87`: `24 => { 24 => 241, _ => 240 }` (all `Z00..Z23, Z25` share `240`); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L464-L494` (front-locked Pale Blue Mote + twin Astrolabe Shields + Golden Ethical Balance Scales; `tilt = sin(0.8*t + 0.85*feat)*0.10` or static `0.22` at `Z24`). Tier breakdown for this batch: **`0A / 1B / 2C / 0D` Zones**, **`0A / 6B / 12C / 0D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/3` zones and `0/18` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3423..=3425` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3859` (allocated from `3720 + (24 - 11)*10 + k`; Ouroboros Mirror Lotus Crown (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z23` | `meaning-without-certainty` (*Significado sin certeza* / *Meaning without certainty*) | `[TIER-C]` | `vid=240` | **`3423`** | `3859` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z24` | `self-valuing-universe` (*El universo que se valora a sí mismo* / *The universe that values itself*) | `[TIER-B]` | `vid=241` | **`3424`** | `3859` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z25` | `final-horizon` (*Hacia el horizonte final* / *Towards the final horizon*) | `[TIER-C]` | `vid=240` | **`3425`** | `3859` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 semantic cue mask) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z23` `meaning-without-certainty` (`vid=3423`) — Significado sin certeza (*Meaning without certainty*) `[TIER-C]`

- **Catalog Metadata:** `scene=23` · `[0.885, 0.923]` · `rgb=[150, 235, 170]` · `sound=(sawtooth/123.25Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *SIN GARANTÍA FINAL · CON RAZONES PARA ACTUAR*
- **Metric & Evidence (ES / EN):** `Razones parciales + consecuencias comparables + incertidumbre → decisión responsable` / *`Partial reasons + comparable consequences + uncertainty → responsible decision`* · Evidence: `Compromiso provisional · actuar sin garantía metafísica` (*Provisional commitment · act without metaphysical guarantee*)
- **Spoken Macro Narrative (ES / EN):** La falta de una respuesta final no impide formar compromisos, comparar futuros ni cuidar aquello que puede perderse. (*EN:* The lack of a final answer does not prevent us from forming commitments, comparing futures or taking care of what may be lost.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` static rocking scales; misses fog-bound horizon lantern / provisional navigation bridge without rigid bedrock. **Target (`vid=3423`):** Render an explorer's luminous spherical lantern suspended inside misty dark unknown expanse.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **origen desconocido** (*unknown origin*) | `[TIER-C]` | No conocer por qué existe el universo no borra las experiencias que ocurren dentro de él. / *Not knowing why the universe exists does not erase the experiences that occur within it.* | **Target:** vivid warm lantern hearth contrasting pitch-black unseen rim *(Gap: No warm luminous local campfire/lantern ring glowing brightly despite pitch-black outer horizon.)* |
| `F1` | **conciencia abierta** (*open awareness*) | `[TIER-C]` | La teoría incompleta de la mente exige cautela, no indiferencia hacia posibles sujetos. / *The incomplete theory of mind requires caution, not indifference toward possible subjects.* | **Target:** soft protective amber aura circling dim uncertain candidate orbs *(Gap: No cautious soft protective halo around dim ambiguous potential minds.)* |
| `F2` | **moralidad debatida** (*debated morality*) | `[TIER-C]` | No poseer una prueba absoluta no vuelve equivalentes todas las consecuencias. / *Not having absolute proof does not make all consequences equivalent.* | **Target:** stark radiant peak vs deep dark valley beneath diffused overhead mist *(Gap: No non-flat asymmetric terrain proving outcomes remain vastly unequal despite dim overhead light.)* |
| `F3` | **futuro incierto** (*uncertain future*) | `[TIER-C]` | Un proyecto puede ser razonable aunque no exista garantía de éxito o permanencia. / *A project can be reasonable even if there is no guarantee of success or permanence.* | **Target:** delicate glowing truss bridge extending bravely into dark unknown *(Gap: No slender courageous bridge arc built out across misty uncertain chasm.)* |
| `F4` | **humildad** (*humility*) | `[TIER-C]` | Reconocer límites permite revisar decisiones antes de convertirlas en dogma. / *Recognizing limits allows us to review decisions before turning them into dogma.* | **Target:** pliable luminous contour lines flexing gently rather than freezing into dogma crystal *(Gap: No flexible breathing wireframe lattice avoiding brittle crystal shattering.)* |
| `F5` | **esperanza sin promesa** (*hope without promise*) | `[TIER-C]` | Intentar construir un futuro mejor no requiere creer que la historia lo producirá automáticamente. / *Trying to build a better future does not require believing that history will automatically produce it.* | **Target:** ascending footsteps of bright golden light forged solely by active agents (`zone_col`) *(Gap: No human-forged ascending spark stairway rising without automatic destiny beam.)* |

##### `Z24` `self-valuing-universe` (`vid=3424`) — El universo que se valora a sí mismo (*The universe that values itself*) `[TIER-B]`

- **Catalog Metadata:** `scene=24` · `[0.923, 0.962]` · `rgb=[220, 225, 145]` · `sound=(sine/126.6Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *EL UNIVERSO CONTIENE LUGARES CAPACES DE VALORAR FUTUROS*
- **Metric & Evidence (ES / EN):** `Estrellas → elementos → planetas → vida → experiencia → reflexión → propósito` / *`Stars → elements → planets → life → experience → reflection → purpose`* · Evidence: `Emergencia cósmica · materia capaz de sentir, comprender y elegir` (*Cosmic emergence · matter capable of feeling, understanding and choosing*)
- **Spoken Macro Narrative (ES / EN):** No es necesario imaginar un universo consciente como totalidad: basta reconocer procesos locales donde el universo se representa y evalúa dentro de sí. (*EN:* It is not necessary to imagine a conscious universe as a totality: it is enough to recognize local processes where the universe is represented and evaluated within itself.)
- **Current Diagnosis & Target 3D Blueprint:** ONLY zone with bespoke `doc_stages.rs:85` (`24 => 241`). Yet WGSL `hero_doc_civilization_horizon.wgsl:466,475` ONLY freezes balance scale tilt at `0.22` (`mode == 1u`) and COMPLETELY IGNORES `doc_feat`! (`f32(feat)` only runs in `mode == 0u`). **Target (`vid=3424`):** Upgrade legacy `vid=241` so the Pale Blue Mote & Scales form a recursive ouroboros lens—the cosmos beholding & weighing itself—with true `doc_feat=0..5` sub-staging.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **admiración** (*admiration*) | `[TIER-B]` | La materia organizada puede experimentar asombro ante estructuras de las que ella misma forma parte. / *Organized matter can experience amazement at structures of which it itself is a part.* | **Target:** outer nebula light curving inward into luminous eye/mote glowing back in awe *(Gap: `vid=241` ignores `doc_feat=0` completely (locked 12.6 deg tilt; no cosmic-mirror amazement bloom).)* |
| `F1` | **duelo** (*grief*) | `[TIER-B]` | El universo contiene regiones capaces de reconocer pérdidas y recordar lo que ya no existe. / *The universe contains regions capable of recognizing loss and remembering what no longer exists.* | **Target:** faint ghost-star dimming inside mote halo surrounded by golden remembrance wave *(Gap: `vid=241` ignores `doc_feat=1` completely (no extinguished-world mourning ripple inside cosmos).)* |
| `F2` | **comprensión** (*understanding*) | `[TIER-B]` | Modelos construidos por mentes permiten que procesos cósmicos sean representados dentro del cosmos. / *Models constructed by minds allow cosmic processes to be represented within the cosmos.* | **Target:** intricate nested miniature galaxy holographic orb floating inside the observer mote *(Gap: `vid=241` ignores `doc_feat=2` completely (no internal miniature cosmos model sphere inside mind).)* |
| `F3` | **preservación** (*preservation*) | `[TIER-B]` | Seres conscientes pueden intentar conservar vida, memoria y condiciones habitables. / *Conscious beings can attempt to preserve life, memory, and habitable conditions.* | **Target:** dual cyan astrolabe rings locking defensively around fragile biosphere mote *(Gap: `vid=241` ignores `doc_feat=3` completely (no astrolabe preservation shield tightening around biosphere).)* |
| `F4` | **creación** (*creation*) | `[TIER-B]` | Nuevas obras, organismos, sistemas y formas de mente amplían lo que el universo contiene. / *New works, organisms, systems and forms of mind expand what the universe contains.* | **Target:** new radiant geometric artifacts blooming outward from balance pans *(Gap: `vid=241` ignores `doc_feat=4` completely (no newly blooming artistic/synthetic forms expanding ring).)* |
| `F5` | **elección** (*choice*) | `[TIER-B]` | Distintos futuros dejan de ser físicamente posibles de manera indiferente y se convierten en alternativas evaluadas. / *Different futures cease to be physically possible indifferently and become evaluated alternatives.* | **Target:** glowing fork of alternative futures tipped cleanly toward chosen gold trajectory (`doc_cue` synced) *(Gap: `vid=241` ignores `doc_feat=5` completely (no branching evaluated future arcs hovering over scales).)* |

##### `Z25` `final-horizon` (`vid=3425`) — Hacia el horizonte final (*Towards the final horizon*) `[TIER-C]`

- **Catalog Metadata:** `scene=25` · `[0.962, 1.000]` · `rgb=[245, 240, 220]` · `sound=(triangle/129.95Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `horizonte` · Footer: *MUCHAS INTELIGENCIAS · UN FUTURO COMPARTIDO E INACABADO*
- **Metric & Evidence (ES / EN):** `Propósito sin certeza + responsabilidad compartida + límites de existencia → horizonte final` / *`Purpose without certainty + shared responsibility + limits of existence → final horizon`* · Evidence: `Frontera abierta · una sola historia incompleta de vida, mente y universo` (*Open frontier · a single incomplete story of life, mind and universe*)
- **Spoken Macro Narrative (ES / EN):** La experiencia termina sin una respuesta única, pero deja visible la posibilidad de construir futuros mejores o peores para quienes los vivirán. (*EN:* The experience ends without a single answer, but it leaves visible the possibility of building better or worse futures for those who will live them.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=240` identical generic scale as Z00..Z23; ignores Z25 bridge into Exp 25, ignores `kw:horizonte` exact match in resolver (`resolver.rs:960`), and shows no threshold portal. **Target (`vid=3425`):** Morph legacy `vid=240` smoothly toward Exp 25's boundary architecture across `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **significado descubierto** (*discovered meaning*) | `[TIER-C]` | Puede existir una dimensión de valor que las mentes reconocen en lugar de inventar por completo. / *There may be a dimension of value that minds recognize rather than invent entirely.* | **Target:** faint eternal geometric grid etched deep in void *(Gap: No pre-existing crystalline value pattern shimmering softly in background strata.)* |
| `F1` | **significado creado** (*created meaning*) | `[TIER-C]` | Los propósitos pueden surgir cuando agentes eligen y sostienen compromisos. / *Purposes can emerge when agents choose and hold commitments.* | **Target:** golden luminous filaments spun directly by orbiting spark agents *(Gap: No active golden agent beams weaving fresh architectural lattice.)* |
| `F2` | **significado relacional** (*relational meaning*) | `[TIER-C]` | Lo importante puede existir principalmente entre seres, memorias, lugares y futuros. / *What is important can exist mainly between beings, memories, places and futures.* | **Target:** bright resonant harmonic edges humming solely *between* paired orbs *(Gap: No glowing tensioned connection matrix existing *between* rather than inside orbs.)* |
| `F3` | **significado emergente** (*emerging meaning*) | `[TIER-C]` | La vida y la conciencia pueden introducir propiedades que no estaban presentes en componentes aislados. / *Life and consciousness can introduce properties that were not present in isolated components.* | **Target:** layered base-particles bursting into high iridescent crown complexity *(Gap: No multi-tier phase transition sparking novel emergent crown colors above plain base.)* |
| `F4` | **pluralidad transformable** (*transformable plurality*) | `[TIER-C]` | Los valores cambian a medida que cambian las formas de inteligencia y sus relaciones. / *Values change as the forms of intelligence and their relationships change.* | **Target:** morphing morphogenetic prism reshaping value contours smoothly *(Gap: No shifting polymorphic horizon petals morphing as agent shapes evolve.)* |
| `F5` | **más allá** (*beyond*) | `[TIER-C]` | Si la inteligencia puede construir significado sin poseer respuestas definitivas, queda una última frontera: ¿qué puede llegar a ser la existencia cuando vida, mente, civilización y universo son contemplados como una sola historia incompleta? / *If intelligence can construct meaning without possessing definitive answers, a final frontier remains: what can existence become when life, mind, civilization, and universe are viewed as a single incomplete story?* | **Target:** dissolve balance chains into a looming concentric Horizon Portal (`25`-notched ring echoing Exp 25's lotus gateway) *(Gap: F5 culminates in the direct handover question to Exp 25 without opening a 25-petal horizon gate.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP24-Z23-25)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP24-Z23-25` (Lucid Action Under Uncertainty, Self-Valuing Cosmos (`vid=241` Upgrade) & Final Transition) for HyperCosmosVR Odyssey Act V (`Exp 24: 24_cosmic_meaning`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z23`->`3423` (`F0..F5`), `Z24`->`3424` (`F0..F5`), `Z25`->`3425` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3859`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP24-Z23-25`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z23: meaning-without-certainty / Meaning without certainty, Z24: self-valuing-universe / The universe that values itself, Z25: final-horizon / Towards the final horizon.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

### 4.8 Experience 25: `25_final_horizon` — Espectro del Horizonte Final: del Propósito Cósmico a los Límites de la Existencia

- **Odyssey & Spatial Placement:** Act 5 (`8/8`, overall stop #25 — Odyssey Grand Finale) · `HYPER_LOTUS_25_HORIZON` (`parent UNIVERSE_ROOT`, `u = [27.5, 29.0]`, `focus_scale_exp = 28.2`, `2.0e28 m`) ([`topology.rs#L390-L401, L657`](../../crates/cosmos_spatial/src/topology.rs#L390-L401))
- **Experience Totals:** `25` Zones (`0A / 1B / 6C / 18D`) · `150` Features (`0A / 6B / 38C / 106D`)
- **Current Shader & Compute:** `doc_stages.rs#L88`: `25 => 250` (single variant across all 25 zones!) · `shaders/hero_doc_civilization_horizon.wgsl#L495-L518` (3 nested longitude-ribbed wireframe spheres `r=0.24,0.44,0.64` + 25-petal HSL lotus ring + `horizon_fade`/`slow_rate` across `z=0..24`; `feat` shifts sphere phase `0.3*feat` at `L504`) · `mode=15` (`universe_scene.rs#L323`, `compute_universe.wgsl#L347-L357` 25-petal lotus) + `far_shell.wgsl#L150-L161` (`lotus_boost=0.42`)

<a id="pkg-act5-exp25-z00-04"></a>
### PKG-ACT5-EXP25-Z00-04: Open Future Prelude, Evolved Limits, Mind Redesign, Identity Continuity & Postbiology

- **Phase:** Phase 1 + Phase 4 | **Priority:** `P0` | **Scope:** Exp `25` (`25_final_horizon`) `Z00..Z04` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L88`: `25 => 250` (single variant across all 25 zones!); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L495-L518` (3 nested longitude-ribbed wireframe spheres `r=0.24,0.44,0.64` + 25-petal HSL lotus ring + `horizon_fade`/`slow_rate` across `z=0..24`; `feat` shifts sphere phase `0.3*feat` at `L504`). Tier breakdown for this batch: **`0A / 0B / 1C / 4D` Zones**, **`0A / 0B / 6C / 24D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `0/5` zones and `0/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3500..=3504` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3860`, `3861` (allocated from `3720 + (25 - 11)*10 + k`; Morphing Mind-Substrate Lotus Field (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z00` | `incomplete-history` (*Una sola historia incompleta* / *A single incomplete story*) | `[TIER-C]` | `vid=250` | **`3500`** | `3860` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z01` | `biological-limits` (*Los límites biológicos* / *The biological limits*) | `[TIER-D]` | `vid=250` | **`3501`** | `3861` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z02` | `mind-transformation` (*La transformación de la mente* / *The transformation of the mind*) | `[TIER-D]` | `vid=250` | **`3502`** | `3860` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z03` | `identity-continuity` (*Identidad y continuidad* / *Identity and continuity*) | `[TIER-D]` | `vid=250` | **`3503`** | `3861` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z04` | `postbiological-life` (*Vida postbiológica* / *Postbiological life*) | `[TIER-D]` | `vid=250` | **`3504`** | `3860` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z00` `incomplete-history` (`vid=3500`) — Una sola historia incompleta (*A single incomplete story*) `[TIER-C]`

- **Catalog Metadata:** `scene=0` · `[0.000, 0.040]` · `rgb=[246, 232, 205]` · `sound=(triangle/76Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *NINGÚN FUTURO ES INEVITABLE*
- **Metric & Evidence (ES / EN):** `Propósito elegido + capacidad física + incertidumbre → varios futuros realmente posibles` / *`Chosen purpose + physical ability + uncertainty → several truly possible futures`* · Evidence: `Continuidad · muchas inteligencias heredan un futuro todavía no decidido` (*Continuity · many intelligences inherit a future not yet decided*)
- **Spoken Macro Narrative (ES / EN):** Preservar, expandirse, transformarse, coexistir, retirarse o terminar son trayectorias posibles; ninguna está garantizada ni es inevitable. (*EN:* Preserve, expand, transform, coexist, withdraw or terminate are possible trajectories; none are guaranteed or inevitable.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` renders 3 slotted longitude spheres + 25-petal horizontal rainbow lotus (`horizon_fade=1.0`, `slow_rate=1.0`, petal highlight at `0/25`); `doc_feat` rotates spheres by 17.2 deg (`f32(feat)*0.3`) with zero trajectory branching. **Target (`vid=3500`):** Build a 6-branching cosmic trajectory compass launching out from Earth/present origin seed.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **preservación** (*preservation*) | `[TIER-C]` | Conservar vida, memoria y diversidad puede orientar una civilización sin exigir que permanezca idéntica para siempre. / *Preserving life, memory, and diversity can guide a civilization without requiring it to remain identical forever.* | **Target:** emerald preservation shield orb floating steadily *(Gap: Only subtle 17 deg cage rotation; no preserved living seed-globe halo.)* |
| `F1` | **expansión** (*expansion*) | `[TIER-D]` | Extenderse a nuevos hábitats aumenta oportunidades y también multiplica distancias, riesgos y responsabilidades. / *Expanding to new habitats increases opportunities and also multiplies distances, risks and responsibilities.* | **Target:** radial fan of outward golden interstellar colonizing streaks *(Gap: F1 expansion lacks outward multi-system colonization beacon rays multiplying across space.)* |
| `F2` | **transformación** (*transformation*) | `[TIER-C]` | Rediseñar cuerpos, mentes y entornos puede abrir capacidades que alteren aquello que la civilización entiende por continuar. / *Redesigning bodies, minds and environments can open up capabilities that alter what civilization means to continue.* | **Target:** iris-morphing crystalline postbiological knot shedding old shell *(Gap: Only 17 deg rotation; no morphing topological self-redesign metamorphosis.)* |
| `F3` | **coexistencia** (*coexistence*) | `[TIER-C]` | Coordinar futuros con otras formas de vida o inteligencia exige límites compartidos, traducción y renuncias mutuas. / *Coordinating futures with other forms of life or intelligence requires shared boundaries, translation, and mutual renunciations.* | **Target:** two contrasting glowing rings locking cleanly via harmonic boundary ring *(Gap: Only 17 deg rotation; no shared boundary negotiation torus between twin habitats.)* |
| `F4` | **retiro** (*withdrawal*) | `[TIER-C]` | Una civilización puede elegir menor escala, baja actividad o aislamiento en vez de expansión continua. / *A civilization may choose smaller scale, low activity, or isolation rather than continued expansion.* | **Target:** calm compact amber sanctuary sphere dimming peacefully toward deep void *(Gap: Only 17 deg rotation; no softly dimming minimalist sanctuary contracting gently inward.)* |
| `F5` | **extinción e investigación** (*extinction and research*) | `[TIER-C]` | Terminar sigue siendo posible; continuar preguntando también, aunque ninguna respuesta final esté asegurada. / *Ending is still possible; continue to ask too, although no final answer is assured.* | **Target:** twin diverging tips—one fizzling to dark soot, one bright curious searchlight cone piercing dark frontier *(Gap: Only 17 deg rotation; no branching fork contrasting dark vanished spur vs glowing quest ray.)* |

##### `Z01` `biological-limits` (`vid=3501`) — Los límites biológicos (*The biological limits*) `[TIER-D]`

- **Catalog Metadata:** `scene=1` · `[0.040, 0.080]` · `rgb=[215, 226, 208]` · `sound=(sine/73.95Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *SUPERAR UN LÍMITE CREA NUEVAS DEPENDENCIAS*
- **Metric & Evidence (ES / EN):** `Envejecimiento + enfermedad + fragilidad + entorno estrecho → presión para modificar la vida` / *`Aging + illness + frailty + narrow environment → pressure to modify life`* · Evidence: `Herencia evolutiva · cuerpos finitos adaptados a una franja estrecha del universo` (*Evolutionary inheritance · finite bodies adapted to a narrow strip of the universe*)
- **Spoken Macro Narrative (ES / EN):** Superar una limitación biológica puede resolver un problema mientras crea nuevas dependencias y nuevas formas de vulnerabilidad. (*EN:* Overcoming a biological limitation can solve a problem while creating new dependencies and new forms of vulnerability.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` identical slotted spheres + 25-petal lotus (`horizon_fade=0.974`, `slow_rate=0.967`); zero cellular repair helices, artificial organs, exoskeletons, neural-memory implants, extremophile shells, or cryonic stasis chambers. **Target (`vid=3501`):** Stage an organic double-helix / soft luminous cellular vessel flanked by bio-engineered augmentation scaffolding.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **envejecimiento** (*aging*) | `[TIER-D]` | Reparar daños celulares podría prolongar la vida, pero una duración mayor no elimina accidentes, conflictos ni pérdida de sentido. / *Repairing cellular damage could prolong life, but a longer lifespan does not eliminate accidents, conflicts, or loss of meaning.* | **Target:** glowing DNA/cellular lattice patched by golden repair nanobots *(Gap: F0 misses cellular damage/telomere repair lattice & accident fracture contrast.)* |
| `F1` | **enfermedad** (*disease*) | `[TIER-D]` | La ingeniería genética y los órganos artificiales pueden reducir enfermedades mientras introducen fallos técnicos y desigualdad de acceso. / *Genetic engineering and artificial organs can reduce disease while introducing technical glitches and inequality of access.* | **Target:** glowing edited helical bands beside geometric mechanical heart/organ pump with supply conduit *(Gap: F1 misses CRISPR-like DNA double-helix repair splines & mechanical organ scaffolding.)* |
| `F2` | **cuerpo frágil** (*fragile body*) | `[TIER-D]` | Exoesqueletos, prótesis y rediseño corporal amplían capacidades, aunque también vuelven la vida dependiente de mantenimiento e infraestructura. / *Exoskeletons, prostheses and body redesign expand capabilities, although they also make life dependent on maintenance and infrastructure.* | **Target:** faceted golden exoskeleton plates cradling soft cyan core connected to glowing maintenance tethers *(Gap: F2 misses rigid geometric exoskeleton ribs armoring fragile organic core + umbilical tether.)* |
| `F3` | **memoria limitada** (*limited memory*) | `[TIER-D]` | Interfaces neurales pueden externalizar recuerdos, pero almacenar información no garantiza integrarla en una identidad coherente. / *Neural interfaces can externalize memories, but storing information does not guarantee integrating it into a coherent identity.* | **Target:** neural orb beaming fiber-optic filaments into hovering diamond storage monoliths *(Gap: F3 misses externalized silicon memory crystal array docked beside warm neural node.)* |
| `F4` | **entorno estrecho** (*narrow environment*) | `[TIER-D]` | Organismos sintéticos podrían vivir en frío, vacío o radiación, a costa de alejarse de las condiciones que formaron su experiencia. / *Synthetic organisms could live in cold, vacuum or radiation, at the cost of distancing themselves from the conditions that formed their experience.* | **Target:** ice/vacuum-armored crystal extremophile enduring harsh purple cosmic radiation spikes *(Gap: F4 misses radiation-hardened crystalline synthetic organism floating in vacuum frost.)* |
| `F5` | **muerte** (*death*) | `[TIER-D]` | Animación suspendida o reparación avanzada pueden aplazar la muerte sin demostrar que toda vida pueda sostenerse indefinidamente. / *Suspended animation or advanced repair can postpone death without proving that all life can be sustained indefinitely.* | **Target:** suspended-stasis cryogenic cylinder freezing internal ember pulses *(Gap: F5 misses suspended-animation icy stasis capsule slowing metabolism while remaining mortal.)* |

##### `Z02` `mind-transformation` (`vid=3502`) — La transformación de la mente (*The transformation of the mind*) `[TIER-D]`

- **Catalog Metadata:** `scene=2` · `[0.080, 0.120]` · `rgb=[168, 226, 198]` · `sound=(triangle/71.9Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *REDISEÑAR LA MENTE CAMBIA TAMBIÉN SUS VALORES*
- **Metric & Evidence (ES / EN):** `Módulos artificiales + plasticidad + interfaces → nuevas arquitecturas de experiencia y decisión` / *`Artificial modules + plasticity + interfaces → new architectures of experience and decision`* · Evidence: `Cognición rediseñable · memoria, atención, emoción y velocidad dejan de ser constantes` (*Redesignable cognition · memory, attention, emotion and speed are no longer constant*)
- **Spoken Macro Narrative (ES / EN):** Modificar la mente no solo cambia lo que puede hacer; puede cambiar qué desea, qué recuerda y qué considera valioso. (*EN:* Modifying the mind doesn't just change what it can do; You can change what you want, what you remember, and what you consider valuable.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` generic slotted spheres + 25-petal lotus (`horizon_fade=0.947`, `slow_rate=0.933`); zero neural architecture, memory compression, accelerated inner clock, shared network bus, or splitting mind threads. **Target (`vid=3502`):** Sculpt a glowing multilayer neural torus manifold (`sd_torus` interwoven axons + cortical hub) modulated per `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **memoria expandida** (*expanded memory*) | `[TIER-D]` | Una mente puede recordar con enorme detalle y aun así necesitar olvidar, resumir y reinterpretar para pensar con claridad. / *A mind can remember in enormous detail and still need to forget, summarize, and reinterpret to think clearly.* | **Target:** swirling archive snow compressing cleanly into crystal summary knots *(Gap: F0 misses dense chaotic memory shards folding/summarizing into crisp golden index torus.)* |
| `F1` | **pensamiento acelerado** (*accelerated thinking*) | `[TIER-D]` | Procesar más rápido aumenta la vida subjetiva dentro del mismo tiempo externo, pero puede separar a la mente de ritmos sociales y físicos. / *Processing faster increases subjective life within the same external time, but can separate the mind from social and physical rhythms.* | **Target:** blistering fast inner compute spiral racing beside near-static outside stars *(Gap: F1 misses high-frequency inner strobe clock outracing frozen external world ring.)* |
| `F2` | **sentidos compartidos** (*shared senses*) | `[TIER-D]` | Varias mentes podrían percibir mediante una red común sin experimentar exactamente la misma perspectiva. / *Multiple minds could perceive through a common network without experiencing exactly the same perspective.* | **Target:** three distinct colored orbs wired into shared luminous central sensory prism *(Gap: F2 misses multiple distinct mind orbs plugged into a common glowing sensory ring-bus.)* |
| `F3` | **emociones alteradas** (*altered emotions*) | `[TIER-D]` | Regular miedo, deseo o apego puede reducir sufrimiento y también transformar motivaciones que sostenían vínculos e identidad. / *Regulating fear, desire or attachment can reduce suffering and also transform motivations that sustained ties and identity.* | **Target:** jagged crimson-gold emotional spikes smoothly damped into flat cool teal *(Gap: F3 misses dampening/flattening turbulent fear/desire waves while relational bond cords pale.)* |
| `F4` | **atención controlada** (*controlled attention*) | `[TIER-D]` | Una atención programable mejora concentración, aunque quien controla sus prioridades puede influir profundamente en su autonomía. / *Programmable attention improves concentration, although who controls your priorities can profoundly influence your autonomy.* | **Target:** razor-sharp glowing laser spotlight cone locked onto outer controller ring *(Gap: F4 misses crisp sweeping spotlight aperture steering focus under external control lock.)* |
| `F5` | **conciencia paralela** (*parallel consciousness*) | `[TIER-D]` | Procesos simultáneos podrían cooperar, competir o divergir hasta que ya no resulte claro si constituyen una sola mente. / *Simultaneous processes may cooperate, compete, or diverge until it is no longer clear whether they constitute a single mind.* | **Target:** unified cortical torus unbraiding into three parallel independent glowing rings *(Gap: F5 misses single neural trunk splitting into simultaneously computing competing sub-threads.)* |

##### `Z03` `identity-continuity` (`vid=3503`) — Identidad y continuidad (*Identity and continuity*) `[TIER-D]`

- **Catalog Metadata:** `scene=3` · `[0.120, 0.160]` · `rgb=[130, 222, 214]` · `sound=(sawtooth/69.85Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *COPIAR UN PATRÓN NO PRUEBA CONTINUIDAD SUBJETIVA*
- **Metric & Evidence (ES / EN):** `Cuerpo + memoria + patrón + continuidad causal → criterios distintos de seguir siendo alguien` / *`Body + memory + pattern + causal continuity → different criteria for continuing to be someone`* · Evidence: `Problema abierto · conservar funciones no demuestra conservar la misma perspectiva subjetiva` (*Open problem · preserving functions does not prove to preserve the same subjective perspective*)
- **Spoken Macro Narrative (ES / EN):** Una copia perfecta puede creer que es el original sin demostrar que la perspectiva original continuó en ella. (*EN:* A perfect copy can believe that it is the original without demonstrating that the original perspective continued in it.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` slotted spheres + lotus (`horizon_fade=0.921`, `slow_rate=0.90`); zero Ship-of-Theseus neuron swap, copy split, destructive scan dissolution, or merged dual helix (`kw:neurona`, `kw:fusion` ignored by GPU!). **Target (`vid=3503`):** Animate clear topological state-machine transforms on a sculpted glowing neural orb (`sd_sphere` + axonal filaments) across `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **reemplazo gradual** (*gradual replacement*) | `[TIER-D]` | Sustituir neuronas una a una preservaría cambios continuos, pero no resuelve por consenso qué propiedad sostiene la identidad subjetiva. / *Replacing neurons one by one would preserve continuous changes, but it does not resolve by consensus which property sustains subjective identity.* | **Target:** organic teal cells popping cleanly into gold synthetic voxels sequentially around meridian (`0%->100%`) *(Gap: F0 misses biological sphere slowly swapping cyan cells 1-by-1 for gold silicon cells (`kw:neurona`).)* |
| `F1` | **copia digital** (*digital copy*) | `[TIER-D]` | Una copia puede compartir todos los recuerdos hasta el momento de creación y convertirse inmediatamente en una persona divergente. / *A copy can share all memories up to the moment of creation and immediately become a divergent person.* | **Target:** side-by-side live copy clone splitting orbit smoothly *(Gap: F1 misses intact original sphere spawning twin crystalline sibling drifting cleanly apart.)* |
| `F2` | **carga destructiva** (*destructive upload*) | `[TIER-D]` | Destruir el cerebro al copiarlo evita dos versiones corporales, pero no demuestra que la experiencia haya sido transferida en vez de terminada. / *Destroying the brain by copying it avoids two bodily versions, but does not prove that the experience has been transferred rather than finished.* | **Target:** destructive scan beam turning left organic orb to dark dust while activating right digital lattice *(Gap: F2 misses slicing laser vaporizing biological brain-orb while lighting distant digital twin.)* |
| `F3` | **copias simultáneas** (*simultaneous copies*) | `[TIER-D]` | Varias continuaciones igualmente fieles vuelven insuficiente la idea de un único futuro personal. / *Several equally faithful continuations render the idea of a single personal future insufficient.* | **Target:** crisp Y/trident worldline bifurcation tree *(Gap: F3 misses 1-to-3 worldline trifurcation cone questioning 'which branch is me?')* |
| `F4` | **interrupción y restauración** (*interruption and restoration*) | `[TIER-D]` | Una mente detenida y reiniciada puede conservar patrón y memoria mientras deja abierta la cuestión de continuidad subjetiva. / *A stopped and restarted mind can preserve pattern and memory while leaving open the question of subjective continuity.* | **Target:** dark zero-density frame interruption gap bridging two identical crystalline reboots *(Gap: F4 misses state-snapshot save crystal flickering completely off then rebooting identically.)* |
| `F5` | **memorias fusionadas** (*merged memories*) | `[TIER-D]` | Combinar historias personales puede producir una identidad nueva que hereda a varias anteriores sin ser idéntica a ninguna. / *Combining personal stories can produce a new identity that inherits several previous ones without being identical to any of them.* | **Target:** crimson + cyan twin spirals fusing cleanly into luminous violet composite torus *(Gap: F5 misses twin diverging colored memory helices braiding into one novel third hue (`kw:fusion`).)* |

##### `Z04` `postbiological-life` (`vid=3504`) — Vida postbiológica (*Postbiological life*) `[TIER-D]`

- **Catalog Metadata:** `scene=4` · `[0.160, 0.200]` · `rgb=[130, 196, 234]` · `sound=(triangle/67.8Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *DURABILIDAD NO EQUIVALE AUTOMÁTICAMENTE A PROGRESO*
- **Metric & Evidence (ES / EN):** `Mentes máquina + ecosistemas sintéticos + autorreparación → existencia en entornos antes inhabitables` / *`Machine minds + synthetic ecosystems + self-repair → existence in previously uninhabitable environments`* · Evidence: `Transición posible · la continuidad de la vida abandona algunos rasgos de su origen orgánico` (*Possible transition · the continuity of life abandons some features of its organic origin*)
- **Spoken Macro Narrative (ES / EN):** La postbiología puede aumentar duración y adaptabilidad sin constituir automáticamente progreso moral, continuidad personal o una experiencia mejor. (*EN:* Postbiology can increase durability and adaptability without automatically constituting moral progress, personal continuity, or a better experience.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` slotted spheres + lotus (`horizon_fade=0.895`, `slow_rate=0.867`); zero self-healing machine core, synthetic biodome, distributed node cloud, or long-endurance interstellar starship probe. **Target (`vid=3504`):** Build a gleaming metallic-crystal polyhedral probe & closed biosphere torus.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **mentes máquina** (*machine minds*) | `[TIER-D]` | Un sustrato artificial podría repararse y copiarse, aunque la posibilidad de conciencia en él continúa siendo una cuestión no demostrada. / *An artificial substrate could be repaired and copied, although the possibility of consciousness on it remains an unproven question.* | **Target:** self-healing octahedral crystal regrowing chipped faces around ambiguous faint spark *(Gap: F0 misses radiant metallic crystal core questioning inner phenomenal spark inside cold lattice.)* |
| `F1` | **ecosistemas sintéticos** (*synthetic ecosystems*) | `[TIER-D]` | Sistemas artificiales pueden reciclar materia y energía sin reproducir la diversidad, historia y vulnerabilidad de una biosfera natural. / *Artificial systems can recycle matter and energy without reproducing the diversity, history and vulnerability of a natural biosphere.* | **Target:** split globe showing stark industrial recycling piping left vs lush fractal garden right *(Gap: F1 misses artificial geometric biodome pipes vs wild rich fractal coral biosphere.)* |
| `F2` | **computación distribuida** (*distributed computing*) | `[TIER-D]` | Una mente extendida entre muchos nodos resiste fallos locales, pero depende de sincronización, seguridad y continuidad de la red. / *A mind spread across many nodes resists local failures, but depends on synchronization, security, and network continuity.* | **Target:** 12-node compute icosahedron re-routing live pulses past extinguished vertex *(Gap: F2 misses fault-tolerant peer-to-peer mesh routing smoothly around one dead black node.)* |
| `F3` | **naves autorreparables** (*self-healing ships*) | `[TIER-D]` | Máquinas capaces de mantenerse durante milenios podrían explorar lejos de sus creadores y evolucionar hacia objetivos propios. / *Machines capable of sustaining themselves for millennia could explore far from their creators and evolve towards their own goals.* | **Target:** slender autonomous probe hull healing micrometeorite scars with molten gold arcs *(Gap: F3 misses sleek metallic von-Neumann/self-healing probe repairing hull plating over millennia.)* |
| `F4` | **formas híbridas** (*hybrid forms*) | `[TIER-D]` | Componentes biológicos y digitales pueden coexistir sin que exista una frontera única entre organismo, herramienta y entorno. / *Biological and digital components can coexist without there being a single boundary between organism, tool and environment.* | **Target:** continuous cyborg gradient weaving glowing chlorophyll helix into copper waveguide lattice *(Gap: F4 misses interwoven wet organic tendrils and crisp geometric circuit fins blurring boundary.)* |
| `F5` | **pérdidas posibles** (*possible losses*) | `[TIER-C]` | Durabilidad y velocidad pueden obtenerse a cambio de corporalidad, diversidad biológica y continuidad con la vida ancestral. / *Durability and speed can be obtained in exchange for corporality, biological diversity and continuity with ancestral life.* | **Target:** bright geometric crystal hull casting long fading fossil shadow *(Gap: F5 misses pale organic fossil silhouette fading behind sleek gleaming postbiological hull.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP25-Z00-04)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP25-Z00-04` (Open Future Prelude, Evolved Limits, Mind Redesign, Identity Continuity & Postbiology) for HyperCosmosVR Odyssey Act V (`Exp 25: 25_final_horizon`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z00`->`3500` (`F0..F5`), `Z01`->`3501` (`F0..F5`), `Z02`->`3502` (`F0..F5`), `Z03`->`3503` (`F0..F5`), `Z04`->`3504` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3860`, `3861`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP25-Z00-04`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z00: incomplete-history / A single incomplete story, Z01: biological-limits / The biological limits, Z02: mind-transformation / The transformation of the mind, Z03: identity-continuity / Identity and continuity, Z04: postbiological-life / Postbiological life.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp25-z05-09"></a>
### PKG-ACT5-EXP25-Z05-09: Deep-Time Minds, Galactic Diasporas, Dyson/Shkadov Engineering & Constructed Worlds

- **Phase:** Phase 1 + Phase 4 | **Priority:** `P0` | **Scope:** Exp `25` (`25_final_horizon`) `Z05..Z09` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L88`: `25 => 250` (single variant across all 25 zones!); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L495-L518` (3 nested longitude-ribbed wireframe spheres `r=0.24,0.44,0.64` + 25-petal HSL lotus ring + `horizon_fade`/`slow_rate` across `z=0..24`; `feat` shifts sphere phase `0.3*feat` at `L504`). Tier breakdown for this batch: **`0A / 0B / 0C / 5D` Zones**, **`0A / 0B / 2C / 28D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `1/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3505..=3509` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3862`, `3863` (allocated from `3720 + (25 - 11)*10 + k`; Megastructure & Constructed-World Thermal Shells (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z05` | `long-duration-intelligences` (*Inteligencias de larga duración* / *Long-lasting intelligences*) | `[TIER-D]` | `vid=250` | **`3505`** | `3862` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z06` | `galactic-civilizations` (*Civilizaciones galácticas* / *Galactic civilizations*) | `[TIER-D]` | `vid=250` | **`3506`** | `3863` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z07` | `cosmic-engineering` (*Ingeniería cósmica* / *cosmic engineering*) | `[TIER-D]` | `vid=250` | **`3507`** | `3862` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x07` (0x04 semantic cue mask, 0x01 cutaway, 0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z08` | `constructed-reality` (*Computación y realidad construida* / *Computing and constructed reality*) | `[TIER-D]` | `vid=250` | **`3508`** | `3863` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z09` | `plural-worlds` (*La multiplicación de mundos* / *The multiplication of worlds*) | `[TIER-D]` | `vid=250` | **`3509`** | `3862` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z05` `long-duration-intelligences` (`vid=3505`) — Inteligencias de larga duración (*Long-lasting intelligences*) `[TIER-D]`

- **Catalog Metadata:** `scene=5` · `[0.200, 0.240]` · `rgb=[142, 176, 242]` · `sound=(sine/65.75Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *UNA VIDA PUEDE DESVINCULARSE DEL RITMO BIOLÓGICO*
- **Metric & Evidence (ES / EN):** `Velocidad cognitiva × duración externa × pausas → tiempos subjetivos radicalmente distintos` / *`Cognitive speed × external duration × pauses → radically different subjective times`* · Evidence: `Escalas temporales nuevas · una vida ya no sigue necesariamente el ritmo de un organismo` (*New time scales · a life no longer necessarily follows the rhythm of an organism*)
- **Spoken Macro Narrative (ES / EN):** El significado cambia cuando una mente puede vivir milenios subjetivos en un año o esperar siglos entre dos pensamientos. (*EN:* The meaning changes when a mind can live subjective millennia in a year or wait centuries between two thoughts.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` slowly turns all 3 spheres monotonically via global `slow_rate=0.833`; fails to show contrasting simultaneous ultra-fast vs tectonic/stellar clocks or intermittent burst hibernation. **Target (`vid=3505`):** Construct nested multi-rate relativistic chronometer rings spanning milliseconds to gigayears.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **mente acelerada** (*racing mind*) | `[TIER-D]` | Una mente rápida puede experimentar generaciones internas antes de que cambie apreciablemente su entorno físico. / *A fast mind can undergo internal generations before its physical environment changes appreciably.* | **Target:** white-hot inner millisecond spark ring whirring furiously against static crimson backdrop *(Gap: F0 misses flickering hyper-dense micro-spark civilization orbiting beside frozen red star.)* |
| `F1` | **vida secular** (*centuries-long life*) | `[TIER-C]` | Una persona de siglos afrontaría memoria acumulada, transformación cultural y relaciones con seres de ritmos muy diferentes. / *A person of centuries would face accumulated memory, cultural transformation and relationships with beings of very different rhythms.* | **Target:** long luminous gold ribbon threading dozens of rapid twinkling sparks *(Gap: F1 misses multi-century continuous thread spanning flickering brief ambient arcs.)* |
| `F2` | **mente geológica** (*geological mind*) | `[TIER-D]` | Una inteligencia lenta podría percibir montañas, océanos y civilizaciones como procesos breves dentro de una sola experiencia. / *A slow intelligence might perceive mountains, oceans, and civilizations as brief processes within a single experience.* | **Target:** volumetric undulating tectonic ridge waves rising swiftly when inner slow clock blinks once *(Gap: F2 misses slow uplifting mountain fold morphing visibly within one giant glowing eye blink.)* |
| `F3` | **vida estelar** (*stellar life*) | `[TIER-D]` | Proyectos ligados a la evolución de estrellas requieren instituciones o mentes capaces de conservar objetivos durante eras. / *Projects linked to the evolution of stars require institutions or minds capable of maintaining objectives for ages.* | **Target:** stellar evolution ring watching yellow star swell into red giant & white dwarf *(Gap: F3 misses grand keeper halo stewarding swelling stellar evolution cycles.)* |
| `F4` | **hibernación** (*hibernation*) | `[TIER-D]` | Dormir entre eventos raros reduce gasto energético, pero convierte la continuidad en una secuencia de intervalos enormes. / *Sleeping between rare events reduces energy expenditure, but turns continuity into a sequence of enormous intervals.* | **Target:** narrow beacon flare separated by long cold zero-power void arcs *(Gap: F4 misses sharp crisp burst flash separated by vast pitch-black dormant time gaps.)* |
| `F5` | **desfase temporal** (*time lag*) | `[TIER-C]` | Seres con velocidades incompatibles pueden compartir causalidad sin compartir conversación, urgencia ni una escala común de valor. / *Beings with incompatible speeds can share causality without sharing conversation, urgency, or a common scale of value.* | **Target:** fast violet pulse passing clean through slow amber aperture without locking phase *(Gap: F5 misses side-by-side desynchronized pulse frequencies visibly unable to handshake.)* |

##### `Z06` `galactic-civilizations` (`vid=3506`) — Civilizaciones galácticas (*Galactic civilizations*) `[TIER-D]`

- **Catalog Metadata:** `scene=6` · `[0.240, 0.280]` · `rgb=[155, 160, 244]` · `sound=(triangle/63.7Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *UNA GALAXIA CONECTADA NO PUEDE SER SINCRÓNICA*
- **Metric & Evidence (ES / EN):** `Asentamientos + sondas + retraso de la luz + evolución local → familia de civilizaciones divergentes` / *`Settlements + probes + light delay + local evolution → family of divergent civilizations`* · Evidence: `Expansión fragmentada · parentesco sin sincronización ni centro único` (*Fragmented expansion · kinship without synchronization or single center*)
- **Spoken Macro Narrative (ES / EN):** Una civilización galáctica sería probablemente una red de descendientes parcialmente independientes, no un imperio sincronizado. (*EN:* A galactic civilization would probably be a network of partially independent descendants, not a synchronized empire.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` spherical wire cages + flat rainbow 25-petal lotus (`horizon_fade=0.842`, `slow_rate=0.80`); zero spiral-galaxy scale structure, expanding probe tree, light-lagged communication arcs, or decentralized cluster hues. **Target (`vid=3506`):** Render a tilted miniature logarithmic spiral galaxy disk (`compute` scatter / 3D spiral density ribs + star hubs) navigated per `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **asentamientos** (*settlements*) | `[TIER-D]` | Cada sistema estelar desarrolla soluciones locales porque la ayuda, la autoridad y la información tardan años o milenios en llegar. / *Each star system develops local solutions because help, authority and information take years or millennia to arrive.* | **Target:** isolated bright gold local bubble pockets across spiral arms *(Gap: F0 misses glowing island star settlements thriving independently across black galactic arms.)* |
| `F1` | **sondas autónomas** (*autonomous probes*) | `[TIER-D]` | Exploradores autorreplicantes pueden cartografiar y construir infraestructura, pero sus objetivos deben sobrevivir a errores y evolución. / *Self-replicating explorers can map and build infrastructure, but their goals must remain stable despite errors and evolution.* | **Target:** branching glowing probe trajectories leaping star-to-star radially outward *(Gap: F1 misses branching von-Neumann probe wave radiating outward with slight mutation drift hues.)* |
| `F2` | **culturas descendientes** (*descendant cultures*) | `[TIER-D]` | Lenguas, cuerpos, valores y tecnologías divergen cuando poblaciones permanecen separadas durante muchas generaciones. / *Languages, bodies, values and technologies diverge when populations remain separated for many generations.* | **Target:** color spectrum continuously mutating from golden core outward into violet/cyan rim colonies *(Gap: F2 misses cladistic hue divergence shifting distant daughter systems far from golden Earth origin.)* |
| `F3` | **mensajes lentos** (*slow messages*) | `[TIER-D]` | Una decisión enviada a través de la galaxia puede llegar cuando quienes la originaron ya no existen o han cambiado por completo. / *A decision sent across the galaxy may arrive when those who originated it no longer exist or have changed completely.* | **Target:** slow creeping light arc arriving at an empty/changed supernova node *(Gap: F3 misses faint photon wave arriving millennia late at an already transformed/dark star node.)* |
| `F4` | **archivos galácticos** (*galactic archives*) | `[TIER-D]` | Memorias redundantes preservan una historia común sin impedir que cada región interprete esa historia de forma distinta. / *Redundant memories preserve a common history without preventing each region from interpreting that history differently.* | **Target:** identical triangular archive beacons pulsing symmetrically in 4 distant quadrants *(Gap: F4 misses replicated diamond archival beacons anchored at multiple galactic rim clusters.)* |
| `F5` | **sin centro único** (*no single center*) | `[TIER-D]` | La coordinación puede basarse en protocolos y parentesco histórico, pero ninguna capital puede gobernar en tiempo real toda la galaxia. / *Coordination may be based on protocols and historical kinship, but no capital can govern the entire galaxy in real time.* | **Target:** decentralized peer web with empty galactic center void *(Gap: F5 misses peer-to-peer multi-hub mesh conspicuously lacking any central imperial throne hub.)* |

##### `Z07` `cosmic-engineering` (`vid=3507`) — Ingeniería cósmica (*cosmic engineering*) `[TIER-D]`

- **Catalog Metadata:** `scene=7` · `[0.280, 0.320]` · `rgb=[180, 158, 236]` · `sound=(sawtooth/61.65Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `dyson` · Footer: *REORGANIZAR MATERIA NO ABOLIR LAS LEYES FÍSICAS*
- **Metric & Evidence (ES / EN):** `Tiempo profundo + automatización + energía estelar → proyectos mayores que planetas` / *`Deep time + automation + stellar energy → projects bigger than planets`* · Evidence: `Capacidad extrema · reorganizar materia y energía sin abolir las leyes físicas` (*Extreme ability · reorganize matter and energy without abolishing physical laws*)
- **Spoken Macro Narrative (ES / EN):** Una tecnología suficientemente avanzada puede transformar estrellas y hábitats, pero no convierte todas las ideas imaginables en posibilidades físicas. (*EN:* Sufficiently advanced technology can transform stars and habitats, but it doesn't turn every imaginable idea into physical possibilities.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` generic 3 wire cages + rainbow lotus (`horizon_fade=0.816`, `slow_rate=0.767`); ZERO Dyson swarm shells, rotating O'Neill cylinders, stellar lifting plumes, spinning Kerr black hole accretion, or star thruster! (`F0` hits `kw:dyson`, `kw:orbital` in resolver; ignored by GPU!) **Target (`vid=3507`):** Build a dedicated Megastructures Armory showcase (`vid=3507`, `kw:dyson` override) switching crisp 3D engineering SDFs cleanly on `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **enjambres Dyson** (*Dyson swarms*) `[kw:dyson]` | `[TIER-D]` | Millones de colectores orbitales pueden capturar gran parte de la luz de una estrella sin exigir una esfera sólida imposible. / *Millions of orbital collectors can capture much of a star's light without requiring an impossible solid sphere.* | **Target:** radiant golden host star surrounded by hundreds of tilted faceted black-gold Dyson swarm collector arcs (`fract` instanced orbital shells + thermal IR glow) *(Gap: F0 Dyson swarms (`kw:dyson`) completely missing—no nested orbiting photovoltaic swarm collectors dimming warm starlight.)* |
| `F1` | **extracción estelar** (*stellar extraction*) | `[TIER-D]` | Retirar materia de una estrella modifica su vida útil y proporciona combustible, aunque requiere control durante escalas enormes. / *Removing matter from a star changes its lifetime and provides fuel, although it requires control over enormous scales.* | **Target:** star orb with twin glowing magnetic lifting plasma siphons feeding an orbital collector torus *(Gap: F1 stellar extraction completely missing—no magnetic siphon arc lifting glowing plasma from solar equator.)* |
| `F2` | **hábitats artificiales** (*artificial habitats*) | `[TIER-D]` | Mundos rotatorios pueden ofrecer superficies habitables sin depender de planetas naturales. / *Rotating worlds can offer habitable surfaces without relying on natural planets.* | **Target:** spinning O'Neill/Stanford double-counter-rotating cylinder + torus habitat (`sd_capped_cylinder`/`sd_torus`) with warm sunlit interior bands *(Gap: F2 artificial habitats completely missing—no spinning glowing cylinder/torus rings with interior horizon band.)* |
| `F3` | **estabilización planetaria** (*planetary stabilization*) | `[TIER-D]` | Controlar impactos, clima y órbita reduce riesgos mientras exige mantenimiento y decisiones sobre qué ecosistemas preservar. / *Controlling impacts, climate and orbit reduces risks while requiring maintenance and decisions about which ecosystems to preserve.* | **Target:** blue globe orbited by L1 sunshield ring & gravity-tractor tug shifting trajectory *(Gap: F3 planetary stabilization completely missing—no orbital sun-shade ring & asteroid tractor arc.)* |
| `F4` | **energía de agujeros negros** (*black hole energy*) | `[TIER-D]` | Acreción y rotación permiten extraer energía, pero acercan infraestructura a entornos extremos y no ilimitados. / *Accretion and rotation allow energy to be extracted, but they bring infrastructure closer to extreme and not unlimited environments.* | **Target:** pitch-black Kerr photon-ring shadow surrounded by blazing beamed Doppler-shifted accretion disk + Penrose extraction spires *(Gap: F4 black hole energy completely missing—no pitch-black event horizon orb with relativistic Kerr accretion disk & polar extraction jets.)* |
| `F5` | **movimiento de estrellas** (*star movement*) | `[TIER-D]` | Impulsos minúsculos sostenidos durante millones de años pueden alterar trayectorias estelares sin permitir movimientos instantáneos. / *Tiny impulses sustained for millions of years can alter stellar trajectories without allowing instantaneous movements.* | **Target:** blazing star cradled inside colossal concave Shkadov reflector mirror + twin luminous thermonuclear steering jets *(Gap: F5 star movement completely missing—no asymmetric Shkadov parabolic mirror / twin jet thruster steering an entire glowing star.)* |

##### `Z08` `constructed-reality` (`vid=3508`) — Computación y realidad construida (*Computing and constructed reality*) `[TIER-D]`

- **Catalog Metadata:** `scene=8` · `[0.320, 0.360]` · `rgb=[196, 145, 226]` · `sound=(triangle/59.6Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *EL MUNDO VIVIDO Y EL SUSTRATO SON CAPAS DISTINTAS*
- **Metric & Evidence (ES / EN):** `Materia computacional + modelos + habitantes posibles → historias y entornos artificiales` / *`Computational matter + models + possible inhabitants → stories and artificial environments`* · Evidence: `Dos capas · un sustrato físico sostiene mundos cuya experiencia puede obedecer otras reglas` (*Two layers · a physical substrate supports worlds whose experience can obey other rules*)
- **Spoken Macro Narrative (ES / EN):** Una experiencia dentro de una realidad construida no sería menos real para quien la vive, aunque la conciencia simulada no está demostrada. (*EN:* An experience within a constructed reality would be no less real for those who experience it, although simulated consciousness is not demonstrated.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` slotted spheres + lotus (`horizon_fade=0.789`, `slow_rate=0.733`); zero computronium radiator shells, dual hardware-vs-virtual world stack, clock-dilation visualizer, or thermodynamic Matrioshka shell stack. **Target (`vid=3508`):** Construct a split-hemisphere 'Substrate vs Simulation' crystal world.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **computación cósmica** (*cosmic computing*) | `[TIER-D]` | Planetas, estrellas o agujeros negros podrían formar parte de infraestructuras de cálculo limitadas por energía, calor y causalidad. / *Planets, stars or black holes could be part of computing infrastructures limited by energy, heat and causality.* | **Target:** dense geometric radiator monolith radiating red IR heat waste lines outward from cold star core *(Gap: F0 misses glowing Matrioshka-shell stellar/planetary computer radiating waste-heat IR filaments outward.)* |
| `F1` | **mundos simulados** (*simulated worlds*) | `[TIER-D]` | Un entorno digital puede contener geografía, historia y leyes efectivas distintas de las del hardware que lo sostiene. / *A digital environment may contain geography, history, and effective laws other than those of the hardware that supports it.* | **Target:** sharp dividing glass equator showing dark wireframe server matrix below and vibrant golden sky dome above *(Gap: F1 misses stark contrast between dark geometric silicon substrate plane below & vibrant mountain-sky mirage dome above.)* |
| `F2` | **física artificial** (*artificial physics*) | `[TIER-D]` | Las reglas internas pueden diseñarse, pero toda simulación continúa limitada por los recursos y errores de su sustrato. / *Internal rules can be designed, but any simulation remains limited by the resources and errors of its substrate.* | **Target:** shimmering non-Euclidean geodesic terrain grid flickering occasionally into hardware quantization voxels *(Gap: F2 misses custom low-gravity/warped rule grid straining visibly against hardware precision artifacts.)* |
| `F3` | **historias aceleradas** (*accelerated stories*) | `[TIER-D]` | Procesos internos podrían ejecutarse más rápido o más lento que el mundo exterior, alterando duración y responsabilidad. / *Internal processes could run faster or slower than the outside world, altering duration and accountability.* | **Target:** roaring bright fast-spinning simulated orb nestled inside sluggish outer physical tick-ring *(Gap: F3 misses outer red physical second tick enclosing 100 fast gold virtual sunrises inside inner sphere.)* |
| `F4` | **habitantes conscientes** (*conscious inhabitants*) | `[TIER-D]` | Crear posibles sujetos digitales introduciría obligaciones éticas incluso si sus cuerpos y cielos fueran construidos. / *Creating possible digital subjects would introduce ethical obligations even if their bodies and skies were constructed.* | **Target:** delicate warm golden shield enveloping tiny conscious avatar orbs inside digital landscape *(Gap: F4 misses tender moral focus beacon illuminating tiny fragile avatar sparks thriving inside virtual dome.)* |
| `F5` | **simulaciones anidadas** (*nested simulations*) | `[TIER-D]` | Cada capa adicional consume recursos de otra y no ofrece una vía demostrada para escapar de la física del nivel sustentador. / *Each additional layer consumes resources from another and offers no proven way to escape the physics of the supporting layer.* | **Target:** three nested decreasing bright wireframe cubes tethered to one thick finite physical power cable *(Gap: F5 misses cascading diminishing nested Matrioshka frames visibly bottlenecked by outer hardware power rail.)* |

##### `Z09` `plural-worlds` (`vid=3509`) — La multiplicación de mundos (*The multiplication of worlds*) `[TIER-D]`

- **Catalog Metadata:** `scene=9` · `[0.360, 0.400]` · `rgb=[211, 143, 222]` · `sound=(sine/57.55Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *EL CONCEPTO DE MUNDO SE VUELVE PLURAL*
- **Metric & Evidence (ES / EN):** `Múltiples sustratos + sentidos + reglas locales → muchas formas simultáneas de habitar` / *`Multiple substrates + senses + local rules → many simultaneous ways of inhabiting`* · Evidence: `Civilización plural · planeta, hábitat y entorno virtual dejan de compartir una sola realidad cotidiana` (*Plural civilization · planet, habitat and virtual environment stop sharing a single daily reality*)
- **Spoken Macro Narrative (ES / EN):** Cuando “el mundo” se vuelve plural, mantener una civilización común exige traducir experiencias que pueden no tener equivalentes directos. (*EN:* When “the world” becomes plural, maintaining a common civilization requires translating experiences that may have no direct equivalents.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` 3 wire spheres + lotus (`horizon_fade=0.763`, `slow_rate=0.70`); zero contrasting planetary/orbital/virtual/sensory/historical/mathematical habitat spheres (`kw:orbital` ignored). **Target (`vid=3509`):** Display an archipelago of 6 radically distinct world-spheres arranged cleanly in a 3D constellation spotlighted individually per `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **planetas físicos** (*physical planets*) | `[TIER-D]` | Gravedad, clima y ecología continúan organizando vidas corporales dentro de límites materiales locales. / *Gravity, climate, and ecology continue to organize bodily lives within local material boundaries.* | **Target:** rocky ocean/continent gravity-well orb *(Gap: F0 misses classic rocky biosphere planet locked by gravity & swirling cloud bands.)* |
| `F1` | **hábitats orbitales** (*orbital habitats*) | `[TIER-D]` | Entornos construidos permiten elegir gravedad, ciclo temporal y paisaje, pero dependen de infraestructura activa. / *Built environments allow you to choose gravity, time cycle and landscape, but depend on active infrastructure.* | **Target:** sleek spinning Stanford torus orbital colony lit by central mirror *(Gap: F1 misses engineered zero-G orbital torus ring spinning synthetic day/night (`kw:orbital` ignored).)* |
| `F2` | **espacios virtuales** (*virtual spaces*) | `[TIER-D]` | La identidad puede moverse entre escenarios sin que desaparezcan los costes físicos del cálculo. / *The identity can move between scenarios without the physical costs of the calculation disappearing.* | **Target:** morphing iridescent voxel/holographic avatar leaping between dissolving floating arenas *(Gap: F2 misses translucent shifting avatar shifting fluidly between wireframe polyhedral arenas.)* |
| `F3` | **mundos sensoriales** (*sensory worlds*) | `[TIER-D]` | Mentes máquina pueden habitar campos de datos o dimensiones perceptivas sin analogía humana directa. / *Machine minds can inhabit data fields or perceptual dimensions without direct human analogy.* | **Target:** radiant multi-axis sensory tensor nebula without ground plane *(Gap: F3 misses multidimensional glowing data-stream field devoid of horizon, floor, or sky.)* |
| `F4` | **pasados reconstruidos** (*reconstructed pasts*) | `[TIER-D]` | Simulaciones históricas preservan perspectivas parciales, no recuperan automáticamente cada hecho o experiencia perdida. / *Historical simulations preserve partial perspectives, they do not automatically recover every lost fact or experience.* | **Target:** ghostly amber translucent historical dome riddled with jagged missing unrecorded voids *(Gap: F4 misses sepia-gold incomplete glowing museum diorama fragments with dark missing gaps.)* |
| `F5` | **entornos matemáticos** (*mathematical environments*) | `[TIER-D]` | Una mente podría operar en espacios abstractos donde relación y estructura importan más que objetos parecidos a cuerpos. / *A mind could operate in abstract spaces where relationship and structure matter more than body-like objects.* | **Target:** crisp radiant hyperbolic saddle & fiber-bundle mathematical crystal *(Gap: F5 misses gleaming abstract topological knot & phase-space vectors devoid of physical bodies.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP25-Z05-09)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP25-Z05-09` (Deep-Time Minds, Galactic Diasporas, Dyson/Shkadov Engineering & Constructed Worlds) for HyperCosmosVR Odyssey Act V (`Exp 25: 25_final_horizon`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z05`->`3505` (`F0..F5`), `Z06`->`3506` (`F0..F5`), `Z07`->`3507` (`F0..F5`), `Z08`->`3508` (`F0..F5`), `Z09`->`3509` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3862`, `3863`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP25-Z05-09`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z05: long-duration-intelligences / Long-lasting intelligences, Z06: galactic-civilizations / Galactic civilizations, Z07: cosmic-engineering / cosmic engineering, Z08: constructed-reality / Computing and constructed reality, Z09: plural-worlds / The multiplication of worlds.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp25-z10-14"></a>
### PKG-ACT5-EXP25-Z10-14: Merged/Split Identities, Experience & Intelligence Ceilings & Cosmic Event Horizon

- **Phase:** Phase 1 + Phase 4 | **Priority:** `P1` | **Scope:** Exp `25` (`25_final_horizon`) `Z10..Z14` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L88`: `25 => 250` (single variant across all 25 zones!); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L495-L518` (3 nested longitude-ribbed wireframe spheres `r=0.24,0.44,0.64` + 25-petal HSL lotus ring + `horizon_fade`/`slow_rate` across `z=0..24`; `feat` shifts sphere phase `0.3*feat` at `L504`). Tier breakdown for this batch: **`0A / 0B / 2C / 3D` Zones**, **`0A / 0B / 12C / 18D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `3/5` zones and `3/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3510..=3514` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3864`, `3865` (allocated from `3720 + (25 - 11)*10 + k`; Asymptotic Horizon & Bounds Compression (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z10` | `merged-divided-minds` (*Mentes fusionadas y divididas* / *Minds fused and divided*) | `[TIER-D]` | `vid=250` | **`3510`** | `3864` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x06` (0x04 semantic cue mask, 0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z11` | `experience-limits` (*Los límites de la experiencia* / *The limits of experience*) | `[TIER-C]` | `vid=250` | **`3511`** | `3865` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x01` (0x01 cutaway) | `0..5` (`0xFF` macro) |
| `Z12` | `intelligence-limits` (*Los límites de la inteligencia* / *The limits of intelligence*) | `[TIER-D]` | `vid=250` | **`3512`** | `3864` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x02` (0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |
| `Z13` | `knowledge-limits` (*Los límites del conocimiento* / *The limits of knowledge*) | `[TIER-C]` | `vid=250` | **`3513`** | `3865` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 semantic cue mask) | `0..5` (`0xFF` macro) |
| `Z14` | `cosmological-horizon` (*El horizonte cosmológico* / *The cosmological horizon*) | `[TIER-D]` | `vid=250` | **`3514`** | `3864` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 semantic cue mask) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z10` `merged-divided-minds` (`vid=3510`) — Mentes fusionadas y divididas (*Minds fused and divided*) `[TIER-D]`

- **Catalog Metadata:** `scene=10` · `[0.400, 0.440]` · `rgb=[188, 149, 232]` · `sound=(triangle/55.5Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `fusion` · Footer: *INDIVIDUO Y COLECTIVO DEJAN DE SER FRONTERAS FIJAS*
- **Metric & Evidence (ES / EN):** `Memoria compartida + enlaces temporales + separación posterior → nuevos sujetos y nuevas responsabilidades` / *`Shared memory + temporary links + subsequent separation → new subjects and new responsibilities`* · Evidence: `Identidad distribuida · individuo, relación y grupo dejan de ser categorías estables` (*Distributed identity · individual, relationship and group are no longer stable categories*)
- **Spoken Macro Narrative (ES / EN):** Fusionar o dividir mentes obliga a redefinir privacidad, consentimiento, muerte y responsabilidad sin asumir que toda unión es deseable. (*EN:* Merging or dividing minds forces us to redefine privacy, consent, death and responsibility without assuming that all union is desirable.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` static spheres + lotus (`horizon_fade=0.737`, `slow_rate=0.667`); zero paired merging spheres, topological bridge tendrils, mitotic fission, or overlapping Venn fusion (`kw:fusion` ignored!). **Target (`vid=3510`):** Animate fluid topological sphere unions & fissions (`smin` smooth-boolean metaballs + neural bridge filaments) across `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **enlace temporal** (*temporary link*) | `[TIER-D]` | Dos mentes pueden compartir una tarea o sensación durante un intervalo sin perder necesariamente toda separación. / *Two minds can share a task or sensation for an interval without necessarily losing all separation.* | **Target:** cyan & amber cores joined briefly by bright golden fiber bridge then cleanly detached *(Gap: F0 misses two distinct orbs extending a brief luminous synaptic bridge while retaining distinct outer halos.)* |
| `F1` | **memoria común** (*common memory*) | `[TIER-D]` | Acceder a recuerdos ajenos crea conocimiento íntimo y riesgos de manipulación, exposición o apropiación. / *Accessing other people's memories creates intimate knowledge and risks of manipulation, exposure or appropriation.* | **Target:** glowing exposed memory conduit pulsing softly between twin cores *(Gap: F1 misses glowing open archival cloud overlapping two spheres with sharp privacy-leak spark warnings.)* |
| `F2` | **razonamiento colectivo** (*collective reasoning*) | `[TIER-C]` | Varias perspectivas pueden formar una decisión conjunta sin eliminar desacuerdo interno. / *Multiple perspectives can form a joint decision without eliminating internal disagreement.* | **Target:** tightly triangulated three-node chorus radiating joint forward vector despite internal flicker *(Gap: F2 misses pulsing tri-node ensemble debating with counter-phase crimson/cyan oscillations.)* |
| `F3` | **fusión parcial** (*partial fusion*) `[kw:fusion]` | `[TIER-D]` | Solo algunos procesos pueden unirse, produciendo un sistema que es simultáneamente individuo y relación. / *Only some processes can come together, producing a system that is simultaneously individual and relationship.* | **Target:** two glowing semi-intersecting spheres (`smin`) creating radiant gold central overlap while outer wings stay pure *(Gap: F3 misses two translucent spheres passing partially into luminous bright Venn overlap (`kw:fusion`).)* |
| `F4` | **división descendente** (*descending division*) | `[TIER-D]` | Una mente puede separarse en continuaciones que comparten pasado y reclaman futuros independientes. / *A mind can separate into continuations that share a past and claim independent futures.* | **Target:** hourglass fission splitting one orb into diverging indigo & amber trails *(Gap: F4 misses single vibrating glowing orb undergoing clean mitotic fission (`smin` pinch) into twin diverging arcs.)* |
| `F5` | **reconexión** (*reconnection*) | `[TIER-D]` | Copias divergentes pueden intercambiar memorias, pero una integración completa podría destruir diferencias adquiridas. / *Divergent copies can exchange memories, but complete integration could destroy acquired differences.* | **Target:** fine fractal surface engravings smudging upon forced total merger *(Gap: F5 misses two rich uniquely patterned orbs trying to merge cleanly and blurring intricate fine details.)* |

##### `Z11` `experience-limits` (`vid=3511`) — Los límites de la experiencia (*The limits of experience*) `[TIER-C]`

- **Catalog Metadata:** `scene=11` · `[0.440, 0.480]` · `rgb=[160, 154, 234]` · `sound=(sawtooth/53.45Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *DETECTAR NO ES LO MISMO QUE PODER EXPERIMENTAR*
- **Metric & Evidence (ES / EN):** `Sustrato + sentidos + memoria de trabajo + conceptos → espacio limitado de experiencias representables` / *`Substrate + senses + working memory + concepts → limited space of representable experiences`* · Evidence: `Intraducibilidad posible · detectar un estado no equivale a poder vivirlo o comunicarlo` (*Possible untranslatability · detecting a state is not equivalent to being able to live or communicate it*)
- **Spoken Macro Narrative (ES / EN):** Aumentar inteligencia y sensores no garantiza eliminar todas las barreras entre experiencias. (*EN:* Increasing intelligence and sensors does not guarantee eliminating all barriers between experiences.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` 3 nested spheres + rainbow lotus (`horizon_fade=0.711`, `slow_rate=0.633`); lacks opaque subjectivity wall / sensor-to-intuition compression bottleneck. **Target (`vid=3511`):** Stage an iridescent semi-opaque interior sphere ringed by raw external instrument graphs.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **dimensiones sensoriales** (*sensory dimensions*) | `[TIER-C]` | Un sistema puede medir variables que una conciencia no logra organizar como una experiencia unificada. / *A system can measure variables that a consciousness fails to organize as a unified experience.* | **Target:** dazzling 12-axis sensor needles bouncing off smooth inner membrane *(Gap: No crisp outer sensor-grid spikes failing to enter soft interior glowing sphere.)* |
| `F1` | **memoria de trabajo** (*working memory*) | `[TIER-C]` | Comprender simultáneamente demasiadas relaciones puede superar incluso una mente ampliada y exigir simplificación. / *Understanding too many relationships simultaneously can overwhelm even an expanded mind and demand simplification.* | **Target:** intricate cosmic knot simplifying sharply inside small bright center bubble *(Gap: No overflowing swirling knot network collapsing past 7 bright central working slots.)* |
| `F2` | **complejidad conceptual** (*conceptual complexity*) | `[TIER-C]` | Algunas estructuras quizá solo puedan manipularse mediante herramientas sin convertirse en intuiciones vividas. / *Some structures may only be able to be manipulated by tools without becoming lived intuitions.* | **Target:** giant crystal calculator floating untouched outside glowing organic halo *(Gap: No gleaming algebraic/tool crystal operated externally by dim hand beam without inner light.)* |
| `F3` | **estados incompatibles** (*incompatible states*) | `[TIER-C]` | Dos formas de conciencia pueden carecer de una escala común para comparar intensidad, calidad o significado. / *Two forms of consciousness may lack a common scale for comparing intensity, quality, or meaning.* | **Target:** orthogonal perpendicular axes unable to project cleanly onto shared meter *(Gap: No two disjoint color-bars lacking shared axis ticks (incommensurable scales).)* |
| `F4` | **traducción entre sustratos** (*translation between substrates*) | `[TIER-C]` | La misma señal física puede integrarse de manera distinta en cuerpos biológicos, máquinas o colectivos. / *The same physical signal can be integrated differently in biological bodies, machines or collectives.* | **Target:** single incoming pure frequency ray lighting wildly mismatched shapes in 3 substrate domes *(Gap: No single physical sine wave splitting cleanly into mismatched visual/audio/tactile glyphs across substrates.)* |
| `F5` | **experiencia privada** (*private experience*) | `[TIER-C]` | Una descripción perfecta del comportamiento no demuestra acceso directo a cómo se siente otra perspectiva. / *A perfect description of behavior does not demonstrate direct access to how another perspective feels.* | **Target:** smooth mirror-shelled inner core radiating rich unseen symphony behind impenetrable veil *(Gap: No opaque one-way silvered horizon sphere hiding pulsing vibrant color cosmos inside from external scanners.)* |

##### `Z12` `intelligence-limits` (`vid=3512`) — Los límites de la inteligencia (*The limits of intelligence*) `[TIER-D]`

- **Catalog Metadata:** `scene=12` · `[0.480, 0.520]` · `rgb=[132, 165, 232]` · `sound=(triangle/51.4Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *MÁS INTELIGENCIA NO PRODUCE OMNISCIENCIA*
- **Metric & Evidence (ES / EN):** `Memoria finita + energía finita + tiempo finito + complejidad → problemas resistentes o insolubles` / *`Finite memory + finite energy + finite time + complexity → resistant or insoluble problems`* · Evidence: `Capacidad finita · más poder cognitivo no produce omnisciencia ni cálculo instantáneo` (*Finite capacity · more cognitive power does not produce omniscience or instantaneous calculation*)
- **Spoken Macro Narrative (ES / EN):** Una civilización puede ser inmensamente capaz y seguir encontrando preguntas que no puede responder eficientemente, o en absoluto. (*EN:* A civilization can be immensely capable and still encounter questions that it cannot answer efficiently, or at all.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` 3 slotted spheres + lotus (`horizon_fade=0.684`, `slow_rate=0.60`); zero thermodynamic Landauer heat plumes, cellular-automaton irreducibility tower, Lorenz strange attractor chaos, or Gödel/Turing self-referential loop. **Target (`vid=3512`):** Build a crisp mathematical-thermodynamic ceiling engine.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **memoria finita** (*finite memory*) | `[TIER-C]` | Toda mente física debe seleccionar qué conservar, comprimir o olvidar cuando la información supera su capacidad. / *Every physical mind must select what to keep, compress, or forget when information exceeds its capacity.* | **Target:** massive star cloud compressed into sparse bright knots inside finite wireframe cube *(Gap: No dense data nebula filtering down through tight finite archival sieve.)* |
| `F1` | **coste energético** (*energy cost*) | `[TIER-D]` | Pensar, medir y comunicar requieren energía y generan calor; la inteligencia no está separada de la termodinámica. / *Thinking, measuring and communicating require energy and generate heat; Intelligence is not separate from thermodynamics.* | **Target:** gleaming compute core pulsing crisp blue bits outward into billowing crimson Landauer infrared dissipation plumes *(Gap: F1 misses crisp glowing bit-flip operations emitting swirling orange-red infrared waste heat.)* |
| `F2` | **irreducibilidad** (*irreducibility*) | `[TIER-D]` | Algunos sistemas solo revelan su comportamiento al ejecutarse paso a paso, sin atajo predictivo conocido. / *Some systems only reveal their behavior by executing step by step, with no known predictive shortcut.* | **Target:** stacked step-by-step cellular lattice cone impossible to shortcut with dashed failed jump arrow *(Gap: F2 misses step-by-step cellular-automaton cascade forbidding skip-ahead tunnel shortcut.)* |
| `F3` | **caos** (*chaos*) | `[TIER-D]` | Errores mínimos en condiciones iniciales pueden crecer hasta destruir predicciones detalladas a largo plazo. / *Minimal errors in initial conditions can grow to destroy detailed long-term predictions.* | **Target:** vibrant 3D Lorenz butterfly chaotic ribbon peeling twin initially overlapping sparks cleanly onto opposite wings *(Gap: F3 misses classic Lorenz butterfly double-scroll strange attractor splitting twin nearby green/red sparks.)* |
| `F4` | **indecidibilidad** (*undecidability*) | `[TIER-D]` | Existen clases de problemas para las que ningún procedimiento general puede garantizar una respuesta correcta. / *There are classes of problems for which no general procedure can guarantee a correct answer.* | **Target:** endless non-halting fractal labyrinth enclosing search pulse *(Gap: F4 misses formal maze/halting wall trapping algorithmic beams inside non-terminating horizon.)* |
| `F5` | **autorreferencia** (*self-reference*) | `[TIER-D]` | Una mente que intenta modelarse completamente puede alterar el sistema que estudia y encontrar límites lógicos internos. / *A mind that attempts to model itself completely can alter the system it studies and find internal logical limits.* | **Target:** optical self-probe curving back to perturb the exact core it measures (ouroboros glitch loop) *(Gap: F5 misses self-measuring scanner beam curving inward to destabilize & scramble its own target core.)* |

##### `Z13` `knowledge-limits` (`vid=3513`) — Los límites del conocimiento (*The limits of knowledge*) `[TIER-C]`

- **Catalog Metadata:** `scene=13` · `[0.520, 0.560]` · `rgb=[112, 176, 224]` · `sound=(sine/49.35Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `horizonte` · Footer: *NO SABIDO TODAVÍA ≠ INACCESIBLE EN PRINCIPIO*
- **Metric & Evidence (ES / EN):** `Evidencia + causalidad + medición + cómputo + conceptos → frontera cambiante de lo conocible` / *`Evidence + causality + measurement + computation + concepts → shifting frontier of the knowable`* · Evidence: `Grados de ignorancia · lo no sabido hoy no es igual a lo inaccesible en principio` (*Degrees of ignorance · what is not known today is not the same as what is inaccessible in principle*)
- **Spoken Macro Narrative (ES / EN):** La app distingue lo desconocido temporalmente de aquello que podría permanecer desconocido para cualquier observador físicamente posible. (*EN:* The app distinguishes what is temporarily unknown from what could remain unknown to any physically possible observer.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` 3 concentric spheres + lotus (`horizon_fade=0.658`, `slow_rate=0.567`); concentric spheres loosely suggest horizons, yet F0..F5 only step sphere phase by 17 deg without separating temporary cloud vs permanent horizon. **Target (`vid=3513`):** Place three nested graded ignorance membranes centered around an observatory core.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **aún no observado** (*not yet observed*) | `[TIER-C]` | Nueva instrumentación o exploración puede convertir una ausencia de datos en conocimiento futuro. / *New instrumentation or exploration can turn a lack of data into future knowledge.* | **Target:** soft mist outer shell pierced cleanly by expanding bright telescope scan wedge *(Gap: No translucent fog veil dissolving cleanly ahead of new telescope/probe laser arc.)* |
| `F1` | **información destruida** (*destroyed information*) | `[TIER-D]` | Registros perdidos pueden dejar solo trazas insuficientes para reconstruir una historia única. / *Lost records may leave only insufficient traces to reconstruct a unique history.* | **Target:** fragmentary scorched relic ring bridging 3 equally plausible ghost trajectories backward *(Gap: F1 misses shattered fossil archive ring whose missing arcs make multiple reconstructions diverge.)* |
| `F2` | **fuera de contacto** (*out of touch*) `[kw:horizonte]` | `[TIER-D]` | Acontecimientos más allá de un horizonte causal no pueden enviar información hasta nosotros. / *Events beyond a causal horizon cannot send information to us.* | **Target:** blazing crisp causal light-cone surface locking external flares forever outside contact *(Gap: F2 misses strict relativistic causal light-cone wall (`kw:horizonte`) separating zero-signal exterior.)* |
| `F3` | **medición limitada** (*limited measurement*) | `[TIER-D]` | Todo instrumento posee resolución, ruido y una interacción física con aquello que intenta medir. / *Every instrument has resolution, noise and a physical interaction with what it is trying to measure.* | **Target:** laser probe pulse visibly knocking delicate measured particle off-course (`Heisenberg`/disturbance) *(Gap: F3 misses active probe photon kick visibly scrambling delicate target trajectory upon measurement.)* |
| `F4` | **cálculo imposible** (*impossible calculation*) | `[TIER-C]` | Una respuesta puede estar definida y aun exigir más tiempo, memoria o energía de la que existe disponible. / *A response can be defined and even require more time, memory or energy than is available.* | **Target:** combinatorial explosion fractal bursting right past outer compute shell energy gauge *(Gap: No towering exponential tree outscaling physical volume/energy capsule bounds.)* |
| `F5` | **concepto inaccesible** (*inaccessible concept*) | `[TIER-C]` | Una arquitectura mental puede carecer de representaciones necesarias para formular una pregunta que otra mente sí comprende. / *A mental architecture may lack the representations necessary to formulate a question that another mind does understand.* | **Target:** glowing tilted 3D object invisible to flat 2D plane slice *(Gap: No hidden higher-dimensional cross-section invisible to flat 2D observer plane.)* |

##### `Z14` `cosmological-horizon` (`vid=3514`) — El horizonte cosmológico (*The cosmological horizon*) `[TIER-D]`

- **Catalog Metadata:** `scene=14` · `[0.560, 0.600]` · `rgb=[96, 184, 214]` · `sound=(triangle/47.3Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `horizonte` · Footer: *EL UNIVERSO OBSERVABLE PUEDE ENCOGER LOCALMENTE*
- **Metric & Evidence (ES / EN):** `Expansión acelerada + velocidad finita de la luz → regiones que dejan de ser causalmente alcanzables` / *`Accelerated expansion + finite speed of light → regions that are no longer causally reachable`* · Evidence: `Pérdida de acceso · la expansión puede reducir el universo observable del futuro` (*Loss of access · expansion may shrink the observable universe of the future*)
- **Spoken Macro Narrative (ES / EN):** El universo disponible para observación puede disminuir incluso mientras el conocimiento y la tecnología aumentan localmente. (*EN:* The universe available for observation may decrease even as knowledge and technology increase locally.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` draws 3 bright physical-looking slotted cages + full 25-petal rainbow lotus (`horizon_fade=0.632`, `slow_rate=0.533`); ACTIVELY CONTRADICTS `F4 causal horizon` ('It is not a material wall') and shows ZERO receding galaxies, redshift stretch, or emptying sky! **Target (`vid=3514`):** Replace solid wireframe cages with an expanding relativistic de-Sitter causal horizon simulation.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **galaxias que se alejan** (*galaxies that move away*) | `[TIER-D]` | La luz emitida hoy por regiones muy distantes puede no alcanzar nunca a observadores ligados gravitacionalmente aquí. / *Light emitted today from very distant regions may never reach observers gravitationally bound here.* | **Target:** hundreds of spiral-galaxy sprites (`instanced_sprites` outward radial stretch) sliding across glowing crimson-to-black redshift horizon *(Gap: F0 misses distant galaxies accelerating outward, redshifting crimson and winking out across dark rim.)* |
| `F1` | **región accesible** (*accessible region*) | `[TIER-D]` | Solo una parte del cosmos puede intercambiar materia, señales o influencia con una civilización dada. / *Only part of the cosmos can exchange matter, signals, or influence with a given civilization.* | **Target:** bound warm Local-Group galaxy cluster glowing calmly inside quiet clear bubble *(Gap: F1 misses sharp glowing gravitational-bound inner island glowing warm while outer cosmos stretches void.)* |
| `F2` | **cielo empobrecido** (*impoverished sky*) | `[TIER-D]` | Futuras civilizaciones aisladas podrían ver pocas galaxias y deducir con mayor dificultad la antigua expansión cósmica. / *Future isolated civilizations could see few galaxies and deduce the ancient cosmic expansion with greater difficulty.* | **Target:** dense starry sky shell dramatically dimming outward until only core cluster dots remain *(Gap: F2 misses sweeping sky-fade collapsing rich thousand-galaxy night into lonely single-galaxy blackness.)* |
| `F3` | **evidencia antigua** (*ancient evidence*) | `[TIER-D]` | Archivos del fondo cósmico y de galaxias lejanas pueden volverse esenciales cuando las señales naturales se debiliten. / *Archives of the cosmic background and distant galaxies may become essential when natural signals weaken.* | **Target:** faint glowing redshifted CMB afterglow sphere inscribed inside archival glass crystal *(Gap: F3 misses fragile pale CMB ripple sphere & archived galaxy blueprint slabs preserving lost sky.)* |
| `F4` | **horizonte causal** (*causal horizon*) `[kw:horizonte]` | `[TIER-D]` | No es una pared material, sino un límite de qué acontecimientos pueden conectarse mediante señales. / *It is not a material wall, but a limit on what events can be connected by signals.* | **Target:** phantom soft translucent asymptotic light-cone wave—zero hard walls, just light rays asymptoting inward *(Gap: F4 misses immaterial light-cone asymptotic boundary—3 solid slotted wireframe spheres actually suggest a material cage!)* |
| `F5` | **conocimiento local** (*local knowledge*) | `[TIER-C]` | Comprender mejor una región no recupera automáticamente información de regiones que han abandonado contacto causal. / *Better understanding a region does not automatically recover information from regions that have abandoned causal contact.* | **Target:** brightly refined local map contrasting unbroken zero-star outer void *(Gap: No crisp deepening inner island clarity contrasted cleanly against permanent zero-signal outer dark.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP25-Z10-14)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP25-Z10-14` (Merged/Split Identities, Experience & Intelligence Ceilings & Cosmic Event Horizon) for HyperCosmosVR Odyssey Act V (`Exp 25: 25_final_horizon`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z10`->`3510` (`F0..F5`), `Z11`->`3511` (`F0..F5`), `Z12`->`3512` (`F0..F5`), `Z13`->`3513` (`F0..F5`), `Z14`->`3514` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3864`, `3865`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP25-Z10-14`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z10: merged-divided-minds / Minds fused and divided, Z11: experience-limits / The limits of experience, Z12: intelligence-limits / The limits of intelligence, Z13: knowledge-limits / The limits of knowledge, Z14: cosmological-horizon / The cosmological horizon.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp25-z15-19"></a>
### PKG-ACT5-EXP25-Z15-19: Disconnected Hubble Islands, Free-Energy Budget, Late Cosmos, Information & Heat Death

- **Phase:** Phase 1 + Phase 4 | **Priority:** `P0` | **Scope:** Exp `25` (`25_final_horizon`) `Z15..Z19` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L88`: `25 => 250` (single variant across all 25 zones!); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L495-L518` (3 nested longitude-ribbed wireframe spheres `r=0.24,0.44,0.64` + 25-petal HSL lotus ring + `horizon_fade`/`slow_rate` across `z=0..24`; `feat` shifts sphere phase `0.3*feat` at `L504`). Tier breakdown for this batch: **`0A / 0B / 0C / 5D` Zones**, **`0A / 0B / 6C / 24D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `1/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3515..=3519` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3866`, `3867` (allocated from `3720 + (25 - 11)*10 + k`; Thermodynamic Fade (`horizon_fade`) + Slowed Quantum Drift (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z15` | `isolated-civilizations` (*Civilizaciones aisladas* / *Isolated civilizations*) | `[TIER-D]` | `vid=250` | **`3515`** | `3866` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 semantic cue mask) | `0..5` (`0xFF` macro) |
| `Z16` | `energy-budget` (*El presupuesto energético* / *The energy budget*) | `[TIER-D]` | `vid=250` | **`3516`** | `3867` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z17` | `late-universe` (*El universo tardío* / *The late universe*) | `[TIER-D]` | `vid=250` | **`3517`** | `3866` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z18` | `information-destiny` (*El destino de la información* / *The fate of information*) | `[TIER-D]` | `vid=250` | **`3518`** | `3867` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z19` | `heat-death` (*La muerte térmica* / *thermal death*) | `[TIER-D]` | `vid=250` | **`3519`** | `3866` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z15` `isolated-civilizations` (`vid=3515`) — Civilizaciones aisladas (*Isolated civilizations*) `[TIER-D]`

- **Catalog Metadata:** `scene=15` · `[0.600, 0.640]` · `rgb=[88, 172, 202]` · `sound=(sawtooth/45.25Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `horizonte` · Footer: *UN COSMOS COMPARTIDO PUEDE VOLVERSE MUCHOS COSMOS AISLADOS*
- **Metric & Evidence (ES / EN):** `Horizontes + retrasos + extinción posible → historias comunes convertidas en archivos incompletos` / *`Horizons + delays + possible extinction → common stories turned into incomplete files`* · Evidence: `Separación irreversible · culturas relacionadas dejan de poder confirmar su supervivencia mutua` (*Irreversible separation · related cultures stop being able to confirm their mutual survival*)
- **Spoken Macro Narrative (ES / EN):** Un universo una vez compartido puede convertirse en muchos universos observables aislados. (*EN:* A once shared universe can become many isolated observable universes.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` 3 concentric slotted spheres + lotus (`horizon_fade=0.605`, `slow_rate=0.50`); zero separated causal islands, stranded drifting photon beams chasing receding targets, or isolated local light-cones. **Target (`vid=3515`):** Stage multiple widely separated glowing causal horizon bubbles drifting rapidly apart across OLED void.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **mensajes sin llegada** (*messages without arrival*) | `[TIER-D]` | Una señal puede viajar indefinidamente hacia una región que se aleja demasiado rápido para recibirla. / *A signal can travel indefinitely to a region that is moving away too quickly to receive it.* | **Target:** crisp cyan laser wave expanding forward while target galaxy retreats faster, stretching dark chasm *(Gap: F0 misses luminous radio/laser packet chasing an accelerating sister island galaxy that widens faster than c.)* |
| `F1` | **culturas perdidas** (*lost cultures*) | `[TIER-D]` | Una civilización conserva relatos de otras sin poder saber si continuaron, cambiaron o desaparecieron. / *One civilization preserves stories of others without being able to know if they continued, changed or disappeared.* | **Target:** central glowing island cradling antique golden holographic echo of vanished neighbor *(Gap: F1 misses warm archival shrine inside Bubble-A holding dead static holographic snapshot of distant Bubble-B.)* |
| `F2` | **archivos parciales** (*partial files*) | `[TIER-C]` | Copias antiguas preservan voces y decisiones, pero no sustituyen una relación causal todavía activa. / *Old copies preserve voices and decisions, but do not replace a still active causal relationship.* | **Target:** dim static archived trace looping softly beside severed dotted transmission link *(Gap: No fading broken historical voice loop contrasting dead archived transcript vs live pulsing reality.)* |
| `F3` | **supervivencia incierta** (*uncertain survival*) | `[TIER-D]` | La ausencia de nuevas señales ya no permite distinguir silencio, transformación, extinción o separación cosmológica. / *The absence of new signals no longer allows us to distinguish silence, transformation, extinction or cosmological separation.* | **Target:** quiet dark question void cycling four translucent faint ghost silhouettes of silent neighbor *(Gap: F3 misses ambiguous blank silent sky hiding 4 divergent ghost outcomes (quiet/changed/extinct/beyond horizon).)* |
| `F4` | **memoria del cosmos** (*memory of the cosmos*) | `[TIER-D]` | Mapas y observaciones tempranas pueden conservar evidencia de una época en que el cielo contenía muchas galaxias. / *Early maps and observations can preserve evidence of a time when the sky contained many galaxies.* | **Target:** split holographic globe showing lush rich ancient galaxy map vs stark empty present window *(Gap: F4 misses golden ancient full-sky star map glowing beside barren black contemporary telescope view.)* |
| `F5` | **pluralidad desconectada** (*disconnected plurality*) `[kw:horizonte]` | `[TIER-D]` | Cada región puede construir una historia verdadera pero incompleta a partir de su propio horizonte observable. / *Each region can construct a true but incomplete history based on its own observable horizon.* | **Target:** three separate glowing horizon orbs floating isolated in pitch-black space *(Gap: F5 misses 4 disconnected glowing Hubble-bubble islands (`kw:horizonte`) each illuminating only its tiny sphere.)* |

##### `Z16` `energy-budget` (`vid=3516`) — El presupuesto energético (*The energy budget*) `[TIER-D]`

- **Catalog Metadata:** `scene=16` · `[0.640, 0.680]` · `rgb=[82, 151, 184]` · `sound=(triangle/43.2Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *ENERGÍA TOTAL NO EQUIVALE A TRABAJO DISPONIBLE*
- **Metric & Evidence (ES / EN):** `Gradientes aprovechables ÷ coste por proceso → duración y ritmo posibles de una civilización` / *`Usable gradients ÷ cost per process → possible duration and pace of a civilization`* · Evidence: `Termodinámica práctica · energía total no equivale a energía disponible para realizar trabajo` (*Practical thermodynamics · total energy does not equal energy available to do work*)
- **Spoken Macro Narrative (ES / EN):** Continuar exige diferencias de energía, disipación de calor y recursos capaces de sostener organización. (*EN:* Continuing requires differences in energy, heat dissipation and resources capable of sustaining organization.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` identical rainbow lotus + 3 wire spheres (`horizon_fade=0.579`, `slow_rate=0.467`); zero yellow-to-crimson stellar aging, white dwarf/neutron pulsar, black hole Hawking/accretion engine, or Landauer cold-compute sink. **Target (`vid=3516`):** Build a Deep-Time Cosmic Battery & Stellar Furnace gauge morphing across thermodynamic eras per `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **energía estelar** (*stellar energy*) | `[TIER-D]` | Las estrellas ofrecen gradientes abundantes durante tiempos finitos y con tasas que cambian según su masa. / *Stars offer abundant gradients for finite times and at rates that change depending on their mass.* | **Target:** brilliant sunlit yellow star driving rich golden gradient turbine torus *(Gap: F0 misses blazing blue/yellow main-sequence stars pouring rich high-gradient heat fluxes quickly.)* |
| `F1` | **enanas rojas** (*red dwarfs*) | `[TIER-D]` | Estrellas pequeñas pueden durar billones de años, aunque sus entornos y actividad presentan desafíos propios. / *Small stars can last for billions of years, although their environments and activity present challenges of their own.* | **Target:** steady glowing deep-crimson M-dwarf sphere flanked by subtle magnetic flare loops *(Gap: F1 misses compact deep-crimson convective red dwarf burning steadily at low luminosity beside flare arcs.)* |
| `F2` | **remanentes** (*remnants*) | `[TIER-D]` | Enanas blancas, estrellas de neutrones y acreción tardía ofrecen fuentes más raras y difíciles de utilizar. / *White dwarfs, neutron stars and late accretion offer rarer and more difficult to use sources.* | **Target:** tiny blazing diamond-white dwarf core paired with tilted lighthouse-pulsing neutron star beams *(Gap: F2 misses glowing dense crystalline white dwarf & tilted spinning magnetized neutron star beam.)* |
| `F3` | **agujeros negros** (*black holes*) | `[TIER-D]` | Rotación, acreción y evaporación pueden proporcionar energía en eras distintas sin constituir reservas infinitas. / *Rotation, accretion and evaporation can provide energy in different eras without constituting infinite reserves.* | **Target:** pitch-black black hole horizon surrounded by swirling golden accretion ring & subtle Hawking rim glow *(Gap: F3 misses pitch-black Kerr black hole shedding bright accretion spirals & delicate quantum Hawking halo.)* |
| `F4` | **cálculo eficiente** (*efficient calculation*) | `[TIER-D]` | Reducir operaciones, temperatura y errores extiende recursos, pero no elimina todos los costes físicos. / *Reducing operations, temperature, and errors stretches resources, but does not eliminate all physical costs.* | **Target:** chilled ultra-cold sapphire lattice processor glowing sharply near dark background (`kT ln 2` efficiency) *(Gap: F4 misses cryogenic superconducting blue crystal processor dissipating tiny heat pulses vs blazing warm chip.)* |
| `F5` | **hibernación lenta** (*slow hibernation*) | `[TIER-D]` | Esperar entre eventos raros puede prolongar tiempo externo mientras reduce la continuidad de experiencia activa. / *Waiting between rare events can prolong external time while reducing the continuity of active experience.* | **Target:** slow low-frequency ember sleeping calmly while cosmic background cools *(Gap: F5 misses pulsing slow hibernation beacon saving glowing reserve battery ticks across huge void spans.)* |

##### `Z17` `late-universe` (`vid=3517`) — El universo tardío (*The late universe*) `[TIER-D]`

- **Catalog Metadata:** `scene=17` · `[0.680, 0.720]` · `rgb=[93, 139, 165]` · `sound=(sine/41.15Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *LA INTELIGENCIA ACTÚA DENTRO DEL DECLIVE CÓSMICO*
- **Metric & Evidence (ES / EN):** `Formación estelar decreciente → remanentes → agujeros negros → interacciones cada vez más escasas` / *`Decreasing star formation → remnants → black holes → increasingly rare interactions`* · Evidence: `Supervivencia en declive · inteligencia dentro de un cosmos con menos estrellas y menos encuentros` (*Survival in decline · intelligence within a cosmos with fewer stars and fewer encounters*)
- **Spoken Macro Narrative (ES / EN):** La pregunta ya no es solo qué ocurre físicamente, sino qué estrategias permanecen disponibles para seres que intentan continuar. (*EN:* The question is no longer just what happens physically, but what strategies remain available to beings that try to continue.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` 3 golden-cyan spheres + bright rainbow lotus (`horizon_fade=0.553`, `slow_rate=0.433`); bright rainbow 25-petal neon lotus belies the frozen Degenerate & Black Hole Eras described! **Target (`vid=3517`):** Stepped cosmic-era transition lowering luminous rainbow saturation toward stark deep-time embers across `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **últimas estrellas** (*last stars*) | `[TIER-D]` | Enanas rojas prolongan la era estelar mientras la creación de nuevas estrellas se vuelve cada vez menos frecuente. / *Red dwarfs prolong the stellar era while the creation of new stars becomes less and less frequent.* | **Target:** dark unlit galactic cloud with tiny fading crimson dwarf embers *(Gap: F0 misses dim ember-crimson last red dwarfs flickering alone in dark non-star-forming galaxy.)* |
| `F1` | **mundos remanentes** (*remnant worlds*) | `[TIER-D]` | Planetas y hábitats deben adaptarse a fuentes débiles, encuentros raros y ambientes progresivamente fríos. / *Planets and habitats must adapt to weak sources, rare encounters, and progressively cold environments.* | **Target:** frosted slate rogue planet cocooned inside tiny dim artificial thermal dome ring *(Gap: F1 misses frozen crystalline rogue planet drifting near faint dim dying ember without starlight.)* |
| `F2` | **desintegración incierta** (*uncertain disintegration*) | `[TIER-D]` | La posible desintegración del protón cambiaría profundamente el futuro de la materia, pero todavía no está confirmada. / *The possible decay of the proton would profoundly change the future of matter, but it is not yet confirmed.* | **Target:** translucent iron-crystal cube flickering subtly as scattered quantum decay sparks fizzle (with uncertain question fade) *(Gap: F2 misses subtle matter-lattice sparkle flickering uncertainly toward faint quantum decay dissolution.)* |
| `F3` | **era degenerada** (*degenerate era*) | `[TIER-D]` | Enanas blancas y estrellas de neutrones dominan un universo donde la materia luminosa pierde actividad. / *White dwarfs and neutron stars dominate a universe where luminous matter loses activity.* | **Target:** cool icy white-blue diamond dwarf sphere + crisp faint pulsar beam in vast blackness *(Gap: F3 misses dark expanse lit sparsely only by cool diamond white dwarfs & lonely magnetized neutron stars.)* |
| `F4` | **era de agujeros negros** (*era of black holes*) | `[TIER-D]` | Durante tiempos inmensos, los agujeros negros pueden convertirse en las estructuras compactas principales. / *For immense times, black holes can become the main compact structures.* | **Target:** massive deep-black gravitational shadow halo warping faint edge starlight slowly (`Horizon` dominance) *(Gap: F4 misses colossal silent dark black-hole horizons dominating void while stellar lights stay dead.)* |
| `F5` | **intervalos vacíos** (*empty intervals*) | `[TIER-D]` | Acontecimientos útiles aparecen separados por duraciones que superan cualquier escala biológica familiar. / *Useful events appear separated by durations that exceed any familiar biological scale.* | **Target:** pitch-black expanse lit once briefly by sparse luminous spark flash *(Gap: F5 misses rare single blue spark flashing across vast black oceanic intervals.)* |

##### `Z18` `information-destiny` (`vid=3518`) — El destino de la información (*The fate of information*) `[TIER-D]`

- **Catalog Metadata:** `scene=18` · `[0.720, 0.760]` · `rgb=[109, 130, 150]` · `sound=(triangle/39.1Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *PRESERVAR UN PATRÓN NO GARANTIZA PODER RECUPERARLO*
- **Metric & Evidence (ES / EN):** `Registro + soporte + mantenimiento + acceso → posibilidad de reconstruir un pasado` / *`Registration + support + maintenance + access → possibility of reconstructing a past`* · Evidence: `Frontera física y conceptual · preservar un patrón exige que alguna diferencia siga siendo recuperable` (*Physical and conceptual boundary · preserving a pattern requires that some difference remain recoverable.*)
- **Spoken Macro Narrative (ES / EN):** La física moderna sugiere restricciones profundas sobre la información, pero la recuperación práctica y la interpretación continúan siendo problemas distintos. (*EN:* Modern physics suggests profound restrictions on information, but practical retrieval and interpretation remain distinct problems.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` static 3 wire cages + lotus (`horizon_fade=0.526`, `slow_rate=0.40`); zero scrambled phase-space bit cloud, evaporating black hole horizon radiation, or radiation-damaged ECC storage crystal. **Target (`vid=3518`):** Stage an iridescent crystal holographic archive plate beside an evaporating black-hole horizon across `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **preservación fundamental** (*fundamental preservation*) | `[TIER-C]` | Que la evolución física conserve información en algún sentido no implica que una civilización pueda localizarla o reconstruirla. / *That physical evolution preserves information in some sense does not imply that a civilization can locate or reconstruct it.* | **Target:** crisp luminous mandala dissolving deterministically into vast diffuse unreadable star-fog *(Gap: No crisp structured geometric mosaic exploding smoothly into invisible scrambled chaotic phase-space swirl.)* |
| `F1` | **agujeros negros** (*black holes*) | `[TIER-D]` | La relación entre evaporación, horizontes e información es un problema central de la gravedad cuántica. / *The relationship between evaporation, horizons and information is a central problem of quantum gravity.* | **Target:** pitch-black event horizon orb shedding shimmering entangled red-cyan Hawking particle pairs upward (`sd_sphere` dark core + subtle rim emission) *(Gap: F1 misses black hole horizon shedding faint Hawking quantum pairs & entangled Page-curve arc (`kw:horizonte`).)* |
| `F2` | **archivos duraderos** (*durable archives*) | `[TIER-D]` | Todo soporte enfrenta errores, radiación, colisiones y cambios de contexto; preservar requiere redundancia y mantenimiento. / *Every support faces errors, radiation, collisions and context switches; preserving requires redundancy and maintenance.* | **Target:** gleaming hexagonal ECC archive monolith dodging radiant violet cosmic-ray slashes with active redundant stitching beams *(Gap: F2 misses golden lattice archive plate scarred by violet cosmic-ray strikes healed by ECC repair beams.)* |
| `F3` | **pasado incompleto** (*incomplete past*) | `[TIER-C]` | Trazas parciales pueden permitir varias historias compatibles sin señalar cuál ocurrió realmente. / *Partial traces can allow several compatible stories without indicating which one actually occurred.* | **Target:** fragmented partial relic arcs fanning three faint plausible ghost stories *(Gap: No shattered fragment puzzle admitting 3 conflicting ghost trajectory reconstructions.)* |
| `F4` | **patrón inaccesible** (*inaccessible pattern*) | `[TIER-C]` | Una estructura puede persistir físicamente y carecer de cualquier proceso capaz de leerla. / *A structure can persist physically and lack any process capable of reading it.* | **Target:** spotless glowing crystal disk untouched beside darkened broken optical reader *(Gap: No intact glowing crystalline pattern matrix glowing bright beside dark broken reader head.)* |
| `F5` | **significado perdido** (*lost meaning*) | `[TIER-C]` | Bits intactos no garantizan conservar el idioma, la referencia o la experiencia que alguna vez expresaron. / *Unbroken bits are no guarantee of preserving the language, reference or experience they once expressed.* | **Target:** bright intact binary ring glowing warmly over darkened faded semantic tree *(Gap: No gleaming Rosetta glyph beads floating detached from vanished living referents.)* |

##### `Z19` `heat-death` (`vid=3519`) — La muerte térmica (*thermal death*) `[TIER-D]`

- **Catalog Metadata:** `scene=19` · `[0.760, 0.800]` · `rgb=[126, 123, 139]` · `sound=(sawtooth/37.05Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *LA MUERTE TÉRMICA ES PÉRDIDA DE GRADIENTES ÚTILES*
- **Metric & Evidence (ES / EN):** `Entropía creciente + expansión + equilibrio local → capacidad decreciente de realizar trabajo` / *`Increasing entropy + expansion + local equilibrium → decreasing capacity to do work`* · Evidence: `Máxima dispersión práctica · disminuyen los gradientes que sostienen procesos complejos` (*Maximum practical dispersion · gradients that support complex processes decrease*)
- **Spoken Macro Narrative (ES / EN):** La muerte térmica no es una desaparición instantánea, sino un futuro donde mantener organización resulta progresivamente más difícil. (*EN:* Thermal death is not an instant disappearance, but rather a future where maintaining organization becomes progressively more difficult.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` still draws crisp high-contrast glowing gold/cyan lines & full rainbow 25-petal lotus (`horizon_fade=0.50`, `slow_rate=0.367`); high-contrast neon flower fails to depict thermodynamic gradient flattening, dissolving bound structures, or rare stochastic fluctuations. **Target (`vid=3519`):** Make thermodynamic dissipation visibly transform geometry rather than merely dimming overall exposure.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **gradientes menores** (*minor gradients*) | `[TIER-D]` | Diferencias de temperatura y densidad se reducen, limitando motores, metabolismo y cálculo. / *Differences in temperature and density are reduced, limiting motors, metabolism and calculation.* | **Target:** sharp bright warm-to-cold temperature dipole spreading smoothly outward into uniform dim slate mist *(Gap: F0 misses crisp fiery-gold hot & icy-blue cold poles blending irreversibly into faint uniform grey haze.)* |
| `F1` | **trabajo escaso** (*scarce usable work*) | `[TIER-D]` | Puede existir energía total mientras casi ninguna transformación útil pueda extraerse de ella. / *Total energy can exist as long as almost no useful transformation can be extracted from it.* | **Target:** still dark idle computational gear surrounded by flat non-working ambient glow *(Gap: F1 misses stationary stopped turbine/wheel immersed inside pale non-zero uniform thermal fog.)* |
| `F2` | **procesos lentos** (*slow processes*) | `[TIER-D]` | Las interacciones continúan, pero ocurren con tasas extremadamente bajas y largas pausas. / *Interactions continue, but they occur at extremely low rates and long pauses.* | **Target:** sparse drifting photon specks crawling infinitely slowly across dark void *(Gap: F2 misses ultra-rare faint particle collision tick separated by vast dark dead time.)* |
| `F3` | **fluctuaciones** (*fluctuations*) | `[TIER-D]` | A escalas enormes pueden aparecer fluctuaciones raras sin proporcionar una estrategia segura de supervivencia. / *At enormous scales, rare fluctuations can appear without providing a safe survival strategy.* | **Target:** flat near-black canvas spiking tiny fleeting Boltzmann fluctuation bubbles that dissolve immediately *(Gap: F3 misses sudden tiny random transient spark popping fleetingly inside uniform fog and vanishing.)* |
| `F4` | **estructura disuelta** (*dissolved structure*) | `[TIER-D]` | Sistemas ligados pueden decaer o separarse hasta que casi no queden configuraciones complejas activas. / *Linked systems can decay or separate until there are almost no active complex configurations left.* | **Target:** nested spheres actually unthreading & expanding into sparse diffuse uniform haze *(Gap: F4 misses bound wireframe lattice structures untethering and drifting silently into featureless dust.)* |
| `F5` | **experiencia límite** (*limits of experience*) | `[TIER-C]` | Sostener una secuencia indefinida de estados conscientes puede encontrar un límite termodinámico real. / *Sustaining an indefinite sequence of conscious states can encounter a real thermodynamic limit.* | **Target:** lone fragile conscious pulse decelerating toward complete gentle stillness *(Gap: No faint dimming flickering conscious thought-loop slowing cleanly against Landauer zero-gradient wall.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP25-Z15-19)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP25-Z15-19` (Disconnected Hubble Islands, Free-Energy Budget, Late Cosmos, Information & Heat Death) for HyperCosmosVR Odyssey Act V (`Exp 25: 25_final_horizon`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z15`->`3515` (`F0..F5`), `Z16`->`3516` (`F0..F5`), `Z17`->`3517` (`F0..F5`), `Z18`->`3518` (`F0..F5`), `Z19`->`3519` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3866`, `3867`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP25-Z15-19`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z15: isolated-civilizations / Isolated civilizations, Z16: energy-budget / The energy budget, Z17: late-universe / The late universe, Z18: information-destiny / The fate of information, Z19: heat-death / thermal death.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---

<a id="pkg-act5-exp25-z20-24"></a>
### PKG-ACT5-EXP25-Z20-24: Unknown Frontiers, Witnessless Matter, Cosmic Legacy, Common History & Finale Lotus Aperture

- **Phase:** Phase 1 + Phase 4 | **Priority:** `P0` | **Scope:** Exp `25` (`25_final_horizon`) `Z20..Z24` (`5` zones, `30` features) | **Prerequisites:** `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`
- **Current State:** `doc_stages.rs#L88`: `25 => 250` (single variant across all 25 zones!); rendered by `shaders/hero_doc_civilization_horizon.wgsl#L495-L518` (3 nested longitude-ribbed wireframe spheres `r=0.24,0.44,0.64` + 25-petal HSL lotus ring + `horizon_fade`/`slow_rate` across `z=0..24`; `feat` shifts sphere phase `0.3*feat` at `L504`). Tier breakdown for this batch: **`0A / 1B / 3C / 1D` Zones**, **`0A / 6B / 12C / 12D` Features** (`doc_feat` unused or pulse/tilt/lens-index only; `DOMAIN_CUE_KEYWORDS` matches `1/5` zones and `1/30` features; `doc_stages.rs` drops `cue_tags`).
- **Reserved Target IDs:** Primary `vid = 3520..=3524` (`kind = 9`, decoded via `vid >= 1000u`); Companion slots `3868`, `3869` (allocated from `3720 + (25 - 11)*10 + k`; Phase 4 Finale: 25-Petal Jewel Box + `far_shell` Void Crown (`mode=15`)):

| Zone | Slug (`ES / EN`) | Tier | Current Dispatch | Target `vid` | Companion `vid` | `submode` | `base_flags` (`kind_tags.w`) | `doc_feat` Slots |
|---|---|---:|---|---:|---:|---|---|---|
| `Z20` | `unknown-possibilities` (*Posibilidades desconocidas* / *Unknown possibilities*) | `[TIER-D]` | `vid=250` | **`3520`** | `3868` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z21` | `observerless-existence` (*Existencia sin observadores* / *Existence without observers*) | `[TIER-C]` | `vid=250` | **`3521`** | `3869` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x01` (0x01 cutaway) | `0..5` (`0xFF` macro) |
| `Z22` | `cosmic-legacy` (*El legado cósmico* / *The cosmic legacy*) | `[TIER-C]` | `vid=250` | **`3522`** | `3868` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x04` (0x04 cue-ready bits 2..7) | `0..5` (`0xFF` macro) |
| `Z23` | `common-history` (*La existencia como historia común* / *Existence as common history*) | `[TIER-C]` | `vid=250` | **`3523`** | `3869` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x01` (0x01 cutaway) | `0..5` (`0xFF` macro) |
| `Z24` | `final-horizon` (*No vemos más allá del horizonte final.* / *We do not see beyond the final horizon.*) | `[TIER-B]` | `vid=250` | **`3524`** | `3868` | `0` default (`1` dual-perspective cutaway, `2` deep-void telemetry overlay) | `0x07` (0x04 semantic cue mask, 0x01 cutaway, 0x02 dual-hero/lens) | `0..5` (`0xFF` macro) |

#### Spoken Transcript Summary & 3D Visual Blueprint

##### `Z20` `unknown-possibilities` (`vid=3520`) — Posibilidades desconocidas (*Unknown possibilities*) `[TIER-D]`

- **Catalog Metadata:** `scene=20` · `[0.800, 0.840]` · `rgb=[140, 113, 125]` · `sound=(triangle/35Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *INCERTIDUMBRE NO ES UNA PROMESA DE ESCAPE*
- **Metric & Evidence (ES / EN):** `Extrapolación actual + física incompleta → futuros posibles con grados muy distintos de evidencia` / *`Current extrapolation + incomplete physics → possible futures with very different degrees of evidence`* · Evidence: `Incertidumbre responsable · nuestros modelos no son promesas de escape ni descripciones finales garantizadas` (*Responsible uncertainty · our models are not escape promises or guaranteed final descriptions*)
- **Spoken Macro Narrative (ES / EN):** La incertidumbre no demuestra que exista una salida, pero impide tratar nuestros modelos presentes como la última descripción posible. (*EN:* Uncertainty does not prove that there is a way out, but it prevents treating our present models as the last possible description.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` static 3 wire cages + rainbow lotus (`horizon_fade=0.474`, `slow_rate=0.333`); zero false-vacuum bubble nucleation wave, evolving dark-energy expansion curve, cyclic Big Bounce neck, or quantum tunneling halo. **Target (`vid=3520`):** Build a speculative frontier boundary prism projecting uncertain theoretical futures beyond bright observed cone.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **decaimiento del vacío** (*vacuum decay*) | `[TIER-D]` | Una transición a otro estado del vacío cambiaría las leyes efectivas localmente y no ofrece una vía controlable conocida. / *A transition to another vacuum state would change locally effective laws and offers no known controllable pathway.* | **Target:** translucent metastable sea breached by an expanding shimmering crisp bubble converting outer field colors cleanly *(Gap: F0 misses sudden bright phase-transition bubble nucleating inside metastable sky and expanding at lightspeed.)* |
| `F1` | **energía oscura cambiante** (*changing dark energy*) | `[TIER-D]` | Si su comportamiento evoluciona, el destino de la expansión podría diferir de las proyecciones actuales. / *If its behavior evolves, the fate of the expansion could differ from current projections.* | **Target:** cosmic expansion funnel wobbling dynamically between accelerated flare and gentle slowdown *(Gap: F1 misses morphing expansion trumpet bending away from fixed exponential cosmological constant contour.)* |
| `F2` | **física no descubierta** (*undiscovered physics*) | `[TIER-C]` | Nuevas partículas, campos o principios pueden modificar detalles sin garantizar supervivencia eterna. / *New particles, fields or principles can modify details without guaranteeing eternal survival.* | **Target:** translucent phantom geometric ribs blinking subtly between known particle rings *(Gap: No crisp unseen ghost symmetry lattice shimmering faintly outside Standard Model ring.)* |
| `F3` | **cosmologías cíclicas** (*cyclic cosmologies*) | `[TIER-D]` | Algunos modelos proponen ciclos o rebotes, pero no existe evidencia concluyente de continuidad de información o identidad entre ellos. / *Some models propose cycles or rebounds, but there is no conclusive evidence of information continuity or identity between them.* | **Target:** glowing golden double-cone cyclic bounce waist pinching cleanly while identity ribbons scatter *(Gap: F3 misses symmetric Big-Crunch-to-Big-Bounce hourglass neck lacking crisp continuous identity threads.)* |
| `F4` | **transiciones cuánticas** (*quantum transitions*) | `[TIER-D]` | La cosmología cuántica explora posibilidades más allá de la descripción clásica con importantes incertidumbres. / *Quantum cosmology explores possibilities beyond the classical description with significant uncertainties.* | **Target:** shimmering quantum interference wave leaking subtly outside classical dome *(Gap: F4 misses blurred quantum wave-function cloud tunneling probabilistically past solid classical barrier.)* |
| `F5` | **error de extrapolación** (*extrapolation error*) | `[TIER-C]` | Leyes comprobadas en escalas accesibles pueden requerir revisión cuando se extienden a tiempos inconcebiblemente mayores. / *Laws tested on accessible scales may require revision when extended to inconceivably longer times.* | **Target:** razor-sharp glowing calibration line fanning into dashed uncertain nebula cone *(Gap: No dashed theoretical projection curve fraying wildly beyond narrow bright observed strip.)* |

##### `Z21` `observerless-existence` (`vid=3521`) — Existencia sin observadores (*Existence without observers*) `[TIER-C]`

- **Catalog Metadata:** `scene=21` · `[0.840, 0.880]` · `rgb=[151, 106, 116]` · `sound=(sine/32.95Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *LOS PROCESOS CONTINÚAN AUNQUE NADIE LOS VIVA*
- **Metric & Evidence (ES / EN):** `Fin de las mentes + rastros temporales + causalidad física → historia sin recuerdo activo` / *`End of minds + temporal traces + physical causality → history without active memory`* · Evidence: `Retorno final · los procesos físicos continúan aunque ninguna experiencia permanezca` (*Final return - physical processes continue even though no experience remains*)
- **Spoken Macro Narrative (ES / EN):** La app no impone una respuesta a si una historia cambia de significado porque finalmente nadie quede para recordarla. (*EN:* The app does not impose an answer to whether a story changes its meaning because in the end no one is left to remember it.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` still shows bright 25-petal neon lotus + 3 spheres (`horizon_fade=0.447`, `slow_rate=0.30`); fails to extinguish subjective glow over drifting dead ruins and silent cold particles. **Target (`vid=3521`):** Stage a stark dark horizon cross-section contrasting quiet extinguished present above with luminous recorded past worldlines below.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **civilizaciones terminadas** (*civilizations that have ended*) | `[TIER-C]` | Toda perspectiva puede desaparecer aunque sus acciones hayan alterado durante mucho tiempo las condiciones posteriores. / *All perspective can disappear even if your actions have long altered subsequent conditions.* | **Target:** dark geometric city silhouette orbiting a subtly reshaped pale star *(Gap: No extinguished dark civilization citadel drifting calmly while altered starlight passes onward.)* |
| `F1` | **ninguna experiencia** (*no experience*) | `[TIER-C]` | Sin sujetos, ya no existen dolor, esperanza, sorpresa ni memoria vivida, aunque continúe la evolución física. / *Without subjects, there is no longer pain, hope, surprise or vivid memory, even though physical evolution continues.* | **Target:** clear unlit crystal observer shell stripped of emotional flares *(Gap: No dark unlit interior observer shell free of pain/hope sparks.)* |
| `F2` | **registros restantes** (*remaining records*) | `[TIER-D]` | Ruinas y archivos pueden persistir durante un intervalo sin ser leídos ni comprendidos. / *Ruins and files may persist for an interval without being read or understood.* | **Target:** icy drifting stone-and-diamond archival ring floating silent in starlight *(Gap: F2 misses silent abandoned megalith/archive ring drifting cold and unread under dim starlight.)* |
| `F3` | **procesos continuos** (*continuous processes*) | `[TIER-D]` | Partículas, campos y expansión no necesitan observadores para producir consecuencias. / *Particles, fields and expansion do not need observers to produce consequences.* | **Target:** sleek indifferent pale blue quantum field ripples gliding cleanly past empty ruins *(Gap: F3 misses austere cold subatomic fields/wavelets propagating indifferently through black void.)* |
| `F4` | **olvido total** (*total oblivion*) | `[TIER-C]` | Puede llegar un momento en que ninguna estructura contenga una representación recuperable de lo que ocurrió. / *There may come a time when no structure contains a retrievable representation of what occurred.* | **Target:** last readable inscribed ring diffusing gracefully to darkness *(Gap: No last readable structural ripple dissolving cleanly into featureless uniform sea.)* |
| `F5` | **pregunta abierta** (*open question*) | `[TIER-C]` | La desaparición futura de todo recuerdo no retroactúa físicamente para borrar que seres conscientes vivieron y eligieron. / *The future disappearance of all memory does not physically retroact to erase which conscious beings lived and chose.* | **Target:** glowing timeless slice of vivid past consciousness resting intact inside lower spacetime wedge *(Gap: No golden frozen historical light-cone fossil floating untouched in deep past slice beneath black present.)* |

##### `Z22` `cosmic-legacy` (`vid=3522`) — El legado cósmico (*The cosmic legacy*) `[TIER-C]`

- **Catalog Metadata:** `scene=22` · `[0.880, 0.920]` · `rgb=[136, 118, 132]` · `sound=(triangle/30.9Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *UNA CONSECUENCIA REAL NO NECESITA SER ETERNA*
- **Metric & Evidence (ES / EN):** `Descendientes + archivos + ecosistemas + conocimiento + obras → efectos que continúan más allá de sus autores` / *`Descendants + archives + ecosystems + knowledge + works → effects that continue beyond their authors`* · Evidence: `Consecuencia finita · transformar el futuro no exige durar para siempre` (*Finite consequence · transforming the future does not require lasting forever*)
- **Spoken Macro Narrative (ES / EN):** La duración infinita no es una condición necesaria para que una acción transforme realmente el futuro. (*EN:* Infinite duration is not a necessary condition for an action to truly transform the future.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` faded 25-petal lotus + 3 spheres (`horizon_fade=0.421`, `slow_rate=0.267`); lacks engineered transformed star scars, panspermia spore trajectories, or cascading legacy ripples. **Target (`vid=3522`):** Trace radiant outward golden ripple crests launched from a vanishing central spark into thriving outer systems.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **descendientes** (*descendants*) | `[TIER-C]` | Nuevas formas de vida o mente pueden heredar capacidades sin conservar exactamente la identidad de sus creadores. / *New forms of life or mind can inherit capabilities without exactly preserving the identity of their creators.* | **Target:** brief origin spark handing bright torch into diverse unfamiliar crystalline descendants *(Gap: No luminous heritage spark passing baton into radically transformed new polyhedral descendant forms.)* |
| `F1` | **archivos** (*files*) | `[TIER-D]` | Registros distribuidos amplían la memoria histórica aunque cada soporte tenga una vida finita. / *Distributed records expand historical memory although each medium has a finite life.* | **Target:** network of faceted diamond archival capsules echoing across distant star hubs *(Gap: F1 misses distributed redundant archival crystal array glittering across multiple distant hubs.)* |
| `F2` | **ecosistemas protegidos** (*protected ecosystems*) | `[TIER-C]` | Conservar diversidad crea futuros posibles para seres que todavía no existen. / *Conserving diversity creates possible futures for beings that do not yet exist.* | **Target:** thriving verdant multi-colored reef halo flourishing long after founders vanish *(Gap: No thriving emerald biosphere bloom expanding inside safe future boundary.)* |
| `F3` | **estrellas transformadas** (*transformed stars*) | `[TIER-D]` | Ingeniería de largo plazo puede dejar huellas físicas que sobrevivan a culturas y lenguas. / *Long-term engineering can leave physical traces that survive cultures and languages.* | **Target:** subtly engineered pulsing star halo with geometric megastructure orbital scars *(Gap: F3 misses unmistakable re-engineered binary/metal-enriched star ring beaming visible artificial signature across eons.)* |
| `F4` | **arte y conocimiento** (*art and knowledge*) | `[TIER-C]` | Una obra o explicación puede cambiar experiencias y decisiones aun cuando finalmente se pierda. / *A work or explanation can change experiences and decisions even if it is ultimately lost.* | **Target:** golden harmonic chord wave lighting remote minds *(Gap: No resonant golden chord propagating art/knowledge waves through downstream nodes.)* |
| `F5` | **vida sembrada** (*sown life*) | `[TIER-D]` | Introducir vida en nuevos entornos prolonga una historia biológica y exige considerar daños, autonomía y contaminación. / *Introducing life into new environments prolongs a biological history and requires consideration of damage, autonomy, and contamination.* | **Target:** delicate glowing panspermia capsule trajectory blooming gently over sterile ocean world *(Gap: F5 misses directed panspermia spore arc carefully seeding bare ocean moon with ethical boundary shield.)* |

##### `Z23` `common-history` (`vid=3523`) — La existencia como historia común (*Existence as common history*) `[TIER-C]`

- **Catalog Metadata:** `scene=23` · `[0.920, 0.960]` · `rgb=[172, 164, 158]` · `sound=(sawtooth/28.85Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `[none — add bilingual tokens]` · Footer: *UNA SOLA HISTORIA · MUCHOS NIVELES · LÍMITES DESDE DENTRO*
- **Metric & Evidence (ES / EN):** `Materia → vida → experiencia → memoria → comprensión → elección → transformación → pregunta` / *`Matter → life → experience → memory → understanding → choice → transformation → question`* · Evidence: `Síntesis · cosmos, vida, mente y civilización son niveles de un mismo proceso desplegado` (*Synthesis · cosmos, life, mind and civilization are levels of the same unfolding process*)
- **Spoken Macro Narrative (ES / EN):** Las aplicaciones no narran historias separadas, sino una secuencia en la que el universo produce regiones capaces de examinar sus propios límites. (*EN:* The applications do not tell separate stories, but rather a sequence in which the universe produces regions capable of examining their own limits.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` 3 wire spheres + 25-petal lotus (`horizon_fade=0.395`, `slow_rate=0.233`); flat 25-petal ring hints at 25 chapters only abstractly, missing the 6-tier cumulative emergence spiral from stars -> planets -> life -> mind -> civilization -> limits. **Target (`vid=3523`):** Assemble a grand 6-stratum ascending cosmic Ouroboros spiral linking Acts 1-5 vertically inside one crystal horizon dome.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **cosmos y estrellas** (*cosmos and stars*) | `[TIER-D]` | La expansión, la gravedad y la nucleosíntesis construyen los escenarios y elementos de toda historia posterior. / *Expansion, gravity and nucleosynthesis build the settings and elements of every subsequent story.* | **Target:** golden nucleosynthesis supernova nebula swirls at base *(Gap: F0 misses concrete expansion/gravity/nucleosynthesis crucible igniting heavy-element nebula dust.)* |
| `F1` | **planetas y materia** (*planets and matter*) | `[TIER-D]` | Estructuras físicas concentran energía y química hasta permitir entornos estables y complejos. / *Physical structures concentrate energy and chemistry to allow stable and complex environments.* | **Target:** banded pale blue rocky planet condensing cleanly *(Gap: F1 misses rocky planet condensation disk concentrating chemical energy & ocean stability.)* |
| `F2` | **vida** (*life*) | `[TIER-D]` | Sistemas capaces de mantenerse y evolucionar convierten condiciones físicas en oportunidades y amenazas funcionales. / *Systems capable of maintaining and evolving convert physical conditions into functional opportunities and threats.* | **Target:** emerald mycelial/cellular biosphere spiral wrapping planet *(Gap: F2 misses cellular living membrane & evolutionary branching blossoming over planetary cradle.)* |
| `F3` | **mente** (*mind*) | `[TIER-C]` | La experiencia, la memoria y la representación producen perspectivas para las que algo puede importar. / *Experience, memory, and representation produce perspectives for which something can matter.* | **Target:** luminous iridescent neural corona blooming above biosphere *(Gap: No luminous sentient nervous crown blooming atop living ecosystem orb.)* |
| `F4` | **cultura y tecnología** (*culture and technology*) | `[TIER-C]` | Conocimiento compartido permite que decisiones acumuladas transformen cuerpos, sociedades y mundos. / *Shared knowledge allows accumulated decisions to transform bodies, societies and worlds.* | **Target:** delicate golden civilization astrolabe & Dyson filigree spanning zenith *(Gap: No golden technological astrolabe/ring canopy spanning globe.)* |
| `F5` | **límites** (*limits*) | `[TIER-C]` | La capacidad de comprender crece dentro de fronteras físicas, lógicas, temporales y conceptuales que nunca observamos desde fuera. / *The ability to understand grows within physical, logical, temporal and conceptual boundaries that we never observe from the outside.* | **Target:** translucent exterior horizon dome sealing softly around viewer from inside without outside vantage point *(Gap: No sudden overarching semi-transparent horizon dome framing all 6 strata *strictly from inside*.)* |

##### `Z24` `final-horizon` (`vid=3524`) — No vemos más allá del horizonte final. (*We do not see beyond the final horizon.*) `[TIER-B]`

- **Catalog Metadata:** `scene=24` · `[0.960, 1.000]` · `rgb=[232, 228, 214]` · `sound=(triangle/26.8Hz)` · `speak_evidence_default=True` · `DOMAIN_CUE_KEYWORDS`: `fusion`, `horizonte` · Footer: *LA EXISTENCIA PERMANECE INCOMPLETA PARA TODA PERSPECTIVA INTERIOR*
- **Metric & Evidence (ES / EN):** `Evidencia disponible → afirmación responsable \| más allá → posibilidad, especulación o inconcebible` / *`Available evidence → responsible claim \| beyond → possibility, speculation or inconceivable`* · Evidence: `El horizonte final · límite simultáneo de observación, predicción, cómputo, identidad, supervivencia, lenguaje y física` (*The final horizon · simultaneous limit of observation, prediction, computation, identity, survival, language and physics*)
- **Spoken Macro Narrative (ES / EN):** Esto no demuestra que no exista nada más. Solo marca el límite entre aquello que podemos describir con evidencia y aquello que todavía pertenece a la posibilidad, la especulación o lo inconcebible. (*EN:* This does not prove that nothing else exists. It only marks the limit between what we can describe with evidence and what still belongs to possibility, speculation or the inconceivable.)
- **Current Diagnosis & Target 3D Blueprint:** legacy `vid=250` literally models 'Nested Horizon Spheres + 25-Petal Lotus' at maximum fade (`horizon_fade=0.368`, `slow_rate=0.20`, highlighting lotus petal `24/25`), matching the zone's exact centerpiece (`horizonte`), making it `TIER-B`, yet `doc_feat=0..5` merely twists sphere longitude by 17.2 deg (`f32(feat)*0.3`) and leaves `doc_cue`/`zone_col` dead! **Target (`vid=3524`):** Keep legacy `vid=250`'s iconic 3 Nested Horizon Spheres (`r=0.24, 0.44, 0.64`) + 25-Petal Finale Lotus while giving each horizon shell distinct meaning across `doc_feat=0..5`.

| Slot | Feature (`ES / EN`) | Tier | Spoken Narrative (`ES` / `EN`) | `doc_feat` 3D Choreography & Cue Action |
|:---:|---|:---:|---|---|
| `F0` | **observación** (*observation*) | `[TIER-B]` | No podemos describir con evidencia acontecimientos de los que ninguna señal puede llegar a una perspectiva posible. / *We cannot describe with evidence events from which no signal can reach any possible observer.* | **Target:** outermost sphere seals into luminous one-way causal horizon mirror halting external photon streaks *(Gap: F0 causal observation limit only twists wire spheres 0 deg instead of sealing outer light-cone veil (`kw:horizonte`).)* |
| `F1` | **predicción** (*prediction*) | `[TIER-B]` | Modelos finitos pierden precisión cuando dependen de condiciones inaccesibles, caos o física todavía desconocida. / *Finite models lose precision when they depend on inaccessible conditions, chaos, or still unknown physics.* | **Target:** crisp trajectories disperse cleanly into dashed chaotic probability haze at middle shell *(Gap: F1 prediction limit only twists wire spheres 17 deg instead of fraying model tracks beyond horizon.)* |
| `F2` | **computación** (*computing*) | `[TIER-B]` | Algunas respuestas requieren recursos mayores que los disponibles en el universo accesible o no admiten un procedimiento general. / *Some responses require resources greater than those available in the accessible universe or do not support a general procedure.* | **Target:** innermost compute grid caps bit-density smoothly against outer entropy ceiling *(Gap: F2 computing ceiling only twists wire spheres 34 deg instead of illuminating finite compute shell.)* |
| `F3` | **identidad** (*identity*) `[kw:fusion]` | `[TIER-B]` | No poseemos un criterio externo definitivo que resuelva toda copia, fusión, interrupción o transformación de una mente. / *We do not have a definitive external criterion that resolves every copy, fusion, interruption or transformation of a mind.* | **Target:** glowing twin-core center pulses softly between split/merged superpositions without fixed border *(Gap: F3 identity ambiguity only twists wire spheres 52 deg instead of pulsing double split core (`kw:fusion` in F3!).)* |
| `F4` | **supervivencia** (*survival*) | `[TIER-B]` | Una civilización puede prolongar su historia enormemente sin demostrar que toda experiencia pueda continuar para siempre. / *A civilization can prolong its history enormously without demonstrating that all experience can continue forever.* | **Target:** ember ring pulses slowly at `0.20x` beat fading respectfully at cosmic rim *(Gap: F4 survival ceiling only twists wire spheres 69 deg instead of dimming thermodynamic embers cleanly.)* |
| `F5` | **perspectiva interior** (*inner perspective*) | `[TIER-B]` | La existencia permanece incompleta para toda perspectiva que intenta comprenderla desde dentro. / *Existence remains incomplete for any perspective that attempts to understand it from within.* | **Target:** camera-facing interior aperture illuminates all 25 lotus petals simultaneously like a quiet jewel box opening onto deep serene OLED infinity (`act5_ouroboros_earth_return` bridge) *(Gap: F5 interior perspective only twists wire spheres 86 deg instead of glowing softly from within while framing vast unseeable beyond.)* |

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-ACT5-EXP25-Z20-24)</summary>

```markdown
Implement Work Package `PKG-ACT5-EXP25-Z20-24` (Unknown Frontiers, Witnessless Matter, Cosmic Legacy, Common History & Finale Lotus Aperture) for HyperCosmosVR Odyssey Act V (`Exp 25: 25_final_horizon`).

1. Prerequisites & Touched Files (stay strictly scoped to these 4 hot-path areas):
   - Confirm `PKG-P0-DISPATCH-AND-IDS` (`ch_exp_and_sub` `vid >= 1000u` decoding) and `PKG-P0-FEATURE-AND-CUE-PLUMBING` (`cue_tags` -> `base_flags`, `doc_cue`, `doc_weight`).
   - Rust dispatch & companions: `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_doc_stage_hero`, `pack_doc_tags`).
   - Bilingual semantic cues (ES/EN/FI): `crates/cosmos_narration/src/resolver.rs` (`DOMAIN_CUE_KEYWORDS`, `VisualCueTag`, `populate_segment_visual_hints`) — and preserve `cue_tags` in `crates/cosmos_app/src/engine.rs`.
   - Hero WGSL raymarching: `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub`, `march_doc_*`, `raymarch_doc_civilization_horizon`).
   - Particle/field coupling: `shaders/compute_universe.wgsl` + `crates/cosmos_render/src/universe_scene.rs` (`select_compute_mode`, `DocumentaryFocusState`).

2. Exact IDs, Companions & Sub-Staging Requirements:
   - Route zones to reserved primary IDs: `Z20`->`3520` (`F0..F5`), `Z21`->`3521` (`F0..F5`), `Z22`->`3522` (`F0..F5`), `Z23`->`3523` (`F0..F5`), `Z24`->`3524` (`F0..F5`) (`submode = 0` macro at `doc_feat = 0xFF`, `1..2` cutaway/counterfactual/field overlays).
   - Companion `HeroVolumeInstance` IDs (when dual-hero/lens/orb surfaces apply): `3868`, `3869`.
   - Wire every spoken feature `F0..Fk` (`doc_feat(inst)`) to the exact 3D visual transitions specified in `ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md` (`PKG-ACT5-EXP25-Z20-24`), modulated smoothly (`0.72..1.35x` clamped HDR, zero flashing/strobe) by `doc_cue(inst)` and `doc_weight(inst)`.
   - Bilingual keyword triggers (ES/EN/FI folded ASCII tokens): cover Z20: unknown-possibilities / Unknown possibilities, Z21: observerless-existence / Existence without observers, Z22: cosmic-legacy / The cosmic legacy, Z23: common-history / Existence as common history, Z24: final-horizon / We do not see beyond the final horizon.

3. Mandatory Rendering, Performance & Hygiene Guardrails:
   - Pure-Black OLED Void: preserve >= 50.91% exact `#000000` background pixels and `0.0000%` highlight blowout; integrate only inside bounded local shells (`r <= 0.94..0.96`), never emit full-screen fog wash.
   - VR 90/120 Hz Budget: keep new documentary raymarch loops at <= 28 steps with `trans < 0.03` early exit, preserving the RTX 5070 DX12 sweep baseline (`mean = 1.927 ms`, `p99 = 4.873 ms` at `1600x900` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - Shader Contract: comply with `shaders/SHADER_CONTRACTS.md` (112 B `HeroVolumeInstance`, Naga 30.0.2 + DX12 HLSL SM 6.5 clean).
   - Repo Hygiene (`CONTRIBUTING.md`): zero binary files > 50 MiB, zero `.ttf`/`.otf` fonts, zero machine-specific `.cargo/config.toml` or local paths.

4. Required Verification Commands Before Opening PR:
   - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey`
   - `cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe`
   - `cargo test -p cosmos_spatial topology`
   - `cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
   - `cargo test --workspace -- --test-threads=1` (local full suite including GPU shader compilation & verification)
```
</details>

---
