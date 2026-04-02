//! Reputation module for the Morpheum SDK.
//!
//! This module provides full support for the Morpheum reputation system,
//! including:
//! - Querying agent reputation scores, event history, and milestone status
//! - Forcing milestones (governance only)
//! - Governance-controlled parameter updates
//!
//! The `ReputationClient` is the main entry point for all reputation queries.
//! Transaction construction is handled via the fluent builders.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// Public API modules — each has a single, clear responsibility
pub mod builder;
pub mod client;
pub mod types;
pub mod requests;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all reputation queries (scores, history, milestones, params).
pub use client::ReputationClient;

/// Fluent builders for reputation transaction construction.
pub use builder::{
    ForceMilestoneBuilder,
    UpdateParamsBuilder,
};

/// Core domain types for the reputation module.
pub use types::{
    MilestoneStatus,
    Params,
    RecoveryActionType,
    ReputationEvent,
    ReputationEventType,
    ReputationScore,
    MAX_SCORE,
};

/// Re-export all request and response types for clean message construction.
pub use requests::*;

/// Re-export core SDK types commonly used with reputation flows.
pub use morpheum_sdk_core::{
    SdkError,
    SdkConfig,
};

/// Recommended prelude for the reputation module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_reputation::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        ReputationClient,
        ForceMilestoneBuilder,
        UpdateParamsBuilder,
        MilestoneStatus,
        Params,
        RecoveryActionType,
        ReputationEvent,
        ReputationEventType,
        ReputationScore,
        SdkError,
        SdkConfig,
        MAX_SCORE,
    };
}

/// Current version of the reputation module (synchronized with workspace version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn public_api_compiles_cleanly() {
        // Ensures all re-exports are valid and the prelude is ergonomic
        #[allow(unused_imports)]
        use prelude::*;
        let _ = VERSION;
    }
}
