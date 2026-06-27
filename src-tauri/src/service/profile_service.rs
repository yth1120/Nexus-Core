use uuid::Uuid;

use crate::core::AppState;
use crate::models::{Profile, ProfileStatus, ProfileType};
use crate::utils::AppResult;

/// Request payload for creating a profile.
/// Mirrors the frontend's `Omit<Profile, 'id' | 'status' | 'latency' | 'updated' | 'trafficUsed' | 'trafficTotal'>`.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileRequest {
    pub name: String,
    #[serde(rename = "type")]
    pub profile_type: String,
    pub config_url: Option<String>,
}

pub fn get_all(state: &AppState) -> Vec<Profile> {
    state.profiles.read().clone()
}

pub fn get_by_id(state: &AppState, id: &str) -> AppResult<Profile> {
    state
        .profiles
        .read()
        .iter()
        .find(|p| p.id == id)
        .cloned()
        .ok_or_else(|| crate::utils::AppError::NotFound(format!("Profile {}", id)))
}

pub fn create(state: &AppState, data: CreateProfileRequest) -> Profile {
    let profile_type = match data.profile_type.as_str() {
        "Subscription" => ProfileType::Subscription,
        "WireGuard" => ProfileType::WireGuard,
        "VLESS" => ProfileType::Vless,
        "Clash Meta" => ProfileType::ClashMeta,
        _ => ProfileType::Custom,
    };

    let profile = Profile {
        id: format!("profile-{}", Uuid::new_v4()),
        name: data.name,
        profile_type,
        status: ProfileStatus::Inactive,
        latency: 0,
        updated: chrono::Utc::now().to_rfc3339(),
        config_url: data.config_url,
        traffic_used: Some(0),
        traffic_total: None,
    };

    let mut profiles = state.profiles.write();
    profiles.insert(0, profile.clone());

    // Dual-write to database if repository is available
    if let Some(rm) = state.get_resource_manager() {
        let _ = rm.profile_repo.insert(&profile);
    }

    profile
}

pub fn update(state: &AppState, id: &str, data: serde_json::Value) -> AppResult<Profile> {
    let mut profiles = state.profiles.write();
    let index = profiles
        .iter()
        .position(|p| p.id == id)
        .ok_or_else(|| crate::utils::AppError::NotFound(format!("Profile {}", id)))?;

    let mut profile = profiles[index].clone();

    if let Some(name) = data.get("name").and_then(|v| v.as_str()) {
        profile.name = name.to_string();
    }
    if let Some(t) = data.get("type").and_then(|v| v.as_str()) {
        profile.profile_type = match t {
            "Subscription" => ProfileType::Subscription,
            "WireGuard" => ProfileType::WireGuard,
            "VLESS" => ProfileType::Vless,
            "Clash Meta" => ProfileType::ClashMeta,
            _ => ProfileType::Custom,
        };
    }
    if let Some(url) = data.get("configUrl").and_then(|v| v.as_str()) {
        profile.config_url = if url.is_empty() {
            None
        } else {
            Some(url.to_string())
        };
    }
    profile.updated = chrono::Utc::now().to_rfc3339();

    profiles[index] = profile.clone();
    Ok(profile)
}

pub fn delete(state: &AppState, id: &str) -> AppResult<()> {
    let mut profiles = state.profiles.write();
    let len_before = profiles.len();
    profiles.retain(|p| p.id != id);
    if profiles.len() == len_before {
        return Err(crate::utils::AppError::NotFound(format!("Profile {}", id)));
    }
    Ok(())
}

pub fn toggle_active(state: &AppState, id: &str) -> AppResult<Profile> {
    let mut profiles = state.profiles.write();
    let index = profiles
        .iter()
        .position(|p| p.id == id)
        .ok_or_else(|| crate::utils::AppError::NotFound(format!("Profile {}", id)))?;

    let currently_active = matches!(profiles[index].status, ProfileStatus::Active);

    if currently_active {
        profiles[index].status = ProfileStatus::Inactive;
    } else {
        // Deactivate all others, activate this one
        for p in profiles.iter_mut() {
            p.status = ProfileStatus::Inactive;
        }
        profiles[index].status = ProfileStatus::Active;
    }

    Ok(profiles[index].clone())
}
