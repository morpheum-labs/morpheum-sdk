//! Sign → submit → finality: the single write path for native clients.
//!
//! Every Morpheum write is a signed transaction submitted through
//! `IngressService/SubmitTx`; module Msgs are never sent over the query
//! transport. [`TxSubmitter`] owns the parts every writer otherwise
//! re-implements: nonce resolution against chain state, genesis-bound
//! signing, submission, bounded stale-nonce recovery, and waiting for a
//! terminal execution status.
//!
//! Admission is not finality: [`TxSubmitter::submit`] returns once the node
//! admits the transaction; [`TxSubmitter::wait_final`] reports whether it
//! executed.

use core::time::Duration;

use async_trait::async_trait;
use prost::Message;
use tokio::sync::Mutex;

use morpheum_proto::auth::v1::{QueryNonceStateRequest, QueryNonceStateResponse};
use morpheum_proto::tx::v1::{QueryTxStatusRequest, QueryTxStatusResponse, SubmitTxResponse, Tx};
use morpheum_sdk_core::builder::TxBuilder;
use morpheum_sdk_core::prelude::Any;
use morpheum_sdk_core::signing::proto::tx::v1::Nonce;
use morpheum_sdk_core::{BroadcastResult, SdkError};
use morpheum_signing_native::signer::Signer;

use crate::NativeSigner;

/// The node surface [`TxSubmitter`] needs, with `Send` futures so a
/// submitter can be driven from any task.
///
/// Unlike `Transport::broadcast_tx`, which folds a rejection into an error,
/// [`Self::submit_tx`] returns the node's raw admission verdict: the
/// submitter must tell "rejected" (possibly a stale nonce it can repair) from
/// "the transport failed".
#[async_trait]
pub trait IngressTransport: Send + Sync {
    /// Submits a signed transaction; `accepted = false` is a response, not an
    /// error.
    async fn submit_tx(&self, tx: Tx) -> Result<SubmitTxResponse, SdkError>;

    /// Unary query at a gRPC method path with pre-encoded request bytes.
    async fn query(&self, path: &str, data: Vec<u8>) -> Result<Vec<u8>, SdkError>;
}

/// Re-signs after a rejection at most this many times, and only when chain
/// state proves the nonce was stale (another writer on the same key).
const STALE_NONCE_RETRIES: u32 = 3;

/// Interval between `QueryTxStatus` polls in [`TxSubmitter::wait_final`].
const STATUS_POLL_INTERVAL: Duration = Duration::from_millis(250);

/// Terminal execution status of a submitted transaction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TxOutcome {
    /// Executed successfully.
    Confirmed {
        /// Height of the block that executed the transaction.
        height: u64,
        /// Shard that executed the transaction, when the node reports it.
        shard_id: Option<u32>,
    },
    /// Reached a terminal status without succeeding (`"failed"` or
    /// `"skipped"`); the node reports no further reason.
    Failed {
        /// The node's status string.
        status: String,
    },
}

/// Signs and submits transactions for one key, and waits for their
/// execution.
///
/// Construction requires the chain id **and** the target chain's 32-byte
/// genesis hash, so a signature that could be replayed onto another chain
/// sharing the chain id cannot be produced through this type.
///
/// Nonces are `max(highest signed here, chain last_monotonic) + 1`, resolved
/// under a lock so concurrent submits through one submitter never collide.
pub struct TxSubmitter<T: IngressTransport> {
    transport: T,
    signer: NativeSigner,
    chain_id: String,
    genesis_hash: [u8; 32],
    high_water: Mutex<u64>,
}

impl<T: IngressTransport> TxSubmitter<T> {
    /// Creates a submitter for `signer` on the chain identified by
    /// `chain_id` and `genesis_hash`.
    pub fn new(
        transport: T,
        signer: NativeSigner,
        chain_id: impl Into<String>,
        genesis_hash: [u8; 32],
    ) -> Self {
        Self {
            transport,
            signer,
            chain_id: chain_id.into(),
            genesis_hash,
            high_water: Mutex::new(0),
        }
    }

    /// The signer's account id, hex-encoded (the form nonce state is keyed by).
    pub fn account_hex(&self) -> String {
        self.signer
            .account_id()
            .0
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }

