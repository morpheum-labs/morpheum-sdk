//! Domain types for the Bucket module.
//!
//! Clean, idiomatic Rust representations of the bucket and related protobuf
//! messages. They provide type safety, ergonomic APIs, and full round-trip
//! conversion to/from protobuf while remaining strictly `no_std` compatible.
//!
//! Position-related types (`Position`, `PositionPnLInfo`) are defined here
//! because they appear in bucket query responses. They mirror the client-facing
//! `position.v1.Position` proto (string-heavy) rather than the keeper-level
//! `position.v1.PositionState`.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::bucket::v1 as proto;

// ====================== ENUMS ======================

/// Bucket type — isolated (1 position) or cross (shared margin).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BucketType {
    Unspecified,
    Isolated,
    Cross,
}

impl From<i32> for BucketType {
    fn from(v: i32) -> Self {
        match proto::BucketType::try_from(v).unwrap_or(proto::BucketType::Unspecified) {
            proto::BucketType::Unspecified => Self::Unspecified,
            proto::BucketType::Isolated => Self::Isolated,
            proto::BucketType::Cross => Self::Cross,
        }
    }
}

impl From<BucketType> for i32 {
    fn from(t: BucketType) -> Self {
        match t {
            BucketType::Unspecified => proto::BucketType::Unspecified as i32,
            BucketType::Isolated => proto::BucketType::Isolated as i32,
            BucketType::Cross => proto::BucketType::Cross as i32,
        }
    }
}

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
        match v {
            1 => Self::Buy,
            2 => Self::Sell,
            _ => Self::Unspecified,
        }
    }
}

impl From<Side> for i32 {
    fn from(s: Side) -> Self {
        match s {
            Side::Unspecified => 0,
            Side::Buy => 1,
            Side::Sell => 2,
        }
    }
}

// ====================== HELPER ======================

fn timestamp_seconds(ts: Option<morpheum_proto::google::protobuf::Timestamp>) -> u64 {
    ts.map(|t| t.seconds as u64).unwrap_or(0)
}

// ====================== BUCKET ======================

/// A margin bucket containing positions for perpetual trading.
///
/// All monetary values are u256 satoshi-format strings (1e8 precision).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Bucket {
    pub bucket_id: String,
    pub address: String,
    pub bucket_type: BucketType,
    pub collateral_asset_index: u64,
    pub deposited_margin: String,
    pub total_equity: String,
    pub available_margin: String,
    pub used_margin: String,
    pub unrealized_profit: String,
    pub unrealized_loss: String,
    pub realized_profit: String,
    pub realized_loss: String,
    pub margin_ratio: String,
    pub imr: String,
    pub total_notional_value: String,
    pub risk_level: String,
    pub max_leverage: i64,
    pub correlation_offset: u32,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub sequence_id: i64,
}

impl From<proto::Bucket> for Bucket {
    fn from(p: proto::Bucket) -> Self {
        Self {
            bucket_id: p.bucket_id,
            address: p.address,
            bucket_type: BucketType::from(p.bucket_type),
            collateral_asset_index: p.collateral_asset_index,
            deposited_margin: p.deposited_margin,
            total_equity: p.total_equity,
            available_margin: p.available_margin,
            used_margin: p.used_margin,
            unrealized_profit: p.unrealized_profit,
            unrealized_loss: p.unrealized_loss,
            realized_profit: p.realized_profit,
            realized_loss: p.realized_loss,
            margin_ratio: p.margin_ratio,
            imr: p.imr,
            total_notional_value: p.total_notional_value,
            risk_level: p.risk_level,
            max_leverage: p.max_leverage,
            correlation_offset: p.correlation_offset,
            status: p.status,
            created_at: timestamp_seconds(p.created_at),
            updated_at: timestamp_seconds(p.updated_at),
            sequence_id: p.sequence_id,
        }
    }
}

impl From<Bucket> for proto::Bucket {
    fn from(b: Bucket) -> Self {
        Self {
            bucket_id: b.bucket_id,
            address: b.address,
            bucket_type: i32::from(b.bucket_type),
            collateral_asset_index: b.collateral_asset_index,
            deposited_margin: b.deposited_margin,
            total_equity: b.total_equity,
            available_margin: b.available_margin,
            used_margin: b.used_margin,
            unrealized_profit: b.unrealized_profit,
            unrealized_loss: b.unrealized_loss,
            realized_profit: b.realized_profit,
            realized_loss: b.realized_loss,
            margin_ratio: b.margin_ratio,
            imr: b.imr,
            total_notional_value: b.total_notional_value,
            risk_level: b.risk_level,
            max_leverage: b.max_leverage,
            correlation_offset: b.correlation_offset,
            status: b.status,
            created_at: Some(morpheum_proto::google::protobuf::Timestamp {
                seconds: b.created_at as i64,
                nanos: 0,
            }),
            updated_at: Some(morpheum_proto::google::protobuf::Timestamp {
                seconds: b.updated_at as i64,
                nanos: 0,
            }),
            sequence_id: b.sequence_id,
        }
    }
}

