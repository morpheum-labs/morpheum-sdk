//! Fluent builders for the token module.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use morpheum_sdk_core::SdkError;

use crate::requests::{
    CreateTokenRequest, DisableHookRequest, SetOriginMetadataRequest,
    SetTradableRequest, UpdateMetadataRequest,
};
use crate::types::{HookPoint, ProgrammableLogicConfig};

// ====================== CREATE TOKEN ======================

#[derive(Default)]
pub struct CreateTokenBuilder {
    name: Option<String>,
    symbol: Option<String>,
    decimals: u32,
    initial_supply: Option<String>,
    creator_address: Option<String>,
    metadata: BTreeMap<String, String>,
    agent_creator_did: Option<String>,
    programmable: Option<ProgrammableLogicConfig>,
    tradable: bool,
    origin_chain: Option<String>,
    origin_address: Option<String>,
    bridge_protocol: Option<String>,
    is_wrapped: bool,
}

impl CreateTokenBuilder {
    pub fn new() -> Self { Self { decimals: 18, ..Self::default() } }

    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn symbol(mut self, v: impl Into<String>) -> Self { self.symbol = Some(v.into()); self }
    pub fn decimals(mut self, v: u32) -> Self { self.decimals = v; self }
    pub fn initial_supply(mut self, v: impl Into<String>) -> Self { self.initial_supply = Some(v.into()); self }
    pub fn creator_address(mut self, v: impl Into<String>) -> Self { self.creator_address = Some(v.into()); self }
    pub fn metadata(mut self, v: BTreeMap<String, String>) -> Self { self.metadata = v; self }
    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into()); self
    }
    pub fn agent_creator_did(mut self, v: impl Into<String>) -> Self { self.agent_creator_did = Some(v.into()); self }
    pub fn programmable(mut self, v: ProgrammableLogicConfig) -> Self { self.programmable = Some(v); self }
    pub fn tradable(mut self, v: bool) -> Self { self.tradable = v; self }
    pub fn origin(mut self, chain: impl Into<String>, address: impl Into<String>, protocol: impl Into<String>, wrapped: bool) -> Self {
        self.origin_chain = Some(chain.into());
        self.origin_address = Some(address.into());
        self.bridge_protocol = Some(protocol.into());
        self.is_wrapped = wrapped;
        self
    }

    pub fn build(self) -> Result<CreateTokenRequest, SdkError> {
        let mut req = CreateTokenRequest::new(
            self.name.ok_or_else(|| SdkError::invalid_input("name is required"))?,
            self.symbol.ok_or_else(|| SdkError::invalid_input("symbol is required"))?,
            self.decimals,
            self.initial_supply.ok_or_else(|| SdkError::invalid_input("initial_supply is required"))?,
            self.creator_address.ok_or_else(|| SdkError::invalid_input("creator_address is required"))?,
        );
        req.metadata = self.metadata;
        req.agent_creator_did = self.agent_creator_did;
        req.programmable = self.programmable;
        req.tradable = self.tradable;
        req.origin_chain = self.origin_chain;
        req.origin_address = self.origin_address;
        req.bridge_protocol = self.bridge_protocol;
        req.is_wrapped = self.is_wrapped;
        Ok(req)
    }
}

// ====================== SET TRADABLE ======================

#[derive(Default)]
pub struct SetTradableBuilder {
    asset_index: Option<u64>,
    tradable: bool,
    signer_address: Option<String>,
}

impl SetTradableBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn tradable(mut self, v: bool) -> Self { self.tradable = v; self }
    pub fn signer_address(mut self, v: impl Into<String>) -> Self { self.signer_address = Some(v.into()); self }

    pub fn build(self) -> Result<SetTradableRequest, SdkError> {
        Ok(SetTradableRequest::new(
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.tradable,
            self.signer_address.ok_or_else(|| SdkError::invalid_input("signer_address is required"))?,
        ))
    }
}

// ====================== UPDATE METADATA ======================

pub struct UpdateMetadataBuilder {
    asset_index: Option<u64>,
    signer_address: Option<String>,
    name: Option<String>,
    symbol: Option<String>,
    icon: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    uri: Option<String>,
    metadata: BTreeMap<String, String>,
}

impl UpdateMetadataBuilder {
    pub fn new() -> Self {
        Self {
            asset_index: None, signer_address: None,
            name: None, symbol: None, icon: None, description: None,
            tags: Vec::new(), uri: None, metadata: BTreeMap::new(),
        }
    }

    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn signer_address(mut self, v: impl Into<String>) -> Self { self.signer_address = Some(v.into()); self }
    pub fn name(mut self, v: impl Into<String>) -> Self { self.name = Some(v.into()); self }
    pub fn symbol(mut self, v: impl Into<String>) -> Self { self.symbol = Some(v.into()); self }
    pub fn icon(mut self, v: impl Into<String>) -> Self { self.icon = Some(v.into()); self }
    pub fn description(mut self, v: impl Into<String>) -> Self { self.description = Some(v.into()); self }
    pub fn tags(mut self, v: Vec<String>) -> Self { self.tags = v; self }
    pub fn uri(mut self, v: impl Into<String>) -> Self { self.uri = Some(v.into()); self }
    pub fn add_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into()); self
    }

    pub fn build(self) -> Result<UpdateMetadataRequest, SdkError> {
        let mut req = UpdateMetadataRequest::new(
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.signer_address.ok_or_else(|| SdkError::invalid_input("signer_address is required"))?,
        );
        req.name = self.name;
        req.symbol = self.symbol;
        req.icon = self.icon;
        req.description = self.description;
        req.tags = self.tags;
        req.uri = self.uri;
        req.metadata = self.metadata;
        Ok(req)
    }
}

