use super::super::rule_matcher::RuleMatcher;

#[derive(Default)]
pub struct DomainMatcher;

impl RuleMatcher for DomainMatcher {
    fn match_rule(&self, payload: &str, target: &str) -> bool {
        payload.eq_ignore_ascii_case(target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_match() {
        assert!(DomainMatcher.match_rule("example.com", "example.com"));
    }
    #[test]
    fn no_match() {
        assert!(!DomainMatcher.match_rule("example.com", "other.com"));
    }
}
