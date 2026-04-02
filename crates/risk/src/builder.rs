//! Fluent builders for the risk module.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    TriggerLiquidationRequest, UpdateRiskConfigRequest,
};
use crate::types::RiskConfig;

// ====================== TRIGGER LIQUIDATION ======================

#[derive(Default)]
pub struct TriggerLiquidationBuilder {
    market_index: Option<u64>,
    bucket_id: Option<u64>,
}

impl TriggerLiquidationBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn bucket_id(mut self, v: u64) -> Self { self.bucket_id = Some(v); self }

    pub fn build(self) -> Result<TriggerLiquidationRequest, SdkError> {
        Ok(TriggerLiquidationRequest::new(
            self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            self.bucket_id.ok_or_else(|| SdkError::invalid_input("bucket_id is required"))?,
        ))
    }
}

// ====================== UPDATE RISK CONFIG ======================

pub struct UpdateRiskConfigBuilder {
    authority: Option<String>,
    config: Option<RiskConfig>,
}

impl UpdateRiskConfigBuilder {
    pub fn new() -> Self { Self { authority: None, config: None } }

    pub fn authority(mut self, v: impl Into<String>) -> Self { self.authority = Some(v.into()); self }
    pub fn config(mut self, v: RiskConfig) -> Self { self.config = Some(v); self }

    pub fn build(self) -> Result<UpdateRiskConfigRequest, SdkError> {
        Ok(UpdateRiskConfigRequest::new(
            self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?,
            self.config.ok_or_else(|| SdkError::invalid_input("config is required"))?,
        ))
    }
}

impl Default for UpdateRiskConfigBuilder {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_liquidation_builder_works() {
        let req = TriggerLiquidationBuilder::new()
            .market_index(0).bucket_id(42)
            .build().unwrap();
        assert_eq!(req.market_index, 0);
        assert_eq!(req.bucket_id, 42);
    }

    #[test]
    fn trigger_liquidation_validation() {
        assert!(TriggerLiquidationBuilder::new().build().is_err());
    }

    #[test]
    fn update_risk_config_builder_works() {
        let req = UpdateRiskConfigBuilder::new()
            .authority("morpheum1gov")
            .config(RiskConfig {
                band_width_bps: 100,
                num_bands_above_below: 10,
                imbalance_threshold_bps: 500,
                imbalance_hysteresis_bps: 100,
                cascade_max_per_market_per_epoch: 5,
                max_scan_limit: 100,
                liquidation_margin_ratio_bps: 500,
                prediction_margin_ratio_bps: 700,
                price_move_threshold_bps: 300,
                partial_band_shift_enabled: true,
                var_confidence_bps: 9900,
                var_horizon_hours: 24,
                enable_vrf_fairness: false,
                enable_proactive_liquidation_events: true,
                enable_pre_trade_simulation: true,
                enable_spot_risk_integration: false,
                contagion_threshold_sat: 1_000_000,
            })
            .build().unwrap();
        assert_eq!(req.authority, "morpheum1gov");
    }

    #[test]
    fn update_risk_config_validation() {
        assert!(UpdateRiskConfigBuilder::new().build().is_err());
    }
}