impl Default for UpdateMetadataBuilder {
    fn default() -> Self { Self::new() }
}

// ====================== DISABLE HOOK ======================

#[derive(Default)]
pub struct DisableHookBuilder {
    asset_index: Option<u64>,
    hook_point: HookPoint,
    signer_address: Option<String>,
}

impl DisableHookBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn hook_point(mut self, v: HookPoint) -> Self { self.hook_point = v; self }
    pub fn signer_address(mut self, v: impl Into<String>) -> Self { self.signer_address = Some(v.into()); self }

    pub fn build(self) -> Result<DisableHookRequest, SdkError> {
        if self.hook_point == HookPoint::Unspecified {
            return Err(SdkError::invalid_input("hook_point must be specified"));
        }
        Ok(DisableHookRequest::new(
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.hook_point,
            self.signer_address.ok_or_else(|| SdkError::invalid_input("signer_address is required"))?,
        ))
    }
}

// ====================== SET ORIGIN METADATA ======================

#[derive(Default)]
pub struct SetOriginMetadataBuilder {
    asset_index: Option<u64>,
    origin_chain: Option<String>,
    origin_address: Option<String>,
    bridge_protocol: Option<String>,
    is_wrapped: bool,
    signer_address: Option<String>,
}

impl SetOriginMetadataBuilder {
    pub fn new() -> Self { Self::default() }

    pub fn asset_index(mut self, v: u64) -> Self { self.asset_index = Some(v); self }
    pub fn origin_chain(mut self, v: impl Into<String>) -> Self { self.origin_chain = Some(v.into()); self }
    pub fn origin_address(mut self, v: impl Into<String>) -> Self { self.origin_address = Some(v.into()); self }
    pub fn bridge_protocol(mut self, v: impl Into<String>) -> Self { self.bridge_protocol = Some(v.into()); self }
    pub fn is_wrapped(mut self, v: bool) -> Self { self.is_wrapped = v; self }
    pub fn signer_address(mut self, v: impl Into<String>) -> Self { self.signer_address = Some(v.into()); self }

    pub fn build(self) -> Result<SetOriginMetadataRequest, SdkError> {
        Ok(SetOriginMetadataRequest::new(
            self.asset_index.ok_or_else(|| SdkError::invalid_input("asset_index is required"))?,
            self.origin_chain.ok_or_else(|| SdkError::invalid_input("origin_chain is required"))?,
            self.origin_address.ok_or_else(|| SdkError::invalid_input("origin_address is required"))?,
            self.bridge_protocol.ok_or_else(|| SdkError::invalid_input("bridge_protocol is required"))?,
            self.is_wrapped,
            self.signer_address.ok_or_else(|| SdkError::invalid_input("signer_address is required"))?,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_token_builder_works() {
        let req = CreateTokenBuilder::new()
            .name("Test").symbol("TST").initial_supply("1000000")
            .creator_address("morph1xyz").tradable(true)
            .add_metadata("icon", "https://example.com/icon.png")
            .build().unwrap();
        assert_eq!(req.symbol, "TST");
        assert!(req.tradable);
        assert_eq!(req.decimals, 18);
    }

    #[test]
    fn create_token_validation() {
        assert!(CreateTokenBuilder::new().build().is_err());
    }

    #[test]
    fn create_token_with_origin() {
        let req = CreateTokenBuilder::new()
            .name("USDC").symbol("USDC").decimals(6)
            .initial_supply("0").creator_address("morph1bridge")
            .origin("ethereum", "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", "cctp", true)
            .build().unwrap();
        assert!(req.is_wrapped);
        assert_eq!(req.origin_chain.as_deref(), Some("ethereum"));
    }

    #[test]
    fn set_tradable_builder_works() {
        let req = SetTradableBuilder::new()
            .asset_index(1).tradable(true).signer_address("morph1xyz")
            .build().unwrap();
        assert!(req.tradable);
    }

    #[test]
    fn update_metadata_builder_works() {
        let req = UpdateMetadataBuilder::new()
            .asset_index(1).signer_address("morph1xyz")
            .name("New Name").icon("https://new-icon.png")
            .build().unwrap();
        assert_eq!(req.name.as_deref(), Some("New Name"));
    }

    #[test]
    fn disable_hook_requires_specified_point() {
        let result = DisableHookBuilder::new()
            .asset_index(1).signer_address("morph1gov")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn disable_hook_builder_works() {
        let req = DisableHookBuilder::new()
            .asset_index(1).hook_point(HookPoint::OnMint).signer_address("morph1gov")
            .build().unwrap();
        assert_eq!(req.hook_point, HookPoint::OnMint);
    }

    #[test]
    fn set_origin_metadata_builder_works() {
        let req = SetOriginMetadataBuilder::new()
            .asset_index(1).origin_chain("ethereum").origin_address("0xabc")
            .bridge_protocol("cctp").is_wrapped(true).signer_address("morph1gov")
            .build().unwrap();
        assert!(req.is_wrapped);
    }

    #[test]
    fn set_origin_metadata_validation() {
        assert!(SetOriginMetadataBuilder::new().build().is_err());
    }
}
