//! Recommended prelude for the Morpheum Native SDK.
//!
//! This prelude is designed for maximum ergonomics in native Rust applications
//! (CLI tools, trading bots, autonomous agents).
//!
//! Most users should start their files with:
//!
//! ```rust
//! use morpheum_sdk_native::prelude::*;
//! ```

// ==================== CORE SDK FOUNDATION ====================

/// Re-export the most commonly used items from the core SDK.
pub use morpheum_sdk_core::{
    AccountId,
    ChainId,
    SdkConfig,
    SdkError,
    SignedTx,
};

// ==================== NATIVE FACADE ====================

/// The main SDK facade.
pub use crate::MorpheumSdk;

/// Convenience constructors for human and agent signers.
pub use crate::{native, agent};

// ==================== SIGNING LIBRARY ====================

/// Primary signers and key signing types.
pub use morpheum_signing_native::{
    NativeSigner,
    AgentSigner,
    TradingKeyClaim,
    VcClaimBuilder,
    PublicKey,
    Signature,
    WalletType,
};

// ==================== TRANSACTION BUILDING ====================

/// The canonical `Any` type (from `prost_types`) used in `TxBuilder.add_message()`.
pub use morpheum_sdk_core::signing::Any;

/// Transaction builder for constructing and signing Morpheum transactions.
pub use morpheum_sdk_core::builder::TxBuilder;

// ==================== FEATURE-GATED MODULE CLIENTS ====================

/// Market module client (only available when the `market` feature is enabled).
#[cfg(feature = "market")]
pub use morpheum_sdk_market::MarketClient;

/// VC module client (only available when the `vc` feature is enabled).
#[cfg(feature = "vc")]
pub use morpheum_sdk_vc::VcClient;

/// Auth module client (only available when the `auth` feature is enabled).
#[cfg(feature = "auth")]
pub use morpheum_sdk_auth::AuthClient;

// ==================== COMMON TYPE ALIASES ====================

/// Convenient alias for the native SDK's signed transaction type.
pub type SdkSignedTx = SignedTx;

/// Convenient alias for the signing library's raw `AccountId`.
pub type SigningAccountId = morpheum_signing_native::AccountId;

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    // This test ensures the prelude can be imported cleanly without conflicts
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn prelude_imports_cleanly() {
        // Basic sanity check
        let _ = native(NativeSigner::from_seed(&[0u8; 32]));
        let _ = agent(AgentSigner::new(&[0u8; 32], SigningAccountId([0u8; 32]), None));
    }
}