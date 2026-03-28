//! Domain types for the CLAMM Graduation module.
//!
//! Covers the graduation state machine, parameters, checkpoints, and
//! all lifecycle events from initiation through completion or failure.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::clammgrad::v1 as proto;

// ====================== ENUM ======================

/// Graduation lifecycle status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GraduationStatus {
    Unspecified,
    Pending,
    Draining,
    MarketsCreated,
    Completed,
    Failed,
}

impl From<i32> for GraduationStatus {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Pending, 2 => Self::Draining, 3 => Self::MarketsCreated,
            4 => Self::Completed, 5 => Self::Failed, _ => Self::Unspecified,
        }
    }
}

impl From<GraduationStatus> for i32 {
    fn from(s: GraduationStatus) -> Self {
        match s {
            GraduationStatus::Unspecified => 0, GraduationStatus::Pending => 1,
            GraduationStatus::Draining => 2, GraduationStatus::MarketsCreated => 3,
            GraduationStatus::Completed => 4, GraduationStatus::Failed => 5,
        }
    }
}

// ====================== HELPER ======================

fn ts_secs(ts: Option<morpheum_proto::google::protobuf::Timestamp>) -> u64 {
    ts.map(|t| t.seconds as u64).unwrap_or(0)
}

// ====================== STATE ======================

/// A checkpoint in the graduation process.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraduationCheckpoint {
    pub step: u32,
    pub description: String,
    pub at: u64,
}

impl From<proto::GraduationCheckpoint> for GraduationCheckpoint {
    fn from(p: proto::GraduationCheckpoint) -> Self {
        Self { step: p.step, description: p.description, at: ts_secs(p.at) }
    }
}

/// Full graduation state for a token.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraduationState {
    pub token_index: String,
    pub status: GraduationStatus,
    pub spot_market_index: String,
    pub perp_market_index: String,
    pub clamm_pool_id: String,
    pub initiated_at: u64,
    pub completed_at: u64,
    pub failure_reason: String,
    pub checkpoints: Vec<GraduationCheckpoint>,
    pub last_activity: u64,
    pub failed_step: u32,
}

impl From<proto::GraduationState> for GraduationState {
    fn from(p: proto::GraduationState) -> Self {
        Self {
            token_index: p.token_index,
            status: GraduationStatus::from(p.status),
            spot_market_index: p.spot_market_index,
            perp_market_index: p.perp_market_index,
            clamm_pool_id: p.clamm_pool_id,
            initiated_at: p.initiated_at,
            completed_at: p.completed_at,
            failure_reason: p.failure_reason,
            checkpoints: p.checkpoints.into_iter().map(Into::into).collect(),
            last_activity: ts_secs(p.last_activity),
            failed_step: p.failed_step,
        }
    }
}

// ====================== PARAMS ======================

/// Module-level graduation parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClammGraduationParams {
    pub min_mcap_sat: String,
    pub min_tvl_sat: String,
    pub min_volume_30d_sat: String,
    pub min_age_blocks: u64,
    pub incentives_bps: u32,
    pub cooldown_blocks: u64,
    pub authority: String,
    pub protocol_fee_bps: u32,
    pub graduation_timeout_blocks: u64,
}

impl From<proto::ClammGraduationParams> for ClammGraduationParams {
    fn from(p: proto::ClammGraduationParams) -> Self {
        Self {
            min_mcap_sat: p.min_mcap_sat, min_tvl_sat: p.min_tvl_sat,
            min_volume_30d_sat: p.min_volume_30d_sat, min_age_blocks: p.min_age_blocks,
            incentives_bps: p.incentives_bps, cooldown_blocks: p.cooldown_blocks,
            authority: p.authority, protocol_fee_bps: p.protocol_fee_bps,
            graduation_timeout_blocks: p.graduation_timeout_blocks,
        }
    }
}

// ====================== EVENTS ======================

/// Emitted when graduation is initiated for a token.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraduationInitiated {
    pub token_index: String,
    pub mcap_at_trigger: u64,
    pub timestamp: u64,
}

impl From<proto::GraduationInitiated> for GraduationInitiated {
    fn from(p: proto::GraduationInitiated) -> Self {
        Self { token_index: p.token_index, mcap_at_trigger: p.mcap_at_trigger, timestamp: ts_secs(p.timestamp) }
    }
}

