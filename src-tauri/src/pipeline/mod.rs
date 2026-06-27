// Packet pipeline — the data-plane MVP.
//
// Phase 6: real TCP listener → packet processor chain → echo.
// This is the first phase with actual data flow.

pub mod packet;
pub mod packet_context;
pub mod packet_dispatcher;
pub mod packet_pipeline;
pub mod packet_processor;
pub mod pipeline_manager;
pub mod pipeline_state;
pub mod processors;

pub use packet::Packet;
pub use packet_context::PacketContext;
pub use packet_dispatcher::PacketDispatcher;
pub use packet_pipeline::PacketPipeline;
pub use packet_processor::PacketProcessor;
pub use pipeline_manager::PipelineManager;
pub use pipeline_state::{PipelineState, PipelineStateCell};
pub use processors::{EchoProcessor, LogProcessor, StatisticsProcessor};
