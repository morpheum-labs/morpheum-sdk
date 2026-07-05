//! Fluent builders for the Reputation module.
//!
//! This module provides ergonomic, type-safe fluent builders for the supported
//! reputation transaction operations (milestone forcing and params updates). Each
//! builder follows the classic Builder pattern and returns the
//! corresponding request type from `requests.rs` for seamless integration
//! with `TxBuilder`.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{ForceMilestoneRequest, UpdateParamsRequest};
use crate::types::Params;

/// Fluent builder for forcing a milestone (governance only).
///
/// # Example
/// ```rust,ignore
/// let request = ForceMilestoneBuilder::new()
///     .agent_hash("abc123def456")
///     .milestone_level(5)
///     .build()?;
/// ```
#[derive(Default)]
pub struct ForceMilestoneBuilder {
    agent_hash: Option<String>,
    milestone_level: Option<u32>,
}

impl ForceMilestoneBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the target agent hash.
    pub fn agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.agent_hash = Some(hash.into());
        self
    }

    /// Sets the milestone level to force.
    pub fn milestone_level(mut self, level: u32) -> Self {
        self.milestone_level = Some(level);
        self
    }

    /// Builds the force-milestone request, performing validation.
    pub fn build(self) -> Result<ForceMilestoneRequest, SdkError> {
        let agent_hash = self
            .agent_hash
            .ok_or_else(|| SdkError::invalid_input("agent_hash is required for force milestone"))?;

        let milestone_level = self.milestone_level.ok_or_else(|| {
            SdkError::invalid_input("milestone_level is required for force milestone")
        })?;

        Ok(ForceMilestoneRequest::new(agent_hash, milestone_level))
    }
}

/// Fluent builder for governance parameter updates.
#[derive(Default)]
pub struct UpdateParamsBuilder {
    authority: Option<String>,
    params: Option<Params>,
}

impl UpdateParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn authority(mut self, authority: impl Into<String>) -> Self {
        self.authority = Some(authority.into());
        self
    }

    pub fn params(mut self, params: Params) -> Self {
        self.params = Some(params);
        self
    }

    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        let authority = self
            .authority
            .ok_or_else(|| SdkError::invalid_input("authority is required for params update"))?;
        let params = self
            .params
            .ok_or_else(|| SdkError::invalid_input("params are required for params update"))?;

        Ok(UpdateParamsRequest::new(authority, params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn force_milestone_builder_works() {
        let request = ForceMilestoneBuilder::new()
            .agent_hash("test-agent")
            .milestone_level(5)
            .build()
            .unwrap();

        assert_eq!(request.agent_hash, "test-agent");
        assert_eq!(request.milestone_level, 5);
    }

    #[test]
    fn force_milestone_builder_validation() {
        let result = ForceMilestoneBuilder::new().build();
        assert!(result.is_err());
    }

    #[test]
    fn update_params_builder_works() {
        let request = UpdateParamsBuilder::new()
            .authority("morpheum1gov")
            .params(Params {
                daily_recovery_cap_bps: 3000,
                min_reputation_to_register: 0,
                enable_reputation_priority: true,
                slashing_multiplier: 100,
                milestone_thresholds: vec![10_000, 50_000],
                milestone_rewards: vec![500, 1_000],
                perk_multiplier_bps: 1500,
                economics: None,
                authorized_recovery_signers: vec![],
            })
            .build()
            .unwrap();

        assert_eq!(request.authority, "morpheum1gov");
    }

    #[test]
    fn update_params_builder_validation() {
        let result = UpdateParamsBuilder::new().build();
        assert!(result.is_err());
    }
}
