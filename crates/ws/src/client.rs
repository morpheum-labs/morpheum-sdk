//! Public [`WsClient`] handle — the main entry point for consumers.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use tokio::sync::{mpsc, oneshot};

use crate::connection::{Command, ConnectionActor};
use crate::error::WsError;
use crate::subscription::Subscription;
use crate::types::{AuthCredentials, AuthResponse, ChannelSpec, WsConfig};

/// Command channel buffer size — large enough to avoid blocking the caller
/// during bursts of subscribe/unsubscribe calls.
const CMD_CHANNEL_CAPACITY: usize = 64;

/// Subscribe-only multiplex WebSocket client for the Morpheum streaming layer.
///
/// `WsClient` is a lightweight, cloneable handle backed by an `Arc`. The
/// actual WebSocket connection lives in a background actor task spawned at
/// construction time. Dropping **all** clones of the handle shuts the actor
/// down gracefully.
///
/// # Lifecycle
///
/// 1. **Connect** — [`WsClient::connect`] or [`WsClient::connect_with_config`].
/// 2. **Authenticate** — [`WsClient::authenticate`] (free tier requires no
///    signature; paid tiers require an x402 receipt).
/// 3. **Subscribe** — [`WsClient::subscribe`] returns a [`Subscription`] that
///    implements [`futures::Stream`].
/// 4. **Close** — [`WsClient::close`] or simply drop all handles.
#[derive(Clone)]
pub struct WsClient {
    inner: Arc<ClientInner>,
}

struct ClientInner {
    cmd_tx: mpsc::Sender<Command>,
    connected: Arc<AtomicBool>,
    config: WsConfig,
}

impl WsClient {
    /// Connects to the given WebSocket endpoint with default configuration.
    ///
    /// The initial TCP + TLS + WS handshake is performed eagerly; this method
    /// returns only after the connection is established.
    pub async fn connect(url: &str) -> Result<Self, WsError> {
        Self::connect_with_config(WsConfig::new(url)).await
    }

    /// Connects with a fully customised [`WsConfig`].
    ///
    /// Blocks until the WebSocket handshake completes. If `config.auth` is
    /// set, the client authenticates automatically before returning.
    pub async fn connect_with_config(config: WsConfig) -> Result<Self, WsError> {
        let (cmd_tx, cmd_rx) = mpsc::channel(CMD_CHANNEL_CAPACITY);
        let connected = Arc::new(AtomicBool::new(false));
        let (connect_tx, connect_rx) = oneshot::channel();

        let actor =
            ConnectionActor::new(config.clone(), cmd_rx, Arc::clone(&connected), connect_tx);
        tokio::spawn(actor.run());

        connect_rx.await.map_err(|_| WsError::Closed)??;

        let client = Self {
            inner: Arc::new(ClientInner {
                cmd_tx,
                connected,
                config: config.clone(),
            }),
        };

        if let Some(ref credentials) = config.auth {
            client.authenticate(credentials.clone()).await?;
        }

        Ok(client)
    }

    /// Authenticates with the server using the provided credentials.
    ///
    /// Must be called before subscribing to any channel (unless free-tier
    /// auto-auth was configured via [`WsConfig::auto_auth`]).
    pub async fn authenticate(
        &self,
        credentials: AuthCredentials,
    ) -> Result<AuthResponse, WsError> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.send_command(Command::Authenticate {
            credentials,
            reply: reply_tx,
        })
        .await?;
        reply_rx.await?
    }

    /// Subscribes to a streaming channel and returns a [`Subscription`] handle
    /// that implements [`futures::Stream`].
    ///
    /// The first event on the stream is a full snapshot (`is_snapshot == true`);
    /// subsequent events are deltas.
    pub async fn subscribe(&self, spec: ChannelSpec) -> Result<Subscription, WsError> {
        let (event_tx, event_rx) =
            mpsc::channel(self.inner.config.buffer_capacity);
        let (reply_tx, reply_rx) = oneshot::channel();

        self.send_command(Command::Subscribe {
            spec: spec.clone(),
            sender: event_tx,
            reply: reply_tx,
        })
        .await?;

        reply_rx.await??;

        Ok(Subscription::new(spec, event_rx))
    }

    /// Subscribes to multiple channels concurrently and returns all
    /// [`Subscription`] handles.
    ///
    /// All subscribe requests are sent in parallel over the single multiplexed
    /// connection, reducing total latency from `N * RTT` to approximately
    /// `1 RTT` when the actor processes them in sequence from its command
    /// buffer.
    ///
    /// If any individual subscription fails, the entire batch fails and all
    /// successfully created subscriptions in this batch are dropped.
    pub async fn subscribe_many(
        &self,
        specs: Vec<ChannelSpec>,
    ) -> Result<Vec<Subscription>, WsError> {
        let futs = specs.into_iter().map(|s| self.subscribe(s));
        futures_util::future::try_join_all(futs).await
    }

    /// Removes a subscription. The corresponding [`Subscription`] stream will
    /// end after this call.
    pub async fn unsubscribe(&self, spec: &ChannelSpec) -> Result<(), WsError> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.send_command(Command::Unsubscribe {
            spec: spec.clone(),
            reply: reply_tx,
        })
        .await?;
        reply_rx.await?
    }

    /// Returns `true` if the underlying WebSocket connection is currently open.
    pub fn is_connected(&self) -> bool {
        self.inner.connected.load(Ordering::SeqCst)
    }

    /// Gracefully closes the WebSocket connection and shuts down the background
    /// actor.
    pub async fn close(&self) -> Result<(), WsError> {
        self.send_command(Command::Close).await
    }

    // ── Internal ─────────────────────────────────────────────────────────

    async fn send_command(&self, cmd: Command) -> Result<(), WsError> {
        self.inner
            .cmd_tx
            .send(cmd)
            .await
            .map_err(|_| WsError::Closed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Compile-time assertions — WsClient must be Send + Sync + Clone.
    fn _assert_send_sync_clone<T: Send + Sync + Clone>() {}

    #[test]
    fn ws_client_is_send_sync_clone() {
        _assert_send_sync_clone::<WsClient>();
    }
}
