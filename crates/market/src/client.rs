//! MarketClient — the main entry point for all market-related operations
//! in the Morpheum SDK.
//!
//! This client provides high-level, type-safe methods for querying markets,
//! active markets, market statistics, and market parameters. Transaction
//! operations (create, activate, suspend, update, margin changes) are handled
//! via the fluent builders in `builder.rs` + `TxBuilder`.

use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{
    MorpheumClient, SdkConfig, SdkError, Transport,
};

use crate::{
    requests::{
        QueryActiveMarketsRequest,
        QueryMarketRequest,
        QueryMarketStatsRequest,
        QueryMarketsRequest,
    },
    types::{Market, MarketStats},
};

/// Primary client for all market-related queries.
///
/// Transaction construction (create, activate, suspend, update, margin changes)
/// is delegated to the fluent builders in `builder.rs` for maximum ergonomics
/// and type safety.
pub struct MarketClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl MarketClient {
    /// Creates a new `MarketClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a single market by its index.
    pub async fn query_market(
        &self,
        market_index: u64,
        shard_id: Option<String>,
    ) -> Result<Market, SdkError> {
        let req = QueryMarketRequest::new(market_index).with_shard_id_opt(shard_id);
        let proto_req: morpheum_proto::market::v1::QueryMarketRequest = req.into();

        let path = "/market.v1.Query/QueryMarket";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::market::v1::QueryMarketResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .market
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("market field missing in response"))
    }

    /// Queries a list of markets with pagination and optional filters.
    pub async fn query_markets(
        &self,
        limit: u32,
        offset: u32,
        status_filter: Option<String>,
        type_filter: Option<crate::types::MarketType>,
    ) -> Result<Vec<Market>, SdkError> {
        let req = QueryMarketsRequest::new(limit, offset)
            .status_filter_opt(status_filter)
            .type_filter_opt(type_filter);

        let proto_req: morpheum_proto::market::v1::QueryMarketsRequest = req.into();

        let path = "/market.v1.Query/QueryMarkets";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::market::v1::QueryMarketsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.markets.into_iter().map(Into::into).collect())
    }

    /// Queries only active (tradable) markets with pagination.
    pub async fn query_active_markets(
        &self,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Market>, SdkError> {
        let req = QueryActiveMarketsRequest::new(limit, offset);
        let proto_req: morpheum_proto::market::v1::QueryActiveMarketsRequest = req.into();

        let path = "/market.v1.Query/QueryActiveMarkets";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::market::v1::QueryActiveMarketsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.markets.into_iter().map(Into::into).collect())
    }

    /// Queries statistics for a specific market.
    pub async fn query_market_stats(
        &self,
        market_index: u64,
        time_range: Option<String>,
        shard_id: Option<String>,
    ) -> Result<MarketStats, SdkError> {
        let req = QueryMarketStatsRequest::new(market_index)
            .time_range_opt(time_range)
            .shard_id_opt(shard_id);

        let proto_req: morpheum_proto::market::v1::QueryMarketStatsRequest = req.into();

        let path = "/market.v1.Query/QueryMarketStats";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::market::v1::QueryMarketStatsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .stats
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("stats field missing in response"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for MarketClient {
    fn config(&self) -> &SdkConfig {
        &self.config
    }

    fn transport(&self) -> &dyn Transport {
        &*self.transport
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morpheum_sdk_core::SdkConfig;

    // Dummy transport for testing
    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _tx_bytes: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!("not needed for market query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/market.v1.Query/QueryMarket" => {
                        let dummy = morpheum_proto::market::v1::QueryMarketResponse {
                        success: true,
                        error_message: "".into(),
                        market: Some(Default::default()),
                        shard_id: "shard-1".into(),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/market.v1.Query/QueryMarkets" | "/market.v1.Query/QueryActiveMarkets" => {
                    let dummy = morpheum_proto::market::v1::QueryMarketsResponse {
                        success: true,
                        error_message: "".into(),
                        markets: vec![],
                        total_count: 0,
                        shard_id: "shard-1".into(),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/market.v1.Query/QueryMarketStats" => {
                    let dummy = morpheum_proto::market::v1::QueryMarketStatsResponse {
                        success: true,
                        error_message: "".into(),
                        stats: Some(Default::default()),
                        shard_id: "shard-1".into(),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn market_client_query_market_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = MarketClient::new(config, Box::new(DummyTransport));

        let result = client.query_market(42, None).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn market_client_query_active_markets_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = MarketClient::new(config, Box::new(DummyTransport));

        let result = client.query_active_markets(10, 0).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn market_client_query_market_stats_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = MarketClient::new(config, Box::new(DummyTransport));

        let result = client.query_market_stats(42, None, None).await;
        assert!(result.is_ok());
    }
}