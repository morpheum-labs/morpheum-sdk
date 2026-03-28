//! Domain types for the Morpheum WebSocket client.
//!
//! Provides channel specifications, authentication, configuration, reconnection
//! policies, and the [`StreamEvent`] payload delivered to subscription consumers.

use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::WsError;

// ─── ChannelSpec ─────────────────────────────────────────────────────────────

/// Specifies a streaming channel to subscribe to.
///
/// Constructed via typed factory methods for well-known channels, or
/// [`ChannelSpec::custom`] for arbitrary / future channels. The fields are
/// serialized into the JSON `"subscription"` object on the wire.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChannelSpec {
    /// Channel identifier (e.g. `"l2Book"`, `"priceFeed"`, `"agentState"`).
    #[serde(rename = "type")]
    pub channel_type: String,

    /// Asset / market symbol, when applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,

    /// Symbol list (pricefeed multi-symbol subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,

    /// Kline / candle interval (e.g. `"1m"`, `"1h"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,

    /// Orderbook depth limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,

    /// Agent DID for agent-specific subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,

    /// User address for user-specific subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

impl ChannelSpec {
    /// Returns a canonical routing key used internally to match incoming
    /// server frames to the correct subscription channel.
    pub fn routing_key(&self) -> String {
        let mut key = self.channel_type.clone();
        if let Some(ref coin) = self.coin {
            key.push(':');
            key.push_str(coin);
        }
        if let Some(ref agent_id) = self.agent_id {
            key.push(':');
            key.push_str(agent_id);
        }
        if let Some(ref address) = self.address {
            key.push(':');
            key.push_str(address);
        }
        if let Some(ref interval) = self.interval {
            key.push(':');
            key.push_str(interval);
        }
        key
    }

    // ── Constructors: market data ────────────────────────────────────────

    fn with_coin(channel_type: &str, coin: impl Into<String>) -> Self {
        Self {
            channel_type: channel_type.into(),
            coin: Some(coin.into()),
            symbols: None,
            interval: None,
            depth: None,
            agent_id: None,
            address: None,
        }
    }

    fn bare(channel_type: &str) -> Self {
        Self {
            channel_type: channel_type.into(),
            coin: None,
            symbols: None,
            interval: None,
            depth: None,
            agent_id: None,
            address: None,
        }
    }

    /// All mid-prices across markets.
    pub fn all_mids() -> Self {
        Self::bare("allMids")
    }

    /// Level-2 orderbook for a given coin.
    pub fn l2_book(coin: impl Into<String>) -> Self {
        Self::with_coin("l2Book", coin)
    }

    /// Level-2 orderbook with depth limit.
    pub fn l2_book_depth(coin: impl Into<String>, depth: u32) -> Self {
        let mut s = Self::l2_book(coin);
        s.depth = Some(depth);
        s
    }

    /// Trade feed for a coin.
    pub fn trades(coin: impl Into<String>) -> Self {
        Self::with_coin("trades", coin)
    }

    /// Best bid/offer for a coin.
    pub fn bbo(coin: impl Into<String>) -> Self {
        Self::with_coin("bbo", coin)
    }

    /// Candlestick / kline data.
    pub fn candle(coin: impl Into<String>, interval: impl Into<String>) -> Self {
        let mut s = Self::with_coin("candle", coin);
        s.interval = Some(interval.into());
        s
    }

    /// Active asset context.
    pub fn active_asset_ctx() -> Self {
        Self::bare("activeAssetCtx")
    }

    // ── Constructors: price / oracle ─────────────────────────────────────

    /// Price feed subscription (multi-symbol).
    pub fn price_feed(symbols: &[impl AsRef<str>]) -> Self {
        Self {
            channel_type: "priceFeed".into(),
            coin: None,
            symbols: Some(symbols.iter().map(|s| s.as_ref().to_owned()).collect()),
            interval: None,
            depth: None,
            agent_id: None,
            address: None,
        }
    }

