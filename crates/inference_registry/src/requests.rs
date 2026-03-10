//! Request and response wrappers for the Inference Registry module.
//!
//! Clean, ergonomic Rust types that wrap the raw protobuf messages. They
//! provide type safety, validation, helper methods, and seamless conversion
//! to/from protobuf for use with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;
use morpheum_proto::inference_registry::v1 as proto;

use crate::types::{ModelCommitment, Params, QuantFormat};

// ====================== TRANSACTION REQUESTS ======================

/// Request to register a new model (governance only).
///
/// Creates a `ModelCommitment`, registers the precompile in AgentCore VM,
/// and triggers export to `agent_registry` for instant visibility.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterModelRequest {
    pub authority: String,
    pub display_name: String,
    pub quant_format: QuantFormat,
    pub param_count: u64,
    pub zk_commitment: Vec<u8>,
    pub supported_ops: u64,
    pub version: u32,
    pub weights_payload: Vec<u8>,
}

impl RegisterModelRequest {
    pub fn new(
        authority: impl Into<String>,
        display_name: impl Into<String>,
        quant_format: QuantFormat,
        param_count: u64,
        zk_commitment: Vec<u8>,
        supported_ops: u64,
        version: u32,
    ) -> Self {
        Self {
            authority: authority.into(),
            display_name: display_name.into(),
            quant_format,
            param_count,
            zk_commitment,
            supported_ops,
            version,
            weights_payload: Vec::new(),
        }
    }

    /// Sets the optional raw model weights payload.
    pub fn with_weights_payload(mut self, payload: Vec<u8>) -> Self {
        self.weights_payload = payload;
        self
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgRegisterModel = self.clone().into();
        ProtoAny {
            type_url: "/inference_registry.v1.MsgRegisterModel".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<RegisterModelRequest> for proto::MsgRegisterModel {
    fn from(req: RegisterModelRequest) -> Self {
        Self {
            authority: req.authority,
            display_name: req.display_name,
            quant_format: req.quant_format.to_proto(),
            param_count: req.param_count,
            zk_commitment: req.zk_commitment,
            supported_ops: req.supported_ops,
            version: req.version,
            weights_payload: req.weights_payload,
        }
    }
}

/// Response from registering a model.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RegisterModelResponse {
    pub success: bool,
    pub model_id: Vec<u8>,
    pub display_name: String,
}

impl RegisterModelResponse {
    pub fn model_id_hex(&self) -> String {
        hex::encode(&self.model_id)
    }
}

impl From<proto::MsgRegisterModelResponse> for RegisterModelResponse {
    fn from(p: proto::MsgRegisterModelResponse) -> Self {
        Self {
            success: p.success,
            model_id: p.model_id,
            display_name: p.display_name,
        }
    }
}

/// Request to update module parameters (governance only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsRequest {
    pub authority: String,
    pub params: Params,
}

impl UpdateParamsRequest {
    pub fn new(authority: impl Into<String>, params: Params) -> Self {
        Self {
            authority: authority.into(),
            params,
        }
    }

    /// Converts this request into a protobuf `Any` ready for `TxBuilder::add_message`.
    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdateParams = self.clone().into();
        ProtoAny {
            type_url: "/inference_registry.v1.MsgUpdateParams".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdateParamsRequest> for proto::MsgUpdateParams {
    fn from(req: UpdateParamsRequest) -> Self {
        Self {
            authority: req.authority,
            params: Some(req.params.into()),
        }
    }
}

/// Response from updating parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateParamsResponse {
    pub success: bool,
}

impl From<proto::MsgUpdateParamsResponse> for UpdateParamsResponse {
    fn from(p: proto::MsgUpdateParamsResponse) -> Self {
        Self { success: p.success }
    }
}

// ====================== QUERY REQUESTS & RESPONSES ======================

/// Query a single model by its model_id.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryModelRequest {
    pub model_id: String,
}

impl QueryModelRequest {
    pub fn new(model_id: impl Into<String>) -> Self {
        Self {
            model_id: model_id.into(),
        }
    }
}

impl From<QueryModelRequest> for proto::QueryModelRequest {
    fn from(req: QueryModelRequest) -> Self {
        Self {
            model_id: req.model_id,
        }
    }
}

/// Response containing a model commitment (or `None` if not found).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryModelResponse {
    pub model: Option<ModelCommitment>,
}

impl From<proto::QueryModelResponse> for QueryModelResponse {
    fn from(p: proto::QueryModelResponse) -> Self {
        Self {
            model: p.model.map(Into::into),
        }
    }
}

/// Query models by quantization format.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryModelsByQuantRequest {
    pub quant_format: QuantFormat,
}

impl QueryModelsByQuantRequest {
    pub fn new(quant_format: QuantFormat) -> Self {
        Self { quant_format }
    }
}

