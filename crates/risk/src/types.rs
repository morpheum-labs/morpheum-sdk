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
