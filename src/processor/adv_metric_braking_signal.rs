use std::collections::HashMap;
use std::time::Instant;
use crate::common::circular_buffer::CircularBuffer;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{MetricID, TelemetryValue};

pub struct BrakingSignal {
    pub metrics: HashMap<MetricID, f32>,
    pub timestamp: Instant,
    history: CircularBuffer<ProcessedBrakingSignal>
}

#[derive(Clone)]
pub struct ProcessedBrakingSignal {
    pub g_force: f32,
    pub total_braking_time: Option<f32>,
    pub peak_brake_force: Option<f32>,
    pub timestamp: Instant
}

impl Telemetry for BrakingSignal {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) -> ProcessedTelemetry {
        crate::update_telemetry!(self, telemetry_value);

        //todo calculate necessary continuities and derive current metrics

        let p_b_s = ProcessedBrakingSignal {};

        self.history.push(p_b_s.clone());

        ProcessedTelemetry::BrakingSignal(p_b_s)
    }
}