//! # QuickTransform
//!
//! Lightning-fast encoder/decoder/hasher library.
//!
//! ## LAZYFROG-kindware.dev
//! Free and open source under MIT license.
//!
//! ## Features
//! - Base64, Hex, URL, HTML encoding/decoding
//! - MD5, SHA1, SHA256, SHA512 hashing
//! - UUID and password generation

pub mod transforms;

pub use transforms::encode;
pub use transforms::hash;
pub use transforms::generate;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// App name
pub const APP_NAME: &str = "QuickTransform";

/// Branding
pub const BRAND: &str = "LAZYFROG-kindware.dev";
