//! Request and response wrappers for the DAO module.
//!
//! Type-safe Rust APIs around the raw protobuf messages. Uses `AccountId` for
//! addresses, provides ergonomic constructors, and includes `to_any()` methods
//! for seamless integration with `TxBuilder`.

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::dao::v1 as proto;
use morpheum_sdk_core::AccountId;

use crate::types::{
    DaoConfig, DaoProposalStatus, DaoStatus, DaoType, GovernedAsset,
    WeightedDaoVoteOption,
};

// ====================== TRANSACTION REQUESTS ======================

/// Create a new permissionless DAO instance.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateDaoRequest {
    pub from_address: AccountId,
    pub name: String,
    pub description: String,
    pub community_token_mint: String,
    pub council_token_mint: Option<String>,
    pub dao_type: DaoType,
    pub config: DaoConfig,
    pub initial_governed_assets: Vec<GovernedAsset>,
    pub metadata: BTreeMap<String, String>,
}

impl CreateDaoRequest {
    pub fn new(
        from_address: AccountId,
        name: impl Into<String>,
        community_token_mint: impl Into<String>,
        dao_type: DaoType,
        config: DaoConfig,
    ) -> Self {
        Self {
            from_address,
            name: name.into(),
            description: String::new(),
            community_token_mint: community_token_mint.into(),
            council_token_mint: None,
            dao_type,
            config,
            initial_governed_assets: Vec::new(),
            metadata: BTreeMap::new(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_council_token_mint(mut self, mint: impl Into<String>) -> Self {
        self.council_token_mint = Some(mint.into());
        self
    }

    pub fn with_governed_assets(mut self, assets: Vec<GovernedAsset>) -> Self {
        self.initial_governed_assets = assets;
        self
    }

    pub fn with_metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCreateDaoRequest = self.clone().into();
        ProtoAny {
            type_url: "/dao.v1.MsgCreateDaoRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CreateDaoRequest> for proto::MsgCreateDaoRequest {
    fn from(req: CreateDaoRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            name: req.name,
            description: req.description,
            community_token_mint: req.community_token_mint,
            council_token_mint: req.council_token_mint.unwrap_or_default(),
            dao_type: i32::from(req.dao_type),
            config: Some(req.config.into()),
            initial_governed_assets: req.initial_governed_assets.into_iter().map(Into::into).collect(),
            metadata: req.metadata.into_iter().collect(),
            timestamp: None,
        }
    }
}

/// Create a new proposal inside a specific DAO.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateDaoProposalRequest {
    pub from_address: AccountId,
    pub dao_id: u64,
    pub title: String,
    pub description: String,
    pub metadata: String,
    pub instructions: Vec<ProtoAny>,
    pub additional_metadata: BTreeMap<String, String>,
}

impl CreateDaoProposalRequest {
    pub fn new(
        from_address: AccountId,
        dao_id: u64,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            from_address,
            dao_id,
            title: title.into(),
            description: description.into(),
            metadata: String::new(),
            instructions: Vec::new(),
            additional_metadata: BTreeMap::new(),
        }
    }

    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = metadata.into();
        self
    }

    pub fn with_instructions(mut self, instructions: Vec<ProtoAny>) -> Self {
        self.instructions = instructions;
        self
    }

    pub fn with_additional_metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.additional_metadata = metadata;
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCreateProposalRequest = self.clone().into();
        ProtoAny {
            type_url: "/dao.v1.MsgCreateProposalRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CreateDaoProposalRequest> for proto::MsgCreateProposalRequest {
    fn from(req: CreateDaoProposalRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            dao_id: req.dao_id,
            title: req.title,
            description: req.description,
            metadata: req.metadata,
            instructions: req.instructions,
            additional_metadata: req.additional_metadata.into_iter().collect(),
            timestamp: None,
        }
    }
}

/// Add a council signatory approval to a proposal.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SignDaoProposalRequest {
    pub from_address: AccountId,
    pub dao_id: u64,
    pub proposal_id: u64,
}

impl SignDaoProposalRequest {
    pub fn new(from_address: AccountId, dao_id: u64, proposal_id: u64) -> Self {
        Self { from_address, dao_id, proposal_id }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgSignProposalRequest = self.clone().into();
        ProtoAny {
            type_url: "/dao.v1.MsgSignProposalRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<SignDaoProposalRequest> for proto::MsgSignProposalRequest {
    fn from(req: SignDaoProposalRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            dao_id: req.dao_id,
            proposal_id: req.proposal_id,
            timestamp: None,
        }
    }
}

/// Deposit governance tokens to gain voting power in a DAO.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DaoDepositRequest {
    pub from_address: AccountId,
    pub dao_id: u64,
    pub token_mint: String,
    pub amount: String,
    pub lock_until: u64,
}

