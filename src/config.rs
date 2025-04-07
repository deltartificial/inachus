/// src/config.rs
use crate::error::{Error, Result};
use crate::validation;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents the application configuration.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Directory containing ABI files
    pub abi_dir: PathBuf,
    /// Ethereum RPC URL
    pub rpc_url: String,
    /// Optional private key for transaction signing
    pub private_key: Option<String>,
    /// Chain ID for transaction signing
    pub chain_id: u64,
    /// Wait time for transaction confirmation (e.g., "30s")
    pub wait_time: String,
    /// Optional name of the current contract
    pub contract_name: Option<String>,
    /// Optional address of the current contract
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
    /// Loads a configuration from a file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the configuration file
    ///
    /// # Returns
    ///
    /// * `Result<Config>` - The loaded configuration or an error
    pub fn from_file(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        toml::from_str(&content).map_err(Error::from)
    }

    /// Saves the configuration to a file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path where the configuration will be saved
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Success or an error during saving
    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| Error::Other(format!("Failed to serialize config: {}", e)))?;
        fs::write(path, content)?;
        Ok(())
    }

    /// Validates the configuration.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Success if the configuration is valid, or an error
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

/// Represents information about a contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    /// Name of the contract
    pub name: String,
    /// Address of the deployed contract
    pub address: String,
}

impl ContractInfo {
    /// Loads all contract information from a file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file containing contract information
    ///
    /// # Returns
    ///
    /// * `Result<Vec<ContractInfo>>` - List of contract information or an error
    pub fn load_all(path: &PathBuf) -> Result<Vec<Self>> {
        let content = std::fs::read_to_string(path)?;
        let infos = serde_json::from_str(&content)?;
        Ok(infos)
    }

    /// Saves all contract information to a file.
    ///
    /// # Arguments
    ///
    /// * `infos` - List of contract information to save
    /// * `path` - Path where the information will be saved
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Success or an error during saving
    pub fn save_all(infos: &[Self], path: &PathBuf) -> Result<()> {
        let content = serde_json::to_string_pretty(infos)?;
        std::fs::write(path, content)?;
        Ok(())
    }

    /// Validates the contract information.
    ///
    /// # Returns
    ///
    /// * `Result<()>` - Success if the information is valid, or an error
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

/// Validates a list of contract information.
///
/// # Arguments
///
/// * `infos` - List of contract information to validate
///
/// # Returns
///
/// * `Result<()>` - Success if all information is valid, or an error on the first invalid one
pub fn validate_contract_infos(infos: &[ContractInfo]) -> Result<()> {
    for info in infos {
        info.validate()?;
    }
    Ok(())
}
