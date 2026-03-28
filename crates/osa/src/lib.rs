//! Outcome Settlement Account (OSA) module for the Morpheum SDK.
//!
//! Provides support for creating per-outcome settlement accounts,
//! buying/selling shares, merging positions, settling outcomes after
//! oracle resolution, and claiming payouts for prediction markets.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::OsaClient;

pub use types::{
    AccountStatus,
    Balance,
    OutcomeSettlementAccount,
    PayoutEvent,
    SettlementEvent,
};

pub use requests::{
    BuySharesRequest,
    ClaimPayoutRequest,
    CreateAccountRequest,
    GetAccountRequest,
    GetBalanceRequest,
    MergePositionsRequest,
    SellSharesRequest,
    SettleOutcomeRequest,
};

pub use builder::{
    BuySharesBuilder,
    ClaimPayoutBuilder,
    CreateAccountBuilder,
    MergePositionsBuilder,
    SellSharesBuilder,
    SettleOutcomeBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the OSA module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_osa::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        OsaClient,
        OutcomeSettlementAccount,
        AccountStatus,
        Balance,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the OSA module (synchronized with workspace version).
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
