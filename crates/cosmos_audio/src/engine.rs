use std::path::Path;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;

use cosmos_manifest::{CueProgress, PhraseTriple, Locale, UniverseCatalog, ZoneSoundRecord};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Stream, StreamConfig};
use crossbeam_channel::{bounded, Receiver, Sender};
use glam::Vec3;

use crate::decoder::{
    clamp_playback_rate, snap_playback_rate, NarrationCache, PreparedClip, OUTPUT_SAMPLE_RATE,
};
use crate::error::AudioError;
use crate::spatial::{SpatialNarrationRenderer, AUDIO_BLOCK_FRAMES};
use crate::synth::{FeedbackChimeSynth, MusicPolySynth, ZoneAmbientSynth};
use crate::uniforms::{AtomicAudioReactiveBus, AudioReactiveUniforms, OutputAnalyzer};

const DEFAULT_STOP_FADE_MS: f32 = 120.0;
const NONE_ASSET_SENTINEL: u32 = u32::MAX;

/// Audio backend initialization strategy.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AudioOutputMode {
    /// Attempt WASAPI / system default 48 kHz stereo output stream; fall back to Headless if unavailable.
    Auto,
    /// Pure deterministic software mixer (no OS audio stream opened).
    Headless,
}

/// Real-time sample-accurate SyncVoice playback progress paired cleanly with `UniverseCatalog`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SyncVoiceClockState<'a> {
    pub asset_idx: u32,
    pub source_ms: u32,
    pub duration_ms: u32,
    pub cue_index: Option<usize>,
    pub cue_progress: CueProgress<'a>,
    pub is_speaking: bool,
    pub is_finished: bool,
}

/// Owned snapshot from the mixer's atomic clock before cue lookup.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlaybackClockSnapshot {
    pub asset_idx: Option<u32>,
    pub source_ms: u32,
    pub is_speaking: bool,
    pub is_finished: bool,
    pub playback_rate: f32,
}

const SOURCE_MS_MASK: u32 = 0x3FFF_FFFF;
const SPEAKING_FLAG_BIT: u32 = 1 << 30;
const FINISHED_FLAG_BIT: u32 = 1 << 31;

/// Lock-free generation-guarded atomic clock written per 512-frame mix block on the audio thread.
///
/// Separates caller-side `pending_*` generation state from mixer-side `published_*` state so
/// restarting an `asset_idx` immediately clears stale `is_finished = true` before the next
/// WASAPI block drains the queue.
#[derive(Debug)]
pub struct AtomicPlaybackClock {
    next_gen: AtomicU64,
    pending_gen: AtomicU64,
    pending_asset_idx: AtomicU32,
    pending_rate_bits: AtomicU32,
    published_gen: AtomicU64,
    published_packed: AtomicU64,
    published_rate_bits: AtomicU32,
}

impl Default for AtomicPlaybackClock {
    fn default() -> Self {
        Self::new()
    }
}

impl AtomicPlaybackClock {
    pub fn new() -> Self {
        let initial_packed = Self::pack_snapshot(PlaybackClockSnapshot {
            asset_idx: None,
            source_ms: 0,
            is_speaking: false,
            is_finished: false,
            playback_rate: 1.0,
        });
        Self {
            next_gen: AtomicU64::new(0),
            pending_gen: AtomicU64::new(0),
            pending_asset_idx: AtomicU32::new(NONE_ASSET_SENTINEL),
            pending_rate_bits: AtomicU32::new(1.0_f32.to_bits()),
            published_gen: AtomicU64::new(0),
            published_packed: AtomicU64::new(initial_packed),
            published_rate_bits: AtomicU32::new(1.0_f32.to_bits()),
        }
    }

    #[inline]
    fn pack_snapshot(snap: PlaybackClockSnapshot) -> u64 {
        let asset_raw = snap.asset_idx.unwrap_or(NONE_ASSET_SENTINEL) as u64;
        let mut lo = snap.source_ms.min(SOURCE_MS_MASK);
        if snap.is_speaking {
            lo |= SPEAKING_FLAG_BIT;
        }
        if snap.is_finished {
            lo |= FINISHED_FLAG_BIT;
        }
        (asset_raw << 32) | (lo as u64)
    }

