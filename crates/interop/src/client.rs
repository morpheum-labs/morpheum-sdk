//! InteropClient — the main entry point for interop-related operations
//! in the Morpheum SDK.
//!
//! This client provides high-level, type-safe methods for querying bridge
//! request statuses, intent export statuses, proof export statuses, and
//! module parameters. Transaction operations (bridge request, intent export,
//! proof export, parameter updates) are handled via the fluent builders in
//! `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{
    MorpheumClient, SdkConfig, SdkError, Transport,
};

use crate::{
    requests::{
        QueryBridgeRequestRequest,
        QueryIntentExportRequest,
        QueryProofExportRequest,
    },
    types::{BridgeResponse, CrossChainProofPacket, IntentExportPacket, Params},
};

/// Primary client for all interop-related queries.
///
/// Transaction construction (bridge request, intent export, proof export, params)
/// is delegated to the fluent builders in `builder.rs` for maximum ergonomics
/// and type safety.
pub struct InteropClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl InteropClient {
    /// Creates a new `InteropClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries the status of a bridge request by request ID.
    ///
    /// Returns `None` if the bridge request is not found.
    pub async fn query_bridge_request(
        &self,
        request_id: impl Into<alloc::string::String>,
    ) -> Result<Option<BridgeResponse>, SdkError> {
        let req = QueryBridgeRequestRequest::new(request_id);
        let proto_req: morpheum_proto::interop::v1::QueryBridgeRequestRequest = req.into();

        let path = "/interop.v1.Query/QueryBridgeRequest";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::interop::v1::QueryBridgeRequestResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryBridgeRequestResponse = proto_res.into();
        if response.found {
            Ok(response.response)
        } else {
            Ok(None)
        }
    }

    /// Queries the status of an exported intent by intent ID.
    ///
    /// Returns `None` if the intent export is not found.
    /// On success, returns `(intent_packet, target_tx_hash)`.
    pub async fn query_intent_export(
        &self,
        intent_id: impl Into<alloc::string::String>,
    ) -> Result<Option<(IntentExportPacket, alloc::string::String)>, SdkError> {
        let req = QueryIntentExportRequest::new(intent_id);
        let proto_req: morpheum_proto::interop::v1::QueryIntentExportRequest = req.into();

        let path = "/interop.v1.Query/QueryIntentExport";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::interop::v1::QueryIntentExportResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryIntentExportResponse = proto_res.into();
        if response.found {
            Ok(response.packet.map(|pkt| (pkt, response.target_tx_hash)))
        } else {
            Ok(None)
        }
    }

    /// Queries the status of an exported proof by proof ID.
    ///
    /// Returns `None` if the proof export is not found.
    pub async fn query_proof_export(
        &self,
        proof_id: impl Into<alloc::string::String>,
    ) -> Result<Option<CrossChainProofPacket>, SdkError> {
        let req = QueryProofExportRequest::new(proof_id);
        let proto_req: morpheum_proto::interop::v1::QueryProofExportRequest = req.into();

        let path = "/interop.v1.Query/QueryProofExport";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::interop::v1::QueryProofExportResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryProofExportResponse = proto_res.into();
        if response.found {
            Ok(response.packet)
        } else {
            Ok(None)
        }
    }

