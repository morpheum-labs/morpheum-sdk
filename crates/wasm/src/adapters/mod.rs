//! WASM wallet adapter module.
//!
//! This module declares the three browser wallet adapters and provides
//! a unified `WasmSigner` enum for **static dispatch** inside `TxBuilderWasm`.
//! This avoids the overhead of `Box<dyn Signer>` in the WASM boundary while
//! keeping the API clean and zero-cost.

pub mod metamask;
pub mod phantom;
pub mod taproot;

pub use metamask::MetaMaskAdapterWasm;
pub use phantom::PhantomAdapterWasm;
pub use taproot::TaprootAdapterWasm;

use async_trait::async_trait;

use morpheum_sdk_core::{
    error::SigningError,
    proto::tx::v1::SignDoc,
    signer::Signer,
    types::{AccountId, PublicKey, Signature},
};

// ==================== WASM SIGNER DISPATCH ENUM ====================

/// Static-dispatch signer enum for all supported WASM wallet adapters.
///
/// Used by `TxBuilderWasm` instead of `Box<dyn Signer>` to enable zero-cost
/// dispatch and better tree-shaking in WASM builds.
pub(crate) enum WasmSigner {
    /// MetaMask / Rabby / any EVM injected wallet.
    MetaMask(MetaMaskAdapterWasm),
    /// Phantom / Solflare / any Solana injected wallet.
    Phantom(PhantomAdapterWasm),
    /// Unisat / Leather / Xverse — Bitcoin Taproot injected wallet.
    Taproot(TaprootAdapterWasm),
}

// SAFETY: WASM (wasm32-unknown-unknown) is strictly single-threaded by specification.
// `RefCell` is used inside each adapter, but no concurrent access is possible.
// Therefore `Send + Sync` is sound.
unsafe impl Send for WasmSigner {}
unsafe impl Sync for WasmSigner {}

#[async_trait(?Send)]
impl Signer for WasmSigner {
    async fn sign(&self, sign_doc: &SignDoc) -> Result<Signature, SigningError> {
        match self {
            Self::MetaMask(a) => a.sign_impl(sign_doc).await,
            Self::Phantom(a) => a.sign_impl(sign_doc).await,
            Self::Taproot(a) => a.sign_impl(sign_doc).await,
        }
    }

    fn public_key(&self) -> PublicKey {
        match self {
            Self::MetaMask(a) => a.public_key(),
            Self::Phantom(a) => a.public_key(),
            Self::Taproot(a) => a.public_key(),
        }
    }

    fn public_key_proto(&self) -> prost_types::Any {
        match self {
            Self::MetaMask(a) => a.public_key_proto(),
            Self::Phantom(a) => a.public_key_proto(),
            Self::Taproot(a) => a.public_key_proto(),
        }
    }

    fn account_id(&self) -> AccountId {
        match self {
            Self::MetaMask(a) => a.account_id(),
            Self::Phantom(a) => a.account_id(),
            Self::Taproot(a) => a.account_id(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wasm_signer_enum_compiles() {
        // Compile-time test to ensure the enum and trait impl are valid
        let _ = std::mem::size_of::<WasmSigner>();
    }
}