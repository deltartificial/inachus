use std::{collections::HashMap, path::PathBuf};
use alloy_json_abi::{Function, JsonAbi};
use alloy_primitives::{Address, Bytes};
use alloy_providers::Provider;
use alloy_transport_http::Http;
use inachus::{
    abi::{self, MethodType},
    config::{Config, ContractInfo},
    context::Context,
    error::{Error, Result},
    prompt,
    step::Step,
    ABI_DIR, INACHUS_DIR,
};

#[tokio::main]
async fn main() -> Result<()> {
    inachus::init_logging();

    let config_path = PathBuf::from(INACHUS_DIR).join("config.json");
    let contracts_path = PathBuf::from(INACHUS_DIR).join("contracts.json");
    let abi_dir = PathBuf::from(ABI_DIR);

    let mut config = Config::load(&config_path).unwrap_or_default();
    let mut contract_infos = ContractInfo::load_all(&contracts_path).unwrap_or_default();
    let abis = abi::load_abis(&abi_dir)?;

    let mut ctx = Context::new(&config)?;
    ctx.ensure_chain_id().await?;

    loop {
        let step = prompt::select_step()?;
        match step {
            Step::ChangeContract => {
                let contract_names: Vec<String> = abis.keys().cloned().collect();
                let contract_name = prompt::select_contract_name(&contract_names)?;
                let contract_info = ContractInfo {
                    name: contract_name,
                    address: None,
                };
                contract_infos.push(contract_info);
                ContractInfo::save_all(&contract_infos, &contracts_path)?;
            }
            Step::ChangeContractAddress => {
                let contract_names: Vec<String> = contract_infos.iter()
                    .map(|info| info.name.clone())
                    .collect();
                let contract_name = prompt::select_contract_name(&contract_names)?;
                let address = prompt::input_contract_address()?;

                if let Some(info) = contract_infos.iter_mut()
                    .find(|info| info.name == contract_name)
                {
                    info.address = Some(address.clone());
                    ContractInfo::save_all(&contract_infos, &contracts_path)?;
                    ctx.set_contract_address(Address::parse_checksummed(&address, None)?);
                }
            }
            Step::SelectMethod => {
                let current_contract = contract_infos.iter()
                    .find(|info| info.address.is_some())
                    .ok_or_else(|| Error::NoContractSelected)?;

                let abi = abis.get(&current_contract.name)
                    .ok_or_else(|| Error::ContractNotFound(current_contract.name.clone()))?;

                let method_type = prompt::select_method_type()?;
                let methods = abi::get_methods_by_type(abi, method_type);
                let method_name = prompt::select_method(&methods)?;
                let function = methods.get(&method_name)
                    .ok_or_else(|| Error::MethodNotFound(method_name.clone()))?;

                let params = prompt::input_method_params(function)?;
                let result = execute_method(&ctx, function, &params).await?;
                prompt::display_result(&result);
            }
            Step::Exit => break,
        }
    }

    Ok(())
}

async fn execute_method(ctx: &Context, function: &Function, params: &[String]) -> Result<String> {
    let mut encoded_params = Vec::new();
    for (param, value) in function.inputs.iter().zip(params) {
        let param_type = param.ty.to_string();
        let bytes = if param_type.ends_with("[]") || param_type.starts_with("(") {
            if param_type.ends_with("[]") {
                let inner_type = param_type.trim_end_matches("[]");
                abi::parse_array_or_slice_input(value, inner_type)?
            } else {
                let param_types: Vec<String> = param.ty.tuple_elements()
                    .iter()
                    .map(|ty| ty.to_string())
                    .collect();
                abi::parse_tuple_input(value, &param_types)?
            }
        } else {
            let mut bytes = Vec::new();
            match param_type.as_str() {
                "address" => {
                    let addr = Address::parse_checksummed(value, None)?;
                    bytes.push(addr.into());
                }
                "uint256" | "int256" => {
                    let num = alloy_primitives::U256::from_str_radix(value, 10)?;
                    bytes.push(num.into());
                }
                "bool" => {
                    let b = value.parse::<bool>()?;
                    bytes.push(Bytes::from_static(if b { &[1] } else { &[0] }));
                }
                "string" => {
                    bytes.push(Bytes::copy_from_slice(value.as_bytes()));
                }
                "bytes" => {
                    let b = hex::decode(value.trim_start_matches("0x"))?;
                    bytes.push(Bytes::copy_from_slice(&b));
                }
                _ => return Err(Error::UnsupportedType(param_type)),
            }
            bytes
        };
        encoded_params.extend(bytes);
    }

    let contract_address = ctx.contract_address()
        .ok_or_else(|| Error::NoContractSelected)?;

    let result = if function.state_mutability.is_view() || function.state_mutability.is_pure() {
        let data = function.encode_input(&encoded_params)
            .map_err(|e| Error::Abi(e.to_string()))?;

        let result = ctx.provider().call(contract_address, data).await?;
        let decoded = function.decode_output(&result)
            .map_err(|e| Error::Abi(e.to_string()))?;

        format!("{:?}", decoded)
    } else {
        if !prompt::confirm_transaction()? {
            return Ok("Transaction cancelled".to_string());
        }

        let wallet = ctx.wallet()
            .ok_or_else(|| Error::NoWalletConfigured)?;

        let data = function.encode_input(&encoded_params)
            .map_err(|e| Error::Abi(e.to_string()))?;

        let tx = wallet.sign_and_send(contract_address, data).await?;
        format!("Transaction sent: {}", tx.tx_hash())
    };

    Ok(result)
} 