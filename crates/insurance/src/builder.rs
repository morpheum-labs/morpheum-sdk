//! Fluent builders for the insurance vault module.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    AbsorbDeficitRequest, ClaimBountyRequest, ClaimYieldRequest,
    HedgeIlRequest, ReplenishVaultRequest, StakeToVaultRequest, WithdrawStakeRequest,
};
use crate::types::ChainType;

// ====================== ABSORB DEFICIT ======================

#[derive(Default)]
pub struct AbsorbDeficitBuilder {
    position_id: Option<String>,
    market_index: Option<u64>,
    asset_index: Option<u64>,
    deficit_amount: Option<String>,
    recovered_amount: Option<String>,
    absorber_address: Option<String>,
    absorber_external_address: Option<String>,
    absorber_chain_type: Option<ChainType>,
}

impl AbsorbDeficitBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn position_id(mut self, v: impl Into<String>) -> Self { self.position_id = Some(v.into()); self }
    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn deficit_amount(mut self, v: impl Into<String>) -> Self { self.deficit_amount = Some(v.into()); self }
    pub fn recovered_amount(mut self, v: impl Into<String>) -> Self { self.recovered_amount = Some(v.into()); self }
    pub fn absorber_address(mut self, v: impl Into<String>) -> Self { self.absorber_address = Some(v.into()); self }
    pub fn absorber_external(mut self, addr: impl Into<String>, chain: ChainType) -> Self {
        self.absorber_external_address = Some(addr.into());
        self.absorber_chain_type = Some(chain);
        self
    }

    pub fn build(self) -> Result<AbsorbDeficitRequest, SdkError> {
        let mut req = AbsorbDeficitRequest::new(
            self.position_id.ok_or_else(|| SdkError::invalid_input("position_id is required"))?,
            self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.deficit_amount.ok_or_else(|| SdkError::invalid_input("deficit_amount is required"))?,
            self.recovered_amount.ok_or_else(|| SdkError::invalid_input("recovered_amount is required"))?,
            self.absorber_address.ok_or_else(|| SdkError::invalid_input("absorber_address is required"))?,
        );
        req.absorber_external_address = self.absorber_external_address;
        req.absorber_chain_type = self.absorber_chain_type;
        Ok(req)
    }
}

// ====================== REPLENISH VAULT ======================

#[derive(Default)]
pub struct ReplenishVaultBuilder {
    asset_index: Option<u64>,
    amount: Option<String>,
    source: Option<String>,
    replenisher_address: Option<String>,
    replenisher_external_address: Option<String>,
    replenisher_chain_type: Option<ChainType>,
}

impl ReplenishVaultBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn amount(mut self, v: impl Into<String>) -> Self { self.amount = Some(v.into()); self }
    pub fn source(mut self, v: impl Into<String>) -> Self { self.source = Some(v.into()); self }
    pub fn replenisher_address(mut self, v: impl Into<String>) -> Self { self.replenisher_address = Some(v.into()); self }
    pub fn replenisher_external(mut self, addr: impl Into<String>, chain: ChainType) -> Self {
        self.replenisher_external_address = Some(addr.into());
        self.replenisher_chain_type = Some(chain);
        self
    }

    pub fn build(self) -> Result<ReplenishVaultRequest, SdkError> {
        let mut req = ReplenishVaultRequest::new(
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.amount.ok_or_else(|| SdkError::invalid_input("amount is required"))?,
            self.source.ok_or_else(|| SdkError::invalid_input("source is required"))?,
            self.replenisher_address.ok_or_else(|| SdkError::invalid_input("replenisher_address is required"))?,
        );
        req.replenisher_external_address = self.replenisher_external_address;
        req.replenisher_chain_type = self.replenisher_chain_type;
        Ok(req)
    }
}

// ====================== STAKE TO VAULT ======================

#[derive(Default)]
pub struct StakeToVaultBuilder {
    address: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    external_address: Option<String>,
}

impl StakeToVaultBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, v: impl Into<String>) -> Self { self.address = Some(v.into()); self }
    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn amount(mut self, v: impl Into<String>) -> Self { self.amount = Some(v.into()); self }
    pub fn external_address(mut self, v: impl Into<String>) -> Self { self.external_address = Some(v.into()); self }

    pub fn build(self) -> Result<StakeToVaultRequest, SdkError> {
        let mut req = StakeToVaultRequest::new(
            self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?,
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.amount.ok_or_else(|| SdkError::invalid_input("amount is required"))?,
        );
        req.external_address = self.external_address;
        Ok(req)
    }
}

// ====================== WITHDRAW STAKE ======================

#[derive(Default)]
pub struct WithdrawStakeBuilder {
    address: Option<String>,
    asset_index: Option<u64>,
    shares: Option<String>,
    external_address: Option<String>,
}

