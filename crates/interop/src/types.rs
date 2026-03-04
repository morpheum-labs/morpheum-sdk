//! Domain types for the Interop module.
//!
//! These are clean, idiomatic Rust representations of the interop protobuf
//! messages. They provide type safety, ergonomic APIs, and full round-trip
//! conversion to/from protobuf while remaining strictly `no_std` compatible.
//!
//! Key types:
//! - [`CrossChainProofPacket`] — wraps any proof type for cross-chain export
//! - [`IntentExportPacket`] — wraps an intent for cross-chain routing
//! - [`BridgePayload`] / [`BridgeRequestData`] — general bridge request with
//!   typed payload
//! - [`BridgeResponse`] — response from a bridge operation

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::interop::v1 as proto;

// ====================== PROOF PACKETS ======================

/// Reputation proof for cross-chain verification.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReputationProofPacket {
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Current reputation score.
    pub score: u64,
    /// Milestone level achieved.
    pub milestone_level: u32,
    /// Whether the agent has reached immortal status.
    pub is_immortal: bool,
    /// Timestamp of this proof snapshot.
    pub timestamp: u64,
}

impl From<proto::ReputationProofPacket> for ReputationProofPacket {
    fn from(p: proto::ReputationProofPacket) -> Self {
        Self {
            agent_hash: p.agent_hash,
            score: p.score,
            milestone_level: p.milestone_level,
            is_immortal: p.is_immortal,
            timestamp: p.timestamp,
        }
    }
}

impl From<ReputationProofPacket> for proto::ReputationProofPacket {
    fn from(p: ReputationProofPacket) -> Self {
        Self {
            agent_hash: p.agent_hash,
            score: p.score,
            milestone_level: p.milestone_level,
            is_immortal: p.is_immortal,
            timestamp: p.timestamp,
        }
    }
}

/// Validation proof for cross-chain verification.
///
/// The inner `proof` is kept as the protobuf-generated type from the
/// `validation.v1` package since it is a cross-module reference. This avoids
/// creating a hard dependency on the validation SDK crate while preserving
/// full fidelity.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationProofPacket {
    /// The validation proof being exported.
    pub proof: Option<morpheum_proto::validation::v1::ValidationProof>,
    /// Merkle proof anchoring this proof to the chain state.
    pub merkle_proof: String,
}

impl From<proto::ValidationProofPacket> for ValidationProofPacket {
    fn from(p: proto::ValidationProofPacket) -> Self {
        Self {
            proof: p.proof,
            merkle_proof: p.merkle_proof,
        }
    }
}

impl From<ValidationProofPacket> for proto::ValidationProofPacket {
    fn from(p: ValidationProofPacket) -> Self {
        Self {
            proof: p.proof,
            merkle_proof: p.merkle_proof,
        }
    }
}

/// Identity proof for cross-chain verification.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IdentityProofPacket {
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Owner's agent hash.
    pub owner_agent_hash: String,
    /// Capability bitflags.
    pub capabilities: u64,
    /// Whether the agent has reached immortal status.
    pub is_immortal: bool,
    /// Timestamp when the agent was registered.
    pub registered_at: u64,
}

impl From<proto::IdentityProofPacket> for IdentityProofPacket {
    fn from(p: proto::IdentityProofPacket) -> Self {
        Self {
            agent_hash: p.agent_hash,
            owner_agent_hash: p.owner_agent_hash,
            capabilities: p.capabilities,
            is_immortal: p.is_immortal,
            registered_at: p.registered_at,
        }
    }
}

impl From<IdentityProofPacket> for proto::IdentityProofPacket {
    fn from(p: IdentityProofPacket) -> Self {
        Self {
            agent_hash: p.agent_hash,
            owner_agent_hash: p.owner_agent_hash,
            capabilities: p.capabilities,
            is_immortal: p.is_immortal,
            registered_at: p.registered_at,
        }
    }
}

// ====================== CROSS-CHAIN PROOF ======================

/// The type of proof carried inside a [`CrossChainProofPacket`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CrossChainProof {
    /// A reputation proof.
    Reputation(ReputationProofPacket),
    /// A validation proof.
    Validation(ValidationProofPacket),
    /// An identity proof.
    Identity(IdentityProofPacket),
}

impl From<proto::cross_chain_proof_packet::Proof> for CrossChainProof {
    fn from(p: proto::cross_chain_proof_packet::Proof) -> Self {
        match p {
            proto::cross_chain_proof_packet::Proof::ReputationProof(r) => {
                Self::Reputation(r.into())
            }
            proto::cross_chain_proof_packet::Proof::ValidationProof(v) => {
                Self::Validation(v.into())
            }
            proto::cross_chain_proof_packet::Proof::IdentityProof(i) => {
                Self::Identity(i.into())
            }
        }
    }
}

