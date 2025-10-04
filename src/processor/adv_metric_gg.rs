use std::cmp::min;
use std::collections::HashMap;
use serde_json::json;
use crate::common::circular_buffer::CircularBuffer;
use crate::processor::telemetry::Telemetry;
use crate::processor::types::{MetricID, TelemetryValue, G_LAT, G_LONG, SPEED};

pub struct GG {
    pub metrics: HashMap<MetricID, f32>,
    pub timestamp: u64,
    history: CircularBuffer<ProcessedGG>,
    new_messages_since_last_concatenation: u16
}

#[derive(Clone)]
pub struct ProcessedGG {
    pub g_force_long: f32,
    pub g_force_lat: f32,
    pub speed: f32,
    pub timestamp: u64
}

impl GG {
    pub fn new(history_size: usize) -> Self {
        let mut metrics = HashMap::new();
        metrics.insert(G_LONG, 0.0);
        metrics.insert(G_LAT, 0.0);
        metrics.insert(SPEED, 0.0);
        Self {
            metrics,
            timestamp: 0,
            history: CircularBuffer::new(history_size),
            new_messages_since_last_concatenation: 0
        }
    }}

impl Telemetry for GG {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) {
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
        
        self.history.push(p_gg);
        self.new_messages_since_last_concatenation = min(
            self.new_messages_since_last_concatenation + 1, self.history.capacity() as u16);
    }

    fn produce_concatenated_message(&mut self) -> (String, serde_json::Value) {
        let mut concat_g_force_long = 0.0;
        let mut concat_g_force_lat = 0.0;
        let mut concat_speed = 0.0;
        let mut concat_timestamp: u64 = 0;
        let mut count = 0;

        for p_gg in self.history.iter_rev() {
            if count >= self.new_messages_since_last_concatenation {
                break;
            }
            
            concat_g_force_long += p_gg.g_force_long;
            concat_g_force_lat += p_gg.g_force_lat;
            concat_speed += p_gg.speed;
            concat_timestamp += p_gg.timestamp;
            count += 1;
        }

        if count == 0 {
            return (self.get_type(), json!({
                "g_force_long": 0.0,
                "g_force_lat": 0.0,
                "speed": 0.0,
                "timestamp": 0
            }));
        }

        concat_g_force_long /= count as f32;
        concat_g_force_lat /= count as f32;
        concat_speed /= count as f32;
        concat_timestamp /= count as u64;

        self.new_messages_since_last_concatenation = 0;

        (self.get_type(), json!({
            "g_force_long": concat_g_force_long,
            "g_force_lat": concat_g_force_lat,
            "speed": concat_speed,
            "timestamp": concat_timestamp
        })
        )
    }

    fn get_type(&self) -> String {
        String::from("gg")
    }
}
