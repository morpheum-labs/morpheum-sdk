//! Request wrappers for the CLOB module.
//!
//! Transaction requests include `to_any()` for `TxBuilder` integration.
//! Query requests convert to proto via `From` impls.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::clob::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::{OrderStatus, OrderType, Side, TimeInForce};

// ====================== TRANSACTION REQUESTS ======================

/// Request to place a single order.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlaceOrderRequest {
    pub address: String,
    pub market_index: u64,
    pub price: String,
    pub quantity: String,
    pub side: Side,
    pub order_type: OrderType,
    pub client_order_id: Option<String>,
    pub leverage: Option<String>,
    pub take_profit: Option<String>,
    pub stop_loss: Option<String>,
    pub time_in_force: TimeInForce,
    pub post_only: bool,
    pub hidden: bool,
    pub display_quantity: Option<String>,
    pub reduce_only: bool,
    pub bucket_id: Option<String>,
}

impl PlaceOrderRequest {
    pub fn new(
        address: impl Into<String>,
        market_index: u64,
        price: impl Into<String>,
        quantity: impl Into<String>,
        side: Side,
        order_type: OrderType,
    ) -> Self {
        Self {
            address: address.into(),
            market_index,
            price: price.into(),
            quantity: quantity.into(),
            side,
            order_type,
            client_order_id: None,
            leverage: None,
            take_profit: None,
            stop_loss: None,
            time_in_force: TimeInForce::Gtc,
            post_only: false,
            hidden: false,
            display_quantity: None,
            reduce_only: false,
            bucket_id: None,
        }
    }

    pub fn client_order_id(mut self, id: impl Into<String>) -> Self {
        self.client_order_id = Some(id.into()); self
    }
    pub fn leverage(mut self, lev: impl Into<String>) -> Self {
        self.leverage = Some(lev.into()); self
    }
    pub fn take_profit(mut self, tp: impl Into<String>) -> Self {
        self.take_profit = Some(tp.into()); self
    }
    pub fn stop_loss(mut self, sl: impl Into<String>) -> Self {
        self.stop_loss = Some(sl.into()); self
    }
    pub fn time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = tif; self
    }
    pub fn post_only(mut self, po: bool) -> Self { self.post_only = po; self }
    pub fn hidden(mut self, h: bool) -> Self { self.hidden = h; self }
    pub fn reduce_only(mut self, ro: bool) -> Self { self.reduce_only = ro; self }
    pub fn bucket_id(mut self, id: impl Into<String>) -> Self {
        self.bucket_id = Some(id.into()); self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgPlaceOrderRequest = self.clone().into();
        ProtoAny { type_url: "/clob.v1.MsgPlaceOrderRequest".into(), value: msg.encode_to_vec() }
    }
}

impl From<PlaceOrderRequest> for proto::MsgPlaceOrderRequest {
    fn from(r: PlaceOrderRequest) -> Self {
        Self {
            address: r.address,
            market_index: r.market_index,
            price: r.price,
            quantity: r.quantity,
            side: i32::from(r.side),
            order_type: i32::from(r.order_type),
            timestamp: None,
            client_order_id: r.client_order_id.unwrap_or_default(),
            leverage: r.leverage.unwrap_or_default(),
            take_profit: r.take_profit.unwrap_or_default(),
            stop_loss: r.stop_loss.unwrap_or_default(),
            time_in_force: i32::from(r.time_in_force),
            post_only: r.post_only,
            hidden: r.hidden,
            display_quantity: r.display_quantity.unwrap_or_default(),
            reduce_only: r.reduce_only,
            bucket_id: r.bucket_id.unwrap_or_default(),
        }
    }
}

/// Request to modify an existing order.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ModifyOrderRequest {
    pub address: String,
    pub order_id: String,
    pub symbol: Option<String>,
    pub new_price: Option<String>,
    pub new_quantity: Option<String>,
}

impl ModifyOrderRequest {
    pub fn new(address: impl Into<String>, order_id: impl Into<String>) -> Self {
        Self {
            address: address.into(), order_id: order_id.into(),
            symbol: None, new_price: None, new_quantity: None,
        }
    }
    pub fn new_price(mut self, p: impl Into<String>) -> Self { self.new_price = Some(p.into()); self }
    pub fn new_quantity(mut self, q: impl Into<String>) -> Self { self.new_quantity = Some(q.into()); self }
    pub fn symbol(mut self, s: impl Into<String>) -> Self { self.symbol = Some(s.into()); self }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgModifyOrderRequest = self.clone().into();
        ProtoAny { type_url: "/clob.v1.MsgModifyOrderRequest".into(), value: msg.encode_to_vec() }
    }
}

