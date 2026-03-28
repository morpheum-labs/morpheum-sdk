//! Domain types for the liquidity pool module.
//!
//! Covers pool types, statuses, provider types, pool state,
//! LP positions, depth metrics, and pool health.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::liquidity::v1 as proto;

// ====================== ENUMS ======================

/// Type of liquidity pool.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PoolType {
    Unspecified,
    ProtocolOwned,
    Community,
}

impl From<i32> for PoolType {
    fn from(v: i32) -> Self {
        match v { 1 => Self::ProtocolOwned, 2 => Self::Community, _ => Self::Unspecified }
    }
}

impl From<PoolType> for i32 {
    fn from(t: PoolType) -> Self {
        match t { PoolType::Unspecified => 0, PoolType::ProtocolOwned => 1, PoolType::Community => 2 }
    }
}

/// Current status of a liquidity pool.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PoolStatus {
    Unspecified,
    Active,
    Paused,
    Rebalancing,
    LowDepth,
}

impl From<i32> for PoolStatus {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Active, 2 => Self::Paused,
            3 => Self::Rebalancing, 4 => Self::LowDepth,
            _ => Self::Unspecified,
        }
    }
}

impl From<PoolStatus> for i32 {
    fn from(s: PoolStatus) -> Self {
        match s {
            PoolStatus::Unspecified => 0, PoolStatus::Active => 1, PoolStatus::Paused => 2,
            PoolStatus::Rebalancing => 3, PoolStatus::LowDepth => 4,
        }
    }
}

/// Pluggable AMM engine type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LiquidityProviderType {
    Unspecified,
    Concentrated,
    StableSwap,
    ConstantProduct,
    ProtocolVault,
}

impl From<u32> for LiquidityProviderType {
    fn from(v: u32) -> Self {
        match v {
            1 => Self::Concentrated, 2 => Self::StableSwap,
            3 => Self::ConstantProduct, 4 => Self::ProtocolVault,
            _ => Self::Unspecified,
        }
    }
}

impl From<LiquidityProviderType> for u32 {
    fn from(t: LiquidityProviderType) -> Self {
        match t {
            LiquidityProviderType::Unspecified => 0, LiquidityProviderType::Concentrated => 1,
            LiquidityProviderType::StableSwap => 2, LiquidityProviderType::ConstantProduct => 3,
            LiquidityProviderType::ProtocolVault => 4,
        }
    }
}

// ====================== DOMAIN TYPES ======================

/// Liquidity pool. Monetary values are satoshi-format strings (u256).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pool {
    pub pool_id: String,
    pub market_index: u64,
    pub asset_index: u64,
    pub asset_symbol: String,
    pub total_liquidity: String,
    pub available_liquidity: String,
    pub reserved_liquidity: String,
    pub target_liquidity: String,
    pub pool_type: PoolType,
    pub status: PoolStatus,
    pub created_at: u64,
    pub updated_at: u64,
    pub provider_type: LiquidityProviderType,
    pub provider_config: Vec<u8>,
    pub depth_2pct_bid: String,
    pub depth_2pct_ask: String,
    pub health_score_bps: u32,
    pub display_name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub logo_uri: String,
}

impl From<proto::Pool> for Pool {
    fn from(p: proto::Pool) -> Self {
        let (asset_index, asset_symbol) = p.asset.map_or((0, String::new()), |a| (a.asset_index, a.symbol));
        Self {
            pool_id: p.pool_id,
            market_index: p.market_index,
            asset_index,
            asset_symbol,
            total_liquidity: p.total_liquidity,
            available_liquidity: p.available_liquidity,
            reserved_liquidity: p.reserved_liquidity,
            target_liquidity: p.target_liquidity,
            pool_type: PoolType::from(p.r#type),
            status: PoolStatus::from(p.status),
            created_at: p.created_at.map_or(0, |t| t.seconds as u64),
            updated_at: p.updated_at.map_or(0, |t| t.seconds as u64),
            provider_type: LiquidityProviderType::from(p.provider_type),
            provider_config: p.provider_config,
            depth_2pct_bid: p.depth_2pct_bid,
            depth_2pct_ask: p.depth_2pct_ask,
            health_score_bps: p.health_score_bps,
            display_name: p.display_name,
            description: p.description,
            tags: p.tags,
            logo_uri: p.logo_uri,
        }
    }
}

/// LP position held by a provider.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LpPosition {
    pub position_id: String,
    pub address: String,
    pub pool_id: String,
    pub asset_index: u64,
    pub asset_symbol: String,
    pub amount: String,
    pub shares: String,
    pub pending_yield: String,
    pub deposit_time: u64,
    pub last_claim_time: u64,
    pub external_address: Option<String>,
    pub chain_type: Option<i32>,
}

