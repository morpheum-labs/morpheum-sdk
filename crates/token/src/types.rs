//! Domain types for the token module.
//!
//! Covers hook points, token metadata, programmable logic, origin
//! summaries, token summaries, and streaming token events.

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::token::v1 as proto;

// ====================== ENUMS ======================

/// Hook point for programmable token logic.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum HookPoint {
    #[default]
    Unspecified,
    OnMint,
    OnTransfer,
    OnBurn,
    RevenueShare,
    CustomGovernance,
    ViewQuery,
}

impl From<i32> for HookPoint {
    fn from(v: i32) -> Self {
        match v {
            1 => Self::OnMint,       2 => Self::OnTransfer, 3 => Self::OnBurn,
            4 => Self::RevenueShare, 5 => Self::CustomGovernance, 6 => Self::ViewQuery,
            _ => Self::Unspecified,
        }
    }
}

impl From<HookPoint> for i32 {
    fn from(h: HookPoint) -> Self {
        match h {
            HookPoint::Unspecified => 0,     HookPoint::OnMint => 1,
            HookPoint::OnTransfer => 2,      HookPoint::OnBurn => 3,
            HookPoint::RevenueShare => 4,    HookPoint::CustomGovernance => 5,
            HookPoint::ViewQuery => 6,
        }
    }
}

// ====================== DOMAIN TYPES ======================

/// Programmable logic (MWVM hooks) configuration for a token.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ProgrammableLogicConfig {
    pub bytecode_hash: Vec<u8>,
    pub version: u32,
    pub enabled_hooks: Vec<HookPoint>,
    pub params_cbor: Vec<u8>,
    pub required_runtime_version: u32,
    pub disabled_hooks: Vec<HookPoint>,
}

impl From<proto::ProgrammableLogicConfig> for ProgrammableLogicConfig {
    fn from(p: proto::ProgrammableLogicConfig) -> Self {
        Self {
            bytecode_hash: p.bytecode_hash, version: p.version,
            enabled_hooks: p.enabled_hooks.into_iter().map(HookPoint::from).collect(),
            params_cbor: p.params_cbor, required_runtime_version: p.required_runtime_version,
            disabled_hooks: p.disabled_hooks.into_iter().map(HookPoint::from).collect(),
        }
    }
}

impl From<ProgrammableLogicConfig> for proto::ProgrammableLogicConfig {
    fn from(c: ProgrammableLogicConfig) -> Self {
        Self {
            bytecode_hash: c.bytecode_hash, version: c.version,
            enabled_hooks: c.enabled_hooks.into_iter().map(i32::from).collect(),
            params_cbor: c.params_cbor, required_runtime_version: c.required_runtime_version,
            disabled_hooks: c.disabled_hooks.into_iter().map(i32::from).collect(),
        }
    }
}

/// Cross-chain origin summary for bridged/wrapped tokens.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TokenOriginSummary {
    pub origin_chain: String,
    pub origin_address: String,
    pub bridge_protocol: String,
    pub is_wrapped: bool,
}

impl From<proto::TokenOriginSummary> for TokenOriginSummary {
    fn from(p: proto::TokenOriginSummary) -> Self {
        Self {
            origin_chain: p.origin_chain, origin_address: p.origin_address,
            bridge_protocol: p.bridge_protocol, is_wrapped: p.is_wrapped,
        }
    }
}

/// Lightweight token summary for list responses.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TokenSummary {
    pub asset_index: u64,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub tradable: bool,
}

impl From<proto::TokenSummary> for TokenSummary {
    fn from(p: proto::TokenSummary) -> Self {
        Self {
            asset_index: p.asset_index, name: p.name, symbol: p.symbol,
            decimals: p.decimals, tradable: p.tradable,
        }
    }
}

/// Full token info returned by GetTokenInfo query.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub tradable: bool,
    pub agent_creator_did: Option<String>,
    pub metadata: BTreeMap<String, String>,
    pub created_at: u64,
    pub origin: Option<TokenOriginSummary>,
}