impl From<ModifyOrderRequest> for proto::MsgModifyOrderRequest {
    fn from(r: ModifyOrderRequest) -> Self {
        Self {
            address: r.address, order_id: r.order_id,
            symbol: r.symbol.unwrap_or_default(),
            new_price: r.new_price.unwrap_or_default(),
            new_quantity: r.new_quantity.unwrap_or_default(),
            timestamp: None,
        }
    }
}

/// Request to cancel an order.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CancelOrderRequest {
    pub address: String,
    pub order_id: String,
    pub symbol: Option<String>,
}

impl CancelOrderRequest {
    pub fn new(address: impl Into<String>, order_id: impl Into<String>) -> Self {
        Self { address: address.into(), order_id: order_id.into(), symbol: None }
    }
    pub fn symbol(mut self, s: impl Into<String>) -> Self { self.symbol = Some(s.into()); self }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCancelOrderRequest = self.clone().into();
        ProtoAny { type_url: "/clob.v1.MsgCancelOrderRequest".into(), value: msg.encode_to_vec() }
    }
}

impl From<CancelOrderRequest> for proto::MsgCancelOrderRequest {
    fn from(r: CancelOrderRequest) -> Self {
        Self {
            address: r.address, order_id: r.order_id,
            symbol: r.symbol.unwrap_or_default(), timestamp: None,
        }
    }
}

/// Request to place multiple orders atomically.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PlaceBatchOrdersRequest {
    pub from_address: String,
    pub orders: alloc::vec::Vec<PlaceOrderRequest>,
    pub orders_hash: String,
}

impl PlaceBatchOrdersRequest {
    pub fn new(
        from_address: impl Into<String>,
        orders: alloc::vec::Vec<PlaceOrderRequest>,
    ) -> Self {
        Self {
            from_address: from_address.into(),
            orders,
            orders_hash: String::new(),
        }
    }

    pub fn orders_hash(mut self, orders_hash: impl Into<String>) -> Self {
        self.orders_hash = orders_hash.into();
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgPlaceBatchOrdersRequest = self.clone().into();
        ProtoAny {
            type_url: "/clob.v1.MsgPlaceBatchOrdersRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<PlaceBatchOrdersRequest> for proto::MsgPlaceBatchOrdersRequest {
    fn from(r: PlaceBatchOrdersRequest) -> Self {
        Self {
            from_address: r.from_address,
            orders: r.orders.into_iter().map(Into::into).collect(),
            orders_hash: r.orders_hash,
        }
    }
}

/// Request to provide a market-maker quote.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProvideMarketMakerQuoteRequest {
    pub provider: String,
    pub pool_id: String,
    pub market_index: u64,
    pub side: Side,
    pub price: String,
    pub amount: String,
}

impl ProvideMarketMakerQuoteRequest {
    pub fn new(
        provider: impl Into<String>, pool_id: impl Into<String>,
        market_index: u64, side: Side, price: impl Into<String>, amount: impl Into<String>,
    ) -> Self {
        Self {
            provider: provider.into(), pool_id: pool_id.into(),
            market_index, side, price: price.into(), amount: amount.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgProvideMarketMakerQuoteRequest = self.clone().into();
        ProtoAny { type_url: "/clob.v1.MsgProvideMarketMakerQuoteRequest".into(), value: msg.encode_to_vec() }
    }
}

impl From<ProvideMarketMakerQuoteRequest> for proto::MsgProvideMarketMakerQuoteRequest {
    fn from(r: ProvideMarketMakerQuoteRequest) -> Self {
        Self {
            provider: r.provider, pool_id: r.pool_id, market_index: r.market_index,
            side: i32::from(r.side), price: r.price, amount: r.amount,
            duration: None, timestamp: None, provider_external_address: None,
        }
    }
}

/// Request to cancel a market-maker quote.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CancelMarketMakerQuoteRequest {
    pub quote_id: String,
    pub provider: String,
}

impl CancelMarketMakerQuoteRequest {
    pub fn new(quote_id: impl Into<String>, provider: impl Into<String>) -> Self {
        Self { quote_id: quote_id.into(), provider: provider.into() }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCancelMarketMakerQuoteRequest = self.clone().into();
        ProtoAny { type_url: "/clob.v1.MsgCancelMarketMakerQuoteRequest".into(), value: msg.encode_to_vec() }
    }
}

impl From<CancelMarketMakerQuoteRequest> for proto::MsgCancelMarketMakerQuoteRequest {
    fn from(r: CancelMarketMakerQuoteRequest) -> Self {
        Self { quote_id: r.quote_id, provider: r.provider, timestamp: None }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query orderbook snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryOrderBookSnapshotRequest {
    pub market_index: u64,
    pub depth: i32,
}

impl QueryOrderBookSnapshotRequest {
    pub fn new(market_index: u64, depth: i32) -> Self { Self { market_index, depth } }
}

impl From<QueryOrderBookSnapshotRequest> for proto::QueryOrderbookSnapshotRequest {
    fn from(r: QueryOrderBookSnapshotRequest) -> Self { Self { market_index: r.market_index, depth: r.depth } }
}

/// Query orders by address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryOrdersByAddressRequest {
    pub address: String,
    pub market_index: Option<u64>,
    pub status: Option<OrderStatus>,
}

impl QueryOrdersByAddressRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self { address: address.into(), market_index: None, status: None }
    }
    pub fn market_index(mut self, idx: u64) -> Self { self.market_index = Some(idx); self }
    pub fn status(mut self, s: OrderStatus) -> Self { self.status = Some(s); self }
}

