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
    /// VA5 — protocol-owned MLP house liquidity + liquidation backstop.
    Protocol,
}

impl From<i32> for VaultType {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Custom,
            2 => Self::Yield,
            3 => Self::Protocol,
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
            VaultType::Protocol => 3,
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
            1 => Self::Active,
            2 => Self::Paused,
            3 => Self::Executing,
            4 => Self::Cooldown,
            5 => Self::Liquidating,
            _ => Self::Unspecified,
        }
    }
}

impl From<VaultStatus> for i32 {
    fn from(s: VaultStatus) -> Self {
        match s {
            VaultStatus::Unspecified => 0,
            VaultStatus::Active => 1,
            VaultStatus::Paused => 2,
            VaultStatus::Executing => 3,
            VaultStatus::Cooldown => 4,
            VaultStatus::Liquidating => 5,
        }
    }
}

/// VB9 (spec §7 / §12 gate #4) — the fee model a vault is created under. Governs
/// the create-time fee-validation gate; `Unspecified` is the inert value and
/// coalesces to `Standard` when the gate is armed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VaultFeePreset {
    #[default]
    Unspecified,
    /// 5-15% perf (default 10%), 0% management locked.
    Standard,
    /// 10-25% perf (default 15%), 0-1% management; eligibility-gated.
    Premium,
    /// Protocol-defined; `MsgCreateProtocolVault` only.
    Protocol,
}

impl From<i32> for VaultFeePreset {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Standard,
            2 => Self::Premium,
            3 => Self::Protocol,
            _ => Self::Unspecified,
        }
    }
}

impl From<VaultFeePreset> for i32 {
    fn from(p: VaultFeePreset) -> Self {
        match p {
            VaultFeePreset::Unspecified => 0,
            VaultFeePreset::Standard => 1,
            VaultFeePreset::Premium => 2,
            VaultFeePreset::Protocol => 3,
        }
    }
}

// ====================== HELPERS ======================

fn ts_to_u64(ts: &Option<morpheum_proto::google::protobuf::Timestamp>) -> u64 {
    ts.as_ref().map_or(0, |t| t.seconds as u64)
}

fn extract_asset(a: &Option<morpheum_proto::primitives::v1::Asset>) -> (u64, String) {
    a.as_ref()
        .map_or((0, String::new()), |a| (a.asset_index, a.symbol.clone()))
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
    pub total_shares: String,
    pub bucket_id: String,
    pub collateral_asset_index: u32,
    pub deployed_assets: String,
    pub high_water_mark: String,
    pub clmm_pool_id: String,
    pub clmm_position_id: String,
    pub clmm_collateral_token_index: u32,
    pub clmm_deployed_assets: String,
    /// VB2 (spec §7 / §12) — persisted fee model. The leader (creator / agent
    /// manager) performance-fee payout recipient; the crystallized fee is split
    /// to this address plus the treasury and the insurance/MLP reserve.
    pub leader_payout_address: String,
    /// Performance-fee rate (bps of profit above the high-water mark). "0" ⇒
    /// inherit the live `treasury_cut_bps` (pre-VB2 vaults).
    pub performance_fee_bps: u32,
    /// Management-fee rate (bps of AUM); locked at 0 for the Standard preset.
    pub management_fee_bps: u32,
    /// VB3 (spec §8 / G3) — the leader's custody / stake-key address, captured at
    /// create. The key under which the leader's first-loss `Stake` accrues for the
    /// skin-in-the-game clamp / soft-close (distinct from `agent_id` and
    /// `leader_payout_address`). Empty ⇒ the skin gate is inert for the vault.
    pub leader_custody_address: String,
    /// VB5 (spec §14 G4) — hard deposit capacity cap in base-asset native units.
    /// "0" / empty ⇒ uncapped.
    pub deposit_capacity_native: String,
    /// VB5 (spec §14 G4) — manager soft-close: while true, new deposits are
    /// rejected; existing depositors may stay and redeem.
    pub soft_closed: bool,
    /// VB6 (spec P7 / §2) — operating mandate. Empty `allowed_markets` ⇒ all
    /// markets; `max_leverage = 0` ⇒ unbounded. Once armed, updates are
    /// tightening-only.
    pub mandate: VaultMandate,
    /// VB7 (spec §5) — buffer-floor auto-allocation policy. Empty / zero
    /// targets ⇒ disarmed (deployment stays fully manual).
    pub allocation_policy: AllocationPolicy,
    /// D5 (spec §1) — creator's agent identity (`tx_meta.agent_hash`). The
    /// delegation target for `DELEGATION_SCOPE_VAULT`. Empty ⇒ delegation
    /// disabled (byte-identical to pre-D5).
    pub owner_agent_hash: String,
    /// D10 (spec §7 / §15 #12) — last epoch at which a periodic fee
    /// crystallization ran (`epoch = height / fee_crystallization_interval_blocks`).
    /// Seeded 0 at create; inert while the cadence is disarmed.
    pub last_fee_crystallization_epoch: u64,
    /// D8 (spec §3) — the vault's owned margin buckets (SSOT). A vault may own N
    /// buckets (Cross and/or Isolated). Empty until the first deploy; the scalar
    /// `bucket_id` / `collateral_asset_index` / `deployed_assets` above are the
    /// derived mirror (primary bucket + total cost basis).
    pub buckets: Vec<VaultBucket>,
    /// VB9 (spec §7 / §12 gate #4) — the fee preset this vault was created under.
    /// `Unspecified` on pre-VB9 vaults and when the preset gate is disarmed.
    pub fee_preset: VaultFeePreset,
    /// D6 (spec §14 G6) — deterministic block-time (whole seconds) of the
    /// manager's most recent authorized state-changing op. Written only while the
    /// dead-man switch is armed; `0` ⇒ untracked (never auto-paused, fail-safe).
    pub last_manager_activity_secs: u64,
}

