use std::cmp::Ordering;
use std::sync::Arc;

use crate::utils::AppResult;

use super::core_registry::{CoreRegistry, VersionManifest};
use super::release_provider::ReleaseProvider;

pub struct VersionManager {
    registry: Arc<CoreRegistry>,
    provider: Arc<dyn ReleaseProvider>,
}

impl VersionManager {
    pub fn new(registry: Arc<CoreRegistry>, provider: Arc<dyn ReleaseProvider>) -> Self {
        Self { registry, provider }
    }

    /// List all installed versions for a core.
    pub fn get_installed_versions(&self, core: &str) -> Vec<VersionManifest> {
        self.registry.list_versions(core)
    }

    /// Query the release provider for the latest available version.
    pub async fn get_latest_version(&self, core: &str) -> AppResult<String> {
        self.provider.latest_version(core).await
    }

    /// Compare two semantic-version strings.
    ///
    /// Handles simple semver (`1.2.3`) and pre-release suffixes
    /// (`1.2.3-beta.1`). Pre-release versions sort before the release.
    pub fn compare(&self, v1: &str, v2: &str) -> Ordering {
        compare_semver(v1, v2)
    }

    /// Switch the active version for a core.
    pub fn switch_version(&self, core: &str, version: &str) -> AppResult<bool> {
        self.registry.set_current(core, version)
    }

    /// Remove an installed version and its on-disk files.
    pub fn remove_version(&self, core: &str, version: &str) -> AppResult<Option<VersionManifest>> {
        self.registry.remove_version(core, version)
    }

    /// Rollback to the previous version for a core.
    pub fn rollback(&self, core: &str) -> AppResult<String> {
        let rm = super::rollback_manager::RollbackManager::new(self.registry.clone());
        rm.rollback(core)
    }

    /// Check whether an update is available for the given core.
    /// Returns `Some(version)` if a newer version exists, `None` otherwise.
    pub async fn check_update(&self, core: &str) -> AppResult<Option<String>> {
        let latest = self.provider.latest_version(core).await?;
        let installed = self.registry.list_versions(core);

        // If nothing installed, the latest is the update.
        if installed.is_empty() {
            return Ok(Some(latest));
        }

        // Check if latest is newer than all installed versions
        let is_newer = installed
            .iter()
            .all(|v| self.compare(&latest, &v.version).is_gt());
        if is_newer {
            Ok(Some(latest))
        } else {
            Ok(None)
        }
    }
}

/// Parse a single component of a semver string.
/// Splits on `-` to handle pre-release, then parses numeric segments.
pub fn compare_semver(v1: &str, v2: &str) -> Ordering {
    // Strip leading 'v' if present
    let v1 = v1.strip_prefix('v').unwrap_or(v1);
    let v2 = v2.strip_prefix('v').unwrap_or(v2);

    // Split into release and pre-release parts
    let (rel1, pre1) = split_prerelease(v1);
    let (rel2, pre2) = split_prerelease(v2);

    // Compare release segments numerically
    let parts1: Vec<u32> = rel1
        .split('.')
        .filter_map(|p| p.parse::<u32>().ok())
        .collect();
    let parts2: Vec<u32> = rel2
        .split('.')
        .filter_map(|p| p.parse::<u32>().ok())
        .collect();

    match parts1.cmp(&parts2) {
        Ordering::Equal => {
            // Same release: a version without pre-release is newer than one with
            match (pre1, pre2) {
                (None, None) => Ordering::Equal,
                (None, Some(_)) => Ordering::Greater,
                (Some(_), None) => Ordering::Less,
                (Some(p1), Some(p2)) => p1.cmp(p2),
            }
        }
        other => other,
    }
}

fn split_prerelease(v: &str) -> (&str, Option<&str>) {
    if let Some(pos) = v.find('-') {
        (&v[..pos], Some(&v[pos + 1..]))
    } else {
        (v, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::AppError;

    /// A mock release provider for testing — returns hardcoded versions.
    struct MockReleaseProvider;
    #[async_trait::async_trait]
    impl ReleaseProvider for MockReleaseProvider {
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
            Ok("mock notes".into())
        }
        async fn checksum_url(&self, _: &str, _: &str, _: &str) -> AppResult<String> {
            Ok("https://example.com/test.sha256".into())
        }
    }

    fn test_vm() -> AppResult<VersionManager> {
        let tmp = std::env::temp_dir().join(format!("vm-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp)?;
        let registry = Arc::new(CoreRegistry::new(&tmp)?);
        Ok(VersionManager::new(registry, Arc::new(MockReleaseProvider)))
    }

    #[tokio::test]
    async fn get_latest() -> AppResult<()> {
        let vm = test_vm()?;
        let v = vm.get_latest_version("sing-box").await?;
        assert_eq!(v, "1.11.0");
        Ok(())
    }

    #[test]
    fn compare_versions() -> AppResult<()> {
        let vm = test_vm()?;
        assert_eq!(vm.compare("1.11.0", "1.10.0"), Ordering::Greater);
        assert_eq!(vm.compare("1.9.0", "1.10.0"), Ordering::Less);
        assert_eq!(vm.compare("1.0.0", "1.0.0"), Ordering::Equal);
        Ok(())
    }

    #[test]
    fn compare_with_v_prefix() {
        let vm = VersionManager::new(
            Arc::new(CoreRegistry::new(&std::env::temp_dir()).unwrap()),
            Arc::new(MockReleaseProvider),
        );
        assert_eq!(vm.compare("v1.11.0", "v1.10.0"), Ordering::Greater);
    }

    #[test]
    fn prerelease_sorts_before_release() {
        let vm = VersionManager::new(
            Arc::new(CoreRegistry::new(&std::env::temp_dir()).unwrap()),
            Arc::new(MockReleaseProvider),
        );
        assert_eq!(vm.compare("1.0.0-beta", "1.0.0"), Ordering::Less);
        assert_eq!(vm.compare("1.0.0", "1.0.0-beta"), Ordering::Greater);
    }

    #[test]
    fn compare_semver_fn() {
        assert_eq!(compare_semver("2.0.0", "1.0.0"), Ordering::Greater);
        assert_eq!(
            compare_semver("1.0.0-alpha", "1.0.0-alpha"),
            Ordering::Equal
        );
        assert_eq!(
            compare_semver("1.0.0-alpha.1", "1.0.0-alpha.2"),
            Ordering::Less
        );
    }
}
