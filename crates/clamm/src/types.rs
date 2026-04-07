//! Domain types for the CLAMM module.
//!
//! Covers concentrated-liquidity positions, quotes, liquidity depth bands,
//! and events (swap, mint, burn, collect, ReClamm glide).

use alloc::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::generated::clamm::v1 as proto;

// ====================== ENUM ======================

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

// ====================== HELPER ======================

fn ts_secs(ts: Option<morpheum_proto::google::protobuf::Timestamp>) -> u64 {
    ts.map(|t| t.seconds as u64).unwrap_or(0)
}

// ====================== POSITION ======================

/// A concentrated-liquidity position.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClammPosition {
    pub position_id: String,
    pub address: String,
    pub pool_id: String,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity: String,
    pub fee_growth_inside_0_last: String,
    pub fee_growth_inside_1_last: String,
    pub tokens_owed_0: String,
    pub tokens_owed_1: String,
    pub amount: String,
    pub shares: String,
    pub pending_yield: String,
    pub deposit_time: u64,
    pub last_claim_time: u64,
    pub external_address: Option<String>,
}

impl From<proto::ClammPosition> for ClammPosition {
    fn from(p: proto::ClammPosition) -> Self {
        Self {
            position_id: p.position_id,
            address: p.address,
            pool_id: p.pool_id,
            tick_lower: p.tick_lower,
            tick_upper: p.tick_upper,
            liquidity: p.liquidity,
            fee_growth_inside_0_last: p.fee_growth_inside_0_last,
            fee_growth_inside_1_last: p.fee_growth_inside_1_last,
            tokens_owed_0: p.tokens_owed_0,
            tokens_owed_1: p.tokens_owed_1,
            amount: p.amount,
            shares: p.shares,
            pending_yield: p.pending_yield,
            deposit_time: ts_secs(p.deposit_time),
            last_claim_time: ts_secs(p.last_claim_time),
            external_address: p.external_address,
        }
    }
}

// ====================== QUOTE ======================

/// A market-maker quote (CLAMM-side; deprecated in favour of CLOB quotes).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClammQuote {
    pub quote_id: String,
    pub pool_id: String,
    pub market_index: u64,
    pub side: Side,
    pub price: String,
    pub amount: String,
    pub expiry: u64,
    pub status: String,
    pub created_at: u64,
}

impl From<proto::ClammQuote> for ClammQuote {
    fn from(p: proto::ClammQuote) -> Self {
        Self {
            quote_id: p.quote_id,
            pool_id: p.pool_id,
            market_index: p.market_index,
            side: Side::from(p.side),
            price: p.price,
            amount: p.amount,
            expiry: ts_secs(p.expiry),
            status: p.status,
            created_at: ts_secs(p.created_at),
        }
    }
}

// ====================== DEPTH ======================

/// A liquidity-depth band for a pool.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiquidityDepthBand {
    pub price_lower: String,
    pub price_upper: String,
    pub liquidity: String,
    pub volume_24h: String,
}

impl From<proto::LiquidityDepthBand> for LiquidityDepthBand {
    fn from(p: proto::LiquidityDepthBand) -> Self {
        Self {
            price_lower: p.price_lower, price_upper: p.price_upper,
            liquidity: p.liquidity, volume_24h: p.volume_24h,
        }
    }
}

// ====================== RESPONSE TYPES ======================

/// Swap simulation result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwapSimulation {
    pub amount_in: String,
    pub amount_out: String,
    pub fee_amount: String,
    pub price_impact: String,
}

/// AMM quote result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QuoteResult {
    pub amount_in: String,
    pub amount_out: String,
    pub fee_amount: String,
    pub price: String,
}

/// Pool risk summary.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PoolRiskSummary {
    pub pool_id: String,
    pub health_score_bps: String,
    pub utilization_rate: String,
    pub concentration_risk: String,
}

/// Boosted pool buffer state.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BoostedBuffer {
    pub pool_id: String,
    pub buffer_amount: String,
    pub pending_yield: String,
    pub apy_estimate: String,
}

/// ReClamm glide simulation result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GlideSimulation {
    pub current_virtual_price: String,
    pub projected_virtual_price: String,
    pub estimated_trades: String,
}

// ====================== FEE STATS ======================

/// Cumulative per-pool fee statistics.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PoolFeeStats {
    pub pool_id: String,
    pub total_protocol_fees: String,
    pub total_lp_fees: String,
    pub swap_count: u64,
}

impl From<proto::QueryPoolFeeStatsResponse> for PoolFeeStats {
    fn from(p: proto::QueryPoolFeeStatsResponse) -> Self {
        Self {
            pool_id: p.pool_id,
            total_protocol_fees: p.total_protocol_fees,
            total_lp_fees: p.total_lp_fees,
            swap_count: p.swap_count,
        }
    }
}

