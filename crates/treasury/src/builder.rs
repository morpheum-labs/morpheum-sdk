//! Fluent builders for the treasury module.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{AllocateFundsRequest, SweepRevenueRequest, UpdateParamsRequest};
use crate::types::{ReserveCategory, TreasuryParams};

// ====================== SWEEP REVENUE ======================

#[derive(Default)]
pub struct SweepRevenueBuilder {
    source_module: Option<String>,
    target_category: ReserveCategory,
    amount: Option<u64>,
    reason: Option<String>,
    tx_hash: Vec<u8>,
    authority: Option<String>,
}

impl SweepRevenueBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn source_module(mut self, v: impl Into<String>) -> Self { self.source_module = Some(v.into()); self }
    pub fn target_category(mut self, v: ReserveCategory) -> Self { self.target_category = v; self }
    pub fn amount(mut self, v: u64) -> Self { self.amount = Some(v); self }
    pub fn reason(mut self, v: impl Into<String>) -> Self { self.reason = Some(v.into()); self }
    pub fn tx_hash(mut self, v: Vec<u8>) -> Self { self.tx_hash = v; self }
    pub fn authority(mut self, v: impl Into<String>) -> Self { self.authority = Some(v.into()); self }

    pub fn build(self) -> Result<SweepRevenueRequest, SdkError> {
        if self.target_category == ReserveCategory::Unspecified {
            return Err(SdkError::invalid_input("target_category must be specified"));
        }
        let mut req = SweepRevenueRequest::new(
            self.source_module.ok_or_else(|| SdkError::invalid_input("source_module is required"))?,
            self.target_category,
            self.amount.ok_or_else(|| SdkError::invalid_input("amount is required"))?,
            self.reason.ok_or_else(|| SdkError::invalid_input("reason is required"))?,
            self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?,
        );
        req.tx_hash = self.tx_hash;
        Ok(req)
    }
}

// ====================== ALLOCATE FUNDS ======================

#[derive(Default)]
pub struct AllocateFundsBuilder {
    authority: Option<String>,
    source_category: ReserveCategory,
    target_module: String,
    target_category: ReserveCategory,
    amount: Option<u64>,
    reason: Option<String>,
    proposal_id: u64,
    signature: Vec<u8>,
}

impl AllocateFundsBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn authority(mut self, v: impl Into<String>) -> Self { self.authority = Some(v.into()); self }
    pub fn source_category(mut self, v: ReserveCategory) -> Self { self.source_category = v; self }
    pub fn target_module(mut self, v: impl Into<String>) -> Self { self.target_module = v.into(); self }
    pub fn target_category(mut self, v: ReserveCategory) -> Self { self.target_category = v; self }
    pub fn amount(mut self, v: u64) -> Self { self.amount = Some(v); self }
    pub fn reason(mut self, v: impl Into<String>) -> Self { self.reason = Some(v.into()); self }
    pub fn proposal_id(mut self, v: u64) -> Self { self.proposal_id = v; self }
    pub fn signature(mut self, v: Vec<u8>) -> Self { self.signature = v; self }

    pub fn build(self) -> Result<AllocateFundsRequest, SdkError> {
        if self.source_category == ReserveCategory::Unspecified {
            return Err(SdkError::invalid_input("source_category must be specified"));
        }
        let mut req = AllocateFundsRequest::new(
            self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?,
            self.source_category,
            self.amount.ok_or_else(|| SdkError::invalid_input("amount is required"))?,
            self.reason.ok_or_else(|| SdkError::invalid_input("reason is required"))?,
        );
        req.target_module = self.target_module;
        req.target_category = self.target_category;
        req.proposal_id = self.proposal_id;
        req.signature = self.signature;
        Ok(req)
    }
}

// ====================== UPDATE PARAMS ======================

pub struct UpdateParamsBuilder {
    authority: Option<String>,
    params: Option<TreasuryParams>,
}

impl UpdateParamsBuilder {
    pub fn new() -> Self { Self { authority: None, params: None } }

    pub fn authority(mut self, v: impl Into<String>) -> Self { self.authority = Some(v.into()); self }
    pub fn params(mut self, v: TreasuryParams) -> Self { self.params = Some(v); self }

    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        Ok(UpdateParamsRequest::new(
            self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?,
            self.params.ok_or_else(|| SdkError::invalid_input("params is required"))?,
        ))
    }
}

impl Default for UpdateParamsBuilder {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sweep_revenue_builder_works() {
        let req = SweepRevenueBuilder::new()
            .source_module("clob").target_category(ReserveCategory::InsuranceProtection)
            .amount(1000).reason("maker_taker_fees").authority("morph1mod")
            .build().unwrap();
        assert_eq!(req.amount, 1000);
    }

    #[test]
    fn sweep_revenue_requires_category() {
        let result = SweepRevenueBuilder::new()
            .source_module("clob").amount(1000).reason("test").authority("morph1mod")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn allocate_funds_builder_works() {
        let req = AllocateFundsBuilder::new()
            .authority("morph1gov").source_category(ReserveCategory::InsuranceProtection)
            .target_module("insurance").amount(5000).reason("insurance_topup")
            .build().unwrap();
        assert_eq!(req.target_module, "insurance");
    }

    #[test]
    fn allocate_funds_requires_source() {
        let result = AllocateFundsBuilder::new()
            .authority("morph1gov").amount(5000).reason("test")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn update_params_validation() {
        assert!(UpdateParamsBuilder::new().build().is_err());
    }
}
