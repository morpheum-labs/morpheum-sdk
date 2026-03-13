//! Domain types for the DAO module.
//!
//! Clean, idiomatic Rust representations of the DAO protobuf messages.
//! Full round-trip conversion to/from protobuf, strictly `no_std` compatible.

use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::dao::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

// ====================== ENUMS ======================

/// Governance model of a DAO instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DaoType {
    Unspecified,
    Community,
    Council,
    Hybrid,
}

impl From<i32> for DaoType {
    fn from(v: i32) -> Self {
        match proto::DaoType::try_from(v).unwrap_or(proto::DaoType::Unspecified) {
            proto::DaoType::Unspecified => Self::Unspecified,
            proto::DaoType::Community => Self::Community,
            proto::DaoType::Council => Self::Council,
            proto::DaoType::Hybrid => Self::Hybrid,
        }
    }
}

impl From<DaoType> for i32 {
    fn from(t: DaoType) -> Self {
        match t {
            DaoType::Unspecified => proto::DaoType::Unspecified as i32,
            DaoType::Community => proto::DaoType::Community as i32,
            DaoType::Council => proto::DaoType::Council as i32,
            DaoType::Hybrid => proto::DaoType::Hybrid as i32,
        }
    }
}

/// Lifecycle status of a DAO instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DaoStatus {
    Unspecified,
    Active,
    Paused,
    Deprecated,
}

impl From<i32> for DaoStatus {
    fn from(v: i32) -> Self {
        match proto::DaoStatus::try_from(v).unwrap_or(proto::DaoStatus::Unspecified) {
            proto::DaoStatus::Unspecified => Self::Unspecified,
            proto::DaoStatus::Active => Self::Active,
            proto::DaoStatus::Paused => Self::Paused,
            proto::DaoStatus::Deprecated => Self::Deprecated,
        }
    }
}

impl From<DaoStatus> for i32 {
    fn from(s: DaoStatus) -> Self {
        match s {
            DaoStatus::Unspecified => proto::DaoStatus::Unspecified as i32,
            DaoStatus::Active => proto::DaoStatus::Active as i32,
            DaoStatus::Paused => proto::DaoStatus::Paused as i32,
            DaoStatus::Deprecated => proto::DaoStatus::Deprecated as i32,
        }
    }
}

/// Status of a proposal scoped to a specific DAO.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DaoProposalStatus {
    Unspecified,
    Draft,
    Signing,
    Voting,
    Succeeded,
    Defeated,
    Executed,
    Cancelled,
    Failed,
}

impl From<i32> for DaoProposalStatus {
    fn from(v: i32) -> Self {
        match proto::DaoProposalStatus::try_from(v).unwrap_or(proto::DaoProposalStatus::Unspecified) {
            proto::DaoProposalStatus::Unspecified => Self::Unspecified,
            proto::DaoProposalStatus::Draft => Self::Draft,
            proto::DaoProposalStatus::Signing => Self::Signing,
            proto::DaoProposalStatus::Voting => Self::Voting,
            proto::DaoProposalStatus::Succeeded => Self::Succeeded,
            proto::DaoProposalStatus::Defeated => Self::Defeated,
            proto::DaoProposalStatus::Executed => Self::Executed,
            proto::DaoProposalStatus::Cancelled => Self::Cancelled,
            proto::DaoProposalStatus::Failed => Self::Failed,
        }
    }
}

impl From<DaoProposalStatus> for i32 {
    fn from(s: DaoProposalStatus) -> Self {
        match s {
            DaoProposalStatus::Unspecified => proto::DaoProposalStatus::Unspecified as i32,
            DaoProposalStatus::Draft => proto::DaoProposalStatus::Draft as i32,
            DaoProposalStatus::Signing => proto::DaoProposalStatus::Signing as i32,
            DaoProposalStatus::Voting => proto::DaoProposalStatus::Voting as i32,
            DaoProposalStatus::Succeeded => proto::DaoProposalStatus::Succeeded as i32,
            DaoProposalStatus::Defeated => proto::DaoProposalStatus::Defeated as i32,
            DaoProposalStatus::Executed => proto::DaoProposalStatus::Executed as i32,
            DaoProposalStatus::Cancelled => proto::DaoProposalStatus::Cancelled as i32,
            DaoProposalStatus::Failed => proto::DaoProposalStatus::Failed as i32,
        }
    }
}

