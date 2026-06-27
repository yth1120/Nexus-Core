use crate::core::CoreManager;
use crate::subscription::Subscription;
use crate::utils::AppError;
use std::sync::Arc;
use tauri::State;

/// Validate that a URL uses an allowed scheme (http/https only).
fn validate_url_scheme(url: &str, context: &str) -> Result<(), String> {
    let lower = url.to_lowercase();
    if !lower.starts_with("https://") && !lower.starts_with("http://") {
        return Err(AppError::Validation(format!(
            "{context}: only http/https URLs are allowed, got: {url}"
        ))
        .to_string());
    }
    Ok(())
}

#[tauri::command]
pub async fn add_subscription(
    core: State<'_, Arc<CoreManager>>,
    name: String,
    url: String,
) -> Result<Subscription, String> {
    validate_url_scheme(&url, "subscription URL")?;
    let sm = core
        .subscription_manager()
        .ok_or_else(|| AppError::Internal("not initialized".into()))
        .map_err(|e| e.to_string())?;
    sm.add_subscription(&name, &url).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_subscription(
    core: State<'_, Arc<CoreManager>>,
    id: String,
) -> Result<(), String> {
    let sm = core
        .subscription_manager()
        .ok_or_else(|| AppError::Internal("not initialized".into()))
        .map_err(|e| e.to_string())?;
    sm.remove_subscription(&id);
    Ok(())
}

#[tauri::command]
pub async fn update_subscription(
    _core: State<'_, Arc<CoreManager>>,
    _id: String,
) -> Result<(), String> {
    Ok(())
}
#[tauri::command]
pub async fn update_all_subscriptions(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    let sm = core
        .subscription_manager()
        .ok_or_else(|| AppError::Internal("not initialized".into()))
        .map_err(|e| e.to_string())?;
    sm.update_all().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_subscriptions(
    core: State<'_, Arc<CoreManager>>,
) -> Result<Vec<Subscription>, String> {
    let sm = core
        .subscription_manager()
        .ok_or_else(|| AppError::Internal("not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(sm.list())
}

#[tauri::command]
pub async fn download_ruleset(
    core: State<'_, Arc<CoreManager>>,
    url: String,
) -> Result<(), String> {
    validate_url_scheme(&url, "ruleset URL")?;
    let rm = core
        .ruleset_manager()
        .ok_or_else(|| AppError::Internal("not initialized".into()))
        .map_err(|e| e.to_string())?;
    rm.download(&url).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reload_rulesets(core: State<'_, Arc<CoreManager>>) -> Result<(), String> {
    core.reload_rulesets().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_rulesets(
    core: State<'_, Arc<CoreManager>>,
) -> Result<Vec<crate::ruleset::RuleSetInfo>, String> {
    let rm = core
        .ruleset_manager()
        .ok_or_else(|| AppError::Internal("not initialized".into()))
        .map_err(|e| e.to_string())?;
    Ok(rm.list())
}
