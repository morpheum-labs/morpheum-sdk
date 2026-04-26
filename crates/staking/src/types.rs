//! Domain types for the Staking module.
//!
//! Clean, idiomatic Rust representations of the staking protobuf messages.
//! Provides type safety, ergonomic APIs, and full round-trip conversion
//! to/from protobuf while remaining strictly `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::staking::v1 as proto;

// ====================== ENUMS ======================

/// Validator lifecycle state.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ValidatorStatus {
    #[default]
    Unspecified,
    Active,
    Jailed,
    Tombstoned,
}

impl ValidatorStatus {
    pub fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }

    pub fn is_penalized(self) -> bool {
        matches!(self, Self::Jailed | Self::Tombstoned)
    }
}

impl From<i32> for ValidatorStatus {
    fn from(v: i32) -> Self {
        match proto::ValidatorStatus::try_from(v) {
            Ok(proto::ValidatorStatus::Active) => Self::Active,
            Ok(proto::ValidatorStatus::Jailed) => Self::Jailed,
            Ok(proto::ValidatorStatus::Tombstoned) => Self::Tombstoned,
            _ => Self::Unspecified,
        }
    }
}

impl From<ValidatorStatus> for i32 {
    fn from(s: ValidatorStatus) -> Self {
        match s {
            ValidatorStatus::Unspecified => proto::ValidatorStatus::Unspecified as i32,
            ValidatorStatus::Active => proto::ValidatorStatus::Active as i32,
            ValidatorStatus::Jailed => proto::ValidatorStatus::Jailed as i32,
            ValidatorStatus::Tombstoned => proto::ValidatorStatus::Tombstoned as i32,
        }
    }
}

/// Type of validator misbehavior.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MisbehaviorType {
    #[default]
    Unspecified,
    Downtime,
    DoubleVote,
    Race,
}

impl From<i32> for MisbehaviorType {
    fn from(v: i32) -> Self {
        match proto::MisbehaviorType::try_from(v) {
            Ok(proto::MisbehaviorType::Downtime) => Self::Downtime,
            Ok(proto::MisbehaviorType::DoubleVote) => Self::DoubleVote,
            Ok(proto::MisbehaviorType::Race) => Self::Race,
            _ => Self::Unspecified,
        }
    }
}

impl From<MisbehaviorType> for i32 {
    fn from(m: MisbehaviorType) -> Self {
        match m {
            MisbehaviorType::Unspecified => proto::MisbehaviorType::Unspecified as i32,
            MisbehaviorType::Downtime => proto::MisbehaviorType::Downtime as i32,
            MisbehaviorType::DoubleVote => proto::MisbehaviorType::DoubleVote as i32,
            MisbehaviorType::Race => proto::MisbehaviorType::Race as i32,
        }
    }
}

// ====================== DOMAIN STRUCTS ======================

/// Network validator (supports solo and DVT clusters).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Validator {
    pub validator_id: String,
    pub moniker: String,
    pub description: String,
    pub asset_index: u64,
    pub self_stake: String,
    pub delegated_stake: String,
    pub commission_rate: String,
    pub status: ValidatorStatus,
    pub operator_address: String,
    pub consensus_pubkey: Vec<u8>,
    /// 32-byte ECVRF (Ristretto255) public key. Mandatory per active
    /// validator; the leader-proof verifier rejects proposals from a
    /// validator whose `vrf_pubkey` is missing or malformed.
    pub vrf_pubkey: Vec<u8>,
    pub dvt_cluster_id: Option<String>,
    pub dvt_threshold_t: Option<u32>,
    pub dvt_threshold_n: Option<u32>,
    /// Rewards and unstaked funds; chain default is operator address when unset on-chain.
    pub withdrawal_address: Option<String>,
}

impl Validator {
    pub fn is_dvt_cluster(&self) -> bool {
        self.dvt_cluster_id.is_some()
    }
}

