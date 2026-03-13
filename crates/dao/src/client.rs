//! DaoClient — the main entry point for all DAO-related queries
//! in the Morpheum SDK.
//!
//! Provides high-level, type-safe methods for querying DAOs, proposals, votes,
//! deposits, and tally results. Transaction operations (create DAO, propose,
//! vote, deposit, execute, withdraw) are handled via the fluent builders in
//! `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

use crate::requests::{
    QueryDaoDepositsRequest, QueryDaoDepositRequest, QueryDaoProposalRequest,
    QueryDaoProposalsRequest, QueryDaoRequest, QueryDaoTallyResultRequest,
    QueryDaoVoteRequest, QueryDaoVotesRequest, QueryDaosRequest,
};
use crate::types::{
    Dao, DaoDeposit, DaoProposal, DaoProposalStatus, DaoStatus, DaoTallyResult,
    DaoType, DaoVote,
};

/// Primary client for all DAO-related queries.
///
/// Transaction construction (create DAO, propose, vote, deposit, sign, execute,
/// cancel, withdraw) is delegated to the fluent builders in `builder.rs`.
pub struct DaoClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl DaoClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a single DAO by ID.
    pub async fn query_dao(&self, dao_id: u64) -> Result<Dao, SdkError> {
        let req = QueryDaoRequest::new(dao_id);
        let proto_req: morpheum_proto::dao::v1::QueryDaoRequest = req.into();

        let response_bytes = self
            .query("/dao.v1.Query/QueryDao", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::dao::v1::QueryDaoResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .dao
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("dao field missing in response"))
    }

    /// Queries DAOs with optional filters and pagination.
    pub async fn query_daos(
        &self,
        limit: i32,
        offset: i32,
        status_filter: Option<DaoStatus>,
        type_filter: Option<DaoType>,
        creator_filter: Option<String>,
    ) -> Result<Vec<Dao>, SdkError> {
        let mut req = QueryDaosRequest::new(limit, offset);
        if let Some(status) = status_filter {
            req = req.status_filter(status);
        }
        if let Some(dao_type) = type_filter {
            req = req.type_filter(dao_type);
        }
        if let Some(creator) = creator_filter {
            req = req.creator_filter(creator);
        }

        let proto_req: morpheum_proto::dao::v1::QueryDaosRequest = req.into();

        let response_bytes = self
            .query("/dao.v1.Query/QueryDaos", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::dao::v1::QueryDaosResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.daos.into_iter().map(Into::into).collect())
    }

    /// Queries a specific proposal by dao_id + proposal_id.
    pub async fn query_proposal(
        &self,
        dao_id: u64,
        proposal_id: u64,
    ) -> Result<DaoProposal, SdkError> {
        let req = QueryDaoProposalRequest::new(dao_id, proposal_id);
        let proto_req: morpheum_proto::dao::v1::QueryProposalRequest = req.into();

        let response_bytes = self
            .query("/dao.v1.Query/QueryProposal", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::dao::v1::QueryProposalResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .proposal
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("proposal field missing in response"))
    }

    /// Queries proposals for a specific DAO with optional filters.
    pub async fn query_proposals(
        &self,
        dao_id: u64,
        limit: i32,
        offset: i32,
        status_filter: Option<DaoProposalStatus>,
        proposer_filter: Option<String>,
    ) -> Result<Vec<DaoProposal>, SdkError> {
        let mut req = QueryDaoProposalsRequest::new(dao_id, limit, offset);
        if let Some(status) = status_filter {
            req = req.status_filter(status);
        }
        if let Some(proposer) = proposer_filter {
            req = req.proposer_filter(proposer);
        }

        let proto_req: morpheum_proto::dao::v1::QueryProposalsRequest = req.into();

        let response_bytes = self
            .query("/dao.v1.Query/QueryProposals", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::dao::v1::QueryProposalsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.proposals.into_iter().map(Into::into).collect())
    }

    /// Queries a specific vote on a DAO proposal.
    pub async fn query_vote(
        &self,
        dao_id: u64,
        proposal_id: u64,
        voter: impl Into<String>,
    ) -> Result<DaoVote, SdkError> {
        let req = QueryDaoVoteRequest::new(dao_id, proposal_id, voter);
        let proto_req: morpheum_proto::dao::v1::QueryVoteRequest = req.into();

        let response_bytes = self
            .query("/dao.v1.Query/QueryVote", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::dao::v1::QueryVoteResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .vote
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("vote field missing in response"))
    }

    /// Queries all votes on a DAO proposal (paginated).
    pub async fn query_votes(
        &self,
        dao_id: u64,
        proposal_id: u64,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<DaoVote>, SdkError> {
        let req = QueryDaoVotesRequest::new(dao_id, proposal_id, limit, offset);
        let proto_req: morpheum_proto::dao::v1::QueryVotesRequest = req.into();

        let response_bytes = self
            .query("/dao.v1.Query/QueryVotes", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::dao::v1::QueryVotesResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.votes.into_iter().map(Into::into).collect())
    }

    /// Queries a specific deposit in a DAO.
    pub async fn query_deposit(
        &self,
        dao_id: u64,
        depositor: impl Into<String>,
    ) -> Result<DaoDeposit, SdkError> {
        let req = QueryDaoDepositRequest::new(dao_id, depositor);
        let proto_req: morpheum_proto::dao::v1::QueryDepositRequest = req.into();

        let response_bytes = self
            .query("/dao.v1.Query/QueryDeposit", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::dao::v1::QueryDepositResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .deposit
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("deposit field missing in response"))
    }

    /// Queries deposits in a DAO (paginated).
    pub async fn query_deposits(
        &self,
        dao_id: u64,
        limit: i32,
        offset: i32,
        depositor_filter: Option<String>,
    ) -> Result<Vec<DaoDeposit>, SdkError> {
        let mut req = QueryDaoDepositsRequest::new(dao_id, limit, offset);
        if let Some(depositor) = depositor_filter {
            req = req.depositor_filter(depositor);
        }

        let proto_req: morpheum_proto::dao::v1::QueryDepositsRequest = req.into();

        let response_bytes = self
            .query("/dao.v1.Query/QueryDeposits", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::dao::v1::QueryDepositsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.deposits.into_iter().map(Into::into).collect())
    }

    /// Queries the tally result for a DAO proposal.
    pub async fn query_tally_result(
        &self,
        dao_id: u64,
        proposal_id: u64,
    ) -> Result<DaoTallyResult, SdkError> {
        let req = QueryDaoTallyResultRequest::new(dao_id, proposal_id);
        let proto_req: morpheum_proto::dao::v1::QueryTallyResultRequest = req.into();

        let response_bytes = self
            .query("/dao.v1.Query/QueryTallyResult", proto_req.encode_to_vec())
            .await?;

        let proto_res = morpheum_proto::dao::v1::QueryTallyResultResponse::decode(
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
impl MorpheumClient for DaoClient {
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
            unimplemented!("not needed for dao query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/dao.v1.Query/QueryDao" => {
                    let dummy = morpheum_proto::dao::v1::QueryDaoResponse {
                        success: true,
                        error_message: "".into(),
                        dao: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/dao.v1.Query/QueryDaos" => {
                    let dummy = morpheum_proto::dao::v1::QueryDaosResponse {
                        success: true,
                        error_message: "".into(),
                        daos: vec![],
                        total_count: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/dao.v1.Query/QueryProposal" => {
                    let dummy = morpheum_proto::dao::v1::QueryProposalResponse {
                        success: true,
                        error_message: "".into(),
                        proposal: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/dao.v1.Query/QueryProposals" => {
                    let dummy = morpheum_proto::dao::v1::QueryProposalsResponse {
                        success: true,
                        error_message: "".into(),
                        proposals: vec![],
                        total_count: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/dao.v1.Query/QueryVote" => {
                    let dummy = morpheum_proto::dao::v1::QueryVoteResponse {
                        success: true,
                        error_message: "".into(),
                        vote: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/dao.v1.Query/QueryVotes" => {
                    let dummy = morpheum_proto::dao::v1::QueryVotesResponse {
                        success: true,
                        error_message: "".into(),
                        votes: vec![],
                        total_count: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/dao.v1.Query/QueryDeposit" => {
                    let dummy = morpheum_proto::dao::v1::QueryDepositResponse {
                        success: true,
                        error_message: "".into(),
                        deposit: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/dao.v1.Query/QueryDeposits" => {
                    let dummy = morpheum_proto::dao::v1::QueryDepositsResponse {
                        success: true,
                        error_message: "".into(),
                        deposits: vec![],
                        total_count: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/dao.v1.Query/QueryTallyResult" => {
                    let dummy = morpheum_proto::dao::v1::QueryTallyResultResponse {
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
    async fn dao_client_query_dao_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = DaoClient::new(config, Box::new(DummyTransport));
        let result = client.query_dao(1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn dao_client_query_daos_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = DaoClient::new(config, Box::new(DummyTransport));
        let result = client.query_daos(10, 0, None, None, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn dao_client_query_proposal_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = DaoClient::new(config, Box::new(DummyTransport));
        let result = client.query_proposal(1, 1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn dao_client_query_vote_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = DaoClient::new(config, Box::new(DummyTransport));
        let result = client.query_vote(1, 1, "morpheum1voter").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn dao_client_query_tally_result_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = DaoClient::new(config, Box::new(DummyTransport));
        let result = client.query_tally_result(1, 1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn dao_client_query_deposit_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = DaoClient::new(config, Box::new(DummyTransport));
        let result = client.query_deposit(1, "morpheum1user").await;
        assert!(result.is_ok());
    }
}
