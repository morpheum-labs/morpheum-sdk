//! Request and response wrappers for the Validation module.
//!
//! These are clean, ergonomic Rust types that wrap the raw protobuf messages.
//! They provide type safety, validation, helper methods, and seamless conversion
//! to/from protobuf for use with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::validation::v1 as proto;

use crate::types::{Params, ProofType, ValidationProof};

// ====================== TRANSACTION REQUESTS ======================

/// Request to submit a new validation proof.
///
/// The `verifier_signature` authenticates the verifier that performed the
/// validation. The runtime assigns a `proof_id` and `timestamp` on acceptance.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubmitProofRequest {
    /// Agent being validated.
    pub agent_hash: String,
    /// Type of validation performed.
    pub proof_type: ProofType,
    /// Score contribution (0–10 000).
    pub score_contribution: u32,
    /// Hash of the full proof payload (stored in Persistent Memory).
    pub data_hash: String,
    /// Verifier's BLS signature.
    pub verifier_signature: Vec<u8>,
}

impl SubmitProofRequest {
    /// Creates a new submit-proof request with required fields.
    pub fn new(
        agent_hash: impl Into<String>,
        proof_type: ProofType,
        score_contribution: u32,
        data_hash: impl Into<String>,
        verifier_signature: Vec<u8>,
    ) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            proof_type,
            score_contribution,
            data_hash: data_hash.into(),
            verifier_signature,
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgSubmitProof = self.clone().into();
        ProtoAny {
            type_url: "/validation.v1.MsgSubmitProof".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<SubmitProofRequest> for proto::MsgSubmitProof {
    fn from(req: SubmitProofRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            proof_type: req.proof_type.to_proto(),
            score_contribution: req.score_contribution,
            data_hash: req.data_hash,
            verifier_signature: req.verifier_signature,
        }
    }
}

/// Response from submitting a validation proof.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubmitProofResponse {
    /// Assigned proof ID.
    pub proof_id: String,
    /// Block timestamp when the proof was recorded.
    pub timestamp: u64,
    /// Whether the proof was accepted.
    pub accepted: bool,
}

impl From<proto::SubmitProofResponse> for SubmitProofResponse {
    fn from(p: proto::SubmitProofResponse) -> Self {
        Self {
            proof_id: p.proof_id,
            timestamp: p.timestamp,
            accepted: p.accepted,
        }
    }
}

/// Request to revoke a validation proof.
///
/// May be issued by the original verifier or governance.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RevokeProofRequest {
    /// ID of the proof to revoke.
    pub proof_id: String,
    /// Hash of the verifier (or governance) agent revoking the proof.
    pub verifier_agent_hash: String,
    /// Verifier's BLS signature authorising the revocation.
    pub verifier_signature: Vec<u8>,
    /// Human-readable reason for revocation.
    pub reason: String,
}

impl RevokeProofRequest {
    /// Creates a new revoke-proof request.
    pub fn new(
        proof_id: impl Into<String>,
        verifier_agent_hash: impl Into<String>,
        verifier_signature: Vec<u8>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            proof_id: proof_id.into(),
            verifier_agent_hash: verifier_agent_hash.into(),
            verifier_signature,
            reason: reason.into(),
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgRevokeProof = self.clone().into();
        ProtoAny {
            type_url: "/validation.v1.MsgRevokeProof".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<RevokeProofRequest> for proto::MsgRevokeProof {
    fn from(req: RevokeProofRequest) -> Self {
        Self {
            proof_id: req.proof_id,
            verifier_agent_hash: req.verifier_agent_hash,
            verifier_signature: req.verifier_signature,
            reason: req.reason,
        }
    }
}

/// Response from revoking a validation proof.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RevokeProofResponse {
    pub success: bool,
    pub revoked_at: u64,
}

impl From<proto::RevokeProofResponse> for RevokeProofResponse {
    fn from(p: proto::RevokeProofResponse) -> Self {
        Self {
            success: p.success,
            revoked_at: p.revoked_at,
        }
    }
}

/// Request to update module parameters (governance only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    /// New parameters.
    pub params: Params,
    /// Governance signature authorising this update.
    pub gov_signature: Vec<u8>,
}

impl UpdateParamsRequest {
    /// Creates a new update-params request.
    pub fn new(params: Params, gov_signature: Vec<u8>) -> Self {
        Self { params, gov_signature }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdateParams = self.clone().into();
        ProtoAny {
            type_url: "/validation.v1.MsgUpdateParams".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdateParamsRequest> for proto::MsgUpdateParams {
    fn from(req: UpdateParamsRequest) -> Self {
        Self {
            params: Some(req.params.into()),
            gov_signature: req.gov_signature,
        }
    }
}

/// Response from updating parameters.
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

/// Query a specific validation proof by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProofRequest {
    pub proof_id: String,
}

impl QueryProofRequest {
    pub fn new(proof_id: impl Into<String>) -> Self {
        Self { proof_id: proof_id.into() }
    }
}

impl From<QueryProofRequest> for proto::QueryProofRequest {
    fn from(req: QueryProofRequest) -> Self {
        Self { proof_id: req.proof_id }
    }
}

/// Response containing a validation proof (or indicating not found).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProofResponse {
    pub proof: Option<ValidationProof>,
    pub found: bool,
}

impl From<proto::QueryProofResponse> for QueryProofResponse {
    fn from(p: proto::QueryProofResponse) -> Self {
        Self {
            proof: p.proof.map(Into::into),
            found: p.found,
        }
    }
}

/// Query all validation proofs for a specific agent (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProofsByAgentRequest {
    pub agent_hash: String,
    pub limit: u32,
    pub offset: u32,
}

impl QueryProofsByAgentRequest {
    pub fn new(agent_hash: impl Into<String>, limit: u32, offset: u32) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            limit,
            offset,
        }
    }
}

