/// src/step.rs
use serde::{Deserialize, Serialize};

/// Represents the various steps or actions that can be taken in the application workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Step {
    /// Change the current contract being interacted with
    ChangeContract,
    /// Change the address of the current contract
    ChangeContractAddress,
    /// Select a method to call on the current contract
    SelectMethod,
    /// Exit the application
    Exit,
}

impl std::fmt::Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Step::ChangeContract => write!(f, "Change contract"),
            Step::ChangeContractAddress => write!(f, "Change contract address"),
            Step::SelectMethod => write!(f, "Select method"),
            Step::Exit => write!(f, "Exit"),
        }
    }
}

impl Step {
    /// Returns a slice containing all available steps in the application.
    ///
    /// # Returns
    ///
    /// * A static slice containing all Step variants
    pub fn all() -> &'static [Step] {
        &[
            Step::ChangeContract,
            Step::ChangeContractAddress,
            Step::SelectMethod,
            Step::Exit,
        ]
    }
}
