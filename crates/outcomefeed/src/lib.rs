//! Outcome feed module for the Morpheum SDK.
//!
//! Provides support for registering prediction market feeds,
//! querying resolved outcomes, and listing feeds with filtering
//! by resolution paradigm and status.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{OutcomeFeedClient, PredictionFeedsPage};

pub use types::{
    FeedStatus,
    MarketResolutionCriteria,
    PredictionMarketFeed,
    ResolutionParadigm,
    ResolvedOutcome,
};

pub use requests::{
    QueryPredictionFeedRequest,
    QueryPredictionFeedsRequest,
    QueryResolvedOutcomeRequest,
    RegisterPredictionFeedRequest,
};

pub use builder::RegisterPredictionFeedBuilder;

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the outcome feed module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_outcomefeed::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        OutcomeFeedClient,
        PredictionMarketFeed,
        ResolvedOutcome,
        ResolutionParadigm,
        FeedStatus,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the outcome feed module (synchronized with workspace version).
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
