use crate::models::{Node, NodeStatus, Rule};
use crate::utils::{AppError, AppResult};

pub struct SingBoxParser;

impl SingBoxParser {
    pub fn parse(json: &str) -> AppResult<(Vec<Node>, Vec<Rule>)> {
        let v: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| AppError::Validation(format!("sing-box json: {e}")))?;
        let mut nodes = Vec::new();
        if let Some(arr) = v.get("outbounds").and_then(|o| o.as_array()) {
            for item in arr {
                nodes.push(Node {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: item
                        .get("tag")
                        .and_then(|t| t.as_str())
                        .unwrap_or("unknown")
                        .to_string(),
                    country: String::new(),
                    country_code: String::new(),
                    delay: None,
                    loss: None,
                    status: NodeStatus::Untested,
                    is_favorite: false,
                    is_connected: false,
                    node_type: item
                        .get("type")
                        .and_then(|t| t.as_str())
                        .unwrap_or("direct")
                        .to_string(),
                    group: String::new(),
                });
            }
        }
        Ok((nodes, vec![]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_valid_json() -> AppResult<()> {
        let json = r#"{"outbounds":[{"type":"vmess","tag":"HK-01","server":"hk.example.com","server_port":443}]}"#;
        let (nodes, _) = SingBoxParser::parse(json)?;
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].name, "HK-01");
        Ok(())
    }
}
