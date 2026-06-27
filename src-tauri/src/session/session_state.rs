use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// A network session — the live binding of a profile (and optional node) to a
/// running engine instance.
///
/// Serializable (`camelCase`) so it can be returned to the frontend via the
/// `get_current_session` IPC command.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub profile_id: String,
    pub node_id: Option<String>,
}

impl Session {
    /// Create a new session with a fresh UUID and the current timestamp.
    pub fn new(profile_id: String, node_id: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            profile_id,
            node_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_session_has_unique_id_and_expected_fields() {
        let a = Session::new("profile-1".into(), Some("node-1".into()));
        let b = Session::new("profile-1".into(), None);

        assert_ne!(a.id, b.id, "each session gets a unique id");
        assert_eq!(a.profile_id, "profile-1");
        assert_eq!(a.node_id.as_deref(), Some("node-1"));
        assert_eq!(b.node_id, None);
    }
}
