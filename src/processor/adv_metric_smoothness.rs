use std::collections::HashMap;
use crate::common::circular_buffer::CircularBuffer;
use crate::processor::telemetry::Telemetry;
use crate::processor::types::{MetricID, TelemetryValue, G_LAT, G_LONG, SPEED, YAW};

const SMOOTHNESS_ALPHA: f32 = 0.2;

pub struct Smoothness {
    pub metrics: HashMap<MetricID, f32>,
    pub timestamp: u64,
    history: CircularBuffer<ProcessedSmoothness>,
    new_messages_since_last_concatenation: u16
}

#[derive(Clone)]
pub struct ProcessedSmoothness {
    pub smoothness_index: f32,
    pub timestamp: u64,
    smoothed_g_long: f32,
    smoothed_g_lat: f32,
    dv_dt_g_long: f32,
    dv_dt_g_lat: f32,
}

impl Smoothness {
    pub fn new(history_size: usize) -> Self {
        let mut metrics = HashMap::new();
        metrics.insert(G_LONG, 0.0);
        metrics.insert(G_LAT, 0.0);
        Self {
            metrics,
            timestamp: 0,
            history: CircularBuffer::new(history_size),
            new_messages_since_last_concatenation: 0
        }
    }}

impl Telemetry for Smoothness {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) {
        crate::update_telemetry!(self, telemetry_value);

        let mut current_smoothed_g_long = self.metrics.get(&G_LONG).unwrap_or(&0.0).clone();
        let mut current_smoothed_g_lat = self.metrics.get(&G_LAT).unwrap_or(&0.0).clone();
        let mut dv_dt_g_long = 0.0;
        let mut dv_dt_g_lat = 0.0;
        let mut current_smoothness_index= 0.0;

        if self.history.len() > 0 {
            current_smoothed_g_long = SMOOTHNESS_ALPHA * current_smoothed_g_long +
                (1.0 - SMOOTHNESS_ALPHA) * self.history.peek_newest().unwrap().smoothed_g_long;
            current_smoothed_g_lat = SMOOTHNESS_ALPHA * current_smoothed_g_lat +
                (1.0 - SMOOTHNESS_ALPHA) * self.history.peek_newest().unwrap().smoothed_g_lat;
            let delta = (telemetry_value.timestamp - self.history.peek_newest().unwrap().timestamp) as f32;
            dv_dt_g_long = (current_smoothed_g_long - self.history.peek_newest().unwrap().dv_dt_g_long) / delta;
            dv_dt_g_lat = (current_smoothed_g_lat - self.history.peek_newest().unwrap().dv_dt_g_lat) / delta;
            current_smoothness_index = (dv_dt_g_long * dv_dt_g_long + dv_dt_g_lat * dv_dt_g_lat).sqrt();
        }

        let p_s = ProcessedSmoothness {
            smoothness_index: current_smoothness_index.clone(),
            timestamp: telemetry_value.timestamp.clone(),
            smoothed_g_long: current_smoothed_g_long.clone(),
            smoothed_g_lat: current_smoothed_g_lat.clone(),
            dv_dt_g_long: dv_dt_g_long.clone(),
            dv_dt_g_lat: dv_dt_g_lat.clone()
        };

        self.history.push(p_s);
        self.new_messages_since_last_concatenation += 1;
    }

    fn produce_concatenated_message(&mut self) -> (String, serde_json::Value) {
        todo!()
    }

    fn get_type(&self) -> String {
        String::from("smoothness")
    }
}