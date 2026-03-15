//! Request and response wrappers for the Intent module.
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
use morpheum_proto::intent::v1 as proto;

use crate::types::AgentIntent;

// ====================== TRANSACTION REQUESTS ======================

/// Request to submit a new agent intent.
///
/// Supports all intent types: conditional, TWAP, multi-leg, and declarative.
/// The `agent_signature` proves that the submitting agent (or a delegated VC)
/// is authorised.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubmitIntentRequest {
    /// The full intent to submit.
    pub intent: AgentIntent,
    /// Signature proving the agent (or delegated VC).
    pub agent_signature: Vec<u8>,
}

impl SubmitIntentRequest {
    /// Creates a new submit request.
    pub fn new(intent: AgentIntent, agent_signature: Vec<u8>) -> Self {
        Self { intent, agent_signature }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgSubmitIntent = self.clone().into();
        ProtoAny {
            type_url: "/intent.v1.MsgSubmitIntent".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<SubmitIntentRequest> for proto::MsgSubmitIntent {
    fn from(req: SubmitIntentRequest) -> Self {
        Self {
            intent: Some(req.intent.into()),
            agent_signature: req.agent_signature,
        }
    }
}

/// Response from submitting an intent.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubmitIntentResponse {
    /// Assigned intent ID.
    pub intent_id: String,
    /// Whether the intent was accepted.
    pub accepted: bool,
    /// Human-readable decomposition summary (for declarative intents).
    pub decomposition_summary: String,
    /// Timestamp when the intent was created.
    pub created_at: u64,
}

impl From<proto::SubmitIntentResponse> for SubmitIntentResponse {
    fn from(p: proto::SubmitIntentResponse) -> Self {
        Self {
            intent_id: p.intent_id,
            accepted: p.accepted,
            decomposition_summary: p.decomposition_summary,
            created_at: p.created_at,
        }
    }
}

/// Request to cancel an active intent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CancelIntentRequest {
    /// ID of the intent to cancel.
    pub intent_id: String,
    /// Signature of the submitting agent.
    pub agent_signature: Vec<u8>,
    /// Reason for cancellation.
    pub reason: String,
}

impl CancelIntentRequest {
    /// Creates a new cancel request.
    pub fn new(
        intent_id: impl Into<String>,
        agent_signature: Vec<u8>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            intent_id: intent_id.into(),
            agent_signature,
            reason: reason.into(),
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCancelIntent = self.clone().into();
        ProtoAny {
            type_url: "/intent.v1.MsgCancelIntent".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CancelIntentRequest> for proto::MsgCancelIntent {
    fn from(req: CancelIntentRequest) -> Self {
        Self {
            intent_id: req.intent_id,
            agent_signature: req.agent_signature,
            reason: req.reason,
        }
    }
}

/// Response from cancelling an intent.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CancelIntentResponse {
    pub success: bool,
    pub cancelled_at: u64,
}

impl From<proto::CancelIntentResponse> for CancelIntentResponse {
    fn from(p: proto::CancelIntentResponse) -> Self {
        Self {
            success: p.success,
            cancelled_at: p.cancelled_at,
        }
    }
}

// ====================== QUERY REQUESTS & RESPONSES ======================

/// Query a specific intent by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryIntentRequest {
    pub intent_id: String,
}

impl QueryIntentRequest {
    pub fn new(intent_id: impl Into<String>) -> Self {
        Self { intent_id: intent_id.into() }
    }
}

impl From<QueryIntentRequest> for proto::QueryIntentRequest {
    fn from(req: QueryIntentRequest) -> Self {
        Self { intent_id: req.intent_id }
    }
}

/// Response containing an intent (or `None` if not found).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryIntentResponse {
    pub intent: Option<AgentIntent>,
    pub found: bool,
}

impl From<proto::QueryIntentResponse> for QueryIntentResponse {
    fn from(p: proto::QueryIntentResponse) -> Self {
        Self {
            intent: p.intent.map(Into::into),
            found: p.found,
        }
    }
}

/// Query all intents for a specific agent (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryIntentsByAgentRequest {
    pub agent_hash: String,
    pub limit: u32,
    pub offset: u32,
}

impl QueryIntentsByAgentRequest {
    pub fn new(agent_hash: impl Into<String>, limit: u32, offset: u32) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            limit,
            offset,
        }
    }
}

