//! Request and response wrappers for the Memory module.
//!
//! These are clean, ergonomic Rust types that wrap the raw protobuf messages.
//! They provide type safety, validation, helper methods, and seamless conversion
//! to/from protobuf for use with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::types::Params;
use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::memory::v1 as proto;

use crate::types::{MemoryEntry, MemoryEntryType, MemoryRoot};

// ====================== TRANSACTION REQUESTS ======================

/// Request to store a new memory entry.
///
/// The `owner_signature` must be signed by the agent's owner or a delegated VC
/// to authorise the write.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoreEntryRequest {
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Unique key within the agent's memory namespace.
    pub key: String,
    /// Raw value bytes.
    pub value: Vec<u8>,
    /// Type classification of this entry.
    pub entry_type: MemoryEntryType,
    /// Expiry timestamp (0 = never expires).
    pub expires_at: u64,
    /// Owner or delegated VC signature.
    pub owner_signature: Vec<u8>,
}

impl StoreEntryRequest {
    /// Creates a new store-entry request with required fields.
    pub fn new(
        agent_hash: impl Into<String>,
        key: impl Into<String>,
        value: Vec<u8>,
        entry_type: MemoryEntryType,
        owner_signature: Vec<u8>,
    ) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            key: key.into(),
            value,
            entry_type,
            expires_at: 0,
            owner_signature,
        }
    }

    /// Sets the expiry timestamp.
    pub fn with_expires_at(mut self, expires_at: u64) -> Self {
        self.expires_at = expires_at;
        self
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgStoreEntry = self.clone().into();
        ProtoAny {
            type_url: "/memory.v1.MsgStoreEntry".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<StoreEntryRequest> for proto::MsgStoreEntry {
    fn from(req: StoreEntryRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            key: req.key,
            value: req.value,
            entry_type: req.entry_type.to_proto(),
            expires_at: req.expires_at,
            owner_signature: req.owner_signature,
        }
    }
}

/// Response from storing a memory entry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StoreEntryResponse {
    pub success: bool,
    pub entry_key: String,
    pub timestamp: u64,
}

impl From<proto::StoreEntryResponse> for StoreEntryResponse {
    fn from(p: proto::StoreEntryResponse) -> Self {
        Self {
            success: p.success,
            entry_key: p.entry_key,
            timestamp: p.timestamp,
        }
    }
}

/// Request to update an existing memory entry.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateEntryRequest {
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Key of the entry to update.
    pub key: String,
    /// New value bytes.
    pub new_value: Vec<u8>,
    /// New expiry timestamp (0 = never expires).
    pub new_expires_at: u64,
    /// Owner or delegated VC signature.
    pub owner_signature: Vec<u8>,
}

impl UpdateEntryRequest {
    /// Creates a new update-entry request.
    pub fn new(
        agent_hash: impl Into<String>,
        key: impl Into<String>,
        new_value: Vec<u8>,
        owner_signature: Vec<u8>,
    ) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            key: key.into(),
            new_value,
            new_expires_at: 0,
            owner_signature,
        }
    }

    /// Sets the new expiry timestamp.
    pub fn with_new_expires_at(mut self, expires_at: u64) -> Self {
        self.new_expires_at = expires_at;
        self
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdateEntry = self.clone().into();
        ProtoAny {
            type_url: "/memory.v1.MsgUpdateEntry".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdateEntryRequest> for proto::MsgUpdateEntry {
    fn from(req: UpdateEntryRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            key: req.key,
            new_value: req.new_value,
            new_expires_at: req.new_expires_at,
            owner_signature: req.owner_signature,
        }
    }
}

/// Response from updating a memory entry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateEntryResponse {
    pub success: bool,
    pub updated_at: u64,
}

impl From<proto::UpdateEntryResponse> for UpdateEntryResponse {
    fn from(p: proto::UpdateEntryResponse) -> Self {
        Self {
            success: p.success,
            updated_at: p.updated_at,
        }
    }
}

/// Request to delete a memory entry.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeleteEntryRequest {
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Key of the entry to delete.
    pub key: String,
    /// Owner or delegated VC signature.
    pub owner_signature: Vec<u8>,
}

