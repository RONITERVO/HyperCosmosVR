# Third-Party Notices & Supply-Chain License Compliance

`HyperCosmosVR` engine, shader, and Rust workspace source code (`Cargo.toml` workspace) is dual-licensed under **MIT OR Apache-2.0** (`LICENSE-MIT`, `LICENSE-APACHE`).

For legal and open-source compliance teams reviewing downstream forks or commercial products, this document records the exact third-party crates, binary data assets, content sources, and system runtime dependencies verified in the repository.

---

## 1. Direct Rust Crate Dependencies (`Cargo.toml` Workspace)

Extracted directly from `cargo metadata`:

| Crate | Version | License (SPDX) | Upstream Repository | Compliance Notes |
| :--- | :--- | :--- | :--- | :--- |
| `ab_glyph` | `0.2.32` | `Apache-2.0` | `https://github.com/alexheretic/ab-glyph` | Typography layout metrics |
| `anyhow` | `1.0.104` | `MIT OR Apache-2.0` | `https://github.com/dtolnay/anyhow` | Application error handling |
| `bytemuck` | `1.25.2` | `Zlib OR Apache-2.0 OR MIT` | `https://github.com/Lokathor/bytemuck` | Plain-old-data GPU struct casting |
| `cpal` | `0.18.2` | `Apache-2.0` | `https://github.com/RustAudio/cpal` | WASAPI 48 kHz real-time audio I/O |
| `crc32fast` | `1.5.5` | `MIT OR Apache-2.0` | `https://github.com/srijs/rust-crc32fast` | Catalog & audio pack integrity checks |
| `crossbeam-channel` | `0.5.17` | `MIT OR Apache-2.0` | `https://github.com/crossbeam-rs/crossbeam` | Lock-free audio/UI command messaging |
| `fdsm` | `0.8.0` | `MIT` | `https://gitlab.com/Kyarei/fdsm` | Multi-channel SDF glyph atlas generation |
| `fdsm-ttf-parser` | `0.2.0` | `MIT` | `https://gitlab.com/Kyarei/fdsm` | Outline bridging for `fdsm` |
| `fundsp` | `0.23.0` | `MIT OR Apache-2.0` | `https://github.com/SamiPerttu/fundsp` | Real-time App 02 synth & zone bed audio |
| `glam` | `0.34.3` | `MIT OR Apache-2.0` | `https://github.com/bitshifter/glam-rs` | `DVec3`/`DQuat`/`Mat4` 64-bit floating-origin math |
| `hrtf` | `0.10.0` | `MIT` | `https://github.com/mrDIMAS/hrtf` | Binaural spatial audio convolution |
| `image` | `0.25.10` | `MIT OR Apache-2.0` | `https://github.com/image-rs/image` | Headless verification PNG export & blue noise load |
| `memmap2` | `0.9.12` | `MIT OR Apache-2.0` | `https://github.com/RazrFalcon/memmap2-rs` | Zero-copy mmap of `.hcvb` and `.hcvp` |
| `nalgebra` | `0.34.2` | `Apache-2.0` | `https://github.com/dimforge/nalgebra` | Geometry math used with `fdsm` |
| `openxr` | `0.22.0` | `MIT OR Apache-2.0` | `https://github.com/Ralith/openxrs` | Static Khronos OpenXR loader & D3D12 VR bindings |
| `pollster` | `0.4.0` | `Apache-2.0/MIT` | `https://github.com/zesterer/pollster` | Synchronous Future polling for `wgpu` init |
| `rayon` | `1.12.0` | `MIT OR Apache-2.0` | `https://github.com/rayon-rs/rayon` | Parallel MSDF atlas baking & batch prep |
| `rubato` | `5.1.0` | `MIT OR Apache-2.0` | `https://github.com/HEnquist/rubato` | High-quality 24 kHz -> 48 kHz resampling |
| `serde` | `1.0.229` | `MIT OR Apache-2.0` | `https://github.com/serde-rs/serde` | Serialization |
| `serde_json` | `1.0.154` | `MIT OR Apache-2.0` | `https://github.com/serde-rs/json` | Manifest & MSDF metadata JSON parsing |
| `symphonia` | `0.6.1` | **`MPL-2.0`** | `https://github.com/pdeljanov/Symphonia` | **Weak-copyleft MP3 audio decoding (`crates/cosmos_audio`)** |
| `thiserror` | `2.0.21` | `MIT OR Apache-2.0` | `https://github.com/dtolnay/thiserror` | Typed library errors |
| `ttf-parser` | `0.25.1` | `MIT OR Apache-2.0` | `https://github.com/harfbuzz/ttf-parser` | Font glyph coverage & metric extraction |
| `wgpu` / `wgpu-hal` | `30.0.2` | `MIT OR Apache-2.0` | `https://github.com/gfx-rs/wgpu` | DirectX 12 (`DynamicDxc`) + WGSL rendering |
| `windows` / `windows-core` | `0.62.2` | `MIT OR Apache-2.0` | `https://github.com/microsoft/windows-rs` | Win32 / DXGI LUID / D3D12 interop |
| `winit` | `0.30.13` | `Apache-2.0` | `https://github.com/rust-windowing/winit` | Desktop HDR window & input handling |
| `wsola` | `0.3.0` | `MIT` | `https://github.com/jhheider/pdcst` | Pitch-preserving 1.0x–3.0x speech time-stretching |

