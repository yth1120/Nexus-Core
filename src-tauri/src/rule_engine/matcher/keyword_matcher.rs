use super::super::rule_matcher::RuleMatcher;

#[derive(Default)]
pub struct KeywordMatcher;

impl RuleMatcher for KeywordMatcher {
    fn match_rule(&self, payload: &str, target: &str) -> bool {
        target.to_lowercase().contains(&payload.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn keyword_match() {
        assert!(KeywordMatcher.match_rule("google", "www.google.com"));
    }
    #[test]
    fn no_match() {
        assert!(!KeywordMatcher.match_rule("bing", "www.google.com"));
    }
}
