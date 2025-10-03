use std::sync::Arc;
use tokio::sync::Mutex;
use crate::processor::metric_manager::MetricManager;
use crate::server::flow_control::FlowControl;
use crate::server::server::{Server, MetricSender};

pub async fn create_server(
    address: &str,
    metric_manager: Arc<Mutex<MetricManager>>
) -> Result<Server, String> {
    let metric_sender = MetricSender {
        metric_manager,
        flow_control: FlowControl::new()
    };
    match Server::new(address, metric_sender).await {
        Ok(server) => Ok(server),
        Err(e) => Err(e.to_string())
    }
}