use std::collections::{HashMap, VecDeque};
use std::io::Cursor;
use std::num::{NonZeroU16, NonZeroU32};
use std::sync::{Arc, Mutex};

use cosmos_manifest::UniverseCatalog;
use rubato::audioadapter_buffers::direct::InterleavedSlice;
use rubato::{
    Async, FixedAsync, Indexing, Resampler, SincInterpolationParameters, SincInterpolationType,
    WindowFunction,
};
use symphonia::core::audio::sample::Sample;
use symphonia::core::codecs::audio::AudioDecoderOptions;
use symphonia::core::errors::Error as SymphoniaError;
use symphonia::core::formats::probe::Hint;
use symphonia::core::formats::{FormatOptions, TrackType};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;

use crate::error::AudioError;

/// Canonical SyncVoice playback speed beads (`1.0x` to `3.0x` in `0.25x` increments).
pub const SUPPORTED_PLAYBACK_RATES: [f32; 9] =
    [1.0, 1.25, 1.5, 1.75, 2.0, 2.25, 2.5, 2.75, 3.0];

/// Output sample rate used across WSOLA->Rubato->HRTF->CPAL mixing.
pub const OUTPUT_SAMPLE_RATE: u32 = 48_000;

/// Snaps any arbitrary rate into the supported `[1.0, 3.0]` range (`0.25x` bead if desired).
pub fn clamp_playback_rate(rate: f32) -> f32 {
    if !rate.is_finite() {
        return 1.0;
    }
    rate.clamp(1.0, 3.0)
}

/// Snaps `rate` to the nearest quarter-step bead in `SUPPORTED_PLAYBACK_RATES`.
pub fn snap_playback_rate(rate: f32) -> f32 {
    let clamped = clamp_playback_rate(rate);
    let mut best = SUPPORTED_PLAYBACK_RATES[0];
    let mut best_diff = f32::MAX;
    for &candidate in &SUPPORTED_PLAYBACK_RATES {
        let diff = (clamped - candidate).abs();
        if diff < best_diff {
            best_diff = diff;
            best = candidate;
        }
    }
    best
}

/// Decoded native-rate mono PCM payload from one MP3 asset.
#[derive(Clone, Debug)]
pub struct DecodedPcm {
    pub samples_mono: Vec<f32>,
    pub sample_rate: u32,
    pub duration_ms: u32,
}

/// Stretched and resampled `48 kHz` mono clip ready for real-time HRTF mixing.
#[derive(Clone, Debug)]
pub struct PreparedClip {
    pub asset_idx: u32,
    pub samples_48k: Arc<Vec<f32>>,
    pub playback_rate: f32,
    pub source_duration_ms: u32,
}

/// Decodes raw MP3 bytes (e.g. from `UniverseCatalog::mp3_slice`) into mono `f32` PCM.
pub fn decode_mp3_bytes(mp3_bytes: &[u8], fallback_duration_ms: u32) -> Result<DecodedPcm, AudioError> {
    let cursor = Cursor::new(mp3_bytes.to_vec());
    let mss = MediaSourceStream::new(Box::new(cursor), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    let probed = symphonia::default::get_probe()
        .probe(
            &hint,
            mss,
            FormatOptions::default(),
            MetadataOptions::default(),
        )
        .map_err(|e| AudioError::ProbeFailed(e.to_string()))?;

    let mut format = probed;
    let track = format
        .default_track(TrackType::Audio)
        .ok_or(AudioError::MissingAudioTrack)?;
    let track_id = track.id;
    let audio_params = track
        .codec_params
        .as_ref()
        .and_then(|cp| cp.audio())
        .ok_or(AudioError::MissingCodecParams)?
        .clone();

    let sample_rate = audio_params.sample_rate.unwrap_or(24_000);
    let mut decoder = symphonia::default::get_codecs()
        .make_audio_decoder(&audio_params, &AudioDecoderOptions::default())
        .map_err(|e| AudioError::DecoderInit(e.to_string()))?;

    let mut scratch: Vec<f32> = Vec::new();
    let mut mono_samples: Vec<f32> = Vec::new();
    let mut detected_sr = sample_rate;

    loop {
        let packet = match format.next_packet() {
            Ok(Some(pkt)) => pkt,
            Ok(None) => break,
            Err(SymphoniaError::IoError(err)) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                break;
            }
            Err(_) => break,
        };
        if packet.track_id != track_id {
            continue;
        }
        match decoder.decode(&packet) {
            Ok(audio_buf) => {
                let spec = audio_buf.spec();
                detected_sr = spec.rate();
                let channels = spec.channels().count().max(1);
                let total_interleaved = audio_buf.samples_interleaved();
                scratch.resize(total_interleaved, f32::MID);
                audio_buf.copy_to_slice_interleaved(&mut scratch);

                if channels == 1 {
                    mono_samples.extend_from_slice(&scratch);
                } else {
                    let inv_ch = 1.0 / channels as f32;
                    for frame in scratch.chunks_exact(channels) {
                        let sum: f32 = frame.iter().copied().sum();
                        mono_samples.push(sum * inv_ch);
                    }
                }
            }
            Err(SymphoniaError::DecodeError(_)) => continue,
            Err(_) => break,
        }
    }

    if mono_samples.is_empty() {
        return Err(AudioError::EmptyPcm);
    }

    let computed_ms = ((mono_samples.len() as f64 / detected_sr as f64) * 1000.0).round() as u32;
    Ok(DecodedPcm {
        samples_mono: mono_samples,
        sample_rate: detected_sr,
        duration_ms: fallback_duration_ms.max(computed_ms),
    })
}

