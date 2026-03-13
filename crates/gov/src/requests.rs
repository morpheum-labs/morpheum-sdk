//! Request and response wrappers for the Governance module.
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
use morpheum_proto::gov::v1 as proto;
use morpheum_sdk_core::AccountId;

use crate::types::{
    ProposalClass, ProposalStatus, UpgradePlan, WeightedVoteOption,
};

// ====================== TRANSACTION REQUESTS ======================

/// Submit a new governance proposal.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubmitProposalRequest {
    pub from_address: AccountId,
    pub proposal_class: ProposalClass,
    pub title: String,
    pub description: String,
    pub metadata: String,
    pub messages: Vec<ProtoAny>,
    pub initial_deposit: String,
    pub additional_metadata: BTreeMap<String, String>,
}

impl SubmitProposalRequest {
    pub fn new(
        from_address: AccountId,
        proposal_class: ProposalClass,
        title: impl Into<String>,
        description: impl Into<String>,
        initial_deposit: impl Into<String>,
    ) -> Self {
        Self {
            from_address,
            proposal_class,
            title: title.into(),
            description: description.into(),
            metadata: String::new(),
            messages: Vec::new(),
            initial_deposit: initial_deposit.into(),
            additional_metadata: BTreeMap::new(),
        }
    }

    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = metadata.into();
        self
    }

    pub fn with_messages(mut self, messages: Vec<ProtoAny>) -> Self {
        self.messages = messages;
        self
    }

    pub fn with_additional_metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.additional_metadata = metadata;
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgSubmitProposalRequest = self.clone().into();
        ProtoAny {
            type_url: "/gov.v1.MsgSubmitProposalRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<SubmitProposalRequest> for proto::MsgSubmitProposalRequest {
    fn from(req: SubmitProposalRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            proposal_class: i32::from(req.proposal_class),
            title: req.title,
            description: req.description,
            metadata: req.metadata,
            messages: req.messages,
            initial_deposit: req.initial_deposit,
            additional_metadata: req.additional_metadata.into_iter().collect(),
            timestamp: None,
        }
    }
}

/// Schedule a zero-downtime software/runtime upgrade.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScheduleUpgradeRequest {
    pub from_address: AccountId,
    pub proposal_class: ProposalClass,
    pub upgrade_plan: UpgradePlan,
    pub title: String,
    pub description: String,
    pub metadata: String,
    pub initial_deposit: String,
    pub additional_metadata: BTreeMap<String, String>,
}

impl ScheduleUpgradeRequest {
    pub fn new(
        from_address: AccountId,
        proposal_class: ProposalClass,
        upgrade_plan: UpgradePlan,
        title: impl Into<String>,
        description: impl Into<String>,
        initial_deposit: impl Into<String>,
    ) -> Self {
        Self {
            from_address,
            proposal_class,
            upgrade_plan,
            title: title.into(),
            description: description.into(),
            metadata: String::new(),
            initial_deposit: initial_deposit.into(),
            additional_metadata: BTreeMap::new(),
        }
    }

    pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
        self.metadata = metadata.into();
        self
    }

    pub fn with_additional_metadata(mut self, metadata: BTreeMap<String, String>) -> Self {
        self.additional_metadata = metadata;
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgScheduleUpgradeRequest = self.clone().into();
        ProtoAny {
            type_url: "/gov.v1.MsgScheduleUpgradeRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ScheduleUpgradeRequest> for proto::MsgScheduleUpgradeRequest {
    fn from(req: ScheduleUpgradeRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            proposal_class: i32::from(req.proposal_class),
            upgrade_plan: Some(req.upgrade_plan.into()),
            title: req.title,
            description: req.description,
            metadata: req.metadata,
            initial_deposit: req.initial_deposit,
            additional_metadata: req.additional_metadata.into_iter().collect(),
            timestamp: None,
        }
    }
}

/// Add a deposit to an existing proposal (during deposit period).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProposalDepositRequest {
    pub from_address: AccountId,
    pub proposal_id: u64,
    pub amount: String,
}

