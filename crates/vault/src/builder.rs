//! Fluent builders for the vault module.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    ClaimYieldRequest, CreateVaultRequest, DepositToVaultRequest, ExecuteStrategyRequest,
    PauseVaultRequest, ResumeVaultRequest, UpdateParamsRequest, UpdateVaultParamsRequest,
    WithdrawFromVaultRequest,
};
use crate::types::{VaultParams, VaultType};

// ====================== CREATE VAULT ======================

pub struct CreateVaultBuilder {
    creator_address: Option<String>,
    vault_type: VaultType,
    name: Option<String>,
    description: String,
    asset_index: Option<u64>,
    initial_assets: Option<String>,
    strategy_goal: String,
    creator_signature: Vec<u8>,
}

impl CreateVaultBuilder {
    pub fn new() -> Self {
        Self {
            creator_address: None, vault_type: VaultType::Unspecified,
            name: None, description: String::new(), asset_index: None,
            initial_assets: None, strategy_goal: String::new(),
            creator_signature: Vec::new(),
        }
    }

    pub fn creator_address(mut self, v: impl Into<String>) -> Self { self.creator_address = Some(v.into()); self }
    pub fn vault_type(mut self, v: VaultType) -> Self { self.vault_type = v; self }
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = v.into(); self }
    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn initial_assets(mut self, v: impl Into<String>) -> Self { self.initial_assets = Some(v.into()); self }
    pub fn strategy_goal(mut self, v: impl Into<String>) -> Self { self.strategy_goal = v.into(); self }
    pub fn creator_signature(mut self, v: Vec<u8>) -> Self { self.creator_signature = v; self }

    pub fn build(self) -> Result<CreateVaultRequest, SdkError> {
        if self.vault_type == VaultType::Unspecified {
            return Err(SdkError::invalid_input("vault_type must be specified"));
        }
        let mut req = CreateVaultRequest::new(
            self.creator_address.ok_or_else(|| SdkError::invalid_input("creator_address is required"))?,
            self.vault_type,
            self.name.ok_or_else(|| SdkError::invalid_input("name is required"))?,
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.initial_assets.ok_or_else(|| SdkError::invalid_input("initial_assets is required"))?,
        );
        req.description = self.description;
        req.strategy_goal = self.strategy_goal;
        req.creator_signature = self.creator_signature;
        Ok(req)
    }
}

impl Default for CreateVaultBuilder {
    fn default() -> Self { Self::new() }
}

// ====================== UPDATE VAULT PARAMS ======================

pub struct UpdateVaultParamsBuilder {
    vault_id: Option<String>,
    min_stake: String,
    max_stake: String,
    new_description: String,
    updater_signature: Vec<u8>,
}

impl UpdateVaultParamsBuilder {
    pub fn new() -> Self {
        Self {
            vault_id: None, min_stake: String::new(), max_stake: String::new(),
            new_description: String::new(), updater_signature: Vec::new(),
        }
    }

    pub fn vault_id(mut self, v: impl Into<String>) -> Self { self.vault_id = Some(v.into()); self }
    pub fn min_stake(mut self, v: impl Into<String>) -> Self { self.min_stake = v.into(); self }
    pub fn max_stake(mut self, v: impl Into<String>) -> Self { self.max_stake = v.into(); self }
    pub fn new_description(mut self, v: impl Into<String>) -> Self { self.new_description = v.into(); self }
    pub fn updater_signature(mut self, v: Vec<u8>) -> Self { self.updater_signature = v; self }

    pub fn build(self) -> Result<UpdateVaultParamsRequest, SdkError> {
        let mut req = UpdateVaultParamsRequest::new(
            self.vault_id.ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
        );
        req.min_stake = self.min_stake;
        req.max_stake = self.max_stake;
        req.new_description = self.new_description;
        req.updater_signature = self.updater_signature;
        Ok(req)
    }
}

impl Default for UpdateVaultParamsBuilder {
    fn default() -> Self { Self::new() }
}

// ====================== EXECUTE STRATEGY ======================

pub struct ExecuteStrategyBuilder {
    vault_id: Option<String>,
    strategy_params: Option<String>,
    executor_signature: Vec<u8>,
}

impl ExecuteStrategyBuilder {
    pub fn new() -> Self { Self { vault_id: None, strategy_params: None, executor_signature: Vec::new() } }

