use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rule {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub rule_type: String,
    pub payload: String,
    pub proxy: String,
    pub enabled: bool,
    pub tags: Vec<String>,
    pub created_at: i64,
}
