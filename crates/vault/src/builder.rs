//! Fluent builders for the vault module.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    ClaimYieldRequest, CreateVaultRequest, DepositToVaultRequest, ExecuteStrategyRequest,
    PauseVaultRequest, ResumeVaultRequest, UpdateParamsRequest, UpdateVaultParamsRequest,
    WithdrawFromVaultRequest,
};
use crate::types::{VaultParams, VaultType};

// ====================== CREATE VAULT ======================

pub struct CreateVaultBuilder {
    vault_type: VaultType,
    name: Option<String>,
    description: String,
    asset_index: Option<u64>,
    initial_assets: Option<String>,
    strategy_goal: String,
}

impl CreateVaultBuilder {
    pub fn new() -> Self {
        Self {
            vault_type: VaultType::Unspecified,
            name: None,
            description: String::new(),
            asset_index: None,
            initial_assets: None,
            strategy_goal: String::new(),
        }
    }

    pub fn vault_type(mut self, v: VaultType) -> Self {
        self.vault_type = v;
        self
    }
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }
    pub fn description(mut self, v: impl Into<String>) -> Self {
        self.description = v.into();
        self
    }
    pub fn asset_index(mut self, v: u64) -> Self {
        self.asset_index = Some(v);
        self
    }
    pub fn initial_assets(mut self, v: impl Into<String>) -> Self {
        self.initial_assets = Some(v.into());
        self
    }
    pub fn strategy_goal(mut self, v: impl Into<String>) -> Self {
        self.strategy_goal = v.into();
        self
    }

    pub fn build(self) -> Result<CreateVaultRequest, SdkError> {
        if self.vault_type == VaultType::Unspecified {
            return Err(SdkError::invalid_input("vault_type must be specified"));
        }
        let mut req = CreateVaultRequest::new(
            self.vault_type,
            self.name
                .ok_or_else(|| SdkError::invalid_input("name is required"))?,
            self.asset_index
                .ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.initial_assets
                .ok_or_else(|| SdkError::invalid_input("initial_assets is required"))?,
        );
        req.description = self.description;
        req.strategy_goal = self.strategy_goal;
        Ok(req)
    }
}

impl Default for CreateVaultBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ====================== UPDATE VAULT PARAMS ======================

pub struct UpdateVaultParamsBuilder {
    vault_id: Option<String>,
    min_stake: String,
    max_stake: String,
    new_description: String,
    deposit_capacity_native: Option<String>,
    soft_closed: Option<bool>,
    mandate: Option<crate::types::VaultMandate>,
}

impl UpdateVaultParamsBuilder {
    pub fn new() -> Self {
        Self {
            vault_id: None,
            min_stake: String::new(),
            max_stake: String::new(),
            new_description: String::new(),
            deposit_capacity_native: None,
            soft_closed: None,
            mandate: None,
        }
    }

    pub fn vault_id(mut self, v: impl Into<String>) -> Self {
        self.vault_id = Some(v.into());
        self
    }
    pub fn min_stake(mut self, v: impl Into<String>) -> Self {
        self.min_stake = v.into();
        self
    }
    pub fn max_stake(mut self, v: impl Into<String>) -> Self {
        self.max_stake = v.into();
        self
    }
    pub fn new_description(mut self, v: impl Into<String>) -> Self {
        self.new_description = v.into();
        self
    }
    /// VB5 — set the hard deposit capacity (base-asset native). Pass `"0"` to
    /// clear (uncapped).
    pub fn deposit_capacity(mut self, v: impl Into<String>) -> Self {
        self.deposit_capacity_native = Some(v.into());
        self
    }
    /// VB5 — toggle the manager soft-close (true stops new deposits).
    pub fn soft_closed(mut self, v: bool) -> Self {
        self.soft_closed = Some(v);
        self
    }
    /// VB6 — replace-as-unit mandate (must be a tightening of the current one).
    pub fn mandate(mut self, v: crate::types::VaultMandate) -> Self {
        self.mandate = Some(v);
        self
    }

    pub fn build(self) -> Result<UpdateVaultParamsRequest, SdkError> {
        let mut req = UpdateVaultParamsRequest::new(
            self.vault_id
                .ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
        );
        req.min_stake = self.min_stake;
        req.max_stake = self.max_stake;
        req.new_description = self.new_description;
        req.deposit_capacity_native = self.deposit_capacity_native;
        req.soft_closed = self.soft_closed;
        req.mandate = self.mandate;
        Ok(req)
    }
}

impl Default for UpdateVaultParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ====================== EXECUTE STRATEGY ======================

pub struct ExecuteStrategyBuilder {
    vault_id: Option<String>,
    strategy_params: Option<String>,
}

impl ExecuteStrategyBuilder {
    pub fn new() -> Self {
        Self {
            vault_id: None,
            strategy_params: None,
        }
    }

