//! IntentClient — the main entry point for intent-related operations
//! in the Morpheum SDK.
//!
//! This client provides high-level, type-safe methods for querying intents
//! by ID, by agent, active intents, and module parameters. Transaction
//! operations (submit, cancel, parameter updates) are handled via the fluent
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
        QueryActiveIntentsRequest,
        QueryIntentRequest,
        QueryIntentsByAgentRequest,
    },
    types::{AgentIntent, Params},
};

/// Primary client for all intent-related queries.
///
/// Transaction construction (submit, cancel, params) is delegated to the
/// fluent builders in `builder.rs` for maximum ergonomics and type safety.
pub struct IntentClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl IntentClient {
    /// Creates a new `IntentClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a specific intent by its ID.
    ///
    /// Returns `None` if the intent is not found.
    pub async fn query_intent(
        &self,
        intent_id: impl Into<alloc::string::String>,
    ) -> Result<Option<AgentIntent>, SdkError> {
        let req = QueryIntentRequest::new(intent_id);
        let proto_req: morpheum_proto::intent::v1::QueryIntentRequest = req.into();

        let path = "/intent.v1.Query/QueryIntent";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::intent::v1::QueryIntentResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryIntentResponse = proto_res.into();
        Ok(response.intent)
    }

    /// Queries all intents for a specific agent (paginated).
    pub async fn query_intents_by_agent(
        &self,
        agent_hash: impl Into<alloc::string::String>,
        limit: u32,
        offset: u32,
    ) -> Result<(Vec<AgentIntent>, u32), SdkError> {
        let req = QueryIntentsByAgentRequest::new(agent_hash, limit, offset);
        let proto_req: morpheum_proto::intent::v1::QueryIntentsByAgentRequest = req.into();

        let path = "/intent.v1.Query/QueryIntentsByAgent";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::intent::v1::QueryIntentsByAgentResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryIntentsByAgentResponse = proto_res.into();
        Ok((response.intents, response.total_count))
    }

    /// Queries active (pending/executing) intents for an agent.
    pub async fn query_active_intents(
        &self,
        agent_hash: impl Into<alloc::string::String>,
        limit: u32,
    ) -> Result<(Vec<AgentIntent>, u32), SdkError> {
        let req = QueryActiveIntentsRequest::new(agent_hash, limit);
        let proto_req: morpheum_proto::intent::v1::QueryActiveIntentsRequest = req.into();

        let path = "/intent.v1.Query/QueryActiveIntents";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::intent::v1::QueryActiveIntentsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryActiveIntentsResponse = proto_res.into();
        Ok((response.intents, response.total_count))
    }

    /// Queries the current module parameters.
    pub async fn query_params(&self) -> Result<Option<Params>, SdkError> {
        let req = crate::requests::QueryParamsRequest;
        let proto_req: morpheum_proto::intent::v1::QueryParamsRequest = req.into();

        let path = "/intent.v1.Query/QueryParams";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::intent::v1::QueryParamsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryParamsResponse = proto_res.into();
        Ok(response.params)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for IntentClient {
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
            unimplemented!("not needed for intent query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/intent.v1.Query/QueryIntent" => {
                    let dummy = morpheum_proto::intent::v1::QueryIntentResponse {
                        intent: Some(morpheum_proto::intent::v1::AgentIntent {
                            intent_id: "intent-001".into(),
                            agent_hash: "agent-abc".into(),
                            intent_type: 0, // Conditional
                            params: Some(
                                morpheum_proto::intent::v1::agent_intent::Params::Conditional(
                                    morpheum_proto::intent::v1::ConditionalParams {
                                        condition: "price > 50000".into(),
                                        action: "buy 1 BTC".into(),
                                    },
                                ),
                            ),
                            vc_proof_hash: "vc-hash".into(),
                            expiry_timestamp: 1_700_003_600,
                            priority_boost: 5,
                            status: 0, // Pending
                            created_at: 1_700_000_000,
                            blob_merkle_root: alloc::vec::Vec::new(),
                            context_data: alloc::vec::Vec::new(),
                        }),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/intent.v1.Query/QueryIntentsByAgent" => {
                    let dummy = morpheum_proto::intent::v1::QueryIntentsByAgentResponse {
                        intents: vec![Default::default()],
                        total_count: 1,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/intent.v1.Query/QueryActiveIntents" => {
                    let dummy = morpheum_proto::intent::v1::QueryActiveIntentsResponse {
                        intents: vec![],
                        total_count: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/intent.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::intent::v1::QueryParamsResponse {
                        params: Some(morpheum_proto::intent::v1::Params {
                            default_expiry_seconds: 3600,
                            max_concurrent_intents_per_agent: 10,
                            enable_declarative_decomposition: true,
                            scheduler_tick_ms: 500,
                            require_simulation: false,
                            max_decomposition_steps: 20,
                        }),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn query_intent_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = IntentClient::new(config, Box::new(DummyTransport));

        let result = client.query_intent("intent-001").await;
        assert!(result.is_ok());

        let intent = result.unwrap().expect("intent should be present");
        assert_eq!(intent.intent_id, "intent-001");
        assert_eq!(intent.agent_hash, "agent-abc");
        assert_eq!(intent.intent_type, crate::types::IntentType::Conditional);
        assert!(intent.is_active());
    }

    #[tokio::test]
    async fn query_intents_by_agent_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = IntentClient::new(config, Box::new(DummyTransport));

        let (intents, total) = client
            .query_intents_by_agent("agent-abc", 10, 0)
            .await
            .unwrap();
        assert_eq!(total, 1);
        assert_eq!(intents.len(), 1);
    }

    #[tokio::test]
    async fn query_active_intents_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = IntentClient::new(config, Box::new(DummyTransport));

        let (intents, total) = client
            .query_active_intents("agent-abc", 10)
            .await
            .unwrap();
        assert_eq!(total, 0);
        assert!(intents.is_empty());
    }

    #[tokio::test]
    async fn query_params_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = IntentClient::new(config, Box::new(DummyTransport));

        let params = client.query_params().await.unwrap().expect("params should be present");
        assert_eq!(params.default_expiry_seconds, 3600);
        assert_eq!(params.max_concurrent_intents_per_agent, 10);
        assert!(params.enable_declarative_decomposition);
    }
}
