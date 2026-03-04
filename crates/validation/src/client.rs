//! ValidationClient — the main entry point for validation-related operations
//! in the Morpheum SDK.
//!
//! This client provides high-level, type-safe methods for querying validation
//! proofs by ID, by agent, by type, and module parameters. Transaction
//! operations (submit, revoke, parameter updates) are handled via the fluent
//! builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{
    MorpheumClient, SdkConfig, SdkError, Transport,
};

use crate::{
    requests::{
        QueryProofRequest,
        QueryProofsByAgentRequest,
        QueryProofsByTypeRequest,
    },
    types::{Params, ProofType, ValidationProof},
};

/// Primary client for all validation-related queries.
///
/// Transaction construction (submit, revoke, params) is delegated to the
/// fluent builders in `builder.rs` for maximum ergonomics and type safety.
pub struct ValidationClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl ValidationClient {
    /// Creates a new `ValidationClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a specific validation proof by ID.
    ///
    /// Returns `None` if the proof is not found.
    pub async fn query_proof(
        &self,
        proof_id: impl Into<alloc::string::String>,
    ) -> Result<Option<ValidationProof>, SdkError> {
        let req = QueryProofRequest::new(proof_id);
        let proto_req: morpheum_proto::validation::v1::QueryProofRequest = req.into();

        let path = "/validation.v1.Query/QueryProof";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::validation::v1::QueryProofResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryProofResponse = proto_res.into();
        Ok(response.proof)
    }

    /// Queries all validation proofs for a specific agent (paginated).
    pub async fn query_proofs_by_agent(
        &self,
        agent_hash: impl Into<alloc::string::String>,
        limit: u32,
        offset: u32,
    ) -> Result<(Vec<ValidationProof>, u32), SdkError> {
        let req = QueryProofsByAgentRequest::new(agent_hash, limit, offset);
        let proto_req: morpheum_proto::validation::v1::QueryProofsByAgentRequest = req.into();

        let path = "/validation.v1.Query/QueryProofsByAgent";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::validation::v1::QueryProofsByAgentResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryProofsByAgentResponse = proto_res.into();
        Ok((response.proofs, response.total_count))
    }

    /// Queries validation proofs by type (paginated).
    pub async fn query_proofs_by_type(
        &self,
        proof_type: ProofType,
        limit: u32,
        offset: u32,
    ) -> Result<(Vec<ValidationProof>, u32), SdkError> {
        let req = QueryProofsByTypeRequest::new(proof_type, limit, offset);
        let proto_req: morpheum_proto::validation::v1::QueryProofsByTypeRequest = req.into();

        let path = "/validation.v1.Query/QueryProofsByType";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::validation::v1::QueryProofsByTypeResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryProofsByTypeResponse = proto_res.into();
        Ok((response.proofs, response.total_count))
    }

    /// Queries the current module parameters.
    pub async fn query_params(&self) -> Result<Option<Params>, SdkError> {
        let req = crate::requests::QueryParamsRequest;
        let proto_req: morpheum_proto::validation::v1::QueryParamsRequest = req.into();

        let path = "/validation.v1.Query/QueryParams";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::validation::v1::QueryParamsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryParamsResponse = proto_res.into();
        Ok(response.params)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for ValidationClient {
    fn config(&self) -> &SdkConfig {
        &self.config
    }

    fn transport(&self) -> &dyn Transport {
        &*self.transport
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use morpheum_sdk_core::SdkConfig;

    // Dummy transport for compile-time and basic runtime testing
    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(
            &self,
            _tx_bytes: Vec<u8>,
        ) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!("not needed for validation query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/validation.v1.Query/QueryProof" => {
                    let dummy = morpheum_proto::validation::v1::QueryProofResponse {
                        proof: Some(morpheum_proto::validation::v1::ValidationProof {
                            proof_id: "proof-001".into(),
                            agent_hash: "agent-abc".into(),
                            verifier_agent_hash: "verifier-xyz".into(),
                            proof_type: 3, // TEE_ATTESTATION
                            score_contribution: 8500,
                            timestamp: 1_700_000_000,
                            data_hash: "data-hash".into(),
                            merkle_root: "merkle-root".into(),
                            signature: vec![1, 2, 3],
                        }),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/validation.v1.Query/QueryProofsByAgent" => {
                    let dummy = morpheum_proto::validation::v1::QueryProofsByAgentResponse {
                        proofs: vec![Default::default()],
                        total_count: 1,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/validation.v1.Query/QueryProofsByType" => {
                    let dummy = morpheum_proto::validation::v1::QueryProofsByTypeResponse {
                        proofs: vec![Default::default(), Default::default()],
                        total_count: 2,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/validation.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::validation::v1::QueryParamsResponse {
                        params: Some(morpheum_proto::validation::v1::Params {
                            min_score_contribution: 100,
                            max_proofs_per_agent: 50,
                            require_verifier_signature: true,
                            default_proof_expiry_seconds: 86400,
                        }),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn query_proof_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = ValidationClient::new(config, Box::new(DummyTransport));

        let result = client.query_proof("proof-001").await;
        assert!(result.is_ok());

        let proof = result.unwrap().expect("proof should be present");
        assert_eq!(proof.proof_id, "proof-001");
        assert_eq!(proof.agent_hash, "agent-abc");
        assert_eq!(proof.proof_type, crate::types::ProofType::TeeAttestation);
        assert_eq!(proof.score_contribution, 8500);
        assert!(proof.is_valid_score());
    }

    #[tokio::test]
    async fn query_proofs_by_agent_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = ValidationClient::new(config, Box::new(DummyTransport));

        let (proofs, total) = client
            .query_proofs_by_agent("agent-abc", 10, 0)
            .await
            .unwrap();
        assert_eq!(total, 1);
        assert_eq!(proofs.len(), 1);
    }

    #[tokio::test]
    async fn query_proofs_by_type_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = ValidationClient::new(config, Box::new(DummyTransport));

        let (proofs, total) = client
            .query_proofs_by_type(ProofType::TeeAttestation, 10, 0)
            .await
            .unwrap();
        assert_eq!(total, 2);
        assert_eq!(proofs.len(), 2);
    }

    #[tokio::test]
    async fn query_params_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = ValidationClient::new(config, Box::new(DummyTransport));

        let params = client.query_params().await.unwrap().expect("params should be present");
        assert_eq!(params.min_score_contribution, 100);
        assert_eq!(params.max_proofs_per_agent, 50);
        assert!(params.require_verifier_signature);
        assert_eq!(params.default_proof_expiry_seconds, 86400);
    }
}
