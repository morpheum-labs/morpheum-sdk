//! Domain types for the vesting module.
//!
//! Covers schedule types, categories, vesting entries, summaries,
//! governance parameters, and streaming event types.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::vesting::v1 as proto;

// ====================== ENUMS ======================

/// Release curve type for a vesting schedule.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ScheduleType {
    #[default]
    Unspecified,
    Linear,
    CliffLinear,
    Step,
}

impl From<i32> for ScheduleType {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Linear, 2 => Self::CliffLinear, 3 => Self::Step,
            _ => Self::Unspecified,
        }
    }
}

impl From<ScheduleType> for i32 {
    fn from(s: ScheduleType) -> Self {
        match s {
            ScheduleType::Unspecified => 0, ScheduleType::Linear => 1,
            ScheduleType::CliffLinear => 2, ScheduleType::Step => 3,
        }
    }
}

/// Classification for tokenomics transparency and reporting.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VestingCategory {
    #[default]
    Unspecified,
    Team,
    CoreContributors,
    Investors,
    Advisors,
    CommunityRewards,
    Foundation,
    Ecosystem,
    Treasury,
    Airdrop,
    Partnership,
}

impl From<i32> for VestingCategory {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Team,             2 => Self::CoreContributors,
            3 => Self::Investors,        4 => Self::Advisors,
            5 => Self::CommunityRewards, 6 => Self::Foundation,
            7 => Self::Ecosystem,        8 => Self::Treasury,
            9 => Self::Airdrop,          10 => Self::Partnership,
            _ => Self::Unspecified,
        }
    }
}

impl From<VestingCategory> for i32 {
    fn from(c: VestingCategory) -> Self {
        match c {
            VestingCategory::Unspecified => 0,      VestingCategory::Team => 1,
            VestingCategory::CoreContributors => 2, VestingCategory::Investors => 3,
            VestingCategory::Advisors => 4,         VestingCategory::CommunityRewards => 5,
            VestingCategory::Foundation => 6,       VestingCategory::Ecosystem => 7,
            VestingCategory::Treasury => 8,         VestingCategory::Airdrop => 9,
            VestingCategory::Partnership => 10,
        }
    }
}

// ====================== DOMAIN TYPES ======================

/// Core on-chain vesting record.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VestingEntry {
    pub id: u64,
    pub beneficiary: String,
    pub total_amount: String,
    pub released: String,
    pub start_timestamp: u64,
    pub cliff_duration: u64,
    pub vesting_duration: u64,
    pub schedule_type: ScheduleType,
    pub revocable: bool,
    pub category: VestingCategory,
    pub step_timestamps: Vec<u64>,
    pub step_amounts: Vec<String>,
}

impl From<proto::VestingEntry> for VestingEntry {
    fn from(p: proto::VestingEntry) -> Self {
        Self {
            id: p.id, beneficiary: p.beneficiary,
            total_amount: p.total_amount, released: p.released,
            start_timestamp: p.start_timestamp, cliff_duration: p.cliff_duration,
            vesting_duration: p.vesting_duration,
            schedule_type: ScheduleType::from(p.schedule_type),
            revocable: p.revocable, category: VestingCategory::from(p.category),
            step_timestamps: p.step_timestamps, step_amounts: p.step_amounts,
        }
    }
}

/// Aggregated vesting view for a beneficiary.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VestingSummary {
    pub beneficiary: String,
    pub total_vested: String,
    pub total_released: String,
    pub currently_releasable: String,
    pub total_locked: String,
    pub next_unlock_timestamp: u64,
    pub entry_count: u64,
}

impl From<proto::VestingSummary> for VestingSummary {
    fn from(p: proto::VestingSummary) -> Self {
        Self {
            beneficiary: p.beneficiary, total_vested: p.total_vested,
            total_released: p.total_released, currently_releasable: p.currently_releasable,
            total_locked: p.total_locked, next_unlock_timestamp: p.next_unlock_timestamp,
            entry_count: p.entry_count,
        }
    }
}

/// Governance-tunable vesting parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VestingParams {
    pub max_entries_per_account: u64,
    pub max_cliff_duration: u64,
    pub max_vesting_duration: u64,
    pub min_vesting_duration: u64,
    pub min_vesting_amount: String,
    pub allow_governance_revocation: bool,
    pub default_cliff_duration: u64,
}

