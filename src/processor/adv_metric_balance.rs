use std::collections::HashMap;
use serde_json::json;
use crate::common::circular_buffer::CircularBuffer;
use crate::processor::telemetry::Telemetry;
use crate::processor::types::{MetricID, TelemetryValue, G_LAT, SPEED, YAW};

const GRAVITATIONAL_ACCELERATION_EARTH: f32 = 9.81;
const EPS: f32 = 0.001;
const PI: f32 = std::f32::consts::PI;

pub struct Balance {
    pub metrics: HashMap<MetricID, f32>,
    pub timestamp: u64,
    history: CircularBuffer<ProcessedBalance>,
    new_messages_since_last_concatenation: u16
}

#[derive(Clone)]
pub struct ProcessedBalance {
    pub balance_index: f32,
    pub timestamp: u64
}

impl Balance {
    pub fn new(history_size: usize) -> Self {
        let mut metrics = HashMap::new();
        metrics.insert(G_LAT, 0.0);
        metrics.insert(SPEED, 0.0);
        metrics.insert(YAW, 0.0);
        Self {
            metrics,
            timestamp: 0,
            history: CircularBuffer::new(history_size),
            new_messages_since_last_concatenation: 0
        }
    }}

impl Telemetry for Balance {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) {
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
        
        self.history.push(p_b);
        self.new_messages_since_last_concatenation += 1;
    }

    fn produce_concatenated_message(&mut self) -> (String, serde_json::Value) {
        let mut concat_balance_index = 0.0;
        let mut concat_timestamp: u64 = 0;
        let mut count = 0;

        for p_b in self.history.iter_rev() {
            concat_balance_index += p_b.balance_index;
            concat_timestamp += p_b.timestamp;
            count += 1;

            if count > self.new_messages_since_last_concatenation {
                break;
            }
        }
        
        if count == 0 {
            return (self.get_type(), json!({
                "balance_index": 0.0,
                "timestamp": 0
            }));
        }

        concat_balance_index /= count as f32;
        concat_timestamp /= count as u64;

        self.new_messages_since_last_concatenation = 0;

        (self.get_type(), json!({
            "balance_index": concat_balance_index,
            "timestamp": concat_timestamp
        })
        )
    }
    
    fn get_type(&self) -> String {
        String::from("balance")
    }
}