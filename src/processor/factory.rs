use std::collections::HashMap;
use std::sync::Arc;
use socketcan::CanFrame;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use crate::can_rules::can_message_ids::{MessageID, BRAKE_ID, FORCES_ID, SPEED_ID, STEERING_ID};
use crate::processor::adv_metric_balance::Balance;
use crate::processor::adv_metric_braking_signal::BrakingSignal;
use crate::processor::adv_metric_gg::GG;
use crate::processor::adv_metric_grip::Grip;
use crate::processor::adv_metric_smoothness::Smoothness;
use crate::processor::decoders::{TelemetryDecoder, SpeedDecoder, GForceLongDecoder, GForceLatDecoder, YawRateDecoder, SteeringAngleDecoder, BrakeOnOffDecoder};
use crate::processor::metric_manager::MetricManager;
use crate::processor::processor::TelemetryProcessor;
use crate::processor::telemetry::Telemetry;
use crate::processor::types::{BRAKE_ON_OFF, G_LAT, G_LONG, SPEED, STEERING, YAW};

pub fn create_metric_manager() -> Arc<Mutex<MetricManager>> {
    let history_size = 50;
    let subscribers:Vec<Box<dyn Telemetry>> = vec![
        Box::new(Balance::new(history_size)), //todo size
        Box::new(GG::new(history_size)), //todo size
        Box::new(Smoothness::new(history_size)), //todo size
        Box::new(Grip::new(history_size)), //todo size
        Box::new(BrakingSignal::new(history_size)) //todo size
    ];
    let mut subscriptions = HashMap::new();
    subscriptions.insert(SPEED, vec![0, 1]);
    subscriptions.insert(G_LAT, vec![0, 1, 2, 3]);
    subscriptions.insert(G_LONG, vec![1, 2, 4]);
    subscriptions.insert(YAW, vec![0]);
    subscriptions.insert(STEERING, vec![]);
    subscriptions.insert(BRAKE_ON_OFF, vec![4]);

    Arc::new(Mutex::new(MetricManager {
        subscribers,
        subscriptions
    }))
}

pub fn create_telemetry_processor(
    receiver: Receiver<(CanFrame, u64)>,
    metric_manager: Arc<Mutex<MetricManager>>
) -> TelemetryProcessor {
    let mut telemetry_decoder: HashMap<MessageID, Vec<Box<dyn TelemetryDecoder>>> = HashMap::new();
    telemetry_decoder.insert(SPEED_ID, vec![Box::new(SpeedDecoder {})]);
    telemetry_decoder.insert(FORCES_ID, vec![
        Box::new(GForceLongDecoder {}),
        Box::new(GForceLatDecoder {}),
        Box::new(YawRateDecoder {})
    ]);
    telemetry_decoder.insert(STEERING_ID, vec![Box::new(SteeringAngleDecoder {})]);
    telemetry_decoder.insert(BRAKE_ID, vec![Box::new(BrakeOnOffDecoder {})]);

    TelemetryProcessor::new(receiver, telemetry_decoder, metric_manager)
}