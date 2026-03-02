//! Unified error type for the Morpheum SDK core.
//!
//! This is the single source of truth for all errors in the SDK.
//! It wraps `morpheum_signing_core::SigningError` and provides clean,
//! actionable variants for other common failure modes.

use core::fmt;

use thiserror::Error;

/// Unified error type for the Morpheum SDK.
///
/// This enum is deliberately `no_std` compatible by default.
/// When the `std` feature is enabled, it also implements `std::error::Error`.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SdkError {
    /// Errors originating from the official Morpheum signing library.
    /// This includes key handling, claim construction/verification,
    /// wallet adapter failures, signature generation, and nonce issues.
    #[error("signing error: {0}")]
    Signing(#[from] crate::signing::SigningError),

    /// Transport-layer errors (gRPC, HTTP, connection, etc.).
    /// Concrete transport implementations will convert their errors into this variant.
    #[error("transport error: {0}")]
    Transport(alloc::string::String),

    /// Protobuf encoding failure (e.g. when building SignDoc or TxRaw).
    #[error("protobuf encode error: {0}")]
    Encode(#[from] prost::EncodeError),

    /// Protobuf decoding failure (rare in SDK usage, but provided for completeness).
    #[error("protobuf decode error: {0}")]
    Decode(#[from] prost::DecodeError),

    /// Configuration or initialization error (invalid endpoint, missing signer, etc.).
    #[error("configuration error: {0}")]
    Config(alloc::string::String),

    /// Invalid input provided by the user (e.g. empty message list, invalid claim).
    #[error("invalid input: {0}")]
    InvalidInput(alloc::string::String),

    /// Generic catch-all for rare cases that don't fit other variants.
    /// Prefer specific variants whenever possible.
    #[error("sdk error: {0}")]
    Other(alloc::string::String),
}

impl SdkError {
    /// Creates a new transport error from any error that can be displayed.
    pub fn transport<E: fmt::Display>(err: E) -> Self {
        Self::Transport(alloc::format!("{}", err))
    }

    /// Creates a new configuration error.
    pub fn config(msg: impl Into<alloc::string::String>) -> Self {
        Self::Config(msg.into())
    }

    /// Creates a new invalid input error.
    pub fn invalid_input(msg: impl Into<alloc::string::String>) -> Self {
        Self::InvalidInput(msg.into())
    }

    /// Creates a generic "other" error.
    pub fn other(msg: impl Into<alloc::string::String>) -> Self {
        Self::Other(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn signing_error_converts() {
        let signing_err = crate::signing::SigningError::invalid_key("test key");
        let sdk_err: SdkError = signing_err.into();
        assert!(matches!(sdk_err, SdkError::Signing(_)));
    }

    #[test]
    fn transport_error_works() {
        let err = SdkError::transport("connection refused");
        assert!(err.to_string().contains("transport error"));
    }
}