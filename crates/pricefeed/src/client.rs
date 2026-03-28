//! PriceFeedClient — queries for feed registry, prices, and feed listings.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::pricefeed::v1 as proto;

use crate::requests;
use crate::types::{PriceEntry, PriceFeed};

/// Result of listing feeds including total count.
pub struct FeedsPage {
    pub feeds: Vec<PriceFeed>,
    pub total_count: i32,
}

/// Primary client for price feed queries.
pub struct PriceFeedClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl PriceFeedClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets a single feed by index.
    pub async fn get_feed(&self, feed_index: u64) -> Result<PriceFeed, SdkError> {
        let req = requests::QueryFeedRequest::new(feed_index);
        let proto_req: proto::QueryFeedRequest = req.into();
        let resp = self.query("/pricefeed.v1.Query/QueryFeed", proto_req.encode_to_vec()).await?;
        let p = proto::QueryFeedResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.feed.map(Into::into).ok_or_else(|| SdkError::transport("feed field missing"))
    }

    /// Gets a single feed by symbol.
    pub async fn get_feed_by_symbol(&self, symbol: &str) -> Result<PriceFeed, SdkError> {
        let req = requests::QueryFeedBySymbolRequest::new(symbol);
        let proto_req: proto::QueryFeedBySymbolRequest = req.into();
        let resp = self.query("/pricefeed.v1.Query/QueryFeedBySymbol", proto_req.encode_to_vec()).await?;
        let p = proto::QueryFeedBySymbolResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.feed.map(Into::into).ok_or_else(|| SdkError::transport("feed field missing"))
    }

    /// Lists feeds with pagination and optional active-only filter.
    pub async fn list_feeds(&self, req: requests::QueryFeedsRequest) -> Result<FeedsPage, SdkError> {
        let proto_req: proto::QueryFeedsRequest = req.into();
        let resp = self.query("/pricefeed.v1.Query/QueryFeeds", proto_req.encode_to_vec()).await?;
        let p = proto::QueryFeedsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(FeedsPage {
            feeds: p.feeds.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        })
    }

    /// Gets the latest price for a feed by index.
    pub async fn get_price(&self, feed_index: u64) -> Result<PriceEntry, SdkError> {
        let req = requests::QueryPriceRequest::new(feed_index);
        let proto_req: proto::QueryPriceRequest = req.into();
        let resp = self.query("/pricefeed.v1.Query/QueryPrice", proto_req.encode_to_vec()).await?;
        let p = proto::QueryPriceResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.price.map(Into::into).ok_or_else(|| SdkError::transport("price field missing"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for PriceFeedClient {
    fn config(&self) -> &SdkConfig { &self.config }
    fn transport(&self) -> &dyn Transport { &*self.transport }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/pricefeed.v1.Query/QueryFeed" | "/pricefeed.v1.Query/QueryFeedBySymbol" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryFeedResponse {
                        success: true, error_message: String::new(),
                        feed: Some(proto::PriceFeed {
                            feed_index: 1, symbol: "BTC/USD".into(), active: true,
                            config: None, shard_id: "shard-0".into(),
                            created_at: None, updated_at: None,
                            base_asset_index: 0, quote_asset_index: 1,
                            feed_kind: 0, base_feed_index: 0, quote_feed_index: 0,
                        }),
                    }))
                }
                "/pricefeed.v1.Query/QueryFeeds" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryFeedsResponse {
                        success: true, error_message: String::new(),
                        feeds: alloc::vec![], total_count: 0,
                    }))
                }
                "/pricefeed.v1.Query/QueryPrice" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryPriceResponse {
                        success: true, error_message: String::new(),
                        price: Some(proto::PriceEntry {
                            value: 5_000_000_000_000, timestamp: 1700000000,
                            source_count: 5, confidence: 95,
                        }),
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> PriceFeedClient {
        PriceFeedClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_feed_works() {
        let f = make_client().get_feed(1).await.unwrap();
        assert_eq!(f.symbol, "BTC/USD");
        assert!(f.active);
    }

    #[tokio::test]
    async fn get_feed_by_symbol_works() {
        let f = make_client().get_feed_by_symbol("BTC/USD").await.unwrap();
        assert_eq!(f.feed_index, 1);
    }

    #[tokio::test]
    async fn list_feeds_works() {
        let page = make_client()
            .list_feeds(requests::QueryFeedsRequest::new(50, 0))
            .await.unwrap();
        assert!(page.feeds.is_empty());
        assert_eq!(page.total_count, 0);
    }

    #[tokio::test]
    async fn get_price_works() {
        let e = make_client().get_price(1).await.unwrap();
        assert_eq!(e.value, 5_000_000_000_000);
        assert_eq!(e.source_count, 5);
    }
}
