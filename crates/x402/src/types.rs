//! Domain types for the x402 payment module.
//!
//! Clean, idiomatic Rust representations of the x402 protobuf messages.
//! Each type provides full round-trip conversion to/from protobuf via `From`
//! impls and remains strictly `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::x402::v1 as proto;

// ====================== ENUMS ======================

/// Direction of an x402 payment (inbound to agent or outbound from agent).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PaymentDirection {
    Unspecified,
    Inbound,
    Outbound,
}

impl From<i32> for PaymentDirection {
    fn from(v: i32) -> Self {
        match proto::PaymentDirection::try_from(v).unwrap_or(proto::PaymentDirection::Unspecified) {
            proto::PaymentDirection::Unspecified => Self::Unspecified,
            proto::PaymentDirection::Inbound => Self::Inbound,
            proto::PaymentDirection::Outbound => Self::Outbound,
        }
    }
}

impl From<PaymentDirection> for i32 {
    fn from(d: PaymentDirection) -> Self {
        match d {
            PaymentDirection::Unspecified => proto::PaymentDirection::Unspecified as i32,
            PaymentDirection::Inbound => proto::PaymentDirection::Inbound as i32,
            PaymentDirection::Outbound => proto::PaymentDirection::Outbound as i32,
        }
    }
}

/// x402 payment scheme (exact, EVM-specific, etc.).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Scheme {
    Unspecified,
    Exact,
    ExactEvm,
}

impl From<i32> for Scheme {
    fn from(v: i32) -> Self {
        match proto::X402Scheme::try_from(v).unwrap_or(proto::X402Scheme::SchemeUnspecified) {
            proto::X402Scheme::SchemeUnspecified => Self::Unspecified,
            proto::X402Scheme::SchemeExact => Self::Exact,
            proto::X402Scheme::SchemeExactEvm => Self::ExactEvm,
        }
    }
}

impl From<Scheme> for i32 {
    fn from(s: Scheme) -> Self {
        match s {
            Scheme::Unspecified => proto::X402Scheme::SchemeUnspecified as i32,
            Scheme::Exact => proto::X402Scheme::SchemeExact as i32,
            Scheme::ExactEvm => proto::X402Scheme::SchemeExactEvm as i32,
        }
    }
}

/// Status of an x402 payment receipt.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ReceiptStatus {
    Unspecified,
    Pending,
    Completed,
    Failed,
}

impl From<i32> for ReceiptStatus {
    fn from(v: i32) -> Self {
        match proto::ReceiptStatus::try_from(v).unwrap_or(proto::ReceiptStatus::Unspecified) {
            proto::ReceiptStatus::Unspecified => Self::Unspecified,
            proto::ReceiptStatus::Pending => Self::Pending,
            proto::ReceiptStatus::Completed => Self::Completed,
            proto::ReceiptStatus::Failed => Self::Failed,
        }
    }
}

impl From<ReceiptStatus> for i32 {
    fn from(s: ReceiptStatus) -> Self {
        match s {
            ReceiptStatus::Unspecified => proto::ReceiptStatus::Unspecified as i32,
            ReceiptStatus::Pending => proto::ReceiptStatus::Pending as i32,
            ReceiptStatus::Completed => proto::ReceiptStatus::Completed as i32,
            ReceiptStatus::Failed => proto::ReceiptStatus::Failed as i32,
        }
    }
}

// ====================== STRUCTS ======================

/// A settled x402 payment receipt.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Receipt {
    pub receipt_id: String,
    pub agent_id: String,
    pub direction: PaymentDirection,
    pub scheme: Scheme,
    pub amount: u64,
    pub asset: String,
    pub counterparty: String,
    pub memo: String,
    pub status: ReceiptStatus,
    pub merkle_root: String,
    pub validated_at: u64,
    pub attestation: Vec<u8>,
}

impl From<proto::X402Receipt> for Receipt {
    fn from(p: proto::X402Receipt) -> Self {
        Self {
            receipt_id: p.receipt_id,
            agent_id: p.agent_id,
            direction: PaymentDirection::from(p.direction),
            scheme: Scheme::from(p.scheme),
            amount: p.amount,
            asset: p.asset,
            counterparty: p.counterparty,
            memo: p.memo,
            status: ReceiptStatus::from(p.status),
            merkle_root: p.merkle_root,
            validated_at: p.validated_at,
            attestation: p.attestation,
        }
    }
}

