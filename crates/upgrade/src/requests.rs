//! Request and response wrappers for the Upgrade module.
//!
//! Type-safe Rust APIs around the raw protobuf messages. Uses `AccountId` for
//! addresses, provides ergonomic constructors, and includes `to_any()` methods
//! for seamless integration with `TxBuilder`.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::upgrade::v1 as proto;
use morpheum_sdk_core::AccountId;

use crate::types::{UpgradeStatus, UpgradeType};

// ====================== TRANSACTION REQUESTS ======================

/// Signal that a validator has loaded the new binary in shadow mode and is ready.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SignalUpgradeReadyRequest {
    pub from_address: AccountId,
    pub upgrade_id: u64,
    pub validator_pubkey: Vec<u8>,
    pub signature: Vec<u8>,
}

impl SignalUpgradeReadyRequest {
    pub fn new(
        from_address: AccountId,
        upgrade_id: u64,
        validator_pubkey: Vec<u8>,
        signature: Vec<u8>,
    ) -> Self {
        Self { from_address, upgrade_id, validator_pubkey, signature }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgSignalUpgradeReadyRequest = self.clone().into();
        ProtoAny {
            type_url: "/upgrade.v1.MsgSignalUpgradeReadyRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<SignalUpgradeReadyRequest> for proto::MsgSignalUpgradeReadyRequest {
    fn from(req: SignalUpgradeReadyRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            upgrade_id: req.upgrade_id,
            validator_pubkey: req.validator_pubkey,
            signature: req.signature,
            timestamp: None,
        }
    }
}

/// Cancel a scheduled upgrade (governance or emergency admin path).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CancelUpgradeRequest {
    pub from_address: AccountId,
    pub upgrade_id: u64,
    pub reason: String,
}

impl CancelUpgradeRequest {
    pub fn new(
        from_address: AccountId,
        upgrade_id: u64,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            from_address,
            upgrade_id,
            reason: reason.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCancelUpgradeRequest = self.clone().into();
        ProtoAny {
            type_url: "/upgrade.v1.MsgCancelUpgradeRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CancelUpgradeRequest> for proto::MsgCancelUpgradeRequest {
    fn from(req: CancelUpgradeRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            upgrade_id: req.upgrade_id,
            reason: req.reason,
            timestamp: None,
        }
    }
}

/// Manually trigger activation of an upgrade (safety net after readiness + grace period).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecuteUpgradeRequest {
    pub from_address: AccountId,
    pub upgrade_id: u64,
}

