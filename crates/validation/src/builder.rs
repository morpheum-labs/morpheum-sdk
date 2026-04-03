//! Fluent builders for the Validation module.
//!
//! This module provides ergonomic, type-safe fluent builders for all validation
//! transaction operations (submit proof, revoke proof). Each
//! builder follows the classic Builder pattern and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{RevokeProofRequest, SubmitProofRequest, UpdateParamsRequest};
use crate::types::{Params, ProofType, ValidationProof};

/// Fluent builder for `UpdateParamsRequest` (governance-only parameter update).
#[derive(Default)]
pub struct UpdateParamsBuilder {
    authority: Option<String>,
    params: Option<Params>,
}

impl UpdateParamsBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn authority(mut self, v: impl Into<String>) -> Self { self.authority = Some(v.into()); self }
    pub fn params(mut self, p: Params) -> Self { self.params = Some(p); self }

    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        Ok(UpdateParamsRequest::new(
            self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?,
            self.params.ok_or_else(|| SdkError::invalid_input("params is required"))?,
        ))
    }
}

/// Fluent builder for submitting a new validation proof.
///
/// # Example
/// ```rust,ignore
/// let request = SubmitProofBuilder::new()
///     .agent_hash("agent-abc")
///     .proof_type(ProofType::TeeAttestation)
///     .score_contribution(8500)
///     .data_hash("proof-data-hash")
///     .verifier_signature(sig_bytes)
///     .build()?;
///
/// let any = request.to_any();
/// ```
#[derive(Default)]
pub struct SubmitProofBuilder {
    agent_hash: Option<String>,
    proof_type: Option<ProofType>,
    score_contribution: Option<u32>,
    data_hash: Option<String>,
    verifier_signature: Option<Vec<u8>>,
}

impl SubmitProofBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the agent hash of the agent being validated.
    pub fn agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.agent_hash = Some(hash.into());
        self
    }

    /// Sets the type of validation proof.
    pub fn proof_type(mut self, proof_type: ProofType) -> Self {
        self.proof_type = Some(proof_type);
        self
    }

    /// Sets the score contribution (0–10 000).
    pub fn score_contribution(mut self, score: u32) -> Self {
        self.score_contribution = Some(score);
        self
    }

    /// Sets the hash of the full proof payload (stored in Persistent Memory).
    pub fn data_hash(mut self, hash: impl Into<String>) -> Self {
        self.data_hash = Some(hash.into());
        self
    }

    /// Sets the verifier's BLS signature.
    pub fn verifier_signature(mut self, sig: Vec<u8>) -> Self {
        self.verifier_signature = Some(sig);
        self
    }

    /// Builds the submit-proof request, performing validation.
    pub fn build(self) -> Result<SubmitProofRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for SubmitProof")
        })?;

        let proof_type = self.proof_type.ok_or_else(|| {
            SdkError::invalid_input("proof_type is required for SubmitProof")
        })?;

        let score_contribution = self.score_contribution.ok_or_else(|| {
            SdkError::invalid_input("score_contribution is required for SubmitProof")
        })?;

        if score_contribution > ValidationProof::MAX_SCORE {
            return Err(SdkError::invalid_input(alloc::format!(
                "score_contribution {} exceeds maximum {}",
                score_contribution,
                ValidationProof::MAX_SCORE,
            )));
        }

        let data_hash = self.data_hash.ok_or_else(|| {
            SdkError::invalid_input("data_hash is required for SubmitProof")
        })?;

        let verifier_signature = self.verifier_signature.ok_or_else(|| {
            SdkError::invalid_input("verifier_signature is required for SubmitProof")
        })?;

        Ok(SubmitProofRequest::new(
            agent_hash,
            proof_type,
            score_contribution,
            data_hash,
            verifier_signature,
        ))
    }
}

/// Fluent builder for revoking a validation proof.
///
/// # Example
/// ```rust,ignore
/// let request = RevokeProofBuilder::new()
///     .proof_id("proof-001")
///     .verifier_agent_hash("verifier-xyz")
///     .verifier_signature(sig_bytes)
///     .reason("Fraudulent backtest data")
///     .build()?;
/// ```
#[derive(Default)]
pub struct RevokeProofBuilder {
    proof_id: Option<String>,
    verifier_agent_hash: Option<String>,
    verifier_signature: Option<Vec<u8>>,
    reason: Option<String>,
}