impl From<CrossChainProof> for proto::cross_chain_proof_packet::Proof {
    fn from(p: CrossChainProof) -> Self {
        match p {
            CrossChainProof::Reputation(r) => Self::ReputationProof(r.into()),
            CrossChainProof::Validation(v) => Self::ValidationProof(v.into()),
            CrossChainProof::Identity(i) => Self::IdentityProof(i.into()),
        }
    }
}

/// Cross-chain proof packet exported from an agent registry.
///
/// Contains routing metadata (source/target chain, agent hash, Merkle proof)
/// and exactly one typed proof variant.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CrossChainProofPacket {
    /// Source chain identifier.
    pub source_chain: String,
    /// Target chain identifier.
    pub target_chain: String,
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// The proof payload (one of reputation, validation, or identity).
    pub proof: Option<CrossChainProof>,
    /// Timestamp when this proof was exported.
    pub exported_at: u64,
    /// Merkle proof anchoring this packet to the source chain state.
    pub merkle_proof: String,
}

impl From<proto::CrossChainProofPacket> for CrossChainProofPacket {
    fn from(p: proto::CrossChainProofPacket) -> Self {
        Self {
            source_chain: p.source_chain,
            target_chain: p.target_chain,
            agent_hash: p.agent_hash,
            proof: p.proof.map(Into::into),
            exported_at: p.exported_at,
            merkle_proof: p.merkle_proof,
        }
    }
}

impl From<CrossChainProofPacket> for proto::CrossChainProofPacket {
    fn from(p: CrossChainProofPacket) -> Self {
        Self {
            source_chain: p.source_chain,
            target_chain: p.target_chain,
            agent_hash: p.agent_hash,
            proof: p.proof.map(Into::into),
            exported_at: p.exported_at,
            merkle_proof: p.merkle_proof,
        }
    }
}

// ====================== INTENT EXPORT ======================

/// Intent export packet for cross-chain intent routing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IntentExportPacket {
    /// Unique intent identifier.
    pub intent_id: String,
    /// Agent hash of the intent source.
    pub source_agent_hash: String,
    /// Target chain for execution.
    pub target_chain: String,
    /// Serialized `AgentIntent` bytes.
    pub intent_data: Vec<u8>,
    /// Signature authorising the export.
    pub signature: Vec<u8>,
    /// Timestamp when this intent was exported.
    pub exported_at: u64,
}

impl From<proto::IntentExportPacket> for IntentExportPacket {
    fn from(p: proto::IntentExportPacket) -> Self {
        Self {
            intent_id: p.intent_id,
            source_agent_hash: p.source_agent_hash,
            target_chain: p.target_chain,
            intent_data: p.intent_data,
            signature: p.signature,
            exported_at: p.exported_at,
        }
    }
}

impl From<IntentExportPacket> for proto::IntentExportPacket {
    fn from(p: IntentExportPacket) -> Self {
        Self {
            intent_id: p.intent_id,
            source_agent_hash: p.source_agent_hash,
            target_chain: p.target_chain,
            intent_data: p.intent_data,
            signature: p.signature,
            exported_at: p.exported_at,
        }
    }
}

// ====================== BRIDGE REQUEST / RESPONSE ======================

/// The payload type carried inside a [`BridgeRequestData`].
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BridgePayload {
    /// A cross-chain proof packet.
    Proof(CrossChainProofPacket),
    /// An intent export packet.
    Intent(IntentExportPacket),
}

impl From<proto::bridge_request::Payload> for BridgePayload {
    fn from(p: proto::bridge_request::Payload) -> Self {
        match p {
            proto::bridge_request::Payload::ProofPacket(pkt) => Self::Proof(pkt.into()),
            proto::bridge_request::Payload::IntentPacket(pkt) => Self::Intent(pkt.into()),
        }
    }
}

impl From<BridgePayload> for proto::bridge_request::Payload {
    fn from(p: BridgePayload) -> Self {
        match p {
            BridgePayload::Proof(pkt) => Self::ProofPacket(pkt.into()),
            BridgePayload::Intent(pkt) => Self::IntentPacket(pkt.into()),
        }
    }
}

/// General-purpose bridge request with typed payload.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BridgeRequestData {
    /// Source chain identifier.
    pub source_chain: String,
    /// Target chain identifier.
    pub target_chain: String,
    /// The bridge payload (proof or intent).
    pub payload: Option<BridgePayload>,
}

impl From<proto::BridgeRequest> for BridgeRequestData {
    fn from(p: proto::BridgeRequest) -> Self {
        Self {
            source_chain: p.source_chain,
            target_chain: p.target_chain,
            payload: p.payload.map(Into::into),
        }
    }
}

impl From<BridgeRequestData> for proto::BridgeRequest {
    fn from(p: BridgeRequestData) -> Self {
        Self {
            source_chain: p.source_chain,
            target_chain: p.target_chain,
            payload: p.payload.map(Into::into),
        }
    }
}