> **Important Note for Corporate Legal Teams:** `symphonia` (`0.6.1`, pulled in by `cosmos_audio`) is licensed under **`MPL-2.0`** (file-level weak copyleft). Modifying `symphonia` source files triggers MPL-2.0 source-disclosure requirements for those files; using `symphonia` unmodified as a library dependency in a combined MIT/Apache-2.0 binary is permitted under MPL-2.0 Section 3.3.

---

## 2. Binary & Data Assets Included in the Repository

| Path | Origin / License | Notes |
| :--- | :--- | :--- |
| `assets/hrir/irc_1040_c.bin` (`772,656 B`) | IRCAM Listen / `firewheel-ircam-hrtf` (`MIT OR Apache-2.0` compatible binary sphere) | Binaural HRTF sphere (`44.1/48 kHz` resampled at load by `hrtf`). |
| `assets/textures/blue_noise_128.png` (`49,358 B`) | Project texture asset (`MIT OR Apache-2.0`) | `128x128` RGB triangular-PDF dithering texture sampled by `scene_pass.rs` / `post_pass.rs` (`BLUE_NOISE_PNG`). |
| `assets/textures/earth/*.png` | NASA Earth Observatory / MODIS / VIIRS; NASA SVS / NOAA CPC; GEBCO via NASA. Educational visualization under [NASA media usage guidelines](https://www.nasa.gov/nasa-brand-center/images-and-media/). | Derived July 2004 Blue Marble, 2016 Black Marble, 2014-06-02 SVS/CPC clouds with 2002 polar cloud fill, and GEBCO 2008 relief. [Credits, processing and limits](assets/textures/earth/README.md); exact download URLs and SHA-256 in `sources.json`. No NASA endorsement. |
| `assets/baked/universe_catalog.hcvb` & `.v1.json` | Derived from `RONITERVO/Spanish-Quick-Apps` & `RONITERVO/Spanish-Quick-Apps-audio` (`tools/bake_universe.py`) | Zero-copy catalog (`25` experiences, `376` zones, `2,456` features, `148,045` SyncVoice cues). |
| `assets/baked/universe_audio.hcvp` *(GitHub Release `v0.1.0`)* | Derived from `RONITERVO/Spanish-Quick-Apps` & `RONITERVO/Spanish-Quick-Apps-audio` | Concatenated `18,898` MP3 narration recordings (`1,125,699,912 B`). |

---

## 3. Excluded Proprietary / Runtime Dependencies (Staged Locally)

- **Windows System Fonts (`assets/fonts/*.ttf`):** `HyperCosmosVR` does **not** redistribute `Segoe UI`, `Segoe Script`, or `Segoe UI Symbol` (© Microsoft Corporation). `tools/stage_runtime_deps.ps1` copies those fonts locally from the user's own `C:\Windows\Fonts\` directory (or contributors can drop any open-source `.ttf`/`.otf` font set into `assets/fonts/` — see [assets/fonts/README.md](assets/fonts/README.md)).
- **Windows SDK DirectX Shader Compiler (`dxcompiler.dll`, `dxil.dll`):** Located at runtime from the developer/end-user's installed Windows 10/11 SDK (`10.0.26100.0` or latest `x64` SDK kit) via `tools/stage_runtime_deps.ps1` and staged alongside `target/{debug,release}/cosmos_app.exe`.

