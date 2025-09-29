use std::collections::HashMap;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{MetricID, TelemetryValue};

pub struct MetricManager {
    pub subscribers: Vec<Box<dyn Telemetry>>,
    pub subscriptions: HashMap<MetricID, Vec<Box<dyn Telemetry>>>
}

impl MetricManager {
    pub fn add_subscriber(&mut self, metric: MetricID, subscriber: &Box<dyn Telemetry>) {
        if !self.subscribers.contains(&subscriber) {
            self.subscribers.push(subscriber);
        }
        self.subscriptions
            .entry(metric)
            .or_insert_with(Vec::new)
            .push(subscriber);
    }

    //called by telemetry processor
    pub fn notify_subscribers(&mut self, metric_value: TelemetryValue) {
        if let Some(subscribers) = self.subscriptions.get_mut(&metric_value.metric) {
            for sub in subscribers.iter_mut() {
                sub.update_metric(&metric_value);
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
            //todo insert general message info
        }
        message
    }
}