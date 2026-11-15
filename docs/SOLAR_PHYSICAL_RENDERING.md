# Solar observing geometry

The Sun and planetary primary spheres now use physical radii shared by scene
assembly, navigation and the authored observation cameras. The Sun uses the
[IAU 2015 nominal radius of 695,700 km](https://www.iau.org/common/Uploaded%20files/IAUGA2015-Resolution-B3-recommended-nominal-conversion.pdf).
Planetary spheres use the volumetric mean radii in the
[NASA planetary fact sheets](https://nssdc.gsfc.nasa.gov/planetary/planetfact.html):
Mercury 2,439.7 km, Venus 6,051.8 km, Earth 6,371 km, Mars 3,389.5 km,
Jupiter 69,911 km, Saturn 58,232 km, Uranus 25,362 km, Neptune 24,622 km and
Pluto 1,188 km. These spherical approximations do not reproduce oblateness.

`solar_body_radius_m` is the shared source. A surface instance's proxy radius
is the physical radius divided by 0.84, matching the shader's analytic sphere.
The Sun's coronal proxy extends to 3 solar radii; its `phys_a.y = 1/3` gives the
photospheric radius in the volume's local coordinates. Neither radius is
clamped to a minimum apparent size or adjusted as the observer approaches.
Planetary illumination points toward this same heliocentric Sun, so the phase
changes consistently with the observer's position instead of following a
fixed studio-light direction. The Sun's photosphere is emissive.

Active Solar System observations use physical meters throughout the Solar
System sector. There is no distance or observer-zoom boundary that changes
the world's stereo scale. The policy is resolved once from the untracked
observer and passed explicitly to scene assembly. Real head translations
are converted into the navigation system's normalized coordinates; OpenXR
eye offsets remain measured in meters. Holographic controls retain their
nearby physical depth, while astronomical spheres have correspondingly tiny
binocular disparity. Normalized culling uses the cyclopean view. Actual body
positions bypass the distant navigation-shell proxy before conversion to
meters. A selected body's fallback retains its world position when looking
away.

Free exploration retains the current Solar sector while the observer stays
within its declared `4e13 m` spatial domain and its scale exponent is at most
`log10(4e13) ≈ 13.602`. Leaving that domain or zooming beyond it resumes the
normal sector-selection rules; an explicit destination, including Home,
can change ownership. This prevents an idle frame or close zoom after leaving
the tour from selecting an unrelated Earth/Technology experience by scale alone.

The lower observer-unit exponent is derived from the same domain radius:
`log10(radius_m / (sqrt(f32::MAX) / 64)) ≈ -3.858`, corresponding to roughly
0.14 mm per normalized unit. This leaves a linear safety margin for `f32`
squared lengths and proxy transforms before conversion back to physical
meters, while supporting ordinary meter-scale exploration. The clamp changes
unit metadata, preserving physical local position. CPU checks exercise actual
desktop/VR idle, motion, zoom and Home updates, high zoom at `u=13.5`, and
runtime rays at `u=0,4,8.8,13.5`; these numerical checks do not certify headset
navigation comfort.

Within 12 solar radii, the solar observing filter/exposure suppresses stars and
diffuse Galactic light to black in production scene assembly.
Their radiance returns smoothly between 12 and 24 solar radii as the observing
exposure relaxes, without changing geometry or stereo units. Arbitrary
catalog sprites, orbital hairlines, generic
toroidal particles and ambient labels are omitted. The explicit narration,
spatial guide and requested holographic controls remain available. This
distance changes decoration only; it never changes physical geometry or
units. At a wide overview, real planetary discs can be smaller than a pixel.
Their navigation targets remain available; scene assembly does not enlarge
the spheres to satisfy an overview visibility quota.

Resolved strand geometry is skipped only when the Sun's photospheric radius
projects below two pixels in both eyes. The test uses each eye's projection
and the actual eye target dimensions; the natural surface and diffuse corona
remain present. Close and oblique observations retain the full strand detail.

Cool strand absorption is composited onto the opaque solar background before
any coronal light is added. These transmittance products commute. The diffuse
corona and resolved strand emission are then added with zero extinction alpha,
so reversing strand submission order cannot darken crossing loops. This is a
bounded optically thin emission model: it represents extinction of the common
opaque background, not attenuation of one luminous strand by another. It is
not a full radiative-transfer or order-independent transparency solution.

The Sun's opaque surface and diffuse corona have dedicated mono/stereo entry
points in `solar_hero.wgsl`, sharing the same physical geometry and material
helpers. Other documentary heroes retain their existing dispatch. Draw
batches preserve storage indices and neighboring instance order.

The atmosphere optimization retains radial density integration while moving
selected angular texture evaluations to emission-weighted directions. The
thin chromosphere also accumulates angular variance and viewing-distance
moments to filter its mottling. A rotated, quintic-interpolated lattice
provides three smooth noise channels for the diffuse atmosphere. This is an
appearance approximation, not an exact integral of every previous noise
sample; resolved plasma remains a separate three-dimensional strand pass.
EUV contouring and sampling stability are still being revised. The
[solar review report](../artifacts/solar_presence/README.md) records diagnostic
performance results separately from visual and native-headset acceptance.

The fusion observation is an explicitly explanatory cutaway. It removes the
positive octant in the fixed active-region basis: east `(1,0,0)`, north
`(0,cos(20°),-sin(20°))`, outward `(0,sin(20°),cos(20°))`. Analytic ray
intersections find the nearest retained photosphere, deep radial wall or
independent curved core. The core extends to approximately 0.25 solar radii;
the radiative zone reaches approximately 0.70 radii, followed by convection,
following the [NASA/MSFC interior overview](https://solarscience.msfc.nasa.gov/interior.shtml).
Interior colors compress the temperature range for explanation. They are not
a naked-eye view into optically opaque plasma. The same retained-geometry
intersection clips emission, preventing light from leaking through the
uncut hemisphere.

The rest of the Solar System is not a dated ephemeris. Orbital positions and
hairlines still follow the catalog's schematic arrangement. Belt zones 5 and
10 retain explicit teaching-volume extents. Companion moon sizes, placements,
selected teaching rings and feature overlays outside the Sun chapter remain
authored explanatory representations; they have not been converted into a
complete physical satellite system. The physical sphere radii and stereo
units must not be read as a claim that those remaining teaching props are
metrically accurate.

CPU regressions cover fixed solar radius/center/orientation across observer
scales and a 180° head turn, all planetary primary radii, authored Sun visual
ownership without narration, real-meter head tracking and preserved nearby
holographic controls. These checks do not establish motion stability or
perceived presence in a headset; moving-camera stereo captures and native
headset review remain necessary visual acceptance evidence.

## Observing modes and accelerated time

The photosphere uses a neutral, filtered visible-band appearance with a
5,772 K reference, procedural granules around 900 km across and filtered
120 km substructure. The appearance is informed by the
[NSO Inouye convection movie](https://nso.edu/telescopes/dkist/first-light-full-field-movie/)
and [observed sunspot morphology](https://nso.edu/press-release/new-images-released-by-nsfs-inouye-solar-telescope/).
These references are not texture assets or a claim of matching the telescope's
resolution, calibration or measured radiance. A fixed co-rotating frame keeps
the same sunspots and magnetic roots available during inspection; plasma and
granules evolve within it.

The whole-Sun overview (macro `255`, no selected feature) and F1/F2 combine
this filtered visible photosphere with illustrative temperature-colored
coronal strands and blue outflow tracers. Those added plasma colors and their
visibility are explanatory; this composite is not an unassisted-eye view or
a single visible-light exposure. The neutral photospheric appearance does
not make the whole composite natural-color solar radiometry.

Granulation and active-region observations (F1/F2) retain the visible continuum
with an explicit display-contrast enhancement. Before limb darkening and tone
mapping, the linear RGB radiance is scaled by luminance to the power 1.1,
making the resulting luminance proportional to the original luminance to the
power 2.1, with a small detail-dependent mean normalization. This preserves
the input temperature-derived chromaticity and applies the limb curve after
the enhancement. It is a viewing transform that makes cool lanes and umbrae
more legible, not calibrated solar radiometry. The whole-Sun overview does
not apply this enhancement.

Eruption and outflow observations (F3/F4) select an explicit EUV-atmosphere
mode: the visible continuum is attenuated to 3.5% for spatial reference while
the surrounding low corona and magnetic network become the principal image.
The mode uses illustrative 171/193-like, multiple-temperature color mapping,
informed by the [SDO instrument channel descriptions](https://sdo.gsfc.nasa.gov/data/channels.php).
It is not calibrated synthetic AIA data. The dark coronal-hole source shares
its location with the open funnel; its reduced coronal emission represents
lower density, not an additional dark photospheric sunspot. This distinction
follows [NOAA's coronal-hole description](https://www.spaceweather.gov/phenomena/coronal-holes).

The runtime discloses the overview as “Visible light + illustrative plasma”.
F1/F2 use that same composite disclosure plus a second short line, “Contrast
enhanced”. F3/F4 remain “EUV atmosphere · false color”, and F0 remains
“Interior cutaway · schematic transport”. English, Finnish and Spanish
all preserve these distinctions: the composite line is “Näkyvä valo +
havainnollistava plasma” in Finnish and “Luz visible + plasma ilustrativo”
in Spanish, with “Tehostettu kontrasti” / “Contraste realzado” for F1/F2.
The lettering is curved and scene-occluded at 2.35 m, anchored in tracking
space on entering that mode. The label has no panel and stays fixed through head turns and lean;
it does not depend on narration or the wrist controls being open.
Deterministic verification overrides omit it for caption-free visual review.
`caption_hint` metadata alone is not an implemented display path.

Coronal, chromospheric and interior emission use explanatory color mappings.
The combined scene is not a naked-eye view, a single calibrated instrument
passband, an absolute HDR luminance measurement or an MHD simulation. The
cutaway reveals otherwise opaque material. A color change by itself is not
evidence that a learner can infer a temperature or physical mechanism.

`doc_sun_clock` advances the photospheric and chromospheric reference clock at
30 times application time. Granule lifetimes range from 280 to 680 reference
seconds. The global coronal story repeats every 60 application seconds,
independently of narration or the currently selected beat. It does not restart
when the user enters a view: neighboring strands heat with different delays,
closed loops fill from their anchored feet, cooling material returns along
closed paths, and a separate open funnel carries material outward. The
eruptive branch grows during approximately seconds 12–34 and fades during
36–48 before the cycle repeats. This is an authored sequence, not a dated
solar event or a validated reconnection calculation.

The eruption now has cooler embedded prominence material, a depleted space
and a broader patchy leading envelope. Its 128 strands include 32 denser
inner strands and 96 broader outer strands, with 24 peripheral strands
fraying as it expands. This draws on observed three-part CME morphology
described by [Xie, Gopalswamy and Akiyama](https://cor1.gsfc.nasa.gov/docs/Xie_apj_2021.pdf).
The depiction combines explanatory thermal colors; it does not reproduce a
single white-light or EUV instrument image, evolve a self-consistent magnetic
field, or demonstrate reconnection. The earlier full-cycle diagnostic movies
predate this broader material structure and cannot validate its current
detail, occlusion or bounds.

The open-funnel packet phase traverses its approximately 1.70-radius radial
span in about 53.6 application seconds. Interpreting that phase with the
30-times reference clock gives approximately 735 km/s of radial bulk flow.
This is a constant packet-phase approximation on authored geometry, not a
simulation of coronal acceleration. Other illustrative motions have not all
been validated against one uniform physical clock. In the cutaway, 512
authored packet paths take short backtracking steps with a net outward bias
through approximately 0.252–0.696 solar radii. Only moving packets trace these
paths; permanent radial wires would misleadingly suggest straight-line
transport. Their roughly 23–29 application-second crossing time, visible
glyph sizes and macroscopic step lengths are compressed explanatory choices.
They do not depict individual photons at physical size or speed, and do not
use the 30-times surface clock. The outer material-circulation paths repeat
in 12.5 application seconds; the cut-wall convection rolls use approximately
20–30 application seconds. Both are schematic, with broad hot upflows and
cooler returning material, rather than a calibrated common flow solution.
The distinct random walk and returning circulation are intended to contrast
energy diffusion with material convection.
The qualitative distinction follows [NASA's Sun structure overview](https://science.nasa.gov/sun/facts/#h-structure):
fusion supplies energy in the core, radiation transports energy through the
deeper interior, and hot plasma rises in the convection zone. The reference
does not specify these authored packet paths, colors, counts or shortened
clocks; the illustration does not resolve photon mean free paths, opacity,
hydrostatic balance or a nuclear reaction network.

## Six observations and required visual evidence

All six observations are quiet `StaticHold` views. Free head motion remains
available; neither a sentence change nor the event apex turns the user's
head. Exterior travel follows arcs that stay outside the photosphere.
The two event observations each hold for at least one complete global cycle,
even when they are entered at an arbitrary phase.

| Observation | Minimum hold | Evidence required without captions or narration |
| --- | ---: | --- |
| Whole Sun | 24 s | A coherent stellar surface, readable active regions and surrounding atmosphere; an isolated decorative jet must not dominate the star. The oblique observer at `5R * normalize(C + 0.55E + 0.65N)` aims at `0.6R * C` to retain the entire disc and late eruption with margin. |
| Interior opening | 32 s | Real occlusion and depth of the retained surface, core and radial zones; outward energy packets with local backtracking must remain visibly different from circulating material in the outer convection zone. |
| Granulation | 32 s | Hot cell interiors, cooler lanes and independent evolving cells resolve in the foreground; their scale and behavior remain legible as the view moves toward the horizon. |
| Active region | 60 s | Both sunspot footpoints remain distinct; strands rise around and above the observer; footpoint filling, heating and returning cooled material can be followed along the same anchored paths. |
| Limb eruption | 60 s | The same rooted structure visibly rises above the limb; the default view keeps the quiet surface and roots legible, while the late apex rises overhead and is reachable by a voluntary upward head turn. |
| Open outflow | 32 s | Material escapes continuously from a separate open region; its source and outward progression remain distinguishable from closed loops and falling material. |

The interior currently provides depth, differentiated zones and schematic
transport paths, but explicit hydrogen-to-helium transformation remains an
open teaching gap. A glowing core is not a quantum reaction animation.
Geometry tests and the existence of packet paths do not establish that the
fusion and energy-transport narration is visually taught; the packet and
circulation mechanisms still need visual learnability review. Nor does an
erupting tethered branch alone demonstrate magnetic reconnection.

## Visual acceptance checklist

These are acceptance requirements, not a record of completed headset review.
Keep each item open until the corresponding evidence has been inspected.

- [ ] Watch a full cycle with narration, captions and guide overlays disabled.
  A viewer can describe what fills, what heats or cools, what returns and what
  escapes using the visible motion and causal connections.
- [ ] Review both eyes at the six actual journey poses, including quiet,
  filling, peak eruption and cooling phases. Bright strands retain internal
  separation; overlapping plasma does not become an opaque plastic tube.
  The photosphere does not read as a static ceramic tessellation.
- [ ] Inspect the active region from front, flank and back, including a
  position among the loops. Roots remain attached to the same sunspots;
  foreground and background strands have consistent occlusion and depth.
  A head turn never moves or reorients the Sun toward the observer.
- [ ] Record consecutive frames through heating, cooling and the 60-second
  wrap. Inspect motion for flashing, popping, synchronized births and visible
  resets. At the harness's default synthetic 30 Hz, 1,801 frames include both the
  initial phase and the endpoint one full cycle later; sparse saved frames
  alone cannot establish temporal stability.
- [ ] Use the `earth_review` ninth argument `head`, `orbit` or `head-orbit`
  alongside static captures. Preserve the reported simulation times,
  frustum, eye resolution, physical IPD and frame poses. Verify detail stays
  attached to the world during the sweep, and inspect slower free exploration
  beyond those bounded presets. Synthetic motion is not native tracking proof.
  The `head-look-up` preset rises smoothly from 0° to 30° pitch and returns
  over 60 seconds, making the late eruption inspectable without changing the
  production tour camera. Begin both capture and simulation time at zero to
  align that illustrative head movement with the event.
  `solar-review` selects this for the eruption and `head-orbit` for the other
  observations. The optional tenth argument `video` records every frame to
  both eye MP4s independently of PNG stride. Inspect the full movies for
  temporal behavior, retain PNGs for lossless pixel checks, and do not use
  encoding-active frame timings as isolated rendering benchmarks.
  The optional eleventh argument `capture_hz` changes simulation/motion
  sampling and encoded playback rate together; it defaults to 30 and must
  be positive and finite. For example, `video 120` gives 120 synthetic
  samples per second, while `no-video 120` omits encoding. A 241-frame
  sequence spans two seconds including both endpoints. This is useful for
  inspecting temporal aliasing at the target sampling interval, but does
  not establish that the app rendered those frames at native headset speed.
- [ ] Inspect granule and filament detail at actual per-eye output sizes in
  the 1440p–4K range. Record dimensions and render scale explicitly; an
  exported screenshot or desktop mirror does not establish headset detail.
- [ ] Review on an OLED headset, including PSVR2 at the user's Steam-selected
  maximum resolution and 120 Hz setting. Record native application cadence
  separately from compositor refresh, reprojection and dropped frames. Report
  30, 90 and 120 application-FPS tiers honestly rather than treating any one
  threshold as a substitute for richness or presence.
- [ ] Preserve exact black where the observing model has empty space while
  retaining faint local structure. Close horizons have no arbitrary minimum
  black-pixel quota. Holographic controls remain spatial; no flat panel is
  required to understand the scene.

The first integrated stills exposed excessive strand brightness, edge-on
arcade framing and overly uniform granulation. They are diagnostic evidence,
not acceptance of documentary quality. Subsequent visual and native-headset
review must assess the revised implementation against the requirements above.
