use cosmos_manifest::{Waveform, ZoneSoundRecord};
use fundsp::prelude::{
    lowpass_q, pass, pink, saw, shared, sine, square, triangle, var, AudioUnit, Shared,
};

use crate::decoder::OUTPUT_SAMPLE_RATE;

/// Crossfade smoothing rate and `-9 dB` speech ducking linear factor.
const DUCKING_LINEAR_GAIN: f32 = 0.354_813_4; // 10^(-9 / 20)
const DUCK_SMOOTH_PER_SAMPLE: f32 = 0.00055; // ~38 ms time constant @ 48 kHz
const ZONE_XFADE_STEP: f32 = 1.0 / (0.25 * OUTPUT_SAMPLE_RATE as f32);

const MUSIC_TIMBRE_PARAMS: [(Waveform, f32, f32, f32, f32); 4] = [
    (Waveform::Sine, 7.0, 5200.0, 0.015, 0.62),      // cristal
    (Waveform::Triangle, 5.0, 3400.0, 0.025, 0.46),  // aire
    (Waveform::Sawtooth, 3.0, 1900.0, 0.035, 0.38),  // cálido
    (Waveform::Square, 2.0, 980.0, 0.045, 0.50),     // profundo
];

/// Dynamic snapshot from procedural synthesis fed into `AudioReactiveUniforms`.
#[derive(Copy, Clone, Debug, Default)]
pub struct SynthTelemetry {
    pub ambient_rms: f32,
    pub beat_phase: f32,
    pub drone_base_hz: f32,
    pub synth_filter_hz: f32,
    pub synth_pulse: f32,
    pub drone_noise: f32,
    pub active_note_hue: f32,
    pub active_timbre_id: f32,
    pub music_energy: f32,
    pub resonance_glow: f32,
}

struct AmbientVoiceSlot {
    freq: Shared,
    cutoff: Shared,
    w_sine: Shared,
    w_tri: Shared,
    w_saw: Shared,
    w_sq: Shared,
    w_sub: Shared,
    w_noise: Shared,
    unit: Box<dyn AudioUnit>,
    pulse_hz: f32,
    pulse_depth: f32,
    base_hz: f32,
    filter_hz: f32,
    noise_level: f32,
    target_gain: f32,
    phase: f32,
}

impl AmbientVoiceSlot {
    fn new() -> Self {
        let freq = shared(110.0);
        let cutoff = shared(1200.0);
        let w_sine = shared(1.0);
        let w_tri = shared(0.0);
        let w_saw = shared(0.0);
        let w_sq = shared(0.0);
        let w_sub = shared(0.35);
        let w_noise = shared(0.15);

        let graph = (((var(&freq) >> sine::<f32>()) * var(&w_sine)
            + (var(&freq) >> triangle()) * var(&w_tri)
            + (var(&freq) >> saw()) * var(&w_saw)
            + (var(&freq) >> square()) * var(&w_sq)
            + ((var(&freq) * 0.5) >> sine::<f32>()) * var(&w_sub)
            + (pink::<f32>() * 0.25) * var(&w_noise))
            | var(&cutoff))
            >> lowpass_q::<f32>(0.85);

        let mut unit: Box<dyn AudioUnit> = Box::new(graph);
        unit.set_sample_rate(OUTPUT_SAMPLE_RATE as f64);
        unit.allocate();

        Self {
            freq,
            cutoff,
            w_sine,
            w_tri,
            w_saw,
            w_sq,
            w_sub,
            w_noise,
            unit,
            pulse_hz: 0.4,
            pulse_depth: 0.12,
            base_hz: 110.0,
            filter_hz: 1200.0,
            noise_level: 0.15,
            target_gain: 0.0,
            phase: 0.0,
        }
    }

