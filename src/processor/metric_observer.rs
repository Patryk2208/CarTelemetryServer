use std::collections::HashMap;
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};
use crate::processor::types::{Metrics, TelemetryValue};

pub struct MetricObserver {
    pub subscribers: HashMap<Metrics, Vec<Box<dyn Telemetry>>>
}

impl MetricObserver {
    pub fn add_subscriber(&mut self, metric: Metrics, subscriber: Box<dyn Telemetry>) {
        self.subscribers
            .entry(metric)
            .or_insert_with(Vec::new)
            .push(subscriber);
    }

    pub fn notify_subscribers(&mut self, metric_value: TelemetryValue) -> impl Iterator<Item = ProcessedTelemetry> {
        let subscribers = self.subscribers
            .get_mut(&metric_value.metric)
            .into_iter()
            .flatten();

        subscribers.map(move |subscriber| {
            return subscriber.update_metric(&metric_value);
        })
    }
}