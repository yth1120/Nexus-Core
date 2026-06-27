use std::sync::Arc;

use crate::utils::{AppError, AppResult};

use super::core_registry::CoreRegistry;

pub struct RollbackManager {
    registry: Arc<CoreRegistry>,
}

impl RollbackManager {
    pub fn new(registry: Arc<CoreRegistry>) -> Self {
        Self { registry }
    }

    /// Rollback to the previous version (the one immediately before the
    /// current version in registration order).
    pub fn rollback(&self, core: &str) -> AppResult<String> {
        let versions = self.registry.list_versions(core);
        let current = versions
            .iter()
            .position(|v| v.is_current)
            .ok_or_else(|| AppError::NotFound("no current version".into()))?;
        if current == 0 {
            return Err(AppError::NotFound(
                "no previous version to rollback to".into(),
            ));
        }
        let prev = &versions[current - 1];
        self.registry.set_current(core, &prev.version)?;
        Ok(prev.version.clone())
    }

    /// Rollback to a specific named version.
    pub fn rollback_to_specific(&self, core: &str, target_version: &str) -> AppResult<()> {
        if !self.registry.has_version(core, target_version) {
            return Err(AppError::NotFound(format!(
                "version {target_version} of {core} is not installed"
            )));
        }
        self.registry.set_current(core, target_version)?;
        Ok(())
    }

    /// List all versions that are available rollback targets (everything
    /// except the current version).
    pub fn list_rollback_targets(&self, core: &str) -> Vec<String> {
        self.registry
            .list_versions(core)
            .iter()
            .filter(|v| !v.is_current)
            .map(|v| v.version.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_installer::VersionManifest;

    fn test_rm() -> AppResult<RollbackManager> {
        let tmp = std::env::temp_dir().join(format!("rb-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp)?;
        let r = Arc::new(CoreRegistry::new(&tmp)?);
        Ok(RollbackManager::new(r))
    }

    #[test]
    fn rollback_switches_to_previous() -> AppResult<()> {
        let rm = test_rm()?;
        rm.registry.register(
            "sing-box",
            VersionManifest {
                version: "1.10.0".into(),
                path: "/a".into(),
                sha256: "x".into(),
                installed_at: 0,
                is_current: false,
            },
        )?;
        rm.registry.register(
            "sing-box",
            VersionManifest {
                version: "1.11.0".into(),
                path: "/b".into(),
                sha256: "y".into(),
                installed_at: 0,
                is_current: true,
            },
        )?;
        let rolled = rm.rollback("sing-box")?;
        assert_eq!(rolled, "1.10.0");
        assert!(rm.registry.get_current("sing-box").unwrap().is_current);
        assert_eq!(
            rm.registry.get_current("sing-box").unwrap().version,
            "1.10.0"
        );
        Ok(())
    }

    #[test]
    fn rollback_to_specific_works() -> AppResult<()> {
        let rm = test_rm()?;
        rm.registry.register(
            "mihomo",
            VersionManifest {
                version: "1.18.0".into(),
                path: "/a".into(),
                sha256: "x".into(),
                installed_at: 0,
                is_current: false,
            },
        )?;
        rm.registry.register(
            "mihomo",
            VersionManifest {
                version: "1.19.0".into(),
                path: "/b".into(),
                sha256: "y".into(),
                installed_at: 0,
                is_current: true,
            },
        )?;
        rm.rollback_to_specific("mihomo", "1.18.0")?;
        assert_eq!(rm.registry.get_current("mihomo").unwrap().version, "1.18.0");
        Ok(())
    }

    #[test]
    fn rollback_to_missing_version_fails() -> AppResult<()> {
        let rm = test_rm()?;
        let result = rm.rollback_to_specific("sing-box", "9.9.9");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn rollback_at_first_version_fails() -> AppResult<()> {
        let rm = test_rm()?;
        rm.registry.register(
            "xray",
            VersionManifest {
                version: "1.0.0".into(),
                path: "/a".into(),
                sha256: "x".into(),
                installed_at: 0,
                is_current: true,
            },
        )?;
        let result = rm.rollback("xray");
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn list_rollback_targets_excludes_current() -> AppResult<()> {
        let rm = test_rm()?;
        rm.registry.register(
            "sing-box",
            VersionManifest {
                version: "1.10.0".into(),
                path: "/a".into(),
                sha256: "x".into(),
                installed_at: 0,
                is_current: false,
            },
        )?;
        rm.registry.register(
            "sing-box",
            VersionManifest {
                version: "1.11.0".into(),
                path: "/b".into(),
                sha256: "y".into(),
                installed_at: 0,
                is_current: true,
            },
        )?;
        let targets = rm.list_rollback_targets("sing-box");
        assert_eq!(targets.len(), 1);
        assert_eq!(targets[0], "1.10.0");
        Ok(())
    }
}
