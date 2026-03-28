//! Fluent builders for the prediction market module.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    CreateMarketRequest, DisputeMarketRequest, LightChallengeVoteRequest,
    OpenLightChallengeRequest, ResolveMarketRequest,
};
use crate::types::ResolvedOutcome;

// ====================== CREATE MARKET ======================

#[derive(Default)]
pub struct CreateMarketBuilder {
    creator: Option<String>,
    feed_id: Option<String>,
    outcomes: Vec<String>,
    criteria_json: Option<String>,
    min_morm_stake: Option<String>,
    creator_bucket_id: Option<u64>,
    quote_asset_index: u64,
}

impl CreateMarketBuilder {
    pub fn new() -> Self { Self { quote_asset_index: 1, ..Self::default() } }

    pub fn creator(mut self, v: impl Into<String>) -> Self { self.creator = Some(v.into()); self }
    pub fn feed_id(mut self, v: impl Into<String>) -> Self { self.feed_id = Some(v.into()); self }
    pub fn outcomes(mut self, v: Vec<String>) -> Self { self.outcomes = v; self }
    pub fn criteria_json(mut self, v: impl Into<String>) -> Self { self.criteria_json = Some(v.into()); self }
    pub fn min_morm_stake(mut self, v: impl Into<String>) -> Self { self.min_morm_stake = Some(v.into()); self }
    pub fn creator_bucket_id(mut self, v: u64) -> Self { self.creator_bucket_id = Some(v); self }
    pub fn quote_asset_index(mut self, v: u64) -> Self { self.quote_asset_index = v; self }

    pub fn build(self) -> Result<CreateMarketRequest, SdkError> {
        if self.outcomes.len() < 2 {
            return Err(SdkError::invalid_input("at least 2 outcomes required"));
        }
        Ok(CreateMarketRequest::new(
            self.creator.ok_or_else(|| SdkError::invalid_input("creator is required"))?,
            self.feed_id.ok_or_else(|| SdkError::invalid_input("feed_id is required"))?,
            self.outcomes,
            self.criteria_json.ok_or_else(|| SdkError::invalid_input("criteria_json is required"))?,
            self.min_morm_stake.ok_or_else(|| SdkError::invalid_input("min_morm_stake is required"))?,
            self.creator_bucket_id.ok_or_else(|| SdkError::invalid_input("creator_bucket_id is required"))?,
            self.quote_asset_index,
        ))
    }
}

// ====================== RESOLVE MARKET ======================

#[derive(Default)]
pub struct ResolveMarketBuilder {
    signer: Option<String>,
    feed_id: Option<String>,
    winning_outcome_id: Option<u32>,
    confidence: Option<u32>,
}

impl ResolveMarketBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn signer(mut self, v: impl Into<String>) -> Self { self.signer = Some(v.into()); self }
    pub fn feed_id(mut self, v: impl Into<String>) -> Self { self.feed_id = Some(v.into()); self }
    pub fn winning_outcome_id(mut self, v: u32) -> Self { self.winning_outcome_id = Some(v); self }
    /// Fixed-point confidence: 0..1_000_000_000 (1e9 scale).
    pub fn confidence(mut self, v: u32) -> Self { self.confidence = Some(v); self }

    pub fn build(self) -> Result<ResolveMarketRequest, SdkError> {
        let outcome = ResolvedOutcome {
            feed_id: self.feed_id.ok_or_else(|| SdkError::invalid_input("feed_id is required"))?,
            winning_outcome_id: self.winning_outcome_id.ok_or_else(|| SdkError::invalid_input("winning_outcome_id is required"))?,
            confidence: self.confidence.ok_or_else(|| SdkError::invalid_input("confidence is required"))?,
        };
        Ok(ResolveMarketRequest::new(
            self.signer.ok_or_else(|| SdkError::invalid_input("signer is required"))?,
            outcome,
        ))
    }
}

// ====================== DISPUTE MARKET ======================

#[derive(Default)]
pub struct DisputeMarketBuilder {
    challenger: Option<String>,
    feed_id: Option<String>,
    bond_amount: Option<String>,
    evidence_data: Vec<u8>,
    reason_code: u32,
    challenger_bucket_id: Option<u64>,
}

impl DisputeMarketBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn challenger(mut self, v: impl Into<String>) -> Self { self.challenger = Some(v.into()); self }
    pub fn feed_id(mut self, v: impl Into<String>) -> Self { self.feed_id = Some(v.into()); self }
    pub fn bond_amount(mut self, v: impl Into<String>) -> Self { self.bond_amount = Some(v.into()); self }
    pub fn evidence_data(mut self, v: Vec<u8>) -> Self { self.evidence_data = v; self }
    pub fn reason_code(mut self, v: u32) -> Self { self.reason_code = v; self }
    pub fn challenger_bucket_id(mut self, v: u64) -> Self { self.challenger_bucket_id = Some(v); self }

    pub fn build(self) -> Result<DisputeMarketRequest, SdkError> {
        Ok(DisputeMarketRequest::new(
            self.challenger.ok_or_else(|| SdkError::invalid_input("challenger is required"))?,
            self.feed_id.ok_or_else(|| SdkError::invalid_input("feed_id is required"))?,
            self.bond_amount.ok_or_else(|| SdkError::invalid_input("bond_amount is required"))?,
            self.evidence_data,
            self.reason_code,
            self.challenger_bucket_id.ok_or_else(|| SdkError::invalid_input("challenger_bucket_id is required"))?,
        ))
    }
}

