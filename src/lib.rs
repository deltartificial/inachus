/// src/lib.rs
pub mod abi;
pub mod config;
pub mod context;
pub mod error;
pub mod prompt;
pub mod step;
pub mod validation;

use error::Result;

pub use context::{GlobalContext, ReadContext, WriteContext};

/// Directory name for storing Inachus configuration and data.
pub const INACHUS_DIR: &str = ".inachus";

/// Directory name for storing ABI files.
pub const ABI_DIR: &str = "abis";

/// Initializes the application environment, particularly logging.
///
/// # Returns
///
/// * `Result<()>` - Success or an error during initialization
pub fn init() -> Result<()> {
    use tracing_subscriber::{fmt, EnvFilter};

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    Ok(())
}

/// Runs the main application logic.
///
/// # Returns
///
/// * `Result<()>` - Success or an error during application execution
pub fn run() -> Result<()> {
    init()?;
    Ok(())
}
