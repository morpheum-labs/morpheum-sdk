//! BucketClient — the main entry point for all bucket-related operations
//! in the Morpheum SDK.
//!
//! Provides high-level, type-safe methods for querying buckets, positions,
//! PnL, health, liquidation history, and balance information. Transaction
//! operations (create bucket, close position, transfer, etc.) are handled
//! via the fluent builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

use crate::requests;
use crate::types::{
    AddressPnL, AllBucketsBalance, Bucket, BucketPnL, BucketStatus,
    LiquidationEvent, LiquidationMetrics, Position, PositionHealth, PositionPnL,
};

use morpheum_proto::bucket::v1 as proto;

/// Primary client for all bucket-related queries.
pub struct BucketClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl BucketClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Gets a specific bucket by ID.
    pub async fn get_bucket(
        &self,
        bucket_id: impl Into<String>,
    ) -> Result<Bucket, SdkError> {
        let req = requests::QueryBucketRequest::new(bucket_id);
        let proto_req: proto::QueryBucketRequest = req.into();
        let resp = self.query("/bucket.v1.Query/GetBucket", proto_req.encode_to_vec()).await?;
        let proto_res = proto::QueryBucketResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(proto_res.success, &proto_res.message)?;
        proto_res.bucket.map(Into::into).ok_or_else(|| SdkError::transport("bucket field missing"))
    }

    /// Gets all buckets for an address, with optional type filter.
    pub async fn get_buckets_by_address(
        &self,
        address: impl Into<String>,
        type_filter: Option<crate::types::BucketType>,
    ) -> Result<Vec<Bucket>, SdkError> {
        let mut req = requests::QueryBucketsByAddressRequest::new(address);
        if let Some(tf) = type_filter {
            req = req.type_filter(tf);
        }
        let proto_req: proto::QueryBucketsByAddressRequest = req.into();
        let resp = self.query("/bucket.v1.Query/GetBucketsByAddress", proto_req.encode_to_vec()).await?;
        let proto_res = proto::QueryBucketsByAddressResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(proto_res.success, &proto_res.message)?;
        Ok(proto_res.buckets.into_iter().map(Into::into).collect())
    }

    /// Gets aggregated PnL for all buckets of an address.
    pub async fn get_address_pnl(
        &self,
        address: impl Into<String>,
    ) -> Result<AddressPnL, SdkError> {
        let req = requests::QueryAddressPnLRequest::new(address);
        let proto_req: proto::QueryAddressPnLRequest = req.into();
        let resp = self.query("/bucket.v1.Query/GetAddressPnL", proto_req.encode_to_vec()).await?;
        let p = proto::QueryAddressPnLResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(AddressPnL {
            address: p.address,
            unrealized_profit: p.unrealized_profit,
            unrealized_loss: p.unrealized_loss,
            realized_profit: p.realized_profit,
            realized_loss: p.realized_loss,
            net_profit: p.net_profit,
            net_loss: p.net_loss,
            buckets: p.buckets.into_iter().map(Into::into).collect(),
            calculated_at: p.calculated_at.map(|t| t.seconds as u64).unwrap_or(0),
        })
    }

    /// Gets PnL for a specific bucket with position-level breakdown.
    pub async fn query_bucket_pnl(
        &self,
        bucket_id: impl Into<String>,
    ) -> Result<BucketPnL, SdkError> {
        let req = requests::QueryBucketPnLRequest::new(bucket_id);
        let proto_req: proto::QueryBucketPnLRequest = req.into();
        let resp = self.query("/bucket.v1.Query/QueryBucketPnL", proto_req.encode_to_vec()).await?;
        let p = proto::QueryBucketPnLResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(BucketPnL {
            bucket_id: p.bucket_id,
            address: p.address,
            unrealized_profit: p.unrealized_profit,
            unrealized_loss: p.unrealized_loss,
            realized_profit: p.realized_profit,
            realized_loss: p.realized_loss,
            net_profit: p.net_profit,
            net_loss: p.net_loss,
            volatility_factor: p.volatility_factor,
            adjusted_profit: p.adjusted_profit,
            adjusted_loss: p.adjusted_loss,
            position_pnl_infos: p.position_pnl_infos.into_iter().map(Into::into).collect(),
            calculated_at: p.calculated_at.map(|t| t.seconds as u64).unwrap_or(0),
        })
    }

    /// Gets all positions for an address.
    pub async fn query_positions_by_address(
        &self,
        address: impl Into<String>,
        active_only: bool,
    ) -> Result<Vec<Position>, SdkError> {
        let req = requests::QueryPositionsByAddressRequest::new(address).active_only(active_only);
        let proto_req: proto::QueryPositionsByAddressRequest = req.into();
        let resp = self.query("/bucket.v1.Query/QueryPositionsByAddress", proto_req.encode_to_vec()).await?;
        let p = proto::QueryPositionsByAddressResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(p.positions.into_iter().map(Into::into).collect())
    }

    /// Gets all positions in a specific bucket.
    pub async fn query_positions_by_bucket(
        &self,
        bucket_id: impl Into<String>,
    ) -> Result<Vec<Position>, SdkError> {
        let req = requests::QueryPositionsByBucketRequest::new(bucket_id);
        let proto_req: proto::QueryPositionsByBucketRequest = req.into();
        let resp = self.query("/bucket.v1.Query/QueryPositionsByBucket", proto_req.encode_to_vec()).await?;
        let p = proto::QueryPositionsByBucketResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.message)?;
        Ok(p.positions.into_iter().map(Into::into).collect())
    }

    /// Gets PnL for a specific position.
    pub async fn query_position_pnl(
        &self,
        address: impl Into<String>,
        market_index: u64,
    ) -> Result<PositionPnL, SdkError> {
        let req = requests::QueryPositionPnLRequest::new(address, market_index);
        let proto_req: proto::QueryPositionPnLRequest = req.into();
        let resp = self.query("/bucket.v1.Query/QueryPositionPnL", proto_req.encode_to_vec()).await?;
        let p = proto::QueryPositionPnLResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(PositionPnL {
            unrealized_profit: p.unrealized_profit,
            unrealized_loss: p.unrealized_loss,
            realized_profit: p.realized_profit,
            realized_loss: p.realized_loss,
            net_profit: p.net_profit,
            net_loss: p.net_loss,
        })
    }

    /// Queries liquidation events with optional filters.
    pub async fn query_liquidations(
        &self,
        request: requests::QueryLiquidationsRequest,
    ) -> Result<Vec<LiquidationEvent>, SdkError> {
        let proto_req: proto::QueryLiquidationsRequest = request.into();
        let resp = self.query("/bucket.v1.Query/QueryLiquidations", proto_req.encode_to_vec()).await?;
        let p = proto::QueryLiquidationsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(p.liquidations.into_iter().map(Into::into).collect())
    }

    /// Gets all positions for a market across all addresses.
    pub async fn query_all_positions_by_market(
        &self,
        request: requests::QueryAllPositionsByMarketRequest,
    ) -> Result<Vec<Position>, SdkError> {
        let proto_req: proto::QueryAllPositionsByMarketRequest = request.into();
        let resp = self.query("/bucket.v1.Query/QueryAllPositionsByMarket", proto_req.encode_to_vec()).await?;
        let p = proto::QueryAllPositionsByMarketResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(p.positions.into_iter().map(Into::into).collect())
    }

    /// Checks a bucket's status.
    pub async fn query_bucket_status(
        &self,
        address: impl Into<String>,
        bucket_id: impl Into<String>,
    ) -> Result<BucketStatus, SdkError> {
        let req = requests::QueryBucketStatusRequest::new(address, bucket_id);
        let proto_req: proto::QueryBucketStatusRequest = req.into();
        let resp = self.query("/bucket.v1.Query/QueryBucketStatus", proto_req.encode_to_vec()).await?;
        let p = proto::QueryBucketStatusResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.message)?;
        Ok(BucketStatus {
            exists: p.exists,
            available_balance: p.available_balance,
            risk_factor: p.risk_factor,
        })
    }

    /// Gets balance summary for all buckets owned by an address.
    pub async fn query_all_buckets_balance(
        &self,
        address: impl Into<String>,
    ) -> Result<AllBucketsBalance, SdkError> {
        let req = requests::QueryAllBucketsBalanceByAddressRequest::new(address);
        let proto_req: proto::QueryAllBucketsBalanceByAddressRequest = req.into();
        let resp = self.query("/bucket.v1.Query/QueryAllBucketsBalanceByAddress", proto_req.encode_to_vec()).await?;
        let p = proto::QueryAllBucketsBalanceByAddressResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(AllBucketsBalance {
            balance: p.balance,
            bucket_balances: p.bucket_balances.into_iter().map(Into::into).collect(),
            total_balance: p.total_balance,
        })
    }

    /// Checks a position's health (margin ratio, liquidation status).
    pub async fn query_position_health(
        &self,
        address: impl Into<String>,
        market_index: u64,
        current_price: impl Into<String>,
    ) -> Result<PositionHealth, SdkError> {
        let req = requests::QueryPositionHealthRequest::new(address, market_index, current_price);
        let proto_req: proto::QueryPositionHealthRequest = req.into();
        let resp = self.query("/bucket.v1.Query/QueryPositionHealth", proto_req.encode_to_vec()).await?;
        let p = proto::QueryPositionHealthResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(PositionHealth {
            liquidation_required: p.liquidation_required,
            margin_ratio: p.margin_ratio,
            unrealized_profit: p.unrealized_profit,
            unrealized_loss: p.unrealized_loss,
        })
    }

    /// Gets liquidation metrics over a time range.
    pub async fn query_liquidation_metrics(
        &self,
        start_time: i64,
        end_time: i64,
    ) -> Result<LiquidationMetrics, SdkError> {
        let req = requests::QueryLiquidationMetricsRequest::new(start_time, end_time);
        let proto_req: proto::QueryLiquidationMetricsRequest = req.into();
        let resp = self.query("/bucket.v1.Query/QueryLiquidationMetrics", proto_req.encode_to_vec()).await?;
        let p = proto::QueryLiquidationMetricsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.message)?;
        Ok(LiquidationMetrics {
            total_liquidations: p.total_liquidations,
            total_warnings: p.total_warnings,
            avg_processing_time_ms: p.avg_processing_time_ms,
            max_processing_time_ms: p.max_processing_time_ms,
            liquidations_by_market: p.liquidations_by_market.into_iter().collect(),
            liquidations_by_algorithm: p.liquidations_by_algorithm.into_iter().collect(),
        })
    }

    /// Gets ADL execution history.
    pub async fn get_adl_history(
        &self,
        request: requests::QueryAdlHistoryRequest,
    ) -> Result<i32, SdkError> {
        let proto_req: proto::QueryAdlHistoryRequest = request.into();
        let resp = self.query("/bucket.v1.Query/GetADLHistory", proto_req.encode_to_vec()).await?;
        let p = proto::QueryAdlHistoryResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(p.total_count)
    }
}

