//! QuickTransform CLI
//!
//! LAZYFROG-kindware.dev | MIT License

use clap::{Parser, Subcommand};
use std::io::{self, BufRead, IsTerminal};
use std::path::PathBuf;

use quicktransform::transforms::{encode, hash, generate};
use quicktransform::{BRAND, VERSION};

// ============================================================================
// CLI STRUCTURE
// ============================================================================

#[derive(Parser)]
#[command(
    name = "qt",
    author = "LAZYFROG-kindware.dev",
    version = VERSION,
    about = "QuickTransform - Lightning-fast encoder/decoder/hasher",
    long_about = None,
    after_help = "Run 'qt guide' for comprehensive help organized by skill level.",
    styles = get_styles(),
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(clap::builder::styling::AnsiColor::Cyan.on_default().bold())
        .usage(clap::builder::styling::AnsiColor::Cyan.on_default().bold())
        .literal(clap::builder::styling::AnsiColor::Green.on_default())
        .placeholder(clap::builder::styling::AnsiColor::Yellow.on_default())
}

#[derive(Subcommand)]
enum Commands {
    // === ENCODING ===
    /// Base64 encode text
    #[command(name = "b64")]
    Base64Encode {
        /// Text to encode (or pipe via stdin)
        input: Option<String>,
    },

    /// Base64 decode text
    #[command(name = "b64d")]
    Base64Decode {
        /// Base64 string to decode (or pipe via stdin)
        input: Option<String>,
    },

    /// Hexadecimal encode text
    Hex {
        /// Text to encode (or pipe via stdin)
        input: Option<String>,
    },

    /// Hexadecimal decode text
    Hexd {
        /// Hex string to decode (or pipe via stdin)
        input: Option<String>,
    },

    /// URL encode text (percent-encoding)
    Url {
        /// Text to encode (or pipe via stdin)
        input: Option<String>,
    },

    /// URL decode text (percent-decoding)
    Urld {
        /// URL-encoded string to decode (or pipe via stdin)
        input: Option<String>,
    },

    /// HTML entity encode (escape special chars)
    Html {
        /// Text to encode (or pipe via stdin)
        input: Option<String>,
    },

    /// HTML entity decode (unescape entities)
    Htmld {
        /// HTML-encoded string to decode (or pipe via stdin)
        input: Option<String>,
    },

    // === HASHING ===
    /// MD5 hash (legacy, not for security)
    Md5 {
        /// File path to hash
        file: Option<PathBuf>,
        /// Hash a string instead of file
        #[arg(short, long)]
        string: Option<String>,
    },

    /// SHA-1 hash (legacy, not for security)
    Sha1 {
        /// File path to hash
        file: Option<PathBuf>,
        /// Hash a string instead of file
        #[arg(short, long)]
        string: Option<String>,
    },

    /// SHA-256 hash (recommended for security)
    Sha256 {
        /// File path to hash
        file: Option<PathBuf>,
        /// Hash a string instead of file
        #[arg(short, long)]
        string: Option<String>,
    },

    /// SHA-512 hash (maximum security)
    Sha512 {
        /// File path to hash
        file: Option<PathBuf>,
        /// Hash a string instead of file
        #[arg(short, long)]
        string: Option<String>,
    },

    /// Compute all hash algorithms at once
    Hash {
        /// File path to hash
        file: Option<PathBuf>,
        /// Hash a string instead of file
        #[arg(short, long)]
        string: Option<String>,
    },

    // === GENERATION ===
    /// Generate random UUID v4
    Uuid,

    /// Generate secure random password
    Pass {
        /// Password length
        #[arg(default_value = "16")]
        length: usize,
        /// Alphanumeric only (no symbols)
        #[arg(short, long)]
        alpha: bool,
    },

    /// Generate random hex bytes
    Randhex {
        /// Number of random bytes
        #[arg(default_value = "16")]
        bytes: usize,
    },

    /// Generate random base64 bytes
    Randb64 {
        /// Number of random bytes
        #[arg(default_value = "16")]
        bytes: usize,
    },

    // === HELP & INFO ===
    /// Show version and branding
    Info,

    /// Comprehensive help guide (beginner to advanced)
    Guide {
        /// Topic: all, basics, encode, hash, generate, pipe, dev
        #[arg(default_value = "all")]
        topic: String,
    },
}

// ============================================================================
// HELP SYSTEM
// ============================================================================

