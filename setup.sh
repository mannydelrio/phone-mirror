#!/usr/bin/env bash
#
# One-click setup for PhoneMirror on Linux/macOS
# Usage: ./setup.sh
#
set -euo pipefail

# ── Colors ────────────────────────────────────────────────────────
CYAN='\033[0;36m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

step()  { echo -e "\n${CYAN}=== $1 ===${NC}"; }
ok()    { echo -e "  ${GREEN}✓ $1${NC}"; }
skip()  { echo -e "  ${YELLOW}⊘ $1${NC}"; }
err()   { echo -e "  ${RED}✗ $1${NC}"; }

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
OS="$(uname -s)"

# ── Detect package manager ────────────────────────────────────────
PKG_CMD=""
if command -v apt-get &>/dev/null; then
    PKG_CMD="sudo apt-get install -y"
elif command -v dnf &>/dev/null; then
    PKG_CMD="sudo dnf install -y"
elif command -v yum &>/dev/null; then
    PKG_CMD="sudo yum install -y"
elif command -v pacman &>/dev/null; then
    PKG_CMD="sudo pacman -S --noconfirm"
elif command -v brew &>/dev/null; then
    PKG_CMD="brew install"
fi

# ── Step 1: Rust ──────────────────────────────────────────────────
step "Checking Rust"
if command -v cargo &>/dev/null; then
    ok "Rust ($(cargo --version | awk '{print $2}'))"
else
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env" 2>/dev/null || true
    ok "Rust installed ($(cargo --version | awk '{print $2}'))"
fi

# ── Step 2: Node.js ──────────────────────────────────────────────
step "Checking Node.js"
if command -v node &>/dev/null; then
    ok "Node.js ($(node --version))"
else
    if [ -n "$PKG_CMD" ]; then
        if command -v brew &>/dev/null; then
            $PKG_CMD node
        else
            $PKG_CMD nodejs npm 2>/dev/null || $PKG_CMD nodejs 2>/dev/null || true
        fi
        ok "Node.js installed ($(node --version))"
    else
        err "Node.js not found — please install from https://nodejs.org/"
    fi
fi

# ── Step 3: Build dependencies ───────────────────────────────────
step "Checking build dependencies"
if [ -n "$PKG_CMD" ]; then
    case "$OS" in
        Linux)
            if command -v apt-get &>/dev/null; then
                sudo apt-get install -y -qq \
                    pkg-config libssl-dev \
                    libgtk-3-dev libwebkit2gtk-4.1-dev \
                    ffmpeg clang libc6-dev 2>/dev/null || true
            elif command -v dnf &>/dev/null; then
                sudo dnf install -y -q \
                    pkgconfig openssl-devel \
                    gtk3-devel webkit2gtk4.1-devel \
                    ffmpeg clang glibc-devel 2>/dev/null || true
            elif command -v pacman &>/dev/null; then
                sudo pacman -S --noconfirm --quiet \
                    pkg-config openssl \
                    gtk3 webkit2gtk-4.1 \
                    ffmpeg clang glibc 2>/dev/null || true
            fi
            ;;
        Darwin)
            $PKG_CMD pkg-config openssl gtk+3 webkit2gtk ffmpeg 2>/dev/null || true
            ;;
    esac
    ok "Build dependencies checked"
else
    skip "No package manager found — install dependencies manually"
fi

# ── Step 4: Android Platform Tools ──────────────────────────────
step "Checking Android Platform Tools (adb)"
if command -v adb &>/dev/null; then
    ok "adb already available ($(adb version | head -1))"
else
    ADB_DIR="$SCRIPT_DIR/platform-tools"
    if [ "$OS" = "Darwin" ]; then
        ADB_URL="https://dl.google.com/android/repository/platform-tools-latest-darwin.zip"
        ADB_ARCH="darwin"
    elif [ "$OS" = "Linux" ]; then
        ADB_URL="https://dl.google.com/android/repository/platform-tools-latest-linux.zip"
        ADB_ARCH="linux"
    else
        ADB_URL="https://dl.google.com/android/repository/platform-tools-latest-windows.zip"
        ADB_ARCH="windows"
    fi

    echo "Downloading Android Platform Tools for $ADB_ARCH..."
    ADB_ZIP="/tmp/platform-tools.zip"
    curl -sL "$ADB_URL" -o "$ADB_ZIP"

    mkdir -p "$ADB_DIR"
    unzip -qo "$ADB_ZIP" -d "$ADB_DIR" || true
    rm -f "$ADB_ZIP"

    if [ -f "$ADB_DIR/adb" ]; then
        chmod +x "$ADB_DIR/adb"
        export PATH="$ADB_DIR:$PATH"
        ok "Android Platform Tools downloaded to $ADB_DIR"
    else
        err "adb not found — add Android SDK Platform Tools to PATH"
    fi
fi

# ── Step 5: npm dependencies ────────────────────────────────────
step "Installing npm dependencies"
npm install --prefix "$SCRIPT_DIR" 2>&1 | tail -3
ok "npm dependencies installed"

# ── Step 6: Rust build check ────────────────────────────────────
step "Checking Rust build"
source "$HOME/.cargo/env" 2>/dev/null || true
pushd "$SCRIPT_DIR/src-tauri" > /dev/null
if cargo check 2>&1 | grep -q "^error"; then
    err "Rust build failed — see errors above"
else
    ok "Rust build passes"
fi
popd > /dev/null

# ── Step 7: USB Debugging reminder ──────────────────────────────
step "Final Checklist"
echo -e "  1. Enable USB Debugging on your phone:"
echo -e "     Settings → About Phone → Tap 'Build Number' 7 times"
echo -e "     Settings → Developer Options → USB Debugging ON"
echo ""
echo -e "  2. Connect your phone via USB"
echo ""
echo -e "  3. Verify connection:"
echo -e "     ${CYAN}adb devices${NC}"
echo ""
echo -e "  4. Run the app:"
echo -e "     ${CYAN}cargo tauri dev${NC}   (from src-tauri folder)"
echo -e "     or"
echo -e "     ${CYAN}npm run tauri dev${NC}  (from project root)"

echo ""
echo -e "${GREEN}All done! 🎉${NC}"
