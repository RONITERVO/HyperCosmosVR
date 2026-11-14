# Customization & Adaptation Guide

This guide explains how to adapt **HyperCosmosVR** for your organization, museum exhibit, VR classroom, or custom experience — starting with zero-code changes first and moving down to Rust/WGSL code customization.

---

## Fast "I Just Want to Change X" Lookup

| What you want to change | Effort / Rebuild? | Exact Action |
| :--- | :--- | :--- |
| **Switch caption language (`EN` / `FI`) live** | **Zero code (Runtime)** | Press `L` on desktop or touch the `EN` / `FI` bead on the 3D Wrist Resonance Ring (`Right-Hold` / `Tab`, or palm-up in VR). |
| **Adjust speech playback speed (`1.0×`–`3.0×`)** | **Zero code (Runtime)** | Press `[` / `]` (or `-` / `=`), or touch the 9 speed beads on the Wrist Resonance Ring (pitch-preserved via `wsola`). |
| **Run silently / spatial synth only (skip `1.05 GiB` download)** | **Zero code (Launch)** | Skip `tools\fetch_audio_pack.ps1` (or launch without `universe_audio.hcvp`); app boots cleanly in visual + synth mode. |
| **Force Desktop HDR even with headset attached** | **Zero code (CLI)** | `.\Start-HyperCosmosVR.cmd --desktop --hdr-surface` |
| **Record & share a custom museum/lesson route in 2 min** | **Zero code (Runtime)** | Fly + zoom, press `R` (`+WPT`) or `Shift+R` (`TRACE`), then `Ctrl+S` / `F9` (`SHARE` exports `user_journeys/<id>.hcj.json` + `hypercosmos://` URI). |
| **Auto-play a custom `.hcj.json` or share link (Kiosk mode)** | **Zero code (CLI + JSON)** | `.\Start-HyperCosmosVR.cmd --desktop --journey user_journeys\my_tour.hcj.json --autoplay-journey` (edit dwell sec / chapter titles right in JSON). |
| **Use custom brand or open-source fonts** | **Zero code (Assets)** | Replace the 5 `.ttf` files in `assets/fonts/` ([manifest.json](../assets/fonts/manifest.json)), delete `assets/baked/msdf_cache/`, relaunch. |
| **Deploy offline / shared drive (`HYPERCOSMOS_ROOT`)** | **Zero code (Env var)** | `$env:HYPERCOSMOS_ROOT = "D:\Exhibits\MuseumPackA"; .\cosmos_app.exe --desktop` |
| **Swap scripted text, translations, or audio recordings** | **Zero Rust (Python bake)** | `python tools\bake_universe.py --sqa-root <path> --sqaa-root <path>`, then run `cargo test -p cosmos_manifest`. |
| **Change cold-start scale, dwell timing, or HDR accent** | **1-line Rust recompile** | Edit constants in [Part 2](#part-2--engineer-customizations-rust--wgsl) (`flight.rs`, `fsm.rs`, `elastic.rs`, `universe_scene.rs`) + `cargo build --release`. |
| **Upgrade a zone to 1:1 documentary hero graphics** | **4-file Rust/WGSL PR** | Claim a `PKG-...` package in [CONTRIBUTOR_VISUAL_ROADMAP.md](CONTRIBUTOR_VISUAL_ROADMAP.md) (`doc_stages.rs` + `hero_doc_*.wgsl`). |

---

## Part 1 — Non-Code & Asset Customizations

### 1. Choose Visual-Only Mode vs. Full Narrated Audio Mode
- **Visual + Spatial Synth Mode (Default right after `git clone`):**
  If `assets/baked/universe_audio.hcvp` is not present, `HyperCosmosVR` automatically starts in visual + procedural spatial synthesis mode (using `fundsp` + IRCAM HRTF `irc_1040_c.bin`) and logs a one-line notice to `stderr`.
  This lets designers, kiosk operators, or CI pipelines preview all **25 experiences** visually without downloading the `1.05 GiB` bilingual voice archive.
- **Full 18,898-Recording Bilingual Narration Mode:**
  Run:
  ```powershell
  powershell -NoProfile -ExecutionPolicy Bypass -File tools\fetch_audio_pack.ps1
  ```
  The script downloads `universe_audio.hcvp` (`1,125,699,912` bytes) from GitHub Release `v0.1.0` and verifies its SHA-256 checksum automatically.

### 2. Swap Fonts & Brand Typography (`assets/fonts/`)
You do not need to recompile shaders or install build tools just to refresh glyph fonts:
1. Check [assets/fonts/manifest.json](../assets/fonts/manifest.json) for the 5 font roles and required glyph coverage (`required_coverage`, including Spanish `áéíóúüñ¿¡`, Finnish `äöå`, math/unit superscripts `⁻²³⁵`, and symbols `Ω♯→·×`).
2. Replace any of the 5 `.ttf` files in `assets/fonts/`:
   - `display_script.ttf`
   - `display_script_bold.ttf`
   - `ui_sans.ttf`
   - `ui_sans_bold.ttf`
   - `symbols_fallback.ttf`
3. Delete `assets/baked/msdf_cache/` so `crates/cosmos_typo` detects the change.
4. Launch `HyperCosmosVR` — `MsdfAtlas::load_or_bake` automatically regenerates the MSDF distance-field atlas on startup.

### 3. Override the Asset Directory (`HYPERCOSMOS_ROOT`)
For shared corporate network drives, USB deployments, or side-by-side content packs, set `HYPERCOSMOS_ROOT` before launching `cosmos_app.exe` (resolved in [crates/cosmos_app/src/assets.rs](../crates/cosmos_app/src/assets.rs)):
```powershell
$env:HYPERCOSMOS_ROOT = "D:\Exhibits\MuseumPackA"
.\target\release\cosmos_app.exe --desktop
```
The target folder must contain `assets/baked/universe_catalog.hcvb`, `assets/fonts/` (or auto-staged cache), `assets/hrir/irc_1040_c.bin`, `assets/textures/blue_noise_128.png`, and `shaders/`.

### 4. Generate High-Resolution Screenshots & GPU Benchmarks (Marketing / QA)
Capture all verification viewpoints across all 25 experiences headlessly on DX12 without opening a window:
```powershell
Start-HyperCosmosVR.cmd --headless-render .\my_renders --width 1920 --height 1080 --benchmark-sweep
```
- CLI flags (`crates/cosmos_app/src/main.rs`):
  - `--desktop`: Force interactive desktop HDR window even if OpenXR detects a VR runtime.
  - `--hdr-surface`: Prefer `Rgba16Float` HDR swapchain surface on supported desktop displays.
  - `--headless-render <dir>`: Render verification viewpoints to PNG files into `<dir>`.
  - `--benchmark-sweep`: Include the 120-frame scale-sweep latency report.
  - `--width <px> --height <px>`: Override headless output dimensions.

### 5. Re-Bake Custom Text, Translations, or Audio (`tools/bake_universe.py`)
All runtime narration text, word-by-word SyncVoice timestamps (`syncvoice-cues-v1`), zones, features, metrics, and MP3 slices are packed from [Spanish-Quick-Apps](https://github.com/RONITERVO/Spanish-Quick-Apps) and [Spanish-Quick-Apps-audio](https://github.com/RONITERVO/Spanish-Quick-Apps-audio) into:
- `assets/baked/universe_catalog.v1.json` (human-inspectable JSON mirror)
- `assets/baked/universe_catalog.hcvb` (zero-copy mmap catalog)
- `assets/baked/universe_audio.hcvp` (concatenated MP3 archive)

Point the baker at your customized content fork using CLI flags or environment variables:
```powershell
python tools\bake_universe.py `
  --sqa-root D:\Content\MyCustom-Spanish-Quick-Apps `
  --sqaa-root D:\Content\MyCustom-Spanish-Quick-Apps-audio
```
Or via `HYPERCOSMOS_SQA_ROOT` / `HYPERCOSMOS_SQAA_ROOT`. Remember to re-run `cargo test -p cosmos_manifest` afterwards to verify CRC32 integrity and table invariants.

### 6. Create, Edit, or Auto-Play Guided Tours (`assets/journeys/*.hcj.json`)
Non-technical educators, curators, or museum operators can customize automated flights with zero code changes:
- **Record Inside the Simulation:** Launch normally and press `R` (`+WPT` bead) to capture keyframe viewpoints, or `Shift+R` (`TRACE` bead) to record continuous flight and scale dives (`u`), then press `Ctrl+S` / `F9` (`SHARE` bead) to export `user_journeys/<id>.hcj.json` + `.share.txt` (and copy the `hypercosmos://` URI to the Windows clipboard).
- **Drop-In File Sharing:** Place any `.hcj.json` file under `assets/journeys/` or `user_journeys/`; `UniverseRuntime::new` discovers it automatically at launch. See [assets/journeys/cosmic_highlights_tour.hcj.json](../assets/journeys/cosmic_highlights_tour.hcj.json) and [docs/JOURNEY_PLUGINS.md](JOURNEY_PLUGINS.md) for the full `schema_version = 1` format.
- **Kiosk / Exhibit Autoplay:** Launch straight into an automated tour with `--journey <id|path|uri> --autoplay-journey`:
  ```powershell
  .\Start-HyperCosmosVR.cmd --desktop --journey cosmic_highlights_tour --autoplay-journey
  ```

---

## Part 2 — Engineer Customizations (Rust & WGSL)

| Goal | File / Symbol | What to Adjust |
| :--- | :--- | :--- |
| **Add or customize Rust Story-Journey plugins** | [crates/cosmos_journey/src/plugin.rs](../crates/cosmos_journey/src/plugin.rs), [odyssey.rs](../crates/cosmos_journey/src/odyssey.rs) (`StoryJourneyPlugin`, `GrandCosmicOdysseyPlugin`) | Implement `StoryJourneyPlugin` and register it in `JourneyRegistry::with_builtins()`. |
| **Tune elastic override leash & magnetic spring pull** | [crates/cosmos_journey/src/elastic.rs](../crates/cosmos_journey/src/elastic.rs) (`ElasticOverrideConfig::default()`) | `rejoin_tau_sec` (`0.42`), `active_pull_tau_sec` (`3.6`), `leash_dist_ru` (`1.25`), `leash_angle_rad` (`36 deg`), `leash_scale_du` (`0.75`), `leash_hold_sec` (`1.15`). |
| **Change Rest-to-Learn dwell timing or gaze tolerance** | [crates/cosmos_narration/src/fsm.rs](../crates/cosmos_narration/src/fsm.rs) (`FsmConfig::default()`) | `settling_end_ms` (`200.0`), `lock_dwell_ms` (`900.0`), `cancel_fade_ms` (`120.0`), `entry_cone_deg` (`2.4`), `exit_cone_multiplier` (`1.35`), linear/scale/angular motion thresholds. |
| **Change starting scale or Earth orbit position** | [crates/cosmos_nav/src/flight.rs](../crates/cosmos_nav/src/flight.rs) | `COLD_START_SCALE_EXP` (`6.6`), `EARTH_ORBIT_HOME_LOCAL_POS` (`(0.0, 0.0, 3.0)`), `DEFAULT_INERTIAL_TAU_SEC` (`0.18`), `MIN_SCALE_EXP` (`-36.0`), `MAX_SCALE_EXP` (`30.0`). |
| **Edit per-experience HDR accent colors** | [crates/cosmos_render/src/universe_scene.rs](../crates/cosmos_render/src/universe_scene.rs) (`experience_accent_linear`) | Returns linear RGB HDR values per experience (`1..=25`) driving halo tint, beacon glow, and bloom energy. |
| **Adjust VR/Desktop spatial anchors or bridges** | [crates/cosmos_spatial/src/topology.rs](../crates/cosmos_spatial/src/topology.rs) (`ManifoldArchetype`, `ContinuityEdge`) | Controls how each of the 25 experiences attaches to the continuous logarithmic scale manifold. |
| **Adjust OLED tonemapping / `#000000` black gate** | [shaders/post_bloom_agx.wgsl](../shaders/post_bloom_agx.wgsl) & [shaders/SHADER_CONTRACTS.md](../shaders/SHADER_CONTRACTS.md) | Bloom prefilter threshold (`>= 1.05`), Troy Sobotka AgX tonemap curve, `#000000` void floor clamp, blue-noise dither amplitude. |

### Adding or Modifying WGSL Documentary Shaders
Read [shaders/SHADER_CONTRACTS.md](../shaders/SHADER_CONTRACTS.md) before editing `.wgsl` files:
1. Respect the strict hero shader concatenation order:
   `common_oled.wgsl` -> `doc_common.wgsl` -> `hero_doc_cosmos.wgsl` -> `hero_doc_matter_mind.wgsl` -> `hero_doc_civilization_horizon.wgsl` -> `hero_volumetric.wgsl`.
2. Preserve both mono (`vs_main`) and OpenXR stereo multiview (`vs_stereo`) entry points so Desktop and VR pipelines never diverge.
3. Preserve true `#000000` OLED void pixels (`0.0000%` highlight blowout across all scenes).

