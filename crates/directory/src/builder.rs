//! Fluent builders for the Directory module.
//!
//! This module provides ergonomic, type-safe fluent builders for all directory
//! transaction operations (profile update, visibility update, parameter updates).
//! Each builder follows the classic Builder pattern and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{UpdateProfileRequest, UpdateVisibilityRequest};
use crate::types::VisibilityLevel;

/// Fluent builder for updating an agent's directory profile.
///
/// # Example
/// ```rust,ignore
/// let request = UpdateProfileBuilder::new()
///     .agent_hash("agent-abc")
///     .display_name("AlphaBot")
///     .description("High-frequency trading agent")
///     .tags("hft,btc,eth")
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

    /// Builds the update-profile request, performing validation.
    pub fn build(self) -> Result<UpdateProfileRequest, SdkError> {
        let agent_hash = self
            .agent_hash
            .ok_or_else(|| SdkError::invalid_input("agent_hash is required for UpdateProfile"))?;

        let display_name = self
            .display_name
            .ok_or_else(|| SdkError::invalid_input("display_name is required for UpdateProfile"))?;

        let mut req = UpdateProfileRequest::new(agent_hash, display_name);

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
///     .build()?;
/// ```
#[derive(Default)]
pub struct UpdateVisibilityBuilder {
    agent_hash: Option<String>,
    new_visibility: Option<VisibilityLevel>,
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

    /// Builds the update-visibility request, performing validation.
    pub fn build(self) -> Result<UpdateVisibilityRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for UpdateVisibility")
        })?;

        let new_visibility = self.new_visibility.ok_or_else(|| {
            SdkError::invalid_input("new_visibility is required for UpdateVisibility")
        })?;

        Ok(UpdateVisibilityRequest::new(agent_hash, new_visibility))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_profile_builder_full_flow() {
        let request = UpdateProfileBuilder::new()
            .agent_hash("agent-abc")
            .display_name("AlphaBot")
            .description("High-frequency trading agent")
            .tags("hft,btc,eth")
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
        let result = UpdateProfileBuilder::new().agent_hash("agent-abc").build();
        assert!(result.is_err());
    }

    #[test]
    fn update_visibility_builder_full_flow() {
        let request = UpdateVisibilityBuilder::new()
            .agent_hash("agent-abc")
            .new_visibility(VisibilityLevel::EvaluatorOnly)
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
            .build();
        assert!(result.is_err());
    }
}
