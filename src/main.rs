use std::sync::Arc;
use socketcan::CanFrame;
use tokio::runtime;
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::can_reader::factory::create_can_reader;
use crate::processor::factory::create_metric_manager;
use crate::processor::factory::create_telemetry_processor;
use crate::server::factory::create_server;

mod can_reader;
mod processor;
mod can_rules;
mod server;
mod common;

fn main() {
    let metric_manager = create_metric_manager();
    let buffer_size = 100;
    let (telemetry_sender, telemetry_receiver): (Sender<(CanFrame, u64)>, Receiver<(CanFrame, u64)>) =
        tokio::sync::mpsc::channel(buffer_size);
    let interface = "vcan0"; //for testing
    let reader = create_can_reader(interface, telemetry_sender);
    let processor = create_telemetry_processor(telemetry_receiver, Arc::clone(&metric_manager));
    let server = create_server("127.0.0.1:8080", Arc::clone(&metric_manager));

    let async_runtime:Runtime;
    match runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .max_blocking_threads(2)
        .enable_all()
        .build() {
        Ok(rt) => async_runtime = rt,
        Err(e) => panic!("Failed to create async runtime: {}", e)
    }
    let processor_handle = async_runtime.spawn(async move {
        processor.run().await;
    });
    let server_handle = async_runtime.spawn(async move {
        match server.await {
            Ok(mut server) => {
                match server.run().await {
                    Ok(_) => {},
                    Err(_) => {}
                }
            },
            Err(_) => {}
        }
    });
    let reader_handle = reader.start();
    match reader_handle.join() {
        Ok(_) => {}
        Err(_) => {}
    }
    async_runtime.block_on(async move {
        match processor_handle.await {
            Ok(_) => {}
            Err(_) => {}
        }
        match server_handle.await {
            Ok(_) => {},
            Err(_) => {}
        }
    });
}
