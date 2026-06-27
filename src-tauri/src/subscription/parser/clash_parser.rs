use crate::models::{Node, NodeStatus, Rule};
use crate::utils::{AppError, AppResult};

pub struct ClashParser;

impl ClashParser {
    pub fn parse(yaml: &str) -> AppResult<(Vec<Node>, Vec<Rule>)> {
        let v: serde_yaml::Value = serde_yaml::from_str(yaml)
            .map_err(|e| AppError::Validation(format!("clash yaml: {e}")))?;
        let mut nodes = Vec::new();
        let mut rules = Vec::new();

        if let Some(proxies) = v.get("proxies").and_then(|p| p.as_sequence()) {
            for p in proxies {
                nodes.push(Node {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: p
                        .get("name")
                        .and_then(|n| n.as_str())
                        .unwrap_or("unknown")
                        .to_string(),
                    country: String::new(),
                    country_code: String::new(),
                    delay: None,
                    loss: None,
                    status: NodeStatus::Untested,
                    is_favorite: false,
                    is_connected: false,
                    node_type: p
                        .get("type")
                        .and_then(|t| t.as_str())
                        .unwrap_or("unknown")
                        .to_string(),
                    group: String::new(),
                });
            }
        }

        if let Some(rls) = v.get("rules").and_then(|r| r.as_sequence()) {
            for r in rls {
                if let Some(s) = r.as_str() {
                    let parts: Vec<&str> = s.splitn(2, ',').collect();
                    if parts.len() >= 2 {
                        rules.push(Rule {
                            id: uuid::Uuid::new_v4().to_string(),
                            name: format!("clash-rule-{}", rules.len()),
                            rule_type: parts[0].trim().to_string(),
                            payload: parts[1].trim().to_string(),
                            proxy: "Proxy".into(),
                            enabled: true,
                            tags: vec![],
                            created_at: 0,
                        });
                    }
                }
            }
        }

        Ok((nodes, rules))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_valid_yaml() -> AppResult<()> {
        let yaml = r#"
proxies:
  - name: "HK-01"
    type: vmess
    server: hk.example.com
    port: 443
rules:
  - DOMAIN-SUFFIX,google.com,Proxy
  - MATCH,DIRECT
"#;
        let (nodes, rules) = ClashParser::parse(yaml)?;
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "HK-01");
        assert!(rules.len() >= 2);
        Ok(())
    }
}
