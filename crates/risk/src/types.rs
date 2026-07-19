//! Domain types for the risk module.
//!
//! Covers heatmaps, margin snapshots, OI analytics, risk configuration,
//! the Dutch-auction lifecycle, and streaming risk events.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::risk::v1 as proto;

// ====================== DOMAIN TYPES ======================

/// Single price band with satoshi-scale aggregates.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HeatmapBand {
    pub notional: String,
    pub oi_long: String,
    pub oi_short: String,
    pub count: u32,
}

impl From<proto::HeatmapBand> for HeatmapBand {
    fn from(p: proto::HeatmapBand) -> Self {
        Self {
            notional: p.notional,
            oi_long: p.oi_long,
            oi_short: p.oi_short,
            count: p.count,
        }
    }
}

/// Full heatmap snapshot for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HeatmapData {
    pub market_index: u64,
    pub mark_price: u64,
    pub bands: Vec<HeatmapBand>,
    pub total_at_risk: String,
}

impl From<proto::HeatmapData> for HeatmapData {
    fn from(p: proto::HeatmapData) -> Self {
        Self {
            market_index: p.market_index,
            mark_price: p.mark_price,
            bands: p.bands.into_iter().map(Into::into).collect(),
            total_at_risk: p.total_at_risk,
        }
    }
}

/// Band index + notional at risk pair.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiquidationPlanBand {
    pub band_index: i32,
    pub notional: String,
}

impl From<proto::LiquidationPlanBand> for LiquidationPlanBand {
    fn from(p: proto::LiquidationPlanBand) -> Self {
        Self {
            band_index: p.band_index,
            notional: p.notional,
        }
    }
}

/// Lightweight risk summary for a bucket.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BucketRiskSummary {
    pub bucket_id: u64,
    pub market_index: u64,
    pub total_notional: String,
    pub at_risk_bands: Vec<LiquidationPlanBand>,
}

impl From<proto::BucketRiskSummary> for BucketRiskSummary {
    fn from(p: proto::BucketRiskSummary) -> Self {
        Self {
            bucket_id: p.bucket_id,
            market_index: p.market_index,
            total_notional: p.total_notional,
            at_risk_bands: p.at_risk_bands.into_iter().map(Into::into).collect(),
        }
    }
}

/// Module configuration (governance hot-reloadable).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RiskConfig {
    pub band_width_bps: u32,
    pub num_bands_above_below: u32,
    pub imbalance_threshold_bps: u32,
    pub imbalance_hysteresis_bps: u32,
    pub max_scan_limit: u32,
    pub liquidation_margin_ratio_bps: u32,
    pub partial_band_shift_enabled: bool,
}

impl From<proto::RiskConfig> for RiskConfig {
    fn from(p: proto::RiskConfig) -> Self {
        Self {
            band_width_bps: p.band_width_bps,
            num_bands_above_below: p.num_bands_above_below,
            imbalance_threshold_bps: p.imbalance_threshold_bps,
            imbalance_hysteresis_bps: p.imbalance_hysteresis_bps,
            max_scan_limit: p.max_scan_limit,
            liquidation_margin_ratio_bps: p.liquidation_margin_ratio_bps,
            partial_band_shift_enabled: p.partial_band_shift_enabled,
        }
    }
}

impl From<RiskConfig> for proto::RiskConfig {
    fn from(c: RiskConfig) -> Self {
        Self {
            band_width_bps: c.band_width_bps,
            num_bands_above_below: c.num_bands_above_below,
            imbalance_threshold_bps: c.imbalance_threshold_bps,
            imbalance_hysteresis_bps: c.imbalance_hysteresis_bps,
            max_scan_limit: c.max_scan_limit,
            liquidation_margin_ratio_bps: c.liquidation_margin_ratio_bps,
            partial_band_shift_enabled: c.partial_band_shift_enabled,
        }
    }
}