impl ExecuteUpgradeRequest {
    pub fn new(from_address: AccountId, upgrade_id: u64) -> Self {
        Self { from_address, upgrade_id }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgExecuteUpgradeRequest = self.clone().into();
        ProtoAny {
            type_url: "/upgrade.v1.MsgExecuteUpgradeRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ExecuteUpgradeRequest> for proto::MsgExecuteUpgradeRequest {
    fn from(req: ExecuteUpgradeRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            upgrade_id: req.upgrade_id,
            timestamp: None,
        }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query a single upgrade by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryUpgradeRequest {
    pub upgrade_id: u64,
}

impl QueryUpgradeRequest {
    pub fn new(upgrade_id: u64) -> Self {
        Self { upgrade_id }
    }
}

impl From<QueryUpgradeRequest> for proto::QueryUpgradeRequest {
    fn from(req: QueryUpgradeRequest) -> Self {
        Self { upgrade_id: req.upgrade_id }
    }
}

/// Query upgrades with optional filters and pagination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryUpgradesRequest {
    pub limit: i32,
    pub offset: i32,
    pub status_filter: Option<UpgradeStatus>,
    pub type_filter: Option<UpgradeType>,
}

impl QueryUpgradesRequest {
    pub fn new(limit: i32, offset: i32) -> Self {
        Self {
            limit,
            offset,
            status_filter: None,
            type_filter: None,
        }
    }

    pub fn status_filter(mut self, status: UpgradeStatus) -> Self {
        self.status_filter = Some(status);
        self
    }

    pub fn type_filter(mut self, upgrade_type: UpgradeType) -> Self {
        self.type_filter = Some(upgrade_type);
        self
    }
}

impl From<QueryUpgradesRequest> for proto::QueryUpgradesRequest {
    fn from(req: QueryUpgradesRequest) -> Self {
        Self {
            limit: req.limit,
            offset: req.offset,
            status_filter: req.status_filter.map(i32::from).unwrap_or(0),
            type_filter: req.type_filter.map(i32::from).unwrap_or(0),
        }
    }
}

/// Query currently active upgrades (usually 0 or 1).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryActiveUpgradesRequest {
    pub limit: i32,
    pub offset: i32,
}

impl QueryActiveUpgradesRequest {
    pub fn new(limit: i32, offset: i32) -> Self {
        Self { limit, offset }
    }
}

impl From<QueryActiveUpgradesRequest> for proto::QueryActiveUpgradesRequest {
    fn from(req: QueryActiveUpgradesRequest) -> Self {
        Self {
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query validator readiness for a specific upgrade.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryValidatorReadinessRequest {
    pub upgrade_id: u64,
    pub validator_address: Option<String>,
}

impl QueryValidatorReadinessRequest {
    pub fn new(upgrade_id: u64) -> Self {
        Self { upgrade_id, validator_address: None }
    }

    pub fn validator_address(mut self, address: impl Into<String>) -> Self {
        self.validator_address = Some(address.into());
        self
    }
}

impl From<QueryValidatorReadinessRequest> for proto::QueryValidatorReadinessRequest {
    fn from(req: QueryValidatorReadinessRequest) -> Self {
        Self {
            upgrade_id: req.upgrade_id,
            validator_address: req.validator_address.unwrap_or_default(),
        }
    }
}

/// Query upgrade status summary (fast path for AI agents).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryUpgradeStatusRequest {
    pub upgrade_id: u64,
}

impl QueryUpgradeStatusRequest {
    pub fn new(upgrade_id: u64) -> Self {
        Self { upgrade_id }
    }
}

impl From<QueryUpgradeStatusRequest> for proto::QueryUpgradeStatusRequest {
    fn from(req: QueryUpgradeStatusRequest) -> Self {
        Self { upgrade_id: req.upgrade_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn signal_upgrade_ready_to_any() {
        let from = AccountId::new([1u8; 32]);
        let req = SignalUpgradeReadyRequest::new(
            from,
            42,
            alloc::vec![0xaa, 0xbb],
            alloc::vec![0x01, 0x02],
        );

        let any = req.to_any();
        assert_eq!(any.type_url, "/upgrade.v1.MsgSignalUpgradeReadyRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn cancel_upgrade_to_any() {
        let from = AccountId::new([2u8; 32]);
        let req = CancelUpgradeRequest::new(from, 5, "critical bug found");

        let any = req.to_any();
        assert_eq!(any.type_url, "/upgrade.v1.MsgCancelUpgradeRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn execute_upgrade_to_any() {
        let from = AccountId::new([3u8; 32]);
        let req = ExecuteUpgradeRequest::new(from, 10);

        let any = req.to_any();
        assert_eq!(any.type_url, "/upgrade.v1.MsgExecuteUpgradeRequest");
    }

    #[test]
    fn query_upgrades_with_filters() {
        let req = QueryUpgradesRequest::new(20, 0)
            .status_filter(UpgradeStatus::ShadowMode)
            .type_filter(UpgradeType::Binary);

        let proto_req: proto::QueryUpgradesRequest = req.into();
        assert_eq!(proto_req.limit, 20);
        assert_eq!(proto_req.status_filter, i32::from(UpgradeStatus::ShadowMode));
        assert_eq!(proto_req.type_filter, i32::from(UpgradeType::Binary));
    }

    #[test]
    fn query_validator_readiness_with_filter() {
        let req = QueryValidatorReadinessRequest::new(7)
            .validator_address("morpheum1val123");

        let proto_req: proto::QueryValidatorReadinessRequest = req.into();
        assert_eq!(proto_req.upgrade_id, 7);
        assert_eq!(proto_req.validator_address, "morpheum1val123");
    }
}
