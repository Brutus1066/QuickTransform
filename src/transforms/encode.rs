//! Encoding and decoding functions

use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// Base64 encode a string
pub fn base64_encode(input: &str) -> String {
    BASE64.encode(input.as_bytes())
}

/// Base64 decode a string
pub fn base64_decode(input: &str) -> Result<String, String> {
    BASE64
        .decode(input.trim())
        .map_err(|e| format!("Base64 decode error: {}", e))
        .and_then(|bytes| {
            String::from_utf8(bytes).map_err(|e| format!("UTF-8 error: {}", e))
        })
}

/// Hex encode a string
pub fn hex_encode(input: &str) -> String {
    hex::encode(input.as_bytes())
}

/// Hex decode a string
pub fn hex_decode(input: &str) -> Result<String, String> {
    hex::decode(input.trim())
        .map_err(|e| format!("Hex decode error: {}", e))
        .and_then(|bytes| {
            String::from_utf8(bytes).map_err(|e| format!("UTF-8 error: {}", e))
        })
}

/// URL encode a string
pub fn url_encode(input: &str) -> String {
    urlencoding::encode(input).into_owned()
}

/// URL decode a string
pub fn url_decode(input: &str) -> Result<String, String> {
    urlencoding::decode(input.trim())
        .map(|s| s.into_owned())
        .map_err(|e| format!("URL decode error: {}", e))
}

/// HTML entity encode
pub fn html_encode(input: &str) -> String {
    html_escape::encode_text(input).into_owned()
}

/// HTML entity decode
pub fn html_decode(input: &str) -> String {
    html_escape::decode_html_entities(input).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_roundtrip() {
        let input = "Hello, World!";
        let encoded = base64_encode(input);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_hex_roundtrip() {
        let input = "Test123";
        let encoded = hex_encode(input);
        let decoded = hex_decode(&encoded).unwrap();
        assert_eq!(input, decoded);
    }

    #[test]
    fn test_url_roundtrip() {
        let input = "hello world & more";
        let encoded = url_encode(input);
        let decoded = url_decode(&encoded).unwrap();
        assert_eq!(input, decoded);
    }
}
