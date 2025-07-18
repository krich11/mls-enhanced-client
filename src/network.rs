use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio::io::AsyncWriteExt;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

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
pub struct ListKeyPackagesMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub client_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateGroupMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub group_id: String,
    pub creator_id: String,
    pub group_info: String, // base64 encoded
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JoinGroupMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub group_id: String,
    pub client_id: String,
    pub key_package: String, // base64 encoded
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListGroupsMessage {
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
                    message_type: "list_key_packages".to_string(),
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

    pub async fn create_group(&self, group_id: &str, group_info: &[u8], creator_id: &str) -> Result<()> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        if let Some(stream_arc) = &self.stream {
            let create_message = CreateGroupMessage {
                message_type: "create_group".to_string(),
                group_id: group_id.to_string(),
                creator_id: creator_id.to_string(),
                group_info: BASE64.encode(group_info),
            };
            
            let message_json = serde_json::to_string(&create_message)?;
            let mut stream_guard = stream_arc.lock().unwrap();
            stream_guard.write_all(message_json.as_bytes()).await?;
            stream_guard.write_all(b"\n").await?;
            stream_guard.flush().await?;
            println!("Creating group {} ({} bytes) on server", group_id, group_info.len());
        }
        
        Ok(())
    }

    pub async fn join_group(&self, group_id: &str, key_package: &[u8], client_id: &str) -> Result<Vec<u8>> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        if let Some(stream_arc) = &self.stream {
            let join_message = JoinGroupMessage {
                message_type: "join_group".to_string(),
                group_id: group_id.to_string(),
                client_id: client_id.to_string(),
                key_package: BASE64.encode(key_package),
            };
            
            let message_json = serde_json::to_string(&join_message)?;
            let mut stream_guard = stream_arc.lock().unwrap();
            stream_guard.write_all(message_json.as_bytes()).await?;
            stream_guard.write_all(b"\n").await?;
            stream_guard.flush().await?;
            println!("Joining group {} with key package ({} bytes) on server", group_id, key_package.len());
        }
        
        // For now, return empty to indicate group not found
        // In a real implementation, this would wait for a response from the server
        Ok(Vec::new())
    }

    pub async fn list_groups(&self) -> Result<Vec<String>> {
        if !self.connected {
            return Err(anyhow::anyhow!("Not connected to delivery service"));
        }
        
        if let Some(stream_arc) = &self.stream {
            let list_message = ListGroupsMessage {
                message_type: "list_groups".to_string(),
                client_id: "mls-client".to_string(),
            };
            
            let message_json = serde_json::to_string(&list_message)?;
            let mut stream_guard = stream_arc.lock().unwrap();
            stream_guard.write_all(message_json.as_bytes()).await?;
            stream_guard.write_all(b"\n").await?;
            stream_guard.flush().await?;
            println!("Requesting list of groups from server");
        }
        
        // For now, return empty list
        // In a real implementation, this would wait for a response from the server
        Ok(Vec::new())
    }
}