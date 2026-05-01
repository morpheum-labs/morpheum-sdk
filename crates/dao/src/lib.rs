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

pub mod builder;
pub mod client;
pub mod requests;
pub mod types;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::DaoClient;

pub use types::{
    Dao, DaoConfig, DaoDeposit, DaoPlugin, DaoProposal, DaoProposalStatus, DaoProposalUpdate,
    DaoStatus, DaoTallyResult, DaoType, DaoVote, DaoVoteOption, GovernedAsset,
    WeightedDaoVoteOption,
};

pub use requests::*;

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the DAO module.
///
/// ```rust,ignore
/// use morpheum_sdk_dao::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        AccountId, ChainId, Dao, DaoClient, DaoConfig, DaoProposal, DaoProposalStatus, DaoStatus,
        DaoTallyResult, DaoType, DaoVote, DaoVoteOption, SdkError, SignedTx, WeightedDaoVoteOption,
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
