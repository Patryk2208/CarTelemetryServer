use std::sync::Arc;
use tokio::sync::Mutex;
use crate::processor::metric_manager::MetricManager;
use crate::server::flow_control::FlowControl;
use crate::server::websocket_interface::WebSocketServer;

pub struct Server {
    pub websocket_sender: WebSocketServer,
    pub metric_manager: Arc<Mutex<MetricManager>>,
    pub flow_control: FlowControl
}

impl Server {
    pub async fn run(&mut self) {
        loop {

        }
    }
}