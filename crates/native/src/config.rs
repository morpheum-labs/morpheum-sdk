//! Native-specific configuration extensions for the Morpheum SDK.
//!
//! This module extends the core `SdkConfig` with native-only settings
//! such as transport preference, connection pooling, and default behaviors.
//! It follows the extension pattern to keep the core crate pure `no_std`.

use morpheum_sdk_core::{ChainId, SdkConfig, SdkError};

/// Native-specific configuration for the Morpheum SDK.
///
/// This struct wraps the core `SdkConfig` and adds native-only options
/// such as transport preference and connection settings.
#[derive(Clone, Debug)]
pub struct NativeConfig {
    /// The core SDK configuration (shared with WASM).
    pub core: SdkConfig,

    /// Preferred transport backend.
    pub transport_preference: TransportPreference,

    /// Maximum number of concurrent connections (for HTTP/gRPC pools).
    pub max_connections: usize,

    /// Whether to enable connection keep-alive and pooling.
    pub enable_connection_pool: bool,
}

/// Transport preference for native environments.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportPreference {
    /// Prefer gRPC (tonic) — recommended for Cosmos chains.
    Grpc,
    /// Prefer HTTP (reqwest).
    Http,
    /// Auto-select based on available features (gRPC > HTTP).
    Auto,
}

impl NativeConfig {
    /// Creates a new `NativeConfig` with sensible defaults.
    pub fn new(rpc_endpoint: impl Into<String>, chain_id: impl Into<ChainId>) -> Self {
        Self::builder()
            .rpc_endpoint(rpc_endpoint)
            .chain_id(chain_id)
            .build()
            .expect("NativeConfig::new() with valid inputs should never fail")
    }

    /// Returns a fluent builder for `NativeConfig`.
    pub fn builder() -> NativeConfigBuilder {
        NativeConfigBuilder::default()
    }

    /// Returns a reference to the underlying core config.
    pub fn core(&self) -> &SdkConfig {
        &self.core
    }
}

/// Fluent builder for `NativeConfig`.
#[derive(Default)]
pub struct NativeConfigBuilder {
    core_builder: morpheum_sdk_core::SdkConfigBuilder,
    transport_preference: Option<TransportPreference>,
    max_connections: Option<usize>,
    enable_connection_pool: Option<bool>,
}

impl NativeConfigBuilder {
    /// Sets the RPC endpoint (required).
    pub fn rpc_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.core_builder = self.core_builder.rpc_endpoint(endpoint);
        self
    }

    /// Sets the default chain ID (required).
    pub fn chain_id(mut self, chain_id: impl Into<ChainId>) -> Self {
        self.core_builder = self.core_builder.chain_id(chain_id);
        self
    }

    /// Sets the transport preference.
    pub fn transport_preference(mut self, preference: TransportPreference) -> Self {
        self.transport_preference = Some(preference);
        self
    }

    /// Sets the maximum number of concurrent connections.
    pub fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = Some(max);
        self
    }

    /// Enables or disables connection pooling.
    pub fn enable_connection_pool(mut self, enable: bool) -> Self {
        self.enable_connection_pool = Some(enable);
        self
    }

    /// Builds the final `NativeConfig` with validation.
    pub fn build(self) -> Result<NativeConfig, SdkError> {
        let core = self.core_builder.build()?;

        Ok(NativeConfig {
            core,
            transport_preference: self.transport_preference.unwrap_or(TransportPreference::Auto),
            max_connections: self.max_connections.unwrap_or(16),
            enable_connection_pool: self.enable_connection_pool.unwrap_or(true),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_config_builder_minimal() {
        let config = NativeConfig::builder()
            .rpc_endpoint("https://sentry.morpheum.xyz")
            .chain_id("morpheum-test-1")
            .build()
            .unwrap();

        assert_eq!(config.core.default_chain_id.as_str(), "morpheum-test-1");
        assert_eq!(config.transport_preference, TransportPreference::Auto);
        assert_eq!(config.max_connections, 16);
        assert!(config.enable_connection_pool);
    }

    #[test]
    fn native_config_builder_full() {
        let config = NativeConfig::builder()
            .rpc_endpoint("https://grpc.morpheum.xyz")
            .chain_id("morpheum-1")
            .transport_preference(TransportPreference::Grpc)
            .max_connections(32)
            .enable_connection_pool(false)
            .build()
            .unwrap();

        assert_eq!(config.transport_preference, TransportPreference::Grpc);
        assert_eq!(config.max_connections, 32);
        assert!(!config.enable_connection_pool);
    }

    #[test]
    fn validation_fails_on_missing_fields() {
        let result = NativeConfig::builder().build();
        assert!(result.is_err());
    }
}