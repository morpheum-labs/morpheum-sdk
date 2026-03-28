//! Request wrappers for the CLAMM module.
//!
//! Transaction requests include `to_any()` for `TxBuilder` integration.
//! Query requests convert to proto via `From` impls.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::generated::clamm::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::Side;

// ====================== TRANSACTION REQUESTS ======================

/// Request to add concentrated liquidity to a pool.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AddLiquidityRequest {
    pub pool_id: String,
    pub owner: String,
    pub tick_lower: i32,
    pub tick_upper: i32,
    pub amount_desired_a: String,
    pub amount_desired_b: String,
    pub external_address: Option<String>,
}

impl AddLiquidityRequest {
    pub fn new(
        pool_id: impl Into<String>, owner: impl Into<String>,
        tick_lower: i32, tick_upper: i32,
        amount_a: impl Into<String>, amount_b: impl Into<String>,
    ) -> Self {
        Self {
            pool_id: pool_id.into(), owner: owner.into(),
            tick_lower, tick_upper,
            amount_desired_a: amount_a.into(), amount_desired_b: amount_b.into(),
            external_address: None,
        }
    }

    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into()); self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::AddLiquidityRequest = self.clone().into();
        ProtoAny { type_url: "/clamm.v1.AddLiquidityRequest".into(), value: msg.encode_to_vec() }
    }
}

impl From<AddLiquidityRequest> for proto::AddLiquidityRequest {
    fn from(r: AddLiquidityRequest) -> Self {
        Self {
            pool_id: r.pool_id, owner: r.owner,
            tick_lower: r.tick_lower, tick_upper: r.tick_upper,
            amount_desired_a: r.amount_desired_a, amount_desired_b: r.amount_desired_b,
            timestamp: None, external_address: r.external_address,
        }
    }
}

/// Request to remove liquidity from a position.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RemoveLiquidityRequest {
    pub position_id: String,
    pub liquidity_amount: String,
    pub min_amount_a: Option<String>,
    pub min_amount_b: Option<String>,
}

impl RemoveLiquidityRequest {
    pub fn new(position_id: impl Into<String>, liquidity_amount: impl Into<String>) -> Self {
        Self {
            position_id: position_id.into(), liquidity_amount: liquidity_amount.into(),
            min_amount_a: None, min_amount_b: None,
        }
    }

    pub fn min_amounts(mut self, a: impl Into<String>, b: impl Into<String>) -> Self {
        self.min_amount_a = Some(a.into()); self.min_amount_b = Some(b.into()); self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::RemoveLiquidityRequest = self.clone().into();
        ProtoAny { type_url: "/clamm.v1.RemoveLiquidityRequest".into(), value: msg.encode_to_vec() }
    }
}

impl From<RemoveLiquidityRequest> for proto::RemoveLiquidityRequest {
    fn from(r: RemoveLiquidityRequest) -> Self {
        Self {
            position_id: r.position_id, liquidity_amount: r.liquidity_amount,
            min_amount_a: r.min_amount_a.unwrap_or_default(),
            min_amount_b: r.min_amount_b.unwrap_or_default(),
            timestamp: None,
        }
    }
}

/// Request to collect accrued fees from a position.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CollectFeesRequest {
    pub position_id: String,
}

impl CollectFeesRequest {
    pub fn new(position_id: impl Into<String>) -> Self { Self { position_id: position_id.into() } }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::CollectFeesRequest { position_id: self.position_id.clone(), timestamp: None };
        ProtoAny { type_url: "/clamm.v1.CollectFeesRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Request to claim yield from a pool.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClaimYieldRequest {
    pub address: String,
    pub pool_id: String,
    pub external_address: Option<String>,
}

impl ClaimYieldRequest {
    pub fn new(address: impl Into<String>, pool_id: impl Into<String>) -> Self {
        Self { address: address.into(), pool_id: pool_id.into(), external_address: None }
    }

    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into()); self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::ClaimYieldRequest = self.clone().into();
        ProtoAny { type_url: "/clamm.v1.ClaimYieldRequest".into(), value: msg.encode_to_vec() }
    }
}

