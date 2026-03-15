//! Request and response wrappers for the Job module (ERC-8183).
//!
//! Clean, type-safe Rust APIs around the raw protobuf messages.
//! Includes `to_any()` methods for seamless integration with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::job::v1 as proto;

use crate::types::{Deliverable, Job, JobState};

// ====================== TRANSACTION REQUESTS ======================

/// Request to create a new job.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateJobRequest {
    pub job: Job,
    pub client_signature: Vec<u8>,
}

impl CreateJobRequest {
    pub fn new(job: Job, client_signature: Vec<u8>) -> Self {
        Self { job, client_signature }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCreateJob = self.clone().into();
        ProtoAny {
            type_url: "/job.v1.MsgCreateJob".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CreateJobRequest> for proto::MsgCreateJob {
    fn from(req: CreateJobRequest) -> Self {
        Self {
            job: Some(req.job.into()),
            client_signature: req.client_signature,
        }
    }
}

/// Request to fund an existing job (locks budget in bank).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FundJobRequest {
    pub job_id: String,
    pub amount_usd: u64,
    pub client_signature: Vec<u8>,
}

impl FundJobRequest {
    pub fn new(job_id: impl Into<String>, amount_usd: u64, client_signature: Vec<u8>) -> Self {
        Self {
            job_id: job_id.into(),
            amount_usd,
            client_signature,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgFundJob = self.clone().into();
        ProtoAny {
            type_url: "/job.v1.MsgFundJob".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<FundJobRequest> for proto::MsgFundJob {
    fn from(req: FundJobRequest) -> Self {
        Self {
            job_id: req.job_id,
            amount_usd: req.amount_usd,
            client_signature: req.client_signature,
        }
    }
}

/// Request to submit a deliverable (provider action).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SubmitDeliverableRequest {
    pub job_id: String,
    pub deliverable: Deliverable,
    pub provider_signature: Vec<u8>,
}

impl SubmitDeliverableRequest {
    pub fn new(
        job_id: impl Into<String>,
        deliverable: Deliverable,
        provider_signature: Vec<u8>,
    ) -> Self {
        Self {
            job_id: job_id.into(),
            deliverable,
            provider_signature,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgSubmitDeliverable = self.clone().into();
        ProtoAny {
            type_url: "/job.v1.MsgSubmitDeliverable".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<SubmitDeliverableRequest> for proto::MsgSubmitDeliverable {
    fn from(req: SubmitDeliverableRequest) -> Self {
        Self {
            job_id: req.job_id,
            deliverable: Some(req.deliverable.into()),
            provider_signature: req.provider_signature,
        }
    }
}

/// Request to attest completion or rejection (evaluator action).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttestRequest {
    pub job_id: String,
    pub completed: bool,
    pub reason_hash: String,
    pub evaluator_signature: Vec<u8>,
}

impl AttestRequest {
    pub fn new(
        job_id: impl Into<String>,
        completed: bool,
        evaluator_signature: Vec<u8>,
    ) -> Self {
        Self {
            job_id: job_id.into(),
            completed,
            reason_hash: String::new(),
            evaluator_signature,
        }
    }

    pub fn with_reason_hash(mut self, hash: impl Into<String>) -> Self {
        self.reason_hash = hash.into();
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgAttest = self.clone().into();
        ProtoAny {
            type_url: "/job.v1.MsgAttest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<AttestRequest> for proto::MsgAttest {
    fn from(req: AttestRequest) -> Self {
        Self {
            job_id: req.job_id,
            completed: req.completed,
            reason_hash: req.reason_hash,
            evaluator_signature: req.evaluator_signature,
        }
    }
}

/// Request to claim a refund after expiry.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClaimRefundRequest {
    pub job_id: String,
    pub caller_signature: Vec<u8>,
}

impl ClaimRefundRequest {
    pub fn new(job_id: impl Into<String>, caller_signature: Vec<u8>) -> Self {
        Self {
            job_id: job_id.into(),
            caller_signature,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgClaimRefund = self.clone().into();
        ProtoAny {
            type_url: "/job.v1.MsgClaimRefund".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ClaimRefundRequest> for proto::MsgClaimRefund {
    fn from(req: ClaimRefundRequest) -> Self {
        Self {
            job_id: req.job_id,
            caller_signature: req.caller_signature,
        }
    }
}

/// Request to set or change the provider.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SetProviderRequest {
    pub job_id: String,
    pub new_provider_agent_hash: String,
    pub client_signature: Vec<u8>,
}

impl SetProviderRequest {
    pub fn new(
        job_id: impl Into<String>,
        new_provider_agent_hash: impl Into<String>,
        client_signature: Vec<u8>,
    ) -> Self {
        Self {
            job_id: job_id.into(),
            new_provider_agent_hash: new_provider_agent_hash.into(),
            client_signature,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgSetProvider = self.clone().into();
        ProtoAny {
            type_url: "/job.v1.MsgSetProvider".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<SetProviderRequest> for proto::MsgSetProvider {
    fn from(req: SetProviderRequest) -> Self {
        Self {
            job_id: req.job_id,
            new_provider_agent_hash: req.new_provider_agent_hash,
            client_signature: req.client_signature,
        }
    }
}

/// Request to cancel a job.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CancelJobRequest {
    pub job_id: String,
    pub signer_signature: Vec<u8>,
}

impl CancelJobRequest {
    pub fn new(job_id: impl Into<String>, signer_signature: Vec<u8>) -> Self {
        Self {
            job_id: job_id.into(),
            signer_signature,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCancelJob = self.clone().into();
        ProtoAny {
            type_url: "/job.v1.MsgCancelJob".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CancelJobRequest> for proto::MsgCancelJob {
    fn from(req: CancelJobRequest) -> Self {
        Self {
            job_id: req.job_id,
            signer_signature: req.signer_signature,
        }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query a single job by ID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryJobRequest {
    pub job_id: String,
}

impl QueryJobRequest {
    pub fn new(job_id: impl Into<String>) -> Self {
        Self { job_id: job_id.into() }
    }
}

impl From<QueryJobRequest> for proto::QueryJobRequest {
    fn from(req: QueryJobRequest) -> Self {
        Self { job_id: req.job_id }
    }
}

/// Query jobs by client agent hash with optional state filter and pagination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryJobsByClientRequest {
    pub client_agent_hash: String,
    pub state: Option<JobState>,
    pub limit: u32,
    pub offset: u32,
}

impl QueryJobsByClientRequest {
    pub fn new(client_agent_hash: impl Into<String>, limit: u32, offset: u32) -> Self {
        Self {
            client_agent_hash: client_agent_hash.into(),
            state: None,
            limit,
            offset,
        }
    }

    pub fn with_state(mut self, state: JobState) -> Self {
        self.state = Some(state);
        self
    }
}

impl From<QueryJobsByClientRequest> for proto::QueryJobsByClientRequest {
    fn from(req: QueryJobsByClientRequest) -> Self {
        Self {
            client_agent_hash: req.client_agent_hash,
            state: req.state.map(i32::from).unwrap_or(0),
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query jobs by provider agent hash with optional state filter and pagination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryJobsByProviderRequest {
    pub provider_agent_hash: String,
    pub state: Option<JobState>,
    pub limit: u32,
    pub offset: u32,
}

impl QueryJobsByProviderRequest {
    pub fn new(provider_agent_hash: impl Into<String>, limit: u32, offset: u32) -> Self {
        Self {
            provider_agent_hash: provider_agent_hash.into(),
            state: None,
            limit,
            offset,
        }
    }

    pub fn with_state(mut self, state: JobState) -> Self {
        self.state = Some(state);
        self
    }
}

impl From<QueryJobsByProviderRequest> for proto::QueryJobsByProviderRequest {
    fn from(req: QueryJobsByProviderRequest) -> Self {
        Self {
            provider_agent_hash: req.provider_agent_hash,
            state: req.state.map(i32::from).unwrap_or(0),
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query jobs by evaluator agent hash with optional state filter and pagination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryJobsByEvaluatorRequest {
    pub evaluator_agent_hash: String,
    pub state: Option<JobState>,
    pub limit: u32,
    pub offset: u32,
}

impl QueryJobsByEvaluatorRequest {
    pub fn new(evaluator_agent_hash: impl Into<String>, limit: u32, offset: u32) -> Self {
        Self {
            evaluator_agent_hash: evaluator_agent_hash.into(),
            state: None,
            limit,
            offset,
        }
    }

    pub fn with_state(mut self, state: JobState) -> Self {
        self.state = Some(state);
        self
    }
}

impl From<QueryJobsByEvaluatorRequest> for proto::QueryJobsByEvaluatorRequest {
    fn from(req: QueryJobsByEvaluatorRequest) -> Self {
        Self {
            evaluator_agent_hash: req.evaluator_agent_hash,
            state: req.state.map(i32::from).unwrap_or(0),
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query currently active jobs with optional client/provider filter.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryActiveJobsRequest {
    pub client_agent_hash: Option<String>,
    pub provider_agent_hash: Option<String>,
    pub limit: u32,
    pub offset: u32,
}

impl QueryActiveJobsRequest {
    pub fn new(limit: u32, offset: u32) -> Self {
        Self {
            client_agent_hash: None,
            provider_agent_hash: None,
            limit,
            offset,
        }
    }

    pub fn with_client(mut self, client_agent_hash: impl Into<String>) -> Self {
        self.client_agent_hash = Some(client_agent_hash.into());
        self
    }

    pub fn with_provider(mut self, provider_agent_hash: impl Into<String>) -> Self {
        self.provider_agent_hash = Some(provider_agent_hash.into());
        self
    }
}

impl From<QueryActiveJobsRequest> for proto::QueryActiveJobsRequest {
    fn from(req: QueryActiveJobsRequest) -> Self {
        Self {
            client_agent_hash: req.client_agent_hash.unwrap_or_default(),
            provider_agent_hash: req.provider_agent_hash.unwrap_or_default(),
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query jobs filtered by state.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryJobsByStateRequest {
    pub state: JobState,
    pub limit: u32,
    pub offset: u32,
}

impl QueryJobsByStateRequest {
    pub fn new(state: JobState, limit: u32, offset: u32) -> Self {
        Self { state, limit, offset }
    }
}

impl From<QueryJobsByStateRequest> for proto::QueryJobsByStateRequest {
    fn from(req: QueryJobsByStateRequest) -> Self {
        Self {
            state: i32::from(req.state),
            limit: req.limit,
            offset: req.offset,
        }
    }
}

/// Query module parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsRequest;

impl From<QueryParamsRequest> for proto::QueryParamsRequest {
    fn from(_req: QueryParamsRequest) -> Self {
        Self {}
    }
}

// ====================== RESPONSE WRAPPERS ======================

/// Response for paginated job list queries.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JobListResponse {
    pub jobs: Vec<Job>,
    pub total_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn create_job_request_to_any() {
        let req = CreateJobRequest::new(Job::default(), vec![1, 2, 3]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/job.v1.MsgCreateJob");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn fund_job_request_to_any() {
        let req = FundJobRequest::new("job-1", 500, vec![4, 5, 6]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/job.v1.MsgFundJob");
    }

    #[test]
    fn attest_request_with_reason() {
        let req = AttestRequest::new("job-1", true, vec![7, 8])
            .with_reason_hash("reason-abc");
        assert_eq!(req.reason_hash, "reason-abc");
        let any = req.to_any();
        assert_eq!(any.type_url, "/job.v1.MsgAttest");
    }

    #[test]
    fn cancel_job_request_to_any() {
        let req = CancelJobRequest::new("job-1", vec![9, 10]);
        let any = req.to_any();
        assert_eq!(any.type_url, "/job.v1.MsgCancelJob");
    }

    #[test]
    fn query_jobs_by_client_with_state() {
        let req = QueryJobsByClientRequest::new("client-hash", 10, 0)
            .with_state(JobState::Funded);
        let proto_req: proto::QueryJobsByClientRequest = req.into();
        assert_eq!(proto_req.state, proto::JobState::Funded as i32);
    }

    #[test]
    fn query_active_jobs_with_filters() {
        let req = QueryActiveJobsRequest::new(20, 0)
            .with_client("client-1")
            .with_provider("provider-1");
        let proto_req: proto::QueryActiveJobsRequest = req.into();
        assert_eq!(proto_req.client_agent_hash, "client-1");
        assert_eq!(proto_req.provider_agent_hash, "provider-1");
    }
}
