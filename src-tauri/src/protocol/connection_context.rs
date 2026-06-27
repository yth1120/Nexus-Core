use serde::Serialize;

/// Lightweight context for a connection being dispatched through the protocol
/// and transport layers. Phase 4 is a pure data struct; Phase 5 adds behavior.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionContext {
    pub id: String,
    pub source: String,
    pub destination: String,
    pub protocol: String, // "TCP" or "UDP"
    pub created_at: i64,  // Unix millis
}
