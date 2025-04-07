pub mod abi;
pub mod config;
pub mod context;
pub mod error;
pub mod prompt;
pub mod step;
pub mod validation;

// Re-export commonly used types
pub use context::Context;
pub use error::{Error, Result};
pub use step::Step;

// Constants
pub const INACHUS_DIR: &str = ".inachus";
pub const ABI_DIR: &str = "abis";

// Initialize logging
pub fn init() -> Result<()> {
    use tracing_subscriber::{fmt, EnvFilter};

    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    Ok(())
}

pub fn run() -> Result<()> {
    init()?;
    Ok(())
}
