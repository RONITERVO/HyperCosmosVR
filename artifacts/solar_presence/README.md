# Solar presence review

This change replaces the short Sun stop with six connected observations of a
living stellar surface, magnetic atmosphere and explanatory interior. It also
corrects Solar System body scale and navigation. The broader visual-quality
goal remains active: passing geometry tests or producing new shaders does not
establish documentary quality, unaided understanding or headset presence.

## Implemented scope

| Observation | Hold | Intended visual explanation |
| --- | ---: | --- |
| Whole Sun | 24 s | The same coherent star and fixed active regions that the following views approach. |
| Interior cutaway | 32 s | A curved core, deep radial walls, outward energy packets with local backtracking, and returning material circulation in the outer zone. |
| Granulation | 32 s | Independently evolving bright cell interiors and cool lanes at a physically scaled surface. |
| Active region | 60 s | Two distinct sunspot footpoints, surrounding loops, filling, heating, cooling and falling material. |
| Limb eruption | 60 s | A rooted branch rising overhead; the wearer can look up without a forced camera pan. |
| Open outflow | 32 s | A separate low-density coronal source and outward flow, distinguished from closed loops. |

The Sun uses a 695,700 km photospheric radius and a fixed world center and
orientation. Actual planetary primary spheres use physical mean radii;
planet approaches observe their illuminated hemisphere at a 35° phase angle.
The active Solar System sector renders in physical meters, including real
head translation and eye separation. Nearby holographic controls retain room
scale. Subjects do not turn toward the viewer or grow with head movement.
The 48-stop Highlights manifest uses quiet static observations, exterior
solar arcs and an explicit Earth-to-Solar chapter cut.
The current whole-Sun eye is `5R * normalize(C + 0.55E + 0.65N)`, looking
toward `0.6R * C`, with the same 24-second static hold. This oblique approach
separates eruption depth from the disc; the other five Sun poses are unchanged.
The star and active-region frame do not rotate to accommodate the view.
CPU projection of the current 128 eruptive strands at all 65 segment endpoints,
at 0/16/24/32/40 seconds, includes their 3.5-sigma material support and a
conservative 720-pixel filtering footprint. The largest absolute horizontal
NDC coordinate is 0.686 at 3400×3468 per eye; square and 16:9 views also fit.
The journey regression encloses the entire eruption in a larger physical
box with margin and checks the complete stellar rim. This is framing
evidence, not a claim that the revised composition has passed visual review.

The whole-Sun view and F1/F2 combine a filtered visible photosphere with
illustrative temperature-colored coronal plasma and blue outflow tracers;
the complete composite is not an unassisted-eye view. F1/F2 also enhance
surface contrast. F3/F4 use an illustrative EUV atmosphere with the visible
continuum reduced for spatial reference. F0 reveals an explanatory interior.
All six observations have a short,
room-anchored holographic disclosure without a flat panel; the evidence harness
omits text so visual teaching can be assessed independently. The
[physical and observing-mode contract](../../docs/SOLAR_PHYSICAL_RENDERING.md)
and [shader contracts](../../shaders/SHADER_CONTRACTS.md) specify the geometry,
color mappings, separate clocks and remaining limitations.

The current source expands the eruption into cooler embedded prominence
material, a depleted cavity and a broader clumpy leading envelope, with
peripheral strands dispersing while their feet remain anchored. This replaces
the earlier thin eruptive rope visible in the completed diagnostic movies.
The diffuse corona now integrates radial columns and angular moments before
evaluating selected broad texture terms, uses smooth quintic atmosphere noise,
and has dedicated Sun surface/corona pipelines. These are still under visual
and performance review; this report does not assume visual equivalence from
the optimization alone.

## Completed checks

For a direct headset review, start SteamVR with the headset connected, then
run this from the repository root:

```powershell
.\Start-HyperCosmosVR.cmd --journey artifacts/solar_presence/solar_observatory.hcj.json --autoplay-journey
```

The [standalone review journey](solar_observatory.hcj.json) contains exactly
the six shipped Sun beats, including the oblique whole-disc pose, entry cut,
static observations and exterior arcs. It loops within the solar chapter;
the main 48-stop Highlights is unchanged. The actual manifest loader and CPU
scene projection audit accept this file. This is a prepared review entry
point; native headset operation and acceptance are still pending.

