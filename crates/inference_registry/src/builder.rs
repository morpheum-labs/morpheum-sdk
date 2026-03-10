//! Fluent builders for the Inference Registry module.
//!
//! Each builder follows the classic Builder pattern with validation and returns
//! the corresponding request type from `requests.rs` for seamless integration
//! with `TxBuilder`.

use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{RegisterModelRequest, UpdateParamsRequest};
use crate::types::{Params, QuantFormat};

/// Fluent builder for registering a new inference model (governance only).
///
/// # Example
/// ```rust,ignore
/// let request = RegisterModelBuilder::new()
///     .authority("morpheum1gov")
///     .display_name("Llama-3.1-8B-Q4")
///     .quant_format(QuantFormat::Q4KM)
///     .param_count(8)
///     .zk_commitment(commitment_hash.to_vec())
///     .supported_ops(ops::INFER | ops::EMBED)
///     .version(1)
///     .build()?;
///
/// let any = request.to_any();
/// ```
#[derive(Default)]
pub struct RegisterModelBuilder {
    authority: Option<String>,
    display_name: Option<String>,
    quant_format: Option<QuantFormat>,
    param_count: Option<u64>,
    zk_commitment: Option<Vec<u8>>,
    supported_ops: Option<u64>,
    version: Option<u32>,
    weights_payload: Option<Vec<u8>>,
}

impl RegisterModelBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the governance authority address.
    pub fn authority(mut self, authority: impl Into<String>) -> Self {
        self.authority = Some(authority.into());
        self
    }

    /// Sets the human-readable model name.
    pub fn display_name(mut self, name: impl Into<String>) -> Self {
        self.display_name = Some(name.into());
        self
    }

    /// Sets the quantization format.
    pub fn quant_format(mut self, format: QuantFormat) -> Self {
        self.quant_format = Some(format);
        self
    }

    /// Sets the parameter count in billions.
    pub fn param_count(mut self, count: u64) -> Self {
        self.param_count = Some(count);
        self
    }

    /// Sets the halo2 zk commitment hash.
    pub fn zk_commitment(mut self, commitment: Vec<u8>) -> Self {
        self.zk_commitment = Some(commitment);
        self
    }

    /// Sets the supported operations bitflags (e.g. `ops::INFER | ops::EMBED`).
    pub fn supported_ops(mut self, ops: u64) -> Self {
        self.supported_ops = Some(ops);
        self
    }

    /// Sets the model version.
    pub fn version(mut self, version: u32) -> Self {
        self.version = Some(version);
        self
    }

    /// Sets the optional raw model weights payload.
    pub fn weights_payload(mut self, payload: Vec<u8>) -> Self {
        self.weights_payload = Some(payload);
        self
    }

    /// Builds the register-model request, performing validation.
    pub fn build(self) -> Result<RegisterModelRequest, SdkError> {
        let authority = self.authority.ok_or_else(|| {
            SdkError::invalid_input("authority is required for RegisterModel")
        })?;

        let display_name = self.display_name.ok_or_else(|| {
            SdkError::invalid_input("display_name is required for RegisterModel")
        })?;

        let quant_format = self.quant_format.ok_or_else(|| {
            SdkError::invalid_input("quant_format is required for RegisterModel")
        })?;

        let param_count = self.param_count.ok_or_else(|| {
            SdkError::invalid_input("param_count is required for RegisterModel")
        })?;

        let zk_commitment = self.zk_commitment.ok_or_else(|| {
            SdkError::invalid_input("zk_commitment is required for RegisterModel")
        })?;

        let supported_ops = self.supported_ops.ok_or_else(|| {
            SdkError::invalid_input("supported_ops is required for RegisterModel")
        })?;

        let version = self.version.ok_or_else(|| {
            SdkError::invalid_input("version is required for RegisterModel")
        })?;

        let mut req = RegisterModelRequest::new(
            authority,
            display_name,
            quant_format,
            param_count,
            zk_commitment,
            supported_ops,
            version,
        );

        if let Some(payload) = self.weights_payload {
            req = req.with_weights_payload(payload);
        }

        Ok(req)
    }
}

