//! Subscription handle returned by [`WsClient::subscribe`](crate::WsClient::subscribe).
//!
//! Implements [`futures::Stream`] so callers can consume events with
//! `StreamExt::next`, `select!`, or any combinator from the futures ecosystem.

use std::pin::Pin;
use std::task::{Context, Poll};

use futures_util::Stream;
use tokio::sync::mpsc;

use crate::types::{ChannelSpec, StreamEvent};
use crate::WsError;

/// An active subscription to a single streaming channel.
///
/// Created by [`WsClient::subscribe`](crate::WsClient::subscribe).
/// Events flow from the internal connection actor to this handle via a
/// bounded `mpsc` channel. The stream ends when the subscription is
/// explicitly cancelled, the connection shuts down, or the actor drops
/// the sender side.
pub struct Subscription {
    spec: ChannelSpec,
    rx: mpsc::Receiver<Result<StreamEvent, WsError>>,
}

impl Subscription {
    pub(crate) fn new(
        spec: ChannelSpec,
        rx: mpsc::Receiver<Result<StreamEvent, WsError>>,
    ) -> Self {
        Self { spec, rx }
    }

    /// Returns the channel specification this subscription was created from.
    pub fn channel_spec(&self) -> &ChannelSpec {
        &self.spec
    }
}

impl Stream for Subscription {
    type Item = Result<StreamEvent, WsError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.rx.poll_recv(cx)
    }
}