    pub fn vault_id(mut self, v: impl Into<String>) -> Self {
        self.vault_id = Some(v.into());
        self
    }
    pub fn strategy_params(mut self, v: impl Into<String>) -> Self {
        self.strategy_params = Some(v.into());
        self
    }

    pub fn build(self) -> Result<ExecuteStrategyRequest, SdkError> {
        let req = ExecuteStrategyRequest::new(
            self.vault_id
                .ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
            self.strategy_params
                .ok_or_else(|| SdkError::invalid_input("strategy_params is required"))?,
        );
        Ok(req)
    }
}

impl Default for ExecuteStrategyBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ====================== PAUSE / RESUME ======================

pub struct PauseVaultBuilder {
    vault_id: Option<String>,
    reason: Option<String>,
}

impl PauseVaultBuilder {
    pub fn new() -> Self {
        Self {
            vault_id: None,
            reason: None,
        }
    }

    pub fn vault_id(mut self, v: impl Into<String>) -> Self {
        self.vault_id = Some(v.into());
        self
    }
    pub fn reason(mut self, v: impl Into<String>) -> Self {
        self.reason = Some(v.into());
        self
    }

    pub fn build(self) -> Result<PauseVaultRequest, SdkError> {
        let req = PauseVaultRequest::new(
            self.vault_id
                .ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
            self.reason
                .ok_or_else(|| SdkError::invalid_input("reason is required"))?,
        );
        Ok(req)
    }
}

impl Default for PauseVaultBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ResumeVaultBuilder {
    vault_id: Option<String>,
}

impl ResumeVaultBuilder {
    pub fn new() -> Self {
        Self { vault_id: None }
    }

    pub fn vault_id(mut self, v: impl Into<String>) -> Self {
        self.vault_id = Some(v.into());
        self
    }

    pub fn build(self) -> Result<ResumeVaultRequest, SdkError> {
        let req = ResumeVaultRequest::new(
            self.vault_id
                .ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
        );
        Ok(req)
    }
}

impl Default for ResumeVaultBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ====================== DEPOSIT / WITHDRAW / CLAIM ======================

pub struct DepositToVaultBuilder {
    address: Option<String>,
    vault_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
}

impl DepositToVaultBuilder {
    pub fn new() -> Self {
        Self {
            address: None,
            vault_id: None,
            asset_index: None,
            amount: None,
        }
    }

    pub fn address(mut self, v: impl Into<String>) -> Self {
        self.address = Some(v.into());
        self
    }
    pub fn vault_id(mut self, v: impl Into<String>) -> Self {
        self.vault_id = Some(v.into());
        self
    }
    pub fn asset_index(mut self, v: u64) -> Self {
        self.asset_index = Some(v);
        self
    }
    pub fn amount(mut self, v: impl Into<String>) -> Self {
        self.amount = Some(v.into());
        self
    }

    pub fn build(self) -> Result<DepositToVaultRequest, SdkError> {
        let req = DepositToVaultRequest::new(
            self.address
                .ok_or_else(|| SdkError::invalid_input("address is required"))?,
            self.vault_id
                .ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
            self.asset_index
                .ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.amount
                .ok_or_else(|| SdkError::invalid_input("amount is required"))?,
        );
        Ok(req)
    }
}

impl Default for DepositToVaultBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct WithdrawFromVaultBuilder {
    address: Option<String>,
    vault_id: Option<String>,
    asset_index: Option<u64>,
    shares: Option<String>,
}

impl WithdrawFromVaultBuilder {
    pub fn new() -> Self {
        Self {
            address: None,
            vault_id: None,
            asset_index: None,
            shares: None,
        }
    }

    pub fn address(mut self, v: impl Into<String>) -> Self {
        self.address = Some(v.into());
        self
    }
    pub fn vault_id(mut self, v: impl Into<String>) -> Self {
        self.vault_id = Some(v.into());
        self
    }
    pub fn asset_index(mut self, v: u64) -> Self {
        self.asset_index = Some(v);
        self
    }
    pub fn shares(mut self, v: impl Into<String>) -> Self {
        self.shares = Some(v.into());
        self
    }

    pub fn build(self) -> Result<WithdrawFromVaultRequest, SdkError> {
        let req = WithdrawFromVaultRequest::new(
            self.address
                .ok_or_else(|| SdkError::invalid_input("address is required"))?,
            self.vault_id
                .ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
            self.asset_index
                .ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.shares
                .ok_or_else(|| SdkError::invalid_input("shares is required"))?,
        );
        Ok(req)
    }
}

impl Default for WithdrawFromVaultBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ClaimYieldBuilder {
    address: Option<String>,
    vault_id: Option<String>,
}

impl ClaimYieldBuilder {
    pub fn new() -> Self {
        Self {
            address: None,
            vault_id: None,
        }
    }