impl RevokeProofBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the ID of the proof to revoke.
    pub fn proof_id(mut self, id: impl Into<String>) -> Self {
        self.proof_id = Some(id.into());
        self
    }

    /// Sets the verifier (or governance) agent hash performing the revocation.
    pub fn verifier_agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.verifier_agent_hash = Some(hash.into());
        self
    }

    /// Sets the verifier's BLS signature authorising the revocation.
    pub fn verifier_signature(mut self, sig: Vec<u8>) -> Self {
        self.verifier_signature = Some(sig);
        self
    }

    /// Sets the reason for revocation.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Builds the revoke-proof request, performing validation.
    pub fn build(self) -> Result<RevokeProofRequest, SdkError> {
        let proof_id = self.proof_id.ok_or_else(|| {
            SdkError::invalid_input("proof_id is required for RevokeProof")
        })?;

        let verifier_agent_hash = self.verifier_agent_hash.ok_or_else(|| {
            SdkError::invalid_input("verifier_agent_hash is required for RevokeProof")
        })?;

        let verifier_signature = self.verifier_signature.ok_or_else(|| {
            SdkError::invalid_input("verifier_signature is required for RevokeProof")
        })?;

        let reason = self.reason.ok_or_else(|| {
            SdkError::invalid_input("reason is required for RevokeProof")
        })?;

        Ok(RevokeProofRequest::new(
            proof_id,
            verifier_agent_hash,
            verifier_signature,
            reason,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn submit_proof_builder_full_flow() {
        let request = SubmitProofBuilder::new()
            .agent_hash("agent-abc")
            .proof_type(ProofType::TeeAttestation)
            .score_contribution(8500)
            .data_hash("proof-data-hash")
            .verifier_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.agent_hash, "agent-abc");
        assert_eq!(request.proof_type, ProofType::TeeAttestation);
        assert_eq!(request.score_contribution, 8500);
        assert_eq!(request.data_hash, "proof-data-hash");
    }

    #[test]
    fn submit_proof_builder_score_validation() {
        // At maximum — valid
        let result = SubmitProofBuilder::new()
            .agent_hash("agent-abc")
            .proof_type(ProofType::Backtest)
            .score_contribution(10_000)
            .data_hash("hash")
            .verifier_signature(vec![0u8; 64])
            .build();
        assert!(result.is_ok());

        // Over maximum — rejected
        let result = SubmitProofBuilder::new()
            .agent_hash("agent-abc")
            .proof_type(ProofType::Backtest)
            .score_contribution(10_001)
            .data_hash("hash")
            .verifier_signature(vec![0u8; 64])
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn submit_proof_builder_validation_missing_fields() {
        // Missing all fields
        let result = SubmitProofBuilder::new().build();
        assert!(result.is_err());

        // Missing proof_type
        let result = SubmitProofBuilder::new()
            .agent_hash("agent-abc")
            .score_contribution(1000)
            .data_hash("hash")
            .verifier_signature(vec![0u8; 64])
            .build();
        assert!(result.is_err());

        // Missing data_hash
        let result = SubmitProofBuilder::new()
            .agent_hash("agent-abc")
            .proof_type(ProofType::Backtest)
            .score_contribution(1000)
            .verifier_signature(vec![0u8; 64])
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn revoke_proof_builder_full_flow() {
        let request = RevokeProofBuilder::new()
            .proof_id("proof-001")
            .verifier_agent_hash("verifier-xyz")
            .verifier_signature(vec![0u8; 64])
            .reason("Fraudulent backtest data")
            .build()
            .unwrap();

        assert_eq!(request.proof_id, "proof-001");
        assert_eq!(request.verifier_agent_hash, "verifier-xyz");
        assert_eq!(request.reason, "Fraudulent backtest data");
    }

    #[test]
    fn revoke_proof_builder_validation() {
        let result = RevokeProofBuilder::new().build();
        assert!(result.is_err());

        let result = RevokeProofBuilder::new()
            .proof_id("proof-001")
            .build();
        assert!(result.is_err());

        let result = RevokeProofBuilder::new()
            .proof_id("proof-001")
            .verifier_agent_hash("verifier-xyz")
            .verifier_signature(vec![0u8; 64])
            .build();
        assert!(result.is_err()); // missing reason
    }

}