impl From<proto::GetTokenInfoResponse> for TokenInfo {
    fn from(p: proto::GetTokenInfoResponse) -> Self {
        Self {
            name: p.name, symbol: p.symbol, decimals: p.decimals, tradable: p.tradable,
            agent_creator_did: p.agent_creator_did,
            metadata: p.metadata.into_iter().collect(),
            created_at: p.created_at.as_ref().map_or(0, |t| t.seconds as u64),
            origin: p.origin.map(Into::into),
        }
    }
}

/// Hook simulation result.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SimulateHookResult {
    pub success: bool,
    pub fuel_used: u64,
    pub return_data: Vec<u8>,
    pub error_message: String,
    pub emitted_events: Vec<String>,
}

impl From<proto::SimulateHookResponse> for SimulateHookResult {
    fn from(p: proto::SimulateHookResponse) -> Self {
        Self {
            success: p.success, fuel_used: p.fuel_used, return_data: p.return_data,
            error_message: p.error_message, emitted_events: p.emitted_events,
        }
    }
}

// ====================== STREAM EVENT TYPES ======================

/// Token created event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TokenCreated {
    pub asset_index: u64,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub initial_supply: String,
    pub tradable: bool,
    pub agent_creator_did: Option<String>,
    pub timestamp: u64,
    pub origin_chain: Option<String>,
    pub origin_address: Option<String>,
    pub bridge_protocol: Option<String>,
    pub is_wrapped: bool,
}

impl From<proto::TokenCreated> for TokenCreated {
    fn from(p: proto::TokenCreated) -> Self {
        Self {
            asset_index: p.asset_index, name: p.name, symbol: p.symbol,
            decimals: p.decimals, initial_supply: p.initial_supply, tradable: p.tradable,
            agent_creator_did: p.agent_creator_did,
            timestamp: p.timestamp.as_ref().map_or(0, |t| t.seconds as u64),
            origin_chain: p.origin_chain, origin_address: p.origin_address,
            bridge_protocol: p.bridge_protocol, is_wrapped: p.is_wrapped,
        }
    }
}

/// Token tradability changed event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TokenTradabilityChanged {
    pub asset_index: u64,
    pub old_tradable: bool,
    pub new_tradable: bool,
    pub timestamp: u64,
}

impl From<proto::TokenTradabilityChanged> for TokenTradabilityChanged {
    fn from(p: proto::TokenTradabilityChanged) -> Self {
        Self {
            asset_index: p.asset_index, old_tradable: p.old_tradable,
            new_tradable: p.new_tradable,
            timestamp: p.timestamp.as_ref().map_or(0, |t| t.seconds as u64),
        }
    }
}

/// Hook executed event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HookExecuted {
    pub asset_index: u64,
    pub hook_point: HookPoint,
    pub success: bool,
    pub fuel_used: u64,
    pub error_code: u32,
    pub return_data_hash: Vec<u8>,
    pub emitted_events: Vec<String>,
    pub timestamp: u64,
}

impl From<proto::HookExecuted> for HookExecuted {
    fn from(p: proto::HookExecuted) -> Self {
        Self {
            asset_index: p.asset_index, hook_point: HookPoint::from(p.hook_point),
            success: p.success, fuel_used: p.fuel_used, error_code: p.error_code,
            return_data_hash: p.return_data_hash, emitted_events: p.emitted_events,
            timestamp: p.timestamp.as_ref().map_or(0, |t| t.seconds as u64),
        }
    }
}

/// Hook disabled event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct HookDisabled {
    pub asset_index: u64,
    pub hook_point: HookPoint,
    pub timestamp: u64,
}

impl From<proto::HookDisabled> for HookDisabled {
    fn from(p: proto::HookDisabled) -> Self {
        Self {
            asset_index: p.asset_index, hook_point: HookPoint::from(p.hook_point),
            timestamp: p.timestamp.as_ref().map_or(0, |t| t.seconds as u64),
        }
    }
}

