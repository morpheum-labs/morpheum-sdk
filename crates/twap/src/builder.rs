//! Fluent builder for the TWAP module.

use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::UpdateTwapConfigRequest;
use crate::types::MarketTwapConfig;

/// Builder for `UpdateTwapConfigRequest`.
#[derive(Default)]
pub struct UpdateTwapConfigBuilder {
    market_index: Option<u64>,
    windows: Vec<u32>,
    staleness_blocks: u64,
}

impl UpdateTwapConfigBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn window(mut self, blocks: u32) -> Self { self.windows.push(blocks); self }
    pub fn windows(mut self, v: Vec<u32>) -> Self { self.windows = v; self }
    pub fn staleness_blocks(mut self, v: u64) -> Self { self.staleness_blocks = v; self }

    pub fn build(self) -> Result<UpdateTwapConfigRequest, SdkError> {
        if self.windows.is_empty() {
            return Err(SdkError::invalid_input("at least one window size is required"));
        }
        Ok(UpdateTwapConfigRequest::new(
            self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            MarketTwapConfig { windows: self.windows, staleness_blocks: self.staleness_blocks },
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn builder_works() {
        let req = UpdateTwapConfigBuilder::new()
            .market_index(1)
            .window(60).window(300).window(900)
            .staleness_blocks(10)
            .build().unwrap();
        assert_eq!(req.market_index, 1);
        assert_eq!(req.config.windows, vec![60, 300, 900]);
    }

    #[test]
    fn requires_at_least_one_window() {
        let result = UpdateTwapConfigBuilder::new().market_index(1).build();
        assert!(result.is_err());
    }

    #[test]
    fn requires_market_index() {
        let result = UpdateTwapConfigBuilder::new().window(60).build();
        assert!(result.is_err());
    }

    #[test]
    fn windows_setter_replaces() {
        let req = UpdateTwapConfigBuilder::new()
            .market_index(1).window(60).windows(vec![300, 900])
            .build().unwrap();
        assert_eq!(req.config.windows, vec![300, 900]);
    }
}
