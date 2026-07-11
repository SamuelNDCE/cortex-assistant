# jarvis-assistant

Building-block Rust crates for a fully on-device voice assistant: wake word → VAD → speech-to-text → intent parsing → Claude API → text-to-speech, with no cloud STT/TTS dependency (whisper.cpp and Piper both run locally).

This is the assistant's **engine**, not a full app — there's no `main.rs`/UI here, since the reference implementation wires in a proprietary memory backend that isn't included. Wire these crates into your own binary and plug in whatever persistent-memory store (or none) you want.

## Crates

- **jarvis-core** — `AppConfig` (env-var loading), `Command` enum + `parse_intent()` (keyword-based intent classification: `Ask`, `Remember`, `SearchVault`, `Shutdown`, `Unknown`), and `Orchestrator` (routes a transcript string to a `Command`).
- **jarvis-voice** — `AudioCapture` (mic input via `cpal`), `resample_to_16k`, `Vad` (voice-activity detection), `Stt` (whisper.cpp speech-to-text via `whisper-rs`), `Tts` (Piper text-to-speech, shells out to a `piper.exe` binary).
- **jarvis-security** — `OwnerAuth` (SHA-256 PIN hashing/verification — the plaintext PIN is hashed immediately and never stored), plus an optional `intruder` feature (`IntruderDetector`, OpenCV Haar-cascade face detection over a video capture device — off by default, gated behind `--features intruder`).
- **jarvis-agents** — `ClaudeClient`, a thin wrapper around the Anthropic Messages API for the assistant's reasoning loop.

## Typical wiring (what the excluded `main.rs` does)

```rust
let cfg = AppConfig::from_env()?;
let stt = Stt::new(&cfg.model_path())?;
let tts = Tts::new(&cfg.piper_path, &cfg.voice_model);
let auth = OwnerAuth::new(&cfg.owner_pin);
let mut claude = ClaudeClient::new(&cfg.anthropic_api_key);

let (mut audio_rx, _stream) = AudioCapture::start()?;
// ... VAD loop: buffer speech chunks, run Stt::transcribe on utterance end ...
let command = parse_intent(&transcript);
match command {
    Command::Ask(q) => { /* send q to claude, speak the reply via tts */ }
    Command::Remember(fact) => { /* your own persistence here */ }
    Command::SearchVault(q) => { /* your own retrieval here */ }
    Command::Shutdown => { /* auth.verify(pin_attempt) before honoring */ }
    Command::Unknown(_) => {}
}
```

## Setup

1. `cp .env.example .env` and fill in `ANTHROPIC_API_KEY` (and `OWNER_PIN` if using `jarvis-security`).
2. Download the Whisper STT model per `models/DOWNLOAD_MODEL.txt` (a direct Hugging Face link — `ggml-tiny.en.bin` by default, larger models available for better accuracy).
3. Download Piper + a voice per `resources/piper/DOWNLOAD_INSTRUCTIONS.txt`.
4. `cargo build` (add `--features intruder` on `jarvis-security` if you want face-detection support — requires OpenCV installed on your system).
5. `cargo test` runs each crate's unit tests (intent parsing, auth hashing, orchestrator routing).

## Part of a larger collection

This repo is one piece of a set of tools published together — see [toolkit](https://github.com/SamuelNDCE/toolkit) for the full index.

## License

MIT — see [LICENSE](LICENSE).
