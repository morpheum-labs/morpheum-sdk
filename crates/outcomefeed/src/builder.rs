//! Fluent builder for the outcome feed module.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::RegisterPredictionFeedRequest;
use crate::types::{MarketResolutionCriteria, ResolutionParadigm};

/// Fluent builder for registering a prediction market feed.
#[derive(Default)]
pub struct RegisterPredictionFeedBuilder {
    from_address: Option<String>,
    feed_id: Option<String>,
    paradigm: Option<ResolutionParadigm>,
    resolution_deadline: Option<u64>,
    dispute_window_sec: u64,
    trusted_sources: Vec<String>,
    criteria_json_bytes: Vec<u8>,
}

impl RegisterPredictionFeedBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn from_address(mut self, v: impl Into<String>) -> Self { self.from_address = Some(v.into()); self }
    pub fn feed_id(mut self, v: impl Into<String>) -> Self { self.feed_id = Some(v.into()); self }
    pub fn paradigm(mut self, v: ResolutionParadigm) -> Self { self.paradigm = Some(v); self }
    pub fn resolution_deadline(mut self, v: u64) -> Self { self.resolution_deadline = Some(v); self }
    pub fn dispute_window_sec(mut self, v: u64) -> Self { self.dispute_window_sec = v; self }
    pub fn trusted_sources(mut self, v: Vec<String>) -> Self { self.trusted_sources = v; self }
    pub fn criteria_json_bytes(mut self, v: Vec<u8>) -> Self { self.criteria_json_bytes = v; self }

    pub fn build(self) -> Result<RegisterPredictionFeedRequest, SdkError> {
        let feed_id = self.feed_id.ok_or_else(|| SdkError::invalid_input("feed_id is required"))?;
        let criteria = MarketResolutionCriteria {
            feed_id: feed_id.clone(),
            resolution_deadline: self.resolution_deadline.ok_or_else(|| SdkError::invalid_input("resolution_deadline is required"))?,
            dispute_window_sec: self.dispute_window_sec,
            trusted_sources: self.trusted_sources,
            criteria_json_bytes: self.criteria_json_bytes,
        };

        Ok(RegisterPredictionFeedRequest::new(
            self.from_address.ok_or_else(|| SdkError::invalid_input("from_address is required"))?,
            feed_id,
            self.paradigm.ok_or_else(|| SdkError::invalid_input("paradigm is required"))?,
            criteria,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_works() {
        let req = RegisterPredictionFeedBuilder::new()
            .from_address("morph1xyz").feed_id("btc-50k")
            .paradigm(ResolutionParadigm::MarketPrice)
            .resolution_deadline(1_700_000_000)
            .dispute_window_sec(3600)
            .build().unwrap();
        assert_eq!(req.feed_id, "btc-50k");
        assert_eq!(req.paradigm, ResolutionParadigm::MarketPrice);
    }

    #[test]
    fn builder_validation() {
        assert!(RegisterPredictionFeedBuilder::new().build().is_err());
    }

    #[test]
    fn builder_missing_paradigm() {
        let result = RegisterPredictionFeedBuilder::new()
            .from_address("morph1xyz").feed_id("f1")
            .resolution_deadline(1_700_000_000)
            .build();
        assert!(result.is_err());
    }
}
