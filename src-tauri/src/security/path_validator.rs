use std::path::{Path, PathBuf};

use crate::utils::{AppError, AppResult};

/// Validates file paths for security issues (traversal attacks, injection).
pub struct PathValidator;

impl PathValidator {
    /// Check `path` for traversal attacks. The resolved path must stay
    /// within `base_dir`. Returns the canonicalized safe path on success.
    pub fn validate(path: &str, base_dir: &Path) -> AppResult<PathBuf> {
        // 1. Reject null bytes
        if path.contains('\0') {
            return Err(AppError::Validation("path contains null byte".into()));
        }

        // 2. Parse the path
        let candidate = Path::new(path);

        // 3. Reject absolute paths when a base_dir is specified
        if candidate.is_absolute() {
            return Err(AppError::Validation(format!(
                "absolute path not allowed: {path}"
            )));
        }

        // 4. Build the full path relative to the base
        let full = base_dir.join(candidate);

        // 5. Canonicalize (resolves symlinks and `..` segments)
        let canonical = full
            .canonicalize()
            .map_err(|e| AppError::Validation(format!("cannot resolve path '{path}': {e}")))?;

        // 6. Verify the result is within the base directory
        // Reject if base_dir cannot be canonicalized — falling back to a
        // non-canonicalized path would allow symlink bypass.
        let base_canonical = base_dir
            .canonicalize()
            .map_err(|e| AppError::Validation(format!("cannot resolve base directory: {e}")))?;
        if !canonical.starts_with(&base_canonical) {
            return Err(AppError::Validation(format!(
                "path traversal detected: {path} resolves outside {base_dir:?}"
            )));
        }

        Ok(canonical)
    }

    /// Simple check: does a path contain obvious traversal sequences?
    pub fn has_traversal_markers(path: &str) -> bool {
        path.contains("..")
    }

    /// Check if a file name is safe (no traversal, no special system names).
    pub fn is_safe_filename(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        if name.contains("..") || name.contains('/') || name.contains('\\') {
            return false;
        }
        if name.contains('\0') {
            return false;
        }
        // Block Windows reserved names
        let lower = name.to_lowercase();
        let reserved = [
            "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7",
            "com8", "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
        ];
        let stem = lower.split('.').next().unwrap_or(&lower);
        if reserved.contains(&stem) {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_traversal_markers() {
        assert!(PathValidator::has_traversal_markers("../etc/passwd"));
        assert!(PathValidator::has_traversal_markers("..\\windows"));
        assert!(!PathValidator::has_traversal_markers("config.toml"));
    }

    #[test]
    fn null_byte_rejected() {
        let tmp = std::env::temp_dir();
        let result = PathValidator::validate("file\0.txt", &tmp);
        assert!(result.is_err());
    }

    #[test]
    fn absolute_path_rejected() {
        let tmp = std::env::temp_dir();
        let result = PathValidator::validate("/etc/passwd", &tmp);
        assert!(result.is_err());
    }

    #[test]
    fn traversal_prevented() {
        let tmp = std::env::temp_dir();
        let result = PathValidator::validate("../../../etc/passwd", &tmp);
        assert!(result.is_err());
    }

    #[test]
    fn safe_filename_checks() {
        assert!(PathValidator::is_safe_filename("config.toml"));
        assert!(PathValidator::is_safe_filename("sing-box.exe"));
        assert!(!PathValidator::is_safe_filename(""));
        assert!(!PathValidator::is_safe_filename("../escape.bin"));
        assert!(!PathValidator::is_safe_filename("evil\0.bin"));
        assert!(!PathValidator::is_safe_filename("CON"));
        assert!(!PathValidator::is_safe_filename("NUL.txt"));
    }

    #[test]
    fn validate_good_path() -> crate::utils::AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("pv-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp)?;
        let result = PathValidator::validate("test.txt", &tmp);
        assert!(result.is_ok());
        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }
}