impl From<proto::Params> for VestingParams {
    fn from(p: proto::Params) -> Self {
        Self {
            max_entries_per_account: p.max_entries_per_account,
            max_cliff_duration: p.max_cliff_duration,
            max_vesting_duration: p.max_vesting_duration,
            min_vesting_duration: p.min_vesting_duration,
            min_vesting_amount: p.min_vesting_amount,
            allow_governance_revocation: p.allow_governance_revocation,
            default_cliff_duration: p.default_cliff_duration,
        }
    }
}

impl From<VestingParams> for proto::Params {
    fn from(p: VestingParams) -> Self {
        Self {
            max_entries_per_account: p.max_entries_per_account,
            max_cliff_duration: p.max_cliff_duration,
            max_vesting_duration: p.max_vesting_duration,
            min_vesting_duration: p.min_vesting_duration,
            min_vesting_amount: p.min_vesting_amount,
            allow_governance_revocation: p.allow_governance_revocation,
            default_cliff_duration: p.default_cliff_duration,
        }
    }
}

// ====================== STREAM EVENT ======================

/// Vesting lifecycle event (from stream.proto `SubscribeVestingEventsResponse`).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VestingEvent {
    pub event_type: String,
    pub beneficiary: String,
    pub vesting_id: u64,
    pub amount: String,
    pub reason: String,
    pub timestamp: u64,
}

impl From<proto::SubscribeVestingEventsResponse> for VestingEvent {
    fn from(p: proto::SubscribeVestingEventsResponse) -> Self {
        Self {
            event_type: p.event_type, beneficiary: p.beneficiary,
            vesting_id: p.vesting_id, amount: p.amount, reason: p.reason,
            timestamp: p.timestamp.as_ref().map_or(0, |t| t.seconds as u64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn schedule_type_roundtrip() {
        for s in [ScheduleType::Linear, ScheduleType::CliffLinear, ScheduleType::Step] {
            assert_eq!(s, ScheduleType::from(i32::from(s)));
        }
        assert_eq!(ScheduleType::Unspecified, ScheduleType::from(99));
    }

    #[test]
    fn vesting_category_roundtrip() {
        for c in [VestingCategory::Team, VestingCategory::CoreContributors,
                  VestingCategory::Investors, VestingCategory::Advisors,
                  VestingCategory::CommunityRewards, VestingCategory::Foundation,
                  VestingCategory::Ecosystem, VestingCategory::Treasury,
                  VestingCategory::Airdrop, VestingCategory::Partnership] {
            assert_eq!(c, VestingCategory::from(i32::from(c)));
        }
    }

    #[test]
    fn vesting_entry_from_proto() {
        let p = proto::VestingEntry {
            id: 1, beneficiary: "morph1user".into(), total_amount: "1000000".into(),
            released: "0".into(), start_timestamp: 1700000000, cliff_duration: 31536000,
            vesting_duration: 63072000, schedule_type: 2, revocable: true, category: 2,
            step_timestamps: vec![], step_amounts: vec![],
        };
        let e: VestingEntry = p.into();
        assert_eq!(e.schedule_type, ScheduleType::CliffLinear);
        assert_eq!(e.category, VestingCategory::CoreContributors);
        assert!(e.revocable);
    }

    #[test]
    fn params_roundtrip() {
        let p = VestingParams {
            max_entries_per_account: 32, max_cliff_duration: 157680000,
            max_vesting_duration: 315360000, min_vesting_duration: 2592000,
            min_vesting_amount: "100000".into(), allow_governance_revocation: true,
            default_cliff_duration: 31536000,
        };
        let proto_p: proto::Params = p.clone().into();
        let p2: VestingParams = proto_p.into();
        assert_eq!(p, p2);
    }

    #[test]
    fn vesting_summary_from_proto() {
        let p = proto::VestingSummary {
            beneficiary: "morph1user".into(), total_vested: "1000000".into(),
            total_released: "250000".into(), currently_releasable: "50000".into(),
            total_locked: "700000".into(), next_unlock_timestamp: 1700100000,
            entry_count: 3,
        };
        let s: VestingSummary = p.into();
        assert_eq!(s.entry_count, 3);
        assert_eq!(s.currently_releasable, "50000");
    }
}
