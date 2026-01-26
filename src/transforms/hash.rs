//! Hashing functions

use md5::Md5;
use sha1::Sha1;
use sha2::{Sha256, Sha512, Digest};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

/// Hash result containing all algorithms
#[derive(Debug, Clone)]
pub struct HashResult {
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String,
}

/// Hash a string with specified algorithm
pub fn hash_string(input: &str, algorithm: &str) -> Result<String, String> {
    let bytes = input.as_bytes();
    match algorithm.to_lowercase().as_str() {
        "md5" => Ok(format!("{:x}", Md5::digest(bytes))),
        "sha1" => Ok(format!("{:x}", Sha1::digest(bytes))),
        "sha256" => Ok(format!("{:x}", Sha256::digest(bytes))),
        "sha512" => Ok(format!("{:x}", Sha512::digest(bytes))),
        _ => Err(format!("Unknown algorithm: {}", algorithm)),
    }
}

/// Hash a file with specified algorithm
pub fn hash_file(path: &Path, algorithm: &str) -> Result<String, String> {
    let file = File::open(path).map_err(|e| format!("Cannot open file: {}", e))?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).map_err(|e| format!("Read error: {}", e))?;

    match algorithm.to_lowercase().as_str() {
        "md5" => Ok(format!("{:x}", Md5::digest(&buffer))),
        "sha1" => Ok(format!("{:x}", Sha1::digest(&buffer))),
        "sha256" => Ok(format!("{:x}", Sha256::digest(&buffer))),
        "sha512" => Ok(format!("{:x}", Sha512::digest(&buffer))),
        _ => Err(format!("Unknown algorithm: {}", algorithm)),
    }
}

/// Hash with all algorithms
pub fn hash_all(input: &[u8]) -> HashResult {
    HashResult {
        md5: format!("{:x}", Md5::digest(input)),
        sha1: format!("{:x}", Sha1::digest(input)),
        sha256: format!("{:x}", Sha256::digest(input)),
        sha512: format!("{:x}", Sha512::digest(input)),
    }
}

/// Hash file with all algorithms
pub fn hash_file_all(path: &Path) -> Result<HashResult, String> {
    let file = File::open(path).map_err(|e| format!("Cannot open file: {}", e))?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).map_err(|e| format!("Read error: {}", e))?;
    Ok(hash_all(&buffer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_string() {
        let input = "hello";
        assert_eq!(
            hash_string(input, "md5").unwrap(),
            "5d41402abc4b2a76b9719d911017c592"
        );
        assert_eq!(
            hash_string(input, "sha256").unwrap(),
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }
}
