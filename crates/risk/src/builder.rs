//! Fluent builders for the risk module.

use morpheum_sdk_core::SdkError;

use crate::requests::{
    BucketLiquidationExecutedRequest, EpochRiskTickRequest, LiquidationCheckRequest,
    ShortfallReportRequest, UpdateRiskConfigRequest,
};
use crate::types::RiskConfig;

// ====================== EPOCH RISK TICK ======================

#[derive(Default)]
pub struct EpochRiskTickBuilder {
    epoch_id: Option<u64>,
    market_index: Option<u64>,
    logical_timestamp: Option<u64>,
}

impl EpochRiskTickBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn epoch_id(mut self, v: u64) -> Self { self.epoch_id = Some(v); self }
    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn logical_timestamp(mut self, v: u64) -> Self { self.logical_timestamp = Some(v); self }

    pub fn build(self) -> Result<EpochRiskTickRequest, SdkError> {
        Ok(EpochRiskTickRequest::new(
            self.epoch_id.ok_or_else(|| SdkError::invalid_input("epoch_id is required"))?,
            self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            self.logical_timestamp.ok_or_else(|| SdkError::invalid_input("logical_timestamp is required"))?,
        ))
    }
}

// ====================== LIQUIDATION CHECK ======================

#[derive(Default)]
pub struct LiquidationCheckBuilder {
    market_index: Option<u64>,
    mark_price: Option<u64>,
    logical_timestamp: Option<u64>,
}

impl LiquidationCheckBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn mark_price(mut self, v: u64) -> Self { self.mark_price = Some(v); self }
    pub fn logical_timestamp(mut self, v: u64) -> Self { self.logical_timestamp = Some(v); self }

    pub fn build(self) -> Result<LiquidationCheckRequest, SdkError> {
        Ok(LiquidationCheckRequest::new(
            self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            self.mark_price.ok_or_else(|| SdkError::invalid_input("mark_price is required"))?,
            self.logical_timestamp.ok_or_else(|| SdkError::invalid_input("logical_timestamp is required"))?,
        ))
    }
}

// ====================== UPDATE RISK CONFIG ======================

pub struct UpdateRiskConfigBuilder {
    config: Option<RiskConfig>,
}

impl UpdateRiskConfigBuilder {
    pub fn new() -> Self { Self { config: None } }

    pub fn config(mut self, v: RiskConfig) -> Self { self.config = Some(v); self }

    pub fn build(self) -> Result<UpdateRiskConfigRequest, SdkError> {
        Ok(UpdateRiskConfigRequest::new(
            self.config.ok_or_else(|| SdkError::invalid_input("config is required"))?,
        ))
    }
}

impl Default for UpdateRiskConfigBuilder {
    fn default() -> Self { Self::new() }
}

// ====================== SHORTFALL REPORT ======================

#[derive(Default)]
pub struct ShortfallReportBuilder {
    bucket_id: Option<u64>,
    liquidation_id: Option<u64>,
    market_index: Option<u64>,
    shortfall_amount: Option<u64>,
}

impl ShortfallReportBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn bucket_id(mut self, v: u64) -> Self { self.bucket_id = Some(v); self }
    pub fn liquidation_id(mut self, v: u64) -> Self { self.liquidation_id = Some(v); self }
    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn shortfall_amount(mut self, v: u64) -> Self { self.shortfall_amount = Some(v); self }

    pub fn build(self) -> Result<ShortfallReportRequest, SdkError> {
        Ok(ShortfallReportRequest::new(
            self.bucket_id.ok_or_else(|| SdkError::invalid_input("bucket_id is required"))?,
            self.liquidation_id.ok_or_else(|| SdkError::invalid_input("liquidation_id is required"))?,
            self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            self.shortfall_amount.ok_or_else(|| SdkError::invalid_input("shortfall_amount is required"))?,
        ))
    }
}

// ====================== BUCKET LIQUIDATION EXECUTED ======================

#[derive(Default)]
pub struct BucketLiquidationExecutedBuilder {
    bucket_id: Option<u64>,
    liquidation_id: Option<u64>,
    market_index: Option<u64>,
    shortfall_sat: Option<u64>,
    block_height: Option<u64>,
    shard_id: u64,
}

impl BucketLiquidationExecutedBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn bucket_id(mut self, v: u64) -> Self { self.bucket_id = Some(v); self }
    pub fn liquidation_id(mut self, v: u64) -> Self { self.liquidation_id = Some(v); self }
    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn shortfall_sat(mut self, v: u64) -> Self { self.shortfall_sat = Some(v); self }
    pub fn block_height(mut self, v: u64) -> Self { self.block_height = Some(v); self }
    pub fn shard_id(mut self, v: u64) -> Self { self.shard_id = v; self }

    pub fn build(self) -> Result<BucketLiquidationExecutedRequest, SdkError> {
        Ok(BucketLiquidationExecutedRequest::new(
            self.bucket_id.ok_or_else(|| SdkError::invalid_input("bucket_id is required"))?,
            self.liquidation_id.ok_or_else(|| SdkError::invalid_input("liquidation_id is required"))?,
            self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            self.shortfall_sat.ok_or_else(|| SdkError::invalid_input("shortfall_sat is required"))?,
            self.block_height.ok_or_else(|| SdkError::invalid_input("block_height is required"))?,
            self.shard_id,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn epoch_risk_tick_builder_works() {
        let req = EpochRiskTickBuilder::new()
            .epoch_id(1).market_index(0).logical_timestamp(100)
            .build().unwrap();
        assert_eq!(req.epoch_id, 1);
    }

    #[test]
    fn epoch_risk_tick_validation() {
        assert!(EpochRiskTickBuilder::new().build().is_err());
    }

    #[test]
    fn liquidation_check_builder_works() {
        let req = LiquidationCheckBuilder::new()
            .market_index(0).mark_price(50000).logical_timestamp(100)
            .build().unwrap();
        assert_eq!(req.mark_price, 50000);
    }

    #[test]
    fn shortfall_report_builder_works() {
        let req = ShortfallReportBuilder::new()
            .bucket_id(1).liquidation_id(2).market_index(0).shortfall_amount(5000)
            .build().unwrap();
        assert_eq!(req.shortfall_amount, 5000);
    }

    #[test]
    fn bucket_liquidation_executed_builder_works() {
        let req = BucketLiquidationExecutedBuilder::new()
            .bucket_id(1).liquidation_id(2).market_index(0)
            .shortfall_sat(5000).block_height(100)
            .build().unwrap();
        assert_eq!(req.block_height, 100);
    }

    #[test]
    fn shortfall_report_validation() {
        assert!(ShortfallReportBuilder::new().build().is_err());
    }
}
