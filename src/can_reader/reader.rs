use socketcan;
use tokio::sync::{mpsc};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use anyhow::{Result, Context};
use socketcan::{CanFilter, CanFrame, Socket, SocketOptions};

pub struct CanReader {
    pub interface: String,
    pub message_filter: Vec<CanFilter>,
    pub read_timeout: Duration,
    pub frame_sender: mpsc::Sender<(CanFrame, u64)>
}
impl CanReader {

    pub fn start(self) -> JoinHandle<()> {
        thread::spawn(move || {
            if let Err(e) = self.run_can_reader() {
                eprintln!("CAN reader thread error: {}", e);
            }
        })
    }

    fn run_can_reader(self) -> Result<()>{
        let socket = socketcan::CanSocket::open(&self.interface)
            .with_context(|| format!("Failed to open CAN interface {}", self.interface))?;
        socket.set_read_timeout(self.read_timeout)?;
        socket.set_filters(self.message_filter.as_slice())?;
        println!("[CAN Reader] Starting CAN Reader on interface {} with filters: {:?}",
                 self.interface, self.message_filter);

        loop {
            match socket.read_frame() {
                Ok(frame) => {
                    let timestamp: u64;
                    match SystemTime::now().duration_since(UNIX_EPOCH) {
                        Ok(t) => timestamp = t.as_millis() as u64,
                        Err(_) => continue
                    }

                    //println!("[CAN Reader] Received frame: {:?}", frame);

                    match self.frame_sender.try_send((frame, timestamp)) {
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
                    thread::sleep(Duration::from_millis(500));
                }
            }
        }

        Ok(())
    }
}