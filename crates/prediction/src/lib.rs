//! Prediction market module for the Morpheum SDK.
//!
//! Provides support for creating prediction markets, resolving outcomes,
//! disputing resolutions (bonded and light challenges), querying market
//! state, implied probabilities, and consuming streaming market events.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod builder;
pub mod client;
pub mod requests;
pub mod types;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{PredictionClient, PredictionMarketsPage};

pub use types::{
    DisputeAcceptedEvent,
    DisputeConfig,
    DisputeRejectedEvent,
    DisputeVoidedEvent,
    FeeAppliedEvent,
    LightChallengeEscalatedEvent,
    LightChallengeOpenedEvent,
    LightChallengeResolvedEvent,
    LightChallengeVoteEvent,
    MarketCreatedEvent,
    MarketDisputedEvent,
    PredictionKlineUpdate,
    PredictionMarket,
    // Stream event types
    PredictionMarketEvent,
    PredictionPhase,
    PredictionPriceUpdate,
    ResolvedOutcome,
};

pub use requests::{
    CreateMarketRequest, DisputeMarketRequest, LightChallengeVoteRequest,
    OpenLightChallengeRequest, QueryImpliedProbabilityRequest, QueryPredictionMarketRequest,
    QueryPredictionMarketsRequest, ResolveMarketRequest,
};

pub use builder::{
    CreateMarketBuilder, DisputeMarketBuilder, LightChallengeVoteBuilder,
    OpenLightChallengeBuilder, ResolveMarketBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the prediction module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_prediction::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        AccountId, ChainId, PredictionClient, PredictionMarket, PredictionPhase, ResolvedOutcome,
        SdkError, SignedTx,
    };
}

/// Current version of the prediction module (synchronized with workspace version).
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
