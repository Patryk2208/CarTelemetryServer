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
            self.flow_control.start_iteration();
            let message: serde_json::Value;
            {
                let mut manager = self.metric_manager.lock().await;
                message = manager.get_message();
            }
            self.websocket_sender.send_telemetry(message).await.unwrap();
            self.flow_control.complete_iteration();
        }
    }
}