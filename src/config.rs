use crate::error::{Error, Result};
use crate::validation;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub abi_dir: PathBuf,
    pub rpc_url: String,
    pub private_key: Option<String>,
    pub chain_id: u64,
    pub wait_time: String,
    pub contract_name: Option<String>,
    pub contract_address: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            abi_dir: PathBuf::from("./abis"),
            rpc_url: "http://localhost:8545".to_string(),
            private_key: None,
            chain_id: 1,
            wait_time: "30s".to_string(),
            contract_name: None,
            contract_address: None,
        }
    }
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        toml::from_str(&content).map_err(Error::from)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| Error::Other(format!("Failed to serialize config: {}", e)))?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn validate(&self) -> Result<()> {
        validation::validate_rpc_url(&self.rpc_url)?;

        validation::validate_chain_id(&self.chain_id.to_string())?;

        validation::validate_wait_time(&self.wait_time)
            .map_err(|e| Error::InvalidWaitTime(format!("Invalid wait time: {}", e)))?;

        if let Some(ref pk) = self.private_key {
            validation::validate_private_key(pk)?;
        }

        if let Some(ref name) = self.contract_name {
            if name.is_empty() {
                return Err(Error::InvalidContract(
                    "Contract name cannot be empty".to_string(),
                ));
            }

            validation::validate_contract_name(name)?;
        }

        if let Some(ref addr) = self.contract_address {
            if addr.is_empty() {
                return Err(Error::InvalidContract(
                    "Contract address cannot be empty".to_string(),
                ));
            }

            validation::validate_contract_address(addr)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    pub name: String,
    pub address: String,
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
            return Err(Error::InvalidContract(
                "Contract name cannot be empty".to_string(),
            ));
        }
        if self.address.is_empty() {
            return Err(Error::InvalidContract(
                "Contract address cannot be empty".to_string(),
            ));
        }
        if !self.address.starts_with("0x") {
            return Err(Error::InvalidContract(
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
