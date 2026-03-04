//! Interop module for the Morpheum SDK.
//!
//! This module provides full support for the Morpheum cross-chain
//! interoperability system, including:
//! - Submitting bridge requests (proof or intent payloads)
//! - Exporting intents for cross-chain execution
//! - Exporting proofs (reputation, validation, identity) for cross-chain
//!   verification
//! - Querying bridge request, intent export, and proof export statuses
//! - Governance-controlled parameter updates
//!
//! The `InteropClient` is the main entry point for all interop queries.
//! Transaction construction is handled via the fluent builders.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// Public API modules — each has a single, clear responsibility
pub mod builder;
pub mod client;
pub mod types;
pub mod requests;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all interop queries (bridge, intent export, proof export, params).
pub use client::InteropClient;

/// Fluent builders for interop transaction construction.
pub use builder::{
    BridgeRequestBuilder,
    ExportIntentBuilder,
    ExportProofBuilder,
    UpdateParamsBuilder,
};

/// Core domain types for the interop module.
pub use types::{
    BridgePayload,
    BridgeRequestData,
    BridgeResponse,
    CrossChainProof,
    CrossChainProofPacket,
    IdentityProofPacket,
    IntentExportPacket,
    Params,
    ReputationProofPacket,
    ValidationProofPacket,
};

/// Re-export all request and response types for clean message construction.
pub use requests::*;

/// Re-export core SDK types commonly used with interop flows.
pub use morpheum_sdk_core::{
    SdkError,
    SdkConfig,
};

/// Recommended prelude for the interop module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_interop::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        InteropClient,
        BridgeRequestBuilder,
        ExportIntentBuilder,
        ExportProofBuilder,
        UpdateParamsBuilder,
        BridgePayload,
        BridgeRequestData,
        BridgeResponse,
        CrossChainProof,
        CrossChainProofPacket,
        IdentityProofPacket,
        IntentExportPacket,
        Params,
        ReputationProofPacket,
        ValidationProofPacket,
        SdkError,
        SdkConfig,
    };
}

/// Current version of the interop module (synchronized with workspace version).
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
