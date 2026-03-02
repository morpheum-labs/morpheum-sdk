//! VcClient — the main entry point for all Verifiable Credential operations
//! in the Morpheum SDK.
//!
//! This client provides high-level, type-safe methods for querying VCs,
//! their status, revocation bitmaps, and parameters. Transaction operations
//! (issue, revoke, self-revoke, update claims) are handled via the fluent
//! builders in `builder.rs` + `TxBuilder`.

use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{
    AccountId, MorpheumClient, SdkConfig, SdkError, Transport,
};

use crate::{
    requests::{
        QueryVcRequest,
        QueryVcStatusRequest,
        QueryVcsByIssuerRequest,
        QueryVcsBySubjectRequest,
        QueryRevocationBitmapRequest,
        QueryParamsRequest,
    },
    types::{Vc, VcStatus, Params},
};

/// Primary client for all VC-related operations.
///
/// Focused on queries. Transaction construction is delegated to the
/// fluent builders (`VcIssueBuilder`, `VcRevokeBuilder`, etc.) for maximum
/// ergonomics and type safety.
pub struct VcClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl VcClient {
    /// Creates a new `VcClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a specific Verifiable Credential by its ID.
    pub async fn query_vc(&self, vc_id: impl Into<String>) -> Result<Vc, SdkError> {
        let req = QueryVcRequest::new(vc_id.into());
        let proto_req: morpheum_proto::vc::v1::QueryVcRequest = req.into();

        let path = "/vc.v1.Query/QueryVc";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::vc::v1::QueryVcResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        proto_res
            .vc
            .map(Into::into)
            .ok_or_else(|| SdkError::transport("vc field missing in response"))
    }

    /// Queries the current status of a VC (valid, revoked, expired, etc.).
    pub async fn query_vc_status(
        &self,
        vc_id: impl Into<String>,
    ) -> Result<VcStatus, SdkError> {
        let req = QueryVcStatusRequest::new(vc_id.into());
        let proto_req: morpheum_proto::vc::v1::QueryVcStatusRequest = req.into();

        let path = "/vc.v1.Query/QueryVcStatus";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::vc::v1::QueryVcStatusResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(VcStatus {
            vc_id: proto_res.vc_id,
            is_valid: proto_res.is_valid,
            is_revoked: proto_res.is_revoked,
            is_expired: proto_res.is_expired,
            revoked_at: proto_res.revoked_at,
        })
    }

    /// Queries all VCs issued by a specific issuer (paginated).
    pub async fn query_vcs_by_issuer(
        &self,
        issuer: AccountId,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Vc>, SdkError> {
        let req = QueryVcsByIssuerRequest {
            issuer,
            limit,
            offset,
        };
        let proto_req: morpheum_proto::vc::v1::QueryVcsByIssuerRequest = req.into();

        let path = "/vc.v1.Query/QueryVcsByIssuer";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::vc::v1::QueryVcsByIssuerResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.vcs.into_iter().map(Into::into).collect())
    }

    /// Queries all VCs issued to a specific subject (paginated).
    pub async fn query_vcs_by_subject(
        &self,
        subject: AccountId,
        limit: u32,
        offset: u32,
    ) -> Result<Vec<Vc>, SdkError> {
        let req = QueryVcsBySubjectRequest {
            subject,
            limit,
            offset,
        };
        let proto_req: morpheum_proto::vc::v1::QueryVcsBySubjectRequest = req.into();

        let path = "/vc.v1.Query/QueryVcsBySubject";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::vc::v1::QueryVcsBySubjectResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.vcs.into_iter().map(Into::into).collect())
    }

    /// Queries the revocation bitmap for an issuer (for cross-chain verification).
    pub async fn query_revocation_bitmap(
        &self,
        issuer: AccountId,
    ) -> Result<Vec<u8>, SdkError> {
        let req = QueryRevocationBitmapRequest::new(issuer);
        let proto_req: morpheum_proto::vc::v1::QueryRevocationBitmapRequest = req.into();

        let path = "/vc.v1.Query/QueryRevocationBitmap";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::vc::v1::QueryRevocationBitmapResponse::decode(
            response_bytes.as_slice(),
        )
        .map_err(SdkError::Decode)?;

        Ok(proto_res.bitmap)
    }

    /// Queries the current VC module parameters.
    pub async fn query_params(&self) -> Result<Params, SdkError> {
        let req = QueryParamsRequest {};
        let proto_req: morpheum_proto::vc::v1::QueryParamsRequest = req.into();

        let path = "/vc.v1.Query/QueryParams";
        let data = proto_req.encode_to_vec();

        let response_bytes = self.query(path, data).await?;

        let proto_res = morpheum_proto::vc::v1::QueryParamsResponse::decode(
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
impl MorpheumClient for VcClient {
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
    use morpheum_sdk_core::SdkConfig;

    // Dummy transport for testing
    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _tx_bytes: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!("not needed for VC query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/vc.v1.Query/QueryVc" => {
                    let dummy = morpheum_proto::vc::v1::QueryVcResponse {
                        vc: Some(Default::default()),
                        is_valid: true,
                        is_revoked: false,
                        is_expired: false,
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/vc.v1.Query/QueryVcStatus" => {
                    let dummy = morpheum_proto::vc::v1::QueryVcStatusResponse {
                        vc_id: "vc_test".into(),
                        is_valid: true,
                        is_revoked: false,
                        is_expired: false,
                        revoked_at: 0,
                        reason: "".into(),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn vc_client_query_vc_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = VcClient::new(config, Box::new(DummyTransport));

        let result = client.query_vc("vc_test_001").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn vc_client_query_status_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = VcClient::new(config, Box::new(DummyTransport));

        let result = client.query_vc_status("vc_test_001").await;
        assert!(result.is_ok());
    }
}
