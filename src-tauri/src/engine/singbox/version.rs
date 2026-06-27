pub fn parse_version(output: &str) -> Option<String> {
    for line in output.lines() {
        let trimmed = line.trim();
        if let Some(v) = trimmed.strip_prefix("sing-box version ") {
            return Some(v.to_string());
        }
        if let Some(v) = trimmed.strip_prefix("version ") {
            return Some(v.to_string());
        }
        // Fallback: find semver-like pattern
        if trimmed.starts_with(|c: char| c.is_ascii_digit()) {
            return Some(trimmed.split_whitespace().next()?.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_standard() {
        assert_eq!(
            parse_version("sing-box version 1.11.0"),
            Some("1.11.0".into())
        );
    }
    #[test]
    fn parse_alt() {
        assert_eq!(
            parse_version("version 1.10.0-beta"),
            Some("1.10.0-beta".into())
        );
    }
    #[test]
    fn parse_none() {
        assert_eq!(parse_version("unknown tool"), None);
    }
}
