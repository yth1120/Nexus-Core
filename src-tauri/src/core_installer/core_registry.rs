use std::collections::HashMap;
use std::path::{Path, PathBuf};

use parking_lot::RwLock;

use crate::utils::{AppError, AppResult};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionManifest {
    pub version: String,
    pub path: String,
    pub sha256: String,
    pub installed_at: i64,
    pub is_current: bool,
}

/// Serializable form of the entire registry for disk persistence.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct RegistryData {
    cores: HashMap<String, Vec<VersionManifest>>,
}

pub struct CoreRegistry {
    cores: RwLock<HashMap<String, Vec<VersionManifest>>>,
    registry_path: PathBuf,
}

impl CoreRegistry {
    /// Create a new registry. If a `registry.json` file exists at
    /// `engines_dir/registry.json`, it is loaded; otherwise an empty
    /// registry is created.
    pub fn new(engines_dir: &Path) -> AppResult<Self> {
        let registry_path = engines_dir.join("registry.json");
        let cores = if registry_path.exists() {
            let data = std::fs::read_to_string(&registry_path)
                .map_err(|e| AppError::Io(format!("read registry: {e}")))?;
            let rd: RegistryData = serde_json::from_str(&data)
                .map_err(|e| AppError::Config(format!("parse registry: {e}")))?;
            rd.cores
        } else {
            HashMap::new()
        };
        Ok(Self {
            cores: RwLock::new(cores),
            registry_path,
        })
    }

    /// Register a version manifest for the given core name.
    pub fn register(&self, core: &str, manifest: VersionManifest) -> AppResult<()> {
        self.cores
            .write()
            .entry(core.to_string())
            .or_default()
            .push(manifest);
        self.save()
    }

    /// Unregister (remove) a specific version of a core. Returns true if a
    /// matching entry was found and removed.
    pub fn unregister(&self, core: &str, version: &str) -> AppResult<bool> {
        let mut guard = self.cores.write();
        if let Some(versions) = guard.get_mut(core) {
            let pos = versions.iter().position(|v| v.version == version);
            if let Some(idx) = pos {
                versions.remove(idx);
                drop(guard);
                self.save()?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Remove a specific version and its on-disk directory.
    /// Returns the removed manifest if found.
    pub fn remove_version(&self, core: &str, version: &str) -> AppResult<Option<VersionManifest>> {
        let mut guard = self.cores.write();
        if let Some(versions) = guard.get_mut(core) {
            let pos = versions.iter().position(|v| v.version == version);
            if let Some(idx) = pos {
                let removed = versions.remove(idx);
                // Attempt to remove the version directory from disk
                let version_path = Path::new(&removed.path);
                if version_path.exists() {
                    let _ = std::fs::remove_dir_all(version_path);
                }
                drop(guard);
                self.save()?;
                return Ok(Some(removed));
            }
        }
        Ok(None)
    }

    /// Get the currently-active version manifest for a core.
    pub fn get_current(&self, core: &str) -> Option<VersionManifest> {
        self.cores
            .read()
            .get(core)?
            .iter()
            .find(|v| v.is_current)
            .cloned()
    }

    /// List all installed versions for a core.
    pub fn list_versions(&self, core: &str) -> Vec<VersionManifest> {
        self.cores.read().get(core).cloned().unwrap_or_default()
    }

    /// Set the current (active) version for a core. Returns true if the
    /// core and version were found.
    pub fn set_current(&self, core: &str, version: &str) -> AppResult<bool> {
        let mut guard = self.cores.write();
        if let Some(versions) = guard.get_mut(core) {
            for v in versions.iter_mut() {
                v.is_current = v.version == version;
            }
            drop(guard);
            self.save()?;
            return Ok(true);
        }
        Ok(false)
    }

    /// Check whether a specific version of a core is registered.
    pub fn has_version(&self, core: &str, version: &str) -> bool {
        self.cores
            .read()
            .get(core)
            .is_some_and(|v| v.iter().any(|m| m.version == version))
    }

    /// Return the path to the registry file on disk.
    pub fn registry_path(&self) -> &Path {
        &self.registry_path
    }

    // ----- persistence -----

    fn save(&self) -> AppResult<()> {
        let data = RegistryData {
            cores: self.cores.read().clone(),
        };
        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| AppError::Config(format!("serialize registry: {e}")))?;
        if let Some(parent) = self.registry_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::fs::write(&self.registry_path, &json)
            .map_err(|e| AppError::Io(format!("write registry: {e}")))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(v: &str) -> VersionManifest {
        VersionManifest {
            version: v.into(),
            path: format!("/opt/{v}"),
            sha256: "abc".into(),
            installed_at: 0,
            is_current: false,
        }
    }

    fn test_registry() -> AppResult<CoreRegistry> {
        let tmp = std::env::temp_dir().join(format!("reg-test-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp)?;
        CoreRegistry::new(&tmp)
    }

    #[test]
    fn register_and_list() -> AppResult<()> {
        let r = test_registry()?;
        r.register("sing-box", manifest("1.11.0"))?;
        r.register("sing-box", manifest("1.10.0"))?;
        assert_eq!(r.list_versions("sing-box").len(), 2);
        assert!(r.has_version("sing-box", "1.11.0"));
        Ok(())
    }

    #[test]
    fn set_and_get_current() -> AppResult<()> {
        let r = test_registry()?;
        r.register("sing-box", manifest("1.11.0"))?;
        r.set_current("sing-box", "1.11.0")?;
        assert_eq!(r.get_current("sing-box").unwrap().version, "1.11.0");
        Ok(())
    }

    #[test]
    fn remove_version_works() -> AppResult<()> {
        let r = test_registry()?;
        r.register("mihomo", manifest("1.19.0"))?;
        r.register("mihomo", manifest("1.18.0"))?;
        assert_eq!(r.list_versions("mihomo").len(), 2);
        let removed = r.remove_version("mihomo", "1.18.0")?;
        assert!(removed.is_some());
        assert_eq!(r.list_versions("mihomo").len(), 1);
        assert!(!r.has_version("mihomo", "1.18.0"));
        Ok(())
    }

    #[test]
    fn unregister_removes_version() -> AppResult<()> {
        let r = test_registry()?;
        r.register("xray", manifest("1.0.0"))?;
        assert!(r.has_version("xray", "1.0.0"));
        let removed = r.unregister("xray", "1.0.0")?;
        assert!(removed);
        assert!(!r.has_version("xray", "1.0.0"));
        Ok(())
    }

    #[test]
    fn persistence_survives_reload() -> AppResult<()> {
        let tmp = std::env::temp_dir().join(format!("reg-persist-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&tmp)?;

        {
            let r = CoreRegistry::new(&tmp)?;
            r.register("sing-box", manifest("1.11.0"))?;
            r.set_current("sing-box", "1.11.0")?;
        }

        // Reload from disk
        let r2 = CoreRegistry::new(&tmp)?;
        assert!(r2.has_version("sing-box", "1.11.0"));
        assert_eq!(r2.get_current("sing-box").unwrap().version, "1.11.0");

        let _ = std::fs::remove_dir_all(&tmp);
        Ok(())
    }
}
