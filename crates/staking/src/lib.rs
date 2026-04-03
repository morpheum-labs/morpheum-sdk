//! Staking module for the Morpheum SDK.
//!
//! This module provides full support for the staking lifecycle on Morpheum,
//! including validator staking/unstaking, delegation/undelegation/redelegation,
//! reward claiming, misbehavior
//! reporting, slashing votes, and comprehensive staking queries.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::StakingClient;

pub use types::{
    Delegation, MisbehaviorType, Penalty, Reward, SlashingEvent,
    StakingParams, UnbondingDelegation, UserStaking, Validator,
    ValidatorStake, ValidatorStatus,
};

pub use requests::*;

pub use builder::{
    StakeBuilder, UnstakeBuilder,
    DelegateBuilder, UndelegateBuilder, RedelegateBuilder,
    ClaimRewardsBuilder,
    ReportMisbehaviorBuilder, VoteOnSlashingBuilder, ApplySlashingBuilder,
    UpdateParamsBuilder,
};

pub use morpheum_sdk_core::{
    AccountId, ChainId, SdkError, SignedTx,
};

/// Recommended prelude for the staking module.
pub mod prelude {
    pub use super::{
        StakingClient,
        Validator, ValidatorStatus, ValidatorStake,
        Delegation, UnbondingDelegation, Reward, UserStaking,
        MisbehaviorType, Penalty, SlashingEvent,
        StakingParams,
        StakeBuilder, UnstakeBuilder,
        DelegateBuilder, UndelegateBuilder, RedelegateBuilder,
        ClaimRewardsBuilder,
        AccountId, ChainId, SdkError, SignedTx,
    };
}

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