impl From<proto::LpPosition> for LpPosition {
    fn from(p: proto::LpPosition) -> Self {
        let (asset_index, asset_symbol) = p.asset.map_or((0, String::new()), |a| (a.asset_index, a.symbol));
        Self {
            position_id: p.position_id,
            address: p.address,
            pool_id: p.pool_id,
            asset_index,
            asset_symbol,
            amount: p.amount,
            shares: p.shares,
            pending_yield: p.pending_yield,
            deposit_time: p.deposit_time.map_or(0, |t| t.seconds as u64),
            last_claim_time: p.last_claim_time.map_or(0, |t| t.seconds as u64),
            external_address: p.external_address,
            chain_type: p.chain_type,
        }
    }
}

/// Depth metrics for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DepthMetrics {
    pub market_index: u64,
    pub bid_depth: String,
    pub ask_depth: String,
    pub spread: String,
    pub timestamp: u64,
}

impl From<proto::DepthMetrics> for DepthMetrics {
    fn from(p: proto::DepthMetrics) -> Self {
        Self {
            market_index: p.market_index,
            bid_depth: p.bid_depth,
            ask_depth: p.ask_depth,
            spread: p.spread,
            timestamp: p.timestamp.map_or(0, |t| t.seconds as u64),
        }
    }
}

/// Pool health metrics.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PoolHealth {
    pub pool_id: String,
    pub health_score: String,
    pub utilization_rate: String,
    pub apy: String,
    pub timestamp: u64,
}

impl From<proto::PoolHealth> for PoolHealth {
    fn from(p: proto::PoolHealth) -> Self {
        Self {
            pool_id: p.pool_id,
            health_score: p.health_score,
            utilization_rate: p.utilization_rate,
            apy: p.apy,
            timestamp: p.timestamp.map_or(0, |t| t.seconds as u64),
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
    fn pool_type_roundtrip() {
        for t in [PoolType::ProtocolOwned, PoolType::Community] {
            let v: i32 = t.into();
            assert_eq!(t, PoolType::from(v));
        }
    }

    #[test]
    fn pool_status_roundtrip() {
        for s in [PoolStatus::Active, PoolStatus::Paused, PoolStatus::Rebalancing, PoolStatus::LowDepth] {
            let v: i32 = s.into();
            assert_eq!(s, PoolStatus::from(v));
        }
    }

    #[test]
    fn provider_type_roundtrip() {
        for t in [LiquidityProviderType::Concentrated, LiquidityProviderType::StableSwap,
                  LiquidityProviderType::ConstantProduct, LiquidityProviderType::ProtocolVault] {
            let v: u32 = t.into();
            assert_eq!(t, LiquidityProviderType::from(v));
        }
    }

    #[test]
    fn pool_from_proto() {
        let p = proto::Pool {
            pool_id: "abc".into(), market_index: 1,
            asset: Some(morpheum_proto::primitives::v1::Asset {
                asset_index: 2, symbol: "USDC".into(), ..Default::default()
            }),
            total_liquidity: "1000".into(), available_liquidity: "800".into(),
            reserved_liquidity: "200".into(), target_liquidity: "1200".into(),
            r#type: 1, status: 1,
            created_at: None, updated_at: None,
            provider_type: 1, provider_config: Vec::new(),
            depth_2pct_bid: "500".into(), depth_2pct_ask: "500".into(),
            health_score_bps: 9500,
            display_name: "Main Pool".into(), description: String::new(),
            tags: Vec::new(), logo_uri: String::new(),
        };
        let pool: Pool = p.into();
        assert_eq!(pool.pool_id, "abc");
        assert_eq!(pool.pool_type, PoolType::ProtocolOwned);
        assert_eq!(pool.provider_type, LiquidityProviderType::Concentrated);
        assert_eq!(pool.health_score_bps, 9500);
    }

    #[test]
    fn depth_metrics_from_proto() {
        let p = proto::DepthMetrics {
            market_index: 42, bid_depth: "5000".into(), ask_depth: "4500".into(),
            spread: "100".into(), timestamp: None,
        };
        let d: DepthMetrics = p.into();
        assert_eq!(d.market_index, 42);
        assert_eq!(d.bid_depth, "5000");
    }
}
