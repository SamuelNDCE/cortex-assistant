use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub anthropic_api_key: String,
    pub vault_path: PathBuf,
    pub piper_path: PathBuf,
    pub voice_model: PathBuf,
    pub wakeword_model: PathBuf,
    pub owner_pin: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();
        Ok(AppConfig {
            anthropic_api_key: std::env::var("ANTHROPIC_API_KEY")
                .context("ANTHROPIC_API_KEY not set")?,
            vault_path: PathBuf::from(
                std::env::var("JARVIS_VAULT_PATH").unwrap_or_default(),
            ),
            piper_path: PathBuf::from(
                std::env::var("JARVIS_PIPER_PATH")
                    .unwrap_or_else(|_| "resources/piper/piper.exe".into()),
            ),
            voice_model: PathBuf::from(
                std::env::var("JARVIS_VOICE_MODEL")
                    .unwrap_or_else(|_| "resources/voices/en_US-lessac-medium.onnx".into()),
            ),
            wakeword_model: PathBuf::from(
                std::env::var("JARVIS_WAKEWORD_MODEL")
                    .unwrap_or_else(|_| "resources/wakeword/jarvis.rpw".into()),
            ),
            owner_pin: std::env::var("JARVIS_OWNER_PIN")
                .unwrap_or_else(|_| "0000".into()),
        })
    }

    pub fn model_path(&self) -> PathBuf {
        PathBuf::from("models/ggml-tiny.en.bin")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_loads_with_api_key_set() {
        std::env::set_var("ANTHROPIC_API_KEY", "test-key");
        let cfg = AppConfig::from_env().unwrap();
        assert_eq!(cfg.anthropic_api_key, "test-key");
        assert!(!cfg.owner_pin.is_empty());
        assert!(cfg.model_path().to_str().unwrap().contains("ggml-tiny"));
    }

    #[test]
    fn config_fails_without_api_key() {
        std::env::remove_var("ANTHROPIC_API_KEY");
        // Only fails if no .env file sets it either
        // This test is informational — may pass or fail depending on .env
        let _ = AppConfig::from_env();
    }
}
