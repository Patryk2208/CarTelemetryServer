use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;
use futures_util::{stream, SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use tokio::net;
use tokio_tungstenite::WebSocketStream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
}

pub struct WebSocketServer {
    listener: TcpListener,
    current_connection: Arc<Mutex<Option<WebSocketConnection>>>,
}

struct WebSocketConnection {
    sender: stream::SplitSink<
        WebSocketStream<net::TcpStream>,
        Message
    >
}

impl WebSocketServer {
    pub async fn new(addr: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("WebSocket server listening on {}", addr);

        Ok(Self {
            listener,
            current_connection: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            match self.listener.accept().await {
                Ok((stream, addr)) => {
                    println!("Client connected from {}", addr);

                    let ws_stream = tokio_tungstenite::accept_async(stream).await?;
                    let (sender, receiver) = ws_stream.split();

                    let connection = WebSocketConnection { sender };

                    {
                        let mut current = self.current_connection.lock().await;
                        *current = Some(connection);
                    }

                    self.handle_connection(receiver).await;

                    println!("Client disconnected, waiting for new connection...");
                }
                Err(e) => {
                    eprintln!("Accept error: {}", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            }
        }
    }

    async fn handle_connection(&self, receiver:  stream::SplitStream<WebSocketStream<net::TcpStream>>) {
        let receiver_task = Some(tokio::spawn(async move {
            Self::handle_receiver(receiver).await;
        }));

        if let Some(task) = receiver_task {
            let _ = task.await;
        }

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
    }

    pub async fn send_telemetry(&self, telemetry: &WebSocketMessage) -> Result<(), Box<dyn std::error::Error>> {
        let mut current = self.current_connection.lock().await;

        if let Some(connection) = current.as_mut() {
            let json = serde_json::to_string(telemetry)?;
            let message = Message::from(json);

            match connection.sender.send(message).await {
                Ok(()) => Ok(()),
                Err(e) => {
                    println!("Send failed: {}, closing connection", e);
                    *current = None;
                    Err(e.into())
                }
            }
        } else {
            Ok(())
        }
    }
}