//! Taproot (Bitcoin) injected wallet adapter for WASM/browser environments.
//!
//! This adapter connects to `window.unisat` (compatible with Unisat, Leather,
//! Xverse, and other Taproot wallets), requests account access, and signs
//! using BIP-322-simple `signMessage`.
//!
//! Uses `RefCell` for interior mutability of the cached x-only Schnorr public
//! key and address — safe and idiomatic in single-threaded WASM.

use std::cell::RefCell;

use js_sys::{Object, Reflect};
use prost::Message;
use sha2::{Digest, Sha256};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

use morpheum_sdk_core::{
    error::SigningError,
    proto::tx::v1::SignDoc,
    signer::Signer,
    types::{AccountId, PublicKey, Signature},
};

// ==================== JS INTEROP ====================

#[wasm_bindgen]
extern "C" {
    /// Opaque handle to the injected `window.unisat` provider.
    type Unisat;

    #[wasm_bindgen(method, js_name = "getPublicKey")]
    fn get_public_key(this: &Unisat) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "requestAccounts")]
    fn request_accounts(this: &Unisat) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "signMessage")]
    fn sign_message(this: &Unisat, message: &str, options: &JsValue) -> js_sys::Promise;
}

/// Returns the injected `window.unisat` provider or a clear error.
fn get_unisat() -> Result<Unisat, SigningError> {
    let window = web_sys::window()
        .ok_or_else(|| SigningError::wallet_adapter("No window object (not running in a browser)"))?;

    let unisat = Reflect::get(&window, &JsValue::from_str("unisat"))
        .map_err(|_| SigningError::wallet_adapter("Failed to access window.unisat"))?;

    if unisat.is_undefined() || unisat.is_null() {
        return Err(SigningError::wallet_adapter(
            "Unisat (or compatible Taproot wallet) not detected. Please install Unisat or Leather.",
        ));
    }

    Ok(unisat.unchecked_into::<Unisat>())
}

// ==================== ADAPTER ====================

/// Taproot (Bitcoin) injected wallet adapter for the Morpheum WASM SDK.
pub struct TaprootAdapterWasm {
    /// Cached x-only Schnorr public key (32 bytes, BIP-340).
    ///
    /// SAFETY: `RefCell` is `!Sync`, but WASM is strictly single-threaded,
    /// so this is sound and idiomatic for browser wallet adapters.
    cached_pubkey: RefCell<[u8; 32]>,

    /// Cached Taproot address (bc1p...).
    cached_address: RefCell<String>,
}

// SAFETY: WASM (wasm32-unknown-unknown) is single-threaded by specification.
// No concurrent access is possible.
unsafe impl Send for TaprootAdapterWasm {}
unsafe impl Sync for TaprootAdapterWasm {}

impl TaprootAdapterWasm {
    /// Connects to the Taproot wallet, requests account access, and caches
    /// the x-only public key and address.
    ///
    /// This is the only public constructor — ensures the wallet is connected
    /// before any signing can occur.
    pub async fn connect() -> Result<Self, SigningError> {
        let unisat = get_unisat()?;

        // Request account access
        let accounts_promise = unisat.request_accounts();
        let accounts_result = JsFuture::from(accounts_promise)
            .await
            .map_err(|e| SigningError::wallet_adapter(format!("Unisat requestAccounts failed: {:?}", e)))?;

        let accounts = js_sys::Array::from(&accounts_result);
        let address = accounts
            .get(0)
            .as_string()
            .ok_or_else(|| SigningError::wallet_adapter("Unisat returned no accounts"))?;

        // Fetch the x-only public key (64 hex chars = 32 bytes)
        let pk_promise = unisat.get_public_key();
        let pk_result = JsFuture::from(pk_promise)
            .await
            .map_err(|e| SigningError::wallet_adapter(format!("Unisat getPublicKey failed: {:?}", e)))?;

        let pk_hex: String = pk_result
            .as_string()
            .ok_or_else(|| SigningError::wallet_adapter("Unisat getPublicKey returned non-string"))?;

        let pk_bytes = hex::decode(&pk_hex)
            .map_err(|e| SigningError::wallet_adapter(format!("Invalid public key hex from Unisat: {}", e)))?;

        // Accept both 32-byte (x-only) and 33-byte (compressed) formats
        let mut pubkey = [0u8; 32];
        match pk_bytes.len() {
            32 => pubkey.copy_from_slice(&pk_bytes),
            33 => pubkey.copy_from_slice(&pk_bytes[1..]), // strip prefix byte
            other => {
                return Err(SigningError::wallet_adapter(format!(
                    "Unisat returned unexpected public key length: {} (expected 32 or 33)",
                    other
                )));
            }
        }

        Ok(Self {
            cached_pubkey: RefCell::new(pubkey),
            cached_address: RefCell::new(address),
        })
    }

