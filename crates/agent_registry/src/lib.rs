//! Agent Registry module for the Morpheum SDK.
//!
//! The agent_registry is a thin, event-driven meta-registry that maintains
//! a canonical superset `AgentRecord` by aggregating live views from identity,
//! reputation, validation, memory, and inference registries. It handles
//! CAIP-10/CAIP-2 ID generation and powers the central sync engine for
//! external protocols (ERC-8004, MCP/A2A, DID, x402, GMP).
//!
//! Direct transactions are limited to:
//! - `TriggerProtocolSync` — manual protocol sync (governance / emergency / testing)
//! - `UpdateParams` — governance parameter updates
//!
//! All other state mutations arrive via cross-module domain events.
//!
//! # Quick Start
//! ```rust,ignore
//! use morpheum_sdk_agent_registry::prelude::*;
//!
//! // Build a trigger-protocol-sync request
//! let request = TriggerProtocolSyncBuilder::new()
//!     .authority("morpheum1gov")
//!     .agent_hash(hash_bytes.to_vec())
//!     .protocol("erc8004")
//!     .protocol("a2a")
//!     .build()?;
//!
//! // Wrap it for TxBuilder
//! let any = request.to_any();
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod builder;
pub mod client;
pub mod types;
pub mod requests;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::AgentRegistryClient;

pub use builder::{TriggerProtocolSyncBuilder, UpdateParamsBuilder};

pub use types::{
    AgentRecord,
    CaipAgentId,
    ExportStatus,
    Params,
    VisibilityLevel,
};

pub use requests::*;

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// All supported protocols for the central sync engine.
pub const ALL_PROTOCOLS: &[&str] = &["erc8004", "a2a", "mcp", "did", "x402", "gmp"];

/// Recommended prelude for the Agent Registry module.
pub mod prelude {
    pub use super::{
        AgentRegistryClient,
        TriggerProtocolSyncBuilder,
        UpdateParamsBuilder,
        AgentRecord,
        CaipAgentId,
        ExportStatus,
        Params,
        VisibilityLevel,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
        ALL_PROTOCOLS,
    };
}

/// Current version of the Agent Registry module (synchronized with workspace version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn version_is_set() {
        assert!(!super::VERSION.is_empty());
    }

    #[test]
    fn all_protocols_contains_known() {
        assert!(super::ALL_PROTOCOLS.contains(&"erc8004"));
        assert!(super::ALL_PROTOCOLS.contains(&"a2a"));
        assert!(super::ALL_PROTOCOLS.contains(&"mcp"));
        assert!(super::ALL_PROTOCOLS.contains(&"did"));
        assert!(super::ALL_PROTOCOLS.contains(&"x402"));
        assert!(super::ALL_PROTOCOLS.contains(&"gmp"));
    }
}
