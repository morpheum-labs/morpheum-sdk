//! Request wrappers for the bonding-curve module.
//!
//! Transaction requests include `to_any()` for `TxBuilder` integration.
//! Query requests convert to proto via `From` impls.

use alloc::string::String;
use alloc::vec::Vec;

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::bonding_curve::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

// ====================== TRANSACTION REQUESTS ======================

/// Request to create a new agent token with a bonding curve.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateAgentTokenRequest {
    pub sender: String,
    pub agent_creator_did: String,
    pub name: String,
    pub symbol: String,
    pub decimals: u32,
    pub icon_url: Option<String>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub mwvm_hooks: Vec<String>,
    pub initial_k: Option<String>,
    pub initial_graduation_mcap: Option<String>,
}

impl CreateAgentTokenRequest {
    pub fn new(
        sender: impl Into<String>, agent_creator_did: impl Into<String>,
        name: impl Into<String>, symbol: impl Into<String>, decimals: u32,
    ) -> Self {
        Self {
            sender: sender.into(), agent_creator_did: agent_creator_did.into(),
            name: name.into(), symbol: symbol.into(), decimals,
            icon_url: None, description: None, tags: Vec::new(),
            mwvm_hooks: Vec::new(), initial_k: None, initial_graduation_mcap: None,
        }
    }

    pub fn icon_url(mut self, url: impl Into<String>) -> Self { self.icon_url = Some(url.into()); self }
    pub fn description(mut self, d: impl Into<String>) -> Self { self.description = Some(d.into()); self }
    pub fn tags(mut self, t: Vec<String>) -> Self { self.tags = t; self }
    pub fn mwvm_hooks(mut self, h: Vec<String>) -> Self { self.mwvm_hooks = h; self }
    pub fn initial_k(mut self, k: impl Into<String>) -> Self { self.initial_k = Some(k.into()); self }
    pub fn initial_graduation_mcap(mut self, m: impl Into<String>) -> Self {
        self.initial_graduation_mcap = Some(m.into()); self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgLaunchToken = self.clone().into();
        ProtoAny { type_url: "/bondingcurve.v1.MsgLaunchToken".into(), value: msg.encode_to_vec() }
    }
}

impl From<CreateAgentTokenRequest> for proto::MsgLaunchToken {
    fn from(r: CreateAgentTokenRequest) -> Self {
        Self {
            sender: r.sender, creator_did: r.agent_creator_did,
            name: r.name, symbol: r.symbol, decimals: r.decimals,
            icon_url: r.icon_url.unwrap_or_default(),
            description: r.description.unwrap_or_default(),
            tags: r.tags, mwvm_hooks: r.mwvm_hooks,
            prediction_enhancement: None,
            initial_k: r.initial_k, initial_graduation_mcap: r.initial_graduation_mcap,
        }
    }
}

/// Request to buy tokens on the bonding curve.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BuyRequest {
    pub token_index: u64,
    pub morm_amount: String,
}

impl BuyRequest {
    pub fn new(token_index: u64, morm_amount: impl Into<String>) -> Self {
        Self { token_index, morm_amount: morm_amount.into() }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgBuy = self.clone().into();
        ProtoAny { type_url: "/bondingcurve.v1.MsgBuy".into(), value: msg.encode_to_vec() }
    }
}

impl From<BuyRequest> for proto::MsgBuy {
    fn from(r: BuyRequest) -> Self { Self { token_index: r.token_index, morm_amount: r.morm_amount } }
}

/// Request to sell tokens on the bonding curve.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SellRequest {
    pub token_index: u64,
    pub token_amount: String,
}

impl SellRequest {
    pub fn new(token_index: u64, token_amount: impl Into<String>) -> Self {
        Self { token_index, token_amount: token_amount.into() }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgSell = self.clone().into();
        ProtoAny { type_url: "/bondingcurve.v1.MsgSell".into(), value: msg.encode_to_vec() }
    }
}

impl From<SellRequest> for proto::MsgSell {
    fn from(r: SellRequest) -> Self { Self { token_index: r.token_index, token_amount: r.token_amount } }
}

/// Request to execute graduation (transition to CLAMM pool).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ExecuteGraduationRequest {
    pub token_index: u64,
}

impl ExecuteGraduationRequest {
    pub fn new(token_index: u64) -> Self { Self { token_index } }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgExecuteGraduation { token_index: self.token_index };
        ProtoAny { type_url: "/bondingcurve.v1.MsgExecuteGraduation".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query the bonding-curve state for a token.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetCurveStateRequest {
    pub token_index: u64,
}

impl GetCurveStateRequest {
    pub fn new(token_index: u64) -> Self { Self { token_index } }
}

impl From<GetCurveStateRequest> for proto::GetCurveStateRequest {
    fn from(r: GetCurveStateRequest) -> Self { Self { token_index: r.token_index } }
}

/// Query the current price and effective market cap for a token.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetPriceRequest {
    pub token_index: u64,
}

impl GetPriceRequest {
    pub fn new(token_index: u64) -> Self { Self { token_index } }
}

impl From<GetPriceRequest> for proto::GetPriceRequest {
    fn from(r: GetPriceRequest) -> Self { Self { token_index: r.token_index } }
}

/// Query module-level governance parameters.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryParamsRequest;

impl QueryParamsRequest {
    pub fn new() -> Self { Self }
}

impl From<QueryParamsRequest> for proto::QueryParamsRequest {
    fn from(_: QueryParamsRequest) -> Self { Self {} }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_agent_token_to_any() {
        let req = CreateAgentTokenRequest::new("morpheum1abc", "did:morpheum:agent1", "AgentCoin", "AGNT", 8)
            .initial_k("1000000");
        let any = req.to_any();
        assert_eq!(any.type_url, "/bondingcurve.v1.MsgLaunchToken");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn buy_request_to_any() {
        let any = BuyRequest::new(42, "100000000").to_any();
        assert_eq!(any.type_url, "/bondingcurve.v1.MsgBuy");
    }

    #[test]
    fn sell_request_to_any() {
        let any = SellRequest::new(42, "50000000").to_any();
        assert_eq!(any.type_url, "/bondingcurve.v1.MsgSell");
    }

    #[test]
    fn execute_graduation_to_any() {
        let any = ExecuteGraduationRequest::new(42).to_any();
        assert_eq!(any.type_url, "/bondingcurve.v1.MsgExecuteGraduation");
    }

    #[test]
    fn query_params_converts() {
        let _: proto::QueryParamsRequest = QueryParamsRequest::new().into();
    }
}
