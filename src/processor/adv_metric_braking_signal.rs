use std::collections::HashMap;
use serde_json::json;
use crate::common::circular_buffer::CircularBuffer;
use crate::processor::telemetry::Telemetry;
use crate::processor::types::{MetricID, TelemetryValue, BRAKE_ON_OFF, G_LONG};

pub struct BrakingSignal {
    pub metrics: HashMap<MetricID, f32>,
    pub timestamp: u64,
    history: CircularBuffer<ProcessedBrakingSignal>,
    new_messages_since_last_concatenation: u16
}

#[derive(Clone)]
pub struct ProcessedBrakingSignal {
    pub g_force: f32,
    pub total_braking_time: Option<f32>,
    pub peak_brake_force: Option<f32>,
    pub timestamp: u64
}

impl BrakingSignal {
    pub fn new(history_size: usize) -> Self {
        let mut metrics = HashMap::new();
        metrics.insert(G_LONG, 0.0);
        metrics.insert(BRAKE_ON_OFF, 0.0);
        Self {
            metrics,
            timestamp: 0,
            history: CircularBuffer::new(history_size),
            new_messages_since_last_concatenation: 0
        }
    }
}

impl Telemetry for BrakingSignal {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) {
        crate::update_telemetry!(self, telemetry_value);

        let is_braking = self.metrics.get(&BRAKE_ON_OFF).unwrap_or(&0.0).eq(&1.0);
        let g_force = if is_braking { self.metrics.get(&G_LONG).unwrap_or(&0.0) } else { &0.0 };
        let mut total_braking_time = if is_braking { Some(0.0) } else { None };
        let mut peak_brake_force = if is_braking { Some(g_force.clone()) } else { None };

        let last = self.history.peek_newest();
        if last.is_some()
        {
            let was_braking = last.unwrap().total_braking_time.is_some();
            if is_braking {
                if was_braking {
                    total_braking_time = Some(last.unwrap().total_braking_time.unwrap() + ((telemetry_value.timestamp - last.unwrap().timestamp) as f32));
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

        self.history.push(p_b_s);
        self.new_messages_since_last_concatenation += 1;
    }

    fn produce_concatenated_message(&mut self) -> (String, serde_json::Value) {
        let mut concat_g_force = 0.0;
        let mut concat_total_braking_time = 0.0;
        let mut concat_peak_brake_force = 0.0;
        let mut concat_timestamp: u64 = 0;
        let mut count = 0;

        for p_b_s in self.history.iter_rev() {
            concat_g_force += p_b_s.g_force;
            concat_total_braking_time += p_b_s.total_braking_time.unwrap_or(0.0); //todo
            concat_peak_brake_force += p_b_s.peak_brake_force.unwrap_or(0.0); //todo
            concat_timestamp += p_b_s.timestamp;
            count += 1;

            if count > self.new_messages_since_last_concatenation {
                break;
            }
        }

        concat_g_force /= count as f32;
        concat_total_braking_time /= count as f32;
        concat_peak_brake_force /= count as f32;
        concat_timestamp /= count as u64;

        self.new_messages_since_last_concatenation = 0;

        (self.get_type(), json!({
            "g_force": concat_g_force,
            "total_braking_time": concat_total_braking_time,
            "peak_brake_force": concat_peak_brake_force,
            "timestamp": concat_timestamp
        })
        )
    }

    fn get_type(&self) -> String {
        String::from("braking_signal")
    }
}