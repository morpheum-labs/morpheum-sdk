//! Request and response wrappers for the Interop module.
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
use morpheum_proto::interop::v1 as proto;

use crate::types::{
    BridgeRequestData, BridgeResponse, CrossChainProofPacket,
    IntentExportPacket,
};

// ====================== TRANSACTION REQUESTS ======================

/// Request to submit a general bridge request (proof or intent).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubmitBridgeRequest {
    /// The bridge request data containing routing info and payload.
    pub request: BridgeRequestData,
    /// Signer bytes (e.g. compressed public key or address).
    pub signer: Vec<u8>,
}

impl SubmitBridgeRequest {
    /// Creates a new submit-bridge request.
    pub fn new(request: BridgeRequestData, signer: Vec<u8>) -> Self {
        Self { request, signer }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgBridgeRequest = self.clone().into();
        ProtoAny {
            type_url: "/interop.v1.MsgBridgeRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<SubmitBridgeRequest> for proto::MsgBridgeRequest {
    fn from(req: SubmitBridgeRequest) -> Self {
        Self {
            request: Some(req.request.into()),
            signer: req.signer,
        }
    }
}

/// Response from a bridge request.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubmitBridgeResponse {
    pub success: bool,
    pub response: Option<BridgeResponse>,
}

impl From<proto::BridgeRequestResponse> for SubmitBridgeResponse {
    fn from(p: proto::BridgeRequestResponse) -> Self {
        Self {
            success: p.success,
            response: p.response.map(Into::into),
        }
    }
}

/// Request to export an intent for cross-chain execution.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExportIntentRequest {
    /// The intent packet to export.
    pub intent_packet: IntentExportPacket,
    /// Signer bytes.
    pub signer: Vec<u8>,
}

impl ExportIntentRequest {
    /// Creates a new export-intent request.
    pub fn new(intent_packet: IntentExportPacket, signer: Vec<u8>) -> Self {
        Self { intent_packet, signer }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgExportIntent = self.clone().into();
        ProtoAny {
            type_url: "/interop.v1.MsgExportIntent".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ExportIntentRequest> for proto::MsgExportIntent {
    fn from(req: ExportIntentRequest) -> Self {
        Self {
            intent_packet: Some(req.intent_packet.into()),
            signer: req.signer,
        }
    }
}

/// Response from exporting an intent.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExportIntentResponse {
    pub success: bool,
    pub target_tx_hash: String,
    pub exported_at: u64,
}

impl From<proto::ExportIntentResponse> for ExportIntentResponse {
    fn from(p: proto::ExportIntentResponse) -> Self {
        Self {
            success: p.success,
            target_tx_hash: p.target_tx_hash,
            exported_at: p.exported_at,
        }
    }
}

/// Request to export a proof for cross-chain verification.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExportProofRequest {
    /// The cross-chain proof packet to export.
    pub proof_packet: CrossChainProofPacket,
    /// Signer bytes.
    pub signer: Vec<u8>,
}

impl ExportProofRequest {
    /// Creates a new export-proof request.
    pub fn new(proof_packet: CrossChainProofPacket, signer: Vec<u8>) -> Self {
        Self { proof_packet, signer }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgExportProof = self.clone().into();
        ProtoAny {
            type_url: "/interop.v1.MsgExportProof".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ExportProofRequest> for proto::MsgExportProof {
    fn from(req: ExportProofRequest) -> Self {
        Self {
            proof_packet: Some(req.proof_packet.into()),
            signer: req.signer,
        }
    }
}

/// Response from exporting a proof.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExportProofResponse {
    pub success: bool,
    pub proof_id: String,
    pub exported_at: u64,
}

impl From<proto::ExportProofResponse> for ExportProofResponse {
    fn from(p: proto::ExportProofResponse) -> Self {
        Self {
            success: p.success,
            proof_id: p.proof_id,
            exported_at: p.exported_at,
        }
    }
}

// ====================== QUERY REQUESTS & RESPONSES ======================

/// Query the status of a bridge request.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryBridgeRequestRequest {
    pub request_id: String,
}

impl QueryBridgeRequestRequest {
    pub fn new(request_id: impl Into<String>) -> Self {
        Self { request_id: request_id.into() }
    }
}

impl From<QueryBridgeRequestRequest> for proto::QueryBridgeRequestRequest {
    fn from(req: QueryBridgeRequestRequest) -> Self {
        Self { request_id: req.request_id }
    }
}

/// Response containing the bridge request status.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryBridgeRequestResponse {
    pub response: Option<BridgeResponse>,
    pub found: bool,
}

impl From<proto::QueryBridgeRequestResponse> for QueryBridgeRequestResponse {
    fn from(p: proto::QueryBridgeRequestResponse) -> Self {
        Self {
            response: p.response.map(Into::into),
            found: p.found,
        }
    }
}

/// Query the status of an exported intent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryIntentExportRequest {
    pub intent_id: String,
}

