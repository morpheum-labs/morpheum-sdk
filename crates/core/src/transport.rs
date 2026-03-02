//! Pluggable transport abstraction for the Morpheum SDK.
//!
//! This trait defines the interface for sending signed transactions and performing
//! queries. Concrete implementations (gRPC via tonic, HTTP via reqwest) live in
//! the `native` crate.
//!
//! The design is deliberately minimal, extensible, and `no_std`-friendly where possible.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;

use crate::SdkError;

/// Result of broadcasting a signed transaction.
#[derive(Debug, Clone)]
pub struct BroadcastResult {
    /// Transaction hash (SHA-256 hex).
    pub txhash: String,

    /// Optional raw response from the node (for advanced debugging).
    pub raw_response: Option<Vec<u8>>,
}

/// Core transport abstraction.
///
/// This trait is object-safe and can be used as `Box<dyn Transport>` or
/// via generics. It is the single point of network I/O in the SDK.
#[async_trait(?Send)] // ?Send for WASM compatibility (single-threaded)
pub trait Transport: Send + Sync + 'static {
    /// Broadcasts a signed transaction (TxRaw encoded bytes) to the network.
    ///
    /// Returns the transaction hash on success.
    async fn broadcast_tx(&self, tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError>;

    /// Performs a generic ABCI query.
    ///
    /// Used internally by `AuthClient` for nonce and account queries.
    /// Default implementation returns a clear error so that transports
    /// that only support broadcasting can still compile.
    async fn query(
        &self,
        path: &str,
        data: Vec<u8>,
    ) -> Result<Vec<u8>, SdkError> {
        let _ = (path, data);
        Err(SdkError::transport(
            "query is not supported by this transport implementation",
        ))
    }
}

// Convenience blanket implementation for Box<dyn Transport>
#[async_trait(?Send)]
impl<T: Transport + ?Sized> Transport for Box<T> {
    async fn broadcast_tx(&self, tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError> {
        (**self).broadcast_tx(tx_bytes).await
    }

    async fn query(&self, path: &str, data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
        (**self).query(path, data).await
    }
}

/// Minimal no-op transport for internal testing. Not part of the public API.
#[cfg(test)]
pub(crate) struct DummyTransport;

#[cfg(test)]
#[async_trait(?Send)]
impl Transport for DummyTransport {
    async fn broadcast_tx(&self, _tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError> {
        Ok(BroadcastResult {
            txhash: "0000000000000000000000000000000000000000000000000000000000000000".into(),
            raw_response: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[tokio::test]
    async fn transport_trait_object_works() {
        let transport: Box<dyn Transport> = Box::new(DummyTransport);
        let result = transport.broadcast_tx(vec![]).await.unwrap();
        assert!(!result.txhash.is_empty());
    }
}