use std::time::Instant;
use socketcan::CanFrame;
use crate::processor::metric_observer::MetricObserver;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum Metrics {
    Speed, //km/h
    GForceLong, //G, so 9.81 m/s^2
    GForceLat, //G, so 9.81 m/s^2
    YawRate, //deg/s
    SteeringAngle //deg/s
}

pub struct TelemetryValue {
    pub metric: Metrics,
    pub value: f32,
    pub timestamp: Instant
}

pub(crate) trait TelemetryDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: Instant) -> TelemetryValue;
}