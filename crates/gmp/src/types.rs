//! GMP SDK domain types.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use morpheum_proto::gmp::v1 as pb;

/// Module parameters.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GmpParams {
    pub hyperlane: Option<HyperlaneParams>,
}

/// Hyperlane-specific parameters.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HyperlaneParams {
    pub validators: Vec<Vec<u8>>,
    pub threshold: u32,
}

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

// ── Proto conversions ───────────────────────────────────────────────

impl From<pb::Params> for GmpParams {
    fn from(p: pb::Params) -> Self {
        Self {
            hyperlane: p.hyperlane.map(|h| HyperlaneParams {
                validators: h.validators,
                threshold: h.threshold,
            }),
        }
    }
}

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
