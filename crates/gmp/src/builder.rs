//! Fluent builders for GMP SDK request types.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
use alloc::{
    collections::BTreeMap,
    format,
    string::String,
    vec::Vec,
};
#[cfg(feature = "std")]
use std::collections::BTreeMap;

use morpheum_primitives::address::GOVERNANCE_ADDRESS;
use morpheum_sdk_core::SdkError;

use crate::requests::{
    SettleGmpPaymentRequest, UpdateGmpParamsRequest,
    WarpRouteTransferRequest,
};
use crate::types;

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

// ── Governance param builders ───────────────────────────────────────

/// EVM-style address length (20 bytes). Mirrors `morpheum_primitives::constants::gmp::EVM_ADDRESS_LEN`.
const EVM_ADDRESS_LEN: usize = 20;
/// Hyperlane universal address length (32 bytes). Mirrors `morpheum_primitives::constants::gmp::UNIVERSAL_ADDRESS_LEN`.
const UNIVERSAL_ADDRESS_LEN: usize = 32;

/// Builder for Hyperlane protocol security parameters.
#[derive(Default)]
pub struct HyperlaneParamsBuilder {
    validators: Vec<Vec<u8>>,
    threshold: Option<u32>,
    domain_to_caip2: BTreeMap<u32, String>,
    trusted_senders: Vec<Vec<u8>>,
}

impl HyperlaneParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_validator(mut self, address: Vec<u8>) -> Self {
        self.validators.push(address);
        self
    }

    pub fn validators(mut self, addrs: Vec<Vec<u8>>) -> Self {
        self.validators = addrs;
        self
    }

    pub fn threshold(mut self, threshold: u32) -> Self {
        self.threshold = Some(threshold);
        self
    }

    pub fn add_domain(mut self, domain: u32, caip2: impl Into<String>) -> Self {
        self.domain_to_caip2.insert(domain, caip2.into());
        self
    }

    pub fn domain_to_caip2(mut self, map: BTreeMap<u32, String>) -> Self {
        self.domain_to_caip2 = map;
        self
    }

    pub fn add_trusted_sender(mut self, sender: Vec<u8>) -> Self {
        self.trusted_senders.push(sender);
        self
    }

    pub fn trusted_senders(mut self, senders: Vec<Vec<u8>>) -> Self {
        self.trusted_senders = senders;
        self
    }

    pub fn build(self) -> Result<types::HyperlaneParams, SdkError> {
        if self.validators.is_empty() {
            return Err(SdkError::InvalidInput(
                "at least one validator is required".into(),
            ));
        }
        for (i, v) in self.validators.iter().enumerate() {
            if v.len() != EVM_ADDRESS_LEN {
                return Err(SdkError::InvalidInput(format!(
                    "validator[{i}] must be exactly {EVM_ADDRESS_LEN} bytes, got {}",
                    v.len()
                )));
            }
        }

        let threshold = self.threshold.unwrap_or(1);
        if threshold == 0 || threshold as usize > self.validators.len() {
            return Err(SdkError::InvalidInput(format!(
                "threshold must be in 1..={}, got {threshold}",
                self.validators.len()
            )));
        }

        for (i, s) in self.trusted_senders.iter().enumerate() {
            if s.len() != UNIVERSAL_ADDRESS_LEN {
                return Err(SdkError::InvalidInput(format!(
                    "trusted_sender[{i}] must be exactly {UNIVERSAL_ADDRESS_LEN} bytes, got {}",
                    s.len()
                )));
            }
        }

        Ok(types::HyperlaneParams {
            validators: self.validators,
            threshold,
            domain_to_caip2: self.domain_to_caip2,
            trusted_senders: self.trusted_senders,
        })
    }
}

/// Builder for Warp Route configuration.
#[derive(Default)]
pub struct WarpRouteConfigBuilder {
    recipient_address: Option<Vec<u8>>,
    routes: BTreeMap<u32, types::WarpRouteToken>,
}

impl WarpRouteConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn recipient_address(mut self, addr: Vec<u8>) -> Self {
        self.recipient_address = Some(addr);
        self
    }

    pub fn add_route(
        mut self,
        domain: u32,
        collateral_address: Vec<u8>,
        asset_index: u64,
    ) -> Self {
        self.routes.insert(
            domain,
            types::WarpRouteToken {
                collateral_address,
                asset_index,
            },
        );
        self
    }

    pub fn build(self) -> Result<types::WarpRouteConfig, SdkError> {
        let recipient_address = self
            .recipient_address
            .ok_or_else(|| SdkError::InvalidInput("recipient_address is required".into()))?;
        if recipient_address.len() != UNIVERSAL_ADDRESS_LEN {
            return Err(SdkError::InvalidInput(format!(
                "recipient_address must be exactly {UNIVERSAL_ADDRESS_LEN} bytes, got {}",
                recipient_address.len()
            )));
        }

        for (&domain, token) in &self.routes {
            if token.collateral_address.len() != UNIVERSAL_ADDRESS_LEN {
                return Err(SdkError::InvalidInput(format!(
                    "route[{domain}].collateral_address must be exactly {UNIVERSAL_ADDRESS_LEN} bytes, got {}",
                    token.collateral_address.len()
                )));
            }
        }

        Ok(types::WarpRouteConfig {
            recipient_address,
            routes: self.routes,
        })
    }
}

/// Builder for `MsgUpdateParams` governance submission.
#[derive(Default)]
pub struct UpdateGmpParamsBuilder {
    authority: Option<String>,
    hyperlane: Option<types::HyperlaneParams>,
    warp_route: Option<types::WarpRouteConfig>,
}

impl UpdateGmpParamsBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Governance authority address (defaults to the deterministic governance module address).
    pub fn authority(mut self, authority: impl Into<String>) -> Self {
        self.authority = Some(authority.into());
        self
    }

    pub fn hyperlane(mut self, params: types::HyperlaneParams) -> Self {
        self.hyperlane = Some(params);
        self
    }

    pub fn warp_route(mut self, config: types::WarpRouteConfig) -> Self {
        self.warp_route = Some(config);
        self
    }

    pub fn build(self) -> Result<UpdateGmpParamsRequest, SdkError> {
        if self.hyperlane.is_none() && self.warp_route.is_none() {
            return Err(SdkError::InvalidInput(
                "at least one of hyperlane or warp_route must be set".into(),
            ));
        }

        let authority = self
            .authority
            .unwrap_or_else(|| GOVERNANCE_ADDRESS.into());

        Ok(UpdateGmpParamsRequest {
            authority,
            params: types::GmpParams {
                hyperlane: self.hyperlane,
                warp_route: self.warp_route,
            },
        })
    }
}