/// Vote option for DAO proposals.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DaoVoteOption {
    Unspecified,
    Yes,
    No,
    Abstain,
}

impl From<i32> for DaoVoteOption {
    fn from(v: i32) -> Self {
        match proto::DaoVoteOption::try_from(v).unwrap_or(proto::DaoVoteOption::Unspecified) {
            proto::DaoVoteOption::Unspecified => Self::Unspecified,
            proto::DaoVoteOption::Yes => Self::Yes,
            proto::DaoVoteOption::No => Self::No,
            proto::DaoVoteOption::Abstain => Self::Abstain,
        }
    }
}

impl From<DaoVoteOption> for i32 {
    fn from(o: DaoVoteOption) -> Self {
        match o {
            DaoVoteOption::Unspecified => proto::DaoVoteOption::Unspecified as i32,
            DaoVoteOption::Yes => proto::DaoVoteOption::Yes as i32,
            DaoVoteOption::No => proto::DaoVoteOption::No as i32,
            DaoVoteOption::Abstain => proto::DaoVoteOption::Abstain as i32,
        }
    }
}

// ====================== STRUCT TYPES ======================

/// Weighted vote option for split voting.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WeightedDaoVoteOption {
    pub option: DaoVoteOption,
    pub weight: String,
}

impl WeightedDaoVoteOption {
    pub fn new(option: DaoVoteOption, weight: impl Into<String>) -> Self {
        Self { option, weight: weight.into() }
    }
}

impl From<proto::WeightedDaoVoteOption> for WeightedDaoVoteOption {
    fn from(p: proto::WeightedDaoVoteOption) -> Self {
        Self {
            option: DaoVoteOption::from(p.option),
            weight: p.weight,
        }
    }
}

impl From<WeightedDaoVoteOption> for proto::WeightedDaoVoteOption {
    fn from(w: WeightedDaoVoteOption) -> Self {
        Self {
            option: i32::from(w.option),
            weight: w.weight,
        }
    }
}

/// Tally result for a DAO proposal.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DaoTallyResult {
    pub yes: String,
    pub no: String,
    pub abstain: String,
    pub total_voted: String,
    pub quorum_reached: bool,
    pub passed: bool,
}

impl From<proto::TallyResult> for DaoTallyResult {
    fn from(p: proto::TallyResult) -> Self {
        Self {
            yes: p.yes,
            no: p.no,
            abstain: p.abstain,
            total_voted: p.total_voted,
            quorum_reached: p.quorum_reached,
            passed: p.passed,
        }
    }
}

impl From<DaoTallyResult> for proto::TallyResult {
    fn from(t: DaoTallyResult) -> Self {
        Self {
            yes: t.yes,
            no: t.no,
            abstain: t.abstain,
            total_voted: t.total_voted,
            quorum_reached: t.quorum_reached,
            passed: t.passed,
        }
    }
}

/// Configuration parameters for a DAO (self-governed via its own proposals).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DaoConfig {
    pub voting_period: String,
    pub hold_up_time: String,
    pub min_deposit_for_proposal: String,
    pub quorum: String,
    pub approval_threshold: String,
    pub allow_council_override: bool,
    pub use_conviction_voting: bool,
    pub max_active_proposals: u64,
    pub plugin_configs: BTreeMap<String, String>,
}

impl From<proto::DaoConfig> for DaoConfig {
    fn from(p: proto::DaoConfig) -> Self {
        Self {
            voting_period: p.voting_period,
            hold_up_time: p.hold_up_time,
            min_deposit_for_proposal: p.min_deposit_for_proposal,
            quorum: p.quorum,
            approval_threshold: p.approval_threshold,
            allow_council_override: p.allow_council_override,
            use_conviction_voting: p.use_conviction_voting,
            max_active_proposals: p.max_active_proposals,
            plugin_configs: p.plugin_configs.into_iter().collect(),
        }
    }
}

