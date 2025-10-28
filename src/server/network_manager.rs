use crate::server::server::Server;
use std::process::{Command};
use std::time::Duration;
use tokio::time::sleep;

pub struct NetworkManager {
    server: Server,
    address: String,
    port: u16,
    target_ssid: String,
    recheck_interval: u64,
}

impl NetworkManager {
    pub fn new(server: Server, target_ssid: &str, port: u16, interval: u64) -> Self {
        Self {
            server,
            address: String::new(),
            port,
            target_ssid: target_ssid.to_string(),
            recheck_interval: interval,
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
                    sleep(Duration::from_secs(self.recheck_interval)).await;
                    continue;
                }
            }
            res = self.server.assign_address(format!("{}:{}", self.address, self.port).as_str()).await;
            match res {
                Ok(_) => (),
                Err(e) => {
                    println!("[NetworkManager] Error: {}", e);
                    sleep(Duration::from_secs(self.recheck_interval)).await;
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
                    sleep(Duration::from_secs(self.recheck_interval)).await;
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
            println!("[Network Manager] Checking for hotspot");
            match self.is_target_ssid_available() {
                Ok(_) => {
                    println!("[Network Manager] Hotspot found! Connecting...");

                    match self.connect_to_hotspot() {
                        Ok(_) => {
                            println!("[Network Manager] Connected! Waiting for IP assignment...");

                            if let Some(ip) = self.get_wlan_ip() {
                                println!("Success! Pi IP: {}", ip);
                                return Ok(ip);
                            } else {
                                println!("Could not connect to hotspot");
                            }
                        },
                        Err(e) => {
                            return Err(format!("Error connecting to hotspot: {}", e));
                        }
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            }
            println!("[Network Manager] Hotspot not available, retrying in {} seconds...", self.recheck_interval);
            sleep(Duration::from_secs(self.recheck_interval)).await;
        }
    }

    fn is_target_ssid_available(&self) -> Result<(), String> {
        let output = Command::new("nmcli")
            .args(&["-t", "-f", "SSID", "dev", "wifi"])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let ssids = String::from_utf8_lossy(&output.stdout);
                if ssids.lines().any(|line| line.trim() == self.target_ssid) {
                    Ok(())
                } else {
                    Err("[Network Manager] Error: SSID not found".to_string())
                }
            }
            _ => {
                Err("[Network Manager] Error: nmcli error.".to_string())
            }
        }
    }

    fn connect_to_hotspot(&self) -> Result<(), String> {
        let status = Command::new("nmcli")
            .args(&["dev", "wifi", "connect", &self.target_ssid])
            .status();

        match status {
            Ok(status) if status.success() => Ok(()),
            _ => Err("[Network Manager] Error: nmcli connect error.".to_string()),
        }
    }

    fn get_wlan_ip(&self) -> Option<String> {
        let ip_output = Command::new("ip")
            .args(&["-4", "addr", "show", "wlp59s0"]) //todo only for testing
            .output()
            .ok()?;

        if ip_output.status.success() {
            let output_str = String::from_utf8_lossy(&ip_output.stdout);
            for line in output_str.lines() {
                if line.trim().starts_with("inet ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let ip_with_mask = parts[1];
                        if let Some(ip) = ip_with_mask.split('/').next() {
                            return Some(ip.to_string());
                        }
                    }
                }
            }
        }
        None
    }
}