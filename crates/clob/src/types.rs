//! Domain types for the CLOB module.
//!
//! Clean, idiomatic Rust representations of the CLOB protobuf messages
//! covering orders, trades, orderbook state, funding rates, mark prices,
//! tickers, and market-maker quotes.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::clob::v1 as proto;

// ====================== ENUMS ======================

/// Trade side — Buy or Sell. Maps to `primitives.v1.Side`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Side {
    Unspecified,
    Buy,
    Sell,
}

impl From<i32> for Side {
    fn from(v: i32) -> Self {
        match v { 1 => Self::Buy, 2 => Self::Sell, _ => Self::Unspecified }
    }
}

impl From<Side> for i32 {
    fn from(s: Side) -> Self {
        match s { Side::Unspecified => 0, Side::Buy => 1, Side::Sell => 2 }
    }
}

/// Order type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OrderType {
    Unspecified,
    Limit,
    Market,
    StopLimit,
    StopMarket,
    TakeProfitLimit,
    TakeProfitMarket,
}

impl From<i32> for OrderType {
    fn from(v: i32) -> Self {
        match proto::OrderType::try_from(v).unwrap_or(proto::OrderType::Unspecified) {
            proto::OrderType::Unspecified => Self::Unspecified,
            proto::OrderType::Limit => Self::Limit,
            proto::OrderType::Market => Self::Market,
            proto::OrderType::StopLimit => Self::StopLimit,
            proto::OrderType::StopMarket => Self::StopMarket,
            proto::OrderType::TakeProfitLimit => Self::TakeProfitLimit,
            proto::OrderType::TakeProfitMarket => Self::TakeProfitMarket,
        }
    }
}

impl From<OrderType> for i32 {
    fn from(t: OrderType) -> Self {
        match t {
            OrderType::Unspecified => proto::OrderType::Unspecified as i32,
            OrderType::Limit => proto::OrderType::Limit as i32,
            OrderType::Market => proto::OrderType::Market as i32,
            OrderType::StopLimit => proto::OrderType::StopLimit as i32,
            OrderType::StopMarket => proto::OrderType::StopMarket as i32,
            OrderType::TakeProfitLimit => proto::OrderType::TakeProfitLimit as i32,
            OrderType::TakeProfitMarket => proto::OrderType::TakeProfitMarket as i32,
        }
    }
}

/// Order status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum OrderStatus {
    Unspecified,
    Pending,
    Active,
    Filled,
    PartiallyFilled,
    Cancelled,
    Rejected,
}

impl From<i32> for OrderStatus {
    fn from(v: i32) -> Self {
        match proto::OrderStatus::try_from(v).unwrap_or(proto::OrderStatus::Unspecified) {
            proto::OrderStatus::Unspecified => Self::Unspecified,
            proto::OrderStatus::Pending => Self::Pending,
            proto::OrderStatus::Active => Self::Active,
            proto::OrderStatus::Filled => Self::Filled,
            proto::OrderStatus::PartiallyFilled => Self::PartiallyFilled,
            proto::OrderStatus::Cancelled => Self::Cancelled,
            proto::OrderStatus::Rejected => Self::Rejected,
        }
    }
}

impl From<OrderStatus> for i32 {
    fn from(s: OrderStatus) -> Self {
        match s {
            OrderStatus::Unspecified => proto::OrderStatus::Unspecified as i32,
            OrderStatus::Pending => proto::OrderStatus::Pending as i32,
            OrderStatus::Active => proto::OrderStatus::Active as i32,
            OrderStatus::Filled => proto::OrderStatus::Filled as i32,
            OrderStatus::PartiallyFilled => proto::OrderStatus::PartiallyFilled as i32,
            OrderStatus::Cancelled => proto::OrderStatus::Cancelled as i32,
            OrderStatus::Rejected => proto::OrderStatus::Rejected as i32,
        }
    }
}

/// Time-in-force options.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TimeInForce {
    Unspecified,
    Gtc,
    Gtt,
    Ioc,
    Fok,
    Day,
    PostOnly,
    ReduceOnly,
    CloseOnly,
    MakerOnly,
    TakerOnly,
}

impl From<i32> for TimeInForce {
    fn from(v: i32) -> Self {
        match proto::TimeInForce::try_from(v).unwrap_or(proto::TimeInForce::Unspecified) {
            proto::TimeInForce::Unspecified => Self::Unspecified,
            proto::TimeInForce::Gtc => Self::Gtc,
            proto::TimeInForce::Gtt => Self::Gtt,
            proto::TimeInForce::Ioc => Self::Ioc,
            proto::TimeInForce::Fok => Self::Fok,
            proto::TimeInForce::Day => Self::Day,
            proto::TimeInForce::PostOnly => Self::PostOnly,
            proto::TimeInForce::ReduceOnly => Self::ReduceOnly,
            proto::TimeInForce::CloseOnly => Self::CloseOnly,
            proto::TimeInForce::MakerOnly => Self::MakerOnly,
            proto::TimeInForce::TakerOnly => Self::TakerOnly,
        }
    }
}

