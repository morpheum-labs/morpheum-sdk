//! Generic transaction builder for the Morpheum SDK.
//!
//! This module provides a clean, ergonomic `TxBuilder` that wraps the official
//! `morpheum_signing_core::TxBuilder` while integrating seamlessly with SDK types
//! (`ChainId`, `SignedTx`, etc.). All heavy lifting (signing, claim embedding,
//! dynamic SignerInfo, etc.) is delegated to the signing library — keeping this
//! crate clean, DRY, and truly `no_std` compatible.

use crate::{
    signing::{signer::Signer, claim::TradingKeyClaim, builder::TxBuilder as SigningTxBuilder},
    ChainId, SignedTx, SdkError,
};

// Re-use the same `Any` type the signing library uses (prost_types::Any),
// ensuring zero-cost pass-through to the inner TxBuilder.
use crate::signing::Any as ProtoAny;

/// Fluent transaction builder for the Morpheum SDK.
///
/// This is a thin, zero-cost wrapper around the signing library's `TxBuilder`.
/// It provides SDK-native ergonomics while delegating all cryptographic and
/// protobuf logic to `morpheum_signing_core`.
pub struct TxBuilder<S: Signer> {
    inner: SigningTxBuilder<S>,
}

impl<S: Signer> TxBuilder<S> {
    /// Creates a new transaction builder with the given signer.
    pub fn new(signer: S) -> Self {
        Self {
            inner: SigningTxBuilder::new(signer),
        }
    }

    /// Sets the chain ID for this transaction.
    pub fn chain_id(mut self, chain_id: impl Into<ChainId>) -> Self {
        let chain_id = chain_id.into();
        self.inner = self.inner.chain_id(chain_id.as_str());
        self
    }

    /// Sets an optional memo for the transaction.
    pub fn memo(mut self, memo: impl Into<alloc::string::String>) -> Self {
        self.inner = self.inner.memo(memo);
        self
    }

    /// Adds a raw protobuf `Any` message to the transaction body.
    ///
    /// This is the most generic way to add messages and keeps the core SDK
    /// completely decoupled from specific module types.
    pub fn add_message(mut self, msg: ProtoAny) -> Self {
        self.inner = self.inner.add_message(msg);
        self
    }

    /// Convenience method to add a typed protobuf message by packing it into `Any`.
    pub fn add_typed_message<M: prost::Message>(
        mut self,
        type_url: impl Into<alloc::string::String>,
        msg: &M,
    ) -> Self {
        self.inner = self.inner.add_typed_message(type_url, msg);
        self
    }

    /// Attaches a `TradingKeyClaim` for agent delegation.
    ///
    /// The claim will be embedded in `SignerInfo.signing_options` and covered
    /// by the transaction signature (delegated to the signing library).
    pub fn with_trading_key_claim(mut self, claim: TradingKeyClaim) -> Self {
        self.inner = self.inner.with_trading_key_claim(claim);
        self
    }

    /// Finalizes and signs the transaction.
    ///
    /// Returns the SDK's `SignedTx` wrapper on success.
    pub async fn sign(self) -> Result<SignedTx, SdkError> {
        let signed = self.inner.sign().await.map_err(SdkError::from)?;
        Ok(SignedTx::from(signed))
    }
}

// Re-export the signing library's TxBuilder for advanced users who need
// direct access to all its methods.
pub use crate::signing::builder::TxBuilder as RawTxBuilder;
