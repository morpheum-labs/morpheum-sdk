//! Internal connection actor that owns the WebSocket and routes messages.
//!
//! The actor is spawned by [`WsClient::connect`](crate::WsClient::connect) and
//! communicates with the client handle exclusively through typed channels. It is
//! not part of the public API.

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::sync::{mpsc, oneshot};
use tokio_tungstenite::tungstenite::Message as WsMessage;
use tracing::{debug, error, info, warn};

use crate::error::WsError;
use crate::protocol::{ClientMessage, ServerMessage};
use crate::types::{
    AuthCredentials, AuthResponse, ChannelSpec, ReconnectPolicy, StreamEvent, WsConfig,
};

// ─── Command channel ─────────────────────────────────────────────────────────

/// Commands sent from [`WsClient`](crate::WsClient) to the actor.
pub(crate) enum Command {
    Authenticate {
        credentials: AuthCredentials,
        reply: oneshot::Sender<Result<AuthResponse, WsError>>,
    },
    Subscribe {
        spec: ChannelSpec,
        sender: mpsc::Sender<Result<StreamEvent, WsError>>,
        reply: oneshot::Sender<Result<(), WsError>>,
    },
    Unsubscribe {
        spec: ChannelSpec,
        reply: oneshot::Sender<Result<(), WsError>>,
    },
    Close,
}

// ─── Subscription bookkeeping ────────────────────────────────────────────────

struct SubscriptionEntry {
    spec: ChannelSpec,
    sender: mpsc::Sender<Result<StreamEvent, WsError>>,
    snapshot_received: bool,
}

// ─── Actor ───────────────────────────────────────────────────────────────────

pub(crate) struct ConnectionActor {
    config: WsConfig,
    cmd_rx: mpsc::Receiver<Command>,
    connected: Arc<AtomicBool>,
    subscriptions: HashMap<String, SubscriptionEntry>,
    cached_auth: Option<AuthCredentials>,
    /// One-shot signal for the initial connection attempt. `Some` only for
    /// the first call to `connect_and_serve`; consumed on success or failure.
    connect_signal: Option<oneshot::Sender<Result<(), WsError>>>,
}

impl ConnectionActor {
    pub(crate) fn new(
        config: WsConfig,
        cmd_rx: mpsc::Receiver<Command>,
        connected: Arc<AtomicBool>,
        connect_signal: oneshot::Sender<Result<(), WsError>>,
    ) -> Self {
        Self {
            config,
            cmd_rx,
            connected,
            subscriptions: HashMap::new(),
            cached_auth: None,
            connect_signal: Some(connect_signal),
        }
    }

    /// Main entry point — runs until the client drops the command channel or
    /// an explicit `Close` command is received.
    pub(crate) async fn run(mut self) {
        loop {
            match self.connect_and_serve().await {
                Ok(ShutdownReason::GracefulClose) => {
                    debug!("connection actor shutting down gracefully");
                    break;
                }
                Ok(ShutdownReason::CommandChannelClosed) => {
                    debug!("all WsClient handles dropped — shutting down");
                    break;
                }
                Err(err) => {
                    warn!(%err, "WebSocket connection lost");
                    self.connected.store(false, Ordering::SeqCst);
                    self.notify_subscribers_error(&err);

                    if !self.wait_before_reconnect().await {
                        error!("reconnection exhausted or disabled — actor exiting");
                        break;
                    }
                }
            }
        }
        self.connected.store(false, Ordering::SeqCst);
    }

    // ── Core loop ────────────────────────────────────────────────────────