    #[inline]
    fn unpack_snapshot(packed: u64, rate_bits: u32) -> PlaybackClockSnapshot {
        let asset_raw = (packed >> 32) as u32;
        let lo = packed as u32;
        PlaybackClockSnapshot {
            asset_idx: if asset_raw == NONE_ASSET_SENTINEL {
                None
            } else {
                Some(asset_raw)
            },
            source_ms: lo & SOURCE_MS_MASK,
            is_speaking: (lo & SPEAKING_FLAG_BIT) != 0,
            is_finished: (lo & FINISHED_FLAG_BIT) != 0,
            playback_rate: f32::from_bits(rate_bits),
        }
    }

    pub(crate) fn allocate_generation(&self) -> u64 {
        self.next_gen.fetch_add(1, Ordering::AcqRel) + 1
    }

    pub fn play_generation(&self) -> u64 {
        self.pending_gen.load(Ordering::Acquire)
    }

    pub(crate) fn commit_pending(&self, generation: u64, asset_idx: u32, playback_rate: f32) -> u64 {
        let prev = self.pending_gen.load(Ordering::Acquire);
        self.pending_asset_idx.store(asset_idx, Ordering::Relaxed);
        self.pending_rate_bits
            .store(playback_rate.to_bits(), Ordering::Relaxed);
        self.pending_gen.store(generation, Ordering::Release);
        prev
    }

    pub(crate) fn rollback_pending(&self, expected_generation: u64, prev_generation: u64) {
        let _ = self.pending_gen.compare_exchange(
            expected_generation,
            prev_generation,
            Ordering::AcqRel,
            Ordering::Acquire,
        );
    }

    pub(crate) fn store_for_generation(&self, generation: u64, snap: PlaybackClockSnapshot) {
        self.published_rate_bits
            .store(snap.playback_rate.to_bits(), Ordering::Release);
        self.published_packed
            .store(Self::pack_snapshot(snap), Ordering::Release);
        self.published_gen.store(generation, Ordering::Release);
    }

    pub fn store(&self, snap: PlaybackClockSnapshot) {
        let generation = self.pending_gen.load(Ordering::Acquire);
        self.store_for_generation(generation, snap);
    }

    pub fn load(&self) -> PlaybackClockSnapshot {
        let pub_g = self.published_gen.load(Ordering::Acquire);
        let packed = self.published_packed.load(Ordering::Acquire);
        let pub_rate_bits = self.published_rate_bits.load(Ordering::Acquire);
        let pend_g = self.pending_gen.load(Ordering::Acquire);
        if pend_g > pub_g {
            let asset_raw = self.pending_asset_idx.load(Ordering::Acquire);
            let rate = f32::from_bits(self.pending_rate_bits.load(Ordering::Acquire));
            PlaybackClockSnapshot {
                asset_idx: if asset_raw == NONE_ASSET_SENTINEL {
                    None
                } else {
                    Some(asset_raw)
                },
                source_ms: 0,
                is_speaking: true,
                is_finished: false,
                playback_rate: rate,
            }
        } else {
            Self::unpack_snapshot(packed, pub_rate_bits)
        }
    }
}

/// Commands queued lock-free from gameplay/render threads into `AudioMixer`.
#[derive(Clone, Debug)]
pub enum AudioCommand {
    PlayClip {
        clip: PreparedClip,
        emitter_dir_cam: Vec3,
        generation: u64,
    },
    StopNarration {
        fade_ms: f32,
    },
    SetNarrationPaused(bool),
    SetNarrationEmitterDir(Vec3),
    SetZoneSound {
        sound: Option<ZoneSoundRecord>,
        scene_index: i32,
    },
    SetDwellProgress(f32),
    TriggerSpeedTick {
        bead_index: u8,
    },
    MusicNoteOn {
        id: u64,
        midi: u8,
        timbre_idx: usize,
        pan: f32,
    },
    MusicNoteUpdate {
        id: u64,
        midi: u8,
        timbre_idx: usize,
        pan: f32,
    },
    MusicNoteOff {
        id: u64,
    },
}

struct ActiveNarration {
    clip: PreparedClip,
    frame_cursor: usize,
    emitter_dir_cam: Vec3,
    fade_gain: f32,
    fade_step_per_sample: f32,
    fading_out: bool,
    finished_reported: bool,
    stopped_source_ms: Option<u32>,
}