impl From<QueryProofsByAgentRequest> for proto::QueryProofsByAgentRequest {
    fn from(req: QueryProofsByAgentRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Response containing paginated proofs for an agent.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProofsByAgentResponse {
    pub proofs: Vec<ValidationProof>,
    pub total_count: u32,
}

impl From<proto::QueryProofsByAgentResponse> for QueryProofsByAgentResponse {
    fn from(p: proto::QueryProofsByAgentResponse) -> Self {
        Self {
            proofs: p.proofs.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        }
    }
}

/// Query validation proofs by type (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProofsByTypeRequest {
    pub proof_type: ProofType,
    pub limit: u32,
    pub offset: u32,
}

impl QueryProofsByTypeRequest {
    pub fn new(proof_type: ProofType, limit: u32, offset: u32) -> Self {
        Self { proof_type, limit, offset }
    }
}

impl From<QueryProofsByTypeRequest> for proto::QueryProofsByTypeRequest {
    fn from(req: QueryProofsByTypeRequest) -> Self {
        Self {
            proof_type: req.proof_type.to_proto(),
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Response containing proofs filtered by type.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProofsByTypeResponse {
    pub proofs: Vec<ValidationProof>,
    pub total_count: u32,
}

impl From<proto::QueryProofsByTypeResponse> for QueryProofsByTypeResponse {
    fn from(p: proto::QueryProofsByTypeResponse) -> Self {
        Self {
            proofs: p.proofs.into_iter().map(Into::into).collect(),
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

    #[test]
    fn submit_proof_request_to_any() {
        let req = SubmitProofRequest::new(
            "agent-abc",
            ProofType::TeeAttestation,
            8500,
            "data-hash-abc",
            vec![0u8; 64],
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/validation.v1.MsgSubmitProof");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn revoke_proof_request_to_any() {
        let req = RevokeProofRequest::new(
            "proof-001",
            "verifier-xyz",
            vec![0u8; 64],
            "Fraudulent backtest data",
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/validation.v1.MsgRevokeProof");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn update_params_request_to_any() {
        let req = UpdateParamsRequest::new(Params::default(), vec![0u8; 64]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/validation.v1.MsgUpdateParams");
    }

    #[test]
    fn submit_proof_response_conversion() {
        let proto_res = proto::SubmitProofResponse {
            proof_id: "proof-001".into(),
            timestamp: 1_700_000_000,
            accepted: true,
        };
        let res: SubmitProofResponse = proto_res.into();
        assert!(res.accepted);
        assert_eq!(res.proof_id, "proof-001");
        assert_eq!(res.timestamp, 1_700_000_000);
    }

    #[test]
    fn revoke_proof_response_conversion() {
        let proto_res = proto::RevokeProofResponse {
            success: true,
            revoked_at: 1_700_001_000,
        };
        let res: RevokeProofResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.revoked_at, 1_700_001_000);
    }

    #[test]
    fn query_proof_response_conversion() {
        let proto_res = proto::QueryProofResponse {
            proof: Some(proto::ValidationProof {
                proof_id: "proof-001".into(),
                agent_hash: "agent-abc".into(),
                proof_type: 3, // TEE_ATTESTATION
                score_contribution: 9000,
                ..Default::default()
            }),
            found: true,
        };
        let res: QueryProofResponse = proto_res.into();
        assert!(res.found);
        let proof = res.proof.unwrap();
        assert_eq!(proof.proof_id, "proof-001");
        assert_eq!(proof.proof_type, ProofType::TeeAttestation);
        assert_eq!(proof.score_contribution, 9000);
    }

    #[test]
    fn query_proofs_by_agent_response_conversion() {
        let proto_res = proto::QueryProofsByAgentResponse {
            proofs: vec![Default::default(), Default::default()],
            total_count: 2,
        };
        let res: QueryProofsByAgentResponse = proto_res.into();
        assert_eq!(res.total_count, 2);
        assert_eq!(res.proofs.len(), 2);
    }

    #[test]
    fn query_proofs_by_type_response_conversion() {
        let proto_res = proto::QueryProofsByTypeResponse {
            proofs: vec![Default::default()],
            total_count: 1,
        };
        let res: QueryProofsByTypeResponse = proto_res.into();
        assert_eq!(res.total_count, 1);
        assert_eq!(res.proofs.len(), 1);
    }

    #[test]
    fn query_params_response_conversion() {
        let proto_res = proto::QueryParamsResponse {
            params: Some(proto::Params {
                min_score_contribution: 100,
                max_proofs_per_agent: 50,
                require_verifier_signature: false,
                default_proof_expiry_seconds: 86400,
            }),
        };
        let res: QueryParamsResponse = proto_res.into();
        let p = res.params.unwrap();
        assert_eq!(p.min_score_contribution, 100);
        assert_eq!(p.max_proofs_per_agent, 50);
        assert!(!p.require_verifier_signature);
        assert_eq!(p.default_proof_expiry_seconds, 86400);
    }
}
