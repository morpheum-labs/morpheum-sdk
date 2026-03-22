//! Direct gRPC client for CosmWasm operations on Morpheum.
//!
//! Provides wire types and helper functions for the Morpheum GMP compatibility
//! layer's `dev_messages` path, which enables synchronous CosmWasm execution
//! via `cosmos.tx.v1beta1.Service/BroadcastTx`.
//!
//! This module is an alternative to the [`CosmWasmClient`](super::client::CosmWasmClient)
//! trait-based approach, for consumers that have a direct `tonic::transport::Channel`
//! to a Morpheum sentry node.
//!
//! Requires the `grpc` feature flag.

use crate::types::CosmWasmError;

// ── Wire types ───────────────────────────────────────────────────────

#[derive(Clone, prost::Message)]
pub struct WireAny {
    #[prost(string, tag = "1")]
    pub type_url: String,
    #[prost(bytes = "vec", tag = "2")]
    pub value: Vec<u8>,
}

#[derive(Clone, prost::Message)]
pub struct BroadcastTxRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub tx_bytes: Vec<u8>,
    #[prost(int32, tag = "2")]
    pub mode: i32,
    #[prost(message, repeated, tag = "99")]
    pub dev_messages: Vec<WireAny>,
}

#[derive(Clone, prost::Message)]
pub struct TxResponse {
    #[prost(string, tag = "2")]
    pub txhash: String,
    #[prost(uint32, tag = "4")]
    pub code: u32,
    #[prost(bytes = "vec", tag = "5")]
    pub data: Vec<u8>,
    #[prost(string, tag = "6")]
    pub raw_log: String,
}

#[derive(Clone, prost::Message)]
pub struct BroadcastTxResponse {
    #[prost(message, optional, tag = "1")]
    pub tx_response: Option<TxResponse>,
}

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

/// Submits `MsgExecuteContract` via the GMP compat layer's `dev_messages`
/// path for synchronous CosmWasm execution.
pub async fn broadcast_execute_contract(
    channel: &tonic::transport::Channel,
    sender: &str,
    contract: &str,
    msg_json: &[u8],
) -> Result<TxResponse, CosmWasmError> {
    broadcast_execute_contract_with_funds(channel, sender, contract, msg_json, &[]).await
}

/// Submits `MsgExecuteContract` with attached funds (coins).
///
/// Each fund entry is `(denom, amount)` where denom is typically the asset
/// index as a string (e.g. `"3"` for ETH) and amount is the stringified value.
pub async fn broadcast_execute_contract_with_funds(
    channel: &tonic::transport::Channel,
    sender: &str,
    contract: &str,
    msg_json: &[u8],
    funds: &[(&str, &str)],
) -> Result<TxResponse, CosmWasmError> {
    let coins: Vec<serde_json::Value> = funds
        .iter()
        .map(|(denom, amount)| serde_json::json!({ "denom": denom, "amount": amount }))
        .collect();

    let exec_json = serde_json::json!({
        "sender": sender,
        "contract": contract,
        "msg": msg_json,
        "funds": coins
    });
    let value_bytes = serde_json::to_vec(&exec_json)
        .map_err(|e| CosmWasmError::Serialization(e.to_string()))?;

    let request = BroadcastTxRequest {
        tx_bytes: Vec::new(),
        mode: 0,
        dev_messages: vec![WireAny {
            type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".into(),
            value: value_bytes,
        }],
    };

    let mut client = tonic::client::Grpc::new(channel.clone());
    client
        .ready()
        .await
        .map_err(|e| CosmWasmError::Transport(format!("gRPC channel not ready: {e}")))?;

    let path = "/cosmos.tx.v1beta1.Service/BroadcastTx"
        .parse::<http::uri::PathAndQuery>()
        .expect("valid gRPC path");

    let codec = tonic_prost::ProstCodec::default();
    let response: tonic::Response<BroadcastTxResponse> = client
        .unary(tonic::Request::new(request), path, codec)
        .await
        .map_err(|e| CosmWasmError::ExecutionFailed(format!("BroadcastTx RPC failed: {e}")))?;

    let inner = response.into_inner();
    let tx_resp = inner
        .tx_response
        .ok_or_else(|| CosmWasmError::ExecutionFailed("empty tx_response".into()))?;

    if tx_resp.code != 0 {
        return Err(CosmWasmError::ExecutionFailed(format!(
            "code={}, log={}",
            tx_resp.code, tx_resp.raw_log
        )));
    }

    tracing::info!(
        txhash = %tx_resp.txhash,
        code = tx_resp.code,
        "BroadcastTx dev_messages succeeded"
    );

    Ok(tx_resp)
}

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
