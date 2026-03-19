//! GMP SDK domain types.
//!
//! Mirrors the proto `gmp.v1.*` messages as idiomatic Rust structs with
//! bidirectional `From` conversions. Maps use `BTreeMap` to match prost codegen.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{
    collections::BTreeMap,
    string::String,
    vec::Vec,
};
#[cfg(feature = "std")]
use std::collections::BTreeMap;

use morpheum_proto::gmp::v1 as pb;

// ── Governance parameters ───────────────────────────────────────────

/// GMP module parameters (governance-controlled).
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GmpParams {
    pub hyperlane: Option<HyperlaneParams>,
    pub warp_route: Option<WarpRouteConfig>,
}

/// Hyperlane protocol security parameters.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HyperlaneParams {
    /// Ethereum-style validator addresses (20 bytes each).
    pub validators: Vec<Vec<u8>>,
    /// Minimum signatures required (m of n).
    pub threshold: u32,
    /// Hyperlane domain ID -> CAIP-2 chain identifier.
    pub domain_to_caip2: BTreeMap<u32, String>,
    /// Trusted sender addresses on source chains (32 bytes each, left-padded).
    pub trusted_senders: Vec<Vec<u8>>,
}

/// Warp Route configuration (governance-controlled).
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WarpRouteConfig {
    /// 32-byte address identifying the Warp Route handler on Morpheum.
    pub recipient_address: Vec<u8>,
    /// Per-domain token route config (key = Hyperlane domain ID).
    pub routes: BTreeMap<u32, WarpRouteToken>,
}

/// Token configuration for a single Warp Route domain.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WarpRouteToken {
    /// 32-byte collateral contract address on the source chain.
    pub collateral_address: Vec<u8>,
    /// Morpheum bank asset index (e.g., 1 for USDC).
    pub asset_index: u64,
}

// ── Response / info types ───────────────────────────────────────────

/// Protocol info entry.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProtocolInfo {
    pub protocol_id: String,
    pub configured: bool,
}

/// Result of a warp route transfer.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WarpRouteTransferResult {
    pub success: bool,
    pub message_id: Vec<u8>,
}

/// Result of processing a Hyperlane message.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ProcessHyperlaneResult {
    pub success: bool,
    pub message_id: Vec<u8>,
}

/// Result of settling a GMP payment.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SettleGmpPaymentResult {
    pub success: bool,
}

// ── Proto conversions: pb -> SDK ────────────────────────────────────

impl From<pb::Params> for GmpParams {
    fn from(p: pb::Params) -> Self {
        Self {
            hyperlane: p.hyperlane.map(Into::into),
            warp_route: p.warp_route.map(Into::into),
        }
    }
}

impl From<pb::HyperlaneParams> for HyperlaneParams {
    fn from(h: pb::HyperlaneParams) -> Self {
        Self {
            validators: h.validators,
            threshold: h.threshold,
            domain_to_caip2: h.domain_to_caip2,
            trusted_senders: h.trusted_senders,
        }
    }
}

impl From<pb::WarpRouteConfig> for WarpRouteConfig {
    fn from(w: pb::WarpRouteConfig) -> Self {
        Self {
            recipient_address: w.recipient_address,
            routes: w
                .routes
                .into_iter()
                .map(|(k, v)| (k, WarpRouteToken::from(v)))
                .collect(),
        }
    }
}

impl From<pb::WarpRouteToken> for WarpRouteToken {
    fn from(t: pb::WarpRouteToken) -> Self {
        Self {
            collateral_address: t.collateral_address,
            asset_index: t.asset_index,
        }
    }
}

// ── Proto conversions: SDK -> pb ────────────────────────────────────

impl From<GmpParams> for pb::Params {
    fn from(p: GmpParams) -> Self {
        Self {
            hyperlane: p.hyperlane.map(Into::into),
            axelar: None,
            wormhole: None,
            layerzero: None,
            warp_route: p.warp_route.map(Into::into),
        }
    }
}

impl From<HyperlaneParams> for pb::HyperlaneParams {
    fn from(h: HyperlaneParams) -> Self {
        Self {
            validators: h.validators,
            threshold: h.threshold,
            domain_to_caip2: h.domain_to_caip2,
            trusted_senders: h.trusted_senders,
        }
    }
}

impl From<WarpRouteConfig> for pb::WarpRouteConfig {
    fn from(w: WarpRouteConfig) -> Self {
        Self {
            recipient_address: w.recipient_address,
            routes: w
                .routes
                .into_iter()
                .map(|(k, v)| (k, pb::WarpRouteToken::from(v)))
                .collect(),
        }
    }
}

impl From<WarpRouteToken> for pb::WarpRouteToken {
    fn from(t: WarpRouteToken) -> Self {
        Self {
            collateral_address: t.collateral_address,
            asset_index: t.asset_index,
        }
    }
}

// ── Response conversions: pb -> SDK ─────────────────────────────────

impl From<pb::ProtocolInfo> for ProtocolInfo {
    fn from(p: pb::ProtocolInfo) -> Self {
        Self {
            protocol_id: p.protocol_id,
            configured: p.configured,
        }
    }
}

impl From<pb::WarpRouteTransferResponse> for WarpRouteTransferResult {
    fn from(r: pb::WarpRouteTransferResponse) -> Self {
        Self {
            success: r.success,
            message_id: r.message_id,
        }
    }
}

impl From<pb::ProcessHyperlaneMessageResponse> for ProcessHyperlaneResult {
    fn from(r: pb::ProcessHyperlaneMessageResponse) -> Self {
        Self {
            success: r.success,
            message_id: r.message_id,
        }
    }
}

impl From<pb::SettleGmpPaymentResponse> for SettleGmpPaymentResult {
    fn from(r: pb::SettleGmpPaymentResponse) -> Self {
        Self {
            success: r.success,
        }
    }
}
