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

pub mod builder;
pub mod client;
pub mod requests;
pub mod types;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::UpgradeClient;

pub use types::{
    Upgrade, UpgradePlan, UpgradeSignal, UpgradeStatus, UpgradeStatusSummary, UpgradeType,
    UpgradeUpdate, ValidatorReadiness, ValidatorReadinessOverview,
};

pub use requests::*;

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the upgrade module.
///
/// ```rust,ignore
/// use morpheum_sdk_upgrade::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        AccountId, ChainId, SdkError, SignedTx, Upgrade, UpgradeClient, UpgradePlan, UpgradeStatus,
        UpgradeStatusSummary, UpgradeType, ValidatorReadiness,
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
