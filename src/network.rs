use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub sender: String,
    pub recipient: Option<String>,
    pub group_id: Option<String>,
    pub content: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HandshakeMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub client_id: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListKeyPackagesMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub client_id: String,
}

pub struct NetworkClient {
    delivery_service_address: String,
    connected: bool,
    stream: Option<Arc<Mutex<TcpStream>>>,
}

impl NetworkClient {
    pub async fn new(delivery_service_address: &str) -> Result<Self> {
        let mut client = Self {
            delivery_service_address: delivery_service_address.to_string(),
            connected: false,
            stream: None,
        };
        
        // Attempt to connect to the delivery service
        client.connect().await?;
        
        Ok(client)
    }

    pub async fn connect(&mut self) -> Result<()> {
        // Attempt to connect with timeout
        match timeout(Duration::from_secs(5), TcpStream::connect(&self.delivery_service_address)).await {
            Ok(Ok(stream)) => {
                // Send initial message to establish connection
                let list_message = ListKeyPackagesMessage {
                    message_type: "ListKeyPackages".to_string(),
                    client_id: "mls-client".to_string(),
                };
                
                let message_json = serde_json::to_string(&list_message)?;
                let stream_arc = Arc::new(Mutex::new(stream));
                
                // Send initial message
                {
                    let mut stream_guard = stream_arc.lock().unwrap();
                    stream_guard.write_all(message_json.as_bytes()).await?;
                    stream_guard.write_all(b"\n").await?; // Add newline for line-based protocol
                    stream_guard.flush().await?;
                }
                
                self.stream = Some(stream_arc);
                self.connected = true;
                println!("Connected to MLS Delivery Service at {}", self.delivery_service_address);
                Ok(())
            }
            Ok(Err(e)) => {
                self.connected = false;
                self.stream = None;
                // Don't fail completely, just mark as disconnected
                println!("Failed to connect to MLS Delivery Service: {}", e);
                Ok(())
            }
            Err(_) => {
                self.connected = false;
                self.stream = None;
                println!("Connection timeout to MLS Delivery Service");
                Ok(())
            }
        }
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub async fn send_message(&self, message: &NetworkMessage) -> Result<()> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        if let Some(stream_arc) = &self.stream {
            let message_json = serde_json::to_string(message)?;
            let mut stream_guard = stream_arc.lock().unwrap();
            stream_guard.write_all(message_json.as_bytes()).await?;
            stream_guard.write_all(b"\n").await?;
            stream_guard.flush().await?;
            println!("Sending message: {:?}", message);
        }
        
        Ok(())
    }

    pub async fn fetch_messages(&self, _group_id: &str) -> Result<Vec<NetworkMessage>> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        // In a real implementation, this would fetch messages from the delivery service
        // For now, we'll return an empty vector
        Ok(Vec::new())
    }

    pub async fn publish_key_package(&self, key_package: &[u8]) -> Result<()> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        // In a real implementation, this would publish the key package to the delivery service
        println!("Publishing key package ({} bytes)", key_package.len());
        Ok(())
    }

    pub async fn fetch_key_packages(&self, identity: &str) -> Result<Vec<Vec<u8>>> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        // In a real implementation, this would fetch key packages from the delivery service
        println!("Fetching key packages for identity: {}", identity);
        Ok(Vec::new())
    }

    pub async fn create_group(&self, group_id: &str, group_info: &[u8]) -> Result<()> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        // In a real implementation, this would create the group on the delivery service
        println!("Creating group {} ({} bytes)", group_id, group_info.len());
        Ok(())
    }

    pub async fn join_group(&self, group_id: &str, key_package: &[u8]) -> Result<Vec<u8>> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        // In a real implementation, this would join the group on the delivery service
        // and return the Welcome message
        println!("Joining group {} with key package ({} bytes)", group_id, key_package.len());
        Ok(Vec::new())
    }
}