use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use crate::processor::metric_manager::MetricManager;
use crate::server::flow_control::{FlowControl, RefreshRate};
use crate::server::network_manager::NetworkManager;
use crate::server::server::{Server, MetricSender};

pub fn create_server(
    metric_manager: Arc<Mutex<MetricManager>>,
    shutdown: broadcast::Receiver<()>
) -> Server {
    let metric_sender = MetricSender {
        metric_manager,
        flow_control: FlowControl::new(RefreshRate{rate: RefreshRate::FAST})
    };
    Server::new(metric_sender, shutdown)
}

pub fn create_network_manager(
    server: Server,
    target_ssid: &str,
) -> NetworkManager {
    NetworkManager::new(server, target_ssid, 8080, 1)
}