//! MetaMask (EVM) injected wallet adapter for WASM/browser environments.
//!
//! This adapter connects to `window.ethereum`, requests account access,
//! and signs transactions using the secure EIP-712 typed data standard
//! (`eth_signTypedData_v4`).
//!
//! Uses `RefCell` for interior mutability of the cached EVM address —
//! safe in single-threaded WASM environments.

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
    /// Opaque handle to the injected `window.ethereum` provider.
    type Ethereum;

    #[wasm_bindgen(method, js_name = "request")]
    fn request(this: &Ethereum, params: &JsValue) -> js_sys::Promise;
}

/// Returns the injected `window.ethereum` provider or a clear error.
fn get_ethereum() -> Result<Ethereum, SigningError> {
    let window = web_sys::window()
        .ok_or_else(|| SigningError::wallet_adapter("No window object (not running in a browser)"))?;

    let ethereum = Reflect::get(&window, &JsValue::from_str("ethereum"))
        .map_err(|_| SigningError::wallet_adapter("Failed to access window.ethereum"))?;

    if ethereum.is_undefined() || ethereum.is_null() {
        return Err(SigningError::wallet_adapter(
            "MetaMask (or compatible EVM wallet) not detected. Please install MetaMask.",
        ));
    }

    Ok(ethereum.unchecked_into::<Ethereum>())
}

// ==================== ADAPTER ====================

/// MetaMask (EVM) injected wallet adapter for the Morpheum WASM SDK.
pub struct MetaMaskAdapterWasm {
    /// Cached EVM address (20 bytes). Updated on reconnection.
    ///
    /// SAFETY: `RefCell` is `!Sync`, but WASM is strictly single-threaded,
    /// so this is sound and idiomatic for browser adapters.
    cached_address: RefCell<[u8; 20]>,

    /// Hex address string (0x...) for passing back to MetaMask APIs.
    address_hex: String,
}

// SAFETY: WASM (wasm32-unknown-unknown) is single-threaded by specification.
// No concurrent access is possible.
unsafe impl Send for MetaMaskAdapterWasm {}
unsafe impl Sync for MetaMaskAdapterWasm {}

impl MetaMaskAdapterWasm {
    /// Connects to MetaMask, requests account access, and caches the first address.
    ///
    /// This is the only public constructor — ensures the wallet is connected
    /// before any signing can occur.
    pub async fn connect() -> Result<Self, SigningError> {
        let ethereum = get_ethereum()?;

        // Request account access (triggers MetaMask popup)
        let request_obj = Object::new();
        Reflect::set(
            &request_obj,
            &JsValue::from_str("method"),
            &JsValue::from_str("eth_requestAccounts"),
        )
            .map_err(|_| SigningError::wallet_adapter("Failed to build eth_requestAccounts payload"))?;

        let promise = ethereum.request(&request_obj.into());
        let result = JsFuture::from(promise)
            .await
            .map_err(|e| SigningError::wallet_adapter(format!("eth_requestAccounts failed: {:?}", e)))?;

        // Extract first account
        let accounts = js_sys::Array::from(&result);
        let first_account = accounts
            .get(0)
            .as_string()
            .ok_or_else(|| SigningError::wallet_adapter("MetaMask returned no accounts"))?;

        let hex_str = first_account.strip_prefix("0x").unwrap_or(&first_account);
        let mut address_bytes = [0u8; 20];
        hex::decode_to_slice(hex_str, &mut address_bytes)
            .map_err(|e| SigningError::wallet_adapter(format!("Invalid EVM address from MetaMask: {}", e)))?;

        Ok(Self {
            cached_address: RefCell::new(address_bytes),
            address_hex: first_account,
        })
    }

