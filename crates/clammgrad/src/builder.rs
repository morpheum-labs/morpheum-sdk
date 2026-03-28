//! Fluent builders for the CLAMM Graduation module.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    CancelGraduationRequest, ExecuteGraduationStepRequest, InitiateGraduationRequest,
};

// ====================== INITIATE GRADUATION ======================

/// Fluent builder for initiating CLAMM graduation.
#[derive(Default)]
pub struct InitiateGraduationBuilder {
    token_index: Option<String>,
}

impl InitiateGraduationBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn token_index(mut self, idx: impl Into<String>) -> Self { self.token_index = Some(idx.into()); self }

    pub fn build(self) -> Result<InitiateGraduationRequest, SdkError> {
        let token_index = self.token_index.ok_or_else(|| SdkError::invalid_input("token_index is required"))?;
        Ok(InitiateGraduationRequest::new(token_index))
    }
}

// ====================== EXECUTE STEP ======================

/// Fluent builder for executing a specific graduation step.
#[derive(Default)]
pub struct ExecuteStepBuilder {
    token_index: Option<String>,
    step: Option<u32>,
}

impl ExecuteStepBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn token_index(mut self, idx: impl Into<String>) -> Self { self.token_index = Some(idx.into()); self }
    pub fn step(mut self, s: u32) -> Self { self.step = Some(s); self }

    pub fn build(self) -> Result<ExecuteGraduationStepRequest, SdkError> {
        let token_index = self.token_index.ok_or_else(|| SdkError::invalid_input("token_index is required"))?;
        let step = self.step.ok_or_else(|| SdkError::invalid_input("step is required"))?;
        Ok(ExecuteGraduationStepRequest::new(token_index, step))
    }
}

// ====================== CANCEL GRADUATION ======================

/// Fluent builder for cancelling an in-progress graduation.
#[derive(Default)]
pub struct CancelGraduationBuilder {
    token_index: Option<String>,
}

impl CancelGraduationBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn token_index(mut self, idx: impl Into<String>) -> Self { self.token_index = Some(idx.into()); self }

    pub fn build(self) -> Result<CancelGraduationRequest, SdkError> {
        let token_index = self.token_index.ok_or_else(|| SdkError::invalid_input("token_index is required"))?;
        Ok(CancelGraduationRequest::new(token_index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initiate_builder_works() {
        let req = InitiateGraduationBuilder::new().token_index("42").build().unwrap();
        assert_eq!(req.token_index, "42");
    }

    #[test]
    fn initiate_builder_validation() {
        assert!(InitiateGraduationBuilder::new().build().is_err());
    }

    #[test]
    fn execute_step_builder_works() {
        let req = ExecuteStepBuilder::new().token_index("42").step(2).build().unwrap();
        assert_eq!(req.token_index, "42");
        assert_eq!(req.step, 2);
    }

    #[test]
    fn execute_step_builder_validation() {
        assert!(ExecuteStepBuilder::new().build().is_err());
        assert!(ExecuteStepBuilder::new().token_index("42").build().is_err());
    }

    #[test]
    fn cancel_builder_works() {
        let req = CancelGraduationBuilder::new().token_index("42").build().unwrap();
        assert_eq!(req.token_index, "42");
    }

    #[test]
    fn cancel_builder_validation() {
        assert!(CancelGraduationBuilder::new().build().is_err());
    }
}
