//! Request and response wrappers for the Position module.
//!
//! These provide clean, type-safe Rust APIs around the raw protobuf messages.
//! Transaction requests include `to_any()` methods for seamless integration
//! with `TxBuilder`. Query requests convert to proto via `From` impls.

use alloc::string::String;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::position::v1 as proto;

use crate::types::PositionSide;

// ====================== TRANSACTION REQUESTS ======================

/// Request to open a new position.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OpenPositionRequest {
    pub address: String,
    pub market_index: u64,
    pub size: u64,
    pub entry_price: u64,
    pub side: PositionSide,
    pub leverage: u32,
    pub power: u32,
}

impl OpenPositionRequest {
    pub fn new(
        address: impl Into<String>,
        market_index: u64,
        size: u64,
        entry_price: u64,
        side: PositionSide,
        leverage: u32,
    ) -> Self {
        Self {
            address: address.into(),
            market_index,
            size,
            entry_price,
            side,
            leverage,
            power: 1,
        }
    }

    pub fn with_power(mut self, power: u32) -> Self {
        self.power = power;
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgOpenPosition = self.clone().into();
        ProtoAny {
            type_url: "/position.v1.MsgOpenPosition".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<OpenPositionRequest> for proto::MsgOpenPosition {
    fn from(req: OpenPositionRequest) -> Self {
        Self {
            address: req.address,
            market_index: req.market_index,
            size: req.size,
            entry_price: req.entry_price,
            side: i32::from(req.side),
            leverage: req.leverage,
            power: req.power,
        }
    }
}

/// Request to update an existing position (add to or reduce size).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdatePositionRequest {
    pub address: String,
    pub market_index: u64,
    pub size_delta: i64,
    pub price: u64,
}

impl UpdatePositionRequest {
    pub fn new(
        address: impl Into<String>,
        market_index: u64,
        size_delta: i64,
        price: u64,
    ) -> Self {
        Self {
            address: address.into(),
            market_index,
            size_delta,
            price,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdatePosition = self.clone().into();
        ProtoAny {
            type_url: "/position.v1.MsgUpdatePosition".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdatePositionRequest> for proto::MsgUpdatePosition {
    fn from(req: UpdatePositionRequest) -> Self {
        Self {
            address: req.address,
            market_index: req.market_index,
            size_delta: req.size_delta,
            price: req.price,
        }
    }
}

/// Request to close a position entirely.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClosePositionRequest {
    pub address: String,
    pub market_index: u64,
    pub exit_price: u64,
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
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgClosePosition = self.clone().into();
        ProtoAny {
            type_url: "/position.v1.MsgClosePosition".into(),
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
        }
    }
}

/// Request to close a position within a specific bucket.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CloseBucketPositionRequest {
    pub address: String,
    pub bucket_id: u64,
    pub market_index: u64,
    pub exit_price: u64,
}

impl CloseBucketPositionRequest {
    pub fn new(
        address: impl Into<String>,
        bucket_id: u64,
        market_index: u64,
        exit_price: u64,
    ) -> Self {
        Self {
            address: address.into(),
            bucket_id,
            market_index,
            exit_price,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCloseBucketPosition = self.clone().into();
        ProtoAny {
            type_url: "/position.v1.MsgCloseBucketPosition".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CloseBucketPositionRequest> for proto::MsgCloseBucketPosition {
    fn from(req: CloseBucketPositionRequest) -> Self {
        Self {
            address: req.address,
            bucket_id: req.bucket_id,
            market_index: req.market_index,
            exit_price: req.exit_price,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_position_request_to_any() {
        let req = OpenPositionRequest::new(
            "morpheum1abc",
            42,
            1000,
            50000,
            PositionSide::Long,
            10,
        )
        .with_power(2);

        let any = req.to_any();
        assert_eq!(any.type_url, "/position.v1.MsgOpenPosition");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn update_position_request_to_any() {
        let req = UpdatePositionRequest::new("morpheum1abc", 42, -500, 51000);
        let any = req.to_any();
        assert_eq!(any.type_url, "/position.v1.MsgUpdatePosition");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn close_position_request_to_any() {
        let req = ClosePositionRequest::new("morpheum1abc", 42, 52000);
        let any = req.to_any();
        assert_eq!(any.type_url, "/position.v1.MsgClosePosition");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn close_bucket_position_request_to_any() {
        let req = CloseBucketPositionRequest::new("morpheum1abc", 1, 42, 52000);
        let any = req.to_any();
        assert_eq!(any.type_url, "/position.v1.MsgCloseBucketPosition");
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
}
