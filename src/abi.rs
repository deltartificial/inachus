use crate::error::{Error, Result};
use alloy_json_abi::{Function, JsonAbi};
use alloy_primitives::{Address, Bytes, U256};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodType {
    Read,
    Write,
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

pub fn load_abis(abi_dir: &Path) -> Result<HashMap<String, JsonAbi>> {
    let mut abis = HashMap::new();
    for entry in std::fs::read_dir(abi_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "abi") {
            let content = std::fs::read_to_string(&path)?;
            let abi: JsonAbi = serde_json::from_str(&content)
                .map_err(|e| Error::Abi(format!("Failed to parse ABI: {}", e)))?;
            let name = path
                .file_name()
                .ok_or_else(|| Error::Abi("Invalid ABI filename".to_string()))?
                .to_string_lossy()
                .to_string();
            abis.insert(name, abi);
        }
    }
    Ok(abis)
}

pub fn get_methods_by_type(abi: &JsonAbi, method_type: MethodType) -> HashMap<String, Function> {
    let mut read_methods = HashMap::new();
    let mut write_methods = HashMap::new();
    let mut all_methods = HashMap::new();

    for function in abi.functions() {
        let name = function.name.clone();
        if function.state_mutability.is_view() || function.state_mutability.is_pure() {
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

pub fn parse_array_or_slice_input(input: &str, param_type: &str) -> Result<Vec<Bytes>> {
    let input = input.trim().trim_matches(|c| c == '[' || c == ']');
    let parts: Vec<&str> = input.split(',').map(str::trim).collect();

    let mut result = Vec::with_capacity(parts.len());
    for part in parts {
        match param_type {
            "address" => {
                let addr = Address::parse_checksummed(part, None)
                    .map_err(|_| Error::Abi(format!("Invalid address: {}", part)))?;
                result.push(addr.into());
            }
            "uint256" | "int256" => {
                let num = U256::from_str_radix(part, 10)
                    .map_err(|_| Error::Abi(format!("Invalid number: {}", part)))?;
                result.push(num.into());
            }
            "bool" => {
                let b = part
                    .parse::<bool>()
                    .map_err(|_| Error::Abi(format!("Invalid boolean: {}", part)))?;
                result.push(Bytes::from_static(if b { &[1] } else { &[0] }));
            }
            "string" => {
                result.push(Bytes::copy_from_slice(part.as_bytes()));
            }
            "bytes" => {
                let bytes = hex::decode(part.trim_start_matches("0x"))
                    .map_err(|_| Error::Abi(format!("Invalid hex: {}", part)))?;
                result.push(Bytes::copy_from_slice(&bytes));
            }
            _ => {
                return Err(Error::Abi(format!(
                    "Unsupported array type: {}",
                    param_type
                )))
            }
        }
    }
    Ok(result)
}

pub fn parse_tuple_input(input: &str, param_types: &[String]) -> Result<Vec<Bytes>> {
    let input = input.trim().trim_matches(|c| c == '(' || c == ')');
    let parts: Vec<&str> = input.split(',').map(str::trim).collect();

    if parts.len() != param_types.len() {
        return Err(Error::Abi(format!(
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
                    .map_err(|_| Error::Abi(format!("Invalid address: {}", part)))?;
                result.push(addr.into());
            }
            "uint256" | "int256" => {
                let num = U256::from_str_radix(part, 10)
                    .map_err(|_| Error::Abi(format!("Invalid number: {}", part)))?;
                result.push(num.into());
            }
            "bool" => {
                let b = part
                    .parse::<bool>()
                    .map_err(|_| Error::Abi(format!("Invalid boolean: {}", part)))?;
                result.push(Bytes::from_static(if b { &[1] } else { &[0] }));
            }
            "string" => {
                result.push(Bytes::copy_from_slice(part.as_bytes()));
            }
            "bytes" => {
                let bytes = hex::decode(part.trim_start_matches("0x"))
                    .map_err(|_| Error::Abi(format!("Invalid hex: {}", part)))?;
                result.push(Bytes::copy_from_slice(&bytes));
            }
            _ => {
                return Err(Error::Abi(format!(
                    "Unsupported tuple type: {}",
                    param_type
                )))
            }
        }
    }
    Ok(result)
}
