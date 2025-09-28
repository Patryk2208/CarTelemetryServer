use std::collections::HashMap;
use std::time::Instant;
use socketcan::{CanFrame, Frame};
use tokio::sync::{broadcast, mpsc};
use crate::processor::metric_manager::MetricManager;
use crate::processor::types::{TelemetryDecoder, TelemetryValue};
use crate::can_rules::can_message_ids::{MessageID, G_LAT_ID, G_LONG_ID, SPEED_ID, STEERING_ID, YAW_ID};
use crate::processor::decoders::{GForceLatDecoder, GForceLongDecoder, SpeedDecoder, SteeringAngleDecoder, YawRateDecoder};
use crate::processor::telemetry::{ProcessedTelemetry, Telemetry};

pub struct TelemetryProcessor {
    can_receiver: mpsc::Receiver<(CanFrame, Instant)>,
    broadcaster: broadcast::Sender<ProcessedTelemetry>,
    message_decoders: HashMap<u32, Box<dyn TelemetryDecoder>>,
    metric_observer: MetricManager
}

impl TelemetryProcessor {
    pub fn new(can_receiver: mpsc::Receiver<(CanFrame, Instant)>,
               broadcaster: broadcast::Sender<ProcessedTelemetry>,
               message_decoders: HashMap<MessageID, Box<dyn TelemetryDecoder>>,
               metric_observer: MetricManager) -> Self {
        Self {
            can_receiver,
            broadcaster,
            message_decoders,
            metric_observer
        }
    }

    pub async fn run(mut self) {
        while let Some((frame, timestamp)) = self.can_receiver.recv().await {
            if !self.message_decoders.contains_key(&frame.can_id().as_raw()) {
                continue;
            }
            let decoded_message = self.message_decoders[&frame.can_id().as_raw()]
                .decode_frame(frame, timestamp);
            for updated_telemetry in self.metric_observer.notify_subscribers(decoded_message) {
                match self.broadcaster.send(updated_telemetry) {
                    Ok(_) => (),
                    Err(_) => eprintln!("update could not be sent, continuing")
                }
            }
        }
    }
}

/*
let mut decoders: HashMap<MessageID, Box<dyn TelemetryDecoder>> = HashMap::new();
decoders.insert(SPEED_ID, Box::new(SpeedDecoder));
decoders.insert(G_LONG_ID, Box::new(GForceLongDecoder));
decoders.insert(G_LAT_ID, Box::new(GForceLatDecoder));
decoders.insert(YAW_ID, Box::new(YawRateDecoder));
decoders.insert(STEERING_ID, Box::new(SteeringAngleDecoder));*/