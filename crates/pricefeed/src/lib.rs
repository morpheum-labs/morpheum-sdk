//! Price feed module for the Morpheum SDK.
//!
//! Provides support for registering and deregistering price feeds,
//! querying feed configurations, latest prices, and listing feeds
//! from the pricefeed registry.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod builder;
pub mod client;
pub mod requests;
pub mod types;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{FeedsPage, PriceFeedClient};

pub use types::{
    AggregationMethod, FeedKind, PriceEntry, PriceFeed, PriceFeedConfig, PriceSource, SourceConfig,
};

pub use requests::{
    DeregisterFeedRequest, QueryFeedBySymbolRequest, QueryFeedRequest, QueryFeedsRequest,
    QueryPriceRequest, RegisterFeedRequest,
};

pub use builder::{DeregisterFeedBuilder, RegisterFeedBuilder};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the price feed module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_pricefeed::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        AccountId, AggregationMethod, ChainId, FeedKind, PriceEntry, PriceFeed, PriceFeedClient,
        SdkError, SignedTx,
    };
}

/// Current version of the price feed module (synchronized with workspace version).
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