impl DeleteEntryRequest {
    /// Creates a new delete-entry request.
    pub fn new(
        agent_hash: impl Into<String>,
        key: impl Into<String>,
        owner_signature: Vec<u8>,
    ) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            key: key.into(),
            owner_signature,
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgDeleteEntry = self.clone().into();
        ProtoAny {
            type_url: "/memory.v1.MsgDeleteEntry".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<DeleteEntryRequest> for proto::MsgDeleteEntry {
    fn from(req: DeleteEntryRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            key: req.key,
            owner_signature: req.owner_signature,
        }
    }
}

/// Response from deleting a memory entry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeleteEntryResponse {
    pub success: bool,
    pub deleted_at: u64,
}

impl From<proto::DeleteEntryResponse> for DeleteEntryResponse {
    fn from(p: proto::DeleteEntryResponse) -> Self {
        Self {
            success: p.success,
            deleted_at: p.deleted_at,
        }
    }
}

// ====================== QUERY REQUESTS & RESPONSES ======================

/// Query a specific memory entry by agent hash and key.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMemoryEntryRequest {
    pub agent_hash: String,
    pub key: String,
}

impl QueryMemoryEntryRequest {
    pub fn new(agent_hash: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            key: key.into(),
        }
    }
}

impl From<QueryMemoryEntryRequest> for proto::QueryMemoryEntryRequest {
    fn from(req: QueryMemoryEntryRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            key: req.key,
        }
    }
}

/// Response containing a memory entry (or indicating not found).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMemoryEntryResponse {
    pub entry: Option<MemoryEntry>,
    pub found: bool,
}

impl From<proto::QueryMemoryEntryResponse> for QueryMemoryEntryResponse {
    fn from(p: proto::QueryMemoryEntryResponse) -> Self {
        Self {
            entry: p.entry.map(Into::into),
            found: p.found,
        }
    }
}

/// Query all memory entries for an agent (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMemoryEntriesByAgentRequest {
    pub agent_hash: String,
    pub limit: u32,
    pub offset: u32,
}

impl QueryMemoryEntriesByAgentRequest {
    pub fn new(agent_hash: impl Into<String>, limit: u32, offset: u32) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            limit,
            offset,
        }
    }
}

impl From<QueryMemoryEntriesByAgentRequest> for proto::QueryMemoryEntriesByAgentRequest {
    fn from(req: QueryMemoryEntriesByAgentRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Response containing paginated memory entries for an agent.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMemoryEntriesByAgentResponse {
    pub entries: Vec<MemoryEntry>,
    pub total_count: u32,
}

impl From<proto::QueryMemoryEntriesByAgentResponse> for QueryMemoryEntriesByAgentResponse {
    fn from(p: proto::QueryMemoryEntriesByAgentResponse) -> Self {
        Self {
            entries: p.entries.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
        }
    }
}

/// Query the memory root for an agent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMemoryRootRequest {
    pub agent_hash: String,
}

impl QueryMemoryRootRequest {
    pub fn new(agent_hash: impl Into<String>) -> Self {
        Self { agent_hash: agent_hash.into() }
    }
}

impl From<QueryMemoryRootRequest> for proto::QueryMemoryRootRequest {
    fn from(req: QueryMemoryRootRequest) -> Self {
        Self { agent_hash: req.agent_hash }
    }
}

/// Response containing the memory root (or indicating not found).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMemoryRootResponse {
    pub root: Option<MemoryRoot>,
    pub found: bool,
}

