use socketcan::{CanFrame};
use socketcan::frame::AsPtr;
use crate::processor::types::{TelemetryValue, BRAKE_ON_OFF, G_LAT, G_LONG, SPEED, STEERING, YAW};

pub trait TelemetryDecoder: Send {
    fn decode_frame(&self, frame: CanFrame, timestamp: u64) -> TelemetryValue;
}

pub struct SpeedDecoder;
impl TelemetryDecoder for SpeedDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: u64) -> TelemetryValue {
        let bytes = frame.as_bytes();
        let raw_value = u16::from_be_bytes([bytes[4], bytes[5]]);
        let mut value = raw_value as f32;
        value *= 0.01;
        TelemetryValue {
            metric: SPEED,
            value,
            timestamp
        }
    }
}

pub struct GForceLongDecoder;
impl TelemetryDecoder for GForceLongDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: u64) -> TelemetryValue {
        let bytes = frame.as_bytes();
        let raw_value = u16::from_be_bytes([bytes[0], bytes[1]]);
        let mut value = (raw_value >> 4) as f32;
        value *= 0.0009765625;
        value -= 2.0;
        TelemetryValue{
            metric: G_LONG,
            value,
            timestamp
        }
    }
}

pub struct GForceLatDecoder;
impl TelemetryDecoder for GForceLatDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: u64) -> TelemetryValue {
        let bytes = frame.as_bytes();
        let raw_value = u16::from_be_bytes([bytes[1], bytes[2]]);
        let mut value = (raw_value << 4) as f32;
        value *= 0.0009765625;
        value -= 2.0;
        TelemetryValue{
            metric: G_LAT,
            value,
            timestamp
        }
    }
}

pub struct YawRateDecoder;
impl TelemetryDecoder for YawRateDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: u64) -> TelemetryValue {
        let bytes = frame.as_bytes();
        let raw_value = u16::from_be_bytes([bytes[3], bytes[4]]);
        let mut value = (raw_value >> 4) as f32;
        value *= 0.1;
        value -= 204.7;
        TelemetryValue{
            metric: YAW,
            value,
            timestamp
        }
    }
}

pub struct SteeringAngleDecoder;
impl TelemetryDecoder for SteeringAngleDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: u64) -> TelemetryValue {
        let bytes = frame.as_bytes();
        let raw_value = i16::from_le_bytes([bytes[0], bytes[1]]);
        let mut value = raw_value as f32;
        value *= 0.1;
        TelemetryValue{
            metric: STEERING,
            value,
            timestamp
        }
    }
}

pub struct BrakeOnOffDecoder;
impl TelemetryDecoder for BrakeOnOffDecoder {
    fn decode_frame(&self, frame: CanFrame, timestamp: u64) -> TelemetryValue {
        let bytes = frame.as_bytes();
        let mut raw_value = i8::from_le_bytes([bytes[6]]);
        raw_value >>= 6;
        raw_value &= 1;
        let value = raw_value as f32;
        TelemetryValue {
            metric: BRAKE_ON_OFF,
            value,
            timestamp
        }
    }
}