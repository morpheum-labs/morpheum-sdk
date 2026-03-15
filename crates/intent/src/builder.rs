//! Fluent builders for the Intent module.
//!
//! This module provides ergonomic, type-safe fluent builders for all intent
//! transaction operations (submit, cancel, parameter updates). Each builder
//! follows the classic Builder pattern and returns the corresponding request
//! type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{CancelIntentRequest, SubmitIntentRequest};
use crate::types::{
    AgentIntent, ConditionalParams, DeclarativeParams, IntentParams, IntentStatus, IntentType,
    MultiLegParams, TwapParams,
};

/// Fluent builder for constructing and submitting an agent intent.
///
/// Supports all four intent types. Use the type-specific parameter setter
/// (`conditional()`, `twap()`, `multi_leg()`, or `declarative()`) to configure
/// the intent's execution logic.
///
/// # Example
/// ```rust,ignore
/// let request = SubmitIntentBuilder::new()
///     .agent_hash("abc123def456")
///     .intent_type(IntentType::Conditional)
///     .conditional(ConditionalParams {
///         condition: "price > 50000".into(),
///         action: "market_buy 1 BTC".into(),
///     })
///     .vc_proof_hash("vc-proof-hash")
///     .expiry_timestamp(1_700_003_600)
///     .agent_signature(sig_bytes)
///     .build()?;
///
/// let any = request.to_any();
/// ```
#[derive(Default)]
pub struct SubmitIntentBuilder {
    agent_hash: Option<String>,
    intent_type: Option<IntentType>,
    params: Option<IntentParams>,
    vc_proof_hash: Option<String>,
    expiry_timestamp: Option<u64>,
    priority_boost: Option<u32>,
    agent_signature: Option<Vec<u8>>,
}

impl SubmitIntentBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the agent hash (SHA-256 of the agent's DID).
    pub fn agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.agent_hash = Some(hash.into());
        self
    }

    /// Sets the intent type.
    pub fn intent_type(mut self, intent_type: IntentType) -> Self {
        self.intent_type = Some(intent_type);
        self
    }

    /// Sets conditional parameters.
    pub fn conditional(mut self, params: ConditionalParams) -> Self {
        self.intent_type = Some(IntentType::Conditional);
        self.params = Some(IntentParams::Conditional(params));
        self
    }

    /// Sets TWAP parameters.
    pub fn twap(mut self, params: TwapParams) -> Self {
        self.intent_type = Some(IntentType::Twap);
        self.params = Some(IntentParams::Twap(params));
        self
    }

    /// Sets multi-leg parameters.
    pub fn multi_leg(mut self, params: MultiLegParams) -> Self {
        self.intent_type = Some(IntentType::MultiLeg);
        self.params = Some(IntentParams::MultiLeg(params));
        self
    }

    /// Sets declarative parameters.
    pub fn declarative(mut self, params: DeclarativeParams) -> Self {
        self.intent_type = Some(IntentType::Declarative);
        self.params = Some(IntentParams::Declarative(params));
        self
    }

    /// Sets the VC proof hash (delegation authorisation).
    pub fn vc_proof_hash(mut self, hash: impl Into<String>) -> Self {
        self.vc_proof_hash = Some(hash.into());
        self
    }

    /// Sets the expiry timestamp (0 = no expiry).
    pub fn expiry_timestamp(mut self, ts: u64) -> Self {
        self.expiry_timestamp = Some(ts);
        self
    }

    /// Sets the priority boost from reputation/milestones.
    pub fn priority_boost(mut self, boost: u32) -> Self {
        self.priority_boost = Some(boost);
        self
    }

    /// Sets the agent signature.
    pub fn agent_signature(mut self, sig: Vec<u8>) -> Self {
        self.agent_signature = Some(sig);
        self
    }

    /// Builds the submit request, performing validation.
    pub fn build(self) -> Result<SubmitIntentRequest, SdkError> {
        let agent_hash = self.agent_hash.ok_or_else(|| {
            SdkError::invalid_input("agent_hash is required for intent submission")
        })?;

        let intent_type = self.intent_type.ok_or_else(|| {
            SdkError::invalid_input("intent_type is required for intent submission")
        })?;

        let params = self.params.ok_or_else(|| {
            SdkError::invalid_input("params are required for intent submission (use conditional(), twap(), multi_leg(), or declarative())")
        })?;

        let agent_signature = self.agent_signature.ok_or_else(|| {
            SdkError::invalid_input("agent_signature is required for intent submission")
        })?;

        let intent = AgentIntent {
            intent_id: String::new(), // Assigned by the runtime
            agent_hash,
            intent_type,
            params: Some(params),
            vc_proof_hash: self.vc_proof_hash.unwrap_or_default(),
            expiry_timestamp: self.expiry_timestamp.unwrap_or(0),
            priority_boost: self.priority_boost.unwrap_or(0),
            status: IntentStatus::Pending,
            created_at: 0, // Set by the runtime
        };

        Ok(SubmitIntentRequest::new(intent, agent_signature))
    }
}

/// Fluent builder for cancelling an active intent.
///
/// # Example
/// ```rust,ignore
/// let request = CancelIntentBuilder::new()
///     .intent_id("intent-001")
///     .agent_signature(sig_bytes)
///     .reason("Market conditions changed")
///     .build()?;
/// ```
#[derive(Default)]
pub struct CancelIntentBuilder {
    intent_id: Option<String>,
    agent_signature: Option<Vec<u8>>,
    reason: Option<String>,
}