impl From<QueryOrdersByAddressRequest> for proto::QueryOrdersByAddressRequest {
    fn from(r: QueryOrdersByAddressRequest) -> Self {
        Self {
            address: r.address,
            market_index: r.market_index,
            status: r.status.map(i32::from),
            pagination_request: None,
        }
    }
}

/// Query orders by market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryOrdersByMarketRequest {
    pub market_index: u64,
    pub symbol: Option<String>,
    pub status: Option<OrderStatus>,
    pub side: Option<Side>,
}

impl QueryOrdersByMarketRequest {
    pub fn new(market_index: u64) -> Self {
        Self { market_index, symbol: None, status: None, side: None }
    }
    pub fn symbol(mut self, s: impl Into<String>) -> Self { self.symbol = Some(s.into()); self }
    pub fn status(mut self, s: OrderStatus) -> Self { self.status = Some(s); self }
    pub fn side(mut self, s: Side) -> Self { self.side = Some(s); self }
}

impl From<QueryOrdersByMarketRequest> for proto::QueryOrdersByMarketRequest {
    fn from(r: QueryOrdersByMarketRequest) -> Self {
        Self {
            market_index: r.market_index,
            symbol: r.symbol.unwrap_or_default(),
            status: r.status.map(i32::from),
            side: r.side.map(i32::from),
            pagination_request: None,
        }
    }
}

/// Query a single order by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryOrderByIdRequest {
    pub order_id: String,
    pub address: String,
}

impl QueryOrderByIdRequest {
    pub fn new(order_id: impl Into<String>, address: impl Into<String>) -> Self {
        Self { order_id: order_id.into(), address: address.into() }
    }
}

impl From<QueryOrderByIdRequest> for proto::QueryOrderByIdRequest {
    fn from(r: QueryOrderByIdRequest) -> Self { Self { order_id: r.order_id, address: r.address } }
}

/// Query trades by address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryTradesByAddressRequest {
    pub address: String,
    pub market_index: Option<u64>,
    pub start_time: i64,
    pub end_time: i64,
}

impl QueryTradesByAddressRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self { address: address.into(), market_index: None, start_time: 0, end_time: 0 }
    }
    pub fn market_index(mut self, idx: u64) -> Self { self.market_index = Some(idx); self }
    pub fn time_range(mut self, start: i64, end: i64) -> Self {
        self.start_time = start; self.end_time = end; self
    }
}

impl From<QueryTradesByAddressRequest> for proto::QueryTradesByAddressRequest {
    fn from(r: QueryTradesByAddressRequest) -> Self {
        Self {
            address: r.address, market_index: r.market_index,
            start_time: r.start_time, end_time: r.end_time, pagination_request: None,
        }
    }
}

/// Query trades by market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryTradesByMarketRequest {
    pub market_index: u64,
    pub symbol: Option<String>,
    pub start_time: i64,
    pub end_time: i64,
}

impl QueryTradesByMarketRequest {
    pub fn new(market_index: u64) -> Self {
        Self { market_index, symbol: None, start_time: 0, end_time: 0 }
    }
    pub fn symbol(mut self, s: impl Into<String>) -> Self { self.symbol = Some(s.into()); self }
    pub fn time_range(mut self, start: i64, end: i64) -> Self {
        self.start_time = start; self.end_time = end; self
    }
}

impl From<QueryTradesByMarketRequest> for proto::QueryTradesByMarketRequest {
    fn from(r: QueryTradesByMarketRequest) -> Self {
        Self {
            market_index: r.market_index, symbol: r.symbol.unwrap_or_default(),
            start_time: r.start_time, end_time: r.end_time, pagination_request: None,
        }
    }
}

/// Query funding rate for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryFundingRateRequest {
    pub market_index: u64,
}

impl QueryFundingRateRequest {
    pub fn new(market_index: u64) -> Self { Self { market_index } }
}

impl From<QueryFundingRateRequest> for proto::QueryFundingRateRequest {
    fn from(r: QueryFundingRateRequest) -> Self {
        Self { market_index: r.market_index, timestamp: None, calculation_period_seconds: 0 }
    }
}

