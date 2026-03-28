//! Request wrappers for the price feed module.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::pricefeed::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::{FeedKind, PriceFeedConfig};

// ====================== TRANSACTION REQUESTS ======================

/// Register a new price feed.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterFeedRequest {
    pub from_address: String,
    pub symbol: String,
    pub config: PriceFeedConfig,
    pub base_asset_index: u64,
    pub quote_asset_index: u64,
    pub feed_kind: FeedKind,
    pub base_feed_index: u64,
    pub quote_feed_index: u64,
}

impl RegisterFeedRequest {
    pub fn new(
        from_address: impl Into<String>, symbol: impl Into<String>,
        config: PriceFeedConfig, base_asset_index: u64, quote_asset_index: u64,
    ) -> Self {
        Self {
            from_address: from_address.into(), symbol: symbol.into(),
            config, base_asset_index, quote_asset_index,
            feed_kind: FeedKind::Direct, base_feed_index: 0, quote_feed_index: 0,
        }
    }

    pub fn derived(
        from_address: impl Into<String>, symbol: impl Into<String>,
        config: PriceFeedConfig, base_asset_index: u64, quote_asset_index: u64,
        base_feed_index: u64, quote_feed_index: u64,
    ) -> Self {
        Self {
            from_address: from_address.into(), symbol: symbol.into(),
            config, base_asset_index, quote_asset_index,
            feed_kind: FeedKind::Derived, base_feed_index, quote_feed_index,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgRegisterFeedRequest {
            from_address: self.from_address.clone(), symbol: self.symbol.clone(),
            config: Some(self.config.clone().into()),
            base_asset_index: self.base_asset_index, quote_asset_index: self.quote_asset_index,
            feed_kind: i32::from(self.feed_kind),
            base_feed_index: self.base_feed_index, quote_feed_index: self.quote_feed_index,
        };
        ProtoAny { type_url: "/pricefeed.v1.MsgRegisterFeedRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Deregister an existing price feed.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeregisterFeedRequest {
    pub from_address: String,
    pub feed_index: u64,
}

impl DeregisterFeedRequest {
    pub fn new(from_address: impl Into<String>, feed_index: u64) -> Self {
        Self { from_address: from_address.into(), feed_index }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgDeregisterFeedRequest {
            from_address: self.from_address.clone(), feed_index: self.feed_index,
        };
        ProtoAny { type_url: "/pricefeed.v1.MsgDeregisterFeedRequest".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query a single feed by index.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryFeedRequest {
    pub feed_index: u64,
}

impl QueryFeedRequest {
    pub fn new(feed_index: u64) -> Self { Self { feed_index } }
}

impl From<QueryFeedRequest> for proto::QueryFeedRequest {
    fn from(r: QueryFeedRequest) -> Self { Self { feed_index: r.feed_index } }
}

/// Query a single feed by symbol.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryFeedBySymbolRequest {
    pub symbol: String,
}

impl QueryFeedBySymbolRequest {
    pub fn new(symbol: impl Into<String>) -> Self { Self { symbol: symbol.into() } }
}

impl From<QueryFeedBySymbolRequest> for proto::QueryFeedBySymbolRequest {
    fn from(r: QueryFeedBySymbolRequest) -> Self { Self { symbol: r.symbol } }
}

/// List feeds with pagination and optional active-only filter.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryFeedsRequest {
    pub limit: i32,
    pub offset: i32,
    pub active_only: bool,
}

impl QueryFeedsRequest {
    pub fn new(limit: i32, offset: i32) -> Self { Self { limit, offset, active_only: false } }
    pub fn active_only(mut self, v: bool) -> Self { self.active_only = v; self }
}

impl From<QueryFeedsRequest> for proto::QueryFeedsRequest {
    fn from(r: QueryFeedsRequest) -> Self { Self { limit: r.limit, offset: r.offset, active_only: r.active_only } }
}

/// Query the latest price for a feed.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPriceRequest {
    pub feed_index: u64,
}

impl QueryPriceRequest {
    pub fn new(feed_index: u64) -> Self { Self { feed_index } }
}

impl From<QueryPriceRequest> for proto::QueryPriceRequest {
    fn from(r: QueryPriceRequest) -> Self { Self { feed_index: r.feed_index } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;
    use crate::types::AggregationMethod;

    fn test_config() -> PriceFeedConfig {
        PriceFeedConfig {
            sources: Vec::new(), agg_method: AggregationMethod::Median, decimals: 8,
            threshold_pct: 5, heartbeat_sec: 60, staleness_sec: 300,
            min_answer: "0".into(), max_answer: "100000000000000".into(),
        }
    }

    #[test]
    fn register_feed_to_any() {
        let any = RegisterFeedRequest::new("morph1xyz", "BTC/USD", test_config(), 0, 1).to_any();
        assert_eq!(any.type_url, "/pricefeed.v1.MsgRegisterFeedRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn register_derived_feed_to_any() {
        let any = RegisterFeedRequest::derived("morph1xyz", "BTC/USDC", test_config(), 0, 2, 1, 3).to_any();
        assert_eq!(any.type_url, "/pricefeed.v1.MsgRegisterFeedRequest");
    }

    #[test]
    fn deregister_feed_to_any() {
        let any = DeregisterFeedRequest::new("morph1xyz", 1).to_any();
        assert_eq!(any.type_url, "/pricefeed.v1.MsgDeregisterFeedRequest");
    }

    #[test]
    fn query_conversions() {
        let p: proto::QueryFeedRequest = QueryFeedRequest::new(1).into();
        assert_eq!(p.feed_index, 1);

        let p: proto::QueryFeedBySymbolRequest = QueryFeedBySymbolRequest::new("BTC/USD").into();
        assert_eq!(p.symbol, "BTC/USD");

        let p: proto::QueryFeedsRequest = QueryFeedsRequest::new(50, 0).active_only(true).into();
        assert!(p.active_only);

        let p: proto::QueryPriceRequest = QueryPriceRequest::new(1).into();
        assert_eq!(p.feed_index, 1);
    }
}
