use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::timeout;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub message_type: String,
    pub sender: String,
    pub recipient: Option<String>,
    pub group_id: Option<String>,
    pub content: Vec<u8>,
    pub timestamp: u64,
}

pub struct NetworkClient {
    delivery_service_address: String,
    connected: bool,
    websocket: Option<tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>>,
}

impl NetworkClient {
    pub async fn new(delivery_service_address: &str) -> Result<Self> {
        let mut client = Self {
            delivery_service_address: delivery_service_address.to_string(),
            connected: false,
            websocket: None,
        };
        
        // Attempt to connect to the delivery service
        client.connect().await?;
        
        Ok(client)
    }

    pub async fn connect(&mut self) -> Result<()> {
        // Convert address to WebSocket URL
        let ws_url = if self.delivery_service_address.starts_with("ws://") || self.delivery_service_address.starts_with("wss://") {
            self.delivery_service_address.clone()
        } else {
            format!("ws://{}", self.delivery_service_address)
        };
        
        let url = Url::parse(&ws_url)?;
        
        // Attempt to connect with timeout
        match timeout(Duration::from_secs(5), connect_async(url)).await {
            Ok(Ok((ws_stream, _))) => {
                self.websocket = Some(ws_stream);
                self.connected = true;
                println!("Connected to MLS Delivery Service at {}", self.delivery_service_address);
                Ok(())
            }
            Ok(Err(e)) => {
                self.connected = false;
                self.websocket = None;
                println!("Failed to connect to MLS Delivery Service: {}", e);
                Ok(())
            }
            Err(_) => {
                self.connected = false;
                self.websocket = None;
                println!("Connection timeout to MLS Delivery Service");
                Ok(())
            }
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected && self.websocket.is_some()
    }

    pub async fn send_message(&self, message: &NetworkMessage) -> Result<()> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        println!("Would send message: {:?}", message);
        Ok(())
    }

    pub async fn fetch_messages(&self, _group_id: &str) -> Result<Vec<NetworkMessage>> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        println!("Would fetch messages for group: {}", _group_id);
        Ok(Vec::new())
    }

    pub async fn publish_key_package(&self, key_package: &[u8]) -> Result<()> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        println!("Would publish key package ({} bytes)", key_package.len());
        Ok(())
    }

    pub async fn fetch_key_packages(&self, identity: &str) -> Result<Vec<Vec<u8>>> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        println!("Would fetch key packages for identity: {}", identity);
        Ok(Vec::new())
    }

    pub async fn create_group(&mut self, group_id: &str, group_info: &[u8]) -> Result<()> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        println!("Would create group {} ({} bytes) via WebSocket", group_id, group_info.len());
        Ok(())
    }

    pub async fn join_group(&mut self, group_id: &str, key_package: &[u8]) -> Result<Vec<u8>> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        println!("Would join group {} with key package ({} bytes) via WebSocket", group_id, key_package.len());
        
        // For now, return empty to indicate group not found
        // In a real implementation, this would send the join request and wait for response
        Ok(Vec::new())
    }
}