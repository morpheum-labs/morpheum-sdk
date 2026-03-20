//! Chain configuration registry.
//!
//! Loads per-chain RPC URLs, contract addresses, Hyperlane domains, and token
//! metadata from a TOML config file (`chains.toml`). The CLI resolves
//! human-friendly names like `--chain ethereum --token USDC` to concrete
//! addresses and parameters via this registry.

use std::collections::HashMap;

use alloy::primitives::Address;
use morpheum_sdk_core::ChainRegistryOps;
use serde::{Deserialize, Serialize};

use crate::types::EvmError;

/// Default chains configuration shipped with the SDK.
///
/// This is loaded as the base configuration; users can override individual
/// chains/tokens in `~/.config/morpheum/chains.toml`.
pub const DEFAULT_CHAINS_TOML: &str = include_str!("../config/chains.toml");

/// Top-level chain configuration registry.
///
/// Loaded from `chains.toml`; merges a default config shipped with the SDK
/// and user-local overrides in `~/.config/morpheum/chains.toml`.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ChainRegistry {
    #[serde(default)]
    pub chains: HashMap<String, ChainConfig>,
}

/// Configuration for a single EVM chain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChainConfig {
    pub chain_id: u64,
    pub rpc_url: String,
    pub hyperlane_domain: u32,
    #[serde(default)]
    pub explorer: Option<String>,
    #[serde(default)]
    pub tokens: HashMap<String, TokenConfig>,
}

/// Configuration for a single token on a chain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenConfig {
    pub address: String,
    pub decimals: u8,
    #[serde(default)]
    pub collateral_contract: Option<String>,
    #[serde(default)]
    pub settlement_contract: Option<String>,
    #[serde(default)]
    pub morpheum_asset_index: u64,
}

/// Resolved chain + token configuration with parsed addresses.
#[derive(Clone, Debug)]
pub struct ResolvedChain {
    pub name: String,
    pub chain_id: u64,
    pub rpc_url: String,
    pub hyperlane_domain: u32,
    pub explorer: Option<String>,
}

/// Resolved token configuration with parsed addresses.
#[derive(Clone, Debug)]
pub struct ResolvedToken {
    pub symbol: String,
    pub address: Address,
    pub decimals: u8,
    pub collateral_contract: Option<Address>,
    pub settlement_contract: Option<Address>,
    pub morpheum_asset_index: u64,
}

impl ChainRegistryOps for ChainRegistry {
    type Error = EvmError;

    fn from_toml(content: &str) -> Result<Self, EvmError> {
        toml::from_str(content).map_err(|e| EvmError::Config(format!("TOML parse error: {e}")))
    }

    fn merge(&mut self, other: Self) {
        for (chain_name, other_chain) in other.chains {
            match self.chains.get_mut(&chain_name) {
                Some(existing) => {
                    existing.rpc_url = other_chain.rpc_url;
                    existing.chain_id = other_chain.chain_id;
                    existing.hyperlane_domain = other_chain.hyperlane_domain;
                    if other_chain.explorer.is_some() {
                        existing.explorer = other_chain.explorer;
                    }
                    for (token_name, token_cfg) in other_chain.tokens {
                        existing.tokens.insert(token_name, token_cfg);
                    }
                }
                None => {
                    self.chains.insert(chain_name, other_chain);
                }
            }
        }
    }

    fn override_filename() -> &'static str {
        "chains.toml"
    }

    fn config_error(msg: String) -> EvmError {
        EvmError::Config(msg)
    }
}

impl ChainRegistry {
    /// Resolves a chain by its human-friendly name (case-insensitive).
    pub fn get_chain(&self, name: &str) -> Option<&ChainConfig> {
        let lower = name.to_ascii_lowercase();
        self.chains.get(&lower).or_else(|| {
            let alias = match lower.as_str() {
                "eth" => "ethereum",
                "arb" => "arbitrum",
                "op" => "optimism",
                "avax" => "avalanche",
                "matic" => "polygon",
                "bsc" | "binance" => "bsc",
                "local" => "anvil",
                _ => return None,
            };
            self.chains.get(alias)
        })
    }