    fn apply_sound(&mut self, sound: Option<ZoneSoundRecord>, scene_index: i32) {
        let Some(snd) = sound else {
            self.target_gain = 0.0;
            return;
        };
        self.base_hz = snd.base_hz.clamp(28.0, 2000.0);
        // Slightly warm sub-octave filter modulation driven by scene progression when present.
        let scene_detune = if scene_index >= 0 {
            1.0 + ((scene_index % 5) as f32) * 0.012
        } else {
            1.0
        };
        self.filter_hz = (snd.filter_hz * scene_detune).clamp(120.0, 12_000.0);
        self.noise_level = snd.noise.clamp(0.0, 1.0);
        self.pulse_depth = snd.pulse.clamp(0.0, 1.0);
        self.pulse_hz = (0.25 + snd.pulse.clamp(0.0, 1.0) * 3.2).clamp(0.15, 6.0);

        self.freq.set_value(self.base_hz);
        self.cutoff.set_value(self.filter_hz);
        self.w_noise.set_value(self.noise_level * 0.55);
        self.w_sub.set_value(0.32);

        let wf = snd.waveform_enum().unwrap_or(Waveform::Sine);
        self.w_sine
            .set_value(if wf == Waveform::Sine { 0.85 } else { 0.0 });
        self.w_tri
            .set_value(if wf == Waveform::Triangle { 0.85 } else { 0.0 });
        self.w_saw
            .set_value(if wf == Waveform::Sawtooth { 0.45 } else { 0.0 });
        self.w_sq
            .set_value(if wf == Waveform::Square { 0.40 } else { 0.0 });
        self.target_gain = 0.18;
    }

    fn tick_sample(&mut self, sr_inv: f32) -> f32 {
        let mut out = [0.0_f32; 1];
        self.unit.tick(&[], &mut out);
        self.phase = (self.phase + self.pulse_hz * sr_inv).fract();
        let lfo = 1.0 - 0.5 * self.pulse_depth * (1.0 - (self.phase * std::f32::consts::TAU).sin());
        out[0] * lfo * self.target_gain
    }
}

/// Dual-slot A/B crossfading zone ambient drone synthesizer (`fundsp 0.23.0`) with `-9 dB` ducking.
pub struct ZoneAmbientSynth {
    slots: [AmbientVoiceSlot; 2],
    active_slot: usize,
    xfade: f32, // 0.0 = slot 0 full, 1.0 = slot 1 full
    duck_gain: f32,
    current_sound: Option<ZoneSoundRecord>,
    scene_index: i32,
}

impl Default for ZoneAmbientSynth {
    fn default() -> Self {
        Self::new()
    }
}

impl ZoneAmbientSynth {
    pub fn new() -> Self {
        Self {
            slots: [AmbientVoiceSlot::new(), AmbientVoiceSlot::new()],
            active_slot: 0,
            xfade: 0.0,
            duck_gain: 1.0,
            current_sound: None,
            scene_index: -1,
        }
    }

    pub fn set_zone(&mut self, sound: Option<ZoneSoundRecord>, scene_index: i32) {
        if self.current_sound == sound && self.scene_index == scene_index {
            return;
        }
        self.current_sound = sound;
        self.scene_index = scene_index;
        let next = 1 - self.active_slot;
        self.slots[next].apply_sound(sound, scene_index);
        self.active_slot = next;
    }

