# Rusty Asteroid - Distribution Guide

## About

A classic asteroid game built with the Bevy engine in Rust.

## Available Executables

### Automatically Built (via GitHub Actions)

- **Linux**: `rusty_asteroid-linux-x86_64` (Ubuntu 20.04+ compatible)
- **Windows**: `rusty_asteroid-windows-x86_64.exe` (Windows 10+ 64-bit)
- **macOS Intel**: `rusty_asteroid-macos-x86_64` (macOS 10.15+ Intel)
- **macOS Apple Silicon**: `rusty_asteroid-macos-arm64` (macOS 11.0+ M1/M2/M3)

### Legacy Manual Build

- **File**: `rusty_asteroid-macos-arm64`
- **Platform**: macOS 11.0+ (Apple Silicon M1/M2/M3)
- **Size**: ~41MB

### How to Run

#### Linux

1. Download `rusty_asteroid-linux-x86_64`
2. Open terminal and navigate to download folder
3. Make it executable: `chmod +x rusty_asteroid-linux-x86_64`
4. Run: `./rusty_asteroid-linux-x86_64`

#### Windows

1. Download `rusty_asteroid-windows-x86_64.exe`
2. Double-click to run, or run from Command Prompt

#### macOS

1. Download the appropriate version for your Mac:
   - Intel Macs: `rusty_asteroid-macos-x86_64`
   - Apple Silicon (M1/M2/M3): `rusty_asteroid-macos-arm64`
2. Open Terminal and navigate to the download folder
3. Make it executable: `chmod +x rusty_asteroid-macos-*`
4. Run: `./rusty_asteroid-macos-*`

**Note**: You may need to allow the app in System Preferences > Security & Privacy if macOS blocks it.

## Building for Other Platforms

To build for Linux or Windows, you can use cross-compilation:

### For Linux (from macOS)

```bash
# Install target
rustup target add x86_64-unknown-linux-gnu

# Build without audio for easier cross-compilation
cargo build --release --target x86_64-unknown-linux-gnu --no-default-features --features "bevy/bevy_winit,bevy/bevy_render,bevy/bevy_core_pipeline,bevy/bevy_sprite,bevy/bevy_text,bevy/bevy_ui,bevy/png"
```

### For Windows (from macOS)

```bash
# Install target
rustup target add x86_64-pc-windows-gnu

# Build
cargo build --release --target x86_64-pc-windows-gnu
```

## Requirements

- No additional dependencies required - the executable is statically linked
- Graphics: OpenGL 3.3+ support
- Audio: System audio support (if enabled)

## File Structure for GitHub Release

```
releases/
├── rusty_asteroid-macos-arm64          # macOS Apple Silicon
├── rusty_asteroid-linux-x86_64         # Linux 64-bit
├── rusty_asteroid-windows-x86_64.exe   # Windows 64-bit
└── README.md                           # This file
```
