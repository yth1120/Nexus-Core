use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LogLevel {
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

impl LogLevel {
    /// Weight used for weighted random selection.
    /// INFO=50%, DEBUG=25%, WARN=15%, ERROR=5%, TRACE=5%
    pub fn weight(&self) -> u32 {
        match self {
            LogLevel::TRACE => 5,
            LogLevel::DEBUG => 25,
            LogLevel::INFO => 50,
            LogLevel::WARN => 15,
            LogLevel::ERROR => 5,
        }
    }

    pub fn all() -> Vec<LogLevel> {
        vec![
            LogLevel::TRACE,
            LogLevel::DEBUG,
            LogLevel::INFO,
            LogLevel::WARN,
            LogLevel::ERROR,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    pub id: String,
    pub timestamp: i64,
    pub level: LogLevel,
    pub message: String,
}

impl Default for LogEntry {
    fn default() -> Self {
        Self {
            id: String::new(),
            timestamp: 0,
            level: LogLevel::INFO,
            message: String::new(),
        }
    }
}
