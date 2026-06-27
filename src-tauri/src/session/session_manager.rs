use std::sync::Arc;

use parking_lot::RwLock;

use crate::event::AppEvent;
use crate::runtime::RuntimeContext;
use crate::utils::AppResult;

use super::session_state::Session;

/// Manages the lifecycle of the current network [`Session`].
///
/// Phase 3 tracks a single active session in memory and publishes
/// `SessionCreated` / `SessionDestroyed` events. There is no persistence and no
/// real connection behind a session yet.
pub struct SessionManager {
    context: Arc<RuntimeContext>,
    current: RwLock<Option<Session>>,
}

impl SessionManager {
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        Self {
            context,
            current: RwLock::new(None),
        }
    }

    /// Create and store a new session, replacing any existing one.
    /// Publishes `SessionCreated`.
    pub fn create(&self, profile_id: String, node_id: Option<String>) -> AppResult<Session> {
        let session = Session::new(profile_id, node_id);
        *self.current.write() = Some(session.clone());

        self.context.publish(AppEvent::SessionCreated {
            id: session.id.to_string(),
            profile_id: session.profile_id.clone(),
            node_id: session.node_id.clone(),
        });
        tracing::info!("Session created: {}", session.id);
        Ok(session)
    }

    /// Destroy the current session, if any. Publishes `SessionDestroyed`.
    pub fn destroy(&self) -> AppResult<()> {
        let removed = self.current.write().take();
        if let Some(session) = removed {
            self.context.publish(AppEvent::SessionDestroyed {
                id: session.id.to_string(),
            });
            tracing::info!("Session destroyed: {}", session.id);
        }
        Ok(())
    }

    /// The current session snapshot, if one is active.
    pub fn current(&self) -> Option<Session> {
        self.current.read().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runtime::RuntimeContext;
    use crate::utils::{AppError, AppResult};

    #[test]
    fn create_sets_current_and_destroy_clears_it() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let mgr = SessionManager::new(ctx);

        assert!(mgr.current().is_none());

        let created = mgr.create("profile-1".into(), Some("node-1".into()))?;
        let current = mgr
            .current()
            .ok_or_else(|| AppError::Internal("expected a current session".into()))?;
        assert_eq!(current.id, created.id);
        assert_eq!(current.profile_id, "profile-1");
        assert_eq!(current.node_id.as_deref(), Some("node-1"));

        mgr.destroy()?;
        assert!(mgr.current().is_none());
        Ok(())
    }

    #[test]
    fn each_created_session_has_a_unique_id() -> AppResult<()> {
        let ctx = RuntimeContext::new_for_test()?;
        let mgr = SessionManager::new(ctx);

        let a = mgr.create("p".into(), None)?;
        let b = mgr.create("p".into(), None)?;
        assert_ne!(a.id, b.id);
        Ok(())
    }
}