fn print_guide(topic: &str) {
    match topic.to_lowercase().as_str() {
        "all" => print_full_guide(),
        "basics" | "basic" | "start" | "beginner" => print_basics(),
        "encode" | "encoding" => print_encode_guide(),
        "hash" | "hashing" => print_hash_guide(),
        "generate" | "gen" => print_generate_guide(),
        "pipe" | "pipes" | "stdin" => print_pipe_guide(),
        "dev" | "developer" | "advanced" => print_dev_guide(),
        _ => {
            eprintln!("Unknown topic: {}", topic);
            eprintln!("Available: all, basics, encode, hash, generate, pipe, dev");
            std::process::exit(1);
        }
    }
}

fn print_header() {
    println!();
    println!("  ╔═══════════════════════════════════════════════════════════════╗");
    println!("  ║                                                               ║");
    println!("  ║   ▄▀▀▀▄ █  █ █ ▄▀▀▀ █  █    QuickTransform v{}            ║", VERSION);
    println!("  ║   █   █ █  █ █ █    █▄▀     {}               ║", BRAND);
    println!("  ║   ▀▀█▄▄ █  █ █ █    █▀▄     MIT License | Free Software       ║");
    println!("  ║       █ █▄▄█ █ ▀▄▄▄ █  █    Encode • Hash • Generate          ║");
    println!("  ║   ▀▄▄▄▀                                                       ║");
    println!("  ║                                                               ║");
    println!("  ╚═══════════════════════════════════════════════════════════════╝");
    println!();
}

fn print_full_guide() {
    print_header();
    print_basics();
    println!();
    print_encode_guide();
    println!();
    print_hash_guide();
    println!();
    print_generate_guide();
    println!();
    print_pipe_guide();
    println!();
    print_dev_guide();
}

fn print_basics() {
    println!("  ┌─────────────────────────────────────────────────────────────────┐");
    println!("  │  GETTING STARTED                                                │");
    println!("  └─────────────────────────────────────────────────────────────────┘");
    println!();
    println!("  QX is a fast, offline tool for encoding, hashing, and generating");
    println!("  random data. No internet required - your data stays private.");
    println!();
    println!("  QUICK START:");
    println!("  ─────────────");
    println!("    qt b64 \"hello\"              Encode 'hello' to Base64");
    println!("    qt sha256 -s \"password\"     Hash a string with SHA-256");
    println!("    qt uuid                     Generate a random UUID");
    println!("    qt pass 20                  Generate 20-char password");
    println!();
    println!("  GUIDE TOPICS:");
    println!("  ─────────────");
    println!("    qt guide basics             This introduction");
    println!("    qt guide encode             Encoding & decoding");
    println!("    qt guide hash               Hashing files & text");
    println!("    qt guide generate           Random data generation");
    println!("    qt guide pipe               Pipe & stdin usage");
    println!("    qt guide dev                Developer & advanced");
}

fn print_encode_guide() {
    println!("  ┌─────────────────────────────────────────────────────────────────┐");
    println!("  │  ENCODING & DECODING                                            │");
    println!("  └─────────────────────────────────────────────────────────────────┘");
    println!();
    println!("  BASE64 - Binary-to-text encoding (emails, data URIs, configs)");
    println!("  ─────────────────────────────────────────────────────────────");
    println!("    qt b64 \"Hello World\"        → SGVsbG8gV29ybGQ=");
    println!("    qt b64d \"SGVsbG8gV29ybGQ=\"  → Hello World");
    println!();
    println!("  HEXADECIMAL - Byte representation (debugging, colors, crypto)");
    println!("  ─────────────────────────────────────────────────────────────");
    println!("    qt hex \"ABC\"                → 414243");
    println!("    qt hexd \"414243\"            → ABC");
    println!();
    println!("  URL ENCODING - Safe URLs (query params, special chars)");
    println!("  ─────────────────────────────────────────────────────────────");
    println!("    qt url \"name=John Doe\"      → name%3DJohn%20Doe");
    println!("    qt urld \"name%3DJohn%20Doe\" → name=John Doe");
    println!();
    println!("  HTML ENTITIES - Safe HTML (prevent XSS, display code)");
    println!("  ─────────────────────────────────────────────────────────────");
    println!("    qt html \"<script>\"          → &lt;script&gt;");
    println!("    qt htmld \"&lt;script&gt;\"   → <script>");
}

