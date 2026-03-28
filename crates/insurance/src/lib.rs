//! Insurance vault module for the Morpheum SDK.
//!
//! Provides support for querying vault balances, managing LP stakes,
//! absorbing bad debt, claiming yields and bounties, hedging impermanent
//! loss, and monitoring vault threshold status.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::InsuranceClient;

pub use types::{
    BadDebtRecord,
    ChainType,
    IlMetrics,
    LpStake,
    PageInfo,
    ThresholdStatus,
    VaultBalance,
};

pub use requests::{
    AbsorbDeficitRequest,
    ClaimBountyRequest,
    ClaimYieldRequest,
    GetBadDebtHistoryRequest,
    GetIlMetricsRequest,
    GetLpStakeRequest,
    GetThresholdStatusRequest,
    GetVaultBalanceRequest,
    HedgeIlRequest,
    ListLpStakesRequest,
    ReplenishVaultRequest,
    StakeToVaultRequest,
    WithdrawStakeRequest,
};

pub use builder::{
    AbsorbDeficitBuilder,
    ClaimBountyBuilder,
    ClaimYieldBuilder,
    HedgeIlBuilder,
    ReplenishVaultBuilder,
    StakeToVaultBuilder,
    WithdrawStakeBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the insurance module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_insurance::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        InsuranceClient,
        VaultBalance,
        LpStake,
        BadDebtRecord,
        IlMetrics,
        ThresholdStatus,
        ChainType,
        PageInfo,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the insurance module (synchronized with workspace version).
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
