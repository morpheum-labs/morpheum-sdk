//! Fluent builders for the Bucket module.
//!
//! Provides ergonomic, type-safe builders for all bucket transaction operations.
//! Each builder validates required fields and returns the corresponding request
//! type from `requests.rs` for integration with `TxBuilder`.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    CloseBucketRequest, ClosePositionRequest, CreateBucketRequest, ExecuteAdlRequest,
    LiquidatePositionRequest, TransferBetweenBucketsRequest, TransferToBankRequest,
    UpdatePositionLeverageRequest, UpdatePositionRequest,
};
use crate::types::BucketType;

// ====================== CREATE BUCKET ======================

/// Fluent builder for creating a new margin bucket.
#[derive(Default)]
pub struct CreateBucketBuilder {
    address: Option<String>,
    bucket_id: Option<String>,
    bucket_type: Option<BucketType>,
    collateral_asset_index: Option<u64>,
    initial_margin: Option<String>,
}

impl CreateBucketBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn bucket_id(mut self, id: impl Into<String>) -> Self {
        self.bucket_id = Some(id.into());
        self
    }

    pub fn bucket_type(mut self, bt: BucketType) -> Self {
        self.bucket_type = Some(bt);
        self
    }

    pub fn collateral_asset_index(mut self, index: u64) -> Self {
        self.collateral_asset_index = Some(index);
        self
    }

    pub fn initial_margin(mut self, margin: impl Into<String>) -> Self {
        self.initial_margin = Some(margin.into());
        self
    }

    pub fn build(self) -> Result<CreateBucketRequest, SdkError> {
        let address = self.address
            .ok_or_else(|| SdkError::invalid_input("address is required to create a bucket"))?;
        let bucket_id = self.bucket_id
            .ok_or_else(|| SdkError::invalid_input("bucket_id is required"))?;
        let bucket_type = self.bucket_type
            .ok_or_else(|| SdkError::invalid_input("bucket_type is required"))?;
        let collateral_asset_index = self.collateral_asset_index
            .ok_or_else(|| SdkError::invalid_input("collateral_asset_index is required"))?;
        let initial_margin = self.initial_margin
            .ok_or_else(|| SdkError::invalid_input("initial_margin is required"))?;

        Ok(CreateBucketRequest::new(
            address, bucket_id, bucket_type, collateral_asset_index, initial_margin,
        ))
    }
}

// ====================== UPDATE POSITION ======================

/// Fluent builder for updating a position within a bucket.
#[derive(Default)]
pub struct UpdatePositionBuilder {
    address: Option<String>,
    market_index: Option<u64>,
    position_id: Option<String>,
    size_delta: Option<String>,
    leverage: Option<String>,
    price: Option<String>,
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

    pub fn position_id(mut self, id: impl Into<String>) -> Self {
        self.position_id = Some(id.into());
        self
    }

    pub fn size_delta(mut self, delta: impl Into<String>) -> Self {
        self.size_delta = Some(delta.into());
        self
    }

    pub fn leverage(mut self, lev: impl Into<String>) -> Self {
        self.leverage = Some(lev.into());
        self
    }

    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }

    pub fn build(self) -> Result<UpdatePositionRequest, SdkError> {
        let address = self.address
            .ok_or_else(|| SdkError::invalid_input("address is required to update a position"))?;
        let market_index = self.market_index
            .ok_or_else(|| SdkError::invalid_input("market_index is required"))?;

        let mut req = UpdatePositionRequest::new(address, market_index);
        if let Some(pid) = self.position_id { req = req.position_id(pid); }
        if let Some(sd) = self.size_delta { req = req.size_delta(sd); }
        if let Some(l) = self.leverage { req = req.leverage(l); }
        if let Some(p) = self.price { req = req.price(p); }
        Ok(req)
    }
}

// ====================== UPDATE POSITION LEVERAGE ======================

