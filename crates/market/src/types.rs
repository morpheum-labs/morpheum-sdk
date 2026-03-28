//! Domain types for the Market module.
//!
//! Clean, idiomatic Rust representations of the market protobuf messages.
//! They provide type safety, ergonomic APIs, and full round-trip conversion to/from
//! protobuf while remaining strictly `no_std` compatible.
//!
//! The proto schema uses `oneof type_config` (for market-type-specific parameters)
//! and `oneof type_stats` (for market-type-specific statistics). The SDK mirrors this
//! with [`MarketTypeConfig`] and [`MarketTypeStats`] enums respectively.

use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::market::v1 as proto;

// ====================== ENUMS ======================

/// Market type enum — matches `market.v1.MarketType` proto enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MarketType {
    Unspecified,
    Spot,
    Perp,
    Future,
    Option,
    Prediction,
}

impl From<i32> for MarketType {
    fn from(v: i32) -> Self {
        match proto::MarketType::try_from(v).unwrap_or(proto::MarketType::Unspecified) {
            proto::MarketType::Unspecified => Self::Unspecified,
            proto::MarketType::Spot => Self::Spot,
            proto::MarketType::Perp => Self::Perp,
            proto::MarketType::Future => Self::Future,
            proto::MarketType::Option => Self::Option,
            proto::MarketType::Prediction => Self::Prediction,
        }
    }
}

impl From<MarketType> for i32 {
    fn from(t: MarketType) -> Self {
        match t {
            MarketType::Unspecified => proto::MarketType::Unspecified as i32,
            MarketType::Spot => proto::MarketType::Spot as i32,
            MarketType::Perp => proto::MarketType::Perp as i32,
            MarketType::Future => proto::MarketType::Future as i32,
            MarketType::Option => proto::MarketType::Option as i32,
            MarketType::Prediction => proto::MarketType::Prediction as i32,
        }
    }
}

/// Market status enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MarketStatus {
    Unknown,
    Pending,
    Active,
    Suspended,
    Inactive,
}

impl From<i32> for MarketStatus {
    fn from(v: i32) -> Self {
        match proto::MarketStatus::try_from(v).unwrap_or(proto::MarketStatus::Unknown) {
            proto::MarketStatus::Unknown => Self::Unknown,
            proto::MarketStatus::Pending => Self::Pending,
            proto::MarketStatus::Active => Self::Active,
            proto::MarketStatus::Suspended => Self::Suspended,
            proto::MarketStatus::Inactive => Self::Inactive,
        }
    }
}

impl From<MarketStatus> for i32 {
    fn from(s: MarketStatus) -> Self {
        match s {
            MarketStatus::Unknown => proto::MarketStatus::Unknown as i32,
            MarketStatus::Pending => proto::MarketStatus::Pending as i32,
            MarketStatus::Active => proto::MarketStatus::Active as i32,
            MarketStatus::Suspended => proto::MarketStatus::Suspended as i32,
            MarketStatus::Inactive => proto::MarketStatus::Inactive as i32,
        }
    }
}

/// Market category enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MarketCategory {
    Unspecified,
    Spot,
    Linear,
    Power,
}

impl From<i32> for MarketCategory {
    fn from(v: i32) -> Self {
        match proto::MarketCategory::try_from(v).unwrap_or(proto::MarketCategory::Unspecified) {
            proto::MarketCategory::Unspecified => Self::Unspecified,
            proto::MarketCategory::Spot => Self::Spot,
            proto::MarketCategory::Linear => Self::Linear,
            proto::MarketCategory::Power => Self::Power,
        }
    }
}

impl From<MarketCategory> for i32 {
    fn from(c: MarketCategory) -> Self {
        match c {
            MarketCategory::Unspecified => proto::MarketCategory::Unspecified as i32,
            MarketCategory::Spot => proto::MarketCategory::Spot as i32,
            MarketCategory::Linear => proto::MarketCategory::Linear as i32,
            MarketCategory::Power => proto::MarketCategory::Power as i32,
        }
    }
}

// ====================== TYPE-SPECIFIC CONFIGURATION ======================

/// PerpConfig — perpetual futures specific configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerpConfig {
    pub funding_method: String,
    pub max_funding_rate: String,
    pub base_funding_rate: String,
    pub funding_interval: i64,
    pub max_leverage: String,
}

impl From<proto::PerpConfig> for PerpConfig {
    fn from(p: proto::PerpConfig) -> Self {
        Self {
            funding_method: p.funding_method,
            max_funding_rate: p.max_funding_rate,
            base_funding_rate: p.base_funding_rate,
            funding_interval: p.funding_interval,
            max_leverage: p.max_leverage,
        }
    }
}

