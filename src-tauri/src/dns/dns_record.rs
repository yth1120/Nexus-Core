use std::net::IpAddr;
use std::time::Instant;

#[derive(Debug, Clone)]
pub struct DnsRecord {
    pub domain: String,
    pub ips: Vec<IpAddr>,
    pub ttl: u64,
    pub created_at: Instant,
}

impl DnsRecord {
    pub fn new(domain: &str, ips: Vec<IpAddr>, ttl: u64) -> Self {
        Self {
            domain: domain.to_lowercase(),
            ips,
            ttl,
            created_at: Instant::now(),
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed().as_secs() > self.ttl
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;
    #[test]
    fn not_expired_immediately() {
        let r = DnsRecord::new("example.com", vec![IpAddr::V4(Ipv4Addr::LOCALHOST)], 300);
        assert!(!r.is_expired());
    }
}