impl From<proto::QueryMemoryRootResponse> for QueryMemoryRootResponse {
    fn from(p: proto::QueryMemoryRootResponse) -> Self {
        Self {
            root: p.root.map(Into::into),
            found: p.found,
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
    fn store_entry_request_to_any() {
        let req = StoreEntryRequest::new(
            "agent-abc",
            "strategy/v1",
            vec![1, 2, 3],
            MemoryEntryType::Semantic,
            vec![0u8; 64],
        )
        .with_expires_at(1_700_003_600);

        let any = req.to_any();
        assert_eq!(any.type_url, "/memory.v1.MsgStoreEntry");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn update_entry_request_to_any() {
        let req = UpdateEntryRequest::new(
            "agent-abc",
            "strategy/v1",
            vec![4, 5, 6],
            vec![0u8; 64],
        )
        .with_new_expires_at(1_700_010_000);

        let any = req.to_any();
        assert_eq!(any.type_url, "/memory.v1.MsgUpdateEntry");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn delete_entry_request_to_any() {
        let req = DeleteEntryRequest::new("agent-abc", "strategy/v1", vec![0u8; 64]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/memory.v1.MsgDeleteEntry");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn store_entry_response_conversion() {
        let proto_res = proto::StoreEntryResponse {
            success: true,
            entry_key: "strategy/v1".into(),
            timestamp: 1_700_000_000,
        };
        let res: StoreEntryResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.entry_key, "strategy/v1");
        assert_eq!(res.timestamp, 1_700_000_000);
    }

    #[test]
    fn update_entry_response_conversion() {
        let proto_res = proto::UpdateEntryResponse {
            success: true,
            updated_at: 1_700_001_000,
        };
        let res: UpdateEntryResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.updated_at, 1_700_001_000);
    }

    #[test]
    fn delete_entry_response_conversion() {
        let proto_res = proto::DeleteEntryResponse {
            success: true,
            deleted_at: 1_700_002_000,
        };
        let res: DeleteEntryResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.deleted_at, 1_700_002_000);
    }

    #[test]
    fn query_memory_entry_response_conversion() {
        let proto_res = proto::QueryMemoryEntryResponse {
            entry: Some(proto::MemoryEntry {
                agent_hash: "agent-abc".into(),
                key: "strategy/v1".into(),
                value: vec![1, 2, 3],
                entry_type: 1, // Semantic
                timestamp: 1_700_000_000,
                expires_at: 0,
                version: "1.0".into(),
                deposit_amount: 1_000,
            }),
            found: true,
        };
        let res: QueryMemoryEntryResponse = proto_res.into();
        assert!(res.found);
        let entry = res.entry.unwrap();
        assert_eq!(entry.agent_hash, "agent-abc");
        assert_eq!(entry.entry_type, MemoryEntryType::Semantic);
        assert_eq!(entry.deposit_amount, 1_000);
    }

    #[test]
    fn query_memory_entries_by_agent_response_conversion() {
        let proto_res = proto::QueryMemoryEntriesByAgentResponse {
            entries: vec![Default::default(), Default::default()],
            total_count: 2,
        };
        let res: QueryMemoryEntriesByAgentResponse = proto_res.into();
        assert_eq!(res.total_count, 2);
        assert_eq!(res.entries.len(), 2);
    }

    #[test]
    fn query_memory_root_response_conversion() {
        let proto_res = proto::QueryMemoryRootResponse {
            root: Some(proto::MemoryRoot {
                agent_hash: "agent-abc".into(),
                root_hash: "deadbeef".into(),
                last_updated: 1_700_000_000,
                entry_count: 42,
                total_size_bytes: 1_048_576,
            }),
            found: true,
        };
        let res: QueryMemoryRootResponse = proto_res.into();
        assert!(res.found);
        let root = res.root.unwrap();
        assert_eq!(root.entry_count, 42);
    }

    #[test]
    fn query_params_response_conversion() {
        let proto_res = proto::QueryParamsResponse {
            params: Some(proto::Params {
                max_memory_per_agent_bytes: 10_000_000,
                default_entry_ttl_seconds: 86400,
                enable_vector_search: true,
                max_entries_per_agent: 1000,
                default_vector_dimension: 768,
                enable_auto_pruning: true,
            }),
        };
        let res: QueryParamsResponse = proto_res.into();
        let p = res.params.unwrap();
        assert_eq!(p.max_memory_per_agent_bytes, 10_000_000);
        assert_eq!(p.default_vector_dimension, 768);
        assert!(p.enable_auto_pruning);
    }
}
