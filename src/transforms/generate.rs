//! Generation functions for UUIDs, passwords, and random data

use rand::Rng;
use uuid::Uuid;

/// Character sets for password generation
pub const CHARS_ALPHA: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const CHARS_NUMERIC: &str = "0123456789";
pub const CHARS_SYMBOL: &str = "!@#$%^&*()-_=+[]{}|;:,.<>?";
pub const CHARS_ALPHANUMERIC: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
pub const CHARS_ALL: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+";

/// Generate a new UUID v4
pub fn uuid_v4() -> String {
    Uuid::new_v4().to_string()
}

/// Generate a password with specified length and character set
pub fn password(length: usize, charset: &str) -> String {
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = charset.chars().collect();
    (0..length)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}

/// Generate a strong password (alphanumeric + symbols)
pub fn strong_password(length: usize) -> String {
    password(length, CHARS_ALL)
}

/// Generate an alphanumeric password
pub fn alphanum_password(length: usize) -> String {
    password(length, CHARS_ALPHANUMERIC)
}

/// Generate random hex bytes
pub fn random_hex(byte_count: usize) -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..byte_count).map(|_| rng.gen()).collect();
    hex::encode(bytes)
}

/// Generate random bytes as base64
pub fn random_base64(byte_count: usize) -> String {
    use base64::{Engine as _, engine::general_purpose::STANDARD};
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..byte_count).map(|_| rng.gen()).collect();
    STANDARD.encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid() {
        let id = uuid_v4();
        assert_eq!(id.len(), 36);
        assert!(id.contains('-'));
    }

    #[test]
    fn test_password_length() {
        let pass = strong_password(16);
        assert_eq!(pass.len(), 16);
    }

    #[test]
    fn test_random_hex() {
        let hex = random_hex(16);
        assert_eq!(hex.len(), 32); // 16 bytes = 32 hex chars
    }
}