/// VB6 (spec P7 / §2) — per-vault operating constraints.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VaultMandate {
    /// Markets the vault may open / add exposure on. Empty ⇒ unconstrained.
    pub allowed_markets: Vec<u64>,
    /// Maximum leverage the vault may configure per market. 0 ⇒ unbounded.
    pub max_leverage: u32,
    /// VA3 — assets the vault may hold as SpotToken NAV legs. Empty ⇒ no
    /// SpotToken holdings (byte-identical to `{cash, bucket, clmm}`). Opposite
    /// of `allowed_markets`.
    pub allowed_assets: Vec<u64>,
    /// D9 spot leg — governed CLMM exit pool per whitelisted spot asset. The
    /// single source of truth for the base⇄asset pool used by acquisition,
    /// manager exit default, and forced spot liquidation on redemption. One
    /// entry per asset; each `asset_index` must be in `allowed_assets`.
    pub spot_exit_pools: Vec<SpotExitPool>,
}

/// D9 spot leg — a governed base⇄asset CLMM exit pool binding.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SpotExitPool {
    /// Whitelisted spot token (must be in `allowed_assets`, != base).
    pub asset_index: u32,
    /// CLMM pool pairing (base, asset_index) — 0x-hex or decimal id.
    pub pool_id: String,
}

impl From<proto::SpotExitPool> for SpotExitPool {
    fn from(p: proto::SpotExitPool) -> Self {
        Self {
            asset_index: p.asset_index,
            pool_id: p.pool_id,
        }
    }
}

impl From<SpotExitPool> for proto::SpotExitPool {
    fn from(b: SpotExitPool) -> Self {
        Self {
            asset_index: b.asset_index,
            pool_id: b.pool_id,
        }
    }
}

/// VB7 (spec §5) — destination tier for a target weight.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AllocationKind {
    #[default]
    Unspecified,
    Bucket,
    SpotToken,
}

impl From<i32> for AllocationKind {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Bucket,
            2 => Self::SpotToken,
            _ => Self::Unspecified,
        }
    }
}

impl From<AllocationKind> for i32 {
    fn from(v: AllocationKind) -> Self {
        match v {
            AllocationKind::Unspecified => 0,
            AllocationKind::Bucket => 1,
            AllocationKind::SpotToken => 2,
        }
    }
}

/// VB7 (spec §5) — one destination's target weight as a fraction of NAV.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AllocationTarget {
    pub kind: AllocationKind,
    pub target_weight_bps: u32,
    /// SPOT_TOKEN only: the whitelisted spot asset this target allocates into
    /// (ignored / 0 for BUCKET).
    pub asset_index: u32,
}

impl From<proto::AllocationTarget> for AllocationTarget {
    fn from(p: proto::AllocationTarget) -> Self {
        Self {
            kind: AllocationKind::from(p.kind),
            target_weight_bps: p.target_weight_bps,
            asset_index: p.asset_index,
        }
    }
}

impl From<AllocationTarget> for proto::AllocationTarget {
    fn from(t: AllocationTarget) -> Self {
        Self {
            kind: i32::from(t.kind),
            target_weight_bps: t.target_weight_bps,
            asset_index: t.asset_index,
        }
    }
}

/// VB7 (spec §5) — per-vault buffer-floor auto-allocation policy.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AllocationPolicy {
    pub cash_buffer_floor_bps: u32,
    pub deployment_ceiling_bps: u32,
    pub targets: Vec<AllocationTarget>,
}

impl From<proto::AllocationPolicy> for AllocationPolicy {
    fn from(p: proto::AllocationPolicy) -> Self {
        Self {
            cash_buffer_floor_bps: p.cash_buffer_floor_bps,
            deployment_ceiling_bps: p.deployment_ceiling_bps,
            targets: p.targets.into_iter().map(AllocationTarget::from).collect(),
        }
    }
}

