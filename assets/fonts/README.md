# Font Provisioning (`assets/fonts/`)

`HyperCosmosVR` ships `manifest.json` describing the five font roles used by the 3D MSDF live-ink typography baker (`crates/cosmos_typo`):

| Role filename | Default Windows system source | Visual purpose |
|---|---|---|
| `display_script.ttf` | `segoesc.ttf` (Segoe Script) | Handwritten organic headline display script |
| `display_script_bold.ttf` | `segoescb.ttf` (Segoe Script Bold) | Heavy display headlines and milestone titles |
| `ui_sans.ttf` | `segoeui.ttf` (Segoe UI) | Clean metric readouts and crisp subtitles |
| `ui_sans_bold.ttf` | `segoeuib.ttf` (Segoe UI Bold) | Emphasized spatial callouts and ring labels |
| `symbols_fallback.ttf` | `seguisym.ttf` (Segoe UI Symbol) | Mathematical units (`⁻²³⁵`), arrows (`→`), Greek/accents (`Ω`, `ñ`, `ä`) |

## Why `.ttf` Binaries Are Not Committed to Git

Microsoft system fonts (`Segoe UI`, `Segoe Script`, `Segoe UI Symbol`) carry proprietary Microsoft licenses (`© Microsoft Corporation. All Rights Reserved.`) and must not be redistributed in a public open-source repository.

When you run `Start-HyperCosmosVR.cmd` or `powershell -ExecutionPolicy Bypass -File tools\stage_runtime_deps.ps1`, the script automatically copies these five `.ttf` files from your local `C:\Windows\Fonts\` installation into `assets/fonts/` (which is `.gitignore`d).

On the first application launch, `crates/cosmos_typo` bakes a high-precision multi-channel signed distance field (MSDF) atlas (`assets/baked/msdf_cache/msdf_atlas_v1.bin` and `msdf_atlas_v1.json`) covering Latin, Spanish diacritics (`áéíóúüñ¿¡`), Finnish diacritics (`äöå`), and technical/musical notation.

## Using Custom or Open-Source Fonts

To brand or customize typography, replace any of the five filenames in `assets/fonts/` with your own `.ttf`/`.otf` font (for example **Inter**, **Caveat**, **Noto Sans Symbols 2**, or a corporate typeface), delete `assets/baked/msdf_cache/`, and launch the app cleanly to re-bake the MSDF atlas automatically.

Ensure your replacement font chain covers all characters in `assets/fonts/manifest.json` (`required_coverage`).

