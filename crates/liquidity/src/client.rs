//! LiquidityClient — queries for pools, depth metrics, and pool health.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::liquidity::v1 as proto;

use crate::requests;
use crate::types::{DepthMetrics, PageInfo, Pool, PoolHealth};

macro_rules! svc_path {
    ($method:expr) => { concat!("/liquidity.v1.LiquidityService/", $method) };
}

/// Primary client for liquidity pool queries.
pub struct LiquidityClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl LiquidityClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets a specific pool by ID.
    pub async fn get_pool(&self, pool_id: &str) -> Result<Pool, SdkError> {
        let req = requests::GetPoolRequest::new(pool_id);
        let proto_req: proto::GetPoolRequest = req.into();
        let resp = self.query(svc_path!("GetPool"), proto_req.encode_to_vec()).await?;
        let p = proto::GetPoolResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.pool.map(Into::into).ok_or_else(|| SdkError::transport("pool field missing"))
    }

    /// Lists all pools with pagination and optional status filter.
    pub async fn list_pools(&self, req: requests::ListPoolsRequest) -> Result<(Vec<Pool>, PageInfo), SdkError> {
        let proto_req: proto::ListPoolsRequest = req.into();
        let resp = self.query(svc_path!("ListPools"), proto_req.encode_to_vec()).await?;
        let p = proto::ListPoolsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        let pools = p.pools.into_iter().map(Into::into).collect();
        let page = p.pagination.map_or(PageInfo { next_key: Vec::new(), total: 0 }, |pg| {
            PageInfo { next_key: pg.next_key, total: pg.total }
        });
        Ok((pools, page))
    }

    /// Gets all pools for a specific market with pagination.
    pub async fn get_pools_by_market(&self, req: requests::GetPoolsByMarketRequest) -> Result<(Vec<Pool>, PageInfo), SdkError> {
        let proto_req: proto::GetPoolsByMarketRequest = req.into();
        let resp = self.query(svc_path!("GetPoolsByMarket"), proto_req.encode_to_vec()).await?;
        let p = proto::GetPoolsByMarketResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        let pools = p.pools.into_iter().map(Into::into).collect();
        let page = p.pagination.map_or(PageInfo { next_key: Vec::new(), total: 0 }, |pg| {
            PageInfo { next_key: pg.next_key, total: pg.total }
        });
        Ok((pools, page))
    }

    /// Gets depth metrics for a market.
    pub async fn get_depth_metrics(&self, market_index: u64) -> Result<DepthMetrics, SdkError> {
        let req = requests::GetDepthMetricsRequest::new(market_index);
        let proto_req: proto::GetDepthMetricsRequest = req.into();
        let resp = self.query(svc_path!("GetDepthMetrics"), proto_req.encode_to_vec()).await?;
        let p = proto::GetDepthMetricsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.metrics.map(Into::into).ok_or_else(|| SdkError::transport("metrics field missing"))
    }

    /// Gets pool health metrics by pool ID.
    pub async fn get_pool_health(&self, pool_id: &str) -> Result<PoolHealth, SdkError> {
        let req = requests::GetPoolHealthRequest::new(pool_id);
        let proto_req: proto::GetPoolHealthRequest = req.into();
        let resp = self.query(svc_path!("GetPoolHealth"), proto_req.encode_to_vec()).await?;
        let p = proto::GetPoolHealthResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.health.map(Into::into).ok_or_else(|| SdkError::transport("health field missing"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for LiquidityClient {
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
                "/liquidity.v1.LiquidityService/GetPool" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetPoolResponse {
                        success: true,
                        pool: Some(proto::Pool {
                            pool_id: "pool1".into(), market_index: 1,
                            asset: None, total_liquidity: "1000".into(),
                            available_liquidity: "800".into(), reserved_liquidity: "200".into(),
                            target_liquidity: "1200".into(),
                            r#type: 1, status: 1,
                            created_at: None, updated_at: None,
                            provider_type: 1, provider_config: Vec::new(),
                            depth_2pct_bid: "500".into(), depth_2pct_ask: "500".into(),
                            health_score_bps: 9500,
                            display_name: String::new(), description: String::new(),
                            tags: Vec::new(), logo_uri: String::new(),
                        }),
                    }))
                }
                "/liquidity.v1.LiquidityService/ListPools" => {
                    Ok(prost::Message::encode_to_vec(&proto::ListPoolsResponse {
                        success: true, pools: vec![], pagination: None,
                    }))
                }
                "/liquidity.v1.LiquidityService/GetDepthMetrics" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetDepthMetricsResponse {
                        success: true,
                        metrics: Some(proto::DepthMetrics {
                            market_index: 42, bid_depth: "5000".into(),
                            ask_depth: "4500".into(), spread: "100".into(),
                            timestamp: None,
                        }),
                    }))
                }
                "/liquidity.v1.LiquidityService/GetPoolHealth" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetPoolHealthResponse {
                        success: true,
                        health: Some(proto::PoolHealth {
                            pool_id: "pool1".into(), health_score: "9500".into(),
                            utilization_rate: "6500".into(), apy: "1200".into(),
                            timestamp: None,
                        }),
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> LiquidityClient {
        LiquidityClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_pool_works() {
        let pool = make_client().get_pool("pool1").await.unwrap();
        assert_eq!(pool.pool_id, "pool1");
        assert_eq!(pool.total_liquidity, "1000");
    }

    #[tokio::test]
    async fn list_pools_works() {
        let (pools, page) = make_client().list_pools(requests::ListPoolsRequest::new(0, 50)).await.unwrap();
        assert!(pools.is_empty());
        assert_eq!(page.total, 0);
    }

    #[tokio::test]
    async fn get_depth_metrics_works() {
        let dm = make_client().get_depth_metrics(42).await.unwrap();
        assert_eq!(dm.market_index, 42);
        assert_eq!(dm.bid_depth, "5000");
    }

    #[tokio::test]
    async fn get_pool_health_works() {
        let h = make_client().get_pool_health("pool1").await.unwrap();
        assert_eq!(h.health_score, "9500");
    }
}
