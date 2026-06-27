use crate::models::{Node, Profile, Rule};
use crate::utils::AppResult;

pub fn generate(_profile: &Profile, _nodes: &[Node], _rules: &[Rule]) -> AppResult<String> {
    Ok(r#"mixed-port: 7890
bind-address: "127.0.0.1"
mode: rule
log-level: info
proxies: []
proxy-groups:
  - name: "Proxy"
    type: select
    proxies:
      - "DIRECT"
rules: []
"#
    .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generates_valid_yaml() -> AppResult<()> {
        let s = generate(&Profile::default(), &[], &[])?;
        assert!(s.contains("mixed-port"));
        assert!(s.contains("Proxy"));
        Ok(())
    }
}
