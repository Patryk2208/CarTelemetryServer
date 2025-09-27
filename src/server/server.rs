use tokio::sync::broadcast;
use crate::processor::telemetry::ProcessedTelemetry;
use crate::server::metric_concatenation::MetricConcat;
use crate::server::websocket_interface::WebsocketInterface;

pub struct Server {
    pub broadcast_receiver: broadcast::Receiver<ProcessedTelemetry>,
    pub websocket_sender: WebsocketInterface,
    pub metric_concat: MetricConcat
}

impl Server {
    pub async fn run(&mut self) {
        while let Ok(message) = self.broadcast_receiver.recv().await {
            match message {
                ProcessedTelemetry::GG(m_gg) =>
                    self.metric_concat.gg_concat.append_telemetry(m_gg),
                ProcessedTelemetry::Balance(m_balance) =>
                    self.metric_concat.balance_concat.append_telemetry(m_balance),
                ProcessedTelemetry::SteeringResponse(m_steering) =>
                    self.metric_concat.steering_response_concat.append_telemetry(m_steering),
                ProcessedTelemetry::BrakingSignal(m_braking) =>
                    self.metric_concat.braking_signal_concat.append_telemetry(m_braking),
                ProcessedTelemetry::Smoothness(m_smoothness) =>
                    self.metric_concat.smoothness_concat.append_telemetry(m_smoothness)
            }
            
        }
    }
}