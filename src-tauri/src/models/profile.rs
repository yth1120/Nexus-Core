use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ProfileStatus {
    Active,
    Inactive,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProfileType {
    Subscription,
    #[serde(rename = "WireGuard")]
    WireGuard,
    #[serde(rename = "VLESS")]
    Vless,
    #[serde(rename = "Clash Meta")]
    ClashMeta,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub profile_type: ProfileType,
    pub status: ProfileStatus,
    pub latency: i64,
    pub updated: String,
    pub config_url: Option<String>,
    pub traffic_used: Option<i64>,
    pub traffic_total: Option<i64>,
}

impl Default for Profile {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            profile_type: ProfileType::Custom,
            status: ProfileStatus::Inactive,
            latency: 0,
            updated: String::new(),
            config_url: None,
            traffic_used: None,
            traffic_total: None,
        }
    }
}
