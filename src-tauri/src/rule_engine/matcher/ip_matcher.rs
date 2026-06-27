use std::net::IpAddr;
use std::str::FromStr;

use ipnet::IpNet;

use super::super::rule_matcher::RuleMatcher;

#[derive(Default)]
pub struct IpCidrMatcher;

impl RuleMatcher for IpCidrMatcher {
    fn match_rule(&self, payload: &str, target: &str) -> bool {
        let cidr: IpNet = match payload.parse() {
            Ok(n) => n,
            Err(_) => return false,
        };
        let ip: IpAddr = match IpAddr::from_str(target) {
            Ok(i) => i,
            Err(_) => return false,
        };
        cidr.contains(&ip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ip_match() {
        assert!(IpCidrMatcher.match_rule("192.168.0.0/16", "192.168.1.1"));
    }
    #[test]
    fn ip_no_match() {
        assert!(!IpCidrMatcher.match_rule("192.168.0.0/16", "10.0.0.1"));
    }
    #[test]
    fn ipv6_match() {
        assert!(IpCidrMatcher.match_rule("::1/128", "::1"));
    }
}
