//! Fluent builders for x402 module transactions.
//!
//! Each builder validates required fields and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::{AccountId, SdkError};

use crate::requests::{
    ApproveOutboundRequest,
    RegisterPolicyRequest,
    RotateAddressRequest,
    UpdatePolicyRequest,
};
use crate::types::{Policy, Scheme};

/// Fluent builder for registering a new spending policy.
#[derive(Default)]
pub struct RegisterPolicyBuilder {
    owner_address: Option<AccountId>,
    policy: Option<Policy>,
    owner_signature: Option<Vec<u8>>,
}

impl RegisterPolicyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn owner_address(mut self, address: impl Into<AccountId>) -> Self {
        self.owner_address = Some(address.into());
        self
    }

    pub fn policy(mut self, policy: Policy) -> Self {
        self.policy = Some(policy);
        self
    }

    pub fn owner_signature(mut self, sig: Vec<u8>) -> Self {
        self.owner_signature = Some(sig);
        self
    }

    pub fn build(self) -> Result<RegisterPolicyRequest, SdkError> {
        let owner_address = self.owner_address.ok_or_else(|| {
            SdkError::invalid_input("owner_address is required for policy registration")
        })?;

        let policy = self.policy.ok_or_else(|| {
            SdkError::invalid_input("policy is required")
        })?;

        let owner_signature = self.owner_signature.ok_or_else(|| {
            SdkError::invalid_input("owner_signature is required")
        })?;

        Ok(RegisterPolicyRequest::new(owner_address, policy, owner_signature))
    }
}

/// Fluent builder for updating an existing spending policy.
#[derive(Default)]
pub struct UpdatePolicyBuilder {
    owner_address: Option<AccountId>,
    policy_id: Option<String>,
    updated_policy: Option<Policy>,
    owner_signature: Option<Vec<u8>>,
}

impl UpdatePolicyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn owner_address(mut self, address: impl Into<AccountId>) -> Self {
        self.owner_address = Some(address.into());
        self
    }

    pub fn policy_id(mut self, id: impl Into<String>) -> Self {
        self.policy_id = Some(id.into());
        self
    }

    pub fn updated_policy(mut self, policy: Policy) -> Self {
        self.updated_policy = Some(policy);
        self
    }

    pub fn owner_signature(mut self, sig: Vec<u8>) -> Self {
        self.owner_signature = Some(sig);
        self
    }

    pub fn build(self) -> Result<UpdatePolicyRequest, SdkError> {
        let owner_address = self.owner_address.ok_or_else(|| {
            SdkError::invalid_input("owner_address is required for policy update")
        })?;

        let policy_id = self.policy_id.ok_or_else(|| {
            SdkError::invalid_input("policy_id is required")
        })?;

        let updated_policy = self.updated_policy.ok_or_else(|| {
            SdkError::invalid_input("updated_policy is required")
        })?;

        let owner_signature = self.owner_signature.ok_or_else(|| {
            SdkError::invalid_input("owner_signature is required")
        })?;

        Ok(UpdatePolicyRequest::new(owner_address, policy_id, updated_policy, owner_signature))
    }
}

/// Fluent builder for rotating an agent's payment address.
#[derive(Default)]
pub struct RotateAddressBuilder {
    owner_address: Option<AccountId>,
    owner_signature: Option<Vec<u8>>,
    reason: Option<String>,
}

impl RotateAddressBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn owner_address(mut self, address: impl Into<AccountId>) -> Self {
        self.owner_address = Some(address.into());
        self
    }

    pub fn owner_signature(mut self, sig: Vec<u8>) -> Self {
        self.owner_signature = Some(sig);
        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn build(self) -> Result<RotateAddressRequest, SdkError> {
        let owner_address = self.owner_address.ok_or_else(|| {
            SdkError::invalid_input("owner_address is required for address rotation")
        })?;

        let owner_signature = self.owner_signature.ok_or_else(|| {
            SdkError::invalid_input("owner_signature is required")
        })?;

        Ok(RotateAddressRequest::new(
            owner_address,
            owner_signature,
            self.reason.unwrap_or_else(|| "unspecified".into()),
        ))
    }
}

/// Fluent builder for approving an outbound x402 payment.
#[derive(Default)]
pub struct ApproveOutboundBuilder {
    agent_id: Option<String>,
    destination: Option<String>,
    amount: Option<u64>,
    asset: Option<String>,
    memo: Option<String>,
    scheme: Option<Scheme>,
    idempotency_key: Option<Vec<u8>>,
}

