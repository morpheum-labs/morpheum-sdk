//! Domain types for the outcome feed module.
//!
//! Covers resolution paradigms, feed statuses, prediction market feeds,
//! resolution criteria, and resolved outcomes.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::outcomefeed::v1 as proto;

// ====================== ENUMS ======================

/// Resolution strategy for a prediction market feed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ResolutionParadigm {
    Unspecified,
    AuthorityCertified,
    MarketPrice,
    TimestampedAnnouncement,
    ParametricMeasurement,
    SubjectiveConsensus,
}

impl From<i32> for ResolutionParadigm {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::AuthorityCertified, 2 => Self::MarketPrice,
            3 => Self::TimestampedAnnouncement, 4 => Self::ParametricMeasurement,
            5 => Self::SubjectiveConsensus, _ => Self::Unspecified,
        }
    }
}

impl From<ResolutionParadigm> for i32 {
    fn from(p: ResolutionParadigm) -> Self {
        match p {
            ResolutionParadigm::Unspecified => 0, ResolutionParadigm::AuthorityCertified => 1,
            ResolutionParadigm::MarketPrice => 2, ResolutionParadigm::TimestampedAnnouncement => 3,
            ResolutionParadigm::ParametricMeasurement => 4, ResolutionParadigm::SubjectiveConsensus => 5,
        }
    }
}

/// Lifecycle state of an outcome feed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FeedStatus {
    Unspecified,
    Active,
    Resolving,
    Resolved,
    Disputed,
}

impl From<i32> for FeedStatus {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::Active, 2 => Self::Resolving,
            3 => Self::Resolved, 4 => Self::Disputed,
            _ => Self::Unspecified,
        }
    }
}

impl From<FeedStatus> for i32 {
    fn from(s: FeedStatus) -> Self {
        match s {
            FeedStatus::Unspecified => 0, FeedStatus::Active => 1,
            FeedStatus::Resolving => 2, FeedStatus::Resolved => 3,
            FeedStatus::Disputed => 4,
        }
    }
}

// ====================== DOMAIN TYPES ======================

/// Resolution criteria for a prediction market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MarketResolutionCriteria {
    pub feed_id: String,
    pub resolution_deadline: u64,
    pub dispute_window_sec: u64,
    pub trusted_sources: Vec<String>,
    /// Opaque criteria JSON (prost-encoded `google.protobuf.Struct`).
    pub criteria_json_bytes: Vec<u8>,
}

impl From<proto::MarketResolutionCriteria> for MarketResolutionCriteria {
    fn from(p: proto::MarketResolutionCriteria) -> Self {
        Self {
            feed_id: p.feed_id,
            resolution_deadline: p.resolution_deadline,
            dispute_window_sec: p.dispute_window_sec,
            trusted_sources: p.trusted_sources,
            criteria_json_bytes: p.criteria_json.map_or(Vec::new(), |s| prost::Message::encode_to_vec(&s)),
        }
    }
}

impl From<MarketResolutionCriteria> for proto::MarketResolutionCriteria {
    fn from(c: MarketResolutionCriteria) -> Self {
        Self {
            feed_id: c.feed_id,
            resolution_deadline: c.resolution_deadline,
            dispute_window_sec: c.dispute_window_sec,
            trusted_sources: c.trusted_sources,
            criteria_json: if c.criteria_json_bytes.is_empty() {
                None
            } else {
                prost::Message::decode(c.criteria_json_bytes.as_slice()).ok()
            },
        }
    }
}

/// Registered prediction market feed entry.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PredictionMarketFeed {
    pub feed_id: String,
    pub paradigm: ResolutionParadigm,
    pub criteria: Option<MarketResolutionCriteria>,
    pub status: FeedStatus,
}

impl From<proto::PredictionMarketFeed> for PredictionMarketFeed {
    fn from(p: proto::PredictionMarketFeed) -> Self {
        Self {
            feed_id: p.feed_id,
            paradigm: ResolutionParadigm::from(p.paradigm),
            criteria: p.criteria.map(Into::into),
            status: FeedStatus::from(p.status),
        }
    }
}

/// Final settlement result for a prediction market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResolvedOutcome {
    pub outcome: String,
    pub confidence_bps: u32,
    pub final_ts: u64,
    pub resolution_source: String,
    pub paradigm: ResolutionParadigm,
    pub evidence_hash: String,
}

impl From<proto::ResolvedOutcome> for ResolvedOutcome {
    fn from(p: proto::ResolvedOutcome) -> Self {
        Self {
            outcome: p.outcome,
            confidence_bps: p.confidence_bps,
            final_ts: p.final_ts,
            resolution_source: p.resolution_source,
            paradigm: ResolutionParadigm::from(p.paradigm),
            evidence_hash: p.evidence_hash,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolution_paradigm_roundtrip() {
        for p in [ResolutionParadigm::AuthorityCertified, ResolutionParadigm::MarketPrice,
                  ResolutionParadigm::TimestampedAnnouncement, ResolutionParadigm::ParametricMeasurement,
                  ResolutionParadigm::SubjectiveConsensus] {
            let v: i32 = p.into();
            assert_eq!(p, ResolutionParadigm::from(v));
        }
        assert_eq!(ResolutionParadigm::Unspecified, ResolutionParadigm::from(99));
    }

    #[test]
    fn feed_status_roundtrip() {
        for s in [FeedStatus::Active, FeedStatus::Resolving, FeedStatus::Resolved, FeedStatus::Disputed] {
            let v: i32 = s.into();
            assert_eq!(s, FeedStatus::from(v));
        }
    }

    #[test]
    fn prediction_market_feed_from_proto() {
        let p = proto::PredictionMarketFeed {
            feed_id: "btc-50k-eoy".into(), paradigm: 2,
            criteria: None, status: 1,
        };
        let f: PredictionMarketFeed = p.into();
        assert_eq!(f.feed_id, "btc-50k-eoy");
        assert_eq!(f.paradigm, ResolutionParadigm::MarketPrice);
        assert_eq!(f.status, FeedStatus::Active);
        assert!(f.criteria.is_none());
    }

    #[test]
    fn resolved_outcome_from_proto() {
        let p = proto::ResolvedOutcome {
            outcome: "yes".into(), confidence_bps: 10000, final_ts: 1_700_000_000,
            resolution_source: "oracle".into(), paradigm: 1,
            evidence_hash: "abc123".into(),
        };
        let r: ResolvedOutcome = p.into();
        assert_eq!(r.outcome, "yes");
        assert_eq!(r.confidence_bps, 10000);
        assert_eq!(r.paradigm, ResolutionParadigm::AuthorityCertified);
    }
}