impl From<Receipt> for proto::X402Receipt {
    fn from(r: Receipt) -> Self {
        Self {
            receipt_id: r.receipt_id,
            agent_id: r.agent_id,
            direction: i32::from(r.direction),
            scheme: i32::from(r.scheme),
            amount: r.amount,
            asset: r.asset,
            counterparty: r.counterparty,
            memo: r.memo,
            status: i32::from(r.status),
            merkle_root: r.merkle_root,
            validated_at: r.validated_at,
            attestation: r.attestation,
        }
    }
}

/// Spending policy governing an agent's x402 payment capabilities.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Policy {
    pub policy_id: String,
    pub agent_id: String,
    pub max_per_service_usd: u64,
    pub daily_cap_usd: u64,
    pub hourly_cap_usd: u64,
    pub reputation_multiplier_bps: u32,
    pub last_updated: u64,
}

impl From<proto::X402Policy> for Policy {
    fn from(p: proto::X402Policy) -> Self {
        Self {
            policy_id: p.policy_id,
            agent_id: p.agent_id,
            max_per_service_usd: p.max_per_service_usd,
            daily_cap_usd: p.daily_cap_usd,
            hourly_cap_usd: p.hourly_cap_usd,
            reputation_multiplier_bps: p.reputation_multiplier_bps,
            last_updated: p.last_updated,
        }
    }
}

impl From<Policy> for proto::X402Policy {
    fn from(p: Policy) -> Self {
        Self {
            policy_id: p.policy_id,
            agent_id: p.agent_id,
            max_per_service_usd: p.max_per_service_usd,
            daily_cap_usd: p.daily_cap_usd,
            hourly_cap_usd: p.hourly_cap_usd,
            reputation_multiplier_bps: p.reputation_multiplier_bps,
            last_updated: p.last_updated,
        }
    }
}

/// Advertised x402 capabilities for an agent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Capabilities {
    pub agent_id: String,
    pub enabled: bool,
    pub preferred_schemes: u64,
    pub min_amount_usd: u64,
    pub endpoint: String,
    pub updated_at: u64,
}

impl Capabilities {
    /// Whether the agent has x402 capabilities enabled.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

impl From<proto::X402Capabilities> for Capabilities {
    fn from(c: proto::X402Capabilities) -> Self {
        Self {
            agent_id: c.agent_id,
            enabled: c.enabled,
            preferred_schemes: c.preferred_schemes,
            min_amount_usd: c.min_amount_usd,
            endpoint: c.endpoint,
            updated_at: c.updated_at,
        }
    }
}

impl From<Capabilities> for proto::X402Capabilities {
    fn from(c: Capabilities) -> Self {
        Self {
            agent_id: c.agent_id,
            enabled: c.enabled,
            preferred_schemes: c.preferred_schemes,
            min_amount_usd: c.min_amount_usd,
            endpoint: c.endpoint,
            updated_at: c.updated_at,
        }
    }
}

/// TEE-attested receipt with cryptographic proofs.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttestedReceipt {
    pub receipt_id: String,
    pub receipt: Option<Receipt>,
    pub tee_quote: Vec<u8>,
    pub signature: Vec<u8>,
    pub merkle_proof: Vec<u8>,
}

impl From<proto::AttestedReceipt> for AttestedReceipt {
    fn from(a: proto::AttestedReceipt) -> Self {
        Self {
            receipt_id: a.receipt_id,
            receipt: a.receipt.map(Into::into),
            tee_quote: a.tee_quote,
            signature: a.signature,
            merkle_proof: a.merkle_proof,
        }
    }
}

impl From<AttestedReceipt> for proto::AttestedReceipt {
    fn from(a: AttestedReceipt) -> Self {
        Self {
            receipt_id: a.receipt_id,
            receipt: a.receipt.map(Into::into),
            tee_quote: a.tee_quote,
            signature: a.signature,
            merkle_proof: a.merkle_proof,
        }
    }
}

