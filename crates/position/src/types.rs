//! Domain types for the Position module.
//!
//! Clean, idiomatic Rust representations of the position protobuf messages.
//! They provide type safety, ergonomic APIs, and full round-trip conversion
//! to/from protobuf while remaining strictly `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::position::v1 as proto;

// ====================== ENUMS ======================

/// Position side — Long or Short.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PositionSide {
    Unspecified,
    Long,
    Short,
}

impl From<i32> for PositionSide {
    fn from(v: i32) -> Self {
        match proto::PositionSide::try_from(v).unwrap_or(proto::PositionSide::Unspecified) {
            proto::PositionSide::Unspecified => Self::Unspecified,
            proto::PositionSide::Long => Self::Long,
            proto::PositionSide::Short => Self::Short,
        }
    }
}

impl From<PositionSide> for i32 {
    fn from(s: PositionSide) -> Self {
        match s {
            PositionSide::Unspecified => proto::PositionSide::Unspecified as i32,
            PositionSide::Long => proto::PositionSide::Long as i32,
            PositionSide::Short => proto::PositionSide::Short as i32,
        }
    }
}

// ====================== POSITION ENTRY ======================

/// A single partial-fill entry within a position.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PositionEntry {
    pub size: u64,
    pub price: u64,
}

impl From<proto::PositionEntry> for PositionEntry {
    fn from(p: proto::PositionEntry) -> Self {
        Self {
            size: p.size,
            price: p.price,
        }
    }
}

impl From<PositionEntry> for proto::PositionEntry {
    fn from(e: PositionEntry) -> Self {
        Self {
            size: e.size,
            price: e.price,
        }
    }
}

// ====================== POSITION STATE ======================

/// Internal module state for a position (keeper representation).
///
/// This is the canonical type returned by query RPCs.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PositionState {
    pub address: String,
    pub bucket_id: u64,
    pub market_index: u64,
    pub size: u64,
    pub entry_price: u64,
    pub leverage: u32,
    pub power: u32,
    pub side: PositionSide,
    pub unrealized_pnl: i64,
    pub entries: Vec<PositionEntry>,
}

impl From<proto::PositionState> for PositionState {
    fn from(p: proto::PositionState) -> Self {
        Self {
            address: p.address,
            bucket_id: p.bucket_id,
            market_index: p.market_index,
            size: p.size,
            entry_price: p.entry_price,
            leverage: p.leverage,
            power: p.power,
            side: PositionSide::from(p.side),
            unrealized_pnl: p.unrealized_pnl,
            entries: p.entries.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<PositionState> for proto::PositionState {
    fn from(s: PositionState) -> Self {
        Self {
            address: s.address,
            bucket_id: s.bucket_id,
            market_index: s.market_index,
            size: s.size,
            entry_price: s.entry_price,
            leverage: s.leverage,
            power: s.power,
            side: i32::from(s.side),
            unrealized_pnl: s.unrealized_pnl,
            entries: s.entries.into_iter().map(Into::into).collect(),
        }
    }
}

// ====================== POSITION CONFIG ======================

/// Module-level configuration for the position module.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PositionConfig {
    pub reduce_fifo: bool,
    pub top_trader_count: u32,
    pub snapshot_interval_blocks: u64,
}

impl Default for PositionConfig {
    fn default() -> Self {
        Self {
            reduce_fifo: true,
            top_trader_count: 100,
            snapshot_interval_blocks: 100,
        }
    }
}

impl From<proto::PositionConfig> for PositionConfig {
    fn from(p: proto::PositionConfig) -> Self {
        Self {
            reduce_fifo: p.reduce_fifo,
            top_trader_count: p.top_trader_count,
            snapshot_interval_blocks: p.snapshot_interval_blocks,
        }
    }
}

impl From<PositionConfig> for proto::PositionConfig {
    fn from(c: PositionConfig) -> Self {
        Self {
            reduce_fifo: c.reduce_fifo,
            top_trader_count: c.top_trader_count,
            snapshot_interval_blocks: c.snapshot_interval_blocks,
        }
    }
}

// ====================== LONG/SHORT VOLUME ======================

/// Aggregated long/short volume for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LongShortVolume {
    pub long_volume: u64,
    pub short_volume: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn position_state_roundtrip() {
        let state = PositionState {
            address: "morpheum1abc".into(),
            bucket_id: 1,
            market_index: 42,
            size: 1000,
            entry_price: 50000,
            leverage: 10,
            power: 2,
            side: PositionSide::Long,
            unrealized_pnl: 500,
            entries: vec![
                PositionEntry { size: 500, price: 49000 },
                PositionEntry { size: 500, price: 51000 },
            ],
        };

        let proto_state: proto::PositionState = state.clone().into();
        let back: PositionState = proto_state.into();
        assert_eq!(state, back);
    }

    #[test]
    fn position_side_roundtrip() {
        for side in [PositionSide::Unspecified, PositionSide::Long, PositionSide::Short] {
            let proto_val: i32 = side.into();
            let back: PositionSide = proto_val.into();
            assert_eq!(side, back);
        }
    }

    #[test]
    fn position_config_roundtrip() {
        let config = PositionConfig {
            reduce_fifo: true,
            top_trader_count: 50,
            snapshot_interval_blocks: 200,
        };

        let proto_config: proto::PositionConfig = config.clone().into();
        let back: PositionConfig = proto_config.into();
        assert_eq!(config, back);
    }

    #[test]
    fn position_entry_roundtrip() {
        let entry = PositionEntry { size: 100, price: 45000 };
        let proto_entry: proto::PositionEntry = entry.clone().into();
        let back: PositionEntry = proto_entry.into();
        assert_eq!(entry, back);
    }
}