impl From<TimeInForce> for i32 {
    fn from(t: TimeInForce) -> Self {
        match t {
            TimeInForce::Unspecified => proto::TimeInForce::Unspecified as i32,
            TimeInForce::Gtc => proto::TimeInForce::Gtc as i32,
            TimeInForce::Gtt => proto::TimeInForce::Gtt as i32,
            TimeInForce::Ioc => proto::TimeInForce::Ioc as i32,
            TimeInForce::Fok => proto::TimeInForce::Fok as i32,
            TimeInForce::Day => proto::TimeInForce::Day as i32,
            TimeInForce::PostOnly => proto::TimeInForce::PostOnly as i32,
            TimeInForce::ReduceOnly => proto::TimeInForce::ReduceOnly as i32,
            TimeInForce::CloseOnly => proto::TimeInForce::CloseOnly as i32,
            TimeInForce::MakerOnly => proto::TimeInForce::MakerOnly as i32,
            TimeInForce::TakerOnly => proto::TimeInForce::TakerOnly as i32,
        }
    }
}

// ====================== HELPER ======================

fn ts_secs(ts: Option<morpheum_proto::google::protobuf::Timestamp>) -> u64 {
    ts.map(|t| t.seconds as u64).unwrap_or(0)
}

// ====================== ORDER ======================

/// A CLOB order.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Order {
    pub order_id: String,
    pub address: String,
    pub side: Side,
    pub bucket_id: String,
    pub order_type: OrderType,
    pub category: i32,
    pub price: String,
    pub quantity: String,
    pub status: OrderStatus,
    pub power: f64,
    pub leverage: String,
    pub filled_quantity: String,
    pub average_price: String,
    pub take_profit: String,
    pub stop_loss: String,
    pub timestamp: u64,
    pub market_index: u64,
    pub time_in_force: TimeInForce,
    pub post_only: bool,
    pub hidden: bool,
    pub display_quantity: String,
    pub sequence_id: i64,
}

impl From<proto::Order> for Order {
    fn from(p: proto::Order) -> Self {
        Self {
            order_id: p.order_id,
            address: p.address,
            side: Side::from(p.side),
            bucket_id: p.bucket_id,
            order_type: OrderType::from(p.r#type),
            category: p.category,
            price: p.price,
            quantity: p.quantity,
            status: OrderStatus::from(p.status),
            power: p.power,
            leverage: p.leverage,
            filled_quantity: p.filled_quantity,
            average_price: p.average_price,
            take_profit: p.take_profit,
            stop_loss: p.stop_loss,
            timestamp: ts_secs(p.timestamp),
            market_index: p.market_index,
            time_in_force: TimeInForce::from(p.time_in_force),
            post_only: p.post_only,
            hidden: p.hidden,
            display_quantity: p.display_quantity,
            sequence_id: p.sequence_id,
        }
    }
}

// ====================== TRADE ======================

/// A trade execution.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Trade {
    pub trade_id: String,
    pub buy_order_id: String,
    pub sell_order_id: String,
    pub price: String,
    pub quantity: String,
    pub timestamp: u64,
    pub market_index: u64,
    pub maker_fee: String,
    pub taker_fee: String,
    pub buyer_address: String,
    pub seller_address: String,
    pub is_buyer_maker: bool,
    pub is_liquidation: bool,
    pub sequence_id: i64,
}

impl From<proto::Trade> for Trade {
    fn from(p: proto::Trade) -> Self {
        Self {
            trade_id: p.trade_id,
            buy_order_id: p.buy_order_id,
            sell_order_id: p.sell_order_id,
            price: p.price,
            quantity: p.quantity,
            timestamp: ts_secs(p.timestamp),
            market_index: p.market_index,
            maker_fee: p.maker_fee,
            taker_fee: p.taker_fee,
            buyer_address: p.buyer_address,
            seller_address: p.seller_address,
            is_buyer_maker: p.is_buyer_maker,
            is_liquidation: p.is_liquidation,
            sequence_id: p.sequence_id,
        }
    }
}

// ====================== ORDERBOOK ======================

/// A single price level in the orderbook.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OrderLevel {
    pub price: String,
    pub quantity: String,
    pub order_count: i32,
}

