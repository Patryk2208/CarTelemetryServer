use std::collections::HashMap;
use std::time::Duration;
use socketcan::{CanFilter, CanFrame};

pub struct CanReaderConfig {
    pub interface: String,
    pub message_filter: Vec<CanFilter>,
    pub buffer_size: usize,
    pub read_timeout: Duration
}