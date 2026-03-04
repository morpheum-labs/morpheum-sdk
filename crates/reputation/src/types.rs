//! Domain types for the Reputation module.
//!
//! These are clean, idiomatic Rust representations of the reputation protobuf
//! messages. They provide type safety, ergonomic APIs, and full round-trip
//! conversion to/from protobuf while remaining strictly `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::reputation::v1 as proto;

// ====================== REPUTATION SCORE ======================

/// Maximum raw reputation score (1 000 000).
pub const MAX_SCORE: u64 = 1_000_000;

/// Current reputation record for an agent.
///
/// Scores range from 0 to [`MAX_SCORE`] (1 000 000). The `milestone_bitflags`
/// and `perk_bitflags` fields are bitmasks tracking permanently achieved
/// milestones and their corresponding perks.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReputationScore {
    /// Agent hash (hex-encoded SHA-256 of the DID).
    pub agent_hash: String,
    /// Current reputation score (0 – 1 000 000).
    pub score: u64,
    /// Block timestamp of the last score update.
    pub last_updated: u64,
    /// Number of penalties in the trailing 30-day window.
    pub penalty_count_30d: u32,
    /// Bitmask of achieved milestones (8 levels + Immortal flag).
    pub milestone_bitflags: u32,
    /// Whether the agent has achieved permanent Immortal status.
    pub is_immortal: bool,
    /// Points gained in the last 24-hour window.
    pub recovery_velocity: u32,
    /// Cumulative permanent perks (bitmask).
    pub perk_bitflags: u32,
    /// Whether non-floor Immortal perks are temporarily suspended.
    pub luxury_perks_throttled: bool,
}

impl ReputationScore {
    /// Returns the score as a percentage (0.0 – 100.0).
    pub fn score_percent(&self) -> f64 {
        (self.score as f64 / MAX_SCORE as f64) * 100.0
    }

    /// Returns `true` if the given milestone level (0-indexed) has been achieved.
    pub fn has_milestone(&self, level: u32) -> bool {
        self.milestone_bitflags & (1 << level) != 0
    }

    /// Returns `true` if the given perk (0-indexed bit) is active.
    pub fn has_perk(&self, perk_bit: u32) -> bool {
        self.perk_bitflags & (1 << perk_bit) != 0
    }
}

impl From<proto::ReputationScore> for ReputationScore {
    fn from(p: proto::ReputationScore) -> Self {
        Self {
            agent_hash: p.agent_hash,
            score: p.score,
            last_updated: p.last_updated,
            penalty_count_30d: p.penalty_count_30d,
            milestone_bitflags: p.milestone_bitflags,
            is_immortal: p.is_immortal,
            recovery_velocity: p.recovery_velocity,
            perk_bitflags: p.perk_bitflags,
            luxury_perks_throttled: p.luxury_perks_throttled,
        }
    }
}

impl From<ReputationScore> for proto::ReputationScore {
    fn from(r: ReputationScore) -> Self {
        Self {
            agent_hash: r.agent_hash,
            score: r.score,
            last_updated: r.last_updated,
            penalty_count_30d: r.penalty_count_30d,
            milestone_bitflags: r.milestone_bitflags,
            is_immortal: r.is_immortal,
            recovery_velocity: r.recovery_velocity,
            perk_bitflags: r.perk_bitflags,
            luxury_perks_throttled: r.luxury_perks_throttled,
        }
    }
}

// ====================== REPUTATION EVENT ======================

/// A single reputation change event.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReputationEvent {
    /// Agent hash.
    pub agent_hash: String,
    /// Type of event.
    pub event_type: ReputationEventType,
    /// Signed change in score (positive for recovery, negative for penalty).
    pub delta: i64,
    /// Human-readable reason.
    pub reason: String,
    /// Score after this event was applied.
    pub new_score: u64,
    /// Block timestamp.
    pub timestamp: u64,
}

impl From<proto::ReputationEvent> for ReputationEvent {
    fn from(p: proto::ReputationEvent) -> Self {
        Self {
            agent_hash: p.agent_hash,
            event_type: ReputationEventType::from_proto(p.event_type),
            delta: p.delta,
            reason: p.reason,
            new_score: p.new_score,
            timestamp: p.timestamp,
        }
    }
}

impl From<ReputationEvent> for proto::ReputationEvent {
    fn from(e: ReputationEvent) -> Self {
        Self {
            agent_hash: e.agent_hash,
            event_type: e.event_type.to_proto(),
            delta: e.delta,
            reason: e.reason,
            new_score: e.new_score,
            timestamp: e.timestamp,
        }
    }
}

// ====================== REPUTATION EVENT TYPE ======================

