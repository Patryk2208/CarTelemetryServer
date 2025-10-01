use std::sync::Arc;
use tokio::sync::Mutex;
use crate::processor::metric_manager::MetricManager;
use crate::server::flow_control::FlowControl;
use crate::server::server::Server;
use crate::server::websocket_interface::WebSocketServer;

pub async fn create_server(
    address: &str,
    metric_manager: Arc<Mutex<MetricManager>>
) -> Server {
    let websocket_server: WebSocketServer;
    match WebSocketServer::new(address).await {
        Ok(s) => websocket_server = s,
        Err(e) => panic!("Failed to create websocket server: {}", e)
    }
    
    let flow_control = FlowControl::new();
    
    Server {
        websocket_server,
        metric_manager,
        flow_control
    }
}