impl From<PerpConfig> for proto::PerpConfig {
    fn from(c: PerpConfig) -> Self {
        Self {
            funding_method: c.funding_method,
            max_funding_rate: c.max_funding_rate,
            base_funding_rate: c.base_funding_rate,
            funding_interval: c.funding_interval,
            max_leverage: c.max_leverage,
        }
    }
}

/// CLOB/order-book-specific trading parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClobMarketConfig {
    pub tick_size: String,
    pub lot_size: String,
    pub max_leverage: String,
    pub initial_margin_ratio: String,
    pub maintenance_margin_ratio: String,
    pub allow_market_orders: bool,
    pub allow_stop_orders: bool,
    pub perp_config: Option<PerpConfig>,
}

impl Default for ClobMarketConfig {
    fn default() -> Self {
        Self {
            tick_size: "0.01".into(),
            lot_size: "1".into(),
            max_leverage: "10".into(),
            initial_margin_ratio: "0.1".into(),
            maintenance_margin_ratio: "0.05".into(),
            allow_market_orders: true,
            allow_stop_orders: true,
            perp_config: None,
        }
    }
}

impl From<proto::ClobMarketConfig> for ClobMarketConfig {
    fn from(p: proto::ClobMarketConfig) -> Self {
        Self {
            tick_size: p.tick_size,
            lot_size: p.lot_size,
            max_leverage: p.max_leverage,
            initial_margin_ratio: p.initial_margin_ratio,
            maintenance_margin_ratio: p.maintenance_margin_ratio,
            allow_market_orders: p.allow_market_orders,
            allow_stop_orders: p.allow_stop_orders,
            perp_config: p.perp_config.map(Into::into),
        }
    }
}

impl From<ClobMarketConfig> for proto::ClobMarketConfig {
    fn from(c: ClobMarketConfig) -> Self {
        Self {
            tick_size: c.tick_size,
            lot_size: c.lot_size,
            max_leverage: c.max_leverage,
            initial_margin_ratio: c.initial_margin_ratio,
            maintenance_margin_ratio: c.maintenance_margin_ratio,
            allow_market_orders: c.allow_market_orders,
            allow_stop_orders: c.allow_stop_orders,
            perp_config: c.perp_config.map(Into::into),
        }
    }
}

/// Prediction market-specific parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PredictionMarketConfig {
    pub feed_id: String,
    pub num_outcomes: u32,
    pub spread_bps: u32,
}

impl From<proto::PredictionMarketConfig> for PredictionMarketConfig {
    fn from(p: proto::PredictionMarketConfig) -> Self {
        Self {
            feed_id: p.feed_id,
            num_outcomes: p.num_outcomes,
            spread_bps: p.spread_bps,
        }
    }
}

impl From<PredictionMarketConfig> for proto::PredictionMarketConfig {
    fn from(c: PredictionMarketConfig) -> Self {
        Self {
            feed_id: c.feed_id,
            num_outcomes: c.num_outcomes,
            spread_bps: c.spread_bps,
        }
    }
}

/// Market-type-specific configuration — mirrors the proto `oneof type_config`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MarketTypeConfig {
    Clob(ClobMarketConfig),
    Prediction(PredictionMarketConfig),
}

// ====================== MARKET PARAMS ======================

/// MarketParams — trading parameters for a market.
///
/// Universal fields (`min_order_size`, `taker_fee_rate`, `maker_fee_rate`) apply to
/// all market types. Market-type-specific configuration lives in [`type_config`](Self::type_config).
///
/// ```rust,ignore
/// let params = MarketParams::clob_default();
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarketParams {
    pub min_order_size: String,
    pub taker_fee_rate: String,
    pub maker_fee_rate: String,
    pub additional_params: BTreeMap<String, String>,
    pub type_config: Option<MarketTypeConfig>,
}

impl Default for MarketParams {
    fn default() -> Self {
        Self {
            min_order_size: "1".into(),
            taker_fee_rate: "0.001".into(),
            maker_fee_rate: "0.0005".into(),
            additional_params: BTreeMap::new(),
            type_config: None,
        }
    }
}

impl MarketParams {
    /// Returns CLOB market params with sensible defaults.
    pub fn clob_default() -> Self {
        Self {
            type_config: Some(MarketTypeConfig::Clob(ClobMarketConfig::default())),
            ..Self::default()
        }
    }
}

