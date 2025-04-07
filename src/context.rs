use crate::{config::Config, error::Result};
use alloy_primitives::{Address, U256};
use alloy_providers::{HttpProvider, Provider};
use alloy_signer::{LocalWallet, Signer};
use alloy_transport_http::Http;
use std::sync::Arc;

#[derive(Clone)]
pub struct Context {
    provider: Arc<Provider<Http>>,
    wallet: Option<LocalWallet>,
    chain_id: U256,
    contract_address: Option<Address>,
}

impl Context {
    pub fn new(config: &Config) -> Result<Self> {
        let provider = Provider::new(HttpProvider::new(config.rpc_url.clone()));
        let provider = Arc::new(provider);

        let wallet = if let Some(ref pk) = config.private_key {
            let wallet = LocalWallet::from_bytes(&hex::decode(pk)?)?;
            Some(wallet)
        } else {
            None
        };

        Ok(Self {
            provider,
            wallet,
            chain_id: U256::from(config.chain_id),
            contract_address: None,
        })
    }

    pub fn provider(&self) -> &Provider<Http> {
        &self.provider
    }

    pub fn wallet(&self) -> Option<&LocalWallet> {
        self.wallet.as_ref()
    }

    pub fn chain_id(&self) -> U256 {
        self.chain_id
    }

    pub fn contract_address(&self) -> Option<Address> {
        self.contract_address
    }

    pub fn set_wallet(&mut self, wallet: LocalWallet) {
        self.wallet = Some(wallet);
    }

    pub fn set_chain_id(&mut self, chain_id: U256) {
        self.chain_id = chain_id;
    }

    pub fn set_contract_address(&mut self, address: Address) {
        self.contract_address = Some(address);
    }

    pub async fn ensure_chain_id(&mut self) -> Result<()> {
        let chain_id = self.provider.get_chainid().await?;
        if chain_id != self.chain_id {
            tracing::warn!(
                "Chain ID mismatch: config={}, node={}",
                self.chain_id,
                chain_id
            );
        }
        self.chain_id = chain_id;
        Ok(())
    }
}

pub fn print_context(ctx: &Context, chain_infos: &[chain_info::ChainInfo]) {
    use colored::*;
    use text::pad_right_ansi_aware;

    const CONTENT_WIDTH: usize = 35;

    println!("╔═════════════════════════════════════╗");
    println!(
        "║         {}       ║",
        "Current Configuration".yellow().bold()
    );
    println!("╟─────────────────────────────────────╢");

    // Print RPC URL
    println!(
        "║ {} ║",
        pad_right_ansi_aware(
            &format!("{}: {}", "RPC URL".cyan().bold(), ctx.provider().url()),
            CONTENT_WIDTH
        )
    );

    // Print Chain ID
    println!(
        "║ {} ║",
        pad_right_ansi_aware(
            &format!("{}: {}", "Chain ID".cyan().bold(), ctx.chain_id()),
            CONTENT_WIDTH
        )
    );

    // Print chain info if available
    if let Some(chain_info) = chain_infos
        .iter()
        .find(|c| c.chain_id == ctx.chain_id().as_u64())
    {
        println!(
            "║ {} ║",
            pad_right_ansi_aware(
                &format!("{}: {}", "Chain Name".cyan().bold(), chain_info.name),
                CONTENT_WIDTH
            )
        );
        println!(
            "║ {} ║",
            pad_right_ansi_aware(
                &format!(
                    "{}: {} ({})",
                    "Native Currency".cyan().bold(),
                    chain_info.native_currency.name,
                    chain_info.native_currency.symbol
                ),
                CONTENT_WIDTH
            )
        );
        println!(
            "║ {} ║",
            pad_right_ansi_aware(
                &format!(
                    "{}: {}",
                    "Decimals".cyan().bold(),
                    chain_info.native_currency.decimals
                ),
                CONTENT_WIDTH
            )
        );
    }

    println!("╚═════════════════════════════════════╝");
    println!();
}
