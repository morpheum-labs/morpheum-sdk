//! TWAP module for the Morpheum SDK.
//!
//! Provides support for querying time-weighted average prices,
//! updating per-market TWAP configuration, and receiving
//! streaming TWAP update events.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{TwapClient, TwapResult};

pub use types::{
    MarketTwapConfig,
    TwapData,
    TwapEvent,
    TwapModuleConfig,
    TwapUpdated,
    WindowEntry,
    WindowSnapshot,
};

pub use requests::{GetTwapRequest, UpdateTwapConfigRequest};

pub use builder::UpdateTwapConfigBuilder;

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the TWAP module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_twap::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        TwapClient,
        TwapData,
        MarketTwapConfig,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the TWAP module (synchronized with workspace version).
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
