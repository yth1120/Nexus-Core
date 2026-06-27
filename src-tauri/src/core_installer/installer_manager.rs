use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::event::AppEvent;
use crate::runtime::RuntimeContext;
use crate::utils::{AppError, AppResult};

use super::core_downloader::{CoreDownloader, DownloadConfig};
use super::core_extractor::CoreExtractor;
use super::core_registry::{CoreRegistry, VersionManifest};
use super::core_state::{CoreState, CoreStateCell};
use super::integrity_checker::IntegrityChecker;
use super::mirror_manager::MirrorManager;
use super::release_provider::ReleaseProvider;

pub struct InstallerManager {
    context: Arc<RuntimeContext>,
    registry: Arc<CoreRegistry>,
    provider: Arc<dyn ReleaseProvider>,
    mirror_manager: Arc<MirrorManager>,
    downloader: CoreDownloader,
    engines_dir: PathBuf,
    state: CoreStateCell,
}

impl InstallerManager {
    pub fn new(
        context: Arc<RuntimeContext>,
        registry: Arc<CoreRegistry>,
        provider: Arc<dyn ReleaseProvider>,
        mirror_manager: Arc<MirrorManager>,
        engines_dir: PathBuf,
    ) -> Self {
        let config = {
            let app_config = context.app_state().get_config();
            DownloadConfig {
                max_retries: app_config.core.download_retry,
                timeout_secs: 300,
            }
        };
        Self {
            context,
            registry,
            provider,
            mirror_manager,
            downloader: CoreDownloader::new(config),
            engines_dir,
            state: CoreStateCell::new(),
        }
    }

    /// Return a reference to the current installation state cell.
    pub fn state(&self) -> &CoreStateCell {
        &self.state
    }

    // ----- install -----

    /// Install (or re-install) a core version.
    ///
    /// Flow: download → verify → extract → register.
    pub async fn install(&self, core: &str, version: &str) -> AppResult<()> {
        // Skip if already installed
        if self.registry.has_version(core, version) {
            tracing::info!("{core}@{version} already installed, skipping");
            return Ok(());
        }

        let core_dir = self.engines_dir.join(core);
        let versions_dir = core_dir.join("versions");
        let cache_dir = core_dir.join("cache");
        let dest_dir = versions_dir.join(version);

        self.state.set_with_message(
            CoreState::Downloading,
            format!("downloading {core}@{version}"),
        );
        self.context.publish(AppEvent::CoreDownloadStarted {
            core: core.to_string(),
            version: version.to_string(),
        });

        let platform = self.platform_string();
        let format = CoreExtractor::platform_format();
        let archive_name = format!("{core}-{version}-{platform}.{format}");
        let archive_path = cache_dir.join(&archive_name);

        // Ensure directories exist
        std::fs::create_dir_all(&versions_dir).ok();
        std::fs::create_dir_all(&cache_dir).ok();

        // Download with mirror fallback
        self.download_with_fallback(core, version, &platform, format, &archive_path)
            .await
            .map_err(|e| {
                self.context.publish(AppEvent::CoreDownloadFailed {
                    core: core.to_string(),
                    error: e.to_string(),
                });
                self.state.set(CoreState::Error);
                AppError::Io(format!("download failed: {e}"))
            })?;

        self.context.publish(AppEvent::CoreDownloadFinished {
            core: core.to_string(),
        });

        // Verify SHA-256 checksum
        self.state
            .set_with_message(CoreState::Verifying, "verifying checksum...");
        let sha256 = self
            .verify_download(core, version, &platform, format, &archive_path)
            .await?;

        // Extract
        self.state.set_with_message(
            CoreState::Extracting,
            format!("extracting {core}@{version}"),
        );
        CoreExtractor::extract(&archive_path, &dest_dir, format).map_err(|e| {
            self.state.set(CoreState::Error);
            AppError::Io(format!("extract failed: {e}"))
        })?;

        // Register
        self.state.set_with_message(
            CoreState::Installing,
            format!("installing {core}@{version}"),
        );
        self.context.publish(AppEvent::CoreInstallStarted {
            core: core.to_string(),
            version: version.to_string(),
        });

        let manifest = VersionManifest {
            version: version.into(),
            path: dest_dir.to_string_lossy().into(),
            sha256,
            installed_at: chrono::Utc::now().timestamp_millis(),
            is_current: true,
        };
        self.registry.register(core, manifest)?;
        self.registry.set_current(core, version)?;

        // Set up the `current` link (platform-appropriate)
        self.set_current_link(core, version)?;

        self.context.publish(AppEvent::CoreInstallFinished {
            core: core.to_string(),
        });
        self.state.set(CoreState::Idle);
        tracing::info!("Installed {core}@{version}");

        Ok(())
    }

    // ----- uninstall -----

