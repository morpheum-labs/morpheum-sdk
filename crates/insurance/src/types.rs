//! Domain types for the insurance vault module.
//!
//! Covers vault balances, LP stakes, bad debt records, impermanent loss
//! metrics, threshold status, and streaming vault update events.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::insurance::v1 as proto;

// ====================== SHARED ENUMS ======================

/// Chain type for multi-chain address derivation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ChainType {
    Unspecified,
    Ethereum,
    Solana,
    Bitcoin,
}

impl From<i32> for ChainType {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Ethereum,
            2 => Self::Solana,
            3 => Self::Bitcoin,
            _ => Self::Unspecified,
        }
    }
}

impl From<ChainType> for i32 {
    fn from(t: ChainType) -> Self {
        match t {
            ChainType::Unspecified => 0,
            ChainType::Ethereum => 1,
            ChainType::Solana => 2,
            ChainType::Bitcoin => 3,
        }
    }
}

// ====================== DOMAIN TYPES ======================

/// Vault balance for an asset. Monetary values are satoshi-format strings (u256).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VaultBalance {
    pub asset_index: u64,
    pub asset_symbol: String,
    pub total_balance: String,
    pub available_balance: String,
    pub reserved_balance: String,
    pub min_threshold: String,
    pub updated_at: u64,
}

impl From<proto::VaultBalance> for VaultBalance {
    fn from(p: proto::VaultBalance) -> Self {
        let (asset_index, asset_symbol) = p.asset.map_or((0, String::new()), |a| (a.asset_index, a.symbol));
        Self {
            asset_index,
            asset_symbol,
            total_balance: p.total_balance,
            available_balance: p.available_balance,
            reserved_balance: p.reserved_balance,
            min_threshold: p.min_threshold,
            updated_at: p.updated_at.map_or(0, |t| t.seconds as u64),
        }
    }
}

/// LP stake held by a provider. Monetary values are satoshi-format strings (u256).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LpStake {
    pub stake_id: String,
    pub address: String,
    pub asset_index: u64,
    pub asset_symbol: String,
    pub amount: String,
    pub shares: String,
    pub pending_yield: String,
    pub stake_time: u64,
    pub last_claim_time: u64,
    pub external_address: Option<String>,
    pub chain_type: Option<ChainType>,
}

impl From<proto::LpStake> for LpStake {
    fn from(p: proto::LpStake) -> Self {
        let (asset_index, asset_symbol) = p.asset.map_or((0, String::new()), |a| (a.asset_index, a.symbol));
        Self {
            stake_id: p.stake_id,
            address: p.address,
            asset_index,
            asset_symbol,
            amount: p.amount,
            shares: p.shares,
            pending_yield: p.pending_yield,
            stake_time: p.stake_time.map_or(0, |t| t.seconds as u64),
            last_claim_time: p.last_claim_time.map_or(0, |t| t.seconds as u64),
            external_address: p.external_address,
            chain_type: p.chain_type.map(ChainType::from),
        }
    }
}

/// Record of bad debt absorbed by the insurance vault.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BadDebtRecord {
    pub record_id: String,
    pub position_id: String,
    pub market_index: u64,
    pub asset_index: u64,
    pub asset_symbol: String,
    pub deficit_amount: String,
    pub recovered_amount: String,
    pub gap_amount: String,
    pub absorber_address: String,
    pub timestamp: u64,
    pub event_type: String,
    pub absorber_external_address: Option<String>,
    pub absorber_chain_type: Option<ChainType>,
}

impl From<proto::BadDebtRecord> for BadDebtRecord {
    fn from(p: proto::BadDebtRecord) -> Self {
        let (asset_index, asset_symbol) = p.asset.map_or((0, String::new()), |a| (a.asset_index, a.symbol));
        Self {
            record_id: p.record_id,
            position_id: p.position_id,
            market_index: p.market_index,
            asset_index,
            asset_symbol,
            deficit_amount: p.deficit_amount,
            recovered_amount: p.recovered_amount,
            gap_amount: p.gap_amount,
            absorber_address: p.absorber_address,
            timestamp: p.timestamp.map_or(0, |t| t.seconds as u64),
            event_type: p.event_type,
            absorber_external_address: p.absorber_external_address,
            absorber_chain_type: p.absorber_chain_type.map(ChainType::from),
        }
    }
}

/// Impermanent loss metrics for an asset.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IlMetrics {
    pub asset_index: u64,
    pub asset_symbol: String,
    pub current_il: String,
    pub il_1h_avg: String,
    pub il_24h_avg: String,
    pub timestamp: u64,
}

