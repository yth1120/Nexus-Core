use super::super::rule_matcher::RuleMatcher;

#[derive(Default)]
pub struct SuffixMatcher;

impl RuleMatcher for SuffixMatcher {
    fn match_rule(&self, payload: &str, target: &str) -> bool {
        target.to_lowercase().ends_with(&payload.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn suffix_match() {
        assert!(SuffixMatcher.match_rule(".com", "example.com"));
    }
    #[test]
    fn no_match() {
        assert!(!SuffixMatcher.match_rule(".org", "example.com"));
    }
}
