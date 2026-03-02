//! Base trait for all Morpheum module clients.
//!
//! This trait is the foundation of the Client Pattern used throughout the SDK.
//! Every module-specific client (`MarketClient`, `VcClient`, `AuthClient`, etc.)
//! implements this trait. It provides uniform access to configuration and
//! transport while offering default implementations for common operations
//! to keep the design DRY and SOLID.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;

use crate::{BroadcastResult, SdkConfig, SdkError, Transport};

/// Base trait implemented by all module clients in the Morpheum SDK.
///
/// This trait follows the **Client Pattern** and ensures a consistent,
/// extensible API across all modules while delegating actual I/O to the
/// pluggable `Transport` trait.
#[async_trait(?Send)] // ?Send required for WASM compatibility (single-threaded)
pub trait MorpheumClient: Send + Sync + 'static {
    /// Returns a reference to the SDK configuration.
    fn config(&self) -> &SdkConfig;

    /// Returns a reference to the underlying transport.
    fn transport(&self) -> &dyn Transport;

    /// Broadcasts a signed transaction (raw TxRaw bytes) to the network.
    ///
    /// This is the most common operation and is provided as a default
    /// implementation to avoid code duplication across modules.
    async fn broadcast(&self, tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError> {
        self.transport().broadcast_tx(tx_bytes).await
    }

    /// Performs an ABCI query. Most modules do not need this directly,
    /// so a default implementation is provided that forwards to the transport.
    async fn query(&self, path: &str, data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
        self.transport().query(path, data).await
    }
}

// Blanket implementation for boxed trait objects.
// This allows storing `Box<dyn MorpheumClient>` when needed.
#[async_trait(?Send)]
impl<T: MorpheumClient + ?Sized> MorpheumClient for Box<T> {
    fn config(&self) -> &SdkConfig {
        (**self).config()
    }

    fn transport(&self) -> &dyn Transport {
        (**self).transport()
    }

    async fn broadcast(&self, tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError> {
        (**self).broadcast(tx_bytes).await
    }

    async fn query(&self, path: &str, data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
        (**self).query(path, data).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transport::DummyTransport; // from transport.rs test helper

    // Minimal test client for compile-time verification
    struct TestClient {
        config: SdkConfig,
        transport: Box<dyn Transport>,
    }

    impl TestClient {
        fn new() -> Self {
            Self {
                config: SdkConfig::new("https://test.morpheum.xyz", "morpheum-test-1"),
                transport: Box::new(DummyTransport),
            }
        }
    }

    #[async_trait(?Send)]
    impl MorpheumClient for TestClient {
        fn config(&self) -> &SdkConfig {
            &self.config
        }

        fn transport(&self) -> &dyn Transport {
            &*self.transport
        }
    }

    #[tokio::test]
    async fn morpheum_client_trait_works() {
        let client = TestClient::new();

        assert_eq!(client.config().default_chain_id.as_str(), "morpheum-test-1");

        let result = client.broadcast(vec![1, 2, 3]).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn boxed_client_works() {
        let client: Box<dyn MorpheumClient> = Box::new(TestClient::new());
        let result = client.broadcast(vec![]).await;
        assert!(result.is_ok());
    }
}