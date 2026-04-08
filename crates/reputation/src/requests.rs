//! Request and response wrappers for the Reputation module.
//!
//! These are clean, ergonomic Rust types that wrap the raw protobuf messages.
//! They provide type safety, validation, helper methods, and seamless conversion
//! to/from protobuf for use with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::types::Params;
use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::reputation::v1 as proto;

use crate::types::{ReputationEvent, ReputationScore};

// ====================== TRANSACTION REQUESTS ======================

/// Request to force a milestone (governance only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForceMilestoneRequest {
    /// Target agent hash.
    pub agent_hash: String,
    /// Milestone level to force (0-indexed).
    pub milestone_level: u32,
    /// Governance signature authorising this operation.
    pub gov_signature: Vec<u8>,
}

impl ForceMilestoneRequest {
    /// Creates a new force-milestone request.
    pub fn new(
        agent_hash: impl Into<String>,
        milestone_level: u32,
        gov_signature: Vec<u8>,
    ) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            milestone_level,
            gov_signature,
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgForceMilestone = self.clone().into();
        ProtoAny {
            type_url: "/reputation.v1.MsgForceMilestone".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ForceMilestoneRequest> for proto::MsgForceMilestone {
    fn from(req: ForceMilestoneRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            milestone_level: req.milestone_level,
            gov_signature: req.gov_signature,
        }
    }
}

/// Response from forcing a milestone.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForceMilestoneResponse {
    pub success: bool,
    pub new_score: u64,
}

impl From<proto::ForceMilestoneResponse> for ForceMilestoneResponse {
    fn from(p: proto::ForceMilestoneResponse) -> Self {
        Self {
            success: p.success,
            new_score: p.new_score,
        }
    }
}

/// Governance request to update reputation module parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    pub authority: String,
    pub params: Params,
}

impl UpdateParamsRequest {
    pub fn new(authority: impl Into<String>, params: Params) -> Self {
        Self {
            authority: authority.into(),
            params,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateParams {
            authority: self.authority.clone(),
            params: Some(self.params.clone().into()),
        };
        ProtoAny {
            type_url: "/reputation.v1.MsgUpdateParams".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Response from a params update.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsResponse {
    pub success: bool,
}

impl From<proto::UpdateParamsResponse> for UpdateParamsResponse {
    fn from(p: proto::UpdateParamsResponse) -> Self {
        Self { success: p.success }
    }
}

// ====================== QUERY REQUESTS & RESPONSES ======================

/// Query the current reputation score for an agent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryReputationScoreRequest {
    pub agent_hash: String,
}

impl QueryReputationScoreRequest {
    pub fn new(agent_hash: impl Into<String>) -> Self {
        Self { agent_hash: agent_hash.into() }
    }
}

impl From<QueryReputationScoreRequest> for proto::QueryReputationScoreRequest {
    fn from(req: QueryReputationScoreRequest) -> Self {
        Self { agent_hash: req.agent_hash }
    }
}

/// Response containing the reputation score (or `None` if not found).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryReputationScoreResponse {
    /// The score, if found.
    pub score: Option<ReputationScore>,
    /// Whether the agent was found.
    pub found: bool,
}

impl From<proto::QueryReputationScoreResponse> for QueryReputationScoreResponse {
    fn from(p: proto::QueryReputationScoreResponse) -> Self {
        Self {
            score: p.score.map(Into::into),
            found: p.found,
        }
    }
}

/// Query the reputation event history for an agent (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryReputationHistoryRequest {
    pub agent_hash: String,
    pub limit: u32,
    pub offset: u32,
}

impl QueryReputationHistoryRequest {
    pub fn new(agent_hash: impl Into<String>, limit: u32, offset: u32) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            limit,
            offset,
        }
    }
}

impl From<QueryReputationHistoryRequest> for proto::QueryReputationHistoryRequest {
    fn from(req: QueryReputationHistoryRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Response containing paginated reputation events.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryReputationHistoryResponse {
    pub events: Vec<ReputationEvent>,
    pub total_count: u32,
}

impl From<proto::QueryReputationHistoryResponse> for QueryReputationHistoryResponse {
    fn from(p: proto::QueryReputationHistoryResponse) -> Self {
        Self {
            events: p.events.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        }
    }
}

/// Query the milestone and perk status for an agent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMilestoneStatusRequest {
    pub agent_hash: String,
}

impl QueryMilestoneStatusRequest {
    pub fn new(agent_hash: impl Into<String>) -> Self {
        Self { agent_hash: agent_hash.into() }
    }
}

impl From<QueryMilestoneStatusRequest> for proto::QueryMilestoneStatusRequest {
    fn from(req: QueryMilestoneStatusRequest) -> Self {
        Self { agent_hash: req.agent_hash }
    }
}

/// Query the current module parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsRequest;

impl From<QueryParamsRequest> for proto::QueryParamsRequest {
    fn from(_: QueryParamsRequest) -> Self {
        Self {}
    }
}

/// Response containing the current module parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsResponse {
    pub params: Option<Params>,
}

impl From<proto::QueryParamsResponse> for QueryParamsResponse {
    fn from(p: proto::QueryParamsResponse) -> Self {
        Self {
            params: p.params.map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ReputationEventType;
    use alloc::vec;

    #[test]
    fn force_milestone_request_to_any() {
        let req = ForceMilestoneRequest::new("agent789", 3, vec![9u8; 64]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/reputation.v1.MsgForceMilestone");
    }

    #[test]
    fn update_params_request_to_any() {
        let req = UpdateParamsRequest::new("morpheum1gov", Params {
            daily_recovery_cap_bps: 3000,
            min_reputation_to_register: 0,
            enable_reputation_priority: true,
            slashing_multiplier: 100,
            milestone_thresholds: vec![10_000, 50_000, 100_000],
            milestone_rewards: vec![500, 1_000, 2_000],
            perk_multiplier_bps: 1500,
        });
        let any = req.to_any();
        assert_eq!(any.type_url, "/reputation.v1.MsgUpdateParams");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn query_score_response_conversion() {
        let proto_res = proto::QueryReputationScoreResponse {
            score: Some(proto::ReputationScore {
                agent_hash: "abc".into(),
                score: 750_000,
                ..Default::default()
            }),
            found: true,
        };
        let res: QueryReputationScoreResponse = proto_res.into();
        assert!(res.found);
        assert_eq!(res.score.unwrap().score, 750_000);
    }

    #[test]
    fn query_history_response_conversion() {
        let proto_res = proto::QueryReputationHistoryResponse {
            events: vec![proto::ReputationEvent {
                agent_hash: "xyz".into(),
                event_type: 1,
                delta: 100,
                reason: "test".into(),
                new_score: 100_100,
                timestamp: 1_700_000_000,
            }],
            total_count: 1,
        };
        let res: QueryReputationHistoryResponse = proto_res.into();
        assert_eq!(res.total_count, 1);
        assert_eq!(res.events.len(), 1);
        assert_eq!(res.events[0].event_type, ReputationEventType::Recovery);
    }
}
