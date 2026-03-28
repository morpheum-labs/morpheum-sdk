//! Domain types for the mark price module.
//!
//! Covers mark price sources, canonical price data, per-market
//! weight configuration, module config, and streaming events.

use alloc::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::markprice::v1 as proto;

// ====================== ENUMS ======================

/// Source of the canonical mark price.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MarkSource {
    Unspecified,
    /// Order-book TWAP (primary).
    Twap,
    /// Kline 1m close (fallback).
    Kline,
    /// Oracle index price (event-driven or HotPath fallback).
    OracleIndex,
}

impl From<i32> for MarkSource {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Twap, 2 => Self::Kline, 3 => Self::OracleIndex,
            _ => Self::Unspecified,
        }
    }
}

impl From<MarkSource> for i32 {
    fn from(s: MarkSource) -> Self {
        match s {
            MarkSource::Unspecified => 0, MarkSource::Twap => 1,
            MarkSource::Kline => 2, MarkSource::OracleIndex => 3,
        }
    }
}

// ====================== DOMAIN TYPES ======================

/// Canonical mark price for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarkPriceData {
    pub market_index: u64,
    pub mark_price: u64,
    pub source: MarkSource,
    pub delta_bps: i64,
}

impl From<proto::MarkPriceData> for MarkPriceData {
    fn from(p: proto::MarkPriceData) -> Self {
        Self {
            market_index: p.market_index,
            mark_price: p.mark_price,
            source: MarkSource::from(p.source),
            delta_bps: p.delta_bps,
        }
    }
}

/// Per-market mark price weight configuration. Weights in basis points (sum to 10000).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarkConfig {
    pub weight_twap_bps: u32,
    pub weight_oracle_index_bps: u32,
    pub weight_kline_bps: u32,
    pub staleness_blocks: u64,
    pub strategy: String,
}

impl From<proto::MarkConfig> for MarkConfig {
    fn from(p: proto::MarkConfig) -> Self {
        Self {
            weight_twap_bps: p.weight_twap_bps,
            weight_oracle_index_bps: p.weight_oracle_index_bps,
            weight_kline_bps: p.weight_kline_bps,
            staleness_blocks: p.staleness_blocks,
            strategy: p.strategy,
        }
    }
}

impl From<MarkConfig> for proto::MarkConfig {
    fn from(c: MarkConfig) -> Self {
        Self {
            weight_twap_bps: c.weight_twap_bps,
            weight_oracle_index_bps: c.weight_oracle_index_bps,
            weight_kline_bps: c.weight_kline_bps,
            staleness_blocks: c.staleness_blocks,
            strategy: c.strategy,
        }
    }
}

/// Module-level mark price configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarkPriceModuleConfig {
    pub staleness_threshold: u64,
    pub enable_kline_fallback: bool,
    pub price_move_alert_bps: i64,
}

impl From<proto::MarkPriceModuleConfig> for MarkPriceModuleConfig {
    fn from(p: proto::MarkPriceModuleConfig) -> Self {
        Self {
            staleness_threshold: p.staleness_threshold,
            enable_kline_fallback: p.enable_kline_fallback,
            price_move_alert_bps: p.price_move_alert_bps,
        }
    }
}

// ====================== EVENTS ======================

/// Emitted when canonical mark price changes.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarkPriceUpdated {
    pub market_index: u64,
    pub mark_price: u64,
    pub source: MarkSource,
    pub delta_bps: i64,
}

impl From<proto::MarkPriceUpdated> for MarkPriceUpdated {
    fn from(p: proto::MarkPriceUpdated) -> Self {
        Self {
            market_index: p.market_index,
            mark_price: p.mark_price,
            source: MarkSource::from(p.source),
            delta_bps: p.delta_bps,
        }
    }
}

/// Emitted on significant price movement.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PriceMoveAlert {
    pub market_index: u64,
    pub mark_price: u64,
    pub delta_bps: i64,
}

impl From<proto::PriceMoveAlert> for PriceMoveAlert {
    fn from(p: proto::PriceMoveAlert) -> Self {
        Self { market_index: p.market_index, mark_price: p.mark_price, delta_bps: p.delta_bps }
    }
}

/// Mark price with source attribution (query response).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarkPriceWithSource {
    pub mark_price: u64,
    pub source: MarkSource,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mark_source_roundtrip() {
        for s in [MarkSource::Twap, MarkSource::Kline, MarkSource::OracleIndex] {
            let v: i32 = s.into();
            assert_eq!(s, MarkSource::from(v));
        }
        assert_eq!(MarkSource::Unspecified, MarkSource::from(99));
    }

    #[test]
    fn mark_price_data_from_proto() {
        let p = proto::MarkPriceData {
            market_index: 1, mark_price: 50_000_000_000, source: 1, delta_bps: -25,
        };
        let d: MarkPriceData = p.into();
        assert_eq!(d.source, MarkSource::Twap);
        assert_eq!(d.delta_bps, -25);
    }

    #[test]
    fn mark_config_bidirectional() {
        let cfg = MarkConfig {
            weight_twap_bps: 8000, weight_oracle_index_bps: 1500,
            weight_kline_bps: 500, staleness_blocks: 10,
            strategy: "linear_perp".into(),
        };
        let proto_cfg: proto::MarkConfig = cfg.clone().into();
        let back: MarkConfig = proto_cfg.into();
        assert_eq!(cfg, back);
    }

    #[test]
    fn mark_price_updated_from_proto() {
        let p = proto::MarkPriceUpdated {
            market_index: 42, mark_price: 50_000, source: 3, delta_bps: 100,
        };
        let u: MarkPriceUpdated = p.into();
        assert_eq!(u.source, MarkSource::OracleIndex);
    }
}
