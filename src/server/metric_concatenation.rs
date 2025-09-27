use std::time::Instant;
use crate::processor::adv_metric_balance::ProcessedBalance;
use crate::processor::adv_metric_braking_signal::ProcessedBrakingSignal;
use crate::processor::adv_metric_gg::ProcessedGG;
use crate::processor::adv_metric_smoothness::ProcessedSmoothness;
use crate::processor::adv_metric_steering_response::ProcessedSteeringResponse;
use crate::processor::telemetry::ProcessedTelemetry;
use crate::common::circular_buffer::CircularBuffer;

pub struct MetricConcat {
    pub gg_concat: GGConcat,
    pub balance_concat: BalanceConcat,
    pub braking_signal_concat: BrakingSignalConcat,
    pub smoothness_concat: SmoothnessConcat,
    pub steering_response_concat: SteeringResponseConcat
}
impl MetricConcat {
    pub fn prepare_message(&self) {

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

pub struct SteeringResponseConcat {
    messages: CircularBuffer<ProcessedSteeringResponse>
}
impl SteeringResponseConcat {
    pub fn append_telemetry(&mut self, telemetry: ProcessedSteeringResponse) {
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