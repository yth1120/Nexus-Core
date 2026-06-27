use super::super::rule_matcher::RuleMatcher;

#[derive(Default)]
pub struct PortMatcher;

impl RuleMatcher for PortMatcher {
    fn match_rule(&self, payload: &str, target: &str) -> bool {
        let target_port: u16 = match target.parse() {
            Ok(p) => p,
            Err(_) => return false,
        };
        for part in payload.split(',') {
            let part = part.trim();
            if let Ok(p) = part.parse::<u16>() {
                if p == target_port {
                    return true;
                }
            }
            if let Some((start, end)) = part.split_once('-') {
                if let (Ok(s), Ok(e)) = (start.trim().parse::<u16>(), end.trim().parse::<u16>()) {
                    if target_port >= s && target_port <= e {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn port_match() {
        assert!(PortMatcher.match_rule("80,443,8080", "443"));
    }
    #[test]
    fn port_no_match() {
        assert!(!PortMatcher.match_rule("80,443", "22"));
    }
    #[test]
    fn port_range() {
        assert!(PortMatcher.match_rule("8000-9000", "8443"));
    }
}
