pub fn parse_version(output: &str) -> Option<String> {
    for line in output.lines() {
        let t = line.trim();
        if let Some(v) = t.strip_prefix("Mihomo ") {
            return Some(v.to_string());
        }
        if let Some(v) = t.strip_prefix("Clash Meta ") {
            return Some(v.to_string());
        }
        if let Some(v) = t.strip_prefix("mihomo ") {
            return Some(v.to_string());
        }
        if let Some(v) = t.strip_prefix("v") {
            return Some(v.to_string());
        }
        if t.starts_with(|c: char| c.is_ascii_digit()) && t.contains('.') {
            return Some(t.split_whitespace().next()?.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_standard() {
        assert_eq!(parse_version("Mihomo 1.19.0"), Some("1.19.0".into()));
    }
    #[test]
    fn parse_alt() {
        assert_eq!(parse_version("Clash Meta v1.18.7"), Some("1.18.7".into()));
    }
    #[test]
    fn parse_none() {
        assert_eq!(parse_version("unknown"), None);
    }
}