impl From<proto::Validator> for Validator {
    fn from(p: proto::Validator) -> Self {
        Self {
            validator_id: p.validator_id,
            moniker: p.moniker,
            description: p.description,
            asset_index: p.asset_index,
            self_stake: p.self_stake,
            delegated_stake: p.delegated_stake,
            commission_rate: p.commission_rate,
            status: ValidatorStatus::from(p.status),
            operator_address: p.operator_address,
            consensus_pubkey: p.consensus_pubkey,
            vrf_pubkey: p.vrf_pubkey,
            dvt_cluster_id: p.dvt_cluster_id,
            dvt_threshold_t: p.dvt_threshold_t,
            dvt_threshold_n: p.dvt_threshold_n,
            withdrawal_address: p.withdrawal_address,
        }
    }
}

impl From<Validator> for proto::Validator {
    fn from(v: Validator) -> Self {
        Self {
            validator_id: v.validator_id,
            moniker: v.moniker,
            description: v.description,
            asset_index: v.asset_index,
            self_stake: v.self_stake,
            delegated_stake: v.delegated_stake,
            commission_rate: v.commission_rate,
            status: i32::from(v.status),
            last_active: None,
            operator_address: v.operator_address,
            operator_external_address: None,
            operator_chain_type: None,
            consensus_pubkey: v.consensus_pubkey,
            vrf_pubkey: v.vrf_pubkey,
            dvt_cluster_id: v.dvt_cluster_id,
            dvt_threshold_t: v.dvt_threshold_t,
            dvt_threshold_n: v.dvt_threshold_n,
            withdrawal_address: v.withdrawal_address,
        }
    }
}

/// Delegation from a user to a validator.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Delegation {
    pub delegator_address: String,
    pub validator_id: String,
    pub asset_index: u64,
    pub amount: String,
}

impl From<proto::Delegation> for Delegation {
    fn from(p: proto::Delegation) -> Self {
        Self {
            delegator_address: p.delegator_address,
            validator_id: p.validator_id,
            asset_index: p.asset_index,
            amount: p.amount,
        }
    }
}

/// Unbonding delegation entry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UnbondingDelegation {
    pub delegator_address: String,
    pub validator_id: String,
    pub asset_index: u64,
    pub amount: String,
}

impl From<proto::UnbondingDelegation> for UnbondingDelegation {
    fn from(p: proto::UnbondingDelegation) -> Self {
        Self {
            delegator_address: p.delegator_address,
            validator_id: p.validator_id,
            asset_index: p.asset_index,
            amount: p.amount,
        }
    }
}

/// Accrued reward entry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Reward {
    pub recipient_address: String,
    pub validator_id: String,
    pub asset_index: u64,
    pub amount: String,
}

impl From<proto::Reward> for Reward {
    fn from(p: proto::Reward) -> Self {
        Self {
            recipient_address: p.recipient_address,
            validator_id: p.validator_id,
            asset_index: p.asset_index,
            amount: p.amount,
        }
    }
}

/// User staking overview (aggregated from multiple delegations, unbondings, rewards).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UserStaking {
    pub delegations: Vec<Delegation>,
    pub unbonding_delegations: Vec<UnbondingDelegation>,
    pub rewards: Vec<Reward>,
    pub total_staked: String,
    pub total_rewards: String,
}

/// Validator stake breakdown.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidatorStake {
    pub self_stake: String,
    pub delegated: String,
}

/// Slashing penalty record.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Penalty {
    pub penalty_id: String,
    pub validator_id: String,
    pub penalty_type: MisbehaviorType,
    pub asset_index: u64,
    pub amount: String,
    pub reason: String,
}

impl From<proto::Penalty> for Penalty {
    fn from(p: proto::Penalty) -> Self {
        Self {
            penalty_id: p.penalty_id,
            validator_id: p.validator_id,
            penalty_type: MisbehaviorType::from(p.penalty_type),
            asset_index: p.asset_index,
            amount: p.amount,
            reason: p.reason,
        }
    }
}

/// Slashing event record.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SlashingEvent {
    pub id: String,
    pub validator_id: String,
    pub slash_type: MisbehaviorType,
    pub asset_index: u64,
    pub amount: String,
    pub reason: String,
    pub height: u64,
    pub processed: bool,
}