impl From<proto::OrderLevel> for OrderLevel {
    fn from(p: proto::OrderLevel) -> Self {
        Self { price: p.price, quantity: p.quantity, order_count: p.order_count }
    }
}

/// Orderbook snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OrderBookSnapshot {
    pub symbol: String,
    pub bids: Vec<PriceLevel>,
    pub asks: Vec<PriceLevel>,
    pub sequence_id: i64,
    pub checksum: String,
    pub block_height: u64,
    pub updated_at: u64,
}

/// A price level (used in orderbook snapshots and deltas).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PriceLevel {
    pub price: String,
    pub quantity: String,
}

// ====================== FUNDING RATE ======================

/// Funding rate data.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FundingRate {
    pub market_index: u64,
    pub funding_rate: String,
    pub twap_price: String,
    pub mark_price: String,
    pub index_price: String,
    pub calculation_time: u64,
    pub next_funding_time: u64,
    pub calculation_details: BTreeMap<String, String>,
}

/// Historical funding rate entry.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FundingRateEntry {
    pub timestamp: u64,
    pub rate: String,
    pub mark_price: String,
    pub index_price: String,
    pub next_funding_time: i64,
}

impl From<proto::FundingRateData> for FundingRateEntry {
    fn from(p: proto::FundingRateData) -> Self {
        Self {
            timestamp: ts_secs(p.timestamp),
            rate: p.rate,
            mark_price: p.mark_price,
            index_price: p.index_price,
            next_funding_time: p.next_funding_time,
        }
    }
}

// ====================== MARKET-MAKER QUOTE ======================

/// A market-maker quote linked to an AMM pool.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarketMakerQuote {
    pub quote_id: String,
    pub pool_id: String,
    pub market_index: u64,
    pub side: Side,
    pub price: String,
    pub amount: String,
    pub expiry: u64,
    pub status: String,
    pub provider: String,
    pub created_at: u64,
    pub external_address: Option<String>,
}

impl From<proto::MarketMakerQuote> for MarketMakerQuote {
    fn from(p: proto::MarketMakerQuote) -> Self {
        Self {
            quote_id: p.quote_id,
            pool_id: p.pool_id,
            market_index: p.market_index,
            side: Side::from(p.side),
            price: p.price,
            amount: p.amount,
            expiry: ts_secs(p.expiry),
            status: p.status,
            provider: p.provider,
            created_at: ts_secs(p.created_at),
            external_address: p.external_address,
        }
    }
}

// ====================== CHECKSUM ======================

/// Orderbook checksum for data integrity validation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OrderBookChecksum {
    pub market_index: u64,
    pub checksum: String,
    pub sequence_id: i64,
    pub timestamp: u64,
    pub block_height: u64,
    pub is_valid: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_type_roundtrip() {
        for ot in [OrderType::Limit, OrderType::Market, OrderType::StopLimit,
                    OrderType::StopMarket, OrderType::TakeProfitLimit, OrderType::TakeProfitMarket] {
            let v: i32 = ot.into();
            let back: OrderType = v.into();
            assert_eq!(ot, back);
        }
    }

    #[test]
    fn order_status_roundtrip() {
        for s in [OrderStatus::Pending, OrderStatus::Active, OrderStatus::Filled,
                   OrderStatus::PartiallyFilled, OrderStatus::Cancelled, OrderStatus::Rejected] {
            let v: i32 = s.into();
            let back: OrderStatus = v.into();
            assert_eq!(s, back);
        }
    }

    #[test]
    fn time_in_force_roundtrip() {
        for tif in [TimeInForce::Gtc, TimeInForce::Ioc, TimeInForce::Fok,
                     TimeInForce::Day, TimeInForce::PostOnly, TimeInForce::ReduceOnly] {
            let v: i32 = tif.into();
            let back: TimeInForce = v.into();
            assert_eq!(tif, back);
        }
    }

    #[test]
    fn side_roundtrip() {
        for s in [Side::Unspecified, Side::Buy, Side::Sell] {
            let v: i32 = s.into();
            let back: Side = v.into();
            assert_eq!(s, back);
        }
    }

    #[test]
    fn funding_rate_entry_from_proto() {
        let p = proto::FundingRateData {
            timestamp: Some(morpheum_proto::google::protobuf::Timestamp { seconds: 1_700_000_000, nanos: 0 }),
            rate: "0.0001".into(),
            mark_price: "50000".into(),
            index_price: "49950".into(),
            next_funding_time: 1_700_003_600,
        };
        let entry: FundingRateEntry = p.into();
        assert_eq!(entry.timestamp, 1_700_000_000);
        assert_eq!(entry.rate, "0.0001");
    }
}
