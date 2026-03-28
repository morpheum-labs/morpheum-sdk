//! Domain types for the kline (OHLC) module.
//!
//! Covers candle periods, OHLC data, sentiment, trade data,
//! position snapshots, streaming events, and module configuration.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::kline::v1 as proto;

// ====================== ENUMS ======================

/// Candle period granularity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Period {
    OneMinute,
    FiveMinutes,
    FifteenMinutes,
    OneHour,
    FourHours,
    OneDay,
}

impl From<Period> for u32 {
    fn from(p: Period) -> Self {
        match p {
            Period::OneMinute => 1,
            Period::FiveMinutes => 2,
            Period::FifteenMinutes => 3,
            Period::OneHour => 4,
            Period::FourHours => 5,
            Period::OneDay => 6,
        }
    }
}

impl Period {
    /// Parse from the proto `u32` period byte. Returns `None` for unknown values.
    pub fn from_u32(v: u32) -> Option<Self> {
        match v {
            1 => Some(Self::OneMinute),
            2 => Some(Self::FiveMinutes),
            3 => Some(Self::FifteenMinutes),
            4 => Some(Self::OneHour),
            5 => Some(Self::FourHours),
            6 => Some(Self::OneDay),
            _ => None,
        }
    }

    /// Returns the human-readable interval string (e.g. `"1m"`, `"4h"`).
    pub fn as_interval_str(&self) -> &'static str {
        match self {
            Self::OneMinute => "1m",
            Self::FiveMinutes => "5m",
            Self::FifteenMinutes => "15m",
            Self::OneHour => "1h",
            Self::FourHours => "4h",
            Self::OneDay => "1d",
        }
    }
}

// ====================== DOMAIN TYPES ======================

/// Single OHLC candle. Prices/volumes are satoshi-scale strings (u256 compatible).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KlineData {
    pub market_index: u64,
    pub period: u32,
    pub open_at_logical: u64,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub volume_base: String,
    pub quote_volume: String,
    pub trade_count: i64,
    pub taker_buy_base: String,
    pub taker_buy_quote: String,
    pub spread_satoshi: u64,
    pub mark_quality: u64,
    pub proof: Vec<u8>,
    pub outcome_id: u32,
}

impl From<proto::KlineData> for KlineData {
    fn from(p: proto::KlineData) -> Self {
        Self {
            market_index: p.market_index,
            period: p.period,
            open_at_logical: p.open_at_logical,
            open: p.open,
            high: p.high,
            low: p.low,
            close: p.close,
            volume_base: p.volume_base,
            quote_volume: p.quote_volume,
            trade_count: p.trade_count,
            taker_buy_base: p.taker_buy_base,
            taker_buy_quote: p.taker_buy_quote,
            spread_satoshi: p.spread_satoshi,
            mark_quality: p.mark_quality,
            proof: p.proof,
            outcome_id: p.outcome_id,
        }
    }
}

/// Long/short sentiment at a point in time.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SentimentData {
    pub market_index: u64,
    pub period: u32,
    pub open_at_logical: u64,
    pub long_short_ratio: u64,
    pub long_oi: String,
    pub short_oi: String,
    pub proof: Vec<u8>,
    pub outcome_id: u32,
}

impl From<proto::SentimentData> for SentimentData {
    fn from(p: proto::SentimentData) -> Self {
        Self {
            market_index: p.market_index,
            period: p.period,
            open_at_logical: p.open_at_logical,
            long_short_ratio: p.long_short_ratio,
            long_oi: p.long_oi,
            short_oi: p.short_oi,
            proof: p.proof,
            outcome_id: p.outcome_id,
        }
    }
}

/// Trade data for ingestion into OHLC candles.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TradeData {
    pub market_index: u64,
    pub price: u64,
    pub quantity: String,
    pub is_taker_buy: bool,
    pub block_height: u64,
    pub logical_timestamp: u64,
    pub feed_id: String,
    pub outcome_id: u32,
}

impl From<TradeData> for proto::TradeData {
    fn from(t: TradeData) -> Self {
        Self {
            market_index: t.market_index,
            price: t.price,
            quantity: t.quantity,
            is_taker_buy: t.is_taker_buy,
            block_height: t.block_height,
            logical_timestamp: t.logical_timestamp,
            feed_id: t.feed_id,
            outcome_id: t.outcome_id,
        }
    }
}

/// Position snapshot for sentiment candle updates.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PositionSnapshot {
    pub market_index: u64,
    pub long_oi: String,
    pub short_oi: String,
    pub block_height: u64,
    pub logical_timestamp: u64,
}

impl From<PositionSnapshot> for proto::PositionSnapshot {
    fn from(s: PositionSnapshot) -> Self {
        Self {
            market_index: s.market_index,
            long_oi: s.long_oi,
            short_oi: s.short_oi,
            block_height: s.block_height,
            logical_timestamp: s.logical_timestamp,
        }
    }
}

/// Module-level kline configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KlineModuleConfig {
    pub staleness_threshold_blocks: u32,
    pub hot_candles_per_period: Vec<u32>,
    pub enable_sentiment_candles: bool,
    pub sentiment_update_interval_blocks: u32,
    pub min_volume_for_vwap: String,
    pub enable_volume_weighted_mark: bool,
    pub spread_multiplier: u64,
    pub mark_quality_threshold: u64,
}

impl From<proto::KlineModuleConfig> for KlineModuleConfig {
    fn from(p: proto::KlineModuleConfig) -> Self {
        Self {
            staleness_threshold_blocks: p.staleness_threshold_blocks,
            hot_candles_per_period: p.hot_candles_per_period,
            enable_sentiment_candles: p.enable_sentiment_candles,
            sentiment_update_interval_blocks: p.sentiment_update_interval_blocks,
            min_volume_for_vwap: p.min_volume_for_vwap,
            enable_volume_weighted_mark: p.enable_volume_weighted_mark,
            spread_multiplier: p.spread_multiplier,
            mark_quality_threshold: p.mark_quality_threshold,
        }
    }
}

