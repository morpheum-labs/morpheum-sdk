//! DirectoryClient — the main entry point for directory-related operations
//! in the Morpheum SDK.
//!
//! This client provides high-level, type-safe methods for querying agent
//! directory profiles, filtered/paginated profile listings, and module
//! parameters. Transaction operations (profile update, visibility update,
//! parameter updates) are handled via the fluent builders in `builder.rs`
//! + `TxBuilder`.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{
    MorpheumClient, SdkConfig, SdkError, Transport,
};

use crate::{
    requests::{
        QueryDirectoryProfileRequest,
        QueryDirectoryProfilesRequest,
    },
    types::{AgentDirectoryProfile, DirectoryFilter, Params},
};

/// Primary client for all directory-related queries.
///
/// Transaction construction (profile update, visibility update, params) is
/// delegated to the fluent builders in `builder.rs` for maximum ergonomics
/// and type safety.
pub struct DirectoryClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl DirectoryClient {
    /// Creates a new `DirectoryClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a specific agent's directory profile by agent hash.
    ///
    /// Returns `None` if the profile is not found.
    pub async fn query_profile(
        &self,
        agent_hash: impl Into<alloc::string::String>,
    ) -> Result<Option<AgentDirectoryProfile>, SdkError> {
        let req = QueryDirectoryProfileRequest::new(agent_hash);
        let proto_req: morpheum_proto::directory::v1::QueryDirectoryProfileRequest = req.into();

        let path = "/directory.v1.Query/QueryDirectoryProfile";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::directory::v1::QueryDirectoryProfileResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryDirectoryProfileResponse = proto_res.into();
        Ok(response.profile)
    }

    /// Queries multiple directory profiles with optional filter and pagination.
    ///
    /// Returns a tuple of `(profiles, total_count)`.
    pub async fn query_profiles(
        &self,
        limit: u32,
        offset: u32,
        filter: Option<DirectoryFilter>,
    ) -> Result<(Vec<AgentDirectoryProfile>, u32), SdkError> {
        let mut req = QueryDirectoryProfilesRequest::new(limit, offset);
        if let Some(f) = filter {
            req = req.with_filter(f);
        }
        let proto_req: morpheum_proto::directory::v1::QueryDirectoryProfilesRequest = req.into();

        let path = "/directory.v1.Query/QueryDirectoryProfiles";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::directory::v1::QueryDirectoryProfilesResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryDirectoryProfilesResponse = proto_res.into();
        Ok((response.profiles, response.total_count))
    }

    /// Queries the current module parameters.
    pub async fn query_params(&self) -> Result<Option<Params>, SdkError> {
        let req = crate::requests::QueryParamsRequest;
        let proto_req: morpheum_proto::directory::v1::QueryParamsRequest = req.into();

        let path = "/directory.v1.Query/QueryParams";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::directory::v1::QueryParamsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryParamsResponse = proto_res.into();
        Ok(response.params)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for DirectoryClient {
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
            unimplemented!("not needed for directory query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/directory.v1.Query/QueryDirectoryProfile" => {
                    let dummy = morpheum_proto::directory::v1::QueryDirectoryProfileResponse {
                        profile: Some(morpheum_proto::directory::v1::AgentDirectoryProfile {
                            agent_hash: "agent-abc".into(),
                            display_name: "AlphaBot".into(),
                            description: "Trading agent".into(),
                            tags: "hft,btc".into(),
                            reputation_score: 90_000,
                            milestone_level: 3,
                            is_immortal: false,
                            capabilities: 0xFF,
                            total_volume_usd_30d: 500_000,
                            success_rate_bps: 9200,
                            latest_intent_summary: "TWAP buy".into(),
                            memory_health_score: 80,
                            visibility: 0, // PUBLIC
                            last_updated: 1_700_000_000,
                            ..Default::default()
                        }),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/directory.v1.Query/QueryDirectoryProfiles" => {
                    let dummy = morpheum_proto::directory::v1::QueryDirectoryProfilesResponse {
                        profiles: vec![Default::default(), Default::default()],
                        total_count: 2,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/directory.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::directory::v1::QueryParamsResponse {
                        params: Some(morpheum_proto::directory::v1::Params {
                            default_query_limit: 50,
                            profile_cache_ttl_seconds: 300,
                            enable_semantic_search: true,
                            max_subscriptions_per_agent: 10,
                            public_directory_enabled: true,
                            min_reputation_for_public: 0,
                        }),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn query_profile_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = DirectoryClient::new(config, Box::new(DummyTransport));

        let result = client.query_profile("agent-abc").await;
        assert!(result.is_ok());

        let profile = result.unwrap().expect("profile should be present");
        assert_eq!(profile.agent_hash, "agent-abc");
        assert_eq!(profile.display_name, "AlphaBot");
        assert_eq!(profile.visibility, crate::types::VisibilityLevel::Public);
    }

    #[tokio::test]
    async fn query_profiles_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = DirectoryClient::new(config, Box::new(DummyTransport));

        let (profiles, total) = client
            .query_profiles(20, 0, None)
            .await
            .unwrap();
        assert_eq!(total, 2);
        assert_eq!(profiles.len(), 2);
    }

    #[tokio::test]
    async fn query_profiles_with_filter_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = DirectoryClient::new(config, Box::new(DummyTransport));

        let filter = DirectoryFilter {
            min_reputation: 50_000,
            tags: "hft".into(),
            ..Default::default()
        };

        let (profiles, total) = client
            .query_profiles(10, 0, Some(filter))
            .await
            .unwrap();
        assert_eq!(total, 2);
        assert_eq!(profiles.len(), 2);
    }

    #[tokio::test]
    async fn query_params_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = DirectoryClient::new(config, Box::new(DummyTransport));

        let params = client.query_params().await.unwrap().expect("params should be present");
        assert_eq!(params.default_query_limit, 50);
        assert_eq!(params.profile_cache_ttl_seconds, 300);
        assert!(params.enable_semantic_search);
        assert_eq!(params.max_subscriptions_per_agent, 10);
        assert!(params.public_directory_enabled);
        assert_eq!(params.min_reputation_for_public, 0);
    }
}