/// Applies pitch-preserving `wsola 0.3.0` timestretch natively at input rate (bypassing cleanly at
/// `1.0x`), then resamples via `rubato 5.1.0` to `48_000 Hz`.
pub fn stretch_and_resample_to_48k(
    pcm: &DecodedPcm,
    playback_rate: f32,
) -> Result<Vec<f32>, AudioError> {
    let rate = clamp_playback_rate(playback_rate);
    let stretched = if (rate - 1.0).abs() <= 1e-4 {
        pcm.samples_mono.clone()
    } else {
        let sr = NonZeroU32::new(pcm.sample_rate)
            .ok_or_else(|| AudioError::Resample("zero sample rate".to_owned()))?;
        wsola::stretch(&pcm.samples_mono, sr, NonZeroU16::MIN, rate)?
    };

    if pcm.sample_rate == OUTPUT_SAMPLE_RATE {
        return Ok(stretched);
    }

    resample_mono_to_48k(&stretched, pcm.sample_rate)
}

fn resample_mono_to_48k(input: &[f32], src_rate: u32) -> Result<Vec<f32>, AudioError> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    let chunk_size = 512usize;
    let ratio = OUTPUT_SAMPLE_RATE as f64 / src_rate as f64;
    let params = SincInterpolationParameters {
        sinc_len: 64,
        f_cutoff: Some(0.92),
        interpolation: SincInterpolationType::Linear,
        oversampling_factor: 128,
        window: WindowFunction::BlackmanHarris2,
    };

    let mut resampler =
        Async::<f32>::new_sinc(ratio, 1.05, &params, chunk_size, 1, FixedAsync::Input)
            .map_err(|e| AudioError::Resample(e.to_string()))?;

    let delay = resampler.output_delay();
    let expected_out_len = ((input.len() as f64) * ratio).round() as usize;
    let max_out_per_chunk = resampler.output_frames_max();
    let mut out_buf = vec![0.0_f32; max_out_per_chunk];
    let mut collected = Vec::with_capacity(expected_out_len + delay + max_out_per_chunk);

    // Feed full chunks + partial final chunk + flush silence to drain sinc kernel latency cleanly.
    let mut padded = Vec::with_capacity(input.len() + delay + chunk_size);
    padded.extend_from_slice(input);
    let extra_zeros = ((delay as f64 / ratio).ceil() as usize) + chunk_size;
    padded.resize(input.len() + extra_zeros, 0.0);

    let mut pos = 0usize;
    while pos < padded.len() {
        let needed = resampler.input_frames_next();
        let avail = padded.len() - pos;
        let frames_in = avail.min(needed);
        let in_slice = &padded[pos..pos + frames_in];
        let in_adapter = InterleavedSlice::new(in_slice, 1, frames_in)
            .map_err(|e| AudioError::Resample(e.to_string()))?;
        let mut out_adapter = InterleavedSlice::new_mut(&mut out_buf, 1, max_out_per_chunk)
            .map_err(|e| AudioError::Resample(e.to_string()))?;

        let mut indexing = Indexing::default();
        if frames_in < needed {
            indexing.partial_len = Some(frames_in);
        }

        let (_read, written) = resampler
            .process_into_buffer(&in_adapter, &mut out_adapter, Some(&indexing))
            .map_err(|e| AudioError::Resample(e.to_string()))?;
        collected.extend_from_slice(&out_buf[..written]);
        pos += frames_in;
    }

    if collected.len() <= delay {
        return Ok(Vec::new());
    }
    let trimmed = &collected[delay..];
    let final_len = expected_out_len.min(trimmed.len());
    Ok(trimmed[..final_len].to_vec())
}

