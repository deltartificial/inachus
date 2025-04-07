/// src/error.rs

/// Represents the result type for Inachus operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents errors that can occur during Inachus operations.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error related to invalid private key format or content.
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    /// Error related to invalid chain ID format or content.
    #[error("Invalid chain ID: {0}")]
    InvalidChainId(String),

    /// Error related to invalid wait time format.
    #[error("Invalid wait time: {0}")]
    InvalidWaitTime(String),

    /// Error related to invalid contract specification.
    #[error("Invalid contract: {0}")]
    InvalidContract(String),

    /// Error related to invalid Ethereum address format.
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    /// Error related to invalid or non-existent function.
    #[error("Invalid function: {0}")]
    InvalidFunction(String),

    /// Error related to invalid ABI format or content.
    #[error("Invalid ABI: {0}")]
    InvalidAbi(String),

    /// Error related to invalid function arguments.
    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    /// Error from the Ethereum provider.
    #[error("Provider error: {0}")]
    Provider(String),

    /// IO error during file operations.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON parsing or serialization error.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// TOML parsing or serialization error.
    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    /// Hexadecimal conversion error.
    #[error("Hex error: {0}")]
    Hex(#[from] hex::FromHexError),

    /// Any other error not covered by the specific variants.
    #[error("Other error: {0}")]
    Other(String),
}

impl From<alloy::primitives::Bytes> for Error {
    fn from(e: alloy::primitives::Bytes) -> Self {
        Error::Other(format!("Bytes error: {:?}", e))
    }
}
