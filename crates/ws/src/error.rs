//! Unified error type for the Morpheum WebSocket client.

use std::fmt;

/// All errors produced by the WebSocket client.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum WsError {
    /// WebSocket connection or transport failure.
    #[error("connection error: {0}")]
    Connection(String),

    /// Authentication was rejected by the server.
    #[error("authentication failed: {0}")]
    Auth(String),

    /// Wire-protocol violation (malformed frame, unexpected message type).
    #[error("protocol error: {0}")]
    Protocol(String),

    /// The server rejected a subscription due to quota limits.
    #[error("subscription quota exceeded: {0}")]
    QuotaExceeded(String),

    /// The underlying connection was closed (gracefully or not).
    #[error("connection closed")]
    Closed,

    /// JSON (de)serialization failure on the wire protocol.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Propagated SDK-core error.
    #[error(transparent)]
    Sdk(#[from] morpheum_sdk_core::SdkError),
}

impl WsError {
    /// Convenience constructor for connection errors from any `Display` source.
    pub fn connection(err: impl fmt::Display) -> Self {
        Self::Connection(err.to_string())
    }

    /// Convenience constructor for protocol errors.
    pub fn protocol(msg: impl Into<String>) -> Self {
        Self::Protocol(msg.into())
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for WsError {
    fn from(err: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::Connection(err.to_string())
    }
}

impl From<tokio::sync::oneshot::error::RecvError> for WsError {
    fn from(_: tokio::sync::oneshot::error::RecvError) -> Self {
        Self::Closed
    }
}
