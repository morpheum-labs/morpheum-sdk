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
    /// Encodes as `/cosmwasm.wasm.v1.MsgStoreCode`.
    pub fn to_any(&self) -> morpheum_proto::google::protobuf::Any {
        let mut value = Vec::new();
        prost::encoding::string::encode(1, &self.sender, &mut value);
        prost::encoding::bytes::encode(2, &self.wasm_byte_code, &mut value);

        morpheum_proto::google::protobuf::Any {
            type_url: "/cosmwasm.wasm.v1.MsgStoreCode".into(),
            value,
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
    /// Encodes as `/cosmwasm.wasm.v1.MsgInstantiateContract`.
    pub fn to_any(&self) -> morpheum_proto::google::protobuf::Any {
        let mut value = Vec::new();
        prost::encoding::string::encode(1, &self.sender, &mut value);
        if let Some(ref admin) = self.admin {
            prost::encoding::string::encode(2, admin, &mut value);
        }
        prost::encoding::uint64::encode(3, &self.code_id, &mut value);
        prost::encoding::string::encode(4, &self.label, &mut value);
        prost::encoding::bytes::encode(5, &self.msg, &mut value);

        for coin in &self.funds {
            let mut coin_buf = Vec::new();
            prost::encoding::string::encode(1, &coin.denom, &mut coin_buf);
            prost::encoding::string::encode(2, &coin.amount, &mut coin_buf);
            prost::encoding::message::encode(6, &EncodedMessage(coin_buf), &mut value);
        }

        morpheum_proto::google::protobuf::Any {
            type_url: "/cosmwasm.wasm.v1.MsgInstantiateContract".into(),
            value,
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
    /// Encodes as `/cosmwasm.wasm.v1.MsgExecuteContract`.
    pub fn to_any(&self) -> morpheum_proto::google::protobuf::Any {
        let mut value = Vec::new();
        prost::encoding::string::encode(1, &self.sender, &mut value);
        prost::encoding::string::encode(2, &self.contract, &mut value);
        prost::encoding::bytes::encode(3, &self.msg, &mut value);

        for coin in &self.funds {
            let mut coin_buf = Vec::new();
            prost::encoding::string::encode(1, &coin.denom, &mut coin_buf);
            prost::encoding::string::encode(2, &coin.amount, &mut coin_buf);
            prost::encoding::message::encode(5, &EncodedMessage(coin_buf), &mut value);
        }

        morpheum_proto::google::protobuf::Any {
            type_url: "/cosmwasm.wasm.v1.MsgExecuteContract".into(),
            value,
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

/// Helper to encode pre-serialized bytes as a prost message.
struct EncodedMessage(Vec<u8>);

impl prost::Message for EncodedMessage {
    fn encode_raw(&self, buf: &mut impl prost::bytes::BufMut) {
        buf.put_slice(&self.0);
    }

    fn merge_field(
        &mut self,
        _tag: u32,
        _wire_type: prost::encoding::WireType,
        _buf: &mut impl prost::bytes::Buf,
        _ctx: prost::encoding::DecodeContext,
    ) -> Result<(), prost::DecodeError> {
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        self.0.len()
    }

    fn clear(&mut self) {
        self.0.clear();
    }
}
