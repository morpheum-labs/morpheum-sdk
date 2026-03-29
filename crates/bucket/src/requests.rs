//! Request wrappers for the Bucket module.
//!
//! Transaction requests include `to_any()` for seamless integration with
//! `TxBuilder`. Query requests convert to proto via `From` impls.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::bucket::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::BucketType;

// ====================== TRANSACTION REQUESTS ======================

/// Request to create a new margin bucket.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateBucketRequest {
    pub address: String,
    pub bucket_id: String,
    pub bucket_type: BucketType,
    pub collateral_asset_index: u64,
    pub initial_margin: String,
}

impl CreateBucketRequest {
    pub fn new(
        address: impl Into<String>,
        bucket_id: impl Into<String>,
        bucket_type: BucketType,
        collateral_asset_index: u64,
        initial_margin: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            bucket_id: bucket_id.into(),
            bucket_type,
            collateral_asset_index,
            initial_margin: initial_margin.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCreateBucketRequest = self.clone().into();
        ProtoAny {
            type_url: "/bucket.v1.MsgCreateBucketRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CreateBucketRequest> for proto::MsgCreateBucketRequest {
    fn from(req: CreateBucketRequest) -> Self {
        Self {
            address: req.address,
            bucket_id: req.bucket_id,
            bucket_type: i32::from(req.bucket_type),
            collateral_asset_index: req.collateral_asset_index,
            initial_margin: req.initial_margin,
            timestamp: None,
        }
    }
}

/// Request to transfer margin between buckets.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransferBetweenBucketsRequest {
    pub address: String,
    pub source_bucket_id: String,
    pub target_bucket_id: String,
    pub amount: String,
    pub reason: Option<String>,
}

impl TransferBetweenBucketsRequest {
    pub fn new(
        address: impl Into<String>,
        source_bucket_id: impl Into<String>,
        target_bucket_id: impl Into<String>,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            source_bucket_id: source_bucket_id.into(),
            target_bucket_id: target_bucket_id.into(),
            amount: amount.into(),
            reason: None,
        }
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgTransferBetweenBucketsRequest = self.clone().into();
        ProtoAny {
            type_url: "/bucket.v1.MsgTransferBetweenBucketsRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<TransferBetweenBucketsRequest> for proto::MsgTransferBetweenBucketsRequest {
    fn from(req: TransferBetweenBucketsRequest) -> Self {
        Self {
            address: req.address,
            source_bucket_id: req.source_bucket_id,
            target_bucket_id: req.target_bucket_id,
            amount: req.amount,
            reason: req.reason.unwrap_or_default(),
            timestamp: None,
        }
    }
}

/// Request to transfer funds from a bucket to the bank module.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransferToBankRequest {
    pub address: String,
    pub from_address: String,
    pub bucket_id: String,
    pub asset_index: u64,
    pub amount: String,
}

impl TransferToBankRequest {
    pub fn new(
        address: impl Into<String>,
        from_address: impl Into<String>,
        bucket_id: impl Into<String>,
        asset_index: u64,
        amount: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            from_address: from_address.into(),
            bucket_id: bucket_id.into(),
            asset_index,
            amount: amount.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgTransferToBankRequest = self.clone().into();
        ProtoAny {
            type_url: "/bucket.v1.MsgTransferToBankRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<TransferToBankRequest> for proto::MsgTransferToBankRequest {
    fn from(req: TransferToBankRequest) -> Self {
        Self {
            address: req.address,
            from_address: req.from_address,
            bucket_id: req.bucket_id,
            asset_index: req.asset_index,
            amount: req.amount,
            timestamp: None,
        }
    }
}

/// Request to close an empty bucket and return remaining margin to spot.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CloseBucketRequest {
    pub address: String,
    pub bucket_id: String,
}

impl CloseBucketRequest {
    pub fn new(address: impl Into<String>, bucket_id: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            bucket_id: bucket_id.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCloseBucketRequest = self.clone().into();
        ProtoAny {
            type_url: "/bucket.v1.MsgCloseBucketRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CloseBucketRequest> for proto::MsgCloseBucketRequest {
    fn from(req: CloseBucketRequest) -> Self {
        Self {
            address: req.address,
            bucket_id: req.bucket_id,
            timestamp: None,
        }
    }
}

/// Request to liquidate a bucket (permissioned: risk module / governance / keeper bot).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiquidateBucketRequest {
    pub from_address: String,
    pub bucket_id: String,
    pub market_index: u64,
    pub liquidation_price: String,
    pub reason: Option<String>,
}

impl LiquidateBucketRequest {
    pub fn new(
        from_address: impl Into<String>,
        bucket_id: impl Into<String>,
        market_index: u64,
        liquidation_price: impl Into<String>,
    ) -> Self {
        Self {
            from_address: from_address.into(),
            bucket_id: bucket_id.into(),
            market_index,
            liquidation_price: liquidation_price.into(),
            reason: None,
        }
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgLiquidateBucketRequest = self.clone().into();
        ProtoAny {
            type_url: "/bucket.v1.MsgLiquidateBucketRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<LiquidateBucketRequest> for proto::MsgLiquidateBucketRequest {
    fn from(req: LiquidateBucketRequest) -> Self {
        Self {
            from_address: req.from_address,
            bucket_id: req.bucket_id,
            market_index: req.market_index,
            liquidation_price: req.liquidation_price,
            reason: req.reason.unwrap_or_default(),
        }
    }
}

/// Request to execute auto-deleveraging (permissioned: risk module / governance).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecuteAdlRequest {
    pub from_address: String,
    pub market_index: u64,
    pub mark_price: String,
    pub oi_imbalance: String,
    pub trigger_reason: String,
}

impl ExecuteAdlRequest {
    pub fn new(
        from_address: impl Into<String>,
        market_index: u64,
        mark_price: impl Into<String>,
    ) -> Self {
        Self {
            from_address: from_address.into(),
            market_index,
            mark_price: mark_price.into(),
            oi_imbalance: String::new(),
            trigger_reason: String::new(),
        }
    }