// ====================== BUCKET PNL INFO ======================

/// Summary PnL for a single bucket.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BucketPnLInfo {
    pub bucket_id: String,
    pub unrealized_profit: String,
    pub unrealized_loss: String,
    pub realized_profit: String,
    pub realized_loss: String,
    pub net_profit: String,
    pub net_loss: String,
    pub position_count: i32,
}

impl From<proto::BucketPnLInfo> for BucketPnLInfo {
    fn from(p: proto::BucketPnLInfo) -> Self {
        Self {
            bucket_id: p.bucket_id,
            unrealized_profit: p.unrealized_profit,
            unrealized_loss: p.unrealized_loss,
            realized_profit: p.realized_profit,
            realized_loss: p.realized_loss,
            net_profit: p.net_profit,
            net_loss: p.net_loss,
            position_count: p.position_count,
        }
    }
}

// ====================== POSITION (client-facing) ======================

/// Client-facing position representation (from `position.v1.Position`).
///
/// This is the rich, string-heavy type returned in bucket query responses,
/// distinct from the keeper-level `PositionState` in the position module.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Position {
    pub position_id: String,
    pub bucket_id: String,
    pub position_type: String,
    pub market_index: u64,
    pub address: String,
    pub symbol: String,
    pub size: String,
    pub side: Side,
    pub average_entry_price: String,
    pub mark_price: String,
    pub leverage: String,
    pub unrealized_profit: String,
    pub unrealized_loss: String,
    pub maintenance_margin_requirement: String,
    pub margin_balance: String,
    pub status: String,
    pub created_at: u64,
    pub updated_at: u64,
    pub liquidation_price: String,
    pub market_type: String,
    pub realized_profit: String,
    pub realized_loss: String,
    pub position_side: String,
    pub margin_ratio: String,
    pub timestamp: u64,
    pub update_reason: String,
    pub sequence_id: i64,
}

impl From<morpheum_proto::position::v1::Position> for Position {
    fn from(p: morpheum_proto::position::v1::Position) -> Self {
        Self {
            position_id: p.position_id,
            bucket_id: p.bucket_id,
            position_type: p.r#type,
            market_index: p.market_index,
            address: p.address,
            symbol: p.symbol,
            size: p.size,
            side: Side::from(p.side),
            average_entry_price: p.average_entry_price,
            mark_price: p.mark_price,
            leverage: p.leverage,
            unrealized_profit: p.unrealized_profit,
            unrealized_loss: p.unrealized_loss,
            maintenance_margin_requirement: p.maintenance_margin_requirement,
            margin_balance: p.margin_balance,
            status: p.status,
            created_at: timestamp_seconds(p.created_at),
            updated_at: timestamp_seconds(p.updated_at),
            liquidation_price: p.liquidation_price,
            market_type: p.market_type,
            realized_profit: p.realized_profit,
            realized_loss: p.realized_loss,
            position_side: p.position_side,
            margin_ratio: p.margin_ratio,
            timestamp: timestamp_seconds(p.timestamp),
            update_reason: p.update_reason,
            sequence_id: p.sequence_id,
        }
    }
}

// ====================== POSITION PNL INFO ======================

/// PnL details for a single position.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PositionPnLInfo {
    pub position_id: String,
    pub market_index: u64,
    pub unrealized_profit: String,
    pub unrealized_loss: String,
    pub realized_profit: String,
    pub realized_loss: String,
    pub net_profit: String,
    pub net_loss: String,
    pub average_entry_price: String,
    pub mark_price: String,
    pub size: String,
    pub side: Side,
    pub leverage: String,
}

impl From<morpheum_proto::position::v1::PositionPnLInfo> for PositionPnLInfo {
    fn from(p: morpheum_proto::position::v1::PositionPnLInfo) -> Self {
        Self {
            position_id: p.position_id,
            market_index: p.market_index,
            unrealized_profit: p.unrealized_profit,
            unrealized_loss: p.unrealized_loss,
            realized_profit: p.realized_profit,
            realized_loss: p.realized_loss,
            net_profit: p.net_profit,
            net_loss: p.net_loss,
            average_entry_price: p.average_entry_price,
            mark_price: p.mark_price,
            size: p.size,
            side: Side::from(p.side),
            leverage: p.leverage,
        }
    }
}

// ====================== BUCKET BALANCE ======================

