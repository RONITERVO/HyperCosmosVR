"""Bake NASA Earth maps and linear-light mip atlases (Pillow + NumPy).

Run from the repo root. Downloads are cached under ignored target/earth_sources.
No network or asset installation is needed at app runtime.
"""
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
from pathlib import Path
import urllib.request

import numpy as np
from PIL import Image

# The pinned 21600×10800 day/height images contain 233,280,000 pixels.
# Decode and reduce one channel at a time, never a native-size float RGB array.
Image.MAX_IMAGE_PIXELS = 240_000_000

ROOT = Path(__file__).resolve().parents[1]
CACHE = ROOT / "target/earth_sources"
OUT = ROOT / "assets/textures/earth"
STAGING = CACHE / "baked_8k"
BASE = "https://assets.science.nasa.gov/content/dam/science/esd/eo/images/"
SIZE = (8192, 4096)
SOURCES = {
    "day": BASE + "bmng/bmng-base/july/world.200407.3x21600x10800.jpg",
    "night": BASE + "imagerecords/144000/144897/BlackMarble_2016_3km_gray.jpg",
    "height": BASE + "bmng/topography/gebco_08_rev_elev_21600x10800.jpg",
    "cloud": "https://svs.gsfc.nasa.gov/vis/a000000/a004400/a004439/frames/8192x4096_1x1_30p/clouds_cpc/flat_cpc_clouds_8192x4096.01000.tif",
    "cloud_polar": "https://assets.science.nasa.gov/content/dam/science/esd/eo/content-feature/bluemarble/images/cloud_combined_2048.jpg",
}
EXPECTED_SHA256 = {
    "day": "dea8b4dc8a4f93f5f8bce0c8c85a508a178e7901e9ed8e6bf86e6ce7ef6d61e2",
    "night": "3fbc8aae3529dc1ebfcf3aa9361bd3dfc11d19950a6af6d8ed2df61bd8b0134d",
    "height": "aeecb1d0ae367f836e95f5fec8edcacf37df423f07f9ecaa1739debab8985b58",
    "cloud": "9a2ee6ac43f6db684179a5515a498c6d6d6f4bee11134357dcc94dfb4d664f65",
    "cloud_polar": "daddaad84d7a33bbbc86cdda3f591099f57cee8607b7bcf3b67eb7e4f7a1c793",
}
EXPECTED_SIZE = {
    "day": (21600, 10800), "height": (21600, 10800),
    "night": (13500, 6750), "cloud": SIZE, "cloud_polar": (2048, 1024),
}


def fetch(item):
    name, url = item
    path = CACHE / Path(url).name
    if not path.exists():
        with urllib.request.urlopen(url, timeout=45) as response:
            data = response.read()
        # Validate image before caching an error/redirect response.
        from io import BytesIO
        Image.open(BytesIO(data)).verify()
        path.write_bytes(data)
    digest = hashlib.sha256(path.read_bytes()).hexdigest()
    if digest != EXPECTED_SHA256[name]:
        raise ValueError(f"{name}: source hash changed; review before replacing baked data")
    image = Image.open(path)
    if image.size != EXPECTED_SIZE[name]:
        raise ValueError(f"{name}: unexpected native dimensions {image.size}")
    print(name, image.size, digest, flush=True)
    return name, path


def linear(encoded):
    return np.where(encoded <= 0.04045, encoded / 12.92, ((encoded + 0.055) / 1.055) ** 2.4)


def encode(value):
    value = np.clip(value, 0, 1)
    return np.uint8(np.round(np.where(value <= 0.0031308, value * 12.92, 1.055 * value ** (1 / 2.4) - 0.055) * 255))


def resize_float(value, size):
    if value.ndim == 2:
        return np.asarray(Image.fromarray(value).resize(size, Image.Resampling.LANCZOS))
    return np.stack([np.asarray(Image.fromarray(value[:, :, i]).resize(size, Image.Resampling.LANCZOS)) for i in range(value.shape[2])], axis=-1)


