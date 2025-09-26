use std::collections::HashMap;
use std::time::Instant;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{Metrics, TelemetryValue};

pub struct BrakingSignal {
    pub metrics: HashMap<Metrics, f32>,
    pub timestamp: Instant
}

pub struct ProcessedBrakingSignal {
    pub g_force: f32,
    pub total_braking_time: Option<f32>,
    pub peak_brake_force: Option<f32>,
    pub total_braking_force: Option<f32>,
    pub timestamp: Instant
}

impl Telemetry for BrakingSignal {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) -> ProcessedTelemetry {
        crate::update_telemetry!(self, telemetry_value);

        ProcessedTelemetry::BrakingSignal(ProcessedBrakingSignal {})
    }
}