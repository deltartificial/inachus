/// src/abi.rs
use crate::error::{Error, Result};
use alloy::json_abi::{Function, JsonAbi, StateMutability};
use alloy::primitives::{Address, Bytes, U256};
use std::{collections::HashMap, path::Path};

/// Represents the types of methods that can be called on a contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodType {
    /// Read-only methods (view or pure)
    Read,
    /// Methods that modify state
    Write,
    /// All methods regardless of their state mutability
    All,
}

impl std::fmt::Display for MethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MethodType::Read => write!(f, "Read"),
            MethodType::Write => write!(f, "Write"),
            MethodType::All => write!(f, "All"),
        }
    }
}

/// Loads all ABI files from a directory.
///
/// # Arguments
///
/// * `abi_dir` - Path to the directory containing ABI files
///
/// # Returns
///
/// * `Result<HashMap<String, JsonAbi>>` - Map of filenames to parsed ABIs, or an error
pub fn load_abis(abi_dir: &Path) -> Result<HashMap<String, JsonAbi>> {
    let mut abis = HashMap::new();
    for entry in std::fs::read_dir(abi_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "abi") {
            let content = std::fs::read_to_string(&path)?;
            let abi: JsonAbi = serde_json::from_str(&content)
                .map_err(|e| Error::InvalidAbi(format!("Failed to parse ABI: {}", e)))?;
            let name = path
                .file_name()
                .ok_or_else(|| Error::InvalidAbi("Invalid ABI filename".to_string()))?
                .to_string_lossy()
                .to_string();
            abis.insert(name, abi);
        }
    }
    Ok(abis)
}

/// Gets methods from an ABI filtered by the specified method type.
///
/// # Arguments
///
/// * `abi` - The ABI to extract methods from
/// * `method_type` - The type of methods to extract (Read, Write, or All)
///
/// # Returns
///
/// * `HashMap<String, Function>` - Map of method names to Function objects
pub fn get_methods_by_type(abi: &JsonAbi, method_type: MethodType) -> HashMap<String, Function> {
    let mut read_methods = HashMap::new();
    let mut write_methods = HashMap::new();
    let mut all_methods = HashMap::new();

    for function in abi.functions() {
        let name = function.name.clone();
        if matches!(
            function.state_mutability,
            StateMutability::View | StateMutability::Pure
        ) {
            read_methods.insert(name.clone(), function.clone());
        } else {
            write_methods.insert(name.clone(), function.clone());
        }
        all_methods.insert(name, function.clone());
    }

    match method_type {
        MethodType::Read => read_methods,
        MethodType::Write => write_methods,
        MethodType::All => all_methods,
    }
}

/// Parses an array or slice input string into a vector of Bytes.
///
/// # Arguments
///
/// * `input` - The input string representing an array (e.g., "[1, 2, 3]")
/// * `param_type` - The type of elements in the array
///
/// # Returns
///
/// * `Result<Vec<Bytes>>` - Vector of parsed elements as Bytes, or an error
pub fn parse_array_or_slice_input(input: &str, param_type: &str) -> Result<Vec<Bytes>> {
    let input = input.trim().trim_matches(|c| c == '[' || c == ']');
    let parts: Vec<&str> = input.split(',').map(str::trim).collect();

    let mut result = Vec::with_capacity(parts.len());
    for part in parts {
        match param_type {
            "address" => {
                let addr = Address::parse_checksummed(part, None)
                    .map_err(|_| Error::InvalidAddress(format!("Invalid address: {}", part)))?;
                result.push(addr.to_vec().into());
            }
            "uint256" | "int256" => {
                let num = U256::from_str_radix(part, 10)
                    .map_err(|_| Error::InvalidArguments(format!("Invalid number: {}", part)))?;
                result.push(num.to_be_bytes::<32>().into());
            }
            "bool" => {
                let b = part
                    .parse::<bool>()
                    .map_err(|_| Error::InvalidArguments(format!("Invalid boolean: {}", part)))?;
                result.push(Bytes::from_static(if b { &[1] } else { &[0] }));
            }
            "string" => {
                result.push(Bytes::copy_from_slice(part.as_bytes()));
            }
            "bytes" => {
                let bytes = hex::decode(part.trim_start_matches("0x"))
                    .map_err(|_| Error::InvalidArguments(format!("Invalid hex: {}", part)))?;
                result.push(Bytes::copy_from_slice(&bytes));
            }
            _ => {
                return Err(Error::InvalidArguments(format!(
                    "Unsupported array type: {}",
                    param_type
                )))
            }
        }
    }
    Ok(result)
}

/// Parses a tuple input string into a vector of Bytes.
///
/// # Arguments
///
/// * `input` - The input string representing a tuple (e.g., "(1, true, 0x1234)")
/// * `param_types` - The types of elements in the tuple
///
/// # Returns
///
/// * `Result<Vec<Bytes>>` - Vector of parsed elements as Bytes, or an error
pub fn parse_tuple_input(input: &str, param_types: &[String]) -> Result<Vec<Bytes>> {
    let input = input.trim().trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<&str> = input.split(',').map(str::trim).collect();

    if parts.len() != param_types.len() {
        return Err(Error::InvalidArguments(format!(
            "Tuple input length mismatch: expected {}, got {}",
            param_types.len(),
            parts.len()
        )));
    }

    let mut result = Vec::with_capacity(parts.len());
    for (part, param_type) in parts.iter().zip(param_types) {
        match param_type.as_str() {
            "address" => {
                let addr = Address::parse_checksummed(part, None)
                    .map_err(|_| Error::InvalidAddress(format!("Invalid address: {}", part)))?;
                result.push(addr.to_vec().into());
            }
            "uint256" | "int256" => {
                let num = U256::from_str_radix(part, 10)
                    .map_err(|_| Error::InvalidArguments(format!("Invalid number: {}", part)))?;
                result.push(num.to_be_bytes::<32>().into());
            }
            "bool" => {
                let b = part
                    .parse::<bool>()
                    .map_err(|_| Error::InvalidArguments(format!("Invalid boolean: {}", part)))?;
                result.push(Bytes::from_static(if b { &[1] } else { &[0] }));
            }
            "string" => {
                result.push(Bytes::copy_from_slice(part.as_bytes()));
            }
            "bytes" => {
                let bytes = hex::decode(part.trim_start_matches("0x"))
                    .map_err(|_| Error::InvalidArguments(format!("Invalid hex: {}", part)))?;
                result.push(Bytes::copy_from_slice(&bytes));
            }
            _ => {
                return Err(Error::InvalidArguments(format!(
                    "Unsupported tuple type: {}",
                    param_type
                )))
            }
        }
    }
    Ok(result)
}
