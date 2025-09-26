use std::collections::HashMap;
use std::time::Instant;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{Metrics, TelemetryValue};

pub struct Smoothness {
    pub metrics: HashMap<Metrics, f32>,
    pub timestamp: Instant
}

pub struct ProcessedSmoothness {
    //todo
    pub timestamp: Instant
}


impl Telemetry for Smoothness {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) -> ProcessedTelemetry {
        crate::update_telemetry!(self, telemetry_value);
        //todo update smoothness data
        ProcessedTelemetry::Smoothness(ProcessedSmoothness{})
    }
}