/// Fluent builder for changing a position's leverage.
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
        let address = self.address
            .ok_or_else(|| SdkError::invalid_input("address is required"))?;
        let market_index = self.market_index
            .ok_or_else(|| SdkError::invalid_input("market_index is required"))?;
        let new_leverage = self.new_leverage
            .ok_or_else(|| SdkError::invalid_input("new_leverage is required"))?;

        let mut req = UpdatePositionLeverageRequest::new(address, market_index, new_leverage);
        if let Some(pid) = self.position_id { req = req.position_id(pid); }
        if let Some(bid) = self.bucket_id { req = req.bucket_id(bid); }
        Ok(req)
    }
}

// ====================== CLOSE POSITION ======================

/// Fluent builder for closing a position.
#[derive(Default)]
pub struct ClosePositionBuilder {
    address: Option<String>,
    market_index: Option<u64>,
    close_size: Option<String>,
    market_price: Option<String>,
    position_id: Option<String>,
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

    pub fn close_size(mut self, size: impl Into<String>) -> Self {
        self.close_size = Some(size.into());
        self
    }

    pub fn market_price(mut self, price: impl Into<String>) -> Self {
        self.market_price = Some(price.into());
        self
    }

    pub fn position_id(mut self, id: impl Into<String>) -> Self {
        self.position_id = Some(id.into());
        self
    }

    pub fn build(self) -> Result<ClosePositionRequest, SdkError> {
        let address = self.address
            .ok_or_else(|| SdkError::invalid_input("address is required to close a position"))?;
        let market_index = self.market_index
            .ok_or_else(|| SdkError::invalid_input("market_index is required"))?;
        let close_size = self.close_size
            .ok_or_else(|| SdkError::invalid_input("close_size is required"))?;
        let market_price = self.market_price
            .ok_or_else(|| SdkError::invalid_input("market_price is required"))?;

        let mut req = ClosePositionRequest::new(address, market_index, close_size, market_price);
        if let Some(pid) = self.position_id { req = req.position_id(pid); }
        Ok(req)
    }
}

// ====================== TRANSFER BETWEEN BUCKETS ======================

/// Fluent builder for transferring margin between buckets.
#[derive(Default)]
pub struct TransferBetweenBucketsBuilder {
    address: Option<String>,
    source_bucket_id: Option<String>,
    target_bucket_id: Option<String>,
    amount: Option<String>,
    reason: Option<String>,
}

impl TransferBetweenBucketsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn source_bucket_id(mut self, id: impl Into<String>) -> Self {
        self.source_bucket_id = Some(id.into());
        self
    }

    pub fn target_bucket_id(mut self, id: impl Into<String>) -> Self {
        self.target_bucket_id = Some(id.into());
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn build(self) -> Result<TransferBetweenBucketsRequest, SdkError> {
        let address = self.address
            .ok_or_else(|| SdkError::invalid_input("address is required"))?;
        let source = self.source_bucket_id
            .ok_or_else(|| SdkError::invalid_input("source_bucket_id is required"))?;
        let target = self.target_bucket_id
            .ok_or_else(|| SdkError::invalid_input("target_bucket_id is required"))?;
        let amount = self.amount
            .ok_or_else(|| SdkError::invalid_input("amount is required"))?;

        let mut req = TransferBetweenBucketsRequest::new(address, source, target, amount);
        if let Some(r) = self.reason { req = req.reason(r); }
        Ok(req)
    }
}

// ====================== TRANSFER TO BANK ======================

/// Fluent builder for withdrawing funds from a bucket to the bank module.
#[derive(Default)]
pub struct TransferToBankBuilder {
    address: Option<String>,
    from_address: Option<String>,
    bucket_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
}

