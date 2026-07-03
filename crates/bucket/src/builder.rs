//! Fluent builders for the Bucket module.
//!
//! Provides ergonomic, type-safe builders for all bucket transaction operations.
//! Each builder validates required fields and returns the corresponding request
//! type from `requests.rs` for integration with `TxBuilder`.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    CloseBucketRequest, CreateBucketRequest, TransferBetweenBucketsRequest, TransferToBankRequest,
};
use crate::types::BucketType;

// ====================== CREATE BUCKET ======================

/// Fluent builder for creating a new margin bucket.
#[derive(Default)]
pub struct CreateBucketBuilder {
    bucket_id: Option<String>,
    bucket_type: Option<BucketType>,
    collateral_asset_index: Option<u64>,
    initial_margin: Option<String>,
}

impl CreateBucketBuilder {
    pub fn new() -> Self {
        Self::default()
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
        let bucket_id = self
            .bucket_id
            .ok_or_else(|| SdkError::invalid_input("bucket_id is required"))?;
        let bucket_type = self
            .bucket_type
            .ok_or_else(|| SdkError::invalid_input("bucket_type is required"))?;
        let collateral_asset_index = self
            .collateral_asset_index
            .ok_or_else(|| SdkError::invalid_input("collateral_asset_index is required"))?;
        let initial_margin = self
            .initial_margin
            .ok_or_else(|| SdkError::invalid_input("initial_margin is required"))?;

        Ok(CreateBucketRequest::new(
            bucket_id,
            bucket_type,
            collateral_asset_index,
            initial_margin,
        ))
    }
}

// ====================== TRANSFER BETWEEN BUCKETS ======================

/// Fluent builder for transferring margin between buckets.
#[derive(Default)]
pub struct TransferBetweenBucketsBuilder {
    source_bucket_id: Option<String>,
    target_bucket_id: Option<String>,
    amount: Option<String>,
    reason: Option<String>,
}

impl TransferBetweenBucketsBuilder {
    pub fn new() -> Self {
        Self::default()
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
        let source = self
            .source_bucket_id
            .ok_or_else(|| SdkError::invalid_input("source_bucket_id is required"))?;
        let target = self
            .target_bucket_id
            .ok_or_else(|| SdkError::invalid_input("target_bucket_id is required"))?;
        let amount = self
            .amount
            .ok_or_else(|| SdkError::invalid_input("amount is required"))?;

        let mut req = TransferBetweenBucketsRequest::new(source, target, amount);
        if let Some(r) = self.reason {
            req = req.reason(r);
        }
        Ok(req)
    }
}

// ====================== TRANSFER TO BANK ======================

/// Fluent builder for withdrawing funds from a bucket to the bank module.
#[derive(Default)]
pub struct TransferToBankBuilder {
    bucket_id: Option<String>,
    asset_index: Option<u64>,
    amount: Option<String>,
}

impl TransferToBankBuilder {
    pub fn new() -> Self {
        Self::default()
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
        let bucket_id = self
            .bucket_id
            .ok_or_else(|| SdkError::invalid_input("bucket_id is required"))?;
        let asset_index = self
            .asset_index
            .ok_or_else(|| SdkError::invalid_input("asset_index is required"))?;
        let amount = self
            .amount
            .ok_or_else(|| SdkError::invalid_input("amount is required"))?;

        Ok(TransferToBankRequest::new(bucket_id, asset_index, amount))
    }
}

// ====================== CLOSE BUCKET ======================

/// Fluent builder for closing an empty bucket.
#[derive(Default)]
pub struct CloseBucketBuilder {
    bucket_id: Option<String>,
}

impl CloseBucketBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bucket_id(mut self, id: impl Into<String>) -> Self {
        self.bucket_id = Some(id.into());
        self
    }

    pub fn build(self) -> Result<CloseBucketRequest, SdkError> {
        let bucket_id = self
            .bucket_id
            .ok_or_else(|| SdkError::invalid_input("bucket_id is required"))?;

        Ok(CloseBucketRequest::new(bucket_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_bucket_builder_full_flow() {
        let req = CreateBucketBuilder::new()
            .bucket_id("bucket-1")
            .bucket_type(BucketType::Cross)
            .collateral_asset_index(4)
            .initial_margin("100000000000")
            .build()
            .unwrap();

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
    fn transfer_between_buckets_builder_works() {
        let req = TransferBetweenBucketsBuilder::new()
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
            .bucket_id("bucket-1")
            .build()
            .unwrap();

        assert_eq!(req.bucket_id, "bucket-1");
    }
}
