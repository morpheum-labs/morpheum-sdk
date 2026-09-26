//! Fluent builders for the Staking module.
//!
//! Ergonomic, type-safe builders for all staking transaction operations.
//! Each builder validates required fields and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::*;

// ============================================================================
// StakeBuilder
// ============================================================================

/// Fluent builder for staking MORM to a validator.
#[derive(Default)]
pub struct StakeBuilder {
    address: Option<String>,
    validator_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    external_address: Option<String>,
}

impl StakeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(mut self, addr: impl Into<String>) -> Self {
        self.address = Some(addr.into());
        self
    }
    pub fn validator_id(mut self, id: impl Into<String>) -> Self {
        self.validator_id = Some(id.into());
        self
    }
    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx);
        self
    }
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }
    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into());
        self
    }

    pub fn build(self) -> Result<StakeRequest, SdkError> {
        let address = self
            .address
            .ok_or_else(|| SdkError::invalid_input("address is required for staking"))?;
        let validator_id = self
            .validator_id
            .ok_or_else(|| SdkError::invalid_input("validator_id is required for staking"))?;
        let asset_index = self
            .asset_index
            .ok_or_else(|| SdkError::invalid_input("asset_index is required for staking"))?;
        let amount = self
            .amount
            .ok_or_else(|| SdkError::invalid_input("amount is required for staking"))?;

        let mut req = StakeRequest::new(address, validator_id, asset_index, amount);
        req.external_address = self.external_address;
        Ok(req)
    }
}

// ============================================================================
// UnstakeBuilder
// ============================================================================

/// Fluent builder for unstaking MORM.
#[derive(Default)]
pub struct UnstakeBuilder {
    address: Option<String>,
    validator_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    external_address: Option<String>,
}

impl UnstakeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(mut self, addr: impl Into<String>) -> Self {
        self.address = Some(addr.into());
        self
    }
    pub fn validator_id(mut self, id: impl Into<String>) -> Self {
        self.validator_id = Some(id.into());
        self
    }
    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx);
        self
    }
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }
    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into());
        self
    }

    pub fn build(self) -> Result<UnstakeRequest, SdkError> {
        let address = self
            .address
            .ok_or_else(|| SdkError::invalid_input("address is required for unstaking"))?;
        let validator_id = self
            .validator_id
            .ok_or_else(|| SdkError::invalid_input("validator_id is required for unstaking"))?;
        let asset_index = self
            .asset_index
            .ok_or_else(|| SdkError::invalid_input("asset_index is required for unstaking"))?;
        let amount = self
            .amount
            .ok_or_else(|| SdkError::invalid_input("amount is required for unstaking"))?;

        let mut req = UnstakeRequest::new(address, validator_id, asset_index, amount);
        req.external_address = self.external_address;
        Ok(req)
    }
}

// ============================================================================
// DelegateBuilder
// ============================================================================

/// Fluent builder for delegating MORM to a validator.
#[derive(Default)]
pub struct DelegateBuilder {
    delegator_address: Option<String>,
    validator_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    delegator_external_address: Option<String>,
}

impl DelegateBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn delegator_address(mut self, addr: impl Into<String>) -> Self {
        self.delegator_address = Some(addr.into());
        self
    }
    pub fn validator_id(mut self, id: impl Into<String>) -> Self {
        self.validator_id = Some(id.into());
        self
    }
    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx);
        self
    }
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }
    pub fn delegator_external_address(mut self, addr: impl Into<String>) -> Self {
        self.delegator_external_address = Some(addr.into());
        self
    }

    pub fn build(self) -> Result<DelegateRequest, SdkError> {
        let delegator_address = self.delegator_address.ok_or_else(|| {
            SdkError::invalid_input("delegator_address is required for delegation")
        })?;
        let validator_id = self
            .validator_id
            .ok_or_else(|| SdkError::invalid_input("validator_id is required for delegation"))?;
        let asset_index = self
            .asset_index
            .ok_or_else(|| SdkError::invalid_input("asset_index is required for delegation"))?;
        let amount = self
            .amount
            .ok_or_else(|| SdkError::invalid_input("amount is required for delegation"))?;

        let mut req = DelegateRequest::new(delegator_address, validator_id, asset_index, amount);
        req.delegator_external_address = self.delegator_external_address;
        Ok(req)
    }
}

