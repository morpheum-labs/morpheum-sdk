//! Domain types for the Market module.
//!
//! These are clean, idiomatic Rust representations of the market protobuf messages.
//! They provide type safety, ergonomic APIs, and full round-trip conversion to/from
//! protobuf while remaining strictly `no_std` compatible.

use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::market::v1 as proto;

/// Market type enum.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MarketType {
    Unspecified,
    Spot,
    Perp,
    Future,
    Option,
    Custom,
}

impl From<i32> for MarketType {
    fn from(v: i32) -> Self {
        match proto::MarketType::try_from(v).unwrap_or(proto::MarketType::Unspecified) {
            proto::MarketType::Unspecified => Self::Unspecified,
            proto::MarketType::Spot => Self::Spot,
            proto::MarketType::Perp => Self::Perp,
            proto::MarketType::Future => Self::Future,
            proto::MarketType::Option => Self::Option,
            proto::MarketType::Custom => Self::Custom,
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
            MarketType::Custom => proto::MarketType::Custom as i32,
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

/// MarketParams — trading parameters for a market.
///
/// Provides sensible defaults via [`Default`] for common use cases:
/// - `tick_size`: `"0.01"`, `lot_size`: `"1"`, `min_order_size`: `"1"`
/// - `max_leverage`: `"10"`, margins: `10%` initial / `5%` maintenance
/// - `taker_fee_rate`: `"0.001"` (10 bps), `maker_fee_rate`: `"0.0005"` (5 bps)
/// - Both market and stop orders allowed
///
/// Override only the fields you need:
/// ```rust,ignore
/// let params = MarketParams {
///     tick_size: "0.001".into(),
///     min_order_size: "0.1".into(),
///     ..Default::default()
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarketParams {
    pub min_order_size: String,
    pub tick_size: String,
    pub lot_size: String,
    pub max_leverage: String,
    pub initial_margin_ratio: String,
    pub maintenance_margin_ratio: String,
    pub taker_fee_rate: String,
    pub maker_fee_rate: String,
    pub allow_market_orders: bool,
    pub allow_stop_orders: bool,
    pub perp_config: Option<PerpConfig>,
    pub additional_params: BTreeMap<String, String>,
}

impl Default for MarketParams {
    fn default() -> Self {
        Self {
            min_order_size: "1".into(),
            tick_size: "0.01".into(),
            lot_size: "1".into(),
            max_leverage: "10".into(),
            initial_margin_ratio: "0.1".into(),
            maintenance_margin_ratio: "0.05".into(),
            taker_fee_rate: "0.001".into(),
            maker_fee_rate: "0.0005".into(),
            allow_market_orders: true,
            allow_stop_orders: true,
            perp_config: None,
            additional_params: BTreeMap::new(),
        }
    }
}

impl From<proto::MarketParams> for MarketParams {
    fn from(p: proto::MarketParams) -> Self {
        Self {
            min_order_size: p.min_order_size,
            tick_size: p.tick_size,
            lot_size: p.lot_size,
            max_leverage: p.max_leverage,
            initial_margin_ratio: p.initial_margin_ratio,
            maintenance_margin_ratio: p.maintenance_margin_ratio,
            taker_fee_rate: p.taker_fee_rate,
            maker_fee_rate: p.maker_fee_rate,
            allow_market_orders: p.allow_market_orders,
            allow_stop_orders: p.allow_stop_orders,
            perp_config: p.perp_config.map(Into::into),
            additional_params: p.additional_params.into_iter().collect(),
        }
    }
}

impl From<MarketParams> for proto::MarketParams {
    fn from(p: MarketParams) -> Self {
        Self {
            min_order_size: p.min_order_size,
            tick_size: p.tick_size,
            lot_size: p.lot_size,
            max_leverage: p.max_leverage,
            initial_margin_ratio: p.initial_margin_ratio,
            maintenance_margin_ratio: p.maintenance_margin_ratio,
            taker_fee_rate: p.taker_fee_rate,
            maker_fee_rate: p.maker_fee_rate,
            allow_market_orders: p.allow_market_orders,
            allow_stop_orders: p.allow_stop_orders,
            perp_config: p.perp_config.map(Into::into),
            additional_params: p.additional_params.into_iter().collect(),
        }
    }
}

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
    pub created_at: u64,      // Unix seconds
    pub activated_at: u64,    // Unix seconds
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
            params: p.params.map(Into::into).unwrap_or_else(|| MarketParams {
                min_order_size: String::new(),
                tick_size: String::new(),
                lot_size: String::new(),
                max_leverage: String::new(),
                initial_margin_ratio: String::new(),
                maintenance_margin_ratio: String::new(),
                taker_fee_rate: String::new(),
                maker_fee_rate: String::new(),
                allow_market_orders: false,
                allow_stop_orders: false,
                perp_config: None,
                additional_params: BTreeMap::new(),
            }),
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
    pub open_interest: String,
    pub trade_count: i64,
    pub last_trade_at: u64,
}

impl From<proto::MarketStats> for MarketStats {
    fn from(p: proto::MarketStats) -> Self {
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
            open_interest: p.open_interest,
            trade_count: p.trade_count,
            last_trade_at: timestamp_seconds(p.last_trade_at),
        }
    }
}

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
                tick_size: "0.01".into(),
                lot_size: "1".into(),
                max_leverage: "100".into(),
                initial_margin_ratio: "0.1".into(),
                maintenance_margin_ratio: "0.05".into(),
                taker_fee_rate: "0.0005".into(),
                maker_fee_rate: "0.0002".into(),
                allow_market_orders: true,
                allow_stop_orders: true,
                perp_config: None,
                additional_params: BTreeMap::new(),
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
            params: MarketParams {
                min_order_size: "".into(),
                tick_size: "".into(),
                lot_size: "".into(),
                max_leverage: "".into(),
                initial_margin_ratio: "".into(),
                maintenance_margin_ratio: "".into(),
                taker_fee_rate: "".into(),
                maker_fee_rate: "".into(),
                allow_market_orders: true,
                allow_stop_orders: true,
                perp_config: None,
                additional_params: BTreeMap::new(),
            },
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
}
