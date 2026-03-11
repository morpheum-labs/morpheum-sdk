//! Job module for the Morpheum SDK (ERC-8183 agentic commerce).
//!
//! This module provides full support for the ERC-8183 job lifecycle on Morpheum,
//! including creation, funding, deliverable submission, attestation, cancellation,
//! refund claims, provider assignment, and rich querying of job data.
//!
//! It integrates seamlessly with the agent identity, reputation, and bank modules
//! for end-to-end agentic commerce workflows.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod client;
pub mod types;
pub mod requests;
pub mod builder;

// ==================== PUBLIC RE-EXPORTS ====================

/// Main client for all job query operations.
pub use client::JobClient;

/// Core domain types for jobs.
pub use types::{
    Job,
    JobState,
    JobParams,
    JobAttestation,
    Deliverable,
    RevenueShareConfig,
};

/// Request and response wrappers for transaction construction and queries.
pub use requests::*;

/// Fluent builders for job lifecycle operations.
pub use builder::{
    CreateJobBuilder,
    FundJobBuilder,
    SubmitDeliverableBuilder,
    AttestBuilder,
    ClaimRefundBuilder,
    SetProviderBuilder,
    CancelJobBuilder,
};

// Re-export core SDK types commonly used with job flows.
pub use morpheum_sdk_core::{
    AccountId,
    ChainId,
    SdkError,
    SignedTx,
};

/// Recommended prelude for the job module.
///
/// Most users should start with:
/// ```rust
/// use morpheum_sdk_job::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        JobClient,
        Job,
        JobState,
        JobParams,
        JobAttestation,
        Deliverable,
        RevenueShareConfig,
        CreateJobBuilder,
        FundJobBuilder,
        SubmitDeliverableBuilder,
        AttestBuilder,
        AccountId,
        ChainId,
        SdkError,
        SignedTx,
    };
}

/// Current version of the job module (synchronized with workspace version).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn public_api_compiles_cleanly() {
        #[allow(unused_imports)]
        use prelude::*;
        let _ = VERSION;
    }
}
