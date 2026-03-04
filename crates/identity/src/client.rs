//! IdentityClient — the main entry point for all Agent Identity operations
//! in the Morpheum SDK.
//!
//! This client provides high-level, type-safe methods for querying agent
//! identities, metadata cards, statuses, and module parameters. Transaction
//! operations (register, transfer, update, burn) are handled via the fluent
//! builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{
    MorpheumClient, SdkConfig, SdkError, Transport,
};

use crate::{
    requests::{
        QueryAgentRequest,
        QueryAgentByOwnerRequest,
        QueryMetadataCardRequest,
        QueryAgentStatusRequest,
        QueryParamsRequest,
    },
    types::{AgentIdentity, AgentMetadataCard, AgentStatus, Params},
};

/// Primary client for all Identity-related operations.
///
/// Focused on queries. Transaction construction is delegated to the
/// fluent builders (`RegisterAgentBuilder`, `TransferOwnershipBuilder`, etc.)
/// for maximum ergonomics and type safety.
pub struct IdentityClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl IdentityClient {
    /// Creates a new `IdentityClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries an agent identity by its hash.
    pub async fn query_agent(
        &self,
        agent_hash: impl Into<String>,
    ) -> Result<AgentIdentity, SdkError> {
        self.execute_query_agent(QueryAgentRequest::by_hash(agent_hash)).await
    }

    /// Queries an agent identity by its DID.
    pub async fn query_agent_by_did(
        &self,
        did: impl Into<String>,
    ) -> Result<AgentIdentity, SdkError> {
        self.execute_query_agent(QueryAgentRequest::by_did(did)).await
    }

    /// Shared implementation for both agent lookup variants.
    async fn execute_query_agent(
        &self,
        req: QueryAgentRequest,
    ) -> Result<AgentIdentity, SdkError> {
        let proto_req: morpheum_proto::identity::v1::QueryAgentRequest = req.into();
        let data = proto_req.encode_to_vec();

        let response_bytes = self
            .query("/identity.v1.Query/QueryAgent", data)
            .await?;

        let proto_res = morpheum_proto::identity::v1::QueryAgentResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        if !proto_res.found {
            return Err(SdkError::transport("agent not found"));
        }

        proto_res
            .identity
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("identity field missing in response"))
    }

    /// Queries all agents owned by a specific owner (paginated).
    ///
    /// Returns `(identities, total_count)`.
    pub async fn query_agents_by_owner(
        &self,
        owner_agent_hash: impl Into<String>,
        limit: u32,
        offset: u32,
    ) -> Result<(Vec<AgentIdentity>, u32), SdkError> {
        let req = QueryAgentByOwnerRequest::new(owner_agent_hash, limit, offset);
        let proto_req: morpheum_proto::identity::v1::QueryAgentByOwnerRequest = req.into();
        let data = proto_req.encode_to_vec();

        let response_bytes = self
            .query("/identity.v1.Query/QueryAgentByOwner", data)
            .await?;

        let proto_res = morpheum_proto::identity::v1::QueryAgentByOwnerResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let identities = proto_res.identities.into_iter().map(Into::into).collect();
        Ok((identities, proto_res.total_count))
    }

    /// Queries the metadata card for an agent.
    pub async fn query_metadata_card(
        &self,
        agent_hash: impl Into<String>,
    ) -> Result<AgentMetadataCard, SdkError> {
        let req = QueryMetadataCardRequest::new(agent_hash);
        let proto_req: morpheum_proto::identity::v1::QueryMetadataCardRequest = req.into();
        let data = proto_req.encode_to_vec();

        let response_bytes = self
            .query("/identity.v1.Query/QueryMetadataCard", data)
            .await?;

        let proto_res = morpheum_proto::identity::v1::QueryMetadataCardResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        if !proto_res.found {
            return Err(SdkError::transport("metadata card not found"));
        }

        proto_res
            .metadata
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("metadata field missing in response"))
    }

    /// Queries the current status of an agent.
    ///
    /// Returns `(status, last_updated)`.
    pub async fn query_agent_status(
        &self,
        agent_hash: impl Into<String>,
    ) -> Result<(AgentStatus, u64), SdkError> {
        let req = QueryAgentStatusRequest::new(agent_hash);
        let proto_req: morpheum_proto::identity::v1::QueryAgentStatusRequest = req.into();
        let data = proto_req.encode_to_vec();

        let response_bytes = self
            .query("/identity.v1.Query/QueryAgentStatus", data)
            .await?;

        let proto_res = morpheum_proto::identity::v1::QueryAgentStatusResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok((
            AgentStatus::from_proto(proto_res.status),
            proto_res.last_updated,
        ))
    }

    /// Queries the current identity module parameters.
    pub async fn query_params(&self) -> Result<Params, SdkError> {
        let proto_req: morpheum_proto::identity::v1::QueryParamsRequest =
            QueryParamsRequest.into();
        let data = proto_req.encode_to_vec();

        let response_bytes = self
            .query("/identity.v1.Query/QueryParams", data)
            .await?;

        let proto_res = morpheum_proto::identity::v1::QueryParamsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .params
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("params field missing in response"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for IdentityClient {
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
    use morpheum_sdk_core::{BroadcastResult, SdkConfig};

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError> {
            unimplemented!("not needed for identity query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/identity.v1.Query/QueryAgent" => {
                    let dummy = morpheum_proto::identity::v1::QueryAgentResponse {
                        identity: Some(Default::default()),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/identity.v1.Query/QueryAgentStatus" => {
                    let dummy = morpheum_proto::identity::v1::QueryAgentStatusResponse {
                        status: 0,
                        last_updated: 1_700_000_000,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/identity.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::identity::v1::QueryParamsResponse {
                        params: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn identity_client_query_agent_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = IdentityClient::new(config, Box::new(DummyTransport));

        let result = client.query_agent("agent_hash_123").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn identity_client_query_agent_by_did_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = IdentityClient::new(config, Box::new(DummyTransport));

        let result = client.query_agent_by_did("did:agent:abc123").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn identity_client_query_status_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = IdentityClient::new(config, Box::new(DummyTransport));

        let (status, last_updated) = client.query_agent_status("agent_hash").await.unwrap();
        assert_eq!(status, AgentStatus::Active);
        assert_eq!(last_updated, 1_700_000_000);
    }

    #[tokio::test]
    async fn identity_client_query_params_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = IdentityClient::new(config, Box::new(DummyTransport));

        let result = client.query_params().await;
        assert!(result.is_ok());
    }
}