/// Full governance-tunable risk module parameters (wire `Params`).
///
/// `config` mirrors [`RiskConfig`]. The remaining sub-configs
/// (`auction_params`, `spot_risk`, `spot_collateral`, `tiered_margin`,
/// `portfolio_var`) pass through the canonical generated proto types
/// unchanged rather than being hand-mirrored: they are deep, map/list-heavy
/// governance blobs, and a second hand-written mirror is exactly what let
/// this crate drift out of sync with the wire format previously. Passing the
/// generated type straight through keeps a single source of truth.
///
/// `MsgUpdateParams` is a **full-replace** write (see `reload_config` in the
/// on-chain risk keeper) — omitting a sub-config here does not leave it
/// unchanged on chain, it clears it. Always seed an update from the current
/// on-chain value (see [`crate::client::RiskClient::get_params`] and
/// [`crate::builder::UpdateParamsBuilder::from_current`]) rather than
/// starting from a fresh/default value.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RiskParams {
    pub config: RiskConfig,
    pub auction_params: Option<proto::AuctionParams>,
    pub spot_risk: Option<proto::SpotRiskConfig>,
    pub spot_collateral: Option<proto::SpotCollateralConfig>,
    pub tiered_margin: Option<proto::TieredMarginConfig>,
    pub portfolio_var: Option<proto::PortfolioVarConfig>,
    pub clmm_collateral: Option<proto::ClmmCollateralConfig>,
    pub oi_cap: Option<proto::OiCapConfig>,
    pub systemic_risk: Option<proto::SystemicRiskConfig>,
}

impl From<proto::Params> for RiskParams {
    fn from(p: proto::Params) -> Self {
        Self {
            config: p.config.unwrap_or_default().into(),
            auction_params: p.auction_params,
            spot_risk: p.spot_risk,
            spot_collateral: p.spot_collateral,
            tiered_margin: p.tiered_margin,
            portfolio_var: p.portfolio_var,
            clmm_collateral: p.clmm_collateral,
            oi_cap: p.oi_cap,
            systemic_risk: p.systemic_risk,
        }
    }
}

impl From<RiskParams> for proto::Params {
    fn from(p: RiskParams) -> Self {
        Self {
            config: Some(p.config.into()),
            auction_params: p.auction_params,
            spot_risk: p.spot_risk,
            spot_collateral: p.spot_collateral,
            tiered_margin: p.tiered_margin,
            portfolio_var: p.portfolio_var,
            clmm_collateral: p.clmm_collateral,
            oi_cap: p.oi_cap,
            systemic_risk: p.systemic_risk,
        }
    }
}

// ====================== STREAM EVENT TYPES ======================

/// OI updated for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OiUpdated {
    pub market_index: u64,
    pub long_oi: String,
    pub short_oi: String,
}

impl From<proto::OiUpdated> for OiUpdated {
    fn from(p: proto::OiUpdated) -> Self {
        Self {
            market_index: p.market_index,
            long_oi: p.long_oi,
            short_oi: p.short_oi,
        }
    }
}

/// Heatmap updated for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HeatmapUpdatedEvent {
    pub market_index: u64,
    pub band_count: u64,
}

impl From<proto::HeatmapUpdatedEvent> for HeatmapUpdatedEvent {
    fn from(p: proto::HeatmapUpdatedEvent) -> Self {
        Self {
            market_index: p.market_index,
            band_count: p.band_count,
        }
    }
}

/// A liquidation auction was opened (block-clock Dutch auction).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuctionOpened {
    pub auction_id: u64,
    pub bucket_id: u64,
    pub market_index: u64,
    pub position_is_long: bool,
    pub size: u64,
    pub start_price: u64,
    pub floor_price: u64,
    pub open_height: u64,
    pub deadline_height: u64,
}

impl From<proto::AuctionOpened> for AuctionOpened {
    fn from(p: proto::AuctionOpened) -> Self {
        Self {
            auction_id: p.auction_id,
            bucket_id: p.bucket_id,
            market_index: p.market_index,
            position_is_long: p.position_is_long,
            size: p.size,
            start_price: p.start_price,
            floor_price: p.floor_price,
            open_height: p.open_height,
            deadline_height: p.deadline_height,
        }
    }
}

