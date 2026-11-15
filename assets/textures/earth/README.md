# Earth material sources

These are embedded educational visualization assets, not live Earth observations.
NASA is the source; NASA does not endorse HyperCosmosVR. Usage follows
[NASA Images and Media Guidelines](https://www.nasa.gov/nasa-brand-center/images-and-media/).
Source URLs, downloaded dimensions and SHA-256 hashes are in `sources.json`.

| Layer | Source and credit | Processing / limits |
|---|---|---|
| `day.png` | [Blue Marble Next Generation, July 2004 base map](https://science.nasa.gov/earth/earth-observatory/blue-marble-next-generation/base-map/), Reto Stöckli, NASA Earth Observatory / MODIS | Cloud-free color composite, native 21600×10800 source, reduced in linear light to 8192×4096. |
| `night.png` | [Black Marble 2016 grayscale flat map](https://science.nasa.gov/earth/earth-observatory/earth-at-night/maps/), NASA Earth Observatory / Suomi NPP VIIRS | 13500×6750 source; grayscale light distribution receives an artistic warm-neutral tint. No daytime base map is emitted. This is not calibrated radiance. |
| `cloud.png`, absolute latitude ≤50° | [NASA Scientific Visualization Studio 4439](https://svs.gsfc.nasa.gov/4439/), Greg Shirah / NASA GSFC; NOAA Climate Prediction Center cloud composite | Native 8192×4096 rendered TIFF, frame 01000: **2014-06-02 00:00 UTC**. Its **alpha**, not its near-white RGB, supplies the visualized cloud opacity. This is interpreted as a density proxy, not measured optical depth. Source coverage ends at ±60°. |
| `cloud.png`, polar fill | [Blue Marble 2002 cloud composite](https://science.nasa.gov/earth/earth-observatory/the-blue-marble-true-color-global-imagery-at-1km-resolution/), NASA GSFC, Reto Stöckli / Robert Simmon / MODIS Science Team | Retained 2048×1024 global source fills absolute latitudes ≥59°, with a smooth 50–59° blend into CPC. Polar upsampling adds no observed detail. This deliberately combines different epochs; cloud height and transport remain authored, not measured weather. |
| `relief.png` | [GEBCO 2008 elevation via NASA Earth Observatory](https://science.nasa.gov/earth/earth-observatory/blue-marble-next-generation/topography-bathymetry-maps/), Jesse Allen / British Oceanographic Data Centre | Native 21600×10800 source. R: land elevation normalized over the source image's 0–6400 m range (clipped above; shading, not terrain mesh). G: water shading mask derived from Blue Marble blue dominance and low elevation, excluding ice. It is not an authoritative coastline or sea-ice product. B: reserved. |

All four PNGs are 8192×8192 **mip atlases**, not ordinary square planet maps.
The first 8192×4096 rectangle is the equirectangular base map (north at top,
longitude −180° at left). Each successive half-size mip follows vertically,
left-aligned, through 1×1. There are 14 levels, totaling 8192 rows.
RGB is sRGB encoded, including scalar data, so hardware sRGB sampling returns
linear values. Mips are downsampled in linear light. Longitude wraps, latitude
clamps. The GPU array uses approximately **683 MiB** including all mips.
The cloud atlas is a lossless grayscale PNG to keep its checked-in size small;
the loader expands it to RGBA without changing its red-channel scalar values.
Every individual PNG remains below 50 MiB. An 8192-pixel global map represents
about 4.9 km per equatorial texel; it does not replace local terrain/cloud assets
for low-altitude inspection.

Rebuild with `python tools/bake_earth_maps.py` (Pillow and NumPy required).
Original downloads are cached in ignored `target/earth_sources`; the application
needs no network, Python, or source downloads at runtime. Review the original
source hash changes before replacing baked imagery. The baker reduces each
native channel separately before assembling 8K RGB, avoiding a multi-gigabyte
native-resolution float RGB allocation. All resampling and mip generation use
linear values; no neural detail generation or sharpening is applied.

The shared 16-second particle/aurora animation is an accelerated explanatory
illustration. It is not a numerical magnetohydrodynamic simulation, and particle
marker sizes and emission brightness are magnified for inspection.
