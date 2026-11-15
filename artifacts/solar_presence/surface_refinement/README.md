# Solar surface refinement

This evidence records the surface/material refinement after baseline
`9926ad17b263fda086b142f107287c111b2418b0`. The [earlier report and preview](../README.md)
remain historical evidence for that baseline. This update improves procedural
surface structure and reduces redundant strand rendering; it does not close
the native 120 Hz gap or establish final artistic and learning acceptance.

## Retained changes

Granule lane shading now uses half the gap between the two lowest cell powers.
Removing the runner-up-dependent denominator avoids a finite thermal jump
when the second-nearest cell changes. The new coordinate is not Euclidean
distance to a lane. Bright plumes vary across each cell and develop two shaped
daughter lobes with a cooling intrusion. These are explanatory appearances
within the evolving power cell, not a convection simulation. At a winner
boundary, the quiet downflow temperature joins continuously. RGB ensemble
normalization follows the morphology footprint before detail fades out.

Penumbral material has rounded filament heads, darker internal cores,
structured material between filaments, and tails that taper both width and
thermal amplitude. The curved spot coordinates and periodic angular material
retain a fixed physical frame. Each spot's 24-knot unresolved mean table now
comes from the actual GPU material at 8,192 angles and three physical times;
it is not an exact CPU reproduction or calibration against observations.

Strand proxy bounds use the actual perspective tangent extrema of finite end
caps. Closed-loop emission uses a specialized fragment pipeline for the first
49,152 segments, followed by the remaining families in the same order. The
vertex program, material equations, blend targets, 122,880 total segments,
volume samples, output resolution, physical dimensions and stereo units stay
the same. No experimental thermal buffer, added angular texture cache or
granule-neighbor pruning is retained. See the
[physical/material contract](../../../docs/SOLAR_PHYSICAL_RENDERING.md) and
[shader contract](../../../shaders/SHADER_CONTRACTS.md).

## Native-size rendering reference

The [capture log](native_capture.txt) records all six observations on an
RTX 5070 at **3400×3468 per eye**, 64 mm synthetic IPD, symmetric 72° vertical
and 70.924° horizontal FOV. Each static view measured 120 frames, simulation
time 30.000–33.967 seconds, with GPU profiling enabled and video disabled.
These are **CPU submission plus GPU wait** measurements; readback is excluded.
There is no headset session or compositor in this run.

| Observation | Mean ms | p95 ms |
| --- | ---: | ---: |
| Whole Sun | 6.953 | 7.153 |
| Interior | 13.611 | 13.846 |
| Granulation | 13.513 | 13.959 |
| Active region | 22.536 | 22.849 |
| Eruption | 17.900 | 18.391 |
| Open outflow | 11.031 | 11.327 |

Only the whole-Sun reference fits the nominal 8.33 ms interval for 120
application FPS. The other five exceed it before application/compositor
overhead; the active-region gap remains largest. These short references
neither prove a native 30 FPS floor nor establish a headset refresh or
reprojection rate. Surface materials changed as well as rendering code, so
comparisons with the baseline are not equal-image optimization measurements.
The previous OpenXR probe failed at `xrCreateInstance`; no new native-headset
operation or acceptance is claimed here.

Reproduce with a fresh output directory:

```powershell
$env:HYPERCOSMOS_GPU_PROFILE='1'
cargo run --release -p cosmos_app --example earth_review -- target/solar_refinement_reproduction 3400 120 exp04_z000_sun,solar_ 30 3468 0 4 static no-video
Remove-Item Env:HYPERCOSMOS_GPU_PROFILE
```

## Original-pixel detail

The two crops below retain exact 1:1 pixels from eye 0, frame 119, simulation
time 33.966667 seconds. Each is 1024×1024, saved losslessly without resizing
or color adjustment. The complete six-view original frames and both-eye
camera records remain under `target/solar_loop_specialized`.