/// Query funding rate history.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryFundingRatesRequest {
    pub market_index: u64,
    pub start_time: i64,
    pub end_time: i64,
    pub limit: i32,
}

impl QueryFundingRatesRequest {
    pub fn new(market_index: u64, limit: i32) -> Self {
        Self { market_index, start_time: 0, end_time: 0, limit }
    }
    pub fn time_range(mut self, start: i64, end: i64) -> Self {
        self.start_time = start; self.end_time = end; self
    }
}

impl From<QueryFundingRatesRequest> for proto::QueryFundingRatesRequest {
    fn from(r: QueryFundingRatesRequest) -> Self {
        Self { market_index: r.market_index, start_time: r.start_time, end_time: r.end_time, limit: r.limit }
    }
}

/// Query orderbook checksum.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetChecksumRequest {
    pub market_index: u64,
    pub depth: i32,
}

impl GetChecksumRequest {
    pub fn new(market_index: u64, depth: i32) -> Self { Self { market_index, depth } }
}

impl From<GetChecksumRequest> for proto::GetChecksumRequest {
    fn from(r: GetChecksumRequest) -> Self { Self { market_index: r.market_index, depth: r.depth } }
}

/// Query active market-maker quotes.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryActiveMarketMakerQuotesRequest {
    pub pool_id: String,
    pub market_index: Option<u64>,
    pub side: Option<Side>,
    pub status: Option<String>,
}

impl QueryActiveMarketMakerQuotesRequest {
    pub fn new(pool_id: impl Into<String>) -> Self {
        Self { pool_id: pool_id.into(), market_index: None, side: None, status: None }
    }
    pub fn market_index(mut self, idx: u64) -> Self { self.market_index = Some(idx); self }
    pub fn side(mut self, s: Side) -> Self { self.side = Some(s); self }
    pub fn status(mut self, s: impl Into<String>) -> Self { self.status = Some(s.into()); self }
}

impl From<QueryActiveMarketMakerQuotesRequest> for proto::QueryActiveMarketMakerQuotesRequest {
    fn from(r: QueryActiveMarketMakerQuotesRequest) -> Self {
        Self {
            pool_id: r.pool_id, market_index: r.market_index,
            side: r.side.map(i32::from), status: r.status, pagination: None,
        }
    }
}

/// Query a single market-maker quote by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMarketMakerQuoteByIdRequest {
    pub quote_id: String,
}

impl QueryMarketMakerQuoteByIdRequest {
    pub fn new(quote_id: impl Into<String>) -> Self { Self { quote_id: quote_id.into() } }
}

impl From<QueryMarketMakerQuoteByIdRequest> for proto::QueryMarketMakerQuoteByIdRequest {
    fn from(r: QueryMarketMakerQuoteByIdRequest) -> Self { Self { quote_id: r.quote_id } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_order_request_to_any() {
        let req = PlaceOrderRequest::new("morpheum1abc", 42, "50000", "1000", Side::Buy, OrderType::Limit)
            .leverage("10").bucket_id("bucket-1").time_in_force(TimeInForce::Gtc);
        let any = req.to_any();
        assert_eq!(any.type_url, "/clob.v1.MsgPlaceOrderRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn modify_order_request_to_any() {
        let req = ModifyOrderRequest::new("morpheum1abc", "order-1").new_price("51000");
        let any = req.to_any();
        assert_eq!(any.type_url, "/clob.v1.MsgModifyOrderRequest");
    }

    #[test]
    fn cancel_order_request_to_any() {
        let req = CancelOrderRequest::new("morpheum1abc", "order-1");
        let any = req.to_any();
        assert_eq!(any.type_url, "/clob.v1.MsgCancelOrderRequest");
    }

    #[test]
    fn place_batch_orders_to_any() {
        let order = PlaceOrderRequest::new(
            "morpheum1abc",
            42,
            "50000",
            "100",
            Side::Buy,
            OrderType::Limit,
        );
        let req = PlaceBatchOrdersRequest::new("morpheum1abc", alloc::vec![order]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/clob.v1.MsgPlaceBatchOrdersRequest");
    }

    #[test]
    fn provide_mm_quote_to_any() {
        let req = ProvideMarketMakerQuoteRequest::new("morpheum1mm", "pool-1", 42, Side::Buy, "50000", "100");
        let any = req.to_any();
        assert_eq!(any.type_url, "/clob.v1.MsgProvideMarketMakerQuoteRequest");
    }

    #[test]
    fn query_orders_by_address_conversion() {
        let req = QueryOrdersByAddressRequest::new("morpheum1abc").market_index(42).status(OrderStatus::Active);
        let p: proto::QueryOrdersByAddressRequest = req.into();
        assert_eq!(p.address, "morpheum1abc");
        assert_eq!(p.market_index, Some(42));
    }
}
