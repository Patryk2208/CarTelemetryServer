use std::collections::HashMap;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{MetricID, TelemetryValue};

pub struct MetricManager {
    pub subscribers: HashMap<MetricID, Vec<Box<dyn Telemetry>>>
}

impl MetricManager {
    pub fn add_subscriber(&mut self, metric: MetricID, subscriber: Box<dyn Telemetry>) {
        self.subscribers
            .entry(metric)
            .or_insert_with(Vec::new)
            .push(subscriber);
    }

    pub fn notify_subscribers(&mut self, metric_value: TelemetryValue) {
        if let Some(subscribers) = self.subscribers.get_mut(&metric_value.metric) {
            for sub in subscribers.iter_mut() {
                sub.update_metric(&metric_value);
            }
        }
    }

    pub fn get_message(&self) -> String {
        //todo generate message since last update
        return String::new();
    }
}