/// Fluent builder for updating inference_registry module parameters (governance only).
///
/// # Example
/// ```rust,ignore
/// let request = UpdateParamsBuilder::new()
///     .authority("morpheum1gov")
///     .params(Params { max_models: 500, ..Default::default() })
///     .build()?;
/// ```
#[derive(Default)]
pub struct UpdateParamsBuilder {
    authority: Option<String>,
    params: Option<Params>,
}

impl UpdateParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the governance authority address.
    pub fn authority(mut self, authority: impl Into<String>) -> Self {
        self.authority = Some(authority.into());
        self
    }

    /// Sets the new module parameters.
    pub fn params(mut self, params: Params) -> Self {
        self.params = Some(params);
        self
    }

    /// Builds the update-params request, performing validation.
    pub fn build(self) -> Result<UpdateParamsRequest, SdkError> {
        let authority = self.authority.ok_or_else(|| {
            SdkError::invalid_input("authority is required for UpdateParams")
        })?;

        let params = self.params.ok_or_else(|| {
            SdkError::invalid_input("params are required for UpdateParams")
        })?;

        Ok(UpdateParamsRequest::new(authority, params))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use crate::types::ops;

    #[test]
    fn register_model_builder_full_flow() {
        let request = RegisterModelBuilder::new()
            .authority("morpheum1gov")
            .display_name("Llama-3.1-8B-Q4")
            .quant_format(QuantFormat::Q4KM)
            .param_count(8)
            .zk_commitment(vec![0xBB; 32])
            .supported_ops(ops::INFER | ops::EMBED)
            .version(1)
            .build()
            .unwrap();

        assert_eq!(request.authority, "morpheum1gov");
        assert_eq!(request.display_name, "Llama-3.1-8B-Q4");
        assert_eq!(request.quant_format, QuantFormat::Q4KM);
        assert_eq!(request.param_count, 8);
        assert_eq!(request.version, 1);
        assert!(request.weights_payload.is_empty());
    }

    #[test]
    fn register_model_builder_with_weights() {
        let request = RegisterModelBuilder::new()
            .authority("morpheum1gov")
            .display_name("Test")
            .quant_format(QuantFormat::Fp16)
            .param_count(70)
            .zk_commitment(vec![0; 32])
            .supported_ops(ops::INFER)
            .version(1)
            .weights_payload(vec![1, 2, 3])
            .build()
            .unwrap();

        assert_eq!(request.weights_payload, vec![1, 2, 3]);
    }

    #[test]
    fn register_model_builder_validation() {
        let result = RegisterModelBuilder::new().build();
        assert!(result.is_err());

        let result = RegisterModelBuilder::new()
            .authority("morpheum1gov")
            .display_name("Test")
            .quant_format(QuantFormat::Q4KM)
            .build();
        assert!(result.is_err());

        let result = RegisterModelBuilder::new()
            .authority("morpheum1gov")
            .display_name("Test")
            .quant_format(QuantFormat::Q4KM)
            .param_count(8)
            .zk_commitment(vec![0; 32])
            .supported_ops(ops::INFER)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn update_params_builder_full_flow() {
        let request = UpdateParamsBuilder::new()
            .authority("morpheum1gov")
            .params(Params {
                max_models: 500,
                governance_threshold: 3,
                ..Default::default()
            })
            .build()
            .unwrap();

        assert_eq!(request.authority, "morpheum1gov");
        assert_eq!(request.params.max_models, 500);
        assert_eq!(request.params.governance_threshold, 3);
    }

    #[test]
    fn update_params_builder_validation() {
        let result = UpdateParamsBuilder::new().build();
        assert!(result.is_err());

        let result = UpdateParamsBuilder::new()
            .authority("morpheum1gov")
            .build();
        assert!(result.is_err());
    }
}
