use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use tokio::sync::mpsc;

/// Captures microphone audio and sends f32 sample chunks over a channel.
pub struct AudioCapture;

impl AudioCapture {
    /// Start audio capture. Returns (receiver, stream_handle).
    /// Keep the stream_handle alive — dropping it stops capture.
    pub fn start() -> Result<(mpsc::Receiver<Vec<f32>>, impl StreamTrait)> {
        let host   = cpal::default_host();
        let device = host.default_input_device().context("no input device found")?;
        let config = device.default_input_config()?;
        let (tx, rx) = mpsc::channel::<Vec<f32>>(32);

        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _| {
                let _ = tx.try_send(data.to_vec());
            },
            |err| tracing::error!("audio stream error: {err}"),
            None,
        )?;
        stream.play()?;
        Ok((rx, stream))
    }
}
