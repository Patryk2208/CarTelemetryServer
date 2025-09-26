use std::collections::HashMap;
use std::path::absolute;
use std::time::Instant;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{Metrics, TelemetryValue};
use crate::processor::types::Metrics::{GForceLat, Speed, YawRate};

const GRAVITATIONAL_ACCELERATION_EARTH: f32 = 9.81;
const EPS: f32 = 0.001;
const PI: f32 = std::f32::consts::PI;

pub struct Balance {
    pub metrics: HashMap<Metrics, f32>,
    pub timestamp: Instant
}

pub struct ProcessedBalance {
    pub balance_index: f32,
    pub timestamp: Instant
}

impl Telemetry for Balance {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) -> ProcessedTelemetry {
        crate::update_telemetry!(self, telemetry_value);
        let raw_yaw_rate = self.metrics.get(&YawRate).unwrap_or(&0.0);
        let lat_g = self.metrics.get(&GForceLat).unwrap_or(&0.0);
        let speed_ms = self.metrics.get(&Speed).unwrap_or(&0.0) / 3.6;
        let expected_yaw_rate = (lat_g * GRAVITATIONAL_ACCELERATION_EARTH * 180.0)
            / (speed_ms * PI);
        let balance_index = (raw_yaw_rate - expected_yaw_rate)
            / (raw_yaw_rate.abs() + expected_yaw_rate.abs() + EPS);
        ProcessedTelemetry::Balance(ProcessedBalance {
            balance_index: balance_index.clone(),
            timestamp: self.timestamp.clone()
        })
    }
}