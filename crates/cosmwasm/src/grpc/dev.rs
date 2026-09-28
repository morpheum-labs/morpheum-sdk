//! Unsigned contract execution for development networks (feature `dev`).
//!
//! These helpers put a `MsgExecuteContract` in the `dev_messages` field of
//! `cosmos.tx.v1beta1.Service/BroadcastTx`, which executes it synchronously
//! and without a signature. Only nodes built with development endpoints
//! serve that field; every other node refuses the request. Use them for
//! deploy scripts and tests against such a node, never as a client's write
//! path.
//!
//! Contract execution is otherwise a signed transaction: build it with
//! [`ExecuteContractBuilder`](crate::ExecuteContractBuilder) and submit it
//! with `morpheum_sdk_native::TxSubmitter`.

use crate::types::CosmWasmError;

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

/// Executes `MsgExecuteContract` unsigned, through the `dev_messages` path
/// that only nodes built with development endpoints serve; every other node
/// refuses it. Anywhere else, submit a signed transaction instead.
///
/// `sender` is the account the call executes as; pass its bech32 address.
/// No funds are attached.
pub async fn broadcast_execute_contract(
    channel: &tonic::transport::Channel,
    sender: &str,
    contract: &str,
    msg_json: &[u8],
) -> Result<TxResponse, CosmWasmError> {
    let exec_json = serde_json::json!({
        "sender": sender,
        "contract": contract,
        "msg": msg_json,
        "funds": []
    });
    let value_bytes =
        serde_json::to_vec(&exec_json).map_err(|e| CosmWasmError::Serialization(e.to_string()))?;

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