fn print_hash_guide() {
    println!("  ┌─────────────────────────────────────────────────────────────────┐");
    println!("  │  HASHING                                                        │");
    println!("  └─────────────────────────────────────────────────────────────────┘");
    println!();
    println!("  Hashes create a fixed-size fingerprint of data. Same input always");
    println!("  produces same hash. Used for verification, not encryption.");
    println!();
    println!("  ALGORITHMS (strongest to legacy):");
    println!("  ─────────────────────────────────");
    println!("    SHA-512    512-bit   Maximum security, large output");
    println!("    SHA-256    256-bit   Recommended for most uses");
    println!("    SHA-1      160-bit   Legacy only, cryptographically broken");
    println!("    MD5        128-bit   Legacy only, cryptographically broken");
    println!();
    println!("  HASH A FILE:");
    println!("  ────────────");
    println!("    qt sha256 document.pdf      Single algorithm");
    println!("    qt hash document.pdf        All algorithms at once");
    println!();
    println!("  HASH A STRING:");
    println!("  ──────────────");
    println!("    qt sha256 -s \"my secret\"    Use -s flag for strings");
    println!("    qt hash -s \"my secret\"      All algorithms");
    println!();
    println!("  USE CASES:");
    println!("  ──────────");
    println!("    • Verify file integrity after download");
    println!("    • Compare files without reading contents");
    println!("    • Generate checksums for distribution");
}

fn print_generate_guide() {
    println!("  ┌─────────────────────────────────────────────────────────────────┐");
    println!("  │  GENERATION                                                     │");
    println!("  └─────────────────────────────────────────────────────────────────┘");
    println!();
    println!("  UUID - Universally Unique Identifier (v4 random)");
    println!("  ─────────────────────────────────────────────────");
    println!("    qt uuid                     → 550e8400-e29b-41d4-a716-446655440000");
    println!();
    println!("    Use for: Database IDs, session tokens, unique filenames");
    println!();
    println!("  PASSWORD - Cryptographically secure random password");
    println!("  ────────────────────────────────────────────────────");
    println!("    qt pass                     16 chars (default)");
    println!("    qt pass 32                  32 chars");
    println!("    qt pass 24 --alpha          24 chars, no symbols");
    println!();
    println!("    Default charset: a-z A-Z 0-9 !@#$%^&*()-_=+");
    println!("    Alpha charset:   a-z A-Z 0-9");
    println!();
    println!("  RANDOM BYTES - Raw entropy as hex or base64");
    println!("  ────────────────────────────────────────────");
    println!("    qt randhex 32               64 hex chars (32 bytes)");
    println!("    qt randb64 32               32 bytes as base64");
    println!();
    println!("    Use for: API keys, encryption keys, nonces");
}

fn print_pipe_guide() {
    println!("  ┌─────────────────────────────────────────────────────────────────┐");
    println!("  │  PIPES & STDIN                                                  │");
    println!("  └─────────────────────────────────────────────────────────────────┘");
    println!();
    println!("  QX reads from stdin when no argument is provided, enabling pipes.");
    println!();
    println!("  BASIC PIPES:");
    println!("  ────────────");
    println!("    echo \"hello\" | qt b64           Encode piped text");
    println!("    cat file.txt | qt sha256        Hash piped content");
    println!("    curl -s URL | qt md5            Hash downloaded data");
    println!();
    println!("  CHAINING:");
    println!("  ─────────");
    println!("    echo \"test\" | qt b64 | qt b64d  Encode then decode");
    println!("    qt uuid | qt b64                UUID as base64");
    println!();
    println!("  CLIPBOARD (platform-specific):");
    println!("  ──────────────────────────────");
    println!("    # Windows PowerShell");
    println!("    Get-Clipboard | qt b64");
    println!("    qt uuid | Set-Clipboard");
    println!();
    println!("    # Linux (xclip)");
    println!("    xclip -o | qt b64");
    println!("    qt uuid | xclip -selection clipboard");
    println!();
    println!("    # macOS");
    println!("    pbpaste | qt b64");
    println!("    qt uuid | pbcopy");
}

fn print_dev_guide() {
    println!("  ┌─────────────────────────────────────────────────────────────────┐");
    println!("  │  DEVELOPER & ADVANCED                                           │");
    println!("  └─────────────────────────────────────────────────────────────────┘");
    println!();
    println!("  EXIT CODES:");
    println!("  ───────────");
    println!("    0    Success");
    println!("    1    Error (invalid input, file not found, decode failure)");
    println!();
    println!("  SCRIPTING EXAMPLES:");
    println!("  ───────────────────");
    println!("    # Bash: Generate API key and save");
    println!("    API_KEY=$(qt randhex 32)");
    println!("    echo \"API_KEY=$API_KEY\" >> .env");
    println!();
    println!("    # PowerShell: Verify file hash");
    println!("    $hash = qt sha256 file.zip");
    println!("    if ($hash -eq $expected) {{ Write-Host \"OK\" }}");
    println!();
    println!("    # Batch process files");
    println!("    for f in *.txt; do echo \"$f: $(qt sha256 $f)\"; done");
    println!();
    println!("  LIBRARY USAGE (Rust):");
    println!("  ─────────────────────");
    println!("    use quicktransform::transforms::{{encode, hash, generate}};");
    println!();
    println!("    let encoded = encode::base64_encode(\"hello\");");
    println!("    let hashed = hash::hash_string(\"data\", \"sha256\")?;");
    println!("    let uuid = generate::uuid_v4();");
    println!();
    println!("  BUILD FROM SOURCE:");
    println!("  ──────────────────");
    println!("    git clone https://github.com/Brutus1066/quicktransform.git");
    println!("    cd quicktransform");
    println!("    cargo build --release                 # CLI only");
    println!("    cargo build --release --features gui  # With GUI");
    println!();
    println!("  GUI MODE:");
    println!("  ─────────");
    println!("    qt-gui                      Launch graphical interface");
    println!();
    println!("  ─────────────────────────────────────────────────────────────────");
    println!("  Source:  https://github.com/Brutus1066/quicktransform");
    println!("  License: MIT | {}", BRAND);
    println!("  ─────────────────────────────────────────────────────────────────");
}