    /// Signs `msg` into a transaction and submits it.
    ///
    /// Returns once the node **admits** the transaction; use
    /// [`Self::wait_final`] for its execution outcome.
    ///
    /// # Errors
    ///
    /// A transport failure, a signing failure, or the node's rejection
    /// (after [`STALE_NONCE_RETRIES`] re-signs when chain state proves the
    /// nonce was stale).
    pub async fn submit(&self, msg: Any) -> Result<BroadcastResult, SdkError> {
        let mut high_water = self.high_water.lock().await;
        let mut chain_last = self.last_monotonic().await?;
        for _ in 0..=STALE_NONCE_RETRIES {
            let monotonic = (*high_water).max(chain_last) + 1;
            let tx = self.sign(msg.clone(), monotonic).await?;
            let resp = self.transport.submit_tx(tx).await?;
            if resp.accepted {
                *high_water = monotonic;
                return Ok(BroadcastResult {
                    txhash: resp.txhash,
                    shard_id: resp.shard_id,
                    raw_response: None,
                });
            }
            // Retry only on proof of staleness: the chain has moved to or
            // past the nonce we used. Anything else is a real rejection.
            chain_last = self.last_monotonic().await?;
            if chain_last < monotonic {
                return Err(SdkError::transport(format!(
                    "transaction rejected: {}",
                    resp.error_message
                )));
            }
        }
        Err(SdkError::transport(format!(
            "nonce stayed stale across {STALE_NONCE_RETRIES} re-signs (last_monotonic={chain_last}); \
             another writer is racing this key"
        )))
    }

    /// Polls the admitting node until `txhash` reaches a terminal status.
    ///
    /// Query the same node the transaction was submitted to: `"pending"`
    /// state is node-local, and another node answers `"not_found"` until it
    /// has executed the transaction.
    ///
    /// # Errors
    ///
    /// A transport failure, or `timeout` elapsing before a terminal status.
    pub async fn wait_final(&self, txhash: &str, timeout: Duration) -> Result<TxOutcome, SdkError> {
        let deadline = tokio::time::Instant::now() + timeout;
        loop {
            let req = QueryTxStatusRequest {
                txhash: txhash.to_string(),
            };
            let bytes = self
                .transport
                .query("/tx.v1.Query/QueryTxStatus", req.encode_to_vec())
                .await?;
            let status = QueryTxStatusResponse::decode(bytes.as_slice())?;
            match status.status.as_str() {
                "confirmed" => {
                    return Ok(TxOutcome::Confirmed {
                        height: status.height,
                        shard_id: status.shard_id,
                    });
                }
                "failed" | "skipped" => {
                    return Ok(TxOutcome::Failed {
                        status: status.status,
                    });
                }
                _ => {}
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(SdkError::transport(format!(
                    "tx {txhash} not terminal after {timeout:?} (last status '{}')",
                    status.status
                )));
            }
            tokio::time::sleep(STATUS_POLL_INTERVAL).await;
        }
    }

    async fn last_monotonic(&self) -> Result<u64, SdkError> {
        let req = QueryNonceStateRequest {
            address: self.account_hex(),
        };
        let bytes = self
            .transport
            .query("/auth.v1.Query/QueryNonceState", req.encode_to_vec())
            .await?;
        let resp = QueryNonceStateResponse::decode(bytes.as_slice())?;
        Ok(resp.state.map_or(0, |s| s.last_monotonic))
    }

    async fn sign(&self, msg: Any, monotonic: u64) -> Result<Tx, SdkError> {
        let signed = TxBuilder::new(self.signer.clone())
            .chain_id(self.chain_id.clone())
            .with_genesis_hash(self.genesis_hash.to_vec())
            .with_nonce(Nonce {
                monotonic,
                ts_ms: unix_ms_low32(),
                sub: 0,
            })
            .add_message(msg)
            .sign()
            .await?;
        // The signing crate's `Tx` is wire-identical to the proto crate's.
        Ok(Tx::decode(signed.tx().encode_to_vec().as_slice())?)
    }
}