impl From<proto::MarketParams> for MarketParams {
    #[allow(deprecated)]
    fn from(p: proto::MarketParams) -> Self {
        let type_config = match p.type_config {
            Some(proto::market_params::TypeConfig::ClobConfig(c)) => {
                Some(MarketTypeConfig::Clob(c.into()))
            }
            Some(proto::market_params::TypeConfig::PredictionConfig(c)) => {
                Some(MarketTypeConfig::Prediction(c.into()))
            }
            None => {
                // Backward compat: reconstruct ClobMarketConfig from deprecated fields
                // if any non-default value is present.
                let has_legacy = !p.tick_size.is_empty()
                    || !p.lot_size.is_empty()
                    || !p.max_leverage.is_empty()
                    || !p.initial_margin_ratio.is_empty()
                    || !p.maintenance_margin_ratio.is_empty()
                    || p.allow_market_orders
                    || p.allow_stop_orders
                    || p.perp_config.is_some();

                if has_legacy {
                    Some(MarketTypeConfig::Clob(ClobMarketConfig {
                        tick_size: p.tick_size,
                        lot_size: p.lot_size,
                        max_leverage: p.max_leverage,
                        initial_margin_ratio: p.initial_margin_ratio,
                        maintenance_margin_ratio: p.maintenance_margin_ratio,
                        allow_market_orders: p.allow_market_orders,
                        allow_stop_orders: p.allow_stop_orders,
                        perp_config: p.perp_config.map(Into::into),
                    }))
                } else {
                    None
                }
            }
        };

        Self {
            min_order_size: p.min_order_size,
            taker_fee_rate: p.taker_fee_rate,
            maker_fee_rate: p.maker_fee_rate,
            additional_params: p.additional_params.into_iter().collect(),
            type_config,
        }
    }
}

impl From<MarketParams> for proto::MarketParams {
    #[allow(deprecated)]
    fn from(p: MarketParams) -> Self {
        let type_config = p.type_config.as_ref().map(|tc| match tc {
            MarketTypeConfig::Clob(c) => {
                proto::market_params::TypeConfig::ClobConfig(c.clone().into())
            }
            MarketTypeConfig::Prediction(c) => {
                proto::market_params::TypeConfig::PredictionConfig(c.clone().into())
            }
        });

        // Populate deprecated fields from ClobMarketConfig for wire compat.
        let (tick_size, lot_size, max_leverage, initial_margin_ratio, maintenance_margin_ratio,
            allow_market_orders, allow_stop_orders, perp_config) =
            match &p.type_config {
                Some(MarketTypeConfig::Clob(c)) => (
                    c.tick_size.clone(),
                    c.lot_size.clone(),
                    c.max_leverage.clone(),
                    c.initial_margin_ratio.clone(),
                    c.maintenance_margin_ratio.clone(),
                    c.allow_market_orders,
                    c.allow_stop_orders,
                    c.perp_config.clone().map(Into::into),
                ),
                _ => Default::default(),
            };

        Self {
            min_order_size: p.min_order_size,
            taker_fee_rate: p.taker_fee_rate,
            maker_fee_rate: p.maker_fee_rate,
            additional_params: p.additional_params.into_iter().collect(),
            type_config,
            tick_size,
            lot_size,
            max_leverage,
            initial_margin_ratio,
            maintenance_margin_ratio,
            allow_market_orders,
            allow_stop_orders,
            perp_config,
        }
    }
}

// ====================== TYPE-SPECIFIC STATISTICS ======================

/// CLOB/order-book-specific statistics.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClobStats {
    pub open_interest: String,
    pub best_bid: String,
    pub best_ask: String,
    pub spread: String,
}

impl From<proto::ClobStats> for ClobStats {
    fn from(p: proto::ClobStats) -> Self {
        Self {
            open_interest: p.open_interest,
            best_bid: p.best_bid,
            best_ask: p.best_ask,
            spread: p.spread,
        }
    }
}

impl From<ClobStats> for proto::ClobStats {
    fn from(s: ClobStats) -> Self {
        Self {
            open_interest: s.open_interest,
            best_bid: s.best_bid,
            best_ask: s.best_ask,
            spread: s.spread,
        }
    }
}

/// Prediction market-specific statistics.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PredictionStats {
    pub implied_probability: u32,
    pub total_pot_usdc: String,
    pub num_participants: u32,
}

impl From<proto::PredictionStats> for PredictionStats {
    fn from(p: proto::PredictionStats) -> Self {
        Self {
            implied_probability: p.implied_probability,
            total_pot_usdc: p.total_pot_usdc,
            num_participants: p.num_participants,
        }
    }
}

