//! Fluent builders for x402 module transactions.
//!
//! Each builder validates required fields and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::{AccountId, SdkError};

use crate::requests::{
    ApproveOutboundRequest,
    FinalizeUptoRequest,
    RegisterPolicyRequest,
    RotateAddressRequest,
    SettleBridgePaymentRequest,
    UpdatePolicyRequest,
};
use crate::types::{PaymentPacket, Policy, Scheme};

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

/// Fluent builder for settling a cross-chain bridge payment.
///
/// Constructs a `SettleBridgePaymentRequest` with all required fields
/// validated before submission. Used by relay services and operators.
#[derive(Default)]
pub struct SettleBridgePaymentBuilder {
    relayer_address: Option<String>,
    payment_id: Option<String>,
    source_chain: Option<String>,
    target_agent_id: Option<String>,
    amount: Option<u64>,
    asset: Option<String>,
    memo: Option<String>,
    signature_payload: Option<Vec<u8>>,
    reply_channel: Option<String>,
    payer_address: Option<String>,
}

impl SettleBridgePaymentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn relayer_address(mut self, addr: impl Into<String>) -> Self {
        self.relayer_address = Some(addr.into());
        self
    }

    pub fn payment_id(mut self, id: impl Into<String>) -> Self {
        self.payment_id = Some(id.into());
        self
    }

    pub fn source_chain(mut self, chain: impl Into<String>) -> Self {
        self.source_chain = Some(chain.into());
        self
    }

    pub fn target_agent_id(mut self, agent: impl Into<String>) -> Self {
        self.target_agent_id = Some(agent.into());
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

    pub fn signature_payload(mut self, payload: Vec<u8>) -> Self {
        self.signature_payload = Some(payload);
        self
    }

    pub fn reply_channel(mut self, channel: impl Into<String>) -> Self {
        self.reply_channel = Some(channel.into());
        self
    }

    pub fn payer_address(mut self, addr: impl Into<String>) -> Self {
        self.payer_address = Some(addr.into());
        self
    }

    /// Convenience: set all packet fields from an existing `PaymentPacket`.
    pub fn packet(mut self, p: PaymentPacket) -> Self {
        self.payment_id = Some(p.payment_id);
        self.source_chain = Some(p.source_chain);
        self.target_agent_id = Some(p.target_agent_id);
        self.amount = Some(p.amount);
        self.asset = Some(p.asset);
        self.memo = Some(p.memo);
        self.signature_payload = Some(p.signature_payload);
        self.reply_channel = Some(p.reply_channel);
        self.payer_address = Some(p.payer_address);
        self
    }

    pub fn build(self) -> Result<SettleBridgePaymentRequest, SdkError> {
        let relayer_address = self.relayer_address.ok_or_else(|| {
            SdkError::invalid_input("relayer_address is required for bridge settlement")
        })?;

        let payment_id = self.payment_id.ok_or_else(|| {
            SdkError::invalid_input("payment_id is required")
        })?;

        let source_chain = self.source_chain.ok_or_else(|| {
            SdkError::invalid_input("source_chain is required")
        })?;

        let target_agent_id = self.target_agent_id.ok_or_else(|| {
            SdkError::invalid_input("target_agent_id is required")
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

        let signature_payload = self.signature_payload.ok_or_else(|| {
            SdkError::invalid_input("signature_payload is required")
        })?;

        let reply_channel = self.reply_channel.ok_or_else(|| {
            SdkError::invalid_input("reply_channel is required")
        })?;

        let packet = PaymentPacket {
            payment_id,
            source_chain,
            target_agent_id,
            amount,
            asset,
            memo: self.memo.unwrap_or_default(),
            signature_payload,
            reply_channel,
            payer_address: self.payer_address.unwrap_or_default(),
        };

        Ok(SettleBridgePaymentRequest::new(relayer_address, packet))
    }
}

/// Fluent builder for finalizing an Upto usage-based payment.
///
/// The seller specifies the actual consumed amount (which must be <=
/// the pre-authorized maximum). The unused portion is atomically refunded.
#[derive(Default)]
pub struct FinalizeUptoBuilder {
    seller_address: Option<String>,
    pre_auth_id: Option<String>,
    actual_amount: Option<u64>,
}

impl FinalizeUptoBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn seller_address(mut self, addr: impl Into<String>) -> Self {
        self.seller_address = Some(addr.into());
        self
    }

    pub fn pre_auth_id(mut self, id: impl Into<String>) -> Self {
        self.pre_auth_id = Some(id.into());
        self
    }

    pub fn actual_amount(mut self, amount: u64) -> Self {
        self.actual_amount = Some(amount);
        self
    }

    pub fn build(self) -> Result<FinalizeUptoRequest, SdkError> {
        let seller_address = self.seller_address.ok_or_else(|| {
            SdkError::invalid_input("seller_address is required for Upto finalization")
        })?;

        let pre_auth_id = self.pre_auth_id.ok_or_else(|| {
            SdkError::invalid_input("pre_auth_id is required")
        })?;

        let actual_amount = self.actual_amount.ok_or_else(|| {
            SdkError::invalid_input("actual_amount is required")
        })?;

        Ok(FinalizeUptoRequest::new(seller_address, pre_auth_id, actual_amount))
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
            max_amount_required: 1000,
            supported_schemes: 3,
            asset: "USDC".into(),
            network: "eip155:8453".into(),
            last_updated: 0,
            upto_details: None,
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

    #[test]
    fn settle_bridge_payment_builder_full_flow() {
        let req = SettleBridgePaymentBuilder::new()
            .relayer_address("relayer-1")
            .payment_id("pay-001")
            .source_chain("eip155:8453")
            .target_agent_id("agent-1")
            .amount(5000)
            .asset("USDC")
            .memo("bridge test")
            .signature_payload(vec![0xAA, 0xBB])
            .reply_channel("gmp-42")
            .build()
            .unwrap();

        assert_eq!(req.relayer_address, "relayer-1");
        assert_eq!(req.packet.payment_id, "pay-001");
        assert_eq!(req.packet.source_chain, "eip155:8453");
        assert_eq!(req.packet.amount, 5000);
    }

    #[test]
    fn settle_bridge_payment_builder_from_packet() {
        use crate::types::PaymentPacket;

        let packet = PaymentPacket {
            payment_id: "pay-002".into(),
            source_chain: "eip155:1".into(),
            target_agent_id: "agent-2".into(),
            amount: 10000,
            asset: "USDC".into(),
            memo: String::new(),
            signature_payload: vec![0xCC],
            reply_channel: "gmp-99".into(),
            payer_address: "0xpayer".into(),
        };

        let req = SettleBridgePaymentBuilder::new()
            .relayer_address("relayer-2")
            .packet(packet.clone())
            .build()
            .unwrap();

        assert_eq!(req.packet, packet);
    }

    #[test]
    fn settle_bridge_payment_builder_zero_amount_rejected() {
        let result = SettleBridgePaymentBuilder::new()
            .relayer_address("r")
            .payment_id("p")
            .source_chain("c")
            .target_agent_id("a")
            .amount(0)
            .asset("USDC")
            .signature_payload(vec![1])
            .reply_channel("ch")
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn settle_bridge_payment_builder_missing_fields() {
        assert!(SettleBridgePaymentBuilder::new().build().is_err());
        assert!(SettleBridgePaymentBuilder::new()
            .relayer_address("r")
            .build()
            .is_err());
    }

    #[test]
    fn finalize_upto_builder_full_flow() {
        let req = FinalizeUptoBuilder::new()
            .seller_address("seller-1")
            .pre_auth_id("preauth-001")
            .actual_amount(750)
            .build()
            .unwrap();

        assert_eq!(req.seller_address, "seller-1");
        assert_eq!(req.pre_auth_id, "preauth-001");
        assert_eq!(req.actual_amount, 750);
    }

    #[test]
    fn finalize_upto_builder_missing_fields() {
        assert!(FinalizeUptoBuilder::new().build().is_err());
        assert!(FinalizeUptoBuilder::new()
            .seller_address("s")
            .build()
            .is_err());
        assert!(FinalizeUptoBuilder::new()
            .seller_address("s")
            .pre_auth_id("p")
            .build()
            .is_err());
    }
}