impl From<AllocationPolicy> for proto::AllocationPolicy {
    fn from(p: AllocationPolicy) -> Self {
        Self {
            cash_buffer_floor_bps: p.cash_buffer_floor_bps,
            deployment_ceiling_bps: p.deployment_ceiling_bps,
            targets: p.targets.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<proto::VaultMandate> for VaultMandate {
    fn from(p: proto::VaultMandate) -> Self {
        Self {
            allowed_markets: p.allowed_markets,
            max_leverage: p.max_leverage,
            allowed_assets: p.allowed_assets,
            spot_exit_pools: p
                .spot_exit_pools
                .into_iter()
                .map(SpotExitPool::from)
                .collect(),
        }
    }
}

impl From<VaultMandate> for proto::VaultMandate {
    fn from(m: VaultMandate) -> Self {
        Self {
            allowed_markets: m.allowed_markets,
            max_leverage: m.max_leverage,
            allowed_assets: m.allowed_assets,
            spot_exit_pools: m.spot_exit_pools.into_iter().map(Into::into).collect(),
        }
    }
}

/// D8 (spec §3) — margin isolation mode of an owned bucket.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BucketMode {
    /// Treated as Cross.
    #[default]
    Unspecified,
    /// Shared margin across the bucket's markets.
    Cross,
    /// Isolated margin (one position per bucket).
    Isolated,
}

impl From<i32> for BucketMode {
    fn from(v: i32) -> Self {
        match proto::BucketMode::try_from(v).unwrap_or(proto::BucketMode::Unspecified) {
            proto::BucketMode::Unspecified => Self::Unspecified,
            proto::BucketMode::Cross => Self::Cross,
            proto::BucketMode::Isolated => Self::Isolated,
        }
    }
}

impl From<BucketMode> for i32 {
    fn from(m: BucketMode) -> Self {
        match m {
            BucketMode::Unspecified => 0,
            BucketMode::Cross => 1,
            BucketMode::Isolated => 2,
        }
    }
}

/// D8 (spec §3) — one owned margin bucket with its own principal cost basis and
/// isolation mode.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VaultBucket {
    pub bucket_id: String,
    pub collateral_asset_index: u32,
    /// string(uint256) — this bucket's principal cost basis.
    pub deployed_assets: String,
    pub mode: BucketMode,
}

impl From<proto::VaultBucket> for VaultBucket {
    fn from(p: proto::VaultBucket) -> Self {
        Self {
            bucket_id: p.bucket_id,
            collateral_asset_index: p.collateral_asset_index,
            deployed_assets: p.deployed_assets,
            mode: BucketMode::from(p.mode),
        }
    }
}

impl From<VaultBucket> for proto::VaultBucket {
    fn from(b: VaultBucket) -> Self {
        Self {
            bucket_id: b.bucket_id,
            collateral_asset_index: b.collateral_asset_index,
            deployed_assets: b.deployed_assets,
            mode: i32::from(b.mode),
        }
    }
}

/// VB9 (spec §7) — governance-armed fee envelope for one preset. A requested
/// performance fee must fall within `[min_perf_bps, max_perf_bps]` (`0` resolves
/// to `default_perf_bps`) and the management fee must be `<= max_mgmt_bps`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FeePresetBound {
    pub preset: VaultFeePreset,
    pub min_perf_bps: u32,
    pub max_perf_bps: u32,
    pub default_perf_bps: u32,
    pub max_mgmt_bps: u32,
}

impl From<proto::FeePresetBound> for FeePresetBound {
    fn from(p: proto::FeePresetBound) -> Self {
        Self {
            preset: VaultFeePreset::from(p.preset),
            min_perf_bps: p.min_perf_bps,
            max_perf_bps: p.max_perf_bps,
            default_perf_bps: p.default_perf_bps,
            max_mgmt_bps: p.max_mgmt_bps,
        }
    }
}

impl From<FeePresetBound> for proto::FeePresetBound {
    fn from(b: FeePresetBound) -> Self {
        Self {
            preset: i32::from(b.preset),
            min_perf_bps: b.min_perf_bps,
            max_perf_bps: b.max_perf_bps,
            default_perf_bps: b.default_perf_bps,
            max_mgmt_bps: b.max_mgmt_bps,
        }
    }
}