// ====================== OPEN LIGHT CHALLENGE ======================

#[derive(Default)]
pub struct OpenLightChallengeBuilder {
    challenger: Option<String>,
    feed_id: Option<String>,
    winning_outcome_id: Option<u32>,
    confidence: Option<u32>,
}

impl OpenLightChallengeBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn challenger(mut self, v: impl Into<String>) -> Self { self.challenger = Some(v.into()); self }
    pub fn feed_id(mut self, v: impl Into<String>) -> Self { self.feed_id = Some(v.into()); self }
    pub fn winning_outcome_id(mut self, v: u32) -> Self { self.winning_outcome_id = Some(v); self }
    pub fn confidence(mut self, v: u32) -> Self { self.confidence = Some(v); self }

    pub fn build(self) -> Result<OpenLightChallengeRequest, SdkError> {
        let feed_id = self.feed_id.ok_or_else(|| SdkError::invalid_input("feed_id is required"))?;
        let outcome = ResolvedOutcome {
            feed_id: feed_id.clone(),
            winning_outcome_id: self.winning_outcome_id.ok_or_else(|| SdkError::invalid_input("winning_outcome_id is required"))?,
            confidence: self.confidence.ok_or_else(|| SdkError::invalid_input("confidence is required"))?,
        };
        Ok(OpenLightChallengeRequest::new(
            self.challenger.ok_or_else(|| SdkError::invalid_input("challenger is required"))?,
            feed_id, outcome,
        ))
    }
}

// ====================== LIGHT CHALLENGE VOTE ======================

#[derive(Default)]
pub struct LightChallengeVoteBuilder {
    voter: Option<String>,
    feed_id: Option<String>,
    agree: bool,
    weight: u64,
}

impl LightChallengeVoteBuilder {
    pub fn new() -> Self { Self { weight: 1, ..Self::default() } }

    pub fn voter(mut self, v: impl Into<String>) -> Self { self.voter = Some(v.into()); self }
    pub fn feed_id(mut self, v: impl Into<String>) -> Self { self.feed_id = Some(v.into()); self }
    pub fn agree(mut self, v: bool) -> Self { self.agree = v; self }
    pub fn weight(mut self, v: u64) -> Self { self.weight = v; self }

    pub fn build(self) -> Result<LightChallengeVoteRequest, SdkError> {
        Ok(LightChallengeVoteRequest::new(
            self.voter.ok_or_else(|| SdkError::invalid_input("voter is required"))?,
            self.feed_id.ok_or_else(|| SdkError::invalid_input("feed_id is required"))?,
            self.agree,
            self.weight,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn create_market_builder_works() {
        let req = CreateMarketBuilder::new()
            .creator("morph1xyz").feed_id("btc-50k")
            .outcomes(vec!["yes".into(), "no".into()])
            .criteria_json("{}").min_morm_stake("100000")
            .creator_bucket_id(1)
            .build().unwrap();
        assert_eq!(req.feed_id, "btc-50k");
        assert_eq!(req.quote_asset_index, 1);
    }

    #[test]
    fn create_market_requires_two_outcomes() {
        let result = CreateMarketBuilder::new()
            .creator("morph1xyz").feed_id("f1")
            .outcomes(vec!["only_one".into()])
            .criteria_json("{}").min_morm_stake("100000")
            .creator_bucket_id(1)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn create_market_validation() {
        assert!(CreateMarketBuilder::new().build().is_err());
    }

    #[test]
    fn resolve_market_builder_works() {
        let req = ResolveMarketBuilder::new()
            .signer("morph1gov").feed_id("f1")
            .winning_outcome_id(0).confidence(1_000_000_000)
            .build().unwrap();
        assert_eq!(req.outcome.confidence, 1_000_000_000);
    }

    #[test]
    fn dispute_market_builder_works() {
        let req = DisputeMarketBuilder::new()
            .challenger("morph1ch").feed_id("f1")
            .bond_amount("50000").reason_code(1)
            .challenger_bucket_id(2)
            .build().unwrap();
        assert_eq!(req.feed_id, "f1");
    }

    #[test]
    fn open_light_challenge_builder_works() {
        let req = OpenLightChallengeBuilder::new()
            .challenger("morph1ch").feed_id("f1")
            .winning_outcome_id(1).confidence(800_000_000)
            .build().unwrap();
        assert_eq!(req.proposed_outcome.winning_outcome_id, 1);
    }

    #[test]
    fn light_challenge_vote_builder_works() {
        let req = LightChallengeVoteBuilder::new()
            .voter("morph1v").feed_id("f1").agree(true)
            .build().unwrap();
        assert!(req.agree);
        assert_eq!(req.weight, 1);
    }

    #[test]
    fn light_challenge_vote_validation() {
        assert!(LightChallengeVoteBuilder::new().build().is_err());
    }
}
