use std::collections::HashMap;
use std::time::Instant;
use crate::processor::types::{Metrics, TelemetryValue};

pub struct GGDiagram {
    pub metrics: HashMap<Metrics, f32>,
    pub timestamp: Instant
}

pub struct Balance {
    pub metrics: HashMap<Metrics, f32>,
    pub expected_yaw_rate: f32,
    pub timestamp: Instant
}

pub struct SteeringResponse {
    pub metrics: HashMap<Metrics, f32>,
    pub yaw_rate: f32,
    pub timestamp: Instant
}

pub struct DerivedBrakingSignal {
    pub metrics: HashMap<Metrics, f32>,
    pub timestamp: Instant
}

pub struct Smoothness {
    pub metrics: HashMap<Metrics, f32>,
    pub timestamp: Instant
}
macro_rules! update_telemetry {
    ($self:ident, $telemetry_value:ident) => {
        $self.metrics
            .entry($telemetry_value.metric.clone())
            .and_modify(|v| *v = $telemetry_value.value)
            .or_insert($telemetry_value.value);
        let delta = ($telemetry_value.timestamp - $self.timestamp) / 2;
        $self.timestamp += delta;
    };
}

pub trait ProcessedTelemetry {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue);
}

impl ProcessedTelemetry for GGDiagram {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) {
        update_telemetry!(self, telemetry_value);
        //todo update GG data
    }
}
impl ProcessedTelemetry for Balance {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) {
        update_telemetry!(self, telemetry_value);
        //todo update balance data
    }
}
impl ProcessedTelemetry for SteeringResponse {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) {
        update_telemetry!(self, telemetry_value);
        //todo update steering response data
    }
}
impl ProcessedTelemetry for DerivedBrakingSignal {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) {
        update_telemetry!(self, telemetry_value);
        //todo update braking signal data
    }
}
impl ProcessedTelemetry for Smoothness {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) {
        update_telemetry!(self, telemetry_value);
        //todo update smoothness data
    }
}