impl From<proto::SlashingEvent> for SlashingEvent {
    fn from(p: proto::SlashingEvent) -> Self {
        Self {
            id: p.id,
            validator_id: p.validator_id,
            slash_type: MisbehaviorType::from(p.slash_type),
            asset_index: p.asset_index,
            amount: p.amount,
            reason: p.reason,
            height: p.height,
            processed: p.processed,
        }
    }
}

/// Module parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StakingParams {
    pub asset_index: u64,
    pub unbonding_period_seconds: i64,
    pub redelegation_cooldown_seconds: i64,
    pub max_validators: String,
    pub slash_fraction_downtime: String,
    pub slash_fraction_double_sign: String,
    pub jail_duration_seconds: i64,
    pub min_self_delegation: String,
    pub scoring_params: Option<ScoringParams>,
    pub liveness_params: Option<LivenessParams>,
    pub inflation_rate_bps: String,
    pub max_commission_bps: String,
}

impl Default for StakingParams {
    fn default() -> Self {
        Self {
            asset_index: 0,
            unbonding_period_seconds: 1_209_600, // 14 days
            redelegation_cooldown_seconds: 604_800, // 7 days
            max_validators: "100".into(),
            slash_fraction_downtime: "0.01".into(),
            slash_fraction_double_sign: "0.05".into(),
            jail_duration_seconds: 600,
            min_self_delegation: "1000000".into(),
            scoring_params: Some(ScoringParams::default()),
            liveness_params: Some(LivenessParams::default()),
            inflation_rate_bps: "0".into(),
            max_commission_bps: "2000".into(),
        }
    }
}

impl From<proto::Params> for StakingParams {
    fn from(p: proto::Params) -> Self {
        Self {
            asset_index: p.asset_index,
            unbonding_period_seconds: p.unbonding_period.map_or(0, |d| d.seconds),
            redelegation_cooldown_seconds: p.redelegation_cooldown.map_or(0, |d| d.seconds),
            max_validators: p.max_validators,
            slash_fraction_downtime: p.slash_fraction_downtime,
            slash_fraction_double_sign: p.slash_fraction_double_sign,
            jail_duration_seconds: p.jail_duration.map_or(0, |d| d.seconds),
            min_self_delegation: p.min_self_delegation,
            scoring_params: p.scoring_params.map(Into::into),
            liveness_params: p.liveness_params.map(Into::into),
            inflation_rate_bps: p.inflation_rate_bps,
            max_commission_bps: p.max_commission_bps,
        }
    }
}

impl From<StakingParams> for proto::Params {
    fn from(p: StakingParams) -> Self {
        use morpheum_proto::google::protobuf::Duration;
        Self {
            asset_index: p.asset_index,
            unbonding_period: Some(Duration { seconds: p.unbonding_period_seconds, nanos: 0 }),
            redelegation_cooldown: Some(Duration { seconds: p.redelegation_cooldown_seconds, nanos: 0 }),
            max_validators: p.max_validators,
            slash_fraction_downtime: p.slash_fraction_downtime,
            slash_fraction_double_sign: p.slash_fraction_double_sign,
            jail_duration: Some(Duration { seconds: p.jail_duration_seconds, nanos: 0 }),
            min_self_delegation: p.min_self_delegation,
            scoring_params: p.scoring_params.map(Into::into),
            liveness_params: p.liveness_params.map(Into::into),
            inflation_rate_bps: p.inflation_rate_bps,
            max_commission_bps: p.max_commission_bps,
        }
    }
}

// ====================== VALIDATOR ECONOMICS TYPES ======================

/// Scoring parameters for validator ranking.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ScoringParams {
    pub stake_weight_bps: u32,
    pub mana_weight_bps: u32,
    pub reputation_weight_bps: u32,
}

impl From<proto::ScoringParams> for ScoringParams {
    fn from(p: proto::ScoringParams) -> Self {
        Self {
            stake_weight_bps: p.stake_weight_bps,
            mana_weight_bps: p.mana_weight_bps,
            reputation_weight_bps: p.reputation_weight_bps,
        }
    }
}

