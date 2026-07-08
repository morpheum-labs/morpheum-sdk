//! Request wrappers for the vault module.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::vault::v1 as proto;

use crate::types::{VaultParams, VaultStatus, VaultType};

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
        }
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
}

impl UpdateVaultParamsRequest {
    pub fn new(vault_id: impl Into<String>) -> Self {
        Self {
            vault_id: vault_id.into(),
            min_stake: String::new(),
            max_stake: String::new(),
            new_description: String::new(),
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
}

impl ExecuteStrategyRequest {
    pub fn new(vault_id: impl Into<String>, strategy_params: impl Into<String>) -> Self {
        Self {
            vault_id: vault_id.into(),
            strategy_params: strategy_params.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgExecuteStrategy {
            vault_id: self.vault_id.clone(),
            strategy_params: self.strategy_params.clone(),
            timestamp: None,
        };
        ProtoAny {
            type_url: "/vault.v1.MsgExecuteStrategy".into(),
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
}
