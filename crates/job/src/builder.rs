//! Fluent builders for the Job module (ERC-8183).
//!
//! Ergonomic, type-safe builders for all job lifecycle operations.
//! Each builder validates required fields and returns the corresponding
//! request type from `requests.rs` for seamless integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    AttestRequest, CancelJobRequest, ClaimRefundRequest, CreateJobRequest,
    FundJobRequest, SetProviderRequest, SubmitDeliverableRequest,
};
use crate::types::{Deliverable, Job, RevenueShareConfig};

/// Fluent builder for creating a new job.
#[derive(Default)]
pub struct CreateJobBuilder {
    client_agent_hash: Option<String>,
    evaluator_agent_hash: Option<String>,
    budget_usd: Option<u64>,
    expiry_timestamp: Option<u64>,
    provider_agent_hash: Option<String>,
    hook_address: Option<String>,
    vc_proof_hash: Option<String>,
    revenue_share_config: Option<RevenueShareConfig>,
    job_spec_hash: Option<String>,
    metadata_payload: Option<Vec<u8>>,
    client_signature: Option<Vec<u8>>,
}

impl CreateJobBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn client_agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.client_agent_hash = Some(hash.into());
        self
    }

    pub fn evaluator_agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.evaluator_agent_hash = Some(hash.into());
        self
    }

    pub fn budget_usd(mut self, budget: u64) -> Self {
        self.budget_usd = Some(budget);
        self
    }

    pub fn expiry_timestamp(mut self, ts: u64) -> Self {
        self.expiry_timestamp = Some(ts);
        self
    }

    pub fn provider_agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.provider_agent_hash = Some(hash.into());
        self
    }

    pub fn hook_address(mut self, addr: impl Into<String>) -> Self {
        self.hook_address = Some(addr.into());
        self
    }

    pub fn vc_proof_hash(mut self, hash: impl Into<String>) -> Self {
        self.vc_proof_hash = Some(hash.into());
        self
    }

    pub fn revenue_share_config(mut self, config: RevenueShareConfig) -> Self {
        self.revenue_share_config = Some(config);
        self
    }

    pub fn job_spec_hash(mut self, hash: impl Into<String>) -> Self {
        self.job_spec_hash = Some(hash.into());
        self
    }

    pub fn metadata_payload(mut self, payload: Vec<u8>) -> Self {
        self.metadata_payload = Some(payload);
        self
    }

    pub fn client_signature(mut self, sig: Vec<u8>) -> Self {
        self.client_signature = Some(sig);
        self
    }

    pub fn build(self) -> Result<CreateJobRequest, SdkError> {
        let client_agent_hash = self.client_agent_hash.ok_or_else(|| {
            SdkError::invalid_input("client_agent_hash is required for job creation")
        })?;

        let evaluator_agent_hash = self.evaluator_agent_hash.ok_or_else(|| {
            SdkError::invalid_input("evaluator_agent_hash is required for job creation")
        })?;

        let budget_usd = self.budget_usd.ok_or_else(|| {
            SdkError::invalid_input("budget_usd is required for job creation")
        })?;

        let client_signature = self.client_signature.ok_or_else(|| {
            SdkError::invalid_input("client_signature is required for job creation")
        })?;

        let job = Job {
            client_agent_hash,
            evaluator_agent_hash,
            budget_usd,
            expiry_timestamp: self.expiry_timestamp.unwrap_or(0),
            provider_agent_hash: self.provider_agent_hash.unwrap_or_default(),
            hook_address: self.hook_address.unwrap_or_default(),
            vc_proof_hash: self.vc_proof_hash.unwrap_or_default(),
            revenue_share_config: self.revenue_share_config,
            job_spec_hash: self.job_spec_hash.unwrap_or_default(),
            metadata_payload: self.metadata_payload.unwrap_or_default(),
            ..Job::default()
        };

        Ok(CreateJobRequest::new(job, client_signature))
    }
}

/// Fluent builder for funding a job.
#[derive(Default)]
pub struct FundJobBuilder {
    job_id: Option<String>,
    amount_usd: Option<u64>,
    client_signature: Option<Vec<u8>>,
}

