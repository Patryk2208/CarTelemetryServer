use std::time::Instant;
use crate::processor::adv_metric_balance::ProcessedBalance;
use crate::processor::adv_metric_braking_signal::ProcessedBrakingSignal;
use crate::processor::adv_metric_gg::ProcessedGG;
use crate::processor::adv_metric_smoothness::ProcessedSmoothness;
use crate::processor::adv_metric_grip::ProcessedGrip;
use crate::common::circular_buffer::CircularBuffer;
use crate::server::websocket_interface::WebSocketMessage;

pub struct MetricConcat {
    pub gg_concat: GGConcat,
    pub balance_concat: BalanceConcat,
    pub braking_signal_concat: BrakingSignalConcat,
    pub smoothness_concat: SmoothnessConcat,
    pub grip_concat: GripConcat
}
impl MetricConcat {
    pub fn prepare_message(&self) -> WebSocketMessage {
        //todo
        WebSocketMessage{
            message_type: String::new(),
            data: serde_json::Value::Null,
            timestamp: 0
        }
    }
}

pub struct GGConcat {
    messages: CircularBuffer<ProcessedGG>
}
impl GGConcat {
    pub fn append_telemetry(&mut self, telemetry: ProcessedGG) {
        self.messages.push(telemetry);
    }
}

pub struct BalanceConcat {
    messages: CircularBuffer<ProcessedBalance>
}
impl BalanceConcat {
    pub fn append_telemetry(&mut self, telemetry: ProcessedBalance) {
        self.messages.push(telemetry);
    }
}

pub struct GripConcat {
    messages: CircularBuffer<ProcessedGrip>
}
impl GripConcat {
    pub fn append_telemetry(&mut self, telemetry: ProcessedGrip) {
        self.messages.push(telemetry);
    }
}

pub struct BrakingSignalConcat {
    messages: CircularBuffer<ProcessedBrakingSignal>
}
impl BrakingSignalConcat {
    pub fn append_telemetry(&mut self, telemetry: ProcessedBrakingSignal) {
        self.messages.push(telemetry);
    }
}

pub struct SmoothnessConcat {
    messages: CircularBuffer<ProcessedSmoothness>
}
impl SmoothnessConcat {
    pub fn append_telemetry(&mut self, telemetry: ProcessedSmoothness) {
        self.messages.push(telemetry);
    }
}