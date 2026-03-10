//! AgentRegistryClient — the main entry point for agent-registry queries
//! in the Morpheum SDK.
//!
//! Provides high-level, type-safe methods for querying agent records, resolving
//! CAIP-10 identifiers, inspecting export status, and reading module parameters.
//! Transaction operations (trigger sync, update params) are handled via the
//! fluent builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

use crate::{
    requests::{
        QueryAgentByCaipRequest,
        QueryAgentRecordRequest,
        QueryExportStatusRequest,
    },
    types::{AgentRecord, ExportStatus, Params},
};

/// Primary client for all agent-registry queries.
///
/// Transaction construction (trigger protocol sync, update params) is delegated
/// to the fluent builders in `builder.rs` for maximum ergonomics and type safety.
pub struct AgentRegistryClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl AgentRegistryClient {
    /// Creates a new `AgentRegistryClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a specific agent record by its hash.
    ///
    /// Returns `None` if the record is not found.
    pub async fn query_agent_record(
        &self,
        agent_hash: impl Into<alloc::string::String>,
    ) -> Result<Option<AgentRecord>, SdkError> {
        let req = QueryAgentRecordRequest::new(agent_hash);
        let proto_req: morpheum_proto::agent_registry::v1::QueryAgentRecordRequest = req.into();

        let path = "/agent_registry.v1.Query/QueryAgentRecord";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res =
            morpheum_proto::agent_registry::v1::QueryAgentRecordResponse::decode(
                response_bytes.as_slice(),
            )
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryAgentRecordResponse = proto_res.into();
        Ok(response.record)
    }

    /// Resolves an agent by CAIP-10 identifier.
    ///
    /// Returns `None` if no agent matches the given CAIP-10 string.
    pub async fn query_agent_by_caip(
        &self,
        caip_id: impl Into<alloc::string::String>,
    ) -> Result<Option<AgentRecord>, SdkError> {
        let req = QueryAgentByCaipRequest::new(caip_id);
        let proto_req: morpheum_proto::agent_registry::v1::QueryAgentByCaipRequest = req.into();

        let path = "/agent_registry.v1.Query/QueryAgentByCAIP";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res =
            morpheum_proto::agent_registry::v1::QueryAgentByCaipResponse::decode(
                response_bytes.as_slice(),
            )
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryAgentByCaipResponse = proto_res.into();
        Ok(response.record)
    }

    /// Queries export/sync status for an agent, optionally filtered by protocols.
    ///
    /// Pass an empty `protocols` list to get all export statuses.
    pub async fn query_export_status(
        &self,
        agent_hash: impl Into<alloc::string::String>,
        protocols: Vec<alloc::string::String>,
    ) -> Result<Vec<ExportStatus>, SdkError> {
        let req = QueryExportStatusRequest::new(agent_hash, protocols);
        let proto_req: morpheum_proto::agent_registry::v1::QueryExportStatusRequest = req.into();

        let path = "/agent_registry.v1.Query/QueryExportStatus";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res =
            morpheum_proto::agent_registry::v1::QueryExportStatusResponse::decode(
                response_bytes.as_slice(),
            )
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryExportStatusResponse = proto_res.into();
        Ok(response.export_statuses)
    }

    /// Queries the current module parameters.
    pub async fn query_params(&self) -> Result<Option<Params>, SdkError> {
        let req = crate::requests::QueryParamsRequest;
        let proto_req: morpheum_proto::agent_registry::v1::QueryParamsRequest = req.into();

        let path = "/agent_registry.v1.Query/QueryParams";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res =
            morpheum_proto::agent_registry::v1::QueryParamsResponse::decode(
                response_bytes.as_slice(),
            )
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryParamsResponse = proto_res.into();
        Ok(response.params)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for AgentRegistryClient {
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

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(
            &self,
            _tx_bytes: Vec<u8>,
        ) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!("not needed for agent_registry query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/agent_registry.v1.Query/QueryAgentRecord" => {
                    let dummy = morpheum_proto::agent_registry::v1::QueryAgentRecordResponse {
                        record: Some(morpheum_proto::agent_registry::v1::AgentRecord {
                            agent_hash: vec![0xAA; 32],
                            version: 3,
                            ..Default::default()
                        }),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/agent_registry.v1.Query/QueryAgentByCAIP" => {
                    let dummy = morpheum_proto::agent_registry::v1::QueryAgentByCaipResponse {
                        record: Some(morpheum_proto::agent_registry::v1::AgentRecord {
                            agent_hash: vec![0xBB; 32],
                            version: 1,
                            ..Default::default()
                        }),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/agent_registry.v1.Query/QueryExportStatus" => {
                    let dummy = morpheum_proto::agent_registry::v1::QueryExportStatusResponse {
                        export_statuses: vec![
                            morpheum_proto::agent_registry::v1::ExportStatus {
                                protocol: "erc8004".into(),
                                success: true,
                                ..Default::default()
                            },
                        ],
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/agent_registry.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::agent_registry::v1::QueryParamsResponse {
                        params: Some(morpheum_proto::agent_registry::v1::Params {
                            max_metadata_size_bytes: 1_048_576,
                            sync_timeout_ms: 100,
                            enable_auto_export: true,
                            default_visibility: 3,
                            max_export_retries: 3,
                        }),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn query_agent_record_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = AgentRegistryClient::new(config, Box::new(DummyTransport));

        let record = client
            .query_agent_record("aa".repeat(32))
            .await
            .unwrap()
            .expect("record should be present");
        assert_eq!(record.agent_hash, vec![0xAA; 32]);
        assert_eq!(record.version, 3);
    }

    #[tokio::test]
    async fn query_agent_by_caip_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = AgentRegistryClient::new(config, Box::new(DummyTransport));

        let record = client
            .query_agent_by_caip("morpheum:1:actor-0xDEAD")
            .await
            .unwrap()
            .expect("record should be present");
        assert_eq!(record.agent_hash, vec![0xBB; 32]);
    }

    #[tokio::test]
    async fn query_export_status_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = AgentRegistryClient::new(config, Box::new(DummyTransport));

        let statuses = client
            .query_export_status("aa".repeat(32), vec![])
            .await
            .unwrap();
        assert_eq!(statuses.len(), 1);
        assert_eq!(statuses[0].protocol, "erc8004");
        assert!(statuses[0].success);
    }

    #[tokio::test]
    async fn query_params_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = AgentRegistryClient::new(config, Box::new(DummyTransport));

        let params = client
            .query_params()
            .await
            .unwrap()
            .expect("params should be present");
        assert_eq!(params.max_metadata_size_bytes, 1_048_576);
        assert!(params.enable_auto_export);
    }
}
