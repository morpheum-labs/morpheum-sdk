//! WASM-specific utility functions for the Morpheum SDK.
//!
//! This module contains small, focused, reusable helpers for common WASM
//! interop patterns such as byte array conversions, hex encoding, and
//! error bridging between Rust and JavaScript.
//!
//! All functions are zero-cost where possible and follow strict safety
//! and ergonomics standards.

use alloc::vec::Vec;
use wasm_bindgen::prelude::*;

/// Converts a `Vec<u8>` into a JavaScript `Uint8Array`.
///
/// This is the most common conversion when returning binary data to JS.
#[inline]
pub fn to_uint8_array(bytes: Vec<u8>) -> js_sys::Uint8Array {
    js_sys::Uint8Array::from(bytes.as_slice())
}

/// Converts a JavaScript `Uint8Array` into a Rust `Vec<u8>`.
///
/// Returns an empty vector if the input is null or undefined (safe default).
#[inline]
pub fn from_uint8_array(array: JsValue) -> Vec<u8> {
    if array.is_undefined() || array.is_null() {
        return Vec::new();
    }
    js_sys::Uint8Array::from(array).to_vec()
}

/// Converts a `SdkError` into a JavaScript `JsValue` with a clear error message.
///
/// Used throughout the bindings layer for consistent error reporting to JS.
pub fn sdk_error_to_js(error: morpheum_sdk_core::SdkError) -> JsValue {
    JsValue::from_str(&format!("Morpheum SDK Error: {}", error))
}

/// Safely decodes a hex string into bytes.
///
/// Returns a clear error (as `JsValue`) if the input is invalid.
/// Strips optional "0x" prefix automatically.
pub fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, JsValue> {
    let cleaned = hex.strip_prefix("0x").unwrap_or(hex).trim();
    hex::decode(cleaned).map_err(|e| {
        JsValue::from_str(&format!("Invalid hex string: {}", e))
    })
}

/// Encodes bytes as a lowercase hex string (without 0x prefix).
#[inline]
pub fn bytes_to_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_uint8_array_roundtrip() {
        let original = vec![1u8, 2, 3, 4, 255];
        let js_array = to_uint8_array(original.clone());
        let recovered = from_uint8_array(js_array.into());

        assert_eq!(original, recovered);
    }

    #[test]
    fn hex_conversion_works() {
        let bytes = vec![0x01, 0x23, 0xab, 0xff];
        let hex = bytes_to_hex(&bytes);
        assert_eq!(hex, "0123abff");

        let recovered = hex_to_bytes(&hex).unwrap();
        assert_eq!(bytes, recovered);

        // Test with 0x prefix
        let recovered2 = hex_to_bytes("0x0123abff").unwrap();
        assert_eq!(bytes, recovered2);
    }

    #[test]
    fn hex_invalid_input_returns_error() {
        let result = hex_to_bytes("0xgg");
        assert!(result.is_err());
    }
}