//! Vault module for the Morpheum SDK.
//!
//! Provides support for creating and managing strategy/yield vaults,
//! depositing/withdrawing, executing strategies, claiming yield,
//! querying vault health, IL metrics, and streaming events.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{StakeListPage, StrategyHistoryPage, VaultClient, VaultListPage};

pub use types::{
    IlMetrics,
    RevenueShareConfig,
    Stake,
    StrategyExecution,
    Vault,
    VaultHealth,
    VaultParams,
    VaultRecord,
    VaultStatus,
    VaultStreamEvent,
    VaultType,
    VaultUpdateEvent,
};

pub use requests::{
    ClaimYieldRequest,
    CreateVaultRequest,
    DepositToVaultRequest,
    ExecuteStrategyRequest,
    GetIlMetricsRequest,
    GetStrategyHistoryRequest,
    GetTopVaultsRequest,
    GetUserStakeRequest,
    GetVaultHealthRequest,
    GetVaultRequest,
    GetVaultsByAgentRequest,
    GetVaultsByTypeRequest,
    ListUserStakesRequest,
    ListVaultsRequest,
    PauseVaultRequest,
    ResumeVaultRequest,
    UpdateParamsRequest,
    UpdateVaultParamsRequest,
    WithdrawFromVaultRequest,
};

pub use builder::{
    ClaimYieldBuilder,
    CreateVaultBuilder,
    DepositToVaultBuilder,
    ExecuteStrategyBuilder,
    PauseVaultBuilder,
    ResumeVaultBuilder,
    UpdateModuleParamsBuilder,
    UpdateVaultParamsBuilder,
    WithdrawFromVaultBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the vault module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_vault::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        VaultClient,
        Vault,
        VaultType,
        VaultStatus,
        VaultHealth,
        Stake,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the vault module (synchronized with workspace version).
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
