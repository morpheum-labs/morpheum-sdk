//! Request wrappers for the Bucket module.
//!
//! Transaction requests include `to_any()` for seamless integration with
//! `TxBuilder`. Query requests convert to proto via `From` impls.

use alloc::string::String;
use alloc::vec::Vec;

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

/// Request to update a position within a bucket.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdatePositionRequest {
    pub address: String,
    pub position_id: Option<String>,
    pub market_index: u64,
    pub size_delta: Option<String>,
    pub leverage: Option<String>,
    pub price: Option<String>,
    pub order_id: Option<String>,
    pub trade_id: Option<String>,
}

impl UpdatePositionRequest {
    pub fn new(address: impl Into<String>, market_index: u64) -> Self {
        Self {
            address: address.into(),
            position_id: None,
            market_index,
            size_delta: None,
            leverage: None,
            price: None,
            order_id: None,
            trade_id: None,
        }
    }

    pub fn position_id(mut self, id: impl Into<String>) -> Self {
        self.position_id = Some(id.into());
        self
    }

    pub fn size_delta(mut self, delta: impl Into<String>) -> Self {
        self.size_delta = Some(delta.into());
        self
    }

    pub fn leverage(mut self, leverage: impl Into<String>) -> Self {
        self.leverage = Some(leverage.into());
        self
    }

    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdatePositionRequest = self.clone().into();
        ProtoAny {
            type_url: "/bucket.v1.MsgUpdatePositionRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdatePositionRequest> for proto::MsgUpdatePositionRequest {
    fn from(req: UpdatePositionRequest) -> Self {
        Self {
            address: req.address,
            position_id: req.position_id.unwrap_or_default(),
            market_index: req.market_index,
            size_delta: req.size_delta.unwrap_or_default(),
            leverage: req.leverage.unwrap_or_default(),
            price: req.price.unwrap_or_default(),
            order_id: req.order_id.unwrap_or_default(),
            trade_id: req.trade_id.unwrap_or_default(),
            timestamp: None,
            system_auth_token: String::new(),
        }
    }
}

/// Request to update a position's leverage.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdatePositionLeverageRequest {
    pub address: String,
    pub position_id: Option<String>,
    pub bucket_id: Option<String>,
    pub market_index: u64,
    pub new_leverage: String,
}

