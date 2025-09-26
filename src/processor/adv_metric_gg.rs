use std::collections::HashMap;
use std::time::Instant;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{Metrics, TelemetryValue};
use crate::processor::types::Metrics::{GForceLat, GForceLong, Speed};

pub struct GG {
    pub metrics: HashMap<Metrics, f32>,
    pub timestamp: Instant
}

pub struct ProcessedGG {
    pub current_g_force_long: f32,
    pub current_g_force_lat: f32,
    pub current_speed: f32,
    pub timestamp: Instant
}

impl Telemetry for GG {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) -> ProcessedTelemetry {
        crate::update_telemetry!(self, telemetry_value);

        let c_g_f_long = self.metrics.get(&GForceLong).unwrap_or(&0.0);
        let c_g_f_lat = self.metrics.get(&GForceLat).unwrap_or(&0.0);
        let speed = self.metrics.get(&Speed).unwrap_or(&0.0);
        ProcessedTelemetry::GG(ProcessedGG {
            current_g_force_long: c_g_f_long.clone(),
            current_g_force_lat: c_g_f_lat.clone(),
            current_speed: speed.clone(),
            timestamp: self.timestamp.clone()
        })
    }
}