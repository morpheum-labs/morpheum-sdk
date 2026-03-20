//! Solana chain configuration registry.
//!
//! Loads per-chain RPC URLs, program IDs, Hyperlane domains, and token
//! metadata from a TOML config file. The CLI resolves human-friendly names
//! like `--chain solana --token USDC` to concrete addresses and parameters.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

use crate::types::SvmError;

/// Default Solana chains configuration shipped with the SDK.
pub const DEFAULT_CHAINS_TOML: &str = include_str!("../config/chains.toml");

/// Top-level Solana chain configuration registry.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct SolanaChainRegistry {
    #[serde(default)]
    pub chains: HashMap<String, SolanaChainConfig>,
}

/// Configuration for a single Solana-compatible chain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolanaChainConfig {
    pub rpc_url: String,
    pub hyperlane_domain: u32,
    #[serde(default)]
    pub ws_url: Option<String>,
    #[serde(default)]
    pub explorer: Option<String>,
    #[serde(default)]
    pub warp_route_program: Option<String>,
    #[serde(default)]
    pub x402_settlement_program: Option<String>,
    #[serde(default)]
    pub hyperlane_mailbox_program: Option<String>,
    #[serde(default)]
    pub tokens: HashMap<String, SolanaTokenConfig>,
}

/// Configuration for a single SPL token on a Solana chain.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SolanaTokenConfig {
    pub mint: String,
    pub decimals: u8,
    #[serde(default)]
    pub morpheum_asset_index: u64,
}

/// Resolved Solana chain configuration with parsed addresses.
#[derive(Clone, Debug)]
pub struct ResolvedSolanaChain {
    pub name: String,
    pub rpc_url: String,
    pub hyperlane_domain: u32,
    pub explorer: Option<String>,
    pub warp_route_program: Option<Pubkey>,
    pub x402_settlement_program: Option<Pubkey>,
    pub hyperlane_mailbox_program: Option<Pubkey>,
}

/// Resolved SPL token configuration with parsed addresses.
#[derive(Clone, Debug)]
pub struct ResolvedSolanaToken {
    pub symbol: String,
    pub mint: Pubkey,
    pub decimals: u8,
    pub morpheum_asset_index: u64,
}

