//! Subscribe-only multiplex WebSocket client for the Morpheum streaming layer.
//!
//! This crate provides a single-connection, multi-subscription client for
//! the Morpheum node's WebSocket endpoint (`wss://api.morpheum.xyz/ws`).
//! All 40+ on-chain data channels (orderbooks, trades, price feeds, agent state,
//! positions, funding rates, and more) are accessible through a unified API.
//!
//! # Architecture
//!
//! The client uses an internal actor model: a single background task owns the
//! WebSocket connection and routes incoming messages to per-subscription
//! channels. The public [`WsClient`] handle is `Clone + Send + Sync` and
//! communicates with the actor via a command channel.
//!
//! # Quick start
//!
//! ```rust,ignore
//! use morpheum_sdk_ws::prelude::*;
//! use futures_util::StreamExt;
//!
//! let client = WsClient::connect("wss://api.morpheum.xyz/ws").await?;
//! client.authenticate(AuthCredentials::free()).await?;
//!
//! let mut book = client.subscribe(ChannelSpec::l2_book("BTC")).await?;
//! while let Some(event) = book.next().await {
//!     let event = event?;
//!     println!("{}: {}", event.channel, event.data);
//! }
//! ```

pub mod error;
pub mod types;
pub mod protocol;
pub mod subscription;
pub mod connection;
pub mod client;

pub use error::WsError;
pub use types::{
    AuthCredentials, AuthResponse, ChannelSpec, ReconnectPolicy, StreamEvent,
    StreamTier, WsConfig,
};
pub use protocol::{ClientMessage, ServerMessage};
pub use subscription::Subscription;
pub use client::WsClient;

/// Recommended imports for most users.
///
/// ```rust,ignore
/// use morpheum_sdk_ws::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{
        AuthCredentials, AuthResponse, ChannelSpec, ReconnectPolicy, StreamEvent,
        StreamTier, Subscription, WsClient, WsConfig, WsError,
    };
}

/// Current crate version (from Cargo.toml).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[test]
    fn version_is_set() {
        assert!(!super::VERSION.is_empty());
    }
}
