use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChainError {
    #[error("Failed to read chain info file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Chain ID {0} not found")]
    ChainNotFound(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainInfo {
    pub name: String,
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "networkId")]
    pub network_id: u64,
    #[serde(rename = "nativeCurrency")]
    pub native_currency: NativeCurrency,
    pub rpc: Vec<String>,
    pub faucets: Vec<String>,
    #[serde(rename = "infoURL")]
    pub info_url: String,
}

impl ChainInfo {
    pub fn parse_chains_json(path: impl AsRef<Path>) -> Result<Vec<ChainInfo>, ChainError> {
        let json_data = fs::read_to_string(path)?;
        let chain_infos = serde_json::from_str(&json_data)?;
        Ok(chain_infos)
    }

    pub fn get_by_id(chain_infos: &[ChainInfo], chain_id: u64) -> Result<&ChainInfo, ChainError> {
        chain_infos
            .iter()
            .find(|info| info.chain_id == chain_id)
            .ok_or(ChainError::ChainNotFound(chain_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_info_serialization() {
        let chain_info = ChainInfo {
            name: "Test Chain".to_string(),
            chain_id: 1,
            short_name: "TEST".to_string(),
            network_id: 1,
            native_currency: NativeCurrency {
                name: "Test Token".to_string(),
                symbol: "TEST".to_string(),
                decimals: 18,
            },
            rpc: vec!["https://test.rpc".to_string()],
            faucets: vec!["https://test.faucet".to_string()],
            info_url: "https://test.info".to_string(),
        };

        let json = serde_json::to_string(&chain_info).unwrap();
        let deserialized: ChainInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.chain_id, chain_info.chain_id);
        assert_eq!(deserialized.name, chain_info.name);
    }

    #[test]
    fn test_get_chain_by_id() {
        let chain_infos = vec![
            ChainInfo {
                name: "Test Chain 1".to_string(),
                chain_id: 1,
                short_name: "TEST1".to_string(),
                network_id: 1,
                native_currency: NativeCurrency {
                    name: "Test Token 1".to_string(),
                    symbol: "TEST1".to_string(),
                    decimals: 18,
                },
                rpc: vec![],
                faucets: vec![],
                info_url: "".to_string(),
            },
            ChainInfo {
                name: "Test Chain 2".to_string(),
                chain_id: 2,
                short_name: "TEST2".to_string(),
                network_id: 2,
                native_currency: NativeCurrency {
                    name: "Test Token 2".to_string(),
                    symbol: "TEST2".to_string(),
                    decimals: 18,
                },
                rpc: vec![],
                faucets: vec![],
                info_url: "".to_string(),
            },
        ];

        let found = ChainInfo::get_by_id(&chain_infos, 1).unwrap();
        assert_eq!(found.chain_id, 1);
        assert_eq!(found.name, "Test Chain 1");

        let not_found = ChainInfo::get_by_id(&chain_infos, 999);
        assert!(matches!(not_found, Err(ChainError::ChainNotFound(999))));
    }
}
