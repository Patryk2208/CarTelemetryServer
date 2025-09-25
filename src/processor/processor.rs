use std::collections::HashMap;
use std::time::Instant;
use socketcan::CanFrame;
use tokio::sync::{broadcast, mpsc};
use crate::processor::types::TelemetryValue;

pub struct TelemetryProcessor {
    can_receiver: mpsc::Receiver<(CanFrame, Instant)>,
    broadcaster: broadcast::Sender<()>, //todo
    message_decoders: HashMap<u32, TelemetryValue>
}

impl TelemetryProcessor {
    pub async fn run(mut self) {
        while let Some((frame, timestamp)) = self.can_receiver.recv().await {
            //todo decode message
        }
    }
}