/// Response from a bridge operation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BridgeResponse {
    /// Whether the operation succeeded.
    pub success: bool,
    /// Error message (empty on success).
    pub error: String,
    /// Transaction hash on the target chain (if applicable).
    pub target_tx_hash: String,
    /// Timestamp when the response was processed.
    pub processed_at: u64,
}

impl BridgeResponse {
    /// Returns `true` if the bridge operation succeeded without errors.
    pub fn is_ok(&self) -> bool {
        self.success && self.error.is_empty()
    }
}

impl From<proto::BridgeResponse> for BridgeResponse {
    fn from(p: proto::BridgeResponse) -> Self {
        Self {
            success: p.success,
            error: p.error,
            target_tx_hash: p.target_tx_hash,
            processed_at: p.processed_at,
        }
    }
}

impl From<BridgeResponse> for proto::BridgeResponse {
    fn from(p: BridgeResponse) -> Self {
        Self {
            success: p.success,
            error: p.error,
            target_tx_hash: p.target_tx_hash,
            processed_at: p.processed_at,
        }
    }
}

// ====================== PARAMS ======================

/// Module parameters (governance-controlled).
///
/// Provides sensible defaults:
/// - `bridging_enabled`: true
/// - `intent_export_enabled`: true
/// - `default_proof_ttl_seconds`: 86400 (24 hours)
/// - `supported_target_chains`: empty
/// - `enable_reputation_sync`: true
/// - `max_concurrent_bridge_requests`: 100
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    /// Whether cross-chain bridging is enabled.
    pub bridging_enabled: bool,
    /// Whether intent export is enabled.
    pub intent_export_enabled: bool,
    /// Default proof TTL in seconds for cross-chain proofs.
    pub default_proof_ttl_seconds: u64,
    /// Supported target chains (comma-separated).
    pub supported_target_chains: String,
    /// Whether automatic reputation proof syncing is enabled.
    pub enable_reputation_sync: bool,
    /// Maximum number of concurrent bridge requests.
    pub max_concurrent_bridge_requests: u32,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            bridging_enabled: true,
            intent_export_enabled: true,
            default_proof_ttl_seconds: 86_400,
            supported_target_chains: String::new(),
            enable_reputation_sync: true,
            max_concurrent_bridge_requests: 100,
        }
    }
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            bridging_enabled: p.bridging_enabled,
            intent_export_enabled: p.intent_export_enabled,
            default_proof_ttl_seconds: p.default_proof_ttl_seconds,
            supported_target_chains: p.supported_target_chains,
            enable_reputation_sync: p.enable_reputation_sync,
            max_concurrent_bridge_requests: p.max_concurrent_bridge_requests,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            bridging_enabled: p.bridging_enabled,
            intent_export_enabled: p.intent_export_enabled,
            default_proof_ttl_seconds: p.default_proof_ttl_seconds,
            supported_target_chains: p.supported_target_chains,
            enable_reputation_sync: p.enable_reputation_sync,
            max_concurrent_bridge_requests: p.max_concurrent_bridge_requests,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn reputation_proof_packet_roundtrip() {
        let pkt = ReputationProofPacket {
            agent_hash: "agent-abc".into(),
            score: 95_000,
            milestone_level: 5,
            is_immortal: true,
            timestamp: 1_700_000_000,
        };
        let proto: proto::ReputationProofPacket = pkt.clone().into();
        let back: ReputationProofPacket = proto.into();
        assert_eq!(pkt, back);
    }

    #[test]
    fn identity_proof_packet_roundtrip() {
        let pkt = IdentityProofPacket {
            agent_hash: "agent-abc".into(),
            owner_agent_hash: "owner-xyz".into(),
            capabilities: 0xFF,
            is_immortal: false,
            registered_at: 1_700_000_000,
        };
        let proto: proto::IdentityProofPacket = pkt.clone().into();
        let back: IdentityProofPacket = proto.into();
        assert_eq!(pkt, back);
    }

    #[test]
    fn validation_proof_packet_roundtrip() {
        let pkt = ValidationProofPacket {
            proof: None,
            merkle_proof: "merkle123".into(),
        };
        let proto: proto::ValidationProofPacket = pkt.clone().into();
        let back: ValidationProofPacket = proto.into();
        assert_eq!(pkt, back);
    }

    #[test]
    fn cross_chain_proof_reputation_roundtrip() {
        let packet = CrossChainProofPacket {
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
        };
        let proto: proto::CrossChainProofPacket = packet.clone().into();
        let back: CrossChainProofPacket = proto.into();
        assert_eq!(packet, back);
    }

    #[test]
    fn cross_chain_proof_identity_roundtrip() {
        let packet = CrossChainProofPacket {
            source_chain: "morpheum".into(),
            target_chain: "solana".into(),
            agent_hash: "agent-xyz".into(),
            proof: Some(CrossChainProof::Identity(IdentityProofPacket {
                agent_hash: "agent-xyz".into(),
                owner_agent_hash: "owner-1".into(),
                capabilities: 0xAB,
                is_immortal: true,
                registered_at: 1_699_000_000,
            })),
            exported_at: 1_700_001_000,
            merkle_proof: "mp-2".into(),
        };
        let proto: proto::CrossChainProofPacket = packet.clone().into();
        let back: CrossChainProofPacket = proto.into();
        assert_eq!(packet, back);
    }

    #[test]
    fn cross_chain_proof_none_proof() {
        let packet = CrossChainProofPacket {
            source_chain: "morpheum".into(),
            target_chain: "ethereum".into(),
            agent_hash: "agent-abc".into(),
            proof: None,
            exported_at: 0,
            merkle_proof: String::new(),
        };
        let proto: proto::CrossChainProofPacket = packet.clone().into();
        let back: CrossChainProofPacket = proto.into();
        assert_eq!(packet, back);
    }

    #[test]
    fn intent_export_packet_roundtrip() {
        let pkt = IntentExportPacket {
            intent_id: "intent-001".into(),
            source_agent_hash: "agent-abc".into(),
            target_chain: "ethereum".into(),
            intent_data: vec![1, 2, 3, 4],
            signature: vec![0u8; 64],
            exported_at: 1_700_000_000,
        };
        let proto: proto::IntentExportPacket = pkt.clone().into();
        let back: IntentExportPacket = proto.into();
        assert_eq!(pkt, back);
    }

    #[test]
    fn bridge_payload_proof_roundtrip() {
        let payload = BridgePayload::Proof(CrossChainProofPacket {
            source_chain: "morpheum".into(),
            target_chain: "ethereum".into(),
            agent_hash: "agent-abc".into(),
            proof: None,
            exported_at: 0,
            merkle_proof: String::new(),
        });
        let proto: proto::bridge_request::Payload = payload.clone().into();
        let back: BridgePayload = proto.into();
        assert_eq!(payload, back);
    }

    #[test]
    fn bridge_payload_intent_roundtrip() {
        let payload = BridgePayload::Intent(IntentExportPacket {
            intent_id: "intent-001".into(),
            source_agent_hash: "agent-abc".into(),
            target_chain: "ethereum".into(),
            intent_data: vec![1, 2, 3],
            signature: vec![0u8; 64],
            exported_at: 1_700_000_000,
        });
        let proto: proto::bridge_request::Payload = payload.clone().into();
        let back: BridgePayload = proto.into();
        assert_eq!(payload, back);
    }

    #[test]
    fn bridge_request_data_roundtrip() {
        let req = BridgeRequestData {
            source_chain: "morpheum".into(),
            target_chain: "ethereum".into(),
            payload: Some(BridgePayload::Intent(IntentExportPacket {
                intent_id: "i-1".into(),
                ..Default::default()
            })),
        };
        let proto: proto::BridgeRequest = req.clone().into();
        let back: BridgeRequestData = proto.into();
        assert_eq!(req, back);
    }

    #[test]
    fn bridge_response_roundtrip() {
        let resp = BridgeResponse {
            success: true,
            error: String::new(),
            target_tx_hash: "0xdeadbeef".into(),
            processed_at: 1_700_000_000,
        };
        let proto: proto::BridgeResponse = resp.clone().into();
        let back: BridgeResponse = proto.into();
        assert_eq!(resp, back);
    }

    #[test]
    fn bridge_response_is_ok() {
        let ok = BridgeResponse { success: true, error: String::new(), ..Default::default() };
        assert!(ok.is_ok());

        let fail = BridgeResponse { success: false, error: "timeout".into(), ..Default::default() };
        assert!(!fail.is_ok());

        let partial = BridgeResponse { success: true, error: "warning".into(), ..Default::default() };
        assert!(!partial.is_ok());
    }

    #[test]
    fn params_defaults() {
        let params = Params::default();
        assert!(params.bridging_enabled);
        assert!(params.intent_export_enabled);
        assert_eq!(params.default_proof_ttl_seconds, 86_400);
        assert!(params.supported_target_chains.is_empty());
        assert!(params.enable_reputation_sync);
        assert_eq!(params.max_concurrent_bridge_requests, 100);
    }

    #[test]
    fn params_roundtrip() {
        let params = Params {
            bridging_enabled: false,
            intent_export_enabled: true,
            default_proof_ttl_seconds: 43_200,
            supported_target_chains: "ethereum,solana".into(),
            enable_reputation_sync: false,
            max_concurrent_bridge_requests: 50,
        };
        let proto: proto::Params = params.clone().into();
        let back: Params = proto.into();
        assert_eq!(params, back);
    }
}
