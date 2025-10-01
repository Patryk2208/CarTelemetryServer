use std::collections::HashMap;
use serde::__private226::de::IdentifierDeserializer;
use crate::common::circular_buffer::CircularBuffer;
use crate::processor::telemetry::Telemetry;
use crate::processor::types::{MetricID, TelemetryValue, G_LAT, SPEED, YAW};

pub struct Grip {
    pub metrics: HashMap<MetricID, f32>,
    pub timestamp: u64,
    history: CircularBuffer<ProcessedGrip>,
    new_messages_since_last_concatenation: u16
}

const GRIP_CORNERING_THRESHOLD: f32 = 0.1;

#[derive(Clone)]
pub struct ProcessedGrip {
    pub grip_force: f32,
    pub max_grip_per_corner: Option<f32>,
    pub max_grip_per_ride: f32,
    pub timestamp: u64
}

impl Grip {
    pub fn new(history_size: usize) -> Self {
        let mut metrics = HashMap::new();
        metrics.insert(G_LAT, 0.0);
        Self {
            metrics,
            timestamp: 0,
            history: CircularBuffer::new(history_size),
            new_messages_since_last_concatenation: 0
        }
    }}

impl Telemetry for Grip {
    fn update_metric(&mut self, telemetry_value: &TelemetryValue) {
        crate::update_telemetry!(self, telemetry_value);

        let grip_force = self.metrics.get(&G_LAT).unwrap_or(&0.0);
        let mut max_grip_per_ride: &f32;
        let mut max_grip_per_corner: Option<f32>;
        let is_cornering = grip_force > &GRIP_CORNERING_THRESHOLD;
        max_grip_per_ride = grip_force;
        max_grip_per_corner = if is_cornering { Some(grip_force.clone()) } else { None };

        let last = self.history.peek_newest();
        if last.is_some() {
            if max_grip_per_ride < &last.unwrap().max_grip_per_ride {
                max_grip_per_ride = &last.unwrap().max_grip_per_ride;
            }
            if is_cornering {
                let last_corner_grip = &last.unwrap().max_grip_per_corner;
                if last_corner_grip.is_some() {
                    if grip_force < &last_corner_grip.unwrap() {
                        max_grip_per_corner = Some(last_corner_grip.unwrap().clone());
                    }
                }
            }
        }

        let p_s_r = ProcessedGrip {
            grip_force: grip_force.clone(),
            max_grip_per_corner,
            max_grip_per_ride: max_grip_per_ride.clone(),
            timestamp: telemetry_value.timestamp.clone()
        };
        
        self.history.push(p_s_r);
        self.new_messages_since_last_concatenation += 1;
    }

    fn produce_concatenated_message(&mut self) -> (String, serde_json::Value) {
        todo!()
    }

    fn get_type(&self) -> String {
        String::from("grip")
    }
}