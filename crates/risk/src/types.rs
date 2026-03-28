//! Domain types for the risk module.
//!
//! Covers heatmaps, liquidation plans, margin snapshots, OI analytics,
//! risk configuration, and streaming risk events.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::risk::v1 as proto;

// ====================== ENUMS ======================

/// Resolution path for liquidation shortfall.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ShortfallIntendedPath {
    #[default]
    Unspecified,
    DirectToInsurance,
    RunAuction,
}

impl From<i32> for ShortfallIntendedPath {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::DirectToInsurance, 2 => Self::RunAuction,
            _ => Self::Unspecified,
        }
    }
}

impl From<ShortfallIntendedPath> for i32 {
    fn from(p: ShortfallIntendedPath) -> Self {
        match p {
            ShortfallIntendedPath::Unspecified => 0,
            ShortfallIntendedPath::DirectToInsurance => 1,
            ShortfallIntendedPath::RunAuction => 2,
        }
    }
}

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
        Self { notional: p.notional, oi_long: p.oi_long, oi_short: p.oi_short, count: p.count }
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
            market_index: p.market_index, mark_price: p.mark_price,
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
        Self { band_index: p.band_index, notional: p.notional }
    }
}

/// Heatmap scan + anti-cascade sequencing for a bucket.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiquidationPlan {
    pub bucket_id: u64,
    pub market_index: u64,
    pub bands: Vec<LiquidationPlanBand>,
    pub total_at_risk: String,
    pub cascade_remaining: u32,
}

impl From<proto::LiquidationPlan> for LiquidationPlan {
    fn from(p: proto::LiquidationPlan) -> Self {
        Self {
            bucket_id: p.bucket_id, market_index: p.market_index,
            bands: p.bands.into_iter().map(Into::into).collect(),
            total_at_risk: p.total_at_risk, cascade_remaining: p.cascade_remaining,
        }
    }
}

/// Pre-trade margin simulation result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PreTradeMarginResult {
    pub projected_margin_impact: String,
    pub health_ratio_bps: u32,
}

impl From<proto::PreTradeMarginResult> for PreTradeMarginResult {
    fn from(p: proto::PreTradeMarginResult) -> Self {
        Self { projected_margin_impact: p.projected_margin_impact, health_ratio_bps: p.health_ratio_bps }
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
            bucket_id: p.bucket_id, market_index: p.market_index,
            total_notional: p.total_notional,
            at_risk_bands: p.at_risk_bands.into_iter().map(Into::into).collect(),
        }
    }
}

/// Governance-tunable Dutch auction parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AuctionParams {
    pub auction_duration_ms: u32,
    pub initial_premium_bps: u32,
    pub max_discount_bps: u32,
    pub auction_rate_limit: u32,
    pub min_bid_increment_bps: u32,
    pub partial_fill_allowed: bool,
    pub insurance_escalation_bps: u32,
}