impl ProposalDepositRequest {
    pub fn new(
        from_address: AccountId,
        proposal_id: u64,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            from_address,
            proposal_id,
            amount: amount.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgDepositRequest = self.clone().into();
        ProtoAny {
            type_url: "/gov.v1.MsgDepositRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ProposalDepositRequest> for proto::MsgDepositRequest {
    fn from(req: ProposalDepositRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            proposal_id: req.proposal_id,
            amount: req.amount,
            timestamp: None,
        }
    }
}

/// Cast or update a vote on a proposal (supports weighted split + conviction).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProposalVoteRequest {
    pub from_address: AccountId,
    pub proposal_id: u64,
    pub options: Vec<WeightedVoteOption>,
    pub conviction_multiplier: u64,
}

impl ProposalVoteRequest {
    pub fn new(
        from_address: AccountId,
        proposal_id: u64,
        options: Vec<WeightedVoteOption>,
    ) -> Self {
        Self {
            from_address,
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
            type_url: "/gov.v1.MsgVoteRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ProposalVoteRequest> for proto::MsgVoteRequest {
    fn from(req: ProposalVoteRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            proposal_id: req.proposal_id,
            options: req.options.into_iter().map(Into::into).collect(),
            conviction_multiplier: req.conviction_multiplier,
            timestamp: None,
        }
    }
}

/// Cancel a proposal (proposer only, during deposit period).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CancelProposalRequest {
    pub from_address: AccountId,
    pub proposal_id: u64,
    pub reason: String,
}

impl CancelProposalRequest {
    pub fn new(
        from_address: AccountId,
        proposal_id: u64,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            from_address,
            proposal_id,
            reason: reason.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCancelProposalRequest = self.clone().into();
        ProtoAny {
            type_url: "/gov.v1.MsgCancelProposalRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CancelProposalRequest> for proto::MsgCancelProposalRequest {
    fn from(req: CancelProposalRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            proposal_id: req.proposal_id,
            reason: req.reason,
            timestamp: None,
        }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query current governance parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryGovParamsRequest;

impl From<QueryGovParamsRequest> for proto::QueryParamsRequest {
    fn from(_: QueryGovParamsRequest) -> Self {
        Self {}
    }
}

/// Query a single proposal by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProposalRequest {
    pub proposal_id: u64,
}

impl QueryProposalRequest {
    pub fn new(proposal_id: u64) -> Self {
        Self { proposal_id }
    }
}

impl From<QueryProposalRequest> for proto::QueryProposalRequest {
    fn from(req: QueryProposalRequest) -> Self {
        Self { proposal_id: req.proposal_id }
    }
}

/// Query proposals with filters and pagination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProposalsRequest {
    pub limit: i32,
    pub offset: i32,
    pub status_filter: Option<ProposalStatus>,
    pub class_filter: Option<ProposalClass>,
    pub proposer_filter: Option<String>,
}

impl QueryProposalsRequest {
    pub fn new(limit: i32, offset: i32) -> Self {
        Self {
            limit,
            offset,
            status_filter: None,
            class_filter: None,
            proposer_filter: None,
        }
    }

    pub fn status_filter(mut self, status: ProposalStatus) -> Self {
        self.status_filter = Some(status);
        self
    }

    pub fn class_filter(mut self, class: ProposalClass) -> Self {
        self.class_filter = Some(class);
        self
    }

    pub fn proposer_filter(mut self, proposer: impl Into<String>) -> Self {
        self.proposer_filter = Some(proposer.into());
        self
    }
}

impl From<QueryProposalsRequest> for proto::QueryProposalsRequest {
    fn from(req: QueryProposalsRequest) -> Self {
        Self {
            limit: req.limit,
            offset: req.offset,
            status_filter: req.status_filter.map(i32::from).unwrap_or(0),
            class_filter: req.class_filter.map(i32::from).unwrap_or(0),
            proposer_filter: req.proposer_filter.unwrap_or_default(),
        }
    }
}

/// Query a specific vote on a proposal.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProposalVoteRequest {
    pub proposal_id: u64,
    pub voter: String,
}

impl QueryProposalVoteRequest {
    pub fn new(proposal_id: u64, voter: impl Into<String>) -> Self {
        Self { proposal_id, voter: voter.into() }
    }
}

