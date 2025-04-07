/// src/validation.rs
use crate::error::{Error, Result};
use alloy::primitives::U256;
use std::str::FromStr;

/// Validates that the RPC URL is correctly formatted.
///
/// # Arguments
///
/// * `url` - The RPC URL to validate
///
/// # Returns
///
/// * `Ok(())` if the URL is valid
/// * `Err(Error)` if the URL is invalid
pub fn validate_rpc_url(url: &str) -> Result<()> {
    if !url.starts_with("http://") && !url.starts_with("https://") {
        return Err(Error::InvalidAddress(format!("Invalid RPC URL: {}", url)));
    }
    Ok(())
}

/// Validates that an Ethereum address is correctly formatted.
///
/// # Arguments
///
/// * `address` - The Ethereum address to validate
///
/// # Returns
///
/// * `Ok(())` if the address is valid
/// * `Err(Error)` if the address is invalid
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

/// Validates that a private key is correctly formatted.
///
/// # Arguments
///
/// * `private_key` - The private key to validate
///
/// # Returns
///
/// * `Ok(())` if the private key is valid
/// * `Err(Error)` if the private key is invalid
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

/// Validates that a chain ID is correctly formatted.
///
/// # Arguments
///
/// * `chain_id` - The chain ID to validate
///
/// # Returns
///
/// * `Ok(())` if the chain ID is valid
/// * `Err(Error)` if the chain ID is invalid
pub fn validate_chain_id(chain_id: &str) -> Result<()> {
    U256::from_str_radix(chain_id, 10).map_err(|_| Error::InvalidChainId(chain_id.to_string()))?;
    Ok(())
}

/// Validates that a wait time is correctly formatted.
///
/// # Arguments
///
/// * `wait_time` - The wait time to validate (e.g., "30s", "1m")
///
/// # Returns
///
/// * `Ok(())` if the wait time is valid
/// * `Err(Error)` if the wait time is invalid
pub fn validate_wait_time(wait_time: &str) -> Result<()> {
    humantime::Duration::from_str(wait_time)
        .map_err(|_| Error::InvalidWaitTime(wait_time.to_string()))?;
    Ok(())
}

/// Validates that a contract name is correctly formatted.
///
/// # Arguments
///
/// * `contract_name` - The contract name to validate
///
/// # Returns
///
/// * `Ok(())` if the contract name is valid
/// * `Err(Error)` if the contract name is invalid
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

/// Validates that a contract address is correctly formatted.
///
/// # Arguments
///
/// * `address` - The contract address to validate
///
/// # Returns
///
/// * `Ok(())` if the contract address is valid
/// * `Err(Error)` if the contract address is invalid
pub fn validate_contract_address(address: &str) -> Result<()> {
    validate_address(address).map_err(|_| Error::InvalidContract(address.to_string()))
}