/// A liquidation auction was cleared by a winning bid.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuctionCleared {
    pub auction_id: u64,
    pub bucket_id: u64,
    pub market_index: u64,
    pub taker_bucket_id: u64,
    pub clear_price: u64,
    pub recovery: u64,
    pub residual: u64,
    pub settle_height: u64,
    pub reward: u64,
}

impl From<proto::AuctionCleared> for AuctionCleared {
    fn from(p: proto::AuctionCleared) -> Self {
        Self {
            auction_id: p.auction_id,
            bucket_id: p.bucket_id,
            market_index: p.market_index,
            taker_bucket_id: p.taker_bucket_id,
            clear_price: p.clear_price,
            recovery: p.recovery,
            residual: p.residual,
            settle_height: p.settle_height,
            reward: p.reward,
        }
    }
}

/// A liquidation auction expired and was force-settled.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuctionExpired {
    pub auction_id: u64,
    pub bucket_id: u64,
    pub market_index: u64,
    pub recovery: u64,
    pub residual: u64,
    pub settle_height: u64,
}

impl From<proto::AuctionExpired> for AuctionExpired {
    fn from(p: proto::AuctionExpired) -> Self {
        Self {
            auction_id: p.auction_id,
            bucket_id: p.bucket_id,
            market_index: p.market_index,
            recovery: p.recovery,
            residual: p.residual,
            settle_height: p.settle_height,
        }
    }
}

/// A liquidation auction was force-cleared into the insurance-owned backstop
/// bucket at the auction floor, with a delta-hedge swap absorbing/shedding
/// the resulting base exposure.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuctionBackstopped {
    pub auction_id: u64,
    pub bucket_id: u64,
    pub market_index: u64,
    pub taker_bucket_id: u64,
    pub clear_price: u64,
    pub recovery: u64,
    pub residual: u64,
    /// Base absorbed/shed by the delta-hedge swap (pool-native units, u128 string).
    pub hedged_base: String,
    /// Signed quote-leg delta from the hedge (1e8 native, i128 string): positive
    /// when the fund sold base (long takeover), negative when it bought base
    /// (short takeover).
    pub quote_delta: String,
    pub settle_height: u64,
}

impl From<proto::AuctionBackstopped> for AuctionBackstopped {
    fn from(p: proto::AuctionBackstopped) -> Self {
        Self {
            auction_id: p.auction_id,
            bucket_id: p.bucket_id,
            market_index: p.market_index,
            taker_bucket_id: p.taker_bucket_id,
            clear_price: p.clear_price,
            recovery: p.recovery,
            residual: p.residual,
            hedged_base: p.hedged_base,
            quote_delta: p.quote_delta,
            settle_height: p.settle_height,
        }
    }
}

/// Systemic stress index recomputed on a cadence scan (WS-BE).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SystemicStressUpdated {
    pub ssi_bps: u32,
    pub amplification_bps: u32,
    pub concentration_hhi_bps: u32,
    pub oi_contain_bps: u32,
    pub last_updated_ms: u64,
}

impl From<proto::SystemicStressUpdated> for SystemicStressUpdated {
    fn from(p: proto::SystemicStressUpdated) -> Self {
        Self {
            ssi_bps: p.ssi_bps,
            amplification_bps: p.amplification_bps,
            concentration_hhi_bps: p.concentration_hhi_bps,
            oi_contain_bps: p.oi_contain_bps,
            last_updated_ms: p.last_updated_ms,
        }
    }
}

/// SSI crossed the governance warn threshold (WS-BE, informational).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SystemicStressAlert {
    pub ssi_bps: u32,
    pub ssi_warn_bps: u32,
    pub oi_contain_bps: u32,
}

