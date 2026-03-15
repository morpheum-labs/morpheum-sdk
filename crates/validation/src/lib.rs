//! Validation module for the Morpheum SDK.
//!
//! This module provides full support for the Morpheum validation proof system,
//! including:
//! - Submitting validation proofs (backtest, inference, TEE attestation,
//!   human eval, external validator, marketplace eval, custom)
//! - Revoking proofs (by verifier or governance)
//! - Querying proofs by ID, by agent, by type (with pagination)
//! - Cross-chain proof packets for bridge verification
//!
//! The `ValidationClient` is the main entry point for all validation queries.
//! Transaction construction is handled via the fluent builders.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// Public API modules — each has a single, clear responsibility
pub mod builder;
pub mod client;
pub mod types;
pub mod requests;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all validation queries (proof, proofs by agent/type, params).
pub use client::ValidationClient;

/// Fluent builders for validation transaction construction.
pub use builder::{
    RevokeProofBuilder,
    SubmitProofBuilder,
};

/// Core domain types for the validation module.
pub use types::{
    CrossChainProofPacket,
    Params,
    ProofType,
    ValidationProof,
};

/// Re-export all request and response types for clean message construction.
pub use requests::*;

/// Re-export core SDK types commonly used with validation flows.
pub use morpheum_sdk_core::{
    SdkError,
    SdkConfig,
};

/// Recommended prelude for the validation module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_validation::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        ValidationClient,
        SubmitProofBuilder,
        RevokeProofBuilder,
        CrossChainProofPacket,
        Params,
        ProofType,
        ValidationProof,
        SdkError,
        SdkConfig,
    };
}

/// Current version of the validation module (synchronized with workspace version).
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
