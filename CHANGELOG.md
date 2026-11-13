# Changelog

All notable changes to **HyperCosmosVR** are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.1.0] — Initial Public Release

### Added
- **10-Crate Native Rust Workspace (`edition = "2024"`):** `cosmos_app`, `cosmos_audio`, `cosmos_manifest`, `cosmos_math`, `cosmos_narration`, `cosmos_nav`, `cosmos_render`, `cosmos_spatial`, `cosmos_typo`, and `cosmos_xr`.
- **DirectX 12 (`DynamicDxc`) + OpenXR Multiview Stereo Engine:**
  - `wgpu 30.0.2` DX12 pipeline with explicit DXGI Adapter LUID matching for OpenXR (`openxr 0.22.0` statically linked) plus graceful Desktop HDR 6DoF fallback (`--desktop`, `--hdr-surface`).
  - Camera-relative Reversed-Z infinite far plane in `f64` logarithmic scale coordinates (`u = -36..+30`, centered at Earth Low Orbit `u = +6.6`).
  - Troy Sobotka AgX OLED tonemapping + tight-threshold Dual-Kawase bloom + `128x128` RGB blue-noise TPDF dithering + strict `#000000` OLED void clamp (`post_bloom_agx.wgsl`).
- **Complete 25-Experience Documentary Cosmos (`Spanish-Quick-Apps` Content Parity):**
  - Zero-copy mmap catalog (`universe_catalog.hcvb` & `universe_catalog.v1.json`) spanning **25 experiences**, **376 zones**, **2,456 features**, **6,415 key entries**, **18,898 bilingual audio assets** (`6,068 es-ES`, `6,415 en-US`, `6,415 fi-FI`), and **148,045 word-level SyncVoice UTF-8 cues**.
  - 6-module concatenated WGSL documentary hero pipeline covering deep Earth interior, Solar System (`04`), Milky Way/Local Group/Laniakea/Cosmic Web/CMB/Inflation (`05..10`), Quantum/Matter/Spacetime (`13..14`), Complexity/Life/Mind/Prism/Harp (`15..17`, `01..02`), Culture/Tech/Futures/Cosmic Life/SETI Contact (`18..22`), and Hyper-Horizon Lotus Worlds (`11..12`, `23..25`).
- **Zero-2D-UI Diegetic Interaction & 3D Typography:**
  - Rest-to-Learn FSM (`Moving -> Settling (0..200 ms) -> Locked (200..900 ms) -> Narrating -> Complete`) with region-heading/shared-context skip memory.
  - 3D world-space MSDF live-ink glyphs (`fdsm 0.8.0`) synced word-by-word to audio clocks and leader tethers.
  - Diegetic 3D **Wrist Resonance Ring** (`9` pitch-preserving `wsola 0.3.0` speed beads `1.0x..3.0x`, `EN`/`FI` locale pearls, quick-return Earth anchor).
  - Real-time spatial audio (`symphonia 0.6.1`, `rubato 5.1.0`, `hrtf 0.10.0` with IRCAM `irc_1040_c.bin`, `fundsp 0.23.0` synth, WASAPI `cpal 0.18.2`), plus graceful visual + synth mode when optional `universe_audio.hcvp` is not installed.
- **Distribution, Legal Compliance & CI:**
  - Dual `MIT OR Apache-2.0` licensing (`LICENSE-MIT`, `LICENSE-APACHE`), `THIRD_PARTY_NOTICES.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, `CONTRIBUTING.md`, `docs/CUSTOMIZATION.md`.
  - Automated dependency/font stager (`tools/stage_runtime_deps.ps1`), GitHub Release `universe_audio.hcvp` SHA-256 fetcher (`tools/fetch_audio_pack.ps1`), configurable `tools/bake_universe.py`, and `.github/workflows/ci.yml`.

