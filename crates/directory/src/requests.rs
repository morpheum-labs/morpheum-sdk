//! Request and response wrappers for the Directory module.
//!
//! These are clean, ergonomic Rust types that wrap the raw protobuf messages.
//! They provide type safety, validation, helper methods, and seamless conversion
//! to/from protobuf for use with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::directory::v1 as proto;

use crate::types::{AgentDirectoryProfile, DirectoryFilter, Params, VisibilityLevel};

// ====================== TRANSACTION REQUESTS ======================

/// Request to update an agent's directory profile.
///
/// The `owner_signature` must be signed by the agent's owner or a delegated VC
/// to authorise the profile change.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateProfileRequest {
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// Human-readable display name.
    pub display_name: String,
    /// Short description of the agent.
    pub description: String,
    /// Comma-separated tags for discovery.
    pub tags: String,
    /// Owner or delegated VC signature.
    pub owner_signature: Vec<u8>,
}

impl UpdateProfileRequest {
    /// Creates a new update-profile request with required fields.
    pub fn new(
        agent_hash: impl Into<String>,
        display_name: impl Into<String>,
        owner_signature: Vec<u8>,
    ) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            display_name: display_name.into(),
            description: String::new(),
            tags: String::new(),
            owner_signature,
        }
    }

    /// Sets the profile description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    /// Sets the profile tags (comma-separated).
    pub fn with_tags(mut self, tags: impl Into<String>) -> Self {
        self.tags = tags.into();
        self
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdateProfile = self.clone().into();
        ProtoAny {
            type_url: "/directory.v1.MsgUpdateProfile".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdateProfileRequest> for proto::MsgUpdateProfile {
    fn from(req: UpdateProfileRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            display_name: req.display_name,
            description: req.description,
            tags: req.tags,
            owner_signature: req.owner_signature,
        }
    }
}

/// Response from updating a directory profile.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateProfileResponse {
    pub success: bool,
    pub updated_at: u64,
    pub new_profile_hash: String,
}

impl From<proto::UpdateProfileResponse> for UpdateProfileResponse {
    fn from(p: proto::UpdateProfileResponse) -> Self {
        Self {
            success: p.success,
            updated_at: p.updated_at,
            new_profile_hash: p.new_profile_hash,
        }
    }
}

/// Request to update visibility settings for an agent's directory profile.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateVisibilityRequest {
    /// Agent hash (SHA-256 of the agent's DID).
    pub agent_hash: String,
    /// New visibility level.
    pub new_visibility: VisibilityLevel,
    /// Owner or delegated VC signature.
    pub owner_signature: Vec<u8>,
}

impl UpdateVisibilityRequest {
    /// Creates a new update-visibility request.
    pub fn new(
        agent_hash: impl Into<String>,
        new_visibility: VisibilityLevel,
        owner_signature: Vec<u8>,
    ) -> Self {
        Self {
            agent_hash: agent_hash.into(),
            new_visibility,
            owner_signature,
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdateVisibility = self.clone().into();
        ProtoAny {
            type_url: "/directory.v1.MsgUpdateVisibility".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdateVisibilityRequest> for proto::MsgUpdateVisibility {
    fn from(req: UpdateVisibilityRequest) -> Self {
        Self {
            agent_hash: req.agent_hash,
            new_visibility: req.new_visibility.to_proto(),
            owner_signature: req.owner_signature,
        }
    }
}

/// Response from updating visibility.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateVisibilityResponse {
    pub success: bool,
    pub updated_at: u64,
}

impl From<proto::UpdateVisibilityResponse> for UpdateVisibilityResponse {
    fn from(p: proto::UpdateVisibilityResponse) -> Self {
        Self {
            success: p.success,
            updated_at: p.updated_at,
        }
    }
}

/// Request to update module parameters (governance only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    /// New parameters.
    pub params: Params,
    /// Governance signature authorising this update.
    pub gov_signature: Vec<u8>,
}

impl UpdateParamsRequest {
    /// Creates a new update-params request.
    pub fn new(params: Params, gov_signature: Vec<u8>) -> Self {
        Self { params, gov_signature }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdateParams = self.clone().into();
        ProtoAny {
            type_url: "/directory.v1.MsgUpdateParams".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdateParamsRequest> for proto::MsgUpdateParams {
    fn from(req: UpdateParamsRequest) -> Self {
        Self {
            params: Some(req.params.into()),
            gov_signature: req.gov_signature,
        }
    }
}

/// Response from updating parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsResponse {
    pub success: bool,
}

impl From<proto::UpdateParamsResponse> for UpdateParamsResponse {
    fn from(p: proto::UpdateParamsResponse) -> Self {
        Self { success: p.success }
    }
}

// ====================== QUERY REQUESTS & RESPONSES ======================

/// Query a specific agent's directory profile.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDirectoryProfileRequest {
    pub agent_hash: String,
}

impl QueryDirectoryProfileRequest {
    pub fn new(agent_hash: impl Into<String>) -> Self {
        Self { agent_hash: agent_hash.into() }
    }
}

impl From<QueryDirectoryProfileRequest> for proto::QueryDirectoryProfileRequest {
    fn from(req: QueryDirectoryProfileRequest) -> Self {
        Self { agent_hash: req.agent_hash }
    }
}

/// Response containing an agent's directory profile (or indicating not found).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDirectoryProfileResponse {
    pub profile: Option<AgentDirectoryProfile>,
    pub found: bool,
}

impl From<proto::QueryDirectoryProfileResponse> for QueryDirectoryProfileResponse {
    fn from(p: proto::QueryDirectoryProfileResponse) -> Self {
        Self {
            profile: p.profile.map(Into::into),
            found: p.found,
        }
    }
}

/// Query multiple directory profiles with filter (paginated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDirectoryProfilesRequest {
    /// Optional filter criteria.
    pub filter: Option<DirectoryFilter>,
    /// Maximum number of results.
    pub limit: u32,
    /// Pagination offset.
    pub offset: u32,
}

impl QueryDirectoryProfilesRequest {
    /// Creates a new request with the given pagination parameters.
    pub fn new(limit: u32, offset: u32) -> Self {
        Self {
            filter: None,
            limit,
            offset,
        }
    }