impl DaoDepositRequest {
    pub fn new(
        from_address: AccountId,
        dao_id: u64,
        token_mint: impl Into<String>,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            from_address,
            dao_id,
            token_mint: token_mint.into(),
            amount: amount.into(),
            lock_until: 0,
        }
    }

    pub fn with_lock_until(mut self, lock_until: u64) -> Self {
        self.lock_until = lock_until;
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgDepositRequest = self.clone().into();
        ProtoAny {
            type_url: "/dao.v1.MsgDepositRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<DaoDepositRequest> for proto::MsgDepositRequest {
    fn from(req: DaoDepositRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            dao_id: req.dao_id,
            token_mint: req.token_mint,
            amount: req.amount,
            lock_until: if req.lock_until > 0 {
                Some(morpheum_proto::google::protobuf::Timestamp {
                    seconds: req.lock_until as i64,
                    nanos: 0,
                })
            } else {
                None
            },
            timestamp: None,
        }
    }
}

/// Cast or update a vote on a DAO proposal.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DaoVoteRequest {
    pub from_address: AccountId,
    pub dao_id: u64,
    pub proposal_id: u64,
    pub options: Vec<WeightedDaoVoteOption>,
    pub conviction_multiplier: u64,
}

impl DaoVoteRequest {
    pub fn new(
        from_address: AccountId,
        dao_id: u64,
        proposal_id: u64,
        options: Vec<WeightedDaoVoteOption>,
    ) -> Self {
        Self {
            from_address,
            dao_id,
            proposal_id,
            options,
            conviction_multiplier: 0,
        }
    }

    pub fn with_conviction(mut self, multiplier: u64) -> Self {
        self.conviction_multiplier = multiplier;
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgVoteRequest = self.clone().into();
        ProtoAny {
            type_url: "/dao.v1.MsgVoteRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<DaoVoteRequest> for proto::MsgVoteRequest {
    fn from(req: DaoVoteRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            dao_id: req.dao_id,
            proposal_id: req.proposal_id,
            options: req.options.into_iter().map(Into::into).collect(),
            conviction_multiplier: req.conviction_multiplier,
            timestamp: None,
        }
    }
}

/// Cancel a DAO proposal (proposer only, while in Draft/Signing).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CancelDaoProposalRequest {
    pub from_address: AccountId,
    pub dao_id: u64,
    pub proposal_id: u64,
    pub reason: String,
}

impl CancelDaoProposalRequest {
    pub fn new(
        from_address: AccountId,
        dao_id: u64,
        proposal_id: u64,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            from_address,
            dao_id,
            proposal_id,
            reason: reason.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCancelProposalRequest = self.clone().into();
        ProtoAny {
            type_url: "/dao.v1.MsgCancelProposalRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CancelDaoProposalRequest> for proto::MsgCancelProposalRequest {
    fn from(req: CancelDaoProposalRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            dao_id: req.dao_id,
            proposal_id: req.proposal_id,
            reason: req.reason,
            timestamp: None,
        }
    }
}

/// Manually trigger execution of a passed DAO proposal after hold-up delay.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecuteDaoProposalRequest {
    pub from_address: AccountId,
    pub dao_id: u64,
    pub proposal_id: u64,
}

impl ExecuteDaoProposalRequest {
    pub fn new(from_address: AccountId, dao_id: u64, proposal_id: u64) -> Self {
        Self { from_address, dao_id, proposal_id }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgExecuteProposalRequest = self.clone().into();
        ProtoAny {
            type_url: "/dao.v1.MsgExecuteProposalRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ExecuteDaoProposalRequest> for proto::MsgExecuteProposalRequest {
    fn from(req: ExecuteDaoProposalRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            dao_id: req.dao_id,
            proposal_id: req.proposal_id,
            timestamp: None,
        }
    }
}

/// Withdraw deposited tokens from a DAO after lock period expires.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WithdrawDaoDepositRequest {
    pub from_address: AccountId,
    pub dao_id: u64,
    pub token_mint: String,
    pub amount: String,
}

