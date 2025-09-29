use std::collections::HashMap;
use std::time::Instant;
use crate::common::circular_buffer::CircularBuffer;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{MetricID, TelemetryValue, BRAKE_ON_OFF, G_LONG};

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

        let is_braking = self.metrics.get(&BRAKE_ON_OFF).unwrap_or(&0.0).eq(&1.0);
        let g_force = if is_braking { self.metrics.get(&G_LONG).unwrap_or(&0.0) } else { &0.0 };
        let mut total_braking_time = if is_braking { Some(0.0f32) } else { None };
        let mut peak_brake_force = if is_braking { Some(g_force.clone()) } else { None };

        let last = self.history.peek_newest();
        if last.is_some()
        {
            let was_braking = last.unwrap().total_braking_time.is_some();
            if is_braking {
                if was_braking {
                    total_braking_time = Some((telemetry_value.timestamp - last.unwrap().timestamp).as_millis() as f32);
                    peak_brake_force = Some(peak_brake_force.unwrap().max(last.unwrap().peak_brake_force.unwrap()));
                }
            }
        }

        let p_b_s = ProcessedBrakingSignal {
            g_force: g_force.clone(),
            total_braking_time,
            peak_brake_force,
            timestamp: telemetry_value.timestamp.clone()
        };

        self.history.push(p_b_s.clone());

        ProcessedTelemetry::BrakingSignal(p_b_s)
    }
}