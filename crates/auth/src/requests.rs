//! Request and response wrappers for the Auth module.
//!
//! These are clean, ergonomic Rust types that wrap the raw protobuf messages.
//! They provide type safety (using `AccountId`), validation, helper methods,
//! and seamless conversion to/from protobuf for use with `TxBuilder`.

use alloc::{string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use prost_types::Any as ProtoAny;

use morpheum_sdk_core::{AccountId, SdkError};
use morpheum_sdk_proto::auth::v1 as proto;

/// Request to approve a Trading Key (delegated session key) for an agent.
///
/// This is the primary mechanism for secure, high-frequency agent trading
/// with isolated nonce sub-ranges.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ApproveTradingKeyRequest {
    pub owner_address: AccountId,
    pub trading_key_address: AccountId,
    pub expiry_timestamp: u64,
    pub owner_signature: Vec<u8>,
    pub reason: Option<String>,
}

impl ApproveTradingKeyRequest {
    /// Creates a new approve request with required fields.
    pub fn new(
        owner_address: AccountId,
        trading_key_address: AccountId,
        expiry_timestamp: u64,
        owner_signature: Vec<u8>,
    ) -> Self {
        Self {
            owner_address,
            trading_key_address,
            expiry_timestamp,
            owner_signature,
            reason: None,
        }
    }

    /// Sets an optional reason for the approval (for auditability).
    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgApproveTradingKey = self.clone().into();
        ProtoAny {
            type_url: "/auth.v1.MsgApproveTradingKey".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ApproveTradingKeyRequest> for proto::MsgApproveTradingKey {
    fn from(req: ApproveTradingKeyRequest) -> Self {
        Self {
            owner_address: req.owner_address.to_string(),
            trading_key_address: req.trading_key_address.to_string(),
            expiry_timestamp: req.expiry_timestamp,
            owner_signature: req.owner_signature,
            reason: req.reason.unwrap_or_default(),
        }
    }
}

/// Request to revoke a previously approved Trading Key.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RevokeTradingKeyRequest {
    pub owner_address: AccountId,
    pub trading_key_address: AccountId,
    pub owner_signature: Vec<u8>,
    pub reason: Option<String>,
}

impl RevokeTradingKeyRequest {
    pub fn new(
        owner_address: AccountId,
        trading_key_address: AccountId,
        owner_signature: Vec<u8>,
    ) -> Self {
        Self {
            owner_address,
            trading_key_address,
            owner_signature,
            reason: None,
        }
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgRevokeTradingKey = self.clone().into();
        ProtoAny {
            type_url: "/auth.v1.MsgRevokeTradingKey".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<RevokeTradingKeyRequest> for proto::MsgRevokeTradingKey {
    fn from(req: RevokeTradingKeyRequest) -> Self {
        Self {
            owner_address: req.owner_address.to_string(),
            trading_key_address: req.trading_key_address.to_string(),
            owner_signature: req.owner_signature,
            reason: req.reason.unwrap_or_default(),
        }
    }
}

/// Request to update module parameters via governance.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    pub authority: AccountId,
    pub params: crate::types::Params,
}

impl UpdateParamsRequest {
    pub fn new(authority: AccountId, params: crate::types::Params) -> Self {
        Self { authority, params }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdateParams = self.clone().into();
        ProtoAny {
            type_url: "/auth.v1.MsgUpdateParams".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdateParamsRequest> for proto::MsgUpdateParams {
    fn from(req: UpdateParamsRequest) -> Self {
        Self {
            authority: req.authority.to_string(),
            params: req.params.into(),
        }
    }
}

/// Query request for nonce state of an account.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryNonceStateRequest {
    pub address: AccountId,
}

impl QueryNonceStateRequest {
    pub fn new(address: AccountId) -> Self {
        Self { address }
    }
}

impl From<QueryNonceStateRequest> for proto::QueryNonceStateRequest {
    fn from(req: QueryNonceStateRequest) -> Self {
        Self {
            address: req.address.to_string(),
        }
    }
}

/// Query response containing the full nonce state.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryNonceStateResponse {
    pub state: crate::types::NonceState,
}

impl From<proto::QueryNonceStateResponse> for QueryNonceStateResponse {
    fn from(res: proto::QueryNonceStateResponse) -> Self {
        Self {
            state: res.state.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn approve_request_to_any() {
        let owner = AccountId::new([1u8; 32]);
        let trading_key = AccountId::new([2u8; 32]);

        let req = ApproveTradingKeyRequest::new(owner, trading_key, 1_800_000_000, vec![3; 64])
            .with_reason("Test agent delegation");

        let any = req.to_any();
        assert_eq!(any.type_url, "/auth.v1.MsgApproveTradingKey");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn conversions_work() {
        let req = ApproveTradingKeyRequest::new(
            AccountId::new([1u8; 32]),
            AccountId::new([2u8; 32]),
            1_800_000_000,
            vec![0u8; 64],
        );

        let proto: proto::MsgApproveTradingKey = req.into();
        assert_eq!(proto.expiry_timestamp, 1_800_000_000);
    }
}