

pub enum Metrics {
    Speed,
    GForceLong,
    GForceLat,
    YawRate,
    SteeringAngle
}

pub struct TelemetryValue {
    metric: Metrics,
    value: f32
}