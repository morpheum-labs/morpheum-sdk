//! Identity module for the Morpheum SDK.
//!
//! Provides full support for registering, querying, transferring, updating,
//! and managing **AI Agent identities** on the Morpheum blockchain.
//!
//! Key capabilities:
//! - Register new agents with metadata and optional initial VC delegation
//! - Query agent identities by hash or DID
//! - Query metadata cards, statuses, and module parameters
//! - Transfer ownership between agents
//! - Update metadata and status (suspend, slash)
//! - Burn agents (governance)

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// Public API modules — each has a single, clear responsibility
pub mod builder;
pub mod client;
pub mod types;
pub mod requests;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all Identity operations (query agents, metadata, status, params).
pub use client::IdentityClient;

/// Fluent builders for identity transactions.
pub use builder::{
    RegisterAgentBuilder,
    TransferOwnershipBuilder,
    UpdateMetadataBuilder,
    UpdateStatusBuilder,
    BurnAgentBuilder,
};

/// Core domain types for Agent Identity.
pub use types::{
    AgentId,
    AgentIdentity,
    AgentMetadataCard,
    AgentMetadataCardInput,
    AgentStatus,
    Capability,
    Params,
};

/// Request and response wrappers for transaction construction and queries.
pub use requests::*;

// Re-export core SDK types commonly used with identity flows.
pub use morpheum_sdk_core::{
    AccountId,
    ChainId,
    SdkError,
    SignedTx,
};

// Re-export signing types needed for agent identity operations.
pub use morpheum_sdk_core::signing::signer::Signer;

/// Recommended prelude for the Identity module.
///
/// Most users should start with:
/// ```rust,ignore
/// use morpheum_sdk_identity::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        IdentityClient,
        RegisterAgentBuilder,
        TransferOwnershipBuilder,
        UpdateMetadataBuilder,
        UpdateStatusBuilder,
        BurnAgentBuilder,
        AgentId,
        AgentIdentity,
        AgentMetadataCard,
        AgentMetadataCardInput,
        AgentStatus,
        Capability,
        Params,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
        Signer,
    };
}

/// Current version of the Identity module (synchronized with workspace version).
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
