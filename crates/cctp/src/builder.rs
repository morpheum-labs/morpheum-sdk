//! Typed execute builders for the `hpl-cctp-handler` contract.
//!
//! Each builder produces an [`ExecuteContractRequest`] with the correctly
//! serialized [`ExecuteMsg`] variant, keeping callers free from manual JSON
//! construction and base64 encoding.

use cosmwasm_std::HexBinary;
use morpheum_sdk_cosmwasm::requests::ExecuteContractRequest;

use crate::error::CctpError;
use crate::types::ExecuteMsg;

/// Builder for `ExecuteMsg::FulfillCctp`.
#[derive(Default)]
pub struct FulfillCctpBuilder {
    sender: Option<String>,
    contract: Option<String>,
    cctp_message: Option<Vec<u8>>,
    attestation: Option<Vec<u8>>,
}

impl FulfillCctpBuilder {
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

    pub fn cctp_message(mut self, msg: Vec<u8>) -> Self {
        self.cctp_message = Some(msg);
        self
    }

    pub fn attestation(mut self, att: Vec<u8>) -> Self {
        self.attestation = Some(att);
        self
    }

    pub fn build(self) -> Result<ExecuteContractRequest, CctpError> {
        let exec_msg = ExecuteMsg::FulfillCctp {
            cctp_message: HexBinary::from(
                self.cctp_message
                    .ok_or_else(|| CctpError::Builder("cctp_message is required".into()))?,
            ),
            attestation: HexBinary::from(
                self.attestation
                    .ok_or_else(|| CctpError::Builder("attestation is required".into()))?,
            ),
        };
        build_request(
            self.sender,
            self.contract,
            &exec_msg,
        )
    }
}

/// Builder for `ExecuteMsg::EnrollRemoteRouter`.
#[derive(Default)]
pub struct EnrollRemoteRouterBuilder {
    sender: Option<String>,
    contract: Option<String>,
    domain: Option<u32>,
    router: Option<Vec<u8>>,
}

impl EnrollRemoteRouterBuilder {
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

    pub fn domain(mut self, domain: u32) -> Self {
        self.domain = Some(domain);
        self
    }

    /// 32-byte router address on the remote chain.
    pub fn router(mut self, router: Vec<u8>) -> Self {
        self.router = Some(router);
        self
    }

    pub fn build(self) -> Result<ExecuteContractRequest, CctpError> {
        let exec_msg = ExecuteMsg::EnrollRemoteRouter {
            domain: self
                .domain
                .ok_or_else(|| CctpError::Builder("domain is required".into()))?,
            router: HexBinary::from(
                self.router
                    .ok_or_else(|| CctpError::Builder("router is required".into()))?,
            ),
        };
        build_request(self.sender, self.contract, &exec_msg)
    }
}

/// Builder for `ExecuteMsg::UpdateAttesters`.
#[derive(Default)]
pub struct UpdateAttestersBuilder {
    sender: Option<String>,
    contract: Option<String>,
    add: Vec<Vec<u8>>,
    remove: Vec<Vec<u8>>,
    threshold: Option<u32>,
}

impl UpdateAttestersBuilder {
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

    /// Add an attester ETH address (20 bytes).
    pub fn add_attester(mut self, attester: Vec<u8>) -> Self {
        self.add.push(attester);
        self
    }

    /// Remove an attester ETH address (20 bytes).
    pub fn remove_attester(mut self, attester: Vec<u8>) -> Self {
        self.remove.push(attester);
        self
    }

    pub fn threshold(mut self, threshold: u32) -> Self {
        self.threshold = Some(threshold);
        self
    }

    pub fn build(self) -> Result<ExecuteContractRequest, CctpError> {
        let exec_msg = ExecuteMsg::UpdateAttesters {
            add: self.add.into_iter().map(HexBinary::from).collect(),
            remove: self.remove.into_iter().map(HexBinary::from).collect(),
            threshold: self.threshold,
        };
        build_request(self.sender, self.contract, &exec_msg)
    }
}

/// Builder for `ExecuteMsg::SetPostMintHook`.
#[derive(Default)]
pub struct SetPostMintHookBuilder {
    sender: Option<String>,
    contract: Option<String>,
    hook: Option<Option<String>>,
}

impl SetPostMintHookBuilder {
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

    /// Set the hook contract address. Pass `None` to clear the hook.
    pub fn hook(mut self, hook: Option<String>) -> Self {
        self.hook = Some(hook);
        self
    }

    pub fn build(self) -> Result<ExecuteContractRequest, CctpError> {
        let exec_msg = ExecuteMsg::SetPostMintHook {
            hook: self
                .hook
                .ok_or_else(|| CctpError::Builder("hook must be explicitly set (Some or None)".into()))?,
        };
        build_request(self.sender, self.contract, &exec_msg)
    }
}

fn build_request(
    sender: Option<String>,
    contract: Option<String>,
    msg: &ExecuteMsg,
) -> Result<ExecuteContractRequest, CctpError> {
    let sender =
        sender.ok_or_else(|| CctpError::Builder("sender is required".into()))?;
    let contract =
        contract.ok_or_else(|| CctpError::Builder("contract is required".into()))?;
    let msg_bytes = serde_json::to_vec(msg)
        .map_err(|e| CctpError::Serialization(e.to_string()))?;

    Ok(ExecuteContractRequest {
        sender,
        contract,
        msg: msg_bytes,
        funds: Vec::new(),
    })
}
