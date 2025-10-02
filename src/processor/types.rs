
pub type MetricID = u8;
pub const SPEED: MetricID = 0;
pub const G_LAT: MetricID = 1;
pub const G_LONG: MetricID = 2;
pub const YAW: MetricID = 3;
pub const STEERING: MetricID = 4;
pub const BRAKE_ON_OFF: MetricID = 5;

pub struct TelemetryValue {
    pub metric: MetricID,
    pub value: f32,
    pub timestamp: u64
}