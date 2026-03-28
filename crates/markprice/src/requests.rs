//! Request wrappers for the mark price module.

use prost::Message as _;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use morpheum_proto::markprice::v1 as proto;
use morpheum_proto::google::protobuf::Any as ProtoAny;

use crate::types::MarkConfig;

// ====================== TRANSACTION REQUESTS ======================

/// Update per-market mark price configuration (governance).
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct UpdateMarkConfigRequest {
    pub market_index: u64,
    pub config: MarkConfig,
}

impl UpdateMarkConfigRequest {
    pub fn new(market_index: u64, config: MarkConfig) -> Self {
        Self { market_index, config }
    }

    pub fn to_any(&self) -> ProtoAny {
        let msg = proto::MsgUpdateMarkConfig {
            market_index: self.market_index,
            config: Some(self.config.clone().into()),
        };
        ProtoAny { type_url: "/markprice.v1.MsgUpdateMarkConfig".into(), value: msg.encode_to_vec() }
    }
}

// ====================== QUERY REQUESTS ======================

/// Query canonical mark price for a market.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetMarkPriceRequest {
    pub market_index: u64,
}

impl GetMarkPriceRequest {
    pub fn new(market_index: u64) -> Self { Self { market_index } }
}

impl From<GetMarkPriceRequest> for proto::GetMarkPriceRequest {
    fn from(r: GetMarkPriceRequest) -> Self { Self { market_index: r.market_index } }
}

/// Query mark price with source attribution.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GetMarkPriceWithSourceRequest {
    pub market_index: u64,
}

impl GetMarkPriceWithSourceRequest {
    pub fn new(market_index: u64) -> Self { Self { market_index } }
}

impl From<GetMarkPriceWithSourceRequest> for proto::GetMarkPriceWithSourceRequest {
    fn from(r: GetMarkPriceWithSourceRequest) -> Self { Self { market_index: r.market_index } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_mark_config_to_any() {
        let cfg = MarkConfig {
            weight_twap_bps: 8000, weight_oracle_index_bps: 1500,
            weight_kline_bps: 500, staleness_blocks: 10,
            strategy: "linear_perp".into(),
        };
        let any = UpdateMarkConfigRequest::new(42, cfg).to_any();
        assert_eq!(any.type_url, "/markprice.v1.MsgUpdateMarkConfig");
        assert!(!any.value.is_empty());
    }

    #[test]
    fn query_conversions() {
        let p: proto::GetMarkPriceRequest = GetMarkPriceRequest::new(42).into();
        assert_eq!(p.market_index, 42);

        let p: proto::GetMarkPriceWithSourceRequest = GetMarkPriceWithSourceRequest::new(42).into();
        assert_eq!(p.market_index, 42);
    }
}
