use std::sync::Arc;

use serde::Serialize;
use tauri::State;

use crate::core::CoreManager;
use crate::utils::AppError;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DnsStatusDto {
    pub state: String,
    pub resolver: String,
    pub cache_size: usize,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RuleStatsDto {
    pub rule_count: usize,
}

#[tauri::command]
pub async fn resolve_domain(
    core: State<'_, Arc<CoreManager>>,
    domain: String,
) -> Result<Vec<String>, String> {
    let dm = core
        .dns_manager()
        .ok_or_else(|| AppError::Internal("dns not initialized".into()))
        .map_err(|e| e.to_string())?;
    dm.resolve(&domain)
        .await
        .map(|ips| ips.iter().map(|i| i.to_string()).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn flush_dns_cache(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    let dm = core
        .dns_manager()
        .ok_or_else(|| AppError::Internal("dns not initialized".into()))
        .map_err(|e| e.to_string())?;
    dm.flush_cache();
    Ok(())
}

#[tauri::command]
pub async fn get_dns_status(core: State<'_, Arc<CoreManager>>) -> Result<DnsStatusDto, String> {
    let dm = core
        .dns_manager()
        .ok_or_else(|| AppError::Internal("dns not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(DnsStatusDto {
        state: format!("{:?}", dm.status()),
        resolver: dm.resolver_kind(),
        cache_size: dm.cache_size(),
    })
}

#[tauri::command]
pub async fn reload_rules(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.reload_rules().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn match_rule(
    core: State<'_, Arc<CoreManager>>,
    domain: String,
) -> Result<String, String> {
    let re = core
        .rule_engine()
        .ok_or_else(|| AppError::Internal("rule engine not initialized".into()))
        .map_err(|e| e.to_string())?;
    let result = re
        .match_connection(&domain, 443, None)
        .map_err(|e| e.to_string())?;
    Ok(format!("{:?}", result))
}

#[tauri::command]
pub async fn get_rule_statistics(
    core: State<'_, Arc<CoreManager>>,
) -> Result<RuleStatsDto, String> {
    let re = core
        .rule_engine()
        .ok_or_else(|| AppError::Internal("rule engine not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(RuleStatsDto {
        rule_count: re.rule_count(),
    })
}
