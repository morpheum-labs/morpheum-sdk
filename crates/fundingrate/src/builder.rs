//! Fluent builders for the funding-rate module.

use morpheum_sdk_core::SdkError;

use crate::requests::{EpochTickRequest, UpdateMarketProfileRequest};
use crate::types::FundingMarketProfile;

// ====================== EPOCH TICK ======================

/// Fluent builder for triggering an epoch tick.
#[derive(Default)]
pub struct EpochTickBuilder {
    epoch_id: Option<u64>,
    logical_timestamp: Option<u64>,
}

impl EpochTickBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn epoch_id(mut self, id: u64) -> Self { self.epoch_id = Some(id); self }
    pub fn logical_timestamp(mut self, ts: u64) -> Self { self.logical_timestamp = Some(ts); self }

    pub fn build(self) -> Result<EpochTickRequest, SdkError> {
        let epoch_id = self.epoch_id.ok_or_else(|| SdkError::invalid_input("epoch_id is required"))?;
        let logical_timestamp = self.logical_timestamp.ok_or_else(|| SdkError::invalid_input("logical_timestamp is required"))?;
        Ok(EpochTickRequest::new(epoch_id, logical_timestamp))
    }
}

// ====================== UPDATE MARKET PROFILE ======================

/// Fluent builder for updating a market's funding profile.
#[derive(Default)]
pub struct UpdateMarketProfileBuilder {
    market_index: Option<u64>,
    profile: Option<FundingMarketProfile>,
}

impl UpdateMarketProfileBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn market_index(mut self, idx: u64) -> Self { self.market_index = Some(idx); self }
    pub fn profile(mut self, p: FundingMarketProfile) -> Self { self.profile = Some(p); self }

    pub fn build(self) -> Result<UpdateMarketProfileRequest, SdkError> {
        let market_index = self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?;
        let profile = self.profile.ok_or_else(|| SdkError::invalid_input("profile is required"))?;
        Ok(UpdateMarketProfileRequest::new(market_index, profile))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FundingApplicationMode;

    #[test]
    fn epoch_tick_builder_works() {
        let req = EpochTickBuilder::new().epoch_id(1).logical_timestamp(1_700_000_000).build().unwrap();
        assert_eq!(req.epoch_id, 1);
    }

    #[test]
    fn epoch_tick_builder_validation() {
        assert!(EpochTickBuilder::new().build().is_err());
    }

    #[test]
    fn update_profile_builder_works() {
        let profile = FundingMarketProfile {
            mode: FundingApplicationMode::BothSides,
            vrf_bias_bps: 0, protocol_cut_bps: 0, lp_incentive_bps: 0,
        };
        let req = UpdateMarketProfileBuilder::new().market_index(42).profile(profile).build().unwrap();
        assert_eq!(req.market_index, 42);
    }

    #[test]
    fn update_profile_builder_validation() {
        assert!(UpdateMarketProfileBuilder::new().build().is_err());
    }
}
