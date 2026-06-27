use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NetworkProtocol {
    TCP,
    UDP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    pub id: String,
    pub process: String,
    pub source: String,
    pub destination: String,
    pub rule: String,
    pub network: NetworkProtocol,
    pub upload: i64,
    pub download: i64,
    pub duration: f64,
    pub created_at: i64,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            id: String::new(),
            process: String::new(),
            source: String::new(),
            destination: String::new(),
            rule: String::new(),
            network: NetworkProtocol::TCP,
            upload: 0,
            download: 0,
            duration: 0.0,
            created_at: 0,
        }
    }
}
