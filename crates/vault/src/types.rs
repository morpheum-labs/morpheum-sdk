//! Domain types for the vault module.
//!
//! Covers vault types/statuses, vault records, stakes, strategy executions,
//! IL metrics, health snapshots, revenue share config, governance params,
//! and streaming events.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::vault::v1 as proto;

// ====================== ENUMS ======================

/// Vault type — custom strategy or yield.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VaultType {
    #[default]
    Unspecified,
    Custom,
    Yield,
}

impl From<i32> for VaultType {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Custom,
            2 => Self::Yield,
            _ => Self::Unspecified,
        }
    }
}

impl From<VaultType> for i32 {
    fn from(v: VaultType) -> Self {
        match v {
            VaultType::Unspecified => 0,
            VaultType::Custom => 1,
            VaultType::Yield => 2,
        }
    }
}

/// Vault lifecycle status.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VaultStatus {
    #[default]
    Unspecified,
    Active,
    Paused,
    Executing,
    Cooldown,
    Liquidating,
}

impl From<i32> for VaultStatus {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Active,     2 => Self::Paused,
            3 => Self::Executing,  4 => Self::Cooldown,
            5 => Self::Liquidating,
            _ => Self::Unspecified,
        }
    }
}

impl From<VaultStatus> for i32 {
    fn from(s: VaultStatus) -> Self {
        match s {
            VaultStatus::Unspecified => 0,  VaultStatus::Active => 1,
            VaultStatus::Paused => 2,       VaultStatus::Executing => 3,
            VaultStatus::Cooldown => 4,     VaultStatus::Liquidating => 5,
        }
    }
}

// ====================== HELPERS ======================

fn ts_to_u64(ts: &Option<morpheum_proto::google::protobuf::Timestamp>) -> u64 {
    ts.as_ref().map_or(0, |t| t.seconds as u64)
}

fn extract_asset(a: &Option<morpheum_proto::primitives::v1::Asset>) -> (u64, String) {
    a.as_ref().map_or((0, String::new()), |a| (a.asset_index, a.symbol.clone()))
}

// ====================== DOMAIN TYPES ======================

/// Rich on-wire vault record.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vault {
    pub vault_id: String,
    pub agent_id: String,
    pub vault_type: VaultType,
    pub name: String,
    pub description: String,
    pub asset_index: u64,
    pub asset_symbol: String,
    pub total_assets: String,
    pub available_assets: String,
    pub reserved_assets: String,
    pub status: VaultStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub strategy_hash: String,
    pub health_score: String,
    pub pnl_30d_usd: String,
    pub apy_bps: String,
    pub vc_claim_hash: Vec<u8>,
    pub copy_count: String,
}

impl From<proto::Vault> for Vault {
    fn from(p: proto::Vault) -> Self {
        let (asset_index, asset_symbol) = extract_asset(&p.asset);
        Self {
            vault_id: p.vault_id, agent_id: p.agent_id,
            vault_type: VaultType::from(p.r#type), name: p.name,
            description: p.description, asset_index, asset_symbol,
            total_assets: p.total_assets, available_assets: p.available_assets,
            reserved_assets: p.reserved_assets, status: VaultStatus::from(p.status),
            created_at: ts_to_u64(&p.created_at), updated_at: ts_to_u64(&p.updated_at),
            strategy_hash: p.strategy_hash, health_score: p.health_score,
            pnl_30d_usd: p.pnl_30d_usd, apy_bps: p.apy_bps,
            vc_claim_hash: p.vc_claim_hash, copy_count: p.copy_count,
        }
    }
}

/// Compact keeper vault record.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VaultRecord {
    pub vault_id: String,
    pub agent_id: String,
    pub vault_type: VaultType,
    pub status: VaultStatus,
    pub total_assets: String,
    pub available_assets: String,
    pub health_score: String,
    pub pnl_30d_usd: String,
    pub strategy_hash: Vec<u8>,
    pub last_executed: u64,
}

