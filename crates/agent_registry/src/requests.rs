//! Request and response wrappers for the Agent Registry module.
//!
//! Clean, ergonomic Rust types that wrap the raw protobuf messages. They
//! provide type safety, validation, helper methods, and seamless conversion
//! to/from protobuf for use with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::agent_registry::v1 as proto;

use crate::types::{AgentRecord, ExportStatus, Params};

// ====================== TRANSACTION REQUESTS ======================

/// Request to manually trigger protocol sync for an agent
/// (governance / emergency / testing).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TriggerProtocolSyncRequest {
    pub authority: String,
    pub agent_hash: Vec<u8>,
    pub protocols: Vec<String>,
}

impl TriggerProtocolSyncRequest {
    pub fn new(
        authority: impl Into<String>,
        agent_hash: Vec<u8>,
        protocols: Vec<String>,
    ) -> Self {
        Self {
            authority: authority.into(),
            agent_hash,
            protocols,
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgTriggerProtocolSync = self.clone().into();
        ProtoAny {
            type_url: "/agent_registry.v1.MsgTriggerProtocolSync".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<TriggerProtocolSyncRequest> for proto::MsgTriggerProtocolSync {
    fn from(req: TriggerProtocolSyncRequest) -> Self {
        Self {
            authority: req.authority,
            agent_hash: req.agent_hash,
            protocols: req.protocols,
        }
    }
}

/// Response from triggering a protocol sync.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TriggerProtocolSyncResponse {
    pub success: bool,
}

impl From<proto::MsgTriggerProtocolSyncResponse> for TriggerProtocolSyncResponse {
    fn from(p: proto::MsgTriggerProtocolSyncResponse) -> Self {
        Self { success: p.success }
    }
}

/// Request to update module parameters (governance only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    pub authority: String,
    pub params: Params,
}

impl UpdateParamsRequest {
    pub fn new(authority: impl Into<String>, params: Params) -> Self {
        Self {
            authority: authority.into(),
            params,
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdateParams = self.clone().into();
        ProtoAny {
            type_url: "/agent_registry.v1.MsgUpdateParams".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdateParamsRequest> for proto::MsgUpdateParams {
    fn from(req: UpdateParamsRequest) -> Self {
        Self {
            authority: req.authority,
            params: Some(req.params.into()),
        }
    }
}

/// Response from updating parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsResponse {
    pub success: bool,
}

impl From<proto::MsgUpdateParamsResponse> for UpdateParamsResponse {
    fn from(p: proto::MsgUpdateParamsResponse) -> Self {
        Self { success: p.success }
    }
}

// ====================== QUERY REQUESTS & RESPONSES ======================

/// Query a specific agent record by its hash.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryAgentRecordRequest {
    pub agent_hash: String,
}

impl QueryAgentRecordRequest {
    pub fn new(agent_hash: impl Into<String>) -> Self {
        Self {
            agent_hash: agent_hash.into(),
        }
    }
}

impl From<QueryAgentRecordRequest> for proto::QueryAgentRecordRequest {
    fn from(req: QueryAgentRecordRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
        }
    }
}

/// Response containing an agent record (or `None` if not found).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryAgentRecordResponse {
    pub record: Option<AgentRecord>,
}

impl From<proto::QueryAgentRecordResponse> for QueryAgentRecordResponse {
    fn from(p: proto::QueryAgentRecordResponse) -> Self {
        Self {
            record: p.record.map(Into::into),
        }
    }
}

/// Query an agent by CAIP-10 identifier (cross-chain resolution).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryAgentByCaipRequest {
    pub caip_id: String,
}

impl QueryAgentByCaipRequest {
    pub fn new(caip_id: impl Into<String>) -> Self {
        Self {
            caip_id: caip_id.into(),
        }
    }
}

impl From<QueryAgentByCaipRequest> for proto::QueryAgentByCaipRequest {
    fn from(req: QueryAgentByCaipRequest) -> Self {
        Self {
            caip_id: req.caip_id,
        }
    }
}

/// Response from CAIP-10 resolution.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryAgentByCaipResponse {
    pub record: Option<AgentRecord>,
}