impl SolanaChainRegistry {
    /// Loads from a TOML file.
    pub fn from_file(path: &Path) -> Result<Self, SvmError> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| SvmError::Config(format!("failed to read {}: {e}", path.display())))?;
        Self::from_toml(&content)
    }

    /// Parses from a TOML string.
    pub fn from_toml(content: &str) -> Result<Self, SvmError> {
        toml::from_str(content).map_err(|e| SvmError::Config(format!("TOML parse error: {e}")))
    }

    /// Loads the default config, then merges user overrides from
    /// `~/.config/morpheum/solana-chains.toml` (if present).
    pub fn load_with_defaults(default_toml: &str) -> Result<Self, SvmError> {
        let mut registry = Self::from_toml(default_toml)?;

        if let Some(config_dir) = dirs_next::config_dir() {
            let user_path = config_dir.join("morpheum").join("solana-chains.toml");
            if user_path.exists() {
                let user = Self::from_file(&user_path)?;
                registry.merge(user);
            }
        }

        Ok(registry)
    }

    /// Merges another registry into this one (other takes precedence).
    pub fn merge(&mut self, other: Self) {
        for (name, other_chain) in other.chains {
            match self.chains.get_mut(&name) {
                Some(existing) => {
                    existing.rpc_url = other_chain.rpc_url;
                    existing.hyperlane_domain = other_chain.hyperlane_domain;
                    if other_chain.ws_url.is_some() {
                        existing.ws_url = other_chain.ws_url;
                    }
                    if other_chain.explorer.is_some() {
                        existing.explorer = other_chain.explorer;
                    }
                    if other_chain.warp_route_program.is_some() {
                        existing.warp_route_program = other_chain.warp_route_program;
                    }
                    if other_chain.x402_settlement_program.is_some() {
                        existing.x402_settlement_program = other_chain.x402_settlement_program;
                    }
                    if other_chain.hyperlane_mailbox_program.is_some() {
                        existing.hyperlane_mailbox_program = other_chain.hyperlane_mailbox_program;
                    }
                    for (token_name, token_cfg) in other_chain.tokens {
                        existing.tokens.insert(token_name, token_cfg);
                    }
                }
                None => {
                    self.chains.insert(name, other_chain);
                }
            }
        }
    }

    /// Resolves a chain by name (case-insensitive, with aliases).
    pub fn get_chain(&self, name: &str) -> Option<&SolanaChainConfig> {
        let lower = name.to_ascii_lowercase();
        self.chains.get(&lower).or_else(|| {
            let alias = match lower.as_str() {
                "sol" => "solana",
                "local" | "localnet" => "localnet",
                "dev" => "devnet",
                "test" => "testnet",
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
    ) -> Result<(ResolvedSolanaChain, ResolvedSolanaToken), SvmError> {
        let chain = self.get_chain(chain_name).ok_or_else(|| {
            SvmError::Config(format!("unknown Solana chain: '{chain_name}'"))
        })?;

        let upper = token_symbol.to_ascii_uppercase();
        let token = chain.tokens.get(&upper).ok_or_else(|| {
            SvmError::Config(format!(
                "token '{token_symbol}' not configured for Solana chain '{chain_name}'"
            ))
        })?;

        let mint = parse_pubkey(&token.mint)?;
        let warp_route = chain
            .warp_route_program
            .as_deref()
            .map(parse_pubkey)
            .transpose()?;
        let x402 = chain
            .x402_settlement_program
            .as_deref()
            .map(parse_pubkey)
            .transpose()?;
        let mailbox = chain
            .hyperlane_mailbox_program
            .as_deref()
            .map(parse_pubkey)
            .transpose()?;

        Ok((
            ResolvedSolanaChain {
                name: chain_name.to_string(),
                rpc_url: chain.rpc_url.clone(),
                hyperlane_domain: chain.hyperlane_domain,
                explorer: chain.explorer.clone(),
                warp_route_program: warp_route,
                x402_settlement_program: x402,
                hyperlane_mailbox_program: mailbox,
            },
            ResolvedSolanaToken {
                symbol: upper,
                mint,
                decimals: token.decimals,
                morpheum_asset_index: token.morpheum_asset_index,
            },
        ))
    }

    /// Lists all configured chain names.
    pub fn chain_names(&self) -> Vec<&str> {
        self.chains.keys().map(String::as_str).collect()
    }
}

fn parse_pubkey(s: &str) -> Result<Pubkey, SvmError> {
    s.parse()
        .map_err(|e| SvmError::AddressParse(format!("'{s}': {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TOML: &str = r#"
[chains.localnet]
rpc_url = "http://127.0.0.1:8899"
hyperlane_domain = 1399811150
warp_route_program = "HypWarpCoLLRt111111111111111111111111111111"
x402_settlement_program = "95mAeRSnfH8KtP72sNJ5Ks8zSwf91WbhXn7E6HLjKSKZ"

[chains.localnet.tokens.USDC]
mint = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"
decimals = 6
morpheum_asset_index = 1
"#;

    #[test]
    fn parse_and_resolve() {
        let registry = SolanaChainRegistry::from_toml(TEST_TOML).unwrap();
        let (chain, token) = registry.resolve("localnet", "USDC").unwrap();

        assert_eq!(chain.hyperlane_domain, 1399811150);
        assert_eq!(token.decimals, 6);
        assert!(chain.warp_route_program.is_some());
        assert!(chain.x402_settlement_program.is_some());
        assert_eq!(token.morpheum_asset_index, 1);
    }

    #[test]
    fn alias_resolution() {
        let registry = SolanaChainRegistry::from_toml(TEST_TOML).unwrap();
        assert!(registry.get_chain("local").is_some());
    }

    #[test]
    fn unknown_chain_errors() {
        let registry = SolanaChainRegistry::from_toml(TEST_TOML).unwrap();
        assert!(registry.resolve("nonexistent", "USDC").is_err());
    }

    #[test]
    fn unknown_token_errors() {
        let registry = SolanaChainRegistry::from_toml(TEST_TOML).unwrap();
        assert!(registry.resolve("localnet", "WBTC").is_err());
    }
}
