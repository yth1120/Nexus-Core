use crate::utils::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionState {
    pub profile_id: Option<String>,
    pub engine_type: String,
    pub traffic_mode: String,
}

impl Default for SessionState {
    fn default() -> Self {
        Self {
            profile_id: None,
            engine_type: "native".into(),
            traffic_mode: "system_proxy".into(),
        }
    }
}

pub struct SessionRecovery {
    path: PathBuf,
}

impl SessionRecovery {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn save(&self, state: &SessionState) -> AppResult<()> {
        let json = serde_json::to_string_pretty(state)
            .map_err(|e| AppError::Internal(format!("serialize session: {e}")))?;
        std::fs::write(&self.path, json).map_err(|e| AppError::Io(format!("write session: {e}")))
    }

    pub fn load(&self) -> AppResult<Option<SessionState>> {
        if !self.path.exists() {
            return Ok(None);
        }
        let data = std::fs::read_to_string(&self.path)
            .map_err(|e| AppError::Io(format!("read session: {e}")))?;
        let state: SessionState = serde_json::from_str(&data)
            .map_err(|e| AppError::Internal(format!("deserialize session: {e}")))?;
        Ok(Some(state))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn save_load_roundtrip() -> AppResult<()> {
        let p = std::env::temp_dir().join(format!("nexus-session-{}.json", uuid::Uuid::new_v4()));
        let rec = SessionRecovery::new(p.clone());
        let state = SessionState {
            profile_id: Some("p1".into()),
            engine_type: "native".into(),
            traffic_mode: "tun".into(),
        };
        rec.save(&state)?;
        let loaded = rec
            .load()?
            .ok_or_else(|| AppError::Internal("not found".into()))?;
        assert_eq!(loaded.profile_id.unwrap(), "p1");
        let _ = std::fs::remove_file(&p);
        Ok(())
    }
}