/// Deterministic 48 kHz stereo real-time audio mixer suitable for both WASAPI callbacks and offline tests.
pub struct AudioMixer {
    cmd_rx: Receiver<AudioCommand>,
    bus: Arc<AtomicAudioReactiveBus>,
    clock: Arc<AtomicPlaybackClock>,
    spatial: SpatialNarrationRenderer,
    ambient: ZoneAmbientSynth,
    music: MusicPolySynth,
    chimes: FeedbackChimeSynth,
    analyzer: OutputAnalyzer,
    narration: Option<ActiveNarration>,
    narration_paused: bool,
    active_generation: u64,
    last_playback_rate: f32,
    block_mono_voice: [f32; AUDIO_BLOCK_FRAMES],
    fifo_stereo: [f32; AUDIO_BLOCK_FRAMES * 2],
    fifo_read_frames: usize,
    fifo_avail_frames: usize,
}

impl AudioMixer {
    pub fn new(
        cmd_rx: Receiver<AudioCommand>,
        bus: Arc<AtomicAudioReactiveBus>,
        clock: Arc<AtomicPlaybackClock>,
        spatial: SpatialNarrationRenderer,
    ) -> Self {
        Self {
            cmd_rx,
            bus,
            clock,
            spatial,
            ambient: ZoneAmbientSynth::new(),
            music: MusicPolySynth::new(),
            chimes: FeedbackChimeSynth::new(),
            analyzer: OutputAnalyzer::new(OUTPUT_SAMPLE_RATE as f32),
            narration: None,
            narration_paused: false,
            active_generation: 0,
            last_playback_rate: 1.0,
            block_mono_voice: [0.0; AUDIO_BLOCK_FRAMES],
            fifo_stereo: [0.0; AUDIO_BLOCK_FRAMES * 2],
            fifo_read_frames: 0,
            fifo_avail_frames: 0,
        }
    }

    fn start_narration(&mut self, clip: PreparedClip, emitter_dir_cam: Vec3, generation: u64) {
        self.active_generation = generation;
        self.last_playback_rate = clip.playback_rate;
        self.narration = Some(ActiveNarration {
            clip,
            frame_cursor: 0,
            emitter_dir_cam,
            fade_gain: 1.0,
            fade_step_per_sample: 0.0,
            fading_out: false,
            finished_reported: false,
            stopped_source_ms: None,
        });
    }

    /// Processes all queued lock-free commands without blocking.
    fn drain_commands(&mut self) {
        while let Ok(cmd) = self.cmd_rx.try_recv() {
            match cmd {
                AudioCommand::PlayClip {
                    clip,
                    emitter_dir_cam,
                    generation,
                } => {
                    self.start_narration(clip, emitter_dir_cam, generation);
                }
                AudioCommand::StopNarration { fade_ms } => {
                    if let Some(ref mut n) = self.narration {
                        let samples = ((fade_ms.max(1.0) / 1000.0) * OUTPUT_SAMPLE_RATE as f32)
                            .max(1.0);
                        n.fading_out = true;
                        n.fade_step_per_sample = 1.0 / samples;
                    }
                }
                AudioCommand::SetNarrationPaused(paused) => self.narration_paused = paused,
                AudioCommand::SetNarrationEmitterDir(dir) => {
                    if let Some(ref mut n) = self.narration {
                        n.emitter_dir_cam = dir;
                    }
                }
                AudioCommand::SetZoneSound { sound, scene_index } => {
                    self.ambient.set_zone(sound, scene_index);
                }
                AudioCommand::SetDwellProgress(p) => {
                    self.chimes.set_dwell_progress(p);
                }
                AudioCommand::TriggerSpeedTick { bead_index } => {
                    self.chimes.trigger_speed_tick(bead_index);
                }
                AudioCommand::MusicNoteOn {
                    id,
                    midi,
                    timbre_idx,
                    pan,
                } => {
                    self.music.note_on(id, midi, timbre_idx, pan);
                }
                AudioCommand::MusicNoteUpdate {
                    id,
                    midi,
                    timbre_idx,
                    pan,
                } => {
                    self.music.note_update(id, midi, timbre_idx, pan);
                }
                AudioCommand::MusicNoteOff { id } => {
                    self.music.note_off(id);
                }
            }
        }
    }

