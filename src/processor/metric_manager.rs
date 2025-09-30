use std::collections::HashMap;
use crate::processor::telemetry::{Telemetry};
use crate::processor::types::{MetricID, TelemetryValue};

pub struct MetricManager {
    pub subscribers: Vec<Box<dyn Telemetry>>,
    pub subscriptions: HashMap<MetricID, Vec<usize>>
}

impl MetricManager {
    pub fn add_subscriber(&mut self, metric: MetricID, subscriber: Box<dyn Telemetry>) -> bool {
        let subscriber_id = subscriber.get_type();

        if let Some(existing_index) = self.subscribers.iter().position(|sub| sub.get_type() == subscriber_id) {
            self.subscriptions
                .entry(metric)
                .or_insert_with(Vec::new)
                .push(existing_index);
            false
        } else {
            let subscriber_index = self.subscribers.len();
            self.subscribers.push(subscriber);

            self.subscriptions
                .entry(metric)
                .or_insert_with(Vec::new)
                .push(subscriber_index);
            true
        }
    }

    //called by telemetry processor
    pub fn notify_subscribers(&mut self, metric_value: TelemetryValue) {
        if let Some(subscribers) = self.subscriptions.get_mut(&metric_value.metric) {
            for sub_index in subscribers.iter() {
                self.subscribers[sub_index.clone()].update_metric(&metric_value);
            }
        }
    }

    //called by the server
    pub fn get_message(&mut self) -> serde_json::Value {
        let mut message = serde_json::json!({});
        if let Some(map) = message.as_object_mut() {
            for sub in self.subscribers.iter_mut() {
                let (key, value) = sub.produce_concatenated_message();
                map.insert(key, value);
            }
        }
        message
    }
}