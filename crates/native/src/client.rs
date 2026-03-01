//! NativeClient — the central native implementation that integrates all
//! feature-gated module clients (MarketClient, VcClient, AuthClient).
//!
//! This file provides the concrete native glue layer between the high-level
//! `MorpheumSdk` facade and the individual module clients. It is responsible
//! for constructing module clients with the correct config and transport.

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

/// The main native client that integrates all enabled modules.
///
/// This struct holds the shared configuration and transport, and provides
/// convenient accessors to feature-gated module clients.
#[derive(Clone)]
pub struct NativeClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl NativeClient {
    /// Creates a new `NativeClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Returns the underlying SDK configuration.
    pub fn config(&self) -> &SdkConfig {
        &self.config
    }

    // ==================== MODULE CLIENT ACCESSORS ====================

    /// Returns a MarketClient (only available when the "market" feature is enabled).
    #[cfg(feature = "market")]
    pub fn market(&self) -> morpheum_sdk_market::MarketClient {
        morpheum_sdk_market::MarketClient::new(self.config.clone(), self.transport.clone())
    }

    /// Returns a VcClient (only available when the "vc" feature is enabled).
    #[cfg(feature = "vc")]
    pub fn vc(&self) -> morpheum_sdk_vc::VcClient {
        morpheum_sdk_vc::VcClient::new(self.config.clone(), self.transport.clone())
    }

    /// Returns an AuthClient (only available when the "auth" feature is enabled).
    #[cfg(feature = "auth")]
    pub fn auth(&self) -> morpheum_sdk_auth::AuthClient {
        morpheum_sdk_auth::AuthClient::new(self.config.clone(), self.transport.clone())
    }
}

// ==================== INTEGRATION WITH MorpheumSdk ====================

// This allows MorpheumSdk to delegate to NativeClient when in native context.
// (The actual delegation happens in lib.rs via the convenience constructors.)

#[cfg(test)]
mod tests {
    use super::*;
    use morpheum_sdk_core::SdkConfig;

    // Dummy transport for testing
    #[derive(Clone)]
    struct DummyTransport;

    #[async_trait::async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _tx_bytes: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            Err(SdkError::transport("dummy transport"))
        }
    }

    #[test]
    fn native_client_creates_cleanly() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = NativeClient::new(config, Box::new(DummyTransport));

        assert_eq!(client.config().default_chain_id.as_str(), "morpheum-test-1");
    }

    #[cfg(feature = "market")]
    #[test]
    fn market_client_accessible() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = NativeClient::new(config, Box::new(DummyTransport));

        let _market_client = client.market();
        // Compilation test — ensures the accessor works when feature is enabled
    }
}