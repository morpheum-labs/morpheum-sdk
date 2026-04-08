//! Bonding-curve module for the Morpheum SDK.
//!
//! Provides full support for interacting with Morpheum's bonding-curve
//! launchpad, including agent token creation, buying/selling on the curve,
//! graduation to CLMM, querying curve state and pricing, and
//! prediction-enhanced token support.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::{BondingCurveClient, PriceSnapshot};

pub use types::{
    BondingCurveParams,
    BondingCurveState,
    BuyExecuted,
    CurveStatus,
    CurveType,
    GraduationComplete,
    GraduationThresholdReached,
    LpAntiRugStrategy,
    PredictionEnhancement,
    PredictionEnhancementActivated,
    PredictionFeed,
    PredictionMetadata,
    PredictionMode,
    ReputationBondTier,
    SellExecuted,
};

pub use requests::{
    BuyRequest,
    CreateAgentTokenRequest,
    ExecuteGraduationRequest,
    GetCurveStateRequest,
    GetPriceRequest,
    QueryParamsRequest,
    SellRequest,
};

pub use builder::{
    BuyBuilder,
    CreateAgentTokenBuilder,
    ExecuteGraduationBuilder,
    SellBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the bonding-curve module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_bondingcurve::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        BondingCurveClient,
        BondingCurveState,
        CurveStatus,
        CurveType,
        PriceSnapshot,
        CreateAgentTokenBuilder,
        BuyBuilder,
        SellBuilder,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the bonding-curve module (synchronized with workspace version).
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
