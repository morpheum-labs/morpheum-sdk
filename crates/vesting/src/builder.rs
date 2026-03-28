//! Fluent builders for the vesting module.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{ClaimRequest, CreateVestingRequest, RevokeVestingRequest, UpdateParamsRequest};
use crate::types::{ScheduleType, VestingCategory, VestingParams};

// ====================== CREATE VESTING ======================

pub struct CreateVestingBuilder {
    authority: Option<String>,
    beneficiary: Option<String>,
    total_amount: Option<String>,
    start_timestamp: u64,
    cliff_duration: u64,
    vesting_duration: Option<u64>,
    schedule_type: ScheduleType,
    category: VestingCategory,
    revocable: bool,
    step_timestamps: Vec<u64>,
    step_amounts: Vec<String>,
}

impl CreateVestingBuilder {
    pub fn new() -> Self {
        Self {
            authority: None, beneficiary: None, total_amount: None,
            start_timestamp: 0, cliff_duration: 0, vesting_duration: None,
            schedule_type: ScheduleType::Unspecified,
            category: VestingCategory::Unspecified,
            revocable: false, step_timestamps: Vec::new(), step_amounts: Vec::new(),
        }
    }

    pub fn authority(mut self, v: impl Into<String>) -> Self { self.authority = Some(v.into()); self }
    pub fn beneficiary(mut self, v: impl Into<String>) -> Self { self.beneficiary = Some(v.into()); self }
    pub fn total_amount(mut self, v: impl Into<String>) -> Self { self.total_amount = Some(v.into()); self }
    pub fn start_timestamp(mut self, v: u64) -> Self { self.start_timestamp = v; self }
    pub fn cliff_duration(mut self, v: u64) -> Self { self.cliff_duration = v; self }
    pub fn vesting_duration(mut self, v: u64) -> Self { self.vesting_duration = Some(v); self }
    pub fn schedule_type(mut self, v: ScheduleType) -> Self { self.schedule_type = v; self }
    pub fn category(mut self, v: VestingCategory) -> Self { self.category = v; self }
    pub fn revocable(mut self, v: bool) -> Self { self.revocable = v; self }
    pub fn add_step(mut self, timestamp: u64, amount: impl Into<String>) -> Self {
        self.step_timestamps.push(timestamp);
        self.step_amounts.push(amount.into());
        self
    }

    pub fn build(self) -> Result<CreateVestingRequest, SdkError> {
        if self.schedule_type == ScheduleType::Unspecified {
            return Err(SdkError::invalid_input("schedule_type must be specified"));
        }
        if self.schedule_type == ScheduleType::Step && self.step_timestamps.is_empty() {
            return Err(SdkError::invalid_input("step schedule requires at least one step"));
        }
        let mut req = CreateVestingRequest::new(
            self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?,
            self.beneficiary.ok_or_else(|| SdkError::invalid_input("beneficiary is required"))?,
            self.total_amount.ok_or_else(|| SdkError::invalid_input("total_amount is required"))?,
            self.vesting_duration.ok_or_else(|| SdkError::invalid_input("vesting_duration is required"))?,
            self.schedule_type,
        );
        req.start_timestamp = self.start_timestamp;
        req.cliff_duration = self.cliff_duration;
        req.category = self.category;
        req.revocable = self.revocable;
        req.step_timestamps = self.step_timestamps;
        req.step_amounts = self.step_amounts;
        Ok(req)
    }
}

impl Default for CreateVestingBuilder {
    fn default() -> Self { Self::new() }
}

// ====================== CLAIM ======================

pub struct ClaimBuilder {
    beneficiary: Option<String>,
    amount: Option<String>,
}

impl ClaimBuilder {
    pub fn new() -> Self { Self { beneficiary: None, amount: None } }

    pub fn beneficiary(mut self, v: impl Into<String>) -> Self { self.beneficiary = Some(v.into()); self }
    pub fn amount(mut self, v: impl Into<String>) -> Self { self.amount = Some(v.into()); self }

    pub fn build(self) -> Result<ClaimRequest, SdkError> {
        let b = self.beneficiary.ok_or_else(|| SdkError::invalid_input("beneficiary is required"))?;
        match self.amount {
            Some(a) if !a.is_empty() => Ok(ClaimRequest::amount(b, a)),
            _ => Ok(ClaimRequest::max(b)),
        }
    }
}

impl Default for ClaimBuilder {
    fn default() -> Self { Self::new() }
}

// ====================== REVOKE ======================

pub struct RevokeVestingBuilder {
    authority: Option<String>,
    beneficiary: Option<String>,
    vesting_id: Option<u64>,
    reason: Option<String>,
}