impl UpdatePositionLeverageRequest {
    pub fn new(
        address: impl Into<String>,
        market_index: u64,
        new_leverage: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            position_id: None,
            bucket_id: None,
            market_index,
            new_leverage: new_leverage.into(),
        }
    }

    pub fn position_id(mut self, id: impl Into<String>) -> Self {
        self.position_id = Some(id.into());
        self
    }

    pub fn bucket_id(mut self, id: impl Into<String>) -> Self {
        self.bucket_id = Some(id.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdatePositionLeverageRequest = self.clone().into();
        ProtoAny {
            type_url: "/bucket.v1.MsgUpdatePositionLeverageRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdatePositionLeverageRequest> for proto::MsgUpdatePositionLeverageRequest {
    fn from(req: UpdatePositionLeverageRequest) -> Self {
        Self {
            address: req.address,
            position_id: req.position_id.unwrap_or_default(),
            bucket_id: req.bucket_id.unwrap_or_default(),
            market_index: req.market_index,
            new_leverage: req.new_leverage,
            timestamp: None,
        }
    }
}

/// Request to close a position.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClosePositionRequest {
    pub address: String,
    pub market_index: u64,
    pub close_size: String,
    pub market_price: String,
    pub position_id: Option<String>,
}

impl ClosePositionRequest {
    pub fn new(
        address: impl Into<String>,
        market_index: u64,
        close_size: impl Into<String>,
        market_price: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            market_index,
            close_size: close_size.into(),
            market_price: market_price.into(),
            position_id: None,
        }
    }

    pub fn position_id(mut self, id: impl Into<String>) -> Self {
        self.position_id = Some(id.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgClosePositionRequest = self.clone().into();
        ProtoAny {
            type_url: "/bucket.v1.MsgClosePositionRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ClosePositionRequest> for proto::MsgClosePositionRequest {
    fn from(req: ClosePositionRequest) -> Self {
        Self {
            address: req.address,
            market_index: req.market_index,
            close_size: req.close_size,
            market_price: req.market_price,
            position_id: req.position_id.unwrap_or_default(),
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

/// Request to liquidate a position.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiquidatePositionRequest {
    pub address: String,
    pub market_index: u64,
    pub liquidation_price: String,
    pub reason: Option<String>,
}

impl LiquidatePositionRequest {
    pub fn new(
        address: impl Into<String>,
        market_index: u64,
        liquidation_price: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
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
        let msg: proto::MsgLiquidatePositionRequest = self.clone().into();
        ProtoAny {
            type_url: "/bucket.v1.MsgLiquidatePositionRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<LiquidatePositionRequest> for proto::MsgLiquidatePositionRequest {
    fn from(req: LiquidatePositionRequest) -> Self {
        Self {
            address: req.address,
            market_index: req.market_index,
            liquidation_price: req.liquidation_price,
            reason: req.reason.unwrap_or_default(),
        }
    }
}

/// Request to execute auto-deleveraging.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecuteAdlRequest {
    pub execution_id: String,
    pub market_index: u64,
    pub symbol: String,
    pub mark_price: String,
    pub oi_imbalance: String,
    pub insurance_before: String,
    pub insurance_after: String,
    pub affected_addresses: Vec<String>,
    pub positions_processed: i32,
    pub positions_closed: i32,
    pub total_size_closed: String,
    pub total_value_closed: String,
    pub deficit_covered: String,
    pub trigger_reason: String,
    pub status: String,
}

impl ExecuteAdlRequest {
    pub fn new(
        execution_id: impl Into<String>,
        market_index: u64,
        symbol: impl Into<String>,
        mark_price: impl Into<String>,
    ) -> Self {
        Self {
            execution_id: execution_id.into(),
            market_index,
            symbol: symbol.into(),
            mark_price: mark_price.into(),
            oi_imbalance: String::new(),
            insurance_before: String::new(),
            insurance_after: String::new(),
            affected_addresses: Vec::new(),
            positions_processed: 0,
            positions_closed: 0,
            total_size_closed: String::new(),
            total_value_closed: String::new(),
            deficit_covered: String::new(),
            trigger_reason: String::new(),
            status: String::new(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgAdlExecutionRequest = self.clone().into();
        ProtoAny {
            type_url: "/bucket.v1.MsgADLExecutionRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ExecuteAdlRequest> for proto::MsgAdlExecutionRequest {
    fn from(req: ExecuteAdlRequest) -> Self {
        Self {
            execution_id: req.execution_id,
            market_index: req.market_index,
            symbol: req.symbol,
            mark_price: req.mark_price,
            oi_imbalance: req.oi_imbalance,
            insurance_before: req.insurance_before,
            insurance_after: req.insurance_after,
            affected_addresses: req.affected_addresses,
            positions_processed: req.positions_processed,
            positions_closed: req.positions_closed,
            total_size_closed: req.total_size_closed,
            total_value_closed: req.total_value_closed,
            deficit_covered: req.deficit_covered,
            trigger_reason: req.trigger_reason,
            status: req.status,
            start_time: None,
            end_time: None,
            timestamp: None,
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

/// Query positions for an address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPositionsByAddressRequest {
    pub address: String,
    pub active_only: bool,
}

impl QueryPositionsByAddressRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self { address: address.into(), active_only: false }
    }

    pub fn active_only(mut self, active: bool) -> Self {
        self.active_only = active;
        self
    }
}

impl From<QueryPositionsByAddressRequest> for proto::QueryPositionsByAddressRequest {
    fn from(req: QueryPositionsByAddressRequest) -> Self {
        Self { address: req.address, active_only: req.active_only }
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

/// Query PnL for a specific position.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPositionPnLRequest {
    pub address: String,
    pub market_index: u64,
}

impl QueryPositionPnLRequest {
    pub fn new(address: impl Into<String>, market_index: u64) -> Self {
        Self { address: address.into(), market_index }
    }
}

impl From<QueryPositionPnLRequest> for proto::QueryPositionPnLRequest {
    fn from(req: QueryPositionPnLRequest) -> Self {
        Self { address: req.address, market_index: req.market_index }
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

/// Query all positions in a market across all addresses.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryAllPositionsByMarketRequest {
    pub market_index: String,
    pub symbol: Option<String>,
    pub active_only: bool,
    pub side: Option<String>,
    pub limit: i32,
    pub offset: i32,
}

impl QueryAllPositionsByMarketRequest {
    pub fn new(market_index: impl Into<String>) -> Self {
        Self {
            market_index: market_index.into(),
            symbol: None,
            active_only: false,
            side: None,
            limit: 50,
            offset: 0,
        }
    }

    pub fn symbol(mut self, sym: impl Into<String>) -> Self {
        self.symbol = Some(sym.into());
        self
    }

    pub fn active_only(mut self, active: bool) -> Self {
        self.active_only = active;
        self
    }

    pub fn side(mut self, side: impl Into<String>) -> Self {
        self.side = Some(side.into());
        self
    }

    pub fn paginate(mut self, limit: i32, offset: i32) -> Self {
        self.limit = limit;
        self.offset = offset;
        self
    }
}

impl From<QueryAllPositionsByMarketRequest> for proto::QueryAllPositionsByMarketRequest {
    fn from(req: QueryAllPositionsByMarketRequest) -> Self {
        Self {
            market_index: req.market_index,
            symbol: req.symbol.unwrap_or_default(),
            active_only: req.active_only,
            side: req.side.unwrap_or_default(),
            limit: req.limit,
            offset: req.offset,
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
    fn close_position_request_to_any() {
        let req = ClosePositionRequest::new("morpheum1abc", 42, "1000", "50000")
            .position_id("pos-1");
        let any = req.to_any();
        assert_eq!(any.type_url, "/bucket.v1.MsgClosePositionRequest");
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
    fn liquidate_position_to_any() {
        let req = LiquidatePositionRequest::new("morpheum1abc", 42, "48000")
            .reason("margin call");
        let any = req.to_any();
        assert_eq!(any.type_url, "/bucket.v1.MsgLiquidatePositionRequest");
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

    #[test]
    fn query_all_positions_by_market_request() {
        let req = QueryAllPositionsByMarketRequest::new("42")
            .active_only(true)
            .side("LONG")
            .paginate(100, 50);
        let proto_req: proto::QueryAllPositionsByMarketRequest = req.into();
        assert_eq!(proto_req.market_index, "42");
        assert!(proto_req.active_only);
        assert_eq!(proto_req.side, "LONG");
        assert_eq!(proto_req.limit, 100);
        assert_eq!(proto_req.offset, 50);
    }
}
