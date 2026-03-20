#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

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
pub use morpheum_proto as proto;

// Public modules — each has a single, clear responsibility (SOLID)
pub mod error;
pub mod types;
pub mod config;
pub mod transport;
pub mod client;
pub mod builder;

#[cfg(feature = "chain-registry")]
pub mod chain_registry;

// ── Root-level re-exports for ergonomic `crate::SdkError` style access ──

pub use error::SdkError;
pub use types::{AccountId, ChainId, SignedTx};
pub use config::SdkConfig;
pub use transport::{Transport, BroadcastResult};
pub use client::MorpheumClient;

#[cfg(feature = "chain-registry")]
pub use chain_registry::ChainRegistryOps;

/// Current version of the Morpheum SDK core (from Cargo.toml).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Recommended imports for most users of the core crate.
///
/// Usage:
/// ```rust,ignore
/// use morpheum_sdk_core::prelude::*;
/// ```
pub mod prelude {
    // Core SDK types
    pub use crate::client::MorpheumClient;
    pub use crate::{AccountId, BroadcastResult, ChainId, SdkConfig, SdkError, SignedTx, Transport, VERSION};

    // Signing library domain types (via submodule paths)
    pub use crate::signing::types::{PublicKey, Signature, WalletType};
    pub use crate::signing::claim::{TradingKeyClaim, VcClaimBuilder};
    pub use crate::signing::signer::Signer;

    // The prost_types::Any re-exported from the signing library.
    // This is the canonical `Any` type used in TxBuilder.add_message().
    pub use crate::signing::Any;

    // Transaction builder
    pub use crate::builder::TxBuilder;

    // Frequently used protobuf types when constructing transactions
    pub use crate::proto::tx::v1::{
        AuthInfo,
        SignDoc,
        SignerInfo,
        Tx,
        TxBody,
        TxRaw,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!VERSION.is_empty());
    }
}
