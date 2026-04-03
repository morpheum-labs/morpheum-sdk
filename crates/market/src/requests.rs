//! Request and response wrappers for the Market module.
//!
//! These provide clean, type-safe Rust APIs around the raw protobuf messages.
//! They use `AccountId` for addresses, offer ergonomic constructors and helpers,
//! and include `to_any()` methods for seamless integration with `TxBuilder`.

use alloc::string::{String, ToString};

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::google::protobuf::Any as ProtoAny;

use morpheum_sdk_core::AccountId;
use morpheum_proto::market::v1 as proto;

use crate::types::{MarketParams, MarketType};

// ====================== TRANSACTION REQUESTS ======================

/// Request to create a new market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CreateMarketRequest {
    pub from_address: AccountId,
    pub base_asset_index: u64,
    pub quote_asset_index: u64,
    pub market_type: MarketType,
    pub orderbook_type: String,
    pub params: MarketParams,
    pub governance_proposal_id: Option<String>,
}

impl CreateMarketRequest {
    pub fn new(
        from_address: AccountId,
        base_asset_index: u64,
        quote_asset_index: u64,
        market_type: MarketType,
        orderbook_type: String,
        params: MarketParams,
    ) -> Self {
        Self {
            from_address,
            base_asset_index,
            quote_asset_index,
            market_type,
            orderbook_type,
            params,
            governance_proposal_id: None,
        }
    }

    pub fn with_governance_proposal_id(mut self, id: impl Into<String>) -> Self {
        self.governance_proposal_id = Some(id.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgCreateMarketRequest = self.clone().into();
        ProtoAny {
            type_url: "/market.v1.MsgCreateMarketRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<CreateMarketRequest> for proto::MsgCreateMarketRequest {
    fn from(req: CreateMarketRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            base_asset_index: req.base_asset_index,
            quote_asset_index: req.quote_asset_index,
            market_type: i32::from(req.market_type),
            orderbook_type: req.orderbook_type,
            params: Some(req.params.into()),
            governance_proposal_id: req.governance_proposal_id.unwrap_or_default(),
            timestamp: None, // Set by signer layer if needed
        }
    }
}

/// Request to activate a pending market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ActivateMarketRequest {
    pub market_index: u64,
    pub activator: AccountId,
}

impl ActivateMarketRequest {
    pub fn new(market_index: u64, activator: AccountId) -> Self {
        Self { market_index, activator }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgActivateMarketRequest = self.clone().into();
        ProtoAny {
            type_url: "/market.v1.MsgActivateMarketRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ActivateMarketRequest> for proto::MsgActivateMarketRequest {
    fn from(req: ActivateMarketRequest) -> Self {
        Self {
            market_index: req.market_index,
            activator: req.activator.to_string(),
            timestamp: None,
        }
    }
}

/// Request to suspend a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SuspendMarketRequest {
    pub market_index: u64,
    pub reason: String,
    pub suspender: AccountId,
}

impl SuspendMarketRequest {
    pub fn new(market_index: u64, reason: String, suspender: AccountId) -> Self {
        Self { market_index, reason, suspender }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgSuspendMarketRequest = self.clone().into();
        ProtoAny {
            type_url: "/market.v1.MsgSuspendMarketRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<SuspendMarketRequest> for proto::MsgSuspendMarketRequest {
    fn from(req: SuspendMarketRequest) -> Self {
        Self {
            market_index: req.market_index,
            reason: req.reason,
            suspender: req.suspender.to_string(),
            timestamp: None,
        }
    }
}

/// Request to update market parameters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateMarketRequest {
    pub market_index: u64,
    pub params: MarketParams,
    pub from_address: AccountId,
    pub governance_proposal_id: Option<String>,
}

impl UpdateMarketRequest {
    pub fn new(market_index: u64, params: MarketParams, from_address: AccountId) -> Self {
        Self {
            market_index,
            params,
            from_address,
            governance_proposal_id: None,
        }
    }