    /// Mark price stream.
    pub fn mark_price(coin: impl Into<String>) -> Self {
        Self::with_coin("markPrice", coin)
    }

    /// Funding rate stream.
    pub fn funding_rate(coin: impl Into<String>) -> Self {
        Self::with_coin("fundingRate", coin)
    }

    /// TWAP stream.
    pub fn twap(coin: impl Into<String>) -> Self {
        Self::with_coin("twap", coin)
    }

    // ── Constructors: CLOB / exchange ────────────────────────────────────

    /// CLOB aggregate stream for a coin.
    pub fn clob(coin: impl Into<String>) -> Self {
        Self::with_coin("clob", coin)
    }

    /// Risk events for a coin.
    pub fn risk(coin: impl Into<String>) -> Self {
        Self::with_coin("risk", coin)
    }

    /// Position updates for a coin.
    pub fn position(coin: impl Into<String>) -> Self {
        Self::with_coin("position", coin)
    }

    /// Bucket updates for a coin.
    pub fn bucket(coin: impl Into<String>) -> Self {
        Self::with_coin("bucket", coin)
    }

    /// Balance / bank updates for a coin.
    pub fn balances(coin: impl Into<String>) -> Self {
        Self::with_coin("balances", coin)
    }

    /// Market status updates for a coin.
    pub fn market(coin: impl Into<String>) -> Self {
        Self::with_coin("market", coin)
    }

    // ── Constructors: DeFi modules ───────────────────────────────────────

    /// Vault updates.
    pub fn vault(coin: impl Into<String>) -> Self {
        Self::with_coin("vault", coin)
    }

    /// Treasury updates.
    pub fn treasury(coin: impl Into<String>) -> Self {
        Self::with_coin("treasury", coin)
    }

    /// Vesting events.
    pub fn vesting(coin: impl Into<String>) -> Self {
        Self::with_coin("vesting", coin)
    }

    /// Token events.
    pub fn token(coin: impl Into<String>) -> Self {
        Self::with_coin("token", coin)
    }

    /// Insurance events.
    pub fn insurance(coin: impl Into<String>) -> Self {
        Self::with_coin("insurance", coin)
    }

    /// Staking / reward events.
    pub fn staking(coin: impl Into<String>) -> Self {
        Self::with_coin("staking", coin)
    }

    /// CLAMM pool events.
    pub fn clamm(coin: impl Into<String>) -> Self {
        Self::with_coin("clamm", coin)
    }

    /// CLAMM graduation events.
    pub fn clamm_grad(coin: impl Into<String>) -> Self {
        Self::with_coin("clammgrad", coin)
    }

    /// Bonding curve events.
    pub fn bonding_curve(coin: impl Into<String>) -> Self {
        Self::with_coin("bondingcurve", coin)
    }

    // ── Constructors: governance ─────────────────────────────────────────

    /// Governance proposal updates.
    pub fn governance(coin: impl Into<String>) -> Self {
        Self::with_coin("governance", coin)
    }

    /// DAO proposal updates.
    pub fn dao(coin: impl Into<String>) -> Self {
        Self::with_coin("dao", coin)
    }

    /// Upgrade events.
    pub fn upgrade(coin: impl Into<String>) -> Self {
        Self::with_coin("upgrade", coin)
    }

    // ── Constructors: prediction / outcome ───────────────────────────────

    /// Prediction market events.
    pub fn prediction(coin: impl Into<String>) -> Self {
        Self::with_coin("prediction", coin)
    }

    /// Outcome feed events.
    pub fn outcome_feed(coin: impl Into<String>) -> Self {
        Self::with_coin("outcomeFeed", coin)
    }

    /// OSA (on-chain settlement account) events.
    pub fn osa(coin: impl Into<String>) -> Self {
        Self::with_coin("osa", coin)
    }

    // ── Constructors: agent / AI pillar ──────────────────────────────────

    /// Agent state stream (by agent DID).
    pub fn agent_state(agent_id: impl Into<String>) -> Self {
        Self {
            channel_type: "agentState".into(),
            coin: None,
            symbols: None,
            interval: None,
            depth: None,
            agent_id: Some(agent_id.into()),
            address: None,
        }
    }

