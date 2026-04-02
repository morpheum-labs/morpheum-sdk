//! Fluent builders for the funding-rate module.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::UpdateMarketProfileRequest;
use crate::types::FundingMarketProfile;

// ====================== UPDATE MARKET PROFILE ======================

/// Fluent builder for updating a market's funding profile.
#[derive(Default)]
pub struct UpdateMarketProfileBuilder {
    authority: Option<String>,
    market_index: Option<u64>,
    profile: Option<FundingMarketProfile>,
}

impl UpdateMarketProfileBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn authority(mut self, authority: impl Into<String>) -> Self {
        self.authority = Some(authority.into());
        self
    }
    pub fn market_index(mut self, idx: u64) -> Self { self.market_index = Some(idx); self }
    pub fn profile(mut self, p: FundingMarketProfile) -> Self { self.profile = Some(p); self }

    pub fn build(self) -> Result<UpdateMarketProfileRequest, SdkError> {
        let authority = self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?;
        let market_index = self.market_index.ok_or_else(|| SdkError::invalid_input("market_index is required"))?;
        let profile = self.profile.ok_or_else(|| SdkError::invalid_input("profile is required"))?;
        Ok(UpdateMarketProfileRequest::new(authority, market_index, profile))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::FundingApplicationMode;

    #[test]
    fn update_profile_builder_works() {
        let profile = FundingMarketProfile {
            mode: FundingApplicationMode::BothSides,
            vrf_bias_bps: 0, protocol_cut_bps: 0, lp_incentive_bps: 0,
        };
        let req = UpdateMarketProfileBuilder::new()
            .authority("morpheum1gov")
            .market_index(42)
            .profile(profile)
            .build()
            .unwrap();
        assert_eq!(req.authority, "morpheum1gov");
        assert_eq!(req.market_index, 42);
    }

    #[test]
    fn update_profile_builder_validation() {
        assert!(UpdateMarketProfileBuilder::new().build().is_err());
    }
}
