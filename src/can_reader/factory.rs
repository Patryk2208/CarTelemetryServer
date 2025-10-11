use socketcan::{CanFilter, CanFrame};
use socketcan::frame::CAN_SFF_MASK;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use crate::can_reader::reader::CanReader;
use crate::can_rules::can_message_ids::{BRAKE_ID, FORCES_ID, SPEED_ID, STEERING_ID};

pub fn create_can_reader(
    interface: &str, 
    tx: Sender<(CanFrame, u64)>, 
    shutdown: mpsc::Receiver<()>
) -> CanReader {
    let filters = vec![
        CanFilter::new(SPEED_ID, CAN_SFF_MASK),
        CanFilter::new(FORCES_ID, CAN_SFF_MASK),
        CanFilter::new(STEERING_ID, CAN_SFF_MASK),
        CanFilter::new(BRAKE_ID, CAN_SFF_MASK)
    ];
    CanReader {
        interface: String::from(interface),
        message_filter: filters,
        read_timeout: std::time::Duration::from_millis(100),
        frame_sender: tx,
        shutdown_channel: shutdown
    }
}