    async fn connect_and_serve(&mut self) -> Result<ShutdownReason, WsError> {
        let (ws_stream, _) = match tokio_tungstenite::connect_async(&self.config.url).await {
            Ok(pair) => pair,
            Err(e) => {
                let err = WsError::from(e);
                if let Some(signal) = self.connect_signal.take() {
                    let _ = signal.send(Err(WsError::Connection(err.to_string())));
                }
                return Err(err);
            }
        };
        self.connected.store(true, Ordering::SeqCst);
        info!(url = %self.config.url, "WebSocket connected");

        if let Some(signal) = self.connect_signal.take() {
            let _ = signal.send(Ok(()));
        }

        let (mut ws_sink, mut ws_source) = ws_stream.split();

        // On reconnect: re-authenticate and re-subscribe with fresh snapshot state.
        if let Some(ref creds) = self.cached_auth.clone() {
            self.send_auth_to_ws(&mut ws_sink, creds.clone()).await?;
            if let Some(msg) = tokio::time::timeout(
                Duration::from_secs(10),
                ws_source.next(),
            )
            .await
            .ok()
            .flatten()
            {
                if let WsMessage::Text(text) = msg? {
                    if let Ok(ServerMessage::Auth(auth)) = serde_json::from_str::<ServerMessage>(&text) {
                        if auth.status != "ok" {
                            let msg = auth.message.unwrap_or(auth.status);
                            return Err(WsError::Auth(msg));
                        }
                    }
                }
            }
            self.resubscribe_all(&mut ws_sink).await?;
        }

        let mut pending_auth: Option<oneshot::Sender<Result<AuthResponse, WsError>>> = None;
        let mut pending_subs: HashMap<String, oneshot::Sender<Result<(), WsError>>> = HashMap::new();

        loop {
            tokio::select! {
                frame = ws_source.next() => {
                    let frame = match frame {
                        Some(Ok(f)) => f,
                        Some(Err(e)) => return Err(WsError::from(e)),
                        None => return Err(WsError::Closed),
                    };
                    match frame {
                        WsMessage::Text(text) => {
                            if let Err(e) = self.process_server_message(
                                &text,
                                &mut pending_auth,
                                &mut pending_subs,
                            ) {
                                warn!(%e, "error processing server message");
                            }
                        }
                        WsMessage::Close(_) => return Err(WsError::Closed),
                        WsMessage::Ping(payload) => {
                            let _ = ws_sink.send(WsMessage::Pong(payload)).await;
                        }
                        _ => {}
                    }
                }

                cmd = self.cmd_rx.recv() => {
                    let cmd = match cmd {
                        Some(c) => c,
                        None => return Ok(ShutdownReason::CommandChannelClosed),
                    };
                    match cmd {
                        Command::Authenticate { credentials, reply } => {
                            self.cached_auth = Some(credentials.clone());
                            if let Err(e) = self.send_auth_to_ws(&mut ws_sink, credentials).await {
                                let _ = reply.send(Err(e));
                            } else {
                                pending_auth = Some(reply);
                            }
                        }
                        Command::Subscribe { spec, sender, reply } => {
                            let key = spec.routing_key();
                            self.subscriptions.insert(key.clone(), SubscriptionEntry {
                                spec: spec.clone(),
                                sender,
                                snapshot_received: false,
                            });
                            let msg = ClientMessage::subscribe(spec);
                            if let Err(e) = self.send_json(&mut ws_sink, &msg).await {
                                let _ = reply.send(Err(e));
                            } else {
                                pending_subs.insert(key, reply);
                            }
                        }
                        Command::Unsubscribe { spec, reply } => {
                            let key = spec.routing_key();
                            self.subscriptions.remove(&key);
                            pending_subs.remove(&key);
                            let msg = ClientMessage::unsubscribe(spec);
                            match self.send_json(&mut ws_sink, &msg).await {
                                Ok(()) => { let _ = reply.send(Ok(())); }
                                Err(e) => { let _ = reply.send(Err(e)); }
                            }
                        }
                        Command::Close => {
                            let _ = ws_sink.close().await;
                            return Ok(ShutdownReason::GracefulClose);
                        }
                    }
                }
            }
        }
    }

    // ── Message processing ───────────────────────────────────────────────

    fn process_server_message(
        &mut self,
        text: &str,
        pending_auth: &mut Option<oneshot::Sender<Result<AuthResponse, WsError>>>,
        pending_subs: &mut HashMap<String, oneshot::Sender<Result<(), WsError>>>,
    ) -> Result<(), WsError> {
        let msg: ServerMessage = serde_json::from_str(text)?;
        match msg {
            ServerMessage::Auth(auth_data) => {
                if let Some(reply) = pending_auth.take() {
                    if auth_data.status == "ok" {
                        let resp = AuthResponse {
                            tier: auth_data.tier.unwrap_or_default(),
                            receipt_id: auth_data.receipt_id,
                        };
                        let _ = reply.send(Ok(resp));
                    } else {
                        let msg = auth_data
                            .message
                            .unwrap_or_else(|| auth_data.status.clone());
                        let _ = reply.send(Err(WsError::Auth(msg)));
                    }
                }
            }
            ServerMessage::SubscriptionResponse(sub_data) => {
                let matched_key = sub_data
                    .subscription
                    .as_ref()
                    .map(|s| s.routing_key())
                    .and_then(|k| pending_subs.remove(&k));

                let reply = matched_key.or_else(|| {
                    let first_key = pending_subs.keys().next().cloned();
                    first_key.and_then(|k| pending_subs.remove(&k))
                });

                if let Some(reply) = reply {
                    let _ = reply.send(Ok(()));
                }
            }
            ServerMessage::Error(err_data) => {
                if let Some(reply) = pending_auth.take() {
                    let _ = reply.send(Err(WsError::Auth(err_data.message.clone())));
                }
                if let Some((_key, reply)) = pending_subs.drain().next() {
                    let _ = reply.send(Err(WsError::protocol(err_data.message.clone())));
                }
                warn!(message = %err_data.message, "server error");
            }
            ServerMessage::Data { channel, data } => {
                self.route_data_frame(&channel, data);
            }
        }
        Ok(())
    }

