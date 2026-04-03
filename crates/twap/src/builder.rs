//! Fluent builder for the TWAP module.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{UpdateParamsRequest, UpdateTwapConfigRequest};
use crate::types::{MarketTwapConfig, TwapModuleConfig, TwapParams};

/// Builder for `UpdateParamsRequest` (module-level governance parameters).
#[derive(Default)]
pub struct UpdateParamsBuilder {
    authority: Option<String>,
    default_staleness_blocks: Option<u64>,
}

impl UpdateParamsBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn authority(mut self, v: impl Into<String>) -> Self { self.authority = Some(v.into()); self }
    pub fn default_staleness_blocks(mut self, v: u64) -> Self { self.default_staleness_blocks = Some(v); self }

    pub fn params(mut self, p: TwapParams) -> Self {
        self.default_staleness_blocks = Some(p.module_config.default_staleness_blocks);
        self
    }

    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        Ok(UpdateParamsRequest::new(
            self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?,
            TwapParams {
                module_config: TwapModuleConfig {
                    default_staleness_blocks: self
                        .default_staleness_blocks
                        .ok_or_else(|| SdkError::invalid_input("default_staleness_blocks is required"))?,
                },
            },
        ))
    }
}

/// Builder for `UpdateTwapConfigRequest`.
#[derive(Default)]
pub struct UpdateTwapConfigBuilder {
    authority: Option<String>,
    market_index: Option<u64>,
    windows: Vec<u32>,
    staleness_blocks: u64,
}

impl UpdateTwapConfigBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn authority(mut self, v: impl Into<String>) -> Self { self.authority = Some(v.into()); self }
    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn window(mut self, blocks: u32) -> Self { self.windows.push(blocks); self }
    pub fn windows(mut self, v: Vec<u32>) -> Self { self.windows = v; self }
    pub fn staleness_blocks(mut self, v: u64) -> Self { self.staleness_blocks = v; self }

    pub fn build(self) -> Result<UpdateTwapConfigRequest, SdkError> {
        if self.windows.is_empty() {
            return Err(SdkError::invalid_input("at least one window size is required"));
        }
        Ok(UpdateTwapConfigRequest::new(
            self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?,
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
            .authority("morpheum1gov")
            .market_index(1)
            .window(60).window(300).window(900)
            .staleness_blocks(10)
            .build().unwrap();
        assert_eq!(req.authority, "morpheum1gov");
        assert_eq!(req.market_index, 1);
        assert_eq!(req.config.windows, vec![60, 300, 900]);
    }

    #[test]
    fn requires_at_least_one_window() {
        let result = UpdateTwapConfigBuilder::new()
            .authority("morpheum1gov")
            .market_index(1)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn requires_market_index() {
        let result = UpdateTwapConfigBuilder::new()
            .authority("morpheum1gov")
            .window(60)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn requires_authority() {
        let result = UpdateTwapConfigBuilder::new().market_index(1).window(60).build();
        assert!(result.is_err());
    }

    #[test]
    fn windows_setter_replaces() {
        let req = UpdateTwapConfigBuilder::new()
            .authority("morpheum1gov")
            .market_index(1).window(60).windows(vec![300, 900])
            .build().unwrap();
        assert_eq!(req.config.windows, vec![300, 900]);
    }
}
