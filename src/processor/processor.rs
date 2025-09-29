use std::collections::HashMap;
use std::time::Instant;
use socketcan::{CanFrame, Frame};
use tokio::sync::{mpsc};
use crate::processor::metric_manager::MetricManager;
use crate::processor::types::{TelemetryDecoder, TelemetryValue};
use crate::can_rules::can_message_ids::{MessageID};

pub struct TelemetryProcessor {
    can_receiver: mpsc::Receiver<(CanFrame, Instant)>,
    message_decoders: HashMap<u32, Box<dyn TelemetryDecoder>>,
    metric_manager: MetricManager
}

impl TelemetryProcessor {
    pub fn new(can_receiver: mpsc::Receiver<(CanFrame, Instant)>,
               message_decoders: HashMap<MessageID, Box<dyn TelemetryDecoder>>,
               metric_manager: MetricManager) -> Self {
        Self {
            can_receiver,
            message_decoders,
            metric_manager
        }
    }

    pub async fn run(mut self) {
        while let Some((frame, timestamp)) = self.can_receiver.recv().await {
            if !self.message_decoders.contains_key(&frame.can_id().as_raw()) {
                continue;
            }
            let decoded_message = self.message_decoders[&frame.can_id().as_raw()]
                .decode_frame(frame, timestamp);
            self.metric_manager.notify_subscribers(decoded_message);
        }
    }
}