    pub fn with_governance_proposal_id(mut self, id: impl Into<String>) -> Self {
        self.governance_proposal_id = Some(id.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgUpdateMarketRequest = self.clone().into();
        ProtoAny {
            type_url: "/market.v1.MsgUpdateMarketRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<UpdateMarketRequest> for proto::MsgUpdateMarketRequest {
    fn from(req: UpdateMarketRequest) -> Self {
        Self {
            market_index: req.market_index,
            params: Some(req.params.into()),
            from_address: req.from_address.to_string(),
            governance_proposal_id: req.governance_proposal_id.unwrap_or_default(),
            timestamp: None,
        }
    }
}

/// Request to change market margin ratios (initial and/or maintenance).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ChangeMarketMarginRatioRequest {
    pub from_address: AccountId,
    pub market_index: u64,
    pub new_initial_margin_ratio: Option<String>,
    pub new_maintenance_margin_ratio: Option<String>,
    pub reason: Option<String>,
}

impl ChangeMarketMarginRatioRequest {
    pub fn new(from_address: AccountId, market_index: u64) -> Self {
        Self {
            from_address,
            market_index,
            new_initial_margin_ratio: None,
            new_maintenance_margin_ratio: None,
            reason: None,
        }
    }

    pub fn new_initial_margin_ratio(mut self, ratio: impl Into<String>) -> Self {
        self.new_initial_margin_ratio = Some(ratio.into());
        self
    }

    pub fn new_maintenance_margin_ratio(mut self, ratio: impl Into<String>) -> Self {
        self.new_maintenance_margin_ratio = Some(ratio.into());
        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg: proto::MsgChangeMarketMarginRatioRequest = self.clone().into();
        ProtoAny {
            type_url: "/market.v1.MsgChangeMarketMarginRatioRequest".into(),
            value: msg.encode_to_vec(),
        }
    }
}

impl From<ChangeMarketMarginRatioRequest> for proto::MsgChangeMarketMarginRatioRequest {
    fn from(req: ChangeMarketMarginRatioRequest) -> Self {
        Self {
            from_address: req.from_address.to_string(),
            market_index: req.market_index,
            new_initial_margin_ratio: req.new_initial_margin_ratio.unwrap_or_default(),
            new_maintenance_margin_ratio: req.new_maintenance_margin_ratio.unwrap_or_default(),
            reason: req.reason.unwrap_or_default(),
            timestamp: None,
            authority: String::new(),
        }
    }
}

// ====================== QUERY REQUESTS & RESPONSES ======================

/// Query a single market by index.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMarketRequest {
    pub market_index: u64,
}

impl QueryMarketRequest {
    pub fn new(market_index: u64) -> Self {
        Self { market_index }
    }
}

impl From<QueryMarketRequest> for proto::QueryMarketRequest {
    fn from(req: QueryMarketRequest) -> Self {
        Self {
            market_index: req.market_index,
        }
    }
}

/// Query multiple markets with pagination and optional filters.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMarketsRequest {
    pub limit: u32,
    pub offset: u32,
    pub status_filter: Option<String>,
    pub type_filter: Option<MarketType>,
}

impl QueryMarketsRequest {
    pub fn new(limit: u32, offset: u32) -> Self {
        Self {
            limit,
            offset,
            status_filter: None,
            type_filter: None,
        }
    }

    pub fn status_filter(mut self, status: impl Into<String>) -> Self {
        self.status_filter = Some(status.into());
        self
    }

    pub fn type_filter(mut self, market_type: MarketType) -> Self {
        self.type_filter = Some(market_type);
        self
    }

    /// Sets status_filter from an `Option<String>`, ignoring `None`.
    pub fn status_filter_opt(mut self, status: Option<String>) -> Self {
        if status.is_some() {
            self.status_filter = status;
        }
        self
    }

    /// Sets type_filter from an `Option<MarketType>`, ignoring `None`.
    pub fn type_filter_opt(mut self, market_type: Option<MarketType>) -> Self {
        if market_type.is_some() {
            self.type_filter = market_type;
        }
        self
    }
}

impl From<QueryMarketsRequest> for proto::QueryMarketsRequest {
    fn from(req: QueryMarketsRequest) -> Self {
        Self {
            limit: req.limit as i32,
            offset: req.offset as i32,
            status_filter: req.status_filter.unwrap_or_default(),
            type_filter: req.type_filter.map(i32::from).unwrap_or(0),
        }
    }
}

/// Query active (tradable) markets.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryActiveMarketsRequest {
    pub limit: u32,
    pub offset: u32,
}

impl QueryActiveMarketsRequest {
    pub fn new(limit: u32, offset: u32) -> Self {
        Self { limit, offset }
    }
}

impl From<QueryActiveMarketsRequest> for proto::QueryActiveMarketsRequest {
    fn from(req: QueryActiveMarketsRequest) -> Self {
        Self {
            limit: req.limit as i32,
            offset: req.offset as i32,
        }
    }
}

/// Query market statistics.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct QueryMarketStatsRequest {
    pub market_index: u64,
    pub time_range: Option<String>,
}

impl QueryMarketStatsRequest {
    pub fn new(market_index: u64) -> Self {
        Self {
            market_index,
            time_range: None,
        }
    }

    pub fn time_range(mut self, range: impl Into<String>) -> Self {
        self.time_range = Some(range.into());
        self
    }

    /// Sets time_range from an `Option<String>`, ignoring `None`.
    pub fn time_range_opt(mut self, range: Option<String>) -> Self {
        if range.is_some() {
            self.time_range = range;
        }
        self
    }
}

impl From<QueryMarketStatsRequest> for proto::QueryMarketStatsRequest {
    fn from(req: QueryMarketStatsRequest) -> Self {
        Self {
            market_index: req.market_index,
            time_range: req.time_range.unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use morpheum_sdk_core::AccountId;

    #[test]
    fn create_market_request_to_any() {
        let from = AccountId::new([1u8; 32]);
        let params = crate::types::MarketParams {
            min_order_size: "0.001".into(),
            taker_fee_rate: "0.0005".into(),
            maker_fee_rate: "0.0002".into(),
            additional_params: alloc::collections::BTreeMap::new(),
            type_config: Some(crate::types::MarketTypeConfig::Clob(
                crate::types::ClobMarketConfig {
                    tick_size: "0.01".into(),
                    lot_size: "1".into(),
                    max_leverage: "100".into(),
                    initial_margin_ratio: "0.1".into(),
                    maintenance_margin_ratio: "0.05".into(),
                    allow_market_orders: true,
                    allow_stop_orders: true,
                    perp_config: None,
                },
            )),
        };

        let req = CreateMarketRequest::new(
            from,
            1,
            2,
            MarketType::Perp,
            "clob".into(),
            params,
        )
            .with_governance_proposal_id("gov-123");

        let any = req.to_any();
        assert_eq!(any.type_url, "/market.v1.MsgCreateMarketRequest");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn activate_market_request_works() {
        let req = ActivateMarketRequest::new(42, AccountId::new([3u8; 32]));
        let any = req.to_any();
        assert_eq!(any.type_url, "/market.v1.MsgActivateMarketRequest");
    }
}