//! Domain types for the VC (Verifiable Credentials) module.
//!
//! These are clean, idiomatic Rust representations of the VC protobuf messages.
//! They provide type safety (using `AccountId`), ergonomic APIs, and full
//! round-trip conversion to/from protobuf while remaining strictly `no_std`
//! compatible.

use alloc::{string::String, vec::Vec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use prost_types::Any as ProtoAny;

use morpheum_sdk_core::{AccountId, SdkError};
use morpheum_sdk_proto::vc::v1 as proto;

/// Verifiable Credential (VC) — the core data structure issued by an agent
/// to another agent with specific claims and permissions.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vc {
    pub vc_id: String,
    pub issuer: AccountId,
    pub subject: AccountId,
    pub claims: VcClaims,
    pub issuance_timestamp: u64,
    pub expiry_timestamp: u64,
    pub status_list_index: u32,
}

impl Vc {
    /// Returns whether the VC is currently expired based on the given timestamp.
    pub fn is_expired(&self, current_timestamp: u64) -> bool {
        self.expiry_timestamp != 0 && current_timestamp >= self.expiry_timestamp
    }
}

impl From<proto::Vc> for Vc {
    fn from(p: proto::Vc) -> Self {
        Self {
            vc_id: p.vc_id,
            issuer: AccountId::new(hex::decode(&p.issuer_agent_hash)
                .ok()
                .and_then(|b| b.try_into().ok())
                .unwrap_or([0u8; 32])),
            subject: AccountId::new(hex::decode(&p.subject_agent_hash)
                .ok()
                .and_then(|b| b.try_into().ok())
                .unwrap_or([0u8; 32])),
            claims: p.claims.into(),
            issuance_timestamp: p.issuance_timestamp,
            expiry_timestamp: p.expiry_timestamp,
            status_list_index: p.status_list_index,
        }
    }
}

impl From<Vc> for proto::Vc {
    fn from(v: Vc) -> Self {
        Self {
            vc_id: v.vc_id,
            issuer_agent_hash: v.issuer.to_string(),
            subject_agent_hash: v.subject.to_string(),
            claims: v.claims.into(),
            issuance_timestamp: v.issuance_timestamp,
            expiry_timestamp: v.expiry_timestamp,
            status_list_index: v.status_list_index,
        }
    }
}

/// VC Claims — the permissions and limits embedded in a Verifiable Credential.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VcClaims {
    pub max_daily_usd: u64,
    pub allowed_pairs_bitflags: u64,
    pub max_slippage_bps: u32,
    pub max_position_usd: u64,
    pub custom_constraints: Option<String>,
}

impl From<proto::VcClaims> for VcClaims {
    fn from(p: proto::VcClaims) -> Self {
        Self {
            max_daily_usd: p.max_daily_usd,
            allowed_pairs_bitflags: p.allowed_pairs_bitflags,
            max_slippage_bps: p.max_slippage_bps,
            max_position_usd: p.max_position_usd,
            custom_constraints: if p.custom_constraints.is_empty() {
                None
            } else {
                Some(p.custom_constraints)
            },
        }
    }
}

impl From<VcClaims> for proto::VcClaims {
    fn from(c: VcClaims) -> Self {
        Self {
            max_daily_usd: c.max_daily_usd,
            allowed_pairs_bitflags: c.allowed_pairs_bitflags,
            max_slippage_bps: c.max_slippage_bps,
            max_position_usd: c.max_position_usd,
            custom_constraints: c.custom_constraints.unwrap_or_default(),
        }
    }
}

/// Verifiable Presentation (VP) — an agent-signed bundle containing a VC.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Vp {
    pub vc: Vc,
    pub agent_signature: Vec<u8>,
    pub presentation_timestamp: u64,
}

impl From<proto::Vp> for Vp {
    fn from(p: proto::Vp) -> Self {
        Self {
            vc: p.vc.into(),
            agent_signature: p.agent_signature,
            presentation_timestamp: p.presentation_timestamp,
        }
    }
}

impl From<Vp> for proto::Vp {
    fn from(v: Vp) -> Self {
        Self {
            vc: v.vc.into(),
            agent_signature: v.agent_signature,
            presentation_timestamp: v.presentation_timestamp,
        }
    }
}

