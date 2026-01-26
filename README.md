<p align="center">
  <img src="https://img.shields.io/badge/Rust-1.70%2B-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Rust">
  <img src="https://img.shields.io/badge/CLI-Enabled-green?style=for-the-badge&logo=windowsterminal&logoColor=white" alt="CLI">
  <img src="https://img.shields.io/badge/GUI-Included-blue?style=for-the-badge&logo=windows&logoColor=white" alt="GUI">
</p>

<p align="center">
  <img src="https://img.shields.io/github/license/Brutus1066/QuickTransform?style=flat-square" alt="License">
  <img src="https://img.shields.io/github/stars/Brutus1066/QuickTransform?style=flat-square" alt="Stars">
  <img src="https://img.shields.io/github/forks/Brutus1066/QuickTransform?style=flat-square" alt="Forks">
  <img src="https://img.shields.io/github/issues/Brutus1066/QuickTransform?style=flat-square" alt="Issues">
  <img src="https://img.shields.io/github/last-commit/Brutus1066/QuickTransform?style=flat-square" alt="Last Commit">
</p>

<h1 align="center">⚡ QuickTransform</h1>

<p align="center">
  <strong>Lightning-fast encoder/decoder/hasher for the command line and desktop</strong>
</p>

<p align="center">
  <a href="#-features">Features</a> •
  <a href="#-screenshots">Screenshots</a> •
  <a href="#-installation">Installation</a> •
  <a href="#-usage">Usage</a> •
  <a href="#-commands">Commands</a> •
  <a href="#-building">Building</a>
</p>

---

## 💡 Why QuickTransform?

**The Problem:** You need to encode, decode, or hash data quickly. Online tools are slow, track you, or require internet.

**The Solution:** One command, instant result. Works 100% offline. No data leaves your machine. CLI for power users, GUI for everyone else.

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🔐 **Encode/Decode** | Base64, Hex, URL, HTML entities |
| 🔒 **Hash** | MD5, SHA1, SHA256, SHA512 (files or strings) |
| 🎲 **Generate** | UUIDs, secure passwords, random bytes |
| 💻 **Cross-platform** | Windows, Linux, macOS |
| 📟 **Pipe-friendly** | Works seamlessly with stdin/stdout |
| 🖥️ **GUI included** | Modern interface with dark/light themes |
| 🔌 **Zero network** | All operations offline - no data leaks |

---

## 📸 Screenshots

| Dark Theme | Light Theme |
|------------|-------------|
| ![Encode Dark](screenshots/encode-dark.png) | ![Hash Light](screenshots/hash-light.png) |
| ![About Dark](screenshots/about-dark.png) | ![Quick Start Light](screenshots/quickstart-light.png) |

---

## 📦 Installation

### Option 1: From Source (Recommended)

```bash
# Clone the repository
git clone https://github.com/Brutus1066/QuickTransform.git
cd QuickTransform

# Build CLI only
cargo build --release

# Build with GUI
cargo build --release --features gui

# Install to system
cargo install --path .
```

### Option 2: Pre-built Binaries

Download from [Releases](https://github.com/Brutus1066/QuickTransform/releases).

---

## 🚀 Usage

### 🔄 Encoding & Decoding

```bash
# Base64
qt b64 "hello world"              # SGVsbG8gV29ybGQh
qt b64d "SGVsbG8gV29ybGQh"        # hello world

# Hex
qt hex "hello"                    # 68656c6c6f
qt hexd "68656c6c6f"              # hello

# URL
qt url "hello world & more"       # hello%20world%20%26%20more
qt urld "hello%20world"           # hello world

# HTML Entities
qt html "<script>alert(1)</script>"
qt htmld "&lt;script&gt;"
```

### 🔒 Hashing

```bash
# Hash a file
qt sha256 document.pdf
qt md5 archive.zip

# Hash a string
qt sha256 -s "password123"
qt sha512 -s "secret data"

# Hash with all algorithms
qt hash document.pdf
qt hash -s "test"
```

### 🎲 Generation

```bash
# UUID
qt uuid                           # 550e8400-e29b-41d4-a716-446655440000

# Password (default 16 chars)
qt pass                           # Kj8$mNp2@xLq9#Yw
qt pass 32                        # 32-character password
qt pass 24 --alpha                # Alphanumeric only

# Random bytes
qt randhex 32                     # 64 hex characters (32 bytes)
qt randb64 32                     # 32 bytes as base64
```

### 📟 Pipe Support

```bash
echo "secret" | qt b64
cat file.txt | qt sha256
curl -s https://example.com | qt md5
```

### 🖥️ GUI Mode

```bash
# Launch graphical interface
qt-gui
```

---

## 📋 Commands

| Command | Description | Example |
|---------|-------------|---------|
| `b64` | Base64 encode | `qt b64 "text"` |
| `b64d` | Base64 decode | `qt b64d "dGV4dA=="` |
| `hex` | Hex encode | `qt hex "text"` |
| `hexd` | Hex decode | `qt hexd "74657874"` |
| `url` | URL encode | `qt url "a b"` |
| `urld` | URL decode | `qt urld "a%20b"` |
| `html` | HTML encode | `qt html "<>"` |
| `htmld` | HTML decode | `qt htmld "&lt;"` |
| `md5` | MD5 hash | `qt md5 file.txt` |
| `sha1` | SHA1 hash | `qt sha1 file.txt` |
| `sha256` | SHA256 hash | `qt sha256 file.txt` |
| `sha512` | SHA512 hash | `qt sha512 file.txt` |
| `hash` | All hashes | `qt hash file.txt` |
| `uuid` | Generate UUID | `qt uuid` |
| `pass` | Generate password | `qt pass 24` |
| `randhex` | Random hex | `qt randhex 16` |
| `randb64` | Random base64 | `qt randb64 16` |
| `info` | Show version | `qt info` |

---

## 🔧 Building

### Requirements

- Rust 1.70 or later
- Cargo

### Build Commands

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# With GUI feature
cargo build --release --features gui

# Run tests
cargo test
```

---

## 📊 Technical Details

| Aspect | Detail |
|--------|--------|
| **Language** | Rust 🦀 |
| **GUI Framework** | egui/eframe |
| **Dependencies** | Minimal, audited crates |
| **Binary Size** | ~3MB (CLI), ~8MB (GUI) |
| **Offline** | 100% - no network calls |

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing`)
5. Open a Pull Request

---

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

---

<p align="center">
  Made with ❤️ by <a href="https://github.com/Brutus1066">Brutus1066</a> at <a href="https://kindware.dev">LAZYFROG-kindware.dev</a>
</p>

<p align="center">
  <a href="https://github.com/Brutus1066/QuickTransform/stargazers">⭐ Star this repo</a> if you find it useful!
</p>