    /// Renders `frames` stereo samples additively into `out_stereo`, returning telemetry metrics.
    pub fn mix_block(
        &mut self,
        frames: usize,
        narration_active: bool,
        out_stereo: &mut [f32],
    ) -> (f32, f32, f32, f32, f32, f32) {
        let sr_inv = 1.0 / OUTPUT_SAMPLE_RATE as f32;
        let target_duck = if narration_active {
            DUCKING_LINEAR_GAIN
        } else {
            1.0
        };
        let target_xfade = if self.active_slot == 0 { 0.0 } else { 1.0 };
        let mut sum_sq = 0.0_f32;

        for frame_idx in 0..frames {
            self.duck_gain += DUCK_SMOOTH_PER_SAMPLE * (target_duck - self.duck_gain);
            if (self.xfade - target_xfade).abs() > ZONE_XFADE_STEP {
                self.xfade += if target_xfade > self.xfade {
                    ZONE_XFADE_STEP
                } else {
                    -ZONE_XFADE_STEP
                };
            } else {
                self.xfade = target_xfade;
            }

            let s0 = self.slots[0].tick_sample(sr_inv);
            let s1 = self.slots[1].tick_sample(sr_inv);
            let g0 = (1.0 - self.xfade) * self.duck_gain;
            let g1 = self.xfade * self.duck_gain;
            let mono = s0 * g0 + s1 * g1;

            // Subtle stereo Haas spread around head center.
            let l = mono * 0.96 + s1 * 0.05 * self.duck_gain;
            let r = mono * 0.96 - s0 * 0.05 * self.duck_gain;
            out_stereo[frame_idx * 2] += l;
            out_stereo[frame_idx * 2 + 1] += r;
            sum_sq += 0.5 * (l * l + r * r);
        }

        let rms = (sum_sq / frames.max(1) as f32).sqrt();
        let active = &self.slots[self.active_slot];
        (
            rms,
            active.phase,
            active.base_hz,
            active.filter_hz,
            active.pulse_depth,
            active.noise_level,
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum EnvStage {
    Idle,
    Attack,
    Sustain,
    Release,
}

struct MusicVoiceSlot {
    id: u64,
    midi: u8,
    timbre_idx: usize,
    pan: f32,
    age: u64,
    stage: EnvStage,
    env: f32,
    attack_step: f32,
    release_coef: f32,
    freq_lo: Shared,
    freq_hi: Shared,
    cutoff: Shared,
    w_sine: Shared,
    w_tri: Shared,
    w_saw: Shared,
    w_sq: Shared,
    unit: Box<dyn AudioUnit>,
}

impl MusicVoiceSlot {
    fn new() -> Self {
        let freq_lo = shared(440.0);
        let freq_hi = shared(440.0);
        let cutoff = shared(5200.0);
        let w_sine = shared(1.0);
        let w_tri = shared(0.0);
        let w_saw = shared(0.0);
        let w_sq = shared(0.0);

        let pair_sine = ((var(&freq_lo) >> sine::<f32>()) + (var(&freq_hi) >> sine::<f32>())) * 0.5;
        let pair_tri = ((var(&freq_lo) >> triangle()) + (var(&freq_hi) >> triangle())) * 0.5;
        let pair_saw = ((var(&freq_lo) >> saw()) + (var(&freq_hi) >> saw())) * 0.5;
        let pair_sq = ((var(&freq_lo) >> square()) + (var(&freq_hi) >> square())) * 0.5;

        let mix = pair_sine * var(&w_sine)
            + pair_tri * var(&w_tri)
            + pair_saw * var(&w_saw)
            + pair_sq * var(&w_sq);

        let graph = (mix | var(&cutoff)) >> lowpass_q::<f32>(0.75);
        let mut unit: Box<dyn AudioUnit> = Box::new(graph);
        unit.set_sample_rate(OUTPUT_SAMPLE_RATE as f64);
        unit.allocate();

        Self {
            id: 0,
            midi: 69,
            timbre_idx: 0,
            pan: 0.0,
            age: 0,
            stage: EnvStage::Idle,
            env: 0.0,
            attack_step: 1.0,
            release_coef: 0.999,
            freq_lo,
            freq_hi,
            cutoff,
            w_sine,
            w_tri,
            w_saw,
            w_sq,
            unit,
        }
    }

    fn configure(&mut self, midi: u8, timbre_idx: usize, pan: f32) {
        let t_idx = timbre_idx.min(3);
        let (wf, detune_cents, filter_hz, atk_sec, rel_sec) = MUSIC_TIMBRE_PARAMS[t_idx];
        let hz = 440.0 * 2.0_f32.powf((midi as f32 - 69.0) / 12.0);
        let detune_ratio = 2.0_f32.powf(detune_cents / 1200.0);

        self.midi = midi;
        self.timbre_idx = t_idx;
        self.pan = pan.clamp(-1.0, 1.0);
        self.freq_lo.set_value(hz / detune_ratio);
        self.freq_hi.set_value(hz * detune_ratio);
        self.cutoff.set_value(filter_hz);

        self.w_sine
            .set_value(if wf == Waveform::Sine { 1.0 } else { 0.0 });
        self.w_tri
            .set_value(if wf == Waveform::Triangle { 1.0 } else { 0.0 });
        self.w_saw
            .set_value(if wf == Waveform::Sawtooth { 0.55 } else { 0.0 });
        self.w_sq
            .set_value(if wf == Waveform::Square { 0.45 } else { 0.0 });

        self.attack_step = 1.0 / (atk_sec.max(0.002) * OUTPUT_SAMPLE_RATE as f32);
        // Matches WebAudio setTargetAtTime decay `tau = release / 4`.
        let tau_samples = (rel_sec.max(0.02) * 0.25) * OUTPUT_SAMPLE_RATE as f32;
        self.release_coef = (-1.0 / tau_samples).exp();
    }

    fn note_on(&mut self, id: u64, age: u64, midi: u8, timbre_idx: usize, pan: f32) {
        self.id = id;
        self.age = age;
        self.configure(midi, timbre_idx, pan);
        self.stage = EnvStage::Attack;
    }

    fn tick_stereo(&mut self) -> (f32, f32) {
        match self.stage {
            EnvStage::Idle => return (0.0, 0.0),
            EnvStage::Attack => {
                self.env += self.attack_step;
                if self.env >= 1.0 {
                    self.env = 1.0;
                    self.stage = EnvStage::Sustain;
                }
            }
            EnvStage::Sustain => {}
            EnvStage::Release => {
                self.env *= self.release_coef;
                if self.env <= 0.0002 {
                    self.env = 0.0;
                    self.stage = EnvStage::Idle;
                    return (0.0, 0.0);
                }
            }
        }

        let mut out = [0.0_f32; 1];
        self.unit.tick(&[], &mut out);
        let sample = out[0] * self.env * 0.22;
        let angle = (self.pan + 1.0) * std::f32::consts::FRAC_PI_4;
        (sample * angle.cos(), sample * angle.sin())
    }
}

/// 16-voice polyphonic Experience `02` sanctuary synth (`fundsp 0.23.0`) + `290 ms` stereo echo.
pub struct MusicPolySynth {
    voices: [MusicVoiceSlot; 16],
    delay_l: Vec<f32>,
    delay_r: Vec<f32>,
    delay_pos: usize,
    age_counter: u64,
    last_hue: f32,
    last_timbre: f32,
}

impl Default for MusicPolySynth {
    fn default() -> Self {
        Self::new()
    }
}

impl MusicPolySynth {
    pub fn new() -> Self {
        let delay_len = (0.29 * OUTPUT_SAMPLE_RATE as f32).round() as usize;
        Self {
            voices: std::array::from_fn(|_| MusicVoiceSlot::new()),
            delay_l: vec![0.0_f32; delay_len],
            delay_r: vec![0.0_f32; delay_len],
            delay_pos: 0,
            age_counter: 0,
            last_hue: 0.0,
            last_timbre: 0.0,
        }
    }

    pub fn note_on(&mut self, id: u64, midi: u8, timbre_idx: usize, pan: f32) {
        self.age_counter += 1;
        let slot_idx = self
            .voices
            .iter()
            .position(|v| v.id == id && v.stage != EnvStage::Idle)
            .or_else(|| self.voices.iter().position(|v| v.stage == EnvStage::Idle))
            .or_else(|| {
                self.voices
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| v.stage == EnvStage::Release)
                    .min_by_key(|(_, v)| v.age)
                    .map(|(i, _)| i)
            })
            .unwrap_or_else(|| {
                self.voices
                    .iter()
                    .enumerate()
                    .min_by_key(|(_, v)| v.age)
                    .map(|(i, _)| i)
                    .unwrap_or(0)
            });

        self.voices[slot_idx].note_on(id, self.age_counter, midi, timbre_idx, pan);
        self.last_hue = (midi % 12) as f32 / 12.0;
        self.last_timbre = timbre_idx.min(3) as f32;
    }

    pub fn note_update(&mut self, id: u64, midi: u8, timbre_idx: usize, pan: f32) {
        if let Some(v) = self
            .voices
            .iter_mut()
            .find(|v| v.id == id && v.stage != EnvStage::Idle)
        {
            v.configure(midi, timbre_idx, pan);
            self.last_hue = (midi % 12) as f32 / 12.0;
            self.last_timbre = timbre_idx.min(3) as f32;
        }
    }

    pub fn note_off(&mut self, id: u64) {
        for v in &mut self.voices {
            if v.id == id && v.stage != EnvStage::Idle {
                v.stage = EnvStage::Release;
            }
        }
    }

    pub fn active_voice_count(&self) -> usize {
        self.voices
            .iter()
            .filter(|v| v.stage != EnvStage::Idle)
            .count()
    }

    /// Mixes all active voices + 290ms stereo ping-pong echo into `out_stereo`, returning
    /// `(music_energy, active_note_hue, active_timbre_id)`.
    pub fn mix_block(&mut self, frames: usize, out_stereo: &mut [f32]) -> (f32, f32, f32) {
        let mut sum_sq = 0.0_f32;
        let delay_len = self.delay_l.len();

        for i in 0..frames {
            let mut dry_l = 0.0_f32;
            let mut dry_r = 0.0_f32;
            for v in &mut self.voices {
                if v.stage != EnvStage::Idle {
                    let (vl, vr) = v.tick_stereo();
                    dry_l += vl;
                    dry_r += vr;
                }
            }

            let d_l = self.delay_l[self.delay_pos];
            let d_r = self.delay_r[self.delay_pos];
            // Matches 02/scene.js:58-65 (`delayTime=0.29`, `delayGain=0.18`, `feedback=0.26`).
            self.delay_l[self.delay_pos] = dry_l + d_r * 0.26;
            self.delay_r[self.delay_pos] = dry_r + d_l * 0.26;
            self.delay_pos = (self.delay_pos + 1) % delay_len;

            let out_l = (dry_l + d_l * 0.18).tanh();
            let out_r = (dry_r + d_r * 0.18).tanh();
            out_stereo[i * 2] += out_l;
            out_stereo[i * 2 + 1] += out_r;
            sum_sq += 0.5 * (out_l * out_l + out_r * out_r);
        }

        let rms = (sum_sq / frames.max(1) as f32).sqrt();
        (rms, self.last_hue, self.last_timbre)
    }
}

/// Crystalline dwell-shimmer (200..900ms rest lock) and Wrist Resonance Ring speed-bead tick chime.
pub struct FeedbackChimeSynth {
    dwell_progress: f32,
    shimmer_phase: f32,
    lock_env: f32,
    lock_phase: f32,
    tick_env: f32,
    tick_phase: f32,
    tick_hz: f32,
}

impl Default for FeedbackChimeSynth {
    fn default() -> Self {
        Self::new()
    }
}

impl FeedbackChimeSynth {
    pub fn new() -> Self {
        Self {
            dwell_progress: 0.0,
            shimmer_phase: 0.0,
            lock_env: 0.0,
            lock_phase: 0.0,
            tick_env: 0.0,
            tick_phase: 0.0,
            tick_hz: 1180.0,
        }
    }

    pub fn set_dwell_progress(&mut self, progress: f32) {
        let clamped = progress.clamp(0.0, 1.0);
        if self.dwell_progress < 0.99 && clamped >= 0.99 {
            // Subtle crystalline harmonic bloom triggered once when dwell crosses 0.99 lock threshold.
            self.lock_env = 1.0;
        }
        self.dwell_progress = clamped;
    }

    pub fn trigger_speed_tick(&mut self, bead_index: u8) {
        // Ascending glassy pentatonic pitch across speed beads 0..8.
        const BEAD_HZ: [f32; 9] = [
            880.0, 987.77, 1108.73, 1318.51, 1479.98, 1760.0, 1975.53, 2217.46, 2637.02,
        ];
        self.tick_hz = BEAD_HZ[(bead_index as usize).min(BEAD_HZ.len() - 1)];
        self.tick_env = 1.0;
    }

    /// Mixes subtle diegetic chimes into `out_stereo`, returning `resonance_glow` `[0..1]`.
    pub fn mix_block(&mut self, frames: usize, out_stereo: &mut [f32]) -> f32 {
        let sr_inv = 1.0 / OUTPUT_SAMPLE_RATE as f32;
        let shimmer_amp = if self.dwell_progress > 0.22 {
            ((self.dwell_progress - 0.22) / 0.78).clamp(0.0, 1.0) * 0.032
        } else {
            0.0
        };
        let shimmer_hz = 528.0 + self.dwell_progress * 396.0;
        let lock_decay = (-1.0 / (0.14 * OUTPUT_SAMPLE_RATE as f32)).exp();
        let tick_decay = (-1.0 / (0.045 * OUTPUT_SAMPLE_RATE as f32)).exp();

        for i in 0..frames {
            let mut sample = 0.0_f32;
            if shimmer_amp > 0.0001 {
                self.shimmer_phase = (self.shimmer_phase + shimmer_hz * sr_inv).fract();
                let s1 = (self.shimmer_phase * std::f32::consts::TAU).sin();
                let s2 = (self.shimmer_phase * 2.0 * std::f32::consts::TAU).sin() * 0.35;
                sample += (s1 + s2) * shimmer_amp;
            }
            if self.lock_env > 0.0005 {
                self.lock_phase = (self.lock_phase + 1318.5 * sr_inv).fract();
                sample += (self.lock_phase * std::f32::consts::TAU).sin() * self.lock_env * 0.065;
                self.lock_env *= lock_decay;
            }
            if self.tick_env > 0.0005 {
                self.tick_phase = (self.tick_phase + self.tick_hz * sr_inv).fract();
                sample += (self.tick_phase * std::f32::consts::TAU).sin() * self.tick_env * 0.07;
                self.tick_env *= tick_decay;
            }

            out_stereo[i * 2] += sample;
            out_stereo[i * 2 + 1] += sample * 0.98;
        }

        (self.dwell_progress.max(self.tick_env * 0.8 + self.lock_env)).clamp(0.0, 1.0)
    }
}

/// Placeholder helper proving `fundsp::pass` linkage for future DSP inserts.
#[allow(dead_code)]
fn _pass_smoke() {
    let _p = pass();
}
