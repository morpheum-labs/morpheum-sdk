//! MemoryClient — the main entry point for memory-related operations
//! in the Morpheum SDK.
//!
//! This client provides high-level, type-safe methods for querying memory
//! entries, memory roots, and module parameters. Transaction operations
//! (store, update, delete, parameter updates) are handled via the fluent
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
        QueryMemoryEntriesByAgentRequest,
        QueryMemoryEntryRequest,
        QueryMemoryRootRequest,
    },
    types::{MemoryEntry, MemoryRoot, Params},
};

/// Primary client for all memory-related queries.
///
/// Transaction construction (store, update, delete, params) is delegated to the
/// fluent builders in `builder.rs` for maximum ergonomics and type safety.
pub struct MemoryClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl MemoryClient {
    /// Creates a new `MemoryClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a specific memory entry by agent hash and key.
    ///
    /// Returns `None` if the entry is not found.
    pub async fn query_entry(
        &self,
        agent_hash: impl Into<alloc::string::String>,
        key: impl Into<alloc::string::String>,
    ) -> Result<Option<MemoryEntry>, SdkError> {
        let req = QueryMemoryEntryRequest::new(agent_hash, key);
        let proto_req: morpheum_proto::memory::v1::QueryMemoryEntryRequest = req.into();

        let path = "/memory.v1.Query/QueryMemoryEntry";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::memory::v1::QueryMemoryEntryResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryMemoryEntryResponse = proto_res.into();
        Ok(response.entry)
    }

    /// Queries all memory entries for a specific agent (paginated).
    pub async fn query_entries_by_agent(
        &self,
        agent_hash: impl Into<alloc::string::String>,
        limit: u32,
        offset: u32,
    ) -> Result<(Vec<MemoryEntry>, u32), SdkError> {
        let req = QueryMemoryEntriesByAgentRequest::new(agent_hash, limit, offset);
        let proto_req: morpheum_proto::memory::v1::QueryMemoryEntriesByAgentRequest = req.into();

        let path = "/memory.v1.Query/QueryMemoryEntriesByAgent";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::memory::v1::QueryMemoryEntriesByAgentResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryMemoryEntriesByAgentResponse = proto_res.into();
        Ok((response.entries, response.total_count))
    }

    /// Queries the memory root for an agent.
    ///
    /// Returns `None` if the agent has no memory root.
    pub async fn query_memory_root(
        &self,
        agent_hash: impl Into<alloc::string::String>,
    ) -> Result<Option<MemoryRoot>, SdkError> {
        let req = QueryMemoryRootRequest::new(agent_hash);
        let proto_req: morpheum_proto::memory::v1::QueryMemoryRootRequest = req.into();

        let path = "/memory.v1.Query/QueryMemoryRoot";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::memory::v1::QueryMemoryRootResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryMemoryRootResponse = proto_res.into();
        Ok(response.root)
    }

    /// Queries the current module parameters.
    pub async fn query_params(&self) -> Result<Option<Params>, SdkError> {
        let req = crate::requests::QueryParamsRequest;
        let proto_req: morpheum_proto::memory::v1::QueryParamsRequest = req.into();

        let path = "/memory.v1.Query/QueryParams";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::memory::v1::QueryParamsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryParamsResponse = proto_res.into();
        Ok(response.params)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for MemoryClient {
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
            unimplemented!("not needed for memory query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/memory.v1.Query/QueryMemoryEntry" => {
                    let dummy = morpheum_proto::memory::v1::QueryMemoryEntryResponse {
                        entry: Some(morpheum_proto::memory::v1::MemoryEntry {
                            agent_hash: "agent-abc".into(),
                            key: "strategy/v1".into(),
                            value: vec![1, 2, 3],
                            entry_type: 1, // Semantic
                            timestamp: 1_700_000_000,
                            expires_at: 0,
                            version: "1.0".into(),
                        }),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/memory.v1.Query/QueryMemoryEntriesByAgent" => {
                    let dummy = morpheum_proto::memory::v1::QueryMemoryEntriesByAgentResponse {
                        entries: vec![Default::default()],
                        total_count: 1,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/memory.v1.Query/QueryMemoryRoot" => {
                    let dummy = morpheum_proto::memory::v1::QueryMemoryRootResponse {
                        root: Some(morpheum_proto::memory::v1::MemoryRoot {
                            agent_hash: "agent-abc".into(),
                            root_hash: "deadbeef".into(),
                            last_updated: 1_700_000_000,
                            entry_count: 42,
                            total_size_bytes: 1_048_576,
                        }),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/memory.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::memory::v1::QueryParamsResponse {
                        params: Some(morpheum_proto::memory::v1::Params {
                            max_memory_per_agent_bytes: 10_000_000,
                            default_entry_ttl_seconds: 86400,
                            enable_vector_search: true,
                            max_entries_per_agent: 1000,
                            default_vector_dimension: 512,
                            enable_auto_pruning: false,
                        }),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn query_entry_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = MemoryClient::new(config, Box::new(DummyTransport));

        let result = client.query_entry("agent-abc", "strategy/v1").await;
        assert!(result.is_ok());

        let entry = result.unwrap().expect("entry should be present");
        assert_eq!(entry.agent_hash, "agent-abc");
        assert_eq!(entry.key, "strategy/v1");
        assert_eq!(entry.entry_type, crate::types::MemoryEntryType::Semantic);
    }

    #[tokio::test]
    async fn query_entries_by_agent_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = MemoryClient::new(config, Box::new(DummyTransport));

        let (entries, total) = client
            .query_entries_by_agent("agent-abc", 10, 0)
            .await
            .unwrap();
        assert_eq!(total, 1);
        assert_eq!(entries.len(), 1);
    }

    #[tokio::test]
    async fn query_memory_root_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = MemoryClient::new(config, Box::new(DummyTransport));

        let root = client
            .query_memory_root("agent-abc")
            .await
            .unwrap()
            .expect("root should be present");
        assert_eq!(root.agent_hash, "agent-abc");
        assert_eq!(root.entry_count, 42);
        assert_eq!(root.total_size_bytes, 1_048_576);
    }

    #[tokio::test]
    async fn query_params_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = MemoryClient::new(config, Box::new(DummyTransport));

        let params = client.query_params().await.unwrap().expect("params should be present");
        assert_eq!(params.max_memory_per_agent_bytes, 10_000_000);
        assert_eq!(params.default_entry_ttl_seconds, 86400);
        assert!(params.enable_vector_search);
        assert_eq!(params.max_entries_per_agent, 1000);
        assert_eq!(params.default_vector_dimension, 512);
        assert!(!params.enable_auto_pruning);
    }
}
