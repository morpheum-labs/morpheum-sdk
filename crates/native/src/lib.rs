//! Morpheum Native SDK — Full Rust SDK for CLI tools, trading bots, and autonomous AI agents.
//!
//! This is the main entry point for the native SDK. It provides:
//! - `MorpheumSdk` — the central, ergonomic facade
//! - Convenience constructors: `native()` and `agent()`
//! - Feature-gated access to all modules (`market`, `vc`, `auth`)
//! - Seamless integration with the official `morpheum-signing-native` library
//!
//! Recommended usage:
//! ```rust
//! use morpheum_sdk_native::prelude::*;
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

// ==================== RE-EXPORTS ====================

// Core SDK foundation
pub use morpheum_sdk_core as core;

// Official signing library (native version with full crypto + BIP-39)
pub use morpheum_signing_native as signing;

// Feature-gated module clients
#[cfg(feature = "market")]
pub use morpheum_sdk_market as market;

#[cfg(feature = "vc")]
pub use morpheum_sdk_vc as vc;

#[cfg(feature = "auth")]
pub use morpheum_sdk_auth as auth;

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
    TradingKeyClaim,
    VcClaimBuilder,
};

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
        Self::with_config(config)
    }

    /// Creates a new SDK using a custom `SdkConfig`.
    pub fn with_config(config: SdkConfig) -> Self {
        // In a full implementation, we would choose transport based on features
        // (e.g. tonic for gRPC, reqwest for HTTP). For now we use a placeholder.
        let transport: Box<dyn core::Transport> = Box::new(core::transport::DummyTransport);

        Self { config, transport }
    }

    /// Returns the current SDK configuration.
    pub fn config(&self) -> &SdkConfig {
        &self.config
    }

    // ==================== MODULE CLIENT ACCESSORS ====================

    #[cfg(feature = "market")]
    pub fn market(&self) -> market::MarketClient {
        market::MarketClient::new(self.config.clone(), self.transport.clone())
    }

    #[cfg(feature = "vc")]
    pub fn vc(&self) -> vc::VcClient {
        vc::VcClient::new(self.config.clone(), self.transport.clone())
    }

    #[cfg(feature = "auth")]
    pub fn auth(&self) -> auth::AuthClient {
        auth::AuthClient::new(self.config.clone(), self.transport.clone())
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
/// ```rust
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

    // Feature-gated module re-exports
    #[cfg(feature = "market")]
    pub use super::market::MarketClient;

    #[cfg(feature = "vc")]
    pub use super::vc::VcClient;

    #[cfg(feature = "auth")]
    pub use super::auth::AuthClient;
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
        let _sdk2 = agent(AgentSigner::new(&[0u8; 32], AccountId::new([0u8; 32]), None));
    }

    #[test]
    fn prelude_compiles_cleanly() {
        use prelude::*;
        let _ = VERSION;
    }
}