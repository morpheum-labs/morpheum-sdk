//! Liquidity pool module for the Morpheum SDK.
//!
//! Provides support for creating and managing liquidity pools,
//! querying depth metrics, monitoring pool health, and managing
//! pool parameters for CLOB market making.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::LiquidityClient;

pub use types::{
    DepthMetrics,
    LiquidityProviderType,
    PageInfo,
    Pool,
    PoolHealth,
    PoolStatus,
    PoolType,
};

pub use requests::{
    CreatePoolRequest,
    GetDepthMetricsRequest,
    GetPoolHealthRequest,
    GetPoolRequest,
    GetPoolsByMarketRequest,
    ListPoolsRequest,
    UpdatePoolParamsRequest,
};

pub use builder::{
    CreatePoolBuilder,
    UpdatePoolParamsBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the liquidity module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_liquidity::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        LiquidityClient,
        Pool,
        PoolType,
        PoolStatus,
        LiquidityProviderType,
        DepthMetrics,
        PoolHealth,
        PageInfo,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the liquidity module (synchronized with workspace version).
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
