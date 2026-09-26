//! Request wrappers for the outcome feed module.

use alloc::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::outcomefeed::v1 as proto;

use crate::types::{FeedStatus, ResolutionParadigm};

// ====================== QUERY REQUESTS ======================

/// Query the resolved outcome for a feed.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryResolvedOutcomeRequest {
    pub feed_id: String,
}

impl QueryResolvedOutcomeRequest {
    pub fn new(feed_id: impl Into<String>) -> Self {
        Self {
            feed_id: feed_id.into(),
        }
    }
}

impl From<QueryResolvedOutcomeRequest> for proto::QueryResolvedOutcomeRequest {
    fn from(r: QueryResolvedOutcomeRequest) -> Self {
        Self { feed_id: r.feed_id }
    }
}

/// Query a single prediction market feed by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPredictionFeedRequest {
    pub feed_id: String,
}

impl QueryPredictionFeedRequest {
    pub fn new(feed_id: impl Into<String>) -> Self {
        Self {
            feed_id: feed_id.into(),
        }
    }
}

impl From<QueryPredictionFeedRequest> for proto::QueryPredictionFeedRequest {
    fn from(r: QueryPredictionFeedRequest) -> Self {
        Self { feed_id: r.feed_id }
    }
}

/// List prediction market feeds with pagination and optional filters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPredictionFeedsRequest {
    pub limit: i32,
    pub offset: i32,
    pub paradigm_filter: Option<ResolutionParadigm>,
    pub status_filter: Option<FeedStatus>,
}

impl QueryPredictionFeedsRequest {
    pub fn new(limit: i32, offset: i32) -> Self {
        Self {
            limit,
            offset,
            paradigm_filter: None,
            status_filter: None,
        }
    }

    pub fn paradigm_filter(mut self, p: ResolutionParadigm) -> Self {
        self.paradigm_filter = Some(p);
        self
    }
    pub fn status_filter(mut self, s: FeedStatus) -> Self {
        self.status_filter = Some(s);
        self
    }
}

impl From<QueryPredictionFeedsRequest> for proto::QueryPredictionFeedsRequest {
    fn from(r: QueryPredictionFeedsRequest) -> Self {
        Self {
            limit: r.limit,
            offset: r.offset,
            paradigm_filter: r.paradigm_filter.map_or(0, i32::from),
            status_filter: r.status_filter.map_or(0, i32::from),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_conversions() {
        let p: proto::QueryResolvedOutcomeRequest = QueryResolvedOutcomeRequest::new("f1").into();
        assert_eq!(p.feed_id, "f1");

        let p: proto::QueryPredictionFeedRequest = QueryPredictionFeedRequest::new("f1").into();
        assert_eq!(p.feed_id, "f1");
    }

    #[test]
    fn query_feeds_with_filters() {
        let p: proto::QueryPredictionFeedsRequest = QueryPredictionFeedsRequest::new(50, 0)
            .paradigm_filter(ResolutionParadigm::MarketPrice)
            .status_filter(FeedStatus::Active)
            .into();
        assert_eq!(p.limit, 50);
        assert_eq!(p.paradigm_filter, 2);
        assert_eq!(p.status_filter, 1);
    }
}
