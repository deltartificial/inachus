use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Step {
    ChangeContract,
    ChangeContractAddress,
    SelectMethod,
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
    pub fn all() -> &'static [Step] {
        &[
            Step::ChangeContract,
            Step::ChangeContractAddress,
            Step::SelectMethod,
            Step::Exit,
        ]
    }
}
