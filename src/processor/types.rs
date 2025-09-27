use std::time::Instant;
use socketcan::CanFrame;
use crate::processor::metric_observer::MetricObserver;

pub type MetricID = u8;
pub const SPEED: MetricID = 0;
pub const G_LAT: MetricID = 1;
pub const G_LONG: MetricID = 2;
pub const YAW: MetricID = 3;
pub const STEERING: MetricID = 4;

pub struct TelemetryValue {
    pub metric: MetricID,
    pub value: f32,
    pub timestamp: Instant
}

pub(crate) trait TelemetryDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: Instant) -> TelemetryValue;
}