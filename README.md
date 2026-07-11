# sam-jarvis-assistant

Building-block Rust crates for a fully on-device voice assistant: wake word → speech-to-text → intent parsing → Claude API → text-to-speech, with no cloud STT/TTS dependency.

- **jarvis-core** — config loading, intent parsing, shared types.
- **jarvis-voice** — audio capture, VAD, whisper.cpp STT (`whisper-rs`), piper TTS.
- **jarvis-security** — owner authentication (PIN/voice).
- **jarvis-agents** — Claude API client for the assistant's reasoning loop.

This is the assistant's engine, not a full app — `main.rs`/UI is intentionally not included here since the reference implementation bundles a proprietary memory backend. Wire these crates into your own binary and swap in whatever persistent-memory store you like.

## Setup

1. `cp .env.example .env` and fill in `ANTHROPIC_API_KEY`.
2. Download the Whisper model per `models/DOWNLOAD_MODEL.txt`.
3. Download Piper + a voice per `resources/piper/DOWNLOAD_INSTRUCTIONS.txt`.
4. `cargo build`.

## Part of a larger collection

This repo is one piece of a set of tools published together — see [sam-toolkit](https://github.com/SamuelNDCE/sam-toolkit) for the full index.

## License

MIT — see [LICENSE](LICENSE).