// ====================== EVENTS ======================

/// OHLC candle updated (streaming).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KlineUpdated {
    pub market_index: u64,
    pub period: u32,
    pub open_at_logical: u64,
    pub close: u64,
    pub volume_base: String,
    pub spread_satoshi: u64,
    pub mark_quality: u64,
    pub proof: Vec<u8>,
    pub outcome_id: u32,
}

impl From<proto::KlineUpdated> for KlineUpdated {
    fn from(p: proto::KlineUpdated) -> Self {
        Self {
            market_index: p.market_index,
            period: p.period,
            open_at_logical: p.open_at_logical,
            close: p.close,
            volume_base: p.volume_base,
            spread_satoshi: p.spread_satoshi,
            mark_quality: p.mark_quality,
            proof: p.proof,
            outcome_id: p.outcome_id,
        }
    }
}

/// Sentiment candle updated (streaming).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KlineSentimentUpdated {
    pub market_index: u64,
    pub period: u32,
    pub open_at_logical: u64,
    pub long_short_ratio: u64,
    pub long_oi: String,
    pub short_oi: String,
    pub proof: Vec<u8>,
    pub outcome_id: u32,
}

impl From<proto::KlineSentimentUpdated> for KlineSentimentUpdated {
    fn from(p: proto::KlineSentimentUpdated) -> Self {
        Self {
            market_index: p.market_index,
            period: p.period,
            open_at_logical: p.open_at_logical,
            long_short_ratio: p.long_short_ratio,
            long_oi: p.long_oi,
            short_oi: p.short_oi,
            proof: p.proof,
            outcome_id: p.outcome_id,
        }
    }
}

/// Epoch boundary processed (streaming).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KlineEpochBoundary {
    pub epoch_id: u64,
    pub archive_root: Vec<u8>,
}

impl From<proto::KlineEpochBoundary> for KlineEpochBoundary {
    fn from(p: proto::KlineEpochBoundary) -> Self {
        Self { epoch_id: p.epoch_id, archive_root: p.archive_root }
    }
}

/// Pruned data event for indexer (streaming).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KlinePruned {
    pub kind: String,
    pub pruned_count: u64,
    pub new_archive_root: Vec<u8>,
}

impl From<proto::KlinePruned> for KlinePruned {
    fn from(p: proto::KlinePruned) -> Self {
        Self { kind: p.kind, pruned_count: p.pruned_count, new_archive_root: p.new_archive_root }
    }
}

// ====================== QUERY RESPONSE TYPES ======================

/// Mark price with spread bands and quality score.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarkPriceWithSpread {
    pub mark: u64,
    pub spread: u64,
    pub lower_band: u64,
    pub upper_band: u64,
    pub quality: u64,
    pub proof: Vec<u8>,
}

/// Volume-weighted average price result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vwap {
    pub vwap: u64,
    pub total_volume: String,
    pub proof: Vec<u8>,
}

/// Long/short ratio result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LongShortRatio {
    pub ratio: u64,
    pub long_oi: String,
    pub short_oi: String,
    pub proof: Vec<u8>,
}

/// Last completed OHLC kline for a market/period.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LastKline {
    pub open: u64,
    pub high: u64,
    pub low: u64,
    pub close: u64,
    pub volume_base: String,
    pub proof: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn period_roundtrip() {
        for p in [Period::OneMinute, Period::FiveMinutes, Period::FifteenMinutes,
                  Period::OneHour, Period::FourHours, Period::OneDay] {
            let v: u32 = p.into();
            assert_eq!(Some(p), Period::from_u32(v));
        }
        assert_eq!(None, Period::from_u32(99));
    }

    #[test]
    fn period_interval_str() {
        assert_eq!(Period::OneMinute.as_interval_str(), "1m");
        assert_eq!(Period::FourHours.as_interval_str(), "4h");
        assert_eq!(Period::OneDay.as_interval_str(), "1d");
    }

    #[test]
    fn kline_data_from_proto() {
        let p = proto::KlineData {
            market_index: 1, period: 4, open_at_logical: 100,
            open: "50000".into(), high: "51000".into(), low: "49000".into(), close: "50500".into(),
            volume_base: "1000".into(), quote_volume: "500000".into(),
            trade_count: 42, taker_buy_base: "600".into(), taker_buy_quote: "300000".into(),
            spread_satoshi: 2000, mark_quality: 95_000_000, proof: vec![0u8; 32], outcome_id: 0,
        };
        let k: KlineData = p.into();
        assert_eq!(k.market_index, 1);
        assert_eq!(k.trade_count, 42);
        assert_eq!(k.proof.len(), 32);
    }

    #[test]
    fn trade_data_to_proto() {
        let t = TradeData {
            market_index: 1, price: 50000, quantity: "10".into(),
            is_taker_buy: true, block_height: 100, logical_timestamp: 200,
            feed_id: String::new(), outcome_id: 0,
        };
        let p: proto::TradeData = t.into();
        assert_eq!(p.price, 50000);
        assert!(p.is_taker_buy);
    }

    #[test]
    fn position_snapshot_to_proto() {
        let s = PositionSnapshot {
            market_index: 1, long_oi: "500".into(), short_oi: "300".into(),
            block_height: 100, logical_timestamp: 200,
        };
        let p: proto::PositionSnapshot = s.into();
        assert_eq!(p.long_oi, "500");
    }
}
