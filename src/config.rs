use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub username: String,
    pub delivery_service_address: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            username: "user".to_string(),
            delivery_service_address: "127.0.0.1:8080".to_string(),
        }
    }
}

impl Config {
    pub async fn load_or_default() -> Result<Self> {
        let config_path = "config.json";
        
        if Path::new(config_path).exists() {
            let content = fs::read_to_string(config_path).await?;
            let config: Config = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save().await?;
            Ok(config)
        }
    }

    pub async fn save(&self) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write("config.json", content).await?;
        Ok(())
    }
}