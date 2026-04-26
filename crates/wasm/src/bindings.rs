//! WASM bindings — the main `MorpheumSdkWasm` facade exposed to JavaScript/TypeScript.
//!
//! Provides typed module builders (createBucket, transferBetweenBuckets, etc.)
//! that internally encode proto messages, build a signed transaction via the
//! shared WASM wallet adapters, and return broadcast-ready bytes.

use prost::Message;
use sha2::Digest;
use wasm_bindgen::prelude::*;
use serde::Deserialize;

use morpheum_signing_core::{
    signer::Signer,
    proto::tx::v1::{
        self as tx, AuthInfo, ModeInfo, Nonce, SignDoc, SignerInfo, Tx, TxBody, TxRaw,
    },
};
use morpheum_signing_wasm_lib::{
    MetaMaskAdapterWasm, PhantomAdapterWasm, WasmSigner,
};
use morpheum_sdk_bucket::requests::{
    CreateBucketRequest, TransferBetweenBucketsRequest, TransferToBankRequest,
};
use morpheum_sdk_bucket::types::BucketType;

// ==================== JS PARAM TYPES ====================

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CreateBucketParams {
    bucket_id: String,
    bucket_type: String,
    collateral_asset_index: u64,
    initial_margin: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransferBetweenBucketsParams {
    source_bucket_id: String,
    target_bucket_id: String,
    amount: String,
    reason: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TransferToBankParams {
    bucket_id: String,
    asset_index: u64,
    amount: String,
}

fn parse_bucket_type(s: &str) -> Result<BucketType, JsValue> {
    match s.to_lowercase().as_str() {
        "cross" | "1" => Ok(BucketType::Cross),
        "isolated" | "2" => Ok(BucketType::Isolated),
        other => Err(JsValue::from_str(&format!(
            "invalid bucketType: '{other}' (expected 'cross' or 'isolated')"
        ))),
    }
}

// ==================== MAIN SDK FACADE ====================

/// The main WASM entry point for the Morpheum SDK.
///
/// ```typescript
/// const sdk = await MorpheumSdkWasm.newMetamask("https://sentry.morpheum.xyz", "morm-1");
/// const result = await sdk.createBucket({
///   bucketId: "my-bucket",
///   bucketType: "cross",
///   collateralAssetIndex: 1,
///   initialMargin: "1000000000",
/// });
/// ```
#[wasm_bindgen]
pub struct MorpheumSdkWasm {
    _config: morpheum_sdk_core::SdkConfig,
    signer: WasmSigner,
}

#[wasm_bindgen]
impl MorpheumSdkWasm {
    /// Creates a new SDK instance backed by MetaMask (EVM injected wallet).
    #[wasm_bindgen(js_name = "newMetamask")]
    pub async fn new_metamask(sentry_url: &str, chain_id: &str) -> Result<MorpheumSdkWasm, JsValue> {
        let adapter = MetaMaskAdapterWasm::connect()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(Self {
            _config: morpheum_sdk_core::SdkConfig::new(sentry_url, chain_id),
            signer: WasmSigner::MetaMask(adapter),
        })
    }

    /// Creates a new SDK instance backed by Phantom (Solana injected wallet).
    #[wasm_bindgen(js_name = "newPhantom")]
    pub async fn new_phantom(sentry_url: &str, chain_id: &str) -> Result<MorpheumSdkWasm, JsValue> {
        let adapter = PhantomAdapterWasm::connect()
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(Self {
            _config: morpheum_sdk_core::SdkConfig::new(sentry_url, chain_id),
            signer: WasmSigner::Phantom(adapter),
        })
    }

    /// Returns the SDK version.
    #[wasm_bindgen(getter)]
    pub fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    // ==================== BUCKET MODULE ====================

    /// Creates a new margin bucket.
    #[wasm_bindgen(js_name = "createBucket")]
    pub async fn create_bucket(&self, params: JsValue) -> Result<JsValue, JsValue> {
        let p: CreateBucketParams = serde_wasm_bindgen::from_value(params)
            .map_err(|e| JsValue::from_str(&format!("invalid createBucket params: {e}")))?;
        let bucket_type = parse_bucket_type(&p.bucket_type)?;

        let address = self.signer_address();
        let req = CreateBucketRequest::new(
            &address,
            p.bucket_id,
            bucket_type,
            p.collateral_asset_index,
            p.initial_margin,
        );
        self.sign_and_return(req.to_any()).await
    }

    /// Transfers margin between two buckets.
    #[wasm_bindgen(js_name = "transferBetweenBuckets")]
    pub async fn transfer_between_buckets(&self, params: JsValue) -> Result<JsValue, JsValue> {
        let p: TransferBetweenBucketsParams = serde_wasm_bindgen::from_value(params)
            .map_err(|e| JsValue::from_str(&format!("invalid transferBetweenBuckets params: {e}")))?;

        let address = self.signer_address();
        let mut req = TransferBetweenBucketsRequest::new(
            &address,
            p.source_bucket_id,
            p.target_bucket_id,
            p.amount,
        );
        if let Some(r) = p.reason {
            req = req.reason(r);
        }
        self.sign_and_return(req.to_any()).await
    }

    /// Transfers margin from a bucket to the bank.
    #[wasm_bindgen(js_name = "transferToBank")]
    pub async fn transfer_to_bank(&self, params: JsValue) -> Result<JsValue, JsValue> {
        let p: TransferToBankParams = serde_wasm_bindgen::from_value(params)
            .map_err(|e| JsValue::from_str(&format!("invalid transferToBank params: {e}")))?;

        let address = self.signer_address();
        let req = TransferToBankRequest::new(
            &address,
            &address,
            p.bucket_id,
            p.asset_index,
            p.amount,
        );
        self.sign_and_return(req.to_any()).await
    }
}

// ==================== INTERNAL HELPERS ====================

impl MorpheumSdkWasm {
    fn signer_address(&self) -> String {
        hex::encode(self.signer.account_id().0)
    }

    async fn sign_and_return(
        &self,
        msg_any: morpheum_proto::google::protobuf::Any,
    ) -> Result<JsValue, JsValue> {
        let chain_id = self._config.default_chain_id.as_str().to_string();

        let body = TxBody {
            messages: vec![msg_any],
            memo: String::new(),
            timeout_timestamp: None,
            priority_tip: String::new(),
        };

        let signer_info = SignerInfo {
            public_key: Some(self.signer.public_key_proto()),
            mode_info: Some(ModeInfo {
                sum: Some(tx::mode_info::Sum::Single(tx::mode_info::Single {
                    mode: self.signer.sign_mode() as i32,
                })),
            }),
            chain_type: 0,
            ..Default::default()
        };

        let auth_info = AuthInfo {
            signer_infos: vec![signer_info],
            gas_limit: 0,
        };

        let body_bytes = body.encode_to_vec();
        let auth_info_bytes = auth_info.encode_to_vec();

        let sign_doc = SignDoc {
            body_bytes: body_bytes.clone(),
            auth_info_bytes: auth_info_bytes.clone(),
            chain_id,
            account_number: 0,
            genesis_hash: Vec::new(),
        };

        let signature = self.signer.sign(&sign_doc)
            .await
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let tx = Tx {
            body: Some(body),
            auth_info: Some(auth_info),
            signatures: vec![signature.to_bytes()],
            nonce: Some(Nonce {
                monotonic: 0,
                ts_ms: 0,
                sub: 0,
            }),
        };

        let tx_raw = TxRaw {
            body_bytes,
            auth_info_bytes,
            signatures: vec![signature.to_bytes()],
        };

        let raw_bytes = tx_raw.encode_to_vec();
        let hash = sha2::Sha256::digest(&raw_bytes);

        let obj = js_sys::Object::new();
        js_sys::Reflect::set(
            &obj,
            &"rawBytes".into(),
            &js_sys::Uint8Array::from(raw_bytes.as_slice()).into(),
        )
        .map_err(|_| JsValue::from_str("failed to set rawBytes"))?;
        js_sys::Reflect::set(
            &obj,
            &"txBytes".into(),
            &js_sys::Uint8Array::from(tx.encode_to_vec().as_slice()).into(),
        )
        .map_err(|_| JsValue::from_str("failed to set txBytes"))?;
        js_sys::Reflect::set(
            &obj,
            &"txhash".into(),
            &JsValue::from_str(&hex::encode(hash)),
        )
        .map_err(|_| JsValue::from_str("failed to set txhash"))?;

        Ok(obj.into())
    }
}
