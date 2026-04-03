//! Request wrappers for the funding-rate module.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::fundingrate::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::FundingMarketProfile;

// ====================== TRANSACTION REQUESTS ======================

/// Update a market's funding profile (governance only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateMarketProfileRequest {
    pub authority: String,
    pub market_index: u64,
    pub profile: FundingMarketProfile,
}

impl UpdateMarketProfileRequest {
    pub fn new(
        authority: impl Into<String>,
        market_index: u64,
        profile: FundingMarketProfile,
    ) -> Self {
        Self {
            authority: authority.into(),
            market_index,
            profile,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateMarketConfig {
            authority: self.authority.clone(),
            market_index: self.market_index,
            profile: Some(self.profile.clone().into()),
        };
        ProtoAny { type_url: "/fundingrate.v1.MsgUpdateMarketConfig".into(), value: msg.encode_to_vec() }
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
    fn update_market_profile_to_any() {
        let profile = FundingMarketProfile {
            mode: FundingApplicationMode::BothSides,
            vrf_bias_bps: 0, protocol_cut_bps: 0, lp_incentive_bps: 0,
        };
        let any = UpdateMarketProfileRequest::new("morpheum1gov", 42, profile).to_any();
        assert_eq!(any.type_url, "/fundingrate.v1.MsgUpdateMarketConfig");
    }

    #[test]
    fn query_request_conversions() {
        let p: proto::GetFundingRateRequest = GetFundingRateRequest::new(42).into();
        assert_eq!(p.market_index, 42);
        let p: proto::GetNextFundingTimeRequest = GetNextFundingTimeRequest::new(42).into();
        assert_eq!(p.market_index, 42);
    }
}
