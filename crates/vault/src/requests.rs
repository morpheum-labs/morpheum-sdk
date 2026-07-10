//! Request wrappers for the vault module.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::vault::v1 as proto;

use crate::types::{VaultFeePreset, VaultParams, VaultStatus, VaultType};

// ====================== TRANSACTION REQUESTS ======================

/// Create a new vault.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateVaultRequest {
    pub vault_type: VaultType,
    pub name: String,
    pub description: String,
    pub asset_index: u64,
    pub initial_assets: String,
    pub strategy_goal: String,
    /// VB9 (spec §7 / §12 gate #4) — requested fee preset. `Unspecified` (the
    /// default) is inert when the preset gate is disarmed and coalesces to
    /// `Standard` when armed.
    pub fee_preset: VaultFeePreset,
    /// VB9 — requested performance fee (bps). `0` resolves to the preset default.
    pub performance_fee_bps: u32,
    /// VB9 — requested management fee (bps). Must be within the preset ceiling.
    pub management_fee_bps: u32,
}

impl CreateVaultRequest {
    pub fn new(
        vault_type: VaultType,
        name: impl Into<String>,
        asset_index: u64,
        initial_assets: impl Into<String>,
    ) -> Self {
        Self {
            vault_type,
            name: name.into(),
            description: String::new(),
            asset_index,
            initial_assets: initial_assets.into(),
            strategy_goal: String::new(),
            fee_preset: VaultFeePreset::Unspecified,
            performance_fee_bps: 0,
            management_fee_bps: 0,
        }
    }

