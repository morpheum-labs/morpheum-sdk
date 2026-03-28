//! Domain types for the price feed module.
//!
//! Covers aggregation methods, feed kinds, source configuration,
//! feed config, registered feeds, price entries, and price sources.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::pricefeed::v1 as proto;

// ====================== ENUMS ======================

/// How multiple price sources are combined.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AggregationMethod {
    #[default]
    Unspecified,
    Median,
    Twap,
    WeightedMedian,
}

impl From<i32> for AggregationMethod {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Median, 2 => Self::Twap, 3 => Self::WeightedMedian,
            _ => Self::Unspecified,
        }
    }
}

impl From<AggregationMethod> for i32 {
    fn from(a: AggregationMethod) -> Self {
        match a {
            AggregationMethod::Unspecified => 0, AggregationMethod::Median => 1,
            AggregationMethod::Twap => 2, AggregationMethod::WeightedMedian => 3,
        }
    }
}

/// Distinguishes direct oracle feeds from cross-rate derived feeds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FeedKind {
    #[default]
    Direct,
    Derived,
}

impl From<i32> for FeedKind {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Derived,
            _ => Self::Direct,
        }
    }
}

impl From<FeedKind> for i32 {
    fn from(k: FeedKind) -> Self {
        match k {
            FeedKind::Direct => 0, FeedKind::Derived => 1,
        }
    }
}

// ====================== DOMAIN TYPES ======================

/// Single external data source configuration (Chainlink, Pyth, Band, etc.).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceConfig {
    pub source_type: String,
    pub params: BTreeMap<String, String>,
}

impl From<proto::SourceConfig> for SourceConfig {
    fn from(p: proto::SourceConfig) -> Self {
        Self { source_type: p.source_type, params: p.params.into_iter().collect() }
    }
}

impl From<SourceConfig> for proto::SourceConfig {
    fn from(s: SourceConfig) -> Self {
        Self { source_type: s.source_type, params: s.params.into_iter().collect() }
    }
}

/// Feed acquisition, aggregation, and validation configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PriceFeedConfig {
    pub sources: Vec<SourceConfig>,
    pub agg_method: AggregationMethod,
    pub decimals: u32,
    pub threshold_pct: u32,
    pub heartbeat_sec: u32,
    pub staleness_sec: u32,
    pub min_answer: String,
    pub max_answer: String,
}

impl From<proto::PriceFeedConfig> for PriceFeedConfig {
    fn from(p: proto::PriceFeedConfig) -> Self {
        Self {
            sources: p.sources.into_iter().map(Into::into).collect(),
            agg_method: AggregationMethod::from(p.agg_method),
            decimals: p.decimals, threshold_pct: p.threshold_pct,
            heartbeat_sec: p.heartbeat_sec, staleness_sec: p.staleness_sec,
            min_answer: p.min_answer, max_answer: p.max_answer,
        }
    }
}

impl From<PriceFeedConfig> for proto::PriceFeedConfig {
    fn from(c: PriceFeedConfig) -> Self {
        Self {
            sources: c.sources.into_iter().map(Into::into).collect(),
            agg_method: i32::from(c.agg_method),
            decimals: c.decimals, threshold_pct: c.threshold_pct,
            heartbeat_sec: c.heartbeat_sec, staleness_sec: c.staleness_sec,
            min_answer: c.min_answer, max_answer: c.max_answer,
        }
    }
}

/// Registered price feed entry.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PriceFeed {
    pub feed_index: u64,
    pub symbol: String,
    pub active: bool,
    pub config: Option<PriceFeedConfig>,
    pub shard_id: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub base_asset_index: u64,
    pub quote_asset_index: u64,
    pub feed_kind: FeedKind,
    pub base_feed_index: u64,
    pub quote_feed_index: u64,
}

