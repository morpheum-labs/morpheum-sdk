//! Risk module for the Morpheum SDK.
//!
//! Provides support for querying liquidation heatmaps, OI ratios,
//! maintenance margin calculations, risk configuration updates,
//! shortfall reporting, and consuming streaming risk events.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{HeatmapResult, OiRatioResult, RiskClient};

pub use types::{
    AuctionParams,
    BucketRiskSummary,
    ContagionDetected,
    HeatmapBand,
    HeatmapData,
    HeatmapUpdatedEvent,
    InsurancePayoutRequested,
    LiquidationPlan,
    LiquidationPlanBand,
    LiquidationShortfallReady,
    LiquidationTriggered,
    OiUpdated,
    PreTradeMarginResult,
    RiskConfig,
    RiskEvent,
    ShortfallIntendedPath,
};

pub use requests::{
    BucketLiquidationExecutedRequest,
    EpochRiskTickRequest,
    GetHeatmapRequest,
    GetMaintenanceMarginRequest,
    GetOiRatioRequest,
    LiquidationCheckRequest,
    ShortfallReportRequest,
    UpdateRiskConfigRequest,
};

pub use builder::{
    BucketLiquidationExecutedBuilder,
    EpochRiskTickBuilder,
    LiquidationCheckBuilder,
    ShortfallReportBuilder,
    UpdateRiskConfigBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the risk module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_risk::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        RiskClient,
        RiskConfig,
        RiskEvent,
        HeatmapResult,
        OiRatioResult,
        ShortfallIntendedPath,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
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
