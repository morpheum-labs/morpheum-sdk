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

use crate::types::{RecoveryActionType, ReputationEvent, ReputationScore};

// ====================== TRANSACTION REQUESTS ======================

/// Request to apply a penalty to an agent's reputation.
///
/// This message is typically sent by governance or an authorised module signer
/// (e.g. risk-monitor, liquidation engine).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ApplyPenaltyRequest {
    /// Target agent hash.
    pub agent_hash: String,
    /// Base penalty amount (before multiplier).
    pub base_amount: u64,
    /// Multiplier (100 = 1.0×, 200 = 2.0×).
    pub multiplier: u32,
    /// Human-readable reason for the penalty.
    pub reason: String,
    /// Signer (governance or module authority).
    pub signer: Vec<u8>,
}

impl ApplyPenaltyRequest {
    /// Creates a new penalty request.
    pub fn new(
        agent_hash: impl Into<String>,
        base_amount: u64,
        multiplier: u32,
        reason: impl Into<String>,
        signer: Vec<u8>,
    ) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            base_amount,
            multiplier,
            reason: reason.into(),
            signer,
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgApplyPenalty = self.clone().into();
        ProtoAny {
            type_url: "/reputation.v1.MsgApplyPenalty".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ApplyPenaltyRequest> for proto::MsgApplyPenalty {
    fn from(req: ApplyPenaltyRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            base_amount: req.base_amount,
            multiplier: req.multiplier,
            reason: req.reason,
            signer: req.signer,
        }
    }
}

/// Response from a penalty application.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ApplyPenaltyResponse {
    /// Score after the penalty was applied.
    pub new_score: u64,
    /// Whether the Immortal floor prevented further reduction.
    pub floor_protected: bool,
}

impl From<proto::ApplyPenaltyResponse> for ApplyPenaltyResponse {
    fn from(p: proto::ApplyPenaltyResponse) -> Self {
        Self {
            new_score: p.new_score,
            floor_protected: p.floor_protected,
        }
    }
}

/// Request to apply a recovery / positive boost to an agent's reputation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ApplyRecoveryRequest {
    /// Target agent hash.
    pub agent_hash: String,
    /// Type of recovery action.
    pub action_type: RecoveryActionType,
    /// Amount of reputation to add.
    pub amount: u64,
    /// Human-readable reason.
    pub reason: String,
}

impl ApplyRecoveryRequest {
    /// Creates a new recovery request.
    pub fn new(
        agent_hash: impl Into<String>,
        action_type: RecoveryActionType,
        amount: u64,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            action_type,
            amount,
            reason: reason.into(),
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgApplyRecovery = self.clone().into();
        ProtoAny {
            type_url: "/reputation.v1.MsgApplyRecovery".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ApplyRecoveryRequest> for proto::MsgApplyRecovery {
    fn from(req: ApplyRecoveryRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            action_type: req.action_type.to_proto(),
            amount: req.amount,
            reason: req.reason,
        }
    }
}

/// Response from a recovery application.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ApplyRecoveryResponse {
    /// Score after recovery was applied.
    pub new_score: u64,
    /// Whether a milestone was reached as a result.
    pub milestone_reached: bool,
}

impl From<proto::ApplyRecoveryResponse> for ApplyRecoveryResponse {
    fn from(p: proto::ApplyRecoveryResponse) -> Self {
        Self {
            new_score: p.new_score,
            milestone_reached: p.milestone_reached,
        }
    }
}

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
    use alloc::vec;
    use crate::types::ReputationEventType;

    #[test]
    fn apply_penalty_request_to_any() {
        let req = ApplyPenaltyRequest::new(
            "agent123",
            5000,
            200,
            "front-running detected",
            vec![1u8; 64],
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/reputation.v1.MsgApplyPenalty");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn apply_recovery_request_to_any() {
        let req = ApplyRecoveryRequest::new(
            "agent456",
            RecoveryActionType::TradeFill,
            1000,
            "successful trade fill",
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/reputation.v1.MsgApplyRecovery");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn force_milestone_request_to_any() {
        let req = ForceMilestoneRequest::new("agent789", 3, vec![9u8; 64]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/reputation.v1.MsgForceMilestone");
    }

    #[test]
    fn apply_penalty_response_conversion() {
        let proto_res = proto::ApplyPenaltyResponse {
            new_score: 500_000,
            floor_protected: true,
        };
        let res: ApplyPenaltyResponse = proto_res.into();
        assert_eq!(res.new_score, 500_000);
        assert!(res.floor_protected);
    }

    #[test]
    fn apply_recovery_response_conversion() {
        let proto_res = proto::ApplyRecoveryResponse {
            new_score: 600_000,
            milestone_reached: true,
        };
        let res: ApplyRecoveryResponse = proto_res.into();
        assert_eq!(res.new_score, 600_000);
        assert!(res.milestone_reached);
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
