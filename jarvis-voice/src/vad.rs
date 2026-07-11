/// Energy-based voice activity detector.
/// Detects speech when RMS of samples exceeds a threshold.
pub struct Vad {
    threshold:      f32,
    silence_frames: usize,
    pub silence_limit: usize, // frames of silence before a segment ends
}

impl Vad {
    pub fn new(threshold: f32, silence_limit_frames: usize) -> Self {
        Self { threshold, silence_frames: 0, silence_limit: silence_limit_frames }
    }

    /// Returns (is_speech, is_segment_end).
    /// is_segment_end is true when silence has exceeded silence_limit consecutive frames.
    pub fn process(&mut self, samples: &[f32]) -> (bool, bool) {
        let rms = (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt();
        let is_speech = rms > self.threshold;
        if is_speech {
            self.silence_frames = 0;
        } else {
            self.silence_frames += 1;
        }
        let segment_end = self.silence_frames >= self.silence_limit;
        (is_speech, segment_end)
    }

    pub fn reset(&mut self) {
        self.silence_frames = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn speech_detected_above_threshold() {
        let mut vad = Vad::new(0.01, 10);
        let loud = vec![0.5f32; 256];
        let (is_speech, _) = vad.process(&loud);
        assert!(is_speech);
    }

    #[test]
    fn silence_below_threshold() {
        let mut vad = Vad::new(0.01, 10);
        let quiet = vec![0.001f32; 256];
        let (is_speech, _) = vad.process(&quiet);
        assert!(!is_speech);
    }

    #[test]
    fn segment_ends_after_silence_limit() {
        let mut vad = Vad::new(0.01, 3);
        let quiet = vec![0.0f32; 256];
        vad.process(&quiet); // frame 1
        vad.process(&quiet); // frame 2
        vad.process(&quiet); // frame 3 — hits limit
        let (_, ended) = vad.process(&quiet); // frame 4
        assert!(ended);
    }

    #[test]
    fn speech_resets_silence_counter() {
        let mut vad = Vad::new(0.01, 3);
        let quiet = vec![0.0f32; 256];
        let loud  = vec![0.5f32; 256];
        vad.process(&quiet); // 1 silence
        vad.process(&quiet); // 2 silences
        vad.process(&loud);  // speech — resets counter
        let (_, ended) = vad.process(&quiet); // only 1 silence after reset
        assert!(!ended);
    }

    #[test]
    fn reset_clears_counter() {
        let mut vad = Vad::new(0.01, 2);
        let quiet = vec![0.0f32; 256];
        vad.process(&quiet);
        vad.process(&quiet);
        vad.reset();
        let (_, ended) = vad.process(&quiet); // only 1 after reset
        assert!(!ended);
    }
}
