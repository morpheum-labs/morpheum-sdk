//! Domain types for the Upgrade module.
//!
//! Clean, idiomatic Rust representations of the upgrade protobuf messages.
//! Full round-trip conversion to/from protobuf, strictly `no_std` compatible.

use alloc::{collections::BTreeMap, string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::upgrade::v1 as proto;

// ====================== ENUMS ======================

/// Status of an upgrade through its lifecycle.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UpgradeStatus {
    Unspecified,
    Scheduled,
    ShadowMode,
    Ready,
    Activated,
    Cancelled,
    Failed,
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
            proto::UpgradeStatus::Failed => Self::Failed,
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
            UpgradeStatus::Failed => proto::UpgradeStatus::Failed as i32,
        }
    }
}

/// Type of upgrade (determines activation path).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UpgradeType {
    Unspecified,
    Parameter,
    HotFeature,
    Binary,
    Emergency,
}

impl From<i32> for UpgradeType {
    fn from(v: i32) -> Self {
        match proto::UpgradeType::try_from(v).unwrap_or(proto::UpgradeType::Unspecified) {
            proto::UpgradeType::Unspecified => Self::Unspecified,
            proto::UpgradeType::Parameter => Self::Parameter,
            proto::UpgradeType::HotFeature => Self::HotFeature,
            proto::UpgradeType::Binary => Self::Binary,
            proto::UpgradeType::Emergency => Self::Emergency,
        }
    }
}

impl From<UpgradeType> for i32 {
    fn from(t: UpgradeType) -> Self {
        match t {
            UpgradeType::Unspecified => proto::UpgradeType::Unspecified as i32,
            UpgradeType::Parameter => proto::UpgradeType::Parameter as i32,
            UpgradeType::HotFeature => proto::UpgradeType::HotFeature as i32,
            UpgradeType::Binary => proto::UpgradeType::Binary as i32,
            UpgradeType::Emergency => proto::UpgradeType::Emergency as i32,
        }
    }
}

// ====================== STRUCT TYPES ======================

/// An upgrade plan specifying what, when, and how to upgrade.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpgradePlan {
    pub proposal_id: u64,
    pub name: String,
    pub info: String,
    pub activation_staple_id: u64,
    pub activation_time: u64,
    pub binary_hash: Vec<u8>,
    pub grace_period_seconds: u64,
    pub upgrade_type: UpgradeType,
    pub additional_metadata: BTreeMap<String, String>,
}

impl From<proto::UpgradePlan> for UpgradePlan {
    fn from(p: proto::UpgradePlan) -> Self {
        Self {
            proposal_id: p.proposal_id,
            name: p.name,
            info: p.info,
            activation_staple_id: p.activation_staple_id,
            activation_time: timestamp_seconds(p.activation_time),
            binary_hash: p.binary_hash,
            grace_period_seconds: p.grace_period_seconds,
            upgrade_type: UpgradeType::from(p.upgrade_type),
            additional_metadata: p.additional_metadata.into_iter().collect(),
        }
    }
}

impl From<UpgradePlan> for proto::UpgradePlan {
    fn from(p: UpgradePlan) -> Self {
        Self {
            proposal_id: p.proposal_id,
            name: p.name,
            info: p.info,
            activation_staple_id: p.activation_staple_id,
            activation_time: to_timestamp(p.activation_time),
            binary_hash: p.binary_hash,
            grace_period_seconds: p.grace_period_seconds,
            upgrade_type: i32::from(p.upgrade_type),
            additional_metadata: p.additional_metadata.into_iter().collect(),
        }
    }
}

/// On-chain state of a single upgrade.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Upgrade {
    pub upgrade_id: u64,
    pub plan: Option<UpgradePlan>,
    pub status: UpgradeStatus,
    pub scheduled_at: u64,
    pub shadow_mode_entered_at: u64,
    pub ready_validator_count: u64,
    pub activated_at: u64,
    pub activated_staple_hash: String,
    pub cancellation_reason: String,
    pub timestamp: u64,
}

