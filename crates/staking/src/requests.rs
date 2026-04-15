//! Request and response wrappers for the Staking module.
//!
//! Clean, type-safe Rust APIs around the raw protobuf messages.
//! Includes `to_any()` methods for seamless integration with `TxBuilder`.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::staking::v1 as proto;

use crate::types::MisbehaviorType;

// ====================== TRANSACTION REQUESTS ======================

/// Request to stake MORM to a validator.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StakeRequest {
    pub address: String,
    pub validator_id: String,
    pub asset_index: u64,
    pub amount: String,
    pub external_address: Option<String>,
}

impl StakeRequest {
    pub fn new(
        address: impl Into<String>,
        validator_id: impl Into<String>,
        asset_index: u64,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            validator_id: validator_id.into(),
            asset_index,
            amount: amount.into(),
            external_address: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgStakeRequest = self.clone().into();
        ProtoAny {
            type_url: "/staking.v1.MsgStakeRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<StakeRequest> for proto::MsgStakeRequest {
    fn from(req: StakeRequest) -> Self {
        Self {
            address: req.address,
            validator_id: req.validator_id,
            asset_index: req.asset_index,
            amount: req.amount,
            timestamp: None,
            external_address: req.external_address,
        }
    }
}

/// Request to unstake MORM (with unbonding period).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnstakeRequest {
    pub address: String,
    pub validator_id: String,
    pub asset_index: u64,
    pub amount: String,
    pub external_address: Option<String>,
}

impl UnstakeRequest {
    pub fn new(
        address: impl Into<String>,
        validator_id: impl Into<String>,
        asset_index: u64,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            validator_id: validator_id.into(),
            asset_index,
            amount: amount.into(),
            external_address: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUnstakeRequest = self.clone().into();
        ProtoAny {
            type_url: "/staking.v1.MsgUnstakeRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UnstakeRequest> for proto::MsgUnstakeRequest {
    fn from(req: UnstakeRequest) -> Self {
        Self {
            address: req.address,
            validator_id: req.validator_id,
            asset_index: req.asset_index,
            amount: req.amount,
            timestamp: None,
            external_address: req.external_address,
        }
    }
}

/// Request to delegate MORM to a validator.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DelegateRequest {
    pub delegator_address: String,
    pub validator_id: String,
    pub asset_index: u64,
    pub amount: String,
    pub delegator_external_address: Option<String>,
}

impl DelegateRequest {
    pub fn new(
        delegator_address: impl Into<String>,
        validator_id: impl Into<String>,
        asset_index: u64,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            delegator_address: delegator_address.into(),
            validator_id: validator_id.into(),
            asset_index,
            amount: amount.into(),
            delegator_external_address: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgDelegateRequest = self.clone().into();
        ProtoAny {
            type_url: "/staking.v1.MsgDelegateRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<DelegateRequest> for proto::MsgDelegateRequest {
    fn from(req: DelegateRequest) -> Self {
        Self {
            delegator_address: req.delegator_address,
            validator_id: req.validator_id,
            asset_index: req.asset_index,
            amount: req.amount,
            timestamp: None,
            delegator_external_address: req.delegator_external_address,
        }
    }
}

/// Request to undelegate MORM from a validator (with unbonding period).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UndelegateRequest {
    pub delegator_address: String,
    pub validator_id: String,
    pub asset_index: u64,
    pub amount: String,
    pub delegator_external_address: Option<String>,
    pub delegator_chain_type: Option<i32>,
}

impl UndelegateRequest {
    pub fn new(
        delegator_address: impl Into<String>,
        validator_id: impl Into<String>,
        asset_index: u64,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            delegator_address: delegator_address.into(),
            validator_id: validator_id.into(),
            asset_index,
            amount: amount.into(),
            delegator_external_address: None,
            delegator_chain_type: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUndelegateRequest = self.clone().into();
        ProtoAny {
            type_url: "/staking.v1.MsgUndelegateRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UndelegateRequest> for proto::MsgUndelegateRequest {
    fn from(req: UndelegateRequest) -> Self {
        Self {
            delegator_address: req.delegator_address,
            validator_id: req.validator_id,
            asset_index: req.asset_index,
            amount: req.amount,
            timestamp: None,
            delegator_external_address: req.delegator_external_address,
            delegator_chain_type: req.delegator_chain_type,
        }
    }
}

/// Request to redelegate MORM between validators.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RedelegateRequest {
    pub delegator_address: String,
    pub from_validator_id: String,
    pub to_validator_id: String,
    pub asset_index: u64,
    pub amount: String,
    pub delegator_external_address: Option<String>,
    pub delegator_chain_type: Option<i32>,
}

impl RedelegateRequest {
    pub fn new(
        delegator_address: impl Into<String>,
        from_validator_id: impl Into<String>,
        to_validator_id: impl Into<String>,
        asset_index: u64,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            delegator_address: delegator_address.into(),
            from_validator_id: from_validator_id.into(),
            to_validator_id: to_validator_id.into(),
            asset_index,
            amount: amount.into(),
            delegator_external_address: None,
            delegator_chain_type: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgRedelegateRequest = self.clone().into();
        ProtoAny {
            type_url: "/staking.v1.MsgRedelegateRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<RedelegateRequest> for proto::MsgRedelegateRequest {
    fn from(req: RedelegateRequest) -> Self {
        Self {
            delegator_address: req.delegator_address,
            from_validator_id: req.from_validator_id,
            to_validator_id: req.to_validator_id,
            asset_index: req.asset_index,
            amount: req.amount,
            timestamp: None,
            delegator_external_address: req.delegator_external_address,
            delegator_chain_type: req.delegator_chain_type,
        }
    }
}

/// Request to claim accumulated staking rewards.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClaimRewardsRequest {
    pub address: String,
    pub validator_id: String,
    pub external_address: Option<String>,
}

impl ClaimRewardsRequest {
    pub fn new(
        address: impl Into<String>,
        validator_id: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            validator_id: validator_id.into(),
            external_address: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgClaimRewardsRequest = self.clone().into();
        ProtoAny {
            type_url: "/staking.v1.MsgClaimRewardsRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ClaimRewardsRequest> for proto::MsgClaimRewardsRequest {
    fn from(req: ClaimRewardsRequest) -> Self {
        Self {
            address: req.address,
            validator_id: req.validator_id,
            timestamp: None,
            external_address: req.external_address,
        }
    }
}

/// Request to report validator misbehavior.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReportMisbehaviorRequest {
    pub validator_id: String,
    pub misbehavior_type: MisbehaviorType,
    pub evidence: Vec<u8>,
    pub severity: String,
    pub height: u64,
    pub sig: Vec<u8>,
    pub reporter_address: String,
    pub reporter_external_address: Option<String>,
    pub reporter_chain_type: Option<i32>,
}

impl ReportMisbehaviorRequest {
    pub fn new(
        validator_id: impl Into<String>,
        misbehavior_type: MisbehaviorType,
        evidence: Vec<u8>,
        severity: impl Into<String>,
        reporter_address: impl Into<String>,
        sig: Vec<u8>,
    ) -> Self {
        Self {
            validator_id: validator_id.into(),
            misbehavior_type,
            evidence,
            severity: severity.into(),
            height: 0,
            sig,
            reporter_address: reporter_address.into(),
            reporter_external_address: None,
            reporter_chain_type: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgReportMisbehaviorRequest = self.clone().into();
        ProtoAny {
            type_url: "/staking.v1.MsgReportMisbehaviorRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ReportMisbehaviorRequest> for proto::MsgReportMisbehaviorRequest {
    fn from(req: ReportMisbehaviorRequest) -> Self {
        Self {
            validator_id: req.validator_id,
            misbehavior_type: i32::from(req.misbehavior_type),
            evidence: req.evidence,
            severity: req.severity,
            height: req.height,
            sig: req.sig,
            timestamp: None,
            reporter_address: req.reporter_address,
            reporter_external_address: req.reporter_external_address,
            reporter_chain_type: req.reporter_chain_type,
        }
    }
}

/// Request to vote on a slashing proposal.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VoteOnSlashingRequest {
    pub misbehavior_id: String,
    pub vote: bool,
    pub voter_address: String,
    pub sig: Vec<u8>,
    pub voter_external_address: Option<String>,
    pub voter_chain_type: Option<i32>,
}

impl VoteOnSlashingRequest {
    pub fn new(
        misbehavior_id: impl Into<String>,
        vote: bool,
        voter_address: impl Into<String>,
        sig: Vec<u8>,
    ) -> Self {
        Self {
            misbehavior_id: misbehavior_id.into(),
            vote,
            voter_address: voter_address.into(),
            sig,
            voter_external_address: None,
            voter_chain_type: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgVoteOnSlashingRequest = self.clone().into();
        ProtoAny {
            type_url: "/staking.v1.MsgVoteOnSlashingRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<VoteOnSlashingRequest> for proto::MsgVoteOnSlashingRequest {
    fn from(req: VoteOnSlashingRequest) -> Self {
        Self {
            misbehavior_id: req.misbehavior_id,
            vote: req.vote,
            voter_address: req.voter_address,
            sig: req.sig,
            timestamp: None,
            voter_external_address: req.voter_external_address,
            voter_chain_type: req.voter_chain_type,
        }
    }
}

/// Request to apply a slashing penalty.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ApplySlashingRequest {
    pub misbehavior_id: String,
    pub validator_id: String,
    pub slash_type: String,
    pub asset_index: u64,
    pub balance_penalty: String,
    pub reputation_penalty: String,
    pub quorum_votes: BTreeMap<String, bool>,
    pub sig: Vec<u8>,
}

impl ApplySlashingRequest {
    pub fn new(
        misbehavior_id: impl Into<String>,
        validator_id: impl Into<String>,
        slash_type: impl Into<String>,
        sig: Vec<u8>,
    ) -> Self {
        Self {
            misbehavior_id: misbehavior_id.into(),
            validator_id: validator_id.into(),
            slash_type: slash_type.into(),
            asset_index: 0,
            balance_penalty: String::new(),
            reputation_penalty: String::new(),
            quorum_votes: BTreeMap::new(),
            sig,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgApplySlashingRequest = self.clone().into();
        ProtoAny {
            type_url: "/staking.v1.MsgApplySlashingRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ApplySlashingRequest> for proto::MsgApplySlashingRequest {
    fn from(req: ApplySlashingRequest) -> Self {
        Self {
            misbehavior_id: req.misbehavior_id,
            validator_id: req.validator_id,
            slash_type: req.slash_type,
            asset_index: req.asset_index,
            balance_penalty: req.balance_penalty,
            reputation_penalty: req.reputation_penalty,
            quorum_votes: req.quorum_votes.into_iter().collect(),
            sig: req.sig,
            timestamp: None,
        }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query a single validator by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryValidatorRequest {
    pub validator_id: String,
}

impl QueryValidatorRequest {
    pub fn new(validator_id: impl Into<String>) -> Self {
        Self { validator_id: validator_id.into() }
    }
}

impl From<QueryValidatorRequest> for proto::QueryValidatorRequest {
    fn from(req: QueryValidatorRequest) -> Self {
        Self { validator_id: req.validator_id }
    }
}

/// Query validators with optional filtering.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryValidatorsRequest {
    pub active_only: bool,
    pub limit: i32,
    pub offset: i32,
}

impl QueryValidatorsRequest {
    pub fn new(limit: i32, offset: i32) -> Self {
        Self { active_only: false, limit, offset }
    }

    pub fn active_only(mut self) -> Self {
        self.active_only = true;
        self
    }
}

impl From<QueryValidatorsRequest> for proto::QueryValidatorsRequest {
    fn from(req: QueryValidatorsRequest) -> Self {
        Self {
            active_only: req.active_only,
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query user's staking overview.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryUserStakingRequest {
    pub address: String,
}

impl QueryUserStakingRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self { address: address.into() }
    }
}

impl From<QueryUserStakingRequest> for proto::QueryUserStakingRequest {
    fn from(req: QueryUserStakingRequest) -> Self {
        Self {
            address: req.address,
            external_address: None,
            chain_type: None,
        }
    }
}

/// Query delegations for an address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDelegationsRequest {
    pub address: String,
    pub limit: i32,
    pub offset: i32,
}

impl QueryDelegationsRequest {
    pub fn new(address: impl Into<String>, limit: i32, offset: i32) -> Self {
        Self { address: address.into(), limit, offset }
    }
}

impl From<QueryDelegationsRequest> for proto::QueryDelegationsRequest {
    fn from(req: QueryDelegationsRequest) -> Self {
        Self {
            address: req.address,
            limit: req.limit,
            offset: req.offset,
            external_address: None,
            chain_type: None,
        }
    }
}

/// Query rewards for a user / validator combination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryRewardsRequest {
    pub address: String,
    pub validator_id: String,
    pub epoch: u64,
}

impl QueryRewardsRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            validator_id: String::new(),
            epoch: 0,
        }
    }

    pub fn with_validator(mut self, validator_id: impl Into<String>) -> Self {
        self.validator_id = validator_id.into();
        self
    }

    pub fn with_epoch(mut self, epoch: u64) -> Self {
        self.epoch = epoch;
        self
    }
}

impl From<QueryRewardsRequest> for proto::QueryRewardsRequest {
    fn from(req: QueryRewardsRequest) -> Self {
        Self {
            validator_id: req.validator_id,
            address: req.address,
            epoch: req.epoch,
            external_address: None,
            chain_type: None,
        }
    }
}

/// Governance-only request to update staking module parameters.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    pub authority: String,
    pub params: proto::Params,
}

impl UpdateParamsRequest {
    pub fn new(authority: impl Into<String>, params: proto::Params) -> Self {
        Self {
            authority: authority.into(),
            params,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateParams {
            authority: self.authority.clone(),
            params: Some(self.params.clone()),
        };
        ProtoAny {
            type_url: "/staking.v1.MsgUpdateParams".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Query module parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsRequest;

impl From<QueryParamsRequest> for proto::QueryParamsRequest {
    fn from(_req: QueryParamsRequest) -> Self {
        Self {}
    }
}

/// Query epoch reward distribution snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryEpochRewardsRequest {
    pub epoch: u64,
}

impl QueryEpochRewardsRequest {
    pub fn new(epoch: u64) -> Self {
        Self { epoch }
    }
}

impl From<QueryEpochRewardsRequest> for proto::QueryEpochRewardsRequest {
    fn from(req: QueryEpochRewardsRequest) -> Self {
        Self { epoch: req.epoch }
    }
}

/// Query a validator's computed score.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryValidatorScoreRequest {
    pub validator_id: String,
    pub epoch: u64,
}

impl QueryValidatorScoreRequest {
    pub fn new(validator_id: impl Into<String>) -> Self {
        Self { validator_id: validator_id.into(), epoch: 0 }
    }

    pub fn with_epoch(mut self, epoch: u64) -> Self {
        self.epoch = epoch;
        self
    }
}

impl From<QueryValidatorScoreRequest> for proto::QueryValidatorScoreRequest {
    fn from(req: QueryValidatorScoreRequest) -> Self {
        Self { validator_id: req.validator_id, epoch: req.epoch }
    }
}

/// Query a validator's commission info.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryCommissionInfoRequest {
    pub validator_id: String,
}

impl QueryCommissionInfoRequest {
    pub fn new(validator_id: impl Into<String>) -> Self {
        Self { validator_id: validator_id.into() }
    }
}

impl From<QueryCommissionInfoRequest> for proto::QueryCommissionInfoRequest {
    fn from(req: QueryCommissionInfoRequest) -> Self {
        Self { validator_id: req.validator_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn stake_request_to_any() {
        let req = StakeRequest::new("morm1abc", "val-1", 0, "1000000");
        let any = req.to_any();
        assert_eq!(any.type_url, "/staking.v1.MsgStakeRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn delegate_request_to_any() {
        let req = DelegateRequest::new("morm1abc", "val-1", 0, "500000");
        let any = req.to_any();
        assert_eq!(any.type_url, "/staking.v1.MsgDelegateRequest");
    }

    #[test]
    fn redelegate_request_to_any() {
        let req = RedelegateRequest::new("morm1abc", "val-1", "val-2", 0, "250000");
        let any = req.to_any();
        assert_eq!(any.type_url, "/staking.v1.MsgRedelegateRequest");
    }

    #[test]
    fn claim_rewards_request_to_any() {
        let req = ClaimRewardsRequest::new("morm1abc", "val-1");
        let any = req.to_any();
        assert_eq!(any.type_url, "/staking.v1.MsgClaimRewardsRequest");
    }

    #[test]
    fn report_misbehavior_request_to_any() {
        let req = ReportMisbehaviorRequest::new(
            "val-1",
            MisbehaviorType::DoubleVote,
            vec![1, 2, 3],
            "critical",
            "morm1reporter",
            vec![4, 5, 6],
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/staking.v1.MsgReportMisbehaviorRequest");
    }

    #[test]
    fn vote_on_slashing_request_to_any() {
        let req = VoteOnSlashingRequest::new("misb-1", true, "morm1voter", vec![7, 8]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/staking.v1.MsgVoteOnSlashingRequest");
    }

    #[test]
    fn apply_slashing_request_to_any() {
        let req = ApplySlashingRequest::new("misb-1", "val-1", "both", vec![9, 10]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/staking.v1.MsgApplySlashingRequest");
    }

}