/// Type of reputation event.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum ReputationEventType {
    /// A penalty was applied.
    #[default]
    Penalty = 0,
    /// A recovery / positive boost was applied.
    Recovery = 1,
    /// A milestone threshold was reached.
    MilestoneReached = 2,
    /// Immortal floor protection was triggered.
    ImmortalFloorProtected = 3,
    /// A milestone perk was activated.
    PerkActivated = 4,
}

impl ReputationEventType {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Recovery,
            2 => Self::MilestoneReached,
            3 => Self::ImmortalFloorProtected,
            4 => Self::PerkActivated,
            _ => Self::Penalty,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for ReputationEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Penalty => f.write_str("PENALTY"),
            Self::Recovery => f.write_str("RECOVERY"),
            Self::MilestoneReached => f.write_str("MILESTONE_REACHED"),
            Self::ImmortalFloorProtected => f.write_str("IMMORTAL_FLOOR_PROTECTED"),
            Self::PerkActivated => f.write_str("PERK_ACTIVATED"),
        }
    }
}

// ====================== RECOVERY ACTION TYPE ======================

/// Type of recovery action that grants reputation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum RecoveryActionType {
    /// A trade was successfully filled.
    #[default]
    TradeFill = 0,
    /// A valid cryptographic proof was submitted.
    ValidProof = 1,
    /// Agent maintained 24h uptime.
    Uptime24h = 2,
    /// Agent completed a marketplace sale.
    MarketplaceSale = 3,
    /// Milestone reward.
    Milestone = 4,
}

impl RecoveryActionType {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::ValidProof,
            2 => Self::Uptime24h,
            3 => Self::MarketplaceSale,
            4 => Self::Milestone,
            _ => Self::TradeFill,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for RecoveryActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TradeFill => f.write_str("TRADE_FILL"),
            Self::ValidProof => f.write_str("VALID_PROOF"),
            Self::Uptime24h => f.write_str("UPTIME_24H"),
            Self::MarketplaceSale => f.write_str("MARKETPLACE_SALE"),
            Self::Milestone => f.write_str("MILESTONE"),
        }
    }
}

// ====================== MILESTONE STATUS ======================

/// Snapshot of an agent's milestone and perk state (returned by queries).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MilestoneStatus {
    /// Current highest milestone level achieved.
    pub current_milestone_level: u32,
    /// Whether the agent has achieved Immortal status.
    pub is_immortal: bool,
    /// Bitmask of achieved milestones.
    pub milestone_bitflags: u32,
    /// Cumulative permanent perks.
    pub perk_bitflags: u32,
    /// Whether non-floor Immortal perks are temporarily suspended.
    pub luxury_perks_throttled: bool,
}

impl From<proto::QueryMilestoneStatusResponse> for MilestoneStatus {
    fn from(p: proto::QueryMilestoneStatusResponse) -> Self {
        Self {
            current_milestone_level: p.current_milestone_level,
            is_immortal: p.is_immortal,
            milestone_bitflags: p.milestone_bitflags,
            perk_bitflags: p.perk_bitflags,
            luxury_perks_throttled: p.luxury_perks_throttled,
        }
    }
}

// ====================== PARAMS ======================

