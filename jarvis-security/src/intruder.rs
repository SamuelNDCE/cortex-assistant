#![cfg(feature = "intruder")]
use anyhow::{Context, Result};
use opencv::{
    core::{self, Rect, Scalar, Size, Vector},
    imgproc,
    objdetect::CascadeClassifier,
    prelude::*,
    videoio::{self, VideoCapture},
};
use std::path::Path;
use tokio::sync::mpsc;

pub struct IntruderDetector {
    alert_tx:     mpsc::UnboundedSender<String>,
    cascade_path: std::path::PathBuf,
}

impl IntruderDetector {
    pub fn new(
        alert_tx:     mpsc::UnboundedSender<String>,
        cascade_path: impl AsRef<Path>,
    ) -> Self {
        Self {
            alert_tx,
            cascade_path: cascade_path.as_ref().to_owned(),
        }
    }

    /// Run detection loop. Intended to be called inside `std::thread::spawn`.
    /// Sends alert strings to `alert_tx` on each detection event.
    /// Runs until the webcam is unavailable or the channel is closed.
    pub fn run_blocking(self) -> Result<()> {
        let cascade_str = self.cascade_path.to_str()
            .context("cascade path not UTF-8")?;

        let mut cam = VideoCapture::new(0, videoio::CAP_ANY)
            .context("open webcam (index 0)")?;
        if !videoio::VideoCapture::is_opened(&cam).context("check webcam open")? {
            anyhow::bail!("could not open default webcam");
        }

        let mut classifier = CascadeClassifier::new(cascade_str)
            .context("load cascade classifier")?;
        let mut frame = core::Mat::default();

        loop {
            cam.read(&mut frame).context("read frame")?;
            if frame.empty() { continue; }

            let mut gray = core::Mat::default();
            imgproc::cvt_color(&frame, &mut gray, imgproc::COLOR_BGR2GRAY, 0)
                .context("convert to grayscale")?;

            let mut faces: Vector<Rect> = Vector::new();
            classifier.detect_multi_scale(
                &gray,
                &mut faces,
                1.1,
                3,
                0,
                Size::new(30, 30),
                Size::new(0, 0),
            ).context("detect faces")?;

            if !faces.is_empty() {
                let msg = format!(
                    "[SECURITY] ⚠ {} person(s) detected at {}",
                    faces.len(),
                    chrono::Local::now().format("%H:%M:%S"),
                );
                if self.alert_tx.send(msg).is_err() {
                    // UI channel closed — exit gracefully
                    break;
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        Ok(())
    }
}