// ====================== BRIDGE / SETTLEMENT ======================

/// A cross-chain payment packet delivered via GMP bridge.
///
/// Represents an x402 payment originating from an external EVM chain
/// (Base, Ethereum, Arbitrum) destined for a Morpheum agent.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PaymentPacket {
    pub payment_id: String,
    pub source_chain: String,
    pub target_agent_id: String,
    pub amount: u64,
    pub asset: String,
    pub memo: String,
    pub signature_payload: Vec<u8>,
    pub reply_channel: String,
    pub payer_address: String,
}

impl From<proto::X402PaymentPacket> for PaymentPacket {
    fn from(p: proto::X402PaymentPacket) -> Self {
        Self {
            payment_id: p.payment_id,
            source_chain: p.source_chain,
            target_agent_id: p.target_agent_id,
            amount: p.amount,
            asset: p.asset,
            memo: p.memo,
            signature_payload: p.signature_payload,
            reply_channel: p.reply_channel,
            payer_address: p.payer_address,
        }
    }
}

impl From<PaymentPacket> for proto::X402PaymentPacket {
    fn from(p: PaymentPacket) -> Self {
        Self {
            payment_id: p.payment_id,
            source_chain: p.source_chain,
            target_agent_id: p.target_agent_id,
            amount: p.amount,
            asset: p.asset,
            memo: p.memo,
            signature_payload: p.signature_payload,
            reply_channel: p.reply_channel,
            payer_address: p.payer_address,
        }
    }
}

/// Result of a cross-chain bridge settlement on Morpheum.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BridgeSettlementResult {
    pub success: bool,
    pub receipt: Option<Receipt>,
    pub gmp_reply_payload: Vec<u8>,
    pub receipt_hash: String,
}

impl From<proto::SettleBridgePaymentResponse> for BridgeSettlementResult {
    fn from(r: proto::SettleBridgePaymentResponse) -> Self {
        Self {
            success: r.success,
            receipt: r.receipt.map(Into::into),
            gmp_reply_payload: r.gmp_reply_payload,
            receipt_hash: r.receipt_hash,
        }
    }
}

