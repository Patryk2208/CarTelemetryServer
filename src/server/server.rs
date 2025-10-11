use std::sync::Arc;
use std::time::SystemTime;
use futures_util::{stream, SinkExt, StreamExt};
use serde_json::json;
use tokio::{net, select};
use tokio::net::TcpListener;
use tokio::sync::broadcast;
use tokio::sync::Mutex;
use tokio_tungstenite::WebSocketStream;
use tungstenite::{Message, Utf8Bytes};
use crate::processor::metric_manager::MetricManager;
use crate::server::flow_control::FlowControl;

struct WebSocketConnection {
    sender: stream::SplitSink<
        WebSocketStream<net::TcpStream>,
        Message
    >
}

pub struct MetricSender {
    pub metric_manager: Arc<Mutex<MetricManager>>,
    pub flow_control: FlowControl
}

pub struct Server {
    listener: TcpListener,
    current_connection: Arc<Mutex<Option<WebSocketConnection>>>,
    metric_sender: MetricSender,
    shutdown_channel: broadcast::Receiver<()>
}

impl Server {
    pub async fn new(addr: &str, metric_sender: MetricSender, shutdown: broadcast::Receiver<()>) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("WebSocket server listening on {}", addr);

        Ok(Self {
            listener,
            current_connection: Arc::new(Mutex::new(None)),
            metric_sender,
            shutdown_channel: shutdown
        })
    }
    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            select! {
                biased;
                _ = self.shutdown_channel.recv() => {
                    println!("[Server] Shutting down");
                    return Ok(());
                }
                listener_result = self.listener.accept() => {
                    match listener_result {
                        Ok((stream, addr)) => {
                            println!("Client connected from {}", addr);

                            let ws_stream = tokio_tungstenite::accept_async(stream).await?;
                            let (sender, /*receiver*/_) = ws_stream.split();

                            let connection = WebSocketConnection { sender };

                            {
                                let mut current = self.current_connection.lock().await;
                                *current = Some(connection);
                            }

                            let was_cancelled = self.transfer_metrics().await;
                            if was_cancelled {
                                println!("[Server] Shutting down");
                                return Ok(());
                            }

                            println!("Client disconnected, waiting for new connection...");
                        }
                        Err(e) => {
                            eprintln!("Accept error: {}", e);
                            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                        }
                    }
                }
            }
        }
    }
    pub async fn transfer_metrics(&mut self) -> bool {
        loop {
            self.metric_sender.flow_control.start_iteration();
            let message: serde_json::Value;
            {
                select! {
                    biased;
                    _ = self.shutdown_channel.recv() => {
                        println!("[Server] Metric manager lock interrupted");
                        return true;
                    }
                    mut manager = self.metric_sender.metric_manager.lock() => {
                        message = manager.get_message();
                    }
                }
            }

            {
                select! {
                    biased;
                    _ = self.shutdown_channel.recv() => {
                        println!("[Server] Websocket connection lock interrupted");
                        return true;
                    }
                    mut current = self.current_connection.lock() => {
                        if let Some(connection) = current.as_mut() {
                            select! {
                                biased;
                                _ = self.shutdown_channel.recv() => {
                                    println!("[Server] Websocket send interrupted");
                                    return true;
                                }
                                send_result = connection.sender.send(prepare_telemetry_message(message)) => {
                                    match send_result {
                                        Ok(_) => {},
                                        Err(e) => {
                                            eprintln!("[Server] Send failed: {}, closing connection", e);
                                            *current = None;
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            select! {
                _ = self.shutdown_channel.recv() => {
                    println!("[Server] Sleep interrupted");
                    return true;
                }
                _ = self.metric_sender.flow_control.complete_iteration() => {}
            }
        }
    }

    /*async fn handle_connection(&self, receiver:  stream::SplitStream<WebSocketStream<net::TcpStream>>) {
        Self::handle_receiver(receiver).await;

        {
            let mut current = self.current_connection.lock().await;
            *current = None;
        }
    }

    async fn handle_receiver(mut receiver: stream::SplitStream<WebSocketStream<net::TcpStream>>) {
        while let Some(message) = receiver.next().await {
            match message {
                Ok(Message::Close(_)) => {
                    println!("Client closed connection");
                    break;
                }
                Ok(Message::Text(text)) => {
                    println!("Client message: {}", text);
                    // todo handle command
                }
                Ok(Message::Ping(data)) => {
                    println!("Client ping ({} bytes)", data.len());
                }
                Err(e) => {
                    println!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }*/
}

fn prepare_telemetry_message(telemetry: serde_json::Value) -> Message {
    let time;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => time = n.as_millis() as u64,
        Err(_) => time = 0
    }
    let message = json!({
            "type": "Telemetry",
            "data": telemetry,
            "timestamp": time
        });
    Message::Text(Utf8Bytes::from(message.to_string()))
}