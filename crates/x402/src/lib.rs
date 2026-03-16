//! x402 payment module for the Morpheum SDK.
//!
//! This module provides full support for managing x402 policy-gated payments
//! on Morpheum, including policy registration and updates, payment address rotation,
//! outbound payment approval, and querying receipts, capabilities, and module parameters.
//!
//! The x402 protocol enables autonomous AI agents to make and receive payments
//! with VC-gated spending caps, TEE-attested settlement, and cross-chain GMP bridging.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

pub use builder::{
    ApproveOutboundBuilder,
    RegisterPolicyBuilder,
    RotateAddressBuilder,
    UpdatePolicyBuilder,
};

pub use client::X402Client;

pub use types::{
    AttestedReceipt,
    Capabilities,
    Params,
    PaymentDirection,
    Policy,
    Receipt,
    ReceiptStatus,
    Scheme,
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
        AttestedReceipt,
        Capabilities,
        Params,
        PaymentDirection,
        Policy,
        Receipt,
        ReceiptStatus,
        Scheme,
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