impl WithdrawDaoDepositRequest {
    pub fn new(
        from_address: AccountId,
        dao_id: u64,
        token_mint: impl Into<String>,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            from_address,
            dao_id,
            token_mint: token_mint.into(),
            amount: amount.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgWithdrawDepositRequest = self.clone().into();
        ProtoAny {
            type_url: "/dao.v1.MsgWithdrawDepositRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<WithdrawDaoDepositRequest> for proto::MsgWithdrawDepositRequest {
    fn from(req: WithdrawDaoDepositRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            dao_id: req.dao_id,
            token_mint: req.token_mint,
            amount: req.amount,
            timestamp: None,
        }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query a single DAO by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDaoRequest {
    pub dao_id: u64,
}

impl QueryDaoRequest {
    pub fn new(dao_id: u64) -> Self {
        Self { dao_id }
    }
}

impl From<QueryDaoRequest> for proto::QueryDaoRequest {
    fn from(req: QueryDaoRequest) -> Self {
        Self { dao_id: req.dao_id }
    }
}

/// Query DAOs with optional filters and pagination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDaosRequest {
    pub limit: i32,
    pub offset: i32,
    pub status_filter: Option<DaoStatus>,
    pub type_filter: Option<DaoType>,
    pub creator_filter: Option<String>,
}

impl QueryDaosRequest {
    pub fn new(limit: i32, offset: i32) -> Self {
        Self {
            limit,
            offset,
            status_filter: None,
            type_filter: None,
            creator_filter: None,
        }
    }

    pub fn status_filter(mut self, status: DaoStatus) -> Self {
        self.status_filter = Some(status);
        self
    }

    pub fn type_filter(mut self, dao_type: DaoType) -> Self {
        self.type_filter = Some(dao_type);
        self
    }

    pub fn creator_filter(mut self, creator: impl Into<String>) -> Self {
        self.creator_filter = Some(creator.into());
        self
    }
}

impl From<QueryDaosRequest> for proto::QueryDaosRequest {
    fn from(req: QueryDaosRequest) -> Self {
        Self {
            limit: req.limit,
            offset: req.offset,
            status_filter: req.status_filter.map(i32::from).unwrap_or(0),
            type_filter: req.type_filter.map(i32::from).unwrap_or(0),
            creator_filter: req.creator_filter.unwrap_or_default(),
        }
    }
}

/// Query a specific proposal by dao_id + proposal_id.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDaoProposalRequest {
    pub dao_id: u64,
    pub proposal_id: u64,
}

impl QueryDaoProposalRequest {
    pub fn new(dao_id: u64, proposal_id: u64) -> Self {
        Self { dao_id, proposal_id }
    }
}

impl From<QueryDaoProposalRequest> for proto::QueryProposalRequest {
    fn from(req: QueryDaoProposalRequest) -> Self {
        Self { dao_id: req.dao_id, proposal_id: req.proposal_id }
    }
}

/// Query proposals for a specific DAO.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDaoProposalsRequest {
    pub dao_id: u64,
    pub limit: i32,
    pub offset: i32,
    pub status_filter: Option<DaoProposalStatus>,
    pub proposer_filter: Option<String>,
}

impl QueryDaoProposalsRequest {
    pub fn new(dao_id: u64, limit: i32, offset: i32) -> Self {
        Self {
            dao_id,
            limit,
            offset,
            status_filter: None,
            proposer_filter: None,
        }
    }

    pub fn status_filter(mut self, status: DaoProposalStatus) -> Self {
        self.status_filter = Some(status);
        self
    }

    pub fn proposer_filter(mut self, proposer: impl Into<String>) -> Self {
        self.proposer_filter = Some(proposer.into());
        self
    }
}

impl From<QueryDaoProposalsRequest> for proto::QueryProposalsRequest {
    fn from(req: QueryDaoProposalsRequest) -> Self {
        Self {
            dao_id: req.dao_id,
            limit: req.limit,
            offset: req.offset,
            status_filter: req.status_filter.map(i32::from).unwrap_or(0),
            proposer_filter: req.proposer_filter.unwrap_or_default(),
        }
    }
}

/// Query a specific vote on a DAO proposal.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDaoVoteRequest {
    pub dao_id: u64,
    pub proposal_id: u64,
    pub voter: String,
}

impl QueryDaoVoteRequest {
    pub fn new(dao_id: u64, proposal_id: u64, voter: impl Into<String>) -> Self {
        Self { dao_id, proposal_id, voter: voter.into() }
    }
}

impl From<QueryDaoVoteRequest> for proto::QueryVoteRequest {
    fn from(req: QueryDaoVoteRequest) -> Self {
        Self { dao_id: req.dao_id, proposal_id: req.proposal_id, voter: req.voter }
    }
}

/// Query all votes on a DAO proposal (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDaoVotesRequest {
    pub dao_id: u64,
    pub proposal_id: u64,
    pub limit: i32,
    pub offset: i32,
}

impl QueryDaoVotesRequest {
    pub fn new(dao_id: u64, proposal_id: u64, limit: i32, offset: i32) -> Self {
        Self { dao_id, proposal_id, limit, offset }
    }
}

