//! Fluent builders for the Interop module.
//!
//! This module provides ergonomic, type-safe fluent builders for all interop
//! transaction operations (bridge request, intent export, proof export,
//! parameter updates). Each builder follows the classic Builder pattern and
//! returns the corresponding request type from `requests.rs` for seamless
//! integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    ExportIntentRequest, ExportProofRequest, SubmitBridgeRequest,
};
use crate::types::{
    BridgePayload, BridgeRequestData, CrossChainProofPacket,
    IntentExportPacket,
};

/// Fluent builder for submitting a general bridge request.
///
/// # Example
/// ```rust,ignore
/// let request = BridgeRequestBuilder::new()
///     .source_chain("morpheum")
///     .target_chain("ethereum")
///     .proof_payload(proof_packet)
///     .signer(signer_bytes)
///     .build()?;
///
/// let any = request.to_any();
/// ```
#[derive(Default)]
pub struct BridgeRequestBuilder {
    source_chain: Option<String>,
    target_chain: Option<String>,
    payload: Option<BridgePayload>,
    signer: Option<Vec<u8>>,
}

impl BridgeRequestBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the source chain identifier.
    pub fn source_chain(mut self, chain: impl Into<String>) -> Self {
        self.source_chain = Some(chain.into());
        self
    }

    /// Sets the target chain identifier.
    pub fn target_chain(mut self, chain: impl Into<String>) -> Self {
        self.target_chain = Some(chain.into());
        self
    }

    /// Sets a proof packet as the bridge payload.
    pub fn proof_payload(mut self, packet: CrossChainProofPacket) -> Self {
        self.payload = Some(BridgePayload::Proof(packet));
        self
    }

    /// Sets an intent packet as the bridge payload.
    pub fn intent_payload(mut self, packet: IntentExportPacket) -> Self {
        self.payload = Some(BridgePayload::Intent(packet));
        self
    }

    /// Sets the signer bytes.
    pub fn signer(mut self, signer: Vec<u8>) -> Self {
        self.signer = Some(signer);
        self
    }

    /// Builds the bridge request, performing validation.
    pub fn build(self) -> Result<SubmitBridgeRequest, SdkError> {
        let source_chain = self.source_chain.ok_or_else(|| {
            SdkError::invalid_input("source_chain is required for BridgeRequest")
        })?;

        let target_chain = self.target_chain.ok_or_else(|| {
            SdkError::invalid_input("target_chain is required for BridgeRequest")
        })?;

        let payload = self.payload.ok_or_else(|| {
            SdkError::invalid_input("payload (proof or intent) is required for BridgeRequest")
        })?;

        let signer = self.signer.ok_or_else(|| {
            SdkError::invalid_input("signer is required for BridgeRequest")
        })?;

        Ok(SubmitBridgeRequest::new(
            BridgeRequestData {
                source_chain,
                target_chain,
                payload: Some(payload),
            },
            signer,
        ))
    }
}

/// Fluent builder for exporting an intent for cross-chain execution.
///
/// # Example
/// ```rust,ignore
/// let request = ExportIntentBuilder::new()
///     .intent_id("intent-001")
///     .source_agent_hash("agent-abc")
///     .target_chain("ethereum")
///     .intent_data(serialized_intent)
///     .signature(sig_bytes)
///     .signer(signer_bytes)
///     .build()?;
/// ```
#[derive(Default)]
pub struct ExportIntentBuilder {
    intent_id: Option<String>,
    source_agent_hash: Option<String>,
    target_chain: Option<String>,
    intent_data: Option<Vec<u8>>,
    signature: Option<Vec<u8>>,
    exported_at: Option<u64>,
    signer: Option<Vec<u8>>,
}

