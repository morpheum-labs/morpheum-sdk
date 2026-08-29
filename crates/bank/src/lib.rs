//! Bank module for the Morpheum SDK.
//!
//! This module provides full support for the bank lifecycle on Morpheum,
//! including native transfers, cross-chain transfers, minting, deposits,
//! withdrawals, asset onboarding, VM bridging, and balance queries.
//!
//! It integrates seamlessly with the asset registry, perpetuals buckets,
//! and multi-chain address derivation for end-to-end financial workflows.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod builder;
pub mod client;
pub mod requests;
pub mod types;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all bank query operations.
pub use client::{BalanceResponse, BankClient};

/// Core domain types for bank operations.
pub use types::{Asset, AssetIdentifier, AssetsResponse, Balance, ChainType, SpendingPolicy};

/// Well-known asset name → registry index resolver.
pub use types::resolve_asset_index;

/// Request and response wrappers for transaction construction and queries.
pub use requests::*;

/// Fluent builders for bank transaction operations.
pub use builder::{
    BridgeAssetBuilder, ClaimSettlementBuilder, CrossChainTransferBuilder, DepositBuilder,
    MintBuilder, OnboardAssetBuilder, SetSpendingPolicyBuilder, TransferBuilder,
    TransferToBucketBuilder, WithdrawBuilder,
};

// Re-export core SDK types commonly used with bank flows.
pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the bank module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_bank::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        resolve_asset_index, AccountId, Asset, AssetIdentifier, AssetsResponse, Balance,
        BankClient, ChainId, ChainType, ClaimSettlementBuilder, CrossChainTransferBuilder,
        DepositBuilder, MintBuilder, SdkError, SetSpendingPolicyBuilder, SignedTx, SpendingPolicy,
        TransferBuilder, WithdrawBuilder,
    };
}

/// Current version of the bank module (synchronized with workspace version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn public_api_compiles_cleanly() {
        #[allow(unused_imports)]
        use prelude::*;
        let _ = VERSION;
    }
}
