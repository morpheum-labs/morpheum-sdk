//! Recommended prelude for the Morpheum SDK core.
//!
//! This prelude is designed for maximum ergonomics while remaining minimal
//! and focused. Most users should start with:
//!
//! ```rust
//! use morpheum_sdk_core::prelude::*;
//! ```
//!
//! It re-exports the most commonly used types, traits, and items from
//! the core crate, the official signing library, and protobuf definitions.

/// Core SDK types and traits
pub use crate::{
    client::MorpheumClient,
    config::SdkConfig,
    error::SdkError,
    transport::Transport,
    types::{AccountId, ChainId, SignedTx},
};

/// Most frequently used items from the official Morpheum signing library
pub use crate::signing::{
    // Signers
    AgentSigner,
    NativeSigner,
    // Core cryptographic types
    PublicKey,
    Signature,
    WalletType,
    // Agent delegation & claims
    TradingKeyClaim,
    VcClaimBuilder,
};

/// Commonly used protobuf types when constructing transactions
pub use crate::proto::tx::v1::{
    AuthInfo,
    SignDoc,
    SignerInfo,
    Tx,
    TxBody,
    TxRaw,
};

/// Current version of the Morpheum SDK core
pub use crate::VERSION;

/// Convenience type alias for the SDK's signed transaction wrapper
pub type SdkSignedTx = crate::types::SignedTx;

/// Convenience type alias for the signing library's raw `AccountId`
pub type SigningAccountId = crate::signing::AccountId;

#[cfg(test)]
mod tests {
    // This test ensures the prelude can be imported without conflicts
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn prelude_imports_cleanly() {
        let _ = VERSION;
    }
}