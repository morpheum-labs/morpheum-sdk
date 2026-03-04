//! Directory module for the Morpheum SDK.
//!
//! This module provides full support for the Morpheum agent directory system,
//! including:
//! - Updating agent directory profiles (display name, description, tags)
//! - Updating profile visibility (public, owner-only, evaluator-only)
//! - Querying profiles by agent hash or with filtered/paginated listings
//! - Governance-controlled parameter updates
//!
//! The `DirectoryClient` is the main entry point for all directory queries.
//! Transaction construction is handled via the fluent builders.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// Public API modules — each has a single, clear responsibility
pub mod builder;
pub mod client;
pub mod types;
pub mod requests;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all directory queries (profile, profiles, params).
pub use client::DirectoryClient;

/// Fluent builders for directory transaction construction.
pub use builder::{
    UpdateParamsBuilder,
    UpdateProfileBuilder,
    UpdateVisibilityBuilder,
};

/// Core domain types for the directory module.
pub use types::{
    AgentDirectoryProfile,
    DirectoryFilter,
    Params,
    VisibilityLevel,
};

/// Re-export all request and response types for clean message construction.
pub use requests::*;

/// Re-export core SDK types commonly used with directory flows.
pub use morpheum_sdk_core::{
    SdkError,
    SdkConfig,
};

/// Recommended prelude for the directory module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_directory::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        DirectoryClient,
        UpdateProfileBuilder,
        UpdateVisibilityBuilder,
        UpdateParamsBuilder,
        AgentDirectoryProfile,
        DirectoryFilter,
        Params,
        VisibilityLevel,
        SdkError,
        SdkConfig,
    };
}

/// Current version of the directory module (synchronized with workspace version).
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
