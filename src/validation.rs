use crate::error::{Error, Result};
use alloy::primitives::U256;
use std::str::FromStr;

pub fn validate_rpc_url(url: &str) -> Result<()> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(Error::InvalidAddress(format!("Invalid RPC URL: {}", url)));
    }
    Ok(())
}

pub fn validate_address(address: &str) -> Result<()> {
    if !address.starts_with("0x") {
        return Err(Error::InvalidAddress(
            "Address must start with 0x".to_string(),
        ));
    }

    if address.len() != 42 {
        return Err(Error::InvalidAddress(
            "Address must be 20 bytes (40 hex characters)".to_string(),
        ));
    }

    if !address[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(Error::InvalidAddress(
            "Address must be hexadecimal".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_private_key(private_key: &str) -> Result<()> {
    if private_key.len() != 64 && private_key.len() != 66 {
        return Err(Error::InvalidPrivateKey(
            "Private key must be 32 bytes (64 hex characters)".to_string(),
        ));
    }

    if !private_key
        .trim_start_matches("0x")
        .chars()
        .all(|c| c.is_ascii_hexdigit())
    {
        return Err(Error::InvalidPrivateKey(
            "Private key must be hexadecimal".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_chain_id(chain_id: &str) -> Result<()> {
    U256::from_str_radix(chain_id, 10).map_err(|_| Error::InvalidChainId(chain_id.to_string()))?;
    Ok(())
}

pub fn validate_wait_time(wait_time: &str) -> Result<()> {
    humantime::Duration::from_str(wait_time)
        .map_err(|_| Error::InvalidWaitTime(wait_time.to_string()))?;
    Ok(())
}

pub fn validate_contract_name(contract_name: &str) -> Result<()> {
    if contract_name.is_empty() {
        return Err(Error::InvalidContract(
            "Contract name cannot be empty".to_string(),
        ));
    }

    if !contract_name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_')
    {
        return Err(Error::InvalidContract(
            "Contract name must be alphanumeric or underscore".to_string(),
        ));
    }

    Ok(())
}

pub fn validate_contract_address(address: &str) -> Result<()> {
    validate_address(address).map_err(|_| Error::InvalidContract(address.to_string()))
}
