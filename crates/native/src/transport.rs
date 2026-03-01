//! Concrete transport implementations for the Morpheum Native SDK.
//!
//! This module provides two production-grade transports:
//! - `GrpcTransport` (tonic-based gRPC) — preferred for Cosmos chains
//! - `HttpTransport` (reqwest-based HTTP) — lightweight alternative
//!
//! Both implement the core `Transport` trait from `morpheum_sdk_core`.
//! Feature-gated for minimal builds. When both features are enabled, `GrpcTransport`
//! is the recommended default.

use alloc::vec::Vec;
use async_trait::async_trait;

use morpheum_sdk_core::{BroadcastResult, SdkError, Transport};

#[cfg(feature = "grpc")]
pub mod grpc;

#[cfg(feature = "http")]
pub mod http;

// ==================== RE-EXPORTS ====================

#[cfg(feature = "grpc")]
pub use grpc::GrpcTransport;

#[cfg(feature = "http")]
pub use http::HttpTransport;

// ==================== DEFAULT TRANSPORT SELECTION ====================

/// Creates the default transport based on enabled features.
/// Prefers gRPC when available (standard for Cosmos chains), falls back to HTTP.
pub fn create_default_transport(endpoint: impl Into<String>) -> Box<dyn Transport> {
    #[cfg(feature = "grpc")]
    {
        Box::new(GrpcTransport::new(endpoint))
    }
    #[cfg(all(not(feature = "grpc"), feature = "http"))]
    {
        Box::new(HttpTransport::new(endpoint))
    }
    #[cfg(not(any(feature = "grpc", feature = "http")))]
    {
        let _ = endpoint;
        Box::new(DummyTransport)
    }
}

// ==================== FALLBACK DUMMY (when no transport feature enabled) ====================

#[cfg(not(any(feature = "grpc", feature = "http")))]
#[derive(Clone, Debug)]
struct DummyTransport;

#[cfg(not(any(feature = "grpc", feature = "http")))]
#[async_trait(?Send)]
impl Transport for DummyTransport {
    async fn broadcast_tx(&self, _tx_bytes: Vec<u8>) -> Result<BroadcastResult, SdkError> {
        Err(SdkError::transport(
            "No transport backend enabled. Enable 'grpc' or 'http' feature.",
        ))
    }
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;
    use morpheum_sdk_core::SdkConfig;

    #[tokio::test]
    async fn default_transport_creates() {
        let transport = create_default_transport("https://sentry.morpheum.xyz");
        let result = transport.broadcast_tx(vec![]).await;
        // In real builds with features enabled, this would succeed or fail based on network
        // For test, we just ensure it compiles and runs
        let _ = result;
    }
}