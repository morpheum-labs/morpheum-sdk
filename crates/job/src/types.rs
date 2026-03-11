//! Domain types for the Job module (ERC-8183 agentic commerce).
//!
//! Clean, idiomatic Rust representations of the job protobuf messages.
//! Provides type safety, ergonomic APIs, and full round-trip conversion
//! to/from protobuf while remaining strictly `no_std` compatible.

use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::job::v1 as proto;

// ====================== ENUMS ======================

/// Job state machine (exact ERC-8183 mapping + CANCELLED for agent control).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum JobState {
    #[default]
    Open,
    Funded,
    Submitted,
    Completed,
    Rejected,
    Expired,
    Cancelled,
}

impl From<i32> for JobState {
    fn from(v: i32) -> Self {
        match proto::JobState::try_from(v).unwrap_or(proto::JobState::Open) {
            proto::JobState::Open => Self::Open,
            proto::JobState::Funded => Self::Funded,
            proto::JobState::Submitted => Self::Submitted,
            proto::JobState::Completed => Self::Completed,
            proto::JobState::Rejected => Self::Rejected,
            proto::JobState::Expired => Self::Expired,
            proto::JobState::Cancelled => Self::Cancelled,
        }
    }
}

impl From<JobState> for i32 {
    fn from(s: JobState) -> Self {
        match s {
            JobState::Open => proto::JobState::Open as i32,
            JobState::Funded => proto::JobState::Funded as i32,
            JobState::Submitted => proto::JobState::Submitted as i32,
            JobState::Completed => proto::JobState::Completed as i32,
            JobState::Rejected => proto::JobState::Rejected as i32,
            JobState::Expired => proto::JobState::Expired as i32,
            JobState::Cancelled => proto::JobState::Cancelled as i32,
        }
    }
}

impl JobState {
    /// Whether the job is in a terminal state (no further transitions possible).
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Rejected | Self::Expired | Self::Cancelled)
    }

    /// Whether the job is actively in progress (non-terminal and past creation).
    pub fn is_active(self) -> bool {
        matches!(self, Self::Open | Self::Funded | Self::Submitted)
    }
}

// ====================== STRUCTS ======================

/// Revenue share configuration (basis points).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RevenueShareConfig {
    pub creator_cut_bps: u32,
    pub provider_cut_bps: u32,
    pub evaluator_cut_bps: u32,
    pub platform_cut_bps: u32,
}

impl From<proto::RevenueShareConfig> for RevenueShareConfig {
    fn from(p: proto::RevenueShareConfig) -> Self {
        Self {
            creator_cut_bps: p.creator_cut_bps,
            provider_cut_bps: p.provider_cut_bps,
            evaluator_cut_bps: p.evaluator_cut_bps,
            platform_cut_bps: p.platform_cut_bps,
        }
    }
}

impl From<RevenueShareConfig> for proto::RevenueShareConfig {
    fn from(r: RevenueShareConfig) -> Self {
        Self {
            creator_cut_bps: r.creator_cut_bps,
            provider_cut_bps: r.provider_cut_bps,
            evaluator_cut_bps: r.evaluator_cut_bps,
            platform_cut_bps: r.platform_cut_bps,
        }
    }
}

/// Deliverable submitted by provider (Persistent Memory native).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Deliverable {
    pub job_id: String,
    pub provider_agent_hash: String,
    pub memory_root_hash: String,
    pub payload: Vec<u8>,
    pub blob_merkle_root: Vec<u8>,
    pub submitted_at: u64,
}

impl From<proto::Deliverable> for Deliverable {
    fn from(p: proto::Deliverable) -> Self {
        Self {
            job_id: p.job_id,
            provider_agent_hash: p.provider_agent_hash,
            memory_root_hash: p.memory_root_hash,
            payload: p.payload,
            blob_merkle_root: p.blob_merkle_root,
            submitted_at: p.submitted_at,
        }
    }
}

impl From<Deliverable> for proto::Deliverable {
    fn from(d: Deliverable) -> Self {
        Self {
            job_id: d.job_id,
            provider_agent_hash: d.provider_agent_hash,
            memory_root_hash: d.memory_root_hash,
            payload: d.payload,
            blob_merkle_root: d.blob_merkle_root,
            submitted_at: d.submitted_at,
        }
    }
}

