//! Domain types for the Agent Registry module.
//!
//! Clean, idiomatic Rust representations of the agent_registry protobuf
//! messages. Full round-trip conversion to/from protobuf and `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::agent_registry::v1 as proto;

// ====================== VISIBILITY LEVEL ======================

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum VisibilityLevel {
    #[default]
    Unspecified = 0,
    Private = 1,
    Discoverable = 2,
    Public = 3,
}

impl VisibilityLevel {
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Private,
            2 => Self::Discoverable,
            3 => Self::Public,
            _ => Self::Unspecified,
        }
    }

    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for VisibilityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unspecified => f.write_str("UNSPECIFIED"),
            Self::Private => f.write_str("PRIVATE"),
            Self::Discoverable => f.write_str("DISCOVERABLE"),
            Self::Public => f.write_str("PUBLIC"),
        }
    }
}

// ====================== CAIP AGENT ID ======================

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CaipAgentId {
    pub namespace: String,
    pub chain_id: u64,
    pub actor_reference: String,
    pub full_hash: Vec<u8>,
}

impl CaipAgentId {
    pub fn to_caip_string(&self) -> String {
        alloc::format!("{}:{}:{}", self.namespace, self.chain_id, self.actor_reference)
    }
}

impl From<proto::CaipAgentId> for CaipAgentId {
    fn from(p: proto::CaipAgentId) -> Self {
        Self {
            namespace: p.namespace,
            chain_id: p.chain_id,
            actor_reference: p.actor_reference,
            full_hash: p.full_hash,
        }
    }
}

impl From<CaipAgentId> for proto::CaipAgentId {
    fn from(c: CaipAgentId) -> Self {
        Self {
            namespace: c.namespace,
            chain_id: c.chain_id,
            actor_reference: c.actor_reference,
            full_hash: c.full_hash,
        }
    }
}

// ====================== AGENT RECORD ======================

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentRecord {
    pub agent_hash: Vec<u8>,
    pub caip_id: Option<CaipAgentId>,
    pub metadata_hash: Vec<u8>,
    pub erc8004_nft_id: Vec<u8>,
    pub a2a_card_hash: Vec<u8>,
    pub mcp_manifest_hash: Vec<u8>,
    pub did_document_hash: Vec<u8>,
    pub x402_payment_address: Vec<u8>,
    pub updated_at: u64,
    pub last_synced: u64,
    pub reputation_score: u32,
    pub reputation_level: u32,
    pub validation_status: u32,
    pub version: u32,
    pub visibility: VisibilityLevel,
    pub blob_merkle_root: Vec<u8>,
}

impl AgentRecord {
    pub fn agent_hash_hex(&self) -> String {
        hex::encode(&self.agent_hash)
    }

    pub fn is_active(&self) -> bool {
        self.validation_status & (1 << 5) != 0
    }

    pub fn is_identity_verified(&self) -> bool {
        self.validation_status & 1 != 0
    }
}

impl From<proto::AgentRecord> for AgentRecord {
    fn from(p: proto::AgentRecord) -> Self {
        Self {
            agent_hash: p.agent_hash,
            caip_id: p.caip_id.map(Into::into),
            metadata_hash: p.metadata_hash,
            erc8004_nft_id: p.erc8004_nft_id,
            a2a_card_hash: p.a2a_card_hash,
            mcp_manifest_hash: p.mcp_manifest_hash,
            did_document_hash: p.did_document_hash,
            x402_payment_address: p.x402_payment_address,
            updated_at: p.updated_at,
            last_synced: p.last_synced,
            reputation_score: p.reputation_score,
            reputation_level: p.reputation_level,
            validation_status: p.validation_status,
            version: p.version,
            visibility: VisibilityLevel::from_proto(p.visibility),
            blob_merkle_root: p.blob_merkle_root,
        }
    }
}

impl From<AgentRecord> for proto::AgentRecord {
    fn from(r: AgentRecord) -> Self {
        Self {
            agent_hash: r.agent_hash,
            caip_id: r.caip_id.map(Into::into),
            metadata_hash: r.metadata_hash,
            erc8004_nft_id: r.erc8004_nft_id,
            a2a_card_hash: r.a2a_card_hash,
            mcp_manifest_hash: r.mcp_manifest_hash,
            did_document_hash: r.did_document_hash,
            x402_payment_address: r.x402_payment_address,
            updated_at: r.updated_at,
            last_synced: r.last_synced,
            reputation_score: r.reputation_score,
            reputation_level: r.reputation_level,
            validation_status: r.validation_status,
            version: r.version,
            visibility: r.visibility.to_proto(),
            blob_merkle_root: r.blob_merkle_root,
            extension: None,
        }
    }
}

