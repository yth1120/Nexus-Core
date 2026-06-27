use std::sync::Arc;

use parking_lot::RwLock;

use crate::utils::AppResult;

use super::packet::Packet;
use super::packet_context::PacketContext;
use super::packet_processor::PacketProcessor;

/// An ordered chain of [`PacketProcessor`]s executed in sequence.
///
/// Processors are stored as `Arc<dyn PacketProcessor>` so the chain can be
/// cloned out of the lock before iteration — never holding a lock across
/// an `.await`.
pub struct PacketPipeline {
    processors: RwLock<Vec<Arc<dyn PacketProcessor>>>,
}

impl PacketPipeline {
    pub fn new() -> Self {
        Self {
            processors: RwLock::new(Vec::new()),
        }
    }

    pub fn add_processor(&self, processor: Arc<dyn PacketProcessor>) {
        self.processors.write().push(processor);
    }

    pub fn remove_processor(&self, name: &str) -> bool {
        let mut guard = self.processors.write();
        let pos = guard.iter().position(|p| p.name() == name);
        if let Some(idx) = pos {
            guard.remove(idx);
            true
        } else {
            false
        }
    }

    pub fn is_empty(&self) -> bool {
        self.processors.read().is_empty()
    }

    pub fn len(&self) -> usize {
        self.processors.read().len()
    }

    /// Run a packet through all registered processors in order.
    pub async fn execute(&self, ctx: &PacketContext, packet: Packet) -> AppResult<Packet> {
        let procs: Vec<Arc<dyn PacketProcessor>> = self.processors.read().clone();
        let mut current = packet;
        for proc in &procs {
            current = proc.process(ctx, current).await?;
        }
        Ok(current)
    }
}

impl Default for PacketPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use bytes::Bytes;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};
    use std::sync::Arc;

    struct AppendProcessor(&'static str);

    #[async_trait]
    impl PacketProcessor for AppendProcessor {
        async fn process(&self, _ctx: &PacketContext, packet: Packet) -> AppResult<Packet> {
            let mut data = packet.payload().to_vec();
            data.extend_from_slice(self.0.as_bytes());
            Ok(Packet::Tcp(Bytes::from(data)))
        }

        fn name(&self) -> &'static str {
            self.0
        }
    }

    #[tokio::test]
    async fn execute_runs_processors_in_order() -> AppResult<()> {
        let pipeline = PacketPipeline::new();
        pipeline.add_processor(Arc::new(AppendProcessor("-A")));
        pipeline.add_processor(Arc::new(AppendProcessor("-B")));
        assert_eq!(pipeline.len(), 2);

        let ctx = PacketContext::new(
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 1),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 2),
            "TCP",
        );
        let pkt = Packet::Tcp(Bytes::from_static(b"X"));
        let result = pipeline.execute(&ctx, pkt).await?;
        assert_eq!(result.payload(), b"X-A-B");
        Ok(())
    }

    #[tokio::test]
    async fn remove_processor_by_name() -> AppResult<()> {
        let pipeline = PacketPipeline::new();
        pipeline.add_processor(Arc::new(AppendProcessor("-keep")));
        assert_eq!(pipeline.len(), 1);
        assert!(pipeline.remove_processor("-keep"));
        assert!(pipeline.is_empty());
        assert!(!pipeline.remove_processor("nonexistent"));
        Ok(())
    }
}