/// Main Job entity (ERC-8183 Job primitive + Morpheum agent-native extensions).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Job {
    pub job_id: String,
    pub client_agent_hash: String,
    pub provider_agent_hash: String,
    pub evaluator_agent_hash: String,
    pub budget_usd: u64,
    pub deliverable: Option<Deliverable>,
    pub state: JobState,
    pub created_at: u64,
    pub expiry_timestamp: u64,
    pub hook_address: String,
    pub vc_proof_hash: String,
    pub revenue_share_config: Option<RevenueShareConfig>,
    pub metadata_payload: Vec<u8>,
    pub blob_merkle_root: Vec<u8>,
    pub job_spec_hash: String,
    pub rejection_reason_hash: String,
}

impl Job {
    /// Whether the job is in a terminal state.
    pub fn is_terminal(&self) -> bool {
        self.state.is_terminal()
    }

    /// Whether the job is actively in progress.
    pub fn is_active(&self) -> bool {
        self.state.is_active()
    }

    /// Whether a provider has been assigned.
    pub fn has_provider(&self) -> bool {
        !self.provider_agent_hash.is_empty()
    }

    /// Whether a deliverable has been submitted.
    pub fn has_deliverable(&self) -> bool {
        self.deliverable.is_some()
    }
}

impl From<proto::Job> for Job {
    fn from(p: proto::Job) -> Self {
        Self {
            job_id: p.job_id,
            client_agent_hash: p.client_agent_hash,
            provider_agent_hash: p.provider_agent_hash,
            evaluator_agent_hash: p.evaluator_agent_hash,
            budget_usd: p.budget_usd,
            deliverable: p.deliverable.map(Into::into),
            state: JobState::from(p.state),
            created_at: p.created_at,
            expiry_timestamp: p.expiry_timestamp,
            hook_address: p.hook_address,
            vc_proof_hash: p.vc_proof_hash,
            revenue_share_config: p.revenue_share_config.map(Into::into),
            metadata_payload: p.metadata_payload,
            blob_merkle_root: p.blob_merkle_root,
            job_spec_hash: p.job_spec_hash,
            rejection_reason_hash: p.rejection_reason_hash,
        }
    }
}

impl From<Job> for proto::Job {
    fn from(j: Job) -> Self {
        Self {
            job_id: j.job_id,
            client_agent_hash: j.client_agent_hash,
            provider_agent_hash: j.provider_agent_hash,
            evaluator_agent_hash: j.evaluator_agent_hash,
            budget_usd: j.budget_usd,
            deliverable: j.deliverable.map(Into::into),
            state: i32::from(j.state),
            created_at: j.created_at,
            expiry_timestamp: j.expiry_timestamp,
            hook_address: j.hook_address,
            vc_proof_hash: j.vc_proof_hash,
            revenue_share_config: j.revenue_share_config.map(Into::into),
            metadata_payload: j.metadata_payload,
            blob_merkle_root: j.blob_merkle_root,
            job_spec_hash: j.job_spec_hash,
            rejection_reason_hash: j.rejection_reason_hash,
        }
    }
}

/// Attestation / evaluation result submitted by evaluator.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JobAttestation {
    pub job_id: String,
    pub evaluator_agent_hash: String,
    pub completed: bool,
    pub reason_hash: String,
    pub attested_at: u64,
    pub detailed_report: Vec<u8>,
    pub detailed_report_blob_merkle_root: Vec<u8>,
}

impl From<proto::JobAttestation> for JobAttestation {
    fn from(p: proto::JobAttestation) -> Self {
        Self {
            job_id: p.job_id,
            evaluator_agent_hash: p.evaluator_agent_hash,
            completed: p.completed,
            reason_hash: p.reason_hash,
            attested_at: p.attested_at,
            detailed_report: p.detailed_report,
            detailed_report_blob_merkle_root: p.detailed_report_blob_merkle_root,
        }
    }
}

impl From<JobAttestation> for proto::JobAttestation {
    fn from(a: JobAttestation) -> Self {
        Self {
            job_id: a.job_id,
            evaluator_agent_hash: a.evaluator_agent_hash,
            completed: a.completed,
            reason_hash: a.reason_hash,
            attested_at: a.attested_at,
            detailed_report: a.detailed_report,
            detailed_report_blob_merkle_root: a.detailed_report_blob_merkle_root,
        }
    }
}

/// Module parameters (configurable via governance or genesis).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct JobParams {
    pub default_platform_cut_bps: u32,
    pub default_escrow_timeout_seconds: u64,
    pub min_reputation_to_create: u64,
    pub min_reputation_to_provide: u64,
    pub job_enabled: bool,
    pub max_active_job_per_client: u32,
    pub max_active_job_per_provider: u32,
    pub default_evaluation_fee_usd: u64,
    pub declarative_job_enabled: bool,
}

