//! Mark price module for the Morpheum SDK.
//!
//! Provides support for querying canonical mark prices (TWAP + Oracle + Kline
//! composite), source attribution, and governance configuration of per-market
//! mark price weights.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::MarkPriceClient;

pub use types::{
    MarkConfig,
    MarkPriceData,
    MarkPriceModuleConfig,
    MarkPriceUpdated,
    MarkPriceWithSource,
    MarkSource,
    PriceMoveAlert,
};

pub use requests::{
    GetMarkPriceRequest,
    GetMarkPriceWithSourceRequest,
    UpdateMarkConfigRequest,
};

pub use builder::UpdateMarkConfigBuilder;

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the mark price module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_markprice::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        MarkPriceClient,
        MarkPriceData,
        MarkPriceWithSource,
        MarkSource,
        MarkConfig,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the mark price module (synchronized with workspace version).
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
