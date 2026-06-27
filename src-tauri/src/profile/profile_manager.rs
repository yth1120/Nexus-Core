use std::sync::Arc;

use parking_lot::RwLock;

use crate::event::AppEvent;
use crate::models::Profile;
use crate::runtime::RuntimeContext;
use crate::utils::{AppError, AppResult};

/// Manages profile loading and activation for the network core.
///
/// Phase 3 reads profiles from the in-memory `AppState` cache (seeded from the
/// database during boot) and tracks which profile is active. Hot-reload is a
/// mock that re-reads the cache. No profile *content* is parsed or applied.
pub struct ProfileManager {
    context: Arc<RuntimeContext>,
    active_profile_id: RwLock<Option<String>>,
}

impl ProfileManager {
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        Self {
            context,
            active_profile_id: RwLock::new(None),
        }
    }

    /// Load all known profiles (from the `AppState` cache).
    pub async fn load(&self) -> AppResult<Vec<Profile>> {
        let profiles = self.context.app_state().profiles.read().clone();
        tracing::info!("ProfileManager loaded {} profiles", profiles.len());
        Ok(profiles)
    }

    /// Activate a profile by id. Publishes `ProfileActivated`.
    pub async fn activate(&self, id: &str) -> AppResult<()> {
        let exists = self
            .context
            .app_state()
            .profiles
            .read()
            .iter()
            .any(|p| p.id == id);
        if !exists {
            return Err(AppError::NotFound(format!("Profile {id}")));
        }

        *self.active_profile_id.write() = Some(id.to_string());
        self.context.publish(AppEvent::ProfileActivated {
            profile_id: id.to_string(),
        });
        tracing::info!("Profile activated: {}", id);
        Ok(())
    }

    /// Deactivate the current profile, if any. Publishes `ProfileDeactivated`.
    pub async fn deactivate(&self) -> AppResult<()> {
        let removed = self.active_profile_id.write().take();
        if let Some(id) = removed {
            self.context.publish(AppEvent::ProfileDeactivated {
                profile_id: id.clone(),
            });
            tracing::info!("Profile deactivated: {}", id);
        }
        Ok(())
    }

    /// Hot-reload profiles (mock: re-reads the cache). No content is applied.
    pub async fn reload(&self) -> AppResult<()> {
        let count = self.context.app_state().profiles.read().len();
        tracing::info!("ProfileManager reload (mock): {} profiles", count);
        Ok(())
    }

    /// The currently-active profile id, if any.
    pub fn active(&self) -> Option<String> {
        self.active_profile_id.read().clone()
    }
}