impl RevokeVestingBuilder {
    pub fn new() -> Self {
        Self { authority: None, beneficiary: None, vesting_id: None, reason: None }
    }

    pub fn authority(mut self, v: impl Into<String>) -> Self { self.authority = Some(v.into()); self }
    pub fn beneficiary(mut self, v: impl Into<String>) -> Self { self.beneficiary = Some(v.into()); self }
    pub fn vesting_id(mut self, v: u64) -> Self { self.vesting_id = Some(v); self }
    pub fn reason(mut self, v: impl Into<String>) -> Self { self.reason = Some(v.into()); self }

    pub fn build(self) -> Result<RevokeVestingRequest, SdkError> {
        Ok(RevokeVestingRequest::new(
            self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?,
            self.beneficiary.ok_or_else(|| SdkError::invalid_input("beneficiary is required"))?,
            self.vesting_id.ok_or_else(|| SdkError::invalid_input("vesting_id is required"))?,
            self.reason.ok_or_else(|| SdkError::invalid_input("reason is required"))?,
        ))
    }
}

impl Default for RevokeVestingBuilder {
    fn default() -> Self { Self::new() }
}

// ====================== UPDATE PARAMS ======================

pub struct UpdateParamsBuilder {
    authority: Option<String>,
    params: Option<VestingParams>,
}

impl UpdateParamsBuilder {
    pub fn new() -> Self { Self { authority: None, params: None } }

    pub fn authority(mut self, v: impl Into<String>) -> Self { self.authority = Some(v.into()); self }
    pub fn params(mut self, v: VestingParams) -> Self { self.params = Some(v); self }

    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        Ok(UpdateParamsRequest::new(
            self.authority.ok_or_else(|| SdkError::invalid_input("authority is required"))?,
            self.params.ok_or_else(|| SdkError::invalid_input("params is required"))?,
        ))
    }
}

impl Default for UpdateParamsBuilder {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_linear_vesting() {
        let req = CreateVestingBuilder::new()
            .authority("morph1gov").beneficiary("morph1user")
            .total_amount("1000000").vesting_duration(63072000)
            .schedule_type(ScheduleType::Linear)
            .category(VestingCategory::Team)
            .build().unwrap();
        assert_eq!(req.schedule_type, ScheduleType::Linear);
        assert_eq!(req.category, VestingCategory::Team);
    }

    #[test]
    fn create_cliff_linear_vesting() {
        let req = CreateVestingBuilder::new()
            .authority("morph1gov").beneficiary("morph1user")
            .total_amount("2000000").cliff_duration(31536000)
            .vesting_duration(63072000).schedule_type(ScheduleType::CliffLinear)
            .category(VestingCategory::CoreContributors).revocable(true)
            .build().unwrap();
        assert_eq!(req.cliff_duration, 31536000);
        assert!(req.revocable);
    }

    #[test]
    fn create_step_requires_steps() {
        let result = CreateVestingBuilder::new()
            .authority("morph1gov").beneficiary("morph1user")
            .total_amount("1000000").vesting_duration(63072000)
            .schedule_type(ScheduleType::Step)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn create_step_with_steps() {
        let req = CreateVestingBuilder::new()
            .authority("morph1gov").beneficiary("morph1user")
            .total_amount("400000").vesting_duration(126144000)
            .schedule_type(ScheduleType::Step)
            .add_step(1700000000, "100000").add_step(1710000000, "100000")
            .add_step(1720000000, "100000").add_step(1730000000, "100000")
            .build().unwrap();
        assert_eq!(req.step_timestamps.len(), 4);
    }

    #[test]
    fn claim_max_builder() {
        let req = ClaimBuilder::new().beneficiary("morph1user").build().unwrap();
        assert!(req.amount.is_empty());
    }

    #[test]
    fn claim_specific_amount() {
        let req = ClaimBuilder::new().beneficiary("morph1user").amount("50000").build().unwrap();
        assert_eq!(req.amount, "50000");
    }

    #[test]
    fn revoke_builder_works() {
        let req = RevokeVestingBuilder::new()
            .authority("morph1gov").beneficiary("morph1user")
            .vesting_id(1).reason("policy violation")
            .build().unwrap();
        assert_eq!(req.vesting_id, 1);
    }

    #[test]
    fn revoke_requires_all_fields() {
        assert!(RevokeVestingBuilder::new().authority("morph1gov").build().is_err());
    }

    #[test]
    fn update_params_validation() {
        assert!(UpdateParamsBuilder::new().build().is_err());
    }
}
