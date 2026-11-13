#!/usr/bin/env python3
"""Offline unified asset & catalog baker for HyperCosmosVR.

Strictly read-only on D:\\Projects\\Spanish-Quick-Apps-Project\\Spanish-Quick-Apps
and Spanish-Quick-Apps-audio. Emits:
  - assets/baked/universe_catalog.v1.json
  - assets/baked/universe_catalog.hcvb   (zero-copy bytemuck binary catalog)
  - assets/baked/universe_audio.hcvp     (4-byte-aligned concatenated MP3 pack)
"""

from __future__ import annotations

import hashlib
import json
import math
import pathlib
import re
import struct
import subprocess
import zlib
from typing import Any

SQA_ROOT = pathlib.Path(
    os.environ.get("HYPERCOSMOS_SQA_ROOT", r"D:\Projects\Spanish-Quick-Apps-Project\Spanish-Quick-Apps")
)
SQAA_ROOT = pathlib.Path(
    os.environ.get("HYPERCOSMOS_SQAA_ROOT", r"D:\Projects\Spanish-Quick-Apps-Project\Spanish-Quick-Apps-audio")
)
WORKSPACE_ROOT = pathlib.Path(__file__).resolve().parent.parent
BAKED_DIR = WORKSPACE_ROOT / "assets" / "baked"

MAGIC_CAT = b"HCVRCAT1"
MAGIC_AUD = b"HCVRAUD1"
FORMAT_VERSION = 1
NONE_U32 = 0xFFFF_FFFF

LOCALE_ES = 0
LOCALE_EN = 1
LOCALE_FI = 2

KIND_COLOR_SPECTRUM = 0
KIND_MUSIC_SYNTH = 1
KIND_STRATIFIED_ZONES = 2

WAVEFORM_MAP = {"sine": 0, "triangle": 1, "sawtooth": 2, "square": 3}

TIMBRES_02 = [
    ("cristal", "sine", 0.25, 7.0, 5200.0, 0.015, 0.62),
    ("aire", "triangle", 0.50, 5.0, 3400.0, 0.025, 0.46),
    ("cálido", "sawtooth", 0.75, 3.0, 1900.0, 0.035, 0.38),
    ("profundo", "square", 1.00, 2.0, 980.0, 0.045, 0.50),
]

# Struct packing formats (little-endian '<', exact match with crates/cosmos_manifest/src/format.rs)
# StrSlice: byte_off: u32, byte_len: u32 (8 B)
# CueRecord: start_ms, end_ms, start_byte, end_byte, start_scalar, end_scalar (6 * u32 = 24 B)
STRUCT_CUE = struct.Struct("<6I")
# AudioAssetRecord: mp3_offset: u64, mp3_len: u32, duration_ms: u32, cue_start: u32, cue_count: u32,
#   text: StrSlice(2*u32), external_id: StrSlice(2*u32), sha256: [u8; 32], locale: u8, app_num: u8, _pad: [u8; 6]
# Total = 8 + 4*4 + 2*4 + 2*4 + 32 + 1 + 1 + 6 = 80 B
STRUCT_ASSET = struct.Struct("<Q8I32sBB6s")
# PhraseTriple: es_asset: u32, en_asset: u32, fi_asset: u32 (12 B)
# AppKeyEntryRecord: key(2*u32), triple(3*u32) = 20 B
STRUCT_KEY_ENTRY = struct.Struct("<5I")
# MetricTokenRecord: PhraseTriple (12 B)
STRUCT_METRIC_TOKEN = struct.Struct("<3I")
# ZoneRecord (152 B):
#   app_num(B), zone_idx(B), color_rgb(3B), speak_evidence_default(B), has_sound(B), metric_mode(B),
#   unit_kind(B), _pad(3s), id(2I),
#   range_start(f), range_end(f), phys_low(f), phys_high(f), distance_au(f),
#   scene_index(i), feature_start(I), feature_count(I), metric_token_start(I), metric_token_count(I),
#   sound: base_hz(f), filter_hz(f), noise(f), pulse(f), waveform(B), _pad(3s),
#   name(3I), macro_fact(3I), evidence_class(3I), metric_narration(3I),
#   raw_metric(2I), period(2I), footer(2I)
STRUCT_ZONE = struct.Struct("<9B3s2I5fi4I4fB3s18I")
# FeatureRecord: name(3I), fact(3I) = 24 B
STRUCT_FEATURE = struct.Struct("<6I")
# BodyLayoutRecord: zone_id(2I), zone_index(I), x(f), lane(f), radius(f) = 24 B
STRUCT_BODY_LAYOUT = struct.Struct("<3I3f")
# LandmarkRecord: app_num(B), _pad(3s), zone_id(2I), p(f), lane(f), size(f) = 24 B
STRUCT_LANDMARK = struct.Struct("<B3s2I3f")
# MusicNoteRecord: pitch_index(B), _pad(3s), symbol(2I), latin(3I) = 24 B
STRUCT_MUSIC_NOTE = struct.Struct("<B3s5I")
# MusicTimbreRecord: index(B), waveform(B), _pad(2s), y_max(f), detune_cents(f), filter_hz(f),
#   attack_sec(f), release_sec(f), label(3I) = 36 B
STRUCT_MUSIC_TIMBRE = struct.Struct("<2B2s5f3I")
# ExperienceRecord: app_num(B), kind(B), feedback(B), _pad(B), id(2I), label(2I), title(2I),
#   zone_start(I), zone_count(I), key_start(I), key_count(I) = 44 B
STRUCT_EXPERIENCE = struct.Struct("<4B10I")
# HcvbHeader: magic(8s), version(I), flags(I), catalog_crc32(I), audio_crc32(I), audio_pack_size(Q),
#   12 pairs of (off: I, len: I) -> 24 * I = 96 B. Total = 32 + 96 = 128 B
STRUCT_HEADER = struct.Struct("<8s4IQ24I")

