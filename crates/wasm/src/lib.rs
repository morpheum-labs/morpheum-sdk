//! WASM + TypeScript bindings for the Morpheum SDK.
//!
//! This crate provides browser-ready bindings for React, Vue, Svelte, Next.js,
//! and other web applications. It exposes a clean JavaScript/TypeScript API
//! while delegating all heavy logic to the core SDK, module crates, and signing library.
//!
//! **Recommended usage in TypeScript:**
//! ```ts
//! import { MorpheumSdkWasm, setPanicHook } from '@morpheum/sdk';
//!
//! setPanicHook();
//! const sdk = await MorpheumSdkWasm.newMetamask("https://sentry.morpheum.xyz", "morm-1");
//! const result = await sdk.createBucket({
//!   bucketId: "my-bucket",
//!   bucketType: "cross",
//!   collateralAssetIndex: 1,
//!   initialMargin: "1000000000",
//! });
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

mod bindings;

// ==================== WASM SETUP ====================

/// Installs a better panic hook for improved error messages in the browser console.
///
/// Call this once at the start of your application:
/// ```ts
/// import { setPanicHook } from '@morpheum/sdk';
/// setPanicHook();
/// ```
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

// ==================== VERSION ====================

/// Current version of the Morpheum WASM SDK.
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").into()
}

// ==================== RE-EXPORTS ====================

pub use crate::bindings::*;

#[cfg(feature = "morpheum-sdk-market")]
pub use morpheum_sdk_market as market;

#[cfg(feature = "morpheum-sdk-vc")]
pub use morpheum_sdk_vc as vc;

#[cfg(feature = "morpheum-sdk-auth")]
pub use morpheum_sdk_auth as auth;

pub use morpheum_sdk_core::{
    AccountId,
    ChainId,
    SdkError,
    SignedTx,
};

pub use morpheum_sdk_core::signing::claim::{TradingKeyClaim, VcClaimBuilder};

// ==================== WASM-SPECIFIC PRELUDE ====================

/// Recommended prelude for WASM/TypeScript usage.
pub mod prelude {
    pub use super::{
        set_panic_hook,
        version,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
        TradingKeyClaim,
        VcClaimBuilder,
    };

    pub use super::bindings::MorpheumSdkWasm;
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn version_is_set() {
        assert!(!version().is_empty());
    }

    #[test]
    fn prelude_compiles_cleanly() {
        use prelude::*;
        let _ = version();
    }
}