impl From<ClaimYieldRequest> for proto::ClaimYieldRequest {
    fn from(r: ClaimYieldRequest) -> Self {
        Self { address: r.address, pool_id: r.pool_id, timestamp: None, external_address: r.external_address }
    }
}

/// Request to claim boosted yield from a pool.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClaimBoostedYieldRequest {
    pub address: String,
    pub pool_id: String,
    pub external_address: Option<String>,
}

impl ClaimBoostedYieldRequest {
    pub fn new(address: impl Into<String>, pool_id: impl Into<String>) -> Self {
        Self { address: address.into(), pool_id: pool_id.into(), external_address: None }
    }

    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into()); self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::ClaimBoostedYieldRequest = self.clone().into();
        ProtoAny { type_url: "/clamm.v1.ClaimBoostedYieldRequest".into(), value: msg.encode_to_vec() }
    }
}

impl From<ClaimBoostedYieldRequest> for proto::ClaimBoostedYieldRequest {
    fn from(r: ClaimBoostedYieldRequest) -> Self {
        Self { address: r.address, pool_id: r.pool_id, timestamp: None, external_address: r.external_address }
    }
}

/// Request to force a ReClamm glide (governance only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForceGlideRequest {
    pub pool_id: String,
    pub target_price: String,
    pub authority: Option<String>,
}

impl ForceGlideRequest {
    pub fn new(pool_id: impl Into<String>, target_price: impl Into<String>) -> Self {
        Self { pool_id: pool_id.into(), target_price: target_price.into(), authority: None }
    }

    pub fn authority(mut self, a: impl Into<String>) -> Self { self.authority = Some(a.into()); self }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::ForceGlideRequest = self.clone().into();
        ProtoAny { type_url: "/clamm.v1.ForceGlideRequest".into(), value: msg.encode_to_vec() }
    }
}

impl From<ForceGlideRequest> for proto::ForceGlideRequest {
    fn from(r: ForceGlideRequest) -> Self {
        Self { pool_id: r.pool_id, target_price: r.target_price, timestamp: None, authority: r.authority }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query a single CLAMM position by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetPositionRequest {
    pub position_id: String,
}

impl GetPositionRequest {
    pub fn new(position_id: impl Into<String>) -> Self { Self { position_id: position_id.into() } }
}

impl From<GetPositionRequest> for proto::GetPositionRequest {
    fn from(r: GetPositionRequest) -> Self { Self { position_id: r.position_id } }
}

/// Simulate a swap on the CLAMM.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SimulateSwapRequest {
    pub pool_id: String,
    pub market_index: u64,
    pub side: Side,
    pub amount_in: String,
    pub amount_out_min: Option<String>,
}

impl SimulateSwapRequest {
    pub fn new(pool_id: impl Into<String>, market_index: u64, side: Side, amount_in: impl Into<String>) -> Self {
        Self { pool_id: pool_id.into(), market_index, side, amount_in: amount_in.into(), amount_out_min: None }
    }

    pub fn amount_out_min(mut self, min: impl Into<String>) -> Self { self.amount_out_min = Some(min.into()); self }
}

impl From<SimulateSwapRequest> for proto::SimulateSwapRequest {
    fn from(r: SimulateSwapRequest) -> Self {
        Self {
            pool_id: r.pool_id, market_index: r.market_index, side: i32::from(r.side),
            amount_in: r.amount_in, amount_out_min: r.amount_out_min, timestamp: None,
        }
    }
}

/// Get an AMM quote for a swap.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetQuoteRequest {
    pub pool_id: String,
    pub market_index: u64,
    pub side: Side,
    pub amount: String,
    pub exact_out: bool,
}

impl GetQuoteRequest {
    pub fn new(pool_id: impl Into<String>, market_index: u64, side: Side, amount: impl Into<String>) -> Self {
        Self { pool_id: pool_id.into(), market_index, side, amount: amount.into(), exact_out: false }
    }

    pub fn exact_out(mut self, v: bool) -> Self { self.exact_out = v; self }
}

impl From<GetQuoteRequest> for proto::GetQuoteRequest {
    fn from(r: GetQuoteRequest) -> Self {
        Self {
            pool_id: r.pool_id, market_index: r.market_index, side: i32::from(r.side),
            amount: r.amount, exact_out: r.exact_out, timestamp: None,
        }
    }
}

