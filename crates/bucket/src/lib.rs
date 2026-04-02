//! Bucket module for the Morpheum SDK.
//!
//! This module provides full support for managing collateralized margin buckets
//! on Morpheum, including bucket creation, margin transfers between buckets and
//! to the bank, bucket PnL queries, health monitoring, and liquidation history.
//!
//! Buckets can be either **isolated** (one position per bucket) or **cross**
//! (multiple positions sharing margin). Position lifecycle management (close,
//! update leverage) is handled by the `position` module.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::BucketClient;

pub use types::{
    AddressPnL,
    AllBucketsBalance,
    Bucket,
    BucketBalance,
    BucketHealthSummary,
    BucketPnL,
    BucketPnLInfo,
    BucketStatus,
    BucketType,
    Liquidation,
    LiquidationEvent,
    LiquidationMetrics,
    Position,
    PositionHealth,
    PositionPnL,
    PositionPnLInfo,
    Side,
};

pub use requests::{
    CloseBucketRequest,
    CreateBucketRequest,
    LiquidateBucketRequest,
    QueryAdlHistoryRequest,
    QueryAddressPnLRequest,
    QueryAllBucketsBalanceByAddressRequest,
    QueryBucketPnLRequest,
    QueryBucketRequest,
    QueryBucketStatusRequest,
    QueryBucketsByAddressRequest,
    QueryLiquidationMetricsRequest,
    QueryLiquidationsRequest,
    QueryPositionHealthRequest,
    QueryPositionsByBucketRequest,
    TransferBetweenBucketsRequest,
    TransferToBankRequest,
};

pub use builder::{
    CloseBucketBuilder,
    CreateBucketBuilder,
    LiquidateBucketBuilder,
    TransferBetweenBucketsBuilder,
    TransferToBankBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the bucket module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_bucket::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        BucketClient,
        Bucket,
        BucketType,
        BucketBalance,
        BucketPnLInfo,
        BucketStatus,
        Position,
        PositionPnLInfo,
        PositionHealth,
        Side,
        CreateBucketBuilder,
        TransferBetweenBucketsBuilder,
        TransferToBankBuilder,
        CloseBucketBuilder,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the bucket module (synchronized with workspace version).
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
