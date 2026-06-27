use std::sync::Arc;

use parking_lot::RwLock;

use crate::runtime::RuntimeContext;

use super::packet_injector::PacketInjector;
use super::packet_receiver::PacketReceiver;
use super::route_table::{NullRouteTable, RouteTable};
use super::tun_device::{NullTunDevice, TunDevice};
use super::tun_route::RouteManager;

pub struct TunContext {
    pub runtime: Arc<RuntimeContext>,
    pub device: RwLock<Arc<dyn TunDevice>>,
    pub route_manager: Arc<RouteManager>,
    pub packet_receiver: Arc<PacketReceiver>,
    pub packet_injector: Arc<PacketInjector>,
}

impl TunContext {
    pub fn new(runtime: Arc<RuntimeContext>) -> Self {
        let device: Arc<dyn TunDevice> = Arc::new(NullTunDevice);
        let table: Arc<dyn RouteTable> = Arc::new(NullRouteTable);
        let route_manager = Arc::new(RouteManager::new(runtime.clone(), table));
        let packet_injector = Arc::new(PacketInjector::new(device.clone()));
        let packet_receiver = Arc::new(PacketReceiver::new(device.clone()));
        Self {
            runtime,
            device: RwLock::new(device),
            route_manager,
            packet_receiver,
            packet_injector,
        }
    }
}

#[cfg(test)]
impl TunContext {
    pub(crate) fn new_for_test(runtime: Arc<RuntimeContext>) -> crate::utils::AppResult<Arc<Self>> {
        Ok(Arc::new(Self::new(runtime)))
    }
}
