pub mod domain_matcher;
pub mod ip_matcher;
pub mod keyword_matcher;
pub mod port_matcher;
pub mod suffix_matcher;

use super::rule_matcher::RuleMatcher;

/// Get the appropriate matcher for a rule type. Returns `None` for unknown types.
pub fn matcher_for(rule_type: &str) -> Option<Box<dyn RuleMatcher>> {
    match rule_type {
        "Domain" | "DOMAIN" => Some(Box::<domain_matcher::DomainMatcher>::default()),
        "DomainKeyword" | "DOMAIN-KEYWORD" => {
            Some(Box::<keyword_matcher::KeywordMatcher>::default())
        }
        "DomainSuffix" | "DOMAIN-SUFFIX" => Some(Box::<suffix_matcher::SuffixMatcher>::default()),
        "IP-CIDR" | "IPCIDR" | "IpCidr" => Some(Box::<ip_matcher::IpCidrMatcher>::default()),
        "Port" | "PORT" => Some(Box::<port_matcher::PortMatcher>::default()),
        "MATCH" | "Match" => Some(Box::<domain_matcher::DomainMatcher>::default()), // catch-all
        _ => None,
    }
}
