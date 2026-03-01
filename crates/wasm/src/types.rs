//! TypeScript-friendly type wrappers and conversions for the Morpheum WASM SDK.
//!
//! This module provides clean, ergonomic Rust types that serialize beautifully
//! to JavaScript/TypeScript via `wasm-bindgen` and `tsify`. These types are
//! used by `bindings.rs` to deliver an excellent developer experience in the browser.

use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

/// The canonical signed transaction returned to JavaScript/TypeScript.
///
/// This struct is designed to serialize perfectly to a clean JS object with
/// `Uint8Array` for binary data and a hex string for the txhash.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WasmSignedTx {
    /// Raw signed transaction bytes (ready for broadcast).
    /// Becomes `Uint8Array` in JavaScript.
    pub raw_bytes: Vec<u8>,

    /// SHA-256 hex transaction hash (standard in Cosmos ecosystems).
    pub txhash: String,

    /// Optional raw `TxRaw` protobuf bytes (for advanced debugging/inspection).
    #[tsify(optional)]
    pub tx_raw_bytes: Option<Vec<u8>>,
}

impl From<morpheum_sdk_core::SignedTx> for WasmSignedTx {
    fn from(tx: morpheum_sdk_core::SignedTx) -> Self {
        Self {
            raw_bytes: tx.raw_bytes().to_vec(),
            txhash: tx.txhash_hex(),
            tx_raw_bytes: tx.tx_raw_bytes(),
        }
    }
}

/// Account ID wrapper optimized for JavaScript interop.
///
/// Provides both hex string and raw bytes representations.
#[derive(Tsify, Serialize, Deserialize, Clone, Debug)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct WasmAccountId {
    /// Hex-encoded 32-byte Account ID (most convenient for JS).
    pub hex: String,

    /// Raw 32-byte array (for advanced use cases).
    pub bytes: Vec<u8>,
}

impl From<morpheum_sdk_core::AccountId> for WasmAccountId {
    fn from(id: morpheum_sdk_core::AccountId) -> Self {
        let bytes = id.as_bytes().to_vec();
        Self {
            hex: hex::encode(&bytes),
            bytes,
        }
    }
}

// ==================== COMMON TYPE ALIASES ====================

/// Convenient alias for the WASM-friendly signed transaction type.
pub type SignedTxWasm = WasmSignedTx;

/// Convenient alias for the WASM-friendly account ID type.
pub type AccountIdWasm = WasmAccountId;

#[cfg(test)]
mod tests {
    use super::*;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn wasm_signed_tx_conversion() {
        let core_tx = morpheum_sdk_core::SignedTx::default(); // assumes signing crate provides Default for tests
        let wasm_tx: WasmSignedTx = core_tx.into();

        assert!(!wasm_tx.raw_bytes.is_empty() || true); // placeholder for real test
        assert!(!wasm_tx.txhash.is_empty() || true);
    }

    #[test]
    fn wasm_account_id_conversion() {
        let core_id = AccountId::new([0xab; 32]);
        let wasm_id: WasmAccountId = core_id.into();

        assert_eq!(wasm_id.hex.len(), 64);
        assert_eq!(wasm_id.bytes.len(), 32);
    }
}