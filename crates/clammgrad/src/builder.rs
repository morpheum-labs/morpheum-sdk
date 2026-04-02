//! Fluent builders for the CLAMM Graduation module.

use alloc::string::String;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    CancelGraduationRequest, InitiateGraduationRequest,
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
    fn cancel_builder_works() {
        let req = CancelGraduationBuilder::new().token_index("42").build().unwrap();
        assert_eq!(req.token_index, "42");
    }

    #[test]
    fn cancel_builder_validation() {
        assert!(CancelGraduationBuilder::new().build().is_err());
    }
}
