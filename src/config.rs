use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub rpc_url: String,
    pub private_key: Option<String>,
    pub chain_id: u64,
    pub wait_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    pub name: String,
    pub address: String,
}

impl Config {
    pub fn default() -> Self {
        Self {
            rpc_url: "http://localhost:8545".to_string(),
            private_key: None,
            chain_id: 1,
            wait_time: "5s".to_string(),
        }
    }

    pub fn load(path: &PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let content = toml::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        use std::time::Duration;

        // Validate wait_time can be parsed as duration
        self.wait_time
            .parse::<humantime::Duration>()
            .map_err(|e| Error::Config(format!("Invalid wait time: {}", e)))?;

        // Validate RPC URL format
        if !self.rpc_url.starts_with("http://") && !self.rpc_url.starts_with("https://") {
            return Err(Error::Config(
                "RPC URL must start with http:// or https://".to_string(),
            ));
        }

        Ok(())
    }
}

impl ContractInfo {
    pub fn load_all(path: &PathBuf) -> Result<Vec<Self>> {
        let content = std::fs::read_to_string(path)?;
        let infos = serde_json::from_str(&content)?;
        Ok(infos)
    }

    pub fn save_all(infos: &[Self], path: &PathBuf) -> Result<()> {
        let content = serde_json::to_string_pretty(infos)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        if self.name.is_empty() {
            return Err(Error::Config("Contract name cannot be empty".to_string()));
        }
        if self.address.is_empty() {
            return Err(Error::Config(
                "Contract address cannot be empty".to_string(),
            ));
        }
        if !self.address.starts_with("0x") {
            return Err(Error::Config(
                "Contract address must start with 0x".to_string(),
            ));
        }
        Ok(())
    }
}

pub fn validate_contract_infos(infos: &[ContractInfo]) -> Result<()> {
    for info in infos {
        info.validate()?;
    }
    Ok(())
}
