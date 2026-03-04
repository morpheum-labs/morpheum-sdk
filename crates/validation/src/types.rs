//! Domain types for the Validation module.
//!
//! These are clean, idiomatic Rust representations of the validation protobuf
//! messages. They provide type safety, ergonomic APIs, and full round-trip
//! conversion to/from protobuf while remaining strictly `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::validation::v1 as proto;

// ====================== PROOF TYPE ======================

/// Type of validation proof.
///
/// Mirrors the protobuf `ProofType` enum and classifies how an agent's
/// capabilities were verified.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ProofType {
    /// Historical backtest against market data.
    #[default]
    Backtest = 0,
    /// Live inference evaluation.
    Inference = 1,
    /// Human-performed evaluation.
    HumanEval = 2,
    /// TEE (Trusted Execution Environment) attestation.
    TeeAttestation = 3,
    /// External validator assessment.
    ExternalValidator = 4,
    /// Marketplace evaluation.
    MarketplaceEval = 5,
    /// Application-defined custom proof type.
    Custom = 255,
}

impl ProofType {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Inference,
            2 => Self::HumanEval,
            3 => Self::TeeAttestation,
            4 => Self::ExternalValidator,
            5 => Self::MarketplaceEval,
            255 => Self::Custom,
            _ => Self::Backtest,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for ProofType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Backtest => f.write_str("BACKTEST"),
            Self::Inference => f.write_str("INFERENCE"),
            Self::HumanEval => f.write_str("HUMAN_EVAL"),
            Self::TeeAttestation => f.write_str("TEE_ATTESTATION"),
            Self::ExternalValidator => f.write_str("EXTERNAL_VALIDATOR"),
            Self::MarketplaceEval => f.write_str("MARKETPLACE_EVAL"),
            Self::Custom => f.write_str("CUSTOM"),
        }
    }
}

// ====================== VALIDATION PROOF ======================

/// On-chain validation proof record.
///
/// Represents a completed verification of an agent's capabilities, signed
/// by a verifier and contributing to the agent's reputation score.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidationProof {
    /// Unique proof identifier (blake3 hash).
    pub proof_id: String,
    /// Agent being validated.
    pub agent_hash: String,
    /// Verifier that issued/verified this proof.
    pub verifier_agent_hash: String,
    /// Type of validation performed.
    pub proof_type: ProofType,
    /// Points contributed to reputation (0–10 000).
    pub score_contribution: u32,
    /// Block timestamp when the proof was recorded.
    pub timestamp: u64,
    /// Hash of the off-chain proof payload.
    pub data_hash: String,
    /// Merkle root for cross-chain verifiability.
    pub merkle_root: String,
    /// BLS aggregate signature.
    pub signature: Vec<u8>,
}

impl ValidationProof {
    /// Maximum allowed score contribution.
    pub const MAX_SCORE: u32 = 10_000;

    /// Returns `true` if the score contribution is within the valid range.
    pub fn is_valid_score(&self) -> bool {
        self.score_contribution <= Self::MAX_SCORE
    }
}

impl From<proto::ValidationProof> for ValidationProof {
    fn from(p: proto::ValidationProof) -> Self {
        Self {
            proof_id: p.proof_id,
            agent_hash: p.agent_hash,
            verifier_agent_hash: p.verifier_agent_hash,
            proof_type: ProofType::from_proto(p.proof_type),
            score_contribution: p.score_contribution,
            timestamp: p.timestamp,
            data_hash: p.data_hash,
            merkle_root: p.merkle_root,
            signature: p.signature,
        }
    }
}

impl From<ValidationProof> for proto::ValidationProof {
    fn from(v: ValidationProof) -> Self {
        Self {
            proof_id: v.proof_id,
            agent_hash: v.agent_hash,
            verifier_agent_hash: v.verifier_agent_hash,
            proof_type: v.proof_type.to_proto(),
            score_contribution: v.score_contribution,
            timestamp: v.timestamp,
            data_hash: v.data_hash,
            merkle_root: v.merkle_root,
            signature: v.signature,
        }
    }
}

// ====================== CROSS-CHAIN PROOF PACKET ======================

/// Cross-chain proof packet exported via the Bridge Framework.
///
/// Contains a validation proof together with its Merkle inclusion proof,
/// enabling external chains to verify the proof's authenticity.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CrossChainProofPacket {
    /// The underlying validation proof.
    pub proof: Option<ValidationProof>,
    /// Merkle inclusion proof for external chain verification.
    pub merkle_proof: String,
    /// Timestamp when the packet was exported.
    pub exported_at: u64,
}

impl From<proto::CrossChainProofPacket> for CrossChainProofPacket {
    fn from(p: proto::CrossChainProofPacket) -> Self {
        Self {
            proof: p.proof.map(Into::into),
            merkle_proof: p.merkle_proof,
            exported_at: p.exported_at,
        }
    }
}

impl From<CrossChainProofPacket> for proto::CrossChainProofPacket {
    fn from(c: CrossChainProofPacket) -> Self {
        Self {
            proof: c.proof.map(Into::into),
            merkle_proof: c.merkle_proof,
            exported_at: c.exported_at,
        }
    }
}

// ====================== PARAMS ======================