#[derive(Default)]
struct CacheInner {
    pcm_order: VecDeque<u32>,
    pcm_map: HashMap<u32, Arc<DecodedPcm>>,
    clip_order: VecDeque<(u32, u16)>,
    clip_map: HashMap<(u32, u16), PreparedClip>,
}

impl CacheInner {
    const MAX_PCM: usize = 64;
    const MAX_CLIPS: usize = 32;

    fn insert_pcm(&mut self, asset_idx: u32, pcm: Arc<DecodedPcm>) {
        if self.pcm_map.contains_key(&asset_idx) {
            return;
        }
        if self.pcm_order.len() >= Self::MAX_PCM
            && let Some(evict) = self.pcm_order.pop_front()
        {
            self.pcm_map.remove(&evict);
        }
        self.pcm_order.push_back(asset_idx);
        self.pcm_map.insert(asset_idx, pcm);
    }

    fn insert_clip(&mut self, key: (u32, u16), clip: PreparedClip) {
        if self.clip_map.contains_key(&key) {
            return;
        }
        if self.clip_order.len() >= Self::MAX_CLIPS
            && let Some(evict) = self.clip_order.pop_front()
        {
            self.clip_map.remove(&evict);
        }
        self.clip_order.push_back(key);
        self.clip_map.insert(key, clip);
    }
}

/// Thread-safe cache and `rayon` background pre-decoder for SyncVoice narration assets.
#[derive(Clone, Default)]
pub struct NarrationCache {
    inner: Arc<Mutex<CacheInner>>,
}

impl NarrationCache {
    pub fn new() -> Self {
        Self::default()
    }

    fn quantize_rate(rate: f32) -> u16 {
        (clamp_playback_rate(rate) * 100.0).round() as u16
    }

    /// Synchronously retrieves (or decodes + caches) native PCM and `48 kHz` stretched clip.
    pub fn prepare_clip(
        &self,
        catalog: &UniverseCatalog,
        asset_idx: u32,
        playback_rate: f32,
    ) -> Result<PreparedClip, AudioError> {
        let rate = clamp_playback_rate(playback_rate);
        let rate_q = Self::quantize_rate(rate);

        {
            let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(clip) = guard.clip_map.get(&(asset_idx, rate_q)) {
                return Ok(clip.clone());
            }
        }

        let pcm_arc = self.get_or_decode_pcm(catalog, asset_idx)?;
        let resampled = stretch_and_resample_to_48k(&pcm_arc, rate)?;
        let clip = PreparedClip {
            asset_idx,
            samples_48k: Arc::new(resampled),
            playback_rate: rate,
            source_duration_ms: pcm_arc.duration_ms,
        };

        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.insert_clip((asset_idx, rate_q), clip.clone());
        Ok(clip)
    }

    /// Retrieves cached native PCM or decodes MP3 bytes from `UniverseCatalog::mp3_slice`.
    pub fn get_or_decode_pcm(
        &self,
        catalog: &UniverseCatalog,
        asset_idx: u32,
    ) -> Result<Arc<DecodedPcm>, AudioError> {
        {
            let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(pcm) = guard.pcm_map.get(&asset_idx) {
                return Ok(Arc::clone(pcm));
            }
        }

        let asset = catalog
            .audio_asset(asset_idx)
            .ok_or(AudioError::MissingAudioTrack)?;
        let mp3 = catalog
            .mp3_slice(asset)
            .ok_or_else(|| AudioError::ProbeFailed("audio pack not loaded".to_owned()))?;

        let decoded = Arc::new(decode_mp3_bytes(mp3, asset.duration_ms)?);
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.insert_pcm(asset_idx, Arc::clone(&decoded));
        Ok(decoded)
    }

    /// Dispatches non-blocking `rayon` tasks to decode & prepare neighboring assets ahead of dwell lock.
    pub fn prefetch_neighbors(
        &self,
        catalog: &Arc<UniverseCatalog>,
        asset_indices: &[u32],
        playback_rate: f32,
    ) {
        let rate = clamp_playback_rate(playback_rate);
        let rate_q = Self::quantize_rate(rate);

        let missing: Vec<u32> = {
            let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
            asset_indices
                .iter()
                .copied()
                .filter(|&id| !guard.clip_map.contains_key(&(id, rate_q)))
                .collect()
        };

        for asset_idx in missing {
            let cache = self.clone();
            let cat = Arc::clone(catalog);
            rayon::spawn(move || {
                let _ = cache.prepare_clip(&cat, asset_idx, rate);
            });
        }
    }
}
