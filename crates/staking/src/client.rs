//! StakingClient — the main entry point for all staking-related query
//! operations in the Morpheum SDK.
//!
//! Provides high-level, type-safe methods for querying validators, delegations,
//! rewards, penalties, slashing history, epoch info, and module parameters.
//! Transaction operations (stake, delegate, claim, report, etc.) are handled
//! via the fluent builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

use crate::requests::{
    QueryDelegationsRequest, QueryParamsRequest, QueryRewardsRequest,
    QueryUserStakingRequest, QueryValidatorRequest, QueryValidatorsRequest,
};
use crate::types::{
    Delegation, Penalty, Reward, SlashingEvent, StakingParams,
    UserStaking, Validator, ValidatorStake,
};

/// Primary client for all staking-related queries.
///
/// Transaction construction (stake, delegate, claim, report, vote, slash)
/// is delegated to the fluent builders in `builder.rs` for maximum ergonomics
/// and type safety.
pub struct StakingClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl StakingClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a single validator by ID.
    pub async fn query_validator(
        &self,
        validator_id: impl Into<String>,
    ) -> Result<Option<Validator>, SdkError> {
        let req = QueryValidatorRequest::new(validator_id);
        let proto_req: morpheum_proto::staking::v1::QueryValidatorRequest = req.into();

        let path = "/staking.v1.Query/QueryValidator";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::staking::v1::QueryValidatorResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        if proto_res.found {
            Ok(proto_res.validator.map(Into::into))
        } else {
            Ok(None)
        }
    }

    /// Queries validators with optional active-only filter and pagination.
    pub async fn query_validators(
        &self,
        active_only: bool,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Validator>, SdkError> {
        let mut req = QueryValidatorsRequest::new(limit, offset);
        if active_only {
            req = req.active_only();
        }
        let proto_req: morpheum_proto::staking::v1::QueryValidatorsRequest = req.into();

        let path = "/staking.v1.Query/QueryValidators";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::staking::v1::QueryValidatorsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.validators.into_iter().map(Into::into).collect())
    }

    /// Queries a user's full staking overview (delegations, unbondings, rewards).
    pub async fn query_user_staking(
        &self,
        address: impl Into<String>,
    ) -> Result<UserStaking, SdkError> {
        let req = QueryUserStakingRequest::new(address);
        let proto_req: morpheum_proto::staking::v1::QueryUserStakingRequest = req.into();

        let path = "/staking.v1.Query/QueryUserStaking";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::staking::v1::QueryUserStakingResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(UserStaking {
            delegations: proto_res.delegations.into_iter().map(Into::into).collect(),
            unbonding_delegations: proto_res.unbonding_delegations.into_iter().map(Into::into).collect(),
            rewards: proto_res.rewards.into_iter().map(Into::into).collect(),
            total_staked: proto_res.total_staked,
            total_rewards: proto_res.total_rewards,
        })
    }

    /// Queries a validator's stake breakdown (self + delegated).
    pub async fn query_validator_stake(
        &self,
        validator_id: impl Into<String>,
    ) -> Result<ValidatorStake, SdkError> {
        let proto_req = morpheum_proto::staking::v1::QueryValidatorStakeRequest {
            validator_id: validator_id.into(),
        };

        let path = "/staking.v1.Query/QueryValidatorStake";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::staking::v1::QueryValidatorStakeResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(ValidatorStake {
            self_stake: proto_res.self_stake,
            delegated: proto_res.delegated,
        })
    }

    /// Queries delegations for an address with pagination.
    pub async fn query_delegations(
        &self,
        address: impl Into<String>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Delegation>, SdkError> {
        let req = QueryDelegationsRequest::new(address, limit, offset);
        let proto_req: morpheum_proto::staking::v1::QueryDelegationsRequest = req.into();

        let path = "/staking.v1.Query/QueryDelegations";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::staking::v1::QueryDelegationsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.delegations.into_iter().map(Into::into).collect())
    }

    /// Queries rewards for an address, optionally filtered by validator and epoch.
    pub async fn query_rewards(
        &self,
        address: impl Into<String>,
        validator_id: Option<String>,
        epoch: Option<u64>,
    ) -> Result<Vec<Reward>, SdkError> {
        let mut req = QueryRewardsRequest::new(address);
        if let Some(v) = validator_id {
            req = req.with_validator(v);
        }
        if let Some(e) = epoch {
            req = req.with_epoch(e);
        }
        let proto_req: morpheum_proto::staking::v1::QueryRewardsRequest = req.into();

        let path = "/staking.v1.Query/QueryRewards";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::staking::v1::QueryRewardsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.rewards.into_iter().map(Into::into).collect())
    }

    /// Queries penalties for a validator with pagination.
    pub async fn query_penalties(
        &self,
        validator_id: impl Into<String>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Penalty>, SdkError> {
        let proto_req = morpheum_proto::staking::v1::QueryPenaltiesRequest {
            validator_id: validator_id.into(),
            limit,
            offset,
        };

        let path = "/staking.v1.Query/QueryPenalties";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::staking::v1::QueryPenaltiesResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.penalties.into_iter().map(Into::into).collect())
    }

    /// Queries slashing history for a validator with pagination.
    pub async fn query_slashing_history(
        &self,
        validator_id: impl Into<String>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<SlashingEvent>, SdkError> {
        let proto_req = morpheum_proto::staking::v1::QuerySlashingHistoryRequest {
            validator_id: validator_id.into(),
            limit,
            offset,
        };

        let path = "/staking.v1.Query/QuerySlashingHistory";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::staking::v1::QuerySlashingHistoryResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.events.into_iter().map(Into::into).collect())
    }

    /// Queries module parameters.
    pub async fn query_params(&self) -> Result<StakingParams, SdkError> {
        let req = QueryParamsRequest;
        let proto_req: morpheum_proto::staking::v1::QueryParamsRequest = req.into();

        let path = "/staking.v1.Query/QueryParams";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::staking::v1::QueryParamsResponse::decode(
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
impl MorpheumClient for StakingClient {
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
            unimplemented!("not needed for staking query tests")
        }

        async fn query(
            &self,
            path: &str,
            _data: Vec<u8>,
        ) -> Result<Vec<u8>, SdkError> {
            match path {
                "/staking.v1.Query/QueryValidator" => {
                    let dummy = morpheum_proto::staking::v1::QueryValidatorResponse {
                        validator: Some(Default::default()),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/staking.v1.Query/QueryValidators" => {
                    let dummy = morpheum_proto::staking::v1::QueryValidatorsResponse {
                        validators: vec![],
                        total_count: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/staking.v1.Query/QueryUserStaking" => {
                    let dummy = morpheum_proto::staking::v1::QueryUserStakingResponse {
                        delegations: vec![],
                        unbonding_delegations: vec![],
                        rewards: vec![],
                        total_staked: "0".into(),
                        total_rewards: "0".into(),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/staking.v1.Query/QueryValidatorStake" => {
                    let dummy = morpheum_proto::staking::v1::QueryValidatorStakeResponse {
                        self_stake: "1000000".into(),
                        delegated: "500000".into(),
                        success: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/staking.v1.Query/QueryDelegations" => {
                    let dummy = morpheum_proto::staking::v1::QueryDelegationsResponse {
                        delegations: vec![],
                        success: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/staking.v1.Query/QueryRewards" => {
                    let dummy = morpheum_proto::staking::v1::QueryRewardsResponse {
                        rewards: vec![],
                        success: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/staking.v1.Query/QueryPenalties" => {
                    let dummy = morpheum_proto::staking::v1::QueryPenaltiesResponse {
                        penalties: vec![],
                        success: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/staking.v1.Query/QuerySlashingHistory" => {
                    let dummy = morpheum_proto::staking::v1::QuerySlashingHistoryResponse {
                        events: vec![],
                        total_count: 0,
                        success: true,
                        error_message: String::new(),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/staking.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::staking::v1::QueryParamsResponse {
                        params: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    fn make_client() -> StakingClient {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        StakingClient::new(config, Box::new(DummyTransport))
    }

    #[tokio::test]
    async fn query_validator_works() {
        let client = make_client();
        let result = client.query_validator("val-1").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn query_validators_works() {
        let client = make_client();
        let result = client.query_validators(true, 10, 0).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_user_staking_works() {
        let client = make_client();
        let result = client.query_user_staking("morm1abc").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_validator_stake_works() {
        let client = make_client();
        let result = client.query_validator_stake("val-1").await;
        assert!(result.is_ok());
        let stake = result.unwrap();
        assert_eq!(stake.self_stake, "1000000");
    }

    #[tokio::test]
    async fn query_delegations_works() {
        let client = make_client();
        let result = client.query_delegations("morm1abc", 10, 0).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_rewards_works() {
        let client = make_client();
        let result = client.query_rewards("morm1abc", Some("val-1".into()), None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_penalties_works() {
        let client = make_client();
        let result = client.query_penalties("val-1", 10, 0).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_slashing_history_works() {
        let client = make_client();
        let result = client.query_slashing_history("val-1", 10, 0).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_params_works() {
        let client = make_client();
        let result = client.query_params().await;
        assert!(result.is_ok());
    }
}
