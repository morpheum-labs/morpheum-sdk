//! JSON wire protocol for the Morpheum multiplex WebSocket endpoint.
//!
//! Mirrors the framing defined in the streaming implementation doc (§3.2).
//! Client sends `auth`, `subscribe`, `unsubscribe` messages; server responds
//! with `auth`, `subscriptionResponse`, `error`, or typed data frames.

use serde::{Deserialize, Serialize};

use crate::types::{AuthCredentials, ChannelSpec, StreamTier};

// ─── Client → Server ─────────────────────────────────────────────────────────

/// A message sent from the SDK client to the server.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "method", rename_all = "camelCase")]
pub enum ClientMessage {
    /// Authentication handshake (must be the first message after connect).
    Auth { data: AuthData },

    /// Subscribe to a channel.
    Subscribe { subscription: ChannelSpec },

    /// Unsubscribe from a channel.
    Unsubscribe { subscription: ChannelSpec },
}

/// Payload of the `auth` client message.
#[derive(Clone, Debug, Serialize)]
pub struct AuthData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    pub tier: StreamTier,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

impl From<AuthCredentials> for AuthData {
    fn from(c: AuthCredentials) -> Self {
        Self {
            signature: c.signature,
            tier: c.tier,
            agent_id: c.agent_id,
        }
    }
}

impl ClientMessage {
    /// Build an `Auth` message from credentials.
    pub fn auth(credentials: AuthCredentials) -> Self {
        Self::Auth {
            data: credentials.into(),
        }
    }

    /// Build a `Subscribe` message.
    pub fn subscribe(spec: ChannelSpec) -> Self {
        Self::Subscribe {
            subscription: spec,
        }
    }

    /// Build an `Unsubscribe` message.
    pub fn unsubscribe(spec: ChannelSpec) -> Self {
        Self::Unsubscribe {
            subscription: spec,
        }
    }
}

// ─── Server → Client ─────────────────────────────────────────────────────────

/// A message received from the server.
///
/// The server uses the `"channel"` field as a discriminator. Well-known control
/// channels (`auth`, `subscriptionResponse`, `error`) are parsed into typed
/// variants; everything else is captured as a [`Data`](ServerMessage::Data)
/// frame.
#[derive(Clone, Debug, Deserialize)]
#[serde(from = "RawServerMessage")]
pub enum ServerMessage {
    /// Authentication response.
    Auth(AuthResponseData),

    /// Subscription confirmation / rejection.
    SubscriptionResponse(SubscriptionResponseData),

    /// Server-side error.
    Error(ErrorData),

    /// Data frame for a subscribed channel.
    Data {
        channel: String,
        data: serde_json::Value,
    },
}

/// Auth response payload.
#[derive(Clone, Debug, Deserialize)]
pub struct AuthResponseData {
    pub status: String,
    #[serde(default)]
    pub tier: Option<StreamTier>,
    #[serde(default)]
    pub receipt_id: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Subscription confirmation payload.
#[derive(Clone, Debug, Deserialize)]
pub struct SubscriptionResponseData {
    #[serde(default)]
    pub success: bool,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub subscription: Option<ChannelSpec>,
}

/// Server error payload.
#[derive(Clone, Debug, Deserialize)]
pub struct ErrorData {
    pub message: String,
}

// ─── Internal: serde helper for untagged server messages ─────────────────────

/// Raw representation used as an intermediate deserialization step so that
/// unknown `channel` values fall through to the `Data` variant instead of
/// producing an error.
#[derive(Deserialize)]
struct RawServerMessage {
    channel: String,
    #[serde(default)]
    data: serde_json::Value,
    // Flatten catches any extra top-level fields the server may add.
    #[serde(flatten)]
    _extra: serde_json::Map<String, serde_json::Value>,
}

impl From<RawServerMessage> for ServerMessage {
    fn from(raw: RawServerMessage) -> Self {
        match raw.channel.as_str() {
            "auth" => {
                match serde_json::from_value::<AuthResponseData>(raw.data.clone()) {
                    Ok(auth) => ServerMessage::Auth(auth),
                    Err(_) => ServerMessage::Error(ErrorData {
                        message: format!("malformed auth response: {}", raw.data),
                    }),
                }
            }
            "subscriptionResponse" => {
                match serde_json::from_value::<SubscriptionResponseData>(raw.data.clone()) {
                    Ok(sub) => ServerMessage::SubscriptionResponse(sub),
                    Err(_) => ServerMessage::Error(ErrorData {
                        message: format!("malformed subscription response: {}", raw.data),
                    }),
                }
            }
            "error" => {
                match serde_json::from_value::<ErrorData>(raw.data.clone()) {
                    Ok(err) => ServerMessage::Error(err),
                    Err(_) => ServerMessage::Error(ErrorData {
                        message: raw.data.to_string(),
                    }),
                }
            }
            channel => ServerMessage::Data {
                channel: channel.to_owned(),
                data: raw.data,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AuthCredentials;

    #[test]
    fn client_auth_serializes_correctly() {
        let msg = ClientMessage::auth(AuthCredentials::free());
        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["method"], "auth");
        assert_eq!(json["data"]["tier"], "free");
    }

    #[test]
    fn client_subscribe_serializes_correctly() {
        let msg = ClientMessage::subscribe(ChannelSpec::l2_book("BTC"));
        let json = serde_json::to_value(&msg).unwrap();
        assert_eq!(json["method"], "subscribe");
        assert_eq!(json["subscription"]["type"], "l2Book");
        assert_eq!(json["subscription"]["coin"], "BTC");
    }

    #[test]
    fn server_auth_ok_parses() {
        let json = r#"{"channel":"auth","data":{"status":"ok","tier":"basic","receipt_id":"r123"}}"#;
        let msg: ServerMessage = serde_json::from_str(json).unwrap();
        match msg {
            ServerMessage::Auth(a) => {
                assert_eq!(a.status, "ok");
                assert_eq!(a.tier, Some(StreamTier::Basic));
                assert_eq!(a.receipt_id.as_deref(), Some("r123"));
            }
            _ => panic!("expected Auth variant"),
        }
    }

    #[test]
    fn server_data_frame_parses() {
        let json = r#"{"channel":"l2Book","data":{"coin":"BTC","levels":[]}}"#;
        let msg: ServerMessage = serde_json::from_str(json).unwrap();
        match msg {
            ServerMessage::Data { channel, data } => {
                assert_eq!(channel, "l2Book");
                assert_eq!(data["coin"], "BTC");
            }
            _ => panic!("expected Data variant"),
        }
    }

    #[test]
    fn server_error_parses() {
        let json = r#"{"channel":"error","data":{"message":"quota exceeded"}}"#;
        let msg: ServerMessage = serde_json::from_str(json).unwrap();
        match msg {
            ServerMessage::Error(e) => assert_eq!(e.message, "quota exceeded"),
            _ => panic!("expected Error variant"),
        }
    }

    #[test]
    fn server_subscription_response_parses() {
        let json = r#"{"channel":"subscriptionResponse","data":{"success":true}}"#;
        let msg: ServerMessage = serde_json::from_str(json).unwrap();
        match msg {
            ServerMessage::SubscriptionResponse(s) => assert!(s.success),
            _ => panic!("expected SubscriptionResponse variant"),
        }
    }

    #[test]
    fn unknown_channel_becomes_data() {
        let json = r#"{"channel":"customModule","data":{"foo":"bar"}}"#;
        let msg: ServerMessage = serde_json::from_str(json).unwrap();
        assert!(matches!(msg, ServerMessage::Data { .. }));
    }
}