impl From<proto::VaultRecord> for VaultRecord {
    fn from(p: proto::VaultRecord) -> Self {
        Self {
            vault_id: p.vault_id, agent_id: p.agent_id,
            vault_type: VaultType::from(p.r#type), status: VaultStatus::from(p.status),
            total_assets: p.total_assets, available_assets: p.available_assets,
            health_score: p.health_score, pnl_30d_usd: p.pnl_30d_usd,
            strategy_hash: p.strategy_hash, last_executed: p.last_executed,
        }
    }
}

/// User position in a vault (yield-bearing shares).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Stake {
    pub stake_id: String,
    pub address: String,
    pub vault_id: String,
    pub asset_index: u64,
    pub asset_symbol: String,
    pub amount: String,
    pub shares: String,
    pub pending_yield: String,
    pub stake_time: u64,
    pub last_claim_time: u64,
}

impl From<proto::Stake> for Stake {
    fn from(p: proto::Stake) -> Self {
        let (asset_index, asset_symbol) = extract_asset(&p.asset);
        Self {
            stake_id: p.stake_id, address: p.address, vault_id: p.vault_id,
            asset_index, asset_symbol, amount: p.amount, shares: p.shares,
            pending_yield: p.pending_yield,
            stake_time: ts_to_u64(&p.stake_time),
            last_claim_time: ts_to_u64(&p.last_claim_time),
        }
    }
}

/// Immutable record of a strategy execution.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StrategyExecution {
    pub execution_id: String,
    pub vault_id: String,
    pub pnl: String,
    pub success: bool,
    pub error_message: String,
    pub timestamp: u64,
    pub memory_snapshot_hash: Vec<u8>,
}

impl From<proto::StrategyExecution> for StrategyExecution {
    fn from(p: proto::StrategyExecution) -> Self {
        Self {
            execution_id: p.execution_id, vault_id: p.vault_id,
            pnl: p.pnl, success: p.success, error_message: p.error_message,
            timestamp: ts_to_u64(&p.timestamp),
            memory_snapshot_hash: p.memory_snapshot_hash,
        }
    }
}

/// Impermanent loss tracking metrics.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IlMetrics {
    pub vault_id: String,
    pub current_il: String,
    pub avg_il_24h: String,
    pub timestamp: u64,
}

impl From<proto::IlMetrics> for IlMetrics {
    fn from(p: proto::IlMetrics) -> Self {
        Self {
            vault_id: p.vault_id, current_il: p.current_il,
            avg_il_24h: p.avg_il_24h, timestamp: ts_to_u64(&p.timestamp),
        }
    }
}

/// Real-time vault health snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VaultHealth {
    pub vault_id: String,
    pub health_score: String,
    pub apy_bps: String,
    pub pnl_24h: String,
    pub risk_score: String,
    pub timestamp: u64,
}

impl From<proto::VaultHealth> for VaultHealth {
    fn from(p: proto::VaultHealth) -> Self {
        Self {
            vault_id: p.vault_id, health_score: p.health_score,
            apy_bps: p.apy_bps, pnl_24h: p.pnl_24h,
            risk_score: p.risk_score, timestamp: ts_to_u64(&p.timestamp),
        }
    }
}

/// Revenue share configuration for a vault.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RevenueShareConfig {
    pub creator_cut_bps: u32,
    pub platform_cut_bps: u32,
    pub evaluator_cut_bps: u32,
}

impl From<proto::RevenueShareConfig> for RevenueShareConfig {
    fn from(p: proto::RevenueShareConfig) -> Self {
        Self {
            creator_cut_bps: p.creator_cut_bps,
            platform_cut_bps: p.platform_cut_bps,
            evaluator_cut_bps: p.evaluator_cut_bps,
        }
    }
}

/// Governance-tunable vault module parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VaultParams {
    pub max_vaults_per_agent: u64,
    pub min_initial_stake_usd: u64,
    pub max_strategy_complexity: u64,
    pub treasury_cut_bps: u64,
    pub last_updated: u64,
}

impl From<proto::Params> for VaultParams {
    fn from(p: proto::Params) -> Self {
        Self {
            max_vaults_per_agent: p.max_vaults_per_agent,
            min_initial_stake_usd: p.min_initial_stake_usd,
            max_strategy_complexity: p.max_strategy_complexity,
            treasury_cut_bps: p.treasury_cut_bps,
            last_updated: ts_to_u64(&p.last_updated),
        }
    }
}

