pub trait RuleMatcher: Send + Sync {
    fn match_rule(&self, payload: &str, target: &str) -> bool;
}
