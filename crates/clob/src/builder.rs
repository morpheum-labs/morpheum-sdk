//! Fluent builders for the CLOB module.
//!
//! Type-safe builders for order placement, modification, cancellation,
//! and market-maker quote operations.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    CancelMarketMakerQuoteRequest, CancelOrderRequest, ModifyOrderRequest, PlaceOrderRequest,
    ProvideMarketMakerQuoteRequest,
};
use crate::types::{OrderType, Side, TimeInForce};

// ====================== PLACE ORDER ======================

/// Fluent builder for placing an order on the CLOB.
#[derive(Default)]
pub struct PlaceOrderBuilder {
    address: Option<String>,
    market_index: Option<u64>,
    price: Option<String>,
    quantity: Option<String>,
    side: Option<Side>,
    order_type: Option<OrderType>,
    client_order_id: Option<String>,
    leverage: Option<String>,
    take_profit: Option<String>,
    stop_loss: Option<String>,
    time_in_force: Option<TimeInForce>,
    post_only: bool,
    hidden: bool,
    reduce_only: bool,
    bucket_id: Option<String>,
}

impl PlaceOrderBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, a: impl Into<String>) -> Self { self.address = Some(a.into()); self }
    pub fn market_index(mut self, i: u64) -> Self { self.market_index = Some(i); self }
    pub fn price(mut self, p: impl Into<String>) -> Self { self.price = Some(p.into()); self }
    pub fn quantity(mut self, q: impl Into<String>) -> Self { self.quantity = Some(q.into()); self }
    pub fn side(mut self, s: Side) -> Self { self.side = Some(s); self }
    pub fn order_type(mut self, t: OrderType) -> Self { self.order_type = Some(t); self }
    pub fn client_order_id(mut self, id: impl Into<String>) -> Self { self.client_order_id = Some(id.into()); self }
    pub fn leverage(mut self, l: impl Into<String>) -> Self { self.leverage = Some(l.into()); self }
    pub fn take_profit(mut self, tp: impl Into<String>) -> Self { self.take_profit = Some(tp.into()); self }
    pub fn stop_loss(mut self, sl: impl Into<String>) -> Self { self.stop_loss = Some(sl.into()); self }
    pub fn time_in_force(mut self, tif: TimeInForce) -> Self { self.time_in_force = Some(tif); self }
    pub fn post_only(mut self, po: bool) -> Self { self.post_only = po; self }
    pub fn hidden(mut self, h: bool) -> Self { self.hidden = h; self }
    pub fn reduce_only(mut self, ro: bool) -> Self { self.reduce_only = ro; self }
    pub fn bucket_id(mut self, id: impl Into<String>) -> Self { self.bucket_id = Some(id.into()); self }

    pub fn build(self) -> Result<PlaceOrderRequest, SdkError> {
        let address = self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?;
        let market_index = self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?;
        let price = self.price.ok_or_else(|| SdkError::invalid_input("price is required"))?;
        let quantity = self.quantity.ok_or_else(|| SdkError::invalid_input("quantity is required"))?;
        let side = self.side.ok_or_else(|| SdkError::invalid_input("side is required"))?;
        let order_type = self.order_type.ok_or_else(|| SdkError::invalid_input("order_type is required"))?;

        let mut req = PlaceOrderRequest::new(address, market_index, price, quantity, side, order_type);
        if let Some(v) = self.client_order_id { req = req.client_order_id(v); }
        if let Some(v) = self.leverage { req = req.leverage(v); }
        if let Some(v) = self.take_profit { req = req.take_profit(v); }
        if let Some(v) = self.stop_loss { req = req.stop_loss(v); }
        if let Some(v) = self.time_in_force { req = req.time_in_force(v); }
        if self.post_only { req = req.post_only(true); }
        if self.hidden { req = req.hidden(true); }
        if self.reduce_only { req = req.reduce_only(true); }
        if let Some(v) = self.bucket_id { req = req.bucket_id(v); }
        Ok(req)
    }
}

// ====================== MODIFY ORDER ======================

/// Fluent builder for modifying an existing order.
#[derive(Default)]
pub struct ModifyOrderBuilder {
    address: Option<String>,
    order_id: Option<String>,
    symbol: Option<String>,
    new_price: Option<String>,
    new_quantity: Option<String>,
}

impl ModifyOrderBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, a: impl Into<String>) -> Self { self.address = Some(a.into()); self }
    pub fn order_id(mut self, id: impl Into<String>) -> Self { self.order_id = Some(id.into()); self }
    pub fn symbol(mut self, s: impl Into<String>) -> Self { self.symbol = Some(s.into()); self }
    pub fn new_price(mut self, p: impl Into<String>) -> Self { self.new_price = Some(p.into()); self }
    pub fn new_quantity(mut self, q: impl Into<String>) -> Self { self.new_quantity = Some(q.into()); self }

    pub fn build(self) -> Result<ModifyOrderRequest, SdkError> {
        let address = self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?;
        let order_id = self.order_id.ok_or_else(|| SdkError::invalid_input("order_id is required"))?;
        let mut req = ModifyOrderRequest::new(address, order_id);
        if let Some(v) = self.symbol { req = req.symbol(v); }
        if let Some(v) = self.new_price { req = req.new_price(v); }
        if let Some(v) = self.new_quantity { req = req.new_quantity(v); }
        Ok(req)
    }
}

// ====================== CANCEL ORDER ======================

/// Fluent builder for cancelling an order.
#[derive(Default)]
pub struct CancelOrderBuilder {
    address: Option<String>,
    order_id: Option<String>,
    symbol: Option<String>,
}

