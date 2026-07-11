//! Vault module for the Morpheum SDK.
//!
//! Provides support for creating and managing strategy/yield vaults,
//! depositing/withdrawing, executing strategies, claiming yield,
//! querying vault health, IL metrics, and streaming events.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod builder;
pub mod client;
pub mod requests;
pub mod types;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{
    GuardianActionListPage, StakeListPage, StrategyHistoryPage, VaultClient, VaultListPage,
};

pub use types::{
    AllocationKind, AllocationPolicy, AllocationTarget, BucketMode, FeePresetBound, GuardianAction,
    GuardianActionKind, GuardianActionStatus, GuardianQuorum, IlMetrics, RevenueShareConfig,
    SpotExitPool, Stake, StrategyExecution, Vault, VaultBucket, VaultFeePreset, VaultHealth,
    VaultMandate, VaultParams, VaultRecord, VaultStatus, VaultStreamEvent, VaultType,
    VaultUpdateEvent,
};

pub use requests::{
    AcquireSpotRequest, ApproveGuardianActionRequest, CancelGuardianActionRequest,
    ClaimYieldRequest, ClearVaultLiquidationRequest, CreateProtocolVaultRequest,
    CreateVaultRequest, CrystallizeFeeRequest, DeployToBucketRequest, DepositToVaultRequest,
    DisposeSpotRequest, ExecuteStrategyRequest, GetIlMetricsRequest, GetStrategyHistoryRequest,
    GetTopVaultsRequest, GetUserStakeRequest, GetVaultHealthRequest, GetVaultRequest,
    GetVaultsByAgentRequest, GetVaultsByTypeRequest, ListGuardianActionsRequest,
    ListUserStakesRequest, ListVaultsRequest, PauseVaultRequest, ProposeGuardianActionRequest,
    RefreshVaultScoreRequest, RestoreVaultOperatorRequest, ResumeVaultRequest,
    SetVaultLeverageRequest, SweepDeadVaultRequest, UndeployFromBucketRequest, UpdateParamsRequest,
    UpdateVaultParamsRequest, WithdrawFromVaultRequest,
};

pub use builder::{
    ClaimYieldBuilder, CreateVaultBuilder, DeployToBucketBuilder, DepositToVaultBuilder,
    ExecuteStrategyBuilder, PauseVaultBuilder, ResumeVaultBuilder, SetVaultLeverageBuilder,
    UndeployFromBucketBuilder, UpdateModuleParamsBuilder, UpdateVaultParamsBuilder,
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
        AccountId, ChainId, SdkError, SignedTx, Stake, Vault, VaultClient, VaultHealth,
        VaultStatus, VaultType,
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