impl FundJobBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn job_id(mut self, id: impl Into<String>) -> Self {
        self.job_id = Some(id.into());
        self
    }

    pub fn amount_usd(mut self, amount: u64) -> Self {
        self.amount_usd = Some(amount);
        self
    }

    pub fn client_signature(mut self, sig: Vec<u8>) -> Self {
        self.client_signature = Some(sig);
        self
    }

    pub fn build(self) -> Result<FundJobRequest, SdkError> {
        let job_id = self.job_id.ok_or_else(|| {
            SdkError::invalid_input("job_id is required for funding")
        })?;

        let amount_usd = self.amount_usd.ok_or_else(|| {
            SdkError::invalid_input("amount_usd is required for funding")
        })?;

        let client_signature = self.client_signature.ok_or_else(|| {
            SdkError::invalid_input("client_signature is required for funding")
        })?;

        Ok(FundJobRequest::new(job_id, amount_usd, client_signature))
    }
}

/// Fluent builder for submitting a deliverable.
#[derive(Default)]
pub struct SubmitDeliverableBuilder {
    job_id: Option<String>,
    deliverable: Option<Deliverable>,
    provider_signature: Option<Vec<u8>>,
}

impl SubmitDeliverableBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn job_id(mut self, id: impl Into<String>) -> Self {
        self.job_id = Some(id.into());
        self
    }

    pub fn deliverable(mut self, deliverable: Deliverable) -> Self {
        self.deliverable = Some(deliverable);
        self
    }

    pub fn provider_signature(mut self, sig: Vec<u8>) -> Self {
        self.provider_signature = Some(sig);
        self
    }

    pub fn build(self) -> Result<SubmitDeliverableRequest, SdkError> {
        let job_id = self.job_id.ok_or_else(|| {
            SdkError::invalid_input("job_id is required for deliverable submission")
        })?;

        let deliverable = self.deliverable.ok_or_else(|| {
            SdkError::invalid_input("deliverable is required for submission")
        })?;

        let provider_signature = self.provider_signature.ok_or_else(|| {
            SdkError::invalid_input("provider_signature is required for submission")
        })?;

        Ok(SubmitDeliverableRequest::new(job_id, deliverable, provider_signature))
    }
}

/// Fluent builder for attesting a job (evaluator action).
#[derive(Default)]
pub struct AttestBuilder {
    job_id: Option<String>,
    completed: Option<bool>,
    reason_hash: Option<String>,
    evaluator_signature: Option<Vec<u8>>,
}

impl AttestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn job_id(mut self, id: impl Into<String>) -> Self {
        self.job_id = Some(id.into());
        self
    }

    pub fn completed(mut self, completed: bool) -> Self {
        self.completed = Some(completed);
        self
    }

    pub fn reason_hash(mut self, hash: impl Into<String>) -> Self {
        self.reason_hash = Some(hash.into());
        self
    }

    pub fn evaluator_signature(mut self, sig: Vec<u8>) -> Self {
        self.evaluator_signature = Some(sig);
        self
    }

    pub fn build(self) -> Result<AttestRequest, SdkError> {
        let job_id = self.job_id.ok_or_else(|| {
            SdkError::invalid_input("job_id is required for attestation")
        })?;

        let completed = self.completed.ok_or_else(|| {
            SdkError::invalid_input("completed flag is required for attestation")
        })?;

        let evaluator_signature = self.evaluator_signature.ok_or_else(|| {
            SdkError::invalid_input("evaluator_signature is required for attestation")
        })?;

        let mut req = AttestRequest::new(job_id, completed, evaluator_signature);
        if let Some(hash) = self.reason_hash {
            req = req.with_reason_hash(hash);
        }
        Ok(req)
    }
}

/// Fluent builder for claiming a refund.
#[derive(Default)]
pub struct ClaimRefundBuilder {
    job_id: Option<String>,
    caller_signature: Option<Vec<u8>>,
}

impl ClaimRefundBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn job_id(mut self, id: impl Into<String>) -> Self {
        self.job_id = Some(id.into());
        self
    }

    pub fn caller_signature(mut self, sig: Vec<u8>) -> Self {
        self.caller_signature = Some(sig);
        self
    }

    pub fn build(self) -> Result<ClaimRefundRequest, SdkError> {
        let job_id = self.job_id.ok_or_else(|| {
            SdkError::invalid_input("job_id is required for refund claim")
        })?;

        let caller_signature = self.caller_signature.ok_or_else(|| {
            SdkError::invalid_input("caller_signature is required for refund claim")
        })?;

        Ok(ClaimRefundRequest::new(job_id, caller_signature))
    }
}

