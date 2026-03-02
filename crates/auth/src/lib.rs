//! Auth module for the Morpheum SDK.
//!
//! This module provides the foundational authentication and authorization
//! primitives for the Morpheum network, including:
//! - Nonce state queries and management (the single source of truth for
//!   replay protection and parallel execution)
//! - BaseAccount and ModuleAccount handling
//! - TradingKey approval and revocation for secure agent delegation
//! - Governance-controlled parameter updates
//!
//! The `AuthClient` is the main entry point for all auth-related operations.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// Public API modules — each has a single, clear responsibility
pub mod client;
pub mod types;
pub mod requests;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all auth operations (nonce queries, TradingKey management,
/// account state, etc.).
pub use client::AuthClient;

/// Re-export key domain types from the auth module.
pub use types::{
    BaseAccount,
    ModuleAccount,
    ModuleCredential,
    NonceState,
    Params,
};

// Re-export all request and response types for clean message construction.
pub use requests::*;

// Re-export commonly used core SDK types for ergonomic use with auth flows.
pub use morpheum_sdk_core::{
    AccountId,
    ChainId,
    SdkError,
    SignedTx,
};

// Re-export key signing types needed for TradingKey delegation.
// Note: AgentSigner/NativeSigner are in morpheum-signing-native (not core).
pub use morpheum_sdk_core::signing::claim::{TradingKeyClaim, VcClaimBuilder};
pub use morpheum_sdk_core::signing::signer::Signer;

/// Recommended prelude for the auth module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_auth::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        AuthClient,
        BaseAccount,
        ModuleAccount,
        NonceState,
        Params,
        Signer,
        TradingKeyClaim,
        VcClaimBuilder,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the auth module (synchronized with workspace version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn public_api_compiles_cleanly() {
        // Ensures all re-exports are valid and the prelude is ergonomic
        use prelude::*;
        let _ = VERSION;
    }
}