//! WASM Bindings — All JavaScript/TypeScript interop for the browser.
//!
//! This is the main binding file for the Morpheum WASM SDK. It provides a clean,
//! TypeScript-friendly API with excellent DX (rich type definitions, JSDoc,
//! async wallet factories, and fluent builders).
//!
//! Key design decisions:
//! - Async factory methods (`newMetamask()`, `newPhantom()`, `newTaproot()`)
//! - `TxBuilderWasm` is the primary signing entry point (not direct `sign()` on SDK)
//! - Full support for feature-gated modules (market, vc, auth)
//! - Rich `VcClaimBuilderWasm` for agent delegation flows
//! - Comprehensive `typescript_custom_section` for outstanding TypeScript experience

use alloc::string::String;
use alloc::vec::Vec;

use wasm_bindgen::prelude::*;

use morpheum_sdk_core::{AccountId, SdkConfig, SdkError};

// Re-export adapters
pub use crate::adapters::{MetaMaskAdapterWasm, PhantomAdapterWasm, TaprootAdapterWasm, WasmSigner};

// ==================== MAIN WASM SDK FACADE ====================

/// Main WASM SDK facade for browser applications (React, Vue, Svelte, Next.js, etc.).
///
/// Created via async factory methods that connect to injected wallets.
#[wasm_bindgen]
pub struct MorpheumSdkWasm {
    config: SdkConfig,
    // The underlying signer (static dispatch via enum for zero-cost)
    #[wasm_bindgen(skip)]
    signer: WasmSigner,
}

#[wasm_bindgen]
impl MorpheumSdkWasm {
    // ==================== ASYNC WALLET FACTORIES ====================

    /// Creates a new SDK instance connected to **MetaMask** (or any EVM wallet).
    #[wasm_bindgen(js_name = "newMetamask")]
    pub async fn new_metamask(rpc_endpoint: String) -> Result<MorpheumSdkWasm, JsValue> {
        let adapter = MetaMaskAdapterWasm::connect()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let config = SdkConfig::new(rpc_endpoint, "morpheum-1");

        Ok(Self {
            config,
            signer: WasmSigner::MetaMask(adapter),
        })
    }

    /// Creates a new SDK instance connected to **Phantom** (or any Solana wallet).
    #[wasm_bindgen(js_name = "newPhantom")]
    pub async fn new_phantom(rpc_endpoint: String) -> Result<MorpheumSdkWasm, JsValue> {
        let adapter = PhantomAdapterWasm::connect()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let config = SdkConfig::new(rpc_endpoint, "morpheum-1");

        Ok(Self {
            config,
            signer: WasmSigner::Phantom(adapter),
        })
    }

    /// Creates a new SDK instance connected to **Unisat / Leather / Xverse** (Bitcoin Taproot).
    #[wasm_bindgen(js_name = "newTaproot")]
    pub async fn new_taproot(rpc_endpoint: String) -> Result<MorpheumSdkWasm, JsValue> {
        let adapter = TaprootAdapterWasm::connect()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let config = SdkConfig::new(rpc_endpoint, "morpheum-1");

        Ok(Self {
            config,
            signer: WasmSigner::Taproot(adapter),
        })
    }

    // ==================== MODULE CLIENTS (feature-gated) ====================

    /// Returns the Market client (only available with "market" feature).
    #[cfg(feature = "market")]
    #[wasm_bindgen]
    pub fn market(&self) -> morpheum_sdk_market::MarketClient {
        morpheum_sdk_market::MarketClient::new(self.config.clone(), Box::new(self.signer.clone()))
    }

    /// Returns the VC client (only available with "vc" feature).
    #[cfg(feature = "vc")]
    #[wasm_bindgen]
    pub fn vc(&self) -> morpheum_sdk_vc::VcClient {
        morpheum_sdk_vc::VcClient::new(self.config.clone(), Box::new(self.signer.clone()))
    }

    /// Returns the Auth client (only available with "auth" feature).
    #[cfg(feature = "auth")]
    #[wasm_bindgen]
    pub fn auth(&self) -> morpheum_sdk_auth::AuthClient {
        morpheum_sdk_auth::AuthClient::new(self.config.clone(), Box::new(self.signer.clone()))
    }
}

// ==================== VC CLAIM BUILDER FOR JS ====================

/// Fluent builder for creating `TradingKeyClaim` from JavaScript/TypeScript.
///
/// This is the primary way to build agent delegation claims in the browser.
#[wasm_bindgen(js_name = "VcClaimBuilder")]
pub struct VcClaimBuilderWasm {
    inner: morpheum_sdk_core::signing::VcClaimBuilder,
}

