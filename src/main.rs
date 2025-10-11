extern crate core;

use std::sync::Arc;
use socketcan::CanFrame;
use tokio::{join, runtime };
use tokio::runtime::Runtime;
use tokio::sync::mpsc::{Receiver, Sender};
use crate::can_reader::factory::create_can_reader;
use crate::processor::factory::create_metric_manager;
use crate::processor::factory::create_telemetry_processor;
use crate::server::factory::create_server;
use crate::shutdown::ShutdownManager;

mod can_reader;
mod processor;
mod can_rules;
mod server;
mod common;
mod shutdown;

#[tokio::main]
async fn main() {
    let interface = "vcan0"; //for testing
    let address = "127.0.0.1:8080";

    let metric_manager = create_metric_manager();
    let buffer_size = 10;
    let (telemetry_sender, telemetry_receiver): (Sender<(CanFrame, u64)>, Receiver<(CanFrame, u64)>) =
        tokio::sync::mpsc::channel(buffer_size);

    let (shutdown,
        server_shutdown,
        processor_shutdown,
        reader_shutdown) = ShutdownManager::new();
    let reader = create_can_reader(
        interface,
        telemetry_sender,
        reader_shutdown
    );
    let processor = create_telemetry_processor(
        telemetry_receiver,
        Arc::clone(&metric_manager),
        processor_shutdown
    );
    let server = create_server(
        address,
        Arc::clone(&metric_manager),
        server_shutdown
    );

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
    let res = join!(shutdown_handle, processor_handle, server_handle);
    match res.0 {
        Ok(_) => {},
        Err(_) => {
            //todo cancel all
        }
    }
    match res.1 {
        Ok(_) => {},
        Err(_) => {
            //todo cancel all
        }
    }
    match res.2 {
        Ok(_) => {},
        Err(_) => {
            //todo cancel all
        }
    }
}