impl From<proto::AuctionParams> for AuctionParams {
    fn from(p: proto::AuctionParams) -> Self {
        Self {
            auction_duration_ms: p.auction_duration_ms, initial_premium_bps: p.initial_premium_bps,
            max_discount_bps: p.max_discount_bps, auction_rate_limit: p.auction_rate_limit,
            min_bid_increment_bps: p.min_bid_increment_bps, partial_fill_allowed: p.partial_fill_allowed,
            insurance_escalation_bps: p.insurance_escalation_bps,
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
    pub cascade_max_per_market_per_epoch: u32,
    pub max_scan_limit: u32,
    pub liquidation_margin_ratio_bps: u32,
    pub prediction_margin_ratio_bps: u32,
    pub price_move_threshold_bps: u32,
    pub partial_band_shift_enabled: bool,
    pub var_confidence_bps: u32,
    pub var_horizon_hours: u32,
    pub enable_vrf_fairness: bool,
    pub enable_proactive_liquidation_events: bool,
    pub enable_pre_trade_simulation: bool,
    pub enable_spot_risk_integration: bool,
    pub contagion_threshold_sat: u64,
}

impl From<proto::RiskConfig> for RiskConfig {
    fn from(p: proto::RiskConfig) -> Self {
        Self {
            band_width_bps: p.band_width_bps, num_bands_above_below: p.num_bands_above_below,
            imbalance_threshold_bps: p.imbalance_threshold_bps, imbalance_hysteresis_bps: p.imbalance_hysteresis_bps,
            cascade_max_per_market_per_epoch: p.cascade_max_per_market_per_epoch, max_scan_limit: p.max_scan_limit,
            liquidation_margin_ratio_bps: p.liquidation_margin_ratio_bps, prediction_margin_ratio_bps: p.prediction_margin_ratio_bps,
            price_move_threshold_bps: p.price_move_threshold_bps, partial_band_shift_enabled: p.partial_band_shift_enabled,
            var_confidence_bps: p.var_confidence_bps, var_horizon_hours: p.var_horizon_hours,
            enable_vrf_fairness: p.enable_vrf_fairness, enable_proactive_liquidation_events: p.enable_proactive_liquidation_events,
            enable_pre_trade_simulation: p.enable_pre_trade_simulation, enable_spot_risk_integration: p.enable_spot_risk_integration,
            contagion_threshold_sat: p.contagion_threshold_sat,
        }
    }
}

impl From<RiskConfig> for proto::RiskConfig {
    fn from(c: RiskConfig) -> Self {
        Self {
            band_width_bps: c.band_width_bps, num_bands_above_below: c.num_bands_above_below,
            imbalance_threshold_bps: c.imbalance_threshold_bps, imbalance_hysteresis_bps: c.imbalance_hysteresis_bps,
            cascade_max_per_market_per_epoch: c.cascade_max_per_market_per_epoch, max_scan_limit: c.max_scan_limit,
            liquidation_margin_ratio_bps: c.liquidation_margin_ratio_bps, prediction_margin_ratio_bps: c.prediction_margin_ratio_bps,
            price_move_threshold_bps: c.price_move_threshold_bps, partial_band_shift_enabled: c.partial_band_shift_enabled,
            var_confidence_bps: c.var_confidence_bps, var_horizon_hours: c.var_horizon_hours,
            enable_vrf_fairness: c.enable_vrf_fairness, enable_proactive_liquidation_events: c.enable_proactive_liquidation_events,
            enable_pre_trade_simulation: c.enable_pre_trade_simulation, enable_spot_risk_integration: c.enable_spot_risk_integration,
            contagion_threshold_sat: c.contagion_threshold_sat,
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
        Self { market_index: p.market_index, long_oi: p.long_oi, short_oi: p.short_oi }
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
        Self { market_index: p.market_index, band_count: p.band_count }
    }
}

/// Liquidation triggered for a bucket.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiquidationTriggered {
    pub bucket_id: u64,
    pub market_index: u64,
    pub candidate_bands: Vec<LiquidationPlanBand>,
}

impl From<proto::LiquidationTriggered> for LiquidationTriggered {
    fn from(p: proto::LiquidationTriggered) -> Self {
        Self {
            bucket_id: p.bucket_id, market_index: p.market_index,
            candidate_bands: p.candidate_bands.into_iter().map(Into::into).collect(),
        }
    }
}

/// Liquidation shortfall ready for resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LiquidationShortfallReady {
    pub liquidation_id: u64,
    pub bucket_id: u64,
    pub market_index: u64,
    pub shortfall_sat: u64,
    pub intended_path: ShortfallIntendedPath,
    pub block_height: u64,
    pub expires_at_height: u64,
    pub auction_params: Option<AuctionParams>,
}

impl From<proto::LiquidationShortfallReady> for LiquidationShortfallReady {
    fn from(p: proto::LiquidationShortfallReady) -> Self {
        Self {
            liquidation_id: p.liquidation_id, bucket_id: p.bucket_id, market_index: p.market_index,
            shortfall_sat: p.shortfall_sat, intended_path: ShortfallIntendedPath::from(p.intended_path),
            block_height: p.block_height, expires_at_height: p.expires_at_height,
            auction_params: p.auction_params.map(Into::into),
        }
    }
}

/// Insurance payout requested from the insurance fund.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InsurancePayoutRequested {
    pub request_id: u64,
    pub amount: u64,
    pub bucket_id: u64,
    pub liquidation_id: u64,
    pub block_height: u64,
}