/// Balance summary for a single bucket, including per-position PnL breakdown.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BucketBalance {
    pub bucket_id: String,
    pub dedicated_margin: String,
    pub total_equity: String,
    pub maintenance_margin_requirement: String,
    pub available_balance: String,
    pub risk_factor: String,
    pub position_count: i32,
    pub position_pnl_infos: Vec<PositionPnLInfo>,
}

impl From<proto::BucketBalance> for BucketBalance {
    fn from(p: proto::BucketBalance) -> Self {
        Self {
            bucket_id: p.bucket_id,
            dedicated_margin: p.dedicated_margin,
            total_equity: p.total_equity,
            maintenance_margin_requirement: p.maintenance_margin_requirement,
            available_balance: p.available_balance,
            risk_factor: p.risk_factor,
            position_count: p.position_count,
            position_pnl_infos: p.position_pnl_infos.into_iter().map(Into::into).collect(),
        }
    }
}

// ====================== BUCKET HEALTH SUMMARY ======================

/// Per-address bucket health snapshot for client display.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BucketHealthSummary {
    pub address: String,
    pub total_margin_ratio: String,
    pub total_equity: String,
    pub total_maintenance_margin: String,
    pub risk_level: String,
    pub positions: Vec<Position>,
    pub liquidation_required: bool,
    pub warning_message: String,
}

impl From<proto::BucketHealthSummary> for BucketHealthSummary {
    fn from(p: proto::BucketHealthSummary) -> Self {
        Self {
            address: p.address,
            total_margin_ratio: p.total_margin_ratio,
            total_equity: p.total_equity,
            total_maintenance_margin: p.total_maintenance_margin,
            risk_level: p.risk_level,
            positions: p.positions.into_iter().map(Into::into).collect(),
            liquidation_required: p.liquidation_required,
            warning_message: p.warning_message,
        }
    }
}

// ====================== LIQUIDATION ======================

/// A completed liquidation record.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Liquidation {
    pub liquidation_id: String,
    pub address: String,
    pub base_asset_index: u64,
    pub quote_asset_index: u64,
    pub liquidated_size: String,
    pub liquidation_price: String,
    pub penalty: String,
    pub trigger_timestamp: u64,
}

impl From<proto::Liquidation> for Liquidation {
    fn from(p: proto::Liquidation) -> Self {
        Self {
            liquidation_id: p.liquidation_id,
            address: p.address,
            base_asset_index: p.base_asset_index,
            quote_asset_index: p.quote_asset_index,
            liquidated_size: p.liquidated_size,
            liquidation_price: p.liquidation_price,
            penalty: p.penalty,
            trigger_timestamp: timestamp_seconds(p.trigger_timestamp),
        }
    }
}

// ====================== LIQUIDATION EVENT ======================

/// A liquidation event with status tracking.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiquidationEvent {
    pub liquidation_id: String,
    pub address: String,
    pub market_index: u64,
    pub side: String,
    pub size: String,
    pub liquidation_price: String,
    pub mark_price: String,
    pub loss: String,
    pub timestamp: u64,
    pub status: String,
}

impl From<proto::LiquidationEvent> for LiquidationEvent {
    fn from(p: proto::LiquidationEvent) -> Self {
        Self {
            liquidation_id: p.liquidation_id,
            address: p.address,
            market_index: p.market_index,
            side: p.side,
            size: p.size,
            liquidation_price: p.liquidation_price,
            mark_price: p.mark_price,
            loss: p.loss,
            timestamp: timestamp_seconds(p.timestamp),
            status: p.status,
        }
    }
}

// ====================== QUERY RESPONSE VALUE TYPES ======================

/// Aggregated PnL across all buckets for an address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AddressPnL {
    pub address: String,
    pub unrealized_profit: String,
    pub unrealized_loss: String,
    pub realized_profit: String,
    pub realized_loss: String,
    pub net_profit: String,
    pub net_loss: String,
    pub buckets: Vec<BucketPnLInfo>,
    pub calculated_at: u64,
}

/// PnL for a specific bucket with position-level breakdown.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BucketPnL {
    pub bucket_id: String,
    pub address: String,
    pub unrealized_profit: String,
    pub unrealized_loss: String,
    pub realized_profit: String,
    pub realized_loss: String,
    pub net_profit: String,
    pub net_loss: String,
    pub volatility_factor: String,
    pub adjusted_profit: String,
    pub adjusted_loss: String,
    pub position_pnl_infos: Vec<PositionPnLInfo>,
    pub calculated_at: u64,
}

/// PnL for a single position.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PositionPnL {
    pub unrealized_profit: String,
    pub unrealized_loss: String,
    pub realized_profit: String,
    pub realized_loss: String,
    pub net_profit: String,
    pub net_loss: String,
}

