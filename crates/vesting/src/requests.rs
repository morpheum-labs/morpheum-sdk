//! Request wrappers for the vesting module.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::vesting::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::{ScheduleType, VestingCategory, VestingParams};

// ====================== TRANSACTION REQUESTS ======================

/// Create a new vesting schedule.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateVestingRequest {
    pub authority: String,
    pub beneficiary: String,
    pub total_amount: String,
    pub start_timestamp: u64,
    pub cliff_duration: u64,
    pub vesting_duration: u64,
    pub schedule_type: ScheduleType,
    pub category: VestingCategory,
    pub revocable: bool,
    pub step_timestamps: Vec<u64>,
    pub step_amounts: Vec<String>,
}

impl CreateVestingRequest {
    pub fn new(
        authority: impl Into<String>, beneficiary: impl Into<String>,
        total_amount: impl Into<String>, vesting_duration: u64,
        schedule_type: ScheduleType,
    ) -> Self {
        Self {
            authority: authority.into(), beneficiary: beneficiary.into(),
            total_amount: total_amount.into(), start_timestamp: 0,
            cliff_duration: 0, vesting_duration, schedule_type,
            category: VestingCategory::Unspecified, revocable: false,
            step_timestamps: Vec::new(), step_amounts: Vec::new(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgCreateVesting {
            authority: self.authority.clone(), beneficiary: self.beneficiary.clone(),
            total_amount: self.total_amount.clone(), start_timestamp: self.start_timestamp,
            cliff_duration: self.cliff_duration, vesting_duration: self.vesting_duration,
            schedule_type: i32::from(self.schedule_type),
            category: i32::from(self.category), revocable: self.revocable,
            step_timestamps: self.step_timestamps.clone(),
            step_amounts: self.step_amounts.clone(),
        };
        ProtoAny { type_url: "/vesting.v1.MsgCreateVesting".into(), value: msg.encode_to_vec() }
    }
}

/// Claim releasable tokens.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClaimRequest {
    pub beneficiary: String,
    pub amount: String,
}

impl ClaimRequest {
    /// Claim max releasable tokens.
    pub fn max(beneficiary: impl Into<String>) -> Self {
        Self { beneficiary: beneficiary.into(), amount: String::new() }
    }

    /// Claim a specific amount.
    pub fn amount(beneficiary: impl Into<String>, amount: impl Into<String>) -> Self {
        Self { beneficiary: beneficiary.into(), amount: amount.into() }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgClaim {
            beneficiary: self.beneficiary.clone(), amount: self.amount.clone(),
        };
        ProtoAny { type_url: "/vesting.v1.MsgClaim".into(), value: msg.encode_to_vec() }
    }
}

/// Revoke a revocable vesting schedule.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RevokeVestingRequest {
    pub authority: String,
    pub beneficiary: String,
    pub vesting_id: u64,
    pub reason: String,
}

impl RevokeVestingRequest {
    pub fn new(
        authority: impl Into<String>, beneficiary: impl Into<String>,
        vesting_id: u64, reason: impl Into<String>,
    ) -> Self {
        Self {
            authority: authority.into(), beneficiary: beneficiary.into(),
            vesting_id, reason: reason.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgRevokeVesting {
            authority: self.authority.clone(), beneficiary: self.beneficiary.clone(),
            vesting_id: self.vesting_id, reason: self.reason.clone(),
        };
        ProtoAny { type_url: "/vesting.v1.MsgRevokeVesting".into(), value: msg.encode_to_vec() }
    }
}

/// Update governance parameters (governance-only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    pub authority: String,
    pub params: VestingParams,
}

impl UpdateParamsRequest {
    pub fn new(authority: impl Into<String>, params: VestingParams) -> Self {
        Self { authority: authority.into(), params }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateParams {
            authority: self.authority.clone(), params: Some(self.params.clone().into()),
        };
        ProtoAny { type_url: "/vesting.v1.MsgUpdateParams".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query aggregated vesting summary for a beneficiary.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryVestingSummaryRequest {
    pub beneficiary: String,
}

impl QueryVestingSummaryRequest {
    pub fn new(beneficiary: impl Into<String>) -> Self { Self { beneficiary: beneficiary.into() } }
}

impl From<QueryVestingSummaryRequest> for proto::QueryVestingSummaryRequest {
    fn from(r: QueryVestingSummaryRequest) -> Self { Self { beneficiary: r.beneficiary } }
}

/// Query a specific vesting entry.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryVestingEntryRequest {
    pub beneficiary: String,
    pub vesting_id: u64,
}

impl QueryVestingEntryRequest {
    pub fn new(beneficiary: impl Into<String>, vesting_id: u64) -> Self {
        Self { beneficiary: beneficiary.into(), vesting_id }
    }
}

impl From<QueryVestingEntryRequest> for proto::QueryVestingEntryRequest {
    fn from(r: QueryVestingEntryRequest) -> Self {
        Self { beneficiary: r.beneficiary, vesting_id: r.vesting_id }
    }
}

/// Query all vesting entries for a beneficiary.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryVestingEntriesRequest {
    pub beneficiary: String,
    pub limit: u32,
    pub offset: u32,
}

impl QueryVestingEntriesRequest {
    pub fn new(beneficiary: impl Into<String>) -> Self {
        Self { beneficiary: beneficiary.into(), limit: 0, offset: 0 }
    }

    pub fn limit(mut self, v: u32) -> Self { self.limit = v; self }
    pub fn offset(mut self, v: u32) -> Self { self.offset = v; self }
}

impl From<QueryVestingEntriesRequest> for proto::QueryVestingEntriesRequest {
    fn from(r: QueryVestingEntriesRequest) -> Self {
        Self { beneficiary: r.beneficiary, limit: r.limit, offset: r.offset }
    }
}

/// Query current module parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsRequest;

impl From<QueryParamsRequest> for proto::QueryParamsRequest {
    fn from(_: QueryParamsRequest) -> Self { Self {} }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vesting_to_any() {
        let any = CreateVestingRequest::new(
            "morph1gov", "morph1user", "1000000", 63072000, ScheduleType::CliffLinear,
        ).to_any();
        assert_eq!(any.type_url, "/vesting.v1.MsgCreateVesting");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn claim_max_to_any() {
        let any = ClaimRequest::max("morph1user").to_any();
        assert_eq!(any.type_url, "/vesting.v1.MsgClaim");
    }

    #[test]
    fn claim_amount_to_any() {
        let any = ClaimRequest::amount("morph1user", "50000").to_any();
        assert_eq!(any.type_url, "/vesting.v1.MsgClaim");
    }

    #[test]
    fn revoke_to_any() {
        let any = RevokeVestingRequest::new("morph1gov", "morph1user", 1, "policy violation").to_any();
        assert_eq!(any.type_url, "/vesting.v1.MsgRevokeVesting");
    }

    #[test]
    fn query_entries_with_pagination() {
        let p: proto::QueryVestingEntriesRequest = QueryVestingEntriesRequest::new("morph1user")
            .limit(10).offset(5).into();
        assert_eq!(p.limit, 10);
        assert_eq!(p.offset, 5);
    }
}
