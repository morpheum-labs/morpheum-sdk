//! x402 payment module for the Morpheum SDK.
//!
//! This module provides support for managing x402 external payments on Morpheum,
//! including service pricing policy registration, payment address rotation,
//! outbound payment approval, and cross-chain GMP bridge settlement.
//!
//! The x402 protocol is a lightweight HTTP 402 standard for external web-scale
//! micropayments between AI agents and services. Internal on-chain payments
//! use the native bank module instead.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod builder;
pub mod chains;
pub mod client;
pub mod requests;
pub mod types;

// ==================== PUBLIC RE-EXPORTS ====================

pub use builder::{
    ApproveOutboundBuilder,
    FinalizeUptoBuilder,
    RegisterPolicyBuilder,
    RotateAddressBuilder,
    SettleBridgePaymentBuilder,
    UpdatePolicyBuilder,
};

pub use client::X402Client;

pub use chains::{
    ChainMetadata,
    SignatureScheme,
    KNOWN_CHAINS,
    find_by_caip2,
    find_by_caip2_str,
    resolve_chain_name,
};

pub use types::{
    BridgeSettlementResult,
    Capabilities,
    FinalizeUptoResult,
    Params,
    PaymentDirection,
    PaymentPacket,
    Policy,
    Receipt,
    ReceiptStatus,
    Scheme,
    UptoDetails,
};

pub use requests::*;

pub use morpheum_sdk_core::{
    AccountId,
    ChainId,
    SdkError,
    SignedTx,
};

/// Recommended prelude for the x402 module.
///
/// ```rust,ignore
/// use morpheum_sdk_x402::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        X402Client,
        BridgeSettlementResult,
        Capabilities,
        ChainMetadata,
        FinalizeUptoBuilder,
        FinalizeUptoResult,
        SignatureScheme,
        KNOWN_CHAINS,
        Params,
        PaymentDirection,
        PaymentPacket,
        Policy,
        Receipt,
        ReceiptStatus,
        Scheme,
        SettleBridgePaymentBuilder,
        UptoDetails,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the x402 module (synchronized with workspace version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn public_api_compiles_cleanly() {
        #[allow(unused_imports)]
        use prelude::*;
        let _ = VERSION;
    }
}