impl WithdrawStakeBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, v: impl Into<String>) -> Self { self.address = Some(v.into()); self }
    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn shares(mut self, v: impl Into<String>) -> Self { self.shares = Some(v.into()); self }
    pub fn external_address(mut self, v: impl Into<String>) -> Self { self.external_address = Some(v.into()); self }

    pub fn build(self) -> Result<WithdrawStakeRequest, SdkError> {
        let mut req = WithdrawStakeRequest::new(
            self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?,
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.shares.ok_or_else(|| SdkError::invalid_input("shares is required"))?,
        );
        req.external_address = self.external_address;
        Ok(req)
    }
}

// ====================== CLAIM YIELD ======================

#[derive(Default)]
pub struct ClaimYieldBuilder {
    address: Option<String>,
    asset_index: Option<u64>,
    external_address: Option<String>,
}

impl ClaimYieldBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, v: impl Into<String>) -> Self { self.address = Some(v.into()); self }
    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn external_address(mut self, v: impl Into<String>) -> Self { self.external_address = Some(v.into()); self }

    pub fn build(self) -> Result<ClaimYieldRequest, SdkError> {
        let mut req = ClaimYieldRequest::new(
            self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?,
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
        );
        req.external_address = self.external_address;
        Ok(req)
    }
}

// ====================== CLAIM BOUNTY ======================

#[derive(Default)]
pub struct ClaimBountyBuilder {
    address: Option<String>,
    asset_index: Option<u64>,
    liquidation_id: Option<String>,
    external_address: Option<String>,
}

impl ClaimBountyBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, v: impl Into<String>) -> Self { self.address = Some(v.into()); self }
    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn liquidation_id(mut self, v: impl Into<String>) -> Self { self.liquidation_id = Some(v.into()); self }
    pub fn external_address(mut self, v: impl Into<String>) -> Self { self.external_address = Some(v.into()); self }

    pub fn build(self) -> Result<ClaimBountyRequest, SdkError> {
        let mut req = ClaimBountyRequest::new(
            self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?,
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.liquidation_id.ok_or_else(|| SdkError::invalid_input("liquidation_id is required"))?,
        );
        req.external_address = self.external_address;
        Ok(req)
    }
}

// ====================== HEDGE IL ======================

#[derive(Default)]
pub struct HedgeIlBuilder {
    address: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
    external_address: Option<String>,
}

impl HedgeIlBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, v: impl Into<String>) -> Self { self.address = Some(v.into()); self }
    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn amount(mut self, v: impl Into<String>) -> Self { self.amount = Some(v.into()); self }
    pub fn external_address(mut self, v: impl Into<String>) -> Self { self.external_address = Some(v.into()); self }

    pub fn build(self) -> Result<HedgeIlRequest, SdkError> {
        let mut req = HedgeIlRequest::new(
            self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?,
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.amount.ok_or_else(|| SdkError::invalid_input("amount is required"))?,
        );
        req.external_address = self.external_address;
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absorb_deficit_builder_works() {
        let req = AbsorbDeficitBuilder::new()
            .position_id("pos1")
            .market_index(42)
            .asset_index(1)
            .deficit_amount("1000")
            .recovered_amount("500")
            .absorber_address("morph1xyz")
            .build().unwrap();
        assert_eq!(req.position_id, "pos1");
        assert_eq!(req.market_index, 42);
    }

    #[test]
    fn absorb_deficit_builder_validation() {
        assert!(AbsorbDeficitBuilder::new().build().is_err());
    }

    #[test]
    fn stake_to_vault_builder_works() {
        let req = StakeToVaultBuilder::new()
            .address("morph1xyz").asset_index(1).amount("1000")
            .build().unwrap();
        assert_eq!(req.address, "morph1xyz");
    }

    #[test]
    fn withdraw_stake_builder_validation() {
        assert!(WithdrawStakeBuilder::new().build().is_err());
    }

    #[test]
    fn claim_yield_builder_works() {
        let req = ClaimYieldBuilder::new()
            .address("morph1xyz").asset_index(1)
            .build().unwrap();
        assert_eq!(req.asset_index, 1);
    }

    #[test]
    fn claim_bounty_builder_validation() {
        assert!(ClaimBountyBuilder::new().build().is_err());
    }

    #[test]
    fn hedge_il_builder_works() {
        let req = HedgeIlBuilder::new()
            .address("morph1xyz").asset_index(1).amount("100")
            .build().unwrap();
        assert_eq!(req.amount, "100");
    }

    #[test]
    fn replenish_vault_builder_with_external() {
        let req = ReplenishVaultBuilder::new()
            .asset_index(1).amount("500").source("fees").replenisher_address("morph1xyz")
            .replenisher_external("0xdead", ChainType::Ethereum)
            .build().unwrap();
        assert_eq!(req.replenisher_chain_type, Some(ChainType::Ethereum));
    }
}
