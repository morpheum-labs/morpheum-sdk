//! Domain types for the treasury module.
//!
//! Covers reserve categories, per-category state, aggregate reserves,
//! allocation history, treasury metrics, and governance parameters.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::treasury::v1 as proto;

// ====================== ENUMS ======================

/// Dedicated Treasury Reserve Category.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ReserveCategory {
    #[default]
    Unspecified,
    InsuranceProtection,
    LiquidityIncentives,
    BuybackBurn,
    OperationsEcosystem,
    StrategicInitiatives,
    EmergencyStabilization,
}

impl From<i32> for ReserveCategory {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::InsuranceProtection,   2 => Self::LiquidityIncentives,
            3 => Self::BuybackBurn,           4 => Self::OperationsEcosystem,
            5 => Self::StrategicInitiatives,  6 => Self::EmergencyStabilization,
            _ => Self::Unspecified,
        }
    }
}

impl From<ReserveCategory> for i32 {
    fn from(c: ReserveCategory) -> Self {
        match c {
            ReserveCategory::Unspecified => 0,           ReserveCategory::InsuranceProtection => 1,
            ReserveCategory::LiquidityIncentives => 2,   ReserveCategory::BuybackBurn => 3,
            ReserveCategory::OperationsEcosystem => 4,   ReserveCategory::StrategicInitiatives => 5,
            ReserveCategory::EmergencyStabilization => 6,
        }
    }
}

// ====================== DOMAIN TYPES ======================

/// Per-category state record.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CategoryReserve {
    pub category: ReserveCategory,
    pub balance: u64,
    pub allocation_bps: u32,
    pub last_updated: u64,
    pub metadata: Vec<u8>,
}

impl From<proto::CategoryReserve> for CategoryReserve {
    fn from(p: proto::CategoryReserve) -> Self {
        Self {
            category: ReserveCategory::from(p.category), balance: p.balance,
            allocation_bps: p.allocation_bps, last_updated: p.last_updated,
            metadata: p.metadata,
        }
    }
}

/// Single source of truth for all protocol reserves.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ReservesState {
    pub total_reserves: u64,
    pub categories: Vec<CategoryReserve>,
    pub merkle_root: Vec<u8>,
    pub last_sweep_timestamp: u64,
    pub last_rebalance_timestamp: u64,
}

impl From<proto::ReservesState> for ReservesState {
    fn from(p: proto::ReservesState) -> Self {
        Self {
            total_reserves: p.total_reserves,
            categories: p.categories.into_iter().map(Into::into).collect(),
            merkle_root: p.merkle_root,
            last_sweep_timestamp: p.last_sweep_timestamp,
            last_rebalance_timestamp: p.last_rebalance_timestamp,
        }
    }
}

/// Immutable allocation history entry.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AllocationRecord {
    pub allocation_id: u64,
    pub source_category: ReserveCategory,
    pub target_category: ReserveCategory,
    pub amount: u64,
    pub reason: String,
    pub timestamp: u64,
    pub tx_hash: Vec<u8>,
    pub authority: String,
    pub signature: Vec<u8>,
}

impl From<proto::AllocationRecord> for AllocationRecord {
    fn from(p: proto::AllocationRecord) -> Self {
        Self {
            allocation_id: p.allocation_id,
            source_category: ReserveCategory::from(p.source_category),
            target_category: ReserveCategory::from(p.target_category),
            amount: p.amount, reason: p.reason, timestamp: p.timestamp,
            tx_hash: p.tx_hash, authority: p.authority, signature: p.signature,
        }
    }
}

/// Real-time treasury health and performance metrics.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TreasuryMetrics {
    pub total_reserves: u64,
    pub insurance_protection_balance: u64,
    pub buyback_burn_balance: u64,
    pub reserve_to_oi_ratio_bps: u64,
    pub insurance_coverage_ratio_bps: u64,
    pub projected_runway_days: u64,
    pub last_updated: u64,
}

impl From<proto::TreasuryMetrics> for TreasuryMetrics {
    fn from(p: proto::TreasuryMetrics) -> Self {
        Self {
            total_reserves: p.total_reserves,
            insurance_protection_balance: p.insurance_protection_balance,
            buyback_burn_balance: p.buyback_burn_balance,
            reserve_to_oi_ratio_bps: p.reserve_to_oi_ratio_bps,
            insurance_coverage_ratio_bps: p.insurance_coverage_ratio_bps,
            projected_runway_days: p.projected_runway_days,
            last_updated: p.last_updated.as_ref().map_or(0, |t| t.seconds as u64),
        }
    }
}

