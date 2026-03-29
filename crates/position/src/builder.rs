//! Fluent builders for the Position module.
//!
//! Position lifecycle:
//! - Positions are CREATED from CLOB order fills (not user txs).
//! - Users can close/reduce positions (ClosePositionBuilder) and
//!   change leverage (UpdatePositionLeverageBuilder).

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{ClosePositionRequest, UpdatePositionLeverageRequest};

/// Fluent builder for closing a position. Supports optional `bucket_id`
/// for disambiguation when an address has multiple buckets with positions
/// in the same market (consolidates the old CloseBucketPositionBuilder).
#[derive(Default)]
pub struct ClosePositionBuilder {
    address: Option<String>,
    market_index: Option<u64>,
    exit_price: Option<u64>,
    bucket_id: Option<u64>,
}

impl ClosePositionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn market_index(mut self, index: u64) -> Self {
        self.market_index = Some(index);
        self
    }

    pub fn exit_price(mut self, price: u64) -> Self {
        self.exit_price = Some(price);
        self
    }

    pub fn bucket_id(mut self, bucket_id: u64) -> Self {
        self.bucket_id = Some(bucket_id);
        self
    }

    pub fn build(self) -> Result<ClosePositionRequest, SdkError> {
        let address = self.address.ok_or_else(|| {
            SdkError::invalid_input("address is required to close a position")
        })?;

        let market_index = self.market_index.ok_or_else(|| {
            SdkError::invalid_input("market_index is required to close a position")
        })?;

        let exit_price = self.exit_price.ok_or_else(|| {
            SdkError::invalid_input("exit_price is required to close a position")
        })?;

        let mut req = ClosePositionRequest::new(address, market_index, exit_price);
        if let Some(bid) = self.bucket_id {
            req = req.with_bucket_id(bid);
        }
        Ok(req)
    }
}

/// Fluent builder for updating position leverage.
#[derive(Default)]
pub struct UpdatePositionLeverageBuilder {
    address: Option<String>,
    market_index: Option<u64>,
    new_leverage: Option<String>,
    position_id: Option<String>,
    bucket_id: Option<String>,
}

impl UpdatePositionLeverageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn market_index(mut self, index: u64) -> Self {
        self.market_index = Some(index);
        self
    }

    pub fn new_leverage(mut self, leverage: impl Into<String>) -> Self {
        self.new_leverage = Some(leverage.into());
        self
    }

    pub fn position_id(mut self, id: impl Into<String>) -> Self {
        self.position_id = Some(id.into());
        self
    }

    pub fn bucket_id(mut self, id: impl Into<String>) -> Self {
        self.bucket_id = Some(id.into());
        self
    }

    pub fn build(self) -> Result<UpdatePositionLeverageRequest, SdkError> {
        let address = self.address.ok_or_else(|| {
            SdkError::invalid_input("address is required to update leverage")
        })?;

        let market_index = self.market_index.ok_or_else(|| {
            SdkError::invalid_input("market_index is required to update leverage")
        })?;

        let new_leverage = self.new_leverage.ok_or_else(|| {
            SdkError::invalid_input("new_leverage is required to update leverage")
        })?;

        let mut req = UpdatePositionLeverageRequest::new(address, market_index, new_leverage);
        if let Some(pid) = self.position_id {
            req = req.with_position_id(pid);
        }
        if let Some(bid) = self.bucket_id {
            req = req.with_bucket_id(bid);
        }
        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_position_builder_works() {
        let req = ClosePositionBuilder::new()
            .address("morpheum1abc")
            .market_index(42)
            .exit_price(52000)
            .build()
            .unwrap();

        assert_eq!(req.exit_price, 52000);
        assert_eq!(req.bucket_id, None);
    }

    #[test]
    fn close_position_builder_with_bucket() {
        let req = ClosePositionBuilder::new()
            .address("morpheum1abc")
            .market_index(42)
            .exit_price(52000)
            .bucket_id(123)
            .build()
            .unwrap();

        assert_eq!(req.bucket_id, Some(123));
    }

    #[test]
    fn close_position_builder_validation() {
        assert!(ClosePositionBuilder::new().build().is_err());
    }

    #[test]
    fn update_leverage_builder_works() {
        let req = UpdatePositionLeverageBuilder::new()
            .address("morpheum1abc")
            .market_index(42)
            .new_leverage("20")
            .build()
            .unwrap();

        assert_eq!(req.new_leverage, "20");
        assert_eq!(req.market_index, 42);
    }

    #[test]
    fn update_leverage_builder_with_ids() {
        let req = UpdatePositionLeverageBuilder::new()
            .address("morpheum1abc")
            .market_index(42)
            .new_leverage("20")
            .position_id("pos-123")
            .bucket_id("bucket-1")
            .build()
            .unwrap();

        assert_eq!(req.position_id, Some("pos-123".into()));
        assert_eq!(req.bucket_id, Some("bucket-1".into()));
    }

    #[test]
    fn update_leverage_builder_validation() {
        assert!(UpdatePositionLeverageBuilder::new().build().is_err());
        assert!(UpdatePositionLeverageBuilder::new()
            .address("x")
            .build()
            .is_err());
    }
}
