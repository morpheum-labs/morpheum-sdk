//! Governance module for the Morpheum SDK.
//!
//! This module provides full support for the native singleton sovereign governance
//! system on Morpheum, including proposal submission, voting (weighted split +
//! conviction), deposits, zero-downtime upgrade scheduling, and rich querying
//! of governance state.
//!
//! Supports all proposal classes (Standard, Expedited, Emergency, Root, Market,
//! Treasury, EmergencyMarket) and integrates seamlessly with the staking and
//! upgrade modules.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod builder;
pub mod client;
pub mod requests;
pub mod types;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::GovClient;

pub use types::{
    Deposit, GovParams, Proposal, ProposalClass, ProposalClassParams, ProposalStatus,
    ProposalUpdate, TallyResult, UpgradePlan, UpgradeStatus, Vote, VoteOption, WeightedVoteOption,
};

pub use requests::*;

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the governance module.
///
/// ```rust,ignore
/// use morpheum_sdk_gov::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        AccountId, ChainId, GovClient, GovParams, Proposal, ProposalClass, ProposalStatus,
        SdkError, SignedTx, TallyResult, UpgradePlan, UpgradeStatus, Vote, VoteOption,
        WeightedVoteOption,
    };
}

/// Current version of the governance module (synchronized with workspace version).
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
