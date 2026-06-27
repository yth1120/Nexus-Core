use parking_lot::RwLock;
use std::collections::HashMap;
use std::net::IpAddr;

use super::dns_record::DnsRecord;

pub struct DnsCache {
    entries: RwLock<HashMap<String, DnsRecord>>,
    max_size: usize,
}

impl DnsCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: RwLock::new(HashMap::new()),
            max_size,
        }
    }

    pub fn insert(&self, record: DnsRecord) {
        let mut guard = self.entries.write();
        if guard.len() >= self.max_size {
            self.cleanup_inner(&mut guard);
            // If still at capacity after cleaning expired entries,
            // evict the entry with the soonest expiry.
            if guard.len() >= self.max_size {
                let oldest = guard
                    .iter()
                    .min_by_key(|(_, r)| r.created_at)
                    .map(|(k, _)| k.clone());
                if let Some(key) = oldest {
                    guard.remove(&key);
                }
            }
        }
        guard.insert(record.domain.clone(), record);
    }

    pub fn get(&self, domain: &str) -> Option<Vec<IpAddr>> {
        let guard = self.entries.read();
        guard
            .get(&domain.to_lowercase())
            .filter(|r| !r.is_expired())
            .map(|r| r.ips.clone())
    }

    pub fn remove(&self, domain: &str) {
        self.entries.write().remove(&domain.to_lowercase());
    }

    pub fn clear(&self) {
        self.entries.write().clear();
    }

    pub fn cleanup(&self) {
        let mut guard = self.entries.write();
        self.cleanup_inner(&mut guard);
    }

    fn cleanup_inner(&self, guard: &mut HashMap<String, DnsRecord>) {
        guard.retain(|_, r| !r.is_expired());
    }

    pub fn len(&self) -> usize {
        self.entries.read().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl Default for DnsCache {
    fn default() -> Self {
        Self::new(4096)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr};

    fn test_record(domain: &str) -> DnsRecord {
        DnsRecord::new(domain, vec![IpAddr::V4(Ipv4Addr::LOCALHOST)], 300)
    }

    #[test]
    fn insert_and_get() {
        let c = DnsCache::new(16);
        c.insert(test_record("example.com"));
        let ips = c.get("example.com");
        assert!(ips.is_some());
        assert_eq!(ips.unwrap()[0], IpAddr::V4(Ipv4Addr::LOCALHOST));
    }

    #[test]
    fn remove_entry() {
        let c = DnsCache::new(16);
        c.insert(test_record("example.com"));
        assert_eq!(c.len(), 1);
        c.remove("example.com");
        assert_eq!(c.len(), 0);
    }

    #[test]
    fn cleanup_removes_expired() {
        let c = DnsCache::new(16);
        c.insert(DnsRecord::new(
            "example.com",
            vec![IpAddr::V4(Ipv4Addr::LOCALHOST)],
            0,
        ));
        std::thread::sleep(std::time::Duration::from_millis(1));
        c.cleanup();
        assert_eq!(c.len(), 0);
    }
}
