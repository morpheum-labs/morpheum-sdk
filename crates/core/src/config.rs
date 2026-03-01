//! Configuration for the Morpheum SDK.
//!
//! This module provides `SdkConfig` and its fluent builder `SdkConfigBuilder`.
//! It is designed to be lightweight, `no_std` compatible, and extensible.

use alloc::string::String;

use crate::{ChainId, SdkError};

/// Main configuration for the Morpheum SDK.
///
/// This struct is intentionally minimal and immutable after creation.
/// All heavy configuration (signer, transport strategy) is handled at the
/// `MorpheumSdk` level to keep the core clean and reusable.
#[derive(Clone, Debug)]
pub struct SdkConfig {
    /// Primary RPC endpoint (gRPC or HTTP).
    /// Example: `"https://sentry.morpheum.xyz:443"`
    pub rpc_endpoint: String,

    /// Default chain ID for all transactions.
    pub default_chain_id: ChainId,

    /// Optional request timeout in seconds (default: 60).
    pub timeout_secs: u64,

    /// Optional custom user-agent string.
    pub user_agent: Option<String>,
}

impl SdkConfig {
    /// Creates a minimal config with sensible defaults.
    pub fn new(rpc_endpoint: impl Into<String>, chain_id: impl Into<ChainId>) -> Self {
        Self::builder()
            .rpc_endpoint(rpc_endpoint)
            .chain_id(chain_id)
            .build()
            .expect("SdkConfig::new() with valid inputs should never fail")
    }

    /// Returns a new fluent builder for `SdkConfig`.
    pub fn builder() -> SdkConfigBuilder {
        SdkConfigBuilder::default()
    }
}

/// Fluent builder for `SdkConfig`.
///
/// Follows the classic Builder pattern for complex configuration objects.
#[derive(Default)]
pub struct SdkConfigBuilder {
    rpc_endpoint: Option<String>,
    chain_id: Option<ChainId>,
    timeout_secs: Option<u64>,
    user_agent: Option<String>,
}

impl SdkConfigBuilder {
    /// Sets the primary RPC endpoint (required).
    pub fn rpc_endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.rpc_endpoint = Some(endpoint.into());
        self
    }

    /// Sets the default chain ID (required).
    pub fn chain_id(mut self, chain_id: impl Into<ChainId>) -> Self {
        self.chain_id = Some(chain_id.into());
        self
    }

    /// Sets the request timeout in seconds (default: 60).
    pub fn timeout_secs(mut self, secs: u64) -> Self {
        self.timeout_secs = Some(secs);
        self
    }

    /// Sets a custom user-agent header.
    pub fn user_agent(mut self, ua: impl Into<String>) -> Self {
        self.user_agent = Some(ua.into());
        self
    }

    /// Builds the final `SdkConfig`, performing validation.
    pub fn build(self) -> Result<SdkConfig, SdkError> {
        let rpc_endpoint = self.rpc_endpoint.ok_or_else(|| {
            SdkError::config("rpc_endpoint is required")
        })?;

        let default_chain_id = self.chain_id.ok_or_else(|| {
            SdkError::config("default_chain_id is required")
        })?;

        // Basic validation
        if rpc_endpoint.trim().is_empty() {
            return Err(SdkError::invalid_input("rpc_endpoint cannot be empty"));
        }

        Ok(SdkConfig {
            rpc_endpoint,
            default_chain_id,
            timeout_secs: self.timeout_secs.unwrap_or(60),
            user_agent: self.user_agent,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_minimal_works() {
        let config = SdkConfig::builder()
            .rpc_endpoint("https://sentry.morpheum.xyz")
            .chain_id("morpheum-1")
            .build()
            .unwrap();

        assert_eq!(config.rpc_endpoint, "https://sentry.morpheum.xyz");
        assert_eq!(config.default_chain_id.as_str(), "morpheum-1");
        assert_eq!(config.timeout_secs, 60);
    }

    #[test]
    fn builder_with_options() {
        let config = SdkConfig::builder()
            .rpc_endpoint("https://grpc.morpheum.xyz")
            .chain_id(ChainId::new("morpheum-test-1").unwrap())
            .timeout_secs(30)
            .user_agent("morpheum-sdk/0.1.0")
            .build()
            .unwrap();

        assert_eq!(config.timeout_secs, 30);
        assert_eq!(config.user_agent.as_deref(), Some("morpheum-sdk/0.1.0"));
    }

    #[test]
    fn builder_validation() {
        assert!(SdkConfig::builder().build().is_err());
        assert!(SdkConfig::builder()
            .rpc_endpoint("")
            .chain_id("morpheum-1")
            .build()
            .is_err());
    }
}