/// Fluent builder for setting a provider on a job.
#[derive(Default)]
pub struct SetProviderBuilder {
    job_id: Option<String>,
    new_provider_agent_hash: Option<String>,
    client_signature: Option<Vec<u8>>,
}

impl SetProviderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn job_id(mut self, id: impl Into<String>) -> Self {
        self.job_id = Some(id.into());
        self
    }

    pub fn new_provider_agent_hash(mut self, hash: impl Into<String>) -> Self {
        self.new_provider_agent_hash = Some(hash.into());
        self
    }

    pub fn client_signature(mut self, sig: Vec<u8>) -> Self {
        self.client_signature = Some(sig);
        self
    }

    pub fn build(self) -> Result<SetProviderRequest, SdkError> {
        let job_id = self.job_id.ok_or_else(|| {
            SdkError::invalid_input("job_id is required for setting provider")
        })?;

        let new_provider_agent_hash = self.new_provider_agent_hash.ok_or_else(|| {
            SdkError::invalid_input("new_provider_agent_hash is required")
        })?;

        let client_signature = self.client_signature.ok_or_else(|| {
            SdkError::invalid_input("client_signature is required for setting provider")
        })?;

        Ok(SetProviderRequest::new(job_id, new_provider_agent_hash, client_signature))
    }
}

/// Fluent builder for cancelling a job.
#[derive(Default)]
pub struct CancelJobBuilder {
    job_id: Option<String>,
    signer_signature: Option<Vec<u8>>,
}

impl CancelJobBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn job_id(mut self, id: impl Into<String>) -> Self {
        self.job_id = Some(id.into());
        self
    }

    pub fn signer_signature(mut self, sig: Vec<u8>) -> Self {
        self.signer_signature = Some(sig);
        self
    }

    pub fn build(self) -> Result<CancelJobRequest, SdkError> {
        let job_id = self.job_id.ok_or_else(|| {
            SdkError::invalid_input("job_id is required for cancellation")
        })?;

        let signer_signature = self.signer_signature.ok_or_else(|| {
            SdkError::invalid_input("signer_signature is required for cancellation")
        })?;

        Ok(CancelJobRequest::new(job_id, signer_signature))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn create_job_builder_full_flow() {
        let req = CreateJobBuilder::new()
            .client_agent_hash("client-abc")
            .evaluator_agent_hash("eval-007")
            .budget_usd(1000)
            .expiry_timestamp(1_700_100_000)
            .provider_agent_hash("provider-xyz")
            .hook_address("hook-addr")
            .job_spec_hash("spec-hash")
            .client_signature(vec![1, 2, 3])
            .build()
            .unwrap();

        assert_eq!(req.job.client_agent_hash, "client-abc");
        assert_eq!(req.job.budget_usd, 1000);
        assert_eq!(req.job.provider_agent_hash, "provider-xyz");
    }

    #[test]
    fn create_job_builder_missing_required() {
        let result = CreateJobBuilder::new().build();
        assert!(result.is_err());
    }

    #[test]
    fn fund_job_builder_works() {
        let req = FundJobBuilder::new()
            .job_id("job-1")
            .amount_usd(500)
            .client_signature(vec![4, 5])
            .build()
            .unwrap();

        assert_eq!(req.job_id, "job-1");
        assert_eq!(req.amount_usd, 500);
    }

    #[test]
    fn attest_builder_with_reason() {
        let req = AttestBuilder::new()
            .job_id("job-1")
            .completed(true)
            .reason_hash("reason-abc")
            .evaluator_signature(vec![6, 7])
            .build()
            .unwrap();

        assert!(req.completed);
        assert_eq!(req.reason_hash, "reason-abc");
    }

    #[test]
    fn set_provider_builder_validation() {
        let result = SetProviderBuilder::new()
            .job_id("job-1")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn cancel_job_builder_works() {
        let req = CancelJobBuilder::new()
            .job_id("job-1")
            .signer_signature(vec![8, 9])
            .build()
            .unwrap();

        assert_eq!(req.job_id, "job-1");
    }
}