impl TransferToBankBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn from_address(mut self, from: impl Into<String>) -> Self {
        self.from_address = Some(from.into());
        self
    }

    pub fn bucket_id(mut self, id: impl Into<String>) -> Self {
        self.bucket_id = Some(id.into());
        self
    }

    pub fn asset_index(mut self, index: u64) -> Self {
        self.asset_index = Some(index);
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn build(self) -> Result<TransferToBankRequest, SdkError> {
        let address = self.address
            .ok_or_else(|| SdkError::invalid_input("address is required"))?;
        let from_address = self.from_address
            .ok_or_else(|| SdkError::invalid_input("from_address is required"))?;
        let bucket_id = self.bucket_id
            .ok_or_else(|| SdkError::invalid_input("bucket_id is required"))?;
        let asset_index = self.asset_index
            .ok_or_else(|| SdkError::invalid_input("asset_index is required"))?;
        let amount = self.amount
            .ok_or_else(|| SdkError::invalid_input("amount is required"))?;

        Ok(TransferToBankRequest::new(address, from_address, bucket_id, asset_index, amount))
    }
}

// ====================== CLOSE BUCKET ======================

/// Fluent builder for closing an empty bucket.
#[derive(Default)]
pub struct CloseBucketBuilder {
    address: Option<String>,
    bucket_id: Option<String>,
}

impl CloseBucketBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn bucket_id(mut self, id: impl Into<String>) -> Self {
        self.bucket_id = Some(id.into());
        self
    }

    pub fn build(self) -> Result<CloseBucketRequest, SdkError> {
        let address = self.address
            .ok_or_else(|| SdkError::invalid_input("address is required to close a bucket"))?;
        let bucket_id = self.bucket_id
            .ok_or_else(|| SdkError::invalid_input("bucket_id is required"))?;

        Ok(CloseBucketRequest::new(address, bucket_id))
    }
}

// ====================== LIQUIDATE POSITION ======================

/// Fluent builder for liquidating a position.
#[derive(Default)]
pub struct LiquidatePositionBuilder {
    address: Option<String>,
    market_index: Option<u64>,
    liquidation_price: Option<String>,
    reason: Option<String>,
}

impl LiquidatePositionBuilder {
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

    pub fn liquidation_price(mut self, price: impl Into<String>) -> Self {
        self.liquidation_price = Some(price.into());
        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn build(self) -> Result<LiquidatePositionRequest, SdkError> {
        let address = self.address
            .ok_or_else(|| SdkError::invalid_input("address is required"))?;
        let market_index = self.market_index
            .ok_or_else(|| SdkError::invalid_input("market_index is required"))?;
        let liquidation_price = self.liquidation_price
            .ok_or_else(|| SdkError::invalid_input("liquidation_price is required"))?;

        let mut req = LiquidatePositionRequest::new(address, market_index, liquidation_price);
        if let Some(r) = self.reason { req = req.reason(r); }
        Ok(req)
    }
}

// ====================== EXECUTE ADL ======================

/// Fluent builder for executing auto-deleveraging.
#[derive(Default)]
pub struct ExecuteAdlBuilder {
    execution_id: Option<String>,
    market_index: Option<u64>,
    symbol: Option<String>,
    mark_price: Option<String>,
}

impl ExecuteAdlBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execution_id(mut self, id: impl Into<String>) -> Self {
        self.execution_id = Some(id.into());
        self
    }

    pub fn market_index(mut self, index: u64) -> Self {
        self.market_index = Some(index);
        self
    }

    pub fn symbol(mut self, sym: impl Into<String>) -> Self {
        self.symbol = Some(sym.into());
        self
    }

    pub fn mark_price(mut self, price: impl Into<String>) -> Self {
        self.mark_price = Some(price.into());
        self
    }

