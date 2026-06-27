use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum DashboardRunStatus {
    Running,
    Stopped,
    Connecting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStatus {
    pub status: DashboardRunStatus,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub uptime: i64,
    pub active_connections: i64,
    pub active_profile_name: String,
    pub active_node_name: String,
    pub ip_address: String,
    pub country: String,
    pub port: u16,
}

impl Default for DashboardStatus {
    fn default() -> Self {
        Self {
            status: DashboardRunStatus::Stopped,
            cpu_usage: 0.0,
            memory_usage: 0.0,
            uptime: 0,
            active_connections: 0,
            active_profile_name: "Global Relay".to_string(),
            active_node_name: "HK-01".to_string(),
            ip_address: "103.136.216.42".to_string(),
            country: "Hong Kong".to_string(),
            port: 7890,
        }
    }
}