/// Query liquidity depth bands for a pool.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetLiquidityDepthRequest {
    pub pool_id: String,
    pub price_center: Option<String>,
}

impl GetLiquidityDepthRequest {
    pub fn new(pool_id: impl Into<String>) -> Self { Self { pool_id: pool_id.into(), price_center: None } }

    pub fn price_center(mut self, p: impl Into<String>) -> Self { self.price_center = Some(p.into()); self }
}

impl From<GetLiquidityDepthRequest> for proto::GetLiquidityDepthRequest {
    fn from(r: GetLiquidityDepthRequest) -> Self { Self { pool_id: r.pool_id, price_center: r.price_center } }
}

/// Query pool risk summary.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetPoolRiskSummaryRequest {
    pub pool_id: String,
}

impl GetPoolRiskSummaryRequest {
    pub fn new(pool_id: impl Into<String>) -> Self { Self { pool_id: pool_id.into() } }
}

impl From<GetPoolRiskSummaryRequest> for proto::GetPoolRiskSummaryRequest {
    fn from(r: GetPoolRiskSummaryRequest) -> Self { Self { pool_id: r.pool_id } }
}

/// Query boosted pool buffer state.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetBoostedBufferRequest {
    pub pool_id: String,
}

impl GetBoostedBufferRequest {
    pub fn new(pool_id: impl Into<String>) -> Self { Self { pool_id: pool_id.into() } }
}

impl From<GetBoostedBufferRequest> for proto::GetBoostedBufferRequest {
    fn from(r: GetBoostedBufferRequest) -> Self { Self { pool_id: r.pool_id } }
}

/// Simulate a ReClamm glide operation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SimulateReClammGlideRequest {
    pub pool_id: String,
    pub target_price: String,
    pub max_slippage_bps: Option<String>,
}

impl SimulateReClammGlideRequest {
    pub fn new(pool_id: impl Into<String>, target_price: impl Into<String>) -> Self {
        Self { pool_id: pool_id.into(), target_price: target_price.into(), max_slippage_bps: None }
    }

    pub fn max_slippage_bps(mut self, bps: impl Into<String>) -> Self { self.max_slippage_bps = Some(bps.into()); self }
}

impl From<SimulateReClammGlideRequest> for proto::SimulateReClammGlideRequest {
    fn from(r: SimulateReClammGlideRequest) -> Self {
        Self { pool_id: r.pool_id, target_price: r.target_price, max_slippage_bps: r.max_slippage_bps, timestamp: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_liquidity_to_any() {
        let req = AddLiquidityRequest::new("0x1234", "morpheum1abc", -100, 100, "500", "500");
        let any = req.to_any();
        assert_eq!(any.type_url, "/clamm.v1.AddLiquidityRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn remove_liquidity_to_any() {
        let req = RemoveLiquidityRequest::new("pos-1", "250").min_amounts("100", "100");
        let any = req.to_any();
        assert_eq!(any.type_url, "/clamm.v1.RemoveLiquidityRequest");
    }

    #[test]
    fn collect_fees_to_any() {
        let any = CollectFeesRequest::new("pos-1").to_any();
        assert_eq!(any.type_url, "/clamm.v1.CollectFeesRequest");
    }

    #[test]
    fn claim_yield_to_any() {
        let any = ClaimYieldRequest::new("morpheum1abc", "0x1234").to_any();
        assert_eq!(any.type_url, "/clamm.v1.ClaimYieldRequest");
    }

    #[test]
    fn force_glide_to_any() {
        let any = ForceGlideRequest::new("0x1234", "50000").authority("governance").to_any();
        assert_eq!(any.type_url, "/clamm.v1.ForceGlideRequest");
    }

    #[test]
    fn simulate_swap_conversion() {
        let req = SimulateSwapRequest::new("0x1234", 42, Side::Buy, "1000");
        let p: proto::SimulateSwapRequest = req.into();
        assert_eq!(p.pool_id, "0x1234");
        assert_eq!(p.market_index, 42);
    }
}
