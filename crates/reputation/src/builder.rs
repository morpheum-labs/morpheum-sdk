//! Fluent builders for the Reputation module.
//!
//! This module provides ergonomic, type-safe fluent builders for all reputation
//! transaction operations (penalties, recoveries, milestone forcing, parameter
//! updates). Each builder follows the classic Builder pattern and returns the
//! corresponding request type from `requests.rs` for seamless integration
//! with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    ApplyPenaltyRequest,
    ApplyRecoveryRequest,
    ForceMilestoneRequest,
    UpdateParamsRequest,
};
use crate::types::{Params, RecoveryActionType};

/// Fluent builder for applying a penalty to an agent's reputation.
///
/// # Example
/// ```rust,ignore
/// let request = ApplyPenaltyBuilder::new()
///     .agent_hash("abc123def456")
///     .base_amount(5000)
///     .multiplier(200)
///     .reason("front-running detected")
///     .signer(gov_key_bytes)
///     .build()?;
///
/// let any = request.to_any();
/// ```
#[derive(Default)]
pub struct ApplyPenaltyBuilder {
    agent_hash: Option<String>,
    base_amount: Option<u64>,
    multiplier: Option<u32>,
    reason: Option<String>,
    signer: Option<Vec<u8>>,
}

impl ApplyPenaltyBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the target agent hash.
    pub fn agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.agent_hash = Some(hash.into());
        self
    }

    /// Sets the base penalty amount (before multiplier).
    pub fn base_amount(mut self, amount: u64) -> Self {
        self.base_amount = Some(amount);
        self
    }

    /// Sets the multiplier (100 = 1.0×, 200 = 2.0×).
    pub fn multiplier(mut self, multiplier: u32) -> Self {
        self.multiplier = Some(multiplier);
        self
    }

    /// Sets the human-readable reason for the penalty.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Sets the signer bytes (governance or module authority).
    pub fn signer(mut self, signer: Vec<u8>) -> Self {
        self.signer = Some(signer);
        self
    }

    /// Builds the penalty request, performing validation.
    pub fn build(self) -> Result<ApplyPenaltyRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for penalty")
        })?;

        let base_amount = self.base_amount.ok_or_else(|| {
            SdkError::invalid_input("base_amount is required for penalty")
        })?;

        let multiplier = self.multiplier.ok_or_else(|| {
            SdkError::invalid_input("multiplier is required for penalty")
        })?;

        let reason = self.reason.ok_or_else(|| {
            SdkError::invalid_input("reason is required for penalty")
        })?;

        let signer = self.signer.ok_or_else(|| {
            SdkError::invalid_input("signer is required for penalty")
        })?;

        Ok(ApplyPenaltyRequest::new(agent_hash, base_amount, multiplier, reason, signer))
    }
}

/// Fluent builder for applying a recovery / positive boost.
///
/// # Example
/// ```rust,ignore
/// let request = ApplyRecoveryBuilder::new()
///     .agent_hash("abc123def456")
///     .action_type(RecoveryActionType::TradeFill)
///     .amount(1000)
///     .reason("successful trade fill")
///     .build()?;
/// ```
#[derive(Default)]
pub struct ApplyRecoveryBuilder {
    agent_hash: Option<String>,
    action_type: Option<RecoveryActionType>,
    amount: Option<u64>,
    reason: Option<String>,
}

impl ApplyRecoveryBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the target agent hash.
    pub fn agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.agent_hash = Some(hash.into());
        self
    }

    /// Sets the type of recovery action.
    pub fn action_type(mut self, action_type: RecoveryActionType) -> Self {
        self.action_type = Some(action_type);
        self
    }

    /// Sets the recovery amount.
    pub fn amount(mut self, amount: u64) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Sets the human-readable reason.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Builds the recovery request, performing validation.
    pub fn build(self) -> Result<ApplyRecoveryRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for recovery")
        })?;

        let action_type = self.action_type.ok_or_else(|| {
            SdkError::invalid_input("action_type is required for recovery")
        })?;

        let amount = self.amount.ok_or_else(|| {
            SdkError::invalid_input("amount is required for recovery")
        })?;

        let reason = self.reason.ok_or_else(|| {
            SdkError::invalid_input("reason is required for recovery")
        })?;

        Ok(ApplyRecoveryRequest::new(agent_hash, action_type, amount, reason))
    }
}

/// Fluent builder for forcing a milestone (governance only).
///
/// # Example
/// ```rust,ignore
/// let request = ForceMilestoneBuilder::new()
///     .agent_hash("abc123def456")
///     .milestone_level(5)
///     .gov_signature(sig_bytes)
///     .build()?;
/// ```
#[derive(Default)]
pub struct ForceMilestoneBuilder {
    agent_hash: Option<String>,
    milestone_level: Option<u32>,
    gov_signature: Option<Vec<u8>>,
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

    /// Sets the governance signature authorising this operation.
    pub fn gov_signature(mut self, sig: Vec<u8>) -> Self {
        self.gov_signature = Some(sig);
        self
    }

    /// Builds the force-milestone request, performing validation.
    pub fn build(self) -> Result<ForceMilestoneRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for force milestone")
        })?;

        let milestone_level = self.milestone_level.ok_or_else(|| {
            SdkError::invalid_input("milestone_level is required for force milestone")
        })?;

        let gov_signature = self.gov_signature.ok_or_else(|| {
            SdkError::invalid_input("gov_signature is required for force milestone")
        })?;

        Ok(ForceMilestoneRequest::new(agent_hash, milestone_level, gov_signature))
    }
}

/// Fluent builder for updating reputation module parameters (governance).
///
/// # Example
/// ```rust,ignore
/// let request = UpdateParamsBuilder::new()
///     .params(Params {
///         slashing_multiplier: 200,
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
    fn apply_penalty_builder_full_flow() {
        let request = ApplyPenaltyBuilder::new()
            .agent_hash("abc123")
            .base_amount(5000)
            .multiplier(200)
            .reason("front-running detected")
            .signer(vec![1u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.agent_hash, "abc123");
        assert_eq!(request.base_amount, 5000);
        assert_eq!(request.multiplier, 200);
    }

    #[test]
    fn apply_penalty_builder_validation() {
        let result = ApplyPenaltyBuilder::new().build();
        assert!(result.is_err());
    }

    #[test]
    fn apply_recovery_builder_full_flow() {
        let request = ApplyRecoveryBuilder::new()
            .agent_hash("xyz789")
            .action_type(RecoveryActionType::TradeFill)
            .amount(1000)
            .reason("successful fill")
            .build()
            .unwrap();

        assert_eq!(request.agent_hash, "xyz789");
        assert_eq!(request.action_type, RecoveryActionType::TradeFill);
        assert_eq!(request.amount, 1000);
    }

    #[test]
    fn apply_recovery_builder_validation() {
        let result = ApplyRecoveryBuilder::new().build();
        assert!(result.is_err());

        // Partial — missing required fields
        let result = ApplyRecoveryBuilder::new()
            .agent_hash("abc")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn force_milestone_builder_works() {
        let request = ForceMilestoneBuilder::new()
            .agent_hash("test-agent")
            .milestone_level(5)
            .gov_signature(vec![9u8; 64])
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
            .params(Params {
                slashing_multiplier: 200,
                ..Default::default()
            })
            .gov_signature(vec![0u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.params.slashing_multiplier, 200);
        assert_eq!(request.params.daily_recovery_cap_bps, 3000); // default
    }

    #[test]
    fn update_params_builder_validation() {
        let result = UpdateParamsBuilder::new().build();
        assert!(result.is_err());
    }
}