    pub fn vault_id(mut self, v: impl Into<String>) -> Self { self.vault_id = Some(v.into()); self }
    pub fn strategy_params(mut self, v: impl Into<String>) -> Self { self.strategy_params = Some(v.into()); self }
    pub fn executor_signature(mut self, v: Vec<u8>) -> Self { self.executor_signature = v; self }

    pub fn build(self) -> Result<ExecuteStrategyRequest, SdkError> {
        let mut req = ExecuteStrategyRequest::new(
            self.vault_id.ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
            self.strategy_params.ok_or_else(|| SdkError::invalid_input("strategy_params is required"))?,
        );
        req.executor_signature = self.executor_signature;
        Ok(req)
    }
}

impl Default for ExecuteStrategyBuilder {
    fn default() -> Self { Self::new() }
}

// ====================== PAUSE / RESUME ======================

pub struct PauseVaultBuilder {
    vault_id: Option<String>,
    reason: Option<String>,
    pauser_signature: Vec<u8>,
}

impl PauseVaultBuilder {
    pub fn new() -> Self { Self { vault_id: None, reason: None, pauser_signature: Vec::new() } }

    pub fn vault_id(mut self, v: impl Into<String>) -> Self { self.vault_id = Some(v.into()); self }
    pub fn reason(mut self, v: impl Into<String>) -> Self { self.reason = Some(v.into()); self }
    pub fn pauser_signature(mut self, v: Vec<u8>) -> Self { self.pauser_signature = v; self }

    pub fn build(self) -> Result<PauseVaultRequest, SdkError> {
        let mut req = PauseVaultRequest::new(
            self.vault_id.ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
            self.reason.ok_or_else(|| SdkError::invalid_input("reason is required"))?,
        );
        req.pauser_signature = self.pauser_signature;
        Ok(req)
    }
}

impl Default for PauseVaultBuilder {
    fn default() -> Self { Self::new() }
}

pub struct ResumeVaultBuilder {
    vault_id: Option<String>,
    resumer_signature: Vec<u8>,
}

impl ResumeVaultBuilder {
    pub fn new() -> Self { Self { vault_id: None, resumer_signature: Vec::new() } }

    pub fn vault_id(mut self, v: impl Into<String>) -> Self { self.vault_id = Some(v.into()); self }
    pub fn resumer_signature(mut self, v: Vec<u8>) -> Self { self.resumer_signature = v; self }

    pub fn build(self) -> Result<ResumeVaultRequest, SdkError> {
        let mut req = ResumeVaultRequest::new(
            self.vault_id.ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
        );
        req.resumer_signature = self.resumer_signature;
        Ok(req)
    }
}

impl Default for ResumeVaultBuilder {
    fn default() -> Self { Self::new() }
}

// ====================== DEPOSIT / WITHDRAW / CLAIM ======================

pub struct DepositToVaultBuilder {
    address: Option<String>,
    vault_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    depositor_signature: Vec<u8>,
}

impl DepositToVaultBuilder {
    pub fn new() -> Self {
        Self { address: None, vault_id: None, asset_index: None, amount: None, depositor_signature: Vec::new() }
    }

    pub fn address(mut self, v: impl Into<String>) -> Self { self.address = Some(v.into()); self }
    pub fn vault_id(mut self, v: impl Into<String>) -> Self { self.vault_id = Some(v.into()); self }
    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn amount(mut self, v: impl Into<String>) -> Self { self.amount = Some(v.into()); self }
    pub fn depositor_signature(mut self, v: Vec<u8>) -> Self { self.depositor_signature = v; self }

    pub fn build(self) -> Result<DepositToVaultRequest, SdkError> {
        let mut req = DepositToVaultRequest::new(
            self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?,
            self.vault_id.ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.amount.ok_or_else(|| SdkError::invalid_input("amount is required"))?,
        );
        req.depositor_signature = self.depositor_signature;
        Ok(req)
    }
}

impl Default for DepositToVaultBuilder {
    fn default() -> Self { Self::new() }
}

pub struct WithdrawFromVaultBuilder {
    address: Option<String>,
    vault_id: Option<String>,
    asset_index: Option<u64>,
    shares: Option<String>,
    withdrawer_signature: Vec<u8>,
}

impl WithdrawFromVaultBuilder {
    pub fn new() -> Self {
        Self { address: None, vault_id: None, asset_index: None, shares: None, withdrawer_signature: Vec::new() }
    }

