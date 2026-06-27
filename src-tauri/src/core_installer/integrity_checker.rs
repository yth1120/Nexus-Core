use std::io::Read;
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::utils::{AppError, AppResult};

pub struct IntegrityChecker;

impl IntegrityChecker {
    /// Verify that `path` has the expected SHA-256 hex digest.
    /// Returns `Ok(true)` on match, `Ok(false)` on mismatch.
    pub fn verify_file(path: &Path, expected_sha256: &str) -> AppResult<bool> {
        let hash = Self::compute_sha256(path)?;
        Ok(hash == expected_sha256)
    }

    /// Verify that the binary at `path` is non-empty and has the expected hash.
    /// An empty or missing file is treated as a failure.
    pub fn verify_binary(path: &Path, expected_sha256: &str) -> AppResult<bool> {
        if !path.exists() {
            return Ok(false);
        }
        let meta = std::fs::metadata(path).map_err(|e| AppError::Io(format!("metadata: {e}")))?;
        if meta.len() == 0 {
            return Ok(false);
        }
        Self::verify_file(path, expected_sha256)
    }

    /// Verify a release: check the binary and optionally a checksum file.
    /// If `sha256_path` is provided, its content is read and compared against
    /// `expected_sha256` as well.
    pub fn verify_release(
        binary_path: &Path,
        expected_sha256: &str,
        sha256_path: Option<&Path>,
    ) -> AppResult<bool> {
        // Verify binary
        if !Self::verify_binary(binary_path, expected_sha256)? {
            return Ok(false);
        }

        // If an external checksum file was provided, verify its content
        if let Some(cs_path) = sha256_path {
            if cs_path.exists() {
                let cs_content = std::fs::read_to_string(cs_path)
                    .map_err(|e| AppError::Io(format!("read checksum file: {e}")))?;
                // A typical checksum file has the format: "<hash>  <filename>"
                let cs_hash = cs_content.split_whitespace().next().unwrap_or(&cs_content);
                if cs_hash != expected_sha256 {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }

    /// Compute the SHA-256 hex digest of a file, reading it in chunks to
    /// avoid loading the whole file into memory.
    pub fn compute_sha256(path: &Path) -> AppResult<String> {
        let file =
            std::fs::File::open(path).map_err(|e| AppError::Io(format!("open for sha256: {e}")))?;
        let mut reader = std::io::BufReader::new(file);
        let mut hasher = Sha256::new();
        let mut buf = [0u8; 8192];
        loop {
            let n = reader
                .read(&mut buf)
                .map_err(|e| AppError::Io(format!("read for sha256: {e}")))?;
            if n == 0 {
                break;
            }
            hasher.update(&buf[..n]);
        }
        Ok(format!("{:x}", hasher.finalize()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_sha256() -> AppResult<()> {
        let p = std::env::temp_dir().join("test-sha256.bin");
        std::fs::write(&p, b"hello world")?;
        let expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
        assert!(IntegrityChecker::verify_file(&p, expected)?);
        assert!(!IntegrityChecker::verify_file(&p, "deadbeef")?);
        let _ = std::fs::remove_file(&p);
        Ok(())
    }

    #[test]
    fn verify_binary_rejects_empty() -> AppResult<()> {
        let p = std::env::temp_dir().join("empty.bin");
        std::fs::write(&p, b"")?;
        assert!(!IntegrityChecker::verify_binary(&p, "any")?);
        let _ = std::fs::remove_file(&p);
        Ok(())
    }

    #[test]
    fn verify_binary_rejects_missing() -> AppResult<()> {
        assert!(!IntegrityChecker::verify_binary(
            Path::new("/nonexistent/path/bin"),
            "any"
        )?);
        Ok(())
    }

    #[test]
    fn verify_release_with_checksum_file() -> AppResult<()> {
        let dir = std::env::temp_dir().join(format!("verify-rel-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&dir)?;

        let bin = dir.join("binary");
        std::fs::write(&bin, b"hello world")?;
        let expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";

        let cs = dir.join("binary.sha256");
        std::fs::write(&cs, format!("{expected}  binary"))?;

        assert!(IntegrityChecker::verify_release(&bin, expected, Some(&cs))?);

        let _ = std::fs::remove_dir_all(&dir);
        Ok(())
    }

    #[test]
    fn streaming_hash_matches_simple_read() -> AppResult<()> {
        let p = std::env::temp_dir().join("stream-hash.bin");
        // Write enough data to span multiple 8 KiB buffers
        let data = vec![0xABu8; 20000];
        std::fs::write(&p, &data)?;

        let h1 = IntegrityChecker::compute_sha256(&p)?;
        // Compare with one-shot hash
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let h2 = format!("{:x}", hasher.finalize());
        assert_eq!(h1, h2);
        let _ = std::fs::remove_file(&p);
        Ok(())
    }
}
