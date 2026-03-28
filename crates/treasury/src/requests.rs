//! Request wrappers for the treasury module.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::treasury::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::{ReserveCategory, TreasuryParams};

// ====================== TRANSACTION REQUESTS ======================

/// Sweep revenue into a Treasury Reserve Category.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SweepRevenueRequest {
    pub source_module: String,
    pub target_category: ReserveCategory,
    pub amount: u64,
    pub reason: String,
    pub tx_hash: Vec<u8>,
    pub authority: String,
}

impl SweepRevenueRequest {
    pub fn new(
        source_module: impl Into<String>, target_category: ReserveCategory,
        amount: u64, reason: impl Into<String>, authority: impl Into<String>,
    ) -> Self {
        Self {
            source_module: source_module.into(), target_category, amount,
            reason: reason.into(), tx_hash: Vec::new(), authority: authority.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgSweepRevenue {
            source_module: self.source_module.clone(), target_category: i32::from(self.target_category),
            amount: self.amount, reason: self.reason.clone(),
            tx_hash: self.tx_hash.clone(), authority: self.authority.clone(),
        };
        ProtoAny { type_url: "/treasury.v1.MsgSweepRevenue".into(), value: msg.encode_to_vec() }
    }
}

/// Allocate funds between reserve categories or to a target module.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AllocateFundsRequest {
    pub authority: String,
    pub source_category: ReserveCategory,
    pub target_module: String,
    pub target_category: ReserveCategory,
    pub amount: u64,
    pub reason: String,
    pub proposal_id: u64,
    pub signature: Vec<u8>,
}

impl AllocateFundsRequest {
    pub fn new(
        authority: impl Into<String>, source_category: ReserveCategory,
        amount: u64, reason: impl Into<String>,
    ) -> Self {
        Self {
            authority: authority.into(), source_category,
            target_module: String::new(), target_category: ReserveCategory::Unspecified,
            amount, reason: reason.into(), proposal_id: 0, signature: Vec::new(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgAllocateFunds {
            authority: self.authority.clone(), source_category: i32::from(self.source_category),
            target_module: self.target_module.clone(), target_category: i32::from(self.target_category),
            amount: self.amount, reason: self.reason.clone(),
            proposal_id: self.proposal_id, signature: self.signature.clone(),
        };
        ProtoAny { type_url: "/treasury.v1.MsgAllocateFunds".into(), value: msg.encode_to_vec() }
    }
}

/// Update governance parameters (governance-only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    pub authority: String,
    pub params: TreasuryParams,
}

impl UpdateParamsRequest {
    pub fn new(authority: impl Into<String>, params: TreasuryParams) -> Self {
        Self { authority: authority.into(), params }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateParams {
            authority: self.authority.clone(), params: Some(self.params.clone().into()),
        };
        ProtoAny { type_url: "/treasury.v1.MsgUpdateParams".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query the complete reserves state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryReservesStateRequest;

impl From<QueryReservesStateRequest> for proto::QueryReservesStateRequest {
    fn from(_: QueryReservesStateRequest) -> Self { Self {} }
}

/// Query real-time treasury metrics.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryTreasuryMetricsRequest;

impl From<QueryTreasuryMetricsRequest> for proto::QueryTreasuryMetricsRequest {
    fn from(_: QueryTreasuryMetricsRequest) -> Self { Self {} }
}

/// Query a specific category reserve.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryCategoryReserveRequest {
    pub category: ReserveCategory,
}

impl QueryCategoryReserveRequest {
    pub fn new(category: ReserveCategory) -> Self { Self { category } }
}

impl From<QueryCategoryReserveRequest> for proto::QueryCategoryReserveRequest {
    fn from(r: QueryCategoryReserveRequest) -> Self { Self { category: i32::from(r.category) } }
}

/// Query current governance parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsRequest;

impl From<QueryParamsRequest> for proto::QueryParamsRequest {
    fn from(_: QueryParamsRequest) -> Self { Self {} }
}

/// Query paginated allocation history.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryAllocationHistoryRequest {
    pub source_category: Option<ReserveCategory>,
    pub target_category: Option<ReserveCategory>,
    pub reason: String,
    pub proposal_id: u64,
    pub limit: u64,
}

impl QueryAllocationHistoryRequest {
    pub fn new() -> Self { Self::default() }

    pub fn source_category(mut self, v: ReserveCategory) -> Self { self.source_category = Some(v); self }
    pub fn target_category(mut self, v: ReserveCategory) -> Self { self.target_category = Some(v); self }
    pub fn reason(mut self, v: impl Into<String>) -> Self { self.reason = v.into(); self }
    pub fn proposal_id(mut self, v: u64) -> Self { self.proposal_id = v; self }
    pub fn limit(mut self, v: u64) -> Self { self.limit = v; self }
}

impl From<QueryAllocationHistoryRequest> for proto::QueryAllocationHistoryRequest {
    fn from(r: QueryAllocationHistoryRequest) -> Self {
        Self {
            pagination: None,
            source_category: r.source_category.map_or(0, i32::from),
            target_category: r.target_category.map_or(0, i32::from),
            reason: r.reason, proposal_id: r.proposal_id, limit: r.limit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep_revenue_to_any() {
        let any = SweepRevenueRequest::new(
            "clob", ReserveCategory::InsuranceProtection, 1000, "maker_taker_fees", "morph1mod",
        ).to_any();
        assert_eq!(any.type_url, "/treasury.v1.MsgSweepRevenue");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn allocate_funds_to_any() {
        let any = AllocateFundsRequest::new(
            "morph1gov", ReserveCategory::InsuranceProtection, 5000, "insurance_topup",
        ).to_any();
        assert_eq!(any.type_url, "/treasury.v1.MsgAllocateFunds");
    }

    #[test]
    fn query_category_reserve_conversion() {
        let p: proto::QueryCategoryReserveRequest =
            QueryCategoryReserveRequest::new(ReserveCategory::BuybackBurn).into();
        assert_eq!(p.category, 3);
    }

    #[test]
    fn query_allocation_history_with_filters() {
        let p: proto::QueryAllocationHistoryRequest = QueryAllocationHistoryRequest::new()
            .source_category(ReserveCategory::InsuranceProtection)
            .reason("fee_sweep").limit(100).into();
        assert_eq!(p.source_category, 1);
        assert_eq!(p.reason, "fee_sweep");
        assert_eq!(p.limit, 100);
    }
}