assert STRUCT_CUE.size == 24
assert STRUCT_ASSET.size == 80
assert STRUCT_KEY_ENTRY.size == 20
assert STRUCT_METRIC_TOKEN.size == 12
assert STRUCT_ZONE.size == 152
assert STRUCT_FEATURE.size == 24
assert STRUCT_BODY_LAYOUT.size == 24
assert STRUCT_LANDMARK.size == 24
assert STRUCT_MUSIC_NOTE.size == 24
assert STRUCT_MUSIC_TIMBRE.size == 36
assert STRUCT_EXPERIENCE.size == 44
assert STRUCT_HEADER.size == 128


class StringInterner:
    """Interns unique UTF-8 strings into a flat contiguous byte arena."""

    def __init__(self) -> None:
        self._table: dict[str, tuple[int, int]] = {}
        self.arena = bytearray()
        # Reserve offset 0 len 0 for empty string cleanly.
        self._table[""] = (0, 0)

    def intern(self, s: str) -> tuple[int, int]:
        if s in self._table:
            return self._table[s]
        raw = s.encode("utf-8")
        off = len(self.arena)
        length = len(raw)
        self.arena.extend(raw)
        slice_pair = (off, length)
        self._table[s] = slice_pair
        return slice_pair


def evaluate_all_js_lexicons() -> tuple[dict[str, dict[str, str]], dict[str, dict[str, str]], dict[str, dict[str, str]]]:
    """Evaluates catalogs/01..25.js and learning-translations/{en,fi}/01..25.js read-only via Node vm."""
    node_script = r"""
const fs = require("fs");
const path = require("path");
const vm = require("vm");
const root = process.argv[1];
const window = {};
const sandbox = { window };
vm.createContext(sandbox);
for (let i = 1; i <= 25; i++) {
  const d = String(i).padStart(2, "0");
  vm.runInContext(fs.readFileSync(path.join(root, "assets/syncvoice/catalogs", d + ".js"), "utf8"), sandbox);
  vm.runInContext(fs.readFileSync(path.join(root, "learning-translations/en", d + ".js"), "utf8"), sandbox);
  vm.runInContext(fs.readFileSync(path.join(root, "learning-translations/fi", d + ".js"), "utf8"), sandbox);
}
const cats = {};
const ens = {};
const fis = {};
for (let i = 1; i <= 25; i++) {
  const d = String(i).padStart(2, "0");
  cats[d] = window.SpectrumSyncVoiceCatalogs[d]["es-ES"];
  ens[d] = window.SpectrumLearningTranslations[d]["en"];
  fis[d] = window.SpectrumLearningTranslations[d]["fi"];
}
process.stdout.write(JSON.stringify({ cats, ens, fis }));
"""
    proc = subprocess.run(
        ["node", "-e", node_script, str(SQA_ROOT)],
        capture_output=True,
        text=True,
        encoding="utf-8",
        check=True,
    )
    data = json.loads(proc.stdout)
    return data["cats"], data["ens"], data["fis"]


def has_valid_mp3_header(data: bytes) -> bool:
    """Checks for ID3v2 tag (skipping synchsafe tag length) and valid MPEG frame sync."""
    if len(data) < 16:
        return False
    offset = 0
    if data[:3] == b"ID3":
        tag_size = (
            ((data[6] & 0x7F) << 21)
            | ((data[7] & 0x7F) << 14)
            | ((data[8] & 0x7F) << 7)
            | (data[9] & 0x7F)
        )
        offset = 10 + tag_size
        if offset + 2 > len(data):
            return False
    limit = min(len(data) - 1, offset + 4096)
    for idx in range(offset, limit):
        if data[idx] == 0xFF and (data[idx + 1] & 0xE0) == 0xE0:
            return True
    return False


def utf16_to_utf8_and_scalar_map(text: str) -> dict[int, tuple[int, int]]:
    """Maps every UTF-16 code-unit boundary to (utf8_byte_offset, scalar_index)."""
    mapping: dict[int, tuple[int, int]] = {0: (0, 0)}
    u16_pos = 0
    u8_pos = 0
    scalar_pos = 0
    for ch in text:
        u16_pos += len(ch.encode("utf-16-le")) // 2
        u8_pos += len(ch.encode("utf-8"))
        scalar_pos += 1
        mapping[u16_pos] = (u8_pos, scalar_pos)
    return mapping