    /// Renders exactly `AUDIO_BLOCK_FRAMES` (`512`) stereo `f32` samples and updates reactive uniforms.
    pub fn render_block(&mut self, out_stereo: &mut [f32; AUDIO_BLOCK_FRAMES * 2]) {
        self.drain_commands();
        out_stereo.fill(0.0);
        self.block_mono_voice.fill(0.0);

        let mut voice_sum_sq = 0.0_f32;
        let mut narration_active = false;
        let mut clock_snap = PlaybackClockSnapshot {
            asset_idx: None,
            source_ms: 0,
            is_speaking: false,
            is_finished: false,
            playback_rate: self.last_playback_rate,
        };
        let mut emitter_dir = Vec3::NEG_Z;
        let mut narration_gain = 1.0_f32;
        let mut clear_narration = false;

        if let Some(ref mut n) = self.narration {
            emitter_dir = n.emitter_dir_cam;
            let samples = n.clip.samples_48k.as_slice();
            let total_len = samples.len();

            if self.narration_paused && !n.fading_out && !n.finished_reported {
                // Hold the sample cursor and SyncVoice reveal together. Ambient audio keeps
                // playing; resuming continues this clip rather than decoding it again.
                clock_snap = PlaybackClockSnapshot {
                    asset_idx: Some(n.clip.asset_idx),
                    source_ms: ((n.frame_cursor as f64 / total_len.max(1) as f64)
                        * n.clip.source_duration_ms as f64)
                        .round() as u32,
                    is_speaking: false,
                    is_finished: false,
                    playback_rate: n.clip.playback_rate,
                };
            } else if n.frame_cursor < total_len && !n.finished_reported {
                narration_active = true;
                for dst in self.block_mono_voice.iter_mut() {
                    if n.frame_cursor >= total_len {
                        break;
                    }
                    let mut s = samples[n.frame_cursor];
                    if n.fading_out {
                        n.fade_gain = (n.fade_gain - n.fade_step_per_sample).max(0.0);
                        s *= n.fade_gain;
                        if n.fade_gain <= 0.0 {
                            clear_narration = true;
                            break;
                        }
                    }
                    *dst = s;
                    voice_sum_sq += s * s;
                    n.frame_cursor += 1;
                }
                narration_gain = 1.0;

                let frac = if total_len > 0 {
                    (n.frame_cursor as f64 / total_len as f64).clamp(0.0, 1.0)
                } else {
                    1.0
                };
                let src_ms = ((frac * n.clip.source_duration_ms as f64).round() as u32)
                    .min(n.clip.source_duration_ms);
                let reached_end = n.frame_cursor >= total_len || clear_narration;
                if reached_end {
                    n.finished_reported = true;
                    n.stopped_source_ms = Some(src_ms);
                }

                clock_snap = PlaybackClockSnapshot {
                    asset_idx: Some(n.clip.asset_idx),
                    source_ms: src_ms,
                    is_speaking: !reached_end,
                    is_finished: reached_end,
                    playback_rate: n.clip.playback_rate,
                };
            } else {
                clock_snap = PlaybackClockSnapshot {
                    asset_idx: Some(n.clip.asset_idx),
                    source_ms: n.stopped_source_ms.unwrap_or(n.clip.source_duration_ms),
                    is_speaking: false,
                    is_finished: true,
                    playback_rate: n.clip.playback_rate,
                };
            }
        }

        if narration_active {
            self.spatial.mix_block(
                &self.block_mono_voice,
                emitter_dir,
                narration_gain,
                out_stereo,
            );
        }

        let voice_rms = (voice_sum_sq / AUDIO_BLOCK_FRAMES as f32).sqrt();
        let (ambient_rms, beat_phase, base_hz, filter_hz, pulse, noise) =
            self.ambient
                .mix_block(AUDIO_BLOCK_FRAMES, narration_active, out_stereo);
        let (music_energy, active_hue, active_timbre) =
            self.music.mix_block(AUDIO_BLOCK_FRAMES, out_stereo);
        let resonance_glow = self.chimes.mix_block(AUDIO_BLOCK_FRAMES, out_stereo);

        // Soft master safety saturation to prevent clipping across overlaid narration/music/drones.
        for s in out_stereo.iter_mut() {
            if !s.is_finite() {
                *s = 0.0;
            } else if s.abs() > 0.92 {
                *s = s.tanh();
            }
        }

        let (bass_energy, mid_energy, high_energy, harmonic_centroid) =
            self.analyzer.analyze_stereo_block(out_stereo);

        self.clock
            .store_for_generation(self.active_generation, clock_snap);
        self.bus.store(AudioReactiveUniforms {
            levels_a: [voice_rms, ambient_rms, bass_energy, mid_energy],
            levels_b: [
                harmonic_centroid,
                beat_phase,
                high_energy,
                self.last_playback_rate,
            ],
            synth_a: [base_hz, filter_hz, pulse, noise],
            synth_b: [active_hue, active_timbre, music_energy, resonance_glow],
        });
    }