    /// Queries the current module parameters.
    pub async fn query_params(&self) -> Result<Option<Params>, SdkError> {
        let req = crate::requests::QueryParamsRequest;
        let proto_req: morpheum_proto::interop::v1::QueryParamsRequest = req.into();

        let path = "/interop.v1.Query/QueryParams";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::interop::v1::QueryParamsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryParamsResponse = proto_res.into();
        Ok(response.params)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for InteropClient {
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
    use alloc::string::String;
    use alloc::vec;
    use alloc::vec::Vec;
    use morpheum_sdk_core::SdkConfig;

    // Dummy transport for compile-time and basic runtime testing
    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(
            &self,
            _tx_bytes: Vec<u8>,
        ) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!("not needed for interop query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/interop.v1.Query/QueryBridgeRequest" => {
                    let dummy = morpheum_proto::interop::v1::QueryBridgeRequestResponse {
                        response: Some(morpheum_proto::interop::v1::BridgeResponse {
                            success: true,
                            error: String::new(),
                            target_tx_hash: "0xabc123".into(),
                            processed_at: 1_700_000_000,
                        }),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/interop.v1.Query/QueryIntentExport" => {
                    let dummy = morpheum_proto::interop::v1::QueryIntentExportResponse {
                        packet: Some(morpheum_proto::interop::v1::IntentExportPacket {
                            intent_id: "intent-001".into(),
                            source_agent_hash: "agent-abc".into(),
                            target_chain: "ethereum".into(),
                            intent_data: vec![1, 2, 3],
                            signature: vec![0u8; 64],
                            exported_at: 1_700_000_000,
                            blob_merkle_root: Vec::new(),
                        }),
                        target_tx_hash: "0xdef456".into(),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/interop.v1.Query/QueryProofExport" => {
                    let dummy = morpheum_proto::interop::v1::QueryProofExportResponse {
                        packet: Some(morpheum_proto::interop::v1::CrossChainProofPacket {
                            source_chain: "morpheum".into(),
                            target_chain: "ethereum".into(),
                            agent_hash: "agent-abc".into(),
                            exported_at: 1_700_000_100,
                            merkle_proof: "merkle".into(),
                            blob_merkle_root: Vec::new(),
                            proof: Some(
                                morpheum_proto::interop::v1::cross_chain_proof_packet::Proof::ReputationProof(
                                    morpheum_proto::interop::v1::ReputationProofPacket {
                                        agent_hash: "agent-abc".into(),
                                        score: 90_000,
                                        milestone_level: 3,
                                        is_immortal: false,
                                        timestamp: 1_700_000_000,
                                    },
                                ),
                            ),
                        }),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/interop.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::interop::v1::QueryParamsResponse {
                        params: Some(morpheum_proto::interop::v1::Params {
                            bridging_enabled: true,
                            intent_export_enabled: true,
                            default_proof_ttl_seconds: 86_400,
                            supported_target_chains: "ethereum,solana".into(),
                            enable_reputation_sync: true,
                            max_concurrent_bridge_requests: 100,
                        }),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn query_bridge_request_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = InteropClient::new(config, Box::new(DummyTransport));

        let result = client.query_bridge_request("req-001").await;
        assert!(result.is_ok());

        let response = result.unwrap().expect("response should be present");
        assert!(response.is_ok());
        assert_eq!(response.target_tx_hash, "0xabc123");
    }

    #[tokio::test]
    async fn query_intent_export_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = InteropClient::new(config, Box::new(DummyTransport));

        let result = client.query_intent_export("intent-001").await;
        assert!(result.is_ok());

        let (packet, tx_hash) = result.unwrap().expect("export should be present");
        assert_eq!(packet.intent_id, "intent-001");
        assert_eq!(packet.source_agent_hash, "agent-abc");
        assert_eq!(tx_hash, "0xdef456");
    }

    #[tokio::test]
    async fn query_proof_export_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = InteropClient::new(config, Box::new(DummyTransport));

        let result = client.query_proof_export("proof-001").await;
        assert!(result.is_ok());

        let packet = result.unwrap().expect("packet should be present");
        assert_eq!(packet.source_chain, "morpheum");
        assert_eq!(packet.target_chain, "ethereum");
        assert!(matches!(
            packet.proof,
            Some(crate::types::CrossChainProof::Reputation(_))
        ));
    }

    #[tokio::test]
    async fn query_params_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = InteropClient::new(config, Box::new(DummyTransport));

        let params = client.query_params().await.unwrap().expect("params should be present");
        assert!(params.bridging_enabled);
        assert!(params.intent_export_enabled);
        assert_eq!(params.default_proof_ttl_seconds, 86_400);
        assert_eq!(params.supported_target_chains, "ethereum,solana");
        assert!(params.enable_reputation_sync);
        assert_eq!(params.max_concurrent_bridge_requests, 100);
    }
}
