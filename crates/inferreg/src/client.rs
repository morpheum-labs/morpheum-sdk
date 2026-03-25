//! InferenceRegistryClient — the main entry point for inferreg
//! queries in the Morpheum SDK.
//!
//! Provides high-level, type-safe methods for querying model commitments,
//! listing models by quantization format, enumerating active models, and
//! reading module parameters. Transaction operations (register model, update
//! params) are handled via the fluent builders in `builder.rs` + `TxBuilder`.

use alloc::boxed::Box;
use alloc::vec::Vec;

use async_trait::async_trait;
use prost::Message as _;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

use crate::{
    requests::{QueryModelRequest, QueryModelsByQuantRequest},
    types::{ModelCommitment, Params, QuantFormat},
};

/// Primary client for all inferreg queries.
///
/// Transaction construction (register model, update params) is delegated to
/// the fluent builders in `builder.rs` for maximum ergonomics and type safety.
pub struct InferenceRegistryClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl InferenceRegistryClient {
    /// Creates a new `InferenceRegistryClient` with the given configuration and transport.
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a single model commitment by its model_id.
    ///
    /// Returns `None` if the model is not found.
    pub async fn query_model(
        &self,
        model_id: impl Into<alloc::string::String>,
    ) -> Result<Option<ModelCommitment>, SdkError> {
        let req = QueryModelRequest::new(model_id);
        let proto_req: morpheum_proto::inferreg::v1::QueryModelRequest = req.into();

        let path = "/inferreg.v1.Query/QueryModel";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res =
            morpheum_proto::inferreg::v1::QueryModelResponse::decode(
                response_bytes.as_slice(),
            )
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryModelResponse = proto_res.into();
        Ok(response.model)
    }

    /// Queries models filtered by quantization format.
    pub async fn query_models_by_quant(
        &self,
        quant_format: QuantFormat,
    ) -> Result<Vec<ModelCommitment>, SdkError> {
        let req = QueryModelsByQuantRequest::new(quant_format);
        let proto_req: morpheum_proto::inferreg::v1::QueryModelsByQuantRequest =
            req.into();

        let path = "/inferreg.v1.Query/QueryModelsByQuant";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res =
            morpheum_proto::inferreg::v1::QueryModelsByQuantResponse::decode(
                response_bytes.as_slice(),
            )
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryModelsByQuantResponse = proto_res.into();
        Ok(response.models)
    }

    /// Queries all currently active models.
    pub async fn query_active_models(&self) -> Result<Vec<ModelCommitment>, SdkError> {
        let req = crate::requests::QueryActiveModelsRequest;
        let proto_req: morpheum_proto::inferreg::v1::QueryActiveModelsRequest =
            req.into();

        let path = "/inferreg.v1.Query/QueryActiveModels";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res =
            morpheum_proto::inferreg::v1::QueryActiveModelsResponse::decode(
                response_bytes.as_slice(),
            )
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryActiveModelsResponse = proto_res.into();
        Ok(response.models)
    }

    /// Queries the current module parameters.
    pub async fn query_params(&self) -> Result<Option<Params>, SdkError> {
        let req = crate::requests::QueryParamsRequest;
        let proto_req: morpheum_proto::inferreg::v1::QueryParamsRequest = req.into();

        let path = "/inferreg.v1.Query/QueryParams";
        let data = proto_req.encode_to_vec();
        let response_bytes = self.query(path, data).await?;

        let proto_res =
            morpheum_proto::inferreg::v1::QueryParamsResponse::decode(
                response_bytes.as_slice(),
            )
            .map_err(SdkError::Decode)?;

        let response: crate::requests::QueryParamsResponse = proto_res.into();
        Ok(response.params)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for InferenceRegistryClient {
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
            unimplemented!("not needed for inferreg query tests")
        }

        async fn query(&self, path: &str, _data: Vec<u8>) -> Result<Vec<u8>, SdkError> {
            match path {
                "/inferreg.v1.Query/QueryModel" => {
                    let dummy = morpheum_proto::inferreg::v1::QueryModelResponse {
                        model: Some(morpheum_proto::inferreg::v1::ModelCommitment {
                            model_id: vec![0xAA; 32],
                            display_name: "Llama-3.1-8B-Q4".into(),
                            quant_format: 1,
                            param_count: 8,
                            version: 1,
                            ..Default::default()
                        }),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/inferreg.v1.Query/QueryModelsByQuant" => {
                    let dummy =
                        morpheum_proto::inferreg::v1::QueryModelsByQuantResponse {
                            models: vec![Default::default()],
                        };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/inferreg.v1.Query/QueryActiveModels" => {
                    let dummy =
                        morpheum_proto::inferreg::v1::QueryActiveModelsResponse {
                            models: vec![Default::default(), Default::default()],
                        };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                "/inferreg.v1.Query/QueryParams" => {
                    let dummy = morpheum_proto::inferreg::v1::QueryParamsResponse {
                        params: Some(morpheum_proto::inferreg::v1::Params {
                            max_models: 1000,
                            max_param_count: 405,
                            enable_auto_proof_submission: true,
                            default_max_tokens: 4096,
                            governance_threshold: 1,
                        }),
                    };
                    Ok(prost::Message::encode_to_vec(&dummy))
                }
                _ => Err(SdkError::transport("unexpected query path in test")),
            }
        }
    }

    #[tokio::test]
    async fn query_model_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = InferenceRegistryClient::new(config, Box::new(DummyTransport));

        let model = client
            .query_model("aa".repeat(32))
            .await
            .unwrap()
            .expect("model should be present");
        assert_eq!(model.display_name, "Llama-3.1-8B-Q4");
        assert_eq!(model.quant_format, QuantFormat::Q4KM);
        assert_eq!(model.param_count, 8);
    }

    #[tokio::test]
    async fn query_models_by_quant_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = InferenceRegistryClient::new(config, Box::new(DummyTransport));

        let models = client
            .query_models_by_quant(QuantFormat::Q4KM)
            .await
            .unwrap();
        assert_eq!(models.len(), 1);
    }

    #[tokio::test]
    async fn query_active_models_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = InferenceRegistryClient::new(config, Box::new(DummyTransport));

        let models = client.query_active_models().await.unwrap();
        assert_eq!(models.len(), 2);
    }

    #[tokio::test]
    async fn query_params_works() {
        let config = SdkConfig::new("https://sentry.morpheum.xyz", "morpheum-test-1");
        let client = InferenceRegistryClient::new(config, Box::new(DummyTransport));

        let params = client
            .query_params()
            .await
            .unwrap()
            .expect("params should be present");
        assert_eq!(params.max_models, 1000);
        assert!(params.enable_auto_proof_submission);
        assert_eq!(params.default_max_tokens, 4096);
    }
}
