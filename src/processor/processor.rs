use std::collections::HashMap;
use std::time::Instant;
use socketcan::{CanFrame, Frame};
use tokio::sync::{broadcast, mpsc};
use crate::processor::metric_observer::MetricObserver;
use crate::processor::types::{TelemetryDecoder, TelemetryValue};
use crate::can_rules::can_message_ids::{G_LAT_ID, G_LONG_ID, SPEED_ID, STEERING_ID, YAW_ID};
use crate::processor::decoders::{GForceLatDecoder, GForceLongDecoder, SpeedDecoder, SteeringAngleDecoder, YawRateDecoder};
use crate::processor::telemetry::ProcessedTelemetry;

pub struct TelemetryProcessor {
    can_receiver: mpsc::Receiver<(CanFrame, Instant)>,
    broadcaster: broadcast::Sender<Box<dyn ProcessedTelemetry>>,
    message_decoders: HashMap<u32, Box<dyn TelemetryDecoder>>,
    metric_processing: MetricObserver
}

impl TelemetryProcessor {
    pub fn new(can_receiver: mpsc::Receiver<(CanFrame, Instant)>,
               broadcaster: broadcast::Sender<Box<dyn ProcessedTelemetry>>,
               metric_observer: MetricObserver) -> Self {
        let mut decoders: HashMap<u32, Box<dyn TelemetryDecoder>> = HashMap::new();
        decoders.insert(SPEED_ID, Box::new(SpeedDecoder));
        decoders.insert(G_LONG_ID, Box::new(GForceLongDecoder));
        decoders.insert(G_LAT_ID, Box::new(GForceLatDecoder));
        decoders.insert(YAW_ID, Box::new(YawRateDecoder));
        decoders.insert(STEERING_ID, Box::new(SteeringAngleDecoder));
        Self {
            can_receiver,
            broadcaster,
            message_decoders: decoders,
            metric_processing: metric_observer
        }
    }
    pub async fn run(mut self) {
        while let Some((frame, timestamp)) = self.can_receiver.recv().await {
            if !self.message_decoders.contains_key(&frame.can_id().as_raw()) {
                continue;
            }
            let decoded_message = self.message_decoders[&frame.can_id().as_raw()]
                .decode_frame(frame, timestamp);
            self.metric_processing.notify_subscribers(decoded_message);
            //todo broadcast to websocket interface
        }
    }
}