impl From<PredictionStats> for proto::PredictionStats {
    fn from(s: PredictionStats) -> Self {
        Self {
            implied_probability: s.implied_probability,
            total_pot_usdc: s.total_pot_usdc,
            num_participants: s.num_participants,
        }
    }
}

/// Market-type-specific statistics — mirrors the proto `oneof type_stats`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MarketTypeStats {
    Clob(ClobStats),
    Prediction(PredictionStats),
}

// ====================== MARKET ======================

/// Helper to safely extract seconds from an optional Timestamp.
fn timestamp_seconds(ts: Option<morpheum_proto::google::protobuf::Timestamp>) -> u64 {
    ts.map(|t| t.seconds as u64).unwrap_or(0)
}

/// Core Market definition.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Market {
    pub market_index: u64,
    pub unique_key: Vec<u8>,
    pub name: String,
    pub base_asset_index: u64,
    pub quote_asset_index: u64,
    pub market_type: MarketType,
    pub orderbook_type: String,
    pub status: MarketStatus,
    pub params: MarketParams,
    pub created_at: u64,
    pub activated_at: u64,
    pub total_volume_quote: String,
    pub mmf: u32,
    pub additional_metadata: BTreeMap<String, String>,
}

impl Market {
    /// Returns whether the market is currently active and tradable.
    pub fn is_active(&self) -> bool {
        matches!(self.status, MarketStatus::Active)
    }
}

impl From<proto::Market> for Market {
    fn from(p: proto::Market) -> Self {
        Self {
            market_index: p.market_index,
            unique_key: p.unique_key,
            name: p.name,
            base_asset_index: p.base_asset_index,
            quote_asset_index: p.quote_asset_index,
            market_type: MarketType::from(p.market_type),
            orderbook_type: p.orderbook_type,
            status: MarketStatus::from(p.status),
            params: p.params.map(Into::into).unwrap_or_default(),
            created_at: timestamp_seconds(p.created_at),
            activated_at: timestamp_seconds(p.activated_at),
            total_volume_quote: p.total_volume_quote,
            mmf: p.mmf,
            additional_metadata: p.additional_metadata.into_iter().collect(),
        }
    }
}

impl From<Market> for proto::Market {
    fn from(m: Market) -> Self {
        Self {
            market_index: m.market_index,
            unique_key: m.unique_key,
            name: m.name,
            base_asset_index: m.base_asset_index,
            quote_asset_index: m.quote_asset_index,
            market_type: i32::from(m.market_type),
            orderbook_type: m.orderbook_type,
            status: i32::from(m.status),
            params: Some(m.params.into()),
            created_at: Some(morpheum_proto::google::protobuf::Timestamp {
                seconds: m.created_at as i64,
                nanos: 0,
            }),
            activated_at: Some(morpheum_proto::google::protobuf::Timestamp {
                seconds: m.activated_at as i64,
                nanos: 0,
            }),
            total_volume_quote: m.total_volume_quote,
            mmf: m.mmf,
            additional_metadata: m.additional_metadata.into_iter().collect(),
        }
    }
}

// ====================== MARKET STATS ======================

/// Market statistics.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarketStats {
    pub market_index: u64,
    pub name: String,
    pub volume_quote: String,
    pub volume_base: String,
    pub high_price: String,
    pub low_price: String,
    pub last_price: String,
    pub price_change: String,
    pub price_change_percent: String,
    pub trade_count: i64,
    pub last_trade_at: u64,
    pub type_stats: Option<MarketTypeStats>,
}

impl From<proto::MarketStats> for MarketStats {
    #[allow(deprecated)]
    fn from(p: proto::MarketStats) -> Self {
        let type_stats = match p.type_stats {
            Some(proto::market_stats::TypeStats::ClobStats(s)) => {
                Some(MarketTypeStats::Clob(s.into()))
            }
            Some(proto::market_stats::TypeStats::PredictionStats(s)) => {
                Some(MarketTypeStats::Prediction(s.into()))
            }
            None if !p.open_interest.is_empty() => {
                Some(MarketTypeStats::Clob(ClobStats {
                    open_interest: p.open_interest,
                    best_bid: String::new(),
                    best_ask: String::new(),
                    spread: String::new(),
                }))
            }
            None => None,
        };

        Self {
            market_index: p.market_index,
            name: p.name,
            volume_quote: p.volume_quote,
            volume_base: p.volume_base,
            high_price: p.high_price,
            low_price: p.low_price,
            last_price: p.last_price,
            price_change: p.price_change,
            price_change_percent: p.price_change_percent,
            trade_count: p.trade_count,
            last_trade_at: timestamp_seconds(p.last_trade_at),
            type_stats,
        }
    }
}

