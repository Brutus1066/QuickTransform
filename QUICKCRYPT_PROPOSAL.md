# QuickCrypt (`qc`) — Tool Proposal

**Tagline:** Fast, offline encryption & key management. No cloud. No complexity.

**Sibling to:** [QuickTransform](https://crates.io/crates/quicktransform) (`qt`)

---

## 1. Why QuickCrypt?

Developers and sysadmins deal with encryption daily — protecting config files, generating SSH keys, inspecting JWTs, signing data, managing certificates — but the tooling is painful. `openssl` has hundreds of subcommands with inconsistent flags. Online tools require pasting sensitive data into a browser. There's no simple, offline-first CLI that "just works."

QuickCrypt applies the same philosophy that made QuickTransform successful:

| QuickTransform | QuickCrypt |
|----------------|------------|
| `qt b64 "hello"` | `qc encrypt "secret"` |
| Encoding / Hashing / Generation | Encryption / Signing / Key Management |
| Data never leaves your machine | Keys never leave your machine |
| ~1 MB binary | ~1.5 MB binary (target) |
| CLI + GUI | CLI + GUI |

---

## 2. Feature Set

### Tier 1 — Core (v1.0)

#### Symmetric Encryption / Decryption
```bash
# Encrypt a string with a passphrase (AES-256-GCM, Argon2 key derivation)
qc encrypt "my secret data" -p "passphrase"
qc decrypt <ciphertext> -p "passphrase"

# Encrypt a file (in-place or to new file)
qc encrypt secret.env -p "passphrase"
qc encrypt secret.env -p "passphrase" -o secret.env.enc

# Pipe support
echo "api_key=sk-1234" | qc encrypt -p "passphrase"
cat secret.env.enc | qc decrypt -p "passphrase"
```

#### Key Generation
```bash
# Generate AES keys
qc keygen aes128          # 128-bit key (hex)
qc keygen aes256          # 256-bit key (hex)

# Generate RSA key pairs
qc keygen rsa             # RSA-2048 (default)
qc keygen rsa 4096        # RSA-4096
qc keygen rsa -o mykey    # Outputs mykey.pem + mykey.pub.pem

# Generate Ed25519 key pairs (modern, fast)
qc keygen ed25519
qc keygen ed25519 -o signing_key

# Generate random bytes (like qt's randhex/randb64, but security-focused)
qc keygen random 32       # 32 random bytes (hex)
```

#### JWT Decode / Inspect
```bash
# Decode a JWT (no verification, just inspect)
qc jwt decode <token>

# Decode and verify with a secret
qc jwt verify <token> -s "secret"

# Decode showing header, payload, and signature status
qc jwt decode <token> --verbose
```

#### Hashing with HMAC
```bash
# HMAC signing (extends qt's hashing into authenticated territory)
qc hmac sha256 "message" -k "key"
qc hmac sha512 "message" -k "key"

# Verify an HMAC
qc hmac verify sha256 "message" -k "key" -m <mac>
```

### Tier 2 — Power Features (v1.1)

#### Asymmetric Encryption / Signing
```bash
# Encrypt with a public key
qc encrypt "secret" --pubkey recipient.pub.pem

# Decrypt with a private key
qc decrypt <ciphertext> --privkey mykey.pem

# Sign a file
qc sign document.pdf --privkey mykey.pem
qc sign document.pdf --privkey mykey.pem -o document.pdf.sig

# Verify a signature
qc verify document.pdf --sig document.pdf.sig --pubkey author.pub.pem
```

#### Certificate Inspection
```bash
# Inspect a local certificate file
qc cert inspect server.crt

# Check expiry
qc cert expiry server.crt

# Show the certificate chain
qc cert chain server.crt
```

#### TOTP / 2FA Codes
```bash
# Generate a TOTP code from a secret
qc totp "JBSWY3DPEHPK3PXP"

# Generate with custom settings
qc totp "JBSWY3DPEHPK3PXP" --digits 8 --period 60
```

### Tier 3 — Ecosystem (v1.2+)

- **Password hashing**: bcrypt, Argon2 (for verifying/generating password hashes)
- **SSH key conversion**: Convert between key formats
- **Age encryption compatibility**: Encrypt/decrypt using the `age` format
- **Checksum files**: Generate and verify `.sha256` / `.md5` checksum files
- **Secure erase**: Overwrite files before deletion

---

## 3. Architecture

Follow QuickTransform's proven structure:

```
quickcrypt/
├── Cargo.toml
├── src/
│   ├── main.rs              # CLI entry point (clap)
│   ├── gui_main.rs          # GUI entry point (eframe/egui)
│   ├── lib.rs               # Public library API
│   └── crypto/
│       ├── mod.rs            # Module exports
│       ├── symmetric.rs      # AES-256-GCM encrypt/decrypt, Argon2 KDF
│       ├── asymmetric.rs     # RSA & Ed25519 encrypt/sign/verify
│       ├── keys.rs           # Key generation (AES, RSA, Ed25519, random)
│       ├── jwt.rs            # JWT decode, inspect, verify
│       ├── hmac.rs           # HMAC sign & verify
│       ├── cert.rs           # X.509 certificate parsing & inspection
│       └── totp.rs           # TOTP generation
├── assets/
│   ├── icon.png
│   └── icon.ico
├── tests/
│   ├── symmetric_tests.rs
│   ├── keygen_tests.rs
│   ├── jwt_tests.rs
│   └── integration_tests.rs
└── scripts/
    ├── install.sh
    ├── install-mac.sh
    └── install.ps1
```

### Key Design Decisions

1. **Argon2 for key derivation** — When users provide a passphrase, derive the AES key using Argon2id (memory-hard, resistant to GPU attacks). This is the modern standard.

2. **AES-256-GCM as the default cipher** — Authenticated encryption. No need to choose between CBC/CTR/ECB. One secure default.

3. **Output format**: Encrypted output is a simple binary format:
   ```
   [1 byte: version] [16 bytes: salt] [12 bytes: nonce] [N bytes: ciphertext + tag]
   ```
   Base64-encoded when output to terminal, raw binary when output to file.

4. **No config files** — Same as QuickTransform. Zero configuration. Everything is CLI flags.

5. **Library-first** — All crypto operations exposed as a clean Rust library, just like QuickTransform's `transforms` module.

---

## 4. Dependencies

| Purpose | Crate | Notes |
|---------|-------|-------|
| CLI framework | `clap 4.x` | Same as QuickTransform |
| AES-256-GCM | `aes-gcm 0.10` | RustCrypto, audited |
| Argon2 KDF | `argon2 0.5` | RustCrypto |
| RSA | `rsa 0.9` | RustCrypto |
| Ed25519 | `ed25519-dalek 2.x` | Well-audited, fast |
| JWT | `jsonwebtoken 9.x` | De facto Rust JWT crate |
| HMAC | `hmac 0.12` + `sha2 0.10` | RustCrypto |
| X.509 certs | `x509-parser 0.16` | Certificate inspection |
| TOTP | `totp-rs 5.x` | Lightweight TOTP |
| Random | `rand 0.8` | Same as QuickTransform |
| Base64 | `base64 0.21` | Same as QuickTransform |
| GUI (optional) | `eframe 0.29` | Same as QuickTransform |

All RustCrypto crates — no OpenSSL dependency. Pure Rust. Cross-platform with no system library requirements.

---

## 5. Binary Name & Branding

| | QuickTransform | QuickCrypt |
|---|---|---|
| **Full name** | QuickTransform | QuickCrypt |
| **Binary** | `qt` | `qc` |
| **Crate name** | `quicktransform` | `quickcrypt` |
| **Tagline** | Encode. Decode. Hash. Generate. | Encrypt. Decrypt. Sign. Verify. |

The `qc` command mirrors `qt` — two letters, fast to type, memorable.

---

## 6. Competitive Landscape

| Tool | Weakness QuickCrypt solves |
|------|---------------------------|
| `openssl` | Terrible UX, hundreds of cryptic subcommands |
| `gpg` | Complex key management, steep learning curve |
| `age` | Encryption only, no JWT/HMAC/certs/TOTP |
| Online tools | Data leaves your machine |
| `jwt-cli` | JWT only, not a general crypto toolkit |
| `step` (smallstep) | Heavy, designed for PKI infrastructure |

QuickCrypt is **the simple, offline Swiss Army knife for everyday crypto tasks**.

---

## 7. Example User Workflows

### Protecting a `.env` file before sharing
```bash
qc encrypt .env -p "team-secret" -o .env.enc
# Send .env.enc via Slack/email
# Recipient runs:
qc decrypt .env.enc -p "team-secret" -o .env
```

### Debugging a JWT from an API response
```bash
curl -s https://api.example.com/token | qc jwt decode
# Shows header, payload, expiry in a readable format
```

### Generating keys for a new service
```bash
qc keygen ed25519 -o service-key
# Creates service-key.pem and service-key.pub.pem
# Use in your application
```

### Signing a release artifact
```bash
qc sign release.tar.gz --privkey release-key.pem -o release.tar.gz.sig
# Users verify:
qc verify release.tar.gz --sig release.tar.gz.sig --pubkey release-key.pub.pem
```

### Quick HMAC for a webhook
```bash
qc hmac sha256 "$WEBHOOK_BODY" -k "$WEBHOOK_SECRET"
```

---

## 8. Development Roadmap

### Phase 1: Foundation (v0.1.0)
- [ ] Project scaffolding (Cargo.toml, module structure)
- [ ] Symmetric encryption/decryption (AES-256-GCM + Argon2)
- [ ] Key generation (AES, random bytes)
- [ ] Pipe/stdin support
- [ ] File encryption/decryption
- [ ] Unit tests for all crypto operations
- [ ] CLI with clap (matching QuickTransform's UX patterns)

### Phase 2: Core Complete (v0.5.0)
- [ ] JWT decode/inspect/verify
- [ ] HMAC sign/verify
- [ ] RSA key generation
- [ ] Ed25519 key generation
- [ ] Interactive help/guide system (like `qt guide`)
- [ ] Comprehensive test suite

### Phase 3: Release (v1.0.0)
- [ ] Asymmetric encryption (RSA)
- [ ] Digital signatures (Ed25519)
- [ ] Certificate inspection
- [ ] GUI (eframe/egui, matching QuickTransform's design)
- [ ] Pre-built binaries (Windows, Linux, macOS)
- [ ] Install scripts
- [ ] Publish to crates.io
- [ ] Documentation & README

### Phase 4: Ecosystem (v1.1.0+)
- [ ] TOTP generation
- [ ] Password hashing (bcrypt, Argon2)
- [ ] SSH key format conversion
- [ ] Checksum file generation/verification

---

## 9. Cross-Promotion Strategy

Since both tools target the same audience:

- Add `qc` recommendations in `qt guide dev` ("Need encryption? Check out QuickCrypt")
- Add `qt` recommendations in `qc guide` ("Need encoding/hashing? Check out QuickTransform")
- Shared branding: "Quick*" family of developer tools
- Combined GitHub org or repo links
- Future: `quicktools` meta-crate that re-exports both libraries

---

## 10. Success Metrics

QuickTransform's crates.io traction validates the formula. QuickCrypt should aim for:

- **Same UX bar**: If someone knows `qt`, `qc` should feel instantly familiar
- **Same binary size target**: Under 2 MB for CLI
- **Same cross-platform story**: Windows, Linux, macOS from day one
- **Same zero-config philosophy**: No setup, no key stores, no daemon processes
- **Crates.io publish within Phase 2**: Get early feedback from the Rust community