/// Low 32 bits of the Unix time in milliseconds — the nonce's `ts_ms`
/// representation, which the chain compares by circular distance.
#[allow(clippy::cast_possible_truncation)]
fn unix_ms_low32() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_millis() as u32)
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Mutex as StdMutex;

    use super::*;
    use morpheum_proto::auth::v1::NonceState;

    /// Scripted node: chain nonce state, admission verdicts, tx statuses.
    #[derive(Default)]
    struct FakeNode {
        last_monotonic: StdMutex<u64>,
        /// Per submit: `Ok(txhash)` admits, `Err(reason)` rejects. When
        /// `bump_on_reject` is set, a rejection also advances chain state to
        /// that value (another writer consumed the nonce).
        verdicts: StdMutex<VecDeque<Result<String, String>>>,
        bump_on_reject: StdMutex<VecDeque<u64>>,
        statuses: StdMutex<VecDeque<(&'static str, u64)>>,
        submitted: StdMutex<Vec<u64>>,
    }

    #[async_trait]
    impl IngressTransport for FakeNode {
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/auth.v1.Query/QueryNonceState" => Ok(QueryNonceStateResponse {
                    state: Some(NonceState {
                        last_monotonic: *self.last_monotonic.lock().unwrap(),
                        ..Default::default()
                    }),
                }
                .encode_to_vec()),
                "/tx.v1.Query/QueryTxStatus" => {
                    let (status, height) = self
                        .statuses
                        .lock()
                        .unwrap()
                        .pop_front()
                        .unwrap_or(("pending", 0));
                    Ok(QueryTxStatusResponse {
                        status: status.into(),
                        success: status == "confirmed",
                        height,
                        shard_id: Some(2),
                        ..Default::default()
                    }
                    .encode_to_vec())
                }
                other => panic!("unexpected query {other}"),
            }
        }

        async fn submit_tx(&self, tx: Tx) -> Result<SubmitTxResponse, SdkError> {
            let monotonic = tx.nonce.expect("signed tx carries a nonce").monotonic;
            self.submitted.lock().unwrap().push(monotonic);
            match self
                .verdicts
                .lock()
                .unwrap()
                .pop_front()
                .expect("scripted verdict")
            {
                Ok(txhash) => Ok(SubmitTxResponse {
                    accepted: true,
                    txhash,
                    ..Default::default()
                }),
                Err(reason) => {
                    if let Some(to) = self.bump_on_reject.lock().unwrap().pop_front() {
                        *self.last_monotonic.lock().unwrap() = to;
                    }
                    Ok(SubmitTxResponse {
                        accepted: false,
                        error_message: reason,
                        ..Default::default()
                    })
                }
            }
        }
    }

    fn submitter(node: FakeNode) -> TxSubmitter<FakeNode> {
        TxSubmitter::new(
            node,
            NativeSigner::from_seed(&[7u8; 32]),
            "morm-test-1",
            [9u8; 32],
        )
    }

    fn msg() -> Any {
        Any {
            type_url: "/test.v1.MsgNoop".into(),
            value: vec![],
        }
    }

    #[tokio::test]
    async fn nonce_follows_chain_state_then_local_high_water() {
        let node = FakeNode::default();
        *node.last_monotonic.lock().unwrap() = 7;
        node.verdicts
            .lock()
            .unwrap()
            .extend([Ok("a".into()), Ok("b".into())]);
        let s = submitter(node);

        assert_eq!(s.submit(msg()).await.unwrap().txhash, "a");
        // Chain state has not caught up with the admitted tx; the second
        // submit must not reuse 8.
        assert_eq!(s.submit(msg()).await.unwrap().txhash, "b");
        assert_eq!(*s.transport.submitted.lock().unwrap(), vec![8, 9]);
    }

    #[tokio::test]
    async fn stale_nonce_is_re_signed_from_fresh_chain_state() {
        let node = FakeNode::default();
        node.verdicts
            .lock()
            .unwrap()
            .extend([Err("stale nonce".into()), Ok("ok".into())]);
        node.bump_on_reject.lock().unwrap().push_back(20);
        let s = submitter(node);

        assert_eq!(s.submit(msg()).await.unwrap().txhash, "ok");
        assert_eq!(*s.transport.submitted.lock().unwrap(), vec![1, 21]);
    }

    #[tokio::test]
    async fn a_real_rejection_is_not_retried() {
        let node = FakeNode::default();
        node.verdicts
            .lock()
            .unwrap()
            .push_back(Err("insufficient funds".into()));
        let s = submitter(node);

        let err = s.submit(msg()).await.unwrap_err().to_string();
        assert!(err.contains("insufficient funds"), "{err}");
        assert_eq!(s.transport.submitted.lock().unwrap().len(), 1);
    }

    #[tokio::test]
    async fn wait_final_polls_until_terminal() {
        let node = FakeNode::default();
        node.statuses
            .lock()
            .unwrap()
            .extend([("pending", 0), ("not_found", 0), ("confirmed", 42)]);
        let s = submitter(node);
        assert_eq!(
            s.wait_final("h", Duration::from_secs(5)).await.unwrap(),
            TxOutcome::Confirmed {
                height: 42,
                shard_id: Some(2)
            }
        );

        let node = FakeNode::default();
        node.statuses.lock().unwrap().push_back(("failed", 0));
        let s = submitter(node);
        assert_eq!(
            s.wait_final("h", Duration::from_secs(5)).await.unwrap(),
            TxOutcome::Failed {
                status: "failed".into()
            }
        );
    }

    #[tokio::test]
    async fn wait_final_times_out_on_a_tx_that_never_lands() {
        let s = submitter(FakeNode::default());
        assert!(s.wait_final("h", Duration::from_millis(10)).await.is_err());
    }
}
