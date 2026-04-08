//! ClmmClient — main entry point for all CLMM-related queries.
//!
//! Covers position lookup, swap simulation, AMM quoting, liquidity depth,
//! pool risk, boosted buffer, and ReClmm glide simulation.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::generated::clmm::v1 as proto;

use crate::requests;
use crate::types::{
    BoostedBuffer, ClmmPosition, GlideSimulation, LiquidityDepthBand,
    PoolFeeStats, PoolRiskSummary, QuoteResult, SwapSimulation,
};

fn check_success(success: bool, err: &str) -> Result<(), SdkError> {
    if success { Ok(()) } else {
        Err(SdkError::transport(if err.is_empty() { "operation failed" } else { err }))
    }
}

/// Primary client for all CLMM queries and simulations.
pub struct ClmmClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl ClmmClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Fetches a CLMM position by ID.
    pub async fn get_position(&self, position_id: impl Into<alloc::string::String>) -> Result<ClmmPosition, SdkError> {
        let req = requests::GetPositionRequest::new(position_id);
        let proto_req: proto::GetPositionRequest = req.into();
        let resp = self.query("/clmm.v1.Query/GetPosition", proto_req.encode_to_vec()).await?;
        let p = proto::GetPositionResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        p.position.map(Into::into).ok_or_else(|| SdkError::transport("position field missing"))
    }

    /// Simulates a swap and returns the expected outcome.
    pub async fn simulate_swap(&self, request: requests::SimulateSwapRequest) -> Result<SwapSimulation, SdkError> {
        let proto_req: proto::SimulateSwapRequest = request.into();
        let resp = self.query("/clmm.v1.Query/SimulateSwap", proto_req.encode_to_vec()).await?;
        let p = proto::SimulateSwapResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(SwapSimulation {
            amount_in: p.amount_in, amount_out: p.amount_out,
            fee_amount: p.fee_amount, price_impact: p.price_impact,
        })
    }

    /// Gets an AMM quote for a swap.
    pub async fn get_quote(&self, request: requests::GetQuoteRequest) -> Result<QuoteResult, SdkError> {
        let proto_req: proto::GetQuoteRequest = request.into();
        let resp = self.query("/clmm.v1.Query/GetQuote", proto_req.encode_to_vec()).await?;
        let p = proto::GetQuoteResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(QuoteResult {
            amount_in: p.amount_in, amount_out: p.amount_out,
            fee_amount: p.fee_amount, price: p.price,
        })
    }

    /// Queries liquidity depth bands for a pool.
    pub async fn get_liquidity_depth(
        &self, request: requests::GetLiquidityDepthRequest,
    ) -> Result<Vec<LiquidityDepthBand>, SdkError> {
        let proto_req: proto::GetLiquidityDepthRequest = request.into();
        let resp = self.query("/clmm.v1.Query/GetLiquidityDepth", proto_req.encode_to_vec()).await?;
        let p = proto::GetLiquidityDepthResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(p.bands.into_iter().map(Into::into).collect())
    }

    /// Queries pool risk summary.
    pub async fn get_pool_risk_summary(
        &self, pool_id: impl Into<alloc::string::String>,
    ) -> Result<PoolRiskSummary, SdkError> {
        let req = requests::GetPoolRiskSummaryRequest::new(pool_id);
        let proto_req: proto::GetPoolRiskSummaryRequest = req.into();
        let resp = self.query("/clmm.v1.Query/GetPoolRiskSummary", proto_req.encode_to_vec()).await?;
        let p = proto::GetPoolRiskSummaryResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(PoolRiskSummary {
            pool_id: p.pool_id, health_score_bps: p.health_score_bps,
            utilization_rate: p.utilization_rate, concentration_risk: p.concentration_risk,
        })
    }

    /// Queries boosted pool buffer state.
    pub async fn get_boosted_buffer(
        &self, pool_id: impl Into<alloc::string::String>,
    ) -> Result<BoostedBuffer, SdkError> {
        let req = requests::GetBoostedBufferRequest::new(pool_id);
        let proto_req: proto::GetBoostedBufferRequest = req.into();
        let resp = self.query("/clmm.v1.Query/GetBoostedBuffer", proto_req.encode_to_vec()).await?;
        let p = proto::GetBoostedBufferResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(BoostedBuffer {
            pool_id: p.pool_id, buffer_amount: p.buffer_amount,
            pending_yield: p.pending_yield, apy_estimate: p.apy_estimate,
        })
    }

    /// Queries cumulative fee statistics for a pool.
    pub async fn query_pool_fee_stats(
        &self, pool_id: impl Into<alloc::string::String>,
    ) -> Result<PoolFeeStats, SdkError> {
        let req = requests::QueryPoolFeeStatsRequest::new(pool_id);
        let proto_req: proto::QueryPoolFeeStatsRequest = req.into();
        let resp = self.query("/clmm.v1.Query/QueryPoolFeeStats", proto_req.encode_to_vec()).await?;
        let p = proto::QueryPoolFeeStatsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(PoolFeeStats::from(p))
    }

    /// Simulates a ReClmm glide operation.
    pub async fn simulate_reclmm_glide(
        &self, request: requests::SimulateReClmmGlideRequest,
    ) -> Result<GlideSimulation, SdkError> {
        let proto_req: proto::SimulateReClmmGlideRequest = request.into();
        let resp = self.query("/clmm.v1.Query/SimulateReClmmGlide", proto_req.encode_to_vec()).await?;
        let p = proto::SimulateReClmmGlideResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(GlideSimulation {
            current_virtual_price: p.current_virtual_price,
            projected_virtual_price: p.projected_virtual_price,
            estimated_trades: p.estimated_trades,
        })
    }
}