```powershell
cargo run --release -p cosmos_app --example tour_projection_audit -- --manifest artifacts/solar_presence/solar_observatory.hcj.json
```

The source-freeze local suite passed in 298.94 seconds. The
[test log](final_test_log.txt) records the following checks; rows overlap
and should not be summed into a unique-test count.

| Check | Result |
| --- | --- |
| Workspace CPU suite | 75 passed, including journey, navigation and spatial checks |
| Application library | 22 passed: 1 asset-cache warmup and 21 application checks |
| Journey / spatial libraries | 9 / 10 passed, included in the CPU suite |
| Actual-runtime tour framing | 1 passed, covering all 3,011 Odyssey stops and the built-in/shipped 48-stop Highlights |
| Renderer library / final color output | 4 / 1 passed, including shader validation, texture contracts, Sun batching/LOD and AgX transfer |
| Renderer GPU regression suite | 17 passed |
| Solar GPU geometry | 4 passed: cutaway/sphere misses, strand integrals, proxy bounds and corona quadrature |
| Solar GPU blending / cached noise | 1 / 1 passed, explicitly run after their normal ignored-test entries |
| Typography and cache | 14 passed in the isolated phase |
| Application integration | 3 passed in the isolated phase |
| Application / renderer doctests | Compiled successfully; no executable doctest cases |
| Workspace all-target Clippy | Passed with `-D warnings` in 3.11 seconds |
| Separate all-experience headless sweep | Completed 43 PNG captures covering all 25 experiences |

Separate focused capture-harness checks also passed, including dual-eye MP4
frame order, decoded color and actual `ffprobe` color metadata, configurable
sampling-rate validation and rate-independent motion. The separate
[headless sweep log](benchmark_sweep.txt) records 120 frames at 960×540:
1.031 ms mean and 2.214 ms p99. This low-resolution desktop reference supplies
no VR visual-quality or performance inference. All final local automated
gates passed; native headset and broader artistic/learning acceptance remain open.

Packaged text logs use LF line endings for stable checkout hashes. The test
log replaces one environment-only OpenVR log path with
`[redacted local OpenVR log path]`; test results and diagnostics are unchanged.
The [provenance manifest](native_reference_manifest.json) records the original
and packaged hashes and these transformations.

The application check also exercises all six actual observing-mode selections
in English, Finnish and Spanish, including the macro composite disclosure, and
verifies that disclosure glyphs retain physical depth through head turns and
lean. The tests establish specific invariants, not visual richness, absence
of shimmer, native frame cadence or user comfort.

## Current review artifacts

The latest `target/solar_release_cycles` run completed all six views and both
eyes: 1,801 frames per eye per view, spanning simulation time 0–60 seconds
at 30 synthetic samples per second, 1280×720 per eye. It includes the current
5R macro viewpoint, cached solar material noise and gradual rain fade.
The raw stereo MP4s and per-frame camera poses remain under `target`.

- [60-second mono preview](solar_observatory_preview.mp4): 1,800 frames,
  1280×720, 30 fps, 35,042,289 bytes (33.4 MiB). No narration or feature labels.
- [Preview contact sheet](preview_contact.png): midpoint images decoded from
  the lossy source videos and resized; these are not original renderer pixels.
- [Preview provenance and hashes](preview_manifest.json): source ranges,
  output properties, color metadata and limitations.

| Preview time | Observation | Source simulation time |
| --- | --- | --- |
| 0–10 s | Whole Sun | 12–22 s |
| 10–20 s | Interior | 9–19 s |
| 20–30 s | Granulation | 20–30 s |
| 30–40 s | Active region | 12–22 s |
| 40–50 s | Eruption | 24–34 s |
| 50–60 s | Open outflow | 12–22 s |

The contact samples and the six original cycle-end PNGs were inspected for
composition. Additional full-cycle contacts were reviewed at five-second
intervals, with the eruption sampled at three frames per second from 27–31
seconds and its 60-second wrap checked. No eruption discontinuity was observed
in those samples. The gradual rain birth/drain correction is present in this
release recording. These are bounded samples, not inspection of every frame.
The eruption excerpt follows the synthetic upward head look,
so its Sun/roots can leave the frame; the full source cycle supplies that
context. These short mono excerpts do not establish full-cycle comprehension,
stereo comfort or temporal stability on a headset. Lossless peak stills from
the final native-size burst are packaged below and remain distinct from decoded-video
PNGs. The contact's labels are review metadata outside the captured image;
they are absent from the preview and production scene.

