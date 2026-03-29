//! PositionClient — the main entry point for all position-related operations
//! in the Morpheum SDK.
//!
//! This client provides high-level, type-safe methods for querying positions,
//! open positions, long/short volume, positions by address, positions by market,
//! and position PnL. Transaction operations (close, update leverage) are handled
//! via the fluent builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

use crate::{
    requests::{
        GetLongShortVolumeRequest, GetPositionRequest, ListOpenPositionsRequest,
        QueryAllPositionsByMarketRequest, QueryPositionPnLRequest,
        QueryPositionsByAddressRequest,
    },
    types::{LongShortVolume, Position, PositionPnL, PositionState},
};

/// Primary client for all position-related queries.
///
/// Transaction construction (open, update, close, close-bucket) is delegated
/// to the fluent builders in `builder.rs`.
pub struct PositionClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl PositionClient {
    /// Creates a new `PositionClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a single position by address and market index.
    ///
    /// Returns `None` if the position is not found.
    pub async fn get_position(
        &self,
        address: impl Into<String>,
        market_index: u64,
    ) -> Result<Option<PositionState>, SdkError> {
        let req = GetPositionRequest::new(address, market_index);
        let proto_req: morpheum_proto::position::v1::GetPositionRequest = req.into();

        let path = "/position.v1.Query/GetPosition";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::position::v1::GetPositionResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        if proto_res.found {
            Ok(proto_res.position.map(Into::into))
        } else {
            Ok(None)
        }
    }

    /// Queries all open positions for the given address.
    pub async fn list_open_positions(
        &self,
        address: impl Into<String>,
    ) -> Result<Vec<PositionState>, SdkError> {
        let req = ListOpenPositionsRequest::new(address);
        let proto_req: morpheum_proto::position::v1::ListOpenPositionsRequest = req.into();

        let path = "/position.v1.Query/ListOpenPositions";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::position::v1::ListOpenPositionsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.positions.into_iter().map(Into::into).collect())
    }

    /// Queries aggregated long/short volume for a market.
    pub async fn get_long_short_volume(
        &self,
        market_index: u64,
    ) -> Result<LongShortVolume, SdkError> {
        let req = GetLongShortVolumeRequest::new(market_index);
        let proto_req: morpheum_proto::position::v1::GetLongShortVolumeRequest = req.into();

        let path = "/position.v1.Query/GetLongShortVolume";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::position::v1::GetLongShortVolumeResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(LongShortVolume {
            long_volume: proto_res.long_volume,
            short_volume: proto_res.short_volume,
        })
    }

    /// Queries all positions for an address across all buckets.
    pub async fn query_positions_by_address(
        &self,
        address: impl Into<String>,
        active_only: bool,
    ) -> Result<Vec<Position>, SdkError> {
        use morpheum_proto::position::v1 as proto;
        let req = QueryPositionsByAddressRequest::new(address).active_only(active_only);
        let proto_req: proto::QueryPositionsByAddressRequest = req.into();
        let resp = self.query("/position.v1.Query/QueryPositionsByAddress", proto_req.encode_to_vec()).await?;
        let p = proto::QueryPositionsByAddressResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(p.positions.into_iter().map(Into::into).collect())
    }

    /// Queries all positions in a specific market across all addresses.
    pub async fn query_all_positions_by_market(
        &self,
        request: QueryAllPositionsByMarketRequest,
    ) -> Result<Vec<Position>, SdkError> {
        use morpheum_proto::position::v1 as proto;
        let proto_req: proto::QueryAllPositionsByMarketRequest = request.into();
        let resp = self.query("/position.v1.Query/QueryAllPositionsByMarket", proto_req.encode_to_vec()).await?;
        let p = proto::QueryAllPositionsByMarketResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        check_success(p.success, &p.error_message)?;
        Ok(p.positions.into_iter().map(Into::into).collect())
    }

    /// Queries PnL for a specific position.
    pub async fn query_position_pnl(
        &self,
        address: impl Into<String>,
        market_index: u64,
    ) -> Result<PositionPnL, SdkError> {
        use morpheum_proto::position::v1 as proto;
        let req = QueryPositionPnLRequest::new(address, market_index);
        let proto_req: proto::QueryPositionPnLRequest = req.into();
        let resp = self.query("/position.v1.Query/QueryPositionPnL", proto_req.encode_to_vec()).await?;
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
}

fn check_success(success: bool, error_message: &str) -> Result<(), SdkError> {
    if success {
        Ok(())
    } else {
        Err(SdkError::transport(if error_message.is_empty() {
            "position query failed"
        } else {
            error_message
        }))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for PositionClient {
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
    use morpheum_sdk_core::SdkConfig;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(
            &self,
            _tx_bytes: Vec<u8>,
        ) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!("not needed for position query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/position.v1.Query/GetPosition" => {
                    let dummy = morpheum_proto::position::v1::GetPositionResponse {
                        found: true,
                        position: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/position.v1.Query/ListOpenPositions" => {
                    let dummy = morpheum_proto::position::v1::ListOpenPositionsResponse {
                        positions: vec![],
                        total_count: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/position.v1.Query/GetLongShortVolume" => {
                    let dummy = morpheum_proto::position::v1::GetLongShortVolumeResponse {
                        long_volume: 1000,
                        short_volume: 800,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/position.v1.Query/QueryPositionsByAddress" => {
                    let dummy = morpheum_proto::position::v1::QueryPositionsByAddressResponse {
                        success: true,
                        ..Default::default()
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/position.v1.Query/QueryAllPositionsByMarket" => {
                    let dummy = morpheum_proto::position::v1::QueryAllPositionsByMarketResponse {
                        success: true,
                        ..Default::default()
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/position.v1.Query/QueryPositionPnL" => {
                    let dummy = morpheum_proto::position::v1::QueryPositionPnLResponse {
                        success: true,
                        ..Default::default()
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn position_client_get_position_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = PositionClient::new(config, Box::new(DummyTransport));

        let result = client.get_position("morpheum1abc", 42).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn position_client_list_open_positions_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = PositionClient::new(config, Box::new(DummyTransport));

        let result = client.list_open_positions("morpheum1abc").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn position_client_get_long_short_volume_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = PositionClient::new(config, Box::new(DummyTransport));

        let result = client.get_long_short_volume(42).await;
        assert!(result.is_ok());
        let volume = result.unwrap();
        assert_eq!(volume.long_volume, 1000);
        assert_eq!(volume.short_volume, 800);
    }

    #[tokio::test]
    async fn position_client_query_positions_by_address_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = PositionClient::new(config, Box::new(DummyTransport));

        let result = client.query_positions_by_address("morpheum1abc", true).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn position_client_query_all_positions_by_market_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = PositionClient::new(config, Box::new(DummyTransport));

        let req = crate::requests::QueryAllPositionsByMarketRequest::new(42);
        let result = client.query_all_positions_by_market(req).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[tokio::test]
    async fn position_client_query_position_pnl_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = PositionClient::new(config, Box::new(DummyTransport));

        let result = client.query_position_pnl("morpheum1abc", 42).await;
        assert!(result.is_ok());
    }
}