impl From<QueryDaoVotesRequest> for proto::QueryVotesRequest {
    fn from(req: QueryDaoVotesRequest) -> Self {
        Self {
            dao_id: req.dao_id,
            proposal_id: req.proposal_id,
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query a specific deposit in a DAO.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDaoDepositRequest {
    pub dao_id: u64,
    pub depositor: String,
}

impl QueryDaoDepositRequest {
    pub fn new(dao_id: u64, depositor: impl Into<String>) -> Self {
        Self { dao_id, depositor: depositor.into() }
    }
}

impl From<QueryDaoDepositRequest> for proto::QueryDepositRequest {
    fn from(req: QueryDaoDepositRequest) -> Self {
        Self { dao_id: req.dao_id, depositor: req.depositor }
    }
}

/// Query deposits in a DAO (paginated, optionally filtered by depositor).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDaoDepositsRequest {
    pub dao_id: u64,
    pub depositor_filter: Option<String>,
    pub limit: i32,
    pub offset: i32,
}

impl QueryDaoDepositsRequest {
    pub fn new(dao_id: u64, limit: i32, offset: i32) -> Self {
        Self { dao_id, depositor_filter: None, limit, offset }
    }

    pub fn depositor_filter(mut self, depositor: impl Into<String>) -> Self {
        self.depositor_filter = Some(depositor.into());
        self
    }
}

impl From<QueryDaoDepositsRequest> for proto::QueryDepositsRequest {
    fn from(req: QueryDaoDepositsRequest) -> Self {
        Self {
            dao_id: req.dao_id,
            depositor_filter: req.depositor_filter.unwrap_or_default(),
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query tally result for a DAO proposal.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDaoTallyResultRequest {
    pub dao_id: u64,
    pub proposal_id: u64,
}

impl QueryDaoTallyResultRequest {
    pub fn new(dao_id: u64, proposal_id: u64) -> Self {
        Self { dao_id, proposal_id }
    }
}

impl From<QueryDaoTallyResultRequest> for proto::QueryTallyResultRequest {
    fn from(req: QueryDaoTallyResultRequest) -> Self {
        Self { dao_id: req.dao_id, proposal_id: req.proposal_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DaoVoteOption;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn create_dao_request_to_any() {
        let from = AccountId::new([1u8; 32]);
        let config = DaoConfig {
            voting_period: "86400s".into(),
            hold_up_time: "3600s".into(),
            min_deposit_for_proposal: "1000".into(),
            quorum: "0.2".into(),
            approval_threshold: "0.5".into(),
            allow_council_override: false,
            use_conviction_voting: false,
            max_active_proposals: 10,
            plugin_configs: BTreeMap::new(),
        };
        let req = CreateDaoRequest::new(from, "My DAO", "morpheum1mint", DaoType::Community, config);

        let any = req.to_any();
        assert_eq!(any.type_url, "/dao.v1.MsgCreateDaoRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn dao_vote_request_to_any() {
        let from = AccountId::new([2u8; 32]);
        let options = alloc::vec![
            WeightedDaoVoteOption::new(DaoVoteOption::Yes, "0.8"),
            WeightedDaoVoteOption::new(DaoVoteOption::Abstain, "0.2"),
        ];

        let req = DaoVoteRequest::new(from, 1, 5, options).with_conviction(3);

        let any = req.to_any();
        assert_eq!(any.type_url, "/dao.v1.MsgVoteRequest");
        assert_eq!(req.conviction_multiplier, 3);
    }

    #[test]
    fn execute_dao_proposal_to_any() {
        let from = AccountId::new([3u8; 32]);
        let req = ExecuteDaoProposalRequest::new(from, 1, 42);

        let any = req.to_any();
        assert_eq!(any.type_url, "/dao.v1.MsgExecuteProposalRequest");
    }

    #[test]
    fn query_daos_with_filters() {
        let req = QueryDaosRequest::new(20, 0)
            .status_filter(DaoStatus::Active)
            .type_filter(DaoType::Hybrid)
            .creator_filter("morpheum1creator");

        let proto_req: proto::QueryDaosRequest = req.into();
        assert_eq!(proto_req.limit, 20);
        assert_eq!(proto_req.status_filter, i32::from(DaoStatus::Active));
        assert_eq!(proto_req.type_filter, i32::from(DaoType::Hybrid));
        assert_eq!(proto_req.creator_filter, "morpheum1creator");
    }

    #[test]
    fn query_dao_proposals_with_filters() {
        let req = QueryDaoProposalsRequest::new(1, 10, 0)
            .status_filter(DaoProposalStatus::Voting);

        let proto_req: proto::QueryProposalsRequest = req.into();
        assert_eq!(proto_req.dao_id, 1);
        assert_eq!(proto_req.status_filter, i32::from(DaoProposalStatus::Voting));
    }
}