    /// Fills any arbitrary callback frame length cleanly from the internal `512`-frame FIFO with
    /// zero heap allocations.
    pub fn render_frames_interleaved(&mut self, out_interleaved: &mut [f32]) {
        let total_frames = out_interleaved.len() / 2;
        let mut written = 0usize;

        while written < total_frames {
            if self.fifo_avail_frames == 0 {
                let mut tmp = [0.0_f32; AUDIO_BLOCK_FRAMES * 2];
                self.render_block(&mut tmp);
                self.fifo_stereo = tmp;
                self.fifo_read_frames = 0;
                self.fifo_avail_frames = AUDIO_BLOCK_FRAMES;
            }

            let copy_frames = (total_frames - written).min(self.fifo_avail_frames);
            let src_off = self.fifo_read_frames * 2;
            let dst_off = written * 2;
            let count = copy_frames * 2;
            out_interleaved[dst_off..dst_off + count]
                .copy_from_slice(&self.fifo_stereo[src_off..src_off + count]);

            self.fifo_read_frames += copy_frames;
            self.fifo_avail_frames -= copy_frames;
            written += copy_frames;
        }
    }

    pub fn active_music_voice_count(&self) -> usize {
        self.music.active_voice_count()
    }
}

/// High-level Spatial Audio & SyncVoice runtime controller (`cpal` WASAPI 48 kHz + headless fallback).
pub struct AudioEngine {
    cmd_tx: Sender<AudioCommand>,
    bus: Arc<AtomicAudioReactiveBus>,
    clock: Arc<AtomicPlaybackClock>,
    cache: NarrationCache,
    playback_rate: f32,
    uses_hrtf: bool,
    active_output_device: Option<String>,
    _stream: Option<Stream>,
    headless_mixer: Option<AudioMixer>,
}

impl std::fmt::Debug for AudioEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AudioEngine")
            .field("playback_rate", &self.playback_rate)
            .field("uses_hrtf", &self.uses_hrtf)
            .field("active_output_device", &self.active_output_device)
            .field("headless", &self.is_headless())
            .finish()
    }
}

impl AudioEngine {
    /// Launches `AudioEngine` using an optional HRIR sphere path (`irc_1040_c.bin`) and output mode.
    pub fn new(hrir_path: Option<&Path>, mode: AudioOutputMode) -> Result<Self, AudioError> {
        Self::start_with(|| match hrir_path {
            Some(path) if path.exists() => SpatialNarrationRenderer::from_hrir_file(path),
            _ => Ok(SpatialNarrationRenderer::stereo_fallback()),
        }, mode)
    }

    /// Launches `AudioEngine` from pre-loaded HRIR bytes (`irc_1040_c.bin`).
    pub fn from_hrir_bytes(hrir_bytes: &[u8], mode: AudioOutputMode) -> Result<Self, AudioError> {
        Self::start_with(|| SpatialNarrationRenderer::from_hrir_bytes(hrir_bytes), mode)
    }

