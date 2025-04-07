use alloy::json_abi::Function;
use colored::Colorize;
use inquire::{validator::Validation, Select, Text};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::{
    abi::MethodType,
    error::{Error, Result},
    step::Step,
    validation,
};

pub fn select_step() -> Result<Step> {
    let steps = Step::all();
    let step = Select::new("Select an action:", steps.to_vec())
        .prompt()
        .map_err(|e| Error::Other(e.to_string()))?;
    Ok(step)
}

pub fn select_contract_name(contract_names: &[String]) -> Result<String> {
    let contract_name = Select::new("Select a contract:", contract_names.to_vec())
        .prompt()
        .map_err(|e| Error::Other(e.to_string()))?;
    Ok(contract_name)
}

pub fn input_contract_address() -> Result<String> {
    let address = Text::new("Enter contract address:")
        .with_validator(|input: &str| -> std::result::Result<Validation, Box<dyn std::error::Error + Send + Sync>> {
            match validation::validate_contract_address(input) {
                Ok(_) => Ok(Validation::Valid),
                Err(e) => Ok(Validation::Invalid(e.to_string().into()))
            }
        })
        .prompt()
        .map_err(|e| Error::Other(e.to_string()))?;
    Ok(address)
}

pub fn select_method_type() -> Result<MethodType> {
    let method_types = vec![MethodType::Read, MethodType::Write, MethodType::All];
    let method_type = Select::new("Select method type:", method_types)
        .prompt()
        .map_err(|e| Error::Other(e.to_string()))?;
    Ok(method_type)
}

pub fn select_method(methods: &HashMap<String, Function>) -> Result<String> {
    let method_names: Vec<String> = methods.keys().cloned().collect();
    let method_name = Select::new("Select a method:", method_names)
        .prompt()
        .map_err(|e| Error::Other(e.to_string()))?;
    Ok(method_name)
}

pub fn input_method_params(function: &Function) -> Result<Vec<String>> {
    let mut params = Vec::new();
    for param in function.inputs.iter() {
        let param_name = if param.name.is_empty() {
            "unnamed"
        } else {
            &param.name
        };
        let param_type = &param.ty;
        let prompt = format!("Enter {} ({}):", param_name, param_type);
        let value = Text::new(&prompt)
            .prompt()
            .map_err(|e| Error::Other(e.to_string()))?;
        params.push(value);
    }
    Ok(params)
}

pub fn confirm_transaction() -> Result<bool> {
    println!(
        "{}",
        "Warning: This is a write operation that will modify the blockchain state.".yellow()
    );
    let confirm = Select::new("Do you want to proceed?", vec!["Yes", "No"])
        .prompt()
        .map_err(|e| Error::Other(e.to_string()))?;
    Ok(confirm == "Yes")
}

pub fn display_result(result: &str) {
    println!("\n{}", "Result:".green());
    println!("{}", result);
}

pub fn prompt_abi_dir() -> Result<PathBuf> {
    Text::new("Enter the path to the ABI directory:")
        .with_default("./abis")
        .with_help_message("Directory containing ABI JSON files")
        .prompt()
        .map(PathBuf::from)
        .map_err(|e| Error::Other(e.to_string()))
}

pub fn prompt_contract_name() -> Result<String> {
    Text::new("Enter the contract name:")
        .with_help_message("Name of the contract to interact with")
        .prompt()
        .map_err(|e| Error::Other(e.to_string()))
}

pub fn prompt_contract_address() -> Result<String> {
    Text::new("Enter the contract address:")
        .with_help_message("Ethereum address of the deployed contract")
        .with_validator(|input: &str| -> std::result::Result<Validation, Box<dyn std::error::Error + Send + Sync>> {
            match validation::validate_contract_address(input) {
                Ok(_) => Ok(Validation::Valid),
                Err(e) => Ok(Validation::Invalid(e.to_string().into()))
            }
        })
        .prompt()
        .map_err(|e| Error::Other(e.to_string()))
}

pub fn prompt_rpc_url() -> Result<String> {
    Text::new("Enter the Ethereum RPC URL:")
        .with_default("http://localhost:8545")
        .with_help_message("URL of the Ethereum JSON-RPC endpoint")
        .prompt()
        .map_err(|e| Error::Other(e.to_string()))
}

pub fn prompt_private_key() -> Result<String> {
    Text::new("Enter your private key (without 0x prefix):")
        .with_help_message("Private key for transaction signing")
        .prompt()
        .map_err(|e| Error::Other(e.to_string()))
}

pub fn prompt_chain_id() -> Result<String> {
    Text::new("Enter the chain ID:")
        .with_default("1")
        .with_help_message("Chain ID for transaction signing (1 for Ethereum Mainnet)")
        .prompt()
        .map_err(|e| Error::Other(e.to_string()))
}

pub fn prompt_method(methods: &[Function]) -> Result<Function> {
    let method_names: Vec<String> = methods.iter().map(|m| m.name.clone()).collect();

    let selected_name = Select::new("Select a method to call:", method_names)
        .with_help_message("Choose a contract method to execute")
        .prompt()
        .map_err(|e| Error::Other(e.to_string()))?;

    methods
        .iter()
        .find(|m| m.name == selected_name)
        .cloned()
        .ok_or_else(|| Error::InvalidFunction(format!("Method {} not found", selected_name)))
}