    pub fn build(self) -> Result<ExecuteAdlRequest, SdkError> {
        let execution_id = self.execution_id
            .ok_or_else(|| SdkError::invalid_input("execution_id is required"))?;
        let market_index = self.market_index
            .ok_or_else(|| SdkError::invalid_input("market_index is required"))?;
        let symbol = self.symbol
            .ok_or_else(|| SdkError::invalid_input("symbol is required"))?;
        let mark_price = self.mark_price
            .ok_or_else(|| SdkError::invalid_input("mark_price is required"))?;

        Ok(ExecuteAdlRequest::new(execution_id, market_index, symbol, mark_price))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_bucket_builder_full_flow() {
        let req = CreateBucketBuilder::new()
            .address("morpheum1abc")
            .bucket_id("bucket-1")
            .bucket_type(BucketType::Cross)
            .collateral_asset_index(4)
            .initial_margin("100000000000")
            .build()
            .unwrap();

        assert_eq!(req.address, "morpheum1abc");
        assert_eq!(req.bucket_id, "bucket-1");
        assert_eq!(req.bucket_type, BucketType::Cross);
        assert_eq!(req.collateral_asset_index, 4);
        assert_eq!(req.initial_margin, "100000000000");
    }

    #[test]
    fn create_bucket_builder_validation() {
        assert!(CreateBucketBuilder::new().build().is_err());
    }

    #[test]
    fn update_position_builder_works() {
        let req = UpdatePositionBuilder::new()
            .address("morpheum1abc")
            .market_index(42)
            .size_delta("1000")
            .price("50000")
            .build()
            .unwrap();

        assert_eq!(req.address, "morpheum1abc");
        assert_eq!(req.market_index, 42);
    }

    #[test]
    fn update_position_leverage_builder_works() {
        let req = UpdatePositionLeverageBuilder::new()
            .address("morpheum1abc")
            .market_index(42)
            .new_leverage("20000000000")
            .position_id("pos-1")
            .build()
            .unwrap();

        assert_eq!(req.new_leverage, "20000000000");
        assert_eq!(req.position_id, Some("pos-1".into()));
    }

    #[test]
    fn close_position_builder_works() {
        let req = ClosePositionBuilder::new()
            .address("morpheum1abc")
            .market_index(42)
            .close_size("1000")
            .market_price("52000")
            .position_id("pos-1")
            .build()
            .unwrap();

        assert_eq!(req.close_size, "1000");
        assert_eq!(req.position_id, Some("pos-1".into()));
    }

    #[test]
    fn close_position_builder_validation() {
        assert!(ClosePositionBuilder::new().build().is_err());
        assert!(ClosePositionBuilder::new().address("x").build().is_err());
    }

    #[test]
    fn transfer_between_buckets_builder_works() {
        let req = TransferBetweenBucketsBuilder::new()
            .address("morpheum1abc")
            .source_bucket_id("bucket-1")
            .target_bucket_id("bucket-2")
            .amount("50000000000")
            .reason("rebalance")
            .build()
            .unwrap();

        assert_eq!(req.source_bucket_id, "bucket-1");
        assert_eq!(req.target_bucket_id, "bucket-2");
    }

    #[test]
    fn transfer_to_bank_builder_works() {
        let req = TransferToBankBuilder::new()
            .address("morpheum1abc")
            .from_address("morpheum1abc")
            .bucket_id("bucket-1")
            .asset_index(4)
            .amount("10000000000")
            .build()
            .unwrap();

        assert_eq!(req.bucket_id, "bucket-1");
        assert_eq!(req.asset_index, 4);
    }

    #[test]
    fn close_bucket_builder_works() {
        let req = CloseBucketBuilder::new()
            .address("morpheum1abc")
            .bucket_id("bucket-1")
            .build()
            .unwrap();

        assert_eq!(req.bucket_id, "bucket-1");
    }

    #[test]
    fn liquidate_position_builder_works() {
        let req = LiquidatePositionBuilder::new()
            .address("morpheum1abc")
            .market_index(42)
            .liquidation_price("48000")
            .reason("below maintenance margin")
            .build()
            .unwrap();

        assert_eq!(req.liquidation_price, "48000");
        assert_eq!(req.reason, Some("below maintenance margin".into()));
    }

    #[test]
    fn execute_adl_builder_works() {
        let req = ExecuteAdlBuilder::new()
            .execution_id("adl-1")
            .market_index(42)
            .symbol("BTCUSDC")
            .mark_price("50000")
            .build()
            .unwrap();

        assert_eq!(req.execution_id, "adl-1");
        assert_eq!(req.market_index, 42);
    }

    #[test]
    fn execute_adl_builder_validation() {
        assert!(ExecuteAdlBuilder::new().build().is_err());
    }
}