    fn start_with(
        build_spatial: impl Fn() -> Result<SpatialNarrationRenderer, AudioError>,
        mode: AudioOutputMode,
    ) -> Result<Self, AudioError> {
        let (cmd_tx, cmd_rx) = bounded(256);
        let bus = Arc::new(AtomicAudioReactiveBus::new());
        let clock = Arc::new(AtomicPlaybackClock::new());
        let cache = NarrationCache::new();

        if mode == AudioOutputMode::Auto
            && let Some((stream, device_name, uses_hrtf)) = try_open_device_stream(
                &build_spatial,
                cmd_rx.clone(),
                Arc::clone(&bus),
                Arc::clone(&clock),
            )
        {
            return Ok(Self {
                cmd_tx,
                bus,
                clock,
                cache,
                playback_rate: 1.0,
                uses_hrtf,
                active_output_device: Some(device_name),
                _stream: Some(stream),
                headless_mixer: None,
            });
        }

        let spatial = build_spatial()?;
        let uses_hrtf = spatial.uses_hrtf();
        let mixer = AudioMixer::new(cmd_rx, Arc::clone(&bus), Arc::clone(&clock), spatial);

        Ok(Self {
            cmd_tx,
            bus,
            clock,
            cache,
            playback_rate: 1.0,
            uses_hrtf,
            active_output_device: None,
            _stream: None,
            headless_mixer: Some(mixer),
        })
    }

    pub const fn uses_hrtf(&self) -> bool {
        self.uses_hrtf
    }

    pub fn active_output_device(&self) -> Option<&str> {
        self.active_output_device.as_deref()
    }

    pub fn is_headless(&self) -> bool {
        self._stream.is_none()
    }

    pub const fn playback_rate(&self) -> f32 {
        self.playback_rate
    }

    pub fn set_playback_rate(&mut self, rate: f32) -> f32 {
        self.playback_rate = clamp_playback_rate(rate);
        self.playback_rate
    }

    pub fn snap_playback_rate_bead(&mut self, rate: f32, play_chime: bool) -> f32 {
        let snapped = snap_playback_rate(rate);
        self.playback_rate = snapped;
        if play_chime {
            let idx = crate::decoder::SUPPORTED_PLAYBACK_RATES
                .iter()
                .position(|&r| (r - snapped).abs() < 0.01)
                .unwrap_or(0) as u8;
            let _ = self
                .cmd_tx
                .try_send(AudioCommand::TriggerSpeedTick { bead_index: idx });
        }
        snapped
    }

    /// Prepares & starts narration for a localized `PhraseTriple` asset in `locale`.
    pub fn play_triple(
        &self,
        catalog: &UniverseCatalog,
        triple: PhraseTriple,
        locale: Locale,
        emitter_dir_cam: Vec3,
    ) -> Result<u32, AudioError> {
        let asset_idx = triple
            .asset_for(locale)
            .ok_or(AudioError::MissingAudioTrack)?;
        self.play_asset(catalog, asset_idx, emitter_dir_cam)?;
        Ok(asset_idx)
    }

    /// Returns the monotonic narration playback generation counter.
    pub fn play_generation(&self) -> u64 {
        self.clock.play_generation()
    }

    /// Decodes/stretches (using `NarrationCache`) and starts playback for `asset_idx`.
    pub fn play_asset(
        &self,
        catalog: &UniverseCatalog,
        asset_idx: u32,
        emitter_dir_cam: Vec3,
    ) -> Result<(), AudioError> {
        let clip = self
            .cache
            .prepare_clip(catalog, asset_idx, self.playback_rate)?;
        let generation = self.clock.allocate_generation();
        // Reset atomic clock BEFORE enqueueing so live WASAPI never exposes stale `is_finished`.
        let prev_gen = self
            .clock
            .commit_pending(generation, asset_idx, self.playback_rate);
        if let Err(e) = self.cmd_tx.try_send(AudioCommand::PlayClip {
            clip,
            emitter_dir_cam,
            generation,
        }) {
            self.clock.rollback_pending(generation, prev_gen);
            return Err(AudioError::OutputDevice(format!(
                "audio command queue full: {e}"
            )));
        }
        Ok(())
    }

    /// Smoothly fades out active narration over `~120 ms` when the user flies away.
    pub fn stop_narration(&self) {
        let _ = self.cmd_tx.try_send(AudioCommand::StopNarration {
            fade_ms: DEFAULT_STOP_FADE_MS,
        });
    }

    /// Pauses speech and its word cursor while preserving the current clip position.
    pub fn set_narration_paused(&self, paused: bool) {
        let _ = self
            .cmd_tx
            .try_send(AudioCommand::SetNarrationPaused(paused));
    }

    pub fn set_emitter_direction(&self, emitter_dir_cam: Vec3) {
        let _ = self
            .cmd_tx
            .try_send(AudioCommand::SetNarrationEmitterDir(emitter_dir_cam));
    }

