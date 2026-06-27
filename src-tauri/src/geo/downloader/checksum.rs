use std::path::Path;

use crate::core_installer::integrity_checker::IntegrityChecker;
use crate::utils::{AppError, AppResult};

/// Thin wrapper around `IntegrityChecker` for geo-specific checksum operations.
pub struct ChecksumVerifier;

impl ChecksumVerifier {
    /// Verify a file against its expected SHA-256 hex digest.
    pub fn verify(path: &Path, expected_sha256: &str) -> AppResult<bool> {
        IntegrityChecker::verify_file(path, expected_sha256)
    }

    /// Parse a checksum file (format: `<hash>  <filename>` or just `<hash>`)
    /// and return the hash string.
    pub fn parse_checksum_file(path: &Path) -> AppResult<String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AppError::Io(format!("read checksum file: {e}")))?;
        let hash = content.split_whitespace().next().unwrap_or("").to_string();
        if hash.is_empty() {
            return Err(AppError::Io("empty checksum file".into()));
        }
        Ok(hash)
    }
}
