use std::collections::HashMap;
use crate::processor::telemetry::ProcessedTelemetry;
use crate::processor::types::{Metrics, TelemetryValue};

pub struct MetricObserver {
    pub subscribers: HashMap<Metrics, Vec<Box<dyn ProcessedTelemetry>>>
}

impl MetricObserver {
    pub fn add_subscriber(&mut self, metric: Metrics, subscriber: Box<dyn ProcessedTelemetry>) {
        self.subscribers
            .entry(metric)
            .or_insert_with(Vec::new)
            .push(subscriber);
    }

    pub fn notify_subscribers(&mut self, metric_value: TelemetryValue) {
        if let Some(subscribers) = self.subscribers.get_mut(&metric_value.metric) {
            for subscriber in subscribers {
                subscriber.update_metric(&metric_value);
            }
        }
    }
}