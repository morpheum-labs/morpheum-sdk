//! Domain types for the Bank module.
//!
//! Clean, idiomatic Rust representations of key bank protobuf concepts.
//! Provides type safety, ergonomic APIs, and full round-trip conversion
//! to/from protobuf while remaining strictly `no_std` compatible.

use alloc::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::primitives::v1 as primitives_proto;

// ====================== ENUMS ======================

/// Multi-chain type identifier for cross-chain operations.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ChainType {
    #[default]
    Unspecified,
    Ethereum,
    Solana,
    Bitcoin,
}

impl From<i32> for ChainType {
    fn from(v: i32) -> Self {
        match primitives_proto::ChainType::try_from(v) {
            Ok(primitives_proto::ChainType::Ethereum) => Self::Ethereum,
            Ok(primitives_proto::ChainType::Solana) => Self::Solana,
            Ok(primitives_proto::ChainType::Bitcoin) => Self::Bitcoin,
            _ => Self::Unspecified,
        }
    }
}

impl From<ChainType> for i32 {
    fn from(c: ChainType) -> Self {
        match c {
            ChainType::Unspecified => primitives_proto::ChainType::Unspecified as i32,
            ChainType::Ethereum => primitives_proto::ChainType::Ethereum as i32,
            ChainType::Solana => primitives_proto::ChainType::Solana as i32,
            ChainType::Bitcoin => primitives_proto::ChainType::Bitcoin as i32,
        }
    }
}

/// Asset identifier for operations that accept either an index or a symbol.
///
/// Many bank operations (CrossChainTransfer, Deposit, Withdraw, BridgeAsset)
/// use a `oneof assetIdentifier` pattern. `ByIndex` is preferred for O(1)
/// lookup in the asset registry; `BySymbol` is a convenience fallback.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AssetIdentifier {
    ByIndex(u64),
    BySymbol(String),
}

impl AssetIdentifier {
    pub fn index(idx: u64) -> Self {
        Self::ByIndex(idx)
    }

    pub fn symbol(sym: impl Into<String>) -> Self {
        Self::BySymbol(sym.into())
    }
}

// ====================== BALANCE ======================

/// Balance information for an account's asset holding.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Balance {
    pub asset_index: u64,
    pub asset_symbol: String,
    pub balance: String,
    pub available_balance: String,
    pub locked_balance: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_type_roundtrip() {
        for ct in [
            ChainType::Unspecified,
            ChainType::Ethereum,
            ChainType::Solana,
            ChainType::Bitcoin,
        ] {
            let i: i32 = ct.into();
            let back = ChainType::from(i);
            assert_eq!(ct, back);
        }
    }

    #[test]
    fn asset_identifier_constructors() {
        let by_idx = AssetIdentifier::index(42);
        assert_eq!(by_idx, AssetIdentifier::ByIndex(42));

        let by_sym = AssetIdentifier::symbol("MORM");
        assert_eq!(by_sym, AssetIdentifier::BySymbol("MORM".into()));
    }
}
