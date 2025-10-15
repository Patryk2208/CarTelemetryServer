use tokio::select;
use crate::server::server::Server;

pub struct NetworkManager {
    pub server: Server,
    pub address: String,
}

impl NetworkManager {
    pub fn new(server: Server) -> Self {
        Self {
            server,
            address: String::new(),
        }
    }

    pub async fn run(mut self) {
        let mut res;
        loop {
            res = self.wait_for_network().await;
            match res {
                Ok(_) => (),
                Err(e) => {
                    println!("[NetworkManager] Error: {}", e);
                    continue;
                }
            }
            res = self.server.run().await;
            match res {
                Ok(_) => {
                    break;
                },
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            }
        }
    }

    async fn wait_for_network(&mut self) -> Result<(), String> {
        let ip = self.get_ip().await;
        match ip {
            Ok(ip) => {
                self.address = ip;
                Ok(())
            }
            Err(e) => {
                Err(e)
            }
        }
    }

    async fn get_ip(&self) -> Result<String, String> {
        loop {
            //todo get ip link show info and parse the address
            //todo verify address is from correct source
            //todo if not sleep a second
        }
    }
}