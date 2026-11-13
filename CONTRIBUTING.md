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
  cargo test -p cosmos_math -p cosmos_narration -p cosmos_nav
  cargo test -p cosmos_xr -- --skip openxr_static_entry_and_graceful_headset_probe
  cargo test -p cosmos_spatial topology
  cargo test -p cosmos_manifest --test manifest_tests -- --skip real_mmap_catalog_and_audio_pack_verify_all_18898_assets
  ```
- If you have a Discrete DX12 GPU + local fonts + optional `.hcvp` pack, also run the full hardware & visual suite locally:
  ```powershell
  cargo test --workspace -- --test-threads=1
  Start-HyperCosmosVR.cmd --headless-render .\artifacts\check_renders --width 960 --height 540 --benchmark-sweep
  ```

### WGSL Shader Standards
- Review [shaders/SHADER_CONTRACTS.md](shaders/SHADER_CONTRACTS.md) before modifying `.wgsl` files.
- Keep `vs_main` (mono) and `vs_stereo` (`@builtin(view_index)`) signatures consistent across every render pass.
- Preserve `#000000` OLED deep space void discipline (zero gray fog lifting the cosmic background above pure black) and check highlights against AgX blowout.

### UI & Architecture Discipline
- **Zero 2D overlays:** Do not introduce screen-space HTML/DOM/ImGui menus or HUD bars. All controls belong on diegetic 3D spatial anchors (`Wrist Resonance Ring`, gaze dwell, peripheral meridian compass).

---

## 4. Pull Request Process

1. Fork the repository and create a topic branch (`feat/...`, `fix/...`, `docs/...`).
2. Keep changes focused and well-tested; add unit tests for math, FSM, catalog, or DSP changes.
3. Update relevant documentation (`README.md`, `docs/CUSTOMIZATION.md`, or `shaders/SHADER_CONTRACTS.md`) when behavior or CLI flags change.
4. Fill out all sections of `.github/pull_request_template.md`, explicitly noting which tests ran on local GPU hardware vs. CPU-only CI.

---

## 5. Licensing

By submitting a contribution, you agree that your code contributions will be dual-licensed under **MIT OR Apache-2.0** (`LICENSE-MIT` and `LICENSE-APACHE`), consistent with the workspace `Cargo.toml`. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) for existing dependency licenses (including `MPL-2.0` for `symphonia`).

