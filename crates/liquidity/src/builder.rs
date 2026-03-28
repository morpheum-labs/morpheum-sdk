//! Fluent builders for liquidity pool transactions.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{CreatePoolRequest, RebalancePoolRequest, UpdatePoolParamsRequest};

// ====================== CREATE POOL ======================

/// Fluent builder for creating a new liquidity pool.
#[derive(Default)]
pub struct CreatePoolBuilder {
    market_index: Option<u64>,
    asset_index: Option<u64>,
    initial_liquidity: Option<String>,
    provider_type: Option<u32>,
    provider_config: Vec<u8>,
    display_name: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    logo_uri: Option<String>,
}

impl CreatePoolBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn initial_liquidity(mut self, v: impl Into<String>) -> Self { self.initial_liquidity = Some(v.into()); self }
    pub fn provider_type(mut self, v: u32) -> Self { self.provider_type = Some(v); self }
    pub fn provider_config(mut self, v: Vec<u8>) -> Self { self.provider_config = v; self }
    pub fn display_name(mut self, v: impl Into<String>) -> Self { self.display_name = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn tags(mut self, v: Vec<String>) -> Self { self.tags = v; self }
    pub fn logo_uri(mut self, v: impl Into<String>) -> Self { self.logo_uri = Some(v.into()); self }

    pub fn build(self) -> Result<CreatePoolRequest, SdkError> {
        let mut req = CreatePoolRequest::new(
            self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.initial_liquidity.ok_or_else(|| SdkError::invalid_input("initial_liquidity is required"))?,
            self.provider_type.ok_or_else(|| SdkError::invalid_input("provider_type is required"))?,
        );
        req.provider_config = self.provider_config;
        req.display_name = self.display_name;
        req.description = self.description;
        req.tags = self.tags;
        req.logo_uri = self.logo_uri;
        Ok(req)
    }
}

// ====================== UPDATE POOL PARAMS ======================

/// Fluent builder for updating pool parameters.
#[derive(Default)]
pub struct UpdatePoolParamsBuilder {
    pool_id: Option<String>,
    min_deposit: Option<String>,
    max_deposit: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    logo_uri: Option<String>,
}

impl UpdatePoolParamsBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn pool_id(mut self, v: impl Into<String>) -> Self { self.pool_id = Some(v.into()); self }
    pub fn min_deposit(mut self, v: impl Into<String>) -> Self { self.min_deposit = Some(v.into()); self }
    pub fn max_deposit(mut self, v: impl Into<String>) -> Self { self.max_deposit = Some(v.into()); self }
    pub fn display_name(mut self, v: impl Into<String>) -> Self { self.display_name = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn tags(mut self, v: Vec<String>) -> Self { self.tags = v; self }
    pub fn logo_uri(mut self, v: impl Into<String>) -> Self { self.logo_uri = Some(v.into()); self }

    pub fn build(self) -> Result<UpdatePoolParamsRequest, SdkError> {
        let mut req = UpdatePoolParamsRequest::new(
            self.pool_id.ok_or_else(|| SdkError::invalid_input("pool_id is required"))?,
            self.min_deposit.ok_or_else(|| SdkError::invalid_input("min_deposit is required"))?,
            self.max_deposit.ok_or_else(|| SdkError::invalid_input("max_deposit is required"))?,
        );
        req.display_name = self.display_name;
        req.description = self.description;
        req.tags = self.tags;
        req.logo_uri = self.logo_uri;
        Ok(req)
    }
}

// ====================== REBALANCE POOL ======================

/// Fluent builder for rebalancing pool depth.
#[derive(Default)]
pub struct RebalancePoolBuilder {
    pool_id: Option<String>,
    target_liquidity: Option<String>,
}

impl RebalancePoolBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn pool_id(mut self, v: impl Into<String>) -> Self { self.pool_id = Some(v.into()); self }
    pub fn target_liquidity(mut self, v: impl Into<String>) -> Self { self.target_liquidity = Some(v.into()); self }

    pub fn build(self) -> Result<RebalancePoolRequest, SdkError> {
        Ok(RebalancePoolRequest::new(
            self.pool_id.ok_or_else(|| SdkError::invalid_input("pool_id is required"))?,
            self.target_liquidity.ok_or_else(|| SdkError::invalid_input("target_liquidity is required"))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_pool_builder_works() {
        let req = CreatePoolBuilder::new()
            .market_index(1).asset_index(2).initial_liquidity("1000").provider_type(1)
            .display_name("Main")
            .build().unwrap();
        assert_eq!(req.market_index, 1);
        assert_eq!(req.display_name, Some("Main".into()));
    }

    #[test]
    fn create_pool_builder_validation() {
        assert!(CreatePoolBuilder::new().build().is_err());
    }

    #[test]
    fn update_pool_params_builder_works() {
        let req = UpdatePoolParamsBuilder::new()
            .pool_id("pool1").min_deposit("100").max_deposit("10000")
            .build().unwrap();
        assert_eq!(req.pool_id, "pool1");
    }

    #[test]
    fn update_pool_params_builder_validation() {
        assert!(UpdatePoolParamsBuilder::new().build().is_err());
    }

    #[test]
    fn rebalance_pool_builder_works() {
        let req = RebalancePoolBuilder::new()
            .pool_id("pool1").target_liquidity("2000")
            .build().unwrap();
        assert_eq!(req.target_liquidity, "2000");
    }

    #[test]
    fn rebalance_pool_builder_validation() {
        assert!(RebalancePoolBuilder::new().build().is_err());
    }
}
