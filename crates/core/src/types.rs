//! Common types used throughout the Morpheum SDK.
//!
//! This module provides ergonomic, type-safe wrappers around primitives
//! from `morpheum-signing-core` while remaining strictly `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::signing::types::{
    AccountId as SigningAccountId, SignedTx as SigningSignedTx,
};
use crate::SdkError;

/// SDK-level `AccountId` — thin newtype over the signing library's `AccountId`.
///
/// This provides a clean, extensible surface for the SDK while delegating
/// all cryptographic logic to the official signing crate.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct AccountId(pub SigningAccountId);

impl AccountId {
    /// Creates a new `AccountId` from raw 32 bytes.
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(SigningAccountId(bytes))
    }

    /// Returns the underlying 32-byte array.
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0.0
    }
}

impl From<SigningAccountId> for AccountId {
    fn from(id: SigningAccountId) -> Self {
        Self(id)
    }
}

impl From<AccountId> for SigningAccountId {
    fn from(id: AccountId) -> Self {
        id.0
    }
}

impl fmt::Debug for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Hex-encode inline without pulling in the `hex` crate.
        write!(f, "AccountId(")?;
        for byte in self.as_bytes() {
            write!(f, "{:02x}", byte)?;
        }
        write!(f, ")")
    }
}

impl fmt::Display for AccountId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for byte in self.as_bytes() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

/// Canonical chain identifier (e.g. `"morpheum-1"`, `"morpheum-test-1"`).
///
/// Newtype pattern for type safety and future validation/extension.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ChainId(String);

impl ChainId {
    /// Creates a new `ChainId` after validating it is non-empty.
    pub fn new<S: Into<String>>(id: S) -> Result<Self, SdkError> {
        let s = id.into();
        if s.trim().is_empty() {
            return Err(SdkError::invalid_input("chain_id cannot be empty"));
        }
        Ok(Self(s))
    }

    /// Returns the chain ID as a `&str`.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for ChainId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for ChainId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// Ergonomic From conversions so callers can pass &str / String directly
// to builders and constructors (e.g. `SdkConfig::new("endpoint", "morpheum-1")`).

impl From<&str> for ChainId {
    fn from(s: &str) -> Self {
        Self(s.into())
    }
}

impl From<String> for ChainId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

#[cfg(feature = "serde")]
impl Serialize for ChainId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for ChainId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Self::new(s).map_err(serde::de::Error::custom)
    }
}

/// The canonical signed transaction returned by the SDK.
///
/// Thin ergonomic wrapper around the signing library's `SignedTx` that adds
/// convenient SDK-level helper methods while delegating all core logic.
#[derive(Clone, Debug)]
pub struct SignedTx(pub SigningSignedTx);

impl SignedTx {
    /// Returns the raw bytes ready for broadcast.
    pub fn raw_bytes(&self) -> &[u8] {
        self.0.raw_bytes()
    }

    /// Returns the SHA-256 hex transaction hash (standard in Cosmos ecosystems).
    #[cfg(feature = "std")]
    pub fn txhash_hex(&self) -> String {
        self.0.txhash_hex()
    }

    /// Returns the underlying decoded `Tx` protobuf (for inspection/debugging).
    ///
    /// Note: This returns the signing library's `Tx` type (from `morpheum-signing-core`).
    /// It is structurally identical to `morpheum_proto::tx::v1::Tx`.
    pub fn tx(&self) -> &crate::signing::proto::tx::v1::Tx {
        self.0.tx()
    }

    /// Returns the raw `TxRaw` protobuf bytes (if present).
    pub fn tx_raw_bytes(&self) -> Option<Vec<u8>> {
        self.0.tx_raw().map(prost::Message::encode_to_vec)
    }
}

impl From<SigningSignedTx> for SignedTx {
    fn from(tx: SigningSignedTx) -> Self {
        Self(tx)
    }
}

// Re-exports of the most commonly used types from the signing library.
// This keeps the SDK API clean and DRY — users can do `use morpheum_sdk_core::types::*;`
pub use crate::signing::types::{PublicKey, Signature, WalletType};
pub use crate::signing::claim::{TradingKeyClaim, VcClaimBuilder};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_id_validation() {
        assert!(ChainId::new("").is_err());
        assert!(ChainId::new("   ").is_err());
        assert!(ChainId::new("morpheum-1").is_ok());
    }

    #[test]
    fn chain_id_from_str() {
        let id: ChainId = "morpheum-1".into();
        assert_eq!(id.as_str(), "morpheum-1");
    }
}
