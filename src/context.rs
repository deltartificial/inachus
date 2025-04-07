/// src/context.rs
use crate::error::{Error, Result};
use alloy::json_abi::JsonAbi;
use alloy::primitives::{Address, U256};
use std::collections::HashMap;
use std::path::PathBuf;

/// Global context holding the application's state and configuration.
#[derive(Debug, Clone)]
pub struct GlobalContext {
    /// Directory containing ABI files
    pub abis_dir: PathBuf,
    /// Map of contract names to their parsed ABIs
    pub abis: HashMap<String, JsonAbi>,
    /// User's private key for transaction signing
    pub private_key: String,
    /// Ethereum RPC URL
    pub rpc_url: String,
    /// Chain ID for transaction signing
    pub chain_id: String,
    /// Name of the current contract being interacted with
    pub contract_name: String,
    /// Address of the current contract being interacted with
    pub contract_address: Address,
}

impl GlobalContext {
    /// Creates a new GlobalContext with the provided values.
    ///
    /// # Arguments
    ///
    /// * `abis_dir` - Path to the directory containing ABI files
    /// * `abis` - Map of contract names to their parsed ABIs
    /// * `rpc_url` - Ethereum RPC URL
    /// * `private_key` - User's private key for transaction signing
    /// * `chain_id` - Chain ID for transaction signing
    /// * `contract_name` - Name of the current contract
    /// * `contract_address` - Address of the current contract
    ///
    /// # Returns
    ///
    /// * `Result<GlobalContext>` - A new GlobalContext or an error if validation fails
    pub fn new(
        abis_dir: PathBuf,
        abis: HashMap<String, JsonAbi>,
        rpc_url: &str,
        private_key: &str,
        chain_id: &str,
        contract_name: &str,
        contract_address: &str,
    ) -> Result<Self> {
        U256::from_str_radix(chain_id, 10)
            .map_err(|_| Error::InvalidChainId(chain_id.to_string()))?;

        if !private_key
            .trim_start_matches("0x")
            .chars()
            .all(|c| c.is_ascii_hexdigit())
        {
            return Err(Error::InvalidPrivateKey(
                "Invalid private key format".to_string(),
            ));
        }

        let contract_address = Address::parse_checksummed(contract_address, None)
            .map_err(|_| Error::InvalidAddress(contract_address.to_string()))?;

        Ok(Self {
            abis_dir,
            abis,
            private_key: private_key.to_string(),
            rpc_url: rpc_url.to_string(),
            chain_id: chain_id.to_string(),
            contract_name: contract_name.to_string(),
            contract_address,
        })
    }

    /// Gets the ABI for the current contract.
    ///
    /// # Returns
    ///
    /// * `Result<&JsonAbi>` - The ABI or an error if not found
    pub fn get_abi(&self) -> Result<&JsonAbi> {
        self.abis.get(&self.contract_name).ok_or_else(|| {
            Error::InvalidContract(format!(
                "ABI not found for contract: {}",
                self.contract_name
            ))
        })
    }
}

/// Context for read-only operations.
#[derive(Debug, Clone)]
pub struct ReadContext {
    /// The global context
    pub global: GlobalContext,
}

impl ReadContext {
    /// Creates a new ReadContext with the provided global context.
    ///
    /// # Arguments
    ///
    /// * `global` - The global context
    ///
    /// # Returns
    ///
    /// * `ReadContext` - A new ReadContext
    pub fn new(global: GlobalContext) -> Self {
        Self { global }
    }
}

/// Context for write operations.
#[derive(Debug, Clone)]
pub struct WriteContext {
    /// The global context
    pub global: GlobalContext,
}

impl WriteContext {
    /// Creates a new WriteContext with the provided global context.
    ///
    /// # Arguments
    ///
    /// * `global` - The global context
    ///
    /// # Returns
    ///
    /// * `WriteContext` - A new WriteContext
    pub fn new(global: GlobalContext) -> Self {
        Self { global }
    }
}
