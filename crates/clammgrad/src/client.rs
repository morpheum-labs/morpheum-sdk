//! ClammGradClient — queries for CLAMM graduation state, eligibility, and params.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};
use morpheum_proto::clammgrad::v1 as proto;

use crate::requests;
use crate::types::{ClammGraduationParams, GraduationState};

/// Eligible-tokens query result with pagination cursor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EligibleTokensPage {
    pub token_indexes: Vec<String>,
    pub next_offset: u64,
}

/// Primary client for CLAMM Graduation queries.
pub struct ClammGradClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl ClammGradClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Fetches graduation state for a token.
    pub async fn get_graduation_state(
        &self, token_index: impl Into<String>,
    ) -> Result<GraduationState, SdkError> {
        let req = requests::GetGraduationStateRequest::new(token_index);
        let proto_req: proto::GetGraduationStateRequest = req.into();
        let resp = self.query(
            "/clammgrad.v1.ClammGraduationService/GetGraduationState",
            proto_req.encode_to_vec(),
        ).await?;
        let p = proto::GetGraduationStateResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.state.map(Into::into).ok_or_else(|| SdkError::transport("state field missing"))
    }

    /// Lists tokens eligible for graduation.
    pub async fn list_eligible_tokens(
        &self, request: requests::ListEligibleTokensRequest,
    ) -> Result<EligibleTokensPage, SdkError> {
        let proto_req: proto::ListEligibleTokensRequest = request.into();
        let resp = self.query(
            "/clammgrad.v1.ClammGraduationService/ListEligibleTokens",
            proto_req.encode_to_vec(),
        ).await?;
        let p = proto::ListEligibleTokensResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        Ok(EligibleTokensPage { token_indexes: p.token_indexes, next_offset: p.next_offset })
    }

    /// Queries module-level graduation parameters.
    pub async fn get_params(&self) -> Result<ClammGraduationParams, SdkError> {
        let proto_req: proto::GetParamsRequest = requests::GetParamsRequest::new().into();
        let resp = self.query(
            "/clammgrad.v1.ClammGraduationService/GetParams",
            proto_req.encode_to_vec(),
        ).await?;
        let p = proto::GetParamsResponse::decode(resp.as_slice()).map_err(SdkError::Decode)?;
        p.params.map(Into::into).ok_or_else(|| SdkError::transport("params field missing"))
    }
}

#[async_trait(?Send)]
impl MorpheumClient for ClammGradClient {
    fn config(&self) -> &SdkConfig { &self.config }
    fn transport(&self) -> &dyn Transport { &*self.transport }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    struct DummyTransport;

    #[async_trait(?Send)]
    impl Transport for DummyTransport {
        async fn broadcast_tx(&self, _: Vec<u8>) -> Result<morpheum_sdk_core::BroadcastResult, SdkError> {
            unimplemented!()
        }
        async fn query(&self, path: &str, _: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/clammgrad.v1.ClammGraduationService/GetGraduationState" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetGraduationStateResponse {
                        state: Some(proto::GraduationState {
                            token_index: "42".into(), status: 1,
                            ..Default::default()
                        }),
                    }))
                }
                "/clammgrad.v1.ClammGraduationService/ListEligibleTokens" => {
                    Ok(prost::Message::encode_to_vec(&proto::ListEligibleTokensResponse {
                        token_indexes: vec!["42".into(), "99".into()], next_offset: 2,
                    }))
                }
                "/clammgrad.v1.ClammGraduationService/GetParams" => {
                    Ok(prost::Message::encode_to_vec(&proto::GetParamsResponse {
                        params: Some(proto::ClammGraduationParams::default()),
                    }))
                }
                _ => Err(SdkError::transport("unexpected path")),
            }
        }
    }

    fn make_client() -> ClammGradClient {
        ClammGradClient::new(SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1"), Box::new(DummyTransport))
    }

    #[tokio::test]
    async fn get_graduation_state_works() {
        let state = make_client().get_graduation_state("42").await.unwrap();
        assert_eq!(state.token_index, "42");
        assert_eq!(state.status, crate::types::GraduationStatus::Pending);
    }

    #[tokio::test]
    async fn list_eligible_works() {
        let page = make_client().list_eligible_tokens(requests::ListEligibleTokensRequest::new(10)).await.unwrap();
        assert_eq!(page.token_indexes.len(), 2);
        assert_eq!(page.next_offset, 2);
    }

    #[tokio::test]
    async fn get_params_works() {
        let params = make_client().get_params().await;
        assert!(params.is_ok());
    }
}