impl From<proto::InsurancePayoutRequested> for InsurancePayoutRequested {
    fn from(p: proto::InsurancePayoutRequested) -> Self {
        Self {
            request_id: p.request_id, amount: p.amount, bucket_id: p.bucket_id,
            liquidation_id: p.liquidation_id, block_height: p.block_height,
        }
    }
}

/// Contagion detected between buckets.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContagionDetected {
    pub source_bucket_id: u64,
    pub affected_bucket_id: u64,
    pub amount_sat: u64,
}

impl From<proto::ContagionDetected> for ContagionDetected {
    fn from(p: proto::ContagionDetected) -> Self {
        Self { source_bucket_id: p.source_bucket_id, affected_bucket_id: p.affected_bucket_id, amount_sat: p.amount_sat }
    }
}

/// Union of risk module streaming events.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RiskEvent {
    OiUpdated(OiUpdated),
    HeatmapUpdated(HeatmapUpdatedEvent),
    LiquidationTriggered(LiquidationTriggered),
    OiImbalanceAlert { market_index: u64, ratio_bps: u32, long_oi: String, short_oi: String },
    LiquidationCapped { market_index: u64, capped_count: u32 },
    LiquidationShortfallReady(LiquidationShortfallReady),
    InsurancePayoutRequested(InsurancePayoutRequested),
    ContagionDetected(ContagionDetected),
}

impl RiskEvent {
    /// Converts from the proto oneof wrapper. Returns `None` if the event field is unset.
    pub fn from_proto(p: proto::RiskEvent) -> Option<Self> {
        use proto::risk_event::Event;
        p.event.map(|e| match e {
            Event::OiUpdated(v) => Self::OiUpdated(v.into()),
            Event::HeatmapUpdated(v) => Self::HeatmapUpdated(v.into()),
            Event::LiquidationTriggered(v) => Self::LiquidationTriggered(v.into()),
            Event::OiImbalanceAlert(v) => Self::OiImbalanceAlert {
                market_index: v.market_index, ratio_bps: v.ratio_bps,
                long_oi: v.long_oi, short_oi: v.short_oi,
            },
            Event::LiquidationCapped(v) => Self::LiquidationCapped {
                market_index: v.market_index, capped_count: v.capped_count,
            },
            Event::LiquidationShortfallReady(v) => Self::LiquidationShortfallReady(v.into()),
            Event::InsurancePayoutRequested(v) => Self::InsurancePayoutRequested(v.into()),
            Event::ContagionDetected(v) => Self::ContagionDetected(v.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shortfall_path_roundtrip() {
        for p in [ShortfallIntendedPath::DirectToInsurance, ShortfallIntendedPath::RunAuction] {
            let v: i32 = p.into();
            assert_eq!(p, ShortfallIntendedPath::from(v));
        }
        assert_eq!(ShortfallIntendedPath::Unspecified, ShortfallIntendedPath::from(99));
    }

    #[test]
    fn risk_config_roundtrip() {
        let c = RiskConfig {
            band_width_bps: 100, num_bands_above_below: 10,
            imbalance_threshold_bps: 500, imbalance_hysteresis_bps: 50,
            cascade_max_per_market_per_epoch: 3, max_scan_limit: 100,
            liquidation_margin_ratio_bps: 500, prediction_margin_ratio_bps: 1000,
            price_move_threshold_bps: 200, partial_band_shift_enabled: true,
            var_confidence_bps: 9900, var_horizon_hours: 24,
            enable_vrf_fairness: true, enable_proactive_liquidation_events: true,
            enable_pre_trade_simulation: true, enable_spot_risk_integration: false,
            contagion_threshold_sat: 1_000_000,
        };
        let p: proto::RiskConfig = c.clone().into();
        let c2: RiskConfig = p.into();
        assert_eq!(c, c2);
    }

    #[test]
    fn heatmap_band_from_proto() {
        let p = proto::HeatmapBand { notional: "1000".into(), oi_long: "500".into(), oi_short: "500".into(), count: 10 };
        let b: HeatmapBand = p.into();
        assert_eq!(b.count, 10);
    }

    #[test]
    fn risk_event_from_proto() {
        let proto_event = proto::RiskEvent {
            event: Some(proto::risk_event::Event::OiUpdated(proto::OiUpdated {
                market_index: 1, long_oi: "100".into(), short_oi: "200".into(),
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
