use std::sync::atomic::{AtomicU32, Ordering};

use bytemuck::{Pod, Zeroable};

/// Real-time 64-byte audio-reactive uniform block matching `shaders/SHADER_CONTRACTS.md`
/// and `shaders/common_oled.wgsl` (`AudioReactiveUniforms`, `@group(0) @binding(1)`).
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Pod, Zeroable)]
pub struct AudioReactiveUniforms {
    /// `[voice_rms, ambient_rms, bass_energy, mid_energy]`
    pub levels_a: [f32; 4],
    /// `[harmonic_centroid (0..1), beat_phase (0..1), high_energy, playback_rate (1..3)]`
    pub levels_b: [f32; 4],
    /// `[drone_base_hz, synth_filter_hz, synth_pulse, drone_noise]`
    pub synth_a: [f32; 4],
    /// `[active_note_hue (0..1), active_timbre_id (0..3), music_energy, resonance_glow]`
    pub synth_b: [f32; 4],
}

const _: () = {
    assert!(std::mem::size_of::<AudioReactiveUniforms>() == 64);
    assert!(std::mem::align_of::<AudioReactiveUniforms>() == 4);
};

/// Lock-free 16-lane atomic bridge publishing `AudioReactiveUniforms` from the audio thread
/// to the GPU render thread with zero mutex contention or allocations.
#[derive(Debug)]
pub struct AtomicAudioReactiveBus {
    lanes: [AtomicU32; 16],
}

impl Default for AtomicAudioReactiveBus {
    fn default() -> Self {
        Self::new()
    }
}

impl AtomicAudioReactiveBus {
    pub fn new() -> Self {
        Self {
            lanes: std::array::from_fn(|_| AtomicU32::new(0)),
        }
    }

    pub fn store(&self, u: AudioReactiveUniforms) {
        let words: [u32; 16] = bytemuck::cast(u);
        for (dst, &w) in self.lanes.iter().zip(words.iter()) {
            dst.store(w, Ordering::Relaxed);
        }
    }

    pub fn load(&self) -> AudioReactiveUniforms {
        let mut words = [0_u32; 16];
        for (dst, src) in words.iter_mut().zip(self.lanes.iter()) {
            *dst = src.load(Ordering::Relaxed);
        }
        bytemuck::cast(words)
    }
}

/// Zero-allocation 3-band spectral & RMS tracker evaluated on every 48 kHz output block.
#[derive(Debug, Clone)]
pub struct OutputAnalyzer {
    lp_bass_state: f32,
    lp_mid_state: f32,
    prev_mono: f32,
    smooth_bass: f32,
    smooth_mid: f32,
    smooth_high: f32,
    smooth_centroid: f32,
    bass_alpha: f32,
    mid_alpha: f32,
}

impl OutputAnalyzer {
    pub fn new(sample_rate: f32) -> Self {
        let two_pi_dt = std::f32::consts::TAU / sample_rate.max(1.0);
        let lp_coef = |fc: f32| {
            let x = fc * two_pi_dt;
            x / (1.0 + x)
        };
        Self {
            lp_bass_state: 0.0,
            lp_mid_state: 0.0,
            prev_mono: 0.0,
            smooth_bass: 0.0,
            smooth_mid: 0.0,
            smooth_high: 0.0,
            smooth_centroid: 0.25,
            bass_alpha: lp_coef(220.0),
            mid_alpha: lp_coef(2400.0),
        }
    }

    /// Processes stereo interleaved samples `(L, R)` and returns `(bass, mid, high, centroid)`.
    pub fn analyze_stereo_block(&mut self, stereo_interleaved: &[f32]) -> (f32, f32, f32, f32) {
        let frames = stereo_interleaved.len() / 2;
        if frames == 0 {
            return (
                self.smooth_bass,
                self.smooth_mid,
                self.smooth_high,
                self.smooth_centroid,
            );
        }

        let mut e_bass = 0.0_f32;
        let mut e_mid = 0.0_f32;
        let mut e_high = 0.0_f32;
        let mut zero_crossings = 0_u32;

        for frame in stereo_interleaved.as_chunks::<2>().0 {
            let mono = 0.5 * (frame[0] + frame[1]);
            self.lp_bass_state += self.bass_alpha * (mono - self.lp_bass_state);
            self.lp_mid_state += self.mid_alpha * (mono - self.lp_mid_state);

            let bass = self.lp_bass_state;
            let mid = self.lp_mid_state - bass;
            let high = mono - self.lp_mid_state;

            e_bass += bass * bass;
            e_mid += mid * mid;
            e_high += high * high;

            if (mono >= 0.0) != (self.prev_mono >= 0.0) {
                zero_crossings += 1;
            }
            self.prev_mono = mono;
        }

        let inv_n = 1.0 / frames as f32;
        let rms_bass = (e_bass * inv_n).sqrt();
        let rms_mid = (e_mid * inv_n).sqrt();
        let rms_high = (e_high * inv_n).sqrt();

        // Zero-crossing ratio normalized cleanly into [0, 1] as spectral brightness proxy.
        let zcr_centroid = (zero_crossings as f32 * inv_n * 3.5).clamp(0.0, 1.0);
        let energy_sum = (rms_bass + rms_mid + rms_high).max(1e-6);
        let weighted_centroid = ((0.15 * rms_bass + 0.50 * rms_mid + 0.92 * rms_high) / energy_sum)
            .clamp(0.0, 1.0);
        let centroid = (0.65 * weighted_centroid + 0.35 * zcr_centroid).clamp(0.0, 1.0);

        let attack = 0.38;
        self.smooth_bass += attack * (rms_bass - self.smooth_bass);
        self.smooth_mid += attack * (rms_mid - self.smooth_mid);
        self.smooth_high += attack * (rms_high - self.smooth_high);
        self.smooth_centroid += 0.25 * (centroid - self.smooth_centroid);

        (
            self.smooth_bass,
            self.smooth_mid,
            self.smooth_high,
            self.smooth_centroid,
        )
    }
}
