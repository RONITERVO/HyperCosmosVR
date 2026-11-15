# HyperCosmosVR Visual Expansion Contributor Roadmap (`01..25`)

> **Headline Audit & Live Reference Progress:** Across all **25 experiences**, **376 documentary zones**, **2 interactive spatial laboratories** (`01_colors`, `02_music`), and **2,456 spoken features** (`18,898` baked catalog audio assets), HyperCosmosVR has completed **`2 / 3` Core Pipeline P0 packages** (`PKG-P0-DISPATCH-AND-IDS` & `PKG-P0-FEATURE-AND-CUE-PLUMBING` + core runtime/test pieces of `PKG-P0-CATALOG-SCENE-AND-CI`) and **`21 / 85` Content Packages** (`PKG-ACT1-EXP03-Z00-04`, `Z05-08`, `Z09-13` + `PKG-ACT1-EXP04-Z00-03`, `Z04-07`, `Z08-11` = **`100%` of Odyssey Act I** [`03_deep_earth` + `04_solar_system`], plus `PKG-ACT2-EXP05-Z00-04`, `Z05-08`, `Z09-12`, `PKG-ACT2-EXP06-Z00-04`, `Z05-08`, `PKG-ACT2-EXP07-Z00-04`, `Z05-09`, `PKG-ACT2-EXP08-Z00-03`, `Z04-07`, `Z08-10`, `PKG-ACT2-EXP09-Z00-03`, `Z04-07`, `Z08-10`, and `PKG-ACT2-EXP10-Z00-04`, `Z05-08` = **`100%` of `Experiences 03..=10`: `05_milky_way`, `06_local_group`, `07_laniakea`, `08_cosmic_web`, `09_observable_universe`, `10_early_universe` & all `32 / 32` Act II landmarks** at `[TIER-A]`, covering `89 / 376` zones and `446 / 2,456` features):
> - **Zones (`376`):** `89` Tier-A (`23.7%`) · `57` Tier-B (`15.2%`) · `121` Tier-C (`32.2%`) · `109` Tier-D (`29.0%`)
> - **Features (`2,456`):** `446` Tier-A (`18.2%`) · `277` Tier-B (`11.3%`) · `794` Tier-C (`32.3%`) · `939` Tier-D (`38.2%`)
> - **Spatial Labs (`01, 02`):** `[TIER-B]` (`01_colors`: `130` keys / `72` lattice nodes; `02_music`: `48x4` filaments / `20` keys; `field_grid_tex` Chladni coupling active)
> - **Claimable Scope:** **`88` Total Work Packages** = **`3` Core Pipeline P0 Packages** (status below) + **`85` Content Packages** (`21` `[TIER-A]` complete = `100%` of `Experiences 03..=10` [Odyssey Act I + `05..=10` in Act II] + `64` open for contributors) detailed across **5 Act Gap Maps** in [`docs/roadmap/`](roadmap/).

### For Non-Technical Evaluators, Team Leads & Independent Fact-Checkers (30-Second Brief)

- **Important Product Context ("Does the app work today?"):** **Yes.** All `25` experiences (`376` zones, `2,456` spoken topics, `18,898` audio assets, `01_colors`, `02_music`, Desktop HDR, OpenXR VR, and the full 5-act `grand_cosmic_odyssey` tour) run and render in real time today (`RTX 5070` `1600×900` baseline: `min = 0.837 ms`, `mean = 1.974 ms / 506.7 FPS`, `p99 = 5.109 ms`, `max = 5.188 ms`). Today **`89 / 376` zones (`446 / 2,456` features across `100%` of `Experiences 03..=10`: `03_deep_earth` via [`PKG-ACT1-EXP03-Z00-04`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp03-z00-04), [`Z05-08`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp03-z05-08), [`Z09-13`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp03-z09-13) and `04_solar_system` via [`PKG-ACT1-EXP04-Z00-03`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp04-z00-03), [`Z04-07`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp04-z04-07), [`Z08-11`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp04-z08-11), plus `100%` of `05_milky_way`, `06_local_group`, `07_laniakea`, `08_cosmic_web`, `09_observable_universe`, and `10_early_universe` via [`PKG-ACT2-EXP05-Z00-04`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp05-z00-04), [`Z05-08`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp05-z05-08), [`Z09-12`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp05-z09-12), [`PKG-ACT2-EXP06-Z00-04`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp06-z00-04), [`Z05-08`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp06-z05-08), [`PKG-ACT2-EXP07-Z00-04`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp07-z00-04), [`Z05-09`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp07-z05-09), [`PKG-ACT2-EXP08-Z00-03`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp08-z00-03), [`Z04-07`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp08-z04-07), [`Z08-10`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp08-z08-10), [`PKG-ACT2-EXP09-Z00-03`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp09-z00-03), [`Z04-07`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp09-z04-07), [`Z08-10`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp09-z08-10), [`PKG-ACT2-EXP10-Z00-04`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp10-z00-04), and [`Z05-08`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp10-z05-08))** meet our strict **`[TIER-A]` 1:1 muted / language-free comprehension** bar — where every individual spoken sentence (`doc_feat`) animates its exact bespoke 3D structure rather than relying on shared domain shaders or cycling modulo fallbacks (`sc % N`).
- **Verify Core Numbers Yourself Without AI (10 Seconds):**
  ```powershell
  # 1. Confirm 25 experiences, 376 zones, 2,456 features, 18,898 audio assets straight from the baked catalog:
  python -c "import json; d=json.load(open('assets/baked/universe_catalog.v1.json',encoding='utf-8')); z=[z for e in d['experiences'] for z in e['zones']]; f=[ft for x in z for ft in x['features']]; print(len(d['experiences']), len(z), len(f), len(d['audio_assets']))"
  # 2. Confirm all 88 unique claimable package anchors exist in docs/:
  (Select-String -Path docs/CONTRIBUTOR_VISUAL_ROADMAP.md, docs/roadmap/ACT_*.md -Pattern '<a id="pkg-[^"]+">').Count
  # 3. Inspect checked-in RTX 5070 frame timings & run CPU unit/catalog verification:
  Get-Content artifacts/verification_renders/benchmark_sweep.txt
  cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets
  ```
- **Threshold Note:** Existing `43` baseline PNGs hold `>= 50.90%` `#000000` (`50.9004%` minimum on `06_cmb_and_inflation.png` per `README.md`); newly authored or upgraded zone packages in this roadmap enforce the stricter **`>= 50.91%` `#000000`** PR acceptance floor (`87/88` cards carry the `<= 28` raymarch step gate; `PKG-P0-CATALOG-SCENE-AND-CI` is catalog/test-only).

*(Technical grounding note: All audit classifications in this roadmap inspect real runtime Rust and WGSL execution in `assets/baked/universe_catalog.v1.json`, `crates/cosmos_render/src/doc_stages.rs`, `crates/cosmos_render/src/universe_scene.rs`, `crates/cosmos_app/src/engine.rs`, `crates/cosmos_narration/src/resolver.rs`, `crates/cosmos_journey/src/odyssey.rs`, `crates/cosmos_spatial/src/topology.rs`, and `shaders/*.wgsl`. Do not treat aspirational prose in [`docs/documentary_visuals_spec.md`](documentary_visuals_spec.md) as already implemented.)*

---

## Audit Tier Rubric

| Tier | Classification | Strict Engineering Criterion |
|---|---|---|
| `[TIER-A]` | **Bespoke 1:1 Hero Stage (Complete)** | Dedicated zero-conflict `vid` (`1000 + exp*100 + zone`) AND distinct `doc_feat(inst)` (`0..=7`) WGSL visual responses, cross-sections, callouts, or phase transitions matching every spoken subject. |
| `[TIER-B]` | **Shared Archetype (Needs Feature Sub-Staging)** | Domain shader/body visually matches the zone's topic, OR a bespoke zone stage ignores `doc_feat` / pulses uniformly without distinct per-feature visual choreography. |
| `[TIER-C]` | **Generic Modulo Fallback (Needs Bespoke Zone Stage)** | Routed through `sc % N`, `zone_index % 3`, or a single collapsed experience variant whose visual metaphor mismatches the zone's spoken narrative. |
| `[TIER-D]` | **Missing Visual Metaphor (High Priority)** | Narration describes a concrete physical experiment, structure, biological process, social institution, or astrophysical concept with no visual counterpart on screen. |

---

## Odyssey Travel Order (`ODYSSEY_ACTS`) vs. Catalog Experience IDs (`01..25`)

