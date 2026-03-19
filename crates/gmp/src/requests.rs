//! GMP SDK request types.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use morpheum_proto::gmp::v1 as pb;
use prost::Message;

/// Request to initiate a warp route transfer (Morpheum -> EVM).
#[derive(Clone, Debug)]
pub struct WarpRouteTransferRequest {
    pub sender: String,
    pub destination_domain: u32,
    pub recipient: Vec<u8>,
    pub asset_index: u64,
    pub amount: String,
}

impl WarpRouteTransferRequest {
    pub fn to_any(&self) -> morpheum_proto::google::protobuf::Any {
        let msg = pb::MsgWarpRouteTransfer {
            sender: self.sender.clone(),
            destination_domain: self.destination_domain,
            recipient: self.recipient.clone(),
            asset_index: self.asset_index,
            amount: self.amount.clone(),
        };
        morpheum_proto::google::protobuf::Any {
            type_url: "/gmp.v1.MsgWarpRouteTransfer".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Request to process a Hyperlane message (relayer entry point).
#[derive(Clone, Debug)]
pub struct ProcessHyperlaneMessageRequest {
    pub metadata: Vec<u8>,
    pub message: Vec<u8>,
}

impl ProcessHyperlaneMessageRequest {
    pub fn to_any(&self) -> morpheum_proto::google::protobuf::Any {
        let msg = pb::MsgProcessHyperlaneMessage {
            metadata: self.metadata.clone(),
            message: self.message.clone(),
        };
        morpheum_proto::google::protobuf::Any {
            type_url: "/gmp.v1.MsgProcessHyperlaneMessage".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Request to settle a generic GMP payment.
#[derive(Clone, Debug)]
pub struct SettleGmpPaymentRequest {
    pub protocol_id: String,
    pub raw_envelope: Vec<u8>,
}

impl SettleGmpPaymentRequest {
    pub fn to_any(&self) -> morpheum_proto::google::protobuf::Any {
        let msg = pb::MsgSettleGmpPayment {
            protocol_id: self.protocol_id.clone(),
            raw_envelope: self.raw_envelope.clone(),
        };
        morpheum_proto::google::protobuf::Any {
            type_url: "/gmp.v1.MsgSettleGmpPayment".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Request to update GMP module parameters via governance.
#[derive(Clone, Debug)]
pub struct UpdateGmpParamsRequest {
    pub authority: String,
    pub params: crate::types::GmpParams,
}

impl UpdateGmpParamsRequest {
    pub fn to_any(&self) -> morpheum_proto::google::protobuf::Any {
        let msg = pb::MsgUpdateParams {
            authority: self.authority.clone(),
            params: Some(self.params.clone().into()),
        };
        morpheum_proto::google::protobuf::Any {
            type_url: "/gmp.v1.MsgUpdateParams".into(),
            value: msg.encode_to_vec(),
        }
    }
}

/// Query request for Hyperlane delivery status.
#[derive(Clone, Debug)]
pub struct QueryHyperlaneDeliveryRequest {
    pub message_id: String,
}

/// Query request for Hyperlane nonce.
#[derive(Clone, Debug)]
pub struct QueryHyperlaneNonceRequest;

/// Query request for GMP params.
#[derive(Clone, Debug)]
pub struct QueryGmpParamsRequest;
