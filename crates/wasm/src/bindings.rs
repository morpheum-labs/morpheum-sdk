//! WASM bindings — the main `MorpheumSdkWasm` facade exposed to JavaScript/TypeScript.
//!
//! This module provides the top-level API surface for browser applications.
//! All heavy logic is delegated to the core SDK and signing library.

use wasm_bindgen::prelude::*;

/// The main WASM entry point for the Morpheum SDK.
///
/// Exposed to JavaScript as `MorpheumSdkWasm`.
///
/// ```ts
/// const sdk = await MorpheumSdkWasm.newMetamask("https://sentry.morpheum.xyz");
/// ```
#[wasm_bindgen]
pub struct MorpheumSdkWasm {
    _config: morpheum_sdk_core::SdkConfig,
}

#[wasm_bindgen]
impl MorpheumSdkWasm {
    /// Creates a new SDK instance configured with the given sentry URL and chain ID.
    #[wasm_bindgen(constructor)]
    pub fn new(sentry_url: &str, chain_id: &str) -> Self {
        Self {
            _config: morpheum_sdk_core::SdkConfig::new(sentry_url, chain_id),
        }
    }

    /// Returns the SDK version.
    #[wasm_bindgen(getter)]
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }
}
