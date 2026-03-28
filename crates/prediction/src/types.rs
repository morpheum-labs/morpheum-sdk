//! Domain types for the prediction market module.
//!
//! Covers prediction phases, dispute configuration, resolved outcomes,
//! prediction markets, and streaming event payloads.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::prediction::v1 as proto;

// ====================== ENUMS ======================

/// Prediction market resolution lifecycle phase.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PredictionPhase {
    Unspecified,
    Active,
    Resolved,
    Disputed,
    Settled,
    Voided,
    Cancelled,
}

impl From<i32> for PredictionPhase {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Active,   2 => Self::Resolved,  3 => Self::Disputed,
            4 => Self::Settled,  5 => Self::Voided,    6 => Self::Cancelled,
            _ => Self::Unspecified,
        }
    }
}

impl From<PredictionPhase> for i32 {
    fn from(p: PredictionPhase) -> Self {
        match p {
            PredictionPhase::Unspecified => 0, PredictionPhase::Active => 1,
            PredictionPhase::Resolved => 2,   PredictionPhase::Disputed => 3,
            PredictionPhase::Settled => 4,    PredictionPhase::Voided => 5,
            PredictionPhase::Cancelled => 6,
        }
    }
}

// ====================== DOMAIN TYPES ======================

/// Per-feed dispute parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisputeConfig {
    pub dispute_window_blocks: u64,
    pub dispute_bond_pct: u32,
    pub challenger_bonus_pct: u32,
}

impl From<proto::DisputeConfig> for DisputeConfig {
    fn from(p: proto::DisputeConfig) -> Self {
        Self {
            dispute_window_blocks: p.dispute_window_blocks,
            dispute_bond_pct: p.dispute_bond_pct,
            challenger_bonus_pct: p.challenger_bonus_pct,
        }
    }
}

impl From<DisputeConfig> for proto::DisputeConfig {
    fn from(d: DisputeConfig) -> Self {
        Self {
            dispute_window_blocks: d.dispute_window_blocks,
            dispute_bond_pct: d.dispute_bond_pct,
            challenger_bonus_pct: d.challenger_bonus_pct,
        }
    }
}

/// Oracle resolution output for a prediction market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResolvedOutcome {
    pub feed_id: String,
    pub winning_outcome_id: u32,
    /// Fixed-point confidence: 0..1_000_000_000 (1e9 scale).
    pub confidence: u32,
}

impl From<proto::ResolvedOutcome> for ResolvedOutcome {
    fn from(p: proto::ResolvedOutcome) -> Self {
        Self { feed_id: p.feed_id, winning_outcome_id: p.winning_outcome_id, confidence: p.confidence }
    }
}

impl From<ResolvedOutcome> for proto::ResolvedOutcome {
    fn from(r: ResolvedOutcome) -> Self {
        Self { feed_id: r.feed_id, winning_outcome_id: r.winning_outcome_id, confidence: r.confidence }
    }
}

/// Prediction market registry entry (read API).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PredictionMarket {
    pub feed_id: String,
    pub outcomes: Vec<String>,
    pub creator: String,
    pub phase: PredictionPhase,
    /// u128 as decimal string (locked MORM stake).
    pub locked_stake: String,
    /// u128 as decimal string (quote asset satoshi).
    pub pot: String,
    pub dispute_deadline: u64,
    pub resolved_outcome: Option<u32>,
    pub dispute_config: Option<DisputeConfig>,
    pub accumulated_fees: u64,
    pub current_confidence: Option<u32>,
    pub spread_bps: u32,
    pub depth: u64,
    pub daily_volume: u64,
    pub quote_asset_index: u64,
}

impl From<proto::PredictionMarket> for PredictionMarket {
    fn from(p: proto::PredictionMarket) -> Self {
        Self {
            feed_id: p.feed_id,
            outcomes: p.outcomes,
            creator: p.creator,
            phase: PredictionPhase::from(p.phase),
            locked_stake: p.locked_stake,
            pot: p.pot,
            dispute_deadline: p.dispute_deadline,
            resolved_outcome: p.resolved_outcome,
            dispute_config: p.dispute_config.map(Into::into),
            accumulated_fees: p.accumulated_fees,
            current_confidence: p.current_confidence,
            spread_bps: p.spread_bps,
            depth: p.depth,
            daily_volume: p.daily_volume,
            quote_asset_index: p.quote_asset_index,
        }
    }
}

// ====================== STREAM EVENT TYPES ======================

/// Market created event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarketCreatedEvent {
    pub feed_id: String,
    pub creator: String,
    pub outcomes: Vec<String>,
}

impl From<proto::MarketCreatedEvent> for MarketCreatedEvent {
    fn from(p: proto::MarketCreatedEvent) -> Self {
        Self { feed_id: p.feed_id, creator: p.creator, outcomes: p.outcomes }
    }
}

/// Market disputed event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarketDisputedEvent {
    pub feed_id: String,
    pub challenger: String,
    /// u128 as decimal string.
    pub bond: String,
}

