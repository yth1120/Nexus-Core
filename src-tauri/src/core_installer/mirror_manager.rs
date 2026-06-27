use parking_lot::RwLock;

use crate::utils::{AppError, AppResult};

pub struct MirrorManager {
    mirrors: Vec<String>,
    /// Index of the last mirror that succeeded, used for sticky selection.
    last_success: RwLock<Option<usize>>,
}

impl MirrorManager {
    /// Create a new mirror manager with the given list of mirror base URLs.
    /// At least one mirror must be provided.
    pub fn new(mirrors: Vec<String>) -> Self {
        Self {
            mirrors,
            last_success: RwLock::new(None),
        }
    }

    /// Return the best mirror. Prefers the last-known-good mirror; falls back
    /// to the first mirror in the list.
    pub fn select_best_mirror(&self) -> AppResult<String> {
        let sticky = self.last_success.read();
        if let Some(idx) = *sticky {
            if idx < self.mirrors.len() {
                return Ok(self.mirrors[idx].clone());
            }
        }
        self.mirrors
            .first()
            .cloned()
            .ok_or_else(|| AppError::Internal("no mirrors configured".into()))
    }

    /// Return the next mirror after the one returned by `select_best_mirror`.
    /// If there is no next mirror, returns an error.
    pub fn fallback(&self) -> AppResult<String> {
        let best_idx = self.last_success.read().unwrap_or(0);
        let next = best_idx + 1;
        self.mirrors
            .get(next)
            .cloned()
            .ok_or_else(|| AppError::NotFound("no fallback mirror available".into()))
    }

    /// Mark a mirror URL as having succeeded. Future `select_best_mirror`
    /// calls will prefer it until the next failure.
    pub fn mark_success(&self, url: &str) {
        if let Some(idx) = self.mirrors.iter().position(|m| m == url) {
            *self.last_success.write() = Some(idx);
        }
    }

    /// Mark the current best mirror as failed, forcing the next
    /// `select_best_mirror` to pick a different one.
    pub fn mark_failure(&self) {
        let mut sticky = self.last_success.write();
        if let Some(idx) = *sticky {
            let next = idx + 1;
            *sticky = if next < self.mirrors.len() {
                Some(next)
            } else {
                None
            };
        }
    }

    /// Return the number of configured mirrors.
    pub fn mirror_count(&self) -> usize {
        self.mirrors.len()
    }
}

impl Default for MirrorManager {
    fn default() -> Self {
        Self::new(vec!["https://github.com".into()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_best_returns_first() -> AppResult<()> {
        let m = MirrorManager::new(vec![
            "https://a.example.com".into(),
            "https://b.example.com".into(),
        ]);
        assert_eq!(m.select_best_mirror()?, "https://a.example.com");
        Ok(())
    }

    #[test]
    fn fallback_returns_next() -> AppResult<()> {
        let m = MirrorManager::new(vec![
            "https://a.example.com".into(),
            "https://b.example.com".into(),
        ]);
        assert_eq!(m.fallback()?, "https://b.example.com");
        Ok(())
    }

    #[test]
    fn fallback_fails_with_one_mirror() {
        let m = MirrorManager::new(vec!["https://only.example.com".into()]);
        assert!(m.fallback().is_err());
    }

    #[test]
    fn mark_success_changes_sticky() -> AppResult<()> {
        let m = MirrorManager::new(vec![
            "https://a.example.com".into(),
            "https://b.example.com".into(),
        ]);
        m.mark_success("https://b.example.com");
        assert_eq!(m.select_best_mirror()?, "https://b.example.com");
        Ok(())
    }

    #[test]
    fn mark_failure_moves_to_next() -> AppResult<()> {
        let m = MirrorManager::new(vec![
            "https://a.example.com".into(),
            "https://b.example.com".into(),
            "https://c.example.com".into(),
        ]);
        m.mark_success("https://a.example.com");
        m.mark_failure();
        assert_eq!(m.select_best_mirror()?, "https://b.example.com");
        Ok(())
    }

    #[test]
    fn mark_failure_at_end_resets() -> AppResult<()> {
        let m = MirrorManager::new(vec![
            "https://a.example.com".into(),
            "https://b.example.com".into(),
        ]);
        m.mark_success("https://b.example.com");
        m.mark_failure();
        // After failure at last mirror, select_best falls back to first
        assert_eq!(m.select_best_mirror()?, "https://a.example.com");
        Ok(())
    }
}