impl QueryIntentExportRequest {
    pub fn new(intent_id: impl Into<String>) -> Self {
        Self { intent_id: intent_id.into() }
    }
}

impl From<QueryIntentExportRequest> for proto::QueryIntentExportRequest {
    fn from(req: QueryIntentExportRequest) -> Self {
        Self { intent_id: req.intent_id }
    }
}

/// Response containing exported intent status.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryIntentExportResponse {
    pub packet: Option<IntentExportPacket>,
    pub target_tx_hash: String,
    pub found: bool,
}

impl From<proto::QueryIntentExportResponse> for QueryIntentExportResponse {
    fn from(p: proto::QueryIntentExportResponse) -> Self {
        Self {
            packet: p.packet.map(Into::into),
            target_tx_hash: p.target_tx_hash,
            found: p.found,
        }
    }
}

/// Query the status of an exported proof.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProofExportRequest {
    pub proof_id: String,
}

impl QueryProofExportRequest {
    pub fn new(proof_id: impl Into<String>) -> Self {
        Self { proof_id: proof_id.into() }
    }
}

impl From<QueryProofExportRequest> for proto::QueryProofExportRequest {
    fn from(req: QueryProofExportRequest) -> Self {
        Self { proof_id: req.proof_id }
    }
}

/// Response containing exported proof status.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProofExportResponse {
    pub packet: Option<CrossChainProofPacket>,
    pub found: bool,
}

