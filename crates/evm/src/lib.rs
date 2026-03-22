//! Morpheum SDK — EVM contract interaction.
//!
//! Provides shared alloy-based bindings and high-level helpers for all EVM
//! contracts used in Morpheum cross-chain operations:
//!
//! - **`contracts`** — `sol!` ABI bindings (IERC20, HypERC20Collateral, Mailbox, X402Settlement)
//! - **`provider`** — `EvmProvider` factory from RPC URL + signer
//! - **`bridge`** — Warp Route helpers (`approve_erc20`, `transfer_remote`, `parse_dispatch_id`)
//! - **`x402`** — x402 settlement helpers (`pay_x402`, `quote_fee`)
//! - **`config`** — Chain configuration registry (`ChainRegistry`, `chains.toml` loader)
//! - **`types`** — Domain types (`DispatchResult`, `PaymentResult`, `EvmError`)

#![forbid(unsafe_code)]

pub mod bridge;
pub mod cctp;
pub mod config;
pub mod contracts;
pub mod provider;
pub mod types;
pub mod x402;

pub use bridge::{approve_erc20, parse_dispatch_id, transfer_remote};
pub use cctp::{bridge_usdc, quote_cctp_dispatch, CctpBridgeResult, ICctpHyperlaneWrapper};
pub use config::{ChainConfig, ChainRegistry, TokenConfig, DEFAULT_CHAINS_TOML};
pub use contracts::{IERC20, IHypERC20Collateral, IMailbox, IX402Settlement};
pub use provider::{build_provider, EvmProvider};
pub use types::{DispatchResult, EvmError, PaymentInfo, PaymentResult, X402PayParams};
pub use x402::{get_payment, pay_x402, quote_fee};

pub use alloy;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