impl From<QueryIntentsByAgentRequest> for proto::QueryIntentsByAgentRequest {
    fn from(req: QueryIntentsByAgentRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Response containing paginated intents for an agent.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryIntentsByAgentResponse {
    pub intents: Vec<AgentIntent>,
    pub total_count: u32,
}

impl From<proto::QueryIntentsByAgentResponse> for QueryIntentsByAgentResponse {
    fn from(p: proto::QueryIntentsByAgentResponse) -> Self {
        Self {
            intents: p.intents.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        }
    }
}

/// Query active (pending/executing) intents for an agent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryActiveIntentsRequest {
    pub agent_hash: String,
    pub limit: u32,
}

impl QueryActiveIntentsRequest {
    pub fn new(agent_hash: impl Into<String>, limit: u32) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            limit,
        }
    }
}

impl From<QueryActiveIntentsRequest> for proto::QueryActiveIntentsRequest {
    fn from(req: QueryActiveIntentsRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            limit: req.limit,
        }
    }
}

/// Response containing active intents.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryActiveIntentsResponse {
    pub intents: Vec<AgentIntent>,
    pub total_count: u32,
}

impl From<proto::QueryActiveIntentsResponse> for QueryActiveIntentsResponse {
    fn from(p: proto::QueryActiveIntentsResponse) -> Self {
        Self {
            intents: p.intents.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        }
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
    use crate::types::*;

    fn sample_conditional_intent() -> AgentIntent {
        AgentIntent {
            intent_id: "intent-001".into(),
            agent_hash: "agent-abc".into(),
            intent_type: IntentType::Conditional,
            params: Some(IntentParams::Conditional(ConditionalParams {
                condition: "price > 50000".into(),
                action: "buy 1 BTC".into(),
            })),
            vc_proof_hash: "vc-hash".into(),
            expiry_timestamp: 1_700_003_600,
            priority_boost: 5,
            status: IntentStatus::Pending,
            created_at: 1_700_000_000,
        }
    }

    #[test]
    fn submit_intent_request_to_any() {
        let req = SubmitIntentRequest::new(sample_conditional_intent(), vec![1u8; 64]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/intent.v1.MsgSubmitIntent");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn cancel_intent_request_to_any() {
        let req = CancelIntentRequest::new("intent-001", vec![2u8; 64], "no longer needed");
        let any = req.to_any();
        assert_eq!(any.type_url, "/intent.v1.MsgCancelIntent");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn submit_intent_response_conversion() {
        let proto_res = proto::SubmitIntentResponse {
            intent_id: "intent-001".into(),
            accepted: true,
            decomposition_summary: "2 steps".into(),
            created_at: 1_700_000_000,
        };
        let res: SubmitIntentResponse = proto_res.into();
        assert!(res.accepted);
        assert_eq!(res.intent_id, "intent-001");
    }

    #[test]
    fn cancel_intent_response_conversion() {
        let proto_res = proto::CancelIntentResponse {
            success: true,
            cancelled_at: 1_700_001_000,
        };
        let res: CancelIntentResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.cancelled_at, 1_700_001_000);
    }

    #[test]
    fn query_intent_response_conversion() {
        let proto_res = proto::QueryIntentResponse {
            intent: Some(proto::AgentIntent {
                intent_id: "intent-001".into(),
                agent_hash: "agent-abc".into(),
                ..Default::default()
            }),
            found: true,
        };
        let res: QueryIntentResponse = proto_res.into();
        assert!(res.found);
        assert_eq!(res.intent.unwrap().intent_id, "intent-001");
    }

    #[test]
    fn query_intents_by_agent_response_conversion() {
        let proto_res = proto::QueryIntentsByAgentResponse {
            intents: vec![Default::default()],
            total_count: 1,
        };
        let res: QueryIntentsByAgentResponse = proto_res.into();
        assert_eq!(res.total_count, 1);
        assert_eq!(res.intents.len(), 1);
    }

    #[test]
    fn query_active_intents_response_conversion() {
        let proto_res = proto::QueryActiveIntentsResponse {
            intents: vec![],
            total_count: 0,
        };
        let res: QueryActiveIntentsResponse = proto_res.into();
        assert_eq!(res.total_count, 0);
        assert!(res.intents.is_empty());
    }

    #[test]
    fn query_params_response_conversion() {
        let proto_res = proto::QueryParamsResponse {
            params: Some(proto::Params {
                default_expiry_seconds: 7200,
                max_concurrent_intents_per_agent: 5,
                ..Default::default()
            }),
        };
        let res: QueryParamsResponse = proto_res.into();
        let p = res.params.unwrap();
        assert_eq!(p.default_expiry_seconds, 7200);
        assert_eq!(p.max_concurrent_intents_per_agent, 5);
    }
}