/// Emitted when liquidity is drained from the CLAMM pool.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiquidityDrained {
    pub token_index: String,
    pub pool_id: String,
    pub drained_amount_base: String,
    pub drained_amount_quote: String,
    pub returned_to_lps_base: String,
    pub returned_to_lps_quote: String,
    pub protocol_fee_base: String,
    pub protocol_fee_quote: String,
    pub timestamp: u64,
}

impl From<proto::LiquidityDrained> for LiquidityDrained {
    fn from(p: proto::LiquidityDrained) -> Self {
        Self {
            token_index: p.token_index, pool_id: p.pool_id,
            drained_amount_base: p.drained_amount_base, drained_amount_quote: p.drained_amount_quote,
            returned_to_lps_base: p.returned_to_lps_base, returned_to_lps_quote: p.returned_to_lps_quote,
            protocol_fee_base: p.protocol_fee_base, protocol_fee_quote: p.protocol_fee_quote,
            timestamp: ts_secs(p.timestamp),
        }
    }
}

/// Emitted when a spot market is created for the token.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpotMarketCreated {
    pub token_index: String,
    pub market_index: String,
    pub timestamp: u64,
}

impl From<proto::SpotMarketCreated> for SpotMarketCreated {
    fn from(p: proto::SpotMarketCreated) -> Self {
        Self { token_index: p.token_index, market_index: p.market_index, timestamp: ts_secs(p.timestamp) }
    }
}

/// Emitted when a perpetual market is created for the token.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PerpMarketCreated {
    pub token_index: String,
    pub market_index: String,
    pub timestamp: u64,
}

impl From<proto::PerpMarketCreated> for PerpMarketCreated {
    fn from(p: proto::PerpMarketCreated) -> Self {
        Self { token_index: p.token_index, market_index: p.market_index, timestamp: ts_secs(p.timestamp) }
    }
}

/// Emitted when graduation completes successfully.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraduationComplete {
    pub token_index: String,
    pub spot_market_index: String,
    pub perp_market_index: String,
    pub timestamp: u64,
}

impl From<proto::GraduationComplete> for GraduationComplete {
    fn from(p: proto::GraduationComplete) -> Self {
        Self {
            token_index: p.token_index, spot_market_index: p.spot_market_index,
            perp_market_index: p.perp_market_index, timestamp: ts_secs(p.timestamp),
        }
    }
}

/// Emitted when graduation fails.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraduationFailed {
    pub token_index: String,
    pub reason: String,
    pub timestamp: u64,
}

impl From<proto::GraduationFailed> for GraduationFailed {
    fn from(p: proto::GraduationFailed) -> Self {
        Self { token_index: p.token_index, reason: p.reason, timestamp: ts_secs(p.timestamp) }
    }
}

/// Emitted when a rollback is attempted after a failed step.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GraduationRollbackAttempted {
    pub token_index: String,
    pub failed_step: u32,
    pub reason: String,
    pub timestamp: u64,
}

impl From<proto::GraduationRollbackAttempted> for GraduationRollbackAttempted {
    fn from(p: proto::GraduationRollbackAttempted) -> Self {
        Self {
            token_index: p.token_index, failed_step: p.failed_step,
            reason: p.reason, timestamp: ts_secs(p.timestamp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn graduation_status_roundtrip() {
        for s in [GraduationStatus::Pending, GraduationStatus::Draining,
                   GraduationStatus::MarketsCreated, GraduationStatus::Completed,
                   GraduationStatus::Failed] {
            let v: i32 = s.into();
            assert_eq!(s, GraduationStatus::from(v));
        }
    }

    #[test]
    fn graduation_state_from_proto() {
        let p = proto::GraduationState {
            token_index: "42".into(), status: 1,
            spot_market_index: "100".into(), perp_market_index: "101".into(),
            clamm_pool_id: "0x1234".into(), initiated_at: 1000, completed_at: 0,
            failure_reason: String::new(),
            checkpoints: vec![proto::GraduationCheckpoint {
                step: 1, description: "draining".into(),
                at: Some(morpheum_proto::google::protobuf::Timestamp { seconds: 1000, nanos: 0 }),
            }],
            last_activity: Some(morpheum_proto::google::protobuf::Timestamp { seconds: 1000, nanos: 0 }),
            failed_step: 0,
        };
        let state: GraduationState = p.into();
        assert_eq!(state.token_index, "42");
        assert_eq!(state.status, GraduationStatus::Pending);
        assert_eq!(state.checkpoints.len(), 1);
        assert_eq!(state.checkpoints[0].step, 1);
    }
}