// ============================================================================
// INPUT HANDLING
// ============================================================================

fn get_input(arg: Option<String>) -> String {
    if let Some(input) = arg {
        input
    } else if !io::stdin().is_terminal() {
        let stdin = io::stdin();
        let mut input = String::new();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                input.push_str(&line);
            }
        }
        input
    } else {
        eprintln!("Error: No input provided.");
        eprintln!("Usage: qt <command> \"text\" or echo \"text\" | qt <command>");
        eprintln!("Run 'qt guide basics' for help.");
        std::process::exit(1);
    }
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        // Encoding
        Commands::Base64Encode { input } => {
            Ok(encode::base64_encode(&get_input(input)))
        }
        Commands::Base64Decode { input } => {
            encode::base64_decode(&get_input(input))
        }
        Commands::Hex { input } => {
            Ok(encode::hex_encode(&get_input(input)))
        }
        Commands::Hexd { input } => {
            encode::hex_decode(&get_input(input))
        }
        Commands::Url { input } => {
            Ok(encode::url_encode(&get_input(input)))
        }
        Commands::Urld { input } => {
            encode::url_decode(&get_input(input))
        }
        Commands::Html { input } => {
            Ok(encode::html_encode(&get_input(input)))
        }
        Commands::Htmld { input } => {
            Ok(encode::html_decode(&get_input(input)))
        }

        // Hashing
        Commands::Md5 { file, string } => hash_cmd(file, string, "md5"),
        Commands::Sha1 { file, string } => hash_cmd(file, string, "sha1"),
        Commands::Sha256 { file, string } => hash_cmd(file, string, "sha256"),
        Commands::Sha512 { file, string } => hash_cmd(file, string, "sha512"),
        Commands::Hash { file, string } => hash_all_cmd(file, string),

        // Generation
        Commands::Uuid => Ok(generate::uuid_v4()),
        Commands::Pass { length, alpha } => {
            if alpha {
                Ok(generate::alphanum_password(length))
            } else {
                Ok(generate::strong_password(length))
            }
        }
        Commands::Randhex { bytes } => Ok(generate::random_hex(bytes)),
        Commands::Randb64 { bytes } => Ok(generate::random_base64(bytes)),

        // Help & Info
        Commands::Info => {
            print_header();
            println!("  Run 'qt guide' for comprehensive documentation.");
            println!("  Run 'qt --help' for command list.");
            println!();
            return;
        }
        Commands::Guide { topic } => {
            print_guide(&topic);
            return;
        }
    };

    match result {
        Ok(output) => println!("{}", output),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

// ============================================================================
// HASH HELPERS
// ============================================================================

fn hash_cmd(file: Option<PathBuf>, string: Option<String>, algo: &str) -> Result<String, String> {
    if let Some(s) = string {
        hash::hash_string(&s, algo)
    } else if let Some(path) = file {
        hash::hash_file(&path, algo)
    } else if !io::stdin().is_terminal() {
        let input = get_input(None);
        hash::hash_string(&input, algo)
    } else {
        Err(format!("Usage: qt{} <file> or qt{} -s \"string\"", algo, algo))
    }
}

fn hash_all_cmd(file: Option<PathBuf>, string: Option<String>) -> Result<String, String> {
    let result = if let Some(s) = string {
        hash::hash_all(s.as_bytes())
    } else if let Some(path) = file {
        hash::hash_file_all(&path)?
    } else if !io::stdin().is_terminal() {
        let input = get_input(None);
        hash::hash_all(input.as_bytes())
    } else {
        return Err("Usage: qt hash <file> or qt hash -s \"string\"".to_string());
    };

    Ok(format!(
        "MD5:    {}\nSHA1:   {}\nSHA256: {}\nSHA512: {}",
        result.md5, result.sha1, result.sha256, result.sha512
    ))
}