impl From<ScoringParams> for proto::ScoringParams {
    fn from(s: ScoringParams) -> Self {
        Self {
            stake_weight_bps: s.stake_weight_bps,
            mana_weight_bps: s.mana_weight_bps,
            reputation_weight_bps: s.reputation_weight_bps,
        }
    }
}

/// Liveness enforcement parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LivenessParams {
    pub max_missed_epochs: u64,
    pub downtime_slash_bps: u32,
    pub jail_after_misses: u64,
}

impl From<proto::LivenessParams> for LivenessParams {
    fn from(p: proto::LivenessParams) -> Self {
        Self {
            max_missed_epochs: p.max_missed_epochs,
            downtime_slash_bps: p.downtime_slash_bps,
            jail_after_misses: p.jail_after_misses,
        }
    }
}

impl From<LivenessParams> for proto::LivenessParams {
    fn from(l: LivenessParams) -> Self {
        Self {
            max_missed_epochs: l.max_missed_epochs,
            downtime_slash_bps: l.downtime_slash_bps,
            jail_after_misses: l.jail_after_misses,
        }
    }
}

/// Epoch reward distribution snapshot.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EpochRewardSnapshot {
    pub epoch: u64,
    pub total_distributed: String,
    pub storage_fund_contribution: String,
    pub inflation_contribution: String,
    pub validators_rewarded: u32,
    pub delegators_rewarded: u32,
}

impl From<proto::EpochRewardSnapshot> for EpochRewardSnapshot {
    fn from(p: proto::EpochRewardSnapshot) -> Self {
        Self {
            epoch: p.epoch,
            total_distributed: p.total_distributed,
            storage_fund_contribution: p.storage_fund_contribution,
            inflation_contribution: p.inflation_contribution,
            validators_rewarded: p.validators_rewarded,
            delegators_rewarded: p.delegators_rewarded,
        }
    }
}

impl From<EpochRewardSnapshot> for proto::EpochRewardSnapshot {
    fn from(s: EpochRewardSnapshot) -> Self {
        Self {
            epoch: s.epoch,
            total_distributed: s.total_distributed,
            storage_fund_contribution: s.storage_fund_contribution,
            inflation_contribution: s.inflation_contribution,
            validators_rewarded: s.validators_rewarded,
            delegators_rewarded: s.delegators_rewarded,
        }
    }
}

/// Computed validator score for ranking.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidatorScore {
    pub validator_id: String,
    pub score: String,
    pub stake_component: String,
    pub mana_component: String,
    pub reputation_component: String,
    pub epoch: u64,
}

impl From<proto::ValidatorScore> for ValidatorScore {
    fn from(p: proto::ValidatorScore) -> Self {
        Self {
            validator_id: p.validator_id,
            score: p.score,
            stake_component: p.stake_component,
            mana_component: p.mana_component,
            reputation_component: p.reputation_component,
            epoch: p.epoch,
        }
    }
}

impl From<ValidatorScore> for proto::ValidatorScore {
    fn from(s: ValidatorScore) -> Self {
        Self {
            validator_id: s.validator_id,
            score: s.score,
            stake_component: s.stake_component,
            mana_component: s.mana_component,
            reputation_component: s.reputation_component,
            epoch: s.epoch,
        }
    }
}

/// Commission tracking for a validator.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CommissionInfo {
    pub validator_id: String,
    pub commission_bps: u32,
    pub total_commission_earned: String,
    pub last_epoch_commission: String,
    pub last_epoch: u64,
}

impl From<proto::CommissionInfo> for CommissionInfo {
    fn from(p: proto::CommissionInfo) -> Self {
        Self {
            validator_id: p.validator_id,
            commission_bps: p.commission_bps,
            total_commission_earned: p.total_commission_earned,
            last_epoch_commission: p.last_epoch_commission,
            last_epoch: p.last_epoch,
        }
    }
}