    pub fn address(mut self, v: impl Into<String>) -> Self { self.address = Some(v.into()); self }
    pub fn vault_id(mut self, v: impl Into<String>) -> Self { self.vault_id = Some(v.into()); self }
    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn shares(mut self, v: impl Into<String>) -> Self { self.shares = Some(v.into()); self }
    pub fn withdrawer_signature(mut self, v: Vec<u8>) -> Self { self.withdrawer_signature = v; self }

    pub fn build(self) -> Result<WithdrawFromVaultRequest, SdkError> {
        let mut req = WithdrawFromVaultRequest::new(
            self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?,
            self.vault_id.ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.shares.ok_or_else(|| SdkError::invalid_input("shares is required"))?,
        );
        req.withdrawer_signature = self.withdrawer_signature;
        Ok(req)
    }
}

impl Default for WithdrawFromVaultBuilder {
    fn default() -> Self { Self::new() }
}

pub struct ClaimYieldBuilder {
    address: Option<String>,
    vault_id: Option<String>,
    claimer_signature: Vec<u8>,
}

impl ClaimYieldBuilder {
    pub fn new() -> Self { Self { address: None, vault_id: None, claimer_signature: Vec::new() } }

    pub fn address(mut self, v: impl Into<String>) -> Self { self.address = Some(v.into()); self }
    pub fn vault_id(mut self, v: impl Into<String>) -> Self { self.vault_id = Some(v.into()); self }
    pub fn claimer_signature(mut self, v: Vec<u8>) -> Self { self.claimer_signature = v; self }

    pub fn build(self) -> Result<ClaimYieldRequest, SdkError> {
        let mut req = ClaimYieldRequest::new(
            self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?,
            self.vault_id.ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
        );
        req.claimer_signature = self.claimer_signature;
        Ok(req)
    }
}

impl Default for ClaimYieldBuilder {
    fn default() -> Self { Self::new() }
}

// ====================== UPDATE PARAMS (GOVERNANCE) ======================

pub struct UpdateModuleParamsBuilder {
    authority: Option<String>,
    params: Option<VaultParams>,
}

impl UpdateModuleParamsBuilder {
    pub fn new() -> Self { Self { authority: None, params: None } }

    pub fn authority(mut self, v: impl Into<String>) -> Self { self.authority = Some(v.into()); self }
    pub fn params(mut self, v: VaultParams) -> Self { self.params = Some(v); self }

    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        Ok(UpdateParamsRequest::new(
            self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?,
            self.params.ok_or_else(|| SdkError::invalid_input("params is required"))?,
        ))
    }
}

impl Default for UpdateModuleParamsBuilder {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vault_builder_works() {
        let req = CreateVaultBuilder::new()
            .creator_address("morph1user").vault_type(VaultType::Custom)
            .name("My Vault").asset_index(1).initial_assets("1000")
            .strategy_goal("maximize yield")
            .build().unwrap();
        assert_eq!(req.name, "My Vault");
        assert_eq!(req.strategy_goal, "maximize yield");
    }

    #[test]
    fn create_vault_requires_type() {
        assert!(CreateVaultBuilder::new()
            .creator_address("morph1user").name("V").asset_index(1).initial_assets("1000")
            .build().is_err());
    }

    #[test]
    fn deposit_builder_works() {
        let req = DepositToVaultBuilder::new()
            .address("morph1user").vault_id("v1").asset_index(1).amount("500")
            .build().unwrap();
        assert_eq!(req.amount, "500");
    }

    #[test]
    fn withdraw_builder_requires_shares() {
        assert!(WithdrawFromVaultBuilder::new()
            .address("morph1user").vault_id("v1").asset_index(1)
            .build().is_err());
    }

    #[test]
    fn execute_strategy_builder_works() {
        let req = ExecuteStrategyBuilder::new()
            .vault_id("v1").strategy_params(r#"{"action":"rebalance"}"#)
            .build().unwrap();
        assert_eq!(req.vault_id, "v1");
    }

    #[test]
    fn pause_resume_builders_work() {
        let pause = PauseVaultBuilder::new().vault_id("v1").reason("maintenance").build().unwrap();
        assert_eq!(pause.reason, "maintenance");

        let resume = ResumeVaultBuilder::new().vault_id("v1").build().unwrap();
        assert_eq!(resume.vault_id, "v1");
    }

    #[test]
    fn claim_yield_builder_works() {
        let req = ClaimYieldBuilder::new().address("morph1user").vault_id("v1").build().unwrap();
        assert_eq!(req.vault_id, "v1");
    }

    #[test]
    fn update_module_params_validation() {
        assert!(UpdateModuleParamsBuilder::new().build().is_err());
    }
}