impl From<proto::Vault> for Vault {
    fn from(p: proto::Vault) -> Self {
        let (asset_index, asset_symbol) = extract_asset(&p.asset);
        Self {
            vault_id: p.vault_id,
            agent_id: p.agent_id,
            vault_type: VaultType::from(p.r#type),
            name: p.name,
            description: p.description,
            asset_index,
            asset_symbol,
            total_assets: p.total_assets,
            available_assets: p.available_assets,
            reserved_assets: p.reserved_assets,
            status: VaultStatus::from(p.status),
            created_at: ts_to_u64(&p.created_at),
            updated_at: ts_to_u64(&p.updated_at),
            strategy_hash: p.strategy_hash,
            health_score: p.health_score,
            pnl_30d_usd: p.pnl_30d_usd,
            apy_bps: p.apy_bps,
            vc_claim_hash: p.vc_claim_hash,
            copy_count: p.copy_count,
            total_shares: p.total_shares,
            bucket_id: p.bucket_id,
            collateral_asset_index: p.collateral_asset_index,
            deployed_assets: p.deployed_assets,
            high_water_mark: p.high_water_mark,
            clmm_pool_id: p.clmm_pool_id,
            clmm_position_id: p.clmm_position_id,
            clmm_collateral_token_index: p.clmm_collateral_token_index,
            clmm_deployed_assets: p.clmm_deployed_assets,
            leader_payout_address: p.leader_payout_address,
            performance_fee_bps: p.performance_fee_bps,
            management_fee_bps: p.management_fee_bps,
            leader_custody_address: p.leader_custody_address,
            deposit_capacity_native: p.deposit_capacity_native,
            soft_closed: p.soft_closed,
            mandate: p.mandate.map(VaultMandate::from).unwrap_or_default(),
            allocation_policy: p
                .allocation_policy
                .map(AllocationPolicy::from)
                .unwrap_or_default(),
            owner_agent_hash: p.owner_agent_hash,
            last_fee_crystallization_epoch: p.last_fee_crystallization_epoch,
            buckets: p.buckets.into_iter().map(VaultBucket::from).collect(),
            fee_preset: VaultFeePreset::from(p.fee_preset),
            last_manager_activity_secs: p.last_manager_activity_secs,
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
            vault_id: p.vault_id,
            agent_id: p.agent_id,
            vault_type: VaultType::from(p.r#type),
            status: VaultStatus::from(p.status),
            total_assets: p.total_assets,
            available_assets: p.available_assets,
            health_score: p.health_score,
            pnl_30d_usd: p.pnl_30d_usd,
            strategy_hash: p.strategy_hash,
            last_executed: p.last_executed,
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
    /// VB1 (spec §7 / G9) — the depositor's per-position high-water mark (e8
    /// fixed-point share price). "0" ⇒ par.
    pub high_water_mark: String,
    /// VB4 (spec §6 / §14 G1) — the per-deposit minimum-hold unlock time (unix
    /// secs); shares are redeemable once `now >= unlock_at`. 0 ⇒ no lock.
    pub unlock_at: u64,
}

impl From<proto::Stake> for Stake {
    fn from(p: proto::Stake) -> Self {
        let (asset_index, asset_symbol) = extract_asset(&p.asset);
        Self {
            stake_id: p.stake_id,
            address: p.address,
            vault_id: p.vault_id,
            asset_index,
            asset_symbol,
            amount: p.amount,
            shares: p.shares,
            pending_yield: p.pending_yield,
            stake_time: ts_to_u64(&p.stake_time),
            last_claim_time: ts_to_u64(&p.last_claim_time),
            high_water_mark: p.high_water_mark,
            unlock_at: ts_to_u64(&p.unlock_at),
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
            execution_id: p.execution_id,
            vault_id: p.vault_id,
            pnl: p.pnl,
            success: p.success,
            error_message: p.error_message,
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
            vault_id: p.vault_id,
            current_il: p.current_il,
            avg_il_24h: p.avg_il_24h,
            timestamp: ts_to_u64(&p.timestamp),
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
    /// VA4 — 30d PnL in USD (e8 string, signed). Distinct from `pnl_24h`.
    pub pnl_30d_usd: String,
    /// VA4 — Capacity-TVL snapshot in USD (e8 string) at last score refresh.
    pub tvl_usd: String,
}

impl From<proto::VaultHealth> for VaultHealth {
    fn from(p: proto::VaultHealth) -> Self {
        Self {
            vault_id: p.vault_id,
            health_score: p.health_score,
            apy_bps: p.apy_bps,
            pnl_24h: p.pnl_24h,
            risk_score: p.risk_score,
            timestamp: ts_to_u64(&p.timestamp),
            pnl_30d_usd: p.pnl_30d_usd,
            tvl_usd: p.tvl_usd,
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
    /// Default-OFF gate for the periodic deterministic collateral-conservation audit.
    pub collateral_audit_enabled: bool,
    /// Addresses authorized to submit `MsgAuditCollateral`.
    pub authorized_audit_signers: Vec<String>,
    /// Default-OFF gate for strategy execution.
    pub enable_strategy_execution: bool,
    /// Default-OFF gate for the withdrawal queue / scan path.
    pub enable_withdrawal_queue: bool,
    /// Addresses authorized to submit withdrawal-queue scan messages.
    pub authorized_withdrawal_signers: Vec<String>,
    /// Max withdrawals processed per `MsgProcessWithdrawals` scan.
    pub max_withdrawals_per_scan: u64,
    /// Default-OFF gate for per-depositor high-water marks (spec §7 / G9). While
    /// false the performance fee crystallizes against the vault-global mark.
    pub per_depositor_hwm_enabled: bool,
    /// VB2 (spec §7 / G9) — internal performance-fee split: the leader payout
    /// share (bps). The treasury receives the remainder (`10000 − leader −
    /// reserve`); 0 ⇒ treasury takes the full fee (byte-identical pre-VB2).
    pub perf_fee_leader_bps: u32,
    /// The insurance/MLP-reserve share of the performance fee (bps).
    pub perf_fee_reserve_bps: u32,
    /// VB3 (spec §8 / G3) — leader skin-in-the-game floor (bps of the vault the
    /// leader must retain as first-loss capital). 0 ⇒ the withdrawal clamp +
    /// deposit soft-close are no-ops (byte-identical pre-VB3).
    pub min_leader_skin_bps: u32,
    /// VB3 (spec §8 / G3) — anti-spam vault creation fee in MORM sat, swept to
    /// the treasury at create. 0 ⇒ no fee.
    pub vault_creation_fee_sat: u64,
    /// VB4 (spec §6 / §14 G1) — per-deposit minimum-hold lock in seconds, baked
    /// into each new deposit's unlock time. 0 ⇒ no lock (instant-redeemable).
    pub lockup_secs: u64,
    /// VB4 (spec §6 / §14 G1) — redemption notice/queue window in seconds; a
    /// queued redemption is serviceable only after `requested_at + this`. 0 ⇒
    /// instantly serviceable.
    pub redemption_notice_secs: u64,
    /// VB7 (spec §5) — default-OFF gate for the buffer-floor auto-allocation
    /// cadence (`MsgAllocateBuffer`).
    pub enable_auto_allocation: bool,
    /// Addresses authorized to submit `MsgAllocateBuffer`. Empty = permissionless.
    pub authorized_allocation_signers: Vec<String>,
    /// VB8 (spec §14 / G5) — max age (ms) of a committed mark before NAV
    /// rejects it. 0 ⇒ staleness check off.
    pub max_mark_staleness_ms: u64,
    /// VB8 — Perp (bucket) illiquidity haircut in bps. 0 ⇒ no haircut.
    pub perp_haircut_bps: u32,
    /// VB8 — CLMM illiquidity haircut in bps. 0 ⇒ no haircut.
    pub clmm_haircut_bps: u32,
    /// G5 CLMM leg — deadband (bps) for the CLMM NAV basis-deviation haircut.
    /// Ignored unless `clmm_max_basis_haircut_bps > 0`.
    pub clmm_basis_tolerance_bps: u32,
    /// G5 CLMM leg — default-OFF cap (bps) for the CLMM NAV basis-deviation
    /// haircut. 0 ⇒ disarmed (the CLMM leg keeps only `clmm_haircut_bps`).
    pub clmm_max_basis_haircut_bps: u32,
    /// VA3 — per-tier SpotToken illiquidity haircut in bps. Index =
    /// `SpotAssetTier.tier` ordinal. Empty ⇒ no spot haircut.
    pub spot_haircut_bps_by_tier: Vec<u32>,
    /// VA4 — default-OFF gate for the analyst-score refresh cadence.
    pub enable_analyst_scoring: bool,
    /// Addresses authorized to submit `MsgRefreshVaultScore`. Empty = permissionless.
    pub authorized_score_signers: Vec<String>,
    /// VA4 — share-price sampling epoch length in blocks. Required (> 0) when
    /// `enable_analyst_scoring` is true.
    pub score_sample_interval_blocks: u64,
    /// VA5 — designated MLP protocol vault id. Empty ⇒ no MLP registered.
    pub mlp_backstop_vault_id: String,
    /// VA1 — default-OFF gate: deposits require a valid VC for the depositor.
    pub require_deposit_credential: bool,
    /// VA1 — default-OFF gate: deposits require the depositor identity Active.
    pub require_depositor_active: bool,
    /// D10 — default-OFF gate for the periodic performance-fee crystallization
    /// cadence (`MsgCrystallizeFee`). While false, fees crystallize only on
    /// redemption (byte-identical to pre-D10).
    pub enable_fee_crystallization: bool,
    /// Addresses authorized to submit `MsgCrystallizeFee`. Empty = permissionless.
    pub authorized_crystallize_signers: Vec<String>,
    /// D10 — crystallization epoch length in blocks. Required (> 0) when
    /// `enable_fee_crystallization` is true.
    pub fee_crystallization_interval_blocks: u64,
    /// D9 — default-OFF gate for forced position unwind on matured redemptions.
    pub enable_forced_unwind: bool,
    /// D9 — redeemer-borne exit fee (bps of forced-native). Validated ≤ 10000.
    pub unwind_exit_fee_bps: u32,
    /// VB9 (spec §7 / §12 gate #4) — default-OFF gate for the create-time
    /// fee-preset validation. Disarmed ⇒ legacy fee seed (byte-identical).
    pub enable_fee_presets: bool,
    /// VB9 — per-preset fee envelopes. Must contain a STANDARD entry when
    /// `enable_fee_presets` is true.
    pub fee_preset_bounds: Vec<FeePresetBound>,
    /// VB9 — governance allowlist of agents eligible to create a PREMIUM vault.
    /// Strict membership: an empty list locks Premium for everyone.
    pub authorized_premium_agents: Vec<String>,
    /// D9 CLMM extension — default-OFF gate for forced CLMM undeploy on matured
    /// redemptions. Reuses `unwind_exit_fee_bps` for the redeemer-borne exit fee.
    pub enable_forced_clmm_undeploy: bool,
    /// D6 (spec §14 G6) — manager-silence threshold in seconds. Required (> 0)
    /// when `enable_dead_man_switch` is true; `0` ⇒ disarmed.
    pub dead_man_switch_secs: u64,
    /// D6 — default-OFF gate for the dead-man-switch auto-pause sweep cadence
    /// (`MsgSweepDeadVault`) and the manager-activity stamp. Byte-identical to
    /// pre-D6 while false.
    pub enable_dead_man_switch: bool,
    /// D6 — addresses authorized to submit `MsgSweepDeadVault`. Empty =
    /// permissionless (bounded by the default-OFF gate).
    pub authorized_dead_man_signers: Vec<String>,
    /// VA3 producer — default-OFF gate for the manager-driven spot-acquisition
    /// path (`MsgAcquireSpot`). The reduce-only `MsgDisposeSpot` exit is never
    /// gated by this flag. Byte-identical to pre-VA3-producer while false.
    pub enable_spot_acquisition: bool,
    /// D9 spot leg — default-OFF gate for forced spot liquidation on matured
    /// redemptions. While false, held SpotToken is reachable only via a manager
    /// `MsgDisposeSpot`. Armed independently of the D9 perp/CLMM legs.
    pub enable_forced_spot_unwind: bool,
    /// D9 spot leg — max slippage (bps) the forced spot-liquidation floor
    /// tolerates versus the committed spot mark. Required (0 < bps <= 10000)
    /// when `enable_forced_spot_unwind`.
    pub forced_spot_max_slippage_bps: u32,
    /// VB7 spot auto-allocation — max slippage (bps) the buffer-floor spot
    /// acquisition floor tolerates versus the committed spot mark. `0` ⇒ spot
    /// auto-allocation disarmed (SPOT_TOKEN targets fail-safe-skipped).
    pub auto_alloc_spot_max_slippage_bps: u32,
    /// D4 in-kind redemption — governance master gate for redeemer-elected
    /// in-kind spot payout. `false` ⇒ all redemptions settle-to-base
    /// (byte-identical); a `MsgWithdrawFromVault { in_kind: true }` then falls
    /// back to settle-to-base.
    pub enable_in_kind_redemption: bool,
}

impl From<proto::Params> for VaultParams {
    fn from(p: proto::Params) -> Self {
        Self {
            max_vaults_per_agent: p.max_vaults_per_agent,
            min_initial_stake_usd: p.min_initial_stake_usd,
            max_strategy_complexity: p.max_strategy_complexity,
            treasury_cut_bps: p.treasury_cut_bps,
            last_updated: ts_to_u64(&p.last_updated),
            collateral_audit_enabled: p.collateral_audit_enabled,
            authorized_audit_signers: p.authorized_audit_signers,
            enable_strategy_execution: p.enable_strategy_execution,
            enable_withdrawal_queue: p.enable_withdrawal_queue,
            authorized_withdrawal_signers: p.authorized_withdrawal_signers,
            max_withdrawals_per_scan: p.max_withdrawals_per_scan,
            per_depositor_hwm_enabled: p.per_depositor_hwm_enabled,
            perf_fee_leader_bps: p.perf_fee_leader_bps,
            perf_fee_reserve_bps: p.perf_fee_reserve_bps,
            min_leader_skin_bps: p.min_leader_skin_bps,
            vault_creation_fee_sat: p.vault_creation_fee_sat,
            lockup_secs: p.lockup_secs,
            redemption_notice_secs: p.redemption_notice_secs,
            enable_auto_allocation: p.enable_auto_allocation,
            authorized_allocation_signers: p.authorized_allocation_signers,
            max_mark_staleness_ms: p.max_mark_staleness_ms,
            perp_haircut_bps: p.perp_haircut_bps,
            clmm_haircut_bps: p.clmm_haircut_bps,
            clmm_basis_tolerance_bps: p.clmm_basis_tolerance_bps,
            clmm_max_basis_haircut_bps: p.clmm_max_basis_haircut_bps,
            spot_haircut_bps_by_tier: p.spot_haircut_bps_by_tier,
            enable_analyst_scoring: p.enable_analyst_scoring,
            authorized_score_signers: p.authorized_score_signers,
            score_sample_interval_blocks: p.score_sample_interval_blocks,
            mlp_backstop_vault_id: p.mlp_backstop_vault_id,
            require_deposit_credential: p.require_deposit_credential,
            require_depositor_active: p.require_depositor_active,
            enable_fee_crystallization: p.enable_fee_crystallization,
            authorized_crystallize_signers: p.authorized_crystallize_signers,
            fee_crystallization_interval_blocks: p.fee_crystallization_interval_blocks,
            enable_forced_unwind: p.enable_forced_unwind,
            unwind_exit_fee_bps: p.unwind_exit_fee_bps,
            enable_fee_presets: p.enable_fee_presets,
            fee_preset_bounds: p
                .fee_preset_bounds
                .into_iter()
                .map(FeePresetBound::from)
                .collect(),
            authorized_premium_agents: p.authorized_premium_agents,
            enable_forced_clmm_undeploy: p.enable_forced_clmm_undeploy,
            dead_man_switch_secs: p.dead_man_switch_secs,
            enable_dead_man_switch: p.enable_dead_man_switch,
            authorized_dead_man_signers: p.authorized_dead_man_signers,
            enable_spot_acquisition: p.enable_spot_acquisition,
            enable_forced_spot_unwind: p.enable_forced_spot_unwind,
            forced_spot_max_slippage_bps: p.forced_spot_max_slippage_bps,
            auto_alloc_spot_max_slippage_bps: p.auto_alloc_spot_max_slippage_bps,
            enable_in_kind_redemption: p.enable_in_kind_redemption,
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
            collateral_audit_enabled: p.collateral_audit_enabled,
            authorized_audit_signers: p.authorized_audit_signers,
            enable_strategy_execution: p.enable_strategy_execution,
            enable_withdrawal_queue: p.enable_withdrawal_queue,
            authorized_withdrawal_signers: p.authorized_withdrawal_signers,
            max_withdrawals_per_scan: p.max_withdrawals_per_scan,
            per_depositor_hwm_enabled: p.per_depositor_hwm_enabled,
            perf_fee_leader_bps: p.perf_fee_leader_bps,
            perf_fee_reserve_bps: p.perf_fee_reserve_bps,
            min_leader_skin_bps: p.min_leader_skin_bps,
            vault_creation_fee_sat: p.vault_creation_fee_sat,
            lockup_secs: p.lockup_secs,
            redemption_notice_secs: p.redemption_notice_secs,
            enable_auto_allocation: p.enable_auto_allocation,
            authorized_allocation_signers: p.authorized_allocation_signers,
            max_mark_staleness_ms: p.max_mark_staleness_ms,
            perp_haircut_bps: p.perp_haircut_bps,
            clmm_haircut_bps: p.clmm_haircut_bps,
            clmm_basis_tolerance_bps: p.clmm_basis_tolerance_bps,
            clmm_max_basis_haircut_bps: p.clmm_max_basis_haircut_bps,
            spot_haircut_bps_by_tier: p.spot_haircut_bps_by_tier,
            enable_analyst_scoring: p.enable_analyst_scoring,
            authorized_score_signers: p.authorized_score_signers,
            score_sample_interval_blocks: p.score_sample_interval_blocks,
            mlp_backstop_vault_id: p.mlp_backstop_vault_id,
            require_deposit_credential: p.require_deposit_credential,
            require_depositor_active: p.require_depositor_active,
            enable_fee_crystallization: p.enable_fee_crystallization,
            authorized_crystallize_signers: p.authorized_crystallize_signers,
            fee_crystallization_interval_blocks: p.fee_crystallization_interval_blocks,
            enable_forced_unwind: p.enable_forced_unwind,
            unwind_exit_fee_bps: p.unwind_exit_fee_bps,
            enable_fee_presets: p.enable_fee_presets,
            fee_preset_bounds: p
                .fee_preset_bounds
                .into_iter()
                .map(Into::into)
                .collect(),
            authorized_premium_agents: p.authorized_premium_agents,
            enable_forced_clmm_undeploy: p.enable_forced_clmm_undeploy,
            dead_man_switch_secs: p.dead_man_switch_secs,
            enable_dead_man_switch: p.enable_dead_man_switch,
            authorized_dead_man_signers: p.authorized_dead_man_signers,
            enable_spot_acquisition: p.enable_spot_acquisition,
            enable_forced_spot_unwind: p.enable_forced_spot_unwind,
            forced_spot_max_slippage_bps: p.forced_spot_max_slippage_bps,
            auto_alloc_spot_max_slippage_bps: p.auto_alloc_spot_max_slippage_bps,
            enable_in_kind_redemption: p.enable_in_kind_redemption,
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
            proto::vault_update::Event::VaultUpdate(v) => {
                VaultUpdateEvent::VaultUpdate(alloc::boxed::Box::new(v.into()))
            }
            proto::vault_update::Event::ExecutionUpdate(e) => {
                VaultUpdateEvent::ExecutionUpdate(e.into())
            }
            proto::vault_update::Event::IlUpdate(il) => VaultUpdateEvent::IlUpdate(il.into()),
            proto::vault_update::Event::HealthUpdate(h) => VaultUpdateEvent::HealthUpdate(h.into()),
        });
        Self {
            event_type: p.event_type,
            event,
            timestamp: ts_to_u64(&p.timestamp),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn vault_type_roundtrip() {
        for t in [VaultType::Custom, VaultType::Yield, VaultType::Protocol] {
            assert_eq!(t, VaultType::from(i32::from(t)));
        }
        assert_eq!(VaultType::Unspecified, VaultType::from(99));
    }

    #[test]
    fn bucket_mode_roundtrip() {
        for m in [BucketMode::Cross, BucketMode::Isolated, BucketMode::Unspecified] {
            assert_eq!(m, BucketMode::from(i32::from(m)));
        }
        assert_eq!(BucketMode::Unspecified, BucketMode::from(99));
    }

    #[test]
    fn vault_bucket_roundtrip() {
        let b = VaultBucket {
            bucket_id: "bkt-1".into(),
            collateral_asset_index: 3,
            deployed_assets: "1234".into(),
            mode: BucketMode::Isolated,
        };
        let p: proto::VaultBucket = b.clone().into();
        assert_eq!(VaultBucket::from(p), b);
    }

    #[test]
    fn vault_status_roundtrip() {
        for s in [
            VaultStatus::Active,
            VaultStatus::Paused,
            VaultStatus::Executing,
            VaultStatus::Cooldown,
            VaultStatus::Liquidating,
        ] {
            assert_eq!(s, VaultStatus::from(i32::from(s)));
        }
    }

    #[test]
    fn vault_from_proto() {
        let p = proto::Vault {
            vault_id: "v1".into(),
            agent_id: "a1".into(),
            r#type: 1,
            name: "Test".into(),
            description: "Desc".into(),
            asset: Some(morpheum_proto::primitives::v1::Asset {
                asset_index: 1,
                symbol: "MORM".into(),
                ..Default::default()
            }),
            total_assets: "1000".into(),
            available_assets: "800".into(),
            reserved_assets: "200".into(),
            status: 1,
            created_at: None,
            updated_at: None,
            strategy_hash: "abc".into(),
            health_score: "9500".into(),
            pnl_30d_usd: "100".into(),
            apy_bps: "1200".into(),
            vc_claim_hash: vec![],
            copy_count: "5".into(),
            total_shares: "1000".into(),
            bucket_id: "bucket-v1".into(),
            collateral_asset_index: 1,
            deployed_assets: "100".into(),
            high_water_mark: "100000000".into(),
            clmm_pool_id: "pool-1".into(),
            clmm_position_id: "position-1".into(),
            clmm_collateral_token_index: 1,
            clmm_deployed_assets: "50".into(),
            strategy_type: 0,
            leader_payout_address: "a1".into(),
            performance_fee_bps: 500,
            management_fee_bps: 0,
            leader_custody_address: "a1".into(),
            deposit_capacity_native: "0".into(),
            soft_closed: false,
            mandate: None,
            allocation_policy: None,
            owner_agent_hash: "agent-hash-1".into(),
            last_fee_crystallization_epoch: 7,
            buckets: vec![
                proto::VaultBucket {
                    bucket_id: "bucket-v1".into(),
                    collateral_asset_index: 1,
                    deployed_assets: "60".into(),
                    mode: 1,
                },
                proto::VaultBucket {
                    bucket_id: "bucket-v2".into(),
                    collateral_asset_index: 1,
                    deployed_assets: "40".into(),
                    mode: 2,
                },
            ],
            fee_preset: 2,
            last_manager_activity_secs: 1_700_000_000,
        };
        let v: Vault = p.into();
        assert_eq!(v.vault_type, VaultType::Custom);
        assert_eq!(v.fee_preset, VaultFeePreset::Premium);
        assert_eq!(v.last_manager_activity_secs, 1_700_000_000);
        assert_eq!(v.buckets.len(), 2);
        assert_eq!(v.buckets[0].bucket_id, "bucket-v1");
        assert_eq!(v.buckets[0].mode, BucketMode::Cross);
        assert_eq!(v.buckets[1].bucket_id, "bucket-v2");
        assert_eq!(v.buckets[1].mode, BucketMode::Isolated);
        assert_eq!(v.asset_symbol, "MORM");
        assert_eq!(v.total_assets, "1000");
        assert_eq!(v.total_shares, "1000");
        assert_eq!(v.bucket_id, "bucket-v1");
        assert_eq!(v.clmm_position_id, "position-1");
        assert_eq!(v.leader_payout_address, "a1");
        assert_eq!(v.performance_fee_bps, 500);
        assert_eq!(v.leader_custody_address, "a1");
        assert_eq!(v.deposit_capacity_native, "0");
        assert!(!v.soft_closed);
        assert!(v.mandate.allowed_markets.is_empty());
        assert_eq!(v.mandate.max_leverage, 0);
        assert_eq!(v.allocation_policy.cash_buffer_floor_bps, 0);
        assert!(v.allocation_policy.targets.is_empty());
        assert_eq!(v.owner_agent_hash, "agent-hash-1");
        assert_eq!(v.last_fee_crystallization_epoch, 7);
    }

    #[test]
    fn vault_stream_event_from_proto() {
        let p = proto::VaultUpdate {
            event_type: "health_updated".into(),
            event: Some(proto::vault_update::Event::HealthUpdate(
                proto::VaultHealth {
                    vault_id: "v1".into(),
                    health_score: "9500".into(),
                    apy_bps: "1200".into(),
                    pnl_24h: "50".into(),
                    risk_score: "300".into(),
                    timestamp: None,
                    pnl_30d_usd: "100".into(),
                    tvl_usd: "1000".into(),
                },
            )),
            timestamp: None,
        };
        let e = VaultStreamEvent::from_proto(p);
        assert_eq!(e.event_type, "health_updated");
        assert!(matches!(e.event, Some(VaultUpdateEvent::HealthUpdate(_))));
    }

    #[test]
    fn params_roundtrip() {
        let p = VaultParams {
            max_vaults_per_agent: 100,
            min_initial_stake_usd: 100,
            max_strategy_complexity: 50,
            treasury_cut_bps: 500,
            last_updated: 0,
            collateral_audit_enabled: true,
            authorized_audit_signers: alloc::vec!["morpheum1audit".into()],
            enable_strategy_execution: true,
            enable_withdrawal_queue: true,
            authorized_withdrawal_signers: alloc::vec!["morpheum1withdraw".into()],
            max_withdrawals_per_scan: 25,
            per_depositor_hwm_enabled: true,
            perf_fee_leader_bps: 7_000,
            perf_fee_reserve_bps: 1_000,
            min_leader_skin_bps: 500,
            vault_creation_fee_sat: 1_000_000,
            lockup_secs: 86_400,
            redemption_notice_secs: 43_200,
            enable_auto_allocation: true,
            authorized_allocation_signers: alloc::vec!["morpheum1alloc".into()],
            max_mark_staleness_ms: 5_000,
            perp_haircut_bps: 100,
            clmm_haircut_bps: 250,
            clmm_basis_tolerance_bps: 100,
            clmm_max_basis_haircut_bps: 300,
            spot_haircut_bps_by_tier: alloc::vec![50, 150],
            enable_analyst_scoring: true,
            authorized_score_signers: alloc::vec!["morpheum1score".into()],
            score_sample_interval_blocks: 100,
            mlp_backstop_vault_id: "mlp-v1".into(),
            require_deposit_credential: true,
            require_depositor_active: true,
            enable_fee_crystallization: true,
            authorized_crystallize_signers: alloc::vec!["morpheum1fee".into()],
            fee_crystallization_interval_blocks: 43_200,
            enable_forced_unwind: true,
            unwind_exit_fee_bps: 50,
            enable_fee_presets: true,
            fee_preset_bounds: alloc::vec![
                FeePresetBound {
                    preset: VaultFeePreset::Standard,
                    min_perf_bps: 500,
                    max_perf_bps: 1_500,
                    default_perf_bps: 1_000,
                    max_mgmt_bps: 0,
                },
                FeePresetBound {
                    preset: VaultFeePreset::Premium,
                    min_perf_bps: 1_000,
                    max_perf_bps: 2_500,
                    default_perf_bps: 1_500,
                    max_mgmt_bps: 100,
                },
            ],
            authorized_premium_agents: alloc::vec!["morpheum1premium".into()],
            enable_forced_clmm_undeploy: true,
            dead_man_switch_secs: 604_800,
            enable_dead_man_switch: true,
            authorized_dead_man_signers: alloc::vec!["morpheum1deadman".into()],
            enable_spot_acquisition: true,
            enable_forced_spot_unwind: true,
            forced_spot_max_slippage_bps: 250,
            auto_alloc_spot_max_slippage_bps: 150,
            enable_in_kind_redemption: true,
        };
        let proto_p: proto::Params = p.clone().into();
        let p2: VaultParams = proto_p.into();
        assert_eq!(p, p2);
    }

    #[test]
    fn vault_fee_preset_roundtrip() {
        for pre in [
            VaultFeePreset::Standard,
            VaultFeePreset::Premium,
            VaultFeePreset::Protocol,
            VaultFeePreset::Unspecified,
        ] {
            assert_eq!(pre, VaultFeePreset::from(i32::from(pre)));
        }
        assert_eq!(VaultFeePreset::Unspecified, VaultFeePreset::from(99));
    }

    #[test]
    fn fee_preset_bound_roundtrip() {
        let b = FeePresetBound {
            preset: VaultFeePreset::Premium,
            min_perf_bps: 1_000,
            max_perf_bps: 2_500,
            default_perf_bps: 1_500,
            max_mgmt_bps: 100,
        };
        let p: proto::FeePresetBound = b.into();
        assert_eq!(FeePresetBound::from(p), b);
    }
}
