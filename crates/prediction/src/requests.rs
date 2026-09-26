//! Request wrappers for the prediction market module.

use alloc::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::prediction::v1 as proto;

// ====================== QUERY REQUESTS ======================

/// Query a single prediction market by feed ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPredictionMarketRequest {
    pub feed_id: String,
}

impl QueryPredictionMarketRequest {
    pub fn new(feed_id: impl Into<String>) -> Self {
        Self {
            feed_id: feed_id.into(),
        }
    }
}

impl From<QueryPredictionMarketRequest> for proto::QueryPredictionMarketRequest {
    fn from(r: QueryPredictionMarketRequest) -> Self {
        Self { feed_id: r.feed_id }
    }
}

/// List prediction markets with pagination and optional phase filter.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPredictionMarketsRequest {
    pub limit: i32,
    pub offset: i32,
    /// Phase filter string: "active", "resolved", "disputed", "settled", "voided", "cancelled", or empty for all.
    pub phase_filter: String,
}

impl QueryPredictionMarketsRequest {
    pub fn new(limit: i32, offset: i32) -> Self {
        Self {
            limit,
            offset,
            phase_filter: String::new(),
        }
    }

    pub fn phase_filter(mut self, phase: impl Into<String>) -> Self {
        self.phase_filter = phase.into();
        self
    }
}

impl From<QueryPredictionMarketsRequest> for proto::QueryPredictionMarketsRequest {
    fn from(r: QueryPredictionMarketsRequest) -> Self {
        Self {
            limit: r.limit,
            offset: r.offset,
            phase_filter: r.phase_filter,
        }
    }
}

/// Query implied probability (1e9 scale) for a feed.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryImpliedProbabilityRequest {
    pub feed_id: String,
}

impl QueryImpliedProbabilityRequest {
    pub fn new(feed_id: impl Into<String>) -> Self {
        Self {
            feed_id: feed_id.into(),
        }
    }
}

impl From<QueryImpliedProbabilityRequest> for proto::QueryPredictionImpliedProbabilityRequest {
    fn from(r: QueryImpliedProbabilityRequest) -> Self {
        Self { feed_id: r.feed_id }
    }
}

/// Query cumulative fee statistics for a prediction market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMarketFeeStatsRequest {
    pub feed_id: String,
}

impl QueryMarketFeeStatsRequest {
    pub fn new(feed_id: impl Into<String>) -> Self {
        Self {
            feed_id: feed_id.into(),
        }
    }
}

impl From<QueryMarketFeeStatsRequest> for proto::QueryMarketFeeStatsRequest {
    fn from(r: QueryMarketFeeStatsRequest) -> Self {
        Self { feed_id: r.feed_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_conversions() {
        let p: proto::QueryPredictionMarketRequest = QueryPredictionMarketRequest::new("f1").into();
        assert_eq!(p.feed_id, "f1");

        let p: proto::QueryPredictionMarketsRequest = QueryPredictionMarketsRequest::new(50, 0)
            .phase_filter("active")
            .into();
        assert_eq!(p.phase_filter, "active");

        let p: proto::QueryPredictionImpliedProbabilityRequest =
            QueryImpliedProbabilityRequest::new("f1").into();
        assert_eq!(p.feed_id, "f1");
    }
}
