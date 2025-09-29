use std::collections::HashMap;
use std::time::Instant;
use crate::common::circular_buffer::CircularBuffer;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{MetricID, TelemetryValue, G_LAT, G_LONG, SPEED};

pub struct GG {
    pub metrics: HashMap<MetricID, f32>,
    pub timestamp: Instant,
    history: CircularBuffer<ProcessedGG>
}

#[derive(Clone)]
pub struct ProcessedGG {
    pub g_force_long: f32,
    pub g_force_lat: f32,
    pub speed: f32,
    pub timestamp: Instant
}

impl Telemetry for GG {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) -> ProcessedTelemetry {
        crate::update_telemetry!(self, telemetry_value);

        let c_g_f_long = self.metrics.get(&G_LONG).unwrap_or(&0.0);
        let c_g_f_lat = self.metrics.get(&G_LAT).unwrap_or(&0.0);
        let speed = self.metrics.get(&SPEED).unwrap_or(&0.0);
        
        let p_gg = ProcessedGG{
            g_force_long: c_g_f_long.clone(),
            g_force_lat: c_g_f_lat.clone(),
            speed: speed.clone(),
            timestamp: self.timestamp.clone()
        };
        
        self.history.push(p_gg.clone());
        
        ProcessedTelemetry::GG(p_gg)
    }
}
