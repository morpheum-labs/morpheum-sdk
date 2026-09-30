//! Staking module for the Morpheum SDK.
//!
//! This module provides full support for the staking lifecycle on Morpheum,
//! including validator staking/unstaking, delegation/undelegation/redelegation,
//! reward claiming, and comprehensive staking queries (validators, delegations,
//! rewards, penalties, slashing history, epochs, and parameters).
//!
//! Slashing has no transaction form on the chain: misbehavior reports, slashing
//! votes, and slashing application are not client-submitted messages, so this
//! crate carries no builders for them. Slashing outcomes are read through
//! `StakingClient::query_penalties` and `StakingClient::query_slashing_history`.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod builder;
pub mod client;
pub mod requests;
pub mod types;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::StakingClient;

pub use types::{
    CommissionInfo, Delegation, EpochRewardSnapshot, LivenessParams, MisbehaviorType, Penalty,
    Reward, ScoringParams, SlashingEvent, StakingParams, UnbondingDelegation, UserStaking,
    Validator, ValidatorScore, ValidatorStake, ValidatorStatus,
};

pub use requests::*;

pub use builder::{
    ClaimRewardsBuilder, DelegateBuilder, RedelegateBuilder, StakeBuilder, UndelegateBuilder,
    UnstakeBuilder, UpdateParamsBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the staking module.
pub mod prelude {
    pub use super::{
        AccountId, ChainId, ClaimRewardsBuilder, CommissionInfo, DelegateBuilder, Delegation,
        EpochRewardSnapshot, LivenessParams, MisbehaviorType, Penalty, RedelegateBuilder, Reward,
        ScoringParams, SdkError, SignedTx, SlashingEvent, StakeBuilder, StakingClient,
        StakingParams, UnbondingDelegation, UndelegateBuilder, UnstakeBuilder, UserStaking,
        Validator, ValidatorScore, ValidatorStake, ValidatorStatus,
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
