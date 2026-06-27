use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CrashReport {
    pub panic_msg: String,
    pub backtrace: String,
    pub runtime_state: String,
    pub timestamp: i64,
}

impl CrashReport {
    pub fn new(panic_msg: &str) -> Self {
        Self {
            panic_msg: panic_msg.into(),
            backtrace: String::new(),
            runtime_state: String::new(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn creates_report() {
        let r = CrashReport::new("test panic");
        assert!(r.to_json().contains("test panic"));
    }
}
