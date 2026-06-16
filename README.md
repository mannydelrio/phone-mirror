# Phone Mirror

Cross-platform phone screen mirroring app — mirror your Android or iOS device to your PC or Mac via USB-C, Wi-Fi, or Bluetooth.

## Features

- **Screen mirroring** — Real-time phone screen on your desktop/browser
- **Input control** — Use your PC mouse/keyboard to control the phone
- **Screen recording** — Record the mirrored session to MP4
- **Cross-platform** — Linux, macOS, Windows
- **Multiple connections** — USB-C, Wi-Fi, or Bluetooth
- **Android ready** — Full support via ADB
- **iOS coming** — SCRP protocol architecture in place

## Tech Stack

- **Rust** — Core video pipeline and protocol handling
- **Tauri v2** — Cross-platform desktop shell
- **React + TypeScript** — Web frontend
- **FFmpeg** — H.264 encoding/decoding

## Quick Start

```bash
# Prerequisites
cargo install tauri-cli
npm install

# Development
cargo tauri dev

# Build release
cargo tauri build
```

## Roadmap

See [implementation plan](.hermes/plans/2026-06-16_120000-phone-mirror-app.md) for the full breakdown.

## License

MIT