/// Status of a Verifiable Credential (returned by queries).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VcStatus {
    pub vc_id: String,
    pub is_valid: bool,
    pub is_revoked: bool,
    pub is_expired: bool,
    pub revoked_at: u64,
}

impl From<proto::VcStatus> for VcStatus {
    fn from(p: proto::VcStatus) -> Self {
        Self {
            vc_id: p.vc_id,
            is_valid: p.is_valid,
            is_revoked: p.is_revoked,
            is_expired: p.is_expired,
            revoked_at: p.revoked_at,
        }
    }
}

/// Module parameters (governance-controlled).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Params {
    pub default_expiry_seconds: u64,
    pub revocation_bitmap_chunk_size: u32,
    pub max_vcs_per_issuer: u32,
    pub enable_self_revocation: bool,
    pub slashing_multiplier: u32,
    pub min_reputation_to_issue_vc: u64,
}

impl From<proto::Params> for Params {
    fn from(p: proto::Params) -> Self {
        Self {
            default_expiry_seconds: p.default_expiry_seconds,
            revocation_bitmap_chunk_size: p.revocation_bitmap_chunk_size,
            max_vcs_per_issuer: p.max_vcs_per_issuer,
            enable_self_revocation: p.enable_self_revocation,
            slashing_multiplier: p.slashing_multiplier,
            min_reputation_to_issue_vc: p.min_reputation_to_issue_vc,
        }
    }
}

impl From<Params> for proto::Params {
    fn from(p: Params) -> Self {
        Self {
            default_expiry_seconds: p.default_expiry_seconds,
            revocation_bitmap_chunk_size: p.revocation_bitmap_chunk_size,
            max_vcs_per_issuer: p.max_vcs_per_issuer,
            enable_self_revocation: p.enable_self_revocation,
            slashing_multiplier: p.slashing_multiplier,
            min_reputation_to_issue_vc: p.min_reputation_to_issue_vc,
        }
    }
}

/// ActiveVc — used primarily in genesis state or for bulk queries.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ActiveVc {
    pub vc_id: String,
    pub issuer: AccountId,
    pub subject: AccountId,
    pub expiry_timestamp: u64,
    pub status_list_index: u32,
}

impl From<proto::ActiveVc> for ActiveVc {
    fn from(p: proto::ActiveVc) -> Self {
        Self {
            vc_id: p.vc_id,
            issuer: AccountId::new(hex::decode(&p.issuer_agent_hash)
                .ok()
                .and_then(|b| b.try_into().ok())
                .unwrap_or([0u8; 32])),
            subject: AccountId::new(hex::decode(&p.subject_agent_hash)
                .ok()
                .and_then(|b| b.try_into().ok())
                .unwrap_or([0u8; 32])),
            expiry_timestamp: p.expiry_timestamp,
            status_list_index: p.status_list_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vc_roundtrip() {
        let vc = Vc {
            vc_id: "vc_123456".into(),
            issuer: AccountId::new([1u8; 32]),
            subject: AccountId::new([2u8; 32]),
            claims: VcClaims {
                max_daily_usd: 100_000,
                allowed_pairs_bitflags: 0b0001,
                max_slippage_bps: 50,
                max_position_usd: 500_000,
                custom_constraints: Some("{\"max_leverage\": 10}".into()),
            },
            issuance_timestamp: 1_700_000_000,
            expiry_timestamp: 1_800_000_000,
            status_list_index: 42,
        };

        let proto: proto::Vc = vc.clone().into();
        let back: Vc = proto.into();

        assert_eq!(vc, back);
    }

    #[test]
    fn vc_is_expired() {
        let vc = Vc {
            vc_id: "test".into(),
            issuer: AccountId::new([0; 32]),
            subject: AccountId::new([0; 32]),
            claims: VcClaims {
                max_daily_usd: 0,
                allowed_pairs_bitflags: 0,
                max_slippage_bps: 0,
                max_position_usd: 0,
                custom_constraints: None,
            },
            issuance_timestamp: 0,
            expiry_timestamp: 1_700_000_000,
            status_list_index: 0,
        };

        assert!(vc.is_expired(1_700_000_001));
        assert!(!vc.is_expired(1_699_999_999));
    }
}