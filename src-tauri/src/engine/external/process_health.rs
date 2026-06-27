use std::sync::Arc;

use crate::utils::AppResult;

use super::process_handle::ProcessHandle;

pub struct ProcessHealth {
    handle: Arc<ProcessHandle>,
    #[allow(dead_code)]
    api_url: Option<String>,
}

impl ProcessHealth {
    pub fn new(handle: Arc<ProcessHandle>, api_url: Option<String>) -> Self {
        Self { handle, api_url }
    }

    pub fn is_alive(&self) -> bool {
        self.handle.is_running()
    }

    pub fn check(&self) -> AppResult<()> {
        if self.handle.is_running() {
            Ok(())
        } else {
            Err(crate::utils::AppError::Internal(
                "process not running".into(),
            ))
        }
    }

    pub fn restart_count(&self) -> u32 {
        self.handle
            .restart_count
            .load(std::sync::atomic::Ordering::Relaxed)
    }
}