impl From<proto::QueryAgentByCaipResponse> for QueryAgentByCaipResponse {
    fn from(p: proto::QueryAgentByCaipResponse) -> Self {
        Self {
            record: p.record.map(Into::into),
        }
    }
}

/// Query export/sync status for an agent's protocols.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryExportStatusRequest {
    pub agent_hash: String,
    pub protocols: Vec<String>,
}

impl QueryExportStatusRequest {
    pub fn new(agent_hash: impl Into<String>, protocols: Vec<String>) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            protocols,
        }
    }
}

impl From<QueryExportStatusRequest> for proto::QueryExportStatusRequest {
    fn from(req: QueryExportStatusRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            protocols: req.protocols,
        }
    }
}

/// Response containing export statuses.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryExportStatusResponse {
    pub export_statuses: Vec<ExportStatus>,
}

impl From<proto::QueryExportStatusResponse> for QueryExportStatusResponse {
    fn from(p: proto::QueryExportStatusResponse) -> Self {
        Self {
            export_statuses: p.export_statuses.into_iter().map(Into::into).collect(),
        }
    }
}

/// Query the current module parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsRequest;

impl From<QueryParamsRequest> for proto::QueryParamsRequest {
    fn from(_: QueryParamsRequest) -> Self {
        Self {}
    }
}

/// Response containing the current module parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsResponse {
    pub params: Option<Params>,
}

impl From<proto::QueryParamsResponse> for QueryParamsResponse {
    fn from(p: proto::QueryParamsResponse) -> Self {
        Self {
            params: p.params.map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn trigger_protocol_sync_to_any() {
        let req = TriggerProtocolSyncRequest::new(
            "morpheum1gov",
            vec![0xAA; 32],
            vec!["erc8004".into(), "a2a".into()],
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/agent_registry.v1.MsgTriggerProtocolSync");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn update_params_to_any() {
        let req = UpdateParamsRequest::new("morpheum1gov", Params::default());
        let any = req.to_any();
        assert_eq!(any.type_url, "/agent_registry.v1.MsgUpdateParams");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn trigger_protocol_sync_response_conversion() {
        let proto_res = proto::MsgTriggerProtocolSyncResponse { success: true };
        let res: TriggerProtocolSyncResponse = proto_res.into();
        assert!(res.success);
    }

    #[test]
    fn update_params_response_conversion() {
        let proto_res = proto::MsgUpdateParamsResponse { success: true };
        let res: UpdateParamsResponse = proto_res.into();
        assert!(res.success);
    }

    #[test]
    fn query_agent_record_response_conversion() {
        let proto_res = proto::QueryAgentRecordResponse {
            record: Some(proto::AgentRecord {
                agent_hash: vec![0xAA; 32],
                version: 5,
                ..Default::default()
            }),
        };
        let res: QueryAgentRecordResponse = proto_res.into();
        let rec = res.record.unwrap();
        assert_eq!(rec.agent_hash, vec![0xAA; 32]);
        assert_eq!(rec.version, 5);
    }

    #[test]
    fn query_export_status_response_conversion() {
        let proto_res = proto::QueryExportStatusResponse {
            export_statuses: vec![
                proto::ExportStatus {
                    protocol: "erc8004".into(),
                    success: true,
                    ..Default::default()
                },
            ],
        };
        let res: QueryExportStatusResponse = proto_res.into();
        assert_eq!(res.export_statuses.len(), 1);
        assert_eq!(res.export_statuses[0].protocol, "erc8004");
    }

    #[test]
    fn query_params_response_conversion() {
        let proto_res = proto::QueryParamsResponse {
            params: Some(proto::Params {
                max_metadata_size_bytes: 2_000_000,
                sync_timeout_ms: 200,
                enable_auto_export: false,
                default_visibility: 2,
                max_export_retries: 5,
            }),
        };
        let res: QueryParamsResponse = proto_res.into();
        let p = res.params.unwrap();
        assert_eq!(p.max_metadata_size_bytes, 2_000_000);
        assert!(!p.enable_auto_export);
    }
}
