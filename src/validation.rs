use crate::error::{Error, Result};
use alloy_primitives::{Address, U256};

pub fn validate_rpc_url(url: &str) -> Result<()> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(Error::InvalidRpcUrl(url.to_string()));
    }
    Ok(())
}

pub fn validate_address(address: &str) -> Result<Address> {
    Address::parse_checksummed(address, None)
        .map_err(|_| Error::InvalidAddress(address.to_string()))
}

pub fn validate_private_key(private_key: &str) -> Result<()> {
    let private_key = private_key.trim_start_matches("0x");
    if private_key.len() != 64 || !private_key.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(Error::InvalidPrivateKey);
    }
    Ok(())
}

pub fn validate_chain_id(chain_id: &str) -> Result<U256> {
    U256::from_str_radix(chain_id, 10).map_err(|_| Error::InvalidChainId(chain_id.to_string()))
}

pub fn validate_wait_time(wait_time: &str) -> Result<u64> {
    wait_time
        .parse::<u64>()
        .map_err(|_| Error::InvalidWaitTime(wait_time.to_string()))
}

pub fn validate_contract_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(Error::InvalidContractName(
            "Contract name cannot be empty".to_string(),
        ));
    }
    if !name.ends_with(".abi") {
        return Err(Error::InvalidContractName(
            "Contract name must end with .abi".to_string(),
        ));
    }
    Ok(())
}

pub fn validate_contract_address(address: &str) -> Result<Address> {
    validate_address(address).map_err(|_| Error::InvalidContractAddress(address.to_string()))
}
