use std::time::Instant;
use socketcan::{CanFrame};
use socketcan::frame::AsPtr;
use crate::processor::processor::TelemetryProcessor;
use crate::processor::types::{Metrics, TelemetryDecoder, TelemetryValue};

pub struct SpeedDecoder;
impl TelemetryDecoder for SpeedDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: Instant) -> TelemetryValue {
        let bytes = frame.as_bytes();
        let raw_value = u16::from_be_bytes([bytes[4], bytes[5]]);
        let mut value = raw_value as f32;
        value *= 0.01;
        TelemetryValue {
            metric: Metrics::Speed,
            value,
            timestamp
        }
    }
}

pub struct GForceLongDecoder;
impl TelemetryDecoder for GForceLongDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: Instant) -> TelemetryValue {
        let bytes = frame.as_bytes();
        let raw_value = u16::from_be_bytes([bytes[0], bytes[1]]);
        let mut value = (raw_value >> 4) as f32;
        value *= 0.0009765625;
        value -= 2.0;
        TelemetryValue{
            metric: Metrics::GForceLong,
            value,
            timestamp
        }
    }
}

pub struct GForceLatDecoder;
impl TelemetryDecoder for GForceLatDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: Instant) -> TelemetryValue {
        let bytes = frame.as_bytes();
        let raw_value = u16::from_be_bytes([bytes[1], bytes[2]]);
        let mut value = (raw_value << 4) as f32;
        value *= 0.0009765625;
        value -= 2.0;
        TelemetryValue{
            metric: Metrics::GForceLat,
            value,
            timestamp
        }
    }
}

pub struct YawRateDecoder;
impl TelemetryDecoder for YawRateDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: Instant) -> TelemetryValue {
        let bytes = frame.as_bytes();
        let raw_value = u16::from_be_bytes([bytes[3], bytes[4]]);
        let mut value = (raw_value >> 4) as f32;
        value *= 0.1;
        value -= 204.7;
        TelemetryValue{
            metric: Metrics::YawRate,
            value,
            timestamp
        }
    }
}

pub struct SteeringAngleDecoder;
impl TelemetryDecoder for SteeringAngleDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: Instant) -> TelemetryValue {
        let bytes = frame.as_bytes();
        let raw_value = i16::from_le_bytes([bytes[0], bytes[1]]);
        let mut value = raw_value as f32;
        value *= 0.1;
        TelemetryValue{
            metric: Metrics::SteeringAngle,
            value,
            timestamp
        }
    }
}