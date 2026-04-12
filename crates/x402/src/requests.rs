//! Request and response wrappers for the x402 payment module.
//!
//! Transaction request types provide `to_any()` for seamless `TxBuilder` integration.
//! Query request types provide `From` impls for direct protobuf conversion.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::x402::v1 as proto;
use morpheum_sdk_core::AccountId;

use crate::types::{PaymentPacket, Policy, Scheme};

// ====================== TRANSACTION REQUESTS ======================

/// Request to register a new spending policy for an agent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterPolicyRequest {
    pub owner_address: AccountId,
    pub policy: Policy,
    pub owner_signature: Vec<u8>,
}

impl RegisterPolicyRequest {
    pub fn new(owner_address: AccountId, policy: Policy, owner_signature: Vec<u8>) -> Self {
        Self { owner_address, policy, owner_signature }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgRegisterPolicy = self.clone().into();
        ProtoAny {
            type_url: "/x402.v1.MsgRegisterPolicy".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<RegisterPolicyRequest> for proto::MsgRegisterPolicy {
    fn from(req: RegisterPolicyRequest) -> Self {
        Self {
            owner_address: req.owner_address.to_string(),
            policy: Some(req.policy.into()),
            owner_signature: req.owner_signature,
        }
    }
}

/// Request to update an existing spending policy.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdatePolicyRequest {
    pub owner_address: AccountId,
    pub policy_id: String,
    pub updated_policy: Policy,
    pub owner_signature: Vec<u8>,
}

impl UpdatePolicyRequest {
    pub fn new(
        owner_address: AccountId,
        policy_id: impl Into<String>,
        updated_policy: Policy,
        owner_signature: Vec<u8>,
    ) -> Self {
        Self {
            owner_address,
            policy_id: policy_id.into(),
            updated_policy,
            owner_signature,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdatePolicy = self.clone().into();
        ProtoAny {
            type_url: "/x402.v1.MsgUpdatePolicy".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdatePolicyRequest> for proto::MsgUpdatePolicy {
    fn from(req: UpdatePolicyRequest) -> Self {
        Self {
            owner_address: req.owner_address.to_string(),
            policy_id: req.policy_id,
            updated_policy: Some(req.updated_policy.into()),
            owner_signature: req.owner_signature,
        }
    }
}

/// Request to rotate an agent's payment address.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RotateAddressRequest {
    pub owner_address: AccountId,
    pub owner_signature: Vec<u8>,
    pub reason: String,
}

impl RotateAddressRequest {
    pub fn new(
        owner_address: AccountId,
        owner_signature: Vec<u8>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            owner_address,
            owner_signature,
            reason: reason.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgRotateAddress = self.clone().into();
        ProtoAny {
            type_url: "/x402.v1.MsgRotateAddress".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<RotateAddressRequest> for proto::MsgRotateAddress {
    fn from(req: RotateAddressRequest) -> Self {
        Self {
            owner_address: req.owner_address.to_string(),
            owner_signature: req.owner_signature,
            reason: req.reason,
        }
    }
}

/// Request to approve an outbound payment from an agent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ApproveOutboundRequest {
    pub agent_id: String,
    pub destination: String,
    pub amount: u64,
    pub asset: String,
    pub memo: String,
    pub scheme: Scheme,
    pub idempotency_key: Vec<u8>,
}

impl ApproveOutboundRequest {
    pub fn new(
        agent_id: impl Into<String>,
        destination: impl Into<String>,
        amount: u64,
        asset: impl Into<String>,
        scheme: Scheme,
        idempotency_key: Vec<u8>,
    ) -> Self {
        Self {
            agent_id: agent_id.into(),
            destination: destination.into(),
            amount,
            asset: asset.into(),
            memo: String::new(),
            scheme,
            idempotency_key,
        }
    }

    pub fn with_memo(mut self, memo: impl Into<String>) -> Self {
        self.memo = memo.into();
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgApproveOutbound = self.clone().into();
        ProtoAny {
            type_url: "/x402.v1.MsgApproveOutbound".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ApproveOutboundRequest> for proto::MsgApproveOutbound {
    fn from(req: ApproveOutboundRequest) -> Self {
        Self {
            agent_id: req.agent_id,
            destination: req.destination,
            amount: req.amount,
            asset: req.asset,
            memo: req.memo,
            scheme: i32::from(req.scheme),
            idempotency_key: req.idempotency_key,
        }
    }
}

/// Request to settle a cross-chain bridge payment delivered via GMP.
///
/// Submits an `X402PaymentPacket` from an external EVM chain for settlement
/// on Morpheum. Used by relay services and operators for manual settlement.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SettleBridgePaymentRequest {
    pub relayer_address: String,
    pub packet: PaymentPacket,
}

impl SettleBridgePaymentRequest {
    pub fn new(relayer_address: impl Into<String>, packet: PaymentPacket) -> Self {
        Self {
            relayer_address: relayer_address.into(),
            packet,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgSettleBridgePayment = self.clone().into();
        ProtoAny {
            type_url: "/x402.v1.MsgSettleBridgePayment".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<SettleBridgePaymentRequest> for proto::MsgSettleBridgePayment {
    fn from(req: SettleBridgePaymentRequest) -> Self {
        Self {
            relayer_address: req.relayer_address,
            packet: Some(req.packet.into()),
        }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query a single receipt by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryReceiptRequest {
    pub receipt_id: String,
}

impl QueryReceiptRequest {
    pub fn new(receipt_id: impl Into<String>) -> Self {
        Self { receipt_id: receipt_id.into() }
    }
}

impl From<QueryReceiptRequest> for proto::QueryReceiptRequest {
    fn from(req: QueryReceiptRequest) -> Self {
        Self { receipt_id: req.receipt_id }
    }
}

/// Query receipts for an agent with pagination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryReceiptsByAgentRequest {
    pub agent_id: String,
    pub limit: u32,
    pub pagination_key: Option<String>,
}

impl QueryReceiptsByAgentRequest {
    pub fn new(agent_id: impl Into<String>, limit: u32) -> Self {
        Self {
            agent_id: agent_id.into(),
            limit,
            pagination_key: None,
        }
    }

    pub fn pagination_key(mut self, key: impl Into<String>) -> Self {
        self.pagination_key = Some(key.into());
        self
    }
}

impl From<QueryReceiptsByAgentRequest> for proto::QueryReceiptsByAgentRequest {
    fn from(req: QueryReceiptsByAgentRequest) -> Self {
        Self {
            agent_id: req.agent_id,
            limit: req.limit,
            pagination_key: req.pagination_key.unwrap_or_default(),
        }
    }
}

/// Query an agent's spending policy.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPolicyRequest {
    pub agent_id: String,
    pub policy_id: String,
}

impl QueryPolicyRequest {
    pub fn new(agent_id: impl Into<String>, policy_id: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
            policy_id: policy_id.into(),
        }
    }
}

impl From<QueryPolicyRequest> for proto::QueryPolicyRequest {
    fn from(req: QueryPolicyRequest) -> Self {
        Self {
            agent_id: req.agent_id,
            policy_id: req.policy_id,
        }
    }
}

/// Query an agent's x402 capabilities.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryCapabilitiesRequest {
    pub agent_id: String,
}

impl QueryCapabilitiesRequest {
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self { agent_id: agent_id.into() }
    }
}

impl From<QueryCapabilitiesRequest> for proto::QueryCapabilitiesRequest {
    fn from(req: QueryCapabilitiesRequest) -> Self {
        Self { agent_id: req.agent_id }
    }
}

/// Query the x402 module parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsRequest;

impl From<QueryParamsRequest> for proto::QueryParamsRequest {
    fn from(_: QueryParamsRequest) -> Self {
        Self {}
    }
}

/// Request to finalize an Upto usage-based payment.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FinalizeUptoRequest {
    pub seller_address: String,
    pub pre_auth_id: String,
    pub actual_amount: u64,
}

impl FinalizeUptoRequest {
    pub fn new(
        seller_address: impl Into<String>,
        pre_auth_id: impl Into<String>,
        actual_amount: u64,
    ) -> Self {
        Self {
            seller_address: seller_address.into(),
            pre_auth_id: pre_auth_id.into(),
            actual_amount,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgFinalizeUpto = self.clone().into();
        ProtoAny {
            type_url: "/x402.v1.MsgFinalizeUpto".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<FinalizeUptoRequest> for proto::MsgFinalizeUpto {
    fn from(req: FinalizeUptoRequest) -> Self {
        Self {
            seller_address: req.seller_address,
            pre_auth_id: req.pre_auth_id,
            actual_amount: req.actual_amount,
        }
    }
}

/// Query pending Upto pre-authorizations.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPendingUptoRequest {
    pub agent_id: String,
    pub seller_address: Option<String>,
    pub pre_auth_id: Option<String>,
}

impl QueryPendingUptoRequest {
    pub fn new(agent_id: impl Into<String>) -> Self {
        Self {
            agent_id: agent_id.into(),
            seller_address: None,
            pre_auth_id: None,
        }
    }

    pub fn seller_address(mut self, addr: impl Into<String>) -> Self {
        self.seller_address = Some(addr.into());
        self
    }

    pub fn pre_auth_id(mut self, id: impl Into<String>) -> Self {
        self.pre_auth_id = Some(id.into());
        self
    }
}

impl From<QueryPendingUptoRequest> for proto::QueryPendingUptoRequest {
    fn from(req: QueryPendingUptoRequest) -> Self {
        Self {
            agent_id: req.agent_id,
            seller_address: req.seller_address.unwrap_or_default(),
            pre_auth_id: req.pre_auth_id.unwrap_or_default(),
        }
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
    fn register_policy_to_any() {
        let req = RegisterPolicyRequest::new(
            AccountId::new([1u8; 32]),
            test_policy(),
            vec![0xAB, 0xCD],
        );

        let any = req.to_any();
        assert_eq!(any.type_url, "/x402.v1.MsgRegisterPolicy");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn update_policy_to_any() {
        let mut policy = test_policy();
        policy.policy_id = "pol-1".into();

        let req = UpdatePolicyRequest::new(
            AccountId::new([2u8; 32]),
            "pol-1",
            policy,
            vec![0xEF],
        );

        let any = req.to_any();
        assert_eq!(any.type_url, "/x402.v1.MsgUpdatePolicy");
    }

    #[test]
    fn rotate_address_to_any() {
        let req = RotateAddressRequest::new(
            AccountId::new([3u8; 32]),
            vec![0xFF],
            "scheduled rotation",
        );

        let any = req.to_any();
        assert_eq!(any.type_url, "/x402.v1.MsgRotateAddress");
    }

    #[test]
    fn approve_outbound_to_any() {
        let req = ApproveOutboundRequest::new(
            "agent-1",
            "0xdest",
            5000,
            "USDC",
            Scheme::ExactEvm,
            vec![0x01, 0x02],
        )
        .with_memo("MCP tool call");

        let any = req.to_any();
        assert_eq!(any.type_url, "/x402.v1.MsgApproveOutbound");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn settle_bridge_payment_to_any() {
        use crate::types::PaymentPacket;

        let packet = PaymentPacket {
            payment_id: "pay-001".into(),
            source_chain: "eip155:8453".into(),
            target_agent_id: "agent-1".into(),
            amount: 5000,
            asset: "USDC".into(),
            memo: "bridge payment".into(),
            signature_payload: vec![0xAA],
            reply_channel: "gmp-42".into(),
            payer_address: String::new(),
        };

        let req = SettleBridgePaymentRequest::new("relayer-1", packet);
        let any = req.to_any();
        assert_eq!(any.type_url, "/x402.v1.MsgSettleBridgePayment");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn query_request_conversions() {
        let r1: proto::QueryReceiptRequest = QueryReceiptRequest::new("rcpt-1").into();
        assert_eq!(r1.receipt_id, "rcpt-1");

        let r2: proto::QueryReceiptsByAgentRequest =
            QueryReceiptsByAgentRequest::new("agent-1", 50)
                .pagination_key("key-abc")
                .into();
        assert_eq!(r2.agent_id, "agent-1");
        assert_eq!(r2.limit, 50);
        assert_eq!(r2.pagination_key, "key-abc");

        let r3: proto::QueryPolicyRequest = QueryPolicyRequest::new("agent-1", "pol-1").into();
        assert_eq!(r3.agent_id, "agent-1");
        assert_eq!(r3.policy_id, "pol-1");

        let r4: proto::QueryCapabilitiesRequest =
            QueryCapabilitiesRequest::new("agent-1").into();
        assert_eq!(r4.agent_id, "agent-1");

        let _r5: proto::QueryParamsRequest = QueryParamsRequest.into();
    }

    #[test]
    fn finalize_upto_to_any() {
        let req = FinalizeUptoRequest::new("seller-1", "preauth-001", 750);
        let any = req.to_any();
        assert_eq!(any.type_url, "/x402.v1.MsgFinalizeUpto");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn query_pending_upto_conversion() {
        let req = QueryPendingUptoRequest::new("agent-1")
            .seller_address("seller-1")
            .pre_auth_id("preauth-001");

        let proto_req: proto::QueryPendingUptoRequest = req.into();
        assert_eq!(proto_req.agent_id, "agent-1");
        assert_eq!(proto_req.seller_address, "seller-1");
        assert_eq!(proto_req.pre_auth_id, "preauth-001");
    }
}
