//! Request wrappers for the prediction market module.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::prediction::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::ResolvedOutcome;

// ====================== TRANSACTION REQUESTS ======================

/// Create a new prediction market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateMarketRequest {
    pub creator: String,
    pub feed_id: String,
    pub outcomes: Vec<String>,
    pub criteria_json: String,
    /// u128 as decimal string.
    pub min_morm_stake: String,
    pub creator_bucket_id: u64,
    pub quote_asset_index: u64,
}

impl CreateMarketRequest {
    pub fn new(
        creator: impl Into<String>, feed_id: impl Into<String>,
        outcomes: Vec<String>, criteria_json: impl Into<String>,
        min_morm_stake: impl Into<String>, creator_bucket_id: u64,
        quote_asset_index: u64,
    ) -> Self {
        Self {
            creator: creator.into(), feed_id: feed_id.into(),
            outcomes, criteria_json: criteria_json.into(),
            min_morm_stake: min_morm_stake.into(), creator_bucket_id,
            quote_asset_index,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgCreateMarket {
            creator: self.creator.clone(), feed_id: self.feed_id.clone(),
            outcomes: self.outcomes.clone(), criteria_json: self.criteria_json.clone(),
            min_morm_stake: self.min_morm_stake.clone(), creator_bucket_id: self.creator_bucket_id,
            quote_asset_index: self.quote_asset_index,
        };
        ProtoAny { type_url: "/prediction.v1.MsgCreateMarket".into(), value: msg.encode_to_vec() }
    }
}

/// Resolve a prediction market with the winning outcome.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResolveMarketRequest {
    pub signer: String,
    pub outcome: ResolvedOutcome,
}

impl ResolveMarketRequest {
    pub fn new(signer: impl Into<String>, outcome: ResolvedOutcome) -> Self {
        Self { signer: signer.into(), outcome }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgResolveMarket {
            signer: self.signer.clone(),
            outcome: Some(self.outcome.clone().into()),
        };
        ProtoAny { type_url: "/prediction.v1.MsgResolveMarket".into(), value: msg.encode_to_vec() }
    }
}

/// Dispute a resolved market (bonded challenge within dispute window).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisputeMarketRequest {
    pub challenger: String,
    pub feed_id: String,
    /// u128 as decimal string.
    pub bond_amount: String,
    pub evidence_data: Vec<u8>,
    pub reason_code: u32,
    pub challenger_bucket_id: u64,
}

impl DisputeMarketRequest {
    pub fn new(
        challenger: impl Into<String>, feed_id: impl Into<String>,
        bond_amount: impl Into<String>, evidence_data: Vec<u8>,
        reason_code: u32, challenger_bucket_id: u64,
    ) -> Self {
        Self {
            challenger: challenger.into(), feed_id: feed_id.into(),
            bond_amount: bond_amount.into(), evidence_data,
            reason_code, challenger_bucket_id,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgDisputeMarket {
            challenger: self.challenger.clone(), feed_id: self.feed_id.clone(),
            bond_amount: self.bond_amount.clone(), evidence_data: self.evidence_data.clone(),
            reason_code: self.reason_code, challenger_bucket_id: self.challenger_bucket_id,
        };
        ProtoAny { type_url: "/prediction.v1.MsgDisputeMarket".into(), value: msg.encode_to_vec() }
    }
}

/// Open a light challenge (no bond, reputation-gated).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OpenLightChallengeRequest {
    pub challenger: String,
    pub feed_id: String,
    pub proposed_outcome: ResolvedOutcome,
}

impl OpenLightChallengeRequest {
    pub fn new(
        challenger: impl Into<String>, feed_id: impl Into<String>,
        proposed_outcome: ResolvedOutcome,
    ) -> Self {
        Self { challenger: challenger.into(), feed_id: feed_id.into(), proposed_outcome }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgOpenLightChallenge {
            challenger: self.challenger.clone(), feed_id: self.feed_id.clone(),
            proposed_outcome: Some(self.proposed_outcome.clone().into()),
        };
        ProtoAny { type_url: "/prediction.v1.MsgOpenLightChallenge".into(), value: msg.encode_to_vec() }
    }
}

/// Vote on an active light challenge.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LightChallengeVoteRequest {
    pub voter: String,
    pub feed_id: String,
    pub agree: bool,
    pub weight: u64,
}

