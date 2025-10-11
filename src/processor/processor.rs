use std::collections::HashMap;
use std::sync::Arc;
use socketcan::{CanFrame, Frame};
use tokio::select;
use tokio::sync::{broadcast, mpsc, Mutex};
use crate::processor::metric_manager::MetricManager;
use crate::processor::types::TelemetryValue;
use crate::can_rules::can_message_ids::{MessageID};
use crate::processor::decoders::TelemetryDecoder;

pub struct TelemetryProcessor {
    can_receiver: mpsc::Receiver<(CanFrame, u64)>,
    message_decoders: HashMap<MessageID, Vec<Box<dyn TelemetryDecoder>>>,
    metric_manager: Arc<Mutex<MetricManager>>,
    shutdown_channel: broadcast::Receiver<()>
}

impl TelemetryProcessor {
    pub fn new(can_receiver: mpsc::Receiver<(CanFrame, u64)>,
               message_decoders: HashMap<MessageID, Vec<Box<dyn TelemetryDecoder>>>,
               metric_manager: Arc<Mutex<MetricManager>>,
               shutdown_channel: broadcast::Receiver<()>
    ) -> Self {
        Self {
            can_receiver,
            message_decoders,
            metric_manager,
            shutdown_channel
        }
    }

    pub async fn run(mut self) {
        loop {
            select! {
                biased;
                _ = self.shutdown_channel.recv() => {
                    println!("[Processor] Shutting down");
                    break;
                }
                can_results = self.can_receiver.recv() => {
                    match can_results {
                         Some((frame, timestamp)) => {
                            if !self.message_decoders.contains_key(&frame.can_id().as_raw()) {
                                continue;
                            }
                            let decoded_messages = self.process_frame(frame, timestamp);
                            {
                                let mut manager = self.metric_manager.lock().await;
                                manager.notify_subscribers(decoded_messages);
                            }
                        },
                        None => {
                            println!("[Processor] Channel closed. Shutting down");
                            break;
                        }   
                    }
                }
            }
        }
    }

    pub fn process_frame(&self, frame: CanFrame, timestamp: u64) -> Vec<TelemetryValue> {
        let can_id = frame.can_id().as_raw();

        if let Some(decoders) = self.message_decoders.get(&can_id) {
            decoders.iter()
                .map(|decoder| decoder.decode_frame(frame.clone(), timestamp))
                .collect()
        } else {
            Vec::new()
        }
    }
}