# Contributing to HyperCosmosVR

Thank you for your interest in contributing to **HyperCosmosVR**! We welcome contributions from graphics/VR engineers, audio DSP developers, translators, educators, and technical writers.

By participating in this project, you agree to abide by our [Code of Conduct](CODE_OF_CONDUCT.md).

---

## 1. Repository & Binary Hygiene Rules (Strict)

To keep cloning fast (~45 MiB Git repo) and legally clean for both enterprise and individual contributors:

1. **Never commit binaries > 50 MiB** (GitHub hard-blocks files `>= 100 MiB`). The `1.05 GiB` bilingual voice archive `assets/baked/universe_audio.hcvp` belongs only on GitHub Releases (`tools/fetch_audio_pack.ps1`).
2. **Never commit proprietary fonts (`.ttf` / `.otf`)**. Microsoft Windows system fonts (`Segoe UI`, `Segoe Script`, `Segoe UI Symbol`) are staged locally by `tools/stage_runtime_deps.ps1` and excluded via `.gitignore`.
3. **Never commit local machine paths** in `.cargo/config.toml` (use `.cargo/config.toml.example` as a personal template).
4. **Keep upstream content repositories read-only here**: translations, scripts, and audio metadata originate from [RONITERVO/Spanish-Quick-Apps](https://github.com/RONITERVO/Spanish-Quick-Apps) and [RONITERVO/Spanish-Quick-Apps-audio](https://github.com/RONITERVO/Spanish-Quick-Apps-audio) via `tools/bake_universe.py`.

---

## 2. Development Environment Setup (Windows x64)

### Prerequisites
- **OS:** Windows 10/11 (`x86_64-pc-windows-msvc`)
- **Rust:** Stable toolchain (`rustup`, tested with `rustc 1.92.0`, 2024 edition)
- **Visual Studio 2022+ (or Build Tools) with C++ Desktop workload + CMake** (required by `openxr-sys` static loader)
- **Windows 10/11 SDK** (`10.0.26100.0` or newer) providing `dxcompiler.dll` & `dxil.dll` for DX12 `DynamicDxc`
- **Optional (content baking):** Python 3.12+
- **Optional (VR testing):** Active OpenXR runtime (SteamVR, Meta Quest Link, or VirtualDesktopXR)

### First-Time Staging
```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\stage_runtime_deps.ps1
```
Optionally fetch the full `1.05 GiB` narration pack if working on SyncVoice cues or audio decoding:
```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File tools\fetch_audio_pack.ps1
```

---

## 3. Coding, Shader & Verification Standards

### Rust Workspace Standards
- Every pull request MUST compile cleanly with **zero warnings**:
  ```powershell
  cargo clippy --workspace --all-targets -- -D warnings
  ```
- Hardware-independent unit & catalog checks (matches `.github/workflows/ci.yml`):
  ```powershell
  cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav -p cosmos_journey
  cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe
  cargo test -p cosmos_spatial topology
  cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets
  ```
- If you have a Discrete DX12 GPU + local fonts + optional `.hcvp` pack, also run the full hardware & visual suite locally:
  ```powershell
  powershell -NoProfile -ExecutionPolicy Bypass -File tools\test_local.ps1
  Start-HyperCosmosVR.cmd --headless-render .\artifacts\check_renders --width 960 --height 540 --benchmark-sweep
  ```
  `test_local.ps1` runs the complete workspace suite with CPU parallelism, four concurrent DX12 correctness tests, and isolated typography timings and application benchmarks. It warms the shared font cache before parallel app tests and stops on the first failed phase. Use `-CpuThreads 16 -GpuThreads 8` to tune concurrency, or `-Release` for optimized test binaries. The serial diagnostic equivalent remains `cargo test --workspace -- --test-threads=1`; debug assertions remain enabled in the runner's default profile.

### WGSL Shader Standards
- Review [shaders/SHADER_CONTRACTS.md](shaders/SHADER_CONTRACTS.md) before modifying `.wgsl` files.
- Keep `vs_main` (mono) and `vs_stereo` (`@builtin(view_index)`) signatures consistent across every render pass.
- Preserve `#000000` OLED deep space void discipline (zero gray fog lifting the cosmic background above pure black) and check highlights against AgX blowout.

### UI & Architecture Discipline
- **Zero 2D overlays:** Do not introduce screen-space HTML/DOM/ImGui menus or HUD bars. All controls belong on diegetic 3D spatial anchors (`Wrist Resonance Ring`, gaze dwell, peripheral meridian compass).
- **VR-first acceptance target:** PSVR2 at 120 Hz and SteamVR's recommended per-eye resolution, plus higher-end OLED headsets. Preserve visual quality; do not silently reduce resolution or scene richness to pass a timing gate. Report actual eye dimensions, refresh rate, compositor involvement, and whether evidence came from a physical headset or an offscreen reference.
- **Holographic presence:** VR guidance must have real depth, binocular parallax, scene occlusion, and a stable room/world anchor. Moving a flat screen panel into world space is not sufficient. Curved lettering, volumetric nodes, controller rays, and wrist-summoned instruments should behave as things accompanying the viewer in space. Desktop contrast backplates must not leak into VR.
- **Documentary richness:** Judge each scene against the substance of its narration: clear subject composition, feature-specific structure, meaningful motion and causality, multiple viewpoints, and enough detail to reward inspection. Tier-A counts and pixel differences establish coverage/distinctness, not visual quality, perceived aliveness, or educational effectiveness. Include visual review evidence alongside automated checks.

---

## 4. Visual Expansion Roadmap & Claimable Work Packages

Contributors working on documentary shaders, zone disambiguation, or GPU compute choreography should claim **one self-contained 1-PR package** (`3–5` zones or `1` Core P0 prerequisite) from **[`docs/CONTRIBUTOR_VISUAL_ROADMAP.md`](docs/CONTRIBUTOR_VISUAL_ROADMAP.md)** — see **[`PKG-ACT1-EXP03-Z05-08`](docs/roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp03-z05-08)** as the completed `[TIER-A]` reference exemplar (`canonical_zone_vid`, `encode_cue_tags_mask`, `doc_feat_focus`, and GPU pairwise feature regression testing), now joined by all `6` Act I packages (`03_deep_earth` [`Z00-04`](docs/roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp03-z00-04), [`Z09-13`](docs/roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp03-z09-13) and `04_solar_system` [`Z00-03`](docs/roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp04-z00-03), [`Z04-07`](docs/roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp04-z04-07), [`Z08-11`](docs/roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md#pkg-act1-exp04-z08-11)), all `26` Odyssey Act II packages (`05`, `21`, `22`, `06`, `07`, `08`, `09`, `10`), all `7` Odyssey Act III packages (`13_matter_energy` [`Z00-03`](docs/roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp13-z00-03), [`Z04-07`](docs/roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp13-z04-07), [`Z08-11`](docs/roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp13-z08-11) and `14_spacetime` [`Z00-03`](docs/roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp14-z00-03), [`Z04-07`](docs/roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp14-z04-07), [`Z08-10`](docs/roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp14-z08-10), [`Z11-14`](docs/roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md#pkg-act3-exp14-z11-14)), and the first `4` Odyssey Act IV packages (`15_complexity` [`Z00-03`](docs/roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp15-z00-03), [`Z04-07`](docs/roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp15-z04-07), [`Z08-11`](docs/roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp15-z08-11), [`Z12-15`](docs/roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md#pkg-act4-exp15-z12-15)), plus all `4` Life packages (`16_life`, `19` zones, `153` features, including `Z18 F8`), bringing **`204/376` zones (`1,273/2,456` features — `47/85` content packages = `55.3%` — `100%` of `Experiences 03..=10, 13..=16, 21, 22`: all of Acts I, II, III & Act IV Stops 1–2)** to `[TIER-A]`:

- **Master Roadmap & P0 Plumbing Cards:** [`docs/CONTRIBUTOR_VISUAL_ROADMAP.md`](docs/CONTRIBUTOR_VISUAL_ROADMAP.md) (`3` Core Pipeline P0 packages + `88`-package master index, `1100..=3869` pre-allocated shader ID registry, and 4-phase rollout plan).
- **Act Gap Maps (in `ODYSSEY_ACTS` travel order):**
  - Act I (`03, 04` — `6` pkgs, `6` done = `100%` `[TIER-A]`): [`docs/roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md`](docs/roadmap/ACT_1_EARTH_AND_SOLAR_SYSTEM_MAP.md)
  - Act II (`05, 21, 22, 06, 07, 08, 09, 10` — `26` pkgs, `26` done = `100%` `[TIER-A]`): [`docs/roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md`](docs/roadmap/ACT_2_COSMIC_WEB_AND_ORIGINS_MAP.md)
  - Act III (`13, 14` — `7` pkgs, `7` done = `100%` `[TIER-A]`): [`docs/roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md`](docs/roadmap/ACT_3_MATTER_AND_SPACETIME_MAP.md)
  - Act IV (`15, 16, 17, 01, 02` — `14` pkgs, `8` done = `57.1%` `[TIER-A]` / `15, 16` at `100%`): [`docs/roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md`](docs/roadmap/ACT_4_LIFE_MIND_AND_SENSES_MAP.md)
  - Act V (`18, 19, 20, 11, 12, 23, 24, 25` — `32` pkgs): [`docs/roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md`](docs/roadmap/ACT_5_CIVILIZATION_AND_HORIZONS_MAP.md)

### Two Parallel Contribution Tracks (Engineers & Non-Technical Teams)

- **Corporate Multi-Team / Engineering Track (Collision-Free Claiming):**
  1. Check open Issues and PRs for `[PKG-...]` to ensure no other team has claimed that slice, and confirm prerequisite Core `P0` packages (`#pkg-p0-dispatch-and-ids`, `#pkg-p0-feature-and-cue-plumbing`, `#pkg-p0-catalog-scene-and-ci`) have landed or are stacked cleanly.
  2. Open a claim Issue or Draft PR titled `[PKG-ACTx-EXPyy-Zaa-bb] <short title>` on a package branch (e.g., `feat/pkg-act5-exp23-z00-04`), staying strictly inside that package's reserved `1100..=3599` / `3600..=3869` ID block.
- **No-Code / Educational / Visual Design Track:**
  1. Author or curate guided `.hcj.json` exhibits (`assets/journeys/` or [`docs/JOURNEY_PLUGINS.md`](docs/JOURNEY_PLUGINS.md)) and review domain accuracy of the `2,456` feature Storyboard Blueprints in `docs/roadmap/ACT_*.md` before shader coding starts.
  2. Run visual QA captures (`Start-HyperCosmosVR.cmd --headless-render .\artifacts\check_renders`) to audit muted visual self-sufficiency, scientific accuracy, and typography legibility, or contribute script/translation refinements in the upstream content repositories (`Spanish-Quick-Apps` / `Spanish-Quick-Apps-audio`).

**Shader & Pipeline Package Acceptance Requirements:**
1. Follow the standard 4-file touch pattern (`crates/cosmos_narration/src/resolver.rs`, `crates/cosmos_render/src/doc_stages.rs`, `shaders/hero_doc_*.wgsl` + `shaders/doc_common.wgsl`, and `shaders/compute_universe.wgsl` / `shaders/instanced_sprites.wgsl` / `crates/cosmos_render/src/universe_scene.rs`) using pre-allocated IDs (`1000 + exp_id * 100 + zone_index` in `1100..=3599`; companions `3600..=3869`) and distinct `doc_feat` transitions for every catalog feature (including ninth features such as Life `Z18 F8`).
2. Preserve **OLED pure-black void** (existing `43` baseline PNGs hold `>= 50.90%` `#000000` with `50.9004%` min in `06_cmb_and_inflation.png`; newly upgraded roadmap packages must achieve **`>= 50.91%` `#000000`**, `0.0000%` highlight blowout) and **VR 90/120 Hz budget (`<= 28` raymarch steps for new hero shells; RTX 5070 baseline `min = 0.837 ms`, `mean = 1.974 ms`, `p99 = 5.109 ms`, `max = 5.188 ms`)**.
3. Run `cargo run -p cosmos_app -- --headless-render artifacts/verification_renders --benchmark-sweep`, `cargo clippy --workspace --all-targets -- -D warnings`, and the workspace / `.github/workflows/ci.yml` tests before opening your PR.

---

## 5. Pull Request Process

1. Fork the repository and create a topic branch (`feat/...`, `fix/...`, `docs/...`).
2. Keep changes focused and well-tested; add unit tests for math, FSM, catalog, or DSP changes.
3. Update relevant documentation (`README.md`, `docs/CUSTOMIZATION.md`, or `shaders/SHADER_CONTRACTS.md`) when behavior or CLI flags change.
4. Fill out all sections of `.github/pull_request_template.md`, explicitly noting which tests ran on local GPU hardware vs. CPU-only CI.

---

## 6. Licensing

By submitting a contribution, you agree that your code contributions will be dual-licensed under **MIT OR Apache-2.0** (`LICENSE-MIT` and `LICENSE-APACHE`), consistent with the workspace `Cargo.toml`. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for existing dependency licenses (including `MPL-2.0` for `symphonia`).