impl From<CommissionInfo> for proto::CommissionInfo {
    fn from(c: CommissionInfo) -> Self {
        Self {
            validator_id: c.validator_id,
            commission_bps: c.commission_bps,
            total_commission_earned: c.total_commission_earned,
            last_epoch_commission: c.last_epoch_commission,
            last_epoch: c.last_epoch,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validator_status_roundtrip() {
        for s in [
            ValidatorStatus::Unspecified,
            ValidatorStatus::Active,
            ValidatorStatus::Jailed,
            ValidatorStatus::Tombstoned,
        ] {
            let i: i32 = s.into();
            let back = ValidatorStatus::from(i);
            assert_eq!(s, back);
        }
    }

    #[test]
    fn misbehavior_type_roundtrip() {
        for m in [
            MisbehaviorType::Unspecified,
            MisbehaviorType::Downtime,
            MisbehaviorType::DoubleVote,
            MisbehaviorType::Race,
        ] {
            let i: i32 = m.into();
            let back = MisbehaviorType::from(i);
            assert_eq!(m, back);
        }
    }

    #[test]
    fn validator_status_helpers() {
        assert!(ValidatorStatus::Active.is_active());
        assert!(!ValidatorStatus::Jailed.is_active());
        assert!(ValidatorStatus::Jailed.is_penalized());
        assert!(ValidatorStatus::Tombstoned.is_penalized());
        assert!(!ValidatorStatus::Active.is_penalized());
    }

    #[test]
    fn validator_roundtrip() {
        let v = Validator {
            validator_id: "val-1".into(),
            moniker: "Test Validator".into(),
            self_stake: "1000000".into(),
            status: ValidatorStatus::Active,
            withdrawal_address: Some("morm1withdraw".into()),
            ..Default::default()
        };
        let proto_v: proto::Validator = v.clone().into();
        let back = Validator::from(proto_v);
        assert_eq!(v.validator_id, back.validator_id);
        assert_eq!(v.status, back.status);
        assert_eq!(v.withdrawal_address, back.withdrawal_address);
    }

    #[test]
    fn staking_params_roundtrip() {
        let params = StakingParams::default();
        let proto_params: proto::Params = params.clone().into();
        let back = StakingParams::from(proto_params);
        assert_eq!(params, back);
    }

    #[test]
    fn scoring_params_roundtrip() {
        let sp = ScoringParams { stake_weight_bps: 8000, mana_weight_bps: 1500, reputation_weight_bps: 500 };
        let proto_sp: proto::ScoringParams = sp.clone().into();
        let back = ScoringParams::from(proto_sp);
        assert_eq!(sp, back);
    }

    #[test]
    fn liveness_params_roundtrip() {
        let lp = LivenessParams { max_missed_epochs: 5, downtime_slash_bps: 500, jail_after_misses: 10 };
        let proto_lp: proto::LivenessParams = lp.clone().into();
        let back = LivenessParams::from(proto_lp);
        assert_eq!(lp, back);
    }

    #[test]
    fn epoch_reward_snapshot_roundtrip() {
        let snap = EpochRewardSnapshot {
            epoch: 42,
            total_distributed: "1000000".into(),
            storage_fund_contribution: "600000".into(),
            inflation_contribution: "400000".into(),
            validators_rewarded: 10,
            delegators_rewarded: 50,
        };
        let proto_snap: proto::EpochRewardSnapshot = snap.clone().into();
        let back = EpochRewardSnapshot::from(proto_snap);
        assert_eq!(snap, back);
    }

    #[test]
    fn validator_score_roundtrip() {
        let vs = ValidatorScore {
            validator_id: "val-1".into(),
            score: "9500".into(),
            stake_component: "8000".into(),
            mana_component: "1000".into(),
            reputation_component: "500".into(),
            epoch: 42,
        };
        let proto_vs: proto::ValidatorScore = vs.clone().into();
        let back = ValidatorScore::from(proto_vs);
        assert_eq!(vs, back);
    }

    #[test]
    fn commission_info_roundtrip() {
        let ci = CommissionInfo {
            validator_id: "val-1".into(),
            commission_bps: 1000,
            total_commission_earned: "50000".into(),
            last_epoch_commission: "5000".into(),
            last_epoch: 41,
        };
        let proto_ci: proto::CommissionInfo = ci.clone().into();
        let back = CommissionInfo::from(proto_ci);
        assert_eq!(ci, back);
    }
}
