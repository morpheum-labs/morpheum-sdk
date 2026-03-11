//! Morpheum Native SDK — Full Rust SDK for CLI tools, trading bots, and autonomous AI agents.
//!
//! This is the main entry point for the native SDK. It provides:
//! - `MorpheumSdk` — the central, ergonomic facade
//! - Convenience constructors: `native()` and `agent()`
//! - Feature-gated access to all modules (`market`, `vc`, `auth`)
//! - Seamless integration with the official `morpheum-signing-native` library
//!
//! Recommended usage:
//! ```rust,ignore
//! use morpheum_sdk_native::prelude::*;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;

// ==================== RE-EXPORTS ====================

// Core SDK foundation
pub use morpheum_sdk_core as core;

// Official signing library (native version with full crypto + BIP-39)
pub use morpheum_signing_native as signing;

// Feature-gated module clients
#[cfg(feature = "identity")]
pub use morpheum_sdk_identity as identity;

#[cfg(feature = "market")]
pub use morpheum_sdk_market as market;

#[cfg(feature = "vc")]
pub use morpheum_sdk_vc as vc;

#[cfg(feature = "auth")]
pub use morpheum_sdk_auth as auth;

#[cfg(feature = "agent_registry")]
pub use morpheum_sdk_agent_registry as agent_registry;

#[cfg(feature = "inference_registry")]
pub use morpheum_sdk_inference_registry as inference_registry;

#[cfg(feature = "interop")]
pub use morpheum_sdk_interop as interop;

#[cfg(feature = "job")]
pub use morpheum_sdk_job as job;

// Re-export commonly used core types
pub use core::{
    AccountId,
    ChainId,
    SdkConfig,
    SdkError,
    SignedTx,
};

// Re-export commonly used signing types
pub use signing::{
    AgentSigner,
    NativeSigner,
};

// Claim types are in signing-core, re-exported through native
pub use signing::claim::{TradingKeyClaim, VcClaimBuilder};

// ==================== PLACEHOLDER TRANSPORT ====================

/// A placeholder transport that returns errors for all operations.
///
/// Used as the default when no concrete transport (gRPC, HTTP, etc.) is provided.
/// In production, replace with a real transport implementation.
struct PlaceholderTransport;

#[async_trait(?Send)]
impl core::Transport for PlaceholderTransport {
    async fn broadcast_tx(&self, _tx_bytes: Vec<u8>) -> Result<core::BroadcastResult, SdkError> {
        Err(SdkError::transport(
            "no transport configured — provide a concrete transport implementation",
        ))
    }

    async fn query(&self, _path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
        Err(SdkError::transport(
            "no transport configured — provide a concrete transport implementation",
        ))
    }
}

// ==================== MAIN FACADE ====================

/// The main Morpheum SDK facade for native Rust applications.
///
/// This is the recommended entry point for most users. It holds configuration
/// and provides access to all enabled module clients.
pub struct MorpheumSdk {
    config: SdkConfig,
    transport: Box<dyn core::Transport>,
}

impl MorpheumSdk {
    /// Creates a new SDK with the given RPC endpoint and default chain ID.
    pub fn new(rpc_endpoint: impl Into<String>, chain_id: impl Into<ChainId>) -> Self {
        let config = SdkConfig::new(rpc_endpoint, chain_id);
        Self {
            config,
            transport: Box::new(PlaceholderTransport),
        }
    }

    /// Creates a new SDK with a custom transport and configuration.
    pub fn with_transport(
        config: SdkConfig,
        transport: Box<dyn core::Transport>,
    ) -> Self {
        Self { config, transport }
    }

    /// Returns the current SDK configuration.
    pub fn config(&self) -> &SdkConfig {
        &self.config
    }

    /// Returns a reference to the underlying transport.
    pub fn transport(&self) -> &dyn core::Transport {
        &*self.transport
    }
}

// ==================== CONVENIENCE CONSTRUCTORS ====================

/// Creates a new SDK instance using a `NativeSigner` (recommended for humans).
///
/// This is a convenience function. The signer is **not** stored in the SDK —
/// it is passed to `TxBuilder` per-transaction for maximum flexibility.
pub fn native(signer: NativeSigner) -> MorpheumSdk {
    let _ = signer; // Signer is used in TxBuilder, not stored in SDK
    MorpheumSdk::new("https://sentry.morpheum.xyz", "morpheum-1")
}

/// Creates a new SDK instance using an `AgentSigner` (recommended for autonomous agents).
pub fn agent(signer: AgentSigner) -> MorpheumSdk {
    let _ = signer; // Signer is used in TxBuilder, not stored in SDK
    MorpheumSdk::new("https://sentry.morpheum.xyz", "morpheum-1")
}

// ==================== PRELUDE ====================

/// Recommended prelude for the native SDK.
///
/// Most users should start with:
/// ```rust,ignore
/// use morpheum_sdk_native::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        MorpheumSdk,
        native,
        agent,
        AccountId,
        ChainId,
        SdkConfig,
        SdkError,
        SignedTx,
        NativeSigner,
        AgentSigner,
        TradingKeyClaim,
        VcClaimBuilder,
    };

    // Transaction builder and the canonical `Any` type for constructing messages.
    pub use super::core::builder::TxBuilder;
    pub use super::core::prelude::Any;

    // Feature-gated module re-exports
    #[cfg(feature = "market")]
    pub use super::market::MarketClient;

    #[cfg(feature = "vc")]
    pub use super::vc::VcClient;

    #[cfg(feature = "auth")]
    pub use super::auth::AuthClient;

    #[cfg(feature = "agent_registry")]
    pub use super::agent_registry::AgentRegistryClient;

    #[cfg(feature = "inference_registry")]
    pub use super::inference_registry::InferenceRegistryClient;

    #[cfg(feature = "interop")]
    pub use super::interop::InteropClient;

    #[cfg(feature = "job")]
    pub use super::job::JobClient;
}

// Current version of the native SDK
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn sdk_creates_cleanly() {
        let sdk = MorpheumSdk::new("https://test.morpheum.xyz", "morpheum-test-1");
        assert_eq!(sdk.config().default_chain_id.as_str(), "morpheum-test-1");
    }

    #[test]
    fn convenience_functions_work() {
        let _sdk = native(NativeSigner::from_seed(&[0u8; 32]));
        let _sdk2 = agent(AgentSigner::new(&[0u8; 32], signing::types::AccountId([0u8; 32]), None));
    }

    #[test]
    fn prelude_compiles_cleanly() {
        #[allow(unused_imports)]
        use prelude::*;
        let _ = VERSION;
    }
}
