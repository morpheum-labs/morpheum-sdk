//! Request wrappers for the kline module.

use alloc::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::kline::v1 as proto;

// ====================== QUERY REQUESTS ======================

/// Query mark price with spread for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetMarkPriceWithSpreadRequest {
    pub market_index: u64,
    pub logical_timestamp: u64,
}

impl GetMarkPriceWithSpreadRequest {
    pub fn new(market_index: u64, logical_timestamp: u64) -> Self {
        Self { market_index, logical_timestamp }
    }
}

impl From<GetMarkPriceWithSpreadRequest> for proto::GetMarkPriceWithSpreadRequest {
    fn from(r: GetMarkPriceWithSpreadRequest) -> Self {
        Self { market_index: r.market_index, logical_timestamp: r.logical_timestamp }
    }
}

/// Query VWAP over a time range.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetVwapRequest {
    pub market_index: u64,
    pub start_logical: u64,
    pub end_logical: u64,
}

impl GetVwapRequest {
    pub fn new(market_index: u64, start_logical: u64, end_logical: u64) -> Self {
        Self { market_index, start_logical, end_logical }
    }
}

impl From<GetVwapRequest> for proto::GetVwapRequest {
    fn from(r: GetVwapRequest) -> Self {
        Self { market_index: r.market_index, start_logical: r.start_logical, end_logical: r.end_logical }
    }
}

/// Query long/short ratio for sentiment.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetLongShortRatioRequest {
    pub market_index: u64,
    pub period: u32,
    pub open_at_logical: u64,
}

impl GetLongShortRatioRequest {
    pub fn new(market_index: u64, period: u32, open_at_logical: u64) -> Self {
        Self { market_index, period, open_at_logical }
    }
}

impl From<GetLongShortRatioRequest> for proto::GetLongShortRatioRequest {
    fn from(r: GetLongShortRatioRequest) -> Self {
        Self { market_index: r.market_index, period: r.period, open_at_logical: r.open_at_logical }
    }
}

/// Query last completed OHLC kline for a market/period.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetLastKlineRequest {
    pub market_index: u64,
    pub period: u32,
    pub logical_timestamp: u64,
}

impl GetLastKlineRequest {
    pub fn new(market_index: u64, period: u32, logical_timestamp: u64) -> Self {
        Self { market_index, period, logical_timestamp }
    }
}

impl From<GetLastKlineRequest> for proto::GetLastKlineRequest {
    fn from(r: GetLastKlineRequest) -> Self {
        Self { market_index: r.market_index, period: r.period, logical_timestamp: r.logical_timestamp }
    }
}

/// Snapshot query: batch of klines over a time range.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryKlinesSnapshotRequest {
    pub market_index: u64,
    pub interval: String,
    pub start_time: u64,
    pub end_time: u64,
}

impl QueryKlinesSnapshotRequest {
    pub fn new(market_index: u64, interval: impl Into<String>, start_time: u64, end_time: u64) -> Self {
        Self { market_index, interval: interval.into(), start_time, end_time }
    }
}

impl From<QueryKlinesSnapshotRequest> for proto::QueryKlinesSnapshotRequest {
    fn from(r: QueryKlinesSnapshotRequest) -> Self {
        Self { market_index: r.market_index, interval: r.interval, start_time: r.start_time, end_time: r.end_time }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_conversions() {
        let p: proto::GetMarkPriceWithSpreadRequest = GetMarkPriceWithSpreadRequest::new(1, 100).into();
        assert_eq!(p.market_index, 1);

        let p: proto::GetVwapRequest = GetVwapRequest::new(1, 100, 200).into();
        assert_eq!(p.end_logical, 200);

        let p: proto::GetLastKlineRequest = GetLastKlineRequest::new(1, 4, 100).into();
        assert_eq!(p.period, 4);

        let p: proto::QueryKlinesSnapshotRequest = QueryKlinesSnapshotRequest::new(1, "1h", 100, 200).into();
        assert_eq!(p.interval, "1h");
    }
}