/// Bucket status check result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BucketStatus {
    pub exists: bool,
    pub available_balance: String,
    pub risk_factor: String,
}

/// Position health check result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PositionHealth {
    pub liquidation_required: bool,
    pub margin_ratio: String,
    pub unrealized_profit: String,
    pub unrealized_loss: String,
}

/// Liquidation metrics over a time range.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiquidationMetrics {
    pub total_liquidations: i32,
    pub total_warnings: i32,
    pub avg_processing_time_ms: String,
    pub max_processing_time_ms: String,
    pub liquidations_by_market: BTreeMap<String, i32>,
    pub liquidations_by_algorithm: BTreeMap<String, i32>,
}

/// Aggregated balance across all buckets for an address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AllBucketsBalance {
    pub balance: String,
    pub bucket_balances: Vec<BucketBalance>,
    pub total_balance: String,
}

// ====================== BUCKET FEE STATS ======================

/// Aggregate fee statistics for the bucket module.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BucketFeeStats {
    pub total_deposit_fees: u64,
    pub total_liquidation_penalties: u64,
    pub total_treasury_swept: u64,
    pub total_insurance_contributions: u64,
    pub deposit_fee_count: u64,
    pub liquidation_count: u64,
}

impl From<proto::QueryBucketFeeStatsResponse> for BucketFeeStats {
    fn from(p: proto::QueryBucketFeeStatsResponse) -> Self {
        Self {
            total_deposit_fees: p.total_deposit_fees,
            total_liquidation_penalties: p.total_liquidation_penalties,
            total_treasury_swept: p.total_treasury_swept,
            total_insurance_contributions: p.total_insurance_contributions,
            deposit_fee_count: p.deposit_fee_count,
            liquidation_count: p.liquidation_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bucket_type_roundtrip() {
        for bt in [BucketType::Unspecified, BucketType::Isolated, BucketType::Cross] {
            let proto_val: i32 = bt.into();
            let back: BucketType = proto_val.into();
            assert_eq!(bt, back);
        }
    }

    #[test]
    fn side_roundtrip() {
        for side in [Side::Unspecified, Side::Buy, Side::Sell] {
            let proto_val: i32 = side.into();
            let back: Side = proto_val.into();
            assert_eq!(side, back);
        }
    }

    #[test]
    fn bucket_roundtrip() {
        let bucket = Bucket {
            bucket_id: "bucket-1".into(),
            address: "morpheum1abc".into(),
            bucket_type: BucketType::Cross,
            collateral_asset_index: 4,
            deposited_margin: "100000000000".into(),
            total_equity: "110000000000".into(),
            available_margin: "50000000000".into(),
            used_margin: "60000000000".into(),
            unrealized_profit: "10000000000".into(),
            unrealized_loss: "0".into(),
            realized_profit: "5000000000".into(),
            realized_loss: "1000000000".into(),
            margin_ratio: "18333".into(),
            imr: "10000".into(),
            total_notional_value: "600000000000".into(),
            risk_level: "low".into(),
            max_leverage: 100,
            correlation_offset: 30,
            status: "active".into(),
            created_at: 1_700_000_000,
            updated_at: 1_700_001_000,
            sequence_id: 42,
        };

        let proto_bucket: proto::Bucket = bucket.clone().into();
        let back: Bucket = proto_bucket.into();
        assert_eq!(bucket, back);
    }

    #[test]
    fn bucket_pnl_info_from_proto() {
        let proto_info = proto::BucketPnLInfo {
            bucket_id: "b1".into(),
            unrealized_profit: "100".into(),
            unrealized_loss: "50".into(),
            realized_profit: "200".into(),
            realized_loss: "75".into(),
            net_profit: "300".into(),
            net_loss: "125".into(),
            position_count: 3,
        };

        let info: BucketPnLInfo = proto_info.into();
        assert_eq!(info.bucket_id, "b1");
        assert_eq!(info.position_count, 3);
    }

    #[test]
    fn liquidation_event_from_proto() {
        let proto_event = proto::LiquidationEvent {
            liquidation_id: "liq-1".into(),
            address: "morpheum1abc".into(),
            market_index: 42,
            side: "long".into(),
            size: "1000".into(),
            liquidation_price: "49000".into(),
            mark_price: "48900".into(),
            loss: "100".into(),
            timestamp: Some(morpheum_proto::google::protobuf::Timestamp {
                seconds: 1_700_000_000,
                nanos: 0,
            }),
            status: "completed".into(),
        };

        let event: LiquidationEvent = proto_event.into();
        assert_eq!(event.liquidation_id, "liq-1");
        assert_eq!(event.market_index, 42);
        assert_eq!(event.timestamp, 1_700_000_000);
    }
}