/// Module-level parameters for the x402 module.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    pub default_daily_cap_usd: u64,
    pub platform_min_amount_usd: u64,
    pub facilitator_timeout_seconds: u64,
    pub enable_tee_facilitator: bool,
    pub authorized_relayers: Vec<String>,
    pub enable_signature_verification: bool,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            default_daily_cap_usd: 10_000,
            platform_min_amount_usd: 1,
            facilitator_timeout_seconds: 30,
            enable_tee_facilitator: true,
            authorized_relayers: Vec::new(),
            enable_signature_verification: false,
        }
    }
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            default_daily_cap_usd: p.default_daily_cap_usd,
            platform_min_amount_usd: p.platform_min_amount_usd,
            facilitator_timeout_seconds: p.facilitator_timeout_seconds,
            enable_tee_facilitator: p.enable_tee_facilitator,
            authorized_relayers: p.authorized_relayers,
            enable_signature_verification: p.enable_signature_verification,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            default_daily_cap_usd: p.default_daily_cap_usd,
            platform_min_amount_usd: p.platform_min_amount_usd,
            facilitator_timeout_seconds: p.facilitator_timeout_seconds,
            enable_tee_facilitator: p.enable_tee_facilitator,
            authorized_relayers: p.authorized_relayers,
            enable_signature_verification: p.enable_signature_verification,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn receipt_roundtrip() {
        let receipt = Receipt {
            receipt_id: "rcpt-001".into(),
            agent_id: "agent-1".into(),
            direction: PaymentDirection::Inbound,
            scheme: Scheme::ExactEvm,
            amount: 5000,
            asset: "USDC".into(),
            counterparty: "0xabc".into(),
            memo: "tool call".into(),
            status: ReceiptStatus::Completed,
            merkle_root: "0xdeadbeef".into(),
            validated_at: 1710000000,
            attestation: vec![1, 2, 3],
        };

        let proto: proto::X402Receipt = receipt.clone().into();
        let back: Receipt = proto.into();
        assert_eq!(receipt, back);
    }

    #[test]
    fn policy_roundtrip() {
        let policy = Policy {
            policy_id: "pol-1".into(),
            agent_id: "agent-1".into(),
            max_per_service_usd: 100,
            daily_cap_usd: 1000,
            hourly_cap_usd: 200,
            reputation_multiplier_bps: 15000,
            last_updated: 1710000000,
        };

        let proto: proto::X402Policy = policy.clone().into();
        let back: Policy = proto.into();
        assert_eq!(policy, back);
    }

    #[test]
    fn capabilities_roundtrip() {
        let caps = Capabilities {
            agent_id: "agent-1".into(),
            enabled: true,
            preferred_schemes: 3,
            min_amount_usd: 1,
            endpoint: "pay://agent-1".into(),
            updated_at: 1710000000,
        };

        let proto: proto::X402Capabilities = caps.clone().into();
        let back: Capabilities = proto.into();
        assert_eq!(caps, back);
        assert!(back.is_enabled());
    }

    #[test]
    fn params_roundtrip() {
        let params = Params {
            default_daily_cap_usd: 50000,
            platform_min_amount_usd: 5,
            facilitator_timeout_seconds: 60,
            enable_tee_facilitator: false,
            authorized_relayers: vec!["relayer-1".into()],
            enable_signature_verification: true,
        };

        let proto: proto::Params = params.clone().into();
        let back: Params = proto.into();
        assert_eq!(params, back);
    }

    #[test]
    fn attested_receipt_roundtrip() {
        let attested = AttestedReceipt {
            receipt_id: "rcpt-001".into(),
            receipt: Some(Receipt {
                receipt_id: "rcpt-001".into(),
                agent_id: "agent-1".into(),
                direction: PaymentDirection::Inbound,
                scheme: Scheme::Exact,
                amount: 100,
                asset: "USDC".into(),
                counterparty: "peer".into(),
                memo: String::new(),
                status: ReceiptStatus::Completed,
                merkle_root: String::new(),
                validated_at: 0,
                attestation: vec![],
            }),
            tee_quote: vec![0xDE, 0xAD],
            signature: vec![0xBE, 0xEF],
            merkle_proof: vec![0xCA, 0xFE],
        };

        let proto: proto::AttestedReceipt = attested.clone().into();
        let back: AttestedReceipt = proto.into();
        assert_eq!(attested, back);
    }

    #[test]
    fn enum_from_unknown_value_defaults() {
        assert_eq!(PaymentDirection::from(99), PaymentDirection::Unspecified);
        assert_eq!(Scheme::from(99), Scheme::Unspecified);
        assert_eq!(ReceiptStatus::from(99), ReceiptStatus::Unspecified);
    }

    #[test]
    fn params_default_is_sensible() {
        let p = Params::default();
        assert!(p.default_daily_cap_usd > 0);
        assert!(p.platform_min_amount_usd > 0);
        assert!(p.enable_tee_facilitator);
    }

    #[test]
    fn payment_packet_roundtrip() {
        let packet = PaymentPacket {
            payment_id: "pay-001".into(),
            source_chain: "eip155:8453".into(),
            target_agent_id: "agent-1".into(),
            amount: 5000,
            asset: "USDC".into(),
            memo: "cross-chain tool call".into(),
            signature_payload: vec![0xAA, 0xBB],
            reply_channel: "gmp-reply-42".into(),
            payer_address: "0x1234abcd".into(),
        };

        let proto: proto::X402PaymentPacket = packet.clone().into();
        let back: PaymentPacket = proto.into();
        assert_eq!(packet, back);
    }

    #[test]
    fn bridge_settlement_result_from_proto() {
        let proto_resp = proto::SettleBridgePaymentResponse {
            success: true,
            receipt: Some(Default::default()),
            gmp_reply_payload: vec![1, 2, 3],
            receipt_hash: "abc123".into(),
        };

        let result: BridgeSettlementResult = proto_resp.into();
        assert!(result.success);
        assert!(result.receipt.is_some());
        assert_eq!(result.gmp_reply_payload, vec![1, 2, 3]);
        assert_eq!(result.receipt_hash, "abc123");
    }
}