    /// Agent registry events.
    pub fn agentreg(coin: impl Into<String>) -> Self {
        Self::with_coin("agentreg", coin)
    }

    /// Identity events.
    pub fn identity(coin: impl Into<String>) -> Self {
        Self::with_coin("identity", coin)
    }

    /// Reputation events.
    pub fn reputation(coin: impl Into<String>) -> Self {
        Self::with_coin("reputation", coin)
    }

    /// Validation / proof events.
    pub fn validation(coin: impl Into<String>) -> Self {
        Self::with_coin("validation", coin)
    }

    /// Agent memory events.
    pub fn memory(coin: impl Into<String>) -> Self {
        Self::with_coin("memory", coin)
    }

    /// Job events.
    pub fn job(coin: impl Into<String>) -> Self {
        Self::with_coin("job", coin)
    }

    /// Intent events.
    pub fn intent(coin: impl Into<String>) -> Self {
        Self::with_coin("intent", coin)
    }

    /// Agent directory events.
    pub fn directory(coin: impl Into<String>) -> Self {
        Self::with_coin("directory", coin)
    }

    /// Verifiable-credential events.
    pub fn vc(coin: impl Into<String>) -> Self {
        Self::with_coin("vc", coin)
    }

    /// Marketplace events.
    pub fn marketplace(coin: impl Into<String>) -> Self {
        Self::with_coin("marketplace", coin)
    }

    /// Inference registry events.
    pub fn inferreg(coin: impl Into<String>) -> Self {
        Self::with_coin("inferreg", coin)
    }

    // ── Constructors: cross-chain / payments ─────────────────────────────

    /// Interop / bridge events.
    pub fn interop(coin: impl Into<String>) -> Self {
        Self::with_coin("interop", coin)
    }

    /// x402 payment events.
    pub fn x402(coin: impl Into<String>) -> Self {
        Self::with_coin("x402", coin)
    }

    // ── Constructors: infrastructure ─────────────────────────────────────

    /// Consensus / block events.
    pub fn consensus(coin: impl Into<String>) -> Self {
        Self::with_coin("consensus", coin)
    }

    /// Auth / account events.
    pub fn auth(coin: impl Into<String>) -> Self {
        Self::with_coin("auth", coin)
    }

    // ── Constructors: user-specific ──────────────────────────────────────

    /// User fills (requires address).
    pub fn user_fills(address: impl Into<String>) -> Self {
        let mut s = Self::bare("userFills");
        s.address = Some(address.into());
        s
    }

    /// User order updates (requires address).
    pub fn order_updates(address: impl Into<String>) -> Self {
        let mut s = Self::bare("orderUpdates");
        s.address = Some(address.into());
        s
    }

    /// User funding events (requires address).
    pub fn user_fundings(address: impl Into<String>) -> Self {
        let mut s = Self::bare("userFundings");
        s.address = Some(address.into());
        s
    }

    /// Clearinghouse state for a user (requires address).
    pub fn clearinghouse_state(address: impl Into<String>) -> Self {
        let mut s = Self::bare("clearinghouseState");
        s.address = Some(address.into());
        s
    }

    /// Open orders for a user (requires address).
    pub fn open_orders(address: impl Into<String>) -> Self {
        let mut s = Self::bare("openOrders");
        s.address = Some(address.into());
        s
    }

    // ── Generic constructor ──────────────────────────────────────────────

    /// Subscribe to an arbitrary channel type not covered by the typed
    /// constructors. Use this for new or module-specific channels.
    pub fn custom(channel_type: impl Into<String>) -> Self {
        Self {
            channel_type: channel_type.into(),
            coin: None,
            symbols: None,
            interval: None,
            depth: None,
            agent_id: None,
            address: None,
        }
    }

    /// Attach a coin filter to a custom channel spec.
    pub fn with_coin_filter(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }

    /// Attach an agent_id filter to any channel spec.
    pub fn with_agent_id(mut self, agent_id: impl Into<String>) -> Self {
        self.agent_id = Some(agent_id.into());
        self
    }

    /// Attach a user address filter to any channel spec.
    pub fn with_address(mut self, address: impl Into<String>) -> Self {
        self.address = Some(address.into());
        self
    }
}

// ─── StreamTier ──────────────────────────────────────────────────────────────

/// Subscription tier that governs quota and access level.
///
/// Maps to the tiers defined in the streaming design document: free discovery,
/// basic paid, premium paid, and raw-core (feature-gated on the node).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamTier {
    #[default]
    Free,
    Basic,
    Premium,
    #[serde(rename = "rawcore")]
    RawCore,
}

// ─── AuthCredentials ─────────────────────────────────────────────────────────

/// Credentials sent during the initial authentication handshake.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthCredentials {
    /// x402 payment signature (required for paid tiers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,

    /// Requested subscription tier.
    pub tier: StreamTier,

    /// Agent DID (optional, for agent-specific quotas).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

impl AuthCredentials {
    /// Free-tier credentials (no signature required).
    pub fn free() -> Self {
        Self {
            signature: None,
            tier: StreamTier::Free,
            agent_id: None,
        }
    }

    /// Basic paid-tier credentials.
    pub fn basic(signature: impl Into<String>, agent_id: impl Into<String>) -> Self {
        Self {
            signature: Some(signature.into()),
            tier: StreamTier::Basic,
            agent_id: Some(agent_id.into()),
        }
    }

    /// Premium paid-tier credentials.
    pub fn premium(signature: impl Into<String>, agent_id: impl Into<String>) -> Self {
        Self {
            signature: Some(signature.into()),
            tier: StreamTier::Premium,
            agent_id: Some(agent_id.into()),
        }
    }

    /// Raw-core tier credentials (feature-gated on the node, requires minimum
    /// reputation score).
    pub fn raw_core(signature: impl Into<String>, agent_id: impl Into<String>) -> Self {
        Self {
            signature: Some(signature.into()),
            tier: StreamTier::RawCore,
            agent_id: Some(agent_id.into()),
        }
    }
}

// ─── AuthResponse ────────────────────────────────────────────────────────────

/// Server response after a successful authentication handshake.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Tier that the server granted (may differ from requested tier if
    /// the node only supports free-tier).
    pub tier: StreamTier,

    /// x402 receipt identifier (present for paid tiers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_id: Option<String>,
}

// ─── StreamEvent ─────────────────────────────────────────────────────────────

/// A single event received on a subscription channel.
///
/// Each event carries the raw JSON payload as emitted by the node. Callers
/// that need typed access to proto-compatible structures can use
/// [`try_decode`](Self::try_decode).
#[derive(Clone, Debug)]
pub struct StreamEvent {
    /// Channel that produced this event (e.g. `"l2Book"`, `"trades"`).
    pub channel: String,

    /// Raw JSON payload from the server.
    pub data: serde_json::Value,

    /// `true` for the first message after subscribing (snapshot),
    /// `false` for subsequent delta updates.
    pub is_snapshot: bool,
}

impl StreamEvent {
    /// Attempt to deserialize the raw JSON payload into a concrete type.
    ///
    /// ```rust,ignore
    /// let book: L2BookData = event.try_decode()?;
    /// ```
    pub fn try_decode<T: serde::de::DeserializeOwned>(&self) -> Result<T, WsError> {
        serde_json::from_value(self.data.clone()).map_err(WsError::from)
    }
}

// ─── ReconnectPolicy ─────────────────────────────────────────────────────────

/// Controls automatic reconnection behaviour when the WebSocket drops.
///
/// On reconnect the client re-authenticates with cached credentials and
/// re-subscribes to every active channel automatically.
#[derive(Clone, Debug)]
pub enum ReconnectPolicy {
    /// Never reconnect — the client errors on disconnect.
    Disabled,