/// Checks the `success` field common to bucket query responses.
fn check_success(success: bool, error_message: &str) -> Result<(), SdkError> {
    if success {
        Ok(())
    } else {
        Err(SdkError::transport(if error_message.is_empty() {
            "bucket query failed"
        } else {
            error_message
        }))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for BucketClient {
    fn config(&self) -> &SdkConfig {
        &self.config
    }

    fn transport(&self) -> &dyn Transport {
        &*self.transport
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(
            &self, _tx_bytes: Vec<u8>,
        ) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/bucket.v1.Query/GetBucket" => {
                    let r = proto::QueryBucketResponse {
                        success: true,
                        message: String::new(),
                        bucket: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&r))
                }
                "/bucket.v1.Query/GetBucketsByAddress" => {
                    let r = proto::QueryBucketsByAddressResponse {
                        success: true,
                        message: String::new(),
                        buckets: vec![],
                        total_buckets: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&r))
                }
                "/bucket.v1.Query/GetAddressPnL" => {
                    let r = proto::QueryAddressPnLResponse {
                        success: true,
                        ..Default::default()
                    };
                    Ok(prost::Message::encode_to_vec(&r))
                }
                "/bucket.v1.Query/QueryBucketPnL" => {
                    let r = proto::QueryBucketPnLResponse {
                        success: true,
                        ..Default::default()
                    };
                    Ok(prost::Message::encode_to_vec(&r))
                }
                "/bucket.v1.Query/QueryPositionsByAddress" => {
                    let r = proto::QueryPositionsByAddressResponse {
                        success: true,
                        ..Default::default()
                    };
                    Ok(prost::Message::encode_to_vec(&r))
                }
                "/bucket.v1.Query/QueryPositionPnL" => {
                    let r = proto::QueryPositionPnLResponse {
                        success: true,
                        ..Default::default()
                    };
                    Ok(prost::Message::encode_to_vec(&r))
                }
                "/bucket.v1.Query/QueryBucketStatus" => {
                    let r = proto::QueryBucketStatusResponse {
                        success: true,
                        message: String::new(),
                        exists: true,
                        available_balance: "50000000000".into(),
                        risk_factor: "100".into(),
                    };
                    Ok(prost::Message::encode_to_vec(&r))
                }
                "/bucket.v1.Query/QueryPositionHealth" => {
                    let r = proto::QueryPositionHealthResponse {
                        success: true,
                        ..Default::default()
                    };
                    Ok(prost::Message::encode_to_vec(&r))
                }
                "/bucket.v1.Query/QueryLiquidationMetrics" => {
                    let r = proto::QueryLiquidationMetricsResponse {
                        success: true,
                        ..Default::default()
                    };
                    Ok(prost::Message::encode_to_vec(&r))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    fn make_client() -> BucketClient {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        BucketClient::new(config, Box::new(DummyTransport))
    }

    #[tokio::test]
    async fn get_bucket_works() {
        let client = make_client();
        let result = client.get_bucket("bucket-1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_buckets_by_address_works() {
        let client = make_client();
        let result = client.get_buckets_by_address("morpheum1abc", None).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn get_address_pnl_works() {
        let client = make_client();
        let result = client.get_address_pnl("morpheum1abc").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_bucket_pnl_works() {
        let client = make_client();
        let result = client.query_bucket_pnl("bucket-1").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_positions_by_address_works() {
        let client = make_client();
        let result = client.query_positions_by_address("morpheum1abc", true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_position_pnl_works() {
        let client = make_client();
        let result = client.query_position_pnl("morpheum1abc", 42).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_bucket_status_works() {
        let client = make_client();
        let result = client.query_bucket_status("morpheum1abc", "bucket-1").await;
        assert!(result.is_ok());
        let status = result.unwrap();
        assert!(status.exists);
        assert_eq!(status.available_balance, "50000000000");
    }

    #[tokio::test]
    async fn query_position_health_works() {
        let client = make_client();
        let result = client.query_position_health("morpheum1abc", 42, "50000").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn query_liquidation_metrics_works() {
        let client = make_client();
        let result = client.query_liquidation_metrics(0, 1_700_000_000).await;
        assert!(result.is_ok());
    }
}