`ffprobe` verifies the packaged preview's BT.709 primaries/matrix, sRGB
transfer and limited YUV range. The raw cycle MP4s had unknown primaries and
transfer metadata despite the intended encoder options; their matrix/range
were present. The preview explicitly tags the intended properties without
applying a new pixel transfer, and the harness now sets frame-level metadata
with a dual-encoder regression. MP4s remain lossy review copies. Original
renderer PNGs are the pixel/black-level reference. No timing gathered while
encoding these movies is treated as an isolated benchmark.

## Final rendering references and original pixels

The final offscreen reference used an RTX 5070, 3400×3468 pixels per eye,
static symmetric projections, 64 mm IPD, 72° vertical / 70.924° horizontal
FOV, 120 measured frames per view after warmup, and simulation time
30.000–33.967 seconds. Video encoding was disabled; GPU timestamp profiling
was enabled. The copied [capture log](native_reference_capture.txt) preserves
the configuration and per-pass measurements. A second
[2560×1440-per-eye reference](reference_1440_capture.txt) used the same
static poses, 120-frame interval, profiling and disabled video. Its symmetric
72° vertical projection has a 104.505° horizontal FOV, compared with 70.924°
at 3400×3468. Both pixel count and view coverage differ; this is not a pure
resolution-scaling comparison. These are **CPU submission plus GPU wait**,
not native headset frame times:

| View | 3400×3468 mean / p95 ms | 2560×1440 mean / p95 ms |
| --- | ---: | ---: |
| Whole Sun, current 5R view | 7.071 / 7.437 | 2.211 / 2.414 |
| Interior | 14.093 / 14.401 | 5.093 / 5.434 |
| Granulation | 14.153 / 14.576 | 4.658 / 4.976 |
| Active region | 23.154 / 23.755 | 7.855 / 8.278 |
| Eruption | 18.454 / 19.035 | 5.148 / 5.505 |
| Open outflow | 11.190 / 11.600 | 3.580 / 3.864 |

At 3400×3468, only the macro's mean fits the nominal 8.33 ms interval for
120 application FPS. The other five views exceed it before headset/compositor
overhead; the active region is the largest measured gap. At 2560×1440, the
sampled means and p95 values fit that interval, but the active-region p95 of
8.278 ms leaves almost no allowance for omitted application/compositor work.
Neither run establishes native 120 FPS. All means below 33.33 ms in these
short references also do not prove a native 30 FPS floor. The new macro pose changes both composition and workload, so its
result is not a direct same-view comparison with the old 3R diagnostics.

An OpenXR probe run outside the sandbox failed at `xrCreateInstance` with
`InstanceCreationFailed` (unexpected runtime error). No native session,
headset projection, compositor cadence, reprojection or dropped-frame
measurement was obtained. This failure does not identify the underlying
runtime/headset cause; the offscreen renderer cannot substitute for that
acceptance evidence.

A separate [120-step temporal capture](temporal_120_capture.txt) completed
granulation, active region and eruption: 241 frames per eye at 3400×3468,
simulation time 30–32 seconds, synthetic head movement and 120 samples per
second. All six encoders exited successfully; raw clips and camera records
remain under `target/solar_native_120_steps`. Exact 1:1 decoded-video crops of
frames 0–7 were reviewed for granules, penumbra, the arcade and erupting material.
Those bounded samples showed coherent translation without obvious sparkle or
jumps. They are lossy video samples and cover only the opening 58 milliseconds
of the two-second clip. They do not establish stability throughout the cycle,
native headset cadence or comfort. Encoding-run timings are not benchmarks.

The native burst's final eye0 frame is simulation time 33.967 seconds.
The [six-view contact](native_contact.png) was resized from those original
PNGs solely for composition review. The following files preserve original
pixel values, with no resizing or color adjustment:

