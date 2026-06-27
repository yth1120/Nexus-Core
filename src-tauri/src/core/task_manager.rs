use std::collections::HashMap;
use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::utils::{AppError, AppResult};

/// Reported status of a managed background task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    Running,
    Stopped,
    Error,
}

/// A handle to a running task, allowing stop and status queries.
#[derive(Debug, Clone)]
pub struct TaskHandle {
    id: String,
    shutdown_flag: Arc<AtomicBool>,
    status: Arc<RwLock<TaskStatus>>,
}

impl TaskHandle {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn status(&self) -> TaskStatus {
        *self.status.read()
    }

    pub fn is_running(&self) -> bool {
        matches!(*self.status.read(), TaskStatus::Running)
    }

    /// Signal the task to stop. The task's shutdown flag becomes true;
    /// the task's Future should exit promptly when it observes this.
    pub fn stop(&self) {
        self.shutdown_flag.store(true, Ordering::SeqCst);
        *self.status.write() = TaskStatus::Stopped;
    }
}

/// Manages the lifecycle of named background tasks.
///
/// Replaces the scattered `tokio::spawn` + `AtomicBool` pattern
/// with structured start/stop/restart/shutdown semantics.
#[derive(Clone)]
pub struct TaskManager {
    tasks: Arc<RwLock<HashMap<String, TaskHandle>>>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register and start a background task.
    ///
    /// The closure receives an `Arc<AtomicBool>` shutdown flag.
    /// The Future should poll this flag and exit promptly when true.
    pub fn spawn<F, Fut>(&self, id: impl Into<String>, task_fn: F) -> TaskHandle
    where
        F: FnOnce(Arc<AtomicBool>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let id = id.into();
        let shutdown_flag = Arc::new(AtomicBool::new(false));
        let status = Arc::new(RwLock::new(TaskStatus::Running));

        let handle = TaskHandle {
            id: id.clone(),
            shutdown_flag: shutdown_flag.clone(),
            status: status.clone(),
        };

        // Spawn the task
        let flag = shutdown_flag.clone();
        let task_status = status.clone();
        let task_id = id.clone();
        tokio::spawn(async move {
            task_fn(flag).await;
            // If the task exits on its own (not via stop), mark as Stopped
            let mut s = task_status.write();
            if matches!(*s, TaskStatus::Running) {
                *s = TaskStatus::Stopped;
            }
            tracing::info!("Task '{}' exited", task_id);
        });

        self.tasks.write().insert(id, handle.clone());
        handle
    }

    /// Stop a task by ID and remove it from the registry.
    pub fn stop(&self, id: &str) -> AppResult<()> {
        let mut tasks = self.tasks.write();
        if let Some(handle) = tasks.remove(id) {
            handle.stop();
            Ok(())
        } else {
            Err(AppError::NotFound(format!("Task '{}' not found", id)))
        }
    }

    /// Stop and re-spawn a task with a new closure.
    pub fn restart<F, Fut>(&self, id: impl Into<String>, task_fn: F) -> AppResult<TaskHandle>
    where
        F: FnOnce(Arc<AtomicBool>) -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let id_str: String = id.into();
        // Stop existing if present
        let _ = self.stop(&id_str);
        // Spawn fresh
        Ok(self.spawn(id_str, task_fn))
    }

    /// Get the status of a task by ID.
    pub fn status(&self, id: &str) -> Option<TaskStatus> {
        self.tasks.read().get(id).map(|h| h.status())
    }

    /// List all registered tasks with their statuses.
    pub fn list(&self) -> Vec<(String, TaskStatus)> {
        self.tasks
            .read()
            .iter()
            .map(|(id, h)| (id.clone(), h.status()))
            .collect()
    }

    /// Stop all tasks. Used during graceful shutdown.
    /// Does NOT wait for tasks to complete — callers should
    /// use a brief delay or join handles if needed.
    pub fn shutdown_all(&self) {
        let tasks: Vec<TaskHandle> = self.tasks.read().values().cloned().collect();
        for handle in tasks {
            handle.stop();
        }
        self.tasks.write().clear();
        tracing::info!("All tasks shut down");
    }

    /// Return the number of registered tasks.
    pub fn len(&self) -> usize {
        self.tasks.read().len()
    }

    /// Return true if no tasks are registered.
    pub fn is_empty(&self) -> bool {
        self.tasks.read().is_empty()
    }
}

impl Default for TaskManager {
    fn default() -> Self {
        Self::new()
    }
}
