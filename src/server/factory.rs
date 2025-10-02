use std::sync::Arc;
use tokio::sync::Mutex;
use crate::processor::metric_manager::MetricManager;
use crate::server::flow_control::FlowControl;
use crate::server::server::Server;
use crate::server::websocket_interface::WebSocketServer;

pub async fn create_server(
    address: &str,
    metric_manager: Arc<Mutex<MetricManager>>
) -> Result<Server, String> {
    let websocket_server = WebSocketServer::new(address).await
        .map_err(|e| format!("Failed to create websocket server: {}", e))?;
    
    let flow_control = FlowControl::new();
    
    Ok(Server {
        websocket_server,
        metric_manager,
        flow_control
    })
}