impl Upgrade {
    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            UpgradeStatus::Scheduled | UpgradeStatus::ShadowMode | UpgradeStatus::Ready
        )
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            UpgradeStatus::Activated | UpgradeStatus::Cancelled | UpgradeStatus::Failed
        )
    }
}

impl From<proto::Upgrade> for Upgrade {
    fn from(p: proto::Upgrade) -> Self {
        Self {
            upgrade_id: p.upgrade_id,
            plan: p.plan.map(Into::into),
            status: UpgradeStatus::from(p.status),
            scheduled_at: timestamp_seconds(p.scheduled_at),
            shadow_mode_entered_at: timestamp_seconds(p.shadow_mode_entered_at),
            ready_validator_count: p.ready_validator_count,
            activated_at: timestamp_seconds(p.activated_at),
            activated_staple_hash: p.activated_staple_hash,
            cancellation_reason: p.cancellation_reason,
            timestamp: timestamp_seconds(p.timestamp),
        }
    }
}

impl From<Upgrade> for proto::Upgrade {
    fn from(u: Upgrade) -> Self {
        Self {
            upgrade_id: u.upgrade_id,
            plan: u.plan.map(Into::into),
            status: i32::from(u.status),
            scheduled_at: to_timestamp(u.scheduled_at),
            shadow_mode_entered_at: to_timestamp(u.shadow_mode_entered_at),
            ready_validator_count: u.ready_validator_count,
            activated_at: to_timestamp(u.activated_at),
            activated_staple_hash: u.activated_staple_hash,
            cancellation_reason: u.cancellation_reason,
            timestamp: to_timestamp(u.timestamp),
        }
    }
}

/// Per-validator shadow-mode readiness confirmation.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidatorReadiness {
    pub validator_address: String,
    pub validator_pubkey: Vec<u8>,
    pub upgrade_id: u64,
    pub ready: bool,
    pub signaled_at: u64,
    pub signature: Vec<u8>,
}

impl From<proto::ValidatorReadiness> for ValidatorReadiness {
    fn from(p: proto::ValidatorReadiness) -> Self {
        Self {
            validator_address: p.validator_address,
            validator_pubkey: p.validator_pubkey,
            upgrade_id: p.upgrade_id,
            ready: p.ready,
            signaled_at: timestamp_seconds(p.signaled_at),
            signature: p.signature,
        }
    }
}

impl From<ValidatorReadiness> for proto::ValidatorReadiness {
    fn from(v: ValidatorReadiness) -> Self {
        Self {
            validator_address: v.validator_address,
            validator_pubkey: v.validator_pubkey,
            upgrade_id: v.upgrade_id,
            ready: v.ready,
            signaled_at: to_timestamp(v.signaled_at),
            signature: v.signature,
        }
    }
}

/// CAN gossip signal for upgrade readiness events.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpgradeSignal {
    pub upgrade_id: u64,
    pub validator_address: String,
    pub status: UpgradeStatus,
    pub binary_hash: Vec<u8>,
    pub timestamp: u64,
}

impl From<proto::UpgradeSignal> for UpgradeSignal {
    fn from(p: proto::UpgradeSignal) -> Self {
        Self {
            upgrade_id: p.upgrade_id,
            validator_address: p.validator_address,
            status: UpgradeStatus::from(p.status),
            binary_hash: p.binary_hash,
            timestamp: timestamp_seconds(p.timestamp),
        }
    }
}

/// Upgrade lifecycle event (for streams).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpgradeUpdate {
    pub upgrade_id: u64,
    pub old_status: UpgradeStatus,
    pub new_status: UpgradeStatus,
    pub reason: String,
    pub ready_count: u64,
    pub timestamp: u64,
}

impl From<proto::UpgradeUpdate> for UpgradeUpdate {
    fn from(p: proto::UpgradeUpdate) -> Self {
        Self {
            upgrade_id: p.upgrade_id,
            old_status: UpgradeStatus::from(p.old_status),
            new_status: UpgradeStatus::from(p.new_status),
            reason: p.reason,
            ready_count: p.ready_count,
            timestamp: timestamp_seconds(p.timestamp),
        }
    }
}