impl ApproveOutboundBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn agent_id(mut self, id: impl Into<String>) -> Self {
        self.agent_id = Some(id.into());
        self
    }

    pub fn destination(mut self, dest: impl Into<String>) -> Self {
        self.destination = Some(dest.into());
        self
    }

    pub fn amount(mut self, amount: u64) -> Self {
        self.amount = Some(amount);
        self
    }

    pub fn asset(mut self, asset: impl Into<String>) -> Self {
        self.asset = Some(asset.into());
        self
    }

    pub fn memo(mut self, memo: impl Into<String>) -> Self {
        self.memo = Some(memo.into());
        self
    }

    pub fn scheme(mut self, scheme: Scheme) -> Self {
        self.scheme = Some(scheme);
        self
    }

    pub fn idempotency_key(mut self, key: Vec<u8>) -> Self {
        self.idempotency_key = Some(key);
        self
    }

    pub fn build(self) -> Result<ApproveOutboundRequest, SdkError> {
        let agent_id = self.agent_id.ok_or_else(|| {
            SdkError::invalid_input("agent_id is required for outbound approval")
        })?;

        let destination = self.destination.ok_or_else(|| {
            SdkError::invalid_input("destination is required")
        })?;

        let amount = self.amount.ok_or_else(|| {
            SdkError::invalid_input("amount is required")
        })?;

        if amount == 0 {
            return Err(SdkError::invalid_input("amount must be greater than zero"));
        }

        let asset = self.asset.ok_or_else(|| {
            SdkError::invalid_input("asset is required")
        })?;

        let scheme = self.scheme.ok_or_else(|| {
            SdkError::invalid_input("scheme is required")
        })?;

        let idempotency_key = self.idempotency_key.ok_or_else(|| {
            SdkError::invalid_input("idempotency_key is required")
        })?;

        let mut req = ApproveOutboundRequest::new(
            agent_id,
            destination,
            amount,
            asset,
            scheme,
            idempotency_key,
        );

        if let Some(memo) = self.memo {
            req = req.with_memo(memo);
        }

        Ok(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use morpheum_sdk_core::AccountId;

    fn test_policy() -> Policy {
        Policy {
            policy_id: String::new(),
            agent_id: "agent-1".into(),
            max_per_service_usd: 100,
            daily_cap_usd: 1000,
            hourly_cap_usd: 200,
            reputation_multiplier_bps: 10000,
            last_updated: 0,
        }
    }

    #[test]
    fn register_policy_builder_full_flow() {
        let req = RegisterPolicyBuilder::new()
            .owner_address(AccountId::new([1u8; 32]))
            .policy(test_policy())
            .owner_signature(vec![0xAB])
            .build()
            .unwrap();

        assert_eq!(req.policy.agent_id, "agent-1");
        assert_eq!(req.owner_signature, vec![0xAB]);
    }

    #[test]
    fn register_policy_builder_missing_fields() {
        assert!(RegisterPolicyBuilder::new().build().is_err());
        assert!(RegisterPolicyBuilder::new()
            .owner_address(AccountId::new([1u8; 32]))
            .build()
            .is_err());
    }

    #[test]
    fn update_policy_builder_full_flow() {
        let req = UpdatePolicyBuilder::new()
            .owner_address(AccountId::new([2u8; 32]))
            .policy_id("pol-1")
            .updated_policy(test_policy())
            .owner_signature(vec![0xCD])
            .build()
            .unwrap();

        assert_eq!(req.policy_id, "pol-1");
    }

    #[test]
    fn rotate_address_builder_full_flow() {
        let req = RotateAddressBuilder::new()
            .owner_address(AccountId::new([3u8; 32]))
            .owner_signature(vec![0xEF])
            .reason("key compromise")
            .build()
            .unwrap();

        assert_eq!(req.reason, "key compromise");
    }

    #[test]
    fn rotate_address_builder_default_reason() {
        let req = RotateAddressBuilder::new()
            .owner_address(AccountId::new([3u8; 32]))
            .owner_signature(vec![0xFF])
            .build()
            .unwrap();

        assert_eq!(req.reason, "unspecified");
    }

    #[test]
    fn approve_outbound_builder_full_flow() {
        let req = ApproveOutboundBuilder::new()
            .agent_id("agent-1")
            .destination("0xdest")
            .amount(5000)
            .asset("USDC")
            .scheme(Scheme::ExactEvm)
            .idempotency_key(vec![0x01])
            .memo("tool call")
            .build()
            .unwrap();

        assert_eq!(req.amount, 5000);
        assert_eq!(req.memo, "tool call");
        assert_eq!(req.scheme, Scheme::ExactEvm);
    }

    #[test]
    fn approve_outbound_builder_zero_amount_rejected() {
        let result = ApproveOutboundBuilder::new()
            .agent_id("agent-1")
            .destination("0xdest")
            .amount(0)
            .asset("USDC")
            .scheme(Scheme::Exact)
            .idempotency_key(vec![0x01])
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn approve_outbound_builder_missing_fields() {
        assert!(ApproveOutboundBuilder::new().build().is_err());
    }
}
