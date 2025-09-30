use std::collections::HashMap;
use std::sync::Arc;
use socketcan::{CanFrame, Frame};
use tokio::sync::{mpsc, Mutex};
use crate::processor::metric_manager::MetricManager;
use crate::processor::types::{TelemetryDecoder};
use crate::can_rules::can_message_ids::{MessageID};

pub struct TelemetryProcessor {
    can_receiver: mpsc::Receiver<(CanFrame, u64)>,
    message_decoders: HashMap<u32, Box<dyn TelemetryDecoder>>,
    metric_manager: Arc<Mutex<MetricManager>>
}

impl TelemetryProcessor {
    pub fn new(can_receiver: mpsc::Receiver<(CanFrame, u64)>,
               message_decoders: HashMap<MessageID, Box<dyn TelemetryDecoder>>,
               metric_manager: Arc<Mutex<MetricManager>>) -> Self {
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
            {
                let mut manager = self.metric_manager.lock().await;
                manager.notify_subscribers(decoded_message);
            }
        }
    }
}