use crate::can_reader::factory::create_can_reader;
use crate::processor::factory::create_metric_manager;
use crate::processor::factory::create_telemetry_processor;
use crate::server::factory::create_server;

mod can_reader;
mod processor;
mod can_rules;
mod server;
mod common;

fn main() {
    todo!()
}
