//! ReputationClient — the main entry point for reputation-related operations
//! in the Morpheum SDK.
//!
//! This client provides high-level, type-safe methods for querying reputation
//! scores, event history, milestone status, and module parameters. Transaction
//! operations (penalty, recovery, milestone forcing, parameter updates) are
//! handled via the fluent builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{
    MorpheumClient, SdkConfig, SdkError, Transport,
};

use crate::{
    requests::{
        QueryMilestoneStatusRequest,
        QueryReputationHistoryRequest,
        QueryReputationScoreRequest,
    },
    types::{MilestoneStatus, Params, ReputationEvent, ReputationScore},
};

/// Primary client for all reputation-related queries.
///
/// Transaction construction (penalty, recovery, milestone, params) is delegated
/// to the fluent builders in `builder.rs` for maximum ergonomics and type safety.
pub struct ReputationClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl ReputationClient {
    /// Creates a new `ReputationClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries the current reputation score for an agent.
    ///
    /// Returns `None` if the agent has no reputation record.
    pub async fn query_score(
        &self,
        agent_hash: impl Into<alloc::string::String>,
    ) -> Result<Option<ReputationScore>, SdkError> {
        let req = QueryReputationScoreRequest::new(agent_hash);
        let proto_req: morpheum_proto::reputation::v1::QueryReputationScoreRequest = req.into();

        let path = "/reputation.v1.Query/QueryReputationScore";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::reputation::v1::QueryReputationScoreResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryReputationScoreResponse = proto_res.into();
        Ok(response.score)
    }

    /// Queries the reputation event history for an agent (paginated).
    pub async fn query_history(
        &self,
        agent_hash: impl Into<alloc::string::String>,
        limit: u32,
        offset: u32,
    ) -> Result<(Vec<ReputationEvent>, u32), SdkError> {
        let req = QueryReputationHistoryRequest::new(agent_hash, limit, offset);
        let proto_req: morpheum_proto::reputation::v1::QueryReputationHistoryRequest = req.into();

        let path = "/reputation.v1.Query/QueryReputationHistory";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::reputation::v1::QueryReputationHistoryResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryReputationHistoryResponse = proto_res.into();
        Ok((response.events, response.total_count))
    }

    /// Queries the milestone and perk status for an agent.
    pub async fn query_milestone_status(
        &self,
        agent_hash: impl Into<alloc::string::String>,
    ) -> Result<MilestoneStatus, SdkError> {
        let req = QueryMilestoneStatusRequest::new(agent_hash);
        let proto_req: morpheum_proto::reputation::v1::QueryMilestoneStatusRequest = req.into();

        let path = "/reputation.v1.Query/QueryMilestoneStatus";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::reputation::v1::QueryMilestoneStatusResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(MilestoneStatus::from(proto_res))
    }

    /// Queries the current module parameters.
    pub async fn query_params(&self) -> Result<Option<Params>, SdkError> {
        let req = crate::requests::QueryParamsRequest;
        let proto_req: morpheum_proto::reputation::v1::QueryParamsRequest = req.into();

        let path = "/reputation.v1.Query/QueryParams";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::reputation::v1::QueryParamsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryParamsResponse = proto_res.into();
        Ok(response.params)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for ReputationClient {
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
            unimplemented!("not needed for reputation query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/reputation.v1.Query/QueryReputationScore" => {
                    let dummy = morpheum_proto::reputation::v1::QueryReputationScoreResponse {
                        score: Some(morpheum_proto::reputation::v1::ReputationScore {
                            agent_hash: "test-agent".into(),
                            score: 750_000,
                            last_updated: 1_700_000_000,
                            penalty_count_30d: 1,
                            milestone_bitflags: 0b0000_0111,
                            is_immortal: false,
                            recovery_velocity: 200,
                            perk_bitflags: 0b0000_0011,
                            luxury_perks_throttled: false,
                        }),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/reputation.v1.Query/QueryReputationHistory" => {
                    let dummy = morpheum_proto::reputation::v1::QueryReputationHistoryResponse {
                        events: vec![morpheum_proto::reputation::v1::ReputationEvent {
                            agent_hash: "test-agent".into(),
                            event_type: 1, // Recovery
                            delta: 500,
                            reason: "trade fill".into(),
                            new_score: 750_500,
                            timestamp: 1_700_000_100,
                        }],
                        total_count: 1,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/reputation.v1.Query/QueryMilestoneStatus" => {
                    let dummy = morpheum_proto::reputation::v1::QueryMilestoneStatusResponse {
                        current_milestone_level: 3,
                        is_immortal: false,
                        milestone_bitflags: 0b0000_0111,
                        perk_bitflags: 0b0000_0011,
                        luxury_perks_throttled: false,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/reputation.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::reputation::v1::QueryParamsResponse {
                        params: Some(morpheum_proto::reputation::v1::Params {
                            daily_recovery_cap_bps: 3000,
                            min_reputation_to_register: 0,
                            enable_reputation_priority: true,
                            slashing_multiplier: 100,
                            milestone_thresholds: vec![
                                10_000, 50_000, 100_000, 250_000, 500_000, 750_000, 900_000,
                                1_000_000,
                            ],
                            milestone_rewards: vec![
                                500, 1_000, 2_000, 5_000, 10_000, 20_000, 50_000, 100_000,
                            ],
                            perk_multiplier_bps: 1500,
                        }),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn query_score_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = ReputationClient::new(config, Box::new(DummyTransport));

        let result = client.query_score("test-agent").await;
        assert!(result.is_ok());

        let score = result.unwrap().expect("score should be present");
        assert_eq!(score.score, 750_000);
        assert_eq!(score.agent_hash, "test-agent");
        assert!(score.has_milestone(0));
        assert!(score.has_milestone(1));
        assert!(score.has_milestone(2));
        assert!(!score.has_milestone(3));
    }

    #[tokio::test]
    async fn query_history_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = ReputationClient::new(config, Box::new(DummyTransport));

        let (events, total) = client.query_history("test-agent", 10, 0).await.unwrap();
        assert_eq!(total, 1);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].delta, 500);
    }

    #[tokio::test]
    async fn query_milestone_status_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = ReputationClient::new(config, Box::new(DummyTransport));

        let status = client.query_milestone_status("test-agent").await.unwrap();
        assert_eq!(status.current_milestone_level, 3);
        assert!(!status.is_immortal);
    }

    #[tokio::test]
    async fn query_params_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = ReputationClient::new(config, Box::new(DummyTransport));

        let params = client.query_params().await.unwrap().expect("params should be present");
        assert_eq!(params.daily_recovery_cap_bps, 3000);
        assert!(params.enable_reputation_priority);
        assert_eq!(params.milestone_thresholds.len(), 8);
    }
}
