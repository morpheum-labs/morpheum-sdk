//! Fluent builder for the mark price module.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::UpdateMarkConfigRequest;
use crate::types::MarkConfig;

/// Fluent builder for updating a market's mark price configuration.
#[derive(Default)]
pub struct UpdateMarkConfigBuilder {
    authority: Option<String>,
    market_index: Option<u64>,
    weight_twap_bps: Option<u32>,
    weight_oracle_index_bps: Option<u32>,
    weight_kline_bps: Option<u32>,
    staleness_blocks: u64,
    strategy: Option<String>,
}

impl UpdateMarkConfigBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn authority(mut self, v: impl Into<String>) -> Self { self.authority = Some(v.into()); self }
    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn weight_twap_bps(mut self, v: u32) -> Self { self.weight_twap_bps = Some(v); self }
    pub fn weight_oracle_index_bps(mut self, v: u32) -> Self { self.weight_oracle_index_bps = Some(v); self }
    pub fn weight_kline_bps(mut self, v: u32) -> Self { self.weight_kline_bps = Some(v); self }
    pub fn staleness_blocks(mut self, v: u64) -> Self { self.staleness_blocks = v; self }
    pub fn strategy(mut self, v: impl Into<String>) -> Self { self.strategy = Some(v.into()); self }

    pub fn build(self) -> Result<UpdateMarkConfigRequest, SdkError> {
        let twap = self.weight_twap_bps.ok_or_else(|| SdkError::invalid_input("weight_twap_bps is required"))?;
        let oracle = self.weight_oracle_index_bps.ok_or_else(|| SdkError::invalid_input("weight_oracle_index_bps is required"))?;
        let kline = self.weight_kline_bps.ok_or_else(|| SdkError::invalid_input("weight_kline_bps is required"))?;

        let total = twap + oracle + kline;
        if total != 10_000 {
            return Err(SdkError::invalid_input("weights must sum to 10000 bps"));
        }

        let config = MarkConfig {
            weight_twap_bps: twap,
            weight_oracle_index_bps: oracle,
            weight_kline_bps: kline,
            staleness_blocks: self.staleness_blocks,
            strategy: self.strategy.ok_or_else(|| SdkError::invalid_input("strategy is required"))?,
        };

        Ok(UpdateMarkConfigRequest::new(
            self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?,
            self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            config,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works() {
        let req = UpdateMarkConfigBuilder::new()
            .authority("morpheum1gov")
            .market_index(42)
            .weight_twap_bps(8000).weight_oracle_index_bps(1500).weight_kline_bps(500)
            .strategy("linear_perp")
            .build().unwrap();
        assert_eq!(req.authority, "morpheum1gov");
        assert_eq!(req.market_index, 42);
        assert_eq!(req.config.weight_twap_bps, 8000);
    }

    #[test]
    fn builder_validation_missing_fields() {
        assert!(UpdateMarkConfigBuilder::new().build().is_err());
    }

    #[test]
    fn builder_validation_weights_must_sum_to_10000() {
        let result = UpdateMarkConfigBuilder::new()
            .authority("morpheum1gov")
            .market_index(1)
            .weight_twap_bps(5000).weight_oracle_index_bps(3000).weight_kline_bps(1000)
            .strategy("linear_perp")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn builder_valid_weights() {
        let result = UpdateMarkConfigBuilder::new()
            .authority("morpheum1gov")
            .market_index(1)
            .weight_twap_bps(5000).weight_oracle_index_bps(3000).weight_kline_bps(2000)
            .strategy("spot")
            .build();
        assert!(result.is_ok());
    }
}
