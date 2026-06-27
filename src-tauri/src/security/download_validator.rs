use std::path::Path;

use crate::core_installer::integrity_checker::IntegrityChecker;
use crate::utils::{AppError, AppResult};

use super::path_validator::PathValidator;

/// A single issue found during validation.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationIssue {
    pub severity: String, // "error" | "warning"
    pub message: String,
}

/// Result of validating a downloaded file.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadValidation {
    pub passed: bool,
    pub issues: Vec<ValidationIssue>,
}

/// Validates downloaded files for integrity, naming safety, and size.
pub struct DownloadValidator;

impl DownloadValidator {
    /// Validate a downloaded file:
    /// 1. Check the file name is safe
    /// 2. Check the file exists and is non-empty
    /// 3. Verify SHA-256 checksum if provided
    /// 4. Check file size is within reasonable bounds
    pub fn validate(
        path: &Path,
        expected_hash: Option<&str>,
        max_size_mb: Option<u64>,
    ) -> AppResult<DownloadValidation> {
        let mut issues = Vec::new();

        // 1. File name safety
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if !PathValidator::is_safe_filename(name) {
                issues.push(ValidationIssue {
                    severity: "error".into(),
                    message: format!("unsafe file name: {name}"),
                });
            }
        }

        // 2. Existence and non-empty
        if !path.exists() {
            issues.push(ValidationIssue {
                severity: "error".into(),
                message: "file does not exist".into(),
            });
        } else {
            let meta =
                std::fs::metadata(path).map_err(|e| AppError::Io(format!("metadata: {e}")))?;

            if meta.len() == 0 {
                issues.push(ValidationIssue {
                    severity: "error".into(),
                    message: "file is empty".into(),
                });
            }

            // 3. Size check
            if let Some(max_mb) = max_size_mb {
                let max_bytes = max_mb * 1024 * 1024;
                if meta.len() > max_bytes {
                    issues.push(ValidationIssue {
                        severity: "warning".into(),
                        message: format!(
                            "file size {}MB exceeds limit {}MB",
                            meta.len() / (1024 * 1024),
                            max_mb
                        ),
                    });
                }
            }

            // 4. Integrity check
            if let Some(hash) = expected_hash {
                if !hash.is_empty() {
                    match IntegrityChecker::verify_file(path, hash) {
                        Ok(true) => {} // OK
                        Ok(false) => {
                            issues.push(ValidationIssue {
                                severity: "error".into(),
                                message: "SHA-256 checksum mismatch".into(),
                            });
                        }
                        Err(e) => {
                            issues.push(ValidationIssue {
                                severity: "error".into(),
                                message: format!("checksum verification failed: {e}"),
                            });
                        }
                    }
                }
            }
        }

        let has_errors = issues.iter().any(|i| i.severity == "error");
        Ok(DownloadValidation {
            passed: !has_errors,
            issues,
        })
    }

    /// Check extracted archive contents for zip slip attacks.
    /// Ensures no extracted file escapes the destination directory.
    pub fn check_zip_slip(entry_path: &str, dest_dir: &Path) -> AppResult<()> {
        let full = dest_dir.join(entry_path);
        // Reject if paths cannot be canonicalized — a non-canonicalized
        // fallback would allow symlink bypass of the containment check.
        let canonical = full
            .canonicalize()
            .map_err(|e| AppError::Validation(format!("cannot resolve entry path: {e}")))?;
        let dest_canonical = dest_dir
            .canonicalize()
            .map_err(|e| AppError::Validation(format!("cannot resolve dest directory: {e}")))?;

        if !canonical.starts_with(&dest_canonical) {
            return Err(AppError::Validation(format!(
                "zip slip detected: '{entry_path}' would extract outside {dest_dir:?}"
            )));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_good_file() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("dv-{}", uuid::Uuid::new_v4()));
        let file = tmp.join("test.bin");
        std::fs::write(&file, b"hello world")?;

        let result = DownloadValidator::validate(&file, None, Some(1))?;
        assert!(result.passed);
        let _ = std::fs::remove_file(&file);
        Ok(())
    }

    #[test]
    fn detects_missing_file() -> AppResult<()> {
        let result = DownloadValidator::validate(Path::new("/nonexistent/file.bin"), None, None)?;
        assert!(!result.passed);
        Ok(())
    }

    #[test]
    fn detects_size_exceeded() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("dv-size-{}", uuid::Uuid::new_v4()));
        let file = tmp.join("large.bin");
        std::fs::write(&file, vec![0u8; 100])?;

        let result = DownloadValidator::validate(&file, None, Some(0))?; // 0MB max
        assert!(!result.passed || result.issues.iter().any(|i| i.severity == "warning"));
        let _ = std::fs::remove_file(&file);
        Ok(())
    }

    #[test]
    fn detects_checksum_mismatch() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("dv-cs-{}", uuid::Uuid::new_v4()));
        let file = tmp.join("test.bin");
        std::fs::write(&file, b"content")?;

        let result = DownloadValidator::validate(
            &file,
            Some("deadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"),
            None,
        )?;
        assert!(!result.passed);
        let _ = std::fs::remove_file(&file);
        Ok(())
    }

    #[test]
    fn zip_slip_detection() {
        assert!(DownloadValidator::check_zip_slip(
            "../../../etc/passwd",
            Path::new("/tmp/extract")
        )
        .is_err());

        assert!(
            DownloadValidator::check_zip_slip("config.json", Path::new("/tmp/extract")).is_ok()
        );
    }
}
