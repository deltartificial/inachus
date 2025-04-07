pub mod abi;
pub mod config;
pub mod context;
pub mod error;
pub mod prompt;
pub mod step;
pub mod validation;

use error::Result;

pub use context::{GlobalContext, ReadContext, WriteContext};

pub const INACHUS_DIR: &str = ".inachus";
pub const ABI_DIR: &str = "abis";

pub fn init() -> Result<()> {
    use tracing_subscriber::{fmt, EnvFilter};

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    Ok(())
}

pub fn run() -> Result<()> {
    init()?;
    Ok(())
}