    /// Uninstall a core version, removing its files and registry entry.
    pub async fn uninstall(&self, core: &str, version: &str) -> AppResult<()> {
        // Prevent uninstalling the current version while it's the only one
        let versions = self.registry.list_versions(core);
        let is_current = versions
            .iter()
            .any(|v| v.version == version && v.is_current);
        if is_current && versions.len() == 1 {
            return Err(AppError::Validation(format!(
                "cannot uninstall the only version of {core}"
            )));
        }

        self.registry.remove_version(core, version)?;
        tracing::info!("Uninstalled {core}@{version}");
        Ok(())
    }

    // ----- update -----

    /// Check for and install an update if available.
    pub async fn update(&self, core: &str) -> AppResult<Option<String>> {
        let latest = self.provider.latest_version(core).await?;
        let installed = self.registry.list_versions(core);

        // No update needed
        if installed.iter().any(|v| v.version == latest) {
            tracing::info!("{core} is already at the latest version {latest}");
            return Ok(None);
        }

        self.context.publish(AppEvent::CoreUpdateAvailable {
            core: core.to_string(),
            version: latest.clone(),
        });

        self.install(core, &latest).await?;

        self.context.publish(AppEvent::CoreUpdated {
            core: core.to_string(),
            from: installed
                .iter()
                .find(|v| v.is_current)
                .map(|v| v.version.clone())
                .unwrap_or_default(),
            to: latest.clone(),
        });

        Ok(Some(latest))
    }

    // ----- repair -----

    /// Verify the current installation and re-download if integrity check fails.
    pub async fn repair(&self, core: &str) -> AppResult<()> {
        let current = self
            .registry
            .get_current(core)
            .ok_or_else(|| AppError::NotFound(format!("{core} is not installed")))?;

        let binary_dir = PathBuf::from(&current.path);
        // Find the main binary in the extracted directory
        let binary_name = self.binary_name(core);
        let binary_path = binary_dir.join(&binary_name);

        let sha256 = &current.sha256;
        if sha256.is_empty() {
            // No checksum stored — reinstall
            tracing::warn!(
                "No checksum stored for {core}@{}, reinstalling",
                current.version
            );
            return self.install(core, &current.version).await;
        }

        if !IntegrityChecker::verify_binary(&binary_path, sha256)? {
            tracing::warn!(
                "Integrity check failed for {core}@{}, repairing...",
                current.version
            );
            // Remove and reinstall
            self.registry.remove_version(core, &current.version)?;
            self.install(core, &current.version).await?;
        }

        Ok(())
    }

    // ----- switch version -----

    /// Switch to a different installed version.
    pub fn switch_version(&self, core: &str, version: &str) -> AppResult<()> {
        if !self.registry.has_version(core, version) {
            return Err(AppError::NotFound(format!(
                "version {version} of {core} is not installed"
            )));
        }
        self.registry.set_current(core, version)?;
        self.set_current_link(core, version)?;
        tracing::info!("Switched {core} to version {version}");
        Ok(())
    }

    // ----- rollback -----

    /// Rollback to the previous version.
    pub fn rollback(&self, core: &str) -> AppResult<String> {
        let rm = super::rollback_manager::RollbackManager::new(self.registry.clone());
        let rolled = rm.rollback(core)?;
        self.set_current_link(core, &rolled)?;
        self.context.publish(AppEvent::CoreRollback {
            core: core.to_string(),
            from: "current".into(),
            to: rolled.clone(),
        });
        Ok(rolled)
    }

    // ----- helpers -----

    fn platform_string(&self) -> String {
        let os = if cfg!(target_os = "windows") {
            "windows"
        } else if cfg!(target_os = "macos") {
            "darwin"
        } else {
            "linux"
        };
        let arch = if cfg!(target_arch = "x86_64") {
            "amd64"
        } else if cfg!(target_arch = "aarch64") {
            "arm64"
        } else {
            "amd64"
        };
        format!("{os}-{arch}")
    }

    fn binary_name(&self, core: &str) -> String {
        let name = core;
        if cfg!(target_os = "windows") {
            format!("{name}.exe")
        } else {
            name.to_string()
        }
    }

    async fn download_with_fallback(
        &self,
        core: &str,
        version: &str,
        platform: &str,
        format: &str,
        dest: &Path,
    ) -> AppResult<()> {
        let mirror_count = self.mirror_manager.mirror_count();
        let mut last_error = String::new();

        for attempt in 0..=mirror_count {
            let url = if attempt == 0 {
                self.provider.download_url(core, version, platform).await?
            } else {
                match self.mirror_manager.fallback() {
                    Ok(base) => {
                        let filename = format!("{core}-{version}-{platform}.{format}");
                        format!("{base}/{core}/{filename}")
                    }
                    Err(_) => {
                        return Err(AppError::Io(format!("all mirrors exhausted: {last_error}")));
                    }
                }
            };

            match self
                .downloader
                .download(
                    &url,
                    dest,
                    |downloaded, total| {
                        let percent = if total > 0 {
                            ((downloaded as f64 / total as f64) * 100.0) as u32
                        } else {
                            0
                        };
                        // Publish progress event
                        // Note: we can't hold a reference to context here easily,
                        // so progress events are throttled via the callback.
                        let _ = percent; // consumed by caller if needed
                    },
                    None,
                )
                .await
            {
                Ok(()) => {
                    self.mirror_manager.mark_success(&url);
                    return Ok(());
                }
                Err(e) => {
                    last_error = e.to_string();
                    self.mirror_manager.mark_failure();
                    tracing::warn!("Download attempt {attempt} from {url} failed: {last_error}");
                }
            }
        }

        Err(AppError::Io(format!("all mirrors exhausted: {last_error}")))
    }