impl From<DaoConfig> for proto::DaoConfig {
    fn from(c: DaoConfig) -> Self {
        Self {
            voting_period: c.voting_period,
            hold_up_time: c.hold_up_time,
            min_deposit_for_proposal: c.min_deposit_for_proposal,
            quorum: c.quorum,
            approval_threshold: c.approval_threshold,
            allow_council_override: c.allow_council_override,
            use_conviction_voting: c.use_conviction_voting,
            max_active_proposals: c.max_active_proposals,
            plugin_configs: c.plugin_configs.into_iter().collect(),
        }
    }
}

/// An on-chain resource controlled by a DAO.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GovernedAsset {
    pub asset_type: String,
    pub address: String,
    pub authority_type: String,
    pub current_authority: String,
    pub is_active: bool,
    pub added_at: u64,
}

impl From<proto::GovernedAsset> for GovernedAsset {
    fn from(p: proto::GovernedAsset) -> Self {
        Self {
            asset_type: p.asset_type,
            address: p.address,
            authority_type: p.authority_type,
            current_authority: p.current_authority,
            is_active: p.is_active,
            added_at: timestamp_seconds(p.added_at),
        }
    }
}

impl From<GovernedAsset> for proto::GovernedAsset {
    fn from(a: GovernedAsset) -> Self {
        Self {
            asset_type: a.asset_type,
            address: a.address,
            authority_type: a.authority_type,
            current_authority: a.current_authority,
            is_active: a.is_active,
            added_at: to_timestamp(a.added_at),
        }
    }
}

/// A permissionless DAO instance (Realm).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Dao {
    pub dao_id: u64,
    pub unique_key: Vec<u8>,
    pub name: String,
    pub description: String,
    pub dao_address: String,
    pub dao_type: DaoType,
    pub community_token_mint: String,
    pub council_token_mint: String,
    pub config: Option<DaoConfig>,
    pub status: DaoStatus,
    pub created_at: u64,
    pub creator: String,
    pub governed_assets: Vec<GovernedAsset>,
    pub metadata: BTreeMap<String, String>,
}

impl Dao {
    pub fn is_active(&self) -> bool {
        matches!(self.status, DaoStatus::Active)
    }

    pub fn is_hybrid(&self) -> bool {
        matches!(self.dao_type, DaoType::Hybrid)
    }
}

impl From<proto::Dao> for Dao {
    fn from(p: proto::Dao) -> Self {
        Self {
            dao_id: p.dao_id,
            unique_key: p.unique_key,
            name: p.name,
            description: p.description,
            dao_address: p.dao_address,
            dao_type: DaoType::from(p.dao_type),
            community_token_mint: p.community_token_mint,
            council_token_mint: p.council_token_mint,
            config: p.config.map(Into::into),
            status: DaoStatus::from(p.status),
            created_at: timestamp_seconds(p.created_at),
            creator: p.creator,
            governed_assets: p.governed_assets.into_iter().map(Into::into).collect(),
            metadata: p.metadata.into_iter().collect(),
        }
    }
}

impl From<Dao> for proto::Dao {
    fn from(d: Dao) -> Self {
        Self {
            dao_id: d.dao_id,
            unique_key: d.unique_key,
            name: d.name,
            description: d.description,
            dao_address: d.dao_address,
            dao_type: i32::from(d.dao_type),
            community_token_mint: d.community_token_mint,
            council_token_mint: d.council_token_mint,
            config: d.config.map(Into::into),
            status: i32::from(d.status),
            created_at: to_timestamp(d.created_at),
            creator: d.creator,
            governed_assets: d.governed_assets.into_iter().map(Into::into).collect(),
            metadata: d.metadata.into_iter().collect(),
        }
    }
}

/// A proposal scoped to one DAO.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DaoProposal {
    pub dao_id: u64,
    pub proposal_id: u64,
    pub title: String,
    pub description: String,
    pub metadata: String,
    pub status: DaoProposalStatus,
    pub instructions: Vec<ProtoAny>,
    pub created_at: u64,
    pub voting_start_time: u64,
    pub voting_end_time: u64,
    pub enactment_time: u64,
    pub proposer: String,
    pub tally: Option<DaoTallyResult>,
    pub additional_metadata: BTreeMap<String, String>,
}

impl DaoProposal {
    pub fn is_votable(&self) -> bool {
        matches!(self.status, DaoProposalStatus::Voting)
    }