    pub fn oi_imbalance(mut self, imbalance: impl Into<String>) -> Self {
        self.oi_imbalance = imbalance.into();
        self
    }

    pub fn trigger_reason(mut self, reason: impl Into<String>) -> Self {
        self.trigger_reason = reason.into();
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgExecuteAdlRequest = self.clone().into();
        ProtoAny {
            type_url: "/bucket.v1.MsgExecuteADLRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ExecuteAdlRequest> for proto::MsgExecuteAdlRequest {
    fn from(req: ExecuteAdlRequest) -> Self {
        Self {
            from_address: req.from_address,
            market_index: req.market_index,
            mark_price: req.mark_price,
            oi_imbalance: req.oi_imbalance,
            trigger_reason: req.trigger_reason,
        }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query a specific bucket by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryBucketRequest {
    pub bucket_id: String,
}

impl QueryBucketRequest {
    pub fn new(bucket_id: impl Into<String>) -> Self {
        Self { bucket_id: bucket_id.into() }
    }
}

impl From<QueryBucketRequest> for proto::QueryBucketRequest {
    fn from(req: QueryBucketRequest) -> Self {
        Self { bucket_id: req.bucket_id }
    }
}

/// Query buckets by address with optional type filter.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryBucketsByAddressRequest {
    pub address: String,
    pub type_filter: Option<BucketType>,
}

impl QueryBucketsByAddressRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self { address: address.into(), type_filter: None }
    }

    pub fn type_filter(mut self, bucket_type: BucketType) -> Self {
        self.type_filter = Some(bucket_type);
        self
    }
}

impl From<QueryBucketsByAddressRequest> for proto::QueryBucketsByAddressRequest {
    fn from(req: QueryBucketsByAddressRequest) -> Self {
        Self {
            address: req.address,
            type_filter: req.type_filter.map(i32::from),
        }
    }
}

/// Query aggregated PnL for an address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryAddressPnLRequest {
    pub address: String,
}

impl QueryAddressPnLRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self { address: address.into() }
    }
}

impl From<QueryAddressPnLRequest> for proto::QueryAddressPnLRequest {
    fn from(req: QueryAddressPnLRequest) -> Self {
        Self { address: req.address }
    }
}

/// Query PnL for a specific bucket.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryBucketPnLRequest {
    pub bucket_id: String,
}

impl QueryBucketPnLRequest {
    pub fn new(bucket_id: impl Into<String>) -> Self {
        Self { bucket_id: bucket_id.into() }
    }
}

impl From<QueryBucketPnLRequest> for proto::QueryBucketPnLRequest {
    fn from(req: QueryBucketPnLRequest) -> Self {
        Self { bucket_id: req.bucket_id }
    }
}

/// Query positions within a specific bucket.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPositionsByBucketRequest {
    pub bucket_id: String,
}

impl QueryPositionsByBucketRequest {
    pub fn new(bucket_id: impl Into<String>) -> Self {
        Self { bucket_id: bucket_id.into() }
    }
}

impl From<QueryPositionsByBucketRequest> for proto::QueryPositionsByBucketRequest {
    fn from(req: QueryPositionsByBucketRequest) -> Self {
        Self { bucket_id: req.bucket_id }
    }
}

/// Query liquidation events with optional filters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryLiquidationsRequest {
    pub market_index: u64,
    pub address: Option<String>,
    pub start_time: i64,
    pub end_time: i64,
    pub limit: i32,
}

impl QueryLiquidationsRequest {
    pub fn new(limit: i32) -> Self {
        Self { market_index: 0, address: None, start_time: 0, end_time: 0, limit }
    }

    pub fn market_index(mut self, index: u64) -> Self {
        self.market_index = index;
        self
    }

    pub fn address(mut self, addr: impl Into<String>) -> Self {
        self.address = Some(addr.into());
        self
    }

    pub fn time_range(mut self, start: i64, end: i64) -> Self {
        self.start_time = start;
        self.end_time = end;
        self
    }
}

