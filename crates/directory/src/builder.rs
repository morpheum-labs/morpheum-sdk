//! Fluent builders for the Directory module.
//!
//! This module provides ergonomic, type-safe fluent builders for all directory
//! transaction operations (profile update, visibility update, parameter updates).
//! Each builder follows the classic Builder pattern and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{UpdateParamsRequest, UpdateProfileRequest, UpdateVisibilityRequest};
use crate::types::{Params, VisibilityLevel};

/// Fluent builder for updating an agent's directory profile.
///
/// # Example
/// ```rust,ignore
/// let request = UpdateProfileBuilder::new()
///     .agent_hash("agent-abc")
///     .display_name("AlphaBot")
///     .description("High-frequency trading agent")
///     .tags("hft,btc,eth")
///     .owner_signature(sig_bytes)
///     .build()?;
///
/// let any = request.to_any();
/// ```
#[derive(Default)]
pub struct UpdateProfileBuilder {
    agent_hash: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    tags: Option<String>,
    owner_signature: Option<Vec<u8>>,
}

impl UpdateProfileBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the agent hash (SHA-256 of the agent's DID).
    pub fn agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.agent_hash = Some(hash.into());
        self
    }

    /// Sets the display name.
    pub fn display_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = Some(name.into());
        self
    }

    /// Sets the profile description.
    pub fn description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Sets the profile tags (comma-separated).
    pub fn tags(mut self, tags: impl Into<String>) -> Self {
        self.tags = Some(tags.into());
        self
    }

    /// Sets the owner signature authorising this update.
    pub fn owner_signature(mut self, sig: Vec<u8>) -> Self {
        self.owner_signature = Some(sig);
        self
    }

    /// Builds the update-profile request, performing validation.
    pub fn build(self) -> Result<UpdateProfileRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for UpdateProfile")
        })?;

        let display_name = self.display_name.ok_or_else(|| {
            SdkError::invalid_input("display_name is required for UpdateProfile")
        })?;

        let owner_signature = self.owner_signature.ok_or_else(|| {
            SdkError::invalid_input("owner_signature is required for UpdateProfile")
        })?;

        let mut req = UpdateProfileRequest::new(agent_hash, display_name, owner_signature);

        if let Some(description) = self.description {
            req = req.with_description(description);
        }

        if let Some(tags) = self.tags {
            req = req.with_tags(tags);
        }

        Ok(req)
    }
}

/// Fluent builder for updating an agent's directory visibility.
///
/// # Example
/// ```rust,ignore
/// let request = UpdateVisibilityBuilder::new()
///     .agent_hash("agent-abc")
///     .new_visibility(VisibilityLevel::OwnerOnly)
///     .owner_signature(sig_bytes)
///     .build()?;
/// ```
#[derive(Default)]
pub struct UpdateVisibilityBuilder {
    agent_hash: Option<String>,
    new_visibility: Option<VisibilityLevel>,
    owner_signature: Option<Vec<u8>>,
}

impl UpdateVisibilityBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the agent hash.
    pub fn agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.agent_hash = Some(hash.into());
        self
    }

    /// Sets the new visibility level.
    pub fn new_visibility(mut self, visibility: VisibilityLevel) -> Self {
        self.new_visibility = Some(visibility);
        self
    }

    /// Sets the owner signature authorising this update.
    pub fn owner_signature(mut self, sig: Vec<u8>) -> Self {
        self.owner_signature = Some(sig);
        self
    }

    /// Builds the update-visibility request, performing validation.
    pub fn build(self) -> Result<UpdateVisibilityRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for UpdateVisibility")
        })?;

        let new_visibility = self.new_visibility.ok_or_else(|| {
            SdkError::invalid_input("new_visibility is required for UpdateVisibility")
        })?;

        let owner_signature = self.owner_signature.ok_or_else(|| {
            SdkError::invalid_input("owner_signature is required for UpdateVisibility")
        })?;

        Ok(UpdateVisibilityRequest::new(agent_hash, new_visibility, owner_signature))
    }
}