impl From<QueryModelsByQuantRequest> for proto::QueryModelsByQuantRequest {
    fn from(req: QueryModelsByQuantRequest) -> Self {
        Self {
            quant_format: req.quant_format.to_proto(),
        }
    }
}

/// Response containing models matching the quant format filter.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryModelsByQuantResponse {
    pub models: Vec<ModelCommitment>,
}

impl From<proto::QueryModelsByQuantResponse> for QueryModelsByQuantResponse {
    fn from(p: proto::QueryModelsByQuantResponse) -> Self {
        Self {
            models: p.models.into_iter().map(Into::into).collect(),
        }
    }
}

/// Query all currently active models.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryActiveModelsRequest;

impl From<QueryActiveModelsRequest> for proto::QueryActiveModelsRequest {
    fn from(_: QueryActiveModelsRequest) -> Self {
        Self {}
    }
}

/// Response containing all active models.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryActiveModelsResponse {
    pub models: Vec<ModelCommitment>,
}

impl From<proto::QueryActiveModelsResponse> for QueryActiveModelsResponse {
    fn from(p: proto::QueryActiveModelsResponse) -> Self {
        Self {
            models: p.models.into_iter().map(Into::into).collect(),
        }
    }
}

/// Query the current module parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsRequest;

impl From<QueryParamsRequest> for proto::QueryParamsRequest {
    fn from(_: QueryParamsRequest) -> Self {
        Self {}
    }
}

/// Response containing the current module parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsResponse {
    pub params: Option<Params>,
}

impl From<proto::QueryParamsResponse> for QueryParamsResponse {
    fn from(p: proto::QueryParamsResponse) -> Self {
        Self {
            params: p.params.map(Into::into),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use crate::types::ops;

    #[test]
    fn register_model_to_any() {
        let req = RegisterModelRequest::new(
            "morpheum1gov",
            "Llama-3.1-8B-Q4",
            QuantFormat::Q4KM,
            8,
            vec![0xBB; 32],
            ops::INFER | ops::EMBED,
            1,
        );
        let any = req.to_any();
        assert_eq!(any.type_url, "/inference_registry.v1.MsgRegisterModel");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn register_model_with_weights() {
        let req = RegisterModelRequest::new(
            "morpheum1gov",
            "Test",
            QuantFormat::Fp16,
            70,
            vec![0; 32],
            ops::INFER,
            1,
        )
        .with_weights_payload(vec![1, 2, 3, 4]);
        assert_eq!(req.weights_payload, vec![1, 2, 3, 4]);
    }

    #[test]
    fn update_params_to_any() {
        let req = UpdateParamsRequest::new("morpheum1gov", Params::default());
        let any = req.to_any();
        assert_eq!(any.type_url, "/inference_registry.v1.MsgUpdateParams");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn register_model_response_conversion() {
        let proto_res = proto::MsgRegisterModelResponse {
            success: true,
            model_id: vec![0xAA; 32],
            display_name: "Llama-3.1-8B-Q4".into(),
        };
        let res: RegisterModelResponse = proto_res.into();
        assert!(res.success);
        assert_eq!(res.model_id_hex(), "aa".repeat(32));
    }

    #[test]
    fn query_model_response_conversion() {
        let proto_res = proto::QueryModelResponse {
            model: Some(proto::ModelCommitment {
                model_id: vec![0xAA; 32],
                display_name: "Llama".into(),
                quant_format: 1,
                param_count: 8,
                version: 1,
                ..Default::default()
            }),
        };
        let res: QueryModelResponse = proto_res.into();
        let model = res.model.unwrap();
        assert_eq!(model.quant_format, QuantFormat::Q4KM);
        assert_eq!(model.param_count, 8);
    }

    #[test]
    fn query_models_by_quant_response_conversion() {
        let proto_res = proto::QueryModelsByQuantResponse {
            models: vec![Default::default(), Default::default()],
        };
        let res: QueryModelsByQuantResponse = proto_res.into();
        assert_eq!(res.models.len(), 2);
    }

    #[test]
    fn query_active_models_response_conversion() {
        let proto_res = proto::QueryActiveModelsResponse {
            models: vec![Default::default()],
        };
        let res: QueryActiveModelsResponse = proto_res.into();
        assert_eq!(res.models.len(), 1);
    }

    #[test]
    fn query_params_response_conversion() {
        let proto_res = proto::QueryParamsResponse {
            params: Some(proto::Params {
                max_models: 500,
                max_param_count: 70,
                enable_auto_proof_submission: true,
                default_max_tokens: 8192,
                governance_threshold: 3,
            }),
        };
        let res: QueryParamsResponse = proto_res.into();
        let p = res.params.unwrap();
        assert_eq!(p.max_models, 500);
        assert_eq!(p.governance_threshold, 3);
    }
}