/// Token metadata updated event.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TokenMetadataUpdated {
    pub asset_index: u64,
    pub updated_fields: Vec<String>,
    pub timestamp: u64,
}

impl From<proto::TokenMetadataUpdated> for TokenMetadataUpdated {
    fn from(p: proto::TokenMetadataUpdated) -> Self {
        Self {
            asset_index: p.asset_index, updated_fields: p.updated_fields,
            timestamp: p.timestamp.as_ref().map_or(0, |t| t.seconds as u64),
        }
    }
}

/// Union of token module streaming events.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TokenEvent {
    Created(TokenCreated),
    TradabilityChanged(TokenTradabilityChanged),
    HookExecuted(HookExecuted),
    MetadataUpdated(TokenMetadataUpdated),
    OriginMetadataUpdated { asset_index: u64, origin_chain: String, origin_address: String, timestamp: u64 },
    HookDisabled(HookDisabled),
}

impl TokenEvent {
    /// Converts from the proto oneof wrapper. Returns `None` if the event field is unset.
    pub fn from_proto(p: proto::TokenEvent) -> Option<Self> {
        use proto::token_event::Event;
        p.event.map(|e| match e {
            Event::TokenCreated(v) => Self::Created(v.into()),
            Event::TokenTradabilityChanged(v) => Self::TradabilityChanged(v.into()),
            Event::HookExecuted(v) => Self::HookExecuted(v.into()),
            Event::TokenMetadataUpdated(v) => Self::MetadataUpdated(v.into()),
            Event::TokenOriginMetadataUpdated(v) => Self::OriginMetadataUpdated {
                asset_index: v.asset_index, origin_chain: v.origin_chain,
                origin_address: v.origin_address,
                timestamp: v.timestamp.as_ref().map_or(0, |t| t.seconds as u64),
            },
            Event::HookDisabled(v) => Self::HookDisabled(v.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn hook_point_roundtrip() {
        for h in [HookPoint::OnMint, HookPoint::OnTransfer, HookPoint::OnBurn,
                  HookPoint::RevenueShare, HookPoint::CustomGovernance, HookPoint::ViewQuery] {
            let v: i32 = h.into();
            assert_eq!(h, HookPoint::from(v));
        }
        assert_eq!(HookPoint::Unspecified, HookPoint::from(99));
    }

    #[test]
    fn programmable_logic_roundtrip() {
        let c = ProgrammableLogicConfig {
            bytecode_hash: vec![1, 2, 3], version: 1,
            enabled_hooks: vec![HookPoint::OnMint, HookPoint::OnTransfer],
            params_cbor: vec![0xA0], required_runtime_version: 1,
            disabled_hooks: vec![],
        };
        let p: proto::ProgrammableLogicConfig = c.clone().into();
        let c2: ProgrammableLogicConfig = p.into();
        assert_eq!(c, c2);
    }

    #[test]
    fn token_summary_from_proto() {
        let p = proto::TokenSummary {
            asset_index: 1, name: "MORM".into(), symbol: "MORM".into(),
            decimals: 18, tradable: true,
        };
        let s: TokenSummary = p.into();
        assert_eq!(s.asset_index, 1);
        assert!(s.tradable);
    }

    #[test]
    fn token_event_from_proto() {
        let proto_event = proto::TokenEvent {
            event: Some(proto::token_event::Event::TokenCreated(proto::TokenCreated {
                asset_index: 1, name: "Test".into(), symbol: "TST".into(),
                decimals: 18, initial_supply: "1000000".into(), tradable: true,
                agent_creator_did: None, programmable: None, timestamp: None,
                origin_chain: None, origin_address: None, bridge_protocol: None,
                is_wrapped: false,
            })),
        };
        let event = TokenEvent::from_proto(proto_event);
        assert!(matches!(event, Some(TokenEvent::Created(_))));
    }

    #[test]
    fn token_event_none_on_empty() {
        assert!(TokenEvent::from_proto(proto::TokenEvent { event: None }).is_none());
    }
}