impl ExportIntentBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the intent identifier.
    pub fn intent_id(mut self, id: impl Into<String>) -> Self {
        self.intent_id = Some(id.into());
        self
    }

    /// Sets the source agent hash.
    pub fn source_agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.source_agent_hash = Some(hash.into());
        self
    }

    /// Sets the target chain.
    pub fn target_chain(mut self, chain: impl Into<String>) -> Self {
        self.target_chain = Some(chain.into());
        self
    }

    /// Sets the serialized intent data.
    pub fn intent_data(mut self, data: Vec<u8>) -> Self {
        self.intent_data = Some(data);
        self
    }

    /// Sets the export signature.
    pub fn signature(mut self, sig: Vec<u8>) -> Self {
        self.signature = Some(sig);
        self
    }

    /// Sets the export timestamp (optional; defaults to 0).
    pub fn exported_at(mut self, ts: u64) -> Self {
        self.exported_at = Some(ts);
        self
    }

    /// Sets the signer bytes.
    pub fn signer(mut self, signer: Vec<u8>) -> Self {
        self.signer = Some(signer);
        self
    }

    /// Builds the export-intent request, performing validation.
    pub fn build(self) -> Result<ExportIntentRequest, SdkError> {
        let intent_id = self.intent_id.ok_or_else(|| {
            SdkError::invalid_input("intent_id is required for ExportIntent")
        })?;

        let source_agent_hash = self.source_agent_hash.ok_or_else(|| {
            SdkError::invalid_input("source_agent_hash is required for ExportIntent")
        })?;

        let target_chain = self.target_chain.ok_or_else(|| {
            SdkError::invalid_input("target_chain is required for ExportIntent")
        })?;

        let intent_data = self.intent_data.ok_or_else(|| {
            SdkError::invalid_input("intent_data is required for ExportIntent")
        })?;

        let signature = self.signature.ok_or_else(|| {
            SdkError::invalid_input("signature is required for ExportIntent")
        })?;

        let signer = self.signer.ok_or_else(|| {
            SdkError::invalid_input("signer is required for ExportIntent")
        })?;

        Ok(ExportIntentRequest::new(
            IntentExportPacket {
                intent_id,
                source_agent_hash,
                target_chain,
                intent_data,
                signature,
                exported_at: self.exported_at.unwrap_or(0),
            },
            signer,
        ))
    }
}

/// Fluent builder for exporting a proof for cross-chain verification.
///
/// # Example
/// ```rust,ignore
/// let request = ExportProofBuilder::new()
///     .proof_packet(proof_packet)
///     .signer(signer_bytes)
///     .build()?;
/// ```
#[derive(Default)]
pub struct ExportProofBuilder {
    proof_packet: Option<CrossChainProofPacket>,
    signer: Option<Vec<u8>>,
}