    pub fn address(mut self, v: impl Into<String>) -> Self {
        self.address = Some(v.into());
        self
    }
    pub fn vault_id(mut self, v: impl Into<String>) -> Self {
        self.vault_id = Some(v.into());
        self
    }

    pub fn build(self) -> Result<ClaimYieldRequest, SdkError> {
        let req = ClaimYieldRequest::new(
            self.address
                .ok_or_else(|| SdkError::invalid_input("address is required"))?,
            self.vault_id
                .ok_or_else(|| SdkError::invalid_input("vault_id is required"))?,
        );
        Ok(req)
    }
}

impl Default for ClaimYieldBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ====================== UPDATE PARAMS (GOVERNANCE) ======================

pub struct UpdateModuleParamsBuilder {
    authority: Option<String>,
    params: Option<VaultParams>,
}

impl UpdateModuleParamsBuilder {
    pub fn new() -> Self {
        Self {
            authority: None,
            params: None,
        }
    }

    pub fn authority(mut self, v: impl Into<String>) -> Self {
        self.authority = Some(v.into());
        self
    }
    pub fn params(mut self, v: VaultParams) -> Self {
        self.params = Some(v);
        self
    }

    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        Ok(UpdateParamsRequest::new(
            self.authority
                .ok_or_else(|| SdkError::invalid_input("authority is required"))?,
            self.params
                .ok_or_else(|| SdkError::invalid_input("params is required"))?,
        ))
    }
}

impl Default for UpdateModuleParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_vault_builder_works() {
        let req = CreateVaultBuilder::new()
            .vault_type(VaultType::Custom)
            .name("My Vault")
            .asset_index(1)
            .initial_assets("1000")
            .strategy_goal("maximize yield")
            .build()
            .unwrap();
        assert_eq!(req.name, "My Vault");
        assert_eq!(req.strategy_goal, "maximize yield");
    }

    #[test]
    fn create_vault_requires_type() {
        assert!(CreateVaultBuilder::new()
            .name("V")
            .asset_index(1)
            .initial_assets("1000")
            .build()
            .is_err());
    }

    #[test]
    fn deposit_builder_works() {
        let req = DepositToVaultBuilder::new()
            .address("morph1user")
            .vault_id("v1")
            .asset_index(1)
            .amount("500")
            .build()
            .unwrap();
        assert_eq!(req.amount, "500");
    }

    #[test]
    fn withdraw_builder_requires_shares() {
        assert!(WithdrawFromVaultBuilder::new()
            .address("morph1user")
            .vault_id("v1")
            .asset_index(1)
            .build()
            .is_err());
    }

    #[test]
    fn execute_strategy_builder_works() {
        let req = ExecuteStrategyBuilder::new()
            .vault_id("v1")
            .strategy_params(r#"{"action":"rebalance"}"#)
            .build()
            .unwrap();
        assert_eq!(req.vault_id, "v1");
    }

    #[test]
    fn pause_resume_builders_work() {
        let pause = PauseVaultBuilder::new()
            .vault_id("v1")
            .reason("maintenance")
            .build()
            .unwrap();
        assert_eq!(pause.reason, "maintenance");

        let resume = ResumeVaultBuilder::new().vault_id("v1").build().unwrap();
        assert_eq!(resume.vault_id, "v1");
    }

    #[test]
    fn claim_yield_builder_works() {
        let req = ClaimYieldBuilder::new()
            .address("morph1user")
            .vault_id("v1")
            .build()
            .unwrap();
        assert_eq!(req.vault_id, "v1");
    }

    #[test]
    fn update_module_params_validation() {
        assert!(UpdateModuleParamsBuilder::new().build().is_err());
    }

    #[test]
    fn update_vault_params_builder_capacity_fields() {
        let req = UpdateVaultParamsBuilder::new()
            .vault_id("v1")
            .deposit_capacity("5000")
            .soft_closed(true)
            .build()
            .unwrap();
        assert_eq!(req.deposit_capacity_native.as_deref(), Some("5000"));
        assert_eq!(req.soft_closed, Some(true));
        // Absent setters leave the optionals unset (leave-unchanged semantics).
        let plain = UpdateVaultParamsBuilder::new()
            .vault_id("v1")
            .new_description("x")
            .build()
            .unwrap();
        assert!(plain.deposit_capacity_native.is_none());
        assert!(plain.soft_closed.is_none());
        assert!(plain.mandate.is_none());
    }

    #[test]
    fn update_vault_params_builder_mandate() {
        use crate::types::VaultMandate;
        let req = UpdateVaultParamsBuilder::new()
            .vault_id("v1")
            .mandate(VaultMandate {
                allowed_markets: vec![7, 9],
                max_leverage: 3,
            })
            .build()
            .unwrap();
        let m = req.mandate.expect("mandate set");
        assert_eq!(m.allowed_markets, vec![7, 9]);
        assert_eq!(m.max_leverage, 3);
    }
}
