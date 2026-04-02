//! Fluent builders for the OSA module.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    BuySharesRequest, ClaimPayoutRequest, CreateAccountRequest,
    MergePositionsRequest, SellSharesRequest,
};

// ====================== CREATE ACCOUNT ======================

#[derive(Default)]
pub struct CreateAccountBuilder {
    creator: Option<String>,
    market_index: Option<u64>,
    outcome_id: Option<String>,
    collateral_asset_index: Option<u64>,
    initial_collateral: Option<u64>,
}

impl CreateAccountBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn creator(mut self, v: impl Into<String>) -> Self { self.creator = Some(v.into()); self }
    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn outcome_id(mut self, v: impl Into<String>) -> Self { self.outcome_id = Some(v.into()); self }
    pub fn collateral_asset_index(mut self, v: u64) -> Self { self.collateral_asset_index = Some(v); self }
    pub fn initial_collateral(mut self, v: u64) -> Self { self.initial_collateral = Some(v); self }

    pub fn build(self) -> Result<CreateAccountRequest, SdkError> {
        Ok(CreateAccountRequest::new(
            self.creator.ok_or_else(|| SdkError::invalid_input("creator is required"))?,
            self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            self.outcome_id.ok_or_else(|| SdkError::invalid_input("outcome_id is required"))?,
            self.collateral_asset_index.ok_or_else(|| SdkError::invalid_input("collateral_asset_index is required"))?,
            self.initial_collateral.ok_or_else(|| SdkError::invalid_input("initial_collateral is required"))?,
        ))
    }
}

// ====================== BUY SHARES ======================

#[derive(Default)]
pub struct BuySharesBuilder {
    buyer: Option<String>,
    account_id: Option<String>,
    collateral_amount: Option<u64>,
    min_shares_received: u64,
    max_fee_bps: u32,
}

impl BuySharesBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn buyer(mut self, v: impl Into<String>) -> Self { self.buyer = Some(v.into()); self }
    pub fn account_id(mut self, v: impl Into<String>) -> Self { self.account_id = Some(v.into()); self }
    pub fn collateral_amount(mut self, v: u64) -> Self { self.collateral_amount = Some(v); self }
    pub fn min_shares_received(mut self, v: u64) -> Self { self.min_shares_received = v; self }
    pub fn max_fee_bps(mut self, v: u32) -> Self { self.max_fee_bps = v; self }

    pub fn build(self) -> Result<BuySharesRequest, SdkError> {
        Ok(BuySharesRequest::new(
            self.buyer.ok_or_else(|| SdkError::invalid_input("buyer is required"))?,
            self.account_id.ok_or_else(|| SdkError::invalid_input("account_id is required"))?,
            self.collateral_amount.ok_or_else(|| SdkError::invalid_input("collateral_amount is required"))?,
            self.min_shares_received, self.max_fee_bps,
        ))
    }
}

// ====================== SELL SHARES ======================

#[derive(Default)]
pub struct SellSharesBuilder {
    seller: Option<String>,
    account_id: Option<String>,
    shares_amount: Option<u64>,
    min_collateral_received: u64,
}

impl SellSharesBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn seller(mut self, v: impl Into<String>) -> Self { self.seller = Some(v.into()); self }
    pub fn account_id(mut self, v: impl Into<String>) -> Self { self.account_id = Some(v.into()); self }
    pub fn shares_amount(mut self, v: u64) -> Self { self.shares_amount = Some(v); self }
    pub fn min_collateral_received(mut self, v: u64) -> Self { self.min_collateral_received = v; self }

    pub fn build(self) -> Result<SellSharesRequest, SdkError> {
        Ok(SellSharesRequest::new(
            self.seller.ok_or_else(|| SdkError::invalid_input("seller is required"))?,
            self.account_id.ok_or_else(|| SdkError::invalid_input("account_id is required"))?,
            self.shares_amount.ok_or_else(|| SdkError::invalid_input("shares_amount is required"))?,
            self.min_collateral_received,
        ))
    }
}

// ====================== MERGE POSITIONS ======================

#[derive(Default)]
pub struct MergePositionsBuilder {
    merger: Option<String>,
    account_ids: Vec<String>,
    min_collateral_received: u64,
}

impl MergePositionsBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn merger(mut self, v: impl Into<String>) -> Self { self.merger = Some(v.into()); self }
    pub fn account_ids(mut self, v: Vec<String>) -> Self { self.account_ids = v; self }
    pub fn min_collateral_received(mut self, v: u64) -> Self { self.min_collateral_received = v; self }

    pub fn build(self) -> Result<MergePositionsRequest, SdkError> {
        if self.account_ids.len() < 2 {
            return Err(SdkError::invalid_input("at least 2 account_ids required for merge"));
        }
        Ok(MergePositionsRequest::new(
            self.merger.ok_or_else(|| SdkError::invalid_input("merger is required"))?,
            self.account_ids, self.min_collateral_received,
        ))
    }
}

// ====================== CLAIM PAYOUT ======================

#[derive(Default)]
pub struct ClaimPayoutBuilder {
    claimer: Option<String>,
    beneficiary: Option<String>,
    account_id: Option<String>,
}

impl ClaimPayoutBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn claimer(mut self, v: impl Into<String>) -> Self { self.claimer = Some(v.into()); self }
    pub fn beneficiary(mut self, v: impl Into<String>) -> Self { self.beneficiary = Some(v.into()); self }
    pub fn account_id(mut self, v: impl Into<String>) -> Self { self.account_id = Some(v.into()); self }

    pub fn build(self) -> Result<ClaimPayoutRequest, SdkError> {
        Ok(ClaimPayoutRequest::new(
            self.claimer.ok_or_else(|| SdkError::invalid_input("claimer is required"))?,
            self.beneficiary.ok_or_else(|| SdkError::invalid_input("beneficiary is required"))?,
            self.account_id.ok_or_else(|| SdkError::invalid_input("account_id is required"))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn create_account_builder_works() {
        let req = CreateAccountBuilder::new()
            .creator("morph1xyz").market_index(1).outcome_id("yes")
            .collateral_asset_index(2).initial_collateral(1000)
            .build().unwrap();
        assert_eq!(req.market_index, 1);
    }

    #[test]
    fn create_account_builder_validation() {
        assert!(CreateAccountBuilder::new().build().is_err());
    }

    #[test]
    fn buy_shares_builder_works() {
        let req = BuySharesBuilder::new()
            .buyer("morph1xyz").account_id("acct1").collateral_amount(1000)
            .min_shares_received(990).max_fee_bps(100)
            .build().unwrap();
        assert_eq!(req.collateral_amount, 1000);
    }

    #[test]
    fn sell_shares_builder_validation() {
        assert!(SellSharesBuilder::new().build().is_err());
    }

    #[test]
    fn merge_positions_requires_two_accounts() {
        let result = MergePositionsBuilder::new()
            .merger("morph1xyz")
            .account_ids(vec!["acct1".into()])
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn merge_positions_builder_works() {
        let req = MergePositionsBuilder::new()
            .merger("morph1xyz")
            .account_ids(vec!["acct1".into(), "acct2".into()])
            .min_collateral_received(500)
            .build().unwrap();
        assert_eq!(req.account_ids.len(), 2);
    }

    #[test]
    fn claim_payout_builder_validation() {
        assert!(ClaimPayoutBuilder::new().build().is_err());
    }
}