    /// VB9 — select the fee preset and rates (`0` performance ⇒ preset default).
    #[must_use]
    pub fn with_fee(
        mut self,
        preset: VaultFeePreset,
        performance_fee_bps: u32,
        management_fee_bps: u32,
    ) -> Self {
        self.fee_preset = preset;
        self.performance_fee_bps = performance_fee_bps;
        self.management_fee_bps = management_fee_bps;
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgCreateVault {
            r#type: i32::from(self.vault_type),
            name: self.name.clone(),
            description: self.description.clone(),
            asset: Some(morpheum_proto::primitives::v1::Asset {
                asset_index: self.asset_index,
                ..Default::default()
            }),
            initial_assets: self.initial_assets.clone(),
            strategy_goal: self.strategy_goal.clone(),
            timestamp: None,
            creator_external_address: None,
            creator_chain_type: None,
            // Left UNSPECIFIED; the vault module coalesces it to the
            // MARKET_MAKING default at create (spec §12).
            strategy_type: 0,
            // VB9 (spec §7 / §12 gate #4) — fee preset selection. Inert when the
            // `enable_fee_presets` gate is disarmed (create uses the legacy seed).
            fee_preset: i32::from(self.fee_preset),
            performance_fee_bps: self.performance_fee_bps,
            management_fee_bps: self.management_fee_bps,
        };
        ProtoAny {
            type_url: "/vault.v1.MsgCreateVault".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Update vault parameters (owner or governance).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateVaultParamsRequest {
    pub vault_id: String,
    pub min_stake: String,
    pub max_stake: String,
    pub new_description: String,
    /// VB5 (spec §14 G4) — hard deposit capacity. `None` ⇒ leave unchanged;
    /// `Some("0")` clears the cap (uncapped).
    pub deposit_capacity_native: Option<String>,
    /// VB5 (spec §14 G4) — manager soft-close toggle. `None` ⇒ leave unchanged.
    pub soft_closed: Option<bool>,
    /// VB6 (spec P7 / §2) — replace-as-unit mandate. `None` ⇒ leave unchanged;
    /// `Some(...)` must be a tightening of the current mandate.
    pub mandate: Option<crate::types::VaultMandate>,
    /// VB7 (spec §5) — replace-as-unit allocation policy. `None` ⇒ leave unchanged.
    pub allocation_policy: Option<crate::types::AllocationPolicy>,
}

impl UpdateVaultParamsRequest {
    pub fn new(vault_id: impl Into<String>) -> Self {
        Self {
            vault_id: vault_id.into(),
            min_stake: String::new(),
            max_stake: String::new(),
            new_description: String::new(),
            deposit_capacity_native: None,
            soft_closed: None,
            mandate: None,
            allocation_policy: None,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateVaultParams {
            vault_id: self.vault_id.clone(),
            min_stake: self.min_stake.clone(),
            max_stake: self.max_stake.clone(),
            new_description: self.new_description.clone(),
            timestamp: None,
            updater_external_address: None,
            updater_chain_type: None,
            deposit_capacity_native: self.deposit_capacity_native.clone(),
            soft_closed: self.soft_closed,
            mandate: self.mandate.clone().map(Into::into),
            allocation_policy: self.allocation_policy.clone().map(Into::into),
        };
        ProtoAny {
            type_url: "/vault.v1.MsgUpdateVaultParams".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Execute a strategy manually.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecuteStrategyRequest {
    pub vault_id: String,
    pub strategy_params: String,
    /// D8 (spec §3) — target owned bucket. Empty ⇒ the vault's sole bucket
    /// (rejected when the vault owns more than one — specify explicitly).
    pub bucket_id: String,
}

impl ExecuteStrategyRequest {
    pub fn new(vault_id: impl Into<String>, strategy_params: impl Into<String>) -> Self {
        Self {
            vault_id: vault_id.into(),
            strategy_params: strategy_params.into(),
            bucket_id: String::new(),
        }
    }

    /// D8 — target a specific owned bucket.
    pub fn with_bucket(mut self, bucket_id: impl Into<String>) -> Self {
        self.bucket_id = bucket_id.into();
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgExecuteStrategy {
            vault_id: self.vault_id.clone(),
            strategy_params: self.strategy_params.clone(),
            timestamp: None,
            bucket_id: self.bucket_id.clone(),
        };
        ProtoAny {
            type_url: "/vault.v1.MsgExecuteStrategy".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// D8 (spec §3) — deploy idle principal into an owned margin bucket. Empty
/// `bucket_id` with `provision_new = false` targets the sole bucket (or
/// provisions the first). `provision_new = true` always provisions a NEW bucket
/// of `new_bucket_mode`.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeployToBucketRequest {
    pub vault_id: String,
    pub amount: String,
    pub bucket_id: String,
    pub provision_new: bool,
    pub new_bucket_mode: crate::types::BucketMode,
}

impl DeployToBucketRequest {
    pub fn new(vault_id: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            vault_id: vault_id.into(),
            amount: amount.into(),
            bucket_id: String::new(),
            provision_new: false,
            new_bucket_mode: crate::types::BucketMode::Unspecified,
        }
    }

    /// Target an existing owned bucket by id.
    pub fn with_bucket(mut self, bucket_id: impl Into<String>) -> Self {
        self.bucket_id = bucket_id.into();
        self
    }

    /// Provision a NEW owned bucket of `mode` and fund it.
    pub fn provision(mut self, mode: crate::types::BucketMode) -> Self {
        self.provision_new = true;
        self.new_bucket_mode = mode;
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgDeployToBucket {
            vault_id: self.vault_id.clone(),
            amount: self.amount.clone(),
            bucket_id: self.bucket_id.clone(),
            provision_new: self.provision_new,
            new_bucket_mode: i32::from(self.new_bucket_mode),
        };
        ProtoAny {
            type_url: "/vault.v1.MsgDeployToBucket".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// D8 (spec §3) — pull margin back from an owned bucket into idle principal.
/// Empty `bucket_id` targets the sole bucket (rejected when ambiguous).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UndeployFromBucketRequest {
    pub vault_id: String,
    pub amount: String,
    pub bucket_id: String,
}

impl UndeployFromBucketRequest {
    pub fn new(vault_id: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            vault_id: vault_id.into(),
            amount: amount.into(),
            bucket_id: String::new(),
        }
    }

    pub fn with_bucket(mut self, bucket_id: impl Into<String>) -> Self {
        self.bucket_id = bucket_id.into();
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUndeployFromBucket {
            vault_id: self.vault_id.clone(),
            amount: self.amount.clone(),
            bucket_id: self.bucket_id.clone(),
        };
        ProtoAny {
            type_url: "/vault.v1.MsgUndeployFromBucket".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// D8 (spec §3) — set per-(bucket, market) leverage on an owned bucket. Empty
/// `bucket_id` targets the sole bucket (rejected when ambiguous).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SetVaultLeverageRequest {
    pub vault_id: String,
    pub market_index: u64,
    pub leverage: u32,
    pub bucket_id: String,
}

impl SetVaultLeverageRequest {
    pub fn new(vault_id: impl Into<String>, market_index: u64, leverage: u32) -> Self {
        Self {
            vault_id: vault_id.into(),
            market_index,
            leverage,
            bucket_id: String::new(),
        }
    }

    pub fn with_bucket(mut self, bucket_id: impl Into<String>) -> Self {
        self.bucket_id = bucket_id.into();
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgSetVaultLeverage {
            vault_id: self.vault_id.clone(),
            market_index: self.market_index,
            leverage: self.leverage,
            bucket_id: self.bucket_id.clone(),
        };
        ProtoAny {
            type_url: "/vault.v1.MsgSetVaultLeverage".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// VA3 producer — acquire a mandate-whitelisted spot token by swapping `amount`
/// of idle base collateral into `asset_index` via the CLMM hybrid swap DIP. The
/// pool must pair exactly `(base, asset_index)` and a committed spot mark must
/// exist for the target. Gated by the default-OFF `enable_spot_acquisition` +
/// `enable_strategy_execution` params and the vault manager signer.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AcquireSpotRequest {
    pub vault_id: String,
    pub pool_id: String,
    pub asset_index: u32,
    pub amount: String,
    pub min_out: String,
}

impl AcquireSpotRequest {
    pub fn new(
        vault_id: impl Into<String>,
        pool_id: impl Into<String>,
        asset_index: u32,
        amount: impl Into<String>,
        min_out: impl Into<String>,
    ) -> Self {
        Self {
            vault_id: vault_id.into(),
            pool_id: pool_id.into(),
            asset_index,
            amount: amount.into(),
            min_out: min_out.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgAcquireSpot {
            vault_id: self.vault_id.clone(),
            pool_id: self.pool_id.clone(),
            asset_index: self.asset_index,
            amount: self.amount.clone(),
            min_out: self.min_out.clone(),
        };
        ProtoAny {
            type_url: "/vault.v1.MsgAcquireSpot".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// VA3 producer — dispose a held spot token by swapping `amount` of `asset_index`
/// back to base collateral via the CLMM hybrid swap DIP. The reduce-only exit is
/// gated only by `enable_strategy_execution` (never by `enable_spot_acquisition`
/// or the whitelist) so capital can always leave.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisposeSpotRequest {
    pub vault_id: String,
    pub pool_id: String,
    pub asset_index: u32,
    pub amount: String,
    pub min_out: String,
}

impl DisposeSpotRequest {
    pub fn new(
        vault_id: impl Into<String>,
        pool_id: impl Into<String>,
        asset_index: u32,
        amount: impl Into<String>,
        min_out: impl Into<String>,
    ) -> Self {
        Self {
            vault_id: vault_id.into(),
            pool_id: pool_id.into(),
            asset_index,
            amount: amount.into(),
            min_out: min_out.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgDisposeSpot {
            vault_id: self.vault_id.clone(),
            pool_id: self.pool_id.clone(),
            asset_index: self.asset_index,
            amount: self.amount.clone(),
            min_out: self.min_out.clone(),
        };
        ProtoAny {
            type_url: "/vault.v1.MsgDisposeSpot".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Pause vault operations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PauseVaultRequest {
    pub vault_id: String,
    pub reason: String,
}

impl PauseVaultRequest {
    pub fn new(vault_id: impl Into<String>, reason: impl Into<String>) -> Self {
        Self {
            vault_id: vault_id.into(),
            reason: reason.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgPauseVault {
            vault_id: self.vault_id.clone(),
            reason: self.reason.clone(),
            timestamp: None,
        };
        ProtoAny {
            type_url: "/vault.v1.MsgPauseVault".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Resume vault operations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResumeVaultRequest {
    pub vault_id: String,
}

impl ResumeVaultRequest {
    pub fn new(vault_id: impl Into<String>) -> Self {
        Self {
            vault_id: vault_id.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgResumeVault {
            vault_id: self.vault_id.clone(),
            timestamp: None,
        };
        ProtoAny {
            type_url: "/vault.v1.MsgResumeVault".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Deposit / stake into a vault.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DepositToVaultRequest {
    pub address: String,
    pub vault_id: String,
    pub asset_index: u64,
    pub amount: String,
}

impl DepositToVaultRequest {
    pub fn new(
        address: impl Into<String>,
        vault_id: impl Into<String>,
        asset_index: u64,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            vault_id: vault_id.into(),
            asset_index,
            amount: amount.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgDepositToVault {
            address: self.address.clone(),
            vault_id: self.vault_id.clone(),
            asset: Some(morpheum_proto::primitives::v1::Asset {
                asset_index: self.asset_index,
                ..Default::default()
            }),
            amount: self.amount.clone(),
            timestamp: None,
            external_address: None,
            chain_type: None,
        };
        ProtoAny {
            type_url: "/vault.v1.MsgDepositToVault".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Withdraw / unstake from a vault.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WithdrawFromVaultRequest {
    pub address: String,
    pub vault_id: String,
    pub asset_index: u64,
    pub shares: String,
}

impl WithdrawFromVaultRequest {
    pub fn new(
        address: impl Into<String>,
        vault_id: impl Into<String>,
        asset_index: u64,
        shares: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            vault_id: vault_id.into(),
            asset_index,
            shares: shares.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgWithdrawFromVault {
            address: self.address.clone(),
            vault_id: self.vault_id.clone(),
            asset: Some(morpheum_proto::primitives::v1::Asset {
                asset_index: self.asset_index,
                ..Default::default()
            }),
            shares: self.shares.clone(),
            timestamp: None,
            external_address: None,
        };
        ProtoAny {
            type_url: "/vault.v1.MsgWithdrawFromVault".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Claim accumulated yield.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClaimYieldRequest {
    pub address: String,
    pub vault_id: String,
}

impl ClaimYieldRequest {
    pub fn new(address: impl Into<String>, vault_id: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            vault_id: vault_id.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgClaimYield {
            address: self.address.clone(),
            vault_id: self.vault_id.clone(),
            timestamp: None,
            external_address: None,
        };
        ProtoAny {
            type_url: "/vault.v1.MsgClaimYield".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// VA4 — refresh one vault's analyst score (keeper cadence).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RefreshVaultScoreRequest {
    pub keeper: String,
    pub vault_id: String,
}

impl RefreshVaultScoreRequest {
    pub fn new(keeper: impl Into<String>, vault_id: impl Into<String>) -> Self {
        Self {
            keeper: keeper.into(),
            vault_id: vault_id.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgRefreshVaultScore {
            keeper: self.keeper.clone(),
            vault_id: self.vault_id.clone(),
        };
        ProtoAny {
            type_url: "/vault.v1.MsgRefreshVaultScore".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// D10 — crystallize one vault's performance fee (keeper cadence).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CrystallizeFeeRequest {
    pub keeper: String,
    pub vault_id: String,
}

impl CrystallizeFeeRequest {
    pub fn new(keeper: impl Into<String>, vault_id: impl Into<String>) -> Self {
        Self {
            keeper: keeper.into(),
            vault_id: vault_id.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgCrystallizeFee {
            keeper: self.keeper.clone(),
            vault_id: self.vault_id.clone(),
        };
        ProtoAny {
            type_url: "/vault.v1.MsgCrystallizeFee".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// VA5 — governance-only creation of a `VaultType::Protocol` MLP vault.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateProtocolVaultRequest {
    pub authority: String,
    pub name: String,
    pub description: String,
    pub asset_index: u64,
    pub protocol_leader: String,
    pub strategy_goal: String,
    pub strategy_type: i32,
}

impl CreateProtocolVaultRequest {
    pub fn new(
        authority: impl Into<String>,
        name: impl Into<String>,
        asset_index: u64,
        protocol_leader: impl Into<String>,
    ) -> Self {
        Self {
            authority: authority.into(),
            name: name.into(),
            description: String::new(),
            asset_index,
            protocol_leader: protocol_leader.into(),
            strategy_goal: String::new(),
            strategy_type: 0,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgCreateProtocolVault {
            authority: self.authority.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            asset: Some(morpheum_proto::primitives::v1::Asset {
                asset_index: self.asset_index,
                ..Default::default()
            }),
            protocol_leader: self.protocol_leader.clone(),
            strategy_goal: self.strategy_goal.clone(),
            strategy_type: self.strategy_type,
        };
        ProtoAny {
            type_url: "/vault.v1.MsgCreateProtocolVault".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// VA5 — governance recovery: Liquidating → Active.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClearVaultLiquidationRequest {
    pub authority: String,
    pub vault_id: String,
}

impl ClearVaultLiquidationRequest {
    pub fn new(authority: impl Into<String>, vault_id: impl Into<String>) -> Self {
        Self {
            authority: authority.into(),
            vault_id: vault_id.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgClearVaultLiquidation {
            authority: self.authority.clone(),
            vault_id: self.vault_id.clone(),
        };
        ProtoAny {
            type_url: "/vault.v1.MsgClearVaultLiquidation".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Update global vault module parameters (governance-only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    pub authority: String,
    pub params: VaultParams,
}

impl UpdateParamsRequest {
    pub fn new(authority: impl Into<String>, params: VaultParams) -> Self {
        Self {
            authority: authority.into(),
            params,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateParams {
            authority: self.authority.clone(),
            params: Some(self.params.clone().into()),
        };
        ProtoAny {
            type_url: "/vault.v1.MsgUpdateParams".into(),
            value: msg.encode_to_vec(),
        }
    }
}

// ====================== QUERY REQUESTS ======================

/// Get a specific vault by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetVaultRequest {
    pub vault_id: String,
}

impl GetVaultRequest {
    pub fn new(vault_id: impl Into<String>) -> Self {
        Self {
            vault_id: vault_id.into(),
        }
    }
}

impl From<GetVaultRequest> for proto::GetVaultRequest {
    fn from(r: GetVaultRequest) -> Self {
        Self {
            vault_id: r.vault_id,
        }
    }
}

/// List vaults with optional filters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListVaultsRequest {
    pub type_filter: Option<VaultType>,
    pub status_filter: Option<VaultStatus>,
    pub agent_id_filter: Option<String>,
}

impl ListVaultsRequest {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn vault_type(mut self, v: VaultType) -> Self {
        self.type_filter = Some(v);
        self
    }
    pub fn status(mut self, v: VaultStatus) -> Self {
        self.status_filter = Some(v);
        self
    }
    pub fn agent_id(mut self, v: impl Into<String>) -> Self {
        self.agent_id_filter = Some(v.into());
        self
    }
}

impl From<ListVaultsRequest> for proto::ListVaultsRequest {
    fn from(r: ListVaultsRequest) -> Self {
        Self {
            pagination: None,
            type_filter: r.type_filter.map(i32::from),
            status_filter: r.status_filter.map(i32::from),
            agent_id_filter: r.agent_id_filter,
        }
    }
}

/// Get vaults by agent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetVaultsByAgentRequest {
    pub agent_id: String,
}

impl GetVaultsByAgentRequest {
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
        }
    }
}

impl From<GetVaultsByAgentRequest> for proto::GetVaultsByAgentRequest {
    fn from(r: GetVaultsByAgentRequest) -> Self {
        Self {
            agent_id: r.agent_id,
            pagination: None,
        }
    }
}

/// Get vaults by type.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetVaultsByTypeRequest {
    pub vault_type: VaultType,
}

impl GetVaultsByTypeRequest {
    pub fn new(vault_type: VaultType) -> Self {
        Self { vault_type }
    }
}

impl From<GetVaultsByTypeRequest> for proto::GetVaultsByTypeRequest {
    fn from(r: GetVaultsByTypeRequest) -> Self {
        Self {
            r#type: i32::from(r.vault_type),
            pagination: None,
        }
    }
}

/// Get a user's stake in a specific vault.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetUserStakeRequest {
    pub address: String,
    pub vault_id: String,
}

impl GetUserStakeRequest {
    pub fn new(address: impl Into<String>, vault_id: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            vault_id: vault_id.into(),
        }
    }
}

impl From<GetUserStakeRequest> for proto::GetUserStakeRequest {
    fn from(r: GetUserStakeRequest) -> Self {
        Self {
            address: r.address,
            vault_id: r.vault_id,
        }
    }
}

/// List all stakes for a user across vaults.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListUserStakesRequest {
    pub address: String,
}

impl ListUserStakesRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }
}

impl From<ListUserStakesRequest> for proto::ListUserStakesRequest {
    fn from(r: ListUserStakesRequest) -> Self {
        Self {
            address: r.address,
            pagination: None,
        }
    }
}

/// Get strategy execution history for a vault.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetStrategyHistoryRequest {
    pub vault_id: String,
}

impl GetStrategyHistoryRequest {
    pub fn new(vault_id: impl Into<String>) -> Self {
        Self {
            vault_id: vault_id.into(),
        }
    }
}

impl From<GetStrategyHistoryRequest> for proto::GetStrategyHistoryRequest {
    fn from(r: GetStrategyHistoryRequest) -> Self {
        Self {
            vault_id: r.vault_id,
            pagination: None,
        }
    }
}

/// Get IL metrics for a vault.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetIlMetricsRequest {
    pub vault_id: String,
}

impl GetIlMetricsRequest {
    pub fn new(vault_id: impl Into<String>) -> Self {
        Self {
            vault_id: vault_id.into(),
        }
    }
}

impl From<GetIlMetricsRequest> for proto::GetIlMetricsRequest {
    fn from(r: GetIlMetricsRequest) -> Self {
        Self {
            vault_id: r.vault_id,
        }
    }
}

/// Get real-time vault health.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetVaultHealthRequest {
    pub vault_id: String,
}

impl GetVaultHealthRequest {
    pub fn new(vault_id: impl Into<String>) -> Self {
        Self {
            vault_id: vault_id.into(),
        }
    }
}

impl From<GetVaultHealthRequest> for proto::GetVaultHealthRequest {
    fn from(r: GetVaultHealthRequest) -> Self {
        Self {
            vault_id: r.vault_id,
        }
    }
}

/// Get top vaults ranked by metric.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetTopVaultsRequest {
    pub sort_by: String,
    pub type_filter: Option<VaultType>,
}

impl GetTopVaultsRequest {
    pub fn new(sort_by: impl Into<String>) -> Self {
        Self {
            sort_by: sort_by.into(),
            type_filter: None,
        }
    }
    pub fn vault_type(mut self, v: VaultType) -> Self {
        self.type_filter = Some(v);
        self
    }
}

impl From<GetTopVaultsRequest> for proto::GetTopVaultsRequest {
    fn from(r: GetTopVaultsRequest) -> Self {
        Self {
            sort_by: r.sort_by,
            pagination: None,
            type_filter: r.type_filter.map(i32::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vault_to_any() {
        let any =
            CreateVaultRequest::new(VaultType::Custom, "My Vault", 1, "1000").to_any();
        assert_eq!(any.type_url, "/vault.v1.MsgCreateVault");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn deposit_to_vault_to_any() {
        let any = DepositToVaultRequest::new("morph1user", "v1", 1, "500").to_any();
        assert_eq!(any.type_url, "/vault.v1.MsgDepositToVault");
    }

    #[test]
    fn withdraw_from_vault_to_any() {
        let any = WithdrawFromVaultRequest::new("morph1user", "v1", 1, "100").to_any();
        assert_eq!(any.type_url, "/vault.v1.MsgWithdrawFromVault");
    }

    #[test]
    fn acquire_spot_to_any() {
        let req = AcquireSpotRequest::new("v1", "0x1", 7, "1000", "950");
        let any = req.to_any();
        assert_eq!(any.type_url, "/vault.v1.MsgAcquireSpot");
        let decoded = proto::MsgAcquireSpot::decode(any.value.as_slice()).unwrap();
        assert_eq!(decoded.vault_id, "v1");
        assert_eq!(decoded.pool_id, "0x1");
        assert_eq!(decoded.asset_index, 7);
        assert_eq!(decoded.amount, "1000");
        assert_eq!(decoded.min_out, "950");
    }

    #[test]
    fn dispose_spot_to_any() {
        let req = DisposeSpotRequest::new("v1", "0x1", 7, "500", "480");
        let any = req.to_any();
        assert_eq!(any.type_url, "/vault.v1.MsgDisposeSpot");
        let decoded = proto::MsgDisposeSpot::decode(any.value.as_slice()).unwrap();
        assert_eq!(decoded.vault_id, "v1");
        assert_eq!(decoded.asset_index, 7);
        assert_eq!(decoded.amount, "500");
        assert_eq!(decoded.min_out, "480");
    }

    #[test]
    fn list_vaults_with_filters() {
        let p: proto::ListVaultsRequest = ListVaultsRequest::new()
            .vault_type(VaultType::Yield)
            .status(VaultStatus::Active)
            .into();
        assert_eq!(p.type_filter, Some(2));
        assert_eq!(p.status_filter, Some(1));
    }

    #[test]
    fn get_top_vaults_conversion() {
        let p: proto::GetTopVaultsRequest = GetTopVaultsRequest::new("apy")
            .vault_type(VaultType::Custom)
            .into();
        assert_eq!(p.sort_by, "apy");
        assert_eq!(p.type_filter, Some(1));
    }

    #[test]
    fn update_vault_params_capacity_roundtrip() {
        let any = UpdateVaultParamsRequest {
            vault_id: "v1".into(),
            min_stake: String::new(),
            max_stake: String::new(),
            new_description: String::new(),
            deposit_capacity_native: Some("5000".into()),
            soft_closed: Some(true),
            mandate: None,
            allocation_policy: None,
        }
        .to_any();
        assert_eq!(any.type_url, "/vault.v1.MsgUpdateVaultParams");
        let msg = proto::MsgUpdateVaultParams::decode(any.value.as_slice()).unwrap();
        assert_eq!(msg.deposit_capacity_native.as_deref(), Some("5000"));
        assert_eq!(msg.soft_closed, Some(true));
    }

    #[test]
    fn update_vault_params_mandate_roundtrip() {
        use crate::types::VaultMandate;
        let any = UpdateVaultParamsRequest {
            vault_id: "v1".into(),
            min_stake: String::new(),
            max_stake: String::new(),
            new_description: String::new(),
            deposit_capacity_native: None,
            soft_closed: None,
            mandate: Some(VaultMandate {
                allowed_markets: alloc::vec![1, 2],
                max_leverage: 5,
                allowed_assets: alloc::vec![],
            }),
            allocation_policy: None,
        }
        .to_any();
        assert_eq!(any.type_url, "/vault.v1.MsgUpdateVaultParams");
        let msg = proto::MsgUpdateVaultParams::decode(any.value.as_slice()).unwrap();
        let m = msg.mandate.expect("mandate present");
        assert_eq!(m.allowed_markets, alloc::vec![1, 2]);
        assert_eq!(m.max_leverage, 5);
    }

    #[test]
    fn update_vault_params_allocation_roundtrip() {
        use crate::types::{AllocationKind, AllocationPolicy, AllocationTarget};
        let any = UpdateVaultParamsRequest {
            vault_id: "v1".into(),
            min_stake: String::new(),
            max_stake: String::new(),
            new_description: String::new(),
            deposit_capacity_native: None,
            soft_closed: None,
            mandate: None,
            allocation_policy: Some(AllocationPolicy {
                cash_buffer_floor_bps: 2_000,
                deployment_ceiling_bps: 7_000,
                targets: alloc::vec![AllocationTarget {
                    kind: AllocationKind::Bucket,
                    target_weight_bps: 5_000,
                }],
            }),
        }
        .to_any();
        assert_eq!(any.type_url, "/vault.v1.MsgUpdateVaultParams");
        let msg = proto::MsgUpdateVaultParams::decode(any.value.as_slice()).unwrap();
        let p = msg.allocation_policy.expect("allocation_policy present");
        assert_eq!(p.cash_buffer_floor_bps, 2_000);
        assert_eq!(p.deployment_ceiling_bps, 7_000);
        assert_eq!(p.targets.len(), 1);
        assert_eq!(p.targets[0].target_weight_bps, 5_000);
        assert_eq!(p.targets[0].kind, 1);
    }
}
