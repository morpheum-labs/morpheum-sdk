//! Position module for the Morpheum SDK.
//!
//! Position lifecycle:
//! - Positions are CREATED from CLOB order fills via the PositionFillApplier hot path.
//!   There is no user-facing "OpenPosition" RPC.
//! - Position SIZE CHANGES come from subsequent fills.
//! - Users can close/reduce positions (ClosePosition) and change leverage (UpdatePositionLeverage).
//!
//! This module provides query support for position state, open positions, long/short volume,
//! positions by address, positions by market, and position PnL.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::PositionClient;

pub use types::{
    LongShortVolume,
    Position,
    PositionConfig,
    PositionEntry,
    PositionPnL,
    PositionSide,
    PositionState,
    Side,
};

pub use requests::{
    ClosePositionRequest,
    GetLongShortVolumeRequest,
    GetPositionRequest,
    ListOpenPositionsRequest,
    QueryAllPositionsByMarketRequest,
    QueryPositionPnLRequest,
    QueryPositionsByAddressRequest,
    UpdatePositionLeverageRequest,
};

pub use builder::{
    ClosePositionBuilder,
    UpdatePositionLeverageBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the position module.
///
/// ```rust
/// use morpheum_sdk_position::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        PositionClient,
        PositionState,
        PositionSide,
        PositionEntry,
        PositionConfig,
        LongShortVolume,
        ClosePositionBuilder,
        UpdatePositionLeverageBuilder,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the position module (synchronized with workspace version).
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
