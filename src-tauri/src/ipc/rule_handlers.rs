use std::sync::Arc;
use tauri::State;

use crate::core::AppState;
use crate::models::Rule;
use crate::service::rule_service::{self, CreateRuleRequest};

#[tauri::command]
pub async fn get_rules(state: State<'_, Arc<AppState>>) -> Result<Vec<Rule>, String> {
    Ok(rule_service::get_all(&state))
}

#[tauri::command]
pub async fn create_rule(
    state: State<'_, Arc<AppState>>,
    data: CreateRuleRequest,
) -> Result<Rule, String> {
    Ok(rule_service::create(&state, data))
}

#[tauri::command]
pub async fn update_rule(
    state: State<'_, Arc<AppState>>,
    id: String,
    data: serde_json::Value,
) -> Result<Rule, String> {
    rule_service::update(&state, &id, data).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_rule(state: State<'_, Arc<AppState>>, id: String) -> Result<(), String> {
    rule_service::delete(&state, &id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_rule(state: State<'_, Arc<AppState>>, id: String) -> Result<Rule, String> {
    rule_service::toggle_enabled(&state, &id).map_err(|e| e.to_string())
}
