//! Request wrappers for the funding-rate module.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::fundingrate::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::{FundingMarketProfile, FundingPosition};

// ====================== TRANSACTION REQUESTS ======================

/// Trigger funding application at an epoch boundary.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EpochTickRequest {
    pub epoch_id: u64,
    pub logical_timestamp: u64,
}

impl EpochTickRequest {
    pub fn new(epoch_id: u64, logical_timestamp: u64) -> Self {
        Self { epoch_id, logical_timestamp }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::EpochTickRequest { epoch_id: self.epoch_id, logical_timestamp: self.logical_timestamp };
        ProtoAny { type_url: "/fundingrate.v1.EpochTickRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Apply funding rate to positions within a shard.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ApplyShardedFundingRequest {
    pub shard_id: String,
    pub market_index: u64,
    pub rate_satoshi: i64,
    pub logical_timestamp: u64,
    pub positions: Vec<FundingPosition>,
}

impl ApplyShardedFundingRequest {
    pub fn new(
        shard_id: impl Into<String>, market_index: u64,
        rate_satoshi: i64, logical_timestamp: u64,
    ) -> Self {
        Self {
            shard_id: shard_id.into(), market_index,
            rate_satoshi, logical_timestamp, positions: Vec::new(),
        }
    }

    pub fn positions(mut self, positions: Vec<FundingPosition>) -> Self {
        self.positions = positions; self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::ApplyShardedFundingRequest {
            shard_id: self.shard_id.clone(), market_index: self.market_index,
            rate_satoshi: self.rate_satoshi, logical_timestamp: self.logical_timestamp,
            positions: self.positions.iter().cloned().map(Into::into).collect(),
        };
        ProtoAny { type_url: "/fundingrate.v1.ApplyShardedFundingRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Update a market's funding profile (governance only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateMarketProfileRequest {
    pub market_index: u64,
    pub profile: FundingMarketProfile,
}

impl UpdateMarketProfileRequest {
    pub fn new(market_index: u64, profile: FundingMarketProfile) -> Self {
        Self { market_index, profile }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::UpdateMarketProfileRequest {
            market_index: self.market_index,
            profile: Some(self.profile.clone().into()),
        };
        ProtoAny { type_url: "/fundingrate.v1.UpdateMarketProfileRequest".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query current funding rate for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetFundingRateRequest {
    pub market_index: u64,
}

impl GetFundingRateRequest {
    pub fn new(market_index: u64) -> Self { Self { market_index } }
}

impl From<GetFundingRateRequest> for proto::GetFundingRateRequest {
    fn from(r: GetFundingRateRequest) -> Self { Self { market_index: r.market_index } }
}

/// Query next funding time for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetNextFundingTimeRequest {
    pub market_index: u64,
}

impl GetNextFundingTimeRequest {
    pub fn new(market_index: u64) -> Self { Self { market_index } }
}

impl From<GetNextFundingTimeRequest> for proto::GetNextFundingTimeRequest {
    fn from(r: GetNextFundingTimeRequest) -> Self { Self { market_index: r.market_index } }
}

/// Query market funding profile.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetMarketProfileRequest {
    pub market_index: u64,
}

impl GetMarketProfileRequest {
    pub fn new(market_index: u64) -> Self { Self { market_index } }
}

impl From<GetMarketProfileRequest> for proto::GetMarketProfileRequest {
    fn from(r: GetMarketProfileRequest) -> Self { Self { market_index: r.market_index } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FundingApplicationMode;

    #[test]
    fn epoch_tick_to_any() {
        let any = EpochTickRequest::new(1, 1_700_000_000).to_any();
        assert_eq!(any.type_url, "/fundingrate.v1.EpochTickRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn update_market_profile_to_any() {
        let profile = FundingMarketProfile {
            mode: FundingApplicationMode::BothSides,
            vrf_bias_bps: 0, protocol_cut_bps: 0, lp_incentive_bps: 0,
        };
        let any = UpdateMarketProfileRequest::new(42, profile).to_any();
        assert_eq!(any.type_url, "/fundingrate.v1.UpdateMarketProfileRequest");
    }

    #[test]
    fn query_request_conversions() {
        let p: proto::GetFundingRateRequest = GetFundingRateRequest::new(42).into();
        assert_eq!(p.market_index, 42);
        let p: proto::GetNextFundingTimeRequest = GetNextFundingTimeRequest::new(42).into();
        assert_eq!(p.market_index, 42);
    }
}