/// Module parameters (governance-controlled).
///
/// Provides sensible defaults:
/// - `min_score_contribution`: 0 (no minimum)
/// - `max_proofs_per_agent`: 0 (unlimited)
/// - `require_verifier_signature`: true
/// - `default_proof_expiry_seconds`: 0 (no expiry)
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    /// Minimum score contribution a proof must provide to be accepted.
    pub min_score_contribution: u32,
    /// Maximum number of proofs allowed per agent (0 = unlimited).
    pub max_proofs_per_agent: u32,
    /// Whether proofs must be signed by a whitelisted verifier.
    pub require_verifier_signature: bool,
    /// Default proof expiry in seconds (0 = no expiry).
    pub default_proof_expiry_seconds: u64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            min_score_contribution: 0,
            max_proofs_per_agent: 0,
            require_verifier_signature: true,
            default_proof_expiry_seconds: 0,
        }
    }
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            min_score_contribution: p.min_score_contribution,
            max_proofs_per_agent: p.max_proofs_per_agent,
            require_verifier_signature: p.require_verifier_signature,
            default_proof_expiry_seconds: p.default_proof_expiry_seconds,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            min_score_contribution: p.min_score_contribution,
            max_proofs_per_agent: p.max_proofs_per_agent,
            require_verifier_signature: p.require_verifier_signature,
            default_proof_expiry_seconds: p.default_proof_expiry_seconds,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn proof_type_roundtrip() {
        for t in [
            ProofType::Backtest,
            ProofType::Inference,
            ProofType::HumanEval,
            ProofType::TeeAttestation,
            ProofType::ExternalValidator,
            ProofType::MarketplaceEval,
            ProofType::Custom,
        ] {
            assert_eq!(ProofType::from_proto(t.to_proto()), t);
        }
    }

    #[test]
    fn proof_type_display() {
        assert_eq!(ProofType::Backtest.to_string(), "BACKTEST");
        assert_eq!(ProofType::Inference.to_string(), "INFERENCE");
        assert_eq!(ProofType::HumanEval.to_string(), "HUMAN_EVAL");
        assert_eq!(ProofType::TeeAttestation.to_string(), "TEE_ATTESTATION");
        assert_eq!(ProofType::ExternalValidator.to_string(), "EXTERNAL_VALIDATOR");
        assert_eq!(ProofType::MarketplaceEval.to_string(), "MARKETPLACE_EVAL");
        assert_eq!(ProofType::Custom.to_string(), "CUSTOM");
    }

    #[test]
    fn proof_type_unknown_defaults_to_backtest() {
        assert_eq!(ProofType::from_proto(999), ProofType::Backtest);
    }

    #[test]
    fn validation_proof_score_check() {
        let proof = ValidationProof {
            score_contribution: 5000,
            ..Default::default()
        };
        assert!(proof.is_valid_score());

        let proof = ValidationProof {
            score_contribution: 10_000,
            ..Default::default()
        };
        assert!(proof.is_valid_score());

        let proof = ValidationProof {
            score_contribution: 10_001,
            ..Default::default()
        };
        assert!(!proof.is_valid_score());
    }

    #[test]
    fn validation_proof_roundtrip() {
        let proof = ValidationProof {
            proof_id: "proof-001".into(),
            agent_hash: "agent-abc".into(),
            verifier_agent_hash: "verifier-xyz".into(),
            proof_type: ProofType::TeeAttestation,
            score_contribution: 8500,
            timestamp: 1_700_000_000,
            data_hash: "data-hash-abc".into(),
            merkle_root: "merkle-root".into(),
            signature: vec![1, 2, 3, 4],
        };
        let proto: proto::ValidationProof = proof.clone().into();
        let back: ValidationProof = proto.into();
        assert_eq!(proof, back);
    }

    #[test]
    fn cross_chain_proof_packet_roundtrip() {
        let packet = CrossChainProofPacket {
            proof: Some(ValidationProof {
                proof_id: "proof-001".into(),
                agent_hash: "agent-abc".into(),
                proof_type: ProofType::Backtest,
                ..Default::default()
            }),
            merkle_proof: "inclusion-proof".into(),
            exported_at: 1_700_000_000,
        };
        let proto: proto::CrossChainProofPacket = packet.clone().into();
        let back: CrossChainProofPacket = proto.into();
        assert_eq!(packet, back);
    }

    #[test]
    fn cross_chain_proof_packet_no_proof() {
        let packet = CrossChainProofPacket {
            proof: None,
            merkle_proof: String::new(),
            exported_at: 0,
        };
        let proto: proto::CrossChainProofPacket = packet.clone().into();
        let back: CrossChainProofPacket = proto.into();
        assert_eq!(packet, back);
    }

    #[test]
    fn params_defaults() {
        let params = Params::default();
        assert_eq!(params.min_score_contribution, 0);
        assert_eq!(params.max_proofs_per_agent, 0);
        assert!(params.require_verifier_signature);
        assert_eq!(params.default_proof_expiry_seconds, 0);
    }

    #[test]
    fn params_roundtrip() {
        let params = Params {
            min_score_contribution: 100,
            max_proofs_per_agent: 50,
            require_verifier_signature: false,
            default_proof_expiry_seconds: 86400,
        };
        let proto: proto::Params = params.clone().into();
        let back: Params = proto.into();
        assert_eq!(params, back);
    }
}
