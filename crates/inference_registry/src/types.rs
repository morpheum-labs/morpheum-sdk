//! Domain types for the Inference Registry module.
//!
//! Clean, idiomatic Rust representations of the inference_registry protobuf
//! messages. Full round-trip conversion to/from protobuf and `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::inference_registry::v1 as proto;

// ====================== SUPPORTED OPS BITFLAGS ======================

/// Well-known supported-ops bitflags for `ModelCommitment.supported_ops`.
pub mod ops {
    pub const INFER: u64 = 1 << 0;
    pub const EMBED: u64 = 1 << 1;
    pub const VECTOR_SEARCH: u64 = 1 << 2;
    pub const FINE_TUNE: u64 = 1 << 3;
}

// ====================== QUANT FORMAT ======================

/// Quantization format for a registered model.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum QuantFormat {
    #[default]
    Unspecified = 0,
    Q4KM = 1,
    Q5KM = 2,
    Q80 = 3,
    Fp16 = 4,
}

impl QuantFormat {
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Q4KM,
            2 => Self::Q5KM,
            3 => Self::Q80,
            4 => Self::Fp16,
            _ => Self::Unspecified,
        }
    }

    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for QuantFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => f.write_str("UNSPECIFIED"),
            Self::Q4KM => f.write_str("Q4_K_M"),
            Self::Q5KM => f.write_str("Q5_K_M"),
            Self::Q80 => f.write_str("Q8_0"),
            Self::Fp16 => f.write_str("FP16"),
        }
    }
}

// ====================== MODEL STATUS ======================

/// Lifecycle status of a registered model.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ModelStatus {
    #[default]
    Unspecified = 0,
    Active = 1,
    Deprecated = 2,
    UnderReview = 3,
}

impl ModelStatus {
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Active,
            2 => Self::Deprecated,
            3 => Self::UnderReview,
            _ => Self::Unspecified,
        }
    }

    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for ModelStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => f.write_str("UNSPECIFIED"),
            Self::Active => f.write_str("ACTIVE"),
            Self::Deprecated => f.write_str("DEPRECATED"),
            Self::UnderReview => f.write_str("UNDER_REVIEW"),
        }
    }
}

// ====================== MODEL COMMITMENT ======================

/// Canonical on-chain record for a registered inference model.
///
/// One record per model, governance-controlled, zk-commitment backed.
/// Used by AgentCore VM, validation, agent_registry, and memory modules.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModelCommitment {
    /// Primary key — `blake3(name + quant + version)`.
    pub model_id: Vec<u8>,
    /// Human-readable name (e.g. "Llama-3.1-8B-Q4").
    pub display_name: String,
    /// Quantization format.
    pub quant_format: QuantFormat,
    /// Parameter count in billions.
    pub param_count: u64,
    /// Halo2 zk commitment hash.
    pub zk_commitment: Vec<u8>,
    /// Bitflags for supported operations (INFER, EMBED, etc.).
    pub supported_ops: u64,
    /// Unix timestamp of registration (seconds).
    pub registered_at: u64,
    /// Multi-sig / DAO approval hash.
    pub governance_hash: Vec<u8>,
    /// Model version (for upgrades).
    pub version: u32,
    /// Blob-backed model weights Merkle root.
    pub blob_merkle_root: Vec<u8>,
}

impl ModelCommitment {
    /// Returns the model_id as a hex string.
    pub fn model_id_hex(&self) -> String {
        hex::encode(&self.model_id)
    }

    /// Returns `true` if the model supports the INFER operation.
    pub fn supports_infer(&self) -> bool {
        self.supported_ops & ops::INFER != 0
    }

    /// Returns `true` if the model supports the EMBED operation.
    pub fn supports_embed(&self) -> bool {
        self.supported_ops & ops::EMBED != 0
    }

    /// Returns `true` if the model supports VECTOR_SEARCH.
    pub fn supports_vector_search(&self) -> bool {
        self.supported_ops & ops::VECTOR_SEARCH != 0
    }

    /// Returns `true` if the given op bitflag is set.
    pub fn supports_op(&self, op: u64) -> bool {
        self.supported_ops & op != 0
    }
}

impl From<proto::ModelCommitment> for ModelCommitment {
    fn from(p: proto::ModelCommitment) -> Self {
        Self {
            model_id: p.model_id,
            display_name: p.display_name,
            quant_format: QuantFormat::from_proto(p.quant_format),
            param_count: p.param_count,
            zk_commitment: p.zk_commitment,
            supported_ops: p.supported_ops,
            registered_at: p.registered_at,
            governance_hash: p.governance_hash,
            version: p.version,
            blob_merkle_root: p.blob_merkle_root,
        }
    }
}

