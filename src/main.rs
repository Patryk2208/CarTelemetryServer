extern crate core;

use std::sync::Arc;
use std::time::Duration;
use socketcan::CanFrame;
use tokio::{join, runtime };
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::can_reader::factory::create_can_reader;
use crate::processor::factory::create_metric_manager;
use crate::processor::factory::create_telemetry_processor;
use crate::run_configuration::RunConfiguration;
use crate::server::factory::{create_network_manager, create_server};
use crate::shutdown::ShutdownManager;

mod can_reader;
mod processor;
mod can_rules;
mod server;
mod common;
mod shutdown;
mod run_configuration;

#[tokio::main]
async fn main() {
    let config = RunConfiguration::new();

    let metric_manager = create_metric_manager();
    let buffer_size = 10;
    let (telemetry_sender, telemetry_receiver): (Sender<(CanFrame, u64)>, Receiver<(CanFrame, u64)>) =
        tokio::sync::mpsc::channel(buffer_size);

    let (shutdown,
        server_shutdown,
        processor_shutdown,
        reader_shutdown) = ShutdownManager::new();
    let reader = create_can_reader(
        config.interface.as_str(),
        telemetry_sender,
        reader_shutdown
    );
    let processor = create_telemetry_processor(
        telemetry_receiver,
        Arc::clone(&metric_manager),
        processor_shutdown
    );
    let server = create_server(
        Arc::clone(&metric_manager),
        server_shutdown
    );
    let network_manager = create_network_manager(server, config.target_ssid.as_str());

    let async_runtime:Runtime;
    match runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .max_blocking_threads(2)
        .enable_all()
        .build() {
        Ok(rt) => async_runtime = rt,
        Err(e) => panic!("Failed to create async runtime: {}", e)
    }
    let shutdown_handle = async_runtime.spawn(async move {
        shutdown.run().await;
    });
    let processor_handle = async_runtime.spawn(async move {
        processor.run().await;
    });
    let server_handle = async_runtime.spawn(async move {
        network_manager.run().await;
    });

    let reader_handle = reader.start();
    match reader_handle.join() {
        Ok(_) => {
            println!("[Reader] shut down correctly");
        }
        Err(_) => {
            println!("[Reader] shut down incorrectly");
        }
    }
    let res = join!(processor_handle, server_handle, shutdown_handle);
    match res.0 { 
        Ok(_) => {
            println!("[Processor] shut down correctly");
        }
        Err(_) => {
            println!("[Processor] shut down incorrectly");
        }
    }
    match res.1 { 
        Ok(_) => {
            println!("[Server] shut down correctly");
        }
        Err(_) => {
            println!("[Server] shut down incorrectly");
        }
    }
    match res.2 { 
        Ok(_) => {
            println!("Shutdown almost complete");
        }
        Err(_) => {
            println!("Shutdown almost complete with errors");
        }
    }
    async_runtime.shutdown_timeout(Duration::from_secs(10));
    println!("[AsyncRuntime] shut down correctly");
    println!("Shutdown complete");
}