    /// Reconnect with a fixed delay between attempts.
    FixedDelay {
        interval: Duration,
        max_retries: Option<u32>,
    },

    /// Reconnect with exponential back-off.
    ExponentialBackoff {
        initial: Duration,
        max: Duration,
        max_retries: Option<u32>,
    },
}

impl Default for ReconnectPolicy {
    fn default() -> Self {
        Self::ExponentialBackoff {
            initial: Duration::from_millis(500),
            max: Duration::from_secs(30),
            max_retries: None,
        }
    }
}

// ─── WsConfig ────────────────────────────────────────────────────────────────

/// Configuration for [`WsClient`](crate::WsClient).
#[derive(Clone, Debug)]
pub struct WsConfig {
    /// WebSocket endpoint URL (e.g. `wss://api.morpheum.xyz/ws`).
    pub url: String,

    /// Reconnection policy applied when the connection drops.
    pub reconnect_policy: ReconnectPolicy,

    /// Per-subscription channel buffer capacity (default: 256).
    pub buffer_capacity: usize,

    /// If set, the client authenticates automatically on connect.
    pub auth: Option<AuthCredentials>,
}

impl WsConfig {
    /// Creates a minimal config pointing at the given URL.
    pub fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            reconnect_policy: ReconnectPolicy::default(),
            buffer_capacity: 256,
            auth: None,
        }
    }

    /// Sets the reconnection policy.
    pub fn reconnect_policy(mut self, policy: ReconnectPolicy) -> Self {
        self.reconnect_policy = policy;
        self
    }

    /// Sets the per-subscription channel buffer capacity.
    pub fn buffer_capacity(mut self, cap: usize) -> Self {
        self.buffer_capacity = cap;
        self
    }

    /// Enables automatic authentication on connect with the given credentials.
    pub fn auto_auth(mut self, credentials: AuthCredentials) -> Self {
        self.auth = Some(credentials);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn channel_spec_routing_key_simple() {
        let spec = ChannelSpec::l2_book("BTC");
        assert_eq!(spec.routing_key(), "l2Book:BTC");
    }

    #[test]
    fn channel_spec_routing_key_bare() {
        let spec = ChannelSpec::all_mids();
        assert_eq!(spec.routing_key(), "allMids");
    }

    #[test]
    fn channel_spec_routing_key_agent() {
        let spec = ChannelSpec::agent_state("did:morpheum:123");
        assert_eq!(spec.routing_key(), "agentState:did:morpheum:123");
    }

    #[test]
    fn channel_spec_custom_with_coin() {
        let spec = ChannelSpec::custom("myModule").with_coin_filter("ETH");
        assert_eq!(spec.channel_type, "myModule");
        assert_eq!(spec.coin.as_deref(), Some("ETH"));
    }

    #[test]
    fn stream_tier_default_is_free() {
        assert_eq!(StreamTier::default(), StreamTier::Free);
    }

    #[test]
    fn auth_credentials_free() {
        let creds = AuthCredentials::free();
        assert!(creds.signature.is_none());
        assert_eq!(creds.tier, StreamTier::Free);
    }

    #[test]
    fn ws_config_builder() {
        let cfg = WsConfig::new("wss://test.morpheum.xyz/ws")
            .buffer_capacity(512)
            .reconnect_policy(ReconnectPolicy::Disabled);
        assert_eq!(cfg.buffer_capacity, 512);
        assert!(matches!(cfg.reconnect_policy, ReconnectPolicy::Disabled));
    }

    #[test]
    fn stream_tier_serializes_lowercase() {
        assert_eq!(serde_json::to_string(&StreamTier::Free).unwrap(), "\"free\"");
        assert_eq!(serde_json::to_string(&StreamTier::RawCore).unwrap(), "\"rawcore\"");
    }

    #[test]
    fn channel_spec_serialization_roundtrip() {
        let spec = ChannelSpec::l2_book_depth("BTC", 25);
        let json = serde_json::to_string(&spec).unwrap();
        let back: ChannelSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(spec, back);
    }
}
