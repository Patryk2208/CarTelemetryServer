use std::collections::HashMap;
use std::time::Instant;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{MetricID, TelemetryValue};

pub struct SteeringResponse {
    pub metrics: HashMap<MetricID, f32>,
    pub timestamp: Instant
}

#[derive(Clone)]
pub struct ProcessedSteeringResponse {
    //todo
    pub timestamp: Instant
}

impl Telemetry for SteeringResponse {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) -> ProcessedTelemetry {
        crate::update_telemetry!(self, telemetry_value);
        //todo update steering response data
        ProcessedTelemetry::SteeringResponse(ProcessedSteeringResponse {})
    }
}