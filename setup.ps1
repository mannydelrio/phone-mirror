<#
.SYNOPSIS
  One-click setup for PhoneMirror on Windows
.DESCRIPTION
  Checks and installs all prerequisites, then builds the project.
.EXAMPLE
  .\setup.ps1
#>

$ErrorActionPreference = "Stop"

function Write-Step($msg) {
    Write-Host "`n=== $msg ===" -ForegroundColor Cyan
}

function Write-Ok($msg) {
    Write-Host "  ✓ $msg" -ForegroundColor Green
}

function Write-Skip($msg) {
    Write-Host "  ⊘ $msg" -ForegroundColor Yellow
}

function Write-Err($msg) {
    Write-Host "  ✗ $msg" -ForegroundColor Red
}

function Test-Admin {
    $identity = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($identity)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# ── Step 1: Check for Administrator ──────────────────────────────
Write-Step "Checking permissions"
if (-not (Test-Admin)) {
    Write-Err "This script requires Administrator privileges."
    Write-Host "Right-click PowerShell → 'Run as administrator' and try again." -ForegroundColor Yellow
    Read-Host "Press Enter to exit"
    exit 1
}
Write-Ok "Running as Administrator"

# ── Step 2: Rust ──────────────────────────────────────────────────
Write-Step "Checking Rust"
$cargo = Get-Command cargo -ErrorAction SilentlyContinue

if (-not $cargo) {
    Write-Host "Installing Rust via rustup..." -ForegroundColor White
    $env:RUSTUP_DIST_SERVER = "https://rustup.rs"
    $env:RUSTUP_UPDATE_ROOT = "https://rustup.rs"
    Invoke-WebRequest -Uri "https://win.rustup.rs/x86_64" -OutFile "$env:TEMP\rustup-init.exe"
    & "$env:TEMP\rustup-init.exe" -y --default-toolchain stable --default-host x86_64-pc-windows-msvc | Out-Null
    Remove-Item "$env:TEMP\rustup-init.exe" -ErrorAction SilentlyContinue

    # Reload PATH
    $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
    Write-Ok "Rust installed"
} else {
    $version = (cargo --version).Trim()
    Write-Ok "Rust already installed ($version)"
}

# Ensure MSVC build tools
Write-Step "Checking C++ Build Tools"
$cl = Get-Command cl.exe -ErrorAction SilentlyContinue
if (-not $cl) {
    Write-Host "C++ Build Tools not found." -ForegroundColor Yellow
    Write-Host "Please install 'Desktop development with C++' from:" -ForegroundColor Yellow
    Write-Host "  https://visualstudio.microsoft.com/visual-cpp-build-tools/" -ForegroundColor Cyan
    Write-Host "Then re-run this script." -ForegroundColor Yellow
    Read-Host "Press Enter to continue anyway"
} else {
    Write-Ok "MSVC Build Tools found"
}

# ── Step 3: Node.js ─────────────────────────────────────────────
Write-Step "Checking Node.js"
$node = Get-Command node -ErrorAction SilentlyContinue

if (-not $node) {
    Write-Host "Installing Node.js LTS..." -ForegroundColor White
    $nodeInstaller = "$env:TEMP\node-lts.msi"
    $resp = Invoke-WebRequest "https://nodejs.org/dist/latest-v22.x/sha256sums.txt" -UseBasicParsing
    # Download the Windows x64 MSI
    $nodeUrl = "https://nodejs.org/dist/latest-v22.x/node-latest-v22.x-x64.msi"
    Invoke-WebRequest -Uri $nodeUrl -OutFile $nodeInstaller
    Start-Process msiexec.exe -ArgumentList "/i `"$nodeInstaller`" /quiet /norestart" -Wait
    Remove-Item $nodeInstaller -ErrorAction SilentlyContinue

    $env:PATH = [System.Environment]::GetEnvironmentVariable("PATH", "Machine")
    Write-Ok "Node.js installed"
} else {
    $version = (node --version).Trim()
    Write-Ok "Node.js already installed ($version)"
}

# ── Step 4: Android Platform Tools ──────────────────────────────
Write-Step "Checking Android Platform Tools (adb)"
$adbPath = Join-Path $PSScriptRoot "platform-tools"
$adbExe = Join-Path $adbPath "adb.exe"

if (-not (Test-Path $adbExe)) {
    Write-Host "Downloading Android Platform Tools..." -ForegroundColor White
    $zipUrl = "https://dl.google.com/android/repository/platform-tools-latest-windows.zip"
    $zipFile = "$env:TEMP\platform-tools.zip"
    Invoke-WebRequest -Uri $zipUrl -OutFile $zipFile

    if (Test-Path $adbPath) { Remove-Item $adbPath -Recurse -Force }
    New-Item $adbPath -ItemType Directory | Out-Null
    Expand-Archive -Path $zipFile -DestinationPath $adbPath -Force
    Remove-Item $zipFile -ErrorAction SilentlyContinue
    Write-Ok "Android Platform Tools downloaded"
} else {
    Write-Ok "Android Platform Tools already present"
}

# Add to PATH for this session
$env:PATH = "$adbPath;$env:PATH"

# ── Step 5: npm dependencies ────────────────────────────────────
Write-Step "Installing npm dependencies"
npm install --prefix $PSScriptRoot 2>&1 | Out-Null
Write-Ok "npm dependencies installed"

# ── Step 6: Rust build check ────────────────────────────────────
Write-Step "Checking Rust build"
Push-Location (Join-Path $PSScriptRoot "src-tauri")
$buildResult = cargo check 2>&1
Pop-Location

if ($LASTEXITCODE -eq 0) {
    Write-Ok "Rust build passes"
} else {
    Write-Err "Rust build failed — see output above"
}

# ── Step 7: USB Debugging reminder ──────────────────────────────
Write-Step "Final Checklist"
Write-Host "  1. Enable USB Debugging on your phone:" -ForegroundColor White
Write-Host "     Settings → About Phone → Tap 'Build Number' 7 times" -ForegroundColor Gray
Write-Host "     Settings → Developer Options → USB Debugging ON" -ForegroundColor Gray
Write-Host ""
Write-Host "  2. Connect your phone via USB" -ForegroundColor White
Write-Host ""
Write-Host "  3. Verify connection:" -ForegroundColor White
Write-Host "     adb devices" -ForegroundColor Cyan
Write-Host ""
Write-Host "  4. Run the app:" -ForegroundColor White
Write-Host "     cargo tauri dev   (from src-tauri folder)" -ForegroundColor Cyan
Write-Host "     or" -ForegroundColor Gray
Write-Host "     npm run tauri dev  (from project root)" -ForegroundColor Cyan

Write-Host ""
Write-Host "All done! 🎉" -ForegroundColor Green
Read-Host "`nPress Enter to exit"