    fn route_data_frame(&mut self, channel: &str, data: serde_json::Value) {
        let key = if self.subscriptions.contains_key(channel) {
            Some(channel.to_owned())
        } else {
            self.subscriptions
                .iter()
                .find(|(_, e)| e.spec.channel_type == channel)
                .map(|(k, _)| k.clone())
        };

        let Some(key) = key else {
            debug!(channel, "received data for unknown subscription — ignoring");
            return;
        };

        if let Some(entry) = self.subscriptions.get_mut(&key) {
            let is_snapshot = !entry.snapshot_received;
            entry.snapshot_received = true;

            let event = StreamEvent {
                channel: channel.to_owned(),
                data,
                is_snapshot,
            };
            if entry.sender.try_send(Ok(event)).is_err() {
                warn!(channel, "subscriber lagging — event dropped");
            }
        }
    }

    // ── Wire helpers ─────────────────────────────────────────────────────

    async fn send_json<S>(&self, sink: &mut S, msg: &ClientMessage) -> Result<(), WsError>
    where
        S: futures_util::Sink<WsMessage, Error = tokio_tungstenite::tungstenite::Error>
            + Unpin,
    {
        let text = serde_json::to_string(msg)?;
        sink.send(WsMessage::Text(text.into()))
            .await
            .map_err(WsError::from)
    }

    async fn send_auth_to_ws<S>(
        &self,
        sink: &mut S,
        credentials: AuthCredentials,
    ) -> Result<(), WsError>
    where
        S: futures_util::Sink<WsMessage, Error = tokio_tungstenite::tungstenite::Error>
            + Unpin,
    {
        let msg = ClientMessage::auth(credentials);
        self.send_json(sink, &msg).await
    }

    async fn resubscribe_all<S>(&mut self, sink: &mut S) -> Result<(), WsError>
    where
        S: futures_util::Sink<WsMessage, Error = tokio_tungstenite::tungstenite::Error>
            + Unpin,
    {
        let specs: Vec<ChannelSpec> = self
            .subscriptions
            .values_mut()
            .map(|entry| {
                entry.snapshot_received = false;
                entry.spec.clone()
            })
            .collect();

        for spec in specs {
            let msg = ClientMessage::subscribe(spec);
            self.send_json(sink, &msg).await?;
        }
        Ok(())
    }

    // ── Reconnection ─────────────────────────────────────────────────────

    /// Waits according to the reconnect policy before the next attempt.
    /// Returns `false` if reconnection is disabled or retries are exhausted.
    /// The actual connection retry happens in the outer `run()` loop via
    /// `connect_and_serve`, avoiding the waste of a throwaway probe connection.
    async fn wait_before_reconnect(&mut self) -> bool {
        match self.config.reconnect_policy {
            ReconnectPolicy::Disabled => false,
            ReconnectPolicy::FixedDelay {
                interval,
                ref mut max_retries,
            } => {
                if let Some(ref mut remaining) = max_retries {
                    if *remaining == 0 {
                        return false;
                    }
                    *remaining -= 1;
                }
                info!(delay_ms = %interval.as_millis(), "waiting before reconnect (fixed delay)");
                tokio::time::sleep(interval).await;
                true
            }
            ReconnectPolicy::ExponentialBackoff {
                ref mut initial,
                max,
                ref mut max_retries,
            } => {
                if let Some(ref mut remaining) = max_retries {
                    if *remaining == 0 {
                        return false;
                    }
                    *remaining -= 1;
                }
                let delay = *initial;
                info!(delay_ms = %delay.as_millis(), "waiting before reconnect (exponential backoff)");
                tokio::time::sleep(delay).await;
                *initial = (delay * 2).min(max);
                true
            }
        }
    }

    fn notify_subscribers_error(&self, err: &WsError) {
        let description = err.to_string();
        for entry in self.subscriptions.values() {
            let _ = entry
                .sender
                .try_send(Err(WsError::Connection(description.clone())));
        }
    }
}

enum ShutdownReason {
    GracefulClose,
    CommandChannelClosed,
}
