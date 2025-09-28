use tokio::sync::broadcast;
use crate::processor::telemetry::ProcessedTelemetry;
use crate::server::flow_control::FlowControl;
use crate::server::metric_concatenation::MetricConcat;
use crate::server::websocket_interface::WebSocketServer;

pub struct Server {
    pub broadcast_receiver: broadcast::Receiver<ProcessedTelemetry>,
    pub metric_concat: MetricConcat,
    pub websocket_sender: WebSocketServer,
    pub flow_control: FlowControl
}

impl Server {
    pub async fn run(&mut self) {
        while let Ok(message) = self.broadcast_receiver.recv().await {
            match message {
                ProcessedTelemetry::GG(m_gg) =>
                    self.metric_concat.gg_concat.append_telemetry(m_gg),
                ProcessedTelemetry::Balance(m_balance) =>
                    self.metric_concat.balance_concat.append_telemetry(m_balance),
                ProcessedTelemetry::Grip(m_steering) =>
                    self.metric_concat.grip_concat.append_telemetry(m_steering),
                ProcessedTelemetry::BrakingSignal(m_braking) =>
                    self.metric_concat.braking_signal_concat.append_telemetry(m_braking),
                ProcessedTelemetry::Smoothness(m_smoothness) =>
                    self.metric_concat.smoothness_concat.append_telemetry(m_smoothness)
            }
            if self.flow_control.confirm_concat_decide_send() {
                match self.websocket_sender.send_telemetry(&self.metric_concat.prepare_message()).await {
                    Ok(()) => {}
                    Err(e) => {
                        println!("server failed: {}, closing connection", e);
                        break;
                    }
                }
                self.flow_control.confirm_message_sent()
            }
        }
    }
}