| Image | Original source rectangle, pixels `(left, top, right, bottom)` |
| --- | --- |
| [Full macro](exp04_z000_sun_native_eye0.png) | Entire 3400×3468 frame |
| [Granulation detail](solar_granulation_horizon_native_crop.png) | `(1100, 2200, 2124, 3224)` |
| [Active-region detail](solar_active_region_native_crop.png) | `(800, 2150, 1824, 3174)` |
| [EUV atmosphere detail](solar_limb_eruption_native_crop.png) | `(1100, 2350, 2124, 3374)` |

Crop right/bottom bounds are exclusive; each detail is exactly 1024×1024
source pixels. [Native evidence provenance](native_reference_manifest.json)
records all six original paths/hashes, retained crop rectangles and output
hashes. Full native frames and both-eye camera records remain under
`target/solar_native_final`. The macro retains the whole disc and late
eruption; the static close eruption shot intentionally permits overhead
material outside the initial gaze. Contact/crop inspection is not proof of
full-cycle stability, scientific comprehension or final visual acceptance.

## Diagnostic captures and performance investigation

The six-view recording completed under `target/solar_cycles`. Its
[workspace log](../../target/solar_cycles/capture.txt) records successful
encoder exits and 1,801 frames for each eye of every view, from simulation
time 0 through 60 seconds. **These are superseded diagnostic movies:** they
contain the older thin eruption, before the broad prominence/cavity/envelope
revision and subsequent atmosphere optimization. They are not final evidence
for the current source. Full movie inspection and acceptance are not claimed
by this report. Those earlier movies are not packaged; the newer release-run
excerpts above replace them as the current review preview.

The recording uses 1280×720 per eye, 30 synthetic simulation frames
per second, 64 mm IPD and a symmetric 72° vertical field of view. It records
1,801 frames per view, including both endpoints of a complete 60-second cycle.
`solar-review` uses a gradual upward head look for the eruption and a small
head/orbit sweep for other views. Both eye matrices are rebuilt every frame.

Two CPU libx264 encoders use four encoder threads each, preset `fast`, CRF18
and `yuv420p`. Every frame enters the videos independently of PNG stride;
the final original PNG and all frame poses remain available. MP4s are lossy
review copies, not exact color or black-level references. Encoding-active
timings are affected by concurrent CPU work and must not be presented as
isolated renderer benchmarks. Its small-resolution timing numbers are
therefore not used in the comparison below.

The initial no-video native-size reference completed at 3400×3468 per eye,
120 measured frames per view, simulation time 30.000–33.967 seconds. The
[initial GPU profile](../../target/solar_gpu_profile/capture.txt) then used
30 measured frames and identified the volume pass as the main cost:
21.904–57.311 ms across the six views, including 57.311 ms for the eruption.
Both runs predate the current candidate. The initial no-video log is
[solar_native_reference](../../target/solar_native_reference/capture.txt).

Candidate 2 used the same eye dimensions and static poses, 60 measured frames
at 30.000–31.967 seconds, no video, and GPU timestamp profiling. Values below
are **CPU submission plus GPU wait, mean / p95 in milliseconds**:

| View | Initial reference, 120 frames | Candidate 2, 60 frames | Candidate 3, 60 frames |
| --- | ---: | ---: | ---: |
| Whole Sun | 29.472 / 30.032 | 24.520 / 29.156 | 23.775 / 28.758 |
| Interior | 29.210 / 29.561 | 15.665 / 15.994 | 14.428 / 14.677 |
| Granulation | 45.345 / 45.744 | 18.564 / 18.994 | 17.483 / 17.795 |
| Active region | 44.966 / 45.380 | 23.420 / 23.667 | 22.544 / 22.999 |
| Eruption | 64.060 / 64.373 | 26.743 / 27.299 | 25.580 / 25.955 |
| Outflow | 42.707 / 42.967 | 16.941 / 17.262 | 16.017 / 16.267 |

The [candidate-2 log](../../target/solar_perf_candidate_2/capture.txt) records
volume-pass means of 6.694–17.689 ms. The expanded eruption also changed
emission cost, so this comparison is not an equal-image speedup claim.
The [candidate-3 log](../../target/solar_perf_candidate_3/capture.txt) records
the subsequent dedicated-Sun-pipeline run at the same 60-frame interval.
It improves these means by approximately 0.7–1.2 ms, but visual review found
EUV stacks/contours and insufficiently detailed eruption material. Atmosphere
sampling, eruption detail and bounds were subsequently revised. All three snapshots
remain diagnostics; no final visual or performance acceptance is claimed.
Their whole-Sun pose was the earlier `3R * C` view. Candidate 6 then used an
oblique 3R view that clipped the late eruption; the current 5R framing widens it.
The later macro framing changes the rendered workload as well as composition, so a fresh
macro timing is not a direct same-view A/B comparison with those records.