Contributors new to HyperCosmosVR often expect playback to follow numeric catalog order `01 -> 25`. In runtime execution, `GrandCosmicOdysseyPlugin` traverses **`ODYSSEY_ACTS`** ([`crates/cosmos_journey/src/odyssey.rs#L24-L45`](../crates/cosmos_journey/src/odyssey.rs#L24-L45)) along the scale-continuity graph defined in `continuity_edges()` ([`crates/cosmos_spatial/src/topology.rs#L406-L434`](../crates/cosmos_spatial/src/topology.rs#L406-L434)):

1. **Act I — Earth & Solar System (`03 -> 04`)** · `26` zones, `130` features (`6` packages) — [`ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md)
   Begins in Earth atmosphere/interior (`03_deep_earth`) and rises outward (`ScaleRiseOutward`) across the Solar System (`04_solar_system`).
2. **Act II — Cosmic Web & Origins (`05 -> 21 -> 22 -> 06 -> 07 -> 08 -> 09 -> 10`)** · `116` zones, `684` features (`26` packages) — [`ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md)
   Moves outward into `05_milky_way`, branches into `EmbeddedStellarArm` exoplanet life & contact (`21_cosmic_life -> 22_cosmic_contact`), then continues outward from Local Group (`06`) through Laniakea (`07`), Cosmic Web (`08`), Observable Universe (`09`), to Early Universe (`10`).
3. **Act III — Matter & Spacetime (`13 -> 14`)** · `27` zones, `187` features (`7` packages) — [`ACT_3_MATTER_AND_SPACETIME_MAP.md`](roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md)
   Dives inward (`ScaleDiveInward`) from `03_deep_earth` into Matter & Energy (`13_matter_energy`) and fundamental Spacetime (`14_spacetime`).
4. **Act IV — Life, Mind & Senses (`15 -> 16 -> 17 -> 01 -> 02`)** · `53` zones, `417` features + `01, 02` (`14` packages) — [`ACT_4_LIFE_MIND_AND_SENSES_MAP.md`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md)
   Ascends the `EmergenceHelix` (`15_complexity -> 16_life -> 17_mind`) and nests into sensory laboratories `01_colors` (`130` keys / `72` lattice nodes) and `02_music` (`48x4` filaments / `20` keys) via `NestedSensoryLobe`.
5. **Act V — Civilization & Horizons (`18 -> 19 -> 20 -> 11 -> 12 -> 23 -> 24 -> 25`)** · `154` zones, `1,038` features (`32` packages) — [`ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md)
   Continues `ScaleRiseOutward` through `18_culture -> 19_technology -> 20_futures`, then visits the five `HyperHorizonLotusPetal` wings radiating from `10` (`11_cosmic_evidence`, `12_future_of_universe`, `23_cosmic_perspectives`, `24_cosmic_meaning`, `25_final_horizon`), closing back to `13_matter_energy` via `OuroborosReturn`.

**Catalog Metadata Caveats Found During Audit (Resolved in [`PKG-P0-CATALOG-SCENE-AND-CI`](#pkg-p0-catalog-scene-and-ci)):**
- `188 / 376` zones (`03..=17`) have `scene: null` in `universe_catalog.v1.json`, and `89 / 376` zones (`03..=10`) have `evidence_class: null`.
- `22_cosmic_contact` shares identical `scene_index` values across `5` zone pairs (`10` zones: `(0,1) sc=0`, `(9,10) sc=8`, `(15,16) sc=13`, `(17,18) sc=14`, `(19,20) sc=15`).
- `24_cosmic_meaning` carries `"Significado Cósmico"` in `universe_catalog.v1.json` vs `"Sentido Cósmico"` in [`crates/cosmos_spatial/src/topology.rs#L379`](../crates/cosmos_spatial/src/topology.rs#L379).

---

## Executive Summary Scorecard

### Per-Act Rollup (`5` Odyssey Acts + `3` P0 = `88` Packages)

| Odyssey Act | Experiences (`ODYSSEY_ACTS`) | Zones | Zone `[A / B / C / D]` | Feats | Feat `[A / B / C / D]` | Content Pkgs | Gap Map Deliverable |
|---|---|---:|:---:|---:|:---:|---:|---|
| **Act I** | `03, 04` | `26` | `26 / 0 / 0 / 0` | `130` | `130 / 0 / 0 / 0` | `6` *(6 done = `100%`)* | [`ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md) |
| **Act II** | `05, 21, 22, 06, 07, 08, 09, 10` | `116` | `63 / 9 / 19 / 25` | `684` | `316 / 21 / 120 / 227` | `26` *(15 done = `100%` of `05, 06, 07, 08, 09, 10`)* | [`ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md) |
| **Act III** | `13, 14` | `27` | `0 / 6 / 21 / 0` | `187` | `0 / 22 / 146 / 19` | `7` | [`ACT_3_MATTER_AND_SPACETIME_MAP.md`](roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md) |
| **Act IV** | `15, 16, 17, 01, 02` | `53` *(+01, 02)* | `0 / 20 / 19 / 14` | `417` | `0 / 138 / 125 / 154` | `14` | [`ACT_4_LIFE_MIND_AND_SENSES_MAP.md`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md) |
| **Act V** | `18, 19, 20, 11, 12, 23, 24, 25` | `154` | `0 / 22 / 62 / 70` | `1,038` | `0 / 96 / 403 / 539` | `32` | [`ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md) |
| **Core P0** | Pipeline Infrastructure | — | — | — | — | `3` | Below ([Core Pipeline P0 Packages](#core-pipeline-p0-packages)) |
| **TOTAL** | **`01..25`** (`5` Acts + P0) | **`376`** | **`89 / 57 / 121 / 109`** | **`2,456`** | **`446 / 277 / 794 / 939`** | **`88`** | **`3` P0 + `85` Content (`21` Tier-A live = `Experiences 03..=10`)** |

### All-25 Experiences Ground-Truth Table

| Exp | Slug | Catalog Title (ES) | Odyssey Act | Act Map | Zones | Zone `[A / B / C / D]` | Feats | Feat `[A / B / C / D]` | Current `doc_stage_variant_id` / Dispatch | Reserved `vid` Block |
|---:|---|---|---|---|---:|:---:|---:|:---:|---|---|
| `01` | `01_colors` | Espectro de colores | Act 4 (`4/5`) | [`ACT_4`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md) | 0 (spatial) | `[TIER-B]` spatial (`130` keys) | 0 | — | `kind=5, vid=1/10`; `kind=1, vid=12` | `1100..=1199` (comp `3600..=3609`) |
| `02` | `02_music` | Espectro musical | Act 4 (`5/5`) | [`ACT_4`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md) | 0 (spatial) | `[TIER-B]` spatial (`48x4` filaments, `20` keys) | 0 | — | `kind=6, vid=1, 11..14` + `field_grid_tex` Chladni coupling | `1200..=1299` (comp `3610..=3619`) |
| `03` | `03_deep_earth` | Tierra profunda | Act 1 (`1/2`) | [`ACT_1`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md) | 14 | `14 / 0 / 0 / 0` | 70 | `70 / 0 / 0 / 0` | All `Z00..=Z13` migrated `[TIER-A]` (`1300..=1313`, `Z03` mesosphere & cutaway/macro fixes) | `1300..=1313` (comp `3620..=3629`) |
| `04` | `04_solar_system` | Sistema Solar | Act 1 (`2/2`) | [`ACT_1`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md) | 12 | `12 / 0 / 0 / 0` | 60 | `60 / 0 / 0 / 0` | All `Z00..=Z11` migrated `[TIER-A]` (`1400..=1411`, comp `3640..=3651`, Kirkwood/Plutino, cryo-plume/comet, 60 feats) | `1400..=1411` (comp `3640..=3651`) |
| `05` | `05_milky_way` | Vía Láctea | Act 2 (`1/8`) | [`ACT_2`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md) | 13 | `13 / 0 / 0 / 0` | 65 | `65 / 0 / 0 / 0` | All `Z00..=Z12` migrated `[TIER-A]` (`1500..=1512`, comp `3660..=3671` + 13th `Z12` landmark, 65 feats) | `1500..=1512` (comp `3660..=3671`) |
| `06` | `06_local_group` | Grupo Local | Act 2 (`4/8`) | [`ACT_2`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md) | 9 | `9 / 0 / 0 / 0` | 45 | `45 / 0 / 0 / 0` | All `Z00..=Z08` migrated `[TIER-A]` (`1600..=1608`, comp `3680..=3688`, 9 landmarks, 45 feats) | `1600..=1608` (comp `3680..=3688`) |
| `07` | `07_laniakea` | Laniakea | Act 2 (`5/8`) | [`ACT_2`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md) | 10 | `10 / 0 / 0 / 0` | 50 | `50 / 0 / 0 / 0` | All `Z00..=Z09` migrated `[TIER-A]` (`1700..=1709`, comp `3700..=3709`, 10 landmarks, 50 feats) | `1700..=1709` (comp `3700..=3709`) |
| `08` | `08_cosmic_web` | Red Cósmica | Act 2 (`6/8`) | [`ACT_2`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md) | 11 | `11 / 0 / 0 / 0` | 55 | `55 / 0 / 0 / 0` | All `Z00..=Z10` migrated `[TIER-A]` (`1800..=1810`, Mode 2 + `cs_field_grid` cues, 55 feats) | `1800..=1810` |
| `09` | `09_observable_universe` | Universo Observable | Act 2 (`7/8`) | [`ACT_2`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md) | 11 | `11 / 0 / 0 / 0` | 55 | `55 / 0 / 0 / 0` | All `Z00..=Z10` migrated `[TIER-A]` (`1900..=1910`, epoch-gated Mode 2/13/5 + CMB/star isolation, 55 feats) | `1900..=1910` |
| `10` | `10_early_universe` | Universo Temprano | Act 2 (`8/8`) | [`ACT_2`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md) | 9 | `9 / 0 / 0 / 0` | 46 | `46 / 0 / 0 / 0` | All `Z00..=Z08` migrated `[TIER-A]` (`2000..=2008`, Mode 13/3/5 + pre-stellar far-shell isolation, 46 feats) | `2000..=2008` |
| `11` | `11_cosmic_evidence` | Evidencia Cósmica | Act 5 (`4/8`) | [`ACT_5`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md) | 9 | `0 / 1 / 5 / 3` | 50 | `0 / 1 / 22 / 27` | `kind=9, vid=110 + (sc % 4)` | `2100..=2108` (comp `3720..=3729`) |
| `12` | `12_future_of_universe` | Futuro del Universo | Act 5 (`5/8`) | [`ACT_5`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md) | 10 | `0 / 3 / 4 / 3` | 59 | `0 / 10 / 21 / 28` | Un-bypassed `kind=9, vid=120..=124` + Hawking BH comp `3730` on `Z05..=Z07` | `2200..=2209` (comp `3730..=3739`) |
| `13` | `13_matter_energy` | Materia y Energía | Act 3 (`1/2`) | [`ACT_3`](roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md) | 12 | `0 / 3 / 9 / 0` | 73 | `0 / 13 / 55 / 5` | `kind=7`: `{2->10, 3->11, 6->12, _->10 + (sc % 5)}` | `2300..=2311` (comp `3740..=3749`) |
| `14` | `14_spacetime` | Espacio-Tiempo | Act 3 (`2/2`) | [`ACT_3`](roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md) | 15 | `0 / 3 / 12 / 0` | 114 | `0 / 9 / 91 / 14` | `kind=7`: `{8->22, 11->23, _->20 + (sc % 4)}` | `2400..=2414` (comp `3750..=3759`) |
| `15` | `15_complexity` | Complejidad | Act 4 (`1/5`) | [`ACT_4`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md) | 16 | `0 / 3 / 8 / 5` | 119 | `0 / 19 / 55 / 45` | `kind=8, vid=10 + (sc % 4)` (`field_grid_tex` unread) | `2500..=2515` (comp `3760..=3769`) |
| `16` | `16_life` | Vida | Act 4 (`2/5`) | [`ACT_4`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md) | 19 | `0 / 3 / 11 / 5` | 153 | `0 / 15 / 69 / 69` | `kind=8`: `{0->23, 4->20, _->20 + (sc % 5)}` | `2600..=2618` (comp `3770..=3779`) |
| `17` | `17_mind` | Mente | Act 4 (`3/5`) | [`ACT_4`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md) | 18 | `0 / 14 / 0 / 4` | 145 | `0 / 104 / 1 / 40` | `kind=8, vid=30 + (sc % 5)` | `2700..=2717` (comp `3780..=3789`) |
| `18` | `18_culture` | Cultura | Act 5 (`1/8`) | [`ACT_5`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md) | 20 | `0 / 1 / 12 / 7` | 160 | `0 / 7 / 81 / 72` | `kind=9, vid=180 + (sc % 7)` | `2800..=2819` (comp `3790..=3799`) |
| `19` | `19_technology` | Tecnología | Act 5 (`2/8`) | [`ACT_5`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md) | 20 | `0 / 4 / 6 / 10` | 161 | `0 / 28 / 33 / 100` | `kind=9`: `{0->190, 16->194, _->190 + (sc % 7)}` | `2900..=2919` (comp `3800..=3809`) |
| `20` | `20_futures` | Futuros | Act 5 (`3/8`) | [`ACT_5`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md) | 19 | `0 / 2 / 6 / 11` | 152 | `0 / 4 / 52 / 96` | `kind=9`: `{0->200, 16->202, _->200 + (sc % 4)}` | `3000..=3018` (comp `3810..=3819`) |
| `21` | `21_cosmic_life` | Vida Cósmica | Act 2 (`2/8`) | [`ACT_2`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md) | 25 | `0 / 4 / 4 / 17` | 200 | `0 / 10 / 32 / 158` | `kind=9`: `{12->212, _->210}` + companion globes (`11/10`) | `3100..=3124` (comp `3820..=3829`) |
| `22` | `22_cosmic_contact` | Contacto Cósmico | Act 2 (`3/8`) | [`ACT_2`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md) | 28 | `0 / 5 / 15 / 8` | 168 | `0 / 11 / 88 / 69` | `kind=9`: `{12->221, _->220}` (`5` shared `scene_index` pairs) | `3200..=3227` (comp `3830..=3839`) |
| `23` | `23_cosmic_perspectives` | Perspectivas Cósmicas | Act 5 (`6/8`) | [`ACT_5`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md) | 25 | `0 / 4 / 8 / 13` | 150 | `0 / 11 / 60 / 79` | `kind=9, vid=230` (single variant) | `3300..=3324` (comp `3840..=3849`) |
| `24` | `24_cosmic_meaning` | Significado Cósmico | Act 5 (`7/8`) | [`ACT_5`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md) | 26 | `0 / 6 / 15 / 5` | 156 | `0 / 29 / 96 / 31` | `kind=9`: `{24->241, _->240}` | `3400..=3425` (comp `3850..=3859`) |
| `25` | `25_final_horizon` | Horizonte Final | Act 5 (`8/8`) | [`ACT_5`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md) | 25 | `0 / 1 / 6 / 18` | 150 | `0 / 6 / 38 / 106` | `kind=9, vid=250` (single variant) | `3500..=3524` (comp `3860..=3869`) |
| **TOTAL** | **`01..25`** | **All 25 Experiences** | **5 Acts (`ODYSSEY_ACTS`)** | **`ACT_1..ACT_5`** | **`376`** | **`89 / 57 / 121 / 109`** | **`2,456`** | **`446 / 277 / 794 / 939`** | **Zones: `23.7%` A / `15.2%` B / `32.2%` C / `29.0%` D** | **`1100..=3599` + `3600..=3869`** |

---

## 4-Phase Rollout Schedule

| Phase | Scope | Key Milestones |
|---|---|---|
| **Phase 1 — P0 Infrastructure & Critical Dispatch Bottlenecks** | `PKG-P0-DISPATCH-AND-IDS`, `PKG-P0-FEATURE-AND-CUE-PLUMBING`, `PKG-P0-CATALOG-SCENE-AND-CI` + high-impact collapse/bypass fixes (`03` cutaway clamp, `05` `zone_index % 3` landmarks, `12` `5 \| 12 =>` bypass, `21..25` single-variant splits, `11/13..20` modulo splits, `15_complexity` unread `field_grid_tex`) | Eliminate every `sc % N` modulo collision, bypassed experience, and single-variant collapse; wire `1100..=3869` WGSL decoding and `cue_tags` -> GPU `base_flags` plumbing. |
| **Phase 2 — Granular `doc_feat` (`0..=7`) Choreography** | Remaining Act III (`13, 14`), Act IV (`15..17`), and Act V (`11, 12, 18..20, 23, 24`) zone/feature upgrades + Act II `21, 22` follow-through | Every zone responds distinctly to each spoken feature `doc_feat` `0..=7` with cross-sections, callouts, particle bursts, and `doc_cue`/`doc_weight` modulation (converting Tier-B/C/D features to Tier-A). |
| **Phase 3 — Sensory & Macro-Cosmological Deepening (`01..10`)** | Act I (`03, 04`), Act II core (`05..10`), and Act IV spatial labs (`01, 02`) | Upgrade `03_deep_earth` / `04_solar_system` world shaders, dedicated `3660..=3709` landmarks (`05..07`), CMB/ionization isolation (`08..10`), and full interactive `01_colors` / `02_music` sensory depth. |
| **Phase 4 — Scale-Bridge Transitions & Stereo Framing Polish** | Cross-act `continuity_edges()` (`10 -> 11/12/23/24/25`, `12 -> 13`, `03 -> 13`, `17 -> 01/02`), `10_early_universe` inflation epochs, `25_final_horizon` finale | Smoothly choreograph scale transitions across `grand_cosmic_odyssey`, verify stereo OpenXR & desktop `1600x900` framing, and lock regression screenshots + benchmarks across all `25` experiences. |

---

## Pre-Allocated Zero-Conflict Shader ID Registry

### 1. Variant ID (`vid`) Ranges (`kind_tags.y` / `kind_tags[1]`)

| Range | Purpose | Formula / Allocation Rule |
|---|---|---|
| `0..=299` | **Legacy Range (Read-Only Transitional)** | Existing `doc_stage_variant_id` values (`0..=250`, `hero_kind 0..=9`). Left intact for untouched zones during staged rollout; retired per experience as bespoke packages land. |
| `1100..=3599` | **Primary Zone Hero Stages** (`25` exclusive 100-ID blocks) | `vid = 1000 + (exp_id * 100) + zone_index` (`exp_id in 1..=25`, `zone_index in 0..=99`). Guaranteed zero collision across all `376` zones + spatial states. |
| `3600..=3869` | **Companion / Landmark / Dual-Hero Surfaces** | Fixed domain blocks (`3600..=3719` for Exps `01..=10`; `3720 + (exp_id - 11) * 10 + k`, `k = 0..=9`, for Exps `11..=25`). |

<details>
<summary>Complete 25-Experience Pre-Allocated Shader ID Block Table (`1100..=3869`)</summary>

| Exp | Slug | Primary Zone Block (`1000 + exp*100 + z`) | Active `vid` Span Used | Companion / Landmark Block (`3600..=3869`) |
|---:|---|---|---|---|
| `01` | `01_colors` | `1100..=1199` | `1100..=1109` (spatial modes) | `3600..=3609` (`3601..=3604`: Gold, Glass, Matte, Plasma PBR spheres) |
| `02` | `02_music` | `1200..=1299` | `1200..=1209` (spatial modes) | `3610..=3619` (`3610..=3613`: Timbre chambers; `3614`: Resonator ring) |
| `03` | `03_deep_earth` | `1300..=1399` | `1300..=1313` (`Z00..=Z13`) | `3620..=3629` (`3620`: Moon; `3621`: Magnetosphere outer shell; `3622`: LEO/GEO satellites) |
| `04` | `04_solar_system` | `1400..=1499` | `1400..=1411` (`Z00..=Z11`) | `3640..=3651` (Sun..Pluto + Kuiper companion surfaces) |
| `05` | `05_milky_way` | `1500..=1599` | `1500..=1512` (`Z00..=Z12`) | `3660..=3671` (`3660 + landmark_idx` for `Z00..=Z11` landmarks + 13th `Z12` edge landmark via `1512`) |
| `06` | `06_local_group` | `1600..=1699` | `1600..=1608` (`Z00..=Z08`) | `3680..=3688` (`3680 + landmark_idx` for `9` landmarks) |
| `07` | `07_laniakea` | `1700..=1799` | `1700..=1709` (`Z00..=Z09`) | `3700..=3709` (`3700 + landmark_idx` for `10` landmarks) |
| `08` | `08_cosmic_web` | `1800..=1899` | `1800..=1810` (`Z00..=Z10`) | *(uses shared far shell / compute Mode 2)* |
| `09` | `09_observable_universe` | `1900..=1999` | `1900..=1910` (`Z00..=Z10`) | *(uses shared far shell / compute Mode 2)* |
| `10` | `10_early_universe` | `2000..=2099` | `2000..=2008` (`Z00..=Z08`) | *(uses compute Modes 3/5)* |
| `11` | `11_cosmic_evidence` | `2100..=2199` | `2100..=2108` (`Z00..=Z08`) | `3720..=3729` (`3720 + k`, `k = 0..=9`) |
| `12` | `12_future_of_universe` | `2200..=2299` | `2200..=2209` (`Z00..=Z09`) | `3730..=3739` (`3730 + k`, e.g. cooling dwarf / Hawking companion) |
| `13` | `13_matter_energy` | `2300..=2399` | `2300..=2311` (`Z00..=Z11`) | `3740..=3749` (`3740 + k`) |
| `14` | `14_spacetime` | `2400..=2499` | `2400..=2414` (`Z00..=Z14`) | `3750..=3759` (`3750 + k`) |
| `15` | `15_complexity` | `2500..=2599` | `2500..=2515` (`Z00..=Z15`) | `3760..=3769` (`3760 + k`) |
| `16` | `16_life` | `2600..=2699` | `2600..=2618` (`Z00..=Z18`) | `3770..=3779` (`3770 + k`) |
| `17` | `17_mind` | `2700..=2799` | `2700..=2717` (`Z00..=Z17`) | `3780..=3789` (`3780 + k`) |
| `18` | `18_culture` | `2800..=2899` | `2800..=2819` (`Z00..=Z19`) | `3790..=3799` (`3790 + k`) |
| `19` | `19_technology` | `2900..=2999` | `2900..=2919` (`Z00..=Z19`) | `3800..=3809` (`3800 + k`) |
| `20` | `20_futures` | `3000..=3099` | `3000..=3018` (`Z00..=Z18`) | `3810..=3819` (`3810 + k`) |
| `21` | `21_cosmic_life` | `3100..=3199` | `3100..=3124` (`Z00..=Z24`) | `3820..=3829` (`3820 + k`, exoplanet / Europa companion globes) |
| `22` | `22_cosmic_contact` | `3200..=3299` | `3200..=3227` (`Z00..=Z27`) | `3830..=3839` (`3830 + k`, Earth-origin vs alien transmitter pairs) |
| `23` | `23_cosmic_perspectives` | `3300..=3399` | `3300..=3324` (`Z00..=Z24`) | `3840..=3849` (`3840 + k`) |
| `24` | `24_cosmic_meaning` | `3400..=3499` | `3400..=3425` (`Z00..=Z25`) | `3850..=3859` (`3850 + k`) |
| `25` | `25_final_horizon` | `3500..=3599` | `3500..=3524` (`Z00..=Z24`) | `3860..=3869` (`3860 + k`) |

</details>

### 2. Packed `HeroVolumeInstance::kind_tags: [u32; 4]` & `submode` / `base_flags` Bit Layout

From [`pack_doc_tags`](../crates/cosmos_render/src/doc_stages.rs#L13-L31):
- `kind_tags[0]` (WGSL `.x`): `hero_kind` (`0..=9` legacy domain dispatcher).
- `kind_tags[1]` (WGSL `.y`): `vid` (`0..=299` legacy, `1100..=3599` bespoke zone, `3600..=3869` companion).
- `kind_tags[2]` (WGSL `.z`): `z_u16 | (f_u8 << 16) | (sc_u8 << 24)`
  - `doc_zone(inst)` (`bits 0..15`): `active_zone_index` (`0..=27`, `65535` = experience overview).
  - `doc_feat(inst)` (`bits 16..23`): `active_feature_index` (`0..=7`, `0xFF` = macro zone overview).
  - `doc_scene(inst)` (`bits 24..31`): `(scene_index + 1).clamp(0, 254)` (`0` when `scene_index < 0`).
- `kind_tags[3]` (WGSL `.w`): `(base_flags & 0xFF) | ((cue_u8 & 0xFF) << 8) | ((wt_u8 & 0xFF) << 16) | ((submode & 0xFF) << 24)`
  - **`base_flags` (`bits 0..7`):**
    - `Bit 0` (`0x01`): Cutaway slice active (Earth interior, planetary crust, organ/cell cross-section).
    - `Bit 1` (`0x02`): Gravitational lensing / dual-hero silhouette coupling.
    - `Bits 2..7` (`0x04..=0x80`): Semantic `SegmentVisualHint::cue_tags` domain mask (packed by [`PKG-P0-FEATURE-AND-CUE-PLUMBING`](#pkg-p0-feature-and-cue-plumbing)).
  - **`doc_cue(inst)` (`bits 8..15`):** Normalized narration cue progress (`0.0..=1.0`).
  - **`doc_weight(inst)` (`bits 16..23`):** Active narration weight (`0.0..=1.0`), clamped smoothly into `0.72..=1.35x` HDR brightness without strobe.
  - **`submode` (`bits 24..31`, `0..=255`):**
    - `0`: Default bespoke zone stage (`doc_feat == 0xFF` macro overview / settling).
    - `1..=7`: Cutaway / cross-section / alternate physical state (e.g., `1` = Earth interior slice, `0..=3` = Color material or Music timbre).
    - `8..=31`: Coupled `compute_mode` / `cs_field_grid` hint flags.
    - `32..=254`: Extended overlay / landmark / disambiguator tags.

> [!WARNING]
> **Critical WGSL Decoder Trap (`vid >= 1000u`):**
> Current WGSL dispatchers check `vid >= 100u` with `vid / 10u` and `vid % 10u` (`ch_exp_and_sub` in [`shaders/hero_doc_civilization_horizon.wgsl#L20-L25`](../shaders/hero_doc_civilization_horizon.wgsl#L20-L25)) AND clamp `sub_v < 10u` (`#L48`), while [`shaders/hero_doc_matter_mind.wgsl`](../shaders/hero_doc_matter_mind.wgsl#L468) checks `vid >= 20u && vid < 30u`. If `vid = 2105` is fed into untouched decoders, `2105 / 10u = 210` (`!= 11u`) silently drops or misroutes! [`PKG-P0-DISPATCH-AND-IDS`](#pkg-p0-dispatch-and-ids) MUST add an explicit `vid >= 1000u && vid < 3600u` decode branch (`exp = (vid - 1000u) / 100u`, `zone = (vid - 1000u) % 100u`) without `< 10u` clamping so zones `10..=27` route cleanly.

---

## Core Pipeline P0 Packages

Complete these `3` P0 packages first (or in a tight Phase 1 train) so all `85` zone content packages land onto conflict-free IDs, full `doc_feat` + `cue_tags` plumbing, and deterministic catalog/headless regression checks.

<a id="pkg-p0-dispatch-and-ids"></a>
### PKG-P0-DISPATCH-AND-IDS: Canonical `1100..=3869` Shader ID Dispatch & Legacy Bypass Removal

**Status:** `[COMPLETED]` *(Landed on `main` & aligned to table above: canonical `1100..=3869` Rust registry + WGSL `doc_vid_exp` `u32` underflow guard in `doc_common.wgsl`, `Exp 03` `1300..=1313`, `Exp 04` `1400..=1411` + `3640..=3651`, `Exp 05` canonical companion landmark IDs `3660..=3671` with `submode = zone_index`, and un-bypassed `Exp 12` primary stage `kind=9, vid=120..=124` + Hawking BH companion `3730` on `Z05..=Z07`)* · **Phase:** Phase 1 (Prerequisites) · **Priority:** Critical P0

**Current State:**
- [`crates/cosmos_render/src/doc_stages.rs#L36-L91`](../crates/cosmos_render/src/doc_stages.rs#L36-L91): `doc_stage_variant_id` collapses Exps `11, 12, 13, 14, 15, 16, 17, 18, 19, 20` onto `sc % 4 / 5 / 7` modulo buckets and collapses Exps `21, 22, 23, 24, 25` onto `1–2` hardcoded variants (`210/212`, `220/221`, `230`, `240/241`, `250`).
- [`crates/cosmos_render/src/doc_stages.rs#L359-L367`](../crates/cosmos_render/src/doc_stages.rs#L359-L367): `emit_celestial_or_landmark` overwrites Exp `05` landmark IDs with `vis.zone_index % 3 => 20 | 21 | 22`, cycling only `3` shaders across `12` distinct galactic landmarks.
- [`crates/cosmos_render/src/doc_stages.rs#L567-L611`](../crates/cosmos_render/src/doc_stages.rs#L567-L611): Exp `03` cutaway shell clamps zone indices (`5.2..=6.45` with outer volumetric halo `r_vol * 1.14`) inconsistently with deep Earth interior zone boundaries (`Z06..=Z08`).
- [`crates/cosmos_render/src/doc_stages.rs#L657-L691`](../crates/cosmos_render/src/doc_stages.rs#L657-L691): `5 | 12 =>` match arm bypasses `doc_stage_variant_id` on Exp `12` (`12_future_of_universe`), emitting `kind=4, vid=12` Kerr black hole + `kind=1, vid=12` dwarf instead of calling `doc_stage_variant_id` (leaving `vid=120..124` unreachable dead code).
- [`crates/cosmos_render/src/doc_stages.rs#L764-L786`](../crates/cosmos_render/src/doc_stages.rs#L764-L786): Exp `21` emits extra hardcoded `11/10` globe companions regardless of zone topic.
- [`shaders/hero_doc_civilization_horizon.wgsl#L20-L25`](../shaders/hero_doc_civilization_horizon.wgsl#L20-L25) & [`#L48`](../shaders/hero_doc_civilization_horizon.wgsl#L48), [`shaders/hero_doc_matter_mind.wgsl#L468`](../shaders/hero_doc_matter_mind.wgsl#L468) & [`#L632`](../shaders/hero_doc_matter_mind.wgsl#L632), and [`shaders/hero_doc_cosmos.wgsl`](../shaders/hero_doc_cosmos.wgsl): Only understand legacy `vid < 300u` (`vid / 10u`, `vid - 20u`, etc.) and reject or misdecode `1100..=3869`.
- **Current Tier Impact:** Root blocker keeping `142` zones in Tier-C and `128` zones in Tier-D.

**Reserved Target IDs:**
- Primary feature-flagged lookup helper `resolve_zone_shader_vid(exp_id, zone_index, scene_index)` in `doc_stages.rs`: opts into `1000 + exp_id * 100 + zone_index` (`1100..=3599`) per migrated zone while preserving legacy `0..=299` for unmigrated zones.
- Landmark helper in `emit_celestial_or_landmark`: routes Exp `05` -> `3660..=3671`, Exp `06` -> `3680..=3688`, Exp `07` -> `3700..=3709`.
- Exp `12` black hole / cooling dwarf twin hero in `emit_doc_stage_hero`: moves under `vid >= 2200` / companion `3730..=3739` routing instead of the blanket `5 | 12 =>` bypass.

**Spoken Transcript Summary & 3D Visual Blueprint:**
- **Narrative Driver:** Every spoken zone across Acts I–V (`376` zones, `2,456` bilingual ES/EN features in `universe_catalog.v1.json`) describes a distinct physical or philosophical subject, yet current modulo/bypass routing paints unrelated zones (e.g., `12_future_of_universe` stellar era vs Hawking evaporation, `05_milky_way` `12` landmarks, `21..25` multi-topic chapters) with identical shapes.
- **3D Blueprint:**
  1. Add canonical `HeroIdDecoded { exp_id: u32, sub_id: u32, is_companion: bool }` decoders in WGSL (`ch_exp_and_sub` in `hero_doc_civilization_horizon.wgsl`, plus sibling decoders in `hero_doc_cosmos.wgsl` and `hero_doc_matter_mind.wgsl`) that decode `vid >= 3600u` -> companions, `vid >= 1000u` -> `(vid - 1000u) / 100u` & `(vid - 1000u) % 100u` directly without `sub_v < 10u` truncation, and fallback cleanly to legacy `vid < 1000u` during staged rollout.
  2. Refactor `doc_stages.rs` so each experience explicitly lists migrated bespoke zones (`-> 1000 + exp*100 + z`), separate Exp `12` from `5 =>` (`emit_doc_stage_hero` spawns companion black hole / dwarf via `base_flags` bit `1` + `3730..=3739` only for zones that require it), and fix the `03_deep_earth` cutaway radius clamp bug (`Z06..=Z08`) and `05_milky_way` `zone_index % 3` landmark aliasing (`-> 3660 + landmark_idx`).
  3. Add unit tests in `crates/cosmos_render` verifying zero collisions across all `376` `(exp_id, zone_index)` pairs and backward compatibility for legacy `0..=299`.

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-P0-DISPATCH-AND-IDS — Core ID Decoder & Bypass Removal)</summary>

Implement `PKG-P0-DISPATCH-AND-IDS` in the HyperCosmosVR repository root. Read `shaders/SHADER_CONTRACTS.md` first so WGSL concatenation order and Naga SM 6.5 compatibility remain intact.

1. **Files to Modify (P0 Architecture Train):**
   - `crates/cosmos_render/src/doc_stages.rs` (`doc_stage_variant_id`, `emit_celestial_or_landmark`, `emit_doc_stage_hero`, and unit tests)
   - `shaders/doc_common.wgsl` (shared `doc_decode_vid_exp(vid: u32)` / `doc_decode_vid_sub(vid: u32)` handling `0..=299`, `1100..=3599` as `(vid - 1000u)/100u` & `% 100u`, and `3600..=3869`)
   - `shaders/hero_doc_cosmos.wgsl`, `shaders/hero_doc_matter_mind.wgsl`, `shaders/hero_doc_civilization_horizon.wgsl` (`ch_exp_and_sub` `#L20-L25`, removing the `sub_v < 10u` clamp on `#L48` when `vid >= 1000u`, and updating `#L468` / `#L609-L632` range checks so `1100..=3599` & `3600..=3869` route cleanly while preserving untouched legacy `0..=299` IDs)
   - `crates/cosmos_narration/src/resolver.rs` (only read-only check or helper sync if variant table assertion uses resolver metadata) and `shaders/compute_universe.wgsl` (verify `cs_advect_particles` mode enum alignment if touched).

2. **Specific Bugs to Fix:**
   - Remove blanket `5 | 12 =>` bypass at `doc_stages.rs:657-691` for Experience `12` (`12_future_of_universe`) so `12` uses `doc_stage_variant_id` + opt-in Kerr/Dwarf companion IDs (`3730..=3739`) only on appropriate zones.
   - Replace Exp `05` landmark `vis.zone_index % 3 => 20 | 21 | 22` (`doc_stages.rs:359-367`) with reserved landmark IDs `3660 + landmark_idx` (mapping cleanly back to `20..22` fallback for untouched landmarks until migrated).
   - Gate Exp `21` extra twin globes (`doc_stages.rs:764-786`) behind explicit zone / companion IDs (`3820..=3829`).
   - Fix Exp `03` cutaway zone clamp (`5.2..=6.45` / `r_vol * 1.14` in `doc_stages.rs:567-611`) so Earth crust/lithosphere zones match `Z06..=Z08`.

3. **Mandatory Hard Verification Gates:**
   - **OLED Void Budget:** Preserve `>= 50.91%` exact `#000000` background pixels and `0.0000%` highlight blowout; respect `r <= 0.96` clipping (`hero_volumetric.wgsl:164`) and `oled_void_clamp` (`:201`).
   - **VR 90/120 Hz Budget:** Keep documentary shells `<= 28` raymarch steps (`trans < 0.03` early exit), preserving RTX 5070 `1600x900` baseline (`mean = 1.967 ms`, `p99 = 4.752 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - **Verification Commands:**
     - `cargo clippy --workspace --all-targets -- -D warnings`
     - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
     - All `.github/workflows/ci.yml` CPU tests + local `cargo test --workspace -- --test-threads=1` (verify `phase7_documentary_stages_01_through_25_oled_and_telemetry_hold` and `phase7b_zone_routing_framing_and_sprite_clamps_hold` pass).
   - **Repo Hygiene:** Zero `> 50 MiB` files, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

</details>

---

<a id="pkg-p0-feature-and-cue-plumbing"></a>
### PKG-P0-FEATURE-AND-CUE-PLUMBING: Narration `cue_tags` -> GPU `base_flags`, `doc_feat` & `field_grid_tex` Choreography

**Status:** `[COMPLETED]` *(Landed on `main`: `encode_cue_tags_mask` -> `base_flags[2..8]`, `submode_override`, `emphasis_boost`, `doc_feat_focus`, `doc_sample_field_grid` + Exp `02`/`15` coupling)* · **Phase:** Phase 1 (Prerequisites) · **Priority:** Critical P0

**Current State:**
- [`crates/cosmos_narration/src/resolver.rs#L166-L201`](../crates/cosmos_narration/src/resolver.rs#L166-L201), [`#L940-L961`](../crates/cosmos_narration/src/resolver.rs#L940-L961), [`#L983-L1085`](../crates/cosmos_narration/src/resolver.rs#L983-L1085): `populate_segment_visual_hints` resolves rich `SegmentVisualHint` structs (`DOMAIN_CUE_KEYWORDS` `57` tokens, chromatic material classification, active cue weights), yet `DocumentaryCueSnapshot` drops `cue_tags` on the floor before crossing into `cosmos_render`.
- [`crates/cosmos_app/src/engine.rs#L1675-L1700`](../crates/cosmos_app/src/engine.rs#L1675-L1700): `build_documentary_focus_state` constructs [`DocumentaryFocusState`](../crates/cosmos_render/src/universe_scene.rs#L17-L25) with NO `cue_tags` / `emphasis_boost` fields, and only populates `telemetry = [zone.range_start, zone.range_end, zone.phys_low, zone.phys_high]` when `active_experience_id >= 3` (`engine.rs#L1682-L1684`).
- [`crates/cosmos_render/src/universe_scene.rs#L217-L222`](../crates/cosmos_render/src/universe_scene.rs#L217-L222) & [`crates/cosmos_render/src/scene_pass.rs#L175-L200`](../crates/cosmos_render/src/scene_pass.rs#L175-L200), [`#L755-L764`](../crates/cosmos_render/src/scene_pass.rs#L755-L764): `cs_field_grid` runs into `field_grid_tex` (`FIELD_GRID_DIM = 256`), yet across all hero WGSL shaders ONLY [`shaders/hero_doc_cosmos.wgsl#L282`](../shaders/hero_doc_cosmos.wgsl#L282) (Exp `03` polar aurora) samples it — leaving `02_music` Chladni and `15_complexity` Gray-Scott PDE textures unread.
- [`shaders/doc_common.wgsl`](../shaders/doc_common.wgsl): Exposes `doc_zone`, `doc_feat`, `doc_scene`, `doc_cue`, `doc_weight`, `doc_submode`, and `base_flags` extraction helpers, but needs canonical bitmask extractors (`doc_has_cutaway`, `doc_has_lensing`, `doc_cue_mask`, `doc_feat_active`, smooth `doc_feat_blend`).
- **Current Tier Impact:** Primary reason `396` Tier-B and `865 + 1,195` Tier-C/D features lack semantic `doc_feat` 1:1 visual transitions even where a zone mesh exists.

**Reserved Target IDs & Bit Packing:**
- `DocumentaryFocusState` (`universe_scene.rs:17-25`): add `pub cue_tags_mask: u8`, `pub submode: u8`, `pub emphasis_boost: f32`.
- `base_flags` (`kind_tags.w` bits `0..=7`): bit `0` (`0x01`) = cutaway, bit `1` (`0x02`) = lensing/dual-hero, bits `2..=7` (`0x04..=0x80`) = `cue_tags_mask` (`Atomic/Quantum`, `Relativistic/Gravitational`, `Thermal/Energy`, `Biological/Cellular`, `Neural/Informational`, `Architectural/Technological` domain tags derived from `DOMAIN_CUE_KEYWORDS`).
- `doc_feat` (`kind_tags.z` bits `16..=23`): canonical `0..=7` active feature index (`255` = macro overview), paired with `doc_cue` (`0.0..=1.0`) and `doc_weight` (`0.72..=1.35x` clamped HDR gain).
- `field_grid_tex` (`@group(1) @binding(3)`): exposed via shared helper `doc_sample_field_grid(uv)` in `doc_common.wgsl` for `02_music`, `15_complexity`, and new wave/reaction-diffusion stages.

**Spoken Transcript Summary & 3D Visual Blueprint:**
- **Narrative Driver:** Across all `2,456` spoken features (`F0..F7` per zone in `universe_catalog.v1.json`), narration alternates rapidly between micro-mechanisms (e.g., Earth core dynamo vs crust subduction; DNA base-pair unzip vs ATP synthase; Chladni harmonic nodes vs CQT timbre spectrum) with bilingual cue words tracked in `DOMAIN_CUE_KEYWORDS` (`57` tokens).
- **3D Blueprint:**
  1. Forward `SegmentVisualHint::cue_tags` bitmask + `submode` through `DocumentaryCueSnapshot` -> `build_documentary_focus_state` (`engine.rs`) -> `DocumentaryFocusState` (`universe_scene.rs`) -> `pack_doc_tags` (`doc_stages.rs`).
  2. Pack semantic cue mask into `base_flags` bits `2..=7`, preserve `doc.telemetry` cleanly across all `01..=25`, and add `submode` hints (`8..=31`) driving `select_compute_mode` in `universe_scene.rs#L301-L342` (`compute_universe.wgsl` particle advection + `cs_field_grid`).
  3. Add shared WGSL helpers in `shaders/doc_common.wgsl`: `doc_cue_tag_bit(inst, bit)`, `doc_feat_focus(inst, feat_idx)` (smooth ramp using `doc_cue` and `doc_weight` with strict HDR clamp `0.72..=1.35` — zero abrupt frame popping or strobing), and `doc_sample_field_grid(p_uv)`.
  4. Wire `field_grid_tex` sampling into Exp `15` (`15_complexity` Gray-Scott patterns) and Exp `02` (`02_music` Chladni PDE), establishing the pattern for Phase 2 feature choreography PRs.

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-P0-FEATURE-AND-CUE-PLUMBING — Cue Tags, doc_feat & Field Grid Wiring)</summary>

Implement `PKG-P0-FEATURE-AND-CUE-PLUMBING` in the HyperCosmosVR repository root. Read `shaders/SHADER_CONTRACTS.md` first so `HeroVolumeInstance` (`112 B`), `FrameUniforms`, and bind group layouts stay exact.

1. **Files to Modify:**
   - `crates/cosmos_narration/src/resolver.rs` (`DocumentaryCueSnapshot` `#L191-L201`, pack `SegmentVisualHint::cue_tags` `#L166-L186` from `DOMAIN_CUE_KEYWORDS` `#L952-L961` into a `u8` domain bitmask `bits 0..5` -> shifted to `bits 2..7` of `base_flags`).
   - `crates/cosmos_app/src/engine.rs` (`build_documentary_focus_state` `#L1675-L1700` to populate `cue_tags_mask`, `submode`, and consistent `telemetry` without breaking Exp `01`/`02` interactive color/pitch telemetry).
   - `crates/cosmos_render/src/universe_scene.rs` (`DocumentaryFocusState` `#L17-L25`, `select_compute_mode` `#L301-L342`) & `crates/cosmos_render/src/doc_stages.rs` (`pack_doc_tags` `#L13-L31`, passing `cue_tags_mask` into `base_flags` bits `2..7` and `doc.submode`).
   - `shaders/doc_common.wgsl` + `shaders/hero_doc_matter_mind.wgsl` (add `doc_cue_mask`, `doc_feat_focus`, `doc_sample_field_grid`; wire `field_grid_tex` sampling in Exp `15` Gray-Scott and Exp `02` Chladni), plus verify `shaders/compute_universe.wgsl` modes align.

2. **Mandatory Hard Verification Gates:**
   - **OLED Void Budget:** Keep `>= 50.91%` `#000000` background and `0.0000%` highlight blowout; modulate `doc_feat` emphasis smoothly within `0.72..=1.35x` HDR clamp (zero strobe, zero fog wash).
   - **VR 90/120 Hz Budget:** Keep steps `<= 28` for documentary shells (`trans < 0.03` early exit), preserving RTX 5070 `1600x900` DX12 baseline (`mean = 1.967 ms`, `p99 = 4.752 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - **Verification Commands:**
     - `cargo clippy --workspace --all-targets -- -D warnings`
     - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
     - All `.github/workflows/ci.yml` CPU tests + `cargo test --workspace -- --test-threads=1`.
   - **Repo Hygiene:** Zero `> 50 MiB` files, zero `.ttf`/`.otf`, zero machine-specific `.cargo/config.toml`.

</details>

---

<a id="pkg-p0-catalog-scene-and-ci"></a>
### PKG-P0-CATALOG-SCENE-AND-CI: Catalog `scene` Backfill, Exp 22/24 Metadata Cleanup & Headless Regression Gate

**Status:** `[CORE LANDED]` *(Exp `24` title `"Significado Cósmico"` aligned in `topology.rs`, `pack_doc_tags` null-scene fallback active, and 376-zone canonical ID + Exp `03` Z05..Z08 GPU test added; static `universe_catalog.v1.json` scene backfill remains optional)* · **Phase:** Phase 1 (Prerequisites) · **Priority:** Critical P0

**Current State:**
- `assets/baked/universe_catalog.v1.json`: `188 / 376` (`50.0%`) zones across Exps `03..=17` carry `scene: null`, and `89 / 376` zones across Exps `03..=10` carry `evidence_class: null`, forcing `doc_stages.rs` to fall back to `zone_index` modulo arithmetic when `doc.scene_index < 0`.
- `assets/baked/universe_catalog.v1.json` (Exp `22`, `22_cosmic_contact`): `5` distinct zone pairs (`10` zones) share identical `scene_index` values (`(Z00, Z01) sc=0`, `(Z09, Z10) sc=8`, `(Z15, Z16) sc=13`, `(Z17, Z18) sc=14`, `(Z19, Z20) sc=15`).
- [`crates/cosmos_spatial/src/topology.rs#L377-L380`](../crates/cosmos_spatial/src/topology.rs#L377-L380): Exp `24` (`24_cosmic_meaning`) title is `"Sentido Cósmico"` while `universe_catalog.v1.json` stores `"Significado Cósmico"`.
- [`crates/cosmos_render/tests/render_tests.rs#L380-L460`](../crates/cosmos_render/tests/render_tests.rs#L380-L460) & [`crates/cosmos_app/src/headless.rs`](../crates/cosmos_app/src/headless.rs): Currently run `phase7_documentary_stages_01_through_25_oled_and_telemetry_hold` and `phase7b_zone_routing_framing_and_sprite_clamps_hold` using `1` or a handful of zones per experience — leaving `300+` zone permutations and `doc_feat` transitions untested in regression CI!

**Reserved Target IDs & Contract Guarantees:**
- Every zone `0..z_max` across `03..=25` receives an unambiguous monotonic `scene` / `scene_index` (`0..=27`) in `universe_catalog.v1.json` (or validated schema invariant) while leaving `universe_catalog.v1.json` well below the `50 MiB` repository file cap and compatible with `cosmos_manifest` tests.
- Exp `24` localized ES title unified across `topology.rs:379` and catalog metadata.
- New automated test helper in `crates/cosmos_render/tests/render_tests.rs` asserting across ALL `376` zones (`exp 03..=25`) and `01..=02`: unique `(exp_id, zone_index) -> vid` resolution once migrated, non-zero `HeroVolumeInstance` bounds inside frustum, zero NaN/blowout, and OLED `#000000` void preservation.

**Spoken Transcript Summary & 3D Visual Blueprint:**
- **Narrative Driver:** In `22_cosmic_contact`, `Z00` (*"Una señal sin explicación definitiva"*) vs `Z01` (*"La primera anomalía"*), `Z09` (*"Coordinación internacional"*) vs `Z10` (*"Reacción planetaria"*), and `Z17..Z20` (*Decision to transmit, transmission risks, speed-of-light limit, interstellar wait*) tell starkly different dramatic chapters yet share identical `scene_index` keys. Meanwhile `03..=17` have null catalog scene tags, hiding distinct chapter beats behind implicit modulo math.
- **3D Blueprint (Tooling & Validation Architecture):**
  1. Audit and backfill deterministic per-zone `scene` / `scene_index` metadata cleanly across `universe_catalog.v1.json` (resolving all `188` `null` scenes in `03..=17` and separating Exp `22`'s `5` colliding zone pairs onto unique `0..=27` indices), and normalize Exp `24` title consistency between `topology.rs:379` and `universe_catalog.v1.json` (verifying `cosmos_manifest` & `cosmos_spatial` unit tests pass).
  2. Extend `crates/cosmos_render/tests/render_tests.rs` to loop cleanly across every `(exp_id, zone_index)` (`376` zones + `01, 02`) plus synthetic `doc_feat = 0..=7` sweep checks, asserting zero ID collisions in migrated ranges and valid OLED/frame budgets in `--headless-render`.
  3. Add a fast WGSL/CPU contract test validating `HeroVolumeInstance::kind_tags` bit-packing roundtrips (`pack_doc_tags`) and `resolve_zone_shader_vid` coverage without requiring heavy full-audio mmap packs on CPU-only GitHub CI.

<details>
<summary>Copy-Paste AI Sub-Prompt (PKG-P0-CATALOG-SCENE-AND-CI — Metadata Cleanup & 376-Zone Test Matrix)</summary>

Implement `PKG-P0-CATALOG-SCENE-AND-CI` in the HyperCosmosVR repository root. Respect `CONTRIBUTING.md` (`no files > 50 MiB`, zero `.ttf`/`.otf`) and `.github/workflows/ci.yml`.

1. **Files to Modify:**
   - `assets/baked/universe_catalog.v1.json` (backfill `scene` / `scene_index` cleanly where `null` across `03..=17`, disambiguate the `5` duplicated `scene_index` pairs in `22_cosmic_contact`, and verify file stays strictly well under `50 MiB` — confirm `cosmos_manifest` tests still pass).
   - `crates/cosmos_spatial/src/topology.rs#L377-L380` (align Exp `24` `title` consistently with `universe_catalog.v1.json`).
   - `crates/cosmos_render/tests/render_tests.rs` (expand coverage alongside `phase7_documentary_stages_01_through_25_oled_and_telemetry_hold` and `phase7b_zone_routing_framing_and_sprite_clamps_hold` to verify all `376` catalog zones, unique migrated `1100..=3599` IDs, `pack_doc_tags` bit roundtrips, and `doc_feat 0..=7` telemetry sweep).
   - `crates/cosmos_render/src/doc_stages.rs` / `crates/cosmos_narration/src/resolver.rs` / `shaders/hero_doc_*.wgsl` / `shaders/compute_universe.wgsl` (sync any test-revealed edge cases cleanly without breaking Naga SM 6.5).

2. **Mandatory Hard Verification Gates:**
   - **OLED Void Budget:** Preserve `>= 50.91%` exact `#000000` background pixels (`0.0000%` blowout).
   - **VR 90/120 Hz Budget:** Preserve RTX 5070 `1600x900` DX12 baseline (`mean = 1.967 ms`, `p99 = 4.752 ms` in `artifacts/verification_renders/benchmark_sweep.txt`).
   - **Verification Commands:**
     - `cargo clippy --workspace --all-targets -- -D warnings`
     - `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`
     - `cargo test -p cosmos_manifest --test manifest_tests -- --test-threads=1 --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets`
     - `cargo test -p cosmos_spatial topology -- --test-threads=1`
     - Full CI test list in `.github/workflows/ci.yml` + local `cargo test --workspace -- --test-threads=1`.
   - **Repo Hygiene:** Check `git ls-files` size check `< 50 MiB` and zero `.ttf`/`.otf` fonts.

</details>

---

## Contributor Step-by-Step Workflow & Verification Gates

Every visual expansion PR should claim **one canonical Work Package** (`PKG-ACT<n>-...`, typically `3–5` zones, or an entire spatial lab for `01`/`02`) after checking open PRs and ensuring any prerequisite **P0** plumbing is in place.

### Standard 4-File Implementation Workflow

1. **Narration Cue Tokens (`crates/cosmos_narration/src/resolver.rs`):**
   - Check spoken ES+EN feature transcripts (listed in the target `docs/roadmap/ACT_*.md` Pick-Up Card) against `DOMAIN_CUE_KEYWORDS` (`57` tokens at [`resolver.rs#L952-L961`](../crates/cosmos_narration/src/resolver.rs#L952-L961)) and `populate_segment_visual_hints` (`#L983-L1085`). Add domain keywords only if a zone introduces missing semantic cue bits (`base_flags` bits `2..=7`).
2. **Rust Stage Dispatch & Companions (`crates/cosmos_render/src/doc_stages.rs`):**
   - Update [`doc_stage_variant_id`](../crates/cosmos_render/src/doc_stages.rs#L36-L91) to return the zone's reserved `1000 + exp_id * 100 + zone_index` ID (`1100..=3599`).
   - Update [`emit_doc_stage_hero`](../crates/cosmos_render/src/doc_stages.rs#L438-L789) and [`emit_celestial_or_landmark`](../crates/cosmos_render/src/doc_stages.rs#L139-L434) for any secondary companion surfaces (`3600..=3869`) or `submode` / `base_flags` (`bit 0` cutaway, `bit 1` lensing). Keep unmigrated zones on legacy `0..=299`.
3. **WGSL Hero Geometry & Feature Choreography (`shaders/hero_doc_*.wgsl` + `shaders/doc_common.wgsl`):**
   - Read [`shaders/SHADER_CONTRACTS.md`](../shaders/SHADER_CONTRACTS.md) before editing. Remember `hero_volumetric.wgsl` concatenates:
     `common_oled.wgsl` -> `doc_common.wgsl` -> `hero_doc_cosmos.wgsl` -> `hero_doc_matter_mind.wgsl` -> `hero_doc_civilization_horizon.wgsl`.
   - Branch on `vid = 1000 + exp*100 + z` AND implement distinct visual sub-components / cross-sections / phase transitions for **every** spoken feature `doc_feat(inst) == 0u..7u` (with `255u` overview), modulated smoothly by `doc_cue(inst)`, `doc_weight(inst)`, and `base_flags` cue bits.
4. **GPU Compute & Sprite Support (`shaders/compute_universe.wgsl`, `crates/cosmos_render/src/universe_scene.rs`, `shaders/instanced_sprites.wgsl`):**
   - Where zones need swarming agents, fluid curl fields, reaction-diffusion PDEs, or discrete callout markers, wire [`select_compute_mode`](../crates/cosmos_render/src/universe_scene.rs#L301-L342) (`cs_advect_particles` / `cs_field_grid` in `shaders/compute_universe.wgsl`) or [`shaders/instanced_sprites.wgsl`](../shaders/instanced_sprites.wgsl) (`SpriteKind 0..7`) alongside `field_grid_tex` (`@group(1) @binding(3)`).

### Conflict-Free Claiming Protocol (Corporate Teams & Solo Contributors)

1. Check open GitHub Issues and PRs for `[PKG-...]` to ensure no other team holds that package lock.
2. Open a Claim Issue or Draft PR titled `[PKG-ACTx-EXPyy-Zaa-bb] <short title>` on a dedicated topic branch (`feat/pkg-actx-expyy-...`), check whether Core P0 prerequisites have landed on `main`, and modify only files/IDs scoped to your card.
3. Non-code reviewers (educators, curators, UX designers) can attach domain notes or `.hcj.json` camera framing directly to the package Issue before shader code is written.

### Hard Quality, OLED, Performance & CI Gates

| Gate | Requirement | Verified Reference Baseline / Check |
|---|---|---|
| **Pure-Black OLED Void** | Keep `>= 50.91%` exact `#000000` background pixels on newly added/upgraded zone PRs (`>= 50.90%` on existing `43` baseline PNGs; `50.9004%` min in `06_cmb_and_inflation.png`) and `0.0000%` highlight blowout; zero full-screen ambient haze/fog wash; bound documentary volumes to `r <= 0.96` (`hero_volumetric.wgsl#L164`) and pass outputs through `oled_void_clamp` (`#L201`). | `pure_black` / `non_black` counters in [`crates/cosmos_app/src/headless.rs`](../crates/cosmos_app/src/headless.rs) + README verified gallery (`50.90%–76.77%` `#000000`). |
| **VR 90/120 Hz Frame Budget** | Cap new documentary hero raymarches at `<= 28` steps with `trans < 0.03` early exit; smooth `doc_cue`/`doc_weight` HDR clamping (`0.72..=1.35x`, zero strobing). | RTX 5070 DX12 `1600x900` 120-frame sweep ([`benchmark_sweep.txt`](../artifacts/verification_renders/benchmark_sweep.txt)): `min = 0.837 ms`, `mean = 1.974 ms (506.7 FPS)`, `p99 = 5.109 ms`, `max = 5.188 ms`. |
| **Headless Visual Capture** | Capture and inspect PNGs + sweep telemetry before opening a PR. | `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep` |
| **Clippy & Formatting** | Zero warnings across all workspace targets; clean `rustfmt`. | `cargo clippy --workspace --all-targets -- -D warnings` & `cargo fmt --check` |
| **CPU-Only CI Test Suite** | Must pass cleanly without GPU/OpenXR/full-audio-pack dependencies (exact [`.github/workflows/ci.yml`](../.github/workflows/ci.yml) matrix). | `cargo test -p cosmos_math -- --test-threads=1` · `cosmos_narration` · `cosmos_nav` · `cosmos_journey` · `cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe` · `cosmos_spatial topology` · `cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets` |
| **Local GPU Render Tests** | Verify documentary telemetry, OLED invariants, zone routing, and framing across all `25` experiences. | `cargo test --workspace -- --test-threads=1` (verifies [`phase7_documentary_stages_01_through_25_oled_and_telemetry_hold`](../crates/cosmos_render/tests/render_tests.rs) & `phase7b_zone_routing_framing_and_sprite_clamps_hold`) |
| **Repository Hygiene** | Zero `.ttf`/`.otf` font binaries, zero files `> 50 MiB`, zero machine-specific `.cargo/config.toml` committed. | [`.github/workflows/ci.yml`](../.github/workflows/ci.yml) & [`CONTRIBUTING.md`](../CONTRIBUTING.md) |

---

## Complete `88`-Package Master Index

Every package below links directly to its self-contained Pick-Up Card (complete with zone/feature transcripts, current file:line state, reserved target IDs, 3D WGSL blueprint, and copy-paste AI sub-prompt).

### Core Pipeline P0 (`3` Packages — Land First in Phase 1)

| Package ID | Owner Crates & Shaders | Scope | Target Phase | Priority |
|---|---|---|---|---|
| [`PKG-P0-DISPATCH-AND-IDS`](#pkg-p0-dispatch-and-ids) | `doc_stages.rs`, `doc_common.wgsl`, `hero_doc_*.wgsl` | Canonical `1100..=3599` & `3600..=3869` WGSL decoding; remove Exp `12` bypass, Exp `05` `zone_index % 3`, Exp `21` twin globes, and Exp `03` cutaway clamp | Phase 1 | Critical P0 |
| [`PKG-P0-FEATURE-AND-CUE-PLUMBING`](#pkg-p0-feature-and-cue-plumbing) | `resolver.rs`, `engine.rs`, `universe_scene.rs`, `doc_stages.rs`, `doc_common.wgsl` | Forward `SegmentVisualHint::cue_tags` -> `base_flags` (`bits 2..7`), pack `submode`/`telemetry`, add `doc_feat_focus` & `field_grid_tex` helpers | Phase 1 | Critical P0 |
| [`PKG-P0-CATALOG-SCENE-AND-CI`](#pkg-p0-catalog-scene-and-ci) | `universe_catalog.v1.json`, `topology.rs`, `render_tests.rs` | Backfill `188` null `scene` fields (`03..=17`), resolve Exp `22` `5` duplicate scene pairs, unify Exp `24` title, add 376-zone CI test matrix | Phase 1 | Critical P0 |

### Odyssey Act I — Earth & Solar System (`6` Packages: `03, 04` — `26` Zones, `130` Features)

See full Pick-Up Cards in [`docs/roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md):

| Package ID | Exp | Zones & Catalog Names | Feats | Reserved `vid` Block | Target Phase | Priority |
|---|---|---|---:|---|---|---|
| [`PKG-ACT1-EXP03-Z00-04`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp03-z00-04) | `03` | `Z00` Espacio, `Z01` Exosfera, `Z02` Termosfera, `Z03` Mesosfera, `Z04` Estratosfera | 25 | `1300..=1304` | Phase 3 | **Completed (`[TIER-A]` · `1300..=1304`)** |
| [`PKG-ACT1-EXP03-Z05-08`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp03-z05-08) | `03` | `Z05` Troposfera, `Z06` Superficie, `Z07` Corteza, `Z08` Litosfera | 20 | `1305..=1308` | Phase 1 / Phase 3 | **Completed (`[TIER-A]` Reference Exemplar)** |
| [`PKG-ACT1-EXP03-Z09-13`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp03-z09-13) | `03` | `Z09` Astenosfera, `Z10` Zona de transición, `Z11` Manto inferior, `Z12` Núcleo externo, `Z13` Núcleo interno | 25 | `1309..=1313` | Phase 3 | **Completed (`[TIER-A]` · `1309..=1313`)** |
| [`PKG-ACT1-EXP04-Z00-03`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp04-z00-03) | `04` | `Z00` Sol, `Z01` Mercurio, `Z02` Venus, `Z03` Tierra | 20 | `1400..=1403` | Phase 3 | **Completed (`[TIER-A]` · `1400..=1403`)** |
| [`PKG-ACT1-EXP04-Z04-07`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp04-z04-07) | `04` | `Z04` Marte, `Z05` Cinturón de asteroides, `Z06` Júpiter, `Z07` Saturno | 20 | `1404..=1407` | Phase 3 | **Completed (`[TIER-A]` · `1404..=1407`)** |
| [`PKG-ACT1-EXP04-Z08-11`](roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp04-z08-11) | `04` | `Z08` Urano, `Z09` Neptuno, `Z10` Cinturón de Kuiper, `Z11` Plutón | 20 | `1408..=1411` | Phase 3 | **Completed (`[TIER-A]` · `1408..=1411`)** |

### Odyssey Act II — Cosmic Web & Origins (`26` Packages: `05, 21, 22, 06, 07, 08, 09, 10` — `116` Zones, `684` Features)

See full Pick-Up Cards in [`docs/roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md):

| Package ID | Exp | Zones & Catalog Names | Feats | Reserved `vid` Block | Target Phase | Priority |
|---|---|---|---:|---|---|---|
| [`PKG-ACT2-EXP05-Z00-04`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp05-z00-04) | `05` | `Z00` Frontera planetaria, `Z01` Heliosfera, `Z02` Nube de Oort, `Z03` Espacio interestelar, `Z04` Alfa Centauri | 25 | `1500..=1504` | Phase 1 / Phase 3 | **Completed (`[TIER-A]` · `1500..=1504`)** |
| [`PKG-ACT2-EXP05-Z05-08`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp05-z05-08) | `05` | `Z05` Vecindario estelar, `Z06` Burbuja Local, `Z07` Espolón de Orión, `Z08` Guardería estelar | 20 | `1505..=1508` | Phase 1 / Phase 3 | **Completed (`[TIER-A]` · `1505..=1508`)** |
| [`PKG-ACT2-EXP05-Z09-12`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp05-z09-12) | `05` | `Z09` Disco galáctico, `Z10` Centro galáctico, `Z11` Halo galáctico, `Z12` Borde de la Vía Láctea | 20 | `1509..=1512` | Phase 3 | **Completed (`[TIER-A]` · `1509..=1512`)** |
| [`PKG-ACT2-EXP21-Z00-04`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp21-z00-04) | `21` | `Z00` Horizonte desconocido, `Z01` Ingredientes cósmicos, `Z02` Estrellas y estabilidad, `Z03` Sistemas planetarios, `Z04` Zona habitable | 40 | `3100..=3104` | Phase 1 | Critical (Single-Variant Collapse / High Tier-D) |
| [`PKG-ACT2-EXP21-Z05-09`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp21-z05-09) | `21` | `Z05` Habitabilidad real, `Z06` Química prebiótica, `Z07` Origen de la vida, `Z08` Biosferas simples, `Z09` Vida compleja | 40 | `3105..=3109` | Phase 1 | Critical (Single-Variant Collapse / High Tier-D) |
| [`PKG-ACT2-EXP21-Z10-14`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp21-z10-14) | `21` | `Z10` Extremófilos, `Z11` Marte, `Z12` Mundos oceánicos, `Z13` Exoplanetas, `Z14` Biofirmas | 40 | `3110..=3114` | Phase 1 | Critical (Single-Variant Collapse / High Tier-D) |
| [`PKG-ACT2-EXP21-Z15-19`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp21-z15-19) | `21` | `Z15` Falsos positivos, `Z16` Inteligencia evolutiva, `Z17` Formas de inteligencia alienígena, `Z18` Tecnofirmas, `Z19` Paradoja de Fermi | 40 | `3115..=3119` | Phase 1 | Critical (Single-Variant Collapse / High Tier-D) |
| [`PKG-ACT2-EXP21-Z20-24`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp21-z20-24) | `21` | `Z20` El gran filtro, `Z21` Civilizaciones alienígenas, `Z22` Comunicación interestelar, `Z23` Contacto incierto, `Z24` Vida cósmica desconocida | 40 | `3120..=3124` | Phase 1 | Critical (Single-Variant Collapse / High Tier-D) |
| [`PKG-ACT2-EXP22-Z00-04`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp22-z00-04) | `22` | `Z00` Una señal sin explicación definitiva, `Z01` La primera anomalía, `Z02` Detección instrumental, `Z03` Interferencia descartada, `Z04` Verificación independiente | 30 | `3200..=3204` | Phase 1 | Critical (`Z00/Z01` Shared Scene + Single-Variant Collapse) |
| [`PKG-ACT2-EXP22-Z05-09`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp22-z05-09) | `22` | `Z05` Origen distante, `Z06` Patrón artificial, `Z07` Evidencia acumulada, `Z08` Anuncio científico, `Z09` Coordinación internacional | 30 | `3205..=3209` | Phase 1 | Critical (`Z09/Z10` Shared Scene Boundary) |
| [`PKG-ACT2-EXP22-Z10-14`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp22-z10-14) | `22` | `Z10` Reacción planetaria, `Z11` Interpretación, `Z12` Matemáticas compartidas, `Z13` Construcción de significado, `Z14` Lenguaje desconocido | 30 | `3210..=3214` | Phase 1 | Critical (`Z09/Z10` Shared Scene + Cipher Split) |
| [`PKG-ACT2-EXP22-Z15-19`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp22-z15-19) | `22` | `Z15` Contexto cultural, `Z16` Mensaje recibido, `Z17` Decisión de responder, `Z18` Riesgos de transmisión, `Z19` Límites de la velocidad de la luz | 30 | `3215..=3219` | Phase 1 | Critical (`Z15/Z16`, `Z17/Z18`, `Z19/Z20` Shared Pairs) |
| [`PKG-ACT2-EXP22-Z20-23`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp22-z20-23) | `22` | `Z20` Espera interestelar, `Z21` Primera respuesta, `Z22` Diálogo discontinuo, `Z23` Intercambio de conocimiento | 24 | `3220..=3223` | Phase 1 | Critical (`Z19/Z20` Shared Scene Boundary) |
| [`PKG-ACT2-EXP22-Z24-27`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp22-z24-27) | `22` | `Z24` Malentendidos cósmicos, `Z25` Confianza, `Z26` Diplomacia interestelar, `Z27` Coexistencia incierta | 24 | `3224..=3227` | Phase 1 | Critical (Single-Variant Collapse) |
| [`PKG-ACT2-EXP06-Z00-04`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp06-z00-04) | `06` | `Z00` Borde galáctico, `Z01` Halo exterior, `Z02` Galaxias enanas, `Z03` Espacio intergaláctico, `Z04` Nubes de Magallanes | 25 | `1600..=1604` | Phase 3 | **Completed (`[TIER-A]` · `1600..=1604` + `3680..=3684`)** |
| [`PKG-ACT2-EXP06-Z05-08`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp06-z05-08) | `06` | `Z05` Galaxia del Triángulo, `Z06` Galaxia de Andrómeda, `Z07` Centro del Grupo Local, `Z08` Límite del Grupo Local | 20 | `1605..=1608` | Phase 3 | **Completed (`[TIER-A]` · `1605..=1608` + `3685..=3688`)** |
| [`PKG-ACT2-EXP07-Z00-04`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp07-z00-04) | `07` | `Z00` Límite del Grupo Local, `Z01` Volumen local, `Z02` Lámina Local, `Z03` Cúmulo de Virgo, `Z04` Supercúmulo de Virgo | 25 | `1700..=1704` | Phase 3 | **Completed (`[TIER-A]` · `1700..=1704` + `3700..=3704`)** |
| [`PKG-ACT2-EXP07-Z05-09`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp07-z05-09) | `07` | `Z05` Corrientes galácticas, `Z06` Vacío Local, `Z07` Gran Atractor, `Z08` Cuenca de Laniakea, `Z09` Límite de Laniakea | 25 | `1705..=1709` | Phase 3 | **Completed (`[TIER-A]` · `1705..=1709` + `3705..=3709`)** |
| [`PKG-ACT2-EXP08-Z00-03`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp08-z00-03) | `08` | `Z00` Límite de Laniakea, `Z01` Espacio intersupercúmulo, `Z02` Filamentos galácticos, `Z03` Nodos de cúmulos | 20 | `1800..=1803` | Phase 3 | **Completed (`[TIER-A]` · `1800..=1803` + Mode 2)** |
| [`PKG-ACT2-EXP08-Z04-07`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp08-z04-07) | `08` | `Z04` Vacíos cósmicos, `Z05` Paredes galácticas, `Z06` Intersecciones de filamentos, `Z07` Complejos de supercúmulos | 20 | `1804..=1807` | Phase 3 | **Completed (`[TIER-A]` · `1804..=1807` + Mode 2)** |
| [`PKG-ACT2-EXP08-Z08-10`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp08-z08-10) | `08` | `Z08` Red cósmica profunda, `Z09` Escala de homogeneidad, `Z10` Estructura a gran escala | 15 | `1808..=1810` | Phase 3 | **Completed (`[TIER-A]` · `1808..=1810` + Mode 2)** |
| [`PKG-ACT2-EXP09-Z00-03`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp09-z00-03) | `09` | `Z00` Estructura a gran escala, `Z01` Universo profundo, `Z02` Tiempo de retrospección, `Z03` Galaxias lejanas | 20 | `1900..=1903` | Phase 3 | **Completed (`[TIER-A]` · `1900..=1903` + Mode 2, `cmb_alpha=0`)** |
| [`PKG-ACT2-EXP09-Z04-07`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp09-z04-07) | `09` | `Z04` Era de los cuásares, `Z05` Primeras galaxias, `Z06` Época de reionización, `Z07` Edades oscuras | 20 | `1904..=1907` | Phase 3 | **Completed (`[TIER-A]` · `1904..=1907` + Mode 13, pre-star isolation)** |
| [`PKG-ACT2-EXP09-Z08-10`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp09-z08-10) | `09` | `Z08` Fondo cósmico de microondas, `Z09` Horizonte cosmológico, `Z10` Límite del universo observable | 15 | `1908..=1910` | Phase 3 | **Completed (`[TIER-A]` · `1908..=1910` + Mode 5, isolated CMB shell)** |
| [`PKG-ACT2-EXP10-Z00-04`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp10-z00-04) | `10` | `Z00` Fondo cósmico de microondas, `Z01` Recombinación, `Z02` Plasma primordial, `Z03` Nucleosíntesis primordial, `Z04` Era de los leptones | 25 | `2000..=2004` | Phase 3 / Phase 4 | **Completed (`[TIER-A]` · `2000..=2004` + Mode 13, `star_density=0`)** |
| [`PKG-ACT2-EXP10-Z05-08`](roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md#pkg-act2-exp10-z05-08) | `10` | `Z05` Era de los quarks, `Z06` Transición electrodébil, `Z07` Universo inflacionario, `Z08` Límite de la física conocida | 21 | `2005..=2008` | Phase 3 / Phase 4 | **Completed (`[TIER-A]` · `2005..=2008` + Mode 3/5, inflation isolation)** |

### Odyssey Act III — Matter & Spacetime (`7` Packages: `13, 14` — `27` Zones, `187` Features)

See full Pick-Up Cards in [`docs/roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md`](roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md):

| Package ID | Exp | Zones & Catalog Names | Feats | Reserved `vid` Block | Target Phase | Priority |
|---|---|---|---:|---|---|---|
| [`PKG-ACT3-EXP13-Z00-03`](roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp13-z00-03) | `13` | `Z00` Universo residual, `Z01` Materia observable, `Z02` Moléculas, `Z03` Átomos | 24 | `2300..=2303` | Phase 1 / Phase 2 | High (`sc % 5` Modulo Split + `doc_feat` 0..7) |
| [`PKG-ACT3-EXP13-Z04-07`](roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp13-z04-07) | `13` | `Z04` Núcleo atómico, `Z05` Protones y neutrones, `Z06` Quarks y gluones, `Z07` Partículas elementales | 24 | `2304..=2307` | Phase 1 / Phase 2 | High (`sc % 5` Modulo Split + `doc_feat` 0..7) |
| [`PKG-ACT3-EXP13-Z08-11`](roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp13-z08-11) | `13` | `Z08` Cuatro interacciones, `Z09` Campos cuánticos, `Z10` Vacío cuántico, `Z11` Frontera de la física fundamental | 25 | `2308..=2311` | Phase 1 / Phase 2 | High (`sc % 5` Modulo Split + `doc_feat` 0..7) |
| [`PKG-ACT3-EXP14-Z00-03`](roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp14-z00-03) | `14` | `Z00` Frontera de la física fundamental, `Z01` Espacio y tiempo cotidianos, `Z02` Relatividad especial, `Z03` Luz y causalidad | 28 | `2400..=2403` | Phase 1 / Phase 2 | High (`sc % 4` Modulo Split + `doc_feat` 0..7) |
| [`PKG-ACT3-EXP14-Z04-07`](roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp14-z04-07) | `14` | `Z04` El espacio-tiempo, `Z05` Principio de equivalencia, `Z06` Curvatura gravitatoria, `Z07` Agujeros negros | 30 | `2404..=2407` | Phase 1 / Phase 2 | High (`sc % 4` Modulo Split + `doc_feat` 0..7) |
| [`PKG-ACT3-EXP14-Z08-10`](roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp14-z08-10) | `14` | `Z08` Ondas gravitacionales, `Z09` Un universo dinámico, `Z10` Horizontes, temperatura e información | 24 | `2408..=2410` | Phase 1 / Phase 2 | High (`sc % 4` Modulo Split + `doc_feat` 0..7) |
| [`PKG-ACT3-EXP14-Z11-14`](roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp14-z11-14) | `14` | `Z11` La escala de Planck, `Z12` Caminos hacia la gravedad cuántica, `Z13` Espacio-tiempo emergente, `Z14` El origen del espacio y del tiempo | 32 | `2411..=2414` | Phase 1 / Phase 2 | High (`sc % 4` Modulo Split + `doc_feat` 0..7) |

### Odyssey Act IV — Life, Mind & Senses (`14` Packages: `15, 16, 17, 01, 02` — `53` Zones, `417` Features + Spatial Labs)

See full Pick-Up Cards in [`docs/roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md):

| Package ID | Exp | Zones & Catalog Names | Feats | Reserved `vid` Block | Target Phase | Priority |
|---|---|---|---:|---|---|---|
| [`PKG-ACT4-EXP15-Z00-03`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp15-z00-03) | `15` | `Z00` La naturaleza de la realidad, `Z01` Leyes y simetrías, `Z02` Ruptura de simetría, `Z03` Estados colectivos de la materia | 29 | `2500..=2503` | Phase 1 / Phase 2 | High (`sc % 4` Modulo Split + `field_grid_tex`) |
| [`PKG-ACT4-EXP15-Z04-07`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp15-z04-07) | `15` | `Z04` Lejos del equilibrio, `Z05` Fluctuaciones, `Z06` Patrones espontáneos, `Z07` Retroalimentación | 29 | `2504..=2507` | Phase 1 / Phase 2 | High (`sc % 4` Modulo Split + `field_grid_tex`) |
| [`PKG-ACT4-EXP15-Z08-11`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp15-z08-11) | `15` | `Z08` Redes complejas, `Z09` Autoorganización, `Z10` Información funcional, `Z11` Química organizada | 30 | `2508..=2511` | Phase 1 / Phase 2 | High (`sc % 4` Modulo Split + `field_grid_tex`) |
| [`PKG-ACT4-EXP15-Z12-15`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp15-z12-15) | `15` | `Z12` Replicación y variación, `Z13` Compartimentos y protocélulas, `Z14` Evolución química, `Z15` El umbral de la vida | 31 | `2512..=2515` | Phase 1 / Phase 2 | High (`sc % 4` Modulo Split + `field_grid_tex`) |
| [`PKG-ACT4-EXP16-Z00-04`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp16-z00-04) | `16` | `Z00` El umbral de la vida, `Z01` Primeros sistemas vivos, `Z02` Células primitivas, `Z03` Metabolismo, `Z04` Información heredable | 40 | `2600..=2604` | Phase 1 / Phase 2 | High (`sc % 5` Modulo Split + `doc_feat` 0..7) |
| [`PKG-ACT4-EXP16-Z05-09`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp16-z05-09) | `16` | `Z05` Evolución darwiniana, `Z06` Fotosíntesis, `Z07` Células complejas, `Z08` Reproducción sexual, `Z09` Multicelularidad | 40 | `2605..=2609` | Phase 1 / Phase 2 | High (`sc % 5` Modulo Split + `doc_feat` 0..7) |
| [`PKG-ACT4-EXP16-Z10-14`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp16-z10-14) | `16` | `Z10` Diferenciación y desarrollo, `Z11` Organismos, `Z12` Ecosistemas, `Z13` Movimiento, `Z14` Sentidos | 40 | `2610..=2614` | Phase 1 / Phase 2 | High (`sc % 5` Modulo Split + `doc_feat` 0..7) |
| [`PKG-ACT4-EXP16-Z15-18`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp16-z15-18) | `16` | `Z15` Sistemas nerviosos, `Z16` Cerebros y aprendizaje, `Z17` Percepción, `Z18` La experiencia consciente | 33 | `2615..=2618` | Phase 1 / Phase 2 | High (`sc % 5` Modulo Split + `doc_feat` 0..7) |
| [`PKG-ACT4-EXP17-Z00-04`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp17-z00-04) | `17` | `Z00` La experiencia consciente, `Z01` Sensación, `Z02` Percepción, `Z03` Atención, `Z04` Emoción | 40 | `2700..=2704` | Phase 2 | Medium-High (`sc % 5` + Cortical `doc_feat` 0..7) |
| [`PKG-ACT4-EXP17-Z05-08`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp17-z05-08) | `17` | `Z05` Memoria, `Z06` Aprendizaje, `Z07` Conceptos, `Z08` Lenguaje | 32 | `2705..=2708` | Phase 2 | Medium-High (`sc % 5` + Cortical `doc_feat` 0..7) |
| [`PKG-ACT4-EXP17-Z09-12`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp17-z09-12) | `17` | `Z09` Razonamiento, `Z10` Toma de decisiones, `Z11` Imaginación, `Z12` Identidad | 32 | `2709..=2712` | Phase 2 | Medium-High (`sc % 5` + Cortical `doc_feat` 0..7) |
| [`PKG-ACT4-EXP17-Z13-17`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp17-z13-17) | `17` | `Z13` Inteligencia, `Z14` Metacognición, `Z15` Mentes conectadas, `Z16` Cultura, `Z17` El conocimiento compartido | 41 | `2713..=2717` | Phase 2 | Medium-High (`sc % 5` + Cortical `doc_feat` 0..7) |
| [`PKG-ACT4-EXP01-SPATIAL`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp01-spatial) | `01` | Spatial lab: `130` Chromatic Keys, `72` Lattice Nodes (`01_colors`) | — (`130` keys) | `1100..=1199` (comp `3600..=3609`) | Phase 3 | Medium (`130` Color Keys + `3601..=3604` PBR Spheres) |
| [`PKG-ACT4-EXP02-SPATIAL`](roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp02-spatial) | `02` | Spatial lab: `48x4` MIDI/Timbre Filaments, `20` CQT Keys (`02_music`) | — (`48x4` + `20` keys) | `1200..=1299` (comp `3610..=3619`) | Phase 3 | Medium (`48x4` Filaments + `20` CQT Keys + `3610..=3614`) |

### Odyssey Act V — Civilization & Horizons (`32` Packages: `18, 19, 20, 11, 12, 23, 24, 25` — `154` Zones, `1,038` Features)

See full Pick-Up Cards in [`docs/roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md):

| Package ID | Exp | Zones & Catalog Names | Feats | Reserved `vid` Block | Target Phase | Priority |
|---|---|---|---:|---|---|---|
| [`PKG-ACT5-EXP18-Z00-04`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp18-z00-04) | `18` | `Z00` El conocimiento compartido, `Z01` Comunicación, `Z02` Lenguaje compartido, `Z03` Relatos, `Z04` Enseñanza | 40 | `2800..=2804` | Phase 1 / Phase 2 | Critical (`sc % 7` Modulo + High Tier-D) |
| [`PKG-ACT5-EXP18-Z05-09`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp18-z05-09) | `18` | `Z05` Cooperación, `Z06` Normas y confianza, `Z07` Herramientas, `Z08` Arte y simbolismo, `Z09` Intercambio | 40 | `2805..=2809` | Phase 1 / Phase 2 | Critical (`sc % 7` Modulo + High Tier-D) |
| [`PKG-ACT5-EXP18-Z10-14`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp18-z10-14) | `18` | `Z10` Agricultura, `Z11` Asentamientos, `Z12` Escritura, `Z13` Ciudades, `Z14` Instituciones | 40 | `2810..=2814` | Phase 1 / Phase 2 | Critical (`sc % 7` Modulo + High Tier-D) |
| [`PKG-ACT5-EXP18-Z15-19`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp18-z15-19) | `18` | `Z15` Comercio y redes, `Z16` Ciencia, `Z17` Industria, `Z18` Redes globales, `Z19` Civilización planetaria | 40 | `2815..=2819` | Phase 1 / Phase 2 | Critical (`sc % 7` Modulo + High Tier-D) |
| [`PKG-ACT5-EXP19-Z00-04`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp19-z00-04) | `19` | `Z00` Civilización planetaria, `Z01` Herramientas manuales, `Z02` Máquinas simples, `Z03` Medición y precisión, `Z04` Materiales y metalurgia | 40 | `2900..=2904` | Phase 1 / Phase 2 | Critical (`sc % 7` Modulo + High Tier-D) |
| [`PKG-ACT5-EXP19-Z05-09`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp19-z05-09) | `19` | `Z05` Agricultura tecnológica, `Z06` Motores, `Z07` Industria, `Z08` Electricidad, `Z09` Telecomunicaciones | 40 | `2905..=2909` | Phase 1 / Phase 2 | Critical (`sc % 7` Modulo + High Tier-D) |
| [`PKG-ACT5-EXP19-Z10-14`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp19-z10-14) | `19` | `Z10` Electrónica, `Z11` Computación, `Z12` Software, `Z13` Internet, `Z14` Robótica | 40 | `2910..=2914` | Phase 1 / Phase 2 | Critical (`sc % 7` Modulo + High Tier-D) |
| [`PKG-ACT5-EXP19-Z15-19`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp19-z15-19) | `19` | `Z15` Biotecnología, `Z16` Inteligencia artificial, `Z17` Sistemas autónomos, `Z18` Tecnologías planetarias, `Z19` Futuros tecnológicos | 41 | `2915..=2919` | Phase 1 / Phase 2 | Critical (`sc % 7` Modulo + High Tier-D) |
| [`PKG-ACT5-EXP20-Z00-04`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp20-z00-04) | `20` | `Z00` Futuros tecnológicos, `Z01` Transición energética, `Z02` Automatización, `Z03` Inteligencia avanzada, `Z04` Biotecnología futura | 40 | `3000..=3004` | Phase 1 / Phase 2 | Critical (`sc % 4` Modulo + `11/19` Tier-D Zones) |
| [`PKG-ACT5-EXP20-Z05-09`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp20-z05-09) | `20` | `Z05` Colaboración humano–máquina, `Z06` Civilización sostenible, `Z07` Desigualdad tecnológica, `Z08` Riesgos globales, `Z09` Gobernanza planetaria | 40 | `3005..=3009` | Phase 1 / Phase 2 | Critical (`sc % 4` Modulo + High Tier-D) |
| [`PKG-ACT5-EXP20-Z10-14`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp20-z10-14) | `20` | `Z10` Industria espacial, `Z11` Asentamientos extraplanetarios, `Z12` Civilización multiplanetaria, `Z13` Modificación planetaria, `Z14` Inteligencia postbiológica | 40 | `3010..=3014` | Phase 1 / Phase 2 | Critical (`sc % 4` Modulo + High Tier-D) |
| [`PKG-ACT5-EXP20-Z15-18`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp20-z15-18) | `20` | `Z15` Civilización interestelar, `Z16` Escalas de Kardashev, `Z17` Futuros divergentes, `Z18` Horizonte desconocido | 32 | `3015..=3018` | Phase 1 / Phase 2 | Critical (`sc % 4` Modulo + High Tier-D) |
| [`PKG-ACT5-EXP11-Z00-04`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp11-z00-04) | `11` | `Z00` Límite de la física conocida, `Z01` Observación directa, `Z02` Huellas fósiles, `Z03` Inferencias robustas, `Z04` Predicciones comprobables | 26 | `2100..=2104` | Phase 1 / Phase 2 | High (`sc % 4` Modulo + Tier-C/D Evidence Stages) |
| [`PKG-ACT5-EXP11-Z05-08`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp11-z05-08) | `11` | `Z05` Señales aún no detectadas, `Z06` Modelos de gravedad cuántica, `Z07` Condiciones iniciales, `Z08` Frontera de lo conocible | 24 | `2105..=2108` | Phase 1 / Phase 2 | High (`sc % 4` Modulo + Tier-C/D Evidence Stages) |
| [`PKG-ACT5-EXP12-Z00-04`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp12-z00-04) | `12` | `Z00` Frontera de lo conocible, `Z01` Universo actual, `Z02` Ocaso de la formación estelar, `Z03` Era de las enanas rojas, `Z04` Las últimas estrellas | 29 | `2200..=2204` | Phase 1 / Phase 2 | High (`5 \| 12 =>` Bypass Removal + Dead `120..124`) |
| [`PKG-ACT5-EXP12-Z05-09`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp12-z05-09) | `12` | `Z05` Era degenerada, `Z06` Era de los agujeros negros, `Z07` Evaporación de Hawking, `Z08` Era oscura, `Z09` Futuro indeterminado | 30 | `2205..=2209` | Phase 1 / Phase 2 | High (`5 \| 12 =>` Bypass Removal + `3730..=3739`) |
| [`PKG-ACT5-EXP23-Z00-04`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp23-z00-04) | `23` | `Z00` Otra perspectiva sobre la realidad, `Z01` Perspectiva humana, `Z02` Sentidos terrestres, `Z03` Cerebro y percepción, `Z04` Lenguaje | 30 | `3300..=3304` | Phase 1 / Phase 2 | Critical (Single-Variant `vid=230` Collapse) |
| [`PKG-ACT5-EXP23-Z05-09`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp23-z05-09) | `23` | `Z05` Cultura, `Z06` Modelos científicos, `Z07` Límites cognitivos, `Z08` Perspectiva alienígena, `Z09` Sentidos desconocidos | 30 | `3305..=3309` | Phase 1 / Phase 2 | Critical (Single-Variant `vid=230` Collapse) |
| [`PKG-ACT5-EXP23-Z10-14`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp23-z10-14) | `23` | `Z10` Escalas temporales distintas, `Z11` Otras formas de individualidad, `Z12` Inteligencias colectivas, `Z13` Mentes artificiales, `Z14` Inteligencias planetarias | 30 | `3310..=3314` | Phase 1 / Phase 2 | Critical (Single-Variant `vid=230` Collapse) |
| [`PKG-ACT5-EXP23-Z15-19`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp23-z15-19) | `23` | `Z15` Inteligencias distribuidas, `Z16` Conceptos intraducibles, `Z17` Múltiples matemáticas representadas, `Z18` Diferentes modelos de causalidad, `Z19` Cosmologías comparadas | 30 | `3315..=3319` | Phase 1 / Phase 2 | Critical (Single-Variant `vid=230` Collapse) |
| [`PKG-ACT5-EXP23-Z20-24`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp23-z20-24) | `23` | `Z20` Verdades compartidas, `Z21` Desacuerdos fundamentales, `Z22` Conocimiento complementario, `Z23` Realidad observada desde muchas mentes, `Z24` Perspectiva cósmica plural | 30 | `3320..=3324` | Phase 1 / Phase 2 | Critical (Single-Variant `vid=230` Collapse) |
| [`PKG-ACT5-EXP24-Z00-04`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp24-z00-04) | `24` | `Z00` Perspectiva cósmica plural, `Z01` Un universo sin observadores, `Z02` La aparición de la diferencia, `Z03` Sentir convierte estados en experiencias, `Z04` De la preferencia al valor | 30 | `3400..=3404` | Phase 1 / Phase 2 | High (`vid=240` Collapse + `24_cosmic_meaning` Split) |
| [`PKG-ACT5-EXP24-Z05-09`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp24-z05-09) | `24` | `Z05` Metas y agencia, `Z06` El significado personal, `Z07` Significado compartido, `Z08` Relatos y civilizaciones, `Z09` Mortalidad y límite | 30 | `3405..=3409` | Phase 1 / Phase 2 | High (`vid=240` Collapse + `doc_feat` 0..7) |
| [`PKG-ACT5-EXP24-Z10-14`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp24-z10-14) | `24` | `Z10` La insignificancia cósmica, `Z11` El silencio del universo, `Z12` Valores alienígenas, `Z13` Propósitos artificiales, `Z14` Significado colectivo | 30 | `3410..=3414` | Phase 1 / Phase 2 | High (`vid=240` Collapse + `doc_feat` 0..7) |
| [`PKG-ACT5-EXP24-Z15-19`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp24-z15-19) | `24` | `Z15` Conflictos de significado, `Z16` Límites morales, `Z17` Proyectos compartidos, `Z18` Conocimiento como propósito, `Z19` Belleza y contemplación | 30 | `3415..=3419` | Phase 1 / Phase 2 | High (`vid=240` Collapse + `doc_feat` 0..7) |
| [`PKG-ACT5-EXP24-Z20-22`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp24-z20-22) | `24` | `Z20` Cuidado y relación, `Z21` Responsabilidad planetaria, `Z22` Propósito elegido | 18 | `3420..=3422` | Phase 1 / Phase 2 | High (`vid=240` Collapse + `doc_feat` 0..7) |
| [`PKG-ACT5-EXP24-Z23-25`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp24-z23-25) | `24` | `Z23` Significado sin certeza, `Z24` El universo que se valora a sí mismo, `Z25` Hacia el horizonte final | 18 | `3423..=3425` | Phase 1 / Phase 2 | High (`vid=240/241` + `doc_feat` 0..7) |
| [`PKG-ACT5-EXP25-Z00-04`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp25-z00-04) | `25` | `Z00` Una sola historia incompleta, `Z01` Los límites biológicos, `Z02` La transformación de la mente, `Z03` Identidad y continuidad, `Z04` Vida postbiológica | 30 | `3500..=3504` | Phase 1 / Phase 4 | Critical (Single-Variant `vid=250`, `18/25` Tier-D) |
| [`PKG-ACT5-EXP25-Z05-09`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp25-z05-09) | `25` | `Z05` Inteligencias de larga duración, `Z06` Civilizaciones galácticas, `Z07` Ingeniería cósmica, `Z08` Computación y realidad construida, `Z09` La multiplicación de mundos | 30 | `3505..=3509` | Phase 1 / Phase 4 | Critical (Single-Variant `vid=250`, `18/25` Tier-D) |
| [`PKG-ACT5-EXP25-Z10-14`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp25-z10-14) | `25` | `Z10` Mentes fusionadas y divididas, `Z11` Los límites de la experiencia, `Z12` Los límites de la inteligencia, `Z13` Los límites del conocimiento, `Z14` El horizonte cosmológico | 30 | `3510..=3514` | Phase 1 / Phase 4 | Critical (Single-Variant `vid=250`, `18/25` Tier-D) |
| [`PKG-ACT5-EXP25-Z15-19`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp25-z15-19) | `25` | `Z15` Civilizaciones aisladas, `Z16` El presupuesto energético, `Z17` El universo tardío, `Z18` El destino de la información, `Z19` La muerte térmica | 30 | `3515..=3519` | Phase 1 / Phase 4 | Critical (Single-Variant `vid=250`, `18/25` Tier-D) |
| [`PKG-ACT5-EXP25-Z20-24`](roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md#pkg-act5-exp25-z20-24) | `25` | `Z20` Posibilidades desconocidas, `Z21` Existencia sin observadores, `Z22` El legado cósmico, `Z23` La existencia como historia común, `Z24` No vemos más allá del horizonte final. | 30 | `3520..=3524` | Phase 1 / Phase 4 | Critical (Finale Horizon Transitions + `vid=250` Split) |