| Crop | Source rectangle `(left, top, right, bottom)`, exclusive right/bottom |
| --- | --- |
| [Granulation](granulation_native_crop.png) | `(1100, 2200, 2124, 3224)` |
| [Full penumbra region](penumbra_native_crop.png) | `(448, 2378, 1472, 3402)` |

Both crops were inspected after extraction and verified against the original
pixel values. The penumbra crop is centered on the complete spot region;
it differs from the earlier report's crop rectangle. These samples show the
actual filtered visible-light/illustrative-plasma composite. They do not
establish motion stability, headset comfort or visual learning without words.
The [manifest](manifest.json) records every native eye-0 source hash, crop
rectangle/output hash, log provenance and the frozen source-file hashes.

## Compact motion preview

The [16-second eye-0 preview](surface_refinement_preview.mp4) contains two
eight-second excerpts, at 1280×720 and 30 fps (480 frames, 19,447,134 bytes).
It has no narration or added labels. This is a lossy mono review copy decoded
from the source MP4s and re-encoded with CPU libx264, preset `fast`, CRF 18;
it does not preserve original renderer pixels.

| Preview time | Observation | Source simulation time |
| --- | --- | --- |
| 0–8 s | Granulation | 10–18 s |
| 8–16 s | Active region | 18–26 s |

`ffprobe` verified BT.709 primaries/matrix, sRGB transfer and limited YUV range
on both source clips and the packaged preview. The
[preview manifest](preview_manifest.json) records source ranges/hashes, encoder
command, tool versions and the actual probe output. The excerpts do not
substitute for the complete stereo cycles, native headset viewing or motion
acceptance; their encoding-active timings are not benchmarks.

## Numerical evidence and pending acceptance

The [earlier three-test surface probe](surface_probe_prior_penumbra.txt)
passed with the current granulation and the preceding penumbral version:

- Actual granule boundaries: 2,015–2,021 confirmed winner changes per time,
  for each of macro/F1 at three physical times. The shader bisects each sphere
  arc to float precision; the largest spatial step was `1.333e-7 R` and the
  largest luminance jump was 0.00686% of the quiet reference, below the 0.5%
  gate. This checks actual material across boundaries, not assumed nearby pairs.
- Quiet surface mean/LOD: 16,384 directions across 15 footprints and three
  physical times; maximum measured luminance drift was 0.329% macro and
  0.782% F1, below the 3.5% gate. Per-channel drift and adjacent-footprint
  changes have separate guards. These are finite ensemble checks, not absolute
  solar radiometry or proof that every moving pixel is stable.
- Its penumbral result is **superseded**, not the final material validation.

The [final calibrated penumbral probe](penumbra_probe_final.txt) passed:
282 cases × 8,192 angles, covering both spots, three physical times and all
47 radial knots/midpoints. Maximum mean-temperature bias was 14.8528 K and
outer coverage error 0.017639, within the unchanged 20 K / 0.02 budgets.
The [calibration record](penumbra_gpu_calibration.json) contains the GPU-derived
tables and their source hashes. Temperature bias describes the procedural
material's resolved-to-filtered mean, not error against a measured sunspot.

Workspace all-target Clippy passed with warnings denied in 3.41 seconds.
The [source-freeze local suite](final_test_log.txt) passed in 307.64 seconds,
including all three final surface tests in 1.78 seconds. The DX12 phase took
163.69 seconds; isolated application integration took 114.00 seconds.

| Check | Passed |
| --- | ---: |
| CPU libraries/catalog/audio/XR | 75 |
| Application library, including asset warmup | 22 |
| Actual-runtime tour framing | 1 |
| Renderer library / final color transfer | 4 / 1 |
| Renderer regression suite | 17 |
| Solar geometry / final surface material | 4 / 3 |
| Solar blending and family specialization / noise lattice | 1 / 1 |
| Typography and cache | 14 |
| Isolated application integration | 3 |
| Application/renderer doctests | Compiled; zero executable cases |