/// Module parameters (governance-controlled).
///
/// Provides sensible defaults:
/// - `daily_recovery_cap_bps`: 3000 (30%)
/// - `min_reputation_to_register`: 0
/// - `enable_reputation_priority`: true
/// - `slashing_multiplier`: 100 (1.0×)
/// - `milestone_thresholds`: standard 8-level progression
/// - `perk_multiplier_bps`: 1500 (+15%)
///
/// Override only the fields you need:
/// ```rust,ignore
/// let params = Params {
///     slashing_multiplier: 200,  // 2.0× for severe violations
///     ..Default::default()
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    /// Base daily recovery cap as basis points of current score (e.g. 3000 = 30%).
    pub daily_recovery_cap_bps: u32,
    /// Minimum reputation required to register an agent.
    pub min_reputation_to_register: u64,
    /// Whether reputation-based priority in CLOB is enabled.
    pub enable_reputation_priority: bool,
    /// Base slashing multiplier for severe violations (100 = 1.0×).
    pub slashing_multiplier: u32,
    /// Score thresholds for each milestone level.
    pub milestone_thresholds: Vec<u64>,
    /// One-time boost rewards for each milestone level.
    pub milestone_rewards: Vec<u64>,
    /// Base for dynamic perk formulas in basis points (1500 = +15%).
    pub perk_multiplier_bps: u32,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            daily_recovery_cap_bps: 3000,
            min_reputation_to_register: 0,
            enable_reputation_priority: true,
            slashing_multiplier: 100,
            milestone_thresholds: alloc::vec![
                10_000, 50_000, 100_000, 250_000, 500_000, 750_000, 900_000, 1_000_000,
            ],
            milestone_rewards: alloc::vec![500, 1_000, 2_000, 5_000, 10_000, 20_000, 50_000, 100_000],
            perk_multiplier_bps: 1500,
        }
    }
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            daily_recovery_cap_bps: p.daily_recovery_cap_bps,
            min_reputation_to_register: p.min_reputation_to_register,
            enable_reputation_priority: p.enable_reputation_priority,
            slashing_multiplier: p.slashing_multiplier,
            milestone_thresholds: p.milestone_thresholds,
            milestone_rewards: p.milestone_rewards,
            perk_multiplier_bps: p.perk_multiplier_bps,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            daily_recovery_cap_bps: p.daily_recovery_cap_bps,
            min_reputation_to_register: p.min_reputation_to_register,
            enable_reputation_priority: p.enable_reputation_priority,
            slashing_multiplier: p.slashing_multiplier,
            milestone_thresholds: p.milestone_thresholds,
            milestone_rewards: p.milestone_rewards,
            perk_multiplier_bps: p.perk_multiplier_bps,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reputation_score_roundtrip() {
        let score = ReputationScore {
            agent_hash: "abc123".into(),
            score: 750_000,
            last_updated: 1_700_000_000,
            penalty_count_30d: 2,
            milestone_bitflags: 0b0000_0111,
            is_immortal: false,
            recovery_velocity: 500,
            perk_bitflags: 0b0000_0011,
            luxury_perks_throttled: false,
        };

        let proto: proto::ReputationScore = score.clone().into();
        let back: ReputationScore = proto.into();
        assert_eq!(score, back);
    }

    #[test]
    fn reputation_score_helpers() {
        let score = ReputationScore {
            score: 500_000,
            milestone_bitflags: 0b0000_0101,
            perk_bitflags: 0b0000_0010,
            ..Default::default()
        };

        assert!((score.score_percent() - 50.0).abs() < f64::EPSILON);
        assert!(score.has_milestone(0));
        assert!(!score.has_milestone(1));
        assert!(score.has_milestone(2));
        assert!(!score.has_perk(0));
        assert!(score.has_perk(1));
    }

    #[test]
    fn event_type_roundtrip() {
        for et in [
            ReputationEventType::Penalty,
            ReputationEventType::Recovery,
            ReputationEventType::MilestoneReached,
            ReputationEventType::ImmortalFloorProtected,
            ReputationEventType::PerkActivated,
        ] {
            assert_eq!(ReputationEventType::from_proto(et.to_proto()), et);
        }
    }

    #[test]
    fn recovery_action_type_roundtrip() {
        for at in [
            RecoveryActionType::TradeFill,
            RecoveryActionType::ValidProof,
            RecoveryActionType::Uptime24h,
            RecoveryActionType::MarketplaceSale,
            RecoveryActionType::Milestone,
        ] {
            assert_eq!(RecoveryActionType::from_proto(at.to_proto()), at);
        }
    }

    #[test]
    fn reputation_event_roundtrip() {
        let event = ReputationEvent {
            agent_hash: "abc".into(),
            event_type: ReputationEventType::Recovery,
            delta: 5000,
            reason: "trade fill".into(),
            new_score: 755_000,
            timestamp: 1_700_000_000,
        };

        let proto: proto::ReputationEvent = event.clone().into();
        let back: ReputationEvent = proto.into();
        assert_eq!(event, back);
    }

    #[test]
    fn params_defaults() {
        let params = Params::default();
        assert_eq!(params.daily_recovery_cap_bps, 3000);
        assert!(params.enable_reputation_priority);
        assert_eq!(params.slashing_multiplier, 100);
        assert_eq!(params.milestone_thresholds.len(), 8);
        assert_eq!(params.perk_multiplier_bps, 1500);
    }

    #[test]
    fn params_roundtrip() {
        let params = Params {
            daily_recovery_cap_bps: 5000,
            min_reputation_to_register: 100,
            enable_reputation_priority: false,
            slashing_multiplier: 200,
            milestone_thresholds: alloc::vec![1, 2, 3],
            milestone_rewards: alloc::vec![10, 20, 30],
            perk_multiplier_bps: 2000,
        };
        let proto: proto::Params = params.clone().into();
        let back: Params = proto.into();
        assert_eq!(params, back);
    }

    #[test]
    fn milestone_status_from_proto() {
        let proto_res = proto::QueryMilestoneStatusResponse {
            current_milestone_level: 5,
            is_immortal: true,
            milestone_bitflags: 0b0001_1111,
            perk_bitflags: 0b0000_1111,
            luxury_perks_throttled: false,
        };
        let status: MilestoneStatus = proto_res.into();
        assert_eq!(status.current_milestone_level, 5);
        assert!(status.is_immortal);
    }
}
