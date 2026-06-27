use std::path::PathBuf;

use parking_lot::RwLock;

/// Cached metadata about a geo database file.
#[derive(Debug, Clone)]
pub struct DatabaseMeta {
    pub path: PathBuf,
    pub version: String,
    pub size_bytes: u64,
    pub loaded_at: i64,
}

/// Simple cache for database file metadata.
///
/// Used to track which databases are loaded and their versions.
pub struct DatabaseCache {
    entries: RwLock<Vec<DatabaseMeta>>,
}

impl DatabaseCache {
    pub fn new() -> Self {
        Self {
            entries: RwLock::new(Vec::new()),
        }
    }

    /// Record a loaded database.
    pub fn record(&self, meta: DatabaseMeta) {
        let mut guard = self.entries.write();
        // Replace any existing entry for the same path
        guard.retain(|m| m.path != meta.path);
        guard.push(meta);
    }

    /// Get metadata for a database by path.
    pub fn get(&self, path: &PathBuf) -> Option<DatabaseMeta> {
        self.entries
            .read()
            .iter()
            .find(|m| m.path == *path)
            .cloned()
    }

    /// Return all tracked database entries.
    pub fn all(&self) -> Vec<DatabaseMeta> {
        self.entries.read().clone()
    }

    /// Remove an entry.
    pub fn remove(&self, path: &PathBuf) {
        self.entries.write().retain(|m| m.path != *path);
    }
}

impl Default for DatabaseCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_and_retrieve() {
        let cache = DatabaseCache::new();
        let meta = DatabaseMeta {
            path: PathBuf::from("/tmp/geoip.mmdb"),
            version: "v1".into(),
            size_bytes: 1024,
            loaded_at: 1000,
        };
        cache.record(meta.clone());
        let found = cache.get(&PathBuf::from("/tmp/geoip.mmdb"));
        assert!(found.is_some());
        assert_eq!(found.unwrap().version, "v1");
    }

    #[test]
    fn record_replaces_duplicate() {
        let cache = DatabaseCache::new();
        let meta1 = DatabaseMeta {
            path: PathBuf::from("/tmp/db.dat"),
            version: "v1".into(),
            size_bytes: 100,
            loaded_at: 1,
        };
        let meta2 = DatabaseMeta {
            path: PathBuf::from("/tmp/db.dat"),
            version: "v2".into(),
            size_bytes: 200,
            loaded_at: 2,
        };
        cache.record(meta1);
        cache.record(meta2);
        let all = cache.all();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].version, "v2");
    }
}