impl From<QueryProposalVoteRequest> for proto::QueryVoteRequest {
    fn from(req: QueryProposalVoteRequest) -> Self {
        Self {
            proposal_id: req.proposal_id,
            voter: req.voter,
        }
    }
}

/// Query all votes on a proposal (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProposalVotesRequest {
    pub proposal_id: u64,
    pub limit: i32,
    pub offset: i32,
}

impl QueryProposalVotesRequest {
    pub fn new(proposal_id: u64, limit: i32, offset: i32) -> Self {
        Self { proposal_id, limit, offset }
    }
}

impl From<QueryProposalVotesRequest> for proto::QueryVotesRequest {
    fn from(req: QueryProposalVotesRequest) -> Self {
        Self {
            proposal_id: req.proposal_id,
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query a specific deposit on a proposal.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProposalDepositRequest {
    pub proposal_id: u64,
    pub depositor: String,
}

impl QueryProposalDepositRequest {
    pub fn new(proposal_id: u64, depositor: impl Into<String>) -> Self {
        Self { proposal_id, depositor: depositor.into() }
    }
}

impl From<QueryProposalDepositRequest> for proto::QueryDepositRequest {
    fn from(req: QueryProposalDepositRequest) -> Self {
        Self {
            proposal_id: req.proposal_id,
            depositor: req.depositor,
        }
    }
}

/// Query all deposits on a proposal (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryProposalDepositsRequest {
    pub proposal_id: u64,
    pub limit: i32,
    pub offset: i32,
}

impl QueryProposalDepositsRequest {
    pub fn new(proposal_id: u64, limit: i32, offset: i32) -> Self {
        Self { proposal_id, limit, offset }
    }
}

impl From<QueryProposalDepositsRequest> for proto::QueryDepositsRequest {
    fn from(req: QueryProposalDepositsRequest) -> Self {
        Self {
            proposal_id: req.proposal_id,
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query the tally result for a proposal.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryTallyResultRequest {
    pub proposal_id: u64,
}

impl QueryTallyResultRequest {
    pub fn new(proposal_id: u64) -> Self {
        Self { proposal_id }
    }
}

impl From<QueryTallyResultRequest> for proto::QueryTallyResultRequest {
    fn from(req: QueryTallyResultRequest) -> Self {
        Self { proposal_id: req.proposal_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::VoteOption;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn submit_proposal_request_to_any() {
        let from = AccountId::new([1u8; 32]);
        let req = SubmitProposalRequest::new(
            from,
            ProposalClass::Standard,
            "Test Proposal",
            "A test proposal description",
            "1000000",
        );

        let any = req.to_any();
        assert_eq!(any.type_url, "/gov.v1.MsgSubmitProposalRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn proposal_deposit_request_to_any() {
        let from = AccountId::new([2u8; 32]);
        let req = ProposalDepositRequest::new(from, 1, "500000");

        let any = req.to_any();
        assert_eq!(any.type_url, "/gov.v1.MsgDepositRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn proposal_vote_request_to_any() {
        let from = AccountId::new([3u8; 32]);
        let options = alloc::vec![
            WeightedVoteOption::new(VoteOption::Yes, "0.7"),
            WeightedVoteOption::new(VoteOption::Abstain, "0.3"),
        ];

        let req = ProposalVoteRequest::new(from, 1, options)
            .with_conviction(3);

        let any = req.to_any();
        assert_eq!(any.type_url, "/gov.v1.MsgVoteRequest");
        assert!(!any.value.is_empty());
        assert_eq!(req.conviction_multiplier, 3);
    }

    #[test]
    fn query_proposals_with_filters() {
        let req = QueryProposalsRequest::new(20, 0)
            .status_filter(ProposalStatus::VotingPeriod)
            .class_filter(ProposalClass::Market)
            .proposer_filter("morpheum1abc");

        let proto_req: proto::QueryProposalsRequest = req.into();
        assert_eq!(proto_req.limit, 20);
        assert_eq!(proto_req.offset, 0);
        assert_eq!(proto_req.status_filter, i32::from(ProposalStatus::VotingPeriod));
        assert_eq!(proto_req.class_filter, i32::from(ProposalClass::Market));
        assert_eq!(proto_req.proposer_filter, "morpheum1abc");
    }
}
