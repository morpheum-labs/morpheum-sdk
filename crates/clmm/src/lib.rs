//! CLMM (Concentrated Liquidity AMM) module for the Morpheum SDK.
//!
//! Provides full support for interacting with Morpheum's concentrated-liquidity
//! AMM, including adding/removing liquidity with tick ranges, fee collection,
//! yield claiming (standard and boosted), swap simulation, AMM quoting,
//! liquidity depth queries, pool risk analysis, and ReClmm glide operations.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::ClmmClient;

pub use types::{
    BoostedBuffer,
    BurnEvent,
    ClmmPosition,
    ClmmQuote,
    CollectEvent,
    GlideSimulation,
    LiquidityDepthBand,
    MintEvent,
    PoolRiskSummary,
    QuoteResult,
    ReClmmGlideUpdated,
    Side,
    SwapExecuted,
    SwapSimulation,
};

pub use requests::{
    AddLiquidityRequest,
    ClaimBoostedYieldRequest,
    ClaimYieldRequest,
    CollectFeesRequest,
    ForceGlideRequest,
    GetBoostedBufferRequest,
    GetLiquidityDepthRequest,
    GetPoolRiskSummaryRequest,
    GetPositionRequest,
    GetQuoteRequest,
    RemoveLiquidityRequest,
    SimulateReClmmGlideRequest,
    SimulateSwapRequest,
};

pub use builder::{
    AddLiquidityBuilder,
    ClaimBoostedYieldBuilder,
    ClaimYieldBuilder,
    CollectFeesBuilder,
    ForceGlideBuilder,
    RemoveLiquidityBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the CLMM module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_clmm::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        ClmmClient,
        ClmmPosition,
        Side,
        SwapSimulation,
        QuoteResult,
        LiquidityDepthBand,
        AddLiquidityBuilder,
        RemoveLiquidityBuilder,
        CollectFeesBuilder,
        ClaimYieldBuilder,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the CLMM module (synchronized with workspace version).
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
