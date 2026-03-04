//! Domain types for the Identity (Agent Identity) module.
//!
//! These are clean, idiomatic Rust representations of the identity protobuf
//! messages. They provide type safety, ergonomic APIs, and full round-trip
//! conversion to/from protobuf while remaining strictly `no_std` compatible.

use alloc::string::String;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::identity::v1 as proto;

// ====================== AGENT ID ======================

/// Canonical agent identifier — a DID string paired with its deterministic hash.
///
/// The `hash` is the SHA-256 digest of the DID (hex-encoded), used for
/// sharding and indexing on-chain. The `did` is the human-readable W3C DID
/// string (e.g. `"did:agent:abc123…"`).
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentId {
    /// Full DID string (e.g. `"did:agent:abc123…"`).
    pub did: String,
    /// Deterministic hash of the DID (hex-encoded, used for sharding & indexing).
    pub hash: String,
}

impl AgentId {
    /// Creates a new `AgentId`.
    pub fn new(did: impl Into<String>, hash: impl Into<String>) -> Self {
        Self {
            did: did.into(),
            hash: hash.into(),
        }
    }

    /// Returns `true` if both fields are empty (sentinel for "not set").
    pub fn is_empty(&self) -> bool {
        self.did.is_empty() && self.hash.is_empty()
    }
}

impl fmt::Display for AgentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.did.is_empty() {
            f.write_str(&self.hash)
        } else {
            f.write_str(&self.did)
        }
    }
}

impl From<proto::AgentId> for AgentId {
    fn from(p: proto::AgentId) -> Self {
        Self {
            did: p.did,
            hash: p.hash,
        }
    }
}

impl From<AgentId> for proto::AgentId {
    fn from(a: AgentId) -> Self {
        Self {
            did: a.did,
            hash: a.hash,
        }
    }
}

// ====================== AGENT STATUS ======================

/// Status of an agent on-chain.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum AgentStatus {
    /// Agent is active and operational.
    #[default]
    Active = 0,
    /// Agent is temporarily suspended.
    Suspended = 1,
    /// Agent has been slashed (penalty applied).
    Slashed = 2,
    /// Agent has been permanently burned.
    Burned = 3,
}

impl AgentStatus {
    /// Converts from the proto `i32` representation.
    ///
    /// Unknown values default to `Active` (proto3 zero-value semantics).
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::Suspended,
            2 => Self::Slashed,
            3 => Self::Burned,
            _ => Self::Active,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }

    /// Returns `true` if the agent can perform operations.
    pub fn is_active(self) -> bool {
        matches!(self, Self::Active)
    }
}

impl fmt::Display for AgentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Active => f.write_str("ACTIVE"),
            Self::Suspended => f.write_str("SUSPENDED"),
            Self::Slashed => f.write_str("SLASHED"),
            Self::Burned => f.write_str("BURNED"),
        }
    }
}

// ====================== CAPABILITIES ======================

/// Well-known capability bitflags for agent permissions.
///
/// These can be combined via bitwise OR:
/// ```rust,ignore
/// use morpheum_sdk_identity::Capability;
/// let caps = Capability::TRADE | Capability::EVALUATE;
/// assert!(Capability::has(caps, Capability::TRADE));
/// ```
pub struct Capability;

impl Capability {
    /// Permission to execute trades.
    pub const TRADE: u64 = 1 << 0;
    /// Permission to evaluate markets/signals.
    pub const EVALUATE: u64 = 1 << 1;
    /// Permission to manage sub-agents.
    pub const MANAGE: u64 = 1 << 2;
    /// Permission to access persistent memory.
    pub const MEMORY: u64 = 1 << 3;

    /// Returns `true` if `flags` includes all bits of `capability`.
    pub fn has(flags: u64, capability: u64) -> bool {
        flags & capability == capability
    }
}

// ====================== AGENT IDENTITY ======================

/// Main agent identity record stored on-chain.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentIdentity {
    /// Canonical agent identifier (DID + hash).
    pub agent_id: AgentId,
    /// Current owner's agent identifier.
    pub owner_did: AgentId,
    /// Capability bitflags.
    pub capabilities: u64,
    /// Current agent status.
    pub status: AgentStatus,
    /// Hash of the full `AgentMetadataCard`.
    pub metadata_card_hash: String,
    /// Link to Persistent Memory root.
    pub memory_root_hash: String,
    /// Block timestamp when the agent was registered.
    pub registered_at: u64,
    /// Block timestamp of the last state change.
    pub last_updated: u64,
    /// Current milestone level (progression).
    pub milestone_level: u32,
    /// Whether the agent has achieved permanent Immortal status.
    pub is_immortal: bool,
    /// Reference to the base account (where nonce state lives).
    pub base_account_address: String,
}