/// Fluent builder for updating directory module parameters (governance only).
///
/// # Example
/// ```rust,ignore
/// let request = UpdateParamsBuilder::new()
///     .params(Params {
///         default_query_limit: 100,
///         enable_semantic_search: true,
///         ..Default::default()
///     })
///     .gov_signature(sig_bytes)
///     .build()?;
/// ```
#[derive(Default)]
pub struct UpdateParamsBuilder {
    params: Option<Params>,
    gov_signature: Option<Vec<u8>>,
}

impl UpdateParamsBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the new module parameters.
    pub fn params(mut self, params: Params) -> Self {
        self.params = Some(params);
        self
    }

    /// Sets the governance signature authorising this update.
    pub fn gov_signature(mut self, sig: Vec<u8>) -> Self {
        self.gov_signature = Some(sig);
        self
    }

    /// Builds the update-params request, performing validation.
    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        let params = self.params.ok_or_else(|| {
            SdkError::invalid_input("params are required for UpdateParams")
        })?;

        let gov_signature = self.gov_signature.ok_or_else(|| {
            SdkError::invalid_input("gov_signature is required for UpdateParams")
        })?;

        Ok(UpdateParamsRequest::new(params, gov_signature))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn update_profile_builder_full_flow() {
        let request = UpdateProfileBuilder::new()
            .agent_hash("agent-abc")
            .display_name("AlphaBot")
            .description("High-frequency trading agent")
            .tags("hft,btc,eth")
            .owner_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.agent_hash, "agent-abc");
        assert_eq!(request.display_name, "AlphaBot");
        assert_eq!(request.description, "High-frequency trading agent");
        assert_eq!(request.tags, "hft,btc,eth");
    }

    #[test]
    fn update_profile_builder_minimal() {
        let request = UpdateProfileBuilder::new()
            .agent_hash("agent-abc")
            .display_name("Bot")
            .owner_signature(vec![0u8; 64])
            .build()
            .unwrap();

        // Optional fields default to empty strings
        assert!(request.description.is_empty());
        assert!(request.tags.is_empty());
    }

    #[test]
    fn update_profile_builder_validation() {
        // Missing all fields
        let result = UpdateProfileBuilder::new().build();
        assert!(result.is_err());

        // Missing display_name
        let result = UpdateProfileBuilder::new()
            .agent_hash("agent-abc")
            .owner_signature(vec![0u8; 64])
            .build();
        assert!(result.is_err());

        // Missing owner_signature
        let result = UpdateProfileBuilder::new()
            .agent_hash("agent-abc")
            .display_name("Bot")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn update_visibility_builder_full_flow() {
        let request = UpdateVisibilityBuilder::new()
            .agent_hash("agent-abc")
            .new_visibility(VisibilityLevel::EvaluatorOnly)
            .owner_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.agent_hash, "agent-abc");
        assert_eq!(request.new_visibility, VisibilityLevel::EvaluatorOnly);
    }

    #[test]
    fn update_visibility_builder_validation() {
        // Missing all fields
        let result = UpdateVisibilityBuilder::new().build();
        assert!(result.is_err());

        // Missing new_visibility
        let result = UpdateVisibilityBuilder::new()
            .agent_hash("agent-abc")
            .owner_signature(vec![0u8; 64])
            .build();
        assert!(result.is_err());

        // Missing owner_signature
        let result = UpdateVisibilityBuilder::new()
            .agent_hash("agent-abc")
            .new_visibility(VisibilityLevel::OwnerOnly)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn update_params_builder_works() {
        let request = UpdateParamsBuilder::new()
            .params(Params {
                default_query_limit: 100,
                enable_semantic_search: true,
                ..Default::default()
            })
            .gov_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.params.default_query_limit, 100);
        assert!(request.params.enable_semantic_search);
        assert_eq!(request.params.profile_cache_ttl_seconds, 300); // default
    }

    #[test]
    fn update_params_builder_validation() {
        // Missing all fields
        let result = UpdateParamsBuilder::new().build();
        assert!(result.is_err());

        // Missing gov_signature
        let result = UpdateParamsBuilder::new()
            .params(Params::default())
            .build();
        assert!(result.is_err());
    }
}
