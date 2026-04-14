//! Domain types for the Memory module.
//!
//! These are clean, idiomatic Rust representations of the memory protobuf
//! messages. They provide type safety, ergonomic APIs, and full round-trip
//! conversion to/from protobuf while remaining strictly `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::memory::v1 as proto;

// ====================== MEMORY ENTRY TYPE ======================

/// Type classification for a memory entry.
///
/// Mirrors the protobuf `MemoryEntryType` enum.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum MemoryEntryType {
    /// Past events / trades.
    #[default]
    Episodic = 0,
    /// Learned knowledge / strategies.
    Semantic = 1,
    /// Rules / templates.
    Procedural = 2,
    /// Raw embedding vector.
    Vector = 3,
    /// Application-defined custom type.
    Custom = 255,
}

impl MemoryEntryType {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Semantic,
            2 => Self::Procedural,
            3 => Self::Vector,
            255 => Self::Custom,
            _ => Self::Episodic,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }
}

impl fmt::Display for MemoryEntryType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Episodic => f.write_str("EPISODIC"),
            Self::Semantic => f.write_str("SEMANTIC"),
            Self::Procedural => f.write_str("PROCEDURAL"),
            Self::Vector => f.write_str("VECTOR"),
            Self::Custom => f.write_str("CUSTOM"),
        }
    }
}

// ====================== MEMORY ENTRY ======================

/// Individual memory entry for an agent.
///
/// Represents a single piece of stored knowledge, experience, or embedding
/// within an agent's memory space.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryEntry {
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Unique key within the agent's memory namespace.
    pub key: String,
    /// Raw value bytes (opaque to the module).
    pub value: Vec<u8>,
    /// Type classification of this entry.
    pub entry_type: MemoryEntryType,
    /// Block timestamp when the entry was created.
    pub timestamp: u64,
    /// Expiry timestamp (0 = never expires).
    pub expires_at: u64,
    /// Application-defined version string.
    pub version: String,
    /// Storage deposit currently held for this entry, in uMORM.
    pub deposit_amount: u64,
}

impl MemoryEntry {
    /// Returns `true` if this entry has an expiry set.
    pub fn has_expiry(&self) -> bool {
        self.expires_at > 0
    }

    /// Returns `true` if this entry has expired relative to the given timestamp.
    pub fn is_expired(&self, now: u64) -> bool {
        self.has_expiry() && now >= self.expires_at
    }
}

impl From<proto::MemoryEntry> for MemoryEntry {
    fn from(p: proto::MemoryEntry) -> Self {
        Self {
            agent_hash: p.agent_hash,
            key: p.key,
            value: p.value,
            entry_type: MemoryEntryType::from_proto(p.entry_type),
            timestamp: p.timestamp,
            expires_at: p.expires_at,
            version: p.version,
            deposit_amount: p.deposit_amount,
        }
    }
}

impl From<MemoryEntry> for proto::MemoryEntry {
    fn from(e: MemoryEntry) -> Self {
        Self {
            agent_hash: e.agent_hash,
            key: e.key,
            value: e.value,
            entry_type: e.entry_type.to_proto(),
            timestamp: e.timestamp,
            expires_at: e.expires_at,
            version: e.version,
            deposit_amount: e.deposit_amount,
        }
    }
}

// ====================== VECTOR EMBEDDING ======================

/// Vector embedding for semantic search within an agent's memory.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VectorEmbedding {
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Key identifying the embedding.
    pub key: String,
    /// Embedding vector (typically 512 or 768 dimensions).
    pub vector: Vec<f32>,
    /// Block timestamp when the embedding was stored.
    pub timestamp: u64,
}

impl VectorEmbedding {
    /// Returns the dimensionality of the embedding vector.
    pub fn dimension(&self) -> usize {
        self.vector.len()
    }
}

impl From<proto::VectorEmbedding> for VectorEmbedding {
    fn from(p: proto::VectorEmbedding) -> Self {
        Self {
            agent_hash: p.agent_hash,
            key: p.key,
            vector: p.vector,
            timestamp: p.timestamp,
        }
    }
}

impl From<VectorEmbedding> for proto::VectorEmbedding {
    fn from(v: VectorEmbedding) -> Self {
        Self {
            agent_hash: v.agent_hash,
            key: v.key,
            vector: v.vector,
            timestamp: v.timestamp,
        }
    }
}

// ====================== MEMORY ROOT ======================

