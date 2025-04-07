use alloy_json_abi::Function;
use colored::Colorize;
use inquire::{Select, Text};
use std::collections::HashMap;

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
        .map_err(|e| Error::Prompt(e.to_string()))?;
    Ok(step)
}

pub fn select_contract_name(contract_names: &[String]) -> Result<String> {
    let contract_name = Select::new("Select a contract:", contract_names.to_vec())
        .prompt()
        .map_err(|e| Error::Prompt(e.to_string()))?;
    Ok(contract_name)
}

pub fn input_contract_address() -> Result<String> {
    let address = Text::new("Enter contract address:")
        .with_validator(|input: &str| {
            validation::validate_contract_address(input)
                .map(|_| ())
                .map_err(|e| format!("{}", e))
        })
        .prompt()
        .map_err(|e| Error::Prompt(e.to_string()))?;
    Ok(address)
}

pub fn select_method_type() -> Result<MethodType> {
    let method_types = vec![MethodType::Read, MethodType::Write, MethodType::All];
    let method_type = Select::new("Select method type:", method_types)
        .prompt()
        .map_err(|e| Error::Prompt(e.to_string()))?;
    Ok(method_type)
}

pub fn select_method(methods: &HashMap<String, Function>) -> Result<String> {
    let method_names: Vec<String> = methods.keys().cloned().collect();
    let method_name = Select::new("Select a method:", method_names)
        .prompt()
        .map_err(|e| Error::Prompt(e.to_string()))?;
    Ok(method_name)
}

pub fn input_method_params(function: &Function) -> Result<Vec<String>> {
    let mut params = Vec::new();
    for param in function.inputs.iter() {
        let param_name = param.name.as_deref().unwrap_or("unnamed");
        let param_type = &param.ty;
        let prompt = format!("Enter {} ({}):", param_name, param_type);
        let value = Text::new(&prompt)
            .prompt()
            .map_err(|e| Error::Prompt(e.to_string()))?;
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
        .map_err(|e| Error::Prompt(e.to_string()))?;
    Ok(confirm == "Yes")
}

pub fn display_result(result: &str) {
    println!("\n{}", "Result:".green());
    println!("{}", result);
}
