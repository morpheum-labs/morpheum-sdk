//! TreasuryClient — queries for reserves, metrics, categories, params, and allocation history.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::treasury::v1 as proto;

use crate::requests;
use crate::types::{
    AllocationRecord, CategoryReserve, ReservesState, TreasuryMetrics, TreasuryParams,
};

/// Category reserve query result with total reserves context.
pub struct CategoryReserveResult {
    pub category_reserve: CategoryReserve,
    pub total_reserves: u64,
}

/// Paginated allocation history result.
pub struct AllocationHistoryPage {
    pub records: Vec<AllocationRecord>,
    pub total_count: u64,
}

/// Primary client for treasury module queries.
pub struct TreasuryClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl TreasuryClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets the complete reserves state.
    pub async fn get_reserves_state(&self) -> Result<ReservesState, SdkError> {
        let proto_req: proto::QueryReservesStateRequest = requests::QueryReservesStateRequest.into();
        let resp = self.query("/treasury.v1.Query/QueryReservesState", proto_req.encode_to_vec()).await?;
        let p = proto::QueryReservesStateResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.state.map(Into::into).ok_or_else(|| SdkError::transport("state field missing"))
    }

    /// Gets real-time treasury metrics.
    pub async fn get_metrics(&self) -> Result<TreasuryMetrics, SdkError> {
        let proto_req: proto::QueryTreasuryMetricsRequest = requests::QueryTreasuryMetricsRequest.into();
        let resp = self.query("/treasury.v1.Query/QueryTreasuryMetrics", proto_req.encode_to_vec()).await?;
        let p = proto::QueryTreasuryMetricsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.metrics.map(Into::into).ok_or_else(|| SdkError::transport("metrics field missing"))
    }

    /// Gets a specific category reserve.
    pub async fn get_category_reserve(&self, req: requests::QueryCategoryReserveRequest) -> Result<CategoryReserveResult, SdkError> {
        let proto_req: proto::QueryCategoryReserveRequest = req.into();
        let resp = self.query("/treasury.v1.Query/QueryCategoryReserve", proto_req.encode_to_vec()).await?;
        let p = proto::QueryCategoryReserveResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(CategoryReserveResult {
            category_reserve: p.category_reserve.map(Into::into)
                .ok_or_else(|| SdkError::transport("category_reserve field missing"))?,
            total_reserves: p.total_reserves,
        })
    }

    /// Gets current governance parameters.
    pub async fn get_params(&self) -> Result<TreasuryParams, SdkError> {
        let proto_req: proto::QueryParamsRequest = requests::QueryParamsRequest.into();
        let resp = self.query("/treasury.v1.Query/QueryParams", proto_req.encode_to_vec()).await?;
        let p = proto::QueryParamsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.params.map(Into::into).ok_or_else(|| SdkError::transport("params field missing"))
    }

    /// Queries paginated allocation history.
    pub async fn get_allocation_history(&self, req: requests::QueryAllocationHistoryRequest) -> Result<AllocationHistoryPage, SdkError> {
        let proto_req: proto::QueryAllocationHistoryRequest = req.into();
        let resp = self.query("/treasury.v1.Query/QueryAllocationHistory", proto_req.encode_to_vec()).await?;
        let p = proto::QueryAllocationHistoryResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(AllocationHistoryPage {
            records: p.records.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        })
    }
}

#[async_trait(?Send)]
impl MorpheumClient for TreasuryClient {
    fn config(&self) -> &SdkConfig { &self.config }
    fn transport(&self) -> &dyn Transport { &*self.transport }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/treasury.v1.Query/QueryReservesState" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryReservesStateResponse {
                        state: Some(proto::ReservesState {
                            total_reserves: 1_000_000, categories: vec![],
                            merkle_root: vec![], last_sweep_timestamp: 100,
                            last_rebalance_timestamp: 90,
                        }),
                        block_height: 50, timestamp: 100,
                    }))
                }
                "/treasury.v1.Query/QueryTreasuryMetrics" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryTreasuryMetricsResponse {
                        metrics: Some(proto::TreasuryMetrics {
                            total_reserves: 1_000_000, insurance_protection_balance: 400_000,
                            buyback_burn_balance: 200_000, reserve_to_oi_ratio_bps: 1500,
                            insurance_coverage_ratio_bps: 2000, projected_runway_days: 365,
                            last_updated: None,
                        }),
                        block_height: 50,
                    }))
                }
                "/treasury.v1.Query/QueryCategoryReserve" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryCategoryReserveResponse {
                        category_reserve: Some(proto::CategoryReserve {
                            category: 1, balance: 400_000, allocation_bps: 4000,
                            last_updated: 100, metadata: vec![],
                        }),
                        total_reserves: 1_000_000,
                    }))
                }
                "/treasury.v1.Query/QueryParams" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryParamsResponse {
                        params: Some(proto::Params {
                            insurance_protection_bps: 4000, liquidity_incentives_bps: 2500,
                            buyback_burn_bps: 2000, operations_ecosystem_bps: 1000,
                            strategic_initiatives_bps: 300, emergency_stabilization_bps: 200,
                            min_insurance_coverage_bps: 1500, auto_rebalance_threshold_bps: 500,
                            max_single_allocation_bps: 2000, buyback_frequency_blocks: 100,
                            min_buyback_amount: 1_000_000,
                        }),
                    }))
                }
                "/treasury.v1.Query/QueryAllocationHistory" => {
                    Ok(prost::Message::encode_to_vec(&proto::QueryAllocationHistoryResponse {
                        records: vec![], pagination: None, total_count: 0,
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> TreasuryClient {
        TreasuryClient::new(
            SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"),
            Box::new(DummyTransport),
        )
    }

    #[tokio::test]
    async fn get_reserves_state_works() {
        let s = make_client().get_reserves_state().await.unwrap();
        assert_eq!(s.total_reserves, 1_000_000);
    }

    #[tokio::test]
    async fn get_metrics_works() {
        let m = make_client().get_metrics().await.unwrap();
        assert_eq!(m.projected_runway_days, 365);
    }

    #[tokio::test]
    async fn get_category_reserve_works() {
        let r = make_client()
            .get_category_reserve(requests::QueryCategoryReserveRequest::new(
                crate::types::ReserveCategory::InsuranceProtection,
            ))
            .await.unwrap();
        assert_eq!(r.category_reserve.balance, 400_000);
        assert_eq!(r.total_reserves, 1_000_000);
    }

    #[tokio::test]
    async fn get_params_works() {
        let p = make_client().get_params().await.unwrap();
        assert_eq!(p.insurance_protection_bps, 4000);
    }

    #[tokio::test]
    async fn get_allocation_history_works() {
        let page = make_client()
            .get_allocation_history(requests::QueryAllocationHistoryRequest::new())
            .await.unwrap();
        assert!(page.records.is_empty());
        assert_eq!(page.total_count, 0);
    }
}
