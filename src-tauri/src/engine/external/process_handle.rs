use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use parking_lot::{Mutex, RwLock};
use tokio::process::Child;

pub struct ProcessHandle {
    pub child: Mutex<Option<Child>>,
    pub pid: RwLock<Option<u32>>,
    pub binary_path: String,
    pub config_path: String,
    pub started_at: RwLock<Option<i64>>,
    pub restart_count: AtomicU32,
}

impl ProcessHandle {
    pub fn new(binary: &str, config: &str) -> Arc<Self> {
        Arc::new(Self {
            child: Mutex::new(None),
            pid: RwLock::new(None),
            binary_path: binary.to_string(),
            config_path: config.to_string(),
            started_at: RwLock::new(None),
            restart_count: AtomicU32::new(0),
        })
    }

    pub fn spawned(&self, child: Child, pid: u32) {
        *self.child.lock() = Some(child);
        *self.pid.write() = Some(pid);
        *self.started_at.write() = Some(chrono::Utc::now().timestamp_millis());
    }

    pub fn killed(&self) {
        *self.child.lock() = None;
        *self.pid.write() = None;
    }

    pub fn is_running(&self) -> bool {
        let mut guard = self.child.lock();
        match *guard {
            Some(ref mut c) => c.try_wait().ok().flatten().is_none(),
            None => false,
        }
    }

    pub fn increment_restart(&self) {
        self.restart_count.fetch_add(1, Ordering::Relaxed);
    }
}