// ====================== EVENTS ======================

/// A swap executed on the CLAMM.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SwapExecuted {
    pub pool_id: String,
    pub market_index: u64,
    pub sender: String,
    pub side: Side,
    pub amount_in: String,
    pub amount_out: String,
    pub fee_amount: String,
    pub timestamp: u64,
}

impl From<proto::SwapExecuted> for SwapExecuted {
    fn from(p: proto::SwapExecuted) -> Self {
        Self {
            pool_id: p.pool_id, market_index: p.market_index, sender: p.sender,
            side: Side::from(p.side), amount_in: p.amount_in, amount_out: p.amount_out,
            fee_amount: p.fee_amount, timestamp: ts_secs(p.timestamp),
        }
    }
}

/// Liquidity mint event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MintEvent {
    pub pool_id: String,
    pub position_id: String,
    pub owner: String,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub liquidity: String,
    pub amount_a: String,
    pub amount_b: String,
    pub timestamp: u64,
}

impl From<proto::MintEvent> for MintEvent {
    fn from(p: proto::MintEvent) -> Self {
        Self {
            pool_id: p.pool_id, position_id: p.position_id, owner: p.owner,
            tick_lower: p.tick_lower, tick_upper: p.tick_upper, liquidity: p.liquidity,
            amount_a: p.amount_a, amount_b: p.amount_b, timestamp: ts_secs(p.timestamp),
        }
    }
}

/// Liquidity burn event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BurnEvent {
    pub pool_id: String,
    pub position_id: String,
    pub owner: String,
    pub liquidity: String,
    pub amount_a: String,
    pub amount_b: String,
    pub timestamp: u64,
}

impl From<proto::BurnEvent> for BurnEvent {
    fn from(p: proto::BurnEvent) -> Self {
        Self {
            pool_id: p.pool_id, position_id: p.position_id, owner: p.owner,
            liquidity: p.liquidity, amount_a: p.amount_a, amount_b: p.amount_b,
            timestamp: ts_secs(p.timestamp),
        }
    }
}

/// Fee collect event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CollectEvent {
    pub pool_id: String,
    pub position_id: String,
    pub owner: String,
    pub amount_0: String,
    pub amount_1: String,
    pub timestamp: u64,
}

impl From<proto::CollectEvent> for CollectEvent {
    fn from(p: proto::CollectEvent) -> Self {
        Self {
            pool_id: p.pool_id, position_id: p.position_id, owner: p.owner,
            amount_0: p.amount_0, amount_1: p.amount_1, timestamp: ts_secs(p.timestamp),
        }
    }
}

/// ReClamm glide update event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReClammGlideUpdated {
    pub pool_id: String,
    pub virtual_price: String,
    pub glide_target: String,
    pub glide_speed: String,
    pub timestamp: u64,
}

impl From<proto::ReClammGlideUpdated> for ReClammGlideUpdated {
    fn from(p: proto::ReClammGlideUpdated) -> Self {
        Self {
            pool_id: p.pool_id, virtual_price: p.virtual_price,
            glide_target: p.glide_target, glide_speed: p.glide_speed,
            timestamp: ts_secs(p.timestamp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn side_roundtrip() {
        for s in [Side::Unspecified, Side::Buy, Side::Sell] {
            let v: i32 = s.into();
            assert_eq!(s, Side::from(v));
        }
    }

    #[test]
    fn clamm_position_from_proto() {
        let p = proto::ClammPosition {
            position_id: "pos-1".into(),
            address: "morpheum1abc".into(),
            pool_id: "0x1234".into(),
            tick_lower: -1000,
            tick_upper: 1000,
            liquidity: "500000".into(),
            deposit_time: Some(morpheum_proto::google::protobuf::Timestamp { seconds: 1_700_000_000, nanos: 0 }),
            ..Default::default()
        };
        let pos: ClammPosition = p.into();
        assert_eq!(pos.position_id, "pos-1");
        assert_eq!(pos.tick_lower, -1000);
        assert_eq!(pos.tick_upper, 1000);
        assert_eq!(pos.deposit_time, 1_700_000_000);
    }

    #[test]
    fn depth_band_from_proto() {
        let p = proto::LiquidityDepthBand {
            price_lower: "49000".into(), price_upper: "51000".into(),
            liquidity: "1000000".into(), volume_24h: "500000".into(),
        };
        let band: LiquidityDepthBand = p.into();
        assert_eq!(band.price_lower, "49000");
        assert_eq!(band.liquidity, "1000000");
    }
}
