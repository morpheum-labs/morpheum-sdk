//! VC module for the Morpheum SDK.
//!
//! This module provides full support for issuing, querying, revoking, updating,
//! and managing **Verifiable Credentials (VCs)** and **Verifiable Presentations (VPs)**.
//!
//! It is deeply integrated with `TradingKeyClaim` and `VcClaimBuilder` for secure,
//! delegated agent workflows (e.g. autonomous trading agents with isolated nonce
//! sub-ranges and permission limits).
//!
//! Key capabilities:
//! - Issue VCs with rich claims (max_daily_usd, allowed_pairs, slippage, etc.)
//! - Revoke VCs (issuer-initiated or self-revoke by subject)
//! - Query VC status, revocation bitmap, and issuer/subject lists
//! - Full support for agent delegation via `TradingKeyClaim`

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// Public API modules — each has a single, clear responsibility
pub mod builder;
pub mod client;
pub mod types;
pub mod requests;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all VC operations (issue, revoke, query, status, etc.).
pub use client::VcClient;

/// Fluent builders for VC issuance, revocation, self-revocation, and claims updates.
pub use builder::{
    VcIssueBuilder,
    VcRevokeBuilder,
    VcSelfRevokeBuilder,
    UpdateClaimsBuilder,
    UpdateModuleParamsBuilder,
};

/// Core domain types for Verifiable Credentials.
pub use types::{
    Vc,
    VcClaims,
    Vp,
    VcStatus,
    Params,
    ActiveVc,
};

/// Request and response wrappers for transaction construction and queries.
pub use requests::*;

// Re-export core SDK types commonly used with VC flows.
pub use morpheum_sdk_core::{
    AccountId,
    ChainId,
    SdkError,
    SignedTx,
};

// Re-export key signing types needed for VC issuance and agent delegation.
// Note: AgentSigner/NativeSigner are in morpheum-signing-native (not core).
pub use morpheum_sdk_core::signing::claim::{TradingKeyClaim, VcClaimBuilder};
pub use morpheum_sdk_core::signing::signer::Signer;

/// Recommended prelude for the VC module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_vc::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        VcClient,
        VcIssueBuilder,
        VcRevokeBuilder,
        VcSelfRevokeBuilder,
        UpdateClaimsBuilder,
        UpdateModuleParamsBuilder,
        Vc,
        VcClaims,
        Vp,
        VcStatus,
        ActiveVc,
        Params,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
        Signer,
        TradingKeyClaim,
        VcClaimBuilder,
    };
}

/// Current version of the VC module (synchronized with workspace version).
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