#[wasm_bindgen(js_class = "VcClaimBuilder")]
impl VcClaimBuilderWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: morpheum_sdk_core::signing::VcClaimBuilder::new(),
        }
    }

    #[wasm_bindgen]
    pub fn issuer(mut self, bytes: Vec<u8>) -> Result<VcClaimBuilderWasm, JsValue> {
        let arr: [u8; 32] = bytes.try_into()
            .map_err(|_| JsValue::from_str("issuer must be exactly 32 bytes"))?;
        self.inner = self.inner.issuer(AccountId::new(arr));
        Ok(self)
    }

    #[wasm_bindgen]
    pub fn subject(mut self, bytes: Vec<u8>) -> Result<VcClaimBuilderWasm, JsValue> {
        let arr: [u8; 32] = bytes.try_into()
            .map_err(|_| JsValue::from_str("subject must be exactly 32 bytes"))?;
        self.inner = self.inner.subject(AccountId::new(arr));
        Ok(self)
    }

    #[wasm_bindgen]
    pub fn permissions(mut self, perms: u64) -> VcClaimBuilderWasm {
        self.inner = self.inner.permissions(perms);
        self
    }

    #[wasm_bindgen(js_name = "maxDailyUsd")]
    pub fn max_daily_usd(mut self, amount: u64) -> VcClaimBuilderWasm {
        self.inner = self.inner.max_daily_usd(amount);
        self
    }

    #[wasm_bindgen]
    pub fn expiry(mut self, timestamp: u64) -> VcClaimBuilderWasm {
        self.inner = self.inner.expiry(timestamp);
        self
    }

    #[wasm_bindgen(js_name = "nonceSubRange")]
    pub fn nonce_sub_range(mut self, start: u32, end: u32) -> VcClaimBuilderWasm {
        self.inner = self.inner.nonce_sub_range(start, end);
        self
    }

    #[wasm_bindgen]
    pub fn signature(
        mut self,
        sig_bytes: Vec<u8>,
        sig_type: String,
    ) -> Result<VcClaimBuilderWasm, JsValue> {
        let arr: [u8; 64] = sig_bytes.try_into()
            .map_err(|_| JsValue::from_str("signature must be exactly 64 bytes"))?;

        let sig = match sig_type.as_str() {
            "ed25519" => morpheum_sdk_core::signing::Signature::Ed25519(arr),
            "secp256k1" => morpheum_sdk_core::signing::Signature::Secp256k1(arr),
            "schnorr" => morpheum_sdk_core::signing::Signature::Schnorr(arr),
            _ => return Err(JsValue::from_str("signature_type must be 'ed25519', 'secp256k1', or 'schnorr'")),
        };

        self.inner = self.inner.signature(sig);
        Ok(self)
    }

    #[wasm_bindgen]
    pub fn build(self, current_timestamp: u64) -> Result<JsValue, JsValue> {
        let claim = self.inner.build(current_timestamp)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let obj = js_sys::Object::new();

        js_sys::Reflect::set(
            &obj,
            &"issuer".into(),
            &js_sys::Uint8Array::from(claim.issuer.as_bytes()).into(),
        ).map_err(|_| JsValue::from_str("failed to set issuer"))?;

        js_sys::Reflect::set(
            &obj,
            &"subject".into(),
            &js_sys::Uint8Array::from(claim.subject.as_bytes()).into(),
        ).map_err(|_| JsValue::from_str("failed to set subject"))?;

        js_sys::Reflect::set(
            &obj,
            &"permissions".into(),
            &JsValue::from(claim.permissions as f64),
        ).map_err(|_| JsValue::from_str("failed to set permissions"))?;

        js_sys::Reflect::set(
            &obj,
            &"max_daily_usd".into(),
            &JsValue::from(claim.max_daily_usd as f64),
        ).map_err(|_| JsValue::from_str("failed to set max_daily_usd"))?;

        js_sys::Reflect::set(
            &obj,
            &"expiry_timestamp".into(),
            &JsValue::from(claim.expiry_timestamp as f64),
        ).map_err(|_| JsValue::from_str("failed to set expiry_timestamp"))?;

        js_sys::Reflect::set(
            &obj,
            &"nonce_sub_range_start".into(),
            &JsValue::from(claim.nonce_sub_range_start),
        ).map_err(|_| JsValue::from_str("failed to set nonce_sub_range_start"))?;

        js_sys::Reflect::set(
            &obj,
            &"nonce_sub_range_end".into(),
            &JsValue::from(claim.nonce_sub_range_end),
        ).map_err(|_| JsValue::from_str("failed to set nonce_sub_range_end"))?;

        // Add proto_any for direct embedding in transactions
        let any = claim.to_proto_any();
        js_sys::Reflect::set(
            &obj,
            &"proto_any_type_url".into(),
            &JsValue::from_str(&any.type_url),
        ).map_err(|_| JsValue::from_str("failed to set proto_any_type_url"))?;

        js_sys::Reflect::set(
            &obj,
            &"proto_any_value".into(),
            &js_sys::Uint8Array::from(any.value.as_slice()).into(),
        ).map_err(|_| JsValue::from_str("failed to set proto_any_value"))?;

        Ok(obj.into())
    }
}

// ==================== RICH TYPESCRIPT DEFINITIONS ====================

#[wasm_bindgen(typescript_custom_section)]
const TS_TYPES: &'static str = r#"
/**
 * Morpheum SDK — TypeScript Definitions (WASM)
 *
 * Full, rich type definitions for browser usage with excellent DX.
 */

export interface SignedTx {
    raw_bytes: Uint8Array;
    txhash: string;
    tx_raw_bytes?: Uint8Array;
}

export class MorpheumSdkWasm {
    private constructor();

    static newMetamask(rpc_endpoint: string): Promise<MorpheumSdkWasm>;
    static newPhantom(rpc_endpoint: string): Promise<MorpheumSdkWasm>;
    static newTaproot(rpc_endpoint: string): Promise<MorpheumSdkWasm>;

    market(): any;   // MarketClient when feature enabled
    vc(): any;       // VcClient when feature enabled
    auth(): any;     // AuthClient when feature enabled
}

export class VcClaimBuilder {
    constructor();
    issuer(bytes: Uint8Array): VcClaimBuilder;
    subject(bytes: Uint8Array): VcClaimBuilder;
    permissions(perms: number): VcClaimBuilder;
    maxDailyUsd(amount: number): VcClaimBuilder;
    expiry(timestamp: number): VcClaimBuilder;
    nonceSubRange(start: number, end: number): VcClaimBuilder;
    signature(sig_bytes: Uint8Array, sig_type: "ed25519" | "secp256k1" | "schnorr"): VcClaimBuilder;
    build(current_timestamp: number): any;  // Rich claim object with proto_any
}
"#;