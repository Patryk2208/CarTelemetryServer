use socketcan;
use tokio::sync::{mpsc};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use anyhow::{Result, Context};
use socketcan::{CanFrame, Socket, SocketOptions};
use crate::can_reader::types::{CanReaderConfig};

pub struct CanReader {
    pub config: CanReaderConfig,
}
impl CanReader {
    pub fn new(config: CanReaderConfig) -> Self {
        Self {
            config
        }
    }

    pub fn start(self) -> Result<mpsc::Receiver<(CanFrame, u64)>> {
        let (tx, rx) = mpsc::channel(self.config.buffer_size);

        let thread_builder = thread::Builder::new().name("can-reader".to_string());

        thread_builder.spawn(move || {
            if let Err(e) = self.run_can_reader(tx) {
                eprintln!("CAN reader thread error: {}", e);
            }
        })?;

        Ok(rx)
    }

    fn run_can_reader(self, tx: mpsc::Sender<(CanFrame, u64)>) -> Result<()>{
        let socket = socketcan::CanSocket::open(&self.config.interface)
            .with_context(|| format!("Failed to open CAN interface {}", self.config.interface))?;
        socket.set_read_timeout(self.config.read_timeout)?;
        socket.set_filters(self.config.message_filter.as_slice())?;
        println!("[CAN Reader] Starting CAN Reader on interface {} with filters: {:?}",
                 self.config.interface, self.config.message_filter);
        loop {
                match socket.read_frame() {
                    Ok(frame) => {
                        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;
                        println!("[CAN Reader] Received frame: {:?}", frame);
                        match tx.try_send((frame, timestamp)) {
                            Ok(_) => {}
                            Err(mpsc::error::TrySendError::Full(_)) => {
                                eprintln!("[CAN Reader] Can't send frame!!! Should not happen!");
                            }
                            Err(mpsc::error::TrySendError::Closed(_)) => {
                                eprintln!("[CAN Reader] Channel closed");
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("CAN reader error: {}", e);
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            //todo exit with select or simple
        }

        Ok(())
    }
}