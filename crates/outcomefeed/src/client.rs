//! OutcomeFeedClient — queries for resolved outcomes, feed details, and feed listings.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::outcomefeed::v1 as proto;

use crate::requests;
use crate::types::{PredictionMarketFeed, ResolvedOutcome};

/// Result of listing prediction feeds including total count.
pub struct PredictionFeedsPage {
    pub feeds: Vec<PredictionMarketFeed>,
    pub total_count: i32,
}

/// Primary client for outcome feed queries.
pub struct OutcomeFeedClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl OutcomeFeedClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets the resolved outcome for a feed.
    pub async fn get_resolved_outcome(&self, feed_id: &str) -> Result<ResolvedOutcome, SdkError> {
        let req = requests::QueryResolvedOutcomeRequest::new(feed_id);
        let proto_req: proto::QueryResolvedOutcomeRequest = req.into();
        let resp = self.query("/outcomefeed.v1.Query/QueryResolvedOutcome", proto_req.encode_to_vec()).await?;
        let p = proto::QueryResolvedOutcomeResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.outcome.map(Into::into).ok_or_else(|| SdkError::transport("outcome field missing"))
    }

    /// Gets a single prediction market feed by ID.
    pub async fn get_prediction_feed(&self, feed_id: &str) -> Result<PredictionMarketFeed, SdkError> {
        let req = requests::QueryPredictionFeedRequest::new(feed_id);
        let proto_req: proto::QueryPredictionFeedRequest = req.into();
        let resp = self.query("/outcomefeed.v1.Query/QueryPredictionFeed", proto_req.encode_to_vec()).await?;
        let p = proto::QueryPredictionFeedResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.feed.map(Into::into).ok_or_else(|| SdkError::transport("feed field missing"))
    }

    /// Lists prediction market feeds with pagination and optional filters.
    pub async fn list_prediction_feeds(&self, req: requests::QueryPredictionFeedsRequest) -> Result<PredictionFeedsPage, SdkError> {
        let proto_req: proto::QueryPredictionFeedsRequest = req.into();
        let resp = self.query("/outcomefeed.v1.Query/QueryPredictionFeeds", proto_req.encode_to_vec()).await?;
        let p = proto::QueryPredictionFeedsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(PredictionFeedsPage {
            feeds: p.feeds.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        })
    }
}

#[async_trait(?Send)]
impl MorpheumClient for OutcomeFeedClient {
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
                "/outcomefeed.v1.Query/QueryResolvedOutcome" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryResolvedOutcomeResponse {
                        success: true, error_message: String::new(),
                        outcome: Some(proto::ResolvedOutcome {
                            outcome: "yes".into(), confidence_bps: 10000,
                            final_ts: 1_700_000_000, resolution_source: "oracle".into(),
                            paradigm: 1, evidence_hash: "abc".into(),
                        }),
                    }))
                }
                "/outcomefeed.v1.Query/QueryPredictionFeed" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryPredictionFeedResponse {
                        success: true, error_message: String::new(),
                        feed: Some(proto::PredictionMarketFeed {
                            feed_id: "btc-50k".into(), paradigm: 2,
                            criteria: None, status: 1,
                        }),
                    }))
                }
                "/outcomefeed.v1.Query/QueryPredictionFeeds" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryPredictionFeedsResponse {
                        success: true, error_message: String::new(),
                        feeds: alloc::vec![], total_count: 0,
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> OutcomeFeedClient {
        OutcomeFeedClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_resolved_outcome_works() {
        let o = make_client().get_resolved_outcome("f1").await.unwrap();
        assert_eq!(o.outcome, "yes");
        assert_eq!(o.confidence_bps, 10000);
    }

    #[tokio::test]
    async fn get_prediction_feed_works() {
        let f = make_client().get_prediction_feed("btc-50k").await.unwrap();
        assert_eq!(f.feed_id, "btc-50k");
        assert_eq!(f.paradigm, crate::types::ResolutionParadigm::MarketPrice);
    }

    #[tokio::test]
    async fn list_prediction_feeds_works() {
        let page = make_client()
            .list_prediction_feeds(requests::QueryPredictionFeedsRequest::new(50, 0))
            .await.unwrap();
        assert!(page.feeds.is_empty());
        assert_eq!(page.total_count, 0);
    }
}
