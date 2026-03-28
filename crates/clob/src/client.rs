//! ClobClient — main entry point for all CLOB-related queries.
//!
//! Transaction operations (place, modify, cancel orders, MM quotes) are
//! handled via fluent builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::clob::v1 as proto;

use crate::requests;
use crate::types::{
    FundingRate, FundingRateEntry, MarketMakerQuote, Order, OrderBookChecksum,
    OrderBookSnapshot, PriceLevel, Trade,
};

/// Primary client for all CLOB-related queries.
pub struct ClobClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl ClobClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Fetches an orderbook snapshot for a market.
    pub async fn query_orderbook_snapshot(
        &self, market_index: u64, depth: i32,
    ) -> Result<OrderBookSnapshot, SdkError> {
        let req = requests::QueryOrderBookSnapshotRequest::new(market_index, depth);
        let proto_req: proto::QueryOrderbookSnapshotRequest = req.into();
        let resp = self.query("/clob.v1.Query/QueryOrderBookSnapshot", proto_req.encode_to_vec()).await?;
        let p = proto::QueryOrderbookSnapshotResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(OrderBookSnapshot {
            symbol: p.symbol,
            bids: p.bids.into_iter().map(|l| PriceLevel { price: l.price, quantity: l.quantity }).collect(),
            asks: p.asks.into_iter().map(|l| PriceLevel { price: l.price, quantity: l.quantity }).collect(),
            sequence_id: p.sequence_id,
            checksum: p.checksum,
            block_height: p.block_height,
            updated_at: p.updated_at.map(|t| t.seconds as u64).unwrap_or(0),
        })
    }

    /// Queries orders for an address with optional filters.
    pub async fn query_orders_by_address(
        &self, request: requests::QueryOrdersByAddressRequest,
    ) -> Result<Vec<Order>, SdkError> {
        let proto_req: proto::QueryOrdersByAddressRequest = request.into();
        let resp = self.query("/clob.v1.Query/QueryOrdersByAddress", proto_req.encode_to_vec()).await?;
        let p = proto::QueryOrdersByAddressResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.orders.into_iter().map(Into::into).collect())
    }

    /// Queries orders for a market with optional filters.
    pub async fn query_orders_by_market(
        &self, request: requests::QueryOrdersByMarketRequest,
    ) -> Result<Vec<Order>, SdkError> {
        let proto_req: proto::QueryOrdersByMarketRequest = request.into();
        let resp = self.query("/clob.v1.Query/QueryOrdersByMarket", proto_req.encode_to_vec()).await?;
        let p = proto::QueryOrdersByMarketResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.orders.into_iter().map(Into::into).collect())
    }

    /// Queries a single order by ID.
    pub async fn query_order_by_id(
        &self, order_id: impl Into<String>, address: impl Into<String>,
    ) -> Result<Option<Order>, SdkError> {
        let req = requests::QueryOrderByIdRequest::new(order_id, address);
        let proto_req: proto::QueryOrderByIdRequest = req.into();
        let resp = self.query("/clob.v1.Query/QueryOrderById", proto_req.encode_to_vec()).await?;
        let p = proto::QueryOrderByIdResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.order.map(Into::into))
    }

    /// Queries trades for an address.
    pub async fn query_trades_by_address(
        &self, request: requests::QueryTradesByAddressRequest,
    ) -> Result<Vec<Trade>, SdkError> {
        let proto_req: proto::QueryTradesByAddressRequest = request.into();
        let resp = self.query("/clob.v1.Query/QueryTradesByAddress", proto_req.encode_to_vec()).await?;
        let p = proto::QueryTradesByAddressResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.trades.into_iter().map(Into::into).collect())
    }

    /// Queries trades for a market.
    pub async fn query_trades_by_market(
        &self, request: requests::QueryTradesByMarketRequest,
    ) -> Result<Vec<Trade>, SdkError> {
        let proto_req: proto::QueryTradesByMarketRequest = request.into();
        let resp = self.query("/clob.v1.Query/QueryTradesByMarket", proto_req.encode_to_vec()).await?;
        let p = proto::QueryTradesByMarketResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.trades.into_iter().map(Into::into).collect())
    }

    /// Queries the current funding rate for a market.
    pub async fn query_funding_rate(&self, market_index: u64) -> Result<FundingRate, SdkError> {
        let req = requests::QueryFundingRateRequest::new(market_index);
        let proto_req: proto::QueryFundingRateRequest = req.into();
        let resp = self.query("/clob.v1.Query/QueryFundingRate", proto_req.encode_to_vec()).await?;
        let p = proto::QueryFundingRateResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        if !p.success {
            return Err(SdkError::transport(if p.error_message.is_empty() { "funding rate query failed" } else { &p.error_message }));
        }
        Ok(FundingRate {
            market_index: p.market_index,
            funding_rate: p.funding_rate,
            twap_price: p.twap_price,
            mark_price: p.mark_price,
            index_price: p.index_price,
            calculation_time: p.calculation_time.map(|t| t.seconds as u64).unwrap_or(0),
            next_funding_time: p.next_funding_time.map(|t| t.seconds as u64).unwrap_or(0),
            calculation_details: p.calculation_details.into_iter().collect(),
        })
    }

    /// Queries funding rate history for a market.
    pub async fn query_funding_rates(
        &self, request: requests::QueryFundingRatesRequest,
    ) -> Result<Vec<FundingRateEntry>, SdkError> {
        let proto_req: proto::QueryFundingRatesRequest = request.into();
        let resp = self.query("/clob.v1.Query/QueryFundingRates", proto_req.encode_to_vec()).await?;
        let p = proto::QueryFundingRatesResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.funding_rates.into_iter().map(Into::into).collect())
    }

    /// Gets the current orderbook checksum for integrity validation.
    pub async fn get_checksum(
        &self, market_index: u64, depth: i32,
    ) -> Result<OrderBookChecksum, SdkError> {
        let req = requests::GetChecksumRequest::new(market_index, depth);
        let proto_req: proto::GetChecksumRequest = req.into();
        let resp = self.query("/clob.v1.Query/GetChecksum", proto_req.encode_to_vec()).await?;
        let p = proto::GetChecksumResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(OrderBookChecksum {
            market_index: p.market_index,
            checksum: p.checksum,
            sequence_id: p.sequence_id,
            timestamp: p.timestamp.map(|t| t.seconds as u64).unwrap_or(0),
            block_height: p.block_height,
            is_valid: p.is_valid,
        })
    }

    /// Queries active market-maker quotes.
    pub async fn query_active_mm_quotes(
        &self, request: requests::QueryActiveMarketMakerQuotesRequest,
    ) -> Result<Vec<MarketMakerQuote>, SdkError> {
        let proto_req: proto::QueryActiveMarketMakerQuotesRequest = request.into();
        let resp = self.query("/clob.v1.Query/QueryActiveMarketMakerQuotes", proto_req.encode_to_vec()).await?;
        let p = proto::QueryActiveMarketMakerQuotesResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.quotes.into_iter().map(Into::into).collect())
    }

    /// Queries a single market-maker quote by ID.
    pub async fn query_mm_quote_by_id(
        &self, quote_id: impl Into<String>,
    ) -> Result<Option<MarketMakerQuote>, SdkError> {
        let req = requests::QueryMarketMakerQuoteByIdRequest::new(quote_id);
        let proto_req: proto::QueryMarketMakerQuoteByIdRequest = req.into();
        let resp = self.query("/clob.v1.Query/QueryMarketMakerQuoteById", proto_req.encode_to_vec()).await?;
        let p = proto::QueryMarketMakerQuoteByIdResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.quote.map(Into::into))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for ClobClient {
    fn config(&self) -> &SdkConfig { &self.config }
    fn transport(&self) -> &dyn Transport { &*self.transport }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/clob.v1.Query/QueryOrdersByAddress" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryOrdersByAddressResponse {
                        orders: vec![], total_count: 0, timestamp: None, pagination_response: None,
                    }))
                }
                "/clob.v1.Query/QueryOrderById" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryOrderByIdResponse { order: None }))
                }
                "/clob.v1.Query/QueryFundingRate" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryFundingRateResponse {
                        success: true, market_index: 42, ..Default::default()
                    }))
                }
                "/clob.v1.Query/GetChecksum" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetChecksumResponse {
                        market_index: 42, checksum: "abc123".into(), sequence_id: 100,
                        timestamp: None, block_height: 1000, is_valid: true,
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> ClobClient {
        ClobClient::new(SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"), Box::new(DummyTransport))
    }

    #[tokio::test]
    async fn query_orders_by_address_works() {
        let client = make_client();
        let result = client.query_orders_by_address(requests::QueryOrdersByAddressRequest::new("morpheum1abc")).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn query_order_by_id_works() {
        let client = make_client();
        let result = client.query_order_by_id("order-1", "morpheum1abc").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn query_funding_rate_works() {
        let client = make_client();
        let result = client.query_funding_rate(42).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_checksum_works() {
        let client = make_client();
        let result = client.get_checksum(42, 25).await;
        assert!(result.is_ok());
        let cs = result.unwrap();
        assert_eq!(cs.checksum, "abc123");
        assert!(cs.is_valid);
    }
}
