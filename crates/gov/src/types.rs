//! Domain types for the Governance module.
//!
//! Clean, idiomatic Rust representations of the governance protobuf messages.
//! Full round-trip conversion to/from protobuf, strictly `no_std` compatible.

use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::gov::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

// ====================== ENUMS ======================

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProposalClass {
    Unspecified,
    Standard,
    Expedited,
    Emergency,
    Root,
    Market,
    Treasury,
    EmergencyMarket,
}

impl From<i32> for ProposalClass {
    fn from(v: i32) -> Self {
        match proto::ProposalClass::try_from(v).unwrap_or(proto::ProposalClass::Unspecified) {
            proto::ProposalClass::Unspecified => Self::Unspecified,
            proto::ProposalClass::Standard => Self::Standard,
            proto::ProposalClass::Expedited => Self::Expedited,
            proto::ProposalClass::Emergency => Self::Emergency,
            proto::ProposalClass::Root => Self::Root,
            proto::ProposalClass::Market => Self::Market,
            proto::ProposalClass::Treasury => Self::Treasury,
            proto::ProposalClass::EmergencyMarket => Self::EmergencyMarket,
        }
    }
}

impl From<ProposalClass> for i32 {
    fn from(c: ProposalClass) -> Self {
        match c {
            ProposalClass::Unspecified => proto::ProposalClass::Unspecified as i32,
            ProposalClass::Standard => proto::ProposalClass::Standard as i32,
            ProposalClass::Expedited => proto::ProposalClass::Expedited as i32,
            ProposalClass::Emergency => proto::ProposalClass::Emergency as i32,
            ProposalClass::Root => proto::ProposalClass::Root as i32,
            ProposalClass::Market => proto::ProposalClass::Market as i32,
            ProposalClass::Treasury => proto::ProposalClass::Treasury as i32,
            ProposalClass::EmergencyMarket => proto::ProposalClass::EmergencyMarket as i32,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ProposalStatus {
    Unspecified,
    DepositPeriod,
    VotingPeriod,
    Passed,
    Rejected,
    Failed,
    Cancelled,
}

impl From<i32> for ProposalStatus {
    fn from(v: i32) -> Self {
        match proto::ProposalStatus::try_from(v).unwrap_or(proto::ProposalStatus::Unspecified) {
            proto::ProposalStatus::Unspecified => Self::Unspecified,
            proto::ProposalStatus::DepositPeriod => Self::DepositPeriod,
            proto::ProposalStatus::VotingPeriod => Self::VotingPeriod,
            proto::ProposalStatus::Passed => Self::Passed,
            proto::ProposalStatus::Rejected => Self::Rejected,
            proto::ProposalStatus::Failed => Self::Failed,
            proto::ProposalStatus::Cancelled => Self::Cancelled,
        }
    }
}

impl From<ProposalStatus> for i32 {
    fn from(s: ProposalStatus) -> Self {
        match s {
            ProposalStatus::Unspecified => proto::ProposalStatus::Unspecified as i32,
            ProposalStatus::DepositPeriod => proto::ProposalStatus::DepositPeriod as i32,
            ProposalStatus::VotingPeriod => proto::ProposalStatus::VotingPeriod as i32,
            ProposalStatus::Passed => proto::ProposalStatus::Passed as i32,
            ProposalStatus::Rejected => proto::ProposalStatus::Rejected as i32,
            ProposalStatus::Failed => proto::ProposalStatus::Failed as i32,
            ProposalStatus::Cancelled => proto::ProposalStatus::Cancelled as i32,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VoteOption {
    Unspecified,
    Yes,
    Abstain,
    No,
    NoWithVeto,
}

impl From<i32> for VoteOption {
    fn from(v: i32) -> Self {
        match proto::VoteOption::try_from(v).unwrap_or(proto::VoteOption::Unspecified) {
            proto::VoteOption::Unspecified => Self::Unspecified,
            proto::VoteOption::Yes => Self::Yes,
            proto::VoteOption::Abstain => Self::Abstain,
            proto::VoteOption::No => Self::No,
            proto::VoteOption::NoWithVeto => Self::NoWithVeto,
        }
    }
}

impl From<VoteOption> for i32 {
    fn from(o: VoteOption) -> Self {
        match o {
            VoteOption::Unspecified => proto::VoteOption::Unspecified as i32,
            VoteOption::Yes => proto::VoteOption::Yes as i32,
            VoteOption::Abstain => proto::VoteOption::Abstain as i32,
            VoteOption::No => proto::VoteOption::No as i32,
            VoteOption::NoWithVeto => proto::VoteOption::NoWithVeto as i32,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UpgradeStatus {
    Unspecified,
    Scheduled,
    ShadowMode,
    Ready,
    Activated,
    Cancelled,
}

impl From<i32> for UpgradeStatus {
    fn from(v: i32) -> Self {
        match proto::UpgradeStatus::try_from(v).unwrap_or(proto::UpgradeStatus::Unspecified) {
            proto::UpgradeStatus::Unspecified => Self::Unspecified,
            proto::UpgradeStatus::Scheduled => Self::Scheduled,
            proto::UpgradeStatus::ShadowMode => Self::ShadowMode,
            proto::UpgradeStatus::Ready => Self::Ready,
            proto::UpgradeStatus::Activated => Self::Activated,
            proto::UpgradeStatus::Cancelled => Self::Cancelled,
        }
    }
}

impl From<UpgradeStatus> for i32 {
    fn from(s: UpgradeStatus) -> Self {
        match s {
            UpgradeStatus::Unspecified => proto::UpgradeStatus::Unspecified as i32,
            UpgradeStatus::Scheduled => proto::UpgradeStatus::Scheduled as i32,
            UpgradeStatus::ShadowMode => proto::UpgradeStatus::ShadowMode as i32,
            UpgradeStatus::Ready => proto::UpgradeStatus::Ready as i32,
            UpgradeStatus::Activated => proto::UpgradeStatus::Activated as i32,
            UpgradeStatus::Cancelled => proto::UpgradeStatus::Cancelled as i32,
        }
    }
}

// ====================== STRUCT TYPES ======================

/// Weighted vote option for split voting.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WeightedVoteOption {
    pub option: VoteOption,
    pub weight: String,
}

impl WeightedVoteOption {
    pub fn new(option: VoteOption, weight: impl Into<String>) -> Self {
        Self { option, weight: weight.into() }
    }
}

impl From<proto::WeightedVoteOption> for WeightedVoteOption {
    fn from(p: proto::WeightedVoteOption) -> Self {
        Self {
            option: VoteOption::from(p.option),
            weight: p.weight,
        }
    }
}

impl From<WeightedVoteOption> for proto::WeightedVoteOption {
    fn from(w: WeightedVoteOption) -> Self {
        Self {
            option: i32::from(w.option),
            weight: w.weight,
        }
    }
}

/// Tally result for a governance proposal.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TallyResult {
    pub yes: String,
    pub abstain: String,
    pub no: String,
    pub no_with_veto: String,
    pub total_voted: String,
}

impl From<proto::TallyResult> for TallyResult {
    fn from(p: proto::TallyResult) -> Self {
        Self {
            yes: p.yes,
            abstain: p.abstain,
            no: p.no,
            no_with_veto: p.no_with_veto,
            total_voted: p.total_voted,
        }
    }
}

impl From<TallyResult> for proto::TallyResult {
    fn from(t: TallyResult) -> Self {
        Self {
            yes: t.yes,
            abstain: t.abstain,
            no: t.no,
            no_with_veto: t.no_with_veto,
            total_voted: t.total_voted,
        }
    }
}

/// Per-class governance parameters (track-specific overrides).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProposalClassParams {
    pub min_deposit: String,
    pub voting_period: String,
    pub threshold: String,
    pub quorum: String,
    pub veto_threshold: String,
    pub enactment_delay: String,
    pub allow_conviction: bool,
}

impl From<proto::ProposalClassParams> for ProposalClassParams {
    fn from(p: proto::ProposalClassParams) -> Self {
        Self {
            min_deposit: p.min_deposit,
            voting_period: p.voting_period,
            threshold: p.threshold,
            quorum: p.quorum,
            veto_threshold: p.veto_threshold,
            enactment_delay: p.enactment_delay,
            allow_conviction: p.allow_conviction,
        }
    }
}

impl From<ProposalClassParams> for proto::ProposalClassParams {
    fn from(p: ProposalClassParams) -> Self {
        Self {
            min_deposit: p.min_deposit,
            voting_period: p.voting_period,
            threshold: p.threshold,
            quorum: p.quorum,
            veto_threshold: p.veto_threshold,
            enactment_delay: p.enactment_delay,
            allow_conviction: p.allow_conviction,
        }
    }
}

/// Global governance parameters (self-governed via proposals).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GovParams {
    pub min_deposit: String,
    pub max_deposit_period: String,
    pub min_initial_deposit_ratio: String,
    pub proposal_cancel_ratio: String,
    pub proposal_cancel_dest: String,
    pub voting_period: String,
    pub expedited_voting_period: String,
    pub emergency_voting_period: String,
    pub quorum: String,
    pub threshold: String,
    pub veto_threshold: String,
    pub burn_vote_quorum: String,
    pub burn_proposal_deposit_prevote: String,
    pub burn_vote_veto: String,
    pub class_params: BTreeMap<String, ProposalClassParams>,
    pub constitution: String,
    pub max_proposals_per_block: u64,
    pub min_enactment_delay: String,
    pub max_proposals_per_epoch: u64,
    pub max_market_proposals_per_block: u64,
    pub min_grace_period: String,
}

impl From<proto::GovParams> for GovParams {
    fn from(p: proto::GovParams) -> Self {
        Self {
            min_deposit: p.min_deposit,
            max_deposit_period: p.max_deposit_period,
            min_initial_deposit_ratio: p.min_initial_deposit_ratio,
            proposal_cancel_ratio: p.proposal_cancel_ratio,
            proposal_cancel_dest: p.proposal_cancel_dest,
            voting_period: p.voting_period,
            expedited_voting_period: p.expedited_voting_period,
            emergency_voting_period: p.emergency_voting_period,
            quorum: p.quorum,
            threshold: p.threshold,
            veto_threshold: p.veto_threshold,
            burn_vote_quorum: p.burn_vote_quorum,
            burn_proposal_deposit_prevote: p.burn_proposal_deposit_prevote,
            burn_vote_veto: p.burn_vote_veto,
            class_params: p.class_params.into_iter().map(|(k, v)| (k, v.into())).collect(),
            constitution: p.constitution,
            max_proposals_per_block: p.max_proposals_per_block,
            min_enactment_delay: p.min_enactment_delay,
            max_proposals_per_epoch: p.max_proposals_per_epoch,
            max_market_proposals_per_block: p.max_market_proposals_per_block,
            min_grace_period: p.min_grace_period,
        }
    }
}

impl From<GovParams> for proto::GovParams {
    fn from(p: GovParams) -> Self {
        Self {
            min_deposit: p.min_deposit,
            max_deposit_period: p.max_deposit_period,
            min_initial_deposit_ratio: p.min_initial_deposit_ratio,
            proposal_cancel_ratio: p.proposal_cancel_ratio,
            proposal_cancel_dest: p.proposal_cancel_dest,
            voting_period: p.voting_period,
            expedited_voting_period: p.expedited_voting_period,
            emergency_voting_period: p.emergency_voting_period,
            quorum: p.quorum,
            threshold: p.threshold,
            veto_threshold: p.veto_threshold,
            burn_vote_quorum: p.burn_vote_quorum,
            burn_proposal_deposit_prevote: p.burn_proposal_deposit_prevote,
            burn_vote_veto: p.burn_vote_veto,
            class_params: p.class_params.into_iter().map(|(k, v)| (k, v.into())).collect(),
            constitution: p.constitution,
            max_proposals_per_block: p.max_proposals_per_block,
            min_enactment_delay: p.min_enactment_delay,
            max_proposals_per_epoch: p.max_proposals_per_epoch,
            max_market_proposals_per_block: p.max_market_proposals_per_block,
            min_grace_period: p.min_grace_period,
        }
    }
}

/// Zero-downtime software/runtime upgrade plan.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpgradePlan {
    pub name: String,
    pub info: String,
    pub activation_staple_id: u64,
    pub activation_time: u64,
    pub binary_hash: Vec<u8>,
    pub grace_period_seconds: u64,
    pub additional_metadata: BTreeMap<String, String>,
}

impl From<proto::UpgradePlan> for UpgradePlan {
    fn from(p: proto::UpgradePlan) -> Self {
        Self {
            name: p.name,
            info: p.info,
            activation_staple_id: p.activation_staple_id,
            activation_time: timestamp_seconds(p.activation_time),
            binary_hash: p.binary_hash,
            grace_period_seconds: p.grace_period_seconds,
            additional_metadata: p.additional_metadata.into_iter().collect(),
        }
    }
}

impl From<UpgradePlan> for proto::UpgradePlan {
    fn from(p: UpgradePlan) -> Self {
        Self {
            name: p.name,
            info: p.info,
            activation_staple_id: p.activation_staple_id,
            activation_time: Some(morpheum_proto::google::protobuf::Timestamp {
                seconds: p.activation_time as i64,
                nanos: 0,
            }),
            binary_hash: p.binary_hash,
            grace_period_seconds: p.grace_period_seconds,
            additional_metadata: p.additional_metadata.into_iter().collect(),
        }
    }
}

/// Core governance proposal.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Proposal {
    pub proposal_id: u64,
    pub proposal_class: ProposalClass,
    pub messages: Vec<ProtoAny>,
    pub title: String,
    pub description: String,
    pub metadata: String,
    pub submit_time: u64,
    pub deposit_end_time: u64,
    pub voting_start_time: u64,
    pub voting_end_time: u64,
    pub status: ProposalStatus,
    pub final_tally_result: Option<TallyResult>,
    pub total_deposit: String,
    pub proposer: String,
    pub enactment_time: u64,
    pub upgrade_status: UpgradeStatus,
    pub upgrade_plan_id: u64,
    pub additional_metadata: BTreeMap<String, String>,
}

impl Proposal {
    /// Whether the proposal is still accepting deposits.
    pub fn is_deposit_period(&self) -> bool {
        matches!(self.status, ProposalStatus::DepositPeriod)
    }

    /// Whether the proposal is currently in the voting period.
    pub fn is_voting_period(&self) -> bool {
        matches!(self.status, ProposalStatus::VotingPeriod)
    }

    /// Whether the proposal has passed and is awaiting execution.
    pub fn is_passed(&self) -> bool {
        matches!(self.status, ProposalStatus::Passed)
    }
}

impl From<proto::Proposal> for Proposal {
    fn from(p: proto::Proposal) -> Self {
        Self {
            proposal_id: p.proposal_id,
            proposal_class: ProposalClass::from(p.proposal_class),
            messages: p.messages,
            title: p.title,
            description: p.description,
            metadata: p.metadata,
            submit_time: timestamp_seconds(p.submit_time),
            deposit_end_time: timestamp_seconds(p.deposit_end_time),
            voting_start_time: timestamp_seconds(p.voting_start_time),
            voting_end_time: timestamp_seconds(p.voting_end_time),
            status: ProposalStatus::from(p.status),
            final_tally_result: p.final_tally_result.map(Into::into),
            total_deposit: p.total_deposit,
            proposer: p.proposer,
            enactment_time: timestamp_seconds(p.enactment_time),
            upgrade_status: UpgradeStatus::from(p.upgrade_status),
            upgrade_plan_id: p.upgrade_plan_id,
            additional_metadata: p.additional_metadata.into_iter().collect(),
        }
    }
}

impl From<Proposal> for proto::Proposal {
    fn from(p: Proposal) -> Self {
        Self {
            proposal_id: p.proposal_id,
            proposal_class: i32::from(p.proposal_class),
            messages: p.messages,
            title: p.title,
            description: p.description,
            metadata: p.metadata,
            submit_time: to_timestamp(p.submit_time),
            deposit_end_time: to_timestamp(p.deposit_end_time),
            voting_start_time: to_timestamp(p.voting_start_time),
            voting_end_time: to_timestamp(p.voting_end_time),
            status: i32::from(p.status),
            final_tally_result: p.final_tally_result.map(Into::into),
            total_deposit: p.total_deposit,
            proposer: p.proposer,
            enactment_time: to_timestamp(p.enactment_time),
            upgrade_status: i32::from(p.upgrade_status),
            upgrade_plan_id: p.upgrade_plan_id,
            additional_metadata: p.additional_metadata.into_iter().collect(),
        }
    }
}

/// A deposit on a governance proposal.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Deposit {
    pub proposal_id: u64,
    pub depositor: String,
    pub amount: String,
    pub deposit_time: u64,
}

impl From<proto::Deposit> for Deposit {
    fn from(p: proto::Deposit) -> Self {
        Self {
            proposal_id: p.proposal_id,
            depositor: p.depositor,
            amount: p.amount,
            deposit_time: timestamp_seconds(p.deposit_time),
        }
    }
}

impl From<Deposit> for proto::Deposit {
    fn from(d: Deposit) -> Self {
        Self {
            proposal_id: d.proposal_id,
            depositor: d.depositor,
            amount: d.amount,
            deposit_time: to_timestamp(d.deposit_time),
        }
    }
}

/// A vote on a governance proposal (supports weighted split + conviction).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vote {
    pub proposal_id: u64,
    pub voter: String,
    pub options: Vec<WeightedVoteOption>,
    pub vote_time: u64,
    pub conviction_multiplier: u64,
    pub locked_until: String,
}

impl From<proto::Vote> for Vote {
    fn from(p: proto::Vote) -> Self {
        Self {
            proposal_id: p.proposal_id,
            voter: p.voter,
            options: p.options.into_iter().map(Into::into).collect(),
            vote_time: timestamp_seconds(p.vote_time),
            conviction_multiplier: p.conviction_multiplier,
            locked_until: p.locked_until,
        }
    }
}

impl From<Vote> for proto::Vote {
    fn from(v: Vote) -> Self {
        Self {
            proposal_id: v.proposal_id,
            voter: v.voter,
            options: v.options.into_iter().map(Into::into).collect(),
            vote_time: to_timestamp(v.vote_time),
            conviction_multiplier: v.conviction_multiplier,
            locked_until: v.locked_until,
        }
    }
}

/// Proposal lifecycle event (for streams).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProposalUpdate {
    pub proposal_id: u64,
    pub old_status: ProposalStatus,
    pub new_status: ProposalStatus,
    pub reason: String,
    pub tally: Option<TallyResult>,
    pub timestamp: u64,
}

