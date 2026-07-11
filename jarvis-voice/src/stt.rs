use anyhow::{Context, Result};
use std::path::Path;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct Stt {
    ctx: WhisperContext,
}

impl Stt {
    pub fn new(model_path: impl AsRef<Path>) -> Result<Self> {
        let ctx = WhisperContext::new_with_params(
            model_path.as_ref(),
            WhisperContextParameters::default(),
        )
        .map_err(|e| {
            anyhow::anyhow!(
                "failed to load whisper model at {}: {e}",
                model_path.as_ref().display()
            )
        })?;
        Ok(Self { ctx })
    }

    /// Transcribe 16kHz mono f32 audio. Returns trimmed text.
    pub fn transcribe(&self, samples: &[f32]) -> Result<String> {
        let mut state = self.ctx.create_state().context("create whisper state")?;
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_language(Some("en"));
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);

        state.full(params, samples).context("whisper inference failed")?;

        let n = state.full_n_segments();
        let mut text = String::new();
        for i in 0..n {
            if let Some(seg) = state.get_segment(i) {
                let seg_text = seg.to_str().context("get segment text")?;
                text.push_str(seg_text);
            }
        }
        Ok(text.trim().to_string())
    }
}

/// Resample f32 audio from src_hz to 16000 Hz (nearest-neighbor, fast).
/// Whisper always expects 16kHz mono.
pub fn resample_to_16k(samples: &[f32], src_hz: u32) -> Vec<f32> {
    if src_hz == 16000 {
        return samples.to_vec();
    }
    let ratio = 16000.0 / src_hz as f64;
    let out_len = (samples.len() as f64 * ratio) as usize;
    (0..out_len)
        .map(|i| {
            let src_i = (i as f64 / ratio) as usize;
            samples[src_i.min(samples.len() - 1)]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resample_identity_at_16k() {
        let samples = vec![0.1f32, 0.2, 0.3, 0.4];
        let result = resample_to_16k(&samples, 16000);
        assert_eq!(result, samples);
    }

    #[test]
    fn resample_downsamples_from_44100() {
        let samples: Vec<f32> = (0..4410).map(|i| i as f32 / 4410.0).collect();
        let result = resample_to_16k(&samples, 44100);
        // 4410 samples at 44100Hz = 0.1s → should produce ~1600 samples at 16kHz
        assert!(result.len() > 1500 && result.len() < 1700);
    }

    #[test]
    fn resample_empty_input() {
        let result = resample_to_16k(&[], 44100);
        assert!(result.is_empty());
    }
}
