//! Morpheum SDK — SVM (Solana) contract interaction.
//!
//! Provides Solana-native bindings and high-level helpers for all SVM
//! programs used in Morpheum cross-chain operations:
//!
//! - **`contracts`** — Program IDs, PDA derivation, and instruction builders
//! - **`provider`** — `SvmProvider` factory from RPC URL + `Keypair`
//! - **`bridge`** — Warp Route helpers (`create_ata_if_needed`, `transfer_remote`, `balance_of`)
//! - **`x402`** — x402 settlement helpers (`pay_x402`, `get_payment`)
//! - **`config`** — Solana chain registry (`SolanaChainRegistry`, `chains.toml` loader)
//! - **`types`** — Domain types (`DispatchResult`, `PaymentResult`, `SvmError`)

#![forbid(unsafe_code)]

pub mod bridge;
pub mod config;
pub mod contracts;
pub mod provider;
pub mod types;
pub mod x402;

pub use bridge::{balance_of, create_ata_if_needed, transfer_remote};
pub use config::{SolanaChainConfig, SolanaChainRegistry, SolanaTokenConfig, DEFAULT_CHAINS_TOML};
pub use contracts::{
    HYPERLANE_WARP_ROUTE_PROGRAM_ID, X402_SETTLEMENT_PROGRAM_ID,
};
pub use provider::{build_provider, SvmProvider};
pub use types::{DispatchResult, PaymentResult, SvmError};
pub use x402::{get_payment, pay_x402};

pub use solana_sdk;
pub use solana_client;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
