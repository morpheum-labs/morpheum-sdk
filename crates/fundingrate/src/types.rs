//! Domain types for the funding-rate module.
//!
//! Covers funding-rate calculation results, application modes,
//! market profiles, module configuration, and funding events.

use alloc::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::fundingrate::v1 as proto;

// ====================== ENUMS ======================

/// How funding is realized per position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FundingApplicationMode {
    Unspecified,
    /// Full signed payment (standard zero-sum).
    BothSides,
    /// Skip deduction when payer is in unrealized loss.
    UpsideOnly,
    /// Full apply + configurable skim to treasury/LP.
    MormBoost,
}

impl From<i32> for FundingApplicationMode {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::BothSides, 2 => Self::UpsideOnly, 3 => Self::MormBoost,
            _ => Self::Unspecified,
        }
    }
}

impl From<FundingApplicationMode> for i32 {
    fn from(m: FundingApplicationMode) -> Self {
        match m {
            FundingApplicationMode::Unspecified => 0, FundingApplicationMode::BothSides => 1,
            FundingApplicationMode::UpsideOnly => 2, FundingApplicationMode::MormBoost => 3,
        }
    }
}

/// Leverage model used for funding calculation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FundingType {
    Unspecified,
    Linear,
    Power,
}

impl From<i32> for FundingType {
    fn from(v: i32) -> Self {
        match v { 1 => Self::Linear, 2 => Self::Power, _ => Self::Unspecified }
    }
}

impl From<FundingType> for i32 {
    fn from(t: FundingType) -> Self {
        match t { FundingType::Unspecified => 0, FundingType::Linear => 1, FundingType::Power => 2 }
    }
}

// ====================== DOMAIN TYPES ======================

/// Per-market funding-rate calculation result. All monetary values in satoshi (1e8).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FundingRateData {
    pub market_index: u64,
    pub symbol: String,
    pub funding_rate: i64,
    pub mark_price: u64,
    pub index_price: u64,
    pub ema_funding_rate: i64,
    pub next_funding_time: u64,
}

impl From<proto::FundingRateData> for FundingRateData {
    fn from(p: proto::FundingRateData) -> Self {
        Self {
            market_index: p.market_index, symbol: p.symbol,
            funding_rate: p.funding_rate, mark_price: p.mark_price,
            index_price: p.index_price, ema_funding_rate: p.ema_funding_rate,
            next_funding_time: p.next_funding_time,
        }
    }
}

/// Per-market funding application configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FundingMarketProfile {
    pub mode: FundingApplicationMode,
    pub vrf_bias_bps: u32,
    pub protocol_cut_bps: u32,
    pub lp_incentive_bps: u32,
}

impl From<proto::FundingMarketProfile> for FundingMarketProfile {
    fn from(p: proto::FundingMarketProfile) -> Self {
        Self {
            mode: FundingApplicationMode::from(p.mode),
            vrf_bias_bps: p.vrf_bias_bps,
            protocol_cut_bps: p.protocol_cut_bps,
            lp_incentive_bps: p.lp_incentive_bps,
        }
    }
}

impl From<FundingMarketProfile> for proto::FundingMarketProfile {
    fn from(p: FundingMarketProfile) -> Self {
        Self {
            mode: i32::from(p.mode),
            vrf_bias_bps: p.vrf_bias_bps,
            protocol_cut_bps: p.protocol_cut_bps,
            lp_incentive_bps: p.lp_incentive_bps,
        }
    }
}

/// Minimal position data needed for funding application.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FundingPosition {
    pub address: String,
    pub position_id: String,
    pub size: u64,
    pub entry_price: u64,
    pub is_long: bool,
    pub leverage: u64,
    pub power: u64,
    pub entries_count: u32,
}

impl From<FundingPosition> for proto::FundingPosition {
    fn from(p: FundingPosition) -> Self {
        Self {
            address: p.address, position_id: p.position_id,
            size: p.size, entry_price: p.entry_price, is_long: p.is_long,
            leverage: p.leverage, power: p.power, entries_count: p.entries_count,
        }
    }
}