impl From<proto::QueryProofExportResponse> for QueryProofExportResponse {
    fn from(p: proto::QueryProofExportResponse) -> Self {
        Self {
            packet: p.packet.map(Into::into),
            found: p.found,
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
    use crate::types::{BridgePayload, CrossChainProof, ReputationProofPacket};
    use alloc::vec;

    fn sample_proof_packet() -> CrossChainProofPacket {
        CrossChainProofPacket {
            source_chain: "morpheum".into(),
            target_chain: "ethereum".into(),
            agent_hash: "agent-abc".into(),
            proof: Some(CrossChainProof::Reputation(ReputationProofPacket {
                agent_hash: "agent-abc".into(),
                score: 90_000,
                milestone_level: 3,
                is_immortal: false,
                timestamp: 1_700_000_000,
            })),
            exported_at: 1_700_000_100,
            merkle_proof: "merkle-root".into(),
        }
    }

    fn sample_intent_packet() -> IntentExportPacket {
        IntentExportPacket {
            intent_id: "intent-001".into(),
            source_agent_hash: "agent-abc".into(),
            target_chain: "ethereum".into(),
            intent_data: vec![1, 2, 3],
            signature: vec![0u8; 64],
            exported_at: 1_700_000_000,
        }
    }

    #[test]
    fn submit_bridge_request_to_any() {
        let req = SubmitBridgeRequest::new(
            BridgeRequestData {
                source_chain: "morpheum".into(),
                target_chain: "ethereum".into(),
                payload: Some(BridgePayload::Proof(sample_proof_packet())),
            },
            vec![0u8; 33],
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/interop.v1.MsgBridgeRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn export_intent_request_to_any() {
        let req = ExportIntentRequest::new(sample_intent_packet(), vec![0u8; 33]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/interop.v1.MsgExportIntent");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn export_proof_request_to_any() {
        let req = ExportProofRequest::new(sample_proof_packet(), vec![0u8; 33]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/interop.v1.MsgExportProof");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn submit_bridge_response_conversion() {
        let proto_res = proto::BridgeRequestResponse {
            success: true,
            response: Some(proto::BridgeResponse {
                success: true,
                error: String::new(),
                target_tx_hash: "0xabc".into(),
                processed_at: 1_700_000_000,
            }),
        };
        let res: SubmitBridgeResponse = proto_res.into();
        assert!(res.success);
        let br = res.response.unwrap();
        assert!(br.is_ok());
        assert_eq!(br.target_tx_hash, "0xabc");
    }

    #[test]
    fn export_intent_response_conversion() {
        let proto_res = proto::ExportIntentResponse {
            success: true,
            target_tx_hash: "0xdef".into(),
            exported_at: 1_700_000_500,
        };
        let res: ExportIntentResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.target_tx_hash, "0xdef");
        assert_eq!(res.exported_at, 1_700_000_500);
    }

    #[test]
    fn export_proof_response_conversion() {
        let proto_res = proto::ExportProofResponse {
            success: true,
            proof_id: "proof-123".into(),
            exported_at: 1_700_001_000,
        };
        let res: ExportProofResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.proof_id, "proof-123");
    }

    #[test]
    fn query_bridge_request_response_conversion() {
        let proto_res = proto::QueryBridgeRequestResponse {
            response: Some(proto::BridgeResponse {
                success: true,
                error: String::new(),
                target_tx_hash: "0x123".into(),
                processed_at: 1_700_000_000,
            }),
            found: true,
        };
        let res: QueryBridgeRequestResponse = proto_res.into();
        assert!(res.found);
        assert!(res.response.unwrap().is_ok());
    }

    #[test]
    fn query_intent_export_response_conversion() {
        let proto_res = proto::QueryIntentExportResponse {
            packet: Some(proto::IntentExportPacket {
                intent_id: "intent-001".into(),
                source_agent_hash: "agent-abc".into(),
                target_chain: "ethereum".into(),
                intent_data: vec![1, 2, 3],
                signature: vec![0u8; 64],
                exported_at: 1_700_000_000,
                blob_merkle_root: Vec::new(),
            }),
            target_tx_hash: "0xfeed".into(),
            found: true,
        };
        let res: QueryIntentExportResponse = proto_res.into();
        assert!(res.found);
        let pkt = res.packet.unwrap();
        assert_eq!(pkt.intent_id, "intent-001");
        assert_eq!(res.target_tx_hash, "0xfeed");
    }

    #[test]
    fn query_proof_export_response_conversion() {
        let proto_res = proto::QueryProofExportResponse {
            packet: Some(proto::CrossChainProofPacket {
                source_chain: "morpheum".into(),
                target_chain: "ethereum".into(),
                agent_hash: "agent-abc".into(),
                exported_at: 1_700_000_100,
                merkle_proof: "merkle".into(),
                blob_merkle_root: Vec::new(),
                proof: Some(proto::cross_chain_proof_packet::Proof::ReputationProof(
                    proto::ReputationProofPacket {
                        agent_hash: "agent-abc".into(),
                        score: 90_000,
                        milestone_level: 3,
                        is_immortal: false,
                        timestamp: 1_700_000_000,
                    },
                )),
            }),
            found: true,
        };
        let res: QueryProofExportResponse = proto_res.into();
        assert!(res.found);
        let pkt = res.packet.unwrap();
        assert_eq!(pkt.source_chain, "morpheum");
        assert!(matches!(pkt.proof, Some(CrossChainProof::Reputation(_))));
    }

    #[test]
    fn query_params_response_conversion() {
        let proto_res = proto::QueryParamsResponse {
            params: Some(proto::Params {
                bridging_enabled: true,
                intent_export_enabled: false,
                default_proof_ttl_seconds: 43_200,
                supported_target_chains: "ethereum".into(),
                enable_reputation_sync: true,
                max_concurrent_bridge_requests: 50,
            }),
        };
        let res: QueryParamsResponse = proto_res.into();
        let p = res.params.unwrap();
        assert!(p.bridging_enabled);
        assert!(!p.intent_export_enabled);
        assert_eq!(p.default_proof_ttl_seconds, 43_200);
        assert_eq!(p.supported_target_chains, "ethereum");
    }
}
