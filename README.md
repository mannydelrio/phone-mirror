# 📱 PhoneMirror

Cross-platform phone screen mirroring app with touch input and recording. Built with Tauri v2 (Rust) + React.

## Quick Start

### Windows
```powershell
.\setup.ps1
```
Run in **PowerShell as Administrator**.

### Linux / macOS
```bash
./setup.sh
```

### Manual
```bash
# 1. Install prerequisites (Rust, Node.js, adb, build tools)
# 2. Install dependencies
npm install

# 3. Run
cargo tauri dev    # from src-tauri/
# or
npm run tauri dev  # from project root
```

## Features

- **Device Discovery** — Auto-detect connected Android devices via ADB
- **Screen Mirroring** — H.264 screen capture streamed to a canvas view
- **Touch Input** — Tap, swipe, drag forwarded from mouse to phone
- **Navigation** — Home, Back, Recent Apps buttons
- **Recording** — Record screen sessions to MP4
- **Remote Viewing** — WebSocket server for LAN-based mirror sharing

## Architecture

```
┌─────────────────────────────────────────────┐
│  React Frontend (Vite + TypeScript)         │
│  ├── Device List    │── Discovery polling   │
│  ├── Screen Mirror  │── Canvas renderer     │
│  ├── Control Panel  │── Nav + input controls│
│  └── Status Bar     │── FPS, recording state│
└────────────┬────────────────────────────────┘
             │ Tauri IPC
┌────────────▼────────────────────────────────┐
│  Rust Backend (Tauri v2)                    │
│  ├── adb/         │── Device + input        │
│  ├── video/       │── Stream + record       │
│  ├── server/      │── WebSocket LAN view    │
│  └── state/       │── Shared frame buffer   │
└────────────┬────────────────────────────────┘
             │ ADB (USB/Wi-Fi)
┌────────────▼────────────────────────────────┐
│  Android Device                             │
│  ├── screenrecord --stream                   │
│  └── input tap/swipe/drag/keyevent          │
└─────────────────────────────────────────────┘
```

## Requirements

- **Rust** 1.70+ (via rustup)
- **Node.js** 18+
- **Android SDK Platform Tools** (for `adb`)
- **MSVC Build Tools** (Windows) / **clang + GTK3 + WebKit2** (Linux)
- **USB Debugging** enabled on your Android device

## Project Structure

```
phone-mirror/
├── setup.ps1           # Windows one-click setup
├── setup.sh            # Linux/macOS one-click setup
├── src/                # React + TypeScript frontend
│   ├── components/     # UI components
│   └── hooks/          # React hooks
├── src-tauri/          # Tauri v2 backend
│   ├── src/
│   │   ├── adb/        # ADB integration
│   │   ├── video/      # Screen capture & recording
│   │   ├── server.rs   # WebSocket remote viewing
│   │   └── state.rs    # Shared app state
│   └── tauri.conf.json # Tauri config
└── package.json
```

## License

MIT