impl From<proto::PriceFeed> for PriceFeed {
    fn from(p: proto::PriceFeed) -> Self {
        Self {
            feed_index: p.feed_index, symbol: p.symbol, active: p.active,
            config: p.config.map(Into::into), shard_id: p.shard_id,
            created_at: p.created_at.as_ref().map_or(0, |t| t.seconds as u64),
            updated_at: p.updated_at.as_ref().map_or(0, |t| t.seconds as u64),
            base_asset_index: p.base_asset_index, quote_asset_index: p.quote_asset_index,
            feed_kind: FeedKind::from(p.feed_kind),
            base_feed_index: p.base_feed_index, quote_feed_index: p.quote_feed_index,
        }
    }
}

/// Point-in-time price observation (satoshi-scaled u64).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PriceEntry {
    pub value: u64,
    pub timestamp: u64,
    pub source_count: u32,
    pub confidence: u32,
}

impl From<proto::PriceEntry> for PriceEntry {
    fn from(p: proto::PriceEntry) -> Self {
        Self { value: p.value, timestamp: p.timestamp, source_count: p.source_count, confidence: p.confidence }
    }
}

/// Individual contribution from a single exchange/oracle.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PriceSource {
    pub exchange: String,
    pub price: String,
    pub weight: String,
    pub is_active: bool,
    pub last_update: u64,
}

impl From<proto::PriceSource> for PriceSource {
    fn from(p: proto::PriceSource) -> Self {
        Self {
            exchange: p.exchange, price: p.price, weight: p.weight,
            is_active: p.is_active,
            last_update: p.last_update.as_ref().map_or(0, |t| t.seconds as u64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn aggregation_method_roundtrip() {
        for a in [AggregationMethod::Median, AggregationMethod::Twap, AggregationMethod::WeightedMedian] {
            let v: i32 = a.into();
            assert_eq!(a, AggregationMethod::from(v));
        }
        assert_eq!(AggregationMethod::Unspecified, AggregationMethod::from(99));
    }

    #[test]
    fn feed_kind_roundtrip() {
        for k in [FeedKind::Direct, FeedKind::Derived] {
            let v: i32 = k.into();
            assert_eq!(k, FeedKind::from(v));
        }
    }

    #[test]
    fn source_config_roundtrip() {
        let mut params = BTreeMap::new();
        params.insert("pair".into(), "BTC/USD".into());
        let s = SourceConfig { source_type: "chainlink".into(), params };
        let p: proto::SourceConfig = s.clone().into();
        let s2: SourceConfig = p.into();
        assert_eq!(s, s2);
    }

    #[test]
    fn price_feed_from_proto() {
        let p = proto::PriceFeed {
            feed_index: 1, symbol: "BTC/USD".into(), active: true,
            config: None, shard_id: "shard-0".into(),
            created_at: Some(morpheum_proto::google::protobuf::Timestamp { seconds: 1000, nanos: 0 }),
            updated_at: None,
            base_asset_index: 0, quote_asset_index: 1,
            feed_kind: 0, base_feed_index: 0, quote_feed_index: 0,
        };
        let f: PriceFeed = p.into();
        assert_eq!(f.feed_index, 1);
        assert_eq!(f.symbol, "BTC/USD");
        assert!(f.active);
        assert_eq!(f.created_at, 1000);
        assert_eq!(f.updated_at, 0);
        assert_eq!(f.feed_kind, FeedKind::Direct);
    }

    #[test]
    fn price_entry_from_proto() {
        let p = proto::PriceEntry { value: 5_000_000_000_000, timestamp: 1700000000, source_count: 5, confidence: 95 };
        let e: PriceEntry = p.into();
        assert_eq!(e.value, 5_000_000_000_000);
        assert_eq!(e.source_count, 5);
    }

    #[test]
    fn price_feed_config_roundtrip() {
        let c = PriceFeedConfig {
            sources: vec![SourceConfig { source_type: "pyth".into(), params: BTreeMap::new() }],
            agg_method: AggregationMethod::Median, decimals: 8,
            threshold_pct: 5, heartbeat_sec: 60, staleness_sec: 300,
            min_answer: "0".into(), max_answer: "100000000000000".into(),
        };
        let p: proto::PriceFeedConfig = c.clone().into();
        let c2: PriceFeedConfig = p.into();
        assert_eq!(c, c2);
    }
}