    pub fn prefetch_neighbors(&self, catalog: &Arc<UniverseCatalog>, asset_indices: &[u32]) {
        self.cache
            .prefetch_neighbors(catalog, asset_indices, self.playback_rate);
    }

    pub fn set_zone_sound(&self, sound: Option<ZoneSoundRecord>, scene_index: i32) {
        let _ = self
            .cmd_tx
            .try_send(AudioCommand::SetZoneSound { sound, scene_index });
    }

    pub fn set_dwell_progress(&self, progress: f32) {
        let _ = self.cmd_tx.try_send(AudioCommand::SetDwellProgress(progress));
    }

    pub fn music_note_on(&self, id: u64, midi: u8, timbre_idx: usize, pan: f32) {
        let _ = self.cmd_tx.try_send(AudioCommand::MusicNoteOn {
            id,
            midi,
            timbre_idx,
            pan,
        });
    }

    pub fn music_note_update(&self, id: u64, midi: u8, timbre_idx: usize, pan: f32) {
        let _ = self.cmd_tx.try_send(AudioCommand::MusicNoteUpdate {
            id,
            midi,
            timbre_idx,
            pan,
        });
    }

    pub fn music_note_off(&self, id: u64) {
        let _ = self.cmd_tx.try_send(AudioCommand::MusicNoteOff { id });
    }

    /// Reads the latest lock-free `AudioReactiveUniforms` for GPU upload.
    pub fn reactive_uniforms(&self) -> AudioReactiveUniforms {
        self.bus.load()
    }

    /// Reads raw atomic playback clock without requiring a `UniverseCatalog`.
    pub fn playback_clock(&self) -> PlaybackClockSnapshot {
        self.clock.load()
    }

    /// Pairs current `PlaybackClockSnapshot` with `UniverseCatalog::cue_progress_at_ms` for live-ink text.
    pub fn poll_sync_clock<'a>(
        &self,
        catalog: &'a UniverseCatalog,
    ) -> Option<SyncVoiceClockState<'a>> {
        let snap = self.clock.load();
        let asset_idx = snap.asset_idx?;
        let asset = catalog.audio_asset(asset_idx)?;
        let cue_progress = catalog.cue_progress_at_ms(asset, snap.source_ms);
        Some(SyncVoiceClockState {
            asset_idx,
            source_ms: snap.source_ms,
            duration_ms: asset.duration_ms,
            cue_index: cue_progress.active_cue_index,
            cue_progress,
            is_speaking: snap.is_speaking,
            is_finished: snap.is_finished,
        })
    }

    /// Advances headless software mixer manually (no-op when hardware `cpal` stream owns mixer).
    pub fn render_headless_frames(&mut self, out_interleaved: &mut [f32]) -> bool {
        if let Some(ref mut mixer) = self.headless_mixer {
            mixer.render_frames_interleaved(out_interleaved);
            true
        } else {
            false
        }
    }
}

fn try_open_device_stream(
    build_spatial: &impl Fn() -> Result<SpatialNarrationRenderer, AudioError>,
    cmd_rx: Receiver<AudioCommand>,
    bus: Arc<AtomicAudioReactiveBus>,
    clock: Arc<AtomicPlaybackClock>,
) -> Option<(Stream, String, bool)> {
    let host = cpal::default_host();
    let device = host.default_output_device()?;
    let device_name = device
        .description()
        .map(|d| d.to_string())
        .unwrap_or_else(|_| "Default Audio Output".to_owned());

    let _default_cfg = device.default_output_config().ok()?;
    let use_cfg = StreamConfig {
        channels: 2,
        sample_rate: OUTPUT_SAMPLE_RATE,
        buffer_size: cpal::BufferSize::Default,
    };

    let spatial = build_spatial().ok()?;
    let uses_hrtf = spatial.uses_hrtf();
    let mut mixer = AudioMixer::new(cmd_rx, bus, clock, spatial);

    let stream = device
        .build_output_stream(
            use_cfg,
            move |data: &mut [f32], _| {
                mixer.render_frames_interleaved(data);
            },
            |_err| {},
            None,
        )
        .ok()?;
    stream.play().ok()?;
    Some((stream, device_name, uses_hrtf))
}