    /// Signs the canonical `SignDoc` using EIP-712 typed data via MetaMask.
    pub(crate) async fn sign_impl(&self, sign_doc: &SignDoc) -> Result<Signature, SigningError> {
        let ethereum = get_ethereum()?;

        let payload = self.build_eip712_payload(sign_doc)?;

        let params = js_sys::Array::of2(&JsValue::from_str(&self.address_hex), &payload);
        let request_obj = Object::new();

        Reflect::set(
            &request_obj,
            &JsValue::from_str("method"),
            &JsValue::from_str("eth_signTypedData_v4"),
        )
            .map_err(|_| SigningError::wallet_adapter("Failed to set eth_signTypedData_v4 method"))?;

        Reflect::set(&request_obj, &JsValue::from_str("params"), &params)
            .map_err(|_| SigningError::wallet_adapter("Failed to set params for eth_signTypedData_v4"))?;

        let promise = ethereum.request(&request_obj.into());
        let result = JsFuture::from(promise)
            .await
            .map_err(|e| SigningError::wallet_adapter(format!("eth_signTypedData_v4 failed: {:?}", e)))?;

        let sig_hex: String = result
            .as_string()
            .ok_or_else(|| SigningError::wallet_adapter("MetaMask returned non-string signature"))?;

        let sig_hex = sig_hex.strip_prefix("0x").unwrap_or(&sig_hex);
        let sig_bytes = hex::decode(sig_hex)
            .map_err(|e| SigningError::wallet_adapter(format!("Invalid signature hex: {}", e)))?;

        if sig_bytes.len() < 64 {
            return Err(SigningError::wallet_adapter(format!(
                "MetaMask returned signature too short: {} bytes (expected ≥64)",
                sig_bytes.len()
            )));
        }

        let mut arr = [0u8; 64];
        arr.copy_from_slice(&sig_bytes[0..64]);
        Ok(Signature::Secp256k1(arr))
    }

    /// Returns the cached EVM public key placeholder.
    ///
    /// MetaMask does not expose the raw secp256k1 public key. We return a
    /// placeholder; the chain recovers the real key via ecrecover during verification.
    pub(crate) fn public_key(&self) -> PublicKey {
        let addr = *self.cached_address.borrow();
        let mut key = [0u8; 33];
        key[0] = 0x02; // compressed even-y prefix
        key[1..21].copy_from_slice(&addr);
        PublicKey::Secp256k1(key)
    }

    /// Returns the protobuf-encoded public key for `SignerInfo`.
    pub(crate) fn public_key_proto(&self) -> morpheum_sdk_core::signing::Any {
        morpheum_sdk_core::signing::Any {
            type_url: "/cosmos.crypto.secp256k1.PubKey".to_string(),
            value: self.cached_address.borrow().to_vec(),
        }
    }

    /// Derives the `AccountId` from the cached EVM address.
    pub(crate) fn account_id(&self) -> AccountId {
        let addr = *self.cached_address.borrow();
        AccountId::new(addr) // In real SDK, this would hash appropriately
    }

    // ==================== PRIVATE HELPERS ====================

    /// Builds a canonical EIP-712 typed data payload for the `SignDoc`.
    fn build_eip712_payload(&self, sign_doc: &SignDoc) -> Result<JsValue, SigningError> {
        let hash = Sha256::digest(sign_doc.encode_to_vec());
        let hash_hex = format!("0x{}", hex::encode(hash));

        let domain = Object::new();
        set_prop(&domain, "name", &JsValue::from_str("Morpheum"))?;
        set_prop(&domain, "version", &JsValue::from_str("1"))?;
        set_prop(&domain, "chainId", &JsValue::from(1))?;

        let message = Object::new();
        set_prop(&message, "signDocHash", &JsValue::from_str(&hash_hex))?;

        let types = Object::new();
        let domain_type = js_sys::Array::new();
        let name_field = Object::new();
        set_prop(&name_field, "name", &JsValue::from_str("name"))?;
        set_prop(&name_field, "type", &JsValue::from_str("string"))?;
        domain_type.push(&name_field);

        let message_type = js_sys::Array::new();
        let hash_field = Object::new();
        set_prop(&hash_field, "name", &JsValue::from_str("signDocHash"))?;
        set_prop(&hash_field, "type", &JsValue::from_str("string"))?;
        message_type.push(&hash_field);

        set_prop(&types, "EIP712Domain", &domain_type)?;
        set_prop(&types, "MorpheumSignDoc", &message_type)?;

        let payload = Object::new();
        set_prop(&payload, "types", &types)?;
        set_prop(&payload, "domain", &domain)?;
        set_prop(&payload, "primaryType", &JsValue::from_str("MorpheumSignDoc"))?;
        set_prop(&payload, "message", &message)?;

        js_sys::JSON::stringify(&payload)
            .map(Into::into)
            .map_err(|_| SigningError::wallet_adapter("Failed to stringify EIP-712 payload"))
    }
}

/// Tiny helper to reduce Reflect::set boilerplate.
fn set_prop(obj: &Object, key: &str, val: &JsValue) -> Result<(), SigningError> {
    Reflect::set(obj, &JsValue::from_str(key), val)
        .map_err(|_| SigningError::wallet_adapter(format!("Failed to set property '{}'", key)))?;
    Ok(())
}