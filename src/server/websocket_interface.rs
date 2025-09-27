use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::{Message, Error};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::broadcast;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub message_type: String,
    pub data: serde_json::Value,
    pub timestamp: u64,
    pub sequence_id: u64,
}

#[derive(Debug, Clone)]
pub struct ClientSession {
    pub id: u64,
    pub connected_at: std::time::Instant,
}

pub struct WebSocketServer {
    listener: TcpListener,
    telemetry_sender: broadcast::Sender<WebSocketMessage>,
    next_client_id: Arc<RwLock<u64>>,
}