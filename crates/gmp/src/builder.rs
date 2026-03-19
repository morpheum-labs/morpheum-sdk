//! Fluent builders for GMP SDK request types.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{string::String, vec::Vec};

use morpheum_sdk_core::SdkError;

use crate::requests::{
    ProcessHyperlaneMessageRequest, SettleGmpPaymentRequest, WarpRouteTransferRequest,
};

/// Builder for `MsgWarpRouteTransfer`.
#[derive(Default)]
pub struct WarpRouteTransferBuilder {
    sender: Option<String>,
    destination_domain: Option<u32>,
    recipient: Option<Vec<u8>>,
    asset_index: Option<u64>,
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

    pub fn destination_domain(mut self, domain: u32) -> Self {
        self.destination_domain = Some(domain);
        self
    }

    pub fn recipient(mut self, recipient: Vec<u8>) -> Self {
        self.recipient = Some(recipient);
        self
    }

    pub fn asset_index(mut self, index: u64) -> Self {
        self.asset_index = Some(index);
        self
    }

    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    pub fn build(self) -> Result<WarpRouteTransferRequest, SdkError> {
        let sender = self
            .sender
            .ok_or_else(|| SdkError::InvalidInput("sender is required".into()))?;
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
        let asset_index = self
            .asset_index
            .ok_or_else(|| SdkError::InvalidInput("asset_index is required".into()))?;
        let amount = self
            .amount
            .ok_or_else(|| SdkError::InvalidInput("amount is required".into()))?;

        Ok(WarpRouteTransferRequest {
            sender,
            destination_domain,
            recipient,
            asset_index,
            amount,
        })
    }
}

/// Builder for `MsgProcessHyperlaneMessage`.
#[derive(Default)]
pub struct ProcessHyperlaneMessageBuilder {
    metadata: Option<Vec<u8>>,
    message: Option<Vec<u8>>,
}

impl ProcessHyperlaneMessageBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn metadata(mut self, metadata: Vec<u8>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    pub fn message(mut self, message: Vec<u8>) -> Self {
        self.message = Some(message);
        self
    }

    pub fn build(self) -> Result<ProcessHyperlaneMessageRequest, SdkError> {
        let metadata = self
            .metadata
            .ok_or_else(|| SdkError::InvalidInput("metadata is required".into()))?;
        let message = self
            .message
            .ok_or_else(|| SdkError::InvalidInput("message is required".into()))?;

        Ok(ProcessHyperlaneMessageRequest { metadata, message })
    }
}

/// Builder for `MsgSettleGmpPayment`.
#[derive(Default)]
pub struct SettleGmpPaymentBuilder {
    protocol_id: Option<String>,
    raw_envelope: Option<Vec<u8>>,
}

impl SettleGmpPaymentBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn protocol_id(mut self, id: impl Into<String>) -> Self {
        self.protocol_id = Some(id.into());
        self
    }

    pub fn raw_envelope(mut self, envelope: Vec<u8>) -> Self {
        self.raw_envelope = Some(envelope);
        self
    }

    pub fn build(self) -> Result<SettleGmpPaymentRequest, SdkError> {
        let protocol_id = self
            .protocol_id
            .ok_or_else(|| SdkError::InvalidInput("protocol_id is required".into()))?;
        let raw_envelope = self
            .raw_envelope
            .ok_or_else(|| SdkError::InvalidInput("raw_envelope is required".into()))?;

        Ok(SettleGmpPaymentRequest {
            protocol_id,
            raw_envelope,
        })
    }
}
