use crate::error::{Error, Result};
use alloy::json_abi::JsonAbi;
use alloy::primitives::{Address, U256};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GlobalContext {
    pub abis_dir: PathBuf,
    pub abis: HashMap<String, JsonAbi>,
    pub private_key: String,
    pub rpc_url: String,
    pub chain_id: String,
    pub contract_name: String,
    pub contract_address: Address,
}

impl GlobalContext {
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

    pub fn get_abi(&self) -> Result<&JsonAbi> {
        self.abis.get(&self.contract_name).ok_or_else(|| {
            Error::InvalidContract(format!(
                "ABI not found for contract: {}",
                self.contract_name
            ))
        })
    }
}

#[derive(Debug, Clone)]
pub struct ReadContext {
    pub global: GlobalContext,
}

impl ReadContext {
    pub fn new(global: GlobalContext) -> Self {
        Self { global }
    }
}

#[derive(Debug, Clone)]
pub struct WriteContext {
    pub global: GlobalContext,
}

impl WriteContext {
    pub fn new(global: GlobalContext) -> Self {
        Self { global }
    }
}