/// Module-level funding rate configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FundingRateConfig {
    pub payment_interval_secs: u64,
    pub enable_linear_leverage: bool,
    pub enable_power_leverage: bool,
    pub enable_vrf: bool,
    pub worker_count: u32,
    pub ema_lambda: u64,
    pub max_funding_rate: i64,
    pub min_funding_rate: i64,
    pub default_volatility_sigma: u64,
}

impl From<proto::FundingRateConfig> for FundingRateConfig {
    fn from(p: proto::FundingRateConfig) -> Self {
        Self {
            payment_interval_secs: p.payment_interval_secs,
            enable_linear_leverage: p.enable_linear_leverage,
            enable_power_leverage: p.enable_power_leverage,
            enable_vrf: p.enable_vrf,
            worker_count: p.worker_count,
            ema_lambda: p.ema_lambda,
            max_funding_rate: p.max_funding_rate,
            min_funding_rate: p.min_funding_rate,
            default_volatility_sigma: p.default_volatility_sigma,
        }
    }
}

// ====================== EVENTS ======================

/// Emitted for each position that receives a funding payment.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FundingApplied {
    pub address: String,
    pub market_index: u64,
    pub payment: i64,
    pub shortfall: u64,
    pub profile_mode: u32,
    pub funding_type: FundingType,
    pub timestamp: u64,
    pub size: u64,
    pub is_long: bool,
    pub mark_price: u64,
    pub index_price: u64,
}

impl From<proto::FundingApplied> for FundingApplied {
    fn from(p: proto::FundingApplied) -> Self {
        Self {
            address: p.address, market_index: p.market_index,
            payment: p.payment, shortfall: p.shortfall,
            profile_mode: p.profile_mode, funding_type: FundingType::from(p.funding_type),
            timestamp: p.timestamp, size: p.size, is_long: p.is_long,
            mark_price: p.mark_price, index_price: p.index_price,
        }
    }
}

/// Emitted when UpsideOnly skips a deduction.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FundingShortfall {
    pub market_index: u64,
    pub shortfall_satoshi: u64,
    pub timestamp: u64,
}

impl From<proto::FundingShortfall> for FundingShortfall {
    fn from(p: proto::FundingShortfall) -> Self {
        Self { market_index: p.market_index, shortfall_satoshi: p.shortfall_satoshi, timestamp: p.timestamp }
    }
}

/// Emitted for MormBoost treasury/LP routing.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MormFundingCutEvent {
    pub market_index: u64,
    pub treasury_amount: u64,
    pub lp_reward_amount: u64,
}

impl From<proto::MormFundingCutEvent> for MormFundingCutEvent {
    fn from(p: proto::MormFundingCutEvent) -> Self {
        Self { market_index: p.market_index, treasury_amount: p.treasury_amount, lp_reward_amount: p.lp_reward_amount }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_mode_roundtrip() {
        for m in [FundingApplicationMode::BothSides, FundingApplicationMode::UpsideOnly, FundingApplicationMode::MormBoost] {
            let v: i32 = m.into();
            assert_eq!(m, FundingApplicationMode::from(v));
        }
    }

    #[test]
    fn funding_type_roundtrip() {
        for t in [FundingType::Linear, FundingType::Power] {
            let v: i32 = t.into();
            assert_eq!(t, FundingType::from(v));
        }
    }

    #[test]
    fn funding_rate_data_from_proto() {
        let p = proto::FundingRateData {
            market_index: 42, symbol: "BTC-USDC-PERP".into(),
            funding_rate: -500, mark_price: 5_000_000_000_000,
            index_price: 4_999_000_000_000, ema_funding_rate: -480,
            next_funding_time: 1_700_028_800,
        };
        let d: FundingRateData = p.into();
        assert_eq!(d.market_index, 42);
        assert_eq!(d.funding_rate, -500);
    }

    #[test]
    fn market_profile_bidirectional() {
        let profile = FundingMarketProfile {
            mode: FundingApplicationMode::MormBoost,
            vrf_bias_bps: 50, protocol_cut_bps: 500, lp_incentive_bps: 200,
        };
        let proto_profile: proto::FundingMarketProfile = profile.clone().into();
        let back: FundingMarketProfile = proto_profile.into();
        assert_eq!(profile, back);
    }
}