impl From<ModelCommitment> for proto::ModelCommitment {
    fn from(m: ModelCommitment) -> Self {
        Self {
            model_id: m.model_id,
            display_name: m.display_name,
            quant_format: m.quant_format.to_proto(),
            param_count: m.param_count,
            zk_commitment: m.zk_commitment,
            supported_ops: m.supported_ops,
            registered_at: m.registered_at,
            governance_hash: m.governance_hash,
            version: m.version,
            blob_merkle_root: m.blob_merkle_root,
            extension: None,
        }
    }
}

// ====================== PARAMS ======================

/// Module parameters (governance-controlled).
///
/// Sensible defaults:
/// - `max_models`: 1000
/// - `max_param_count`: 405 billion
/// - `enable_auto_proof_submission`: true
/// - `default_max_tokens`: 4096
/// - `governance_threshold`: 1
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    /// Hard safety cap on total registered models.
    pub max_models: u64,
    /// Maximum parameter count in billions.
    pub max_param_count: u64,
    /// Whether automatic proof submission is enabled on inference.
    pub enable_auto_proof_submission: bool,
    /// Per-inference safety limit on max tokens.
    pub default_max_tokens: u64,
    /// Required governance votes for model registration.
    pub governance_threshold: u64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            max_models: 1000,
            max_param_count: 405,
            enable_auto_proof_submission: true,
            default_max_tokens: 4096,
            governance_threshold: 1,
        }
    }
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            max_models: p.max_models,
            max_param_count: p.max_param_count,
            enable_auto_proof_submission: p.enable_auto_proof_submission,
            default_max_tokens: p.default_max_tokens,
            governance_threshold: p.governance_threshold,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            max_models: p.max_models,
            max_param_count: p.max_param_count,
            enable_auto_proof_submission: p.enable_auto_proof_submission,
            default_max_tokens: p.default_max_tokens,
            governance_threshold: p.governance_threshold,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn quant_format_roundtrip() {
        for q in [
            QuantFormat::Unspecified,
            QuantFormat::Q4KM,
            QuantFormat::Q5KM,
            QuantFormat::Q80,
            QuantFormat::Fp16,
        ] {
            assert_eq!(QuantFormat::from_proto(q.to_proto()), q);
        }
    }

    #[test]
    fn quant_format_display() {
        assert_eq!(QuantFormat::Q4KM.to_string(), "Q4_K_M");
        assert_eq!(QuantFormat::Q5KM.to_string(), "Q5_K_M");
        assert_eq!(QuantFormat::Q80.to_string(), "Q8_0");
        assert_eq!(QuantFormat::Fp16.to_string(), "FP16");
    }

    #[test]
    fn quant_format_unknown_defaults() {
        assert_eq!(QuantFormat::from_proto(999), QuantFormat::Unspecified);
    }

    #[test]
    fn model_status_roundtrip() {
        for s in [
            ModelStatus::Unspecified,
            ModelStatus::Active,
            ModelStatus::Deprecated,
            ModelStatus::UnderReview,
        ] {
            assert_eq!(ModelStatus::from_proto(s.to_proto()), s);
        }
    }

    #[test]
    fn model_commitment_roundtrip() {
        let model = ModelCommitment {
            model_id: vec![0xAA; 32],
            display_name: "Llama-3.1-8B-Q4".into(),
            quant_format: QuantFormat::Q4KM,
            param_count: 8,
            zk_commitment: vec![0xBB; 32],
            supported_ops: ops::INFER | ops::EMBED,
            registered_at: 1_700_000_000,
            governance_hash: vec![0xCC; 32],
            version: 1,
            blob_merkle_root: vec![0xDD; 32],
        };
        let proto: proto::ModelCommitment = model.clone().into();
        let back: ModelCommitment = proto.into();
        assert_eq!(model, back);
    }

    #[test]
    fn model_commitment_helpers() {
        let model = ModelCommitment {
            supported_ops: ops::INFER | ops::EMBED | ops::VECTOR_SEARCH,
            ..Default::default()
        };
        assert!(model.supports_infer());
        assert!(model.supports_embed());
        assert!(model.supports_vector_search());
        assert!(!model.supports_op(ops::FINE_TUNE));
    }

    #[test]
    fn model_id_hex() {
        let model = ModelCommitment {
            model_id: vec![0xAB, 0xCD],
            ..Default::default()
        };
        assert_eq!(model.model_id_hex(), "abcd");
    }

    #[test]
    fn params_roundtrip() {
        let params = Params {
            max_models: 500,
            max_param_count: 70,
            enable_auto_proof_submission: false,
            default_max_tokens: 8192,
            governance_threshold: 3,
        };
        let proto: proto::Params = params.clone().into();
        let back: Params = proto.into();
        assert_eq!(params, back);
    }

    #[test]
    fn params_defaults() {
        let p = Params::default();
        assert_eq!(p.max_models, 1000);
        assert!(p.enable_auto_proof_submission);
        assert_eq!(p.default_max_tokens, 4096);
    }
}