// ====================== MARKET UPDATE ======================

/// Market update event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarketUpdate {
    pub market_index: u64,
    pub name: String,
    pub old_status: MarketStatus,
    pub new_status: MarketStatus,
    pub update_reason: String,
    pub new_params: Option<MarketParams>,
    pub timestamp: u64,
}

impl From<proto::MarketUpdate> for MarketUpdate {
    fn from(p: proto::MarketUpdate) -> Self {
        Self {
            market_index: p.market_index,
            name: p.name,
            old_status: MarketStatus::from(p.old_status),
            new_status: MarketStatus::from(p.new_status),
            update_reason: p.update_reason,
            new_params: p.new_params.map(Into::into),
            timestamp: timestamp_seconds(p.timestamp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn market_roundtrip() {
        let market = Market {
            market_index: 42,
            unique_key: vec![1, 2, 3],
            name: "BTC-USDC-PERP".into(),
            base_asset_index: 1,
            quote_asset_index: 2,
            market_type: MarketType::Perp,
            orderbook_type: "clob".into(),
            status: MarketStatus::Active,
            params: MarketParams {
                min_order_size: "0.001".into(),
                taker_fee_rate: "0.0005".into(),
                maker_fee_rate: "0.0002".into(),
                additional_params: BTreeMap::new(),
                type_config: Some(MarketTypeConfig::Clob(ClobMarketConfig {
                    tick_size: "0.01".into(),
                    lot_size: "1".into(),
                    max_leverage: "100".into(),
                    initial_margin_ratio: "0.1".into(),
                    maintenance_margin_ratio: "0.05".into(),
                    allow_market_orders: true,
                    allow_stop_orders: true,
                    perp_config: None,
                })),
            },
            created_at: 1_700_000_000,
            activated_at: 1_700_001_000,
            total_volume_quote: "1234567.89".into(),
            mmf: 500,
            additional_metadata: BTreeMap::new(),
        };

        let proto: proto::Market = market.clone().into();
        let back: Market = proto.into();

        assert_eq!(market, back);
    }

    #[test]
    fn market_is_active() {
        let mut market = Market {
            market_index: 1,
            unique_key: vec![],
            name: "TEST".into(),
            base_asset_index: 0,
            quote_asset_index: 0,
            market_type: MarketType::Spot,
            orderbook_type: "".into(),
            status: MarketStatus::Active,
            params: MarketParams::default(),
            created_at: 0,
            activated_at: 0,
            total_volume_quote: "".into(),
            mmf: 0,
            additional_metadata: BTreeMap::new(),
        };

        assert!(market.is_active());

        market.status = MarketStatus::Suspended;
        assert!(!market.is_active());
    }

    #[test]
    fn prediction_market_config_roundtrip() {
        let config = PredictionMarketConfig {
            feed_id: "pyth:btc-usd".into(),
            num_outcomes: 2,
            spread_bps: 50,
        };

        let proto_config: proto::PredictionMarketConfig = config.clone().into();
        let back: PredictionMarketConfig = proto_config.into();
        assert_eq!(config, back);
    }

    #[test]
    fn clob_default_params() {
        let params = MarketParams::clob_default();
        assert!(matches!(params.type_config, Some(MarketTypeConfig::Clob(_))));
    }

    #[test]
    fn legacy_proto_compat() {
        #[allow(deprecated)]
        let proto_params = proto::MarketParams {
            min_order_size: "0.1".into(),
            taker_fee_rate: "0.001".into(),
            maker_fee_rate: "0.0005".into(),
            additional_params: Default::default(),
            type_config: None,
            tick_size: "0.01".into(),
            lot_size: "1".into(),
            max_leverage: "10".into(),
            initial_margin_ratio: "0.1".into(),
            maintenance_margin_ratio: "0.05".into(),
            allow_market_orders: true,
            allow_stop_orders: false,
            perp_config: None,
        };

        let params: MarketParams = proto_params.into();
        match &params.type_config {
            Some(MarketTypeConfig::Clob(c)) => {
                assert_eq!(c.tick_size, "0.01");
                assert_eq!(c.max_leverage, "10");
                assert!(c.allow_market_orders);
                assert!(!c.allow_stop_orders);
            }
            _ => panic!("expected ClobMarketConfig from legacy fields"),
        }
    }
}
