//! PredictionClient — queries for prediction markets and implied probabilities.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::prediction::v1 as proto;

use crate::requests;
use crate::types::PredictionMarket;

/// Result of listing prediction markets including total count.
pub struct PredictionMarketsPage {
    pub markets: Vec<PredictionMarket>,
    pub total_count: i32,
}

/// Primary client for prediction market queries.
pub struct PredictionClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl PredictionClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets a single prediction market by feed ID.
    pub async fn get_market(&self, feed_id: &str) -> Result<PredictionMarket, SdkError> {
        let req = requests::QueryPredictionMarketRequest::new(feed_id);
        let proto_req: proto::QueryPredictionMarketRequest = req.into();
        let resp = self.query("/prediction.v1.Query/QueryPredictionMarket", proto_req.encode_to_vec()).await?;
        let p = proto::QueryPredictionMarketResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.market.map(Into::into).ok_or_else(|| SdkError::transport("market field missing"))
    }

    /// Lists prediction markets with pagination and optional phase filter.
    pub async fn list_markets(&self, req: requests::QueryPredictionMarketsRequest) -> Result<PredictionMarketsPage, SdkError> {
        let proto_req: proto::QueryPredictionMarketsRequest = req.into();
        let resp = self.query("/prediction.v1.Query/QueryPredictionMarkets", proto_req.encode_to_vec()).await?;
        let p = proto::QueryPredictionMarketsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(PredictionMarketsPage {
            markets: p.markets.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        })
    }

    /// Gets the implied probability (1e9 scale) for a feed.
    pub async fn get_implied_probability(&self, feed_id: &str) -> Result<u32, SdkError> {
        let req = requests::QueryImpliedProbabilityRequest::new(feed_id);
        let proto_req: proto::QueryPredictionImpliedProbabilityRequest = req.into();
        let resp = self.query("/prediction.v1.Query/QueryPredictionImpliedProbability", proto_req.encode_to_vec()).await?;
        let p = proto::QueryPredictionImpliedProbabilityResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.implied_probability)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for PredictionClient {
    fn config(&self) -> &SdkConfig { &self.config }
    fn transport(&self) -> &dyn Transport { &*self.transport }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use alloc::vec;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/prediction.v1.Query/QueryPredictionMarket" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryPredictionMarketResponse {
                        success: true, error_message: String::new(),
                        market: Some(proto::PredictionMarket {
                            feed_id: "btc-50k".into(),
                            outcomes: vec!["yes".into(), "no".into()],
                            creator: "morph1xyz".into(), phase: 1,
                            locked_stake: "100000".into(), pot: "500000".into(),
                            dispute_deadline: 1000, resolved_outcome: None,
                            dispute_config: None, accumulated_fees: 0,
                            current_confidence: None, spread_bps: 50,
                            depth: 10000, daily_volume: 5000, quote_asset_index: 1,
                        }),
                    }))
                }
                "/prediction.v1.Query/QueryPredictionMarkets" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryPredictionMarketsResponse {
                        success: true, error_message: String::new(),
                        markets: vec![], total_count: 0,
                    }))
                }
                "/prediction.v1.Query/QueryPredictionImpliedProbability" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryPredictionImpliedProbabilityResponse {
                        success: true, error_message: String::new(),
                        implied_probability: 650_000_000,
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> PredictionClient {
        PredictionClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_market_works() {
        let m = make_client().get_market("btc-50k").await.unwrap();
        assert_eq!(m.feed_id, "btc-50k");
        assert_eq!(m.phase, crate::types::PredictionPhase::Active);
        assert_eq!(m.outcomes.len(), 2);
    }

    #[tokio::test]
    async fn list_markets_works() {
        let page = make_client()
            .list_markets(requests::QueryPredictionMarketsRequest::new(50, 0))
            .await.unwrap();
        assert!(page.markets.is_empty());
        assert_eq!(page.total_count, 0);
    }

    #[tokio::test]
    async fn get_implied_probability_works() {
        let prob = make_client().get_implied_probability("f1").await.unwrap();
        assert_eq!(prob, 650_000_000);
    }
}