impl From<VaultParams> for proto::Params {
    fn from(p: VaultParams) -> Self {
        Self {
            max_vaults_per_agent: p.max_vaults_per_agent,
            min_initial_stake_usd: p.min_initial_stake_usd,
            max_strategy_complexity: p.max_strategy_complexity,
            treasury_cut_bps: p.treasury_cut_bps,
            last_updated: None,
        }
    }
}

// ====================== STREAM EVENTS ======================

/// Union of vault streaming events (from `VaultUpdate.oneof event`).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VaultUpdateEvent {
    VaultUpdate(alloc::boxed::Box<Vault>),
    ExecutionUpdate(StrategyExecution),
    IlUpdate(IlMetrics),
    HealthUpdate(VaultHealth),
}

/// Top-level vault streaming event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VaultStreamEvent {
    pub event_type: String,
    pub event: Option<VaultUpdateEvent>,
    pub timestamp: u64,
}

impl VaultStreamEvent {
    /// Converts from the proto `VaultUpdate` wrapper.
    pub fn from_proto(p: proto::VaultUpdate) -> Self {
        let event = p.event.map(|e| match e {
            proto::vault_update::Event::VaultUpdate(v) => VaultUpdateEvent::VaultUpdate(alloc::boxed::Box::new(v.into())),
            proto::vault_update::Event::ExecutionUpdate(e) => VaultUpdateEvent::ExecutionUpdate(e.into()),
            proto::vault_update::Event::IlUpdate(il) => VaultUpdateEvent::IlUpdate(il.into()),
            proto::vault_update::Event::HealthUpdate(h) => VaultUpdateEvent::HealthUpdate(h.into()),
        });
        Self {
            event_type: p.event_type, event, timestamp: ts_to_u64(&p.timestamp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn vault_type_roundtrip() {
        for t in [VaultType::Custom, VaultType::Yield] {
            assert_eq!(t, VaultType::from(i32::from(t)));
        }
        assert_eq!(VaultType::Unspecified, VaultType::from(99));
    }

    #[test]
    fn vault_status_roundtrip() {
        for s in [VaultStatus::Active, VaultStatus::Paused, VaultStatus::Executing,
                  VaultStatus::Cooldown, VaultStatus::Liquidating] {
            assert_eq!(s, VaultStatus::from(i32::from(s)));
        }
    }

    #[test]
    fn vault_from_proto() {
        let p = proto::Vault {
            vault_id: "v1".into(), agent_id: "a1".into(), r#type: 1,
            name: "Test".into(), description: "Desc".into(),
            asset: Some(morpheum_proto::primitives::v1::Asset {
                asset_index: 1, symbol: "MORM".into(), ..Default::default()
            }),
            total_assets: "1000".into(), available_assets: "800".into(),
            reserved_assets: "200".into(), status: 1,
            created_at: None, updated_at: None, strategy_hash: "abc".into(),
            health_score: "9500".into(), pnl_30d_usd: "100".into(),
            apy_bps: "1200".into(), vc_claim_hash: vec![], copy_count: "5".into(),
        };
        let v: Vault = p.into();
        assert_eq!(v.vault_type, VaultType::Custom);
        assert_eq!(v.asset_symbol, "MORM");
        assert_eq!(v.total_assets, "1000");
    }

    #[test]
    fn vault_stream_event_from_proto() {
        let p = proto::VaultUpdate {
            event_type: "health_updated".into(),
            event: Some(proto::vault_update::Event::HealthUpdate(proto::VaultHealth {
                vault_id: "v1".into(), health_score: "9500".into(),
                apy_bps: "1200".into(), pnl_24h: "50".into(),
                risk_score: "300".into(), timestamp: None,
            })),
            timestamp: None,
        };
        let e = VaultStreamEvent::from_proto(p);
        assert_eq!(e.event_type, "health_updated");
        assert!(matches!(e.event, Some(VaultUpdateEvent::HealthUpdate(_))));
    }

    #[test]
    fn params_roundtrip() {
        let p = VaultParams {
            max_vaults_per_agent: 100, min_initial_stake_usd: 100,
            max_strategy_complexity: 50, treasury_cut_bps: 500, last_updated: 0,
        };
        let proto_p: proto::Params = p.clone().into();
        let p2: VaultParams = proto_p.into();
        assert_eq!(p.max_vaults_per_agent, p2.max_vaults_per_agent);
        assert_eq!(p.treasury_cut_bps, p2.treasury_cut_bps);
    }
}
