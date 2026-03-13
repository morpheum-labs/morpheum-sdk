//! Upgrade module for the Morpheum SDK.
//!
//! This module provides full support for the zero-downtime upgrade coordination
//! system on Morpheum. Validators signal shadow-mode readiness, upgrades activate
//! atomically at the next ratified staple, and AI agents can monitor the entire
//! lifecycle in real-time.
//!
//! Supports all upgrade types (Parameter, HotFeature, Binary, Emergency) and
//! integrates with the governance module for proposal-driven scheduling.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::UpgradeClient;

pub use types::{
    Upgrade,
    UpgradePlan,
    UpgradeSignal,
    UpgradeStatus,
    UpgradeStatusSummary,
    UpgradeType,
    UpgradeUpdate,
    ValidatorReadiness,
    ValidatorReadinessOverview,
};

pub use requests::*;

pub use morpheum_sdk_core::{
    AccountId,
    ChainId,
    SdkError,
    SignedTx,
};

/// Recommended prelude for the upgrade module.
///
/// ```rust,ignore
/// use morpheum_sdk_upgrade::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        UpgradeClient,
        Upgrade,
        UpgradePlan,
        UpgradeStatus,
        UpgradeStatusSummary,
        UpgradeType,
        ValidatorReadiness,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the upgrade module (synchronized with workspace version).
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