impl Default for JobParams {
    fn default() -> Self {
        Self {
            default_platform_cut_bps: 250,
            default_escrow_timeout_seconds: 86_400,
            min_reputation_to_create: 0,
            min_reputation_to_provide: 0,
            job_enabled: true,
            max_active_job_per_client: 100,
            max_active_job_per_provider: 50,
            default_evaluation_fee_usd: 0,
            declarative_job_enabled: false,
        }
    }
}

impl From<proto::Params> for JobParams {
    fn from(p: proto::Params) -> Self {
        Self {
            default_platform_cut_bps: p.default_platform_cut_bps,
            default_escrow_timeout_seconds: p.default_escrow_timeout_seconds,
            min_reputation_to_create: p.min_reputation_to_create,
            min_reputation_to_provide: p.min_reputation_to_provide,
            job_enabled: p.job_enabled,
            max_active_job_per_client: p.max_active_job_per_client,
            max_active_job_per_provider: p.max_active_job_per_provider,
            default_evaluation_fee_usd: p.default_evaluation_fee_usd,
            declarative_job_enabled: p.declarative_job_enabled,
        }
    }
}

impl From<JobParams> for proto::Params {
    fn from(p: JobParams) -> Self {
        Self {
            default_platform_cut_bps: p.default_platform_cut_bps,
            default_escrow_timeout_seconds: p.default_escrow_timeout_seconds,
            min_reputation_to_create: p.min_reputation_to_create,
            min_reputation_to_provide: p.min_reputation_to_provide,
            job_enabled: p.job_enabled,
            max_active_job_per_client: p.max_active_job_per_client,
            max_active_job_per_provider: p.max_active_job_per_provider,
            default_evaluation_fee_usd: p.default_evaluation_fee_usd,
            declarative_job_enabled: p.declarative_job_enabled,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn job_state_terminal() {
        assert!(!JobState::Open.is_terminal());
        assert!(!JobState::Funded.is_terminal());
        assert!(!JobState::Submitted.is_terminal());
        assert!(JobState::Completed.is_terminal());
        assert!(JobState::Rejected.is_terminal());
        assert!(JobState::Expired.is_terminal());
        assert!(JobState::Cancelled.is_terminal());
    }

    #[test]
    fn job_state_active() {
        assert!(JobState::Open.is_active());
        assert!(JobState::Funded.is_active());
        assert!(JobState::Submitted.is_active());
        assert!(!JobState::Completed.is_active());
    }

    #[test]
    fn job_roundtrip() {
        let job = Job {
            job_id: "job-123".into(),
            client_agent_hash: "client-abc".into(),
            provider_agent_hash: "provider-xyz".into(),
            evaluator_agent_hash: "eval-007".into(),
            budget_usd: 1000,
            deliverable: Some(Deliverable {
                job_id: "job-123".into(),
                provider_agent_hash: "provider-xyz".into(),
                memory_root_hash: "mem-root".into(),
                payload: vec![1, 2, 3],
                blob_merkle_root: vec![4, 5, 6],
                submitted_at: 1_700_000_000,
            }),
            state: JobState::Submitted,
            created_at: 1_699_000_000,
            expiry_timestamp: 1_700_100_000,
            hook_address: "hook-addr".into(),
            vc_proof_hash: "vc-hash".into(),
            revenue_share_config: Some(RevenueShareConfig {
                creator_cut_bps: 1000,
                provider_cut_bps: 7000,
                evaluator_cut_bps: 1000,
                platform_cut_bps: 1000,
            }),
            metadata_payload: vec![7, 8],
            blob_merkle_root: vec![9, 10],
            job_spec_hash: "spec-hash".into(),
            rejection_reason_hash: String::new(),
        };

        let proto_job: proto::Job = job.clone().into();
        let back: Job = proto_job.into();
        assert_eq!(job, back);
    }

    #[test]
    fn job_params_roundtrip() {
        let params = JobParams::default();
        let proto_params: proto::Params = params.clone().into();
        let back: JobParams = proto_params.into();
        assert_eq!(params, back);
    }

    #[test]
    fn attestation_roundtrip() {
        let att = JobAttestation {
            job_id: "job-123".into(),
            evaluator_agent_hash: "eval-007".into(),
            completed: true,
            reason_hash: "reason".into(),
            attested_at: 1_700_050_000,
            detailed_report: vec![11, 12],
            detailed_report_blob_merkle_root: vec![13, 14],
        };

        let proto_att: proto::JobAttestation = att.clone().into();
        let back: JobAttestation = proto_att.into();
        assert_eq!(att, back);
    }
}