impl CancelOrderBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn address(mut self, a: impl Into<String>) -> Self { self.address = Some(a.into()); self }
    pub fn order_id(mut self, id: impl Into<String>) -> Self { self.order_id = Some(id.into()); self }
    pub fn symbol(mut self, s: impl Into<String>) -> Self { self.symbol = Some(s.into()); self }

    pub fn build(self) -> Result<CancelOrderRequest, SdkError> {
        let address = self.address.ok_or_else(|| SdkError::invalid_input("address is required"))?;
        let order_id = self.order_id.ok_or_else(|| SdkError::invalid_input("order_id is required"))?;
        let mut req = CancelOrderRequest::new(address, order_id);
        if let Some(v) = self.symbol { req = req.symbol(v); }
        Ok(req)
    }
}

// ====================== PROVIDE MM QUOTE ======================

/// Fluent builder for providing a market-maker quote.
#[derive(Default)]
pub struct ProvideMarketMakerQuoteBuilder {
    provider: Option<String>,
    pool_id: Option<String>,
    market_index: Option<u64>,
    side: Option<Side>,
    price: Option<String>,
    amount: Option<String>,
}

impl ProvideMarketMakerQuoteBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn provider(mut self, p: impl Into<String>) -> Self { self.provider = Some(p.into()); self }
    pub fn pool_id(mut self, id: impl Into<String>) -> Self { self.pool_id = Some(id.into()); self }
    pub fn market_index(mut self, i: u64) -> Self { self.market_index = Some(i); self }
    pub fn side(mut self, s: Side) -> Self { self.side = Some(s); self }
    pub fn price(mut self, p: impl Into<String>) -> Self { self.price = Some(p.into()); self }
    pub fn amount(mut self, a: impl Into<String>) -> Self { self.amount = Some(a.into()); self }

    pub fn build(self) -> Result<ProvideMarketMakerQuoteRequest, SdkError> {
        let provider = self.provider.ok_or_else(|| SdkError::invalid_input("provider is required"))?;
        let pool_id = self.pool_id.ok_or_else(|| SdkError::invalid_input("pool_id is required"))?;
        let market_index = self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?;
        let side = self.side.ok_or_else(|| SdkError::invalid_input("side is required"))?;
        let price = self.price.ok_or_else(|| SdkError::invalid_input("price is required"))?;
        let amount = self.amount.ok_or_else(|| SdkError::invalid_input("amount is required"))?;
        Ok(ProvideMarketMakerQuoteRequest::new(provider, pool_id, market_index, side, price, amount))
    }
}

// ====================== CANCEL MM QUOTE ======================

/// Fluent builder for cancelling a market-maker quote.
#[derive(Default)]
pub struct CancelMarketMakerQuoteBuilder {
    quote_id: Option<String>,
    provider: Option<String>,
}

impl CancelMarketMakerQuoteBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn quote_id(mut self, id: impl Into<String>) -> Self { self.quote_id = Some(id.into()); self }
    pub fn provider(mut self, p: impl Into<String>) -> Self { self.provider = Some(p.into()); self }

    pub fn build(self) -> Result<CancelMarketMakerQuoteRequest, SdkError> {
        let quote_id = self.quote_id.ok_or_else(|| SdkError::invalid_input("quote_id is required"))?;
        let provider = self.provider.ok_or_else(|| SdkError::invalid_input("provider is required"))?;
        Ok(CancelMarketMakerQuoteRequest::new(quote_id, provider))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place_order_builder_full() {
        let req = PlaceOrderBuilder::new()
            .address("morpheum1abc").market_index(42).price("50000").quantity("1000")
            .side(Side::Buy).order_type(OrderType::Limit).leverage("10")
            .time_in_force(TimeInForce::Gtc).bucket_id("bucket-1")
            .build().unwrap();
        assert_eq!(req.address, "morpheum1abc");
        assert_eq!(req.market_index, 42);
        assert_eq!(req.side, Side::Buy);
    }

    #[test]
    fn place_order_builder_validation() {
        assert!(PlaceOrderBuilder::new().build().is_err());
        assert!(PlaceOrderBuilder::new().address("x").build().is_err());
    }

    #[test]
    fn modify_order_builder_works() {
        let req = ModifyOrderBuilder::new()
            .address("morpheum1abc").order_id("order-1").new_price("51000")
            .build().unwrap();
        assert_eq!(req.new_price, Some("51000".into()));
    }

    #[test]
    fn cancel_order_builder_works() {
        let req = CancelOrderBuilder::new()
            .address("morpheum1abc").order_id("order-1")
            .build().unwrap();
        assert_eq!(req.order_id, "order-1");
    }

    #[test]
    fn provide_mm_quote_builder_works() {
        let req = ProvideMarketMakerQuoteBuilder::new()
            .provider("morpheum1mm").pool_id("pool-1").market_index(42)
            .side(Side::Buy).price("50000").amount("100")
            .build().unwrap();
        assert_eq!(req.provider, "morpheum1mm");
    }

    #[test]
    fn cancel_mm_quote_builder_works() {
        let req = CancelMarketMakerQuoteBuilder::new()
            .quote_id("quote-1").provider("morpheum1mm")
            .build().unwrap();
        assert_eq!(req.quote_id, "quote-1");
    }

    #[test]
    fn cancel_mm_quote_builder_validation() {
        assert!(CancelMarketMakerQuoteBuilder::new().build().is_err());
    }
}
