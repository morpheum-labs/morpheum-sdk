//! Fluent builders for the price feed module.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{DeregisterFeedRequest, RegisterFeedRequest};
use crate::types::{AggregationMethod, FeedKind, PriceFeedConfig, SourceConfig};

// ====================== REGISTER FEED ======================

/// Fluent builder for registering a price feed.
#[derive(Default)]
pub struct RegisterFeedBuilder {
    from_address: Option<String>,
    symbol: Option<String>,
    sources: Vec<SourceConfig>,
    agg_method: AggregationMethod,
    decimals: u32,
    threshold_pct: u32,
    heartbeat_sec: u32,
    staleness_sec: u32,
    min_answer: String,
    max_answer: String,
    base_asset_index: Option<u64>,
    quote_asset_index: Option<u64>,
    feed_kind: FeedKind,
    base_feed_index: u64,
    quote_feed_index: u64,
}

impl RegisterFeedBuilder {
    pub fn new() -> Self {
        Self { decimals: 8, agg_method: AggregationMethod::Median, ..Self::default() }
    }

    pub fn from_address(mut self, v: impl Into<String>) -> Self { self.from_address = Some(v.into()); self }
    pub fn symbol(mut self, v: impl Into<String>) -> Self { self.symbol = Some(v.into()); self }
    pub fn add_source(mut self, source_type: impl Into<String>, params: BTreeMap<String, String>) -> Self {
        self.sources.push(SourceConfig { source_type: source_type.into(), params });
        self
    }
    pub fn agg_method(mut self, v: AggregationMethod) -> Self { self.agg_method = v; self }
    pub fn decimals(mut self, v: u32) -> Self { self.decimals = v; self }
    pub fn threshold_pct(mut self, v: u32) -> Self { self.threshold_pct = v; self }
    pub fn heartbeat_sec(mut self, v: u32) -> Self { self.heartbeat_sec = v; self }
    pub fn staleness_sec(mut self, v: u32) -> Self { self.staleness_sec = v; self }
    pub fn min_answer(mut self, v: impl Into<String>) -> Self { self.min_answer = v.into(); self }
    pub fn max_answer(mut self, v: impl Into<String>) -> Self { self.max_answer = v.into(); self }
    pub fn base_asset_index(mut self, v: u64) -> Self { self.base_asset_index = Some(v); self }
    pub fn quote_asset_index(mut self, v: u64) -> Self { self.quote_asset_index = Some(v); self }
    pub fn derived(mut self, base_feed: u64, quote_feed: u64) -> Self {
        self.feed_kind = FeedKind::Derived;
        self.base_feed_index = base_feed;
        self.quote_feed_index = quote_feed;
        self
    }

    pub fn build(self) -> Result<RegisterFeedRequest, SdkError> {
        if self.sources.is_empty() {
            return Err(SdkError::invalid_input("at least one source is required"));
        }

        let config = PriceFeedConfig {
            sources: self.sources, agg_method: self.agg_method,
            decimals: self.decimals, threshold_pct: self.threshold_pct,
            heartbeat_sec: self.heartbeat_sec, staleness_sec: self.staleness_sec,
            min_answer: self.min_answer, max_answer: self.max_answer,
        };

        let mut req = RegisterFeedRequest::new(
            self.from_address.ok_or_else(|| SdkError::invalid_input("from_address is required"))?,
            self.symbol.ok_or_else(|| SdkError::invalid_input("symbol is required"))?,
            config,
            self.base_asset_index.ok_or_else(|| SdkError::invalid_input("base_asset_index is required"))?,
            self.quote_asset_index.ok_or_else(|| SdkError::invalid_input("quote_asset_index is required"))?,
        );

        if self.feed_kind == FeedKind::Derived {
            req.feed_kind = FeedKind::Derived;
            req.base_feed_index = self.base_feed_index;
            req.quote_feed_index = self.quote_feed_index;
        }

        Ok(req)
    }
}

// ====================== DEREGISTER FEED ======================

/// Fluent builder for deregistering a price feed.
#[derive(Default)]
pub struct DeregisterFeedBuilder {
    from_address: Option<String>,
    feed_index: Option<u64>,
}

impl DeregisterFeedBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn from_address(mut self, v: impl Into<String>) -> Self { self.from_address = Some(v.into()); self }
    pub fn feed_index(mut self, v: u64) -> Self { self.feed_index = Some(v); self }

    pub fn build(self) -> Result<DeregisterFeedRequest, SdkError> {
        Ok(DeregisterFeedRequest::new(
            self.from_address.ok_or_else(|| SdkError::invalid_input("from_address is required"))?,
            self.feed_index.ok_or_else(|| SdkError::invalid_input("feed_index is required"))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_feed_builder_works() {
        let req = RegisterFeedBuilder::new()
            .from_address("morph1xyz").symbol("BTC/USD")
            .add_source("chainlink", BTreeMap::new())
            .base_asset_index(0).quote_asset_index(1)
            .heartbeat_sec(60).staleness_sec(300)
            .build().unwrap();
        assert_eq!(req.symbol, "BTC/USD");
        assert_eq!(req.feed_kind, FeedKind::Direct);
    }

    #[test]
    fn register_derived_feed_builder_works() {
        let req = RegisterFeedBuilder::new()
            .from_address("morph1xyz").symbol("BTC/USDC")
            .add_source("derived", BTreeMap::new())
            .base_asset_index(0).quote_asset_index(2)
            .derived(1, 3)
            .build().unwrap();
        assert_eq!(req.feed_kind, FeedKind::Derived);
        assert_eq!(req.base_feed_index, 1);
        assert_eq!(req.quote_feed_index, 3);
    }

    #[test]
    fn register_feed_requires_source() {
        let result = RegisterFeedBuilder::new()
            .from_address("morph1xyz").symbol("BTC/USD")
            .base_asset_index(0).quote_asset_index(1)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn register_feed_validation() {
        assert!(RegisterFeedBuilder::new().build().is_err());
    }

    #[test]
    fn deregister_feed_builder_works() {
        let req = DeregisterFeedBuilder::new()
            .from_address("morph1xyz").feed_index(1)
            .build().unwrap();
        assert_eq!(req.feed_index, 1);
    }

    #[test]
    fn deregister_feed_validation() {
        assert!(DeregisterFeedBuilder::new().build().is_err());
    }
}
