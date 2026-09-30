//! Direct gRPC helpers for CosmWasm on Morpheum, for consumers that hold a
//! `tonic::transport::Channel` to a Morpheum node: smart queries, and — under
//! the `dev` feature — unsigned contract execution for development networks.
//!
//! This module is an alternative to the [`CosmWasmClient`](super::client::CosmWasmClient)
//! trait-based query client. Contract execution is a signed transaction:
//! build it with [`ExecuteContractBuilder`](crate::ExecuteContractBuilder) and
//! submit it with `morpheum_sdk_native::TxSubmitter`.
//!
//! Requires the `grpc` feature flag.

use crate::types::CosmWasmError;

#[cfg(feature = "dev")]
mod dev;
#[cfg(feature = "dev")]
pub use dev::{
    broadcast_execute_contract, BroadcastTxRequest, BroadcastTxResponse, TxResponse, WireAny,
};

// ── Wire types ───────────────────────────────────────────────────────

#[derive(Clone, prost::Message)]
pub struct SmartContractStateRequest {
    #[prost(string, tag = "1")]
    pub address: String,
    #[prost(bytes = "vec", tag = "2")]
    pub query_data: Vec<u8>,
}

#[derive(Clone, prost::Message)]
pub struct SmartContractStateResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub data: Vec<u8>,
}

// ── Public API ───────────────────────────────────────────────────────

/// Executes a CosmWasm smart query via direct gRPC.
pub async fn wasm_smart_query(
    channel: &tonic::transport::Channel,
    contract_addr: &str,
    query_json: &[u8],
) -> Result<Vec<u8>, CosmWasmError> {
    let request = SmartContractStateRequest {
        address: contract_addr.to_string(),
        query_data: query_json.to_vec(),
    };

    let mut client = tonic::client::Grpc::new(channel.clone());
    client
        .ready()
        .await
        .map_err(|e| CosmWasmError::Transport(format!("gRPC channel not ready: {e}")))?;

    let path = "/cosmwasm.wasm.v1.Query/SmartContractState"
        .parse::<http::uri::PathAndQuery>()
        .expect("valid gRPC path");

    let codec = tonic_prost::ProstCodec::default();
    let response: tonic::Response<SmartContractStateResponse> = client
        .unary(tonic::Request::new(request), path, codec)
        .await
        .map_err(|e| CosmWasmError::QueryFailed(format!("SmartContractState query: {e}")))?;

    Ok(response.into_inner().data)
}

/// Typed wrapper: deserializes the query response into `T`.
pub async fn wasm_smart_query_typed<T: serde::de::DeserializeOwned>(
    channel: &tonic::transport::Channel,
    contract_addr: &str,
    query_json: &[u8],
) -> Result<T, CosmWasmError> {
    let data = wasm_smart_query(channel, contract_addr, query_json).await?;
    serde_json::from_slice(&data)
        .map_err(|e| CosmWasmError::Deserialization(format!("query response: {e}")))
}
