use uuid::Uuid;

use crate::core::AppState;
use crate::models::Rule;
use crate::utils::AppResult;

/// Request payload for creating a rule.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRuleRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub rule_type: String,
    pub payload: String,
    pub proxy: String,
    pub enabled: Option<bool>,
    pub tags: Option<Vec<String>>,
}

pub fn get_all(state: &AppState) -> Vec<Rule> {
    state.rules.read().clone()
}

pub fn create(state: &AppState, data: CreateRuleRequest) -> Rule {
    let rule = Rule {
        id: format!("rule-{}", Uuid::new_v4()),
        name: data.name,
        rule_type: data.rule_type,
        payload: data.payload,
        proxy: data.proxy,
        enabled: data.enabled.unwrap_or(true),
        tags: data.tags.unwrap_or_default(),
        created_at: chrono::Utc::now().timestamp_millis(),
    };

    let mut rules = state.rules.write();
    rules.push(rule.clone());
    rule
}

pub fn update(state: &AppState, id: &str, data: serde_json::Value) -> AppResult<Rule> {
    let mut rules = state.rules.write();
    let rule = rules
        .iter_mut()
        .find(|r| r.id == id)
        .ok_or_else(|| crate::utils::AppError::NotFound(format!("Rule {}", id)))?;

    if let Some(name) = data.get("name").and_then(|v| v.as_str()) {
        rule.name = name.to_string();
    }
    if let Some(t) = data.get("type").and_then(|v| v.as_str()) {
        rule.rule_type = t.to_string();
    }
    if let Some(payload) = data.get("payload").and_then(|v| v.as_str()) {
        rule.payload = payload.to_string();
    }
    if let Some(proxy) = data.get("proxy").and_then(|v| v.as_str()) {
        rule.proxy = proxy.to_string();
    }
    if let Some(enabled) = data.get("enabled").and_then(|v| v.as_bool()) {
        rule.enabled = enabled;
    }
    if let Some(tags) = data.get("tags").and_then(|v| v.as_array()) {
        rule.tags = tags
            .iter()
            .filter_map(|t| t.as_str().map(String::from))
            .collect();
    }

    Ok(rule.clone())
}

pub fn delete(state: &AppState, id: &str) -> AppResult<()> {
    let mut rules = state.rules.write();
    let len_before = rules.len();
    rules.retain(|r| r.id != id);
    if rules.len() == len_before {
        return Err(crate::utils::AppError::NotFound(format!("Rule {}", id)));
    }
    Ok(())
}

pub fn toggle_enabled(state: &AppState, id: &str) -> AppResult<Rule> {
    let mut rules = state.rules.write();
    let rule = rules
        .iter_mut()
        .find(|r| r.id == id)
        .ok_or_else(|| crate::utils::AppError::NotFound(format!("Rule {}", id)))?;

    rule.enabled = !rule.enabled;
    Ok(rule.clone())
}
