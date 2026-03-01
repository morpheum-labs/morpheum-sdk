//! Signer integration layer for the Morpheum Native SDK.
//!
//! This module provides a thin, zero-cost wrapper over the official
//! `morpheum-signing-native` crate. It re-exports the primary signers
//! (`NativeSigner`, `AgentSigner`) and key supporting types for convenient
//! use within the SDK.
//!
//! All cryptographic operations, claim handling, dynamic `SignerInfo`,
//! and `TradingKeyClaim` logic are delegated to the official signing library.
//! This file exists purely for ergonomic namespacing and discoverability.

pub use morpheum_signing_native::{
    // Primary signers
    NativeSigner,
    AgentSigner,

    // Core cryptographic types
    PublicKey,
    Signature,
    WalletType,

    // Agent delegation & claims
    TradingKeyClaim,
    VcClaimBuilder,

    // Common account type
    AccountId as SigningAccountId,
};

/// Recommended prelude for signer usage.
///
/// Most users should do:
/// ```rust
/// use morpheum_sdk_native::signer::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        NativeSigner,
        AgentSigner,
        TradingKeyClaim,
        VcClaimBuilder,
        PublicKey,
        Signature,
        WalletType,
    };
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn reexports_compile_cleanly() {
        // This test ensures all re-exports are valid and accessible
        let _ = NativeSigner::from_seed(&[0u8; 32]);
        let _ = AgentSigner::new(&[0u8; 32], SigningAccountId([0u8; 32]), None);
    }

    #[test]
    fn prelude_works() {
        use prelude::*;
        let _ = NativeSigner::from_seed(&[0u8; 32]);
    }
}