impl From<QueryLiquidationsRequest> for proto::QueryLiquidationsRequest {
    fn from(req: QueryLiquidationsRequest) -> Self {
        Self {
            market_index: req.market_index,
            address: req.address.unwrap_or_default(),
            start_time: req.start_time,
            end_time: req.end_time,
            limit: req.limit,
        }
    }
}

/// Query bucket status.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryBucketStatusRequest {
    pub address: String,
    pub bucket_id: String,
}

impl QueryBucketStatusRequest {
    pub fn new(address: impl Into<String>, bucket_id: impl Into<String>) -> Self {
        Self { address: address.into(), bucket_id: bucket_id.into() }
    }
}

impl From<QueryBucketStatusRequest> for proto::QueryBucketStatusRequest {
    fn from(req: QueryBucketStatusRequest) -> Self {
        Self { address: req.address, bucket_id: req.bucket_id }
    }
}

/// Query all bucket balances for an address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryAllBucketsBalanceByAddressRequest {
    pub address: String,
}

impl QueryAllBucketsBalanceByAddressRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self { address: address.into() }
    }
}

impl From<QueryAllBucketsBalanceByAddressRequest> for proto::QueryAllBucketsBalanceByAddressRequest {
    fn from(req: QueryAllBucketsBalanceByAddressRequest) -> Self {
        Self { address: req.address }
    }
}

/// Query position health.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPositionHealthRequest {
    pub address: String,
    pub market_index: u64,
    pub current_price: String,
}

impl QueryPositionHealthRequest {
    pub fn new(
        address: impl Into<String>,
        market_index: u64,
        current_price: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            market_index,
            current_price: current_price.into(),
        }
    }
}

impl From<QueryPositionHealthRequest> for proto::QueryPositionHealthRequest {
    fn from(req: QueryPositionHealthRequest) -> Self {
        Self {
            address: req.address,
            market_index: req.market_index,
            current_price: req.current_price,
        }
    }
}

/// Query liquidation metrics.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryLiquidationMetricsRequest {
    pub start_time: i64,
    pub end_time: i64,
}

impl QueryLiquidationMetricsRequest {
    pub fn new(start_time: i64, end_time: i64) -> Self {
        Self { start_time, end_time }
    }
}

impl From<QueryLiquidationMetricsRequest> for proto::QueryLiquidationMetricsRequest {
    fn from(req: QueryLiquidationMetricsRequest) -> Self {
        Self { start_time: req.start_time, end_time: req.end_time }
    }
}

/// Query ADL history.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryAdlHistoryRequest {
    pub market_index: u64,
    pub address: Option<String>,
    pub limit: i32,
    pub offset: i32,
}

impl QueryAdlHistoryRequest {
    pub fn new(limit: i32, offset: i32) -> Self {
        Self { market_index: 0, address: None, limit, offset }
    }

    pub fn market_index(mut self, index: u64) -> Self {
        self.market_index = index;
        self
    }

    pub fn address(mut self, addr: impl Into<String>) -> Self {
        self.address = Some(addr.into());
        self
    }
}

impl From<QueryAdlHistoryRequest> for proto::QueryAdlHistoryRequest {
    fn from(req: QueryAdlHistoryRequest) -> Self {
        Self {
            market_index: req.market_index,
            address: req.address.unwrap_or_default(),
            from_time: None,
            to_time: None,
            limit: req.limit,
            offset: req.offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_bucket_request_to_any() {
        let req = CreateBucketRequest::new(
            "morpheum1abc", "bucket-1", BucketType::Cross, 4, "100000000000",
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/bucket.v1.MsgCreateBucketRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn transfer_between_buckets_to_any() {
        let req = TransferBetweenBucketsRequest::new(
            "morpheum1abc", "bucket-1", "bucket-2", "50000000000",
        )
        .reason("rebalance");
        let any = req.to_any();
        assert_eq!(any.type_url, "/bucket.v1.MsgTransferBetweenBucketsRequest");
    }

    #[test]
    fn close_bucket_request_to_any() {
        let req = CloseBucketRequest::new("morpheum1abc", "bucket-1");
        let any = req.to_any();
        assert_eq!(any.type_url, "/bucket.v1.MsgCloseBucketRequest");
    }

    #[test]
    fn liquidate_bucket_to_any() {
        let req = LiquidateBucketRequest::new("morpheum1risk", "bucket-1", 42, "48000")
            .reason("equity ratio breach");
        let any = req.to_any();
        assert_eq!(any.type_url, "/bucket.v1.MsgLiquidateBucketRequest");
    }

    #[test]
    fn query_bucket_request_proto_conversion() {
        let req = QueryBucketRequest::new("bucket-1");
        let proto_req: proto::QueryBucketRequest = req.into();
        assert_eq!(proto_req.bucket_id, "bucket-1");
    }

    #[test]
    fn query_buckets_by_address_with_filter() {
        let req = QueryBucketsByAddressRequest::new("morpheum1abc")
            .type_filter(BucketType::Cross);
        let proto_req: proto::QueryBucketsByAddressRequest = req.into();
        assert_eq!(proto_req.address, "morpheum1abc");
        assert_eq!(proto_req.type_filter, Some(proto::BucketType::Cross as i32));
    }

}