    /// Signs the canonical `SignDoc` using BIP-322-simple `signMessage` via Unisat.
    pub(crate) async fn sign_impl(&self, sign_doc: &SignDoc) -> Result<Signature, SigningError> {
        let unisat = get_unisat()?;

        let message = Self::build_sign_message(sign_doc);

        let options = Object::new();
        Reflect::set(
            &options,
            &JsValue::from_str("type"),
            &JsValue::from_str("bip322-simple"),
        )
            .map_err(|_| SigningError::wallet_adapter("Failed to set BIP-322 signing options"))?;

        let promise = unisat.sign_message(&message, &options);
        let result = JsFuture::from(promise)
            .await
            .map_err(|e| SigningError::wallet_adapter(format!("Unisat signMessage failed: {:?}", e)))?;

        let sig_str = if let Some(s) = result.as_string() {
            s
        } else {
            let nested = Reflect::get(&result, &JsValue::from_str("signature"))
                .map_err(|_| SigningError::wallet_adapter("Unisat response missing 'signature'"))?;
            nested
                .as_string()
                .ok_or_else(|| SigningError::wallet_adapter("Unisat signature is not a string"))?
        };

        // Decode hex (0x-prefixed) or raw hex
        let sig_bytes = if let Some(hex_str) = sig_str.strip_prefix("0x") {
            hex::decode(hex_str)
                .map_err(|e| SigningError::wallet_adapter(format!("Invalid hex signature: {}", e)))?
        } else {
            hex::decode(&sig_str).unwrap_or_else(|_| sig_str.as_bytes().to_vec())
        };

        if sig_bytes.len() != 64 {
            return Err(SigningError::wallet_adapter(format!(
                "Unisat returned invalid Schnorr signature length: {} (expected 64)",
                sig_bytes.len()
            )));
        }

        let mut arr = [0u8; 64];
        arr.copy_from_slice(&sig_bytes);
        Ok(Signature::Schnorr(arr))
    }

    /// Returns the cached BIP-340 x-only Schnorr public key.
    pub(crate) fn public_key(&self) -> PublicKey {
        PublicKey::Schnorr(*self.cached_pubkey.borrow())
    }

    /// Returns the protobuf-encoded public key for `SignerInfo`.
    pub(crate) fn public_key_proto(&self) -> morpheum_sdk_core::signing::Any {
        morpheum_sdk_core::signing::Any {
            type_url: "/morpheum.crypto.schnorr.PubKey".to_string(),
            value: self.cached_pubkey.borrow().to_vec(),
        }
    }

    /// Derives the `AccountId` from the cached Taproot address.
    pub(crate) fn account_id(&self) -> AccountId {
        let addr = self.cached_address.borrow().clone();
        // In real SDK, this would use proper Taproot address hashing
        // For now we use a direct mapping consistent with core
        AccountId::new(Sha256::digest(addr.as_bytes()).into())
    }

    // ==================== PRIVATE HELPERS ====================

    /// Builds a clear, human-readable message for BIP-322-simple `signMessage`.
    ///
    /// Format: `"Morpheum SignDoc v1\n" || sha256_hex(SignDoc)`
    fn build_sign_message(sign_doc: &SignDoc) -> String {
        let bytes = sign_doc.encode_to_vec();
        let hash = Sha256::digest(bytes);
        format!("Morpheum SignDoc v1\n{}", hex::encode(hash))
    }
}