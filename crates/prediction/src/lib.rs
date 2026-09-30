//! Prediction market module for the Morpheum SDK.
//!
//! Provides queries for prediction market state, implied probabilities, and
//! fee statistics, plus the event types for consuming streaming market events.
//!
//! The chain has no transaction form for market creation, resolution,
//! disputes, or light challenges, so this crate carries no builders for them.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

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
    QueryImpliedProbabilityRequest, QueryPredictionMarketRequest, QueryPredictionMarketsRequest,
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