impl LightChallengeVoteRequest {
    pub fn new(voter: impl Into<String>, feed_id: impl Into<String>, agree: bool, weight: u64) -> Self {
        Self { voter: voter.into(), feed_id: feed_id.into(), agree, weight }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgLightChallengeVote {
            voter: self.voter.clone(), feed_id: self.feed_id.clone(),
            agree: self.agree, weight: self.weight,
        };
        ProtoAny { type_url: "/prediction.v1.MsgLightChallengeVote".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query a single prediction market by feed ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPredictionMarketRequest {
    pub feed_id: String,
}

impl QueryPredictionMarketRequest {
    pub fn new(feed_id: impl Into<String>) -> Self { Self { feed_id: feed_id.into() } }
}

impl From<QueryPredictionMarketRequest> for proto::QueryPredictionMarketRequest {
    fn from(r: QueryPredictionMarketRequest) -> Self { Self { feed_id: r.feed_id } }
}

/// List prediction markets with pagination and optional phase filter.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryPredictionMarketsRequest {
    pub limit: i32,
    pub offset: i32,
    /// Phase filter string: "active", "resolved", "disputed", "settled", "voided", "cancelled", or empty for all.
    pub phase_filter: String,
}

impl QueryPredictionMarketsRequest {
    pub fn new(limit: i32, offset: i32) -> Self {
        Self { limit, offset, phase_filter: String::new() }
    }

    pub fn phase_filter(mut self, phase: impl Into<String>) -> Self { self.phase_filter = phase.into(); self }
}

impl From<QueryPredictionMarketsRequest> for proto::QueryPredictionMarketsRequest {
    fn from(r: QueryPredictionMarketsRequest) -> Self {
        Self { limit: r.limit, offset: r.offset, phase_filter: r.phase_filter }
    }
}

/// Query implied probability (1e9 scale) for a feed.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryImpliedProbabilityRequest {
    pub feed_id: String,
}

impl QueryImpliedProbabilityRequest {
    pub fn new(feed_id: impl Into<String>) -> Self { Self { feed_id: feed_id.into() } }
}

impl From<QueryImpliedProbabilityRequest> for proto::QueryPredictionImpliedProbabilityRequest {
    fn from(r: QueryImpliedProbabilityRequest) -> Self { Self { feed_id: r.feed_id } }
}

/// Query cumulative fee statistics for a prediction market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMarketFeeStatsRequest {
    pub feed_id: String,
}

impl QueryMarketFeeStatsRequest {
    pub fn new(feed_id: impl Into<String>) -> Self { Self { feed_id: feed_id.into() } }
}

impl From<QueryMarketFeeStatsRequest> for proto::QueryMarketFeeStatsRequest {
    fn from(r: QueryMarketFeeStatsRequest) -> Self { Self { feed_id: r.feed_id } }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn create_market_to_any() {
        let any = CreateMarketRequest::new(
            "morph1xyz", "btc-50k", vec!["yes".into(), "no".into()],
            "{}", "100000", 1, 1,
        ).to_any();
        assert_eq!(any.type_url, "/prediction.v1.MsgCreateMarket");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn resolve_market_to_any() {
        let outcome = ResolvedOutcome { feed_id: "f1".into(), winning_outcome_id: 0, confidence: 1_000_000_000 };
        let any = ResolveMarketRequest::new("morph1gov", outcome).to_any();
        assert_eq!(any.type_url, "/prediction.v1.MsgResolveMarket");
    }

    #[test]
    fn dispute_market_to_any() {
        let any = DisputeMarketRequest::new(
            "morph1ch", "f1", "50000", Vec::new(), 1, 2,
        ).to_any();
        assert_eq!(any.type_url, "/prediction.v1.MsgDisputeMarket");
    }

    #[test]
    fn open_light_challenge_to_any() {
        let outcome = ResolvedOutcome { feed_id: "f1".into(), winning_outcome_id: 1, confidence: 800_000_000 };
        let any = OpenLightChallengeRequest::new("morph1ch", "f1", outcome).to_any();
        assert_eq!(any.type_url, "/prediction.v1.MsgOpenLightChallenge");
    }

    #[test]
    fn light_challenge_vote_to_any() {
        let any = LightChallengeVoteRequest::new("morph1v", "f1", true, 1).to_any();
        assert_eq!(any.type_url, "/prediction.v1.MsgLightChallengeVote");
    }

    #[test]
    fn query_conversions() {
        let p: proto::QueryPredictionMarketRequest = QueryPredictionMarketRequest::new("f1").into();
        assert_eq!(p.feed_id, "f1");

        let p: proto::QueryPredictionMarketsRequest = QueryPredictionMarketsRequest::new(50, 0)
            .phase_filter("active").into();
        assert_eq!(p.phase_filter, "active");

        let p: proto::QueryPredictionImpliedProbabilityRequest = QueryImpliedProbabilityRequest::new("f1").into();
        assert_eq!(p.feed_id, "f1");
    }
}