    /// Sets the filter for the query.
    pub fn with_filter(mut self, filter: DirectoryFilter) -> Self {
        self.filter = Some(filter);
        self
    }
}

impl From<QueryDirectoryProfilesRequest> for proto::QueryDirectoryProfilesRequest {
    fn from(req: QueryDirectoryProfilesRequest) -> Self {
        Self {
            filter: req.filter.map(Into::into),
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Response containing paginated directory profiles.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryDirectoryProfilesResponse {
    pub profiles: Vec<AgentDirectoryProfile>,
    pub total_count: u32,
}

impl From<proto::QueryDirectoryProfilesResponse> for QueryDirectoryProfilesResponse {
    fn from(p: proto::QueryDirectoryProfilesResponse) -> Self {
        Self {
            profiles: p.profiles.into_iter().map(Into::into).collect(),
            total_count: p.total_count,
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
    fn update_profile_request_to_any() {
        let req = UpdateProfileRequest::new("agent-abc", "AlphaBot", vec![0u8; 64])
            .with_description("High-frequency trading agent")
            .with_tags("hft,btc,eth");

        let any = req.to_any();
        assert_eq!(any.type_url, "/directory.v1.MsgUpdateProfile");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn update_visibility_request_to_any() {
        let req = UpdateVisibilityRequest::new(
            "agent-abc",
            VisibilityLevel::OwnerOnly,
            vec![0u8; 64],
        );

        let any = req.to_any();
        assert_eq!(any.type_url, "/directory.v1.MsgUpdateVisibility");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn update_params_request_to_any() {
        let req = UpdateParamsRequest::new(Params::default(), vec![0u8; 64]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/directory.v1.MsgUpdateParams");
    }

    #[test]
    fn update_profile_response_conversion() {
        let proto_res = proto::UpdateProfileResponse {
            success: true,
            updated_at: 1_700_000_000,
            new_profile_hash: "hash123".into(),
        };
        let res: UpdateProfileResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.updated_at, 1_700_000_000);
        assert_eq!(res.new_profile_hash, "hash123");
    }

    #[test]
    fn update_visibility_response_conversion() {
        let proto_res = proto::UpdateVisibilityResponse {
            success: true,
            updated_at: 1_700_001_000,
        };
        let res: UpdateVisibilityResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.updated_at, 1_700_001_000);
    }

    #[test]
    fn query_directory_profile_response_conversion() {
        let proto_res = proto::QueryDirectoryProfileResponse {
            profile: Some(proto::AgentDirectoryProfile {
                agent_hash: "agent-abc".into(),
                display_name: "AlphaBot".into(),
                description: "Trading agent".into(),
                tags: "hft".into(),
                reputation_score: 90_000,
                milestone_level: 3,
                is_immortal: false,
                capabilities: 0xFF,
                total_volume_usd_30d: 500_000,
                success_rate_bps: 9200,
                latest_intent_summary: "TWAP buy".into(),
                memory_health_score: 80,
                visibility: 0, // PUBLIC
                last_updated: 1_700_000_000,
            }),
            found: true,
        };
        let res: QueryDirectoryProfileResponse = proto_res.into();
        assert!(res.found);
        let profile = res.profile.unwrap();
        assert_eq!(profile.agent_hash, "agent-abc");
        assert_eq!(profile.display_name, "AlphaBot");
        assert_eq!(profile.visibility, VisibilityLevel::Public);
    }

    #[test]
    fn query_directory_profiles_response_conversion() {
        let proto_res = proto::QueryDirectoryProfilesResponse {
            profiles: vec![Default::default(), Default::default()],
            total_count: 2,
        };
        let res: QueryDirectoryProfilesResponse = proto_res.into();
        assert_eq!(res.total_count, 2);
        assert_eq!(res.profiles.len(), 2);
    }

    #[test]
    fn query_directory_profiles_request_with_filter() {
        let filter = DirectoryFilter {
            min_reputation: 50_000,
            min_milestone_level: 3,
            tags: "hft,defi".into(),
            ..Default::default()
        };
        let req = QueryDirectoryProfilesRequest::new(20, 0).with_filter(filter);

        let proto_req: proto::QueryDirectoryProfilesRequest = req.into();
        assert!(proto_req.filter.is_some());
        assert_eq!(proto_req.limit, 20);
        assert_eq!(proto_req.offset, 0);
        let f = proto_req.filter.unwrap();
        assert_eq!(f.min_reputation, 50_000);
        assert_eq!(f.tags, "hft,defi");
    }

    #[test]
    fn query_params_response_conversion() {
        let proto_res = proto::QueryParamsResponse {
            params: Some(proto::Params {
                default_query_limit: 100,
                profile_cache_ttl_seconds: 600,
                enable_semantic_search: false,
                max_subscriptions_per_agent: 20,
                public_directory_enabled: false,
                min_reputation_for_public: 10_000,
            }),
        };
        let res: QueryParamsResponse = proto_res.into();
        let p = res.params.unwrap();
        assert_eq!(p.default_query_limit, 100);
        assert_eq!(p.profile_cache_ttl_seconds, 600);
        assert!(!p.enable_semantic_search);
    }
}
