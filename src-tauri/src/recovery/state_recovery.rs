use super::session_recovery::{SessionRecovery, SessionState};
use crate::utils::AppResult;
use std::path::Path;

pub struct StateRecovery;

impl StateRecovery {
    pub fn recover_or_reset(path: &Path) -> AppResult<SessionState> {
        let recovery = SessionRecovery::new(path.to_path_buf());
        match recovery.load() {
            Ok(Some(state)) => {
                tracing::info!("Session recovered");
                Ok(state)
            }
            Ok(None) => {
                tracing::info!("No prior session, using defaults");
                Ok(SessionState::default())
            }
            Err(e) => {
                tracing::warn!("Session recovery failed ({}), resetting", e);
                Ok(SessionState::default())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recover_missing_file_returns_default() -> AppResult<()> {
        let p = std::env::temp_dir().join(format!("nexus-missing-{}.json", uuid::Uuid::new_v4()));
        let state = StateRecovery::recover_or_reset(&p)?;
        assert_eq!(state.engine_type, "native");
        Ok(())
    }
}
