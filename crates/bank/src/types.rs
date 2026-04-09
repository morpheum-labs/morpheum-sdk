//! Domain types for the Bank module.
//!
//! Clean, idiomatic Rust representations of key bank protobuf concepts.
//! Provides type safety, ergonomic APIs, and full round-trip conversion
//! to/from protobuf while remaining strictly `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;

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

// ====================== ASSET ======================

/// On-chain asset metadata from the bank module's asset registry.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Asset {
    pub asset_index: u64,
    pub symbol: String,
    pub asset_type: i32,
    pub decimals: u32,
    pub is_native: bool,
}

/// Response from `query_assets`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AssetsResponse {
    pub assets: Vec<Asset>,
    pub total_count: u64,
}

// ====================== FEE STATS ======================

/// Aggregated bank fee statistics across all revenue-bearing operations.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BankFeeStats {
    pub total_withdrawal_fees: u64,
    pub total_bridge_fees: u64,
    pub total_onboarding_fees: u64,
    pub total_mint_fees: u64,
    pub total_transfer_fees: u64,
    pub total_treasury_swept: u64,
    pub withdrawal_count: u64,
    pub bridge_count: u64,
    pub onboard_count: u64,
    pub mint_count: u64,
    pub transfer_count: u64,
}

impl From<morpheum_proto::bank::v1::QueryBankFeeStatsResponse> for BankFeeStats {
    fn from(resp: morpheum_proto::bank::v1::QueryBankFeeStatsResponse) -> Self {
        Self {
            total_withdrawal_fees: resp.total_withdrawal_fees,
            total_bridge_fees: resp.total_bridge_fees,
            total_onboarding_fees: resp.total_onboarding_fees,
            total_mint_fees: resp.total_mint_fees,
            total_transfer_fees: resp.total_transfer_fees,
            total_treasury_swept: resp.total_treasury_swept,
            withdrawal_count: resp.withdrawal_count,
            bridge_count: resp.bridge_count,
            onboard_count: resp.onboard_count,
            mint_count: resp.mint_count,
            transfer_count: resp.transfer_count,
        }
    }
}

// ====================== ASSET INDEX RESOLUTION ======================

/// Well-known asset name → registry index mapping.
///
/// Centralises the lookup so that CLI, SDK callers, and test harnesses all
/// resolve the same indices without duplicating magic numbers.
pub fn resolve_asset_index(name: &str) -> Result<u64, morpheum_sdk_core::SdkError> {
    match name.to_ascii_uppercase().as_str() {
        "MORM" => Ok(0),
        "USDC" => Ok(1),
        "BTC" => Ok(2),
        "ETH" => Ok(3),
        "USDT" => Ok(4),
        "SOL" => Ok(5),
        _ => Err(morpheum_sdk_core::SdkError::invalid_input(alloc::format!(
            "unknown asset name '{name}' — known: MORM (0), USDC (1), BTC (2), ETH (3), USDT (4), SOL (5)"
        ))),
    }
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

    #[test]
    fn resolve_asset_index_known() {
        assert_eq!(resolve_asset_index("MORM").unwrap(), 0);
        assert_eq!(resolve_asset_index("usdc").unwrap(), 1);
        assert_eq!(resolve_asset_index("Sol").unwrap(), 5);
    }

    #[test]
    fn resolve_asset_index_unknown() {
        assert!(resolve_asset_index("DOGE").is_err());
    }
}
