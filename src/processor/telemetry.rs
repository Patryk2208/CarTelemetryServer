use crate::processor::types::{TelemetryValue};

#[macro_export]
macro_rules! update_telemetry {
    ($self:ident, $telemetry_value:ident) => {
        $self.metrics
            .entry($telemetry_value.metric.clone())
            .and_modify(|v| *v = $telemetry_value.value)
            .or_insert($telemetry_value.value);
        let delta = ($telemetry_value.timestamp - $self.timestamp) / 2;
        $self.timestamp += delta;
    };
}

pub trait Telemetry: Send{
    fn update_metric(&mut self, telemetry_value: &TelemetryValue);
    fn produce_concatenated_message(&mut self) -> (String, serde_json::Value);
    fn get_type(&self) -> String;
}