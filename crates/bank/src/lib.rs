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

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all bank query operations.
pub use client::{BankClient, BalanceResponse};

/// Core domain types for bank operations.
pub use types::{Asset, AssetIdentifier, AssetsResponse, Balance, ChainType};

/// Well-known asset name → registry index resolver.
pub use types::resolve_asset_index;

/// Request and response wrappers for transaction construction and queries.
pub use requests::*;

/// Fluent builders for bank transaction operations.
pub use builder::{
    TransferBuilder,
    CrossChainTransferBuilder,
    TransferToBucketBuilder,
    MintBuilder,
    OnboardAssetBuilder,
    BridgeAssetBuilder,
    DepositBuilder,
    WithdrawBuilder,
};

// Re-export core SDK types commonly used with bank flows.
pub use morpheum_sdk_core::{
    AccountId,
    ChainId,
    SdkError,
    SignedTx,
};

/// Recommended prelude for the bank module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_bank::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        BankClient,
        Asset,
        AssetIdentifier,
        AssetsResponse,
        Balance,
        ChainType,
        resolve_asset_index,
        TransferBuilder,
        CrossChainTransferBuilder,
        MintBuilder,
        DepositBuilder,
        WithdrawBuilder,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
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