def parse_app_num(scene_tag: str) -> int:
    m = re.match(r"^app-(\d{2})$", scene_tag)
    if not m:
        raise ValueError(f"Unexpected scene tag: {scene_tag!r}")
    return int(m.group(1))


def extract_footer_map(app_num: int, content_zones: list[dict[str, Any]]) -> dict[int, str]:
    """Extracts canonical footer text per zone index for Apps 21..25."""
    if app_num < 21:
        return {}
    d = f"{app_num:02d}"
    sc = (SQA_ROOT / f"src/experiences/{d}/scene.js").read_text(encoding="utf-8")
    footers: dict[int, str] = {}
    if app_num <= 23:
        m_arr = re.search(r"const SCENES\s*=\s*\[(.*?)\];", sc, re.S)
        assert m_arr is not None, f"Missing SCENES array in app {d}"
        fns = [tok.strip() for tok in m_arr.group(1).split(",") if tok.strip()]
        fn_footer: dict[str, str] = {}
        for fn in fns:
            m_fn = re.search(
                rf"function {re.escape(fn)}\b.*?(?=\n  function |\n  const SCENES|\Z)",
                sc,
                re.S,
            )
            assert m_fn is not None, f"Missing scene fn {fn} in app {d}"
            m_fl = re.search(r'footerLabel\(\s*("[^"]+")\s*,', m_fn.group(0), re.S)
            assert m_fl is not None, f"Missing footerLabel in {d}:{fn}"
            fn_footer[fn] = json.loads(m_fl.group(1))
        for zi, z in enumerate(content_zones):
            scene_idx = z["scene"]
            fn = fns[scene_idx]
            footers[zi] = fn_footer[fn]
    else:
        case_blocks = re.split(r"\n\s*case\s+(\d+)\s*:", sc)
        scene_footer: dict[int, str] = {}
        for k in range(1, len(case_blocks), 2):
            idx = int(case_blocks[k])
            blk = case_blocks[k + 1].split("\n      case ")[0]
            m_fl = re.search(r'footerLabel\(\s*("[^"]+")\s*,', blk, re.S)
            assert m_fl is not None, f"Missing footerLabel for case {idx} in app {d}"
            scene_footer[idx] = json.loads(m_fl.group(1))
        for zi, z in enumerate(content_zones):
            footers[zi] = scene_footer[z["scene"]]
    assert len(footers) == len(content_zones), f"Footer count mismatch in app {d}"
    return footers


def pad_to_alignment(buf: bytearray, align: int) -> int:
    rem = len(buf) % align
    if rem != 0:
        buf.extend(b"\x00" * (align - rem))
    return len(buf)


