use std::path::Path;

use glam::Vec3;
use hrtf::{HrirSphere, HrtfContext, HrtfProcessor, Vec3 as HrtfVec3};

use crate::decoder::OUTPUT_SAMPLE_RATE;
use crate::error::AudioError;

/// Audio mixing quantum matching `HrtfProcessor` fixed block length (`~10.67 ms` @ 48 kHz).
pub const AUDIO_BLOCK_FRAMES: usize = 512;

/// Mixing blend: `70%` HRTF binaural directionality + `30%` head-locked speech clarity center.
pub const SPATIAL_HRTF_BLEND: f32 = 0.70;
pub const HEAD_LOCKED_CLARITY_BLEND: f32 = 0.30;

/// Binaural narrator renderer using IRCAM HRIR sphere + 30% head-locked clarity center.
pub struct SpatialNarrationRenderer {
    processor: Option<HrtfProcessor>,
    history: Vec<f32>,
    hrtf_out: Vec<(f32, f32)>,
    prev_dir: Vec3,
    prev_gain: f32,
}

impl SpatialNarrationRenderer {
    /// Creates an HRTF renderer from an on-disk IRCAM `.bin` HRIR sphere (e.g., `assets/hrir/irc_1040_c.bin`).
    pub fn from_hrir_file(path: impl AsRef<Path>) -> Result<Self, AudioError> {
        let path_ref = path.as_ref();
        let sphere = HrirSphere::from_file(path_ref, OUTPUT_SAMPLE_RATE).map_err(|e| {
            AudioError::HrirLoad {
                path: path_ref.display().to_string(),
                reason: format!("{e:?}"),
            }
        })?;
        Ok(Self::from_sphere(sphere))
    }

    /// Creates an HRTF renderer from raw IRCAM HRIR `.bin` bytes.
    pub fn from_hrir_bytes(bytes: &[u8]) -> Result<Self, AudioError> {
        let sphere = HrirSphere::new(std::io::Cursor::new(bytes), OUTPUT_SAMPLE_RATE).map_err(|e| {
            AudioError::HrirLoad {
                path: "<memory>".to_owned(),
                reason: format!("{e:?}"),
            }
        })?;
        Ok(Self::from_sphere(sphere))
    }

    /// Creates a fallback equal-power stereo panner without an HRIR file (useful in minimal unit tests).
    pub fn stereo_fallback() -> Self {
        Self {
            processor: None,
            history: Vec::new(),
            hrtf_out: vec![(0.0, 0.0); AUDIO_BLOCK_FRAMES],
            prev_dir: Vec3::NEG_Z,
            prev_gain: 0.0,
        }
    }

    fn from_sphere(sphere: HrirSphere) -> Self {
        let hrir_len = sphere.len().saturating_sub(1);
        let mut processor = HrtfProcessor::new(sphere, AUDIO_BLOCK_FRAMES);
        let mut history = vec![0.0_f32; hrir_len * 2];
        let mut hrtf_out = vec![(0.0_f32, 0.0_f32); AUDIO_BLOCK_FRAMES];

        // Warm once at init so `HrtfProcessor` internal buffers never allocate on the audio thread.
        let zeros = [0.0_f32; AUDIO_BLOCK_FRAMES];
        processor.process_samples(HrtfContext {
            source: &zeros,
            output: &mut hrtf_out,
            new_sample_vector: HrtfVec3::new(0.0, 0.0, 1.0),
            prev_sample_vector: HrtfVec3::new(0.0, 0.0, 1.0),
            new_gain: 1.0,
            prev_gain: 1.0,
            history: &mut history,
        });
        hrtf_out.fill((0.0, 0.0));

        Self {
            processor: Some(processor),
            history,
            hrtf_out,
            prev_dir: Vec3::NEG_Z,
            prev_gain: 1.0,
        }
    }

    pub const fn uses_hrtf(&self) -> bool {
        self.processor.is_some()
    }

    /// Renders `AUDIO_BLOCK_FRAMES` mono narrator samples at camera-relative `emitter_dir_cam`
    /// into interleaved `(L, R)` output, blending 70% binaural HRTF with 30% center clarity.
    pub fn mix_block(
        &mut self,
        mono_in: &[f32; AUDIO_BLOCK_FRAMES],
        emitter_dir_cam: Vec3,
        gain: f32,
        out_stereo_interleaved: &mut [f32; AUDIO_BLOCK_FRAMES * 2],
    ) {
        let dir = emitter_dir_cam.try_normalize().unwrap_or(Vec3::NEG_Z);

        if let Some(ref mut proc) = self.processor {
            self.hrtf_out.fill((0.0, 0.0));
            // `hrtf` crate uses listener-to-source vector conventions in its IR sphere:
            // map camera-space (+X right, +Y up, -Z forward) into HRIR coordinates cleanly.
            let prev_v = to_hrtf_vec(self.prev_dir);
            let new_v = to_hrtf_vec(dir);
            proc.process_samples(HrtfContext {
                source: mono_in.as_slice(),
                output: &mut self.hrtf_out,
                new_sample_vector: new_v,
                prev_sample_vector: prev_v,
                new_gain: gain,
                prev_gain: self.prev_gain,
                history: &mut self.history,
            });

            for (i, ((hl, hr), &mono)) in self
                .hrtf_out
                .iter()
                .zip(mono_in.iter())
                .enumerate()
            {
                let dry_center = mono * gain * HEAD_LOCKED_CLARITY_BLEND;
                out_stereo_interleaved[i * 2] += hl * SPATIAL_HRTF_BLEND + dry_center;
                out_stereo_interleaved[i * 2 + 1] += hr * SPATIAL_HRTF_BLEND + dry_center;
            }
        } else {
            // Smooth equal-power stereo pan fallback when constructed without an HRIR sphere.
            let pan = dir.x.clamp(-1.0, 1.0);
            let angle = (pan + 1.0) * std::f32::consts::FRAC_PI_4;
            let pan_l = angle.cos();
            let pan_r = angle.sin();
            for (i, &mono) in mono_in.iter().enumerate() {
                let s = mono * gain;
                let center = s * HEAD_LOCKED_CLARITY_BLEND;
                out_stereo_interleaved[i * 2] += pan_l * s * SPATIAL_HRTF_BLEND + center;
                out_stereo_interleaved[i * 2 + 1] += pan_r * s * SPATIAL_HRTF_BLEND + center;
            }
        }

        self.prev_dir = dir;
        self.prev_gain = gain;
    }
}

/// Converts OpenXR camera-relative direction (`-X` left, `+X` right, `+Y` up, `-Z` forward) into
/// `hrtf::Vec3` matching IRCAM HRIR sphere orientation (`-X` = left ear, `+X` = right ear, `-Z` = front).
#[inline]
fn to_hrtf_vec(dir_cam: Vec3) -> HrtfVec3 {
    HrtfVec3::new(dir_cam.x, dir_cam.y, dir_cam.z)
}
