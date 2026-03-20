//! Fluent builders for CosmWasm SDK request types.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use morpheum_sdk_core::SdkError;

use crate::requests::{
    CoinProto, ExecuteContractRequest, InstantiateContractRequest, StoreCodeRequest,
};

/// Builder for `MsgStoreCode`.
#[derive(Default)]
pub struct StoreCodeBuilder {
    sender: Option<String>,
    wasm_byte_code: Option<Vec<u8>>,
}

impl StoreCodeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sender(mut self, sender: impl Into<String>) -> Self {
        self.sender = Some(sender.into());
        self
    }

    pub fn wasm_byte_code(mut self, code: Vec<u8>) -> Self {
        self.wasm_byte_code = Some(code);
        self
    }

    pub fn build(self) -> Result<StoreCodeRequest, SdkError> {
        Ok(StoreCodeRequest {
            sender: self
                .sender
                .ok_or_else(|| SdkError::InvalidInput("sender is required".into()))?,
            wasm_byte_code: self
                .wasm_byte_code
                .ok_or_else(|| SdkError::InvalidInput("wasm_byte_code is required".into()))?,
        })
    }
}

/// Builder for `MsgInstantiateContract`.
#[derive(Default)]
pub struct InstantiateContractBuilder {
    sender: Option<String>,
    admin: Option<String>,
    code_id: Option<u64>,
    label: Option<String>,
    msg: Option<Vec<u8>>,
    funds: Vec<CoinProto>,
}

impl InstantiateContractBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sender(mut self, sender: impl Into<String>) -> Self {
        self.sender = Some(sender.into());
        self
    }

    pub fn admin(mut self, admin: impl Into<String>) -> Self {
        self.admin = Some(admin.into());
        self
    }

    pub fn code_id(mut self, id: u64) -> Self {
        self.code_id = Some(id);
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the init message as raw JSON bytes.
    pub fn msg(mut self, msg: Vec<u8>) -> Self {
        self.msg = Some(msg);
        self
    }

    /// Sets the init message from a serializable value.
    #[cfg(feature = "serde")]
    pub fn msg_json(mut self, value: &impl serde::Serialize) -> Result<Self, SdkError> {
        let bytes = serde_json::to_vec(value)
            .map_err(|e| SdkError::InvalidInput(format!("JSON serialization: {e}")))?;
        self.msg = Some(bytes);
        Ok(self)
    }

    pub fn add_funds(mut self, denom: impl Into<String>, amount: impl Into<String>) -> Self {
        self.funds.push(CoinProto {
            denom: denom.into(),
            amount: amount.into(),
        });
        self
    }

    pub fn build(self) -> Result<InstantiateContractRequest, SdkError> {
        Ok(InstantiateContractRequest {
            sender: self
                .sender
                .ok_or_else(|| SdkError::InvalidInput("sender is required".into()))?,
            admin: self.admin,
            code_id: self
                .code_id
                .ok_or_else(|| SdkError::InvalidInput("code_id is required".into()))?,
            label: self
                .label
                .ok_or_else(|| SdkError::InvalidInput("label is required".into()))?,
            msg: self
                .msg
                .ok_or_else(|| SdkError::InvalidInput("msg is required".into()))?,
            funds: self.funds,
        })
    }
}

/// Builder for `MsgExecuteContract`.
#[derive(Default)]
pub struct ExecuteContractBuilder {
    sender: Option<String>,
    contract: Option<String>,
    msg: Option<Vec<u8>>,
    funds: Vec<CoinProto>,
}

impl ExecuteContractBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sender(mut self, sender: impl Into<String>) -> Self {
        self.sender = Some(sender.into());
        self
    }

    pub fn contract(mut self, contract: impl Into<String>) -> Self {
        self.contract = Some(contract.into());
        self
    }

    /// Sets the execute message as raw JSON bytes.
    pub fn msg(mut self, msg: Vec<u8>) -> Self {
        self.msg = Some(msg);
        self
    }

    /// Sets the execute message from a serializable value.
    #[cfg(feature = "serde")]
    pub fn msg_json(mut self, value: &impl serde::Serialize) -> Result<Self, SdkError> {
        let bytes = serde_json::to_vec(value)
            .map_err(|e| SdkError::InvalidInput(format!("JSON serialization: {e}")))?;
        self.msg = Some(bytes);
        Ok(self)
    }

    pub fn add_funds(mut self, denom: impl Into<String>, amount: impl Into<String>) -> Self {
        self.funds.push(CoinProto {
            denom: denom.into(),
            amount: amount.into(),
        });
        self
    }

    pub fn build(self) -> Result<ExecuteContractRequest, SdkError> {
        Ok(ExecuteContractRequest {
            sender: self
                .sender
                .ok_or_else(|| SdkError::InvalidInput("sender is required".into()))?,
            contract: self
                .contract
                .ok_or_else(|| SdkError::InvalidInput("contract is required".into()))?,
            msg: self
                .msg
                .ok_or_else(|| SdkError::InvalidInput("msg is required".into()))?,
            funds: self.funds,
        })
    }
}

/// Builder for a Warp Route `transfer_remote` execute message.
///
/// Constructs a `MsgExecuteContract` targeting the CosmWasm Warp Route
/// contract with a `{"transfer_remote":{...}}` JSON payload.
#[derive(Default)]
pub struct WarpRouteTransferBuilder {
    sender: Option<String>,
    warp_route_contract: Option<String>,
    destination_domain: Option<u32>,
    recipient: Option<Vec<u8>>,
    amount: Option<String>,
}

impl WarpRouteTransferBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn sender(mut self, sender: impl Into<String>) -> Self {
        self.sender = Some(sender.into());
        self
    }

    pub fn warp_route_contract(mut self, contract: impl Into<String>) -> Self {
        self.warp_route_contract = Some(contract.into());
        self
    }

    pub fn destination_domain(mut self, domain: u32) -> Self {
        self.destination_domain = Some(domain);
        self
    }

    /// 32-byte canonical recipient address on the destination chain.
    pub fn recipient(mut self, recipient: Vec<u8>) -> Self {
        self.recipient = Some(recipient);
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn build(self) -> Result<ExecuteContractRequest, SdkError> {
        let sender = self
            .sender
            .ok_or_else(|| SdkError::InvalidInput("sender is required".into()))?;
        let contract = self
            .warp_route_contract
            .ok_or_else(|| SdkError::InvalidInput("warp_route_contract is required".into()))?;
        let destination_domain = self
            .destination_domain
            .ok_or_else(|| SdkError::InvalidInput("destination_domain is required".into()))?;
        let recipient = self
            .recipient
            .ok_or_else(|| SdkError::InvalidInput("recipient is required".into()))?;
        if recipient.len() != 32 {
            return Err(SdkError::InvalidInput(
                "recipient must be exactly 32 bytes".into(),
            ));
        }
        let amount = self
            .amount
            .ok_or_else(|| SdkError::InvalidInput("amount is required".into()))?;

        let recipient_hex = hex_encode(&recipient);

        let msg_json = serde_json::json!({
            "transfer_remote": {
                "dest_domain": destination_domain,
                "recipient": recipient_hex,
                "amount": amount
            }
        });

        let msg = serde_json::to_vec(&msg_json)
            .map_err(|e| SdkError::InvalidInput(format!("JSON serialization: {e}")))?;

        Ok(ExecuteContractRequest {
            sender,
            contract,
            msg,
            funds: Vec::new(),
        })
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        use core::fmt::Write;
        let _ = write!(s, "{b:02x}");
    }
    s
}