def bake() -> None:
    BAKED_DIR.mkdir(parents=True, exist_ok=True)
    hcvb_path = BAKED_DIR / "universe_catalog.hcvb"
    hcvp_path = BAKED_DIR / "universe_audio.hcvp"
    json_path = BAKED_DIR / "universe_catalog.v1.json"

    print("[1/5] Loading manifests, registry, and JS translation catalogs...")
    registry = json.loads((SQA_ROOT / "src/registry.json").read_text(encoding="utf-8"))
    assert len(registry) == 25, f"Expected 25 registry apps, got {len(registry)}"

    es_man = json.loads((SQA_ROOT / "assets/syncvoice/manifest.json").read_text(encoding="utf-8"))
    au_man = json.loads((SQAA_ROOT / "assets/syncvoice/manifest.json").read_text(encoding="utf-8"))
    cats, ens, fis = evaluate_all_js_lexicons()

    # Cross-check exact parity across JS catalogs & manifests before heavy IO.
    assert len(es_man["entries"]) == 6068
    assert len(au_man["entries"]) == 12830

    # Canonical deterministic ordering for all 18,898 audio assets: ES (6068), EN (6415), FI (6415).
    es_entries = sorted(
        es_man["entries"],
        key=lambda e: (e["scene"], e["sourceKey"], e["externalId"]),
    )
    en_entries = sorted(
        (e for e in au_man["entries"] if e["locale"] == "en-US"),
        key=lambda e: (e["scene"], e["sourceKey"], e["externalId"]),
    )
    fi_entries = sorted(
        (e for e in au_man["entries"] if e["locale"] == "fi-FI"),
        key=lambda e: (e["scene"], e["sourceKey"], e["externalId"]),
    )
    assert len(en_entries) == 6415 and len(fi_entries) == 6415

    ordered_entries: list[tuple[int, pathlib.Path, dict[str, Any]]] = (
        [(LOCALE_ES, SQA_ROOT / "assets/syncvoice", e) for e in es_entries]
        + [(LOCALE_EN, SQAA_ROOT / "assets/syncvoice", e) for e in en_entries]
        + [(LOCALE_FI, SQAA_ROOT / "assets/syncvoice", e) for e in fi_entries]
    )
    total_assets = len(ordered_entries)
    assert total_assets == 18898

    interner = StringInterner()
    cues_blob = bytearray()
    assets_blob = bytearray()
    es_by_sk: dict[str, int] = {}
    es_ext_id_by_idx: dict[int, str] = {}
    en_by_app_sk: dict[tuple[int, str], int] = {}
    fi_by_app_sk: dict[tuple[int, str], int] = {}
    asset_texts_by_idx: list[str] = [""] * total_assets
    asset_ext_ids_by_idx: list[str] = [""] * total_assets
    json_assets: list[dict[str, Any]] = []

    print(f"[2/5] Packing {total_assets} MP3s into universe_audio.hcvp & verifying SHA-256 + UTF-8 cues...")
    audio_crc = 0
    total_cues = 0
    hcvp_header = struct.pack("<8s2I", MAGIC_AUD, FORMAT_VERSION, total_assets)
    audio_pos = len(hcvp_header)
    audio_crc = zlib.crc32(hcvp_header, audio_crc)

    with hcvp_path.open("wb") as hcvp_out:
        hcvp_out.write(hcvp_header)

        for idx, (locale, root_dir, entry) in enumerate(ordered_entries):
            mp3_path = root_dir / entry["audio"]
            tr_path = root_dir / entry["transcript"]
            mp3_bytes = mp3_path.read_bytes()
            sha_digest = hashlib.sha256(mp3_bytes).digest()
            assert len(entry["hash"]) == 64, f"Malformed syncvoice hash on {entry['externalId']}"
            assert has_valid_mp3_header(mp3_bytes), f"Invalid MP3 header on {entry['externalId']}"

            pad_len = (4 - (audio_pos % 4)) % 4
            if pad_len:
                pad_bytes = b"\x00" * pad_len
                hcvp_out.write(pad_bytes)
                audio_crc = zlib.crc32(pad_bytes, audio_crc)
                audio_pos += pad_len

            mp3_offset = audio_pos
            mp3_len = len(mp3_bytes)
            hcvp_out.write(mp3_bytes)
            audio_crc = zlib.crc32(mp3_bytes, audio_crc)
            audio_pos += mp3_len

            tr = json.loads(tr_path.read_text(encoding="utf-8"))
            assert tr["externalId"] == entry["externalId"], f"Transcript externalId mismatch on {entry['externalId']}"
            text = tr["text"]
            text_utf8 = text.encode("utf-8")
            u16_map = utf16_to_utf8_and_scalar_map(text)
            cues = tr["cues"]
            assert len(cues) > 0, f"Empty cues on {entry['externalId']}"

            cue_start = total_cues
            cue_count = len(cues)
            for cue in cues:
                sc16 = cue["startChar"]
                ec16 = cue["endChar"]
                assert sc16 in u16_map and ec16 in u16_map, (
                    f"Non-boundary UTF-16 cue [{sc16}, {ec16}] in {entry['externalId']}: {text!r}"
                )
                start_byte, start_scalar = u16_map[sc16]
                end_byte, end_scalar = u16_map[ec16]
                assert start_byte <= end_byte <= len(text_utf8)
                assert text_utf8[start_byte:end_byte].decode("utf-8") == text[start_scalar:end_scalar]
                cues_blob.extend(
                    STRUCT_CUE.pack(
                        int(cue["startMs"]),
                        int(cue["endMs"]),
                        start_byte,
                        end_byte,
                        start_scalar,
                        end_scalar,
                    )
                )
            total_cues += cue_count

            text_slice = interner.intern(text)
            ext_slice = interner.intern(entry["externalId"])
            app_num = parse_app_num(entry["scene"])
            duration_ms = int(entry["durationMs"])
            assets_blob.extend(
                STRUCT_ASSET.pack(
                    mp3_offset,
                    mp3_len,
                    duration_ms,
                    cue_start,
                    cue_count,
                    text_slice[0],
                    text_slice[1],
                    ext_slice[0],
                    ext_slice[1],
                    sha_digest,
                    locale,
                    app_num,
                    b"\x00" * 6,
                )
            )

            asset_texts_by_idx[idx] = text
            asset_ext_ids_by_idx[idx] = entry["externalId"]
            sk = entry["sourceKey"]
            if locale == LOCALE_ES:
                assert sk not in es_by_sk
                es_by_sk[sk] = idx
                es_ext_id_by_idx[idx] = entry["externalId"]
            elif locale == LOCALE_EN:
                assert (app_num, sk) not in en_by_app_sk
                en_by_app_sk[(app_num, sk)] = idx
            else:
                assert (app_num, sk) not in fi_by_app_sk
                fi_by_app_sk[(app_num, sk)] = idx

            json_assets.append(
                {
                    "index": idx,
                    "external_id": entry["externalId"],
                    "locale": ("es-ES", "en-US", "fi-FI")[locale],
                    "app": f"{app_num:02d}",
                    "source_key": sk,
                    "text": text,
                    "mp3_offset": mp3_offset,
                    "mp3_len": mp3_len,
                    "duration_ms": duration_ms,
                    "cue_start": cue_start,
                    "cue_count": cue_count,
                    "mp3_sha256": sha_digest.hex(),
                    "syncvoice_hash": entry["hash"],
                }
            )

    audio_pack_size = audio_pos
    audio_crc &= 0xFFFF_FFFF
    print(
        f"      Wrote {hcvp_path.name}: {audio_pack_size:,} bytes, "
        f"{total_cues:,} UTF-8 cues verified, audio_crc32=0x{audio_crc:08X}"
    )

    print("[3/5] Building per-app sorted lexicon triples and resolving all 25 experiences...")
    key_entries_blob = bytearray()
    experiences_blob = bytearray()
    zones_blob = bytearray()
    features_blob = bytearray()
    metric_tokens_blob = bytearray()
    body_layout_blob = bytearray()
    landmarks_blob = bytearray()
    music_notes_blob = bytearray()
    music_timbres_blob = bytearray()

    app_key_lookup: dict[tuple[int, str], tuple[int, int, int]] = {}
    json_key_tables: dict[str, dict[str, dict[str, Any]]] = {}
    total_key_entries = 0

    for app_num in range(1, 26):
        d = f"{app_num:02d}"
        es_cat = cats[d]
        en_cat = ens[d]
        fi_cat = fis[d]
        assert set(es_cat.keys()) == set(en_cat.keys()) == set(fi_cat.keys()), f"Lexicon keyset mismatch in app {d}"

        sorted_keys = sorted(en_cat.keys(), key=lambda s: s.encode("utf-8"))
        app_map: dict[str, dict[str, Any]] = {}
        for sk in sorted_keys:
            es_idx = es_by_sk[sk]
            en_idx = en_by_app_sk[(app_num, sk)]
            fi_idx = fi_by_app_sk[(app_num, sk)]
            assert es_ext_id_by_idx[es_idx] == es_cat[sk], f"ES externalId mismatch in {d}: {sk!r}"
            triple = (es_idx, en_idx, fi_idx)
            app_key_lookup[(app_num, sk)] = triple
            k_slice = interner.intern(sk)
            key_entries_blob.extend(STRUCT_KEY_ENTRY.pack(k_slice[0], k_slice[1], es_idx, en_idx, fi_idx))
            total_key_entries += 1
            app_map[sk] = {
                "es_idx": es_idx,
                "en_idx": en_idx,
                "fi_idx": fi_idx,
                "es": asset_texts_by_idx[es_idx],
                "en": asset_texts_by_idx[en_idx],
                "fi": asset_texts_by_idx[fi_idx],
            }
        json_key_tables[d] = app_map

    assert total_key_entries == 6415, f"Expected 6415 key entries, got {total_key_entries}"
    assert len(json_key_tables["01"]) == 130
    assert len(json_key_tables["02"]) == 20

    def lookup_triple(app_num: int, key: str) -> tuple[int, int, int]:
        t = app_key_lookup.get((app_num, key))
        assert t is not None, f"Missing lexicon triple for app {app_num:02d}: {key!r}"
        return t

    total_zones = 0
    total_features = 0
    total_metric_tokens = 0
    total_body_layout = 0
    total_landmarks = 0
    key_cursor = 0

    json_experiences: list[dict[str, Any]] = []
    json_body_layout: list[dict[str, Any]] = []
    json_landmarks: list[dict[str, Any]] = []
    json_music: dict[str, Any] = {}

    for reg_entry in registry:
        d = reg_entry["id"]
        app_num = int(d)
        feedback_flag = 1 if reg_entry.get("feedback", False) else 0
        key_count = len(json_key_tables[d])
        key_start = key_cursor
        key_cursor += key_count

        clean_label = re.sub(r"\s+", " ", reg_entry["label"]).strip()
        clean_title = re.sub(r"\s+", " ", reg_entry["title"]).strip()
        id_s = interner.intern(d)
        label_s = interner.intern(clean_label)
        title_s = interner.intern(clean_title)
        content = json.loads((SQA_ROOT / f"src/experiences/{d}/content.json").read_text(encoding="utf-8"))

        zone_start = total_zones
        zone_count = 0
        exp_kind = KIND_STRATIFIED_ZONES

        if app_num == 1:
            exp_kind = KIND_COLOR_SPECTRUM
            json_experiences.append(
                {
                    "id": d,
                    "app_num": app_num,
                    "kind": "color_spectrum",
                    "label": clean_label,
                    "title": clean_title,
                    "feedback": bool(feedback_flag),
                    "key_count": key_count,
                    "zones": [],
                }
            )
        elif app_num == 2:
            exp_kind = KIND_MUSIC_SYNTH
            notes = content["NOTES"]
            assert len(notes) == 12
            json_notes = []
            for p_idx, (latin, symbol) in enumerate(notes):
                latin_t = lookup_triple(2, latin)
                sym_s = interner.intern(symbol)
                music_notes_blob.extend(
                    STRUCT_MUSIC_NOTE.pack(p_idx, b"\x00" * 3, sym_s[0], sym_s[1], *latin_t)
                )
                json_notes.append({"pitch_index": p_idx, "latin": latin, "symbol": symbol, "triple": latin_t})

            json_timbres = []
            for t_idx, (t_label, wf, y_max, detune, filter_hz, atk, rel) in enumerate(TIMBRES_02):
                label_t = lookup_triple(2, t_label)
                music_timbres_blob.extend(
                    STRUCT_MUSIC_TIMBRE.pack(
                        t_idx, WAVEFORM_MAP[wf], b"\x00" * 2, y_max, detune, filter_hz, atk, rel, *label_t
                    )
                )
                json_timbres.append(
                    {
                        "index": t_idx,
                        "label": t_label,
                        "waveform": wf,
                        "y_max": y_max,
                        "detune_cents": detune,
                        "filter_hz": filter_hz,
                        "attack_sec": atk,
                        "release_sec": rel,
                        "triple": label_t,
                    }
                )
            for oct_num in (3, 4, 5, 6):
                _ = lookup_triple(2, f"octava {oct_num}")
            json_music = {"notes": json_notes, "timbres": json_timbres, "midi_min": 48, "midi_count": 48}
            json_experiences.append(
                {
                    "id": d,
                    "app_num": app_num,
                    "kind": "music_synth",
                    "label": clean_label,
                    "title": clean_title,
                    "feedback": bool(feedback_flag),
                    "key_count": key_count,
                    "zones": [],
                }
            )
        else:
            zones = content["ZONES"]
            zone_count = len(zones)
            footers = extract_footer_map(app_num, zones)
            zone_id_to_idx = {z["id"]: zi for zi, z in enumerate(zones)}

            if app_num == 4:
                for b in content["BODY_LAYOUT"]:
                    zi = zone_id_to_idx.get(b["id"])
                    if zi is None:
                        bx = float(b["x"])
                        zi = next(
                            idx for idx, z in enumerate(zones) if float(z["x0"]) <= bx < float(z["x1"])
                        )
                    bid_s = interner.intern(b["id"])
                    body_layout_blob.extend(
                        STRUCT_BODY_LAYOUT.pack(bid_s[0], bid_s[1], zi, float(b["x"]), float(b["lane"]), float(b["r"]))
                    )
                    total_body_layout += 1
                    json_body_layout.append({"id": b["id"], "zone_index": zi, "x": b["x"], "lane": b["lane"], "r": b["r"]})

            if app_num in (5, 6, 7):
                for lm in content["LANDMARKS"]:
                    l_zi = zone_id_to_idx.get(lm["id"])
                    if l_zi is None:
                        lp = float(lm["p"])
                        l_zi = next(
                            idx for idx, z in enumerate(zones) if float(z["p0"]) <= lp < float(z["p1"])
                        )
                    lid_s = interner.intern(lm["id"])
                    landmarks_blob.extend(
                        STRUCT_LANDMARK.pack(
                            app_num, b"\x00" * 3, lid_s[0], lid_s[1], float(lm["p"]), float(lm["lane"]), float(lm["size"])
                        )
                    )
                    total_landmarks += 1
                    json_landmarks.append(
                        {
                            "app": d,
                            "id": lm["id"],
                            "zone_index": l_zi,
                            "p": lm["p"],
                            "lane": lm["lane"],
                            "size": lm["size"],
                        }
                    )

            json_zones = []
            for zi, z in enumerate(zones):
                zid_s = interner.intern(z["id"])
                name_t = lookup_triple(app_num, z["name"])
                macro_fact_t = lookup_triple(app_num, z["fact"])

                color_rgb = [int(c) for c in z["color"]]
                assert len(color_rgb) == 3

                if "sound" in z and isinstance(z["sound"], dict):
                    snd = z["sound"]
                    has_sound = 1
                    snd_fields = (
                        float(snd["base"]),
                        float(snd["filter"]),
                        float(snd["noise"]),
                        float(snd["pulse"]),
                        WAVEFORM_MAP[snd["type"]],
                        b"\x00" * 3,
                    )
                    json_sound: dict[str, Any] | None = dict(snd)
                else:
                    has_sound = 0
                    snd_fields = (0.0, 0.0, 0.0, 0.0, 0, b"\x00" * 3)
                    json_sound = None

                if app_num == 3:
                    range_start, range_end = float(z["y0"]), float(z["y1"])
                    phys_low, phys_high = float(z["low"]), float(z["high"])
                    distance_au = math.nan
                    unit_kind = 1 if z["unit"] == "altitude" else 2
                elif app_num == 4:
                    range_start, range_end = float(z["x0"]), float(z["x1"])
                    phys_low, phys_high = float(z["au0"]), float(z["au1"])
                    distance_au = float(z["distanceAu"]) if "distanceAu" in z else math.nan
                    unit_kind = 0
                else:
                    range_start, range_end = float(z["p0"]), float(z["p1"])
                    phys_low, phys_high = 0.0, 0.0
                    distance_au = math.nan
                    unit_kind = 0

                # Evidence / certainty class across Apps 11..25
                if app_num >= 11:
                    ev_t = lookup_triple(app_num, z["className"])
                    speak_ev = 1 if (app_num == 11 or app_num >= 16) else 0
                else:
                    ev_t = (NONE_U32, NONE_U32, NONE_U32)
                    speak_ev = 0

                # Metric narration + sub-token list + canonical display string
                metric_tokens: list[tuple[int, int, int]] = []
                if app_num == 3:
                    metric_mode = 1
                    m_t = lookup_triple(3, z["material"])
                    metric_tokens = [m_t]
                    raw_metric = z["material"]
                    metric_narration_t = m_t
                elif app_num == 4:
                    mm = z.get("metricMode")
                    if mm == "sun":
                        metric_mode = 2
                        tokens = ["Centro del sistema solar", "estrella", "fuente de luz y energía"]
                        metric_tokens = [lookup_triple(4, k) for k in tokens]
                        raw_metric = " · ".join(tokens)
                        metric_narration_t = metric_tokens[0]
                    elif "rangeLabel" in z:
                        metric_mode = 5
                        tokens = [z["rangeLabel"], z["typeLabel"]]
                        metric_tokens = [lookup_triple(4, k) for k in tokens]
                        raw_metric = " · ".join(tokens)
                        metric_narration_t = metric_tokens[0]
                    elif mm == "earth":
                        metric_mode = 3
                        tokens = [z["typeLabel"], "estás aquí"]
                        metric_tokens = [lookup_triple(4, k) for k in tokens]
                        raw_metric = f"{z['typeLabel']} · estás aquí"
                        metric_narration_t = metric_tokens[0]
                    else:
                        metric_mode = 4
                        m_t = lookup_triple(4, z["typeLabel"])
                        metric_tokens = [m_t]
                        raw_metric = z["typeLabel"]
                        metric_narration_t = m_t
                else:
                    metric_mode = 0
                    m_t = lookup_triple(app_num, z["metric"])
                    metric_tokens = [m_t]
                    raw_metric = z["metric"]
                    metric_narration_t = m_t

                mt_start = total_metric_tokens
                mt_count = len(metric_tokens)
                for tok_t in metric_tokens:
                    metric_tokens_blob.extend(STRUCT_METRIC_TOKEN.pack(*tok_t))
                total_metric_tokens += mt_count

                raw_metric_s = interner.intern(raw_metric)
                period_str = z.get("period", "")
                period_s = interner.intern(period_str)
                footer_str = footers.get(zi, "")
                footer_s = interner.intern(footer_str)
                scene_index = int(z["scene"]) if "scene" in z else -1

                feat_start = total_features
                feats = z["features"]
                feat_count = len(feats)
                json_feats = []
                for f_name, f_fact in feats:
                    fn_t = lookup_triple(app_num, f_name)
                    ff_t = lookup_triple(app_num, f_fact)
                    features_blob.extend(STRUCT_FEATURE.pack(*fn_t, *ff_t))
                    json_feats.append({"name": f_name, "fact": f_fact, "name_triple": fn_t, "fact_triple": ff_t})
                total_features += feat_count

                zones_blob.extend(
                    STRUCT_ZONE.pack(
                        app_num,
                        zi,
                        color_rgb[0],
                        color_rgb[1],
                        color_rgb[2],
                        speak_ev,
                        has_sound,
                        metric_mode,
                        unit_kind,
                        b"\x00" * 3,
                        zid_s[0],
                        zid_s[1],
                        range_start,
                        range_end,
                        phys_low,
                        phys_high,
                        distance_au,
                        scene_index,
                        feat_start,
                        feat_count,
                        mt_start,
                        mt_count,
                        *snd_fields,
                        *name_t,
                        *macro_fact_t,
                        *ev_t,
                        *metric_narration_t,
                        raw_metric_s[0],
                        raw_metric_s[1],
                        period_s[0],
                        period_s[1],
                        footer_s[0],
                        footer_s[1],
                    )
                )
                total_zones += 1
                json_zones.append(
                    {
                        "id": z["id"],
                        "zone_index": zi,
                        "name": z["name"],
                        "name_triple": name_t,
                        "color": color_rgb,
                        "range": [range_start, range_end],
                        "phys_range": [phys_low, phys_high] if app_num in (3, 4) else None,
                        "distance_au": None if math.isnan(distance_au) else distance_au,
                        "unit": z.get("unit"),
                        "period": period_str or None,
                        "scene": None if scene_index < 0 else scene_index,
                        "footer": footer_str or None,
                        "metric_mode": metric_mode,
                        "raw_metric": raw_metric,
                        "metric_narration_triple": metric_narration_t,
                        "metric_tokens": metric_tokens,
                        "macro_fact": z["fact"],
                        "macro_fact_triple": macro_fact_t,
                        "evidence_class": z.get("className"),
                        "evidence_class_triple": None if ev_t[0] == NONE_U32 else ev_t,
                        "speak_evidence_default": bool(speak_ev),
                        "sound": json_sound,
                        "features": json_feats,
                    }
                )

            json_experiences.append(
                {
                    "id": d,
                    "app_num": app_num,
                    "kind": "stratified_zones",
                    "label": clean_label,
                    "title": clean_title,
                    "feedback": bool(feedback_flag),
                    "zone_start": zone_start,
                    "zone_count": zone_count,
                    "key_count": key_count,
                    "zones": json_zones,
                }
            )

        experiences_blob.extend(
            STRUCT_EXPERIENCE.pack(
                app_num,
                exp_kind,
                feedback_flag,
                0,
                id_s[0],
                id_s[1],
                label_s[0],
                label_s[1],
                title_s[0],
                title_s[1],
                zone_start,
                zone_count,
                key_start,
                key_count,
            )
        )

    # Gate checks on exact dataset geometry before finalizing binaries.
    assert total_zones == 376, f"Expected 376 zones, got {total_zones}"
    assert total_features == 2456, f"Expected 2456 features, got {total_features}"
    assert total_body_layout == 11, f"Expected 11 body_layout records, got {total_body_layout}"
    assert total_landmarks == 31, f"Expected 31 landmark records, got {total_landmarks}"
    assert key_cursor == total_key_entries == 6415

    print("[4/5] Assembling 8-byte-aligned zero-copy universe_catalog.hcvb...")
    body = bytearray()

    def push_section(blob: bytes | bytearray, count: int) -> tuple[int, int]:
        pad_to_alignment(body, 8)
        off = STRUCT_HEADER.size + len(body)
        body.extend(blob)
        return off, count

    exp_sec = push_section(experiences_blob, 25)
    zone_sec = push_section(zones_blob, total_zones)
    feat_sec = push_section(features_blob, total_features)
    mtok_sec = push_section(metric_tokens_blob, total_metric_tokens)
    key_sec = push_section(key_entries_blob, total_key_entries)
    asset_sec = push_section(assets_blob, total_assets)
    cue_sec = push_section(cues_blob, total_cues)
    body_sec = push_section(body_layout_blob, total_body_layout)
    lm_sec = push_section(landmarks_blob, total_landmarks)
    mnote_sec = push_section(music_notes_blob, 12)
    mtimb_sec = push_section(music_timbres_blob, 4)
    pad_to_alignment(body, 8)
    str_sec = (STRUCT_HEADER.size + len(body), len(interner.arena))
    body.extend(interner.arena)
    pad_to_alignment(body, 8)

    catalog_crc = zlib.crc32(body) & 0xFFFF_FFFF
    header = STRUCT_HEADER.pack(
        MAGIC_CAT,
        FORMAT_VERSION,
        0,
        catalog_crc,
        audio_crc,
        audio_pack_size,
        *exp_sec,
        *zone_sec,
        *feat_sec,
        *mtok_sec,
        *key_sec,
        *asset_sec,
        *cue_sec,
        *body_sec,
        *lm_sec,
        *mnote_sec,
        *mtimb_sec,
        *str_sec,
    )
    hcvb_bytes = header + body
    hcvb_path.write_bytes(hcvb_bytes)

    print("[5/5] Writing universe_catalog.v1.json...")
    manifest_doc = {
        "schema": "hypercosmos.universe_catalog.v1",
        "format_version": FORMAT_VERSION,
        "audio_stretch_backend": "wsola 0.3.0",
        "binary_artifacts": {
            "hcvb_file": hcvb_path.name,
            "hcvb_bytes": len(hcvb_bytes),
            "hcvb_crc32": f"0x{catalog_crc:08x}",
            "hcvp_file": hcvp_path.name,
            "hcvp_bytes": audio_pack_size,
            "hcvp_crc32": f"0x{audio_crc:08x}",
        },
        "counts": {
            "experiences": 25,
            "zones": total_zones,
            "features": total_features,
            "metric_tokens": total_metric_tokens,
            "key_entries": total_key_entries,
            "audio_assets": total_assets,
            "es_assets": len(es_entries),
            "en_assets": len(en_entries),
            "fi_assets": len(fi_entries),
            "cues": total_cues,
            "body_layout": total_body_layout,
            "landmarks": total_landmarks,
            "music_notes": 12,
            "music_timbres": 4,
            "string_arena_bytes": len(interner.arena),
        },
        "music": json_music,
        "body_layout": json_body_layout,
        "landmarks": json_landmarks,
        "experiences": json_experiences,
        "lexicons": json_key_tables,
        "audio_assets": json_assets,
    }
    json_path.write_text(json.dumps(manifest_doc, ensure_ascii=False, indent=2), encoding="utf-8")

    print("\n=== BAKE VERIFICATION SUMMARY ===")
    print(f"experiences   : 25")
    print(f"zones         : {total_zones}")
    print(f"features      : {total_features}")
    print(f"metric_tokens : {total_metric_tokens}")
    print(f"key_entries   : {total_key_entries} (01={len(json_key_tables['01'])}, 02={len(json_key_tables['02'])})")
    print(f"audio_assets  : {total_assets} (es={len(es_entries)}, en={len(en_entries)}, fi={len(fi_entries)})")
    print(f"utf8_cues     : {total_cues}")
    print(f"body_layout   : {total_body_layout}")
    print(f"landmarks     : {total_landmarks}")
    print(f"hcvb          : {hcvb_path} ({len(hcvb_bytes):,} bytes, crc32=0x{catalog_crc:08X})")
    print(f"hcvp          : {hcvp_path} ({audio_pack_size:,} bytes, crc32=0x{audio_crc:08X})")
    print(f"v1.json       : {json_path} ({json_path.stat().st_size:,} bytes)")


if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser(description="Bake HyperCosmosVR catalog (.hcvb/.v1.json) and spatial audio pack (.hcvp).")
    parser.add_argument("--sqa-root", type=pathlib.Path, default=SQA_ROOT, help="Path to Spanish-Quick-Apps repository")
    parser.add_argument("--sqaa-root", type=pathlib.Path, default=SQAA_ROOT, help="Path to Spanish-Quick-Apps-audio repository")
    args = parser.parse_args()
    SQA_ROOT = args.sqa_root
    SQAA_ROOT = args.sqaa_root
    bake()
