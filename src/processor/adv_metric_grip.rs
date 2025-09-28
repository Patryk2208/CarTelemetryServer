use std::collections::HashMap;
use std::time::Instant;
use crate::common::circular_buffer::CircularBuffer;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{MetricID, TelemetryValue};

pub struct Grip {
    pub metrics: HashMap<MetricID, f32>,
    pub timestamp: Instant,
    history: CircularBuffer<ProcessedGrip>
}

#[derive(Clone)]
pub struct ProcessedGrip {
    pub grip_force: f32,
    pub max_grip_per_corner: Option<f32>,
    pub max_grip_per_ride: f32,
    pub timestamp: Instant
}

impl Telemetry for Grip {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) -> ProcessedTelemetry {
        crate::update_telemetry!(self, telemetry_value);
        //todo update steering response data
        let p_s_r = ProcessedGrip {};
        
        self.history.push(p_s_r.clone());
        
        ProcessedTelemetry::Grip(p_s_r)
    }
}