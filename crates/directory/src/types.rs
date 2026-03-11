//! Domain types for the Directory module.
//!
//! These are clean, idiomatic Rust representations of the directory protobuf
//! messages. They provide type safety, ergonomic APIs, and full round-trip
//! conversion to/from protobuf while remaining strictly `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::directory::v1 as proto;

// ====================== VISIBILITY LEVEL ======================

/// Visibility level for an agent's directory profile.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(i32)]
pub enum VisibilityLevel {
    /// Visible to everyone.
    #[default]
    Public = 0,
    /// Visible only to the owner.
    OwnerOnly = 1,
    /// Visible only to evaluators.
    EvaluatorOnly = 2,
}

impl VisibilityLevel {
    /// Converts from the proto `i32` representation.
    pub fn from_proto(value: i32) -> Self {
        match value {
            1 => Self::OwnerOnly,
            2 => Self::EvaluatorOnly,
            _ => Self::Public,
        }
    }

    /// Converts to the proto `i32` representation.
    pub fn to_proto(self) -> i32 {
        self as i32
    }

    /// Returns `true` if this visibility is publicly accessible.
    pub fn is_public(self) -> bool {
        matches!(self, Self::Public)
    }
}

impl fmt::Display for VisibilityLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Public => f.write_str("PUBLIC"),
            Self::OwnerOnly => f.write_str("OWNER_ONLY"),
            Self::EvaluatorOnly => f.write_str("EVALUATOR_ONLY"),
        }
    }
}

// ====================== AGENT DIRECTORY PROFILE ======================

/// Rich directory profile for an agent.
///
/// Contains identity, reputation, performance metrics, and visibility settings
/// surfaced through the agent directory for queries and marketplace listings.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AgentDirectoryProfile {
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Human-readable display name.
    pub display_name: String,
    /// Short description of the agent.
    pub description: String,
    /// Comma-separated tags for discovery.
    pub tags: String,
    /// Current reputation score.
    pub reputation_score: u64,
    /// Milestone level achieved.
    pub milestone_level: u32,
    /// Whether the agent has reached immortal status.
    pub is_immortal: bool,
    /// Capability bitflags.
    pub capabilities: u64,
    /// Total 30-day trading volume in USD (scaled).
    pub total_volume_usd_30d: u64,
    /// Success rate in basis points (e.g. 9500 = 95.00%).
    pub success_rate_bps: u32,
    /// Summary of the latest intent submitted by the agent.
    pub latest_intent_summary: String,
    /// Health score of the agent's memory subsystem.
    pub memory_health_score: u32,
    /// Visibility level of this profile.
    pub visibility: VisibilityLevel,
    /// Timestamp of the last profile update.
    pub last_updated: u64,
}

impl AgentDirectoryProfile {
    /// Returns the success rate as a floating-point percentage (e.g. 95.00).
    pub fn success_rate_percent(&self) -> f64 {
        self.success_rate_bps as f64 / 100.0
    }
}

impl From<proto::AgentDirectoryProfile> for AgentDirectoryProfile {
    fn from(p: proto::AgentDirectoryProfile) -> Self {
        Self {
            agent_hash: p.agent_hash,
            display_name: p.display_name,
            description: p.description,
            tags: p.tags,
            reputation_score: p.reputation_score,
            milestone_level: p.milestone_level,
            is_immortal: p.is_immortal,
            capabilities: p.capabilities,
            total_volume_usd_30d: p.total_volume_usd_30d,
            success_rate_bps: p.success_rate_bps,
            latest_intent_summary: p.latest_intent_summary,
            memory_health_score: p.memory_health_score,
            visibility: VisibilityLevel::from_proto(p.visibility),
            last_updated: p.last_updated,
        }
    }
}

impl From<AgentDirectoryProfile> for proto::AgentDirectoryProfile {
    fn from(p: AgentDirectoryProfile) -> Self {
        Self {
            agent_hash: p.agent_hash,
            display_name: p.display_name,
            description: p.description,
            tags: p.tags,
            reputation_score: p.reputation_score,
            milestone_level: p.milestone_level,
            is_immortal: p.is_immortal,
            capabilities: p.capabilities,
            total_volume_usd_30d: p.total_volume_usd_30d,
            success_rate_bps: p.success_rate_bps,
            latest_intent_summary: p.latest_intent_summary,
            memory_health_score: p.memory_health_score,
            visibility: p.visibility.to_proto(),
            last_updated: p.last_updated,
            caip_id: String::new(),
            erc8004_view_hash: Vec::new(),
            a2a_ready: false,
            mcp_manifest_hash: Vec::new(),
            export_synced: false,
        }
    }
}

// ====================== DIRECTORY FILTER ======================

/// Filter criteria for directory queries.
///
/// All filter fields are optional — zero/empty values mean "no filter".
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DirectoryFilter {
    /// Minimum reputation score.
    pub min_reputation: u64,
    /// Minimum milestone level.
    pub min_milestone_level: u32,
    /// Comma-separated tag filter.
    pub tags: String,
    /// Free-text query for semantic/embedding search.
    pub semantic_query: String,
    /// Maximum number of results.
    pub limit: u32,
    /// Pagination offset.
    pub offset: u32,
}

impl From<proto::DirectoryFilter> for DirectoryFilter {
    fn from(p: proto::DirectoryFilter) -> Self {
        Self {
            min_reputation: p.min_reputation,
            min_milestone_level: p.min_milestone_level,
            tags: p.tags,
            semantic_query: p.semantic_query,
            limit: p.limit,
            offset: p.offset,
        }
    }
}

impl From<DirectoryFilter> for proto::DirectoryFilter {
    fn from(f: DirectoryFilter) -> Self {
        Self {
            min_reputation: f.min_reputation,
            min_milestone_level: f.min_milestone_level,
            tags: f.tags,
            semantic_query: f.semantic_query,
            limit: f.limit,
            offset: f.offset,
        }
    }
}

