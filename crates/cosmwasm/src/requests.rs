//! CosmWasm SDK request types.
//!
//! Each request type wraps the necessary data for a CosmWasm message and
//! provides `to_any()` for encoding into the Cosmos wire format used by
//! the GMP protocol bridge.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

/// Request to store a new WASM code blob on Morpheum.
#[derive(Clone, Debug)]
pub struct StoreCodeRequest {
    pub sender: String,
    pub wasm_byte_code: Vec<u8>,
}

impl StoreCodeRequest {
    /// Encodes as `/cosmwasm.wasm.v1.MsgStoreCode` (JSON wire format).
    ///
    /// Mormcore's CosmWasm actor deserialises message bytes with
    /// `serde_json::from_slice`, so we must produce JSON — not protobuf.
    pub fn to_any(&self) -> morpheum_proto::google::protobuf::Any {
        let value = serde_json::json!({
            "sender": self.sender,
            "wasm_byte_code": self.wasm_byte_code,
        });

        morpheum_proto::google::protobuf::Any {
            type_url: "/cosmwasm.wasm.v1.MsgStoreCode".into(),
            value: serde_json::to_vec(&value).unwrap_or_default(),
        }
    }
}

/// Request to instantiate a CosmWasm contract from stored code.
#[derive(Clone, Debug)]
pub struct InstantiateContractRequest {
    pub sender: String,
    pub admin: Option<String>,
    pub code_id: u64,
    pub label: String,
    pub msg: Vec<u8>,
    pub funds: Vec<CoinProto>,
}

/// A Cosmos SDK Coin for the funds field.
#[derive(Clone, Debug)]
pub struct CoinProto {
    pub denom: String,
    pub amount: String,
}

impl InstantiateContractRequest {
    /// Encodes as `/cosmwasm.wasm.v1.MsgInstantiateContract` (JSON wire format).
    ///
    /// Mormcore's CosmWasm actor deserialises message bytes with
    /// `serde_json::from_slice`, so we must produce JSON — not protobuf.
    pub fn to_any(&self) -> morpheum_proto::google::protobuf::Any {
        let funds: Vec<serde_json::Value> = self.funds.iter().map(|c| {
            serde_json::json!({ "denom": c.denom, "amount": c.amount })
        }).collect();

        let value = serde_json::json!({
            "sender": self.sender,
            "admin": self.admin,
            "code_id": self.code_id,
            "label": self.label,
            "msg": self.msg,
            "funds": funds,
        });

        morpheum_proto::google::protobuf::Any {
            type_url: "/cosmwasm.wasm.v1.MsgInstantiateContract".into(),
            value: serde_json::to_vec(&value).unwrap_or_default(),
        }
    }
}

/// Request to execute a CosmWasm contract.
#[derive(Clone, Debug)]
pub struct ExecuteContractRequest {
    pub sender: String,
    pub contract: String,
    pub msg: Vec<u8>,
    pub funds: Vec<CoinProto>,
}

impl ExecuteContractRequest {
    /// Encodes as `/cosmwasm.wasm.v1.MsgExecuteContract` (JSON wire format).
    ///
    /// Mormcore's CosmWasm actor deserialises message bytes with
    /// `serde_json::from_slice`, so we must produce JSON — not protobuf.
    pub fn to_any(&self) -> morpheum_proto::google::protobuf::Any {
        let funds: Vec<serde_json::Value> = self.funds.iter().map(|c| {
            serde_json::json!({ "denom": c.denom, "amount": c.amount })
        }).collect();

        let value = serde_json::json!({
            "sender": self.sender,
            "contract": self.contract,
            "msg": self.msg,
            "funds": funds,
        });

        morpheum_proto::google::protobuf::Any {
            type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".into(),
            value: serde_json::to_vec(&value).unwrap_or_default(),
        }
    }
}

/// Request to query smart contract state (JSON query -> JSON response).
#[derive(Clone, Debug)]
pub struct QuerySmartRequest {
    pub contract: String,
    pub query_data: Vec<u8>,
}

/// Request to query raw contract state by key.
#[derive(Clone, Debug)]
pub struct QueryRawRequest {
    pub contract: String,
    pub key: Vec<u8>,
}

