//! FundingRateClient — queries for funding rates, next funding time, and market profiles.

use alloc::boxed::Box;
use alloc::string::String;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::fundingrate::v1 as proto;

use crate::requests;
use crate::types::FundingMarketProfile;

/// Funding rate snapshot for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FundingRateSnapshot {
    pub funding_rate: i64,
    pub mark_price: u64,
    pub index_price: u64,
    pub ema_funding_rate: i64,
    pub symbol: String,
}

/// Primary client for funding-rate queries.
pub struct FundingRateClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl FundingRateClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets the current funding rate for a market.
    pub async fn get_funding_rate(&self, market_index: u64) -> Result<FundingRateSnapshot, SdkError> {
        let req = requests::GetFundingRateRequest::new(market_index);
        let proto_req: proto::GetFundingRateRequest = req.into();
        let resp = self.query("/fundingrate.v1.Query/GetFundingRate", proto_req.encode_to_vec()).await?;
        let p = proto::GetFundingRateResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(FundingRateSnapshot {
            funding_rate: p.funding_rate, mark_price: p.mark_price,
            index_price: p.index_price, ema_funding_rate: p.ema_funding_rate,
            symbol: p.symbol,
        })
    }

    /// Gets the next funding time (unix seconds) for a market.
    pub async fn get_next_funding_time(&self, market_index: u64) -> Result<u64, SdkError> {
        let req = requests::GetNextFundingTimeRequest::new(market_index);
        let proto_req: proto::GetNextFundingTimeRequest = req.into();
        let resp = self.query("/fundingrate.v1.Query/GetNextFundingTime", proto_req.encode_to_vec()).await?;
        let p = proto::GetNextFundingTimeResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.next_funding_time)
    }

    /// Gets the funding market profile for a market.
    pub async fn get_market_profile(&self, market_index: u64) -> Result<FundingMarketProfile, SdkError> {
        let req = requests::GetMarketProfileRequest::new(market_index);
        let proto_req: proto::GetMarketProfileRequest = req.into();
        let resp = self.query("/fundingrate.v1.Query/GetMarketProfile", proto_req.encode_to_vec()).await?;
        let p = proto::GetMarketProfileResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.profile.map(Into::into).ok_or_else(|| SdkError::transport("profile field missing"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for FundingRateClient {
    fn config(&self) -> &SdkConfig { &self.config }
    fn transport(&self) -> &dyn Transport { &*self.transport }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/fundingrate.v1.Query/GetFundingRate" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetFundingRateResponse {
                        funding_rate: -500, mark_price: 5_000_000_000_000,
                        index_price: 4_999_000_000_000, ema_funding_rate: -480,
                        symbol: "BTC-USDC-PERP".into(),
                    }))
                }
                "/fundingrate.v1.Query/GetNextFundingTime" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetNextFundingTimeResponse {
                        next_funding_time: 1_700_028_800,
                    }))
                }
                "/fundingrate.v1.Query/GetMarketProfile" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetMarketProfileResponse {
                        profile: Some(proto::FundingMarketProfile {
                            mode: 1, vrf_bias_bps: 0, protocol_cut_bps: 0, lp_incentive_bps: 0,
                        }),
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> FundingRateClient {
        FundingRateClient::new(SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"), Box::new(DummyTransport))
    }

    #[tokio::test]
    async fn get_funding_rate_works() {
        let snap = make_client().get_funding_rate(42).await.unwrap();
        assert_eq!(snap.funding_rate, -500);
        assert_eq!(snap.symbol, "BTC-USDC-PERP");
    }

    #[tokio::test]
    async fn get_next_funding_time_works() {
        let t = make_client().get_next_funding_time(42).await.unwrap();
        assert_eq!(t, 1_700_028_800);
    }

    #[tokio::test]
    async fn get_market_profile_works() {
        let profile = make_client().get_market_profile(42).await.unwrap();
        assert_eq!(profile.mode, crate::types::FundingApplicationMode::BothSides);
    }
}
