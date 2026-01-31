# QuickTransform

Lightning-fast encoder/decoder/hasher for the command line and desktop.

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License](https://img.shields.io/github/license/Brutus1066/QuickTransform)](LICENSE)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-blue)]()

## Overview

QuickTransform is a fast, offline utility for encoding, decoding, hashing, and generating random data. No internet required. No data leaves your machine.

- **CLI** for scripts and power users
- **GUI** for quick visual operations
- **Cross-platform** - Windows, Linux, macOS

## Why QuickTransform?

Online encoding/hashing tools are convenient but risky:
- They may log your data
- They require internet access
- They can be slow or ad-filled

QuickTransform runs locally. Your data never leaves your machine. It's instant, private, and works offline.

## Features

| Category | Operations |
|----------|------------|
| Encode/Decode | Base64, Hex, URL, HTML entities |
| Hash | MD5, SHA-1, SHA-256, SHA-512 |
| Generate | UUID v4, passwords, random hex/base64 |

## Screenshots

| Dark Theme | Light Theme |
|------------|-------------|
| ![Encode Dark](screenshots/encode-dark.png) | ![Hash Light](screenshots/hash-light.png) |
| ![About Dark](screenshots/about-dark.png) | ![Quick Start Light](screenshots/quickstart-light.png) |

## Installation

### Pre-built Binaries

Download from [Releases](https://github.com/Brutus1066/QuickTransform/releases):

| Platform | CLI | GUI |
|----------|-----|-----|
| Windows | `qt.exe` (~1 MB) | `qt-gui.exe` (~4 MB) |
| Linux | `qt` (~900 KB) | `qt-gui` (~4 MB) |
| macOS | `qt` (~1 MB) | `qt-gui` (~5 MB) |

### From Source

```bash
git clone https://github.com/Brutus1066/QuickTransform.git
cd QuickTransform

# CLI only
cargo build --release

# CLI + GUI
cargo build --release --features gui
```

#### Linux Dependencies (GUI)

```bash
# Ubuntu/Debian
sudo apt install libgtk-3-dev

# Fedora
sudo dnf install gtk3-devel

# Arch
sudo pacman -S gtk3
```

## Usage

### CLI Examples

```bash
# Encoding
qt b64 "hello world"          # Base64 encode
qt b64d "aGVsbG8gd29ybGQ="    # Base64 decode
qt hex "hello"                # Hex encode
qt url "hello world"          # URL encode

# Hashing
qt sha256 file.txt            # Hash file
qt sha256 -s "password"       # Hash string
qt hash file.txt              # All algorithms

# Generation
qt uuid                       # UUID v4
qt pass 24                    # 24-char password
qt pass 16 --alpha            # Alphanumeric only
qt randhex 32                 # 32 random bytes as hex

# Pipes
echo "secret" | qt b64
cat file.txt | qt sha256
```

### GUI

```bash
qt-gui
```

## Commands

| Command | Description |
|---------|-------------|
| `b64` / `b64d` | Base64 encode/decode |
| `hex` / `hexd` | Hex encode/decode |
| `url` / `urld` | URL encode/decode |
| `html` / `htmld` | HTML entity encode/decode |
| `md5` | MD5 hash |
| `sha1` | SHA-1 hash |
| `sha256` | SHA-256 hash |
| `sha512` | SHA-512 hash |
| `hash` | All hash algorithms |
| `uuid` | Generate UUID v4 |
| `pass [len]` | Generate password |
| `randhex [bytes]` | Random hex bytes |
| `randb64 [bytes]` | Random base64 bytes |
| `info` | Version info |
| `guide` | Help guide |

## Building

### Requirements

- Rust 1.70+
- Cargo

### Build Commands

```bash
cargo build --release                 # CLI
cargo build --release --features gui  # CLI + GUI
cargo test                            # Run tests
```

### Install Scripts

```bash
# Windows
.\scripts\install-windows.ps1

# Linux
./scripts/install-linux.sh
```

## Project Structure

```
src/
  main.rs          # CLI entry point
  gui_main.rs      # GUI entry point
  lib.rs           # Library exports
  transforms/
    encode.rs      # Encoding functions
    hash.rs        # Hashing functions
    generate.rs    # Generation functions
```

## Troubleshooting

**GUI not starting on Linux?**
Install GTK3 development libraries and rebuild:
```bash
sudo apt install libgtk-3-dev    # Ubuntu/Debian
cargo build --release --features gui
```

**Command not found after install?**
Add the install directory to your PATH:
```bash
export PATH="$PATH:$HOME/.local/bin"
```

**Build fails on Windows?**
Ensure you have the MSVC toolchain installed via Visual Studio Build Tools.

## License

MIT License - see [LICENSE](LICENSE)

---

**LAZYFROG** | [kindware.dev](https://kindware.dev)
