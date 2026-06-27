use std::sync::Arc;
use tokio::process::Command;

use crate::utils::{AppError, AppResult};

use super::external::{ProcessConfig, ProcessHandle};

pub struct ProcessManager;

impl ProcessManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn spawn(&self, config: &ProcessConfig) -> AppResult<Arc<ProcessHandle>> {
        let mut cmd = Command::new(&config.binary_path);
        cmd.args(&config.args);
        for (k, v) in &config.envs {
            cmd.env(k, v);
        }
        cmd.current_dir(&config.work_dir);
        cmd.kill_on_drop(true);

        let child = cmd
            .spawn()
            .map_err(|e| AppError::Io(format!("spawn {} failed: {e}", config.binary_path)))?;
        let pid = child
            .id()
            .ok_or_else(|| AppError::Internal("no pid from child".into()))?;
        let handle = ProcessHandle::new(&config.binary_path, &config.config_path);
        handle.spawned(child, pid);
        tracing::info!(
            "ProcessManager spawned {} (pid={})",
            config.binary_path,
            pid
        );
        Ok(handle)
    }

    pub async fn kill(&self, handle: &ProcessHandle) -> AppResult<()> {
        let c = {
            let mut guard = handle.child.lock();
            guard.take()
        }; // guard dropped
        if let Some(mut c) = c {
            c.start_kill()
                .map_err(|e| AppError::Io(format!("kill error: {e}")))?;
            c.wait().await.ok();
        }
        handle.killed();
        Ok(())
    }

    pub async fn restart(
        &self,
        handle: &ProcessHandle,
        config: &ProcessConfig,
    ) -> AppResult<Arc<ProcessHandle>> {
        self.kill(handle).await?;
        handle.increment_restart();
        self.spawn(config).await
    }

    pub async fn wait(&self, handle: &ProcessHandle) -> AppResult<std::process::ExitStatus> {
        let mut child =
            { handle.child.lock().take() }.ok_or_else(|| AppError::Internal("no child".into()))?;
        let status = child
            .wait()
            .await
            .map_err(|e| AppError::Io(format!("wait error: {e}")))?;
        Ok(status)
    }

    pub fn status(&self, handle: &ProcessHandle) -> AppResult<Option<std::process::ExitStatus>> {
        let mut guard = handle.child.lock();
        let result = match guard.as_mut() {
            Some(c) => c
                .try_wait()
                .map_err(|e| AppError::Io(format!("try_wait error: {e}"))),
            None => Ok(None),
        };
        result
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn spawn_and_kill_echo() -> AppResult<()> {
        let pm = ProcessManager::new();
        let mut cfg = ProcessConfig::new("cmd", "none");
        cfg.args = vec!["/c".into(), "echo".into(), "hello".into()];
        let h = pm.spawn(&cfg).await?;
        let _pid = *h.pid.read();
        pm.kill(&h).await?;
        assert!(!h.is_running());
        Ok(())
    }
}
