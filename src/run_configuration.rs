use std::env;
use std::str::FromStr;

pub enum LogLevel {
    Info,
    Debug,
    Error,
}

impl FromStr for LogLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let case_insensitive = s.to_lowercase();
        if case_insensitive == "info" {
            Ok(LogLevel::Info)
        } else if case_insensitive == "debug" {
            Ok(LogLevel::Debug)
        } else if case_insensitive == "error" {
            Ok(LogLevel::Error)
        } else { 
            Err("Unknown log level".into())
        }
    }
}

pub struct RunConfiguration {
    pub interface : String,
    pub target_ssid : String,
    pub log_level : LogLevel,
}

impl RunConfiguration {
    pub fn new() -> Self {
        let mut log_level: LogLevel = LogLevel::Error;
        let ll_res = env::var("LOG_LEVEL");
        match ll_res {
            Ok(ll_str) => {
                let ll_parsed = ll_str.parse::<LogLevel>();
                match ll_parsed {
                    Ok(ll) => {
                        log_level = ll;
                    }
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
        Self {
            interface: env::var("INTERFACE").unwrap_or("can0".to_string()),
            target_ssid: env::var("TARGET_SSID").unwrap_or("LGV60".to_string()),
            log_level,
        }
    }
}