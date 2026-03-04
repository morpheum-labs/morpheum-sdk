//! Intent module for the Morpheum SDK.
//!
//! This module provides full support for the Morpheum intent-based execution
//! system, including:
//! - Submitting agent intents (conditional, TWAP, multi-leg, declarative)
//! - Cancelling active intents
//! - Querying intents by ID, by agent, and active intents
//! - Governance-controlled parameter updates
//!
//! The `IntentClient` is the main entry point for all intent queries.
//! Transaction construction is handled via the fluent builders.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// Public API modules — each has a single, clear responsibility
pub mod builder;
pub mod client;
pub mod types;
pub mod requests;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all intent queries (by ID, by agent, active, params).
pub use client::IntentClient;

/// Fluent builders for intent transaction construction.
pub use builder::{
    CancelIntentBuilder,
    SubmitIntentBuilder,
    UpdateParamsBuilder,
};

/// Core domain types for the intent module.
pub use types::{
    AgentIntent,
    ConditionalParams,
    DeclarativeParams,
    DecompositionTrace,
    IntentParams,
    IntentStatus,
    IntentType,
    Leg,
    MultiLegParams,
    Params,
    TwapParams,
};

/// Re-export all request and response types for clean message construction.
pub use requests::*;

/// Re-export core SDK types commonly used with intent flows.
pub use morpheum_sdk_core::{
    SdkError,
    SdkConfig,
};

/// Recommended prelude for the intent module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_intent::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        IntentClient,
        SubmitIntentBuilder,
        CancelIntentBuilder,
        UpdateParamsBuilder,
        AgentIntent,
        ConditionalParams,
        DeclarativeParams,
        DecompositionTrace,
        IntentParams,
        IntentStatus,
        IntentType,
        Leg,
        MultiLegParams,
        Params,
        TwapParams,
        SdkError,
        SdkConfig,
    };
}

/// Current version of the intent module (synchronized with workspace version).
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
