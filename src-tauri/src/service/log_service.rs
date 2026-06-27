use crate::core::AppState;
use crate::models::{LogEntry, LogLevel};

/// Get all logs, optionally filtered by level.
pub fn get_all(state: &AppState, level_filter: Option<String>) -> Vec<LogEntry> {
    let logs = state.logs.read();

    match level_filter.as_deref() {
        None | Some("ALL") => logs.clone(),
        Some(filter) => logs
            .iter()
            .filter(|entry| {
                let level_str = match entry.level {
                    LogLevel::TRACE => "TRACE",
                    LogLevel::DEBUG => "DEBUG",
                    LogLevel::INFO => "INFO",
                    LogLevel::WARN => "WARN",
                    LogLevel::ERROR => "ERROR",
                };
                level_str == filter
            })
            .cloned()
            .collect(),
    }
}

/// Get the most recent N log entries.
pub fn get_recent(state: &AppState, limit: usize) -> Vec<LogEntry> {
    let logs = state.logs.read();
    let start = if logs.len() > limit {
        logs.len() - limit
    } else {
        0
    };
    logs[start..].to_vec()
}