/// Upgrade status summary (returned by fast-path status query).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpgradeStatusSummary {
    pub upgrade_id: u64,
    pub status: UpgradeStatus,
    pub activation_staple_id: u64,
    pub estimated_activation_time: u64,
    pub ready_validator_count: u64,
    pub zero_downtime_guaranteed: bool,
}

/// Validator readiness overview for an upgrade.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValidatorReadinessOverview {
    pub upgrade_id: u64,
    pub total_ready_count: u64,
    pub required_threshold: u64,
    pub readiness_list: Vec<ValidatorReadiness>,
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
    fn upgrade_status_roundtrip() {
        for v in 0..=6i32 {
            let status = UpgradeStatus::from(v);
            let back: i32 = status.into();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn upgrade_type_roundtrip() {
        for v in 0..=4i32 {
            let t = UpgradeType::from(v);
            let back: i32 = t.into();
            assert_eq!(v, back);
        }
    }

    #[test]
    fn upgrade_plan_roundtrip() {
        let plan = UpgradePlan {
            proposal_id: 42,
            name: "v2.1.0-morpheum".into(),
            info: "ipfs://QmTest".into(),
            activation_staple_id: 0,
            activation_time: 1_700_000_000,
            binary_hash: alloc::vec![0xde, 0xad, 0xbe, 0xef],
            grace_period_seconds: 3600,
            upgrade_type: UpgradeType::Binary,
            additional_metadata: BTreeMap::new(),
        };

        let proto_plan: proto::UpgradePlan = plan.clone().into();
        let back: UpgradePlan = proto_plan.into();
        assert_eq!(plan, back);
    }

    #[test]
    fn upgrade_lifecycle_helpers() {
        let mut upgrade = Upgrade {
            upgrade_id: 1,
            plan: None,
            status: UpgradeStatus::Scheduled,
            scheduled_at: 1_700_000_000,
            shadow_mode_entered_at: 0,
            ready_validator_count: 0,
            activated_at: 0,
            activated_staple_hash: String::new(),
            cancellation_reason: String::new(),
            timestamp: 1_700_000_000,
        };

        assert!(upgrade.is_active());
        assert!(!upgrade.is_terminal());

        upgrade.status = UpgradeStatus::ShadowMode;
        assert!(upgrade.is_active());

        upgrade.status = UpgradeStatus::Activated;
        assert!(!upgrade.is_active());
        assert!(upgrade.is_terminal());

        upgrade.status = UpgradeStatus::Cancelled;
        assert!(upgrade.is_terminal());

        upgrade.status = UpgradeStatus::Failed;
        assert!(upgrade.is_terminal());
    }

    #[test]
    fn upgrade_roundtrip() {
        let upgrade = Upgrade {
            upgrade_id: 7,
            plan: Some(UpgradePlan {
                proposal_id: 7,
                name: "v3.0.0".into(),
                info: String::new(),
                activation_staple_id: 100,
                activation_time: 0,
                binary_hash: alloc::vec![1, 2, 3],
                grace_period_seconds: 600,
                upgrade_type: UpgradeType::Emergency,
                additional_metadata: BTreeMap::new(),
            }),
            status: UpgradeStatus::ShadowMode,
            scheduled_at: 1_700_000_000,
            shadow_mode_entered_at: 1_700_001_000,
            ready_validator_count: 5,
            activated_at: 0,
            activated_staple_hash: String::new(),
            cancellation_reason: String::new(),
            timestamp: 1_700_001_000,
        };

        let proto_upgrade: proto::Upgrade = upgrade.clone().into();
        let back: Upgrade = proto_upgrade.into();
        assert_eq!(upgrade, back);
    }

    #[test]
    fn validator_readiness_roundtrip() {
        let readiness = ValidatorReadiness {
            validator_address: "morpheum1val".into(),
            validator_pubkey: alloc::vec![0xaa, 0xbb],
            upgrade_id: 1,
            ready: true,
            signaled_at: 1_700_000_500,
            signature: alloc::vec![0x01, 0x02, 0x03],
        };

        let proto_r: proto::ValidatorReadiness = readiness.clone().into();
        let back: ValidatorReadiness = proto_r.into();
        assert_eq!(readiness, back);
    }
}
