pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid private key: {0}")]
    InvalidPrivateKey(String),

    #[error("Invalid chain ID: {0}")]
    InvalidChainId(String),

    #[error("Invalid wait time: {0}")]
    InvalidWaitTime(String),

    #[error("Invalid contract: {0}")]
    InvalidContract(String),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Invalid function: {0}")]
    InvalidFunction(String),

    #[error("Invalid ABI: {0}")]
    InvalidAbi(String),

    #[error("Invalid arguments: {0}")]
    InvalidArguments(String),

    #[error("Provider error: {0}")]
    Provider(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("TOML error: {0}")]
    Toml(#[from] toml::de::Error),

    #[error("Hex error: {0}")]
    Hex(#[from] hex::FromHexError),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<alloy::primitives::Bytes> for Error {
    fn from(e: alloy::primitives::Bytes) -> Self {
        Error::Other(format!("Bytes error: {:?}", e))
    }
}
