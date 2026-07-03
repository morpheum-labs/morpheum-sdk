//! Risk module for the Morpheum SDK.
//!
//! Provides support for querying liquidation heatmaps, OI ratios,
//! maintenance margin calculations, risk configuration updates,
//! liquidation triggering, and consuming streaming risk events.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod builder;
pub mod client;
pub mod requests;
pub mod types;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{HeatmapResult, OiRatioResult, RiskClient};

pub use types::{
    AuctionBackstopped, AuctionCleared, AuctionExpired, AuctionOpened, BucketRiskSummary,
    HeatmapBand, HeatmapData, HeatmapUpdatedEvent, LiquidationPlanBand, OiUpdated, RiskConfig,
    RiskEvent, RiskParams,
};

pub use requests::{
    GetHeatmapRequest, GetMaintenanceMarginRequest, GetOiRatioRequest, GetParamsRequest,
    TriggerLiquidationRequest, UpdateParamsRequest,
};

pub use builder::{TriggerLiquidationBuilder, UpdateParamsBuilder};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the risk module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_risk::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        AccountId, ChainId, HeatmapResult, OiRatioResult, RiskClient, RiskConfig, RiskEvent,
        SdkError, SignedTx,
    };
}

/// Current version of the risk module (synchronized with workspace version).
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
