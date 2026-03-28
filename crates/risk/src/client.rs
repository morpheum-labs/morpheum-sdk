//! RiskClient — queries for heatmaps, OI ratios, and maintenance margin.

use alloc::boxed::Box;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::risk::v1 as proto;

use crate::requests;

/// Heatmap query result.
pub struct HeatmapResult {
    pub market_index: u64,
    pub bands_count: u32,
    pub mark_price: u64,
    /// u128 as decimal string.
    pub total_at_risk: alloc::string::String,
}

/// OI ratio query result.
pub struct OiRatioResult {
    /// u128 as decimal string.
    pub long_oi: alloc::string::String,
    /// u128 as decimal string.
    pub short_oi: alloc::string::String,
}

/// Primary client for risk module queries.
pub struct RiskClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl RiskClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets the liquidation heatmap for a market.
    pub async fn get_heatmap(&self, market_index: u64, depth: u32) -> Result<HeatmapResult, SdkError> {
        let req = requests::GetHeatmapRequest::new(market_index, depth);
        let proto_req: proto::GetHeatmapRequest = req.into();
        let resp = self.query("/risk.v1.Query/GetHeatmap", proto_req.encode_to_vec()).await?;
        let p = proto::GetHeatmapResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(HeatmapResult {
            market_index: p.market_index, bands_count: p.bands_count,
            mark_price: p.mark_price, total_at_risk: p.total_at_risk,
        })
    }

    /// Gets the long/short OI ratio for a market.
    pub async fn get_oi_ratio(&self, market_index: u64) -> Result<OiRatioResult, SdkError> {
        let req = requests::GetOiRatioRequest::new(market_index);
        let proto_req: proto::GetOiRatioRequest = req.into();
        let resp = self.query("/risk.v1.Query/GetOiRatio", proto_req.encode_to_vec()).await?;
        let p = proto::GetOiRatioResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(OiRatioResult { long_oi: p.long_oi, short_oi: p.short_oi })
    }

    /// Gets the maintenance margin for a position.
    pub async fn get_maintenance_margin(&self, req: requests::GetMaintenanceMarginRequest) -> Result<alloc::string::String, SdkError> {
        let proto_req: proto::GetMaintenanceMarginRequest = req.into();
        let resp = self.query("/risk.v1.Query/GetMaintenanceMargin", proto_req.encode_to_vec()).await?;
        let p = proto::GetMaintenanceMarginResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.margin)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for RiskClient {
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
                "/risk.v1.Query/GetHeatmap" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetHeatmapResponse {
                        market_index: 0, bands_count: 10,
                        mark_price: 50000_00000000, total_at_risk: "1000000".into(),
                    }))
                }
                "/risk.v1.Query/GetOiRatio" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetOiRatioResponse {
                        long_oi: "5000000".into(), short_oi: "3000000".into(),
                    }))
                }
                "/risk.v1.Query/GetMaintenanceMargin" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetMaintenanceMarginResponse {
                        margin: "50000".into(),
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> RiskClient {
        RiskClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_heatmap_works() {
        let h = make_client().get_heatmap(0, 10).await.unwrap();
        assert_eq!(h.bands_count, 10);
        assert_eq!(h.total_at_risk, "1000000");
    }

    #[tokio::test]
    async fn get_oi_ratio_works() {
        let r = make_client().get_oi_ratio(0).await.unwrap();
        assert_eq!(r.long_oi, "5000000");
        assert_eq!(r.short_oi, "3000000");
    }

    #[tokio::test]
    async fn get_maintenance_margin_works() {
        let m = make_client()
            .get_maintenance_margin(requests::GetMaintenanceMarginRequest::new(0, "100000", 50000, true, 10, 51000))
            .await.unwrap();
        assert_eq!(m, "50000");
    }
}
