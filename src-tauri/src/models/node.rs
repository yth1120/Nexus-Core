use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum NodeStatus {
    Online,
    Offline,
    Untested,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub name: String,
    pub country: String,
    pub country_code: String,
    pub delay: Option<i64>,
    pub loss: Option<f64>,
    pub status: NodeStatus,
    pub is_favorite: bool,
    pub is_connected: bool,
    #[serde(rename = "type")]
    pub node_type: String,
    pub group: String,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            country: String::new(),
            country_code: String::new(),
            delay: None,
            loss: None,
            status: NodeStatus::Untested,
            is_favorite: false,
            is_connected: false,
            node_type: String::new(),
            group: String::new(),
        }
    }
}
