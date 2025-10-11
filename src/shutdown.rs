use tokio::sync::{broadcast, mpsc};


pub struct ShutdownManager {
    async_shutdown: broadcast::Sender<()>,
    sync_shutdown: mpsc::Sender<()>
}

impl ShutdownManager {
    pub fn new() -> (Self, broadcast::Receiver<()>, broadcast::Receiver<()>, mpsc::Receiver<()>) {
        let async_shutdown = broadcast::channel(1);
        let sync_shutdown = mpsc::channel(1);
        let async_1 = async_shutdown.0.subscribe();
        let async_2 = async_shutdown.0.subscribe();
        (Self {
            async_shutdown: async_shutdown.0,
            sync_shutdown: sync_shutdown.0
        },
         async_1,
         async_2,
         sync_shutdown.1
        )
    }

    pub async fn run(&self) {
        match tokio::signal::ctrl_c().await {
            Ok(_) => {
                self.sync_shutdown.send(()).await.unwrap();
                match self.async_shutdown.send(()) {
                    Ok(_) => {},
                    Err(_) => {}
                }
            },
            Err(_) => {
                self.sync_shutdown.send(()).await.unwrap();
                match self.async_shutdown.send(()) {
                    Ok(_) => {},
                    Err(_) => {}
                }
            }
        }

    }
}