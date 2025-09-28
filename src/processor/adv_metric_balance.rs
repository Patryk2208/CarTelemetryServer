use std::collections::HashMap;
use std::time::Instant;
use crate::common::circular_buffer::CircularBuffer;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{MetricID, TelemetryValue, G_LAT, SPEED, YAW};

const GRAVITATIONAL_ACCELERATION_EARTH: f32 = 9.81;
const EPS: f32 = 0.001;
const PI: f32 = std::f32::consts::PI;

pub struct Balance {
    pub metrics: HashMap<MetricID, f32>,
    pub timestamp: Instant,
    history: CircularBuffer<ProcessedBalance>
}

#[derive(Clone)]
pub struct ProcessedBalance {
    pub balance_index: f32,
    pub timestamp: Instant
}

impl Telemetry for Balance {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) -> ProcessedTelemetry {
        crate::update_telemetry!(self, telemetry_value);
        
        let raw_yaw_rate = self.metrics.get(&YAW).unwrap_or(&0.0);
        let lat_g = self.metrics.get(&G_LAT).unwrap_or(&0.0);
        let speed_ms = self.metrics.get(&SPEED).unwrap_or(&0.0) / 3.6;
        let expected_yaw_rate = (lat_g * GRAVITATIONAL_ACCELERATION_EARTH * 180.0)
            / (speed_ms * PI);
        let balance_index = (raw_yaw_rate - expected_yaw_rate)
            / (raw_yaw_rate.abs() + expected_yaw_rate.abs() + EPS);
        
        let p_b = ProcessedBalance {
            balance_index: balance_index.clone(),
            timestamp: self.timestamp.clone()
        };
        
        self.history.push(p_b.clone());
        
        ProcessedTelemetry::Balance(p_b)
    }
}