impl From<proto::AgentIdentity> for AgentIdentity {
    fn from(p: proto::AgentIdentity) -> Self {
        Self {
            agent_id: p.agent_id.map(AgentId::from).unwrap_or_default(),
            owner_did: p.owner_did.map(AgentId::from).unwrap_or_default(),
            capabilities: p.capabilities,
            status: AgentStatus::from_proto(p.status),
            metadata_card_hash: p.metadata_card_hash,
            memory_root_hash: p.memory_root_hash,
            registered_at: p.registered_at,
            last_updated: p.last_updated,
            milestone_level: p.milestone_level,
            is_immortal: p.is_immortal,
            base_account_address: p.base_account_address,
        }
    }
}

impl From<AgentIdentity> for proto::AgentIdentity {
    fn from(a: AgentIdentity) -> Self {
        Self {
            agent_id: Some(a.agent_id.into()),
            owner_did: Some(a.owner_did.into()),
            capabilities: a.capabilities,
            status: a.status.to_proto(),
            metadata_card_hash: a.metadata_card_hash,
            memory_root_hash: a.memory_root_hash,
            registered_at: a.registered_at,
            last_updated: a.last_updated,
            milestone_level: a.milestone_level,
            is_immortal: a.is_immortal,
            base_account_address: a.base_account_address,
        }
    }
}

// ====================== METADATA ======================

/// Rich metadata card for an agent (stored separately, referenced by hash).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentMetadataCard {
    /// Canonical agent identifier (DID + hash).
    pub agent_id: AgentId,
    /// Owner's agent identifier.
    pub owner_did: AgentId,
    /// Human-readable display name.
    pub display_name: String,
    /// Free-form description.
    pub description: String,
    /// Comma-separated tags for discovery.
    pub tags: String,
    /// Semantic version string (e.g. `"1.0.0"`).
    pub version: String,
    /// Capability bitflags at the time of card creation.
    pub capabilities: u64,
    /// Link to Persistent Memory root.
    pub memory_root_hash: String,
    /// Block timestamp when registered.
    pub registered_at: u64,
}

impl From<proto::AgentMetadataCard> for AgentMetadataCard {
    fn from(p: proto::AgentMetadataCard) -> Self {
        Self {
            agent_id: p.agent_id.map(AgentId::from).unwrap_or_default(),
            owner_did: p.owner_did.map(AgentId::from).unwrap_or_default(),
            display_name: p.display_name,
            description: p.description,
            tags: p.tags,
            version: p.version,
            capabilities: p.capabilities,
            memory_root_hash: p.memory_root_hash,
            registered_at: p.registered_at,
        }
    }
}

impl From<AgentMetadataCard> for proto::AgentMetadataCard {
    fn from(m: AgentMetadataCard) -> Self {
        Self {
            agent_id: Some(m.agent_id.into()),
            owner_did: Some(m.owner_did.into()),
            display_name: m.display_name,
            description: m.description,
            tags: m.tags,
            version: m.version,
            capabilities: m.capabilities,
            memory_root_hash: m.memory_root_hash,
            registered_at: m.registered_at,
        }
    }
}

/// Input version of the metadata card for registration / updates.
///
/// Contains only the user-supplied fields — the keeper fills in
/// `agent_id`, `owner_did`, `memory_root_hash`, and `registered_at`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentMetadataCardInput {
    /// Human-readable display name.
    pub display_name: String,
    /// Free-form description.
    pub description: String,
    /// Comma-separated tags for discovery.
    pub tags: String,
    /// Semantic version string.
    pub version: String,
    /// Capability bitflags.
    pub capabilities: u64,
}

impl From<proto::AgentMetadataCardInput> for AgentMetadataCardInput {
    fn from(p: proto::AgentMetadataCardInput) -> Self {
        Self {
            display_name: p.display_name,
            description: p.description,
            tags: p.tags,
            version: p.version,
            capabilities: p.capabilities,
        }
    }
}

impl From<AgentMetadataCardInput> for proto::AgentMetadataCardInput {
    fn from(m: AgentMetadataCardInput) -> Self {
        Self {
            display_name: m.display_name,
            description: m.description,
            tags: m.tags,
            version: m.version,
            capabilities: m.capabilities,
        }
    }
}

// ====================== PARAMS ======================