The final [CLI regression sweep](all_experiences_log.txt) also passed,
producing 43 PNG captures across all 25 experiences using the current
executable. Its [120-frame benchmark log](benchmark_sweep.txt) records a
960×540 desktop reference: 0.615 ms minimum, 1.142 ms mean, 2.727 ms p99 and
3.189 ms maximum. These low-resolution desktop values are separate from the
native-size stereo table above and provide no headset performance or visual
quality conclusion. All final local automated gates passed.

CI at `d791315463801c68114d046c68acf63a607da811` subsequently failed Rust 1.100's
new `chunks_exact_to_as_chunks` lint at six test-only sites. The compatibility
change uses borrowed `as_chunks` views and removes redundant slice collections;
probes, equations, budgets and rendering are unchanged. Local
[workspace Clippy](ci_compatibility_clippy.txt) passed in 0.91 seconds, and
[all three affected GPU tests](ci_compatibility_tests.txt) passed again in
3.09 seconds. **The new CI result is pending.** The manifest retains the
historical source hashes at that commit and records the revised test file's
LF-normalized hash separately; existing captures and full-suite results above
remain evidence from the original source freeze.

Four complete [60-second stereo cycles](motion_cycles_capture.txt) finished under
`target/solar_refinement_cycles`, with 1,801 frames per eye/view and all eight
encoders exiting successfully. They cover granulation, active region, eruption
and open outflow at 1280×720 per eye and 30 synthetic samples per second.
The [two native-size temporal clips](temporal_120_capture.txt) also completed:
granulation and active region, both eyes, 241 frames per eye/view, 3400×3468,
simulation time 30–32 seconds, synthetic head movement and 120 samples per
second. These encoding-active runs are visual evidence, not isolated
performance or native FPS measurements.
Earlier baseline full-suite and motion evidence must not be
read as final validation of these changed materials. Native OLED-headset
review, the user's PSVR2 at 120 Hz/current SteamVR resolution, and unaided
visual understanding remain open.

The bounded motion review inspected paired phase samples across all four
cycles, consecutive samples from 59.9–60 seconds, and tracked 1:1 native-size
crops at 30/31/32 seconds plus six consecutive 120-step frames from
31–31.0417 seconds. No obvious new pop/flash, proxy edge, world slip or
bilateral phase mismatch appeared in these samples. Granules evolved smoothly
with anchored borders, and the filament taper removed the blunt-ended rod
appearance. Loop heating was visible by 12 seconds, bright at 22/32, dimmer
and broken at 42/48, and faint at 58. **Returning rain remains difficult to
read.** The CME still reads as smooth luminous ropes around an empty cavity,
without a clearly distinct cool core or leading front; blue wind overlaps
that cavity. These remain visual-explanation gaps.

The review contacts remain locally under `target/solar_refinement_motion_review`
and its `native120` subdirectory. Native-size review crops track a fixed world
point by integer translation, with no scale or rotation resampling; their
sources are lossy CRF 18 MP4s. The packaged
[granulation](granulation_motion_crop_metadata.json) and
[active-region](active_region_motion_crop_metadata.json) metadata identify the
world points, frames and crop coordinates. Both native end-frame PNG pairs
were identical, consistent with negligible astronomical disparity; this is
not stereo or headset proof. Sampled review does not establish sustained
shimmer comfort, native 120 Hz operation or unaided scientific comprehension.

Packaged text uses LF line endings; provenance includes original and packaged
hashes. The full-suite log redacts one environment-only OpenVR log path/GUID as
`[redacted local OpenVR log path]`; results and diagnostics are unchanged.
The earlier baseline logs and preview were not replaced; complete new
stereo videos remain under `target`. The [visual acceptance checklist](../../../docs/SOLAR_PHYSICAL_RENDERING.md#visual-acceptance-checklist)
remains applicable, including free head movement and caption-free explanation.