    pub fn is_executable(&self) -> bool {
        matches!(self.status, DaoProposalStatus::Succeeded)
    }
}

impl From<proto::DaoProposal> for DaoProposal {
    fn from(p: proto::DaoProposal) -> Self {
        Self {
            dao_id: p.dao_id,
            proposal_id: p.proposal_id,
            title: p.title,
            description: p.description,
            metadata: p.metadata,
            status: DaoProposalStatus::from(p.status),
            instructions: p.instructions,
            created_at: timestamp_seconds(p.created_at),
            voting_start_time: timestamp_seconds(p.voting_start_time),
            voting_end_time: timestamp_seconds(p.voting_end_time),
            enactment_time: timestamp_seconds(p.enactment_time),
            proposer: p.proposer,
            tally: p.tally.map(Into::into),
            additional_metadata: p.additional_metadata.into_iter().collect(),
        }
    }
}

impl From<DaoProposal> for proto::DaoProposal {
    fn from(p: DaoProposal) -> Self {
        Self {
            dao_id: p.dao_id,
            proposal_id: p.proposal_id,
            title: p.title,
            description: p.description,
            metadata: p.metadata,
            status: i32::from(p.status),
            instructions: p.instructions,
            created_at: to_timestamp(p.created_at),
            voting_start_time: to_timestamp(p.voting_start_time),
            voting_end_time: to_timestamp(p.voting_end_time),
            enactment_time: to_timestamp(p.enactment_time),
            proposer: p.proposer,
            tally: p.tally.map(Into::into),
            additional_metadata: p.additional_metadata.into_iter().collect(),
        }
    }
}

/// A vote cast on a DAO proposal.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DaoVote {
    pub dao_id: u64,
    pub proposal_id: u64,
    pub voter: String,
    pub options: Vec<WeightedDaoVoteOption>,
    pub vote_time: u64,
    pub conviction_multiplier: u64,
    pub locked_until: String,
}

impl From<proto::DaoVote> for DaoVote {
    fn from(p: proto::DaoVote) -> Self {
        Self {
            dao_id: p.dao_id,
            proposal_id: p.proposal_id,
            voter: p.voter,
            options: p.options.into_iter().map(Into::into).collect(),
            vote_time: timestamp_seconds(p.vote_time),
            conviction_multiplier: p.conviction_multiplier,
            locked_until: p.locked_until,
        }
    }
}

impl From<DaoVote> for proto::DaoVote {
    fn from(v: DaoVote) -> Self {
        Self {
            dao_id: v.dao_id,
            proposal_id: v.proposal_id,
            voter: v.voter,
            options: v.options.into_iter().map(Into::into).collect(),
            vote_time: to_timestamp(v.vote_time),
            conviction_multiplier: v.conviction_multiplier,
            locked_until: v.locked_until,
        }
    }
}

/// Tokens locked for voting power in a DAO.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DaoDeposit {
    pub dao_id: u64,
    pub depositor: String,
    pub token_mint: String,
    pub amount: String,
    pub lock_until: u64,
}

impl From<proto::DaoDeposit> for DaoDeposit {
    fn from(p: proto::DaoDeposit) -> Self {
        Self {
            dao_id: p.dao_id,
            depositor: p.depositor,
            token_mint: p.token_mint,
            amount: p.amount,
            lock_until: timestamp_seconds(p.lock_until),
        }
    }
}

impl From<DaoDeposit> for proto::DaoDeposit {
    fn from(d: DaoDeposit) -> Self {
        Self {
            dao_id: d.dao_id,
            depositor: d.depositor,
            token_mint: d.token_mint,
            amount: d.amount,
            lock_until: to_timestamp(d.lock_until),
        }
    }
}

/// An installed voter/decision plugin.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DaoPlugin {
    pub plugin_type: String,
    pub plugin_address: String,
    pub config_params: BTreeMap<String, String>,
}

impl From<proto::DaoPlugin> for DaoPlugin {
    fn from(p: proto::DaoPlugin) -> Self {
        Self {
            plugin_type: p.plugin_type,
            plugin_address: p.plugin_address,
            config_params: p.config_params.into_iter().collect(),
        }
    }
}

