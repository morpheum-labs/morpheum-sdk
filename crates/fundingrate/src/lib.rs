//! Funding-rate module for the Morpheum SDK.
//!
//! Provides support for querying funding rates, next funding times,
//! market profiles, triggering epoch ticks, and applying sharded
//! funding to positions.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{FundingRateClient, FundingRateSnapshot};

pub use types::{
    FundingApplicationMode,
    FundingApplied,
    FundingMarketProfile,
    FundingPosition,
    FundingRateConfig,
    FundingRateData,
    FundingShortfall,
    FundingType,
    MormFundingCutEvent,
};

pub use requests::{
    ApplyShardedFundingRequest,
    EpochTickRequest,
    GetFundingRateRequest,
    GetMarketProfileRequest,
    GetNextFundingTimeRequest,
    UpdateMarketProfileRequest,
};

pub use builder::{
    EpochTickBuilder,
    UpdateMarketProfileBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the funding-rate module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_fundingrate::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        FundingRateClient,
        FundingRateSnapshot,
        FundingRateData,
        FundingMarketProfile,
        FundingApplicationMode,
        FundingType,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the funding-rate module (synchronized with workspace version).
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
