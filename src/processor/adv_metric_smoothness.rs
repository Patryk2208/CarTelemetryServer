use std::collections::HashMap;
use std::time::Instant;
use crate::common::circular_buffer::CircularBuffer;
use crate::processor::adv_metric_braking_signal::ProcessedBrakingSignal;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{MetricID, TelemetryValue};

pub struct Smoothness {
    pub metrics: HashMap<MetricID, f32>,
    pub timestamp: Instant,
    history: CircularBuffer<ProcessedSmoothness>
}

#[derive(Clone)]
pub struct ProcessedSmoothness {
    pub smoothness_index: f32,
    pub timestamp: Instant
}


impl Telemetry for Smoothness {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) -> ProcessedTelemetry {
        crate::update_telemetry!(self, telemetry_value);

        //todo update smoothness data

        let p_s = ProcessedSmoothness {};

        self.history.push(p_s.clone());

        ProcessedTelemetry::Smoothness(p_s)
    }
}