// ============================================================================
// UndelegateBuilder
// ============================================================================

/// Fluent builder for undelegating MORM from a validator.
#[derive(Default)]
pub struct UndelegateBuilder {
    delegator_address: Option<String>,
    validator_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    delegator_external_address: Option<String>,
    delegator_chain_type: Option<i32>,
}

impl UndelegateBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn delegator_address(mut self, addr: impl Into<String>) -> Self {
        self.delegator_address = Some(addr.into());
        self
    }
    pub fn validator_id(mut self, id: impl Into<String>) -> Self {
        self.validator_id = Some(id.into());
        self
    }
    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx);
        self
    }
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }
    pub fn delegator_external_address(mut self, addr: impl Into<String>) -> Self {
        self.delegator_external_address = Some(addr.into());
        self
    }
    pub fn delegator_chain_type(mut self, ct: i32) -> Self {
        self.delegator_chain_type = Some(ct);
        self
    }

    pub fn build(self) -> Result<UndelegateRequest, SdkError> {
        let delegator_address = self.delegator_address.ok_or_else(|| {
            SdkError::invalid_input("delegator_address is required for undelegation")
        })?;
        let validator_id = self
            .validator_id
            .ok_or_else(|| SdkError::invalid_input("validator_id is required for undelegation"))?;
        let asset_index = self
            .asset_index
            .ok_or_else(|| SdkError::invalid_input("asset_index is required for undelegation"))?;
        let amount = self
            .amount
            .ok_or_else(|| SdkError::invalid_input("amount is required for undelegation"))?;

        let mut req = UndelegateRequest::new(delegator_address, validator_id, asset_index, amount);
        req.delegator_external_address = self.delegator_external_address;
        req.delegator_chain_type = self.delegator_chain_type;
        Ok(req)
    }
}

// ============================================================================
// RedelegateBuilder
// ============================================================================

/// Fluent builder for redelegating MORM between validators.
#[derive(Default)]
pub struct RedelegateBuilder {
    delegator_address: Option<String>,
    from_validator_id: Option<String>,
    to_validator_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    delegator_external_address: Option<String>,
    delegator_chain_type: Option<i32>,
}

impl RedelegateBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn delegator_address(mut self, addr: impl Into<String>) -> Self {
        self.delegator_address = Some(addr.into());
        self
    }
    pub fn from_validator_id(mut self, id: impl Into<String>) -> Self {
        self.from_validator_id = Some(id.into());
        self
    }
    pub fn to_validator_id(mut self, id: impl Into<String>) -> Self {
        self.to_validator_id = Some(id.into());
        self
    }
    pub fn asset_index(mut self, idx: u64) -> Self {
        self.asset_index = Some(idx);
        self
    }
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }
    pub fn delegator_external_address(mut self, addr: impl Into<String>) -> Self {
        self.delegator_external_address = Some(addr.into());
        self
    }
    pub fn delegator_chain_type(mut self, ct: i32) -> Self {
        self.delegator_chain_type = Some(ct);
        self
    }

    pub fn build(self) -> Result<RedelegateRequest, SdkError> {
        let delegator_address = self.delegator_address.ok_or_else(|| {
            SdkError::invalid_input("delegator_address is required for redelegation")
        })?;
        let from_validator_id = self.from_validator_id.ok_or_else(|| {
            SdkError::invalid_input("from_validator_id is required for redelegation")
        })?;
        let to_validator_id = self.to_validator_id.ok_or_else(|| {
            SdkError::invalid_input("to_validator_id is required for redelegation")
        })?;
        let asset_index = self
            .asset_index
            .ok_or_else(|| SdkError::invalid_input("asset_index is required for redelegation"))?;
        let amount = self
            .amount
            .ok_or_else(|| SdkError::invalid_input("amount is required for redelegation"))?;

        let mut req = RedelegateRequest::new(
            delegator_address,
            from_validator_id,
            to_validator_id,
            asset_index,
            amount,
        );
        req.delegator_external_address = self.delegator_external_address;
        req.delegator_chain_type = self.delegator_chain_type;
        Ok(req)
    }
}