impl From<proto::IlMetrics> for IlMetrics {
    fn from(p: proto::IlMetrics) -> Self {
        let (asset_index, asset_symbol) = p.asset.map_or((0, String::new()), |a| (a.asset_index, a.symbol));
        Self {
            asset_index,
            asset_symbol,
            current_il: p.current_il,
            il_1h_avg: p.il_1h_avg,
            il_24h_avg: p.il_24h_avg,
            timestamp: p.timestamp.map_or(0, |t| t.seconds as u64),
        }
    }
}

/// Vault threshold status for an asset.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ThresholdStatus {
    pub asset_index: u64,
    pub asset_symbol: String,
    pub near_depletion: bool,
    pub paused: bool,
    pub depletion_percentage: String,
    pub last_checked: u64,
}

impl From<proto::ThresholdStatus> for ThresholdStatus {
    fn from(p: proto::ThresholdStatus) -> Self {
        let (asset_index, asset_symbol) = p.asset.map_or((0, String::new()), |a| (a.asset_index, a.symbol));
        Self {
            asset_index,
            asset_symbol,
            near_depletion: p.near_depletion,
            paused: p.paused,
            depletion_percentage: p.depletion_percentage,
            last_checked: p.last_checked.map_or(0, |t| t.seconds as u64),
        }
    }
}

/// Pagination metadata for list responses.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PageInfo {
    pub next_key: Vec<u8>,
    pub total: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chain_type_roundtrip() {
        for t in [ChainType::Ethereum, ChainType::Solana, ChainType::Bitcoin] {
            let v: i32 = t.into();
            assert_eq!(t, ChainType::from(v));
        }
        assert_eq!(ChainType::Unspecified, ChainType::from(99));
    }

    #[test]
    fn vault_balance_from_proto() {
        let p = proto::VaultBalance {
            asset: Some(morpheum_proto::primitives::v1::Asset {
                asset_index: 1,
                symbol: "USDC".into(),
                ..Default::default()
            }),
            total_balance: "1000000".into(),
            available_balance: "800000".into(),
            reserved_balance: "200000".into(),
            min_threshold: "100000".into(),
            updated_at: Some(morpheum_proto::google::protobuf::Timestamp { seconds: 1_700_000_000, nanos: 0 }),
        };
        let v: VaultBalance = p.into();
        assert_eq!(v.asset_index, 1);
        assert_eq!(v.asset_symbol, "USDC");
        assert_eq!(v.total_balance, "1000000");
        assert_eq!(v.updated_at, 1_700_000_000);
    }

    #[test]
    fn vault_balance_missing_asset() {
        let p = proto::VaultBalance {
            asset: None,
            total_balance: "0".into(),
            ..Default::default()
        };
        let v: VaultBalance = p.into();
        assert_eq!(v.asset_index, 0);
        assert!(v.asset_symbol.is_empty());
    }

    #[test]
    fn lp_stake_from_proto() {
        let p = proto::LpStake {
            stake_id: "abc123".into(),
            address: "morph1xyz".into(),
            asset: Some(morpheum_proto::primitives::v1::Asset {
                asset_index: 2,
                symbol: "ETH".into(),
                ..Default::default()
            }),
            amount: "500".into(),
            shares: "500".into(),
            pending_yield: "10".into(),
            stake_time: Some(morpheum_proto::google::protobuf::Timestamp { seconds: 1_700_000_000, nanos: 0 }),
            last_claim_time: None,
            external_address: Some("0xdead".into()),
            chain_type: Some(1),
        };
        let s: LpStake = p.into();
        assert_eq!(s.stake_id, "abc123");
        assert_eq!(s.chain_type, Some(ChainType::Ethereum));
        assert_eq!(s.external_address, Some("0xdead".into()));
        assert_eq!(s.last_claim_time, 0);
    }

    #[test]
    fn threshold_status_from_proto() {
        let p = proto::ThresholdStatus {
            asset: None,
            near_depletion: true,
            paused: false,
            depletion_percentage: "8500".into(),
            last_checked: Some(morpheum_proto::google::protobuf::Timestamp { seconds: 1_700_000_000, nanos: 0 }),
        };
        let s: ThresholdStatus = p.into();
        assert!(s.near_depletion);
        assert!(!s.paused);
        assert_eq!(s.depletion_percentage, "8500");
    }
}
