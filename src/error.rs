use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid RPC URL: {0}")]
    InvalidRpcUrl(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    #[error("Invalid contract: {0}")]
    InvalidContract(String),

    #[error("Contract not found at address: {0}")]
    ContractNotFound(String),

    #[error("Config error: {0}")]
    Config(String),

    #[error("ABI error: {0}")]
    Abi(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("Signer error: {0}")]
    Signer(String),

    #[error("Prompt error: {0}")]
    Prompt(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Alloy error: {0}")]
    Alloy(String),

    #[error("{0}")]
    Other(String),
}

impl From<alloy_primitives::Bytes> for Error {
    fn from(e: alloy_primitives::Bytes) -> Self {
        Self::Other(format!("Bytes error: {e:?}"))
    }
}

impl From<alloy_providers::ProviderError> for Error {
    fn from(e: alloy_providers::ProviderError) -> Self {
        Self::Provider(e.to_string())
    }
}