/// Anchor point for all memory belonging to an agent.
///
/// Contains the Merkle root digest, entry count, and total size, providing
/// a compact summary suitable for on-chain verification and marketplace listings.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemoryRoot {
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Merkle root covering all entries.
    pub root_hash: String,
    /// Timestamp of the last update.
    pub last_updated: u64,
    /// Number of entries.
    pub entry_count: u32,
    /// Total storage consumed in bytes.
    pub total_size_bytes: u64,
}

impl From<proto::MemoryRoot> for MemoryRoot {
    fn from(p: proto::MemoryRoot) -> Self {
        Self {
            agent_hash: p.agent_hash,
            root_hash: p.root_hash,
            last_updated: p.last_updated,
            entry_count: p.entry_count,
            total_size_bytes: p.total_size_bytes,
        }
    }
}

impl From<MemoryRoot> for proto::MemoryRoot {
    fn from(r: MemoryRoot) -> Self {
        Self {
            agent_hash: r.agent_hash,
            root_hash: r.root_hash,
            last_updated: r.last_updated,
            entry_count: r.entry_count,
            total_size_bytes: r.total_size_bytes,
        }
    }
}

// ====================== MEMORY SNAPSHOT ======================

/// Point-in-time snapshot of an agent's memory state.
///
/// Used for marketplace listings, evaluations, and auditing.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemorySnapshot {
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Merkle root at snapshot time.
    pub root_hash: String,
    /// Timestamp when the snapshot was taken.
    pub snapshot_timestamp: u64,
    /// Number of entries at snapshot time.
    pub entry_count: u32,
    /// Short human-readable summary.
    pub summary: String,
}

impl From<proto::MemorySnapshot> for MemorySnapshot {
    fn from(p: proto::MemorySnapshot) -> Self {
        Self {
            agent_hash: p.agent_hash,
            root_hash: p.root_hash,
            snapshot_timestamp: p.snapshot_timestamp,
            entry_count: p.entry_count,
            summary: p.summary,
        }
    }
}

impl From<MemorySnapshot> for proto::MemorySnapshot {
    fn from(s: MemorySnapshot) -> Self {
        Self {
            agent_hash: s.agent_hash,
            root_hash: s.root_hash,
            snapshot_timestamp: s.snapshot_timestamp,
            entry_count: s.entry_count,
            summary: s.summary,
        }
    }
}

// ====================== PARAMS ======================

