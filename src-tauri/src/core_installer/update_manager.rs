use std::sync::Arc;
use std::time::Duration;

use crate::event::AppEvent;
use crate::runtime::RuntimeContext;
use crate::utils::AppResult;

use super::installer_manager::InstallerManager;
use super::version_manager::VersionManager;

pub struct UpdateManager {
    context: Arc<RuntimeContext>,
    version_manager: Arc<VersionManager>,
    installer_manager: Arc<InstallerManager>,
}

impl UpdateManager {
    pub fn new(
        context: Arc<RuntimeContext>,
        version_manager: Arc<VersionManager>,
        installer_manager: Arc<InstallerManager>,
    ) -> Self {
        Self {
            context,
            version_manager,
            installer_manager,
        }
    }

    /// Check all supported cores for updates. Returns a list of
    /// `"core@version"` strings for cores that have updates available.
    pub async fn check_for_updates(&self) -> AppResult<Vec<String>> {
        let mut updates = Vec::new();
        for core in &["sing-box", "mihomo"] {
            match self.version_manager.check_update(core).await {
                Ok(Some(latest)) => {
                    updates.push(format!("{core}@{}", latest));
                    self.context.publish(AppEvent::CoreUpdateAvailable {
                        core: core.to_string(),
                        version: latest.clone(),
                    });
                    tracing::info!("Update available: {core}@{latest}");
                }
                Ok(None) => {
                    tracing::debug!("{core} is up to date");
                }
                Err(e) => {
                    tracing::warn!("Failed to check updates for {core}: {e}");
                }
            }
        }
        Ok(updates)
    }

    /// Check for updates and install them for all cores that have updates.
    pub async fn run_update(&self) -> AppResult<Vec<String>> {
        let updates = self.check_for_updates().await?;
        let mut installed = Vec::new();

        for update_str in &updates {
            let parts: Vec<&str> = update_str.splitn(2, '@').collect();
            if parts.len() == 2 {
                let core = parts[0];
                match self.installer_manager.update(core).await {
                    Ok(Some(version)) => {
                        installed.push(format!("{core}@{version}"));
                    }
                    Ok(None) => {
                        tracing::info!("{core} already up to date");
                    }
                    Err(e) => {
                        tracing::error!("Failed to update {core}: {e}");
                    }
                }
            }
        }

        Ok(installed)
    }

    /// Start a background task that periodically checks for updates.
    /// The check interval is taken from the core config.
    ///
    /// Returns a `tokio::task::JoinHandle` that can be aborted to stop
    /// the periodic checks.
    pub fn start_periodic_check(self: &Arc<Self>) -> tokio::task::JoinHandle<()> {
        let this = self.clone();
        let interval_secs = {
            let config = this.context.app_state().get_config();
            config.core.check_interval
        };

        tokio::spawn(async move {
            // Initial check after 30 seconds to let the app settle
            tokio::time::sleep(Duration::from_secs(30)).await;

            loop {
                let _ = this.check_for_updates().await;
                tokio::time::sleep(Duration::from_secs(interval_secs)).await;
            }
        })
    }
}

impl Clone for UpdateManager {
    fn clone(&self) -> Self {
        Self {
            context: self.context.clone(),
            version_manager: self.version_manager.clone(),
            installer_manager: self.installer_manager.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_installer::{
        release_provider::ReleaseProvider, CoreRegistry, InstallerManager, MirrorManager,
        VersionManager,
    };

    /// A mock provider that returns a very new version to trigger updates.
    struct MockAlwaysNew;
    #[async_trait::async_trait]
    impl ReleaseProvider for MockAlwaysNew {
        async fn latest_version(&self, core: &str) -> AppResult<String> {
            Ok(format!("99.0.0-{core}"))
        }
        async fn download_url(&self, _: &str, _: &str, _: &str) -> AppResult<String> {
            Ok("https://example.com/test.zip".into())
        }
        async fn release_notes(&self, _: &str, _: &str) -> AppResult<String> {
            Ok("mock".into())
        }
        async fn checksum_url(&self, _: &str, _: &str, _: &str) -> AppResult<String> {
            Ok("https://example.com/test.sha256".into())
        }
    }

    #[tokio::test]
    async fn check_for_updates_detects_new_version() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("um-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp)?;

        let ctx = crate::runtime::RuntimeContext::new_for_test()?;
        let registry = Arc::new(CoreRegistry::new(&tmp)?);
        let provider: Arc<dyn ReleaseProvider> = Arc::new(MockAlwaysNew);
        let vm = Arc::new(VersionManager::new(registry.clone(), provider.clone()));
        let im = Arc::new(InstallerManager::new(
            ctx.clone(),
            registry,
            provider,
            Arc::new(MirrorManager::default()),
            tmp.clone(),
        ));
        let um = UpdateManager::new(ctx, vm, im);

        let updates = um.check_for_updates().await?;
        // Both cores should have updates since MockAlwaysNew returns 99.0.0
        assert!(!updates.is_empty());
        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }
}
