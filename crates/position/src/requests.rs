//! Request and response wrappers for the Position module.
//!
//! Position lifecycle:
//! - Positions are CREATED from CLOB order fills, not user-submitted OpenPosition txs.
//! - Position SIZE CHANGES come from subsequent fills.
//! - Users can close/reduce positions (ClosePosition) and change leverage (UpdatePositionLeverage).
//!
//! Transaction requests include `to_any()` methods for seamless integration with `TxBuilder`.
//! Query requests convert to proto via `From` impls.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::position::v1 as proto;

// ====================== TRANSACTION REQUESTS ======================

/// Request to close a position. Supports optional `bucket_id` for disambiguation
/// when an address has multiple buckets with positions in the same market.
/// Consolidates the old ClosePosition and CloseBucketPosition into one request.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClosePositionRequest {
    pub address: String,
    pub market_index: u64,
    pub exit_price: u64,
    pub bucket_id: Option<u64>,
}

impl ClosePositionRequest {
    pub fn new(
        address: impl Into<String>,
        market_index: u64,
        exit_price: u64,
    ) -> Self {
        Self {
            address: address.into(),
            market_index,
            exit_price,
            bucket_id: None,
        }
    }

    pub fn with_bucket_id(mut self, bucket_id: u64) -> Self {
        self.bucket_id = Some(bucket_id);
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgClosePosition = self.clone().into();
        ProtoAny {
            type_url: "/position.v1.Msg/ClosePosition".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ClosePositionRequest> for proto::MsgClosePosition {
    fn from(req: ClosePositionRequest) -> Self {
        Self {
            address: req.address,
            market_index: req.market_index,
            exit_price: req.exit_price,
            bucket_id: req.bucket_id.unwrap_or(0),
        }
    }
}

/// Request to update position leverage without changing position size.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdatePositionLeverageRequest {
    pub address: String,
    pub position_id: Option<String>,
    pub bucket_id: Option<String>,
    pub market_index: u64,
    pub new_leverage: String,
}

impl UpdatePositionLeverageRequest {
    pub fn new(
        address: impl Into<String>,
        market_index: u64,
        new_leverage: impl Into<String>,
    ) -> Self {
        Self {
            address: address.into(),
            position_id: None,
            bucket_id: None,
            market_index,
            new_leverage: new_leverage.into(),
        }
    }

    pub fn with_position_id(mut self, id: impl Into<String>) -> Self {
        self.position_id = Some(id.into());
        self
    }

    pub fn with_bucket_id(mut self, id: impl Into<String>) -> Self {
        self.bucket_id = Some(id.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdatePositionLeverage = self.clone().into();
        ProtoAny {
            type_url: "/position.v1.Msg/UpdatePositionLeverage".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdatePositionLeverageRequest> for proto::MsgUpdatePositionLeverage {
    fn from(req: UpdatePositionLeverageRequest) -> Self {
        Self {
            address: req.address,
            position_id: req.position_id.unwrap_or_default(),
            bucket_id: req.bucket_id.unwrap_or_default(),
            market_index: req.market_index,
            new_leverage: req.new_leverage,
            timestamp: None,
        }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query a single position by address and market index.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetPositionRequest {
    pub address: String,
    pub market_index: u64,
}

impl GetPositionRequest {
    pub fn new(address: impl Into<String>, market_index: u64) -> Self {
        Self {
            address: address.into(),
            market_index,
        }
    }
}

impl From<GetPositionRequest> for proto::GetPositionRequest {
    fn from(req: GetPositionRequest) -> Self {
        Self {
            address: req.address,
            market_index: req.market_index,
        }
    }
}

/// Query all open positions for an address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListOpenPositionsRequest {
    pub address: String,
}

impl ListOpenPositionsRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }
}

impl From<ListOpenPositionsRequest> for proto::ListOpenPositionsRequest {
    fn from(req: ListOpenPositionsRequest) -> Self {
        Self {
            address: req.address,
        }
    }
}

/// Query aggregated long/short volume for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetLongShortVolumeRequest {
    pub market_index: u64,
}

impl GetLongShortVolumeRequest {
    pub fn new(market_index: u64) -> Self {
        Self { market_index }
    }
}

impl From<GetLongShortVolumeRequest> for proto::GetLongShortVolumeRequest {
    fn from(req: GetLongShortVolumeRequest) -> Self {
        Self {
            market_index: req.market_index,
        }
    }
}

/// Query all positions owned by an address across all buckets.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPositionsByAddressRequest {
    pub address: String,
    pub active_only: bool,
}

impl QueryPositionsByAddressRequest {
    pub fn new(address: impl Into<String>) -> Self {
        Self { address: address.into(), active_only: false }
    }

    pub fn active_only(mut self, active: bool) -> Self {
        self.active_only = active;
        self
    }
}

impl From<QueryPositionsByAddressRequest> for proto::QueryPositionsByAddressRequest {
    fn from(req: QueryPositionsByAddressRequest) -> Self {
        Self { address: req.address, active_only: req.active_only }
    }
}

/// Query all positions in a specific market across all addresses.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryAllPositionsByMarketRequest {
    pub market_index: u64,
    pub symbol: Option<String>,
    pub active_only: bool,
    pub side: Option<String>,
    pub limit: i32,
    pub offset: i32,
}

impl QueryAllPositionsByMarketRequest {
    pub fn new(market_index: u64) -> Self {
        Self {
            market_index,
            symbol: None,
            active_only: false,
            side: None,
            limit: 50,
            offset: 0,
        }
    }

    pub fn symbol(mut self, sym: impl Into<String>) -> Self {
        self.symbol = Some(sym.into());
        self
    }

    pub fn active_only(mut self, active: bool) -> Self {
        self.active_only = active;
        self
    }

    pub fn side(mut self, side: impl Into<String>) -> Self {
        self.side = Some(side.into());
        self
    }

    pub fn paginate(mut self, limit: i32, offset: i32) -> Self {
        self.limit = limit;
        self.offset = offset;
        self
    }
}

impl From<QueryAllPositionsByMarketRequest> for proto::QueryAllPositionsByMarketRequest {
    fn from(req: QueryAllPositionsByMarketRequest) -> Self {
        Self {
            market_index: req.market_index,
            symbol: req.symbol.unwrap_or_default(),
            active_only: req.active_only,
            side: req.side.unwrap_or_default(),
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query PnL for a specific position.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPositionPnLRequest {
    pub address: String,
    pub market_index: u64,
}

impl QueryPositionPnLRequest {
    pub fn new(address: impl Into<String>, market_index: u64) -> Self {
        Self { address: address.into(), market_index }
    }
}

impl From<QueryPositionPnLRequest> for proto::QueryPositionPnLRequest {
    fn from(req: QueryPositionPnLRequest) -> Self {
        Self { address: req.address, market_index: req.market_index }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn close_position_request_to_any() {
        let req = ClosePositionRequest::new("morpheum1abc", 42, 52000);
        let any = req.to_any();
        assert_eq!(any.type_url, "/position.v1.Msg/ClosePosition");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn close_position_with_bucket_id() {
        let req = ClosePositionRequest::new("morpheum1abc", 42, 52000)
            .with_bucket_id(123);
        assert_eq!(req.bucket_id, Some(123));
        let any = req.to_any();
        assert!(!any.value.is_empty());
    }

    #[test]
    fn update_leverage_request_to_any() {
        let req = UpdatePositionLeverageRequest::new("morpheum1abc", 42, "20")
            .with_bucket_id("bucket-1");
        let any = req.to_any();
        assert_eq!(any.type_url, "/position.v1.Msg/UpdatePositionLeverage");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn get_position_request_proto_conversion() {
        let req = GetPositionRequest::new("morpheum1abc", 42);
        let proto_req: proto::GetPositionRequest = req.into();
        assert_eq!(proto_req.address, "morpheum1abc");
        assert_eq!(proto_req.market_index, 42);
    }

    #[test]
    fn list_open_positions_request_proto_conversion() {
        let req = ListOpenPositionsRequest::new("morpheum1abc");
        let proto_req: proto::ListOpenPositionsRequest = req.into();
        assert_eq!(proto_req.address, "morpheum1abc");
    }

    #[test]
    fn get_long_short_volume_request_proto_conversion() {
        let req = GetLongShortVolumeRequest::new(42);
        let proto_req: proto::GetLongShortVolumeRequest = req.into();
        assert_eq!(proto_req.market_index, 42);
    }

    #[test]
    fn query_positions_by_address_request_proto_conversion() {
        let req = QueryPositionsByAddressRequest::new("morpheum1abc").active_only(true);
        let proto_req: proto::QueryPositionsByAddressRequest = req.into();
        assert_eq!(proto_req.address, "morpheum1abc");
        assert!(proto_req.active_only);
    }

    #[test]
    fn query_all_positions_by_market_request_proto_conversion() {
        let req = QueryAllPositionsByMarketRequest::new(42)
            .active_only(true)
            .side("LONG")
            .paginate(100, 50);
        let proto_req: proto::QueryAllPositionsByMarketRequest = req.into();
        assert_eq!(proto_req.market_index, 42);
        assert!(proto_req.active_only);
        assert_eq!(proto_req.side, "LONG");
        assert_eq!(proto_req.limit, 100);
        assert_eq!(proto_req.offset, 50);
    }

    #[test]
    fn query_position_pnl_request_proto_conversion() {
        let req = QueryPositionPnLRequest::new("morpheum1abc", 42);
        let proto_req: proto::QueryPositionPnLRequest = req.into();
        assert_eq!(proto_req.address, "morpheum1abc");
        assert_eq!(proto_req.market_index, 42);
    }
}
