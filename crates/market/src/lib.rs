//! Market module for the Morpheum SDK.
//!
//! This module provides full support for managing trading markets on Morpheum,
//! including creation, activation, suspension, parameter updates, margin ratio changes,
//! and rich querying of market data and statistics.
//!
//! It supports multiple market types (spot, perpetuals, futures, options) and
//! integrates seamlessly with the CLOB and asset registry.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

// Public API modules — each has a single, clear responsibility
pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all market operations (create, activate, suspend, update,
/// query markets and statistics).
pub use client::MarketClient;

/// Core domain types for markets.
pub use types::{
    Market,
    MarketParams,
    MarketStats,
    MarketType,
    MarketStatus,
    MarketCategory,
    MarketTypeConfig,
    MarketTypeStats,
    ClobMarketConfig,
    ClobStats,
    PredictionMarketConfig,
    PredictionStats,
    PerpConfig,
    MarketUpdate,
};

/// Request and response wrappers for transaction construction and queries.
pub use requests::*;

// Re-export core SDK types commonly used with market flows.
pub use morpheum_sdk_core::{
    AccountId,
    ChainId,
    SdkError,
    SignedTx,
};

/// Recommended prelude for the market module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_market::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        MarketClient,
        Market,
        MarketParams,
        MarketStats,
        MarketType,
        MarketStatus,
        MarketCategory,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the market module (synchronized with workspace version).
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