impl From<proto::MarketDisputedEvent> for MarketDisputedEvent {
    fn from(p: proto::MarketDisputedEvent) -> Self {
        Self { feed_id: p.feed_id, challenger: p.challenger, bond: p.bond }
    }
}

/// Dispute accepted — new outcome replaces the original.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisputeAcceptedEvent {
    pub feed_id: String,
    pub new_outcome: Option<ResolvedOutcome>,
}

impl From<proto::DisputeAcceptedEvent> for DisputeAcceptedEvent {
    fn from(p: proto::DisputeAcceptedEvent) -> Self {
        Self { feed_id: p.feed_id, new_outcome: p.new_outcome.map(Into::into) }
    }
}

/// Dispute rejected — original resolution stands.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisputeRejectedEvent {
    pub feed_id: String,
}

impl From<proto::DisputeRejectedEvent> for DisputeRejectedEvent {
    fn from(p: proto::DisputeRejectedEvent) -> Self { Self { feed_id: p.feed_id } }
}

/// Market voided due to dispute.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisputeVoidedEvent {
    pub feed_id: String,
}

impl From<proto::DisputeVoidedEvent> for DisputeVoidedEvent {
    fn from(p: proto::DisputeVoidedEvent) -> Self { Self { feed_id: p.feed_id } }
}

/// Light challenge opened (no bond, reputation-gated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LightChallengeOpenedEvent {
    pub feed_id: String,
    pub challenger: String,
    pub proposed_outcome: Option<ResolvedOutcome>,
}

impl From<proto::LightChallengeOpenedEvent> for LightChallengeOpenedEvent {
    fn from(p: proto::LightChallengeOpenedEvent) -> Self {
        Self { feed_id: p.feed_id, challenger: p.challenger, proposed_outcome: p.proposed_outcome.map(Into::into) }
    }
}

/// Vote cast on a light challenge.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LightChallengeVoteEvent {
    pub feed_id: String,
    pub voter: String,
    pub agree: bool,
}

impl From<proto::LightChallengeVoteEvent> for LightChallengeVoteEvent {
    fn from(p: proto::LightChallengeVoteEvent) -> Self {
        Self { feed_id: p.feed_id, voter: p.voter, agree: p.agree }
    }
}

/// Light challenge resolved.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LightChallengeResolvedEvent {
    pub feed_id: String,
    pub outcome: Option<ResolvedOutcome>,
    pub rep_delta: i32,
}

impl From<proto::LightChallengeResolvedEvent> for LightChallengeResolvedEvent {
    fn from(p: proto::LightChallengeResolvedEvent) -> Self {
        Self { feed_id: p.feed_id, outcome: p.outcome.map(Into::into), rep_delta: p.rep_delta }
    }
}

/// Light challenge escalated to bonded dispute.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LightChallengeEscalatedEvent {
    pub feed_id: String,
    pub challenger: String,
    /// u128 as decimal string.
    pub bond: String,
}

impl From<proto::LightChallengeEscalatedEvent> for LightChallengeEscalatedEvent {
    fn from(p: proto::LightChallengeEscalatedEvent) -> Self {
        Self { feed_id: p.feed_id, challenger: p.challenger, bond: p.bond }
    }
}

/// Fee applied event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FeeAppliedEvent {
    pub feed_id: String,
    pub amount: u64,
    pub reason: String,
    pub paradigm: String,
    pub dry_run: bool,
}

impl From<proto::FeeAppliedEvent> for FeeAppliedEvent {
    fn from(p: proto::FeeAppliedEvent) -> Self {
        Self { feed_id: p.feed_id, amount: p.amount, reason: p.reason, paradigm: p.paradigm, dry_run: p.dry_run }
    }
}

/// Union of prediction market lifecycle events.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PredictionMarketEvent {
    MarketCreated(MarketCreatedEvent),
    MarketDisputed(MarketDisputedEvent),
    DisputeAccepted(DisputeAcceptedEvent),
    DisputeRejected(DisputeRejectedEvent),
    DisputeVoided(DisputeVoidedEvent),
    LightChallengeOpened(LightChallengeOpenedEvent),
    LightChallengeVote(LightChallengeVoteEvent),
    LightChallengeResolved(LightChallengeResolvedEvent),
    LightChallengeEscalated(LightChallengeEscalatedEvent),
    FeeApplied(FeeAppliedEvent),
}

impl PredictionMarketEvent {
    /// Converts from the proto oneof wrapper. Returns `None` if the event field is unset.
    pub fn from_proto(p: proto::PredictionMarketEvent) -> Option<Self> {
        use proto::prediction_market_event::Event;
        p.event.map(|e| match e {
            Event::MarketCreated(v) => Self::MarketCreated(v.into()),
            Event::MarketDisputed(v) => Self::MarketDisputed(v.into()),
            Event::DisputeAccepted(v) => Self::DisputeAccepted(v.into()),
            Event::DisputeRejected(v) => Self::DisputeRejected(v.into()),
            Event::DisputeVoided(v) => Self::DisputeVoided(v.into()),
            Event::LightChallengeOpened(v) => Self::LightChallengeOpened(v.into()),
            Event::LightChallengeVote(v) => Self::LightChallengeVote(v.into()),
            Event::LightChallengeResolved(v) => Self::LightChallengeResolved(v.into()),
            Event::LightChallengeEscalated(v) => Self::LightChallengeEscalated(v.into()),
            Event::FeeApplied(v) => Self::FeeApplied(v.into()),
        })
    }
}