    /// Resolves a chain + token pair into parsed addresses.
    pub fn resolve(
        &self,
        chain_name: &str,
        token_symbol: &str,
    ) -> Result<(ResolvedChain, ResolvedToken), EvmError> {
        let chain = self.get_chain(chain_name).ok_or_else(|| {
            EvmError::Config(format!("unknown chain: '{chain_name}'"))
        })?;

        let upper = token_symbol.to_ascii_uppercase();
        let token = chain.tokens.get(&upper).ok_or_else(|| {
            EvmError::Config(format!(
                "token '{token_symbol}' not configured for chain '{chain_name}'"
            ))
        })?;

        let token_address = parse_address(&token.address)?;
        let collateral = token
            .collateral_contract
            .as_deref()
            .map(parse_address)
            .transpose()?;
        let settlement = token
            .settlement_contract
            .as_deref()
            .map(parse_address)
            .transpose()?;

        Ok((
            ResolvedChain {
                name: chain_name.to_string(),
                chain_id: chain.chain_id,
                rpc_url: chain.rpc_url.clone(),
                hyperlane_domain: chain.hyperlane_domain,
                explorer: chain.explorer.clone(),
            },
            ResolvedToken {
                symbol: upper,
                address: token_address,
                decimals: token.decimals,
                collateral_contract: collateral,
                settlement_contract: settlement,
                morpheum_asset_index: token.morpheum_asset_index,
            },
        ))
    }

    /// Lists all configured chain names.
    pub fn chain_names(&self) -> Vec<&str> {
        self.chains.keys().map(String::as_str).collect()
    }
}

fn parse_address(s: &str) -> Result<Address, EvmError> {
    s.parse()
        .map_err(|e| EvmError::AddressParse(format!("'{s}': {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TOML: &str = r#"
[chains.anvil]
chain_id = 31337
rpc_url = "http://127.0.0.1:8545"
hyperlane_domain = 31337

[chains.anvil.tokens.USDC]
address = "0x5FbDB2315678afecb367f032d93F642f64180aa3"
decimals = 6
collateral_contract = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
morpheum_asset_index = 1
"#;

    #[test]
    fn parse_and_resolve() {
        let registry = ChainRegistry::from_toml(TEST_TOML).unwrap();
        let (chain, token) = registry.resolve("anvil", "USDC").unwrap();

        assert_eq!(chain.chain_id, 31337);
        assert_eq!(chain.hyperlane_domain, 31337);
        assert_eq!(token.decimals, 6);
        assert!(token.collateral_contract.is_some());
        assert!(token.settlement_contract.is_none());
        assert_eq!(token.morpheum_asset_index, 1);
    }

    #[test]
    fn alias_resolution() {
        let registry = ChainRegistry::from_toml(TEST_TOML).unwrap();
        assert!(registry.get_chain("local").is_some());
    }

    #[test]
    fn merge_overrides() {
        let mut base = ChainRegistry::from_toml(TEST_TOML).unwrap();
        let override_toml = r#"
[chains.anvil]
chain_id = 31337
rpc_url = "http://custom:8545"
hyperlane_domain = 31337
"#;
        let user = ChainRegistry::from_toml(override_toml).unwrap();
        base.merge(user);
        assert_eq!(base.chains["anvil"].rpc_url, "http://custom:8545");
        assert!(base.chains["anvil"].tokens.contains_key("USDC"));
    }

    #[test]
    fn unknown_chain_errors() {
        let registry = ChainRegistry::from_toml(TEST_TOML).unwrap();
        assert!(registry.resolve("nonexistent", "USDC").is_err());
    }

    #[test]
    fn unknown_token_errors() {
        let registry = ChainRegistry::from_toml(TEST_TOML).unwrap();
        assert!(registry.resolve("anvil", "WBTC").is_err());
    }
}
