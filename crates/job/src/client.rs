//! JobClient — the main entry point for all job-related operations
//! in the Morpheum SDK.
//!
//! Provides high-level, type-safe methods for querying jobs, active jobs,
//! jobs by role (client/provider/evaluator), jobs by state, and module parameters.
//! Transaction operations (create, fund, submit, attest, cancel, etc.) are handled
//! via the fluent builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

use crate::requests::{
    QueryActiveJobsRequest, QueryJobRequest, QueryJobsByClientRequest,
    QueryJobsByEvaluatorRequest, QueryJobsByProviderRequest,
    QueryJobsByStateRequest, QueryParamsRequest,
};
use crate::types::{Job, JobParams, JobState};

/// Primary client for all job-related queries.
///
/// Transaction construction (create, fund, submit, attest, cancel, set provider)
/// is delegated to the fluent builders in `builder.rs` for maximum ergonomics
/// and type safety.
pub struct JobClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl JobClient {
    /// Creates a new `JobClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a single job by its ID.
    pub async fn query_job(
        &self,
        job_id: impl Into<String>,
    ) -> Result<Option<Job>, SdkError> {
        let req = QueryJobRequest::new(job_id);
        let proto_req: morpheum_proto::job::v1::QueryJobRequest = req.into();

        let path = "/job.v1.Query/QueryJob";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::job::v1::QueryJobResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        if proto_res.found {
            Ok(proto_res.job.map(Into::into))
        } else {
            Ok(None)
        }
    }

    /// Queries jobs belonging to a specific client agent.
    pub async fn query_jobs_by_client(
        &self,
        client_agent_hash: impl Into<String>,
        state: Option<JobState>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Job>, SdkError> {
        let mut req = QueryJobsByClientRequest::new(client_agent_hash, limit, offset);
        if let Some(s) = state {
            req = req.with_state(s);
        }
        let proto_req: morpheum_proto::job::v1::QueryJobsByClientRequest = req.into();

        let path = "/job.v1.Query/QueryJobsByClient";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::job::v1::QueryJobsByClientResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.job.into_iter().map(Into::into).collect())
    }

    /// Queries jobs belonging to a specific provider agent.
    pub async fn query_jobs_by_provider(
        &self,
        provider_agent_hash: impl Into<String>,
        state: Option<JobState>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Job>, SdkError> {
        let mut req = QueryJobsByProviderRequest::new(provider_agent_hash, limit, offset);
        if let Some(s) = state {
            req = req.with_state(s);
        }
        let proto_req: morpheum_proto::job::v1::QueryJobsByProviderRequest = req.into();

        let path = "/job.v1.Query/QueryJobsByProvider";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::job::v1::QueryJobsByProviderResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.job.into_iter().map(Into::into).collect())
    }

    /// Queries jobs belonging to a specific evaluator agent.
    pub async fn query_jobs_by_evaluator(
        &self,
        evaluator_agent_hash: impl Into<String>,
        state: Option<JobState>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Job>, SdkError> {
        let mut req = QueryJobsByEvaluatorRequest::new(evaluator_agent_hash, limit, offset);
        if let Some(s) = state {
            req = req.with_state(s);
        }
        let proto_req: morpheum_proto::job::v1::QueryJobsByEvaluatorRequest = req.into();

        let path = "/job.v1.Query/QueryJobsByEvaluator";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::job::v1::QueryJobsByEvaluatorResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.job.into_iter().map(Into::into).collect())
    }

    /// Queries currently active jobs with optional client/provider filter.
    pub async fn query_active_jobs(
        &self,
        client_filter: Option<String>,
        provider_filter: Option<String>,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Job>, SdkError> {
        let mut req = QueryActiveJobsRequest::new(limit, offset);
        if let Some(c) = client_filter {
            req = req.with_client(c);
        }
        if let Some(p) = provider_filter {
            req = req.with_provider(p);
        }
        let proto_req: morpheum_proto::job::v1::QueryActiveJobsRequest = req.into();

        let path = "/job.v1.Query/QueryActiveJobs";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::job::v1::QueryActiveJobsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.job.into_iter().map(Into::into).collect())
    }

    /// Queries jobs filtered by state with pagination.
    pub async fn query_jobs_by_state(
        &self,
        state: JobState,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Job>, SdkError> {
        let req = QueryJobsByStateRequest::new(state, limit, offset);
        let proto_req: morpheum_proto::job::v1::QueryJobsByStateRequest = req.into();

        let path = "/job.v1.Query/QueryJobsByState";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::job::v1::QueryJobsByStateResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.job.into_iter().map(Into::into).collect())
    }

    /// Queries module parameters.
    pub async fn query_params(&self) -> Result<JobParams, SdkError> {
        let req = QueryParamsRequest;
        let proto_req: morpheum_proto::job::v1::QueryParamsRequest = req.into();

        let path = "/job.v1.Query/QueryParams";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::job::v1::QueryParamsResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .params
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("params field missing in response"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for JobClient {
    fn config(&self) -> &SdkConfig {
        &self.config
    }

    fn transport(&self) -> &dyn Transport {
        &*self.transport
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use morpheum_sdk_core::SdkConfig;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(
            &self,
            _tx_bytes: Vec<u8>,
        ) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!("not needed for job query tests")
        }

        async fn query(
            &self,
            path: &str,
            _data: Vec<u8>,
        ) -> Result<Vec<u8>, SdkError> {
            match path {
                "/job.v1.Query/QueryJob" => {
                    let dummy = morpheum_proto::job::v1::QueryJobResponse {
                        job: Some(Default::default()),
                        found: true,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/job.v1.Query/QueryJobsByClient"
                | "/job.v1.Query/QueryJobsByProvider"
                | "/job.v1.Query/QueryJobsByEvaluator"
                | "/job.v1.Query/QueryActiveJobs"
                | "/job.v1.Query/QueryJobsByState" => {
                    let dummy = morpheum_proto::job::v1::QueryJobsByClientResponse {
                        job: vec![],
                        total_count: 0,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/job.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::job::v1::QueryParamsResponse {
                        params: Some(Default::default()),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn job_client_query_job_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = JobClient::new(config, Box::new(DummyTransport));

        let result = client.query_job("job-123").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }

    #[tokio::test]
    async fn job_client_query_jobs_by_client_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = JobClient::new(config, Box::new(DummyTransport));

        let result = client.query_jobs_by_client("client-abc", None, 10, 0).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn job_client_query_active_jobs_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = JobClient::new(config, Box::new(DummyTransport));

        let result = client.query_active_jobs(None, None, 20, 0).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn job_client_query_params_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = JobClient::new(config, Box::new(DummyTransport));

        let result = client.query_params().await;
        assert!(result.is_ok());
    }
}
