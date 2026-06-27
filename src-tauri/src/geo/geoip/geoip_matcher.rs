use std::sync::Arc;

use super::mmdb_reader::MmdbReader;

/// Matches GEOIP rules: payload is a country code (e.g. "CN", "US"),
/// target is an IP address string.
pub struct GeoIpMatcher {
    reader: Arc<MmdbReader>,
}

impl GeoIpMatcher {
    pub fn new(reader: Arc<MmdbReader>) -> Self {
        Self { reader }
    }

    /// Check whether `ip` is located in the country identified by `country_code`.
    pub fn is_country(&self, ip: &str, country_code: &str) -> bool {
        match self.reader.lookup_country(ip) {
            Ok(Some(code)) => code.eq_ignore_ascii_case(country_code),
            _ => false,
        }
    }

    /// Check whether `ip` is a private/internal IP address.
    /// This is handled without the MMDB — just IP range checks.
    pub fn is_private(ip: &str) -> bool {
        if let Ok(addr) = ip.parse::<std::net::IpAddr>() {
            match addr {
                std::net::IpAddr::V4(v4) => {
                    v4.is_private() || v4.is_loopback() || v4.is_link_local()
                }
                std::net::IpAddr::V6(v6) => {
                    v6.is_loopback() || v6.is_unique_local() || v6.is_multicast()
                }
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_private_detects_private_ips() {
        assert!(GeoIpMatcher::is_private("192.168.1.1"));
        assert!(GeoIpMatcher::is_private("10.0.0.1"));
        assert!(GeoIpMatcher::is_private("127.0.0.1"));
        assert!(GeoIpMatcher::is_private("::1"));
        assert!(!GeoIpMatcher::is_private("8.8.8.8"));
        assert!(!GeoIpMatcher::is_private("invalid"));
    }

    #[test]
    fn is_private_ipv6_detection() {
        assert!(GeoIpMatcher::is_private("fc00::1")); // unique local
        assert!(GeoIpMatcher::is_private("ff02::1")); // multicast
        assert!(!GeoIpMatcher::is_private("2001:4860:4860::8888")); // Google DNS
    }
}
