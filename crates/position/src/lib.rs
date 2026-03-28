//! Position module for the Morpheum SDK.
//!
//! This module provides full support for managing trading positions on Morpheum,
//! including opening, updating, closing positions (both direct and bucket-scoped),
//! and querying position state, open positions, and aggregated long/short volume.
//!
//! It integrates with the bucket, risk, and CLOB modules, and supports both
//! linear and power perpetual position types.

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
    PositionConfig,
    PositionEntry,
    PositionSide,
    PositionState,
};

pub use requests::{
    CloseBucketPositionRequest,
    ClosePositionRequest,
    GetLongShortVolumeRequest,
    GetPositionRequest,
    ListOpenPositionsRequest,
    OpenPositionRequest,
    UpdatePositionRequest,
};

pub use builder::{
    CloseBucketPositionBuilder,
    ClosePositionBuilder,
    OpenPositionBuilder,
    UpdatePositionBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the position module.
///
/// Most users should start with:
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
        OpenPositionBuilder,
        UpdatePositionBuilder,
        ClosePositionBuilder,
        CloseBucketPositionBuilder,
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
