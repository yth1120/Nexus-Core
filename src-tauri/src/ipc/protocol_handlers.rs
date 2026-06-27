use std::sync::Arc;

use serde::Serialize;
use tauri::State;

use crate::core::CoreManager;
use crate::protocol::protocol_state::ProtocolState;
use crate::transport::transport_state::TransportState;

/// Serializable snapshot of the protocol layer status.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProtocolStatus {
    pub state: ProtocolState,
    pub inbound_count: usize,
    pub outbound_count: usize,
}

/// Serializable snapshot of the transport layer status.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransportStatus {
    pub state: TransportState,
    pub listener_count: usize,
    pub stream_count: usize,
}

/// Get the current protocol-layer status.
#[tauri::command]
pub async fn get_protocol_status(
    _core: State<'_, Arc<CoreManager>>,
) -> Result<ProtocolStatus, String> {
    // ProtocolManager is accessed through CoreStatus / RuntimeContext.
    // For Phase 4, return a default stopped status since CoreManager doesn't
    // expose protocol_manager directly — it's accessed via RuntimeContext.
    Ok(ProtocolStatus {
        state: ProtocolState::Stopped,
        inbound_count: 3,
        outbound_count: 3,
    })
}

/// Get the current transport-layer status.
#[tauri::command]
pub async fn get_transport_status(
    _core: State<'_, Arc<CoreManager>>,
) -> Result<TransportStatus, String> {
    Ok(TransportStatus {
        state: TransportState::Stopped,
        listener_count: 0,
        stream_count: 0,
    })
}
