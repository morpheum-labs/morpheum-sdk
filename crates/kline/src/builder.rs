//! Fluent builders for kline module transactions.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{EpochBoundaryRequest, ProcessTradeRequest, UpdateSentimentRequest};
use crate::types::{PositionSnapshot, TradeData};

// ====================== PROCESS TRADE ======================

/// Fluent builder for processing a trade into OHLC candles.
#[derive(Default)]
pub struct ProcessTradeBuilder {
    market_index: Option<u64>,
    price: Option<u64>,
    quantity: Option<String>,
    is_taker_buy: Option<bool>,
    block_height: Option<u64>,
    logical_timestamp: Option<u64>,
    feed_id: String,
    outcome_id: u32,
}

impl ProcessTradeBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn price(mut self, v: u64) -> Self { self.price = Some(v); self }
    pub fn quantity(mut self, v: impl Into<String>) -> Self { self.quantity = Some(v.into()); self }
    pub fn is_taker_buy(mut self, v: bool) -> Self { self.is_taker_buy = Some(v); self }
    pub fn block_height(mut self, v: u64) -> Self { self.block_height = Some(v); self }
    pub fn logical_timestamp(mut self, v: u64) -> Self { self.logical_timestamp = Some(v); self }
    pub fn feed_id(mut self, v: impl Into<String>) -> Self { self.feed_id = v.into(); self }
    pub fn outcome_id(mut self, v: u32) -> Self { self.outcome_id = v; self }

    pub fn build(self) -> Result<ProcessTradeRequest, SdkError> {
        let trade = TradeData {
            market_index: self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            price: self.price.ok_or_else(|| SdkError::invalid_input("price is required"))?,
            quantity: self.quantity.ok_or_else(|| SdkError::invalid_input("quantity is required"))?,
            is_taker_buy: self.is_taker_buy.ok_or_else(|| SdkError::invalid_input("is_taker_buy is required"))?,
            block_height: self.block_height.ok_or_else(|| SdkError::invalid_input("block_height is required"))?,
            logical_timestamp: self.logical_timestamp.ok_or_else(|| SdkError::invalid_input("logical_timestamp is required"))?,
            feed_id: self.feed_id,
            outcome_id: self.outcome_id,
        };
        Ok(ProcessTradeRequest::new(trade))
    }
}

// ====================== UPDATE SENTIMENT ======================

/// Fluent builder for updating a sentiment candle from a position snapshot.
#[derive(Default)]
pub struct UpdateSentimentBuilder {
    market_index: Option<u64>,
    long_oi: Option<String>,
    short_oi: Option<String>,
    block_height: Option<u64>,
    logical_timestamp: Option<u64>,
}

impl UpdateSentimentBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn market_index(mut self, v: u64) -> Self { self.market_index = Some(v); self }
    pub fn long_oi(mut self, v: impl Into<String>) -> Self { self.long_oi = Some(v.into()); self }
    pub fn short_oi(mut self, v: impl Into<String>) -> Self { self.short_oi = Some(v.into()); self }
    pub fn block_height(mut self, v: u64) -> Self { self.block_height = Some(v); self }
    pub fn logical_timestamp(mut self, v: u64) -> Self { self.logical_timestamp = Some(v); self }

    pub fn build(self) -> Result<UpdateSentimentRequest, SdkError> {
        let snapshot = PositionSnapshot {
            market_index: self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?,
            long_oi: self.long_oi.ok_or_else(|| SdkError::invalid_input("long_oi is required"))?,
            short_oi: self.short_oi.ok_or_else(|| SdkError::invalid_input("short_oi is required"))?,
            block_height: self.block_height.ok_or_else(|| SdkError::invalid_input("block_height is required"))?,
            logical_timestamp: self.logical_timestamp.ok_or_else(|| SdkError::invalid_input("logical_timestamp is required"))?,
        };
        Ok(UpdateSentimentRequest::new(snapshot))
    }
}

// ====================== EPOCH BOUNDARY ======================

/// Fluent builder for triggering an epoch boundary.
#[derive(Default)]
pub struct EpochBoundaryBuilder {
    epoch_id: Option<u64>,
}

impl EpochBoundaryBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn epoch_id(mut self, v: u64) -> Self { self.epoch_id = Some(v); self }

    pub fn build(self) -> Result<EpochBoundaryRequest, SdkError> {
        let epoch_id = self.epoch_id.ok_or_else(|| SdkError::invalid_input("epoch_id is required"))?;
        Ok(EpochBoundaryRequest::new(epoch_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_trade_builder_works() {
        let req = ProcessTradeBuilder::new()
            .market_index(1).price(50000).quantity("10")
            .is_taker_buy(true).block_height(100).logical_timestamp(200)
            .build().unwrap();
        assert_eq!(req.trade.price, 50000);
    }

    #[test]
    fn process_trade_builder_validation() {
        assert!(ProcessTradeBuilder::new().build().is_err());
    }

    #[test]
    fn update_sentiment_builder_works() {
        let req = UpdateSentimentBuilder::new()
            .market_index(1).long_oi("600").short_oi("400")
            .block_height(100).logical_timestamp(200)
            .build().unwrap();
        assert_eq!(req.snapshot.long_oi, "600");
    }

    #[test]
    fn update_sentiment_builder_validation() {
        assert!(UpdateSentimentBuilder::new().build().is_err());
    }

    #[test]
    fn epoch_boundary_builder_works() {
        let req = EpochBoundaryBuilder::new().epoch_id(42).build().unwrap();
        assert_eq!(req.epoch_id, 42);
    }

    #[test]
    fn epoch_boundary_builder_validation() {
        assert!(EpochBoundaryBuilder::new().build().is_err());
    }
}
