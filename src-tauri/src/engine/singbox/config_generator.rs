use crate::models::{Node, Profile, Rule};
use crate::utils::AppResult;

pub fn generate(_profile: &Profile, _nodes: &[Node], _rules: &[Rule]) -> AppResult<String> {
    let config = serde_json::json!({
        "log": { "level": "info", "timestamp": true },
        "inbounds": [{
            "type": "mixed",
            "listen": "127.0.0.1",
            "listen_port": 7890,
            "sniff": true,
            "sniff_override_destination": false
        }],
        "outbounds": [{ "type": "direct", "tag": "direct-out" }],
        "route": {
            "rules": [],
            "final": "direct-out"
        }
    });
    serde_json::to_string_pretty(&config)
        .map_err(|e| crate::utils::AppError::Internal(format!("sing-box config: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generates_valid_json() -> AppResult<()> {
        let s = generate(&Profile::default(), &[], &[])?;
        let _v: serde_json::Value = serde_json::from_str(&s)
            .map_err(|e| crate::utils::AppError::Internal(e.to_string()))?;
        assert!(s.contains("mixed"));
        Ok(())
    }
}
