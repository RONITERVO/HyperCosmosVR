# HyperCosmosVR Story-Journey Plugins, Elastic Override & Recording Guide

HyperCosmosVR provides an expandable **Story-Journey** autopilot, elastic override, and 3D waypoint recording architecture implemented in [`crates/cosmos_journey`](../crates/cosmos_journey/Cargo.toml) and integrated into [`UniverseRuntime`](../crates/cosmos_app/src/engine.rs).

---

## 1. Expandable Plugin Architecture

Story-Journeys can be added in **three ways** without modifying renderer, shader, or OpenXR code:

1. **Compiled Rust Plugins (`StoryJourneyPlugin` trait)** in `cosmos_journey`:
   - `GrandCosmicOdysseyPlugin` (`id: "grand_cosmic_odyssey"`, default): Synthesizes a continuous 5-act topological route across all `25` experiences, `376` zones, `2,456` features, all `130` App `01` chromatic prism keys, all `48` MIDI × `4` timbres on App `02` (`20` music keys), and `100%` of the `18,898` baked catalog audio assets (`universe_audio.hcvp`), looping gracefully back to Cold-Start Earth Low Orbit (`u = +6.6`, `SectorId::EARTH_ORBIT`).
   - `CosmicHighlightsPlugin` (`id: "cosmic_highlights_tour"`): Curated compact 5-act highlight showcase across Earth, Solar System, Milky Way (`Sgr A*`), Exoplanet Biospheres, SETI Contact, Laniakea, CMB, Quantum/Planck scales, Life, Mind, Colors, Music, Civilization, and the Final Horizon Lotus.
2. **Portable Drop-In JSON Manifests (`*.hcj.json`)**:
   - Place any `.hcj.json` file inside [`assets/journeys/`](../assets/journeys/) or `user_journeys/`.
   - `UniverseRuntime::new` automatically discovers and registers every `*.hcj.json` file at startup via `JourneyRegistry::discover_in_directory`.
3. **Portable Share Links (`hypercosmos://journey?hcj=v1.<crc32hex>.<base64url>`)**:
   - Exported alongside every recorded `.hcj.json` in `<id>.share.txt` with CRC32 checksum verification (`StoryJourneyManifest::encode_share_uri` / `decode_share_uri`), passable directly via `--journey`.

---

## 2. Launching & Controls

The spatial welcome offers **Start highlights**, **Explore freely**, and **How to explore**. Help also offers the full Odyssey. `--autoplay-journey` skips the welcome and shows the selected tour's controls immediately.

While touring, controls show the chapter, stop count, and **Previous / Pause / Next / Explore** actions. In VR they form a room-anchored holographic instrument summoned through the wrist interaction, with volumetric nodes and curved lettering. They use real scene depth and stay fixed when the wearer turns or leans; there are no VR backplates or camera-following transport bars. Pause freezes the current camera spline, dwell timer, speech position, and SyncVoice captions. Resume continues that same clip and motion phase. Changing stops while paused stays paused and discards the old clip. Explore exits at the current pose; restarting from an exit bookmark begins that beat again. Completed tours offer Restart.

`F1` or the wrist ring's `HELP` bead opens guidance. Opening help and losing desktop/VR focus pause an active tour; closing help or returning focus requires an explicit Resume. The guide is English/Finnish (`L` / wrist language beads). Desktop controls have contrast backplates and foreground rendering for close-ups. VR uses ordinary physical occlusion. Headless content-gallery captures omit interactive controls.

### CLI Flags
```bash
# Auto-play default Grand Cosmic Odyssey on launch (VR or Desktop)
cargo run --release -p cosmos_app -- --autoplay-journey

# Launch with the portable highlights manifest or a custom file / share URI
cargo run --release -p cosmos_app -- --desktop --journey assets/journeys/cosmic_highlights_tour.hcj.json --autoplay-journey
cargo run --release -p cosmos_app -- --journey cosmic_highlights_tour --autoplay-journey
cargo run --release -p cosmos_app -- --journey "hypercosmos://journey?hcj=v1...." --autoplay-journey
```

### Diegetic 3D Wrist Resonance Ring Controls
Open the Wrist Ring in VR (`palm-up gaze`) or Desktop (`Right-Hold` or `Tab`) to access five additional outer lower-arc pearls:
- **`STORY`** (`PLAY / PAUSE`): Starts, pauses, or resumes the active Story-Journey.
- **`NEXT`**: Advances to the next Story-Journey beat.
- **`+WPT`**: Snapshots the current 6-DoF pose, logarithmic scale `scale_exp`, and focused target as a keyframe waypoint.
- **`TRACE`**: Toggles continuous live flight recording (runs 4D logarithmic-scale Ramer–Douglas–Peucker decimation upon finishing).
- `SAVE / SHARE` Pearl (`Ctrl+S` / `F9`): Exports recorded waypoints into `user_journeys/<id>.hcj.json` + `user_journeys/<id>.share.txt` and copies the `hypercosmos://` URI to the Windows clipboard.

