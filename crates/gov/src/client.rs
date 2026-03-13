//! GovClient — the main entry point for all governance-related queries
//! in the Morpheum SDK.
//!
//! Provides high-level, type-safe methods for querying proposals, votes,
//! deposits, tally results, and governance parameters. Transaction operations
//! (submit, vote, deposit, cancel, execute, upgrade) are handled via the
//! fluent builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

use crate::requests::{
    QueryGovParamsRequest, QueryProposalDepositsRequest, QueryProposalDepositRequest,
    QueryProposalRequest, QueryProposalVoteRequest, QueryProposalVotesRequest,
    QueryProposalsRequest, QueryTallyResultRequest,
};
use crate::types::{
    Deposit, GovParams, Proposal, ProposalClass, ProposalStatus, TallyResult, Vote,
};

/// Primary client for all governance-related queries.
///
/// Transaction construction (submit proposal, vote, deposit, cancel, execute,
/// schedule upgrade) is delegated to the fluent builders in `builder.rs`.
pub struct GovClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl GovClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries the current governance parameters.
    pub async fn query_params(&self) -> Result<GovParams, SdkError> {
        let proto_req: morpheum_proto::gov::v1::QueryParamsRequest =
            QueryGovParamsRequest.into();

        let response_bytes = self
            .query("/gov.v1.Query/QueryParams", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::gov::v1::QueryParamsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .params
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("params field missing in response"))
    }

    /// Queries a single proposal by ID.
    pub async fn query_proposal(&self, proposal_id: u64) -> Result<Proposal, SdkError> {
        let req = QueryProposalRequest::new(proposal_id);
        let proto_req: morpheum_proto::gov::v1::QueryProposalRequest = req.into();

        let response_bytes = self
            .query("/gov.v1.Query/QueryProposal", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::gov::v1::QueryProposalResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .proposal
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("proposal field missing in response"))
    }

    /// Queries proposals with optional filters and pagination.
    pub async fn query_proposals(
        &self,
        limit: i32,
        offset: i32,
        status_filter: Option<ProposalStatus>,
        class_filter: Option<ProposalClass>,
        proposer_filter: Option<String>,
    ) -> Result<Vec<Proposal>, SdkError> {
        let mut req = QueryProposalsRequest::new(limit, offset);
        if let Some(status) = status_filter {
            req = req.status_filter(status);
        }
        if let Some(class) = class_filter {
            req = req.class_filter(class);
        }
        if let Some(proposer) = proposer_filter {
            req = req.proposer_filter(proposer);
        }

        let proto_req: morpheum_proto::gov::v1::QueryProposalsRequest = req.into();

        let response_bytes = self
            .query("/gov.v1.Query/QueryProposals", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::gov::v1::QueryProposalsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.proposals.into_iter().map(Into::into).collect())
    }

    /// Queries a specific vote on a proposal.
    pub async fn query_vote(
        &self,
        proposal_id: u64,
        voter: impl Into<String>,
    ) -> Result<Vote, SdkError> {
        let req = QueryProposalVoteRequest::new(proposal_id, voter);
        let proto_req: morpheum_proto::gov::v1::QueryVoteRequest = req.into();

        let response_bytes = self
            .query("/gov.v1.Query/QueryVote", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::gov::v1::QueryVoteResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .vote
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("vote field missing in response"))
    }

    /// Queries all votes on a proposal (paginated).
    pub async fn query_votes(
        &self,
        proposal_id: u64,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Vote>, SdkError> {
        let req = QueryProposalVotesRequest::new(proposal_id, limit, offset);
        let proto_req: morpheum_proto::gov::v1::QueryVotesRequest = req.into();

        let response_bytes = self
            .query("/gov.v1.Query/QueryVotes", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::gov::v1::QueryVotesResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.votes.into_iter().map(Into::into).collect())
    }

    /// Queries a specific deposit on a proposal.
    pub async fn query_deposit(
        &self,
        proposal_id: u64,
        depositor: impl Into<String>,
    ) -> Result<Deposit, SdkError> {
        let req = QueryProposalDepositRequest::new(proposal_id, depositor);
        let proto_req: morpheum_proto::gov::v1::QueryDepositRequest = req.into();

        let response_bytes = self
            .query("/gov.v1.Query/QueryDeposit", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::gov::v1::QueryDepositResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .deposit
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("deposit field missing in response"))
    }

    /// Queries all deposits on a proposal (paginated).
    pub async fn query_deposits(
        &self,
        proposal_id: u64,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<Deposit>, SdkError> {
        let req = QueryProposalDepositsRequest::new(proposal_id, limit, offset);
        let proto_req: morpheum_proto::gov::v1::QueryDepositsRequest = req.into();

        let response_bytes = self
            .query("/gov.v1.Query/QueryDeposits", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::gov::v1::QueryDepositsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.deposits.into_iter().map(Into::into).collect())
    }

    /// Queries the current or final tally result for a proposal.
    pub async fn query_tally_result(
        &self,
        proposal_id: u64,
    ) -> Result<TallyResult, SdkError> {
        let req = QueryTallyResultRequest::new(proposal_id);
        let proto_req: morpheum_proto::gov::v1::QueryTallyResultRequest = req.into();

        let response_bytes = self
            .query("/gov.v1.Query/QueryTallyResult", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::gov::v1::QueryTallyResultResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .tally
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("tally field missing in response"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for GovClient {
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
    use morpheum_sdk_core::{BroadcastResult, SdkConfig};

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError> {
            unimplemented!("not needed for gov query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/gov.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::gov::v1::QueryParamsResponse {
                        success: true,
                        error_message: "".into(),
                        params: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/gov.v1.Query/QueryProposal" => {
                    let dummy = morpheum_proto::gov::v1::QueryProposalResponse {
                        success: true,
                        error_message: "".into(),
                        proposal: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/gov.v1.Query/QueryProposals" => {
                    let dummy = morpheum_proto::gov::v1::QueryProposalsResponse {
                        success: true,
                        error_message: "".into(),
                        proposals: vec![],
                        total_count: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/gov.v1.Query/QueryVote" => {
                    let dummy = morpheum_proto::gov::v1::QueryVoteResponse {
                        success: true,
                        error_message: "".into(),
                        vote: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/gov.v1.Query/QueryVotes" => {
                    let dummy = morpheum_proto::gov::v1::QueryVotesResponse {
                        success: true,
                        error_message: "".into(),
                        votes: vec![],
                        total_count: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/gov.v1.Query/QueryDeposit" => {
                    let dummy = morpheum_proto::gov::v1::QueryDepositResponse {
                        success: true,
                        error_message: "".into(),
                        deposit: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/gov.v1.Query/QueryDeposits" => {
                    let dummy = morpheum_proto::gov::v1::QueryDepositsResponse {
                        success: true,
                        error_message: "".into(),
                        deposits: vec![],
                        total_count: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/gov.v1.Query/QueryTallyResult" => {
                    let dummy = morpheum_proto::gov::v1::QueryTallyResultResponse {
                        success: true,
                        error_message: "".into(),
                        tally: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn gov_client_query_params_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = GovClient::new(config, Box::new(DummyTransport));
        let result = client.query_params().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn gov_client_query_proposal_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = GovClient::new(config, Box::new(DummyTransport));
        let result = client.query_proposal(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn gov_client_query_proposals_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = GovClient::new(config, Box::new(DummyTransport));
        let result = client.query_proposals(10, 0, None, None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn gov_client_query_vote_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = GovClient::new(config, Box::new(DummyTransport));
        let result = client.query_vote(1, "morpheum1voter").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn gov_client_query_tally_result_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = GovClient::new(config, Box::new(DummyTransport));
        let result = client.query_tally_result(1).await;
        assert!(result.is_ok());
    }
}
