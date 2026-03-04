//! Memory module for the Morpheum SDK.
//!
//! This module provides full support for the Morpheum agent memory system,
//! including:
//! - Storing, updating, and deleting memory entries (episodic, semantic,
//!   procedural, vector, custom)
//! - Querying entries by key or agent (with pagination)
//! - Memory root (Merkle anchor) queries
//! - Vector embedding metadata
//! - Governance-controlled parameter updates
//!
//! The `MemoryClient` is the main entry point for all memory queries.
//! Transaction construction is handled via the fluent builders.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// Public API modules — each has a single, clear responsibility
pub mod builder;
pub mod client;
pub mod types;
pub mod requests;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all memory queries (entry, entries by agent, root, params).
pub use client::MemoryClient;

/// Fluent builders for memory transaction construction.
pub use builder::{
    DeleteEntryBuilder,
    StoreEntryBuilder,
    UpdateEntryBuilder,
    UpdateParamsBuilder,
};

/// Core domain types for the memory module.
pub use types::{
    MemoryEntry,
    MemoryEntryType,
    MemoryRoot,
    MemorySnapshot,
    Params,
    VectorEmbedding,
};

/// Re-export all request and response types for clean message construction.
pub use requests::*;

/// Re-export core SDK types commonly used with memory flows.
pub use morpheum_sdk_core::{
    SdkError,
    SdkConfig,
};

/// Recommended prelude for the memory module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_memory::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        MemoryClient,
        StoreEntryBuilder,
        UpdateEntryBuilder,
        DeleteEntryBuilder,
        UpdateParamsBuilder,
        MemoryEntry,
        MemoryEntryType,
        MemoryRoot,
        MemorySnapshot,
        Params,
        VectorEmbedding,
        SdkError,
        SdkConfig,
    };
}

/// Current version of the memory module (synchronized with workspace version).
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
