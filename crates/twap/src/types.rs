//! Domain types for the TWAP module.
//!
//! Covers TWAP data, per-market and module-level configuration,
//! sliding-window entries/snapshots, and streaming events.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::twap::v1 as proto;

// ====================== DOMAIN TYPES ======================

/// Computed TWAP value for a market/window pair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TwapData {
    pub market_index: u64,
    pub window_blocks: u32,
    pub value: u64,
    pub sequence_id: i64,
    pub timestamp: i64,
    pub last_update_block: u64,
}

impl From<proto::TwapData> for TwapData {
    fn from(p: proto::TwapData) -> Self {
        Self {
            market_index: p.market_index,
            window_blocks: p.window_blocks,
            value: p.value,
            sequence_id: p.sequence_id,
            timestamp: p.timestamp,
            last_update_block: p.last_update_block,
        }
    }
}

/// Governance-tunable module parameters (wraps `TwapModuleConfig`).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TwapParams {
    pub module_config: TwapModuleConfig,
}

impl From<proto::Params> for TwapParams {
    fn from(p: proto::Params) -> Self {
        Self {
            module_config: p
                .module_config
                .map(TwapModuleConfig::from)
                .unwrap_or(TwapModuleConfig { default_staleness_blocks: 0 }),
        }
    }
}

impl From<TwapParams> for proto::Params {
    fn from(p: TwapParams) -> Self {
        Self {
            module_config: Some(p.module_config.into()),
        }
    }
}

/// Module-level TWAP configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TwapModuleConfig {
    pub default_staleness_blocks: u64,
}

impl From<proto::TwapModuleConfig> for TwapModuleConfig {
    fn from(p: proto::TwapModuleConfig) -> Self {
        Self { default_staleness_blocks: p.default_staleness_blocks }
    }
}

impl From<TwapModuleConfig> for proto::TwapModuleConfig {
    fn from(c: TwapModuleConfig) -> Self {
        Self { default_staleness_blocks: c.default_staleness_blocks }
    }
}

/// Per-market TWAP configuration (governance-updatable).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarketTwapConfig {
    pub windows: Vec<u32>,
    pub staleness_blocks: u64,
}

impl From<proto::MarketTwapConfig> for MarketTwapConfig {
    fn from(p: proto::MarketTwapConfig) -> Self {
        Self { windows: p.windows, staleness_blocks: p.staleness_blocks }
    }
}

impl From<MarketTwapConfig> for proto::MarketTwapConfig {
    fn from(c: MarketTwapConfig) -> Self {
        Self { windows: c.windows, staleness_blocks: c.staleness_blocks }
    }
}

/// Single sliding window entry (block, price).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WindowEntry {
    pub block: u64,
    pub price: u64,
}

impl From<proto::WindowEntry> for WindowEntry {
    fn from(p: proto::WindowEntry) -> Self {
        Self { block: p.block, price: p.price }
    }
}

/// Serializable snapshot of one (market, window) sliding buffer.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WindowSnapshot {
    pub market_index: u64,
    pub window_blocks: u32,
    pub entries: Vec<WindowEntry>,
    pub sum: String,
    pub capacity: u32,
    pub last_block: u64,
    pub last_price: u64,
    pub last_update_block: u64,
}

impl From<proto::WindowSnapshot> for WindowSnapshot {
    fn from(p: proto::WindowSnapshot) -> Self {
        Self {
            market_index: p.market_index,
            window_blocks: p.window_blocks,
            entries: p.entries.into_iter().map(Into::into).collect(),
            sum: p.sum,
            capacity: p.capacity,
            last_block: p.last_block,
            last_price: p.last_price,
            last_update_block: p.last_update_block,
        }
    }
}

// ====================== STREAM EVENTS ======================

/// Emitted when a TWAP window value changes.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TwapUpdated {
    pub market_index: u64,
    pub window_blocks: u32,
    pub value: u64,
    pub sequence_id: i64,
    pub timestamp: i64,
}

impl From<proto::TwapUpdated> for TwapUpdated {
    fn from(p: proto::TwapUpdated) -> Self {
        Self {
            market_index: p.market_index,
            window_blocks: p.window_blocks,
            value: p.value,
            sequence_id: p.sequence_id,
            timestamp: p.timestamp,
        }
    }
}

/// Union of all TWAP streaming events.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TwapEvent {
    TwapUpdated(TwapUpdated),
}

impl TwapEvent {
    /// Converts from the proto `oneof` wrapper. Returns `None` for empty events.
    pub fn from_proto(e: proto::TwapEvent) -> Option<Self> {
        match e.event? {
            proto::twap_event::Event::TwapUpdated(u) => Some(Self::TwapUpdated(u.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn twap_data_from_proto() {
        let p = proto::TwapData {
            market_index: 1, window_blocks: 300, value: 50_000_000_000,
            sequence_id: 42, timestamp: 1700000000, last_update_block: 9999,
        };
        let d: TwapData = p.into();
        assert_eq!(d.market_index, 1);
        assert_eq!(d.window_blocks, 300);
        assert_eq!(d.value, 50_000_000_000);
    }

    #[test]
    fn market_twap_config_roundtrip() {
        let c = MarketTwapConfig { windows: vec![60, 300, 900], staleness_blocks: 10 };
        let proto_c: proto::MarketTwapConfig = c.clone().into();
        let c2: MarketTwapConfig = proto_c.into();
        assert_eq!(c, c2);
    }

    #[test]
    fn window_snapshot_from_proto() {
        let p = proto::WindowSnapshot {
            market_index: 1, window_blocks: 60,
            entries: vec![proto::WindowEntry { block: 100, price: 5000 }],
            sum: "500000".into(), capacity: 60,
            last_block: 100, last_price: 5000, last_update_block: 100,
        };
        let s: WindowSnapshot = p.into();
        assert_eq!(s.entries.len(), 1);
        assert_eq!(s.entries[0].price, 5000);
    }

    #[test]
    fn twap_event_from_proto() {
        let e = proto::TwapEvent {
            event: Some(proto::twap_event::Event::TwapUpdated(proto::TwapUpdated {
                market_index: 1, window_blocks: 300, value: 42000,
                sequence_id: 1, timestamp: 1700000000,
            })),
        };
        let parsed = TwapEvent::from_proto(e).unwrap();
        match parsed {
            TwapEvent::TwapUpdated(u) => assert_eq!(u.value, 42000),
        }
    }

    #[test]
    fn twap_event_empty_returns_none() {
        let e = proto::TwapEvent { event: None };
        assert!(TwapEvent::from_proto(e).is_none());
    }
}
