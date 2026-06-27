use base64::Engine;

use crate::models::{Node, NodeStatus};
use crate::utils::{AppError, AppResult};

pub struct Base64Parser;

impl Base64Parser {
    pub fn parse(encoded: &str) -> AppResult<Vec<Node>> {
        let trimmed = encoded.trim();
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(trimmed)
            .map_err(|e| AppError::Validation(format!("base64 decode: {e}")))?;
        let text =
            String::from_utf8(decoded).map_err(|e| AppError::Validation(format!("utf8: {e}")))?;
        let mut nodes = Vec::new();
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            nodes.push(Node {
                id: uuid::Uuid::new_v4().to_string(),
                name: format!("node-{}", nodes.len() + 1),
                country: String::new(),
                country_code: String::new(),
                delay: None,
                loss: None,
                status: NodeStatus::Untested,
                is_favorite: false,
                is_connected: false,
                node_type: if line.starts_with("vmess://") {
                    "VMess".into()
                } else if line.starts_with("vless://") {
                    "VLESS".into()
                } else if line.starts_with("trojan://") {
                    "Trojan".into()
                } else if line.starts_with("ss://") {
                    "Shadowsocks".into()
                } else {
                    "Unknown".into()
                },
                group: String::new(),
            });
        }
        Ok(nodes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_base64_uris() -> AppResult<()> {
        let encoded = base64::engine::general_purpose::STANDARD
            .encode("vmess://example.com:443\ntrojan://example.com:8443");
        let nodes = Base64Parser::parse(&encoded)?;
        assert_eq!(nodes.len(), 2);
        assert_eq!(nodes[0].node_type, "VMess");
        assert_eq!(nodes[1].node_type, "Trojan");
        Ok(())
    }
}