/// Module parameters (governance-controlled).
///
/// Provides sensible defaults:
/// - `max_memory_per_agent_bytes`: 0 (unlimited)
/// - `default_entry_ttl_seconds`: 0 (never expires)
/// - `enable_vector_search`: true
/// - `max_entries_per_agent`: 0 (unlimited)
/// - `default_vector_dimension`: 512
/// - `enable_auto_pruning`: false
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    /// Maximum memory size per agent in bytes (0 = unlimited).
    pub max_memory_per_agent_bytes: u64,
    /// Default TTL for memory entries in seconds (0 = never expires).
    pub default_entry_ttl_seconds: u64,
    /// Whether vector search / embeddings are enabled.
    pub enable_vector_search: bool,
    /// Maximum number of entries allowed per agent (0 = unlimited).
    pub max_entries_per_agent: u32,
    /// Default dimension size for vector embeddings.
    pub default_vector_dimension: u32,
    /// Whether automatic pruning of expired entries is enabled.
    pub enable_auto_pruning: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            max_memory_per_agent_bytes: 0,
            default_entry_ttl_seconds: 0,
            enable_vector_search: true,
            max_entries_per_agent: 0,
            default_vector_dimension: 512,
            enable_auto_pruning: false,
        }
    }
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            max_memory_per_agent_bytes: p.max_memory_per_agent_bytes,
            default_entry_ttl_seconds: p.default_entry_ttl_seconds,
            enable_vector_search: p.enable_vector_search,
            max_entries_per_agent: p.max_entries_per_agent,
            default_vector_dimension: p.default_vector_dimension,
            enable_auto_pruning: p.enable_auto_pruning,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            max_memory_per_agent_bytes: p.max_memory_per_agent_bytes,
            default_entry_ttl_seconds: p.default_entry_ttl_seconds,
            enable_vector_search: p.enable_vector_search,
            max_entries_per_agent: p.max_entries_per_agent,
            default_vector_dimension: p.default_vector_dimension,
            enable_auto_pruning: p.enable_auto_pruning,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use alloc::vec;

    #[test]
    fn memory_entry_type_roundtrip() {
        for t in [
            MemoryEntryType::Episodic,
            MemoryEntryType::Semantic,
            MemoryEntryType::Procedural,
            MemoryEntryType::Vector,
            MemoryEntryType::Custom,
        ] {
            assert_eq!(MemoryEntryType::from_proto(t.to_proto()), t);
        }
    }

    #[test]
    fn memory_entry_type_display() {
        assert_eq!(MemoryEntryType::Episodic.to_string(), "EPISODIC");
        assert_eq!(MemoryEntryType::Semantic.to_string(), "SEMANTIC");
        assert_eq!(MemoryEntryType::Procedural.to_string(), "PROCEDURAL");
        assert_eq!(MemoryEntryType::Vector.to_string(), "VECTOR");
        assert_eq!(MemoryEntryType::Custom.to_string(), "CUSTOM");
    }

    #[test]
    fn memory_entry_type_unknown_defaults_to_episodic() {
        assert_eq!(MemoryEntryType::from_proto(999), MemoryEntryType::Episodic);
    }

    #[test]
    fn memory_entry_expiry_helpers() {
        let entry = MemoryEntry {
            expires_at: 0,
            ..Default::default()
        };
        assert!(!entry.has_expiry());
        assert!(!entry.is_expired(1_700_000_000));

        let entry = MemoryEntry {
            expires_at: 1_700_003_600,
            ..Default::default()
        };
        assert!(entry.has_expiry());
        assert!(!entry.is_expired(1_700_000_000));
        assert!(entry.is_expired(1_700_003_600));
        assert!(entry.is_expired(1_700_010_000));
    }

    #[test]
    fn memory_entry_roundtrip() {
        let entry = MemoryEntry {
            agent_hash: "agent-abc".into(),
            key: "strategy/v1".into(),
            value: vec![1, 2, 3, 4],
            entry_type: MemoryEntryType::Semantic,
            timestamp: 1_700_000_000,
            expires_at: 1_700_003_600,
            version: "1.0".into(),
            deposit_amount: 1_000,
        };
        let proto: proto::MemoryEntry = entry.clone().into();
        let back: MemoryEntry = proto.into();
        assert_eq!(entry, back);
    }

    #[test]
    fn vector_embedding_roundtrip() {
        let emb = VectorEmbedding {
            agent_hash: "agent-abc".into(),
            key: "trade-pattern".into(),
            vector: vec![0.1, 0.2, 0.3, 0.4],
            timestamp: 1_700_000_000,
        };
        assert_eq!(emb.dimension(), 4);

        let proto: proto::VectorEmbedding = emb.clone().into();
        let back: VectorEmbedding = proto.into();
        assert_eq!(emb, back);
    }

    #[test]
    fn memory_root_roundtrip() {
        let root = MemoryRoot {
            agent_hash: "agent-abc".into(),
            root_hash: "deadbeef".into(),
            last_updated: 1_700_000_000,
            entry_count: 42,
            total_size_bytes: 1_048_576,
        };
        let proto: proto::MemoryRoot = root.clone().into();
        let back: MemoryRoot = proto.into();
        assert_eq!(root, back);
    }

    #[test]
    fn memory_snapshot_roundtrip() {
        let snap = MemorySnapshot {
            agent_hash: "agent-abc".into(),
            root_hash: "cafebabe".into(),
            snapshot_timestamp: 1_700_000_000,
            entry_count: 100,
            summary: "Trade strategy knowledge base".into(),
        };
        let proto: proto::MemorySnapshot = snap.clone().into();
        let back: MemorySnapshot = proto.into();
        assert_eq!(snap, back);
    }

    #[test]
    fn params_defaults() {
        let params = Params::default();
        assert_eq!(params.max_memory_per_agent_bytes, 0);
        assert_eq!(params.default_entry_ttl_seconds, 0);
        assert!(params.enable_vector_search);
        assert_eq!(params.max_entries_per_agent, 0);
        assert_eq!(params.default_vector_dimension, 512);
        assert!(!params.enable_auto_pruning);
    }

    #[test]
    fn params_roundtrip() {
        let params = Params {
            max_memory_per_agent_bytes: 10_000_000,
            default_entry_ttl_seconds: 86400,
            enable_vector_search: false,
            max_entries_per_agent: 1000,
            default_vector_dimension: 768,
            enable_auto_pruning: true,
        };
        let proto: proto::Params = params.clone().into();
        let back: Params = proto.into();
        assert_eq!(params, back);
    }
}