Even candidate 3 remains above the 90/120 application-FPS planning intervals
of 11.11/8.33 ms. Being below 33.33 ms in a short offscreen reference does not
prove a native 30 FPS floor once application work and the compositor are
included. Current full-cycle samples and final rendering references are
documented above; headset acceptance remains open. The workspace log links in
this historical section are local diagnostics, not packaged release artifacts.

Run from the repository root, using a fresh output directory:

```powershell
# Full-cycle diagnostic command; the existing output predates the latest eruption.
# Use a fresh directory to capture the revised source.
cargo run --release -p cosmos_app --example earth_review -- target/solar_cycles 1280 1801 exp04_z000_sun,solar_ 0 720 0 4 solar-review video

# Initial no-video reference configuration, already run on the earlier source.
cargo run --release -p cosmos_app --example earth_review -- target/solar_native_reference 3400 120 exp04_z000_sun,solar_ 30 3468 0 4 static

# Candidate-2 profiling configuration; readback follows the CPU timing sample.
$env:HYPERCOSMOS_GPU_PROFILE='1'
cargo run --release -p cosmos_app --example earth_review -- target/solar_perf_candidate_2 3400 60 exp04_z000_sun,solar_ 30 3468 0 4 static
Remove-Item Env:HYPERCOSMOS_GPU_PROFILE

# Completed temporal-detail configuration: three views, two simulation seconds.
# Use a fresh directory when reproducing; 120 is synthetic sampling/playback Hz.
cargo run --release -p cosmos_app --example earth_review -- target/solar_native_120_steps 3400 241 solar_granulation_horizon,solar_active_region,solar_limb_eruption 30 3468 0 4 head video 120
```

The optional eleventh argument sets positive finite `capture_hz`, defaulting
to 30. Simulation, head motion and MP4 timestamps use the same rate. Use
`no-video` in the tenth position when setting the rate without encoding.

Those eye dimensions are a reference previously reported for PSVR2 by SteamVR,
not proof of the current
recommended render scale. The offscreen harness uses synthetic symmetric
projections and omits the native headset/compositor. Its submission-plus-wait
measurements cannot establish native application FPS or reprojection behavior.

## Scientific and acceptance limits

The scene follows the qualitative distinctions in
[NASA's solar structure overview](https://science.nasa.gov/sun/facts/),
[SDO's instrument channels](https://sdo.gsfc.nasa.gov/data/channels.php),
[NOAA's coronal-hole description](https://www.spaceweather.gov/phenomena/coronal-holes)
and [NSO's observed convection](https://nso.edu/telescopes/dkist/first-light-full-field-movie/).
It is a procedural illustration, not calibrated instrument data, a dated
ephemeris, an MHD calculation or a full radiative-transfer solution. The
expanded eruption draws on observed three-part CME morphology described by
[Xie, Gopalswamy and Akiyama](https://cor1.gsfc.nasa.gov/docs/Xie_apj_2021.pdf),
without claiming to reproduce one observation or a calibrated EUV passband.

The surface uses a 30× reference clock; the coronal story repeats in 60
application seconds. Interior packet sizes, steps and 23–29-second crossing
times are schematic, as are the separate convection clocks. The glowing core
does not show hydrogen-to-helium reactions. An erupting tethered branch alone
does not demonstrate reconnection. Older satellite, belt and ring teaching
props are not a complete metrically accurate Solar System.

Native OLED-headset acceptance remains unverified, including PSVR2 at 120 Hz
and SteamVR's current recommended resolution. Review must distinguish native
application cadence from compositor refresh/reprojection, assess the 30/90/120
FPS tiers at actual per-eye resolutions, and test whether a viewer can explain
the visible processes with narration and captions disabled. The
[visual acceptance checklist](../../docs/SOLAR_PHYSICAL_RENDERING.md#visual-acceptance-checklist)
remains the review standard; close horizons have no arbitrary black-pixel quota.