// ====================== PARAMS ======================

/// Module parameters (governance-controlled).
///
/// Provides sensible defaults:
/// - `default_query_limit`: 50
/// - `profile_cache_ttl_seconds`: 300 (5 minutes)
/// - `enable_semantic_search`: true
/// - `max_subscriptions_per_agent`: 10
/// - `public_directory_enabled`: true
/// - `min_reputation_for_public`: 0
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    /// Default query result limit.
    pub default_query_limit: u32,
    /// Cache TTL for directory profiles in seconds.
    pub profile_cache_ttl_seconds: u64,
    /// Whether semantic/vector search is enabled.
    pub enable_semantic_search: bool,
    /// Maximum number of active subscriptions per agent.
    pub max_subscriptions_per_agent: u32,
    /// Whether public directory access is enabled by default.
    pub public_directory_enabled: bool,
    /// Minimum reputation required for public visibility.
    pub min_reputation_for_public: u64,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            default_query_limit: 50,
            profile_cache_ttl_seconds: 300,
            enable_semantic_search: true,
            max_subscriptions_per_agent: 10,
            public_directory_enabled: true,
            min_reputation_for_public: 0,
        }
    }
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            default_query_limit: p.default_query_limit,
            profile_cache_ttl_seconds: p.profile_cache_ttl_seconds,
            enable_semantic_search: p.enable_semantic_search,
            max_subscriptions_per_agent: p.max_subscriptions_per_agent,
            public_directory_enabled: p.public_directory_enabled,
            min_reputation_for_public: p.min_reputation_for_public,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            default_query_limit: p.default_query_limit,
            profile_cache_ttl_seconds: p.profile_cache_ttl_seconds,
            enable_semantic_search: p.enable_semantic_search,
            max_subscriptions_per_agent: p.max_subscriptions_per_agent,
            public_directory_enabled: p.public_directory_enabled,
            min_reputation_for_public: p.min_reputation_for_public,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn visibility_level_roundtrip() {
        for v in [
            VisibilityLevel::Public,
            VisibilityLevel::OwnerOnly,
            VisibilityLevel::EvaluatorOnly,
        ] {
            assert_eq!(VisibilityLevel::from_proto(v.to_proto()), v);
        }
    }

    #[test]
    fn visibility_level_display() {
        assert_eq!(VisibilityLevel::Public.to_string(), "PUBLIC");
        assert_eq!(VisibilityLevel::OwnerOnly.to_string(), "OWNER_ONLY");
        assert_eq!(VisibilityLevel::EvaluatorOnly.to_string(), "EVALUATOR_ONLY");
    }

    #[test]
    fn visibility_level_unknown_defaults_to_public() {
        assert_eq!(VisibilityLevel::from_proto(999), VisibilityLevel::Public);
    }

    #[test]
    fn visibility_is_public_helper() {
        assert!(VisibilityLevel::Public.is_public());
        assert!(!VisibilityLevel::OwnerOnly.is_public());
        assert!(!VisibilityLevel::EvaluatorOnly.is_public());
    }

    #[test]
    fn agent_directory_profile_roundtrip() {
        let profile = AgentDirectoryProfile {
            agent_hash: "agent-abc".into(),
            display_name: "AlphaBot".into(),
            description: "High-frequency trading agent".into(),
            tags: "hft,btc,eth".into(),
            reputation_score: 95_000,
            milestone_level: 5,
            is_immortal: false,
            capabilities: 0xFF,
            total_volume_usd_30d: 1_000_000,
            success_rate_bps: 9500,
            latest_intent_summary: "TWAP buy 10 BTC".into(),
            memory_health_score: 85,
            visibility: VisibilityLevel::Public,
            last_updated: 1_700_000_000,
        };
        let proto: proto::AgentDirectoryProfile = profile.clone().into();
        let back: AgentDirectoryProfile = proto.into();
        assert_eq!(profile, back);
    }

    #[test]
    fn agent_directory_profile_success_rate() {
        let profile = AgentDirectoryProfile {
            success_rate_bps: 9500,
            ..Default::default()
        };
        assert!((profile.success_rate_percent() - 95.0).abs() < f64::EPSILON);

        let profile = AgentDirectoryProfile {
            success_rate_bps: 10_000,
            ..Default::default()
        };
        assert!((profile.success_rate_percent() - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn directory_filter_roundtrip() {
        let filter = DirectoryFilter {
            min_reputation: 50_000,
            min_milestone_level: 3,
            tags: "hft,defi".into(),
            semantic_query: "profitable trading strategies".into(),
            limit: 20,
            offset: 10,
        };
        let proto: proto::DirectoryFilter = filter.clone().into();
        let back: DirectoryFilter = proto.into();
        assert_eq!(filter, back);
    }

    #[test]
    fn params_defaults() {
        let params = Params::default();
        assert_eq!(params.default_query_limit, 50);
        assert_eq!(params.profile_cache_ttl_seconds, 300);
        assert!(params.enable_semantic_search);
        assert_eq!(params.max_subscriptions_per_agent, 10);
        assert!(params.public_directory_enabled);
        assert_eq!(params.min_reputation_for_public, 0);
    }

    #[test]
    fn params_roundtrip() {
        let params = Params {
            default_query_limit: 100,
            profile_cache_ttl_seconds: 600,
            enable_semantic_search: false,
            max_subscriptions_per_agent: 20,
            public_directory_enabled: false,
            min_reputation_for_public: 10_000,
        };
        let proto: proto::Params = params.clone().into();
        let back: Params = proto.into();
        assert_eq!(params, back);
    }
}
