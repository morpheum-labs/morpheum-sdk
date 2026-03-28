//! Fluent builders for the Position module.
//!
//! This module provides ergonomic, type-safe fluent builders for all position
//! operations (open, update, close, close-bucket). Each builder follows the
//! classic Builder pattern and returns the corresponding request type from
//! `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    CloseBucketPositionRequest, ClosePositionRequest, OpenPositionRequest, UpdatePositionRequest,
};
use crate::types::PositionSide;

/// Fluent builder for opening a new position.
#[derive(Default)]
pub struct OpenPositionBuilder {
    address: Option<String>,
    market_index: Option<u64>,
    size: Option<u64>,
    entry_price: Option<u64>,
    side: Option<PositionSide>,
    leverage: Option<u32>,
    power: Option<u32>,
}

impl OpenPositionBuilder {
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

    pub fn size(mut self, size: u64) -> Self {
        self.size = Some(size);
        self
    }

    pub fn entry_price(mut self, price: u64) -> Self {
        self.entry_price = Some(price);
        self
    }

    pub fn side(mut self, side: PositionSide) -> Self {
        self.side = Some(side);
        self
    }

    pub fn leverage(mut self, leverage: u32) -> Self {
        self.leverage = Some(leverage);
        self
    }

    pub fn power(mut self, power: u32) -> Self {
        self.power = Some(power);
        self
    }

    pub fn build(self) -> Result<OpenPositionRequest, SdkError> {
        let address = self
            .address
            .ok_or_else(|| SdkError::invalid_input("address is required to open a position"))?;

        let market_index = self.market_index.ok_or_else(|| {
            SdkError::invalid_input("market_index is required to open a position")
        })?;

        let size = self
            .size
            .ok_or_else(|| SdkError::invalid_input("size is required to open a position"))?;

        let entry_price = self.entry_price.ok_or_else(|| {
            SdkError::invalid_input("entry_price is required to open a position")
        })?;

        let side = self
            .side
            .ok_or_else(|| SdkError::invalid_input("side is required to open a position"))?;

        let leverage = self
            .leverage
            .ok_or_else(|| SdkError::invalid_input("leverage is required to open a position"))?;

        let mut req = OpenPositionRequest::new(address, market_index, size, entry_price, side, leverage);
        if let Some(power) = self.power {
            req = req.with_power(power);
        }
        Ok(req)
    }
}

/// Fluent builder for updating an existing position.
#[derive(Default)]
pub struct UpdatePositionBuilder {
    address: Option<String>,
    market_index: Option<u64>,
    size_delta: Option<i64>,
    price: Option<u64>,
}

impl UpdatePositionBuilder {
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

    /// Positive to increase, negative to reduce.
    pub fn size_delta(mut self, delta: i64) -> Self {
        self.size_delta = Some(delta);
        self
    }

    pub fn price(mut self, price: u64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn build(self) -> Result<UpdatePositionRequest, SdkError> {
        let address = self.address.ok_or_else(|| {
            SdkError::invalid_input("address is required to update a position")
        })?;

        let market_index = self.market_index.ok_or_else(|| {
            SdkError::invalid_input("market_index is required to update a position")
        })?;

        let size_delta = self.size_delta.ok_or_else(|| {
            SdkError::invalid_input("size_delta is required to update a position")
        })?;

        let price = self.price.ok_or_else(|| {
            SdkError::invalid_input("price is required to update a position")
        })?;

        Ok(UpdatePositionRequest::new(address, market_index, size_delta, price))
    }
}

/// Fluent builder for closing an entire position.
#[derive(Default)]
pub struct ClosePositionBuilder {
    address: Option<String>,
    market_index: Option<u64>,
    exit_price: Option<u64>,
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

        Ok(ClosePositionRequest::new(address, market_index, exit_price))
    }
}

/// Fluent builder for closing a position within a specific bucket.
#[derive(Default)]
pub struct CloseBucketPositionBuilder {
    address: Option<String>,
    bucket_id: Option<u64>,
    market_index: Option<u64>,
    exit_price: Option<u64>,
}

impl CloseBucketPositionBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn bucket_id(mut self, bucket_id: u64) -> Self {
        self.bucket_id = Some(bucket_id);
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

    pub fn build(self) -> Result<CloseBucketPositionRequest, SdkError> {
        let address = self.address.ok_or_else(|| {
            SdkError::invalid_input("address is required to close a bucket position")
        })?;

        let bucket_id = self.bucket_id.ok_or_else(|| {
            SdkError::invalid_input("bucket_id is required to close a bucket position")
        })?;

        let market_index = self.market_index.ok_or_else(|| {
            SdkError::invalid_input("market_index is required to close a bucket position")
        })?;

        let exit_price = self.exit_price.ok_or_else(|| {
            SdkError::invalid_input("exit_price is required to close a bucket position")
        })?;

        Ok(CloseBucketPositionRequest::new(address, bucket_id, market_index, exit_price))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_position_builder_full_flow() {
        let req = OpenPositionBuilder::new()
            .address("morpheum1abc")
            .market_index(42)
            .size(1000)
            .entry_price(50000)
            .side(PositionSide::Long)
            .leverage(10)
            .power(2)
            .build()
            .unwrap();

        assert_eq!(req.address, "morpheum1abc");
        assert_eq!(req.market_index, 42);
        assert_eq!(req.size, 1000);
        assert_eq!(req.entry_price, 50000);
        assert_eq!(req.side, PositionSide::Long);
        assert_eq!(req.leverage, 10);
        assert_eq!(req.power, 2);
    }

    #[test]
    fn open_position_builder_validation() {
        assert!(OpenPositionBuilder::new().build().is_err());
        assert!(OpenPositionBuilder::new().address("x").build().is_err());
    }

    #[test]
    fn update_position_builder_works() {
        let req = UpdatePositionBuilder::new()
            .address("morpheum1abc")
            .market_index(42)
            .size_delta(-500)
            .price(51000)
            .build()
            .unwrap();

        assert_eq!(req.size_delta, -500);
        assert_eq!(req.price, 51000);
    }

    #[test]
    fn update_position_builder_validation() {
        assert!(UpdatePositionBuilder::new().build().is_err());
    }

    #[test]
    fn close_position_builder_works() {
        let req = ClosePositionBuilder::new()
            .address("morpheum1abc")
            .market_index(42)
            .exit_price(52000)
            .build()
            .unwrap();

        assert_eq!(req.exit_price, 52000);
    }

    #[test]
    fn close_position_builder_validation() {
        assert!(ClosePositionBuilder::new().build().is_err());
    }

    #[test]
    fn close_bucket_position_builder_works() {
        let req = CloseBucketPositionBuilder::new()
            .address("morpheum1abc")
            .bucket_id(1)
            .market_index(42)
            .exit_price(52000)
            .build()
            .unwrap();

        assert_eq!(req.bucket_id, 1);
        assert_eq!(req.market_index, 42);
        assert_eq!(req.exit_price, 52000);
    }

    #[test]
    fn close_bucket_position_builder_validation() {
        assert!(CloseBucketPositionBuilder::new().build().is_err());
        assert!(CloseBucketPositionBuilder::new()
            .address("x")
            .bucket_id(1)
            .build()
            .is_err());
    }
}
