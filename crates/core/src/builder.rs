//! Generic transaction builder for the Morpheum SDK.
//!
//! This module provides a clean, ergonomic `TxBuilder` that wraps the official
//! `morpheum_signing_core::TxBuilder` while integrating seamlessly with SDK types
//! (`ChainId`, `SignedTx`, etc.). All heavy lifting (signing, claim embedding,
//! dynamic SignerInfo, etc.) is delegated to the signing library — keeping this
//! crate clean, DRY, and truly `no_std` compatible.

use crate::{
    signing::{builder::TxBuilder as SigningTxBuilder, claim::TradingKeyClaim, signer::Signer},
    ChainId, SdkError, SignedTx,
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

    /// Sets a pre-built nonce directly, bypassing any nonce provider.
    ///
    /// Takes precedence over any configured `NonceProvider`. Use when the
    /// caller has already queried the nonce state (e.g. via gRPC).
    pub fn with_nonce(mut self, nonce: crate::signing::proto::tx::v1::Nonce) -> Self {
        self.inner = self.inner.with_nonce(nonce);
        self
    }

    /// Injects a nonce provider strategy (Sentry, AgentPortal, etc.).
    ///
    /// Without a provider, the signing library falls back to a zero nonce
    /// (`ts_ms=0`), which will be rejected by the chain's time-window check.
    pub fn with_nonce_provider(
        mut self,
        provider: impl crate::signing::nonce::NonceProvider + 'static,
    ) -> Self {
        self.inner = self.inner.with_nonce_provider(provider);
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

    /// Declares the transaction's semantics tier for the Phase 23A
    /// tier-aware intra-block tie-break. Leaving this unset defaults
    /// to `TxClass::Standard` (wire `0`), matching pre-23A behavior.
    ///
    /// Submitter-asserted on the wire; the consensus crate orders by
    /// tier but does NOT verify semantics — the runtime executor
    /// rejects mis-declared transactions at execution (a `PostOnly`
    /// that crosses, a `Cancel` against a non-existent order, etc.).
    /// See `morpheum_signing_core::tx_class` for the encoding
    /// contract and the SRP boundary.
    pub fn with_tx_class(mut self, class: crate::signing::tx_class::TxClass) -> Self {
        self.inner = self.inner.with_tx_class(class);
        self
    }

    /// Sets an optional priority tip in oneirs (1 MORM = 10^18
    /// oneirs) for faster inclusion during congestion. A value of
    /// `0` (default) means no tip — the transaction relies solely
    /// on mana-score sponsorship. Tips below 1 MORM are treated as
    /// dust and ignored by validators.
    ///
    /// Thin zero-cost wrapper over
    /// `morpheum_signing_core::TxBuilder::priority_tip` so the SDK
    /// builder exposes the wire-side `TxBody.priority_tip` field
    /// without re-implementing signing logic. Required for the
    /// Phase 22T MEV-extraction observability gates
    /// (`non_zero_tip_tx_count >= 1` / `>= 2`) which the bench
    /// drives via the Phase 22X Stage 6 tipped-tx interleave.
    pub fn priority_tip(mut self, tip_oneirs: u128) -> Self {
        self.inner = self.inner.priority_tip(tip_oneirs);
        self
    }

    /// Declares the transaction's `urgent` routing hint
    /// (Phase 22X.5.D Stage 2.E.1 — L17
    /// `WorkloadUrgentFlagAssignmentPolicyMode` implementation).
    ///
    /// Stamped onto `TxBody.urgent` (proto field 6); signed via
    /// `SignDoc.body_bytes` so a relayer or gossip peer cannot
    /// forge it. Consumed chain-side by
    /// `priority_flood::classify_validated` to route between the
    /// MAV path (`false`, default — the `MarkerRoutedToMavPathOnlyByDesign`
    /// Hypothesis-B confirmed path closed in §2.15.R) and the
    /// direct flood path (`true` — the §5.2I 4-axis matrix
    /// `TipsConvergeToFloodPath` slot enabled by §2.15.S).
    ///
    /// Thin zero-cost wrapper over
    /// `morpheum_signing_core::TxBuilder::urgent` so the SDK
    /// builder exposes the wire-side `TxBody.urgent` field
    /// without re-implementing signing logic. Required for the
    /// Phase 22X.5.D Stage 2.E.1 bench workload's
    /// [`WorkloadUrgentFlagAssignmentPolicyMode`]-driven `urgent`
    /// flag plumbing.
    pub fn urgent(mut self, urgent: bool) -> Self {
        self.inner = self.inner.urgent(urgent);
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