### Desktop Keyboard Shortcuts
- `J`: Start or toggle Story-Journey play / pause; `Space` pauses/resumes an active tour or restarts a completed one.
- `F1`: Open/close help. `Escape`: close help, exit the current tour, or open help during free exploration. Close the window or use `Alt+F4` to quit.
- `N` / `PageDown`: Next beat (`Shift+N` / `Shift+PageDown`: next chapter)
- `P` / `PageUp`: Previous beat (`Shift+P` / `Shift+PageUp`: previous chapter)
- `R`: Capture manual 3D waypoint (`Shift+R`: toggle live trace recording)
- **`Ctrl+S` / `F9`**: Export recorded journey + share URI to `user_journeys/` (and copy share URI to clipboard on Windows)
- `Enter`: Confirm **`FOLLOW`** (pull straight back onto Story-Journey rail)
- `Backspace` / `Escape`: Confirm **`EXIT`** (save bookmark & switch to free-flight at current camera pose; `B` button in VR while prompt is visible)

---

## 3. Sentence-Paced Anti-Boredom Choreography

To ensure the viewer is never left stationary during multi-sentence narration:
- Each beat enforces `max_sentences_per_vantage <= 2`.
- While dwelling, the ghost rail continuously executes stereoscopic motion profiles (`OrbitalParallax`, `HelicalAscent`, `ScaleDiveCruise`, `FigureEightShowcase`).
- When `advance_audio_sequencer` finishes translation pairs inside `UniverseRuntime`, `JourneyDirector::notify_translation_sentence_finished` smoothly launches a `1.25s` micro-`ScaleSpline` vantage shift around the subject every `1–2` sentences.

---

## 4. Head-Turn Freedom, Elastic Override & Non-Occluding Exit Prompt

The camera tracking rig composes `Pose_cam = Pose_ghost ⊕ ΔPose_user`:
- **VR Head-Turn Invariance**: HMD head rotation and 6-DoF roomscale leaning act downstream on the OpenXR eye view and are never fed into `ManualOverrideInput`. You can look around freely or lean closer without fighting the camera or cancelling active narration.
- **Elastic Thumbstick / WASD Override & Magnetic Pull**: Intentional stick/WASD/scroll inputs displace `ΔPose_user` around `ghost_pose` with continuous exponential spring damping pulling smoothly back once released.
- **Leash Pause & Peripheral 3D Pearls**: Sustained large overrides (`>= leash_hold_sec`) pause `ghost_pose` progression (`PausedByLeash`) and summon twin glowing 3D pearls (`FOLLOW` and `EXIT`) linked to `ghost_pose` in the **lower-right outer periphery** (`+16.0°..+22.5°` azimuth, `-19.0°..-20.5°` elevation at `1.85m`, `>24°` off the optical axis), keeping the central `12°` foveal cone completely unobstructed and opposite the lower-left Wrist Resonance Ring.

---

## 5. Portable `.hcj.json` Schema (`schema_version = 1`)

```json
{
  "schema_version": 1,
  "id": "my_custom_tour",
  "title": { "es": "Mi Tour", "en": "My Tour", "fi": "Oma Kierros" },
  "subtitle": { "es": "", "en": "", "fi": "" },
  "author": "Creator",
  "loop_journey": true,
  "chapters": [
    {
      "id": "chapter_01",
      "act_index": 1,
      "title": { "es": "Acto I", "en": "Act I", "fi": "Näytös I" },
      "beats": [
        {
          "beat_id": "earth_low_orbit",
          "pose": {
            "sector": "EARTH_ORBIT",
            "local_pos": [0.22, 0.73, 1.01],
            "scale_exp": 6.6,
            "orient_xyzw": [-0.30088, 0.10201, 0.03239, 0.94764]
          },
          "look_at_pose": {
            "sector": "EARTH_ORBIT",
            "local_pos": [0.0, 0.0, 0.0],
            "scale_exp": 6.8,
            "orient_xyzw": [0.0, 0.0, 0.0, 1.0]
          },
          "target": {
            "kind": "CatalogZone",
            "experience_id": 3,
            "zone_index": 0,
            "feature_index": null,
            "is_macro_zoomed_out": true
          },
          "motion": {
            "kind": "OrbitalParallax",
            "yaw_deg_per_sec": 5.5,
            "pitch_wave_deg": 1.8,
            "scale_breathing_du": 0.08,
            "orbit_radius_factor": 0.35,
            "max_sentences_per_vantage": 2,
            "min_dwell_sec": 2.4,
            "max_dwell_sec": 16.0
          },
          "transition_in": {
            "kind": "CatmullRomScaleSpline",
            "duration_sec": 1.9,
            "narration_lead_sec": 0.35
          }
        }
      ]
    }
  ]
}
```