/// Governance-tunable module parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TreasuryParams {
    pub insurance_protection_bps: u32,
    pub liquidity_incentives_bps: u32,
    pub buyback_burn_bps: u32,
    pub operations_ecosystem_bps: u32,
    pub strategic_initiatives_bps: u32,
    pub emergency_stabilization_bps: u32,
    pub min_insurance_coverage_bps: u64,
    pub auto_rebalance_threshold_bps: u64,
    pub max_single_allocation_bps: u64,
    pub buyback_frequency_blocks: u64,
    pub min_buyback_amount: u64,
}

impl From<proto::Params> for TreasuryParams {
    fn from(p: proto::Params) -> Self {
        Self {
            insurance_protection_bps: p.insurance_protection_bps,
            liquidity_incentives_bps: p.liquidity_incentives_bps,
            buyback_burn_bps: p.buyback_burn_bps,
            operations_ecosystem_bps: p.operations_ecosystem_bps,
            strategic_initiatives_bps: p.strategic_initiatives_bps,
            emergency_stabilization_bps: p.emergency_stabilization_bps,
            min_insurance_coverage_bps: p.min_insurance_coverage_bps,
            auto_rebalance_threshold_bps: p.auto_rebalance_threshold_bps,
            max_single_allocation_bps: p.max_single_allocation_bps,
            buyback_frequency_blocks: p.buyback_frequency_blocks,
            min_buyback_amount: p.min_buyback_amount,
        }
    }
}

impl From<TreasuryParams> for proto::Params {
    fn from(p: TreasuryParams) -> Self {
        Self {
            insurance_protection_bps: p.insurance_protection_bps,
            liquidity_incentives_bps: p.liquidity_incentives_bps,
            buyback_burn_bps: p.buyback_burn_bps,
            operations_ecosystem_bps: p.operations_ecosystem_bps,
            strategic_initiatives_bps: p.strategic_initiatives_bps,
            emergency_stabilization_bps: p.emergency_stabilization_bps,
            min_insurance_coverage_bps: p.min_insurance_coverage_bps,
            auto_rebalance_threshold_bps: p.auto_rebalance_threshold_bps,
            max_single_allocation_bps: p.max_single_allocation_bps,
            buyback_frequency_blocks: p.buyback_frequency_blocks,
            min_buyback_amount: p.min_buyback_amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn reserve_category_roundtrip() {
        for c in [ReserveCategory::InsuranceProtection, ReserveCategory::LiquidityIncentives,
                  ReserveCategory::BuybackBurn, ReserveCategory::OperationsEcosystem,
                  ReserveCategory::StrategicInitiatives, ReserveCategory::EmergencyStabilization] {
            let v: i32 = c.into();
            assert_eq!(c, ReserveCategory::from(v));
        }
        assert_eq!(ReserveCategory::Unspecified, ReserveCategory::from(99));
    }

    #[test]
    fn params_roundtrip() {
        let p = TreasuryParams {
            insurance_protection_bps: 4000, liquidity_incentives_bps: 2500,
            buyback_burn_bps: 2000, operations_ecosystem_bps: 1000,
            strategic_initiatives_bps: 300, emergency_stabilization_bps: 200,
            min_insurance_coverage_bps: 1500, auto_rebalance_threshold_bps: 500,
            max_single_allocation_bps: 2000, buyback_frequency_blocks: 100,
            min_buyback_amount: 1_000_000,
        };
        let proto_p: proto::Params = p.clone().into();
        let p2: TreasuryParams = proto_p.into();
        assert_eq!(p, p2);
    }

    #[test]
    fn reserves_state_from_proto() {
        let p = proto::ReservesState {
            total_reserves: 1_000_000,
            categories: vec![proto::CategoryReserve {
                category: 1, balance: 400_000, allocation_bps: 4000,
                last_updated: 100, metadata: vec![],
            }],
            merkle_root: vec![0xAB], last_sweep_timestamp: 90,
            last_rebalance_timestamp: 80,
        };
        let s: ReservesState = p.into();
        assert_eq!(s.total_reserves, 1_000_000);
        assert_eq!(s.categories.len(), 1);
        assert_eq!(s.categories[0].category, ReserveCategory::InsuranceProtection);
    }

    #[test]
    fn treasury_metrics_from_proto() {
        let p = proto::TreasuryMetrics {
            total_reserves: 1_000_000, insurance_protection_balance: 400_000,
            buyback_burn_balance: 200_000, reserve_to_oi_ratio_bps: 1500,
            insurance_coverage_ratio_bps: 2000, projected_runway_days: 365,
            last_updated: Some(morpheum_proto::google::protobuf::Timestamp { seconds: 1700000000, nanos: 0 }),
        };
        let m: TreasuryMetrics = p.into();
        assert_eq!(m.projected_runway_days, 365);
        assert_eq!(m.last_updated, 1700000000);
    }
}