impl From<proto::ProposalUpdate> for ProposalUpdate {
    fn from(p: proto::ProposalUpdate) -> Self {
        Self {
            proposal_id: p.proposal_id,
            old_status: ProposalStatus::from(p.old_status),
            new_status: ProposalStatus::from(p.new_status),
            reason: p.reason,
            tally: p.tally.map(Into::into),
            timestamp: timestamp_seconds(p.timestamp),
        }
    }
}

// ====================== HELPERS ======================

fn timestamp_seconds(ts: Option<morpheum_proto::google::protobuf::Timestamp>) -> u64 {
    ts.map(|t| t.seconds as u64).unwrap_or(0)
}

fn to_timestamp(seconds: u64) -> Option<morpheum_proto::google::protobuf::Timestamp> {
    Some(morpheum_proto::google::protobuf::Timestamp {
        seconds: seconds as i64,
        nanos: 0,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn proposal_class_roundtrip() {
        for v in 0..=7i32 {
            let class = ProposalClass::from(v);
            let back: i32 = class.into();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn proposal_status_roundtrip() {
        for v in 0..=6i32 {
            let status = ProposalStatus::from(v);
            let back: i32 = status.into();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn vote_option_roundtrip() {
        for v in 0..=4i32 {
            let opt = VoteOption::from(v);
            let back: i32 = opt.into();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn upgrade_status_roundtrip() {
        for v in 0..=5i32 {
            let status = UpgradeStatus::from(v);
            let back: i32 = status.into();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn tally_result_roundtrip() {
        let tally = TallyResult {
            yes: "1000".into(),
            abstain: "200".into(),
            no: "50".into(),
            no_with_veto: "10".into(),
            total_voted: "1260".into(),
        };
        let proto_tally: proto::TallyResult = tally.clone().into();
        let back: TallyResult = proto_tally.into();
        assert_eq!(tally, back);
    }

    #[test]
    fn weighted_vote_option_roundtrip() {
        let opt = WeightedVoteOption::new(VoteOption::Yes, "0.7");
        let proto_opt: proto::WeightedVoteOption = opt.clone().into();
        let back: WeightedVoteOption = proto_opt.into();
        assert_eq!(opt, back);
    }

    #[test]
    fn proposal_status_helpers() {
        let mut proposal = Proposal {
            proposal_id: 1,
            proposal_class: ProposalClass::Standard,
            messages: alloc::vec![],
            title: "Test".into(),
            description: "Test proposal".into(),
            metadata: String::new(),
            submit_time: 1_700_000_000,
            deposit_end_time: 1_700_100_000,
            voting_start_time: 0,
            voting_end_time: 0,
            status: ProposalStatus::DepositPeriod,
            final_tally_result: None,
            total_deposit: "1000".into(),
            proposer: "morpheum1abc".into(),
            enactment_time: 0,
            upgrade_status: UpgradeStatus::Unspecified,
            upgrade_plan_id: 0,
            additional_metadata: BTreeMap::new(),
        };

        assert!(proposal.is_deposit_period());
        assert!(!proposal.is_voting_period());
        assert!(!proposal.is_passed());

        proposal.status = ProposalStatus::VotingPeriod;
        assert!(proposal.is_voting_period());

        proposal.status = ProposalStatus::Passed;
        assert!(proposal.is_passed());
    }
}