impl CancelIntentBuilder {
    /// Creates a new empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the intent ID to cancel.
    pub fn intent_id(mut self, id: impl Into<String>) -> Self {
        self.intent_id = Some(id.into());
        self
    }

    /// Sets the agent signature authorising the cancellation.
    pub fn agent_signature(mut self, sig: Vec<u8>) -> Self {
        self.agent_signature = Some(sig);
        self
    }

    /// Sets the reason for cancellation.
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    /// Builds the cancel request, performing validation.
    pub fn build(self) -> Result<CancelIntentRequest, SdkError> {
        let intent_id = self.intent_id.ok_or_else(|| {
            SdkError::invalid_input("intent_id is required for cancellation")
        })?;

        let agent_signature = self.agent_signature.ok_or_else(|| {
            SdkError::invalid_input("agent_signature is required for cancellation")
        })?;

        let reason = self.reason.ok_or_else(|| {
            SdkError::invalid_input("reason is required for cancellation")
        })?;

        Ok(CancelIntentRequest::new(intent_id, agent_signature, reason))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use crate::types::Leg;

    #[test]
    fn submit_conditional_builder_full_flow() {
        let request = SubmitIntentBuilder::new()
            .agent_hash("agent-abc")
            .conditional(ConditionalParams {
                condition: "price > 50000".into(),
                action: "buy 1 BTC".into(),
            })
            .vc_proof_hash("vc-hash")
            .expiry_timestamp(1_700_003_600)
            .priority_boost(5)
            .agent_signature(vec![1u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.intent.agent_hash, "agent-abc");
        assert_eq!(request.intent.intent_type, IntentType::Conditional);
        assert_eq!(request.intent.expiry_timestamp, 1_700_003_600);
        assert_eq!(request.intent.priority_boost, 5);
        assert!(matches!(
            request.intent.params,
            Some(IntentParams::Conditional(_))
        ));
    }

    #[test]
    fn submit_twap_builder() {
        let request = SubmitIntentBuilder::new()
            .agent_hash("agent-xyz")
            .twap(TwapParams {
                direction: "buy".into(),
                total_size: 100_000,
                duration_ms: 60_000,
                num_slices: 10,
                slice_curve: "linear".into(),
                slippage_tolerance_bps: 50,
                rebalance_trigger: String::new(),
            })
            .agent_signature(vec![2u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.intent.intent_type, IntentType::Twap);
    }

    #[test]
    fn submit_multi_leg_builder() {
        let request = SubmitIntentBuilder::new()
            .agent_hash("agent-ml")
            .multi_leg(MultiLegParams {
                legs: vec![
                    Leg { action: "buy".into(), size: 1000, pair: "BTC-USDC".into() },
                    Leg { action: "sell".into(), size: 500, pair: "ETH-USDC".into() },
                ],
                atomic: true,
            })
            .agent_signature(vec![3u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.intent.intent_type, IntentType::MultiLeg);
        assert!(matches!(
            request.intent.params,
            Some(IntentParams::MultiLeg(ref p)) if p.atomic && p.legs.len() == 2
        ));
    }

    #[test]
    fn submit_declarative_builder() {
        let request = SubmitIntentBuilder::new()
            .agent_hash("agent-decl")
            .declarative(DeclarativeParams {
                raw_goal: "Maximise yield on idle USDC".into(),
                goal_embedding: vec![0.1, 0.2, 0.3],
                constraints: r#"{"max_risk": "low"}"#.into(),
                preferred_style: "conservative".into(),
            })
            .agent_signature(vec![4u8; 64])
            .build()
            .unwrap();

        assert_eq!(request.intent.intent_type, IntentType::Declarative);
    }

    #[test]
    fn submit_builder_validation_missing_agent() {
        let result = SubmitIntentBuilder::new().build();
        assert!(result.is_err());
    }

    #[test]
    fn submit_builder_validation_missing_params() {
        let result = SubmitIntentBuilder::new()
            .agent_hash("agent-abc")
            .intent_type(IntentType::Conditional)
            .agent_signature(vec![1u8; 64])
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn submit_builder_defaults() {
        let request = SubmitIntentBuilder::new()
            .agent_hash("agent-abc")
            .conditional(ConditionalParams {
                condition: "test".into(),
                action: "test".into(),
            })
            .agent_signature(vec![1u8; 64])
            .build()
            .unwrap();

        // Defaults when not set
        assert_eq!(request.intent.expiry_timestamp, 0);
        assert_eq!(request.intent.priority_boost, 0);
        assert!(request.intent.vc_proof_hash.is_empty());
        assert!(request.intent.intent_id.is_empty()); // Assigned by runtime
        assert_eq!(request.intent.status, IntentStatus::Pending);
    }

    #[test]
    fn cancel_builder_full_flow() {
        let request = CancelIntentBuilder::new()
            .intent_id("intent-001")
            .agent_signature(vec![5u8; 64])
            .reason("Market conditions changed")
            .build()
            .unwrap();

        assert_eq!(request.intent_id, "intent-001");
        assert_eq!(request.reason, "Market conditions changed");
    }

    #[test]
    fn cancel_builder_validation() {
        let result = CancelIntentBuilder::new().build();
        assert!(result.is_err());

        let result = CancelIntentBuilder::new()
            .intent_id("intent-001")
            .build();
        assert!(result.is_err());
    }

}
