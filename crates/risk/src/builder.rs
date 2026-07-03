//! Fluent builders for the risk module.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{TriggerLiquidationRequest, UpdateParamsRequest};
use crate::types::{RiskConfig, RiskParams};

// ====================== TRIGGER LIQUIDATION ======================

#[derive(Default)]
pub struct TriggerLiquidationBuilder {
    market_index: Option<u64>,
    bucket_id: Option<u64>,
}

impl TriggerLiquidationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn market_index(mut self, v: u64) -> Self {
        self.market_index = Some(v);
        self
    }
    pub fn bucket_id(mut self, v: u64) -> Self {
        self.bucket_id = Some(v);
        self
    }

    pub fn build(self) -> Result<TriggerLiquidationRequest, SdkError> {
        Ok(TriggerLiquidationRequest::new(
            self.market_index
                .ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            self.bucket_id
                .ok_or_else(|| SdkError::invalid_input("bucket_id is required"))?,
        ))
    }
}

// ====================== UPDATE PARAMS ======================

/// Fluent builder for `MsgUpdateParams` (governance).
///
/// `MsgUpdateParams` is a **full-replace** write: any sub-config left unset
/// here is cleared on chain, not left unchanged. Seed the builder with
/// [`Self::from_current`] (typically the result of
/// [`crate::client::RiskClient::get_params`]) before overriding just the
/// sub-configs you intend to change.
#[derive(Default)]
pub struct UpdateParamsBuilder {
    authority: Option<String>,
    config: Option<RiskConfig>,
    auction_params: Option<morpheum_proto::risk::v1::AuctionParams>,
    spot_risk: Option<morpheum_proto::risk::v1::SpotRiskConfig>,
    spot_collateral: Option<morpheum_proto::risk::v1::SpotCollateralConfig>,
    tiered_margin: Option<morpheum_proto::risk::v1::TieredMarginConfig>,
    portfolio_var: Option<morpheum_proto::risk::v1::PortfolioVarConfig>,
}

impl UpdateParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Seeds every sub-config from the currently active on-chain parameters,
    /// so fields not overridden by a setter below are carried over unchanged
    /// by this full-replace write instead of being cleared.
    pub fn from_current(mut self, params: RiskParams) -> Self {
        self.config = Some(params.config);
        self.auction_params = params.auction_params;
        self.spot_risk = params.spot_risk;
        self.spot_collateral = params.spot_collateral;
        self.tiered_margin = params.tiered_margin;
        self.portfolio_var = params.portfolio_var;
        self
    }

    pub fn authority(mut self, v: impl Into<String>) -> Self {
        self.authority = Some(v.into());
        self
    }
    pub fn config(mut self, v: RiskConfig) -> Self {
        self.config = Some(v);
        self
    }
    pub fn auction_params(mut self, v: morpheum_proto::risk::v1::AuctionParams) -> Self {
        self.auction_params = Some(v);
        self
    }
    pub fn spot_risk(mut self, v: morpheum_proto::risk::v1::SpotRiskConfig) -> Self {
        self.spot_risk = Some(v);
        self
    }
    pub fn spot_collateral(mut self, v: morpheum_proto::risk::v1::SpotCollateralConfig) -> Self {
        self.spot_collateral = Some(v);
        self
    }
    pub fn tiered_margin(mut self, v: morpheum_proto::risk::v1::TieredMarginConfig) -> Self {
        self.tiered_margin = Some(v);
        self
    }
    pub fn portfolio_var(mut self, v: morpheum_proto::risk::v1::PortfolioVarConfig) -> Self {
        self.portfolio_var = Some(v);
        self
    }

    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        let authority = self
            .authority
            .ok_or_else(|| SdkError::invalid_input("authority is required"))?;
        let config = self
            .config
            .ok_or_else(|| SdkError::invalid_input("config is required"))?;
        Ok(UpdateParamsRequest::new(
            authority,
            RiskParams {
                config,
                auction_params: self.auction_params,
                spot_risk: self.spot_risk,
                spot_collateral: self.spot_collateral,
                tiered_margin: self.tiered_margin,
                portfolio_var: self.portfolio_var,
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trigger_liquidation_builder_works() {
        let req = TriggerLiquidationBuilder::new()
            .market_index(0)
            .bucket_id(42)
            .build()
            .unwrap();
        assert_eq!(req.market_index, 0);
        assert_eq!(req.bucket_id, 42);
    }

    #[test]
    fn trigger_liquidation_validation() {
        assert!(TriggerLiquidationBuilder::new().build().is_err());
    }

    #[test]
    fn update_params_builder_works() {
        let req = UpdateParamsBuilder::new()
            .authority("morpheum1gov")
            .config(RiskConfig {
                band_width_bps: 100,
                num_bands_above_below: 10,
                imbalance_threshold_bps: 500,
                imbalance_hysteresis_bps: 100,
                max_scan_limit: 100,
                liquidation_margin_ratio_bps: 500,
                partial_band_shift_enabled: true,
            })
            .build()
            .unwrap();
        assert_eq!(req.authority, "morpheum1gov");
        assert!(req.params.auction_params.is_none());
    }

    #[test]
    fn update_params_validation() {
        assert!(UpdateParamsBuilder::new().build().is_err());
    }

    /// `from_current` must carry every sub-config over so a targeted
    /// single-field override doesn't clear the rest via the full-replace write.
    #[test]
    fn update_params_builder_from_current_preserves_subconfigs() {
        let current = RiskParams {
            config: RiskConfig {
                band_width_bps: 100,
                num_bands_above_below: 10,
                imbalance_threshold_bps: 500,
                imbalance_hysteresis_bps: 100,
                max_scan_limit: 100,
                liquidation_margin_ratio_bps: 500,
                partial_band_shift_enabled: true,
            },
            auction_params: Some(morpheum_proto::risk::v1::AuctionParams {
                duration_blocks: 20,
                initial_premium_bps: 500,
                floor_discount_bps: 500,
                decay_bps_per_block: 25,
            }),
            spot_risk: None,
            spot_collateral: None,
            tiered_margin: None,
            portfolio_var: None,
        };
        let req = UpdateParamsBuilder::new()
            .from_current(current.clone())
            .authority("morpheum1gov")
            .build()
            .unwrap();
        assert_eq!(req.params.auction_params, current.auction_params);
    }
}
