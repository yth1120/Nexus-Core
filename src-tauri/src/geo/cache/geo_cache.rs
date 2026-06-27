use std::collections::HashMap;
use std::time::Instant;

use parking_lot::RwLock;

/// A cached country lookup result.
#[derive(Debug, Clone)]
struct CachedCountry {
    country: Option<String>,
    inserted_at: Instant,
}

/// A cached domain category lookup result.
#[derive(Debug, Clone)]
struct CachedCategory {
    #[allow(dead_code)]
    category: String,
    in_category: bool,
    inserted_at: Instant,
}

/// TTL-based cache for GeoIP country lookups and GeoSite domain category lookups.
///
/// Follows the same pattern as `DnsCache`: `HashMap` behind `RwLock` with
/// TTL-based cleanup on insert when the size limit is reached.
pub struct GeoCache {
    country: RwLock<HashMap<String, CachedCountry>>,
    domain_category: RwLock<HashMap<String, CachedCategory>>,
    max_size: usize,
    ttl_secs: u64,
}

impl GeoCache {
    pub fn new(max_size: usize, ttl_secs: u64) -> Self {
        Self {
            country: RwLock::new(HashMap::new()),
            domain_category: RwLock::new(HashMap::new()),
            max_size,
            ttl_secs,
        }
    }

    // ----- country cache -----

    /// Look up a cached country for an IP. Returns `None` if not cached or expired.
    pub fn get_country(&self, ip: &str) -> Option<Option<String>> {
        let guard = self.country.read();
        guard
            .get(&ip.to_lowercase())
            .filter(|c| c.inserted_at.elapsed().as_secs() < self.ttl_secs)
            .map(|c| c.country.clone())
    }

    /// Insert a country lookup result into the cache.
    pub fn put_country(&self, ip: &str, country: Option<String>) {
        let mut guard = self.country.write();
        if guard.len() >= self.max_size {
            self.cleanup_country_inner(&mut guard);
        }
        guard.insert(
            ip.to_lowercase(),
            CachedCountry {
                country,
                inserted_at: Instant::now(),
            },
        );
    }

    // ----- domain category cache -----

    /// Look up a cached domain→category match. Returns `None` if not cached or expired.
    pub fn get_domain_category(&self, domain: &str, category: &str) -> Option<bool> {
        let key = Self::domain_key(domain, category);
        let guard = self.domain_category.read();
        guard
            .get(&key)
            .filter(|c| c.inserted_at.elapsed().as_secs() < self.ttl_secs)
            .map(|c| c.in_category)
    }

    /// Insert a domain→category match result into the cache.
    pub fn put_domain_category(&self, domain: &str, category: &str, in_category: bool) {
        let key = Self::domain_key(domain, category);
        let mut guard = self.domain_category.write();
        if guard.len() >= self.max_size {
            self.cleanup_domain_inner(&mut guard);
        }
        guard.insert(
            key,
            CachedCategory {
                category: category.to_string(),
                in_category,
                inserted_at: Instant::now(),
            },
        );
    }

    // ----- cleanup -----

    /// Remove all expired entries from both caches.
    pub fn cleanup(&self) {
        self.cleanup_country_inner(&mut self.country.write());
        self.cleanup_domain_inner(&mut self.domain_category.write());
    }

    /// Clear all cached entries.
    pub fn clear(&self) {
        self.country.write().clear();
        self.domain_category.write().clear();
    }

    pub fn country_len(&self) -> usize {
        self.country.read().len()
    }

    pub fn domain_len(&self) -> usize {
        self.domain_category.read().len()
    }

    // ----- internals -----

    fn domain_key(domain: &str, category: &str) -> String {
        format!("{}|{}", domain.to_lowercase(), category.to_lowercase())
    }

    fn cleanup_country_inner(&self, guard: &mut HashMap<String, CachedCountry>) {
        guard.retain(|_, c| c.inserted_at.elapsed().as_secs() < self.ttl_secs);
    }

    fn cleanup_domain_inner(&self, guard: &mut HashMap<String, CachedCategory>) {
        guard.retain(|_, c| c.inserted_at.elapsed().as_secs() < self.ttl_secs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn country_cache_hit() {
        let cache = GeoCache::new(16, 3600);
        cache.put_country("1.2.3.4", Some("US".into()));
        assert_eq!(cache.get_country("1.2.3.4"), Some(Some("US".into())));
        assert_eq!(cache.get_country("8.8.8.8"), None);
    }

    #[test]
    fn domain_category_cache_hit() {
        let cache = GeoCache::new(16, 3600);
        cache.put_domain_category("google.com", "google", true);
        assert_eq!(
            cache.get_domain_category("google.com", "google"),
            Some(true)
        );
        assert_eq!(cache.get_domain_category("example.com", "google"), None);
    }

    #[test]
    fn expired_entries_removed() {
        let cache = GeoCache::new(16, 0); // ttl = 0 (immediate expiry)
        cache.put_country("1.2.3.4", Some("US".into()));
        std::thread::sleep(std::time::Duration::from_millis(1));
        assert_eq!(cache.get_country("1.2.3.4"), None);
    }

    #[test]
    fn clear_removes_all() {
        let cache = GeoCache::new(16, 3600);
        cache.put_country("1.2.3.4", Some("US".into()));
        cache.put_domain_category("google.com", "google", true);
        assert!(cache.country_len() > 0);
        assert!(cache.domain_len() > 0);
        cache.clear();
        assert_eq!(cache.country_len(), 0);
        assert_eq!(cache.domain_len(), 0);
    }
}
