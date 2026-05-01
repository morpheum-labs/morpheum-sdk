//! CLOB (Central Limit Order Book) module for the Morpheum SDK.
//!
//! This module provides full support for interacting with Morpheum's CLOB,
//! including order placement (limit, market, stop, take-profit), modification,
//! cancellation, batch orders, orderbook snapshot queries, trade history,
//! funding rate queries, market-maker quote management, and checksum validation
//! for WebSocket data integrity.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod builder;
pub mod client;
pub mod requests;
pub mod types;

// ==================== PUBLIC RE-EXPORTS ====================

pub use client::ClobClient;

pub use types::{
    ClobParams, FundingRate, FundingRateEntry, MarketMakerQuote, Order, OrderBookChecksum,
    OrderBookSnapshot, OrderLevel, OrderStatus, OrderType, PriceLevel, Side, TimeInForce, Trade,
};

pub use requests::{
    CancelMarketMakerQuoteRequest, CancelOrderRequest, GetChecksumRequest, ModifyOrderRequest,
    PlaceBatchOrdersRequest, PlaceOrderRequest, ProvideMarketMakerQuoteRequest,
    QueryActiveMarketMakerQuotesRequest, QueryFundingRateRequest, QueryFundingRatesRequest,
    QueryMarketMakerQuoteByIdRequest, QueryOrderBookSnapshotRequest, QueryOrderByIdRequest,
    QueryOrdersByAddressRequest, QueryOrdersByMarketRequest, QueryTradesByAddressRequest,
    QueryTradesByMarketRequest, UpdateParamsRequest,
};

pub use builder::{
    CancelMarketMakerQuoteBuilder, CancelOrderBuilder, ModifyOrderBuilder, PlaceBatchOrdersBuilder,
    PlaceOrderBuilder, ProvideMarketMakerQuoteBuilder,
};

pub use morpheum_sdk_core::{AccountId, ChainId, SdkError, SignedTx};

/// Recommended prelude for the CLOB module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_clob::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        AccountId, CancelOrderBuilder, ChainId, ClobClient, FundingRate, MarketMakerQuote,
        ModifyOrderBuilder, Order, OrderBookSnapshot, OrderStatus, OrderType,
        PlaceBatchOrdersBuilder, PlaceOrderBuilder, PriceLevel, SdkError, Side, SignedTx,
        TimeInForce, Trade,
    };
}

/// Current version of the CLOB module (synchronized with workspace version).
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