/// Module parameters (governance-controlled).
///
/// Provides sensible defaults suitable for most deployments:
/// - `default_capabilities`: 0 (no capabilities by default)
/// - `registration_enabled`: true
/// - `min_reputation_to_register`: 0
/// - `allow_ownership_transfer`: true
///
/// Override only the fields you need:
/// ```rust,ignore
/// let params = Params {
///     registration_enabled: false,
///     ..Default::default()
/// };
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    /// Default capabilities bitflags for newly registered agents.
    pub default_capabilities: u64,
    /// Whether new agent registration is open.
    pub registration_enabled: bool,
    /// Minimum reputation required to register a new agent (0 = no minimum).
    pub min_reputation_to_register: u64,
    /// Whether agents can transfer ownership freely.
    pub allow_ownership_transfer: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            default_capabilities: 0,
            registration_enabled: true,
            min_reputation_to_register: 0,
            allow_ownership_transfer: true,
        }
    }
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            default_capabilities: p.default_capabilities,
            registration_enabled: p.registration_enabled,
            min_reputation_to_register: p.min_reputation_to_register,
            allow_ownership_transfer: p.allow_ownership_transfer,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            default_capabilities: p.default_capabilities,
            registration_enabled: p.registration_enabled,
            min_reputation_to_register: p.min_reputation_to_register,
            allow_ownership_transfer: p.allow_ownership_transfer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn agent_id_roundtrip() {
        let id = AgentId::new("did:agent:test123", "abcdef0123456789");
        let proto: proto::AgentId = id.clone().into();
        let back: AgentId = proto.into();
        assert_eq!(id, back);
    }

    #[test]
    fn agent_id_display() {
        let with_did = AgentId::new("did:agent:abc", "hash123");
        assert_eq!(with_did.to_string(), "did:agent:abc");

        let hash_only = AgentId::new("", "hash123");
        assert_eq!(hash_only.to_string(), "hash123");
    }

    #[test]
    fn agent_id_is_empty() {
        assert!(AgentId::default().is_empty());
        assert!(!AgentId::new("did:agent:x", "").is_empty());
    }

    #[test]
    fn agent_status_roundtrip() {
        for status in [
            AgentStatus::Active,
            AgentStatus::Suspended,
            AgentStatus::Slashed,
            AgentStatus::Burned,
        ] {
            assert_eq!(AgentStatus::from_proto(status.to_proto()), status);
        }
    }

    #[test]
    fn agent_status_unknown_defaults_to_active() {
        assert_eq!(AgentStatus::from_proto(99), AgentStatus::Active);
    }

    #[test]
    fn agent_status_is_active() {
        assert!(AgentStatus::Active.is_active());
        assert!(!AgentStatus::Suspended.is_active());
        assert!(!AgentStatus::Slashed.is_active());
        assert!(!AgentStatus::Burned.is_active());
    }

    #[test]
    fn capability_flags() {
        let caps = Capability::TRADE | Capability::EVALUATE;
        assert!(Capability::has(caps, Capability::TRADE));
        assert!(Capability::has(caps, Capability::EVALUATE));
        assert!(!Capability::has(caps, Capability::MANAGE));
    }

    #[test]
    fn agent_identity_roundtrip() {
        let identity = AgentIdentity {
            agent_id: AgentId::new("did:agent:test", "hash123"),
            owner_did: AgentId::new("did:agent:owner", "ownerhash"),
            capabilities: Capability::TRADE | Capability::EVALUATE,
            status: AgentStatus::Active,
            metadata_card_hash: "card_hash".into(),
            memory_root_hash: "mem_hash".into(),
            registered_at: 1_700_000_000,
            last_updated: 1_700_000_100,
            milestone_level: 5,
            is_immortal: false,
            base_account_address: "base_addr".into(),
        };

        let proto: proto::AgentIdentity = identity.clone().into();
        let back: AgentIdentity = proto.into();
        assert_eq!(identity, back);
    }

    #[test]
    fn params_defaults() {
        let params = Params::default();
        assert!(params.registration_enabled);
        assert!(params.allow_ownership_transfer);
        assert_eq!(params.default_capabilities, 0);
        assert_eq!(params.min_reputation_to_register, 0);
    }

    #[test]
    fn params_roundtrip() {
        let params = Params {
            default_capabilities: Capability::TRADE,
            registration_enabled: false,
            min_reputation_to_register: 100,
            allow_ownership_transfer: false,
        };
        let proto: proto::Params = params.clone().into();
        let back: Params = proto.into();
        assert_eq!(params, back);
    }

    #[test]
    fn metadata_input_roundtrip() {
        let input = AgentMetadataCardInput {
            display_name: "TestAgent".into(),
            description: "A test agent".into(),
            tags: "test,agent".into(),
            version: "1.0.0".into(),
            capabilities: Capability::TRADE,
        };
        let proto: proto::AgentMetadataCardInput = input.clone().into();
        let back: AgentMetadataCardInput = proto.into();
        assert_eq!(input, back);
    }
}
