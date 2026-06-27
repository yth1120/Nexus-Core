use crate::core::AppState;
use crate::models::{DashboardStatus, TrafficDataPoint};

/// Get the current dashboard status.
pub fn get_dashboard_status(state: &AppState) -> DashboardStatus {
    state.dashboard_status.read().clone()
}

/// Get the traffic history buffer.
pub fn get_traffic_history(state: &AppState) -> Vec<TrafficDataPoint> {
    state.traffic_history.read().clone()
}
