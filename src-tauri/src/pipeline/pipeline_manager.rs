use std::sync::Arc;

use crate::event::AppEvent;
use crate::runtime::RuntimeContext;
use crate::utils::AppResult;

use super::packet_dispatcher::PacketDispatcher;
use super::packet_pipeline::PacketPipeline;
use super::pipeline_state::{PipelineState, PipelineStateCell};
use super::processors::{EchoProcessor, LogProcessor, StatisticsProcessor};

/// Orchestrates the packet pipeline lifecycle: processor registration,
/// dispatcher wiring, and state management.
pub struct PipelineManager {
    #[allow(dead_code)]
    context: Arc<RuntimeContext>,
    pipeline: Arc<PacketPipeline>,
    dispatcher: Arc<PacketDispatcher>,
    statistics: Arc<StatisticsProcessor>,
    state: PipelineStateCell,
}

impl PipelineManager {
    /// Build the manager with three default processors pre-registered:
    /// Echo (identity), Statistics (counters), Log (tracing).
    pub fn new(context: Arc<RuntimeContext>) -> Self {
        let pipeline = Arc::new(PacketPipeline::new());
        let statistics = Arc::new(StatisticsProcessor::new());

        pipeline.add_processor(Arc::new(EchoProcessor));
        pipeline.add_processor(Arc::new(StatisticsProcessor::new()));
        pipeline.add_processor(Arc::new(LogProcessor::new((*context).clone())));

        let dispatcher = Arc::new(PacketDispatcher::new(pipeline.clone(), context.clone()));

        Self {
            context,
            pipeline,
            dispatcher,
            statistics,
            state: PipelineStateCell::new(),
        }
    }

    pub async fn initialize(&self) -> AppResult<()> {
        tracing::info!(
            "PipelineManager initialized ({} processors)",
            self.pipeline.len()
        );
        Ok(())
    }

    pub async fn start(&self) -> AppResult<()> {
        if self.state.is_running() {
            return Ok(());
        }
        self.set_state(PipelineState::Starting);
        self.set_state(PipelineState::Running);
        self.context.publish(AppEvent::PipelineStarted);
        tracing::info!("PipelineManager running");
        Ok(())
    }

    pub async fn stop(&self) -> AppResult<()> {
        if matches!(self.state.get(), PipelineState::Stopped) {
            return Ok(());
        }
        self.set_state(PipelineState::Stopping);
        self.set_state(PipelineState::Stopped);
        self.context.publish(AppEvent::PipelineStopped);
        tracing::info!("PipelineManager stopped");
        Ok(())
    }

    pub async fn restart(&self) -> AppResult<()> {
        self.stop().await?;
        self.start().await
    }

    pub fn status(&self) -> PipelineState {
        self.state.get()
    }

    /// The processor chain (for injection into the engine accept loop).
    pub fn pipeline(&self) -> Arc<PacketPipeline> {
        self.pipeline.clone()
    }

    /// The dispatcher (for direct use outside the manager).
    pub fn dispatcher(&self) -> Arc<PacketDispatcher> {
        self.dispatcher.clone()
    }

    /// Snapshot of packet statistics.
    pub fn statistics_snapshot(&self) -> (u64, u64, u64) {
        self.statistics.snapshot()
    }

    fn set_state(&self, next: PipelineState) {
        self.state.set(next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn lifecycle_start_stop() -> AppResult<()> {
        let rt = RuntimeContext::new_for_test()?;
        let mgr = PipelineManager::new(rt);

        assert_eq!(mgr.status(), PipelineState::Stopped);
        mgr.start().await?;
        assert_eq!(mgr.status(), PipelineState::Running);
        mgr.stop().await?;
        assert_eq!(mgr.status(), PipelineState::Stopped);
        Ok(())
    }

    #[tokio::test]
    async fn statistics_counters() -> AppResult<()> {
        let rt = RuntimeContext::new_for_test()?;
        let mgr = PipelineManager::new(rt);
        let pipeline = mgr.pipeline();

        let ctx = crate::pipeline::PacketContext::new(
            std::net::SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST), 1),
            std::net::SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST), 2),
            "TCP",
        );
        let pkt = crate::pipeline::Packet::Tcp(bytes::Bytes::from_static(b"data"));
        pipeline.execute(&ctx, pkt).await?;

        let (count, bytes_in, bytes_out) = mgr.statistics_snapshot();
        assert_eq!(count, 1);
        assert_eq!(bytes_in, 4);
        assert_eq!(bytes_out, 4);
        Ok(())
    }
}