/// Implied probability update from CLOB trade.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PredictionPriceUpdate {
    pub feed_id: String,
    /// Fixed-point: 0..1_000_000_000 (1e9 scale).
    pub implied_prob: u32,
    /// u128 as decimal string.
    pub raw_price: String,
    pub last_update_id: u64,
}

impl From<proto::PredictionPriceUpdate> for PredictionPriceUpdate {
    fn from(p: proto::PredictionPriceUpdate) -> Self {
        Self {
            feed_id: p.feed_id, implied_prob: p.implied_prob,
            raw_price: p.raw_price, last_update_id: p.last_update_id,
        }
    }
}

/// OHLC kline as implied probability for UI charts.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PredictionKlineUpdate {
    pub feed_id: String,
    pub period: u32,
    pub open_at_logical: u64,
    pub open_prob: u32,
    pub high_prob: u32,
    pub low_prob: u32,
    pub close_prob: u32,
    /// u128 as decimal string.
    pub volume_base: String,
}

impl From<proto::PredictionKlineUpdate> for PredictionKlineUpdate {
    fn from(p: proto::PredictionKlineUpdate) -> Self {
        Self {
            feed_id: p.feed_id, period: p.period, open_at_logical: p.open_at_logical,
            open_prob: p.open_prob, high_prob: p.high_prob, low_prob: p.low_prob,
            close_prob: p.close_prob, volume_base: p.volume_base,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn prediction_phase_roundtrip() {
        for p in [PredictionPhase::Active, PredictionPhase::Resolved, PredictionPhase::Disputed,
                  PredictionPhase::Settled, PredictionPhase::Voided, PredictionPhase::Cancelled] {
            let v: i32 = p.into();
            assert_eq!(p, PredictionPhase::from(v));
        }
        assert_eq!(PredictionPhase::Unspecified, PredictionPhase::from(99));
    }

    #[test]
    fn dispute_config_roundtrip() {
        let d = DisputeConfig { dispute_window_blocks: 100, dispute_bond_pct: 10, challenger_bonus_pct: 5 };
        let p: proto::DisputeConfig = d.clone().into();
        let d2: DisputeConfig = p.into();
        assert_eq!(d, d2);
    }

    #[test]
    fn resolved_outcome_roundtrip() {
        let r = ResolvedOutcome { feed_id: "f1".into(), winning_outcome_id: 0, confidence: 1_000_000_000 };
        let p: proto::ResolvedOutcome = r.clone().into();
        let r2: ResolvedOutcome = p.into();
        assert_eq!(r, r2);
    }

    #[test]
    fn prediction_market_from_proto() {
        let p = proto::PredictionMarket {
            feed_id: "btc-50k".into(), outcomes: vec!["yes".into(), "no".into()],
            creator: "morph1xyz".into(), phase: 1,
            locked_stake: "100000".into(), pot: "500000".into(),
            dispute_deadline: 1000, resolved_outcome: None,
            dispute_config: Some(proto::DisputeConfig { dispute_window_blocks: 50, dispute_bond_pct: 10, challenger_bonus_pct: 5 }),
            accumulated_fees: 100, current_confidence: Some(750_000_000),
            spread_bps: 50, depth: 10000, daily_volume: 5000, quote_asset_index: 1,
        };
        let m: PredictionMarket = p.into();
        assert_eq!(m.feed_id, "btc-50k");
        assert_eq!(m.phase, PredictionPhase::Active);
        assert_eq!(m.outcomes.len(), 2);
        assert!(m.dispute_config.is_some());
        assert_eq!(m.current_confidence, Some(750_000_000));
    }

    #[test]
    fn market_event_conversion() {
        let proto_event = proto::PredictionMarketEvent {
            event: Some(proto::prediction_market_event::Event::MarketCreated(
                proto::MarketCreatedEvent {
                    feed_id: "f1".into(), creator: "morph1".into(),
                    outcomes: vec!["yes".into(), "no".into()],
                },
            )),
        };
        let event = PredictionMarketEvent::from_proto(proto_event);
        assert!(matches!(event, Some(PredictionMarketEvent::MarketCreated(_))));
    }

    #[test]
    fn price_update_from_proto() {
        let p = proto::PredictionPriceUpdate {
            feed_id: "f1".into(), implied_prob: 500_000_000,
            raw_price: "50000".into(), last_update_id: 42,
        };
        let u: PredictionPriceUpdate = p.into();
        assert_eq!(u.implied_prob, 500_000_000);
    }
}