// ============================================================================
// ClaimRewardsBuilder
// ============================================================================

/// Fluent builder for claiming staking rewards.
#[derive(Default)]
pub struct ClaimRewardsBuilder {
    address: Option<String>,
    validator_id: Option<String>,
    external_address: Option<String>,
}

impl ClaimRewardsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(mut self, addr: impl Into<String>) -> Self {
        self.address = Some(addr.into());
        self
    }
    pub fn validator_id(mut self, id: impl Into<String>) -> Self {
        self.validator_id = Some(id.into());
        self
    }
    pub fn external_address(mut self, addr: impl Into<String>) -> Self {
        self.external_address = Some(addr.into());
        self
    }

    pub fn build(self) -> Result<ClaimRewardsRequest, SdkError> {
        let address = self
            .address
            .ok_or_else(|| SdkError::invalid_input("address is required for claiming rewards"))?;
        let validator_id = self.validator_id.unwrap_or_default();

        let mut req = ClaimRewardsRequest::new(address, validator_id);
        req.external_address = self.external_address;
        Ok(req)
    }
}

// ============================================================================
// UpdateParamsBuilder
// ============================================================================

/// Fluent builder for governance-gated staking parameter updates.
#[derive(Default)]
pub struct UpdateParamsBuilder {
    authority: Option<String>,
    params: Option<morpheum_proto::staking::v1::Params>,
}

impl UpdateParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn authority(mut self, authority: impl Into<String>) -> Self {
        self.authority = Some(authority.into());
        self
    }
    pub fn params(mut self, params: morpheum_proto::staking::v1::Params) -> Self {
        self.params = Some(params);
        self
    }

    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        let authority = self
            .authority
            .ok_or_else(|| SdkError::invalid_input("authority is required for update_params"))?;
        let params = self
            .params
            .ok_or_else(|| SdkError::invalid_input("params is required for update_params"))?;
        Ok(UpdateParamsRequest::new(authority, params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stake_builder_works() {
        let req = StakeBuilder::new()
            .address("morm1abc")
            .validator_id("val-1")
            .asset_index(0)
            .amount("1000000")
            .build()
            .unwrap();
        assert_eq!(req.address, "morm1abc");
        assert_eq!(req.validator_id, "val-1");
    }

    #[test]
    fn stake_builder_missing_required() {
        assert!(StakeBuilder::new().build().is_err());
        assert!(StakeBuilder::new().address("a").build().is_err());
    }

    #[test]
    fn delegate_builder_works() {
        let req = DelegateBuilder::new()
            .delegator_address("morm1abc")
            .validator_id("val-1")
            .asset_index(0)
            .amount("500000")
            .build()
            .unwrap();
        assert_eq!(req.delegator_address, "morm1abc");
    }

    #[test]
    fn redelegate_builder_works() {
        let req = RedelegateBuilder::new()
            .delegator_address("morm1abc")
            .from_validator_id("val-1")
            .to_validator_id("val-2")
            .asset_index(0)
            .amount("250000")
            .build()
            .unwrap();
        assert_eq!(req.from_validator_id, "val-1");
        assert_eq!(req.to_validator_id, "val-2");
    }

    #[test]
    fn claim_rewards_builder_works() {
        let req = ClaimRewardsBuilder::new()
            .address("morm1abc")
            .validator_id("val-1")
            .build()
            .unwrap();
        assert_eq!(req.address, "morm1abc");
    }

    #[test]
    fn claim_rewards_builder_no_validator() {
        let req = ClaimRewardsBuilder::new()
            .address("morm1abc")
            .build()
            .unwrap();
        assert!(req.validator_id.is_empty());
    }
}