// ====================== EXPORT STATUS ======================

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExportStatus {
    pub protocol: String,
    pub view_hash: Vec<u8>,
    pub last_exported_at: u64,
    pub success: bool,
    pub error_message: String,
}

impl From<proto::ExportStatus> for ExportStatus {
    fn from(p: proto::ExportStatus) -> Self {
        Self {
            protocol: p.protocol,
            view_hash: p.view_hash,
            last_exported_at: p.last_exported_at,
            success: p.success,
            error_message: p.error_message,
        }
    }
}

impl From<ExportStatus> for proto::ExportStatus {
    fn from(e: ExportStatus) -> Self {
        Self {
            protocol: e.protocol,
            view_hash: e.view_hash,
            last_exported_at: e.last_exported_at,
            success: e.success,
            error_message: e.error_message,
        }
    }
}

// ====================== PARAMS ======================

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    pub max_metadata_size_bytes: u64,
    pub sync_timeout_ms: u64,
    pub enable_auto_export: bool,
    pub default_visibility: VisibilityLevel,
    pub max_export_retries: u64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            max_metadata_size_bytes: 1_048_576,
            sync_timeout_ms: 100,
            enable_auto_export: true,
            default_visibility: VisibilityLevel::Public,
            max_export_retries: 3,
        }
    }
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            max_metadata_size_bytes: p.max_metadata_size_bytes,
            sync_timeout_ms: p.sync_timeout_ms,
            enable_auto_export: p.enable_auto_export,
            default_visibility: VisibilityLevel::from_proto(p.default_visibility),
            max_export_retries: p.max_export_retries,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            max_metadata_size_bytes: p.max_metadata_size_bytes,
            sync_timeout_ms: p.sync_timeout_ms,
            enable_auto_export: p.enable_auto_export,
            default_visibility: p.default_visibility.to_proto(),
            max_export_retries: p.max_export_retries,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn visibility_level_roundtrip() {
        for v in [
            VisibilityLevel::Unspecified,
            VisibilityLevel::Private,
            VisibilityLevel::Discoverable,
            VisibilityLevel::Public,
        ] {
            assert_eq!(VisibilityLevel::from_proto(v.to_proto()), v);
        }
    }

    #[test]
    fn agent_record_roundtrip() {
        let record = AgentRecord {
            agent_hash: vec![0xAA; 32],
            caip_id: Some(CaipAgentId {
                namespace: "morpheum".into(),
                chain_id: 1,
                actor_reference: "actor-0x123".into(),
                full_hash: vec![0xBB; 32],
            }),
            version: 3,
            visibility: VisibilityLevel::Public,
            validation_status: 0b100001,
            ..Default::default()
        };
        let proto: proto::AgentRecord = record.clone().into();
        let back: AgentRecord = proto.into();
        assert_eq!(record, back);
        assert!(back.is_active());
        assert!(back.is_identity_verified());
    }

    #[test]
    fn export_status_roundtrip() {
        let status = ExportStatus {
            protocol: "erc8004".into(),
            view_hash: vec![1, 2, 3],
            last_exported_at: 1_700_000_000,
            success: true,
            error_message: String::new(),
        };
        let proto: proto::ExportStatus = status.clone().into();
        let back: ExportStatus = proto.into();
        assert_eq!(status, back);
    }

    #[test]
    fn params_roundtrip() {
        let params = Params {
            max_metadata_size_bytes: 2_000_000,
            sync_timeout_ms: 200,
            enable_auto_export: false,
            default_visibility: VisibilityLevel::Discoverable,
            max_export_retries: 5,
        };
        let proto: proto::Params = params.clone().into();
        let back: Params = proto.into();
        assert_eq!(params, back);
    }

    #[test]
    fn caip_to_string() {
        let caip = CaipAgentId {
            namespace: "morpheum".into(),
            chain_id: 42,
            actor_reference: "actor-0xDEAD".into(),
            full_hash: vec![],
        };
        assert_eq!(caip.to_caip_string(), "morpheum:42:actor-0xDEAD");
    }
}
