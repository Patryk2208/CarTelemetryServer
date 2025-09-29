use crate::processor::telemetry::ProcessedTelemetry;
use crate::server::flow_control::FlowControl;
use crate::server::websocket_interface::WebSocketServer;

pub struct Server {
    pub websocket_sender: WebSocketServer,
    pub flow_control: FlowControl
}

impl Server {
    pub async fn run(&mut self) {

    }
}