impl From<DaoPlugin> for proto::DaoPlugin {
    fn from(p: DaoPlugin) -> Self {
        Self {
            plugin_type: p.plugin_type,
            plugin_address: p.plugin_address,
            config_params: p.config_params.into_iter().collect(),
        }
    }
}

/// DAO proposal lifecycle event (for streams).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DaoProposalUpdate {
    pub dao_id: u64,
    pub proposal_id: u64,
    pub old_status: DaoProposalStatus,
    pub new_status: DaoProposalStatus,
    pub reason: String,
    pub tally: Option<DaoTallyResult>,
    pub timestamp: u64,
}

impl From<proto::DaoProposalUpdate> for DaoProposalUpdate {
    fn from(p: proto::DaoProposalUpdate) -> Self {
        Self {
            dao_id: p.dao_id,
            proposal_id: p.proposal_id,
            old_status: DaoProposalStatus::from(p.old_status),
            new_status: DaoProposalStatus::from(p.new_status),
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
    fn dao_type_roundtrip() {
        for v in 0..=3i32 {
            let t = DaoType::from(v);
            let back: i32 = t.into();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn dao_status_roundtrip() {
        for v in 0..=3i32 {
            let s = DaoStatus::from(v);
            let back: i32 = s.into();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn dao_proposal_status_roundtrip() {
        for v in 0..=8i32 {
            let s = DaoProposalStatus::from(v);
            let back: i32 = s.into();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn dao_vote_option_roundtrip() {
        for v in 0..=3i32 {
            let o = DaoVoteOption::from(v);
            let back: i32 = o.into();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn tally_result_roundtrip() {
        let tally = DaoTallyResult {
            yes: "5000".into(),
            no: "1000".into(),
            abstain: "200".into(),
            total_voted: "6200".into(),
            quorum_reached: true,
            passed: true,
        };
        let proto_tally: proto::TallyResult = tally.clone().into();
        let back: DaoTallyResult = proto_tally.into();
        assert_eq!(tally, back);
    }

    #[test]
    fn dao_roundtrip() {
        let dao = Dao {
            dao_id: 1,
            unique_key: alloc::vec![0xab, 0xcd],
            name: "Test DAO".into(),
            description: "A test DAO".into(),
            dao_address: "morpheum1dao".into(),
            dao_type: DaoType::Hybrid,
            community_token_mint: "morpheum1mint".into(),
            council_token_mint: "morpheum1council".into(),
            config: Some(DaoConfig {
                voting_period: "86400s".into(),
                hold_up_time: "3600s".into(),
                min_deposit_for_proposal: "1000".into(),
                quorum: "0.2".into(),
                approval_threshold: "0.5".into(),
                allow_council_override: true,
                use_conviction_voting: false,
                max_active_proposals: 10,
                plugin_configs: BTreeMap::new(),
            }),
            status: DaoStatus::Active,
            created_at: 1_700_000_000,
            creator: "morpheum1creator".into(),
            governed_assets: alloc::vec![],
            metadata: BTreeMap::new(),
        };

        let proto_dao: proto::Dao = dao.clone().into();
        let back: Dao = proto_dao.into();
        assert_eq!(dao, back);
    }

    #[test]
    fn dao_helpers() {
        let dao = Dao {
            dao_id: 1,
            unique_key: alloc::vec![],
            name: String::new(),
            description: String::new(),
            dao_address: String::new(),
            dao_type: DaoType::Hybrid,
            community_token_mint: String::new(),
            council_token_mint: String::new(),
            config: None,
            status: DaoStatus::Active,
            created_at: 0,
            creator: String::new(),
            governed_assets: alloc::vec![],
            metadata: BTreeMap::new(),
        };

        assert!(dao.is_active());
        assert!(dao.is_hybrid());
    }

    #[test]
    fn dao_deposit_roundtrip() {
        let deposit = DaoDeposit {
            dao_id: 1,
            depositor: "morpheum1user".into(),
            token_mint: "morpheum1mint".into(),
            amount: "5000000".into(),
            lock_until: 1_700_100_000,
        };

        let proto_d: proto::DaoDeposit = deposit.clone().into();
        let back: DaoDeposit = proto_d.into();
        assert_eq!(deposit, back);
    }
}