def load_base(path, channels, srgb=False):
    """Reduce in linear light before assembling RGB (bounded native float memory)."""
    table = np.arange(256, dtype=np.float32) / 255
    if srgb:
        table = linear(table)
    reduced = []
    with Image.open(path) as image:
        for channel in channels:
            # At most one 21600×10800 float channel exists at a time. Assembling
            # a native float RGB array here would unnecessarily exceed 2.6 GiB.
            source = np.asarray(image.getchannel(channel))
            reduced.append(np.clip(resize_float(table[source], SIZE), 0, 1))
    return reduced[0] if len(reduced) == 1 else np.stack(reduced, axis=-1)


def atlas(name, value):
    if value.shape[:2] != (SIZE[1], SIZE[0]):
        raise ValueError(f"{name}: incorrect base dimensions {value.shape}")
    # Cloud is scalar: lossless L-mode PNG avoids triplicating its payload.
    # Runtime RGBA expansion preserves the same shader's red-channel contract.
    result = Image.new("L" if value.ndim == 2 else "RGB", (SIZE[0], SIZE[0]))
    y = 0
    while True:
        h, w = value.shape[:2]
        result.paste(Image.fromarray(encode(value)), (0, y))
        y += h
        if w == 1 and h == 1:
            break
        value = resize_float(value, (max(1, w // 2), max(1, h // 2)))
    path = STAGING / (name + ".png")
    result.save(path, optimize=True)
    if path.stat().st_size >= 50 * 1024 * 1024:
        raise ValueError(f"{name}: PNG exceeds the 50 MiB checked-in asset limit")
    print("baked", name, f"{path.stat().st_size / 1024**2:.2f} MiB", flush=True)


def main():
    CACHE.mkdir(parents=True, exist_ok=True)
    OUT.mkdir(parents=True, exist_ok=True)
    STAGING.mkdir(parents=True, exist_ok=True)
    with ThreadPoolExecutor(max_workers=5) as pool:
        sources = dict(pool.map(fetch, SOURCES.items()))
    day = load_base(sources["day"], [0, 1, 2], srgb=True)
    atlas("day", day)
    height = load_base(sources["height"], [0])
    # Derived shading mask, not an authoritative coastline or sea-ice dataset.
    day_encoded = encode(day).astype(np.float32) / 255
    water = ((day_encoded[:, :, 2] > day_encoded[:, :, 0] * 1.25 + 0.015)
             & (day_encoded[:, :, 2] > day_encoded[:, :, 1] * 1.02)
             & (height < 0.018)).astype(np.float32)
    del day, day_encoded
    atlas("relief", np.stack([height, water, np.zeros_like(height)], axis=-1))
    del height, water
    # The grayscale Black Marble map contains measured night light radiance without
    # a daytime base layer. A restrained warm-neutral tint is an artistic choice.
    lights = load_base(sources["night"], [0], srgb=True)
    atlas("night", lights[:, :, None] * np.array([1.0, 0.78, 0.48], dtype=np.float32))
    del lights
    # SVS frame01000 is 2014-06-02 00:00UTC. Its alpha is the visualized cloud
    # opacity; near-white RGB is cloud color, not coverage. CPC ends at +/-60°.
    # Fill missing polar coverage with the older global Blue Marble composite,
    # blending only 50..59° to avoid presenting missing data as clear polar skies.
    cloud = load_base(sources["cloud"], ["A"])
    polar = load_base(sources["cloud_polar"], [0])
    latitude = np.abs(90 - (np.arange(SIZE[1], dtype=np.float32) + 0.5) * 180 / SIZE[1])
    blend = np.clip((latitude - 50) / 9, 0, 1)
    blend = (blend * blend * (3 - 2 * blend))[:, None]
    cloud *= 1 - blend
    cloud += polar * blend
    del polar
    atlas("cloud", cloud)
    manifest = {name: {"url": SOURCES[name], "sha256": hashlib.sha256(path.read_bytes()).hexdigest(), "dimensions": Image.open(path).size} for name, path in sources.items()}
    (STAGING / "sources.json").write_text(json.dumps(manifest, indent=2) + "\n", encoding="utf-8")
    # Leave the shipped assets intact if any source/size validation or bake fails.
    # Callers must still avoid compiling while this final publication runs.
    for name in ["day.png", "night.png", "cloud.png", "relief.png", "sources.json"]:
        (STAGING / name).replace(OUT / name)


if __name__ == "__main__":
    main()