#[async_trait(?Send)]
impl MorpheumClient for ClmmClient {
    fn config(&self) -> &SdkConfig { &self.config }
    fn transport(&self) -> &dyn Transport { &*self.transport }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use alloc::vec::Vec;
    use crate::types::Side;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/clmm.v1.Query/GetPosition" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetPositionResponse {
                        success: true,
                        position: Some(proto::ClmmPosition {
                            position_id: "pos-1".into(),
                            pool_id: "0x1234".into(),
                            tick_lower: -100,
                            tick_upper: 100,
                            ..Default::default()
                        }),
                        error_message: String::new(),
                        timestamp: None,
                    }))
                }
                "/clmm.v1.Query/SimulateSwap" => {
                    Ok(prost::Message::encode_to_vec(&proto::SimulateSwapResponse {
                        success: true, amount_in: "1000".into(), amount_out: "990".into(),
                        fee_amount: "3".into(), price_impact: "0.1".into(),
                        error_message: String::new(), timestamp: None,
                    }))
                }
                "/clmm.v1.Query/GetPoolRiskSummary" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetPoolRiskSummaryResponse {
                        success: true, pool_id: "0x1234".into(),
                        health_score_bps: "9500".into(), utilization_rate: "0.45".into(),
                        concentration_risk: "0.2".into(), error_message: String::new(), timestamp: None,
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> ClmmClient {
        ClmmClient::new(SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"), Box::new(DummyTransport))
    }

    #[tokio::test]
    async fn get_position_works() {
        let pos = make_client().get_position("pos-1").await.unwrap();
        assert_eq!(pos.position_id, "pos-1");
        assert_eq!(pos.tick_lower, -100);
    }

    #[tokio::test]
    async fn simulate_swap_works() {
        let sim = make_client().simulate_swap(
            requests::SimulateSwapRequest::new("0x1234", 42, Side::Buy, "1000")
        ).await.unwrap();
        assert_eq!(sim.amount_out, "990");
    }

    #[tokio::test]
    async fn get_pool_risk_works() {
        let risk = make_client().get_pool_risk_summary("0x1234").await.unwrap();
        assert_eq!(risk.health_score_bps, "9500");
    }
}
