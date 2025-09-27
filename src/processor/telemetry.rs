use crate::processor::types::{TelemetryValue};
use crate::processor::adv_metric_balance::ProcessedBalance;
use crate::processor::adv_metric_steering_response::ProcessedSteeringResponse;
use crate::processor::adv_metric_braking_signal::ProcessedBrakingSignal;
use crate::processor::adv_metric_smoothness::ProcessedSmoothness;
use crate::processor::adv_metric_gg::ProcessedGG;

#[derive(Clone)]
pub enum ProcessedTelemetry {
    GG(ProcessedGG),
    Balance(ProcessedBalance),
    SteeringResponse(ProcessedSteeringResponse),
    BrakingSignal(ProcessedBrakingSignal),
    Smoothness(ProcessedSmoothness)
}

#[macro_export]
macro_rules! update_telemetry {
    ($self:ident, $telemetry_value:ident) => {
        $self.metrics
            .entry($telemetry_value.metric.clone())
            .and_modify(|v| *v = $telemetry_value.value)
            .or_insert($telemetry_value.value);
        let delta = ($telemetry_value.timestamp - $self.timestamp) / 2;
        $self.timestamp += delta;
    };
}

pub trait Telemetry {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) -> ProcessedTelemetry;
}