use crate::can_reader::reader::{CanReader, CanReaderConfig};
use crate::processor::processor::TelemetryProcessor;

mod can_reader;
mod processor;
mod can_rules;
mod server;
mod common;

fn main() {
    //reader init
    let interface = "vcan0"; //for testing
    let can_config = CanReaderConfig {
        interface: String::from(interface),
        message_filter: vec![],
        buffer_size: 100,
        read_timeout: std::time::Duration::from_millis(100)
    };
    let can_reader = CanReader::new(can_config);
    
    
    //processor init
    let 
    let processor = TelemetryProcessor::new()
}
