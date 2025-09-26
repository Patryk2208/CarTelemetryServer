use std::collections::HashMap;
use std::time::Instant;
use socketcan::{CanFrame, Frame};
use tokio::sync::{broadcast, mpsc};
use crate::processor::types::{TelemetryDecoder, TelemetryValue};

pub struct TelemetryProcessor {
    can_receiver: mpsc::Receiver<(CanFrame, Instant)>,
    broadcaster: broadcast::Sender<TelemetryValue>,
    message_decoders: HashMap<u32, Box<dyn TelemetryDecoder>>,
    metric_processing: u8 //todo
}

impl TelemetryProcessor {
    pub async fn run(mut self) {
        while let Some((frame, timestamp)) = self.can_receiver.recv().await {
            if(!self.message_decoders.contains_key(&frame.can_id().as_raw())) {
                continue;
            }
            let decoded_message = self.message_decoders[&frame.can_id().as_raw()]
                .decode_frame(frame, timestamp);




        }
    }
}