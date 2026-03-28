//! Request wrappers for the CLAMM Graduation module.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::clammgrad::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

// ====================== TRANSACTION REQUESTS ======================

/// Request to initiate CLAMM graduation for a token.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InitiateGraduationRequest {
    pub token_index: String,
}

impl InitiateGraduationRequest {
    pub fn new(token_index: impl Into<String>) -> Self { Self { token_index: token_index.into() } }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::InitiateClammGraduationRequest { token_index: self.token_index.clone() };
        ProtoAny { type_url: "/clammgrad.v1.InitiateClammGraduationRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Request to execute a specific graduation step.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecuteGraduationStepRequest {
    pub token_index: String,
    pub step: u32,
}

impl ExecuteGraduationStepRequest {
    pub fn new(token_index: impl Into<String>, step: u32) -> Self {
        Self { token_index: token_index.into(), step }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::ExecuteGraduationStepRequest { token_index: self.token_index.clone(), step: self.step };
        ProtoAny { type_url: "/clammgrad.v1.ExecuteGraduationStepRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Request to cancel an in-progress graduation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CancelGraduationRequest {
    pub token_index: String,
}

impl CancelGraduationRequest {
    pub fn new(token_index: impl Into<String>) -> Self { Self { token_index: token_index.into() } }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::CancelGraduationRequest { token_index: self.token_index.clone() };
        ProtoAny { type_url: "/clammgrad.v1.CancelGraduationRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Request to update module parameters (governance only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    pub authority: String,
    pub params: crate::types::ClammGraduationParams,
}

impl UpdateParamsRequest {
    pub fn new(authority: impl Into<String>, params: crate::types::ClammGraduationParams) -> Self {
        Self { authority: authority.into(), params }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateParams {
            authority: self.authority.clone(),
            params: Some(proto::ClammGraduationParams {
                min_mcap_sat: self.params.min_mcap_sat.clone(),
                min_tvl_sat: self.params.min_tvl_sat.clone(),
                min_volume_30d_sat: self.params.min_volume_30d_sat.clone(),
                min_age_blocks: self.params.min_age_blocks,
                incentives_bps: self.params.incentives_bps,
                cooldown_blocks: self.params.cooldown_blocks,
                authority: self.params.authority.clone(),
                protocol_fee_bps: self.params.protocol_fee_bps,
                graduation_timeout_blocks: self.params.graduation_timeout_blocks,
                reserved: alloc::vec::Vec::new(),
            }),
        };
        ProtoAny { type_url: "/clammgrad.v1.MsgUpdateParams".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query graduation state for a token.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetGraduationStateRequest {
    pub token_index: String,
}

impl GetGraduationStateRequest {
    pub fn new(token_index: impl Into<String>) -> Self { Self { token_index: token_index.into() } }
}

impl From<GetGraduationStateRequest> for proto::GetGraduationStateRequest {
    fn from(r: GetGraduationStateRequest) -> Self { Self { token_index: r.token_index } }
}

/// List tokens eligible for graduation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListEligibleTokensRequest {
    pub limit: u32,
    pub offset: u64,
}

impl ListEligibleTokensRequest {
    pub fn new(limit: u32) -> Self { Self { limit, offset: 0 } }

    pub fn offset(mut self, o: u64) -> Self { self.offset = o; self }
}

impl From<ListEligibleTokensRequest> for proto::ListEligibleTokensRequest {
    fn from(r: ListEligibleTokensRequest) -> Self { Self { limit: r.limit, offset: r.offset } }
}

/// Query module parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetParamsRequest;

impl GetParamsRequest {
    pub fn new() -> Self { Self }
}

impl From<GetParamsRequest> for proto::GetParamsRequest {
    fn from(_: GetParamsRequest) -> Self { Self {} }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initiate_to_any() {
        let any = InitiateGraduationRequest::new("42").to_any();
        assert_eq!(any.type_url, "/clammgrad.v1.InitiateClammGraduationRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn execute_step_to_any() {
        let any = ExecuteGraduationStepRequest::new("42", 2).to_any();
        assert_eq!(any.type_url, "/clammgrad.v1.ExecuteGraduationStepRequest");
    }

    #[test]
    fn cancel_to_any() {
        let any = CancelGraduationRequest::new("42").to_any();
        assert_eq!(any.type_url, "/clammgrad.v1.CancelGraduationRequest");
    }

    #[test]
    fn list_eligible_converts() {
        let req = ListEligibleTokensRequest::new(10).offset(20);
        let p: proto::ListEligibleTokensRequest = req.into();
        assert_eq!(p.limit, 10);
        assert_eq!(p.offset, 20);
    }
}