impl From<proto::SystemicStressAlert> for SystemicStressAlert {
    fn from(p: proto::SystemicStressAlert) -> Self {
        Self {
            ssi_bps: p.ssi_bps,
            ssi_warn_bps: p.ssi_warn_bps,
            oi_contain_bps: p.oi_contain_bps,
        }
    }
}

/// Union of risk module streaming events.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RiskEvent {
    OiUpdated(OiUpdated),
    HeatmapUpdated(HeatmapUpdatedEvent),
    OiImbalanceAlert {
        market_index: u64,
        ratio_bps: u32,
        long_oi: String,
        short_oi: String,
    },
    AuctionOpened(AuctionOpened),
    AuctionCleared(AuctionCleared),
    AuctionExpired(AuctionExpired),
    AuctionBackstopped(AuctionBackstopped),
    SystemicStressUpdated(SystemicStressUpdated),
    SystemicStressAlert(SystemicStressAlert),
}

impl RiskEvent {
    /// Converts from the proto oneof wrapper. Returns `None` if the event field is unset.
    pub fn from_proto(p: proto::RiskEvent) -> Option<Self> {
        use proto::risk_event::Event;
        p.event.map(|e| match e {
            Event::OiUpdated(v) => Self::OiUpdated(v.into()),
            Event::HeatmapUpdated(v) => Self::HeatmapUpdated(v.into()),
            Event::OiImbalanceAlert(v) => Self::OiImbalanceAlert {
                market_index: v.market_index,
                ratio_bps: v.ratio_bps,
                long_oi: v.long_oi,
                short_oi: v.short_oi,
            },
            Event::AuctionOpened(v) => Self::AuctionOpened(v.into()),
            Event::AuctionCleared(v) => Self::AuctionCleared(v.into()),
            Event::AuctionExpired(v) => Self::AuctionExpired(v.into()),
            Event::AuctionBackstopped(v) => Self::AuctionBackstopped(v.into()),
            Event::SystemicStressUpdated(v) => Self::SystemicStressUpdated(v.into()),
            Event::SystemicStressAlert(v) => Self::SystemicStressAlert(v.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn risk_config_roundtrip() {
        let c = RiskConfig {
            band_width_bps: 100,
            num_bands_above_below: 10,
            imbalance_threshold_bps: 500,
            imbalance_hysteresis_bps: 50,
            max_scan_limit: 100,
            liquidation_margin_ratio_bps: 500,
            partial_band_shift_enabled: true,
        };
        let p: proto::RiskConfig = c.clone().into();
        let c2: RiskConfig = p.into();
        assert_eq!(c, c2);
    }

    #[test]
    fn risk_params_roundtrip_with_none_subconfigs() {
        let params = RiskParams {
            config: RiskConfig {
                band_width_bps: 100,
                num_bands_above_below: 10,
                imbalance_threshold_bps: 500,
                imbalance_hysteresis_bps: 50,
                max_scan_limit: 100,
                liquidation_margin_ratio_bps: 500,
                partial_band_shift_enabled: true,
            },
            auction_params: None,
            spot_risk: None,
            spot_collateral: None,
            tiered_margin: None,
            portfolio_var: None,
            clmm_collateral: None,
            oi_cap: None,
            systemic_risk: None,
        };
        let p: proto::Params = params.clone().into();
        let back: RiskParams = p.into();
        assert_eq!(params, back);
    }

    /// Confirms the deep governance sub-configs pass through losslessly
    /// (full struct + nested map/list) rather than being dropped/defaulted,
    /// and that a config-only update does not implicitly clear them.
    #[test]
    fn risk_params_roundtrip_with_subconfigs() {
        let mut markets = alloc::collections::BTreeMap::new();
        markets.insert(
            7u64,
            proto::SpotRiskMarket {
                pool_id: 1,
                base_token: 2,
                price_scale: "100000000".into(),
            },
        );
        let params = RiskParams {
            config: RiskConfig {
                band_width_bps: 100,
                num_bands_above_below: 10,
                imbalance_threshold_bps: 500,
                imbalance_hysteresis_bps: 50,
                max_scan_limit: 100,
                liquidation_margin_ratio_bps: 500,
                partial_band_shift_enabled: true,
            },
            auction_params: Some(proto::AuctionParams {
                duration_blocks: 20,
                initial_premium_bps: 500,
                floor_discount_bps: 500,
                decay_bps_per_block: 25,
            }),
            spot_risk: Some(proto::SpotRiskConfig {
                enabled: true,
                band_bps: 200,
                addon_cap_bps: 1000,
                comfortable_multiple_bps: 20000,
                markets,
            }),
            spot_collateral: None,
            tiered_margin: None,
            portfolio_var: None,
            clmm_collateral: Some(proto::ClmmCollateralConfig {
                enabled: true,
                pools: {
                    let mut pools = alloc::collections::BTreeMap::new();
                    pools.insert(
                        9u64,
                        proto::ClmmCollateralPool {
                            base_price_market_index: 7,
                            quote_token_index: 2,
                            collateral_factor_bps: 8_000,
                            quote_swap_pool_id: 0,
                            quote_price_market_index: 0,
                            basis_tolerance_bps: 0,
                            max_basis_haircut_bps: 0,
                        },
                    );
                    pools
                },
            }),
            oi_cap: None,
            systemic_risk: None,
        };
        let p: proto::Params = params.clone().into();
        assert_eq!(p.spot_risk, params.spot_risk);
        assert_eq!(p.clmm_collateral, params.clmm_collateral);
        let back: RiskParams = p.into();
        assert_eq!(params, back);
    }

    #[test]
    fn auction_backstopped_from_proto() {
        let p = proto::AuctionBackstopped {
            auction_id: 1,
            bucket_id: 2,
            market_index: 3,
            taker_bucket_id: 4,
            clear_price: 5,
            recovery: 6,
            residual: 7,
            hedged_base: "1000".into(),
            quote_delta: "-500".into(),
            settle_height: 8,
        };
        let backstopped: AuctionBackstopped = p.clone().into();
        assert_eq!(backstopped.auction_id, p.auction_id);
        assert_eq!(backstopped.hedged_base, "1000");
        assert_eq!(backstopped.quote_delta, "-500");
    }

    #[test]
    fn risk_event_from_proto_auction_backstopped() {
        let proto_event = proto::RiskEvent {
            event: Some(proto::risk_event::Event::AuctionBackstopped(
                proto::AuctionBackstopped {
                    auction_id: 1,
                    bucket_id: 2,
                    market_index: 3,
                    taker_bucket_id: 4,
                    clear_price: 5,
                    recovery: 6,
                    residual: 7,
                    hedged_base: "1000".into(),
                    quote_delta: "-500".into(),
                    settle_height: 8,
                },
            )),
        };
        let event = RiskEvent::from_proto(proto_event);
        assert!(matches!(event, Some(RiskEvent::AuctionBackstopped(_))));
    }

    #[test]
    fn heatmap_band_from_proto() {
        let p = proto::HeatmapBand {
            notional: "1000".into(),
            oi_long: "500".into(),
            oi_short: "500".into(),
            count: 10,
        };
        let b: HeatmapBand = p.into();
        assert_eq!(b.count, 10);
    }

    #[test]
    fn risk_event_from_proto() {
        let proto_event = proto::RiskEvent {
            event: Some(proto::risk_event::Event::OiUpdated(proto::OiUpdated {
                market_index: 1,
                long_oi: "100".into(),
                short_oi: "200".into(),
            })),
        };
        let event = RiskEvent::from_proto(proto_event);
        assert!(matches!(event, Some(RiskEvent::OiUpdated(_))));
    }

    #[test]
    fn risk_event_none_on_empty() {
        let proto_event = proto::RiskEvent { event: None };
        assert!(RiskEvent::from_proto(proto_event).is_none());
    }
}
