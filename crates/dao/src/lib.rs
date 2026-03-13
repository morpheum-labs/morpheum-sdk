//! DAO module for the Morpheum SDK.
//!
//! This module provides full support for the permissionless DAO framework on
//! Morpheum, enabling creation and management of unlimited independent DAOs
//! (Realms). Each DAO has its own treasury, proposals, voting, deposits, and
//! governed assets.
//!
//! Supports community-token voting, council multisig, hybrid governance,
//! conviction voting, weighted split votes, and a plugin system for custom
//! voter weight and decision policies.
//!
//! Distinct from the singleton sovereign `gov` module — DAOs never affect
//! protocol consensus, staking, or core modules.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::DaoClient;

pub use types::{
    Dao,
    DaoConfig,
    DaoDeposit,
    DaoPlugin,
    DaoProposal,
    DaoProposalStatus,
    DaoProposalUpdate,
    DaoStatus,
    DaoTallyResult,
    DaoType,
    DaoVote,
    DaoVoteOption,
    GovernedAsset,
    WeightedDaoVoteOption,
};

pub use requests::*;

pub use morpheum_sdk_core::{
    AccountId,
    ChainId,
    SdkError,
    SignedTx,
};

/// Recommended prelude for the DAO module.
///
/// ```rust,ignore
/// use morpheum_sdk_dao::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        DaoClient,
        Dao,
        DaoConfig,
        DaoProposal,
        DaoProposalStatus,
        DaoStatus,
        DaoTallyResult,
        DaoType,
        DaoVote,
        DaoVoteOption,
        WeightedDaoVoteOption,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the DAO module (synchronized with workspace version).
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
