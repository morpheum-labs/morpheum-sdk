//! Request wrappers for the token module.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::token::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::{HookPoint, ProgrammableLogicConfig};

// ====================== TRANSACTION REQUESTS ======================

/// Create a new token.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateTokenRequest {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub initial_supply: String,
    pub creator_address: String,
    pub metadata: BTreeMap<String, String>,
    pub agent_creator_did: Option<String>,
    pub programmable: Option<ProgrammableLogicConfig>,
    pub tradable: bool,
    pub origin_chain: Option<String>,
    pub origin_address: Option<String>,
    pub bridge_protocol: Option<String>,
    pub is_wrapped: bool,
}

impl CreateTokenRequest {
    pub fn new(
        name: impl Into<String>, symbol: impl Into<String>, decimals: u32,
        initial_supply: impl Into<String>, creator_address: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(), symbol: symbol.into(), decimals,
            initial_supply: initial_supply.into(), creator_address: creator_address.into(),
            metadata: BTreeMap::new(), agent_creator_did: None, programmable: None,
            tradable: false, origin_chain: None, origin_address: None,
            bridge_protocol: None, is_wrapped: false,
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::CreateTokenRequest {
            name: self.name.clone(), symbol: self.symbol.clone(), decimals: self.decimals,
            initial_supply: self.initial_supply.clone(), creator_address: self.creator_address.clone(),
            metadata: self.metadata.clone().into_iter().collect(),
            agent_creator_did: self.agent_creator_did.clone(),
            programmable: self.programmable.clone().map(Into::into),
            tradable: self.tradable,
            origin_chain: self.origin_chain.clone(), origin_address: self.origin_address.clone(),
            bridge_protocol: self.bridge_protocol.clone(), is_wrapped: self.is_wrapped,
        };
        ProtoAny { type_url: "/token.v1.CreateTokenRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Change tradability flag.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SetTradableRequest {
    pub asset_index: u64,
    pub tradable: bool,
    pub signer_address: String,
}

impl SetTradableRequest {
    pub fn new(asset_index: u64, tradable: bool, signer_address: impl Into<String>) -> Self {
        Self { asset_index, tradable, signer_address: signer_address.into() }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::SetTradableRequest {
            asset_index: self.asset_index, tradable: self.tradable,
            signer_address: self.signer_address.clone(),
        };
        ProtoAny { type_url: "/token.v1.SetTradableRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Update mutable metadata.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateMetadataRequest {
    pub asset_index: u64,
    pub signer_address: String,
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub uri: Option<String>,
    pub metadata: BTreeMap<String, String>,
}

impl UpdateMetadataRequest {
    pub fn new(asset_index: u64, signer_address: impl Into<String>) -> Self {
        Self {
            asset_index, signer_address: signer_address.into(),
            name: None, symbol: None, icon: None, description: None,
            tags: Vec::new(), uri: None, metadata: BTreeMap::new(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::UpdateMetadataRequest {
            asset_index: self.asset_index, signer_address: self.signer_address.clone(),
            name: self.name.clone(), symbol: self.symbol.clone(),
            icon: self.icon.clone(), description: self.description.clone(),
            tags: self.tags.clone(), uri: self.uri.clone(),
            metadata: self.metadata.clone().into_iter().collect(),
        };
        ProtoAny { type_url: "/token.v1.UpdateMetadataRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Disable a hook (governance only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DisableHookRequest {
    pub asset_index: u64,
    pub hook_point: HookPoint,
    pub signer_address: String,
}

impl DisableHookRequest {
    pub fn new(asset_index: u64, hook_point: HookPoint, signer_address: impl Into<String>) -> Self {
        Self { asset_index, hook_point, signer_address: signer_address.into() }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::DisableHookRequest {
            asset_index: self.asset_index, hook_point: i32::from(self.hook_point),
            signer_address: self.signer_address.clone(),
        };
        ProtoAny { type_url: "/token.v1.DisableHookRequest".into(), value: msg.encode_to_vec() }
    }
}

/// Set cross-chain origin metadata (governance only).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SetOriginMetadataRequest {
    pub asset_index: u64,
    pub origin_chain: String,
    pub origin_address: String,
    pub bridge_protocol: String,
    pub is_wrapped: bool,
    pub signer_address: String,
}

impl SetOriginMetadataRequest {
    pub fn new(
        asset_index: u64, origin_chain: impl Into<String>, origin_address: impl Into<String>,
        bridge_protocol: impl Into<String>, is_wrapped: bool, signer_address: impl Into<String>,
    ) -> Self {
        Self {
            asset_index, origin_chain: origin_chain.into(), origin_address: origin_address.into(),
            bridge_protocol: bridge_protocol.into(), is_wrapped, signer_address: signer_address.into(),
        }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::SetOriginMetadataRequest {
            asset_index: self.asset_index, origin_chain: self.origin_chain.clone(),
            origin_address: self.origin_address.clone(), bridge_protocol: self.bridge_protocol.clone(),
            is_wrapped: self.is_wrapped, signer_address: self.signer_address.clone(),
        };
        ProtoAny { type_url: "/token.v1.SetOriginMetadataRequest".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query token info by asset index.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetTokenInfoRequest {
    pub asset_index: u64,
}

impl GetTokenInfoRequest {
    pub fn new(asset_index: u64) -> Self { Self { asset_index } }
}

impl From<GetTokenInfoRequest> for proto::GetTokenInfoRequest {
    fn from(r: GetTokenInfoRequest) -> Self { Self { asset_index: r.asset_index } }
}

/// List tokens with pagination.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListTokensRequest {
    pub offset: u64,
    pub limit: u32,
}

impl ListTokensRequest {
    pub fn new(offset: u64, limit: u32) -> Self { Self { offset, limit } }
}

impl From<ListTokensRequest> for proto::ListTokensRequest {
    fn from(r: ListTokensRequest) -> Self { Self { offset: r.offset, limit: r.limit } }
}

/// Query programmable logic configuration.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetProgrammableLogicRequest {
    pub asset_index: u64,
}

impl GetProgrammableLogicRequest {
    pub fn new(asset_index: u64) -> Self { Self { asset_index } }
}

impl From<GetProgrammableLogicRequest> for proto::GetProgrammableLogicRequest {
    fn from(r: GetProgrammableLogicRequest) -> Self { Self { asset_index: r.asset_index } }
}

/// Simulate a hook execution (dry-run).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SimulateHookRequest {
    pub asset_index: u64,
    pub hook_point: HookPoint,
    pub caller: String,
    pub to: String,
    pub amount: String,
}

impl SimulateHookRequest {
    pub fn new(
        asset_index: u64, hook_point: HookPoint, caller: impl Into<String>,
        to: impl Into<String>, amount: impl Into<String>,
    ) -> Self {
        Self {
            asset_index, hook_point, caller: caller.into(),
            to: to.into(), amount: amount.into(),
        }
    }
}

impl From<SimulateHookRequest> for proto::SimulateHookRequest {
    fn from(r: SimulateHookRequest) -> Self {
        Self {
            asset_index: r.asset_index, hook_point: i32::from(r.hook_point),
            context: Some({
                #[allow(deprecated)]
                proto::HookContext {
                    asset_index: r.asset_index, caller: r.caller, to: r.to, amount: r.amount,
                    action_block_hash: alloc::string::String::new(), action_block_height: 0,
                    from: alloc::string::String::new(), memo: Vec::new(),
                    extra: Default::default(),
                }
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_token_to_any() {
        let any = CreateTokenRequest::new("Test Token", "TST", 18, "1000000", "morph1xyz").to_any();
        assert_eq!(any.type_url, "/token.v1.CreateTokenRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn set_tradable_to_any() {
        let any = SetTradableRequest::new(1, true, "morph1xyz").to_any();
        assert_eq!(any.type_url, "/token.v1.SetTradableRequest");
    }

    #[test]
    fn disable_hook_to_any() {
        let any = DisableHookRequest::new(1, HookPoint::OnMint, "morph1gov").to_any();
        assert_eq!(any.type_url, "/token.v1.DisableHookRequest");
    }

    #[test]
    fn set_origin_metadata_to_any() {
        let any = SetOriginMetadataRequest::new(1, "ethereum", "0xabc", "cctp", true, "morph1gov").to_any();
        assert_eq!(any.type_url, "/token.v1.SetOriginMetadataRequest");
    }

    #[test]
    fn query_conversions() {
        let p: proto::GetTokenInfoRequest = GetTokenInfoRequest::new(1).into();
        assert_eq!(p.asset_index, 1);

        let p: proto::ListTokensRequest = ListTokensRequest::new(0, 50).into();
        assert_eq!(p.limit, 50);

        let p: proto::GetProgrammableLogicRequest = GetProgrammableLogicRequest::new(1).into();
        assert_eq!(p.asset_index, 1);
    }
}
