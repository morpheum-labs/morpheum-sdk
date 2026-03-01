#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc = include_str!("../../README.md")]

// When the "std" feature is enabled, we bring in the standard library.
// Otherwise we stay strictly no_std.
#[cfg(feature = "std")]
extern crate std;

extern crate alloc;

// Re-export the official Morpheum signing core (the no_std part).
// This gives us AccountId, PublicKey, Signature, TradingKeyClaim,
// VcClaimBuilder, dynamic SignerInfo, etc. — exactly as designed.
// We do **not** depend on -native here to keep sdk-core truly no_std.
pub use morpheum_signing_core as signing;

// Re-export our generated protobuf definitions for clean access.
pub use morpheum_sdk_proto as proto;

// Public modules — each has a single, clear responsibility (SOLID)
pub mod error;
pub mod types;
pub mod config;
pub mod transport;
pub mod client;
pub mod builder;

/// Current version of the Morpheum SDK core (from Cargo.toml).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Recommended imports for most users of the core crate.
///
/// This prelude is designed to be ergonomic while remaining explicit.
/// Usage:
/// ```rust
/// use morpheum_sdk_core::prelude::*;
/// ```
pub mod prelude {
    // Core SDK types
    pub use crate::client::MorpheumClient;
    pub use crate::config::SdkConfig;
    pub use crate::error::SdkError;
    pub use crate::transport::Transport;
    pub use crate::types::*;

    // Most commonly used items from the signing library
    pub use crate::signing::{
        AccountId,
        PublicKey,
        Signature,
        WalletType,
        SignedTx,
        TradingKeyClaim,
        VcClaimBuilder,
        NativeSigner,
        AgentSigner,
    };

    // Frequently used protobuf types
    pub use crate::proto::tx::v1::{
        SignDoc,
        Tx,
        TxRaw,
        SignerInfo,
        AuthInfo,
        TxBody,
    };
}

// Tests for the prelude and version (always compiled)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn prelude_compiles() {
        // This test ensures the prelude can be imported cleanly
        use crate::prelude::*;
        let _ = VERSION;
    }
}