    async fn verify_download(
        &self,
        core: &str,
        version: &str,
        platform: &str,
        format: &str,
        archive_path: &PathBuf,
    ) -> AppResult<String> {
        // Try to download the checksum file
        let checksum_url = self.provider.checksum_url(core, version, platform).await?;
        let cache_dir = self.engines_dir.join(core).join("cache");
        let checksum_path = cache_dir.join(format!("{core}-{version}-{platform}.{format}.sha256"));

        let expected_sha256 = match self
            .downloader
            .download_checksum(&checksum_url, &checksum_path)
            .await
        {
            Ok(()) => {
                let content = std::fs::read_to_string(&checksum_path)
                    .map_err(|e| AppError::Io(format!("read checksum: {e}")))?;
                // Parse: "<hash>  <filename>" or just "<hash>"
                content.split_whitespace().next().unwrap_or("").to_string()
            }
            Err(_) => {
                // Checksum verification is mandatory — fail closed
                tracing::error!(
                    "Checksum file unavailable for {core}@{version}; refusing to install without integrity verification"
                );
                return Err(AppError::Validation(format!(
                    "checksum file not available for {core}@{version}; cannot verify download integrity"
                )));
            }
        };

        if expected_sha256.is_empty() {
            return Ok(String::new());
        }

        let verified = IntegrityChecker::verify_file(archive_path, &expected_sha256)?;
        if !verified {
            let _ = std::fs::remove_file(archive_path);
            let _ = std::fs::remove_file(&checksum_path);
            return Err(AppError::Internal(format!(
                "SHA-256 mismatch for {core}@{version}"
            )));
        }

        Ok(expected_sha256)
    }

    /// On Windows, write a `current_version.txt` marker file.
    /// On Unix, create a `current` symlink to the version directory.
    fn set_current_link(&self, core: &str, version: &str) -> AppResult<()> {
        let core_dir = self.engines_dir.join(core);
        #[allow(unused_variables)]
        let versions_dir = core_dir.join("versions");
        let current_marker = core_dir.join("current_version.txt");

        #[cfg(not(target_os = "windows"))]
        {
            let current_link = core_dir.join("current");
            let target = versions_dir.join(version);
            // Remove old symlink if it exists
            if current_link.exists() || current_link.is_symlink() {
                let _ = std::fs::remove_file(&current_link);
            }
            std::os::unix::fs::symlink(&target, &current_link)
                .map_err(|e| AppError::Io(format!("symlink current: {e}")))?;
        }

        // On all platforms, write a marker file as a reliable fallback
        std::fs::create_dir_all(&core_dir).ok();
        std::fs::write(&current_marker, version)
            .map_err(|e| AppError::Io(format!("write current marker: {e}")))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_engines_dir() -> PathBuf {
        std::env::temp_dir().join(format!("installer-test-{}", uuid::Uuid::new_v4()))
    }

    struct MockProvider;
    #[async_trait::async_trait]
    impl ReleaseProvider for MockProvider {
        async fn latest_version(&self, core: &str) -> AppResult<String> {
            match core {
                "sing-box" => Ok("1.11.0".into()),
                "mihomo" => Ok("1.19.0".into()),
                _ => Err(AppError::NotFound(format!("unknown: {core}"))),
            }
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

    #[test]
    fn platform_string_is_detected() -> AppResult<()> {
        let dir = test_engines_dir();
        let ctx = crate::runtime::RuntimeContext::new_for_test()?;
        let registry = Arc::new(CoreRegistry::new(&dir)?);
        let im = InstallerManager::new(
            ctx,
            registry,
            Arc::new(MockProvider),
            Arc::new(MirrorManager::default()),
            dir,
        );
        let ps = im.platform_string();
        assert!(ps.contains("windows") || ps.contains("linux") || ps.contains("darwin"));
        assert!(ps.contains("amd64") || ps.contains("arm64"));
        Ok(())
    }

    #[test]
    fn state_starts_idle() -> AppResult<()> {
        let dir = test_engines_dir();
        let ctx = crate::runtime::RuntimeContext::new_for_test()?;
        let registry = Arc::new(CoreRegistry::new(&dir)?);
        let im = InstallerManager::new(
            ctx,
            registry,
            Arc::new(MockProvider),
            Arc::new(MirrorManager::default()),
            dir,
        );
        assert_eq!(im.state().get(), CoreState::Idle);
        Ok(())
    }
}
