//! CosmWasm query client.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{boxed::Box, string::String, vec::Vec};

use async_trait::async_trait;

use morpheum_sdk_core::{MorpheumClient, SdkConfig, SdkError, Transport};

use crate::requests::{QueryRawRequest, QuerySmartRequest};
use crate::types::ContractInfo;
#[cfg(feature = "serde")]
use crate::types::CosmWasmError;

/// Client for querying CosmWasm contracts on Morpheum.
pub struct CosmWasmClient {
    config: SdkConfig,
    transport: Box<dyn Transport>,
}

impl CosmWasmClient {
    pub fn new(config: SdkConfig, transport: Box<dyn Transport>) -> Self {
        Self { config, transport }
    }

    /// Queries a CosmWasm smart contract with a JSON query and returns the JSON response.
    pub async fn query_smart(&self, req: &QuerySmartRequest) -> Result<Vec<u8>, SdkError> {
        let mut query_data = Vec::new();
        prost::encoding::string::encode(1, &req.contract, &mut query_data);
        prost::encoding::bytes::encode(2, &req.query_data, &mut query_data);

        let resp_bytes = self
            .query(
                "/cosmwasm.wasm.v1.Query/SmartContractState",
                query_data,
            )
            .await?;

        decode_smart_response(&resp_bytes)
    }

    /// Queries raw contract state by key.
    pub async fn query_raw(&self, req: &QueryRawRequest) -> Result<Vec<u8>, SdkError> {
        let mut query_data = Vec::new();
        prost::encoding::string::encode(1, &req.contract, &mut query_data);
        prost::encoding::bytes::encode(2, &req.key, &mut query_data);

        let resp_bytes = self
            .query(
                "/cosmwasm.wasm.v1.Query/RawContractState",
                query_data,
            )
            .await?;

        decode_raw_response(&resp_bytes)
    }

    /// Typed wrapper: deserializes the query response into `T`.
    ///
    /// Combines [`query_smart`](Self::query_smart) with serde deserialization
    /// so callers don't need to manually decode JSON bytes.
    ///
    /// Requires the `serde` feature.
    #[cfg(feature = "serde")]
    pub async fn query_smart_typed<T: serde::de::DeserializeOwned>(
        &self,
        contract: &str,
        query: &impl serde::Serialize,
    ) -> Result<T, SdkError> {
        let query_data = serde_json::to_vec(query)
            .map_err(|e| CosmWasmError::Serialization(e.to_string()))?;

        let req = QuerySmartRequest {
            contract: contract.to_string(),
            query_data,
        };
        let resp_bytes = self.query_smart(&req).await?;
        serde_json::from_slice(&resp_bytes)
            .map_err(|e| CosmWasmError::Deserialization(e.to_string()).into())
    }

    /// Queries contract metadata (code_id, admin, label).
    pub async fn query_contract_info(
        &self,
        contract: &str,
    ) -> Result<ContractInfo, SdkError> {
        let mut query_data = Vec::new();
        prost::encoding::string::encode(1, &contract.to_string(), &mut query_data);

        let resp_bytes = self
            .query(
                "/cosmwasm.wasm.v1.Query/ContractInfo",
                query_data,
            )
            .await?;

        decode_contract_info(&resp_bytes, contract)
    }
}

#[async_trait(?Send)]
impl MorpheumClient for CosmWasmClient {
    fn config(&self) -> &SdkConfig {
        &self.config
    }

    fn transport(&self) -> &dyn Transport {
        &*self.transport
    }
}

fn decode_smart_response(data: &[u8]) -> Result<Vec<u8>, SdkError> {
    let mut buf = data;
    let mut result_data = Vec::new();
    while !buf.is_empty() {
        let (tag, wire_type) = prost::encoding::decode_key(&mut buf)?;
        if tag == 1 {
            prost::encoding::bytes::merge(wire_type, &mut result_data, &mut buf, Default::default())?;
        } else {
            prost::encoding::skip_field(wire_type, tag, &mut buf, Default::default())?;
        }
    }
    Ok(result_data)
}

fn decode_raw_response(data: &[u8]) -> Result<Vec<u8>, SdkError> {
    decode_smart_response(data)
}

fn decode_contract_info(data: &[u8], address: &str) -> Result<ContractInfo, SdkError> {
    let mut code_id: u64 = 0;
    let mut admin = String::new();
    let mut label = String::new();

    let mut buf = data;
    while !buf.is_empty() {
        let (tag, wire_type) = prost::encoding::decode_key(&mut buf)?;
        match tag {
            3 => {
                let mut inner_buf_data = Vec::new();
                prost::encoding::bytes::merge(wire_type, &mut inner_buf_data, &mut buf, Default::default())?;
                let mut inner = inner_buf_data.as_slice();
                while !inner.is_empty() {
                    let (inner_tag, inner_wt) = prost::encoding::decode_key(&mut inner)?;
                    match inner_tag {
                        1 => prost::encoding::uint64::merge(inner_wt, &mut code_id, &mut inner, Default::default())?,
                        3 => prost::encoding::string::merge(inner_wt, &mut admin, &mut inner, Default::default())?,
                        4 => prost::encoding::string::merge(inner_wt, &mut label, &mut inner, Default::default())?,
                        _ => prost::encoding::skip_field(inner_wt, inner_tag, &mut inner, Default::default())?,
                    }
                }
            }
            _ => prost::encoding::skip_field(wire_type, tag, &mut buf, Default::default())?,
        }
    }

    Ok(ContractInfo {
        address: address.to_string(),
        code_id,
        admin: if admin.is_empty() { None } else { Some(admin) },
        label,
    })
}