impl ExportProofBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the cross-chain proof packet to export.
    pub fn proof_packet(mut self, packet: CrossChainProofPacket) -> Self {
        self.proof_packet = Some(packet);
        self
    }

    /// Sets the signer bytes.
    pub fn signer(mut self, signer: Vec<u8>) -> Self {
        self.signer = Some(signer);
        self
    }

    /// Builds the export-proof request, performing validation.
    pub fn build(self) -> Result<ExportProofRequest, SdkError> {
        let proof_packet = self.proof_packet.ok_or_else(|| {
            SdkError::invalid_input("proof_packet is required for ExportProof")
        })?;

        let signer = self.signer.ok_or_else(|| {
            SdkError::invalid_input("signer is required for ExportProof")
        })?;

        Ok(ExportProofRequest::new(proof_packet, signer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{CrossChainProof, ReputationProofPacket};
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

    #[test]
    fn bridge_request_builder_with_proof() {
        let request = BridgeRequestBuilder::new()
            .source_chain("morpheum")
            .target_chain("ethereum")
            .proof_payload(sample_proof_packet())
            .signer(vec![0u8; 33])
            .build()
            .unwrap();

        assert_eq!(request.request.source_chain, "morpheum");
        assert_eq!(request.request.target_chain, "ethereum");
        assert!(matches!(
            request.request.payload,
            Some(BridgePayload::Proof(_))
        ));
    }

    #[test]
    fn bridge_request_builder_with_intent() {
        let intent = IntentExportPacket {
            intent_id: "intent-001".into(),
            source_agent_hash: "agent-abc".into(),
            target_chain: "ethereum".into(),
            intent_data: vec![1, 2, 3],
            signature: vec![0u8; 64],
            exported_at: 1_700_000_000,
        };

        let request = BridgeRequestBuilder::new()
            .source_chain("morpheum")
            .target_chain("ethereum")
            .intent_payload(intent)
            .signer(vec![0u8; 33])
            .build()
            .unwrap();

        assert!(matches!(
            request.request.payload,
            Some(BridgePayload::Intent(_))
        ));
    }

    #[test]
    fn bridge_request_builder_validation() {
        // Missing all fields
        let result = BridgeRequestBuilder::new().build();
        assert!(result.is_err());

        // Missing payload
        let result = BridgeRequestBuilder::new()
            .source_chain("morpheum")
            .target_chain("ethereum")
            .signer(vec![0u8; 33])
            .build();
        assert!(result.is_err());

        // Missing signer
        let result = BridgeRequestBuilder::new()
            .source_chain("morpheum")
            .target_chain("ethereum")
            .proof_payload(sample_proof_packet())
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn export_intent_builder_full_flow() {
        let request = ExportIntentBuilder::new()
            .intent_id("intent-001")
            .source_agent_hash("agent-abc")
            .target_chain("ethereum")
            .intent_data(vec![1, 2, 3])
            .signature(vec![0u8; 64])
            .exported_at(1_700_000_000)
            .signer(vec![0u8; 33])
            .build()
            .unwrap();

        assert_eq!(request.intent_packet.intent_id, "intent-001");
        assert_eq!(request.intent_packet.source_agent_hash, "agent-abc");
        assert_eq!(request.intent_packet.target_chain, "ethereum");
        assert_eq!(request.intent_packet.exported_at, 1_700_000_000);
    }

    #[test]
    fn export_intent_builder_defaults() {
        let request = ExportIntentBuilder::new()
            .intent_id("intent-001")
            .source_agent_hash("agent-abc")
            .target_chain("ethereum")
            .intent_data(vec![1])
            .signature(vec![0u8; 64])
            .signer(vec![0u8; 33])
            .build()
            .unwrap();

        // exported_at defaults to 0
        assert_eq!(request.intent_packet.exported_at, 0);
    }

    #[test]
    fn export_intent_builder_validation() {
        // Missing all fields
        let result = ExportIntentBuilder::new().build();
        assert!(result.is_err());

        // Missing intent_data
        let result = ExportIntentBuilder::new()
            .intent_id("intent-001")
            .source_agent_hash("agent-abc")
            .target_chain("ethereum")
            .signature(vec![0u8; 64])
            .signer(vec![0u8; 33])
            .build();
        assert!(result.is_err());

        // Missing signer
        let result = ExportIntentBuilder::new()
            .intent_id("intent-001")
            .source_agent_hash("agent-abc")
            .target_chain("ethereum")
            .intent_data(vec![1])
            .signature(vec![0u8; 64])
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn export_proof_builder_full_flow() {
        let request = ExportProofBuilder::new()
            .proof_packet(sample_proof_packet())
            .signer(vec![0u8; 33])
            .build()
            .unwrap();

        assert_eq!(request.proof_packet.source_chain, "morpheum");
        assert_eq!(request.proof_packet.agent_hash, "agent-abc");
    }

    #[test]
    fn export_proof_builder_validation() {
        // Missing all fields
        let result = ExportProofBuilder::new().build();
        assert!(result.is_err());

        // Missing signer
        let result = ExportProofBuilder::new()
            .proof_packet(sample_proof_packet())
            .build();
        assert!(result.is_err());
    }

}
