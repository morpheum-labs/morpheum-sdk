//! Kline (OHLC) module for the Morpheum SDK.
//!
//! Provides support for querying mark prices, VWAP, long/short
//! sentiment ratios, last completed klines, and kline snapshots.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::KlineClient;

pub use types::{
    KlineData,
    KlineEpochBoundary,
    KlineModuleConfig,
    KlinePruned,
    KlineSentimentUpdated,
    KlineUpdated,
    LastKline,
    LongShortRatio,
    MarkPriceWithSpread,
    Period,
    PositionSnapshot,
    SentimentData,
    TradeData,
    Vwap,
};

pub use requests::{
    GetLastKlineRequest,
    GetLongShortRatioRequest,
    GetMarkPriceWithSpreadRequest,
    GetVwapRequest,
    QueryKlinesSnapshotRequest,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the kline module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_kline::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        KlineClient,
        KlineData,
        LastKline,
        LongShortRatio,
        MarkPriceWithSpread,
        Period,
        Vwap,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the kline module (synchronized with workspace version).
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
