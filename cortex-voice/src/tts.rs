use anyhow::{Context, Result};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub struct Tts {
    piper_exe:   PathBuf,
    voice_model: PathBuf,
}

impl Tts {
    pub fn new(piper_exe: impl AsRef<Path>, voice_model: impl AsRef<Path>) -> Self {
        Self {
            piper_exe:   piper_exe.as_ref().to_owned(),
            voice_model: voice_model.as_ref().to_owned(),
        }
    }

    /// Synthesise text to speech and play it. Blocks until playback completes.
    pub fn speak(&self, text: &str) -> Result<()> {
        let pcm = self.synthesise(text)?;
        self.play_pcm(&pcm)
    }

    /// Run piper and return raw PCM bytes (22050 Hz, mono, s16le).
    pub fn synthesise(&self, text: &str) -> Result<Vec<u8>> {
        let model_str = self.voice_model.to_str()
            .context("voice model path is not valid UTF-8")?;

        let mut child = Command::new(&self.piper_exe)
            .args(["--model", model_str, "--output-raw"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .context("failed to start piper — is piper.exe in resources/piper/?")?;

        child.stdin.as_mut()
            .context("piper stdin unavailable")?
            .write_all(text.as_bytes())?;
        drop(child.stdin.take());

        let output = child.wait_with_output().context("piper process failed")?;
        Ok(output.stdout)
    }

    /// Play raw PCM (22050 Hz mono s16le) via rodio.
    pub fn play_pcm(&self, pcm: &[u8]) -> Result<()> {
        use std::num::NonZero;

        if pcm.is_empty() { return Ok(()); }

        // Convert s16le bytes → f32 samples (rodio 0.22 uses f32 internally)
        let samples: Vec<f32> = pcm.chunks_exact(2)
            .map(|b| {
                let s = i16::from_le_bytes([b[0], b[1]]);
                s as f32 / i16::MAX as f32
            })
            .collect();

        // Piper outputs 22050 Hz mono s16le
        let channels    = NonZero::new(1u16).unwrap();
        let sample_rate = NonZero::new(22050u32).unwrap();
        let source = rodio::buffer::SamplesBuffer::new(channels, sample_rate, samples);

        let handle = rodio::DeviceSinkBuilder::open_default_sink()
            .context("no audio output device")?;
        let player = rodio::Player::connect_new(&handle.mixer());
        player.append(source);
        player.sleep_until_end();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tts_new_stores_paths() {
        let tts = Tts::new("resources/piper/piper.exe", "resources/voices/model.onnx");
        assert!(tts.piper_exe.to_str().unwrap().contains("piper.exe"));
        assert!(tts.voice_model.to_str().unwrap().contains("model.onnx"));
    }

    #[test]
    fn play_pcm_empty_is_noop() {
        let tts = Tts::new("piper.exe", "model.onnx");
        // Empty PCM should return Ok immediately without touching audio hardware
        let result = tts.play_pcm(&[]);
        assert!(result.is_ok());
    }
}
